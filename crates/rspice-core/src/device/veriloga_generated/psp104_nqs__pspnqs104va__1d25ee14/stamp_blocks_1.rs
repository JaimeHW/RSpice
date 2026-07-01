#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_agidl_i: f64,
        var_ax_i: f64,
        var_axac_i: f64,
        var_bgidl_i: f64,
        var_cfr_i: f64,
        var_cgidl_i: f64,
        var_cgov_i: f64,
        var_cgovaccg_i: f64,
        var_chnl_type: f64,
        var_cinr_i: f64,
        var_dphib_i: f64,
        var_eg: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_facneffac_i: f64,
        var_fcgovacc_i: f64,
        var_feta_i: f64,
        var_gc2ov_i: f64,
        var_gc3ov_i: f64,
        var_guard153: f64,
        var_igov_i: f64,
        var_inv_phit: f64,
        var_inv_phita: f64,
        var_neff_i: f64,
        var_nov_i: f64,
        var_np_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_stbgidl_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_vp_i: f64,
        var_agidld_i_slot: &mut f64,
        var_ar_slot: &mut f64,
        var_arac_slot: &mut f64,
        var_arg2max_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
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
        var_g_0_dc_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_vp_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_neffac_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_np_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
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
        var_tox_sq_slot: &mut f64,
    ) {
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_ar: f64 = *var_ar_slot;
        let mut var_arac: f64 = *var_arac_slot;
        let mut var_arg2max: f64 = *var_arg2max_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
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
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_vp: f64 = *var_inv_vp_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_neffac_i: f64 = *var_neffac_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
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
        let mut var_tox_sq: f64 = *var_tox_sq_slot;

        let (assign11110_e10369,) = {
    if (var_guard153 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign11110_e10369;

        let (assign11120_e10373,) = {
    if (var_guard153 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign11120_e10373;

        let (assign11130_e10377,) = {
    if (var_guard153 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign11130_e10377;

        let (assign11140_e10381,) = {
    if (var_guard153 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign11140_e10381;

        let (assign11150_e10385,) = {
    if (var_guard153 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign11150_e10385;

        let (assign11160_e10389,) = {
    if (var_guard153 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11160_e10389;

        let (assign11170_e10393,) = {
    if (var_guard153 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign11170_e10393;

        let (assign11180_e10397,) = {
    if (var_guard153 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign11180_e10397;

        let (assign11190_e10401,) = {
    if (var_guard153 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11190_e10401;

        let (assign11200_e10405,) = {
    if (var_guard153 != 0.0) {
        (var_fcgovacc_i,)
    } else {
        (var_fcgovaccd_i,)
    }
};
        var_fcgovaccd_i = assign11200_e10405;

        let (assign11210_e10409,) = {
    if (var_guard153 != 0.0) {
        (var_cinr_i,)
    } else {
        (var_cinrd_i,)
    }
};
        var_cinrd_i = assign11210_e10409;

        let (assign11220_e10413,) = {
    if (var_guard153 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11220_e10413;

        let assign11230_e10416: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11230_e10416;

        let assign11240_e10419: f64 = (var_epsox / var_tox_i);
        var_coxprime = assign11240_e10419;

        let assign11250_e10422: f64 = (var_tox_i * var_tox_i);
        var_tox_sq = assign11250_e10422;

        let assign11260_e10425: f64 = (var_coxprime / 1.6021918e-19);
        var_cox_over_q = assign11260_e10425;

        let assign11270_e10428: f64 = (var_facneffac_i * var_neff_i);
        var_neffac_i = assign11270_e10428;

        let (assign11280_e10439,) = {
    if (var_neffac_i > 1e20) {
        let (assign11280_e10437,) = {
            if (var_neffac_i < 1e26) {
                (var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11280_e10437,)
    } else {
        (1e20,)
    }
};
        var_neffac_i = assign11280_e10439;

        var_qq = 0.0;

        let assign11300_e10443: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard154 = assign11300_e10443;

        let (assign11310_e10455,) = {
    if (var_guard154 != 0.0) {
        let assign11310_e10447: f64 = (0.4 * 5.951993);
        let assign11310_e10449: f64 = (assign11310_e10447 * p.p52);
        let assign11310_e10452: f64 = (var_coxprime).powf(0.6666666666666666);
        let assign11310_e10453: f64 = (assign11310_e10449 * assign11310_e10452);
        (assign11310_e10453,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11310_e10455;

        let assign11320_e10458: f64 = (-1.0);
        let assign11320_e10459: f64 = if var_chnl_type == assign11320_e10458 { 1.0 } else { 0.0 };
        var_guard155 = assign11320_e10459;

        let (assign11330_e10469,) = {
    if ((var_guard154 != 0.0) && (var_guard155 != 0.0)) {
        let assign11330_e10465: f64 = (7.448711 / 5.951993);
        let assign11330_e10467: f64 = (assign11330_e10465 * var_qq);
        (assign11330_e10467,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11330_e10469;

        let assign11340_e10472: f64 = (1e-8 * var_coxprime);
        let assign11340_e10474: f64 = (assign11340_e10472 / var_epssi);
        var_e_eff0 = assign11340_e10474;

        let assign11350_e10477: f64 = (0.5 * var_feta_i);
        var_eta_mu = assign11350_e10477;

        var_eta_mu1 = 0.5;

        let assign11370_e10481: f64 = (-1.0);
        let assign11370_e10482: f64 = if var_chnl_type == assign11370_e10481 { 1.0 } else { 0.0 };
        var_guard156 = assign11370_e10482;

        let (assign11380_e10488,) = {
    if (var_guard156 != 0.0) {
        let assign11380_e10486: f64 = (0.3333333333333333 * var_feta_i);
        (assign11380_e10486,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign11380_e10488;

        let (assign11390_e10492,) = {
    if (var_guard156 != 0.0) {
        (0.3333333333333333,)
    } else {
        (var_eta_mu1,)
    }
};
        var_eta_mu1 = assign11390_e10492;

        let assign11400_e10495: f64 = (-2.0);
        let assign11400_e10497: f64 = (assign11400_e10495 / var_ax_i);
        let assign11400_e10499: f64 = (assign11400_e10497 + 1.0);
        let assign11400_e10500: f64 = (2.0_f64).powf(assign11400_e10499);
        let assign11400_e10502: f64 = (assign11400_e10500 - 1.0);
        var_temp = assign11400_e10502;

        let assign11410_e10505: f64 = (var_temp - 1.0);
        let assign11410_e10508: f64 = (var_temp - 1.0);
        let assign11410_e10509: f64 = (assign11410_e10505 * assign11410_e10508);
        let assign11410_e10512: f64 = (4.0 * var_temp);
        let (assign11410_e10519,) = {
    if (assign11410_e10512 > 0.0001) {
        let assign11410_e10517: f64 = (4.0 * var_temp);
        (assign11410_e10517,)
    } else {
        (0.0001,)
    }
};
        let assign11410_e10520: f64 = (assign11410_e10509 / assign11410_e10519);
        var_ar = assign11410_e10520;

        let assign11420_e10523: f64 = (-2.0);
        let assign11420_e10525: f64 = (assign11420_e10523 / var_axac_i);
        let assign11420_e10527: f64 = (assign11420_e10525 + 1.0);
        let assign11420_e10528: f64 = (2.0_f64).powf(assign11420_e10527);
        let assign11420_e10530: f64 = (assign11420_e10528 - 1.0);
        var_temp = assign11420_e10530;

        let assign11430_e10533: f64 = (var_temp - 1.0);
        let assign11430_e10536: f64 = (var_temp - 1.0);
        let assign11430_e10537: f64 = (assign11430_e10533 * assign11430_e10536);
        let assign11430_e10540: f64 = (4.0 * var_temp);
        let (assign11430_e10547,) = {
    if (assign11430_e10540 > 0.0001) {
        let assign11430_e10545: f64 = (4.0 * var_temp);
        (assign11430_e10545,)
    } else {
        (0.0001,)
    }
};
        let assign11430_e10548: f64 = (assign11430_e10537 / assign11430_e10547);
        var_arac = assign11430_e10548;

        let assign11440_e10551: f64 = (1.0 / var_vp_i);
        var_inv_vp = assign11440_e10551;

        let assign11450_e10554: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11450_e10554;

        let assign11460_e10557: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11460_e10557;

        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / var_coxovprime);
        var_gov_s = assign11470_e10569;

        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / var_coxovprime_d);
        var_gov_d = assign11480_e10581;

        let assign11490_e10584: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11490_e10584;

        let assign11500_e10587: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11500_e10587;

        let assign11510_e10590: f64 = (var_cgovaccg_i * 0.005);
        let assign11510_e10592: f64 = (assign11510_e10590 * var_inv_phita);
        let assign11510_e10593: f64 = (assign11510_e10592).exp();
        let assign11510_e10595: f64 = (assign11510_e10593 - 1.0);
        let assign11510_e10596: f64 = (assign11510_e10595).ln();
        let assign11510_e10598: f64 = (assign11510_e10596 / var_cgovaccg_i);
        let assign11510_e10601: f64 = (0.005 * var_inv_phita);
        let assign11510_e10602: f64 = (assign11510_e10601).exp();
        let assign11510_e10604: f64 = (assign11510_e10602 - 1.0);
        let assign11510_e10605: f64 = (assign11510_e10604).ln();
        let assign11510_e10606: f64 = (assign11510_e10598 - assign11510_e10605);
        var_dxgb_ov_th = assign11510_e10606;

        let assign11520_e10609: f64 = (0.5 * var_gov_s);
        let assign11520_e10610: f64 = (assign11520_e10609).ln();
        let assign11520_e10612: f64 = (assign11520_e10610 + var_dxgb_ov_th);
        var_dxgb_ov_s = assign11520_e10612;

        let assign11530_e10615: f64 = (0.5 * var_gov_d);
        let assign11530_e10616: f64 = (assign11530_e10615).ln();
        let assign11530_e10618: f64 = (assign11530_e10616 + var_dxgb_ov_th);
        var_dxgb_ov_d = assign11530_e10618;

        let assign11540_e10621: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11540_e10621;

        let assign11550_e10624: f64 = (3.1 * var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        var_sp_ov_eps = assign11550_e10626;

        let assign11560_e10629: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11560_e10629;

        let assign11570_e10632: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11570_e10632;

        let assign11580_e10635: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard157 = assign11580_e10635;

        let (assign11590_e10641,) = {
    if (var_guard157 != 0.0) {
        let assign11590_e10639: f64 = (64.0 * var_inv_gov);
        (assign11590_e10639,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11590_e10641;

        let assign11600_e10644: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard158 = assign11600_e10644;

        let (assign11610_e10655,) = {
    if ((var_guard157 == 0.0) && (var_guard158 != 0.0)) {
        let assign11610_e10651: f64 = (22.0 * var_inv_gov);
        let assign11610_e10653: f64 = (assign11610_e10651 + 3.0);
        (assign11610_e10653,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11610_e10655;

        let assign11620_e10658: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard159 = assign11620_e10658;

        let (assign11630_e10673,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign11630_e10667: f64 = (-7.2);
        let assign11630_e10669: f64 = (assign11630_e10667 * var_inv_gov);
        let assign11630_e10671: f64 = (assign11630_e10669 + 15.5);
        (assign11630_e10671,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11630_e10673;

        let (assign11640_e10684,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11640_e10684;

        let assign11650_e10688: f64 = (var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        var_sp_ov_delta1_s = assign11650_e10700;

        let assign11660_e10703: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11660_e10703;

        let assign11670_e10706: f64 = (3.1 * var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        var_sp_ov_eps = assign11670_e10708;

        let assign11680_e10711: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11680_e10711;

        let assign11690_e10714: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11690_e10714;

        let assign11700_e10717: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard160 = assign11700_e10717;

        let (assign11710_e10723,) = {
    if (var_guard160 != 0.0) {
        let assign11710_e10721: f64 = (64.0 * var_inv_gov);
        (assign11710_e10721,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11710_e10723;

        let assign11720_e10726: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard161 = assign11720_e10726;

        let (assign11730_e10737,) = {
    if ((var_guard160 == 0.0) && (var_guard161 != 0.0)) {
        let assign11730_e10733: f64 = (22.0 * var_inv_gov);
        let assign11730_e10735: f64 = (assign11730_e10733 + 3.0);
        (assign11730_e10735,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11730_e10737;

        let assign11740_e10740: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard162 = assign11740_e10740;

        let (assign11750_e10755,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 != 0.0)) {
        let assign11750_e10749: f64 = (-7.2);
        let assign11750_e10751: f64 = (assign11750_e10749 * var_inv_gov);
        let assign11750_e10753: f64 = (assign11750_e10751 + 15.5);
        (assign11750_e10753,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11750_e10755;

        let (assign11760_e10766,) = {
    if (((var_guard160 == 0.0) && (var_guard161 == 0.0)) && (var_guard162 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11760_e10766;

        let assign11770_e10770: f64 = (var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        var_sp_ov_delta1_d = assign11770_e10782;

        let assign11780_e10785: f64 = (var_eg + var_dphib_i);
        let assign11780_e10788: f64 = (2.0 * var_phit);
        let assign11780_e10792: f64 = (-0.75);
        let assign11780_e10793: f64 = (var_phibfac).powf(assign11780_e10792);
        let assign11780_e10794: f64 = (var_neff_i * assign11780_e10793);
        let assign11780_e10796: f64 = (assign11780_e10794 * 4e-26);
        let assign11780_e10797: f64 = (assign11780_e10796).ln();
        let assign11780_e10798: f64 = (assign11780_e10788 * assign11780_e10797);
        let assign11780_e10799: f64 = (assign11780_e10785 + assign11780_e10798);
        var_phib_dc = assign11780_e10799;

        let (assign11790_e10805,) = {
    if (var_phib_dc > 0.05) {
        (var_phib_dc,)
    } else {
        (0.05,)
    }
};
        var_phib_dc = assign11790_e10805;

        let assign11800_e10808: f64 = (2.0 * 1.6021918e-19);
        let assign11800_e10810: f64 = (assign11800_e10808 * var_neff_i);
        let assign11800_e10812: f64 = (assign11800_e10810 * var_epssi);
        let assign11800_e10814: f64 = (assign11800_e10812 * var_inv_phit);
        let assign11800_e10815: f64 = (assign11800_e10814).sqrt();
        let assign11800_e10817: f64 = (assign11800_e10815 / var_coxprime);
        var_g_0_dc = assign11800_e10817;

        var_kp = 0.0;

        var_np = 0.0;

        let assign11830_e10822: f64 = if var_np_i > 0.0 { 1.0 } else { 0.0 };
        var_guard163 = assign11830_e10822;

        let (assign11840_e10828,) = {
    if (var_guard163 != 0.0) {
        let assign11840_e10826: f64 = (80000000.0 / var_tox_sq);
        (assign11840_e10826,)
    } else {
        (var_arg2max,)
    }
};
        var_arg2max = assign11840_e10828;

        let (assign11850_e10837,) = {
    if (var_guard163 != 0.0) {
        let (assign11850_e10835,) = {
            if (var_np_i > var_arg2max) {
                (var_np_i,)
            } else {
                (var_arg2max,)
            }
        };
        (assign11850_e10835,)
    } else {
        (var_np,)
    }
};
        var_np = assign11850_e10837;

        *var_agidld_i_slot = var_agidld_i;
        *var_ar_slot = var_ar;
        *var_arac_slot = var_arac;
        *var_arg2max_slot = var_arg2max;
        *var_bgidld_i_slot = var_bgidld_i;
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
        *var_g_0_dc_slot = var_g_0_dc;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov_d_slot = var_gov_d;
        *var_gov_s_slot = var_gov_s;
        *var_guard154_slot = var_guard154;
        *var_guard155_slot = var_guard155;
        *var_guard156_slot = var_guard156;
        *var_guard157_slot = var_guard157;
        *var_guard158_slot = var_guard158;
        *var_guard159_slot = var_guard159;
        *var_guard160_slot = var_guard160;
        *var_guard161_slot = var_guard161;
        *var_guard162_slot = var_guard162;
        *var_guard163_slot = var_guard163;
        *var_igovd_i_slot = var_igovd_i;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_vp_slot = var_inv_vp;
        *var_kp_slot = var_kp;
        *var_neffac_i_slot = var_neffac_i;
        *var_novd_i_slot = var_novd_i;
        *var_np_slot = var_np;
        *var_phib_dc_slot = var_phib_dc;
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
        *var_tox_sq_slot = var_tox_sq;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_a2_i: f64,
        var_betn_i: f64,
        var_betnedge_i: f64,
        var_coxprime: f64,
        var_cs_i: f64,
        var_ct_i: f64,
        var_ctedge_i: f64,
        var_ctg_i: f64,
        var_delt: f64,
        var_delvtac_i: f64,
        var_delvto_i: f64,
        var_delvtoedge_i: f64,
        var_dphib_i: f64,
        var_dphibedge_i: f64,
        var_dvsbnud_i: f64,
        var_eg: f64,
        var_epssi: f64,
        var_factuo_i: f64,
        var_factuoedge_i: f64,
        var_fnt_i: f64,
        var_guard163: f64,
        var_inv_phit: f64,
        var_ln_rtn: f64,
        var_mue_i: f64,
        var_neffac_i: f64,
        var_neffedge_i: f64,
        var_phibfac: f64,
        var_phit: f64,
        var_qq: f64,
        var_rs_i: f64,
        var_rtn: f64,
        var_st2vfb_i: f64,
        var_sta2_i: f64,
        var_stbet_i: f64,
        var_stbetedge_i: f64,
        var_stcs_i: f64,
        var_stct_i: f64,
        var_stmue_i: f64,
        var_strs_i: f64,
        var_stthecs_i: f64,
        var_stthemu_i: f64,
        var_stthesat_i: f64,
        var_stvfb_i: f64,
        var_stvfbedge_i: f64,
        var_stxcor_i: f64,
        var_thecs_i: f64,
        var_themu_i: f64,
        var_thesat_i: f64,
        var_thesatac_i: f64,
        var_tkd: f64,
        var_vfb_i: f64,
        var_vfbedge_i: f64,
        var_vsbnud_i: f64,
        var_xcor_i: f64,
        var_a2_t_slot: &mut f64,
        var_alpha_b_slot: &mut f64,
        var_aphi_ac_slot: &mut f64,
        var_aphi_dc_slot: &mut f64,
        var_aphiedge_slot: &mut f64,
        var_bet_i_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betn_t_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_bphi_ac_slot: &mut f64,
        var_bphi_dc_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_ct_t_slot: &mut f64,
        var_ctg_t_slot: &mut f64,
        var_dphibq_slot: &mut f64,
        var_g_0_ac_slot: &mut f64,
        var_g_0_dc_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_np_slot: &mut f64,
        var_nt_slot: &mut f64,
        var_phib_ac_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phix1_ac_slot: &mut f64,
        var_phix1_dc_slot: &mut f64,
        var_phix2_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phix_ac_slot: &mut f64,
        var_phix_dc_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qlim2_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_sqrt_phib_dc_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_ct_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_ther_i_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_us1_slot: &mut f64,
        var_us21_slot: &mut f64,
        var_vfb_t_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
    ) {
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_aphi_ac: f64 = *var_aphi_ac_slot;
        let mut var_aphi_dc: f64 = *var_aphi_dc_slot;
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_bet_i: f64 = *var_bet_i_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betn_t: f64 = *var_betn_t_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_bphi_ac: f64 = *var_bphi_ac_slot;
        let mut var_bphi_dc: f64 = *var_bphi_dc_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_ct_t: f64 = *var_ct_t_slot;
        let mut var_ctg_t: f64 = *var_ctg_t_slot;
        let mut var_dphibq: f64 = *var_dphibq_slot;
        let mut var_g_0_ac: f64 = *var_g_0_ac_slot;
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_nt: f64 = *var_nt_slot;
        let mut var_phib_ac: f64 = *var_phib_ac_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phix1_ac: f64 = *var_phix1_ac_slot;
        let mut var_phix1_dc: f64 = *var_phix1_dc_slot;
        let mut var_phix2: f64 = *var_phix2_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phix_ac: f64 = *var_phix_ac_slot;
        let mut var_phix_dc: f64 = *var_phix_dc_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qlim2: f64 = *var_qlim2_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_sqrt_phib_dc: f64 = *var_sqrt_phib_dc_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_ct: f64 = *var_tf_ct_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_ther_i: f64 = *var_ther_i_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_us1: f64 = *var_us1_slot;
        let mut var_us21: f64 = *var_us21_slot;
        let mut var_vfb_t: f64 = *var_vfb_t_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;

        let (assign11860_e10846,) = {
    if (var_guard163 != 0.0) {
        let (assign11860_e10844,) = {
            if (5e24 > var_np) {
                (5e24,)
            } else {
                (var_np,)
            }
        };
        (assign11860_e10844,)
    } else {
        (var_np,)
    }
};
        var_np = assign11860_e10846;

        let (assign11870_e10862,) = {
    if (var_guard163 != 0.0) {
        let assign11870_e10850: f64 = (2.0 * var_coxprime);
        let assign11870_e10852: f64 = (assign11870_e10850 * var_coxprime);
        let assign11870_e10854: f64 = (assign11870_e10852 * var_phit);
        let assign11870_e10857: f64 = (1.6021918e-19 * var_np);
        let assign11870_e10859: f64 = (assign11870_e10857 * var_epssi);
        let assign11870_e10860: f64 = (assign11870_e10854 / assign11870_e10859);
        (assign11870_e10860,)
    } else {
        (var_kp,)
    }
};
        var_kp = assign11870_e10862;

        let assign11880_e10865: f64 = (100.0 * var_phit);
        let assign11880_e10867: f64 = (assign11880_e10865 * var_phit);
        var_qlim2 = assign11880_e10867;

        let assign11890_e10870: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard164 = assign11890_e10870;

        let (assign11900_e10881,) = {
    if (var_guard164 != 0.0) {
        let assign11900_e10874: f64 = (var_phit * var_g_0_dc);
        let assign11900_e10876: f64 = (assign11900_e10874 * var_g_0_dc);
        let assign11900_e10878: f64 = (assign11900_e10876 * var_phib_dc);
        let assign11900_e10879: f64 = (assign11900_e10878).sqrt();
        (assign11900_e10879,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign11900_e10881;

        let (assign11910_e10891,) = {
    if (var_guard164 != 0.0) {
        let assign11910_e10885: f64 = (0.75 * var_qq);
        let assign11910_e10888: f64 = (var_qb0).powf(0.6666666666666666);
        let assign11910_e10889: f64 = (assign11910_e10885 * assign11910_e10888);
        (assign11910_e10889,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign11910_e10891;

        let (assign11920_e10897,) = {
    if (var_guard164 != 0.0) {
        let assign11920_e10895: f64 = (var_phib_dc + var_dphibq);
        (assign11920_e10895,)
    } else {
        (var_phib_dc,)
    }
};
        var_phib_dc = assign11920_e10897;

        let (assign11930_e10911,) = {
    if (var_guard164 != 0.0) {
        let assign11930_e10903: f64 = (2.0 * 0.6666666666666666);
        let assign11930_e10905: f64 = (assign11930_e10903 * var_dphibq);
        let assign11930_e10907: f64 = (assign11930_e10905 / var_qb0);
        let assign11930_e10908: f64 = (1.0 + assign11930_e10907);
        let assign11930_e10909: f64 = (var_g_0_dc * assign11930_e10908);
        (assign11930_e10909,)
    } else {
        (var_g_0_dc,)
    }
};
        var_g_0_dc = assign11930_e10911;

        let assign11940_e10913: f64 = (var_phib_dc).sqrt();
        var_sqrt_phib_dc = assign11940_e10913;

        let assign11950_e10916: f64 = (0.95 * var_phib_dc);
        var_phix_dc = assign11950_e10916;

        let assign11960_e10919: f64 = (0.0025 * var_phib_dc);
        let assign11960_e10921: f64 = (assign11960_e10919 * var_phib_dc);
        var_aphi_dc = assign11960_e10921;

        var_bphi_dc = var_aphi_dc;

        let assign11980_e10925: f64 = (var_bphi_dc).sqrt();
        let assign11980_e10926: f64 = (0.5 * assign11980_e10925);
        var_phix2 = assign11980_e10926;

        let assign11990_e10930: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10932: f64 = assign11990_e10930;
        let assign11990_e10935: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10937: f64 = assign11990_e10935;
        let assign11990_e10940: f64 = (var_phix_dc - var_phix2);
        let assign11990_e10942: f64 = assign11990_e10940;
        let assign11990_e10943: f64 = (assign11990_e10937 * assign11990_e10942);
        let assign11990_e10945: f64 = (assign11990_e10943 + var_aphi_dc);
        let assign11990_e10946: f64 = (assign11990_e10945).sqrt();
        let assign11990_e10947: f64 = (assign11990_e10932 - assign11990_e10946);
        let assign11990_e10948: f64 = (0.5 * assign11990_e10947);
        var_phix1_dc = assign11990_e10948;

        let assign12000_e10952: f64 = (var_phib_dc + var_eg);
        let assign12000_e10953: f64 = (0.5 * assign12000_e10952);
        var_alpha_b = assign12000_e10953;

        let assign12010_e10956: f64 = (var_vsbnud_i + var_phib_dc);
        let assign12010_e10957: f64 = (assign12010_e10956).sqrt();
        let assign12010_e10959: f64 = (assign12010_e10957 - var_sqrt_phib_dc);
        var_us1 = assign12010_e10959;

        let assign12020_e10962: f64 = (var_vsbnud_i + var_dvsbnud_i);
        let assign12020_e10964: f64 = (assign12020_e10962 + var_phib_dc);
        let assign12020_e10965: f64 = (assign12020_e10964).sqrt();
        let assign12020_e10967: f64 = (assign12020_e10965 - var_sqrt_phib_dc);
        let assign12020_e10969: f64 = (assign12020_e10967 - var_us1);
        var_us21 = assign12020_e10969;

        let assign12030_e10972: f64 = (var_eg + var_dphib_i);
        let assign12030_e10974: f64 = (assign12030_e10972 + var_delvtac_i);
        let assign12030_e10977: f64 = (2.0 * var_phit);
        let assign12030_e10981: f64 = (-0.75);
        let assign12030_e10982: f64 = (var_phibfac).powf(assign12030_e10981);
        let assign12030_e10983: f64 = (var_neffac_i * assign12030_e10982);
        let assign12030_e10985: f64 = (assign12030_e10983 * 4e-26);
        let assign12030_e10986: f64 = (assign12030_e10985).ln();
        let assign12030_e10987: f64 = (assign12030_e10977 * assign12030_e10986);
        let assign12030_e10988: f64 = (assign12030_e10974 + assign12030_e10987);
        var_phib_ac = assign12030_e10988;

        let (assign12040_e10994,) = {
    if (var_phib_ac > 0.05) {
        (var_phib_ac,)
    } else {
        (0.05,)
    }
};
        var_phib_ac = assign12040_e10994;

        let assign12050_e10997: f64 = (2.0 * 1.6021918e-19);
        let assign12050_e10999: f64 = (assign12050_e10997 * var_neffac_i);
        let assign12050_e11001: f64 = (assign12050_e10999 * var_epssi);
        let assign12050_e11003: f64 = (assign12050_e11001 * var_inv_phit);
        let assign12050_e11004: f64 = (assign12050_e11003).sqrt();
        let assign12050_e11006: f64 = (assign12050_e11004 / var_coxprime);
        var_g_0_ac = assign12050_e11006;

        let assign12060_e11009: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        var_guard165 = assign12060_e11009;

        let (assign12070_e11020,) = {
    if (var_guard165 != 0.0) {
        let assign12070_e11013: f64 = (var_phit * var_g_0_ac);
        let assign12070_e11015: f64 = (assign12070_e11013 * var_g_0_ac);
        let assign12070_e11017: f64 = (assign12070_e11015 * var_phib_ac);
        let assign12070_e11018: f64 = (assign12070_e11017).sqrt();
        (assign12070_e11018,)
    } else {
        (var_qb0,)
    }
};
        var_qb0 = assign12070_e11020;

        let (assign12080_e11030,) = {
    if (var_guard165 != 0.0) {
        let assign12080_e11024: f64 = (0.75 * var_qq);
        let assign12080_e11027: f64 = (var_qb0).powf(0.6666666666666666);
        let assign12080_e11028: f64 = (assign12080_e11024 * assign12080_e11027);
        (assign12080_e11028,)
    } else {
        (var_dphibq,)
    }
};
        var_dphibq = assign12080_e11030;

        let (assign12090_e11036,) = {
    if (var_guard165 != 0.0) {
        let assign12090_e11034: f64 = (var_phib_ac + var_dphibq);
        (assign12090_e11034,)
    } else {
        (var_phib_ac,)
    }
};
        var_phib_ac = assign12090_e11036;

        let (assign12100_e11050,) = {
    if (var_guard165 != 0.0) {
        let assign12100_e11042: f64 = (2.0 * 0.6666666666666666);
        let assign12100_e11044: f64 = (assign12100_e11042 * var_dphibq);
        let assign12100_e11046: f64 = (assign12100_e11044 / var_qb0);
        let assign12100_e11047: f64 = (1.0 + assign12100_e11046);
        let assign12100_e11048: f64 = (var_g_0_ac * assign12100_e11047);
        (assign12100_e11048,)
    } else {
        (var_g_0_ac,)
    }
};
        var_g_0_ac = assign12100_e11050;

        let assign12110_e11053: f64 = (0.95 * var_phib_ac);
        var_phix_ac = assign12110_e11053;

        let assign12120_e11056: f64 = (0.0025 * var_phib_ac);
        let assign12120_e11058: f64 = (assign12120_e11056 * var_phib_ac);
        var_aphi_ac = assign12120_e11058;

        var_bphi_ac = var_aphi_ac;

        let assign12140_e11062: f64 = (var_bphi_ac).sqrt();
        let assign12140_e11063: f64 = (0.5 * assign12140_e11062);
        var_phix2 = assign12140_e11063;

        let assign12150_e11067: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11069: f64 = assign12150_e11067;
        let assign12150_e11072: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11074: f64 = assign12150_e11072;
        let assign12150_e11077: f64 = (var_phix_ac - var_phix2);
        let assign12150_e11079: f64 = assign12150_e11077;
        let assign12150_e11080: f64 = (assign12150_e11074 * assign12150_e11079);
        let assign12150_e11082: f64 = (assign12150_e11080 + var_aphi_ac);
        let assign12150_e11083: f64 = (assign12150_e11082).sqrt();
        let assign12150_e11084: f64 = (assign12150_e11069 - assign12150_e11083);
        let assign12150_e11085: f64 = (0.5 * assign12150_e11084);
        var_phix1_ac = assign12150_e11085;

        let assign12160_e11089: f64 = (var_stvfb_i * var_delt);
        let assign12160_e11093: f64 = (var_st2vfb_i * var_delt);
        let assign12160_e11094: f64 = (1.0 + assign12160_e11093);
        let assign12160_e11095: f64 = (assign12160_e11089 * assign12160_e11094);
        let assign12160_e11096: f64 = (var_vfb_i + assign12160_e11095);
        let assign12160_e11098: f64 = (assign12160_e11096 + var_delvto_i);
        var_vfb_t = assign12160_e11098;

        let assign12170_e11101: f64 = (var_stct_i * var_ln_rtn);
        let assign12170_e11102: f64 = (assign12170_e11101).exp();
        var_tf_ct = assign12170_e11102;

        let assign12180_e11105: f64 = (var_ct_i * var_tf_ct);
        var_ct_t = assign12180_e11105;

        let assign12190_e11108: f64 = (var_ctg_i / var_rtn);
        var_ctg_t = assign12190_e11108;

        let assign12200_e11111: f64 = (var_stbet_i * var_ln_rtn);
        let assign12200_e11112: f64 = (assign12200_e11111).exp();
        var_tf_bet = assign12200_e11112;

        let assign12210_e11115: f64 = (var_betn_i * var_tf_bet);
        var_betn_t = assign12210_e11115;

        let assign12220_e11118: f64 = (var_factuo_i * var_betn_t);
        let assign12220_e11120: f64 = (assign12220_e11118 * var_coxprime);
        var_bet_i = assign12220_e11120;

        let assign12230_e11124: f64 = (var_stthemu_i * var_ln_rtn);
        let assign12230_e11125: f64 = (assign12230_e11124).exp();
        let assign12230_e11126: f64 = (var_themu_i * assign12230_e11125);
        var_themu_t = assign12230_e11126;

        let assign12240_e11129: f64 = (var_stmue_i * var_ln_rtn);
        let assign12240_e11130: f64 = (assign12240_e11129).exp();
        var_tf_mue = assign12240_e11130;

        let assign12250_e11133: f64 = (var_mue_i * var_tf_mue);
        var_mue_t = assign12250_e11133;

        let assign12260_e11137: f64 = (var_stthecs_i * var_ln_rtn);
        let assign12260_e11138: f64 = (assign12260_e11137).exp();
        let assign12260_e11139: f64 = (var_thecs_i * assign12260_e11138);
        var_thecs_t = assign12260_e11139;

        let assign12270_e11142: f64 = (var_stcs_i * var_ln_rtn);
        let assign12270_e11143: f64 = (assign12270_e11142).exp();
        var_tf_cs = assign12270_e11143;

        let assign12280_e11146: f64 = (var_cs_i * var_tf_cs);
        var_cs_t = assign12280_e11146;

        let assign12290_e11149: f64 = (var_stxcor_i * var_ln_rtn);
        let assign12290_e11150: f64 = (assign12290_e11149).exp();
        var_tf_xcor = assign12290_e11150;

        let assign12300_e11153: f64 = (var_xcor_i * var_tf_xcor);
        var_xcor_t = assign12300_e11153;

        let assign12310_e11156: f64 = (var_strs_i * var_ln_rtn);
        let assign12310_e11157: f64 = (assign12310_e11156).exp();
        var_tf_ther = assign12310_e11157;

        let assign12320_e11160: f64 = (var_rs_i * var_tf_ther);
        var_rs_t = assign12320_e11160;

        let assign12330_e11163: f64 = (2.0 * var_bet_i);
        let assign12330_e11165: f64 = (assign12330_e11163 * var_rs_t);
        var_ther_i = assign12330_e11165;

        let assign12340_e11168: f64 = (var_stthesat_i * var_ln_rtn);
        let assign12340_e11169: f64 = (assign12340_e11168).exp();
        var_tf_thesat = assign12340_e11169;

        let assign12350_e11172: f64 = (var_thesat_i * var_tf_thesat);
        var_thesat_t = assign12350_e11172;

        let assign12360_e11175: f64 = (var_thesatac_i * var_tf_thesat);
        var_thesatac_t = assign12360_e11175;

        let assign12370_e11178: f64 = (-var_sta2_i);
        let assign12370_e11180: f64 = (assign12370_e11178 * var_ln_rtn);
        let assign12370_e11181: f64 = (assign12370_e11180).exp();
        let assign12370_e11182: f64 = (var_a2_i * assign12370_e11181);
        var_a2_t = assign12370_e11182;

        let assign12380_e11185: f64 = (var_fnt_i * 4.0);
        let assign12380_e11187: f64 = (assign12380_e11185 * 1.3806505e-23);
        let assign12380_e11189: f64 = (assign12380_e11187 * var_tkd);
        var_nt = assign12380_e11189;

        let assign12400_e11203: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard166 = assign12400_e11203;

        let (assign12410_e11213,) = {
    if (var_guard166 != 0.0) {
        let assign12410_e11208: f64 = (var_stvfbedge_i * var_delt);
        let assign12410_e11209: f64 = (var_vfbedge_i + assign12410_e11208);
        let assign12410_e11211: f64 = (assign12410_e11209 + var_delvtoedge_i);
        (assign12410_e11211,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12410_e11213;

        let (assign12420_e11220,) = {
    if (var_guard166 != 0.0) {
        let assign12420_e11217: f64 = (var_stbetedge_i * var_ln_rtn);
        let assign12420_e11218: f64 = (assign12420_e11217).exp();
        (assign12420_e11218,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12420_e11220;

        let (assign12430_e11226,) = {
    if (var_guard166 != 0.0) {
        let assign12430_e11224: f64 = (var_betnedge_i * var_tf_betedge);
        (assign12430_e11224,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12430_e11226;

        let (assign12440_e11234,) = {
    if (var_guard166 != 0.0) {
        let assign12440_e11230: f64 = (var_factuoedge_i * var_betnedge_t);
        let assign12440_e11232: f64 = (assign12440_e11230 * var_coxprime);
        (assign12440_e11232,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12440_e11234;

        let (assign12450_e11244,) = {
    if (var_guard166 != 0.0) {
        let assign12450_e11240: f64 = (var_ctedge_i * var_rtn);
        let assign12450_e11241: f64 = (1.0 + assign12450_e11240);
        let assign12450_e11242: f64 = (var_phit * assign12450_e11241);
        (assign12450_e11242,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12450_e11244;

        let (assign12460_e11264,) = {
    if (var_guard166 != 0.0) {
        let assign12460_e11248: f64 = (var_eg + var_dphibedge_i);
        let assign12460_e11251: f64 = (2.0 * var_phit0edge);
        let assign12460_e11255: f64 = (-0.75);
        let assign12460_e11256: f64 = (var_phibfac).powf(assign12460_e11255);
        let assign12460_e11257: f64 = (var_neffedge_i * assign12460_e11256);
        let assign12460_e11259: f64 = (assign12460_e11257 * 4e-26);
        let assign12460_e11260: f64 = (assign12460_e11259).ln();
        let assign12460_e11261: f64 = (assign12460_e11251 * assign12460_e11260);
        let assign12460_e11262: f64 = (assign12460_e11248 + assign12460_e11261);
        (assign12460_e11262,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12460_e11264;

        let (assign12470_e11273,) = {
    if (var_guard166 != 0.0) {
        let (assign12470_e11271,) = {
            if (var_phibedge > 0.05) {
                (var_phibedge,)
            } else {
                (0.05,)
            }
        };
        (assign12470_e11271,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12470_e11273;

        let (assign12480_e11288,) = {
    if (var_guard166 != 0.0) {
        let assign12480_e11277: f64 = (2.0 * 1.6021918e-19);
        let assign12480_e11279: f64 = (assign12480_e11277 * var_neffedge_i);
        let assign12480_e11281: f64 = (assign12480_e11279 * var_epssi);
        let assign12480_e11283: f64 = (assign12480_e11281 * var_inv_phit);
        let assign12480_e11284: f64 = (assign12480_e11283).sqrt();
        let assign12480_e11286: f64 = (assign12480_e11284 / var_coxprime);
        (assign12480_e11286,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12480_e11288;

        let (assign12490_e11294,) = {
    if (var_guard166 != 0.0) {
        let assign12490_e11292: f64 = (var_gfedge * var_gfedge);
        (assign12490_e11292,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12490_e11294;

        let (assign12500_e11299,) = {
    if (var_guard166 != 0.0) {
        let assign12500_e11297: f64 = (var_gfedge2).ln();
        (assign12500_e11297,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12500_e11299;

        let (assign12510_e11305,) = {
    if (var_guard166 != 0.0) {
        let assign12510_e11303: f64 = (0.95 * var_phibedge);
        (assign12510_e11303,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12510_e11305;

        let (assign12520_e11313,) = {
    if (var_guard166 != 0.0) {
        let assign12520_e11309: f64 = (0.0025 * var_phibedge);
        let assign12520_e11311: f64 = (assign12520_e11309 * var_phibedge);
        (assign12520_e11311,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12520_e11313;

        let (assign12530_e11317,) = {
    if (var_guard166 != 0.0) {
        (var_aphiedge,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12530_e11317;

        let (assign12540_e11324,) = {
    if (var_guard166 != 0.0) {
        let assign12540_e11321: f64 = (var_bphiedge).sqrt();
        let assign12540_e11322: f64 = (0.5 * assign12540_e11321);
        (assign12540_e11322,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12540_e11324;

        *var_a2_t_slot = var_a2_t;
        *var_alpha_b_slot = var_alpha_b;
        *var_aphi_ac_slot = var_aphi_ac;
        *var_aphi_dc_slot = var_aphi_dc;
        *var_aphiedge_slot = var_aphiedge;
        *var_bet_i_slot = var_bet_i;
        *var_betedge_i_slot = var_betedge_i;
        *var_betn_t_slot = var_betn_t;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_bphi_ac_slot = var_bphi_ac;
        *var_bphi_dc_slot = var_bphi_dc;
        *var_bphiedge_slot = var_bphiedge;
        *var_cs_t_slot = var_cs_t;
        *var_ct_t_slot = var_ct_t;
        *var_ctg_t_slot = var_ctg_t;
        *var_dphibq_slot = var_dphibq;
        *var_g_0_ac_slot = var_g_0_ac;
        *var_g_0_dc_slot = var_g_0_dc;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_guard164_slot = var_guard164;
        *var_guard165_slot = var_guard165;
        *var_guard166_slot = var_guard166;
        *var_kp_slot = var_kp;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_mue_t_slot = var_mue_t;
        *var_np_slot = var_np;
        *var_nt_slot = var_nt;
        *var_phib_ac_slot = var_phib_ac;
        *var_phib_dc_slot = var_phib_dc;
        *var_phibedge_slot = var_phibedge;
        *var_phit0edge_slot = var_phit0edge;
        *var_phix1_ac_slot = var_phix1_ac;
        *var_phix1_dc_slot = var_phix1_dc;
        *var_phix2_slot = var_phix2;
        *var_phix2edge_slot = var_phix2edge;
        *var_phix_ac_slot = var_phix_ac;
        *var_phix_dc_slot = var_phix_dc;
        *var_phixedge_slot = var_phixedge;
        *var_qb0_slot = var_qb0;
        *var_qlim2_slot = var_qlim2;
        *var_rs_t_slot = var_rs_t;
        *var_sqrt_phib_dc_slot = var_sqrt_phib_dc;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_ct_slot = var_tf_ct;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_thecs_t_slot = var_thecs_t;
        *var_themu_t_slot = var_themu_t;
        *var_ther_i_slot = var_ther_i;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_us1_slot = var_us1;
        *var_us21_slot = var_us21;
        *var_vfb_t_slot = var_vfb_t;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_xcor_t_slot = var_xcor_t;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_agidl_i: f64,
        var_agidld_i: f64,
        var_axinr_i: f64,
        var_bgidl_i: f64,
        var_bgidld_i: f64,
        var_chib_i: f64,
        var_delta: f64,
        var_fcinracc_i: f64,
        var_fntexc_i: f64,
        var_gc2_i: f64,
        var_gc2ov_i: f64,
        var_gc2ovd_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_gc3ovd_i: f64,
        var_guard166: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_phit: f64,
        var_rbulk_i: f64,
        var_rde_i: f64,
        var_rg_i: f64,
        var_rjund_i: f64,
        var_rjuns_i: f64,
        var_rse_i: f64,
        var_rta: f64,
        var_rwell_i: f64,
        var_stbgidl_i: f64,
        var_stbgidld_i: f64,
        var_stig_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_agidlds_slot: &mut f64,
        var_agidls_slot: &mut f64,
        var_ainr_slot: &mut f64,
        var_aphiedge_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidlds_slot: &mut f64,
        var_bgidls_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_d_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_fac_exc_slot: &mut f64,
        var_gbulk_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcqov_slot: &mut f64,
        var_gcqovd_slot: &mut f64,
        var_gdrain_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_ggate_slot: &mut f64,
        var_gjund_slot: &mut f64,
        var_gjuns_slot: &mut f64,
        var_gsource_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_gwell_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phix1edge_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vinr_max_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_agidlds: f64 = *var_agidlds_slot;
        let mut var_agidls: f64 = *var_agidls_slot;
        let mut var_ainr: f64 = *var_ainr_slot;
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidlds: f64 = *var_bgidlds_slot;
        let mut var_bgidls: f64 = *var_bgidls_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_d: f64 = *var_bov_d_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_fac_exc: f64 = *var_fac_exc_slot;
        let mut var_gbulk: f64 = *var_gbulk_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcqov: f64 = *var_gcqov_slot;
        let mut var_gcqovd: f64 = *var_gcqovd_slot;
        let mut var_gdrain: f64 = *var_gdrain_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_ggate: f64 = *var_ggate_slot;
        let mut var_gjund: f64 = *var_gjund_slot;
        let mut var_gjuns: f64 = *var_gjuns_slot;
        let mut var_gsource: f64 = *var_gsource_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_gwell: f64 = *var_gwell_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phix1edge: f64 = *var_phix1edge_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vinr_max: f64 = *var_vinr_max_slot;

        let (assign12550_e11349,) = {
    if (var_guard166 != 0.0) {
        let assign12550_e11329: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11331: f64 = assign12550_e11329;
        let assign12550_e11334: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11336: f64 = assign12550_e11334;
        let assign12550_e11339: f64 = (var_phixedge - var_phix2edge);
        let assign12550_e11341: f64 = assign12550_e11339;
        let assign12550_e11342: f64 = (assign12550_e11336 * assign12550_e11341);
        let assign12550_e11344: f64 = (assign12550_e11342 + var_aphiedge);
        let assign12550_e11345: f64 = (assign12550_e11344).sqrt();
        let assign12550_e11346: f64 = (assign12550_e11331 - assign12550_e11345);
        let assign12550_e11347: f64 = (0.5 * assign12550_e11346);
        (assign12550_e11347,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12550_e11349;

        let (assign12580_e11374,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_vfbedge_t,)
    }
};
        var_vfbedge_t = assign12580_e11374;

        let (assign12590_e11379,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_tf_betedge,)
    }
};
        var_tf_betedge = assign12590_e11379;

        let (assign12600_e11384,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_betnedge_t,)
    }
};
        var_betnedge_t = assign12600_e11384;

        let (assign12610_e11389,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_betedge_i,)
    }
};
        var_betedge_i = assign12610_e11389;

        let (assign12620_e11394,) = {
    if (var_guard166 == 0.0) {
        (var_phit,)
    } else {
        (var_phit0edge,)
    }
};
        var_phit0edge = assign12620_e11394;

        let (assign12630_e11399,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phibedge,)
    }
};
        var_phibedge = assign12630_e11399;

        let (assign12640_e11404,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge,)
    }
};
        var_gfedge = assign12640_e11404;

        let (assign12650_e11409,) = {
    if (var_guard166 == 0.0) {
        (1.0,)
    } else {
        (var_gfedge2,)
    }
};
        var_gfedge2 = assign12650_e11409;

        let (assign12660_e11414,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_lngfedge2,)
    }
};
        var_lngfedge2 = assign12660_e11414;

        let (assign12670_e11419,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phixedge,)
    }
};
        var_phixedge = assign12670_e11419;

        let (assign12680_e11424,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_aphiedge,)
    }
};
        var_aphiedge = assign12680_e11424;

        let (assign12690_e11429,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_bphiedge,)
    }
};
        var_bphiedge = assign12690_e11429;

        let (assign12700_e11434,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phix2edge,)
    }
};
        var_phix2edge = assign12700_e11434;

        let (assign12710_e11439,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_phix1edge,)
    }
};
        var_phix1edge = assign12710_e11439;

        let assign12740_e11452: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign12740_e11452;

        let assign12750_e11455: f64 = (4.0 * 0.3333333333333333);
        let assign12750_e11458: f64 = (2.0 * 1.6021918e-19);
        let assign12750_e11460: f64 = (assign12750_e11458 * 9.1093826e-31);
        let assign12750_e11462: f64 = (assign12750_e11460 * var_chib_i);
        let assign12750_e11463: f64 = (assign12750_e11462).sqrt();
        let assign12750_e11464: f64 = (assign12750_e11455 * assign12750_e11463);
        let assign12750_e11466: f64 = (assign12750_e11464 / 1.05457168e-34);
        var_b_fact = assign12750_e11466;

        let assign12760_e11469: f64 = (var_b_fact * var_tox_i);
        var_bch = assign12760_e11469;

        let assign12770_e11472: f64 = (var_b_fact * var_toxov_i);
        var_bov = assign12770_e11472;

        let assign12780_e11475: f64 = (var_b_fact * var_toxovd_i);
        var_bov_d = assign12780_e11475;

        var_gcq = 0.0;

        let assign12800_e11479: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard167 = assign12800_e11479;

        let (assign12810_e11488,) = {
    if (var_guard167 != 0.0) {
        let assign12810_e11482: f64 = (-0.495);
        let assign12810_e11484: f64 = (assign12810_e11482 * var_gc2_i);
        let assign12810_e11486: f64 = (assign12810_e11484 / var_gc3_i);
        (assign12810_e11486,)
    } else {
        (var_gcq,)
    }
};
        var_gcq = assign12810_e11488;

        var_gcqov = 0.0;

        let assign12830_e11492: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard168 = assign12830_e11492;

        let (assign12840_e11501,) = {
    if (var_guard168 != 0.0) {
        let assign12840_e11495: f64 = (-0.495);
        let assign12840_e11497: f64 = (assign12840_e11495 * var_gc2ov_i);
        let assign12840_e11499: f64 = (assign12840_e11497 / var_gc3ov_i);
        (assign12840_e11499,)
    } else {
        (var_gcqov,)
    }
};
        var_gcqov = assign12840_e11501;

        let assign12850_e11504: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard169 = assign12850_e11504;

        let (assign12860_e11513,) = {
    if (var_guard169 != 0.0) {
        let assign12860_e11507: f64 = (-0.495);
        let assign12860_e11509: f64 = (assign12860_e11507 * var_gc2ovd_i);
        let assign12860_e11511: f64 = (assign12860_e11509 / var_gc3ovd_i);
        (assign12860_e11511,)
    } else {
        (var_gcqovd,)
    }
};
        var_gcqovd = assign12860_e11513;

        let assign12870_e11516: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign12870_e11516;

        let assign12880_e11519: f64 = (var_iginv_i * var_tf_ig);
        var_iginv_i = assign12880_e11519;

        let assign12890_e11522: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign12890_e11522;

        let assign12900_e11525: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign12900_e11525;

        let assign12910_e11528: f64 = (var_agidl_i * 4e-18);
        let assign12910_e11531: f64 = (var_toxov_i * var_toxov_i);
        let assign12910_e11532: f64 = (assign12910_e11528 / assign12910_e11531);
        var_agidls = assign12910_e11532;

        let assign12920_e11535: f64 = (var_agidld_i * 4e-18);
        let assign12920_e11538: f64 = (var_toxovd_i * var_toxovd_i);
        let assign12920_e11539: f64 = (assign12920_e11535 / assign12920_e11538);
        var_agidlds = assign12920_e11539;

        let assign12930_e11543: f64 = (var_stbgidl_i * var_delta);
        let assign12930_e11544: f64 = (1.0 + assign12930_e11543);
        let (assign12930_e11553,) = {
    if (assign12930_e11544 > 0.0) {
        let assign12930_e11550: f64 = (var_stbgidl_i * var_delta);
        let assign12930_e11551: f64 = (1.0 + assign12930_e11550);
        (assign12930_e11551,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12930_e11553;

        let assign12940_e11556: f64 = (var_bgidl_i * var_b_fact);
        var_bgidl_t = assign12940_e11556;

        let assign12950_e11559: f64 = (var_bgidl_t * var_toxov_i);
        let assign12950_e11561: f64 = (assign12950_e11559 * 500000000.0);
        var_bgidls = assign12950_e11561;

        let assign12960_e11565: f64 = (var_stbgidld_i * var_delta);
        let assign12960_e11566: f64 = (1.0 + assign12960_e11565);
        let (assign12960_e11575,) = {
    if (assign12960_e11566 > 0.0) {
        let assign12960_e11572: f64 = (var_stbgidld_i * var_delta);
        let assign12960_e11573: f64 = (1.0 + assign12960_e11572);
        (assign12960_e11573,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign12960_e11575;

        let assign12970_e11578: f64 = (var_bgidld_i * var_b_fact);
        var_bgidld_t = assign12970_e11578;

        let assign12980_e11581: f64 = (var_bgidld_t * var_toxovd_i);
        let assign12980_e11583: f64 = (assign12980_e11581 * 500000000.0);
        var_bgidlds = assign12980_e11583;

        var_vinr_max = 0.0;

        let assign13000_e11587: f64 = if var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        var_guard170 = assign13000_e11587;

        let (assign13010_e11593,) = {
    if (var_guard170 != 0.0) {
        let assign13010_e11591: f64 = (0.75 / var_fcinracc_i);
        (assign13010_e11591,)
    } else {
        (var_vinr_max,)
    }
};
        var_vinr_max = assign13010_e11593;

        let assign13020_e11596: f64 = (var_axinr_i * var_axinr_i);
        var_ainr = assign13020_e11596;

        let assign13030_e11599: f64 = (9.1093826e-31 * 1000000000.0);
        let assign13030_e11601: f64 = (assign13030_e11599 * var_fntexc_i);
        var_fac_exc = assign13030_e11601;

        let assign13040_e11604: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard171 = assign13040_e11604;

        let (assign13050_e11610,) = {
    if (var_guard171 != 0.0) {
        let assign13050_e11608: f64 = (1.0 / var_rg_i);
        (assign13050_e11608,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign13050_e11610;

        let (assign13060_e11615,) = {
    if (var_guard171 == 0.0) {
        (0.0,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign13060_e11615;

        let assign13070_e11618: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard172 = assign13070_e11618;

        let (assign13080_e11624,) = {
    if (var_guard172 != 0.0) {
        let assign13080_e11622: f64 = (1.0 / var_rse_i);
        (assign13080_e11622,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign13080_e11624;

        let (assign13090_e11629,) = {
    if (var_guard172 == 0.0) {
        (0.0,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign13090_e11629;

        let assign13100_e11632: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard173 = assign13100_e11632;

        let (assign13110_e11638,) = {
    if (var_guard173 != 0.0) {
        let assign13110_e11636: f64 = (1.0 / var_rde_i);
        (assign13110_e11636,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign13110_e11638;

        let (assign13120_e11643,) = {
    if (var_guard173 == 0.0) {
        (0.0,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign13120_e11643;

        let assign13130_e11646: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard174 = assign13130_e11646;

        let (assign13140_e11652,) = {
    if (var_guard174 != 0.0) {
        let assign13140_e11650: f64 = (1.0 / var_rbulk_i);
        (assign13140_e11650,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign13140_e11652;

        let (assign13150_e11657,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign13150_e11657;

        let assign13160_e11660: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign13160_e11660;

        let (assign13170_e11666,) = {
    if (var_guard175 != 0.0) {
        let assign13170_e11664: f64 = (1.0 / var_rjuns_i);
        (assign13170_e11664,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign13170_e11666;

        let (assign13180_e11671,) = {
    if (var_guard175 == 0.0) {
        (0.0,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign13180_e11671;

        let assign13190_e11674: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign13190_e11674;

        let (assign13200_e11680,) = {
    if (var_guard176 != 0.0) {
        let assign13200_e11678: f64 = (1.0 / var_rjund_i);
        (assign13200_e11678,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign13200_e11680;

        let (assign13210_e11685,) = {
    if (var_guard176 == 0.0) {
        (0.0,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign13210_e11685;

        let assign13220_e11688: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign13220_e11688;

        let (assign13230_e11694,) = {
    if (var_guard177 != 0.0) {
        let assign13230_e11692: f64 = (1.0 / var_rwell_i);
        (assign13230_e11692,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign13230_e11694;

        let (assign13240_e11699,) = {
    if (var_guard177 == 0.0) {
        (0.0,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign13240_e11699;

        let assign13250_e11702: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign13250_e11702;

        let assign13260_e11705: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign13260_e11705;

        let assign13270_e11708: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign13270_e11708;

        let assign13280_e11711: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign13280_e11711;

        let assign13290_e11714: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign13290_e11714;

        let assign13300_e11717: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign13300_e11717;

        var_jwcorr = 0.0;

        let assign13320_e11721: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13320_e11721;

        let (assign13330_e11725,) = {
    if (var_guard178 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign13330_e11725;

        var_jww = var_we;

        let assign13350_e11729: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard179 = assign13350_e11729;

        let (assign13360_e11738,) = {
    if (var_guard179 != 0.0) {
        let (assign13360_e11736,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign13360_e11736,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign13360_e11738;

        let assign13370_e11745: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard180 = assign13370_e11745;

        *var_abd_i_slot = var_abd_i;
        *var_abs_i_slot = var_abs_i;
        *var_agidlds_slot = var_agidlds;
        *var_agidls_slot = var_agidls;
        *var_ainr_slot = var_ainr;
        *var_aphiedge_slot = var_aphiedge;
        *var_b_fact_slot = var_b_fact;
        *var_bch_slot = var_bch;
        *var_betedge_i_slot = var_betedge_i;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidlds_slot = var_bgidlds;
        *var_bgidls_slot = var_bgidls;
        *var_bov_slot = var_bov;
        *var_bov_d_slot = var_bov_d;
        *var_bphiedge_slot = var_bphiedge;
        *var_fac_exc_slot = var_fac_exc;
        *var_gbulk_slot = var_gbulk;
        *var_gcq_slot = var_gcq;
        *var_gcqov_slot = var_gcqov;
        *var_gcqovd_slot = var_gcqovd;
        *var_gdrain_slot = var_gdrain;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_ggate_slot = var_ggate;
        *var_gjund_slot = var_gjund;
        *var_gjuns_slot = var_gjuns;
        *var_gsource_slot = var_gsource;
        *var_guard167_slot = var_guard167;
        *var_guard168_slot = var_guard168;
        *var_guard169_slot = var_guard169;
        *var_guard170_slot = var_guard170;
        *var_guard171_slot = var_guard171;
        *var_guard172_slot = var_guard172;
        *var_guard173_slot = var_guard173;
        *var_guard174_slot = var_guard174;
        *var_guard175_slot = var_guard175;
        *var_guard176_slot = var_guard176;
        *var_guard177_slot = var_guard177;
        *var_guard178_slot = var_guard178;
        *var_guard179_slot = var_guard179;
        *var_guard180_slot = var_guard180;
        *var_gwell_slot = var_gwell;
        *var_iginv_i_slot = var_iginv_i;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_inv_chib_slot = var_inv_chib;
        *var_jwcorr_slot = var_jwcorr;
        *var_jww_slot = var_jww;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgs_i_slot = var_lgs_i;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_lsd_i_slot = var_lsd_i;
        *var_lss_i_slot = var_lss_i;
        *var_phibedge_slot = var_phibedge;
        *var_phit0edge_slot = var_phit0edge;
        *var_phix1edge_slot = var_phix1edge;
        *var_phix2edge_slot = var_phix2edge;
        *var_phixedge_slot = var_phixedge;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_ig_slot = var_tf_ig;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vinr_max_slot = var_vinr_max;
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_ad_i: f64,
        var_as_i: f64,
        var_guard180: f64,
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
        var_invnf: f64,
        var_jwcorr: f64,
        var_jww: f64,
        var_pd_i: f64,
        var_phitd: f64,
        var_ps_i: f64,
        var_abd_i_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn5_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_expxhf1_d_slot: &mut f64,
        var_expxhf1_s_slot: &mut f64,
        var_expxhf2_d_slot: &mut f64,
        var_expxhf2_d_dn5_slot: &mut f64,
        var_expxhf2_d_dn6_slot: &mut f64,
        var_expxhf2_d_dn7_slot: &mut f64,
        var_expxhf2_d_dn8_slot: &mut f64,
        var_expxhf2_s_slot: &mut f64,
        var_expxhf2_s_dn5_slot: &mut f64,
        var_expxhf2_s_dn6_slot: &mut f64,
        var_expxhf2_s_dn7_slot: &mut f64,
        var_expxhf2_s_dn8_slot: &mut f64,
        var_expxhr_d_slot: &mut f64,
        var_expxhr_d_dn5_slot: &mut f64,
        var_expxhr_d_dn6_slot: &mut f64,
        var_expxhr_d_dn7_slot: &mut f64,
        var_expxhr_d_dn8_slot: &mut f64,
        var_expxhr_s_slot: &mut f64,
        var_expxhr_s_dn5_slot: &mut f64,
        var_expxhr_s_dn6_slot: &mut f64,
        var_expxhr_s_dn7_slot: &mut f64,
        var_expxhr_s_dn8_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_i1_cor_slot: &mut f64,
        var_i1_cor_dn5_slot: &mut f64,
        var_i1_cor_dn6_slot: &mut f64,
        var_i1_cor_dn7_slot: &mut f64,
        var_i1_cor_dn8_slot: &mut f64,
        var_i2_cor_slot: &mut f64,
        var_i2_cor_dn5_slot: &mut f64,
        var_i2_cor_dn6_slot: &mut f64,
        var_i2_cor_dn7_slot: &mut f64,
        var_i2_cor_dn8_slot: &mut f64,
        var_i3_cor_slot: &mut f64,
        var_i3_cor_dn5_slot: &mut f64,
        var_i3_cor_dn6_slot: &mut f64,
        var_i3_cor_dn7_slot: &mut f64,
        var_i3_cor_dn8_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_dn5_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn5_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_isatfor1_d_slot: &mut f64,
        var_isatfor1_s_slot: &mut f64,
        var_isatfor2_d_slot: &mut f64,
        var_isatfor2_d_dn5_slot: &mut f64,
        var_isatfor2_d_dn6_slot: &mut f64,
        var_isatfor2_d_dn7_slot: &mut f64,
        var_isatfor2_d_dn8_slot: &mut f64,
        var_isatfor2_s_slot: &mut f64,
        var_isatfor2_s_dn5_slot: &mut f64,
        var_isatfor2_s_dn6_slot: &mut f64,
        var_isatfor2_s_dn7_slot: &mut f64,
        var_isatfor2_s_dn8_slot: &mut f64,
        var_isatrev_d_slot: &mut f64,
        var_isatrev_d_dn5_slot: &mut f64,
        var_isatrev_d_dn6_slot: &mut f64,
        var_isatrev_d_dn7_slot: &mut f64,
        var_isatrev_d_dn8_slot: &mut f64,
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_dn5_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn5_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0flag_d_slot: &mut f64,
        var_m0flag_s_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn5_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mfor1_d_slot: &mut f64,
        var_mfor1_s_slot: &mut f64,
        var_mfor2_d_slot: &mut f64,
        var_mfor2_d_dn5_slot: &mut f64,
        var_mfor2_d_dn6_slot: &mut f64,
        var_mfor2_d_dn7_slot: &mut f64,
        var_mfor2_d_dn8_slot: &mut f64,
        var_mfor2_s_slot: &mut f64,
        var_mfor2_s_dn5_slot: &mut f64,
        var_mfor2_s_dn6_slot: &mut f64,
        var_mfor2_s_dn7_slot: &mut f64,
        var_mfor2_s_dn8_slot: &mut f64,
        var_mrev_d_slot: &mut f64,
        var_mrev_d_dn5_slot: &mut f64,
        var_mrev_d_dn6_slot: &mut f64,
        var_mrev_d_dn7_slot: &mut f64,
        var_mrev_d_dn8_slot: &mut f64,
        var_mrev_s_slot: &mut f64,
        var_mrev_s_dn5_slot: &mut f64,
        var_mrev_s_dn6_slot: &mut f64,
        var_mrev_s_dn7_slot: &mut f64,
        var_mrev_s_dn8_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn5_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_dn5_slot: &mut f64,
        var_tt2_dn6_slot: &mut f64,
        var_tt2_dn7_slot: &mut f64,
        var_tt2_dn8_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_xhighf1_d_slot: &mut f64,
        var_xhighf1_s_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_dn5_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighf2_s_slot: &mut f64,
        var_xhighf2_s_dn5_slot: &mut f64,
        var_xhighf2_s_dn6_slot: &mut f64,
        var_xhighf2_s_dn7_slot: &mut f64,
        var_xhighf2_s_dn8_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_dn5_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_dn5_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn5: f64 = *var_alphaje_dn5_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_expxhf1_d: f64 = *var_expxhf1_d_slot;
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
        let mut var_expxhf2_d: f64 = *var_expxhf2_d_slot;
        let mut var_expxhf2_d_dn5: f64 = *var_expxhf2_d_dn5_slot;
        let mut var_expxhf2_d_dn6: f64 = *var_expxhf2_d_dn6_slot;
        let mut var_expxhf2_d_dn7: f64 = *var_expxhf2_d_dn7_slot;
        let mut var_expxhf2_d_dn8: f64 = *var_expxhf2_d_dn8_slot;
        let mut var_expxhf2_s: f64 = *var_expxhf2_s_slot;
        let mut var_expxhf2_s_dn5: f64 = *var_expxhf2_s_dn5_slot;
        let mut var_expxhf2_s_dn6: f64 = *var_expxhf2_s_dn6_slot;
        let mut var_expxhf2_s_dn7: f64 = *var_expxhf2_s_dn7_slot;
        let mut var_expxhf2_s_dn8: f64 = *var_expxhf2_s_dn8_slot;
        let mut var_expxhr_d: f64 = *var_expxhr_d_slot;
        let mut var_expxhr_d_dn5: f64 = *var_expxhr_d_dn5_slot;
        let mut var_expxhr_d_dn6: f64 = *var_expxhr_d_dn6_slot;
        let mut var_expxhr_d_dn7: f64 = *var_expxhr_d_dn7_slot;
        let mut var_expxhr_d_dn8: f64 = *var_expxhr_d_dn8_slot;
        let mut var_expxhr_s: f64 = *var_expxhr_s_slot;
        let mut var_expxhr_s_dn5: f64 = *var_expxhr_s_dn5_slot;
        let mut var_expxhr_s_dn6: f64 = *var_expxhr_s_dn6_slot;
        let mut var_expxhr_s_dn7: f64 = *var_expxhr_s_dn7_slot;
        let mut var_expxhr_s_dn8: f64 = *var_expxhr_s_dn8_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_i1_cor: f64 = *var_i1_cor_slot;
        let mut var_i1_cor_dn5: f64 = *var_i1_cor_dn5_slot;
        let mut var_i1_cor_dn6: f64 = *var_i1_cor_dn6_slot;
        let mut var_i1_cor_dn7: f64 = *var_i1_cor_dn7_slot;
        let mut var_i1_cor_dn8: f64 = *var_i1_cor_dn8_slot;
        let mut var_i2_cor: f64 = *var_i2_cor_slot;
        let mut var_i2_cor_dn5: f64 = *var_i2_cor_dn5_slot;
        let mut var_i2_cor_dn6: f64 = *var_i2_cor_dn6_slot;
        let mut var_i2_cor_dn7: f64 = *var_i2_cor_dn7_slot;
        let mut var_i2_cor_dn8: f64 = *var_i2_cor_dn8_slot;
        let mut var_i3_cor: f64 = *var_i3_cor_slot;
        let mut var_i3_cor_dn5: f64 = *var_i3_cor_dn5_slot;
        let mut var_i3_cor_dn6: f64 = *var_i3_cor_dn6_slot;
        let mut var_i3_cor_dn7: f64 = *var_i3_cor_dn7_slot;
        let mut var_i3_cor_dn8: f64 = *var_i3_cor_dn8_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_dn5: f64 = *var_i4_cor_dn5_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn5: f64 = *var_i5_cor_dn5_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_isatfor1_d: f64 = *var_isatfor1_d_slot;
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
        let mut var_isatfor2_d: f64 = *var_isatfor2_d_slot;
        let mut var_isatfor2_d_dn5: f64 = *var_isatfor2_d_dn5_slot;
        let mut var_isatfor2_d_dn6: f64 = *var_isatfor2_d_dn6_slot;
        let mut var_isatfor2_d_dn7: f64 = *var_isatfor2_d_dn7_slot;
        let mut var_isatfor2_d_dn8: f64 = *var_isatfor2_d_dn8_slot;
        let mut var_isatfor2_s: f64 = *var_isatfor2_s_slot;
        let mut var_isatfor2_s_dn5: f64 = *var_isatfor2_s_dn5_slot;
        let mut var_isatfor2_s_dn6: f64 = *var_isatfor2_s_dn6_slot;
        let mut var_isatfor2_s_dn7: f64 = *var_isatfor2_s_dn7_slot;
        let mut var_isatfor2_s_dn8: f64 = *var_isatfor2_s_dn8_slot;
        let mut var_isatrev_d: f64 = *var_isatrev_d_slot;
        let mut var_isatrev_d_dn5: f64 = *var_isatrev_d_dn5_slot;
        let mut var_isatrev_d_dn6: f64 = *var_isatrev_d_dn6_slot;
        let mut var_isatrev_d_dn7: f64 = *var_isatrev_d_dn7_slot;
        let mut var_isatrev_d_dn8: f64 = *var_isatrev_d_dn8_slot;
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_dn5: f64 = *var_isatrev_s_dn5_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn5: f64 = *var_m0_rev_dn5_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0flag_d: f64 = *var_m0flag_d_slot;
        let mut var_m0flag_s: f64 = *var_m0flag_s_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn5: f64 = *var_mcor_rev_dn5_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mfor1_d: f64 = *var_mfor1_d_slot;
        let mut var_mfor1_s: f64 = *var_mfor1_s_slot;
        let mut var_mfor2_d: f64 = *var_mfor2_d_slot;
        let mut var_mfor2_d_dn5: f64 = *var_mfor2_d_dn5_slot;
        let mut var_mfor2_d_dn6: f64 = *var_mfor2_d_dn6_slot;
        let mut var_mfor2_d_dn7: f64 = *var_mfor2_d_dn7_slot;
        let mut var_mfor2_d_dn8: f64 = *var_mfor2_d_dn8_slot;
        let mut var_mfor2_s: f64 = *var_mfor2_s_slot;
        let mut var_mfor2_s_dn5: f64 = *var_mfor2_s_dn5_slot;
        let mut var_mfor2_s_dn6: f64 = *var_mfor2_s_dn6_slot;
        let mut var_mfor2_s_dn7: f64 = *var_mfor2_s_dn7_slot;
        let mut var_mfor2_s_dn8: f64 = *var_mfor2_s_dn8_slot;
        let mut var_mrev_d: f64 = *var_mrev_d_slot;
        let mut var_mrev_d_dn5: f64 = *var_mrev_d_dn5_slot;
        let mut var_mrev_d_dn6: f64 = *var_mrev_d_dn6_slot;
        let mut var_mrev_d_dn7: f64 = *var_mrev_d_dn7_slot;
        let mut var_mrev_d_dn8: f64 = *var_mrev_d_dn8_slot;
        let mut var_mrev_s: f64 = *var_mrev_s_slot;
        let mut var_mrev_s_dn5: f64 = *var_mrev_s_dn5_slot;
        let mut var_mrev_s_dn6: f64 = *var_mrev_s_dn6_slot;
        let mut var_mrev_s_dn7: f64 = *var_mrev_s_dn7_slot;
        let mut var_mrev_s_dn8: f64 = *var_mrev_s_dn8_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn5: f64 = *var_tt1_dn5_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_dn5: f64 = *var_tt2_dn5_slot;
        let mut var_tt2_dn6: f64 = *var_tt2_dn6_slot;
        let mut var_tt2_dn7: f64 = *var_tt2_dn7_slot;
        let mut var_tt2_dn8: f64 = *var_tt2_dn8_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_xhighf1_d: f64 = *var_xhighf1_d_slot;
        let mut var_xhighf1_s: f64 = *var_xhighf1_s_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_dn5: f64 = *var_xhighf2_d_dn5_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighf2_s: f64 = *var_xhighf2_s_slot;
        let mut var_xhighf2_s_dn5: f64 = *var_xhighf2_s_dn5_slot;
        let mut var_xhighf2_s_dn6: f64 = *var_xhighf2_s_dn6_slot;
        let mut var_xhighf2_s_dn7: f64 = *var_xhighf2_s_dn7_slot;
        let mut var_xhighf2_s_dn8: f64 = *var_xhighf2_s_dn8_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_dn5: f64 = *var_xhighr_d_dn5_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_dn5: f64 = *var_xhighr_s_dn5_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        let (assign13380_e11751,) = {
    if (var_guard180 != 0.0) {
        let assign13380_e11749: f64 = (var_as_i * var_invnf);
        (assign13380_e11749,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign13380_e11751;

        let (assign13390_e11761,) = {
    if (var_guard180 != 0.0) {
        let assign13390_e11755: f64 = (var_ps_i * var_invnf);
        let assign13390_e11758: f64 = (var_jwcorr * var_jww);
        let assign13390_e11759: f64 = (assign13390_e11755 - assign13390_e11758);
        (assign13390_e11759,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign13390_e11761;

        let (assign13400_e11765,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign13400_e11765;

        let (assign13410_e11771,) = {
    if (var_guard180 != 0.0) {
        let assign13410_e11769: f64 = (var_ad_i * var_invnf);
        (assign13410_e11769,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign13410_e11771;

        let (assign13420_e11781,) = {
    if (var_guard180 != 0.0) {
        let assign13420_e11775: f64 = (var_pd_i * var_invnf);
        let assign13420_e11778: f64 = (var_jwcorr * var_jww);
        let assign13420_e11779: f64 = (assign13420_e11775 - assign13420_e11778);
        (assign13420_e11779,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign13420_e11781;

        let (assign13430_e11785,) = {
    if (var_guard180 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign13430_e11785;

        let assign13440_e11796: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard181 = assign13440_e11796;

        let (assign13450_e11805,) = {
    if (var_guard181 != 0.0) {
        let (assign13450_e11803,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13450_e11803,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13450_e11805;

        let (assign13460_e11814,) = {
    if (var_guard181 != 0.0) {
        let (assign13460_e11812,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign13460_e11812,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13460_e11814;

        let (assign13470_e11823,) = {
    if (var_guard181 != 0.0) {
        let (assign13470_e11821,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign13470_e11821,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13470_e11823;

        let (assign13480_e11832,) = {
    if (var_guard181 != 0.0) {
        let (assign13480_e11830,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13480_e11830,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13480_e11832;

        let (assign13490_e11841,) = {
    if (var_guard181 != 0.0) {
        let (assign13490_e11839,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13490_e11839,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13490_e11841;

        let (assign13500_e11850,) = {
    if (var_guard181 != 0.0) {
        let (assign13500_e11848,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign13500_e11848,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13500_e11850;

        let (assign13510_e11855,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign13510_e11855;

        let (assign13520_e11860,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign13520_e11860;

        let (assign13530_e11865,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign13530_e11865;

        let (assign13540_e11870,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign13540_e11870;

        let (assign13550_e11875,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign13550_e11875;

        let (assign13560_e11880,) = {
    if (var_guard181 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign13560_e11880;

        var_vbimin_s = 0.0;

        var_vbimin_d = 0.0;

        var_vfmin_s = 0.0;

        var_vfmin_d = 0.0;

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
        var_isatfor2_s_dn5 = 0.0;
        var_isatfor2_s_dn6 = 0.0;
        var_isatfor2_s_dn7 = 0.0;
        var_isatfor2_s_dn8 = 0.0;

        var_isatfor2_d = 0.0;
        var_isatfor2_d_dn5 = 0.0;
        var_isatfor2_d_dn6 = 0.0;
        var_isatfor2_d_dn7 = 0.0;
        var_isatfor2_d_dn8 = 0.0;

        var_mfor2_s = 1.0;
        var_mfor2_s_dn5 = 0.0;
        var_mfor2_s_dn6 = 0.0;
        var_mfor2_s_dn7 = 0.0;
        var_mfor2_s_dn8 = 0.0;

        var_mfor2_d = 1.0;
        var_mfor2_d_dn5 = 0.0;
        var_mfor2_d_dn6 = 0.0;
        var_mfor2_d_dn7 = 0.0;
        var_mfor2_d_dn8 = 0.0;

        var_isatrev_s = 0.0;
        var_isatrev_s_dn5 = 0.0;
        var_isatrev_s_dn6 = 0.0;
        var_isatrev_s_dn7 = 0.0;
        var_isatrev_s_dn8 = 0.0;

        var_isatrev_d = 0.0;
        var_isatrev_d_dn5 = 0.0;
        var_isatrev_d_dn6 = 0.0;
        var_isatrev_d_dn7 = 0.0;
        var_isatrev_d_dn8 = 0.0;

        var_mrev_s = 1.0;
        var_mrev_s_dn5 = 0.0;
        var_mrev_s_dn6 = 0.0;
        var_mrev_s_dn7 = 0.0;
        var_mrev_s_dn8 = 0.0;

        var_mrev_d = 1.0;
        var_mrev_d_dn5 = 0.0;
        var_mrev_d_dn6 = 0.0;
        var_mrev_d_dn7 = 0.0;
        var_mrev_d_dn8 = 0.0;

        var_m0flag_s = 0.0;

        var_m0flag_d = 0.0;

        var_xhighf1_s = 0.0;

        var_xhighf1_d = 0.0;

        var_expxhf1_s = 0.0;

        var_expxhf1_d = 0.0;

        var_xhighf2_s = 0.0;
        var_xhighf2_s_dn5 = 0.0;
        var_xhighf2_s_dn6 = 0.0;
        var_xhighf2_s_dn7 = 0.0;
        var_xhighf2_s_dn8 = 0.0;

        var_xhighf2_d = 0.0;
        var_xhighf2_d_dn5 = 0.0;
        var_xhighf2_d_dn6 = 0.0;
        var_xhighf2_d_dn7 = 0.0;
        var_xhighf2_d_dn8 = 0.0;

        var_expxhf2_s = 0.0;
        var_expxhf2_s_dn5 = 0.0;
        var_expxhf2_s_dn6 = 0.0;
        var_expxhf2_s_dn7 = 0.0;
        var_expxhf2_s_dn8 = 0.0;

        var_expxhf2_d = 0.0;
        var_expxhf2_d_dn5 = 0.0;
        var_expxhf2_d_dn6 = 0.0;
        var_expxhf2_d_dn7 = 0.0;
        var_expxhf2_d_dn8 = 0.0;

        var_xhighr_s = 0.0;
        var_xhighr_s_dn5 = 0.0;
        var_xhighr_s_dn6 = 0.0;
        var_xhighr_s_dn7 = 0.0;
        var_xhighr_s_dn8 = 0.0;

        var_xhighr_d = 0.0;
        var_xhighr_d_dn5 = 0.0;
        var_xhighr_d_dn6 = 0.0;
        var_xhighr_d_dn7 = 0.0;
        var_xhighr_d_dn8 = 0.0;

        var_expxhr_s = 0.0;
        var_expxhr_s_dn5 = 0.0;
        var_expxhr_s_dn6 = 0.0;
        var_expxhr_s_dn7 = 0.0;
        var_expxhr_s_dn8 = 0.0;

        var_expxhr_d = 0.0;
        var_expxhr_d_dn5 = 0.0;
        var_expxhr_d_dn6 = 0.0;
        var_expxhr_d_dn7 = 0.0;
        var_expxhr_d_dn8 = 0.0;

        var_zflagbot_s = 1.0;

        var_zflagbot_d = 1.0;

        var_zflagsti_s = 1.0;

        var_zflagsti_d = 1.0;

        var_zflaggat_s = 1.0;

        var_zflaggat_d = 1.0;

        var_m0_rev = 0.0;
        var_m0_rev_dn5 = 0.0;
        var_m0_rev_dn6 = 0.0;
        var_m0_rev_dn7 = 0.0;
        var_m0_rev_dn8 = 0.0;

        var_mcor_rev = 0.0;
        var_mcor_rev_dn5 = 0.0;
        var_mcor_rev_dn6 = 0.0;
        var_mcor_rev_dn7 = 0.0;
        var_mcor_rev_dn8 = 0.0;

        var_i1_cor = 0.0;
        var_i1_cor_dn5 = 0.0;
        var_i1_cor_dn6 = 0.0;
        var_i1_cor_dn7 = 0.0;
        var_i1_cor_dn8 = 0.0;

        var_i2_cor = 0.0;
        var_i2_cor_dn5 = 0.0;
        var_i2_cor_dn6 = 0.0;
        var_i2_cor_dn7 = 0.0;
        var_i2_cor_dn8 = 0.0;

        var_i3_cor = 0.0;
        var_i3_cor_dn5 = 0.0;
        var_i3_cor_dn6 = 0.0;
        var_i3_cor_dn7 = 0.0;
        var_i3_cor_dn8 = 0.0;

        var_i4_cor = 0.0;
        var_i4_cor_dn5 = 0.0;
        var_i4_cor_dn6 = 0.0;
        var_i4_cor_dn7 = 0.0;
        var_i4_cor_dn8 = 0.0;

        var_i5_cor = 0.0;
        var_i5_cor_dn5 = 0.0;
        var_i5_cor_dn6 = 0.0;
        var_i5_cor_dn7 = 0.0;
        var_i5_cor_dn8 = 0.0;

        var_tt0 = 0.0;

        var_tt1 = 0.0;
        var_tt1_dn5 = 0.0;
        var_tt1_dn6 = 0.0;
        var_tt1_dn7 = 0.0;
        var_tt1_dn8 = 0.0;

        var_tt2 = 0.0;
        var_tt2_dn5 = 0.0;
        var_tt2_dn6 = 0.0;
        var_tt2_dn7 = 0.0;
        var_tt2_dn8 = 0.0;

        var_zfrac = 0.0;

        var_alphaje = 0.0;
        var_alphaje_dn5 = 0.0;
        var_alphaje_dn6 = 0.0;
        var_alphaje_dn7 = 0.0;
        var_alphaje_dn8 = 0.0;

        let assign14130_e11939: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign14130_e11939;

        let assign14140_e11942: f64 = (var_idsatbot * var_absource_i);
        let assign14140_e11944: f64 = if assign14140_e11942 > 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign14140_e11944;

        let (assign14150_e11959,) = {
    if ((var_guard182 != 0.0) && (var_guard183 != 0.0)) {
        let assign14150_e11952: f64 = (var_idsatbot * var_absource_i);
        let assign14150_e11953: f64 = (p.p822 / assign14150_e11952);
        let assign14150_e11955: f64 = (assign14150_e11953 + 1.0);
        let assign14150_e11956: f64 = (assign14150_e11955).ln();
        let assign14150_e11957: f64 = (var_phitd * assign14150_e11956);
        (assign14150_e11957,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14150_e11959;

        let (assign14160_e11966,) = {
    if ((var_guard182 != 0.0) && (var_guard183 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14160_e11966;

        let assign14170_e11969: f64 = (var_idsatsti * var_lssource_i);
        let assign14170_e11971: f64 = if assign14170_e11969 > 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign14170_e11971;

        let (assign14180_e11986,) = {
    if ((var_guard182 != 0.0) && (var_guard184 != 0.0)) {
        let assign14180_e11979: f64 = (var_idsatsti * var_lssource_i);
        let assign14180_e11980: f64 = (p.p822 / assign14180_e11979);
        let assign14180_e11982: f64 = (assign14180_e11980 + 1.0);
        let assign14180_e11983: f64 = (assign14180_e11982).ln();
        let assign14180_e11984: f64 = (var_phitd * assign14180_e11983);
        (assign14180_e11984,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14180_e11986;

        let (assign14190_e11993,) = {
    if ((var_guard182 != 0.0) && (var_guard184 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14190_e11993;

        let assign14200_e11996: f64 = (var_idsatgat * var_lgsource_i);
        let assign14200_e11998: f64 = if assign14200_e11996 > 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign14200_e11998;

        let (assign14210_e12013,) = {
    if ((var_guard182 != 0.0) && (var_guard185 != 0.0)) {
        let assign14210_e12006: f64 = (var_idsatgat * var_lgsource_i);
        let assign14210_e12007: f64 = (p.p822 / assign14210_e12006);
        let assign14210_e12009: f64 = (assign14210_e12007 + 1.0);
        let assign14210_e12010: f64 = (assign14210_e12009).ln();
        let assign14210_e12011: f64 = (var_phitd * assign14210_e12010);
        (assign14210_e12011,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14210_e12013;

        *var_abd_i_slot = var_abd_i;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abs_i_slot = var_abs_i;
        *var_absource_i_slot = var_absource_i;
        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn5_slot = var_alphaje_dn5;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_expxhf1_d_slot = var_expxhf1_d;
        *var_expxhf1_s_slot = var_expxhf1_s;
        *var_expxhf2_d_slot = var_expxhf2_d;
        *var_expxhf2_d_dn5_slot = var_expxhf2_d_dn5;
        *var_expxhf2_d_dn6_slot = var_expxhf2_d_dn6;
        *var_expxhf2_d_dn7_slot = var_expxhf2_d_dn7;
        *var_expxhf2_d_dn8_slot = var_expxhf2_d_dn8;
        *var_expxhf2_s_slot = var_expxhf2_s;
        *var_expxhf2_s_dn5_slot = var_expxhf2_s_dn5;
        *var_expxhf2_s_dn6_slot = var_expxhf2_s_dn6;
        *var_expxhf2_s_dn7_slot = var_expxhf2_s_dn7;
        *var_expxhf2_s_dn8_slot = var_expxhf2_s_dn8;
        *var_expxhr_d_slot = var_expxhr_d;
        *var_expxhr_d_dn5_slot = var_expxhr_d_dn5;
        *var_expxhr_d_dn6_slot = var_expxhr_d_dn6;
        *var_expxhr_d_dn7_slot = var_expxhr_d_dn7;
        *var_expxhr_d_dn8_slot = var_expxhr_d_dn8;
        *var_expxhr_s_slot = var_expxhr_s;
        *var_expxhr_s_dn5_slot = var_expxhr_s_dn5;
        *var_expxhr_s_dn6_slot = var_expxhr_s_dn6;
        *var_expxhr_s_dn7_slot = var_expxhr_s_dn7;
        *var_expxhr_s_dn8_slot = var_expxhr_s_dn8;
        *var_guard181_slot = var_guard181;
        *var_guard182_slot = var_guard182;
        *var_guard183_slot = var_guard183;
        *var_guard184_slot = var_guard184;
        *var_guard185_slot = var_guard185;
        *var_i1_cor_slot = var_i1_cor;
        *var_i1_cor_dn5_slot = var_i1_cor_dn5;
        *var_i1_cor_dn6_slot = var_i1_cor_dn6;
        *var_i1_cor_dn7_slot = var_i1_cor_dn7;
        *var_i1_cor_dn8_slot = var_i1_cor_dn8;
        *var_i2_cor_slot = var_i2_cor;
        *var_i2_cor_dn5_slot = var_i2_cor_dn5;
        *var_i2_cor_dn6_slot = var_i2_cor_dn6;
        *var_i2_cor_dn7_slot = var_i2_cor_dn7;
        *var_i2_cor_dn8_slot = var_i2_cor_dn8;
        *var_i3_cor_slot = var_i3_cor;
        *var_i3_cor_dn5_slot = var_i3_cor_dn5;
        *var_i3_cor_dn6_slot = var_i3_cor_dn6;
        *var_i3_cor_dn7_slot = var_i3_cor_dn7;
        *var_i3_cor_dn8_slot = var_i3_cor_dn8;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_dn5_slot = var_i4_cor_dn5;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn5_slot = var_i5_cor_dn5;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_isatfor1_d_slot = var_isatfor1_d;
        *var_isatfor1_s_slot = var_isatfor1_s;
        *var_isatfor2_d_slot = var_isatfor2_d;
        *var_isatfor2_d_dn5_slot = var_isatfor2_d_dn5;
        *var_isatfor2_d_dn6_slot = var_isatfor2_d_dn6;
        *var_isatfor2_d_dn7_slot = var_isatfor2_d_dn7;
        *var_isatfor2_d_dn8_slot = var_isatfor2_d_dn8;
        *var_isatfor2_s_slot = var_isatfor2_s;
        *var_isatfor2_s_dn5_slot = var_isatfor2_s_dn5;
        *var_isatfor2_s_dn6_slot = var_isatfor2_s_dn6;
        *var_isatfor2_s_dn7_slot = var_isatfor2_s_dn7;
        *var_isatfor2_s_dn8_slot = var_isatfor2_s_dn8;
        *var_isatrev_d_slot = var_isatrev_d;
        *var_isatrev_d_dn5_slot = var_isatrev_d_dn5;
        *var_isatrev_d_dn6_slot = var_isatrev_d_dn6;
        *var_isatrev_d_dn7_slot = var_isatrev_d_dn7;
        *var_isatrev_d_dn8_slot = var_isatrev_d_dn8;
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_dn5_slot = var_isatrev_s_dn5;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lss_i_slot = var_lss_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn5_slot = var_m0_rev_dn5;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0flag_d_slot = var_m0flag_d;
        *var_m0flag_s_slot = var_m0flag_s;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn5_slot = var_mcor_rev_dn5;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mfor1_d_slot = var_mfor1_d;
        *var_mfor1_s_slot = var_mfor1_s;
        *var_mfor2_d_slot = var_mfor2_d;
        *var_mfor2_d_dn5_slot = var_mfor2_d_dn5;
        *var_mfor2_d_dn6_slot = var_mfor2_d_dn6;
        *var_mfor2_d_dn7_slot = var_mfor2_d_dn7;
        *var_mfor2_d_dn8_slot = var_mfor2_d_dn8;
        *var_mfor2_s_slot = var_mfor2_s;
        *var_mfor2_s_dn5_slot = var_mfor2_s_dn5;
        *var_mfor2_s_dn6_slot = var_mfor2_s_dn6;
        *var_mfor2_s_dn7_slot = var_mfor2_s_dn7;
        *var_mfor2_s_dn8_slot = var_mfor2_s_dn8;
        *var_mrev_d_slot = var_mrev_d;
        *var_mrev_d_dn5_slot = var_mrev_d_dn5;
        *var_mrev_d_dn6_slot = var_mrev_d_dn6;
        *var_mrev_d_dn7_slot = var_mrev_d_dn7;
        *var_mrev_d_dn8_slot = var_mrev_d_dn8;
        *var_mrev_s_slot = var_mrev_s;
        *var_mrev_s_dn5_slot = var_mrev_s_dn5;
        *var_mrev_s_dn6_slot = var_mrev_s_dn6;
        *var_mrev_s_dn7_slot = var_mrev_s_dn7;
        *var_mrev_s_dn8_slot = var_mrev_s_dn8;
        *var_tt0_slot = var_tt0;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn5_slot = var_tt1_dn5;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt2_slot = var_tt2;
        *var_tt2_dn5_slot = var_tt2_dn5;
        *var_tt2_dn6_slot = var_tt2_dn6;
        *var_tt2_dn7_slot = var_tt2_dn7;
        *var_tt2_dn8_slot = var_tt2_dn8;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vch_d_slot = var_vch_d;
        *var_vch_s_slot = var_vch_s;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_xhighf1_d_slot = var_xhighf1_d;
        *var_xhighf1_s_slot = var_xhighf1_s;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_dn5_slot = var_xhighf2_d_dn5;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighf2_s_slot = var_xhighf2_s;
        *var_xhighf2_s_dn5_slot = var_xhighf2_s_dn5;
        *var_xhighf2_s_dn6_slot = var_xhighf2_s_dn6;
        *var_xhighf2_s_dn7_slot = var_xhighf2_s_dn7;
        *var_xhighf2_s_dn8_slot = var_xhighf2_s_dn8;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_dn5_slot = var_xhighr_d_dn5;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_dn5_slot = var_xhighr_s_dn5;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_guard182: f64,
        var_guard185: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vbibot: f64,
        var_vbibot_d: f64,
        var_vbigat: f64,
        var_vbisti: f64,
        var_vbisti_d: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard194_slot: &mut f64,
        var_guard195_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard194: f64 = *var_guard194_slot;
        let mut var_guard195: f64 = *var_guard195_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;

        let (assign14220_e12020,) = {
    if ((var_guard182 != 0.0) && (var_guard185 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14220_e12020;

        let (assign14230_e12028,) = {
    if (var_guard182 != 0.0) {
        let assign14230_e12024: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14230_e12026: f64 = (assign14230_e12024).min(var_vmaxgat);
        (assign14230_e12026,)
    } else {
        (var_vmax_s,)
    }
};
        var_vmax_s = assign14230_e12028;

        let assign14240_e12031: f64 = (var_vmax_s * var_phitdinv);
        let assign14240_e12032: f64 = (assign14240_e12031).abs();
        let assign14240_e12034: f64 = if assign14240_e12032 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard186 = assign14240_e12034;

        let (assign14250_e12043,) = {
    if ((var_guard182 != 0.0) && (var_guard186 != 0.0)) {
        let assign14250_e12040: f64 = (var_vmax_s * var_phitdinv);
        let assign14250_e12041: f64 = (assign14250_e12040).exp();
        (assign14250_e12041,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14250_e12043;

        let assign14260_e12046: f64 = (var_vmax_s * var_phitdinv);
        let assign14260_e12048: f64 = if assign14260_e12046 < 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign14260_e12048;

        let (assign14270_e12088,) = {
    if (((var_guard182 != 0.0) && (var_guard186 == 0.0)) && (var_guard187 != 0.0)) {
        let assign14270_e12058: f64 = (-230.25850929940458);
        let assign14270_e12061: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12062: f64 = (assign14270_e12058 - assign14270_e12061);
        let assign14270_e12066: f64 = (-230.25850929940458);
        let assign14270_e12069: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12070: f64 = (assign14270_e12066 - assign14270_e12069);
        let assign14270_e12073: f64 = (-230.25850929940458);
        let assign14270_e12076: f64 = (var_vmax_s * var_phitdinv);
        let assign14270_e12077: f64 = (assign14270_e12073 - assign14270_e12076);
        let assign14270_e12079: f64 = (assign14270_e12077 * 0.3333333333333333);
        let assign14270_e12080: f64 = (1.0 + assign14270_e12079);
        let assign14270_e12081: f64 = (assign14270_e12070 * assign14270_e12080);
        let assign14270_e12082: f64 = (0.5 * assign14270_e12081);
        let assign14270_e12083: f64 = (1.0 + assign14270_e12082);
        let assign14270_e12084: f64 = (assign14270_e12062 * assign14270_e12083);
        let assign14270_e12085: f64 = (1.0 + assign14270_e12084);
        let assign14270_e12086: f64 = (1e-100 / assign14270_e12085);
        (assign14270_e12086,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14270_e12088;

        let (assign14280_e12126,) = {
    if (((var_guard182 != 0.0) && (var_guard186 == 0.0)) && (var_guard187 == 0.0)) {
        let assign14280_e12100: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12102: f64 = (assign14280_e12100 - 230.25850929940458);
        let assign14280_e12107: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12109: f64 = (assign14280_e12107 - 230.25850929940458);
        let assign14280_e12113: f64 = (var_vmax_s * var_phitdinv);
        let assign14280_e12115: f64 = (assign14280_e12113 - 230.25850929940458);
        let assign14280_e12117: f64 = (assign14280_e12115 * 0.3333333333333333);
        let assign14280_e12118: f64 = (1.0 + assign14280_e12117);
        let assign14280_e12119: f64 = (assign14280_e12109 * assign14280_e12118);
        let assign14280_e12120: f64 = (0.5 * assign14280_e12119);
        let assign14280_e12121: f64 = (1.0 + assign14280_e12120);
        let assign14280_e12122: f64 = (assign14280_e12102 * assign14280_e12121);
        let assign14280_e12123: f64 = (1.0 + assign14280_e12122);
        let assign14280_e12124: f64 = (1e100 * assign14280_e12123);
        (assign14280_e12124,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign14280_e12126;

        let (assign14290_e12130,) = {
    if (var_guard182 != 0.0) {
        (var_vbibot,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14290_e12130;

        let (assign14300_e12134,) = {
    if (var_guard182 != 0.0) {
        (var_vbisti,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14300_e12134;

        let (assign14310_e12138,) = {
    if (var_guard182 != 0.0) {
        (var_vbigat,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14310_e12138;

        let (assign14320_e12142,) = {
    if (var_guard182 != 0.0) {
        (p.p831,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14320_e12142;

        let (assign14330_e12146,) = {
    if (var_guard182 != 0.0) {
        (p.p832,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14330_e12146;

        let (assign14340_e12150,) = {
    if (var_guard182 != 0.0) {
        (p.p833,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14340_e12150;

        let (assign14350_e12154,) = {
    if (var_guard182 != 0.0) {
        (p.p828,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14350_e12154;

        let (assign14360_e12158,) = {
    if (var_guard182 != 0.0) {
        (p.p829,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14360_e12158;

        let (assign14370_e12162,) = {
    if (var_guard182 != 0.0) {
        (p.p830,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14370_e12162;

        let assign14380_e12165: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard188 = assign14380_e12165;

        let (assign14390_e12173,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14390_e12171: f64 = (var_vbisti + var_vbigat);
        (assign14390_e12171,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14390_e12173;

        let (assign14400_e12183,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14400_e12180: f64 = (p.p832).min(p.p833);
        let assign14400_e12181: f64 = (0.9 * assign14400_e12180);
        (assign14400_e12181,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14400_e12183;

        let (assign14410_e12191,) = {
    if ((var_guard182 != 0.0) && (var_guard188 != 0.0)) {
        let assign14410_e12189: f64 = (p.p829 + p.p830);
        (assign14410_e12189,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14410_e12191;

        let assign14420_e12194: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign14420_e12194;

        let (assign14430_e12202,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14430_e12200: f64 = (var_vbibot + var_vbigat);
        (assign14430_e12200,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14430_e12202;

        let (assign14440_e12212,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14440_e12209: f64 = (p.p831).min(p.p833);
        let assign14440_e12210: f64 = (0.9 * assign14440_e12209);
        (assign14440_e12210,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14440_e12212;

        let (assign14450_e12220,) = {
    if ((var_guard182 != 0.0) && (var_guard189 != 0.0)) {
        let assign14450_e12218: f64 = (p.p828 + p.p830);
        (assign14450_e12218,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14450_e12220;

        let assign14460_e12223: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign14460_e12223;

        let (assign14470_e12231,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14470_e12229: f64 = (var_vbibot + var_vbisti);
        (assign14470_e12229,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14470_e12231;

        let (assign14480_e12241,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14480_e12238: f64 = (p.p831).min(p.p832);
        let assign14480_e12239: f64 = (0.9 * assign14480_e12238);
        (assign14480_e12239,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14480_e12241;

        let (assign14490_e12249,) = {
    if ((var_guard182 != 0.0) && (var_guard190 != 0.0)) {
        let assign14490_e12247: f64 = (p.p828 + p.p829);
        (assign14490_e12247,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14490_e12249;

        let (assign14500_e12257,) = {
    if (var_guard182 != 0.0) {
        let assign14500_e12253: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14500_e12255: f64 = (assign14500_e12253).min(var_vbigat2);
        (assign14500_e12255,)
    } else {
        (var_vbimin_s,)
    }
};
        var_vbimin_s = assign14500_e12257;

        let (assign14510_e12263,) = {
    if (var_guard182 != 0.0) {
        let assign14510_e12261: f64 = (var_vbimin_s * 0.1);
        (assign14510_e12261,)
    } else {
        (var_vch_s,)
    }
};
        var_vch_s = assign14510_e12263;

        let (assign14520_e12271,) = {
    if (var_guard182 != 0.0) {
        let assign14520_e12267: f64 = (var_pbot2).max(var_psti2);
        let assign14520_e12269: f64 = (assign14520_e12267).max(var_pgat2);
        (assign14520_e12269,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14520_e12271;

        let (assign14530_e12284,) = {
    if (var_guard182 != 0.0) {
        let assign14530_e12277: f64 = (-1.0);
        let assign14530_e12279: f64 = (assign14530_e12277 / var_pmax);
        let assign14530_e12280: f64 = (2.0_f64).powf(assign14530_e12279);
        let assign14530_e12281: f64 = (1.0 - assign14530_e12280);
        let assign14530_e12282: f64 = (var_vbimin_s * assign14530_e12281);
        (assign14530_e12282,)
    } else {
        (var_vfmin_s,)
    }
};
        var_vfmin_s = assign14530_e12284;

        let (assign14540_e12294,) = {
    if (var_guard182 != 0.0) {
        let assign14540_e12288: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14540_e12290: f64 = (assign14540_e12288).min(var_vbigat2r);
        let assign14540_e12292: f64 = (assign14540_e12290 - 0.05);
        (assign14540_e12292,)
    } else {
        (var_vbbtlim_s,)
    }
};
        var_vbbtlim_s = assign14540_e12294;

        let assign14550_e12297: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14550_e12299: f64 = if assign14550_e12297 > 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign14550_e12299;

        let (assign14560_e12314,) = {
    if ((var_guard182 != 0.0) && (var_guard191 != 0.0)) {
        let assign14560_e12307: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign14560_e12308: f64 = (p.p822 / assign14560_e12307);
        let assign14560_e12310: f64 = (assign14560_e12308 + 1.0);
        let assign14560_e12311: f64 = (assign14560_e12310).ln();
        let assign14560_e12312: f64 = (var_phitd * assign14560_e12311);
        (assign14560_e12312,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14560_e12314;

        let (assign14570_e12321,) = {
    if ((var_guard182 != 0.0) && (var_guard191 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign14570_e12321;

        let assign14580_e12324: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14580_e12326: f64 = if assign14580_e12324 > 0.0 { 1.0 } else { 0.0 };
        var_guard192 = assign14580_e12326;

        let (assign14590_e12341,) = {
    if ((var_guard182 != 0.0) && (var_guard192 != 0.0)) {
        let assign14590_e12334: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign14590_e12335: f64 = (p.p822 / assign14590_e12334);
        let assign14590_e12337: f64 = (assign14590_e12335 + 1.0);
        let assign14590_e12338: f64 = (assign14590_e12337).ln();
        let assign14590_e12339: f64 = (var_phitd * assign14590_e12338);
        (assign14590_e12339,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14590_e12341;

        let (assign14600_e12348,) = {
    if ((var_guard182 != 0.0) && (var_guard192 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign14600_e12348;

        let assign14610_e12351: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14610_e12353: f64 = if assign14610_e12351 > 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign14610_e12353;

        let (assign14620_e12368,) = {
    if ((var_guard182 != 0.0) && (var_guard193 != 0.0)) {
        let assign14620_e12361: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign14620_e12362: f64 = (p.p822 / assign14620_e12361);
        let assign14620_e12364: f64 = (assign14620_e12362 + 1.0);
        let assign14620_e12365: f64 = (assign14620_e12364).ln();
        let assign14620_e12366: f64 = (var_phitd * assign14620_e12365);
        (assign14620_e12366,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14620_e12368;

        let (assign14630_e12375,) = {
    if ((var_guard182 != 0.0) && (var_guard193 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign14630_e12375;

        let (assign14640_e12383,) = {
    if (var_guard182 != 0.0) {
        let assign14640_e12379: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign14640_e12381: f64 = (assign14640_e12379).min(var_vmaxgat);
        (assign14640_e12381,)
    } else {
        (var_vmax_d,)
    }
};
        var_vmax_d = assign14640_e12383;

        let assign14650_e12386: f64 = (var_vmax_d * var_phitdinv);
        let assign14650_e12387: f64 = (assign14650_e12386).abs();
        let assign14650_e12389: f64 = if assign14650_e12387 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard194 = assign14650_e12389;

        let (assign14660_e12398,) = {
    if ((var_guard182 != 0.0) && (var_guard194 != 0.0)) {
        let assign14660_e12395: f64 = (var_vmax_d * var_phitdinv);
        let assign14660_e12396: f64 = (assign14660_e12395).exp();
        (assign14660_e12396,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14660_e12398;

        let assign14670_e12401: f64 = (var_vmax_d * var_phitdinv);
        let assign14670_e12403: f64 = if assign14670_e12401 < 0.0 { 1.0 } else { 0.0 };
        var_guard195 = assign14670_e12403;

        let (assign14680_e12443,) = {
    if (((var_guard182 != 0.0) && (var_guard194 == 0.0)) && (var_guard195 != 0.0)) {
        let assign14680_e12413: f64 = (-230.25850929940458);
        let assign14680_e12416: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12417: f64 = (assign14680_e12413 - assign14680_e12416);
        let assign14680_e12421: f64 = (-230.25850929940458);
        let assign14680_e12424: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12425: f64 = (assign14680_e12421 - assign14680_e12424);
        let assign14680_e12428: f64 = (-230.25850929940458);
        let assign14680_e12431: f64 = (var_vmax_d * var_phitdinv);
        let assign14680_e12432: f64 = (assign14680_e12428 - assign14680_e12431);
        let assign14680_e12434: f64 = (assign14680_e12432 * 0.3333333333333333);
        let assign14680_e12435: f64 = (1.0 + assign14680_e12434);
        let assign14680_e12436: f64 = (assign14680_e12425 * assign14680_e12435);
        let assign14680_e12437: f64 = (0.5 * assign14680_e12436);
        let assign14680_e12438: f64 = (1.0 + assign14680_e12437);
        let assign14680_e12439: f64 = (assign14680_e12417 * assign14680_e12438);
        let assign14680_e12440: f64 = (1.0 + assign14680_e12439);
        let assign14680_e12441: f64 = (1e-100 / assign14680_e12440);
        (assign14680_e12441,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14680_e12443;

        let (assign14690_e12481,) = {
    if (((var_guard182 != 0.0) && (var_guard194 == 0.0)) && (var_guard195 == 0.0)) {
        let assign14690_e12455: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12457: f64 = (assign14690_e12455 - 230.25850929940458);
        let assign14690_e12462: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12464: f64 = (assign14690_e12462 - 230.25850929940458);
        let assign14690_e12468: f64 = (var_vmax_d * var_phitdinv);
        let assign14690_e12470: f64 = (assign14690_e12468 - 230.25850929940458);
        let assign14690_e12472: f64 = (assign14690_e12470 * 0.3333333333333333);
        let assign14690_e12473: f64 = (1.0 + assign14690_e12472);
        let assign14690_e12474: f64 = (assign14690_e12464 * assign14690_e12473);
        let assign14690_e12475: f64 = (0.5 * assign14690_e12474);
        let assign14690_e12476: f64 = (1.0 + assign14690_e12475);
        let assign14690_e12477: f64 = (assign14690_e12457 * assign14690_e12476);
        let assign14690_e12478: f64 = (1.0 + assign14690_e12477);
        let assign14690_e12479: f64 = (1e100 * assign14690_e12478);
        (assign14690_e12479,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign14690_e12481;

        let (assign14700_e12485,) = {
    if (var_guard182 != 0.0) {
        (var_vbibot_d,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14700_e12485;

        let (assign14710_e12489,) = {
    if (var_guard182 != 0.0) {
        (var_vbisti_d,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14710_e12489;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_guard186_slot = var_guard186;
        *var_guard187_slot = var_guard187;
        *var_guard188_slot = var_guard188;
        *var_guard189_slot = var_guard189;
        *var_guard190_slot = var_guard190;
        *var_guard191_slot = var_guard191;
        *var_guard192_slot = var_guard192;
        *var_guard193_slot = var_guard193;
        *var_guard194_slot = var_guard194;
        *var_guard195_slot = var_guard195;
        *var_pbot2_slot = var_pbot2;
        *var_pgat2_slot = var_pgat2;
        *var_pmax_slot = var_pmax;
        *var_psti2_slot = var_psti2;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vch_s_slot = var_vch_s;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxsti_slot = var_vmaxsti;
    }

    pub(super) fn stamp_transient_block_21(
        var_abdrain_i: f64,
        var_guard182: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_swjunexp_i: f64,
        var_vbibot_d: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti_d: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_guard196_slot: &mut f64,
        var_guard197_slot: &mut f64,
        var_guard198_slot: &mut f64,
        var_guard199_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_guard196: f64 = *var_guard196_slot;
        let mut var_guard197: f64 = *var_guard197_slot;
        let mut var_guard198: f64 = *var_guard198_slot;
        let mut var_guard199: f64 = *var_guard199_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign14720_e12493,) = {
    if (var_guard182 != 0.0) {
        (var_vbigat_d,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14720_e12493;

        let (assign14730_e12497,) = {
    if (var_guard182 != 0.0) {
        (var_pbotd_i,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14730_e12497;

        let (assign14740_e12501,) = {
    if (var_guard182 != 0.0) {
        (var_pstid_i,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14740_e12501;

        let (assign14750_e12505,) = {
    if (var_guard182 != 0.0) {
        (var_pgatd_i,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14750_e12505;

        let (assign14760_e12509,) = {
    if (var_guard182 != 0.0) {
        (var_vbirbotd_i,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14760_e12509;

        let (assign14770_e12513,) = {
    if (var_guard182 != 0.0) {
        (var_vbirstid_i,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14770_e12513;

        let (assign14780_e12517,) = {
    if (var_guard182 != 0.0) {
        (var_vbirgatd_i,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14780_e12517;

        let assign14790_e12520: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard196 = assign14790_e12520;

        let (assign14800_e12528,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14800_e12526: f64 = (var_vbisti_d + var_vbigat_d);
        (assign14800_e12526,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign14800_e12528;

        let (assign14810_e12538,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14810_e12535: f64 = (var_pstid_i).min(var_pgatd_i);
        let assign14810_e12536: f64 = (0.9 * assign14810_e12535);
        (assign14810_e12536,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign14810_e12538;

        let (assign14820_e12546,) = {
    if ((var_guard182 != 0.0) && (var_guard196 != 0.0)) {
        let assign14820_e12544: f64 = (var_vbirstid_i + var_vbirgatd_i);
        (assign14820_e12544,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign14820_e12546;

        let assign14830_e12549: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard197 = assign14830_e12549;

        let (assign14840_e12557,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14840_e12555: f64 = (var_vbibot_d + var_vbigat_d);
        (assign14840_e12555,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign14840_e12557;

        let (assign14850_e12567,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14850_e12564: f64 = (var_pbotd_i).min(var_pgatd_i);
        let assign14850_e12565: f64 = (0.9 * assign14850_e12564);
        (assign14850_e12565,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign14850_e12567;

        let (assign14860_e12575,) = {
    if ((var_guard182 != 0.0) && (var_guard197 != 0.0)) {
        let assign14860_e12573: f64 = (var_vbirbotd_i + var_vbirgatd_i);
        (assign14860_e12573,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign14860_e12575;

        let assign14870_e12578: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard198 = assign14870_e12578;

        let (assign14880_e12586,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14880_e12584: f64 = (var_vbibot_d + var_vbisti_d);
        (assign14880_e12584,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign14880_e12586;

        let (assign14890_e12596,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14890_e12593: f64 = (var_pbotd_i).min(var_pstid_i);
        let assign14890_e12594: f64 = (0.9 * assign14890_e12593);
        (assign14890_e12594,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign14890_e12596;

        let (assign14900_e12604,) = {
    if ((var_guard182 != 0.0) && (var_guard198 != 0.0)) {
        let assign14900_e12602: f64 = (var_vbirbotd_i + var_vbirstid_i);
        (assign14900_e12602,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign14900_e12604;

        let (assign14910_e12612,) = {
    if (var_guard182 != 0.0) {
        let assign14910_e12608: f64 = (var_vbibot2).min(var_vbisti2);
        let assign14910_e12610: f64 = (assign14910_e12608).min(var_vbigat2);
        (assign14910_e12610,)
    } else {
        (var_vbimin_d,)
    }
};
        var_vbimin_d = assign14910_e12612;

        let (assign14920_e12618,) = {
    if (var_guard182 != 0.0) {
        let assign14920_e12616: f64 = (var_vbimin_d * 0.1);
        (assign14920_e12616,)
    } else {
        (var_vch_d,)
    }
};
        var_vch_d = assign14920_e12618;

        let (assign14930_e12626,) = {
    if (var_guard182 != 0.0) {
        let assign14930_e12622: f64 = (var_pbot2).max(var_psti2);
        let assign14930_e12624: f64 = (assign14930_e12622).max(var_pgat2);
        (assign14930_e12624,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign14930_e12626;

        let (assign14940_e12639,) = {
    if (var_guard182 != 0.0) {
        let assign14940_e12632: f64 = (-1.0);
        let assign14940_e12634: f64 = (assign14940_e12632 / var_pmax);
        let assign14940_e12635: f64 = (2.0_f64).powf(assign14940_e12634);
        let assign14940_e12636: f64 = (1.0 - assign14940_e12635);
        let assign14940_e12637: f64 = (var_vbimin_d * assign14940_e12636);
        (assign14940_e12637,)
    } else {
        (var_vfmin_d,)
    }
};
        var_vfmin_d = assign14940_e12639;

        let (assign14950_e12649,) = {
    if (var_guard182 != 0.0) {
        let assign14950_e12643: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign14950_e12645: f64 = (assign14950_e12643).min(var_vbigat2r);
        let assign14950_e12647: f64 = (assign14950_e12645 - 0.05);
        (assign14950_e12647,)
    } else {
        (var_vbbtlim_d,)
    }
};
        var_vbbtlim_d = assign14950_e12649;

        let assign14960_e12652: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard199 = assign14960_e12652;

        let (assign14970_e12658, assign14970_e12658_d_n5, assign14970_e12658_d_n6, assign14970_e12658_d_n7, assign14970_e12658_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign14970_e12658;
        var_ysq_dn5 = assign14970_e12658_d_n5;
        var_ysq_dn6 = assign14970_e12658_d_n6;
        var_ysq_dn7 = assign14970_e12658_d_n7;
        var_ysq_dn8 = assign14970_e12658_d_n8;

        let (assign14980_e12664, assign14980_e12664_d_n5, assign14980_e12664_d_n6, assign14980_e12664_d_n7, assign14980_e12664_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign14980_e12664;
        var_terfc_dn5 = assign14980_e12664_d_n5;
        var_terfc_dn6 = assign14980_e12664_d_n6;
        var_terfc_dn7 = assign14980_e12664_d_n7;
        var_terfc_dn8 = assign14980_e12664_d_n8;

        let (assign14990_e12670, assign14990_e12670_d_n5, assign14990_e12670_d_n6, assign14990_e12670_d_n7, assign14990_e12670_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign14990_e12670;
        var_erfcpos_dn5 = assign14990_e12670_d_n5;
        var_erfcpos_dn6 = assign14990_e12670_d_n6;
        var_erfcpos_dn7 = assign14990_e12670_d_n7;
        var_erfcpos_dn8 = assign14990_e12670_d_n8;

        let (assign15060_e12712,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign15060_e12712;

        let (assign15080_e12724,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_z,)
    }
};
        var_z = assign15080_e12724;

        let (assign15090_e12730,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign15090_e12730;

        let (assign15100_e12736,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign15100_e12736;

        let (assign15110_e12742,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign15110_e12742;

        let (assign15120_e12748,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign15120_e12748;

        let (assign15130_e12754,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign15130_e12754;

        let (assign15140_e12760,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign15140_e12760;

        let (assign15150_e12766, assign15150_e12766_d_n5, assign15150_e12766_d_n6, assign15150_e12766_d_n7, assign15150_e12766_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign15150_e12766;
        var_tmp_dn5 = assign15150_e12766_d_n5;
        var_tmp_dn6 = assign15150_e12766_d_n6;
        var_tmp_dn7 = assign15150_e12766_d_n7;
        var_tmp_dn8 = assign15150_e12766_d_n8;

        let (assign15160_e12772,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign15160_e12772;

        let (assign15170_e12778, assign15170_e12778_d_n5, assign15170_e12778_d_n6, assign15170_e12778_d_n7, assign15170_e12778_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign15170_e12778;
        var_isrh_dn5 = assign15170_e12778_d_n5;
        var_isrh_dn6 = assign15170_e12778_d_n6;
        var_isrh_dn7 = assign15170_e12778_d_n7;
        var_isrh_dn8 = assign15170_e12778_d_n8;

        let (assign15180_e12784,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign15180_e12784;

        let (assign15190_e12790,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign15190_e12790;

        let (assign15200_e12796,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign15200_e12796;

        let (assign15210_e12802,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign15210_e12802;

        let (assign15220_e12808, assign15220_e12808_d_n5, assign15220_e12808_d_n6, assign15220_e12808_d_n7, assign15220_e12808_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign15220_e12808;
        var_wdep_dn5 = assign15220_e12808_d_n5;
        var_wdep_dn6 = assign15220_e12808_d_n6;
        var_wdep_dn7 = assign15220_e12808_d_n7;
        var_wdep_dn8 = assign15220_e12808_d_n8;

        let (assign15230_e12814, assign15230_e12814_d_n5, assign15230_e12814_d_n6, assign15230_e12814_d_n7, assign15230_e12814_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign15230_e12814;
        var_asrh_dn5 = assign15230_e12814_d_n5;
        var_asrh_dn6 = assign15230_e12814_d_n6;
        var_asrh_dn7 = assign15230_e12814_d_n7;
        var_asrh_dn8 = assign15230_e12814_d_n8;

        let (assign15240_e12820, assign15240_e12820_d_n5, assign15240_e12820_d_n6, assign15240_e12820_d_n7, assign15240_e12820_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign15240_e12820;
        var_itat_dn5 = assign15240_e12820_d_n5;
        var_itat_dn6 = assign15240_e12820_d_n6;
        var_itat_dn7 = assign15240_e12820_d_n7;
        var_itat_dn8 = assign15240_e12820_d_n8;

        let (assign15250_e12826, assign15250_e12826_d_n5, assign15250_e12826_d_n6, assign15250_e12826_d_n7, assign15250_e12826_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign15250_e12826;
        var_btat_dn5 = assign15250_e12826_d_n5;
        var_btat_dn6 = assign15250_e12826_d_n6;
        var_btat_dn7 = assign15250_e12826_d_n7;
        var_btat_dn8 = assign15250_e12826_d_n8;

        let (assign15260_e12832, assign15260_e12832_d_n5, assign15260_e12832_d_n6, assign15260_e12832_d_n7, assign15260_e12832_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign15260_e12832;
        var_twoatatoverthreebtat_dn5 = assign15260_e12832_d_n5;
        var_twoatatoverthreebtat_dn6 = assign15260_e12832_d_n6;
        var_twoatatoverthreebtat_dn7 = assign15260_e12832_d_n7;
        var_twoatatoverthreebtat_dn8 = assign15260_e12832_d_n8;

        let (assign15270_e12838, assign15270_e12838_d_n5, assign15270_e12838_d_n6, assign15270_e12838_d_n7, assign15270_e12838_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign15270_e12838;
        var_umaxbeforelimiting_dn5 = assign15270_e12838_d_n5;
        var_umaxbeforelimiting_dn6 = assign15270_e12838_d_n6;
        var_umaxbeforelimiting_dn7 = assign15270_e12838_d_n7;
        var_umaxbeforelimiting_dn8 = assign15270_e12838_d_n8;

        let (assign15280_e12844, assign15280_e12844_d_n5, assign15280_e12844_d_n6, assign15280_e12844_d_n7, assign15280_e12844_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign15280_e12844;
        var_umax_dn5 = assign15280_e12844_d_n5;
        var_umax_dn6 = assign15280_e12844_d_n6;
        var_umax_dn7 = assign15280_e12844_d_n7;
        var_umax_dn8 = assign15280_e12844_d_n8;

        let (assign15290_e12850, assign15290_e12850_d_n5, assign15290_e12850_d_n6, assign15290_e12850_d_n7, assign15290_e12850_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign15290_e12850;
        var_sqrtumax_dn5 = assign15290_e12850_d_n5;
        var_sqrtumax_dn6 = assign15290_e12850_d_n6;
        var_sqrtumax_dn7 = assign15290_e12850_d_n7;
        var_sqrtumax_dn8 = assign15290_e12850_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_guard196_slot = var_guard196;
        *var_guard197_slot = var_guard197;
        *var_guard198_slot = var_guard198;
        *var_guard199_slot = var_guard199;
        *var_id__blk219_slot = var_id__blk219;
        *var_idmult_slot = var_idmult;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_pbot2_slot = var_pbot2;
        *var_pgat2_slot = var_pgat2;
        *var_pmax_slot = var_pmax;
        *var_psti2_slot = var_psti2;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vch_d_slot = var_vch_d;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_absource_i: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_vbbtlim_s: f64,
        var_vbimin_s: f64,
        var_vmax_s: f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fraci_slot: &mut f64,
        var_fracna_slot: &mut f64,
        var_fracnb_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard250_slot: &mut f64,
        var_guard251_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_v1_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fraci: f64 = *var_fraci_slot;
        let mut var_fracna: f64 = *var_fracna_slot;
        let mut var_fracnb: f64 = *var_fracnb_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_guard251: f64 = *var_guard251_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign15300_e12856, assign15300_e12856_d_n5, assign15300_e12856_d_n6, assign15300_e12856_d_n7, assign15300_e12856_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign15300_e12856;
        var_umaxpoweronepointfive_dn5 = assign15300_e12856_d_n5;
        var_umaxpoweronepointfive_dn6 = assign15300_e12856_d_n6;
        var_umaxpoweronepointfive_dn7 = assign15300_e12856_d_n7;
        var_umaxpoweronepointfive_dn8 = assign15300_e12856_d_n8;

        let (assign15310_e12862, assign15310_e12862_d_n5, assign15310_e12862_d_n6, assign15310_e12862_d_n7, assign15310_e12862_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign15310_e12862;
        var_wgamma_dn5 = assign15310_e12862_d_n5;
        var_wgamma_dn6 = assign15310_e12862_d_n6;
        var_wgamma_dn7 = assign15310_e12862_d_n7;
        var_wgamma_dn8 = assign15310_e12862_d_n8;

        let (assign15320_e12868, assign15320_e12868_d_n5, assign15320_e12868_d_n6, assign15320_e12868_d_n7, assign15320_e12868_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign15320_e12868;
        var_wtat_dn5 = assign15320_e12868_d_n5;
        var_wtat_dn6 = assign15320_e12868_d_n6;
        var_wtat_dn7 = assign15320_e12868_d_n7;
        var_wtat_dn8 = assign15320_e12868_d_n8;

        let (assign15330_e12874, assign15330_e12874_d_n5, assign15330_e12874_d_n6, assign15330_e12874_d_n7, assign15330_e12874_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign15330_e12874;
        var_ktat_dn5 = assign15330_e12874_d_n5;
        var_ktat_dn6 = assign15330_e12874_d_n6;
        var_ktat_dn7 = assign15330_e12874_d_n7;
        var_ktat_dn8 = assign15330_e12874_d_n8;

        let (assign15340_e12880, assign15340_e12880_d_n5, assign15340_e12880_d_n6, assign15340_e12880_d_n7, assign15340_e12880_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign15340_e12880;
        var_ltat_dn5 = assign15340_e12880_d_n5;
        var_ltat_dn6 = assign15340_e12880_d_n6;
        var_ltat_dn7 = assign15340_e12880_d_n7;
        var_ltat_dn8 = assign15340_e12880_d_n8;

        let (assign15350_e12886, assign15350_e12886_d_n5, assign15350_e12886_d_n6, assign15350_e12886_d_n7, assign15350_e12886_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign15350_e12886;
        var_mtat_dn5 = assign15350_e12886_d_n5;
        var_mtat_dn6 = assign15350_e12886_d_n6;
        var_mtat_dn7 = assign15350_e12886_d_n7;
        var_mtat_dn8 = assign15350_e12886_d_n8;

        let (assign15360_e12892, assign15360_e12892_d_n5, assign15360_e12892_d_n6, assign15360_e12892_d_n7, assign15360_e12892_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign15360_e12892;
        var_xerfc_dn5 = assign15360_e12892_d_n5;
        var_xerfc_dn6 = assign15360_e12892_d_n6;
        var_xerfc_dn7 = assign15360_e12892_d_n7;
        var_xerfc_dn8 = assign15360_e12892_d_n8;

        let (assign15370_e12898, assign15370_e12898_d_n5, assign15370_e12898_d_n6, assign15370_e12898_d_n7, assign15370_e12898_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign15370_e12898;
        var_erfctimesexpmtat_dn5 = assign15370_e12898_d_n5;
        var_erfctimesexpmtat_dn6 = assign15370_e12898_d_n6;
        var_erfctimesexpmtat_dn7 = assign15370_e12898_d_n7;
        var_erfctimesexpmtat_dn8 = assign15370_e12898_d_n8;

        let (assign15380_e12904, assign15380_e12904_d_n5, assign15380_e12904_d_n6, assign15380_e12904_d_n7, assign15380_e12904_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign15380_e12904;
        var_gammamax_dn5 = assign15380_e12904_d_n5;
        var_gammamax_dn6 = assign15380_e12904_d_n6;
        var_gammamax_dn7 = assign15380_e12904_d_n7;
        var_gammamax_dn8 = assign15380_e12904_d_n8;

        let (assign15390_e12910, assign15390_e12910_d_n5, assign15390_e12910_d_n6, assign15390_e12910_d_n7, assign15390_e12910_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign15390_e12910;
        var_ibbt_dn5 = assign15390_e12910_d_n5;
        var_ibbt_dn6 = assign15390_e12910_d_n6;
        var_ibbt_dn7 = assign15390_e12910_d_n7;
        var_ibbt_dn8 = assign15390_e12910_d_n8;

        let (assign15400_e12916, assign15400_e12916_d_n5, assign15400_e12916_d_n6, assign15400_e12916_d_n7, assign15400_e12916_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign15400_e12916;
        var_fmaxr_dn5 = assign15400_e12916_d_n5;
        var_fmaxr_dn6 = assign15400_e12916_d_n6;
        var_fmaxr_dn7 = assign15400_e12916_d_n7;
        var_fmaxr_dn8 = assign15400_e12916_d_n8;

        let (assign15410_e12922, assign15410_e12922_d_n5, assign15410_e12922_d_n6, assign15410_e12922_d_n7, assign15410_e12922_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign15410_e12922;
        var_fbreakdown_dn5 = assign15410_e12922_d_n5;
        var_fbreakdown_dn6 = assign15410_e12922_d_n6;
        var_fbreakdown_dn7 = assign15410_e12922_d_n7;
        var_fbreakdown_dn8 = assign15410_e12922_d_n8;

        let (assign15420_e12928,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign15420_e12928;

        let (assign15430_e12934,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign15430_e12934;

        let (assign15440_e12940,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign15440_e12940;

        let (assign15450_e12949,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign15450_e12945: f64 = (-var_fracna);
        let assign15450_e12947: f64 = (assign15450_e12945 * p.p928);
        (assign15450_e12947,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign15450_e12949;

        let (assign15460_e12958,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign15460_e12954: f64 = (-var_fracnb);
        let assign15460_e12956: f64 = (assign15460_e12954 * p.p928);
        (assign15460_e12956,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign15460_e12958;

        let (assign15470_e12967,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign15470_e12963: f64 = (-var_fraci);
        let assign15470_e12965: f64 = (assign15470_e12963 * p.p928);
        (assign15470_e12965,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign15470_e12967;

        let (assign15480_e12973,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign15480_e12973;

        let (assign15490_e12979,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign15490_e12979;

        let (assign15500_e12985,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign15500_e12985;

        let (assign15510_e12991,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign15510_e12991;

        let assign15520_e13003: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard248 = assign15520_e13003;

        let assign15600_e13089: f64 = if var_v1 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard249 = assign15600_e13089;

        let assign15610_e13091: f64 = (-0.5);
        let assign15610_e13094: f64 = (var_v1 * var_phitdinv);
        let assign15610_e13095: f64 = (assign15610_e13091 * assign15610_e13094);
        let assign15610_e13096: f64 = (assign15610_e13095).abs();
        let assign15610_e13098: f64 = if assign15610_e13096 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard250 = assign15610_e13098;

        let (assign15620_e13116,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) && (var_guard250 != 0.0)) {
        let assign15620_e13109: f64 = (-0.5);
        let assign15620_e13112: f64 = (var_v1 * var_phitdinv);
        let assign15620_e13113: f64 = (assign15620_e13109 * assign15620_e13112);
        let assign15620_e13114: f64 = (assign15620_e13113).exp();
        (assign15620_e13114,)
    } else {
        (var_z,)
    }
};
        var_z = assign15620_e13116;

        let assign15630_e13118: f64 = (-0.5);
        let assign15630_e13121: f64 = (var_v1 * var_phitdinv);
        let assign15630_e13122: f64 = (assign15630_e13118 * assign15630_e13121);
        let assign15630_e13124: f64 = if assign15630_e13122 < 0.0 { 1.0 } else { 0.0 };
        var_guard251 = assign15630_e13124;

        let (assign15640_e13179,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) && (var_guard250 == 0.0)) && (var_guard251 != 0.0)) {
        let assign15640_e13140: f64 = (-230.25850929940458);
        let assign15640_e13142: f64 = (-0.5);
        let assign15640_e13145: f64 = (var_v1 * var_phitdinv);
        let assign15640_e13146: f64 = (assign15640_e13142 * assign15640_e13145);
        let assign15640_e13147: f64 = (assign15640_e13140 - assign15640_e13146);
        let assign15640_e13151: f64 = (-230.25850929940458);
        let assign15640_e13153: f64 = (-0.5);
        let assign15640_e13156: f64 = (var_v1 * var_phitdinv);
        let assign15640_e13157: f64 = (assign15640_e13153 * assign15640_e13156);
        let assign15640_e13158: f64 = (assign15640_e13151 - assign15640_e13157);
        let assign15640_e13161: f64 = (-230.25850929940458);
        let assign15640_e13163: f64 = (-0.5);
        let assign15640_e13166: f64 = (var_v1 * var_phitdinv);
        let assign15640_e13167: f64 = (assign15640_e13163 * assign15640_e13166);
        let assign15640_e13168: f64 = (assign15640_e13161 - assign15640_e13167);
        let assign15640_e13170: f64 = (assign15640_e13168 * 0.3333333333333333);
        let assign15640_e13171: f64 = (1.0 + assign15640_e13170);
        let assign15640_e13172: f64 = (assign15640_e13158 * assign15640_e13171);
        let assign15640_e13173: f64 = (0.5 * assign15640_e13172);
        let assign15640_e13174: f64 = (1.0 + assign15640_e13173);
        let assign15640_e13175: f64 = (assign15640_e13147 * assign15640_e13174);
        let assign15640_e13176: f64 = (1.0 + assign15640_e13175);
        let assign15640_e13177: f64 = (1e-100 / assign15640_e13176);
        (assign15640_e13177,)
    } else {
        (var_z,)
    }
};
        var_z = assign15640_e13179;

        let (assign15650_e13232,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) && (var_guard250 == 0.0)) && (var_guard251 == 0.0)) {
        let assign15650_e13196: f64 = (-0.5);
        let assign15650_e13199: f64 = (var_v1 * var_phitdinv);
        let assign15650_e13200: f64 = (assign15650_e13196 * assign15650_e13199);
        let assign15650_e13202: f64 = (assign15650_e13200 - 230.25850929940458);
        let assign15650_e13206: f64 = (-0.5);
        let assign15650_e13209: f64 = (var_v1 * var_phitdinv);
        let assign15650_e13210: f64 = (assign15650_e13206 * assign15650_e13209);
        let assign15650_e13212: f64 = (assign15650_e13210 - 230.25850929940458);
        let assign15650_e13215: f64 = (-0.5);
        let assign15650_e13218: f64 = (var_v1 * var_phitdinv);
        let assign15650_e13219: f64 = (assign15650_e13215 * assign15650_e13218);
        let assign15650_e13221: f64 = (assign15650_e13219 - 230.25850929940458);
        let assign15650_e13223: f64 = (assign15650_e13221 * 0.3333333333333333);
        let assign15650_e13224: f64 = (1.0 + assign15650_e13223);
        let assign15650_e13225: f64 = (assign15650_e13212 * assign15650_e13224);
        let assign15650_e13226: f64 = (0.5 * assign15650_e13225);
        let assign15650_e13227: f64 = (1.0 + assign15650_e13226);
        let assign15650_e13228: f64 = (assign15650_e13202 * assign15650_e13227);
        let assign15650_e13229: f64 = (1.0 + assign15650_e13228);
        let assign15650_e13230: f64 = (1e100 * assign15650_e13229);
        (assign15650_e13230,)
    } else {
        (var_z,)
    }
};
        var_z = assign15650_e13232;

        let (assign15660_e13244,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        let assign15660_e13242: f64 = (1.0 / var_z);
        (assign15660_e13242,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign15660_e13244;

        let (assign15670_e13256,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 != 0.0)) {
        let assign15670_e13254: f64 = (var_zinv * var_zinv);
        (assign15670_e13254,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign15670_e13256;

        let (assign15680_e13275,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) {
        let assign15680_e13268: f64 = (var_v1 - var_vmax_s);
        let assign15680_e13270: f64 = (assign15680_e13268 * var_phitdinv);
        let assign15680_e13271: f64 = (1.0 + assign15680_e13270);
        let assign15680_e13273: f64 = (assign15680_e13271 * var_exp_vmax_over_phitd_s);
        (assign15680_e13273,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign15680_e13275;

        let (assign15690_e13287,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) {
        let assign15690_e13285: f64 = (var_idmult).sqrt();
        (assign15690_e13285,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign15690_e13287;

        let (assign15700_e13300,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard249 == 0.0)) {
        let assign15700_e13298: f64 = (1.0 / var_zinv);
        (assign15700_e13298,)
    } else {
        (var_z,)
    }
};
        var_z = assign15700_e13300;

        let (assign15710_e13310,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) {
        let assign15710_e13308: f64 = (var_idmult - 1.0);
        (assign15710_e13308,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign15710_e13310;

        let assign15720_e13313: f64 = if var_v1 > 0.0 { 1.0 } else { 0.0 };
        var_guard252 = assign15720_e13313;

        let (assign15730_e13339,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard252 != 0.0)) {
        let assign15730_e13325: f64 = (2.0 + var_z);
        let assign15730_e13328: f64 = (var_z + 1.0);
        let assign15730_e13331: f64 = (var_z + 3.0);
        let assign15730_e13332: f64 = (assign15730_e13328 * assign15730_e13331);
        let assign15730_e13333: f64 = (assign15730_e13332).sqrt();
        let assign15730_e13334: f64 = (assign15730_e13325 + assign15730_e13333);
        let assign15730_e13335: f64 = (assign15730_e13334).ln();
        let assign15730_e13336: f64 = (var_phitd * assign15730_e13335);
        let assign15730_e13337: f64 = (2.0 * assign15730_e13336);
        (assign15730_e13337,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign15730_e13339;

        let (assign15740_e13373,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) && (var_guard252 == 0.0)) {
        let assign15740_e13349: f64 = (-var_v1);
        let assign15740_e13354: f64 = (2.0 * var_zinv);
        let assign15740_e13356: f64 = (assign15740_e13354 + 1.0);
        let assign15740_e13359: f64 = (1.0 + var_zinv);
        let assign15740_e13363: f64 = (3.0 * var_zinv);
        let assign15740_e13364: f64 = (1.0 + assign15740_e13363);
        let assign15740_e13365: f64 = (assign15740_e13359 * assign15740_e13364);
        let assign15740_e13366: f64 = (assign15740_e13365).sqrt();
        let assign15740_e13367: f64 = (assign15740_e13356 + assign15740_e13366);
        let assign15740_e13368: f64 = (assign15740_e13367).ln();
        let assign15740_e13369: f64 = (var_phitd * assign15740_e13368);
        let assign15740_e13370: f64 = (2.0 * assign15740_e13369);
        let assign15740_e13371: f64 = (assign15740_e13349 + assign15740_e13370);
        (assign15740_e13371,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign15740_e13373;

        let (assign15750_e13383,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) {
        let assign15750_e13381: f64 = (var_vbimin_s - var_two_psistar);
        (assign15750_e13381,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign15750_e13383;

        let (assign15760_e13410,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) {
        let assign15760_e13392: f64 = (var_v1 + var_vjlim);
        let assign15760_e13395: f64 = (var_v1 - var_vjlim);
        let assign15760_e13398: f64 = (var_v1 - var_vjlim);
        let assign15760_e13399: f64 = (assign15760_e13395 * assign15760_e13398);
        let assign15760_e13402: f64 = (4.0 * var_phitd);
        let assign15760_e13404: f64 = (assign15760_e13402 * var_phitd);
        let assign15760_e13405: f64 = (assign15760_e13399 + assign15760_e13404);
        let assign15760_e13406: f64 = (assign15760_e13405).sqrt();
        let assign15760_e13407: f64 = (assign15760_e13392 - assign15760_e13406);
        let assign15760_e13408: f64 = (0.5 * assign15760_e13407);
        (assign15760_e13408,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign15760_e13410;

        let (assign15770_e13437,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) {
        let assign15770_e13419: f64 = (var_v1 + var_vbbtlim_s);
        let assign15770_e13422: f64 = (var_v1 - var_vbbtlim_s);
        let assign15770_e13425: f64 = (var_v1 - var_vbbtlim_s);
        let assign15770_e13426: f64 = (assign15770_e13422 * assign15770_e13425);
        let assign15770_e13429: f64 = (4.0 * var_phitr);
        let assign15770_e13431: f64 = (assign15770_e13429 * var_phitr);
        let assign15770_e13432: f64 = (assign15770_e13426 + assign15770_e13431);
        let assign15770_e13433: f64 = (assign15770_e13432).sqrt();
        let assign15770_e13434: f64 = (assign15770_e13419 - assign15770_e13433);
        let assign15770_e13435: f64 = (0.5 * assign15770_e13434);
        (assign15770_e13435,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign15770_e13437;

        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fraci_slot = var_fraci;
        *var_fracna_slot = var_fracna;
        *var_fracnb_slot = var_fracnb;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard248_slot = var_guard248;
        *var_guard249_slot = var_guard249;
        *var_guard250_slot = var_guard250;
        *var_guard251_slot = var_guard251;
        *var_guard252_slot = var_guard252;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_v1_slot = var_v1;
        *var_v2_slot = var_v2;
        *var_v3_slot = var_v3;
        *var_v4_slot = var_v4;
        *var_v5_slot = var_v5;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard248: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vbibot: f64,
        var_vbirbotinv: f64,
        var_vjsrh: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard256_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_guard258_slot: &mut f64,
        var_guard259_slot: &mut f64,
        var_guard260_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard256: f64 = *var_guard256_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_guard258: f64 = *var_guard258_slot;
        let mut var_guard259: f64 = *var_guard259_slot;
        let mut var_guard260: f64 = *var_guard260_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign15780_e13464,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard248 != 0.0)) {
        let assign15780_e13446: f64 = var_v1;
        let assign15780_e13449: f64 = var_v1;
        let assign15780_e13452: f64 = var_v1;
        let assign15780_e13453: f64 = (assign15780_e13449 * assign15780_e13452);
        let assign15780_e13456: f64 = (4.0 * 1e-6);
        let assign15780_e13458: f64 = (assign15780_e13456 * 1e-6);
        let assign15780_e13459: f64 = (assign15780_e13453 + assign15780_e13458);
        let assign15780_e13460: f64 = (assign15780_e13459).sqrt();
        let assign15780_e13461: f64 = (assign15780_e13446 - assign15780_e13460);
        let assign15780_e13462: f64 = (0.5 * assign15780_e13461);
        (assign15780_e13462,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign15780_e13464;

        let assign15790_e13467: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard253 = assign15790_e13467;

        let (assign15800_e13475, assign15800_e13475_d_n5, assign15800_e13475_d_n6, assign15800_e13475_d_n7, assign15800_e13475_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign15800_e13475;
        var_ijunbot_dn5 = assign15800_e13475_d_n5;
        var_ijunbot_dn6 = assign15800_e13475_d_n6;
        var_ijunbot_dn7 = assign15800_e13475_d_n7;
        var_ijunbot_dn8 = assign15800_e13475_d_n8;

        let (assign15810_e13486,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) {
        let assign15810_e13484: f64 = (var_idsatbot * var_idmult);
        (assign15810_e13484,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign15810_e13486;

        let assign15820_e13493: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        var_guard254 = assign15820_e13493;

        let (assign15830_e13504, assign15830_e13504_d_n5, assign15830_e13504_d_n6, assign15830_e13504_d_n7, assign15830_e13504_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign15830_e13504;
        var_isrh_dn5 = assign15830_e13504_d_n5;
        var_isrh_dn6 = assign15830_e13504_d_n6;
        var_isrh_dn7 = assign15830_e13504_d_n7;
        var_isrh_dn8 = assign15830_e13504_d_n8;

        let (assign15840_e13518,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15840_e13516: f64 = (var_vbibot - var_vjsrh);
        (assign15840_e13516,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign15840_e13518;

        let (assign15850_e13537,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15850_e13532: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign15850_e13533: f64 = (1.0 - assign15850_e13532);
        let assign15850_e13534: f64 = (assign15850_e13533).sqrt();
        let assign15850_e13535: f64 = (1.0 - assign15850_e13534);
        (assign15850_e13535,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign15850_e13537;

        let assign15860_e13540: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard255 = assign15860_e13540;

        let (assign15870_e13554,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) && (var_guard255 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign15870_e13554;

        let (assign15880_e13586,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) && (var_guard255 == 0.0)) {
        let assign15880_e13569: f64 = (var_wsrhstep * var_wsrhstep);
        let assign15880_e13571: f64 = (var_wsrhstep).ln();
        let assign15880_e13572: f64 = (assign15880_e13569 * assign15880_e13571);
        let assign15880_e13575: f64 = (1.0 - var_wsrhstep);
        let assign15880_e13576: f64 = (assign15880_e13572 / assign15880_e13575);
        let assign15880_e13578: f64 = (assign15880_e13576 + var_wsrhstep);
        let assign15880_e13582: f64 = (2.0 * p.p831);
        let assign15880_e13583: f64 = (1.0 - assign15880_e13582);
        let assign15880_e13584: f64 = (assign15880_e13578 * assign15880_e13583);
        (assign15880_e13584,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign15880_e13586;

        let (assign15890_e13600,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15890_e13598: f64 = (var_wsrhstep + var_dwsrh);
        (assign15890_e13598,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign15890_e13600;

        let assign15900_e13603: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard256 = assign15900_e13603;

        let (assign15910_e13620, assign15910_e13620_d_n5, assign15910_e13620_d_n6, assign15910_e13620_d_n7, assign15910_e13620_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) && (var_guard256 != 0.0)) {
        let assign15910_e13617: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign15910_e13618: f64 = (assign15910_e13617).sqrt();
        (assign15910_e13618, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign15910_e13620;
        var_tmp_dn5 = assign15910_e13620_d_n5;
        var_tmp_dn6 = assign15910_e13620_d_n6;
        var_tmp_dn7 = assign15910_e13620_d_n7;
        var_tmp_dn8 = assign15910_e13620_d_n8;

        let (assign15920_e13639, assign15920_e13639_d_n5, assign15920_e13639_d_n6, assign15920_e13639_d_n7, assign15920_e13639_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) && (var_guard256 == 0.0)) {
        let assign15920_e13635: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign15920_e13637: f64 = (assign15920_e13635).powf(p.p831);
        (assign15920_e13637, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign15920_e13639;
        var_tmp_dn5 = assign15920_e13639_d_n5;
        var_tmp_dn6 = assign15920_e13639_d_n6;
        var_tmp_dn7 = assign15920_e13639_d_n7;
        var_tmp_dn8 = assign15920_e13639_d_n8;

        let (assign15930_e13653, assign15930_e13653_d_n5, assign15930_e13653_d_n6, assign15930_e13653_d_n7, assign15930_e13653_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15930_e13651: f64 = (var_wdepnulrbot * var_tmp);
        (assign15930_e13651, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign15930_e13653;
        var_wdep_dn5 = assign15930_e13653_d_n5;
        var_wdep_dn6 = assign15930_e13653_d_n6;
        var_wdep_dn7 = assign15930_e13653_d_n7;
        var_wdep_dn8 = assign15930_e13653_d_n8;

        let (assign15940_e13671, assign15940_e13671_d_n5, assign15940_e13671_d_n6, assign15940_e13671_d_n7, assign15940_e13671_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15940_e13666: f64 = (var_zinv - 1.0);
        let assign15940_e13668: f64 = (assign15940_e13666 * var_wdep);
        let assign15940_e13669: f64 = (var_ftdbot * assign15940_e13668);
        (assign15940_e13669, (var_ftdbot * (assign15940_e13666 * var_wdep_dn5)), (var_ftdbot * (assign15940_e13666 * var_wdep_dn6)), (var_ftdbot * (assign15940_e13666 * var_wdep_dn7)), (var_ftdbot * (assign15940_e13666 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign15940_e13671;
        var_asrh_dn5 = assign15940_e13671_d_n5;
        var_asrh_dn6 = assign15940_e13671_d_n6;
        var_asrh_dn7 = assign15940_e13671_d_n7;
        var_asrh_dn8 = assign15940_e13671_d_n8;

        let (assign15950_e13687, assign15950_e13687_d_n5, assign15950_e13687_d_n6, assign15950_e13687_d_n7, assign15950_e13687_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15950_e13684: f64 = (var_asrh * var_wsrh);
        let assign15950_e13685: f64 = (p.p840 * assign15950_e13684);
        (assign15950_e13685, (p.p840 * (var_asrh_dn5 * var_wsrh)), (p.p840 * (var_asrh_dn6 * var_wsrh)), (p.p840 * (var_asrh_dn7 * var_wsrh)), (p.p840 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign15950_e13687;
        var_isrh_dn5 = assign15950_e13687_d_n5;
        var_isrh_dn6 = assign15950_e13687_d_n6;
        var_isrh_dn7 = assign15950_e13687_d_n7;
        var_isrh_dn8 = assign15950_e13687_d_n8;

        let assign15960_e13690: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard257 = assign15960_e13690;

        let (assign15970_e13701, assign15970_e13701_d_n5, assign15970_e13701_d_n6, assign15970_e13701_d_n7, assign15970_e13701_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign15970_e13701;
        var_itat_dn5 = assign15970_e13701_d_n5;
        var_itat_dn6 = assign15970_e13701_d_n6;
        var_itat_dn7 = assign15970_e13701_d_n7;
        var_itat_dn8 = assign15970_e13701_d_n8;

        let (assign15980_e13719, assign15980_e13719_d_n5, assign15980_e13719_d_n6, assign15980_e13719_d_n7, assign15980_e13719_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign15980_e13714: f64 = (var_wdep * var_one_minus_pbot);
        let assign15980_e13716: f64 = (assign15980_e13714 / var_vbi_minus_vjsrh);
        let assign15980_e13717: f64 = (var_btatpartbot * assign15980_e13716);
        (assign15980_e13717, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign15980_e13719;
        var_btat_dn5 = assign15980_e13719_d_n5;
        var_btat_dn6 = assign15980_e13719_d_n6;
        var_btat_dn7 = assign15980_e13719_d_n7;
        var_btat_dn8 = assign15980_e13719_d_n8;

        let (assign15990_e13735, assign15990_e13735_d_n5, assign15990_e13735_d_n6, assign15990_e13735_d_n7, assign15990_e13735_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign15990_e13731: f64 = (0.666666666666667 * var_atatbot);
        let assign15990_e13733: f64 = (assign15990_e13731 / var_btat);
        (assign15990_e13733, (-((assign15990_e13731 * var_btat_dn5) / (var_btat * var_btat))), (-((assign15990_e13731 * var_btat_dn6) / (var_btat * var_btat))), (-((assign15990_e13731 * var_btat_dn7) / (var_btat * var_btat))), (-((assign15990_e13731 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign15990_e13735;
        var_twoatatoverthreebtat_dn5 = assign15990_e13735_d_n5;
        var_twoatatoverthreebtat_dn6 = assign15990_e13735_d_n6;
        var_twoatatoverthreebtat_dn7 = assign15990_e13735_d_n7;
        var_twoatatoverthreebtat_dn8 = assign15990_e13735_d_n8;

        let (assign16000_e13749, assign16000_e13749_d_n5, assign16000_e13749_d_n6, assign16000_e13749_d_n7, assign16000_e13749_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16000_e13747: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign16000_e13747, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign16000_e13749;
        var_umaxbeforelimiting_dn5 = assign16000_e13749_d_n5;
        var_umaxbeforelimiting_dn6 = assign16000_e13749_d_n6;
        var_umaxbeforelimiting_dn7 = assign16000_e13749_d_n7;
        var_umaxbeforelimiting_dn8 = assign16000_e13749_d_n8;

        let (assign16010_e13770, assign16010_e13770_d_n5, assign16010_e13770_d_n6, assign16010_e13770_d_n7, assign16010_e13770_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16010_e13761: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16010_e13764: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16010_e13766: f64 = (assign16010_e13764 + 1.0);
        let assign16010_e13767: f64 = (assign16010_e13761 / assign16010_e13766);
        let assign16010_e13768: f64 = (assign16010_e13767).sqrt();
        (assign16010_e13768, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign16010_e13766) - (assign16010_e13761 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign16010_e13766) - (assign16010_e13761 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign16010_e13766) - (assign16010_e13761 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign16010_e13766) - (assign16010_e13761 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign16010_e13766 * assign16010_e13766)) / (2.0 * assign16010_e13768)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign16010_e13770;
        var_umax_dn5 = assign16010_e13770_d_n5;
        var_umax_dn6 = assign16010_e13770_d_n6;
        var_umax_dn7 = assign16010_e13770_d_n7;
        var_umax_dn8 = assign16010_e13770_d_n8;

        let (assign16020_e13783, assign16020_e13783_d_n5, assign16020_e13783_d_n6, assign16020_e13783_d_n7, assign16020_e13783_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16020_e13781: f64 = (var_umax).sqrt();
        (assign16020_e13781, (var_umax_dn5 / (2.0 * assign16020_e13781)), (var_umax_dn6 / (2.0 * assign16020_e13781)), (var_umax_dn7 / (2.0 * assign16020_e13781)), (var_umax_dn8 / (2.0 * assign16020_e13781)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign16020_e13783;
        var_sqrtumax_dn5 = assign16020_e13783_d_n5;
        var_sqrtumax_dn6 = assign16020_e13783_d_n6;
        var_sqrtumax_dn7 = assign16020_e13783_d_n7;
        var_sqrtumax_dn8 = assign16020_e13783_d_n8;

        let (assign16030_e13797, assign16030_e13797_d_n5, assign16030_e13797_d_n6, assign16030_e13797_d_n7, assign16030_e13797_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16030_e13795: f64 = (var_umax * var_sqrtumax);
        (assign16030_e13795, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign16030_e13797;
        var_umaxpoweronepointfive_dn5 = assign16030_e13797_d_n5;
        var_umaxpoweronepointfive_dn6 = assign16030_e13797_d_n6;
        var_umaxpoweronepointfive_dn7 = assign16030_e13797_d_n7;
        var_umaxpoweronepointfive_dn8 = assign16030_e13797_d_n8;

        let assign16040_e13799: f64 = (-p.p831);
        let assign16040_e13801: f64 = (assign16040_e13799 * var_one_over_one_minus_pbot);
        let assign16040_e13803: f64 = (-1.0);
        let assign16040_e13804: f64 = if assign16040_e13801 == assign16040_e13803 { 1.0 } else { 0.0 };
        var_guard258 = assign16040_e13804;

        let (assign16050_e13824, assign16050_e13824_d_n5, assign16050_e13824_d_n6, assign16050_e13824_d_n7, assign16050_e13824_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard258 != 0.0)) {
        let assign16050_e13820: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16050_e13821: f64 = (1.0 + assign16050_e13820);
        let assign16050_e13822: f64 = (1.0 / assign16050_e13821);
        (assign16050_e13822, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign16050_e13821 * assign16050_e13821))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign16050_e13821 * assign16050_e13821))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign16050_e13821 * assign16050_e13821))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign16050_e13821 * assign16050_e13821))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign16050_e13824;
        var_wgamma_dn5 = assign16050_e13824_d_n5;
        var_wgamma_dn6 = assign16050_e13824_d_n6;
        var_wgamma_dn7 = assign16050_e13824_d_n7;
        var_wgamma_dn8 = assign16050_e13824_d_n8;

        let (assign16060_e13848, assign16060_e13848_d_n5, assign16060_e13848_d_n6, assign16060_e13848_d_n7, assign16060_e13848_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard258 == 0.0)) {
        let assign16060_e13840: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16060_e13841: f64 = (1.0 + assign16060_e13840);
        let assign16060_e13843: f64 = (-p.p831);
        let assign16060_e13845: f64 = (assign16060_e13843 * var_one_over_one_minus_pbot);
        let assign16060_e13846: f64 = (assign16060_e13841).powf(assign16060_e13845);
        (assign16060_e13846, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign16060_e13841))) }, if 0.0 == 0.0 && ((assign16060_e13845) as f64).is_finite() && ((assign16060_e13845) as f64).fract() == 0.0 { if assign16060_e13845 == 0.0 { 0.0 } else { (assign16060_e13845 * ((assign16060_e13841).powf(assign16060_e13845 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign16060_e13846 * (assign16060_e13845 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign16060_e13841))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign16060_e13848;
        var_wgamma_dn5 = assign16060_e13848_d_n5;
        var_wgamma_dn6 = assign16060_e13848_d_n6;
        var_wgamma_dn7 = assign16060_e13848_d_n7;
        var_wgamma_dn8 = assign16060_e13848_d_n8;

        let (assign16070_e13866, assign16070_e13866_d_n5, assign16070_e13866_d_n6, assign16070_e13866_d_n7, assign16070_e13866_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16070_e13860: f64 = (var_wsrh * var_wgamma);
        let assign16070_e13863: f64 = (var_wsrh + var_wgamma);
        let assign16070_e13864: f64 = (assign16070_e13860 / assign16070_e13863);
        (assign16070_e13864, ((((var_wsrh * var_wgamma_dn5) * assign16070_e13863) - (assign16070_e13860 * var_wgamma_dn5)) / (assign16070_e13863 * assign16070_e13863)), ((((var_wsrh * var_wgamma_dn6) * assign16070_e13863) - (assign16070_e13860 * var_wgamma_dn6)) / (assign16070_e13863 * assign16070_e13863)), ((((var_wsrh * var_wgamma_dn7) * assign16070_e13863) - (assign16070_e13860 * var_wgamma_dn7)) / (assign16070_e13863 * assign16070_e13863)), ((((var_wsrh * var_wgamma_dn8) * assign16070_e13863) - (assign16070_e13860 * var_wgamma_dn8)) / (assign16070_e13863 * assign16070_e13863)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign16070_e13866;
        var_wtat_dn5 = assign16070_e13866_d_n5;
        var_wtat_dn6 = assign16070_e13866_d_n6;
        var_wtat_dn7 = assign16070_e13866_d_n7;
        var_wtat_dn8 = assign16070_e13866_d_n8;

        let (assign16080_e13883, assign16080_e13883_d_n5, assign16080_e13883_d_n6, assign16080_e13883_d_n7, assign16080_e13883_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16080_e13879: f64 = (var_btat / var_sqrtumax);
        let assign16080_e13880: f64 = (0.375 * assign16080_e13879);
        let assign16080_e13881: f64 = (assign16080_e13880).sqrt();
        (assign16080_e13881, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16080_e13881)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16080_e13881)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign16080_e13883;
        var_ktat_dn5 = assign16080_e13883_d_n5;
        var_ktat_dn6 = assign16080_e13883_d_n6;
        var_ktat_dn7 = assign16080_e13883_d_n7;
        var_ktat_dn8 = assign16080_e13883_d_n8;

        let (assign16090_e13901, assign16090_e13901_d_n5, assign16090_e13901_d_n6, assign16090_e13901_d_n7, assign16090_e13901_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16090_e13896: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign16090_e13897: f64 = (2.0 * assign16090_e13896);
        let assign16090_e13899: f64 = (assign16090_e13897 - var_umax);
        (assign16090_e13899, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign16090_e13901;
        var_ltat_dn5 = assign16090_e13901_d_n5;
        var_ltat_dn6 = assign16090_e13901_d_n6;
        var_ltat_dn7 = assign16090_e13901_d_n7;
        var_ltat_dn8 = assign16090_e13901_d_n8;

        let (assign16100_e13927, assign16100_e13927_d_n5, assign16100_e13927_d_n6, assign16100_e13927_d_n7, assign16100_e13927_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16100_e13913: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign16100_e13915: f64 = (assign16100_e13913 * var_sqrtumax);
        let assign16100_e13918: f64 = (var_atatbot * var_umax);
        let assign16100_e13919: f64 = (assign16100_e13915 - assign16100_e13918);
        let assign16100_e13923: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16100_e13924: f64 = (0.5 * assign16100_e13923);
        let assign16100_e13925: f64 = (assign16100_e13919 + assign16100_e13924);
        (assign16100_e13925, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign16100_e13913 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign16100_e13913 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign16100_e13913 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign16100_e13913 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign16100_e13927;
        var_mtat_dn5 = assign16100_e13927_d_n5;
        var_mtat_dn6 = assign16100_e13927_d_n6;
        var_mtat_dn7 = assign16100_e13927_d_n7;
        var_mtat_dn8 = assign16100_e13927_d_n8;

        let (assign16110_e13943, assign16110_e13943_d_n5, assign16110_e13943_d_n6, assign16110_e13943_d_n7, assign16110_e13943_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16110_e13939: f64 = (var_ltat - 1.0);
        let assign16110_e13941: f64 = (assign16110_e13939 * var_ktat);
        (assign16110_e13941, ((var_ltat_dn5 * var_ktat) + (assign16110_e13939 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign16110_e13939 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign16110_e13939 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign16110_e13939 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign16110_e13943;
        var_xerfc_dn5 = assign16110_e13943_d_n5;
        var_xerfc_dn6 = assign16110_e13943_d_n6;
        var_xerfc_dn7 = assign16110_e13943_d_n7;
        var_xerfc_dn8 = assign16110_e13943_d_n8;

        let (assign16120_e13957, assign16120_e13957_d_n5, assign16120_e13957_d_n6, assign16120_e13957_d_n7, assign16120_e13957_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16120_e13955: f64 = (var_xerfc * var_xerfc);
        (assign16120_e13955, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign16120_e13957;
        var_ysq_dn5 = assign16120_e13957_d_n5;
        var_ysq_dn6 = assign16120_e13957_d_n6;
        var_ysq_dn7 = assign16120_e13957_d_n7;
        var_ysq_dn8 = assign16120_e13957_d_n8;

        let assign16130_e13960: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard259 = assign16130_e13960;

        let (assign16140_e13980, assign16140_e13980_d_n5, assign16140_e13980_d_n6, assign16140_e13980_d_n7, assign16140_e13980_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard259 != 0.0)) {
        let assign16140_e13976: f64 = (var_perfc * var_xerfc);
        let assign16140_e13977: f64 = (1.0 + assign16140_e13976);
        let assign16140_e13978: f64 = (1.0 / assign16140_e13977);
        (assign16140_e13978, (-((var_perfc * var_xerfc_dn5) / (assign16140_e13977 * assign16140_e13977))), (-((var_perfc * var_xerfc_dn6) / (assign16140_e13977 * assign16140_e13977))), (-((var_perfc * var_xerfc_dn7) / (assign16140_e13977 * assign16140_e13977))), (-((var_perfc * var_xerfc_dn8) / (assign16140_e13977 * assign16140_e13977))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign16140_e13980;
        var_terfc_dn5 = assign16140_e13980_d_n5;
        var_terfc_dn6 = assign16140_e13980_d_n6;
        var_terfc_dn7 = assign16140_e13980_d_n7;
        var_terfc_dn8 = assign16140_e13980_d_n8;

        let (assign16150_e14001, assign16150_e14001_d_n5, assign16150_e14001_d_n6, assign16150_e14001_d_n7, assign16150_e14001_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard259 == 0.0)) {
        let assign16150_e13997: f64 = (var_perfc * var_xerfc);
        let assign16150_e13998: f64 = (1.0 - assign16150_e13997);
        let assign16150_e13999: f64 = (1.0 / assign16150_e13998);
        (assign16150_e13999, (-((-(var_perfc * var_xerfc_dn5)) / (assign16150_e13998 * assign16150_e13998))), (-((-(var_perfc * var_xerfc_dn6)) / (assign16150_e13998 * assign16150_e13998))), (-((-(var_perfc * var_xerfc_dn7)) / (assign16150_e13998 * assign16150_e13998))), (-((-(var_perfc * var_xerfc_dn8)) / (assign16150_e13998 * assign16150_e13998))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign16150_e14001;
        var_terfc_dn5 = assign16150_e14001_d_n5;
        var_terfc_dn6 = assign16150_e14001_d_n6;
        var_terfc_dn7 = assign16150_e14001_d_n7;
        var_terfc_dn8 = assign16150_e14001_d_n8;

        let assign16160_e14003: f64 = (-var_ysq);
        let assign16160_e14005: f64 = (assign16160_e14003 + var_mtat);
        let assign16160_e14007: f64 = (-230.25850929940458);
        let assign16160_e14008: f64 = if assign16160_e14005 > assign16160_e14007 { 1.0 } else { 0.0 };
        var_guard260 = assign16160_e14008;

        let (assign16170_e14026, assign16170_e14026_d_n5, assign16170_e14026_d_n6, assign16170_e14026_d_n7, assign16170_e14026_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard260 != 0.0)) {
        let assign16170_e14021: f64 = (-var_ysq);
        let assign16170_e14023: f64 = (assign16170_e14021 + var_mtat);
        let assign16170_e14024: f64 = (assign16170_e14023).exp();
        (assign16170_e14024, (assign16170_e14024 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign16170_e14024 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign16170_e14024 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign16170_e14024 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16170_e14026;
        var_tmp_dn5 = assign16170_e14026_d_n5;
        var_tmp_dn6 = assign16170_e14026_d_n6;
        var_tmp_dn7 = assign16170_e14026_d_n7;
        var_tmp_dn8 = assign16170_e14026_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard253_slot = var_guard253;
        *var_guard254_slot = var_guard254;
        *var_guard255_slot = var_guard255;
        *var_guard256_slot = var_guard256;
        *var_guard257_slot = var_guard257;
        *var_guard258_slot = var_guard258;
        *var_guard259_slot = var_guard259;
        *var_guard260_slot = var_guard260;
        *var_id__blk219_slot = var_id__blk219;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard253: f64,
        var_guard257: f64,
        var_guard260: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lssource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_ysq: f64,
        var_ysq_dn5: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard261_slot: &mut f64,
        var_guard262_slot: &mut f64,
        var_guard263_slot: &mut f64,
        var_guard264_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_guard270_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard261: f64 = *var_guard261_slot;
        let mut var_guard262: f64 = *var_guard262_slot;
        let mut var_guard263: f64 = *var_guard263_slot;
        let mut var_guard264: f64 = *var_guard264_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_guard270: f64 = *var_guard270_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;

        let (assign16180_e14075, assign16180_e14075_d_n5, assign16180_e14075_d_n6, assign16180_e14075_d_n7, assign16180_e14075_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard260 == 0.0)) {
        let assign16180_e14042: f64 = (-230.25850929940458);
        let assign16180_e14044: f64 = (-var_ysq);
        let assign16180_e14046: f64 = (assign16180_e14044 + var_mtat);
        let assign16180_e14047: f64 = (assign16180_e14042 - assign16180_e14046);
        let assign16180_e14051: f64 = (-230.25850929940458);
        let assign16180_e14053: f64 = (-var_ysq);
        let assign16180_e14055: f64 = (assign16180_e14053 + var_mtat);
        let assign16180_e14056: f64 = (assign16180_e14051 - assign16180_e14055);
        let assign16180_e14059: f64 = (-230.25850929940458);
        let assign16180_e14061: f64 = (-var_ysq);
        let assign16180_e14063: f64 = (assign16180_e14061 + var_mtat);
        let assign16180_e14064: f64 = (assign16180_e14059 - assign16180_e14063);
        let assign16180_e14066: f64 = (assign16180_e14064 * 0.3333333333333333);
        let assign16180_e14067: f64 = (1.0 + assign16180_e14066);
        let assign16180_e14068: f64 = (assign16180_e14056 * assign16180_e14067);
        let assign16180_e14069: f64 = (0.5 * assign16180_e14068);
        let assign16180_e14070: f64 = (1.0 + assign16180_e14069);
        let assign16180_e14071: f64 = (assign16180_e14047 * assign16180_e14070);
        let assign16180_e14072: f64 = (1.0 + assign16180_e14071);
        let assign16180_e14073: f64 = (1e-100 / assign16180_e14072);
        (assign16180_e14073, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign16180_e14067) + (assign16180_e14056 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16180_e14067) + (assign16180_e14056 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16180_e14067) + (assign16180_e14056 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16180_e14070) + (assign16180_e14047 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16180_e14067) + (assign16180_e14056 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign16180_e14072 * assign16180_e14072))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16180_e14075;
        var_tmp_dn5 = assign16180_e14075_d_n5;
        var_tmp_dn6 = assign16180_e14075_d_n6;
        var_tmp_dn7 = assign16180_e14075_d_n7;
        var_tmp_dn8 = assign16180_e14075_d_n8;

        let (assign16190_e14105, assign16190_e14105_d_n5, assign16190_e14105_d_n6, assign16190_e14105_d_n7, assign16190_e14105_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16190_e14087: f64 = (0.29214664 * var_terfc);
        let assign16190_e14091: f64 = (var_terfc * var_terfc);
        let assign16190_e14092: f64 = (var_berfc * assign16190_e14091);
        let assign16190_e14093: f64 = (assign16190_e14087 + assign16190_e14092);
        let assign16190_e14097: f64 = (var_terfc * var_terfc);
        let assign16190_e14099: f64 = (assign16190_e14097 * var_terfc);
        let assign16190_e14100: f64 = (var_cerfc * assign16190_e14099);
        let assign16190_e14101: f64 = (assign16190_e14093 + assign16190_e14100);
        let assign16190_e14103: f64 = (assign16190_e14101 * var_tmp);
        (assign16190_e14103, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign16190_e14097 * var_terfc_dn5)))) * var_tmp) + (assign16190_e14101 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign16190_e14097 * var_terfc_dn6)))) * var_tmp) + (assign16190_e14101 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign16190_e14097 * var_terfc_dn7)))) * var_tmp) + (assign16190_e14101 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign16190_e14097 * var_terfc_dn8)))) * var_tmp) + (assign16190_e14101 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign16190_e14105;
        var_erfcpos_dn5 = assign16190_e14105_d_n5;
        var_erfcpos_dn6 = assign16190_e14105_d_n6;
        var_erfcpos_dn7 = assign16190_e14105_d_n7;
        var_erfcpos_dn8 = assign16190_e14105_d_n8;

        let assign16200_e14108: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard261 = assign16200_e14108;

        let (assign16210_e14122, assign16210_e14122_d_n5, assign16210_e14122_d_n6, assign16210_e14122_d_n7, assign16210_e14122_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard261 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign16210_e14122;
        var_erfctimesexpmtat_dn5 = assign16210_e14122_d_n5;
        var_erfctimesexpmtat_dn6 = assign16210_e14122_d_n6;
        var_erfctimesexpmtat_dn7 = assign16210_e14122_d_n7;
        var_erfctimesexpmtat_dn8 = assign16210_e14122_d_n8;

        let assign16220_e14125: f64 = (-230.25850929940458);
        let assign16220_e14126: f64 = if var_mtat > assign16220_e14125 { 1.0 } else { 0.0 };
        var_guard262 = assign16220_e14126;

        let (assign16230_e14144, assign16230_e14144_d_n5, assign16230_e14144_d_n6, assign16230_e14144_d_n7, assign16230_e14144_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard261 == 0.0)) && (var_guard262 != 0.0)) {
        let assign16230_e14142: f64 = (var_mtat).exp();
        (assign16230_e14142, (assign16230_e14142 * var_mtat_dn5), (assign16230_e14142 * var_mtat_dn6), (assign16230_e14142 * var_mtat_dn7), (assign16230_e14142 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16230_e14144;
        var_tmp_dn5 = assign16230_e14144_d_n5;
        var_tmp_dn6 = assign16230_e14144_d_n6;
        var_tmp_dn7 = assign16230_e14144_d_n7;
        var_tmp_dn8 = assign16230_e14144_d_n8;

        let (assign16240_e14187, assign16240_e14187_d_n5, assign16240_e14187_d_n6, assign16240_e14187_d_n7, assign16240_e14187_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard261 == 0.0)) && (var_guard262 == 0.0)) {
        let assign16240_e14163: f64 = (-230.25850929940458);
        let assign16240_e14165: f64 = (assign16240_e14163 - var_mtat);
        let assign16240_e14169: f64 = (-230.25850929940458);
        let assign16240_e14171: f64 = (assign16240_e14169 - var_mtat);
        let assign16240_e14174: f64 = (-230.25850929940458);
        let assign16240_e14176: f64 = (assign16240_e14174 - var_mtat);
        let assign16240_e14178: f64 = (assign16240_e14176 * 0.3333333333333333);
        let assign16240_e14179: f64 = (1.0 + assign16240_e14178);
        let assign16240_e14180: f64 = (assign16240_e14171 * assign16240_e14179);
        let assign16240_e14181: f64 = (0.5 * assign16240_e14180);
        let assign16240_e14182: f64 = (1.0 + assign16240_e14181);
        let assign16240_e14183: f64 = (assign16240_e14165 * assign16240_e14182);
        let assign16240_e14184: f64 = (1.0 + assign16240_e14183);
        let assign16240_e14185: f64 = (1e-100 / assign16240_e14184);
        (assign16240_e14185, (-((1e-100 * (((-var_mtat_dn5) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-var_mtat_dn5) * assign16240_e14179) + (assign16240_e14171 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-var_mtat_dn6) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-var_mtat_dn6) * assign16240_e14179) + (assign16240_e14171 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-var_mtat_dn7) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-var_mtat_dn7) * assign16240_e14179) + (assign16240_e14171 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))), (-((1e-100 * (((-var_mtat_dn8) * assign16240_e14182) + (assign16240_e14165 * (0.5 * (((-var_mtat_dn8) * assign16240_e14179) + (assign16240_e14171 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign16240_e14184 * assign16240_e14184))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16240_e14187;
        var_tmp_dn5 = assign16240_e14187_d_n5;
        var_tmp_dn6 = assign16240_e14187_d_n6;
        var_tmp_dn7 = assign16240_e14187_d_n7;
        var_tmp_dn8 = assign16240_e14187_d_n8;

        let (assign16250_e14206, assign16250_e14206_d_n5, assign16250_e14206_d_n6, assign16250_e14206_d_n7, assign16250_e14206_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) && (var_guard261 == 0.0)) {
        let assign16250_e14202: f64 = (2.0 * var_tmp);
        let assign16250_e14204: f64 = (assign16250_e14202 - var_erfcpos);
        (assign16250_e14204, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign16250_e14206;
        var_erfctimesexpmtat_dn5 = assign16250_e14206_d_n5;
        var_erfctimesexpmtat_dn6 = assign16250_e14206_d_n6;
        var_erfctimesexpmtat_dn7 = assign16250_e14206_d_n7;
        var_erfctimesexpmtat_dn8 = assign16250_e14206_d_n8;

        let (assign16260_e14226, assign16260_e14226_d_n5, assign16260_e14226_d_n6, assign16260_e14226_d_n7, assign16260_e14226_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16260_e14218: f64 = (1.772453850905516 * 0.5);
        let assign16260_e14221: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign16260_e14223: f64 = (assign16260_e14221 / var_ktat);
        let assign16260_e14224: f64 = (assign16260_e14218 * assign16260_e14223);
        (assign16260_e14224, (assign16260_e14218 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign16260_e14221 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign16260_e14218 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign16260_e14221 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign16260_e14218 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign16260_e14221 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign16260_e14218 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign16260_e14221 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign16260_e14226;
        var_gammamax_dn5 = assign16260_e14226_d_n5;
        var_gammamax_dn6 = assign16260_e14226_d_n6;
        var_gammamax_dn7 = assign16260_e14226_d_n7;
        var_gammamax_dn8 = assign16260_e14226_d_n8;

        let (assign16270_e14244, assign16270_e14244_d_n5, assign16270_e14244_d_n6, assign16270_e14244_d_n7, assign16270_e14244_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard257 == 0.0)) {
        let assign16270_e14239: f64 = (var_asrh * var_gammamax);
        let assign16270_e14241: f64 = (assign16270_e14239 * var_wtat);
        let assign16270_e14242: f64 = (p.p845 * assign16270_e14241);
        (assign16270_e14242, (p.p845 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign16270_e14239 * var_wtat_dn5))), (p.p845 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign16270_e14239 * var_wtat_dn6))), (p.p845 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign16270_e14239 * var_wtat_dn7))), (p.p845 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign16270_e14239 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign16270_e14244;
        var_itat_dn5 = assign16270_e14244_d_n5;
        var_itat_dn6 = assign16270_e14244_d_n6;
        var_itat_dn7 = assign16270_e14244_d_n7;
        var_itat_dn8 = assign16270_e14244_d_n8;

        let assign16280_e14247: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        var_guard263 = assign16280_e14247;

        let (assign16290_e14258, assign16290_e14258_d_n5, assign16290_e14258_d_n6, assign16290_e14258_d_n7, assign16290_e14258_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign16290_e14258;
        var_ibbt_dn5 = assign16290_e14258_d_n5;
        var_ibbt_dn6 = assign16290_e14258_d_n6;
        var_ibbt_dn7 = assign16290_e14258_d_n7;
        var_ibbt_dn8 = assign16290_e14258_d_n8;

        let assign16300_e14261: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard264 = assign16300_e14261;

        let (assign16310_e14280, assign16310_e14280_d_n5, assign16310_e14280_d_n6, assign16310_e14280_d_n7, assign16310_e14280_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) && (var_guard264 != 0.0)) {
        let assign16310_e14275: f64 = (p.p828 - var_vbbt);
        let assign16310_e14277: f64 = (assign16310_e14275 * var_vbirbotinv);
        let assign16310_e14278: f64 = (assign16310_e14277).sqrt();
        (assign16310_e14278, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16310_e14280;
        var_tmp_dn5 = assign16310_e14280_d_n5;
        var_tmp_dn6 = assign16310_e14280_d_n6;
        var_tmp_dn7 = assign16310_e14280_d_n7;
        var_tmp_dn8 = assign16310_e14280_d_n8;

        let (assign16320_e14301, assign16320_e14301_d_n5, assign16320_e14301_d_n6, assign16320_e14301_d_n7, assign16320_e14301_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign16320_e14295: f64 = (p.p828 - var_vbbt);
        let assign16320_e14297: f64 = (assign16320_e14295 * var_vbirbotinv);
        let assign16320_e14299: f64 = (assign16320_e14297).powf(p.p831);
        (assign16320_e14299, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16320_e14301;
        var_tmp_dn5 = assign16320_e14301_d_n5;
        var_tmp_dn6 = assign16320_e14301_d_n6;
        var_tmp_dn7 = assign16320_e14301_d_n7;
        var_tmp_dn8 = assign16320_e14301_d_n8;

        let (assign16330_e14321, assign16330_e14321_d_n5, assign16330_e14321_d_n6, assign16330_e14321_d_n7, assign16330_e14321_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) {
        let assign16330_e14314: f64 = (p.p828 - var_vbbt);
        let assign16330_e14316: f64 = (assign16330_e14314 * var_wdepnulrinvbot);
        let assign16330_e14318: f64 = (assign16330_e14316 / var_tmp);
        let assign16330_e14319: f64 = (var_one_over_one_minus_pbot * assign16330_e14318);
        (assign16330_e14319, (var_one_over_one_minus_pbot * (-((assign16330_e14316 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign16330_e14316 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign16330_e14316 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign16330_e14316 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign16330_e14321;
        var_fmaxr_dn5 = assign16330_e14321_d_n5;
        var_fmaxr_dn6 = assign16330_e14321_d_n6;
        var_fmaxr_dn7 = assign16330_e14321_d_n7;
        var_fmaxr_dn8 = assign16330_e14321_d_n8;

        let assign16340_e14323: f64 = (-var_fbbtbot);
        let assign16340_e14325: f64 = (assign16340_e14323 / var_fmaxr);
        let assign16340_e14326: f64 = (assign16340_e14325).abs();
        let assign16340_e14328: f64 = if assign16340_e14326 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard265 = assign16340_e14328;

        let (assign16350_e14346, assign16350_e14346_d_n5, assign16350_e14346_d_n6, assign16350_e14346_d_n7, assign16350_e14346_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) && (var_guard265 != 0.0)) {
        let assign16350_e14341: f64 = (-var_fbbtbot);
        let assign16350_e14343: f64 = (assign16350_e14341 / var_fmaxr);
        let assign16350_e14344: f64 = (assign16350_e14343).exp();
        (assign16350_e14344, (assign16350_e14344 * (-((assign16350_e14341 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign16350_e14344 * (-((assign16350_e14341 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16350_e14346;
        var_tmp_dn5 = assign16350_e14346_d_n5;
        var_tmp_dn6 = assign16350_e14346_d_n6;
        var_tmp_dn7 = assign16350_e14346_d_n7;
        var_tmp_dn8 = assign16350_e14346_d_n8;

        let assign16360_e14348: f64 = (-var_fbbtbot);
        let assign16360_e14350: f64 = (assign16360_e14348 / var_fmaxr);
        let assign16360_e14352: f64 = if assign16360_e14350 < 0.0 { 1.0 } else { 0.0 };
        var_guard266 = assign16360_e14352;

        let (assign16370_e14403, assign16370_e14403_d_n5, assign16370_e14403_d_n6, assign16370_e14403_d_n7, assign16370_e14403_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) && (var_guard265 == 0.0)) && (var_guard266 != 0.0)) {
        let assign16370_e14370: f64 = (-230.25850929940458);
        let assign16370_e14372: f64 = (-var_fbbtbot);
        let assign16370_e14374: f64 = (assign16370_e14372 / var_fmaxr);
        let assign16370_e14375: f64 = (assign16370_e14370 - assign16370_e14374);
        let assign16370_e14379: f64 = (-230.25850929940458);
        let assign16370_e14381: f64 = (-var_fbbtbot);
        let assign16370_e14383: f64 = (assign16370_e14381 / var_fmaxr);
        let assign16370_e14384: f64 = (assign16370_e14379 - assign16370_e14383);
        let assign16370_e14387: f64 = (-230.25850929940458);
        let assign16370_e14389: f64 = (-var_fbbtbot);
        let assign16370_e14391: f64 = (assign16370_e14389 / var_fmaxr);
        let assign16370_e14392: f64 = (assign16370_e14387 - assign16370_e14391);
        let assign16370_e14394: f64 = (assign16370_e14392 * 0.3333333333333333);
        let assign16370_e14395: f64 = (1.0 + assign16370_e14394);
        let assign16370_e14396: f64 = (assign16370_e14384 * assign16370_e14395);
        let assign16370_e14397: f64 = (0.5 * assign16370_e14396);
        let assign16370_e14398: f64 = (1.0 + assign16370_e14397);
        let assign16370_e14399: f64 = (assign16370_e14375 * assign16370_e14398);
        let assign16370_e14400: f64 = (1.0 + assign16370_e14399);
        let assign16370_e14401: f64 = (1e-100 / assign16370_e14400);
        (assign16370_e14401, (-((1e-100 * (((-(-((assign16370_e14372 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))), (-((1e-100 * (((-(-((assign16370_e14372 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign16370_e14398) + (assign16370_e14375 * (0.5 * (((-(-((assign16370_e14381 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign16370_e14395) + (assign16370_e14384 * ((-(-((assign16370_e14389 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign16370_e14400 * assign16370_e14400))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16370_e14403;
        var_tmp_dn5 = assign16370_e14403_d_n5;
        var_tmp_dn6 = assign16370_e14403_d_n6;
        var_tmp_dn7 = assign16370_e14403_d_n7;
        var_tmp_dn8 = assign16370_e14403_d_n8;

        let (assign16380_e14452, assign16380_e14452_d_n5, assign16380_e14452_d_n6, assign16380_e14452_d_n7, assign16380_e14452_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) && (var_guard265 == 0.0)) && (var_guard266 == 0.0)) {
        let assign16380_e14422: f64 = (-var_fbbtbot);
        let assign16380_e14424: f64 = (assign16380_e14422 / var_fmaxr);
        let assign16380_e14426: f64 = (assign16380_e14424 - 230.25850929940458);
        let assign16380_e14430: f64 = (-var_fbbtbot);
        let assign16380_e14432: f64 = (assign16380_e14430 / var_fmaxr);
        let assign16380_e14434: f64 = (assign16380_e14432 - 230.25850929940458);
        let assign16380_e14437: f64 = (-var_fbbtbot);
        let assign16380_e14439: f64 = (assign16380_e14437 / var_fmaxr);
        let assign16380_e14441: f64 = (assign16380_e14439 - 230.25850929940458);
        let assign16380_e14443: f64 = (assign16380_e14441 * 0.3333333333333333);
        let assign16380_e14444: f64 = (1.0 + assign16380_e14443);
        let assign16380_e14445: f64 = (assign16380_e14434 * assign16380_e14444);
        let assign16380_e14446: f64 = (0.5 * assign16380_e14445);
        let assign16380_e14447: f64 = (1.0 + assign16380_e14446);
        let assign16380_e14448: f64 = (assign16380_e14426 * assign16380_e14447);
        let assign16380_e14449: f64 = (1.0 + assign16380_e14448);
        let assign16380_e14450: f64 = (1e100 * assign16380_e14449);
        (assign16380_e14450, (1e100 * (((-((assign16380_e14422 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16380_e14422 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign16380_e14447) + (assign16380_e14426 * (0.5 * (((-((assign16380_e14430 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign16380_e14444) + (assign16380_e14434 * ((-((assign16380_e14437 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16380_e14452;
        var_tmp_dn5 = assign16380_e14452_d_n5;
        var_tmp_dn6 = assign16380_e14452_d_n6;
        var_tmp_dn7 = assign16380_e14452_d_n7;
        var_tmp_dn8 = assign16380_e14452_d_n8;

        let (assign16390_e14472, assign16390_e14472_d_n5, assign16390_e14472_d_n6, assign16390_e14472_d_n7, assign16390_e14472_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard263 == 0.0)) {
        let assign16390_e14465: f64 = (var_v1 * var_fmaxr);
        let assign16390_e14467: f64 = (assign16390_e14465 * var_fmaxr);
        let assign16390_e14469: f64 = (assign16390_e14467 * var_tmp);
        let assign16390_e14470: f64 = (p.p851 * assign16390_e14469);
        (assign16390_e14470, (p.p851 * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign16390_e14465 * var_fmaxr_dn5)) * var_tmp) + (assign16390_e14467 * var_tmp_dn5))), (p.p851 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign16390_e14465 * var_fmaxr_dn6)) * var_tmp) + (assign16390_e14467 * var_tmp_dn6))), (p.p851 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign16390_e14465 * var_fmaxr_dn7)) * var_tmp) + (assign16390_e14467 * var_tmp_dn7))), (p.p851 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign16390_e14465 * var_fmaxr_dn8)) * var_tmp) + (assign16390_e14467 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign16390_e14472;
        var_ibbt_dn5 = assign16390_e14472_d_n5;
        var_ibbt_dn6 = assign16390_e14472_d_n6;
        var_ibbt_dn7 = assign16390_e14472_d_n7;
        var_ibbt_dn8 = assign16390_e14472_d_n8;

        let assign16400_e14475: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        var_guard267 = assign16400_e14475;

        let (assign16410_e14486, assign16410_e14486_d_n5, assign16410_e14486_d_n6, assign16410_e14486_d_n7, assign16410_e14486_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard267 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign16410_e14486;
        var_fbreakdown_dn5 = assign16410_e14486_d_n5;
        var_fbreakdown_dn6 = assign16410_e14486_d_n6;
        var_fbreakdown_dn7 = assign16410_e14486_d_n7;
        var_fbreakdown_dn8 = assign16410_e14486_d_n8;

        let assign16420_e14489: f64 = (-var_alphaav);
        let assign16420_e14491: f64 = (assign16420_e14489 * p.p860);
        let assign16420_e14492: f64 = if var_vav > assign16420_e14491 { 1.0 } else { 0.0 };
        var_guard268 = assign16420_e14492;

        let assign16430_e14495: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        var_guard269 = assign16430_e14495;

        let (assign16440_e14525, assign16440_e14525_d_n5, assign16440_e14525_d_n6, assign16440_e14525_d_n7, assign16440_e14525_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 != 0.0)) && (var_guard269 != 0.0)) {
        let assign16440_e14511: f64 = (var_vav * var_vbrinvbot);
        let assign16440_e14514: f64 = (var_vav * var_vbrinvbot);
        let assign16440_e14515: f64 = (assign16440_e14511 * assign16440_e14514);
        let assign16440_e14518: f64 = (var_vav * var_vbrinvbot);
        let assign16440_e14519: f64 = (assign16440_e14515 * assign16440_e14518);
        let assign16440_e14522: f64 = (var_vav * var_vbrinvbot);
        let assign16440_e14523: f64 = (assign16440_e14519 * assign16440_e14522);
        (assign16440_e14523, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16440_e14525;
        var_tmp_dn5 = assign16440_e14525_d_n5;
        var_tmp_dn6 = assign16440_e14525_d_n6;
        var_tmp_dn7 = assign16440_e14525_d_n7;
        var_tmp_dn8 = assign16440_e14525_d_n8;

        let (assign16450_e14547, assign16450_e14547_d_n5, assign16450_e14547_d_n6, assign16450_e14547_d_n7, assign16450_e14547_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 != 0.0)) && (var_guard269 == 0.0)) {
        let assign16450_e14542: f64 = (var_vav * var_vbrinvbot);
        let assign16450_e14543: f64 = (assign16450_e14542).abs();
        let assign16450_e14545: f64 = (assign16450_e14543).powf(p.p863);
        (assign16450_e14545, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16450_e14547;
        var_tmp_dn5 = assign16450_e14547_d_n5;
        var_tmp_dn6 = assign16450_e14547_d_n6;
        var_tmp_dn7 = assign16450_e14547_d_n7;
        var_tmp_dn8 = assign16450_e14547_d_n8;

        let (assign16460_e14565, assign16460_e14565_d_n5, assign16460_e14565_d_n6, assign16460_e14565_d_n7, assign16460_e14565_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 != 0.0)) {
        let assign16460_e14562: f64 = (1.0 - var_tmp);
        let assign16460_e14563: f64 = (1.0 / assign16460_e14562);
        (assign16460_e14563, (-((-var_tmp_dn5) / (assign16460_e14562 * assign16460_e14562))), (-((-var_tmp_dn6) / (assign16460_e14562 * assign16460_e14562))), (-((-var_tmp_dn7) / (assign16460_e14562 * assign16460_e14562))), (-((-var_tmp_dn8) / (assign16460_e14562 * assign16460_e14562))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign16460_e14565;
        var_fbreakdown_dn5 = assign16460_e14565_d_n5;
        var_fbreakdown_dn6 = assign16460_e14565_d_n6;
        var_fbreakdown_dn7 = assign16460_e14565_d_n7;
        var_fbreakdown_dn8 = assign16460_e14565_d_n8;

        let (assign16470_e14588, assign16470_e14588_d_n5, assign16470_e14588_d_n6, assign16470_e14588_d_n7, assign16470_e14588_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 == 0.0)) {
        let assign16470_e14582: f64 = (var_alphaav * p.p860);
        let assign16470_e14583: f64 = (var_vav + assign16470_e14582);
        let assign16470_e14585: f64 = (assign16470_e14583 * var_slopebot);
        let assign16470_e14586: f64 = (var_fstopbot + assign16470_e14585);
        (assign16470_e14586, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign16470_e14588;
        var_fbreakdown_dn5 = assign16470_e14588_d_n5;
        var_fbreakdown_dn6 = assign16470_e14588_d_n6;
        var_fbreakdown_dn7 = assign16470_e14588_d_n7;
        var_fbreakdown_dn8 = assign16470_e14588_d_n8;

        let (assign16480_e14607, assign16480_e14607_d_n5, assign16480_e14607_d_n6, assign16480_e14607_d_n7, assign16480_e14607_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard253 == 0.0)) {
        let assign16480_e14598: f64 = (var_id__blk219 + var_isrh);
        let assign16480_e14600: f64 = (assign16480_e14598 + var_itat);
        let assign16480_e14602: f64 = (assign16480_e14600 + var_ibbt);
        let assign16480_e14603: f64 = (p.p29 * assign16480_e14602);
        let assign16480_e14605: f64 = (assign16480_e14603 * var_fbreakdown);
        (assign16480_e14605, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign16480_e14603 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign16480_e14603 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign16480_e14603 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign16480_e14603 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign16480_e14607;
        var_ijunbot_dn5 = assign16480_e14607_d_n5;
        var_ijunbot_dn6 = assign16480_e14607_d_n6;
        var_ijunbot_dn7 = assign16480_e14607_d_n7;
        var_ijunbot_dn8 = assign16480_e14607_d_n8;

        let assign16490_e14610: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard270 = assign16490_e14610;

        let (assign16500_e14618, assign16500_e14618_d_n5, assign16500_e14618_d_n6, assign16500_e14618_d_n7, assign16500_e14618_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign16500_e14618;
        var_ijunsti_dn5 = assign16500_e14618_d_n5;
        var_ijunsti_dn6 = assign16500_e14618_d_n6;
        var_ijunsti_dn7 = assign16500_e14618_d_n7;
        var_ijunsti_dn8 = assign16500_e14618_d_n8;

        let (assign16510_e14629,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) {
        let assign16510_e14627: f64 = (var_idsatsti * var_idmult);
        (assign16510_e14627,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign16510_e14629;

        let assign16520_e14636: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        var_guard271 = assign16520_e14636;

        let (assign16530_e14647, assign16530_e14647_d_n5, assign16530_e14647_d_n6, assign16530_e14647_d_n7, assign16530_e14647_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign16530_e14647;
        var_isrh_dn5 = assign16530_e14647_d_n5;
        var_isrh_dn6 = assign16530_e14647_d_n6;
        var_isrh_dn7 = assign16530_e14647_d_n7;
        var_isrh_dn8 = assign16530_e14647_d_n8;

        let (assign16540_e14661,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16540_e14659: f64 = (var_vbisti - var_vjsrh);
        (assign16540_e14659,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign16540_e14661;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard261_slot = var_guard261;
        *var_guard262_slot = var_guard262;
        *var_guard263_slot = var_guard263;
        *var_guard264_slot = var_guard264;
        *var_guard265_slot = var_guard265;
        *var_guard266_slot = var_guard266;
        *var_guard267_slot = var_guard267;
        *var_guard268_slot = var_guard268;
        *var_guard269_slot = var_guard269;
        *var_guard270_slot = var_guard270;
        *var_guard271_slot = var_guard271;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        var_atatsti: f64,
        var_berfc: f64,
        var_btatpartsti: f64,
        var_cerfc: f64,
        var_ftdsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard270: f64,
        var_guard271: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirstiinv: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard274_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard274: f64 = *var_guard274_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign16550_e14680,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16550_e14675: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign16550_e14676: f64 = (1.0 - assign16550_e14675);
        let assign16550_e14677: f64 = (assign16550_e14676).sqrt();
        let assign16550_e14678: f64 = (1.0 - assign16550_e14677);
        (assign16550_e14678,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign16550_e14680;

        let assign16560_e14683: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard272 = assign16560_e14683;

        let (assign16570_e14697,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) && (var_guard272 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign16570_e14697;

        let (assign16580_e14729,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) && (var_guard272 == 0.0)) {
        let assign16580_e14712: f64 = (var_wsrhstep * var_wsrhstep);
        let assign16580_e14714: f64 = (var_wsrhstep).ln();
        let assign16580_e14715: f64 = (assign16580_e14712 * assign16580_e14714);
        let assign16580_e14718: f64 = (1.0 - var_wsrhstep);
        let assign16580_e14719: f64 = (assign16580_e14715 / assign16580_e14718);
        let assign16580_e14721: f64 = (assign16580_e14719 + var_wsrhstep);
        let assign16580_e14725: f64 = (2.0 * p.p832);
        let assign16580_e14726: f64 = (1.0 - assign16580_e14725);
        let assign16580_e14727: f64 = (assign16580_e14721 * assign16580_e14726);
        (assign16580_e14727,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign16580_e14729;

        let (assign16590_e14743,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16590_e14741: f64 = (var_wsrhstep + var_dwsrh);
        (assign16590_e14741,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign16590_e14743;

        let assign16600_e14746: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard273 = assign16600_e14746;

        let (assign16610_e14763, assign16610_e14763_d_n5, assign16610_e14763_d_n6, assign16610_e14763_d_n7, assign16610_e14763_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) && (var_guard273 != 0.0)) {
        let assign16610_e14760: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign16610_e14761: f64 = (assign16610_e14760).sqrt();
        (assign16610_e14761, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16610_e14763;
        var_tmp_dn5 = assign16610_e14763_d_n5;
        var_tmp_dn6 = assign16610_e14763_d_n6;
        var_tmp_dn7 = assign16610_e14763_d_n7;
        var_tmp_dn8 = assign16610_e14763_d_n8;

        let (assign16620_e14782, assign16620_e14782_d_n5, assign16620_e14782_d_n6, assign16620_e14782_d_n7, assign16620_e14782_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) && (var_guard273 == 0.0)) {
        let assign16620_e14778: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign16620_e14780: f64 = (assign16620_e14778).powf(p.p832);
        (assign16620_e14780, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16620_e14782;
        var_tmp_dn5 = assign16620_e14782_d_n5;
        var_tmp_dn6 = assign16620_e14782_d_n6;
        var_tmp_dn7 = assign16620_e14782_d_n7;
        var_tmp_dn8 = assign16620_e14782_d_n8;

        let (assign16630_e14796, assign16630_e14796_d_n5, assign16630_e14796_d_n6, assign16630_e14796_d_n7, assign16630_e14796_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16630_e14794: f64 = (var_wdepnulrsti * var_tmp);
        (assign16630_e14794, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign16630_e14796;
        var_wdep_dn5 = assign16630_e14796_d_n5;
        var_wdep_dn6 = assign16630_e14796_d_n6;
        var_wdep_dn7 = assign16630_e14796_d_n7;
        var_wdep_dn8 = assign16630_e14796_d_n8;

        let (assign16640_e14814, assign16640_e14814_d_n5, assign16640_e14814_d_n6, assign16640_e14814_d_n7, assign16640_e14814_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16640_e14809: f64 = (var_zinv - 1.0);
        let assign16640_e14811: f64 = (assign16640_e14809 * var_wdep);
        let assign16640_e14812: f64 = (var_ftdsti * assign16640_e14811);
        (assign16640_e14812, (var_ftdsti * (assign16640_e14809 * var_wdep_dn5)), (var_ftdsti * (assign16640_e14809 * var_wdep_dn6)), (var_ftdsti * (assign16640_e14809 * var_wdep_dn7)), (var_ftdsti * (assign16640_e14809 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign16640_e14814;
        var_asrh_dn5 = assign16640_e14814_d_n5;
        var_asrh_dn6 = assign16640_e14814_d_n6;
        var_asrh_dn7 = assign16640_e14814_d_n7;
        var_asrh_dn8 = assign16640_e14814_d_n8;

        let (assign16650_e14830, assign16650_e14830_d_n5, assign16650_e14830_d_n6, assign16650_e14830_d_n7, assign16650_e14830_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard271 == 0.0)) {
        let assign16650_e14827: f64 = (var_asrh * var_wsrh);
        let assign16650_e14828: f64 = (p.p841 * assign16650_e14827);
        (assign16650_e14828, (p.p841 * (var_asrh_dn5 * var_wsrh)), (p.p841 * (var_asrh_dn6 * var_wsrh)), (p.p841 * (var_asrh_dn7 * var_wsrh)), (p.p841 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign16650_e14830;
        var_isrh_dn5 = assign16650_e14830_d_n5;
        var_isrh_dn6 = assign16650_e14830_d_n6;
        var_isrh_dn7 = assign16650_e14830_d_n7;
        var_isrh_dn8 = assign16650_e14830_d_n8;

        let assign16660_e14833: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard274 = assign16660_e14833;

        let (assign16670_e14844, assign16670_e14844_d_n5, assign16670_e14844_d_n6, assign16670_e14844_d_n7, assign16670_e14844_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign16670_e14844;
        var_itat_dn5 = assign16670_e14844_d_n5;
        var_itat_dn6 = assign16670_e14844_d_n6;
        var_itat_dn7 = assign16670_e14844_d_n7;
        var_itat_dn8 = assign16670_e14844_d_n8;

        let (assign16680_e14862, assign16680_e14862_d_n5, assign16680_e14862_d_n6, assign16680_e14862_d_n7, assign16680_e14862_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16680_e14857: f64 = (var_wdep * var_one_minus_psti);
        let assign16680_e14859: f64 = (assign16680_e14857 / var_vbi_minus_vjsrh);
        let assign16680_e14860: f64 = (var_btatpartsti * assign16680_e14859);
        (assign16680_e14860, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign16680_e14862;
        var_btat_dn5 = assign16680_e14862_d_n5;
        var_btat_dn6 = assign16680_e14862_d_n6;
        var_btat_dn7 = assign16680_e14862_d_n7;
        var_btat_dn8 = assign16680_e14862_d_n8;

        let (assign16690_e14878, assign16690_e14878_d_n5, assign16690_e14878_d_n6, assign16690_e14878_d_n7, assign16690_e14878_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16690_e14874: f64 = (0.666666666666667 * var_atatsti);
        let assign16690_e14876: f64 = (assign16690_e14874 / var_btat);
        (assign16690_e14876, (-((assign16690_e14874 * var_btat_dn5) / (var_btat * var_btat))), (-((assign16690_e14874 * var_btat_dn6) / (var_btat * var_btat))), (-((assign16690_e14874 * var_btat_dn7) / (var_btat * var_btat))), (-((assign16690_e14874 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign16690_e14878;
        var_twoatatoverthreebtat_dn5 = assign16690_e14878_d_n5;
        var_twoatatoverthreebtat_dn6 = assign16690_e14878_d_n6;
        var_twoatatoverthreebtat_dn7 = assign16690_e14878_d_n7;
        var_twoatatoverthreebtat_dn8 = assign16690_e14878_d_n8;

        let (assign16700_e14892, assign16700_e14892_d_n5, assign16700_e14892_d_n6, assign16700_e14892_d_n7, assign16700_e14892_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16700_e14890: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign16700_e14890, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign16700_e14892;
        var_umaxbeforelimiting_dn5 = assign16700_e14892_d_n5;
        var_umaxbeforelimiting_dn6 = assign16700_e14892_d_n6;
        var_umaxbeforelimiting_dn7 = assign16700_e14892_d_n7;
        var_umaxbeforelimiting_dn8 = assign16700_e14892_d_n8;

        let (assign16710_e14913, assign16710_e14913_d_n5, assign16710_e14913_d_n6, assign16710_e14913_d_n7, assign16710_e14913_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16710_e14904: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16710_e14907: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16710_e14909: f64 = (assign16710_e14907 + 1.0);
        let assign16710_e14910: f64 = (assign16710_e14904 / assign16710_e14909);
        let assign16710_e14911: f64 = (assign16710_e14910).sqrt();
        (assign16710_e14911, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign16710_e14909) - (assign16710_e14904 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign16710_e14909) - (assign16710_e14904 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign16710_e14909) - (assign16710_e14904 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign16710_e14909) - (assign16710_e14904 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign16710_e14909 * assign16710_e14909)) / (2.0 * assign16710_e14911)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign16710_e14913;
        var_umax_dn5 = assign16710_e14913_d_n5;
        var_umax_dn6 = assign16710_e14913_d_n6;
        var_umax_dn7 = assign16710_e14913_d_n7;
        var_umax_dn8 = assign16710_e14913_d_n8;

        let (assign16720_e14926, assign16720_e14926_d_n5, assign16720_e14926_d_n6, assign16720_e14926_d_n7, assign16720_e14926_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16720_e14924: f64 = (var_umax).sqrt();
        (assign16720_e14924, (var_umax_dn5 / (2.0 * assign16720_e14924)), (var_umax_dn6 / (2.0 * assign16720_e14924)), (var_umax_dn7 / (2.0 * assign16720_e14924)), (var_umax_dn8 / (2.0 * assign16720_e14924)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign16720_e14926;
        var_sqrtumax_dn5 = assign16720_e14926_d_n5;
        var_sqrtumax_dn6 = assign16720_e14926_d_n6;
        var_sqrtumax_dn7 = assign16720_e14926_d_n7;
        var_sqrtumax_dn8 = assign16720_e14926_d_n8;

        let (assign16730_e14940, assign16730_e14940_d_n5, assign16730_e14940_d_n6, assign16730_e14940_d_n7, assign16730_e14940_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16730_e14938: f64 = (var_umax * var_sqrtumax);
        (assign16730_e14938, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign16730_e14940;
        var_umaxpoweronepointfive_dn5 = assign16730_e14940_d_n5;
        var_umaxpoweronepointfive_dn6 = assign16730_e14940_d_n6;
        var_umaxpoweronepointfive_dn7 = assign16730_e14940_d_n7;
        var_umaxpoweronepointfive_dn8 = assign16730_e14940_d_n8;

        let assign16740_e14942: f64 = (-p.p832);
        let assign16740_e14944: f64 = (assign16740_e14942 * var_one_over_one_minus_psti);
        let assign16740_e14946: f64 = (-1.0);
        let assign16740_e14947: f64 = if assign16740_e14944 == assign16740_e14946 { 1.0 } else { 0.0 };
        var_guard275 = assign16740_e14947;

        let (assign16750_e14967, assign16750_e14967_d_n5, assign16750_e14967_d_n6, assign16750_e14967_d_n7, assign16750_e14967_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard275 != 0.0)) {
        let assign16750_e14963: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16750_e14964: f64 = (1.0 + assign16750_e14963);
        let assign16750_e14965: f64 = (1.0 / assign16750_e14964);
        (assign16750_e14965, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign16750_e14964 * assign16750_e14964))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign16750_e14964 * assign16750_e14964))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign16750_e14964 * assign16750_e14964))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign16750_e14964 * assign16750_e14964))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign16750_e14967;
        var_wgamma_dn5 = assign16750_e14967_d_n5;
        var_wgamma_dn6 = assign16750_e14967_d_n6;
        var_wgamma_dn7 = assign16750_e14967_d_n7;
        var_wgamma_dn8 = assign16750_e14967_d_n8;

        let (assign16760_e14991, assign16760_e14991_d_n5, assign16760_e14991_d_n6, assign16760_e14991_d_n7, assign16760_e14991_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard275 == 0.0)) {
        let assign16760_e14983: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16760_e14984: f64 = (1.0 + assign16760_e14983);
        let assign16760_e14986: f64 = (-p.p832);
        let assign16760_e14988: f64 = (assign16760_e14986 * var_one_over_one_minus_psti);
        let assign16760_e14989: f64 = (assign16760_e14984).powf(assign16760_e14988);
        (assign16760_e14989, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign16760_e14984))) }, if 0.0 == 0.0 && ((assign16760_e14988) as f64).is_finite() && ((assign16760_e14988) as f64).fract() == 0.0 { if assign16760_e14988 == 0.0 { 0.0 } else { (assign16760_e14988 * ((assign16760_e14984).powf(assign16760_e14988 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign16760_e14989 * (assign16760_e14988 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign16760_e14984))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign16760_e14991;
        var_wgamma_dn5 = assign16760_e14991_d_n5;
        var_wgamma_dn6 = assign16760_e14991_d_n6;
        var_wgamma_dn7 = assign16760_e14991_d_n7;
        var_wgamma_dn8 = assign16760_e14991_d_n8;

        let (assign16770_e15009, assign16770_e15009_d_n5, assign16770_e15009_d_n6, assign16770_e15009_d_n7, assign16770_e15009_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16770_e15003: f64 = (var_wsrh * var_wgamma);
        let assign16770_e15006: f64 = (var_wsrh + var_wgamma);
        let assign16770_e15007: f64 = (assign16770_e15003 / assign16770_e15006);
        (assign16770_e15007, ((((var_wsrh * var_wgamma_dn5) * assign16770_e15006) - (assign16770_e15003 * var_wgamma_dn5)) / (assign16770_e15006 * assign16770_e15006)), ((((var_wsrh * var_wgamma_dn6) * assign16770_e15006) - (assign16770_e15003 * var_wgamma_dn6)) / (assign16770_e15006 * assign16770_e15006)), ((((var_wsrh * var_wgamma_dn7) * assign16770_e15006) - (assign16770_e15003 * var_wgamma_dn7)) / (assign16770_e15006 * assign16770_e15006)), ((((var_wsrh * var_wgamma_dn8) * assign16770_e15006) - (assign16770_e15003 * var_wgamma_dn8)) / (assign16770_e15006 * assign16770_e15006)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign16770_e15009;
        var_wtat_dn5 = assign16770_e15009_d_n5;
        var_wtat_dn6 = assign16770_e15009_d_n6;
        var_wtat_dn7 = assign16770_e15009_d_n7;
        var_wtat_dn8 = assign16770_e15009_d_n8;

        let (assign16780_e15026, assign16780_e15026_d_n5, assign16780_e15026_d_n6, assign16780_e15026_d_n7, assign16780_e15026_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16780_e15022: f64 = (var_btat / var_sqrtumax);
        let assign16780_e15023: f64 = (0.375 * assign16780_e15022);
        let assign16780_e15024: f64 = (assign16780_e15023).sqrt();
        (assign16780_e15024, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16780_e15024)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16780_e15024)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign16780_e15026;
        var_ktat_dn5 = assign16780_e15026_d_n5;
        var_ktat_dn6 = assign16780_e15026_d_n6;
        var_ktat_dn7 = assign16780_e15026_d_n7;
        var_ktat_dn8 = assign16780_e15026_d_n8;

        let (assign16790_e15044, assign16790_e15044_d_n5, assign16790_e15044_d_n6, assign16790_e15044_d_n7, assign16790_e15044_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16790_e15039: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign16790_e15040: f64 = (2.0 * assign16790_e15039);
        let assign16790_e15042: f64 = (assign16790_e15040 - var_umax);
        (assign16790_e15042, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign16790_e15044;
        var_ltat_dn5 = assign16790_e15044_d_n5;
        var_ltat_dn6 = assign16790_e15044_d_n6;
        var_ltat_dn7 = assign16790_e15044_d_n7;
        var_ltat_dn8 = assign16790_e15044_d_n8;

        let (assign16800_e15070, assign16800_e15070_d_n5, assign16800_e15070_d_n6, assign16800_e15070_d_n7, assign16800_e15070_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16800_e15056: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign16800_e15058: f64 = (assign16800_e15056 * var_sqrtumax);
        let assign16800_e15061: f64 = (var_atatsti * var_umax);
        let assign16800_e15062: f64 = (assign16800_e15058 - assign16800_e15061);
        let assign16800_e15066: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16800_e15067: f64 = (0.5 * assign16800_e15066);
        let assign16800_e15068: f64 = (assign16800_e15062 + assign16800_e15067);
        (assign16800_e15068, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign16800_e15056 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign16800_e15056 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign16800_e15056 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign16800_e15056 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign16800_e15070;
        var_mtat_dn5 = assign16800_e15070_d_n5;
        var_mtat_dn6 = assign16800_e15070_d_n6;
        var_mtat_dn7 = assign16800_e15070_d_n7;
        var_mtat_dn8 = assign16800_e15070_d_n8;

        let (assign16810_e15086, assign16810_e15086_d_n5, assign16810_e15086_d_n6, assign16810_e15086_d_n7, assign16810_e15086_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16810_e15082: f64 = (var_ltat - 1.0);
        let assign16810_e15084: f64 = (assign16810_e15082 * var_ktat);
        (assign16810_e15084, ((var_ltat_dn5 * var_ktat) + (assign16810_e15082 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign16810_e15082 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign16810_e15082 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign16810_e15082 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign16810_e15086;
        var_xerfc_dn5 = assign16810_e15086_d_n5;
        var_xerfc_dn6 = assign16810_e15086_d_n6;
        var_xerfc_dn7 = assign16810_e15086_d_n7;
        var_xerfc_dn8 = assign16810_e15086_d_n8;

        let (assign16820_e15100, assign16820_e15100_d_n5, assign16820_e15100_d_n6, assign16820_e15100_d_n7, assign16820_e15100_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16820_e15098: f64 = (var_xerfc * var_xerfc);
        (assign16820_e15098, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign16820_e15100;
        var_ysq_dn5 = assign16820_e15100_d_n5;
        var_ysq_dn6 = assign16820_e15100_d_n6;
        var_ysq_dn7 = assign16820_e15100_d_n7;
        var_ysq_dn8 = assign16820_e15100_d_n8;

        let assign16830_e15103: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard276 = assign16830_e15103;

        let (assign16840_e15123, assign16840_e15123_d_n5, assign16840_e15123_d_n6, assign16840_e15123_d_n7, assign16840_e15123_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard276 != 0.0)) {
        let assign16840_e15119: f64 = (var_perfc * var_xerfc);
        let assign16840_e15120: f64 = (1.0 + assign16840_e15119);
        let assign16840_e15121: f64 = (1.0 / assign16840_e15120);
        (assign16840_e15121, (-((var_perfc * var_xerfc_dn5) / (assign16840_e15120 * assign16840_e15120))), (-((var_perfc * var_xerfc_dn6) / (assign16840_e15120 * assign16840_e15120))), (-((var_perfc * var_xerfc_dn7) / (assign16840_e15120 * assign16840_e15120))), (-((var_perfc * var_xerfc_dn8) / (assign16840_e15120 * assign16840_e15120))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign16840_e15123;
        var_terfc_dn5 = assign16840_e15123_d_n5;
        var_terfc_dn6 = assign16840_e15123_d_n6;
        var_terfc_dn7 = assign16840_e15123_d_n7;
        var_terfc_dn8 = assign16840_e15123_d_n8;

        let (assign16850_e15144, assign16850_e15144_d_n5, assign16850_e15144_d_n6, assign16850_e15144_d_n7, assign16850_e15144_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard276 == 0.0)) {
        let assign16850_e15140: f64 = (var_perfc * var_xerfc);
        let assign16850_e15141: f64 = (1.0 - assign16850_e15140);
        let assign16850_e15142: f64 = (1.0 / assign16850_e15141);
        (assign16850_e15142, (-((-(var_perfc * var_xerfc_dn5)) / (assign16850_e15141 * assign16850_e15141))), (-((-(var_perfc * var_xerfc_dn6)) / (assign16850_e15141 * assign16850_e15141))), (-((-(var_perfc * var_xerfc_dn7)) / (assign16850_e15141 * assign16850_e15141))), (-((-(var_perfc * var_xerfc_dn8)) / (assign16850_e15141 * assign16850_e15141))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign16850_e15144;
        var_terfc_dn5 = assign16850_e15144_d_n5;
        var_terfc_dn6 = assign16850_e15144_d_n6;
        var_terfc_dn7 = assign16850_e15144_d_n7;
        var_terfc_dn8 = assign16850_e15144_d_n8;

        let assign16860_e15146: f64 = (-var_ysq);
        let assign16860_e15148: f64 = (assign16860_e15146 + var_mtat);
        let assign16860_e15150: f64 = (-230.25850929940458);
        let assign16860_e15151: f64 = if assign16860_e15148 > assign16860_e15150 { 1.0 } else { 0.0 };
        var_guard277 = assign16860_e15151;

        let (assign16870_e15169, assign16870_e15169_d_n5, assign16870_e15169_d_n6, assign16870_e15169_d_n7, assign16870_e15169_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard277 != 0.0)) {
        let assign16870_e15164: f64 = (-var_ysq);
        let assign16870_e15166: f64 = (assign16870_e15164 + var_mtat);
        let assign16870_e15167: f64 = (assign16870_e15166).exp();
        (assign16870_e15167, (assign16870_e15167 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign16870_e15167 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign16870_e15167 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign16870_e15167 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16870_e15169;
        var_tmp_dn5 = assign16870_e15169_d_n5;
        var_tmp_dn6 = assign16870_e15169_d_n6;
        var_tmp_dn7 = assign16870_e15169_d_n7;
        var_tmp_dn8 = assign16870_e15169_d_n8;

        let (assign16880_e15218, assign16880_e15218_d_n5, assign16880_e15218_d_n6, assign16880_e15218_d_n7, assign16880_e15218_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard277 == 0.0)) {
        let assign16880_e15185: f64 = (-230.25850929940458);
        let assign16880_e15187: f64 = (-var_ysq);
        let assign16880_e15189: f64 = (assign16880_e15187 + var_mtat);
        let assign16880_e15190: f64 = (assign16880_e15185 - assign16880_e15189);
        let assign16880_e15194: f64 = (-230.25850929940458);
        let assign16880_e15196: f64 = (-var_ysq);
        let assign16880_e15198: f64 = (assign16880_e15196 + var_mtat);
        let assign16880_e15199: f64 = (assign16880_e15194 - assign16880_e15198);
        let assign16880_e15202: f64 = (-230.25850929940458);
        let assign16880_e15204: f64 = (-var_ysq);
        let assign16880_e15206: f64 = (assign16880_e15204 + var_mtat);
        let assign16880_e15207: f64 = (assign16880_e15202 - assign16880_e15206);
        let assign16880_e15209: f64 = (assign16880_e15207 * 0.3333333333333333);
        let assign16880_e15210: f64 = (1.0 + assign16880_e15209);
        let assign16880_e15211: f64 = (assign16880_e15199 * assign16880_e15210);
        let assign16880_e15212: f64 = (0.5 * assign16880_e15211);
        let assign16880_e15213: f64 = (1.0 + assign16880_e15212);
        let assign16880_e15214: f64 = (assign16880_e15190 * assign16880_e15213);
        let assign16880_e15215: f64 = (1.0 + assign16880_e15214);
        let assign16880_e15216: f64 = (1e-100 / assign16880_e15215);
        (assign16880_e15216, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign16880_e15210) + (assign16880_e15199 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16880_e15210) + (assign16880_e15199 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16880_e15210) + (assign16880_e15199 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16880_e15213) + (assign16880_e15190 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16880_e15210) + (assign16880_e15199 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign16880_e15215 * assign16880_e15215))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16880_e15218;
        var_tmp_dn5 = assign16880_e15218_d_n5;
        var_tmp_dn6 = assign16880_e15218_d_n6;
        var_tmp_dn7 = assign16880_e15218_d_n7;
        var_tmp_dn8 = assign16880_e15218_d_n8;

        let (assign16890_e15248, assign16890_e15248_d_n5, assign16890_e15248_d_n6, assign16890_e15248_d_n7, assign16890_e15248_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16890_e15230: f64 = (0.29214664 * var_terfc);
        let assign16890_e15234: f64 = (var_terfc * var_terfc);
        let assign16890_e15235: f64 = (var_berfc * assign16890_e15234);
        let assign16890_e15236: f64 = (assign16890_e15230 + assign16890_e15235);
        let assign16890_e15240: f64 = (var_terfc * var_terfc);
        let assign16890_e15242: f64 = (assign16890_e15240 * var_terfc);
        let assign16890_e15243: f64 = (var_cerfc * assign16890_e15242);
        let assign16890_e15244: f64 = (assign16890_e15236 + assign16890_e15243);
        let assign16890_e15246: f64 = (assign16890_e15244 * var_tmp);
        (assign16890_e15246, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign16890_e15240 * var_terfc_dn5)))) * var_tmp) + (assign16890_e15244 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign16890_e15240 * var_terfc_dn6)))) * var_tmp) + (assign16890_e15244 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign16890_e15240 * var_terfc_dn7)))) * var_tmp) + (assign16890_e15244 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign16890_e15240 * var_terfc_dn8)))) * var_tmp) + (assign16890_e15244 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign16890_e15248;
        var_erfcpos_dn5 = assign16890_e15248_d_n5;
        var_erfcpos_dn6 = assign16890_e15248_d_n6;
        var_erfcpos_dn7 = assign16890_e15248_d_n7;
        var_erfcpos_dn8 = assign16890_e15248_d_n8;

        let assign16900_e15251: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard278 = assign16900_e15251;

        let (assign16910_e15265, assign16910_e15265_d_n5, assign16910_e15265_d_n6, assign16910_e15265_d_n7, assign16910_e15265_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard278 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign16910_e15265;
        var_erfctimesexpmtat_dn5 = assign16910_e15265_d_n5;
        var_erfctimesexpmtat_dn6 = assign16910_e15265_d_n6;
        var_erfctimesexpmtat_dn7 = assign16910_e15265_d_n7;
        var_erfctimesexpmtat_dn8 = assign16910_e15265_d_n8;

        let assign16920_e15268: f64 = (-230.25850929940458);
        let assign16920_e15269: f64 = if var_mtat > assign16920_e15268 { 1.0 } else { 0.0 };
        var_guard279 = assign16920_e15269;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_guard272_slot = var_guard272;
        *var_guard273_slot = var_guard273;
        *var_guard274_slot = var_guard274;
        *var_guard275_slot = var_guard275;
        *var_guard276_slot = var_guard276;
        *var_guard277_slot = var_guard277;
        *var_guard278_slot = var_guard278;
        *var_guard279_slot = var_guard279;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_erfcpos: f64,
        var_erfcpos_dn5: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard270: f64,
        var_guard274: f64,
        var_guard278: f64,
        var_guard279: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lgsource_i: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrinvsti: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_dwsrh_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard281_slot: &mut f64,
        var_guard282_slot: &mut f64,
        var_guard283_slot: &mut f64,
        var_guard284_slot: &mut f64,
        var_guard285_slot: &mut f64,
        var_guard286_slot: &mut f64,
        var_guard287_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard281: f64 = *var_guard281_slot;
        let mut var_guard282: f64 = *var_guard282_slot;
        let mut var_guard283: f64 = *var_guard283_slot;
        let mut var_guard284: f64 = *var_guard284_slot;
        let mut var_guard285: f64 = *var_guard285_slot;
        let mut var_guard286: f64 = *var_guard286_slot;
        let mut var_guard287: f64 = *var_guard287_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign16930_e15287, assign16930_e15287_d_n5, assign16930_e15287_d_n6, assign16930_e15287_d_n7, assign16930_e15287_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard278 == 0.0)) && (var_guard279 != 0.0)) {
        let assign16930_e15285: f64 = (var_mtat).exp();
        (assign16930_e15285, (assign16930_e15285 * var_mtat_dn5), (assign16930_e15285 * var_mtat_dn6), (assign16930_e15285 * var_mtat_dn7), (assign16930_e15285 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16930_e15287;
        var_tmp_dn5 = assign16930_e15287_d_n5;
        var_tmp_dn6 = assign16930_e15287_d_n6;
        var_tmp_dn7 = assign16930_e15287_d_n7;
        var_tmp_dn8 = assign16930_e15287_d_n8;

        let (assign16940_e15330, assign16940_e15330_d_n5, assign16940_e15330_d_n6, assign16940_e15330_d_n7, assign16940_e15330_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard278 == 0.0)) && (var_guard279 == 0.0)) {
        let assign16940_e15306: f64 = (-230.25850929940458);
        let assign16940_e15308: f64 = (assign16940_e15306 - var_mtat);
        let assign16940_e15312: f64 = (-230.25850929940458);
        let assign16940_e15314: f64 = (assign16940_e15312 - var_mtat);
        let assign16940_e15317: f64 = (-230.25850929940458);
        let assign16940_e15319: f64 = (assign16940_e15317 - var_mtat);
        let assign16940_e15321: f64 = (assign16940_e15319 * 0.3333333333333333);
        let assign16940_e15322: f64 = (1.0 + assign16940_e15321);
        let assign16940_e15323: f64 = (assign16940_e15314 * assign16940_e15322);
        let assign16940_e15324: f64 = (0.5 * assign16940_e15323);
        let assign16940_e15325: f64 = (1.0 + assign16940_e15324);
        let assign16940_e15326: f64 = (assign16940_e15308 * assign16940_e15325);
        let assign16940_e15327: f64 = (1.0 + assign16940_e15326);
        let assign16940_e15328: f64 = (1e-100 / assign16940_e15327);
        (assign16940_e15328, (-((1e-100 * (((-var_mtat_dn5) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-var_mtat_dn5) * assign16940_e15322) + (assign16940_e15314 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-var_mtat_dn6) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-var_mtat_dn6) * assign16940_e15322) + (assign16940_e15314 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-var_mtat_dn7) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-var_mtat_dn7) * assign16940_e15322) + (assign16940_e15314 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))), (-((1e-100 * (((-var_mtat_dn8) * assign16940_e15325) + (assign16940_e15308 * (0.5 * (((-var_mtat_dn8) * assign16940_e15322) + (assign16940_e15314 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign16940_e15327 * assign16940_e15327))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign16940_e15330;
        var_tmp_dn5 = assign16940_e15330_d_n5;
        var_tmp_dn6 = assign16940_e15330_d_n6;
        var_tmp_dn7 = assign16940_e15330_d_n7;
        var_tmp_dn8 = assign16940_e15330_d_n8;

        let (assign16950_e15349, assign16950_e15349_d_n5, assign16950_e15349_d_n6, assign16950_e15349_d_n7, assign16950_e15349_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) && (var_guard278 == 0.0)) {
        let assign16950_e15345: f64 = (2.0 * var_tmp);
        let assign16950_e15347: f64 = (assign16950_e15345 - var_erfcpos);
        (assign16950_e15347, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign16950_e15349;
        var_erfctimesexpmtat_dn5 = assign16950_e15349_d_n5;
        var_erfctimesexpmtat_dn6 = assign16950_e15349_d_n6;
        var_erfctimesexpmtat_dn7 = assign16950_e15349_d_n7;
        var_erfctimesexpmtat_dn8 = assign16950_e15349_d_n8;

        let (assign16960_e15369, assign16960_e15369_d_n5, assign16960_e15369_d_n6, assign16960_e15369_d_n7, assign16960_e15369_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16960_e15361: f64 = (1.772453850905516 * 0.5);
        let assign16960_e15364: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign16960_e15366: f64 = (assign16960_e15364 / var_ktat);
        let assign16960_e15367: f64 = (assign16960_e15361 * assign16960_e15366);
        (assign16960_e15367, (assign16960_e15361 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign16960_e15364 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign16960_e15361 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign16960_e15364 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign16960_e15361 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign16960_e15364 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign16960_e15361 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign16960_e15364 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign16960_e15369;
        var_gammamax_dn5 = assign16960_e15369_d_n5;
        var_gammamax_dn6 = assign16960_e15369_d_n6;
        var_gammamax_dn7 = assign16960_e15369_d_n7;
        var_gammamax_dn8 = assign16960_e15369_d_n8;

        let (assign16970_e15387, assign16970_e15387_d_n5, assign16970_e15387_d_n6, assign16970_e15387_d_n7, assign16970_e15387_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard274 == 0.0)) {
        let assign16970_e15382: f64 = (var_asrh * var_gammamax);
        let assign16970_e15384: f64 = (assign16970_e15382 * var_wtat);
        let assign16970_e15385: f64 = (p.p846 * assign16970_e15384);
        (assign16970_e15385, (p.p846 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign16970_e15382 * var_wtat_dn5))), (p.p846 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign16970_e15382 * var_wtat_dn6))), (p.p846 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign16970_e15382 * var_wtat_dn7))), (p.p846 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign16970_e15382 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign16970_e15387;
        var_itat_dn5 = assign16970_e15387_d_n5;
        var_itat_dn6 = assign16970_e15387_d_n6;
        var_itat_dn7 = assign16970_e15387_d_n7;
        var_itat_dn8 = assign16970_e15387_d_n8;

        let assign16980_e15390: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        var_guard280 = assign16980_e15390;

        let (assign16990_e15401, assign16990_e15401_d_n5, assign16990_e15401_d_n6, assign16990_e15401_d_n7, assign16990_e15401_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign16990_e15401;
        var_ibbt_dn5 = assign16990_e15401_d_n5;
        var_ibbt_dn6 = assign16990_e15401_d_n6;
        var_ibbt_dn7 = assign16990_e15401_d_n7;
        var_ibbt_dn8 = assign16990_e15401_d_n8;

        let assign17000_e15404: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard281 = assign17000_e15404;

        let (assign17010_e15423, assign17010_e15423_d_n5, assign17010_e15423_d_n6, assign17010_e15423_d_n7, assign17010_e15423_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) && (var_guard281 != 0.0)) {
        let assign17010_e15418: f64 = (p.p829 - var_vbbt);
        let assign17010_e15420: f64 = (assign17010_e15418 * var_vbirstiinv);
        let assign17010_e15421: f64 = (assign17010_e15420).sqrt();
        (assign17010_e15421, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17010_e15423;
        var_tmp_dn5 = assign17010_e15423_d_n5;
        var_tmp_dn6 = assign17010_e15423_d_n6;
        var_tmp_dn7 = assign17010_e15423_d_n7;
        var_tmp_dn8 = assign17010_e15423_d_n8;

        let (assign17020_e15444, assign17020_e15444_d_n5, assign17020_e15444_d_n6, assign17020_e15444_d_n7, assign17020_e15444_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign17020_e15438: f64 = (p.p829 - var_vbbt);
        let assign17020_e15440: f64 = (assign17020_e15438 * var_vbirstiinv);
        let assign17020_e15442: f64 = (assign17020_e15440).powf(p.p832);
        (assign17020_e15442, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17020_e15444;
        var_tmp_dn5 = assign17020_e15444_d_n5;
        var_tmp_dn6 = assign17020_e15444_d_n6;
        var_tmp_dn7 = assign17020_e15444_d_n7;
        var_tmp_dn8 = assign17020_e15444_d_n8;

        let (assign17030_e15464, assign17030_e15464_d_n5, assign17030_e15464_d_n6, assign17030_e15464_d_n7, assign17030_e15464_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) {
        let assign17030_e15457: f64 = (p.p829 - var_vbbt);
        let assign17030_e15459: f64 = (assign17030_e15457 * var_wdepnulrinvsti);
        let assign17030_e15461: f64 = (assign17030_e15459 / var_tmp);
        let assign17030_e15462: f64 = (var_one_over_one_minus_psti * assign17030_e15461);
        (assign17030_e15462, (var_one_over_one_minus_psti * (-((assign17030_e15459 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign17030_e15459 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign17030_e15459 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign17030_e15459 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign17030_e15464;
        var_fmaxr_dn5 = assign17030_e15464_d_n5;
        var_fmaxr_dn6 = assign17030_e15464_d_n6;
        var_fmaxr_dn7 = assign17030_e15464_d_n7;
        var_fmaxr_dn8 = assign17030_e15464_d_n8;

        let assign17040_e15466: f64 = (-var_fbbtsti);
        let assign17040_e15468: f64 = (assign17040_e15466 / var_fmaxr);
        let assign17040_e15469: f64 = (assign17040_e15468).abs();
        let assign17040_e15471: f64 = if assign17040_e15469 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard282 = assign17040_e15471;

        let (assign17050_e15489, assign17050_e15489_d_n5, assign17050_e15489_d_n6, assign17050_e15489_d_n7, assign17050_e15489_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) && (var_guard282 != 0.0)) {
        let assign17050_e15484: f64 = (-var_fbbtsti);
        let assign17050_e15486: f64 = (assign17050_e15484 / var_fmaxr);
        let assign17050_e15487: f64 = (assign17050_e15486).exp();
        (assign17050_e15487, (assign17050_e15487 * (-((assign17050_e15484 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign17050_e15487 * (-((assign17050_e15484 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17050_e15489;
        var_tmp_dn5 = assign17050_e15489_d_n5;
        var_tmp_dn6 = assign17050_e15489_d_n6;
        var_tmp_dn7 = assign17050_e15489_d_n7;
        var_tmp_dn8 = assign17050_e15489_d_n8;

        let assign17060_e15491: f64 = (-var_fbbtsti);
        let assign17060_e15493: f64 = (assign17060_e15491 / var_fmaxr);
        let assign17060_e15495: f64 = if assign17060_e15493 < 0.0 { 1.0 } else { 0.0 };
        var_guard283 = assign17060_e15495;

        let (assign17070_e15546, assign17070_e15546_d_n5, assign17070_e15546_d_n6, assign17070_e15546_d_n7, assign17070_e15546_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) && (var_guard282 == 0.0)) && (var_guard283 != 0.0)) {
        let assign17070_e15513: f64 = (-230.25850929940458);
        let assign17070_e15515: f64 = (-var_fbbtsti);
        let assign17070_e15517: f64 = (assign17070_e15515 / var_fmaxr);
        let assign17070_e15518: f64 = (assign17070_e15513 - assign17070_e15517);
        let assign17070_e15522: f64 = (-230.25850929940458);
        let assign17070_e15524: f64 = (-var_fbbtsti);
        let assign17070_e15526: f64 = (assign17070_e15524 / var_fmaxr);
        let assign17070_e15527: f64 = (assign17070_e15522 - assign17070_e15526);
        let assign17070_e15530: f64 = (-230.25850929940458);
        let assign17070_e15532: f64 = (-var_fbbtsti);
        let assign17070_e15534: f64 = (assign17070_e15532 / var_fmaxr);
        let assign17070_e15535: f64 = (assign17070_e15530 - assign17070_e15534);
        let assign17070_e15537: f64 = (assign17070_e15535 * 0.3333333333333333);
        let assign17070_e15538: f64 = (1.0 + assign17070_e15537);
        let assign17070_e15539: f64 = (assign17070_e15527 * assign17070_e15538);
        let assign17070_e15540: f64 = (0.5 * assign17070_e15539);
        let assign17070_e15541: f64 = (1.0 + assign17070_e15540);
        let assign17070_e15542: f64 = (assign17070_e15518 * assign17070_e15541);
        let assign17070_e15543: f64 = (1.0 + assign17070_e15542);
        let assign17070_e15544: f64 = (1e-100 / assign17070_e15543);
        (assign17070_e15544, (-((1e-100 * (((-(-((assign17070_e15515 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))), (-((1e-100 * (((-(-((assign17070_e15515 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign17070_e15541) + (assign17070_e15518 * (0.5 * (((-(-((assign17070_e15524 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign17070_e15538) + (assign17070_e15527 * ((-(-((assign17070_e15532 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17070_e15543 * assign17070_e15543))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17070_e15546;
        var_tmp_dn5 = assign17070_e15546_d_n5;
        var_tmp_dn6 = assign17070_e15546_d_n6;
        var_tmp_dn7 = assign17070_e15546_d_n7;
        var_tmp_dn8 = assign17070_e15546_d_n8;

        let (assign17080_e15595, assign17080_e15595_d_n5, assign17080_e15595_d_n6, assign17080_e15595_d_n7, assign17080_e15595_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) && (var_guard282 == 0.0)) && (var_guard283 == 0.0)) {
        let assign17080_e15565: f64 = (-var_fbbtsti);
        let assign17080_e15567: f64 = (assign17080_e15565 / var_fmaxr);
        let assign17080_e15569: f64 = (assign17080_e15567 - 230.25850929940458);
        let assign17080_e15573: f64 = (-var_fbbtsti);
        let assign17080_e15575: f64 = (assign17080_e15573 / var_fmaxr);
        let assign17080_e15577: f64 = (assign17080_e15575 - 230.25850929940458);
        let assign17080_e15580: f64 = (-var_fbbtsti);
        let assign17080_e15582: f64 = (assign17080_e15580 / var_fmaxr);
        let assign17080_e15584: f64 = (assign17080_e15582 - 230.25850929940458);
        let assign17080_e15586: f64 = (assign17080_e15584 * 0.3333333333333333);
        let assign17080_e15587: f64 = (1.0 + assign17080_e15586);
        let assign17080_e15588: f64 = (assign17080_e15577 * assign17080_e15587);
        let assign17080_e15589: f64 = (0.5 * assign17080_e15588);
        let assign17080_e15590: f64 = (1.0 + assign17080_e15589);
        let assign17080_e15591: f64 = (assign17080_e15569 * assign17080_e15590);
        let assign17080_e15592: f64 = (1.0 + assign17080_e15591);
        let assign17080_e15593: f64 = (1e100 * assign17080_e15592);
        (assign17080_e15593, (1e100 * (((-((assign17080_e15565 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17080_e15565 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign17080_e15590) + (assign17080_e15569 * (0.5 * (((-((assign17080_e15573 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign17080_e15587) + (assign17080_e15577 * ((-((assign17080_e15580 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17080_e15595;
        var_tmp_dn5 = assign17080_e15595_d_n5;
        var_tmp_dn6 = assign17080_e15595_d_n6;
        var_tmp_dn7 = assign17080_e15595_d_n7;
        var_tmp_dn8 = assign17080_e15595_d_n8;

        let (assign17090_e15615, assign17090_e15615_d_n5, assign17090_e15615_d_n6, assign17090_e15615_d_n7, assign17090_e15615_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard280 == 0.0)) {
        let assign17090_e15608: f64 = (var_v1 * var_fmaxr);
        let assign17090_e15610: f64 = (assign17090_e15608 * var_fmaxr);
        let assign17090_e15612: f64 = (assign17090_e15610 * var_tmp);
        let assign17090_e15613: f64 = (p.p852 * assign17090_e15612);
        (assign17090_e15613, (p.p852 * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign17090_e15608 * var_fmaxr_dn5)) * var_tmp) + (assign17090_e15610 * var_tmp_dn5))), (p.p852 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign17090_e15608 * var_fmaxr_dn6)) * var_tmp) + (assign17090_e15610 * var_tmp_dn6))), (p.p852 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign17090_e15608 * var_fmaxr_dn7)) * var_tmp) + (assign17090_e15610 * var_tmp_dn7))), (p.p852 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign17090_e15608 * var_fmaxr_dn8)) * var_tmp) + (assign17090_e15610 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign17090_e15615;
        var_ibbt_dn5 = assign17090_e15615_d_n5;
        var_ibbt_dn6 = assign17090_e15615_d_n6;
        var_ibbt_dn7 = assign17090_e15615_d_n7;
        var_ibbt_dn8 = assign17090_e15615_d_n8;

        let assign17100_e15618: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        var_guard284 = assign17100_e15618;

        let (assign17110_e15629, assign17110_e15629_d_n5, assign17110_e15629_d_n6, assign17110_e15629_d_n7, assign17110_e15629_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17110_e15629;
        var_fbreakdown_dn5 = assign17110_e15629_d_n5;
        var_fbreakdown_dn6 = assign17110_e15629_d_n6;
        var_fbreakdown_dn7 = assign17110_e15629_d_n7;
        var_fbreakdown_dn8 = assign17110_e15629_d_n8;

        let assign17120_e15632: f64 = (-var_alphaav);
        let assign17120_e15634: f64 = (assign17120_e15632 * p.p861);
        let assign17120_e15635: f64 = if var_vav > assign17120_e15634 { 1.0 } else { 0.0 };
        var_guard285 = assign17120_e15635;

        let assign17130_e15638: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        var_guard286 = assign17130_e15638;

        let (assign17140_e15668, assign17140_e15668_d_n5, assign17140_e15668_d_n6, assign17140_e15668_d_n7, assign17140_e15668_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 != 0.0)) && (var_guard286 != 0.0)) {
        let assign17140_e15654: f64 = (var_vav * var_vbrinvsti);
        let assign17140_e15657: f64 = (var_vav * var_vbrinvsti);
        let assign17140_e15658: f64 = (assign17140_e15654 * assign17140_e15657);
        let assign17140_e15661: f64 = (var_vav * var_vbrinvsti);
        let assign17140_e15662: f64 = (assign17140_e15658 * assign17140_e15661);
        let assign17140_e15665: f64 = (var_vav * var_vbrinvsti);
        let assign17140_e15666: f64 = (assign17140_e15662 * assign17140_e15665);
        (assign17140_e15666, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17140_e15668;
        var_tmp_dn5 = assign17140_e15668_d_n5;
        var_tmp_dn6 = assign17140_e15668_d_n6;
        var_tmp_dn7 = assign17140_e15668_d_n7;
        var_tmp_dn8 = assign17140_e15668_d_n8;

        let (assign17150_e15690, assign17150_e15690_d_n5, assign17150_e15690_d_n6, assign17150_e15690_d_n7, assign17150_e15690_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 != 0.0)) && (var_guard286 == 0.0)) {
        let assign17150_e15685: f64 = (var_vav * var_vbrinvsti);
        let assign17150_e15686: f64 = (assign17150_e15685).abs();
        let assign17150_e15688: f64 = (assign17150_e15686).powf(p.p864);
        (assign17150_e15688, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17150_e15690;
        var_tmp_dn5 = assign17150_e15690_d_n5;
        var_tmp_dn6 = assign17150_e15690_d_n6;
        var_tmp_dn7 = assign17150_e15690_d_n7;
        var_tmp_dn8 = assign17150_e15690_d_n8;

        let (assign17160_e15708, assign17160_e15708_d_n5, assign17160_e15708_d_n6, assign17160_e15708_d_n7, assign17160_e15708_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 != 0.0)) {
        let assign17160_e15705: f64 = (1.0 - var_tmp);
        let assign17160_e15706: f64 = (1.0 / assign17160_e15705);
        (assign17160_e15706, (-((-var_tmp_dn5) / (assign17160_e15705 * assign17160_e15705))), (-((-var_tmp_dn6) / (assign17160_e15705 * assign17160_e15705))), (-((-var_tmp_dn7) / (assign17160_e15705 * assign17160_e15705))), (-((-var_tmp_dn8) / (assign17160_e15705 * assign17160_e15705))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17160_e15708;
        var_fbreakdown_dn5 = assign17160_e15708_d_n5;
        var_fbreakdown_dn6 = assign17160_e15708_d_n6;
        var_fbreakdown_dn7 = assign17160_e15708_d_n7;
        var_fbreakdown_dn8 = assign17160_e15708_d_n8;

        let (assign17170_e15731, assign17170_e15731_d_n5, assign17170_e15731_d_n6, assign17170_e15731_d_n7, assign17170_e15731_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 == 0.0)) {
        let assign17170_e15725: f64 = (var_alphaav * p.p861);
        let assign17170_e15726: f64 = (var_vav + assign17170_e15725);
        let assign17170_e15728: f64 = (assign17170_e15726 * var_slopesti);
        let assign17170_e15729: f64 = (var_fstopsti + assign17170_e15728);
        (assign17170_e15729, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17170_e15731;
        var_fbreakdown_dn5 = assign17170_e15731_d_n5;
        var_fbreakdown_dn6 = assign17170_e15731_d_n6;
        var_fbreakdown_dn7 = assign17170_e15731_d_n7;
        var_fbreakdown_dn8 = assign17170_e15731_d_n8;

        let (assign17180_e15750, assign17180_e15750_d_n5, assign17180_e15750_d_n6, assign17180_e15750_d_n7, assign17180_e15750_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard270 == 0.0)) {
        let assign17180_e15741: f64 = (var_id__blk219 + var_isrh);
        let assign17180_e15743: f64 = (assign17180_e15741 + var_itat);
        let assign17180_e15745: f64 = (assign17180_e15743 + var_ibbt);
        let assign17180_e15746: f64 = (p.p29 * assign17180_e15745);
        let assign17180_e15748: f64 = (assign17180_e15746 * var_fbreakdown);
        (assign17180_e15748, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign17180_e15746 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign17180_e15746 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign17180_e15746 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign17180_e15746 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign17180_e15750;
        var_ijunsti_dn5 = assign17180_e15750_d_n5;
        var_ijunsti_dn6 = assign17180_e15750_d_n6;
        var_ijunsti_dn7 = assign17180_e15750_d_n7;
        var_ijunsti_dn8 = assign17180_e15750_d_n8;

        let assign17190_e15753: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard287 = assign17190_e15753;

        let (assign17200_e15761, assign17200_e15761_d_n5, assign17200_e15761_d_n6, assign17200_e15761_d_n7, assign17200_e15761_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign17200_e15761;
        var_ijungat_dn5 = assign17200_e15761_d_n5;
        var_ijungat_dn6 = assign17200_e15761_d_n6;
        var_ijungat_dn7 = assign17200_e15761_d_n7;
        var_ijungat_dn8 = assign17200_e15761_d_n8;

        let (assign17210_e15772,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) {
        let assign17210_e15770: f64 = (var_idsatgat * var_idmult);
        (assign17210_e15770,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign17210_e15772;

        let assign17220_e15779: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        var_guard288 = assign17220_e15779;

        let (assign17230_e15790, assign17230_e15790_d_n5, assign17230_e15790_d_n6, assign17230_e15790_d_n7, assign17230_e15790_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign17230_e15790;
        var_isrh_dn5 = assign17230_e15790_d_n5;
        var_isrh_dn6 = assign17230_e15790_d_n6;
        var_isrh_dn7 = assign17230_e15790_d_n7;
        var_isrh_dn8 = assign17230_e15790_d_n8;

        let (assign17240_e15804,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17240_e15802: f64 = (var_vbigat - var_vjsrh);
        (assign17240_e15802,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign17240_e15804;

        let (assign17250_e15823,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17250_e15818: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign17250_e15819: f64 = (1.0 - assign17250_e15818);
        let assign17250_e15820: f64 = (assign17250_e15819).sqrt();
        let assign17250_e15821: f64 = (1.0 - assign17250_e15820);
        (assign17250_e15821,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign17250_e15823;

        let assign17260_e15826: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard289 = assign17260_e15826;

        let (assign17270_e15840,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) && (var_guard289 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17270_e15840;

        let (assign17280_e15872,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) && (var_guard289 == 0.0)) {
        let assign17280_e15855: f64 = (var_wsrhstep * var_wsrhstep);
        let assign17280_e15857: f64 = (var_wsrhstep).ln();
        let assign17280_e15858: f64 = (assign17280_e15855 * assign17280_e15857);
        let assign17280_e15861: f64 = (1.0 - var_wsrhstep);
        let assign17280_e15862: f64 = (assign17280_e15858 / assign17280_e15861);
        let assign17280_e15864: f64 = (assign17280_e15862 + var_wsrhstep);
        let assign17280_e15868: f64 = (2.0 * p.p833);
        let assign17280_e15869: f64 = (1.0 - assign17280_e15868);
        let assign17280_e15870: f64 = (assign17280_e15864 * assign17280_e15869);
        (assign17280_e15870,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17280_e15872;

        let (assign17290_e15886,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17290_e15884: f64 = (var_wsrhstep + var_dwsrh);
        (assign17290_e15884,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign17290_e15886;

        let assign17300_e15889: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard290 = assign17300_e15889;

        let (assign17310_e15906, assign17310_e15906_d_n5, assign17310_e15906_d_n6, assign17310_e15906_d_n7, assign17310_e15906_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) && (var_guard290 != 0.0)) {
        let assign17310_e15903: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign17310_e15904: f64 = (assign17310_e15903).sqrt();
        (assign17310_e15904, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17310_e15906;
        var_tmp_dn5 = assign17310_e15906_d_n5;
        var_tmp_dn6 = assign17310_e15906_d_n6;
        var_tmp_dn7 = assign17310_e15906_d_n7;
        var_tmp_dn8 = assign17310_e15906_d_n8;

        let (assign17320_e15925, assign17320_e15925_d_n5, assign17320_e15925_d_n6, assign17320_e15925_d_n7, assign17320_e15925_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) && (var_guard290 == 0.0)) {
        let assign17320_e15921: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign17320_e15923: f64 = (assign17320_e15921).powf(p.p833);
        (assign17320_e15923, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17320_e15925;
        var_tmp_dn5 = assign17320_e15925_d_n5;
        var_tmp_dn6 = assign17320_e15925_d_n6;
        var_tmp_dn7 = assign17320_e15925_d_n7;
        var_tmp_dn8 = assign17320_e15925_d_n8;

        *var_dwsrh_slot = var_dwsrh;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard280_slot = var_guard280;
        *var_guard281_slot = var_guard281;
        *var_guard282_slot = var_guard282;
        *var_guard283_slot = var_guard283;
        *var_guard284_slot = var_guard284;
        *var_guard285_slot = var_guard285;
        *var_guard286_slot = var_guard286;
        *var_guard287_slot = var_guard287;
        *var_guard288_slot = var_guard288;
        *var_guard289_slot = var_guard289;
        *var_guard290_slot = var_guard290;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_ftdgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard287: f64,
        var_guard288: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_wsrh: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard291_slot: &mut f64,
        var_guard292_slot: &mut f64,
        var_guard293_slot: &mut f64,
        var_guard294_slot: &mut f64,
        var_guard295_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard291: f64 = *var_guard291_slot;
        let mut var_guard292: f64 = *var_guard292_slot;
        let mut var_guard293: f64 = *var_guard293_slot;
        let mut var_guard294: f64 = *var_guard294_slot;
        let mut var_guard295: f64 = *var_guard295_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign17330_e15939, assign17330_e15939_d_n5, assign17330_e15939_d_n6, assign17330_e15939_d_n7, assign17330_e15939_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17330_e15937: f64 = (var_wdepnulrgat * var_tmp);
        (assign17330_e15937, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign17330_e15939;
        var_wdep_dn5 = assign17330_e15939_d_n5;
        var_wdep_dn6 = assign17330_e15939_d_n6;
        var_wdep_dn7 = assign17330_e15939_d_n7;
        var_wdep_dn8 = assign17330_e15939_d_n8;

        let (assign17340_e15957, assign17340_e15957_d_n5, assign17340_e15957_d_n6, assign17340_e15957_d_n7, assign17340_e15957_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17340_e15952: f64 = (var_zinv - 1.0);
        let assign17340_e15954: f64 = (assign17340_e15952 * var_wdep);
        let assign17340_e15955: f64 = (var_ftdgat * assign17340_e15954);
        (assign17340_e15955, (var_ftdgat * (assign17340_e15952 * var_wdep_dn5)), (var_ftdgat * (assign17340_e15952 * var_wdep_dn6)), (var_ftdgat * (assign17340_e15952 * var_wdep_dn7)), (var_ftdgat * (assign17340_e15952 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign17340_e15957;
        var_asrh_dn5 = assign17340_e15957_d_n5;
        var_asrh_dn6 = assign17340_e15957_d_n6;
        var_asrh_dn7 = assign17340_e15957_d_n7;
        var_asrh_dn8 = assign17340_e15957_d_n8;

        let (assign17350_e15973, assign17350_e15973_d_n5, assign17350_e15973_d_n6, assign17350_e15973_d_n7, assign17350_e15973_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard288 == 0.0)) {
        let assign17350_e15970: f64 = (var_asrh * var_wsrh);
        let assign17350_e15971: f64 = (p.p842 * assign17350_e15970);
        (assign17350_e15971, (p.p842 * (var_asrh_dn5 * var_wsrh)), (p.p842 * (var_asrh_dn6 * var_wsrh)), (p.p842 * (var_asrh_dn7 * var_wsrh)), (p.p842 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign17350_e15973;
        var_isrh_dn5 = assign17350_e15973_d_n5;
        var_isrh_dn6 = assign17350_e15973_d_n6;
        var_isrh_dn7 = assign17350_e15973_d_n7;
        var_isrh_dn8 = assign17350_e15973_d_n8;

        let assign17360_e15976: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        var_guard291 = assign17360_e15976;

        let (assign17370_e15987, assign17370_e15987_d_n5, assign17370_e15987_d_n6, assign17370_e15987_d_n7, assign17370_e15987_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign17370_e15987;
        var_itat_dn5 = assign17370_e15987_d_n5;
        var_itat_dn6 = assign17370_e15987_d_n6;
        var_itat_dn7 = assign17370_e15987_d_n7;
        var_itat_dn8 = assign17370_e15987_d_n8;

        let (assign17380_e16005, assign17380_e16005_d_n5, assign17380_e16005_d_n6, assign17380_e16005_d_n7, assign17380_e16005_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17380_e16000: f64 = (var_wdep * var_one_minus_pgat);
        let assign17380_e16002: f64 = (assign17380_e16000 / var_vbi_minus_vjsrh);
        let assign17380_e16003: f64 = (var_btatpartgat * assign17380_e16002);
        (assign17380_e16003, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign17380_e16005;
        var_btat_dn5 = assign17380_e16005_d_n5;
        var_btat_dn6 = assign17380_e16005_d_n6;
        var_btat_dn7 = assign17380_e16005_d_n7;
        var_btat_dn8 = assign17380_e16005_d_n8;

        let (assign17390_e16021, assign17390_e16021_d_n5, assign17390_e16021_d_n6, assign17390_e16021_d_n7, assign17390_e16021_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17390_e16017: f64 = (0.666666666666667 * var_atatgat);
        let assign17390_e16019: f64 = (assign17390_e16017 / var_btat);
        (assign17390_e16019, (-((assign17390_e16017 * var_btat_dn5) / (var_btat * var_btat))), (-((assign17390_e16017 * var_btat_dn6) / (var_btat * var_btat))), (-((assign17390_e16017 * var_btat_dn7) / (var_btat * var_btat))), (-((assign17390_e16017 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign17390_e16021;
        var_twoatatoverthreebtat_dn5 = assign17390_e16021_d_n5;
        var_twoatatoverthreebtat_dn6 = assign17390_e16021_d_n6;
        var_twoatatoverthreebtat_dn7 = assign17390_e16021_d_n7;
        var_twoatatoverthreebtat_dn8 = assign17390_e16021_d_n8;

        let (assign17400_e16035, assign17400_e16035_d_n5, assign17400_e16035_d_n6, assign17400_e16035_d_n7, assign17400_e16035_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17400_e16033: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign17400_e16033, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign17400_e16035;
        var_umaxbeforelimiting_dn5 = assign17400_e16035_d_n5;
        var_umaxbeforelimiting_dn6 = assign17400_e16035_d_n6;
        var_umaxbeforelimiting_dn7 = assign17400_e16035_d_n7;
        var_umaxbeforelimiting_dn8 = assign17400_e16035_d_n8;

        let (assign17410_e16056, assign17410_e16056_d_n5, assign17410_e16056_d_n6, assign17410_e16056_d_n7, assign17410_e16056_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17410_e16047: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign17410_e16050: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign17410_e16052: f64 = (assign17410_e16050 + 1.0);
        let assign17410_e16053: f64 = (assign17410_e16047 / assign17410_e16052);
        let assign17410_e16054: f64 = (assign17410_e16053).sqrt();
        (assign17410_e16054, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign17410_e16052) - (assign17410_e16047 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign17410_e16052) - (assign17410_e16047 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign17410_e16052) - (assign17410_e16047 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign17410_e16052) - (assign17410_e16047 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign17410_e16052 * assign17410_e16052)) / (2.0 * assign17410_e16054)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign17410_e16056;
        var_umax_dn5 = assign17410_e16056_d_n5;
        var_umax_dn6 = assign17410_e16056_d_n6;
        var_umax_dn7 = assign17410_e16056_d_n7;
        var_umax_dn8 = assign17410_e16056_d_n8;

        let (assign17420_e16069, assign17420_e16069_d_n5, assign17420_e16069_d_n6, assign17420_e16069_d_n7, assign17420_e16069_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17420_e16067: f64 = (var_umax).sqrt();
        (assign17420_e16067, (var_umax_dn5 / (2.0 * assign17420_e16067)), (var_umax_dn6 / (2.0 * assign17420_e16067)), (var_umax_dn7 / (2.0 * assign17420_e16067)), (var_umax_dn8 / (2.0 * assign17420_e16067)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign17420_e16069;
        var_sqrtumax_dn5 = assign17420_e16069_d_n5;
        var_sqrtumax_dn6 = assign17420_e16069_d_n6;
        var_sqrtumax_dn7 = assign17420_e16069_d_n7;
        var_sqrtumax_dn8 = assign17420_e16069_d_n8;

        let (assign17430_e16083, assign17430_e16083_d_n5, assign17430_e16083_d_n6, assign17430_e16083_d_n7, assign17430_e16083_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17430_e16081: f64 = (var_umax * var_sqrtumax);
        (assign17430_e16081, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign17430_e16083;
        var_umaxpoweronepointfive_dn5 = assign17430_e16083_d_n5;
        var_umaxpoweronepointfive_dn6 = assign17430_e16083_d_n6;
        var_umaxpoweronepointfive_dn7 = assign17430_e16083_d_n7;
        var_umaxpoweronepointfive_dn8 = assign17430_e16083_d_n8;

        let assign17440_e16085: f64 = (-p.p833);
        let assign17440_e16087: f64 = (assign17440_e16085 * var_one_over_one_minus_pgat);
        let assign17440_e16089: f64 = (-1.0);
        let assign17440_e16090: f64 = if assign17440_e16087 == assign17440_e16089 { 1.0 } else { 0.0 };
        var_guard292 = assign17440_e16090;

        let (assign17450_e16110, assign17450_e16110_d_n5, assign17450_e16110_d_n6, assign17450_e16110_d_n7, assign17450_e16110_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard292 != 0.0)) {
        let assign17450_e16106: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17450_e16107: f64 = (1.0 + assign17450_e16106);
        let assign17450_e16108: f64 = (1.0 / assign17450_e16107);
        (assign17450_e16108, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign17450_e16107 * assign17450_e16107))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign17450_e16107 * assign17450_e16107))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign17450_e16107 * assign17450_e16107))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign17450_e16107 * assign17450_e16107))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign17450_e16110;
        var_wgamma_dn5 = assign17450_e16110_d_n5;
        var_wgamma_dn6 = assign17450_e16110_d_n6;
        var_wgamma_dn7 = assign17450_e16110_d_n7;
        var_wgamma_dn8 = assign17450_e16110_d_n8;

        let (assign17460_e16134, assign17460_e16134_d_n5, assign17460_e16134_d_n6, assign17460_e16134_d_n7, assign17460_e16134_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard292 == 0.0)) {
        let assign17460_e16126: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17460_e16127: f64 = (1.0 + assign17460_e16126);
        let assign17460_e16129: f64 = (-p.p833);
        let assign17460_e16131: f64 = (assign17460_e16129 * var_one_over_one_minus_pgat);
        let assign17460_e16132: f64 = (assign17460_e16127).powf(assign17460_e16131);
        (assign17460_e16132, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign17460_e16127))) }, if 0.0 == 0.0 && ((assign17460_e16131) as f64).is_finite() && ((assign17460_e16131) as f64).fract() == 0.0 { if assign17460_e16131 == 0.0 { 0.0 } else { (assign17460_e16131 * ((assign17460_e16127).powf(assign17460_e16131 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign17460_e16132 * (assign17460_e16131 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign17460_e16127))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign17460_e16134;
        var_wgamma_dn5 = assign17460_e16134_d_n5;
        var_wgamma_dn6 = assign17460_e16134_d_n6;
        var_wgamma_dn7 = assign17460_e16134_d_n7;
        var_wgamma_dn8 = assign17460_e16134_d_n8;

        let (assign17470_e16152, assign17470_e16152_d_n5, assign17470_e16152_d_n6, assign17470_e16152_d_n7, assign17470_e16152_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17470_e16146: f64 = (var_wsrh * var_wgamma);
        let assign17470_e16149: f64 = (var_wsrh + var_wgamma);
        let assign17470_e16150: f64 = (assign17470_e16146 / assign17470_e16149);
        (assign17470_e16150, ((((var_wsrh * var_wgamma_dn5) * assign17470_e16149) - (assign17470_e16146 * var_wgamma_dn5)) / (assign17470_e16149 * assign17470_e16149)), ((((var_wsrh * var_wgamma_dn6) * assign17470_e16149) - (assign17470_e16146 * var_wgamma_dn6)) / (assign17470_e16149 * assign17470_e16149)), ((((var_wsrh * var_wgamma_dn7) * assign17470_e16149) - (assign17470_e16146 * var_wgamma_dn7)) / (assign17470_e16149 * assign17470_e16149)), ((((var_wsrh * var_wgamma_dn8) * assign17470_e16149) - (assign17470_e16146 * var_wgamma_dn8)) / (assign17470_e16149 * assign17470_e16149)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign17470_e16152;
        var_wtat_dn5 = assign17470_e16152_d_n5;
        var_wtat_dn6 = assign17470_e16152_d_n6;
        var_wtat_dn7 = assign17470_e16152_d_n7;
        var_wtat_dn8 = assign17470_e16152_d_n8;

        let (assign17480_e16169, assign17480_e16169_d_n5, assign17480_e16169_d_n6, assign17480_e16169_d_n7, assign17480_e16169_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17480_e16165: f64 = (var_btat / var_sqrtumax);
        let assign17480_e16166: f64 = (0.375 * assign17480_e16165);
        let assign17480_e16167: f64 = (assign17480_e16166).sqrt();
        (assign17480_e16167, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17480_e16167)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17480_e16167)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign17480_e16169;
        var_ktat_dn5 = assign17480_e16169_d_n5;
        var_ktat_dn6 = assign17480_e16169_d_n6;
        var_ktat_dn7 = assign17480_e16169_d_n7;
        var_ktat_dn8 = assign17480_e16169_d_n8;

        let (assign17490_e16187, assign17490_e16187_d_n5, assign17490_e16187_d_n6, assign17490_e16187_d_n7, assign17490_e16187_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17490_e16182: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign17490_e16183: f64 = (2.0 * assign17490_e16182);
        let assign17490_e16185: f64 = (assign17490_e16183 - var_umax);
        (assign17490_e16185, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign17490_e16187;
        var_ltat_dn5 = assign17490_e16187_d_n5;
        var_ltat_dn6 = assign17490_e16187_d_n6;
        var_ltat_dn7 = assign17490_e16187_d_n7;
        var_ltat_dn8 = assign17490_e16187_d_n8;

        let (assign17500_e16213, assign17500_e16213_d_n5, assign17500_e16213_d_n6, assign17500_e16213_d_n7, assign17500_e16213_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17500_e16199: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign17500_e16201: f64 = (assign17500_e16199 * var_sqrtumax);
        let assign17500_e16204: f64 = (var_atatgat * var_umax);
        let assign17500_e16205: f64 = (assign17500_e16201 - assign17500_e16204);
        let assign17500_e16209: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17500_e16210: f64 = (0.5 * assign17500_e16209);
        let assign17500_e16211: f64 = (assign17500_e16205 + assign17500_e16210);
        (assign17500_e16211, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign17500_e16199 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign17500_e16199 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign17500_e16199 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign17500_e16199 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign17500_e16213;
        var_mtat_dn5 = assign17500_e16213_d_n5;
        var_mtat_dn6 = assign17500_e16213_d_n6;
        var_mtat_dn7 = assign17500_e16213_d_n7;
        var_mtat_dn8 = assign17500_e16213_d_n8;

        let (assign17510_e16229, assign17510_e16229_d_n5, assign17510_e16229_d_n6, assign17510_e16229_d_n7, assign17510_e16229_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17510_e16225: f64 = (var_ltat - 1.0);
        let assign17510_e16227: f64 = (assign17510_e16225 * var_ktat);
        (assign17510_e16227, ((var_ltat_dn5 * var_ktat) + (assign17510_e16225 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign17510_e16225 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign17510_e16225 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign17510_e16225 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign17510_e16229;
        var_xerfc_dn5 = assign17510_e16229_d_n5;
        var_xerfc_dn6 = assign17510_e16229_d_n6;
        var_xerfc_dn7 = assign17510_e16229_d_n7;
        var_xerfc_dn8 = assign17510_e16229_d_n8;

        let (assign17520_e16243, assign17520_e16243_d_n5, assign17520_e16243_d_n6, assign17520_e16243_d_n7, assign17520_e16243_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17520_e16241: f64 = (var_xerfc * var_xerfc);
        (assign17520_e16241, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign17520_e16243;
        var_ysq_dn5 = assign17520_e16243_d_n5;
        var_ysq_dn6 = assign17520_e16243_d_n6;
        var_ysq_dn7 = assign17520_e16243_d_n7;
        var_ysq_dn8 = assign17520_e16243_d_n8;

        let assign17530_e16246: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard293 = assign17530_e16246;

        let (assign17540_e16266, assign17540_e16266_d_n5, assign17540_e16266_d_n6, assign17540_e16266_d_n7, assign17540_e16266_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard293 != 0.0)) {
        let assign17540_e16262: f64 = (var_perfc * var_xerfc);
        let assign17540_e16263: f64 = (1.0 + assign17540_e16262);
        let assign17540_e16264: f64 = (1.0 / assign17540_e16263);
        (assign17540_e16264, (-((var_perfc * var_xerfc_dn5) / (assign17540_e16263 * assign17540_e16263))), (-((var_perfc * var_xerfc_dn6) / (assign17540_e16263 * assign17540_e16263))), (-((var_perfc * var_xerfc_dn7) / (assign17540_e16263 * assign17540_e16263))), (-((var_perfc * var_xerfc_dn8) / (assign17540_e16263 * assign17540_e16263))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign17540_e16266;
        var_terfc_dn5 = assign17540_e16266_d_n5;
        var_terfc_dn6 = assign17540_e16266_d_n6;
        var_terfc_dn7 = assign17540_e16266_d_n7;
        var_terfc_dn8 = assign17540_e16266_d_n8;

        let (assign17550_e16287, assign17550_e16287_d_n5, assign17550_e16287_d_n6, assign17550_e16287_d_n7, assign17550_e16287_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard293 == 0.0)) {
        let assign17550_e16283: f64 = (var_perfc * var_xerfc);
        let assign17550_e16284: f64 = (1.0 - assign17550_e16283);
        let assign17550_e16285: f64 = (1.0 / assign17550_e16284);
        (assign17550_e16285, (-((-(var_perfc * var_xerfc_dn5)) / (assign17550_e16284 * assign17550_e16284))), (-((-(var_perfc * var_xerfc_dn6)) / (assign17550_e16284 * assign17550_e16284))), (-((-(var_perfc * var_xerfc_dn7)) / (assign17550_e16284 * assign17550_e16284))), (-((-(var_perfc * var_xerfc_dn8)) / (assign17550_e16284 * assign17550_e16284))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign17550_e16287;
        var_terfc_dn5 = assign17550_e16287_d_n5;
        var_terfc_dn6 = assign17550_e16287_d_n6;
        var_terfc_dn7 = assign17550_e16287_d_n7;
        var_terfc_dn8 = assign17550_e16287_d_n8;

        let assign17560_e16289: f64 = (-var_ysq);
        let assign17560_e16291: f64 = (assign17560_e16289 + var_mtat);
        let assign17560_e16293: f64 = (-230.25850929940458);
        let assign17560_e16294: f64 = if assign17560_e16291 > assign17560_e16293 { 1.0 } else { 0.0 };
        var_guard294 = assign17560_e16294;

        let (assign17570_e16312, assign17570_e16312_d_n5, assign17570_e16312_d_n6, assign17570_e16312_d_n7, assign17570_e16312_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard294 != 0.0)) {
        let assign17570_e16307: f64 = (-var_ysq);
        let assign17570_e16309: f64 = (assign17570_e16307 + var_mtat);
        let assign17570_e16310: f64 = (assign17570_e16309).exp();
        (assign17570_e16310, (assign17570_e16310 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign17570_e16310 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign17570_e16310 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign17570_e16310 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17570_e16312;
        var_tmp_dn5 = assign17570_e16312_d_n5;
        var_tmp_dn6 = assign17570_e16312_d_n6;
        var_tmp_dn7 = assign17570_e16312_d_n7;
        var_tmp_dn8 = assign17570_e16312_d_n8;

        let (assign17580_e16361, assign17580_e16361_d_n5, assign17580_e16361_d_n6, assign17580_e16361_d_n7, assign17580_e16361_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard294 == 0.0)) {
        let assign17580_e16328: f64 = (-230.25850929940458);
        let assign17580_e16330: f64 = (-var_ysq);
        let assign17580_e16332: f64 = (assign17580_e16330 + var_mtat);
        let assign17580_e16333: f64 = (assign17580_e16328 - assign17580_e16332);
        let assign17580_e16337: f64 = (-230.25850929940458);
        let assign17580_e16339: f64 = (-var_ysq);
        let assign17580_e16341: f64 = (assign17580_e16339 + var_mtat);
        let assign17580_e16342: f64 = (assign17580_e16337 - assign17580_e16341);
        let assign17580_e16345: f64 = (-230.25850929940458);
        let assign17580_e16347: f64 = (-var_ysq);
        let assign17580_e16349: f64 = (assign17580_e16347 + var_mtat);
        let assign17580_e16350: f64 = (assign17580_e16345 - assign17580_e16349);
        let assign17580_e16352: f64 = (assign17580_e16350 * 0.3333333333333333);
        let assign17580_e16353: f64 = (1.0 + assign17580_e16352);
        let assign17580_e16354: f64 = (assign17580_e16342 * assign17580_e16353);
        let assign17580_e16355: f64 = (0.5 * assign17580_e16354);
        let assign17580_e16356: f64 = (1.0 + assign17580_e16355);
        let assign17580_e16357: f64 = (assign17580_e16333 * assign17580_e16356);
        let assign17580_e16358: f64 = (1.0 + assign17580_e16357);
        let assign17580_e16359: f64 = (1e-100 / assign17580_e16358);
        (assign17580_e16359, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign17580_e16353) + (assign17580_e16342 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign17580_e16353) + (assign17580_e16342 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign17580_e16353) + (assign17580_e16342 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign17580_e16356) + (assign17580_e16333 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign17580_e16353) + (assign17580_e16342 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign17580_e16358 * assign17580_e16358))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17580_e16361;
        var_tmp_dn5 = assign17580_e16361_d_n5;
        var_tmp_dn6 = assign17580_e16361_d_n6;
        var_tmp_dn7 = assign17580_e16361_d_n7;
        var_tmp_dn8 = assign17580_e16361_d_n8;

        let (assign17590_e16391, assign17590_e16391_d_n5, assign17590_e16391_d_n6, assign17590_e16391_d_n7, assign17590_e16391_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17590_e16373: f64 = (0.29214664 * var_terfc);
        let assign17590_e16377: f64 = (var_terfc * var_terfc);
        let assign17590_e16378: f64 = (var_berfc * assign17590_e16377);
        let assign17590_e16379: f64 = (assign17590_e16373 + assign17590_e16378);
        let assign17590_e16383: f64 = (var_terfc * var_terfc);
        let assign17590_e16385: f64 = (assign17590_e16383 * var_terfc);
        let assign17590_e16386: f64 = (var_cerfc * assign17590_e16385);
        let assign17590_e16387: f64 = (assign17590_e16379 + assign17590_e16386);
        let assign17590_e16389: f64 = (assign17590_e16387 * var_tmp);
        (assign17590_e16389, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign17590_e16383 * var_terfc_dn5)))) * var_tmp) + (assign17590_e16387 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign17590_e16383 * var_terfc_dn6)))) * var_tmp) + (assign17590_e16387 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign17590_e16383 * var_terfc_dn7)))) * var_tmp) + (assign17590_e16387 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign17590_e16383 * var_terfc_dn8)))) * var_tmp) + (assign17590_e16387 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign17590_e16391;
        var_erfcpos_dn5 = assign17590_e16391_d_n5;
        var_erfcpos_dn6 = assign17590_e16391_d_n6;
        var_erfcpos_dn7 = assign17590_e16391_d_n7;
        var_erfcpos_dn8 = assign17590_e16391_d_n8;

        let assign17600_e16394: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard295 = assign17600_e16394;

        let (assign17610_e16408, assign17610_e16408_d_n5, assign17610_e16408_d_n6, assign17610_e16408_d_n7, assign17610_e16408_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard295 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign17610_e16408;
        var_erfctimesexpmtat_dn5 = assign17610_e16408_d_n5;
        var_erfctimesexpmtat_dn6 = assign17610_e16408_d_n6;
        var_erfctimesexpmtat_dn7 = assign17610_e16408_d_n7;
        var_erfctimesexpmtat_dn8 = assign17610_e16408_d_n8;

        let assign17620_e16411: f64 = (-230.25850929940458);
        let assign17620_e16412: f64 = if var_mtat > assign17620_e16411 { 1.0 } else { 0.0 };
        var_guard296 = assign17620_e16412;

        let (assign17630_e16430, assign17630_e16430_d_n5, assign17630_e16430_d_n6, assign17630_e16430_d_n7, assign17630_e16430_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard295 == 0.0)) && (var_guard296 != 0.0)) {
        let assign17630_e16428: f64 = (var_mtat).exp();
        (assign17630_e16428, (assign17630_e16428 * var_mtat_dn5), (assign17630_e16428 * var_mtat_dn6), (assign17630_e16428 * var_mtat_dn7), (assign17630_e16428 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17630_e16430;
        var_tmp_dn5 = assign17630_e16430_d_n5;
        var_tmp_dn6 = assign17630_e16430_d_n6;
        var_tmp_dn7 = assign17630_e16430_d_n7;
        var_tmp_dn8 = assign17630_e16430_d_n8;

        let (assign17640_e16473, assign17640_e16473_d_n5, assign17640_e16473_d_n6, assign17640_e16473_d_n7, assign17640_e16473_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard295 == 0.0)) && (var_guard296 == 0.0)) {
        let assign17640_e16449: f64 = (-230.25850929940458);
        let assign17640_e16451: f64 = (assign17640_e16449 - var_mtat);
        let assign17640_e16455: f64 = (-230.25850929940458);
        let assign17640_e16457: f64 = (assign17640_e16455 - var_mtat);
        let assign17640_e16460: f64 = (-230.25850929940458);
        let assign17640_e16462: f64 = (assign17640_e16460 - var_mtat);
        let assign17640_e16464: f64 = (assign17640_e16462 * 0.3333333333333333);
        let assign17640_e16465: f64 = (1.0 + assign17640_e16464);
        let assign17640_e16466: f64 = (assign17640_e16457 * assign17640_e16465);
        let assign17640_e16467: f64 = (0.5 * assign17640_e16466);
        let assign17640_e16468: f64 = (1.0 + assign17640_e16467);
        let assign17640_e16469: f64 = (assign17640_e16451 * assign17640_e16468);
        let assign17640_e16470: f64 = (1.0 + assign17640_e16469);
        let assign17640_e16471: f64 = (1e-100 / assign17640_e16470);
        (assign17640_e16471, (-((1e-100 * (((-var_mtat_dn5) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-var_mtat_dn5) * assign17640_e16465) + (assign17640_e16457 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-var_mtat_dn6) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-var_mtat_dn6) * assign17640_e16465) + (assign17640_e16457 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-var_mtat_dn7) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-var_mtat_dn7) * assign17640_e16465) + (assign17640_e16457 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))), (-((1e-100 * (((-var_mtat_dn8) * assign17640_e16468) + (assign17640_e16451 * (0.5 * (((-var_mtat_dn8) * assign17640_e16465) + (assign17640_e16457 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign17640_e16470 * assign17640_e16470))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17640_e16473;
        var_tmp_dn5 = assign17640_e16473_d_n5;
        var_tmp_dn6 = assign17640_e16473_d_n6;
        var_tmp_dn7 = assign17640_e16473_d_n7;
        var_tmp_dn8 = assign17640_e16473_d_n8;

        let (assign17650_e16492, assign17650_e16492_d_n5, assign17650_e16492_d_n6, assign17650_e16492_d_n7, assign17650_e16492_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) && (var_guard295 == 0.0)) {
        let assign17650_e16488: f64 = (2.0 * var_tmp);
        let assign17650_e16490: f64 = (assign17650_e16488 - var_erfcpos);
        (assign17650_e16490, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign17650_e16492;
        var_erfctimesexpmtat_dn5 = assign17650_e16492_d_n5;
        var_erfctimesexpmtat_dn6 = assign17650_e16492_d_n6;
        var_erfctimesexpmtat_dn7 = assign17650_e16492_d_n7;
        var_erfctimesexpmtat_dn8 = assign17650_e16492_d_n8;

        let (assign17660_e16512, assign17660_e16512_d_n5, assign17660_e16512_d_n6, assign17660_e16512_d_n7, assign17660_e16512_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17660_e16504: f64 = (1.772453850905516 * 0.5);
        let assign17660_e16507: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign17660_e16509: f64 = (assign17660_e16507 / var_ktat);
        let assign17660_e16510: f64 = (assign17660_e16504 * assign17660_e16509);
        (assign17660_e16510, (assign17660_e16504 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign17660_e16507 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign17660_e16504 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign17660_e16507 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign17660_e16504 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign17660_e16507 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign17660_e16504 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign17660_e16507 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign17660_e16512;
        var_gammamax_dn5 = assign17660_e16512_d_n5;
        var_gammamax_dn6 = assign17660_e16512_d_n6;
        var_gammamax_dn7 = assign17660_e16512_d_n7;
        var_gammamax_dn8 = assign17660_e16512_d_n8;

        let (assign17670_e16530, assign17670_e16530_d_n5, assign17670_e16530_d_n6, assign17670_e16530_d_n7, assign17670_e16530_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard291 == 0.0)) {
        let assign17670_e16525: f64 = (var_asrh * var_gammamax);
        let assign17670_e16527: f64 = (assign17670_e16525 * var_wtat);
        let assign17670_e16528: f64 = (p.p847 * assign17670_e16527);
        (assign17670_e16528, (p.p847 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign17670_e16525 * var_wtat_dn5))), (p.p847 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign17670_e16525 * var_wtat_dn6))), (p.p847 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign17670_e16525 * var_wtat_dn7))), (p.p847 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign17670_e16525 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign17670_e16530;
        var_itat_dn5 = assign17670_e16530_d_n5;
        var_itat_dn6 = assign17670_e16530_d_n6;
        var_itat_dn7 = assign17670_e16530_d_n7;
        var_itat_dn8 = assign17670_e16530_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard291_slot = var_guard291;
        *var_guard292_slot = var_guard292;
        *var_guard293_slot = var_guard293;
        *var_guard294_slot = var_guard294;
        *var_guard295_slot = var_guard295;
        *var_guard296_slot = var_guard296;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fstopgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard287: f64,
        var_id__blk219: f64,
        var_ijunbot: f64,
        var_ijunbot_dn5: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_isrh: f64,
        var_isrh_dn5: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_v1: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vmax_s: f64,
        var_wdepnulrinvgat: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard297_slot: &mut f64,
        var_guard298_slot: &mut f64,
        var_guard299_slot: &mut f64,
        var_guard300_slot: &mut f64,
        var_guard301_slot: &mut f64,
        var_guard302_slot: &mut f64,
        var_guard303_slot: &mut f64,
        var_guard304_slot: &mut f64,
        var_guard305_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard307_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_i1_slot: &mut f64,
        var_i1_dn5_slot: &mut f64,
        var_i1_dn6_slot: &mut f64,
        var_i1_dn7_slot: &mut f64,
        var_i1_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_guard297: f64 = *var_guard297_slot;
        let mut var_guard298: f64 = *var_guard298_slot;
        let mut var_guard299: f64 = *var_guard299_slot;
        let mut var_guard300: f64 = *var_guard300_slot;
        let mut var_guard301: f64 = *var_guard301_slot;
        let mut var_guard302: f64 = *var_guard302_slot;
        let mut var_guard303: f64 = *var_guard303_slot;
        let mut var_guard304: f64 = *var_guard304_slot;
        let mut var_guard305: f64 = *var_guard305_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_i1: f64 = *var_i1_slot;
        let mut var_i1_dn5: f64 = *var_i1_dn5_slot;
        let mut var_i1_dn6: f64 = *var_i1_dn6_slot;
        let mut var_i1_dn7: f64 = *var_i1_dn7_slot;
        let mut var_i1_dn8: f64 = *var_i1_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let assign17680_e16533: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        var_guard297 = assign17680_e16533;

        let (assign17690_e16544, assign17690_e16544_d_n5, assign17690_e16544_d_n6, assign17690_e16544_d_n7, assign17690_e16544_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign17690_e16544;
        var_ibbt_dn5 = assign17690_e16544_d_n5;
        var_ibbt_dn6 = assign17690_e16544_d_n6;
        var_ibbt_dn7 = assign17690_e16544_d_n7;
        var_ibbt_dn8 = assign17690_e16544_d_n8;

        let assign17700_e16547: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard298 = assign17700_e16547;

        let (assign17710_e16566, assign17710_e16566_d_n5, assign17710_e16566_d_n6, assign17710_e16566_d_n7, assign17710_e16566_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) && (var_guard298 != 0.0)) {
        let assign17710_e16561: f64 = (p.p830 - var_vbbt);
        let assign17710_e16563: f64 = (assign17710_e16561 * var_vbirgatinv);
        let assign17710_e16564: f64 = (assign17710_e16563).sqrt();
        (assign17710_e16564, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17710_e16566;
        var_tmp_dn5 = assign17710_e16566_d_n5;
        var_tmp_dn6 = assign17710_e16566_d_n6;
        var_tmp_dn7 = assign17710_e16566_d_n7;
        var_tmp_dn8 = assign17710_e16566_d_n8;

        let (assign17720_e16587, assign17720_e16587_d_n5, assign17720_e16587_d_n6, assign17720_e16587_d_n7, assign17720_e16587_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) && (var_guard298 == 0.0)) {
        let assign17720_e16581: f64 = (p.p830 - var_vbbt);
        let assign17720_e16583: f64 = (assign17720_e16581 * var_vbirgatinv);
        let assign17720_e16585: f64 = (assign17720_e16583).powf(p.p833);
        (assign17720_e16585, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17720_e16587;
        var_tmp_dn5 = assign17720_e16587_d_n5;
        var_tmp_dn6 = assign17720_e16587_d_n6;
        var_tmp_dn7 = assign17720_e16587_d_n7;
        var_tmp_dn8 = assign17720_e16587_d_n8;

        let (assign17730_e16607, assign17730_e16607_d_n5, assign17730_e16607_d_n6, assign17730_e16607_d_n7, assign17730_e16607_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) {
        let assign17730_e16600: f64 = (p.p830 - var_vbbt);
        let assign17730_e16602: f64 = (assign17730_e16600 * var_wdepnulrinvgat);
        let assign17730_e16604: f64 = (assign17730_e16602 / var_tmp);
        let assign17730_e16605: f64 = (var_one_over_one_minus_pgat * assign17730_e16604);
        (assign17730_e16605, (var_one_over_one_minus_pgat * (-((assign17730_e16602 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign17730_e16602 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign17730_e16602 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign17730_e16602 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign17730_e16607;
        var_fmaxr_dn5 = assign17730_e16607_d_n5;
        var_fmaxr_dn6 = assign17730_e16607_d_n6;
        var_fmaxr_dn7 = assign17730_e16607_d_n7;
        var_fmaxr_dn8 = assign17730_e16607_d_n8;

        let assign17740_e16609: f64 = (-var_fbbtgat);
        let assign17740_e16611: f64 = (assign17740_e16609 / var_fmaxr);
        let assign17740_e16612: f64 = (assign17740_e16611).abs();
        let assign17740_e16614: f64 = if assign17740_e16612 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard299 = assign17740_e16614;

        let (assign17750_e16632, assign17750_e16632_d_n5, assign17750_e16632_d_n6, assign17750_e16632_d_n7, assign17750_e16632_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) && (var_guard299 != 0.0)) {
        let assign17750_e16627: f64 = (-var_fbbtgat);
        let assign17750_e16629: f64 = (assign17750_e16627 / var_fmaxr);
        let assign17750_e16630: f64 = (assign17750_e16629).exp();
        (assign17750_e16630, (assign17750_e16630 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17750_e16627 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign17750_e16630 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17750_e16627 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign17750_e16630 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17750_e16627 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign17750_e16630 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17750_e16627 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17750_e16632;
        var_tmp_dn5 = assign17750_e16632_d_n5;
        var_tmp_dn6 = assign17750_e16632_d_n6;
        var_tmp_dn7 = assign17750_e16632_d_n7;
        var_tmp_dn8 = assign17750_e16632_d_n8;

        let assign17760_e16634: f64 = (-var_fbbtgat);
        let assign17760_e16636: f64 = (assign17760_e16634 / var_fmaxr);
        let assign17760_e16638: f64 = if assign17760_e16636 < 0.0 { 1.0 } else { 0.0 };
        var_guard300 = assign17760_e16638;

        let (assign17770_e16689, assign17770_e16689_d_n5, assign17770_e16689_d_n6, assign17770_e16689_d_n7, assign17770_e16689_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) && (var_guard299 == 0.0)) && (var_guard300 != 0.0)) {
        let assign17770_e16656: f64 = (-230.25850929940458);
        let assign17770_e16658: f64 = (-var_fbbtgat);
        let assign17770_e16660: f64 = (assign17770_e16658 / var_fmaxr);
        let assign17770_e16661: f64 = (assign17770_e16656 - assign17770_e16660);
        let assign17770_e16665: f64 = (-230.25850929940458);
        let assign17770_e16667: f64 = (-var_fbbtgat);
        let assign17770_e16669: f64 = (assign17770_e16667 / var_fmaxr);
        let assign17770_e16670: f64 = (assign17770_e16665 - assign17770_e16669);
        let assign17770_e16673: f64 = (-230.25850929940458);
        let assign17770_e16675: f64 = (-var_fbbtgat);
        let assign17770_e16677: f64 = (assign17770_e16675 / var_fmaxr);
        let assign17770_e16678: f64 = (assign17770_e16673 - assign17770_e16677);
        let assign17770_e16680: f64 = (assign17770_e16678 * 0.3333333333333333);
        let assign17770_e16681: f64 = (1.0 + assign17770_e16680);
        let assign17770_e16682: f64 = (assign17770_e16670 * assign17770_e16681);
        let assign17770_e16683: f64 = (0.5 * assign17770_e16682);
        let assign17770_e16684: f64 = (1.0 + assign17770_e16683);
        let assign17770_e16685: f64 = (assign17770_e16661 * assign17770_e16684);
        let assign17770_e16686: f64 = (1.0 + assign17770_e16685);
        let assign17770_e16687: f64 = (1e-100 / assign17770_e16686);
        (assign17770_e16687, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17770_e16658 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17770_e16667 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17770_e16675 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17770_e16658 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17770_e16667 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17770_e16675 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17770_e16658 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17770_e16667 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17770_e16675 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17770_e16658 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign17770_e16684) + (assign17770_e16661 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17770_e16667 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign17770_e16681) + (assign17770_e16670 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17770_e16675 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign17770_e16686 * assign17770_e16686))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17770_e16689;
        var_tmp_dn5 = assign17770_e16689_d_n5;
        var_tmp_dn6 = assign17770_e16689_d_n6;
        var_tmp_dn7 = assign17770_e16689_d_n7;
        var_tmp_dn8 = assign17770_e16689_d_n8;

        let (assign17780_e16738, assign17780_e16738_d_n5, assign17780_e16738_d_n6, assign17780_e16738_d_n7, assign17780_e16738_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) && (var_guard299 == 0.0)) && (var_guard300 == 0.0)) {
        let assign17780_e16708: f64 = (-var_fbbtgat);
        let assign17780_e16710: f64 = (assign17780_e16708 / var_fmaxr);
        let assign17780_e16712: f64 = (assign17780_e16710 - 230.25850929940458);
        let assign17780_e16716: f64 = (-var_fbbtgat);
        let assign17780_e16718: f64 = (assign17780_e16716 / var_fmaxr);
        let assign17780_e16720: f64 = (assign17780_e16718 - 230.25850929940458);
        let assign17780_e16723: f64 = (-var_fbbtgat);
        let assign17780_e16725: f64 = (assign17780_e16723 / var_fmaxr);
        let assign17780_e16727: f64 = (assign17780_e16725 - 230.25850929940458);
        let assign17780_e16729: f64 = (assign17780_e16727 * 0.3333333333333333);
        let assign17780_e16730: f64 = (1.0 + assign17780_e16729);
        let assign17780_e16731: f64 = (assign17780_e16720 * assign17780_e16730);
        let assign17780_e16732: f64 = (0.5 * assign17780_e16731);
        let assign17780_e16733: f64 = (1.0 + assign17780_e16732);
        let assign17780_e16734: f64 = (assign17780_e16712 * assign17780_e16733);
        let assign17780_e16735: f64 = (1.0 + assign17780_e16734);
        let assign17780_e16736: f64 = (1e100 * assign17780_e16735);
        (assign17780_e16736, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17780_e16708 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17780_e16716 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign17780_e16723 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17780_e16708 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17780_e16716 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign17780_e16723 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17780_e16708 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17780_e16716 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign17780_e16723 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17780_e16708 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign17780_e16733) + (assign17780_e16712 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17780_e16716 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign17780_e16730) + (assign17780_e16720 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign17780_e16723 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17780_e16738;
        var_tmp_dn5 = assign17780_e16738_d_n5;
        var_tmp_dn6 = assign17780_e16738_d_n6;
        var_tmp_dn7 = assign17780_e16738_d_n7;
        var_tmp_dn8 = assign17780_e16738_d_n8;

        let (assign17790_e16758, assign17790_e16758_d_n5, assign17790_e16758_d_n6, assign17790_e16758_d_n7, assign17790_e16758_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard297 == 0.0)) {
        let assign17790_e16751: f64 = (var_v1 * var_fmaxr);
        let assign17790_e16753: f64 = (assign17790_e16751 * var_fmaxr);
        let assign17790_e16755: f64 = (assign17790_e16753 * var_tmp);
        let assign17790_e16756: f64 = (p.p853 * assign17790_e16755);
        (assign17790_e16756, (p.p853 * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign17790_e16751 * var_fmaxr_dn5)) * var_tmp) + (assign17790_e16753 * var_tmp_dn5))), (p.p853 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign17790_e16751 * var_fmaxr_dn6)) * var_tmp) + (assign17790_e16753 * var_tmp_dn6))), (p.p853 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign17790_e16751 * var_fmaxr_dn7)) * var_tmp) + (assign17790_e16753 * var_tmp_dn7))), (p.p853 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign17790_e16751 * var_fmaxr_dn8)) * var_tmp) + (assign17790_e16753 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign17790_e16758;
        var_ibbt_dn5 = assign17790_e16758_d_n5;
        var_ibbt_dn6 = assign17790_e16758_d_n6;
        var_ibbt_dn7 = assign17790_e16758_d_n7;
        var_ibbt_dn8 = assign17790_e16758_d_n8;

        let assign17800_e16761: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        var_guard301 = assign17800_e16761;

        let (assign17810_e16772, assign17810_e16772_d_n5, assign17810_e16772_d_n6, assign17810_e16772_d_n7, assign17810_e16772_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard301 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17810_e16772;
        var_fbreakdown_dn5 = assign17810_e16772_d_n5;
        var_fbreakdown_dn6 = assign17810_e16772_d_n6;
        var_fbreakdown_dn7 = assign17810_e16772_d_n7;
        var_fbreakdown_dn8 = assign17810_e16772_d_n8;

        let assign17820_e16775: f64 = (-var_alphaav);
        let assign17820_e16777: f64 = (assign17820_e16775 * p.p862);
        let assign17820_e16778: f64 = if var_vav > assign17820_e16777 { 1.0 } else { 0.0 };
        var_guard302 = assign17820_e16778;

        let assign17830_e16781: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        var_guard303 = assign17830_e16781;

        let (assign17840_e16811, assign17840_e16811_d_n5, assign17840_e16811_d_n6, assign17840_e16811_d_n7, assign17840_e16811_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard301 == 0.0)) && (var_guard302 != 0.0)) && (var_guard303 != 0.0)) {
        let assign17840_e16797: f64 = (var_vav * var_vbrinvgat);
        let assign17840_e16800: f64 = (var_vav * var_vbrinvgat);
        let assign17840_e16801: f64 = (assign17840_e16797 * assign17840_e16800);
        let assign17840_e16804: f64 = (var_vav * var_vbrinvgat);
        let assign17840_e16805: f64 = (assign17840_e16801 * assign17840_e16804);
        let assign17840_e16808: f64 = (var_vav * var_vbrinvgat);
        let assign17840_e16809: f64 = (assign17840_e16805 * assign17840_e16808);
        (assign17840_e16809, (((((((var_vav * var_vbrinvgat_dn5) * assign17840_e16800) + (assign17840_e16797 * (var_vav * var_vbrinvgat_dn5))) * assign17840_e16804) + (assign17840_e16801 * (var_vav * var_vbrinvgat_dn5))) * assign17840_e16808) + (assign17840_e16805 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign17840_e16800) + (assign17840_e16797 * (var_vav * var_vbrinvgat_dn6))) * assign17840_e16804) + (assign17840_e16801 * (var_vav * var_vbrinvgat_dn6))) * assign17840_e16808) + (assign17840_e16805 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign17840_e16800) + (assign17840_e16797 * (var_vav * var_vbrinvgat_dn7))) * assign17840_e16804) + (assign17840_e16801 * (var_vav * var_vbrinvgat_dn7))) * assign17840_e16808) + (assign17840_e16805 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign17840_e16800) + (assign17840_e16797 * (var_vav * var_vbrinvgat_dn8))) * assign17840_e16804) + (assign17840_e16801 * (var_vav * var_vbrinvgat_dn8))) * assign17840_e16808) + (assign17840_e16805 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17840_e16811;
        var_tmp_dn5 = assign17840_e16811_d_n5;
        var_tmp_dn6 = assign17840_e16811_d_n6;
        var_tmp_dn7 = assign17840_e16811_d_n7;
        var_tmp_dn8 = assign17840_e16811_d_n8;

        let (assign17850_e16833, assign17850_e16833_d_n5, assign17850_e16833_d_n6, assign17850_e16833_d_n7, assign17850_e16833_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard301 == 0.0)) && (var_guard302 != 0.0)) && (var_guard303 == 0.0)) {
        let assign17850_e16828: f64 = (var_vav * var_vbrinvgat);
        let assign17850_e16829: f64 = (assign17850_e16828).abs();
        let assign17850_e16831: f64 = (assign17850_e16829).powf(p.p865);
        (assign17850_e16831, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign17850_e16829))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign17850_e16829).powf(p.p865 - 1.0) * if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign17850_e16831 * (p.p865 * (if assign17850_e16828 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign17850_e16829))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign17850_e16833;
        var_tmp_dn5 = assign17850_e16833_d_n5;
        var_tmp_dn6 = assign17850_e16833_d_n6;
        var_tmp_dn7 = assign17850_e16833_d_n7;
        var_tmp_dn8 = assign17850_e16833_d_n8;

        let (assign17860_e16851, assign17860_e16851_d_n5, assign17860_e16851_d_n6, assign17860_e16851_d_n7, assign17860_e16851_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard301 == 0.0)) && (var_guard302 != 0.0)) {
        let assign17860_e16848: f64 = (1.0 - var_tmp);
        let assign17860_e16849: f64 = (1.0 / assign17860_e16848);
        (assign17860_e16849, (-((-var_tmp_dn5) / (assign17860_e16848 * assign17860_e16848))), (-((-var_tmp_dn6) / (assign17860_e16848 * assign17860_e16848))), (-((-var_tmp_dn7) / (assign17860_e16848 * assign17860_e16848))), (-((-var_tmp_dn8) / (assign17860_e16848 * assign17860_e16848))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17860_e16851;
        var_fbreakdown_dn5 = assign17860_e16851_d_n5;
        var_fbreakdown_dn6 = assign17860_e16851_d_n6;
        var_fbreakdown_dn7 = assign17860_e16851_d_n7;
        var_fbreakdown_dn8 = assign17860_e16851_d_n8;

        let (assign17870_e16874, assign17870_e16874_d_n5, assign17870_e16874_d_n6, assign17870_e16874_d_n7, assign17870_e16874_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) && (var_guard301 == 0.0)) && (var_guard302 == 0.0)) {
        let assign17870_e16868: f64 = (var_alphaav * p.p862);
        let assign17870_e16869: f64 = (var_vav + assign17870_e16868);
        let assign17870_e16871: f64 = (assign17870_e16869 * var_slopegat);
        let assign17870_e16872: f64 = (var_fstopgat + assign17870_e16871);
        (assign17870_e16872, (assign17870_e16869 * var_slopegat_dn5), (assign17870_e16869 * var_slopegat_dn6), (assign17870_e16869 * var_slopegat_dn7), (assign17870_e16869 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign17870_e16874;
        var_fbreakdown_dn5 = assign17870_e16874_d_n5;
        var_fbreakdown_dn6 = assign17870_e16874_d_n6;
        var_fbreakdown_dn7 = assign17870_e16874_d_n7;
        var_fbreakdown_dn8 = assign17870_e16874_d_n8;

        let (assign17880_e16893, assign17880_e16893_d_n5, assign17880_e16893_d_n6, assign17880_e16893_d_n7, assign17880_e16893_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard287 == 0.0)) {
        let assign17880_e16884: f64 = (var_id__blk219 + var_isrh);
        let assign17880_e16886: f64 = (assign17880_e16884 + var_itat);
        let assign17880_e16888: f64 = (assign17880_e16886 + var_ibbt);
        let assign17880_e16889: f64 = (p.p29 * assign17880_e16888);
        let assign17880_e16891: f64 = (assign17880_e16889 * var_fbreakdown);
        (assign17880_e16891, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign17880_e16889 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign17880_e16889 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign17880_e16889 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign17880_e16889 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign17880_e16893;
        var_ijungat_dn5 = assign17880_e16893_d_n5;
        var_ijungat_dn6 = assign17880_e16893_d_n6;
        var_ijungat_dn7 = assign17880_e16893_d_n7;
        var_ijungat_dn8 = assign17880_e16893_d_n8;

        let (assign17890_e16909, assign17890_e16909_d_n5, assign17890_e16909_d_n6, assign17890_e16909_d_n7, assign17890_e16909_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign17890_e16899: f64 = (var_absource_i * var_ijunbot);
        let assign17890_e16902: f64 = (var_lssource_i * var_ijunsti);
        let assign17890_e16903: f64 = (assign17890_e16899 + assign17890_e16902);
        let assign17890_e16906: f64 = (var_lgsource_i * var_ijungat);
        let assign17890_e16907: f64 = (assign17890_e16903 + assign17890_e16906);
        (assign17890_e16907, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i1, var_i1_dn5, var_i1_dn6, var_i1_dn7, var_i1_dn8,)
    }
};
        var_i1 = assign17890_e16909;
        var_i1_dn5 = assign17890_e16909_d_n5;
        var_i1_dn6 = assign17890_e16909_d_n6;
        var_i1_dn7 = assign17890_e16909_d_n7;
        var_i1_dn8 = assign17890_e16909_d_n8;

        let (assign17900_e16915,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign17900_e16915;

        let (assign17910_e16921,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign17910_e16921;

        let assign17920_e16933: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard304 = assign17920_e16933;

        let assign18000_e17019: f64 = if var_v2 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard305 = assign18000_e17019;

        let assign18010_e17021: f64 = (-0.5);
        let assign18010_e17024: f64 = (var_v2 * var_phitdinv);
        let assign18010_e17025: f64 = (assign18010_e17021 * assign18010_e17024);
        let assign18010_e17026: f64 = (assign18010_e17025).abs();
        let assign18010_e17028: f64 = if assign18010_e17026 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard306 = assign18010_e17028;

        let (assign18020_e17046,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) && (var_guard306 != 0.0)) {
        let assign18020_e17039: f64 = (-0.5);
        let assign18020_e17042: f64 = (var_v2 * var_phitdinv);
        let assign18020_e17043: f64 = (assign18020_e17039 * assign18020_e17042);
        let assign18020_e17044: f64 = (assign18020_e17043).exp();
        (assign18020_e17044,)
    } else {
        (var_z,)
    }
};
        var_z = assign18020_e17046;

        let assign18030_e17048: f64 = (-0.5);
        let assign18030_e17051: f64 = (var_v2 * var_phitdinv);
        let assign18030_e17052: f64 = (assign18030_e17048 * assign18030_e17051);
        let assign18030_e17054: f64 = if assign18030_e17052 < 0.0 { 1.0 } else { 0.0 };
        var_guard307 = assign18030_e17054;

        let (assign18040_e17109,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) && (var_guard306 == 0.0)) && (var_guard307 != 0.0)) {
        let assign18040_e17070: f64 = (-230.25850929940458);
        let assign18040_e17072: f64 = (-0.5);
        let assign18040_e17075: f64 = (var_v2 * var_phitdinv);
        let assign18040_e17076: f64 = (assign18040_e17072 * assign18040_e17075);
        let assign18040_e17077: f64 = (assign18040_e17070 - assign18040_e17076);
        let assign18040_e17081: f64 = (-230.25850929940458);
        let assign18040_e17083: f64 = (-0.5);
        let assign18040_e17086: f64 = (var_v2 * var_phitdinv);
        let assign18040_e17087: f64 = (assign18040_e17083 * assign18040_e17086);
        let assign18040_e17088: f64 = (assign18040_e17081 - assign18040_e17087);
        let assign18040_e17091: f64 = (-230.25850929940458);
        let assign18040_e17093: f64 = (-0.5);
        let assign18040_e17096: f64 = (var_v2 * var_phitdinv);
        let assign18040_e17097: f64 = (assign18040_e17093 * assign18040_e17096);
        let assign18040_e17098: f64 = (assign18040_e17091 - assign18040_e17097);
        let assign18040_e17100: f64 = (assign18040_e17098 * 0.3333333333333333);
        let assign18040_e17101: f64 = (1.0 + assign18040_e17100);
        let assign18040_e17102: f64 = (assign18040_e17088 * assign18040_e17101);
        let assign18040_e17103: f64 = (0.5 * assign18040_e17102);
        let assign18040_e17104: f64 = (1.0 + assign18040_e17103);
        let assign18040_e17105: f64 = (assign18040_e17077 * assign18040_e17104);
        let assign18040_e17106: f64 = (1.0 + assign18040_e17105);
        let assign18040_e17107: f64 = (1e-100 / assign18040_e17106);
        (assign18040_e17107,)
    } else {
        (var_z,)
    }
};
        var_z = assign18040_e17109;

        let (assign18050_e17162,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) && (var_guard306 == 0.0)) && (var_guard307 == 0.0)) {
        let assign18050_e17126: f64 = (-0.5);
        let assign18050_e17129: f64 = (var_v2 * var_phitdinv);
        let assign18050_e17130: f64 = (assign18050_e17126 * assign18050_e17129);
        let assign18050_e17132: f64 = (assign18050_e17130 - 230.25850929940458);
        let assign18050_e17136: f64 = (-0.5);
        let assign18050_e17139: f64 = (var_v2 * var_phitdinv);
        let assign18050_e17140: f64 = (assign18050_e17136 * assign18050_e17139);
        let assign18050_e17142: f64 = (assign18050_e17140 - 230.25850929940458);
        let assign18050_e17145: f64 = (-0.5);
        let assign18050_e17148: f64 = (var_v2 * var_phitdinv);
        let assign18050_e17149: f64 = (assign18050_e17145 * assign18050_e17148);
        let assign18050_e17151: f64 = (assign18050_e17149 - 230.25850929940458);
        let assign18050_e17153: f64 = (assign18050_e17151 * 0.3333333333333333);
        let assign18050_e17154: f64 = (1.0 + assign18050_e17153);
        let assign18050_e17155: f64 = (assign18050_e17142 * assign18050_e17154);
        let assign18050_e17156: f64 = (0.5 * assign18050_e17155);
        let assign18050_e17157: f64 = (1.0 + assign18050_e17156);
        let assign18050_e17158: f64 = (assign18050_e17132 * assign18050_e17157);
        let assign18050_e17159: f64 = (1.0 + assign18050_e17158);
        let assign18050_e17160: f64 = (1e100 * assign18050_e17159);
        (assign18050_e17160,)
    } else {
        (var_z,)
    }
};
        var_z = assign18050_e17162;

        let (assign18060_e17174,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign18060_e17172: f64 = (1.0 / var_z);
        (assign18060_e17172,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign18060_e17174;

        let (assign18070_e17186,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign18070_e17184: f64 = (var_zinv * var_zinv);
        (assign18070_e17184,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign18070_e17186;

        let (assign18080_e17205,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign18080_e17198: f64 = (var_v2 - var_vmax_s);
        let assign18080_e17200: f64 = (assign18080_e17198 * var_phitdinv);
        let assign18080_e17201: f64 = (1.0 + assign18080_e17200);
        let assign18080_e17203: f64 = (assign18080_e17201 * var_exp_vmax_over_phitd_s);
        (assign18080_e17203,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign18080_e17205;

        let (assign18090_e17217,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign18090_e17215: f64 = (var_idmult).sqrt();
        (assign18090_e17215,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign18090_e17217;

        let (assign18100_e17230,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign18100_e17228: f64 = (1.0 / var_zinv);
        (assign18100_e17228,)
    } else {
        (var_z,)
    }
};
        var_z = assign18100_e17230;

        let (assign18110_e17240,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) {
        let assign18110_e17238: f64 = (var_idmult - 1.0);
        (assign18110_e17238,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign18110_e17240;

        let assign18120_e17243: f64 = if var_v2 > 0.0 { 1.0 } else { 0.0 };
        var_guard308 = assign18120_e17243;

        let (assign18130_e17269,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18130_e17255: f64 = (2.0 + var_z);
        let assign18130_e17258: f64 = (var_z + 1.0);
        let assign18130_e17261: f64 = (var_z + 3.0);
        let assign18130_e17262: f64 = (assign18130_e17258 * assign18130_e17261);
        let assign18130_e17263: f64 = (assign18130_e17262).sqrt();
        let assign18130_e17264: f64 = (assign18130_e17255 + assign18130_e17263);
        let assign18130_e17265: f64 = (assign18130_e17264).ln();
        let assign18130_e17266: f64 = (var_phitd * assign18130_e17265);
        let assign18130_e17267: f64 = (2.0 * assign18130_e17266);
        (assign18130_e17267,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign18130_e17269;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_guard297_slot = var_guard297;
        *var_guard298_slot = var_guard298;
        *var_guard299_slot = var_guard299;
        *var_guard300_slot = var_guard300;
        *var_guard301_slot = var_guard301;
        *var_guard302_slot = var_guard302;
        *var_guard303_slot = var_guard303;
        *var_guard304_slot = var_guard304;
        *var_guard305_slot = var_guard305;
        *var_guard306_slot = var_guard306;
        *var_guard307_slot = var_guard307;
        *var_guard308_slot = var_guard308;
        *var_i1_slot = var_i1;
        *var_i1_dn5_slot = var_i1_dn5;
        *var_i1_dn6_slot = var_i1_dn6;
        *var_i1_dn7_slot = var_i1_dn7;
        *var_i1_dn8_slot = var_i1_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard304: f64,
        var_guard308: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_v2: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbirbotinv: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard309_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard309: f64 = *var_guard309_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;

        let (assign18140_e17303,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) && (var_guard308 == 0.0)) {
        let assign18140_e17279: f64 = (-var_v2);
        let assign18140_e17284: f64 = (2.0 * var_zinv);
        let assign18140_e17286: f64 = (assign18140_e17284 + 1.0);
        let assign18140_e17289: f64 = (1.0 + var_zinv);
        let assign18140_e17293: f64 = (3.0 * var_zinv);
        let assign18140_e17294: f64 = (1.0 + assign18140_e17293);
        let assign18140_e17295: f64 = (assign18140_e17289 * assign18140_e17294);
        let assign18140_e17296: f64 = (assign18140_e17295).sqrt();
        let assign18140_e17297: f64 = (assign18140_e17286 + assign18140_e17296);
        let assign18140_e17298: f64 = (assign18140_e17297).ln();
        let assign18140_e17299: f64 = (var_phitd * assign18140_e17298);
        let assign18140_e17300: f64 = (2.0 * assign18140_e17299);
        let assign18140_e17301: f64 = (assign18140_e17279 + assign18140_e17300);
        (assign18140_e17301,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign18140_e17303;

        let (assign18150_e17313,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) {
        let assign18150_e17311: f64 = (var_vbimin_s - var_two_psistar);
        (assign18150_e17311,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign18150_e17313;

        let (assign18160_e17340,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) {
        let assign18160_e17322: f64 = (var_v2 + var_vjlim);
        let assign18160_e17325: f64 = (var_v2 - var_vjlim);
        let assign18160_e17328: f64 = (var_v2 - var_vjlim);
        let assign18160_e17329: f64 = (assign18160_e17325 * assign18160_e17328);
        let assign18160_e17332: f64 = (4.0 * var_phitd);
        let assign18160_e17334: f64 = (assign18160_e17332 * var_phitd);
        let assign18160_e17335: f64 = (assign18160_e17329 + assign18160_e17334);
        let assign18160_e17336: f64 = (assign18160_e17335).sqrt();
        let assign18160_e17337: f64 = (assign18160_e17322 - assign18160_e17336);
        let assign18160_e17338: f64 = (0.5 * assign18160_e17337);
        (assign18160_e17338,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign18160_e17340;

        let (assign18170_e17367,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) {
        let assign18170_e17349: f64 = (var_v2 + var_vbbtlim_s);
        let assign18170_e17352: f64 = (var_v2 - var_vbbtlim_s);
        let assign18170_e17355: f64 = (var_v2 - var_vbbtlim_s);
        let assign18170_e17356: f64 = (assign18170_e17352 * assign18170_e17355);
        let assign18170_e17359: f64 = (4.0 * var_phitr);
        let assign18170_e17361: f64 = (assign18170_e17359 * var_phitr);
        let assign18170_e17362: f64 = (assign18170_e17356 + assign18170_e17361);
        let assign18170_e17363: f64 = (assign18170_e17362).sqrt();
        let assign18170_e17364: f64 = (assign18170_e17349 - assign18170_e17363);
        let assign18170_e17365: f64 = (0.5 * assign18170_e17364);
        (assign18170_e17365,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign18170_e17367;

        let (assign18180_e17394,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard304 != 0.0)) {
        let assign18180_e17376: f64 = var_v2;
        let assign18180_e17379: f64 = var_v2;
        let assign18180_e17382: f64 = var_v2;
        let assign18180_e17383: f64 = (assign18180_e17379 * assign18180_e17382);
        let assign18180_e17386: f64 = (4.0 * 1e-6);
        let assign18180_e17388: f64 = (assign18180_e17386 * 1e-6);
        let assign18180_e17389: f64 = (assign18180_e17383 + assign18180_e17388);
        let assign18180_e17390: f64 = (assign18180_e17389).sqrt();
        let assign18180_e17391: f64 = (assign18180_e17376 - assign18180_e17390);
        let assign18180_e17392: f64 = (0.5 * assign18180_e17391);
        (assign18180_e17392,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign18180_e17394;

        let assign18190_e17397: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard309 = assign18190_e17397;

        let (assign18200_e17405, assign18200_e17405_d_n5, assign18200_e17405_d_n6, assign18200_e17405_d_n7, assign18200_e17405_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign18200_e17405;
        var_ijunbot_dn5 = assign18200_e17405_d_n5;
        var_ijunbot_dn6 = assign18200_e17405_d_n6;
        var_ijunbot_dn7 = assign18200_e17405_d_n7;
        var_ijunbot_dn8 = assign18200_e17405_d_n8;

        let (assign18210_e17416,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) {
        let assign18210_e17414: f64 = (var_idsatbot * var_idmult);
        (assign18210_e17414,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign18210_e17416;

        let assign18220_e17423: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        var_guard310 = assign18220_e17423;

        let (assign18230_e17434, assign18230_e17434_d_n5, assign18230_e17434_d_n6, assign18230_e17434_d_n7, assign18230_e17434_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign18230_e17434;
        var_isrh_dn5 = assign18230_e17434_d_n5;
        var_isrh_dn6 = assign18230_e17434_d_n6;
        var_isrh_dn7 = assign18230_e17434_d_n7;
        var_isrh_dn8 = assign18230_e17434_d_n8;

        let (assign18240_e17448,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18240_e17446: f64 = (var_vbibot - var_vjsrh);
        (assign18240_e17446,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign18240_e17448;

        let (assign18250_e17467,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18250_e17462: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign18250_e17463: f64 = (1.0 - assign18250_e17462);
        let assign18250_e17464: f64 = (assign18250_e17463).sqrt();
        let assign18250_e17465: f64 = (1.0 - assign18250_e17464);
        (assign18250_e17465,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign18250_e17467;

        let assign18260_e17470: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard311 = assign18260_e17470;

        let (assign18270_e17484,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) && (var_guard311 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18270_e17484;

        let (assign18280_e17516,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) && (var_guard311 == 0.0)) {
        let assign18280_e17499: f64 = (var_wsrhstep * var_wsrhstep);
        let assign18280_e17501: f64 = (var_wsrhstep).ln();
        let assign18280_e17502: f64 = (assign18280_e17499 * assign18280_e17501);
        let assign18280_e17505: f64 = (1.0 - var_wsrhstep);
        let assign18280_e17506: f64 = (assign18280_e17502 / assign18280_e17505);
        let assign18280_e17508: f64 = (assign18280_e17506 + var_wsrhstep);
        let assign18280_e17512: f64 = (2.0 * p.p831);
        let assign18280_e17513: f64 = (1.0 - assign18280_e17512);
        let assign18280_e17514: f64 = (assign18280_e17508 * assign18280_e17513);
        (assign18280_e17514,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18280_e17516;

        let (assign18290_e17530,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18290_e17528: f64 = (var_wsrhstep + var_dwsrh);
        (assign18290_e17528,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign18290_e17530;

        let assign18300_e17533: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard312 = assign18300_e17533;

        let (assign18310_e17550, assign18310_e17550_d_n5, assign18310_e17550_d_n6, assign18310_e17550_d_n7, assign18310_e17550_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) && (var_guard312 != 0.0)) {
        let assign18310_e17547: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign18310_e17548: f64 = (assign18310_e17547).sqrt();
        (assign18310_e17548, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18310_e17550;
        var_tmp_dn5 = assign18310_e17550_d_n5;
        var_tmp_dn6 = assign18310_e17550_d_n6;
        var_tmp_dn7 = assign18310_e17550_d_n7;
        var_tmp_dn8 = assign18310_e17550_d_n8;

        let (assign18320_e17569, assign18320_e17569_d_n5, assign18320_e17569_d_n6, assign18320_e17569_d_n7, assign18320_e17569_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) && (var_guard312 == 0.0)) {
        let assign18320_e17565: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign18320_e17567: f64 = (assign18320_e17565).powf(p.p831);
        (assign18320_e17567, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18320_e17569;
        var_tmp_dn5 = assign18320_e17569_d_n5;
        var_tmp_dn6 = assign18320_e17569_d_n6;
        var_tmp_dn7 = assign18320_e17569_d_n7;
        var_tmp_dn8 = assign18320_e17569_d_n8;

        let (assign18330_e17583, assign18330_e17583_d_n5, assign18330_e17583_d_n6, assign18330_e17583_d_n7, assign18330_e17583_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18330_e17581: f64 = (var_wdepnulrbot * var_tmp);
        (assign18330_e17581, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign18330_e17583;
        var_wdep_dn5 = assign18330_e17583_d_n5;
        var_wdep_dn6 = assign18330_e17583_d_n6;
        var_wdep_dn7 = assign18330_e17583_d_n7;
        var_wdep_dn8 = assign18330_e17583_d_n8;

        let (assign18340_e17601, assign18340_e17601_d_n5, assign18340_e17601_d_n6, assign18340_e17601_d_n7, assign18340_e17601_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18340_e17596: f64 = (var_zinv - 1.0);
        let assign18340_e17598: f64 = (assign18340_e17596 * var_wdep);
        let assign18340_e17599: f64 = (var_ftdbot * assign18340_e17598);
        (assign18340_e17599, (var_ftdbot * (assign18340_e17596 * var_wdep_dn5)), (var_ftdbot * (assign18340_e17596 * var_wdep_dn6)), (var_ftdbot * (assign18340_e17596 * var_wdep_dn7)), (var_ftdbot * (assign18340_e17596 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign18340_e17601;
        var_asrh_dn5 = assign18340_e17601_d_n5;
        var_asrh_dn6 = assign18340_e17601_d_n6;
        var_asrh_dn7 = assign18340_e17601_d_n7;
        var_asrh_dn8 = assign18340_e17601_d_n8;

        let (assign18350_e17617, assign18350_e17617_d_n5, assign18350_e17617_d_n6, assign18350_e17617_d_n7, assign18350_e17617_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard310 == 0.0)) {
        let assign18350_e17614: f64 = (var_asrh * var_wsrh);
        let assign18350_e17615: f64 = (p.p840 * assign18350_e17614);
        (assign18350_e17615, (p.p840 * (var_asrh_dn5 * var_wsrh)), (p.p840 * (var_asrh_dn6 * var_wsrh)), (p.p840 * (var_asrh_dn7 * var_wsrh)), (p.p840 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign18350_e17617;
        var_isrh_dn5 = assign18350_e17617_d_n5;
        var_isrh_dn6 = assign18350_e17617_d_n6;
        var_isrh_dn7 = assign18350_e17617_d_n7;
        var_isrh_dn8 = assign18350_e17617_d_n8;

        let assign18360_e17620: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard313 = assign18360_e17620;

        let (assign18370_e17631, assign18370_e17631_d_n5, assign18370_e17631_d_n6, assign18370_e17631_d_n7, assign18370_e17631_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign18370_e17631;
        var_itat_dn5 = assign18370_e17631_d_n5;
        var_itat_dn6 = assign18370_e17631_d_n6;
        var_itat_dn7 = assign18370_e17631_d_n7;
        var_itat_dn8 = assign18370_e17631_d_n8;

        let (assign18380_e17649, assign18380_e17649_d_n5, assign18380_e17649_d_n6, assign18380_e17649_d_n7, assign18380_e17649_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18380_e17644: f64 = (var_wdep * var_one_minus_pbot);
        let assign18380_e17646: f64 = (assign18380_e17644 / var_vbi_minus_vjsrh);
        let assign18380_e17647: f64 = (var_btatpartbot * assign18380_e17646);
        (assign18380_e17647, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign18380_e17649;
        var_btat_dn5 = assign18380_e17649_d_n5;
        var_btat_dn6 = assign18380_e17649_d_n6;
        var_btat_dn7 = assign18380_e17649_d_n7;
        var_btat_dn8 = assign18380_e17649_d_n8;

        let (assign18390_e17665, assign18390_e17665_d_n5, assign18390_e17665_d_n6, assign18390_e17665_d_n7, assign18390_e17665_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18390_e17661: f64 = (0.666666666666667 * var_atatbot);
        let assign18390_e17663: f64 = (assign18390_e17661 / var_btat);
        (assign18390_e17663, (-((assign18390_e17661 * var_btat_dn5) / (var_btat * var_btat))), (-((assign18390_e17661 * var_btat_dn6) / (var_btat * var_btat))), (-((assign18390_e17661 * var_btat_dn7) / (var_btat * var_btat))), (-((assign18390_e17661 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign18390_e17665;
        var_twoatatoverthreebtat_dn5 = assign18390_e17665_d_n5;
        var_twoatatoverthreebtat_dn6 = assign18390_e17665_d_n6;
        var_twoatatoverthreebtat_dn7 = assign18390_e17665_d_n7;
        var_twoatatoverthreebtat_dn8 = assign18390_e17665_d_n8;

        let (assign18400_e17679, assign18400_e17679_d_n5, assign18400_e17679_d_n6, assign18400_e17679_d_n7, assign18400_e17679_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18400_e17677: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign18400_e17677, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign18400_e17679;
        var_umaxbeforelimiting_dn5 = assign18400_e17679_d_n5;
        var_umaxbeforelimiting_dn6 = assign18400_e17679_d_n6;
        var_umaxbeforelimiting_dn7 = assign18400_e17679_d_n7;
        var_umaxbeforelimiting_dn8 = assign18400_e17679_d_n8;

        let (assign18410_e17700, assign18410_e17700_d_n5, assign18410_e17700_d_n6, assign18410_e17700_d_n7, assign18410_e17700_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18410_e17691: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18410_e17694: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18410_e17696: f64 = (assign18410_e17694 + 1.0);
        let assign18410_e17697: f64 = (assign18410_e17691 / assign18410_e17696);
        let assign18410_e17698: f64 = (assign18410_e17697).sqrt();
        (assign18410_e17698, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign18410_e17696) - (assign18410_e17691 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign18410_e17696) - (assign18410_e17691 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign18410_e17696) - (assign18410_e17691 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign18410_e17696) - (assign18410_e17691 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign18410_e17696 * assign18410_e17696)) / (2.0 * assign18410_e17698)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign18410_e17700;
        var_umax_dn5 = assign18410_e17700_d_n5;
        var_umax_dn6 = assign18410_e17700_d_n6;
        var_umax_dn7 = assign18410_e17700_d_n7;
        var_umax_dn8 = assign18410_e17700_d_n8;

        let (assign18420_e17713, assign18420_e17713_d_n5, assign18420_e17713_d_n6, assign18420_e17713_d_n7, assign18420_e17713_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18420_e17711: f64 = (var_umax).sqrt();
        (assign18420_e17711, (var_umax_dn5 / (2.0 * assign18420_e17711)), (var_umax_dn6 / (2.0 * assign18420_e17711)), (var_umax_dn7 / (2.0 * assign18420_e17711)), (var_umax_dn8 / (2.0 * assign18420_e17711)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign18420_e17713;
        var_sqrtumax_dn5 = assign18420_e17713_d_n5;
        var_sqrtumax_dn6 = assign18420_e17713_d_n6;
        var_sqrtumax_dn7 = assign18420_e17713_d_n7;
        var_sqrtumax_dn8 = assign18420_e17713_d_n8;

        let (assign18430_e17727, assign18430_e17727_d_n5, assign18430_e17727_d_n6, assign18430_e17727_d_n7, assign18430_e17727_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18430_e17725: f64 = (var_umax * var_sqrtumax);
        (assign18430_e17725, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign18430_e17727;
        var_umaxpoweronepointfive_dn5 = assign18430_e17727_d_n5;
        var_umaxpoweronepointfive_dn6 = assign18430_e17727_d_n6;
        var_umaxpoweronepointfive_dn7 = assign18430_e17727_d_n7;
        var_umaxpoweronepointfive_dn8 = assign18430_e17727_d_n8;

        let assign18440_e17729: f64 = (-p.p831);
        let assign18440_e17731: f64 = (assign18440_e17729 * var_one_over_one_minus_pbot);
        let assign18440_e17733: f64 = (-1.0);
        let assign18440_e17734: f64 = if assign18440_e17731 == assign18440_e17733 { 1.0 } else { 0.0 };
        var_guard314 = assign18440_e17734;

        let (assign18450_e17754, assign18450_e17754_d_n5, assign18450_e17754_d_n6, assign18450_e17754_d_n7, assign18450_e17754_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard314 != 0.0)) {
        let assign18450_e17750: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18450_e17751: f64 = (1.0 + assign18450_e17750);
        let assign18450_e17752: f64 = (1.0 / assign18450_e17751);
        (assign18450_e17752, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign18450_e17751 * assign18450_e17751))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign18450_e17751 * assign18450_e17751))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign18450_e17751 * assign18450_e17751))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign18450_e17751 * assign18450_e17751))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign18450_e17754;
        var_wgamma_dn5 = assign18450_e17754_d_n5;
        var_wgamma_dn6 = assign18450_e17754_d_n6;
        var_wgamma_dn7 = assign18450_e17754_d_n7;
        var_wgamma_dn8 = assign18450_e17754_d_n8;

        let (assign18460_e17778, assign18460_e17778_d_n5, assign18460_e17778_d_n6, assign18460_e17778_d_n7, assign18460_e17778_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard314 == 0.0)) {
        let assign18460_e17770: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18460_e17771: f64 = (1.0 + assign18460_e17770);
        let assign18460_e17773: f64 = (-p.p831);
        let assign18460_e17775: f64 = (assign18460_e17773 * var_one_over_one_minus_pbot);
        let assign18460_e17776: f64 = (assign18460_e17771).powf(assign18460_e17775);
        (assign18460_e17776, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign18460_e17771))) }, if 0.0 == 0.0 && ((assign18460_e17775) as f64).is_finite() && ((assign18460_e17775) as f64).fract() == 0.0 { if assign18460_e17775 == 0.0 { 0.0 } else { (assign18460_e17775 * ((assign18460_e17771).powf(assign18460_e17775 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign18460_e17776 * (assign18460_e17775 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign18460_e17771))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign18460_e17778;
        var_wgamma_dn5 = assign18460_e17778_d_n5;
        var_wgamma_dn6 = assign18460_e17778_d_n6;
        var_wgamma_dn7 = assign18460_e17778_d_n7;
        var_wgamma_dn8 = assign18460_e17778_d_n8;

        let (assign18470_e17796, assign18470_e17796_d_n5, assign18470_e17796_d_n6, assign18470_e17796_d_n7, assign18470_e17796_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18470_e17790: f64 = (var_wsrh * var_wgamma);
        let assign18470_e17793: f64 = (var_wsrh + var_wgamma);
        let assign18470_e17794: f64 = (assign18470_e17790 / assign18470_e17793);
        (assign18470_e17794, ((((var_wsrh * var_wgamma_dn5) * assign18470_e17793) - (assign18470_e17790 * var_wgamma_dn5)) / (assign18470_e17793 * assign18470_e17793)), ((((var_wsrh * var_wgamma_dn6) * assign18470_e17793) - (assign18470_e17790 * var_wgamma_dn6)) / (assign18470_e17793 * assign18470_e17793)), ((((var_wsrh * var_wgamma_dn7) * assign18470_e17793) - (assign18470_e17790 * var_wgamma_dn7)) / (assign18470_e17793 * assign18470_e17793)), ((((var_wsrh * var_wgamma_dn8) * assign18470_e17793) - (assign18470_e17790 * var_wgamma_dn8)) / (assign18470_e17793 * assign18470_e17793)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign18470_e17796;
        var_wtat_dn5 = assign18470_e17796_d_n5;
        var_wtat_dn6 = assign18470_e17796_d_n6;
        var_wtat_dn7 = assign18470_e17796_d_n7;
        var_wtat_dn8 = assign18470_e17796_d_n8;

        let (assign18480_e17813, assign18480_e17813_d_n5, assign18480_e17813_d_n6, assign18480_e17813_d_n7, assign18480_e17813_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18480_e17809: f64 = (var_btat / var_sqrtumax);
        let assign18480_e17810: f64 = (0.375 * assign18480_e17809);
        let assign18480_e17811: f64 = (assign18480_e17810).sqrt();
        (assign18480_e17811, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18480_e17811)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18480_e17811)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign18480_e17813;
        var_ktat_dn5 = assign18480_e17813_d_n5;
        var_ktat_dn6 = assign18480_e17813_d_n6;
        var_ktat_dn7 = assign18480_e17813_d_n7;
        var_ktat_dn8 = assign18480_e17813_d_n8;

        let (assign18490_e17831, assign18490_e17831_d_n5, assign18490_e17831_d_n6, assign18490_e17831_d_n7, assign18490_e17831_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18490_e17826: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign18490_e17827: f64 = (2.0 * assign18490_e17826);
        let assign18490_e17829: f64 = (assign18490_e17827 - var_umax);
        (assign18490_e17829, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign18490_e17831;
        var_ltat_dn5 = assign18490_e17831_d_n5;
        var_ltat_dn6 = assign18490_e17831_d_n6;
        var_ltat_dn7 = assign18490_e17831_d_n7;
        var_ltat_dn8 = assign18490_e17831_d_n8;

        let (assign18500_e17857, assign18500_e17857_d_n5, assign18500_e17857_d_n6, assign18500_e17857_d_n7, assign18500_e17857_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18500_e17843: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign18500_e17845: f64 = (assign18500_e17843 * var_sqrtumax);
        let assign18500_e17848: f64 = (var_atatbot * var_umax);
        let assign18500_e17849: f64 = (assign18500_e17845 - assign18500_e17848);
        let assign18500_e17853: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18500_e17854: f64 = (0.5 * assign18500_e17853);
        let assign18500_e17855: f64 = (assign18500_e17849 + assign18500_e17854);
        (assign18500_e17855, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign18500_e17843 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign18500_e17843 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign18500_e17843 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign18500_e17843 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign18500_e17857;
        var_mtat_dn5 = assign18500_e17857_d_n5;
        var_mtat_dn6 = assign18500_e17857_d_n6;
        var_mtat_dn7 = assign18500_e17857_d_n7;
        var_mtat_dn8 = assign18500_e17857_d_n8;

        let (assign18510_e17873, assign18510_e17873_d_n5, assign18510_e17873_d_n6, assign18510_e17873_d_n7, assign18510_e17873_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18510_e17869: f64 = (var_ltat - 1.0);
        let assign18510_e17871: f64 = (assign18510_e17869 * var_ktat);
        (assign18510_e17871, ((var_ltat_dn5 * var_ktat) + (assign18510_e17869 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign18510_e17869 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign18510_e17869 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign18510_e17869 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign18510_e17873;
        var_xerfc_dn5 = assign18510_e17873_d_n5;
        var_xerfc_dn6 = assign18510_e17873_d_n6;
        var_xerfc_dn7 = assign18510_e17873_d_n7;
        var_xerfc_dn8 = assign18510_e17873_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard309_slot = var_guard309;
        *var_guard310_slot = var_guard310;
        *var_guard311_slot = var_guard311;
        *var_guard312_slot = var_guard312;
        *var_guard313_slot = var_guard313;
        *var_guard314_slot = var_guard314;
        *var_id__blk219_slot = var_id__blk219;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard309: f64,
        var_guard313: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_slopebot: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbrinvbot: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_xerfc: f64,
        var_xerfc_dn5: f64,
        var_xerfc_dn6: f64,
        var_xerfc_dn7: f64,
        var_xerfc_dn8: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn5_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn5_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn5_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_guard315_slot: &mut f64,
        var_guard316_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_guard320_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_guard324_slot: &mut f64,
        var_guard325_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn5: f64 = *var_erfcpos_dn5_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn5: f64 = *var_erfctimesexpmtat_dn5_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn5: f64 = *var_gammamax_dn5_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_guard315: f64 = *var_guard315_slot;
        let mut var_guard316: f64 = *var_guard316_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign18520_e17887, assign18520_e17887_d_n5, assign18520_e17887_d_n6, assign18520_e17887_d_n7, assign18520_e17887_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18520_e17885: f64 = (var_xerfc * var_xerfc);
        (assign18520_e17885, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign18520_e17887;
        var_ysq_dn5 = assign18520_e17887_d_n5;
        var_ysq_dn6 = assign18520_e17887_d_n6;
        var_ysq_dn7 = assign18520_e17887_d_n7;
        var_ysq_dn8 = assign18520_e17887_d_n8;

        let assign18530_e17890: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard315 = assign18530_e17890;

        let (assign18540_e17910, assign18540_e17910_d_n5, assign18540_e17910_d_n6, assign18540_e17910_d_n7, assign18540_e17910_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard315 != 0.0)) {
        let assign18540_e17906: f64 = (var_perfc * var_xerfc);
        let assign18540_e17907: f64 = (1.0 + assign18540_e17906);
        let assign18540_e17908: f64 = (1.0 / assign18540_e17907);
        (assign18540_e17908, (-((var_perfc * var_xerfc_dn5) / (assign18540_e17907 * assign18540_e17907))), (-((var_perfc * var_xerfc_dn6) / (assign18540_e17907 * assign18540_e17907))), (-((var_perfc * var_xerfc_dn7) / (assign18540_e17907 * assign18540_e17907))), (-((var_perfc * var_xerfc_dn8) / (assign18540_e17907 * assign18540_e17907))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign18540_e17910;
        var_terfc_dn5 = assign18540_e17910_d_n5;
        var_terfc_dn6 = assign18540_e17910_d_n6;
        var_terfc_dn7 = assign18540_e17910_d_n7;
        var_terfc_dn8 = assign18540_e17910_d_n8;

        let (assign18550_e17931, assign18550_e17931_d_n5, assign18550_e17931_d_n6, assign18550_e17931_d_n7, assign18550_e17931_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard315 == 0.0)) {
        let assign18550_e17927: f64 = (var_perfc * var_xerfc);
        let assign18550_e17928: f64 = (1.0 - assign18550_e17927);
        let assign18550_e17929: f64 = (1.0 / assign18550_e17928);
        (assign18550_e17929, (-((-(var_perfc * var_xerfc_dn5)) / (assign18550_e17928 * assign18550_e17928))), (-((-(var_perfc * var_xerfc_dn6)) / (assign18550_e17928 * assign18550_e17928))), (-((-(var_perfc * var_xerfc_dn7)) / (assign18550_e17928 * assign18550_e17928))), (-((-(var_perfc * var_xerfc_dn8)) / (assign18550_e17928 * assign18550_e17928))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign18550_e17931;
        var_terfc_dn5 = assign18550_e17931_d_n5;
        var_terfc_dn6 = assign18550_e17931_d_n6;
        var_terfc_dn7 = assign18550_e17931_d_n7;
        var_terfc_dn8 = assign18550_e17931_d_n8;

        let assign18560_e17933: f64 = (-var_ysq);
        let assign18560_e17935: f64 = (assign18560_e17933 + var_mtat);
        let assign18560_e17937: f64 = (-230.25850929940458);
        let assign18560_e17938: f64 = if assign18560_e17935 > assign18560_e17937 { 1.0 } else { 0.0 };
        var_guard316 = assign18560_e17938;

        let (assign18570_e17956, assign18570_e17956_d_n5, assign18570_e17956_d_n6, assign18570_e17956_d_n7, assign18570_e17956_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard316 != 0.0)) {
        let assign18570_e17951: f64 = (-var_ysq);
        let assign18570_e17953: f64 = (assign18570_e17951 + var_mtat);
        let assign18570_e17954: f64 = (assign18570_e17953).exp();
        (assign18570_e17954, (assign18570_e17954 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign18570_e17954 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign18570_e17954 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign18570_e17954 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18570_e17956;
        var_tmp_dn5 = assign18570_e17956_d_n5;
        var_tmp_dn6 = assign18570_e17956_d_n6;
        var_tmp_dn7 = assign18570_e17956_d_n7;
        var_tmp_dn8 = assign18570_e17956_d_n8;

        let (assign18580_e18005, assign18580_e18005_d_n5, assign18580_e18005_d_n6, assign18580_e18005_d_n7, assign18580_e18005_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard316 == 0.0)) {
        let assign18580_e17972: f64 = (-230.25850929940458);
        let assign18580_e17974: f64 = (-var_ysq);
        let assign18580_e17976: f64 = (assign18580_e17974 + var_mtat);
        let assign18580_e17977: f64 = (assign18580_e17972 - assign18580_e17976);
        let assign18580_e17981: f64 = (-230.25850929940458);
        let assign18580_e17983: f64 = (-var_ysq);
        let assign18580_e17985: f64 = (assign18580_e17983 + var_mtat);
        let assign18580_e17986: f64 = (assign18580_e17981 - assign18580_e17985);
        let assign18580_e17989: f64 = (-230.25850929940458);
        let assign18580_e17991: f64 = (-var_ysq);
        let assign18580_e17993: f64 = (assign18580_e17991 + var_mtat);
        let assign18580_e17994: f64 = (assign18580_e17989 - assign18580_e17993);
        let assign18580_e17996: f64 = (assign18580_e17994 * 0.3333333333333333);
        let assign18580_e17997: f64 = (1.0 + assign18580_e17996);
        let assign18580_e17998: f64 = (assign18580_e17986 * assign18580_e17997);
        let assign18580_e17999: f64 = (0.5 * assign18580_e17998);
        let assign18580_e18000: f64 = (1.0 + assign18580_e17999);
        let assign18580_e18001: f64 = (assign18580_e17977 * assign18580_e18000);
        let assign18580_e18002: f64 = (1.0 + assign18580_e18001);
        let assign18580_e18003: f64 = (1e-100 / assign18580_e18002);
        (assign18580_e18003, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign18580_e17997) + (assign18580_e17986 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18580_e17997) + (assign18580_e17986 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18580_e17997) + (assign18580_e17986 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18580_e18000) + (assign18580_e17977 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18580_e17997) + (assign18580_e17986 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign18580_e18002 * assign18580_e18002))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18580_e18005;
        var_tmp_dn5 = assign18580_e18005_d_n5;
        var_tmp_dn6 = assign18580_e18005_d_n6;
        var_tmp_dn7 = assign18580_e18005_d_n7;
        var_tmp_dn8 = assign18580_e18005_d_n8;

        let (assign18590_e18035, assign18590_e18035_d_n5, assign18590_e18035_d_n6, assign18590_e18035_d_n7, assign18590_e18035_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18590_e18017: f64 = (0.29214664 * var_terfc);
        let assign18590_e18021: f64 = (var_terfc * var_terfc);
        let assign18590_e18022: f64 = (var_berfc * assign18590_e18021);
        let assign18590_e18023: f64 = (assign18590_e18017 + assign18590_e18022);
        let assign18590_e18027: f64 = (var_terfc * var_terfc);
        let assign18590_e18029: f64 = (assign18590_e18027 * var_terfc);
        let assign18590_e18030: f64 = (var_cerfc * assign18590_e18029);
        let assign18590_e18031: f64 = (assign18590_e18023 + assign18590_e18030);
        let assign18590_e18033: f64 = (assign18590_e18031 * var_tmp);
        (assign18590_e18033, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign18590_e18027 * var_terfc_dn5)))) * var_tmp) + (assign18590_e18031 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign18590_e18027 * var_terfc_dn6)))) * var_tmp) + (assign18590_e18031 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign18590_e18027 * var_terfc_dn7)))) * var_tmp) + (assign18590_e18031 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign18590_e18027 * var_terfc_dn8)))) * var_tmp) + (assign18590_e18031 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign18590_e18035;
        var_erfcpos_dn5 = assign18590_e18035_d_n5;
        var_erfcpos_dn6 = assign18590_e18035_d_n6;
        var_erfcpos_dn7 = assign18590_e18035_d_n7;
        var_erfcpos_dn8 = assign18590_e18035_d_n8;

        let assign18600_e18038: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard317 = assign18600_e18038;

        let (assign18610_e18052, assign18610_e18052_d_n5, assign18610_e18052_d_n6, assign18610_e18052_d_n7, assign18610_e18052_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard317 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign18610_e18052;
        var_erfctimesexpmtat_dn5 = assign18610_e18052_d_n5;
        var_erfctimesexpmtat_dn6 = assign18610_e18052_d_n6;
        var_erfctimesexpmtat_dn7 = assign18610_e18052_d_n7;
        var_erfctimesexpmtat_dn8 = assign18610_e18052_d_n8;

        let assign18620_e18055: f64 = (-230.25850929940458);
        let assign18620_e18056: f64 = if var_mtat > assign18620_e18055 { 1.0 } else { 0.0 };
        var_guard318 = assign18620_e18056;

        let (assign18630_e18074, assign18630_e18074_d_n5, assign18630_e18074_d_n6, assign18630_e18074_d_n7, assign18630_e18074_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard317 == 0.0)) && (var_guard318 != 0.0)) {
        let assign18630_e18072: f64 = (var_mtat).exp();
        (assign18630_e18072, (assign18630_e18072 * var_mtat_dn5), (assign18630_e18072 * var_mtat_dn6), (assign18630_e18072 * var_mtat_dn7), (assign18630_e18072 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18630_e18074;
        var_tmp_dn5 = assign18630_e18074_d_n5;
        var_tmp_dn6 = assign18630_e18074_d_n6;
        var_tmp_dn7 = assign18630_e18074_d_n7;
        var_tmp_dn8 = assign18630_e18074_d_n8;

        let (assign18640_e18117, assign18640_e18117_d_n5, assign18640_e18117_d_n6, assign18640_e18117_d_n7, assign18640_e18117_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard317 == 0.0)) && (var_guard318 == 0.0)) {
        let assign18640_e18093: f64 = (-230.25850929940458);
        let assign18640_e18095: f64 = (assign18640_e18093 - var_mtat);
        let assign18640_e18099: f64 = (-230.25850929940458);
        let assign18640_e18101: f64 = (assign18640_e18099 - var_mtat);
        let assign18640_e18104: f64 = (-230.25850929940458);
        let assign18640_e18106: f64 = (assign18640_e18104 - var_mtat);
        let assign18640_e18108: f64 = (assign18640_e18106 * 0.3333333333333333);
        let assign18640_e18109: f64 = (1.0 + assign18640_e18108);
        let assign18640_e18110: f64 = (assign18640_e18101 * assign18640_e18109);
        let assign18640_e18111: f64 = (0.5 * assign18640_e18110);
        let assign18640_e18112: f64 = (1.0 + assign18640_e18111);
        let assign18640_e18113: f64 = (assign18640_e18095 * assign18640_e18112);
        let assign18640_e18114: f64 = (1.0 + assign18640_e18113);
        let assign18640_e18115: f64 = (1e-100 / assign18640_e18114);
        (assign18640_e18115, (-((1e-100 * (((-var_mtat_dn5) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-var_mtat_dn5) * assign18640_e18109) + (assign18640_e18101 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-var_mtat_dn6) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-var_mtat_dn6) * assign18640_e18109) + (assign18640_e18101 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-var_mtat_dn7) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-var_mtat_dn7) * assign18640_e18109) + (assign18640_e18101 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))), (-((1e-100 * (((-var_mtat_dn8) * assign18640_e18112) + (assign18640_e18095 * (0.5 * (((-var_mtat_dn8) * assign18640_e18109) + (assign18640_e18101 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign18640_e18114 * assign18640_e18114))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18640_e18117;
        var_tmp_dn5 = assign18640_e18117_d_n5;
        var_tmp_dn6 = assign18640_e18117_d_n6;
        var_tmp_dn7 = assign18640_e18117_d_n7;
        var_tmp_dn8 = assign18640_e18117_d_n8;

        let (assign18650_e18136, assign18650_e18136_d_n5, assign18650_e18136_d_n6, assign18650_e18136_d_n7, assign18650_e18136_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) && (var_guard317 == 0.0)) {
        let assign18650_e18132: f64 = (2.0 * var_tmp);
        let assign18650_e18134: f64 = (assign18650_e18132 - var_erfcpos);
        (assign18650_e18134, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign18650_e18136;
        var_erfctimesexpmtat_dn5 = assign18650_e18136_d_n5;
        var_erfctimesexpmtat_dn6 = assign18650_e18136_d_n6;
        var_erfctimesexpmtat_dn7 = assign18650_e18136_d_n7;
        var_erfctimesexpmtat_dn8 = assign18650_e18136_d_n8;

        let (assign18660_e18156, assign18660_e18156_d_n5, assign18660_e18156_d_n6, assign18660_e18156_d_n7, assign18660_e18156_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18660_e18148: f64 = (1.772453850905516 * 0.5);
        let assign18660_e18151: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign18660_e18153: f64 = (assign18660_e18151 / var_ktat);
        let assign18660_e18154: f64 = (assign18660_e18148 * assign18660_e18153);
        (assign18660_e18154, (assign18660_e18148 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign18660_e18151 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign18660_e18148 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign18660_e18151 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign18660_e18148 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign18660_e18151 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign18660_e18148 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign18660_e18151 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign18660_e18156;
        var_gammamax_dn5 = assign18660_e18156_d_n5;
        var_gammamax_dn6 = assign18660_e18156_d_n6;
        var_gammamax_dn7 = assign18660_e18156_d_n7;
        var_gammamax_dn8 = assign18660_e18156_d_n8;

        let (assign18670_e18174, assign18670_e18174_d_n5, assign18670_e18174_d_n6, assign18670_e18174_d_n7, assign18670_e18174_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18670_e18169: f64 = (var_asrh * var_gammamax);
        let assign18670_e18171: f64 = (assign18670_e18169 * var_wtat);
        let assign18670_e18172: f64 = (p.p845 * assign18670_e18171);
        (assign18670_e18172, (p.p845 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign18670_e18169 * var_wtat_dn5))), (p.p845 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign18670_e18169 * var_wtat_dn6))), (p.p845 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign18670_e18169 * var_wtat_dn7))), (p.p845 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign18670_e18169 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign18670_e18174;
        var_itat_dn5 = assign18670_e18174_d_n5;
        var_itat_dn6 = assign18670_e18174_d_n6;
        var_itat_dn7 = assign18670_e18174_d_n7;
        var_itat_dn8 = assign18670_e18174_d_n8;

        let assign18680_e18177: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        var_guard319 = assign18680_e18177;

        let (assign18690_e18188, assign18690_e18188_d_n5, assign18690_e18188_d_n6, assign18690_e18188_d_n7, assign18690_e18188_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign18690_e18188;
        var_ibbt_dn5 = assign18690_e18188_d_n5;
        var_ibbt_dn6 = assign18690_e18188_d_n6;
        var_ibbt_dn7 = assign18690_e18188_d_n7;
        var_ibbt_dn8 = assign18690_e18188_d_n8;

        let assign18700_e18191: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard320 = assign18700_e18191;

        let (assign18710_e18210, assign18710_e18210_d_n5, assign18710_e18210_d_n6, assign18710_e18210_d_n7, assign18710_e18210_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) && (var_guard320 != 0.0)) {
        let assign18710_e18205: f64 = (p.p828 - var_vbbt);
        let assign18710_e18207: f64 = (assign18710_e18205 * var_vbirbotinv);
        let assign18710_e18208: f64 = (assign18710_e18207).sqrt();
        (assign18710_e18208, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18710_e18210;
        var_tmp_dn5 = assign18710_e18210_d_n5;
        var_tmp_dn6 = assign18710_e18210_d_n6;
        var_tmp_dn7 = assign18710_e18210_d_n7;
        var_tmp_dn8 = assign18710_e18210_d_n8;

        let (assign18720_e18231, assign18720_e18231_d_n5, assign18720_e18231_d_n6, assign18720_e18231_d_n7, assign18720_e18231_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign18720_e18225: f64 = (p.p828 - var_vbbt);
        let assign18720_e18227: f64 = (assign18720_e18225 * var_vbirbotinv);
        let assign18720_e18229: f64 = (assign18720_e18227).powf(p.p831);
        (assign18720_e18229, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18720_e18231;
        var_tmp_dn5 = assign18720_e18231_d_n5;
        var_tmp_dn6 = assign18720_e18231_d_n6;
        var_tmp_dn7 = assign18720_e18231_d_n7;
        var_tmp_dn8 = assign18720_e18231_d_n8;

        let (assign18730_e18251, assign18730_e18251_d_n5, assign18730_e18251_d_n6, assign18730_e18251_d_n7, assign18730_e18251_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) {
        let assign18730_e18244: f64 = (p.p828 - var_vbbt);
        let assign18730_e18246: f64 = (assign18730_e18244 * var_wdepnulrinvbot);
        let assign18730_e18248: f64 = (assign18730_e18246 / var_tmp);
        let assign18730_e18249: f64 = (var_one_over_one_minus_pbot * assign18730_e18248);
        (assign18730_e18249, (var_one_over_one_minus_pbot * (-((assign18730_e18246 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign18730_e18246 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign18730_e18246 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign18730_e18246 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign18730_e18251;
        var_fmaxr_dn5 = assign18730_e18251_d_n5;
        var_fmaxr_dn6 = assign18730_e18251_d_n6;
        var_fmaxr_dn7 = assign18730_e18251_d_n7;
        var_fmaxr_dn8 = assign18730_e18251_d_n8;

        let assign18740_e18253: f64 = (-var_fbbtbot);
        let assign18740_e18255: f64 = (assign18740_e18253 / var_fmaxr);
        let assign18740_e18256: f64 = (assign18740_e18255).abs();
        let assign18740_e18258: f64 = if assign18740_e18256 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard321 = assign18740_e18258;

        let (assign18750_e18276, assign18750_e18276_d_n5, assign18750_e18276_d_n6, assign18750_e18276_d_n7, assign18750_e18276_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) && (var_guard321 != 0.0)) {
        let assign18750_e18271: f64 = (-var_fbbtbot);
        let assign18750_e18273: f64 = (assign18750_e18271 / var_fmaxr);
        let assign18750_e18274: f64 = (assign18750_e18273).exp();
        (assign18750_e18274, (assign18750_e18274 * (-((assign18750_e18271 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign18750_e18274 * (-((assign18750_e18271 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18750_e18276;
        var_tmp_dn5 = assign18750_e18276_d_n5;
        var_tmp_dn6 = assign18750_e18276_d_n6;
        var_tmp_dn7 = assign18750_e18276_d_n7;
        var_tmp_dn8 = assign18750_e18276_d_n8;

        let assign18760_e18278: f64 = (-var_fbbtbot);
        let assign18760_e18280: f64 = (assign18760_e18278 / var_fmaxr);
        let assign18760_e18282: f64 = if assign18760_e18280 < 0.0 { 1.0 } else { 0.0 };
        var_guard322 = assign18760_e18282;

        let (assign18770_e18333, assign18770_e18333_d_n5, assign18770_e18333_d_n6, assign18770_e18333_d_n7, assign18770_e18333_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 != 0.0)) {
        let assign18770_e18300: f64 = (-230.25850929940458);
        let assign18770_e18302: f64 = (-var_fbbtbot);
        let assign18770_e18304: f64 = (assign18770_e18302 / var_fmaxr);
        let assign18770_e18305: f64 = (assign18770_e18300 - assign18770_e18304);
        let assign18770_e18309: f64 = (-230.25850929940458);
        let assign18770_e18311: f64 = (-var_fbbtbot);
        let assign18770_e18313: f64 = (assign18770_e18311 / var_fmaxr);
        let assign18770_e18314: f64 = (assign18770_e18309 - assign18770_e18313);
        let assign18770_e18317: f64 = (-230.25850929940458);
        let assign18770_e18319: f64 = (-var_fbbtbot);
        let assign18770_e18321: f64 = (assign18770_e18319 / var_fmaxr);
        let assign18770_e18322: f64 = (assign18770_e18317 - assign18770_e18321);
        let assign18770_e18324: f64 = (assign18770_e18322 * 0.3333333333333333);
        let assign18770_e18325: f64 = (1.0 + assign18770_e18324);
        let assign18770_e18326: f64 = (assign18770_e18314 * assign18770_e18325);
        let assign18770_e18327: f64 = (0.5 * assign18770_e18326);
        let assign18770_e18328: f64 = (1.0 + assign18770_e18327);
        let assign18770_e18329: f64 = (assign18770_e18305 * assign18770_e18328);
        let assign18770_e18330: f64 = (1.0 + assign18770_e18329);
        let assign18770_e18331: f64 = (1e-100 / assign18770_e18330);
        (assign18770_e18331, (-((1e-100 * (((-(-((assign18770_e18302 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))), (-((1e-100 * (((-(-((assign18770_e18302 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign18770_e18328) + (assign18770_e18305 * (0.5 * (((-(-((assign18770_e18311 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign18770_e18325) + (assign18770_e18314 * ((-(-((assign18770_e18319 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18770_e18330 * assign18770_e18330))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18770_e18333;
        var_tmp_dn5 = assign18770_e18333_d_n5;
        var_tmp_dn6 = assign18770_e18333_d_n6;
        var_tmp_dn7 = assign18770_e18333_d_n7;
        var_tmp_dn8 = assign18770_e18333_d_n8;

        let (assign18780_e18382, assign18780_e18382_d_n5, assign18780_e18382_d_n6, assign18780_e18382_d_n7, assign18780_e18382_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 == 0.0)) {
        let assign18780_e18352: f64 = (-var_fbbtbot);
        let assign18780_e18354: f64 = (assign18780_e18352 / var_fmaxr);
        let assign18780_e18356: f64 = (assign18780_e18354 - 230.25850929940458);
        let assign18780_e18360: f64 = (-var_fbbtbot);
        let assign18780_e18362: f64 = (assign18780_e18360 / var_fmaxr);
        let assign18780_e18364: f64 = (assign18780_e18362 - 230.25850929940458);
        let assign18780_e18367: f64 = (-var_fbbtbot);
        let assign18780_e18369: f64 = (assign18780_e18367 / var_fmaxr);
        let assign18780_e18371: f64 = (assign18780_e18369 - 230.25850929940458);
        let assign18780_e18373: f64 = (assign18780_e18371 * 0.3333333333333333);
        let assign18780_e18374: f64 = (1.0 + assign18780_e18373);
        let assign18780_e18375: f64 = (assign18780_e18364 * assign18780_e18374);
        let assign18780_e18376: f64 = (0.5 * assign18780_e18375);
        let assign18780_e18377: f64 = (1.0 + assign18780_e18376);
        let assign18780_e18378: f64 = (assign18780_e18356 * assign18780_e18377);
        let assign18780_e18379: f64 = (1.0 + assign18780_e18378);
        let assign18780_e18380: f64 = (1e100 * assign18780_e18379);
        (assign18780_e18380, (1e100 * (((-((assign18780_e18352 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18780_e18352 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign18780_e18377) + (assign18780_e18356 * (0.5 * (((-((assign18780_e18360 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign18780_e18374) + (assign18780_e18364 * ((-((assign18780_e18367 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18780_e18382;
        var_tmp_dn5 = assign18780_e18382_d_n5;
        var_tmp_dn6 = assign18780_e18382_d_n6;
        var_tmp_dn7 = assign18780_e18382_d_n7;
        var_tmp_dn8 = assign18780_e18382_d_n8;

        let (assign18790_e18402, assign18790_e18402_d_n5, assign18790_e18402_d_n6, assign18790_e18402_d_n7, assign18790_e18402_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard319 == 0.0)) {
        let assign18790_e18395: f64 = (var_v2 * var_fmaxr);
        let assign18790_e18397: f64 = (assign18790_e18395 * var_fmaxr);
        let assign18790_e18399: f64 = (assign18790_e18397 * var_tmp);
        let assign18790_e18400: f64 = (p.p851 * assign18790_e18399);
        (assign18790_e18400, (p.p851 * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign18790_e18395 * var_fmaxr_dn5)) * var_tmp) + (assign18790_e18397 * var_tmp_dn5))), (p.p851 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign18790_e18395 * var_fmaxr_dn6)) * var_tmp) + (assign18790_e18397 * var_tmp_dn6))), (p.p851 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign18790_e18395 * var_fmaxr_dn7)) * var_tmp) + (assign18790_e18397 * var_tmp_dn7))), (p.p851 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign18790_e18395 * var_fmaxr_dn8)) * var_tmp) + (assign18790_e18397 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign18790_e18402;
        var_ibbt_dn5 = assign18790_e18402_d_n5;
        var_ibbt_dn6 = assign18790_e18402_d_n6;
        var_ibbt_dn7 = assign18790_e18402_d_n7;
        var_ibbt_dn8 = assign18790_e18402_d_n8;

        let assign18800_e18405: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        var_guard323 = assign18800_e18405;

        let (assign18810_e18416, assign18810_e18416_d_n5, assign18810_e18416_d_n6, assign18810_e18416_d_n7, assign18810_e18416_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard323 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign18810_e18416;
        var_fbreakdown_dn5 = assign18810_e18416_d_n5;
        var_fbreakdown_dn6 = assign18810_e18416_d_n6;
        var_fbreakdown_dn7 = assign18810_e18416_d_n7;
        var_fbreakdown_dn8 = assign18810_e18416_d_n8;

        let assign18820_e18419: f64 = (-var_alphaav);
        let assign18820_e18421: f64 = (assign18820_e18419 * p.p860);
        let assign18820_e18422: f64 = if var_vav > assign18820_e18421 { 1.0 } else { 0.0 };
        var_guard324 = assign18820_e18422;

        let assign18830_e18425: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        var_guard325 = assign18830_e18425;

        let (assign18840_e18455, assign18840_e18455_d_n5, assign18840_e18455_d_n6, assign18840_e18455_d_n7, assign18840_e18455_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 != 0.0)) && (var_guard325 != 0.0)) {
        let assign18840_e18441: f64 = (var_vav * var_vbrinvbot);
        let assign18840_e18444: f64 = (var_vav * var_vbrinvbot);
        let assign18840_e18445: f64 = (assign18840_e18441 * assign18840_e18444);
        let assign18840_e18448: f64 = (var_vav * var_vbrinvbot);
        let assign18840_e18449: f64 = (assign18840_e18445 * assign18840_e18448);
        let assign18840_e18452: f64 = (var_vav * var_vbrinvbot);
        let assign18840_e18453: f64 = (assign18840_e18449 * assign18840_e18452);
        (assign18840_e18453, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18840_e18455;
        var_tmp_dn5 = assign18840_e18455_d_n5;
        var_tmp_dn6 = assign18840_e18455_d_n6;
        var_tmp_dn7 = assign18840_e18455_d_n7;
        var_tmp_dn8 = assign18840_e18455_d_n8;

        let (assign18850_e18477, assign18850_e18477_d_n5, assign18850_e18477_d_n6, assign18850_e18477_d_n7, assign18850_e18477_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 != 0.0)) && (var_guard325 == 0.0)) {
        let assign18850_e18472: f64 = (var_vav * var_vbrinvbot);
        let assign18850_e18473: f64 = (assign18850_e18472).abs();
        let assign18850_e18475: f64 = (assign18850_e18473).powf(p.p863);
        (assign18850_e18475, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign18850_e18477;
        var_tmp_dn5 = assign18850_e18477_d_n5;
        var_tmp_dn6 = assign18850_e18477_d_n6;
        var_tmp_dn7 = assign18850_e18477_d_n7;
        var_tmp_dn8 = assign18850_e18477_d_n8;

        let (assign18860_e18495, assign18860_e18495_d_n5, assign18860_e18495_d_n6, assign18860_e18495_d_n7, assign18860_e18495_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 != 0.0)) {
        let assign18860_e18492: f64 = (1.0 - var_tmp);
        let assign18860_e18493: f64 = (1.0 / assign18860_e18492);
        (assign18860_e18493, (-((-var_tmp_dn5) / (assign18860_e18492 * assign18860_e18492))), (-((-var_tmp_dn6) / (assign18860_e18492 * assign18860_e18492))), (-((-var_tmp_dn7) / (assign18860_e18492 * assign18860_e18492))), (-((-var_tmp_dn8) / (assign18860_e18492 * assign18860_e18492))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign18860_e18495;
        var_fbreakdown_dn5 = assign18860_e18495_d_n5;
        var_fbreakdown_dn6 = assign18860_e18495_d_n6;
        var_fbreakdown_dn7 = assign18860_e18495_d_n7;
        var_fbreakdown_dn8 = assign18860_e18495_d_n8;

        let (assign18870_e18518, assign18870_e18518_d_n5, assign18870_e18518_d_n6, assign18870_e18518_d_n7, assign18870_e18518_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 == 0.0)) {
        let assign18870_e18512: f64 = (var_alphaav * p.p860);
        let assign18870_e18513: f64 = (var_vav + assign18870_e18512);
        let assign18870_e18515: f64 = (assign18870_e18513 * var_slopebot);
        let assign18870_e18516: f64 = (var_fstopbot + assign18870_e18515);
        (assign18870_e18516, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign18870_e18518;
        var_fbreakdown_dn5 = assign18870_e18518_d_n5;
        var_fbreakdown_dn6 = assign18870_e18518_d_n6;
        var_fbreakdown_dn7 = assign18870_e18518_d_n7;
        var_fbreakdown_dn8 = assign18870_e18518_d_n8;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn5_slot = var_erfcpos_dn5;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn5_slot = var_erfctimesexpmtat_dn5;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn5_slot = var_gammamax_dn5;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_guard315_slot = var_guard315;
        *var_guard316_slot = var_guard316;
        *var_guard317_slot = var_guard317;
        *var_guard318_slot = var_guard318;
        *var_guard319_slot = var_guard319;
        *var_guard320_slot = var_guard320;
        *var_guard321_slot = var_guard321;
        *var_guard322_slot = var_guard322;
        *var_guard323_slot = var_guard323;
        *var_guard324_slot = var_guard324;
        *var_guard325_slot = var_guard325;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbreakdown: f64,
        var_fbreakdown_dn5: f64,
        var_fbreakdown_dn6: f64,
        var_fbreakdown_dn7: f64,
        var_fbreakdown_dn8: f64,
        var_ftdsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard309: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vjsrh: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn5_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn5_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard326_slot: &mut f64,
        var_guard327_slot: &mut f64,
        var_guard328_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard331_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn5_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn5_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn5_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn5_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn5_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn5_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn5_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn5_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn5_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn5_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn5_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn5_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn5_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn5_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn5_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn5_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn5: f64 = *var_asrh_dn5_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn5: f64 = *var_btat_dn5_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard326: f64 = *var_guard326_slot;
        let mut var_guard327: f64 = *var_guard327_slot;
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard331: f64 = *var_guard331_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn5: f64 = *var_ijunsti_dn5_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn5: f64 = *var_itat_dn5_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn5: f64 = *var_ktat_dn5_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn5: f64 = *var_ltat_dn5_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn5: f64 = *var_mtat_dn5_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn5: f64 = *var_sqrtumax_dn5_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn5: f64 = *var_terfc_dn5_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn5: f64 = *var_twoatatoverthreebtat_dn5_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn5: f64 = *var_umax_dn5_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn5: f64 = *var_umaxbeforelimiting_dn5_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn5: f64 = *var_umaxpoweronepointfive_dn5_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn5: f64 = *var_wgamma_dn5_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn5: f64 = *var_wtat_dn5_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn5: f64 = *var_xerfc_dn5_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign18880_e18537, assign18880_e18537_d_n5, assign18880_e18537_d_n6, assign18880_e18537_d_n7, assign18880_e18537_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard309 == 0.0)) {
        let assign18880_e18528: f64 = (var_id__blk219 + var_isrh);
        let assign18880_e18530: f64 = (assign18880_e18528 + var_itat);
        let assign18880_e18532: f64 = (assign18880_e18530 + var_ibbt);
        let assign18880_e18533: f64 = (p.p29 * assign18880_e18532);
        let assign18880_e18535: f64 = (assign18880_e18533 * var_fbreakdown);
        (assign18880_e18535, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign18880_e18533 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign18880_e18533 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign18880_e18533 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign18880_e18533 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign18880_e18537;
        var_ijunbot_dn5 = assign18880_e18537_d_n5;
        var_ijunbot_dn6 = assign18880_e18537_d_n6;
        var_ijunbot_dn7 = assign18880_e18537_d_n7;
        var_ijunbot_dn8 = assign18880_e18537_d_n8;

        let assign18890_e18540: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard326 = assign18890_e18540;

        let (assign18900_e18548, assign18900_e18548_d_n5, assign18900_e18548_d_n6, assign18900_e18548_d_n7, assign18900_e18548_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign18900_e18548;
        var_ijunsti_dn5 = assign18900_e18548_d_n5;
        var_ijunsti_dn6 = assign18900_e18548_d_n6;
        var_ijunsti_dn7 = assign18900_e18548_d_n7;
        var_ijunsti_dn8 = assign18900_e18548_d_n8;

        let (assign18910_e18559,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) {
        let assign18910_e18557: f64 = (var_idsatsti * var_idmult);
        (assign18910_e18557,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign18910_e18559;

        let assign18920_e18566: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        var_guard327 = assign18920_e18566;

        let (assign18930_e18577, assign18930_e18577_d_n5, assign18930_e18577_d_n6, assign18930_e18577_d_n7, assign18930_e18577_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign18930_e18577;
        var_isrh_dn5 = assign18930_e18577_d_n5;
        var_isrh_dn6 = assign18930_e18577_d_n6;
        var_isrh_dn7 = assign18930_e18577_d_n7;
        var_isrh_dn8 = assign18930_e18577_d_n8;

        let (assign18940_e18591,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign18940_e18589: f64 = (var_vbisti - var_vjsrh);
        (assign18940_e18589,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign18940_e18591;

        let (assign18950_e18610,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign18950_e18605: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign18950_e18606: f64 = (1.0 - assign18950_e18605);
        let assign18950_e18607: f64 = (assign18950_e18606).sqrt();
        let assign18950_e18608: f64 = (1.0 - assign18950_e18607);
        (assign18950_e18608,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign18950_e18610;

        let assign18960_e18613: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard328 = assign18960_e18613;

        let (assign18970_e18627,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) && (var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18970_e18627;

        let (assign18980_e18659,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) && (var_guard328 == 0.0)) {
        let assign18980_e18642: f64 = (var_wsrhstep * var_wsrhstep);
        let assign18980_e18644: f64 = (var_wsrhstep).ln();
        let assign18980_e18645: f64 = (assign18980_e18642 * assign18980_e18644);
        let assign18980_e18648: f64 = (1.0 - var_wsrhstep);
        let assign18980_e18649: f64 = (assign18980_e18645 / assign18980_e18648);
        let assign18980_e18651: f64 = (assign18980_e18649 + var_wsrhstep);
        let assign18980_e18655: f64 = (2.0 * p.p832);
        let assign18980_e18656: f64 = (1.0 - assign18980_e18655);
        let assign18980_e18657: f64 = (assign18980_e18651 * assign18980_e18656);
        (assign18980_e18657,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18980_e18659;

        let (assign18990_e18673,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign18990_e18671: f64 = (var_wsrhstep + var_dwsrh);
        (assign18990_e18671,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign18990_e18673;

        let assign19000_e18676: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard329 = assign19000_e18676;

        let (assign19010_e18693, assign19010_e18693_d_n5, assign19010_e18693_d_n6, assign19010_e18693_d_n7, assign19010_e18693_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) && (var_guard329 != 0.0)) {
        let assign19010_e18690: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign19010_e18691: f64 = (assign19010_e18690).sqrt();
        (assign19010_e18691, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19010_e18693;
        var_tmp_dn5 = assign19010_e18693_d_n5;
        var_tmp_dn6 = assign19010_e18693_d_n6;
        var_tmp_dn7 = assign19010_e18693_d_n7;
        var_tmp_dn8 = assign19010_e18693_d_n8;

        let (assign19020_e18712, assign19020_e18712_d_n5, assign19020_e18712_d_n6, assign19020_e18712_d_n7, assign19020_e18712_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) && (var_guard329 == 0.0)) {
        let assign19020_e18708: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign19020_e18710: f64 = (assign19020_e18708).powf(p.p832);
        (assign19020_e18710, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19020_e18712;
        var_tmp_dn5 = assign19020_e18712_d_n5;
        var_tmp_dn6 = assign19020_e18712_d_n6;
        var_tmp_dn7 = assign19020_e18712_d_n7;
        var_tmp_dn8 = assign19020_e18712_d_n8;

        let (assign19030_e18726, assign19030_e18726_d_n5, assign19030_e18726_d_n6, assign19030_e18726_d_n7, assign19030_e18726_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign19030_e18724: f64 = (var_wdepnulrsti * var_tmp);
        (assign19030_e18724, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign19030_e18726;
        var_wdep_dn5 = assign19030_e18726_d_n5;
        var_wdep_dn6 = assign19030_e18726_d_n6;
        var_wdep_dn7 = assign19030_e18726_d_n7;
        var_wdep_dn8 = assign19030_e18726_d_n8;

        let (assign19040_e18744, assign19040_e18744_d_n5, assign19040_e18744_d_n6, assign19040_e18744_d_n7, assign19040_e18744_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign19040_e18739: f64 = (var_zinv - 1.0);
        let assign19040_e18741: f64 = (assign19040_e18739 * var_wdep);
        let assign19040_e18742: f64 = (var_ftdsti * assign19040_e18741);
        (assign19040_e18742, (var_ftdsti * (assign19040_e18739 * var_wdep_dn5)), (var_ftdsti * (assign19040_e18739 * var_wdep_dn6)), (var_ftdsti * (assign19040_e18739 * var_wdep_dn7)), (var_ftdsti * (assign19040_e18739 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign19040_e18744;
        var_asrh_dn5 = assign19040_e18744_d_n5;
        var_asrh_dn6 = assign19040_e18744_d_n6;
        var_asrh_dn7 = assign19040_e18744_d_n7;
        var_asrh_dn8 = assign19040_e18744_d_n8;

        let (assign19050_e18760, assign19050_e18760_d_n5, assign19050_e18760_d_n6, assign19050_e18760_d_n7, assign19050_e18760_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard327 == 0.0)) {
        let assign19050_e18757: f64 = (var_asrh * var_wsrh);
        let assign19050_e18758: f64 = (p.p841 * assign19050_e18757);
        (assign19050_e18758, (p.p841 * (var_asrh_dn5 * var_wsrh)), (p.p841 * (var_asrh_dn6 * var_wsrh)), (p.p841 * (var_asrh_dn7 * var_wsrh)), (p.p841 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign19050_e18760;
        var_isrh_dn5 = assign19050_e18760_d_n5;
        var_isrh_dn6 = assign19050_e18760_d_n6;
        var_isrh_dn7 = assign19050_e18760_d_n7;
        var_isrh_dn8 = assign19050_e18760_d_n8;

        let assign19060_e18763: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard330 = assign19060_e18763;

        let (assign19070_e18774, assign19070_e18774_d_n5, assign19070_e18774_d_n6, assign19070_e18774_d_n7, assign19070_e18774_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign19070_e18774;
        var_itat_dn5 = assign19070_e18774_d_n5;
        var_itat_dn6 = assign19070_e18774_d_n6;
        var_itat_dn7 = assign19070_e18774_d_n7;
        var_itat_dn8 = assign19070_e18774_d_n8;

        let (assign19080_e18792, assign19080_e18792_d_n5, assign19080_e18792_d_n6, assign19080_e18792_d_n7, assign19080_e18792_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19080_e18787: f64 = (var_wdep * var_one_minus_psti);
        let assign19080_e18789: f64 = (assign19080_e18787 / var_vbi_minus_vjsrh);
        let assign19080_e18790: f64 = (var_btatpartsti * assign19080_e18789);
        (assign19080_e18790, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign19080_e18792;
        var_btat_dn5 = assign19080_e18792_d_n5;
        var_btat_dn6 = assign19080_e18792_d_n6;
        var_btat_dn7 = assign19080_e18792_d_n7;
        var_btat_dn8 = assign19080_e18792_d_n8;

        let (assign19090_e18808, assign19090_e18808_d_n5, assign19090_e18808_d_n6, assign19090_e18808_d_n7, assign19090_e18808_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19090_e18804: f64 = (0.666666666666667 * var_atatsti);
        let assign19090_e18806: f64 = (assign19090_e18804 / var_btat);
        (assign19090_e18806, (-((assign19090_e18804 * var_btat_dn5) / (var_btat * var_btat))), (-((assign19090_e18804 * var_btat_dn6) / (var_btat * var_btat))), (-((assign19090_e18804 * var_btat_dn7) / (var_btat * var_btat))), (-((assign19090_e18804 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign19090_e18808;
        var_twoatatoverthreebtat_dn5 = assign19090_e18808_d_n5;
        var_twoatatoverthreebtat_dn6 = assign19090_e18808_d_n6;
        var_twoatatoverthreebtat_dn7 = assign19090_e18808_d_n7;
        var_twoatatoverthreebtat_dn8 = assign19090_e18808_d_n8;

        let (assign19100_e18822, assign19100_e18822_d_n5, assign19100_e18822_d_n6, assign19100_e18822_d_n7, assign19100_e18822_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19100_e18820: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign19100_e18820, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign19100_e18822;
        var_umaxbeforelimiting_dn5 = assign19100_e18822_d_n5;
        var_umaxbeforelimiting_dn6 = assign19100_e18822_d_n6;
        var_umaxbeforelimiting_dn7 = assign19100_e18822_d_n7;
        var_umaxbeforelimiting_dn8 = assign19100_e18822_d_n8;

        let (assign19110_e18843, assign19110_e18843_d_n5, assign19110_e18843_d_n6, assign19110_e18843_d_n7, assign19110_e18843_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19110_e18834: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19110_e18837: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19110_e18839: f64 = (assign19110_e18837 + 1.0);
        let assign19110_e18840: f64 = (assign19110_e18834 / assign19110_e18839);
        let assign19110_e18841: f64 = (assign19110_e18840).sqrt();
        (assign19110_e18841, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign19110_e18839) - (assign19110_e18834 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign19110_e18839) - (assign19110_e18834 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign19110_e18839) - (assign19110_e18834 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign19110_e18839) - (assign19110_e18834 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign19110_e18839 * assign19110_e18839)) / (2.0 * assign19110_e18841)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign19110_e18843;
        var_umax_dn5 = assign19110_e18843_d_n5;
        var_umax_dn6 = assign19110_e18843_d_n6;
        var_umax_dn7 = assign19110_e18843_d_n7;
        var_umax_dn8 = assign19110_e18843_d_n8;

        let (assign19120_e18856, assign19120_e18856_d_n5, assign19120_e18856_d_n6, assign19120_e18856_d_n7, assign19120_e18856_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19120_e18854: f64 = (var_umax).sqrt();
        (assign19120_e18854, (var_umax_dn5 / (2.0 * assign19120_e18854)), (var_umax_dn6 / (2.0 * assign19120_e18854)), (var_umax_dn7 / (2.0 * assign19120_e18854)), (var_umax_dn8 / (2.0 * assign19120_e18854)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign19120_e18856;
        var_sqrtumax_dn5 = assign19120_e18856_d_n5;
        var_sqrtumax_dn6 = assign19120_e18856_d_n6;
        var_sqrtumax_dn7 = assign19120_e18856_d_n7;
        var_sqrtumax_dn8 = assign19120_e18856_d_n8;

        let (assign19130_e18870, assign19130_e18870_d_n5, assign19130_e18870_d_n6, assign19130_e18870_d_n7, assign19130_e18870_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19130_e18868: f64 = (var_umax * var_sqrtumax);
        (assign19130_e18868, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign19130_e18870;
        var_umaxpoweronepointfive_dn5 = assign19130_e18870_d_n5;
        var_umaxpoweronepointfive_dn6 = assign19130_e18870_d_n6;
        var_umaxpoweronepointfive_dn7 = assign19130_e18870_d_n7;
        var_umaxpoweronepointfive_dn8 = assign19130_e18870_d_n8;

        let assign19140_e18872: f64 = (-p.p832);
        let assign19140_e18874: f64 = (assign19140_e18872 * var_one_over_one_minus_psti);
        let assign19140_e18876: f64 = (-1.0);
        let assign19140_e18877: f64 = if assign19140_e18874 == assign19140_e18876 { 1.0 } else { 0.0 };
        var_guard331 = assign19140_e18877;

        let (assign19150_e18897, assign19150_e18897_d_n5, assign19150_e18897_d_n6, assign19150_e18897_d_n7, assign19150_e18897_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard331 != 0.0)) {
        let assign19150_e18893: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19150_e18894: f64 = (1.0 + assign19150_e18893);
        let assign19150_e18895: f64 = (1.0 / assign19150_e18894);
        (assign19150_e18895, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign19150_e18894 * assign19150_e18894))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign19150_e18894 * assign19150_e18894))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign19150_e18894 * assign19150_e18894))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign19150_e18894 * assign19150_e18894))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19150_e18897;
        var_wgamma_dn5 = assign19150_e18897_d_n5;
        var_wgamma_dn6 = assign19150_e18897_d_n6;
        var_wgamma_dn7 = assign19150_e18897_d_n7;
        var_wgamma_dn8 = assign19150_e18897_d_n8;

        let (assign19160_e18921, assign19160_e18921_d_n5, assign19160_e18921_d_n6, assign19160_e18921_d_n7, assign19160_e18921_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard331 == 0.0)) {
        let assign19160_e18913: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19160_e18914: f64 = (1.0 + assign19160_e18913);
        let assign19160_e18916: f64 = (-p.p832);
        let assign19160_e18918: f64 = (assign19160_e18916 * var_one_over_one_minus_psti);
        let assign19160_e18919: f64 = (assign19160_e18914).powf(assign19160_e18918);
        (assign19160_e18919, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign19160_e18914))) }, if 0.0 == 0.0 && ((assign19160_e18918) as f64).is_finite() && ((assign19160_e18918) as f64).fract() == 0.0 { if assign19160_e18918 == 0.0 { 0.0 } else { (assign19160_e18918 * ((assign19160_e18914).powf(assign19160_e18918 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign19160_e18919 * (assign19160_e18918 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign19160_e18914))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19160_e18921;
        var_wgamma_dn5 = assign19160_e18921_d_n5;
        var_wgamma_dn6 = assign19160_e18921_d_n6;
        var_wgamma_dn7 = assign19160_e18921_d_n7;
        var_wgamma_dn8 = assign19160_e18921_d_n8;

        let (assign19170_e18939, assign19170_e18939_d_n5, assign19170_e18939_d_n6, assign19170_e18939_d_n7, assign19170_e18939_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19170_e18933: f64 = (var_wsrh * var_wgamma);
        let assign19170_e18936: f64 = (var_wsrh + var_wgamma);
        let assign19170_e18937: f64 = (assign19170_e18933 / assign19170_e18936);
        (assign19170_e18937, ((((var_wsrh * var_wgamma_dn5) * assign19170_e18936) - (assign19170_e18933 * var_wgamma_dn5)) / (assign19170_e18936 * assign19170_e18936)), ((((var_wsrh * var_wgamma_dn6) * assign19170_e18936) - (assign19170_e18933 * var_wgamma_dn6)) / (assign19170_e18936 * assign19170_e18936)), ((((var_wsrh * var_wgamma_dn7) * assign19170_e18936) - (assign19170_e18933 * var_wgamma_dn7)) / (assign19170_e18936 * assign19170_e18936)), ((((var_wsrh * var_wgamma_dn8) * assign19170_e18936) - (assign19170_e18933 * var_wgamma_dn8)) / (assign19170_e18936 * assign19170_e18936)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign19170_e18939;
        var_wtat_dn5 = assign19170_e18939_d_n5;
        var_wtat_dn6 = assign19170_e18939_d_n6;
        var_wtat_dn7 = assign19170_e18939_d_n7;
        var_wtat_dn8 = assign19170_e18939_d_n8;

        let (assign19180_e18956, assign19180_e18956_d_n5, assign19180_e18956_d_n6, assign19180_e18956_d_n7, assign19180_e18956_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19180_e18952: f64 = (var_btat / var_sqrtumax);
        let assign19180_e18953: f64 = (0.375 * assign19180_e18952);
        let assign19180_e18954: f64 = (assign19180_e18953).sqrt();
        (assign19180_e18954, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19180_e18954)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19180_e18954)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign19180_e18956;
        var_ktat_dn5 = assign19180_e18956_d_n5;
        var_ktat_dn6 = assign19180_e18956_d_n6;
        var_ktat_dn7 = assign19180_e18956_d_n7;
        var_ktat_dn8 = assign19180_e18956_d_n8;

        let (assign19190_e18974, assign19190_e18974_d_n5, assign19190_e18974_d_n6, assign19190_e18974_d_n7, assign19190_e18974_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19190_e18969: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign19190_e18970: f64 = (2.0 * assign19190_e18969);
        let assign19190_e18972: f64 = (assign19190_e18970 - var_umax);
        (assign19190_e18972, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign19190_e18974;
        var_ltat_dn5 = assign19190_e18974_d_n5;
        var_ltat_dn6 = assign19190_e18974_d_n6;
        var_ltat_dn7 = assign19190_e18974_d_n7;
        var_ltat_dn8 = assign19190_e18974_d_n8;

        let (assign19200_e19000, assign19200_e19000_d_n5, assign19200_e19000_d_n6, assign19200_e19000_d_n7, assign19200_e19000_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19200_e18986: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign19200_e18988: f64 = (assign19200_e18986 * var_sqrtumax);
        let assign19200_e18991: f64 = (var_atatsti * var_umax);
        let assign19200_e18992: f64 = (assign19200_e18988 - assign19200_e18991);
        let assign19200_e18996: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19200_e18997: f64 = (0.5 * assign19200_e18996);
        let assign19200_e18998: f64 = (assign19200_e18992 + assign19200_e18997);
        (assign19200_e18998, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign19200_e18986 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign19200_e18986 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign19200_e18986 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign19200_e18986 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign19200_e19000;
        var_mtat_dn5 = assign19200_e19000_d_n5;
        var_mtat_dn6 = assign19200_e19000_d_n6;
        var_mtat_dn7 = assign19200_e19000_d_n7;
        var_mtat_dn8 = assign19200_e19000_d_n8;

        let (assign19210_e19016, assign19210_e19016_d_n5, assign19210_e19016_d_n6, assign19210_e19016_d_n7, assign19210_e19016_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19210_e19012: f64 = (var_ltat - 1.0);
        let assign19210_e19014: f64 = (assign19210_e19012 * var_ktat);
        (assign19210_e19014, ((var_ltat_dn5 * var_ktat) + (assign19210_e19012 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign19210_e19012 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign19210_e19012 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign19210_e19012 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign19210_e19016;
        var_xerfc_dn5 = assign19210_e19016_d_n5;
        var_xerfc_dn6 = assign19210_e19016_d_n6;
        var_xerfc_dn7 = assign19210_e19016_d_n7;
        var_xerfc_dn8 = assign19210_e19016_d_n8;

        let (assign19220_e19030, assign19220_e19030_d_n5, assign19220_e19030_d_n6, assign19220_e19030_d_n7, assign19220_e19030_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19220_e19028: f64 = (var_xerfc * var_xerfc);
        (assign19220_e19028, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign19220_e19030;
        var_ysq_dn5 = assign19220_e19030_d_n5;
        var_ysq_dn6 = assign19220_e19030_d_n6;
        var_ysq_dn7 = assign19220_e19030_d_n7;
        var_ysq_dn8 = assign19220_e19030_d_n8;

        let assign19230_e19033: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard332 = assign19230_e19033;

        let (assign19240_e19053, assign19240_e19053_d_n5, assign19240_e19053_d_n6, assign19240_e19053_d_n7, assign19240_e19053_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard332 != 0.0)) {
        let assign19240_e19049: f64 = (var_perfc * var_xerfc);
        let assign19240_e19050: f64 = (1.0 + assign19240_e19049);
        let assign19240_e19051: f64 = (1.0 / assign19240_e19050);
        (assign19240_e19051, (-((var_perfc * var_xerfc_dn5) / (assign19240_e19050 * assign19240_e19050))), (-((var_perfc * var_xerfc_dn6) / (assign19240_e19050 * assign19240_e19050))), (-((var_perfc * var_xerfc_dn7) / (assign19240_e19050 * assign19240_e19050))), (-((var_perfc * var_xerfc_dn8) / (assign19240_e19050 * assign19240_e19050))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19240_e19053;
        var_terfc_dn5 = assign19240_e19053_d_n5;
        var_terfc_dn6 = assign19240_e19053_d_n6;
        var_terfc_dn7 = assign19240_e19053_d_n7;
        var_terfc_dn8 = assign19240_e19053_d_n8;

        let (assign19250_e19074, assign19250_e19074_d_n5, assign19250_e19074_d_n6, assign19250_e19074_d_n7, assign19250_e19074_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard332 == 0.0)) {
        let assign19250_e19070: f64 = (var_perfc * var_xerfc);
        let assign19250_e19071: f64 = (1.0 - assign19250_e19070);
        let assign19250_e19072: f64 = (1.0 / assign19250_e19071);
        (assign19250_e19072, (-((-(var_perfc * var_xerfc_dn5)) / (assign19250_e19071 * assign19250_e19071))), (-((-(var_perfc * var_xerfc_dn6)) / (assign19250_e19071 * assign19250_e19071))), (-((-(var_perfc * var_xerfc_dn7)) / (assign19250_e19071 * assign19250_e19071))), (-((-(var_perfc * var_xerfc_dn8)) / (assign19250_e19071 * assign19250_e19071))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19250_e19074;
        var_terfc_dn5 = assign19250_e19074_d_n5;
        var_terfc_dn6 = assign19250_e19074_d_n6;
        var_terfc_dn7 = assign19250_e19074_d_n7;
        var_terfc_dn8 = assign19250_e19074_d_n8;

        let assign19260_e19076: f64 = (-var_ysq);
        let assign19260_e19078: f64 = (assign19260_e19076 + var_mtat);
        let assign19260_e19080: f64 = (-230.25850929940458);
        let assign19260_e19081: f64 = if assign19260_e19078 > assign19260_e19080 { 1.0 } else { 0.0 };
        var_guard333 = assign19260_e19081;

        let (assign19270_e19099, assign19270_e19099_d_n5, assign19270_e19099_d_n6, assign19270_e19099_d_n7, assign19270_e19099_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard333 != 0.0)) {
        let assign19270_e19094: f64 = (-var_ysq);
        let assign19270_e19096: f64 = (assign19270_e19094 + var_mtat);
        let assign19270_e19097: f64 = (assign19270_e19096).exp();
        (assign19270_e19097, (assign19270_e19097 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign19270_e19097 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign19270_e19097 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign19270_e19097 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19270_e19099;
        var_tmp_dn5 = assign19270_e19099_d_n5;
        var_tmp_dn6 = assign19270_e19099_d_n6;
        var_tmp_dn7 = assign19270_e19099_d_n7;
        var_tmp_dn8 = assign19270_e19099_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_btat_slot = var_btat;
        *var_btat_dn5_slot = var_btat_dn5;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard326_slot = var_guard326;
        *var_guard327_slot = var_guard327;
        *var_guard328_slot = var_guard328;
        *var_guard329_slot = var_guard329;
        *var_guard330_slot = var_guard330;
        *var_guard331_slot = var_guard331;
        *var_guard332_slot = var_guard332;
        *var_guard333_slot = var_guard333;
        *var_id__blk219_slot = var_id__blk219;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn5_slot = var_ijunsti_dn5;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_itat_slot = var_itat;
        *var_itat_dn5_slot = var_itat_dn5;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn5_slot = var_ktat_dn5;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn5_slot = var_ltat_dn5;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn5_slot = var_mtat_dn5;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn5_slot = var_sqrtumax_dn5;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn5_slot = var_terfc_dn5;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn5_slot = var_twoatatoverthreebtat_dn5;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_umax_slot = var_umax;
        *var_umax_dn5_slot = var_umax_dn5;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn5_slot = var_umaxbeforelimiting_dn5;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn5_slot = var_umaxpoweronepointfive_dn5;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn5_slot = var_wgamma_dn5;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn5_slot = var_wtat_dn5;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn5_slot = var_xerfc_dn5;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }
}
