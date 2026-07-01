#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_guard83: f64,
        var_psce1_i: f64,
        var_psce2_i: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac1_t_rv_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac2_t_rv_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfr_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_cov_i_rv_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covd_i_rv_slot: &mut f64,
        var_covdl_i_slot: &mut f64,
        var_covdl_i_rv_slot: &mut f64,
        var_covdlb_i_slot: &mut f64,
        var_covdlb_i_rv_slot: &mut f64,
        var_csd_i_slot: &mut f64,
        var_csd_i_rv_slot: &mut f64,
        var_csdbp_i_slot: &mut f64,
        var_csdbp_i_rv_slot: &mut f64,
        var_dvfbov_i_slot: &mut f64,
        var_dvfbov_i_rv_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fif_i_rv_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_fsceac_i_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac1_i_rv_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_psceac2_i_rv_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_rth_t_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac1_t_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac1_t_rv: f64 = *var_cfac1_t_rv_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac2_t_rv: f64 = *var_cfac2_t_rv_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfr_i_rv: f64 = *var_cfr_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_cov_i_rv: f64 = *var_cov_i_rv_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covd_i_rv: f64 = *var_covd_i_rv_slot;
        let mut var_covdl_i: f64 = *var_covdl_i_slot;
        let mut var_covdl_i_rv: f64 = *var_covdl_i_rv_slot;
        let mut var_covdlb_i: f64 = *var_covdlb_i_slot;
        let mut var_covdlb_i_rv: f64 = *var_covdlb_i_rv_slot;
        let mut var_csd_i: f64 = *var_csd_i_slot;
        let mut var_csd_i_rv: f64 = *var_csd_i_rv_slot;
        let mut var_csdbp_i: f64 = *var_csdbp_i_slot;
        let mut var_csdbp_i_rv: f64 = *var_csdbp_i_rv_slot;
        let mut var_dvfbov_i: f64 = *var_dvfbov_i_slot;
        let mut var_dvfbov_i_rv: f64 = *var_dvfbov_i_rv_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fif_i_rv: f64 = *var_fif_i_rv_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_fsceac_i_rv: f64 = *var_fsceac_i_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac1_i_rv: f64 = *var_psceac1_i_rv_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_psceac2_i_rv: f64 = *var_psceac2_i_rv_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_rth_t_rv: f64 = *var_rth_t_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac1_t_rv: f64 = *var_vfbac1_t_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;

        let (assign1450_e1498,) = {
    if (var_guard83 != 0.0) {
        (p.p154,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign1450_e1498;
        var_fif_i_rv = 0.0;

        let (assign1460_e1502,) = {
    if (var_guard83 != 0.0) {
        (p.p155,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign1460_e1502;
        var_fsceac_i_rv = 0.0;

        let (assign1470_e1506, assign1470_e1506_d_n4, assign1470_e1506_d_n6, assign1470_e1506_d_n7, assign1470_e1506_d_n8, assign1470_e1506_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1470_e1506;
        var_vfbac1_t_dn4 = assign1470_e1506_d_n4;
        var_vfbac1_t_dn6 = assign1470_e1506_d_n6;
        var_vfbac1_t_dn7 = assign1470_e1506_d_n7;
        var_vfbac1_t_dn8 = assign1470_e1506_d_n8;
        var_vfbac1_t_dn9 = assign1470_e1506_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign1480_e1510, assign1480_e1510_d_n4, assign1480_e1510_d_n6, assign1480_e1510_d_n7, assign1480_e1510_d_n8, assign1480_e1510_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1480_e1510;
        var_vfbac2_t_dn4 = assign1480_e1510_d_n4;
        var_vfbac2_t_dn6 = assign1480_e1510_d_n6;
        var_vfbac2_t_dn7 = assign1480_e1510_d_n7;
        var_vfbac2_t_dn8 = assign1480_e1510_d_n8;
        var_vfbac2_t_dn9 = assign1480_e1510_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign1490_e1514,) = {
    if (var_guard83 != 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1490_e1514;
        var_psceac1_i_rv = 0.0;

        let (assign1500_e1518,) = {
    if (var_guard83 != 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1500_e1518;
        var_psceac2_i_rv = 0.0;

        let (assign1510_e1522, assign1510_e1522_d_n4, assign1510_e1522_d_n6, assign1510_e1522_d_n7, assign1510_e1522_d_n8, assign1510_e1522_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1510_e1522;
        var_cfac1_t_dn4 = assign1510_e1522_d_n4;
        var_cfac1_t_dn6 = assign1510_e1522_d_n6;
        var_cfac1_t_dn7 = assign1510_e1522_d_n7;
        var_cfac1_t_dn8 = assign1510_e1522_d_n8;
        var_cfac1_t_dn9 = assign1510_e1522_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign1520_e1526, assign1520_e1526_d_n4, assign1520_e1526_d_n6, assign1520_e1526_d_n7, assign1520_e1526_d_n8, assign1520_e1526_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign1520_e1526;
        var_cfac2_t_dn4 = assign1520_e1526_d_n4;
        var_cfac2_t_dn6 = assign1520_e1526_d_n6;
        var_cfac2_t_dn7 = assign1520_e1526_d_n7;
        var_cfac2_t_dn8 = assign1520_e1526_d_n8;
        var_cfac2_t_dn9 = assign1520_e1526_d_n9;
        var_cfac2_t_rv = 0.0;

        let (assign1530_e1530, assign1530_e1530_d_n4, assign1530_e1530_d_n6, assign1530_e1530_d_n7, assign1530_e1530_d_n8, assign1530_e1530_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1530_e1530;
        var_thesatac_t_dn4 = assign1530_e1530_d_n4;
        var_thesatac_t_dn6 = assign1530_e1530_d_n6;
        var_thesatac_t_dn7 = assign1530_e1530_d_n7;
        var_thesatac_t_dn8 = assign1530_e1530_d_n8;
        var_thesatac_t_dn9 = assign1530_e1530_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign1540_e1534,) = {
    if (var_guard83 != 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1540_e1534;
        var_axac_i_rv = 0.0;

        let (assign1550_e1538,) = {
    if (var_guard83 != 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1550_e1538;
        var_alpac_i_rv = 0.0;

        let assign1560_e1541: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard86 = assign1560_e1541;
        var_guard86_rv = 0.0;

        let (assign1570_e1547, assign1570_e1547_d_n4, assign1570_e1547_d_n6, assign1570_e1547_d_n7, assign1570_e1547_d_n8, assign1570_e1547_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1570_e1547;
        var_vfbac1_t_dn4 = assign1570_e1547_d_n4;
        var_vfbac1_t_dn6 = assign1570_e1547_d_n6;
        var_vfbac1_t_dn7 = assign1570_e1547_d_n7;
        var_vfbac1_t_dn8 = assign1570_e1547_d_n8;
        var_vfbac1_t_dn9 = assign1570_e1547_d_n9;
        var_vfbac1_t_rv = 0.0;

        let assign1580_e1549: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1551: f64 = if assign1580_e1549 == 1.0 { 1.0 } else { 0.0 };
        var_guard87 = assign1580_e1551;
        var_guard87_rv = 0.0;

        let (assign1590_e1559, assign1590_e1559_d_n4, assign1590_e1559_d_n6, assign1590_e1559_d_n7, assign1590_e1559_d_n8, assign1590_e1559_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) {
        (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1590_e1559;
        var_vfbac1_t_dn4 = assign1590_e1559_d_n4;
        var_vfbac1_t_dn6 = assign1590_e1559_d_n6;
        var_vfbac1_t_dn7 = assign1590_e1559_d_n7;
        var_vfbac1_t_dn8 = assign1590_e1559_d_n8;
        var_vfbac1_t_dn9 = assign1590_e1559_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign1600_e1565, assign1600_e1565_d_n4, assign1600_e1565_d_n6, assign1600_e1565_d_n7, assign1600_e1565_d_n8, assign1600_e1565_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1600_e1565;
        var_vfbac2_t_dn4 = assign1600_e1565_d_n4;
        var_vfbac2_t_dn6 = assign1600_e1565_d_n6;
        var_vfbac2_t_dn7 = assign1600_e1565_d_n7;
        var_vfbac2_t_dn8 = assign1600_e1565_d_n8;
        var_vfbac2_t_dn9 = assign1600_e1565_d_n9;
        var_vfbac2_t_rv = 0.0;

        let assign1610_e1567: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1569: f64 = if assign1610_e1567 == 1.0 { 1.0 } else { 0.0 };
        var_guard88 = assign1610_e1569;
        var_guard88_rv = 0.0;

        let (assign1620_e1577, assign1620_e1577_d_n4, assign1620_e1577_d_n6, assign1620_e1577_d_n7, assign1620_e1577_d_n8, assign1620_e1577_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard88 != 0.0)) {
        (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1620_e1577;
        var_vfbac2_t_dn4 = assign1620_e1577_d_n4;
        var_vfbac2_t_dn6 = assign1620_e1577_d_n6;
        var_vfbac2_t_dn7 = assign1620_e1577_d_n7;
        var_vfbac2_t_dn8 = assign1620_e1577_d_n8;
        var_vfbac2_t_dn9 = assign1620_e1577_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign1630_e1583,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p57,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1630_e1583;
        var_psceac1_i_rv = 0.0;

        let assign1640_e1585: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1587: f64 = if assign1640_e1585 == 1.0 { 1.0 } else { 0.0 };
        var_guard89 = assign1640_e1587;
        var_guard89_rv = 0.0;

        let (assign1650_e1595,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard89 != 0.0)) {
        (p.p158,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1650_e1595;
        var_psceac1_i_rv = 0.0;

        let (assign1660_e1607,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1660_e1601: f64 = (p.p58 * var_psceac1_i);
        let assign1660_e1603: f64 = (assign1660_e1601 * var_tox2_i);
        let assign1660_e1605: f64 = (assign1660_e1603 / var_tox1_i);
        (assign1660_e1605,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1660_e1607;
        var_psceac2_i_rv = 0.0;

        let (assign1670_e1613, assign1670_e1613_d_n4, assign1670_e1613_d_n6, assign1670_e1613_d_n7, assign1670_e1613_d_n8, assign1670_e1613_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1670_e1613;
        var_cfac1_t_dn4 = assign1670_e1613_d_n4;
        var_cfac1_t_dn6 = assign1670_e1613_d_n6;
        var_cfac1_t_dn7 = assign1670_e1613_d_n7;
        var_cfac1_t_dn8 = assign1670_e1613_d_n8;
        var_cfac1_t_dn9 = assign1670_e1613_d_n9;
        var_cfac1_t_rv = 0.0;

        let assign1680_e1615: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1617: f64 = if assign1680_e1615 == 1.0 { 1.0 } else { 0.0 };
        var_guard90 = assign1680_e1617;
        var_guard90_rv = 0.0;

        let (assign1690_e1625, assign1690_e1625_d_n4, assign1690_e1625_d_n6, assign1690_e1625_d_n7, assign1690_e1625_d_n8, assign1690_e1625_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard90 != 0.0)) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1690_e1625;
        var_cfac1_t_dn4 = assign1690_e1625_d_n4;
        var_cfac1_t_dn6 = assign1690_e1625_d_n6;
        var_cfac1_t_dn7 = assign1690_e1625_d_n7;
        var_cfac1_t_dn8 = assign1690_e1625_d_n8;
        var_cfac1_t_dn9 = assign1690_e1625_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign1700_e1637, assign1700_e1637_d_n4, assign1700_e1637_d_n6, assign1700_e1637_d_n7, assign1700_e1637_d_n8, assign1700_e1637_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1700_e1631: f64 = (p.p63 * var_cfac1_t);
        let assign1700_e1633: f64 = (assign1700_e1631 * var_tox2_i);
        let assign1700_e1635: f64 = (assign1700_e1633 / var_tox1_i);
        (assign1700_e1635, (((p.p63 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign1700_e1637;
        var_cfac2_t_dn4 = assign1700_e1637_d_n4;
        var_cfac2_t_dn6 = assign1700_e1637_d_n6;
        var_cfac2_t_dn7 = assign1700_e1637_d_n7;
        var_cfac2_t_dn8 = assign1700_e1637_d_n8;
        var_cfac2_t_dn9 = assign1700_e1637_d_n9;
        var_cfac2_t_rv = 0.0;

        let (assign1710_e1643, assign1710_e1643_d_n4, assign1710_e1643_d_n6, assign1710_e1643_d_n7, assign1710_e1643_d_n8, assign1710_e1643_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1710_e1643;
        var_thesatac_t_dn4 = assign1710_e1643_d_n4;
        var_thesatac_t_dn6 = assign1710_e1643_d_n6;
        var_thesatac_t_dn7 = assign1710_e1643_d_n7;
        var_thesatac_t_dn8 = assign1710_e1643_d_n8;
        var_thesatac_t_dn9 = assign1710_e1643_d_n9;
        var_thesatac_t_rv = 0.0;

        let assign1720_e1645: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1647: f64 = if assign1720_e1645 == 1.0 { 1.0 } else { 0.0 };
        var_guard91 = assign1720_e1647;
        var_guard91_rv = 0.0;

        let (assign1730_e1655, assign1730_e1655_d_n4, assign1730_e1655_d_n6, assign1730_e1655_d_n7, assign1730_e1655_d_n8, assign1730_e1655_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard91 != 0.0)) {
        (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1730_e1655;
        var_thesatac_t_dn4 = assign1730_e1655_d_n4;
        var_thesatac_t_dn6 = assign1730_e1655_d_n6;
        var_thesatac_t_dn7 = assign1730_e1655_d_n7;
        var_thesatac_t_dn8 = assign1730_e1655_d_n8;
        var_thesatac_t_dn9 = assign1730_e1655_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign1740_e1661,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p97,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1740_e1661;
        var_axac_i_rv = 0.0;

        let assign1750_e1663: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1665: f64 = if assign1750_e1663 == 1.0 { 1.0 } else { 0.0 };
        var_guard92 = assign1750_e1665;
        var_guard92_rv = 0.0;

        let (assign1760_e1673,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard92 != 0.0)) {
        (p.p161,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1760_e1673;
        var_axac_i_rv = 0.0;

        let (assign1770_e1679,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p98,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1770_e1679;
        var_alpac_i_rv = 0.0;

        let assign1780_e1681: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1683: f64 = if assign1780_e1681 == 1.0 { 1.0 } else { 0.0 };
        var_guard93 = assign1780_e1683;
        var_guard93_rv = 0.0;

        let (assign1790_e1691,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard93 != 0.0)) {
        (p.p162,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1790_e1691;
        var_alpac_i_rv = 0.0;

        let (assign1800_e1695, assign1800_e1695_d_n4, assign1800_e1695_d_n6, assign1800_e1695_d_n7, assign1800_e1695_d_n8, assign1800_e1695_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    }
};
        var_cov_i = assign1800_e1695;
        var_cov_i_dn4 = assign1800_e1695_d_n4;
        var_cov_i_dn6 = assign1800_e1695_d_n6;
        var_cov_i_dn7 = assign1800_e1695_d_n7;
        var_cov_i_dn8 = assign1800_e1695_d_n8;
        var_cov_i_dn9 = assign1800_e1695_d_n9;
        var_cov_i_rv = 0.0;

        let (assign1810_e1699, assign1810_e1699_d_n4, assign1810_e1699_d_n6, assign1810_e1699_d_n7, assign1810_e1699_d_n8, assign1810_e1699_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign1810_e1699;
        var_covd_i_dn4 = assign1810_e1699_d_n4;
        var_covd_i_dn6 = assign1810_e1699_d_n6;
        var_covd_i_dn7 = assign1810_e1699_d_n7;
        var_covd_i_dn8 = assign1810_e1699_d_n8;
        var_covd_i_dn9 = assign1810_e1699_d_n9;
        var_covd_i_rv = 0.0;

        let (assign1820_e1703,) = {
    if (var_guard83 != 0.0) {
        (p.p165,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign1820_e1703;
        var_covdl_i_rv = 0.0;

        let (assign1830_e1707,) = {
    if (var_guard83 != 0.0) {
        (p.p166,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign1830_e1707;
        var_covdlb_i_rv = 0.0;

        let (assign1840_e1711,) = {
    if (var_guard83 != 0.0) {
        (p.p167,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign1840_e1711;
        var_dvfbov_i_rv = 0.0;

        let (assign1850_e1715, assign1850_e1715_d_n4, assign1850_e1715_d_n6, assign1850_e1715_d_n7, assign1850_e1715_d_n8, assign1850_e1715_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    }
};
        var_cfr_i = assign1850_e1715;
        var_cfr_i_dn4 = assign1850_e1715_d_n4;
        var_cfr_i_dn6 = assign1850_e1715_d_n6;
        var_cfr_i_dn7 = assign1850_e1715_d_n7;
        var_cfr_i_dn8 = assign1850_e1715_d_n8;
        var_cfr_i_dn9 = assign1850_e1715_d_n9;
        var_cfr_i_rv = 0.0;

        let (assign1860_e1719, assign1860_e1719_d_n4, assign1860_e1719_d_n6, assign1860_e1719_d_n7, assign1860_e1719_d_n8, assign1860_e1719_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign1860_e1719;
        var_cfrd_i_dn4 = assign1860_e1719_d_n4;
        var_cfrd_i_dn6 = assign1860_e1719_d_n6;
        var_cfrd_i_dn7 = assign1860_e1719_d_n7;
        var_cfrd_i_dn8 = assign1860_e1719_d_n8;
        var_cfrd_i_dn9 = assign1860_e1719_d_n9;
        var_cfrd_i_rv = 0.0;

        let (assign1870_e1723,) = {
    if (var_guard83 != 0.0) {
        (p.p170,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign1870_e1723;
        var_csd_i_rv = 0.0;

        let (assign1880_e1727,) = {
    if (var_guard83 != 0.0) {
        (p.p171,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign1880_e1727;
        var_csdbp_i_rv = 0.0;

        let (assign1890_e1731, assign1890_e1731_d_n4, assign1890_e1731_d_n6, assign1890_e1731_d_n7, assign1890_e1731_d_n8, assign1890_e1731_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rth_t, var_rth_t_dn4, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9,)
    }
};
        var_rth_t = assign1890_e1731;
        var_rth_t_dn4 = assign1890_e1731_d_n4;
        var_rth_t_dn6 = assign1890_e1731_d_n6;
        var_rth_t_dn7 = assign1890_e1731_d_n7;
        var_rth_t_dn8 = assign1890_e1731_d_n8;
        var_rth_t_dn9 = assign1890_e1731_d_n9;
        var_rth_t_rv = 0.0;

        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac1_t_rv_slot = var_cfac1_t_rv;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac2_t_rv_slot = var_cfac2_t_rv;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfr_i_rv_slot = var_cfr_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_cov_i_rv_slot = var_cov_i_rv;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covd_i_rv_slot = var_covd_i_rv;
        *var_covdl_i_slot = var_covdl_i;
        *var_covdl_i_rv_slot = var_covdl_i_rv;
        *var_covdlb_i_slot = var_covdlb_i;
        *var_covdlb_i_rv_slot = var_covdlb_i_rv;
        *var_csd_i_slot = var_csd_i;
        *var_csd_i_rv_slot = var_csd_i_rv;
        *var_csdbp_i_slot = var_csdbp_i;
        *var_csdbp_i_rv_slot = var_csdbp_i_rv;
        *var_dvfbov_i_slot = var_dvfbov_i;
        *var_dvfbov_i_rv_slot = var_dvfbov_i_rv;
        *var_fif_i_slot = var_fif_i;
        *var_fif_i_rv_slot = var_fif_i_rv;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_fsceac_i_rv_slot = var_fsceac_i_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac1_i_rv_slot = var_psceac1_i_rv;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_psceac2_i_rv_slot = var_psceac2_i_rv;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_rth_t_rv_slot = var_rth_t_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac1_t_rv_slot = var_vfbac1_t_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_guard83: f64,
        var_adrain_i_slot: &mut f64,
        var_adrain_i_rv_slot: &mut f64,
        var_asource_i_slot: &mut f64,
        var_asource_i_rv_slot: &mut f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_cth_i_rv_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_dellps_rv_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_delwod_rv_slot: &mut f64,
        var_fracinv_i_slot: &mut f64,
        var_fracinv_i_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iae_rv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_il_rv_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iw_rv_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_iwe_rv_slot: &mut f64,
        var_kdiff_i_slot: &mut f64,
        var_kdiff_i_dn4_slot: &mut f64,
        var_kdiff_i_dn6_slot: &mut f64,
        var_kdiff_i_dn7_slot: &mut f64,
        var_kdiff_i_dn8_slot: &mut f64,
        var_kdiff_i_dn9_slot: &mut f64,
        var_kdiff_i_rv_slot: &mut f64,
        var_kdrift_i_slot: &mut f64,
        var_kdrift_i_dn4_slot: &mut f64,
        var_kdrift_i_dn6_slot: &mut f64,
        var_kdrift_i_dn7_slot: &mut f64,
        var_kdrift_i_dn8_slot: &mut f64,
        var_kdrift_i_dn9_slot: &mut f64,
        var_kdrift_i_rv_slot: &mut f64,
        var_kfracinv_i_slot: &mut f64,
        var_kfracinv_i_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_lecv_rv_slot: &mut f64,
        var_len_slot: &mut f64,
        var_len_rv_slot: &mut f64,
        var_lphy_slot: &mut f64,
        var_lphy_dn4_slot: &mut f64,
        var_lphy_dn6_slot: &mut f64,
        var_lphy_dn7_slot: &mut f64,
        var_lphy_dn8_slot: &mut f64,
        var_lphy_dn9_slot: &mut f64,
        var_lphy_rv_slot: &mut f64,
        var_mult_i_int_slot: &mut f64,
        var_mult_i_int_rv_slot: &mut f64,
        var_nch_i_slot: &mut f64,
        var_nch_i_rv_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfa_i_rv_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfb_i_rv_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfc_i_rv_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfe_i_rv_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_nfeb_i_rv_slot: &mut f64,
        var_pdrain_i_slot: &mut f64,
        var_pdrain_i_rv_slot: &mut f64,
        var_psource_i_slot: &mut f64,
        var_psource_i_rv_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_strth_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tox1_i_slot: &mut f64,
        var_tox1_i_rv_slot: &mut f64,
        var_tox2_i_slot: &mut f64,
        var_tox2_i_rv_slot: &mut f64,
        var_tsi_i_slot: &mut f64,
        var_tsi_i_rv_slot: &mut f64,
        var_typech_i_slot: &mut f64,
        var_typech_i_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wecv_rv_slot: &mut f64,
        var_wen_slot: &mut f64,
        var_wen_rv_slot: &mut f64,
        var_wphy_slot: &mut f64,
        var_wphy_dn4_slot: &mut f64,
        var_wphy_dn6_slot: &mut f64,
        var_wphy_dn7_slot: &mut f64,
        var_wphy_dn8_slot: &mut f64,
        var_wphy_dn9_slot: &mut f64,
        var_wphy_rv_slot: &mut f64,
        var_xge_i_slot: &mut f64,
        var_xge_i_rv_slot: &mut f64,
    ) {
        let mut var_adrain_i: f64 = *var_adrain_i_slot;
        let mut var_adrain_i_rv: f64 = *var_adrain_i_rv_slot;
        let mut var_asource_i: f64 = *var_asource_i_slot;
        let mut var_asource_i_rv: f64 = *var_asource_i_rv_slot;
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_cth_i_rv: f64 = *var_cth_i_rv_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_dellps_rv: f64 = *var_dellps_rv_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_delwod_rv: f64 = *var_delwod_rv_slot;
        let mut var_fracinv_i: f64 = *var_fracinv_i_slot;
        let mut var_fracinv_i_rv: f64 = *var_fracinv_i_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iae_rv: f64 = *var_iae_rv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_il_rv: f64 = *var_il_rv_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile_rv: f64 = *var_ile_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iw_rv: f64 = *var_iw_rv_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_iwe_rv: f64 = *var_iwe_rv_slot;
        let mut var_kdiff_i: f64 = *var_kdiff_i_slot;
        let mut var_kdiff_i_dn4: f64 = *var_kdiff_i_dn4_slot;
        let mut var_kdiff_i_dn6: f64 = *var_kdiff_i_dn6_slot;
        let mut var_kdiff_i_dn7: f64 = *var_kdiff_i_dn7_slot;
        let mut var_kdiff_i_dn8: f64 = *var_kdiff_i_dn8_slot;
        let mut var_kdiff_i_dn9: f64 = *var_kdiff_i_dn9_slot;
        let mut var_kdiff_i_rv: f64 = *var_kdiff_i_rv_slot;
        let mut var_kdrift_i: f64 = *var_kdrift_i_slot;
        let mut var_kdrift_i_dn4: f64 = *var_kdrift_i_dn4_slot;
        let mut var_kdrift_i_dn6: f64 = *var_kdrift_i_dn6_slot;
        let mut var_kdrift_i_dn7: f64 = *var_kdrift_i_dn7_slot;
        let mut var_kdrift_i_dn8: f64 = *var_kdrift_i_dn8_slot;
        let mut var_kdrift_i_dn9: f64 = *var_kdrift_i_dn9_slot;
        let mut var_kdrift_i_rv: f64 = *var_kdrift_i_rv_slot;
        let mut var_kfracinv_i: f64 = *var_kfracinv_i_slot;
        let mut var_kfracinv_i_rv: f64 = *var_kfracinv_i_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_lecv_rv: f64 = *var_lecv_rv_slot;
        let mut var_len: f64 = *var_len_slot;
        let mut var_len_rv: f64 = *var_len_rv_slot;
        let mut var_lphy: f64 = *var_lphy_slot;
        let mut var_lphy_dn4: f64 = *var_lphy_dn4_slot;
        let mut var_lphy_dn6: f64 = *var_lphy_dn6_slot;
        let mut var_lphy_dn7: f64 = *var_lphy_dn7_slot;
        let mut var_lphy_dn8: f64 = *var_lphy_dn8_slot;
        let mut var_lphy_dn9: f64 = *var_lphy_dn9_slot;
        let mut var_lphy_rv: f64 = *var_lphy_rv_slot;
        let mut var_mult_i_int: f64 = *var_mult_i_int_slot;
        let mut var_mult_i_int_rv: f64 = *var_mult_i_int_rv_slot;
        let mut var_nch_i: f64 = *var_nch_i_slot;
        let mut var_nch_i_rv: f64 = *var_nch_i_rv_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfa_i_rv: f64 = *var_nfa_i_rv_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfb_i_rv: f64 = *var_nfb_i_rv_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfc_i_rv: f64 = *var_nfc_i_rv_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfe_i_rv: f64 = *var_nfe_i_rv_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_nfeb_i_rv: f64 = *var_nfeb_i_rv_slot;
        let mut var_pdrain_i: f64 = *var_pdrain_i_slot;
        let mut var_pdrain_i_rv: f64 = *var_pdrain_i_rv_slot;
        let mut var_psource_i: f64 = *var_psource_i_slot;
        let mut var_psource_i_rv: f64 = *var_psource_i_rv_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_strth_i_rv: f64 = *var_strth_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tox1_i: f64 = *var_tox1_i_slot;
        let mut var_tox1_i_rv: f64 = *var_tox1_i_rv_slot;
        let mut var_tox2_i: f64 = *var_tox2_i_slot;
        let mut var_tox2_i_rv: f64 = *var_tox2_i_rv_slot;
        let mut var_tsi_i: f64 = *var_tsi_i_slot;
        let mut var_tsi_i_rv: f64 = *var_tsi_i_rv_slot;
        let mut var_typech_i: f64 = *var_typech_i_slot;
        let mut var_typech_i_rv: f64 = *var_typech_i_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wecv_rv: f64 = *var_wecv_rv_slot;
        let mut var_wen: f64 = *var_wen_slot;
        let mut var_wen_rv: f64 = *var_wen_rv_slot;
        let mut var_wphy: f64 = *var_wphy_slot;
        let mut var_wphy_dn4: f64 = *var_wphy_dn4_slot;
        let mut var_wphy_dn6: f64 = *var_wphy_dn6_slot;
        let mut var_wphy_dn7: f64 = *var_wphy_dn7_slot;
        let mut var_wphy_dn8: f64 = *var_wphy_dn8_slot;
        let mut var_wphy_dn9: f64 = *var_wphy_dn9_slot;
        let mut var_wphy_rv: f64 = *var_wphy_rv_slot;
        let mut var_xge_i: f64 = *var_xge_i_slot;
        let mut var_xge_i_rv: f64 = *var_xge_i_rv_slot;

        let (assign1900_e1735,) = {
    if (var_guard83 != 0.0) {
        (p.p173,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign1900_e1735;
        var_strth_i_rv = 0.0;

        let (assign1910_e1739, assign1910_e1739_d_n4, assign1910_e1739_d_n6, assign1910_e1739_d_n7, assign1910_e1739_d_n8, assign1910_e1739_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cth_i, var_cth_i_dn4, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9,)
    }
};
        var_cth_i = assign1910_e1739;
        var_cth_i_dn4 = assign1910_e1739_d_n4;
        var_cth_i_dn6 = assign1910_e1739_d_n6;
        var_cth_i_dn7 = assign1910_e1739_d_n7;
        var_cth_i_dn8 = assign1910_e1739_d_n8;
        var_cth_i_dn9 = assign1910_e1739_d_n9;
        var_cth_i_rv = 0.0;

        let (assign1940_e1751,) = {
    if (var_guard83 != 0.0) {
        (p.p177,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign1940_e1751;
        var_nfa_i_rv = 0.0;

        let (assign1950_e1755,) = {
    if (var_guard83 != 0.0) {
        (p.p178,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign1950_e1755;
        var_nfb_i_rv = 0.0;

        let (assign1960_e1759,) = {
    if (var_guard83 != 0.0) {
        (p.p179,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign1960_e1759;
        var_nfc_i_rv = 0.0;

        let (assign1970_e1763,) = {
    if (var_guard83 != 0.0) {
        (p.p180,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign1970_e1763;
        var_nfe_i_rv = 0.0;

        let (assign1980_e1767,) = {
    if (var_guard83 != 0.0) {
        (p.p181,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign1980_e1767;
        var_nfeb_i_rv = 0.0;

        let (assign2000_e1775, assign2000_e1775_d_n4, assign2000_e1775_d_n6, assign2000_e1775_d_n7, assign2000_e1775_d_n8, assign2000_e1775_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p183, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kdrift_i, var_kdrift_i_dn4, var_kdrift_i_dn6, var_kdrift_i_dn7, var_kdrift_i_dn8, var_kdrift_i_dn9,)
    }
};
        var_kdrift_i = assign2000_e1775;
        var_kdrift_i_dn4 = assign2000_e1775_d_n4;
        var_kdrift_i_dn6 = assign2000_e1775_d_n6;
        var_kdrift_i_dn7 = assign2000_e1775_d_n7;
        var_kdrift_i_dn8 = assign2000_e1775_d_n8;
        var_kdrift_i_dn9 = assign2000_e1775_d_n9;
        var_kdrift_i_rv = 0.0;

        let (assign2010_e1779, assign2010_e1779_d_n4, assign2010_e1779_d_n6, assign2010_e1779_d_n7, assign2010_e1779_d_n8, assign2010_e1779_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p184, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kdiff_i, var_kdiff_i_dn4, var_kdiff_i_dn6, var_kdiff_i_dn7, var_kdiff_i_dn8, var_kdiff_i_dn9,)
    }
};
        var_kdiff_i = assign2010_e1779;
        var_kdiff_i_dn4 = assign2010_e1779_d_n4;
        var_kdiff_i_dn6 = assign2010_e1779_d_n6;
        var_kdiff_i_dn7 = assign2010_e1779_d_n7;
        var_kdiff_i_dn8 = assign2010_e1779_d_n8;
        var_kdiff_i_dn9 = assign2010_e1779_d_n9;
        var_kdiff_i_rv = 0.0;

        let (assign2020_e1783,) = {
    if (var_guard83 != 0.0) {
        (p.p185,)
    } else {
        (var_fracinv_i,)
    }
};
        var_fracinv_i = assign2020_e1783;
        var_fracinv_i_rv = 0.0;

        let (assign2030_e1787,) = {
    if (var_guard83 != 0.0) {
        (p.p186,)
    } else {
        (var_kfracinv_i,)
    }
};
        var_kfracinv_i = assign2030_e1787;
        var_kfracinv_i_rv = 0.0;

        let (assign2080_e1810,) = {
    if (var_guard83 == 0.0) {
        let assign2080_e1808: f64 = (1.0 / p.p29);
        (assign2080_e1808,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign2080_e1810;
        var_invnf_rv = 0.0;

        let (assign2090_e1819,) = {
    if (var_guard83 == 0.0) {
        let assign2090_e1815: f64 = (p.p21 * var_invnf);
        let assign2090_e1817: f64 = (assign2090_e1815).max(1e-9);
        (assign2090_e1817,)
    } else {
        (var_w_i,)
    }
};
        var_w_i = assign2090_e1819;
        var_w_i_rv = 0.0;

        let (assign2100_e1826,) = {
    if (var_guard83 == 0.0) {
        let assign2100_e1824: f64 = (p.p23 * var_invnf);
        (assign2100_e1824,)
    } else {
        (var_adrain_i,)
    }
};
        var_adrain_i = assign2100_e1826;
        var_adrain_i_rv = 0.0;

        let (assign2110_e1833,) = {
    if (var_guard83 == 0.0) {
        let assign2110_e1831: f64 = (p.p22 * var_invnf);
        (assign2110_e1831,)
    } else {
        (var_asource_i,)
    }
};
        var_asource_i = assign2110_e1833;
        var_asource_i_rv = 0.0;

        let (assign2120_e1840,) = {
    if (var_guard83 == 0.0) {
        let assign2120_e1838: f64 = (p.p25 * var_invnf);
        (assign2120_e1838,)
    } else {
        (var_pdrain_i,)
    }
};
        var_pdrain_i = assign2120_e1840;
        var_pdrain_i_rv = 0.0;

        let (assign2130_e1847,) = {
    if (var_guard83 == 0.0) {
        let assign2130_e1845: f64 = (p.p24 * var_invnf);
        (assign2130_e1845,)
    } else {
        (var_psource_i,)
    }
};
        var_psource_i = assign2130_e1847;
        var_psource_i_rv = 0.0;

        let (assign2140_e1854,) = {
    if (var_guard83 == 0.0) {
        let assign2140_e1852: f64 = (p.p30 * p.p29);
        (assign2140_e1852,)
    } else {
        (var_mult_i_int,)
    }
};
        var_mult_i_int = assign2140_e1854;
        var_mult_i_int_rv = 0.0;

        let (assign2150_e1859,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_len,)
    }
};
        var_len = assign2150_e1859;
        var_len_rv = 0.0;

        let (assign2160_e1864,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_wen,)
    }
};
        var_wen = assign2160_e1864;
        var_wen_rv = 0.0;

        let (assign2170_e1871,) = {
    if (var_guard83 == 0.0) {
        let assign2170_e1869: f64 = (var_len / p.p20);
        (assign2170_e1869,)
    } else {
        (var_il,)
    }
};
        var_il = assign2170_e1871;
        var_il_rv = 0.0;

        let (assign2180_e1878,) = {
    if (var_guard83 == 0.0) {
        let assign2180_e1876: f64 = (var_wen / var_w_i);
        (assign2180_e1876,)
    } else {
        (var_iw,)
    }
};
        var_iw = assign2180_e1878;
        var_iw_rv = 0.0;

        let (assign2190_e1895,) = {
    if (var_guard83 == 0.0) {
        let assign2190_e1885: f64 = (p.p192 * var_il);
        let assign2190_e1886: f64 = (1.0 + assign2190_e1885);
        let assign2190_e1887: f64 = (p.p191 * assign2190_e1886);
        let assign2190_e1891: f64 = (p.p193 * var_iw);
        let assign2190_e1892: f64 = (1.0 + assign2190_e1891);
        let assign2190_e1893: f64 = (assign2190_e1887 * assign2190_e1892);
        (assign2190_e1893,)
    } else {
        (var_dellps,)
    }
};
        var_dellps = assign2190_e1895;
        var_dellps_rv = 0.0;

        let (assign2200_e1912,) = {
    if (var_guard83 == 0.0) {
        let assign2200_e1902: f64 = (p.p197 * var_iw);
        let assign2200_e1903: f64 = (1.0 + assign2200_e1902);
        let assign2200_e1904: f64 = (p.p195 * assign2200_e1903);
        let assign2200_e1908: f64 = (p.p196 * var_il);
        let assign2200_e1909: f64 = (1.0 + assign2200_e1908);
        let assign2200_e1910: f64 = (assign2200_e1904 * assign2200_e1909);
        (assign2200_e1910,)
    } else {
        (var_delwod,)
    }
};
        var_delwod = assign2200_e1912;
        var_delwod_rv = 0.0;

        let (assign2210_e1925,) = {
    if (var_guard83 == 0.0) {
        let assign2210_e1917: f64 = (p.p20 + var_dellps);
        let assign2210_e1920: f64 = (2.0 * p.p194);
        let assign2210_e1921: f64 = (assign2210_e1917 - assign2210_e1920);
        let assign2210_e1923: f64 = (assign2210_e1921).max(1e-9);
        (assign2210_e1923,)
    } else {
        (var_le,)
    }
};
        var_le = assign2210_e1925;
        var_le_rv = 0.0;

        let (assign2220_e1938,) = {
    if (var_guard83 == 0.0) {
        let assign2220_e1930: f64 = (var_w_i + var_delwod);
        let assign2220_e1933: f64 = (2.0 * p.p198);
        let assign2220_e1934: f64 = (assign2220_e1930 - assign2220_e1933);
        let assign2220_e1936: f64 = (assign2220_e1934).max(1e-9);
        (assign2220_e1936,)
    } else {
        (var_we,)
    }
};
        var_we = assign2220_e1938;
        var_we_rv = 0.0;

        let (assign2230_e1953,) = {
    if (var_guard83 == 0.0) {
        let assign2230_e1943: f64 = (p.p20 + var_dellps);
        let assign2230_e1946: f64 = (2.0 * p.p194);
        let assign2230_e1947: f64 = (assign2230_e1943 - assign2230_e1946);
        let assign2230_e1949: f64 = (assign2230_e1947 + p.p199);
        let assign2230_e1951: f64 = (assign2230_e1949).max(1e-9);
        (assign2230_e1951,)
    } else {
        (var_lecv,)
    }
};
        var_lecv = assign2230_e1953;
        var_lecv_rv = 0.0;

        let (assign2240_e1968,) = {
    if (var_guard83 == 0.0) {
        let assign2240_e1958: f64 = (var_w_i + var_delwod);
        let assign2240_e1961: f64 = (2.0 * p.p198);
        let assign2240_e1962: f64 = (assign2240_e1958 - assign2240_e1961);
        let assign2240_e1964: f64 = (assign2240_e1962 + p.p200);
        let assign2240_e1966: f64 = (assign2240_e1964).max(1e-9);
        (assign2240_e1966,)
    } else {
        (var_wecv,)
    }
};
        var_wecv = assign2240_e1968;
        var_wecv_rv = 0.0;

        let (assign2250_e1975,) = {
    if (var_guard83 == 0.0) {
        let assign2250_e1973: f64 = (var_len / var_le);
        (assign2250_e1973,)
    } else {
        (var_ile,)
    }
};
        var_ile = assign2250_e1975;
        var_ile_rv = 0.0;

        let (assign2260_e1982,) = {
    if (var_guard83 == 0.0) {
        let assign2260_e1980: f64 = (var_wen / var_we);
        (assign2260_e1980,)
    } else {
        (var_iwe,)
    }
};
        var_iwe = assign2260_e1982;
        var_iwe_rv = 0.0;

        let (assign2270_e1989,) = {
    if (var_guard83 == 0.0) {
        let assign2270_e1987: f64 = (var_ile * var_iwe);
        (assign2270_e1987,)
    } else {
        (var_iae,)
    }
};
        var_iae = assign2270_e1989;
        var_iae_rv = 0.0;

        let (assign2280_e1998, assign2280_e1998_d_n4, assign2280_e1998_d_n6, assign2280_e1998_d_n7, assign2280_e1998_d_n8, assign2280_e1998_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2280_e1994: f64 = (p.p20 + var_dellps);
        let assign2280_e1996: f64 = (assign2280_e1994).max(1e-9);
        (assign2280_e1996, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2280_e1998;
        var_temp_dn4 = assign2280_e1998_d_n4;
        var_temp_dn6 = assign2280_e1998_d_n6;
        var_temp_dn7 = assign2280_e1998_d_n7;
        var_temp_dn8 = assign2280_e1998_d_n8;
        var_temp_dn9 = assign2280_e1998_d_n9;
        var_temp_rv = 0.0;

        let (assign2290_e2005, assign2290_e2005_d_n4, assign2290_e2005_d_n6, assign2290_e2005_d_n7, assign2290_e2005_d_n8, assign2290_e2005_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2290_e2003: f64 = (var_temp / var_len);
        (assign2290_e2003, (var_temp_dn4 / var_len), (var_temp_dn6 / var_len), (var_temp_dn7 / var_len), (var_temp_dn8 / var_len), (var_temp_dn9 / var_len),)
    } else {
        (var_lphy, var_lphy_dn4, var_lphy_dn6, var_lphy_dn7, var_lphy_dn8, var_lphy_dn9,)
    }
};
        var_lphy = assign2290_e2005;
        var_lphy_dn4 = assign2290_e2005_d_n4;
        var_lphy_dn6 = assign2290_e2005_d_n6;
        var_lphy_dn7 = assign2290_e2005_d_n7;
        var_lphy_dn8 = assign2290_e2005_d_n8;
        var_lphy_dn9 = assign2290_e2005_d_n9;
        var_lphy_rv = 0.0;

        let (assign2300_e2014, assign2300_e2014_d_n4, assign2300_e2014_d_n6, assign2300_e2014_d_n7, assign2300_e2014_d_n8, assign2300_e2014_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2300_e2010: f64 = (var_w_i + var_delwod);
        let assign2300_e2012: f64 = (assign2300_e2010).max(1e-9);
        (assign2300_e2012, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2300_e2014;
        var_temp_dn4 = assign2300_e2014_d_n4;
        var_temp_dn6 = assign2300_e2014_d_n6;
        var_temp_dn7 = assign2300_e2014_d_n7;
        var_temp_dn8 = assign2300_e2014_d_n8;
        var_temp_dn9 = assign2300_e2014_d_n9;
        var_temp_rv = 0.0;

        let (assign2310_e2021, assign2310_e2021_d_n4, assign2310_e2021_d_n6, assign2310_e2021_d_n7, assign2310_e2021_d_n8, assign2310_e2021_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2310_e2019: f64 = (var_temp / var_wen);
        (assign2310_e2019, (var_temp_dn4 / var_wen), (var_temp_dn6 / var_wen), (var_temp_dn7 / var_wen), (var_temp_dn8 / var_wen), (var_temp_dn9 / var_wen),)
    } else {
        (var_wphy, var_wphy_dn4, var_wphy_dn6, var_wphy_dn7, var_wphy_dn8, var_wphy_dn9,)
    }
};
        var_wphy = assign2310_e2021;
        var_wphy_dn4 = assign2310_e2021_d_n4;
        var_wphy_dn6 = assign2310_e2021_d_n6;
        var_wphy_dn7 = assign2310_e2021_d_n7;
        var_wphy_dn8 = assign2310_e2021_d_n8;
        var_wphy_dn9 = assign2310_e2021_d_n9;
        var_wphy_rv = 0.0;

        let (assign2360_e2064,) = {
    if (var_guard83 == 0.0) {
        (p.p201,)
    } else {
        (var_tox1_i,)
    }
};
        var_tox1_i = assign2360_e2064;
        var_tox1_i_rv = 0.0;

        let (assign2370_e2069,) = {
    if (var_guard83 == 0.0) {
        (p.p202,)
    } else {
        (var_tsi_i,)
    }
};
        var_tsi_i = assign2370_e2069;
        var_tsi_i_rv = 0.0;

        let (assign2380_e2074,) = {
    if (var_guard83 == 0.0) {
        (p.p203,)
    } else {
        (var_xge_i,)
    }
};
        var_xge_i = assign2380_e2074;
        var_xge_i_rv = 0.0;

        let (assign2390_e2079,) = {
    if (var_guard83 == 0.0) {
        (p.p204,)
    } else {
        (var_tox2_i,)
    }
};
        var_tox2_i = assign2390_e2079;
        var_tox2_i_rv = 0.0;

        let (assign2400_e2084,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2400_e2084;
        var_typech_i_rv = 0.0;

        let assign2410_e2087: f64 = if p.p205 < 0.0 { 1.0 } else { 0.0 };
        var_guard94 = assign2410_e2087;
        var_guard94_rv = 0.0;

        let (assign2420_e2095,) = {
    if ((var_guard83 == 0.0) && (var_guard94 != 0.0)) {
        let assign2420_e2093: f64 = (-1.0);
        (assign2420_e2093,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2420_e2095;
        var_typech_i_rv = 0.0;

        let (assign2430_e2105,) = {
    if (var_guard83 == 0.0) {
        let assign2430_e2099: f64 = (p.p205).abs();
        let assign2430_e2101: f64 = (assign2430_e2099).min(1e19);
        let assign2430_e2103: f64 = (assign2430_e2101 * 1000000.0);
        (assign2430_e2103,)
    } else {
        (var_nch_i,)
    }
};
        var_nch_i = assign2430_e2105;
        var_nch_i_rv = 0.0;

        *var_adrain_i_slot = var_adrain_i;
        *var_adrain_i_rv_slot = var_adrain_i_rv;
        *var_asource_i_slot = var_asource_i;
        *var_asource_i_rv_slot = var_asource_i_rv;
        *var_cth_i_slot = var_cth_i;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_cth_i_rv_slot = var_cth_i_rv;
        *var_dellps_slot = var_dellps;
        *var_dellps_rv_slot = var_dellps_rv;
        *var_delwod_slot = var_delwod;
        *var_delwod_rv_slot = var_delwod_rv;
        *var_fracinv_i_slot = var_fracinv_i;
        *var_fracinv_i_rv_slot = var_fracinv_i_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_iae_slot = var_iae;
        *var_iae_rv_slot = var_iae_rv;
        *var_il_slot = var_il;
        *var_il_rv_slot = var_il_rv;
        *var_ile_slot = var_ile;
        *var_ile_rv_slot = var_ile_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_iw_slot = var_iw;
        *var_iw_rv_slot = var_iw_rv;
        *var_iwe_slot = var_iwe;
        *var_iwe_rv_slot = var_iwe_rv;
        *var_kdiff_i_slot = var_kdiff_i;
        *var_kdiff_i_dn4_slot = var_kdiff_i_dn4;
        *var_kdiff_i_dn6_slot = var_kdiff_i_dn6;
        *var_kdiff_i_dn7_slot = var_kdiff_i_dn7;
        *var_kdiff_i_dn8_slot = var_kdiff_i_dn8;
        *var_kdiff_i_dn9_slot = var_kdiff_i_dn9;
        *var_kdiff_i_rv_slot = var_kdiff_i_rv;
        *var_kdrift_i_slot = var_kdrift_i;
        *var_kdrift_i_dn4_slot = var_kdrift_i_dn4;
        *var_kdrift_i_dn6_slot = var_kdrift_i_dn6;
        *var_kdrift_i_dn7_slot = var_kdrift_i_dn7;
        *var_kdrift_i_dn8_slot = var_kdrift_i_dn8;
        *var_kdrift_i_dn9_slot = var_kdrift_i_dn9;
        *var_kdrift_i_rv_slot = var_kdrift_i_rv;
        *var_kfracinv_i_slot = var_kfracinv_i;
        *var_kfracinv_i_rv_slot = var_kfracinv_i_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_lecv_slot = var_lecv;
        *var_lecv_rv_slot = var_lecv_rv;
        *var_len_slot = var_len;
        *var_len_rv_slot = var_len_rv;
        *var_lphy_slot = var_lphy;
        *var_lphy_dn4_slot = var_lphy_dn4;
        *var_lphy_dn6_slot = var_lphy_dn6;
        *var_lphy_dn7_slot = var_lphy_dn7;
        *var_lphy_dn8_slot = var_lphy_dn8;
        *var_lphy_dn9_slot = var_lphy_dn9;
        *var_lphy_rv_slot = var_lphy_rv;
        *var_mult_i_int_slot = var_mult_i_int;
        *var_mult_i_int_rv_slot = var_mult_i_int_rv;
        *var_nch_i_slot = var_nch_i;
        *var_nch_i_rv_slot = var_nch_i_rv;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfa_i_rv_slot = var_nfa_i_rv;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfb_i_rv_slot = var_nfb_i_rv;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfc_i_rv_slot = var_nfc_i_rv;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfe_i_rv_slot = var_nfe_i_rv;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_nfeb_i_rv_slot = var_nfeb_i_rv;
        *var_pdrain_i_slot = var_pdrain_i;
        *var_pdrain_i_rv_slot = var_pdrain_i_rv;
        *var_psource_i_slot = var_psource_i;
        *var_psource_i_rv_slot = var_psource_i_rv;
        *var_strth_i_slot = var_strth_i;
        *var_strth_i_rv_slot = var_strth_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_tox1_i_slot = var_tox1_i;
        *var_tox1_i_rv_slot = var_tox1_i_rv;
        *var_tox2_i_slot = var_tox2_i;
        *var_tox2_i_rv_slot = var_tox2_i_rv;
        *var_tsi_i_slot = var_tsi_i;
        *var_tsi_i_rv_slot = var_tsi_i_rv;
        *var_typech_i_slot = var_typech_i;
        *var_typech_i_rv_slot = var_typech_i_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
        *var_wecv_slot = var_wecv;
        *var_wecv_rv_slot = var_wecv_rv;
        *var_wen_slot = var_wen;
        *var_wen_rv_slot = var_wen_rv;
        *var_wphy_slot = var_wphy;
        *var_wphy_dn4_slot = var_wphy_dn4;
        *var_wphy_dn6_slot = var_wphy_dn6;
        *var_wphy_dn7_slot = var_wphy_dn7;
        *var_wphy_dn8_slot = var_wphy_dn8;
        *var_wphy_dn9_slot = var_wphy_dn9;
        *var_wphy_rv_slot = var_wphy_rv;
        *var_xge_i_slot = var_xge_i;
        *var_xge_i_rv_slot = var_xge_i_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_tsi_i: f64,
        var_xge_i: f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf1_t_rv_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf2_t_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cfdl_i_slot: &mut f64,
        var_cfdl_i_rv_slot: &mut f64,
        var_cfdlb_i_slot: &mut f64,
        var_cfdlb_i_rv_slot: &mut f64,
        var_cic1_i_slot: &mut f64,
        var_cic1_i_rv_slot: &mut f64,
        var_cic2_i_slot: &mut f64,
        var_cic2_i_rv_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ct_i_rv_slot: &mut f64,
        var_epsch_slot: &mut f64,
        var_epsch_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_lambda_le_slot: &mut f64,
        var_lambda_le_rv_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_nov_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_dn4_slot: &mut f64,
        var_np_i_dn6_slot: &mut f64,
        var_np_i_dn7_slot: &mut f64,
        var_np_i_dn8_slot: &mut f64,
        var_np_i_dn9_slot: &mut f64,
        var_np_i_rv_slot: &mut f64,
        var_nsddc_i_slot: &mut f64,
        var_nsddc_i_rv_slot: &mut f64,
        var_nsub_i_slot: &mut f64,
        var_nsub_i_rv_slot: &mut f64,
        var_one_m_xge_slot: &mut f64,
        var_one_m_xge_rv_slot: &mut f64,
        var_pnce_i_slot: &mut f64,
        var_pnce_i_rv_slot: &mut f64,
        var_pnce_p_slot: &mut f64,
        var_pnce_p_rv_slot: &mut f64,
        var_psce1_i_slot: &mut f64,
        var_psce1_i_rv_slot: &mut f64,
        var_psce2_i_slot: &mut f64,
        var_psce2_i_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_pscedlb_i_slot: &mut f64,
        var_pscedlb_i_rv_slot: &mut f64,
        var_stcf_i_slot: &mut f64,
        var_stcf_i_dn4_slot: &mut f64,
        var_stcf_i_dn6_slot: &mut f64,
        var_stcf_i_dn7_slot: &mut f64,
        var_stcf_i_dn8_slot: &mut f64,
        var_stcf_i_dn9_slot: &mut f64,
        var_stcf_i_rv_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stvfb_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp0__blk79_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_toxp_i_slot: &mut f64,
        var_toxp_i_rv_slot: &mut f64,
        var_typesub_i_slot: &mut f64,
        var_typesub_i_rv_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb1_t_rv_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfb2_t_rv_slot: &mut f64,
    ) {
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf1_t_rv: f64 = *var_cf1_t_rv_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf2_t_rv: f64 = *var_cf2_t_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cfdl_i: f64 = *var_cfdl_i_slot;
        let mut var_cfdl_i_rv: f64 = *var_cfdl_i_rv_slot;
        let mut var_cfdlb_i: f64 = *var_cfdlb_i_slot;
        let mut var_cfdlb_i_rv: f64 = *var_cfdlb_i_rv_slot;
        let mut var_cic1_i: f64 = *var_cic1_i_slot;
        let mut var_cic1_i_rv: f64 = *var_cic1_i_rv_slot;
        let mut var_cic2_i: f64 = *var_cic2_i_slot;
        let mut var_cic2_i_rv: f64 = *var_cic2_i_rv_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ct_i_rv: f64 = *var_ct_i_rv_slot;
        let mut var_epsch: f64 = *var_epsch_slot;
        let mut var_epsch_rv: f64 = *var_epsch_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_lambda_le: f64 = *var_lambda_le_slot;
        let mut var_lambda_le_rv: f64 = *var_lambda_le_rv_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_nov_i_rv: f64 = *var_nov_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_dn4: f64 = *var_np_i_dn4_slot;
        let mut var_np_i_dn6: f64 = *var_np_i_dn6_slot;
        let mut var_np_i_dn7: f64 = *var_np_i_dn7_slot;
        let mut var_np_i_dn8: f64 = *var_np_i_dn8_slot;
        let mut var_np_i_dn9: f64 = *var_np_i_dn9_slot;
        let mut var_np_i_rv: f64 = *var_np_i_rv_slot;
        let mut var_nsddc_i: f64 = *var_nsddc_i_slot;
        let mut var_nsddc_i_rv: f64 = *var_nsddc_i_rv_slot;
        let mut var_nsub_i: f64 = *var_nsub_i_slot;
        let mut var_nsub_i_rv: f64 = *var_nsub_i_rv_slot;
        let mut var_one_m_xge: f64 = *var_one_m_xge_slot;
        let mut var_one_m_xge_rv: f64 = *var_one_m_xge_rv_slot;
        let mut var_pnce_i: f64 = *var_pnce_i_slot;
        let mut var_pnce_i_rv: f64 = *var_pnce_i_rv_slot;
        let mut var_pnce_p: f64 = *var_pnce_p_slot;
        let mut var_pnce_p_rv: f64 = *var_pnce_p_rv_slot;
        let mut var_psce1_i: f64 = *var_psce1_i_slot;
        let mut var_psce1_i_rv: f64 = *var_psce1_i_rv_slot;
        let mut var_psce2_i: f64 = *var_psce2_i_slot;
        let mut var_psce2_i_rv: f64 = *var_psce2_i_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_pscedlb_i: f64 = *var_pscedlb_i_slot;
        let mut var_pscedlb_i_rv: f64 = *var_pscedlb_i_rv_slot;
        let mut var_stcf_i: f64 = *var_stcf_i_slot;
        let mut var_stcf_i_dn4: f64 = *var_stcf_i_dn4_slot;
        let mut var_stcf_i_dn6: f64 = *var_stcf_i_dn6_slot;
        let mut var_stcf_i_dn7: f64 = *var_stcf_i_dn7_slot;
        let mut var_stcf_i_dn8: f64 = *var_stcf_i_dn8_slot;
        let mut var_stcf_i_dn9: f64 = *var_stcf_i_dn9_slot;
        let mut var_stcf_i_rv: f64 = *var_stcf_i_rv_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stvfb_i_rv: f64 = *var_stvfb_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp0__blk79_rv: f64 = *var_temp0__blk79_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_toxp_i: f64 = *var_toxp_i_slot;
        let mut var_toxp_i_rv: f64 = *var_toxp_i_rv_slot;
        let mut var_typesub_i: f64 = *var_typesub_i_slot;
        let mut var_typesub_i_rv: f64 = *var_typesub_i_rv_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb1_t_rv: f64 = *var_vfb1_t_rv_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfb2_t_rv: f64 = *var_vfb2_t_rv_slot;

        let (assign2440_e2110,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2440_e2110;
        var_typesub_i_rv = 0.0;

        let assign2450_e2113: f64 = if p.p206 < 0.0 { 1.0 } else { 0.0 };
        var_guard95 = assign2450_e2113;
        var_guard95_rv = 0.0;

        let (assign2460_e2121,) = {
    if ((var_guard83 == 0.0) && (var_guard95 != 0.0)) {
        let assign2460_e2119: f64 = (-1.0);
        (assign2460_e2119,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2460_e2121;
        var_typesub_i_rv = 0.0;

        let (assign2470_e2133,) = {
    if (var_guard83 == 0.0) {
        let assign2470_e2125: f64 = (p.p206).abs();
        let assign2470_e2127: f64 = (assign2470_e2125).max(1e16);
        let assign2470_e2129: f64 = (assign2470_e2127).min(1e21);
        let assign2470_e2131: f64 = (assign2470_e2129 * 1000000.0);
        (assign2470_e2131,)
    } else {
        (var_nsub_i,)
    }
};
        var_nsub_i = assign2470_e2133;
        var_nsub_i_rv = 0.0;

        let (assign2480_e2138,) = {
    if (var_guard83 == 0.0) {
        (p.p207,)
    } else {
        (var_ct_i,)
    }
};
        var_ct_i = assign2480_e2138;
        var_ct_i_rv = 0.0;

        let (assign2490_e2143,) = {
    if (var_guard83 == 0.0) {
        (p.p208,)
    } else {
        (var_toxp_i,)
    }
};
        var_toxp_i = assign2490_e2143;
        var_toxp_i_rv = 0.0;

        let (assign2500_e2150,) = {
    if (var_guard83 == 0.0) {
        let assign2500_e2148: f64 = (p.p209 * 1000000.0);
        (assign2500_e2148,)
    } else {
        (var_nov_i,)
    }
};
        var_nov_i = assign2500_e2150;
        var_nov_i_rv = 0.0;

        let (assign2510_e2157,) = {
    if (var_guard83 == 0.0) {
        let assign2510_e2155: f64 = (p.p210 * 1000000.0);
        (assign2510_e2155,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign2510_e2157;
        var_novd_i_rv = 0.0;

        let (assign2520_e2174, assign2520_e2174_d_n4, assign2520_e2174_d_n6, assign2520_e2174_d_n7, assign2520_e2174_d_n8, assign2520_e2174_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2520_e2163: f64 = (var_ile).powf(p.p213);
        let assign2520_e2164: f64 = (p.p212 * assign2520_e2163);
        let assign2520_e2169: f64 = (var_ile).powf(p.p215);
        let assign2520_e2170: f64 = (p.p214 * assign2520_e2169);
        let assign2520_e2171: f64 = (1.0 + assign2520_e2170);
        let assign2520_e2172: f64 = (assign2520_e2164 / assign2520_e2171);
        (assign2520_e2172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2520_e2174;
        var_temp_dn4 = assign2520_e2174_d_n4;
        var_temp_dn6 = assign2520_e2174_d_n6;
        var_temp_dn7 = assign2520_e2174_d_n7;
        var_temp_dn8 = assign2520_e2174_d_n8;
        var_temp_dn9 = assign2520_e2174_d_n9;
        var_temp_rv = 0.0;

        let (assign2530_e2189, assign2530_e2189_d_n4, assign2530_e2189_d_n6, assign2530_e2189_d_n7, assign2530_e2189_d_n8, assign2530_e2189_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2530_e2179: f64 = (p.p211 + var_temp);
        let assign2530_e2182: f64 = (p.p216 * var_iwe);
        let assign2530_e2183: f64 = (assign2530_e2179 + assign2530_e2182);
        let assign2530_e2186: f64 = (p.p217 * var_iae);
        let assign2530_e2187: f64 = (assign2530_e2183 + assign2530_e2186);
        (assign2530_e2187, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign2530_e2189;
        var_vfb1_t_dn4 = assign2530_e2189_d_n4;
        var_vfb1_t_dn6 = assign2530_e2189_d_n6;
        var_vfb1_t_dn7 = assign2530_e2189_d_n7;
        var_vfb1_t_dn8 = assign2530_e2189_d_n8;
        var_vfb1_t_dn9 = assign2530_e2189_d_n9;
        var_vfb1_t_rv = 0.0;

        let (assign2540_e2202, assign2540_e2202_d_n4, assign2540_e2202_d_n6, assign2540_e2202_d_n7, assign2540_e2202_d_n8, assign2540_e2202_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2540_e2195: f64 = (p.p219 * var_tox2_i);
        let assign2540_e2197: f64 = (assign2540_e2195 / var_tox1_i);
        let assign2540_e2199: f64 = (assign2540_e2197 * var_temp);
        let assign2540_e2200: f64 = (p.p218 + assign2540_e2199);
        (assign2540_e2200, (assign2540_e2197 * var_temp_dn4), (assign2540_e2197 * var_temp_dn6), (assign2540_e2197 * var_temp_dn7), (assign2540_e2197 * var_temp_dn8), (assign2540_e2197 * var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign2540_e2202;
        var_vfb2_t_dn4 = assign2540_e2202_d_n4;
        var_vfb2_t_dn6 = assign2540_e2202_d_n6;
        var_vfb2_t_dn7 = assign2540_e2202_d_n7;
        var_vfb2_t_dn8 = assign2540_e2202_d_n8;
        var_vfb2_t_dn9 = assign2540_e2202_d_n9;
        var_vfb2_t_rv = 0.0;

        let (assign2550_e2225,) = {
    if (var_guard83 == 0.0) {
        let assign2550_e2209: f64 = (p.p221 * var_ile);
        let assign2550_e2210: f64 = (1.0 + assign2550_e2209);
        let assign2550_e2211: f64 = (p.p220 * assign2550_e2210);
        let assign2550_e2215: f64 = (p.p222 * var_iwe);
        let assign2550_e2216: f64 = (1.0 + assign2550_e2215);
        let assign2550_e2217: f64 = (assign2550_e2211 * assign2550_e2216);
        let assign2550_e2221: f64 = (p.p223 * var_iae);
        let assign2550_e2222: f64 = (1.0 + assign2550_e2221);
        let assign2550_e2223: f64 = (assign2550_e2217 * assign2550_e2222);
        (assign2550_e2223,)
    } else {
        (var_stvfb_i,)
    }
};
        var_stvfb_i = assign2550_e2225;
        var_stvfb_i_rv = 0.0;

        let (assign2560_e2238, assign2560_e2238_d_n4, assign2560_e2238_d_n6, assign2560_e2238_d_n7, assign2560_e2238_d_n8, assign2560_e2238_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2560_e2232: f64 = (p.p225 * var_ile);
        let assign2560_e2233: f64 = (1.0 + assign2560_e2232);
        let assign2560_e2234: f64 = (p.p224 * assign2560_e2233);
        let assign2560_e2236: f64 = (assign2560_e2234 * 1000000.0);
        (assign2560_e2236, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign2560_e2238;
        var_temp0__blk79_dn4 = assign2560_e2238_d_n4;
        var_temp0__blk79_dn6 = assign2560_e2238_d_n6;
        var_temp0__blk79_dn7 = assign2560_e2238_d_n7;
        var_temp0__blk79_dn8 = assign2560_e2238_d_n8;
        var_temp0__blk79_dn9 = assign2560_e2238_d_n9;
        var_temp0__blk79_rv = 0.0;

        let (assign2570_e2247, assign2570_e2247_d_n4, assign2570_e2247_d_n6, assign2570_e2247_d_n7, assign2570_e2247_d_n8, assign2570_e2247_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2570_e2243: f64 = (var_temp0__blk79).max(1e25);
        let assign2570_e2245: f64 = (assign2570_e2243).min(1e28);
        (assign2570_e2245, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_np_i, var_np_i_dn4, var_np_i_dn6, var_np_i_dn7, var_np_i_dn8, var_np_i_dn9,)
    }
};
        var_np_i = assign2570_e2247;
        var_np_i_dn4 = assign2570_e2247_d_n4;
        var_np_i_dn6 = assign2570_e2247_d_n6;
        var_np_i_dn7 = assign2570_e2247_d_n7;
        var_np_i_dn8 = assign2570_e2247_d_n8;
        var_np_i_dn9 = assign2570_e2247_d_n9;
        var_np_i_rv = 0.0;

        let (assign2580_e2252,) = {
    if (var_guard83 == 0.0) {
        (p.p226,)
    } else {
        (var_cic1_i,)
    }
};
        var_cic1_i = assign2580_e2252;
        var_cic1_i_rv = 0.0;

        let (assign2590_e2257,) = {
    if (var_guard83 == 0.0) {
        (p.p227,)
    } else {
        (var_cic2_i,)
    }
};
        var_cic2_i = assign2590_e2257;
        var_cic2_i_rv = 0.0;

        let (assign2600_e2264,) = {
    if (var_guard83 == 0.0) {
        let assign2600_e2262: f64 = (1.0 - var_xge_i);
        (assign2600_e2262,)
    } else {
        (var_one_m_xge,)
    }
};
        var_one_m_xge = assign2600_e2264;
        var_one_m_xge_rv = 0.0;

        let (assign2610_e2275,) = {
    if (var_guard83 == 0.0) {
        let assign2610_e2269: f64 = (1.04479e-10 * var_one_m_xge);
        let assign2610_e2272: f64 = (1.43438e-10 * var_xge_i);
        let assign2610_e2273: f64 = (assign2610_e2269 + assign2610_e2272);
        (assign2610_e2273,)
    } else {
        (var_epsch,)
    }
};
        var_epsch = assign2610_e2275;
        var_epsch_rv = 0.0;

        let (assign2620_e2291,) = {
    if (var_guard83 == 0.0) {
        let assign2620_e2280: f64 = (var_epsch / 3.45313e-11);
        let assign2620_e2282: f64 = (assign2620_e2280 * var_tsi_i);
        let assign2620_e2285: f64 = (var_tox1_i + 4e-10);
        let assign2620_e2286: f64 = (assign2620_e2282 * assign2620_e2285);
        let assign2620_e2287: f64 = (assign2620_e2286).sqrt();
        let assign2620_e2289: f64 = (assign2620_e2287 / var_le);
        (assign2620_e2289,)
    } else {
        (var_lambda_le,)
    }
};
        var_lambda_le = assign2620_e2291;
        var_lambda_le_rv = 0.0;

        let (assign2630_e2308,) = {
    if (var_guard83 == 0.0) {
        let assign2630_e2296: f64 = (p.p228 * 2.0);
        let assign2630_e2299: f64 = (var_lambda_le).powf(p.p229);
        let assign2630_e2300: f64 = (assign2630_e2296 * assign2630_e2299);
        let assign2630_e2304: f64 = (p.p230 * var_iwe);
        let assign2630_e2305: f64 = (1.0 + assign2630_e2304);
        let assign2630_e2306: f64 = (assign2630_e2300 * assign2630_e2305);
        (assign2630_e2306,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign2630_e2308;
        var_psce_p_rv = 0.0;

        let (assign2640_e2317,) = {
    if (var_guard83 == 0.0) {
        let assign2640_e2313: f64 = (var_psce_p).max(0.0);
        let assign2640_e2315: f64 = (assign2640_e2313).min(5.0);
        (assign2640_e2315,)
    } else {
        (var_psce1_i,)
    }
};
        var_psce1_i = assign2640_e2317;
        var_psce1_i_rv = 0.0;

        let (assign2650_e2328,) = {
    if (var_guard83 == 0.0) {
        let assign2650_e2322: f64 = (p.p231 * var_psce1_i);
        let assign2650_e2324: f64 = (assign2650_e2322 * var_tox2_i);
        let assign2650_e2326: f64 = (assign2650_e2324 / var_tox1_i);
        (assign2650_e2326,)
    } else {
        (var_psce2_i,)
    }
};
        var_psce2_i = assign2650_e2328;
        var_psce2_i_rv = 0.0;

        let (assign2660_e2335,) = {
    if (var_guard83 == 0.0) {
        let assign2660_e2333: f64 = (p.p232 * 1000000.0);
        (assign2660_e2333,)
    } else {
        (var_nsddc_i,)
    }
};
        var_nsddc_i = assign2660_e2335;
        var_nsddc_i_rv = 0.0;

        let (assign2670_e2340,) = {
    if (var_guard83 == 0.0) {
        (p.p233,)
    } else {
        (var_pscedlb_i,)
    }
};
        var_pscedlb_i = assign2670_e2340;
        var_pscedlb_i_rv = 0.0;

        let (assign2680_e2347,) = {
    if (var_guard83 == 0.0) {
        let assign2680_e2345: f64 = (p.p234 * var_iwe);
        (assign2680_e2345,)
    } else {
        (var_pnce_p,)
    }
};
        var_pnce_p = assign2680_e2347;
        var_pnce_p_rv = 0.0;

        let (assign2690_e2357,) = {
    if (var_guard83 == 0.0) {
        let assign2690_e2352: f64 = (-1.0);
        let assign2690_e2353: f64 = (var_pnce_p).max(assign2690_e2352);
        let assign2690_e2355: f64 = (assign2690_e2353).min(1.0);
        (assign2690_e2355,)
    } else {
        (var_pnce_i,)
    }
};
        var_pnce_i = assign2690_e2357;
        var_pnce_i_rv = 0.0;

        let (assign2700_e2370, assign2700_e2370_d_n4, assign2700_e2370_d_n6, assign2700_e2370_d_n7, assign2700_e2370_d_n8, assign2700_e2370_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2700_e2362: f64 = (var_lambda_le).powf(p.p236);
        let assign2700_e2366: f64 = (p.p237 * var_iwe);
        let assign2700_e2367: f64 = (1.0 + assign2700_e2366);
        let assign2700_e2368: f64 = (assign2700_e2362 * assign2700_e2367);
        (assign2700_e2368, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2700_e2370;
        var_temp_dn4 = assign2700_e2370_d_n4;
        var_temp_dn6 = assign2700_e2370_d_n6;
        var_temp_dn7 = assign2700_e2370_d_n7;
        var_temp_dn8 = assign2700_e2370_d_n8;
        var_temp_dn9 = assign2700_e2370_d_n9;
        var_temp_rv = 0.0;

        let (assign2710_e2377, assign2710_e2377_d_n4, assign2710_e2377_d_n6, assign2710_e2377_d_n7, assign2710_e2377_d_n8, assign2710_e2377_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2710_e2375: f64 = (p.p235 * var_temp);
        (assign2710_e2375, (p.p235 * var_temp_dn4), (p.p235 * var_temp_dn6), (p.p235 * var_temp_dn7), (p.p235 * var_temp_dn8), (p.p235 * var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign2710_e2377;
        var_cf_p_dn4 = assign2710_e2377_d_n4;
        var_cf_p_dn6 = assign2710_e2377_d_n6;
        var_cf_p_dn7 = assign2710_e2377_d_n7;
        var_cf_p_dn8 = assign2710_e2377_d_n8;
        var_cf_p_dn9 = assign2710_e2377_d_n9;
        var_cf_p_rv = 0.0;

        let (assign2720_e2384, assign2720_e2384_d_n4, assign2720_e2384_d_n6, assign2720_e2384_d_n7, assign2720_e2384_d_n8, assign2720_e2384_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2720_e2382: f64 = (var_cf_p).max(0.0);
        (assign2720_e2382, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign2720_e2384;
        var_cf1_t_dn4 = assign2720_e2384_d_n4;
        var_cf1_t_dn6 = assign2720_e2384_d_n6;
        var_cf1_t_dn7 = assign2720_e2384_d_n7;
        var_cf1_t_dn8 = assign2720_e2384_d_n8;
        var_cf1_t_dn9 = assign2720_e2384_d_n9;
        var_cf1_t_rv = 0.0;

        let (assign2730_e2395, assign2730_e2395_d_n4, assign2730_e2395_d_n6, assign2730_e2395_d_n7, assign2730_e2395_d_n8, assign2730_e2395_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2730_e2389: f64 = (p.p238 * var_cf1_t);
        let assign2730_e2391: f64 = (assign2730_e2389 * var_tox2_i);
        let assign2730_e2393: f64 = (assign2730_e2391 / var_tox1_i);
        (assign2730_e2393, (((p.p238 * var_cf1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign2730_e2395;
        var_cf2_t_dn4 = assign2730_e2395_d_n4;
        var_cf2_t_dn6 = assign2730_e2395_d_n6;
        var_cf2_t_dn7 = assign2730_e2395_d_n7;
        var_cf2_t_dn8 = assign2730_e2395_d_n8;
        var_cf2_t_dn9 = assign2730_e2395_d_n9;
        var_cf2_t_rv = 0.0;

        let (assign2740_e2402, assign2740_e2402_d_n4, assign2740_e2402_d_n6, assign2740_e2402_d_n7, assign2740_e2402_d_n8, assign2740_e2402_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2740_e2400: f64 = (p.p239 * var_temp);
        (assign2740_e2400, (p.p239 * var_temp_dn4), (p.p239 * var_temp_dn6), (p.p239 * var_temp_dn7), (p.p239 * var_temp_dn8), (p.p239 * var_temp_dn9),)
    } else {
        (var_stcf_i, var_stcf_i_dn4, var_stcf_i_dn6, var_stcf_i_dn7, var_stcf_i_dn8, var_stcf_i_dn9,)
    }
};
        var_stcf_i = assign2740_e2402;
        var_stcf_i_dn4 = assign2740_e2402_d_n4;
        var_stcf_i_dn6 = assign2740_e2402_d_n6;
        var_stcf_i_dn7 = assign2740_e2402_d_n7;
        var_stcf_i_dn8 = assign2740_e2402_d_n8;
        var_stcf_i_dn9 = assign2740_e2402_d_n9;
        var_stcf_i_rv = 0.0;

        let (assign2750_e2407,) = {
    if (var_guard83 == 0.0) {
        (p.p240,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign2750_e2407;
        var_cfd_i_rv = 0.0;

        let (assign2760_e2422,) = {
    if (var_guard83 == 0.0) {
        let assign2760_e2412: f64 = (p.p241 * var_ile);
        let assign2760_e2416: f64 = (p.p242 * var_iwe);
        let assign2760_e2417: f64 = (1.0 + assign2760_e2416);
        let assign2760_e2419: f64 = (assign2760_e2417).max(0.001);
        let assign2760_e2420: f64 = (assign2760_e2412 / assign2760_e2419);
        (assign2760_e2420,)
    } else {
        (var_cfdl_i,)
    }
};
        var_cfdl_i = assign2760_e2422;
        var_cfdl_i_rv = 0.0;

        let (assign2770_e2427,) = {
    if (var_guard83 == 0.0) {
        (p.p243,)
    } else {
        (var_cfdlb_i,)
    }
};
        var_cfdlb_i = assign2770_e2427;
        var_cfdlb_i_rv = 0.0;

        let (assign2780_e2443, assign2780_e2443_d_n4, assign2780_e2443_d_n6, assign2780_e2443_d_n7, assign2780_e2443_d_n8, assign2780_e2443_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2780_e2431: f64 = (-var_le);
        let assign2780_e2436: f64 = (p.p248 * var_iwe);
        let assign2780_e2437: f64 = (1.0 + assign2780_e2436);
        let assign2780_e2439: f64 = (assign2780_e2437).max(0.001);
        let assign2780_e2440: f64 = (p.p247 * assign2780_e2439);
        let assign2780_e2441: f64 = (assign2780_e2431 / assign2780_e2440);
        (assign2780_e2441, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign2780_e2443;
        var_temp1_dn4 = assign2780_e2443_d_n4;
        var_temp1_dn6 = assign2780_e2443_d_n6;
        var_temp1_dn7 = assign2780_e2443_d_n7;
        var_temp1_dn8 = assign2780_e2443_d_n8;
        var_temp1_dn9 = assign2780_e2443_d_n9;
        var_temp1_rv = 0.0;

        let assign2790_e2446: f64 = (-80.0);
        let assign2790_e2447: f64 = if var_temp1 > assign2790_e2446 { 1.0 } else { 0.0 };
        var_guard96 = assign2790_e2447;
        var_guard96_rv = 0.0;

        let (assign2800_e2455, assign2800_e2455_d_n4, assign2800_e2455_d_n6, assign2800_e2455_d_n7, assign2800_e2455_d_n8, assign2800_e2455_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard96 != 0.0)) {
        let assign2800_e2453: f64 = (var_temp1).exp();
        (assign2800_e2453, (assign2800_e2453 * var_temp1_dn4), (assign2800_e2453 * var_temp1_dn6), (assign2800_e2453 * var_temp1_dn7), (assign2800_e2453 * var_temp1_dn8), (assign2800_e2453 * var_temp1_dn9),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign2800_e2455;
        var_temp2_dn4 = assign2800_e2455_d_n4;
        var_temp2_dn6 = assign2800_e2455_d_n6;
        var_temp2_dn7 = assign2800_e2455_d_n7;
        var_temp2_dn8 = assign2800_e2455_d_n8;
        var_temp2_dn9 = assign2800_e2455_d_n9;
        var_temp2_rv = 0.0;

        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf1_t_rv_slot = var_cf1_t_rv;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf2_t_rv_slot = var_cf2_t_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cfdl_i_slot = var_cfdl_i;
        *var_cfdl_i_rv_slot = var_cfdl_i_rv;
        *var_cfdlb_i_slot = var_cfdlb_i;
        *var_cfdlb_i_rv_slot = var_cfdlb_i_rv;
        *var_cic1_i_slot = var_cic1_i;
        *var_cic1_i_rv_slot = var_cic1_i_rv;
        *var_cic2_i_slot = var_cic2_i;
        *var_cic2_i_rv_slot = var_cic2_i_rv;
        *var_ct_i_slot = var_ct_i;
        *var_ct_i_rv_slot = var_ct_i_rv;
        *var_epsch_slot = var_epsch;
        *var_epsch_rv_slot = var_epsch_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_lambda_le_slot = var_lambda_le;
        *var_lambda_le_rv_slot = var_lambda_le_rv;
        *var_nov_i_slot = var_nov_i;
        *var_nov_i_rv_slot = var_nov_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_np_i_slot = var_np_i;
        *var_np_i_dn4_slot = var_np_i_dn4;
        *var_np_i_dn6_slot = var_np_i_dn6;
        *var_np_i_dn7_slot = var_np_i_dn7;
        *var_np_i_dn8_slot = var_np_i_dn8;
        *var_np_i_dn9_slot = var_np_i_dn9;
        *var_np_i_rv_slot = var_np_i_rv;
        *var_nsddc_i_slot = var_nsddc_i;
        *var_nsddc_i_rv_slot = var_nsddc_i_rv;
        *var_nsub_i_slot = var_nsub_i;
        *var_nsub_i_rv_slot = var_nsub_i_rv;
        *var_one_m_xge_slot = var_one_m_xge;
        *var_one_m_xge_rv_slot = var_one_m_xge_rv;
        *var_pnce_i_slot = var_pnce_i;
        *var_pnce_i_rv_slot = var_pnce_i_rv;
        *var_pnce_p_slot = var_pnce_p;
        *var_pnce_p_rv_slot = var_pnce_p_rv;
        *var_psce1_i_slot = var_psce1_i;
        *var_psce1_i_rv_slot = var_psce1_i_rv;
        *var_psce2_i_slot = var_psce2_i;
        *var_psce2_i_rv_slot = var_psce2_i_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_pscedlb_i_slot = var_pscedlb_i;
        *var_pscedlb_i_rv_slot = var_pscedlb_i_rv;
        *var_stcf_i_slot = var_stcf_i;
        *var_stcf_i_dn4_slot = var_stcf_i_dn4;
        *var_stcf_i_dn6_slot = var_stcf_i_dn6;
        *var_stcf_i_dn7_slot = var_stcf_i_dn7;
        *var_stcf_i_dn8_slot = var_stcf_i_dn8;
        *var_stcf_i_dn9_slot = var_stcf_i_dn9;
        *var_stcf_i_rv_slot = var_stcf_i_rv;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stvfb_i_rv_slot = var_stvfb_i_rv;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp0__blk79_rv_slot = var_temp0__blk79_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_toxp_i_slot = var_toxp_i;
        *var_toxp_i_rv_slot = var_toxp_i_rv;
        *var_typesub_i_slot = var_typesub_i;
        *var_typesub_i_rv_slot = var_typesub_i_rv;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb1_t_rv_slot = var_vfb1_t_rv;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfb2_t_rv_slot = var_vfb2_t_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_guard83: f64,
        var_guard96: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_temp1: f64,
        var_temp1_dn4: f64,
        var_temp1_dn6: f64,
        var_temp1_dn7: f64,
        var_temp1_dn8: f64,
        var_temp1_dn9: f64,
        var_we: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn1_t_rv_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn2_t_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_cs_t_rv_slot: &mut f64,
        var_csbi_i_slot: &mut f64,
        var_csbi_i_rv_slot: &mut f64,
        var_csfi_i_slot: &mut f64,
        var_csfi_i_rv_slot: &mut f64,
        var_csthr_i_slot: &mut f64,
        var_csthr_i_rv_slot: &mut f64,
        var_csthrb_i_slot: &mut f64,
        var_csthrb_i_rv_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_feta_i_rv_slot: &mut f64,
        var_ge_slot: &mut f64,
        var_ge_dn4_slot: &mut f64,
        var_ge_dn6_slot: &mut f64,
        var_ge_dn7_slot: &mut f64,
        var_ge_dn8_slot: &mut f64,
        var_ge_dn9_slot: &mut f64,
        var_ge_rv_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_dn4_slot: &mut f64,
        var_gpe_dn6_slot: &mut f64,
        var_gpe_dn7_slot: &mut f64,
        var_gpe_dn8_slot: &mut f64,
        var_gpe_dn9_slot: &mut f64,
        var_gpe_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_gwe_rv_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_mue_t_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_rv_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsb_i_rv_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsg_i_rv_slot: &mut f64,
        var_rsig_i_slot: &mut f64,
        var_rsig_i_rv_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stbet_i_rv_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stcs_i_rv_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_stmue_i_rv_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_strs_i_rv_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthecs_i_rv_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthemu_i_rv_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_stxcor_i_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp3_rv_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp4_rv_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_thecs_t_rv_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_themu_t_rv_slot: &mut f64,
        var_thersg_i_slot: &mut f64,
        var_thersg_i_rv_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcor_t_rv_slot: &mut f64,
        var_xcorb_i_slot: &mut f64,
        var_xcorb_i_rv_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn1_t_rv: f64 = *var_betn1_t_rv_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn2_t_rv: f64 = *var_betn2_t_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_cs_t_rv: f64 = *var_cs_t_rv_slot;
        let mut var_csbi_i: f64 = *var_csbi_i_slot;
        let mut var_csbi_i_rv: f64 = *var_csbi_i_rv_slot;
        let mut var_csfi_i: f64 = *var_csfi_i_slot;
        let mut var_csfi_i_rv: f64 = *var_csfi_i_rv_slot;
        let mut var_csthr_i: f64 = *var_csthr_i_slot;
        let mut var_csthr_i_rv: f64 = *var_csthr_i_rv_slot;
        let mut var_csthrb_i: f64 = *var_csthrb_i_slot;
        let mut var_csthrb_i_rv: f64 = *var_csthrb_i_rv_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_feta_i_rv: f64 = *var_feta_i_rv_slot;
        let mut var_ge: f64 = *var_ge_slot;
        let mut var_ge_dn4: f64 = *var_ge_dn4_slot;
        let mut var_ge_dn6: f64 = *var_ge_dn6_slot;
        let mut var_ge_dn7: f64 = *var_ge_dn7_slot;
        let mut var_ge_dn8: f64 = *var_ge_dn8_slot;
        let mut var_ge_dn9: f64 = *var_ge_dn9_slot;
        let mut var_ge_rv: f64 = *var_ge_rv_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_dn4: f64 = *var_gpe_dn4_slot;
        let mut var_gpe_dn6: f64 = *var_gpe_dn6_slot;
        let mut var_gpe_dn7: f64 = *var_gpe_dn7_slot;
        let mut var_gpe_dn8: f64 = *var_gpe_dn8_slot;
        let mut var_gpe_dn9: f64 = *var_gpe_dn9_slot;
        let mut var_gpe_rv: f64 = *var_gpe_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_gwe_rv: f64 = *var_gwe_rv_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_mue_t_rv: f64 = *var_mue_t_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_rv: f64 = *var_rs_t_rv_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsb_i_rv: f64 = *var_rsb_i_rv_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsg_i_rv: f64 = *var_rsg_i_rv_slot;
        let mut var_rsig_i: f64 = *var_rsig_i_slot;
        let mut var_rsig_i_rv: f64 = *var_rsig_i_rv_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stbet_i_rv: f64 = *var_stbet_i_rv_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stcs_i_rv: f64 = *var_stcs_i_rv_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_stmue_i_rv: f64 = *var_stmue_i_rv_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_strs_i_rv: f64 = *var_strs_i_rv_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthecs_i_rv: f64 = *var_stthecs_i_rv_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthemu_i_rv: f64 = *var_stthemu_i_rv_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_stxcor_i_rv: f64 = *var_stxcor_i_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp3_rv: f64 = *var_temp3_rv_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp4_rv: f64 = *var_temp4_rv_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_thecs_t_rv: f64 = *var_thecs_t_rv_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_themu_t_rv: f64 = *var_themu_t_rv_slot;
        let mut var_thersg_i: f64 = *var_thersg_i_slot;
        let mut var_thersg_i_rv: f64 = *var_thersg_i_rv_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcor_t_rv: f64 = *var_xcor_t_rv_slot;
        let mut var_xcorb_i: f64 = *var_xcorb_i_slot;
        let mut var_xcorb_i_rv: f64 = *var_xcorb_i_rv_slot;

        let (assign2810_e2488, assign2810_e2488_d_n4, assign2810_e2488_d_n6, assign2810_e2488_d_n7, assign2810_e2488_d_n8, assign2810_e2488_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard96 == 0.0)) {
        let assign2810_e2464: f64 = (-var_temp1);
        let assign2810_e2466: f64 = (assign2810_e2464 - 80.0);
        let assign2810_e2470: f64 = (-var_temp1);
        let assign2810_e2472: f64 = (assign2810_e2470 - 80.0);
        let assign2810_e2473: f64 = (0.5 * assign2810_e2472);
        let assign2810_e2476: f64 = (-var_temp1);
        let assign2810_e2478: f64 = (assign2810_e2476 - 80.0);
        let assign2810_e2480: f64 = (assign2810_e2478 * 0.3333333333333);
        let assign2810_e2481: f64 = (1.0 + assign2810_e2480);
        let assign2810_e2482: f64 = (assign2810_e2473 * assign2810_e2481);
        let assign2810_e2483: f64 = (1.0 + assign2810_e2482);
        let assign2810_e2484: f64 = (assign2810_e2466 * assign2810_e2483);
        let assign2810_e2485: f64 = (1.0 + assign2810_e2484);
        let assign2810_e2486: f64 = (1.80485e-35 / assign2810_e2485);
        (assign2810_e2486, (-((1.80485e-35 * (((-var_temp1_dn4) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn4)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn4) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn6) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn6)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn6) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn7) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn7)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn7) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn8) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn8)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn8) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn9) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn9)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn9) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign2810_e2488;
        var_temp2_dn4 = assign2810_e2488_d_n4;
        var_temp2_dn6 = assign2810_e2488_d_n6;
        var_temp2_dn7 = assign2810_e2488_d_n7;
        var_temp2_dn8 = assign2810_e2488_d_n8;
        var_temp2_dn9 = assign2810_e2488_d_n9;
        var_temp2_rv = 0.0;

        let (assign2820_e2496, assign2820_e2496_d_n4, assign2820_e2496_d_n6, assign2820_e2496_d_n7, assign2820_e2496_d_n8, assign2820_e2496_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2820_e2492: f64 = (-var_le);
        let assign2820_e2494: f64 = (assign2820_e2492 / p.p250);
        (assign2820_e2494, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign2820_e2496;
        var_temp3_dn4 = assign2820_e2496_d_n4;
        var_temp3_dn6 = assign2820_e2496_d_n6;
        var_temp3_dn7 = assign2820_e2496_d_n7;
        var_temp3_dn8 = assign2820_e2496_d_n8;
        var_temp3_dn9 = assign2820_e2496_d_n9;
        var_temp3_rv = 0.0;

        let assign2830_e2499: f64 = (-80.0);
        let assign2830_e2500: f64 = if var_temp3 > assign2830_e2499 { 1.0 } else { 0.0 };
        var_guard97 = assign2830_e2500;
        var_guard97_rv = 0.0;

        let (assign2840_e2508, assign2840_e2508_d_n4, assign2840_e2508_d_n6, assign2840_e2508_d_n7, assign2840_e2508_d_n8, assign2840_e2508_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard97 != 0.0)) {
        let assign2840_e2506: f64 = (var_temp3).exp();
        (assign2840_e2506, (assign2840_e2506 * var_temp3_dn4), (assign2840_e2506 * var_temp3_dn6), (assign2840_e2506 * var_temp3_dn7), (assign2840_e2506 * var_temp3_dn8), (assign2840_e2506 * var_temp3_dn9),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign2840_e2508;
        var_temp4_dn4 = assign2840_e2508_d_n4;
        var_temp4_dn6 = assign2840_e2508_d_n6;
        var_temp4_dn7 = assign2840_e2508_d_n7;
        var_temp4_dn8 = assign2840_e2508_d_n8;
        var_temp4_dn9 = assign2840_e2508_d_n9;
        var_temp4_rv = 0.0;

        let (assign2850_e2541, assign2850_e2541_d_n4, assign2850_e2541_d_n6, assign2850_e2541_d_n7, assign2850_e2541_d_n8, assign2850_e2541_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard97 == 0.0)) {
        let assign2850_e2517: f64 = (-var_temp3);
        let assign2850_e2519: f64 = (assign2850_e2517 - 80.0);
        let assign2850_e2523: f64 = (-var_temp3);
        let assign2850_e2525: f64 = (assign2850_e2523 - 80.0);
        let assign2850_e2526: f64 = (0.5 * assign2850_e2525);
        let assign2850_e2529: f64 = (-var_temp3);
        let assign2850_e2531: f64 = (assign2850_e2529 - 80.0);
        let assign2850_e2533: f64 = (assign2850_e2531 * 0.3333333333333);
        let assign2850_e2534: f64 = (1.0 + assign2850_e2533);
        let assign2850_e2535: f64 = (assign2850_e2526 * assign2850_e2534);
        let assign2850_e2536: f64 = (1.0 + assign2850_e2535);
        let assign2850_e2537: f64 = (assign2850_e2519 * assign2850_e2536);
        let assign2850_e2538: f64 = (1.0 + assign2850_e2537);
        let assign2850_e2539: f64 = (1.80485e-35 / assign2850_e2538);
        (assign2850_e2539, (-((1.80485e-35 * (((-var_temp3_dn4) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn4)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn4) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn6) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn6)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn6) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn7) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn7)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn7) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn8) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn8)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn8) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn9) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn9)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn9) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign2850_e2541;
        var_temp4_dn4 = assign2850_e2541_d_n4;
        var_temp4_dn6 = assign2850_e2541_d_n6;
        var_temp4_dn7 = assign2850_e2541_d_n7;
        var_temp4_dn8 = assign2850_e2541_d_n8;
        var_temp4_dn9 = assign2850_e2541_d_n9;
        var_temp4_rv = 0.0;

        let (assign2860_e2570, assign2860_e2570_d_n4, assign2860_e2570_d_n6, assign2860_e2570_d_n7, assign2860_e2570_d_n8, assign2860_e2570_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2860_e2549: f64 = (p.p246 * var_iwe);
        let assign2860_e2550: f64 = (1.0 + assign2860_e2549);
        let assign2860_e2551: f64 = (p.p245 * assign2860_e2550);
        let assign2860_e2554: f64 = (var_temp2 - 1.0);
        let assign2860_e2555: f64 = (assign2860_e2551 * assign2860_e2554);
        let assign2860_e2557: f64 = (assign2860_e2555 / var_temp1);
        let assign2860_e2558: f64 = (1.0 + assign2860_e2557);
        let assign2860_e2562: f64 = (var_temp4 - 1.0);
        let assign2860_e2563: f64 = (p.p249 * assign2860_e2562);
        let assign2860_e2565: f64 = (assign2860_e2563 / var_temp3);
        let assign2860_e2566: f64 = (assign2860_e2558 + assign2860_e2565);
        let assign2860_e2568: f64 = (assign2860_e2566).max(1e-6);
        (assign2860_e2568, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn4) * var_temp1) - (assign2860_e2555 * var_temp1_dn4)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn4) * var_temp3) - (assign2860_e2563 * var_temp3_dn4)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn6) * var_temp1) - (assign2860_e2555 * var_temp1_dn6)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn6) * var_temp3) - (assign2860_e2563 * var_temp3_dn6)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn7) * var_temp1) - (assign2860_e2555 * var_temp1_dn7)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn7) * var_temp3) - (assign2860_e2563 * var_temp3_dn7)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn8) * var_temp1) - (assign2860_e2555 * var_temp1_dn8)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn8) * var_temp3) - (assign2860_e2563 * var_temp3_dn8)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn9) * var_temp1) - (assign2860_e2555 * var_temp1_dn9)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn9) * var_temp3) - (assign2860_e2563 * var_temp3_dn9)) / (var_temp3 * var_temp3))) } else { 0.0 },)
    } else {
        (var_gpe, var_gpe_dn4, var_gpe_dn6, var_gpe_dn7, var_gpe_dn8, var_gpe_dn9,)
    }
};
        var_gpe = assign2860_e2570;
        var_gpe_dn4 = assign2860_e2570_d_n4;
        var_gpe_dn6 = assign2860_e2570_d_n6;
        var_gpe_dn7 = assign2860_e2570_d_n7;
        var_gpe_dn8 = assign2860_e2570_d_n8;
        var_gpe_dn9 = assign2860_e2570_d_n9;
        var_gpe_rv = 0.0;

        let (assign2870_e2592,) = {
    if (var_guard83 == 0.0) {
        let assign2870_e2576: f64 = (p.p251 * var_iwe);
        let assign2870_e2577: f64 = (1.0 + assign2870_e2576);
        let assign2870_e2580: f64 = (p.p252 * var_iwe);
        let assign2870_e2584: f64 = (var_we / p.p253);
        let assign2870_e2585: f64 = (1.0 + assign2870_e2584);
        let assign2870_e2586: f64 = (assign2870_e2585).ln();
        let assign2870_e2587: f64 = (assign2870_e2580 * assign2870_e2586);
        let assign2870_e2588: f64 = (assign2870_e2577 + assign2870_e2587);
        let assign2870_e2590: f64 = (assign2870_e2588).max(1e-6);
        (assign2870_e2590,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign2870_e2592;
        var_gwe_rv = 0.0;

        let (assign2880_e2601, assign2880_e2601_d_n4, assign2880_e2601_d_n6, assign2880_e2601_d_n7, assign2880_e2601_d_n8, assign2880_e2601_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2880_e2597: f64 = (p.p244 / var_gpe);
        let assign2880_e2599: f64 = (assign2880_e2597 * var_gwe);
        (assign2880_e2599, ((-((p.p244 * var_gpe_dn4) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn6) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn7) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn8) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn9) / (var_gpe * var_gpe))) * var_gwe),)
    } else {
        (var_ge, var_ge_dn4, var_ge_dn6, var_ge_dn7, var_ge_dn8, var_ge_dn9,)
    }
};
        var_ge = assign2880_e2601;
        var_ge_dn4 = assign2880_e2601_d_n4;
        var_ge_dn6 = assign2880_e2601_d_n6;
        var_ge_dn7 = assign2880_e2601_d_n7;
        var_ge_dn8 = assign2880_e2601_d_n8;
        var_ge_dn9 = assign2880_e2601_d_n9;
        var_ge_rv = 0.0;

        let (assign2890_e2610, assign2890_e2610_d_n4, assign2890_e2610_d_n6, assign2890_e2610_d_n7, assign2890_e2610_d_n8, assign2890_e2610_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2890_e2606: f64 = (var_ge * var_we);
        let assign2890_e2608: f64 = (assign2890_e2606 / var_le);
        (assign2890_e2608, ((var_ge_dn4 * var_we) / var_le), ((var_ge_dn6 * var_we) / var_le), ((var_ge_dn7 * var_we) / var_le), ((var_ge_dn8 * var_we) / var_le), ((var_ge_dn9 * var_we) / var_le),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign2890_e2610;
        var_betn_p_dn4 = assign2890_e2610_d_n4;
        var_betn_p_dn6 = assign2890_e2610_d_n6;
        var_betn_p_dn7 = assign2890_e2610_d_n7;
        var_betn_p_dn8 = assign2890_e2610_d_n8;
        var_betn_p_dn9 = assign2890_e2610_d_n9;
        var_betn_p_rv = 0.0;

        let (assign2900_e2617, assign2900_e2617_d_n4, assign2900_e2617_d_n6, assign2900_e2617_d_n7, assign2900_e2617_d_n8, assign2900_e2617_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2900_e2615: f64 = (var_betn_p).max(1e-10);
        (assign2900_e2615, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign2900_e2617;
        var_betn1_t_dn4 = assign2900_e2617_d_n4;
        var_betn1_t_dn6 = assign2900_e2617_d_n6;
        var_betn1_t_dn7 = assign2900_e2617_d_n7;
        var_betn1_t_dn8 = assign2900_e2617_d_n8;
        var_betn1_t_dn9 = assign2900_e2617_d_n9;
        var_betn1_t_rv = 0.0;

        let (assign2910_e2624, assign2910_e2624_d_n4, assign2910_e2624_d_n6, assign2910_e2624_d_n7, assign2910_e2624_d_n8, assign2910_e2624_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2910_e2622: f64 = (p.p254 * var_betn1_t);
        (assign2910_e2622, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign2910_e2624;
        var_betn2_t_dn4 = assign2910_e2624_d_n4;
        var_betn2_t_dn6 = assign2910_e2624_d_n6;
        var_betn2_t_dn7 = assign2910_e2624_d_n7;
        var_betn2_t_dn8 = assign2910_e2624_d_n8;
        var_betn2_t_dn9 = assign2910_e2624_d_n9;
        var_betn2_t_rv = 0.0;

        let (assign2920_e2647,) = {
    if (var_guard83 == 0.0) {
        let assign2920_e2631: f64 = (p.p256 * var_ile);
        let assign2920_e2632: f64 = (1.0 + assign2920_e2631);
        let assign2920_e2633: f64 = (p.p255 * assign2920_e2632);
        let assign2920_e2637: f64 = (p.p257 * var_iwe);
        let assign2920_e2638: f64 = (1.0 + assign2920_e2637);
        let assign2920_e2639: f64 = (assign2920_e2633 * assign2920_e2638);
        let assign2920_e2643: f64 = (p.p258 * var_iae);
        let assign2920_e2644: f64 = (1.0 + assign2920_e2643);
        let assign2920_e2645: f64 = (assign2920_e2639 * assign2920_e2644);
        (assign2920_e2645,)
    } else {
        (var_stbet_i,)
    }
};
        var_stbet_i = assign2920_e2647;
        var_stbet_i_rv = 0.0;

        let (assign2930_e2670,) = {
    if (var_guard83 == 0.0) {
        let assign2930_e2654: f64 = (var_ile).powf(p.p261);
        let assign2930_e2655: f64 = (p.p260 * assign2930_e2654);
        let assign2930_e2656: f64 = (p.p259 + assign2930_e2655);
        let assign2930_e2660: f64 = (p.p262 * var_iwe);
        let assign2930_e2661: f64 = (1.0 + assign2930_e2660);
        let assign2930_e2662: f64 = (assign2930_e2656 * assign2930_e2661);
        let assign2930_e2666: f64 = (p.p263 * var_iae);
        let assign2930_e2667: f64 = (1.0 + assign2930_e2666);
        let assign2930_e2668: f64 = (assign2930_e2662 * assign2930_e2667);
        (assign2930_e2668,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign2930_e2670;
        var_cs_p_rv = 0.0;

        let (assign2940_e2677,) = {
    if (var_guard83 == 0.0) {
        let assign2940_e2675: f64 = (var_cs_p).max(0.0);
        (assign2940_e2675,)
    } else {
        (var_cs_t,)
    }
};
        var_cs_t = assign2940_e2677;
        var_cs_t_rv = 0.0;

        let (assign2950_e2682,) = {
    if (var_guard83 == 0.0) {
        (p.p264,)
    } else {
        (var_csfi_i,)
    }
};
        var_csfi_i = assign2950_e2682;
        var_csfi_i_rv = 0.0;

        let (assign2960_e2687,) = {
    if (var_guard83 == 0.0) {
        (p.p265,)
    } else {
        (var_csbi_i,)
    }
};
        var_csbi_i = assign2960_e2687;
        var_csbi_i_rv = 0.0;

        let (assign2970_e2710,) = {
    if (var_guard83 == 0.0) {
        let assign2970_e2694: f64 = (p.p267 * var_ile);
        let assign2970_e2695: f64 = (1.0 + assign2970_e2694);
        let assign2970_e2696: f64 = (p.p266 * assign2970_e2695);
        let assign2970_e2700: f64 = (p.p268 * var_iwe);
        let assign2970_e2701: f64 = (1.0 + assign2970_e2700);
        let assign2970_e2702: f64 = (assign2970_e2696 * assign2970_e2701);
        let assign2970_e2706: f64 = (p.p269 * var_iae);
        let assign2970_e2707: f64 = (1.0 + assign2970_e2706);
        let assign2970_e2708: f64 = (assign2970_e2702 * assign2970_e2707);
        (assign2970_e2708,)
    } else {
        (var_stcs_i,)
    }
};
        var_stcs_i = assign2970_e2710;
        var_stcs_i_rv = 0.0;

        let (assign2980_e2715,) = {
    if (var_guard83 == 0.0) {
        (p.p270,)
    } else {
        (var_thecs_t,)
    }
};
        var_thecs_t = assign2980_e2715;
        var_thecs_t_rv = 0.0;

        let (assign2990_e2720,) = {
    if (var_guard83 == 0.0) {
        (p.p271,)
    } else {
        (var_stthecs_i,)
    }
};
        var_stthecs_i = assign2990_e2720;
        var_stthecs_i_rv = 0.0;

        let (assign3000_e2725,) = {
    if (var_guard83 == 0.0) {
        (p.p272,)
    } else {
        (var_csthr_i,)
    }
};
        var_csthr_i = assign3000_e2725;
        var_csthr_i_rv = 0.0;

        let (assign3010_e2730,) = {
    if (var_guard83 == 0.0) {
        (p.p273,)
    } else {
        (var_csthrb_i,)
    }
};
        var_csthrb_i = assign3010_e2730;
        var_csthrb_i_rv = 0.0;

        let (assign3020_e2735,) = {
    if (var_guard83 == 0.0) {
        (p.p274,)
    } else {
        (var_mue_t,)
    }
};
        var_mue_t = assign3020_e2735;
        var_mue_t_rv = 0.0;

        let (assign3030_e2740,) = {
    if (var_guard83 == 0.0) {
        (p.p275,)
    } else {
        (var_stmue_i,)
    }
};
        var_stmue_i = assign3030_e2740;
        var_stmue_i_rv = 0.0;

        let (assign3040_e2745,) = {
    if (var_guard83 == 0.0) {
        (p.p276,)
    } else {
        (var_themu_t,)
    }
};
        var_themu_t = assign3040_e2745;
        var_themu_t_rv = 0.0;

        let (assign3050_e2750,) = {
    if (var_guard83 == 0.0) {
        (p.p277,)
    } else {
        (var_stthemu_i,)
    }
};
        var_stthemu_i = assign3050_e2750;
        var_stthemu_i_rv = 0.0;

        let (assign3060_e2773,) = {
    if (var_guard83 == 0.0) {
        let assign3060_e2757: f64 = (var_ile).powf(p.p280);
        let assign3060_e2758: f64 = (p.p279 * assign3060_e2757);
        let assign3060_e2759: f64 = (p.p278 + assign3060_e2758);
        let assign3060_e2763: f64 = (p.p281 * var_iwe);
        let assign3060_e2764: f64 = (1.0 + assign3060_e2763);
        let assign3060_e2765: f64 = (assign3060_e2759 * assign3060_e2764);
        let assign3060_e2769: f64 = (p.p282 * var_iae);
        let assign3060_e2770: f64 = (1.0 + assign3060_e2769);
        let assign3060_e2771: f64 = (assign3060_e2765 * assign3060_e2770);
        (assign3060_e2771,)
    } else {
        (var_xcor_t,)
    }
};
        var_xcor_t = assign3060_e2773;
        var_xcor_t_rv = 0.0;

        let (assign3070_e2778,) = {
    if (var_guard83 == 0.0) {
        (p.p283,)
    } else {
        (var_xcorb_i,)
    }
};
        var_xcorb_i = assign3070_e2778;
        var_xcorb_i_rv = 0.0;

        let (assign3080_e2783,) = {
    if (var_guard83 == 0.0) {
        (p.p284,)
    } else {
        (var_stxcor_i,)
    }
};
        var_stxcor_i = assign3080_e2783;
        var_stxcor_i_rv = 0.0;

        let (assign3090_e2788,) = {
    if (var_guard83 == 0.0) {
        (p.p285,)
    } else {
        (var_feta_i,)
    }
};
        var_feta_i = assign3090_e2788;
        var_feta_i_rv = 0.0;

        let (assign3100_e2801,) = {
    if (var_guard83 == 0.0) {
        let assign3100_e2793: f64 = (p.p286 * var_iwe);
        let assign3100_e2797: f64 = (p.p287 * var_iwe);
        let assign3100_e2798: f64 = (1.0 + assign3100_e2797);
        let assign3100_e2799: f64 = (assign3100_e2793 * assign3100_e2798);
        (assign3100_e2799,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign3100_e2801;
        var_rs_p_rv = 0.0;

        let (assign3110_e2808,) = {
    if (var_guard83 == 0.0) {
        let assign3110_e2806: f64 = (var_rs_p).max(0.0);
        (assign3110_e2806,)
    } else {
        (var_rs_t,)
    }
};
        var_rs_t = assign3110_e2808;
        var_rs_t_rv = 0.0;

        let (assign3120_e2813,) = {
    if (var_guard83 == 0.0) {
        (p.p288,)
    } else {
        (var_rsig_i,)
    }
};
        var_rsig_i = assign3120_e2813;
        var_rsig_i_rv = 0.0;

        let (assign3130_e2818,) = {
    if (var_guard83 == 0.0) {
        (p.p289,)
    } else {
        (var_strs_i,)
    }
};
        var_strs_i = assign3130_e2818;
        var_strs_i_rv = 0.0;

        let (assign3140_e2823,) = {
    if (var_guard83 == 0.0) {
        (p.p290,)
    } else {
        (var_rsg_i,)
    }
};
        var_rsg_i = assign3140_e2823;
        var_rsg_i_rv = 0.0;

        let (assign3150_e2828,) = {
    if (var_guard83 == 0.0) {
        (p.p291,)
    } else {
        (var_thersg_i,)
    }
};
        var_thersg_i = assign3150_e2828;
        var_thersg_i_rv = 0.0;

        let (assign3160_e2833,) = {
    if (var_guard83 == 0.0) {
        (p.p292,)
    } else {
        (var_rsb_i,)
    }
};
        var_rsb_i = assign3160_e2833;
        var_rsb_i_rv = 0.0;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn1_t_rv_slot = var_betn1_t_rv;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn2_t_rv_slot = var_betn2_t_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_cs_t_slot = var_cs_t;
        *var_cs_t_rv_slot = var_cs_t_rv;
        *var_csbi_i_slot = var_csbi_i;
        *var_csbi_i_rv_slot = var_csbi_i_rv;
        *var_csfi_i_slot = var_csfi_i;
        *var_csfi_i_rv_slot = var_csfi_i_rv;
        *var_csthr_i_slot = var_csthr_i;
        *var_csthr_i_rv_slot = var_csthr_i_rv;
        *var_csthrb_i_slot = var_csthrb_i;
        *var_csthrb_i_rv_slot = var_csthrb_i_rv;
        *var_feta_i_slot = var_feta_i;
        *var_feta_i_rv_slot = var_feta_i_rv;
        *var_ge_slot = var_ge;
        *var_ge_dn4_slot = var_ge_dn4;
        *var_ge_dn6_slot = var_ge_dn6;
        *var_ge_dn7_slot = var_ge_dn7;
        *var_ge_dn8_slot = var_ge_dn8;
        *var_ge_dn9_slot = var_ge_dn9;
        *var_ge_rv_slot = var_ge_rv;
        *var_gpe_slot = var_gpe;
        *var_gpe_dn4_slot = var_gpe_dn4;
        *var_gpe_dn6_slot = var_gpe_dn6;
        *var_gpe_dn7_slot = var_gpe_dn7;
        *var_gpe_dn8_slot = var_gpe_dn8;
        *var_gpe_dn9_slot = var_gpe_dn9;
        *var_gpe_rv_slot = var_gpe_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_gwe_slot = var_gwe;
        *var_gwe_rv_slot = var_gwe_rv;
        *var_mue_t_slot = var_mue_t;
        *var_mue_t_rv_slot = var_mue_t_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_rv_slot = var_rs_t_rv;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsb_i_rv_slot = var_rsb_i_rv;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsg_i_rv_slot = var_rsg_i_rv;
        *var_rsig_i_slot = var_rsig_i;
        *var_rsig_i_rv_slot = var_rsig_i_rv;
        *var_stbet_i_slot = var_stbet_i;
        *var_stbet_i_rv_slot = var_stbet_i_rv;
        *var_stcs_i_slot = var_stcs_i;
        *var_stcs_i_rv_slot = var_stcs_i_rv;
        *var_stmue_i_slot = var_stmue_i;
        *var_stmue_i_rv_slot = var_stmue_i_rv;
        *var_strs_i_slot = var_strs_i;
        *var_strs_i_rv_slot = var_strs_i_rv;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthecs_i_rv_slot = var_stthecs_i_rv;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthemu_i_rv_slot = var_stthemu_i_rv;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_stxcor_i_rv_slot = var_stxcor_i_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp3_rv_slot = var_temp3_rv;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp4_rv_slot = var_temp4_rv;
        *var_thecs_t_slot = var_thecs_t;
        *var_thecs_t_rv_slot = var_thecs_t_rv;
        *var_themu_t_slot = var_themu_t;
        *var_themu_t_rv_slot = var_themu_t_rv;
        *var_thersg_i_slot = var_thersg_i;
        *var_thersg_i_rv_slot = var_thersg_i_rv;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcor_t_rv_slot = var_xcor_t_rv;
        *var_xcorb_i_slot = var_xcorb_i;
        *var_xcorb_i_rv_slot = var_xcorb_i_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_ge: f64,
        var_ge_dn4: f64,
        var_ge_dn6: f64,
        var_ge_dn7: f64,
        var_ge_dn8: f64,
        var_ge_dn9: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_i_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_i_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_alpb_i_slot: &mut f64,
        var_alpb_i_rv_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_ax_i_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidld_t_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_chib_i_rv_slot: &mut f64,
        var_gc2ch_i_slot: &mut f64,
        var_gc2ch_i_rv_slot: &mut f64,
        var_gc2ovacc_i_slot: &mut f64,
        var_gc2ovacc_i_rv_slot: &mut f64,
        var_gc2ovinv_i_slot: &mut f64,
        var_gc2ovinv_i_rv_slot: &mut f64,
        var_gc3ch_i_slot: &mut f64,
        var_gc3ch_i_rv_slot: &mut f64,
        var_gc3ovacc_i_slot: &mut f64,
        var_gc3ovacc_i_rv_slot: &mut f64,
        var_gc3ovinv_i_slot: &mut f64,
        var_gc3ovinv_i_rv_slot: &mut f64,
        var_gcdov_i_slot: &mut f64,
        var_gcdov_i_rv_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gco_i_rv_slot: &mut f64,
        var_gcovinvfn_i_slot: &mut f64,
        var_gcovinvfn_i_rv_slot: &mut f64,
        var_gcvdov_i_slot: &mut f64,
        var_gcvdov_i_rv_slot: &mut f64,
        var_iginv_t_slot: &mut f64,
        var_iginv_t_rv_slot: &mut f64,
        var_igovacc_t_slot: &mut f64,
        var_igovacc_t_rv_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovaccd_t_rv_slot: &mut f64,
        var_igovinv_t_slot: &mut f64,
        var_igovinv_t_rv_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_igovinvd_t_rv_slot: &mut f64,
        var_niginv_i_slot: &mut f64,
        var_niginv_i_rv_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidl_i_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stig_i_rv_slot: &mut f64,
        var_stigfn_i_slot: &mut f64,
        var_stigfn_i_rv_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stthesat_i_rv_slot: &mut f64,
        var_thesat1_i_slot: &mut f64,
        var_thesat1_i_rv_slot: &mut f64,
        var_thesat2_i_slot: &mut f64,
        var_thesat2_i_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vp_i_rv_slot: &mut f64,
        var_vpg_i_slot: &mut f64,
        var_vpg_i_rv_slot: &mut f64,
    ) {
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_i_rv: f64 = *var_alp1_i_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_i_rv: f64 = *var_alp_i_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_alpb_i: f64 = *var_alpb_i_slot;
        let mut var_alpb_i_rv: f64 = *var_alpb_i_rv_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_i_rv: f64 = *var_ax_i_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidld_t_rv: f64 = *var_bgidld_t_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_chib_i_rv: f64 = *var_chib_i_rv_slot;
        let mut var_gc2ch_i: f64 = *var_gc2ch_i_slot;
        let mut var_gc2ch_i_rv: f64 = *var_gc2ch_i_rv_slot;
        let mut var_gc2ovacc_i: f64 = *var_gc2ovacc_i_slot;
        let mut var_gc2ovacc_i_rv: f64 = *var_gc2ovacc_i_rv_slot;
        let mut var_gc2ovinv_i: f64 = *var_gc2ovinv_i_slot;
        let mut var_gc2ovinv_i_rv: f64 = *var_gc2ovinv_i_rv_slot;
        let mut var_gc3ch_i: f64 = *var_gc3ch_i_slot;
        let mut var_gc3ch_i_rv: f64 = *var_gc3ch_i_rv_slot;
        let mut var_gc3ovacc_i: f64 = *var_gc3ovacc_i_slot;
        let mut var_gc3ovacc_i_rv: f64 = *var_gc3ovacc_i_rv_slot;
        let mut var_gc3ovinv_i: f64 = *var_gc3ovinv_i_slot;
        let mut var_gc3ovinv_i_rv: f64 = *var_gc3ovinv_i_rv_slot;
        let mut var_gcdov_i: f64 = *var_gcdov_i_slot;
        let mut var_gcdov_i_rv: f64 = *var_gcdov_i_rv_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gco_i_rv: f64 = *var_gco_i_rv_slot;
        let mut var_gcovinvfn_i: f64 = *var_gcovinvfn_i_slot;
        let mut var_gcovinvfn_i_rv: f64 = *var_gcovinvfn_i_rv_slot;
        let mut var_gcvdov_i: f64 = *var_gcvdov_i_slot;
        let mut var_gcvdov_i_rv: f64 = *var_gcvdov_i_rv_slot;
        let mut var_iginv_t: f64 = *var_iginv_t_slot;
        let mut var_iginv_t_rv: f64 = *var_iginv_t_rv_slot;
        let mut var_igovacc_t: f64 = *var_igovacc_t_slot;
        let mut var_igovacc_t_rv: f64 = *var_igovacc_t_rv_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovaccd_t_rv: f64 = *var_igovaccd_t_rv_slot;
        let mut var_igovinv_t: f64 = *var_igovinv_t_slot;
        let mut var_igovinv_t_rv: f64 = *var_igovinv_t_rv_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_igovinvd_t_rv: f64 = *var_igovinvd_t_rv_slot;
        let mut var_niginv_i: f64 = *var_niginv_i_slot;
        let mut var_niginv_i_rv: f64 = *var_niginv_i_rv_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidl_i_rv: f64 = *var_stbgidl_i_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stig_i_rv: f64 = *var_stig_i_rv_slot;
        let mut var_stigfn_i: f64 = *var_stigfn_i_slot;
        let mut var_stigfn_i_rv: f64 = *var_stigfn_i_rv_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stthesat_i_rv: f64 = *var_stthesat_i_rv_slot;
        let mut var_thesat1_i: f64 = *var_thesat1_i_slot;
        let mut var_thesat1_i_rv: f64 = *var_thesat1_i_rv_slot;
        let mut var_thesat2_i: f64 = *var_thesat2_i_slot;
        let mut var_thesat2_i_rv: f64 = *var_thesat2_i_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vp_i_rv: f64 = *var_vp_i_rv_slot;
        let mut var_vpg_i: f64 = *var_vpg_i_slot;
        let mut var_vpg_i_rv: f64 = *var_vpg_i_rv_slot;

        let (assign3170_e2858, assign3170_e2858_d_n4, assign3170_e2858_d_n6, assign3170_e2858_d_n7, assign3170_e2858_d_n8, assign3170_e2858_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3170_e2841: f64 = (var_ile).powf(p.p295);
        let assign3170_e2842: f64 = (p.p294 * assign3170_e2841);
        let assign3170_e2843: f64 = (p.p293 + assign3170_e2842);
        let assign3170_e2844: f64 = (var_ge * assign3170_e2843);
        let assign3170_e2848: f64 = (p.p296 * var_iwe);
        let assign3170_e2849: f64 = (1.0 + assign3170_e2848);
        let assign3170_e2850: f64 = (assign3170_e2844 * assign3170_e2849);
        let assign3170_e2854: f64 = (p.p297 * var_iae);
        let assign3170_e2855: f64 = (1.0 + assign3170_e2854);
        let assign3170_e2856: f64 = (assign3170_e2850 * assign3170_e2855);
        (assign3170_e2856, (((var_ge_dn4 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn6 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn7 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn8 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn9 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign3170_e2858;
        var_thesat_p_dn4 = assign3170_e2858_d_n4;
        var_thesat_p_dn6 = assign3170_e2858_d_n6;
        var_thesat_p_dn7 = assign3170_e2858_d_n7;
        var_thesat_p_dn8 = assign3170_e2858_d_n8;
        var_thesat_p_dn9 = assign3170_e2858_d_n9;
        var_thesat_p_rv = 0.0;

        let (assign3180_e2865, assign3180_e2865_d_n4, assign3180_e2865_d_n6, assign3180_e2865_d_n7, assign3180_e2865_d_n8, assign3180_e2865_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3180_e2863: f64 = (var_thesat_p).max(0.0);
        (assign3180_e2863, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign3180_e2865;
        var_thesat_t_dn4 = assign3180_e2865_d_n4;
        var_thesat_t_dn6 = assign3180_e2865_d_n6;
        var_thesat_t_dn7 = assign3180_e2865_d_n7;
        var_thesat_t_dn8 = assign3180_e2865_d_n8;
        var_thesat_t_dn9 = assign3180_e2865_d_n9;
        var_thesat_t_rv = 0.0;

        let (assign3190_e2888,) = {
    if (var_guard83 == 0.0) {
        let assign3190_e2872: f64 = (p.p299 * var_ile);
        let assign3190_e2873: f64 = (1.0 + assign3190_e2872);
        let assign3190_e2874: f64 = (p.p298 * assign3190_e2873);
        let assign3190_e2878: f64 = (p.p300 * var_iwe);
        let assign3190_e2879: f64 = (1.0 + assign3190_e2878);
        let assign3190_e2880: f64 = (assign3190_e2874 * assign3190_e2879);
        let assign3190_e2884: f64 = (p.p301 * var_iae);
        let assign3190_e2885: f64 = (1.0 + assign3190_e2884);
        let assign3190_e2886: f64 = (assign3190_e2880 * assign3190_e2885);
        (assign3190_e2886,)
    } else {
        (var_stthesat_i,)
    }
};
        var_stthesat_i = assign3190_e2888;
        var_stthesat_i_rv = 0.0;

        let (assign3200_e2893,) = {
    if (var_guard83 == 0.0) {
        (p.p302,)
    } else {
        (var_thesat1_i,)
    }
};
        var_thesat1_i = assign3200_e2893;
        var_thesat1_i_rv = 0.0;

        let (assign3210_e2898,) = {
    if (var_guard83 == 0.0) {
        (p.p303,)
    } else {
        (var_thesat2_i,)
    }
};
        var_thesat2_i = assign3210_e2898;
        var_thesat2_i_rv = 0.0;

        let (assign3220_e2919,) = {
    if (var_guard83 == 0.0) {
        let assign3220_e2906: f64 = (var_ile).powf(p.p306);
        let assign3220_e2907: f64 = (p.p305 * assign3220_e2906);
        let assign3220_e2912: f64 = (var_ile).powf(p.p308);
        let assign3220_e2913: f64 = (p.p307 * assign3220_e2912);
        let assign3220_e2914: f64 = (1.0 + assign3220_e2913);
        let assign3220_e2915: f64 = (assign3220_e2907 / assign3220_e2914);
        let assign3220_e2916: f64 = (1.0 + assign3220_e2915);
        let assign3220_e2917: f64 = (p.p304 / assign3220_e2916);
        (assign3220_e2917,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign3220_e2919;
        var_ax_p_rv = 0.0;

        let (assign3230_e2928,) = {
    if (var_guard83 == 0.0) {
        let assign3230_e2924: f64 = (var_ax_p).max(1.0);
        let assign3230_e2926: f64 = (assign3230_e2924).min(16.0);
        (assign3230_e2926,)
    } else {
        (var_ax_i,)
    }
};
        var_ax_i = assign3230_e2928;
        var_ax_i_rv = 0.0;

        let (assign3240_e2951,) = {
    if (var_guard83 == 0.0) {
        let assign3240_e2934: f64 = (var_ile).powf(p.p310);
        let assign3240_e2935: f64 = (p.p309 * assign3240_e2934);
        let assign3240_e2939: f64 = (p.p313 * var_iwe);
        let assign3240_e2940: f64 = (1.0 + assign3240_e2939);
        let assign3240_e2941: f64 = (assign3240_e2935 * assign3240_e2940);
        let assign3240_e2946: f64 = (var_ile).powf(p.p312);
        let assign3240_e2947: f64 = (p.p311 * assign3240_e2946);
        let assign3240_e2948: f64 = (1.0 + assign3240_e2947);
        let assign3240_e2949: f64 = (assign3240_e2941 / assign3240_e2948);
        (assign3240_e2949,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign3240_e2951;
        var_alp_p_rv = 0.0;

        let (assign3250_e2958,) = {
    if (var_guard83 == 0.0) {
        let assign3250_e2956: f64 = (var_alp_p).max(0.0);
        (assign3250_e2956,)
    } else {
        (var_alp_i,)
    }
};
        var_alp_i = assign3250_e2958;
        var_alp_i_rv = 0.0;

        let (assign3260_e2981,) = {
    if (var_guard83 == 0.0) {
        let assign3260_e2964: f64 = (var_ile).powf(p.p315);
        let assign3260_e2965: f64 = (p.p314 * assign3260_e2964);
        let assign3260_e2969: f64 = (p.p318 * var_iwe);
        let assign3260_e2970: f64 = (1.0 + assign3260_e2969);
        let assign3260_e2971: f64 = (assign3260_e2965 * assign3260_e2970);
        let assign3260_e2976: f64 = (var_ile).powf(p.p317);
        let assign3260_e2977: f64 = (p.p316 * assign3260_e2976);
        let assign3260_e2978: f64 = (1.0 + assign3260_e2977);
        let assign3260_e2979: f64 = (assign3260_e2971 / assign3260_e2978);
        (assign3260_e2979,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign3260_e2981;
        var_alp1_p_rv = 0.0;

        let (assign3270_e2988,) = {
    if (var_guard83 == 0.0) {
        let assign3270_e2986: f64 = (var_alp1_p).max(0.0);
        (assign3270_e2986,)
    } else {
        (var_alp1_i,)
    }
};
        var_alp1_i = assign3270_e2988;
        var_alp1_i_rv = 0.0;

        let (assign3280_e2993,) = {
    if (var_guard83 == 0.0) {
        (p.p319,)
    } else {
        (var_alpb_i,)
    }
};
        var_alpb_i = assign3280_e2993;
        var_alpb_i_rv = 0.0;

        let (assign3290_e2998,) = {
    if (var_guard83 == 0.0) {
        (p.p320,)
    } else {
        (var_vp_i,)
    }
};
        var_vp_i = assign3290_e2998;
        var_vp_i_rv = 0.0;

        let (assign3300_e3003,) = {
    if (var_guard83 == 0.0) {
        (p.p321,)
    } else {
        (var_vpg_i,)
    }
};
        var_vpg_i = assign3300_e3003;
        var_vpg_i_rv = 0.0;

        let (assign3310_e3008,) = {
    if (var_guard83 == 0.0) {
        (p.p322,)
    } else {
        (var_gco_i,)
    }
};
        var_gco_i = assign3310_e3008;
        var_gco_i_rv = 0.0;

        let (assign3320_e3015,) = {
    if (var_guard83 == 0.0) {
        let assign3320_e3013: f64 = (p.p323 / var_iae);
        (assign3320_e3013,)
    } else {
        (var_iginv_t,)
    }
};
        var_iginv_t = assign3320_e3015;
        var_iginv_t_rv = 0.0;

        let (assign3330_e3022,) = {
    if (var_guard83 == 0.0) {
        let assign3330_e3020: f64 = (p.p324 / var_iwe);
        (assign3330_e3020,)
    } else {
        (var_igovinv_t,)
    }
};
        var_igovinv_t = assign3330_e3022;
        var_igovinv_t_rv = 0.0;

        let (assign3340_e3029,) = {
    if (var_guard83 == 0.0) {
        let assign3340_e3027: f64 = (p.p325 / var_iwe);
        (assign3340_e3027,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign3340_e3029;
        var_igovinvd_t_rv = 0.0;

        let (assign3370_e3050,) = {
    if (var_guard83 == 0.0) {
        let assign3370_e3048: f64 = (p.p326 / var_iwe);
        (assign3370_e3048,)
    } else {
        (var_igovacc_t,)
    }
};
        var_igovacc_t = assign3370_e3050;
        var_igovacc_t_rv = 0.0;

        let (assign3380_e3057,) = {
    if (var_guard83 == 0.0) {
        let assign3380_e3055: f64 = (p.p327 / var_iwe);
        (assign3380_e3055,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign3380_e3057;
        var_igovaccd_t_rv = 0.0;

        let (assign3390_e3062,) = {
    if (var_guard83 == 0.0) {
        (p.p328,)
    } else {
        (var_stig_i,)
    }
};
        var_stig_i = assign3390_e3062;
        var_stig_i_rv = 0.0;

        let (assign3400_e3067,) = {
    if (var_guard83 == 0.0) {
        (p.p342,)
    } else {
        (var_stigfn_i,)
    }
};
        var_stigfn_i = assign3400_e3067;
        var_stigfn_i_rv = 0.0;

        let (assign3410_e3072,) = {
    if (var_guard83 == 0.0) {
        (p.p329,)
    } else {
        (var_gc2ch_i,)
    }
};
        var_gc2ch_i = assign3410_e3072;
        var_gc2ch_i_rv = 0.0;

        let (assign3420_e3077,) = {
    if (var_guard83 == 0.0) {
        (p.p330,)
    } else {
        (var_gc3ch_i,)
    }
};
        var_gc3ch_i = assign3420_e3077;
        var_gc3ch_i_rv = 0.0;

        let (assign3430_e3082,) = {
    if (var_guard83 == 0.0) {
        (p.p331,)
    } else {
        (var_gc2ovinv_i,)
    }
};
        var_gc2ovinv_i = assign3430_e3082;
        var_gc2ovinv_i_rv = 0.0;

        let (assign3440_e3087,) = {
    if (var_guard83 == 0.0) {
        (p.p341,)
    } else {
        (var_gcovinvfn_i,)
    }
};
        var_gcovinvfn_i = assign3440_e3087;
        var_gcovinvfn_i_rv = 0.0;

        let (assign3450_e3092,) = {
    if (var_guard83 == 0.0) {
        (p.p332,)
    } else {
        (var_gc3ovinv_i,)
    }
};
        var_gc3ovinv_i = assign3450_e3092;
        var_gc3ovinv_i_rv = 0.0;

        let (assign3460_e3097,) = {
    if (var_guard83 == 0.0) {
        (p.p333,)
    } else {
        (var_gc2ovacc_i,)
    }
};
        var_gc2ovacc_i = assign3460_e3097;
        var_gc2ovacc_i_rv = 0.0;

        let (assign3470_e3102,) = {
    if (var_guard83 == 0.0) {
        (p.p334,)
    } else {
        (var_gc3ovacc_i,)
    }
};
        var_gc3ovacc_i = assign3470_e3102;
        var_gc3ovacc_i_rv = 0.0;

        let (assign3480_e3109,) = {
    if (var_guard83 == 0.0) {
        let assign3480_e3107: f64 = (p.p335 * var_ile);
        (assign3480_e3107,)
    } else {
        (var_gcdov_i,)
    }
};
        var_gcdov_i = assign3480_e3109;
        var_gcdov_i_rv = 0.0;

        let (assign3490_e3114,) = {
    if (var_guard83 == 0.0) {
        (p.p336,)
    } else {
        (var_gcvdov_i,)
    }
};
        var_gcvdov_i = assign3490_e3114;
        var_gcvdov_i_rv = 0.0;

        let (assign3500_e3119,) = {
    if (var_guard83 == 0.0) {
        (p.p337,)
    } else {
        (var_chib_i,)
    }
};
        var_chib_i = assign3500_e3119;
        var_chib_i_rv = 0.0;

        let (assign3510_e3124,) = {
    if (var_guard83 == 0.0) {
        (p.p338,)
    } else {
        (var_niginv_i,)
    }
};
        var_niginv_i = assign3510_e3124;
        var_niginv_i_rv = 0.0;

        let (assign3520_e3133,) = {
    if (var_guard83 == 0.0) {
        let assign3520_e3130: f64 = (p.p345 / var_iwe);
        let assign3520_e3131: f64 = (p.p343 + assign3520_e3130);
        (assign3520_e3131,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign3520_e3133;
        var_agidl_p_rv = 0.0;

        let (assign3530_e3140, assign3530_e3140_d_n4, assign3530_e3140_d_n6, assign3530_e3140_d_n7, assign3530_e3140_d_n8, assign3530_e3140_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3530_e3138: f64 = (var_agidl_p).max(0.0);
        (assign3530_e3138, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidl_i, var_agidl_i_dn4, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9,)
    }
};
        var_agidl_i = assign3530_e3140;
        var_agidl_i_dn4 = assign3530_e3140_d_n4;
        var_agidl_i_dn6 = assign3530_e3140_d_n6;
        var_agidl_i_dn7 = assign3530_e3140_d_n7;
        var_agidl_i_dn8 = assign3530_e3140_d_n8;
        var_agidl_i_dn9 = assign3530_e3140_d_n9;
        var_agidl_i_rv = 0.0;

        let (assign3540_e3149,) = {
    if (var_guard83 == 0.0) {
        let assign3540_e3146: f64 = (p.p346 / var_iwe);
        let assign3540_e3147: f64 = (p.p344 + assign3540_e3146);
        (assign3540_e3147,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign3540_e3149;
        var_agidld_p_rv = 0.0;

        let (assign3550_e3156, assign3550_e3156_d_n4, assign3550_e3156_d_n6, assign3550_e3156_d_n7, assign3550_e3156_d_n8, assign3550_e3156_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3550_e3154: f64 = (var_agidld_p).max(0.0);
        (assign3550_e3154, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidld_i, var_agidld_i_dn4, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9,)
    }
};
        var_agidld_i = assign3550_e3156;
        var_agidld_i_dn4 = assign3550_e3156_d_n4;
        var_agidld_i_dn6 = assign3550_e3156_d_n6;
        var_agidld_i_dn7 = assign3550_e3156_d_n7;
        var_agidld_i_dn8 = assign3550_e3156_d_n8;
        var_agidld_i_dn9 = assign3550_e3156_d_n9;
        var_agidld_i_rv = 0.0;

        let (assign3560_e3161,) = {
    if (var_guard83 == 0.0) {
        (p.p347,)
    } else {
        (var_bgidl_t,)
    }
};
        var_bgidl_t = assign3560_e3161;
        var_bgidl_t_rv = 0.0;

        let (assign3570_e3166,) = {
    if (var_guard83 == 0.0) {
        (p.p348,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign3570_e3166;
        var_bgidld_t_rv = 0.0;

        let (assign3580_e3171,) = {
    if (var_guard83 == 0.0) {
        (p.p349,)
    } else {
        (var_stbgidl_i,)
    }
};
        var_stbgidl_i = assign3580_e3171;
        var_stbgidl_i_rv = 0.0;

        let (assign3590_e3176,) = {
    if (var_guard83 == 0.0) {
        (p.p350,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign3590_e3176;
        var_stbgidld_i_rv = 0.0;

        let (assign3600_e3181,) = {
    if (var_guard83 == 0.0) {
        (p.p351,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign3600_e3181;
        var_cgidl_i_rv = 0.0;

        let (assign3610_e3186,) = {
    if (var_guard83 == 0.0) {
        (p.p352,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign3610_e3186;
        var_cgidld_i_rv = 0.0;

        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_i_rv_slot = var_alp1_i_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp_i_slot = var_alp_i;
        *var_alp_i_rv_slot = var_alp_i_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_alpb_i_slot = var_alpb_i;
        *var_alpb_i_rv_slot = var_alpb_i_rv;
        *var_ax_i_slot = var_ax_i;
        *var_ax_i_rv_slot = var_ax_i_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidld_t_rv_slot = var_bgidld_t_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_chib_i_slot = var_chib_i;
        *var_chib_i_rv_slot = var_chib_i_rv;
        *var_gc2ch_i_slot = var_gc2ch_i;
        *var_gc2ch_i_rv_slot = var_gc2ch_i_rv;
        *var_gc2ovacc_i_slot = var_gc2ovacc_i;
        *var_gc2ovacc_i_rv_slot = var_gc2ovacc_i_rv;
        *var_gc2ovinv_i_slot = var_gc2ovinv_i;
        *var_gc2ovinv_i_rv_slot = var_gc2ovinv_i_rv;
        *var_gc3ch_i_slot = var_gc3ch_i;
        *var_gc3ch_i_rv_slot = var_gc3ch_i_rv;
        *var_gc3ovacc_i_slot = var_gc3ovacc_i;
        *var_gc3ovacc_i_rv_slot = var_gc3ovacc_i_rv;
        *var_gc3ovinv_i_slot = var_gc3ovinv_i;
        *var_gc3ovinv_i_rv_slot = var_gc3ovinv_i_rv;
        *var_gcdov_i_slot = var_gcdov_i;
        *var_gcdov_i_rv_slot = var_gcdov_i_rv;
        *var_gco_i_slot = var_gco_i;
        *var_gco_i_rv_slot = var_gco_i_rv;
        *var_gcovinvfn_i_slot = var_gcovinvfn_i;
        *var_gcovinvfn_i_rv_slot = var_gcovinvfn_i_rv;
        *var_gcvdov_i_slot = var_gcvdov_i;
        *var_gcvdov_i_rv_slot = var_gcvdov_i_rv;
        *var_iginv_t_slot = var_iginv_t;
        *var_iginv_t_rv_slot = var_iginv_t_rv;
        *var_igovacc_t_slot = var_igovacc_t;
        *var_igovacc_t_rv_slot = var_igovacc_t_rv;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovaccd_t_rv_slot = var_igovaccd_t_rv;
        *var_igovinv_t_slot = var_igovinv_t;
        *var_igovinv_t_rv_slot = var_igovinv_t_rv;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_igovinvd_t_rv_slot = var_igovinvd_t_rv;
        *var_niginv_i_slot = var_niginv_i;
        *var_niginv_i_rv_slot = var_niginv_i_rv;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidl_i_rv_slot = var_stbgidl_i_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_stig_i_slot = var_stig_i;
        *var_stig_i_rv_slot = var_stig_i_rv;
        *var_stigfn_i_slot = var_stigfn_i;
        *var_stigfn_i_rv_slot = var_stigfn_i_rv;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stthesat_i_rv_slot = var_stthesat_i_rv;
        *var_thesat1_i_slot = var_thesat1_i;
        *var_thesat1_i_rv_slot = var_thesat1_i_rv;
        *var_thesat2_i_slot = var_thesat2_i;
        *var_thesat2_i_rv_slot = var_thesat2_i_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_vp_i_slot = var_vp_i;
        *var_vp_i_rv_slot = var_vp_i_rv;
        *var_vpg_i_slot = var_vpg_i;
        *var_vpg_i_rv_slot = var_vpg_i_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_le: f64,
        var_lecv: f64,
        var_lphy: f64,
        var_lphy_dn4: f64,
        var_lphy_dn6: f64,
        var_lphy_dn7: f64,
        var_lphy_dn8: f64,
        var_lphy_dn9: f64,
        var_psce1_i: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_we: f64,
        var_wecv: f64,
        var_wen: f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_i_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_areaq_i_slot: &mut f64,
        var_areaq_i_rv_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_dn6_slot: &mut f64,
        var_betnedge_t_dn7_slot: &mut f64,
        var_betnedge_t_dn8_slot: &mut f64,
        var_betnedge_t_dn9_slot: &mut f64,
        var_betnedge_t_rv_slot: &mut f64,
        var_cf1edge_i_slot: &mut f64,
        var_cf1edge_i_dn4_slot: &mut f64,
        var_cf1edge_i_dn6_slot: &mut f64,
        var_cf1edge_i_dn7_slot: &mut f64,
        var_cf1edge_i_dn8_slot: &mut f64,
        var_cf1edge_i_dn9_slot: &mut f64,
        var_cf1edge_i_rv_slot: &mut f64,
        var_cf2edge_i_slot: &mut f64,
        var_cf2edge_i_dn4_slot: &mut f64,
        var_cf2edge_i_dn6_slot: &mut f64,
        var_cf2edge_i_dn7_slot: &mut f64,
        var_cf2edge_i_dn8_slot: &mut f64,
        var_cf2edge_i_dn9_slot: &mut f64,
        var_cf2edge_i_rv_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cfdedge_i_rv_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_dn4_slot: &mut f64,
        var_cgbov_i_dn6_slot: &mut f64,
        var_cgbov_i_dn7_slot: &mut f64,
        var_cgbov_i_dn8_slot: &mut f64,
        var_cgbov_i_dn9_slot: &mut f64,
        var_cgbov_i_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_dn4_slot: &mut f64,
        var_cgbov_p_dn6_slot: &mut f64,
        var_cgbov_p_dn7_slot: &mut f64,
        var_cgbov_p_dn8_slot: &mut f64,
        var_cgbov_p_dn9_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cic1edge_i_slot: &mut f64,
        var_cic1edge_i_rv_slot: &mut f64,
        var_cic2edge_i_slot: &mut f64,
        var_cic2edge_i_rv_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_ctedge_i_rv_slot: &mut f64,
        var_dgidl_i_slot: &mut f64,
        var_dgidl_i_rv_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_dgidld_i_rv_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fif_i_rv_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_fsceac_i_rv_slot: &mut f64,
        var_nsdac_i_slot: &mut f64,
        var_nsdac_i_rv_slot: &mut f64,
        var_psce1edge_i_slot: &mut f64,
        var_psce1edge_i_dn4_slot: &mut f64,
        var_psce1edge_i_dn6_slot: &mut f64,
        var_psce1edge_i_dn7_slot: &mut f64,
        var_psce1edge_i_dn8_slot: &mut f64,
        var_psce1edge_i_dn9_slot: &mut f64,
        var_psce1edge_i_rv_slot: &mut f64,
        var_psce2edge_i_slot: &mut f64,
        var_psce2edge_i_dn4_slot: &mut f64,
        var_psce2edge_i_dn6_slot: &mut f64,
        var_psce2edge_i_dn7_slot: &mut f64,
        var_psce2edge_i_dn8_slot: &mut f64,
        var_psce2edge_i_dn9_slot: &mut f64,
        var_psce2edge_i_rv_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac1_i_rv_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_sta2_i_rv_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbetedge_i_rv_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_stvfbedge_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_vfb1edge_t_slot: &mut f64,
        var_vfb1edge_t_dn4_slot: &mut f64,
        var_vfb1edge_t_dn6_slot: &mut f64,
        var_vfb1edge_t_dn7_slot: &mut f64,
        var_vfb1edge_t_dn8_slot: &mut f64,
        var_vfb1edge_t_dn9_slot: &mut f64,
        var_vfb1edge_t_rv_slot: &mut f64,
        var_vfb2edge_t_slot: &mut f64,
        var_vfb2edge_t_rv_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac1_t_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
        var_we_edge_slot: &mut f64,
        var_we_edge_rv_slot: &mut f64,
    ) {
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_i_rv: f64 = *var_a3_i_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_areaq_i: f64 = *var_areaq_i_slot;
        let mut var_areaq_i_rv: f64 = *var_areaq_i_rv_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_dn6: f64 = *var_betnedge_t_dn6_slot;
        let mut var_betnedge_t_dn7: f64 = *var_betnedge_t_dn7_slot;
        let mut var_betnedge_t_dn8: f64 = *var_betnedge_t_dn8_slot;
        let mut var_betnedge_t_dn9: f64 = *var_betnedge_t_dn9_slot;
        let mut var_betnedge_t_rv: f64 = *var_betnedge_t_rv_slot;
        let mut var_cf1edge_i: f64 = *var_cf1edge_i_slot;
        let mut var_cf1edge_i_dn4: f64 = *var_cf1edge_i_dn4_slot;
        let mut var_cf1edge_i_dn6: f64 = *var_cf1edge_i_dn6_slot;
        let mut var_cf1edge_i_dn7: f64 = *var_cf1edge_i_dn7_slot;
        let mut var_cf1edge_i_dn8: f64 = *var_cf1edge_i_dn8_slot;
        let mut var_cf1edge_i_dn9: f64 = *var_cf1edge_i_dn9_slot;
        let mut var_cf1edge_i_rv: f64 = *var_cf1edge_i_rv_slot;
        let mut var_cf2edge_i: f64 = *var_cf2edge_i_slot;
        let mut var_cf2edge_i_dn4: f64 = *var_cf2edge_i_dn4_slot;
        let mut var_cf2edge_i_dn6: f64 = *var_cf2edge_i_dn6_slot;
        let mut var_cf2edge_i_dn7: f64 = *var_cf2edge_i_dn7_slot;
        let mut var_cf2edge_i_dn8: f64 = *var_cf2edge_i_dn8_slot;
        let mut var_cf2edge_i_dn9: f64 = *var_cf2edge_i_dn9_slot;
        let mut var_cf2edge_i_rv: f64 = *var_cf2edge_i_rv_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cfdedge_i_rv: f64 = *var_cfdedge_i_rv_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_dn4: f64 = *var_cgbov_i_dn4_slot;
        let mut var_cgbov_i_dn6: f64 = *var_cgbov_i_dn6_slot;
        let mut var_cgbov_i_dn7: f64 = *var_cgbov_i_dn7_slot;
        let mut var_cgbov_i_dn8: f64 = *var_cgbov_i_dn8_slot;
        let mut var_cgbov_i_dn9: f64 = *var_cgbov_i_dn9_slot;
        let mut var_cgbov_i_rv: f64 = *var_cgbov_i_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_dn4: f64 = *var_cgbov_p_dn4_slot;
        let mut var_cgbov_p_dn6: f64 = *var_cgbov_p_dn6_slot;
        let mut var_cgbov_p_dn7: f64 = *var_cgbov_p_dn7_slot;
        let mut var_cgbov_p_dn8: f64 = *var_cgbov_p_dn8_slot;
        let mut var_cgbov_p_dn9: f64 = *var_cgbov_p_dn9_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cic1edge_i: f64 = *var_cic1edge_i_slot;
        let mut var_cic1edge_i_rv: f64 = *var_cic1edge_i_rv_slot;
        let mut var_cic2edge_i: f64 = *var_cic2edge_i_slot;
        let mut var_cic2edge_i_rv: f64 = *var_cic2edge_i_rv_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_ctedge_i_rv: f64 = *var_ctedge_i_rv_slot;
        let mut var_dgidl_i: f64 = *var_dgidl_i_slot;
        let mut var_dgidl_i_rv: f64 = *var_dgidl_i_rv_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_dgidld_i_rv: f64 = *var_dgidld_i_rv_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fif_i_rv: f64 = *var_fif_i_rv_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_fsceac_i_rv: f64 = *var_fsceac_i_rv_slot;
        let mut var_nsdac_i: f64 = *var_nsdac_i_slot;
        let mut var_nsdac_i_rv: f64 = *var_nsdac_i_rv_slot;
        let mut var_psce1edge_i: f64 = *var_psce1edge_i_slot;
        let mut var_psce1edge_i_dn4: f64 = *var_psce1edge_i_dn4_slot;
        let mut var_psce1edge_i_dn6: f64 = *var_psce1edge_i_dn6_slot;
        let mut var_psce1edge_i_dn7: f64 = *var_psce1edge_i_dn7_slot;
        let mut var_psce1edge_i_dn8: f64 = *var_psce1edge_i_dn8_slot;
        let mut var_psce1edge_i_dn9: f64 = *var_psce1edge_i_dn9_slot;
        let mut var_psce1edge_i_rv: f64 = *var_psce1edge_i_rv_slot;
        let mut var_psce2edge_i: f64 = *var_psce2edge_i_slot;
        let mut var_psce2edge_i_dn4: f64 = *var_psce2edge_i_dn4_slot;
        let mut var_psce2edge_i_dn6: f64 = *var_psce2edge_i_dn6_slot;
        let mut var_psce2edge_i_dn7: f64 = *var_psce2edge_i_dn7_slot;
        let mut var_psce2edge_i_dn8: f64 = *var_psce2edge_i_dn8_slot;
        let mut var_psce2edge_i_dn9: f64 = *var_psce2edge_i_dn9_slot;
        let mut var_psce2edge_i_rv: f64 = *var_psce2edge_i_rv_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac1_i_rv: f64 = *var_psceac1_i_rv_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_sta2_i_rv: f64 = *var_sta2_i_rv_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbetedge_i_rv: f64 = *var_stbetedge_i_rv_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_stvfbedge_i_rv: f64 = *var_stvfbedge_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_vfb1edge_t: f64 = *var_vfb1edge_t_slot;
        let mut var_vfb1edge_t_dn4: f64 = *var_vfb1edge_t_dn4_slot;
        let mut var_vfb1edge_t_dn6: f64 = *var_vfb1edge_t_dn6_slot;
        let mut var_vfb1edge_t_dn7: f64 = *var_vfb1edge_t_dn7_slot;
        let mut var_vfb1edge_t_dn8: f64 = *var_vfb1edge_t_dn8_slot;
        let mut var_vfb1edge_t_dn9: f64 = *var_vfb1edge_t_dn9_slot;
        let mut var_vfb1edge_t_rv: f64 = *var_vfb1edge_t_rv_slot;
        let mut var_vfb2edge_t: f64 = *var_vfb2edge_t_slot;
        let mut var_vfb2edge_t_rv: f64 = *var_vfb2edge_t_rv_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac1_t_rv: f64 = *var_vfbac1_t_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;
        let mut var_we_edge_rv: f64 = *var_we_edge_rv_slot;

        let (assign3620_e3195,) = {
    if (var_guard83 == 0.0) {
        let assign3620_e3192: f64 = (p.p355 * var_ile);
        let assign3620_e3193: f64 = (p.p353 + assign3620_e3192);
        (assign3620_e3193,)
    } else {
        (var_dgidl_i,)
    }
};
        var_dgidl_i = assign3620_e3195;
        var_dgidl_i_rv = 0.0;

        let (assign3630_e3204,) = {
    if (var_guard83 == 0.0) {
        let assign3630_e3201: f64 = (p.p356 * var_ile);
        let assign3630_e3202: f64 = (p.p354 + assign3630_e3201);
        (assign3630_e3202,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign3630_e3204;
        var_dgidld_i_rv = 0.0;

        let (assign3660_e3233,) = {
    if (var_guard83 == 0.0) {
        (p.p391,)
    } else {
        (var_a2_t,)
    }
};
        var_a2_t = assign3660_e3233;
        var_a2_t_rv = 0.0;

        let (assign3670_e3238,) = {
    if (var_guard83 == 0.0) {
        (p.p392,)
    } else {
        (var_sta2_i,)
    }
};
        var_sta2_i = assign3670_e3238;
        var_sta2_i_rv = 0.0;

        let (assign3680_e3255,) = {
    if (var_guard83 == 0.0) {
        let assign3680_e3245: f64 = (p.p394 * var_ile);
        let assign3680_e3246: f64 = (1.0 + assign3680_e3245);
        let assign3680_e3247: f64 = (p.p393 * assign3680_e3246);
        let assign3680_e3251: f64 = (p.p395 * var_iwe);
        let assign3680_e3252: f64 = (1.0 + assign3680_e3251);
        let assign3680_e3253: f64 = (assign3680_e3247 * assign3680_e3252);
        (assign3680_e3253,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign3680_e3255;
        var_a3_p_rv = 0.0;

        let (assign3690_e3262,) = {
    if (var_guard83 == 0.0) {
        let assign3690_e3260: f64 = (var_a3_p).max(0.0);
        (assign3690_e3260,)
    } else {
        (var_a3_i,)
    }
};
        var_a3_i = assign3690_e3262;
        var_a3_i_rv = 0.0;

        let (assign3700_e3273,) = {
    if (var_guard83 == 0.0) {
        let assign3700_e3267: f64 = (2.0 * p.p357);
        let assign3700_e3270: f64 = (p.p358 * var_we);
        let assign3700_e3271: f64 = (assign3700_e3267 + assign3700_e3270);
        (assign3700_e3271,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign3700_e3273;
        var_we_edge_rv = 0.0;

        let (assign3710_e3278,) = {
    if (var_guard83 == 0.0) {
        (p.p359,)
    } else {
        (var_ctedge_i,)
    }
};
        var_ctedge_i = assign3710_e3278;
        var_ctedge_i_rv = 0.0;

        let (assign3720_e3287, assign3720_e3287_d_n4, assign3720_e3287_d_n6, assign3720_e3287_d_n7, assign3720_e3287_d_n8, assign3720_e3287_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3720_e3284: f64 = (var_ile).powf(p.p362);
        let assign3720_e3285: f64 = (p.p361 * assign3720_e3284);
        (assign3720_e3285, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3720_e3287;
        var_temp_dn4 = assign3720_e3287_d_n4;
        var_temp_dn6 = assign3720_e3287_d_n6;
        var_temp_dn7 = assign3720_e3287_d_n7;
        var_temp_dn8 = assign3720_e3287_d_n8;
        var_temp_dn9 = assign3720_e3287_d_n9;
        var_temp_rv = 0.0;

        let (assign3730_e3302, assign3730_e3302_d_n4, assign3730_e3302_d_n6, assign3730_e3302_d_n7, assign3730_e3302_d_n8, assign3730_e3302_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3730_e3292: f64 = (p.p360 + var_temp);
        let assign3730_e3295: f64 = (p.p363 * var_iwe);
        let assign3730_e3296: f64 = (assign3730_e3292 + assign3730_e3295);
        let assign3730_e3299: f64 = (p.p364 * var_iae);
        let assign3730_e3300: f64 = (assign3730_e3296 + assign3730_e3299);
        (assign3730_e3300, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfb1edge_t, var_vfb1edge_t_dn4, var_vfb1edge_t_dn6, var_vfb1edge_t_dn7, var_vfb1edge_t_dn8, var_vfb1edge_t_dn9,)
    }
};
        var_vfb1edge_t = assign3730_e3302;
        var_vfb1edge_t_dn4 = assign3730_e3302_d_n4;
        var_vfb1edge_t_dn6 = assign3730_e3302_d_n6;
        var_vfb1edge_t_dn7 = assign3730_e3302_d_n7;
        var_vfb1edge_t_dn8 = assign3730_e3302_d_n8;
        var_vfb1edge_t_dn9 = assign3730_e3302_d_n9;
        var_vfb1edge_t_rv = 0.0;

        let (assign3740_e3307,) = {
    if (var_guard83 == 0.0) {
        (p.p365,)
    } else {
        (var_vfb2edge_t,)
    }
};
        var_vfb2edge_t = assign3740_e3307;
        var_vfb2edge_t_rv = 0.0;

        let (assign3750_e3330,) = {
    if (var_guard83 == 0.0) {
        let assign3750_e3314: f64 = (p.p367 * var_ile);
        let assign3750_e3315: f64 = (1.0 + assign3750_e3314);
        let assign3750_e3316: f64 = (p.p366 * assign3750_e3315);
        let assign3750_e3320: f64 = (p.p368 * var_iwe);
        let assign3750_e3321: f64 = (1.0 + assign3750_e3320);
        let assign3750_e3322: f64 = (assign3750_e3316 * assign3750_e3321);
        let assign3750_e3326: f64 = (p.p369 * var_iae);
        let assign3750_e3327: f64 = (1.0 + assign3750_e3326);
        let assign3750_e3328: f64 = (assign3750_e3322 * assign3750_e3327);
        (assign3750_e3328,)
    } else {
        (var_stvfbedge_i,)
    }
};
        var_stvfbedge_i = assign3750_e3330;
        var_stvfbedge_i_rv = 0.0;

        let (assign3760_e3335,) = {
    if (var_guard83 == 0.0) {
        (p.p370,)
    } else {
        (var_cic1edge_i,)
    }
};
        var_cic1edge_i = assign3760_e3335;
        var_cic1edge_i_rv = 0.0;

        let (assign3770_e3340,) = {
    if (var_guard83 == 0.0) {
        (p.p371,)
    } else {
        (var_cic2edge_i,)
    }
};
        var_cic2edge_i = assign3770_e3340;
        var_cic2edge_i_rv = 0.0;

        let (assign3780_e3357, assign3780_e3357_d_n4, assign3780_e3357_d_n6, assign3780_e3357_d_n7, assign3780_e3357_d_n8, assign3780_e3357_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3780_e3345: f64 = (p.p372 * 2.0);
        let assign3780_e3348: f64 = (var_lambda_le).powf(p.p373);
        let assign3780_e3349: f64 = (assign3780_e3345 * assign3780_e3348);
        let assign3780_e3353: f64 = (p.p374 * var_iwe);
        let assign3780_e3354: f64 = (1.0 + assign3780_e3353);
        let assign3780_e3355: f64 = (assign3780_e3349 * assign3780_e3354);
        (assign3780_e3355, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3780_e3357;
        var_temp_dn4 = assign3780_e3357_d_n4;
        var_temp_dn6 = assign3780_e3357_d_n6;
        var_temp_dn7 = assign3780_e3357_d_n7;
        var_temp_dn8 = assign3780_e3357_d_n8;
        var_temp_dn9 = assign3780_e3357_d_n9;
        var_temp_rv = 0.0;

        let (assign3790_e3366, assign3790_e3366_d_n4, assign3790_e3366_d_n6, assign3790_e3366_d_n7, assign3790_e3366_d_n8, assign3790_e3366_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3790_e3362: f64 = (var_temp).max(0.0);
        let assign3790_e3364: f64 = (assign3790_e3362).min(5.0);
        (assign3790_e3364, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_psce1edge_i, var_psce1edge_i_dn4, var_psce1edge_i_dn6, var_psce1edge_i_dn7, var_psce1edge_i_dn8, var_psce1edge_i_dn9,)
    }
};
        var_psce1edge_i = assign3790_e3366;
        var_psce1edge_i_dn4 = assign3790_e3366_d_n4;
        var_psce1edge_i_dn6 = assign3790_e3366_d_n6;
        var_psce1edge_i_dn7 = assign3790_e3366_d_n7;
        var_psce1edge_i_dn8 = assign3790_e3366_d_n8;
        var_psce1edge_i_dn9 = assign3790_e3366_d_n9;
        var_psce1edge_i_rv = 0.0;

        let (assign3800_e3377, assign3800_e3377_d_n4, assign3800_e3377_d_n6, assign3800_e3377_d_n7, assign3800_e3377_d_n8, assign3800_e3377_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3800_e3371: f64 = (p.p375 * var_psce1edge_i);
        let assign3800_e3373: f64 = (assign3800_e3371 * var_tox2_i);
        let assign3800_e3375: f64 = (assign3800_e3373 / var_tox1_i);
        (assign3800_e3375, (((p.p375 * var_psce1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_psce2edge_i, var_psce2edge_i_dn4, var_psce2edge_i_dn6, var_psce2edge_i_dn7, var_psce2edge_i_dn8, var_psce2edge_i_dn9,)
    }
};
        var_psce2edge_i = assign3800_e3377;
        var_psce2edge_i_dn4 = assign3800_e3377_d_n4;
        var_psce2edge_i_dn6 = assign3800_e3377_d_n6;
        var_psce2edge_i_dn7 = assign3800_e3377_d_n7;
        var_psce2edge_i_dn8 = assign3800_e3377_d_n8;
        var_psce2edge_i_dn9 = assign3800_e3377_d_n9;
        var_psce2edge_i_rv = 0.0;

        let (assign3810_e3390, assign3810_e3390_d_n4, assign3810_e3390_d_n6, assign3810_e3390_d_n7, assign3810_e3390_d_n8, assign3810_e3390_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3810_e3382: f64 = (var_lambda_le).powf(p.p377);
        let assign3810_e3386: f64 = (p.p378 * var_iwe);
        let assign3810_e3387: f64 = (1.0 + assign3810_e3386);
        let assign3810_e3388: f64 = (assign3810_e3382 * assign3810_e3387);
        (assign3810_e3388, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3810_e3390;
        var_temp_dn4 = assign3810_e3390_d_n4;
        var_temp_dn6 = assign3810_e3390_d_n6;
        var_temp_dn7 = assign3810_e3390_d_n7;
        var_temp_dn8 = assign3810_e3390_d_n8;
        var_temp_dn9 = assign3810_e3390_d_n9;
        var_temp_rv = 0.0;

        let (assign3820_e3397, assign3820_e3397_d_n4, assign3820_e3397_d_n6, assign3820_e3397_d_n7, assign3820_e3397_d_n8, assign3820_e3397_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3820_e3395: f64 = (p.p376 * var_temp);
        (assign3820_e3395, (p.p376 * var_temp_dn4), (p.p376 * var_temp_dn6), (p.p376 * var_temp_dn7), (p.p376 * var_temp_dn8), (p.p376 * var_temp_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3820_e3397;
        var_temp_dn4 = assign3820_e3397_d_n4;
        var_temp_dn6 = assign3820_e3397_d_n6;
        var_temp_dn7 = assign3820_e3397_d_n7;
        var_temp_dn8 = assign3820_e3397_d_n8;
        var_temp_dn9 = assign3820_e3397_d_n9;
        var_temp_rv = 0.0;

        let (assign3830_e3404, assign3830_e3404_d_n4, assign3830_e3404_d_n6, assign3830_e3404_d_n7, assign3830_e3404_d_n8, assign3830_e3404_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3830_e3402: f64 = (var_temp).max(0.0);
        (assign3830_e3402, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_cf1edge_i, var_cf1edge_i_dn4, var_cf1edge_i_dn6, var_cf1edge_i_dn7, var_cf1edge_i_dn8, var_cf1edge_i_dn9,)
    }
};
        var_cf1edge_i = assign3830_e3404;
        var_cf1edge_i_dn4 = assign3830_e3404_d_n4;
        var_cf1edge_i_dn6 = assign3830_e3404_d_n6;
        var_cf1edge_i_dn7 = assign3830_e3404_d_n7;
        var_cf1edge_i_dn8 = assign3830_e3404_d_n8;
        var_cf1edge_i_dn9 = assign3830_e3404_d_n9;
        var_cf1edge_i_rv = 0.0;

        let (assign3840_e3415, assign3840_e3415_d_n4, assign3840_e3415_d_n6, assign3840_e3415_d_n7, assign3840_e3415_d_n8, assign3840_e3415_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3840_e3409: f64 = (p.p379 * var_cf1edge_i);
        let assign3840_e3411: f64 = (assign3840_e3409 * var_tox2_i);
        let assign3840_e3413: f64 = (assign3840_e3411 / var_tox1_i);
        (assign3840_e3413, (((p.p379 * var_cf1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2edge_i, var_cf2edge_i_dn4, var_cf2edge_i_dn6, var_cf2edge_i_dn7, var_cf2edge_i_dn8, var_cf2edge_i_dn9,)
    }
};
        var_cf2edge_i = assign3840_e3415;
        var_cf2edge_i_dn4 = assign3840_e3415_d_n4;
        var_cf2edge_i_dn6 = assign3840_e3415_d_n6;
        var_cf2edge_i_dn7 = assign3840_e3415_d_n7;
        var_cf2edge_i_dn8 = assign3840_e3415_d_n8;
        var_cf2edge_i_dn9 = assign3840_e3415_d_n9;
        var_cf2edge_i_rv = 0.0;

        let (assign3850_e3420,) = {
    if (var_guard83 == 0.0) {
        (p.p380,)
    } else {
        (var_cfdedge_i,)
    }
};
        var_cfdedge_i = assign3850_e3420;
        var_cfdedge_i_rv = 0.0;

        let (assign3860_e3439, assign3860_e3439_d_n4, assign3860_e3439_d_n6, assign3860_e3439_d_n7, assign3860_e3439_d_n8, assign3860_e3439_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3860_e3426: f64 = (p.p381 * p.p382);
        let assign3860_e3428: f64 = (assign3860_e3426 / var_le);
        let assign3860_e3431: f64 = (-var_le);
        let assign3860_e3433: f64 = (assign3860_e3431 / p.p382);
        let assign3860_e3434: f64 = (assign3860_e3433).exp();
        let assign3860_e3435: f64 = (1.0 - assign3860_e3434);
        let assign3860_e3436: f64 = (assign3860_e3428 * assign3860_e3435);
        let assign3860_e3437: f64 = (1.0 + assign3860_e3436);
        (assign3860_e3437, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3860_e3439;
        var_temp_dn4 = assign3860_e3439_d_n4;
        var_temp_dn6 = assign3860_e3439_d_n6;
        var_temp_dn7 = assign3860_e3439_d_n7;
        var_temp_dn8 = assign3860_e3439_d_n8;
        var_temp_dn9 = assign3860_e3439_d_n9;
        var_temp_rv = 0.0;

        let (assign3870_e3446, assign3870_e3446_d_n4, assign3870_e3446_d_n6, assign3870_e3446_d_n7, assign3870_e3446_d_n8, assign3870_e3446_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3870_e3444: f64 = (var_temp).max(1e-15);
        (assign3870_e3444, if var_temp >= 1e-15 { var_temp_dn4 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn6 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn7 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn8 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3870_e3446;
        var_temp_dn4 = assign3870_e3446_d_n4;
        var_temp_dn6 = assign3870_e3446_d_n6;
        var_temp_dn7 = assign3870_e3446_d_n7;
        var_temp_dn8 = assign3870_e3446_d_n8;
        var_temp_dn9 = assign3870_e3446_d_n9;
        var_temp_rv = 0.0;

        let (assign3880_e3463, assign3880_e3463_d_n4, assign3880_e3463_d_n6, assign3880_e3463_d_n7, assign3880_e3463_d_n8, assign3880_e3463_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3880_e3451: f64 = (p.p244 * var_we_edge);
        let assign3880_e3454: f64 = (var_temp * var_le);
        let assign3880_e3455: f64 = (assign3880_e3451 / assign3880_e3454);
        let assign3880_e3459: f64 = (p.p383 * var_iwe);
        let assign3880_e3460: f64 = (1.0 + assign3880_e3459);
        let assign3880_e3461: f64 = (assign3880_e3455 * assign3880_e3460);
        (assign3880_e3461, ((-((assign3880_e3451 * (var_temp_dn4 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn6 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn7 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn8 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn9 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460),)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4, var_betnedge_t_dn6, var_betnedge_t_dn7, var_betnedge_t_dn8, var_betnedge_t_dn9,)
    }
};
        var_betnedge_t = assign3880_e3463;
        var_betnedge_t_dn4 = assign3880_e3463_d_n4;
        var_betnedge_t_dn6 = assign3880_e3463_d_n6;
        var_betnedge_t_dn7 = assign3880_e3463_d_n7;
        var_betnedge_t_dn8 = assign3880_e3463_d_n8;
        var_betnedge_t_dn9 = assign3880_e3463_d_n9;
        var_betnedge_t_rv = 0.0;

        let (assign3890_e3482,) = {
    if (var_guard83 == 0.0) {
        let assign3890_e3469: f64 = (p.p385 * var_ile);
        let assign3890_e3470: f64 = (p.p384 + assign3890_e3469);
        let assign3890_e3473: f64 = (p.p386 * var_iwe);
        let assign3890_e3474: f64 = (assign3890_e3470 + assign3890_e3473);
        let assign3890_e3477: f64 = (p.p387 * var_ile);
        let assign3890_e3479: f64 = (assign3890_e3477 * var_iwe);
        let assign3890_e3480: f64 = (assign3890_e3474 + assign3890_e3479);
        (assign3890_e3480,)
    } else {
        (var_stbetedge_i,)
    }
};
        var_stbetedge_i = assign3890_e3482;
        var_stbetedge_i_rv = 0.0;

        let (assign3900_e3489,) = {
    if (var_guard83 == 0.0) {
        let assign3900_e3487: f64 = (var_wecv * var_lecv);
        (assign3900_e3487,)
    } else {
        (var_areaq_i,)
    }
};
        var_areaq_i = assign3900_e3489;
        var_areaq_i_rv = 0.0;

        let (assign3910_e3498, assign3910_e3498_d_n4, assign3910_e3498_d_n6, assign3910_e3498_d_n7, assign3910_e3498_d_n8, assign3910_e3498_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3910_e3495: f64 = (p.p397 * var_lphy);
        let assign3910_e3496: f64 = (p.p396 + assign3910_e3495);
        (assign3910_e3496, (p.p397 * var_lphy_dn4), (p.p397 * var_lphy_dn6), (p.p397 * var_lphy_dn7), (p.p397 * var_lphy_dn8), (p.p397 * var_lphy_dn9),)
    } else {
        (var_cgbov_p, var_cgbov_p_dn4, var_cgbov_p_dn6, var_cgbov_p_dn7, var_cgbov_p_dn8, var_cgbov_p_dn9,)
    }
};
        var_cgbov_p = assign3910_e3498;
        var_cgbov_p_dn4 = assign3910_e3498_d_n4;
        var_cgbov_p_dn6 = assign3910_e3498_d_n6;
        var_cgbov_p_dn7 = assign3910_e3498_d_n7;
        var_cgbov_p_dn8 = assign3910_e3498_d_n8;
        var_cgbov_p_dn9 = assign3910_e3498_d_n9;
        var_cgbov_p_rv = 0.0;

        let (assign3920_e3505, assign3920_e3505_d_n4, assign3920_e3505_d_n6, assign3920_e3505_d_n7, assign3920_e3505_d_n8, assign3920_e3505_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3920_e3503: f64 = (var_cgbov_p).max(0.0);
        (assign3920_e3503, if var_cgbov_p >= 0.0 { var_cgbov_p_dn4 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn6 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn7 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn8 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn9 } else { 0.0 },)
    } else {
        (var_cgbov_i, var_cgbov_i_dn4, var_cgbov_i_dn6, var_cgbov_i_dn7, var_cgbov_i_dn8, var_cgbov_i_dn9,)
    }
};
        var_cgbov_i = assign3920_e3505;
        var_cgbov_i_dn4 = assign3920_e3505_d_n4;
        var_cgbov_i_dn6 = assign3920_e3505_d_n6;
        var_cgbov_i_dn7 = assign3920_e3505_d_n7;
        var_cgbov_i_dn8 = assign3920_e3505_d_n8;
        var_cgbov_i_dn9 = assign3920_e3505_d_n9;
        var_cgbov_i_rv = 0.0;

        let (assign3930_e3512,) = {
    if (var_guard83 == 0.0) {
        let assign3930_e3510: f64 = (p.p398 * 1000000.0);
        (assign3930_e3510,)
    } else {
        (var_nsdac_i,)
    }
};
        var_nsdac_i = assign3930_e3512;
        var_nsdac_i_rv = 0.0;

        let (assign3940_e3521,) = {
    if (var_guard83 == 0.0) {
        let assign3940_e3517: f64 = (p.p399 * var_wecv);
        let assign3940_e3519: f64 = (assign3940_e3517 / var_wen);
        (assign3940_e3519,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign3940_e3521;
        var_fif_i_rv = 0.0;

        let (assign3950_e3526,) = {
    if (var_guard83 == 0.0) {
        (p.p400,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign3950_e3526;
        var_fsceac_i_rv = 0.0;

        let (assign3960_e3531, assign3960_e3531_d_n4, assign3960_e3531_d_n6, assign3960_e3531_d_n7, assign3960_e3531_d_n8, assign3960_e3531_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign3960_e3531;
        var_vfbac1_t_dn4 = assign3960_e3531_d_n4;
        var_vfbac1_t_dn6 = assign3960_e3531_d_n6;
        var_vfbac1_t_dn7 = assign3960_e3531_d_n7;
        var_vfbac1_t_dn8 = assign3960_e3531_d_n8;
        var_vfbac1_t_dn9 = assign3960_e3531_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign3970_e3536, assign3970_e3536_d_n4, assign3970_e3536_d_n6, assign3970_e3536_d_n7, assign3970_e3536_d_n8, assign3970_e3536_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign3970_e3536;
        var_vfbac2_t_dn4 = assign3970_e3536_d_n4;
        var_vfbac2_t_dn6 = assign3970_e3536_d_n6;
        var_vfbac2_t_dn7 = assign3970_e3536_d_n7;
        var_vfbac2_t_dn8 = assign3970_e3536_d_n8;
        var_vfbac2_t_dn9 = assign3970_e3536_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign3980_e3541,) = {
    if (var_guard83 == 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign3980_e3541;
        var_psceac1_i_rv = 0.0;

        *var_a2_t_slot = var_a2_t;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_a3_i_slot = var_a3_i;
        *var_a3_i_rv_slot = var_a3_i_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_areaq_i_slot = var_areaq_i;
        *var_areaq_i_rv_slot = var_areaq_i_rv;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_dn6_slot = var_betnedge_t_dn6;
        *var_betnedge_t_dn7_slot = var_betnedge_t_dn7;
        *var_betnedge_t_dn8_slot = var_betnedge_t_dn8;
        *var_betnedge_t_dn9_slot = var_betnedge_t_dn9;
        *var_betnedge_t_rv_slot = var_betnedge_t_rv;
        *var_cf1edge_i_slot = var_cf1edge_i;
        *var_cf1edge_i_dn4_slot = var_cf1edge_i_dn4;
        *var_cf1edge_i_dn6_slot = var_cf1edge_i_dn6;
        *var_cf1edge_i_dn7_slot = var_cf1edge_i_dn7;
        *var_cf1edge_i_dn8_slot = var_cf1edge_i_dn8;
        *var_cf1edge_i_dn9_slot = var_cf1edge_i_dn9;
        *var_cf1edge_i_rv_slot = var_cf1edge_i_rv;
        *var_cf2edge_i_slot = var_cf2edge_i;
        *var_cf2edge_i_dn4_slot = var_cf2edge_i_dn4;
        *var_cf2edge_i_dn6_slot = var_cf2edge_i_dn6;
        *var_cf2edge_i_dn7_slot = var_cf2edge_i_dn7;
        *var_cf2edge_i_dn8_slot = var_cf2edge_i_dn8;
        *var_cf2edge_i_dn9_slot = var_cf2edge_i_dn9;
        *var_cf2edge_i_rv_slot = var_cf2edge_i_rv;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cfdedge_i_rv_slot = var_cfdedge_i_rv;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_dn4_slot = var_cgbov_i_dn4;
        *var_cgbov_i_dn6_slot = var_cgbov_i_dn6;
        *var_cgbov_i_dn7_slot = var_cgbov_i_dn7;
        *var_cgbov_i_dn8_slot = var_cgbov_i_dn8;
        *var_cgbov_i_dn9_slot = var_cgbov_i_dn9;
        *var_cgbov_i_rv_slot = var_cgbov_i_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_dn4_slot = var_cgbov_p_dn4;
        *var_cgbov_p_dn6_slot = var_cgbov_p_dn6;
        *var_cgbov_p_dn7_slot = var_cgbov_p_dn7;
        *var_cgbov_p_dn8_slot = var_cgbov_p_dn8;
        *var_cgbov_p_dn9_slot = var_cgbov_p_dn9;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cic1edge_i_slot = var_cic1edge_i;
        *var_cic1edge_i_rv_slot = var_cic1edge_i_rv;
        *var_cic2edge_i_slot = var_cic2edge_i;
        *var_cic2edge_i_rv_slot = var_cic2edge_i_rv;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_ctedge_i_rv_slot = var_ctedge_i_rv;
        *var_dgidl_i_slot = var_dgidl_i;
        *var_dgidl_i_rv_slot = var_dgidl_i_rv;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_dgidld_i_rv_slot = var_dgidld_i_rv;
        *var_fif_i_slot = var_fif_i;
        *var_fif_i_rv_slot = var_fif_i_rv;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_fsceac_i_rv_slot = var_fsceac_i_rv;
        *var_nsdac_i_slot = var_nsdac_i;
        *var_nsdac_i_rv_slot = var_nsdac_i_rv;
        *var_psce1edge_i_slot = var_psce1edge_i;
        *var_psce1edge_i_dn4_slot = var_psce1edge_i_dn4;
        *var_psce1edge_i_dn6_slot = var_psce1edge_i_dn6;
        *var_psce1edge_i_dn7_slot = var_psce1edge_i_dn7;
        *var_psce1edge_i_dn8_slot = var_psce1edge_i_dn8;
        *var_psce1edge_i_dn9_slot = var_psce1edge_i_dn9;
        *var_psce1edge_i_rv_slot = var_psce1edge_i_rv;
        *var_psce2edge_i_slot = var_psce2edge_i;
        *var_psce2edge_i_dn4_slot = var_psce2edge_i_dn4;
        *var_psce2edge_i_dn6_slot = var_psce2edge_i_dn6;
        *var_psce2edge_i_dn7_slot = var_psce2edge_i_dn7;
        *var_psce2edge_i_dn8_slot = var_psce2edge_i_dn8;
        *var_psce2edge_i_dn9_slot = var_psce2edge_i_dn9;
        *var_psce2edge_i_rv_slot = var_psce2edge_i_rv;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac1_i_rv_slot = var_psceac1_i_rv;
        *var_sta2_i_slot = var_sta2_i;
        *var_sta2_i_rv_slot = var_sta2_i_rv;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbetedge_i_rv_slot = var_stbetedge_i_rv;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_stvfbedge_i_rv_slot = var_stvfbedge_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_vfb1edge_t_slot = var_vfb1edge_t;
        *var_vfb1edge_t_dn4_slot = var_vfb1edge_t_dn4;
        *var_vfb1edge_t_dn6_slot = var_vfb1edge_t_dn6;
        *var_vfb1edge_t_dn7_slot = var_vfb1edge_t_dn7;
        *var_vfb1edge_t_dn8_slot = var_vfb1edge_t_dn8;
        *var_vfb1edge_t_dn9_slot = var_vfb1edge_t_dn9;
        *var_vfb1edge_t_rv_slot = var_vfb1edge_t_rv;
        *var_vfb2edge_t_slot = var_vfb2edge_t;
        *var_vfb2edge_t_rv_slot = var_vfb2edge_t_rv;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac1_t_rv_slot = var_vfbac1_t_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
        *var_we_edge_slot = var_we_edge;
        *var_we_edge_rv_slot = var_we_edge_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_cf_p: f64,
        var_cf_p_dn4: f64,
        var_cf_p_dn6: f64,
        var_cf_p_dn7: f64,
        var_cf_p_dn8: f64,
        var_cf_p_dn9: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_psce2_i: f64,
        var_thesat_p: f64,
        var_thesat_p_dn4: f64,
        var_thesat_p_dn6: f64,
        var_thesat_p_dn7: f64,
        var_thesat_p_dn8: f64,
        var_thesat_p_dn9: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac1_t_rv_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac2_t_rv_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfac_p_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac1_i_rv_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_psceac2_i_rv_slot: &mut f64,
        var_psceac_p_slot: &mut f64,
        var_psceac_p_rv_slot: &mut f64,
        var_psceacl_i_slot: &mut f64,
        var_psceacl_i_rv_slot: &mut f64,
        var_psceaclexp_i_slot: &mut f64,
        var_psceaclexp_i_rv_slot: &mut f64,
        var_psceacw_i_slot: &mut f64,
        var_psceacw_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac1_t_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
        var_vfbacl2_i_slot: &mut f64,
        var_vfbacl2_i_rv_slot: &mut f64,
        var_vfbacl_i_slot: &mut f64,
        var_vfbacl_i_rv_slot: &mut f64,
        var_vfbaclexp2_i_slot: &mut f64,
        var_vfbaclexp2_i_rv_slot: &mut f64,
        var_vfbaclexp_i_slot: &mut f64,
        var_vfbaclexp_i_rv_slot: &mut f64,
        var_vfbaclw_i_slot: &mut f64,
        var_vfbaclw_i_rv_slot: &mut f64,
        var_vfbaco_i_slot: &mut f64,
        var_vfbaco_i_rv_slot: &mut f64,
        var_vfbacw_i_slot: &mut f64,
        var_vfbacw_i_rv_slot: &mut f64,
        var_vfbbaco_i_slot: &mut f64,
        var_vfbbaco_i_rv_slot: &mut f64,
        var_vfblbaco_i_slot: &mut f64,
        var_vfblbaco_i_rv_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac1_t_rv: f64 = *var_cfac1_t_rv_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac2_t_rv: f64 = *var_cfac2_t_rv_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfac_p_rv: f64 = *var_cfac_p_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac1_i_rv: f64 = *var_psceac1_i_rv_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_psceac2_i_rv: f64 = *var_psceac2_i_rv_slot;
        let mut var_psceac_p: f64 = *var_psceac_p_slot;
        let mut var_psceac_p_rv: f64 = *var_psceac_p_rv_slot;
        let mut var_psceacl_i: f64 = *var_psceacl_i_slot;
        let mut var_psceacl_i_rv: f64 = *var_psceacl_i_rv_slot;
        let mut var_psceaclexp_i: f64 = *var_psceaclexp_i_slot;
        let mut var_psceaclexp_i_rv: f64 = *var_psceaclexp_i_rv_slot;
        let mut var_psceacw_i: f64 = *var_psceacw_i_slot;
        let mut var_psceacw_i_rv: f64 = *var_psceacw_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac1_t_rv: f64 = *var_vfbac1_t_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;
        let mut var_vfbacl2_i: f64 = *var_vfbacl2_i_slot;
        let mut var_vfbacl2_i_rv: f64 = *var_vfbacl2_i_rv_slot;
        let mut var_vfbacl_i: f64 = *var_vfbacl_i_slot;
        let mut var_vfbacl_i_rv: f64 = *var_vfbacl_i_rv_slot;
        let mut var_vfbaclexp2_i: f64 = *var_vfbaclexp2_i_slot;
        let mut var_vfbaclexp2_i_rv: f64 = *var_vfbaclexp2_i_rv_slot;
        let mut var_vfbaclexp_i: f64 = *var_vfbaclexp_i_slot;
        let mut var_vfbaclexp_i_rv: f64 = *var_vfbaclexp_i_rv_slot;
        let mut var_vfbaclw_i: f64 = *var_vfbaclw_i_slot;
        let mut var_vfbaclw_i_rv: f64 = *var_vfbaclw_i_rv_slot;
        let mut var_vfbaco_i: f64 = *var_vfbaco_i_slot;
        let mut var_vfbaco_i_rv: f64 = *var_vfbaco_i_rv_slot;
        let mut var_vfbacw_i: f64 = *var_vfbacw_i_slot;
        let mut var_vfbacw_i_rv: f64 = *var_vfbacw_i_rv_slot;
        let mut var_vfbbaco_i: f64 = *var_vfbbaco_i_slot;
        let mut var_vfbbaco_i_rv: f64 = *var_vfbbaco_i_rv_slot;
        let mut var_vfblbaco_i: f64 = *var_vfblbaco_i_slot;
        let mut var_vfblbaco_i_rv: f64 = *var_vfblbaco_i_rv_slot;

        let (assign3990_e3546,) = {
    if (var_guard83 == 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign3990_e3546;
        var_psceac2_i_rv = 0.0;

        let (assign4000_e3551, assign4000_e3551_d_n4, assign4000_e3551_d_n6, assign4000_e3551_d_n7, assign4000_e3551_d_n8, assign4000_e3551_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign4000_e3551;
        var_cfac_p_dn4 = assign4000_e3551_d_n4;
        var_cfac_p_dn6 = assign4000_e3551_d_n6;
        var_cfac_p_dn7 = assign4000_e3551_d_n7;
        var_cfac_p_dn8 = assign4000_e3551_d_n8;
        var_cfac_p_dn9 = assign4000_e3551_d_n9;
        var_cfac_p_rv = 0.0;

        let (assign4010_e3556, assign4010_e3556_d_n4, assign4010_e3556_d_n6, assign4010_e3556_d_n7, assign4010_e3556_d_n8, assign4010_e3556_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign4010_e3556;
        var_cfac1_t_dn4 = assign4010_e3556_d_n4;
        var_cfac1_t_dn6 = assign4010_e3556_d_n6;
        var_cfac1_t_dn7 = assign4010_e3556_d_n7;
        var_cfac1_t_dn8 = assign4010_e3556_d_n8;
        var_cfac1_t_dn9 = assign4010_e3556_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign4020_e3561, assign4020_e3561_d_n4, assign4020_e3561_d_n6, assign4020_e3561_d_n7, assign4020_e3561_d_n8, assign4020_e3561_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign4020_e3561;
        var_cfac2_t_dn4 = assign4020_e3561_d_n4;
        var_cfac2_t_dn6 = assign4020_e3561_d_n6;
        var_cfac2_t_dn7 = assign4020_e3561_d_n7;
        var_cfac2_t_dn8 = assign4020_e3561_d_n8;
        var_cfac2_t_dn9 = assign4020_e3561_d_n9;
        var_cfac2_t_rv = 0.0;

        let (assign4030_e3566, assign4030_e3566_d_n4, assign4030_e3566_d_n6, assign4030_e3566_d_n7, assign4030_e3566_d_n8, assign4030_e3566_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign4030_e3566;
        var_thesatac_p_dn4 = assign4030_e3566_d_n4;
        var_thesatac_p_dn6 = assign4030_e3566_d_n6;
        var_thesatac_p_dn7 = assign4030_e3566_d_n7;
        var_thesatac_p_dn8 = assign4030_e3566_d_n8;
        var_thesatac_p_dn9 = assign4030_e3566_d_n9;
        var_thesatac_p_rv = 0.0;

        let (assign4040_e3571, assign4040_e3571_d_n4, assign4040_e3571_d_n6, assign4040_e3571_d_n7, assign4040_e3571_d_n8, assign4040_e3571_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign4040_e3571;
        var_thesatac_t_dn4 = assign4040_e3571_d_n4;
        var_thesatac_t_dn6 = assign4040_e3571_d_n6;
        var_thesatac_t_dn7 = assign4040_e3571_d_n7;
        var_thesatac_t_dn8 = assign4040_e3571_d_n8;
        var_thesatac_t_dn9 = assign4040_e3571_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign4050_e3576,) = {
    if (var_guard83 == 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4050_e3576;
        var_axac_i_rv = 0.0;

        let (assign4060_e3581,) = {
    if (var_guard83 == 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign4060_e3581;
        var_alpac_i_rv = 0.0;

        let assign4070_e3584: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard98 = assign4070_e3584;
        var_guard98_rv = 0.0;

        let (assign4080_e3591,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p211,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4080_e3591;
        var_vfbaco_i_rv = 0.0;

        let assign4090_e3593: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4090_e3595: f64 = if assign4090_e3593 == 1.0 { 1.0 } else { 0.0 };
        var_guard99 = assign4090_e3595;
        var_guard99_rv = 0.0;

        let (assign4100_e3604,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard99 != 0.0)) {
        (p.p401,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4100_e3604;
        var_vfbaco_i_rv = 0.0;

        let (assign4110_e3611,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p212,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4110_e3611;
        var_vfbacl_i_rv = 0.0;

        let assign4120_e3613: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4120_e3615: f64 = if assign4120_e3613 == 1.0 { 1.0 } else { 0.0 };
        var_guard100 = assign4120_e3615;
        var_guard100_rv = 0.0;

        let (assign4130_e3624,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard100 != 0.0)) {
        (p.p402,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4130_e3624;
        var_vfbacl_i_rv = 0.0;

        let (assign4140_e3631,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p213,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4140_e3631;
        var_vfbaclexp_i_rv = 0.0;

        let assign4150_e3633: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4150_e3635: f64 = if assign4150_e3633 == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign4150_e3635;
        var_guard101_rv = 0.0;

        let (assign4160_e3644,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard101 != 0.0)) {
        (p.p403,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4160_e3644;
        var_vfbaclexp_i_rv = 0.0;

        let (assign4170_e3651,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p216,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4170_e3651;
        var_vfbacw_i_rv = 0.0;

        let assign4180_e3653: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4180_e3655: f64 = if assign4180_e3653 == 1.0 { 1.0 } else { 0.0 };
        var_guard102 = assign4180_e3655;
        var_guard102_rv = 0.0;

        let (assign4190_e3664,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard102 != 0.0)) {
        (p.p406,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4190_e3664;
        var_vfbacw_i_rv = 0.0;

        let (assign4200_e3671,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p217,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4200_e3671;
        var_vfbaclw_i_rv = 0.0;

        let assign4210_e3673: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4210_e3675: f64 = if assign4210_e3673 == 1.0 { 1.0 } else { 0.0 };
        var_guard103 = assign4210_e3675;
        var_guard103_rv = 0.0;

        let (assign4220_e3684,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard103 != 0.0)) {
        (p.p407,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4220_e3684;
        var_vfbaclw_i_rv = 0.0;

        let (assign4230_e3691,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p214,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4230_e3691;
        var_vfbacl2_i_rv = 0.0;

        let assign4240_e3693: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4240_e3695: f64 = if assign4240_e3693 == 1.0 { 1.0 } else { 0.0 };
        var_guard104 = assign4240_e3695;
        var_guard104_rv = 0.0;

        let (assign4250_e3704,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard104 != 0.0)) {
        (p.p404,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4250_e3704;
        var_vfbacl2_i_rv = 0.0;

        let (assign4260_e3711,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p215,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4260_e3711;
        var_vfbaclexp2_i_rv = 0.0;

        let assign4270_e3713: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4270_e3715: f64 = if assign4270_e3713 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign4270_e3715;
        var_guard105_rv = 0.0;

        let (assign4280_e3724,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p405,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4280_e3724;
        var_vfbaclexp2_i_rv = 0.0;

        let (assign4290_e3743, assign4290_e3743_d_n4, assign4290_e3743_d_n6, assign4290_e3743_d_n7, assign4290_e3743_d_n8, assign4290_e3743_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4290_e3732: f64 = (var_ile).powf(var_vfbaclexp_i);
        let assign4290_e3733: f64 = (var_vfbacl_i * assign4290_e3732);
        let assign4290_e3738: f64 = (var_ile).powf(var_vfbaclexp2_i);
        let assign4290_e3739: f64 = (var_vfbacl2_i * assign4290_e3738);
        let assign4290_e3740: f64 = (1.0 + assign4290_e3739);
        let assign4290_e3741: f64 = (assign4290_e3733 / assign4290_e3740);
        (assign4290_e3741, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign4290_e3743;
        var_temp_dn4 = assign4290_e3743_d_n4;
        var_temp_dn6 = assign4290_e3743_d_n6;
        var_temp_dn7 = assign4290_e3743_d_n7;
        var_temp_dn8 = assign4290_e3743_d_n8;
        var_temp_dn9 = assign4290_e3743_d_n9;
        var_temp_rv = 0.0;

        let (assign4300_e3760, assign4300_e3760_d_n4, assign4300_e3760_d_n6, assign4300_e3760_d_n7, assign4300_e3760_d_n8, assign4300_e3760_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4300_e3750: f64 = (var_vfbaco_i + var_temp);
        let assign4300_e3753: f64 = (var_vfbacw_i * var_iwe);
        let assign4300_e3754: f64 = (assign4300_e3750 + assign4300_e3753);
        let assign4300_e3757: f64 = (var_vfbaclw_i * var_iae);
        let assign4300_e3758: f64 = (assign4300_e3754 + assign4300_e3757);
        (assign4300_e3758, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign4300_e3760;
        var_vfbac1_t_dn4 = assign4300_e3760_d_n4;
        var_vfbac1_t_dn6 = assign4300_e3760_d_n6;
        var_vfbac1_t_dn7 = assign4300_e3760_d_n7;
        var_vfbac1_t_dn8 = assign4300_e3760_d_n8;
        var_vfbac1_t_dn9 = assign4300_e3760_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign4310_e3767,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p218,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4310_e3767;
        var_vfbbaco_i_rv = 0.0;

        let assign4320_e3769: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4320_e3771: f64 = if assign4320_e3769 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign4320_e3771;
        var_guard106_rv = 0.0;

        let (assign4330_e3780,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p408,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4330_e3780;
        var_vfbbaco_i_rv = 0.0;

        let (assign4340_e3787,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p219,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4340_e3787;
        var_vfblbaco_i_rv = 0.0;

        let assign4350_e3789: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4350_e3791: f64 = if assign4350_e3789 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign4350_e3791;
        var_guard107_rv = 0.0;

        let (assign4360_e3800,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p409,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4360_e3800;
        var_vfblbaco_i_rv = 0.0;

        let (assign4370_e3815, assign4370_e3815_d_n4, assign4370_e3815_d_n6, assign4370_e3815_d_n7, assign4370_e3815_d_n8, assign4370_e3815_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4370_e3808: f64 = (var_vfblbaco_i * var_tox2_i);
        let assign4370_e3810: f64 = (assign4370_e3808 / var_tox1_i);
        let assign4370_e3812: f64 = (assign4370_e3810 * var_temp);
        let assign4370_e3813: f64 = (var_vfbbaco_i + assign4370_e3812);
        (assign4370_e3813, (assign4370_e3810 * var_temp_dn4), (assign4370_e3810 * var_temp_dn6), (assign4370_e3810 * var_temp_dn7), (assign4370_e3810 * var_temp_dn8), (assign4370_e3810 * var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign4370_e3815;
        var_vfbac2_t_dn4 = assign4370_e3815_d_n4;
        var_vfbac2_t_dn6 = assign4370_e3815_d_n6;
        var_vfbac2_t_dn7 = assign4370_e3815_d_n7;
        var_vfbac2_t_dn8 = assign4370_e3815_d_n8;
        var_vfbac2_t_dn9 = assign4370_e3815_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign4380_e3822,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p228,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4380_e3822;
        var_psceacl_i_rv = 0.0;

        let assign4390_e3824: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4390_e3826: f64 = if assign4390_e3824 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign4390_e3826;
        var_guard108_rv = 0.0;

        let (assign4400_e3835,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p410,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4400_e3835;
        var_psceacl_i_rv = 0.0;

        let (assign4410_e3842,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p229,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4410_e3842;
        var_psceaclexp_i_rv = 0.0;

        let assign4420_e3844: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4420_e3846: f64 = if assign4420_e3844 == 1.0 { 1.0 } else { 0.0 };
        var_guard109 = assign4420_e3846;
        var_guard109_rv = 0.0;

        let (assign4430_e3855,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard109 != 0.0)) {
        (p.p411,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4430_e3855;
        var_psceaclexp_i_rv = 0.0;

        let (assign4440_e3862,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p230,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4440_e3862;
        var_psceacw_i_rv = 0.0;

        let assign4450_e3864: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4450_e3866: f64 = if assign4450_e3864 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign4450_e3866;
        var_guard110_rv = 0.0;

        let (assign4460_e3875,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p412,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4460_e3875;
        var_psceacw_i_rv = 0.0;

        let (assign4470_e3894,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4470_e3882: f64 = (var_psceacl_i * 2.0);
        let assign4470_e3885: f64 = (var_lambda_le).powf(var_psceaclexp_i);
        let assign4470_e3886: f64 = (assign4470_e3882 * assign4470_e3885);
        let assign4470_e3890: f64 = (var_psceacw_i * var_iwe);
        let assign4470_e3891: f64 = (1.0 + assign4470_e3890);
        let assign4470_e3892: f64 = (assign4470_e3886 * assign4470_e3891);
        (assign4470_e3892,)
    } else {
        (var_psceac_p,)
    }
};
        var_psceac_p = assign4470_e3894;
        var_psceac_p_rv = 0.0;

        let (assign4480_e3905,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4480_e3901: f64 = (var_psceac_p).max(0.0);
        let assign4480_e3903: f64 = (assign4480_e3901).min(5.0);
        (assign4480_e3903,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign4480_e3905;
        var_psceac1_i_rv = 0.0;

        let (assign4490_e3918,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4490_e3912: f64 = (p.p231 * var_psceac1_i);
        let assign4490_e3914: f64 = (assign4490_e3912 * var_tox2_i);
        let assign4490_e3916: f64 = (assign4490_e3914 / var_tox1_i);
        (assign4490_e3916,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign4490_e3918;
        var_psceac2_i_rv = 0.0;

        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac1_t_rv_slot = var_cfac1_t_rv;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac2_t_rv_slot = var_cfac2_t_rv;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfac_p_rv_slot = var_cfac_p_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac1_i_rv_slot = var_psceac1_i_rv;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_psceac2_i_rv_slot = var_psceac2_i_rv;
        *var_psceac_p_slot = var_psceac_p;
        *var_psceac_p_rv_slot = var_psceac_p_rv;
        *var_psceacl_i_slot = var_psceacl_i;
        *var_psceacl_i_rv_slot = var_psceacl_i_rv;
        *var_psceaclexp_i_slot = var_psceaclexp_i;
        *var_psceaclexp_i_rv_slot = var_psceaclexp_i_rv;
        *var_psceacw_i_slot = var_psceacw_i;
        *var_psceacw_i_rv_slot = var_psceacw_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac1_t_rv_slot = var_vfbac1_t_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
        *var_vfbacl2_i_slot = var_vfbacl2_i;
        *var_vfbacl2_i_rv_slot = var_vfbacl2_i_rv;
        *var_vfbacl_i_slot = var_vfbacl_i;
        *var_vfbacl_i_rv_slot = var_vfbacl_i_rv;
        *var_vfbaclexp2_i_slot = var_vfbaclexp2_i;
        *var_vfbaclexp2_i_rv_slot = var_vfbaclexp2_i_rv;
        *var_vfbaclexp_i_slot = var_vfbaclexp_i;
        *var_vfbaclexp_i_rv_slot = var_vfbaclexp_i_rv;
        *var_vfbaclw_i_slot = var_vfbaclw_i;
        *var_vfbaclw_i_rv_slot = var_vfbaclw_i_rv;
        *var_vfbaco_i_slot = var_vfbaco_i;
        *var_vfbaco_i_rv_slot = var_vfbaco_i_rv;
        *var_vfbacw_i_slot = var_vfbacw_i;
        *var_vfbacw_i_rv_slot = var_vfbacw_i_rv;
        *var_vfbbaco_i_slot = var_vfbbaco_i;
        *var_vfbbaco_i_rv_slot = var_vfbbaco_i_rv;
        *var_vfblbaco_i_slot = var_vfblbaco_i;
        *var_vfblbaco_i_rv_slot = var_vfblbaco_i_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_ge: f64,
        var_ge_dn4: f64,
        var_ge_dn6: f64,
        var_ge_dn7: f64,
        var_ge_dn8: f64,
        var_ge_dn9: f64,
        var_guard83: f64,
        var_guard98: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_alpacl1_i_slot: &mut f64,
        var_alpacl1_i_rv_slot: &mut f64,
        var_alpaclexp_i_slot: &mut f64,
        var_alpaclexp_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axacl2_i_slot: &mut f64,
        var_axacl2_i_rv_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axacl_i_rv_slot: &mut f64,
        var_axaclexp2_i_slot: &mut f64,
        var_axaclexp2_i_rv_slot: &mut f64,
        var_axaclexp_i_slot: &mut f64,
        var_axaclexp_i_rv_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_axaco_i_rv_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac1_t_rv_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac2_t_rv_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfac_p_rv_slot: &mut f64,
        var_cfacl_i_slot: &mut f64,
        var_cfacl_i_rv_slot: &mut f64,
        var_cfaclexp_i_slot: &mut f64,
        var_cfaclexp_i_rv_slot: &mut f64,
        var_cfacw_i_slot: &mut f64,
        var_cfacw_i_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesatacl_i_rv_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclexp_i_rv_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataclw_i_rv_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesataco_i_rv_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_thesatacw_i_rv_slot: &mut f64,
    ) {
        let mut var_alpacl1_i: f64 = *var_alpacl1_i_slot;
        let mut var_alpacl1_i_rv: f64 = *var_alpacl1_i_rv_slot;
        let mut var_alpaclexp_i: f64 = *var_alpaclexp_i_slot;
        let mut var_alpaclexp_i_rv: f64 = *var_alpaclexp_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axacl2_i: f64 = *var_axacl2_i_slot;
        let mut var_axacl2_i_rv: f64 = *var_axacl2_i_rv_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axacl_i_rv: f64 = *var_axacl_i_rv_slot;
        let mut var_axaclexp2_i: f64 = *var_axaclexp2_i_slot;
        let mut var_axaclexp2_i_rv: f64 = *var_axaclexp2_i_rv_slot;
        let mut var_axaclexp_i: f64 = *var_axaclexp_i_slot;
        let mut var_axaclexp_i_rv: f64 = *var_axaclexp_i_rv_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_axaco_i_rv: f64 = *var_axaco_i_rv_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac1_t_rv: f64 = *var_cfac1_t_rv_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac2_t_rv: f64 = *var_cfac2_t_rv_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfac_p_rv: f64 = *var_cfac_p_rv_slot;
        let mut var_cfacl_i: f64 = *var_cfacl_i_slot;
        let mut var_cfacl_i_rv: f64 = *var_cfacl_i_rv_slot;
        let mut var_cfaclexp_i: f64 = *var_cfaclexp_i_slot;
        let mut var_cfaclexp_i_rv: f64 = *var_cfaclexp_i_rv_slot;
        let mut var_cfacw_i: f64 = *var_cfacw_i_slot;
        let mut var_cfacw_i_rv: f64 = *var_cfacw_i_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesatacl_i_rv: f64 = *var_thesatacl_i_rv_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclexp_i_rv: f64 = *var_thesataclexp_i_rv_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataclw_i_rv: f64 = *var_thesataclw_i_rv_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesataco_i_rv: f64 = *var_thesataco_i_rv_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_thesatacw_i_rv: f64 = *var_thesatacw_i_rv_slot;

        let (assign4500_e3925,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p235,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4500_e3925;
        var_cfacl_i_rv = 0.0;

        let assign4510_e3927: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4510_e3929: f64 = if assign4510_e3927 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign4510_e3929;
        var_guard111_rv = 0.0;

        let (assign4520_e3938,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p413,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4520_e3938;
        var_cfacl_i_rv = 0.0;

        let (assign4530_e3945,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p236,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4530_e3945;
        var_cfaclexp_i_rv = 0.0;

        let assign4540_e3947: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4540_e3949: f64 = if assign4540_e3947 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign4540_e3949;
        var_guard112_rv = 0.0;

        let (assign4550_e3958,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p414,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4550_e3958;
        var_cfaclexp_i_rv = 0.0;

        let (assign4560_e3965,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p237,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4560_e3965;
        var_cfacw_i_rv = 0.0;

        let assign4570_e3967: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4570_e3969: f64 = if assign4570_e3967 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign4570_e3969;
        var_guard113_rv = 0.0;

        let (assign4580_e3978,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p415,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4580_e3978;
        var_cfacw_i_rv = 0.0;

        let (assign4590_e3993, assign4590_e3993_d_n4, assign4590_e3993_d_n6, assign4590_e3993_d_n7, assign4590_e3993_d_n8, assign4590_e3993_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4590_e3985: f64 = (var_lambda_le).powf(var_cfaclexp_i);
        let assign4590_e3989: f64 = (var_cfacw_i * var_iwe);
        let assign4590_e3990: f64 = (1.0 + assign4590_e3989);
        let assign4590_e3991: f64 = (assign4590_e3985 * assign4590_e3990);
        (assign4590_e3991, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign4590_e3993;
        var_temp_dn4 = assign4590_e3993_d_n4;
        var_temp_dn6 = assign4590_e3993_d_n6;
        var_temp_dn7 = assign4590_e3993_d_n7;
        var_temp_dn8 = assign4590_e3993_d_n8;
        var_temp_dn9 = assign4590_e3993_d_n9;
        var_temp_rv = 0.0;

        let (assign4600_e4002, assign4600_e4002_d_n4, assign4600_e4002_d_n6, assign4600_e4002_d_n7, assign4600_e4002_d_n8, assign4600_e4002_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4600_e4000: f64 = (var_cfacl_i * var_temp);
        (assign4600_e4000, (var_cfacl_i * var_temp_dn4), (var_cfacl_i * var_temp_dn6), (var_cfacl_i * var_temp_dn7), (var_cfacl_i * var_temp_dn8), (var_cfacl_i * var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign4600_e4002;
        var_cfac_p_dn4 = assign4600_e4002_d_n4;
        var_cfac_p_dn6 = assign4600_e4002_d_n6;
        var_cfac_p_dn7 = assign4600_e4002_d_n7;
        var_cfac_p_dn8 = assign4600_e4002_d_n8;
        var_cfac_p_dn9 = assign4600_e4002_d_n9;
        var_cfac_p_rv = 0.0;

        let (assign4610_e4011, assign4610_e4011_d_n4, assign4610_e4011_d_n6, assign4610_e4011_d_n7, assign4610_e4011_d_n8, assign4610_e4011_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4610_e4009: f64 = (var_cfac_p).max(0.0);
        (assign4610_e4009, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign4610_e4011;
        var_cfac1_t_dn4 = assign4610_e4011_d_n4;
        var_cfac1_t_dn6 = assign4610_e4011_d_n6;
        var_cfac1_t_dn7 = assign4610_e4011_d_n7;
        var_cfac1_t_dn8 = assign4610_e4011_d_n8;
        var_cfac1_t_dn9 = assign4610_e4011_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign4620_e4024, assign4620_e4024_d_n4, assign4620_e4024_d_n6, assign4620_e4024_d_n7, assign4620_e4024_d_n8, assign4620_e4024_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4620_e4018: f64 = (p.p238 * var_cfac1_t);
        let assign4620_e4020: f64 = (assign4620_e4018 * var_tox2_i);
        let assign4620_e4022: f64 = (assign4620_e4020 / var_tox1_i);
        (assign4620_e4022, (((p.p238 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign4620_e4024;
        var_cfac2_t_dn4 = assign4620_e4024_d_n4;
        var_cfac2_t_dn6 = assign4620_e4024_d_n6;
        var_cfac2_t_dn7 = assign4620_e4024_d_n7;
        var_cfac2_t_dn8 = assign4620_e4024_d_n8;
        var_cfac2_t_dn9 = assign4620_e4024_d_n9;
        var_cfac2_t_rv = 0.0;

        let (assign4630_e4031,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p293,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4630_e4031;
        var_thesataco_i_rv = 0.0;

        let assign4640_e4033: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4640_e4035: f64 = if assign4640_e4033 == 1.0 { 1.0 } else { 0.0 };
        var_guard114 = assign4640_e4035;
        var_guard114_rv = 0.0;

        let (assign4650_e4044,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard114 != 0.0)) {
        (p.p416,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4650_e4044;
        var_thesataco_i_rv = 0.0;

        let (assign4660_e4051,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p294,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4660_e4051;
        var_thesatacl_i_rv = 0.0;

        let assign4670_e4053: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4670_e4055: f64 = if assign4670_e4053 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign4670_e4055;
        var_guard115_rv = 0.0;

        let (assign4680_e4064,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard115 != 0.0)) {
        (p.p417,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4680_e4064;
        var_thesatacl_i_rv = 0.0;

        let (assign4690_e4071,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p295,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4690_e4071;
        var_thesataclexp_i_rv = 0.0;

        let assign4700_e4073: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4700_e4075: f64 = if assign4700_e4073 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign4700_e4075;
        var_guard116_rv = 0.0;

        let (assign4710_e4084,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard116 != 0.0)) {
        (p.p418,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4710_e4084;
        var_thesataclexp_i_rv = 0.0;

        let (assign4720_e4091,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p296,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4720_e4091;
        var_thesatacw_i_rv = 0.0;

        let assign4730_e4093: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4730_e4095: f64 = if assign4730_e4093 == 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign4730_e4095;
        var_guard117_rv = 0.0;

        let (assign4740_e4104,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard117 != 0.0)) {
        (p.p419,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4740_e4104;
        var_thesatacw_i_rv = 0.0;

        let (assign4750_e4111,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p297,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4750_e4111;
        var_thesataclw_i_rv = 0.0;

        let assign4760_e4113: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4760_e4115: f64 = if assign4760_e4113 == 1.0 { 1.0 } else { 0.0 };
        var_guard118 = assign4760_e4115;
        var_guard118_rv = 0.0;

        let (assign4770_e4124,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard118 != 0.0)) {
        (p.p420,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4770_e4124;
        var_thesataclw_i_rv = 0.0;

        let (assign4780_e4151, assign4780_e4151_d_n4, assign4780_e4151_d_n6, assign4780_e4151_d_n7, assign4780_e4151_d_n8, assign4780_e4151_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4780_e4134: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign4780_e4135: f64 = (var_thesatacl_i * assign4780_e4134);
        let assign4780_e4136: f64 = (var_thesataco_i + assign4780_e4135);
        let assign4780_e4137: f64 = (var_ge * assign4780_e4136);
        let assign4780_e4141: f64 = (var_thesatacw_i * var_iwe);
        let assign4780_e4142: f64 = (1.0 + assign4780_e4141);
        let assign4780_e4143: f64 = (assign4780_e4137 * assign4780_e4142);
        let assign4780_e4147: f64 = (var_thesataclw_i * var_iae);
        let assign4780_e4148: f64 = (1.0 + assign4780_e4147);
        let assign4780_e4149: f64 = (assign4780_e4143 * assign4780_e4148);
        (assign4780_e4149, (((var_ge_dn4 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn6 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn7 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn8 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn9 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign4780_e4151;
        var_thesatac_p_dn4 = assign4780_e4151_d_n4;
        var_thesatac_p_dn6 = assign4780_e4151_d_n6;
        var_thesatac_p_dn7 = assign4780_e4151_d_n7;
        var_thesatac_p_dn8 = assign4780_e4151_d_n8;
        var_thesatac_p_dn9 = assign4780_e4151_d_n9;
        var_thesatac_p_rv = 0.0;

        let (assign4790_e4160, assign4790_e4160_d_n4, assign4790_e4160_d_n6, assign4790_e4160_d_n7, assign4790_e4160_d_n8, assign4790_e4160_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4790_e4158: f64 = (var_thesatac_p).max(0.0);
        (assign4790_e4158, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign4790_e4160;
        var_thesatac_t_dn4 = assign4790_e4160_d_n4;
        var_thesatac_t_dn6 = assign4790_e4160_d_n6;
        var_thesatac_t_dn7 = assign4790_e4160_d_n7;
        var_thesatac_t_dn8 = assign4790_e4160_d_n8;
        var_thesatac_t_dn9 = assign4790_e4160_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign4800_e4167,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p304,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4800_e4167;
        var_axaco_i_rv = 0.0;

        let assign4810_e4169: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4810_e4171: f64 = if assign4810_e4169 == 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign4810_e4171;
        var_guard119_rv = 0.0;

        let (assign4820_e4180,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard119 != 0.0)) {
        (p.p421,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4820_e4180;
        var_axaco_i_rv = 0.0;

        let (assign4830_e4187,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p305,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4830_e4187;
        var_axacl_i_rv = 0.0;

        let assign4840_e4189: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4840_e4191: f64 = if assign4840_e4189 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign4840_e4191;
        var_guard120_rv = 0.0;

        let (assign4850_e4200,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard120 != 0.0)) {
        (p.p422,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4850_e4200;
        var_axacl_i_rv = 0.0;

        let (assign4860_e4207,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p306,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4860_e4207;
        var_axaclexp_i_rv = 0.0;

        let assign4870_e4209: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4870_e4211: f64 = if assign4870_e4209 == 1.0 { 1.0 } else { 0.0 };
        var_guard121 = assign4870_e4211;
        var_guard121_rv = 0.0;

        let (assign4880_e4220,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard121 != 0.0)) {
        (p.p423,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4880_e4220;
        var_axaclexp_i_rv = 0.0;

        let (assign4890_e4227,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p307,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4890_e4227;
        var_axacl2_i_rv = 0.0;

        let assign4900_e4229: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign4900_e4231: f64 = if assign4900_e4229 == 1.0 { 1.0 } else { 0.0 };
        var_guard122 = assign4900_e4231;
        var_guard122_rv = 0.0;

        let (assign4910_e4240,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard122 != 0.0)) {
        (p.p424,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4910_e4240;
        var_axacl2_i_rv = 0.0;

        let (assign4920_e4247,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p308,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4920_e4247;
        var_axaclexp2_i_rv = 0.0;

        let assign4930_e4249: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign4930_e4251: f64 = if assign4930_e4249 == 1.0 { 1.0 } else { 0.0 };
        var_guard123 = assign4930_e4251;
        var_guard123_rv = 0.0;

        let (assign4940_e4260,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard123 != 0.0)) {
        (p.p425,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4940_e4260;
        var_axaclexp2_i_rv = 0.0;

        let (assign4950_e4283,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4950_e4270: f64 = (var_ile).powf(var_axaclexp_i);
        let assign4950_e4271: f64 = (var_axacl_i * assign4950_e4270);
        let assign4950_e4276: f64 = (var_ile).powf(var_axaclexp2_i);
        let assign4950_e4277: f64 = (var_axacl2_i * assign4950_e4276);
        let assign4950_e4278: f64 = (1.0 + assign4950_e4277);
        let assign4950_e4279: f64 = (assign4950_e4271 / assign4950_e4278);
        let assign4950_e4280: f64 = (1.0 + assign4950_e4279);
        let assign4950_e4281: f64 = (var_axaco_i / assign4950_e4280);
        (assign4950_e4281,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4950_e4283;
        var_axac_p_rv = 0.0;

        let (assign4960_e4294,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4960_e4290: f64 = (var_axac_p).max(1.0);
        let assign4960_e4292: f64 = (assign4960_e4290).min(16.0);
        (assign4960_e4292,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4960_e4294;
        var_axac_i_rv = 0.0;

        let (assign4970_e4301,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p309,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4970_e4301;
        var_alpacl1_i_rv = 0.0;

        let assign4980_e4303: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign4980_e4305: f64 = if assign4980_e4303 == 1.0 { 1.0 } else { 0.0 };
        var_guard124 = assign4980_e4305;
        var_guard124_rv = 0.0;

        let (assign4990_e4314,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard124 != 0.0)) {
        (p.p426,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4990_e4314;
        var_alpacl1_i_rv = 0.0;

        let (assign5000_e4321,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p310,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign5000_e4321;
        var_alpaclexp_i_rv = 0.0;

        let assign5010_e4323: f64 = if param_given[427] { 1.0 } else { 0.0 };
        let assign5010_e4325: f64 = if assign5010_e4323 == 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign5010_e4325;
        var_guard125_rv = 0.0;

        *var_alpacl1_i_slot = var_alpacl1_i;
        *var_alpacl1_i_rv_slot = var_alpacl1_i_rv;
        *var_alpaclexp_i_slot = var_alpaclexp_i;
        *var_alpaclexp_i_rv_slot = var_alpaclexp_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axacl2_i_slot = var_axacl2_i;
        *var_axacl2_i_rv_slot = var_axacl2_i_rv;
        *var_axacl_i_slot = var_axacl_i;
        *var_axacl_i_rv_slot = var_axacl_i_rv;
        *var_axaclexp2_i_slot = var_axaclexp2_i;
        *var_axaclexp2_i_rv_slot = var_axaclexp2_i_rv;
        *var_axaclexp_i_slot = var_axaclexp_i;
        *var_axaclexp_i_rv_slot = var_axaclexp_i_rv;
        *var_axaco_i_slot = var_axaco_i;
        *var_axaco_i_rv_slot = var_axaco_i_rv;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac1_t_rv_slot = var_cfac1_t_rv;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac2_t_rv_slot = var_cfac2_t_rv;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfac_p_rv_slot = var_cfac_p_rv;
        *var_cfacl_i_slot = var_cfacl_i;
        *var_cfacl_i_rv_slot = var_cfacl_i_rv;
        *var_cfaclexp_i_slot = var_cfaclexp_i;
        *var_cfaclexp_i_rv_slot = var_cfaclexp_i_rv;
        *var_cfacw_i_slot = var_cfacw_i;
        *var_cfacw_i_rv_slot = var_cfacw_i_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard123_slot = var_guard123;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesatacl_i_rv_slot = var_thesatacl_i_rv;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclexp_i_rv_slot = var_thesataclexp_i_rv;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataclw_i_rv_slot = var_thesataclw_i_rv;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesataco_i_rv_slot = var_thesataco_i_rv;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_thesatacw_i_rv_slot = var_thesatacw_i_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alpacl1_i: f64,
        var_epsch: f64,
        var_guard125: f64,
        var_guard83: f64,
        var_guard98: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_lphy: f64,
        var_lphy_dn4: f64,
        var_lphy_dn6: f64,
        var_lphy_dn7: f64,
        var_lphy_dn8: f64,
        var_lphy_dn9: f64,
        var_tox1_i: f64,
        var_tsi_i: f64,
        var_we: f64,
        var_wecv: f64,
        var_wen: f64,
        var_wphy: f64,
        var_wphy_dn4: f64,
        var_wphy_dn6: f64,
        var_wphy_dn7: f64,
        var_wphy_dn8: f64,
        var_wphy_dn9: f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_alpacl2_i_slot: &mut f64,
        var_alpacl2_i_rv_slot: &mut f64,
        var_alpaclexp2_i_slot: &mut f64,
        var_alpaclexp2_i_rv_slot: &mut f64,
        var_alpaclexp_i_slot: &mut f64,
        var_alpaclexp_i_rv_slot: &mut f64,
        var_alpacw_i_slot: &mut f64,
        var_alpacw_i_rv_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfr_i_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_dn4_slot: &mut f64,
        var_cfr_p_dn6_slot: &mut f64,
        var_cfr_p_dn7_slot: &mut f64,
        var_cfr_p_dn8_slot: &mut f64,
        var_cfr_p_dn9_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_dn4_slot: &mut f64,
        var_cfrd_p_dn6_slot: &mut f64,
        var_cfrd_p_dn7_slot: &mut f64,
        var_cfrd_p_dn8_slot: &mut f64,
        var_cfrd_p_dn9_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_cov_i_rv_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covd_i_rv_slot: &mut f64,
        var_covdl_i_slot: &mut f64,
        var_covdl_i_rv_slot: &mut f64,
        var_covdlb_i_slot: &mut f64,
        var_covdlb_i_rv_slot: &mut f64,
        var_csd_i_slot: &mut f64,
        var_csd_i_rv_slot: &mut f64,
        var_csdbp_i_slot: &mut f64,
        var_csdbp_i_rv_slot: &mut f64,
        var_dvfbov_i_slot: &mut f64,
        var_dvfbov_i_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_rth_p_dn4_slot: &mut f64,
        var_rth_p_dn6_slot: &mut f64,
        var_rth_p_dn7_slot: &mut f64,
        var_rth_p_dn8_slot: &mut f64,
        var_rth_p_dn9_slot: &mut f64,
        var_rth_p_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp3_rv_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp4_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_alpacl2_i: f64 = *var_alpacl2_i_slot;
        let mut var_alpacl2_i_rv: f64 = *var_alpacl2_i_rv_slot;
        let mut var_alpaclexp2_i: f64 = *var_alpaclexp2_i_slot;
        let mut var_alpaclexp2_i_rv: f64 = *var_alpaclexp2_i_rv_slot;
        let mut var_alpaclexp_i: f64 = *var_alpaclexp_i_slot;
        let mut var_alpaclexp_i_rv: f64 = *var_alpaclexp_i_rv_slot;
        let mut var_alpacw_i: f64 = *var_alpacw_i_slot;
        let mut var_alpacw_i_rv: f64 = *var_alpacw_i_rv_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfr_i_rv: f64 = *var_cfr_i_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_dn4: f64 = *var_cfr_p_dn4_slot;
        let mut var_cfr_p_dn6: f64 = *var_cfr_p_dn6_slot;
        let mut var_cfr_p_dn7: f64 = *var_cfr_p_dn7_slot;
        let mut var_cfr_p_dn8: f64 = *var_cfr_p_dn8_slot;
        let mut var_cfr_p_dn9: f64 = *var_cfr_p_dn9_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_dn4: f64 = *var_cfrd_p_dn4_slot;
        let mut var_cfrd_p_dn6: f64 = *var_cfrd_p_dn6_slot;
        let mut var_cfrd_p_dn7: f64 = *var_cfrd_p_dn7_slot;
        let mut var_cfrd_p_dn8: f64 = *var_cfrd_p_dn8_slot;
        let mut var_cfrd_p_dn9: f64 = *var_cfrd_p_dn9_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_cov_i_rv: f64 = *var_cov_i_rv_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covd_i_rv: f64 = *var_covd_i_rv_slot;
        let mut var_covdl_i: f64 = *var_covdl_i_slot;
        let mut var_covdl_i_rv: f64 = *var_covdl_i_rv_slot;
        let mut var_covdlb_i: f64 = *var_covdlb_i_slot;
        let mut var_covdlb_i_rv: f64 = *var_covdlb_i_rv_slot;
        let mut var_csd_i: f64 = *var_csd_i_slot;
        let mut var_csd_i_rv: f64 = *var_csd_i_rv_slot;
        let mut var_csdbp_i: f64 = *var_csdbp_i_slot;
        let mut var_csdbp_i_rv: f64 = *var_csdbp_i_rv_slot;
        let mut var_dvfbov_i: f64 = *var_dvfbov_i_slot;
        let mut var_dvfbov_i_rv: f64 = *var_dvfbov_i_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_rth_p_dn4: f64 = *var_rth_p_dn4_slot;
        let mut var_rth_p_dn6: f64 = *var_rth_p_dn6_slot;
        let mut var_rth_p_dn7: f64 = *var_rth_p_dn7_slot;
        let mut var_rth_p_dn8: f64 = *var_rth_p_dn8_slot;
        let mut var_rth_p_dn9: f64 = *var_rth_p_dn9_slot;
        let mut var_rth_p_rv: f64 = *var_rth_p_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp3_rv: f64 = *var_temp3_rv_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp4_rv: f64 = *var_temp4_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;

        let (assign5020_e4334,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard125 != 0.0)) {
        (p.p427,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign5020_e4334;
        var_alpaclexp_i_rv = 0.0;

        let (assign5030_e4341,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p311,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign5030_e4341;
        var_alpacl2_i_rv = 0.0;

        let assign5040_e4343: f64 = if param_given[428] { 1.0 } else { 0.0 };
        let assign5040_e4345: f64 = if assign5040_e4343 == 1.0 { 1.0 } else { 0.0 };
        var_guard126 = assign5040_e4345;
        var_guard126_rv = 0.0;

        let (assign5050_e4354,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard126 != 0.0)) {
        (p.p428,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign5050_e4354;
        var_alpacl2_i_rv = 0.0;

        let (assign5060_e4361,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p312,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5060_e4361;
        var_alpaclexp2_i_rv = 0.0;

        let assign5070_e4363: f64 = if param_given[429] { 1.0 } else { 0.0 };
        let assign5070_e4365: f64 = if assign5070_e4363 == 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign5070_e4365;
        var_guard127_rv = 0.0;

        let (assign5080_e4374,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard127 != 0.0)) {
        (p.p429,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5080_e4374;
        var_alpaclexp2_i_rv = 0.0;

        let (assign5090_e4381,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p313,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5090_e4381;
        var_alpacw_i_rv = 0.0;

        let assign5100_e4383: f64 = if param_given[430] { 1.0 } else { 0.0 };
        let assign5100_e4385: f64 = if assign5100_e4383 == 1.0 { 1.0 } else { 0.0 };
        var_guard128 = assign5100_e4385;
        var_guard128_rv = 0.0;

        let (assign5110_e4394,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard128 != 0.0)) {
        (p.p430,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5110_e4394;
        var_alpacw_i_rv = 0.0;

        let (assign5120_e4419,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5120_e4402: f64 = (var_ile).powf(var_alpaclexp_i);
        let assign5120_e4403: f64 = (var_alpacl1_i * assign5120_e4402);
        let assign5120_e4407: f64 = (var_alpacw_i * var_iwe);
        let assign5120_e4408: f64 = (1.0 + assign5120_e4407);
        let assign5120_e4409: f64 = (assign5120_e4403 * assign5120_e4408);
        let assign5120_e4414: f64 = (var_ile).powf(var_alpaclexp2_i);
        let assign5120_e4415: f64 = (var_alpacl2_i * assign5120_e4414);
        let assign5120_e4416: f64 = (1.0 + assign5120_e4415);
        let assign5120_e4417: f64 = (assign5120_e4409 / assign5120_e4416);
        (assign5120_e4417,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign5120_e4419;
        var_alpac_p_rv = 0.0;

        let (assign5130_e4428,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5130_e4426: f64 = (var_alpac_p).max(0.0);
        (assign5130_e4426,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign5130_e4428;
        var_alpac_i_rv = 0.0;

        let (assign5140_e4437, assign5140_e4437_d_n4, assign5140_e4437_d_n6, assign5140_e4437_d_n7, assign5140_e4437_d_n8, assign5140_e4437_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5140_e4433: f64 = (3.45313e-11 / var_tox1_i);
        let assign5140_e4435: f64 = (assign5140_e4433 * var_wecv);
        (assign5140_e4435, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5140_e4437;
        var_temp_dn4 = assign5140_e4437_d_n4;
        var_temp_dn6 = assign5140_e4437_d_n6;
        var_temp_dn7 = assign5140_e4437_d_n7;
        var_temp_dn8 = assign5140_e4437_d_n8;
        var_temp_dn9 = assign5140_e4437_d_n9;
        var_temp_rv = 0.0;

        let (assign5150_e4444, assign5150_e4444_d_n4, assign5150_e4444_d_n6, assign5150_e4444_d_n7, assign5150_e4444_d_n8, assign5150_e4444_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5150_e4442: f64 = (var_temp * p.p431);
        (assign5150_e4442, (var_temp_dn4 * p.p431), (var_temp_dn6 * p.p431), (var_temp_dn7 * p.p431), (var_temp_dn8 * p.p431), (var_temp_dn9 * p.p431),)
    } else {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    }
};
        var_cov_i = assign5150_e4444;
        var_cov_i_dn4 = assign5150_e4444_d_n4;
        var_cov_i_dn6 = assign5150_e4444_d_n6;
        var_cov_i_dn7 = assign5150_e4444_d_n7;
        var_cov_i_dn8 = assign5150_e4444_d_n8;
        var_cov_i_dn9 = assign5150_e4444_d_n9;
        var_cov_i_rv = 0.0;

        let (assign5160_e4451, assign5160_e4451_d_n4, assign5160_e4451_d_n6, assign5160_e4451_d_n7, assign5160_e4451_d_n8, assign5160_e4451_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5160_e4449: f64 = (var_temp * p.p432);
        (assign5160_e4449, (var_temp_dn4 * p.p432), (var_temp_dn6 * p.p432), (var_temp_dn7 * p.p432), (var_temp_dn8 * p.p432), (var_temp_dn9 * p.p432),)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign5160_e4451;
        var_covd_i_dn4 = assign5160_e4451_d_n4;
        var_covd_i_dn6 = assign5160_e4451_d_n6;
        var_covd_i_dn7 = assign5160_e4451_d_n7;
        var_covd_i_dn8 = assign5160_e4451_d_n8;
        var_covd_i_dn9 = assign5160_e4451_d_n9;
        var_covd_i_rv = 0.0;

        let (assign5170_e4466,) = {
    if (var_guard83 == 0.0) {
        let assign5170_e4458: f64 = (p.p434 * var_wen);
        let assign5170_e4460: f64 = (assign5170_e4458 / var_wecv);
        let assign5170_e4461: f64 = (1.0 + assign5170_e4460);
        let assign5170_e4463: f64 = (assign5170_e4461).max(0.001);
        let assign5170_e4464: f64 = (p.p433 / assign5170_e4463);
        (assign5170_e4464,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign5170_e4466;
        var_covdl_i_rv = 0.0;

        let (assign5180_e4471,) = {
    if (var_guard83 == 0.0) {
        (p.p435,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign5180_e4471;
        var_covdlb_i_rv = 0.0;

        let (assign5190_e4476,) = {
    if (var_guard83 == 0.0) {
        (p.p436,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign5190_e4476;
        var_dvfbov_i_rv = 0.0;

        let (assign5200_e4485, assign5200_e4485_d_n4, assign5200_e4485_d_n6, assign5200_e4485_d_n7, assign5200_e4485_d_n8, assign5200_e4485_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5200_e4482: f64 = (p.p439 * var_wphy);
        let assign5200_e4483: f64 = (p.p437 + assign5200_e4482);
        (assign5200_e4483, (p.p439 * var_wphy_dn4), (p.p439 * var_wphy_dn6), (p.p439 * var_wphy_dn7), (p.p439 * var_wphy_dn8), (p.p439 * var_wphy_dn9),)
    } else {
        (var_cfr_p, var_cfr_p_dn4, var_cfr_p_dn6, var_cfr_p_dn7, var_cfr_p_dn8, var_cfr_p_dn9,)
    }
};
        var_cfr_p = assign5200_e4485;
        var_cfr_p_dn4 = assign5200_e4485_d_n4;
        var_cfr_p_dn6 = assign5200_e4485_d_n6;
        var_cfr_p_dn7 = assign5200_e4485_d_n7;
        var_cfr_p_dn8 = assign5200_e4485_d_n8;
        var_cfr_p_dn9 = assign5200_e4485_d_n9;
        var_cfr_p_rv = 0.0;

        let (assign5210_e4492, assign5210_e4492_d_n4, assign5210_e4492_d_n6, assign5210_e4492_d_n7, assign5210_e4492_d_n8, assign5210_e4492_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5210_e4490: f64 = (var_cfr_p).max(0.0);
        (assign5210_e4490, if var_cfr_p >= 0.0 { var_cfr_p_dn4 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn6 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn7 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn8 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn9 } else { 0.0 },)
    } else {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    }
};
        var_cfr_i = assign5210_e4492;
        var_cfr_i_dn4 = assign5210_e4492_d_n4;
        var_cfr_i_dn6 = assign5210_e4492_d_n6;
        var_cfr_i_dn7 = assign5210_e4492_d_n7;
        var_cfr_i_dn8 = assign5210_e4492_d_n8;
        var_cfr_i_dn9 = assign5210_e4492_d_n9;
        var_cfr_i_rv = 0.0;

        let (assign5220_e4501, assign5220_e4501_d_n4, assign5220_e4501_d_n6, assign5220_e4501_d_n7, assign5220_e4501_d_n8, assign5220_e4501_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5220_e4498: f64 = (p.p440 * var_wphy);
        let assign5220_e4499: f64 = (p.p438 + assign5220_e4498);
        (assign5220_e4499, (p.p440 * var_wphy_dn4), (p.p440 * var_wphy_dn6), (p.p440 * var_wphy_dn7), (p.p440 * var_wphy_dn8), (p.p440 * var_wphy_dn9),)
    } else {
        (var_cfrd_p, var_cfrd_p_dn4, var_cfrd_p_dn6, var_cfrd_p_dn7, var_cfrd_p_dn8, var_cfrd_p_dn9,)
    }
};
        var_cfrd_p = assign5220_e4501;
        var_cfrd_p_dn4 = assign5220_e4501_d_n4;
        var_cfrd_p_dn6 = assign5220_e4501_d_n6;
        var_cfrd_p_dn7 = assign5220_e4501_d_n7;
        var_cfrd_p_dn8 = assign5220_e4501_d_n8;
        var_cfrd_p_dn9 = assign5220_e4501_d_n9;
        var_cfrd_p_rv = 0.0;

        let (assign5230_e4508, assign5230_e4508_d_n4, assign5230_e4508_d_n6, assign5230_e4508_d_n7, assign5230_e4508_d_n8, assign5230_e4508_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5230_e4506: f64 = (var_cfrd_p).max(0.0);
        (assign5230_e4506, if var_cfrd_p >= 0.0 { var_cfrd_p_dn4 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn6 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn7 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn8 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn9 } else { 0.0 },)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign5230_e4508;
        var_cfrd_i_dn4 = assign5230_e4508_d_n4;
        var_cfrd_i_dn6 = assign5230_e4508_d_n6;
        var_cfrd_i_dn7 = assign5230_e4508_d_n7;
        var_cfrd_i_dn8 = assign5230_e4508_d_n8;
        var_cfrd_i_dn9 = assign5230_e4508_d_n9;
        var_cfrd_i_rv = 0.0;

        let (assign5240_e4521,) = {
    if (var_guard83 == 0.0) {
        let assign5240_e4513: f64 = (p.p441 * var_epsch);
        let assign5240_e4515: f64 = (assign5240_e4513 * var_tsi_i);
        let assign5240_e4517: f64 = (assign5240_e4515 * var_we);
        let assign5240_e4519: f64 = (assign5240_e4517 / var_le);
        (assign5240_e4519,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign5240_e4521;
        var_csd_i_rv = 0.0;

        let (assign5250_e4526,) = {
    if (var_guard83 == 0.0) {
        (p.p442,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign5250_e4526;
        var_csdbp_i_rv = 0.0;

        let (assign5260_e4547, assign5260_e4547_d_n4, assign5260_e4547_d_n6, assign5260_e4547_d_n7, assign5260_e4547_d_n8, assign5260_e4547_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5260_e4532: f64 = (p.p444 * var_lphy);
        let assign5260_e4533: f64 = (1.0 + assign5260_e4532);
        let assign5260_e4536: f64 = (p.p445 * var_wphy);
        let assign5260_e4537: f64 = (assign5260_e4533 + assign5260_e4536);
        let assign5260_e4540: f64 = (p.p446 * var_lphy);
        let assign5260_e4542: f64 = (assign5260_e4540 * var_wphy);
        let assign5260_e4543: f64 = (assign5260_e4537 + assign5260_e4542);
        let assign5260_e4545: f64 = (assign5260_e4543).max(1e-10);
        (assign5260_e4545, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn4) + (p.p445 * var_wphy_dn4)) + (((p.p446 * var_lphy_dn4) * var_wphy) + (assign5260_e4540 * var_wphy_dn4))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn6) + (p.p445 * var_wphy_dn6)) + (((p.p446 * var_lphy_dn6) * var_wphy) + (assign5260_e4540 * var_wphy_dn6))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn7) + (p.p445 * var_wphy_dn7)) + (((p.p446 * var_lphy_dn7) * var_wphy) + (assign5260_e4540 * var_wphy_dn7))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn8) + (p.p445 * var_wphy_dn8)) + (((p.p446 * var_lphy_dn8) * var_wphy) + (assign5260_e4540 * var_wphy_dn8))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn9) + (p.p445 * var_wphy_dn9)) + (((p.p446 * var_lphy_dn9) * var_wphy) + (assign5260_e4540 * var_wphy_dn9))) } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5260_e4547;
        var_temp_dn4 = assign5260_e4547_d_n4;
        var_temp_dn6 = assign5260_e4547_d_n6;
        var_temp_dn7 = assign5260_e4547_d_n7;
        var_temp_dn8 = assign5260_e4547_d_n8;
        var_temp_dn9 = assign5260_e4547_d_n9;
        var_temp_rv = 0.0;

        let (assign5270_e4552, assign5270_e4552_d_n4, assign5270_e4552_d_n6, assign5270_e4552_d_n7, assign5270_e4552_d_n8, assign5270_e4552_d_n9,) = {
    if (var_guard83 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign5270_e4552;
        var_temp1_dn4 = assign5270_e4552_d_n4;
        var_temp1_dn6 = assign5270_e4552_d_n6;
        var_temp1_dn7 = assign5270_e4552_d_n7;
        var_temp1_dn8 = assign5270_e4552_d_n8;
        var_temp1_dn9 = assign5270_e4552_d_n9;
        var_temp1_rv = 0.0;

        let assign5280_e4559: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        var_guard129 = assign5280_e4559;
        var_guard129_rv = 0.0;

        let (assign5290_e4571, assign5290_e4571_d_n4, assign5290_e4571_d_n6, assign5290_e4571_d_n7, assign5290_e4571_d_n8, assign5290_e4571_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5290_e4566: f64 = (p.p28 + p.p20);
        let assign5290_e4567: f64 = (-assign5290_e4566);
        let assign5290_e4569: f64 = (assign5290_e4567 / p.p449);
        (assign5290_e4569, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign5290_e4571;
        var_temp2_dn4 = assign5290_e4571_d_n4;
        var_temp2_dn6 = assign5290_e4571_d_n6;
        var_temp2_dn7 = assign5290_e4571_d_n7;
        var_temp2_dn8 = assign5290_e4571_d_n8;
        var_temp2_dn9 = assign5290_e4571_d_n9;
        var_temp2_rv = 0.0;

        let assign5300_e4573: f64 = (var_temp2).abs();
        let assign5300_e4575: f64 = if assign5300_e4573 < 80.0 { 1.0 } else { 0.0 };
        var_guard130 = assign5300_e4575;
        var_guard130_rv = 0.0;

        let (assign5310_e4585, assign5310_e4585_d_n4, assign5310_e4585_d_n6, assign5310_e4585_d_n7, assign5310_e4585_d_n8, assign5310_e4585_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 != 0.0)) {
        let assign5310_e4583: f64 = (var_temp2).exp();
        (assign5310_e4583, (assign5310_e4583 * var_temp2_dn4), (assign5310_e4583 * var_temp2_dn6), (assign5310_e4583 * var_temp2_dn7), (assign5310_e4583 * var_temp2_dn8), (assign5310_e4583 * var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5310_e4585;
        var_temp3_dn4 = assign5310_e4585_d_n4;
        var_temp3_dn6 = assign5310_e4585_d_n6;
        var_temp3_dn7 = assign5310_e4585_d_n7;
        var_temp3_dn8 = assign5310_e4585_d_n8;
        var_temp3_dn9 = assign5310_e4585_d_n9;
        var_temp3_rv = 0.0;

        let assign5320_e4588: f64 = (-80.0);
        let assign5320_e4589: f64 = if var_temp2 < assign5320_e4588 { 1.0 } else { 0.0 };
        var_guard131 = assign5320_e4589;
        var_guard131_rv = 0.0;

        let (assign5330_e4626, assign5330_e4626_d_n4, assign5330_e4626_d_n6, assign5330_e4626_d_n7, assign5330_e4626_d_n8, assign5330_e4626_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 != 0.0)) {
        let assign5330_e4602: f64 = (-var_temp2);
        let assign5330_e4604: f64 = (assign5330_e4602 - 80.0);
        let assign5330_e4608: f64 = (-var_temp2);
        let assign5330_e4610: f64 = (assign5330_e4608 - 80.0);
        let assign5330_e4611: f64 = (0.5 * assign5330_e4610);
        let assign5330_e4614: f64 = (-var_temp2);
        let assign5330_e4616: f64 = (assign5330_e4614 - 80.0);
        let assign5330_e4618: f64 = (assign5330_e4616 * 0.3333333333333);
        let assign5330_e4619: f64 = (1.0 + assign5330_e4618);
        let assign5330_e4620: f64 = (assign5330_e4611 * assign5330_e4619);
        let assign5330_e4621: f64 = (1.0 + assign5330_e4620);
        let assign5330_e4622: f64 = (assign5330_e4604 * assign5330_e4621);
        let assign5330_e4623: f64 = (1.0 + assign5330_e4622);
        let assign5330_e4624: f64 = (1.80485e-35 / assign5330_e4623);
        (assign5330_e4624, (-((1.80485e-35 * (((-var_temp2_dn4) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn4)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn4) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn6) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn6)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn6) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn7) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn7)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn7) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn8) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn8)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn8) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn9) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn9)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn9) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5330_e4626;
        var_temp3_dn4 = assign5330_e4626_d_n4;
        var_temp3_dn6 = assign5330_e4626_d_n6;
        var_temp3_dn7 = assign5330_e4626_d_n7;
        var_temp3_dn8 = assign5330_e4626_d_n8;
        var_temp3_dn9 = assign5330_e4626_d_n9;
        var_temp3_rv = 0.0;

        let (assign5340_e4661, assign5340_e4661_d_n4, assign5340_e4661_d_n6, assign5340_e4661_d_n7, assign5340_e4661_d_n8, assign5340_e4661_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 == 0.0)) {
        let assign5340_e4641: f64 = (var_temp2 - 80.0);
        let assign5340_e4646: f64 = (var_temp2 - 80.0);
        let assign5340_e4647: f64 = (0.5 * assign5340_e4646);
        let assign5340_e4651: f64 = (var_temp2 - 80.0);
        let assign5340_e4653: f64 = (assign5340_e4651 * 0.3333333333333);
        let assign5340_e4654: f64 = (1.0 + assign5340_e4653);
        let assign5340_e4655: f64 = (assign5340_e4647 * assign5340_e4654);
        let assign5340_e4656: f64 = (1.0 + assign5340_e4655);
        let assign5340_e4657: f64 = (assign5340_e4641 * assign5340_e4656);
        let assign5340_e4658: f64 = (1.0 + assign5340_e4657);
        let assign5340_e4659: f64 = (5.54062e34 * assign5340_e4658);
        (assign5340_e4659, (5.54062e34 * ((var_temp2_dn4 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn4) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn6 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn6) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn7 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn7) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn8 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn8) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn9 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn9) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5340_e4661;
        var_temp3_dn4 = assign5340_e4661_d_n4;
        var_temp3_dn6 = assign5340_e4661_d_n6;
        var_temp3_dn7 = assign5340_e4661_d_n7;
        var_temp3_dn8 = assign5340_e4661_d_n8;
        var_temp3_dn9 = assign5340_e4661_d_n9;
        var_temp3_rv = 0.0;

        let (assign5350_e4670, assign5350_e4670_d_n4, assign5350_e4670_d_n6, assign5350_e4670_d_n7, assign5350_e4670_d_n8, assign5350_e4670_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5350_e4668: f64 = (1.0 - var_temp3);
        (assign5350_e4668, (-var_temp3_dn4), (-var_temp3_dn6), (-var_temp3_dn7), (-var_temp3_dn8), (-var_temp3_dn9),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign5350_e4670;
        var_temp4_dn4 = assign5350_e4670_d_n4;
        var_temp4_dn6 = assign5350_e4670_d_n6;
        var_temp4_dn7 = assign5350_e4670_d_n7;
        var_temp4_dn8 = assign5350_e4670_d_n8;
        var_temp4_dn9 = assign5350_e4670_d_n9;
        var_temp4_rv = 0.0;

        let (assign5360_e4695, assign5360_e4695_d_n4, assign5360_e4695_d_n6, assign5360_e4695_d_n7, assign5360_e4695_d_n8, assign5360_e4695_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5360_e4677: f64 = (2.0 * p.p450);
        let assign5360_e4679: f64 = (assign5360_e4677 * var_temp3);
        let assign5360_e4684: f64 = (var_temp3).powf(p.p29);
        let assign5360_e4685: f64 = (1.0 - assign5360_e4684);
        let assign5360_e4687: f64 = (assign5360_e4685 / p.p29);
        let assign5360_e4688: f64 = (var_temp4 - assign5360_e4687);
        let assign5360_e4689: f64 = (assign5360_e4679 * assign5360_e4688);
        let assign5360_e4692: f64 = (var_temp4 * var_temp4);
        let assign5360_e4693: f64 = (assign5360_e4689 / assign5360_e4692);
        (assign5360_e4693, ((((((assign5360_e4677 * var_temp3_dn4) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn4)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn4 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn4 * var_temp4) + (var_temp4 * var_temp4_dn4)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn6) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn6)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn6 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn6 * var_temp4) + (var_temp4 * var_temp4_dn6)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn7) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn7)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn7 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn7 * var_temp4) + (var_temp4 * var_temp4_dn7)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn8) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn8)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn8 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn8 * var_temp4) + (var_temp4 * var_temp4_dn8)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn9) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn9)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn9 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn9 * var_temp4) + (var_temp4 * var_temp4_dn9)))) / (assign5360_e4692 * assign5360_e4692)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign5360_e4695;
        var_temp1_dn4 = assign5360_e4695_d_n4;
        var_temp1_dn6 = assign5360_e4695_d_n6;
        var_temp1_dn7 = assign5360_e4695_d_n7;
        var_temp1_dn8 = assign5360_e4695_d_n8;
        var_temp1_dn9 = assign5360_e4695_d_n9;
        var_temp1_rv = 0.0;

        let (assign5370_e4704, assign5370_e4704_d_n4, assign5370_e4704_d_n6, assign5370_e4704_d_n7, assign5370_e4704_d_n8, assign5370_e4704_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5370_e4701: f64 = (1.0 + var_temp1);
        let assign5370_e4702: f64 = (var_temp / assign5370_e4701);
        (assign5370_e4702, (((var_temp_dn4 * assign5370_e4701) - (var_temp * var_temp1_dn4)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn6 * assign5370_e4701) - (var_temp * var_temp1_dn6)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn7 * assign5370_e4701) - (var_temp * var_temp1_dn7)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn8 * assign5370_e4701) - (var_temp * var_temp1_dn8)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn9 * assign5370_e4701) - (var_temp * var_temp1_dn9)) / (assign5370_e4701 * assign5370_e4701)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5370_e4704;
        var_temp_dn4 = assign5370_e4704_d_n4;
        var_temp_dn6 = assign5370_e4704_d_n6;
        var_temp_dn7 = assign5370_e4704_d_n7;
        var_temp_dn8 = assign5370_e4704_d_n8;
        var_temp_dn9 = assign5370_e4704_d_n9;
        var_temp_rv = 0.0;

        let (assign5380_e4711, assign5380_e4711_d_n4, assign5380_e4711_d_n6, assign5380_e4711_d_n7, assign5380_e4711_d_n8, assign5380_e4711_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5380_e4709: f64 = (p.p443 / var_temp);
        (assign5380_e4709, (-((p.p443 * var_temp_dn4) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn6) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn7) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn8) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn9) / (var_temp * var_temp))),)
    } else {
        (var_rth_p, var_rth_p_dn4, var_rth_p_dn6, var_rth_p_dn7, var_rth_p_dn8, var_rth_p_dn9,)
    }
};
        var_rth_p = assign5380_e4711;
        var_rth_p_dn4 = assign5380_e4711_d_n4;
        var_rth_p_dn6 = assign5380_e4711_d_n6;
        var_rth_p_dn7 = assign5380_e4711_d_n7;
        var_rth_p_dn8 = assign5380_e4711_d_n8;
        var_rth_p_dn9 = assign5380_e4711_d_n9;
        var_rth_p_rv = 0.0;

        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_alpacl2_i_slot = var_alpacl2_i;
        *var_alpacl2_i_rv_slot = var_alpacl2_i_rv;
        *var_alpaclexp2_i_slot = var_alpaclexp2_i;
        *var_alpaclexp2_i_rv_slot = var_alpaclexp2_i_rv;
        *var_alpaclexp_i_slot = var_alpaclexp_i;
        *var_alpaclexp_i_rv_slot = var_alpaclexp_i_rv;
        *var_alpacw_i_slot = var_alpacw_i;
        *var_alpacw_i_rv_slot = var_alpacw_i_rv;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfr_i_rv_slot = var_cfr_i_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_dn4_slot = var_cfr_p_dn4;
        *var_cfr_p_dn6_slot = var_cfr_p_dn6;
        *var_cfr_p_dn7_slot = var_cfr_p_dn7;
        *var_cfr_p_dn8_slot = var_cfr_p_dn8;
        *var_cfr_p_dn9_slot = var_cfr_p_dn9;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_dn4_slot = var_cfrd_p_dn4;
        *var_cfrd_p_dn6_slot = var_cfrd_p_dn6;
        *var_cfrd_p_dn7_slot = var_cfrd_p_dn7;
        *var_cfrd_p_dn8_slot = var_cfrd_p_dn8;
        *var_cfrd_p_dn9_slot = var_cfrd_p_dn9;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_cov_i_rv_slot = var_cov_i_rv;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covd_i_rv_slot = var_covd_i_rv;
        *var_covdl_i_slot = var_covdl_i;
        *var_covdl_i_rv_slot = var_covdl_i_rv;
        *var_covdlb_i_slot = var_covdlb_i;
        *var_covdlb_i_rv_slot = var_covdlb_i_rv;
        *var_csd_i_slot = var_csd_i;
        *var_csd_i_rv_slot = var_csd_i_rv;
        *var_csdbp_i_slot = var_csdbp_i;
        *var_csdbp_i_rv_slot = var_csdbp_i_rv;
        *var_dvfbov_i_slot = var_dvfbov_i;
        *var_dvfbov_i_rv_slot = var_dvfbov_i_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_rth_p_slot = var_rth_p;
        *var_rth_p_dn4_slot = var_rth_p_dn4;
        *var_rth_p_dn6_slot = var_rth_p_dn6;
        *var_rth_p_dn7_slot = var_rth_p_dn7;
        *var_rth_p_dn8_slot = var_rth_p_dn8;
        *var_rth_p_dn9_slot = var_rth_p_dn9;
        *var_rth_p_rv_slot = var_rth_p_rv;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp3_rv_slot = var_temp3_rv;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp4_rv_slot = var_temp4_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_dellps: f64,
        var_delwod: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_rt: f64,
        var_rt_dn4: f64,
        var_rt_dn6: f64,
        var_rt_dn7: f64,
        var_rt_dn8: f64,
        var_rt_dn9: f64,
        var_rth_p: f64,
        var_rth_p_dn4: f64,
        var_rth_p_dn6: f64,
        var_rth_p_dn7: f64,
        var_rth_p_dn8: f64,
        var_rth_p_dn9: f64,
        var_w_i: f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_cth_i_rv_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_dn4_slot: &mut f64,
        var_cth_p_dn6_slot: &mut f64,
        var_cth_p_dn7_slot: &mut f64,
        var_cth_p_dn8_slot: &mut f64,
        var_cth_p_dn9_slot: &mut f64,
        var_cth_p_rv_slot: &mut f64,
        var_fracinv_i_slot: &mut f64,
        var_fracinv_i_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_iloop_rv_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_dn4_slot: &mut f64,
        var_invsa_dn6_slot: &mut f64,
        var_invsa_dn7_slot: &mut f64,
        var_invsa_dn8_slot: &mut f64,
        var_invsa_dn9_slot: &mut f64,
        var_invsa_rv_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsaref_rv_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsb_rv_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_invsbref_rv_slot: &mut f64,
        var_kdiff_i_slot: &mut f64,
        var_kdiff_i_dn4_slot: &mut f64,
        var_kdiff_i_dn6_slot: &mut f64,
        var_kdiff_i_dn7_slot: &mut f64,
        var_kdiff_i_dn8_slot: &mut f64,
        var_kdiff_i_dn9_slot: &mut f64,
        var_kdiff_i_rv_slot: &mut f64,
        var_kdrift_i_slot: &mut f64,
        var_kdrift_i_dn4_slot: &mut f64,
        var_kdrift_i_dn6_slot: &mut f64,
        var_kdrift_i_dn7_slot: &mut f64,
        var_kdrift_i_dn8_slot: &mut f64,
        var_kdrift_i_dn9_slot: &mut f64,
        var_kdrift_i_rv_slot: &mut f64,
        var_kfracinv_i_slot: &mut f64,
        var_kfracinv_i_rv_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_dn4_slot: &mut f64,
        var_kstressu0_dn6_slot: &mut f64,
        var_kstressu0_dn7_slot: &mut f64,
        var_kstressu0_dn8_slot: &mut f64,
        var_kstressu0_dn9_slot: &mut f64,
        var_kstressu0_rv_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_lx_rv_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfa_i_rv_slot: &mut f64,
        var_nfa_p_slot: &mut f64,
        var_nfa_p_rv_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfb_i_rv_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfc_i_rv_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfe_i_rv_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_nfeb_i_rv_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_dn4_slot: &mut f64,
        var_rhobeta_dn6_slot: &mut f64,
        var_rhobeta_dn7_slot: &mut f64,
        var_rhobeta_dn8_slot: &mut f64,
        var_rhobeta_dn9_slot: &mut f64,
        var_rhobeta_rv_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_dn4_slot: &mut f64,
        var_rhobetaref_dn6_slot: &mut f64,
        var_rhobetaref_dn7_slot: &mut f64,
        var_rhobetaref_dn8_slot: &mut f64,
        var_rhobetaref_dn9_slot: &mut f64,
        var_rhobetaref_rv_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_rth_t_rv_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_strth_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_templ_rv_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_tempw_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_wx_slot: &mut f64,
        var_wx_rv_slot: &mut f64,
    ) {
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_cth_i_rv: f64 = *var_cth_i_rv_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_dn4: f64 = *var_cth_p_dn4_slot;
        let mut var_cth_p_dn6: f64 = *var_cth_p_dn6_slot;
        let mut var_cth_p_dn7: f64 = *var_cth_p_dn7_slot;
        let mut var_cth_p_dn8: f64 = *var_cth_p_dn8_slot;
        let mut var_cth_p_dn9: f64 = *var_cth_p_dn9_slot;
        let mut var_cth_p_rv: f64 = *var_cth_p_rv_slot;
        let mut var_fracinv_i: f64 = *var_fracinv_i_slot;
        let mut var_fracinv_i_rv: f64 = *var_fracinv_i_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_iloop_rv: f64 = *var_iloop_rv_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_dn4: f64 = *var_invsa_dn4_slot;
        let mut var_invsa_dn6: f64 = *var_invsa_dn6_slot;
        let mut var_invsa_dn7: f64 = *var_invsa_dn7_slot;
        let mut var_invsa_dn8: f64 = *var_invsa_dn8_slot;
        let mut var_invsa_dn9: f64 = *var_invsa_dn9_slot;
        let mut var_invsa_rv: f64 = *var_invsa_rv_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsaref_rv: f64 = *var_invsaref_rv_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsb_rv: f64 = *var_invsb_rv_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_invsbref_rv: f64 = *var_invsbref_rv_slot;
        let mut var_kdiff_i: f64 = *var_kdiff_i_slot;
        let mut var_kdiff_i_dn4: f64 = *var_kdiff_i_dn4_slot;
        let mut var_kdiff_i_dn6: f64 = *var_kdiff_i_dn6_slot;
        let mut var_kdiff_i_dn7: f64 = *var_kdiff_i_dn7_slot;
        let mut var_kdiff_i_dn8: f64 = *var_kdiff_i_dn8_slot;
        let mut var_kdiff_i_dn9: f64 = *var_kdiff_i_dn9_slot;
        let mut var_kdiff_i_rv: f64 = *var_kdiff_i_rv_slot;
        let mut var_kdrift_i: f64 = *var_kdrift_i_slot;
        let mut var_kdrift_i_dn4: f64 = *var_kdrift_i_dn4_slot;
        let mut var_kdrift_i_dn6: f64 = *var_kdrift_i_dn6_slot;
        let mut var_kdrift_i_dn7: f64 = *var_kdrift_i_dn7_slot;
        let mut var_kdrift_i_dn8: f64 = *var_kdrift_i_dn8_slot;
        let mut var_kdrift_i_dn9: f64 = *var_kdrift_i_dn9_slot;
        let mut var_kdrift_i_rv: f64 = *var_kdrift_i_rv_slot;
        let mut var_kfracinv_i: f64 = *var_kfracinv_i_slot;
        let mut var_kfracinv_i_rv: f64 = *var_kfracinv_i_rv_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_dn4: f64 = *var_kstressu0_dn4_slot;
        let mut var_kstressu0_dn6: f64 = *var_kstressu0_dn6_slot;
        let mut var_kstressu0_dn7: f64 = *var_kstressu0_dn7_slot;
        let mut var_kstressu0_dn8: f64 = *var_kstressu0_dn8_slot;
        let mut var_kstressu0_dn9: f64 = *var_kstressu0_dn9_slot;
        let mut var_kstressu0_rv: f64 = *var_kstressu0_rv_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_lx_rv: f64 = *var_lx_rv_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfa_i_rv: f64 = *var_nfa_i_rv_slot;
        let mut var_nfa_p: f64 = *var_nfa_p_slot;
        let mut var_nfa_p_rv: f64 = *var_nfa_p_rv_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfb_i_rv: f64 = *var_nfb_i_rv_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfc_i_rv: f64 = *var_nfc_i_rv_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfe_i_rv: f64 = *var_nfe_i_rv_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_nfeb_i_rv: f64 = *var_nfeb_i_rv_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_dn4: f64 = *var_rhobeta_dn4_slot;
        let mut var_rhobeta_dn6: f64 = *var_rhobeta_dn6_slot;
        let mut var_rhobeta_dn7: f64 = *var_rhobeta_dn7_slot;
        let mut var_rhobeta_dn8: f64 = *var_rhobeta_dn8_slot;
        let mut var_rhobeta_dn9: f64 = *var_rhobeta_dn9_slot;
        let mut var_rhobeta_rv: f64 = *var_rhobeta_rv_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_dn4: f64 = *var_rhobetaref_dn4_slot;
        let mut var_rhobetaref_dn6: f64 = *var_rhobetaref_dn6_slot;
        let mut var_rhobetaref_dn7: f64 = *var_rhobetaref_dn7_slot;
        let mut var_rhobetaref_dn8: f64 = *var_rhobetaref_dn8_slot;
        let mut var_rhobetaref_dn9: f64 = *var_rhobetaref_dn9_slot;
        let mut var_rhobetaref_rv: f64 = *var_rhobetaref_rv_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_rth_t_rv: f64 = *var_rth_t_rv_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_strth_i_rv: f64 = *var_strth_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_templ_rv: f64 = *var_templ_rv_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_tempw_rv: f64 = *var_tempw_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_wx: f64 = *var_wx_slot;
        let mut var_wx_rv: f64 = *var_wx_rv_slot;

        let (assign5390_e4718, assign5390_e4718_d_n4, assign5390_e4718_d_n6, assign5390_e4718_d_n7, assign5390_e4718_d_n8, assign5390_e4718_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5390_e4716: f64 = (var_rth_p).max(1e-6);
        (assign5390_e4716, if var_rth_p >= 1e-6 { var_rth_p_dn4 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn6 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn7 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn8 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn9 } else { 0.0 },)
    } else {
        (var_rth_t, var_rth_t_dn4, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9,)
    }
};
        var_rth_t = assign5390_e4718;
        var_rth_t_dn4 = assign5390_e4718_d_n4;
        var_rth_t_dn6 = assign5390_e4718_d_n6;
        var_rth_t_dn7 = assign5390_e4718_d_n7;
        var_rth_t_dn8 = assign5390_e4718_d_n8;
        var_rth_t_dn9 = assign5390_e4718_d_n9;
        var_rth_t_rv = 0.0;

        let (assign5400_e4723,) = {
    if (var_guard83 == 0.0) {
        (p.p447,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign5400_e4723;
        var_strth_i_rv = 0.0;

        let (assign5410_e4730, assign5410_e4730_d_n4, assign5410_e4730_d_n6, assign5410_e4730_d_n7, assign5410_e4730_d_n8, assign5410_e4730_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5410_e4728: f64 = (p.p448 * var_temp);
        (assign5410_e4728, (p.p448 * var_temp_dn4), (p.p448 * var_temp_dn6), (p.p448 * var_temp_dn7), (p.p448 * var_temp_dn8), (p.p448 * var_temp_dn9),)
    } else {
        (var_cth_p, var_cth_p_dn4, var_cth_p_dn6, var_cth_p_dn7, var_cth_p_dn8, var_cth_p_dn9,)
    }
};
        var_cth_p = assign5410_e4730;
        var_cth_p_dn4 = assign5410_e4730_d_n4;
        var_cth_p_dn6 = assign5410_e4730_d_n6;
        var_cth_p_dn7 = assign5410_e4730_d_n7;
        var_cth_p_dn8 = assign5410_e4730_d_n8;
        var_cth_p_dn9 = assign5410_e4730_d_n9;
        var_cth_p_rv = 0.0;

        let (assign5420_e4737, assign5420_e4737_d_n4, assign5420_e4737_d_n6, assign5420_e4737_d_n7, assign5420_e4737_d_n8, assign5420_e4737_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5420_e4735: f64 = (var_cth_p).max(0.0);
        (assign5420_e4735, if var_cth_p >= 0.0 { var_cth_p_dn4 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn6 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn7 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn8 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn9 } else { 0.0 },)
    } else {
        (var_cth_i, var_cth_i_dn4, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9,)
    }
};
        var_cth_i = assign5420_e4737;
        var_cth_i_dn4 = assign5420_e4737_d_n4;
        var_cth_i_dn6 = assign5420_e4737_d_n6;
        var_cth_i_dn7 = assign5420_e4737_d_n7;
        var_cth_i_dn8 = assign5420_e4737_d_n8;
        var_cth_i_dn9 = assign5420_e4737_d_n9;
        var_cth_i_rv = 0.0;

        let (assign5450_e4772,) = {
    if (var_guard83 == 0.0) {
        let assign5450_e4766: f64 = (p.p454 * var_iae);
        let assign5450_e4769: f64 = (p.p455 * var_iwe);
        let assign5450_e4770: f64 = (assign5450_e4766 + assign5450_e4769);
        (assign5450_e4770,)
    } else {
        (var_nfa_p,)
    }
};
        var_nfa_p = assign5450_e4772;
        var_nfa_p_rv = 0.0;

        let (assign5460_e4779,) = {
    if (var_guard83 == 0.0) {
        let assign5460_e4777: f64 = (var_nfa_p).max(0.0);
        (assign5460_e4777,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign5460_e4779;
        var_nfa_i_rv = 0.0;

        let (assign5470_e4786,) = {
    if (var_guard83 == 0.0) {
        let assign5470_e4784: f64 = (p.p456 * var_iae);
        (assign5470_e4784,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign5470_e4786;
        var_nfb_i_rv = 0.0;

        let (assign5480_e4793,) = {
    if (var_guard83 == 0.0) {
        let assign5480_e4791: f64 = (p.p457 * var_iae);
        (assign5480_e4791,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign5480_e4793;
        var_nfc_i_rv = 0.0;

        let (assign5490_e4798,) = {
    if (var_guard83 == 0.0) {
        (p.p458,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign5490_e4798;
        var_nfe_i_rv = 0.0;

        let (assign5500_e4803,) = {
    if (var_guard83 == 0.0) {
        (p.p459,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign5500_e4803;
        var_nfeb_i_rv = 0.0;

        let (assign5520_e4817, assign5520_e4817_d_n4, assign5520_e4817_d_n6, assign5520_e4817_d_n7, assign5520_e4817_d_n8, assign5520_e4817_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5520_e4814: f64 = (p.p490 * var_ile);
        let assign5520_e4815: f64 = (p.p489 + assign5520_e4814);
        (assign5520_e4815, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5520_e4817;
        var_temp_dn4 = assign5520_e4817_d_n4;
        var_temp_dn6 = assign5520_e4817_d_n6;
        var_temp_dn7 = assign5520_e4817_d_n7;
        var_temp_dn8 = assign5520_e4817_d_n8;
        var_temp_dn9 = assign5520_e4817_d_n9;
        var_temp_rv = 0.0;

        let (assign5530_e4824, assign5530_e4824_d_n4, assign5530_e4824_d_n6, assign5530_e4824_d_n7, assign5530_e4824_d_n8, assign5530_e4824_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5530_e4822: f64 = (var_temp).max(0.0);
        (assign5530_e4822, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_kdrift_i, var_kdrift_i_dn4, var_kdrift_i_dn6, var_kdrift_i_dn7, var_kdrift_i_dn8, var_kdrift_i_dn9,)
    }
};
        var_kdrift_i = assign5530_e4824;
        var_kdrift_i_dn4 = assign5530_e4824_d_n4;
        var_kdrift_i_dn6 = assign5530_e4824_d_n6;
        var_kdrift_i_dn7 = assign5530_e4824_d_n7;
        var_kdrift_i_dn8 = assign5530_e4824_d_n8;
        var_kdrift_i_dn9 = assign5530_e4824_d_n9;
        var_kdrift_i_rv = 0.0;

        let (assign5540_e4833, assign5540_e4833_d_n4, assign5540_e4833_d_n6, assign5540_e4833_d_n7, assign5540_e4833_d_n8, assign5540_e4833_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5540_e4830: f64 = (p.p492 * var_ile);
        let assign5540_e4831: f64 = (p.p491 + assign5540_e4830);
        (assign5540_e4831, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5540_e4833;
        var_temp_dn4 = assign5540_e4833_d_n4;
        var_temp_dn6 = assign5540_e4833_d_n6;
        var_temp_dn7 = assign5540_e4833_d_n7;
        var_temp_dn8 = assign5540_e4833_d_n8;
        var_temp_dn9 = assign5540_e4833_d_n9;
        var_temp_rv = 0.0;

        let (assign5550_e4840, assign5550_e4840_d_n4, assign5550_e4840_d_n6, assign5550_e4840_d_n7, assign5550_e4840_d_n8, assign5550_e4840_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5550_e4838: f64 = (var_temp).max(0.0);
        (assign5550_e4838, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_kdiff_i, var_kdiff_i_dn4, var_kdiff_i_dn6, var_kdiff_i_dn7, var_kdiff_i_dn8, var_kdiff_i_dn9,)
    }
};
        var_kdiff_i = assign5550_e4840;
        var_kdiff_i_dn4 = assign5550_e4840_d_n4;
        var_kdiff_i_dn6 = assign5550_e4840_d_n6;
        var_kdiff_i_dn7 = assign5550_e4840_d_n7;
        var_kdiff_i_dn8 = assign5550_e4840_d_n8;
        var_kdiff_i_dn9 = assign5550_e4840_d_n9;
        var_kdiff_i_rv = 0.0;

        let (assign5560_e4845,) = {
    if (var_guard83 == 0.0) {
        (p.p493,)
    } else {
        (var_fracinv_i,)
    }
};
        var_fracinv_i = assign5560_e4845;
        var_fracinv_i_rv = 0.0;

        let (assign5570_e4850,) = {
    if (var_guard83 == 0.0) {
        (p.p494,)
    } else {
        (var_kfracinv_i,)
    }
};
        var_kfracinv_i = assign5570_e4850;
        var_kfracinv_i_rv = 0.0;

        let assign5670_e4958: f64 = if ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard133 = assign5670_e4958;
        var_guard133_rv = 0.0;

        let assign5680_e4961: f64 = if p.p461 == 1.0 { 1.0 } else { 0.0 };
        var_guard134 = assign5680_e4961;
        var_guard134_rv = 0.0;

        let (assign5690_e4970, assign5690_e4970_d_n4, assign5690_e4970_d_n6, assign5690_e4970_d_n7, assign5690_e4970_d_n8, assign5690_e4970_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
        var_tmpa = assign5690_e4970;
        var_tmpa_dn4 = assign5690_e4970_d_n4;
        var_tmpa_dn6 = assign5690_e4970_d_n6;
        var_tmpa_dn7 = assign5690_e4970_d_n7;
        var_tmpa_dn8 = assign5690_e4970_d_n8;
        var_tmpa_dn9 = assign5690_e4970_d_n9;
        var_tmpa_rv = 0.0;

        let (assign5700_e4979,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign5700_e4979;
        var_tmpb_rv = 0.0;

        let (assign5710_e4988,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign5710_e4988;
        var_iloop_rv = 0.0;

        let mut assign5720_loop_guard: usize = 0;
        while {
            let assign5720_cond_e4998: f64 = (p.p29 - 0.5);
            let assign5720_cond_e5000: f64 = if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) && (var_iloop < assign5720_cond_e4998)) { 1.0 } else { 0.0 };
            assign5720_cond_e5000 != 0.0
        } {
            assign5720_loop_guard += 1;
            assert!(assign5720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5720_body0_e5023, assign5720_body0_e5023_d_n4, assign5720_body0_e5023_d_n6, assign5720_body0_e5023_d_n7, assign5720_body0_e5023_d_n8, assign5720_body0_e5023_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body0_e5012: f64 = (0.5 * p.p20);
        let assign5720_body0_e5013: f64 = (p.p26 + assign5720_body0_e5012);
        let assign5720_body0_e5017: f64 = (p.p28 + p.p20);
        let assign5720_body0_e5018: f64 = (var_iloop * assign5720_body0_e5017);
        let assign5720_body0_e5019: f64 = (assign5720_body0_e5013 + assign5720_body0_e5018);
        let assign5720_body0_e5020: f64 = (1.0 / assign5720_body0_e5019);
        let assign5720_body0_e5021: f64 = (var_tmpa + assign5720_body0_e5020);
        (assign5720_body0_e5021, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
            var_tmpa = assign5720_body0_e5023;
            var_tmpa_dn4 = assign5720_body0_e5023_d_n4;
            var_tmpa_dn6 = assign5720_body0_e5023_d_n6;
            var_tmpa_dn7 = assign5720_body0_e5023_d_n7;
            var_tmpa_dn8 = assign5720_body0_e5023_d_n8;
            var_tmpa_dn9 = assign5720_body0_e5023_d_n9;
            var_tmpa_rv = 0.0;
            let (assign5720_body1_e5046,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body1_e5035: f64 = (0.5 * p.p20);
        let assign5720_body1_e5036: f64 = (p.p27 + assign5720_body1_e5035);
        let assign5720_body1_e5040: f64 = (p.p28 + p.p20);
        let assign5720_body1_e5041: f64 = (var_iloop * assign5720_body1_e5040);
        let assign5720_body1_e5042: f64 = (assign5720_body1_e5036 + assign5720_body1_e5041);
        let assign5720_body1_e5043: f64 = (1.0 / assign5720_body1_e5042);
        let assign5720_body1_e5044: f64 = (var_tmpb + assign5720_body1_e5043);
        (assign5720_body1_e5044,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign5720_body1_e5046;
            var_tmpb_rv = 0.0;
            let (assign5720_body2_e5057,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body2_e5055: f64 = (var_iloop + 1.0);
        (assign5720_body2_e5055,)
    } else {
        (var_iloop,)
    }
};
            var_iloop = assign5720_body2_e5057;
            var_iloop_rv = 0.0;
        }

        let (assign5730_e5068, assign5730_e5068_d_n4, assign5730_e5068_d_n6, assign5730_e5068_d_n7, assign5730_e5068_d_n8, assign5730_e5068_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5730_e5066: f64 = (var_tmpa / p.p29);
        (assign5730_e5066, (var_tmpa_dn4 / p.p29), (var_tmpa_dn6 / p.p29), (var_tmpa_dn7 / p.p29), (var_tmpa_dn8 / p.p29), (var_tmpa_dn9 / p.p29),)
    } else {
        (var_invsa, var_invsa_dn4, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9,)
    }
};
        var_invsa = assign5730_e5068;
        var_invsa_dn4 = assign5730_e5068_d_n4;
        var_invsa_dn6 = assign5730_e5068_d_n6;
        var_invsa_dn7 = assign5730_e5068_d_n7;
        var_invsa_dn8 = assign5730_e5068_d_n8;
        var_invsa_dn9 = assign5730_e5068_d_n9;
        var_invsa_rv = 0.0;

        let (assign5740_e5079,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5740_e5077: f64 = (var_tmpb / p.p29);
        (assign5740_e5077,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign5740_e5079;
        var_invsb_rv = 0.0;

        let (assign5750_e5094,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5750_e5090: f64 = (0.5 * p.p20);
        let assign5750_e5091: f64 = (p.p462 + assign5750_e5090);
        let assign5750_e5092: f64 = (1.0 / assign5750_e5091);
        (assign5750_e5092,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign5750_e5094;
        var_invsaref_rv = 0.0;

        let (assign5760_e5109,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5760_e5105: f64 = (0.5 * p.p20);
        let assign5760_e5106: f64 = (p.p463 + assign5760_e5105);
        let assign5760_e5107: f64 = (1.0 / assign5760_e5106);
        (assign5760_e5107,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign5760_e5109;
        var_invsbref_rv = 0.0;

        let (assign5770_e5122,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5770_e5118: f64 = (p.p20 + var_dellps);
        let assign5770_e5120: f64 = (assign5770_e5118).max(1e-9);
        (assign5770_e5120,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign5770_e5122;
        var_lx_rv = 0.0;

        let (assign5780_e5137,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5780_e5131: f64 = (var_w_i + var_delwod);
        let assign5780_e5133: f64 = (assign5780_e5131 + p.p464);
        let assign5780_e5135: f64 = (assign5780_e5133).max(1e-9);
        (assign5780_e5135,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign5780_e5137;
        var_wx_rv = 0.0;

        let (assign5790_e5150,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5790_e5147: f64 = (var_lx).powf(p.p471);
        let assign5790_e5148: f64 = (1.0 / assign5790_e5147);
        (assign5790_e5148,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5790_e5150;
        var_templ_rv = 0.0;

        let (assign5800_e5163,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5800_e5160: f64 = (var_wx).powf(p.p472);
        let assign5800_e5161: f64 = (1.0 / assign5800_e5160);
        (assign5800_e5161,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5800_e5163;
        var_tempw_rv = 0.0;

        let (assign5810_e5194, assign5810_e5194_d_n4, assign5810_e5194_d_n6, assign5810_e5194_d_n7, assign5810_e5194_d_n8, assign5810_e5194_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5810_e5173: f64 = (p.p468 * var_templ);
        let assign5810_e5174: f64 = (1.0 + assign5810_e5173);
        let assign5810_e5177: f64 = (p.p469 * var_tempw);
        let assign5810_e5178: f64 = (assign5810_e5174 + assign5810_e5177);
        let assign5810_e5181: f64 = (p.p470 * var_templ);
        let assign5810_e5183: f64 = (assign5810_e5181 * var_tempw);
        let assign5810_e5184: f64 = (assign5810_e5178 + assign5810_e5183);
        let assign5810_e5189: f64 = (var_rt - 1.0);
        let assign5810_e5190: f64 = (p.p467 * assign5810_e5189);
        let assign5810_e5191: f64 = (1.0 + assign5810_e5190);
        let assign5810_e5192: f64 = (assign5810_e5184 * assign5810_e5191);
        (assign5810_e5192, (assign5810_e5184 * (p.p467 * var_rt_dn4)), (assign5810_e5184 * (p.p467 * var_rt_dn6)), (assign5810_e5184 * (p.p467 * var_rt_dn7)), (assign5810_e5184 * (p.p467 * var_rt_dn8)), (assign5810_e5184 * (p.p467 * var_rt_dn9)),)
    } else {
        (var_kstressu0, var_kstressu0_dn4, var_kstressu0_dn6, var_kstressu0_dn7, var_kstressu0_dn8, var_kstressu0_dn9,)
    }
};
        var_kstressu0 = assign5810_e5194;
        var_kstressu0_dn4 = assign5810_e5194_d_n4;
        var_kstressu0_dn6 = assign5810_e5194_d_n6;
        var_kstressu0_dn7 = assign5810_e5194_d_n7;
        var_kstressu0_dn8 = assign5810_e5194_d_n8;
        var_kstressu0_dn9 = assign5810_e5194_d_n9;
        var_kstressu0_rv = 0.0;

        let (assign5820_e5209, assign5820_e5209_d_n4, assign5820_e5209_d_n6, assign5820_e5209_d_n7, assign5820_e5209_d_n8, assign5820_e5209_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5820_e5204: f64 = (var_invsa + var_invsb);
        let assign5820_e5205: f64 = (p.p465 * assign5820_e5204);
        let assign5820_e5207: f64 = (assign5820_e5205 / var_kstressu0);
        (assign5820_e5207, ((((p.p465 * var_invsa_dn4) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn4)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn6) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn6)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn7) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn7)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn8) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn8)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn9) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn9)) / (var_kstressu0 * var_kstressu0)),)
    } else {
        (var_rhobeta, var_rhobeta_dn4, var_rhobeta_dn6, var_rhobeta_dn7, var_rhobeta_dn8, var_rhobeta_dn9,)
    }
};
        var_rhobeta = assign5820_e5209;
        var_rhobeta_dn4 = assign5820_e5209_d_n4;
        var_rhobeta_dn6 = assign5820_e5209_d_n6;
        var_rhobeta_dn7 = assign5820_e5209_d_n7;
        var_rhobeta_dn8 = assign5820_e5209_d_n8;
        var_rhobeta_dn9 = assign5820_e5209_d_n9;
        var_rhobeta_rv = 0.0;

        let (assign5830_e5224, assign5830_e5224_d_n4, assign5830_e5224_d_n6, assign5830_e5224_d_n7, assign5830_e5224_d_n8, assign5830_e5224_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5830_e5219: f64 = (var_invsaref + var_invsbref);
        let assign5830_e5220: f64 = (p.p465 * assign5830_e5219);
        let assign5830_e5222: f64 = (assign5830_e5220 / var_kstressu0);
        (assign5830_e5222, (-((assign5830_e5220 * var_kstressu0_dn4) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn6) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn7) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn8) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn9) / (var_kstressu0 * var_kstressu0))),)
    } else {
        (var_rhobetaref, var_rhobetaref_dn4, var_rhobetaref_dn6, var_rhobetaref_dn7, var_rhobetaref_dn8, var_rhobetaref_dn9,)
    }
};
        var_rhobetaref = assign5830_e5224;
        var_rhobetaref_dn4 = assign5830_e5224_d_n4;
        var_rhobetaref_dn6 = assign5830_e5224_d_n6;
        var_rhobetaref_dn7 = assign5830_e5224_d_n7;
        var_rhobetaref_dn8 = assign5830_e5224_d_n8;
        var_rhobetaref_dn9 = assign5830_e5224_d_n9;
        var_rhobetaref_rv = 0.0;

        let (assign5840_e5237,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5840_e5234: f64 = (var_lx).powf(p.p477);
        let assign5840_e5235: f64 = (1.0 / assign5840_e5234);
        (assign5840_e5235,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5840_e5237;
        var_templ_rv = 0.0;

        let (assign5850_e5250,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5850_e5247: f64 = (var_wx).powf(p.p478);
        let assign5850_e5248: f64 = (1.0 / assign5850_e5247);
        (assign5850_e5248,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5850_e5250;
        var_tempw_rv = 0.0;

        *var_cth_i_slot = var_cth_i;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_cth_i_rv_slot = var_cth_i_rv;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_dn4_slot = var_cth_p_dn4;
        *var_cth_p_dn6_slot = var_cth_p_dn6;
        *var_cth_p_dn7_slot = var_cth_p_dn7;
        *var_cth_p_dn8_slot = var_cth_p_dn8;
        *var_cth_p_dn9_slot = var_cth_p_dn9;
        *var_cth_p_rv_slot = var_cth_p_rv;
        *var_fracinv_i_slot = var_fracinv_i;
        *var_fracinv_i_rv_slot = var_fracinv_i_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_iloop_slot = var_iloop;
        *var_iloop_rv_slot = var_iloop_rv;
        *var_invsa_slot = var_invsa;
        *var_invsa_dn4_slot = var_invsa_dn4;
        *var_invsa_dn6_slot = var_invsa_dn6;
        *var_invsa_dn7_slot = var_invsa_dn7;
        *var_invsa_dn8_slot = var_invsa_dn8;
        *var_invsa_dn9_slot = var_invsa_dn9;
        *var_invsa_rv_slot = var_invsa_rv;
        *var_invsaref_slot = var_invsaref;
        *var_invsaref_rv_slot = var_invsaref_rv;
        *var_invsb_slot = var_invsb;
        *var_invsb_rv_slot = var_invsb_rv;
        *var_invsbref_slot = var_invsbref;
        *var_invsbref_rv_slot = var_invsbref_rv;
        *var_kdiff_i_slot = var_kdiff_i;
        *var_kdiff_i_dn4_slot = var_kdiff_i_dn4;
        *var_kdiff_i_dn6_slot = var_kdiff_i_dn6;
        *var_kdiff_i_dn7_slot = var_kdiff_i_dn7;
        *var_kdiff_i_dn8_slot = var_kdiff_i_dn8;
        *var_kdiff_i_dn9_slot = var_kdiff_i_dn9;
        *var_kdiff_i_rv_slot = var_kdiff_i_rv;
        *var_kdrift_i_slot = var_kdrift_i;
        *var_kdrift_i_dn4_slot = var_kdrift_i_dn4;
        *var_kdrift_i_dn6_slot = var_kdrift_i_dn6;
        *var_kdrift_i_dn7_slot = var_kdrift_i_dn7;
        *var_kdrift_i_dn8_slot = var_kdrift_i_dn8;
        *var_kdrift_i_dn9_slot = var_kdrift_i_dn9;
        *var_kdrift_i_rv_slot = var_kdrift_i_rv;
        *var_kfracinv_i_slot = var_kfracinv_i;
        *var_kfracinv_i_rv_slot = var_kfracinv_i_rv;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_dn4_slot = var_kstressu0_dn4;
        *var_kstressu0_dn6_slot = var_kstressu0_dn6;
        *var_kstressu0_dn7_slot = var_kstressu0_dn7;
        *var_kstressu0_dn8_slot = var_kstressu0_dn8;
        *var_kstressu0_dn9_slot = var_kstressu0_dn9;
        *var_kstressu0_rv_slot = var_kstressu0_rv;
        *var_lx_slot = var_lx;
        *var_lx_rv_slot = var_lx_rv;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfa_i_rv_slot = var_nfa_i_rv;
        *var_nfa_p_slot = var_nfa_p;
        *var_nfa_p_rv_slot = var_nfa_p_rv;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfb_i_rv_slot = var_nfb_i_rv;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfc_i_rv_slot = var_nfc_i_rv;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfe_i_rv_slot = var_nfe_i_rv;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_nfeb_i_rv_slot = var_nfeb_i_rv;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_dn4_slot = var_rhobeta_dn4;
        *var_rhobeta_dn6_slot = var_rhobeta_dn6;
        *var_rhobeta_dn7_slot = var_rhobeta_dn7;
        *var_rhobeta_dn8_slot = var_rhobeta_dn8;
        *var_rhobeta_dn9_slot = var_rhobeta_dn9;
        *var_rhobeta_rv_slot = var_rhobeta_rv;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_dn4_slot = var_rhobetaref_dn4;
        *var_rhobetaref_dn6_slot = var_rhobetaref_dn6;
        *var_rhobetaref_dn7_slot = var_rhobetaref_dn7;
        *var_rhobetaref_dn8_slot = var_rhobetaref_dn8;
        *var_rhobetaref_dn9_slot = var_rhobetaref_dn9;
        *var_rhobetaref_rv_slot = var_rhobetaref_rv;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_rth_t_rv_slot = var_rth_t_rv;
        *var_strth_i_slot = var_strth_i;
        *var_strth_i_rv_slot = var_strth_i_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_templ_slot = var_templ;
        *var_templ_rv_slot = var_templ_rv;
        *var_tempw_slot = var_tempw;
        *var_tempw_rv_slot = var_tempw_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_wx_slot = var_wx;
        *var_wx_rv_slot = var_wx_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_invsa: f64,
        var_invsa_dn4: f64,
        var_invsa_dn6: f64,
        var_invsa_dn7: f64,
        var_invsa_dn8: f64,
        var_invsa_dn9: f64,
        var_invsaref: f64,
        var_invsb: f64,
        var_invsbref: f64,
        var_rhobeta: f64,
        var_rhobeta_dn4: f64,
        var_rhobeta_dn6: f64,
        var_rhobeta_dn7: f64,
        var_rhobeta_dn8: f64,
        var_rhobeta_dn9: f64,
        var_rhobetaref: f64,
        var_rhobetaref_dn4: f64,
        var_rhobetaref_dn6: f64,
        var_rhobetaref_dn7: f64,
        var_rhobetaref_dn8: f64,
        var_rhobetaref_dn9: f64,
        var_templ: f64,
        var_tempw: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn1_t_rv_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn2_t_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf1_t_rv_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf2_t_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac1_t_rv_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac2_t_rv_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfac_p_rv_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_iloop_rv_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kstressvth0_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp0__blk79_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb1_t_rv_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfb2_t_rv_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac1_t_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn1_t_rv: f64 = *var_betn1_t_rv_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn2_t_rv: f64 = *var_betn2_t_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf1_t_rv: f64 = *var_cf1_t_rv_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf2_t_rv: f64 = *var_cf2_t_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac1_t_rv: f64 = *var_cfac1_t_rv_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac2_t_rv: f64 = *var_cfac2_t_rv_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfac_p_rv: f64 = *var_cfac_p_rv_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_iloop_rv: f64 = *var_iloop_rv_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kstressvth0_rv: f64 = *var_kstressvth0_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp0__blk79_rv: f64 = *var_temp0__blk79_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb1_t_rv: f64 = *var_vfb1_t_rv_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfb2_t_rv: f64 = *var_vfb2_t_rv_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac1_t_rv: f64 = *var_vfbac1_t_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;

        let (assign5860_e5275,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5860_e5260: f64 = (p.p474 * var_templ);
        let assign5860_e5261: f64 = (1.0 + assign5860_e5260);
        let assign5860_e5264: f64 = (p.p475 * var_tempw);
        let assign5860_e5265: f64 = (assign5860_e5261 + assign5860_e5264);
        let assign5860_e5268: f64 = (p.p476 * var_templ);
        let assign5860_e5270: f64 = (assign5860_e5268 * var_tempw);
        let assign5860_e5271: f64 = (assign5860_e5265 + assign5860_e5270);
        let assign5860_e5273: f64 = (assign5860_e5271).max(1e-20);
        (assign5860_e5273,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign5860_e5275;
        var_kstressvth0_rv = 0.0;

        let (assign5870_e5290, assign5870_e5290_d_n4, assign5870_e5290_d_n6, assign5870_e5290_d_n7, assign5870_e5290_d_n8, assign5870_e5290_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5870_e5284: f64 = (var_invsa + var_invsb);
        let assign5870_e5286: f64 = (assign5870_e5284 - var_invsaref);
        let assign5870_e5288: f64 = (assign5870_e5286 - var_invsbref);
        (assign5870_e5288, var_invsa_dn4, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign5870_e5290;
        var_temp0__blk79_dn4 = assign5870_e5290_d_n4;
        var_temp0__blk79_dn6 = assign5870_e5290_d_n6;
        var_temp0__blk79_dn7 = assign5870_e5290_d_n7;
        var_temp0__blk79_dn8 = assign5870_e5290_d_n8;
        var_temp0__blk79_dn9 = assign5870_e5290_d_n9;
        var_temp0__blk79_rv = 0.0;

        let (assign5880_e5307, assign5880_e5307_d_n4, assign5880_e5307_d_n6, assign5880_e5307_d_n7, assign5880_e5307_d_n8, assign5880_e5307_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5880_e5300: f64 = (1.0 + var_rhobeta);
        let assign5880_e5301: f64 = (var_betn_p * assign5880_e5300);
        let assign5880_e5304: f64 = (1.0 + var_rhobetaref);
        let assign5880_e5305: f64 = (assign5880_e5301 / assign5880_e5304);
        (assign5880_e5305, (((((var_betn_p_dn4 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn4)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn4)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn6 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn6)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn6)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn7 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn7)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn7)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn8 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn8)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn8)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn9 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn9)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn9)) / (assign5880_e5304 * assign5880_e5304)),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign5880_e5307;
        var_betn_p_dn4 = assign5880_e5307_d_n4;
        var_betn_p_dn6 = assign5880_e5307_d_n6;
        var_betn_p_dn7 = assign5880_e5307_d_n7;
        var_betn_p_dn8 = assign5880_e5307_d_n8;
        var_betn_p_dn9 = assign5880_e5307_d_n9;
        var_betn_p_rv = 0.0;

        let (assign5890_e5318, assign5890_e5318_d_n4, assign5890_e5318_d_n6, assign5890_e5318_d_n7, assign5890_e5318_d_n8, assign5890_e5318_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5890_e5316: f64 = (var_betn_p).max(1e-10);
        (assign5890_e5316, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign5890_e5318;
        var_betn1_t_dn4 = assign5890_e5318_d_n4;
        var_betn1_t_dn6 = assign5890_e5318_d_n6;
        var_betn1_t_dn7 = assign5890_e5318_d_n7;
        var_betn1_t_dn8 = assign5890_e5318_d_n8;
        var_betn1_t_dn9 = assign5890_e5318_d_n9;
        var_betn1_t_rv = 0.0;

        let (assign5900_e5329, assign5900_e5329_d_n4, assign5900_e5329_d_n6, assign5900_e5329_d_n7, assign5900_e5329_d_n8, assign5900_e5329_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5900_e5327: f64 = (p.p254 * var_betn1_t);
        (assign5900_e5327, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign5900_e5329;
        var_betn2_t_dn4 = assign5900_e5329_d_n4;
        var_betn2_t_dn6 = assign5900_e5329_d_n6;
        var_betn2_t_dn7 = assign5900_e5329_d_n7;
        var_betn2_t_dn8 = assign5900_e5329_d_n8;
        var_betn2_t_dn9 = assign5900_e5329_d_n9;
        var_betn2_t_rv = 0.0;

        let (assign5910_e5356, assign5910_e5356_d_n4, assign5910_e5356_d_n6, assign5910_e5356_d_n7, assign5910_e5356_d_n8, assign5910_e5356_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5910_e5338: f64 = (1.0 + var_rhobeta);
        let assign5910_e5342: f64 = (p.p466 * var_rhobetaref);
        let assign5910_e5343: f64 = (1.0 + assign5910_e5342);
        let assign5910_e5344: f64 = (assign5910_e5338 * assign5910_e5343);
        let assign5910_e5347: f64 = (1.0 + var_rhobetaref);
        let assign5910_e5351: f64 = (p.p466 * var_rhobeta);
        let assign5910_e5352: f64 = (1.0 + assign5910_e5351);
        let assign5910_e5353: f64 = (assign5910_e5347 * assign5910_e5352);
        let assign5910_e5354: f64 = (assign5910_e5344 / assign5910_e5353);
        (assign5910_e5354, (((((var_rhobeta_dn4 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn4))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn4 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn4))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn6 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn6))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn6 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn6))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn7 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn7))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn7 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn7))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn8 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn8))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn8 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn8))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn9 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn9))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn9 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn9))))) / (assign5910_e5353 * assign5910_e5353)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5910_e5356;
        var_temp_dn4 = assign5910_e5356_d_n4;
        var_temp_dn6 = assign5910_e5356_d_n6;
        var_temp_dn7 = assign5910_e5356_d_n7;
        var_temp_dn8 = assign5910_e5356_d_n8;
        var_temp_dn9 = assign5910_e5356_d_n9;
        var_temp_rv = 0.0;

        let (assign5920_e5367, assign5920_e5367_d_n4, assign5920_e5367_d_n6, assign5920_e5367_d_n7, assign5920_e5367_d_n8, assign5920_e5367_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5920_e5365: f64 = (var_thesat_p * var_temp);
        (assign5920_e5365, ((var_thesat_p_dn4 * var_temp) + (var_thesat_p * var_temp_dn4)), ((var_thesat_p_dn6 * var_temp) + (var_thesat_p * var_temp_dn6)), ((var_thesat_p_dn7 * var_temp) + (var_thesat_p * var_temp_dn7)), ((var_thesat_p_dn8 * var_temp) + (var_thesat_p * var_temp_dn8)), ((var_thesat_p_dn9 * var_temp) + (var_thesat_p * var_temp_dn9)),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign5920_e5367;
        var_thesat_p_dn4 = assign5920_e5367_d_n4;
        var_thesat_p_dn6 = assign5920_e5367_d_n6;
        var_thesat_p_dn7 = assign5920_e5367_d_n7;
        var_thesat_p_dn8 = assign5920_e5367_d_n8;
        var_thesat_p_dn9 = assign5920_e5367_d_n9;
        var_thesat_p_rv = 0.0;

        let (assign5930_e5378, assign5930_e5378_d_n4, assign5930_e5378_d_n6, assign5930_e5378_d_n7, assign5930_e5378_d_n8, assign5930_e5378_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5930_e5376: f64 = (var_thesat_p).max(0.0);
        (assign5930_e5376, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign5930_e5378;
        var_thesat_t_dn4 = assign5930_e5378_d_n4;
        var_thesat_t_dn6 = assign5930_e5378_d_n6;
        var_thesat_t_dn7 = assign5930_e5378_d_n7;
        var_thesat_t_dn8 = assign5930_e5378_d_n8;
        var_thesat_t_dn9 = assign5930_e5378_d_n9;
        var_thesat_t_rv = 0.0;

        let (assign5940_e5389, assign5940_e5389_d_n4, assign5940_e5389_d_n6, assign5940_e5389_d_n7, assign5940_e5389_d_n8, assign5940_e5389_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5940_e5387: f64 = (var_thesatac_p * var_temp);
        (assign5940_e5387, ((var_thesatac_p_dn4 * var_temp) + (var_thesatac_p * var_temp_dn4)), ((var_thesatac_p_dn6 * var_temp) + (var_thesatac_p * var_temp_dn6)), ((var_thesatac_p_dn7 * var_temp) + (var_thesatac_p * var_temp_dn7)), ((var_thesatac_p_dn8 * var_temp) + (var_thesatac_p * var_temp_dn8)), ((var_thesatac_p_dn9 * var_temp) + (var_thesatac_p * var_temp_dn9)),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign5940_e5389;
        var_thesatac_p_dn4 = assign5940_e5389_d_n4;
        var_thesatac_p_dn6 = assign5940_e5389_d_n6;
        var_thesatac_p_dn7 = assign5940_e5389_d_n7;
        var_thesatac_p_dn8 = assign5940_e5389_d_n8;
        var_thesatac_p_dn9 = assign5940_e5389_d_n9;
        var_thesatac_p_rv = 0.0;

        let (assign5950_e5400, assign5950_e5400_d_n4, assign5950_e5400_d_n6, assign5950_e5400_d_n7, assign5950_e5400_d_n8, assign5950_e5400_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5950_e5398: f64 = (var_thesatac_p).max(0.0);
        (assign5950_e5398, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign5950_e5400;
        var_thesatac_t_dn4 = assign5950_e5400_d_n4;
        var_thesatac_t_dn6 = assign5950_e5400_d_n6;
        var_thesatac_t_dn7 = assign5950_e5400_d_n7;
        var_thesatac_t_dn8 = assign5950_e5400_d_n8;
        var_thesatac_t_dn9 = assign5950_e5400_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign5960_e5413, assign5960_e5413_d_n4, assign5960_e5413_d_n6, assign5960_e5413_d_n7, assign5960_e5413_d_n8, assign5960_e5413_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5960_e5409: f64 = (p.p473 * var_temp0__blk79);
        let assign5960_e5411: f64 = (assign5960_e5409 / var_kstressvth0);
        (assign5960_e5411, ((p.p473 * var_temp0__blk79_dn4) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn6) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn7) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn8) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn9) / var_kstressvth0),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5960_e5413;
        var_temp_dn4 = assign5960_e5413_d_n4;
        var_temp_dn6 = assign5960_e5413_d_n6;
        var_temp_dn7 = assign5960_e5413_d_n7;
        var_temp_dn8 = assign5960_e5413_d_n8;
        var_temp_dn9 = assign5960_e5413_d_n9;
        var_temp_rv = 0.0;

        let (assign5970_e5424, assign5970_e5424_d_n4, assign5970_e5424_d_n6, assign5970_e5424_d_n7, assign5970_e5424_d_n8, assign5970_e5424_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5970_e5422: f64 = (var_vfb1_t + var_temp);
        (assign5970_e5422, (var_vfb1_t_dn4 + var_temp_dn4), (var_vfb1_t_dn6 + var_temp_dn6), (var_vfb1_t_dn7 + var_temp_dn7), (var_vfb1_t_dn8 + var_temp_dn8), (var_vfb1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign5970_e5424;
        var_vfb1_t_dn4 = assign5970_e5424_d_n4;
        var_vfb1_t_dn6 = assign5970_e5424_d_n6;
        var_vfb1_t_dn7 = assign5970_e5424_d_n7;
        var_vfb1_t_dn8 = assign5970_e5424_d_n8;
        var_vfb1_t_dn9 = assign5970_e5424_d_n9;
        var_vfb1_t_rv = 0.0;

        let (assign5980_e5435, assign5980_e5435_d_n4, assign5980_e5435_d_n6, assign5980_e5435_d_n7, assign5980_e5435_d_n8, assign5980_e5435_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5980_e5433: f64 = (var_vfb2_t + var_temp);
        (assign5980_e5433, (var_vfb2_t_dn4 + var_temp_dn4), (var_vfb2_t_dn6 + var_temp_dn6), (var_vfb2_t_dn7 + var_temp_dn7), (var_vfb2_t_dn8 + var_temp_dn8), (var_vfb2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign5980_e5435;
        var_vfb2_t_dn4 = assign5980_e5435_d_n4;
        var_vfb2_t_dn6 = assign5980_e5435_d_n6;
        var_vfb2_t_dn7 = assign5980_e5435_d_n7;
        var_vfb2_t_dn8 = assign5980_e5435_d_n8;
        var_vfb2_t_dn9 = assign5980_e5435_d_n9;
        var_vfb2_t_rv = 0.0;

        let (assign5990_e5446, assign5990_e5446_d_n4, assign5990_e5446_d_n6, assign5990_e5446_d_n7, assign5990_e5446_d_n8, assign5990_e5446_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5990_e5444: f64 = (var_vfbac1_t + var_temp);
        (assign5990_e5444, (var_vfbac1_t_dn4 + var_temp_dn4), (var_vfbac1_t_dn6 + var_temp_dn6), (var_vfbac1_t_dn7 + var_temp_dn7), (var_vfbac1_t_dn8 + var_temp_dn8), (var_vfbac1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign5990_e5446;
        var_vfbac1_t_dn4 = assign5990_e5446_d_n4;
        var_vfbac1_t_dn6 = assign5990_e5446_d_n6;
        var_vfbac1_t_dn7 = assign5990_e5446_d_n7;
        var_vfbac1_t_dn8 = assign5990_e5446_d_n8;
        var_vfbac1_t_dn9 = assign5990_e5446_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign6000_e5457, assign6000_e5457_d_n4, assign6000_e5457_d_n6, assign6000_e5457_d_n7, assign6000_e5457_d_n8, assign6000_e5457_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6000_e5455: f64 = (var_vfbac2_t + var_temp);
        (assign6000_e5455, (var_vfbac2_t_dn4 + var_temp_dn4), (var_vfbac2_t_dn6 + var_temp_dn6), (var_vfbac2_t_dn7 + var_temp_dn7), (var_vfbac2_t_dn8 + var_temp_dn8), (var_vfbac2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign6000_e5457;
        var_vfbac2_t_dn4 = assign6000_e5457_d_n4;
        var_vfbac2_t_dn6 = assign6000_e5457_d_n6;
        var_vfbac2_t_dn7 = assign6000_e5457_d_n7;
        var_vfbac2_t_dn8 = assign6000_e5457_d_n8;
        var_vfbac2_t_dn9 = assign6000_e5457_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign6010_e5472, assign6010_e5472_d_n4, assign6010_e5472_d_n6, assign6010_e5472_d_n7, assign6010_e5472_d_n8, assign6010_e5472_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6010_e5466: f64 = (p.p479 * var_temp0__blk79);
        let assign6010_e5469: f64 = (var_kstressvth0).powf(p.p480);
        let assign6010_e5470: f64 = (assign6010_e5466 / assign6010_e5469);
        (assign6010_e5470, ((p.p479 * var_temp0__blk79_dn4) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn6) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn7) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn8) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn9) / assign6010_e5469),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6010_e5472;
        var_temp_dn4 = assign6010_e5472_d_n4;
        var_temp_dn6 = assign6010_e5472_d_n6;
        var_temp_dn7 = assign6010_e5472_d_n7;
        var_temp_dn8 = assign6010_e5472_d_n8;
        var_temp_dn9 = assign6010_e5472_d_n9;
        var_temp_rv = 0.0;

        let (assign6020_e5483, assign6020_e5483_d_n4, assign6020_e5483_d_n6, assign6020_e5483_d_n7, assign6020_e5483_d_n8, assign6020_e5483_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6020_e5481: f64 = (var_cf_p + var_temp);
        (assign6020_e5481, (var_cf_p_dn4 + var_temp_dn4), (var_cf_p_dn6 + var_temp_dn6), (var_cf_p_dn7 + var_temp_dn7), (var_cf_p_dn8 + var_temp_dn8), (var_cf_p_dn9 + var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign6020_e5483;
        var_cf_p_dn4 = assign6020_e5483_d_n4;
        var_cf_p_dn6 = assign6020_e5483_d_n6;
        var_cf_p_dn7 = assign6020_e5483_d_n7;
        var_cf_p_dn8 = assign6020_e5483_d_n8;
        var_cf_p_dn9 = assign6020_e5483_d_n9;
        var_cf_p_rv = 0.0;

        let (assign6030_e5494, assign6030_e5494_d_n4, assign6030_e5494_d_n6, assign6030_e5494_d_n7, assign6030_e5494_d_n8, assign6030_e5494_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6030_e5492: f64 = (var_cf_p).max(0.0);
        (assign6030_e5492, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign6030_e5494;
        var_cf1_t_dn4 = assign6030_e5494_d_n4;
        var_cf1_t_dn6 = assign6030_e5494_d_n6;
        var_cf1_t_dn7 = assign6030_e5494_d_n7;
        var_cf1_t_dn8 = assign6030_e5494_d_n8;
        var_cf1_t_dn9 = assign6030_e5494_d_n9;
        var_cf1_t_rv = 0.0;

        let (assign6040_e5505, assign6040_e5505_d_n4, assign6040_e5505_d_n6, assign6040_e5505_d_n7, assign6040_e5505_d_n8, assign6040_e5505_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6040_e5503: f64 = (var_cfac_p + var_temp);
        (assign6040_e5503, (var_cfac_p_dn4 + var_temp_dn4), (var_cfac_p_dn6 + var_temp_dn6), (var_cfac_p_dn7 + var_temp_dn7), (var_cfac_p_dn8 + var_temp_dn8), (var_cfac_p_dn9 + var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign6040_e5505;
        var_cfac_p_dn4 = assign6040_e5505_d_n4;
        var_cfac_p_dn6 = assign6040_e5505_d_n6;
        var_cfac_p_dn7 = assign6040_e5505_d_n7;
        var_cfac_p_dn8 = assign6040_e5505_d_n8;
        var_cfac_p_dn9 = assign6040_e5505_d_n9;
        var_cfac_p_rv = 0.0;

        let (assign6050_e5516, assign6050_e5516_d_n4, assign6050_e5516_d_n6, assign6050_e5516_d_n7, assign6050_e5516_d_n8, assign6050_e5516_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6050_e5514: f64 = (var_cfac_p).max(0.0);
        (assign6050_e5514, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign6050_e5516;
        var_cfac1_t_dn4 = assign6050_e5516_d_n4;
        var_cfac1_t_dn6 = assign6050_e5516_d_n6;
        var_cfac1_t_dn7 = assign6050_e5516_d_n7;
        var_cfac1_t_dn8 = assign6050_e5516_d_n8;
        var_cfac1_t_dn9 = assign6050_e5516_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign6060_e5529, assign6060_e5529_d_n4, assign6060_e5529_d_n6, assign6060_e5529_d_n7, assign6060_e5529_d_n8, assign6060_e5529_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6060_e5525: f64 = (p.p238 * var_tox2_i);
        let assign6060_e5527: f64 = (assign6060_e5525 / var_tox1_i);
        (assign6060_e5527, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6060_e5529;
        var_temp_dn4 = assign6060_e5529_d_n4;
        var_temp_dn6 = assign6060_e5529_d_n6;
        var_temp_dn7 = assign6060_e5529_d_n7;
        var_temp_dn8 = assign6060_e5529_d_n8;
        var_temp_dn9 = assign6060_e5529_d_n9;
        var_temp_rv = 0.0;

        let (assign6070_e5540, assign6070_e5540_d_n4, assign6070_e5540_d_n6, assign6070_e5540_d_n7, assign6070_e5540_d_n8, assign6070_e5540_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6070_e5538: f64 = (var_cf1_t * var_temp);
        (assign6070_e5538, ((var_cf1_t_dn4 * var_temp) + (var_cf1_t * var_temp_dn4)), ((var_cf1_t_dn6 * var_temp) + (var_cf1_t * var_temp_dn6)), ((var_cf1_t_dn7 * var_temp) + (var_cf1_t * var_temp_dn7)), ((var_cf1_t_dn8 * var_temp) + (var_cf1_t * var_temp_dn8)), ((var_cf1_t_dn9 * var_temp) + (var_cf1_t * var_temp_dn9)),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign6070_e5540;
        var_cf2_t_dn4 = assign6070_e5540_d_n4;
        var_cf2_t_dn6 = assign6070_e5540_d_n6;
        var_cf2_t_dn7 = assign6070_e5540_d_n7;
        var_cf2_t_dn8 = assign6070_e5540_d_n8;
        var_cf2_t_dn9 = assign6070_e5540_d_n9;
        var_cf2_t_rv = 0.0;

        let (assign6080_e5551, assign6080_e5551_d_n4, assign6080_e5551_d_n6, assign6080_e5551_d_n7, assign6080_e5551_d_n8, assign6080_e5551_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6080_e5549: f64 = (var_cfac1_t * var_temp);
        (assign6080_e5549, ((var_cfac1_t_dn4 * var_temp) + (var_cfac1_t * var_temp_dn4)), ((var_cfac1_t_dn6 * var_temp) + (var_cfac1_t * var_temp_dn6)), ((var_cfac1_t_dn7 * var_temp) + (var_cfac1_t * var_temp_dn7)), ((var_cfac1_t_dn8 * var_temp) + (var_cfac1_t * var_temp_dn8)), ((var_cfac1_t_dn9 * var_temp) + (var_cfac1_t * var_temp_dn9)),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign6080_e5551;
        var_cfac2_t_dn4 = assign6080_e5551_d_n4;
        var_cfac2_t_dn6 = assign6080_e5551_d_n6;
        var_cfac2_t_dn7 = assign6080_e5551_d_n7;
        var_cfac2_t_dn8 = assign6080_e5551_d_n8;
        var_cfac2_t_dn9 = assign6080_e5551_d_n9;
        var_cfac2_t_rv = 0.0;

        let (assign6090_e5561, assign6090_e5561_d_n4, assign6090_e5561_d_n6, assign6090_e5561_d_n7, assign6090_e5561_d_n8, assign6090_e5561_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
        var_tmpa = assign6090_e5561;
        var_tmpa_dn4 = assign6090_e5561_d_n4;
        var_tmpa_dn6 = assign6090_e5561_d_n6;
        var_tmpa_dn7 = assign6090_e5561_d_n7;
        var_tmpa_dn8 = assign6090_e5561_d_n8;
        var_tmpa_dn9 = assign6090_e5561_d_n9;
        var_tmpa_rv = 0.0;

        let (assign6100_e5571,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign6100_e5571;
        var_iloop_rv = 0.0;

        let (assign6110_e5584, assign6110_e5584_d_n4, assign6110_e5584_d_n6, assign6110_e5584_d_n7, assign6110_e5584_d_n8, assign6110_e5584_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6110_e5580: f64 = (-1.0);
        let assign6110_e5582: f64 = (assign6110_e5580 / p.p482);
        (assign6110_e5582, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6110_e5584;
        var_temp_dn4 = assign6110_e5584_d_n4;
        var_temp_dn6 = assign6110_e5584_d_n6;
        var_temp_dn7 = assign6110_e5584_d_n7;
        var_temp_dn8 = assign6110_e5584_d_n8;
        var_temp_dn9 = assign6110_e5584_d_n9;
        var_temp_rv = 0.0;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn1_t_rv_slot = var_betn1_t_rv;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn2_t_rv_slot = var_betn2_t_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf1_t_rv_slot = var_cf1_t_rv;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf2_t_rv_slot = var_cf2_t_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac1_t_rv_slot = var_cfac1_t_rv;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac2_t_rv_slot = var_cfac2_t_rv;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfac_p_rv_slot = var_cfac_p_rv;
        *var_iloop_slot = var_iloop;
        *var_iloop_rv_slot = var_iloop_rv;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kstressvth0_rv_slot = var_kstressvth0_rv;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp0__blk79_rv_slot = var_temp0__blk79_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb1_t_rv_slot = var_vfb1_t_rv;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfb2_t_rv_slot = var_vfb2_t_rv;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac1_t_rv_slot = var_vfbac1_t_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_delwod: f64,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_rt: f64,
        var_rt_dn4: f64,
        var_rt_dn6: f64,
        var_rt_dn7: f64,
        var_rt_dn8: f64,
        var_rt_dn9: f64,
        var_temp: f64,
        var_temp_dn4: f64,
        var_temp_dn6: f64,
        var_temp_dn7: f64,
        var_temp_dn8: f64,
        var_temp_dn9: f64,
        var_w_i: f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_iloop_rv_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_dn4_slot: &mut f64,
        var_rhobeta_dn6_slot: &mut f64,
        var_rhobeta_dn7_slot: &mut f64,
        var_rhobeta_dn8_slot: &mut f64,
        var_rhobeta_dn9_slot: &mut f64,
        var_rhobeta_rv_slot: &mut f64,
        var_ruo_slot: &mut f64,
        var_ruo_dn4_slot: &mut f64,
        var_ruo_dn6_slot: &mut f64,
        var_ruo_dn7_slot: &mut f64,
        var_ruo_dn8_slot: &mut f64,
        var_ruo_dn9_slot: &mut f64,
        var_ruo_rv_slot: &mut f64,
        var_str_g_slot: &mut f64,
        var_str_g_dn4_slot: &mut f64,
        var_str_g_dn6_slot: &mut f64,
        var_str_g_dn7_slot: &mut f64,
        var_str_g_dn8_slot: &mut f64,
        var_str_g_dn9_slot: &mut f64,
        var_str_g_rv_slot: &mut f64,
        var_str_gref_slot: &mut f64,
        var_str_gref_dn4_slot: &mut f64,
        var_str_gref_dn6_slot: &mut f64,
        var_str_gref_dn7_slot: &mut f64,
        var_str_gref_dn8_slot: &mut f64,
        var_str_gref_dn9_slot: &mut f64,
        var_str_gref_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp3_rv_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp4_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_wx_slot: &mut f64,
        var_wx_rv_slot: &mut f64,
    ) {
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_iloop_rv: f64 = *var_iloop_rv_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_dn4: f64 = *var_rhobeta_dn4_slot;
        let mut var_rhobeta_dn6: f64 = *var_rhobeta_dn6_slot;
        let mut var_rhobeta_dn7: f64 = *var_rhobeta_dn7_slot;
        let mut var_rhobeta_dn8: f64 = *var_rhobeta_dn8_slot;
        let mut var_rhobeta_dn9: f64 = *var_rhobeta_dn9_slot;
        let mut var_rhobeta_rv: f64 = *var_rhobeta_rv_slot;
        let mut var_ruo: f64 = *var_ruo_slot;
        let mut var_ruo_dn4: f64 = *var_ruo_dn4_slot;
        let mut var_ruo_dn6: f64 = *var_ruo_dn6_slot;
        let mut var_ruo_dn7: f64 = *var_ruo_dn7_slot;
        let mut var_ruo_dn8: f64 = *var_ruo_dn8_slot;
        let mut var_ruo_dn9: f64 = *var_ruo_dn9_slot;
        let mut var_ruo_rv: f64 = *var_ruo_rv_slot;
        let mut var_str_g: f64 = *var_str_g_slot;
        let mut var_str_g_dn4: f64 = *var_str_g_dn4_slot;
        let mut var_str_g_dn6: f64 = *var_str_g_dn6_slot;
        let mut var_str_g_dn7: f64 = *var_str_g_dn7_slot;
        let mut var_str_g_dn8: f64 = *var_str_g_dn8_slot;
        let mut var_str_g_dn9: f64 = *var_str_g_dn9_slot;
        let mut var_str_g_rv: f64 = *var_str_g_rv_slot;
        let mut var_str_gref: f64 = *var_str_gref_slot;
        let mut var_str_gref_dn4: f64 = *var_str_gref_dn4_slot;
        let mut var_str_gref_dn6: f64 = *var_str_gref_dn6_slot;
        let mut var_str_gref_dn7: f64 = *var_str_gref_dn7_slot;
        let mut var_str_gref_dn8: f64 = *var_str_gref_dn8_slot;
        let mut var_str_gref_dn9: f64 = *var_str_gref_dn9_slot;
        let mut var_str_gref_rv: f64 = *var_str_gref_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp3_rv: f64 = *var_temp3_rv_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp4_rv: f64 = *var_temp4_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_wx: f64 = *var_wx_slot;
        let mut var_wx_rv: f64 = *var_wx_rv_slot;

        let mut assign6120_loop_guard: usize = 0;
        while {
            let assign6120_cond_e5595: f64 = (p.p29 - 0.5);
            let assign6120_cond_e5597: f64 = if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_iloop < assign6120_cond_e5595)) { 1.0 } else { 0.0 };
            assign6120_cond_e5597 != 0.0
        } {
            assign6120_loop_guard += 1;
            assert!(assign6120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let assign6120_body0_e5601: f64 = (0.5 * p.p20);
            let assign6120_body0_e5602: f64 = (p.p26 + assign6120_body0_e5601);
            let assign6120_body0_e5606: f64 = (p.p28 + p.p20);
            let assign6120_body0_e5607: f64 = (var_iloop * assign6120_body0_e5606);
            let assign6120_body0_e5608: f64 = (assign6120_body0_e5602 + assign6120_body0_e5607);
            let assign6120_body0_e5609: f64 = (-assign6120_body0_e5608);
            let assign6120_body0_e5611: f64 = (assign6120_body0_e5609 / p.p481);
            let assign6120_body0_e5613: f64 = (-80.0);
            let assign6120_body0_e5614: f64 = if assign6120_body0_e5611 > assign6120_body0_e5613 { 1.0 } else { 0.0 };
            var_guard135 = assign6120_body0_e5614;
            var_guard135_rv = 0.0;
            let (assign6120_body1_e5640, assign6120_body1_e5640_d_n4, assign6120_body1_e5640_d_n6, assign6120_body1_e5640_d_n7, assign6120_body1_e5640_d_n8, assign6120_body1_e5640_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard135 != 0.0)) {
        let assign6120_body1_e5627: f64 = (0.5 * p.p20);
        let assign6120_body1_e5628: f64 = (p.p26 + assign6120_body1_e5627);
        let assign6120_body1_e5632: f64 = (p.p28 + p.p20);
        let assign6120_body1_e5633: f64 = (var_iloop * assign6120_body1_e5632);
        let assign6120_body1_e5634: f64 = (assign6120_body1_e5628 + assign6120_body1_e5633);
        let assign6120_body1_e5635: f64 = (-assign6120_body1_e5634);
        let assign6120_body1_e5637: f64 = (assign6120_body1_e5635 / p.p481);
        let assign6120_body1_e5638: f64 = (assign6120_body1_e5637).exp();
        (assign6120_body1_e5638, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
            var_temp1 = assign6120_body1_e5640;
            var_temp1_dn4 = assign6120_body1_e5640_d_n4;
            var_temp1_dn6 = assign6120_body1_e5640_d_n6;
            var_temp1_dn7 = assign6120_body1_e5640_d_n7;
            var_temp1_dn8 = assign6120_body1_e5640_d_n8;
            var_temp1_dn9 = assign6120_body1_e5640_d_n9;
            var_temp1_rv = 0.0;
            let (assign6120_body2_e5717, assign6120_body2_e5717_d_n4, assign6120_body2_e5717_d_n6, assign6120_body2_e5717_d_n7, assign6120_body2_e5717_d_n8, assign6120_body2_e5717_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard135 == 0.0)) {
        let assign6120_body2_e5656: f64 = (0.5 * p.p20);
        let assign6120_body2_e5657: f64 = (p.p26 + assign6120_body2_e5656);
        let assign6120_body2_e5661: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5662: f64 = (var_iloop * assign6120_body2_e5661);
        let assign6120_body2_e5663: f64 = (assign6120_body2_e5657 + assign6120_body2_e5662);
        let assign6120_body2_e5664: f64 = (-assign6120_body2_e5663);
        let assign6120_body2_e5666: f64 = (assign6120_body2_e5664 / p.p481);
        let assign6120_body2_e5667: f64 = (-assign6120_body2_e5666);
        let assign6120_body2_e5669: f64 = (assign6120_body2_e5667 - 80.0);
        let assign6120_body2_e5675: f64 = (0.5 * p.p20);
        let assign6120_body2_e5676: f64 = (p.p26 + assign6120_body2_e5675);
        let assign6120_body2_e5680: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5681: f64 = (var_iloop * assign6120_body2_e5680);
        let assign6120_body2_e5682: f64 = (assign6120_body2_e5676 + assign6120_body2_e5681);
        let assign6120_body2_e5683: f64 = (-assign6120_body2_e5682);
        let assign6120_body2_e5685: f64 = (assign6120_body2_e5683 / p.p481);
        let assign6120_body2_e5686: f64 = (-assign6120_body2_e5685);
        let assign6120_body2_e5688: f64 = (assign6120_body2_e5686 - 80.0);
        let assign6120_body2_e5689: f64 = (0.5 * assign6120_body2_e5688);
        let assign6120_body2_e5694: f64 = (0.5 * p.p20);
        let assign6120_body2_e5695: f64 = (p.p26 + assign6120_body2_e5694);
        let assign6120_body2_e5699: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5700: f64 = (var_iloop * assign6120_body2_e5699);
        let assign6120_body2_e5701: f64 = (assign6120_body2_e5695 + assign6120_body2_e5700);
        let assign6120_body2_e5702: f64 = (-assign6120_body2_e5701);
        let assign6120_body2_e5704: f64 = (assign6120_body2_e5702 / p.p481);
        let assign6120_body2_e5705: f64 = (-assign6120_body2_e5704);
        let assign6120_body2_e5707: f64 = (assign6120_body2_e5705 - 80.0);
        let assign6120_body2_e5709: f64 = (assign6120_body2_e5707 * 0.3333333333333);
        let assign6120_body2_e5710: f64 = (1.0 + assign6120_body2_e5709);
        let assign6120_body2_e5711: f64 = (assign6120_body2_e5689 * assign6120_body2_e5710);
        let assign6120_body2_e5712: f64 = (1.0 + assign6120_body2_e5711);
        let assign6120_body2_e5713: f64 = (assign6120_body2_e5669 * assign6120_body2_e5712);
        let assign6120_body2_e5714: f64 = (1.0 + assign6120_body2_e5713);
        let assign6120_body2_e5715: f64 = (1.80485e-35 / assign6120_body2_e5714);
        (assign6120_body2_e5715, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
            var_temp1 = assign6120_body2_e5717;
            var_temp1_dn4 = assign6120_body2_e5717_d_n4;
            var_temp1_dn6 = assign6120_body2_e5717_d_n6;
            var_temp1_dn7 = assign6120_body2_e5717_d_n7;
            var_temp1_dn8 = assign6120_body2_e5717_d_n8;
            var_temp1_dn9 = assign6120_body2_e5717_d_n9;
            var_temp1_rv = 0.0;
            let assign6120_body3_e5721: f64 = (0.5 * p.p20);
            let assign6120_body3_e5722: f64 = (p.p27 + assign6120_body3_e5721);
            let assign6120_body3_e5725: f64 = (p.p29 - 1.0);
            let assign6120_body3_e5727: f64 = (assign6120_body3_e5725 - var_iloop);
            let assign6120_body3_e5730: f64 = (p.p28 + p.p20);
            let assign6120_body3_e5731: f64 = (assign6120_body3_e5727 * assign6120_body3_e5730);
            let assign6120_body3_e5732: f64 = (assign6120_body3_e5722 + assign6120_body3_e5731);
            let assign6120_body3_e5733: f64 = (-assign6120_body3_e5732);
            let assign6120_body3_e5735: f64 = (assign6120_body3_e5733 / p.p481);
            let assign6120_body3_e5737: f64 = (-80.0);
            let assign6120_body3_e5738: f64 = if assign6120_body3_e5735 > assign6120_body3_e5737 { 1.0 } else { 0.0 };
            var_guard136 = assign6120_body3_e5738;
            var_guard136_rv = 0.0;
            let (assign6120_body4_e5768, assign6120_body4_e5768_d_n4, assign6120_body4_e5768_d_n6, assign6120_body4_e5768_d_n7, assign6120_body4_e5768_d_n8, assign6120_body4_e5768_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard136 != 0.0)) {
        let assign6120_body4_e5751: f64 = (0.5 * p.p20);
        let assign6120_body4_e5752: f64 = (p.p27 + assign6120_body4_e5751);
        let assign6120_body4_e5755: f64 = (p.p29 - 1.0);
        let assign6120_body4_e5757: f64 = (assign6120_body4_e5755 - var_iloop);
        let assign6120_body4_e5760: f64 = (p.p28 + p.p20);
        let assign6120_body4_e5761: f64 = (assign6120_body4_e5757 * assign6120_body4_e5760);
        let assign6120_body4_e5762: f64 = (assign6120_body4_e5752 + assign6120_body4_e5761);
        let assign6120_body4_e5763: f64 = (-assign6120_body4_e5762);
        let assign6120_body4_e5765: f64 = (assign6120_body4_e5763 / p.p481);
        let assign6120_body4_e5766: f64 = (assign6120_body4_e5765).exp();
        (assign6120_body4_e5766, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
            var_temp2 = assign6120_body4_e5768;
            var_temp2_dn4 = assign6120_body4_e5768_d_n4;
            var_temp2_dn6 = assign6120_body4_e5768_d_n6;
            var_temp2_dn7 = assign6120_body4_e5768_d_n7;
            var_temp2_dn8 = assign6120_body4_e5768_d_n8;
            var_temp2_dn9 = assign6120_body4_e5768_d_n9;
            var_temp2_rv = 0.0;
            let (assign6120_body5_e5857, assign6120_body5_e5857_d_n4, assign6120_body5_e5857_d_n6, assign6120_body5_e5857_d_n7, assign6120_body5_e5857_d_n8, assign6120_body5_e5857_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard136 == 0.0)) {
        let assign6120_body5_e5784: f64 = (0.5 * p.p20);
        let assign6120_body5_e5785: f64 = (p.p27 + assign6120_body5_e5784);
        let assign6120_body5_e5788: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5790: f64 = (assign6120_body5_e5788 - var_iloop);
        let assign6120_body5_e5793: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5794: f64 = (assign6120_body5_e5790 * assign6120_body5_e5793);
        let assign6120_body5_e5795: f64 = (assign6120_body5_e5785 + assign6120_body5_e5794);
        let assign6120_body5_e5796: f64 = (-assign6120_body5_e5795);
        let assign6120_body5_e5798: f64 = (assign6120_body5_e5796 / p.p481);
        let assign6120_body5_e5799: f64 = (-assign6120_body5_e5798);
        let assign6120_body5_e5801: f64 = (assign6120_body5_e5799 - 80.0);
        let assign6120_body5_e5807: f64 = (0.5 * p.p20);
        let assign6120_body5_e5808: f64 = (p.p27 + assign6120_body5_e5807);
        let assign6120_body5_e5811: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5813: f64 = (assign6120_body5_e5811 - var_iloop);
        let assign6120_body5_e5816: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5817: f64 = (assign6120_body5_e5813 * assign6120_body5_e5816);
        let assign6120_body5_e5818: f64 = (assign6120_body5_e5808 + assign6120_body5_e5817);
        let assign6120_body5_e5819: f64 = (-assign6120_body5_e5818);
        let assign6120_body5_e5821: f64 = (assign6120_body5_e5819 / p.p481);
        let assign6120_body5_e5822: f64 = (-assign6120_body5_e5821);
        let assign6120_body5_e5824: f64 = (assign6120_body5_e5822 - 80.0);
        let assign6120_body5_e5825: f64 = (0.5 * assign6120_body5_e5824);
        let assign6120_body5_e5830: f64 = (0.5 * p.p20);
        let assign6120_body5_e5831: f64 = (p.p27 + assign6120_body5_e5830);
        let assign6120_body5_e5834: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5836: f64 = (assign6120_body5_e5834 - var_iloop);
        let assign6120_body5_e5839: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5840: f64 = (assign6120_body5_e5836 * assign6120_body5_e5839);
        let assign6120_body5_e5841: f64 = (assign6120_body5_e5831 + assign6120_body5_e5840);
        let assign6120_body5_e5842: f64 = (-assign6120_body5_e5841);
        let assign6120_body5_e5844: f64 = (assign6120_body5_e5842 / p.p481);
        let assign6120_body5_e5845: f64 = (-assign6120_body5_e5844);
        let assign6120_body5_e5847: f64 = (assign6120_body5_e5845 - 80.0);
        let assign6120_body5_e5849: f64 = (assign6120_body5_e5847 * 0.3333333333333);
        let assign6120_body5_e5850: f64 = (1.0 + assign6120_body5_e5849);
        let assign6120_body5_e5851: f64 = (assign6120_body5_e5825 * assign6120_body5_e5850);
        let assign6120_body5_e5852: f64 = (1.0 + assign6120_body5_e5851);
        let assign6120_body5_e5853: f64 = (assign6120_body5_e5801 * assign6120_body5_e5852);
        let assign6120_body5_e5854: f64 = (1.0 + assign6120_body5_e5853);
        let assign6120_body5_e5855: f64 = (1.80485e-35 / assign6120_body5_e5854);
        (assign6120_body5_e5855, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
            var_temp2 = assign6120_body5_e5857;
            var_temp2_dn4 = assign6120_body5_e5857_d_n4;
            var_temp2_dn6 = assign6120_body5_e5857_d_n6;
            var_temp2_dn7 = assign6120_body5_e5857_d_n7;
            var_temp2_dn8 = assign6120_body5_e5857_d_n8;
            var_temp2_dn9 = assign6120_body5_e5857_d_n9;
            var_temp2_rv = 0.0;
            let (assign6120_body6_e5872, assign6120_body6_e5872_d_n4, assign6120_body6_e5872_d_n6, assign6120_body6_e5872_d_n7, assign6120_body6_e5872_d_n8, assign6120_body6_e5872_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body6_e5867: f64 = (1.0 - var_temp1);
        let assign6120_body6_e5869: f64 = (-p.p482);
        let assign6120_body6_e5870: f64 = (assign6120_body6_e5867).powf(assign6120_body6_e5869);
        (assign6120_body6_e5870, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn4))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn4) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn6))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn6) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn7))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn7) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn8))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn8) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn9))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn9) / assign6120_body6_e5867))) },)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
            var_temp3 = assign6120_body6_e5872;
            var_temp3_dn4 = assign6120_body6_e5872_d_n4;
            var_temp3_dn6 = assign6120_body6_e5872_d_n6;
            var_temp3_dn7 = assign6120_body6_e5872_d_n7;
            var_temp3_dn8 = assign6120_body6_e5872_d_n8;
            var_temp3_dn9 = assign6120_body6_e5872_d_n9;
            var_temp3_rv = 0.0;
            let (assign6120_body7_e5887, assign6120_body7_e5887_d_n4, assign6120_body7_e5887_d_n6, assign6120_body7_e5887_d_n7, assign6120_body7_e5887_d_n8, assign6120_body7_e5887_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body7_e5882: f64 = (1.0 - var_temp2);
        let assign6120_body7_e5884: f64 = (-p.p482);
        let assign6120_body7_e5885: f64 = (assign6120_body7_e5882).powf(assign6120_body7_e5884);
        (assign6120_body7_e5885, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn4))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn4) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn6))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn6) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn7))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn7) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn8))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn8) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn9))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn9) / assign6120_body7_e5882))) },)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
            var_temp4 = assign6120_body7_e5887;
            var_temp4_dn4 = assign6120_body7_e5887_d_n4;
            var_temp4_dn6 = assign6120_body7_e5887_d_n6;
            var_temp4_dn7 = assign6120_body7_e5887_d_n7;
            var_temp4_dn8 = assign6120_body7_e5887_d_n8;
            var_temp4_dn9 = assign6120_body7_e5887_d_n9;
            var_temp4_rv = 0.0;
            let (assign6120_body8_e5905, assign6120_body8_e5905_d_n4, assign6120_body8_e5905_d_n6, assign6120_body8_e5905_d_n7, assign6120_body8_e5905_d_n8, assign6120_body8_e5905_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body8_e5899: f64 = (var_temp3 + var_temp4);
        let assign6120_body8_e5900: f64 = (0.5 * assign6120_body8_e5899);
        let assign6120_body8_e5902: f64 = (assign6120_body8_e5900).powf(var_temp);
        let assign6120_body8_e5903: f64 = (var_tmpa + assign6120_body8_e5902);
        (assign6120_body8_e5903, (var_tmpa_dn4 + if var_temp_dn4 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn4 + var_temp4_dn4)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn4 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn4 + var_temp4_dn4)) / assign6120_body8_e5900)))) }), (var_tmpa_dn6 + if var_temp_dn6 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn6 + var_temp4_dn6)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn6 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn6 + var_temp4_dn6)) / assign6120_body8_e5900)))) }), (var_tmpa_dn7 + if var_temp_dn7 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn7 + var_temp4_dn7)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn7 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn7 + var_temp4_dn7)) / assign6120_body8_e5900)))) }), (var_tmpa_dn8 + if var_temp_dn8 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn8 + var_temp4_dn8)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn8 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn8 + var_temp4_dn8)) / assign6120_body8_e5900)))) }), (var_tmpa_dn9 + if var_temp_dn9 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn9 + var_temp4_dn9)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn9 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn9 + var_temp4_dn9)) / assign6120_body8_e5900)))) }),)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
            var_tmpa = assign6120_body8_e5905;
            var_tmpa_dn4 = assign6120_body8_e5905_d_n4;
            var_tmpa_dn6 = assign6120_body8_e5905_d_n6;
            var_tmpa_dn7 = assign6120_body8_e5905_d_n7;
            var_tmpa_dn8 = assign6120_body8_e5905_d_n8;
            var_tmpa_dn9 = assign6120_body8_e5905_d_n9;
            var_tmpa_rv = 0.0;
            let (assign6120_body9_e5917,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body9_e5915: f64 = (var_iloop + 1.0);
        (assign6120_body9_e5915,)
    } else {
        (var_iloop,)
    }
};
            var_iloop = assign6120_body9_e5917;
            var_iloop_rv = 0.0;
        }

        let (assign6130_e5931, assign6130_e5931_d_n4, assign6130_e5931_d_n6, assign6130_e5931_d_n7, assign6130_e5931_d_n8, assign6130_e5931_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6130_e5928: f64 = (var_tmpa / p.p29);
        let assign6130_e5929: f64 = (1.0 - assign6130_e5928);
        (assign6130_e5929, (-(var_tmpa_dn4 / p.p29)), (-(var_tmpa_dn6 / p.p29)), (-(var_tmpa_dn7 / p.p29)), (-(var_tmpa_dn8 / p.p29)), (-(var_tmpa_dn9 / p.p29)),)
    } else {
        (var_str_g, var_str_g_dn4, var_str_g_dn6, var_str_g_dn7, var_str_g_dn8, var_str_g_dn9,)
    }
};
        var_str_g = assign6130_e5931;
        var_str_g_dn4 = assign6130_e5931_d_n4;
        var_str_g_dn6 = assign6130_e5931_d_n6;
        var_str_g_dn7 = assign6130_e5931_d_n7;
        var_str_g_dn8 = assign6130_e5931_d_n8;
        var_str_g_dn9 = assign6130_e5931_d_n9;
        var_str_g_rv = 0.0;

        let assign6140_e5935: f64 = (0.5 * p.p20);
        let assign6140_e5936: f64 = (p.p462 + assign6140_e5935);
        let assign6140_e5937: f64 = (-assign6140_e5936);
        let assign6140_e5939: f64 = (assign6140_e5937 / p.p481);
        let assign6140_e5941: f64 = (-80.0);
        let assign6140_e5942: f64 = if assign6140_e5939 > assign6140_e5941 { 1.0 } else { 0.0 };
        var_guard137 = assign6140_e5942;
        var_guard137_rv = 0.0;

        let (assign6150_e5962, assign6150_e5962_d_n4, assign6150_e5962_d_n6, assign6150_e5962_d_n7, assign6150_e5962_d_n8, assign6150_e5962_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard137 != 0.0)) {
        let assign6150_e5955: f64 = (0.5 * p.p20);
        let assign6150_e5956: f64 = (p.p462 + assign6150_e5955);
        let assign6150_e5957: f64 = (-assign6150_e5956);
        let assign6150_e5959: f64 = (assign6150_e5957 / p.p481);
        let assign6150_e5960: f64 = (assign6150_e5959).exp();
        (assign6150_e5960, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign6150_e5962;
        var_temp1_dn4 = assign6150_e5962_d_n4;
        var_temp1_dn6 = assign6150_e5962_d_n6;
        var_temp1_dn7 = assign6150_e5962_d_n7;
        var_temp1_dn8 = assign6150_e5962_d_n8;
        var_temp1_dn9 = assign6150_e5962_d_n9;
        var_temp1_rv = 0.0;

        let (assign6160_e6021, assign6160_e6021_d_n4, assign6160_e6021_d_n6, assign6160_e6021_d_n7, assign6160_e6021_d_n8, assign6160_e6021_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard137 == 0.0)) {
        let assign6160_e5978: f64 = (0.5 * p.p20);
        let assign6160_e5979: f64 = (p.p462 + assign6160_e5978);
        let assign6160_e5980: f64 = (-assign6160_e5979);
        let assign6160_e5982: f64 = (assign6160_e5980 / p.p481);
        let assign6160_e5983: f64 = (-assign6160_e5982);
        let assign6160_e5985: f64 = (assign6160_e5983 - 80.0);
        let assign6160_e5991: f64 = (0.5 * p.p20);
        let assign6160_e5992: f64 = (p.p462 + assign6160_e5991);
        let assign6160_e5993: f64 = (-assign6160_e5992);
        let assign6160_e5995: f64 = (assign6160_e5993 / p.p481);
        let assign6160_e5996: f64 = (-assign6160_e5995);
        let assign6160_e5998: f64 = (assign6160_e5996 - 80.0);
        let assign6160_e5999: f64 = (0.5 * assign6160_e5998);
        let assign6160_e6004: f64 = (0.5 * p.p20);
        let assign6160_e6005: f64 = (p.p462 + assign6160_e6004);
        let assign6160_e6006: f64 = (-assign6160_e6005);
        let assign6160_e6008: f64 = (assign6160_e6006 / p.p481);
        let assign6160_e6009: f64 = (-assign6160_e6008);
        let assign6160_e6011: f64 = (assign6160_e6009 - 80.0);
        let assign6160_e6013: f64 = (assign6160_e6011 * 0.3333333333333);
        let assign6160_e6014: f64 = (1.0 + assign6160_e6013);
        let assign6160_e6015: f64 = (assign6160_e5999 * assign6160_e6014);
        let assign6160_e6016: f64 = (1.0 + assign6160_e6015);
        let assign6160_e6017: f64 = (assign6160_e5985 * assign6160_e6016);
        let assign6160_e6018: f64 = (1.0 + assign6160_e6017);
        let assign6160_e6019: f64 = (1.80485e-35 / assign6160_e6018);
        (assign6160_e6019, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign6160_e6021;
        var_temp1_dn4 = assign6160_e6021_d_n4;
        var_temp1_dn6 = assign6160_e6021_d_n6;
        var_temp1_dn7 = assign6160_e6021_d_n7;
        var_temp1_dn8 = assign6160_e6021_d_n8;
        var_temp1_dn9 = assign6160_e6021_d_n9;
        var_temp1_rv = 0.0;

        let assign6170_e6025: f64 = (0.5 * p.p20);
        let assign6170_e6026: f64 = (p.p463 + assign6170_e6025);
        let assign6170_e6027: f64 = (-assign6170_e6026);
        let assign6170_e6029: f64 = (assign6170_e6027 / p.p481);
        let assign6170_e6031: f64 = (-80.0);
        let assign6170_e6032: f64 = if assign6170_e6029 > assign6170_e6031 { 1.0 } else { 0.0 };
        var_guard138 = assign6170_e6032;
        var_guard138_rv = 0.0;

        let (assign6180_e6052, assign6180_e6052_d_n4, assign6180_e6052_d_n6, assign6180_e6052_d_n7, assign6180_e6052_d_n8, assign6180_e6052_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard138 != 0.0)) {
        let assign6180_e6045: f64 = (0.5 * p.p20);
        let assign6180_e6046: f64 = (p.p463 + assign6180_e6045);
        let assign6180_e6047: f64 = (-assign6180_e6046);
        let assign6180_e6049: f64 = (assign6180_e6047 / p.p481);
        let assign6180_e6050: f64 = (assign6180_e6049).exp();
        (assign6180_e6050, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign6180_e6052;
        var_temp2_dn4 = assign6180_e6052_d_n4;
        var_temp2_dn6 = assign6180_e6052_d_n6;
        var_temp2_dn7 = assign6180_e6052_d_n7;
        var_temp2_dn8 = assign6180_e6052_d_n8;
        var_temp2_dn9 = assign6180_e6052_d_n9;
        var_temp2_rv = 0.0;

        let (assign6190_e6111, assign6190_e6111_d_n4, assign6190_e6111_d_n6, assign6190_e6111_d_n7, assign6190_e6111_d_n8, assign6190_e6111_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard138 == 0.0)) {
        let assign6190_e6068: f64 = (0.5 * p.p20);
        let assign6190_e6069: f64 = (p.p463 + assign6190_e6068);
        let assign6190_e6070: f64 = (-assign6190_e6069);
        let assign6190_e6072: f64 = (assign6190_e6070 / p.p481);
        let assign6190_e6073: f64 = (-assign6190_e6072);
        let assign6190_e6075: f64 = (assign6190_e6073 - 80.0);
        let assign6190_e6081: f64 = (0.5 * p.p20);
        let assign6190_e6082: f64 = (p.p463 + assign6190_e6081);
        let assign6190_e6083: f64 = (-assign6190_e6082);
        let assign6190_e6085: f64 = (assign6190_e6083 / p.p481);
        let assign6190_e6086: f64 = (-assign6190_e6085);
        let assign6190_e6088: f64 = (assign6190_e6086 - 80.0);
        let assign6190_e6089: f64 = (0.5 * assign6190_e6088);
        let assign6190_e6094: f64 = (0.5 * p.p20);
        let assign6190_e6095: f64 = (p.p463 + assign6190_e6094);
        let assign6190_e6096: f64 = (-assign6190_e6095);
        let assign6190_e6098: f64 = (assign6190_e6096 / p.p481);
        let assign6190_e6099: f64 = (-assign6190_e6098);
        let assign6190_e6101: f64 = (assign6190_e6099 - 80.0);
        let assign6190_e6103: f64 = (assign6190_e6101 * 0.3333333333333);
        let assign6190_e6104: f64 = (1.0 + assign6190_e6103);
        let assign6190_e6105: f64 = (assign6190_e6089 * assign6190_e6104);
        let assign6190_e6106: f64 = (1.0 + assign6190_e6105);
        let assign6190_e6107: f64 = (assign6190_e6075 * assign6190_e6106);
        let assign6190_e6108: f64 = (1.0 + assign6190_e6107);
        let assign6190_e6109: f64 = (1.80485e-35 / assign6190_e6108);
        (assign6190_e6109, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign6190_e6111;
        var_temp2_dn4 = assign6190_e6111_d_n4;
        var_temp2_dn6 = assign6190_e6111_d_n6;
        var_temp2_dn7 = assign6190_e6111_d_n7;
        var_temp2_dn8 = assign6190_e6111_d_n8;
        var_temp2_dn9 = assign6190_e6111_d_n9;
        var_temp2_rv = 0.0;

        let (assign6200_e6126, assign6200_e6126_d_n4, assign6200_e6126_d_n6, assign6200_e6126_d_n7, assign6200_e6126_d_n8, assign6200_e6126_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6200_e6121: f64 = (1.0 - var_temp1);
        let assign6200_e6123: f64 = (-p.p482);
        let assign6200_e6124: f64 = (assign6200_e6121).powf(assign6200_e6123);
        (assign6200_e6124, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn4))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn4) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn6))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn6) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn7))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn7) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn8))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn8) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn9))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn9) / assign6200_e6121))) },)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign6200_e6126;
        var_temp3_dn4 = assign6200_e6126_d_n4;
        var_temp3_dn6 = assign6200_e6126_d_n6;
        var_temp3_dn7 = assign6200_e6126_d_n7;
        var_temp3_dn8 = assign6200_e6126_d_n8;
        var_temp3_dn9 = assign6200_e6126_d_n9;
        var_temp3_rv = 0.0;

        let (assign6210_e6141, assign6210_e6141_d_n4, assign6210_e6141_d_n6, assign6210_e6141_d_n7, assign6210_e6141_d_n8, assign6210_e6141_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6210_e6136: f64 = (1.0 - var_temp2);
        let assign6210_e6138: f64 = (-p.p482);
        let assign6210_e6139: f64 = (assign6210_e6136).powf(assign6210_e6138);
        (assign6210_e6139, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn4))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn4) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn6))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn6) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn7))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn7) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn8))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn8) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn9))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn9) / assign6210_e6136))) },)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign6210_e6141;
        var_temp4_dn4 = assign6210_e6141_d_n4;
        var_temp4_dn6 = assign6210_e6141_d_n6;
        var_temp4_dn7 = assign6210_e6141_d_n7;
        var_temp4_dn8 = assign6210_e6141_d_n8;
        var_temp4_dn9 = assign6210_e6141_d_n9;
        var_temp4_rv = 0.0;

        let (assign6220_e6159, assign6220_e6159_d_n4, assign6220_e6159_d_n6, assign6220_e6159_d_n7, assign6220_e6159_d_n8, assign6220_e6159_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6220_e6153: f64 = (var_temp3 + var_temp4);
        let assign6220_e6154: f64 = (0.5 * assign6220_e6153);
        let assign6220_e6156: f64 = (assign6220_e6154).powf(var_temp);
        let assign6220_e6157: f64 = (1.0 - assign6220_e6156);
        (assign6220_e6157, (-if var_temp_dn4 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn4 + var_temp4_dn4)))) } } else { (assign6220_e6156 * ((var_temp_dn4 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn4 + var_temp4_dn4)) / assign6220_e6154)))) }), (-if var_temp_dn6 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn6 + var_temp4_dn6)))) } } else { (assign6220_e6156 * ((var_temp_dn6 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn6 + var_temp4_dn6)) / assign6220_e6154)))) }), (-if var_temp_dn7 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn7 + var_temp4_dn7)))) } } else { (assign6220_e6156 * ((var_temp_dn7 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn7 + var_temp4_dn7)) / assign6220_e6154)))) }), (-if var_temp_dn8 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn8 + var_temp4_dn8)))) } } else { (assign6220_e6156 * ((var_temp_dn8 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn8 + var_temp4_dn8)) / assign6220_e6154)))) }), (-if var_temp_dn9 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn9 + var_temp4_dn9)))) } } else { (assign6220_e6156 * ((var_temp_dn9 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn9 + var_temp4_dn9)) / assign6220_e6154)))) }),)
    } else {
        (var_str_gref, var_str_gref_dn4, var_str_gref_dn6, var_str_gref_dn7, var_str_gref_dn8, var_str_gref_dn9,)
    }
};
        var_str_gref = assign6220_e6159;
        var_str_gref_dn4 = assign6220_e6159_d_n4;
        var_str_gref_dn6 = assign6220_e6159_d_n6;
        var_str_gref_dn7 = assign6220_e6159_d_n7;
        var_str_gref_dn8 = assign6220_e6159_d_n8;
        var_str_gref_dn9 = assign6220_e6159_d_n9;
        var_str_gref_rv = 0.0;

        let (assign6230_e6175,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6230_e6169: f64 = (var_w_i + var_delwod);
        let assign6230_e6171: f64 = (assign6230_e6169 + p.p464);
        let assign6230_e6173: f64 = (assign6230_e6171).max(1e-9);
        (assign6230_e6173,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign6230_e6175;
        var_wx_rv = 0.0;

        let (assign6240_e6193, assign6240_e6193_d_n4, assign6240_e6193_d_n6, assign6240_e6193_d_n7, assign6240_e6193_d_n8, assign6240_e6193_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6240_e6188: f64 = (var_rt - 1.0);
        let assign6240_e6189: f64 = (p.p487 * assign6240_e6188);
        let assign6240_e6190: f64 = (1.0 + assign6240_e6189);
        let assign6240_e6191: f64 = (p.p486 / assign6240_e6190);
        (assign6240_e6191, (-((p.p486 * (p.p487 * var_rt_dn4)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn6)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn7)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn8)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn9)) / (assign6240_e6190 * assign6240_e6190))),)
    } else {
        (var_ruo, var_ruo_dn4, var_ruo_dn6, var_ruo_dn7, var_ruo_dn8, var_ruo_dn9,)
    }
};
        var_ruo = assign6240_e6193;
        var_ruo_dn4 = assign6240_e6193_d_n4;
        var_ruo_dn6 = assign6240_e6193_d_n6;
        var_ruo_dn7 = assign6240_e6193_d_n7;
        var_ruo_dn8 = assign6240_e6193_d_n8;
        var_ruo_dn9 = assign6240_e6193_d_n9;
        var_ruo_rv = 0.0;

        let (assign6250_e6205, assign6250_e6205_d_n4, assign6250_e6205_d_n6, assign6250_e6205_d_n7, assign6250_e6205_d_n8, assign6250_e6205_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6250_e6203: f64 = (var_ruo * var_str_g);
        (assign6250_e6203, ((var_ruo_dn4 * var_str_g) + (var_ruo * var_str_g_dn4)), ((var_ruo_dn6 * var_str_g) + (var_ruo * var_str_g_dn6)), ((var_ruo_dn7 * var_str_g) + (var_ruo * var_str_g_dn7)), ((var_ruo_dn8 * var_str_g) + (var_ruo * var_str_g_dn8)), ((var_ruo_dn9 * var_str_g) + (var_ruo * var_str_g_dn9)),)
    } else {
        (var_rhobeta, var_rhobeta_dn4, var_rhobeta_dn6, var_rhobeta_dn7, var_rhobeta_dn8, var_rhobeta_dn9,)
    }
};
        var_rhobeta = assign6250_e6205;
        var_rhobeta_dn4 = assign6250_e6205_d_n4;
        var_rhobeta_dn6 = assign6250_e6205_d_n6;
        var_rhobeta_dn7 = assign6250_e6205_d_n7;
        var_rhobeta_dn8 = assign6250_e6205_d_n8;
        var_rhobeta_dn9 = assign6250_e6205_d_n9;
        var_rhobeta_rv = 0.0;

        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_iloop_slot = var_iloop;
        *var_iloop_rv_slot = var_iloop_rv;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_dn4_slot = var_rhobeta_dn4;
        *var_rhobeta_dn6_slot = var_rhobeta_dn6;
        *var_rhobeta_dn7_slot = var_rhobeta_dn7;
        *var_rhobeta_dn8_slot = var_rhobeta_dn8;
        *var_rhobeta_dn9_slot = var_rhobeta_dn9;
        *var_rhobeta_rv_slot = var_rhobeta_rv;
        *var_ruo_slot = var_ruo;
        *var_ruo_dn4_slot = var_ruo_dn4;
        *var_ruo_dn6_slot = var_ruo_dn6;
        *var_ruo_dn7_slot = var_ruo_dn7;
        *var_ruo_dn8_slot = var_ruo_dn8;
        *var_ruo_dn9_slot = var_ruo_dn9;
        *var_ruo_rv_slot = var_ruo_rv;
        *var_str_g_slot = var_str_g;
        *var_str_g_dn4_slot = var_str_g_dn4;
        *var_str_g_dn6_slot = var_str_g_dn6;
        *var_str_g_dn7_slot = var_str_g_dn7;
        *var_str_g_dn8_slot = var_str_g_dn8;
        *var_str_g_dn9_slot = var_str_g_dn9;
        *var_str_g_rv_slot = var_str_g_rv;
        *var_str_gref_slot = var_str_gref;
        *var_str_gref_dn4_slot = var_str_gref_dn4;
        *var_str_gref_dn6_slot = var_str_gref_dn6;
        *var_str_gref_dn7_slot = var_str_gref_dn7;
        *var_str_gref_dn8_slot = var_str_gref_dn8;
        *var_str_gref_dn9_slot = var_str_gref_dn9;
        *var_str_gref_rv_slot = var_str_gref_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp3_rv_slot = var_temp3_rv;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp4_rv_slot = var_temp4_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_wx_slot = var_wx;
        *var_wx_rv_slot = var_wx_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_agidl_i: f64,
        var_agidl_i_dn4: f64,
        var_agidl_i_dn6: f64,
        var_agidl_i_dn7: f64,
        var_agidl_i_dn8: f64,
        var_agidl_i_dn9: f64,
        var_bgidl_t: f64,
        var_cgidl_i: f64,
        var_cov_i: f64,
        var_cov_i_dn4: f64,
        var_cov_i_dn6: f64,
        var_cov_i_dn7: f64,
        var_cov_i_dn8: f64,
        var_cov_i_dn9: f64,
        var_dgidl_i: f64,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_igovacc_t: f64,
        var_igovinv_t: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_nov_i: f64,
        var_rhobeta: f64,
        var_rhobeta_dn4: f64,
        var_rhobeta_dn6: f64,
        var_rhobeta_dn7: f64,
        var_rhobeta_dn8: f64,
        var_rhobeta_dn9: f64,
        var_ruo: f64,
        var_ruo_dn4: f64,
        var_ruo_dn6: f64,
        var_ruo_dn7: f64,
        var_ruo_dn8: f64,
        var_ruo_dn9: f64,
        var_stbgidl_i: f64,
        var_str_g: f64,
        var_str_g_dn4: f64,
        var_str_g_dn6: f64,
        var_str_g_dn7: f64,
        var_str_g_dn8: f64,
        var_str_g_dn9: f64,
        var_str_gref: f64,
        var_str_gref_dn4: f64,
        var_str_gref_dn6: f64,
        var_str_gref_dn7: f64,
        var_str_gref_dn8: f64,
        var_str_gref_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_wen: f64,
        var_wx: f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn1_t_rv_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn2_t_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidld_t_rv_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf1_t_rv_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf2_t_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac1_t_rv_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac2_t_rv_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfac_p_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covd_i_rv_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_dgidld_i_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovaccd_t_rv_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_igovinvd_t_rv_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kstressvth0_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_dn4_slot: &mut f64,
        var_rhobetaref_dn6_slot: &mut f64,
        var_rhobetaref_dn7_slot: &mut f64,
        var_rhobetaref_dn8_slot: &mut f64,
        var_rhobetaref_dn9_slot: &mut f64,
        var_rhobetaref_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp0__blk79_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb1_t_rv_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfb2_t_rv_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac1_t_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
    ) {
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn1_t_rv: f64 = *var_betn1_t_rv_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn2_t_rv: f64 = *var_betn2_t_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidld_t_rv: f64 = *var_bgidld_t_rv_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf1_t_rv: f64 = *var_cf1_t_rv_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf2_t_rv: f64 = *var_cf2_t_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac1_t_rv: f64 = *var_cfac1_t_rv_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac2_t_rv: f64 = *var_cfac2_t_rv_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfac_p_rv: f64 = *var_cfac_p_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covd_i_rv: f64 = *var_covd_i_rv_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_dgidld_i_rv: f64 = *var_dgidld_i_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovaccd_t_rv: f64 = *var_igovaccd_t_rv_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_igovinvd_t_rv: f64 = *var_igovinvd_t_rv_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kstressvth0_rv: f64 = *var_kstressvth0_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_dn4: f64 = *var_rhobetaref_dn4_slot;
        let mut var_rhobetaref_dn6: f64 = *var_rhobetaref_dn6_slot;
        let mut var_rhobetaref_dn7: f64 = *var_rhobetaref_dn7_slot;
        let mut var_rhobetaref_dn8: f64 = *var_rhobetaref_dn8_slot;
        let mut var_rhobetaref_dn9: f64 = *var_rhobetaref_dn9_slot;
        let mut var_rhobetaref_rv: f64 = *var_rhobetaref_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp0__blk79_rv: f64 = *var_temp0__blk79_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb1_t_rv: f64 = *var_vfb1_t_rv_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfb2_t_rv: f64 = *var_vfb2_t_rv_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac1_t_rv: f64 = *var_vfbac1_t_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;

        let (assign6260_e6217, assign6260_e6217_d_n4, assign6260_e6217_d_n6, assign6260_e6217_d_n7, assign6260_e6217_d_n8, assign6260_e6217_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6260_e6215: f64 = (var_ruo * var_str_gref);
        (assign6260_e6215, ((var_ruo_dn4 * var_str_gref) + (var_ruo * var_str_gref_dn4)), ((var_ruo_dn6 * var_str_gref) + (var_ruo * var_str_gref_dn6)), ((var_ruo_dn7 * var_str_gref) + (var_ruo * var_str_gref_dn7)), ((var_ruo_dn8 * var_str_gref) + (var_ruo * var_str_gref_dn8)), ((var_ruo_dn9 * var_str_gref) + (var_ruo * var_str_gref_dn9)),)
    } else {
        (var_rhobetaref, var_rhobetaref_dn4, var_rhobetaref_dn6, var_rhobetaref_dn7, var_rhobetaref_dn8, var_rhobetaref_dn9,)
    }
};
        var_rhobetaref = assign6260_e6217;
        var_rhobetaref_dn4 = assign6260_e6217_d_n4;
        var_rhobetaref_dn6 = assign6260_e6217_d_n6;
        var_rhobetaref_dn7 = assign6260_e6217_d_n7;
        var_rhobetaref_dn8 = assign6260_e6217_d_n8;
        var_rhobetaref_dn9 = assign6260_e6217_d_n9;
        var_rhobetaref_rv = 0.0;

        let (assign6270_e6229, assign6270_e6229_d_n4, assign6270_e6229_d_n6, assign6270_e6229_d_n7, assign6270_e6229_d_n8, assign6270_e6229_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6270_e6227: f64 = (var_str_g - var_str_gref);
        (assign6270_e6227, (var_str_g_dn4 - var_str_gref_dn4), (var_str_g_dn6 - var_str_gref_dn6), (var_str_g_dn7 - var_str_gref_dn7), (var_str_g_dn8 - var_str_gref_dn8), (var_str_g_dn9 - var_str_gref_dn9),)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign6270_e6229;
        var_temp0__blk79_dn4 = assign6270_e6229_d_n4;
        var_temp0__blk79_dn6 = assign6270_e6229_d_n6;
        var_temp0__blk79_dn7 = assign6270_e6229_d_n7;
        var_temp0__blk79_dn8 = assign6270_e6229_d_n8;
        var_temp0__blk79_dn9 = assign6270_e6229_d_n9;
        var_temp0__blk79_rv = 0.0;

        let (assign6280_e6247,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6280_e6240: f64 = (p.p484 * var_wx);
        let assign6280_e6242: f64 = (assign6280_e6240 / var_wen);
        let assign6280_e6243: f64 = (1.0 + assign6280_e6242);
        let assign6280_e6245: f64 = (assign6280_e6243).max(1e-20);
        (assign6280_e6245,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign6280_e6247;
        var_kstressvth0_rv = 0.0;

        let (assign6290_e6265, assign6290_e6265_d_n4, assign6290_e6265_d_n6, assign6290_e6265_d_n7, assign6290_e6265_d_n8, assign6290_e6265_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6290_e6258: f64 = (1.0 + var_rhobeta);
        let assign6290_e6259: f64 = (var_betn_p * assign6290_e6258);
        let assign6290_e6262: f64 = (1.0 + var_rhobetaref);
        let assign6290_e6263: f64 = (assign6290_e6259 / assign6290_e6262);
        (assign6290_e6263, (((((var_betn_p_dn4 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn4)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn4)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn6 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn6)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn6)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn7 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn7)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn7)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn8 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn8)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn8)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn9 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn9)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn9)) / (assign6290_e6262 * assign6290_e6262)),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign6290_e6265;
        var_betn_p_dn4 = assign6290_e6265_d_n4;
        var_betn_p_dn6 = assign6290_e6265_d_n6;
        var_betn_p_dn7 = assign6290_e6265_d_n7;
        var_betn_p_dn8 = assign6290_e6265_d_n8;
        var_betn_p_dn9 = assign6290_e6265_d_n9;
        var_betn_p_rv = 0.0;

        let (assign6300_e6277, assign6300_e6277_d_n4, assign6300_e6277_d_n6, assign6300_e6277_d_n7, assign6300_e6277_d_n8, assign6300_e6277_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6300_e6275: f64 = (var_betn_p).max(1e-10);
        (assign6300_e6275, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign6300_e6277;
        var_betn1_t_dn4 = assign6300_e6277_d_n4;
        var_betn1_t_dn6 = assign6300_e6277_d_n6;
        var_betn1_t_dn7 = assign6300_e6277_d_n7;
        var_betn1_t_dn8 = assign6300_e6277_d_n8;
        var_betn1_t_dn9 = assign6300_e6277_d_n9;
        var_betn1_t_rv = 0.0;

        let (assign6310_e6289, assign6310_e6289_d_n4, assign6310_e6289_d_n6, assign6310_e6289_d_n7, assign6310_e6289_d_n8, assign6310_e6289_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6310_e6287: f64 = (p.p254 * var_betn1_t);
        (assign6310_e6287, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign6310_e6289;
        var_betn2_t_dn4 = assign6310_e6289_d_n4;
        var_betn2_t_dn6 = assign6310_e6289_d_n6;
        var_betn2_t_dn7 = assign6310_e6289_d_n7;
        var_betn2_t_dn8 = assign6310_e6289_d_n8;
        var_betn2_t_dn9 = assign6310_e6289_d_n9;
        var_betn2_t_rv = 0.0;

        let (assign6320_e6317, assign6320_e6317_d_n4, assign6320_e6317_d_n6, assign6320_e6317_d_n7, assign6320_e6317_d_n8, assign6320_e6317_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6320_e6299: f64 = (1.0 + var_rhobeta);
        let assign6320_e6303: f64 = (p.p488 * var_rhobetaref);
        let assign6320_e6304: f64 = (1.0 + assign6320_e6303);
        let assign6320_e6305: f64 = (assign6320_e6299 * assign6320_e6304);
        let assign6320_e6308: f64 = (1.0 + var_rhobetaref);
        let assign6320_e6312: f64 = (p.p488 * var_rhobeta);
        let assign6320_e6313: f64 = (1.0 + assign6320_e6312);
        let assign6320_e6314: f64 = (assign6320_e6308 * assign6320_e6313);
        let assign6320_e6315: f64 = (assign6320_e6305 / assign6320_e6314);
        (assign6320_e6315, (((((var_rhobeta_dn4 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn4))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn4 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn4))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn6 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn6))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn6 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn6))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn7 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn7))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn7 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn7))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn8 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn8))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn8 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn8))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn9 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn9))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn9 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn9))))) / (assign6320_e6314 * assign6320_e6314)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6320_e6317;
        var_temp_dn4 = assign6320_e6317_d_n4;
        var_temp_dn6 = assign6320_e6317_d_n6;
        var_temp_dn7 = assign6320_e6317_d_n7;
        var_temp_dn8 = assign6320_e6317_d_n8;
        var_temp_dn9 = assign6320_e6317_d_n9;
        var_temp_rv = 0.0;

        let (assign6330_e6329, assign6330_e6329_d_n4, assign6330_e6329_d_n6, assign6330_e6329_d_n7, assign6330_e6329_d_n8, assign6330_e6329_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6330_e6327: f64 = (var_thesat_p * var_temp);
        (assign6330_e6327, ((var_thesat_p_dn4 * var_temp) + (var_thesat_p * var_temp_dn4)), ((var_thesat_p_dn6 * var_temp) + (var_thesat_p * var_temp_dn6)), ((var_thesat_p_dn7 * var_temp) + (var_thesat_p * var_temp_dn7)), ((var_thesat_p_dn8 * var_temp) + (var_thesat_p * var_temp_dn8)), ((var_thesat_p_dn9 * var_temp) + (var_thesat_p * var_temp_dn9)),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign6330_e6329;
        var_thesat_p_dn4 = assign6330_e6329_d_n4;
        var_thesat_p_dn6 = assign6330_e6329_d_n6;
        var_thesat_p_dn7 = assign6330_e6329_d_n7;
        var_thesat_p_dn8 = assign6330_e6329_d_n8;
        var_thesat_p_dn9 = assign6330_e6329_d_n9;
        var_thesat_p_rv = 0.0;

        let (assign6340_e6341, assign6340_e6341_d_n4, assign6340_e6341_d_n6, assign6340_e6341_d_n7, assign6340_e6341_d_n8, assign6340_e6341_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6340_e6339: f64 = (var_thesat_p).max(0.0);
        (assign6340_e6339, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign6340_e6341;
        var_thesat_t_dn4 = assign6340_e6341_d_n4;
        var_thesat_t_dn6 = assign6340_e6341_d_n6;
        var_thesat_t_dn7 = assign6340_e6341_d_n7;
        var_thesat_t_dn8 = assign6340_e6341_d_n8;
        var_thesat_t_dn9 = assign6340_e6341_d_n9;
        var_thesat_t_rv = 0.0;

        let (assign6350_e6353, assign6350_e6353_d_n4, assign6350_e6353_d_n6, assign6350_e6353_d_n7, assign6350_e6353_d_n8, assign6350_e6353_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6350_e6351: f64 = (var_thesatac_p * var_temp);
        (assign6350_e6351, ((var_thesatac_p_dn4 * var_temp) + (var_thesatac_p * var_temp_dn4)), ((var_thesatac_p_dn6 * var_temp) + (var_thesatac_p * var_temp_dn6)), ((var_thesatac_p_dn7 * var_temp) + (var_thesatac_p * var_temp_dn7)), ((var_thesatac_p_dn8 * var_temp) + (var_thesatac_p * var_temp_dn8)), ((var_thesatac_p_dn9 * var_temp) + (var_thesatac_p * var_temp_dn9)),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign6350_e6353;
        var_thesatac_p_dn4 = assign6350_e6353_d_n4;
        var_thesatac_p_dn6 = assign6350_e6353_d_n6;
        var_thesatac_p_dn7 = assign6350_e6353_d_n7;
        var_thesatac_p_dn8 = assign6350_e6353_d_n8;
        var_thesatac_p_dn9 = assign6350_e6353_d_n9;
        var_thesatac_p_rv = 0.0;

        let (assign6360_e6365, assign6360_e6365_d_n4, assign6360_e6365_d_n6, assign6360_e6365_d_n7, assign6360_e6365_d_n8, assign6360_e6365_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6360_e6363: f64 = (var_thesatac_p).max(0.0);
        (assign6360_e6363, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign6360_e6365;
        var_thesatac_t_dn4 = assign6360_e6365_d_n4;
        var_thesatac_t_dn6 = assign6360_e6365_d_n6;
        var_thesatac_t_dn7 = assign6360_e6365_d_n7;
        var_thesatac_t_dn8 = assign6360_e6365_d_n8;
        var_thesatac_t_dn9 = assign6360_e6365_d_n9;
        var_thesatac_t_rv = 0.0;

        let (assign6370_e6379, assign6370_e6379_d_n4, assign6370_e6379_d_n6, assign6370_e6379_d_n7, assign6370_e6379_d_n8, assign6370_e6379_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6370_e6375: f64 = (p.p483 * var_temp0__blk79);
        let assign6370_e6377: f64 = (assign6370_e6375 / var_kstressvth0);
        (assign6370_e6377, ((p.p483 * var_temp0__blk79_dn4) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn6) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn7) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn8) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn9) / var_kstressvth0),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6370_e6379;
        var_temp_dn4 = assign6370_e6379_d_n4;
        var_temp_dn6 = assign6370_e6379_d_n6;
        var_temp_dn7 = assign6370_e6379_d_n7;
        var_temp_dn8 = assign6370_e6379_d_n8;
        var_temp_dn9 = assign6370_e6379_d_n9;
        var_temp_rv = 0.0;

        let (assign6380_e6391, assign6380_e6391_d_n4, assign6380_e6391_d_n6, assign6380_e6391_d_n7, assign6380_e6391_d_n8, assign6380_e6391_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6380_e6389: f64 = (var_vfb1_t + var_temp);
        (assign6380_e6389, (var_vfb1_t_dn4 + var_temp_dn4), (var_vfb1_t_dn6 + var_temp_dn6), (var_vfb1_t_dn7 + var_temp_dn7), (var_vfb1_t_dn8 + var_temp_dn8), (var_vfb1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign6380_e6391;
        var_vfb1_t_dn4 = assign6380_e6391_d_n4;
        var_vfb1_t_dn6 = assign6380_e6391_d_n6;
        var_vfb1_t_dn7 = assign6380_e6391_d_n7;
        var_vfb1_t_dn8 = assign6380_e6391_d_n8;
        var_vfb1_t_dn9 = assign6380_e6391_d_n9;
        var_vfb1_t_rv = 0.0;

        let (assign6390_e6403, assign6390_e6403_d_n4, assign6390_e6403_d_n6, assign6390_e6403_d_n7, assign6390_e6403_d_n8, assign6390_e6403_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6390_e6401: f64 = (var_vfb2_t + var_temp);
        (assign6390_e6401, (var_vfb2_t_dn4 + var_temp_dn4), (var_vfb2_t_dn6 + var_temp_dn6), (var_vfb2_t_dn7 + var_temp_dn7), (var_vfb2_t_dn8 + var_temp_dn8), (var_vfb2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign6390_e6403;
        var_vfb2_t_dn4 = assign6390_e6403_d_n4;
        var_vfb2_t_dn6 = assign6390_e6403_d_n6;
        var_vfb2_t_dn7 = assign6390_e6403_d_n7;
        var_vfb2_t_dn8 = assign6390_e6403_d_n8;
        var_vfb2_t_dn9 = assign6390_e6403_d_n9;
        var_vfb2_t_rv = 0.0;

        let (assign6400_e6415, assign6400_e6415_d_n4, assign6400_e6415_d_n6, assign6400_e6415_d_n7, assign6400_e6415_d_n8, assign6400_e6415_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6400_e6413: f64 = (var_vfbac1_t + var_temp);
        (assign6400_e6413, (var_vfbac1_t_dn4 + var_temp_dn4), (var_vfbac1_t_dn6 + var_temp_dn6), (var_vfbac1_t_dn7 + var_temp_dn7), (var_vfbac1_t_dn8 + var_temp_dn8), (var_vfbac1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign6400_e6415;
        var_vfbac1_t_dn4 = assign6400_e6415_d_n4;
        var_vfbac1_t_dn6 = assign6400_e6415_d_n6;
        var_vfbac1_t_dn7 = assign6400_e6415_d_n7;
        var_vfbac1_t_dn8 = assign6400_e6415_d_n8;
        var_vfbac1_t_dn9 = assign6400_e6415_d_n9;
        var_vfbac1_t_rv = 0.0;

        let (assign6410_e6427, assign6410_e6427_d_n4, assign6410_e6427_d_n6, assign6410_e6427_d_n7, assign6410_e6427_d_n8, assign6410_e6427_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6410_e6425: f64 = (var_vfbac2_t + var_temp);
        (assign6410_e6425, (var_vfbac2_t_dn4 + var_temp_dn4), (var_vfbac2_t_dn6 + var_temp_dn6), (var_vfbac2_t_dn7 + var_temp_dn7), (var_vfbac2_t_dn8 + var_temp_dn8), (var_vfbac2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign6410_e6427;
        var_vfbac2_t_dn4 = assign6410_e6427_d_n4;
        var_vfbac2_t_dn6 = assign6410_e6427_d_n6;
        var_vfbac2_t_dn7 = assign6410_e6427_d_n7;
        var_vfbac2_t_dn8 = assign6410_e6427_d_n8;
        var_vfbac2_t_dn9 = assign6410_e6427_d_n9;
        var_vfbac2_t_rv = 0.0;

        let (assign6420_e6449, assign6420_e6449_d_n4, assign6420_e6449_d_n6, assign6420_e6449_d_n7, assign6420_e6449_d_n8, assign6420_e6449_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6420_e6437: f64 = (p.p485 * var_temp0__blk79);
        let assign6420_e6440: f64 = (var_lambda_le).powf(p.p236);
        let assign6420_e6441: f64 = (assign6420_e6437 * assign6420_e6440);
        let assign6420_e6445: f64 = (p.p237 * var_iwe);
        let assign6420_e6446: f64 = (1.0 + assign6420_e6445);
        let assign6420_e6447: f64 = (assign6420_e6441 * assign6420_e6446);
        (assign6420_e6447, (((p.p485 * var_temp0__blk79_dn4) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn6) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn7) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn8) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn9) * assign6420_e6440) * assign6420_e6446),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6420_e6449;
        var_temp_dn4 = assign6420_e6449_d_n4;
        var_temp_dn6 = assign6420_e6449_d_n6;
        var_temp_dn7 = assign6420_e6449_d_n7;
        var_temp_dn8 = assign6420_e6449_d_n8;
        var_temp_dn9 = assign6420_e6449_d_n9;
        var_temp_rv = 0.0;

        let (assign6430_e6461, assign6430_e6461_d_n4, assign6430_e6461_d_n6, assign6430_e6461_d_n7, assign6430_e6461_d_n8, assign6430_e6461_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6430_e6459: f64 = (var_cf_p + var_temp);
        (assign6430_e6459, (var_cf_p_dn4 + var_temp_dn4), (var_cf_p_dn6 + var_temp_dn6), (var_cf_p_dn7 + var_temp_dn7), (var_cf_p_dn8 + var_temp_dn8), (var_cf_p_dn9 + var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign6430_e6461;
        var_cf_p_dn4 = assign6430_e6461_d_n4;
        var_cf_p_dn6 = assign6430_e6461_d_n6;
        var_cf_p_dn7 = assign6430_e6461_d_n7;
        var_cf_p_dn8 = assign6430_e6461_d_n8;
        var_cf_p_dn9 = assign6430_e6461_d_n9;
        var_cf_p_rv = 0.0;

        let (assign6440_e6473, assign6440_e6473_d_n4, assign6440_e6473_d_n6, assign6440_e6473_d_n7, assign6440_e6473_d_n8, assign6440_e6473_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6440_e6471: f64 = (var_cf_p).max(0.0);
        (assign6440_e6471, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign6440_e6473;
        var_cf1_t_dn4 = assign6440_e6473_d_n4;
        var_cf1_t_dn6 = assign6440_e6473_d_n6;
        var_cf1_t_dn7 = assign6440_e6473_d_n7;
        var_cf1_t_dn8 = assign6440_e6473_d_n8;
        var_cf1_t_dn9 = assign6440_e6473_d_n9;
        var_cf1_t_rv = 0.0;

        let (assign6450_e6485, assign6450_e6485_d_n4, assign6450_e6485_d_n6, assign6450_e6485_d_n7, assign6450_e6485_d_n8, assign6450_e6485_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6450_e6483: f64 = (var_cfac_p + var_temp);
        (assign6450_e6483, (var_cfac_p_dn4 + var_temp_dn4), (var_cfac_p_dn6 + var_temp_dn6), (var_cfac_p_dn7 + var_temp_dn7), (var_cfac_p_dn8 + var_temp_dn8), (var_cfac_p_dn9 + var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign6450_e6485;
        var_cfac_p_dn4 = assign6450_e6485_d_n4;
        var_cfac_p_dn6 = assign6450_e6485_d_n6;
        var_cfac_p_dn7 = assign6450_e6485_d_n7;
        var_cfac_p_dn8 = assign6450_e6485_d_n8;
        var_cfac_p_dn9 = assign6450_e6485_d_n9;
        var_cfac_p_rv = 0.0;

        let (assign6460_e6497, assign6460_e6497_d_n4, assign6460_e6497_d_n6, assign6460_e6497_d_n7, assign6460_e6497_d_n8, assign6460_e6497_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6460_e6495: f64 = (var_cfac_p).max(0.0);
        (assign6460_e6495, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign6460_e6497;
        var_cfac1_t_dn4 = assign6460_e6497_d_n4;
        var_cfac1_t_dn6 = assign6460_e6497_d_n6;
        var_cfac1_t_dn7 = assign6460_e6497_d_n7;
        var_cfac1_t_dn8 = assign6460_e6497_d_n8;
        var_cfac1_t_dn9 = assign6460_e6497_d_n9;
        var_cfac1_t_rv = 0.0;

        let (assign6470_e6511, assign6470_e6511_d_n4, assign6470_e6511_d_n6, assign6470_e6511_d_n7, assign6470_e6511_d_n8, assign6470_e6511_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6470_e6507: f64 = (p.p238 * var_tox2_i);
        let assign6470_e6509: f64 = (assign6470_e6507 / var_tox1_i);
        (assign6470_e6509, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6470_e6511;
        var_temp_dn4 = assign6470_e6511_d_n4;
        var_temp_dn6 = assign6470_e6511_d_n6;
        var_temp_dn7 = assign6470_e6511_d_n7;
        var_temp_dn8 = assign6470_e6511_d_n8;
        var_temp_dn9 = assign6470_e6511_d_n9;
        var_temp_rv = 0.0;

        let (assign6480_e6523, assign6480_e6523_d_n4, assign6480_e6523_d_n6, assign6480_e6523_d_n7, assign6480_e6523_d_n8, assign6480_e6523_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6480_e6521: f64 = (var_cf1_t * var_temp);
        (assign6480_e6521, ((var_cf1_t_dn4 * var_temp) + (var_cf1_t * var_temp_dn4)), ((var_cf1_t_dn6 * var_temp) + (var_cf1_t * var_temp_dn6)), ((var_cf1_t_dn7 * var_temp) + (var_cf1_t * var_temp_dn7)), ((var_cf1_t_dn8 * var_temp) + (var_cf1_t * var_temp_dn8)), ((var_cf1_t_dn9 * var_temp) + (var_cf1_t * var_temp_dn9)),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign6480_e6523;
        var_cf2_t_dn4 = assign6480_e6523_d_n4;
        var_cf2_t_dn6 = assign6480_e6523_d_n6;
        var_cf2_t_dn7 = assign6480_e6523_d_n7;
        var_cf2_t_dn8 = assign6480_e6523_d_n8;
        var_cf2_t_dn9 = assign6480_e6523_d_n9;
        var_cf2_t_rv = 0.0;

        let (assign6490_e6535, assign6490_e6535_d_n4, assign6490_e6535_d_n6, assign6490_e6535_d_n7, assign6490_e6535_d_n8, assign6490_e6535_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6490_e6533: f64 = (var_cfac1_t * var_temp);
        (assign6490_e6533, ((var_cfac1_t_dn4 * var_temp) + (var_cfac1_t * var_temp_dn4)), ((var_cfac1_t_dn6 * var_temp) + (var_cfac1_t * var_temp_dn6)), ((var_cfac1_t_dn7 * var_temp) + (var_cfac1_t * var_temp_dn7)), ((var_cfac1_t_dn8 * var_temp) + (var_cfac1_t * var_temp_dn8)), ((var_cfac1_t_dn9 * var_temp) + (var_cfac1_t * var_temp_dn9)),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign6490_e6535;
        var_cfac2_t_dn4 = assign6490_e6535_d_n4;
        var_cfac2_t_dn6 = assign6490_e6535_d_n6;
        var_cfac2_t_dn7 = assign6490_e6535_d_n7;
        var_cfac2_t_dn8 = assign6490_e6535_d_n8;
        var_cfac2_t_dn9 = assign6490_e6535_d_n9;
        var_cfac2_t_rv = 0.0;

        let assign6500_e6538: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard139 = assign6500_e6538;
        var_guard139_rv = 0.0;

        let (assign6510_e6542,) = {
    if (var_guard139 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign6510_e6542;
        var_novd_i_rv = 0.0;

        let (assign6520_e6546,) = {
    if (var_guard139 != 0.0) {
        (var_igovinv_t,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign6520_e6546;
        var_igovinvd_t_rv = 0.0;

        let (assign6540_e6554,) = {
    if (var_guard139 != 0.0) {
        (var_igovacc_t,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign6540_e6554;
        var_igovaccd_t_rv = 0.0;

        let (assign6550_e6558, assign6550_e6558_d_n4, assign6550_e6558_d_n6, assign6550_e6558_d_n7, assign6550_e6558_d_n8, assign6550_e6558_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_agidl_i, var_agidl_i_dn4, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9,)
    } else {
        (var_agidld_i, var_agidld_i_dn4, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9,)
    }
};
        var_agidld_i = assign6550_e6558;
        var_agidld_i_dn4 = assign6550_e6558_d_n4;
        var_agidld_i_dn6 = assign6550_e6558_d_n6;
        var_agidld_i_dn7 = assign6550_e6558_d_n7;
        var_agidld_i_dn8 = assign6550_e6558_d_n8;
        var_agidld_i_dn9 = assign6550_e6558_d_n9;
        var_agidld_i_rv = 0.0;

        let (assign6560_e6562,) = {
    if (var_guard139 != 0.0) {
        (var_bgidl_t,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign6560_e6562;
        var_bgidld_t_rv = 0.0;

        let (assign6570_e6566,) = {
    if (var_guard139 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign6570_e6566;
        var_stbgidld_i_rv = 0.0;

        let (assign6580_e6570,) = {
    if (var_guard139 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign6580_e6570;
        var_cgidld_i_rv = 0.0;

        let (assign6590_e6574,) = {
    if (var_guard139 != 0.0) {
        (var_dgidl_i,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign6590_e6574;
        var_dgidld_i_rv = 0.0;

        let (assign6600_e6578, assign6600_e6578_d_n4, assign6600_e6578_d_n6, assign6600_e6578_d_n7, assign6600_e6578_d_n8, assign6600_e6578_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign6600_e6578;
        var_covd_i_dn4 = assign6600_e6578_d_n4;
        var_covd_i_dn6 = assign6600_e6578_d_n6;
        var_covd_i_dn7 = assign6600_e6578_d_n7;
        var_covd_i_dn8 = assign6600_e6578_d_n8;
        var_covd_i_dn9 = assign6600_e6578_d_n9;
        var_covd_i_rv = 0.0;

        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn1_t_rv_slot = var_betn1_t_rv;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn2_t_rv_slot = var_betn2_t_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidld_t_rv_slot = var_bgidld_t_rv;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf1_t_rv_slot = var_cf1_t_rv;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf2_t_rv_slot = var_cf2_t_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac1_t_rv_slot = var_cfac1_t_rv;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac2_t_rv_slot = var_cfac2_t_rv;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfac_p_rv_slot = var_cfac_p_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covd_i_rv_slot = var_covd_i_rv;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_dgidld_i_rv_slot = var_dgidld_i_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovaccd_t_rv_slot = var_igovaccd_t_rv;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_igovinvd_t_rv_slot = var_igovinvd_t_rv;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kstressvth0_rv_slot = var_kstressvth0_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_dn4_slot = var_rhobetaref_dn4;
        *var_rhobetaref_dn6_slot = var_rhobetaref_dn6;
        *var_rhobetaref_dn7_slot = var_rhobetaref_dn7;
        *var_rhobetaref_dn8_slot = var_rhobetaref_dn8;
        *var_rhobetaref_dn9_slot = var_rhobetaref_dn9;
        *var_rhobetaref_rv_slot = var_rhobetaref_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp0__blk79_rv_slot = var_temp0__blk79_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb1_t_rv_slot = var_vfb1_t_rv;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfb2_t_rv_slot = var_vfb2_t_rv;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac1_t_rv_slot = var_vfbac1_t_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_cfac1_t: f64,
        var_cfac1_t_dn4: f64,
        var_cfac1_t_dn6: f64,
        var_cfac1_t_dn7: f64,
        var_cfac1_t_dn8: f64,
        var_cfac1_t_dn9: f64,
        var_cfac2_t: f64,
        var_cfac2_t_dn4: f64,
        var_cfac2_t_dn6: f64,
        var_cfac2_t_dn7: f64,
        var_cfac2_t_dn8: f64,
        var_cfac2_t_dn9: f64,
        var_cfd_i: f64,
        var_cfr_i: f64,
        var_cfr_i_dn4: f64,
        var_cfr_i_dn6: f64,
        var_cfr_i_dn7: f64,
        var_cfr_i_dn8: f64,
        var_cfr_i_dn9: f64,
        var_ct_i: f64,
        var_dt: f64,
        var_dt_dn4: f64,
        var_dt_dn6: f64,
        var_dt_dn7: f64,
        var_dt_dn8: f64,
        var_dt_dn9: f64,
        var_guard139: f64,
        var_inv_phit0: f64,
        var_inv_phit0_dn4: f64,
        var_inv_phit0_dn6: f64,
        var_inv_phit0_dn7: f64,
        var_inv_phit0_dn8: f64,
        var_inv_phit0_dn9: f64,
        var_nch_i: f64,
        var_nsddc_i: f64,
        var_nsub_i: f64,
        var_phit0: f64,
        var_phit0_dn4: f64,
        var_phit0_dn6: f64,
        var_phit0_dn7: f64,
        var_phit0_dn8: f64,
        var_phit0_dn9: f64,
        var_pnce_i: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_rtn_dn6: f64,
        var_rtn_dn7: f64,
        var_rtn_dn8: f64,
        var_rtn_dn9: f64,
        var_stcf_i: f64,
        var_stcf_i_dn4: f64,
        var_stcf_i_dn6: f64,
        var_stcf_i_dn7: f64,
        var_stcf_i_dn8: f64,
        var_stcf_i_dn9: f64,
        var_tkc: f64,
        var_tkc_dn4: f64,
        var_tkc_dn6: f64,
        var_tkc_dn7: f64,
        var_tkc_dn8: f64,
        var_tkc_dn9: f64,
        var_tkc_sq: f64,
        var_tkc_sq_dn4: f64,
        var_tkc_sq_dn6: f64,
        var_tkc_sq_dn7: f64,
        var_tkc_sq_dn8: f64,
        var_tkc_sq_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_tsi_i: f64,
        var_typech_i: f64,
        var_xge_i: f64,
        var_a0_csisq_slot: &mut f64,
        var_a0_csisq_dn4_slot: &mut f64,
        var_a0_csisq_dn6_slot: &mut f64,
        var_a0_csisq_dn7_slot: &mut f64,
        var_a0_csisq_dn8_slot: &mut f64,
        var_a0_csisq_dn9_slot: &mut f64,
        var_a0_csisq_rv_slot: &mut f64,
        var_cf1_i_slot: &mut f64,
        var_cf1_i_dn4_slot: &mut f64,
        var_cf1_i_dn6_slot: &mut f64,
        var_cf1_i_dn7_slot: &mut f64,
        var_cf1_i_dn8_slot: &mut f64,
        var_cf1_i_dn9_slot: &mut f64,
        var_cf1_i_rv_slot: &mut f64,
        var_cf2_i_slot: &mut f64,
        var_cf2_i_dn4_slot: &mut f64,
        var_cf2_i_dn6_slot: &mut f64,
        var_cf2_i_dn7_slot: &mut f64,
        var_cf2_i_dn8_slot: &mut f64,
        var_cf2_i_dn9_slot: &mut f64,
        var_cf2_i_rv_slot: &mut f64,
        var_cfac1_i_slot: &mut f64,
        var_cfac1_i_dn4_slot: &mut f64,
        var_cfac1_i_dn6_slot: &mut f64,
        var_cfac1_i_dn7_slot: &mut f64,
        var_cfac1_i_dn8_slot: &mut f64,
        var_cfac1_i_dn9_slot: &mut f64,
        var_cfac1_i_rv_slot: &mut f64,
        var_cfac2_i_slot: &mut f64,
        var_cfac2_i_dn4_slot: &mut f64,
        var_cfac2_i_dn6_slot: &mut f64,
        var_cfac2_i_dn7_slot: &mut f64,
        var_cfac2_i_dn8_slot: &mut f64,
        var_cfac2_i_dn9_slot: &mut f64,
        var_cfac2_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cox1init_slot: &mut f64,
        var_cox1init_rv_slot: &mut f64,
        var_cox1prime_slot: &mut f64,
        var_cox1prime_rv_slot: &mut f64,
        var_cox2init_slot: &mut f64,
        var_cox2init_rv_slot: &mut f64,
        var_cox2prime_slot: &mut f64,
        var_cox2prime_rv_slot: &mut f64,
        var_csiprime_0_slot: &mut f64,
        var_csiprime_0_rv_slot: &mut f64,
        var_deg_slot: &mut f64,
        var_deg_dn4_slot: &mut f64,
        var_deg_dn6_slot: &mut f64,
        var_deg_dn7_slot: &mut f64,
        var_deg_dn8_slot: &mut f64,
        var_deg_dn9_slot: &mut f64,
        var_deg_rv_slot: &mut f64,
        var_dvfb1nch_slot: &mut f64,
        var_dvfb1nch_dn4_slot: &mut f64,
        var_dvfb1nch_dn6_slot: &mut f64,
        var_dvfb1nch_dn7_slot: &mut f64,
        var_dvfb1nch_dn8_slot: &mut f64,
        var_dvfb1nch_dn9_slot: &mut f64,
        var_dvfb1nch_rv_slot: &mut f64,
        var_dvfb2nch_slot: &mut f64,
        var_dvfb2nch_dn4_slot: &mut f64,
        var_dvfb2nch_dn6_slot: &mut f64,
        var_dvfb2nch_dn7_slot: &mut f64,
        var_dvfb2nch_dn8_slot: &mut f64,
        var_dvfb2nch_dn9_slot: &mut f64,
        var_dvfb2nch_rv_slot: &mut f64,
        var_dvfbch_slot: &mut f64,
        var_dvfbch_dn4_slot: &mut f64,
        var_dvfbch_dn6_slot: &mut f64,
        var_dvfbch_dn7_slot: &mut f64,
        var_dvfbch_dn8_slot: &mut f64,
        var_dvfbch_dn9_slot: &mut f64,
        var_dvfbch_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_2phit_slot: &mut f64,
        var_eg_2phit0_slot: &mut f64,
        var_eg_2phit0_dn4_slot: &mut f64,
        var_eg_2phit0_dn6_slot: &mut f64,
        var_eg_2phit0_dn7_slot: &mut f64,
        var_eg_2phit0_dn8_slot: &mut f64,
        var_eg_2phit0_dn9_slot: &mut f64,
        var_eg_2phit0_rv_slot: &mut f64,
        var_eg_2phit0_woshe_slot: &mut f64,
        var_eg_2phit0_woshe_dn4_slot: &mut f64,
        var_eg_2phit0_woshe_dn6_slot: &mut f64,
        var_eg_2phit0_woshe_dn7_slot: &mut f64,
        var_eg_2phit0_woshe_dn8_slot: &mut f64,
        var_eg_2phit0_woshe_dn9_slot: &mut f64,
        var_eg_2phit0_woshe_rv_slot: &mut f64,
        var_eg_2phit_dn4_slot: &mut f64,
        var_eg_2phit_dn6_slot: &mut f64,
        var_eg_2phit_dn7_slot: &mut f64,
        var_eg_2phit_dn8_slot: &mut f64,
        var_eg_2phit_dn9_slot: &mut f64,
        var_eg_2phit_rv_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_dn8_slot: &mut f64,
        var_eg_dn9_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_egge_slot: &mut f64,
        var_egge_dn4_slot: &mut f64,
        var_egge_dn6_slot: &mut f64,
        var_egge_dn7_slot: &mut f64,
        var_egge_dn8_slot: &mut f64,
        var_egge_dn9_slot: &mut f64,
        var_egge_rv_slot: &mut f64,
        var_egsi_slot: &mut f64,
        var_egsi_dn4_slot: &mut f64,
        var_egsi_dn6_slot: &mut f64,
        var_egsi_dn7_slot: &mut f64,
        var_egsi_dn8_slot: &mut f64,
        var_egsi_dn9_slot: &mut f64,
        var_egsi_rv_slot: &mut f64,
        var_epsch_slot: &mut f64,
        var_epsch_rv_slot: &mut f64,
        var_gfsub_slot: &mut f64,
        var_gfsub2_slot: &mut f64,
        var_gfsub2_dn4_slot: &mut f64,
        var_gfsub2_dn6_slot: &mut f64,
        var_gfsub2_dn7_slot: &mut f64,
        var_gfsub2_dn8_slot: &mut f64,
        var_gfsub2_dn9_slot: &mut f64,
        var_gfsub2_rv_slot: &mut f64,
        var_gfsub_dn4_slot: &mut f64,
        var_gfsub_dn6_slot: &mut f64,
        var_gfsub_dn7_slot: &mut f64,
        var_gfsub_dn8_slot: &mut f64,
        var_gfsub_dn9_slot: &mut f64,
        var_gfsub_rv_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard140_rv_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard141_rv_slot: &mut f64,
        var_inv_gfsub2_slot: &mut f64,
        var_inv_gfsub2_dn4_slot: &mut f64,
        var_inv_gfsub2_dn6_slot: &mut f64,
        var_inv_gfsub2_dn7_slot: &mut f64,
        var_inv_gfsub2_dn8_slot: &mut f64,
        var_inv_gfsub2_dn9_slot: &mut f64,
        var_inv_gfsub2_rv_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_dn4_slot: &mut f64,
        var_inv_phit_dn6_slot: &mut f64,
        var_inv_phit_dn7_slot: &mut f64,
        var_inv_phit_dn8_slot: &mut f64,
        var_inv_phit_dn9_slot: &mut f64,
        var_inv_phit_rv_slot: &mut f64,
        var_inv_xisub_slot: &mut f64,
        var_inv_xisub_dn4_slot: &mut f64,
        var_inv_xisub_dn6_slot: &mut f64,
        var_inv_xisub_dn7_slot: &mut f64,
        var_inv_xisub_dn8_slot: &mut f64,
        var_inv_xisub_dn9_slot: &mut f64,
        var_inv_xisub_rv_slot: &mut f64,
        var_k1_1d_slot: &mut f64,
        var_k1_1d_rv_slot: &mut f64,
        var_k2_1d_slot: &mut f64,
        var_k2_1d_rv_slot: &mut f64,
        var_keq_1d_slot: &mut f64,
        var_keq_1d_rv_slot: &mut f64,
        var_margin_sub_slot: &mut f64,
        var_margin_sub_dn4_slot: &mut f64,
        var_margin_sub_dn6_slot: &mut f64,
        var_margin_sub_dn7_slot: &mut f64,
        var_margin_sub_dn8_slot: &mut f64,
        var_margin_sub_dn9_slot: &mut f64,
        var_margin_sub_rv_slot: &mut f64,
        var_neff_slot: &mut f64,
        var_neff_dn4_slot: &mut f64,
        var_neff_dn6_slot: &mut f64,
        var_neff_dn7_slot: &mut f64,
        var_neff_dn8_slot: &mut f64,
        var_neff_dn9_slot: &mut f64,
        var_neff_poly_slot: &mut f64,
        var_neff_poly_dn4_slot: &mut f64,
        var_neff_poly_dn6_slot: &mut f64,
        var_neff_poly_dn7_slot: &mut f64,
        var_neff_poly_dn8_slot: &mut f64,
        var_neff_poly_dn9_slot: &mut f64,
        var_neff_poly_rv_slot: &mut f64,
        var_neff_rv_slot: &mut f64,
        var_neff_sub_slot: &mut f64,
        var_neff_sub_dn4_slot: &mut f64,
        var_neff_sub_dn6_slot: &mut f64,
        var_neff_sub_dn7_slot: &mut f64,
        var_neff_sub_dn8_slot: &mut f64,
        var_neff_sub_dn9_slot: &mut f64,
        var_neff_sub_rv_slot: &mut f64,
        var_niratio_slot: &mut f64,
        var_niratio_rv_slot: &mut f64,
        var_one_m_xge_slot: &mut f64,
        var_one_m_xge_rv_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_dn4_slot: &mut f64,
        var_phit_dn6_slot: &mut f64,
        var_phit_dn7_slot: &mut f64,
        var_phit_dn8_slot: &mut f64,
        var_phit_dn9_slot: &mut f64,
        var_phit_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_xd0_slot: &mut f64,
        var_xd0_dn4_slot: &mut f64,
        var_xd0_dn6_slot: &mut f64,
        var_xd0_dn7_slot: &mut f64,
        var_xd0_dn8_slot: &mut f64,
        var_xd0_dn9_slot: &mut f64,
        var_xd0_rv_slot: &mut f64,
        var_xisub_slot: &mut f64,
        var_xisub_dn4_slot: &mut f64,
        var_xisub_dn6_slot: &mut f64,
        var_xisub_dn7_slot: &mut f64,
        var_xisub_dn8_slot: &mut f64,
        var_xisub_dn9_slot: &mut f64,
        var_xisub_rv_slot: &mut f64,
        var_xsddep_slot: &mut f64,
        var_xsddep_dn4_slot: &mut f64,
        var_xsddep_dn6_slot: &mut f64,
        var_xsddep_dn7_slot: &mut f64,
        var_xsddep_dn8_slot: &mut f64,
        var_xsddep_dn9_slot: &mut f64,
        var_xsddep_rv_slot: &mut f64,
        var_xth_1d_slot: &mut f64,
        var_xth_1d_dn4_slot: &mut f64,
        var_xth_1d_dn6_slot: &mut f64,
        var_xth_1d_dn7_slot: &mut f64,
        var_xth_1d_dn8_slot: &mut f64,
        var_xth_1d_dn9_slot: &mut f64,
        var_xth_1d_rv_slot: &mut f64,
    ) {
        let mut var_a0_csisq: f64 = *var_a0_csisq_slot;
        let mut var_a0_csisq_dn4: f64 = *var_a0_csisq_dn4_slot;
        let mut var_a0_csisq_dn6: f64 = *var_a0_csisq_dn6_slot;
        let mut var_a0_csisq_dn7: f64 = *var_a0_csisq_dn7_slot;
        let mut var_a0_csisq_dn8: f64 = *var_a0_csisq_dn8_slot;
        let mut var_a0_csisq_dn9: f64 = *var_a0_csisq_dn9_slot;
        let mut var_a0_csisq_rv: f64 = *var_a0_csisq_rv_slot;
        let mut var_cf1_i: f64 = *var_cf1_i_slot;
        let mut var_cf1_i_dn4: f64 = *var_cf1_i_dn4_slot;
        let mut var_cf1_i_dn6: f64 = *var_cf1_i_dn6_slot;
        let mut var_cf1_i_dn7: f64 = *var_cf1_i_dn7_slot;
        let mut var_cf1_i_dn8: f64 = *var_cf1_i_dn8_slot;
        let mut var_cf1_i_dn9: f64 = *var_cf1_i_dn9_slot;
        let mut var_cf1_i_rv: f64 = *var_cf1_i_rv_slot;
        let mut var_cf2_i: f64 = *var_cf2_i_slot;
        let mut var_cf2_i_dn4: f64 = *var_cf2_i_dn4_slot;
        let mut var_cf2_i_dn6: f64 = *var_cf2_i_dn6_slot;
        let mut var_cf2_i_dn7: f64 = *var_cf2_i_dn7_slot;
        let mut var_cf2_i_dn8: f64 = *var_cf2_i_dn8_slot;
        let mut var_cf2_i_dn9: f64 = *var_cf2_i_dn9_slot;
        let mut var_cf2_i_rv: f64 = *var_cf2_i_rv_slot;
        let mut var_cfac1_i: f64 = *var_cfac1_i_slot;
        let mut var_cfac1_i_dn4: f64 = *var_cfac1_i_dn4_slot;
        let mut var_cfac1_i_dn6: f64 = *var_cfac1_i_dn6_slot;
        let mut var_cfac1_i_dn7: f64 = *var_cfac1_i_dn7_slot;
        let mut var_cfac1_i_dn8: f64 = *var_cfac1_i_dn8_slot;
        let mut var_cfac1_i_dn9: f64 = *var_cfac1_i_dn9_slot;
        let mut var_cfac1_i_rv: f64 = *var_cfac1_i_rv_slot;
        let mut var_cfac2_i: f64 = *var_cfac2_i_slot;
        let mut var_cfac2_i_dn4: f64 = *var_cfac2_i_dn4_slot;
        let mut var_cfac2_i_dn6: f64 = *var_cfac2_i_dn6_slot;
        let mut var_cfac2_i_dn7: f64 = *var_cfac2_i_dn7_slot;
        let mut var_cfac2_i_dn8: f64 = *var_cfac2_i_dn8_slot;
        let mut var_cfac2_i_dn9: f64 = *var_cfac2_i_dn9_slot;
        let mut var_cfac2_i_rv: f64 = *var_cfac2_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cox1init: f64 = *var_cox1init_slot;
        let mut var_cox1init_rv: f64 = *var_cox1init_rv_slot;
        let mut var_cox1prime: f64 = *var_cox1prime_slot;
        let mut var_cox1prime_rv: f64 = *var_cox1prime_rv_slot;
        let mut var_cox2init: f64 = *var_cox2init_slot;
        let mut var_cox2init_rv: f64 = *var_cox2init_rv_slot;
        let mut var_cox2prime: f64 = *var_cox2prime_slot;
        let mut var_cox2prime_rv: f64 = *var_cox2prime_rv_slot;
        let mut var_csiprime_0: f64 = *var_csiprime_0_slot;
        let mut var_csiprime_0_rv: f64 = *var_csiprime_0_rv_slot;
        let mut var_deg: f64 = *var_deg_slot;
        let mut var_deg_dn4: f64 = *var_deg_dn4_slot;
        let mut var_deg_dn6: f64 = *var_deg_dn6_slot;
        let mut var_deg_dn7: f64 = *var_deg_dn7_slot;
        let mut var_deg_dn8: f64 = *var_deg_dn8_slot;
        let mut var_deg_dn9: f64 = *var_deg_dn9_slot;
        let mut var_deg_rv: f64 = *var_deg_rv_slot;
        let mut var_dvfb1nch: f64 = *var_dvfb1nch_slot;
        let mut var_dvfb1nch_dn4: f64 = *var_dvfb1nch_dn4_slot;
        let mut var_dvfb1nch_dn6: f64 = *var_dvfb1nch_dn6_slot;
        let mut var_dvfb1nch_dn7: f64 = *var_dvfb1nch_dn7_slot;
        let mut var_dvfb1nch_dn8: f64 = *var_dvfb1nch_dn8_slot;
        let mut var_dvfb1nch_dn9: f64 = *var_dvfb1nch_dn9_slot;
        let mut var_dvfb1nch_rv: f64 = *var_dvfb1nch_rv_slot;
        let mut var_dvfb2nch: f64 = *var_dvfb2nch_slot;
        let mut var_dvfb2nch_dn4: f64 = *var_dvfb2nch_dn4_slot;
        let mut var_dvfb2nch_dn6: f64 = *var_dvfb2nch_dn6_slot;
        let mut var_dvfb2nch_dn7: f64 = *var_dvfb2nch_dn7_slot;
        let mut var_dvfb2nch_dn8: f64 = *var_dvfb2nch_dn8_slot;
        let mut var_dvfb2nch_dn9: f64 = *var_dvfb2nch_dn9_slot;
        let mut var_dvfb2nch_rv: f64 = *var_dvfb2nch_rv_slot;
        let mut var_dvfbch: f64 = *var_dvfbch_slot;
        let mut var_dvfbch_dn4: f64 = *var_dvfbch_dn4_slot;
        let mut var_dvfbch_dn6: f64 = *var_dvfbch_dn6_slot;
        let mut var_dvfbch_dn7: f64 = *var_dvfbch_dn7_slot;
        let mut var_dvfbch_dn8: f64 = *var_dvfbch_dn8_slot;
        let mut var_dvfbch_dn9: f64 = *var_dvfbch_dn9_slot;
        let mut var_dvfbch_rv: f64 = *var_dvfbch_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_2phit: f64 = *var_eg_2phit_slot;
        let mut var_eg_2phit0: f64 = *var_eg_2phit0_slot;
        let mut var_eg_2phit0_dn4: f64 = *var_eg_2phit0_dn4_slot;
        let mut var_eg_2phit0_dn6: f64 = *var_eg_2phit0_dn6_slot;
        let mut var_eg_2phit0_dn7: f64 = *var_eg_2phit0_dn7_slot;
        let mut var_eg_2phit0_dn8: f64 = *var_eg_2phit0_dn8_slot;
        let mut var_eg_2phit0_dn9: f64 = *var_eg_2phit0_dn9_slot;
        let mut var_eg_2phit0_rv: f64 = *var_eg_2phit0_rv_slot;
        let mut var_eg_2phit0_woshe: f64 = *var_eg_2phit0_woshe_slot;
        let mut var_eg_2phit0_woshe_dn4: f64 = *var_eg_2phit0_woshe_dn4_slot;
        let mut var_eg_2phit0_woshe_dn6: f64 = *var_eg_2phit0_woshe_dn6_slot;
        let mut var_eg_2phit0_woshe_dn7: f64 = *var_eg_2phit0_woshe_dn7_slot;
        let mut var_eg_2phit0_woshe_dn8: f64 = *var_eg_2phit0_woshe_dn8_slot;
        let mut var_eg_2phit0_woshe_dn9: f64 = *var_eg_2phit0_woshe_dn9_slot;
        let mut var_eg_2phit0_woshe_rv: f64 = *var_eg_2phit0_woshe_rv_slot;
        let mut var_eg_2phit_dn4: f64 = *var_eg_2phit_dn4_slot;
        let mut var_eg_2phit_dn6: f64 = *var_eg_2phit_dn6_slot;
        let mut var_eg_2phit_dn7: f64 = *var_eg_2phit_dn7_slot;
        let mut var_eg_2phit_dn8: f64 = *var_eg_2phit_dn8_slot;
        let mut var_eg_2phit_dn9: f64 = *var_eg_2phit_dn9_slot;
        let mut var_eg_2phit_rv: f64 = *var_eg_2phit_rv_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_dn8: f64 = *var_eg_dn8_slot;
        let mut var_eg_dn9: f64 = *var_eg_dn9_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_egge: f64 = *var_egge_slot;
        let mut var_egge_dn4: f64 = *var_egge_dn4_slot;
        let mut var_egge_dn6: f64 = *var_egge_dn6_slot;
        let mut var_egge_dn7: f64 = *var_egge_dn7_slot;
        let mut var_egge_dn8: f64 = *var_egge_dn8_slot;
        let mut var_egge_dn9: f64 = *var_egge_dn9_slot;
        let mut var_egge_rv: f64 = *var_egge_rv_slot;
        let mut var_egsi: f64 = *var_egsi_slot;
        let mut var_egsi_dn4: f64 = *var_egsi_dn4_slot;
        let mut var_egsi_dn6: f64 = *var_egsi_dn6_slot;
        let mut var_egsi_dn7: f64 = *var_egsi_dn7_slot;
        let mut var_egsi_dn8: f64 = *var_egsi_dn8_slot;
        let mut var_egsi_dn9: f64 = *var_egsi_dn9_slot;
        let mut var_egsi_rv: f64 = *var_egsi_rv_slot;
        let mut var_epsch: f64 = *var_epsch_slot;
        let mut var_epsch_rv: f64 = *var_epsch_rv_slot;
        let mut var_gfsub: f64 = *var_gfsub_slot;
        let mut var_gfsub2: f64 = *var_gfsub2_slot;
        let mut var_gfsub2_dn4: f64 = *var_gfsub2_dn4_slot;
        let mut var_gfsub2_dn6: f64 = *var_gfsub2_dn6_slot;
        let mut var_gfsub2_dn7: f64 = *var_gfsub2_dn7_slot;
        let mut var_gfsub2_dn8: f64 = *var_gfsub2_dn8_slot;
        let mut var_gfsub2_dn9: f64 = *var_gfsub2_dn9_slot;
        let mut var_gfsub2_rv: f64 = *var_gfsub2_rv_slot;
        let mut var_gfsub_dn4: f64 = *var_gfsub_dn4_slot;
        let mut var_gfsub_dn6: f64 = *var_gfsub_dn6_slot;
        let mut var_gfsub_dn7: f64 = *var_gfsub_dn7_slot;
        let mut var_gfsub_dn8: f64 = *var_gfsub_dn8_slot;
        let mut var_gfsub_dn9: f64 = *var_gfsub_dn9_slot;
        let mut var_gfsub_rv: f64 = *var_gfsub_rv_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard140_rv: f64 = *var_guard140_rv_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard141_rv: f64 = *var_guard141_rv_slot;
        let mut var_inv_gfsub2: f64 = *var_inv_gfsub2_slot;
        let mut var_inv_gfsub2_dn4: f64 = *var_inv_gfsub2_dn4_slot;
        let mut var_inv_gfsub2_dn6: f64 = *var_inv_gfsub2_dn6_slot;
        let mut var_inv_gfsub2_dn7: f64 = *var_inv_gfsub2_dn7_slot;
        let mut var_inv_gfsub2_dn8: f64 = *var_inv_gfsub2_dn8_slot;
        let mut var_inv_gfsub2_dn9: f64 = *var_inv_gfsub2_dn9_slot;
        let mut var_inv_gfsub2_rv: f64 = *var_inv_gfsub2_rv_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_dn4: f64 = *var_inv_phit_dn4_slot;
        let mut var_inv_phit_dn6: f64 = *var_inv_phit_dn6_slot;
        let mut var_inv_phit_dn7: f64 = *var_inv_phit_dn7_slot;
        let mut var_inv_phit_dn8: f64 = *var_inv_phit_dn8_slot;
        let mut var_inv_phit_dn9: f64 = *var_inv_phit_dn9_slot;
        let mut var_inv_phit_rv: f64 = *var_inv_phit_rv_slot;
        let mut var_inv_xisub: f64 = *var_inv_xisub_slot;
        let mut var_inv_xisub_dn4: f64 = *var_inv_xisub_dn4_slot;
        let mut var_inv_xisub_dn6: f64 = *var_inv_xisub_dn6_slot;
        let mut var_inv_xisub_dn7: f64 = *var_inv_xisub_dn7_slot;
        let mut var_inv_xisub_dn8: f64 = *var_inv_xisub_dn8_slot;
        let mut var_inv_xisub_dn9: f64 = *var_inv_xisub_dn9_slot;
        let mut var_inv_xisub_rv: f64 = *var_inv_xisub_rv_slot;
        let mut var_k1_1d: f64 = *var_k1_1d_slot;
        let mut var_k1_1d_rv: f64 = *var_k1_1d_rv_slot;
        let mut var_k2_1d: f64 = *var_k2_1d_slot;
        let mut var_k2_1d_rv: f64 = *var_k2_1d_rv_slot;
        let mut var_keq_1d: f64 = *var_keq_1d_slot;
        let mut var_keq_1d_rv: f64 = *var_keq_1d_rv_slot;
        let mut var_margin_sub: f64 = *var_margin_sub_slot;
        let mut var_margin_sub_dn4: f64 = *var_margin_sub_dn4_slot;
        let mut var_margin_sub_dn6: f64 = *var_margin_sub_dn6_slot;
        let mut var_margin_sub_dn7: f64 = *var_margin_sub_dn7_slot;
        let mut var_margin_sub_dn8: f64 = *var_margin_sub_dn8_slot;
        let mut var_margin_sub_dn9: f64 = *var_margin_sub_dn9_slot;
        let mut var_margin_sub_rv: f64 = *var_margin_sub_rv_slot;
        let mut var_neff: f64 = *var_neff_slot;
        let mut var_neff_dn4: f64 = *var_neff_dn4_slot;
        let mut var_neff_dn6: f64 = *var_neff_dn6_slot;
        let mut var_neff_dn7: f64 = *var_neff_dn7_slot;
        let mut var_neff_dn8: f64 = *var_neff_dn8_slot;
        let mut var_neff_dn9: f64 = *var_neff_dn9_slot;
        let mut var_neff_poly: f64 = *var_neff_poly_slot;
        let mut var_neff_poly_dn4: f64 = *var_neff_poly_dn4_slot;
        let mut var_neff_poly_dn6: f64 = *var_neff_poly_dn6_slot;
        let mut var_neff_poly_dn7: f64 = *var_neff_poly_dn7_slot;
        let mut var_neff_poly_dn8: f64 = *var_neff_poly_dn8_slot;
        let mut var_neff_poly_dn9: f64 = *var_neff_poly_dn9_slot;
        let mut var_neff_poly_rv: f64 = *var_neff_poly_rv_slot;
        let mut var_neff_rv: f64 = *var_neff_rv_slot;
        let mut var_neff_sub: f64 = *var_neff_sub_slot;
        let mut var_neff_sub_dn4: f64 = *var_neff_sub_dn4_slot;
        let mut var_neff_sub_dn6: f64 = *var_neff_sub_dn6_slot;
        let mut var_neff_sub_dn7: f64 = *var_neff_sub_dn7_slot;
        let mut var_neff_sub_dn8: f64 = *var_neff_sub_dn8_slot;
        let mut var_neff_sub_dn9: f64 = *var_neff_sub_dn9_slot;
        let mut var_neff_sub_rv: f64 = *var_neff_sub_rv_slot;
        let mut var_niratio: f64 = *var_niratio_slot;
        let mut var_niratio_rv: f64 = *var_niratio_rv_slot;
        let mut var_one_m_xge: f64 = *var_one_m_xge_slot;
        let mut var_one_m_xge_rv: f64 = *var_one_m_xge_rv_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_dn4: f64 = *var_phit_dn4_slot;
        let mut var_phit_dn6: f64 = *var_phit_dn6_slot;
        let mut var_phit_dn7: f64 = *var_phit_dn7_slot;
        let mut var_phit_dn8: f64 = *var_phit_dn8_slot;
        let mut var_phit_dn9: f64 = *var_phit_dn9_slot;
        let mut var_phit_rv: f64 = *var_phit_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_xd0: f64 = *var_xd0_slot;
        let mut var_xd0_dn4: f64 = *var_xd0_dn4_slot;
        let mut var_xd0_dn6: f64 = *var_xd0_dn6_slot;
        let mut var_xd0_dn7: f64 = *var_xd0_dn7_slot;
        let mut var_xd0_dn8: f64 = *var_xd0_dn8_slot;
        let mut var_xd0_dn9: f64 = *var_xd0_dn9_slot;
        let mut var_xd0_rv: f64 = *var_xd0_rv_slot;
        let mut var_xisub: f64 = *var_xisub_slot;
        let mut var_xisub_dn4: f64 = *var_xisub_dn4_slot;
        let mut var_xisub_dn6: f64 = *var_xisub_dn6_slot;
        let mut var_xisub_dn7: f64 = *var_xisub_dn7_slot;
        let mut var_xisub_dn8: f64 = *var_xisub_dn8_slot;
        let mut var_xisub_dn9: f64 = *var_xisub_dn9_slot;
        let mut var_xisub_rv: f64 = *var_xisub_rv_slot;
        let mut var_xsddep: f64 = *var_xsddep_slot;
        let mut var_xsddep_dn4: f64 = *var_xsddep_dn4_slot;
        let mut var_xsddep_dn6: f64 = *var_xsddep_dn6_slot;
        let mut var_xsddep_dn7: f64 = *var_xsddep_dn7_slot;
        let mut var_xsddep_dn8: f64 = *var_xsddep_dn8_slot;
        let mut var_xsddep_dn9: f64 = *var_xsddep_dn9_slot;
        let mut var_xsddep_rv: f64 = *var_xsddep_rv_slot;
        let mut var_xth_1d: f64 = *var_xth_1d_slot;
        let mut var_xth_1d_dn4: f64 = *var_xth_1d_dn4_slot;
        let mut var_xth_1d_dn6: f64 = *var_xth_1d_dn6_slot;
        let mut var_xth_1d_dn7: f64 = *var_xth_1d_dn7_slot;
        let mut var_xth_1d_dn8: f64 = *var_xth_1d_dn8_slot;
        let mut var_xth_1d_dn9: f64 = *var_xth_1d_dn9_slot;
        let mut var_xth_1d_rv: f64 = *var_xth_1d_rv_slot;

        let (assign6610_e6582, assign6610_e6582_d_n4, assign6610_e6582_d_n6, assign6610_e6582_d_n7, assign6610_e6582_d_n8, assign6610_e6582_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign6610_e6582;
        var_cfrd_i_dn4 = assign6610_e6582_d_n4;
        var_cfrd_i_dn6 = assign6610_e6582_d_n6;
        var_cfrd_i_dn7 = assign6610_e6582_d_n7;
        var_cfrd_i_dn8 = assign6610_e6582_d_n8;
        var_cfrd_i_dn9 = assign6610_e6582_d_n9;
        var_cfrd_i_rv = 0.0;

        let assign6620_e6585: f64 = (1.0 - var_xge_i);
        var_one_m_xge = assign6620_e6585;
        var_one_m_xge_rv = 0.0;

        let assign6630_e6588: f64 = (1.04479e-10 * var_one_m_xge);
        let assign6630_e6591: f64 = (1.43438e-10 * var_xge_i);
        let assign6630_e6592: f64 = (assign6630_e6588 + assign6630_e6591);
        var_epsch = assign6630_e6592;
        var_epsch_rv = 0.0;

        let assign6640_e6596: f64 = (0.000473 * var_tkc_sq);
        let assign6640_e6599: f64 = (636.0 + var_tkc);
        let assign6640_e6600: f64 = (assign6640_e6596 / assign6640_e6599);
        let assign6640_e6601: f64 = (1.17 - assign6640_e6600);
        var_egsi = assign6640_e6601;
        var_egsi_dn4 = (-((((0.000473 * var_tkc_sq_dn4) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn4)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn6 = (-((((0.000473 * var_tkc_sq_dn6) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn6)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn7 = (-((((0.000473 * var_tkc_sq_dn7) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn7)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn8 = (-((((0.000473 * var_tkc_sq_dn8) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn8)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn9 = (-((((0.000473 * var_tkc_sq_dn9) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn9)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_rv = 0.0;

        let assign6650_e6605: f64 = (0.0004774 * var_tkc_sq);
        let assign6650_e6608: f64 = (235.0 + var_tkc);
        let assign6650_e6609: f64 = (assign6650_e6605 / assign6650_e6608);
        let assign6650_e6610: f64 = (0.744 - assign6650_e6609);
        var_egge = assign6650_e6610;
        var_egge_dn4 = (-((((0.0004774 * var_tkc_sq_dn4) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn4)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn6 = (-((((0.0004774 * var_tkc_sq_dn6) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn6)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn7 = (-((((0.0004774 * var_tkc_sq_dn7) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn7)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn8 = (-((((0.0004774 * var_tkc_sq_dn8) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn8)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn9 = (-((((0.0004774 * var_tkc_sq_dn9) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn9)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_rv = 0.0;

        let assign6660_e6613: f64 = (var_egge - var_egsi);
        let assign6660_e6615: f64 = (-0.4);
        let assign6660_e6617: f64 = (assign6660_e6615 * var_one_m_xge);
        let assign6660_e6618: f64 = (assign6660_e6613 + assign6660_e6617);
        let assign6660_e6620: f64 = (assign6660_e6618 * var_xge_i);
        var_deg = assign6660_e6620;
        var_deg_dn4 = ((var_egge_dn4 - var_egsi_dn4) * var_xge_i);
        var_deg_dn6 = ((var_egge_dn6 - var_egsi_dn6) * var_xge_i);
        var_deg_dn7 = ((var_egge_dn7 - var_egsi_dn7) * var_xge_i);
        var_deg_dn8 = ((var_egge_dn8 - var_egsi_dn8) * var_xge_i);
        var_deg_dn9 = ((var_egge_dn9 - var_egsi_dn9) * var_xge_i);
        var_deg_rv = 0.0;

        let assign6670_e6623: f64 = (var_egsi + var_deg);
        var_eg = assign6670_e6623;
        var_eg_dn4 = (var_egsi_dn4 + var_deg_dn4);
        var_eg_dn6 = (var_egsi_dn6 + var_deg_dn6);
        var_eg_dn7 = (var_egsi_dn7 + var_deg_dn7);
        var_eg_dn8 = (var_egsi_dn8 + var_deg_dn8);
        var_eg_dn9 = (var_egsi_dn9 + var_deg_dn9);
        var_eg_rv = 0.0;

        let assign6680_e6626: f64 = (0.5 * var_eg);
        let assign6680_e6628: f64 = (assign6680_e6626 * var_inv_phit0);
        var_eg_2phit0 = assign6680_e6628;
        var_eg_2phit0_dn4 = (((0.5 * var_eg_dn4) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn4));
        var_eg_2phit0_dn6 = (((0.5 * var_eg_dn6) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn6));
        var_eg_2phit0_dn7 = (((0.5 * var_eg_dn7) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn7));
        var_eg_2phit0_dn8 = (((0.5 * var_eg_dn8) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn8));
        var_eg_2phit0_dn9 = (((0.5 * var_eg_dn9) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn9));
        var_eg_2phit0_rv = 0.0;

        var_eg_2phit0_woshe = var_eg_2phit0;
        var_eg_2phit0_woshe_dn4 = var_eg_2phit0_dn4;
        var_eg_2phit0_woshe_dn6 = var_eg_2phit0_dn6;
        var_eg_2phit0_woshe_dn7 = var_eg_2phit0_dn7;
        var_eg_2phit0_woshe_dn8 = var_eg_2phit0_dn8;
        var_eg_2phit0_woshe_dn9 = var_eg_2phit0_dn9;
        var_eg_2phit0_woshe_rv = 0.0;

        let assign6700_e6634: f64 = (10.0 * var_xge_i);
        let assign6700_e6635: f64 = (assign6700_e6634).sqrt();
        let assign6700_e6636: f64 = (1.0 + assign6700_e6635);
        let assign6700_e6637: f64 = (1.0 / assign6700_e6636);
        var_niratio = assign6700_e6637;
        var_niratio_rv = 0.0;

        let assign6710_e6640: f64 = (0.05 * var_xge_i);
        let assign6710_e6643: f64 = (0.5 * var_deg);
        let assign6710_e6644: f64 = (assign6710_e6640 - assign6710_e6643);
        var_dvfbch = assign6710_e6644;
        var_dvfbch_dn4 = (-(0.5 * var_deg_dn4));
        var_dvfbch_dn6 = (-(0.5 * var_deg_dn6));
        var_dvfbch_dn7 = (-(0.5 * var_deg_dn7));
        var_dvfbch_dn8 = (-(0.5 * var_deg_dn8));
        var_dvfbch_dn9 = (-(0.5 * var_deg_dn9));
        var_dvfbch_rv = 0.0;

        let assign6720_e6647: f64 = (1.602176565e-19 * var_nch_i);
        let assign6720_e6649: f64 = (assign6720_e6647 * 0.5);
        let assign6720_e6651: f64 = (assign6720_e6649 * var_tsi_i);
        let assign6720_e6653: f64 = (assign6720_e6651 / 3.45313e-11);
        var_temp = assign6720_e6653;
        var_temp_dn4 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;
        var_temp_rv = 0.0;

        let assign6730_e6656: f64 = if var_typech_i > 0.0 { 1.0 } else { 0.0 };
        var_guard140 = assign6730_e6656;
        var_guard140_rv = 0.0;

        let (assign6740_e6666, assign6740_e6666_d_n4, assign6740_e6666_d_n6, assign6740_e6666_d_n7, assign6740_e6666_d_n8, assign6740_e6666_d_n9,) = {
    if (var_guard140 != 0.0) {
        let assign6740_e6662: f64 = (p.p13 * 4e-10);
        let assign6740_e6663: f64 = (var_tox1_i + assign6740_e6662);
        let assign6740_e6664: f64 = (var_temp * assign6740_e6663);
        (assign6740_e6664, (var_temp_dn4 * assign6740_e6663), (var_temp_dn6 * assign6740_e6663), (var_temp_dn7 * assign6740_e6663), (var_temp_dn8 * assign6740_e6663), (var_temp_dn9 * assign6740_e6663),)
    } else {
        (var_dvfb1nch, var_dvfb1nch_dn4, var_dvfb1nch_dn6, var_dvfb1nch_dn7, var_dvfb1nch_dn8, var_dvfb1nch_dn9,)
    }
};
        var_dvfb1nch = assign6740_e6666;
        var_dvfb1nch_dn4 = assign6740_e6666_d_n4;
        var_dvfb1nch_dn6 = assign6740_e6666_d_n6;
        var_dvfb1nch_dn7 = assign6740_e6666_d_n7;
        var_dvfb1nch_dn8 = assign6740_e6666_d_n8;
        var_dvfb1nch_dn9 = assign6740_e6666_d_n9;
        var_dvfb1nch_rv = 0.0;

        let (assign6750_e6676, assign6750_e6676_d_n4, assign6750_e6676_d_n6, assign6750_e6676_d_n7, assign6750_e6676_d_n8, assign6750_e6676_d_n9,) = {
    if (var_guard140 != 0.0) {
        let assign6750_e6672: f64 = (p.p13 * 4e-10);
        let assign6750_e6673: f64 = (var_tox2_i + assign6750_e6672);
        let assign6750_e6674: f64 = (var_temp * assign6750_e6673);
        (assign6750_e6674, (var_temp_dn4 * assign6750_e6673), (var_temp_dn6 * assign6750_e6673), (var_temp_dn7 * assign6750_e6673), (var_temp_dn8 * assign6750_e6673), (var_temp_dn9 * assign6750_e6673),)
    } else {
        (var_dvfb2nch, var_dvfb2nch_dn4, var_dvfb2nch_dn6, var_dvfb2nch_dn7, var_dvfb2nch_dn8, var_dvfb2nch_dn9,)
    }
};
        var_dvfb2nch = assign6750_e6676;
        var_dvfb2nch_dn4 = assign6750_e6676_d_n4;
        var_dvfb2nch_dn6 = assign6750_e6676_d_n6;
        var_dvfb2nch_dn7 = assign6750_e6676_d_n7;
        var_dvfb2nch_dn8 = assign6750_e6676_d_n8;
        var_dvfb2nch_dn9 = assign6750_e6676_d_n9;
        var_dvfb2nch_rv = 0.0;

        let (assign6760_e6688, assign6760_e6688_d_n4, assign6760_e6688_d_n6, assign6760_e6688_d_n7, assign6760_e6688_d_n8, assign6760_e6688_d_n9,) = {
    if (var_guard140 == 0.0) {
        let assign6760_e6680: f64 = (-var_temp);
        let assign6760_e6684: f64 = (p.p13 * 4e-10);
        let assign6760_e6685: f64 = (var_tox1_i + assign6760_e6684);
        let assign6760_e6686: f64 = (assign6760_e6680 * assign6760_e6685);
        (assign6760_e6686, ((-var_temp_dn4) * assign6760_e6685), ((-var_temp_dn6) * assign6760_e6685), ((-var_temp_dn7) * assign6760_e6685), ((-var_temp_dn8) * assign6760_e6685), ((-var_temp_dn9) * assign6760_e6685),)
    } else {
        (var_dvfb1nch, var_dvfb1nch_dn4, var_dvfb1nch_dn6, var_dvfb1nch_dn7, var_dvfb1nch_dn8, var_dvfb1nch_dn9,)
    }
};
        var_dvfb1nch = assign6760_e6688;
        var_dvfb1nch_dn4 = assign6760_e6688_d_n4;
        var_dvfb1nch_dn6 = assign6760_e6688_d_n6;
        var_dvfb1nch_dn7 = assign6760_e6688_d_n7;
        var_dvfb1nch_dn8 = assign6760_e6688_d_n8;
        var_dvfb1nch_dn9 = assign6760_e6688_d_n9;
        var_dvfb1nch_rv = 0.0;

        let (assign6770_e6700, assign6770_e6700_d_n4, assign6770_e6700_d_n6, assign6770_e6700_d_n7, assign6770_e6700_d_n8, assign6770_e6700_d_n9,) = {
    if (var_guard140 == 0.0) {
        let assign6770_e6692: f64 = (-var_temp);
        let assign6770_e6696: f64 = (p.p13 * 4e-10);
        let assign6770_e6697: f64 = (var_tox2_i + assign6770_e6696);
        let assign6770_e6698: f64 = (assign6770_e6692 * assign6770_e6697);
        (assign6770_e6698, ((-var_temp_dn4) * assign6770_e6697), ((-var_temp_dn6) * assign6770_e6697), ((-var_temp_dn7) * assign6770_e6697), ((-var_temp_dn8) * assign6770_e6697), ((-var_temp_dn9) * assign6770_e6697),)
    } else {
        (var_dvfb2nch, var_dvfb2nch_dn4, var_dvfb2nch_dn6, var_dvfb2nch_dn7, var_dvfb2nch_dn8, var_dvfb2nch_dn9,)
    }
};
        var_dvfb2nch = assign6770_e6700;
        var_dvfb2nch_dn4 = assign6770_e6700_d_n4;
        var_dvfb2nch_dn6 = assign6770_e6700_d_n6;
        var_dvfb2nch_dn7 = assign6770_e6700_d_n7;
        var_dvfb2nch_dn8 = assign6770_e6700_d_n8;
        var_dvfb2nch_dn9 = assign6770_e6700_d_n9;
        var_dvfb2nch_rv = 0.0;

        let assign6780_e6703: f64 = (var_tkc * 0.0033333333333);
        let assign6780_e6704: f64 = (assign6780_e6703).sqrt();
        var_temp = assign6780_e6704;
        var_temp_dn4 = ((var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn6 = ((var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn7 = ((var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn8 = ((var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn9 = ((var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_rv = 0.0;

        let assign6790_e6707: f64 = (4.05e25 * var_temp);
        let assign6790_e6709: f64 = (assign6790_e6707 * var_temp);
        let assign6790_e6711: f64 = (assign6790_e6709 * var_temp);
        var_temp1 = assign6790_e6711;
        var_temp1_dn4 = (((((4.05e25 * var_temp_dn4) * var_temp) + (assign6790_e6707 * var_temp_dn4)) * var_temp) + (assign6790_e6709 * var_temp_dn4));
        var_temp1_dn6 = (((((4.05e25 * var_temp_dn6) * var_temp) + (assign6790_e6707 * var_temp_dn6)) * var_temp) + (assign6790_e6709 * var_temp_dn6));
        var_temp1_dn7 = (((((4.05e25 * var_temp_dn7) * var_temp) + (assign6790_e6707 * var_temp_dn7)) * var_temp) + (assign6790_e6709 * var_temp_dn7));
        var_temp1_dn8 = (((((4.05e25 * var_temp_dn8) * var_temp) + (assign6790_e6707 * var_temp_dn8)) * var_temp) + (assign6790_e6709 * var_temp_dn8));
        var_temp1_dn9 = (((((4.05e25 * var_temp_dn9) * var_temp) + (assign6790_e6707 * var_temp_dn9)) * var_temp) + (assign6790_e6709 * var_temp_dn9));
        var_temp1_rv = 0.0;

        let assign6800_e6714: f64 = (var_temp1 * var_niratio);
        var_neff = assign6800_e6714;
        var_neff_dn4 = (var_temp1_dn4 * var_niratio);
        var_neff_dn6 = (var_temp1_dn6 * var_niratio);
        var_neff_dn7 = (var_temp1_dn7 * var_niratio);
        var_neff_dn8 = (var_temp1_dn8 * var_niratio);
        var_neff_dn9 = (var_temp1_dn9 * var_niratio);
        var_neff_rv = 0.0;

        let assign6810_e6718: f64 = (0.5 * var_deg);
        let assign6810_e6720: f64 = (assign6810_e6718 * var_inv_phit0);
        let assign6810_e6721: f64 = (assign6810_e6720).exp();
        let assign6810_e6722: f64 = (var_temp1 * assign6810_e6721);
        var_neff_poly = assign6810_e6722;
        var_neff_poly_dn4 = ((var_temp1_dn4 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn4) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn4)))));
        var_neff_poly_dn6 = ((var_temp1_dn6 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn6) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn6)))));
        var_neff_poly_dn7 = ((var_temp1_dn7 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn7) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn7)))));
        var_neff_poly_dn8 = ((var_temp1_dn8 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn8) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn8)))));
        var_neff_poly_dn9 = ((var_temp1_dn9 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn9) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn9)))));
        var_neff_poly_rv = 0.0;

        let assign6820_e6726: f64 = (0.5 * var_deg);
        let assign6820_e6728: f64 = (assign6820_e6726 * var_inv_phit0);
        let assign6820_e6729: f64 = (assign6820_e6728).exp();
        let assign6820_e6730: f64 = (var_temp1 * assign6820_e6729);
        var_neff_sub = assign6820_e6730;
        var_neff_sub_dn4 = ((var_temp1_dn4 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn4) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn4)))));
        var_neff_sub_dn6 = ((var_temp1_dn6 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn6) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn6)))));
        var_neff_sub_dn7 = ((var_temp1_dn7 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn7) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn7)))));
        var_neff_sub_dn8 = ((var_temp1_dn8 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn8) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn8)))));
        var_neff_sub_dn9 = ((var_temp1_dn9 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn9) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn9)))));
        var_neff_sub_rv = 0.0;

        let assign6830_e6733: f64 = (3.45313e-11 / var_tox1_i);
        var_cox1init = assign6830_e6733;
        var_cox1init_rv = 0.0;

        let assign6840_e6736: f64 = (3.45313e-11 / var_tox2_i);
        var_cox2init = assign6840_e6736;
        var_cox2init_rv = 0.0;

        let assign6850_e6739: f64 = if var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        var_guard141 = assign6850_e6739;
        var_guard141_rv = 0.0;

        let (assign6860_e6747,) = {
    if (var_guard141 != 0.0) {
        let assign6860_e6744: f64 = (1.0 + var_pnce_i);
        let assign6860_e6745: f64 = (var_cox1init * assign6860_e6744);
        (assign6860_e6745,)
    } else {
        (var_cox1prime,)
    }
};
        var_cox1prime = assign6860_e6747;
        var_cox1prime_rv = 0.0;

        let (assign6870_e6751,) = {
    if (var_guard141 != 0.0) {
        (var_cox2init,)
    } else {
        (var_cox2prime,)
    }
};
        var_cox2prime = assign6870_e6751;
        var_cox2prime_rv = 0.0;

        let (assign6880_e6756,) = {
    if (var_guard141 == 0.0) {
        (var_cox1init,)
    } else {
        (var_cox1prime,)
    }
};
        var_cox1prime = assign6880_e6756;
        var_cox1prime_rv = 0.0;

        let (assign6890_e6765,) = {
    if (var_guard141 == 0.0) {
        let assign6890_e6762: f64 = (1.0 - var_pnce_i);
        let assign6890_e6763: f64 = (var_cox2init * assign6890_e6762);
        (assign6890_e6763,)
    } else {
        (var_cox2prime,)
    }
};
        var_cox2prime = assign6890_e6765;
        var_cox2prime_rv = 0.0;

        let assign6900_e6768: f64 = (var_epsch / var_tsi_i);
        var_csiprime_0 = assign6900_e6768;
        var_csiprime_0_rv = 0.0;

        let assign6910_e6773: f64 = (var_ct_i * var_rtn);
        let assign6910_e6774: f64 = (1.0 + assign6910_e6773);
        let assign6910_e6775: f64 = (var_phit0 * assign6910_e6774);
        var_phit = assign6910_e6775;
        var_phit_dn4 = ((var_phit0_dn4 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn4)));
        var_phit_dn6 = ((var_phit0_dn6 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn6)));
        var_phit_dn7 = ((var_phit0_dn7 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn7)));
        var_phit_dn8 = ((var_phit0_dn8 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn8)));
        var_phit_dn9 = ((var_phit0_dn9 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn9)));
        var_phit_rv = 0.0;

        let assign6920_e6778: f64 = (1.0 / var_phit);
        var_inv_phit = assign6920_e6778;
        var_inv_phit_dn4 = (-(var_phit_dn4 / (var_phit * var_phit)));
        var_inv_phit_dn6 = (-(var_phit_dn6 / (var_phit * var_phit)));
        var_inv_phit_dn7 = (-(var_phit_dn7 / (var_phit * var_phit)));
        var_inv_phit_dn8 = (-(var_phit_dn8 / (var_phit * var_phit)));
        var_inv_phit_dn9 = (-(var_phit_dn9 / (var_phit * var_phit)));
        var_inv_phit_rv = 0.0;

        let assign6930_e6781: f64 = (0.5 * var_eg);
        let assign6930_e6783: f64 = (assign6930_e6781 * var_inv_phit);
        var_eg_2phit = assign6930_e6783;
        var_eg_2phit_dn4 = (((0.5 * var_eg_dn4) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn4));
        var_eg_2phit_dn6 = (((0.5 * var_eg_dn6) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn6));
        var_eg_2phit_dn7 = (((0.5 * var_eg_dn7) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn7));
        var_eg_2phit_dn8 = (((0.5 * var_eg_dn8) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn8));
        var_eg_2phit_dn9 = (((0.5 * var_eg_dn9) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn9));
        var_eg_2phit_rv = 0.0;

        let assign6940_e6786: f64 = (var_cox1prime / var_csiprime_0);
        var_k1_1d = assign6940_e6786;
        var_k1_1d_rv = 0.0;

        let assign6950_e6789: f64 = (var_cox2prime / var_csiprime_0);
        var_k2_1d = assign6950_e6789;
        var_k2_1d_rv = 0.0;

        let assign6960_e6794: f64 = (1.0 / var_k1_1d);
        let assign6960_e6795: f64 = (1.0 + assign6960_e6794);
        let assign6960_e6798: f64 = (1.0 / var_k2_1d);
        let assign6960_e6799: f64 = (assign6960_e6795 + assign6960_e6798);
        let assign6960_e6800: f64 = (1.0 / assign6960_e6799);
        var_keq_1d = assign6960_e6800;
        var_keq_1d_rv = 0.0;

        let assign6970_e6803: f64 = (2.0 * 1.602176565e-19);
        let assign6970_e6805: f64 = (assign6970_e6803 * var_neff);
        let assign6970_e6807: f64 = (assign6970_e6805 * var_epsch);
        let assign6970_e6809: f64 = (assign6970_e6807 * var_inv_phit);
        var_a0_csisq = assign6970_e6809;
        var_a0_csisq_dn4 = ((((assign6970_e6803 * var_neff_dn4) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn4));
        var_a0_csisq_dn6 = ((((assign6970_e6803 * var_neff_dn6) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn6));
        var_a0_csisq_dn7 = ((((assign6970_e6803 * var_neff_dn7) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn7));
        var_a0_csisq_dn8 = ((((assign6970_e6803 * var_neff_dn8) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn8));
        var_a0_csisq_dn9 = ((((assign6970_e6803 * var_neff_dn9) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn9));
        var_a0_csisq_rv = 0.0;

        let assign6980_e6812: f64 = (var_csiprime_0 * var_csiprime_0);
        let assign6980_e6814: f64 = (assign6980_e6812 / var_a0_csisq);
        let assign6980_e6815: f64 = (assign6980_e6814).ln();
        let assign6980_e6817: f64 = (assign6980_e6815 - 0.6931471805599);
        var_xth_1d = assign6980_e6817;
        var_xth_1d_dn4 = ((-((assign6980_e6812 * var_a0_csisq_dn4) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn6 = ((-((assign6980_e6812 * var_a0_csisq_dn6) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn7 = ((-((assign6980_e6812 * var_a0_csisq_dn7) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn8 = ((-((assign6980_e6812 * var_a0_csisq_dn8) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn9 = ((-((assign6980_e6812 * var_a0_csisq_dn9) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_rv = 0.0;

        let assign6990_e6820: f64 = (0.5 * 1.602176565e-19);
        let assign6990_e6822: f64 = (assign6990_e6820 * var_nsddc_i);
        let assign6990_e6824: f64 = (assign6990_e6822 * var_tsi_i);
        let assign6990_e6827: f64 = (var_cox1prime + var_cox2prime);
        let assign6990_e6828: f64 = (assign6990_e6824 / assign6990_e6827);
        let assign6990_e6830: f64 = (assign6990_e6828 * var_inv_phit);
        var_xsddep = assign6990_e6830;
        var_xsddep_dn4 = (assign6990_e6828 * var_inv_phit_dn4);
        var_xsddep_dn6 = (assign6990_e6828 * var_inv_phit_dn6);
        var_xsddep_dn7 = (assign6990_e6828 * var_inv_phit_dn7);
        var_xsddep_dn8 = (assign6990_e6828 * var_inv_phit_dn8);
        var_xsddep_dn9 = (assign6990_e6828 * var_inv_phit_dn9);
        var_xsddep_rv = 0.0;

        let assign7000_e6833: f64 = (var_stcf_i * var_dt);
        var_temp = assign7000_e6833;
        var_temp_dn4 = ((var_stcf_i_dn4 * var_dt) + (var_stcf_i * var_dt_dn4));
        var_temp_dn6 = ((var_stcf_i_dn6 * var_dt) + (var_stcf_i * var_dt_dn6));
        var_temp_dn7 = ((var_stcf_i_dn7 * var_dt) + (var_stcf_i * var_dt_dn7));
        var_temp_dn8 = ((var_stcf_i_dn8 * var_dt) + (var_stcf_i * var_dt_dn8));
        var_temp_dn9 = ((var_stcf_i_dn9 * var_dt) + (var_stcf_i * var_dt_dn9));
        var_temp_rv = 0.0;

        let assign7010_e6836: f64 = (var_cf1_t + var_temp);
        var_cf1_i = assign7010_e6836;
        var_cf1_i_dn4 = (var_cf1_t_dn4 + var_temp_dn4);
        var_cf1_i_dn6 = (var_cf1_t_dn6 + var_temp_dn6);
        var_cf1_i_dn7 = (var_cf1_t_dn7 + var_temp_dn7);
        var_cf1_i_dn8 = (var_cf1_t_dn8 + var_temp_dn8);
        var_cf1_i_dn9 = (var_cf1_t_dn9 + var_temp_dn9);
        var_cf1_i_rv = 0.0;

        let assign7020_e6839: f64 = (var_cf2_t + var_temp);
        var_cf2_i = assign7020_e6839;
        var_cf2_i_dn4 = (var_cf2_t_dn4 + var_temp_dn4);
        var_cf2_i_dn6 = (var_cf2_t_dn6 + var_temp_dn6);
        var_cf2_i_dn7 = (var_cf2_t_dn7 + var_temp_dn7);
        var_cf2_i_dn8 = (var_cf2_t_dn8 + var_temp_dn8);
        var_cf2_i_dn9 = (var_cf2_t_dn9 + var_temp_dn9);
        var_cf2_i_rv = 0.0;

        let assign7030_e6842: f64 = (var_cfac1_t + var_temp);
        var_cfac1_i = assign7030_e6842;
        var_cfac1_i_dn4 = (var_cfac1_t_dn4 + var_temp_dn4);
        var_cfac1_i_dn6 = (var_cfac1_t_dn6 + var_temp_dn6);
        var_cfac1_i_dn7 = (var_cfac1_t_dn7 + var_temp_dn7);
        var_cfac1_i_dn8 = (var_cfac1_t_dn8 + var_temp_dn8);
        var_cfac1_i_dn9 = (var_cfac1_t_dn9 + var_temp_dn9);
        var_cfac1_i_rv = 0.0;

        let assign7040_e6845: f64 = (var_cfac2_t + var_temp);
        var_cfac2_i = assign7040_e6845;
        var_cfac2_i_dn4 = (var_cfac2_t_dn4 + var_temp_dn4);
        var_cfac2_i_dn6 = (var_cfac2_t_dn6 + var_temp_dn6);
        var_cfac2_i_dn7 = (var_cfac2_t_dn7 + var_temp_dn7);
        var_cfac2_i_dn8 = (var_cfac2_t_dn8 + var_temp_dn8);
        var_cfac2_i_dn9 = (var_cfac2_t_dn9 + var_temp_dn9);
        var_cfac2_i_rv = 0.0;

        let assign7050_e6848: f64 = (var_cfd_i * var_inv_phit);
        var_xd0 = assign7050_e6848;
        var_xd0_dn4 = (var_cfd_i * var_inv_phit_dn4);
        var_xd0_dn6 = (var_cfd_i * var_inv_phit_dn6);
        var_xd0_dn7 = (var_cfd_i * var_inv_phit_dn7);
        var_xd0_dn8 = (var_cfd_i * var_inv_phit_dn8);
        var_xd0_dn9 = (var_cfd_i * var_inv_phit_dn9);
        var_xd0_rv = 0.0;

        let assign7060_e6851: f64 = (2.0 * 1.602176565e-19);
        let assign7060_e6853: f64 = (assign7060_e6851 * var_nsub_i);
        let assign7060_e6855: f64 = (assign7060_e6853 * 1.04479e-10);
        let assign7060_e6857: f64 = (assign7060_e6855 * var_inv_phit0);
        let assign7060_e6858: f64 = (assign7060_e6857).sqrt();
        let assign7060_e6860: f64 = (assign7060_e6858 / var_cox2prime);
        var_gfsub = assign7060_e6860;
        var_gfsub_dn4 = (((assign7060_e6855 * var_inv_phit0_dn4) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn6 = (((assign7060_e6855 * var_inv_phit0_dn6) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn7 = (((assign7060_e6855 * var_inv_phit0_dn7) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn8 = (((assign7060_e6855 * var_inv_phit0_dn8) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn9 = (((assign7060_e6855 * var_inv_phit0_dn9) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_rv = 0.0;

        let assign7070_e6863: f64 = (var_gfsub * var_gfsub);
        var_gfsub2 = assign7070_e6863;
        var_gfsub2_dn4 = ((var_gfsub_dn4 * var_gfsub) + (var_gfsub * var_gfsub_dn4));
        var_gfsub2_dn6 = ((var_gfsub_dn6 * var_gfsub) + (var_gfsub * var_gfsub_dn6));
        var_gfsub2_dn7 = ((var_gfsub_dn7 * var_gfsub) + (var_gfsub * var_gfsub_dn7));
        var_gfsub2_dn8 = ((var_gfsub_dn8 * var_gfsub) + (var_gfsub * var_gfsub_dn8));
        var_gfsub2_dn9 = ((var_gfsub_dn9 * var_gfsub) + (var_gfsub * var_gfsub_dn9));
        var_gfsub2_rv = 0.0;

        let assign7080_e6866: f64 = (1.0 / var_gfsub2);
        var_inv_gfsub2 = assign7080_e6866;
        var_inv_gfsub2_dn4 = (-(var_gfsub2_dn4 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn6 = (-(var_gfsub2_dn6 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn7 = (-(var_gfsub2_dn7 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn8 = (-(var_gfsub2_dn8 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn9 = (-(var_gfsub2_dn9 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_rv = 0.0;

        let assign7090_e6870: f64 = (var_gfsub / 1.4142135623731);
        let assign7090_e6871: f64 = (1.0 + assign7090_e6870);
        var_xisub = assign7090_e6871;
        var_xisub_dn4 = (var_gfsub_dn4 / 1.4142135623731);
        var_xisub_dn6 = (var_gfsub_dn6 / 1.4142135623731);
        var_xisub_dn7 = (var_gfsub_dn7 / 1.4142135623731);
        var_xisub_dn8 = (var_gfsub_dn8 / 1.4142135623731);
        var_xisub_dn9 = (var_gfsub_dn9 / 1.4142135623731);
        var_xisub_rv = 0.0;

        let assign7100_e6874: f64 = (1.0 / var_xisub);
        var_inv_xisub = assign7100_e6874;
        var_inv_xisub_dn4 = (-(var_xisub_dn4 / (var_xisub * var_xisub)));
        var_inv_xisub_dn6 = (-(var_xisub_dn6 / (var_xisub * var_xisub)));
        var_inv_xisub_dn7 = (-(var_xisub_dn7 / (var_xisub * var_xisub)));
        var_inv_xisub_dn8 = (-(var_xisub_dn8 / (var_xisub * var_xisub)));
        var_inv_xisub_dn9 = (-(var_xisub_dn9 / (var_xisub * var_xisub)));
        var_inv_xisub_rv = 0.0;

        let assign7110_e6877: f64 = (1e-5 * var_xisub);
        var_margin_sub = assign7110_e6877;
        var_margin_sub_dn4 = (1e-5 * var_xisub_dn4);
        var_margin_sub_dn6 = (1e-5 * var_xisub_dn6);
        var_margin_sub_dn7 = (1e-5 * var_xisub_dn7);
        var_margin_sub_dn8 = (1e-5 * var_xisub_dn8);
        var_margin_sub_dn9 = (1e-5 * var_xisub_dn9);
        var_margin_sub_rv = 0.0;

        *var_a0_csisq_slot = var_a0_csisq;
        *var_a0_csisq_dn4_slot = var_a0_csisq_dn4;
        *var_a0_csisq_dn6_slot = var_a0_csisq_dn6;
        *var_a0_csisq_dn7_slot = var_a0_csisq_dn7;
        *var_a0_csisq_dn8_slot = var_a0_csisq_dn8;
        *var_a0_csisq_dn9_slot = var_a0_csisq_dn9;
        *var_a0_csisq_rv_slot = var_a0_csisq_rv;
        *var_cf1_i_slot = var_cf1_i;
        *var_cf1_i_dn4_slot = var_cf1_i_dn4;
        *var_cf1_i_dn6_slot = var_cf1_i_dn6;
        *var_cf1_i_dn7_slot = var_cf1_i_dn7;
        *var_cf1_i_dn8_slot = var_cf1_i_dn8;
        *var_cf1_i_dn9_slot = var_cf1_i_dn9;
        *var_cf1_i_rv_slot = var_cf1_i_rv;
        *var_cf2_i_slot = var_cf2_i;
        *var_cf2_i_dn4_slot = var_cf2_i_dn4;
        *var_cf2_i_dn6_slot = var_cf2_i_dn6;
        *var_cf2_i_dn7_slot = var_cf2_i_dn7;
        *var_cf2_i_dn8_slot = var_cf2_i_dn8;
        *var_cf2_i_dn9_slot = var_cf2_i_dn9;
        *var_cf2_i_rv_slot = var_cf2_i_rv;
        *var_cfac1_i_slot = var_cfac1_i;
        *var_cfac1_i_dn4_slot = var_cfac1_i_dn4;
        *var_cfac1_i_dn6_slot = var_cfac1_i_dn6;
        *var_cfac1_i_dn7_slot = var_cfac1_i_dn7;
        *var_cfac1_i_dn8_slot = var_cfac1_i_dn8;
        *var_cfac1_i_dn9_slot = var_cfac1_i_dn9;
        *var_cfac1_i_rv_slot = var_cfac1_i_rv;
        *var_cfac2_i_slot = var_cfac2_i;
        *var_cfac2_i_dn4_slot = var_cfac2_i_dn4;
        *var_cfac2_i_dn6_slot = var_cfac2_i_dn6;
        *var_cfac2_i_dn7_slot = var_cfac2_i_dn7;
        *var_cfac2_i_dn8_slot = var_cfac2_i_dn8;
        *var_cfac2_i_dn9_slot = var_cfac2_i_dn9;
        *var_cfac2_i_rv_slot = var_cfac2_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cox1init_slot = var_cox1init;
        *var_cox1init_rv_slot = var_cox1init_rv;
        *var_cox1prime_slot = var_cox1prime;
        *var_cox1prime_rv_slot = var_cox1prime_rv;
        *var_cox2init_slot = var_cox2init;
        *var_cox2init_rv_slot = var_cox2init_rv;
        *var_cox2prime_slot = var_cox2prime;
        *var_cox2prime_rv_slot = var_cox2prime_rv;
        *var_csiprime_0_slot = var_csiprime_0;
        *var_csiprime_0_rv_slot = var_csiprime_0_rv;
        *var_deg_slot = var_deg;
        *var_deg_dn4_slot = var_deg_dn4;
        *var_deg_dn6_slot = var_deg_dn6;
        *var_deg_dn7_slot = var_deg_dn7;
        *var_deg_dn8_slot = var_deg_dn8;
        *var_deg_dn9_slot = var_deg_dn9;
        *var_deg_rv_slot = var_deg_rv;
        *var_dvfb1nch_slot = var_dvfb1nch;
        *var_dvfb1nch_dn4_slot = var_dvfb1nch_dn4;
        *var_dvfb1nch_dn6_slot = var_dvfb1nch_dn6;
        *var_dvfb1nch_dn7_slot = var_dvfb1nch_dn7;
        *var_dvfb1nch_dn8_slot = var_dvfb1nch_dn8;
        *var_dvfb1nch_dn9_slot = var_dvfb1nch_dn9;
        *var_dvfb1nch_rv_slot = var_dvfb1nch_rv;
        *var_dvfb2nch_slot = var_dvfb2nch;
        *var_dvfb2nch_dn4_slot = var_dvfb2nch_dn4;
        *var_dvfb2nch_dn6_slot = var_dvfb2nch_dn6;
        *var_dvfb2nch_dn7_slot = var_dvfb2nch_dn7;
        *var_dvfb2nch_dn8_slot = var_dvfb2nch_dn8;
        *var_dvfb2nch_dn9_slot = var_dvfb2nch_dn9;
        *var_dvfb2nch_rv_slot = var_dvfb2nch_rv;
        *var_dvfbch_slot = var_dvfbch;
        *var_dvfbch_dn4_slot = var_dvfbch_dn4;
        *var_dvfbch_dn6_slot = var_dvfbch_dn6;
        *var_dvfbch_dn7_slot = var_dvfbch_dn7;
        *var_dvfbch_dn8_slot = var_dvfbch_dn8;
        *var_dvfbch_dn9_slot = var_dvfbch_dn9;
        *var_dvfbch_rv_slot = var_dvfbch_rv;
        *var_eg_slot = var_eg;
        *var_eg_2phit_slot = var_eg_2phit;
        *var_eg_2phit0_slot = var_eg_2phit0;
        *var_eg_2phit0_dn4_slot = var_eg_2phit0_dn4;
        *var_eg_2phit0_dn6_slot = var_eg_2phit0_dn6;
        *var_eg_2phit0_dn7_slot = var_eg_2phit0_dn7;
        *var_eg_2phit0_dn8_slot = var_eg_2phit0_dn8;
        *var_eg_2phit0_dn9_slot = var_eg_2phit0_dn9;
        *var_eg_2phit0_rv_slot = var_eg_2phit0_rv;
        *var_eg_2phit0_woshe_slot = var_eg_2phit0_woshe;
        *var_eg_2phit0_woshe_dn4_slot = var_eg_2phit0_woshe_dn4;
        *var_eg_2phit0_woshe_dn6_slot = var_eg_2phit0_woshe_dn6;
        *var_eg_2phit0_woshe_dn7_slot = var_eg_2phit0_woshe_dn7;
        *var_eg_2phit0_woshe_dn8_slot = var_eg_2phit0_woshe_dn8;
        *var_eg_2phit0_woshe_dn9_slot = var_eg_2phit0_woshe_dn9;
        *var_eg_2phit0_woshe_rv_slot = var_eg_2phit0_woshe_rv;
        *var_eg_2phit_dn4_slot = var_eg_2phit_dn4;
        *var_eg_2phit_dn6_slot = var_eg_2phit_dn6;
        *var_eg_2phit_dn7_slot = var_eg_2phit_dn7;
        *var_eg_2phit_dn8_slot = var_eg_2phit_dn8;
        *var_eg_2phit_dn9_slot = var_eg_2phit_dn9;
        *var_eg_2phit_rv_slot = var_eg_2phit_rv;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_dn8_slot = var_eg_dn8;
        *var_eg_dn9_slot = var_eg_dn9;
        *var_eg_rv_slot = var_eg_rv;
        *var_egge_slot = var_egge;
        *var_egge_dn4_slot = var_egge_dn4;
        *var_egge_dn6_slot = var_egge_dn6;
        *var_egge_dn7_slot = var_egge_dn7;
        *var_egge_dn8_slot = var_egge_dn8;
        *var_egge_dn9_slot = var_egge_dn9;
        *var_egge_rv_slot = var_egge_rv;
        *var_egsi_slot = var_egsi;
        *var_egsi_dn4_slot = var_egsi_dn4;
        *var_egsi_dn6_slot = var_egsi_dn6;
        *var_egsi_dn7_slot = var_egsi_dn7;
        *var_egsi_dn8_slot = var_egsi_dn8;
        *var_egsi_dn9_slot = var_egsi_dn9;
        *var_egsi_rv_slot = var_egsi_rv;
        *var_epsch_slot = var_epsch;
        *var_epsch_rv_slot = var_epsch_rv;
        *var_gfsub_slot = var_gfsub;
        *var_gfsub2_slot = var_gfsub2;
        *var_gfsub2_dn4_slot = var_gfsub2_dn4;
        *var_gfsub2_dn6_slot = var_gfsub2_dn6;
        *var_gfsub2_dn7_slot = var_gfsub2_dn7;
        *var_gfsub2_dn8_slot = var_gfsub2_dn8;
        *var_gfsub2_dn9_slot = var_gfsub2_dn9;
        *var_gfsub2_rv_slot = var_gfsub2_rv;
        *var_gfsub_dn4_slot = var_gfsub_dn4;
        *var_gfsub_dn6_slot = var_gfsub_dn6;
        *var_gfsub_dn7_slot = var_gfsub_dn7;
        *var_gfsub_dn8_slot = var_gfsub_dn8;
        *var_gfsub_dn9_slot = var_gfsub_dn9;
        *var_gfsub_rv_slot = var_gfsub_rv;
        *var_guard140_slot = var_guard140;
        *var_guard140_rv_slot = var_guard140_rv;
        *var_guard141_slot = var_guard141;
        *var_guard141_rv_slot = var_guard141_rv;
        *var_inv_gfsub2_slot = var_inv_gfsub2;
        *var_inv_gfsub2_dn4_slot = var_inv_gfsub2_dn4;
        *var_inv_gfsub2_dn6_slot = var_inv_gfsub2_dn6;
        *var_inv_gfsub2_dn7_slot = var_inv_gfsub2_dn7;
        *var_inv_gfsub2_dn8_slot = var_inv_gfsub2_dn8;
        *var_inv_gfsub2_dn9_slot = var_inv_gfsub2_dn9;
        *var_inv_gfsub2_rv_slot = var_inv_gfsub2_rv;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_dn4_slot = var_inv_phit_dn4;
        *var_inv_phit_dn6_slot = var_inv_phit_dn6;
        *var_inv_phit_dn7_slot = var_inv_phit_dn7;
        *var_inv_phit_dn8_slot = var_inv_phit_dn8;
        *var_inv_phit_dn9_slot = var_inv_phit_dn9;
        *var_inv_phit_rv_slot = var_inv_phit_rv;
        *var_inv_xisub_slot = var_inv_xisub;
        *var_inv_xisub_dn4_slot = var_inv_xisub_dn4;
        *var_inv_xisub_dn6_slot = var_inv_xisub_dn6;
        *var_inv_xisub_dn7_slot = var_inv_xisub_dn7;
        *var_inv_xisub_dn8_slot = var_inv_xisub_dn8;
        *var_inv_xisub_dn9_slot = var_inv_xisub_dn9;
        *var_inv_xisub_rv_slot = var_inv_xisub_rv;
        *var_k1_1d_slot = var_k1_1d;
        *var_k1_1d_rv_slot = var_k1_1d_rv;
        *var_k2_1d_slot = var_k2_1d;
        *var_k2_1d_rv_slot = var_k2_1d_rv;
        *var_keq_1d_slot = var_keq_1d;
        *var_keq_1d_rv_slot = var_keq_1d_rv;
        *var_margin_sub_slot = var_margin_sub;
        *var_margin_sub_dn4_slot = var_margin_sub_dn4;
        *var_margin_sub_dn6_slot = var_margin_sub_dn6;
        *var_margin_sub_dn7_slot = var_margin_sub_dn7;
        *var_margin_sub_dn8_slot = var_margin_sub_dn8;
        *var_margin_sub_dn9_slot = var_margin_sub_dn9;
        *var_margin_sub_rv_slot = var_margin_sub_rv;
        *var_neff_slot = var_neff;
        *var_neff_dn4_slot = var_neff_dn4;
        *var_neff_dn6_slot = var_neff_dn6;
        *var_neff_dn7_slot = var_neff_dn7;
        *var_neff_dn8_slot = var_neff_dn8;
        *var_neff_dn9_slot = var_neff_dn9;
        *var_neff_poly_slot = var_neff_poly;
        *var_neff_poly_dn4_slot = var_neff_poly_dn4;
        *var_neff_poly_dn6_slot = var_neff_poly_dn6;
        *var_neff_poly_dn7_slot = var_neff_poly_dn7;
        *var_neff_poly_dn8_slot = var_neff_poly_dn8;
        *var_neff_poly_dn9_slot = var_neff_poly_dn9;
        *var_neff_poly_rv_slot = var_neff_poly_rv;
        *var_neff_rv_slot = var_neff_rv;
        *var_neff_sub_slot = var_neff_sub;
        *var_neff_sub_dn4_slot = var_neff_sub_dn4;
        *var_neff_sub_dn6_slot = var_neff_sub_dn6;
        *var_neff_sub_dn7_slot = var_neff_sub_dn7;
        *var_neff_sub_dn8_slot = var_neff_sub_dn8;
        *var_neff_sub_dn9_slot = var_neff_sub_dn9;
        *var_neff_sub_rv_slot = var_neff_sub_rv;
        *var_niratio_slot = var_niratio;
        *var_niratio_rv_slot = var_niratio_rv;
        *var_one_m_xge_slot = var_one_m_xge;
        *var_one_m_xge_rv_slot = var_one_m_xge_rv;
        *var_phit_slot = var_phit;
        *var_phit_dn4_slot = var_phit_dn4;
        *var_phit_dn6_slot = var_phit_dn6;
        *var_phit_dn7_slot = var_phit_dn7;
        *var_phit_dn8_slot = var_phit_dn8;
        *var_phit_dn9_slot = var_phit_dn9;
        *var_phit_rv_slot = var_phit_rv;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_xd0_slot = var_xd0;
        *var_xd0_dn4_slot = var_xd0_dn4;
        *var_xd0_dn6_slot = var_xd0_dn6;
        *var_xd0_dn7_slot = var_xd0_dn7;
        *var_xd0_dn8_slot = var_xd0_dn8;
        *var_xd0_dn9_slot = var_xd0_dn9;
        *var_xd0_rv_slot = var_xd0_rv;
        *var_xisub_slot = var_xisub;
        *var_xisub_dn4_slot = var_xisub_dn4;
        *var_xisub_dn6_slot = var_xisub_dn6;
        *var_xisub_dn7_slot = var_xisub_dn7;
        *var_xisub_dn8_slot = var_xisub_dn8;
        *var_xisub_dn9_slot = var_xisub_dn9;
        *var_xisub_rv_slot = var_xisub_rv;
        *var_xsddep_slot = var_xsddep;
        *var_xsddep_dn4_slot = var_xsddep_dn4;
        *var_xsddep_dn6_slot = var_xsddep_dn6;
        *var_xsddep_dn7_slot = var_xsddep_dn7;
        *var_xsddep_dn8_slot = var_xsddep_dn8;
        *var_xsddep_dn9_slot = var_xsddep_dn9;
        *var_xsddep_rv_slot = var_xsddep_rv;
        *var_xth_1d_slot = var_xth_1d;
        *var_xth_1d_dn4_slot = var_xth_1d_dn4;
        *var_xth_1d_dn6_slot = var_xth_1d_dn6;
        *var_xth_1d_dn7_slot = var_xth_1d_dn7;
        *var_xth_1d_dn8_slot = var_xth_1d_dn8;
        *var_xth_1d_dn9_slot = var_xth_1d_dn9;
        *var_xth_1d_rv_slot = var_xth_1d_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_betn1_t: f64,
        var_betn1_t_dn4: f64,
        var_betn1_t_dn6: f64,
        var_betn1_t_dn7: f64,
        var_betn1_t_dn8: f64,
        var_betn1_t_dn9: f64,
        var_betn2_t: f64,
        var_betn2_t_dn4: f64,
        var_betn2_t_dn6: f64,
        var_betn2_t_dn7: f64,
        var_betn2_t_dn8: f64,
        var_betn2_t_dn9: f64,
        var_cox1init: f64,
        var_cs_t: f64,
        var_csthr_i: f64,
        var_csthrb_i: f64,
        var_dt: f64,
        var_dt_dn4: f64,
        var_dt_dn6: f64,
        var_dt_dn7: f64,
        var_dt_dn8: f64,
        var_dt_dn9: f64,
        var_dvfb1nch: f64,
        var_dvfb1nch_dn4: f64,
        var_dvfb1nch_dn6: f64,
        var_dvfb1nch_dn7: f64,
        var_dvfb1nch_dn8: f64,
        var_dvfb1nch_dn9: f64,
        var_dvfb2nch: f64,
        var_dvfb2nch_dn4: f64,
        var_dvfb2nch_dn6: f64,
        var_dvfb2nch_dn7: f64,
        var_dvfb2nch_dn8: f64,
        var_dvfb2nch_dn9: f64,
        var_dvfbch: f64,
        var_dvfbch_dn4: f64,
        var_dvfbch_dn6: f64,
        var_dvfbch_dn7: f64,
        var_dvfbch_dn8: f64,
        var_dvfbch_dn9: f64,
        var_eg_2phit0: f64,
        var_eg_2phit0_dn4: f64,
        var_eg_2phit0_dn6: f64,
        var_eg_2phit0_dn7: f64,
        var_eg_2phit0_dn8: f64,
        var_eg_2phit0_dn9: f64,
        var_epsch: f64,
        var_feta_i: f64,
        var_mue_t: f64,
        var_neff_poly: f64,
        var_neff_poly_dn4: f64,
        var_neff_poly_dn6: f64,
        var_neff_poly_dn7: f64,
        var_neff_poly_dn8: f64,
        var_neff_poly_dn9: f64,
        var_neff_sub: f64,
        var_neff_sub_dn4: f64,
        var_neff_sub_dn6: f64,
        var_neff_sub_dn7: f64,
        var_neff_sub_dn8: f64,
        var_neff_sub_dn9: f64,
        var_np_i: f64,
        var_np_i_dn4: f64,
        var_np_i_dn6: f64,
        var_np_i_dn7: f64,
        var_np_i_dn8: f64,
        var_np_i_dn9: f64,
        var_nsub_i: f64,
        var_phit: f64,
        var_phit0: f64,
        var_phit0_dn4: f64,
        var_phit0_dn6: f64,
        var_phit0_dn7: f64,
        var_phit0_dn8: f64,
        var_phit0_dn9: f64,
        var_phit_dn4: f64,
        var_phit_dn6: f64,
        var_phit_dn7: f64,
        var_phit_dn8: f64,
        var_phit_dn9: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_rtn_dn6: f64,
        var_rtn_dn7: f64,
        var_rtn_dn8: f64,
        var_rtn_dn9: f64,
        var_stbet_i: f64,
        var_stcs_i: f64,
        var_stmue_i: f64,
        var_strs_i: f64,
        var_stthecs_i: f64,
        var_stthemu_i: f64,
        var_stvfb_i: f64,
        var_stxcor_i: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_tkd_dn6: f64,
        var_tkd_dn7: f64,
        var_tkd_dn8: f64,
        var_tkd_dn9: f64,
        var_tsi_i: f64,
        var_typesub_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfbac1_t: f64,
        var_vfbac1_t_dn4: f64,
        var_vfbac1_t_dn6: f64,
        var_vfbac1_t_dn7: f64,
        var_vfbac1_t_dn8: f64,
        var_vfbac1_t_dn9: f64,
        var_xcor_t: f64,
        var_betn1_i_slot: &mut f64,
        var_betn1_i_dn4_slot: &mut f64,
        var_betn1_i_dn6_slot: &mut f64,
        var_betn1_i_dn7_slot: &mut f64,
        var_betn1_i_dn8_slot: &mut f64,
        var_betn1_i_dn9_slot: &mut f64,
        var_betn1_i_rv_slot: &mut f64,
        var_betn2_i_slot: &mut f64,
        var_betn2_i_dn4_slot: &mut f64,
        var_betn2_i_dn6_slot: &mut f64,
        var_betn2_i_dn7_slot: &mut f64,
        var_betn2_i_dn8_slot: &mut f64,
        var_betn2_i_dn9_slot: &mut f64,
        var_betn2_i_rv_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_cs_i_dn4_slot: &mut f64,
        var_cs_i_dn6_slot: &mut f64,
        var_cs_i_dn7_slot: &mut f64,
        var_cs_i_dn8_slot: &mut f64,
        var_cs_i_dn9_slot: &mut f64,
        var_cs_i_rv_slot: &mut f64,
        var_dvfbpdep_slot: &mut f64,
        var_dvfbpdep_dn4_slot: &mut f64,
        var_dvfbpdep_dn6_slot: &mut f64,
        var_dvfbpdep_dn7_slot: &mut f64,
        var_dvfbpdep_dn8_slot: &mut f64,
        var_dvfbpdep_dn9_slot: &mut f64,
        var_dvfbpdep_rv_slot: &mut f64,
        var_dvfbqm_slot: &mut f64,
        var_dvfbqm_rv_slot: &mut f64,
        var_emin_slot: &mut f64,
        var_emin_dn4_slot: &mut f64,
        var_emin_dn6_slot: &mut f64,
        var_emin_dn7_slot: &mut f64,
        var_emin_dn8_slot: &mut f64,
        var_emin_dn9_slot: &mut f64,
        var_emin_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_fmue_slot: &mut f64,
        var_fmue_dn4_slot: &mut f64,
        var_fmue_dn6_slot: &mut f64,
        var_fmue_dn7_slot: &mut f64,
        var_fmue_dn8_slot: &mut f64,
        var_fmue_dn9_slot: &mut f64,
        var_fmue_rv_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard142_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard145_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_inv_qi1cs_slot: &mut f64,
        var_inv_qi1cs_rv_slot: &mut f64,
        var_inv_qi2cs_slot: &mut f64,
        var_inv_qi2cs_rv_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_dn4_slot: &mut f64,
        var_kp_dn6_slot: &mut f64,
        var_kp_dn7_slot: &mut f64,
        var_kp_dn8_slot: &mut f64,
        var_kp_dn9_slot: &mut f64,
        var_kp_rv_slot: &mut f64,
        var_lnrtn_slot: &mut f64,
        var_lnrtn_dn4_slot: &mut f64,
        var_lnrtn_dn6_slot: &mut f64,
        var_lnrtn_dn7_slot: &mut f64,
        var_lnrtn_dn8_slot: &mut f64,
        var_lnrtn_dn9_slot: &mut f64,
        var_lnrtn_rv_slot: &mut f64,
        var_mue_i_slot: &mut f64,
        var_mue_i_dn4_slot: &mut f64,
        var_mue_i_dn6_slot: &mut f64,
        var_mue_i_dn7_slot: &mut f64,
        var_mue_i_dn8_slot: &mut f64,
        var_mue_i_dn9_slot: &mut f64,
        var_mue_i_rv_slot: &mut f64,
        var_one_m_eta_slot: &mut f64,
        var_one_m_eta_rv_slot: &mut f64,
        var_qq_slot: &mut f64,
        var_qq_dn4_slot: &mut f64,
        var_qq_dn6_slot: &mut f64,
        var_qq_dn7_slot: &mut f64,
        var_qq_dn8_slot: &mut f64,
        var_qq_dn9_slot: &mut f64,
        var_qq_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_bet_dn4_slot: &mut f64,
        var_tf_bet_dn6_slot: &mut f64,
        var_tf_bet_dn7_slot: &mut f64,
        var_tf_bet_dn8_slot: &mut f64,
        var_tf_bet_dn9_slot: &mut f64,
        var_tf_bet_rv_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_cs_dn4_slot: &mut f64,
        var_tf_cs_dn6_slot: &mut f64,
        var_tf_cs_dn7_slot: &mut f64,
        var_tf_cs_dn8_slot: &mut f64,
        var_tf_cs_dn9_slot: &mut f64,
        var_tf_cs_rv_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_mue_dn4_slot: &mut f64,
        var_tf_mue_dn6_slot: &mut f64,
        var_tf_mue_dn7_slot: &mut f64,
        var_tf_mue_dn8_slot: &mut f64,
        var_tf_mue_dn9_slot: &mut f64,
        var_tf_mue_rv_slot: &mut f64,
        var_tf_thecs_slot: &mut f64,
        var_tf_thecs_dn4_slot: &mut f64,
        var_tf_thecs_dn6_slot: &mut f64,
        var_tf_thecs_dn7_slot: &mut f64,
        var_tf_thecs_dn8_slot: &mut f64,
        var_tf_thecs_dn9_slot: &mut f64,
        var_tf_thecs_rv_slot: &mut f64,
        var_tf_themu_slot: &mut f64,
        var_tf_themu_dn4_slot: &mut f64,
        var_tf_themu_dn6_slot: &mut f64,
        var_tf_themu_dn7_slot: &mut f64,
        var_tf_themu_dn8_slot: &mut f64,
        var_tf_themu_dn9_slot: &mut f64,
        var_tf_themu_rv_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_ther_dn4_slot: &mut f64,
        var_tf_ther_dn6_slot: &mut f64,
        var_tf_ther_dn7_slot: &mut f64,
        var_tf_ther_dn8_slot: &mut f64,
        var_tf_ther_dn9_slot: &mut f64,
        var_tf_ther_rv_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_tf_xcor_dn4_slot: &mut f64,
        var_tf_xcor_dn6_slot: &mut f64,
        var_tf_xcor_dn7_slot: &mut f64,
        var_tf_xcor_dn8_slot: &mut f64,
        var_tf_xcor_dn9_slot: &mut f64,
        var_tf_xcor_rv_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_thecs_i_dn4_slot: &mut f64,
        var_thecs_i_dn6_slot: &mut f64,
        var_thecs_i_dn7_slot: &mut f64,
        var_thecs_i_dn8_slot: &mut f64,
        var_thecs_i_dn9_slot: &mut f64,
        var_thecs_i_rv_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_themu_i_dn4_slot: &mut f64,
        var_themu_i_dn6_slot: &mut f64,
        var_themu_i_dn7_slot: &mut f64,
        var_themu_i_dn8_slot: &mut f64,
        var_themu_i_dn9_slot: &mut f64,
        var_themu_i_rv_slot: &mut f64,
        var_tsisq_slot: &mut f64,
        var_tsisq_rv_slot: &mut f64,
        var_vfb1_i_slot: &mut f64,
        var_vfb1_i_dn4_slot: &mut f64,
        var_vfb1_i_dn6_slot: &mut f64,
        var_vfb1_i_dn7_slot: &mut f64,
        var_vfb1_i_dn8_slot: &mut f64,
        var_vfb1_i_dn9_slot: &mut f64,
        var_vfb1_i_rv_slot: &mut f64,
        var_vfb2_i_slot: &mut f64,
        var_vfb2_i_dn4_slot: &mut f64,
        var_vfb2_i_dn6_slot: &mut f64,
        var_vfb2_i_dn7_slot: &mut f64,
        var_vfb2_i_dn8_slot: &mut f64,
        var_vfb2_i_dn9_slot: &mut f64,
        var_vfb2_i_rv_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfb2_t_rv_slot: &mut f64,
        var_vfbac1_i_slot: &mut f64,
        var_vfbac1_i_dn4_slot: &mut f64,
        var_vfbac1_i_dn6_slot: &mut f64,
        var_vfbac1_i_dn7_slot: &mut f64,
        var_vfbac1_i_dn8_slot: &mut f64,
        var_vfbac1_i_dn9_slot: &mut f64,
        var_vfbac1_i_rv_slot: &mut f64,
        var_vfbac2_i_slot: &mut f64,
        var_vfbac2_i_dn4_slot: &mut f64,
        var_vfbac2_i_dn6_slot: &mut f64,
        var_vfbac2_i_dn7_slot: &mut f64,
        var_vfbac2_i_dn8_slot: &mut f64,
        var_vfbac2_i_dn9_slot: &mut f64,
        var_vfbac2_i_rv_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbac2_t_rv_slot: &mut f64,
        var_xb_sub_slot: &mut f64,
        var_xb_sub_dn4_slot: &mut f64,
        var_xb_sub_dn6_slot: &mut f64,
        var_xb_sub_dn7_slot: &mut f64,
        var_xb_sub_dn8_slot: &mut f64,
        var_xb_sub_dn9_slot: &mut f64,
        var_xb_sub_rv_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
        var_xcor_i_dn4_slot: &mut f64,
        var_xcor_i_dn6_slot: &mut f64,
        var_xcor_i_dn7_slot: &mut f64,
        var_xcor_i_dn8_slot: &mut f64,
        var_xcor_i_dn9_slot: &mut f64,
        var_xcor_i_rv_slot: &mut f64,
        var_xn_sub_slot: &mut f64,
        var_xn_sub_dn4_slot: &mut f64,
        var_xn_sub_dn6_slot: &mut f64,
        var_xn_sub_dn7_slot: &mut f64,
        var_xn_sub_dn8_slot: &mut f64,
        var_xn_sub_dn9_slot: &mut f64,
        var_xn_sub_rv_slot: &mut f64,
    ) {
        let mut var_betn1_i: f64 = *var_betn1_i_slot;
        let mut var_betn1_i_dn4: f64 = *var_betn1_i_dn4_slot;
        let mut var_betn1_i_dn6: f64 = *var_betn1_i_dn6_slot;
        let mut var_betn1_i_dn7: f64 = *var_betn1_i_dn7_slot;
        let mut var_betn1_i_dn8: f64 = *var_betn1_i_dn8_slot;
        let mut var_betn1_i_dn9: f64 = *var_betn1_i_dn9_slot;
        let mut var_betn1_i_rv: f64 = *var_betn1_i_rv_slot;
        let mut var_betn2_i: f64 = *var_betn2_i_slot;
        let mut var_betn2_i_dn4: f64 = *var_betn2_i_dn4_slot;
        let mut var_betn2_i_dn6: f64 = *var_betn2_i_dn6_slot;
        let mut var_betn2_i_dn7: f64 = *var_betn2_i_dn7_slot;
        let mut var_betn2_i_dn8: f64 = *var_betn2_i_dn8_slot;
        let mut var_betn2_i_dn9: f64 = *var_betn2_i_dn9_slot;
        let mut var_betn2_i_rv: f64 = *var_betn2_i_rv_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_cs_i_dn4: f64 = *var_cs_i_dn4_slot;
        let mut var_cs_i_dn6: f64 = *var_cs_i_dn6_slot;
        let mut var_cs_i_dn7: f64 = *var_cs_i_dn7_slot;
        let mut var_cs_i_dn8: f64 = *var_cs_i_dn8_slot;
        let mut var_cs_i_dn9: f64 = *var_cs_i_dn9_slot;
        let mut var_cs_i_rv: f64 = *var_cs_i_rv_slot;
        let mut var_dvfbpdep: f64 = *var_dvfbpdep_slot;
        let mut var_dvfbpdep_dn4: f64 = *var_dvfbpdep_dn4_slot;
        let mut var_dvfbpdep_dn6: f64 = *var_dvfbpdep_dn6_slot;
        let mut var_dvfbpdep_dn7: f64 = *var_dvfbpdep_dn7_slot;
        let mut var_dvfbpdep_dn8: f64 = *var_dvfbpdep_dn8_slot;
        let mut var_dvfbpdep_dn9: f64 = *var_dvfbpdep_dn9_slot;
        let mut var_dvfbpdep_rv: f64 = *var_dvfbpdep_rv_slot;
        let mut var_dvfbqm: f64 = *var_dvfbqm_slot;
        let mut var_dvfbqm_rv: f64 = *var_dvfbqm_rv_slot;
        let mut var_emin: f64 = *var_emin_slot;
        let mut var_emin_dn4: f64 = *var_emin_dn4_slot;
        let mut var_emin_dn6: f64 = *var_emin_dn6_slot;
        let mut var_emin_dn7: f64 = *var_emin_dn7_slot;
        let mut var_emin_dn8: f64 = *var_emin_dn8_slot;
        let mut var_emin_dn9: f64 = *var_emin_dn9_slot;
        let mut var_emin_rv: f64 = *var_emin_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_fmue: f64 = *var_fmue_slot;
        let mut var_fmue_dn4: f64 = *var_fmue_dn4_slot;
        let mut var_fmue_dn6: f64 = *var_fmue_dn6_slot;
        let mut var_fmue_dn7: f64 = *var_fmue_dn7_slot;
        let mut var_fmue_dn8: f64 = *var_fmue_dn8_slot;
        let mut var_fmue_dn9: f64 = *var_fmue_dn9_slot;
        let mut var_fmue_rv: f64 = *var_fmue_rv_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard142_rv: f64 = *var_guard142_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard145_rv: f64 = *var_guard145_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_inv_qi1cs: f64 = *var_inv_qi1cs_slot;
        let mut var_inv_qi1cs_rv: f64 = *var_inv_qi1cs_rv_slot;
        let mut var_inv_qi2cs: f64 = *var_inv_qi2cs_slot;
        let mut var_inv_qi2cs_rv: f64 = *var_inv_qi2cs_rv_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_dn4: f64 = *var_kp_dn4_slot;
        let mut var_kp_dn6: f64 = *var_kp_dn6_slot;
        let mut var_kp_dn7: f64 = *var_kp_dn7_slot;
        let mut var_kp_dn8: f64 = *var_kp_dn8_slot;
        let mut var_kp_dn9: f64 = *var_kp_dn9_slot;
        let mut var_kp_rv: f64 = *var_kp_rv_slot;
        let mut var_lnrtn: f64 = *var_lnrtn_slot;
        let mut var_lnrtn_dn4: f64 = *var_lnrtn_dn4_slot;
        let mut var_lnrtn_dn6: f64 = *var_lnrtn_dn6_slot;
        let mut var_lnrtn_dn7: f64 = *var_lnrtn_dn7_slot;
        let mut var_lnrtn_dn8: f64 = *var_lnrtn_dn8_slot;
        let mut var_lnrtn_dn9: f64 = *var_lnrtn_dn9_slot;
        let mut var_lnrtn_rv: f64 = *var_lnrtn_rv_slot;
        let mut var_mue_i: f64 = *var_mue_i_slot;
        let mut var_mue_i_dn4: f64 = *var_mue_i_dn4_slot;
        let mut var_mue_i_dn6: f64 = *var_mue_i_dn6_slot;
        let mut var_mue_i_dn7: f64 = *var_mue_i_dn7_slot;
        let mut var_mue_i_dn8: f64 = *var_mue_i_dn8_slot;
        let mut var_mue_i_dn9: f64 = *var_mue_i_dn9_slot;
        let mut var_mue_i_rv: f64 = *var_mue_i_rv_slot;
        let mut var_one_m_eta: f64 = *var_one_m_eta_slot;
        let mut var_one_m_eta_rv: f64 = *var_one_m_eta_rv_slot;
        let mut var_qq: f64 = *var_qq_slot;
        let mut var_qq_dn4: f64 = *var_qq_dn4_slot;
        let mut var_qq_dn6: f64 = *var_qq_dn6_slot;
        let mut var_qq_dn7: f64 = *var_qq_dn7_slot;
        let mut var_qq_dn8: f64 = *var_qq_dn8_slot;
        let mut var_qq_dn9: f64 = *var_qq_dn9_slot;
        let mut var_qq_rv: f64 = *var_qq_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_bet_dn4: f64 = *var_tf_bet_dn4_slot;
        let mut var_tf_bet_dn6: f64 = *var_tf_bet_dn6_slot;
        let mut var_tf_bet_dn7: f64 = *var_tf_bet_dn7_slot;
        let mut var_tf_bet_dn8: f64 = *var_tf_bet_dn8_slot;
        let mut var_tf_bet_dn9: f64 = *var_tf_bet_dn9_slot;
        let mut var_tf_bet_rv: f64 = *var_tf_bet_rv_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_cs_dn4: f64 = *var_tf_cs_dn4_slot;
        let mut var_tf_cs_dn6: f64 = *var_tf_cs_dn6_slot;
        let mut var_tf_cs_dn7: f64 = *var_tf_cs_dn7_slot;
        let mut var_tf_cs_dn8: f64 = *var_tf_cs_dn8_slot;
        let mut var_tf_cs_dn9: f64 = *var_tf_cs_dn9_slot;
        let mut var_tf_cs_rv: f64 = *var_tf_cs_rv_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_mue_dn4: f64 = *var_tf_mue_dn4_slot;
        let mut var_tf_mue_dn6: f64 = *var_tf_mue_dn6_slot;
        let mut var_tf_mue_dn7: f64 = *var_tf_mue_dn7_slot;
        let mut var_tf_mue_dn8: f64 = *var_tf_mue_dn8_slot;
        let mut var_tf_mue_dn9: f64 = *var_tf_mue_dn9_slot;
        let mut var_tf_mue_rv: f64 = *var_tf_mue_rv_slot;
        let mut var_tf_thecs: f64 = *var_tf_thecs_slot;
        let mut var_tf_thecs_dn4: f64 = *var_tf_thecs_dn4_slot;
        let mut var_tf_thecs_dn6: f64 = *var_tf_thecs_dn6_slot;
        let mut var_tf_thecs_dn7: f64 = *var_tf_thecs_dn7_slot;
        let mut var_tf_thecs_dn8: f64 = *var_tf_thecs_dn8_slot;
        let mut var_tf_thecs_dn9: f64 = *var_tf_thecs_dn9_slot;
        let mut var_tf_thecs_rv: f64 = *var_tf_thecs_rv_slot;
        let mut var_tf_themu: f64 = *var_tf_themu_slot;
        let mut var_tf_themu_dn4: f64 = *var_tf_themu_dn4_slot;
        let mut var_tf_themu_dn6: f64 = *var_tf_themu_dn6_slot;
        let mut var_tf_themu_dn7: f64 = *var_tf_themu_dn7_slot;
        let mut var_tf_themu_dn8: f64 = *var_tf_themu_dn8_slot;
        let mut var_tf_themu_dn9: f64 = *var_tf_themu_dn9_slot;
        let mut var_tf_themu_rv: f64 = *var_tf_themu_rv_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_ther_dn4: f64 = *var_tf_ther_dn4_slot;
        let mut var_tf_ther_dn6: f64 = *var_tf_ther_dn6_slot;
        let mut var_tf_ther_dn7: f64 = *var_tf_ther_dn7_slot;
        let mut var_tf_ther_dn8: f64 = *var_tf_ther_dn8_slot;
        let mut var_tf_ther_dn9: f64 = *var_tf_ther_dn9_slot;
        let mut var_tf_ther_rv: f64 = *var_tf_ther_rv_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_tf_xcor_dn4: f64 = *var_tf_xcor_dn4_slot;
        let mut var_tf_xcor_dn6: f64 = *var_tf_xcor_dn6_slot;
        let mut var_tf_xcor_dn7: f64 = *var_tf_xcor_dn7_slot;
        let mut var_tf_xcor_dn8: f64 = *var_tf_xcor_dn8_slot;
        let mut var_tf_xcor_dn9: f64 = *var_tf_xcor_dn9_slot;
        let mut var_tf_xcor_rv: f64 = *var_tf_xcor_rv_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_thecs_i_dn4: f64 = *var_thecs_i_dn4_slot;
        let mut var_thecs_i_dn6: f64 = *var_thecs_i_dn6_slot;
        let mut var_thecs_i_dn7: f64 = *var_thecs_i_dn7_slot;
        let mut var_thecs_i_dn8: f64 = *var_thecs_i_dn8_slot;
        let mut var_thecs_i_dn9: f64 = *var_thecs_i_dn9_slot;
        let mut var_thecs_i_rv: f64 = *var_thecs_i_rv_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_themu_i_dn4: f64 = *var_themu_i_dn4_slot;
        let mut var_themu_i_dn6: f64 = *var_themu_i_dn6_slot;
        let mut var_themu_i_dn7: f64 = *var_themu_i_dn7_slot;
        let mut var_themu_i_dn8: f64 = *var_themu_i_dn8_slot;
        let mut var_themu_i_dn9: f64 = *var_themu_i_dn9_slot;
        let mut var_themu_i_rv: f64 = *var_themu_i_rv_slot;
        let mut var_tsisq: f64 = *var_tsisq_slot;
        let mut var_tsisq_rv: f64 = *var_tsisq_rv_slot;
        let mut var_vfb1_i: f64 = *var_vfb1_i_slot;
        let mut var_vfb1_i_dn4: f64 = *var_vfb1_i_dn4_slot;
        let mut var_vfb1_i_dn6: f64 = *var_vfb1_i_dn6_slot;
        let mut var_vfb1_i_dn7: f64 = *var_vfb1_i_dn7_slot;
        let mut var_vfb1_i_dn8: f64 = *var_vfb1_i_dn8_slot;
        let mut var_vfb1_i_dn9: f64 = *var_vfb1_i_dn9_slot;
        let mut var_vfb1_i_rv: f64 = *var_vfb1_i_rv_slot;
        let mut var_vfb2_i: f64 = *var_vfb2_i_slot;
        let mut var_vfb2_i_dn4: f64 = *var_vfb2_i_dn4_slot;
        let mut var_vfb2_i_dn6: f64 = *var_vfb2_i_dn6_slot;
        let mut var_vfb2_i_dn7: f64 = *var_vfb2_i_dn7_slot;
        let mut var_vfb2_i_dn8: f64 = *var_vfb2_i_dn8_slot;
        let mut var_vfb2_i_dn9: f64 = *var_vfb2_i_dn9_slot;
        let mut var_vfb2_i_rv: f64 = *var_vfb2_i_rv_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfb2_t_rv: f64 = *var_vfb2_t_rv_slot;
        let mut var_vfbac1_i: f64 = *var_vfbac1_i_slot;
        let mut var_vfbac1_i_dn4: f64 = *var_vfbac1_i_dn4_slot;
        let mut var_vfbac1_i_dn6: f64 = *var_vfbac1_i_dn6_slot;
        let mut var_vfbac1_i_dn7: f64 = *var_vfbac1_i_dn7_slot;
        let mut var_vfbac1_i_dn8: f64 = *var_vfbac1_i_dn8_slot;
        let mut var_vfbac1_i_dn9: f64 = *var_vfbac1_i_dn9_slot;
        let mut var_vfbac1_i_rv: f64 = *var_vfbac1_i_rv_slot;
        let mut var_vfbac2_i: f64 = *var_vfbac2_i_slot;
        let mut var_vfbac2_i_dn4: f64 = *var_vfbac2_i_dn4_slot;
        let mut var_vfbac2_i_dn6: f64 = *var_vfbac2_i_dn6_slot;
        let mut var_vfbac2_i_dn7: f64 = *var_vfbac2_i_dn7_slot;
        let mut var_vfbac2_i_dn8: f64 = *var_vfbac2_i_dn8_slot;
        let mut var_vfbac2_i_dn9: f64 = *var_vfbac2_i_dn9_slot;
        let mut var_vfbac2_i_rv: f64 = *var_vfbac2_i_rv_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbac2_t_rv: f64 = *var_vfbac2_t_rv_slot;
        let mut var_xb_sub: f64 = *var_xb_sub_slot;
        let mut var_xb_sub_dn4: f64 = *var_xb_sub_dn4_slot;
        let mut var_xb_sub_dn6: f64 = *var_xb_sub_dn6_slot;
        let mut var_xb_sub_dn7: f64 = *var_xb_sub_dn7_slot;
        let mut var_xb_sub_dn8: f64 = *var_xb_sub_dn8_slot;
        let mut var_xb_sub_dn9: f64 = *var_xb_sub_dn9_slot;
        let mut var_xb_sub_rv: f64 = *var_xb_sub_rv_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;
        let mut var_xcor_i_dn4: f64 = *var_xcor_i_dn4_slot;
        let mut var_xcor_i_dn6: f64 = *var_xcor_i_dn6_slot;
        let mut var_xcor_i_dn7: f64 = *var_xcor_i_dn7_slot;
        let mut var_xcor_i_dn8: f64 = *var_xcor_i_dn8_slot;
        let mut var_xcor_i_dn9: f64 = *var_xcor_i_dn9_slot;
        let mut var_xcor_i_rv: f64 = *var_xcor_i_rv_slot;
        let mut var_xn_sub: f64 = *var_xn_sub_slot;
        let mut var_xn_sub_dn4: f64 = *var_xn_sub_dn4_slot;
        let mut var_xn_sub_dn6: f64 = *var_xn_sub_dn6_slot;
        let mut var_xn_sub_dn7: f64 = *var_xn_sub_dn7_slot;
        let mut var_xn_sub_dn8: f64 = *var_xn_sub_dn8_slot;
        let mut var_xn_sub_dn9: f64 = *var_xn_sub_dn9_slot;
        let mut var_xn_sub_rv: f64 = *var_xn_sub_rv_slot;

        let assign7120_e6880: f64 = (var_nsub_i / var_neff_sub);
        let assign7120_e6881: f64 = (assign7120_e6880).ln();
        let assign7120_e6883: f64 = (assign7120_e6881 + var_eg_2phit0);
        var_xb_sub = assign7120_e6883;
        var_xb_sub_dn4 = (((-((var_nsub_i * var_neff_sub_dn4) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn4);
        var_xb_sub_dn6 = (((-((var_nsub_i * var_neff_sub_dn6) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn6);
        var_xb_sub_dn7 = (((-((var_nsub_i * var_neff_sub_dn7) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn7);
        var_xb_sub_dn8 = (((-((var_nsub_i * var_neff_sub_dn8) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn8);
        var_xb_sub_dn9 = (((-((var_nsub_i * var_neff_sub_dn9) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn9);
        var_xb_sub_rv = 0.0;

        let assign7130_e6886: f64 = (2.0 * var_xb_sub);
        var_xn_sub = assign7130_e6886;
        var_xn_sub_dn4 = (2.0 * var_xb_sub_dn4);
        var_xn_sub_dn6 = (2.0 * var_xb_sub_dn6);
        var_xn_sub_dn7 = (2.0 * var_xb_sub_dn7);
        var_xn_sub_dn8 = (2.0 * var_xb_sub_dn8);
        var_xn_sub_dn9 = (2.0 * var_xb_sub_dn9);
        var_xn_sub_rv = 0.0;

        let assign7140_e6889: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        var_guard142 = assign7140_e6889;
        var_guard142_rv = 0.0;

        let (assign7150_e6899, assign7150_e6899_d_n4, assign7150_e6899_d_n6, assign7150_e6899_d_n7, assign7150_e6899_d_n8, assign7150_e6899_d_n9,) = {
    if (var_guard142 != 0.0) {
        let assign7150_e6894: f64 = (var_typesub_i * var_phit0);
        let assign7150_e6896: f64 = (assign7150_e6894 * var_xb_sub);
        let assign7150_e6897: f64 = (var_vfb2_t + assign7150_e6896);
        (assign7150_e6897, (var_vfb2_t_dn4 + (((var_typesub_i * var_phit0_dn4) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn4))), (var_vfb2_t_dn6 + (((var_typesub_i * var_phit0_dn6) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn6))), (var_vfb2_t_dn7 + (((var_typesub_i * var_phit0_dn7) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn7))), (var_vfb2_t_dn8 + (((var_typesub_i * var_phit0_dn8) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn8))), (var_vfb2_t_dn9 + (((var_typesub_i * var_phit0_dn9) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn9))),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign7150_e6899;
        var_vfb2_t_dn4 = assign7150_e6899_d_n4;
        var_vfb2_t_dn6 = assign7150_e6899_d_n6;
        var_vfb2_t_dn7 = assign7150_e6899_d_n7;
        var_vfb2_t_dn8 = assign7150_e6899_d_n8;
        var_vfb2_t_dn9 = assign7150_e6899_d_n9;
        var_vfb2_t_rv = 0.0;

        let (assign7160_e6909, assign7160_e6909_d_n4, assign7160_e6909_d_n6, assign7160_e6909_d_n7, assign7160_e6909_d_n8, assign7160_e6909_d_n9,) = {
    if (var_guard142 != 0.0) {
        let assign7160_e6904: f64 = (var_typesub_i * var_phit0);
        let assign7160_e6906: f64 = (assign7160_e6904 * var_xb_sub);
        let assign7160_e6907: f64 = (var_vfbac2_t + assign7160_e6906);
        (assign7160_e6907, (var_vfbac2_t_dn4 + (((var_typesub_i * var_phit0_dn4) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn4))), (var_vfbac2_t_dn6 + (((var_typesub_i * var_phit0_dn6) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn6))), (var_vfbac2_t_dn7 + (((var_typesub_i * var_phit0_dn7) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn7))), (var_vfbac2_t_dn8 + (((var_typesub_i * var_phit0_dn8) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn8))), (var_vfbac2_t_dn9 + (((var_typesub_i * var_phit0_dn9) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn9))),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign7160_e6909;
        var_vfbac2_t_dn4 = assign7160_e6909_d_n4;
        var_vfbac2_t_dn6 = assign7160_e6909_d_n6;
        var_vfbac2_t_dn7 = assign7160_e6909_d_n7;
        var_vfbac2_t_dn8 = assign7160_e6909_d_n8;
        var_vfbac2_t_dn9 = assign7160_e6909_d_n9;
        var_vfbac2_t_rv = 0.0;

        var_dvfbpdep = 0.0;
        var_dvfbpdep_dn4 = 0.0;
        var_dvfbpdep_dn6 = 0.0;
        var_dvfbpdep_dn7 = 0.0;
        var_dvfbpdep_dn8 = 0.0;
        var_dvfbpdep_dn9 = 0.0;
        var_dvfbpdep_rv = 0.0;

        let assign7180_e6913: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        var_guard143 = assign7180_e6913;
        var_guard143_rv = 0.0;

        let (assign7190_e6924, assign7190_e6924_d_n4, assign7190_e6924_d_n6, assign7190_e6924_d_n7, assign7190_e6924_d_n8, assign7190_e6924_d_n9,) = {
    if (var_guard143 != 0.0) {
        let assign7190_e6918: f64 = (var_np_i / var_neff_poly);
        let assign7190_e6919: f64 = (assign7190_e6918).ln();
        let assign7190_e6921: f64 = (assign7190_e6919 + var_eg_2phit0);
        let assign7190_e6922: f64 = (var_phit0 * assign7190_e6921);
        (assign7190_e6922, ((var_phit0_dn4 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn4 * var_neff_poly) - (var_np_i * var_neff_poly_dn4)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn4))), ((var_phit0_dn6 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn6 * var_neff_poly) - (var_np_i * var_neff_poly_dn6)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn6))), ((var_phit0_dn7 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn7 * var_neff_poly) - (var_np_i * var_neff_poly_dn7)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn7))), ((var_phit0_dn8 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn8 * var_neff_poly) - (var_np_i * var_neff_poly_dn8)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn8))), ((var_phit0_dn9 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn9 * var_neff_poly) - (var_np_i * var_neff_poly_dn9)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn9))),)
    } else {
        (var_dvfbpdep, var_dvfbpdep_dn4, var_dvfbpdep_dn6, var_dvfbpdep_dn7, var_dvfbpdep_dn8, var_dvfbpdep_dn9,)
    }
};
        var_dvfbpdep = assign7190_e6924;
        var_dvfbpdep_dn4 = assign7190_e6924_d_n4;
        var_dvfbpdep_dn6 = assign7190_e6924_d_n6;
        var_dvfbpdep_dn7 = assign7190_e6924_d_n7;
        var_dvfbpdep_dn8 = assign7190_e6924_d_n8;
        var_dvfbpdep_dn9 = assign7190_e6924_d_n9;
        var_dvfbpdep_rv = 0.0;

        let assign7200_e6927: f64 = (2.0 * 1.602176565e-19);
        let assign7200_e6929: f64 = (assign7200_e6927 * var_epsch);
        let assign7200_e6931: f64 = (assign7200_e6929 * var_np_i);
        let assign7200_e6932: f64 = (assign7200_e6931).sqrt();
        let assign7200_e6934: f64 = (assign7200_e6932 / var_cox1init);
        var_kp = assign7200_e6934;
        var_kp_dn4 = (((assign7200_e6929 * var_np_i_dn4) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn6 = (((assign7200_e6929 * var_np_i_dn6) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn7 = (((assign7200_e6929 * var_np_i_dn7) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn8 = (((assign7200_e6929 * var_np_i_dn8) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn9 = (((assign7200_e6929 * var_np_i_dn9) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_rv = 0.0;

        var_emin = 15.0;
        var_emin_dn4 = 0.0;
        var_emin_dn6 = 0.0;
        var_emin_dn7 = 0.0;
        var_emin_dn8 = 0.0;
        var_emin_dn9 = 0.0;
        var_emin_rv = 0.0;

        let assign7220_e6938: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        var_guard144 = assign7220_e6938;
        var_guard144_rv = 0.0;

        let (assign7230_e6963, assign7230_e6963_d_n4, assign7230_e6963_d_n6, assign7230_e6963_d_n7, assign7230_e6963_d_n8, assign7230_e6963_d_n9,) = {
    if (var_guard144 != 0.0) {
        let assign7230_e6944: f64 = (2970.0 / var_tkd);
        let assign7230_e6945: f64 = (15.0 + assign7230_e6944);
        let assign7230_e6949: f64 = (2970.0 / var_tkd);
        let assign7230_e6950: f64 = (15.0 - assign7230_e6949);
        let assign7230_e6954: f64 = (2970.0 / var_tkd);
        let assign7230_e6955: f64 = (15.0 - assign7230_e6954);
        let assign7230_e6956: f64 = (assign7230_e6950 * assign7230_e6955);
        let assign7230_e6958: f64 = (assign7230_e6956 + 1e-6);
        let assign7230_e6959: f64 = (assign7230_e6958).sqrt();
        let assign7230_e6960: f64 = (assign7230_e6945 + assign7230_e6959);
        let assign7230_e6961: f64 = (0.5 * assign7230_e6960);
        (assign7230_e6961, (0.5 * ((-((2970.0 * var_tkd_dn4) / (var_tkd * var_tkd))) + ((((-(-((2970.0 * var_tkd_dn4) / (var_tkd * var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * var_tkd_dn4) / (var_tkd * var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * var_tkd_dn6) / (var_tkd * var_tkd))) + ((((-(-((2970.0 * var_tkd_dn6) / (var_tkd * var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * var_tkd_dn6) / (var_tkd * var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * var_tkd_dn7) / (var_tkd * var_tkd))) + ((((-(-((2970.0 * var_tkd_dn7) / (var_tkd * var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * var_tkd_dn7) / (var_tkd * var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * var_tkd_dn8) / (var_tkd * var_tkd))) + ((((-(-((2970.0 * var_tkd_dn8) / (var_tkd * var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * var_tkd_dn8) / (var_tkd * var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * var_tkd_dn9) / (var_tkd * var_tkd))) + ((((-(-((2970.0 * var_tkd_dn9) / (var_tkd * var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * var_tkd_dn9) / (var_tkd * var_tkd)))))) / (2.0 * assign7230_e6959)))),)
    } else {
        (var_emin, var_emin_dn4, var_emin_dn6, var_emin_dn7, var_emin_dn8, var_emin_dn9,)
    }
};
        var_emin = assign7230_e6963;
        var_emin_dn4 = assign7230_e6963_d_n4;
        var_emin_dn6 = assign7230_e6963_d_n6;
        var_emin_dn7 = assign7230_e6963_d_n7;
        var_emin_dn8 = assign7230_e6963_d_n8;
        var_emin_dn9 = assign7230_e6963_d_n9;
        var_emin_rv = 0.0;

        var_dvfbqm = 0.0;
        var_dvfbqm_rv = 0.0;

        var_qq = 0.0;
        var_qq_dn4 = 0.0;
        var_qq_dn6 = 0.0;
        var_qq_dn7 = 0.0;
        var_qq_dn8 = 0.0;
        var_qq_dn9 = 0.0;
        var_qq_rv = 0.0;

        let assign7260_e6968: f64 = (1e18 * var_tsi_i);
        let assign7260_e6970: f64 = (assign7260_e6968 * var_tsi_i);
        var_tsisq = assign7260_e6970;
        var_tsisq_rv = 0.0;

        let assign7270_e6973: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard145 = assign7270_e6973;
        var_guard145_rv = 0.0;

        let assign7280_e6976: f64 = 1.0;
        let assign7280_e6977: f64 = if p.p14 == assign7280_e6976 { 1.0 } else { 0.0 };
        var_guard146 = assign7280_e6977;
        var_guard146_rv = 0.0;

        let (assign7290_e6985,) = {
    if ((var_guard145 != 0.0) && (var_guard146 != 0.0)) {
        let assign7290_e6983: f64 = (0.409618895 / var_tsisq);
        (assign7290_e6983,)
    } else {
        (var_dvfbqm,)
    }
};
        var_dvfbqm = assign7290_e6985;
        var_dvfbqm_rv = 0.0;

        let (assign7300_e7004, assign7300_e7004_d_n4, assign7300_e7004_d_n6, assign7300_e7004_d_n7, assign7300_e7004_d_n8, assign7300_e7004_d_n9,) = {
    if ((var_guard145 != 0.0) && (var_guard146 != 0.0)) {
        let assign7300_e6991: f64 = (0.4 * p.p13);
        let assign7300_e6993: f64 = (assign7300_e6991 * 1.27520989);
        let assign7300_e6995: f64 = (-0.3333333333333);
        let assign7300_e6998: f64 = (var_phit * var_tsisq);
        let assign7300_e6999: f64 = (assign7300_e6998).ln();
        let assign7300_e7000: f64 = (assign7300_e6995 * assign7300_e6999);
        let assign7300_e7001: f64 = (assign7300_e7000).exp();
        let assign7300_e7002: f64 = (assign7300_e6993 * assign7300_e7001);
        (assign7300_e7002, (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((var_phit_dn4 * var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((var_phit_dn6 * var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((var_phit_dn7 * var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((var_phit_dn8 * var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((var_phit_dn9 * var_tsisq) / assign7300_e6998)))),)
    } else {
        (var_qq, var_qq_dn4, var_qq_dn6, var_qq_dn7, var_qq_dn8, var_qq_dn9,)
    }
};
        var_qq = assign7300_e7004;
        var_qq_dn4 = assign7300_e7004_d_n4;
        var_qq_dn6 = assign7300_e7004_d_n6;
        var_qq_dn7 = assign7300_e7004_d_n7;
        var_qq_dn8 = assign7300_e7004_d_n8;
        var_qq_dn9 = assign7300_e7004_d_n9;
        var_qq_rv = 0.0;

        let (assign7310_e7013,) = {
    if ((var_guard145 != 0.0) && (var_guard146 == 0.0)) {
        let assign7310_e7011: f64 = (0.723134895 / var_tsisq);
        (assign7310_e7011,)
    } else {
        (var_dvfbqm,)
    }
};
        var_dvfbqm = assign7310_e7013;
        var_dvfbqm_rv = 0.0;

        let (assign7320_e7033, assign7320_e7033_d_n4, assign7320_e7033_d_n6, assign7320_e7033_d_n7, assign7320_e7033_d_n8, assign7320_e7033_d_n9,) = {
    if ((var_guard145 != 0.0) && (var_guard146 == 0.0)) {
        let assign7320_e7020: f64 = (0.4 * p.p13);
        let assign7320_e7022: f64 = (assign7320_e7020 * 1.5412087);
        let assign7320_e7024: f64 = (-0.3333333333333);
        let assign7320_e7027: f64 = (var_phit * var_tsisq);
        let assign7320_e7028: f64 = (assign7320_e7027).ln();
        let assign7320_e7029: f64 = (assign7320_e7024 * assign7320_e7028);
        let assign7320_e7030: f64 = (assign7320_e7029).exp();
        let assign7320_e7031: f64 = (assign7320_e7022 * assign7320_e7030);
        (assign7320_e7031, (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((var_phit_dn4 * var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((var_phit_dn6 * var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((var_phit_dn7 * var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((var_phit_dn8 * var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((var_phit_dn9 * var_tsisq) / assign7320_e7027)))),)
    } else {
        (var_qq, var_qq_dn4, var_qq_dn6, var_qq_dn7, var_qq_dn8, var_qq_dn9,)
    }
};
        var_qq = assign7320_e7033;
        var_qq_dn4 = assign7320_e7033_d_n4;
        var_qq_dn6 = assign7320_e7033_d_n6;
        var_qq_dn7 = assign7320_e7033_d_n7;
        var_qq_dn8 = assign7320_e7033_d_n8;
        var_qq_dn9 = assign7320_e7033_d_n9;
        var_qq_rv = 0.0;

        let assign7330_e7036: f64 = (p.p14 * var_stvfb_i);
        let assign7330_e7038: f64 = (assign7330_e7036 * var_dt);
        let assign7330_e7040: f64 = (assign7330_e7038 + var_dvfbqm);
        var_temp = assign7330_e7040;
        var_temp_dn4 = (assign7330_e7036 * var_dt_dn4);
        var_temp_dn6 = (assign7330_e7036 * var_dt_dn6);
        var_temp_dn7 = (assign7330_e7036 * var_dt_dn7);
        var_temp_dn8 = (assign7330_e7036 * var_dt_dn8);
        var_temp_dn9 = (assign7330_e7036 * var_dt_dn9);
        var_temp_rv = 0.0;

        let assign7340_e7043: f64 = (var_temp + p.p34);
        let assign7340_e7045: f64 = (assign7340_e7043 - var_dvfbpdep);
        var_temp1 = assign7340_e7045;
        var_temp1_dn4 = (var_temp_dn4 - var_dvfbpdep_dn4);
        var_temp1_dn6 = (var_temp_dn6 - var_dvfbpdep_dn6);
        var_temp1_dn7 = (var_temp_dn7 - var_dvfbpdep_dn7);
        var_temp1_dn8 = (var_temp_dn8 - var_dvfbpdep_dn8);
        var_temp1_dn9 = (var_temp_dn9 - var_dvfbpdep_dn9);
        var_temp1_rv = 0.0;

        let assign7350_e7049: f64 = (var_vfb1_t + var_dvfbch);
        let assign7350_e7051: f64 = (assign7350_e7049 + var_dvfb1nch);
        let assign7350_e7052: f64 = (p.p14 * assign7350_e7051);
        let assign7350_e7054: f64 = (assign7350_e7052 + var_temp1);
        var_vfb1_i = assign7350_e7054;
        var_vfb1_i_dn4 = ((p.p14 * ((var_vfb1_t_dn4 + var_dvfbch_dn4) + var_dvfb1nch_dn4)) + var_temp1_dn4);
        var_vfb1_i_dn6 = ((p.p14 * ((var_vfb1_t_dn6 + var_dvfbch_dn6) + var_dvfb1nch_dn6)) + var_temp1_dn6);
        var_vfb1_i_dn7 = ((p.p14 * ((var_vfb1_t_dn7 + var_dvfbch_dn7) + var_dvfb1nch_dn7)) + var_temp1_dn7);
        var_vfb1_i_dn8 = ((p.p14 * ((var_vfb1_t_dn8 + var_dvfbch_dn8) + var_dvfb1nch_dn8)) + var_temp1_dn8);
        var_vfb1_i_dn9 = ((p.p14 * ((var_vfb1_t_dn9 + var_dvfbch_dn9) + var_dvfb1nch_dn9)) + var_temp1_dn9);
        var_vfb1_i_rv = 0.0;

        let assign7360_e7058: f64 = (var_vfb2_t + var_dvfbch);
        let assign7360_e7060: f64 = (assign7360_e7058 + var_dvfb2nch);
        let assign7360_e7061: f64 = (p.p14 * assign7360_e7060);
        let assign7360_e7063: f64 = (assign7360_e7061 + var_temp);
        var_vfb2_i = assign7360_e7063;
        var_vfb2_i_dn4 = ((p.p14 * ((var_vfb2_t_dn4 + var_dvfbch_dn4) + var_dvfb2nch_dn4)) + var_temp_dn4);
        var_vfb2_i_dn6 = ((p.p14 * ((var_vfb2_t_dn6 + var_dvfbch_dn6) + var_dvfb2nch_dn6)) + var_temp_dn6);
        var_vfb2_i_dn7 = ((p.p14 * ((var_vfb2_t_dn7 + var_dvfbch_dn7) + var_dvfb2nch_dn7)) + var_temp_dn7);
        var_vfb2_i_dn8 = ((p.p14 * ((var_vfb2_t_dn8 + var_dvfbch_dn8) + var_dvfb2nch_dn8)) + var_temp_dn8);
        var_vfb2_i_dn9 = ((p.p14 * ((var_vfb2_t_dn9 + var_dvfbch_dn9) + var_dvfb2nch_dn9)) + var_temp_dn9);
        var_vfb2_i_rv = 0.0;

        let assign7370_e7067: f64 = (var_vfbac1_t + var_dvfbch);
        let assign7370_e7069: f64 = (assign7370_e7067 + var_dvfb1nch);
        let assign7370_e7070: f64 = (p.p14 * assign7370_e7069);
        let assign7370_e7072: f64 = (assign7370_e7070 + var_temp1);
        var_vfbac1_i = assign7370_e7072;
        var_vfbac1_i_dn4 = ((p.p14 * ((var_vfbac1_t_dn4 + var_dvfbch_dn4) + var_dvfb1nch_dn4)) + var_temp1_dn4);
        var_vfbac1_i_dn6 = ((p.p14 * ((var_vfbac1_t_dn6 + var_dvfbch_dn6) + var_dvfb1nch_dn6)) + var_temp1_dn6);
        var_vfbac1_i_dn7 = ((p.p14 * ((var_vfbac1_t_dn7 + var_dvfbch_dn7) + var_dvfb1nch_dn7)) + var_temp1_dn7);
        var_vfbac1_i_dn8 = ((p.p14 * ((var_vfbac1_t_dn8 + var_dvfbch_dn8) + var_dvfb1nch_dn8)) + var_temp1_dn8);
        var_vfbac1_i_dn9 = ((p.p14 * ((var_vfbac1_t_dn9 + var_dvfbch_dn9) + var_dvfb1nch_dn9)) + var_temp1_dn9);
        var_vfbac1_i_rv = 0.0;

        let assign7380_e7076: f64 = (var_vfbac2_t + var_dvfbch);
        let assign7380_e7078: f64 = (assign7380_e7076 + var_dvfb2nch);
        let assign7380_e7079: f64 = (p.p14 * assign7380_e7078);
        let assign7380_e7081: f64 = (assign7380_e7079 + var_temp);
        var_vfbac2_i = assign7380_e7081;
        var_vfbac2_i_dn4 = ((p.p14 * ((var_vfbac2_t_dn4 + var_dvfbch_dn4) + var_dvfb2nch_dn4)) + var_temp_dn4);
        var_vfbac2_i_dn6 = ((p.p14 * ((var_vfbac2_t_dn6 + var_dvfbch_dn6) + var_dvfb2nch_dn6)) + var_temp_dn6);
        var_vfbac2_i_dn7 = ((p.p14 * ((var_vfbac2_t_dn7 + var_dvfbch_dn7) + var_dvfb2nch_dn7)) + var_temp_dn7);
        var_vfbac2_i_dn8 = ((p.p14 * ((var_vfbac2_t_dn8 + var_dvfbch_dn8) + var_dvfb2nch_dn8)) + var_temp_dn8);
        var_vfbac2_i_dn9 = ((p.p14 * ((var_vfbac2_t_dn9 + var_dvfbch_dn9) + var_dvfb2nch_dn9)) + var_temp_dn9);
        var_vfbac2_i_rv = 0.0;

        let assign7390_e7083: f64 = (var_rtn).ln();
        var_lnrtn = assign7390_e7083;
        var_lnrtn_dn4 = (var_rtn_dn4 / var_rtn);
        var_lnrtn_dn6 = (var_rtn_dn6 / var_rtn);
        var_lnrtn_dn7 = (var_rtn_dn7 / var_rtn);
        var_lnrtn_dn8 = (var_rtn_dn8 / var_rtn);
        var_lnrtn_dn9 = (var_rtn_dn9 / var_rtn);
        var_lnrtn_rv = 0.0;

        let assign7400_e7086: f64 = (var_stbet_i * var_lnrtn);
        let assign7400_e7087: f64 = (assign7400_e7086).exp();
        let assign7400_e7089: f64 = (assign7400_e7087 * p.p35);
        var_tf_bet = assign7400_e7089;
        var_tf_bet_dn4 = ((assign7400_e7087 * (var_stbet_i * var_lnrtn_dn4)) * p.p35);
        var_tf_bet_dn6 = ((assign7400_e7087 * (var_stbet_i * var_lnrtn_dn6)) * p.p35);
        var_tf_bet_dn7 = ((assign7400_e7087 * (var_stbet_i * var_lnrtn_dn7)) * p.p35);
        var_tf_bet_dn8 = ((assign7400_e7087 * (var_stbet_i * var_lnrtn_dn8)) * p.p35);
        var_tf_bet_dn9 = ((assign7400_e7087 * (var_stbet_i * var_lnrtn_dn9)) * p.p35);
        var_tf_bet_rv = 0.0;

        let assign7410_e7092: f64 = (var_betn1_t * var_tf_bet);
        var_betn1_i = assign7410_e7092;
        var_betn1_i_dn4 = ((var_betn1_t_dn4 * var_tf_bet) + (var_betn1_t * var_tf_bet_dn4));
        var_betn1_i_dn6 = ((var_betn1_t_dn6 * var_tf_bet) + (var_betn1_t * var_tf_bet_dn6));
        var_betn1_i_dn7 = ((var_betn1_t_dn7 * var_tf_bet) + (var_betn1_t * var_tf_bet_dn7));
        var_betn1_i_dn8 = ((var_betn1_t_dn8 * var_tf_bet) + (var_betn1_t * var_tf_bet_dn8));
        var_betn1_i_dn9 = ((var_betn1_t_dn9 * var_tf_bet) + (var_betn1_t * var_tf_bet_dn9));
        var_betn1_i_rv = 0.0;

        let assign7420_e7095: f64 = (var_betn2_t * var_tf_bet);
        var_betn2_i = assign7420_e7095;
        var_betn2_i_dn4 = ((var_betn2_t_dn4 * var_tf_bet) + (var_betn2_t * var_tf_bet_dn4));
        var_betn2_i_dn6 = ((var_betn2_t_dn6 * var_tf_bet) + (var_betn2_t * var_tf_bet_dn6));
        var_betn2_i_dn7 = ((var_betn2_t_dn7 * var_tf_bet) + (var_betn2_t * var_tf_bet_dn7));
        var_betn2_i_dn8 = ((var_betn2_t_dn8 * var_tf_bet) + (var_betn2_t * var_tf_bet_dn8));
        var_betn2_i_dn9 = ((var_betn2_t_dn9 * var_tf_bet) + (var_betn2_t * var_tf_bet_dn9));
        var_betn2_i_rv = 0.0;

        let assign7430_e7098: f64 = (var_stmue_i * var_lnrtn);
        let assign7430_e7099: f64 = (assign7430_e7098).exp();
        var_tf_mue = assign7430_e7099;
        var_tf_mue_dn4 = (assign7430_e7099 * (var_stmue_i * var_lnrtn_dn4));
        var_tf_mue_dn6 = (assign7430_e7099 * (var_stmue_i * var_lnrtn_dn6));
        var_tf_mue_dn7 = (assign7430_e7099 * (var_stmue_i * var_lnrtn_dn7));
        var_tf_mue_dn8 = (assign7430_e7099 * (var_stmue_i * var_lnrtn_dn8));
        var_tf_mue_dn9 = (assign7430_e7099 * (var_stmue_i * var_lnrtn_dn9));
        var_tf_mue_rv = 0.0;

        let assign7440_e7102: f64 = (var_mue_t * var_tf_mue);
        var_mue_i = assign7440_e7102;
        var_mue_i_dn4 = (var_mue_t * var_tf_mue_dn4);
        var_mue_i_dn6 = (var_mue_t * var_tf_mue_dn6);
        var_mue_i_dn7 = (var_mue_t * var_tf_mue_dn7);
        var_mue_i_dn8 = (var_mue_t * var_tf_mue_dn8);
        var_mue_i_dn9 = (var_mue_t * var_tf_mue_dn9);
        var_mue_i_rv = 0.0;

        let assign7450_e7105: f64 = (var_stthemu_i * var_lnrtn);
        let assign7450_e7106: f64 = (assign7450_e7105).exp();
        var_tf_themu = assign7450_e7106;
        var_tf_themu_dn4 = (assign7450_e7106 * (var_stthemu_i * var_lnrtn_dn4));
        var_tf_themu_dn6 = (assign7450_e7106 * (var_stthemu_i * var_lnrtn_dn6));
        var_tf_themu_dn7 = (assign7450_e7106 * (var_stthemu_i * var_lnrtn_dn7));
        var_tf_themu_dn8 = (assign7450_e7106 * (var_stthemu_i * var_lnrtn_dn8));
        var_tf_themu_dn9 = (assign7450_e7106 * (var_stthemu_i * var_lnrtn_dn9));
        var_tf_themu_rv = 0.0;

        let assign7460_e7109: f64 = (var_themu_t * var_tf_themu);
        var_themu_i = assign7460_e7109;
        var_themu_i_dn4 = (var_themu_t * var_tf_themu_dn4);
        var_themu_i_dn6 = (var_themu_t * var_tf_themu_dn6);
        var_themu_i_dn7 = (var_themu_t * var_tf_themu_dn7);
        var_themu_i_dn8 = (var_themu_t * var_tf_themu_dn8);
        var_themu_i_dn9 = (var_themu_t * var_tf_themu_dn9);
        var_themu_i_rv = 0.0;

        let assign7470_e7112: f64 = (var_stcs_i * var_lnrtn);
        let assign7470_e7113: f64 = (assign7470_e7112).exp();
        var_tf_cs = assign7470_e7113;
        var_tf_cs_dn4 = (assign7470_e7113 * (var_stcs_i * var_lnrtn_dn4));
        var_tf_cs_dn6 = (assign7470_e7113 * (var_stcs_i * var_lnrtn_dn6));
        var_tf_cs_dn7 = (assign7470_e7113 * (var_stcs_i * var_lnrtn_dn7));
        var_tf_cs_dn8 = (assign7470_e7113 * (var_stcs_i * var_lnrtn_dn8));
        var_tf_cs_dn9 = (assign7470_e7113 * (var_stcs_i * var_lnrtn_dn9));
        var_tf_cs_rv = 0.0;

        let assign7480_e7116: f64 = (var_cs_t * var_tf_cs);
        var_cs_i = assign7480_e7116;
        var_cs_i_dn4 = (var_cs_t * var_tf_cs_dn4);
        var_cs_i_dn6 = (var_cs_t * var_tf_cs_dn6);
        var_cs_i_dn7 = (var_cs_t * var_tf_cs_dn7);
        var_cs_i_dn8 = (var_cs_t * var_tf_cs_dn8);
        var_cs_i_dn9 = (var_cs_t * var_tf_cs_dn9);
        var_cs_i_rv = 0.0;

        let assign7490_e7119: f64 = (var_stthecs_i * var_lnrtn);
        let assign7490_e7120: f64 = (assign7490_e7119).exp();
        var_tf_thecs = assign7490_e7120;
        var_tf_thecs_dn4 = (assign7490_e7120 * (var_stthecs_i * var_lnrtn_dn4));
        var_tf_thecs_dn6 = (assign7490_e7120 * (var_stthecs_i * var_lnrtn_dn6));
        var_tf_thecs_dn7 = (assign7490_e7120 * (var_stthecs_i * var_lnrtn_dn7));
        var_tf_thecs_dn8 = (assign7490_e7120 * (var_stthecs_i * var_lnrtn_dn8));
        var_tf_thecs_dn9 = (assign7490_e7120 * (var_stthecs_i * var_lnrtn_dn9));
        var_tf_thecs_rv = 0.0;

        let assign7500_e7123: f64 = (var_thecs_t * var_tf_thecs);
        var_thecs_i = assign7500_e7123;
        var_thecs_i_dn4 = (var_thecs_t * var_tf_thecs_dn4);
        var_thecs_i_dn6 = (var_thecs_t * var_tf_thecs_dn6);
        var_thecs_i_dn7 = (var_thecs_t * var_tf_thecs_dn7);
        var_thecs_i_dn8 = (var_thecs_t * var_tf_thecs_dn8);
        var_thecs_i_dn9 = (var_thecs_t * var_tf_thecs_dn9);
        var_thecs_i_rv = 0.0;

        let assign7510_e7126: f64 = (var_stxcor_i * var_lnrtn);
        let assign7510_e7127: f64 = (assign7510_e7126).exp();
        var_tf_xcor = assign7510_e7127;
        var_tf_xcor_dn4 = (assign7510_e7127 * (var_stxcor_i * var_lnrtn_dn4));
        var_tf_xcor_dn6 = (assign7510_e7127 * (var_stxcor_i * var_lnrtn_dn6));
        var_tf_xcor_dn7 = (assign7510_e7127 * (var_stxcor_i * var_lnrtn_dn7));
        var_tf_xcor_dn8 = (assign7510_e7127 * (var_stxcor_i * var_lnrtn_dn8));
        var_tf_xcor_dn9 = (assign7510_e7127 * (var_stxcor_i * var_lnrtn_dn9));
        var_tf_xcor_rv = 0.0;

        let assign7520_e7130: f64 = (var_xcor_t * var_tf_xcor);
        var_xcor_i = assign7520_e7130;
        var_xcor_i_dn4 = (var_xcor_t * var_tf_xcor_dn4);
        var_xcor_i_dn6 = (var_xcor_t * var_tf_xcor_dn6);
        var_xcor_i_dn7 = (var_xcor_t * var_tf_xcor_dn7);
        var_xcor_i_dn8 = (var_xcor_t * var_tf_xcor_dn8);
        var_xcor_i_dn9 = (var_xcor_t * var_tf_xcor_dn9);
        var_xcor_i_rv = 0.0;

        let assign7530_e7133: f64 = (1e-8 * var_phit);
        let assign7530_e7135: f64 = (assign7530_e7133 / var_tsi_i);
        var_temp = assign7530_e7135;
        var_temp_dn4 = ((1e-8 * var_phit_dn4) / var_tsi_i);
        var_temp_dn6 = ((1e-8 * var_phit_dn6) / var_tsi_i);
        var_temp_dn7 = ((1e-8 * var_phit_dn7) / var_tsi_i);
        var_temp_dn8 = ((1e-8 * var_phit_dn8) / var_tsi_i);
        var_temp_dn9 = ((1e-8 * var_phit_dn9) / var_tsi_i);
        var_temp_rv = 0.0;

        let assign7540_e7138: f64 = (var_temp * var_mue_i);
        var_fmue = assign7540_e7138;
        var_fmue_dn4 = ((var_temp_dn4 * var_mue_i) + (var_temp * var_mue_i_dn4));
        var_fmue_dn6 = ((var_temp_dn6 * var_mue_i) + (var_temp * var_mue_i_dn6));
        var_fmue_dn7 = ((var_temp_dn7 * var_mue_i) + (var_temp * var_mue_i_dn7));
        var_fmue_dn8 = ((var_temp_dn8 * var_mue_i) + (var_temp * var_mue_i_dn8));
        var_fmue_dn9 = ((var_temp_dn9 * var_mue_i) + (var_temp * var_mue_i_dn9));
        var_fmue_rv = 0.0;

        let assign7550_e7142: f64 = (0.5 * var_csthr_i);
        let assign7550_e7143: f64 = (1.0 / assign7550_e7142);
        var_inv_qi1cs = assign7550_e7143;
        var_inv_qi1cs_rv = 0.0;

        let assign7560_e7146: f64 = (var_inv_qi1cs / var_csthrb_i);
        var_inv_qi2cs = assign7560_e7146;
        var_inv_qi2cs_rv = 0.0;

        let assign7570_e7149: f64 = 1.0;
        let assign7570_e7150: f64 = if p.p14 == assign7570_e7149 { 1.0 } else { 0.0 };
        var_guard147 = assign7570_e7150;
        var_guard147_rv = 0.0;

        let (assign7580_e7156,) = {
    if (var_guard147 != 0.0) {
        let assign7580_e7154: f64 = (0.5 * var_feta_i);
        (assign7580_e7154,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign7580_e7156;
        var_eta_mu_rv = 0.0;

        let (assign7590_e7163,) = {
    if (var_guard147 == 0.0) {
        let assign7590_e7161: f64 = (0.3333333333333 * var_feta_i);
        (assign7590_e7161,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign7590_e7163;
        var_eta_mu_rv = 0.0;

        let assign7600_e7166: f64 = (1.0 - var_eta_mu);
        var_one_m_eta = assign7600_e7166;
        var_one_m_eta_rv = 0.0;

        let assign7610_e7169: f64 = (var_strs_i * var_lnrtn);
        let assign7610_e7170: f64 = (assign7610_e7169).exp();
        var_tf_ther = assign7610_e7170;
        var_tf_ther_dn4 = (assign7610_e7170 * (var_strs_i * var_lnrtn_dn4));
        var_tf_ther_dn6 = (assign7610_e7170 * (var_strs_i * var_lnrtn_dn6));
        var_tf_ther_dn7 = (assign7610_e7170 * (var_strs_i * var_lnrtn_dn7));
        var_tf_ther_dn8 = (assign7610_e7170 * (var_strs_i * var_lnrtn_dn8));
        var_tf_ther_dn9 = (assign7610_e7170 * (var_strs_i * var_lnrtn_dn9));
        var_tf_ther_rv = 0.0;

        *var_betn1_i_slot = var_betn1_i;
        *var_betn1_i_dn4_slot = var_betn1_i_dn4;
        *var_betn1_i_dn6_slot = var_betn1_i_dn6;
        *var_betn1_i_dn7_slot = var_betn1_i_dn7;
        *var_betn1_i_dn8_slot = var_betn1_i_dn8;
        *var_betn1_i_dn9_slot = var_betn1_i_dn9;
        *var_betn1_i_rv_slot = var_betn1_i_rv;
        *var_betn2_i_slot = var_betn2_i;
        *var_betn2_i_dn4_slot = var_betn2_i_dn4;
        *var_betn2_i_dn6_slot = var_betn2_i_dn6;
        *var_betn2_i_dn7_slot = var_betn2_i_dn7;
        *var_betn2_i_dn8_slot = var_betn2_i_dn8;
        *var_betn2_i_dn9_slot = var_betn2_i_dn9;
        *var_betn2_i_rv_slot = var_betn2_i_rv;
        *var_cs_i_slot = var_cs_i;
        *var_cs_i_dn4_slot = var_cs_i_dn4;
        *var_cs_i_dn6_slot = var_cs_i_dn6;
        *var_cs_i_dn7_slot = var_cs_i_dn7;
        *var_cs_i_dn8_slot = var_cs_i_dn8;
        *var_cs_i_dn9_slot = var_cs_i_dn9;
        *var_cs_i_rv_slot = var_cs_i_rv;
        *var_dvfbpdep_slot = var_dvfbpdep;
        *var_dvfbpdep_dn4_slot = var_dvfbpdep_dn4;
        *var_dvfbpdep_dn6_slot = var_dvfbpdep_dn6;
        *var_dvfbpdep_dn7_slot = var_dvfbpdep_dn7;
        *var_dvfbpdep_dn8_slot = var_dvfbpdep_dn8;
        *var_dvfbpdep_dn9_slot = var_dvfbpdep_dn9;
        *var_dvfbpdep_rv_slot = var_dvfbpdep_rv;
        *var_dvfbqm_slot = var_dvfbqm;
        *var_dvfbqm_rv_slot = var_dvfbqm_rv;
        *var_emin_slot = var_emin;
        *var_emin_dn4_slot = var_emin_dn4;
        *var_emin_dn6_slot = var_emin_dn6;
        *var_emin_dn7_slot = var_emin_dn7;
        *var_emin_dn8_slot = var_emin_dn8;
        *var_emin_dn9_slot = var_emin_dn9;
        *var_emin_rv_slot = var_emin_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_fmue_slot = var_fmue;
        *var_fmue_dn4_slot = var_fmue_dn4;
        *var_fmue_dn6_slot = var_fmue_dn6;
        *var_fmue_dn7_slot = var_fmue_dn7;
        *var_fmue_dn8_slot = var_fmue_dn8;
        *var_fmue_dn9_slot = var_fmue_dn9;
        *var_fmue_rv_slot = var_fmue_rv;
        *var_guard142_slot = var_guard142;
        *var_guard142_rv_slot = var_guard142_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard145_slot = var_guard145;
        *var_guard145_rv_slot = var_guard145_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_inv_qi1cs_slot = var_inv_qi1cs;
        *var_inv_qi1cs_rv_slot = var_inv_qi1cs_rv;
        *var_inv_qi2cs_slot = var_inv_qi2cs;
        *var_inv_qi2cs_rv_slot = var_inv_qi2cs_rv;
        *var_kp_slot = var_kp;
        *var_kp_dn4_slot = var_kp_dn4;
        *var_kp_dn6_slot = var_kp_dn6;
        *var_kp_dn7_slot = var_kp_dn7;
        *var_kp_dn8_slot = var_kp_dn8;
        *var_kp_dn9_slot = var_kp_dn9;
        *var_kp_rv_slot = var_kp_rv;
        *var_lnrtn_slot = var_lnrtn;
        *var_lnrtn_dn4_slot = var_lnrtn_dn4;
        *var_lnrtn_dn6_slot = var_lnrtn_dn6;
        *var_lnrtn_dn7_slot = var_lnrtn_dn7;
        *var_lnrtn_dn8_slot = var_lnrtn_dn8;
        *var_lnrtn_dn9_slot = var_lnrtn_dn9;
        *var_lnrtn_rv_slot = var_lnrtn_rv;
        *var_mue_i_slot = var_mue_i;
        *var_mue_i_dn4_slot = var_mue_i_dn4;
        *var_mue_i_dn6_slot = var_mue_i_dn6;
        *var_mue_i_dn7_slot = var_mue_i_dn7;
        *var_mue_i_dn8_slot = var_mue_i_dn8;
        *var_mue_i_dn9_slot = var_mue_i_dn9;
        *var_mue_i_rv_slot = var_mue_i_rv;
        *var_one_m_eta_slot = var_one_m_eta;
        *var_one_m_eta_rv_slot = var_one_m_eta_rv;
        *var_qq_slot = var_qq;
        *var_qq_dn4_slot = var_qq_dn4;
        *var_qq_dn6_slot = var_qq_dn6;
        *var_qq_dn7_slot = var_qq_dn7;
        *var_qq_dn8_slot = var_qq_dn8;
        *var_qq_dn9_slot = var_qq_dn9;
        *var_qq_rv_slot = var_qq_rv;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_bet_dn4_slot = var_tf_bet_dn4;
        *var_tf_bet_dn6_slot = var_tf_bet_dn6;
        *var_tf_bet_dn7_slot = var_tf_bet_dn7;
        *var_tf_bet_dn8_slot = var_tf_bet_dn8;
        *var_tf_bet_dn9_slot = var_tf_bet_dn9;
        *var_tf_bet_rv_slot = var_tf_bet_rv;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_cs_dn4_slot = var_tf_cs_dn4;
        *var_tf_cs_dn6_slot = var_tf_cs_dn6;
        *var_tf_cs_dn7_slot = var_tf_cs_dn7;
        *var_tf_cs_dn8_slot = var_tf_cs_dn8;
        *var_tf_cs_dn9_slot = var_tf_cs_dn9;
        *var_tf_cs_rv_slot = var_tf_cs_rv;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_mue_dn4_slot = var_tf_mue_dn4;
        *var_tf_mue_dn6_slot = var_tf_mue_dn6;
        *var_tf_mue_dn7_slot = var_tf_mue_dn7;
        *var_tf_mue_dn8_slot = var_tf_mue_dn8;
        *var_tf_mue_dn9_slot = var_tf_mue_dn9;
        *var_tf_mue_rv_slot = var_tf_mue_rv;
        *var_tf_thecs_slot = var_tf_thecs;
        *var_tf_thecs_dn4_slot = var_tf_thecs_dn4;
        *var_tf_thecs_dn6_slot = var_tf_thecs_dn6;
        *var_tf_thecs_dn7_slot = var_tf_thecs_dn7;
        *var_tf_thecs_dn8_slot = var_tf_thecs_dn8;
        *var_tf_thecs_dn9_slot = var_tf_thecs_dn9;
        *var_tf_thecs_rv_slot = var_tf_thecs_rv;
        *var_tf_themu_slot = var_tf_themu;
        *var_tf_themu_dn4_slot = var_tf_themu_dn4;
        *var_tf_themu_dn6_slot = var_tf_themu_dn6;
        *var_tf_themu_dn7_slot = var_tf_themu_dn7;
        *var_tf_themu_dn8_slot = var_tf_themu_dn8;
        *var_tf_themu_dn9_slot = var_tf_themu_dn9;
        *var_tf_themu_rv_slot = var_tf_themu_rv;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_ther_dn4_slot = var_tf_ther_dn4;
        *var_tf_ther_dn6_slot = var_tf_ther_dn6;
        *var_tf_ther_dn7_slot = var_tf_ther_dn7;
        *var_tf_ther_dn8_slot = var_tf_ther_dn8;
        *var_tf_ther_dn9_slot = var_tf_ther_dn9;
        *var_tf_ther_rv_slot = var_tf_ther_rv;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_tf_xcor_dn4_slot = var_tf_xcor_dn4;
        *var_tf_xcor_dn6_slot = var_tf_xcor_dn6;
        *var_tf_xcor_dn7_slot = var_tf_xcor_dn7;
        *var_tf_xcor_dn8_slot = var_tf_xcor_dn8;
        *var_tf_xcor_dn9_slot = var_tf_xcor_dn9;
        *var_tf_xcor_rv_slot = var_tf_xcor_rv;
        *var_thecs_i_slot = var_thecs_i;
        *var_thecs_i_dn4_slot = var_thecs_i_dn4;
        *var_thecs_i_dn6_slot = var_thecs_i_dn6;
        *var_thecs_i_dn7_slot = var_thecs_i_dn7;
        *var_thecs_i_dn8_slot = var_thecs_i_dn8;
        *var_thecs_i_dn9_slot = var_thecs_i_dn9;
        *var_thecs_i_rv_slot = var_thecs_i_rv;
        *var_themu_i_slot = var_themu_i;
        *var_themu_i_dn4_slot = var_themu_i_dn4;
        *var_themu_i_dn6_slot = var_themu_i_dn6;
        *var_themu_i_dn7_slot = var_themu_i_dn7;
        *var_themu_i_dn8_slot = var_themu_i_dn8;
        *var_themu_i_dn9_slot = var_themu_i_dn9;
        *var_themu_i_rv_slot = var_themu_i_rv;
        *var_tsisq_slot = var_tsisq;
        *var_tsisq_rv_slot = var_tsisq_rv;
        *var_vfb1_i_slot = var_vfb1_i;
        *var_vfb1_i_dn4_slot = var_vfb1_i_dn4;
        *var_vfb1_i_dn6_slot = var_vfb1_i_dn6;
        *var_vfb1_i_dn7_slot = var_vfb1_i_dn7;
        *var_vfb1_i_dn8_slot = var_vfb1_i_dn8;
        *var_vfb1_i_dn9_slot = var_vfb1_i_dn9;
        *var_vfb1_i_rv_slot = var_vfb1_i_rv;
        *var_vfb2_i_slot = var_vfb2_i;
        *var_vfb2_i_dn4_slot = var_vfb2_i_dn4;
        *var_vfb2_i_dn6_slot = var_vfb2_i_dn6;
        *var_vfb2_i_dn7_slot = var_vfb2_i_dn7;
        *var_vfb2_i_dn8_slot = var_vfb2_i_dn8;
        *var_vfb2_i_dn9_slot = var_vfb2_i_dn9;
        *var_vfb2_i_rv_slot = var_vfb2_i_rv;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfb2_t_rv_slot = var_vfb2_t_rv;
        *var_vfbac1_i_slot = var_vfbac1_i;
        *var_vfbac1_i_dn4_slot = var_vfbac1_i_dn4;
        *var_vfbac1_i_dn6_slot = var_vfbac1_i_dn6;
        *var_vfbac1_i_dn7_slot = var_vfbac1_i_dn7;
        *var_vfbac1_i_dn8_slot = var_vfbac1_i_dn8;
        *var_vfbac1_i_dn9_slot = var_vfbac1_i_dn9;
        *var_vfbac1_i_rv_slot = var_vfbac1_i_rv;
        *var_vfbac2_i_slot = var_vfbac2_i;
        *var_vfbac2_i_dn4_slot = var_vfbac2_i_dn4;
        *var_vfbac2_i_dn6_slot = var_vfbac2_i_dn6;
        *var_vfbac2_i_dn7_slot = var_vfbac2_i_dn7;
        *var_vfbac2_i_dn8_slot = var_vfbac2_i_dn8;
        *var_vfbac2_i_dn9_slot = var_vfbac2_i_dn9;
        *var_vfbac2_i_rv_slot = var_vfbac2_i_rv;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbac2_t_rv_slot = var_vfbac2_t_rv;
        *var_xb_sub_slot = var_xb_sub;
        *var_xb_sub_dn4_slot = var_xb_sub_dn4;
        *var_xb_sub_dn6_slot = var_xb_sub_dn6;
        *var_xb_sub_dn7_slot = var_xb_sub_dn7;
        *var_xb_sub_dn8_slot = var_xb_sub_dn8;
        *var_xb_sub_dn9_slot = var_xb_sub_dn9;
        *var_xb_sub_rv_slot = var_xb_sub_rv;
        *var_xcor_i_slot = var_xcor_i;
        *var_xcor_i_dn4_slot = var_xcor_i_dn4;
        *var_xcor_i_dn6_slot = var_xcor_i_dn6;
        *var_xcor_i_dn7_slot = var_xcor_i_dn7;
        *var_xcor_i_dn8_slot = var_xcor_i_dn8;
        *var_xcor_i_dn9_slot = var_xcor_i_dn9;
        *var_xcor_i_rv_slot = var_xcor_i_rv;
        *var_xn_sub_slot = var_xn_sub;
        *var_xn_sub_dn4_slot = var_xn_sub_dn4;
        *var_xn_sub_dn6_slot = var_xn_sub_dn6;
        *var_xn_sub_dn7_slot = var_xn_sub_dn7;
        *var_xn_sub_dn8_slot = var_xn_sub_dn8;
        *var_xn_sub_dn9_slot = var_xn_sub_dn9;
        *var_xn_sub_rv_slot = var_xn_sub_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_a2_t: f64,
        var_alp1_i: f64,
        var_areaq_i: f64,
        var_ax_i: f64,
        var_axac_i: f64,
        var_betnedge_t: f64,
        var_betnedge_t_dn4: f64,
        var_betnedge_t_dn6: f64,
        var_betnedge_t_dn7: f64,
        var_betnedge_t_dn8: f64,
        var_betnedge_t_dn9: f64,
        var_bgidl_t: f64,
        var_bgidld_t: f64,
        var_chib_i: f64,
        var_ctedge_i: f64,
        var_dt: f64,
        var_dt_dn4: f64,
        var_dt_dn6: f64,
        var_dt_dn7: f64,
        var_dt_dn8: f64,
        var_dt_dn9: f64,
        var_dvfb1nch: f64,
        var_dvfb1nch_dn4: f64,
        var_dvfb1nch_dn6: f64,
        var_dvfb1nch_dn7: f64,
        var_dvfb1nch_dn8: f64,
        var_dvfb1nch_dn9: f64,
        var_dvfb2nch: f64,
        var_dvfb2nch_dn4: f64,
        var_dvfb2nch_dn6: f64,
        var_dvfb2nch_dn7: f64,
        var_dvfb2nch_dn8: f64,
        var_dvfb2nch_dn9: f64,
        var_dvfbch: f64,
        var_dvfbch_dn4: f64,
        var_dvfbch_dn6: f64,
        var_dvfbch_dn7: f64,
        var_dvfbch_dn8: f64,
        var_dvfbch_dn9: f64,
        var_dvfbpdep: f64,
        var_dvfbpdep_dn4: f64,
        var_dvfbpdep_dn6: f64,
        var_dvfbpdep_dn7: f64,
        var_dvfbpdep_dn8: f64,
        var_dvfbpdep_dn9: f64,
        var_dvfbqm: f64,
        var_eg: f64,
        var_eg_2phit: f64,
        var_eg_2phit_dn4: f64,
        var_eg_2phit_dn6: f64,
        var_eg_2phit_dn7: f64,
        var_eg_2phit_dn8: f64,
        var_eg_2phit_dn9: f64,
        var_eg_dn4: f64,
        var_eg_dn6: f64,
        var_eg_dn7: f64,
        var_eg_dn8: f64,
        var_eg_dn9: f64,
        var_epsch: f64,
        var_gc2ch_i: f64,
        var_gc2ovacc_i: f64,
        var_gc2ovinv_i: f64,
        var_gc3ch_i: f64,
        var_gc3ovacc_i: f64,
        var_gc3ovinv_i: f64,
        var_gco_i: f64,
        var_iginv_t: f64,
        var_igovacc_t: f64,
        var_igovaccd_t: f64,
        var_igovinv_t: f64,
        var_igovinvd_t: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_inv_phit_dn6: f64,
        var_inv_phit_dn7: f64,
        var_inv_phit_dn8: f64,
        var_inv_phit_dn9: f64,
        var_lnrtn: f64,
        var_lnrtn_dn4: f64,
        var_lnrtn_dn6: f64,
        var_lnrtn_dn7: f64,
        var_lnrtn_dn8: f64,
        var_lnrtn_dn9: f64,
        var_neff: f64,
        var_neff_dn4: f64,
        var_neff_dn6: f64,
        var_neff_dn7: f64,
        var_neff_dn8: f64,
        var_neff_dn9: f64,
        var_niginv_i: f64,
        var_phit: f64,
        var_phit0: f64,
        var_phit0_dn4: f64,
        var_phit0_dn6: f64,
        var_phit0_dn7: f64,
        var_phit0_dn8: f64,
        var_phit0_dn9: f64,
        var_phit_dn4: f64,
        var_phit_dn6: f64,
        var_phit_dn7: f64,
        var_phit_dn8: f64,
        var_phit_dn9: f64,
        var_rs_t: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_rtn_dn6: f64,
        var_rtn_dn7: f64,
        var_rtn_dn8: f64,
        var_rtn_dn9: f64,
        var_sta2_i: f64,
        var_stbetedge_i: f64,
        var_stbgidl_i: f64,
        var_stbgidld_i: f64,
        var_stig_i: f64,
        var_stigfn_i: f64,
        var_stthesat_i: f64,
        var_stvfbedge_i: f64,
        var_tf_bet: f64,
        var_tf_bet_dn4: f64,
        var_tf_bet_dn6: f64,
        var_tf_bet_dn7: f64,
        var_tf_bet_dn8: f64,
        var_tf_bet_dn9: f64,
        var_tf_ther: f64,
        var_tf_ther_dn4: f64,
        var_tf_ther_dn6: f64,
        var_tf_ther_dn7: f64,
        var_tf_ther_dn8: f64,
        var_tf_ther_dn9: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_thesatac_t: f64,
        var_thesatac_t_dn4: f64,
        var_thesatac_t_dn6: f64,
        var_thesatac_t_dn7: f64,
        var_thesatac_t_dn8: f64,
        var_thesatac_t_dn9: f64,
        var_toxp_i: f64,
        var_vfb1edge_t: f64,
        var_vfb1edge_t_dn4: f64,
        var_vfb1edge_t_dn6: f64,
        var_vfb1edge_t_dn7: f64,
        var_vfb1edge_t_dn8: f64,
        var_vfb1edge_t_dn9: f64,
        var_vfb2edge_t: f64,
        var_a0_csisq_edge_slot: &mut f64,
        var_a0_csisq_edge_dn4_slot: &mut f64,
        var_a0_csisq_edge_dn6_slot: &mut f64,
        var_a0_csisq_edge_dn7_slot: &mut f64,
        var_a0_csisq_edge_dn8_slot: &mut f64,
        var_a0_csisq_edge_dn9_slot: &mut f64,
        var_a0_csisq_edge_rv_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a2_i_dn4_slot: &mut f64,
        var_a2_i_dn6_slot: &mut f64,
        var_a2_i_dn7_slot: &mut f64,
        var_a2_i_dn8_slot: &mut f64,
        var_a2_i_dn9_slot: &mut f64,
        var_a2_i_rv_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_alp1_phit_slot: &mut f64,
        var_alp1_phit_dn4_slot: &mut f64,
        var_alp1_phit_dn6_slot: &mut f64,
        var_alp1_phit_dn7_slot: &mut f64,
        var_alp1_phit_dn8_slot: &mut f64,
        var_alp1_phit_dn9_slot: &mut f64,
        var_alp1_phit_rv_slot: &mut f64,
        var_alpha_b_slot: &mut f64,
        var_alpha_b_dn4_slot: &mut f64,
        var_alpha_b_dn6_slot: &mut f64,
        var_alpha_b_dn7_slot: &mut f64,
        var_alpha_b_dn8_slot: &mut f64,
        var_alpha_b_dn9_slot: &mut f64,
        var_alpha_b_rv_slot: &mut f64,
        var_area_phit_slot: &mut f64,
        var_area_phit_dn4_slot: &mut f64,
        var_area_phit_dn6_slot: &mut f64,
        var_area_phit_dn7_slot: &mut f64,
        var_area_phit_dn8_slot: &mut f64,
        var_area_phit_dn9_slot: &mut f64,
        var_area_phit_rv_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_bch_dn4_slot: &mut f64,
        var_bch_dn6_slot: &mut f64,
        var_bch_dn7_slot: &mut f64,
        var_bch_dn8_slot: &mut f64,
        var_bch_dn9_slot: &mut f64,
        var_bch_rv_slot: &mut f64,
        var_betnedge_i_slot: &mut f64,
        var_betnedge_i_dn4_slot: &mut f64,
        var_betnedge_i_dn6_slot: &mut f64,
        var_betnedge_i_dn7_slot: &mut f64,
        var_betnedge_i_dn8_slot: &mut f64,
        var_betnedge_i_dn9_slot: &mut f64,
        var_betnedge_i_rv_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidl_i_dn4_slot: &mut f64,
        var_bgidl_i_dn6_slot: &mut f64,
        var_bgidl_i_dn7_slot: &mut f64,
        var_bgidl_i_dn8_slot: &mut f64,
        var_bgidl_i_dn9_slot: &mut f64,
        var_bgidl_i_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_dn4_slot: &mut f64,
        var_bgidld_i_dn6_slot: &mut f64,
        var_bgidld_i_dn7_slot: &mut f64,
        var_bgidld_i_dn8_slot: &mut f64,
        var_bgidld_i_dn9_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_dn4_slot: &mut f64,
        var_bov_dn6_slot: &mut f64,
        var_bov_dn7_slot: &mut f64,
        var_bov_dn8_slot: &mut f64,
        var_bov_dn9_slot: &mut f64,
        var_bov_rv_slot: &mut f64,
        var_dch_slot: &mut f64,
        var_dch_dn4_slot: &mut f64,
        var_dch_dn6_slot: &mut f64,
        var_dch_dn7_slot: &mut f64,
        var_dch_dn8_slot: &mut f64,
        var_dch_dn9_slot: &mut f64,
        var_dch_rv_slot: &mut f64,
        var_dov_slot: &mut f64,
        var_dov_dn4_slot: &mut f64,
        var_dov_dn6_slot: &mut f64,
        var_dov_dn7_slot: &mut f64,
        var_dov_dn8_slot: &mut f64,
        var_dov_dn9_slot: &mut f64,
        var_dov_rv_slot: &mut f64,
        var_frs_slot: &mut f64,
        var_frs_dn4_slot: &mut f64,
        var_frs_dn6_slot: &mut f64,
        var_frs_dn7_slot: &mut f64,
        var_frs_dn8_slot: &mut f64,
        var_frs_dn9_slot: &mut f64,
        var_frs_rv_slot: &mut f64,
        var_gamax_slot: &mut f64,
        var_gamax_ac_slot: &mut f64,
        var_gamax_ac_rv_slot: &mut f64,
        var_gamax_rv_slot: &mut f64,
        var_gcqch_slot: &mut f64,
        var_gcqch_rv_slot: &mut f64,
        var_gcqovacc_slot: &mut f64,
        var_gcqovacc_rv_slot: &mut f64,
        var_gcqovinv_slot: &mut f64,
        var_gcqovinv_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_dn4_slot: &mut f64,
        var_iginv_i_dn6_slot: &mut f64,
        var_iginv_i_dn7_slot: &mut f64,
        var_iginv_i_dn8_slot: &mut f64,
        var_iginv_i_dn9_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igovacc_i_slot: &mut f64,
        var_igovacc_i_dn4_slot: &mut f64,
        var_igovacc_i_dn6_slot: &mut f64,
        var_igovacc_i_dn7_slot: &mut f64,
        var_igovacc_i_dn8_slot: &mut f64,
        var_igovacc_i_dn9_slot: &mut f64,
        var_igovacc_i_rv_slot: &mut f64,
        var_igovaccd_i_slot: &mut f64,
        var_igovaccd_i_dn4_slot: &mut f64,
        var_igovaccd_i_dn6_slot: &mut f64,
        var_igovaccd_i_dn7_slot: &mut f64,
        var_igovaccd_i_dn8_slot: &mut f64,
        var_igovaccd_i_dn9_slot: &mut f64,
        var_igovaccd_i_rv_slot: &mut f64,
        var_igovinv_i_slot: &mut f64,
        var_igovinv_i_dn4_slot: &mut f64,
        var_igovinv_i_dn6_slot: &mut f64,
        var_igovinv_i_dn7_slot: &mut f64,
        var_igovinv_i_dn8_slot: &mut f64,
        var_igovinv_i_dn9_slot: &mut f64,
        var_igovinv_i_rv_slot: &mut f64,
        var_igovinvd_i_slot: &mut f64,
        var_igovinvd_i_dn4_slot: &mut f64,
        var_igovinvd_i_dn6_slot: &mut f64,
        var_igovinvd_i_dn7_slot: &mut f64,
        var_igovinvd_i_dn8_slot: &mut f64,
        var_igovinvd_i_dn9_slot: &mut f64,
        var_igovinvd_i_rv_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_inv_chib_rv_slot: &mut f64,
        var_inv_phit_edge_slot: &mut f64,
        var_inv_phit_edge_dn4_slot: &mut f64,
        var_inv_phit_edge_dn6_slot: &mut f64,
        var_inv_phit_edge_dn7_slot: &mut f64,
        var_inv_phit_edge_dn8_slot: &mut f64,
        var_inv_phit_edge_dn9_slot: &mut f64,
        var_inv_phit_edge_rv_slot: &mut f64,
        var_n_iginv_slot: &mut f64,
        var_n_iginv_dn4_slot: &mut f64,
        var_n_iginv_dn6_slot: &mut f64,
        var_n_iginv_dn7_slot: &mut f64,
        var_n_iginv_dn8_slot: &mut f64,
        var_n_iginv_dn9_slot: &mut f64,
        var_n_iginv_rv_slot: &mut f64,
        var_phit_edge_slot: &mut f64,
        var_phit_edge_dn4_slot: &mut f64,
        var_phit_edge_dn6_slot: &mut f64,
        var_phit_edge_dn7_slot: &mut f64,
        var_phit_edge_dn8_slot: &mut f64,
        var_phit_edge_dn9_slot: &mut f64,
        var_phit_edge_rv_slot: &mut f64,
        var_rs_i_slot: &mut f64,
        var_rs_i_dn4_slot: &mut f64,
        var_rs_i_dn6_slot: &mut f64,
        var_rs_i_dn7_slot: &mut f64,
        var_rs_i_dn8_slot: &mut f64,
        var_rs_i_dn9_slot: &mut f64,
        var_rs_i_rv_slot: &mut f64,
        var_sat_phit_slot: &mut f64,
        var_sat_phit_ac_slot: &mut f64,
        var_sat_phit_ac_dn4_slot: &mut f64,
        var_sat_phit_ac_dn6_slot: &mut f64,
        var_sat_phit_ac_dn7_slot: &mut f64,
        var_sat_phit_ac_dn8_slot: &mut f64,
        var_sat_phit_ac_dn9_slot: &mut f64,
        var_sat_phit_ac_rv_slot: &mut f64,
        var_sat_phit_dn4_slot: &mut f64,
        var_sat_phit_dn6_slot: &mut f64,
        var_sat_phit_dn7_slot: &mut f64,
        var_sat_phit_dn8_slot: &mut f64,
        var_sat_phit_dn9_slot: &mut f64,
        var_sat_phit_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tempm_slot: &mut f64,
        var_tempm_dn4_slot: &mut f64,
        var_tempm_dn6_slot: &mut f64,
        var_tempm_dn7_slot: &mut f64,
        var_tempm_dn8_slot: &mut f64,
        var_tempm_dn9_slot: &mut f64,
        var_tempm_rv_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_tf_ig_dn4_slot: &mut f64,
        var_tf_ig_dn6_slot: &mut f64,
        var_tf_ig_dn7_slot: &mut f64,
        var_tf_ig_dn8_slot: &mut f64,
        var_tf_ig_dn9_slot: &mut f64,
        var_tf_ig_rv_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_thesat_dn4_slot: &mut f64,
        var_tf_thesat_dn6_slot: &mut f64,
        var_tf_thesat_dn7_slot: &mut f64,
        var_tf_thesat_dn8_slot: &mut f64,
        var_tf_thesat_dn9_slot: &mut f64,
        var_tf_thesat_rv_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesat_i_dn4_slot: &mut f64,
        var_thesat_i_dn6_slot: &mut f64,
        var_thesat_i_dn7_slot: &mut f64,
        var_thesat_i_dn8_slot: &mut f64,
        var_thesat_i_dn9_slot: &mut f64,
        var_thesat_i_rv_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_thesatac_i_dn4_slot: &mut f64,
        var_thesatac_i_dn6_slot: &mut f64,
        var_thesatac_i_dn7_slot: &mut f64,
        var_thesatac_i_dn8_slot: &mut f64,
        var_thesatac_i_dn9_slot: &mut f64,
        var_thesatac_i_rv_slot: &mut f64,
        var_vfb1edge_i_slot: &mut f64,
        var_vfb1edge_i_dn4_slot: &mut f64,
        var_vfb1edge_i_dn6_slot: &mut f64,
        var_vfb1edge_i_dn7_slot: &mut f64,
        var_vfb1edge_i_dn8_slot: &mut f64,
        var_vfb1edge_i_dn9_slot: &mut f64,
        var_vfb1edge_i_rv_slot: &mut f64,
        var_vfb2edge_i_slot: &mut f64,
        var_vfb2edge_i_dn4_slot: &mut f64,
        var_vfb2edge_i_dn6_slot: &mut f64,
        var_vfb2edge_i_dn7_slot: &mut f64,
        var_vfb2edge_i_dn8_slot: &mut f64,
        var_vfb2edge_i_dn9_slot: &mut f64,
        var_vfb2edge_i_rv_slot: &mut f64,
    ) {
        let mut var_a0_csisq_edge: f64 = *var_a0_csisq_edge_slot;
        let mut var_a0_csisq_edge_dn4: f64 = *var_a0_csisq_edge_dn4_slot;
        let mut var_a0_csisq_edge_dn6: f64 = *var_a0_csisq_edge_dn6_slot;
        let mut var_a0_csisq_edge_dn7: f64 = *var_a0_csisq_edge_dn7_slot;
        let mut var_a0_csisq_edge_dn8: f64 = *var_a0_csisq_edge_dn8_slot;
        let mut var_a0_csisq_edge_dn9: f64 = *var_a0_csisq_edge_dn9_slot;
        let mut var_a0_csisq_edge_rv: f64 = *var_a0_csisq_edge_rv_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a2_i_dn4: f64 = *var_a2_i_dn4_slot;
        let mut var_a2_i_dn6: f64 = *var_a2_i_dn6_slot;
        let mut var_a2_i_dn7: f64 = *var_a2_i_dn7_slot;
        let mut var_a2_i_dn8: f64 = *var_a2_i_dn8_slot;
        let mut var_a2_i_dn9: f64 = *var_a2_i_dn9_slot;
        let mut var_a2_i_rv: f64 = *var_a2_i_rv_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_alp1_phit: f64 = *var_alp1_phit_slot;
        let mut var_alp1_phit_dn4: f64 = *var_alp1_phit_dn4_slot;
        let mut var_alp1_phit_dn6: f64 = *var_alp1_phit_dn6_slot;
        let mut var_alp1_phit_dn7: f64 = *var_alp1_phit_dn7_slot;
        let mut var_alp1_phit_dn8: f64 = *var_alp1_phit_dn8_slot;
        let mut var_alp1_phit_dn9: f64 = *var_alp1_phit_dn9_slot;
        let mut var_alp1_phit_rv: f64 = *var_alp1_phit_rv_slot;
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_alpha_b_dn4: f64 = *var_alpha_b_dn4_slot;
        let mut var_alpha_b_dn6: f64 = *var_alpha_b_dn6_slot;
        let mut var_alpha_b_dn7: f64 = *var_alpha_b_dn7_slot;
        let mut var_alpha_b_dn8: f64 = *var_alpha_b_dn8_slot;
        let mut var_alpha_b_dn9: f64 = *var_alpha_b_dn9_slot;
        let mut var_alpha_b_rv: f64 = *var_alpha_b_rv_slot;
        let mut var_area_phit: f64 = *var_area_phit_slot;
        let mut var_area_phit_dn4: f64 = *var_area_phit_dn4_slot;
        let mut var_area_phit_dn6: f64 = *var_area_phit_dn6_slot;
        let mut var_area_phit_dn7: f64 = *var_area_phit_dn7_slot;
        let mut var_area_phit_dn8: f64 = *var_area_phit_dn8_slot;
        let mut var_area_phit_dn9: f64 = *var_area_phit_dn9_slot;
        let mut var_area_phit_rv: f64 = *var_area_phit_rv_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_bch_dn4: f64 = *var_bch_dn4_slot;
        let mut var_bch_dn6: f64 = *var_bch_dn6_slot;
        let mut var_bch_dn7: f64 = *var_bch_dn7_slot;
        let mut var_bch_dn8: f64 = *var_bch_dn8_slot;
        let mut var_bch_dn9: f64 = *var_bch_dn9_slot;
        let mut var_bch_rv: f64 = *var_bch_rv_slot;
        let mut var_betnedge_i: f64 = *var_betnedge_i_slot;
        let mut var_betnedge_i_dn4: f64 = *var_betnedge_i_dn4_slot;
        let mut var_betnedge_i_dn6: f64 = *var_betnedge_i_dn6_slot;
        let mut var_betnedge_i_dn7: f64 = *var_betnedge_i_dn7_slot;
        let mut var_betnedge_i_dn8: f64 = *var_betnedge_i_dn8_slot;
        let mut var_betnedge_i_dn9: f64 = *var_betnedge_i_dn9_slot;
        let mut var_betnedge_i_rv: f64 = *var_betnedge_i_rv_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidl_i_dn4: f64 = *var_bgidl_i_dn4_slot;
        let mut var_bgidl_i_dn6: f64 = *var_bgidl_i_dn6_slot;
        let mut var_bgidl_i_dn7: f64 = *var_bgidl_i_dn7_slot;
        let mut var_bgidl_i_dn8: f64 = *var_bgidl_i_dn8_slot;
        let mut var_bgidl_i_dn9: f64 = *var_bgidl_i_dn9_slot;
        let mut var_bgidl_i_rv: f64 = *var_bgidl_i_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_dn4: f64 = *var_bgidld_i_dn4_slot;
        let mut var_bgidld_i_dn6: f64 = *var_bgidld_i_dn6_slot;
        let mut var_bgidld_i_dn7: f64 = *var_bgidld_i_dn7_slot;
        let mut var_bgidld_i_dn8: f64 = *var_bgidld_i_dn8_slot;
        let mut var_bgidld_i_dn9: f64 = *var_bgidld_i_dn9_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_dn4: f64 = *var_bov_dn4_slot;
        let mut var_bov_dn6: f64 = *var_bov_dn6_slot;
        let mut var_bov_dn7: f64 = *var_bov_dn7_slot;
        let mut var_bov_dn8: f64 = *var_bov_dn8_slot;
        let mut var_bov_dn9: f64 = *var_bov_dn9_slot;
        let mut var_bov_rv: f64 = *var_bov_rv_slot;
        let mut var_dch: f64 = *var_dch_slot;
        let mut var_dch_dn4: f64 = *var_dch_dn4_slot;
        let mut var_dch_dn6: f64 = *var_dch_dn6_slot;
        let mut var_dch_dn7: f64 = *var_dch_dn7_slot;
        let mut var_dch_dn8: f64 = *var_dch_dn8_slot;
        let mut var_dch_dn9: f64 = *var_dch_dn9_slot;
        let mut var_dch_rv: f64 = *var_dch_rv_slot;
        let mut var_dov: f64 = *var_dov_slot;
        let mut var_dov_dn4: f64 = *var_dov_dn4_slot;
        let mut var_dov_dn6: f64 = *var_dov_dn6_slot;
        let mut var_dov_dn7: f64 = *var_dov_dn7_slot;
        let mut var_dov_dn8: f64 = *var_dov_dn8_slot;
        let mut var_dov_dn9: f64 = *var_dov_dn9_slot;
        let mut var_dov_rv: f64 = *var_dov_rv_slot;
        let mut var_frs: f64 = *var_frs_slot;
        let mut var_frs_dn4: f64 = *var_frs_dn4_slot;
        let mut var_frs_dn6: f64 = *var_frs_dn6_slot;
        let mut var_frs_dn7: f64 = *var_frs_dn7_slot;
        let mut var_frs_dn8: f64 = *var_frs_dn8_slot;
        let mut var_frs_dn9: f64 = *var_frs_dn9_slot;
        let mut var_frs_rv: f64 = *var_frs_rv_slot;
        let mut var_gamax: f64 = *var_gamax_slot;
        let mut var_gamax_ac: f64 = *var_gamax_ac_slot;
        let mut var_gamax_ac_rv: f64 = *var_gamax_ac_rv_slot;
        let mut var_gamax_rv: f64 = *var_gamax_rv_slot;
        let mut var_gcqch: f64 = *var_gcqch_slot;
        let mut var_gcqch_rv: f64 = *var_gcqch_rv_slot;
        let mut var_gcqovacc: f64 = *var_gcqovacc_slot;
        let mut var_gcqovacc_rv: f64 = *var_gcqovacc_rv_slot;
        let mut var_gcqovinv: f64 = *var_gcqovinv_slot;
        let mut var_gcqovinv_rv: f64 = *var_gcqovinv_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_dn4: f64 = *var_iginv_i_dn4_slot;
        let mut var_iginv_i_dn6: f64 = *var_iginv_i_dn6_slot;
        let mut var_iginv_i_dn7: f64 = *var_iginv_i_dn7_slot;
        let mut var_iginv_i_dn8: f64 = *var_iginv_i_dn8_slot;
        let mut var_iginv_i_dn9: f64 = *var_iginv_i_dn9_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igovacc_i: f64 = *var_igovacc_i_slot;
        let mut var_igovacc_i_dn4: f64 = *var_igovacc_i_dn4_slot;
        let mut var_igovacc_i_dn6: f64 = *var_igovacc_i_dn6_slot;
        let mut var_igovacc_i_dn7: f64 = *var_igovacc_i_dn7_slot;
        let mut var_igovacc_i_dn8: f64 = *var_igovacc_i_dn8_slot;
        let mut var_igovacc_i_dn9: f64 = *var_igovacc_i_dn9_slot;
        let mut var_igovacc_i_rv: f64 = *var_igovacc_i_rv_slot;
        let mut var_igovaccd_i: f64 = *var_igovaccd_i_slot;
        let mut var_igovaccd_i_dn4: f64 = *var_igovaccd_i_dn4_slot;
        let mut var_igovaccd_i_dn6: f64 = *var_igovaccd_i_dn6_slot;
        let mut var_igovaccd_i_dn7: f64 = *var_igovaccd_i_dn7_slot;
        let mut var_igovaccd_i_dn8: f64 = *var_igovaccd_i_dn8_slot;
        let mut var_igovaccd_i_dn9: f64 = *var_igovaccd_i_dn9_slot;
        let mut var_igovaccd_i_rv: f64 = *var_igovaccd_i_rv_slot;
        let mut var_igovinv_i: f64 = *var_igovinv_i_slot;
        let mut var_igovinv_i_dn4: f64 = *var_igovinv_i_dn4_slot;
        let mut var_igovinv_i_dn6: f64 = *var_igovinv_i_dn6_slot;
        let mut var_igovinv_i_dn7: f64 = *var_igovinv_i_dn7_slot;
        let mut var_igovinv_i_dn8: f64 = *var_igovinv_i_dn8_slot;
        let mut var_igovinv_i_dn9: f64 = *var_igovinv_i_dn9_slot;
        let mut var_igovinv_i_rv: f64 = *var_igovinv_i_rv_slot;
        let mut var_igovinvd_i: f64 = *var_igovinvd_i_slot;
        let mut var_igovinvd_i_dn4: f64 = *var_igovinvd_i_dn4_slot;
        let mut var_igovinvd_i_dn6: f64 = *var_igovinvd_i_dn6_slot;
        let mut var_igovinvd_i_dn7: f64 = *var_igovinvd_i_dn7_slot;
        let mut var_igovinvd_i_dn8: f64 = *var_igovinvd_i_dn8_slot;
        let mut var_igovinvd_i_dn9: f64 = *var_igovinvd_i_dn9_slot;
        let mut var_igovinvd_i_rv: f64 = *var_igovinvd_i_rv_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_inv_chib_rv: f64 = *var_inv_chib_rv_slot;
        let mut var_inv_phit_edge: f64 = *var_inv_phit_edge_slot;
        let mut var_inv_phit_edge_dn4: f64 = *var_inv_phit_edge_dn4_slot;
        let mut var_inv_phit_edge_dn6: f64 = *var_inv_phit_edge_dn6_slot;
        let mut var_inv_phit_edge_dn7: f64 = *var_inv_phit_edge_dn7_slot;
        let mut var_inv_phit_edge_dn8: f64 = *var_inv_phit_edge_dn8_slot;
        let mut var_inv_phit_edge_dn9: f64 = *var_inv_phit_edge_dn9_slot;
        let mut var_inv_phit_edge_rv: f64 = *var_inv_phit_edge_rv_slot;
        let mut var_n_iginv: f64 = *var_n_iginv_slot;
        let mut var_n_iginv_dn4: f64 = *var_n_iginv_dn4_slot;
        let mut var_n_iginv_dn6: f64 = *var_n_iginv_dn6_slot;
        let mut var_n_iginv_dn7: f64 = *var_n_iginv_dn7_slot;
        let mut var_n_iginv_dn8: f64 = *var_n_iginv_dn8_slot;
        let mut var_n_iginv_dn9: f64 = *var_n_iginv_dn9_slot;
        let mut var_n_iginv_rv: f64 = *var_n_iginv_rv_slot;
        let mut var_phit_edge: f64 = *var_phit_edge_slot;
        let mut var_phit_edge_dn4: f64 = *var_phit_edge_dn4_slot;
        let mut var_phit_edge_dn6: f64 = *var_phit_edge_dn6_slot;
        let mut var_phit_edge_dn7: f64 = *var_phit_edge_dn7_slot;
        let mut var_phit_edge_dn8: f64 = *var_phit_edge_dn8_slot;
        let mut var_phit_edge_dn9: f64 = *var_phit_edge_dn9_slot;
        let mut var_phit_edge_rv: f64 = *var_phit_edge_rv_slot;
        let mut var_rs_i: f64 = *var_rs_i_slot;
        let mut var_rs_i_dn4: f64 = *var_rs_i_dn4_slot;
        let mut var_rs_i_dn6: f64 = *var_rs_i_dn6_slot;
        let mut var_rs_i_dn7: f64 = *var_rs_i_dn7_slot;
        let mut var_rs_i_dn8: f64 = *var_rs_i_dn8_slot;
        let mut var_rs_i_dn9: f64 = *var_rs_i_dn9_slot;
        let mut var_rs_i_rv: f64 = *var_rs_i_rv_slot;
        let mut var_sat_phit: f64 = *var_sat_phit_slot;
        let mut var_sat_phit_ac: f64 = *var_sat_phit_ac_slot;
        let mut var_sat_phit_ac_dn4: f64 = *var_sat_phit_ac_dn4_slot;
        let mut var_sat_phit_ac_dn6: f64 = *var_sat_phit_ac_dn6_slot;
        let mut var_sat_phit_ac_dn7: f64 = *var_sat_phit_ac_dn7_slot;
        let mut var_sat_phit_ac_dn8: f64 = *var_sat_phit_ac_dn8_slot;
        let mut var_sat_phit_ac_dn9: f64 = *var_sat_phit_ac_dn9_slot;
        let mut var_sat_phit_ac_rv: f64 = *var_sat_phit_ac_rv_slot;
        let mut var_sat_phit_dn4: f64 = *var_sat_phit_dn4_slot;
        let mut var_sat_phit_dn6: f64 = *var_sat_phit_dn6_slot;
        let mut var_sat_phit_dn7: f64 = *var_sat_phit_dn7_slot;
        let mut var_sat_phit_dn8: f64 = *var_sat_phit_dn8_slot;
        let mut var_sat_phit_dn9: f64 = *var_sat_phit_dn9_slot;
        let mut var_sat_phit_rv: f64 = *var_sat_phit_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tempm: f64 = *var_tempm_slot;
        let mut var_tempm_dn4: f64 = *var_tempm_dn4_slot;
        let mut var_tempm_dn6: f64 = *var_tempm_dn6_slot;
        let mut var_tempm_dn7: f64 = *var_tempm_dn7_slot;
        let mut var_tempm_dn8: f64 = *var_tempm_dn8_slot;
        let mut var_tempm_dn9: f64 = *var_tempm_dn9_slot;
        let mut var_tempm_rv: f64 = *var_tempm_rv_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_tf_ig_dn4: f64 = *var_tf_ig_dn4_slot;
        let mut var_tf_ig_dn6: f64 = *var_tf_ig_dn6_slot;
        let mut var_tf_ig_dn7: f64 = *var_tf_ig_dn7_slot;
        let mut var_tf_ig_dn8: f64 = *var_tf_ig_dn8_slot;
        let mut var_tf_ig_dn9: f64 = *var_tf_ig_dn9_slot;
        let mut var_tf_ig_rv: f64 = *var_tf_ig_rv_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_thesat_dn4: f64 = *var_tf_thesat_dn4_slot;
        let mut var_tf_thesat_dn6: f64 = *var_tf_thesat_dn6_slot;
        let mut var_tf_thesat_dn7: f64 = *var_tf_thesat_dn7_slot;
        let mut var_tf_thesat_dn8: f64 = *var_tf_thesat_dn8_slot;
        let mut var_tf_thesat_dn9: f64 = *var_tf_thesat_dn9_slot;
        let mut var_tf_thesat_rv: f64 = *var_tf_thesat_rv_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesat_i_dn4: f64 = *var_thesat_i_dn4_slot;
        let mut var_thesat_i_dn6: f64 = *var_thesat_i_dn6_slot;
        let mut var_thesat_i_dn7: f64 = *var_thesat_i_dn7_slot;
        let mut var_thesat_i_dn8: f64 = *var_thesat_i_dn8_slot;
        let mut var_thesat_i_dn9: f64 = *var_thesat_i_dn9_slot;
        let mut var_thesat_i_rv: f64 = *var_thesat_i_rv_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_thesatac_i_dn4: f64 = *var_thesatac_i_dn4_slot;
        let mut var_thesatac_i_dn6: f64 = *var_thesatac_i_dn6_slot;
        let mut var_thesatac_i_dn7: f64 = *var_thesatac_i_dn7_slot;
        let mut var_thesatac_i_dn8: f64 = *var_thesatac_i_dn8_slot;
        let mut var_thesatac_i_dn9: f64 = *var_thesatac_i_dn9_slot;
        let mut var_thesatac_i_rv: f64 = *var_thesatac_i_rv_slot;
        let mut var_vfb1edge_i: f64 = *var_vfb1edge_i_slot;
        let mut var_vfb1edge_i_dn4: f64 = *var_vfb1edge_i_dn4_slot;
        let mut var_vfb1edge_i_dn6: f64 = *var_vfb1edge_i_dn6_slot;
        let mut var_vfb1edge_i_dn7: f64 = *var_vfb1edge_i_dn7_slot;
        let mut var_vfb1edge_i_dn8: f64 = *var_vfb1edge_i_dn8_slot;
        let mut var_vfb1edge_i_dn9: f64 = *var_vfb1edge_i_dn9_slot;
        let mut var_vfb1edge_i_rv: f64 = *var_vfb1edge_i_rv_slot;
        let mut var_vfb2edge_i: f64 = *var_vfb2edge_i_slot;
        let mut var_vfb2edge_i_dn4: f64 = *var_vfb2edge_i_dn4_slot;
        let mut var_vfb2edge_i_dn6: f64 = *var_vfb2edge_i_dn6_slot;
        let mut var_vfb2edge_i_dn7: f64 = *var_vfb2edge_i_dn7_slot;
        let mut var_vfb2edge_i_dn8: f64 = *var_vfb2edge_i_dn8_slot;
        let mut var_vfb2edge_i_dn9: f64 = *var_vfb2edge_i_dn9_slot;
        let mut var_vfb2edge_i_rv: f64 = *var_vfb2edge_i_rv_slot;

        let assign7620_e7173: f64 = (var_rs_t * var_tf_ther);
        var_rs_i = assign7620_e7173;
        var_rs_i_dn4 = (var_rs_t * var_tf_ther_dn4);
        var_rs_i_dn6 = (var_rs_t * var_tf_ther_dn6);
        var_rs_i_dn7 = (var_rs_t * var_tf_ther_dn7);
        var_rs_i_dn8 = (var_rs_t * var_tf_ther_dn8);
        var_rs_i_dn9 = (var_rs_t * var_tf_ther_dn9);
        var_rs_i_rv = 0.0;

        let assign7630_e7176: f64 = (2.0 * var_rs_i);
        let assign7630_e7178: f64 = (assign7630_e7176 * var_phit);
        var_frs = assign7630_e7178;
        var_frs_dn4 = (((2.0 * var_rs_i_dn4) * var_phit) + (assign7630_e7176 * var_phit_dn4));
        var_frs_dn6 = (((2.0 * var_rs_i_dn6) * var_phit) + (assign7630_e7176 * var_phit_dn6));
        var_frs_dn7 = (((2.0 * var_rs_i_dn7) * var_phit) + (assign7630_e7176 * var_phit_dn7));
        var_frs_dn8 = (((2.0 * var_rs_i_dn8) * var_phit) + (assign7630_e7176 * var_phit_dn8));
        var_frs_dn9 = (((2.0 * var_rs_i_dn9) * var_phit) + (assign7630_e7176 * var_phit_dn9));
        var_frs_rv = 0.0;

        let assign7640_e7182: f64 = (16.0 / var_ax_i);
        let assign7640_e7184: f64 = (assign7640_e7182 * 0.6931471805599);
        let assign7640_e7185: f64 = (assign7640_e7184).exp();
        let assign7640_e7187: f64 = (assign7640_e7185 - 1.0);
        let assign7640_e7188: f64 = (assign7640_e7187).ln();
        let assign7640_e7189: f64 = (0.375 * assign7640_e7188);
        let assign7640_e7190: f64 = (assign7640_e7189).exp();
        let assign7640_e7192: f64 = (assign7640_e7190 - 1.0);
        var_gamax = assign7640_e7192;
        var_gamax_rv = 0.0;

        let assign7650_e7196: f64 = (16.0 / var_axac_i);
        let assign7650_e7198: f64 = (assign7650_e7196 * 0.6931471805599);
        let assign7650_e7199: f64 = (assign7650_e7198).exp();
        let assign7650_e7201: f64 = (assign7650_e7199 - 1.0);
        let assign7650_e7202: f64 = (assign7650_e7201).ln();
        let assign7650_e7203: f64 = (0.375 * assign7650_e7202);
        let assign7650_e7204: f64 = (assign7650_e7203).exp();
        let assign7650_e7206: f64 = (assign7650_e7204 - 1.0);
        var_gamax_ac = assign7650_e7206;
        var_gamax_ac_rv = 0.0;

        let assign7660_e7209: f64 = (var_stthesat_i * var_lnrtn);
        let assign7660_e7210: f64 = (assign7660_e7209).exp();
        var_tf_thesat = assign7660_e7210;
        var_tf_thesat_dn4 = (assign7660_e7210 * (var_stthesat_i * var_lnrtn_dn4));
        var_tf_thesat_dn6 = (assign7660_e7210 * (var_stthesat_i * var_lnrtn_dn6));
        var_tf_thesat_dn7 = (assign7660_e7210 * (var_stthesat_i * var_lnrtn_dn7));
        var_tf_thesat_dn8 = (assign7660_e7210 * (var_stthesat_i * var_lnrtn_dn8));
        var_tf_thesat_dn9 = (assign7660_e7210 * (var_stthesat_i * var_lnrtn_dn9));
        var_tf_thesat_rv = 0.0;

        let assign7670_e7213: f64 = (var_thesat_t * var_tf_thesat);
        let assign7670_e7215: f64 = (assign7670_e7213 * var_tf_bet);
        var_thesat_i = assign7670_e7215;
        var_thesat_i_dn4 = ((((var_thesat_t_dn4 * var_tf_thesat) + (var_thesat_t * var_tf_thesat_dn4)) * var_tf_bet) + (assign7670_e7213 * var_tf_bet_dn4));
        var_thesat_i_dn6 = ((((var_thesat_t_dn6 * var_tf_thesat) + (var_thesat_t * var_tf_thesat_dn6)) * var_tf_bet) + (assign7670_e7213 * var_tf_bet_dn6));
        var_thesat_i_dn7 = ((((var_thesat_t_dn7 * var_tf_thesat) + (var_thesat_t * var_tf_thesat_dn7)) * var_tf_bet) + (assign7670_e7213 * var_tf_bet_dn7));
        var_thesat_i_dn8 = ((((var_thesat_t_dn8 * var_tf_thesat) + (var_thesat_t * var_tf_thesat_dn8)) * var_tf_bet) + (assign7670_e7213 * var_tf_bet_dn8));
        var_thesat_i_dn9 = ((((var_thesat_t_dn9 * var_tf_thesat) + (var_thesat_t * var_tf_thesat_dn9)) * var_tf_bet) + (assign7670_e7213 * var_tf_bet_dn9));
        var_thesat_i_rv = 0.0;

        let assign7680_e7218: f64 = (var_thesat_i * var_phit);
        var_sat_phit = assign7680_e7218;
        var_sat_phit_dn4 = ((var_thesat_i_dn4 * var_phit) + (var_thesat_i * var_phit_dn4));
        var_sat_phit_dn6 = ((var_thesat_i_dn6 * var_phit) + (var_thesat_i * var_phit_dn6));
        var_sat_phit_dn7 = ((var_thesat_i_dn7 * var_phit) + (var_thesat_i * var_phit_dn7));
        var_sat_phit_dn8 = ((var_thesat_i_dn8 * var_phit) + (var_thesat_i * var_phit_dn8));
        var_sat_phit_dn9 = ((var_thesat_i_dn9 * var_phit) + (var_thesat_i * var_phit_dn9));
        var_sat_phit_rv = 0.0;

        let assign7690_e7221: f64 = (var_thesatac_t * var_tf_thesat);
        let assign7690_e7223: f64 = (assign7690_e7221 * var_tf_bet);
        var_thesatac_i = assign7690_e7223;
        var_thesatac_i_dn4 = ((((var_thesatac_t_dn4 * var_tf_thesat) + (var_thesatac_t * var_tf_thesat_dn4)) * var_tf_bet) + (assign7690_e7221 * var_tf_bet_dn4));
        var_thesatac_i_dn6 = ((((var_thesatac_t_dn6 * var_tf_thesat) + (var_thesatac_t * var_tf_thesat_dn6)) * var_tf_bet) + (assign7690_e7221 * var_tf_bet_dn6));
        var_thesatac_i_dn7 = ((((var_thesatac_t_dn7 * var_tf_thesat) + (var_thesatac_t * var_tf_thesat_dn7)) * var_tf_bet) + (assign7690_e7221 * var_tf_bet_dn7));
        var_thesatac_i_dn8 = ((((var_thesatac_t_dn8 * var_tf_thesat) + (var_thesatac_t * var_tf_thesat_dn8)) * var_tf_bet) + (assign7690_e7221 * var_tf_bet_dn8));
        var_thesatac_i_dn9 = ((((var_thesatac_t_dn9 * var_tf_thesat) + (var_thesatac_t * var_tf_thesat_dn9)) * var_tf_bet) + (assign7690_e7221 * var_tf_bet_dn9));
        var_thesatac_i_rv = 0.0;

        let assign7700_e7226: f64 = (var_thesatac_i * var_phit);
        var_sat_phit_ac = assign7700_e7226;
        var_sat_phit_ac_dn4 = ((var_thesatac_i_dn4 * var_phit) + (var_thesatac_i * var_phit_dn4));
        var_sat_phit_ac_dn6 = ((var_thesatac_i_dn6 * var_phit) + (var_thesatac_i * var_phit_dn6));
        var_sat_phit_ac_dn7 = ((var_thesatac_i_dn7 * var_phit) + (var_thesatac_i * var_phit_dn7));
        var_sat_phit_ac_dn8 = ((var_thesatac_i_dn8 * var_phit) + (var_thesatac_i * var_phit_dn8));
        var_sat_phit_ac_dn9 = ((var_thesatac_i_dn9 * var_phit) + (var_thesatac_i * var_phit_dn9));
        var_sat_phit_ac_rv = 0.0;

        let assign7710_e7229: f64 = (var_alp1_i * var_inv_phit);
        var_alp1_phit = assign7710_e7229;
        var_alp1_phit_dn4 = (var_alp1_i * var_inv_phit_dn4);
        var_alp1_phit_dn6 = (var_alp1_i * var_inv_phit_dn6);
        var_alp1_phit_dn7 = (var_alp1_i * var_inv_phit_dn7);
        var_alp1_phit_dn8 = (var_alp1_i * var_inv_phit_dn8);
        var_alp1_phit_dn9 = (var_alp1_i * var_inv_phit_dn9);
        var_alp1_phit_rv = 0.0;

        let assign7720_e7231: f64 = (-var_stig_i);
        let assign7720_e7233: f64 = (assign7720_e7231 * var_lnrtn);
        let assign7720_e7234: f64 = (assign7720_e7233).exp();
        var_tf_ig = assign7720_e7234;
        var_tf_ig_dn4 = (assign7720_e7234 * (assign7720_e7231 * var_lnrtn_dn4));
        var_tf_ig_dn6 = (assign7720_e7234 * (assign7720_e7231 * var_lnrtn_dn6));
        var_tf_ig_dn7 = (assign7720_e7234 * (assign7720_e7231 * var_lnrtn_dn7));
        var_tf_ig_dn8 = (assign7720_e7234 * (assign7720_e7231 * var_lnrtn_dn8));
        var_tf_ig_dn9 = (assign7720_e7234 * (assign7720_e7231 * var_lnrtn_dn9));
        var_tf_ig_rv = 0.0;

        let assign7730_e7237: f64 = (var_iginv_t * var_tf_ig);
        var_iginv_i = assign7730_e7237;
        var_iginv_i_dn4 = (var_iginv_t * var_tf_ig_dn4);
        var_iginv_i_dn6 = (var_iginv_t * var_tf_ig_dn6);
        var_iginv_i_dn7 = (var_iginv_t * var_tf_ig_dn7);
        var_iginv_i_dn8 = (var_iginv_t * var_tf_ig_dn8);
        var_iginv_i_dn9 = (var_iginv_t * var_tf_ig_dn9);
        var_iginv_i_rv = 0.0;

        let assign7740_e7240: f64 = (var_igovinv_t * var_tf_ig);
        var_igovinv_i = assign7740_e7240;
        var_igovinv_i_dn4 = (var_igovinv_t * var_tf_ig_dn4);
        var_igovinv_i_dn6 = (var_igovinv_t * var_tf_ig_dn6);
        var_igovinv_i_dn7 = (var_igovinv_t * var_tf_ig_dn7);
        var_igovinv_i_dn8 = (var_igovinv_t * var_tf_ig_dn8);
        var_igovinv_i_dn9 = (var_igovinv_t * var_tf_ig_dn9);
        var_igovinv_i_rv = 0.0;

        let assign7750_e7243: f64 = (var_igovinvd_t * var_tf_ig);
        var_igovinvd_i = assign7750_e7243;
        var_igovinvd_i_dn4 = (var_igovinvd_t * var_tf_ig_dn4);
        var_igovinvd_i_dn6 = (var_igovinvd_t * var_tf_ig_dn6);
        var_igovinvd_i_dn7 = (var_igovinvd_t * var_tf_ig_dn7);
        var_igovinvd_i_dn8 = (var_igovinvd_t * var_tf_ig_dn8);
        var_igovinvd_i_dn9 = (var_igovinvd_t * var_tf_ig_dn9);
        var_igovinvd_i_rv = 0.0;

        let assign7760_e7246: f64 = (var_igovacc_t * var_tf_ig);
        var_igovacc_i = assign7760_e7246;
        var_igovacc_i_dn4 = (var_igovacc_t * var_tf_ig_dn4);
        var_igovacc_i_dn6 = (var_igovacc_t * var_tf_ig_dn6);
        var_igovacc_i_dn7 = (var_igovacc_t * var_tf_ig_dn7);
        var_igovacc_i_dn8 = (var_igovacc_t * var_tf_ig_dn8);
        var_igovacc_i_dn9 = (var_igovacc_t * var_tf_ig_dn9);
        var_igovacc_i_rv = 0.0;

        let assign7770_e7249: f64 = (var_igovaccd_t * var_tf_ig);
        var_igovaccd_i = assign7770_e7249;
        var_igovaccd_i_dn4 = (var_igovaccd_t * var_tf_ig_dn4);
        var_igovaccd_i_dn6 = (var_igovaccd_t * var_tf_ig_dn6);
        var_igovaccd_i_dn7 = (var_igovaccd_t * var_tf_ig_dn7);
        var_igovaccd_i_dn8 = (var_igovaccd_t * var_tf_ig_dn8);
        var_igovaccd_i_dn9 = (var_igovaccd_t * var_tf_ig_dn9);
        var_igovaccd_i_rv = 0.0;

        let assign7780_e7251: f64 = (-var_stigfn_i);
        let assign7780_e7253: f64 = (assign7780_e7251 * var_lnrtn);
        let assign7780_e7254: f64 = (assign7780_e7253).exp();
        var_tf_ig = assign7780_e7254;
        var_tf_ig_dn4 = (assign7780_e7254 * (assign7780_e7251 * var_lnrtn_dn4));
        var_tf_ig_dn6 = (assign7780_e7254 * (assign7780_e7251 * var_lnrtn_dn6));
        var_tf_ig_dn7 = (assign7780_e7254 * (assign7780_e7251 * var_lnrtn_dn7));
        var_tf_ig_dn8 = (assign7780_e7254 * (assign7780_e7251 * var_lnrtn_dn8));
        var_tf_ig_dn9 = (assign7780_e7254 * (assign7780_e7251 * var_lnrtn_dn9));
        var_tf_ig_rv = 0.0;

        let assign7810_e7263: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign7810_e7263;
        var_inv_chib_rv = 0.0;

        let assign7820_e7266: f64 = (4.0 * 0.3333333333333);
        let assign7820_e7269: f64 = (2.0 * 1.602176565e-19);
        let assign7820_e7271: f64 = (assign7820_e7269 * 9.10938291e-31);
        let assign7820_e7273: f64 = (assign7820_e7271 * var_chib_i);
        let assign7820_e7274: f64 = (assign7820_e7273).sqrt();
        let assign7820_e7275: f64 = (assign7820_e7266 * assign7820_e7274);
        let assign7820_e7277: f64 = (assign7820_e7275 / 1.054571726e-34);
        var_tempm = assign7820_e7277;
        var_tempm_dn4 = 0.0;
        var_tempm_dn6 = 0.0;
        var_tempm_dn7 = 0.0;
        var_tempm_dn8 = 0.0;
        var_tempm_dn9 = 0.0;
        var_tempm_rv = 0.0;

        let assign7830_e7280: f64 = (var_tempm * var_toxp_i);
        var_bch = assign7830_e7280;
        var_bch_dn4 = (var_tempm_dn4 * var_toxp_i);
        var_bch_dn6 = (var_tempm_dn6 * var_toxp_i);
        var_bch_dn7 = (var_tempm_dn7 * var_toxp_i);
        var_bch_dn8 = (var_tempm_dn8 * var_toxp_i);
        var_bch_dn9 = (var_tempm_dn9 * var_toxp_i);
        var_bch_rv = 0.0;

        let assign7840_e7283: f64 = (var_tempm * var_toxp_i);
        var_bov = assign7840_e7283;
        var_bov_dn4 = (var_tempm_dn4 * var_toxp_i);
        var_bov_dn6 = (var_tempm_dn6 * var_toxp_i);
        var_bov_dn7 = (var_tempm_dn7 * var_toxp_i);
        var_bov_dn8 = (var_tempm_dn8 * var_toxp_i);
        var_bov_dn9 = (var_tempm_dn9 * var_toxp_i);
        var_bov_rv = 0.0;

        var_gcqch = 0.0;
        var_gcqch_rv = 0.0;

        let assign7860_e7287: f64 = if var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        var_guard148 = assign7860_e7287;
        var_guard148_rv = 0.0;

        let (assign7870_e7296,) = {
    if (var_guard148 != 0.0) {
        let assign7870_e7290: f64 = (-0.495);
        let assign7870_e7292: f64 = (assign7870_e7290 * var_gc2ch_i);
        let assign7870_e7294: f64 = (assign7870_e7292 / var_gc3ch_i);
        (assign7870_e7294,)
    } else {
        (var_gcqch,)
    }
};
        var_gcqch = assign7870_e7296;
        var_gcqch_rv = 0.0;

        var_gcqovinv = 0.0;
        var_gcqovinv_rv = 0.0;

        let assign7890_e7300: f64 = if var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        var_guard149 = assign7890_e7300;
        var_guard149_rv = 0.0;

        let (assign7900_e7309,) = {
    if (var_guard149 != 0.0) {
        let assign7900_e7303: f64 = (-0.495);
        let assign7900_e7305: f64 = (assign7900_e7303 * var_gc2ovinv_i);
        let assign7900_e7307: f64 = (assign7900_e7305 / var_gc3ovinv_i);
        (assign7900_e7307,)
    } else {
        (var_gcqovinv,)
    }
};
        var_gcqovinv = assign7900_e7309;
        var_gcqovinv_rv = 0.0;

        var_gcqovacc = 0.0;
        var_gcqovacc_rv = 0.0;

        let assign7920_e7313: f64 = if var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        var_guard150 = assign7920_e7313;
        var_guard150_rv = 0.0;

        let (assign7930_e7322,) = {
    if (var_guard150 != 0.0) {
        let assign7930_e7316: f64 = (-0.495);
        let assign7930_e7318: f64 = (assign7930_e7316 * var_gc2ovacc_i);
        let assign7930_e7320: f64 = (assign7930_e7318 / var_gc3ovacc_i);
        (assign7930_e7320,)
    } else {
        (var_gcqovacc,)
    }
};
        var_gcqovacc = assign7930_e7322;
        var_gcqovacc_rv = 0.0;

        let assign7940_e7325: f64 = (0.5 * var_eg);
        var_alpha_b = assign7940_e7325;
        var_alpha_b_dn4 = (0.5 * var_eg_dn4);
        var_alpha_b_dn6 = (0.5 * var_eg_dn6);
        var_alpha_b_dn7 = (0.5 * var_eg_dn7);
        var_alpha_b_dn8 = (0.5 * var_eg_dn8);
        var_alpha_b_dn9 = (0.5 * var_eg_dn9);
        var_alpha_b_rv = 0.0;

        let assign7950_e7328: f64 = (var_gco_i * var_phit);
        var_dch = assign7950_e7328;
        var_dch_dn4 = (var_gco_i * var_phit_dn4);
        var_dch_dn6 = (var_gco_i * var_phit_dn6);
        var_dch_dn7 = (var_gco_i * var_phit_dn7);
        var_dch_dn8 = (var_gco_i * var_phit_dn8);
        var_dch_dn9 = (var_gco_i * var_phit_dn9);
        var_dch_rv = 0.0;

        let assign7960_e7331: f64 = (var_gco_i * var_phit0);
        var_dov = assign7960_e7331;
        var_dov_dn4 = (var_gco_i * var_phit0_dn4);
        var_dov_dn6 = (var_gco_i * var_phit0_dn6);
        var_dov_dn7 = (var_gco_i * var_phit0_dn7);
        var_dov_dn8 = (var_gco_i * var_phit0_dn8);
        var_dov_dn9 = (var_gco_i * var_phit0_dn9);
        var_dov_rv = 0.0;

        let assign7970_e7336: f64 = (var_niginv_i * var_eg_2phit);
        let assign7970_e7337: f64 = (1.0 + assign7970_e7336);
        let assign7970_e7338: f64 = (1.0 / assign7970_e7337);
        var_n_iginv = assign7970_e7338;
        var_n_iginv_dn4 = (-((var_niginv_i * var_eg_2phit_dn4) / (assign7970_e7337 * assign7970_e7337)));
        var_n_iginv_dn6 = (-((var_niginv_i * var_eg_2phit_dn6) / (assign7970_e7337 * assign7970_e7337)));
        var_n_iginv_dn7 = (-((var_niginv_i * var_eg_2phit_dn7) / (assign7970_e7337 * assign7970_e7337)));
        var_n_iginv_dn8 = (-((var_niginv_i * var_eg_2phit_dn8) / (assign7970_e7337 * assign7970_e7337)));
        var_n_iginv_dn9 = (-((var_niginv_i * var_eg_2phit_dn9) / (assign7970_e7337 * assign7970_e7337)));
        var_n_iginv_rv = 0.0;

        let assign7980_e7342: f64 = (var_toxp_i * var_toxp_i);
        let assign7980_e7343: f64 = (4e-18 / assign7980_e7342);
        var_temp = assign7980_e7343;
        var_temp_dn4 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;
        var_temp_rv = 0.0;

        let assign7990_e7346: f64 = (var_agidl_i * var_temp);
        var_agidl_i = assign7990_e7346;
        var_agidl_i_dn4 = ((var_agidl_i_dn4 * var_temp) + (var_agidl_i * var_temp_dn4));
        var_agidl_i_dn6 = ((var_agidl_i_dn6 * var_temp) + (var_agidl_i * var_temp_dn6));
        var_agidl_i_dn7 = ((var_agidl_i_dn7 * var_temp) + (var_agidl_i * var_temp_dn7));
        var_agidl_i_dn8 = ((var_agidl_i_dn8 * var_temp) + (var_agidl_i * var_temp_dn8));
        var_agidl_i_dn9 = ((var_agidl_i_dn9 * var_temp) + (var_agidl_i * var_temp_dn9));
        var_agidl_i_rv = 0.0;

        let assign8000_e7349: f64 = (var_agidld_i * var_temp);
        var_agidld_i = assign8000_e7349;
        var_agidld_i_dn4 = ((var_agidld_i_dn4 * var_temp) + (var_agidld_i * var_temp_dn4));
        var_agidld_i_dn6 = ((var_agidld_i_dn6 * var_temp) + (var_agidld_i * var_temp_dn6));
        var_agidld_i_dn7 = ((var_agidld_i_dn7 * var_temp) + (var_agidld_i * var_temp_dn7));
        var_agidld_i_dn8 = ((var_agidld_i_dn8 * var_temp) + (var_agidld_i * var_temp_dn8));
        var_agidld_i_dn9 = ((var_agidld_i_dn9 * var_temp) + (var_agidld_i * var_temp_dn9));
        var_agidld_i_rv = 0.0;

        let assign8010_e7352: f64 = (var_toxp_i * 500000000.0);
        var_temp = assign8010_e7352;
        var_temp_dn4 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;
        var_temp_rv = 0.0;

        let assign8020_e7357: f64 = (var_stbgidl_i * var_dt);
        let assign8020_e7358: f64 = (1.0 + assign8020_e7357);
        let assign8020_e7360: f64 = assign8020_e7358;
        let assign8020_e7364: f64 = (var_stbgidl_i * var_dt);
        let assign8020_e7365: f64 = (1.0 + assign8020_e7364);
        let assign8020_e7367: f64 = assign8020_e7365;
        let assign8020_e7371: f64 = (var_stbgidl_i * var_dt);
        let assign8020_e7372: f64 = (1.0 + assign8020_e7371);
        let assign8020_e7374: f64 = assign8020_e7372;
        let assign8020_e7375: f64 = (assign8020_e7367 * assign8020_e7374);
        let assign8020_e7377: f64 = (assign8020_e7375 + 0.01);
        let assign8020_e7378: f64 = (assign8020_e7377).sqrt();
        let assign8020_e7379: f64 = (assign8020_e7360 + assign8020_e7378);
        let assign8020_e7380: f64 = (0.5 * assign8020_e7379);
        var_tempm = assign8020_e7380;
        var_tempm_dn4 = (0.5 * ((var_stbgidl_i * var_dt_dn4) + ((((var_stbgidl_i * var_dt_dn4) * assign8020_e7374) + (assign8020_e7367 * (var_stbgidl_i * var_dt_dn4))) / (2.0 * assign8020_e7378))));
        var_tempm_dn6 = (0.5 * ((var_stbgidl_i * var_dt_dn6) + ((((var_stbgidl_i * var_dt_dn6) * assign8020_e7374) + (assign8020_e7367 * (var_stbgidl_i * var_dt_dn6))) / (2.0 * assign8020_e7378))));
        var_tempm_dn7 = (0.5 * ((var_stbgidl_i * var_dt_dn7) + ((((var_stbgidl_i * var_dt_dn7) * assign8020_e7374) + (assign8020_e7367 * (var_stbgidl_i * var_dt_dn7))) / (2.0 * assign8020_e7378))));
        var_tempm_dn8 = (0.5 * ((var_stbgidl_i * var_dt_dn8) + ((((var_stbgidl_i * var_dt_dn8) * assign8020_e7374) + (assign8020_e7367 * (var_stbgidl_i * var_dt_dn8))) / (2.0 * assign8020_e7378))));
        var_tempm_dn9 = (0.5 * ((var_stbgidl_i * var_dt_dn9) + ((((var_stbgidl_i * var_dt_dn9) * assign8020_e7374) + (assign8020_e7367 * (var_stbgidl_i * var_dt_dn9))) / (2.0 * assign8020_e7378))));
        var_tempm_rv = 0.0;

        let assign8030_e7383: f64 = (var_bgidl_t * var_tempm);
        let assign8030_e7385: f64 = (assign8030_e7383 * var_temp);
        var_bgidl_i = assign8030_e7385;
        var_bgidl_i_dn4 = (((var_bgidl_t * var_tempm_dn4) * var_temp) + (assign8030_e7383 * var_temp_dn4));
        var_bgidl_i_dn6 = (((var_bgidl_t * var_tempm_dn6) * var_temp) + (assign8030_e7383 * var_temp_dn6));
        var_bgidl_i_dn7 = (((var_bgidl_t * var_tempm_dn7) * var_temp) + (assign8030_e7383 * var_temp_dn7));
        var_bgidl_i_dn8 = (((var_bgidl_t * var_tempm_dn8) * var_temp) + (assign8030_e7383 * var_temp_dn8));
        var_bgidl_i_dn9 = (((var_bgidl_t * var_tempm_dn9) * var_temp) + (assign8030_e7383 * var_temp_dn9));
        var_bgidl_i_rv = 0.0;

        let assign8040_e7390: f64 = (var_stbgidld_i * var_dt);
        let assign8040_e7391: f64 = (1.0 + assign8040_e7390);
        let assign8040_e7393: f64 = assign8040_e7391;
        let assign8040_e7397: f64 = (var_stbgidld_i * var_dt);
        let assign8040_e7398: f64 = (1.0 + assign8040_e7397);
        let assign8040_e7400: f64 = assign8040_e7398;
        let assign8040_e7404: f64 = (var_stbgidld_i * var_dt);
        let assign8040_e7405: f64 = (1.0 + assign8040_e7404);
        let assign8040_e7407: f64 = assign8040_e7405;
        let assign8040_e7408: f64 = (assign8040_e7400 * assign8040_e7407);
        let assign8040_e7410: f64 = (assign8040_e7408 + 0.01);
        let assign8040_e7411: f64 = (assign8040_e7410).sqrt();
        let assign8040_e7412: f64 = (assign8040_e7393 + assign8040_e7411);
        let assign8040_e7413: f64 = (0.5 * assign8040_e7412);
        var_tempm = assign8040_e7413;
        var_tempm_dn4 = (0.5 * ((var_stbgidld_i * var_dt_dn4) + ((((var_stbgidld_i * var_dt_dn4) * assign8040_e7407) + (assign8040_e7400 * (var_stbgidld_i * var_dt_dn4))) / (2.0 * assign8040_e7411))));
        var_tempm_dn6 = (0.5 * ((var_stbgidld_i * var_dt_dn6) + ((((var_stbgidld_i * var_dt_dn6) * assign8040_e7407) + (assign8040_e7400 * (var_stbgidld_i * var_dt_dn6))) / (2.0 * assign8040_e7411))));
        var_tempm_dn7 = (0.5 * ((var_stbgidld_i * var_dt_dn7) + ((((var_stbgidld_i * var_dt_dn7) * assign8040_e7407) + (assign8040_e7400 * (var_stbgidld_i * var_dt_dn7))) / (2.0 * assign8040_e7411))));
        var_tempm_dn8 = (0.5 * ((var_stbgidld_i * var_dt_dn8) + ((((var_stbgidld_i * var_dt_dn8) * assign8040_e7407) + (assign8040_e7400 * (var_stbgidld_i * var_dt_dn8))) / (2.0 * assign8040_e7411))));
        var_tempm_dn9 = (0.5 * ((var_stbgidld_i * var_dt_dn9) + ((((var_stbgidld_i * var_dt_dn9) * assign8040_e7407) + (assign8040_e7400 * (var_stbgidld_i * var_dt_dn9))) / (2.0 * assign8040_e7411))));
        var_tempm_rv = 0.0;

        let assign8050_e7416: f64 = (var_bgidld_t * var_tempm);
        let assign8050_e7418: f64 = (assign8050_e7416 * var_temp);
        var_bgidld_i = assign8050_e7418;
        var_bgidld_i_dn4 = (((var_bgidld_t * var_tempm_dn4) * var_temp) + (assign8050_e7416 * var_temp_dn4));
        var_bgidld_i_dn6 = (((var_bgidld_t * var_tempm_dn6) * var_temp) + (assign8050_e7416 * var_temp_dn6));
        var_bgidld_i_dn7 = (((var_bgidld_t * var_tempm_dn7) * var_temp) + (assign8050_e7416 * var_temp_dn7));
        var_bgidld_i_dn8 = (((var_bgidld_t * var_tempm_dn8) * var_temp) + (assign8050_e7416 * var_temp_dn8));
        var_bgidld_i_dn9 = (((var_bgidld_t * var_tempm_dn9) * var_temp) + (assign8050_e7416 * var_temp_dn9));
        var_bgidld_i_rv = 0.0;

        let assign8060_e7421: f64 = (-var_sta2_i);
        let assign8060_e7423: f64 = (assign8060_e7421 * var_lnrtn);
        let assign8060_e7424: f64 = (assign8060_e7423).exp();
        let assign8060_e7425: f64 = (var_a2_t * assign8060_e7424);
        var_a2_i = assign8060_e7425;
        var_a2_i_dn4 = (var_a2_t * (assign8060_e7424 * (assign8060_e7421 * var_lnrtn_dn4)));
        var_a2_i_dn6 = (var_a2_t * (assign8060_e7424 * (assign8060_e7421 * var_lnrtn_dn6)));
        var_a2_i_dn7 = (var_a2_t * (assign8060_e7424 * (assign8060_e7421 * var_lnrtn_dn7)));
        var_a2_i_dn8 = (var_a2_t * (assign8060_e7424 * (assign8060_e7421 * var_lnrtn_dn8)));
        var_a2_i_dn9 = (var_a2_t * (assign8060_e7424 * (assign8060_e7421 * var_lnrtn_dn9)));
        var_a2_i_rv = 0.0;

        let assign8070_e7430: f64 = (var_ctedge_i * var_rtn);
        let assign8070_e7431: f64 = (1.0 + assign8070_e7430);
        let assign8070_e7432: f64 = (var_phit0 * assign8070_e7431);
        var_phit_edge = assign8070_e7432;
        var_phit_edge_dn4 = ((var_phit0_dn4 * assign8070_e7431) + (var_phit0 * (var_ctedge_i * var_rtn_dn4)));
        var_phit_edge_dn6 = ((var_phit0_dn6 * assign8070_e7431) + (var_phit0 * (var_ctedge_i * var_rtn_dn6)));
        var_phit_edge_dn7 = ((var_phit0_dn7 * assign8070_e7431) + (var_phit0 * (var_ctedge_i * var_rtn_dn7)));
        var_phit_edge_dn8 = ((var_phit0_dn8 * assign8070_e7431) + (var_phit0 * (var_ctedge_i * var_rtn_dn8)));
        var_phit_edge_dn9 = ((var_phit0_dn9 * assign8070_e7431) + (var_phit0 * (var_ctedge_i * var_rtn_dn9)));
        var_phit_edge_rv = 0.0;

        let assign8080_e7435: f64 = (1.0 / var_phit_edge);
        var_inv_phit_edge = assign8080_e7435;
        var_inv_phit_edge_dn4 = (-(var_phit_edge_dn4 / (var_phit_edge * var_phit_edge)));
        var_inv_phit_edge_dn6 = (-(var_phit_edge_dn6 / (var_phit_edge * var_phit_edge)));
        var_inv_phit_edge_dn7 = (-(var_phit_edge_dn7 / (var_phit_edge * var_phit_edge)));
        var_inv_phit_edge_dn8 = (-(var_phit_edge_dn8 / (var_phit_edge * var_phit_edge)));
        var_inv_phit_edge_dn9 = (-(var_phit_edge_dn9 / (var_phit_edge * var_phit_edge)));
        var_inv_phit_edge_rv = 0.0;

        let assign8090_e7438: f64 = (2.0 * 1.602176565e-19);
        let assign8090_e7440: f64 = (assign8090_e7438 * var_neff);
        let assign8090_e7442: f64 = (assign8090_e7440 * var_epsch);
        let assign8090_e7444: f64 = (assign8090_e7442 * var_inv_phit_edge);
        var_a0_csisq_edge = assign8090_e7444;
        var_a0_csisq_edge_dn4 = ((((assign8090_e7438 * var_neff_dn4) * var_epsch) * var_inv_phit_edge) + (assign8090_e7442 * var_inv_phit_edge_dn4));
        var_a0_csisq_edge_dn6 = ((((assign8090_e7438 * var_neff_dn6) * var_epsch) * var_inv_phit_edge) + (assign8090_e7442 * var_inv_phit_edge_dn6));
        var_a0_csisq_edge_dn7 = ((((assign8090_e7438 * var_neff_dn7) * var_epsch) * var_inv_phit_edge) + (assign8090_e7442 * var_inv_phit_edge_dn7));
        var_a0_csisq_edge_dn8 = ((((assign8090_e7438 * var_neff_dn8) * var_epsch) * var_inv_phit_edge) + (assign8090_e7442 * var_inv_phit_edge_dn8));
        var_a0_csisq_edge_dn9 = ((((assign8090_e7438 * var_neff_dn9) * var_epsch) * var_inv_phit_edge) + (assign8090_e7442 * var_inv_phit_edge_dn9));
        var_a0_csisq_edge_rv = 0.0;

        let assign8100_e7447: f64 = (p.p14 * var_stvfbedge_i);
        let assign8100_e7449: f64 = (assign8100_e7447 * var_dt);
        let assign8100_e7451: f64 = (assign8100_e7449 + var_dvfbqm);
        var_temp = assign8100_e7451;
        var_temp_dn4 = (assign8100_e7447 * var_dt_dn4);
        var_temp_dn6 = (assign8100_e7447 * var_dt_dn6);
        var_temp_dn7 = (assign8100_e7447 * var_dt_dn7);
        var_temp_dn8 = (assign8100_e7447 * var_dt_dn8);
        var_temp_dn9 = (assign8100_e7447 * var_dt_dn9);
        var_temp_rv = 0.0;

        let assign8110_e7455: f64 = (var_vfb1edge_t + var_dvfbch);
        let assign8110_e7457: f64 = (assign8110_e7455 + var_dvfb1nch);
        let assign8110_e7458: f64 = (p.p14 * assign8110_e7457);
        let assign8110_e7460: f64 = (assign8110_e7458 + var_temp);
        let assign8110_e7462: f64 = (assign8110_e7460 + p.p34);
        let assign8110_e7464: f64 = (assign8110_e7462 - var_dvfbpdep);
        var_vfb1edge_i = assign8110_e7464;
        var_vfb1edge_i_dn4 = (((p.p14 * ((var_vfb1edge_t_dn4 + var_dvfbch_dn4) + var_dvfb1nch_dn4)) + var_temp_dn4) - var_dvfbpdep_dn4);
        var_vfb1edge_i_dn6 = (((p.p14 * ((var_vfb1edge_t_dn6 + var_dvfbch_dn6) + var_dvfb1nch_dn6)) + var_temp_dn6) - var_dvfbpdep_dn6);
        var_vfb1edge_i_dn7 = (((p.p14 * ((var_vfb1edge_t_dn7 + var_dvfbch_dn7) + var_dvfb1nch_dn7)) + var_temp_dn7) - var_dvfbpdep_dn7);
        var_vfb1edge_i_dn8 = (((p.p14 * ((var_vfb1edge_t_dn8 + var_dvfbch_dn8) + var_dvfb1nch_dn8)) + var_temp_dn8) - var_dvfbpdep_dn8);
        var_vfb1edge_i_dn9 = (((p.p14 * ((var_vfb1edge_t_dn9 + var_dvfbch_dn9) + var_dvfb1nch_dn9)) + var_temp_dn9) - var_dvfbpdep_dn9);
        var_vfb1edge_i_rv = 0.0;

        let assign8120_e7468: f64 = (var_vfb2edge_t + var_dvfbch);
        let assign8120_e7470: f64 = (assign8120_e7468 + var_dvfb2nch);
        let assign8120_e7471: f64 = (p.p14 * assign8120_e7470);
        let assign8120_e7473: f64 = (assign8120_e7471 + var_temp);
        var_vfb2edge_i = assign8120_e7473;
        var_vfb2edge_i_dn4 = ((p.p14 * (var_dvfbch_dn4 + var_dvfb2nch_dn4)) + var_temp_dn4);
        var_vfb2edge_i_dn6 = ((p.p14 * (var_dvfbch_dn6 + var_dvfb2nch_dn6)) + var_temp_dn6);
        var_vfb2edge_i_dn7 = ((p.p14 * (var_dvfbch_dn7 + var_dvfb2nch_dn7)) + var_temp_dn7);
        var_vfb2edge_i_dn8 = ((p.p14 * (var_dvfbch_dn8 + var_dvfb2nch_dn8)) + var_temp_dn8);
        var_vfb2edge_i_dn9 = ((p.p14 * (var_dvfbch_dn9 + var_dvfb2nch_dn9)) + var_temp_dn9);
        var_vfb2edge_i_rv = 0.0;

        let assign8130_e7476: f64 = (var_stbetedge_i * var_lnrtn);
        let assign8130_e7477: f64 = (assign8130_e7476).exp();
        let assign8130_e7479: f64 = (assign8130_e7477 * p.p35);
        var_temp = assign8130_e7479;
        var_temp_dn4 = ((assign8130_e7477 * (var_stbetedge_i * var_lnrtn_dn4)) * p.p35);
        var_temp_dn6 = ((assign8130_e7477 * (var_stbetedge_i * var_lnrtn_dn6)) * p.p35);
        var_temp_dn7 = ((assign8130_e7477 * (var_stbetedge_i * var_lnrtn_dn7)) * p.p35);
        var_temp_dn8 = ((assign8130_e7477 * (var_stbetedge_i * var_lnrtn_dn8)) * p.p35);
        var_temp_dn9 = ((assign8130_e7477 * (var_stbetedge_i * var_lnrtn_dn9)) * p.p35);
        var_temp_rv = 0.0;

        let assign8140_e7482: f64 = (var_betnedge_t * var_temp);
        var_betnedge_i = assign8140_e7482;
        var_betnedge_i_dn4 = ((var_betnedge_t_dn4 * var_temp) + (var_betnedge_t * var_temp_dn4));
        var_betnedge_i_dn6 = ((var_betnedge_t_dn6 * var_temp) + (var_betnedge_t * var_temp_dn6));
        var_betnedge_i_dn7 = ((var_betnedge_t_dn7 * var_temp) + (var_betnedge_t * var_temp_dn7));
        var_betnedge_i_dn8 = ((var_betnedge_t_dn8 * var_temp) + (var_betnedge_t * var_temp_dn8));
        var_betnedge_i_dn9 = ((var_betnedge_t_dn9 * var_temp) + (var_betnedge_t * var_temp_dn9));
        var_betnedge_i_rv = 0.0;

        let assign8150_e7485: f64 = (var_areaq_i * var_phit);
        var_area_phit = assign8150_e7485;
        var_area_phit_dn4 = (var_areaq_i * var_phit_dn4);
        var_area_phit_dn6 = (var_areaq_i * var_phit_dn6);
        var_area_phit_dn7 = (var_areaq_i * var_phit_dn7);
        var_area_phit_dn8 = (var_areaq_i * var_phit_dn8);
        var_area_phit_dn9 = (var_areaq_i * var_phit_dn9);
        var_area_phit_rv = 0.0;

        *var_a0_csisq_edge_slot = var_a0_csisq_edge;
        *var_a0_csisq_edge_dn4_slot = var_a0_csisq_edge_dn4;
        *var_a0_csisq_edge_dn6_slot = var_a0_csisq_edge_dn6;
        *var_a0_csisq_edge_dn7_slot = var_a0_csisq_edge_dn7;
        *var_a0_csisq_edge_dn8_slot = var_a0_csisq_edge_dn8;
        *var_a0_csisq_edge_dn9_slot = var_a0_csisq_edge_dn9;
        *var_a0_csisq_edge_rv_slot = var_a0_csisq_edge_rv;
        *var_a2_i_slot = var_a2_i;
        *var_a2_i_dn4_slot = var_a2_i_dn4;
        *var_a2_i_dn6_slot = var_a2_i_dn6;
        *var_a2_i_dn7_slot = var_a2_i_dn7;
        *var_a2_i_dn8_slot = var_a2_i_dn8;
        *var_a2_i_dn9_slot = var_a2_i_dn9;
        *var_a2_i_rv_slot = var_a2_i_rv;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_alp1_phit_slot = var_alp1_phit;
        *var_alp1_phit_dn4_slot = var_alp1_phit_dn4;
        *var_alp1_phit_dn6_slot = var_alp1_phit_dn6;
        *var_alp1_phit_dn7_slot = var_alp1_phit_dn7;
        *var_alp1_phit_dn8_slot = var_alp1_phit_dn8;
        *var_alp1_phit_dn9_slot = var_alp1_phit_dn9;
        *var_alp1_phit_rv_slot = var_alp1_phit_rv;
        *var_alpha_b_slot = var_alpha_b;
        *var_alpha_b_dn4_slot = var_alpha_b_dn4;
        *var_alpha_b_dn6_slot = var_alpha_b_dn6;
        *var_alpha_b_dn7_slot = var_alpha_b_dn7;
        *var_alpha_b_dn8_slot = var_alpha_b_dn8;
        *var_alpha_b_dn9_slot = var_alpha_b_dn9;
        *var_alpha_b_rv_slot = var_alpha_b_rv;
        *var_area_phit_slot = var_area_phit;
        *var_area_phit_dn4_slot = var_area_phit_dn4;
        *var_area_phit_dn6_slot = var_area_phit_dn6;
        *var_area_phit_dn7_slot = var_area_phit_dn7;
        *var_area_phit_dn8_slot = var_area_phit_dn8;
        *var_area_phit_dn9_slot = var_area_phit_dn9;
        *var_area_phit_rv_slot = var_area_phit_rv;
        *var_bch_slot = var_bch;
        *var_bch_dn4_slot = var_bch_dn4;
        *var_bch_dn6_slot = var_bch_dn6;
        *var_bch_dn7_slot = var_bch_dn7;
        *var_bch_dn8_slot = var_bch_dn8;
        *var_bch_dn9_slot = var_bch_dn9;
        *var_bch_rv_slot = var_bch_rv;
        *var_betnedge_i_slot = var_betnedge_i;
        *var_betnedge_i_dn4_slot = var_betnedge_i_dn4;
        *var_betnedge_i_dn6_slot = var_betnedge_i_dn6;
        *var_betnedge_i_dn7_slot = var_betnedge_i_dn7;
        *var_betnedge_i_dn8_slot = var_betnedge_i_dn8;
        *var_betnedge_i_dn9_slot = var_betnedge_i_dn9;
        *var_betnedge_i_rv_slot = var_betnedge_i_rv;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidl_i_dn4_slot = var_bgidl_i_dn4;
        *var_bgidl_i_dn6_slot = var_bgidl_i_dn6;
        *var_bgidl_i_dn7_slot = var_bgidl_i_dn7;
        *var_bgidl_i_dn8_slot = var_bgidl_i_dn8;
        *var_bgidl_i_dn9_slot = var_bgidl_i_dn9;
        *var_bgidl_i_rv_slot = var_bgidl_i_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_dn4_slot = var_bgidld_i_dn4;
        *var_bgidld_i_dn6_slot = var_bgidld_i_dn6;
        *var_bgidld_i_dn7_slot = var_bgidld_i_dn7;
        *var_bgidld_i_dn8_slot = var_bgidld_i_dn8;
        *var_bgidld_i_dn9_slot = var_bgidld_i_dn9;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_bov_slot = var_bov;
        *var_bov_dn4_slot = var_bov_dn4;
        *var_bov_dn6_slot = var_bov_dn6;
        *var_bov_dn7_slot = var_bov_dn7;
        *var_bov_dn8_slot = var_bov_dn8;
        *var_bov_dn9_slot = var_bov_dn9;
        *var_bov_rv_slot = var_bov_rv;
        *var_dch_slot = var_dch;
        *var_dch_dn4_slot = var_dch_dn4;
        *var_dch_dn6_slot = var_dch_dn6;
        *var_dch_dn7_slot = var_dch_dn7;
        *var_dch_dn8_slot = var_dch_dn8;
        *var_dch_dn9_slot = var_dch_dn9;
        *var_dch_rv_slot = var_dch_rv;
        *var_dov_slot = var_dov;
        *var_dov_dn4_slot = var_dov_dn4;
        *var_dov_dn6_slot = var_dov_dn6;
        *var_dov_dn7_slot = var_dov_dn7;
        *var_dov_dn8_slot = var_dov_dn8;
        *var_dov_dn9_slot = var_dov_dn9;
        *var_dov_rv_slot = var_dov_rv;
        *var_frs_slot = var_frs;
        *var_frs_dn4_slot = var_frs_dn4;
        *var_frs_dn6_slot = var_frs_dn6;
        *var_frs_dn7_slot = var_frs_dn7;
        *var_frs_dn8_slot = var_frs_dn8;
        *var_frs_dn9_slot = var_frs_dn9;
        *var_frs_rv_slot = var_frs_rv;
        *var_gamax_slot = var_gamax;
        *var_gamax_ac_slot = var_gamax_ac;
        *var_gamax_ac_rv_slot = var_gamax_ac_rv;
        *var_gamax_rv_slot = var_gamax_rv;
        *var_gcqch_slot = var_gcqch;
        *var_gcqch_rv_slot = var_gcqch_rv;
        *var_gcqovacc_slot = var_gcqovacc;
        *var_gcqovacc_rv_slot = var_gcqovacc_rv;
        *var_gcqovinv_slot = var_gcqovinv;
        *var_gcqovinv_rv_slot = var_gcqovinv_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_dn4_slot = var_iginv_i_dn4;
        *var_iginv_i_dn6_slot = var_iginv_i_dn6;
        *var_iginv_i_dn7_slot = var_iginv_i_dn7;
        *var_iginv_i_dn8_slot = var_iginv_i_dn8;
        *var_iginv_i_dn9_slot = var_iginv_i_dn9;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igovacc_i_slot = var_igovacc_i;
        *var_igovacc_i_dn4_slot = var_igovacc_i_dn4;
        *var_igovacc_i_dn6_slot = var_igovacc_i_dn6;
        *var_igovacc_i_dn7_slot = var_igovacc_i_dn7;
        *var_igovacc_i_dn8_slot = var_igovacc_i_dn8;
        *var_igovacc_i_dn9_slot = var_igovacc_i_dn9;
        *var_igovacc_i_rv_slot = var_igovacc_i_rv;
        *var_igovaccd_i_slot = var_igovaccd_i;
        *var_igovaccd_i_dn4_slot = var_igovaccd_i_dn4;
        *var_igovaccd_i_dn6_slot = var_igovaccd_i_dn6;
        *var_igovaccd_i_dn7_slot = var_igovaccd_i_dn7;
        *var_igovaccd_i_dn8_slot = var_igovaccd_i_dn8;
        *var_igovaccd_i_dn9_slot = var_igovaccd_i_dn9;
        *var_igovaccd_i_rv_slot = var_igovaccd_i_rv;
        *var_igovinv_i_slot = var_igovinv_i;
        *var_igovinv_i_dn4_slot = var_igovinv_i_dn4;
        *var_igovinv_i_dn6_slot = var_igovinv_i_dn6;
        *var_igovinv_i_dn7_slot = var_igovinv_i_dn7;
        *var_igovinv_i_dn8_slot = var_igovinv_i_dn8;
        *var_igovinv_i_dn9_slot = var_igovinv_i_dn9;
        *var_igovinv_i_rv_slot = var_igovinv_i_rv;
        *var_igovinvd_i_slot = var_igovinvd_i;
        *var_igovinvd_i_dn4_slot = var_igovinvd_i_dn4;
        *var_igovinvd_i_dn6_slot = var_igovinvd_i_dn6;
        *var_igovinvd_i_dn7_slot = var_igovinvd_i_dn7;
        *var_igovinvd_i_dn8_slot = var_igovinvd_i_dn8;
        *var_igovinvd_i_dn9_slot = var_igovinvd_i_dn9;
        *var_igovinvd_i_rv_slot = var_igovinvd_i_rv;
        *var_inv_chib_slot = var_inv_chib;
        *var_inv_chib_rv_slot = var_inv_chib_rv;
        *var_inv_phit_edge_slot = var_inv_phit_edge;
        *var_inv_phit_edge_dn4_slot = var_inv_phit_edge_dn4;
        *var_inv_phit_edge_dn6_slot = var_inv_phit_edge_dn6;
        *var_inv_phit_edge_dn7_slot = var_inv_phit_edge_dn7;
        *var_inv_phit_edge_dn8_slot = var_inv_phit_edge_dn8;
        *var_inv_phit_edge_dn9_slot = var_inv_phit_edge_dn9;
        *var_inv_phit_edge_rv_slot = var_inv_phit_edge_rv;
        *var_n_iginv_slot = var_n_iginv;
        *var_n_iginv_dn4_slot = var_n_iginv_dn4;
        *var_n_iginv_dn6_slot = var_n_iginv_dn6;
        *var_n_iginv_dn7_slot = var_n_iginv_dn7;
        *var_n_iginv_dn8_slot = var_n_iginv_dn8;
        *var_n_iginv_dn9_slot = var_n_iginv_dn9;
        *var_n_iginv_rv_slot = var_n_iginv_rv;
        *var_phit_edge_slot = var_phit_edge;
        *var_phit_edge_dn4_slot = var_phit_edge_dn4;
        *var_phit_edge_dn6_slot = var_phit_edge_dn6;
        *var_phit_edge_dn7_slot = var_phit_edge_dn7;
        *var_phit_edge_dn8_slot = var_phit_edge_dn8;
        *var_phit_edge_dn9_slot = var_phit_edge_dn9;
        *var_phit_edge_rv_slot = var_phit_edge_rv;
        *var_rs_i_slot = var_rs_i;
        *var_rs_i_dn4_slot = var_rs_i_dn4;
        *var_rs_i_dn6_slot = var_rs_i_dn6;
        *var_rs_i_dn7_slot = var_rs_i_dn7;
        *var_rs_i_dn8_slot = var_rs_i_dn8;
        *var_rs_i_dn9_slot = var_rs_i_dn9;
        *var_rs_i_rv_slot = var_rs_i_rv;
        *var_sat_phit_slot = var_sat_phit;
        *var_sat_phit_ac_slot = var_sat_phit_ac;
        *var_sat_phit_ac_dn4_slot = var_sat_phit_ac_dn4;
        *var_sat_phit_ac_dn6_slot = var_sat_phit_ac_dn6;
        *var_sat_phit_ac_dn7_slot = var_sat_phit_ac_dn7;
        *var_sat_phit_ac_dn8_slot = var_sat_phit_ac_dn8;
        *var_sat_phit_ac_dn9_slot = var_sat_phit_ac_dn9;
        *var_sat_phit_ac_rv_slot = var_sat_phit_ac_rv;
        *var_sat_phit_dn4_slot = var_sat_phit_dn4;
        *var_sat_phit_dn6_slot = var_sat_phit_dn6;
        *var_sat_phit_dn7_slot = var_sat_phit_dn7;
        *var_sat_phit_dn8_slot = var_sat_phit_dn8;
        *var_sat_phit_dn9_slot = var_sat_phit_dn9;
        *var_sat_phit_rv_slot = var_sat_phit_rv;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_rv_slot = var_temp_rv;
        *var_tempm_slot = var_tempm;
        *var_tempm_dn4_slot = var_tempm_dn4;
        *var_tempm_dn6_slot = var_tempm_dn6;
        *var_tempm_dn7_slot = var_tempm_dn7;
        *var_tempm_dn8_slot = var_tempm_dn8;
        *var_tempm_dn9_slot = var_tempm_dn9;
        *var_tempm_rv_slot = var_tempm_rv;
        *var_tf_ig_slot = var_tf_ig;
        *var_tf_ig_dn4_slot = var_tf_ig_dn4;
        *var_tf_ig_dn6_slot = var_tf_ig_dn6;
        *var_tf_ig_dn7_slot = var_tf_ig_dn7;
        *var_tf_ig_dn8_slot = var_tf_ig_dn8;
        *var_tf_ig_dn9_slot = var_tf_ig_dn9;
        *var_tf_ig_rv_slot = var_tf_ig_rv;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_thesat_dn4_slot = var_tf_thesat_dn4;
        *var_tf_thesat_dn6_slot = var_tf_thesat_dn6;
        *var_tf_thesat_dn7_slot = var_tf_thesat_dn7;
        *var_tf_thesat_dn8_slot = var_tf_thesat_dn8;
        *var_tf_thesat_dn9_slot = var_tf_thesat_dn9;
        *var_tf_thesat_rv_slot = var_tf_thesat_rv;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesat_i_dn4_slot = var_thesat_i_dn4;
        *var_thesat_i_dn6_slot = var_thesat_i_dn6;
        *var_thesat_i_dn7_slot = var_thesat_i_dn7;
        *var_thesat_i_dn8_slot = var_thesat_i_dn8;
        *var_thesat_i_dn9_slot = var_thesat_i_dn9;
        *var_thesat_i_rv_slot = var_thesat_i_rv;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_thesatac_i_dn4_slot = var_thesatac_i_dn4;
        *var_thesatac_i_dn6_slot = var_thesatac_i_dn6;
        *var_thesatac_i_dn7_slot = var_thesatac_i_dn7;
        *var_thesatac_i_dn8_slot = var_thesatac_i_dn8;
        *var_thesatac_i_dn9_slot = var_thesatac_i_dn9;
        *var_thesatac_i_rv_slot = var_thesatac_i_rv;
        *var_vfb1edge_i_slot = var_vfb1edge_i;
        *var_vfb1edge_i_dn4_slot = var_vfb1edge_i_dn4;
        *var_vfb1edge_i_dn6_slot = var_vfb1edge_i_dn6;
        *var_vfb1edge_i_dn7_slot = var_vfb1edge_i_dn7;
        *var_vfb1edge_i_dn8_slot = var_vfb1edge_i_dn8;
        *var_vfb1edge_i_dn9_slot = var_vfb1edge_i_dn9;
        *var_vfb1edge_i_rv_slot = var_vfb1edge_i_rv;
        *var_vfb2edge_i_slot = var_vfb2edge_i;
        *var_vfb2edge_i_dn4_slot = var_vfb2edge_i_dn4;
        *var_vfb2edge_i_dn6_slot = var_vfb2edge_i_dn6;
        *var_vfb2edge_i_dn7_slot = var_vfb2edge_i_dn7;
        *var_vfb2edge_i_dn8_slot = var_vfb2edge_i_dn8;
        *var_vfb2edge_i_dn9_slot = var_vfb2edge_i_dn9;
        *var_vfb2edge_i_rv_slot = var_vfb2edge_i_rv;
    }
}
