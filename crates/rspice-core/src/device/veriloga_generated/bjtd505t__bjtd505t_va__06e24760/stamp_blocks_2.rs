#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_bavl_t: f64,
        var_bavl_t_dn0: f64,
        var_bavl_t_dn1: f64,
        var_bavl_t_dn10: f64,
        var_bavl_t_dn3: f64,
        var_bavl_t_dn4: f64,
        var_bavl_t_dn5: f64,
        var_bavl_t_dn6: f64,
        var_bavl_t_dn7: f64,
        var_bavl_t_dn8: f64,
        var_bavl_t_dn9: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard89: f64,
        var_icap: f64,
        var_icap_dn0: f64,
        var_icap_dn1: f64,
        var_icap_dn10: f64,
        var_icap_dn3: f64,
        var_icap_dn4: f64,
        var_icap_dn5: f64,
        var_icap_dn6: f64,
        var_icap_dn7: f64,
        var_icap_dn8: f64,
        var_icap_dn9: f64,
        var_icap_ihc: f64,
        var_icap_ihc_dn0: f64,
        var_icap_ihc_dn1: f64,
        var_icap_ihc_dn10: f64,
        var_icap_ihc_dn3: f64,
        var_icap_ihc_dn4: f64,
        var_icap_ihc_dn5: f64,
        var_icap_ihc_dn6: f64,
        var_icap_ihc_dn7: f64,
        var_icap_ihc_dn8: f64,
        var_icap_ihc_dn9: f64,
        var_in_: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_vb2c1: f64,
        var_vb2c1_dn6: f64,
        var_vb2c1_dn7: f64,
        var_vdc_t: f64,
        var_vdc_t_dn0: f64,
        var_vdc_t_dn1: f64,
        var_vdc_t_dn10: f64,
        var_vdc_t_dn3: f64,
        var_vdc_t_dn4: f64,
        var_vdc_t_dn5: f64,
        var_vdc_t_dn6: f64,
        var_vdc_t_dn7: f64,
        var_vdc_t_dn8: f64,
        var_vdc_t_dn9: f64,
        var_vl: f64,
        var_vl_dn0: f64,
        var_vl_dn1: f64,
        var_vl_dn10: f64,
        var_vl_dn3: f64,
        var_vl_dn4: f64,
        var_vl_dn5: f64,
        var_vl_dn6: f64,
        var_vl_dn7: f64,
        var_vl_dn8: f64,
        var_vl_dn9: f64,
        var_xi_w: f64,
        var_xi_w_dn0: f64,
        var_xi_w_dn1: f64,
        var_xi_w_dn10: f64,
        var_xi_w_dn3: f64,
        var_xi_w_dn4: f64,
        var_xi_w_dn5: f64,
        var_xi_w_dn6: f64,
        var_xi_w_dn7: f64,
        var_xi_w_dn8: f64,
        var_xi_w_dn9: f64,
        var_dedx0_slot: &mut f64,
        var_dedx0_rv_slot: &mut f64,
        var_e0_slot: &mut f64,
        var_e0_dn0_slot: &mut f64,
        var_e0_dn1_slot: &mut f64,
        var_e0_dn10_slot: &mut f64,
        var_e0_dn3_slot: &mut f64,
        var_e0_dn4_slot: &mut f64,
        var_e0_dn5_slot: &mut f64,
        var_e0_dn6_slot: &mut f64,
        var_e0_dn7_slot: &mut f64,
        var_e0_dn8_slot: &mut f64,
        var_e0_dn9_slot: &mut f64,
        var_e0_rv_slot: &mut f64,
        var_eav_slot: &mut f64,
        var_eav_dn0_slot: &mut f64,
        var_eav_dn1_slot: &mut f64,
        var_eav_dn10_slot: &mut f64,
        var_eav_dn3_slot: &mut f64,
        var_eav_dn4_slot: &mut f64,
        var_eav_dn5_slot: &mut f64,
        var_eav_dn6_slot: &mut f64,
        var_eav_dn7_slot: &mut f64,
        var_eav_dn8_slot: &mut f64,
        var_eav_dn9_slot: &mut f64,
        var_eav_rv_slot: &mut f64,
        var_efi_slot: &mut f64,
        var_efi_rv_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn0_slot: &mut f64,
        var_em_dn1_slot: &mut f64,
        var_em_dn10_slot: &mut f64,
        var_em_dn3_slot: &mut f64,
        var_em_dn4_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_dn9_slot: &mut f64,
        var_em_rv_slot: &mut f64,
        var_emeav_em_slot: &mut f64,
        var_emeav_em_dn0_slot: &mut f64,
        var_emeav_em_dn1_slot: &mut f64,
        var_emeav_em_dn10_slot: &mut f64,
        var_emeav_em_dn3_slot: &mut f64,
        var_emeav_em_dn4_slot: &mut f64,
        var_emeav_em_dn5_slot: &mut f64,
        var_emeav_em_dn6_slot: &mut f64,
        var_emeav_em_dn7_slot: &mut f64,
        var_emeav_em_dn8_slot: &mut f64,
        var_emeav_em_dn9_slot: &mut f64,
        var_emeav_em_rv_slot: &mut f64,
        var_ew_slot: &mut f64,
        var_ew_dn0_slot: &mut f64,
        var_ew_dn1_slot: &mut f64,
        var_ew_dn10_slot: &mut f64,
        var_ew_dn3_slot: &mut f64,
        var_ew_dn4_slot: &mut f64,
        var_ew_dn5_slot: &mut f64,
        var_ew_dn6_slot: &mut f64,
        var_ew_dn7_slot: &mut f64,
        var_ew_dn8_slot: &mut f64,
        var_ew_dn9_slot: &mut f64,
        var_ew_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_expmm1_slot: &mut f64,
        var_expmm1_dn0_slot: &mut f64,
        var_expmm1_dn1_slot: &mut f64,
        var_expmm1_dn10_slot: &mut f64,
        var_expmm1_dn3_slot: &mut f64,
        var_expmm1_dn4_slot: &mut f64,
        var_expmm1_dn5_slot: &mut f64,
        var_expmm1_dn6_slot: &mut f64,
        var_expmm1_dn7_slot: &mut f64,
        var_expmm1_dn8_slot: &mut f64,
        var_expmm1_dn9_slot: &mut f64,
        var_expmm1_rv_slot: &mut f64,
        var_gem_slot: &mut f64,
        var_gem_dn0_slot: &mut f64,
        var_gem_dn1_slot: &mut f64,
        var_gem_dn10_slot: &mut f64,
        var_gem_dn3_slot: &mut f64,
        var_gem_dn4_slot: &mut f64,
        var_gem_dn5_slot: &mut f64,
        var_gem_dn6_slot: &mut f64,
        var_gem_dn7_slot: &mut f64,
        var_gem_dn8_slot: &mut f64,
        var_gem_dn9_slot: &mut f64,
        var_gem_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_lambda_slot: &mut f64,
        var_lambda_dn0_slot: &mut f64,
        var_lambda_dn1_slot: &mut f64,
        var_lambda_dn10_slot: &mut f64,
        var_lambda_dn3_slot: &mut f64,
        var_lambda_dn4_slot: &mut f64,
        var_lambda_dn5_slot: &mut f64,
        var_lambda_dn6_slot: &mut f64,
        var_lambda_dn7_slot: &mut f64,
        var_lambda_dn8_slot: &mut f64,
        var_lambda_dn9_slot: &mut f64,
        var_lambda_rv_slot: &mut f64,
        var_shw_slot: &mut f64,
        var_shw_dn0_slot: &mut f64,
        var_shw_dn1_slot: &mut f64,
        var_shw_dn10_slot: &mut f64,
        var_shw_dn3_slot: &mut f64,
        var_shw_dn4_slot: &mut f64,
        var_shw_dn5_slot: &mut f64,
        var_shw_dn6_slot: &mut f64,
        var_shw_dn7_slot: &mut f64,
        var_shw_dn8_slot: &mut f64,
        var_shw_dn9_slot: &mut f64,
        var_shw_rv_slot: &mut f64,
        var_sqr_arg_slot: &mut f64,
        var_sqr_arg_dn0_slot: &mut f64,
        var_sqr_arg_dn1_slot: &mut f64,
        var_sqr_arg_dn10_slot: &mut f64,
        var_sqr_arg_dn3_slot: &mut f64,
        var_sqr_arg_dn4_slot: &mut f64,
        var_sqr_arg_dn5_slot: &mut f64,
        var_sqr_arg_dn6_slot: &mut f64,
        var_sqr_arg_dn7_slot: &mut f64,
        var_sqr_arg_dn8_slot: &mut f64,
        var_sqr_arg_dn9_slot: &mut f64,
        var_sqr_arg_rv_slot: &mut f64,
        var_wd_slot: &mut f64,
        var_wd_dn0_slot: &mut f64,
        var_wd_dn1_slot: &mut f64,
        var_wd_dn10_slot: &mut f64,
        var_wd_dn3_slot: &mut f64,
        var_wd_dn4_slot: &mut f64,
        var_wd_dn5_slot: &mut f64,
        var_wd_dn6_slot: &mut f64,
        var_wd_dn7_slot: &mut f64,
        var_wd_dn8_slot: &mut f64,
        var_wd_dn9_slot: &mut f64,
        var_wd_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_dn0_slot: &mut f64,
        var_weff_dn1_slot: &mut f64,
        var_weff_dn10_slot: &mut f64,
        var_weff_dn3_slot: &mut f64,
        var_weff_dn4_slot: &mut f64,
        var_weff_dn5_slot: &mut f64,
        var_weff_dn6_slot: &mut f64,
        var_weff_dn7_slot: &mut f64,
        var_weff_dn8_slot: &mut f64,
        var_weff_dn9_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
        var_xd_slot: &mut f64,
        var_xd_dn0_slot: &mut f64,
        var_xd_dn1_slot: &mut f64,
        var_xd_dn10_slot: &mut f64,
        var_xd_dn3_slot: &mut f64,
        var_xd_dn4_slot: &mut f64,
        var_xd_dn5_slot: &mut f64,
        var_xd_dn6_slot: &mut f64,
        var_xd_dn7_slot: &mut f64,
        var_xd_dn8_slot: &mut f64,
        var_xd_dn9_slot: &mut f64,
        var_xd_rv_slot: &mut f64,
        var_xi_w1_slot: &mut f64,
        var_xi_w1_dn0_slot: &mut f64,
        var_xi_w1_dn1_slot: &mut f64,
        var_xi_w1_dn10_slot: &mut f64,
        var_xi_w1_dn3_slot: &mut f64,
        var_xi_w1_dn4_slot: &mut f64,
        var_xi_w1_dn5_slot: &mut f64,
        var_xi_w1_dn6_slot: &mut f64,
        var_xi_w1_dn7_slot: &mut f64,
        var_xi_w1_dn8_slot: &mut f64,
        var_xi_w1_dn9_slot: &mut f64,
        var_xi_w1_rv_slot: &mut f64,
    ) {
        let mut var_dedx0: f64 = *var_dedx0_slot;
        let mut var_dedx0_rv: f64 = *var_dedx0_rv_slot;
        let mut var_e0: f64 = *var_e0_slot;
        let mut var_e0_dn0: f64 = *var_e0_dn0_slot;
        let mut var_e0_dn1: f64 = *var_e0_dn1_slot;
        let mut var_e0_dn10: f64 = *var_e0_dn10_slot;
        let mut var_e0_dn3: f64 = *var_e0_dn3_slot;
        let mut var_e0_dn4: f64 = *var_e0_dn4_slot;
        let mut var_e0_dn5: f64 = *var_e0_dn5_slot;
        let mut var_e0_dn6: f64 = *var_e0_dn6_slot;
        let mut var_e0_dn7: f64 = *var_e0_dn7_slot;
        let mut var_e0_dn8: f64 = *var_e0_dn8_slot;
        let mut var_e0_dn9: f64 = *var_e0_dn9_slot;
        let mut var_e0_rv: f64 = *var_e0_rv_slot;
        let mut var_eav: f64 = *var_eav_slot;
        let mut var_eav_dn0: f64 = *var_eav_dn0_slot;
        let mut var_eav_dn1: f64 = *var_eav_dn1_slot;
        let mut var_eav_dn10: f64 = *var_eav_dn10_slot;
        let mut var_eav_dn3: f64 = *var_eav_dn3_slot;
        let mut var_eav_dn4: f64 = *var_eav_dn4_slot;
        let mut var_eav_dn5: f64 = *var_eav_dn5_slot;
        let mut var_eav_dn6: f64 = *var_eav_dn6_slot;
        let mut var_eav_dn7: f64 = *var_eav_dn7_slot;
        let mut var_eav_dn8: f64 = *var_eav_dn8_slot;
        let mut var_eav_dn9: f64 = *var_eav_dn9_slot;
        let mut var_eav_rv: f64 = *var_eav_rv_slot;
        let mut var_efi: f64 = *var_efi_slot;
        let mut var_efi_rv: f64 = *var_efi_rv_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn0: f64 = *var_em_dn0_slot;
        let mut var_em_dn1: f64 = *var_em_dn1_slot;
        let mut var_em_dn10: f64 = *var_em_dn10_slot;
        let mut var_em_dn3: f64 = *var_em_dn3_slot;
        let mut var_em_dn4: f64 = *var_em_dn4_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_dn9: f64 = *var_em_dn9_slot;
        let mut var_em_rv: f64 = *var_em_rv_slot;
        let mut var_emeav_em: f64 = *var_emeav_em_slot;
        let mut var_emeav_em_dn0: f64 = *var_emeav_em_dn0_slot;
        let mut var_emeav_em_dn1: f64 = *var_emeav_em_dn1_slot;
        let mut var_emeav_em_dn10: f64 = *var_emeav_em_dn10_slot;
        let mut var_emeav_em_dn3: f64 = *var_emeav_em_dn3_slot;
        let mut var_emeav_em_dn4: f64 = *var_emeav_em_dn4_slot;
        let mut var_emeav_em_dn5: f64 = *var_emeav_em_dn5_slot;
        let mut var_emeav_em_dn6: f64 = *var_emeav_em_dn6_slot;
        let mut var_emeav_em_dn7: f64 = *var_emeav_em_dn7_slot;
        let mut var_emeav_em_dn8: f64 = *var_emeav_em_dn8_slot;
        let mut var_emeav_em_dn9: f64 = *var_emeav_em_dn9_slot;
        let mut var_emeav_em_rv: f64 = *var_emeav_em_rv_slot;
        let mut var_ew: f64 = *var_ew_slot;
        let mut var_ew_dn0: f64 = *var_ew_dn0_slot;
        let mut var_ew_dn1: f64 = *var_ew_dn1_slot;
        let mut var_ew_dn10: f64 = *var_ew_dn10_slot;
        let mut var_ew_dn3: f64 = *var_ew_dn3_slot;
        let mut var_ew_dn4: f64 = *var_ew_dn4_slot;
        let mut var_ew_dn5: f64 = *var_ew_dn5_slot;
        let mut var_ew_dn6: f64 = *var_ew_dn6_slot;
        let mut var_ew_dn7: f64 = *var_ew_dn7_slot;
        let mut var_ew_dn8: f64 = *var_ew_dn8_slot;
        let mut var_ew_dn9: f64 = *var_ew_dn9_slot;
        let mut var_ew_rv: f64 = *var_ew_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_expmm1: f64 = *var_expmm1_slot;
        let mut var_expmm1_dn0: f64 = *var_expmm1_dn0_slot;
        let mut var_expmm1_dn1: f64 = *var_expmm1_dn1_slot;
        let mut var_expmm1_dn10: f64 = *var_expmm1_dn10_slot;
        let mut var_expmm1_dn3: f64 = *var_expmm1_dn3_slot;
        let mut var_expmm1_dn4: f64 = *var_expmm1_dn4_slot;
        let mut var_expmm1_dn5: f64 = *var_expmm1_dn5_slot;
        let mut var_expmm1_dn6: f64 = *var_expmm1_dn6_slot;
        let mut var_expmm1_dn7: f64 = *var_expmm1_dn7_slot;
        let mut var_expmm1_dn8: f64 = *var_expmm1_dn8_slot;
        let mut var_expmm1_dn9: f64 = *var_expmm1_dn9_slot;
        let mut var_expmm1_rv: f64 = *var_expmm1_rv_slot;
        let mut var_gem: f64 = *var_gem_slot;
        let mut var_gem_dn0: f64 = *var_gem_dn0_slot;
        let mut var_gem_dn1: f64 = *var_gem_dn1_slot;
        let mut var_gem_dn10: f64 = *var_gem_dn10_slot;
        let mut var_gem_dn3: f64 = *var_gem_dn3_slot;
        let mut var_gem_dn4: f64 = *var_gem_dn4_slot;
        let mut var_gem_dn5: f64 = *var_gem_dn5_slot;
        let mut var_gem_dn6: f64 = *var_gem_dn6_slot;
        let mut var_gem_dn7: f64 = *var_gem_dn7_slot;
        let mut var_gem_dn8: f64 = *var_gem_dn8_slot;
        let mut var_gem_dn9: f64 = *var_gem_dn9_slot;
        let mut var_gem_rv: f64 = *var_gem_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_lambda: f64 = *var_lambda_slot;
        let mut var_lambda_dn0: f64 = *var_lambda_dn0_slot;
        let mut var_lambda_dn1: f64 = *var_lambda_dn1_slot;
        let mut var_lambda_dn10: f64 = *var_lambda_dn10_slot;
        let mut var_lambda_dn3: f64 = *var_lambda_dn3_slot;
        let mut var_lambda_dn4: f64 = *var_lambda_dn4_slot;
        let mut var_lambda_dn5: f64 = *var_lambda_dn5_slot;
        let mut var_lambda_dn6: f64 = *var_lambda_dn6_slot;
        let mut var_lambda_dn7: f64 = *var_lambda_dn7_slot;
        let mut var_lambda_dn8: f64 = *var_lambda_dn8_slot;
        let mut var_lambda_dn9: f64 = *var_lambda_dn9_slot;
        let mut var_lambda_rv: f64 = *var_lambda_rv_slot;
        let mut var_shw: f64 = *var_shw_slot;
        let mut var_shw_dn0: f64 = *var_shw_dn0_slot;
        let mut var_shw_dn1: f64 = *var_shw_dn1_slot;
        let mut var_shw_dn10: f64 = *var_shw_dn10_slot;
        let mut var_shw_dn3: f64 = *var_shw_dn3_slot;
        let mut var_shw_dn4: f64 = *var_shw_dn4_slot;
        let mut var_shw_dn5: f64 = *var_shw_dn5_slot;
        let mut var_shw_dn6: f64 = *var_shw_dn6_slot;
        let mut var_shw_dn7: f64 = *var_shw_dn7_slot;
        let mut var_shw_dn8: f64 = *var_shw_dn8_slot;
        let mut var_shw_dn9: f64 = *var_shw_dn9_slot;
        let mut var_shw_rv: f64 = *var_shw_rv_slot;
        let mut var_sqr_arg: f64 = *var_sqr_arg_slot;
        let mut var_sqr_arg_dn0: f64 = *var_sqr_arg_dn0_slot;
        let mut var_sqr_arg_dn1: f64 = *var_sqr_arg_dn1_slot;
        let mut var_sqr_arg_dn10: f64 = *var_sqr_arg_dn10_slot;
        let mut var_sqr_arg_dn3: f64 = *var_sqr_arg_dn3_slot;
        let mut var_sqr_arg_dn4: f64 = *var_sqr_arg_dn4_slot;
        let mut var_sqr_arg_dn5: f64 = *var_sqr_arg_dn5_slot;
        let mut var_sqr_arg_dn6: f64 = *var_sqr_arg_dn6_slot;
        let mut var_sqr_arg_dn7: f64 = *var_sqr_arg_dn7_slot;
        let mut var_sqr_arg_dn8: f64 = *var_sqr_arg_dn8_slot;
        let mut var_sqr_arg_dn9: f64 = *var_sqr_arg_dn9_slot;
        let mut var_sqr_arg_rv: f64 = *var_sqr_arg_rv_slot;
        let mut var_wd: f64 = *var_wd_slot;
        let mut var_wd_dn0: f64 = *var_wd_dn0_slot;
        let mut var_wd_dn1: f64 = *var_wd_dn1_slot;
        let mut var_wd_dn10: f64 = *var_wd_dn10_slot;
        let mut var_wd_dn3: f64 = *var_wd_dn3_slot;
        let mut var_wd_dn4: f64 = *var_wd_dn4_slot;
        let mut var_wd_dn5: f64 = *var_wd_dn5_slot;
        let mut var_wd_dn6: f64 = *var_wd_dn6_slot;
        let mut var_wd_dn7: f64 = *var_wd_dn7_slot;
        let mut var_wd_dn8: f64 = *var_wd_dn8_slot;
        let mut var_wd_dn9: f64 = *var_wd_dn9_slot;
        let mut var_wd_rv: f64 = *var_wd_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_dn0: f64 = *var_weff_dn0_slot;
        let mut var_weff_dn1: f64 = *var_weff_dn1_slot;
        let mut var_weff_dn10: f64 = *var_weff_dn10_slot;
        let mut var_weff_dn3: f64 = *var_weff_dn3_slot;
        let mut var_weff_dn4: f64 = *var_weff_dn4_slot;
        let mut var_weff_dn5: f64 = *var_weff_dn5_slot;
        let mut var_weff_dn6: f64 = *var_weff_dn6_slot;
        let mut var_weff_dn7: f64 = *var_weff_dn7_slot;
        let mut var_weff_dn8: f64 = *var_weff_dn8_slot;
        let mut var_weff_dn9: f64 = *var_weff_dn9_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;
        let mut var_xd: f64 = *var_xd_slot;
        let mut var_xd_dn0: f64 = *var_xd_dn0_slot;
        let mut var_xd_dn1: f64 = *var_xd_dn1_slot;
        let mut var_xd_dn10: f64 = *var_xd_dn10_slot;
        let mut var_xd_dn3: f64 = *var_xd_dn3_slot;
        let mut var_xd_dn4: f64 = *var_xd_dn4_slot;
        let mut var_xd_dn5: f64 = *var_xd_dn5_slot;
        let mut var_xd_dn6: f64 = *var_xd_dn6_slot;
        let mut var_xd_dn7: f64 = *var_xd_dn7_slot;
        let mut var_xd_dn8: f64 = *var_xd_dn8_slot;
        let mut var_xd_dn9: f64 = *var_xd_dn9_slot;
        let mut var_xd_rv: f64 = *var_xd_rv_slot;
        let mut var_xi_w1: f64 = *var_xi_w1_slot;
        let mut var_xi_w1_dn0: f64 = *var_xi_w1_dn0_slot;
        let mut var_xi_w1_dn1: f64 = *var_xi_w1_dn1_slot;
        let mut var_xi_w1_dn10: f64 = *var_xi_w1_dn10_slot;
        let mut var_xi_w1_dn3: f64 = *var_xi_w1_dn3_slot;
        let mut var_xi_w1_dn4: f64 = *var_xi_w1_dn4_slot;
        let mut var_xi_w1_dn5: f64 = *var_xi_w1_dn5_slot;
        let mut var_xi_w1_dn6: f64 = *var_xi_w1_dn6_slot;
        let mut var_xi_w1_dn7: f64 = *var_xi_w1_dn7_slot;
        let mut var_xi_w1_dn8: f64 = *var_xi_w1_dn8_slot;
        let mut var_xi_w1_dn9: f64 = *var_xi_w1_dn9_slot;
        let mut var_xi_w1_rv: f64 = *var_xi_w1_rv_slot;

        let (assign5110_e4874, assign5110_e4874_d_n0, assign5110_e4874_d_n1, assign5110_e4874_d_n3, assign5110_e4874_d_n4, assign5110_e4874_d_n5, assign5110_e4874_d_n6, assign5110_e4874_d_n7, assign5110_e4874_d_n8, assign5110_e4874_d_n9, assign5110_e4874_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) && (var_guard89 != 0.0)) {
        let assign5110_e4867: f64 = (-var_bavl_t);
        let assign5110_e4870: f64 = (var_vl).powf(p.p40);
        let assign5110_e4871: f64 = (assign5110_e4867 * assign5110_e4870);
        let assign5110_e4872: f64 = (assign5110_e4871).exp();
        (assign5110_e4872, (assign5110_e4872 * (((-var_bavl_t_dn0) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn0)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn0 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn1) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn1)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn1 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn3) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn3)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn3 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn4) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn4)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn4 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn5) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn5)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn5 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn6) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn6)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn6 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn7) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn7)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn7 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn8) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn8)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn8 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn9) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn9)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn9 / var_vl))) }))), (assign5110_e4872 * (((-var_bavl_t_dn10) * assign5110_e4870) + (assign5110_e4867 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn10)) } } else { (assign5110_e4870 * (p.p40 * (var_vl_dn10 / var_vl))) }))),)
    } else {
        (var_expmm1, var_expmm1_dn0, var_expmm1_dn1, var_expmm1_dn3, var_expmm1_dn4, var_expmm1_dn5, var_expmm1_dn6, var_expmm1_dn7, var_expmm1_dn8, var_expmm1_dn9, var_expmm1_dn10,)
    }
};
        var_expmm1 = assign5110_e4874;
        var_expmm1_dn0 = assign5110_e4874_d_n0;
        var_expmm1_dn1 = assign5110_e4874_d_n1;
        var_expmm1_dn3 = assign5110_e4874_d_n3;
        var_expmm1_dn4 = assign5110_e4874_d_n4;
        var_expmm1_dn5 = assign5110_e4874_d_n5;
        var_expmm1_dn6 = assign5110_e4874_d_n6;
        var_expmm1_dn7 = assign5110_e4874_d_n7;
        var_expmm1_dn8 = assign5110_e4874_d_n8;
        var_expmm1_dn9 = assign5110_e4874_d_n9;
        var_expmm1_dn10 = assign5110_e4874_d_n10;
        var_expmm1_rv = 0.0;

        let (assign5120_e4886,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) && (var_guard89 == 0.0)) {
        let assign5120_e4884: f64 = (p.p138).exp();
        (assign5120_e4884,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign5120_e4886;
        var_expl_rv = 0.0;

        let (assign5130_e4908, assign5130_e4908_d_n0, assign5130_e4908_d_n1, assign5130_e4908_d_n3, assign5130_e4908_d_n4, assign5130_e4908_d_n5, assign5130_e4908_d_n6, assign5130_e4908_d_n7, assign5130_e4908_d_n8, assign5130_e4908_d_n9, assign5130_e4908_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) && (var_guard89 == 0.0)) {
        let assign5130_e4898: f64 = (-var_bavl_t);
        let assign5130_e4901: f64 = (var_vl).powf(p.p40);
        let assign5130_e4902: f64 = (assign5130_e4898 * assign5130_e4901);
        let assign5130_e4904: f64 = (assign5130_e4902 - p.p138);
        let assign5130_e4905: f64 = (1.0 + assign5130_e4904);
        let assign5130_e4906: f64 = (var_expl * assign5130_e4905);
        (assign5130_e4906, (var_expl * (((-var_bavl_t_dn0) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn0)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn0 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn1) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn1)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn1 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn3) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn3)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn3 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn4) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn4)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn4 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn5) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn5)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn5 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn6) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn6)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn6 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn7) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn7)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn7 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn8) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn8)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn8 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn9) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn9)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn9 / var_vl))) }))), (var_expl * (((-var_bavl_t_dn10) * assign5130_e4901) + (assign5130_e4898 * if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((var_vl).powf(p.p40 - 1.0) * var_vl_dn10)) } } else { (assign5130_e4901 * (p.p40 * (var_vl_dn10 / var_vl))) }))),)
    } else {
        (var_expmm1, var_expmm1_dn0, var_expmm1_dn1, var_expmm1_dn3, var_expmm1_dn4, var_expmm1_dn5, var_expmm1_dn6, var_expmm1_dn7, var_expmm1_dn8, var_expmm1_dn9, var_expmm1_dn10,)
    }
};
        var_expmm1 = assign5130_e4908;
        var_expmm1_dn0 = assign5130_e4908_d_n0;
        var_expmm1_dn1 = assign5130_e4908_d_n1;
        var_expmm1_dn3 = assign5130_e4908_d_n3;
        var_expmm1_dn4 = assign5130_e4908_d_n4;
        var_expmm1_dn5 = assign5130_e4908_d_n5;
        var_expmm1_dn6 = assign5130_e4908_d_n6;
        var_expmm1_dn7 = assign5130_e4908_d_n7;
        var_expmm1_dn8 = assign5130_e4908_d_n8;
        var_expmm1_dn9 = assign5130_e4908_d_n9;
        var_expmm1_dn10 = assign5130_e4908_d_n10;
        var_expmm1_rv = 0.0;

        let (assign5140_e4922, assign5140_e4922_d_n0, assign5140_e4922_d_n1, assign5140_e4922_d_n3, assign5140_e4922_d_n4, assign5140_e4922_d_n5, assign5140_e4922_d_n6, assign5140_e4922_d_n7, assign5140_e4922_d_n8, assign5140_e4922_d_n9, assign5140_e4922_d_n10,) = {
    if (((var_guard85 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) {
        let assign5140_e4916: f64 = (p.p39 / var_bavl_t);
        let assign5140_e4918: f64 = (assign5140_e4916 * var_vl);
        let assign5140_e4920: f64 = (assign5140_e4918 * var_expmm1);
        (assign5140_e4920, (((((-((p.p39 * var_bavl_t_dn0) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn0)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn0)), (((((-((p.p39 * var_bavl_t_dn1) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn1)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn1)), (((((-((p.p39 * var_bavl_t_dn3) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn3)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn3)), (((((-((p.p39 * var_bavl_t_dn4) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn4)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn4)), (((((-((p.p39 * var_bavl_t_dn5) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn5)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn5)), (((((-((p.p39 * var_bavl_t_dn6) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn6)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn6)), (((((-((p.p39 * var_bavl_t_dn7) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn7)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn7)), (((((-((p.p39 * var_bavl_t_dn8) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn8)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn8)), (((((-((p.p39 * var_bavl_t_dn9) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn9)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn9)), (((((-((p.p39 * var_bavl_t_dn10) / (var_bavl_t * var_bavl_t))) * var_vl) + (assign5140_e4916 * var_vl_dn10)) * var_expmm1) + (assign5140_e4918 * var_expmm1_dn10)),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5140_e4922;
        var_gem_dn0 = assign5140_e4922_d_n0;
        var_gem_dn1 = assign5140_e4922_d_n1;
        var_gem_dn3 = assign5140_e4922_d_n3;
        var_gem_dn4 = assign5140_e4922_d_n4;
        var_gem_dn5 = assign5140_e4922_d_n5;
        var_gem_dn6 = assign5140_e4922_d_n6;
        var_gem_dn7 = assign5140_e4922_d_n7;
        var_gem_dn8 = assign5140_e4922_d_n8;
        var_gem_dn9 = assign5140_e4922_d_n9;
        var_gem_dn10 = assign5140_e4922_d_n10;
        var_gem_rv = 0.0;

        let assign5150_e4925: f64 = if p.p38 == 2.0 { 1.0 } else { 0.0 };
        var_guard90 = assign5150_e4925;
        var_guard90_rv = 0.0;

        let assign5160_e4928: f64 = if var_vb2c1 < var_vdc_t { 1.0 } else { 0.0 };
        var_guard91 = assign5160_e4928;
        var_guard91_rv = 0.0;

        let (assign5170_e4945,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5170_e4939: f64 = (2.0 * p.p45);
        let assign5170_e4942: f64 = (p.p44 * p.p44);
        let assign5170_e4943: f64 = (assign5170_e4939 / assign5170_e4942);
        (assign5170_e4943,)
    } else {
        (var_dedx0,)
    }
};
        var_dedx0 = assign5170_e4945;
        var_dedx0_rv = 0.0;

        let (assign5180_e4960, assign5180_e4960_d_n0, assign5180_e4960_d_n1, assign5180_e4960_d_n3, assign5180_e4960_d_n4, assign5180_e4960_d_n5, assign5180_e4960_d_n6, assign5180_e4960_d_n7, assign5180_e4960_d_n8, assign5180_e4960_d_n9, assign5180_e4960_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5180_e4956: f64 = (var_vdc_t - var_vb2c1);
        let assign5180_e4958: f64 = (assign5180_e4956 / var_icap_ihc);
        (assign5180_e4958, (((var_vdc_t_dn0 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn0)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn1 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn1)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn3 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn3)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn4 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn4)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn5 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn5)) / (var_icap_ihc * var_icap_ihc)), ((((var_vdc_t_dn6 - var_vb2c1_dn6) * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn6)) / (var_icap_ihc * var_icap_ihc)), ((((var_vdc_t_dn7 - var_vb2c1_dn7) * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn7)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn8 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn8)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn9 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn9)) / (var_icap_ihc * var_icap_ihc)), (((var_vdc_t_dn10 * var_icap_ihc) - (assign5180_e4956 * var_icap_ihc_dn10)) / (var_icap_ihc * var_icap_ihc)),)
    } else {
        (var_sqr_arg, var_sqr_arg_dn0, var_sqr_arg_dn1, var_sqr_arg_dn3, var_sqr_arg_dn4, var_sqr_arg_dn5, var_sqr_arg_dn6, var_sqr_arg_dn7, var_sqr_arg_dn8, var_sqr_arg_dn9, var_sqr_arg_dn10,)
    }
};
        var_sqr_arg = assign5180_e4960;
        var_sqr_arg_dn0 = assign5180_e4960_d_n0;
        var_sqr_arg_dn1 = assign5180_e4960_d_n1;
        var_sqr_arg_dn3 = assign5180_e4960_d_n3;
        var_sqr_arg_dn4 = assign5180_e4960_d_n4;
        var_sqr_arg_dn5 = assign5180_e4960_d_n5;
        var_sqr_arg_dn6 = assign5180_e4960_d_n6;
        var_sqr_arg_dn7 = assign5180_e4960_d_n7;
        var_sqr_arg_dn8 = assign5180_e4960_d_n8;
        var_sqr_arg_dn9 = assign5180_e4960_d_n9;
        var_sqr_arg_dn10 = assign5180_e4960_d_n10;
        var_sqr_arg_rv = 0.0;

        let (assign5190_e4976, assign5190_e4976_d_n0, assign5190_e4976_d_n1, assign5190_e4976_d_n3, assign5190_e4976_d_n4, assign5190_e4976_d_n5, assign5190_e4976_d_n6, assign5190_e4976_d_n7, assign5190_e4976_d_n8, assign5190_e4976_d_n9, assign5190_e4976_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5190_e4971: f64 = (2.0 * var_sqr_arg);
        let assign5190_e4973: f64 = (assign5190_e4971 / var_dedx0);
        let assign5190_e4974: f64 = (assign5190_e4973).sqrt();
        (assign5190_e4974, (((2.0 * var_sqr_arg_dn0) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn1) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn3) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn4) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn5) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn6) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn7) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn8) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn9) / var_dedx0) / (2.0 * assign5190_e4974)), (((2.0 * var_sqr_arg_dn10) / var_dedx0) / (2.0 * assign5190_e4974)),)
    } else {
        (var_xd, var_xd_dn0, var_xd_dn1, var_xd_dn3, var_xd_dn4, var_xd_dn5, var_xd_dn6, var_xd_dn7, var_xd_dn8, var_xd_dn9, var_xd_dn10,)
    }
};
        var_xd = assign5190_e4976;
        var_xd_dn0 = assign5190_e4976_d_n0;
        var_xd_dn1 = assign5190_e4976_d_n1;
        var_xd_dn3 = assign5190_e4976_d_n3;
        var_xd_dn4 = assign5190_e4976_d_n4;
        var_xd_dn5 = assign5190_e4976_d_n5;
        var_xd_dn6 = assign5190_e4976_d_n6;
        var_xd_dn7 = assign5190_e4976_d_n7;
        var_xd_dn8 = assign5190_e4976_d_n8;
        var_xd_dn9 = assign5190_e4976_d_n9;
        var_xd_dn10 = assign5190_e4976_d_n10;
        var_xd_rv = 0.0;

        let assign5200_e4979: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard92 = assign5200_e4979;
        var_guard92_rv = 0.0;

        let (assign5210_e4992, assign5210_e4992_d_n0, assign5210_e4992_d_n1, assign5210_e4992_d_n3, assign5210_e4992_d_n4, assign5210_e4992_d_n5, assign5210_e4992_d_n6, assign5210_e4992_d_n7, assign5210_e4992_d_n8, assign5210_e4992_d_n9, assign5210_e4992_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard92 != 0.0)) {
        (p.p44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_weff, var_weff_dn0, var_weff_dn1, var_weff_dn3, var_weff_dn4, var_weff_dn5, var_weff_dn6, var_weff_dn7, var_weff_dn8, var_weff_dn9, var_weff_dn10,)
    }
};
        var_weff = assign5210_e4992;
        var_weff_dn0 = assign5210_e4992_d_n0;
        var_weff_dn1 = assign5210_e4992_d_n1;
        var_weff_dn3 = assign5210_e4992_d_n3;
        var_weff_dn4 = assign5210_e4992_d_n4;
        var_weff_dn5 = assign5210_e4992_d_n5;
        var_weff_dn6 = assign5210_e4992_d_n6;
        var_weff_dn7 = assign5210_e4992_d_n7;
        var_weff_dn8 = assign5210_e4992_d_n8;
        var_weff_dn9 = assign5210_e4992_d_n9;
        var_weff_dn10 = assign5210_e4992_d_n10;
        var_weff_rv = 0.0;

        let (assign5220_e5010, assign5220_e5010_d_n0, assign5220_e5010_d_n1, assign5220_e5010_d_n3, assign5220_e5010_d_n4, assign5220_e5010_d_n5, assign5220_e5010_d_n6, assign5220_e5010_d_n7, assign5220_e5010_d_n8, assign5220_e5010_d_n9, assign5220_e5010_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard92 == 0.0)) {
        let assign5220_e5007: f64 = (0.5 * var_xi_w);
        let assign5220_e5008: f64 = (1.0 - assign5220_e5007);
        (assign5220_e5008, (-(0.5 * var_xi_w_dn0)), (-(0.5 * var_xi_w_dn1)), (-(0.5 * var_xi_w_dn3)), (-(0.5 * var_xi_w_dn4)), (-(0.5 * var_xi_w_dn5)), (-(0.5 * var_xi_w_dn6)), (-(0.5 * var_xi_w_dn7)), (-(0.5 * var_xi_w_dn8)), (-(0.5 * var_xi_w_dn9)), (-(0.5 * var_xi_w_dn10)),)
    } else {
        (var_xi_w1, var_xi_w1_dn0, var_xi_w1_dn1, var_xi_w1_dn3, var_xi_w1_dn4, var_xi_w1_dn5, var_xi_w1_dn6, var_xi_w1_dn7, var_xi_w1_dn8, var_xi_w1_dn9, var_xi_w1_dn10,)
    }
};
        var_xi_w1 = assign5220_e5010;
        var_xi_w1_dn0 = assign5220_e5010_d_n0;
        var_xi_w1_dn1 = assign5220_e5010_d_n1;
        var_xi_w1_dn3 = assign5220_e5010_d_n3;
        var_xi_w1_dn4 = assign5220_e5010_d_n4;
        var_xi_w1_dn5 = assign5220_e5010_d_n5;
        var_xi_w1_dn6 = assign5220_e5010_d_n6;
        var_xi_w1_dn7 = assign5220_e5010_d_n7;
        var_xi_w1_dn8 = assign5220_e5010_d_n8;
        var_xi_w1_dn9 = assign5220_e5010_d_n9;
        var_xi_w1_dn10 = assign5220_e5010_d_n10;
        var_xi_w1_rv = 0.0;

        let (assign5230_e5028, assign5230_e5028_d_n0, assign5230_e5028_d_n1, assign5230_e5028_d_n3, assign5230_e5028_d_n4, assign5230_e5028_d_n5, assign5230_e5028_d_n6, assign5230_e5028_d_n7, assign5230_e5028_d_n8, assign5230_e5028_d_n9, assign5230_e5028_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard92 == 0.0)) {
        let assign5230_e5024: f64 = (p.p44 * var_xi_w1);
        let assign5230_e5026: f64 = (assign5230_e5024 * var_xi_w1);
        (assign5230_e5026, (((p.p44 * var_xi_w1_dn0) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn0)), (((p.p44 * var_xi_w1_dn1) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn1)), (((p.p44 * var_xi_w1_dn3) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn3)), (((p.p44 * var_xi_w1_dn4) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn4)), (((p.p44 * var_xi_w1_dn5) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn5)), (((p.p44 * var_xi_w1_dn6) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn6)), (((p.p44 * var_xi_w1_dn7) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn7)), (((p.p44 * var_xi_w1_dn8) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn8)), (((p.p44 * var_xi_w1_dn9) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn9)), (((p.p44 * var_xi_w1_dn10) * var_xi_w1) + (assign5230_e5024 * var_xi_w1_dn10)),)
    } else {
        (var_weff, var_weff_dn0, var_weff_dn1, var_weff_dn3, var_weff_dn4, var_weff_dn5, var_weff_dn6, var_weff_dn7, var_weff_dn8, var_weff_dn9, var_weff_dn10,)
    }
};
        var_weff = assign5230_e5028;
        var_weff_dn0 = assign5230_e5028_d_n0;
        var_weff_dn1 = assign5230_e5028_d_n1;
        var_weff_dn3 = assign5230_e5028_d_n3;
        var_weff_dn4 = assign5230_e5028_d_n4;
        var_weff_dn5 = assign5230_e5028_d_n5;
        var_weff_dn6 = assign5230_e5028_d_n6;
        var_weff_dn7 = assign5230_e5028_d_n7;
        var_weff_dn8 = assign5230_e5028_d_n8;
        var_weff_dn9 = assign5230_e5028_d_n9;
        var_weff_dn10 = assign5230_e5028_d_n10;
        var_weff_rv = 0.0;

        let (assign5240_e5050, assign5240_e5050_d_n0, assign5240_e5050_d_n1, assign5240_e5050_d_n3, assign5240_e5050_d_n4, assign5240_e5050_d_n5, assign5240_e5050_d_n6, assign5240_e5050_d_n7, assign5240_e5050_d_n8, assign5240_e5050_d_n9, assign5240_e5050_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5240_e5039: f64 = (var_xd * var_weff);
        let assign5240_e5042: f64 = (var_xd * var_xd);
        let assign5240_e5045: f64 = (var_weff * var_weff);
        let assign5240_e5046: f64 = (assign5240_e5042 + assign5240_e5045);
        let assign5240_e5047: f64 = (assign5240_e5046).sqrt();
        let assign5240_e5048: f64 = (assign5240_e5039 / assign5240_e5047);
        (assign5240_e5048, (((((var_xd_dn0 * var_weff) + (var_xd * var_weff_dn0)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn0 * var_xd) + (var_xd * var_xd_dn0)) + ((var_weff_dn0 * var_weff) + (var_weff * var_weff_dn0))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn1 * var_weff) + (var_xd * var_weff_dn1)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn1 * var_xd) + (var_xd * var_xd_dn1)) + ((var_weff_dn1 * var_weff) + (var_weff * var_weff_dn1))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn3 * var_weff) + (var_xd * var_weff_dn3)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn3 * var_xd) + (var_xd * var_xd_dn3)) + ((var_weff_dn3 * var_weff) + (var_weff * var_weff_dn3))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn4 * var_weff) + (var_xd * var_weff_dn4)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn4 * var_xd) + (var_xd * var_xd_dn4)) + ((var_weff_dn4 * var_weff) + (var_weff * var_weff_dn4))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn5 * var_weff) + (var_xd * var_weff_dn5)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn5 * var_xd) + (var_xd * var_xd_dn5)) + ((var_weff_dn5 * var_weff) + (var_weff * var_weff_dn5))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn6 * var_weff) + (var_xd * var_weff_dn6)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn6 * var_xd) + (var_xd * var_xd_dn6)) + ((var_weff_dn6 * var_weff) + (var_weff * var_weff_dn6))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn7 * var_weff) + (var_xd * var_weff_dn7)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn7 * var_xd) + (var_xd * var_xd_dn7)) + ((var_weff_dn7 * var_weff) + (var_weff * var_weff_dn7))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn8 * var_weff) + (var_xd * var_weff_dn8)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn8 * var_xd) + (var_xd * var_xd_dn8)) + ((var_weff_dn8 * var_weff) + (var_weff * var_weff_dn8))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn9 * var_weff) + (var_xd * var_weff_dn9)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn9 * var_xd) + (var_xd * var_xd_dn9)) + ((var_weff_dn9 * var_weff) + (var_weff * var_weff_dn9))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)), (((((var_xd_dn10 * var_weff) + (var_xd * var_weff_dn10)) * assign5240_e5047) - (assign5240_e5039 * ((((var_xd_dn10 * var_xd) + (var_xd * var_xd_dn10)) + ((var_weff_dn10 * var_weff) + (var_weff * var_weff_dn10))) / (2.0 * assign5240_e5047)))) / (assign5240_e5047 * assign5240_e5047)),)
    } else {
        (var_wd, var_wd_dn0, var_wd_dn1, var_wd_dn3, var_wd_dn4, var_wd_dn5, var_wd_dn6, var_wd_dn7, var_wd_dn8, var_wd_dn9, var_wd_dn10,)
    }
};
        var_wd = assign5240_e5050;
        var_wd_dn0 = assign5240_e5050_d_n0;
        var_wd_dn1 = assign5240_e5050_d_n1;
        var_wd_dn3 = assign5240_e5050_d_n3;
        var_wd_dn4 = assign5240_e5050_d_n4;
        var_wd_dn5 = assign5240_e5050_d_n5;
        var_wd_dn6 = assign5240_e5050_d_n6;
        var_wd_dn7 = assign5240_e5050_d_n7;
        var_wd_dn8 = assign5240_e5050_d_n8;
        var_wd_dn9 = assign5240_e5050_d_n9;
        var_wd_dn10 = assign5240_e5050_d_n10;
        var_wd_rv = 0.0;

        let (assign5250_e5065, assign5250_e5065_d_n0, assign5250_e5065_d_n1, assign5250_e5065_d_n3, assign5250_e5065_d_n4, assign5250_e5065_d_n5, assign5250_e5065_d_n6, assign5250_e5065_d_n7, assign5250_e5065_d_n8, assign5250_e5065_d_n9, assign5250_e5065_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5250_e5061: f64 = (var_vdc_t - var_vb2c1);
        let assign5250_e5063: f64 = (assign5250_e5061 / var_wd);
        (assign5250_e5063, (((var_vdc_t_dn0 * var_wd) - (assign5250_e5061 * var_wd_dn0)) / (var_wd * var_wd)), (((var_vdc_t_dn1 * var_wd) - (assign5250_e5061 * var_wd_dn1)) / (var_wd * var_wd)), (((var_vdc_t_dn3 * var_wd) - (assign5250_e5061 * var_wd_dn3)) / (var_wd * var_wd)), (((var_vdc_t_dn4 * var_wd) - (assign5250_e5061 * var_wd_dn4)) / (var_wd * var_wd)), (((var_vdc_t_dn5 * var_wd) - (assign5250_e5061 * var_wd_dn5)) / (var_wd * var_wd)), ((((var_vdc_t_dn6 - var_vb2c1_dn6) * var_wd) - (assign5250_e5061 * var_wd_dn6)) / (var_wd * var_wd)), ((((var_vdc_t_dn7 - var_vb2c1_dn7) * var_wd) - (assign5250_e5061 * var_wd_dn7)) / (var_wd * var_wd)), (((var_vdc_t_dn8 * var_wd) - (assign5250_e5061 * var_wd_dn8)) / (var_wd * var_wd)), (((var_vdc_t_dn9 * var_wd) - (assign5250_e5061 * var_wd_dn9)) / (var_wd * var_wd)), (((var_vdc_t_dn10 * var_wd) - (assign5250_e5061 * var_wd_dn10)) / (var_wd * var_wd)),)
    } else {
        (var_eav, var_eav_dn0, var_eav_dn1, var_eav_dn3, var_eav_dn4, var_eav_dn5, var_eav_dn6, var_eav_dn7, var_eav_dn8, var_eav_dn9, var_eav_dn10,)
    }
};
        var_eav = assign5250_e5065;
        var_eav_dn0 = assign5250_e5065_d_n0;
        var_eav_dn1 = assign5250_e5065_d_n1;
        var_eav_dn3 = assign5250_e5065_d_n3;
        var_eav_dn4 = assign5250_e5065_d_n4;
        var_eav_dn5 = assign5250_e5065_d_n5;
        var_eav_dn6 = assign5250_e5065_d_n6;
        var_eav_dn7 = assign5250_e5065_d_n7;
        var_eav_dn8 = assign5250_e5065_d_n8;
        var_eav_dn9 = assign5250_e5065_d_n9;
        var_eav_dn10 = assign5250_e5065_d_n10;
        var_eav_rv = 0.0;

        let (assign5260_e5084, assign5260_e5084_d_n0, assign5260_e5084_d_n1, assign5260_e5084_d_n3, assign5260_e5084_d_n4, assign5260_e5084_d_n5, assign5260_e5084_d_n6, assign5260_e5084_d_n7, assign5260_e5084_d_n8, assign5260_e5084_d_n9, assign5260_e5084_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5260_e5077: f64 = (0.5 * var_wd);
        let assign5260_e5079: f64 = (assign5260_e5077 * var_dedx0);
        let assign5260_e5081: f64 = (assign5260_e5079 * var_icap_ihc);
        let assign5260_e5082: f64 = (var_eav + assign5260_e5081);
        (assign5260_e5082, (var_eav_dn0 + ((((0.5 * var_wd_dn0) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn0))), (var_eav_dn1 + ((((0.5 * var_wd_dn1) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn1))), (var_eav_dn3 + ((((0.5 * var_wd_dn3) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn3))), (var_eav_dn4 + ((((0.5 * var_wd_dn4) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn4))), (var_eav_dn5 + ((((0.5 * var_wd_dn5) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn5))), (var_eav_dn6 + ((((0.5 * var_wd_dn6) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn6))), (var_eav_dn7 + ((((0.5 * var_wd_dn7) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn7))), (var_eav_dn8 + ((((0.5 * var_wd_dn8) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn8))), (var_eav_dn9 + ((((0.5 * var_wd_dn9) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn9))), (var_eav_dn10 + ((((0.5 * var_wd_dn10) * var_dedx0) * var_icap_ihc) + (assign5260_e5079 * var_icap_ihc_dn10))),)
    } else {
        (var_e0, var_e0_dn0, var_e0_dn1, var_e0_dn3, var_e0_dn4, var_e0_dn5, var_e0_dn6, var_e0_dn7, var_e0_dn8, var_e0_dn9, var_e0_dn10,)
    }
};
        var_e0 = assign5260_e5084;
        var_e0_dn0 = assign5260_e5084_d_n0;
        var_e0_dn1 = assign5260_e5084_d_n1;
        var_e0_dn3 = assign5260_e5084_d_n3;
        var_e0_dn4 = assign5260_e5084_d_n4;
        var_e0_dn5 = assign5260_e5084_d_n5;
        var_e0_dn6 = assign5260_e5084_d_n6;
        var_e0_dn7 = assign5260_e5084_d_n7;
        var_e0_dn8 = assign5260_e5084_d_n8;
        var_e0_dn9 = assign5260_e5084_d_n9;
        var_e0_dn10 = assign5260_e5084_d_n10;
        var_e0_rv = 0.0;

        let assign5270_e5087: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard93 = assign5270_e5087;
        var_guard93_rv = 0.0;

        let (assign5280_e5100, assign5280_e5100_d_n0, assign5280_e5100_d_n1, assign5280_e5100_d_n3, assign5280_e5100_d_n4, assign5280_e5100_d_n5, assign5280_e5100_d_n6, assign5280_e5100_d_n7, assign5280_e5100_d_n8, assign5280_e5100_d_n9, assign5280_e5100_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 != 0.0)) {
        (var_e0, var_e0_dn0, var_e0_dn1, var_e0_dn3, var_e0_dn4, var_e0_dn5, var_e0_dn6, var_e0_dn7, var_e0_dn8, var_e0_dn9, var_e0_dn10,)
    } else {
        (var_em, var_em_dn0, var_em_dn1, var_em_dn3, var_em_dn4, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8, var_em_dn9, var_em_dn10,)
    }
};
        var_em = assign5280_e5100;
        var_em_dn0 = assign5280_e5100_d_n0;
        var_em_dn1 = assign5280_e5100_d_n1;
        var_em_dn3 = assign5280_e5100_d_n3;
        var_em_dn4 = assign5280_e5100_d_n4;
        var_em_dn5 = assign5280_e5100_d_n5;
        var_em_dn6 = assign5280_e5100_d_n6;
        var_em_dn7 = assign5280_e5100_d_n7;
        var_em_dn8 = assign5280_e5100_d_n8;
        var_em_dn9 = assign5280_e5100_d_n9;
        var_em_dn10 = assign5280_e5100_d_n10;
        var_em_rv = 0.0;

        let (assign5290_e5124, assign5290_e5124_d_n0, assign5290_e5124_d_n1, assign5290_e5124_d_n3, assign5290_e5124_d_n4, assign5290_e5124_d_n5, assign5290_e5124_d_n6, assign5290_e5124_d_n7, assign5290_e5124_d_n8, assign5290_e5124_d_n9, assign5290_e5124_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 == 0.0)) {
        let assign5290_e5115: f64 = (2.0 * p.p46);
        let assign5290_e5119: f64 = (2.0 * var_xi_w);
        let assign5290_e5120: f64 = (1.0 + assign5290_e5119);
        let assign5290_e5121: f64 = (assign5290_e5115 * assign5290_e5120);
        let assign5290_e5122: f64 = (1.0 + assign5290_e5121);
        (assign5290_e5122, (assign5290_e5115 * (2.0 * var_xi_w_dn0)), (assign5290_e5115 * (2.0 * var_xi_w_dn1)), (assign5290_e5115 * (2.0 * var_xi_w_dn3)), (assign5290_e5115 * (2.0 * var_xi_w_dn4)), (assign5290_e5115 * (2.0 * var_xi_w_dn5)), (assign5290_e5115 * (2.0 * var_xi_w_dn6)), (assign5290_e5115 * (2.0 * var_xi_w_dn7)), (assign5290_e5115 * (2.0 * var_xi_w_dn8)), (assign5290_e5115 * (2.0 * var_xi_w_dn9)), (assign5290_e5115 * (2.0 * var_xi_w_dn10)),)
    } else {
        (var_shw, var_shw_dn0, var_shw_dn1, var_shw_dn3, var_shw_dn4, var_shw_dn5, var_shw_dn6, var_shw_dn7, var_shw_dn8, var_shw_dn9, var_shw_dn10,)
    }
};
        var_shw = assign5290_e5124;
        var_shw_dn0 = assign5290_e5124_d_n0;
        var_shw_dn1 = assign5290_e5124_d_n1;
        var_shw_dn3 = assign5290_e5124_d_n3;
        var_shw_dn4 = assign5290_e5124_d_n4;
        var_shw_dn5 = assign5290_e5124_d_n5;
        var_shw_dn6 = assign5290_e5124_d_n6;
        var_shw_dn7 = assign5290_e5124_d_n7;
        var_shw_dn8 = assign5290_e5124_d_n8;
        var_shw_dn9 = assign5290_e5124_d_n9;
        var_shw_dn10 = assign5290_e5124_d_n10;
        var_shw_rv = 0.0;

        let (assign5300_e5146,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 == 0.0)) {
        let assign5300_e5138: f64 = (1.0 + p.p46);
        let assign5300_e5142: f64 = (2.0 * p.p46);
        let assign5300_e5143: f64 = (1.0 + assign5300_e5142);
        let assign5300_e5144: f64 = (assign5300_e5138 / assign5300_e5143);
        (assign5300_e5144,)
    } else {
        (var_efi,)
    }
};
        var_efi = assign5300_e5146;
        var_efi_rv = 0.0;

        let (assign5310_e5174, assign5310_e5174_d_n0, assign5310_e5174_d_n1, assign5310_e5174_d_n3, assign5310_e5174_d_n4, assign5310_e5174_d_n5, assign5310_e5174_d_n6, assign5310_e5174_d_n7, assign5310_e5174_d_n8, assign5310_e5174_d_n9, assign5310_e5174_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 == 0.0)) {
        let assign5310_e5161: f64 = (0.5 * var_wd);
        let assign5310_e5163: f64 = (assign5310_e5161 * var_dedx0);
        let assign5310_e5168: f64 = (p.p61 * var_shw);
        let assign5310_e5169: f64 = (var_in_ / assign5310_e5168);
        let assign5310_e5170: f64 = (var_efi - assign5310_e5169);
        let assign5310_e5171: f64 = (assign5310_e5163 * assign5310_e5170);
        let assign5310_e5172: f64 = (var_eav - assign5310_e5171);
        (assign5310_e5172, (var_eav_dn0 - ((((0.5 * var_wd_dn0) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn0 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn0))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn1 - ((((0.5 * var_wd_dn1) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn1 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn1))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn3 - ((((0.5 * var_wd_dn3) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn3 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn3))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn4 - ((((0.5 * var_wd_dn4) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn4 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn4))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn5 - ((((0.5 * var_wd_dn5) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn5 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn5))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn6 - ((((0.5 * var_wd_dn6) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn6 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn6))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn7 - ((((0.5 * var_wd_dn7) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn7 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn7))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn8 - ((((0.5 * var_wd_dn8) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn8 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn8))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn9 - ((((0.5 * var_wd_dn9) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn9 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn9))) / (assign5310_e5168 * assign5310_e5168)))))), (var_eav_dn10 - ((((0.5 * var_wd_dn10) * var_dedx0) * assign5310_e5170) + (assign5310_e5163 * (-(((var_in__dn10 * assign5310_e5168) - (var_in_ * (p.p61 * var_shw_dn10))) / (assign5310_e5168 * assign5310_e5168)))))),)
    } else {
        (var_ew, var_ew_dn0, var_ew_dn1, var_ew_dn3, var_ew_dn4, var_ew_dn5, var_ew_dn6, var_ew_dn7, var_ew_dn8, var_ew_dn9, var_ew_dn10,)
    }
};
        var_ew = assign5310_e5174;
        var_ew_dn0 = assign5310_e5174_d_n0;
        var_ew_dn1 = assign5310_e5174_d_n1;
        var_ew_dn3 = assign5310_e5174_d_n3;
        var_ew_dn4 = assign5310_e5174_d_n4;
        var_ew_dn5 = assign5310_e5174_d_n5;
        var_ew_dn6 = assign5310_e5174_d_n6;
        var_ew_dn7 = assign5310_e5174_d_n7;
        var_ew_dn8 = assign5310_e5174_d_n8;
        var_ew_dn9 = assign5310_e5174_d_n9;
        var_ew_dn10 = assign5310_e5174_d_n10;
        var_ew_rv = 0.0;

        let (assign5320_e5204, assign5320_e5204_d_n0, assign5320_e5204_d_n1, assign5320_e5204_d_n3, assign5320_e5204_d_n4, assign5320_e5204_d_n5, assign5320_e5204_d_n6, assign5320_e5204_d_n7, assign5320_e5204_d_n8, assign5320_e5204_d_n9, assign5320_e5204_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 == 0.0)) {
        let assign5320_e5188: f64 = (var_ew - var_e0);
        let assign5320_e5191: f64 = (var_ew - var_e0);
        let assign5320_e5192: f64 = (assign5320_e5188 * assign5320_e5191);
        let assign5320_e5195: f64 = (0.1 * var_eav);
        let assign5320_e5197: f64 = (assign5320_e5195 * var_eav);
        let assign5320_e5199: f64 = (assign5320_e5197 * var_icap);
        let assign5320_e5201: f64 = (assign5320_e5199 / p.p61);
        let assign5320_e5202: f64 = (assign5320_e5192 + assign5320_e5201);
        (assign5320_e5202, ((((var_ew_dn0 - var_e0_dn0) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn0 - var_e0_dn0))) + ((((((0.1 * var_eav_dn0) * var_eav) + (assign5320_e5195 * var_eav_dn0)) * var_icap) + (assign5320_e5197 * var_icap_dn0)) / p.p61)), ((((var_ew_dn1 - var_e0_dn1) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn1 - var_e0_dn1))) + ((((((0.1 * var_eav_dn1) * var_eav) + (assign5320_e5195 * var_eav_dn1)) * var_icap) + (assign5320_e5197 * var_icap_dn1)) / p.p61)), ((((var_ew_dn3 - var_e0_dn3) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn3 - var_e0_dn3))) + ((((((0.1 * var_eav_dn3) * var_eav) + (assign5320_e5195 * var_eav_dn3)) * var_icap) + (assign5320_e5197 * var_icap_dn3)) / p.p61)), ((((var_ew_dn4 - var_e0_dn4) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn4 - var_e0_dn4))) + ((((((0.1 * var_eav_dn4) * var_eav) + (assign5320_e5195 * var_eav_dn4)) * var_icap) + (assign5320_e5197 * var_icap_dn4)) / p.p61)), ((((var_ew_dn5 - var_e0_dn5) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn5 - var_e0_dn5))) + ((((((0.1 * var_eav_dn5) * var_eav) + (assign5320_e5195 * var_eav_dn5)) * var_icap) + (assign5320_e5197 * var_icap_dn5)) / p.p61)), ((((var_ew_dn6 - var_e0_dn6) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn6 - var_e0_dn6))) + ((((((0.1 * var_eav_dn6) * var_eav) + (assign5320_e5195 * var_eav_dn6)) * var_icap) + (assign5320_e5197 * var_icap_dn6)) / p.p61)), ((((var_ew_dn7 - var_e0_dn7) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn7 - var_e0_dn7))) + ((((((0.1 * var_eav_dn7) * var_eav) + (assign5320_e5195 * var_eav_dn7)) * var_icap) + (assign5320_e5197 * var_icap_dn7)) / p.p61)), ((((var_ew_dn8 - var_e0_dn8) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn8 - var_e0_dn8))) + ((((((0.1 * var_eav_dn8) * var_eav) + (assign5320_e5195 * var_eav_dn8)) * var_icap) + (assign5320_e5197 * var_icap_dn8)) / p.p61)), ((((var_ew_dn9 - var_e0_dn9) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn9 - var_e0_dn9))) + ((((((0.1 * var_eav_dn9) * var_eav) + (assign5320_e5195 * var_eav_dn9)) * var_icap) + (assign5320_e5197 * var_icap_dn9)) / p.p61)), ((((var_ew_dn10 - var_e0_dn10) * assign5320_e5191) + (assign5320_e5188 * (var_ew_dn10 - var_e0_dn10))) + ((((((0.1 * var_eav_dn10) * var_eav) + (assign5320_e5195 * var_eav_dn10)) * var_icap) + (assign5320_e5197 * var_icap_dn10)) / p.p61)),)
    } else {
        (var_sqr_arg, var_sqr_arg_dn0, var_sqr_arg_dn1, var_sqr_arg_dn3, var_sqr_arg_dn4, var_sqr_arg_dn5, var_sqr_arg_dn6, var_sqr_arg_dn7, var_sqr_arg_dn8, var_sqr_arg_dn9, var_sqr_arg_dn10,)
    }
};
        var_sqr_arg = assign5320_e5204;
        var_sqr_arg_dn0 = assign5320_e5204_d_n0;
        var_sqr_arg_dn1 = assign5320_e5204_d_n1;
        var_sqr_arg_dn3 = assign5320_e5204_d_n3;
        var_sqr_arg_dn4 = assign5320_e5204_d_n4;
        var_sqr_arg_dn5 = assign5320_e5204_d_n5;
        var_sqr_arg_dn6 = assign5320_e5204_d_n6;
        var_sqr_arg_dn7 = assign5320_e5204_d_n7;
        var_sqr_arg_dn8 = assign5320_e5204_d_n8;
        var_sqr_arg_dn9 = assign5320_e5204_d_n9;
        var_sqr_arg_dn10 = assign5320_e5204_d_n10;
        var_sqr_arg_rv = 0.0;

        let (assign5330_e5225, assign5330_e5225_d_n0, assign5330_e5225_d_n1, assign5330_e5225_d_n3, assign5330_e5225_d_n4, assign5330_e5225_d_n5, assign5330_e5225_d_n6, assign5330_e5225_d_n7, assign5330_e5225_d_n8, assign5330_e5225_d_n9, assign5330_e5225_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard93 == 0.0)) {
        let assign5330_e5219: f64 = (var_ew + var_e0);
        let assign5330_e5221: f64 = (var_sqr_arg).sqrt();
        let assign5330_e5222: f64 = (assign5330_e5219 + assign5330_e5221);
        let assign5330_e5223: f64 = (0.5 * assign5330_e5222);
        (assign5330_e5223, (0.5 * ((var_ew_dn0 + var_e0_dn0) + (var_sqr_arg_dn0 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn1 + var_e0_dn1) + (var_sqr_arg_dn1 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn3 + var_e0_dn3) + (var_sqr_arg_dn3 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn4 + var_e0_dn4) + (var_sqr_arg_dn4 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn5 + var_e0_dn5) + (var_sqr_arg_dn5 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn6 + var_e0_dn6) + (var_sqr_arg_dn6 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn7 + var_e0_dn7) + (var_sqr_arg_dn7 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn8 + var_e0_dn8) + (var_sqr_arg_dn8 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn9 + var_e0_dn9) + (var_sqr_arg_dn9 / (2.0 * assign5330_e5221)))), (0.5 * ((var_ew_dn10 + var_e0_dn10) + (var_sqr_arg_dn10 / (2.0 * assign5330_e5221)))),)
    } else {
        (var_em, var_em_dn0, var_em_dn1, var_em_dn3, var_em_dn4, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8, var_em_dn9, var_em_dn10,)
    }
};
        var_em = assign5330_e5225;
        var_em_dn0 = assign5330_e5225_d_n0;
        var_em_dn1 = assign5330_e5225_d_n1;
        var_em_dn3 = assign5330_e5225_d_n3;
        var_em_dn4 = assign5330_e5225_d_n4;
        var_em_dn5 = assign5330_e5225_d_n5;
        var_em_dn6 = assign5330_e5225_d_n6;
        var_em_dn7 = assign5330_e5225_d_n7;
        var_em_dn8 = assign5330_e5225_d_n8;
        var_em_dn9 = assign5330_e5225_d_n9;
        var_em_dn10 = assign5330_e5225_d_n10;
        var_em_rv = 0.0;

        let (assign5340_e5240, assign5340_e5240_d_n0, assign5340_e5240_d_n1, assign5340_e5240_d_n3, assign5340_e5240_d_n4, assign5340_e5240_d_n5, assign5340_e5240_d_n6, assign5340_e5240_d_n7, assign5340_e5240_d_n8, assign5340_e5240_d_n9, assign5340_e5240_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) {
        let assign5340_e5236: f64 = (var_em - var_eav);
        let assign5340_e5238: f64 = (assign5340_e5236 / var_em);
        (assign5340_e5238, ((((var_em_dn0 - var_eav_dn0) * var_em) - (assign5340_e5236 * var_em_dn0)) / (var_em * var_em)), ((((var_em_dn1 - var_eav_dn1) * var_em) - (assign5340_e5236 * var_em_dn1)) / (var_em * var_em)), ((((var_em_dn3 - var_eav_dn3) * var_em) - (assign5340_e5236 * var_em_dn3)) / (var_em * var_em)), ((((var_em_dn4 - var_eav_dn4) * var_em) - (assign5340_e5236 * var_em_dn4)) / (var_em * var_em)), ((((var_em_dn5 - var_eav_dn5) * var_em) - (assign5340_e5236 * var_em_dn5)) / (var_em * var_em)), ((((var_em_dn6 - var_eav_dn6) * var_em) - (assign5340_e5236 * var_em_dn6)) / (var_em * var_em)), ((((var_em_dn7 - var_eav_dn7) * var_em) - (assign5340_e5236 * var_em_dn7)) / (var_em * var_em)), ((((var_em_dn8 - var_eav_dn8) * var_em) - (assign5340_e5236 * var_em_dn8)) / (var_em * var_em)), ((((var_em_dn9 - var_eav_dn9) * var_em) - (assign5340_e5236 * var_em_dn9)) / (var_em * var_em)), ((((var_em_dn10 - var_eav_dn10) * var_em) - (assign5340_e5236 * var_em_dn10)) / (var_em * var_em)),)
    } else {
        (var_emeav_em, var_emeav_em_dn0, var_emeav_em_dn1, var_emeav_em_dn3, var_emeav_em_dn4, var_emeav_em_dn5, var_emeav_em_dn6, var_emeav_em_dn7, var_emeav_em_dn8, var_emeav_em_dn9, var_emeav_em_dn10,)
    }
};
        var_emeav_em = assign5340_e5240;
        var_emeav_em_dn0 = assign5340_e5240_d_n0;
        var_emeav_em_dn1 = assign5340_e5240_d_n1;
        var_emeav_em_dn3 = assign5340_e5240_d_n3;
        var_emeav_em_dn4 = assign5340_e5240_d_n4;
        var_emeav_em_dn5 = assign5340_e5240_d_n5;
        var_emeav_em_dn6 = assign5340_e5240_d_n6;
        var_emeav_em_dn7 = assign5340_e5240_d_n7;
        var_emeav_em_dn8 = assign5340_e5240_d_n8;
        var_emeav_em_dn9 = assign5340_e5240_d_n9;
        var_emeav_em_dn10 = assign5340_e5240_d_n10;
        var_emeav_em_rv = 0.0;

        let assign5350_e5242: f64 = (var_emeav_em).abs();
        let assign5350_e5244: f64 = if assign5350_e5242 > 1e-7 { 1.0 } else { 0.0 };
        var_guard94 = assign5350_e5244;
        var_guard94_rv = 0.0;

        let (assign5360_e5261, assign5360_e5261_d_n0, assign5360_e5261_d_n1, assign5360_e5261_d_n3, assign5360_e5261_d_n4, assign5360_e5261_d_n5, assign5360_e5261_d_n6, assign5360_e5261_d_n7, assign5360_e5261_d_n8, assign5360_e5261_d_n9, assign5360_e5261_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard94 != 0.0)) {
        let assign5360_e5257: f64 = (0.5 * var_wd);
        let assign5360_e5259: f64 = (assign5360_e5257 / var_emeav_em);
        (assign5360_e5259, ((((0.5 * var_wd_dn0) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn0)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn1) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn1)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn3) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn3)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn4) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn4)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn5) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn5)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn6) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn6)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn7) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn7)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn8) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn8)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn9) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn9)) / (var_emeav_em * var_emeav_em)), ((((0.5 * var_wd_dn10) * var_emeav_em) - (assign5360_e5257 * var_emeav_em_dn10)) / (var_emeav_em * var_emeav_em)),)
    } else {
        (var_lambda, var_lambda_dn0, var_lambda_dn1, var_lambda_dn3, var_lambda_dn4, var_lambda_dn5, var_lambda_dn6, var_lambda_dn7, var_lambda_dn8, var_lambda_dn9, var_lambda_dn10,)
    }
};
        var_lambda = assign5360_e5261;
        var_lambda_dn0 = assign5360_e5261_d_n0;
        var_lambda_dn1 = assign5360_e5261_d_n1;
        var_lambda_dn3 = assign5360_e5261_d_n3;
        var_lambda_dn4 = assign5360_e5261_d_n4;
        var_lambda_dn5 = assign5360_e5261_d_n5;
        var_lambda_dn6 = assign5360_e5261_d_n6;
        var_lambda_dn7 = assign5360_e5261_d_n7;
        var_lambda_dn8 = assign5360_e5261_d_n8;
        var_lambda_dn9 = assign5360_e5261_d_n9;
        var_lambda_dn10 = assign5360_e5261_d_n10;
        var_lambda_rv = 0.0;

        *var_dedx0_slot = var_dedx0;
        *var_dedx0_rv_slot = var_dedx0_rv;
        *var_e0_slot = var_e0;
        *var_e0_dn0_slot = var_e0_dn0;
        *var_e0_dn1_slot = var_e0_dn1;
        *var_e0_dn10_slot = var_e0_dn10;
        *var_e0_dn3_slot = var_e0_dn3;
        *var_e0_dn4_slot = var_e0_dn4;
        *var_e0_dn5_slot = var_e0_dn5;
        *var_e0_dn6_slot = var_e0_dn6;
        *var_e0_dn7_slot = var_e0_dn7;
        *var_e0_dn8_slot = var_e0_dn8;
        *var_e0_dn9_slot = var_e0_dn9;
        *var_e0_rv_slot = var_e0_rv;
        *var_eav_slot = var_eav;
        *var_eav_dn0_slot = var_eav_dn0;
        *var_eav_dn1_slot = var_eav_dn1;
        *var_eav_dn10_slot = var_eav_dn10;
        *var_eav_dn3_slot = var_eav_dn3;
        *var_eav_dn4_slot = var_eav_dn4;
        *var_eav_dn5_slot = var_eav_dn5;
        *var_eav_dn6_slot = var_eav_dn6;
        *var_eav_dn7_slot = var_eav_dn7;
        *var_eav_dn8_slot = var_eav_dn8;
        *var_eav_dn9_slot = var_eav_dn9;
        *var_eav_rv_slot = var_eav_rv;
        *var_efi_slot = var_efi;
        *var_efi_rv_slot = var_efi_rv;
        *var_em_slot = var_em;
        *var_em_dn0_slot = var_em_dn0;
        *var_em_dn1_slot = var_em_dn1;
        *var_em_dn10_slot = var_em_dn10;
        *var_em_dn3_slot = var_em_dn3;
        *var_em_dn4_slot = var_em_dn4;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_dn9_slot = var_em_dn9;
        *var_em_rv_slot = var_em_rv;
        *var_emeav_em_slot = var_emeav_em;
        *var_emeav_em_dn0_slot = var_emeav_em_dn0;
        *var_emeav_em_dn1_slot = var_emeav_em_dn1;
        *var_emeav_em_dn10_slot = var_emeav_em_dn10;
        *var_emeav_em_dn3_slot = var_emeav_em_dn3;
        *var_emeav_em_dn4_slot = var_emeav_em_dn4;
        *var_emeav_em_dn5_slot = var_emeav_em_dn5;
        *var_emeav_em_dn6_slot = var_emeav_em_dn6;
        *var_emeav_em_dn7_slot = var_emeav_em_dn7;
        *var_emeav_em_dn8_slot = var_emeav_em_dn8;
        *var_emeav_em_dn9_slot = var_emeav_em_dn9;
        *var_emeav_em_rv_slot = var_emeav_em_rv;
        *var_ew_slot = var_ew;
        *var_ew_dn0_slot = var_ew_dn0;
        *var_ew_dn1_slot = var_ew_dn1;
        *var_ew_dn10_slot = var_ew_dn10;
        *var_ew_dn3_slot = var_ew_dn3;
        *var_ew_dn4_slot = var_ew_dn4;
        *var_ew_dn5_slot = var_ew_dn5;
        *var_ew_dn6_slot = var_ew_dn6;
        *var_ew_dn7_slot = var_ew_dn7;
        *var_ew_dn8_slot = var_ew_dn8;
        *var_ew_dn9_slot = var_ew_dn9;
        *var_ew_rv_slot = var_ew_rv;
        *var_expl_slot = var_expl;
        *var_expl_rv_slot = var_expl_rv;
        *var_expmm1_slot = var_expmm1;
        *var_expmm1_dn0_slot = var_expmm1_dn0;
        *var_expmm1_dn1_slot = var_expmm1_dn1;
        *var_expmm1_dn10_slot = var_expmm1_dn10;
        *var_expmm1_dn3_slot = var_expmm1_dn3;
        *var_expmm1_dn4_slot = var_expmm1_dn4;
        *var_expmm1_dn5_slot = var_expmm1_dn5;
        *var_expmm1_dn6_slot = var_expmm1_dn6;
        *var_expmm1_dn7_slot = var_expmm1_dn7;
        *var_expmm1_dn8_slot = var_expmm1_dn8;
        *var_expmm1_dn9_slot = var_expmm1_dn9;
        *var_expmm1_rv_slot = var_expmm1_rv;
        *var_gem_slot = var_gem;
        *var_gem_dn0_slot = var_gem_dn0;
        *var_gem_dn1_slot = var_gem_dn1;
        *var_gem_dn10_slot = var_gem_dn10;
        *var_gem_dn3_slot = var_gem_dn3;
        *var_gem_dn4_slot = var_gem_dn4;
        *var_gem_dn5_slot = var_gem_dn5;
        *var_gem_dn6_slot = var_gem_dn6;
        *var_gem_dn7_slot = var_gem_dn7;
        *var_gem_dn8_slot = var_gem_dn8;
        *var_gem_dn9_slot = var_gem_dn9;
        *var_gem_rv_slot = var_gem_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_lambda_slot = var_lambda;
        *var_lambda_dn0_slot = var_lambda_dn0;
        *var_lambda_dn1_slot = var_lambda_dn1;
        *var_lambda_dn10_slot = var_lambda_dn10;
        *var_lambda_dn3_slot = var_lambda_dn3;
        *var_lambda_dn4_slot = var_lambda_dn4;
        *var_lambda_dn5_slot = var_lambda_dn5;
        *var_lambda_dn6_slot = var_lambda_dn6;
        *var_lambda_dn7_slot = var_lambda_dn7;
        *var_lambda_dn8_slot = var_lambda_dn8;
        *var_lambda_dn9_slot = var_lambda_dn9;
        *var_lambda_rv_slot = var_lambda_rv;
        *var_shw_slot = var_shw;
        *var_shw_dn0_slot = var_shw_dn0;
        *var_shw_dn1_slot = var_shw_dn1;
        *var_shw_dn10_slot = var_shw_dn10;
        *var_shw_dn3_slot = var_shw_dn3;
        *var_shw_dn4_slot = var_shw_dn4;
        *var_shw_dn5_slot = var_shw_dn5;
        *var_shw_dn6_slot = var_shw_dn6;
        *var_shw_dn7_slot = var_shw_dn7;
        *var_shw_dn8_slot = var_shw_dn8;
        *var_shw_dn9_slot = var_shw_dn9;
        *var_shw_rv_slot = var_shw_rv;
        *var_sqr_arg_slot = var_sqr_arg;
        *var_sqr_arg_dn0_slot = var_sqr_arg_dn0;
        *var_sqr_arg_dn1_slot = var_sqr_arg_dn1;
        *var_sqr_arg_dn10_slot = var_sqr_arg_dn10;
        *var_sqr_arg_dn3_slot = var_sqr_arg_dn3;
        *var_sqr_arg_dn4_slot = var_sqr_arg_dn4;
        *var_sqr_arg_dn5_slot = var_sqr_arg_dn5;
        *var_sqr_arg_dn6_slot = var_sqr_arg_dn6;
        *var_sqr_arg_dn7_slot = var_sqr_arg_dn7;
        *var_sqr_arg_dn8_slot = var_sqr_arg_dn8;
        *var_sqr_arg_dn9_slot = var_sqr_arg_dn9;
        *var_sqr_arg_rv_slot = var_sqr_arg_rv;
        *var_wd_slot = var_wd;
        *var_wd_dn0_slot = var_wd_dn0;
        *var_wd_dn1_slot = var_wd_dn1;
        *var_wd_dn10_slot = var_wd_dn10;
        *var_wd_dn3_slot = var_wd_dn3;
        *var_wd_dn4_slot = var_wd_dn4;
        *var_wd_dn5_slot = var_wd_dn5;
        *var_wd_dn6_slot = var_wd_dn6;
        *var_wd_dn7_slot = var_wd_dn7;
        *var_wd_dn8_slot = var_wd_dn8;
        *var_wd_dn9_slot = var_wd_dn9;
        *var_wd_rv_slot = var_wd_rv;
        *var_weff_slot = var_weff;
        *var_weff_dn0_slot = var_weff_dn0;
        *var_weff_dn1_slot = var_weff_dn1;
        *var_weff_dn10_slot = var_weff_dn10;
        *var_weff_dn3_slot = var_weff_dn3;
        *var_weff_dn4_slot = var_weff_dn4;
        *var_weff_dn5_slot = var_weff_dn5;
        *var_weff_dn6_slot = var_weff_dn6;
        *var_weff_dn7_slot = var_weff_dn7;
        *var_weff_dn8_slot = var_weff_dn8;
        *var_weff_dn9_slot = var_weff_dn9;
        *var_weff_rv_slot = var_weff_rv;
        *var_xd_slot = var_xd;
        *var_xd_dn0_slot = var_xd_dn0;
        *var_xd_dn1_slot = var_xd_dn1;
        *var_xd_dn10_slot = var_xd_dn10;
        *var_xd_dn3_slot = var_xd_dn3;
        *var_xd_dn4_slot = var_xd_dn4;
        *var_xd_dn5_slot = var_xd_dn5;
        *var_xd_dn6_slot = var_xd_dn6;
        *var_xd_dn7_slot = var_xd_dn7;
        *var_xd_dn8_slot = var_xd_dn8;
        *var_xd_dn9_slot = var_xd_dn9;
        *var_xd_rv_slot = var_xd_rv;
        *var_xi_w1_slot = var_xi_w1;
        *var_xi_w1_dn0_slot = var_xi_w1_dn0;
        *var_xi_w1_dn1_slot = var_xi_w1_dn1;
        *var_xi_w1_dn10_slot = var_xi_w1_dn10;
        *var_xi_w1_dn3_slot = var_xi_w1_dn3;
        *var_xi_w1_dn4_slot = var_xi_w1_dn4;
        *var_xi_w1_dn5_slot = var_xi_w1_dn5;
        *var_xi_w1_dn6_slot = var_xi_w1_dn6;
        *var_xi_w1_dn7_slot = var_xi_w1_dn7;
        *var_xi_w1_dn8_slot = var_xi_w1_dn8;
        *var_xi_w1_dn9_slot = var_xi_w1_dn9;
        *var_xi_w1_rv_slot = var_xi_w1_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_an: f64,
        var_bavl_t: f64,
        var_bavl_t_dn0: f64,
        var_bavl_t_dn1: f64,
        var_bavl_t_dn10: f64,
        var_bavl_t_dn3: f64,
        var_bavl_t_dn4: f64,
        var_bavl_t_dn5: f64,
        var_bavl_t_dn6: f64,
        var_bavl_t_dn7: f64,
        var_bavl_t_dn8: f64,
        var_bavl_t_dn9: f64,
        var_bnt: f64,
        var_bnt_dn3: f64,
        var_cje_t: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_em: f64,
        var_em_dn0: f64,
        var_em_dn1: f64,
        var_em_dn10: f64,
        var_em_dn3: f64,
        var_em_dn4: f64,
        var_em_dn5: f64,
        var_em_dn6: f64,
        var_em_dn7: f64,
        var_em_dn8: f64,
        var_em_dn9: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_guard94: f64,
        var_ibi_t: f64,
        var_ibi_t_dn3: f64,
        var_in_: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_is_t: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn10: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_is_t_dn7: f64,
        var_is_t_dn8: f64,
        var_is_t_dn9: f64,
        var_lambda: f64,
        var_lambda_dn0: f64,
        var_lambda_dn1: f64,
        var_lambda_dn10: f64,
        var_lambda_dn3: f64,
        var_lambda_dn4: f64,
        var_lambda_dn5: f64,
        var_lambda_dn6: f64,
        var_lambda_dn7: f64,
        var_lambda_dn8: f64,
        var_lambda_dn9: f64,
        var_qbi: f64,
        var_qbi_dn0: f64,
        var_qbi_dn1: f64,
        var_qbi_dn10: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_rb2: f64,
        var_rb2_dn0: f64,
        var_rb2_dn1: f64,
        var_rb2_dn10: f64,
        var_rb2_dn3: f64,
        var_rb2_dn4: f64,
        var_rb2_dn5: f64,
        var_rb2_dn6: f64,
        var_rb2_dn7: f64,
        var_rb2_dn8: f64,
        var_rb2_dn9: f64,
        var_rbc_t: f64,
        var_rbc_t_dn3: f64,
        var_re_t: f64,
        var_re_t_dn3: f64,
        var_vb1e1: f64,
        var_vb1e1_dn4: f64,
        var_vb1e1_dn5: f64,
        var_vb2c1: f64,
        var_vb2c1_dn6: f64,
        var_vb2c1_dn7: f64,
        var_vfe: f64,
        var_vfe_dn0: f64,
        var_vfe_dn1: f64,
        var_vfe_dn10: f64,
        var_vfe_dn3: f64,
        var_vfe_dn4: f64,
        var_vfe_dn5: f64,
        var_vfe_dn6: f64,
        var_vfe_dn7: f64,
        var_vfe_dn8: f64,
        var_vfe_dn9: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_vte: f64,
        var_vte_dn0: f64,
        var_vte_dn1: f64,
        var_vte_dn10: f64,
        var_vte_dn3: f64,
        var_vte_dn4: f64,
        var_vte_dn5: f64,
        var_vte_dn6: f64,
        var_vte_dn7: f64,
        var_vte_dn8: f64,
        var_vte_dn9: f64,
        var_weff: f64,
        var_weff_dn0: f64,
        var_weff_dn1: f64,
        var_weff_dn10: f64,
        var_weff_dn3: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn7: f64,
        var_weff_dn8: f64,
        var_weff_dn9: f64,
        var_dxa_slot: &mut f64,
        var_dxa_dn0_slot: &mut f64,
        var_dxa_dn1_slot: &mut f64,
        var_dxa_dn10_slot: &mut f64,
        var_dxa_dn3_slot: &mut f64,
        var_dxa_dn4_slot: &mut f64,
        var_dxa_dn5_slot: &mut f64,
        var_dxa_dn6_slot: &mut f64,
        var_dxa_dn7_slot: &mut f64,
        var_dxa_dn8_slot: &mut f64,
        var_dxa_dn9_slot: &mut f64,
        var_dxa_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_expmm1_slot: &mut f64,
        var_expmm1_dn0_slot: &mut f64,
        var_expmm1_dn1_slot: &mut f64,
        var_expmm1_dn10_slot: &mut f64,
        var_expmm1_dn3_slot: &mut f64,
        var_expmm1_dn4_slot: &mut f64,
        var_expmm1_dn5_slot: &mut f64,
        var_expmm1_dn6_slot: &mut f64,
        var_expmm1_dn7_slot: &mut f64,
        var_expmm1_dn8_slot: &mut f64,
        var_expmm1_dn9_slot: &mut f64,
        var_expmm1_rv_slot: &mut f64,
        var_gem_slot: &mut f64,
        var_gem_dn0_slot: &mut f64,
        var_gem_dn1_slot: &mut f64,
        var_gem_dn10_slot: &mut f64,
        var_gem_dn3_slot: &mut f64,
        var_gem_dn4_slot: &mut f64,
        var_gem_dn5_slot: &mut f64,
        var_gem_dn6_slot: &mut f64,
        var_gem_dn7_slot: &mut f64,
        var_gem_dn8_slot: &mut f64,
        var_gem_dn9_slot: &mut f64,
        var_gem_rv_slot: &mut f64,
        var_gmax_slot: &mut f64,
        var_gmax_dn0_slot: &mut f64,
        var_gmax_dn1_slot: &mut f64,
        var_gmax_dn10_slot: &mut f64,
        var_gmax_dn3_slot: &mut f64,
        var_gmax_dn4_slot: &mut f64,
        var_gmax_dn5_slot: &mut f64,
        var_gmax_dn6_slot: &mut f64,
        var_gmax_dn7_slot: &mut f64,
        var_gmax_dn8_slot: &mut f64,
        var_gmax_dn9_slot: &mut f64,
        var_gmax_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_in_shift_ihcavl_slot: &mut f64,
        var_in_shift_ihcavl_dn0_slot: &mut f64,
        var_in_shift_ihcavl_dn1_slot: &mut f64,
        var_in_shift_ihcavl_dn10_slot: &mut f64,
        var_in_shift_ihcavl_dn3_slot: &mut f64,
        var_in_shift_ihcavl_dn4_slot: &mut f64,
        var_in_shift_ihcavl_dn5_slot: &mut f64,
        var_in_shift_ihcavl_dn6_slot: &mut f64,
        var_in_shift_ihcavl_dn7_slot: &mut f64,
        var_in_shift_ihcavl_dn8_slot: &mut f64,
        var_in_shift_ihcavl_dn9_slot: &mut f64,
        var_in_shift_ihcavl_rv_slot: &mut f64,
        var_in_shift_n_slot: &mut f64,
        var_in_shift_n_dn0_slot: &mut f64,
        var_in_shift_n_dn1_slot: &mut f64,
        var_in_shift_n_dn10_slot: &mut f64,
        var_in_shift_n_dn3_slot: &mut f64,
        var_in_shift_n_dn4_slot: &mut f64,
        var_in_shift_n_dn5_slot: &mut f64,
        var_in_shift_n_dn6_slot: &mut f64,
        var_in_shift_n_dn7_slot: &mut f64,
        var_in_shift_n_dn8_slot: &mut f64,
        var_in_shift_n_dn9_slot: &mut f64,
        var_in_shift_n_rv_slot: &mut f64,
        var_qte_slot: &mut f64,
        var_qte_dn0_slot: &mut f64,
        var_qte_dn1_slot: &mut f64,
        var_qte_dn10_slot: &mut f64,
        var_qte_dn3_slot: &mut f64,
        var_qte_dn4_slot: &mut f64,
        var_qte_dn5_slot: &mut f64,
        var_qte_dn6_slot: &mut f64,
        var_qte_dn7_slot: &mut f64,
        var_qte_dn8_slot: &mut f64,
        var_qte_dn9_slot: &mut f64,
        var_qte_rv_slot: &mut f64,
        var_vdep_slot: &mut f64,
        var_vdep_dn0_slot: &mut f64,
        var_vdep_dn1_slot: &mut f64,
        var_vdep_dn10_slot: &mut f64,
        var_vdep_dn3_slot: &mut f64,
        var_vdep_dn4_slot: &mut f64,
        var_vdep_dn5_slot: &mut f64,
        var_vdep_dn6_slot: &mut f64,
        var_vdep_dn7_slot: &mut f64,
        var_vdep_dn8_slot: &mut f64,
        var_vdep_dn9_slot: &mut f64,
        var_vdep_rv_slot: &mut f64,
        var_vdeptmp_slot: &mut f64,
        var_vdeptmp_dn0_slot: &mut f64,
        var_vdeptmp_dn1_slot: &mut f64,
        var_vdeptmp_dn10_slot: &mut f64,
        var_vdeptmp_dn3_slot: &mut f64,
        var_vdeptmp_dn4_slot: &mut f64,
        var_vdeptmp_dn5_slot: &mut f64,
        var_vdeptmp_dn6_slot: &mut f64,
        var_vdeptmp_dn7_slot: &mut f64,
        var_vdeptmp_dn8_slot: &mut f64,
        var_vdeptmp_dn9_slot: &mut f64,
        var_vdeptmp_rv_slot: &mut f64,
        var_vje_s_slot: &mut f64,
        var_vje_s_dn0_slot: &mut f64,
        var_vje_s_dn1_slot: &mut f64,
        var_vje_s_dn10_slot: &mut f64,
        var_vje_s_dn3_slot: &mut f64,
        var_vje_s_dn4_slot: &mut f64,
        var_vje_s_dn5_slot: &mut f64,
        var_vje_s_dn6_slot: &mut f64,
        var_vje_s_dn7_slot: &mut f64,
        var_vje_s_dn8_slot: &mut f64,
        var_vje_s_dn9_slot: &mut f64,
        var_vje_s_rv_slot: &mut f64,
    ) {
        let mut var_dxa: f64 = *var_dxa_slot;
        let mut var_dxa_dn0: f64 = *var_dxa_dn0_slot;
        let mut var_dxa_dn1: f64 = *var_dxa_dn1_slot;
        let mut var_dxa_dn10: f64 = *var_dxa_dn10_slot;
        let mut var_dxa_dn3: f64 = *var_dxa_dn3_slot;
        let mut var_dxa_dn4: f64 = *var_dxa_dn4_slot;
        let mut var_dxa_dn5: f64 = *var_dxa_dn5_slot;
        let mut var_dxa_dn6: f64 = *var_dxa_dn6_slot;
        let mut var_dxa_dn7: f64 = *var_dxa_dn7_slot;
        let mut var_dxa_dn8: f64 = *var_dxa_dn8_slot;
        let mut var_dxa_dn9: f64 = *var_dxa_dn9_slot;
        let mut var_dxa_rv: f64 = *var_dxa_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_expmm1: f64 = *var_expmm1_slot;
        let mut var_expmm1_dn0: f64 = *var_expmm1_dn0_slot;
        let mut var_expmm1_dn1: f64 = *var_expmm1_dn1_slot;
        let mut var_expmm1_dn10: f64 = *var_expmm1_dn10_slot;
        let mut var_expmm1_dn3: f64 = *var_expmm1_dn3_slot;
        let mut var_expmm1_dn4: f64 = *var_expmm1_dn4_slot;
        let mut var_expmm1_dn5: f64 = *var_expmm1_dn5_slot;
        let mut var_expmm1_dn6: f64 = *var_expmm1_dn6_slot;
        let mut var_expmm1_dn7: f64 = *var_expmm1_dn7_slot;
        let mut var_expmm1_dn8: f64 = *var_expmm1_dn8_slot;
        let mut var_expmm1_dn9: f64 = *var_expmm1_dn9_slot;
        let mut var_expmm1_rv: f64 = *var_expmm1_rv_slot;
        let mut var_gem: f64 = *var_gem_slot;
        let mut var_gem_dn0: f64 = *var_gem_dn0_slot;
        let mut var_gem_dn1: f64 = *var_gem_dn1_slot;
        let mut var_gem_dn10: f64 = *var_gem_dn10_slot;
        let mut var_gem_dn3: f64 = *var_gem_dn3_slot;
        let mut var_gem_dn4: f64 = *var_gem_dn4_slot;
        let mut var_gem_dn5: f64 = *var_gem_dn5_slot;
        let mut var_gem_dn6: f64 = *var_gem_dn6_slot;
        let mut var_gem_dn7: f64 = *var_gem_dn7_slot;
        let mut var_gem_dn8: f64 = *var_gem_dn8_slot;
        let mut var_gem_dn9: f64 = *var_gem_dn9_slot;
        let mut var_gem_rv: f64 = *var_gem_rv_slot;
        let mut var_gmax: f64 = *var_gmax_slot;
        let mut var_gmax_dn0: f64 = *var_gmax_dn0_slot;
        let mut var_gmax_dn1: f64 = *var_gmax_dn1_slot;
        let mut var_gmax_dn10: f64 = *var_gmax_dn10_slot;
        let mut var_gmax_dn3: f64 = *var_gmax_dn3_slot;
        let mut var_gmax_dn4: f64 = *var_gmax_dn4_slot;
        let mut var_gmax_dn5: f64 = *var_gmax_dn5_slot;
        let mut var_gmax_dn6: f64 = *var_gmax_dn6_slot;
        let mut var_gmax_dn7: f64 = *var_gmax_dn7_slot;
        let mut var_gmax_dn8: f64 = *var_gmax_dn8_slot;
        let mut var_gmax_dn9: f64 = *var_gmax_dn9_slot;
        let mut var_gmax_rv: f64 = *var_gmax_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_in_shift_ihcavl: f64 = *var_in_shift_ihcavl_slot;
        let mut var_in_shift_ihcavl_dn0: f64 = *var_in_shift_ihcavl_dn0_slot;
        let mut var_in_shift_ihcavl_dn1: f64 = *var_in_shift_ihcavl_dn1_slot;
        let mut var_in_shift_ihcavl_dn10: f64 = *var_in_shift_ihcavl_dn10_slot;
        let mut var_in_shift_ihcavl_dn3: f64 = *var_in_shift_ihcavl_dn3_slot;
        let mut var_in_shift_ihcavl_dn4: f64 = *var_in_shift_ihcavl_dn4_slot;
        let mut var_in_shift_ihcavl_dn5: f64 = *var_in_shift_ihcavl_dn5_slot;
        let mut var_in_shift_ihcavl_dn6: f64 = *var_in_shift_ihcavl_dn6_slot;
        let mut var_in_shift_ihcavl_dn7: f64 = *var_in_shift_ihcavl_dn7_slot;
        let mut var_in_shift_ihcavl_dn8: f64 = *var_in_shift_ihcavl_dn8_slot;
        let mut var_in_shift_ihcavl_dn9: f64 = *var_in_shift_ihcavl_dn9_slot;
        let mut var_in_shift_ihcavl_rv: f64 = *var_in_shift_ihcavl_rv_slot;
        let mut var_in_shift_n: f64 = *var_in_shift_n_slot;
        let mut var_in_shift_n_dn0: f64 = *var_in_shift_n_dn0_slot;
        let mut var_in_shift_n_dn1: f64 = *var_in_shift_n_dn1_slot;
        let mut var_in_shift_n_dn10: f64 = *var_in_shift_n_dn10_slot;
        let mut var_in_shift_n_dn3: f64 = *var_in_shift_n_dn3_slot;
        let mut var_in_shift_n_dn4: f64 = *var_in_shift_n_dn4_slot;
        let mut var_in_shift_n_dn5: f64 = *var_in_shift_n_dn5_slot;
        let mut var_in_shift_n_dn6: f64 = *var_in_shift_n_dn6_slot;
        let mut var_in_shift_n_dn7: f64 = *var_in_shift_n_dn7_slot;
        let mut var_in_shift_n_dn8: f64 = *var_in_shift_n_dn8_slot;
        let mut var_in_shift_n_dn9: f64 = *var_in_shift_n_dn9_slot;
        let mut var_in_shift_n_rv: f64 = *var_in_shift_n_rv_slot;
        let mut var_qte: f64 = *var_qte_slot;
        let mut var_qte_dn0: f64 = *var_qte_dn0_slot;
        let mut var_qte_dn1: f64 = *var_qte_dn1_slot;
        let mut var_qte_dn10: f64 = *var_qte_dn10_slot;
        let mut var_qte_dn3: f64 = *var_qte_dn3_slot;
        let mut var_qte_dn4: f64 = *var_qte_dn4_slot;
        let mut var_qte_dn5: f64 = *var_qte_dn5_slot;
        let mut var_qte_dn6: f64 = *var_qte_dn6_slot;
        let mut var_qte_dn7: f64 = *var_qte_dn7_slot;
        let mut var_qte_dn8: f64 = *var_qte_dn8_slot;
        let mut var_qte_dn9: f64 = *var_qte_dn9_slot;
        let mut var_qte_rv: f64 = *var_qte_rv_slot;
        let mut var_vdep: f64 = *var_vdep_slot;
        let mut var_vdep_dn0: f64 = *var_vdep_dn0_slot;
        let mut var_vdep_dn1: f64 = *var_vdep_dn1_slot;
        let mut var_vdep_dn10: f64 = *var_vdep_dn10_slot;
        let mut var_vdep_dn3: f64 = *var_vdep_dn3_slot;
        let mut var_vdep_dn4: f64 = *var_vdep_dn4_slot;
        let mut var_vdep_dn5: f64 = *var_vdep_dn5_slot;
        let mut var_vdep_dn6: f64 = *var_vdep_dn6_slot;
        let mut var_vdep_dn7: f64 = *var_vdep_dn7_slot;
        let mut var_vdep_dn8: f64 = *var_vdep_dn8_slot;
        let mut var_vdep_dn9: f64 = *var_vdep_dn9_slot;
        let mut var_vdep_rv: f64 = *var_vdep_rv_slot;
        let mut var_vdeptmp: f64 = *var_vdeptmp_slot;
        let mut var_vdeptmp_dn0: f64 = *var_vdeptmp_dn0_slot;
        let mut var_vdeptmp_dn1: f64 = *var_vdeptmp_dn1_slot;
        let mut var_vdeptmp_dn10: f64 = *var_vdeptmp_dn10_slot;
        let mut var_vdeptmp_dn3: f64 = *var_vdeptmp_dn3_slot;
        let mut var_vdeptmp_dn4: f64 = *var_vdeptmp_dn4_slot;
        let mut var_vdeptmp_dn5: f64 = *var_vdeptmp_dn5_slot;
        let mut var_vdeptmp_dn6: f64 = *var_vdeptmp_dn6_slot;
        let mut var_vdeptmp_dn7: f64 = *var_vdeptmp_dn7_slot;
        let mut var_vdeptmp_dn8: f64 = *var_vdeptmp_dn8_slot;
        let mut var_vdeptmp_dn9: f64 = *var_vdeptmp_dn9_slot;
        let mut var_vdeptmp_rv: f64 = *var_vdeptmp_rv_slot;
        let mut var_vje_s: f64 = *var_vje_s_slot;
        let mut var_vje_s_dn0: f64 = *var_vje_s_dn0_slot;
        let mut var_vje_s_dn1: f64 = *var_vje_s_dn1_slot;
        let mut var_vje_s_dn10: f64 = *var_vje_s_dn10_slot;
        let mut var_vje_s_dn3: f64 = *var_vje_s_dn3_slot;
        let mut var_vje_s_dn4: f64 = *var_vje_s_dn4_slot;
        let mut var_vje_s_dn5: f64 = *var_vje_s_dn5_slot;
        let mut var_vje_s_dn6: f64 = *var_vje_s_dn6_slot;
        let mut var_vje_s_dn7: f64 = *var_vje_s_dn7_slot;
        let mut var_vje_s_dn8: f64 = *var_vje_s_dn8_slot;
        let mut var_vje_s_dn9: f64 = *var_vje_s_dn9_slot;
        let mut var_vje_s_rv: f64 = *var_vje_s_rv_slot;

        let (assign5370_e5298, assign5370_e5298_d_n0, assign5370_e5298_d_n1, assign5370_e5298_d_n3, assign5370_e5298_d_n4, assign5370_e5298_d_n5, assign5370_e5298_d_n6, assign5370_e5298_d_n7, assign5370_e5298_d_n8, assign5370_e5298_d_n9, assign5370_e5298_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard94 != 0.0)) {
        let assign5370_e5274: f64 = (var_an / var_bnt);
        let assign5370_e5276: f64 = (assign5370_e5274 * var_em);
        let assign5370_e5278: f64 = (assign5370_e5276 * var_lambda);
        let assign5370_e5280: f64 = (-var_bnt);
        let assign5370_e5282: f64 = (assign5370_e5280 / var_em);
        let assign5370_e5283: f64 = (assign5370_e5282).exp();
        let assign5370_e5285: f64 = (-var_bnt);
        let assign5370_e5287: f64 = (assign5370_e5285 / var_em);
        let assign5370_e5291: f64 = (var_weff / var_lambda);
        let assign5370_e5292: f64 = (1.0 + assign5370_e5291);
        let assign5370_e5293: f64 = (assign5370_e5287 * assign5370_e5292);
        let assign5370_e5294: f64 = (assign5370_e5293).exp();
        let assign5370_e5295: f64 = (assign5370_e5283 - assign5370_e5294);
        let assign5370_e5296: f64 = (assign5370_e5278 * assign5370_e5295);
        (assign5370_e5296, (((((assign5370_e5274 * var_em_dn0) * var_lambda) + (assign5370_e5276 * var_lambda_dn0)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn0) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn0) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn0 * var_lambda) - (var_weff * var_lambda_dn0)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn1) * var_lambda) + (assign5370_e5276 * var_lambda_dn1)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn1) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn1) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn1 * var_lambda) - (var_weff * var_lambda_dn1)) / (var_lambda * var_lambda)))))))), (((((((-((var_an * var_bnt_dn3) / (var_bnt * var_bnt))) * var_em) + (assign5370_e5274 * var_em_dn3)) * var_lambda) + (assign5370_e5276 * var_lambda_dn3)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * ((((-var_bnt_dn3) * var_em) - (assign5370_e5280 * var_em_dn3)) / (var_em * var_em))) - (assign5370_e5294 * ((((((-var_bnt_dn3) * var_em) - (assign5370_e5285 * var_em_dn3)) / (var_em * var_em)) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn3 * var_lambda) - (var_weff * var_lambda_dn3)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn4) * var_lambda) + (assign5370_e5276 * var_lambda_dn4)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn4) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn4) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn4 * var_lambda) - (var_weff * var_lambda_dn4)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn5) * var_lambda) + (assign5370_e5276 * var_lambda_dn5)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn5) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn5) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn5 * var_lambda) - (var_weff * var_lambda_dn5)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn6) * var_lambda) + (assign5370_e5276 * var_lambda_dn6)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn6) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn6) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn6 * var_lambda) - (var_weff * var_lambda_dn6)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn7) * var_lambda) + (assign5370_e5276 * var_lambda_dn7)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn7) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn7) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn7 * var_lambda) - (var_weff * var_lambda_dn7)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn8) * var_lambda) + (assign5370_e5276 * var_lambda_dn8)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn8) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn8) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn8 * var_lambda) - (var_weff * var_lambda_dn8)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn9) * var_lambda) + (assign5370_e5276 * var_lambda_dn9)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn9) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn9) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn9 * var_lambda) - (var_weff * var_lambda_dn9)) / (var_lambda * var_lambda)))))))), (((((assign5370_e5274 * var_em_dn10) * var_lambda) + (assign5370_e5276 * var_lambda_dn10)) * assign5370_e5295) + (assign5370_e5278 * ((assign5370_e5283 * (-((assign5370_e5280 * var_em_dn10) / (var_em * var_em)))) - (assign5370_e5294 * (((-((assign5370_e5285 * var_em_dn10) / (var_em * var_em))) * assign5370_e5292) + (assign5370_e5287 * (((var_weff_dn10 * var_lambda) - (var_weff * var_lambda_dn10)) / (var_lambda * var_lambda)))))))),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5370_e5298;
        var_gem_dn0 = assign5370_e5298_d_n0;
        var_gem_dn1 = assign5370_e5298_d_n1;
        var_gem_dn3 = assign5370_e5298_d_n3;
        var_gem_dn4 = assign5370_e5298_d_n4;
        var_gem_dn5 = assign5370_e5298_d_n5;
        var_gem_dn6 = assign5370_e5298_d_n6;
        var_gem_dn7 = assign5370_e5298_d_n7;
        var_gem_dn8 = assign5370_e5298_d_n8;
        var_gem_dn9 = assign5370_e5298_d_n9;
        var_gem_dn10 = assign5370_e5298_d_n10;
        var_gem_rv = 0.0;

        let (assign5380_e5320, assign5380_e5320_d_n0, assign5380_e5320_d_n1, assign5380_e5320_d_n3, assign5380_e5320_d_n4, assign5380_e5320_d_n5, assign5380_e5320_d_n6, assign5380_e5320_d_n7, assign5380_e5320_d_n8, assign5380_e5320_d_n9, assign5380_e5320_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 != 0.0)) && (var_guard91 != 0.0)) && (var_guard94 == 0.0)) {
        let assign5380_e5312: f64 = (var_an * var_weff);
        let assign5380_e5314: f64 = (-var_bnt);
        let assign5380_e5316: f64 = (assign5380_e5314 / var_em);
        let assign5380_e5317: f64 = (assign5380_e5316).exp();
        let assign5380_e5318: f64 = (assign5380_e5312 * assign5380_e5317);
        (assign5380_e5318, (((var_an * var_weff_dn0) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn0) / (var_em * var_em)))))), (((var_an * var_weff_dn1) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn1) / (var_em * var_em)))))), (((var_an * var_weff_dn3) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * ((((-var_bnt_dn3) * var_em) - (assign5380_e5314 * var_em_dn3)) / (var_em * var_em))))), (((var_an * var_weff_dn4) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn4) / (var_em * var_em)))))), (((var_an * var_weff_dn5) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn5) / (var_em * var_em)))))), (((var_an * var_weff_dn6) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn6) / (var_em * var_em)))))), (((var_an * var_weff_dn7) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn7) / (var_em * var_em)))))), (((var_an * var_weff_dn8) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn8) / (var_em * var_em)))))), (((var_an * var_weff_dn9) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn9) / (var_em * var_em)))))), (((var_an * var_weff_dn10) * assign5380_e5317) + (assign5380_e5312 * (assign5380_e5317 * (-((assign5380_e5314 * var_em_dn10) / (var_em * var_em)))))),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5380_e5320;
        var_gem_dn0 = assign5380_e5320_d_n0;
        var_gem_dn1 = assign5380_e5320_d_n1;
        var_gem_dn3 = assign5380_e5320_d_n3;
        var_gem_dn4 = assign5380_e5320_d_n4;
        var_gem_dn5 = assign5380_e5320_d_n5;
        var_gem_dn6 = assign5380_e5320_d_n6;
        var_gem_dn7 = assign5380_e5320_d_n7;
        var_gem_dn8 = assign5380_e5320_d_n8;
        var_gem_dn9 = assign5380_e5320_d_n9;
        var_gem_dn10 = assign5380_e5320_d_n10;
        var_gem_rv = 0.0;

        let assign5390_e5323: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        var_guard95 = assign5390_e5323;
        var_guard95_rv = 0.0;

        let assign5400_e5326: f64 = if var_vb2c1 < p.p43 { 1.0 } else { 0.0 };
        var_guard96 = assign5400_e5326;
        var_guard96_rv = 0.0;

        let (assign5410_e5354, assign5410_e5354_d_n0, assign5410_e5354_d_n1, assign5410_e5354_d_n3, assign5410_e5354_d_n4, assign5410_e5354_d_n5, assign5410_e5354_d_n6, assign5410_e5354_d_n7, assign5410_e5354_d_n8, assign5410_e5354_d_n9, assign5410_e5354_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) {
        let assign5410_e5340: f64 = (p.p43 - var_vb2c1);
        let assign5410_e5342: f64 = (assign5410_e5340).powf(p.p40);
        let assign5410_e5347: f64 = (p.p47 + var_in_);
        let assign5410_e5348: f64 = (var_in_ / assign5410_e5347);
        let assign5410_e5349: f64 = (1.0 - assign5410_e5348);
        let assign5410_e5351: f64 = (assign5410_e5349).powf(p.p48);
        let assign5410_e5352: f64 = (assign5410_e5342 * assign5410_e5351);
        (assign5410_e5352, (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn0 * assign5410_e5347) - (var_in_ * var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn0 * assign5410_e5347) - (var_in_ * var_in__dn0)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn1 * assign5410_e5347) - (var_in_ * var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn1 * assign5410_e5347) - (var_in_ * var_in__dn1)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn3 * assign5410_e5347) - (var_in_ * var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn3 * assign5410_e5347) - (var_in_ * var_in__dn3)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn4 * assign5410_e5347) - (var_in_ * var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn4 * assign5410_e5347) - (var_in_ * var_in__dn4)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn5 * assign5410_e5347) - (var_in_ * var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn5 * assign5410_e5347) - (var_in_ * var_in__dn5)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-var_vb2c1_dn6))) } } else { (assign5410_e5342 * (p.p40 * ((-var_vb2c1_dn6) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn6 * assign5410_e5347) - (var_in_ * var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn6 * assign5410_e5347) - (var_in_ * var_in__dn6)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), ((if 0.0 == 0.0 && ((p.p40) as f64).is_finite() && ((p.p40) as f64).fract() == 0.0 { if p.p40 == 0.0 { 0.0 } else { (p.p40 * ((assign5410_e5340).powf(p.p40 - 1.0) * (-var_vb2c1_dn7))) } } else { (assign5410_e5342 * (p.p40 * ((-var_vb2c1_dn7) / assign5410_e5340))) } * assign5410_e5351) + (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn7 * assign5410_e5347) - (var_in_ * var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn7 * assign5410_e5347) - (var_in_ * var_in__dn7)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) })), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn8 * assign5410_e5347) - (var_in_ * var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn8 * assign5410_e5347) - (var_in_ * var_in__dn8)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn9 * assign5410_e5347) - (var_in_ * var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn9 * assign5410_e5347) - (var_in_ * var_in__dn9)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }), (assign5410_e5342 * if 0.0 == 0.0 && ((p.p48) as f64).is_finite() && ((p.p48) as f64).fract() == 0.0 { if p.p48 == 0.0 { 0.0 } else { (p.p48 * ((assign5410_e5349).powf(p.p48 - 1.0) * (-(((var_in__dn10 * assign5410_e5347) - (var_in_ * var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))))) } } else { (assign5410_e5351 * (p.p48 * ((-(((var_in__dn10 * assign5410_e5347) - (var_in_ * var_in__dn10)) / (assign5410_e5347 * assign5410_e5347))) / assign5410_e5349))) }),)
    } else {
        (var_vdeptmp, var_vdeptmp_dn0, var_vdeptmp_dn1, var_vdeptmp_dn3, var_vdeptmp_dn4, var_vdeptmp_dn5, var_vdeptmp_dn6, var_vdeptmp_dn7, var_vdeptmp_dn8, var_vdeptmp_dn9, var_vdeptmp_dn10,)
    }
};
        var_vdeptmp = assign5410_e5354;
        var_vdeptmp_dn0 = assign5410_e5354_d_n0;
        var_vdeptmp_dn1 = assign5410_e5354_d_n1;
        var_vdeptmp_dn3 = assign5410_e5354_d_n3;
        var_vdeptmp_dn4 = assign5410_e5354_d_n4;
        var_vdeptmp_dn5 = assign5410_e5354_d_n5;
        var_vdeptmp_dn6 = assign5410_e5354_d_n6;
        var_vdeptmp_dn7 = assign5410_e5354_d_n7;
        var_vdeptmp_dn8 = assign5410_e5354_d_n8;
        var_vdeptmp_dn9 = assign5410_e5354_d_n9;
        var_vdeptmp_dn10 = assign5410_e5354_d_n10;
        var_vdeptmp_rv = 0.0;

        let assign5420_e5357: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard97 = assign5420_e5357;
        var_guard97_rv = 0.0;

        let (assign5430_e5373, assign5430_e5373_d_n0, assign5430_e5373_d_n1, assign5430_e5373_d_n3, assign5430_e5373_d_n4, assign5430_e5373_d_n5, assign5430_e5373_d_n6, assign5430_e5373_d_n7, assign5430_e5373_d_n8, assign5430_e5373_d_n9, assign5430_e5373_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) {
        (var_vdeptmp, var_vdeptmp_dn0, var_vdeptmp_dn1, var_vdeptmp_dn3, var_vdeptmp_dn4, var_vdeptmp_dn5, var_vdeptmp_dn6, var_vdeptmp_dn7, var_vdeptmp_dn8, var_vdeptmp_dn9, var_vdeptmp_dn10,)
    } else {
        (var_vdep, var_vdep_dn0, var_vdep_dn1, var_vdep_dn3, var_vdep_dn4, var_vdep_dn5, var_vdep_dn6, var_vdep_dn7, var_vdep_dn8, var_vdep_dn9, var_vdep_dn10,)
    }
};
        var_vdep = assign5430_e5373;
        var_vdep_dn0 = assign5430_e5373_d_n0;
        var_vdep_dn1 = assign5430_e5373_d_n1;
        var_vdep_dn3 = assign5430_e5373_d_n3;
        var_vdep_dn4 = assign5430_e5373_d_n4;
        var_vdep_dn5 = assign5430_e5373_d_n5;
        var_vdep_dn6 = assign5430_e5373_d_n6;
        var_vdep_dn7 = assign5430_e5373_d_n7;
        var_vdep_dn8 = assign5430_e5373_d_n8;
        var_vdep_dn9 = assign5430_e5373_d_n9;
        var_vdep_dn10 = assign5430_e5373_d_n10;
        var_vdep_rv = 0.0;

        let (assign5440_e5394, assign5440_e5394_d_n0, assign5440_e5394_d_n1, assign5440_e5394_d_n3, assign5440_e5394_d_n4, assign5440_e5394_d_n5, assign5440_e5394_d_n6, assign5440_e5394_d_n7, assign5440_e5394_d_n8, assign5440_e5394_d_n9, assign5440_e5394_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) {
        let assign5440_e5390: f64 = (var_in_ - p.p51);
        let assign5440_e5392: f64 = (assign5440_e5390 / p.p47);
        (assign5440_e5392, (var_in__dn0 / p.p47), (var_in__dn1 / p.p47), (var_in__dn3 / p.p47), (var_in__dn4 / p.p47), (var_in__dn5 / p.p47), (var_in__dn6 / p.p47), (var_in__dn7 / p.p47), (var_in__dn8 / p.p47), (var_in__dn9 / p.p47), (var_in__dn10 / p.p47),)
    } else {
        (var_in_shift_ihcavl, var_in_shift_ihcavl_dn0, var_in_shift_ihcavl_dn1, var_in_shift_ihcavl_dn3, var_in_shift_ihcavl_dn4, var_in_shift_ihcavl_dn5, var_in_shift_ihcavl_dn6, var_in_shift_ihcavl_dn7, var_in_shift_ihcavl_dn8, var_in_shift_ihcavl_dn9, var_in_shift_ihcavl_dn10,)
    }
};
        var_in_shift_ihcavl = assign5440_e5394;
        var_in_shift_ihcavl_dn0 = assign5440_e5394_d_n0;
        var_in_shift_ihcavl_dn1 = assign5440_e5394_d_n1;
        var_in_shift_ihcavl_dn3 = assign5440_e5394_d_n3;
        var_in_shift_ihcavl_dn4 = assign5440_e5394_d_n4;
        var_in_shift_ihcavl_dn5 = assign5440_e5394_d_n5;
        var_in_shift_ihcavl_dn6 = assign5440_e5394_d_n6;
        var_in_shift_ihcavl_dn7 = assign5440_e5394_d_n7;
        var_in_shift_ihcavl_dn8 = assign5440_e5394_d_n8;
        var_in_shift_ihcavl_dn9 = assign5440_e5394_d_n9;
        var_in_shift_ihcavl_dn10 = assign5440_e5394_d_n10;
        var_in_shift_ihcavl_rv = 0.0;

        let (assign5450_e5415, assign5450_e5415_d_n0, assign5450_e5415_d_n1, assign5450_e5415_d_n3, assign5450_e5415_d_n4, assign5450_e5415_d_n5, assign5450_e5415_d_n6, assign5450_e5415_d_n7, assign5450_e5415_d_n8, assign5450_e5415_d_n9, assign5450_e5415_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) {
        let assign5450_e5411: f64 = (var_in_shift_ihcavl - 1.0);
        let assign5450_e5413: f64 = (assign5450_e5411 / p.p50);
        (assign5450_e5413, (var_in_shift_ihcavl_dn0 / p.p50), (var_in_shift_ihcavl_dn1 / p.p50), (var_in_shift_ihcavl_dn3 / p.p50), (var_in_shift_ihcavl_dn4 / p.p50), (var_in_shift_ihcavl_dn5 / p.p50), (var_in_shift_ihcavl_dn6 / p.p50), (var_in_shift_ihcavl_dn7 / p.p50), (var_in_shift_ihcavl_dn8 / p.p50), (var_in_shift_ihcavl_dn9 / p.p50), (var_in_shift_ihcavl_dn10 / p.p50),)
    } else {
        (var_dxa, var_dxa_dn0, var_dxa_dn1, var_dxa_dn3, var_dxa_dn4, var_dxa_dn5, var_dxa_dn6, var_dxa_dn7, var_dxa_dn8, var_dxa_dn9, var_dxa_dn10,)
    }
};
        var_dxa = assign5450_e5415;
        var_dxa_dn0 = assign5450_e5415_d_n0;
        var_dxa_dn1 = assign5450_e5415_d_n1;
        var_dxa_dn3 = assign5450_e5415_d_n3;
        var_dxa_dn4 = assign5450_e5415_d_n4;
        var_dxa_dn5 = assign5450_e5415_d_n5;
        var_dxa_dn6 = assign5450_e5415_d_n6;
        var_dxa_dn7 = assign5450_e5415_d_n7;
        var_dxa_dn8 = assign5450_e5415_d_n8;
        var_dxa_dn9 = assign5450_e5415_d_n9;
        var_dxa_dn10 = assign5450_e5415_d_n10;
        var_dxa_rv = 0.0;

        let assign5460_e5418: f64 = if var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        var_guard98 = assign5460_e5418;
        var_guard98_rv = 0.0;

        let (assign5470_e5445, assign5470_e5445_d_n0, assign5470_e5445_d_n1, assign5470_e5445_d_n3, assign5470_e5445_d_n4, assign5470_e5445_d_n5, assign5470_e5445_d_n6, assign5470_e5445_d_n7, assign5470_e5445_d_n8, assign5470_e5445_d_n9, assign5470_e5445_d_n10,) = {
    if (((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && (var_guard98 != 0.0)) {
        let assign5470_e5439: f64 = (var_dxa).exp();
        let assign5470_e5440: f64 = (1.0 + assign5470_e5439);
        let assign5470_e5441: f64 = (assign5470_e5440).ln();
        let assign5470_e5442: f64 = (p.p50 * assign5470_e5441);
        let assign5470_e5443: f64 = (1.0 + assign5470_e5442);
        (assign5470_e5443, (p.p50 * ((assign5470_e5439 * var_dxa_dn0) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn1) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn3) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn4) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn5) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn6) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn7) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn8) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn9) / assign5470_e5440)), (p.p50 * ((assign5470_e5439 * var_dxa_dn10) / assign5470_e5440)),)
    } else {
        (var_in_shift_n, var_in_shift_n_dn0, var_in_shift_n_dn1, var_in_shift_n_dn3, var_in_shift_n_dn4, var_in_shift_n_dn5, var_in_shift_n_dn6, var_in_shift_n_dn7, var_in_shift_n_dn8, var_in_shift_n_dn9, var_in_shift_n_dn10,)
    }
};
        var_in_shift_n = assign5470_e5445;
        var_in_shift_n_dn0 = assign5470_e5445_d_n0;
        var_in_shift_n_dn1 = assign5470_e5445_d_n1;
        var_in_shift_n_dn3 = assign5470_e5445_d_n3;
        var_in_shift_n_dn4 = assign5470_e5445_d_n4;
        var_in_shift_n_dn5 = assign5470_e5445_d_n5;
        var_in_shift_n_dn6 = assign5470_e5445_d_n6;
        var_in_shift_n_dn7 = assign5470_e5445_d_n7;
        var_in_shift_n_dn8 = assign5470_e5445_d_n8;
        var_in_shift_n_dn9 = assign5470_e5445_d_n9;
        var_in_shift_n_dn10 = assign5470_e5445_d_n10;
        var_in_shift_n_rv = 0.0;

        let (assign5480_e5474, assign5480_e5474_d_n0, assign5480_e5474_d_n1, assign5480_e5474_d_n3, assign5480_e5474_d_n4, assign5480_e5474_d_n5, assign5480_e5474_d_n6, assign5480_e5474_d_n7, assign5480_e5474_d_n8, assign5480_e5474_d_n9, assign5480_e5474_d_n10,) = {
    if (((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && (var_guard98 == 0.0)) {
        let assign5480_e5467: f64 = (-var_dxa);
        let assign5480_e5468: f64 = (assign5480_e5467).exp();
        let assign5480_e5469: f64 = (1.0 + assign5480_e5468);
        let assign5480_e5470: f64 = (assign5480_e5469).ln();
        let assign5480_e5471: f64 = (p.p50 * assign5480_e5470);
        let assign5480_e5472: f64 = (var_in_shift_ihcavl + assign5480_e5471);
        (assign5480_e5472, (var_in_shift_ihcavl_dn0 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn0)) / assign5480_e5469))), (var_in_shift_ihcavl_dn1 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn1)) / assign5480_e5469))), (var_in_shift_ihcavl_dn3 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn3)) / assign5480_e5469))), (var_in_shift_ihcavl_dn4 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn4)) / assign5480_e5469))), (var_in_shift_ihcavl_dn5 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn5)) / assign5480_e5469))), (var_in_shift_ihcavl_dn6 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn6)) / assign5480_e5469))), (var_in_shift_ihcavl_dn7 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn7)) / assign5480_e5469))), (var_in_shift_ihcavl_dn8 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn8)) / assign5480_e5469))), (var_in_shift_ihcavl_dn9 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn9)) / assign5480_e5469))), (var_in_shift_ihcavl_dn10 + (p.p50 * ((assign5480_e5468 * (-var_dxa_dn10)) / assign5480_e5469))),)
    } else {
        (var_in_shift_n, var_in_shift_n_dn0, var_in_shift_n_dn1, var_in_shift_n_dn3, var_in_shift_n_dn4, var_in_shift_n_dn5, var_in_shift_n_dn6, var_in_shift_n_dn7, var_in_shift_n_dn8, var_in_shift_n_dn9, var_in_shift_n_dn10,)
    }
};
        var_in_shift_n = assign5480_e5474;
        var_in_shift_n_dn0 = assign5480_e5474_d_n0;
        var_in_shift_n_dn1 = assign5480_e5474_d_n1;
        var_in_shift_n_dn3 = assign5480_e5474_d_n3;
        var_in_shift_n_dn4 = assign5480_e5474_d_n4;
        var_in_shift_n_dn5 = assign5480_e5474_d_n5;
        var_in_shift_n_dn6 = assign5480_e5474_d_n6;
        var_in_shift_n_dn7 = assign5480_e5474_d_n7;
        var_in_shift_n_dn8 = assign5480_e5474_d_n8;
        var_in_shift_n_dn9 = assign5480_e5474_d_n9;
        var_in_shift_n_dn10 = assign5480_e5474_d_n10;
        var_in_shift_n_rv = 0.0;

        let (assign5490_e5495, assign5490_e5495_d_n0, assign5490_e5495_d_n1, assign5490_e5495_d_n3, assign5490_e5495_d_n4, assign5490_e5495_d_n5, assign5490_e5495_d_n6, assign5490_e5495_d_n7, assign5490_e5495_d_n8, assign5490_e5495_d_n9, assign5490_e5495_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) {
        let assign5490_e5492: f64 = (var_in_shift_n).powf(p.p49);
        let assign5490_e5493: f64 = (var_vdeptmp * assign5490_e5492);
        (assign5490_e5493, ((var_vdeptmp_dn0 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn0)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn0 / var_in_shift_n))) })), ((var_vdeptmp_dn1 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn1)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn1 / var_in_shift_n))) })), ((var_vdeptmp_dn3 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn3)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn3 / var_in_shift_n))) })), ((var_vdeptmp_dn4 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn4)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn4 / var_in_shift_n))) })), ((var_vdeptmp_dn5 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn5)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn5 / var_in_shift_n))) })), ((var_vdeptmp_dn6 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn6)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn6 / var_in_shift_n))) })), ((var_vdeptmp_dn7 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn7)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn7 / var_in_shift_n))) })), ((var_vdeptmp_dn8 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn8)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn8 / var_in_shift_n))) })), ((var_vdeptmp_dn9 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn9)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn9 / var_in_shift_n))) })), ((var_vdeptmp_dn10 * assign5490_e5492) + (var_vdeptmp * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((var_in_shift_n).powf(p.p49 - 1.0) * var_in_shift_n_dn10)) } } else { (assign5490_e5492 * (p.p49 * (var_in_shift_n_dn10 / var_in_shift_n))) })),)
    } else {
        (var_vdep, var_vdep_dn0, var_vdep_dn1, var_vdep_dn3, var_vdep_dn4, var_vdep_dn5, var_vdep_dn6, var_vdep_dn7, var_vdep_dn8, var_vdep_dn9, var_vdep_dn10,)
    }
};
        var_vdep = assign5490_e5495;
        var_vdep_dn0 = assign5490_e5495_d_n0;
        var_vdep_dn1 = assign5490_e5495_d_n1;
        var_vdep_dn3 = assign5490_e5495_d_n3;
        var_vdep_dn4 = assign5490_e5495_d_n4;
        var_vdep_dn5 = assign5490_e5495_d_n5;
        var_vdep_dn6 = assign5490_e5495_d_n6;
        var_vdep_dn7 = assign5490_e5495_d_n7;
        var_vdep_dn8 = assign5490_e5495_d_n8;
        var_vdep_dn9 = assign5490_e5495_d_n9;
        var_vdep_dn10 = assign5490_e5495_d_n10;
        var_vdep_rv = 0.0;

        let assign5500_e5497: f64 = (-var_bavl_t);
        let assign5500_e5499: f64 = (assign5500_e5497 * var_vdep);
        let assign5500_e5501: f64 = if assign5500_e5499 < p.p138 { 1.0 } else { 0.0 };
        var_guard99 = assign5500_e5501;
        var_guard99_rv = 0.0;

        let (assign5510_e5521, assign5510_e5521_d_n0, assign5510_e5521_d_n1, assign5510_e5521_d_n3, assign5510_e5521_d_n4, assign5510_e5521_d_n5, assign5510_e5521_d_n6, assign5510_e5521_d_n7, assign5510_e5521_d_n8, assign5510_e5521_d_n9, assign5510_e5521_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard99 != 0.0)) {
        let assign5510_e5516: f64 = (-var_bavl_t);
        let assign5510_e5518: f64 = (assign5510_e5516 * var_vdep);
        let assign5510_e5519: f64 = (assign5510_e5518).exp();
        (assign5510_e5519, (assign5510_e5519 * (((-var_bavl_t_dn0) * var_vdep) + (assign5510_e5516 * var_vdep_dn0))), (assign5510_e5519 * (((-var_bavl_t_dn1) * var_vdep) + (assign5510_e5516 * var_vdep_dn1))), (assign5510_e5519 * (((-var_bavl_t_dn3) * var_vdep) + (assign5510_e5516 * var_vdep_dn3))), (assign5510_e5519 * (((-var_bavl_t_dn4) * var_vdep) + (assign5510_e5516 * var_vdep_dn4))), (assign5510_e5519 * (((-var_bavl_t_dn5) * var_vdep) + (assign5510_e5516 * var_vdep_dn5))), (assign5510_e5519 * (((-var_bavl_t_dn6) * var_vdep) + (assign5510_e5516 * var_vdep_dn6))), (assign5510_e5519 * (((-var_bavl_t_dn7) * var_vdep) + (assign5510_e5516 * var_vdep_dn7))), (assign5510_e5519 * (((-var_bavl_t_dn8) * var_vdep) + (assign5510_e5516 * var_vdep_dn8))), (assign5510_e5519 * (((-var_bavl_t_dn9) * var_vdep) + (assign5510_e5516 * var_vdep_dn9))), (assign5510_e5519 * (((-var_bavl_t_dn10) * var_vdep) + (assign5510_e5516 * var_vdep_dn10))),)
    } else {
        (var_expmm1, var_expmm1_dn0, var_expmm1_dn1, var_expmm1_dn3, var_expmm1_dn4, var_expmm1_dn5, var_expmm1_dn6, var_expmm1_dn7, var_expmm1_dn8, var_expmm1_dn9, var_expmm1_dn10,)
    }
};
        var_expmm1 = assign5510_e5521;
        var_expmm1_dn0 = assign5510_e5521_d_n0;
        var_expmm1_dn1 = assign5510_e5521_d_n1;
        var_expmm1_dn3 = assign5510_e5521_d_n3;
        var_expmm1_dn4 = assign5510_e5521_d_n4;
        var_expmm1_dn5 = assign5510_e5521_d_n5;
        var_expmm1_dn6 = assign5510_e5521_d_n6;
        var_expmm1_dn7 = assign5510_e5521_d_n7;
        var_expmm1_dn8 = assign5510_e5521_d_n8;
        var_expmm1_dn9 = assign5510_e5521_d_n9;
        var_expmm1_dn10 = assign5510_e5521_d_n10;
        var_expmm1_rv = 0.0;

        let (assign5520_e5539,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard99 == 0.0)) {
        let assign5520_e5537: f64 = (p.p138).exp();
        (assign5520_e5537,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign5520_e5539;
        var_expl_rv = 0.0;

        let (assign5530_e5565, assign5530_e5565_d_n0, assign5530_e5565_d_n1, assign5530_e5565_d_n3, assign5530_e5565_d_n4, assign5530_e5565_d_n5, assign5530_e5565_d_n6, assign5530_e5565_d_n7, assign5530_e5565_d_n8, assign5530_e5565_d_n9, assign5530_e5565_d_n10,) = {
    if ((((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) && (var_guard99 == 0.0)) {
        let assign5530_e5557: f64 = (-var_bavl_t);
        let assign5530_e5559: f64 = (assign5530_e5557 * var_vdep);
        let assign5530_e5561: f64 = (assign5530_e5559 - p.p138);
        let assign5530_e5562: f64 = (1.0 + assign5530_e5561);
        let assign5530_e5563: f64 = (var_expl * assign5530_e5562);
        (assign5530_e5563, (var_expl * (((-var_bavl_t_dn0) * var_vdep) + (assign5530_e5557 * var_vdep_dn0))), (var_expl * (((-var_bavl_t_dn1) * var_vdep) + (assign5530_e5557 * var_vdep_dn1))), (var_expl * (((-var_bavl_t_dn3) * var_vdep) + (assign5530_e5557 * var_vdep_dn3))), (var_expl * (((-var_bavl_t_dn4) * var_vdep) + (assign5530_e5557 * var_vdep_dn4))), (var_expl * (((-var_bavl_t_dn5) * var_vdep) + (assign5530_e5557 * var_vdep_dn5))), (var_expl * (((-var_bavl_t_dn6) * var_vdep) + (assign5530_e5557 * var_vdep_dn6))), (var_expl * (((-var_bavl_t_dn7) * var_vdep) + (assign5530_e5557 * var_vdep_dn7))), (var_expl * (((-var_bavl_t_dn8) * var_vdep) + (assign5530_e5557 * var_vdep_dn8))), (var_expl * (((-var_bavl_t_dn9) * var_vdep) + (assign5530_e5557 * var_vdep_dn9))), (var_expl * (((-var_bavl_t_dn10) * var_vdep) + (assign5530_e5557 * var_vdep_dn10))),)
    } else {
        (var_expmm1, var_expmm1_dn0, var_expmm1_dn1, var_expmm1_dn3, var_expmm1_dn4, var_expmm1_dn5, var_expmm1_dn6, var_expmm1_dn7, var_expmm1_dn8, var_expmm1_dn9, var_expmm1_dn10,)
    }
};
        var_expmm1 = assign5530_e5565;
        var_expmm1_dn0 = assign5530_e5565_d_n0;
        var_expmm1_dn1 = assign5530_e5565_d_n1;
        var_expmm1_dn3 = assign5530_e5565_d_n3;
        var_expmm1_dn4 = assign5530_e5565_d_n4;
        var_expmm1_dn5 = assign5530_e5565_d_n5;
        var_expmm1_dn6 = assign5530_e5565_d_n6;
        var_expmm1_dn7 = assign5530_e5565_d_n7;
        var_expmm1_dn8 = assign5530_e5565_d_n8;
        var_expmm1_dn9 = assign5530_e5565_d_n9;
        var_expmm1_dn10 = assign5530_e5565_d_n10;
        var_expmm1_rv = 0.0;

        let (assign5540_e5587, assign5540_e5587_d_n0, assign5540_e5587_d_n1, assign5540_e5587_d_n3, assign5540_e5587_d_n4, assign5540_e5587_d_n5, assign5540_e5587_d_n6, assign5540_e5587_d_n7, assign5540_e5587_d_n8, assign5540_e5587_d_n9, assign5540_e5587_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard86 == 0.0)) && (var_guard90 == 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) {
        let assign5540_e5579: f64 = (p.p39 / var_bavl_t);
        let assign5540_e5582: f64 = (p.p43 - var_vb2c1);
        let assign5540_e5583: f64 = (assign5540_e5579 * assign5540_e5582);
        let assign5540_e5585: f64 = (assign5540_e5583 * var_expmm1);
        (assign5540_e5585, ((((-((p.p39 * var_bavl_t_dn0) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn0)), ((((-((p.p39 * var_bavl_t_dn1) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn1)), ((((-((p.p39 * var_bavl_t_dn3) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn3)), ((((-((p.p39 * var_bavl_t_dn4) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn4)), ((((-((p.p39 * var_bavl_t_dn5) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn5)), (((((-((p.p39 * var_bavl_t_dn6) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-var_vb2c1_dn6))) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn6)), (((((-((p.p39 * var_bavl_t_dn7) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) + (assign5540_e5579 * (-var_vb2c1_dn7))) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn7)), ((((-((p.p39 * var_bavl_t_dn8) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn8)), ((((-((p.p39 * var_bavl_t_dn9) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn9)), ((((-((p.p39 * var_bavl_t_dn10) / (var_bavl_t * var_bavl_t))) * assign5540_e5582) * var_expmm1) + (assign5540_e5583 * var_expmm1_dn10)),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5540_e5587;
        var_gem_dn0 = assign5540_e5587_d_n0;
        var_gem_dn1 = assign5540_e5587_d_n1;
        var_gem_dn3 = assign5540_e5587_d_n3;
        var_gem_dn4 = assign5540_e5587_d_n4;
        var_gem_dn5 = assign5540_e5587_d_n5;
        var_gem_dn6 = assign5540_e5587_d_n6;
        var_gem_dn7 = assign5540_e5587_d_n7;
        var_gem_dn8 = assign5540_e5587_d_n8;
        var_gem_dn9 = assign5540_e5587_d_n9;
        var_gem_dn10 = assign5540_e5587_d_n10;
        var_gem_rv = 0.0;

        let assign5550_e5590: f64 = if var_gem > 0.0 { 1.0 } else { 0.0 };
        var_guard100 = assign5550_e5590;
        var_guard100_rv = 0.0;

        let assign5560_e5593: f64 = if p.p52 == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign5560_e5593;
        var_guard101_rv = 0.0;

        let (assign5570_e5619, assign5570_e5619_d_n0, assign5570_e5619_d_n1, assign5570_e5619_d_n3, assign5570_e5619_d_n4, assign5570_e5619_d_n5, assign5570_e5619_d_n6, assign5570_e5619_d_n7, assign5570_e5619_d_n8, assign5570_e5619_d_n9, assign5570_e5619_d_n10,) = {
    if (((var_guard85 != 0.0) && (var_guard100 != 0.0)) && (var_guard101 != 0.0)) {
        let assign5570_e5603: f64 = (var_rbc_t + var_rb2);
        let assign5570_e5604: f64 = (var_in_ * assign5570_e5603);
        let assign5570_e5605: f64 = (var_vt / assign5570_e5604);
        let assign5570_e5608: f64 = (var_qbi / var_is_t);
        let assign5570_e5610: f64 = (assign5570_e5608 * var_ibi_t);
        let assign5570_e5611: f64 = (assign5570_e5605 + assign5570_e5610);
        let assign5570_e5615: f64 = (var_rbc_t + var_rb2);
        let assign5570_e5616: f64 = (var_re_t / assign5570_e5615);
        let assign5570_e5617: f64 = (assign5570_e5611 + assign5570_e5616);
        (assign5570_e5617, (((-((var_vt * ((var_in__dn0 * assign5570_e5603) + (var_in_ * var_rb2_dn0))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn0 * var_is_t) - (var_qbi * var_is_t_dn0)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn0) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn1 * assign5570_e5603) + (var_in_ * var_rb2_dn1))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn1 * var_is_t) - (var_qbi * var_is_t_dn1)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn1) / (assign5570_e5615 * assign5570_e5615)))), (((((var_vt_dn3 * assign5570_e5604) - (var_vt * ((var_in__dn3 * assign5570_e5603) + (var_in_ * (var_rbc_t_dn3 + var_rb2_dn3))))) / (assign5570_e5604 * assign5570_e5604)) + (((((var_qbi_dn3 * var_is_t) - (var_qbi * var_is_t_dn3)) / (var_is_t * var_is_t)) * var_ibi_t) + (assign5570_e5608 * var_ibi_t_dn3))) + (((var_re_t_dn3 * assign5570_e5615) - (var_re_t * (var_rbc_t_dn3 + var_rb2_dn3))) / (assign5570_e5615 * assign5570_e5615))), (((-((var_vt * ((var_in__dn4 * assign5570_e5603) + (var_in_ * var_rb2_dn4))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn4 * var_is_t) - (var_qbi * var_is_t_dn4)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn4) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn5 * assign5570_e5603) + (var_in_ * var_rb2_dn5))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn5 * var_is_t) - (var_qbi * var_is_t_dn5)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn5) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn6 * assign5570_e5603) + (var_in_ * var_rb2_dn6))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn6 * var_is_t) - (var_qbi * var_is_t_dn6)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn6) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn7 * assign5570_e5603) + (var_in_ * var_rb2_dn7))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn7 * var_is_t) - (var_qbi * var_is_t_dn7)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn7) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn8 * assign5570_e5603) + (var_in_ * var_rb2_dn8))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn8 * var_is_t) - (var_qbi * var_is_t_dn8)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn8) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn9 * assign5570_e5603) + (var_in_ * var_rb2_dn9))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn9 * var_is_t) - (var_qbi * var_is_t_dn9)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn9) / (assign5570_e5615 * assign5570_e5615)))), (((-((var_vt * ((var_in__dn10 * assign5570_e5603) + (var_in_ * var_rb2_dn10))) / (assign5570_e5604 * assign5570_e5604))) + ((((var_qbi_dn10 * var_is_t) - (var_qbi * var_is_t_dn10)) / (var_is_t * var_is_t)) * var_ibi_t)) + (-((var_re_t * var_rb2_dn10) / (assign5570_e5615 * assign5570_e5615)))),)
    } else {
        (var_gmax, var_gmax_dn0, var_gmax_dn1, var_gmax_dn3, var_gmax_dn4, var_gmax_dn5, var_gmax_dn6, var_gmax_dn7, var_gmax_dn8, var_gmax_dn9, var_gmax_dn10,)
    }
};
        var_gmax = assign5570_e5619;
        var_gmax_dn0 = assign5570_e5619_d_n0;
        var_gmax_dn1 = assign5570_e5619_d_n1;
        var_gmax_dn3 = assign5570_e5619_d_n3;
        var_gmax_dn4 = assign5570_e5619_d_n4;
        var_gmax_dn5 = assign5570_e5619_d_n5;
        var_gmax_dn6 = assign5570_e5619_d_n6;
        var_gmax_dn7 = assign5570_e5619_d_n7;
        var_gmax_dn8 = assign5570_e5619_d_n8;
        var_gmax_dn9 = assign5570_e5619_d_n9;
        var_gmax_dn10 = assign5570_e5619_d_n10;
        var_gmax_rv = 0.0;

        let assign5580_e5622: f64 = if p.p38 == 3.0 { 1.0 } else { 0.0 };
        var_guard102 = assign5580_e5622;
        var_guard102_rv = 0.0;

        let (assign5590_e5636, assign5590_e5636_d_n0, assign5590_e5636_d_n1, assign5590_e5636_d_n3, assign5590_e5636_d_n4, assign5590_e5636_d_n5, assign5590_e5636_d_n6, assign5590_e5636_d_n7, assign5590_e5636_d_n8, assign5590_e5636_d_n9, assign5590_e5636_d_n10,) = {
    if ((((var_guard85 != 0.0) && (var_guard100 != 0.0)) && (var_guard101 != 0.0)) && (var_guard102 != 0.0)) {
        let assign5590_e5632: f64 = (var_gem - var_gmax);
        let assign5590_e5634: f64 = (assign5590_e5632 / 1e-6);
        (assign5590_e5634, ((var_gem_dn0 - var_gmax_dn0) / 1e-6), ((var_gem_dn1 - var_gmax_dn1) / 1e-6), ((var_gem_dn3 - var_gmax_dn3) / 1e-6), ((var_gem_dn4 - var_gmax_dn4) / 1e-6), ((var_gem_dn5 - var_gmax_dn5) / 1e-6), ((var_gem_dn6 - var_gmax_dn6) / 1e-6), ((var_gem_dn7 - var_gmax_dn7) / 1e-6), ((var_gem_dn8 - var_gmax_dn8) / 1e-6), ((var_gem_dn9 - var_gmax_dn9) / 1e-6), ((var_gem_dn10 - var_gmax_dn10) / 1e-6),)
    } else {
        (var_dxa, var_dxa_dn0, var_dxa_dn1, var_dxa_dn3, var_dxa_dn4, var_dxa_dn5, var_dxa_dn6, var_dxa_dn7, var_dxa_dn8, var_dxa_dn9, var_dxa_dn10,)
    }
};
        var_dxa = assign5590_e5636;
        var_dxa_dn0 = assign5590_e5636_d_n0;
        var_dxa_dn1 = assign5590_e5636_d_n1;
        var_dxa_dn3 = assign5590_e5636_d_n3;
        var_dxa_dn4 = assign5590_e5636_d_n4;
        var_dxa_dn5 = assign5590_e5636_d_n5;
        var_dxa_dn6 = assign5590_e5636_d_n6;
        var_dxa_dn7 = assign5590_e5636_d_n7;
        var_dxa_dn8 = assign5590_e5636_d_n8;
        var_dxa_dn9 = assign5590_e5636_d_n9;
        var_dxa_dn10 = assign5590_e5636_d_n10;
        var_dxa_rv = 0.0;

        let assign5600_e5639: f64 = if var_gem < var_gmax { 1.0 } else { 0.0 };
        var_guard103 = assign5600_e5639;
        var_guard103_rv = 0.0;

        let (assign5610_e5659, assign5610_e5659_d_n0, assign5610_e5659_d_n1, assign5610_e5659_d_n3, assign5610_e5659_d_n4, assign5610_e5659_d_n5, assign5610_e5659_d_n6, assign5610_e5659_d_n7, assign5610_e5659_d_n8, assign5610_e5659_d_n9, assign5610_e5659_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard100 != 0.0)) && (var_guard101 != 0.0)) && (var_guard102 != 0.0)) && (var_guard103 != 0.0)) {
        let assign5610_e5653: f64 = (var_dxa).exp();
        let assign5610_e5654: f64 = (1.0 + assign5610_e5653);
        let assign5610_e5655: f64 = (assign5610_e5654).ln();
        let assign5610_e5656: f64 = (1e-6 * assign5610_e5655);
        let assign5610_e5657: f64 = (var_gem - assign5610_e5656);
        (assign5610_e5657, (var_gem_dn0 - (1e-6 * ((assign5610_e5653 * var_dxa_dn0) / assign5610_e5654))), (var_gem_dn1 - (1e-6 * ((assign5610_e5653 * var_dxa_dn1) / assign5610_e5654))), (var_gem_dn3 - (1e-6 * ((assign5610_e5653 * var_dxa_dn3) / assign5610_e5654))), (var_gem_dn4 - (1e-6 * ((assign5610_e5653 * var_dxa_dn4) / assign5610_e5654))), (var_gem_dn5 - (1e-6 * ((assign5610_e5653 * var_dxa_dn5) / assign5610_e5654))), (var_gem_dn6 - (1e-6 * ((assign5610_e5653 * var_dxa_dn6) / assign5610_e5654))), (var_gem_dn7 - (1e-6 * ((assign5610_e5653 * var_dxa_dn7) / assign5610_e5654))), (var_gem_dn8 - (1e-6 * ((assign5610_e5653 * var_dxa_dn8) / assign5610_e5654))), (var_gem_dn9 - (1e-6 * ((assign5610_e5653 * var_dxa_dn9) / assign5610_e5654))), (var_gem_dn10 - (1e-6 * ((assign5610_e5653 * var_dxa_dn10) / assign5610_e5654))),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5610_e5659;
        var_gem_dn0 = assign5610_e5659_d_n0;
        var_gem_dn1 = assign5610_e5659_d_n1;
        var_gem_dn3 = assign5610_e5659_d_n3;
        var_gem_dn4 = assign5610_e5659_d_n4;
        var_gem_dn5 = assign5610_e5659_d_n5;
        var_gem_dn6 = assign5610_e5659_d_n6;
        var_gem_dn7 = assign5610_e5659_d_n7;
        var_gem_dn8 = assign5610_e5659_d_n8;
        var_gem_dn9 = assign5610_e5659_d_n9;
        var_gem_dn10 = assign5610_e5659_d_n10;
        var_gem_rv = 0.0;

        let (assign5620_e5681, assign5620_e5681_d_n0, assign5620_e5681_d_n1, assign5620_e5681_d_n3, assign5620_e5681_d_n4, assign5620_e5681_d_n5, assign5620_e5681_d_n6, assign5620_e5681_d_n7, assign5620_e5681_d_n8, assign5620_e5681_d_n9, assign5620_e5681_d_n10,) = {
    if (((((var_guard85 != 0.0) && (var_guard100 != 0.0)) && (var_guard101 != 0.0)) && (var_guard102 != 0.0)) && (var_guard103 == 0.0)) {
        let assign5620_e5674: f64 = (-var_dxa);
        let assign5620_e5675: f64 = (assign5620_e5674).exp();
        let assign5620_e5676: f64 = (1.0 + assign5620_e5675);
        let assign5620_e5677: f64 = (assign5620_e5676).ln();
        let assign5620_e5678: f64 = (1e-6 * assign5620_e5677);
        let assign5620_e5679: f64 = (var_gmax - assign5620_e5678);
        (assign5620_e5679, (var_gmax_dn0 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn0)) / assign5620_e5676))), (var_gmax_dn1 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn1)) / assign5620_e5676))), (var_gmax_dn3 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn3)) / assign5620_e5676))), (var_gmax_dn4 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn4)) / assign5620_e5676))), (var_gmax_dn5 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn5)) / assign5620_e5676))), (var_gmax_dn6 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn6)) / assign5620_e5676))), (var_gmax_dn7 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn7)) / assign5620_e5676))), (var_gmax_dn8 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn8)) / assign5620_e5676))), (var_gmax_dn9 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn9)) / assign5620_e5676))), (var_gmax_dn10 - (1e-6 * ((assign5620_e5675 * (-var_dxa_dn10)) / assign5620_e5676))),)
    } else {
        (var_gem, var_gem_dn0, var_gem_dn1, var_gem_dn3, var_gem_dn4, var_gem_dn5, var_gem_dn6, var_gem_dn7, var_gem_dn8, var_gem_dn9, var_gem_dn10,)
    }
};
        var_gem = assign5620_e5681;
        var_gem_dn0 = assign5620_e5681_d_n0;
        var_gem_dn1 = assign5620_e5681_d_n1;
        var_gem_dn3 = assign5620_e5681_d_n3;
        var_gem_dn4 = assign5620_e5681_d_n4;
        var_gem_dn5 = assign5620_e5681_d_n5;
        var_gem_dn6 = assign5620_e5681_d_n6;
        var_gem_dn7 = assign5620_e5681_d_n7;
        var_gem_dn8 = assign5620_e5681_d_n8;
        var_gem_dn9 = assign5620_e5681_d_n9;
        var_gem_dn10 = assign5620_e5681_d_n10;
        var_gem_rv = 0.0;

        let assign5730_e5844: f64 = (1.0 - p.p67);
        let assign5730_e5846: f64 = (assign5730_e5844 * var_cje_t);
        let assign5730_e5848: f64 = (assign5730_e5846 * var_vte);
        var_qte = assign5730_e5848;
        var_qte_dn0 = (((assign5730_e5844 * var_cje_t_dn0) * var_vte) + (assign5730_e5846 * var_vte_dn0));
        var_qte_dn1 = (((assign5730_e5844 * var_cje_t_dn1) * var_vte) + (assign5730_e5846 * var_vte_dn1));
        var_qte_dn3 = (((assign5730_e5844 * var_cje_t_dn3) * var_vte) + (assign5730_e5846 * var_vte_dn3));
        var_qte_dn4 = (((assign5730_e5844 * var_cje_t_dn4) * var_vte) + (assign5730_e5846 * var_vte_dn4));
        var_qte_dn5 = (((assign5730_e5844 * var_cje_t_dn5) * var_vte) + (assign5730_e5846 * var_vte_dn5));
        var_qte_dn6 = (((assign5730_e5844 * var_cje_t_dn6) * var_vte) + (assign5730_e5846 * var_vte_dn6));
        var_qte_dn7 = (((assign5730_e5844 * var_cje_t_dn7) * var_vte) + (assign5730_e5846 * var_vte_dn7));
        var_qte_dn8 = (((assign5730_e5844 * var_cje_t_dn8) * var_vte) + (assign5730_e5846 * var_vte_dn8));
        var_qte_dn9 = (((assign5730_e5844 * var_cje_t_dn9) * var_vte) + (assign5730_e5846 * var_vte_dn9));
        var_qte_dn10 = (((assign5730_e5844 * var_cje_t_dn10) * var_vte) + (assign5730_e5846 * var_vte_dn10));
        var_qte_rv = 0.0;

        let assign5740_e5851: f64 = (var_vb1e1 - var_vfe);
        let assign5740_e5853: f64 = (assign5740_e5851 / var_a_vde);
        var_dxa = assign5740_e5853;
        var_dxa_dn0 = ((((-var_vfe_dn0) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn0)) / (var_a_vde * var_a_vde));
        var_dxa_dn1 = ((((-var_vfe_dn1) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn1)) / (var_a_vde * var_a_vde));
        var_dxa_dn3 = ((((-var_vfe_dn3) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn3)) / (var_a_vde * var_a_vde));
        var_dxa_dn4 = ((((var_vb1e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn4)) / (var_a_vde * var_a_vde));
        var_dxa_dn5 = ((((var_vb1e1_dn5 - var_vfe_dn5) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn5)) / (var_a_vde * var_a_vde));
        var_dxa_dn6 = ((((-var_vfe_dn6) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn6)) / (var_a_vde * var_a_vde));
        var_dxa_dn7 = ((((-var_vfe_dn7) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn7)) / (var_a_vde * var_a_vde));
        var_dxa_dn8 = ((((-var_vfe_dn8) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn8)) / (var_a_vde * var_a_vde));
        var_dxa_dn9 = ((((-var_vfe_dn9) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn9)) / (var_a_vde * var_a_vde));
        var_dxa_dn10 = ((((-var_vfe_dn10) * var_a_vde) - (assign5740_e5851 * var_a_vde_dn10)) / (var_a_vde * var_a_vde));
        var_dxa_rv = 0.0;

        let assign5750_e5856: f64 = if var_vb1e1 < var_vfe { 1.0 } else { 0.0 };
        var_guard106 = assign5750_e5856;
        var_guard106_rv = 0.0;

        let (assign5760_e5868, assign5760_e5868_d_n0, assign5760_e5868_d_n1, assign5760_e5868_d_n3, assign5760_e5868_d_n4, assign5760_e5868_d_n5, assign5760_e5868_d_n6, assign5760_e5868_d_n7, assign5760_e5868_d_n8, assign5760_e5868_d_n9, assign5760_e5868_d_n10,) = {
    if (var_guard106 != 0.0) {
        let assign5760_e5862: f64 = (var_dxa).exp();
        let assign5760_e5863: f64 = (1.0 + assign5760_e5862);
        let assign5760_e5864: f64 = (assign5760_e5863).ln();
        let assign5760_e5865: f64 = (var_a_vde * assign5760_e5864);
        let assign5760_e5866: f64 = (var_vb1e1 - assign5760_e5865);
        (assign5760_e5866, (-((var_a_vde_dn0 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn0) / assign5760_e5863)))), (-((var_a_vde_dn1 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn1) / assign5760_e5863)))), (-((var_a_vde_dn3 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn3) / assign5760_e5863)))), (var_vb1e1_dn4 - ((var_a_vde_dn4 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn4) / assign5760_e5863)))), (var_vb1e1_dn5 - ((var_a_vde_dn5 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn5) / assign5760_e5863)))), (-((var_a_vde_dn6 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn6) / assign5760_e5863)))), (-((var_a_vde_dn7 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn7) / assign5760_e5863)))), (-((var_a_vde_dn8 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn8) / assign5760_e5863)))), (-((var_a_vde_dn9 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn9) / assign5760_e5863)))), (-((var_a_vde_dn10 * assign5760_e5864) + (var_a_vde * ((assign5760_e5862 * var_dxa_dn10) / assign5760_e5863)))),)
    } else {
        (var_vje_s, var_vje_s_dn0, var_vje_s_dn1, var_vje_s_dn3, var_vje_s_dn4, var_vje_s_dn5, var_vje_s_dn6, var_vje_s_dn7, var_vje_s_dn8, var_vje_s_dn9, var_vje_s_dn10,)
    }
};
        var_vje_s = assign5760_e5868;
        var_vje_s_dn0 = assign5760_e5868_d_n0;
        var_vje_s_dn1 = assign5760_e5868_d_n1;
        var_vje_s_dn3 = assign5760_e5868_d_n3;
        var_vje_s_dn4 = assign5760_e5868_d_n4;
        var_vje_s_dn5 = assign5760_e5868_d_n5;
        var_vje_s_dn6 = assign5760_e5868_d_n6;
        var_vje_s_dn7 = assign5760_e5868_d_n7;
        var_vje_s_dn8 = assign5760_e5868_d_n8;
        var_vje_s_dn9 = assign5760_e5868_d_n9;
        var_vje_s_dn10 = assign5760_e5868_d_n10;
        var_vje_s_rv = 0.0;

        *var_dxa_slot = var_dxa;
        *var_dxa_dn0_slot = var_dxa_dn0;
        *var_dxa_dn1_slot = var_dxa_dn1;
        *var_dxa_dn10_slot = var_dxa_dn10;
        *var_dxa_dn3_slot = var_dxa_dn3;
        *var_dxa_dn4_slot = var_dxa_dn4;
        *var_dxa_dn5_slot = var_dxa_dn5;
        *var_dxa_dn6_slot = var_dxa_dn6;
        *var_dxa_dn7_slot = var_dxa_dn7;
        *var_dxa_dn8_slot = var_dxa_dn8;
        *var_dxa_dn9_slot = var_dxa_dn9;
        *var_dxa_rv_slot = var_dxa_rv;
        *var_expl_slot = var_expl;
        *var_expl_rv_slot = var_expl_rv;
        *var_expmm1_slot = var_expmm1;
        *var_expmm1_dn0_slot = var_expmm1_dn0;
        *var_expmm1_dn1_slot = var_expmm1_dn1;
        *var_expmm1_dn10_slot = var_expmm1_dn10;
        *var_expmm1_dn3_slot = var_expmm1_dn3;
        *var_expmm1_dn4_slot = var_expmm1_dn4;
        *var_expmm1_dn5_slot = var_expmm1_dn5;
        *var_expmm1_dn6_slot = var_expmm1_dn6;
        *var_expmm1_dn7_slot = var_expmm1_dn7;
        *var_expmm1_dn8_slot = var_expmm1_dn8;
        *var_expmm1_dn9_slot = var_expmm1_dn9;
        *var_expmm1_rv_slot = var_expmm1_rv;
        *var_gem_slot = var_gem;
        *var_gem_dn0_slot = var_gem_dn0;
        *var_gem_dn1_slot = var_gem_dn1;
        *var_gem_dn10_slot = var_gem_dn10;
        *var_gem_dn3_slot = var_gem_dn3;
        *var_gem_dn4_slot = var_gem_dn4;
        *var_gem_dn5_slot = var_gem_dn5;
        *var_gem_dn6_slot = var_gem_dn6;
        *var_gem_dn7_slot = var_gem_dn7;
        *var_gem_dn8_slot = var_gem_dn8;
        *var_gem_dn9_slot = var_gem_dn9;
        *var_gem_rv_slot = var_gem_rv;
        *var_gmax_slot = var_gmax;
        *var_gmax_dn0_slot = var_gmax_dn0;
        *var_gmax_dn1_slot = var_gmax_dn1;
        *var_gmax_dn10_slot = var_gmax_dn10;
        *var_gmax_dn3_slot = var_gmax_dn3;
        *var_gmax_dn4_slot = var_gmax_dn4;
        *var_gmax_dn5_slot = var_gmax_dn5;
        *var_gmax_dn6_slot = var_gmax_dn6;
        *var_gmax_dn7_slot = var_gmax_dn7;
        *var_gmax_dn8_slot = var_gmax_dn8;
        *var_gmax_dn9_slot = var_gmax_dn9;
        *var_gmax_rv_slot = var_gmax_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_in_shift_ihcavl_slot = var_in_shift_ihcavl;
        *var_in_shift_ihcavl_dn0_slot = var_in_shift_ihcavl_dn0;
        *var_in_shift_ihcavl_dn1_slot = var_in_shift_ihcavl_dn1;
        *var_in_shift_ihcavl_dn10_slot = var_in_shift_ihcavl_dn10;
        *var_in_shift_ihcavl_dn3_slot = var_in_shift_ihcavl_dn3;
        *var_in_shift_ihcavl_dn4_slot = var_in_shift_ihcavl_dn4;
        *var_in_shift_ihcavl_dn5_slot = var_in_shift_ihcavl_dn5;
        *var_in_shift_ihcavl_dn6_slot = var_in_shift_ihcavl_dn6;
        *var_in_shift_ihcavl_dn7_slot = var_in_shift_ihcavl_dn7;
        *var_in_shift_ihcavl_dn8_slot = var_in_shift_ihcavl_dn8;
        *var_in_shift_ihcavl_dn9_slot = var_in_shift_ihcavl_dn9;
        *var_in_shift_ihcavl_rv_slot = var_in_shift_ihcavl_rv;
        *var_in_shift_n_slot = var_in_shift_n;
        *var_in_shift_n_dn0_slot = var_in_shift_n_dn0;
        *var_in_shift_n_dn1_slot = var_in_shift_n_dn1;
        *var_in_shift_n_dn10_slot = var_in_shift_n_dn10;
        *var_in_shift_n_dn3_slot = var_in_shift_n_dn3;
        *var_in_shift_n_dn4_slot = var_in_shift_n_dn4;
        *var_in_shift_n_dn5_slot = var_in_shift_n_dn5;
        *var_in_shift_n_dn6_slot = var_in_shift_n_dn6;
        *var_in_shift_n_dn7_slot = var_in_shift_n_dn7;
        *var_in_shift_n_dn8_slot = var_in_shift_n_dn8;
        *var_in_shift_n_dn9_slot = var_in_shift_n_dn9;
        *var_in_shift_n_rv_slot = var_in_shift_n_rv;
        *var_qte_slot = var_qte;
        *var_qte_dn0_slot = var_qte_dn0;
        *var_qte_dn1_slot = var_qte_dn1;
        *var_qte_dn10_slot = var_qte_dn10;
        *var_qte_dn3_slot = var_qte_dn3;
        *var_qte_dn4_slot = var_qte_dn4;
        *var_qte_dn5_slot = var_qte_dn5;
        *var_qte_dn6_slot = var_qte_dn6;
        *var_qte_dn7_slot = var_qte_dn7;
        *var_qte_dn8_slot = var_qte_dn8;
        *var_qte_dn9_slot = var_qte_dn9;
        *var_qte_rv_slot = var_qte_rv;
        *var_vdep_slot = var_vdep;
        *var_vdep_dn0_slot = var_vdep_dn0;
        *var_vdep_dn1_slot = var_vdep_dn1;
        *var_vdep_dn10_slot = var_vdep_dn10;
        *var_vdep_dn3_slot = var_vdep_dn3;
        *var_vdep_dn4_slot = var_vdep_dn4;
        *var_vdep_dn5_slot = var_vdep_dn5;
        *var_vdep_dn6_slot = var_vdep_dn6;
        *var_vdep_dn7_slot = var_vdep_dn7;
        *var_vdep_dn8_slot = var_vdep_dn8;
        *var_vdep_dn9_slot = var_vdep_dn9;
        *var_vdep_rv_slot = var_vdep_rv;
        *var_vdeptmp_slot = var_vdeptmp;
        *var_vdeptmp_dn0_slot = var_vdeptmp_dn0;
        *var_vdeptmp_dn1_slot = var_vdeptmp_dn1;
        *var_vdeptmp_dn10_slot = var_vdeptmp_dn10;
        *var_vdeptmp_dn3_slot = var_vdeptmp_dn3;
        *var_vdeptmp_dn4_slot = var_vdeptmp_dn4;
        *var_vdeptmp_dn5_slot = var_vdeptmp_dn5;
        *var_vdeptmp_dn6_slot = var_vdeptmp_dn6;
        *var_vdeptmp_dn7_slot = var_vdeptmp_dn7;
        *var_vdeptmp_dn8_slot = var_vdeptmp_dn8;
        *var_vdeptmp_dn9_slot = var_vdeptmp_dn9;
        *var_vdeptmp_rv_slot = var_vdeptmp_rv;
        *var_vje_s_slot = var_vje_s;
        *var_vje_s_dn0_slot = var_vje_s_dn0;
        *var_vje_s_dn1_slot = var_vje_s_dn1;
        *var_vje_s_dn10_slot = var_vje_s_dn10;
        *var_vje_s_dn3_slot = var_vje_s_dn3;
        *var_vje_s_dn4_slot = var_vje_s_dn4;
        *var_vje_s_dn5_slot = var_vje_s_dn5;
        *var_vje_s_dn6_slot = var_vje_s_dn6;
        *var_vje_s_dn7_slot = var_vje_s_dn7;
        *var_vje_s_dn8_slot = var_vje_s_dn8;
        *var_vje_s_dn9_slot = var_vje_s_dn9;
        *var_vje_s_rv_slot = var_vje_s_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_bjc: f64,
        var_bjc_dn0: f64,
        var_bjc_dn1: f64,
        var_bjc_dn10: f64,
        var_bjc_dn3: f64,
        var_bjc_dn4: f64,
        var_bjc_dn5: f64,
        var_bjc_dn6: f64,
        var_bjc_dn7: f64,
        var_bjc_dn8: f64,
        var_bjc_dn9: f64,
        var_cjc_t: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn10: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_cje_t: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_guard106: f64,
        var_ik_t: f64,
        var_ik_t_dn3: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_is_t: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn10: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_is_t_dn7: f64,
        var_is_t_dn8: f64,
        var_is_t_dn9: f64,
        var_n0: f64,
        var_n0_dn0: f64,
        var_n0_dn1: f64,
        var_n0_dn10: f64,
        var_n0_dn3: f64,
        var_n0_dn4: f64,
        var_n0_dn5: f64,
        var_n0_dn6: f64,
        var_n0_dn7: f64,
        var_n0_dn8: f64,
        var_n0_dn9: f64,
        var_nb: f64,
        var_nb_dn0: f64,
        var_nb_dn1: f64,
        var_nb_dn10: f64,
        var_nb_dn3: f64,
        var_nb_dn4: f64,
        var_nb_dn5: f64,
        var_nb_dn6: f64,
        var_nb_dn7: f64,
        var_nb_dn8: f64,
        var_nb_dn9: f64,
        var_nbex: f64,
        var_nbex_dn0: f64,
        var_nbex_dn1: f64,
        var_nbex_dn10: f64,
        var_nbex_dn3: f64,
        var_nbex_dn4: f64,
        var_nbex_dn5: f64,
        var_nbex_dn6: f64,
        var_nbex_dn7: f64,
        var_nbex_dn8: f64,
        var_nbex_dn9: f64,
        var_p0star: f64,
        var_p0star_dn0: f64,
        var_p0star_dn1: f64,
        var_p0star_dn10: f64,
        var_p0star_dn3: f64,
        var_p0star_dn4: f64,
        var_p0star_dn5: f64,
        var_p0star_dn6: f64,
        var_p0star_dn7: f64,
        var_p0star_dn8: f64,
        var_p0star_dn9: f64,
        var_pw: f64,
        var_pw_dn0: f64,
        var_pw_dn1: f64,
        var_pw_dn10: f64,
        var_pw_dn3: f64,
        var_pw_dn4: f64,
        var_pw_dn5: f64,
        var_pw_dn6: f64,
        var_pw_dn7: f64,
        var_pw_dn8: f64,
        var_pw_dn9: f64,
        var_pwex: f64,
        var_pwex_dn0: f64,
        var_pwex_dn1: f64,
        var_pwex_dn10: f64,
        var_pwex_dn3: f64,
        var_pwex_dn4: f64,
        var_pwex_dn5: f64,
        var_pwex_dn6: f64,
        var_pwex_dn7: f64,
        var_pwex_dn8: f64,
        var_pwex_dn9: f64,
        var_q1q: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_rcv_t: f64,
        var_rcv_t_dn3: f64,
        var_taub_t: f64,
        var_taub_t_dn3: f64,
        var_taue_t: f64,
        var_taue_t_dn3: f64,
        var_taur_t: f64,
        var_taur_t_dn3: f64,
        var_tepi_t: f64,
        var_tepi_t_dn3: f64,
        var_vb1c4: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1e1: f64,
        var_vb1e1_dn4: f64,
        var_vb1e1_dn5: f64,
        var_vb2e1: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn6: f64,
        var_vbc3: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn5: f64,
        var_vbc3_dn6: f64,
        var_vbc3_dn7: f64,
        var_vbc3_dn8: f64,
        var_vbc3_dn9: f64,
        var_vdc_ctc_t: f64,
        var_vdc_ctc_t_dn0: f64,
        var_vdc_ctc_t_dn1: f64,
        var_vdc_ctc_t_dn10: f64,
        var_vdc_ctc_t_dn3: f64,
        var_vdc_ctc_t_dn4: f64,
        var_vdc_ctc_t_dn5: f64,
        var_vdc_ctc_t_dn6: f64,
        var_vdc_ctc_t_dn7: f64,
        var_vdc_ctc_t_dn8: f64,
        var_vdc_ctc_t_dn9: f64,
        var_vdcex_t: f64,
        var_vde_t: f64,
        var_vde_t_dn0: f64,
        var_vde_t_dn1: f64,
        var_vde_t_dn10: f64,
        var_vde_t_dn3: f64,
        var_vde_t_dn4: f64,
        var_vde_t_dn5: f64,
        var_vde_t_dn6: f64,
        var_vde_t_dn7: f64,
        var_vde_t_dn8: f64,
        var_vde_t_dn9: f64,
        var_vfc: f64,
        var_vfc_dn0: f64,
        var_vfc_dn1: f64,
        var_vfc_dn10: f64,
        var_vfc_dn3: f64,
        var_vfc_dn4: f64,
        var_vfc_dn5: f64,
        var_vfc_dn6: f64,
        var_vfc_dn7: f64,
        var_vfc_dn8: f64,
        var_vfc_dn9: f64,
        var_vfe: f64,
        var_vfe_dn0: f64,
        var_vfe_dn1: f64,
        var_vfe_dn10: f64,
        var_vfe_dn3: f64,
        var_vfe_dn4: f64,
        var_vfe_dn5: f64,
        var_vfe_dn6: f64,
        var_vfe_dn7: f64,
        var_vfe_dn8: f64,
        var_vfe_dn9: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_vtc: f64,
        var_vtc_dn0: f64,
        var_vtc_dn1: f64,
        var_vtc_dn10: f64,
        var_vtc_dn3: f64,
        var_vtc_dn4: f64,
        var_vtc_dn5: f64,
        var_vtc_dn6: f64,
        var_vtc_dn7: f64,
        var_vtc_dn8: f64,
        var_vtc_dn9: f64,
        var_vtinv: f64,
        var_xi_w: f64,
        var_xi_w_dn0: f64,
        var_xi_w_dn1: f64,
        var_xi_w_dn10: f64,
        var_xi_w_dn3: f64,
        var_xi_w_dn4: f64,
        var_xi_w_dn5: f64,
        var_xi_w_dn6: f64,
        var_xi_w_dn7: f64,
        var_xi_w_dn8: f64,
        var_xi_w_dn9: f64,
        var_xp_t: f64,
        var_xp_t_dn0: f64,
        var_xp_t_dn1: f64,
        var_xp_t_dn10: f64,
        var_xp_t_dn3: f64,
        var_xp_t_dn4: f64,
        var_xp_t_dn5: f64,
        var_xp_t_dn6: f64,
        var_xp_t_dn7: f64,
        var_xp_t_dn8: f64,
        var_xp_t_dn9: f64,
        var_a_vdcctc_slot: &mut f64,
        var_a_vdcctc_dn0_slot: &mut f64,
        var_a_vdcctc_dn1_slot: &mut f64,
        var_a_vdcctc_dn10_slot: &mut f64,
        var_a_vdcctc_dn3_slot: &mut f64,
        var_a_vdcctc_dn4_slot: &mut f64,
        var_a_vdcctc_dn5_slot: &mut f64,
        var_a_vdcctc_dn6_slot: &mut f64,
        var_a_vdcctc_dn7_slot: &mut f64,
        var_a_vdcctc_dn8_slot: &mut f64,
        var_a_vdcctc_dn9_slot: &mut f64,
        var_a_vdcctc_rv_slot: &mut f64,
        var_dxa_slot: &mut f64,
        var_dxa_dn0_slot: &mut f64,
        var_dxa_dn1_slot: &mut f64,
        var_dxa_dn10_slot: &mut f64,
        var_dxa_dn3_slot: &mut f64,
        var_dxa_dn4_slot: &mut f64,
        var_dxa_dn5_slot: &mut f64,
        var_dxa_dn6_slot: &mut f64,
        var_dxa_dn7_slot: &mut f64,
        var_dxa_dn8_slot: &mut f64,
        var_dxa_dn9_slot: &mut f64,
        var_dxa_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_dn3_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_qbc_qs_slot: &mut f64,
        var_qbc_qs_dn0_slot: &mut f64,
        var_qbc_qs_dn1_slot: &mut f64,
        var_qbc_qs_dn10_slot: &mut f64,
        var_qbc_qs_dn3_slot: &mut f64,
        var_qbc_qs_dn4_slot: &mut f64,
        var_qbc_qs_dn5_slot: &mut f64,
        var_qbc_qs_dn6_slot: &mut f64,
        var_qbc_qs_dn7_slot: &mut f64,
        var_qbc_qs_dn8_slot: &mut f64,
        var_qbc_qs_dn9_slot: &mut f64,
        var_qbc_qs_rv_slot: &mut f64,
        var_qbe_qs_slot: &mut f64,
        var_qbe_qs_dn0_slot: &mut f64,
        var_qbe_qs_dn1_slot: &mut f64,
        var_qbe_qs_dn10_slot: &mut f64,
        var_qbe_qs_dn3_slot: &mut f64,
        var_qbe_qs_dn4_slot: &mut f64,
        var_qbe_qs_dn5_slot: &mut f64,
        var_qbe_qs_dn6_slot: &mut f64,
        var_qbe_qs_dn7_slot: &mut f64,
        var_qbe_qs_dn8_slot: &mut f64,
        var_qbe_qs_dn9_slot: &mut f64,
        var_qbe_qs_rv_slot: &mut f64,
        var_qe0_slot: &mut f64,
        var_qe0_dn0_slot: &mut f64,
        var_qe0_dn1_slot: &mut f64,
        var_qe0_dn10_slot: &mut f64,
        var_qe0_dn3_slot: &mut f64,
        var_qe0_dn4_slot: &mut f64,
        var_qe0_dn5_slot: &mut f64,
        var_qe0_dn6_slot: &mut f64,
        var_qe0_dn7_slot: &mut f64,
        var_qe0_dn8_slot: &mut f64,
        var_qe0_dn9_slot: &mut f64,
        var_qe0_rv_slot: &mut f64,
        var_qe_qs_slot: &mut f64,
        var_qe_qs_dn0_slot: &mut f64,
        var_qe_qs_dn1_slot: &mut f64,
        var_qe_qs_dn10_slot: &mut f64,
        var_qe_qs_dn3_slot: &mut f64,
        var_qe_qs_dn4_slot: &mut f64,
        var_qe_qs_dn5_slot: &mut f64,
        var_qe_qs_dn6_slot: &mut f64,
        var_qe_qs_dn7_slot: &mut f64,
        var_qe_qs_dn8_slot: &mut f64,
        var_qe_qs_dn9_slot: &mut f64,
        var_qe_qs_rv_slot: &mut f64,
        var_qepi_slot: &mut f64,
        var_qepi0_slot: &mut f64,
        var_qepi0_dn3_slot: &mut f64,
        var_qepi0_rv_slot: &mut f64,
        var_qepi_dn0_slot: &mut f64,
        var_qepi_dn1_slot: &mut f64,
        var_qepi_dn10_slot: &mut f64,
        var_qepi_dn3_slot: &mut f64,
        var_qepi_dn4_slot: &mut f64,
        var_qepi_dn5_slot: &mut f64,
        var_qepi_dn6_slot: &mut f64,
        var_qepi_dn7_slot: &mut f64,
        var_qepi_dn8_slot: &mut f64,
        var_qepi_dn9_slot: &mut f64,
        var_qepi_rv_slot: &mut f64,
        var_qex_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_qex_rv_slot: &mut f64,
        var_qtc_slot: &mut f64,
        var_qtc_dn0_slot: &mut f64,
        var_qtc_dn1_slot: &mut f64,
        var_qtc_dn10_slot: &mut f64,
        var_qtc_dn3_slot: &mut f64,
        var_qtc_dn4_slot: &mut f64,
        var_qtc_dn5_slot: &mut f64,
        var_qtc_dn6_slot: &mut f64,
        var_qtc_dn7_slot: &mut f64,
        var_qtc_dn8_slot: &mut f64,
        var_qtc_dn9_slot: &mut f64,
        var_qtc_rv_slot: &mut f64,
        var_qte_s_slot: &mut f64,
        var_qte_s_dn0_slot: &mut f64,
        var_qte_s_dn1_slot: &mut f64,
        var_qte_s_dn10_slot: &mut f64,
        var_qte_s_dn3_slot: &mut f64,
        var_qte_s_dn4_slot: &mut f64,
        var_qte_s_dn5_slot: &mut f64,
        var_qte_s_dn6_slot: &mut f64,
        var_qte_s_dn7_slot: &mut f64,
        var_qte_s_dn8_slot: &mut f64,
        var_qte_s_dn9_slot: &mut f64,
        var_qte_s_rv_slot: &mut f64,
        var_qtex_slot: &mut f64,
        var_qtex_dn0_slot: &mut f64,
        var_qtex_dn1_slot: &mut f64,
        var_qtex_dn10_slot: &mut f64,
        var_qtex_dn3_slot: &mut f64,
        var_qtex_dn4_slot: &mut f64,
        var_qtex_dn5_slot: &mut f64,
        var_qtex_dn6_slot: &mut f64,
        var_qtex_dn7_slot: &mut f64,
        var_qtex_dn8_slot: &mut f64,
        var_qtex_dn9_slot: &mut f64,
        var_qtex_rv_slot: &mut f64,
        var_tmpexp_slot: &mut f64,
        var_tmpexp_dn0_slot: &mut f64,
        var_tmpexp_dn1_slot: &mut f64,
        var_tmpexp_dn10_slot: &mut f64,
        var_tmpexp_dn3_slot: &mut f64,
        var_tmpexp_dn4_slot: &mut f64,
        var_tmpexp_dn5_slot: &mut f64,
        var_tmpexp_dn6_slot: &mut f64,
        var_tmpexp_dn7_slot: &mut f64,
        var_tmpexp_dn8_slot: &mut f64,
        var_tmpexp_dn9_slot: &mut f64,
        var_tmpexp_rv_slot: &mut f64,
        var_vjcex_slot: &mut f64,
        var_vjcex_dn0_slot: &mut f64,
        var_vjcex_dn1_slot: &mut f64,
        var_vjcex_dn10_slot: &mut f64,
        var_vjcex_dn3_slot: &mut f64,
        var_vjcex_dn4_slot: &mut f64,
        var_vjcex_dn5_slot: &mut f64,
        var_vjcex_dn6_slot: &mut f64,
        var_vjcex_dn7_slot: &mut f64,
        var_vjcex_dn8_slot: &mut f64,
        var_vjcex_dn9_slot: &mut f64,
        var_vjcex_rv_slot: &mut f64,
        var_vje_s_slot: &mut f64,
        var_vje_s_dn0_slot: &mut f64,
        var_vje_s_dn1_slot: &mut f64,
        var_vje_s_dn10_slot: &mut f64,
        var_vje_s_dn3_slot: &mut f64,
        var_vje_s_dn4_slot: &mut f64,
        var_vje_s_dn5_slot: &mut f64,
        var_vje_s_dn6_slot: &mut f64,
        var_vje_s_dn7_slot: &mut f64,
        var_vje_s_dn8_slot: &mut f64,
        var_vje_s_dn9_slot: &mut f64,
        var_vje_s_rv_slot: &mut f64,
        var_vtexv_slot: &mut f64,
        var_vtexv_dn0_slot: &mut f64,
        var_vtexv_dn1_slot: &mut f64,
        var_vtexv_dn10_slot: &mut f64,
        var_vtexv_dn3_slot: &mut f64,
        var_vtexv_dn4_slot: &mut f64,
        var_vtexv_dn5_slot: &mut f64,
        var_vtexv_dn6_slot: &mut f64,
        var_vtexv_dn7_slot: &mut f64,
        var_vtexv_dn8_slot: &mut f64,
        var_vtexv_dn9_slot: &mut f64,
        var_vtexv_rv_slot: &mut f64,
        var_xqtex_slot: &mut f64,
        var_xqtex_dn0_slot: &mut f64,
        var_xqtex_dn1_slot: &mut f64,
        var_xqtex_dn10_slot: &mut f64,
        var_xqtex_dn3_slot: &mut f64,
        var_xqtex_dn4_slot: &mut f64,
        var_xqtex_dn5_slot: &mut f64,
        var_xqtex_dn6_slot: &mut f64,
        var_xqtex_dn7_slot: &mut f64,
        var_xqtex_dn8_slot: &mut f64,
        var_xqtex_dn9_slot: &mut f64,
        var_xqtex_rv_slot: &mut f64,
        var_xvjcex_slot: &mut f64,
        var_xvjcex_dn0_slot: &mut f64,
        var_xvjcex_dn1_slot: &mut f64,
        var_xvjcex_dn10_slot: &mut f64,
        var_xvjcex_dn3_slot: &mut f64,
        var_xvjcex_dn4_slot: &mut f64,
        var_xvjcex_dn5_slot: &mut f64,
        var_xvjcex_dn6_slot: &mut f64,
        var_xvjcex_dn7_slot: &mut f64,
        var_xvjcex_dn8_slot: &mut f64,
        var_xvjcex_dn9_slot: &mut f64,
        var_xvjcex_rv_slot: &mut f64,
        var_xvtexv_slot: &mut f64,
        var_xvtexv_dn0_slot: &mut f64,
        var_xvtexv_dn1_slot: &mut f64,
        var_xvtexv_dn10_slot: &mut f64,
        var_xvtexv_dn3_slot: &mut f64,
        var_xvtexv_dn4_slot: &mut f64,
        var_xvtexv_dn5_slot: &mut f64,
        var_xvtexv_dn6_slot: &mut f64,
        var_xvtexv_dn7_slot: &mut f64,
        var_xvtexv_dn8_slot: &mut f64,
        var_xvtexv_dn9_slot: &mut f64,
        var_xvtexv_rv_slot: &mut f64,
    ) {
        let mut var_a_vdcctc: f64 = *var_a_vdcctc_slot;
        let mut var_a_vdcctc_dn0: f64 = *var_a_vdcctc_dn0_slot;
        let mut var_a_vdcctc_dn1: f64 = *var_a_vdcctc_dn1_slot;
        let mut var_a_vdcctc_dn10: f64 = *var_a_vdcctc_dn10_slot;
        let mut var_a_vdcctc_dn3: f64 = *var_a_vdcctc_dn3_slot;
        let mut var_a_vdcctc_dn4: f64 = *var_a_vdcctc_dn4_slot;
        let mut var_a_vdcctc_dn5: f64 = *var_a_vdcctc_dn5_slot;
        let mut var_a_vdcctc_dn6: f64 = *var_a_vdcctc_dn6_slot;
        let mut var_a_vdcctc_dn7: f64 = *var_a_vdcctc_dn7_slot;
        let mut var_a_vdcctc_dn8: f64 = *var_a_vdcctc_dn8_slot;
        let mut var_a_vdcctc_dn9: f64 = *var_a_vdcctc_dn9_slot;
        let mut var_a_vdcctc_rv: f64 = *var_a_vdcctc_rv_slot;
        let mut var_dxa: f64 = *var_dxa_slot;
        let mut var_dxa_dn0: f64 = *var_dxa_dn0_slot;
        let mut var_dxa_dn1: f64 = *var_dxa_dn1_slot;
        let mut var_dxa_dn10: f64 = *var_dxa_dn10_slot;
        let mut var_dxa_dn3: f64 = *var_dxa_dn3_slot;
        let mut var_dxa_dn4: f64 = *var_dxa_dn4_slot;
        let mut var_dxa_dn5: f64 = *var_dxa_dn5_slot;
        let mut var_dxa_dn6: f64 = *var_dxa_dn6_slot;
        let mut var_dxa_dn7: f64 = *var_dxa_dn7_slot;
        let mut var_dxa_dn8: f64 = *var_dxa_dn8_slot;
        let mut var_dxa_dn9: f64 = *var_dxa_dn9_slot;
        let mut var_dxa_rv: f64 = *var_dxa_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_dn3: f64 = *var_qb0_dn3_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_qbc_qs: f64 = *var_qbc_qs_slot;
        let mut var_qbc_qs_dn0: f64 = *var_qbc_qs_dn0_slot;
        let mut var_qbc_qs_dn1: f64 = *var_qbc_qs_dn1_slot;
        let mut var_qbc_qs_dn10: f64 = *var_qbc_qs_dn10_slot;
        let mut var_qbc_qs_dn3: f64 = *var_qbc_qs_dn3_slot;
        let mut var_qbc_qs_dn4: f64 = *var_qbc_qs_dn4_slot;
        let mut var_qbc_qs_dn5: f64 = *var_qbc_qs_dn5_slot;
        let mut var_qbc_qs_dn6: f64 = *var_qbc_qs_dn6_slot;
        let mut var_qbc_qs_dn7: f64 = *var_qbc_qs_dn7_slot;
        let mut var_qbc_qs_dn8: f64 = *var_qbc_qs_dn8_slot;
        let mut var_qbc_qs_dn9: f64 = *var_qbc_qs_dn9_slot;
        let mut var_qbc_qs_rv: f64 = *var_qbc_qs_rv_slot;
        let mut var_qbe_qs: f64 = *var_qbe_qs_slot;
        let mut var_qbe_qs_dn0: f64 = *var_qbe_qs_dn0_slot;
        let mut var_qbe_qs_dn1: f64 = *var_qbe_qs_dn1_slot;
        let mut var_qbe_qs_dn10: f64 = *var_qbe_qs_dn10_slot;
        let mut var_qbe_qs_dn3: f64 = *var_qbe_qs_dn3_slot;
        let mut var_qbe_qs_dn4: f64 = *var_qbe_qs_dn4_slot;
        let mut var_qbe_qs_dn5: f64 = *var_qbe_qs_dn5_slot;
        let mut var_qbe_qs_dn6: f64 = *var_qbe_qs_dn6_slot;
        let mut var_qbe_qs_dn7: f64 = *var_qbe_qs_dn7_slot;
        let mut var_qbe_qs_dn8: f64 = *var_qbe_qs_dn8_slot;
        let mut var_qbe_qs_dn9: f64 = *var_qbe_qs_dn9_slot;
        let mut var_qbe_qs_rv: f64 = *var_qbe_qs_rv_slot;
        let mut var_qe0: f64 = *var_qe0_slot;
        let mut var_qe0_dn0: f64 = *var_qe0_dn0_slot;
        let mut var_qe0_dn1: f64 = *var_qe0_dn1_slot;
        let mut var_qe0_dn10: f64 = *var_qe0_dn10_slot;
        let mut var_qe0_dn3: f64 = *var_qe0_dn3_slot;
        let mut var_qe0_dn4: f64 = *var_qe0_dn4_slot;
        let mut var_qe0_dn5: f64 = *var_qe0_dn5_slot;
        let mut var_qe0_dn6: f64 = *var_qe0_dn6_slot;
        let mut var_qe0_dn7: f64 = *var_qe0_dn7_slot;
        let mut var_qe0_dn8: f64 = *var_qe0_dn8_slot;
        let mut var_qe0_dn9: f64 = *var_qe0_dn9_slot;
        let mut var_qe0_rv: f64 = *var_qe0_rv_slot;
        let mut var_qe_qs: f64 = *var_qe_qs_slot;
        let mut var_qe_qs_dn0: f64 = *var_qe_qs_dn0_slot;
        let mut var_qe_qs_dn1: f64 = *var_qe_qs_dn1_slot;
        let mut var_qe_qs_dn10: f64 = *var_qe_qs_dn10_slot;
        let mut var_qe_qs_dn3: f64 = *var_qe_qs_dn3_slot;
        let mut var_qe_qs_dn4: f64 = *var_qe_qs_dn4_slot;
        let mut var_qe_qs_dn5: f64 = *var_qe_qs_dn5_slot;
        let mut var_qe_qs_dn6: f64 = *var_qe_qs_dn6_slot;
        let mut var_qe_qs_dn7: f64 = *var_qe_qs_dn7_slot;
        let mut var_qe_qs_dn8: f64 = *var_qe_qs_dn8_slot;
        let mut var_qe_qs_dn9: f64 = *var_qe_qs_dn9_slot;
        let mut var_qe_qs_rv: f64 = *var_qe_qs_rv_slot;
        let mut var_qepi: f64 = *var_qepi_slot;
        let mut var_qepi0: f64 = *var_qepi0_slot;
        let mut var_qepi0_dn3: f64 = *var_qepi0_dn3_slot;
        let mut var_qepi0_rv: f64 = *var_qepi0_rv_slot;
        let mut var_qepi_dn0: f64 = *var_qepi_dn0_slot;
        let mut var_qepi_dn1: f64 = *var_qepi_dn1_slot;
        let mut var_qepi_dn10: f64 = *var_qepi_dn10_slot;
        let mut var_qepi_dn3: f64 = *var_qepi_dn3_slot;
        let mut var_qepi_dn4: f64 = *var_qepi_dn4_slot;
        let mut var_qepi_dn5: f64 = *var_qepi_dn5_slot;
        let mut var_qepi_dn6: f64 = *var_qepi_dn6_slot;
        let mut var_qepi_dn7: f64 = *var_qepi_dn7_slot;
        let mut var_qepi_dn8: f64 = *var_qepi_dn8_slot;
        let mut var_qepi_dn9: f64 = *var_qepi_dn9_slot;
        let mut var_qepi_rv: f64 = *var_qepi_rv_slot;
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_qex_rv: f64 = *var_qex_rv_slot;
        let mut var_qtc: f64 = *var_qtc_slot;
        let mut var_qtc_dn0: f64 = *var_qtc_dn0_slot;
        let mut var_qtc_dn1: f64 = *var_qtc_dn1_slot;
        let mut var_qtc_dn10: f64 = *var_qtc_dn10_slot;
        let mut var_qtc_dn3: f64 = *var_qtc_dn3_slot;
        let mut var_qtc_dn4: f64 = *var_qtc_dn4_slot;
        let mut var_qtc_dn5: f64 = *var_qtc_dn5_slot;
        let mut var_qtc_dn6: f64 = *var_qtc_dn6_slot;
        let mut var_qtc_dn7: f64 = *var_qtc_dn7_slot;
        let mut var_qtc_dn8: f64 = *var_qtc_dn8_slot;
        let mut var_qtc_dn9: f64 = *var_qtc_dn9_slot;
        let mut var_qtc_rv: f64 = *var_qtc_rv_slot;
        let mut var_qte_s: f64 = *var_qte_s_slot;
        let mut var_qte_s_dn0: f64 = *var_qte_s_dn0_slot;
        let mut var_qte_s_dn1: f64 = *var_qte_s_dn1_slot;
        let mut var_qte_s_dn10: f64 = *var_qte_s_dn10_slot;
        let mut var_qte_s_dn3: f64 = *var_qte_s_dn3_slot;
        let mut var_qte_s_dn4: f64 = *var_qte_s_dn4_slot;
        let mut var_qte_s_dn5: f64 = *var_qte_s_dn5_slot;
        let mut var_qte_s_dn6: f64 = *var_qte_s_dn6_slot;
        let mut var_qte_s_dn7: f64 = *var_qte_s_dn7_slot;
        let mut var_qte_s_dn8: f64 = *var_qte_s_dn8_slot;
        let mut var_qte_s_dn9: f64 = *var_qte_s_dn9_slot;
        let mut var_qte_s_rv: f64 = *var_qte_s_rv_slot;
        let mut var_qtex: f64 = *var_qtex_slot;
        let mut var_qtex_dn0: f64 = *var_qtex_dn0_slot;
        let mut var_qtex_dn1: f64 = *var_qtex_dn1_slot;
        let mut var_qtex_dn10: f64 = *var_qtex_dn10_slot;
        let mut var_qtex_dn3: f64 = *var_qtex_dn3_slot;
        let mut var_qtex_dn4: f64 = *var_qtex_dn4_slot;
        let mut var_qtex_dn5: f64 = *var_qtex_dn5_slot;
        let mut var_qtex_dn6: f64 = *var_qtex_dn6_slot;
        let mut var_qtex_dn7: f64 = *var_qtex_dn7_slot;
        let mut var_qtex_dn8: f64 = *var_qtex_dn8_slot;
        let mut var_qtex_dn9: f64 = *var_qtex_dn9_slot;
        let mut var_qtex_rv: f64 = *var_qtex_rv_slot;
        let mut var_tmpexp: f64 = *var_tmpexp_slot;
        let mut var_tmpexp_dn0: f64 = *var_tmpexp_dn0_slot;
        let mut var_tmpexp_dn1: f64 = *var_tmpexp_dn1_slot;
        let mut var_tmpexp_dn10: f64 = *var_tmpexp_dn10_slot;
        let mut var_tmpexp_dn3: f64 = *var_tmpexp_dn3_slot;
        let mut var_tmpexp_dn4: f64 = *var_tmpexp_dn4_slot;
        let mut var_tmpexp_dn5: f64 = *var_tmpexp_dn5_slot;
        let mut var_tmpexp_dn6: f64 = *var_tmpexp_dn6_slot;
        let mut var_tmpexp_dn7: f64 = *var_tmpexp_dn7_slot;
        let mut var_tmpexp_dn8: f64 = *var_tmpexp_dn8_slot;
        let mut var_tmpexp_dn9: f64 = *var_tmpexp_dn9_slot;
        let mut var_tmpexp_rv: f64 = *var_tmpexp_rv_slot;
        let mut var_vjcex: f64 = *var_vjcex_slot;
        let mut var_vjcex_dn0: f64 = *var_vjcex_dn0_slot;
        let mut var_vjcex_dn1: f64 = *var_vjcex_dn1_slot;
        let mut var_vjcex_dn10: f64 = *var_vjcex_dn10_slot;
        let mut var_vjcex_dn3: f64 = *var_vjcex_dn3_slot;
        let mut var_vjcex_dn4: f64 = *var_vjcex_dn4_slot;
        let mut var_vjcex_dn5: f64 = *var_vjcex_dn5_slot;
        let mut var_vjcex_dn6: f64 = *var_vjcex_dn6_slot;
        let mut var_vjcex_dn7: f64 = *var_vjcex_dn7_slot;
        let mut var_vjcex_dn8: f64 = *var_vjcex_dn8_slot;
        let mut var_vjcex_dn9: f64 = *var_vjcex_dn9_slot;
        let mut var_vjcex_rv: f64 = *var_vjcex_rv_slot;
        let mut var_vje_s: f64 = *var_vje_s_slot;
        let mut var_vje_s_dn0: f64 = *var_vje_s_dn0_slot;
        let mut var_vje_s_dn1: f64 = *var_vje_s_dn1_slot;
        let mut var_vje_s_dn10: f64 = *var_vje_s_dn10_slot;
        let mut var_vje_s_dn3: f64 = *var_vje_s_dn3_slot;
        let mut var_vje_s_dn4: f64 = *var_vje_s_dn4_slot;
        let mut var_vje_s_dn5: f64 = *var_vje_s_dn5_slot;
        let mut var_vje_s_dn6: f64 = *var_vje_s_dn6_slot;
        let mut var_vje_s_dn7: f64 = *var_vje_s_dn7_slot;
        let mut var_vje_s_dn8: f64 = *var_vje_s_dn8_slot;
        let mut var_vje_s_dn9: f64 = *var_vje_s_dn9_slot;
        let mut var_vje_s_rv: f64 = *var_vje_s_rv_slot;
        let mut var_vtexv: f64 = *var_vtexv_slot;
        let mut var_vtexv_dn0: f64 = *var_vtexv_dn0_slot;
        let mut var_vtexv_dn1: f64 = *var_vtexv_dn1_slot;
        let mut var_vtexv_dn10: f64 = *var_vtexv_dn10_slot;
        let mut var_vtexv_dn3: f64 = *var_vtexv_dn3_slot;
        let mut var_vtexv_dn4: f64 = *var_vtexv_dn4_slot;
        let mut var_vtexv_dn5: f64 = *var_vtexv_dn5_slot;
        let mut var_vtexv_dn6: f64 = *var_vtexv_dn6_slot;
        let mut var_vtexv_dn7: f64 = *var_vtexv_dn7_slot;
        let mut var_vtexv_dn8: f64 = *var_vtexv_dn8_slot;
        let mut var_vtexv_dn9: f64 = *var_vtexv_dn9_slot;
        let mut var_vtexv_rv: f64 = *var_vtexv_rv_slot;
        let mut var_xqtex: f64 = *var_xqtex_slot;
        let mut var_xqtex_dn0: f64 = *var_xqtex_dn0_slot;
        let mut var_xqtex_dn1: f64 = *var_xqtex_dn1_slot;
        let mut var_xqtex_dn10: f64 = *var_xqtex_dn10_slot;
        let mut var_xqtex_dn3: f64 = *var_xqtex_dn3_slot;
        let mut var_xqtex_dn4: f64 = *var_xqtex_dn4_slot;
        let mut var_xqtex_dn5: f64 = *var_xqtex_dn5_slot;
        let mut var_xqtex_dn6: f64 = *var_xqtex_dn6_slot;
        let mut var_xqtex_dn7: f64 = *var_xqtex_dn7_slot;
        let mut var_xqtex_dn8: f64 = *var_xqtex_dn8_slot;
        let mut var_xqtex_dn9: f64 = *var_xqtex_dn9_slot;
        let mut var_xqtex_rv: f64 = *var_xqtex_rv_slot;
        let mut var_xvjcex: f64 = *var_xvjcex_slot;
        let mut var_xvjcex_dn0: f64 = *var_xvjcex_dn0_slot;
        let mut var_xvjcex_dn1: f64 = *var_xvjcex_dn1_slot;
        let mut var_xvjcex_dn10: f64 = *var_xvjcex_dn10_slot;
        let mut var_xvjcex_dn3: f64 = *var_xvjcex_dn3_slot;
        let mut var_xvjcex_dn4: f64 = *var_xvjcex_dn4_slot;
        let mut var_xvjcex_dn5: f64 = *var_xvjcex_dn5_slot;
        let mut var_xvjcex_dn6: f64 = *var_xvjcex_dn6_slot;
        let mut var_xvjcex_dn7: f64 = *var_xvjcex_dn7_slot;
        let mut var_xvjcex_dn8: f64 = *var_xvjcex_dn8_slot;
        let mut var_xvjcex_dn9: f64 = *var_xvjcex_dn9_slot;
        let mut var_xvjcex_rv: f64 = *var_xvjcex_rv_slot;
        let mut var_xvtexv: f64 = *var_xvtexv_slot;
        let mut var_xvtexv_dn0: f64 = *var_xvtexv_dn0_slot;
        let mut var_xvtexv_dn1: f64 = *var_xvtexv_dn1_slot;
        let mut var_xvtexv_dn10: f64 = *var_xvtexv_dn10_slot;
        let mut var_xvtexv_dn3: f64 = *var_xvtexv_dn3_slot;
        let mut var_xvtexv_dn4: f64 = *var_xvtexv_dn4_slot;
        let mut var_xvtexv_dn5: f64 = *var_xvtexv_dn5_slot;
        let mut var_xvtexv_dn6: f64 = *var_xvtexv_dn6_slot;
        let mut var_xvtexv_dn7: f64 = *var_xvtexv_dn7_slot;
        let mut var_xvtexv_dn8: f64 = *var_xvtexv_dn8_slot;
        let mut var_xvtexv_dn9: f64 = *var_xvtexv_dn9_slot;
        let mut var_xvtexv_rv: f64 = *var_xvtexv_rv_slot;

        let (assign5770_e5882, assign5770_e5882_d_n0, assign5770_e5882_d_n1, assign5770_e5882_d_n3, assign5770_e5882_d_n4, assign5770_e5882_d_n5, assign5770_e5882_d_n6, assign5770_e5882_d_n7, assign5770_e5882_d_n8, assign5770_e5882_d_n9, assign5770_e5882_d_n10,) = {
    if (var_guard106 == 0.0) {
        let assign5770_e5875: f64 = (-var_dxa);
        let assign5770_e5876: f64 = (assign5770_e5875).exp();
        let assign5770_e5877: f64 = (1.0 + assign5770_e5876);
        let assign5770_e5878: f64 = (assign5770_e5877).ln();
        let assign5770_e5879: f64 = (var_a_vde * assign5770_e5878);
        let assign5770_e5880: f64 = (var_vfe - assign5770_e5879);
        (assign5770_e5880, (var_vfe_dn0 - ((var_a_vde_dn0 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn0)) / assign5770_e5877)))), (var_vfe_dn1 - ((var_a_vde_dn1 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn1)) / assign5770_e5877)))), (var_vfe_dn3 - ((var_a_vde_dn3 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn3)) / assign5770_e5877)))), (var_vfe_dn4 - ((var_a_vde_dn4 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn4)) / assign5770_e5877)))), (var_vfe_dn5 - ((var_a_vde_dn5 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn5)) / assign5770_e5877)))), (var_vfe_dn6 - ((var_a_vde_dn6 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn6)) / assign5770_e5877)))), (var_vfe_dn7 - ((var_a_vde_dn7 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn7)) / assign5770_e5877)))), (var_vfe_dn8 - ((var_a_vde_dn8 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn8)) / assign5770_e5877)))), (var_vfe_dn9 - ((var_a_vde_dn9 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn9)) / assign5770_e5877)))), (var_vfe_dn10 - ((var_a_vde_dn10 * assign5770_e5878) + (var_a_vde * ((assign5770_e5876 * (-var_dxa_dn10)) / assign5770_e5877)))),)
    } else {
        (var_vje_s, var_vje_s_dn0, var_vje_s_dn1, var_vje_s_dn3, var_vje_s_dn4, var_vje_s_dn5, var_vje_s_dn6, var_vje_s_dn7, var_vje_s_dn8, var_vje_s_dn9, var_vje_s_dn10,)
    }
};
        var_vje_s = assign5770_e5882;
        var_vje_s_dn0 = assign5770_e5882_d_n0;
        var_vje_s_dn1 = assign5770_e5882_d_n1;
        var_vje_s_dn3 = assign5770_e5882_d_n3;
        var_vje_s_dn4 = assign5770_e5882_d_n4;
        var_vje_s_dn5 = assign5770_e5882_d_n5;
        var_vje_s_dn6 = assign5770_e5882_d_n6;
        var_vje_s_dn7 = assign5770_e5882_d_n7;
        var_vje_s_dn8 = assign5770_e5882_d_n8;
        var_vje_s_dn9 = assign5770_e5882_d_n9;
        var_vje_s_dn10 = assign5770_e5882_d_n10;
        var_vje_s_rv = 0.0;

        let assign5780_e5885: f64 = (p.p67 * var_cje_t);
        let assign5780_e5889: f64 = (1.0 - p.p66);
        let assign5780_e5890: f64 = (var_vde_t / assign5780_e5889);
        let assign5780_e5895: f64 = (var_vje_s * var_inv_vde_t);
        let assign5780_e5896: f64 = (1.0 - assign5780_e5895);
        let assign5780_e5899: f64 = (1.0 - p.p66);
        let assign5780_e5900: f64 = (assign5780_e5896).powf(assign5780_e5899);
        let assign5780_e5901: f64 = (1.0 - assign5780_e5900);
        let assign5780_e5902: f64 = (assign5780_e5890 * assign5780_e5901);
        let assign5780_e5906: f64 = (var_vb1e1 - var_vje_s);
        let assign5780_e5907: f64 = (3.0 * assign5780_e5906);
        let assign5780_e5908: f64 = (assign5780_e5902 + assign5780_e5907);
        let assign5780_e5909: f64 = (assign5780_e5885 * assign5780_e5908);
        var_qte_s = assign5780_e5909;
        var_qte_s_dn0 = (((p.p67 * var_cje_t_dn0) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn0 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn0))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn0))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn0)))));
        var_qte_s_dn1 = (((p.p67 * var_cje_t_dn1) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn1 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn1))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn1))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn1)))));
        var_qte_s_dn3 = (((p.p67 * var_cje_t_dn3) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn3 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn3 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn3))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn3 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn3))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn3)))));
        var_qte_s_dn4 = (((p.p67 * var_cje_t_dn4) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn4 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn4 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn4))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn4 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn4))) / assign5780_e5896))) }))) + (3.0 * (var_vb1e1_dn4 - var_vje_s_dn4)))));
        var_qte_s_dn5 = (((p.p67 * var_cje_t_dn5) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn5 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn5 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn5))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn5 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn5))) / assign5780_e5896))) }))) + (3.0 * (var_vb1e1_dn5 - var_vje_s_dn5)))));
        var_qte_s_dn6 = (((p.p67 * var_cje_t_dn6) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn6 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn6 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn6))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn6 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn6))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn6)))));
        var_qte_s_dn7 = (((p.p67 * var_cje_t_dn7) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn7 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn7 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn7))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn7 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn7))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn7)))));
        var_qte_s_dn8 = (((p.p67 * var_cje_t_dn8) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn8 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn8 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn8))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn8 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn8))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn8)))));
        var_qte_s_dn9 = (((p.p67 * var_cje_t_dn9) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn9 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn9 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn9))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn9 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn9))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn9)))));
        var_qte_s_dn10 = (((p.p67 * var_cje_t_dn10) * assign5780_e5908) + (assign5780_e5885 * ((((var_vde_t_dn10 / assign5780_e5889) * assign5780_e5901) + (assign5780_e5890 * (-if 0.0 == 0.0 && ((assign5780_e5899) as f64).is_finite() && ((assign5780_e5899) as f64).fract() == 0.0 { if assign5780_e5899 == 0.0 { 0.0 } else { (assign5780_e5899 * ((assign5780_e5896).powf(assign5780_e5899 - 1.0) * (-((var_vje_s_dn10 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn10))))) } } else { (assign5780_e5900 * (assign5780_e5899 * ((-((var_vje_s_dn10 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn10))) / assign5780_e5896))) }))) + (3.0 * (-var_vje_s_dn10)))));
        var_qte_s_rv = 0.0;

        let assign5790_e5912: f64 = (p.p76 * var_cjc_t);
        let assign5790_e5914: f64 = (assign5790_e5912 * var_vtc);
        var_qtc = assign5790_e5914;
        var_qtc_dn0 = (((p.p76 * var_cjc_t_dn0) * var_vtc) + (assign5790_e5912 * var_vtc_dn0));
        var_qtc_dn1 = (((p.p76 * var_cjc_t_dn1) * var_vtc) + (assign5790_e5912 * var_vtc_dn1));
        var_qtc_dn3 = (((p.p76 * var_cjc_t_dn3) * var_vtc) + (assign5790_e5912 * var_vtc_dn3));
        var_qtc_dn4 = (((p.p76 * var_cjc_t_dn4) * var_vtc) + (assign5790_e5912 * var_vtc_dn4));
        var_qtc_dn5 = (((p.p76 * var_cjc_t_dn5) * var_vtc) + (assign5790_e5912 * var_vtc_dn5));
        var_qtc_dn6 = (((p.p76 * var_cjc_t_dn6) * var_vtc) + (assign5790_e5912 * var_vtc_dn6));
        var_qtc_dn7 = (((p.p76 * var_cjc_t_dn7) * var_vtc) + (assign5790_e5912 * var_vtc_dn7));
        var_qtc_dn8 = (((p.p76 * var_cjc_t_dn8) * var_vtc) + (assign5790_e5912 * var_vtc_dn8));
        var_qtc_dn9 = (((p.p76 * var_cjc_t_dn9) * var_vtc) + (assign5790_e5912 * var_vtc_dn9));
        var_qtc_dn10 = (((p.p76 * var_cjc_t_dn10) * var_vtc) + (assign5790_e5912 * var_vtc_dn10));
        var_qtc_rv = 0.0;

        let assign5800_e5917: f64 = (var_taub_t * var_ik_t);
        var_qb0 = assign5800_e5917;
        var_qb0_dn3 = ((var_taub_t_dn3 * var_ik_t) + (var_taub_t * var_ik_t_dn3));
        var_qb0_rv = 0.0;

        let assign5810_e5920: f64 = (0.5 * var_qb0);
        let assign5810_e5922: f64 = (assign5810_e5920 * var_n0);
        let assign5810_e5924: f64 = (assign5810_e5922 * var_q1q);
        var_qbe_qs = assign5810_e5924;
        var_qbe_qs_dn0 = (((assign5810_e5920 * var_n0_dn0) * var_q1q) + (assign5810_e5922 * var_q1q_dn0));
        var_qbe_qs_dn1 = (((assign5810_e5920 * var_n0_dn1) * var_q1q) + (assign5810_e5922 * var_q1q_dn1));
        var_qbe_qs_dn3 = (((((0.5 * var_qb0_dn3) * var_n0) + (assign5810_e5920 * var_n0_dn3)) * var_q1q) + (assign5810_e5922 * var_q1q_dn3));
        var_qbe_qs_dn4 = (((assign5810_e5920 * var_n0_dn4) * var_q1q) + (assign5810_e5922 * var_q1q_dn4));
        var_qbe_qs_dn5 = (((assign5810_e5920 * var_n0_dn5) * var_q1q) + (assign5810_e5922 * var_q1q_dn5));
        var_qbe_qs_dn6 = (((assign5810_e5920 * var_n0_dn6) * var_q1q) + (assign5810_e5922 * var_q1q_dn6));
        var_qbe_qs_dn7 = (((assign5810_e5920 * var_n0_dn7) * var_q1q) + (assign5810_e5922 * var_q1q_dn7));
        var_qbe_qs_dn8 = (((assign5810_e5920 * var_n0_dn8) * var_q1q) + (assign5810_e5922 * var_q1q_dn8));
        var_qbe_qs_dn9 = (((assign5810_e5920 * var_n0_dn9) * var_q1q) + (assign5810_e5922 * var_q1q_dn9));
        var_qbe_qs_dn10 = (((assign5810_e5920 * var_n0_dn10) * var_q1q) + (assign5810_e5922 * var_q1q_dn10));
        var_qbe_qs_rv = 0.0;

        let assign5820_e5927: f64 = (0.5 * var_qb0);
        let assign5820_e5929: f64 = (assign5820_e5927 * var_nb);
        let assign5820_e5931: f64 = (assign5820_e5929 * var_q1q);
        var_qbc_qs = assign5820_e5931;
        var_qbc_qs_dn0 = (((assign5820_e5927 * var_nb_dn0) * var_q1q) + (assign5820_e5929 * var_q1q_dn0));
        var_qbc_qs_dn1 = (((assign5820_e5927 * var_nb_dn1) * var_q1q) + (assign5820_e5929 * var_q1q_dn1));
        var_qbc_qs_dn3 = (((((0.5 * var_qb0_dn3) * var_nb) + (assign5820_e5927 * var_nb_dn3)) * var_q1q) + (assign5820_e5929 * var_q1q_dn3));
        var_qbc_qs_dn4 = (((assign5820_e5927 * var_nb_dn4) * var_q1q) + (assign5820_e5929 * var_q1q_dn4));
        var_qbc_qs_dn5 = (((assign5820_e5927 * var_nb_dn5) * var_q1q) + (assign5820_e5929 * var_q1q_dn5));
        var_qbc_qs_dn6 = (((assign5820_e5927 * var_nb_dn6) * var_q1q) + (assign5820_e5929 * var_q1q_dn6));
        var_qbc_qs_dn7 = (((assign5820_e5927 * var_nb_dn7) * var_q1q) + (assign5820_e5929 * var_q1q_dn7));
        var_qbc_qs_dn8 = (((assign5820_e5927 * var_nb_dn8) * var_q1q) + (assign5820_e5929 * var_q1q_dn8));
        var_qbc_qs_dn9 = (((assign5820_e5927 * var_nb_dn9) * var_q1q) + (assign5820_e5929 * var_q1q_dn9));
        var_qbc_qs_dn10 = (((assign5820_e5927 * var_nb_dn10) * var_q1q) + (assign5820_e5929 * var_q1q_dn10));
        var_qbc_qs_rv = 0.0;

        let assign5830_e5934: f64 = (0.1 * var_vdc_ctc_t);
        var_a_vdcctc = assign5830_e5934;
        var_a_vdcctc_dn0 = (0.1 * var_vdc_ctc_t_dn0);
        var_a_vdcctc_dn1 = (0.1 * var_vdc_ctc_t_dn1);
        var_a_vdcctc_dn3 = (0.1 * var_vdc_ctc_t_dn3);
        var_a_vdcctc_dn4 = (0.1 * var_vdc_ctc_t_dn4);
        var_a_vdcctc_dn5 = (0.1 * var_vdc_ctc_t_dn5);
        var_a_vdcctc_dn6 = (0.1 * var_vdc_ctc_t_dn6);
        var_a_vdcctc_dn7 = (0.1 * var_vdc_ctc_t_dn7);
        var_a_vdcctc_dn8 = (0.1 * var_vdc_ctc_t_dn8);
        var_a_vdcctc_dn9 = (0.1 * var_vdc_ctc_t_dn9);
        var_a_vdcctc_dn10 = (0.1 * var_vdc_ctc_t_dn10);
        var_a_vdcctc_rv = 0.0;

        let assign5840_e5937: f64 = (var_vb1c4 - var_vfc);
        let assign5840_e5939: f64 = (assign5840_e5937 / var_a_vdcctc);
        var_dxa = assign5840_e5939;
        var_dxa_dn0 = ((((-var_vfc_dn0) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn1 = ((((-var_vfc_dn1) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn3 = ((((-var_vfc_dn3) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn3)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn4 = ((((-var_vfc_dn4) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn4)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn5 = ((((var_vb1c4_dn5 - var_vfc_dn5) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn5)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn6 = ((((var_vb1c4_dn6 - var_vfc_dn6) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn6)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn7 = ((((var_vb1c4_dn7 - var_vfc_dn7) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn7)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn8 = ((((var_vb1c4_dn8 - var_vfc_dn8) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn8)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn9 = ((((-var_vfc_dn9) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn9)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn10 = ((((var_vb1c4_dn10 - var_vfc_dn10) * var_a_vdcctc) - (assign5840_e5937 * var_a_vdcctc_dn10)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_rv = 0.0;

        let assign5850_e5942: f64 = if var_vb1c4 < var_vfc { 1.0 } else { 0.0 };
        var_guard107 = assign5850_e5942;
        var_guard107_rv = 0.0;

        let (assign5860_e5954, assign5860_e5954_d_n0, assign5860_e5954_d_n1, assign5860_e5954_d_n3, assign5860_e5954_d_n4, assign5860_e5954_d_n5, assign5860_e5954_d_n6, assign5860_e5954_d_n7, assign5860_e5954_d_n8, assign5860_e5954_d_n9, assign5860_e5954_d_n10,) = {
    if (var_guard107 != 0.0) {
        let assign5860_e5948: f64 = (var_dxa).exp();
        let assign5860_e5949: f64 = (1.0 + assign5860_e5948);
        let assign5860_e5950: f64 = (assign5860_e5949).ln();
        let assign5860_e5951: f64 = (var_a_vdcctc * assign5860_e5950);
        let assign5860_e5952: f64 = (var_vb1c4 - assign5860_e5951);
        (assign5860_e5952, (-((var_a_vdcctc_dn0 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn0) / assign5860_e5949)))), (-((var_a_vdcctc_dn1 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn1) / assign5860_e5949)))), (-((var_a_vdcctc_dn3 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn3) / assign5860_e5949)))), (-((var_a_vdcctc_dn4 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn4) / assign5860_e5949)))), (var_vb1c4_dn5 - ((var_a_vdcctc_dn5 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn5) / assign5860_e5949)))), (var_vb1c4_dn6 - ((var_a_vdcctc_dn6 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn6) / assign5860_e5949)))), (var_vb1c4_dn7 - ((var_a_vdcctc_dn7 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn7) / assign5860_e5949)))), (var_vb1c4_dn8 - ((var_a_vdcctc_dn8 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn8) / assign5860_e5949)))), (-((var_a_vdcctc_dn9 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn9) / assign5860_e5949)))), (var_vb1c4_dn10 - ((var_a_vdcctc_dn10 * assign5860_e5950) + (var_a_vdcctc * ((assign5860_e5948 * var_dxa_dn10) / assign5860_e5949)))),)
    } else {
        (var_vjcex, var_vjcex_dn0, var_vjcex_dn1, var_vjcex_dn3, var_vjcex_dn4, var_vjcex_dn5, var_vjcex_dn6, var_vjcex_dn7, var_vjcex_dn8, var_vjcex_dn9, var_vjcex_dn10,)
    }
};
        var_vjcex = assign5860_e5954;
        var_vjcex_dn0 = assign5860_e5954_d_n0;
        var_vjcex_dn1 = assign5860_e5954_d_n1;
        var_vjcex_dn3 = assign5860_e5954_d_n3;
        var_vjcex_dn4 = assign5860_e5954_d_n4;
        var_vjcex_dn5 = assign5860_e5954_d_n5;
        var_vjcex_dn6 = assign5860_e5954_d_n6;
        var_vjcex_dn7 = assign5860_e5954_d_n7;
        var_vjcex_dn8 = assign5860_e5954_d_n8;
        var_vjcex_dn9 = assign5860_e5954_d_n9;
        var_vjcex_dn10 = assign5860_e5954_d_n10;
        var_vjcex_rv = 0.0;

        let (assign5870_e5968, assign5870_e5968_d_n0, assign5870_e5968_d_n1, assign5870_e5968_d_n3, assign5870_e5968_d_n4, assign5870_e5968_d_n5, assign5870_e5968_d_n6, assign5870_e5968_d_n7, assign5870_e5968_d_n8, assign5870_e5968_d_n9, assign5870_e5968_d_n10,) = {
    if (var_guard107 == 0.0) {
        let assign5870_e5961: f64 = (-var_dxa);
        let assign5870_e5962: f64 = (assign5870_e5961).exp();
        let assign5870_e5963: f64 = (1.0 + assign5870_e5962);
        let assign5870_e5964: f64 = (assign5870_e5963).ln();
        let assign5870_e5965: f64 = (var_a_vdcctc * assign5870_e5964);
        let assign5870_e5966: f64 = (var_vfc - assign5870_e5965);
        (assign5870_e5966, (var_vfc_dn0 - ((var_a_vdcctc_dn0 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn0)) / assign5870_e5963)))), (var_vfc_dn1 - ((var_a_vdcctc_dn1 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn1)) / assign5870_e5963)))), (var_vfc_dn3 - ((var_a_vdcctc_dn3 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn3)) / assign5870_e5963)))), (var_vfc_dn4 - ((var_a_vdcctc_dn4 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn4)) / assign5870_e5963)))), (var_vfc_dn5 - ((var_a_vdcctc_dn5 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn5)) / assign5870_e5963)))), (var_vfc_dn6 - ((var_a_vdcctc_dn6 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn6)) / assign5870_e5963)))), (var_vfc_dn7 - ((var_a_vdcctc_dn7 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn7)) / assign5870_e5963)))), (var_vfc_dn8 - ((var_a_vdcctc_dn8 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn8)) / assign5870_e5963)))), (var_vfc_dn9 - ((var_a_vdcctc_dn9 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn9)) / assign5870_e5963)))), (var_vfc_dn10 - ((var_a_vdcctc_dn10 * assign5870_e5964) + (var_a_vdcctc * ((assign5870_e5962 * (-var_dxa_dn10)) / assign5870_e5963)))),)
    } else {
        (var_vjcex, var_vjcex_dn0, var_vjcex_dn1, var_vjcex_dn3, var_vjcex_dn4, var_vjcex_dn5, var_vjcex_dn6, var_vjcex_dn7, var_vjcex_dn8, var_vjcex_dn9, var_vjcex_dn10,)
    }
};
        var_vjcex = assign5870_e5968;
        var_vjcex_dn0 = assign5870_e5968_d_n0;
        var_vjcex_dn1 = assign5870_e5968_d_n1;
        var_vjcex_dn3 = assign5870_e5968_d_n3;
        var_vjcex_dn4 = assign5870_e5968_d_n4;
        var_vjcex_dn5 = assign5870_e5968_d_n5;
        var_vjcex_dn6 = assign5870_e5968_d_n6;
        var_vjcex_dn7 = assign5870_e5968_d_n7;
        var_vjcex_dn8 = assign5870_e5968_d_n8;
        var_vjcex_dn9 = assign5870_e5968_d_n9;
        var_vjcex_dn10 = assign5870_e5968_d_n10;
        var_vjcex_rv = 0.0;

        let assign5880_e5972: f64 = (1.0 - p.p71);
        let assign5880_e5973: f64 = (var_vdc_ctc_t / assign5880_e5972);
        let assign5880_e5978: f64 = (var_vjcex / var_vdc_ctc_t);
        let assign5880_e5979: f64 = (1.0 - assign5880_e5978);
        let assign5880_e5982: f64 = (1.0 - p.p71);
        let assign5880_e5983: f64 = (assign5880_e5979).powf(assign5880_e5982);
        let assign5880_e5984: f64 = (1.0 - assign5880_e5983);
        let assign5880_e5985: f64 = (assign5880_e5973 * assign5880_e5984);
        let assign5880_e5989: f64 = (var_vb1c4 - var_vjcex);
        let assign5880_e5990: f64 = (var_bjc * assign5880_e5989);
        let assign5880_e5991: f64 = (assign5880_e5985 + assign5880_e5990);
        var_vtexv = assign5880_e5991;
        var_vtexv_dn0 = ((((var_vdc_ctc_t_dn0 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn0 * assign5880_e5989) + (var_bjc * (-var_vjcex_dn0))));
        var_vtexv_dn1 = ((((var_vdc_ctc_t_dn1 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn1 * assign5880_e5989) + (var_bjc * (-var_vjcex_dn1))));
        var_vtexv_dn3 = ((((var_vdc_ctc_t_dn3 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn3 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn3 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn3 * assign5880_e5989) + (var_bjc * (-var_vjcex_dn3))));
        var_vtexv_dn4 = ((((var_vdc_ctc_t_dn4 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn4 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn4 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn4 * assign5880_e5989) + (var_bjc * (-var_vjcex_dn4))));
        var_vtexv_dn5 = ((((var_vdc_ctc_t_dn5 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn5 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn5 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn5 * assign5880_e5989) + (var_bjc * (var_vb1c4_dn5 - var_vjcex_dn5))));
        var_vtexv_dn6 = ((((var_vdc_ctc_t_dn6 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn6 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn6 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn6 * assign5880_e5989) + (var_bjc * (var_vb1c4_dn6 - var_vjcex_dn6))));
        var_vtexv_dn7 = ((((var_vdc_ctc_t_dn7 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn7 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn7 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn7 * assign5880_e5989) + (var_bjc * (var_vb1c4_dn7 - var_vjcex_dn7))));
        var_vtexv_dn8 = ((((var_vdc_ctc_t_dn8 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn8 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn8 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn8 * assign5880_e5989) + (var_bjc * (var_vb1c4_dn8 - var_vjcex_dn8))));
        var_vtexv_dn9 = ((((var_vdc_ctc_t_dn9 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn9 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn9 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn9 * assign5880_e5989) + (var_bjc * (-var_vjcex_dn9))));
        var_vtexv_dn10 = ((((var_vdc_ctc_t_dn10 / assign5880_e5972) * assign5880_e5984) + (assign5880_e5973 * (-if 0.0 == 0.0 && ((assign5880_e5982) as f64).is_finite() && ((assign5880_e5982) as f64).fract() == 0.0 { if assign5880_e5982 == 0.0 { 0.0 } else { (assign5880_e5982 * ((assign5880_e5979).powf(assign5880_e5982 - 1.0) * (-(((var_vjcex_dn10 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5880_e5983 * (assign5880_e5982 * ((-(((var_vjcex_dn10 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5880_e5979))) }))) + ((var_bjc_dn10 * assign5880_e5989) + (var_bjc * (var_vb1c4_dn10 - var_vjcex_dn10))));
        var_vtexv_rv = 0.0;

        let assign5890_e5995: f64 = (1.0 - var_xp_t);
        let assign5890_e5997: f64 = (assign5890_e5995 * var_vtexv);
        let assign5890_e6000: f64 = (var_xp_t * var_vb1c4);
        let assign5890_e6001: f64 = (assign5890_e5997 + assign5890_e6000);
        let assign5890_e6002: f64 = (var_cjc_t * assign5890_e6001);
        let assign5890_e6005: f64 = (1.0 - p.p76);
        let assign5890_e6006: f64 = (assign5890_e6002 * assign5890_e6005);
        let assign5890_e6009: f64 = (1.0 - p.p32);
        let assign5890_e6010: f64 = (assign5890_e6006 * assign5890_e6009);
        var_qtex = assign5890_e6010;
        var_qtex_dn0 = ((((var_cjc_t_dn0 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn0) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn0)) + (var_xp_t_dn0 * var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn1 = ((((var_cjc_t_dn1 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn1) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn1)) + (var_xp_t_dn1 * var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn3 = ((((var_cjc_t_dn3 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn3) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn3)) + (var_xp_t_dn3 * var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn4 = ((((var_cjc_t_dn4 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn4) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn4)) + (var_xp_t_dn4 * var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn5 = ((((var_cjc_t_dn5 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn5) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn5)) + ((var_xp_t_dn5 * var_vb1c4) + (var_xp_t * var_vb1c4_dn5))))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn6 = ((((var_cjc_t_dn6 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn6) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn6)) + ((var_xp_t_dn6 * var_vb1c4) + (var_xp_t * var_vb1c4_dn6))))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn7 = ((((var_cjc_t_dn7 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn7) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn7)) + ((var_xp_t_dn7 * var_vb1c4) + (var_xp_t * var_vb1c4_dn7))))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn8 = ((((var_cjc_t_dn8 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn8) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn8)) + ((var_xp_t_dn8 * var_vb1c4) + (var_xp_t * var_vb1c4_dn8))))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn9 = ((((var_cjc_t_dn9 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn9) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn9)) + (var_xp_t_dn9 * var_vb1c4)))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_dn10 = ((((var_cjc_t_dn10 * assign5890_e6001) + (var_cjc_t * ((((-var_xp_t_dn10) * var_vtexv) + (assign5890_e5995 * var_vtexv_dn10)) + ((var_xp_t_dn10 * var_vb1c4) + (var_xp_t * var_vb1c4_dn10))))) * assign5890_e6005) * assign5890_e6009);
        var_qtex_rv = 0.0;

        let assign5900_e6013: f64 = (var_vbc3 - var_vfc);
        let assign5900_e6015: f64 = (assign5900_e6013 / var_a_vdcctc);
        var_dxa = assign5900_e6015;
        var_dxa_dn0 = ((((var_vbc3_dn0 - var_vfc_dn0) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn1 = ((((var_vbc3_dn1 - var_vfc_dn1) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn3 = ((((-var_vfc_dn3) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn3)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn4 = ((((-var_vfc_dn4) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn4)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn5 = ((((var_vbc3_dn5 - var_vfc_dn5) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn5)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn6 = ((((var_vbc3_dn6 - var_vfc_dn6) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn6)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn7 = ((((var_vbc3_dn7 - var_vfc_dn7) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn7)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn8 = ((((var_vbc3_dn8 - var_vfc_dn8) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn8)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn9 = ((((var_vbc3_dn9 - var_vfc_dn9) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn9)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn10 = ((((var_vbc3_dn10 - var_vfc_dn10) * var_a_vdcctc) - (assign5900_e6013 * var_a_vdcctc_dn10)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_rv = 0.0;

        let assign5910_e6018: f64 = if var_vbc3 < var_vfc { 1.0 } else { 0.0 };
        var_guard108 = assign5910_e6018;
        var_guard108_rv = 0.0;

        let (assign5920_e6030, assign5920_e6030_d_n0, assign5920_e6030_d_n1, assign5920_e6030_d_n3, assign5920_e6030_d_n4, assign5920_e6030_d_n5, assign5920_e6030_d_n6, assign5920_e6030_d_n7, assign5920_e6030_d_n8, assign5920_e6030_d_n9, assign5920_e6030_d_n10,) = {
    if (var_guard108 != 0.0) {
        let assign5920_e6024: f64 = (var_dxa).exp();
        let assign5920_e6025: f64 = (1.0 + assign5920_e6024);
        let assign5920_e6026: f64 = (assign5920_e6025).ln();
        let assign5920_e6027: f64 = (var_a_vdcctc * assign5920_e6026);
        let assign5920_e6028: f64 = (var_vbc3 - assign5920_e6027);
        (assign5920_e6028, (var_vbc3_dn0 - ((var_a_vdcctc_dn0 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn0) / assign5920_e6025)))), (var_vbc3_dn1 - ((var_a_vdcctc_dn1 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn1) / assign5920_e6025)))), (-((var_a_vdcctc_dn3 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn3) / assign5920_e6025)))), (-((var_a_vdcctc_dn4 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn4) / assign5920_e6025)))), (var_vbc3_dn5 - ((var_a_vdcctc_dn5 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn5) / assign5920_e6025)))), (var_vbc3_dn6 - ((var_a_vdcctc_dn6 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn6) / assign5920_e6025)))), (var_vbc3_dn7 - ((var_a_vdcctc_dn7 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn7) / assign5920_e6025)))), (var_vbc3_dn8 - ((var_a_vdcctc_dn8 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn8) / assign5920_e6025)))), (var_vbc3_dn9 - ((var_a_vdcctc_dn9 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn9) / assign5920_e6025)))), (var_vbc3_dn10 - ((var_a_vdcctc_dn10 * assign5920_e6026) + (var_a_vdcctc * ((assign5920_e6024 * var_dxa_dn10) / assign5920_e6025)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10,)
    }
};
        var_xvjcex = assign5920_e6030;
        var_xvjcex_dn0 = assign5920_e6030_d_n0;
        var_xvjcex_dn1 = assign5920_e6030_d_n1;
        var_xvjcex_dn3 = assign5920_e6030_d_n3;
        var_xvjcex_dn4 = assign5920_e6030_d_n4;
        var_xvjcex_dn5 = assign5920_e6030_d_n5;
        var_xvjcex_dn6 = assign5920_e6030_d_n6;
        var_xvjcex_dn7 = assign5920_e6030_d_n7;
        var_xvjcex_dn8 = assign5920_e6030_d_n8;
        var_xvjcex_dn9 = assign5920_e6030_d_n9;
        var_xvjcex_dn10 = assign5920_e6030_d_n10;
        var_xvjcex_rv = 0.0;

        let (assign5930_e6044, assign5930_e6044_d_n0, assign5930_e6044_d_n1, assign5930_e6044_d_n3, assign5930_e6044_d_n4, assign5930_e6044_d_n5, assign5930_e6044_d_n6, assign5930_e6044_d_n7, assign5930_e6044_d_n8, assign5930_e6044_d_n9, assign5930_e6044_d_n10,) = {
    if (var_guard108 == 0.0) {
        let assign5930_e6037: f64 = (-var_dxa);
        let assign5930_e6038: f64 = (assign5930_e6037).exp();
        let assign5930_e6039: f64 = (1.0 + assign5930_e6038);
        let assign5930_e6040: f64 = (assign5930_e6039).ln();
        let assign5930_e6041: f64 = (var_a_vdcctc * assign5930_e6040);
        let assign5930_e6042: f64 = (var_vfc - assign5930_e6041);
        (assign5930_e6042, (var_vfc_dn0 - ((var_a_vdcctc_dn0 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn0)) / assign5930_e6039)))), (var_vfc_dn1 - ((var_a_vdcctc_dn1 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn1)) / assign5930_e6039)))), (var_vfc_dn3 - ((var_a_vdcctc_dn3 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn3)) / assign5930_e6039)))), (var_vfc_dn4 - ((var_a_vdcctc_dn4 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn4)) / assign5930_e6039)))), (var_vfc_dn5 - ((var_a_vdcctc_dn5 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn5)) / assign5930_e6039)))), (var_vfc_dn6 - ((var_a_vdcctc_dn6 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn6)) / assign5930_e6039)))), (var_vfc_dn7 - ((var_a_vdcctc_dn7 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn7)) / assign5930_e6039)))), (var_vfc_dn8 - ((var_a_vdcctc_dn8 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn8)) / assign5930_e6039)))), (var_vfc_dn9 - ((var_a_vdcctc_dn9 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn9)) / assign5930_e6039)))), (var_vfc_dn10 - ((var_a_vdcctc_dn10 * assign5930_e6040) + (var_a_vdcctc * ((assign5930_e6038 * (-var_dxa_dn10)) / assign5930_e6039)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10,)
    }
};
        var_xvjcex = assign5930_e6044;
        var_xvjcex_dn0 = assign5930_e6044_d_n0;
        var_xvjcex_dn1 = assign5930_e6044_d_n1;
        var_xvjcex_dn3 = assign5930_e6044_d_n3;
        var_xvjcex_dn4 = assign5930_e6044_d_n4;
        var_xvjcex_dn5 = assign5930_e6044_d_n5;
        var_xvjcex_dn6 = assign5930_e6044_d_n6;
        var_xvjcex_dn7 = assign5930_e6044_d_n7;
        var_xvjcex_dn8 = assign5930_e6044_d_n8;
        var_xvjcex_dn9 = assign5930_e6044_d_n9;
        var_xvjcex_dn10 = assign5930_e6044_d_n10;
        var_xvjcex_rv = 0.0;

        let assign5940_e6048: f64 = (1.0 - p.p71);
        let assign5940_e6049: f64 = (var_vdc_ctc_t / assign5940_e6048);
        let assign5940_e6054: f64 = (var_xvjcex / var_vdc_ctc_t);
        let assign5940_e6055: f64 = (1.0 - assign5940_e6054);
        let assign5940_e6058: f64 = (1.0 - p.p71);
        let assign5940_e6059: f64 = (assign5940_e6055).powf(assign5940_e6058);
        let assign5940_e6060: f64 = (1.0 - assign5940_e6059);
        let assign5940_e6061: f64 = (assign5940_e6049 * assign5940_e6060);
        let assign5940_e6065: f64 = (var_vbc3 - var_xvjcex);
        let assign5940_e6066: f64 = (var_bjc * assign5940_e6065);
        let assign5940_e6067: f64 = (assign5940_e6061 + assign5940_e6066);
        var_xvtexv = assign5940_e6067;
        var_xvtexv_dn0 = ((((var_vdc_ctc_t_dn0 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn0 * assign5940_e6065) + (var_bjc * (var_vbc3_dn0 - var_xvjcex_dn0))));
        var_xvtexv_dn1 = ((((var_vdc_ctc_t_dn1 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn1 * assign5940_e6065) + (var_bjc * (var_vbc3_dn1 - var_xvjcex_dn1))));
        var_xvtexv_dn3 = ((((var_vdc_ctc_t_dn3 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn3 * assign5940_e6065) + (var_bjc * (-var_xvjcex_dn3))));
        var_xvtexv_dn4 = ((((var_vdc_ctc_t_dn4 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn4 * assign5940_e6065) + (var_bjc * (-var_xvjcex_dn4))));
        var_xvtexv_dn5 = ((((var_vdc_ctc_t_dn5 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn5 * assign5940_e6065) + (var_bjc * (var_vbc3_dn5 - var_xvjcex_dn5))));
        var_xvtexv_dn6 = ((((var_vdc_ctc_t_dn6 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn6 * assign5940_e6065) + (var_bjc * (var_vbc3_dn6 - var_xvjcex_dn6))));
        var_xvtexv_dn7 = ((((var_vdc_ctc_t_dn7 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn7 * assign5940_e6065) + (var_bjc * (var_vbc3_dn7 - var_xvjcex_dn7))));
        var_xvtexv_dn8 = ((((var_vdc_ctc_t_dn8 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn8 * assign5940_e6065) + (var_bjc * (var_vbc3_dn8 - var_xvjcex_dn8))));
        var_xvtexv_dn9 = ((((var_vdc_ctc_t_dn9 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn9 * assign5940_e6065) + (var_bjc * (var_vbc3_dn9 - var_xvjcex_dn9))));
        var_xvtexv_dn10 = ((((var_vdc_ctc_t_dn10 / assign5940_e6048) * assign5940_e6060) + (assign5940_e6049 * (-if 0.0 == 0.0 && ((assign5940_e6058) as f64).is_finite() && ((assign5940_e6058) as f64).fract() == 0.0 { if assign5940_e6058 == 0.0 { 0.0 } else { (assign5940_e6058 * ((assign5940_e6055).powf(assign5940_e6058 - 1.0) * (-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5940_e6059 * (assign5940_e6058 * ((-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5940_e6055))) }))) + ((var_bjc_dn10 * assign5940_e6065) + (var_bjc * (var_vbc3_dn10 - var_xvjcex_dn10))));
        var_xvtexv_rv = 0.0;

        let assign5950_e6071: f64 = (1.0 - var_xp_t);
        let assign5950_e6073: f64 = (assign5950_e6071 * var_xvtexv);
        let assign5950_e6076: f64 = (var_xp_t * var_vbc3);
        let assign5950_e6077: f64 = (assign5950_e6073 + assign5950_e6076);
        let assign5950_e6078: f64 = (var_cjc_t * assign5950_e6077);
        let assign5950_e6081: f64 = (1.0 - p.p76);
        let assign5950_e6082: f64 = (assign5950_e6078 * assign5950_e6081);
        let assign5950_e6084: f64 = (assign5950_e6082 * p.p32);
        var_xqtex = assign5950_e6084;
        var_xqtex_dn0 = ((((var_cjc_t_dn0 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn0) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn0)) + ((var_xp_t_dn0 * var_vbc3) + (var_xp_t * var_vbc3_dn0))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn1 = ((((var_cjc_t_dn1 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn1) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn1)) + ((var_xp_t_dn1 * var_vbc3) + (var_xp_t * var_vbc3_dn1))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn3 = ((((var_cjc_t_dn3 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn3) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn3)) + (var_xp_t_dn3 * var_vbc3)))) * assign5950_e6081) * p.p32);
        var_xqtex_dn4 = ((((var_cjc_t_dn4 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn4) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn4)) + (var_xp_t_dn4 * var_vbc3)))) * assign5950_e6081) * p.p32);
        var_xqtex_dn5 = ((((var_cjc_t_dn5 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn5) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn5)) + ((var_xp_t_dn5 * var_vbc3) + (var_xp_t * var_vbc3_dn5))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn6 = ((((var_cjc_t_dn6 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn6) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn6)) + ((var_xp_t_dn6 * var_vbc3) + (var_xp_t * var_vbc3_dn6))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn7 = ((((var_cjc_t_dn7 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn7) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn7)) + ((var_xp_t_dn7 * var_vbc3) + (var_xp_t * var_vbc3_dn7))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn8 = ((((var_cjc_t_dn8 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn8) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn8)) + ((var_xp_t_dn8 * var_vbc3) + (var_xp_t * var_vbc3_dn8))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn9 = ((((var_cjc_t_dn9 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn9) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn9)) + ((var_xp_t_dn9 * var_vbc3) + (var_xp_t * var_vbc3_dn9))))) * assign5950_e6081) * p.p32);
        var_xqtex_dn10 = ((((var_cjc_t_dn10 * assign5950_e6077) + (var_cjc_t * ((((-var_xp_t_dn10) * var_xvtexv) + (assign5950_e6071 * var_xvtexv_dn10)) + ((var_xp_t_dn10 * var_vbc3) + (var_xp_t * var_vbc3_dn10))))) * assign5950_e6081) * p.p32);
        var_xqtex_rv = 0.0;

        let assign5960_e6087: f64 = (var_taue_t * var_ik_t);
        let assign5960_e6090: f64 = (var_is_t / var_ik_t);
        let assign5960_e6093: f64 = (1.0 / p.p84);
        let assign5960_e6094: f64 = (assign5960_e6090).powf(assign5960_e6093);
        let assign5960_e6095: f64 = (assign5960_e6087 * assign5960_e6094);
        var_qe0 = assign5960_e6095;
        var_qe0_dn0 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn0 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn0 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn1 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn1 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn1 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn3 = ((((var_taue_t_dn3 * var_ik_t) + (var_taue_t * var_ik_t_dn3)) * assign5960_e6094) + (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (((var_is_t_dn3 * var_ik_t) - (var_is_t * var_ik_t_dn3)) / (var_ik_t * var_ik_t)))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((((var_is_t_dn3 * var_ik_t) - (var_is_t * var_ik_t_dn3)) / (var_ik_t * var_ik_t)) / assign5960_e6090))) }));
        var_qe0_dn4 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn4 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn4 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn5 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn5 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn5 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn6 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn6 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn6 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn7 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn7 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn7 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn8 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn8 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn8 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn9 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn9 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn9 / var_ik_t) / assign5960_e6090))) });
        var_qe0_dn10 = (assign5960_e6087 * if 0.0 == 0.0 && ((assign5960_e6093) as f64).is_finite() && ((assign5960_e6093) as f64).fract() == 0.0 { if assign5960_e6093 == 0.0 { 0.0 } else { (assign5960_e6093 * ((assign5960_e6090).powf(assign5960_e6093 - 1.0) * (var_is_t_dn10 / var_ik_t))) } } else { (assign5960_e6094 * (assign5960_e6093 * ((var_is_t_dn10 / var_ik_t) / assign5960_e6090))) });
        var_qe0_rv = 0.0;

        let assign5970_e6099: f64 = (p.p84 * var_vt);
        let assign5970_e6100: f64 = (var_vb2e1 / assign5970_e6099);
        let assign5970_e6102: f64 = if assign5970_e6100 < p.p138 { 1.0 } else { 0.0 };
        var_guard109 = assign5970_e6102;
        var_guard109_rv = 0.0;

        let (assign5980_e6111, assign5980_e6111_d_n0, assign5980_e6111_d_n1, assign5980_e6111_d_n3, assign5980_e6111_d_n4, assign5980_e6111_d_n5, assign5980_e6111_d_n6, assign5980_e6111_d_n7, assign5980_e6111_d_n8, assign5980_e6111_d_n9, assign5980_e6111_d_n10,) = {
    if (var_guard109 != 0.0) {
        let assign5980_e6107: f64 = (p.p84 * var_vt);
        let assign5980_e6108: f64 = (var_vb2e1 / assign5980_e6107);
        let assign5980_e6109: f64 = (assign5980_e6108).exp();
        (assign5980_e6109, 0.0, 0.0, (assign5980_e6109 * (-((var_vb2e1 * (p.p84 * var_vt_dn3)) / (assign5980_e6107 * assign5980_e6107)))), (assign5980_e6109 * (var_vb2e1_dn4 / assign5980_e6107)), 0.0, (assign5980_e6109 * (var_vb2e1_dn6 / assign5980_e6107)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10,)
    }
};
        var_tmpexp = assign5980_e6111;
        var_tmpexp_dn0 = assign5980_e6111_d_n0;
        var_tmpexp_dn1 = assign5980_e6111_d_n1;
        var_tmpexp_dn3 = assign5980_e6111_d_n3;
        var_tmpexp_dn4 = assign5980_e6111_d_n4;
        var_tmpexp_dn5 = assign5980_e6111_d_n5;
        var_tmpexp_dn6 = assign5980_e6111_d_n6;
        var_tmpexp_dn7 = assign5980_e6111_d_n7;
        var_tmpexp_dn8 = assign5980_e6111_d_n8;
        var_tmpexp_dn9 = assign5980_e6111_d_n9;
        var_tmpexp_dn10 = assign5980_e6111_d_n10;
        var_tmpexp_rv = 0.0;

        let (assign5990_e6117,) = {
    if (var_guard109 == 0.0) {
        let assign5990_e6115: f64 = (p.p138).exp();
        (assign5990_e6115,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign5990_e6117;
        var_expl_rv = 0.0;

        let (assign6000_e6132, assign6000_e6132_d_n0, assign6000_e6132_d_n1, assign6000_e6132_d_n3, assign6000_e6132_d_n4, assign6000_e6132_d_n5, assign6000_e6132_d_n6, assign6000_e6132_d_n7, assign6000_e6132_d_n8, assign6000_e6132_d_n9, assign6000_e6132_d_n10,) = {
    if (var_guard109 == 0.0) {
        let assign6000_e6125: f64 = (p.p84 * var_vt);
        let assign6000_e6126: f64 = (var_vb2e1 / assign6000_e6125);
        let assign6000_e6128: f64 = (assign6000_e6126 - p.p138);
        let assign6000_e6129: f64 = (1.0 + assign6000_e6128);
        let assign6000_e6130: f64 = (var_expl * assign6000_e6129);
        (assign6000_e6130, 0.0, 0.0, (var_expl * (-((var_vb2e1 * (p.p84 * var_vt_dn3)) / (assign6000_e6125 * assign6000_e6125)))), (var_expl * (var_vb2e1_dn4 / assign6000_e6125)), 0.0, (var_expl * (var_vb2e1_dn6 / assign6000_e6125)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10,)
    }
};
        var_tmpexp = assign6000_e6132;
        var_tmpexp_dn0 = assign6000_e6132_d_n0;
        var_tmpexp_dn1 = assign6000_e6132_d_n1;
        var_tmpexp_dn3 = assign6000_e6132_d_n3;
        var_tmpexp_dn4 = assign6000_e6132_d_n4;
        var_tmpexp_dn5 = assign6000_e6132_d_n5;
        var_tmpexp_dn6 = assign6000_e6132_d_n6;
        var_tmpexp_dn7 = assign6000_e6132_d_n7;
        var_tmpexp_dn8 = assign6000_e6132_d_n8;
        var_tmpexp_dn9 = assign6000_e6132_d_n9;
        var_tmpexp_dn10 = assign6000_e6132_d_n10;
        var_tmpexp_rv = 0.0;

        let assign6010_e6135: f64 = (var_qe0 * var_tmpexp);
        var_qe_qs = assign6010_e6135;
        var_qe_qs_dn0 = ((var_qe0_dn0 * var_tmpexp) + (var_qe0 * var_tmpexp_dn0));
        var_qe_qs_dn1 = ((var_qe0_dn1 * var_tmpexp) + (var_qe0 * var_tmpexp_dn1));
        var_qe_qs_dn3 = ((var_qe0_dn3 * var_tmpexp) + (var_qe0 * var_tmpexp_dn3));
        var_qe_qs_dn4 = ((var_qe0_dn4 * var_tmpexp) + (var_qe0 * var_tmpexp_dn4));
        var_qe_qs_dn5 = ((var_qe0_dn5 * var_tmpexp) + (var_qe0 * var_tmpexp_dn5));
        var_qe_qs_dn6 = ((var_qe0_dn6 * var_tmpexp) + (var_qe0 * var_tmpexp_dn6));
        var_qe_qs_dn7 = ((var_qe0_dn7 * var_tmpexp) + (var_qe0 * var_tmpexp_dn7));
        var_qe_qs_dn8 = ((var_qe0_dn8 * var_tmpexp) + (var_qe0 * var_tmpexp_dn8));
        var_qe_qs_dn9 = ((var_qe0_dn9 * var_tmpexp) + (var_qe0 * var_tmpexp_dn9));
        var_qe_qs_dn10 = ((var_qe0_dn10 * var_tmpexp) + (var_qe0 * var_tmpexp_dn10));
        var_qe_qs_rv = 0.0;

        let assign6020_e6138: f64 = (4.0 * var_tepi_t);
        let assign6020_e6140: f64 = (assign6020_e6138 * var_vt);
        let assign6020_e6142: f64 = (assign6020_e6140 / var_rcv_t);
        var_qepi0 = assign6020_e6142;
        var_qepi0_dn3 = ((((((4.0 * var_tepi_t_dn3) * var_vt) + (assign6020_e6138 * var_vt_dn3)) * var_rcv_t) - (assign6020_e6140 * var_rcv_t_dn3)) / (var_rcv_t * var_rcv_t));
        var_qepi0_rv = 0.0;

        let assign6030_e6145: f64 = (0.5 * var_qepi0);
        let assign6030_e6147: f64 = (assign6030_e6145 * var_xi_w);
        let assign6030_e6150: f64 = (var_p0star + var_pw);
        let assign6030_e6152: f64 = (assign6030_e6150 + 2.0);
        let assign6030_e6153: f64 = (assign6030_e6147 * assign6030_e6152);
        var_qepi = assign6030_e6153;
        var_qepi_dn0 = (((assign6030_e6145 * var_xi_w_dn0) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn0 + var_pw_dn0)));
        var_qepi_dn1 = (((assign6030_e6145 * var_xi_w_dn1) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn1 + var_pw_dn1)));
        var_qepi_dn3 = (((((0.5 * var_qepi0_dn3) * var_xi_w) + (assign6030_e6145 * var_xi_w_dn3)) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn3 + var_pw_dn3)));
        var_qepi_dn4 = (((assign6030_e6145 * var_xi_w_dn4) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn4 + var_pw_dn4)));
        var_qepi_dn5 = (((assign6030_e6145 * var_xi_w_dn5) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn5 + var_pw_dn5)));
        var_qepi_dn6 = (((assign6030_e6145 * var_xi_w_dn6) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn6 + var_pw_dn6)));
        var_qepi_dn7 = (((assign6030_e6145 * var_xi_w_dn7) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn7 + var_pw_dn7)));
        var_qepi_dn8 = (((assign6030_e6145 * var_xi_w_dn8) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn8 + var_pw_dn8)));
        var_qepi_dn9 = (((assign6030_e6145 * var_xi_w_dn9) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn9 + var_pw_dn9)));
        var_qepi_dn10 = (((assign6030_e6145 * var_xi_w_dn10) * assign6030_e6152) + (assign6030_e6147 * (var_p0star_dn10 + var_pw_dn10)));
        var_qepi_rv = 0.0;

        let assign6040_e6156: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        var_guard110 = assign6040_e6156;
        var_guard110_rv = 0.0;

        let (assign6050_e6174, assign6050_e6174_d_n0, assign6050_e6174_d_n1, assign6050_e6174_d_n3, assign6050_e6174_d_n4, assign6050_e6174_d_n5, assign6050_e6174_d_n6, assign6050_e6174_d_n7, assign6050_e6174_d_n8, assign6050_e6174_d_n9, assign6050_e6174_d_n10,) = {
    if (var_guard110 != 0.0) {
        let assign6050_e6160: f64 = (var_taur_t * 0.5);
        let assign6050_e6163: f64 = (var_qb0 * var_nbex);
        let assign6050_e6166: f64 = (var_qepi0 * var_pwex);
        let assign6050_e6167: f64 = (assign6050_e6163 + assign6050_e6166);
        let assign6050_e6168: f64 = (assign6050_e6160 * assign6050_e6167);
        let assign6050_e6171: f64 = (var_taub_t + var_tepi_t);
        let assign6050_e6172: f64 = (assign6050_e6168 / assign6050_e6171);
        (assign6050_e6172, ((assign6050_e6160 * ((var_qb0 * var_nbex_dn0) + (var_qepi0 * var_pwex_dn0))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn1) + (var_qepi0 * var_pwex_dn1))) / assign6050_e6171), ((((((var_taur_t_dn3 * 0.5) * assign6050_e6167) + (assign6050_e6160 * (((var_qb0_dn3 * var_nbex) + (var_qb0 * var_nbex_dn3)) + ((var_qepi0_dn3 * var_pwex) + (var_qepi0 * var_pwex_dn3))))) * assign6050_e6171) - (assign6050_e6168 * (var_taub_t_dn3 + var_tepi_t_dn3))) / (assign6050_e6171 * assign6050_e6171)), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn4) + (var_qepi0 * var_pwex_dn4))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn5) + (var_qepi0 * var_pwex_dn5))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn6) + (var_qepi0 * var_pwex_dn6))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn7) + (var_qepi0 * var_pwex_dn7))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn8) + (var_qepi0 * var_pwex_dn8))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn9) + (var_qepi0 * var_pwex_dn9))) / assign6050_e6171), ((assign6050_e6160 * ((var_qb0 * var_nbex_dn10) + (var_qepi0 * var_pwex_dn10))) / assign6050_e6171),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10,)
    }
};
        var_qex = assign6050_e6174;
        var_qex_dn0 = assign6050_e6174_d_n0;
        var_qex_dn1 = assign6050_e6174_d_n1;
        var_qex_dn3 = assign6050_e6174_d_n3;
        var_qex_dn4 = assign6050_e6174_d_n4;
        var_qex_dn5 = assign6050_e6174_d_n5;
        var_qex_dn6 = assign6050_e6174_d_n6;
        var_qex_dn7 = assign6050_e6174_d_n7;
        var_qex_dn8 = assign6050_e6174_d_n8;
        var_qex_dn9 = assign6050_e6174_d_n9;
        var_qex_dn10 = assign6050_e6174_d_n10;
        var_qex_rv = 0.0;

        let assign6060_e6177: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6060_e6179: f64 = (assign6060_e6177 / p.p90);
        let assign6060_e6181: f64 = (assign6060_e6179 * var_vtinv);
        let assign6060_e6183: f64 = if assign6060_e6181 < p.p138 { 1.0 } else { 0.0 };
        var_guard111 = assign6060_e6183;
        var_guard111_rv = 0.0;

        *var_a_vdcctc_slot = var_a_vdcctc;
        *var_a_vdcctc_dn0_slot = var_a_vdcctc_dn0;
        *var_a_vdcctc_dn1_slot = var_a_vdcctc_dn1;
        *var_a_vdcctc_dn10_slot = var_a_vdcctc_dn10;
        *var_a_vdcctc_dn3_slot = var_a_vdcctc_dn3;
        *var_a_vdcctc_dn4_slot = var_a_vdcctc_dn4;
        *var_a_vdcctc_dn5_slot = var_a_vdcctc_dn5;
        *var_a_vdcctc_dn6_slot = var_a_vdcctc_dn6;
        *var_a_vdcctc_dn7_slot = var_a_vdcctc_dn7;
        *var_a_vdcctc_dn8_slot = var_a_vdcctc_dn8;
        *var_a_vdcctc_dn9_slot = var_a_vdcctc_dn9;
        *var_a_vdcctc_rv_slot = var_a_vdcctc_rv;
        *var_dxa_slot = var_dxa;
        *var_dxa_dn0_slot = var_dxa_dn0;
        *var_dxa_dn1_slot = var_dxa_dn1;
        *var_dxa_dn10_slot = var_dxa_dn10;
        *var_dxa_dn3_slot = var_dxa_dn3;
        *var_dxa_dn4_slot = var_dxa_dn4;
        *var_dxa_dn5_slot = var_dxa_dn5;
        *var_dxa_dn6_slot = var_dxa_dn6;
        *var_dxa_dn7_slot = var_dxa_dn7;
        *var_dxa_dn8_slot = var_dxa_dn8;
        *var_dxa_dn9_slot = var_dxa_dn9;
        *var_dxa_rv_slot = var_dxa_rv;
        *var_expl_slot = var_expl;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_dn3_slot = var_qb0_dn3;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_qbc_qs_slot = var_qbc_qs;
        *var_qbc_qs_dn0_slot = var_qbc_qs_dn0;
        *var_qbc_qs_dn1_slot = var_qbc_qs_dn1;
        *var_qbc_qs_dn10_slot = var_qbc_qs_dn10;
        *var_qbc_qs_dn3_slot = var_qbc_qs_dn3;
        *var_qbc_qs_dn4_slot = var_qbc_qs_dn4;
        *var_qbc_qs_dn5_slot = var_qbc_qs_dn5;
        *var_qbc_qs_dn6_slot = var_qbc_qs_dn6;
        *var_qbc_qs_dn7_slot = var_qbc_qs_dn7;
        *var_qbc_qs_dn8_slot = var_qbc_qs_dn8;
        *var_qbc_qs_dn9_slot = var_qbc_qs_dn9;
        *var_qbc_qs_rv_slot = var_qbc_qs_rv;
        *var_qbe_qs_slot = var_qbe_qs;
        *var_qbe_qs_dn0_slot = var_qbe_qs_dn0;
        *var_qbe_qs_dn1_slot = var_qbe_qs_dn1;
        *var_qbe_qs_dn10_slot = var_qbe_qs_dn10;
        *var_qbe_qs_dn3_slot = var_qbe_qs_dn3;
        *var_qbe_qs_dn4_slot = var_qbe_qs_dn4;
        *var_qbe_qs_dn5_slot = var_qbe_qs_dn5;
        *var_qbe_qs_dn6_slot = var_qbe_qs_dn6;
        *var_qbe_qs_dn7_slot = var_qbe_qs_dn7;
        *var_qbe_qs_dn8_slot = var_qbe_qs_dn8;
        *var_qbe_qs_dn9_slot = var_qbe_qs_dn9;
        *var_qbe_qs_rv_slot = var_qbe_qs_rv;
        *var_qe0_slot = var_qe0;
        *var_qe0_dn0_slot = var_qe0_dn0;
        *var_qe0_dn1_slot = var_qe0_dn1;
        *var_qe0_dn10_slot = var_qe0_dn10;
        *var_qe0_dn3_slot = var_qe0_dn3;
        *var_qe0_dn4_slot = var_qe0_dn4;
        *var_qe0_dn5_slot = var_qe0_dn5;
        *var_qe0_dn6_slot = var_qe0_dn6;
        *var_qe0_dn7_slot = var_qe0_dn7;
        *var_qe0_dn8_slot = var_qe0_dn8;
        *var_qe0_dn9_slot = var_qe0_dn9;
        *var_qe0_rv_slot = var_qe0_rv;
        *var_qe_qs_slot = var_qe_qs;
        *var_qe_qs_dn0_slot = var_qe_qs_dn0;
        *var_qe_qs_dn1_slot = var_qe_qs_dn1;
        *var_qe_qs_dn10_slot = var_qe_qs_dn10;
        *var_qe_qs_dn3_slot = var_qe_qs_dn3;
        *var_qe_qs_dn4_slot = var_qe_qs_dn4;
        *var_qe_qs_dn5_slot = var_qe_qs_dn5;
        *var_qe_qs_dn6_slot = var_qe_qs_dn6;
        *var_qe_qs_dn7_slot = var_qe_qs_dn7;
        *var_qe_qs_dn8_slot = var_qe_qs_dn8;
        *var_qe_qs_dn9_slot = var_qe_qs_dn9;
        *var_qe_qs_rv_slot = var_qe_qs_rv;
        *var_qepi_slot = var_qepi;
        *var_qepi0_slot = var_qepi0;
        *var_qepi0_dn3_slot = var_qepi0_dn3;
        *var_qepi0_rv_slot = var_qepi0_rv;
        *var_qepi_dn0_slot = var_qepi_dn0;
        *var_qepi_dn1_slot = var_qepi_dn1;
        *var_qepi_dn10_slot = var_qepi_dn10;
        *var_qepi_dn3_slot = var_qepi_dn3;
        *var_qepi_dn4_slot = var_qepi_dn4;
        *var_qepi_dn5_slot = var_qepi_dn5;
        *var_qepi_dn6_slot = var_qepi_dn6;
        *var_qepi_dn7_slot = var_qepi_dn7;
        *var_qepi_dn8_slot = var_qepi_dn8;
        *var_qepi_dn9_slot = var_qepi_dn9;
        *var_qepi_rv_slot = var_qepi_rv;
        *var_qex_slot = var_qex;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_qex_rv_slot = var_qex_rv;
        *var_qtc_slot = var_qtc;
        *var_qtc_dn0_slot = var_qtc_dn0;
        *var_qtc_dn1_slot = var_qtc_dn1;
        *var_qtc_dn10_slot = var_qtc_dn10;
        *var_qtc_dn3_slot = var_qtc_dn3;
        *var_qtc_dn4_slot = var_qtc_dn4;
        *var_qtc_dn5_slot = var_qtc_dn5;
        *var_qtc_dn6_slot = var_qtc_dn6;
        *var_qtc_dn7_slot = var_qtc_dn7;
        *var_qtc_dn8_slot = var_qtc_dn8;
        *var_qtc_dn9_slot = var_qtc_dn9;
        *var_qtc_rv_slot = var_qtc_rv;
        *var_qte_s_slot = var_qte_s;
        *var_qte_s_dn0_slot = var_qte_s_dn0;
        *var_qte_s_dn1_slot = var_qte_s_dn1;
        *var_qte_s_dn10_slot = var_qte_s_dn10;
        *var_qte_s_dn3_slot = var_qte_s_dn3;
        *var_qte_s_dn4_slot = var_qte_s_dn4;
        *var_qte_s_dn5_slot = var_qte_s_dn5;
        *var_qte_s_dn6_slot = var_qte_s_dn6;
        *var_qte_s_dn7_slot = var_qte_s_dn7;
        *var_qte_s_dn8_slot = var_qte_s_dn8;
        *var_qte_s_dn9_slot = var_qte_s_dn9;
        *var_qte_s_rv_slot = var_qte_s_rv;
        *var_qtex_slot = var_qtex;
        *var_qtex_dn0_slot = var_qtex_dn0;
        *var_qtex_dn1_slot = var_qtex_dn1;
        *var_qtex_dn10_slot = var_qtex_dn10;
        *var_qtex_dn3_slot = var_qtex_dn3;
        *var_qtex_dn4_slot = var_qtex_dn4;
        *var_qtex_dn5_slot = var_qtex_dn5;
        *var_qtex_dn6_slot = var_qtex_dn6;
        *var_qtex_dn7_slot = var_qtex_dn7;
        *var_qtex_dn8_slot = var_qtex_dn8;
        *var_qtex_dn9_slot = var_qtex_dn9;
        *var_qtex_rv_slot = var_qtex_rv;
        *var_tmpexp_slot = var_tmpexp;
        *var_tmpexp_dn0_slot = var_tmpexp_dn0;
        *var_tmpexp_dn1_slot = var_tmpexp_dn1;
        *var_tmpexp_dn10_slot = var_tmpexp_dn10;
        *var_tmpexp_dn3_slot = var_tmpexp_dn3;
        *var_tmpexp_dn4_slot = var_tmpexp_dn4;
        *var_tmpexp_dn5_slot = var_tmpexp_dn5;
        *var_tmpexp_dn6_slot = var_tmpexp_dn6;
        *var_tmpexp_dn7_slot = var_tmpexp_dn7;
        *var_tmpexp_dn8_slot = var_tmpexp_dn8;
        *var_tmpexp_dn9_slot = var_tmpexp_dn9;
        *var_tmpexp_rv_slot = var_tmpexp_rv;
        *var_vjcex_slot = var_vjcex;
        *var_vjcex_dn0_slot = var_vjcex_dn0;
        *var_vjcex_dn1_slot = var_vjcex_dn1;
        *var_vjcex_dn10_slot = var_vjcex_dn10;
        *var_vjcex_dn3_slot = var_vjcex_dn3;
        *var_vjcex_dn4_slot = var_vjcex_dn4;
        *var_vjcex_dn5_slot = var_vjcex_dn5;
        *var_vjcex_dn6_slot = var_vjcex_dn6;
        *var_vjcex_dn7_slot = var_vjcex_dn7;
        *var_vjcex_dn8_slot = var_vjcex_dn8;
        *var_vjcex_dn9_slot = var_vjcex_dn9;
        *var_vjcex_rv_slot = var_vjcex_rv;
        *var_vje_s_slot = var_vje_s;
        *var_vje_s_dn0_slot = var_vje_s_dn0;
        *var_vje_s_dn1_slot = var_vje_s_dn1;
        *var_vje_s_dn10_slot = var_vje_s_dn10;
        *var_vje_s_dn3_slot = var_vje_s_dn3;
        *var_vje_s_dn4_slot = var_vje_s_dn4;
        *var_vje_s_dn5_slot = var_vje_s_dn5;
        *var_vje_s_dn6_slot = var_vje_s_dn6;
        *var_vje_s_dn7_slot = var_vje_s_dn7;
        *var_vje_s_dn8_slot = var_vje_s_dn8;
        *var_vje_s_dn9_slot = var_vje_s_dn9;
        *var_vje_s_rv_slot = var_vje_s_rv;
        *var_vtexv_slot = var_vtexv;
        *var_vtexv_dn0_slot = var_vtexv_dn0;
        *var_vtexv_dn1_slot = var_vtexv_dn1;
        *var_vtexv_dn10_slot = var_vtexv_dn10;
        *var_vtexv_dn3_slot = var_vtexv_dn3;
        *var_vtexv_dn4_slot = var_vtexv_dn4;
        *var_vtexv_dn5_slot = var_vtexv_dn5;
        *var_vtexv_dn6_slot = var_vtexv_dn6;
        *var_vtexv_dn7_slot = var_vtexv_dn7;
        *var_vtexv_dn8_slot = var_vtexv_dn8;
        *var_vtexv_dn9_slot = var_vtexv_dn9;
        *var_vtexv_rv_slot = var_vtexv_rv;
        *var_xqtex_slot = var_xqtex;
        *var_xqtex_dn0_slot = var_xqtex_dn0;
        *var_xqtex_dn1_slot = var_xqtex_dn1;
        *var_xqtex_dn10_slot = var_xqtex_dn10;
        *var_xqtex_dn3_slot = var_xqtex_dn3;
        *var_xqtex_dn4_slot = var_xqtex_dn4;
        *var_xqtex_dn5_slot = var_xqtex_dn5;
        *var_xqtex_dn6_slot = var_xqtex_dn6;
        *var_xqtex_dn7_slot = var_xqtex_dn7;
        *var_xqtex_dn8_slot = var_xqtex_dn8;
        *var_xqtex_dn9_slot = var_xqtex_dn9;
        *var_xqtex_rv_slot = var_xqtex_rv;
        *var_xvjcex_slot = var_xvjcex;
        *var_xvjcex_dn0_slot = var_xvjcex_dn0;
        *var_xvjcex_dn1_slot = var_xvjcex_dn1;
        *var_xvjcex_dn10_slot = var_xvjcex_dn10;
        *var_xvjcex_dn3_slot = var_xvjcex_dn3;
        *var_xvjcex_dn4_slot = var_xvjcex_dn4;
        *var_xvjcex_dn5_slot = var_xvjcex_dn5;
        *var_xvjcex_dn6_slot = var_xvjcex_dn6;
        *var_xvjcex_dn7_slot = var_xvjcex_dn7;
        *var_xvjcex_dn8_slot = var_xvjcex_dn8;
        *var_xvjcex_dn9_slot = var_xvjcex_dn9;
        *var_xvjcex_rv_slot = var_xvjcex_rv;
        *var_xvtexv_slot = var_xvtexv;
        *var_xvtexv_dn0_slot = var_xvtexv_dn0;
        *var_xvtexv_dn1_slot = var_xvtexv_dn1;
        *var_xvtexv_dn10_slot = var_xvtexv_dn10;
        *var_xvtexv_dn3_slot = var_xvtexv_dn3;
        *var_xvtexv_dn4_slot = var_xvtexv_dn4;
        *var_xvtexv_dn5_slot = var_xvtexv_dn5;
        *var_xvtexv_dn6_slot = var_xvtexv_dn6;
        *var_xvtexv_dn7_slot = var_xvtexv_dn7;
        *var_xvtexv_dn8_slot = var_xvtexv_dn8;
        *var_xvtexv_dn9_slot = var_xvtexv_dn9;
        *var_xvtexv_rv_slot = var_xvtexv_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_cje_t: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_evb1c4: f64,
        var_evb1c4_dn10: f64,
        var_evb1c4_dn3: f64,
        var_evb1c4_dn5: f64,
        var_evb1c4_dn6: f64,
        var_evb1c4_dn7: f64,
        var_evb1c4_dn8: f64,
        var_evbc3: f64,
        var_evbc3_dn0: f64,
        var_evbc3_dn1: f64,
        var_evbc3_dn10: f64,
        var_evbc3_dn3: f64,
        var_evbc3_dn5: f64,
        var_evbc3_dn6: f64,
        var_evbc3_dn7: f64,
        var_evbc3_dn8: f64,
        var_evbc3_dn9: f64,
        var_evbc3vdc: f64,
        var_evbc3vdc_dn0: f64,
        var_evbc3vdc_dn1: f64,
        var_evbc3vdc_dn10: f64,
        var_evbc3vdc_dn3: f64,
        var_evbc3vdc_dn4: f64,
        var_evbc3vdc_dn5: f64,
        var_evbc3vdc_dn6: f64,
        var_evbc3vdc_dn7: f64,
        var_evbc3vdc_dn8: f64,
        var_evbc3vdc_dn9: f64,
        var_fex: f64,
        var_fex_dn0: f64,
        var_fex_dn1: f64,
        var_fex_dn10: f64,
        var_fex_dn3: f64,
        var_fex_dn4: f64,
        var_fex_dn5: f64,
        var_fex_dn6: f64,
        var_fex_dn7: f64,
        var_fex_dn8: f64,
        var_fex_dn9: f64,
        var_guard110: f64,
        var_guard111: f64,
        var_ibx_t: f64,
        var_ibx_t_dn3: f64,
        var_if0: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_qb0: f64,
        var_qb0_dn3: f64,
        var_qepi0: f64,
        var_qepi0_dn3: f64,
        var_taub_t: f64,
        var_taub_t_dn3: f64,
        var_tauex_t: f64,
        var_tauex_t_dn3: f64,
        var_taur_t: f64,
        var_taur_t_dn3: f64,
        var_tepi_t: f64,
        var_tepi_t_dn3: f64,
        var_vb1c4: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb2e1: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn6: f64,
        var_vbc3: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn5: f64,
        var_vbc3_dn6: f64,
        var_vbc3_dn7: f64,
        var_vbc3_dn8: f64,
        var_vbc3_dn9: f64,
        var_vdcex_t: f64,
        var_vdcex_t_dn0: f64,
        var_vdcex_t_dn1: f64,
        var_vdcex_t_dn10: f64,
        var_vdcex_t_dn3: f64,
        var_vdcex_t_dn4: f64,
        var_vdcex_t_dn5: f64,
        var_vdcex_t_dn6: f64,
        var_vdcex_t_dn7: f64,
        var_vdcex_t_dn8: f64,
        var_vdcex_t_dn9: f64,
        var_vfe: f64,
        var_vfe_dn0: f64,
        var_vfe_dn1: f64,
        var_vfe_dn10: f64,
        var_vfe_dn3: f64,
        var_vfe_dn4: f64,
        var_vfe_dn5: f64,
        var_vfe_dn6: f64,
        var_vfe_dn7: f64,
        var_vfe_dn8: f64,
        var_vfe_dn9: f64,
        var_vje: f64,
        var_vje_dn0: f64,
        var_vje_dn1: f64,
        var_vje_dn10: f64,
        var_vje_dn3: f64,
        var_vje_dn4: f64,
        var_vje_dn5: f64,
        var_vje_dn6: f64,
        var_vje_dn7: f64,
        var_vje_dn8: f64,
        var_vje_dn9: f64,
        var_vtinv: f64,
        var_vtinv_dn3: f64,
        var_xext1: f64,
        var_dqtevb2e1_slot: &mut f64,
        var_dqtevb2e1_dn0_slot: &mut f64,
        var_dqtevb2e1_dn1_slot: &mut f64,
        var_dqtevb2e1_dn10_slot: &mut f64,
        var_dqtevb2e1_dn3_slot: &mut f64,
        var_dqtevb2e1_dn4_slot: &mut f64,
        var_dqtevb2e1_dn5_slot: &mut f64,
        var_dqtevb2e1_dn6_slot: &mut f64,
        var_dqtevb2e1_dn7_slot: &mut f64,
        var_dqtevb2e1_dn8_slot: &mut f64,
        var_dqtevb2e1_dn9_slot: &mut f64,
        var_dqtevb2e1_rv_slot: &mut f64,
        var_dvjevb2e1_slot: &mut f64,
        var_dvjevb2e1_dn0_slot: &mut f64,
        var_dvjevb2e1_dn1_slot: &mut f64,
        var_dvjevb2e1_dn10_slot: &mut f64,
        var_dvjevb2e1_dn3_slot: &mut f64,
        var_dvjevb2e1_dn4_slot: &mut f64,
        var_dvjevb2e1_dn5_slot: &mut f64,
        var_dvjevb2e1_dn6_slot: &mut f64,
        var_dvjevb2e1_dn7_slot: &mut f64,
        var_dvjevb2e1_dn8_slot: &mut f64,
        var_dvjevb2e1_dn9_slot: &mut f64,
        var_dvjevb2e1_rv_slot: &mut f64,
        var_dvtevb2e1_slot: &mut f64,
        var_dvtevb2e1_dn0_slot: &mut f64,
        var_dvtevb2e1_dn1_slot: &mut f64,
        var_dvtevb2e1_dn10_slot: &mut f64,
        var_dvtevb2e1_dn3_slot: &mut f64,
        var_dvtevb2e1_dn4_slot: &mut f64,
        var_dvtevb2e1_dn5_slot: &mut f64,
        var_dvtevb2e1_dn6_slot: &mut f64,
        var_dvtevb2e1_dn7_slot: &mut f64,
        var_dvtevb2e1_dn8_slot: &mut f64,
        var_dvtevb2e1_dn9_slot: &mut f64,
        var_dvtevb2e1_rv_slot: &mut f64,
        var_dvtevje_slot: &mut f64,
        var_dvtevje_dn0_slot: &mut f64,
        var_dvtevje_dn1_slot: &mut f64,
        var_dvtevje_dn10_slot: &mut f64,
        var_dvtevje_dn3_slot: &mut f64,
        var_dvtevje_dn4_slot: &mut f64,
        var_dvtevje_dn5_slot: &mut f64,
        var_dvtevje_dn6_slot: &mut f64,
        var_dvtevje_dn7_slot: &mut f64,
        var_dvtevje_dn8_slot: &mut f64,
        var_dvtevje_dn9_slot: &mut f64,
        var_dvtevje_rv_slot: &mut f64,
        var_evb1c4vdcex_slot: &mut f64,
        var_evb1c4vdcex_dn0_slot: &mut f64,
        var_evb1c4vdcex_dn1_slot: &mut f64,
        var_evb1c4vdcex_dn10_slot: &mut f64,
        var_evb1c4vdcex_dn3_slot: &mut f64,
        var_evb1c4vdcex_dn4_slot: &mut f64,
        var_evb1c4vdcex_dn5_slot: &mut f64,
        var_evb1c4vdcex_dn6_slot: &mut f64,
        var_evb1c4vdcex_dn7_slot: &mut f64,
        var_evb1c4vdcex_dn8_slot: &mut f64,
        var_evb1c4vdcex_dn9_slot: &mut f64,
        var_evb1c4vdcex_rv_slot: &mut f64,
        var_evbc3vdcex_slot: &mut f64,
        var_evbc3vdcex_dn0_slot: &mut f64,
        var_evbc3vdcex_dn1_slot: &mut f64,
        var_evbc3vdcex_dn10_slot: &mut f64,
        var_evbc3vdcex_dn3_slot: &mut f64,
        var_evbc3vdcex_dn4_slot: &mut f64,
        var_evbc3vdcex_dn5_slot: &mut f64,
        var_evbc3vdcex_dn6_slot: &mut f64,
        var_evbc3vdcex_dn7_slot: &mut f64,
        var_evbc3vdcex_dn8_slot: &mut f64,
        var_evbc3vdcex_dn9_slot: &mut f64,
        var_evbc3vdcex_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
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
        var_qex_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_qex_rv_slot: &mut f64,
        var_vb2e1vfe_slot: &mut f64,
        var_vb2e1vfe_dn0_slot: &mut f64,
        var_vb2e1vfe_dn1_slot: &mut f64,
        var_vb2e1vfe_dn10_slot: &mut f64,
        var_vb2e1vfe_dn3_slot: &mut f64,
        var_vb2e1vfe_dn4_slot: &mut f64,
        var_vb2e1vfe_dn5_slot: &mut f64,
        var_vb2e1vfe_dn6_slot: &mut f64,
        var_vb2e1vfe_dn7_slot: &mut f64,
        var_vb2e1vfe_dn8_slot: &mut f64,
        var_vb2e1vfe_dn9_slot: &mut f64,
        var_vb2e1vfe_rv_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn0_slot: &mut f64,
        var_xg1_dn1_slot: &mut f64,
        var_xg1_dn10_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
        var_xg1_dn9_slot: &mut f64,
        var_xg1_rv_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_dn0_slot: &mut f64,
        var_xg2_dn1_slot: &mut f64,
        var_xg2_dn10_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
        var_xg2_dn9_slot: &mut f64,
        var_xg2_rv_slot: &mut f64,
        var_xnbex_slot: &mut f64,
        var_xnbex_dn0_slot: &mut f64,
        var_xnbex_dn1_slot: &mut f64,
        var_xnbex_dn10_slot: &mut f64,
        var_xnbex_dn3_slot: &mut f64,
        var_xnbex_dn4_slot: &mut f64,
        var_xnbex_dn5_slot: &mut f64,
        var_xnbex_dn6_slot: &mut f64,
        var_xnbex_dn7_slot: &mut f64,
        var_xnbex_dn8_slot: &mut f64,
        var_xnbex_dn9_slot: &mut f64,
        var_xnbex_rv_slot: &mut f64,
        var_xpwex_slot: &mut f64,
        var_xpwex_dn0_slot: &mut f64,
        var_xpwex_dn1_slot: &mut f64,
        var_xpwex_dn10_slot: &mut f64,
        var_xpwex_dn3_slot: &mut f64,
        var_xpwex_dn4_slot: &mut f64,
        var_xpwex_dn5_slot: &mut f64,
        var_xpwex_dn6_slot: &mut f64,
        var_xpwex_dn7_slot: &mut f64,
        var_xpwex_dn8_slot: &mut f64,
        var_xpwex_dn9_slot: &mut f64,
        var_xpwex_rv_slot: &mut f64,
        var_xqex_slot: &mut f64,
        var_xqex_dn0_slot: &mut f64,
        var_xqex_dn1_slot: &mut f64,
        var_xqex_dn10_slot: &mut f64,
        var_xqex_dn3_slot: &mut f64,
        var_xqex_dn4_slot: &mut f64,
        var_xqex_dn5_slot: &mut f64,
        var_xqex_dn6_slot: &mut f64,
        var_xqex_dn7_slot: &mut f64,
        var_xqex_dn8_slot: &mut f64,
        var_xqex_dn9_slot: &mut f64,
        var_xqex_rv_slot: &mut f64,
        var_xqmex_slot: &mut f64,
        var_xqmex_dn0_slot: &mut f64,
        var_xqmex_dn1_slot: &mut f64,
        var_xqmex_dn10_slot: &mut f64,
        var_xqmex_dn3_slot: &mut f64,
        var_xqmex_dn4_slot: &mut f64,
        var_xqmex_dn5_slot: &mut f64,
        var_xqmex_dn6_slot: &mut f64,
        var_xqmex_dn7_slot: &mut f64,
        var_xqmex_dn8_slot: &mut f64,
        var_xqmex_dn9_slot: &mut f64,
        var_xqmex_rv_slot: &mut f64,
    ) {
        let mut var_dqtevb2e1: f64 = *var_dqtevb2e1_slot;
        let mut var_dqtevb2e1_dn0: f64 = *var_dqtevb2e1_dn0_slot;
        let mut var_dqtevb2e1_dn1: f64 = *var_dqtevb2e1_dn1_slot;
        let mut var_dqtevb2e1_dn10: f64 = *var_dqtevb2e1_dn10_slot;
        let mut var_dqtevb2e1_dn3: f64 = *var_dqtevb2e1_dn3_slot;
        let mut var_dqtevb2e1_dn4: f64 = *var_dqtevb2e1_dn4_slot;
        let mut var_dqtevb2e1_dn5: f64 = *var_dqtevb2e1_dn5_slot;
        let mut var_dqtevb2e1_dn6: f64 = *var_dqtevb2e1_dn6_slot;
        let mut var_dqtevb2e1_dn7: f64 = *var_dqtevb2e1_dn7_slot;
        let mut var_dqtevb2e1_dn8: f64 = *var_dqtevb2e1_dn8_slot;
        let mut var_dqtevb2e1_dn9: f64 = *var_dqtevb2e1_dn9_slot;
        let mut var_dqtevb2e1_rv: f64 = *var_dqtevb2e1_rv_slot;
        let mut var_dvjevb2e1: f64 = *var_dvjevb2e1_slot;
        let mut var_dvjevb2e1_dn0: f64 = *var_dvjevb2e1_dn0_slot;
        let mut var_dvjevb2e1_dn1: f64 = *var_dvjevb2e1_dn1_slot;
        let mut var_dvjevb2e1_dn10: f64 = *var_dvjevb2e1_dn10_slot;
        let mut var_dvjevb2e1_dn3: f64 = *var_dvjevb2e1_dn3_slot;
        let mut var_dvjevb2e1_dn4: f64 = *var_dvjevb2e1_dn4_slot;
        let mut var_dvjevb2e1_dn5: f64 = *var_dvjevb2e1_dn5_slot;
        let mut var_dvjevb2e1_dn6: f64 = *var_dvjevb2e1_dn6_slot;
        let mut var_dvjevb2e1_dn7: f64 = *var_dvjevb2e1_dn7_slot;
        let mut var_dvjevb2e1_dn8: f64 = *var_dvjevb2e1_dn8_slot;
        let mut var_dvjevb2e1_dn9: f64 = *var_dvjevb2e1_dn9_slot;
        let mut var_dvjevb2e1_rv: f64 = *var_dvjevb2e1_rv_slot;
        let mut var_dvtevb2e1: f64 = *var_dvtevb2e1_slot;
        let mut var_dvtevb2e1_dn0: f64 = *var_dvtevb2e1_dn0_slot;
        let mut var_dvtevb2e1_dn1: f64 = *var_dvtevb2e1_dn1_slot;
        let mut var_dvtevb2e1_dn10: f64 = *var_dvtevb2e1_dn10_slot;
        let mut var_dvtevb2e1_dn3: f64 = *var_dvtevb2e1_dn3_slot;
        let mut var_dvtevb2e1_dn4: f64 = *var_dvtevb2e1_dn4_slot;
        let mut var_dvtevb2e1_dn5: f64 = *var_dvtevb2e1_dn5_slot;
        let mut var_dvtevb2e1_dn6: f64 = *var_dvtevb2e1_dn6_slot;
        let mut var_dvtevb2e1_dn7: f64 = *var_dvtevb2e1_dn7_slot;
        let mut var_dvtevb2e1_dn8: f64 = *var_dvtevb2e1_dn8_slot;
        let mut var_dvtevb2e1_dn9: f64 = *var_dvtevb2e1_dn9_slot;
        let mut var_dvtevb2e1_rv: f64 = *var_dvtevb2e1_rv_slot;
        let mut var_dvtevje: f64 = *var_dvtevje_slot;
        let mut var_dvtevje_dn0: f64 = *var_dvtevje_dn0_slot;
        let mut var_dvtevje_dn1: f64 = *var_dvtevje_dn1_slot;
        let mut var_dvtevje_dn10: f64 = *var_dvtevje_dn10_slot;
        let mut var_dvtevje_dn3: f64 = *var_dvtevje_dn3_slot;
        let mut var_dvtevje_dn4: f64 = *var_dvtevje_dn4_slot;
        let mut var_dvtevje_dn5: f64 = *var_dvtevje_dn5_slot;
        let mut var_dvtevje_dn6: f64 = *var_dvtevje_dn6_slot;
        let mut var_dvtevje_dn7: f64 = *var_dvtevje_dn7_slot;
        let mut var_dvtevje_dn8: f64 = *var_dvtevje_dn8_slot;
        let mut var_dvtevje_dn9: f64 = *var_dvtevje_dn9_slot;
        let mut var_dvtevje_rv: f64 = *var_dvtevje_rv_slot;
        let mut var_evb1c4vdcex: f64 = *var_evb1c4vdcex_slot;
        let mut var_evb1c4vdcex_dn0: f64 = *var_evb1c4vdcex_dn0_slot;
        let mut var_evb1c4vdcex_dn1: f64 = *var_evb1c4vdcex_dn1_slot;
        let mut var_evb1c4vdcex_dn10: f64 = *var_evb1c4vdcex_dn10_slot;
        let mut var_evb1c4vdcex_dn3: f64 = *var_evb1c4vdcex_dn3_slot;
        let mut var_evb1c4vdcex_dn4: f64 = *var_evb1c4vdcex_dn4_slot;
        let mut var_evb1c4vdcex_dn5: f64 = *var_evb1c4vdcex_dn5_slot;
        let mut var_evb1c4vdcex_dn6: f64 = *var_evb1c4vdcex_dn6_slot;
        let mut var_evb1c4vdcex_dn7: f64 = *var_evb1c4vdcex_dn7_slot;
        let mut var_evb1c4vdcex_dn8: f64 = *var_evb1c4vdcex_dn8_slot;
        let mut var_evb1c4vdcex_dn9: f64 = *var_evb1c4vdcex_dn9_slot;
        let mut var_evb1c4vdcex_rv: f64 = *var_evb1c4vdcex_rv_slot;
        let mut var_evbc3vdcex: f64 = *var_evbc3vdcex_slot;
        let mut var_evbc3vdcex_dn0: f64 = *var_evbc3vdcex_dn0_slot;
        let mut var_evbc3vdcex_dn1: f64 = *var_evbc3vdcex_dn1_slot;
        let mut var_evbc3vdcex_dn10: f64 = *var_evbc3vdcex_dn10_slot;
        let mut var_evbc3vdcex_dn3: f64 = *var_evbc3vdcex_dn3_slot;
        let mut var_evbc3vdcex_dn4: f64 = *var_evbc3vdcex_dn4_slot;
        let mut var_evbc3vdcex_dn5: f64 = *var_evbc3vdcex_dn5_slot;
        let mut var_evbc3vdcex_dn6: f64 = *var_evbc3vdcex_dn6_slot;
        let mut var_evbc3vdcex_dn7: f64 = *var_evbc3vdcex_dn7_slot;
        let mut var_evbc3vdcex_dn8: f64 = *var_evbc3vdcex_dn8_slot;
        let mut var_evbc3vdcex_dn9: f64 = *var_evbc3vdcex_dn9_slot;
        let mut var_evbc3vdcex_rv: f64 = *var_evbc3vdcex_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
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
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_qex_rv: f64 = *var_qex_rv_slot;
        let mut var_vb2e1vfe: f64 = *var_vb2e1vfe_slot;
        let mut var_vb2e1vfe_dn0: f64 = *var_vb2e1vfe_dn0_slot;
        let mut var_vb2e1vfe_dn1: f64 = *var_vb2e1vfe_dn1_slot;
        let mut var_vb2e1vfe_dn10: f64 = *var_vb2e1vfe_dn10_slot;
        let mut var_vb2e1vfe_dn3: f64 = *var_vb2e1vfe_dn3_slot;
        let mut var_vb2e1vfe_dn4: f64 = *var_vb2e1vfe_dn4_slot;
        let mut var_vb2e1vfe_dn5: f64 = *var_vb2e1vfe_dn5_slot;
        let mut var_vb2e1vfe_dn6: f64 = *var_vb2e1vfe_dn6_slot;
        let mut var_vb2e1vfe_dn7: f64 = *var_vb2e1vfe_dn7_slot;
        let mut var_vb2e1vfe_dn8: f64 = *var_vb2e1vfe_dn8_slot;
        let mut var_vb2e1vfe_dn9: f64 = *var_vb2e1vfe_dn9_slot;
        let mut var_vb2e1vfe_rv: f64 = *var_vb2e1vfe_rv_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn0: f64 = *var_xg1_dn0_slot;
        let mut var_xg1_dn1: f64 = *var_xg1_dn1_slot;
        let mut var_xg1_dn10: f64 = *var_xg1_dn10_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;
        let mut var_xg1_dn9: f64 = *var_xg1_dn9_slot;
        let mut var_xg1_rv: f64 = *var_xg1_rv_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_dn0: f64 = *var_xg2_dn0_slot;
        let mut var_xg2_dn1: f64 = *var_xg2_dn1_slot;
        let mut var_xg2_dn10: f64 = *var_xg2_dn10_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;
        let mut var_xg2_dn9: f64 = *var_xg2_dn9_slot;
        let mut var_xg2_rv: f64 = *var_xg2_rv_slot;
        let mut var_xnbex: f64 = *var_xnbex_slot;
        let mut var_xnbex_dn0: f64 = *var_xnbex_dn0_slot;
        let mut var_xnbex_dn1: f64 = *var_xnbex_dn1_slot;
        let mut var_xnbex_dn10: f64 = *var_xnbex_dn10_slot;
        let mut var_xnbex_dn3: f64 = *var_xnbex_dn3_slot;
        let mut var_xnbex_dn4: f64 = *var_xnbex_dn4_slot;
        let mut var_xnbex_dn5: f64 = *var_xnbex_dn5_slot;
        let mut var_xnbex_dn6: f64 = *var_xnbex_dn6_slot;
        let mut var_xnbex_dn7: f64 = *var_xnbex_dn7_slot;
        let mut var_xnbex_dn8: f64 = *var_xnbex_dn8_slot;
        let mut var_xnbex_dn9: f64 = *var_xnbex_dn9_slot;
        let mut var_xnbex_rv: f64 = *var_xnbex_rv_slot;
        let mut var_xpwex: f64 = *var_xpwex_slot;
        let mut var_xpwex_dn0: f64 = *var_xpwex_dn0_slot;
        let mut var_xpwex_dn1: f64 = *var_xpwex_dn1_slot;
        let mut var_xpwex_dn10: f64 = *var_xpwex_dn10_slot;
        let mut var_xpwex_dn3: f64 = *var_xpwex_dn3_slot;
        let mut var_xpwex_dn4: f64 = *var_xpwex_dn4_slot;
        let mut var_xpwex_dn5: f64 = *var_xpwex_dn5_slot;
        let mut var_xpwex_dn6: f64 = *var_xpwex_dn6_slot;
        let mut var_xpwex_dn7: f64 = *var_xpwex_dn7_slot;
        let mut var_xpwex_dn8: f64 = *var_xpwex_dn8_slot;
        let mut var_xpwex_dn9: f64 = *var_xpwex_dn9_slot;
        let mut var_xpwex_rv: f64 = *var_xpwex_rv_slot;
        let mut var_xqex: f64 = *var_xqex_slot;
        let mut var_xqex_dn0: f64 = *var_xqex_dn0_slot;
        let mut var_xqex_dn1: f64 = *var_xqex_dn1_slot;
        let mut var_xqex_dn10: f64 = *var_xqex_dn10_slot;
        let mut var_xqex_dn3: f64 = *var_xqex_dn3_slot;
        let mut var_xqex_dn4: f64 = *var_xqex_dn4_slot;
        let mut var_xqex_dn5: f64 = *var_xqex_dn5_slot;
        let mut var_xqex_dn6: f64 = *var_xqex_dn6_slot;
        let mut var_xqex_dn7: f64 = *var_xqex_dn7_slot;
        let mut var_xqex_dn8: f64 = *var_xqex_dn8_slot;
        let mut var_xqex_dn9: f64 = *var_xqex_dn9_slot;
        let mut var_xqex_rv: f64 = *var_xqex_rv_slot;
        let mut var_xqmex: f64 = *var_xqmex_slot;
        let mut var_xqmex_dn0: f64 = *var_xqmex_dn0_slot;
        let mut var_xqmex_dn1: f64 = *var_xqmex_dn1_slot;
        let mut var_xqmex_dn10: f64 = *var_xqmex_dn10_slot;
        let mut var_xqmex_dn3: f64 = *var_xqmex_dn3_slot;
        let mut var_xqmex_dn4: f64 = *var_xqmex_dn4_slot;
        let mut var_xqmex_dn5: f64 = *var_xqmex_dn5_slot;
        let mut var_xqmex_dn6: f64 = *var_xqmex_dn6_slot;
        let mut var_xqmex_dn7: f64 = *var_xqmex_dn7_slot;
        let mut var_xqmex_dn8: f64 = *var_xqmex_dn8_slot;
        let mut var_xqmex_dn9: f64 = *var_xqmex_dn9_slot;
        let mut var_xqmex_rv: f64 = *var_xqmex_rv_slot;

        let (assign6070_e6197, assign6070_e6197_d_n0, assign6070_e6197_d_n1, assign6070_e6197_d_n3, assign6070_e6197_d_n4, assign6070_e6197_d_n5, assign6070_e6197_d_n6, assign6070_e6197_d_n7, assign6070_e6197_d_n8, assign6070_e6197_d_n9, assign6070_e6197_d_n10,) = {
    if ((var_guard110 == 0.0) && (var_guard111 != 0.0)) {
        let assign6070_e6190: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6070_e6192: f64 = (assign6070_e6190 / p.p90);
        let assign6070_e6194: f64 = (assign6070_e6192 * var_vtinv);
        let assign6070_e6195: f64 = (assign6070_e6194).exp();
        (assign6070_e6195, (assign6070_e6195 * (((-var_vdcex_t_dn0) / p.p90) * var_vtinv)), (assign6070_e6195 * (((-var_vdcex_t_dn1) / p.p90) * var_vtinv)), (assign6070_e6195 * ((((-var_vdcex_t_dn3) / p.p90) * var_vtinv) + (assign6070_e6192 * var_vtinv_dn3))), (assign6070_e6195 * (((-var_vdcex_t_dn4) / p.p90) * var_vtinv)), (assign6070_e6195 * (((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p90) * var_vtinv)), (assign6070_e6195 * (((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p90) * var_vtinv)), (assign6070_e6195 * (((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p90) * var_vtinv)), (assign6070_e6195 * (((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p90) * var_vtinv)), (assign6070_e6195 * (((-var_vdcex_t_dn9) / p.p90) * var_vtinv)), (assign6070_e6195 * (((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p90) * var_vtinv)),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10,)
    }
};
        var_evb1c4vdcex = assign6070_e6197;
        var_evb1c4vdcex_dn0 = assign6070_e6197_d_n0;
        var_evb1c4vdcex_dn1 = assign6070_e6197_d_n1;
        var_evb1c4vdcex_dn3 = assign6070_e6197_d_n3;
        var_evb1c4vdcex_dn4 = assign6070_e6197_d_n4;
        var_evb1c4vdcex_dn5 = assign6070_e6197_d_n5;
        var_evb1c4vdcex_dn6 = assign6070_e6197_d_n6;
        var_evb1c4vdcex_dn7 = assign6070_e6197_d_n7;
        var_evb1c4vdcex_dn8 = assign6070_e6197_d_n8;
        var_evb1c4vdcex_dn9 = assign6070_e6197_d_n9;
        var_evb1c4vdcex_dn10 = assign6070_e6197_d_n10;
        var_evb1c4vdcex_rv = 0.0;

        let (assign6080_e6206,) = {
    if ((var_guard110 == 0.0) && (var_guard111 == 0.0)) {
        let assign6080_e6204: f64 = (p.p138).exp();
        (assign6080_e6204,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6080_e6206;
        var_expl_rv = 0.0;

        let (assign6090_e6226, assign6090_e6226_d_n0, assign6090_e6226_d_n1, assign6090_e6226_d_n3, assign6090_e6226_d_n4, assign6090_e6226_d_n5, assign6090_e6226_d_n6, assign6090_e6226_d_n7, assign6090_e6226_d_n8, assign6090_e6226_d_n9, assign6090_e6226_d_n10,) = {
    if ((var_guard110 == 0.0) && (var_guard111 == 0.0)) {
        let assign6090_e6216: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6090_e6218: f64 = (assign6090_e6216 / p.p90);
        let assign6090_e6220: f64 = (assign6090_e6218 * var_vtinv);
        let assign6090_e6222: f64 = (assign6090_e6220 - p.p138);
        let assign6090_e6223: f64 = (1.0 + assign6090_e6222);
        let assign6090_e6224: f64 = (var_expl * assign6090_e6223);
        (assign6090_e6224, (var_expl * (((-var_vdcex_t_dn0) / p.p90) * var_vtinv)), (var_expl * (((-var_vdcex_t_dn1) / p.p90) * var_vtinv)), (var_expl * ((((-var_vdcex_t_dn3) / p.p90) * var_vtinv) + (assign6090_e6218 * var_vtinv_dn3))), (var_expl * (((-var_vdcex_t_dn4) / p.p90) * var_vtinv)), (var_expl * (((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p90) * var_vtinv)), (var_expl * (((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p90) * var_vtinv)), (var_expl * (((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p90) * var_vtinv)), (var_expl * (((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p90) * var_vtinv)), (var_expl * (((-var_vdcex_t_dn9) / p.p90) * var_vtinv)), (var_expl * (((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p90) * var_vtinv)),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10,)
    }
};
        var_evb1c4vdcex = assign6090_e6226;
        var_evb1c4vdcex_dn0 = assign6090_e6226_d_n0;
        var_evb1c4vdcex_dn1 = assign6090_e6226_d_n1;
        var_evb1c4vdcex_dn3 = assign6090_e6226_d_n3;
        var_evb1c4vdcex_dn4 = assign6090_e6226_d_n4;
        var_evb1c4vdcex_dn5 = assign6090_e6226_d_n5;
        var_evb1c4vdcex_dn6 = assign6090_e6226_d_n6;
        var_evb1c4vdcex_dn7 = assign6090_e6226_d_n7;
        var_evb1c4vdcex_dn8 = assign6090_e6226_d_n8;
        var_evb1c4vdcex_dn9 = assign6090_e6226_d_n9;
        var_evb1c4vdcex_dn10 = assign6090_e6226_d_n10;
        var_evb1c4vdcex_rv = 0.0;

        let (assign6100_e6246, assign6100_e6246_d_n0, assign6100_e6246_d_n1, assign6100_e6246_d_n3, assign6100_e6246_d_n4, assign6100_e6246_d_n5, assign6100_e6246_d_n6, assign6100_e6246_d_n7, assign6100_e6246_d_n8, assign6100_e6246_d_n9, assign6100_e6246_d_n10,) = {
    if (var_guard110 == 0.0) {
        let assign6100_e6231: f64 = (2.0 * var_ibx_t);
        let assign6100_e6233: f64 = (assign6100_e6231 * var_tauex_t);
        let assign6100_e6235: f64 = (assign6100_e6233 * var_evb1c4);
        let assign6100_e6240: f64 = (4.0 * var_evb1c4vdcex);
        let assign6100_e6241: f64 = (1.0 + assign6100_e6240);
        let assign6100_e6242: f64 = (assign6100_e6241).sqrt();
        let assign6100_e6243: f64 = (1.0 + assign6100_e6242);
        let assign6100_e6244: f64 = (assign6100_e6235 / assign6100_e6243);
        (assign6100_e6244, (-((assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn0) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), (-((assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn1) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((((((2.0 * var_ibx_t_dn3) * var_tauex_t) + (assign6100_e6231 * var_tauex_t_dn3)) * var_evb1c4) + (assign6100_e6233 * var_evb1c4_dn3)) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn3) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn4) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * var_evb1c4_dn5) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn5) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * var_evb1c4_dn6) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn6) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * var_evb1c4_dn7) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn7) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), ((((assign6100_e6233 * var_evb1c4_dn8) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn8) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)), (-((assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn9) / (2.0 * assign6100_e6242))) / (assign6100_e6243 * assign6100_e6243))), ((((assign6100_e6233 * var_evb1c4_dn10) * assign6100_e6243) - (assign6100_e6235 * ((4.0 * var_evb1c4vdcex_dn10) / (2.0 * assign6100_e6242)))) / (assign6100_e6243 * assign6100_e6243)),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10,)
    }
};
        var_qex = assign6100_e6246;
        var_qex_dn0 = assign6100_e6246_d_n0;
        var_qex_dn1 = assign6100_e6246_d_n1;
        var_qex_dn3 = assign6100_e6246_d_n3;
        var_qex_dn4 = assign6100_e6246_d_n4;
        var_qex_dn5 = assign6100_e6246_d_n5;
        var_qex_dn6 = assign6100_e6246_d_n6;
        var_qex_dn7 = assign6100_e6246_d_n7;
        var_qex_dn8 = assign6100_e6246_d_n8;
        var_qex_dn9 = assign6100_e6246_d_n9;
        var_qex_dn10 = assign6100_e6246_d_n10;
        var_qex_rv = 0.0;

        let assign6110_e6257: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        var_guard112 = assign6110_e6257;
        var_guard112_rv = 0.0;

        let (assign6120_e6263, assign6120_e6263_d_n0, assign6120_e6263_d_n1, assign6120_e6263_d_n3, assign6120_e6263_d_n4, assign6120_e6263_d_n5, assign6120_e6263_d_n6, assign6120_e6263_d_n7, assign6120_e6263_d_n8, assign6120_e6263_d_n9, assign6120_e6263_d_n10,) = {
    if (var_guard112 != 0.0) {
        let assign6120_e6261: f64 = (var_qex * var_xext1);
        (assign6120_e6261, (var_qex_dn0 * var_xext1), (var_qex_dn1 * var_xext1), (var_qex_dn3 * var_xext1), (var_qex_dn4 * var_xext1), (var_qex_dn5 * var_xext1), (var_qex_dn6 * var_xext1), (var_qex_dn7 * var_xext1), (var_qex_dn8 * var_xext1), (var_qex_dn9 * var_xext1), (var_qex_dn10 * var_xext1),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10,)
    }
};
        var_qex = assign6120_e6263;
        var_qex_dn0 = assign6120_e6263_d_n0;
        var_qex_dn1 = assign6120_e6263_d_n1;
        var_qex_dn3 = assign6120_e6263_d_n3;
        var_qex_dn4 = assign6120_e6263_d_n4;
        var_qex_dn5 = assign6120_e6263_d_n5;
        var_qex_dn6 = assign6120_e6263_d_n6;
        var_qex_dn7 = assign6120_e6263_d_n7;
        var_qex_dn8 = assign6120_e6263_d_n8;
        var_qex_dn9 = assign6120_e6263_d_n9;
        var_qex_dn10 = assign6120_e6263_d_n10;
        var_qex_rv = 0.0;

        let assign6130_e6266: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        var_guard113 = assign6130_e6266;
        var_guard113_rv = 0.0;

        let (assign6140_e6274, assign6140_e6274_d_n0, assign6140_e6274_d_n1, assign6140_e6274_d_n3, assign6140_e6274_d_n4, assign6140_e6274_d_n5, assign6140_e6274_d_n6, assign6140_e6274_d_n7, assign6140_e6274_d_n8, assign6140_e6274_d_n9, assign6140_e6274_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6140_e6272: f64 = (var_if0 * var_evbc3);
        (assign6140_e6272, ((var_if0_dn0 * var_evbc3) + (var_if0 * var_evbc3_dn0)), ((var_if0_dn1 * var_evbc3) + (var_if0 * var_evbc3_dn1)), ((var_if0_dn3 * var_evbc3) + (var_if0 * var_evbc3_dn3)), (var_if0_dn4 * var_evbc3), ((var_if0_dn5 * var_evbc3) + (var_if0 * var_evbc3_dn5)), ((var_if0_dn6 * var_evbc3) + (var_if0 * var_evbc3_dn6)), ((var_if0_dn7 * var_evbc3) + (var_if0 * var_evbc3_dn7)), ((var_if0_dn8 * var_evbc3) + (var_if0 * var_evbc3_dn8)), ((var_if0_dn9 * var_evbc3) + (var_if0 * var_evbc3_dn9)), ((var_if0_dn10 * var_evbc3) + (var_if0 * var_evbc3_dn10)),)
    } else {
        (var_xg1, var_xg1_dn0, var_xg1_dn1, var_xg1_dn3, var_xg1_dn4, var_xg1_dn5, var_xg1_dn6, var_xg1_dn7, var_xg1_dn8, var_xg1_dn9, var_xg1_dn10,)
    }
};
        var_xg1 = assign6140_e6274;
        var_xg1_dn0 = assign6140_e6274_d_n0;
        var_xg1_dn1 = assign6140_e6274_d_n1;
        var_xg1_dn3 = assign6140_e6274_d_n3;
        var_xg1_dn4 = assign6140_e6274_d_n4;
        var_xg1_dn5 = assign6140_e6274_d_n5;
        var_xg1_dn6 = assign6140_e6274_d_n6;
        var_xg1_dn7 = assign6140_e6274_d_n7;
        var_xg1_dn8 = assign6140_e6274_d_n8;
        var_xg1_dn9 = assign6140_e6274_d_n9;
        var_xg1_dn10 = assign6140_e6274_d_n10;
        var_xg1_rv = 0.0;

        let (assign6150_e6289, assign6150_e6289_d_n0, assign6150_e6289_d_n1, assign6150_e6289_d_n3, assign6150_e6289_d_n4, assign6150_e6289_d_n5, assign6150_e6289_d_n6, assign6150_e6289_d_n7, assign6150_e6289_d_n8, assign6150_e6289_d_n9, assign6150_e6289_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6150_e6280: f64 = (var_xg1 - var_if0);
        let assign6150_e6284: f64 = (1.0 + var_xg1);
        let assign6150_e6285: f64 = (assign6150_e6284).sqrt();
        let assign6150_e6286: f64 = (1.0 + assign6150_e6285);
        let assign6150_e6287: f64 = (assign6150_e6280 / assign6150_e6286);
        (assign6150_e6287, ((((var_xg1_dn0 - var_if0_dn0) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn0 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn1 - var_if0_dn1) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn1 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn3 - var_if0_dn3) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn3 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn4 - var_if0_dn4) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn4 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn5 - var_if0_dn5) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn5 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn6 - var_if0_dn6) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn6 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn7 - var_if0_dn7) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn7 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn8 - var_if0_dn8) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn8 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn9 - var_if0_dn9) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn9 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)), ((((var_xg1_dn10 - var_if0_dn10) * assign6150_e6286) - (assign6150_e6280 * (var_xg1_dn10 / (2.0 * assign6150_e6285)))) / (assign6150_e6286 * assign6150_e6286)),)
    } else {
        (var_xnbex, var_xnbex_dn0, var_xnbex_dn1, var_xnbex_dn3, var_xnbex_dn4, var_xnbex_dn5, var_xnbex_dn6, var_xnbex_dn7, var_xnbex_dn8, var_xnbex_dn9, var_xnbex_dn10,)
    }
};
        var_xnbex = assign6150_e6289;
        var_xnbex_dn0 = assign6150_e6289_d_n0;
        var_xnbex_dn1 = assign6150_e6289_d_n1;
        var_xnbex_dn3 = assign6150_e6289_d_n3;
        var_xnbex_dn4 = assign6150_e6289_d_n4;
        var_xnbex_dn5 = assign6150_e6289_d_n5;
        var_xnbex_dn6 = assign6150_e6289_d_n6;
        var_xnbex_dn7 = assign6150_e6289_d_n7;
        var_xnbex_dn8 = assign6150_e6289_d_n8;
        var_xnbex_dn9 = assign6150_e6289_d_n9;
        var_xnbex_dn10 = assign6150_e6289_d_n10;
        var_xnbex_rv = 0.0;

        let (assign6160_e6297, assign6160_e6297_d_n0, assign6160_e6297_d_n1, assign6160_e6297_d_n3, assign6160_e6297_d_n4, assign6160_e6297_d_n5, assign6160_e6297_d_n6, assign6160_e6297_d_n7, assign6160_e6297_d_n8, assign6160_e6297_d_n9, assign6160_e6297_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6160_e6295: f64 = (4.0 * var_evbc3vdc);
        (assign6160_e6295, (4.0 * var_evbc3vdc_dn0), (4.0 * var_evbc3vdc_dn1), (4.0 * var_evbc3vdc_dn3), (4.0 * var_evbc3vdc_dn4), (4.0 * var_evbc3vdc_dn5), (4.0 * var_evbc3vdc_dn6), (4.0 * var_evbc3vdc_dn7), (4.0 * var_evbc3vdc_dn8), (4.0 * var_evbc3vdc_dn9), (4.0 * var_evbc3vdc_dn10),)
    } else {
        (var_xg2, var_xg2_dn0, var_xg2_dn1, var_xg2_dn3, var_xg2_dn4, var_xg2_dn5, var_xg2_dn6, var_xg2_dn7, var_xg2_dn8, var_xg2_dn9, var_xg2_dn10,)
    }
};
        var_xg2 = assign6160_e6297;
        var_xg2_dn0 = assign6160_e6297_d_n0;
        var_xg2_dn1 = assign6160_e6297_d_n1;
        var_xg2_dn3 = assign6160_e6297_d_n3;
        var_xg2_dn4 = assign6160_e6297_d_n4;
        var_xg2_dn5 = assign6160_e6297_d_n5;
        var_xg2_dn6 = assign6160_e6297_d_n6;
        var_xg2_dn7 = assign6160_e6297_d_n7;
        var_xg2_dn8 = assign6160_e6297_d_n8;
        var_xg2_dn9 = assign6160_e6297_d_n9;
        var_xg2_dn10 = assign6160_e6297_d_n10;
        var_xg2_rv = 0.0;

        let (assign6170_e6310, assign6170_e6310_d_n0, assign6170_e6310_d_n1, assign6170_e6310_d_n3, assign6170_e6310_d_n4, assign6170_e6310_d_n5, assign6170_e6310_d_n6, assign6170_e6310_d_n7, assign6170_e6310_d_n8, assign6170_e6310_d_n9, assign6170_e6310_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6170_e6305: f64 = (1.0 + var_xg2);
        let assign6170_e6306: f64 = (assign6170_e6305).sqrt();
        let assign6170_e6307: f64 = (1.0 + assign6170_e6306);
        let assign6170_e6308: f64 = (var_xg2 / assign6170_e6307);
        (assign6170_e6308, (((var_xg2_dn0 * assign6170_e6307) - (var_xg2 * (var_xg2_dn0 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn1 * assign6170_e6307) - (var_xg2 * (var_xg2_dn1 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn3 * assign6170_e6307) - (var_xg2 * (var_xg2_dn3 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn4 * assign6170_e6307) - (var_xg2 * (var_xg2_dn4 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn5 * assign6170_e6307) - (var_xg2 * (var_xg2_dn5 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn6 * assign6170_e6307) - (var_xg2 * (var_xg2_dn6 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn7 * assign6170_e6307) - (var_xg2 * (var_xg2_dn7 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn8 * assign6170_e6307) - (var_xg2 * (var_xg2_dn8 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn9 * assign6170_e6307) - (var_xg2 * (var_xg2_dn9 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)), (((var_xg2_dn10 * assign6170_e6307) - (var_xg2 * (var_xg2_dn10 / (2.0 * assign6170_e6306)))) / (assign6170_e6307 * assign6170_e6307)),)
    } else {
        (var_xpwex, var_xpwex_dn0, var_xpwex_dn1, var_xpwex_dn3, var_xpwex_dn4, var_xpwex_dn5, var_xpwex_dn6, var_xpwex_dn7, var_xpwex_dn8, var_xpwex_dn9, var_xpwex_dn10,)
    }
};
        var_xpwex = assign6170_e6310;
        var_xpwex_dn0 = assign6170_e6310_d_n0;
        var_xpwex_dn1 = assign6170_e6310_d_n1;
        var_xpwex_dn3 = assign6170_e6310_d_n3;
        var_xpwex_dn4 = assign6170_e6310_d_n4;
        var_xpwex_dn5 = assign6170_e6310_d_n5;
        var_xpwex_dn6 = assign6170_e6310_d_n6;
        var_xpwex_dn7 = assign6170_e6310_d_n7;
        var_xpwex_dn8 = assign6170_e6310_d_n8;
        var_xpwex_dn9 = assign6170_e6310_d_n9;
        var_xpwex_dn10 = assign6170_e6310_d_n10;
        var_xpwex_rv = 0.0;

        let (assign6180_e6332, assign6180_e6332_d_n0, assign6180_e6332_d_n1, assign6180_e6332_d_n3, assign6180_e6332_d_n4, assign6180_e6332_d_n5, assign6180_e6332_d_n6, assign6180_e6332_d_n7, assign6180_e6332_d_n8, assign6180_e6332_d_n9, assign6180_e6332_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6180_e6316: f64 = (0.5 * p.p32);
        let assign6180_e6318: f64 = (assign6180_e6316 * var_taur_t);
        let assign6180_e6321: f64 = (var_qb0 * var_xnbex);
        let assign6180_e6324: f64 = (var_qepi0 * var_xpwex);
        let assign6180_e6325: f64 = (assign6180_e6321 + assign6180_e6324);
        let assign6180_e6326: f64 = (assign6180_e6318 * assign6180_e6325);
        let assign6180_e6329: f64 = (var_taub_t + var_tepi_t);
        let assign6180_e6330: f64 = (assign6180_e6326 / assign6180_e6329);
        (assign6180_e6330, ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn0) + (var_qepi0 * var_xpwex_dn0))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn1) + (var_qepi0 * var_xpwex_dn1))) / assign6180_e6329), ((((((assign6180_e6316 * var_taur_t_dn3) * assign6180_e6325) + (assign6180_e6318 * (((var_qb0_dn3 * var_xnbex) + (var_qb0 * var_xnbex_dn3)) + ((var_qepi0_dn3 * var_xpwex) + (var_qepi0 * var_xpwex_dn3))))) * assign6180_e6329) - (assign6180_e6326 * (var_taub_t_dn3 + var_tepi_t_dn3))) / (assign6180_e6329 * assign6180_e6329)), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn4) + (var_qepi0 * var_xpwex_dn4))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn5) + (var_qepi0 * var_xpwex_dn5))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn6) + (var_qepi0 * var_xpwex_dn6))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn7) + (var_qepi0 * var_xpwex_dn7))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn8) + (var_qepi0 * var_xpwex_dn8))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn9) + (var_qepi0 * var_xpwex_dn9))) / assign6180_e6329), ((assign6180_e6318 * ((var_qb0 * var_xnbex_dn10) + (var_qepi0 * var_xpwex_dn10))) / assign6180_e6329),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10,)
    }
};
        var_xqmex = assign6180_e6332;
        var_xqmex_dn0 = assign6180_e6332_d_n0;
        var_xqmex_dn1 = assign6180_e6332_d_n1;
        var_xqmex_dn3 = assign6180_e6332_d_n3;
        var_xqmex_dn4 = assign6180_e6332_d_n4;
        var_xqmex_dn5 = assign6180_e6332_d_n5;
        var_xqmex_dn6 = assign6180_e6332_d_n6;
        var_xqmex_dn7 = assign6180_e6332_d_n7;
        var_xqmex_dn8 = assign6180_e6332_d_n8;
        var_xqmex_dn9 = assign6180_e6332_d_n9;
        var_xqmex_dn10 = assign6180_e6332_d_n10;
        var_xqmex_rv = 0.0;

        let assign6190_e6335: f64 = (var_vbc3 - var_vdcex_t);
        let assign6190_e6337: f64 = (assign6190_e6335 * var_vtinv);
        let assign6190_e6339: f64 = if assign6190_e6337 < p.p138 { 1.0 } else { 0.0 };
        var_guard114 = assign6190_e6339;
        var_guard114_rv = 0.0;

        let (assign6200_e6353, assign6200_e6353_d_n0, assign6200_e6353_d_n1, assign6200_e6353_d_n3, assign6200_e6353_d_n4, assign6200_e6353_d_n5, assign6200_e6353_d_n6, assign6200_e6353_d_n7, assign6200_e6353_d_n8, assign6200_e6353_d_n9, assign6200_e6353_d_n10,) = {
    if (((var_guard112 != 0.0) && (var_guard113 == 0.0)) && (var_guard114 != 0.0)) {
        let assign6200_e6348: f64 = (var_vbc3 - var_vdcex_t);
        let assign6200_e6350: f64 = (assign6200_e6348 * var_vtinv);
        let assign6200_e6351: f64 = (assign6200_e6350).exp();
        (assign6200_e6351, (assign6200_e6351 * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (assign6200_e6351 * (((-var_vdcex_t_dn3) * var_vtinv) + (assign6200_e6348 * var_vtinv_dn3))), (assign6200_e6351 * ((-var_vdcex_t_dn4) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (assign6200_e6351 * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10,)
    }
};
        var_evbc3vdcex = assign6200_e6353;
        var_evbc3vdcex_dn0 = assign6200_e6353_d_n0;
        var_evbc3vdcex_dn1 = assign6200_e6353_d_n1;
        var_evbc3vdcex_dn3 = assign6200_e6353_d_n3;
        var_evbc3vdcex_dn4 = assign6200_e6353_d_n4;
        var_evbc3vdcex_dn5 = assign6200_e6353_d_n5;
        var_evbc3vdcex_dn6 = assign6200_e6353_d_n6;
        var_evbc3vdcex_dn7 = assign6200_e6353_d_n7;
        var_evbc3vdcex_dn8 = assign6200_e6353_d_n8;
        var_evbc3vdcex_dn9 = assign6200_e6353_d_n9;
        var_evbc3vdcex_dn10 = assign6200_e6353_d_n10;
        var_evbc3vdcex_rv = 0.0;

        let (assign6210_e6364,) = {
    if (((var_guard112 != 0.0) && (var_guard113 == 0.0)) && (var_guard114 == 0.0)) {
        let assign6210_e6362: f64 = (p.p138).exp();
        (assign6210_e6362,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6210_e6364;
        var_expl_rv = 0.0;

        let (assign6220_e6384, assign6220_e6384_d_n0, assign6220_e6384_d_n1, assign6220_e6384_d_n3, assign6220_e6384_d_n4, assign6220_e6384_d_n5, assign6220_e6384_d_n6, assign6220_e6384_d_n7, assign6220_e6384_d_n8, assign6220_e6384_d_n9, assign6220_e6384_d_n10,) = {
    if (((var_guard112 != 0.0) && (var_guard113 == 0.0)) && (var_guard114 == 0.0)) {
        let assign6220_e6376: f64 = (var_vbc3 - var_vdcex_t);
        let assign6220_e6378: f64 = (assign6220_e6376 * var_vtinv);
        let assign6220_e6380: f64 = (assign6220_e6378 - p.p138);
        let assign6220_e6381: f64 = (1.0 + assign6220_e6380);
        let assign6220_e6382: f64 = (var_expl * assign6220_e6381);
        (assign6220_e6382, (var_expl * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (var_expl * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (var_expl * (((-var_vdcex_t_dn3) * var_vtinv) + (assign6220_e6376 * var_vtinv_dn3))), (var_expl * ((-var_vdcex_t_dn4) * var_vtinv)), (var_expl * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (var_expl * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (var_expl * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (var_expl * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (var_expl * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (var_expl * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10,)
    }
};
        var_evbc3vdcex = assign6220_e6384;
        var_evbc3vdcex_dn0 = assign6220_e6384_d_n0;
        var_evbc3vdcex_dn1 = assign6220_e6384_d_n1;
        var_evbc3vdcex_dn3 = assign6220_e6384_d_n3;
        var_evbc3vdcex_dn4 = assign6220_e6384_d_n4;
        var_evbc3vdcex_dn5 = assign6220_e6384_d_n5;
        var_evbc3vdcex_dn6 = assign6220_e6384_d_n6;
        var_evbc3vdcex_dn7 = assign6220_e6384_d_n7;
        var_evbc3vdcex_dn8 = assign6220_e6384_d_n8;
        var_evbc3vdcex_dn9 = assign6220_e6384_d_n9;
        var_evbc3vdcex_dn10 = assign6220_e6384_d_n10;
        var_evbc3vdcex_rv = 0.0;

        let (assign6230_e6408, assign6230_e6408_d_n0, assign6230_e6408_d_n1, assign6230_e6408_d_n3, assign6230_e6408_d_n4, assign6230_e6408_d_n5, assign6230_e6408_d_n6, assign6230_e6408_d_n7, assign6230_e6408_d_n8, assign6230_e6408_d_n9, assign6230_e6408_d_n10,) = {
    if ((var_guard112 != 0.0) && (var_guard113 == 0.0)) {
        let assign6230_e6391: f64 = (2.0 * p.p32);
        let assign6230_e6393: f64 = (assign6230_e6391 * var_ibx_t);
        let assign6230_e6395: f64 = (assign6230_e6393 * var_tauex_t);
        let assign6230_e6397: f64 = (assign6230_e6395 * var_evbc3);
        let assign6230_e6402: f64 = (4.0 * var_evbc3vdcex);
        let assign6230_e6403: f64 = (1.0 + assign6230_e6402);
        let assign6230_e6404: f64 = (assign6230_e6403).sqrt();
        let assign6230_e6405: f64 = (1.0 + assign6230_e6404);
        let assign6230_e6406: f64 = (assign6230_e6397 / assign6230_e6405);
        (assign6230_e6406, ((((assign6230_e6395 * var_evbc3_dn0) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn0) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn1) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn1) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((((((assign6230_e6391 * var_ibx_t_dn3) * var_tauex_t) + (assign6230_e6393 * var_tauex_t_dn3)) * var_evbc3) + (assign6230_e6395 * var_evbc3_dn3)) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn3) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), (-((assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn4) / (2.0 * assign6230_e6404))) / (assign6230_e6405 * assign6230_e6405))), ((((assign6230_e6395 * var_evbc3_dn5) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn5) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn6) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn6) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn7) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn7) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn8) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn8) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn9) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn9) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)), ((((assign6230_e6395 * var_evbc3_dn10) * assign6230_e6405) - (assign6230_e6397 * ((4.0 * var_evbc3vdcex_dn10) / (2.0 * assign6230_e6404)))) / (assign6230_e6405 * assign6230_e6405)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10,)
    }
};
        var_xqmex = assign6230_e6408;
        var_xqmex_dn0 = assign6230_e6408_d_n0;
        var_xqmex_dn1 = assign6230_e6408_d_n1;
        var_xqmex_dn3 = assign6230_e6408_d_n3;
        var_xqmex_dn4 = assign6230_e6408_d_n4;
        var_xqmex_dn5 = assign6230_e6408_d_n5;
        var_xqmex_dn6 = assign6230_e6408_d_n6;
        var_xqmex_dn7 = assign6230_e6408_d_n7;
        var_xqmex_dn8 = assign6230_e6408_d_n8;
        var_xqmex_dn9 = assign6230_e6408_d_n9;
        var_xqmex_dn10 = assign6230_e6408_d_n10;
        var_xqmex_rv = 0.0;

        let (assign6240_e6414, assign6240_e6414_d_n0, assign6240_e6414_d_n1, assign6240_e6414_d_n3, assign6240_e6414_d_n4, assign6240_e6414_d_n5, assign6240_e6414_d_n6, assign6240_e6414_d_n7, assign6240_e6414_d_n8, assign6240_e6414_d_n9, assign6240_e6414_d_n10,) = {
    if (var_guard112 != 0.0) {
        let assign6240_e6412: f64 = (var_fex * var_xqmex);
        (assign6240_e6412, ((var_fex_dn0 * var_xqmex) + (var_fex * var_xqmex_dn0)), ((var_fex_dn1 * var_xqmex) + (var_fex * var_xqmex_dn1)), ((var_fex_dn3 * var_xqmex) + (var_fex * var_xqmex_dn3)), ((var_fex_dn4 * var_xqmex) + (var_fex * var_xqmex_dn4)), ((var_fex_dn5 * var_xqmex) + (var_fex * var_xqmex_dn5)), ((var_fex_dn6 * var_xqmex) + (var_fex * var_xqmex_dn6)), ((var_fex_dn7 * var_xqmex) + (var_fex * var_xqmex_dn7)), ((var_fex_dn8 * var_xqmex) + (var_fex * var_xqmex_dn8)), ((var_fex_dn9 * var_xqmex) + (var_fex * var_xqmex_dn9)), ((var_fex_dn10 * var_xqmex) + (var_fex * var_xqmex_dn10)),)
    } else {
        (var_xqex, var_xqex_dn0, var_xqex_dn1, var_xqex_dn3, var_xqex_dn4, var_xqex_dn5, var_xqex_dn6, var_xqex_dn7, var_xqex_dn8, var_xqex_dn9, var_xqex_dn10,)
    }
};
        var_xqex = assign6240_e6414;
        var_xqex_dn0 = assign6240_e6414_d_n0;
        var_xqex_dn1 = assign6240_e6414_d_n1;
        var_xqex_dn3 = assign6240_e6414_d_n3;
        var_xqex_dn4 = assign6240_e6414_d_n4;
        var_xqex_dn5 = assign6240_e6414_d_n5;
        var_xqex_dn6 = assign6240_e6414_d_n6;
        var_xqex_dn7 = assign6240_e6414_d_n7;
        var_xqex_dn8 = assign6240_e6414_d_n8;
        var_xqex_dn9 = assign6240_e6414_d_n9;
        var_xqex_dn10 = assign6240_e6414_d_n10;
        var_xqex_rv = 0.0;

        let assign6250_e6417: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign6250_e6417;
        var_guard115_rv = 0.0;

        let (assign6260_e6430, assign6260_e6430_d_n0, assign6260_e6430_d_n1, assign6260_e6430_d_n3, assign6260_e6430_d_n4, assign6260_e6430_d_n5, assign6260_e6430_d_n6, assign6260_e6430_d_n7, assign6260_e6430_d_n8, assign6260_e6430_d_n9, assign6260_e6430_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6260_e6422: f64 = (var_vje * var_inv_vde_t);
        let assign6260_e6423: f64 = (1.0 - assign6260_e6422);
        let assign6260_e6425: f64 = (-p.p66);
        let assign6260_e6426: f64 = (assign6260_e6423).powf(assign6260_e6425);
        let assign6260_e6428: f64 = (assign6260_e6426 - 3.0);
        (assign6260_e6428, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6260_e6423))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10,)
    }
};
        var_dvtevje = assign6260_e6430;
        var_dvtevje_dn0 = assign6260_e6430_d_n0;
        var_dvtevje_dn1 = assign6260_e6430_d_n1;
        var_dvtevje_dn3 = assign6260_e6430_d_n3;
        var_dvtevje_dn4 = assign6260_e6430_d_n4;
        var_dvtevje_dn5 = assign6260_e6430_d_n5;
        var_dvtevje_dn6 = assign6260_e6430_d_n6;
        var_dvtevje_dn7 = assign6260_e6430_d_n7;
        var_dvtevje_dn8 = assign6260_e6430_d_n8;
        var_dvtevje_dn9 = assign6260_e6430_d_n9;
        var_dvtevje_dn10 = assign6260_e6430_d_n10;
        var_dvtevje_rv = 0.0;

        let (assign6270_e6438, assign6270_e6438_d_n0, assign6270_e6438_d_n1, assign6270_e6438_d_n3, assign6270_e6438_d_n4, assign6270_e6438_d_n5, assign6270_e6438_d_n6, assign6270_e6438_d_n7, assign6270_e6438_d_n8, assign6270_e6438_d_n9, assign6270_e6438_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6270_e6434: f64 = (var_vb2e1 - var_vfe);
        let assign6270_e6436: f64 = (assign6270_e6434 / var_a_vde);
        (assign6270_e6436, ((((-var_vfe_dn0) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn1) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn3) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn5) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn7) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn8) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn9) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn10) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10,)
    }
};
        var_vb2e1vfe = assign6270_e6438;
        var_vb2e1vfe_dn0 = assign6270_e6438_d_n0;
        var_vb2e1vfe_dn1 = assign6270_e6438_d_n1;
        var_vb2e1vfe_dn3 = assign6270_e6438_d_n3;
        var_vb2e1vfe_dn4 = assign6270_e6438_d_n4;
        var_vb2e1vfe_dn5 = assign6270_e6438_d_n5;
        var_vb2e1vfe_dn6 = assign6270_e6438_d_n6;
        var_vb2e1vfe_dn7 = assign6270_e6438_d_n7;
        var_vb2e1vfe_dn8 = assign6270_e6438_d_n8;
        var_vb2e1vfe_dn9 = assign6270_e6438_d_n9;
        var_vb2e1vfe_dn10 = assign6270_e6438_d_n10;
        var_vb2e1vfe_rv = 0.0;

        let assign6280_e6441: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard116 = assign6280_e6441;
        var_guard116_rv = 0.0;

        let (assign6290_e6452, assign6290_e6452_d_n0, assign6290_e6452_d_n1, assign6290_e6452_d_n3, assign6290_e6452_d_n4, assign6290_e6452_d_n5, assign6290_e6452_d_n6, assign6290_e6452_d_n7, assign6290_e6452_d_n8, assign6290_e6452_d_n9, assign6290_e6452_d_n10,) = {
    if ((var_guard115 != 0.0) && (var_guard116 != 0.0)) {
        let assign6290_e6448: f64 = (var_vb2e1vfe).exp();
        let assign6290_e6449: f64 = (1.0 + assign6290_e6448);
        let assign6290_e6450: f64 = (1.0 / assign6290_e6449);
        (assign6290_e6450, (-((assign6290_e6448 * var_vb2e1vfe_dn0) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn1) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn3) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn4) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn5) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn6) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn7) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn8) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn9) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn10) / (assign6290_e6449 * assign6290_e6449))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10,)
    }
};
        var_dvjevb2e1 = assign6290_e6452;
        var_dvjevb2e1_dn0 = assign6290_e6452_d_n0;
        var_dvjevb2e1_dn1 = assign6290_e6452_d_n1;
        var_dvjevb2e1_dn3 = assign6290_e6452_d_n3;
        var_dvjevb2e1_dn4 = assign6290_e6452_d_n4;
        var_dvjevb2e1_dn5 = assign6290_e6452_d_n5;
        var_dvjevb2e1_dn6 = assign6290_e6452_d_n6;
        var_dvjevb2e1_dn7 = assign6290_e6452_d_n7;
        var_dvjevb2e1_dn8 = assign6290_e6452_d_n8;
        var_dvjevb2e1_dn9 = assign6290_e6452_d_n9;
        var_dvjevb2e1_dn10 = assign6290_e6452_d_n10;
        var_dvjevb2e1_rv = 0.0;

        let (assign6300_e6467, assign6300_e6467_d_n0, assign6300_e6467_d_n1, assign6300_e6467_d_n3, assign6300_e6467_d_n4, assign6300_e6467_d_n5, assign6300_e6467_d_n6, assign6300_e6467_d_n7, assign6300_e6467_d_n8, assign6300_e6467_d_n9, assign6300_e6467_d_n10,) = {
    if ((var_guard115 != 0.0) && (var_guard116 == 0.0)) {
        let assign6300_e6458: f64 = (-var_vb2e1vfe);
        let assign6300_e6459: f64 = (assign6300_e6458).exp();
        let assign6300_e6462: f64 = (-var_vb2e1vfe);
        let assign6300_e6463: f64 = (assign6300_e6462).exp();
        let assign6300_e6464: f64 = (1.0 + assign6300_e6463);
        let assign6300_e6465: f64 = (assign6300_e6459 / assign6300_e6464);
        (assign6300_e6465, ((((assign6300_e6459 * (-var_vb2e1vfe_dn0)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn0)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn1)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn1)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn3)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn3)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn4)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn4)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn5)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn5)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn6)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn6)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn7)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn7)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn8)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn8)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn9)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn9)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn10)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn10)))) / (assign6300_e6464 * assign6300_e6464)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10,)
    }
};
        var_dvjevb2e1 = assign6300_e6467;
        var_dvjevb2e1_dn0 = assign6300_e6467_d_n0;
        var_dvjevb2e1_dn1 = assign6300_e6467_d_n1;
        var_dvjevb2e1_dn3 = assign6300_e6467_d_n3;
        var_dvjevb2e1_dn4 = assign6300_e6467_d_n4;
        var_dvjevb2e1_dn5 = assign6300_e6467_d_n5;
        var_dvjevb2e1_dn6 = assign6300_e6467_d_n6;
        var_dvjevb2e1_dn7 = assign6300_e6467_d_n7;
        var_dvjevb2e1_dn8 = assign6300_e6467_d_n8;
        var_dvjevb2e1_dn9 = assign6300_e6467_d_n9;
        var_dvjevb2e1_dn10 = assign6300_e6467_d_n10;
        var_dvjevb2e1_rv = 0.0;

        let (assign6310_e6475, assign6310_e6475_d_n0, assign6310_e6475_d_n1, assign6310_e6475_d_n3, assign6310_e6475_d_n4, assign6310_e6475_d_n5, assign6310_e6475_d_n6, assign6310_e6475_d_n7, assign6310_e6475_d_n8, assign6310_e6475_d_n9, assign6310_e6475_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6310_e6471: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6310_e6473: f64 = (assign6310_e6471 + 3.0);
        (assign6310_e6473, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10,)
    }
};
        var_dvtevb2e1 = assign6310_e6475;
        var_dvtevb2e1_dn0 = assign6310_e6475_d_n0;
        var_dvtevb2e1_dn1 = assign6310_e6475_d_n1;
        var_dvtevb2e1_dn3 = assign6310_e6475_d_n3;
        var_dvtevb2e1_dn4 = assign6310_e6475_d_n4;
        var_dvtevb2e1_dn5 = assign6310_e6475_d_n5;
        var_dvtevb2e1_dn6 = assign6310_e6475_d_n6;
        var_dvtevb2e1_dn7 = assign6310_e6475_d_n7;
        var_dvtevb2e1_dn8 = assign6310_e6475_d_n8;
        var_dvtevb2e1_dn9 = assign6310_e6475_d_n9;
        var_dvtevb2e1_dn10 = assign6310_e6475_d_n10;
        var_dvtevb2e1_rv = 0.0;

        let (assign6320_e6485, assign6320_e6485_d_n0, assign6320_e6485_d_n1, assign6320_e6485_d_n3, assign6320_e6485_d_n4, assign6320_e6485_d_n5, assign6320_e6485_d_n6, assign6320_e6485_d_n7, assign6320_e6485_d_n8, assign6320_e6485_d_n9, assign6320_e6485_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6320_e6479: f64 = (1.0 - p.p67);
        let assign6320_e6481: f64 = (assign6320_e6479 * var_cje_t);
        let assign6320_e6483: f64 = (assign6320_e6481 * var_dvtevb2e1);
        (assign6320_e6483, (((assign6320_e6479 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn0)), (((assign6320_e6479 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn1)), (((assign6320_e6479 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn3)), (((assign6320_e6479 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn4)), (((assign6320_e6479 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn5)), (((assign6320_e6479 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn6)), (((assign6320_e6479 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn7)), (((assign6320_e6479 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn8)), (((assign6320_e6479 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn9)), (((assign6320_e6479 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn10)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10,)
    }
};
        var_dqtevb2e1 = assign6320_e6485;
        var_dqtevb2e1_dn0 = assign6320_e6485_d_n0;
        var_dqtevb2e1_dn1 = assign6320_e6485_d_n1;
        var_dqtevb2e1_dn3 = assign6320_e6485_d_n3;
        var_dqtevb2e1_dn4 = assign6320_e6485_d_n4;
        var_dqtevb2e1_dn5 = assign6320_e6485_d_n5;
        var_dqtevb2e1_dn6 = assign6320_e6485_d_n6;
        var_dqtevb2e1_dn7 = assign6320_e6485_d_n7;
        var_dqtevb2e1_dn8 = assign6320_e6485_d_n8;
        var_dqtevb2e1_dn9 = assign6320_e6485_d_n9;
        var_dqtevb2e1_dn10 = assign6320_e6485_d_n10;
        var_dqtevb2e1_rv = 0.0;

        *var_dqtevb2e1_slot = var_dqtevb2e1;
        *var_dqtevb2e1_dn0_slot = var_dqtevb2e1_dn0;
        *var_dqtevb2e1_dn1_slot = var_dqtevb2e1_dn1;
        *var_dqtevb2e1_dn10_slot = var_dqtevb2e1_dn10;
        *var_dqtevb2e1_dn3_slot = var_dqtevb2e1_dn3;
        *var_dqtevb2e1_dn4_slot = var_dqtevb2e1_dn4;
        *var_dqtevb2e1_dn5_slot = var_dqtevb2e1_dn5;
        *var_dqtevb2e1_dn6_slot = var_dqtevb2e1_dn6;
        *var_dqtevb2e1_dn7_slot = var_dqtevb2e1_dn7;
        *var_dqtevb2e1_dn8_slot = var_dqtevb2e1_dn8;
        *var_dqtevb2e1_dn9_slot = var_dqtevb2e1_dn9;
        *var_dqtevb2e1_rv_slot = var_dqtevb2e1_rv;
        *var_dvjevb2e1_slot = var_dvjevb2e1;
        *var_dvjevb2e1_dn0_slot = var_dvjevb2e1_dn0;
        *var_dvjevb2e1_dn1_slot = var_dvjevb2e1_dn1;
        *var_dvjevb2e1_dn10_slot = var_dvjevb2e1_dn10;
        *var_dvjevb2e1_dn3_slot = var_dvjevb2e1_dn3;
        *var_dvjevb2e1_dn4_slot = var_dvjevb2e1_dn4;
        *var_dvjevb2e1_dn5_slot = var_dvjevb2e1_dn5;
        *var_dvjevb2e1_dn6_slot = var_dvjevb2e1_dn6;
        *var_dvjevb2e1_dn7_slot = var_dvjevb2e1_dn7;
        *var_dvjevb2e1_dn8_slot = var_dvjevb2e1_dn8;
        *var_dvjevb2e1_dn9_slot = var_dvjevb2e1_dn9;
        *var_dvjevb2e1_rv_slot = var_dvjevb2e1_rv;
        *var_dvtevb2e1_slot = var_dvtevb2e1;
        *var_dvtevb2e1_dn0_slot = var_dvtevb2e1_dn0;
        *var_dvtevb2e1_dn1_slot = var_dvtevb2e1_dn1;
        *var_dvtevb2e1_dn10_slot = var_dvtevb2e1_dn10;
        *var_dvtevb2e1_dn3_slot = var_dvtevb2e1_dn3;
        *var_dvtevb2e1_dn4_slot = var_dvtevb2e1_dn4;
        *var_dvtevb2e1_dn5_slot = var_dvtevb2e1_dn5;
        *var_dvtevb2e1_dn6_slot = var_dvtevb2e1_dn6;
        *var_dvtevb2e1_dn7_slot = var_dvtevb2e1_dn7;
        *var_dvtevb2e1_dn8_slot = var_dvtevb2e1_dn8;
        *var_dvtevb2e1_dn9_slot = var_dvtevb2e1_dn9;
        *var_dvtevb2e1_rv_slot = var_dvtevb2e1_rv;
        *var_dvtevje_slot = var_dvtevje;
        *var_dvtevje_dn0_slot = var_dvtevje_dn0;
        *var_dvtevje_dn1_slot = var_dvtevje_dn1;
        *var_dvtevje_dn10_slot = var_dvtevje_dn10;
        *var_dvtevje_dn3_slot = var_dvtevje_dn3;
        *var_dvtevje_dn4_slot = var_dvtevje_dn4;
        *var_dvtevje_dn5_slot = var_dvtevje_dn5;
        *var_dvtevje_dn6_slot = var_dvtevje_dn6;
        *var_dvtevje_dn7_slot = var_dvtevje_dn7;
        *var_dvtevje_dn8_slot = var_dvtevje_dn8;
        *var_dvtevje_dn9_slot = var_dvtevje_dn9;
        *var_dvtevje_rv_slot = var_dvtevje_rv;
        *var_evb1c4vdcex_slot = var_evb1c4vdcex;
        *var_evb1c4vdcex_dn0_slot = var_evb1c4vdcex_dn0;
        *var_evb1c4vdcex_dn1_slot = var_evb1c4vdcex_dn1;
        *var_evb1c4vdcex_dn10_slot = var_evb1c4vdcex_dn10;
        *var_evb1c4vdcex_dn3_slot = var_evb1c4vdcex_dn3;
        *var_evb1c4vdcex_dn4_slot = var_evb1c4vdcex_dn4;
        *var_evb1c4vdcex_dn5_slot = var_evb1c4vdcex_dn5;
        *var_evb1c4vdcex_dn6_slot = var_evb1c4vdcex_dn6;
        *var_evb1c4vdcex_dn7_slot = var_evb1c4vdcex_dn7;
        *var_evb1c4vdcex_dn8_slot = var_evb1c4vdcex_dn8;
        *var_evb1c4vdcex_dn9_slot = var_evb1c4vdcex_dn9;
        *var_evb1c4vdcex_rv_slot = var_evb1c4vdcex_rv;
        *var_evbc3vdcex_slot = var_evbc3vdcex;
        *var_evbc3vdcex_dn0_slot = var_evbc3vdcex_dn0;
        *var_evbc3vdcex_dn1_slot = var_evbc3vdcex_dn1;
        *var_evbc3vdcex_dn10_slot = var_evbc3vdcex_dn10;
        *var_evbc3vdcex_dn3_slot = var_evbc3vdcex_dn3;
        *var_evbc3vdcex_dn4_slot = var_evbc3vdcex_dn4;
        *var_evbc3vdcex_dn5_slot = var_evbc3vdcex_dn5;
        *var_evbc3vdcex_dn6_slot = var_evbc3vdcex_dn6;
        *var_evbc3vdcex_dn7_slot = var_evbc3vdcex_dn7;
        *var_evbc3vdcex_dn8_slot = var_evbc3vdcex_dn8;
        *var_evbc3vdcex_dn9_slot = var_evbc3vdcex_dn9;
        *var_evbc3vdcex_rv_slot = var_evbc3vdcex_rv;
        *var_expl_slot = var_expl;
        *var_expl_rv_slot = var_expl_rv;
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
        *var_qex_slot = var_qex;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_qex_rv_slot = var_qex_rv;
        *var_vb2e1vfe_slot = var_vb2e1vfe;
        *var_vb2e1vfe_dn0_slot = var_vb2e1vfe_dn0;
        *var_vb2e1vfe_dn1_slot = var_vb2e1vfe_dn1;
        *var_vb2e1vfe_dn10_slot = var_vb2e1vfe_dn10;
        *var_vb2e1vfe_dn3_slot = var_vb2e1vfe_dn3;
        *var_vb2e1vfe_dn4_slot = var_vb2e1vfe_dn4;
        *var_vb2e1vfe_dn5_slot = var_vb2e1vfe_dn5;
        *var_vb2e1vfe_dn6_slot = var_vb2e1vfe_dn6;
        *var_vb2e1vfe_dn7_slot = var_vb2e1vfe_dn7;
        *var_vb2e1vfe_dn8_slot = var_vb2e1vfe_dn8;
        *var_vb2e1vfe_dn9_slot = var_vb2e1vfe_dn9;
        *var_vb2e1vfe_rv_slot = var_vb2e1vfe_rv;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn0_slot = var_xg1_dn0;
        *var_xg1_dn1_slot = var_xg1_dn1;
        *var_xg1_dn10_slot = var_xg1_dn10;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
        *var_xg1_dn9_slot = var_xg1_dn9;
        *var_xg1_rv_slot = var_xg1_rv;
        *var_xg2_slot = var_xg2;
        *var_xg2_dn0_slot = var_xg2_dn0;
        *var_xg2_dn1_slot = var_xg2_dn1;
        *var_xg2_dn10_slot = var_xg2_dn10;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
        *var_xg2_dn9_slot = var_xg2_dn9;
        *var_xg2_rv_slot = var_xg2_rv;
        *var_xnbex_slot = var_xnbex;
        *var_xnbex_dn0_slot = var_xnbex_dn0;
        *var_xnbex_dn1_slot = var_xnbex_dn1;
        *var_xnbex_dn10_slot = var_xnbex_dn10;
        *var_xnbex_dn3_slot = var_xnbex_dn3;
        *var_xnbex_dn4_slot = var_xnbex_dn4;
        *var_xnbex_dn5_slot = var_xnbex_dn5;
        *var_xnbex_dn6_slot = var_xnbex_dn6;
        *var_xnbex_dn7_slot = var_xnbex_dn7;
        *var_xnbex_dn8_slot = var_xnbex_dn8;
        *var_xnbex_dn9_slot = var_xnbex_dn9;
        *var_xnbex_rv_slot = var_xnbex_rv;
        *var_xpwex_slot = var_xpwex;
        *var_xpwex_dn0_slot = var_xpwex_dn0;
        *var_xpwex_dn1_slot = var_xpwex_dn1;
        *var_xpwex_dn10_slot = var_xpwex_dn10;
        *var_xpwex_dn3_slot = var_xpwex_dn3;
        *var_xpwex_dn4_slot = var_xpwex_dn4;
        *var_xpwex_dn5_slot = var_xpwex_dn5;
        *var_xpwex_dn6_slot = var_xpwex_dn6;
        *var_xpwex_dn7_slot = var_xpwex_dn7;
        *var_xpwex_dn8_slot = var_xpwex_dn8;
        *var_xpwex_dn9_slot = var_xpwex_dn9;
        *var_xpwex_rv_slot = var_xpwex_rv;
        *var_xqex_slot = var_xqex;
        *var_xqex_dn0_slot = var_xqex_dn0;
        *var_xqex_dn1_slot = var_xqex_dn1;
        *var_xqex_dn10_slot = var_xqex_dn10;
        *var_xqex_dn3_slot = var_xqex_dn3;
        *var_xqex_dn4_slot = var_xqex_dn4;
        *var_xqex_dn5_slot = var_xqex_dn5;
        *var_xqex_dn6_slot = var_xqex_dn6;
        *var_xqex_dn7_slot = var_xqex_dn7;
        *var_xqex_dn8_slot = var_xqex_dn8;
        *var_xqex_dn9_slot = var_xqex_dn9;
        *var_xqex_rv_slot = var_xqex_rv;
        *var_xqmex_slot = var_xqmex;
        *var_xqmex_dn0_slot = var_xqmex_dn0;
        *var_xqmex_dn1_slot = var_xqmex_dn1;
        *var_xqmex_dn10_slot = var_xqmex_dn10;
        *var_xqmex_dn3_slot = var_xqmex_dn3;
        *var_xqmex_dn4_slot = var_xqmex_dn4;
        *var_xqmex_dn5_slot = var_xqmex_dn5;
        *var_xqmex_dn6_slot = var_xqmex_dn6;
        *var_xqmex_dn7_slot = var_xqmex_dn7;
        *var_xqmex_dn8_slot = var_xqmex_dn8;
        *var_xqmex_dn9_slot = var_xqmex_dn9;
        *var_xqmex_rv_slot = var_xqmex_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_dqtevb2e1: f64,
        var_dqtevb2e1_dn0: f64,
        var_dqtevb2e1_dn1: f64,
        var_dqtevb2e1_dn10: f64,
        var_dqtevb2e1_dn3: f64,
        var_dqtevb2e1_dn4: f64,
        var_dqtevb2e1_dn5: f64,
        var_dqtevb2e1_dn6: f64,
        var_dqtevb2e1_dn7: f64,
        var_dqtevb2e1_dn8: f64,
        var_dqtevb2e1_dn9: f64,
        var_evb2e1: f64,
        var_evb2e1_dn0: f64,
        var_evb2e1_dn1: f64,
        var_evb2e1_dn10: f64,
        var_evb2e1_dn3: f64,
        var_evb2e1_dn4: f64,
        var_evb2e1_dn5: f64,
        var_evb2e1_dn6: f64,
        var_evb2e1_dn7: f64,
        var_evb2e1_dn8: f64,
        var_evb2e1_dn9: f64,
        var_f1: f64,
        var_f1_dn0: f64,
        var_f1_dn1: f64,
        var_f1_dn10: f64,
        var_f1_dn3: f64,
        var_f1_dn4: f64,
        var_f1_dn5: f64,
        var_f1_dn6: f64,
        var_f1_dn7: f64,
        var_f1_dn8: f64,
        var_f1_dn9: f64,
        var_guard115: f64,
        var_if0: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn10: f64,
        var_if__dn3: f64,
        var_if__dn4: f64,
        var_if__dn5: f64,
        var_if__dn6: f64,
        var_if__dn7: f64,
        var_if__dn8: f64,
        var_if__dn9: f64,
        var_ir: f64,
        var_ir_dn0: f64,
        var_ir_dn1: f64,
        var_ir_dn10: f64,
        var_ir_dn3: f64,
        var_ir_dn4: f64,
        var_ir_dn5: f64,
        var_ir_dn6: f64,
        var_ir_dn7: f64,
        var_ir_dn8: f64,
        var_ir_dn9: f64,
        var_nff_t: f64,
        var_nff_t_dn0: f64,
        var_nff_t_dn1: f64,
        var_nff_t_dn10: f64,
        var_nff_t_dn3: f64,
        var_nff_t_dn4: f64,
        var_nff_t_dn5: f64,
        var_nff_t_dn6: f64,
        var_nff_t_dn7: f64,
        var_nff_t_dn8: f64,
        var_nff_t_dn9: f64,
        var_q1q: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qb0: f64,
        var_qb0_dn3: f64,
        var_qbc_qs: f64,
        var_qbc_qs_dn0: f64,
        var_qbc_qs_dn1: f64,
        var_qbc_qs_dn10: f64,
        var_qbc_qs_dn3: f64,
        var_qbc_qs_dn4: f64,
        var_qbc_qs_dn5: f64,
        var_qbc_qs_dn6: f64,
        var_qbc_qs_dn7: f64,
        var_qbc_qs_dn8: f64,
        var_qbc_qs_dn9: f64,
        var_qbe_qs: f64,
        var_qbe_qs_dn0: f64,
        var_qbe_qs_dn1: f64,
        var_qbe_qs_dn10: f64,
        var_qbe_qs_dn3: f64,
        var_qbe_qs_dn4: f64,
        var_qbe_qs_dn5: f64,
        var_qbe_qs_dn6: f64,
        var_qbe_qs_dn7: f64,
        var_qbe_qs_dn8: f64,
        var_qbe_qs_dn9: f64,
        var_qbi: f64,
        var_qbi_dn0: f64,
        var_qbi_dn1: f64,
        var_qbi_dn10: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_qe_qs: f64,
        var_qe_qs_dn0: f64,
        var_qe_qs_dn1: f64,
        var_qe_qs_dn10: f64,
        var_qe_qs_dn3: f64,
        var_qe_qs_dn4: f64,
        var_qe_qs_dn5: f64,
        var_qe_qs_dn6: f64,
        var_qe_qs_dn7: f64,
        var_qe_qs_dn8: f64,
        var_qe_qs_dn9: f64,
        var_taub_t: f64,
        var_taub_t_dn3: f64,
        var_vb1b2: f64,
        var_vb1b2_dn5: f64,
        var_vb1b2_dn6: f64,
        var_vt: f64,
        var_vt_dn3: f64,
        var_vtinv: f64,
        var_vtinv_dn3: f64,
        var_dn0vb2e1_slot: &mut f64,
        var_dn0vb2e1_dn0_slot: &mut f64,
        var_dn0vb2e1_dn1_slot: &mut f64,
        var_dn0vb2e1_dn10_slot: &mut f64,
        var_dn0vb2e1_dn3_slot: &mut f64,
        var_dn0vb2e1_dn4_slot: &mut f64,
        var_dn0vb2e1_dn5_slot: &mut f64,
        var_dn0vb2e1_dn6_slot: &mut f64,
        var_dn0vb2e1_dn7_slot: &mut f64,
        var_dn0vb2e1_dn8_slot: &mut f64,
        var_dn0vb2e1_dn9_slot: &mut f64,
        var_dn0vb2e1_rv_slot: &mut f64,
        var_dqbevb2e1_slot: &mut f64,
        var_dqbevb2e1_dn0_slot: &mut f64,
        var_dqbevb2e1_dn1_slot: &mut f64,
        var_dqbevb2e1_dn10_slot: &mut f64,
        var_dqbevb2e1_dn3_slot: &mut f64,
        var_dqbevb2e1_dn4_slot: &mut f64,
        var_dqbevb2e1_dn5_slot: &mut f64,
        var_dqbevb2e1_dn6_slot: &mut f64,
        var_dqbevb2e1_dn7_slot: &mut f64,
        var_dqbevb2e1_dn8_slot: &mut f64,
        var_dqbevb2e1_dn9_slot: &mut f64,
        var_dqbevb2e1_rv_slot: &mut f64,
        var_dqevb2e1_slot: &mut f64,
        var_dqevb2e1_dn0_slot: &mut f64,
        var_dqevb2e1_dn1_slot: &mut f64,
        var_dqevb2e1_dn10_slot: &mut f64,
        var_dqevb2e1_dn3_slot: &mut f64,
        var_dqevb2e1_dn4_slot: &mut f64,
        var_dqevb2e1_dn5_slot: &mut f64,
        var_dqevb2e1_dn6_slot: &mut f64,
        var_dqevb2e1_dn7_slot: &mut f64,
        var_dqevb2e1_dn8_slot: &mut f64,
        var_dqevb2e1_dn9_slot: &mut f64,
        var_dqevb2e1_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_i_cth_slot: &mut f64,
        var_i_cth_dn3_slot: &mut f64,
        var_i_cth_rdn3_slot: &mut f64,
        var_i_cth_rv_slot: &mut f64,
        var_in_n_slot: &mut f64,
        var_in_n_dn0_slot: &mut f64,
        var_in_n_dn1_slot: &mut f64,
        var_in_n_dn10_slot: &mut f64,
        var_in_n_dn3_slot: &mut f64,
        var_in_n_dn4_slot: &mut f64,
        var_in_n_dn5_slot: &mut f64,
        var_in_n_dn6_slot: &mut f64,
        var_in_n_dn7_slot: &mut f64,
        var_in_n_dn8_slot: &mut f64,
        var_in_n_dn9_slot: &mut f64,
        var_in_n_rv_slot: &mut f64,
        var_qb1b2_slot: &mut f64,
        var_qb1b2_dn0_slot: &mut f64,
        var_qb1b2_dn1_slot: &mut f64,
        var_qb1b2_dn10_slot: &mut f64,
        var_qb1b2_dn3_slot: &mut f64,
        var_qb1b2_dn4_slot: &mut f64,
        var_qb1b2_dn5_slot: &mut f64,
        var_qb1b2_dn6_slot: &mut f64,
        var_qb1b2_dn7_slot: &mut f64,
        var_qb1b2_dn8_slot: &mut f64,
        var_qb1b2_dn9_slot: &mut f64,
        var_qb1b2_rv_slot: &mut f64,
        var_qbc_slot: &mut f64,
        var_qbc_dn0_slot: &mut f64,
        var_qbc_dn1_slot: &mut f64,
        var_qbc_dn10_slot: &mut f64,
        var_qbc_dn3_slot: &mut f64,
        var_qbc_dn4_slot: &mut f64,
        var_qbc_dn5_slot: &mut f64,
        var_qbc_dn6_slot: &mut f64,
        var_qbc_dn7_slot: &mut f64,
        var_qbc_dn8_slot: &mut f64,
        var_qbc_dn9_slot: &mut f64,
        var_qbc_rv_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn1_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn3_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qbe_dn9_slot: &mut f64,
        var_qbe_qs_eff_slot: &mut f64,
        var_qbe_qs_eff_dn0_slot: &mut f64,
        var_qbe_qs_eff_dn1_slot: &mut f64,
        var_qbe_qs_eff_dn10_slot: &mut f64,
        var_qbe_qs_eff_dn3_slot: &mut f64,
        var_qbe_qs_eff_dn4_slot: &mut f64,
        var_qbe_qs_eff_dn5_slot: &mut f64,
        var_qbe_qs_eff_dn6_slot: &mut f64,
        var_qbe_qs_eff_dn7_slot: &mut f64,
        var_qbe_qs_eff_dn8_slot: &mut f64,
        var_qbe_qs_eff_dn9_slot: &mut f64,
        var_qbe_qs_eff_rv_slot: &mut f64,
        var_qbe_rv_slot: &mut f64,
        var_qe_slot: &mut f64,
        var_qe_dn0_slot: &mut f64,
        var_qe_dn1_slot: &mut f64,
        var_qe_dn10_slot: &mut f64,
        var_qe_dn3_slot: &mut f64,
        var_qe_dn4_slot: &mut f64,
        var_qe_dn5_slot: &mut f64,
        var_qe_dn6_slot: &mut f64,
        var_qe_dn7_slot: &mut f64,
        var_qe_dn8_slot: &mut f64,
        var_qe_dn9_slot: &mut f64,
        var_qe_rv_slot: &mut f64,
        var_taub_n_slot: &mut f64,
        var_taub_n_dn0_slot: &mut f64,
        var_taub_n_dn1_slot: &mut f64,
        var_taub_n_dn10_slot: &mut f64,
        var_taub_n_dn3_slot: &mut f64,
        var_taub_n_dn4_slot: &mut f64,
        var_taub_n_dn5_slot: &mut f64,
        var_taub_n_dn6_slot: &mut f64,
        var_taub_n_dn7_slot: &mut f64,
        var_taub_n_dn8_slot: &mut f64,
        var_taub_n_dn9_slot: &mut f64,
        var_taub_n_rv_slot: &mut f64,
        var_taun_slot: &mut f64,
        var_taun_dn0_slot: &mut f64,
        var_taun_dn1_slot: &mut f64,
        var_taun_dn10_slot: &mut f64,
        var_taun_dn3_slot: &mut f64,
        var_taun_dn4_slot: &mut f64,
        var_taun_dn5_slot: &mut f64,
        var_taun_dn6_slot: &mut f64,
        var_taun_dn7_slot: &mut f64,
        var_taun_dn8_slot: &mut f64,
        var_taun_dn9_slot: &mut f64,
        var_taun_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_dn0vb2e1: f64 = *var_dn0vb2e1_slot;
        let mut var_dn0vb2e1_dn0: f64 = *var_dn0vb2e1_dn0_slot;
        let mut var_dn0vb2e1_dn1: f64 = *var_dn0vb2e1_dn1_slot;
        let mut var_dn0vb2e1_dn10: f64 = *var_dn0vb2e1_dn10_slot;
        let mut var_dn0vb2e1_dn3: f64 = *var_dn0vb2e1_dn3_slot;
        let mut var_dn0vb2e1_dn4: f64 = *var_dn0vb2e1_dn4_slot;
        let mut var_dn0vb2e1_dn5: f64 = *var_dn0vb2e1_dn5_slot;
        let mut var_dn0vb2e1_dn6: f64 = *var_dn0vb2e1_dn6_slot;
        let mut var_dn0vb2e1_dn7: f64 = *var_dn0vb2e1_dn7_slot;
        let mut var_dn0vb2e1_dn8: f64 = *var_dn0vb2e1_dn8_slot;
        let mut var_dn0vb2e1_dn9: f64 = *var_dn0vb2e1_dn9_slot;
        let mut var_dn0vb2e1_rv: f64 = *var_dn0vb2e1_rv_slot;
        let mut var_dqbevb2e1: f64 = *var_dqbevb2e1_slot;
        let mut var_dqbevb2e1_dn0: f64 = *var_dqbevb2e1_dn0_slot;
        let mut var_dqbevb2e1_dn1: f64 = *var_dqbevb2e1_dn1_slot;
        let mut var_dqbevb2e1_dn10: f64 = *var_dqbevb2e1_dn10_slot;
        let mut var_dqbevb2e1_dn3: f64 = *var_dqbevb2e1_dn3_slot;
        let mut var_dqbevb2e1_dn4: f64 = *var_dqbevb2e1_dn4_slot;
        let mut var_dqbevb2e1_dn5: f64 = *var_dqbevb2e1_dn5_slot;
        let mut var_dqbevb2e1_dn6: f64 = *var_dqbevb2e1_dn6_slot;
        let mut var_dqbevb2e1_dn7: f64 = *var_dqbevb2e1_dn7_slot;
        let mut var_dqbevb2e1_dn8: f64 = *var_dqbevb2e1_dn8_slot;
        let mut var_dqbevb2e1_dn9: f64 = *var_dqbevb2e1_dn9_slot;
        let mut var_dqbevb2e1_rv: f64 = *var_dqbevb2e1_rv_slot;
        let mut var_dqevb2e1: f64 = *var_dqevb2e1_slot;
        let mut var_dqevb2e1_dn0: f64 = *var_dqevb2e1_dn0_slot;
        let mut var_dqevb2e1_dn1: f64 = *var_dqevb2e1_dn1_slot;
        let mut var_dqevb2e1_dn10: f64 = *var_dqevb2e1_dn10_slot;
        let mut var_dqevb2e1_dn3: f64 = *var_dqevb2e1_dn3_slot;
        let mut var_dqevb2e1_dn4: f64 = *var_dqevb2e1_dn4_slot;
        let mut var_dqevb2e1_dn5: f64 = *var_dqevb2e1_dn5_slot;
        let mut var_dqevb2e1_dn6: f64 = *var_dqevb2e1_dn6_slot;
        let mut var_dqevb2e1_dn7: f64 = *var_dqevb2e1_dn7_slot;
        let mut var_dqevb2e1_dn8: f64 = *var_dqevb2e1_dn8_slot;
        let mut var_dqevb2e1_dn9: f64 = *var_dqevb2e1_dn9_slot;
        let mut var_dqevb2e1_rv: f64 = *var_dqevb2e1_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_i_cth: f64 = *var_i_cth_slot;
        let mut var_i_cth_dn3: f64 = *var_i_cth_dn3_slot;
        let mut var_i_cth_rdn3: f64 = *var_i_cth_rdn3_slot;
        let mut var_i_cth_rv: f64 = *var_i_cth_rv_slot;
        let mut var_in_n: f64 = *var_in_n_slot;
        let mut var_in_n_dn0: f64 = *var_in_n_dn0_slot;
        let mut var_in_n_dn1: f64 = *var_in_n_dn1_slot;
        let mut var_in_n_dn10: f64 = *var_in_n_dn10_slot;
        let mut var_in_n_dn3: f64 = *var_in_n_dn3_slot;
        let mut var_in_n_dn4: f64 = *var_in_n_dn4_slot;
        let mut var_in_n_dn5: f64 = *var_in_n_dn5_slot;
        let mut var_in_n_dn6: f64 = *var_in_n_dn6_slot;
        let mut var_in_n_dn7: f64 = *var_in_n_dn7_slot;
        let mut var_in_n_dn8: f64 = *var_in_n_dn8_slot;
        let mut var_in_n_dn9: f64 = *var_in_n_dn9_slot;
        let mut var_in_n_rv: f64 = *var_in_n_rv_slot;
        let mut var_qb1b2: f64 = *var_qb1b2_slot;
        let mut var_qb1b2_dn0: f64 = *var_qb1b2_dn0_slot;
        let mut var_qb1b2_dn1: f64 = *var_qb1b2_dn1_slot;
        let mut var_qb1b2_dn10: f64 = *var_qb1b2_dn10_slot;
        let mut var_qb1b2_dn3: f64 = *var_qb1b2_dn3_slot;
        let mut var_qb1b2_dn4: f64 = *var_qb1b2_dn4_slot;
        let mut var_qb1b2_dn5: f64 = *var_qb1b2_dn5_slot;
        let mut var_qb1b2_dn6: f64 = *var_qb1b2_dn6_slot;
        let mut var_qb1b2_dn7: f64 = *var_qb1b2_dn7_slot;
        let mut var_qb1b2_dn8: f64 = *var_qb1b2_dn8_slot;
        let mut var_qb1b2_dn9: f64 = *var_qb1b2_dn9_slot;
        let mut var_qb1b2_rv: f64 = *var_qb1b2_rv_slot;
        let mut var_qbc: f64 = *var_qbc_slot;
        let mut var_qbc_dn0: f64 = *var_qbc_dn0_slot;
        let mut var_qbc_dn1: f64 = *var_qbc_dn1_slot;
        let mut var_qbc_dn10: f64 = *var_qbc_dn10_slot;
        let mut var_qbc_dn3: f64 = *var_qbc_dn3_slot;
        let mut var_qbc_dn4: f64 = *var_qbc_dn4_slot;
        let mut var_qbc_dn5: f64 = *var_qbc_dn5_slot;
        let mut var_qbc_dn6: f64 = *var_qbc_dn6_slot;
        let mut var_qbc_dn7: f64 = *var_qbc_dn7_slot;
        let mut var_qbc_dn8: f64 = *var_qbc_dn8_slot;
        let mut var_qbc_dn9: f64 = *var_qbc_dn9_slot;
        let mut var_qbc_rv: f64 = *var_qbc_rv_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn1: f64 = *var_qbe_dn1_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn3: f64 = *var_qbe_dn3_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qbe_dn9: f64 = *var_qbe_dn9_slot;
        let mut var_qbe_qs_eff: f64 = *var_qbe_qs_eff_slot;
        let mut var_qbe_qs_eff_dn0: f64 = *var_qbe_qs_eff_dn0_slot;
        let mut var_qbe_qs_eff_dn1: f64 = *var_qbe_qs_eff_dn1_slot;
        let mut var_qbe_qs_eff_dn10: f64 = *var_qbe_qs_eff_dn10_slot;
        let mut var_qbe_qs_eff_dn3: f64 = *var_qbe_qs_eff_dn3_slot;
        let mut var_qbe_qs_eff_dn4: f64 = *var_qbe_qs_eff_dn4_slot;
        let mut var_qbe_qs_eff_dn5: f64 = *var_qbe_qs_eff_dn5_slot;
        let mut var_qbe_qs_eff_dn6: f64 = *var_qbe_qs_eff_dn6_slot;
        let mut var_qbe_qs_eff_dn7: f64 = *var_qbe_qs_eff_dn7_slot;
        let mut var_qbe_qs_eff_dn8: f64 = *var_qbe_qs_eff_dn8_slot;
        let mut var_qbe_qs_eff_dn9: f64 = *var_qbe_qs_eff_dn9_slot;
        let mut var_qbe_qs_eff_rv: f64 = *var_qbe_qs_eff_rv_slot;
        let mut var_qbe_rv: f64 = *var_qbe_rv_slot;
        let mut var_qe: f64 = *var_qe_slot;
        let mut var_qe_dn0: f64 = *var_qe_dn0_slot;
        let mut var_qe_dn1: f64 = *var_qe_dn1_slot;
        let mut var_qe_dn10: f64 = *var_qe_dn10_slot;
        let mut var_qe_dn3: f64 = *var_qe_dn3_slot;
        let mut var_qe_dn4: f64 = *var_qe_dn4_slot;
        let mut var_qe_dn5: f64 = *var_qe_dn5_slot;
        let mut var_qe_dn6: f64 = *var_qe_dn6_slot;
        let mut var_qe_dn7: f64 = *var_qe_dn7_slot;
        let mut var_qe_dn8: f64 = *var_qe_dn8_slot;
        let mut var_qe_dn9: f64 = *var_qe_dn9_slot;
        let mut var_qe_rv: f64 = *var_qe_rv_slot;
        let mut var_taub_n: f64 = *var_taub_n_slot;
        let mut var_taub_n_dn0: f64 = *var_taub_n_dn0_slot;
        let mut var_taub_n_dn1: f64 = *var_taub_n_dn1_slot;
        let mut var_taub_n_dn10: f64 = *var_taub_n_dn10_slot;
        let mut var_taub_n_dn3: f64 = *var_taub_n_dn3_slot;
        let mut var_taub_n_dn4: f64 = *var_taub_n_dn4_slot;
        let mut var_taub_n_dn5: f64 = *var_taub_n_dn5_slot;
        let mut var_taub_n_dn6: f64 = *var_taub_n_dn6_slot;
        let mut var_taub_n_dn7: f64 = *var_taub_n_dn7_slot;
        let mut var_taub_n_dn8: f64 = *var_taub_n_dn8_slot;
        let mut var_taub_n_dn9: f64 = *var_taub_n_dn9_slot;
        let mut var_taub_n_rv: f64 = *var_taub_n_rv_slot;
        let mut var_taun: f64 = *var_taun_slot;
        let mut var_taun_dn0: f64 = *var_taun_dn0_slot;
        let mut var_taun_dn1: f64 = *var_taun_dn1_slot;
        let mut var_taun_dn10: f64 = *var_taun_dn10_slot;
        let mut var_taun_dn3: f64 = *var_taun_dn3_slot;
        let mut var_taun_dn4: f64 = *var_taun_dn4_slot;
        let mut var_taun_dn5: f64 = *var_taun_dn5_slot;
        let mut var_taun_dn6: f64 = *var_taun_dn6_slot;
        let mut var_taun_dn7: f64 = *var_taun_dn7_slot;
        let mut var_taun_dn8: f64 = *var_taun_dn8_slot;
        let mut var_taun_dn9: f64 = *var_taun_dn9_slot;
        let mut var_taun_rv: f64 = *var_taun_rv_slot;

        let (assign6330_e6502, assign6330_e6502_d_n0, assign6330_e6502_d_n1, assign6330_e6502_d_n3, assign6330_e6502_d_n4, assign6330_e6502_d_n5, assign6330_e6502_d_n6, assign6330_e6502_d_n7, assign6330_e6502_d_n8, assign6330_e6502_d_n9, assign6330_e6502_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6330_e6489: f64 = (var_if0 * var_evb2e1);
        let assign6330_e6491: f64 = (assign6330_e6489 * var_vtinv);
        let assign6330_e6493: f64 = (assign6330_e6491 / var_nff_t);
        let assign6330_e6497: f64 = (1.0 + var_f1);
        let assign6330_e6498: f64 = (assign6330_e6497).sqrt();
        let assign6330_e6499: f64 = (0.5 / assign6330_e6498);
        let assign6330_e6500: f64 = (assign6330_e6493 * assign6330_e6499);
        (assign6330_e6500, ((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn3)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), ((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10,)
    }
};
        var_dn0vb2e1 = assign6330_e6502;
        var_dn0vb2e1_dn0 = assign6330_e6502_d_n0;
        var_dn0vb2e1_dn1 = assign6330_e6502_d_n1;
        var_dn0vb2e1_dn3 = assign6330_e6502_d_n3;
        var_dn0vb2e1_dn4 = assign6330_e6502_d_n4;
        var_dn0vb2e1_dn5 = assign6330_e6502_d_n5;
        var_dn0vb2e1_dn6 = assign6330_e6502_d_n6;
        var_dn0vb2e1_dn7 = assign6330_e6502_d_n7;
        var_dn0vb2e1_dn8 = assign6330_e6502_d_n8;
        var_dn0vb2e1_dn9 = assign6330_e6502_d_n9;
        var_dn0vb2e1_dn10 = assign6330_e6502_d_n10;
        var_dn0vb2e1_rv = 0.0;

        let (assign6340_e6512, assign6340_e6512_d_n0, assign6340_e6512_d_n1, assign6340_e6512_d_n3, assign6340_e6512_d_n4, assign6340_e6512_d_n5, assign6340_e6512_d_n6, assign6340_e6512_d_n7, assign6340_e6512_d_n8, assign6340_e6512_d_n9, assign6340_e6512_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6340_e6506: f64 = (0.5 * var_qb0);
        let assign6340_e6508: f64 = (assign6340_e6506 * var_q1q);
        let assign6340_e6510: f64 = (assign6340_e6508 * var_dn0vb2e1);
        (assign6340_e6510, (((assign6340_e6506 * var_q1q_dn0) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn0)), (((assign6340_e6506 * var_q1q_dn1) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn1)), (((((0.5 * var_qb0_dn3) * var_q1q) + (assign6340_e6506 * var_q1q_dn3)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn3)), (((assign6340_e6506 * var_q1q_dn4) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn4)), (((assign6340_e6506 * var_q1q_dn5) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn5)), (((assign6340_e6506 * var_q1q_dn6) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn6)), (((assign6340_e6506 * var_q1q_dn7) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn7)), (((assign6340_e6506 * var_q1q_dn8) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn8)), (((assign6340_e6506 * var_q1q_dn9) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn9)), (((assign6340_e6506 * var_q1q_dn10) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn10)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10,)
    }
};
        var_dqbevb2e1 = assign6340_e6512;
        var_dqbevb2e1_dn0 = assign6340_e6512_d_n0;
        var_dqbevb2e1_dn1 = assign6340_e6512_d_n1;
        var_dqbevb2e1_dn3 = assign6340_e6512_d_n3;
        var_dqbevb2e1_dn4 = assign6340_e6512_d_n4;
        var_dqbevb2e1_dn5 = assign6340_e6512_d_n5;
        var_dqbevb2e1_dn6 = assign6340_e6512_d_n6;
        var_dqbevb2e1_dn7 = assign6340_e6512_d_n7;
        var_dqbevb2e1_dn8 = assign6340_e6512_d_n8;
        var_dqbevb2e1_dn9 = assign6340_e6512_d_n9;
        var_dqbevb2e1_dn10 = assign6340_e6512_d_n10;
        var_dqbevb2e1_rv = 0.0;

        let (assign6350_e6520, assign6350_e6520_d_n0, assign6350_e6520_d_n1, assign6350_e6520_d_n3, assign6350_e6520_d_n4, assign6350_e6520_d_n5, assign6350_e6520_d_n6, assign6350_e6520_d_n7, assign6350_e6520_d_n8, assign6350_e6520_d_n9, assign6350_e6520_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6350_e6517: f64 = (p.p84 * var_vt);
        let assign6350_e6518: f64 = (var_qe_qs / assign6350_e6517);
        (assign6350_e6518, (var_qe_qs_dn0 / assign6350_e6517), (var_qe_qs_dn1 / assign6350_e6517), (((var_qe_qs_dn3 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn3))) / (assign6350_e6517 * assign6350_e6517)), (var_qe_qs_dn4 / assign6350_e6517), (var_qe_qs_dn5 / assign6350_e6517), (var_qe_qs_dn6 / assign6350_e6517), (var_qe_qs_dn7 / assign6350_e6517), (var_qe_qs_dn8 / assign6350_e6517), (var_qe_qs_dn9 / assign6350_e6517), (var_qe_qs_dn10 / assign6350_e6517),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10,)
    }
};
        var_dqevb2e1 = assign6350_e6520;
        var_dqevb2e1_dn0 = assign6350_e6520_d_n0;
        var_dqevb2e1_dn1 = assign6350_e6520_d_n1;
        var_dqevb2e1_dn3 = assign6350_e6520_d_n3;
        var_dqevb2e1_dn4 = assign6350_e6520_d_n4;
        var_dqevb2e1_dn5 = assign6350_e6520_d_n5;
        var_dqevb2e1_dn6 = assign6350_e6520_d_n6;
        var_dqevb2e1_dn7 = assign6350_e6520_d_n7;
        var_dqevb2e1_dn8 = assign6350_e6520_d_n8;
        var_dqevb2e1_dn9 = assign6350_e6520_d_n9;
        var_dqevb2e1_dn10 = assign6350_e6520_d_n10;
        var_dqevb2e1_rv = 0.0;

        let (assign6360_e6532, assign6360_e6532_d_n0, assign6360_e6532_d_n1, assign6360_e6532_d_n3, assign6360_e6532_d_n4, assign6360_e6532_d_n5, assign6360_e6532_d_n6, assign6360_e6532_d_n7, assign6360_e6532_d_n8, assign6360_e6532_d_n9, assign6360_e6532_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6360_e6524: f64 = (0.2 * var_vb1b2);
        let assign6360_e6527: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6360_e6529: f64 = (assign6360_e6527 + var_dqevb2e1);
        let assign6360_e6530: f64 = (assign6360_e6524 * assign6360_e6529);
        (assign6360_e6530, (assign6360_e6524 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0)), (assign6360_e6524 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1)), (assign6360_e6524 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3)), (assign6360_e6524 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4)), (((0.2 * var_vb1b2_dn5) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (assign6360_e6524 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7)), (assign6360_e6524 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8)), (assign6360_e6524 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9)), (assign6360_e6524 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10)),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10,)
    }
};
        var_qb1b2 = assign6360_e6532;
        var_qb1b2_dn0 = assign6360_e6532_d_n0;
        var_qb1b2_dn1 = assign6360_e6532_d_n1;
        var_qb1b2_dn3 = assign6360_e6532_d_n3;
        var_qb1b2_dn4 = assign6360_e6532_d_n4;
        var_qb1b2_dn5 = assign6360_e6532_d_n5;
        var_qb1b2_dn6 = assign6360_e6532_d_n6;
        var_qb1b2_dn7 = assign6360_e6532_d_n7;
        var_qb1b2_dn8 = assign6360_e6532_d_n8;
        var_qb1b2_dn9 = assign6360_e6532_d_n9;
        var_qb1b2_dn10 = assign6360_e6532_d_n10;
        var_qb1b2_rv = 0.0;

        let (assign6370_e6540, assign6370_e6540_d_n0, assign6370_e6540_d_n1, assign6370_e6540_d_n3, assign6370_e6540_d_n4, assign6370_e6540_d_n5, assign6370_e6540_d_n6, assign6370_e6540_d_n7, assign6370_e6540_d_n8, assign6370_e6540_d_n9, assign6370_e6540_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6370_e6536: f64 = (1.0 - p.p94);
        let assign6370_e6538: f64 = (assign6370_e6536 * var_qe_qs);
        (assign6370_e6538, (assign6370_e6536 * var_qe_qs_dn0), (assign6370_e6536 * var_qe_qs_dn1), (assign6370_e6536 * var_qe_qs_dn3), (assign6370_e6536 * var_qe_qs_dn4), (assign6370_e6536 * var_qe_qs_dn5), (assign6370_e6536 * var_qe_qs_dn6), (assign6370_e6536 * var_qe_qs_dn7), (assign6370_e6536 * var_qe_qs_dn8), (assign6370_e6536 * var_qe_qs_dn9), (assign6370_e6536 * var_qe_qs_dn10),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10,)
    }
};
        var_qe = assign6370_e6540;
        var_qe_dn0 = assign6370_e6540_d_n0;
        var_qe_dn1 = assign6370_e6540_d_n1;
        var_qe_dn3 = assign6370_e6540_d_n3;
        var_qe_dn4 = assign6370_e6540_d_n4;
        var_qe_dn5 = assign6370_e6540_d_n5;
        var_qe_dn6 = assign6370_e6540_d_n6;
        var_qe_dn7 = assign6370_e6540_d_n7;
        var_qe_dn8 = assign6370_e6540_d_n8;
        var_qe_dn9 = assign6370_e6540_d_n9;
        var_qe_dn10 = assign6370_e6540_d_n10;
        var_qe_rv = 0.0;

        let (assign6380_e6548, assign6380_e6548_d_n0, assign6380_e6548_d_n1, assign6380_e6548_d_n3, assign6380_e6548_d_n4, assign6380_e6548_d_n5, assign6380_e6548_d_n6, assign6380_e6548_d_n7, assign6380_e6548_d_n8, assign6380_e6548_d_n9, assign6380_e6548_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6380_e6545: f64 = (p.p94 * var_qe_qs);
        let assign6380_e6546: f64 = (var_qbe_qs + assign6380_e6545);
        (assign6380_e6546, (var_qbe_qs_dn0 + (p.p94 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p94 * var_qe_qs_dn1)), (var_qbe_qs_dn3 + (p.p94 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p94 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p94 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p94 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p94 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p94 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p94 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p94 * var_qe_qs_dn10)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10,)
    }
};
        var_qbe_qs_eff = assign6380_e6548;
        var_qbe_qs_eff_dn0 = assign6380_e6548_d_n0;
        var_qbe_qs_eff_dn1 = assign6380_e6548_d_n1;
        var_qbe_qs_eff_dn3 = assign6380_e6548_d_n3;
        var_qbe_qs_eff_dn4 = assign6380_e6548_d_n4;
        var_qbe_qs_eff_dn5 = assign6380_e6548_d_n5;
        var_qbe_qs_eff_dn6 = assign6380_e6548_d_n6;
        var_qbe_qs_eff_dn7 = assign6380_e6548_d_n7;
        var_qbe_qs_eff_dn8 = assign6380_e6548_d_n8;
        var_qbe_qs_eff_dn9 = assign6380_e6548_d_n9;
        var_qbe_qs_eff_dn10 = assign6380_e6548_d_n10;
        var_qbe_qs_eff_rv = 0.0;

        let (assign6390_e6556, assign6390_e6556_d_n0, assign6390_e6556_d_n1, assign6390_e6556_d_n3, assign6390_e6556_d_n4, assign6390_e6556_d_n5, assign6390_e6556_d_n6, assign6390_e6556_d_n7, assign6390_e6556_d_n8, assign6390_e6556_d_n9, assign6390_e6556_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6390_e6552: f64 = (p.p93 * var_qbe_qs_eff);
        let assign6390_e6554: f64 = (assign6390_e6552 + var_qbc_qs);
        (assign6390_e6554, ((p.p93 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p93 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p93 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p93 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p93 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p93 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p93 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p93 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p93 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p93 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10,)
    }
};
        var_qbc = assign6390_e6556;
        var_qbc_dn0 = assign6390_e6556_d_n0;
        var_qbc_dn1 = assign6390_e6556_d_n1;
        var_qbc_dn3 = assign6390_e6556_d_n3;
        var_qbc_dn4 = assign6390_e6556_d_n4;
        var_qbc_dn5 = assign6390_e6556_d_n5;
        var_qbc_dn6 = assign6390_e6556_d_n6;
        var_qbc_dn7 = assign6390_e6556_d_n7;
        var_qbc_dn8 = assign6390_e6556_d_n8;
        var_qbc_dn9 = assign6390_e6556_d_n9;
        var_qbc_dn10 = assign6390_e6556_d_n10;
        var_qbc_rv = 0.0;

        let (assign6400_e6564, assign6400_e6564_d_n0, assign6400_e6564_d_n1, assign6400_e6564_d_n3, assign6400_e6564_d_n4, assign6400_e6564_d_n5, assign6400_e6564_d_n6, assign6400_e6564_d_n7, assign6400_e6564_d_n8, assign6400_e6564_d_n9, assign6400_e6564_d_n10,) = {
    if (var_guard115 != 0.0) {
        let assign6400_e6560: f64 = (1.0 - p.p93);
        let assign6400_e6562: f64 = (assign6400_e6560 * var_qbe_qs_eff);
        (assign6400_e6562, (assign6400_e6560 * var_qbe_qs_eff_dn0), (assign6400_e6560 * var_qbe_qs_eff_dn1), (assign6400_e6560 * var_qbe_qs_eff_dn3), (assign6400_e6560 * var_qbe_qs_eff_dn4), (assign6400_e6560 * var_qbe_qs_eff_dn5), (assign6400_e6560 * var_qbe_qs_eff_dn6), (assign6400_e6560 * var_qbe_qs_eff_dn7), (assign6400_e6560 * var_qbe_qs_eff_dn8), (assign6400_e6560 * var_qbe_qs_eff_dn9), (assign6400_e6560 * var_qbe_qs_eff_dn10),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10,)
    }
};
        var_qbe = assign6400_e6564;
        var_qbe_dn0 = assign6400_e6564_d_n0;
        var_qbe_dn1 = assign6400_e6564_d_n1;
        var_qbe_dn3 = assign6400_e6564_d_n3;
        var_qbe_dn4 = assign6400_e6564_d_n4;
        var_qbe_dn5 = assign6400_e6564_d_n5;
        var_qbe_dn6 = assign6400_e6564_d_n6;
        var_qbe_dn7 = assign6400_e6564_d_n7;
        var_qbe_dn8 = assign6400_e6564_d_n8;
        var_qbe_dn9 = assign6400_e6564_d_n9;
        var_qbe_dn10 = assign6400_e6564_d_n10;
        var_qbe_rv = 0.0;

        let (assign6410_e6569, assign6410_e6569_d_n0, assign6410_e6569_d_n1, assign6410_e6569_d_n3, assign6410_e6569_d_n4, assign6410_e6569_d_n5, assign6410_e6569_d_n6, assign6410_e6569_d_n7, assign6410_e6569_d_n8, assign6410_e6569_d_n9, assign6410_e6569_d_n10,) = {
    if (var_guard115 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10,)
    }
};
        var_qbe = assign6410_e6569;
        var_qbe_dn0 = assign6410_e6569_d_n0;
        var_qbe_dn1 = assign6410_e6569_d_n1;
        var_qbe_dn3 = assign6410_e6569_d_n3;
        var_qbe_dn4 = assign6410_e6569_d_n4;
        var_qbe_dn5 = assign6410_e6569_d_n5;
        var_qbe_dn6 = assign6410_e6569_d_n6;
        var_qbe_dn7 = assign6410_e6569_d_n7;
        var_qbe_dn8 = assign6410_e6569_d_n8;
        var_qbe_dn9 = assign6410_e6569_d_n9;
        var_qbe_dn10 = assign6410_e6569_d_n10;
        var_qbe_rv = 0.0;

        let (assign6420_e6574, assign6420_e6574_d_n0, assign6420_e6574_d_n1, assign6420_e6574_d_n3, assign6420_e6574_d_n4, assign6420_e6574_d_n5, assign6420_e6574_d_n6, assign6420_e6574_d_n7, assign6420_e6574_d_n8, assign6420_e6574_d_n9, assign6420_e6574_d_n10,) = {
    if (var_guard115 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10,)
    }
};
        var_qbc = assign6420_e6574;
        var_qbc_dn0 = assign6420_e6574_d_n0;
        var_qbc_dn1 = assign6420_e6574_d_n1;
        var_qbc_dn3 = assign6420_e6574_d_n3;
        var_qbc_dn4 = assign6420_e6574_d_n4;
        var_qbc_dn5 = assign6420_e6574_d_n5;
        var_qbc_dn6 = assign6420_e6574_d_n6;
        var_qbc_dn7 = assign6420_e6574_d_n7;
        var_qbc_dn8 = assign6420_e6574_d_n8;
        var_qbc_dn9 = assign6420_e6574_d_n9;
        var_qbc_dn10 = assign6420_e6574_d_n10;
        var_qbc_rv = 0.0;

        let (assign6430_e6579, assign6430_e6579_d_n0, assign6430_e6579_d_n1, assign6430_e6579_d_n3, assign6430_e6579_d_n4, assign6430_e6579_d_n5, assign6430_e6579_d_n6, assign6430_e6579_d_n7, assign6430_e6579_d_n8, assign6430_e6579_d_n9, assign6430_e6579_d_n10,) = {
    if (var_guard115 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10,)
    }
};
        var_qe = assign6430_e6579;
        var_qe_dn0 = assign6430_e6579_d_n0;
        var_qe_dn1 = assign6430_e6579_d_n1;
        var_qe_dn3 = assign6430_e6579_d_n3;
        var_qe_dn4 = assign6430_e6579_d_n4;
        var_qe_dn5 = assign6430_e6579_d_n5;
        var_qe_dn6 = assign6430_e6579_d_n6;
        var_qe_dn7 = assign6430_e6579_d_n7;
        var_qe_dn8 = assign6430_e6579_d_n8;
        var_qe_dn9 = assign6430_e6579_d_n9;
        var_qe_dn10 = assign6430_e6579_d_n10;
        var_qe_rv = 0.0;

        let assign6450_e6585: f64 = (p.p134 * (nv3 - 0.0));
        let assign6450_e6586_q: f64 = assign6450_e6585;
        let assign6450_e6588: f64 = (assign6450_e6585 * p.p1);
        let assign6450_e6588_q: f64 = (assign6450_e6586_q * p.p1);
        var_i_cth = assign6450_e6588;
        var_i_cth_dn3 = (p.p134 * p.p1);
        var_i_cth_rv = assign6450_e6588_q;
        var_i_cth_rdn3 = (p.p134 * p.p1);

        let assign6630_e6704: f64 = (var_if_ + var_ir);
        let assign6630_e6706: f64 = (assign6630_e6704 / var_qbi);
        var_in_n = assign6630_e6706;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign6630_e6704 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign6630_e6704 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign6630_e6704 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign6630_e6704 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign6630_e6704 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign6630_e6704 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign6630_e6704 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign6630_e6704 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign6630_e6704 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign6630_e6704 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_rv = 0.0;

        let assign6690_e6739: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard124 = assign6690_e6739;
        var_guard124_rv = 0.0;

        let (assign6700_e6747, assign6700_e6747_d_n0, assign6700_e6747_d_n1, assign6700_e6747_d_n3, assign6700_e6747_d_n4, assign6700_e6747_d_n5, assign6700_e6747_d_n6, assign6700_e6747_d_n7, assign6700_e6747_d_n8, assign6700_e6747_d_n9, assign6700_e6747_d_n10,) = {
    if (var_guard124 != 0.0) {
        let assign6700_e6743: f64 = (var_qbe + var_qbc);
        let assign6700_e6745: f64 = (assign6700_e6743 / var_in_n);
        (assign6700_e6745, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign6700_e6743 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign6700_e6743 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign6700_e6743 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign6700_e6743 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign6700_e6743 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign6700_e6743 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign6700_e6743 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign6700_e6743 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign6700_e6743 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign6700_e6743 * var_in_n_dn10)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10,)
    }
};
        var_taub_n = assign6700_e6747;
        var_taub_n_dn0 = assign6700_e6747_d_n0;
        var_taub_n_dn1 = assign6700_e6747_d_n1;
        var_taub_n_dn3 = assign6700_e6747_d_n3;
        var_taub_n_dn4 = assign6700_e6747_d_n4;
        var_taub_n_dn5 = assign6700_e6747_d_n5;
        var_taub_n_dn6 = assign6700_e6747_d_n6;
        var_taub_n_dn7 = assign6700_e6747_d_n7;
        var_taub_n_dn8 = assign6700_e6747_d_n8;
        var_taub_n_dn9 = assign6700_e6747_d_n9;
        var_taub_n_dn10 = assign6700_e6747_d_n10;
        var_taub_n_rv = 0.0;

        let (assign6710_e6756, assign6710_e6756_d_n0, assign6710_e6756_d_n1, assign6710_e6756_d_n3, assign6710_e6756_d_n4, assign6710_e6756_d_n5, assign6710_e6756_d_n6, assign6710_e6756_d_n7, assign6710_e6756_d_n8, assign6710_e6756_d_n9, assign6710_e6756_d_n10,) = {
    if (var_guard124 == 0.0) {
        let assign6710_e6752: f64 = (var_taub_t * var_q1q);
        let assign6710_e6754: f64 = (assign6710_e6752 * var_qbi);
        (assign6710_e6754, (((var_taub_t * var_q1q_dn0) * var_qbi) + (assign6710_e6752 * var_qbi_dn0)), (((var_taub_t * var_q1q_dn1) * var_qbi) + (assign6710_e6752 * var_qbi_dn1)), ((((var_taub_t_dn3 * var_q1q) + (var_taub_t * var_q1q_dn3)) * var_qbi) + (assign6710_e6752 * var_qbi_dn3)), (((var_taub_t * var_q1q_dn4) * var_qbi) + (assign6710_e6752 * var_qbi_dn4)), (((var_taub_t * var_q1q_dn5) * var_qbi) + (assign6710_e6752 * var_qbi_dn5)), (((var_taub_t * var_q1q_dn6) * var_qbi) + (assign6710_e6752 * var_qbi_dn6)), (((var_taub_t * var_q1q_dn7) * var_qbi) + (assign6710_e6752 * var_qbi_dn7)), (((var_taub_t * var_q1q_dn8) * var_qbi) + (assign6710_e6752 * var_qbi_dn8)), (((var_taub_t * var_q1q_dn9) * var_qbi) + (assign6710_e6752 * var_qbi_dn9)), (((var_taub_t * var_q1q_dn10) * var_qbi) + (assign6710_e6752 * var_qbi_dn10)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10,)
    }
};
        var_taub_n = assign6710_e6756;
        var_taub_n_dn0 = assign6710_e6756_d_n0;
        var_taub_n_dn1 = assign6710_e6756_d_n1;
        var_taub_n_dn3 = assign6710_e6756_d_n3;
        var_taub_n_dn4 = assign6710_e6756_d_n4;
        var_taub_n_dn5 = assign6710_e6756_d_n5;
        var_taub_n_dn6 = assign6710_e6756_d_n6;
        var_taub_n_dn7 = assign6710_e6756_d_n7;
        var_taub_n_dn8 = assign6710_e6756_d_n8;
        var_taub_n_dn9 = assign6710_e6756_d_n9;
        var_taub_n_dn10 = assign6710_e6756_d_n10;
        var_taub_n_rv = 0.0;

        let assign6720_e6759: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign6720_e6759;
        var_guard125_rv = 0.0;

        let (assign6730_e6765, assign6730_e6765_d_n0, assign6730_e6765_d_n1, assign6730_e6765_d_n3, assign6730_e6765_d_n4, assign6730_e6765_d_n5, assign6730_e6765_d_n6, assign6730_e6765_d_n7, assign6730_e6765_d_n8, assign6730_e6765_d_n9, assign6730_e6765_d_n10,) = {
    if (var_guard125 != 0.0) {
        let assign6730_e6763: f64 = (p.p93 * var_taub_n);
        (assign6730_e6763, (p.p93 * var_taub_n_dn0), (p.p93 * var_taub_n_dn1), (p.p93 * var_taub_n_dn3), (p.p93 * var_taub_n_dn4), (p.p93 * var_taub_n_dn5), (p.p93 * var_taub_n_dn6), (p.p93 * var_taub_n_dn7), (p.p93 * var_taub_n_dn8), (p.p93 * var_taub_n_dn9), (p.p93 * var_taub_n_dn10),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign6730_e6765;
        var_taun_dn0 = assign6730_e6765_d_n0;
        var_taun_dn1 = assign6730_e6765_d_n1;
        var_taun_dn3 = assign6730_e6765_d_n3;
        var_taun_dn4 = assign6730_e6765_d_n4;
        var_taun_dn5 = assign6730_e6765_d_n5;
        var_taun_dn6 = assign6730_e6765_d_n6;
        var_taun_dn7 = assign6730_e6765_d_n7;
        var_taun_dn8 = assign6730_e6765_d_n8;
        var_taun_dn9 = assign6730_e6765_d_n9;
        var_taun_dn10 = assign6730_e6765_d_n10;
        var_taun_rv = 0.0;

        let assign6740_e6768: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        var_guard126 = assign6740_e6768;
        var_guard126_rv = 0.0;

        let (assign6750_e6777, assign6750_e6777_d_n0, assign6750_e6777_d_n1, assign6750_e6777_d_n3, assign6750_e6777_d_n4, assign6750_e6777_d_n5, assign6750_e6777_d_n6, assign6750_e6777_d_n7, assign6750_e6777_d_n8, assign6750_e6777_d_n9, assign6750_e6777_d_n10,) = {
    if ((var_guard125 == 0.0) && (var_guard126 != 0.0)) {
        let assign6750_e6775: f64 = (p.p131 * var_taub_n);
        (assign6750_e6775, (p.p131 * var_taub_n_dn0), (p.p131 * var_taub_n_dn1), (p.p131 * var_taub_n_dn3), (p.p131 * var_taub_n_dn4), (p.p131 * var_taub_n_dn5), (p.p131 * var_taub_n_dn6), (p.p131 * var_taub_n_dn7), (p.p131 * var_taub_n_dn8), (p.p131 * var_taub_n_dn9), (p.p131 * var_taub_n_dn10),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign6750_e6777;
        var_taun_dn0 = assign6750_e6777_d_n0;
        var_taun_dn1 = assign6750_e6777_d_n1;
        var_taun_dn3 = assign6750_e6777_d_n3;
        var_taun_dn4 = assign6750_e6777_d_n4;
        var_taun_dn5 = assign6750_e6777_d_n5;
        var_taun_dn6 = assign6750_e6777_d_n6;
        var_taun_dn7 = assign6750_e6777_d_n7;
        var_taun_dn8 = assign6750_e6777_d_n8;
        var_taun_dn9 = assign6750_e6777_d_n9;
        var_taun_dn10 = assign6750_e6777_d_n10;
        var_taun_rv = 0.0;

        let (assign6760_e6785, assign6760_e6785_d_n0, assign6760_e6785_d_n1, assign6760_e6785_d_n3, assign6760_e6785_d_n4, assign6760_e6785_d_n5, assign6760_e6785_d_n6, assign6760_e6785_d_n7, assign6760_e6785_d_n8, assign6760_e6785_d_n9, assign6760_e6785_d_n10,) = {
    if ((var_guard125 == 0.0) && (var_guard126 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign6760_e6785;
        var_taun_dn0 = assign6760_e6785_d_n0;
        var_taun_dn1 = assign6760_e6785_d_n1;
        var_taun_dn3 = assign6760_e6785_d_n3;
        var_taun_dn4 = assign6760_e6785_d_n4;
        var_taun_dn5 = assign6760_e6785_d_n5;
        var_taun_dn6 = assign6760_e6785_d_n6;
        var_taun_dn7 = assign6760_e6785_d_n7;
        var_taun_dn8 = assign6760_e6785_d_n8;
        var_taun_dn9 = assign6760_e6785_d_n9;
        var_taun_dn10 = assign6760_e6785_d_n10;
        var_taun_rv = 0.0;

        *var_dn0vb2e1_slot = var_dn0vb2e1;
        *var_dn0vb2e1_dn0_slot = var_dn0vb2e1_dn0;
        *var_dn0vb2e1_dn1_slot = var_dn0vb2e1_dn1;
        *var_dn0vb2e1_dn10_slot = var_dn0vb2e1_dn10;
        *var_dn0vb2e1_dn3_slot = var_dn0vb2e1_dn3;
        *var_dn0vb2e1_dn4_slot = var_dn0vb2e1_dn4;
        *var_dn0vb2e1_dn5_slot = var_dn0vb2e1_dn5;
        *var_dn0vb2e1_dn6_slot = var_dn0vb2e1_dn6;
        *var_dn0vb2e1_dn7_slot = var_dn0vb2e1_dn7;
        *var_dn0vb2e1_dn8_slot = var_dn0vb2e1_dn8;
        *var_dn0vb2e1_dn9_slot = var_dn0vb2e1_dn9;
        *var_dn0vb2e1_rv_slot = var_dn0vb2e1_rv;
        *var_dqbevb2e1_slot = var_dqbevb2e1;
        *var_dqbevb2e1_dn0_slot = var_dqbevb2e1_dn0;
        *var_dqbevb2e1_dn1_slot = var_dqbevb2e1_dn1;
        *var_dqbevb2e1_dn10_slot = var_dqbevb2e1_dn10;
        *var_dqbevb2e1_dn3_slot = var_dqbevb2e1_dn3;
        *var_dqbevb2e1_dn4_slot = var_dqbevb2e1_dn4;
        *var_dqbevb2e1_dn5_slot = var_dqbevb2e1_dn5;
        *var_dqbevb2e1_dn6_slot = var_dqbevb2e1_dn6;
        *var_dqbevb2e1_dn7_slot = var_dqbevb2e1_dn7;
        *var_dqbevb2e1_dn8_slot = var_dqbevb2e1_dn8;
        *var_dqbevb2e1_dn9_slot = var_dqbevb2e1_dn9;
        *var_dqbevb2e1_rv_slot = var_dqbevb2e1_rv;
        *var_dqevb2e1_slot = var_dqevb2e1;
        *var_dqevb2e1_dn0_slot = var_dqevb2e1_dn0;
        *var_dqevb2e1_dn1_slot = var_dqevb2e1_dn1;
        *var_dqevb2e1_dn10_slot = var_dqevb2e1_dn10;
        *var_dqevb2e1_dn3_slot = var_dqevb2e1_dn3;
        *var_dqevb2e1_dn4_slot = var_dqevb2e1_dn4;
        *var_dqevb2e1_dn5_slot = var_dqevb2e1_dn5;
        *var_dqevb2e1_dn6_slot = var_dqevb2e1_dn6;
        *var_dqevb2e1_dn7_slot = var_dqevb2e1_dn7;
        *var_dqevb2e1_dn8_slot = var_dqevb2e1_dn8;
        *var_dqevb2e1_dn9_slot = var_dqevb2e1_dn9;
        *var_dqevb2e1_rv_slot = var_dqevb2e1_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_i_cth_slot = var_i_cth;
        *var_i_cth_dn3_slot = var_i_cth_dn3;
        *var_i_cth_rdn3_slot = var_i_cth_rdn3;
        *var_i_cth_rv_slot = var_i_cth_rv;
        *var_in_n_slot = var_in_n;
        *var_in_n_dn0_slot = var_in_n_dn0;
        *var_in_n_dn1_slot = var_in_n_dn1;
        *var_in_n_dn10_slot = var_in_n_dn10;
        *var_in_n_dn3_slot = var_in_n_dn3;
        *var_in_n_dn4_slot = var_in_n_dn4;
        *var_in_n_dn5_slot = var_in_n_dn5;
        *var_in_n_dn6_slot = var_in_n_dn6;
        *var_in_n_dn7_slot = var_in_n_dn7;
        *var_in_n_dn8_slot = var_in_n_dn8;
        *var_in_n_dn9_slot = var_in_n_dn9;
        *var_in_n_rv_slot = var_in_n_rv;
        *var_qb1b2_slot = var_qb1b2;
        *var_qb1b2_dn0_slot = var_qb1b2_dn0;
        *var_qb1b2_dn1_slot = var_qb1b2_dn1;
        *var_qb1b2_dn10_slot = var_qb1b2_dn10;
        *var_qb1b2_dn3_slot = var_qb1b2_dn3;
        *var_qb1b2_dn4_slot = var_qb1b2_dn4;
        *var_qb1b2_dn5_slot = var_qb1b2_dn5;
        *var_qb1b2_dn6_slot = var_qb1b2_dn6;
        *var_qb1b2_dn7_slot = var_qb1b2_dn7;
        *var_qb1b2_dn8_slot = var_qb1b2_dn8;
        *var_qb1b2_dn9_slot = var_qb1b2_dn9;
        *var_qb1b2_rv_slot = var_qb1b2_rv;
        *var_qbc_slot = var_qbc;
        *var_qbc_dn0_slot = var_qbc_dn0;
        *var_qbc_dn1_slot = var_qbc_dn1;
        *var_qbc_dn10_slot = var_qbc_dn10;
        *var_qbc_dn3_slot = var_qbc_dn3;
        *var_qbc_dn4_slot = var_qbc_dn4;
        *var_qbc_dn5_slot = var_qbc_dn5;
        *var_qbc_dn6_slot = var_qbc_dn6;
        *var_qbc_dn7_slot = var_qbc_dn7;
        *var_qbc_dn8_slot = var_qbc_dn8;
        *var_qbc_dn9_slot = var_qbc_dn9;
        *var_qbc_rv_slot = var_qbc_rv;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn1_slot = var_qbe_dn1;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn3_slot = var_qbe_dn3;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qbe_dn9_slot = var_qbe_dn9;
        *var_qbe_qs_eff_slot = var_qbe_qs_eff;
        *var_qbe_qs_eff_dn0_slot = var_qbe_qs_eff_dn0;
        *var_qbe_qs_eff_dn1_slot = var_qbe_qs_eff_dn1;
        *var_qbe_qs_eff_dn10_slot = var_qbe_qs_eff_dn10;
        *var_qbe_qs_eff_dn3_slot = var_qbe_qs_eff_dn3;
        *var_qbe_qs_eff_dn4_slot = var_qbe_qs_eff_dn4;
        *var_qbe_qs_eff_dn5_slot = var_qbe_qs_eff_dn5;
        *var_qbe_qs_eff_dn6_slot = var_qbe_qs_eff_dn6;
        *var_qbe_qs_eff_dn7_slot = var_qbe_qs_eff_dn7;
        *var_qbe_qs_eff_dn8_slot = var_qbe_qs_eff_dn8;
        *var_qbe_qs_eff_dn9_slot = var_qbe_qs_eff_dn9;
        *var_qbe_qs_eff_rv_slot = var_qbe_qs_eff_rv;
        *var_qbe_rv_slot = var_qbe_rv;
        *var_qe_slot = var_qe;
        *var_qe_dn0_slot = var_qe_dn0;
        *var_qe_dn1_slot = var_qe_dn1;
        *var_qe_dn10_slot = var_qe_dn10;
        *var_qe_dn3_slot = var_qe_dn3;
        *var_qe_dn4_slot = var_qe_dn4;
        *var_qe_dn5_slot = var_qe_dn5;
        *var_qe_dn6_slot = var_qe_dn6;
        *var_qe_dn7_slot = var_qe_dn7;
        *var_qe_dn8_slot = var_qe_dn8;
        *var_qe_dn9_slot = var_qe_dn9;
        *var_qe_rv_slot = var_qe_rv;
        *var_taub_n_slot = var_taub_n;
        *var_taub_n_dn0_slot = var_taub_n_dn0;
        *var_taub_n_dn1_slot = var_taub_n_dn1;
        *var_taub_n_dn10_slot = var_taub_n_dn10;
        *var_taub_n_dn3_slot = var_taub_n_dn3;
        *var_taub_n_dn4_slot = var_taub_n_dn4;
        *var_taub_n_dn5_slot = var_taub_n_dn5;
        *var_taub_n_dn6_slot = var_taub_n_dn6;
        *var_taub_n_dn7_slot = var_taub_n_dn7;
        *var_taub_n_dn8_slot = var_taub_n_dn8;
        *var_taub_n_dn9_slot = var_taub_n_dn9;
        *var_taub_n_rv_slot = var_taub_n_rv;
        *var_taun_slot = var_taun;
        *var_taun_dn0_slot = var_taun_dn0;
        *var_taun_dn1_slot = var_taun_dn1;
        *var_taun_dn10_slot = var_taun_dn10;
        *var_taun_dn3_slot = var_taun_dn3;
        *var_taun_dn4_slot = var_taun_dn4;
        *var_taun_dn5_slot = var_taun_dn5;
        *var_taun_dn6_slot = var_taun_dn6;
        *var_taun_dn7_slot = var_taun_dn7;
        *var_taun_dn8_slot = var_taun_dn8;
        *var_taun_dn9_slot = var_taun_dn9;
        *var_taun_rv_slot = var_taun_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        var_gmin: f64,
        var_guard117: f64,
        var_i_cth: f64,
        var_i_cth_dn3: f64,
        var_iavl: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn10: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_ib1: f64,
        var_ib1_dn0: f64,
        var_ib1_dn1: f64,
        var_ib1_dn10: f64,
        var_ib1_dn3: f64,
        var_ib1_dn4: f64,
        var_ib1_dn5: f64,
        var_ib1_dn6: f64,
        var_ib1_dn7: f64,
        var_ib1_dn8: f64,
        var_ib1_dn9: f64,
        var_ib1_s: f64,
        var_ib1_s_dn0: f64,
        var_ib1_s_dn1: f64,
        var_ib1_s_dn10: f64,
        var_ib1_s_dn3: f64,
        var_ib1_s_dn4: f64,
        var_ib1_s_dn5: f64,
        var_ib1_s_dn6: f64,
        var_ib1_s_dn7: f64,
        var_ib1_s_dn8: f64,
        var_ib1_s_dn9: f64,
        var_ib1b2: f64,
        var_ib1b2_dn0: f64,
        var_ib1b2_dn1: f64,
        var_ib1b2_dn10: f64,
        var_ib1b2_dn3: f64,
        var_ib1b2_dn4: f64,
        var_ib1b2_dn5: f64,
        var_ib1b2_dn6: f64,
        var_ib1b2_dn7: f64,
        var_ib1b2_dn8: f64,
        var_ib1b2_dn9: f64,
        var_ib2: f64,
        var_ib2_dn0: f64,
        var_ib2_dn1: f64,
        var_ib2_dn10: f64,
        var_ib2_dn3: f64,
        var_ib2_dn4: f64,
        var_ib2_dn5: f64,
        var_ib2_dn6: f64,
        var_ib2_dn7: f64,
        var_ib2_dn8: f64,
        var_ib2_dn9: f64,
        var_ib2_s: f64,
        var_ib2_s_dn0: f64,
        var_ib2_s_dn1: f64,
        var_ib2_s_dn10: f64,
        var_ib2_s_dn3: f64,
        var_ib2_s_dn4: f64,
        var_ib2_s_dn5: f64,
        var_ib2_s_dn6: f64,
        var_ib2_s_dn7: f64,
        var_ib2_s_dn8: f64,
        var_ib2_s_dn9: f64,
        var_ibrel: f64,
        var_ibrel_dn0: f64,
        var_ibrel_dn1: f64,
        var_ibrel_dn10: f64,
        var_ibrel_dn3: f64,
        var_ibrel_dn4: f64,
        var_ibrel_dn5: f64,
        var_ibrel_dn6: f64,
        var_ibrel_dn7: f64,
        var_ibrel_dn8: f64,
        var_ibrel_dn9: f64,
        var_ibtbt: f64,
        var_ibtbt_dn0: f64,
        var_ibtbt_dn1: f64,
        var_ibtbt_dn10: f64,
        var_ibtbt_dn3: f64,
        var_ibtbt_dn4: f64,
        var_ibtbt_dn5: f64,
        var_ibtbt_dn6: f64,
        var_ibtbt_dn7: f64,
        var_ibtbt_dn8: f64,
        var_ibtbt_dn9: f64,
        var_ic1c2: f64,
        var_ic1c2_dn0: f64,
        var_ic1c2_dn1: f64,
        var_ic1c2_dn10: f64,
        var_ic1c2_dn3: f64,
        var_ic1c2_dn4: f64,
        var_ic1c2_dn5: f64,
        var_ic1c2_dn6: f64,
        var_ic1c2_dn7: f64,
        var_ic1c2_dn8: f64,
        var_ic1c2_dn9: f64,
        var_in_: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_itat: f64,
        var_itat_dn0: f64,
        var_itat_dn1: f64,
        var_itat_dn10: f64,
        var_itat_dn3: f64,
        var_itat_dn4: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_iztcb: f64,
        var_iztcb_dn0: f64,
        var_iztcb_dn1: f64,
        var_iztcb_dn10: f64,
        var_iztcb_dn3: f64,
        var_iztcb_dn4: f64,
        var_iztcb_dn5: f64,
        var_iztcb_dn6: f64,
        var_iztcb_dn7: f64,
        var_iztcb_dn8: f64,
        var_iztcb_dn9: f64,
        var_izteb: f64,
        var_izteb_dn0: f64,
        var_izteb_dn1: f64,
        var_izteb_dn10: f64,
        var_izteb_dn3: f64,
        var_izteb_dn4: f64,
        var_izteb_dn5: f64,
        var_izteb_dn6: f64,
        var_izteb_dn7: f64,
        var_izteb_dn8: f64,
        var_izteb_dn9: f64,
        var_p_rth: f64,
        var_p_rth_dn3: f64,
        var_power: f64,
        var_power_dn0: f64,
        var_power_dn1: f64,
        var_power_dn10: f64,
        var_power_dn2: f64,
        var_power_dn3: f64,
        var_power_dn4: f64,
        var_power_dn5: f64,
        var_power_dn6: f64,
        var_power_dn7: f64,
        var_power_dn8: f64,
        var_power_dn9: f64,
        var_rbc_t: f64,
        var_rbc_t_dn3: f64,
        var_re_t: f64,
        var_re_t_dn3: f64,
        var_vb2e1: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn6: f64,
        var_vbb1: f64,
        var_vbb1_dn1: f64,
        var_vbb1_dn5: f64,
        var_vee1: f64,
        var_vee1_dn2: f64,
        var_vee1_dn4: f64,
    ) {
        let eq0_e154: f64 = (p.p3 * var_ic1c2);
        let eq0_e154_d_n0: f64 = (p.p3 * var_ic1c2_dn0);
        let eq0_e154_d_n1: f64 = (p.p3 * var_ic1c2_dn1);
        let eq0_e154_d_n3: f64 = (p.p3 * var_ic1c2_dn3);
        let eq0_e154_d_n4: f64 = (p.p3 * var_ic1c2_dn4);
        let eq0_e154_d_n5: f64 = (p.p3 * var_ic1c2_dn5);
        let eq0_e154_d_n6: f64 = (p.p3 * var_ic1c2_dn6);
        let eq0_e154_d_n7: f64 = (p.p3 * var_ic1c2_dn7);
        let eq0_e154_d_n8: f64 = (p.p3 * var_ic1c2_dn8);
        let eq0_e154_d_n9: f64 = (p.p3 * var_ic1c2_dn9);
        let eq0_e154_d_n10: f64 = (p.p3 * var_ic1c2_dn10);
        let eq0_e156: f64 = (eq0_e154 * p.p1);
        let eq0_e156_d_n0: f64 = (eq0_e154_d_n0 * p.p1);
        let eq0_e156_d_n1: f64 = (eq0_e154_d_n1 * p.p1);
        let eq0_e156_d_n3: f64 = (eq0_e154_d_n3 * p.p1);
        let eq0_e156_d_n4: f64 = (eq0_e154_d_n4 * p.p1);
        let eq0_e156_d_n5: f64 = (eq0_e154_d_n5 * p.p1);
        let eq0_e156_d_n6: f64 = (eq0_e154_d_n6 * p.p1);
        let eq0_e156_d_n7: f64 = (eq0_e154_d_n7 * p.p1);
        let eq0_e156_d_n8: f64 = (eq0_e154_d_n8 * p.p1);
        let eq0_e156_d_n9: f64 = (eq0_e154_d_n9 * p.p1);
        let eq0_e156_d_n10: f64 = (eq0_e154_d_n10 * p.p1);
        let eq0_value: f64 = eq0_e156;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq0_e156_d_n0), multiplicity * (eq0_e156_d_n1), multiplicity * (eq0_e156_d_n3), multiplicity * (eq0_e156_d_n4), multiplicity * (eq0_e156_d_n5), multiplicity * (eq0_e156_d_n6), multiplicity * (eq0_e156_d_n7), multiplicity * (eq0_e156_d_n8), multiplicity * (eq0_e156_d_n9), multiplicity * (eq0_e156_d_n10)],
            [],
            [],
            1.0,
        );
        let eq1_e159: f64 = (p.p3 * var_in_);
        let eq1_e159_d_n0: f64 = (p.p3 * var_in__dn0);
        let eq1_e159_d_n1: f64 = (p.p3 * var_in__dn1);
        let eq1_e159_d_n3: f64 = (p.p3 * var_in__dn3);
        let eq1_e159_d_n4: f64 = (p.p3 * var_in__dn4);
        let eq1_e159_d_n5: f64 = (p.p3 * var_in__dn5);
        let eq1_e159_d_n6: f64 = (p.p3 * var_in__dn6);
        let eq1_e159_d_n7: f64 = (p.p3 * var_in__dn7);
        let eq1_e159_d_n8: f64 = (p.p3 * var_in__dn8);
        let eq1_e159_d_n9: f64 = (p.p3 * var_in__dn9);
        let eq1_e159_d_n10: f64 = (p.p3 * var_in__dn10);
        let eq1_e161: f64 = (eq1_e159 * p.p1);
        let eq1_e161_d_n0: f64 = (eq1_e159_d_n0 * p.p1);
        let eq1_e161_d_n1: f64 = (eq1_e159_d_n1 * p.p1);
        let eq1_e161_d_n3: f64 = (eq1_e159_d_n3 * p.p1);
        let eq1_e161_d_n4: f64 = (eq1_e159_d_n4 * p.p1);
        let eq1_e161_d_n5: f64 = (eq1_e159_d_n5 * p.p1);
        let eq1_e161_d_n6: f64 = (eq1_e159_d_n6 * p.p1);
        let eq1_e161_d_n7: f64 = (eq1_e159_d_n7 * p.p1);
        let eq1_e161_d_n8: f64 = (eq1_e159_d_n8 * p.p1);
        let eq1_e161_d_n9: f64 = (eq1_e159_d_n9 * p.p1);
        let eq1_e161_d_n10: f64 = (eq1_e159_d_n10 * p.p1);
        let eq1_value: f64 = eq1_e161;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(4),
            multiplicity * (eq1_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq1_e161_d_n0), multiplicity * (eq1_e161_d_n1), multiplicity * (eq1_e161_d_n3), multiplicity * (eq1_e161_d_n4), multiplicity * (eq1_e161_d_n5), multiplicity * (eq1_e161_d_n6), multiplicity * (eq1_e161_d_n7), multiplicity * (eq1_e161_d_n8), multiplicity * (eq1_e161_d_n9), multiplicity * (eq1_e161_d_n10)],
            [],
            [],
            1.0,
        );
        let eq2_e165: f64 = (var_ib1_s + var_ib2_s);
        let eq2_e165_d_n0: f64 = (var_ib1_s_dn0 + var_ib2_s_dn0);
        let eq2_e165_d_n1: f64 = (var_ib1_s_dn1 + var_ib2_s_dn1);
        let eq2_e165_d_n3: f64 = (var_ib1_s_dn3 + var_ib2_s_dn3);
        let eq2_e165_d_n4: f64 = (var_ib1_s_dn4 + var_ib2_s_dn4);
        let eq2_e165_d_n5: f64 = (var_ib1_s_dn5 + var_ib2_s_dn5);
        let eq2_e165_d_n6: f64 = (var_ib1_s_dn6 + var_ib2_s_dn6);
        let eq2_e165_d_n7: f64 = (var_ib1_s_dn7 + var_ib2_s_dn7);
        let eq2_e165_d_n8: f64 = (var_ib1_s_dn8 + var_ib2_s_dn8);
        let eq2_e165_d_n9: f64 = (var_ib1_s_dn9 + var_ib2_s_dn9);
        let eq2_e165_d_n10: f64 = (var_ib1_s_dn10 + var_ib2_s_dn10);
        let eq2_e167: f64 = (eq2_e165 + var_ibrel);
        let eq2_e167_d_n0: f64 = (eq2_e165_d_n0 + var_ibrel_dn0);
        let eq2_e167_d_n1: f64 = (eq2_e165_d_n1 + var_ibrel_dn1);
        let eq2_e167_d_n3: f64 = (eq2_e165_d_n3 + var_ibrel_dn3);
        let eq2_e167_d_n4: f64 = (eq2_e165_d_n4 + var_ibrel_dn4);
        let eq2_e167_d_n5: f64 = (eq2_e165_d_n5 + var_ibrel_dn5);
        let eq2_e167_d_n6: f64 = (eq2_e165_d_n6 + var_ibrel_dn6);
        let eq2_e167_d_n7: f64 = (eq2_e165_d_n7 + var_ibrel_dn7);
        let eq2_e167_d_n8: f64 = (eq2_e165_d_n8 + var_ibrel_dn8);
        let eq2_e167_d_n9: f64 = (eq2_e165_d_n9 + var_ibrel_dn9);
        let eq2_e167_d_n10: f64 = (eq2_e165_d_n10 + var_ibrel_dn10);
        let eq2_e168: f64 = (p.p3 * eq2_e167);
        let eq2_e168_d_n0: f64 = (p.p3 * eq2_e167_d_n0);
        let eq2_e168_d_n1: f64 = (p.p3 * eq2_e167_d_n1);
        let eq2_e168_d_n3: f64 = (p.p3 * eq2_e167_d_n3);
        let eq2_e168_d_n4: f64 = (p.p3 * eq2_e167_d_n4);
        let eq2_e168_d_n5: f64 = (p.p3 * eq2_e167_d_n5);
        let eq2_e168_d_n6: f64 = (p.p3 * eq2_e167_d_n6);
        let eq2_e168_d_n7: f64 = (p.p3 * eq2_e167_d_n7);
        let eq2_e168_d_n8: f64 = (p.p3 * eq2_e167_d_n8);
        let eq2_e168_d_n9: f64 = (p.p3 * eq2_e167_d_n9);
        let eq2_e168_d_n10: f64 = (p.p3 * eq2_e167_d_n10);
        let eq2_e170: f64 = (eq2_e168 * p.p1);
        let eq2_e170_d_n0: f64 = (eq2_e168_d_n0 * p.p1);
        let eq2_e170_d_n1: f64 = (eq2_e168_d_n1 * p.p1);
        let eq2_e170_d_n3: f64 = (eq2_e168_d_n3 * p.p1);
        let eq2_e170_d_n4: f64 = (eq2_e168_d_n4 * p.p1);
        let eq2_e170_d_n5: f64 = (eq2_e168_d_n5 * p.p1);
        let eq2_e170_d_n6: f64 = (eq2_e168_d_n6 * p.p1);
        let eq2_e170_d_n7: f64 = (eq2_e168_d_n7 * p.p1);
        let eq2_e170_d_n8: f64 = (eq2_e168_d_n8 * p.p1);
        let eq2_e170_d_n9: f64 = (eq2_e168_d_n9 * p.p1);
        let eq2_e170_d_n10: f64 = (eq2_e168_d_n10 * p.p1);
        let eq2_value: f64 = eq2_e170;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq2_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq2_e170_d_n0), multiplicity * (eq2_e170_d_n1), multiplicity * (eq2_e170_d_n3), multiplicity * (eq2_e170_d_n4), multiplicity * (eq2_e170_d_n5), multiplicity * (eq2_e170_d_n6), multiplicity * (eq2_e170_d_n7), multiplicity * (eq2_e170_d_n8), multiplicity * (eq2_e170_d_n9), multiplicity * (eq2_e170_d_n10)],
            [],
            [],
            1.0,
        );
        let eq3_e174: f64 = (var_ib1 + var_ib2);
        let eq3_e174_d_n0: f64 = (var_ib1_dn0 + var_ib2_dn0);
        let eq3_e174_d_n1: f64 = (var_ib1_dn1 + var_ib2_dn1);
        let eq3_e174_d_n3: f64 = (var_ib1_dn3 + var_ib2_dn3);
        let eq3_e174_d_n4: f64 = (var_ib1_dn4 + var_ib2_dn4);
        let eq3_e174_d_n5: f64 = (var_ib1_dn5 + var_ib2_dn5);
        let eq3_e174_d_n6: f64 = (var_ib1_dn6 + var_ib2_dn6);
        let eq3_e174_d_n7: f64 = (var_ib1_dn7 + var_ib2_dn7);
        let eq3_e174_d_n8: f64 = (var_ib1_dn8 + var_ib2_dn8);
        let eq3_e174_d_n9: f64 = (var_ib1_dn9 + var_ib2_dn9);
        let eq3_e174_d_n10: f64 = (var_ib1_dn10 + var_ib2_dn10);
        let eq3_e177: f64 = (var_gmin * var_vb2e1);
        let eq3_e177_d_n4: f64 = (var_gmin * var_vb2e1_dn4);
        let eq3_e177_d_n6: f64 = (var_gmin * var_vb2e1_dn6);
        let eq3_e178: f64 = (eq3_e174 + eq3_e177);
        let eq3_e178_d_n4: f64 = (eq3_e174_d_n4 + eq3_e177_d_n4);
        let eq3_e178_d_n6: f64 = (eq3_e174_d_n6 + eq3_e177_d_n6);
        let eq3_e180: f64 = (eq3_e178 - var_izteb);
        let eq3_e180_d_n0: f64 = (eq3_e174_d_n0 - var_izteb_dn0);
        let eq3_e180_d_n1: f64 = (eq3_e174_d_n1 - var_izteb_dn1);
        let eq3_e180_d_n3: f64 = (eq3_e174_d_n3 - var_izteb_dn3);
        let eq3_e180_d_n4: f64 = (eq3_e178_d_n4 - var_izteb_dn4);
        let eq3_e180_d_n5: f64 = (eq3_e174_d_n5 - var_izteb_dn5);
        let eq3_e180_d_n6: f64 = (eq3_e178_d_n6 - var_izteb_dn6);
        let eq3_e180_d_n7: f64 = (eq3_e174_d_n7 - var_izteb_dn7);
        let eq3_e180_d_n8: f64 = (eq3_e174_d_n8 - var_izteb_dn8);
        let eq3_e180_d_n9: f64 = (eq3_e174_d_n9 - var_izteb_dn9);
        let eq3_e180_d_n10: f64 = (eq3_e174_d_n10 - var_izteb_dn10);
        let eq3_e182: f64 = (eq3_e180 + var_ibtbt);
        let eq3_e182_d_n0: f64 = (eq3_e180_d_n0 + var_ibtbt_dn0);
        let eq3_e182_d_n1: f64 = (eq3_e180_d_n1 + var_ibtbt_dn1);
        let eq3_e182_d_n3: f64 = (eq3_e180_d_n3 + var_ibtbt_dn3);
        let eq3_e182_d_n4: f64 = (eq3_e180_d_n4 + var_ibtbt_dn4);
        let eq3_e182_d_n5: f64 = (eq3_e180_d_n5 + var_ibtbt_dn5);
        let eq3_e182_d_n6: f64 = (eq3_e180_d_n6 + var_ibtbt_dn6);
        let eq3_e182_d_n7: f64 = (eq3_e180_d_n7 + var_ibtbt_dn7);
        let eq3_e182_d_n8: f64 = (eq3_e180_d_n8 + var_ibtbt_dn8);
        let eq3_e182_d_n9: f64 = (eq3_e180_d_n9 + var_ibtbt_dn9);
        let eq3_e182_d_n10: f64 = (eq3_e180_d_n10 + var_ibtbt_dn10);
        let eq3_e184: f64 = (eq3_e182 + var_itat);
        let eq3_e184_d_n0: f64 = (eq3_e182_d_n0 + var_itat_dn0);
        let eq3_e184_d_n1: f64 = (eq3_e182_d_n1 + var_itat_dn1);
        let eq3_e184_d_n3: f64 = (eq3_e182_d_n3 + var_itat_dn3);
        let eq3_e184_d_n4: f64 = (eq3_e182_d_n4 + var_itat_dn4);
        let eq3_e184_d_n5: f64 = (eq3_e182_d_n5 + var_itat_dn5);
        let eq3_e184_d_n6: f64 = (eq3_e182_d_n6 + var_itat_dn6);
        let eq3_e184_d_n7: f64 = (eq3_e182_d_n7 + var_itat_dn7);
        let eq3_e184_d_n8: f64 = (eq3_e182_d_n8 + var_itat_dn8);
        let eq3_e184_d_n9: f64 = (eq3_e182_d_n9 + var_itat_dn9);
        let eq3_e184_d_n10: f64 = (eq3_e182_d_n10 + var_itat_dn10);
        let eq3_e185: f64 = (p.p3 * eq3_e184);
        let eq3_e185_d_n0: f64 = (p.p3 * eq3_e184_d_n0);
        let eq3_e185_d_n1: f64 = (p.p3 * eq3_e184_d_n1);
        let eq3_e185_d_n3: f64 = (p.p3 * eq3_e184_d_n3);
        let eq3_e185_d_n4: f64 = (p.p3 * eq3_e184_d_n4);
        let eq3_e185_d_n5: f64 = (p.p3 * eq3_e184_d_n5);
        let eq3_e185_d_n6: f64 = (p.p3 * eq3_e184_d_n6);
        let eq3_e185_d_n7: f64 = (p.p3 * eq3_e184_d_n7);
        let eq3_e185_d_n8: f64 = (p.p3 * eq3_e184_d_n8);
        let eq3_e185_d_n9: f64 = (p.p3 * eq3_e184_d_n9);
        let eq3_e185_d_n10: f64 = (p.p3 * eq3_e184_d_n10);
        let eq3_e187: f64 = (eq3_e185 * p.p1);
        let eq3_e187_d_n0: f64 = (eq3_e185_d_n0 * p.p1);
        let eq3_e187_d_n1: f64 = (eq3_e185_d_n1 * p.p1);
        let eq3_e187_d_n3: f64 = (eq3_e185_d_n3 * p.p1);
        let eq3_e187_d_n4: f64 = (eq3_e185_d_n4 * p.p1);
        let eq3_e187_d_n5: f64 = (eq3_e185_d_n5 * p.p1);
        let eq3_e187_d_n6: f64 = (eq3_e185_d_n6 * p.p1);
        let eq3_e187_d_n7: f64 = (eq3_e185_d_n7 * p.p1);
        let eq3_e187_d_n8: f64 = (eq3_e185_d_n8 * p.p1);
        let eq3_e187_d_n9: f64 = (eq3_e185_d_n9 * p.p1);
        let eq3_e187_d_n10: f64 = (eq3_e185_d_n10 * p.p1);
        let eq3_value: f64 = eq3_e187;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq3_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq3_e187_d_n0), multiplicity * (eq3_e187_d_n1), multiplicity * (eq3_e187_d_n3), multiplicity * (eq3_e187_d_n4), multiplicity * (eq3_e187_d_n5), multiplicity * (eq3_e187_d_n6), multiplicity * (eq3_e187_d_n7), multiplicity * (eq3_e187_d_n8), multiplicity * (eq3_e187_d_n9), multiplicity * (eq3_e187_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq4_e196, eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10,) = {
    if (var_guard117 != 0.0) {
        let eq4_e191: f64 = (-var_iztcb);
        let eq4_e192: f64 = (p.p3 * eq4_e191);
        let eq4_e192_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq4_e192_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq4_e192_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq4_e192_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq4_e192_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq4_e192_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq4_e192_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq4_e192_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq4_e192_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq4_e192_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq4_e194: f64 = (eq4_e192 * p.p1);
        let eq4_e194_d_n0: f64 = (eq4_e192_d_n0 * p.p1);
        let eq4_e194_d_n1: f64 = (eq4_e192_d_n1 * p.p1);
        let eq4_e194_d_n3: f64 = (eq4_e192_d_n3 * p.p1);
        let eq4_e194_d_n4: f64 = (eq4_e192_d_n4 * p.p1);
        let eq4_e194_d_n5: f64 = (eq4_e192_d_n5 * p.p1);
        let eq4_e194_d_n6: f64 = (eq4_e192_d_n6 * p.p1);
        let eq4_e194_d_n7: f64 = (eq4_e192_d_n7 * p.p1);
        let eq4_e194_d_n8: f64 = (eq4_e192_d_n8 * p.p1);
        let eq4_e194_d_n9: f64 = (eq4_e192_d_n9 * p.p1);
        let eq4_e194_d_n10: f64 = (eq4_e192_d_n10 * p.p1);
        (eq4_e194, eq4_e194_d_n0, eq4_e194_d_n1, eq4_e194_d_n3, eq4_e194_d_n4, eq4_e194_d_n5, eq4_e194_d_n6, eq4_e194_d_n7, eq4_e194_d_n8, eq4_e194_d_n9, eq4_e194_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e196;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq4_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq4_e196_d_n0), multiplicity * (eq4_e196_d_n1), multiplicity * (eq4_e196_d_n3), multiplicity * (eq4_e196_d_n4), multiplicity * (eq4_e196_d_n5), multiplicity * (eq4_e196_d_n6), multiplicity * (eq4_e196_d_n7), multiplicity * (eq4_e196_d_n8), multiplicity * (eq4_e196_d_n9), multiplicity * (eq4_e196_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq5_e206, eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10,) = {
    if (var_guard117 == 0.0) {
        let eq5_e201: f64 = (-var_iztcb);
        let eq5_e202: f64 = (p.p3 * eq5_e201);
        let eq5_e202_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq5_e202_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq5_e202_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq5_e202_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq5_e202_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq5_e202_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq5_e202_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq5_e202_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq5_e202_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq5_e202_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq5_e204: f64 = (eq5_e202 * p.p1);
        let eq5_e204_d_n0: f64 = (eq5_e202_d_n0 * p.p1);
        let eq5_e204_d_n1: f64 = (eq5_e202_d_n1 * p.p1);
        let eq5_e204_d_n3: f64 = (eq5_e202_d_n3 * p.p1);
        let eq5_e204_d_n4: f64 = (eq5_e202_d_n4 * p.p1);
        let eq5_e204_d_n5: f64 = (eq5_e202_d_n5 * p.p1);
        let eq5_e204_d_n6: f64 = (eq5_e202_d_n6 * p.p1);
        let eq5_e204_d_n7: f64 = (eq5_e202_d_n7 * p.p1);
        let eq5_e204_d_n8: f64 = (eq5_e202_d_n8 * p.p1);
        let eq5_e204_d_n9: f64 = (eq5_e202_d_n9 * p.p1);
        let eq5_e204_d_n10: f64 = (eq5_e202_d_n10 * p.p1);
        (eq5_e204, eq5_e204_d_n0, eq5_e204_d_n1, eq5_e204_d_n3, eq5_e204_d_n4, eq5_e204_d_n5, eq5_e204_d_n6, eq5_e204_d_n7, eq5_e204_d_n8, eq5_e204_d_n9, eq5_e204_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e206;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq5_e206_d_n0), multiplicity * (eq5_e206_d_n1), multiplicity * (eq5_e206_d_n3), multiplicity * (eq5_e206_d_n4), multiplicity * (eq5_e206_d_n5), multiplicity * (eq5_e206_d_n6), multiplicity * (eq5_e206_d_n7), multiplicity * (eq5_e206_d_n8), multiplicity * (eq5_e206_d_n9), multiplicity * (eq5_e206_d_n10)],
            [],
            [],
            1.0,
        );
        let eq6_e209: f64 = (p.p3 * var_ib1b2);
        let eq6_e209_d_n0: f64 = (p.p3 * var_ib1b2_dn0);
        let eq6_e209_d_n1: f64 = (p.p3 * var_ib1b2_dn1);
        let eq6_e209_d_n3: f64 = (p.p3 * var_ib1b2_dn3);
        let eq6_e209_d_n4: f64 = (p.p3 * var_ib1b2_dn4);
        let eq6_e209_d_n5: f64 = (p.p3 * var_ib1b2_dn5);
        let eq6_e209_d_n6: f64 = (p.p3 * var_ib1b2_dn6);
        let eq6_e209_d_n7: f64 = (p.p3 * var_ib1b2_dn7);
        let eq6_e209_d_n8: f64 = (p.p3 * var_ib1b2_dn8);
        let eq6_e209_d_n9: f64 = (p.p3 * var_ib1b2_dn9);
        let eq6_e209_d_n10: f64 = (p.p3 * var_ib1b2_dn10);
        let eq6_e211: f64 = (eq6_e209 * p.p1);
        let eq6_e211_d_n0: f64 = (eq6_e209_d_n0 * p.p1);
        let eq6_e211_d_n1: f64 = (eq6_e209_d_n1 * p.p1);
        let eq6_e211_d_n3: f64 = (eq6_e209_d_n3 * p.p1);
        let eq6_e211_d_n4: f64 = (eq6_e209_d_n4 * p.p1);
        let eq6_e211_d_n5: f64 = (eq6_e209_d_n5 * p.p1);
        let eq6_e211_d_n6: f64 = (eq6_e209_d_n6 * p.p1);
        let eq6_e211_d_n7: f64 = (eq6_e209_d_n7 * p.p1);
        let eq6_e211_d_n8: f64 = (eq6_e209_d_n8 * p.p1);
        let eq6_e211_d_n9: f64 = (eq6_e209_d_n9 * p.p1);
        let eq6_e211_d_n10: f64 = (eq6_e209_d_n10 * p.p1);
        let eq6_value: f64 = eq6_e211;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq6_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq6_e211_d_n0), multiplicity * (eq6_e211_d_n1), multiplicity * (eq6_e211_d_n3), multiplicity * (eq6_e211_d_n4), multiplicity * (eq6_e211_d_n5), multiplicity * (eq6_e211_d_n6), multiplicity * (eq6_e211_d_n7), multiplicity * (eq6_e211_d_n8), multiplicity * (eq6_e211_d_n9), multiplicity * (eq6_e211_d_n10)],
            [],
            [],
            1.0,
        );
        let eq7_e214: f64 = (-1.0);
        let eq7_e216: f64 = (eq7_e214 * var_iavl);
        let eq7_e216_d_n0: f64 = (eq7_e214 * var_iavl_dn0);
        let eq7_e216_d_n1: f64 = (eq7_e214 * var_iavl_dn1);
        let eq7_e216_d_n3: f64 = (eq7_e214 * var_iavl_dn3);
        let eq7_e216_d_n4: f64 = (eq7_e214 * var_iavl_dn4);
        let eq7_e216_d_n5: f64 = (eq7_e214 * var_iavl_dn5);
        let eq7_e216_d_n6: f64 = (eq7_e214 * var_iavl_dn6);
        let eq7_e216_d_n7: f64 = (eq7_e214 * var_iavl_dn7);
        let eq7_e216_d_n8: f64 = (eq7_e214 * var_iavl_dn8);
        let eq7_e216_d_n9: f64 = (eq7_e214 * var_iavl_dn9);
        let eq7_e216_d_n10: f64 = (eq7_e214 * var_iavl_dn10);
        let eq7_e217: f64 = (p.p3 * eq7_e216);
        let eq7_e217_d_n0: f64 = (p.p3 * eq7_e216_d_n0);
        let eq7_e217_d_n1: f64 = (p.p3 * eq7_e216_d_n1);
        let eq7_e217_d_n3: f64 = (p.p3 * eq7_e216_d_n3);
        let eq7_e217_d_n4: f64 = (p.p3 * eq7_e216_d_n4);
        let eq7_e217_d_n5: f64 = (p.p3 * eq7_e216_d_n5);
        let eq7_e217_d_n6: f64 = (p.p3 * eq7_e216_d_n6);
        let eq7_e217_d_n7: f64 = (p.p3 * eq7_e216_d_n7);
        let eq7_e217_d_n8: f64 = (p.p3 * eq7_e216_d_n8);
        let eq7_e217_d_n9: f64 = (p.p3 * eq7_e216_d_n9);
        let eq7_e217_d_n10: f64 = (p.p3 * eq7_e216_d_n10);
        let eq7_e219: f64 = (eq7_e217 * p.p1);
        let eq7_e219_d_n0: f64 = (eq7_e217_d_n0 * p.p1);
        let eq7_e219_d_n1: f64 = (eq7_e217_d_n1 * p.p1);
        let eq7_e219_d_n3: f64 = (eq7_e217_d_n3 * p.p1);
        let eq7_e219_d_n4: f64 = (eq7_e217_d_n4 * p.p1);
        let eq7_e219_d_n5: f64 = (eq7_e217_d_n5 * p.p1);
        let eq7_e219_d_n6: f64 = (eq7_e217_d_n6 * p.p1);
        let eq7_e219_d_n7: f64 = (eq7_e217_d_n7 * p.p1);
        let eq7_e219_d_n8: f64 = (eq7_e217_d_n8 * p.p1);
        let eq7_e219_d_n9: f64 = (eq7_e217_d_n9 * p.p1);
        let eq7_e219_d_n10: f64 = (eq7_e217_d_n10 * p.p1);
        let eq7_value: f64 = eq7_e219;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq7_e219_d_n0), multiplicity * (eq7_e219_d_n1), multiplicity * (eq7_e219_d_n3), multiplicity * (eq7_e219_d_n4), multiplicity * (eq7_e219_d_n5), multiplicity * (eq7_e219_d_n6), multiplicity * (eq7_e219_d_n7), multiplicity * (eq7_e219_d_n8), multiplicity * (eq7_e219_d_n9), multiplicity * (eq7_e219_d_n10)],
            [],
            [],
            1.0,
        );
        let eq8_e222: f64 = (p.p3 * var_vee1);
        let eq8_e222_d_n2: f64 = (p.p3 * var_vee1_dn2);
        let eq8_e222_d_n4: f64 = (p.p3 * var_vee1_dn4);
        let __rspice_inv_cse_0: f64 = 1.0 / var_re_t;
        let eq8_e224: f64 = (eq8_e222 * __rspice_inv_cse_0);
        let eq8_e224_d_n2: f64 = (eq8_e222_d_n2 * __rspice_inv_cse_0);
        let eq8_e224_d_n3: f64 = (-((eq8_e222 * var_re_t_dn3) / (var_re_t * var_re_t)));
        let eq8_e224_d_n4: f64 = (eq8_e222_d_n4 / var_re_t);
        let eq8_e226: f64 = (eq8_e224 * p.p1);
        let eq8_e226_d_n2: f64 = (eq8_e224_d_n2 * p.p1);
        let eq8_e226_d_n3: f64 = (eq8_e224_d_n3 * p.p1);
        let eq8_e226_d_n4: f64 = (eq8_e224_d_n4 * p.p1);
        let eq8_value: f64 = eq8_e226;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (eq8_value),
            2,
            multiplicity * (eq8_e226_d_n2),
            3,
            multiplicity * (eq8_e226_d_n3),
            4,
            multiplicity * (eq8_e226_d_n4),
        );
        let eq9_e229: f64 = (p.p3 * var_vbb1);
        let eq9_e229_d_n1: f64 = (p.p3 * var_vbb1_dn1);
        let eq9_e229_d_n5: f64 = (p.p3 * var_vbb1_dn5);
        let __rspice_inv_cse_1: f64 = 1.0 / var_rbc_t;
        let eq9_e231: f64 = (eq9_e229 * __rspice_inv_cse_1);
        let eq9_e231_d_n1: f64 = (eq9_e229_d_n1 * __rspice_inv_cse_1);
        let eq9_e231_d_n3: f64 = (-((eq9_e229 * var_rbc_t_dn3) / (var_rbc_t * var_rbc_t)));
        let eq9_e231_d_n5: f64 = (eq9_e229_d_n5 / var_rbc_t);
        let eq9_e233: f64 = (eq9_e231 * p.p1);
        let eq9_e233_d_n1: f64 = (eq9_e231_d_n1 * p.p1);
        let eq9_e233_d_n3: f64 = (eq9_e231_d_n3 * p.p1);
        let eq9_e233_d_n5: f64 = (eq9_e231_d_n5 * p.p1);
        let eq9_value: f64 = eq9_e233;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (eq9_value),
            1,
            multiplicity * (eq9_e233_d_n1),
            3,
            multiplicity * (eq9_e233_d_n3),
            5,
            multiplicity * (eq9_e233_d_n5),
        );
        let eq10_value: f64 = var_p_rth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq10_value),
            3,
            multiplicity * (var_p_rth_dn3),
        );
        let eq11_value: f64 = var_i_cth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            3,
            multiplicity * (var_i_cth_dn3),
        );
        let eq12_e237: f64 = (-1.0);
        let eq12_e239: f64 = (eq12_e237 * var_power);
        let eq12_e239_d_n0: f64 = (eq12_e237 * var_power_dn0);
        let eq12_e239_d_n1: f64 = (eq12_e237 * var_power_dn1);
        let eq12_e239_d_n2: f64 = (eq12_e237 * var_power_dn2);
        let eq12_e239_d_n3: f64 = (eq12_e237 * var_power_dn3);
        let eq12_e239_d_n4: f64 = (eq12_e237 * var_power_dn4);
        let eq12_e239_d_n5: f64 = (eq12_e237 * var_power_dn5);
        let eq12_e239_d_n6: f64 = (eq12_e237 * var_power_dn6);
        let eq12_e239_d_n7: f64 = (eq12_e237 * var_power_dn7);
        let eq12_e239_d_n8: f64 = (eq12_e237 * var_power_dn8);
        let eq12_e239_d_n9: f64 = (eq12_e237 * var_power_dn9);
        let eq12_e239_d_n10: f64 = (eq12_e237 * var_power_dn10);
        let eq12_e241: f64 = (eq12_e239 * p.p1);
        let eq12_e241_d_n0: f64 = (eq12_e239_d_n0 * p.p1);
        let eq12_e241_d_n1: f64 = (eq12_e239_d_n1 * p.p1);
        let eq12_e241_d_n2: f64 = (eq12_e239_d_n2 * p.p1);
        let eq12_e241_d_n3: f64 = (eq12_e239_d_n3 * p.p1);
        let eq12_e241_d_n4: f64 = (eq12_e239_d_n4 * p.p1);
        let eq12_e241_d_n5: f64 = (eq12_e239_d_n5 * p.p1);
        let eq12_e241_d_n6: f64 = (eq12_e239_d_n6 * p.p1);
        let eq12_e241_d_n7: f64 = (eq12_e239_d_n7 * p.p1);
        let eq12_e241_d_n8: f64 = (eq12_e239_d_n8 * p.p1);
        let eq12_e241_d_n9: f64 = (eq12_e239_d_n9 * p.p1);
        let eq12_e241_d_n10: f64 = (eq12_e239_d_n10 * p.p1);
        let eq12_value: f64 = eq12_e241;
        let eq12_node_derivative_indices: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let eq12_node_derivatives: [f64; 11] = [eq12_e241_d_n0, eq12_e241_d_n1, eq12_e241_d_n2, eq12_e241_d_n3, eq12_e241_d_n4, eq12_e241_d_n5, eq12_e241_d_n6, eq12_e241_d_n7, eq12_e241_d_n8, eq12_e241_d_n9, eq12_e241_d_n10];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gcc_ex_t: f64,
        var_gcc_ex_t_dn3: f64,
        var_gcc_in_t: f64,
        var_gcc_in_t_dn3: f64,
        var_gcc_xx_t: f64,
        var_gcc_xx_t_dn3: f64,
        var_gmin: f64,
        var_guard121: f64,
        var_guard122: f64,
        var_ib3: f64,
        var_ib3_dn0: f64,
        var_ib3_dn1: f64,
        var_ib3_dn10: f64,
        var_ib3_dn3: f64,
        var_ib3_dn4: f64,
        var_ib3_dn5: f64,
        var_ib3_dn6: f64,
        var_ib3_dn7: f64,
        var_ib3_dn8: f64,
        var_ib3_dn9: f64,
        var_iex: f64,
        var_iex_dn0: f64,
        var_iex_dn1: f64,
        var_iex_dn10: f64,
        var_iex_dn3: f64,
        var_iex_dn4: f64,
        var_iex_dn5: f64,
        var_iex_dn6: f64,
        var_iex_dn7: f64,
        var_iex_dn8: f64,
        var_iex_dn9: f64,
        var_qb1b2: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn10: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn10: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_vb1c4: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vbc: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbe: f64,
        var_vbe_dn1: f64,
        var_vbe_dn2: f64,
        var_vc3c4: f64,
        var_vc3c4_dn10: f64,
        var_vc3c4_dn9: f64,
        var_vc4c1: f64,
        var_vc4c1_dn10: f64,
        var_vc4c1_dn7: f64,
        var_vcc3: f64,
        var_vcc3_dn0: f64,
        var_vcc3_dn1: f64,
        var_vcc3_dn10: f64,
        var_vcc3_dn5: f64,
        var_vcc3_dn6: f64,
        var_vcc3_dn7: f64,
        var_vcc3_dn8: f64,
        var_vcc3_dn9: f64,
        var_xiex: f64,
        var_xiex_dn0: f64,
        var_xiex_dn1: f64,
        var_xiex_dn10: f64,
        var_xiex_dn3: f64,
        var_xiex_dn4: f64,
        var_xiex_dn5: f64,
        var_xiex_dn6: f64,
        var_xiex_dn7: f64,
        var_xiex_dn8: f64,
        var_xiex_dn9: f64,
        var_xqex: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn10: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let eq13_e245: f64 = (var_qte + var_qbe);
        let eq13_e245_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq13_e248);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = ((eq13_e248_d_n0 * ddt_scale) * p.p1);
        let eq13_e251_d_n1: f64 = ((eq13_e248_d_n1 * ddt_scale) * p.p1);
        let eq13_e251_d_n3: f64 = ((eq13_e248_d_n3 * ddt_scale) * p.p1);
        let eq13_e251_d_n4: f64 = ((eq13_e248_d_n4 * ddt_scale) * p.p1);
        let eq13_e251_d_n5: f64 = ((eq13_e248_d_n5 * ddt_scale) * p.p1);
        let eq13_e251_d_n6: f64 = ((eq13_e248_d_n6 * ddt_scale) * p.p1);
        let eq13_e251_d_n7: f64 = ((eq13_e248_d_n7 * ddt_scale) * p.p1);
        let eq13_e251_d_n8: f64 = ((eq13_e248_d_n8 * ddt_scale) * p.p1);
        let eq13_e251_d_n9: f64 = ((eq13_e248_d_n9 * ddt_scale) * p.p1);
        let eq13_e251_d_n10: f64 = ((eq13_e248_d_n10 * ddt_scale) * p.p1);
        let eq13_value: f64 = eq13_e251;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq13_e251_d_n0), multiplicity * (eq13_e251_d_n1), multiplicity * (eq13_e251_d_n3), multiplicity * (eq13_e251_d_n4), multiplicity * (eq13_e251_d_n5), multiplicity * (eq13_e251_d_n6), multiplicity * (eq13_e251_d_n7), multiplicity * (eq13_e251_d_n8), multiplicity * (eq13_e251_d_n9), multiplicity * (eq13_e251_d_n10)],
            [],
            [],
            1.0,
        );
        let eq14_e254: f64 = (p.p3 * var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq14_e255: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq14_e254);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = ((eq14_e254_d_n0 * ddt_scale) * p.p1);
        let eq14_e257_d_n1: f64 = ((eq14_e254_d_n1 * ddt_scale) * p.p1);
        let eq14_e257_d_n3: f64 = ((eq14_e254_d_n3 * ddt_scale) * p.p1);
        let eq14_e257_d_n4: f64 = ((eq14_e254_d_n4 * ddt_scale) * p.p1);
        let eq14_e257_d_n5: f64 = ((eq14_e254_d_n5 * ddt_scale) * p.p1);
        let eq14_e257_d_n6: f64 = ((eq14_e254_d_n6 * ddt_scale) * p.p1);
        let eq14_e257_d_n7: f64 = ((eq14_e254_d_n7 * ddt_scale) * p.p1);
        let eq14_e257_d_n8: f64 = ((eq14_e254_d_n8 * ddt_scale) * p.p1);
        let eq14_e257_d_n9: f64 = ((eq14_e254_d_n9 * ddt_scale) * p.p1);
        let eq14_e257_d_n10: f64 = ((eq14_e254_d_n10 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e257;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq14_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq14_e257_d_n0), multiplicity * (eq14_e257_d_n1), multiplicity * (eq14_e257_d_n3), multiplicity * (eq14_e257_d_n4), multiplicity * (eq14_e257_d_n5), multiplicity * (eq14_e257_d_n6), multiplicity * (eq14_e257_d_n7), multiplicity * (eq14_e257_d_n8), multiplicity * (eq14_e257_d_n9), multiplicity * (eq14_e257_d_n10)],
            [],
            [],
            1.0,
        );
        let eq15_e261: f64 = (var_qtc + var_qbc);
        let eq15_e261_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq15_e264);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = ((eq15_e264_d_n0 * ddt_scale) * p.p1);
        let eq15_e267_d_n1: f64 = ((eq15_e264_d_n1 * ddt_scale) * p.p1);
        let eq15_e267_d_n3: f64 = ((eq15_e264_d_n3 * ddt_scale) * p.p1);
        let eq15_e267_d_n4: f64 = ((eq15_e264_d_n4 * ddt_scale) * p.p1);
        let eq15_e267_d_n5: f64 = ((eq15_e264_d_n5 * ddt_scale) * p.p1);
        let eq15_e267_d_n6: f64 = ((eq15_e264_d_n6 * ddt_scale) * p.p1);
        let eq15_e267_d_n7: f64 = ((eq15_e264_d_n7 * ddt_scale) * p.p1);
        let eq15_e267_d_n8: f64 = ((eq15_e264_d_n8 * ddt_scale) * p.p1);
        let eq15_e267_d_n9: f64 = ((eq15_e264_d_n9 * ddt_scale) * p.p1);
        let eq15_e267_d_n10: f64 = ((eq15_e264_d_n10 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e267;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq15_e267_d_n0), multiplicity * (eq15_e267_d_n1), multiplicity * (eq15_e267_d_n3), multiplicity * (eq15_e267_d_n4), multiplicity * (eq15_e267_d_n5), multiplicity * (eq15_e267_d_n6), multiplicity * (eq15_e267_d_n7), multiplicity * (eq15_e267_d_n8), multiplicity * (eq15_e267_d_n9), multiplicity * (eq15_e267_d_n10)],
            [],
            [],
            1.0,
        );
        let eq16_e270: f64 = (p.p3 * var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq16_e271: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq16_e270);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = ((eq16_e270_d_n0 * ddt_scale) * p.p1);
        let eq16_e273_d_n1: f64 = ((eq16_e270_d_n1 * ddt_scale) * p.p1);
        let eq16_e273_d_n3: f64 = ((eq16_e270_d_n3 * ddt_scale) * p.p1);
        let eq16_e273_d_n4: f64 = ((eq16_e270_d_n4 * ddt_scale) * p.p1);
        let eq16_e273_d_n5: f64 = ((eq16_e270_d_n5 * ddt_scale) * p.p1);
        let eq16_e273_d_n6: f64 = ((eq16_e270_d_n6 * ddt_scale) * p.p1);
        let eq16_e273_d_n7: f64 = ((eq16_e270_d_n7 * ddt_scale) * p.p1);
        let eq16_e273_d_n8: f64 = ((eq16_e270_d_n8 * ddt_scale) * p.p1);
        let eq16_e273_d_n9: f64 = ((eq16_e270_d_n9 * ddt_scale) * p.p1);
        let eq16_e273_d_n10: f64 = ((eq16_e270_d_n10 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e273;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq16_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq16_e273_d_n0), multiplicity * (eq16_e273_d_n1), multiplicity * (eq16_e273_d_n3), multiplicity * (eq16_e273_d_n4), multiplicity * (eq16_e273_d_n5), multiplicity * (eq16_e273_d_n6), multiplicity * (eq16_e273_d_n7), multiplicity * (eq16_e273_d_n8), multiplicity * (eq16_e273_d_n9), multiplicity * (eq16_e273_d_n10)],
            [],
            [],
            1.0,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * var_vbe_dn2);
        let eq17_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq17_e278);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n1: f64 = ((eq17_e278_d_n1 * ddt_scale) * p.p1);
        let eq17_e281_d_n2: f64 = ((eq17_e278_d_n2 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e281;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            1,
            multiplicity * (eq17_e281_d_n1),
            2,
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * var_vbc_dn1);
        let eq18_e287: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq18_e286);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = ((eq18_e286_d_n0 * ddt_scale) * p.p1);
        let eq18_e289_d_n1: f64 = ((eq18_e286_d_n1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e289;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            0,
            multiplicity * (eq18_e289_d_n0),
            1,
            multiplicity * (eq18_e289_d_n1),
        );
        let eq19_e292: f64 = (p.p3 * var_xiex);
        let eq19_e292_d_n0: f64 = (p.p3 * var_xiex_dn0);
        let eq19_e292_d_n1: f64 = (p.p3 * var_xiex_dn1);
        let eq19_e292_d_n3: f64 = (p.p3 * var_xiex_dn3);
        let eq19_e292_d_n4: f64 = (p.p3 * var_xiex_dn4);
        let eq19_e292_d_n5: f64 = (p.p3 * var_xiex_dn5);
        let eq19_e292_d_n6: f64 = (p.p3 * var_xiex_dn6);
        let eq19_e292_d_n7: f64 = (p.p3 * var_xiex_dn7);
        let eq19_e292_d_n8: f64 = (p.p3 * var_xiex_dn8);
        let eq19_e292_d_n9: f64 = (p.p3 * var_xiex_dn9);
        let eq19_e292_d_n10: f64 = (p.p3 * var_xiex_dn10);
        let eq19_e294: f64 = (eq19_e292 * p.p1);
        let eq19_e294_d_n0: f64 = (eq19_e292_d_n0 * p.p1);
        let eq19_e294_d_n1: f64 = (eq19_e292_d_n1 * p.p1);
        let eq19_e294_d_n3: f64 = (eq19_e292_d_n3 * p.p1);
        let eq19_e294_d_n4: f64 = (eq19_e292_d_n4 * p.p1);
        let eq19_e294_d_n5: f64 = (eq19_e292_d_n5 * p.p1);
        let eq19_e294_d_n6: f64 = (eq19_e292_d_n6 * p.p1);
        let eq19_e294_d_n7: f64 = (eq19_e292_d_n7 * p.p1);
        let eq19_e294_d_n8: f64 = (eq19_e292_d_n8 * p.p1);
        let eq19_e294_d_n9: f64 = (eq19_e292_d_n9 * p.p1);
        let eq19_e294_d_n10: f64 = (eq19_e292_d_n10 * p.p1);
        let eq19_value: f64 = eq19_e294;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq19_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq19_e294_d_n0), multiplicity * (eq19_e294_d_n1), multiplicity * (eq19_e294_d_n3), multiplicity * (eq19_e294_d_n4), multiplicity * (eq19_e294_d_n5), multiplicity * (eq19_e294_d_n6), multiplicity * (eq19_e294_d_n7), multiplicity * (eq19_e294_d_n8), multiplicity * (eq19_e294_d_n9), multiplicity * (eq19_e294_d_n10)],
            [],
            [],
            1.0,
        );
        let eq20_e297: f64 = (p.p3 * var_vcc3);
        let eq20_e297_d_n0: f64 = (p.p3 * var_vcc3_dn0);
        let eq20_e297_d_n1: f64 = (p.p3 * var_vcc3_dn1);
        let eq20_e297_d_n5: f64 = (p.p3 * var_vcc3_dn5);
        let eq20_e297_d_n6: f64 = (p.p3 * var_vcc3_dn6);
        let eq20_e297_d_n7: f64 = (p.p3 * var_vcc3_dn7);
        let eq20_e297_d_n8: f64 = (p.p3 * var_vcc3_dn8);
        let eq20_e297_d_n9: f64 = (p.p3 * var_vcc3_dn9);
        let eq20_e297_d_n10: f64 = (p.p3 * var_vcc3_dn10);
        let eq20_e299: f64 = (eq20_e297 * var_gcc_xx_t);
        let eq20_e299_d_n0: f64 = (eq20_e297_d_n0 * var_gcc_xx_t);
        let eq20_e299_d_n1: f64 = (eq20_e297_d_n1 * var_gcc_xx_t);
        let eq20_e299_d_n3: f64 = (eq20_e297 * var_gcc_xx_t_dn3);
        let eq20_e299_d_n5: f64 = (eq20_e297_d_n5 * var_gcc_xx_t);
        let eq20_e299_d_n6: f64 = (eq20_e297_d_n6 * var_gcc_xx_t);
        let eq20_e299_d_n7: f64 = (eq20_e297_d_n7 * var_gcc_xx_t);
        let eq20_e299_d_n8: f64 = (eq20_e297_d_n8 * var_gcc_xx_t);
        let eq20_e299_d_n9: f64 = (eq20_e297_d_n9 * var_gcc_xx_t);
        let eq20_e299_d_n10: f64 = (eq20_e297_d_n10 * var_gcc_xx_t);
        let eq20_e301: f64 = (eq20_e299 * p.p1);
        let eq20_e301_d_n0: f64 = (eq20_e299_d_n0 * p.p1);
        let eq20_e301_d_n1: f64 = (eq20_e299_d_n1 * p.p1);
        let eq20_e301_d_n3: f64 = (eq20_e299_d_n3 * p.p1);
        let eq20_e301_d_n5: f64 = (eq20_e299_d_n5 * p.p1);
        let eq20_e301_d_n6: f64 = (eq20_e299_d_n6 * p.p1);
        let eq20_e301_d_n7: f64 = (eq20_e299_d_n7 * p.p1);
        let eq20_e301_d_n8: f64 = (eq20_e299_d_n8 * p.p1);
        let eq20_e301_d_n9: f64 = (eq20_e299_d_n9 * p.p1);
        let eq20_e301_d_n10: f64 = (eq20_e299_d_n10 * p.p1);
        let eq20_value: f64 = eq20_e301;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * (eq20_value),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq20_e301_d_n0), multiplicity * (eq20_e301_d_n1), multiplicity * (eq20_e301_d_n3), multiplicity * (eq20_e301_d_n5), multiplicity * (eq20_e301_d_n6), multiplicity * (eq20_e301_d_n7), multiplicity * (eq20_e301_d_n8), multiplicity * (eq20_e301_d_n9), multiplicity * (eq20_e301_d_n10)],
            [],
            [],
            1.0,
        );
        let eq21_e305: f64 = (var_xqtex + var_xqex);
        let eq21_e305_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq21_e306);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = ((eq21_e306_d_n0 * ddt_scale) * p.p1);
        let eq21_e309_d_n1: f64 = ((eq21_e306_d_n1 * ddt_scale) * p.p1);
        let eq21_e309_d_n3: f64 = ((eq21_e306_d_n3 * ddt_scale) * p.p1);
        let eq21_e309_d_n4: f64 = ((eq21_e306_d_n4 * ddt_scale) * p.p1);
        let eq21_e309_d_n5: f64 = ((eq21_e306_d_n5 * ddt_scale) * p.p1);
        let eq21_e309_d_n6: f64 = ((eq21_e306_d_n6 * ddt_scale) * p.p1);
        let eq21_e309_d_n7: f64 = ((eq21_e306_d_n7 * ddt_scale) * p.p1);
        let eq21_e309_d_n8: f64 = ((eq21_e306_d_n8 * ddt_scale) * p.p1);
        let eq21_e309_d_n9: f64 = ((eq21_e306_d_n9 * ddt_scale) * p.p1);
        let eq21_e309_d_n10: f64 = ((eq21_e306_d_n10 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e309;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq21_e309_d_n0), multiplicity * (eq21_e309_d_n1), multiplicity * (eq21_e309_d_n3), multiplicity * (eq21_e309_d_n4), multiplicity * (eq21_e309_d_n5), multiplicity * (eq21_e309_d_n6), multiplicity * (eq21_e309_d_n7), multiplicity * (eq21_e309_d_n8), multiplicity * (eq21_e309_d_n9), multiplicity * (eq21_e309_d_n10)],
            [],
            [],
            1.0,
        );
        let eq22_e314: f64 = (var_gmin * var_vb1c4);
        let eq22_e314_d_n5: f64 = (var_gmin * var_vb1c4_dn5);
        let eq22_e314_d_n6: f64 = (var_gmin * var_vb1c4_dn6);
        let eq22_e314_d_n7: f64 = (var_gmin * var_vb1c4_dn7);
        let eq22_e314_d_n8: f64 = (var_gmin * var_vb1c4_dn8);
        let eq22_e314_d_n10: f64 = (var_gmin * var_vb1c4_dn10);
        let eq22_e315: f64 = (var_ib3 + eq22_e314);
        let eq22_e315_d_n5: f64 = (var_ib3_dn5 + eq22_e314_d_n5);
        let eq22_e315_d_n6: f64 = (var_ib3_dn6 + eq22_e314_d_n6);
        let eq22_e315_d_n7: f64 = (var_ib3_dn7 + eq22_e314_d_n7);
        let eq22_e315_d_n8: f64 = (var_ib3_dn8 + eq22_e314_d_n8);
        let eq22_e315_d_n10: f64 = (var_ib3_dn10 + eq22_e314_d_n10);
        let eq22_e317: f64 = (eq22_e315 + var_iex);
        let eq22_e317_d_n0: f64 = (var_ib3_dn0 + var_iex_dn0);
        let eq22_e317_d_n1: f64 = (var_ib3_dn1 + var_iex_dn1);
        let eq22_e317_d_n3: f64 = (var_ib3_dn3 + var_iex_dn3);
        let eq22_e317_d_n4: f64 = (var_ib3_dn4 + var_iex_dn4);
        let eq22_e317_d_n5: f64 = (eq22_e315_d_n5 + var_iex_dn5);
        let eq22_e317_d_n6: f64 = (eq22_e315_d_n6 + var_iex_dn6);
        let eq22_e317_d_n7: f64 = (eq22_e315_d_n7 + var_iex_dn7);
        let eq22_e317_d_n8: f64 = (eq22_e315_d_n8 + var_iex_dn8);
        let eq22_e317_d_n9: f64 = (var_ib3_dn9 + var_iex_dn9);
        let eq22_e317_d_n10: f64 = (eq22_e315_d_n10 + var_iex_dn10);
        let eq22_e318: f64 = (p.p3 * eq22_e317);
        let eq22_e318_d_n0: f64 = (p.p3 * eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = (p.p3 * eq22_e317_d_n1);
        let eq22_e318_d_n3: f64 = (p.p3 * eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = (p.p3 * eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = (p.p3 * eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = (p.p3 * eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = (p.p3 * eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = (p.p3 * eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = (p.p3 * eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = (p.p3 * eq22_e317_d_n10);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e318_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e318_d_n1 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e318_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e318_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e318_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e318_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e318_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e318_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e318_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e318_d_n10 * p.p1);
        let eq22_value: f64 = eq22_e320;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq22_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq22_e320_d_n0), multiplicity * (eq22_e320_d_n1), multiplicity * (eq22_e320_d_n3), multiplicity * (eq22_e320_d_n4), multiplicity * (eq22_e320_d_n5), multiplicity * (eq22_e320_d_n6), multiplicity * (eq22_e320_d_n7), multiplicity * (eq22_e320_d_n8), multiplicity * (eq22_e320_d_n9), multiplicity * (eq22_e320_d_n10)],
            [],
            [],
            1.0,
        );
        let eq23_e324: f64 = (var_qtex + var_qex);
        let eq23_e324_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq23_e324_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq23_e324_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq23_e324_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq23_e324_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq23_e324_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq23_e324_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq23_e324_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq23_e324_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq23_e324_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e325_d_n3 * ddt_scale) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e325_d_n4 * ddt_scale) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e325_d_n5 * ddt_scale) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e325_d_n6 * ddt_scale) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e325_d_n7 * ddt_scale) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e325_d_n8 * ddt_scale) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e325_d_n9 * ddt_scale) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e325_d_n10 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq23_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq23_e328_d_n0), multiplicity * (eq23_e328_d_n1), multiplicity * (eq23_e328_d_n3), multiplicity * (eq23_e328_d_n4), multiplicity * (eq23_e328_d_n5), multiplicity * (eq23_e328_d_n6), multiplicity * (eq23_e328_d_n7), multiplicity * (eq23_e328_d_n8), multiplicity * (eq23_e328_d_n9), multiplicity * (eq23_e328_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq24_e338, eq24_e338_d_n3, eq24_e338_d_n9, eq24_e338_d_n10,) = {
    if (var_guard121 != 0.0) {
        let eq24_e332: f64 = (p.p3 * var_vc3c4);
        let eq24_e332_d_n9: f64 = (p.p3 * var_vc3c4_dn9);
        let eq24_e332_d_n10: f64 = (p.p3 * var_vc3c4_dn10);
        let eq24_e334: f64 = (eq24_e332 * var_gcc_ex_t);
        let eq24_e334_d_n3: f64 = (eq24_e332 * var_gcc_ex_t_dn3);
        let eq24_e334_d_n9: f64 = (eq24_e332_d_n9 * var_gcc_ex_t);
        let eq24_e334_d_n10: f64 = (eq24_e332_d_n10 * var_gcc_ex_t);
        let eq24_e336: f64 = (eq24_e334 * p.p1);
        let eq24_e336_d_n3: f64 = (eq24_e334_d_n3 * p.p1);
        let eq24_e336_d_n9: f64 = (eq24_e334_d_n9 * p.p1);
        let eq24_e336_d_n10: f64 = (eq24_e334_d_n10 * p.p1);
        (eq24_e336, eq24_e336_d_n3, eq24_e336_d_n9, eq24_e336_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e338;
        stamper.stamp_current_node3_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            3,
            multiplicity * (eq24_e338_d_n3),
            9,
            multiplicity * (eq24_e338_d_n9),
            10,
            multiplicity * (eq24_e338_d_n10),
        );
        let (eq26_e353, eq26_e353_d_n3, eq26_e353_d_n7, eq26_e353_d_n10,) = {
    if (var_guard122 != 0.0) {
        let eq26_e347: f64 = (p.p3 * var_vc4c1);
        let eq26_e347_d_n7: f64 = (p.p3 * var_vc4c1_dn7);
        let eq26_e347_d_n10: f64 = (p.p3 * var_vc4c1_dn10);
        let eq26_e349: f64 = (eq26_e347 * var_gcc_in_t);
        let eq26_e349_d_n3: f64 = (eq26_e347 * var_gcc_in_t_dn3);
        let eq26_e349_d_n7: f64 = (eq26_e347_d_n7 * var_gcc_in_t);
        let eq26_e349_d_n10: f64 = (eq26_e347_d_n10 * var_gcc_in_t);
        let eq26_e351: f64 = (eq26_e349 * p.p1);
        let eq26_e351_d_n3: f64 = (eq26_e349_d_n3 * p.p1);
        let eq26_e351_d_n7: f64 = (eq26_e349_d_n7 * p.p1);
        let eq26_e351_d_n10: f64 = (eq26_e349_d_n10 * p.p1);
        (eq26_e351, eq26_e351_d_n3, eq26_e351_d_n7, eq26_e351_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e353;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(7),
            multiplicity * (eq26_value),
            3,
            multiplicity * (eq26_e353_d_n3),
            7,
            multiplicity * (eq26_e353_d_n7),
            10,
            multiplicity * (eq26_e353_d_n10),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gem_n: f64,
        var_gem_n_dn0: f64,
        var_gem_n_dn1: f64,
        var_gem_n_dn10: f64,
        var_gem_n_dn3: f64,
        var_gem_n_dn4: f64,
        var_gem_n_dn5: f64,
        var_gem_n_dn6: f64,
        var_gem_n_dn7: f64,
        var_gem_n_dn8: f64,
        var_gem_n_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (var_taun * eq30_e367);
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * eq30_e367);
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * eq30_e367);
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * eq30_e367);
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * eq30_e367);
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * eq30_e367);
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * eq30_e367);
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * eq30_e367);
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * eq30_e367);
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * eq30_e367);
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq30_node_derivatives: [f64; 11] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, (var_taun * ddt_scale)];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e371: f64 = (var_gem_n * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (var_gem_n_dn0 * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (var_gem_n_dn1 * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (var_gem_n_dn3 * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (var_gem_n_dn4 * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (var_gem_n_dn5 * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (var_gem_n_dn6 * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (var_gem_n_dn7 * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (var_gem_n_dn8 * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (var_gem_n_dn9 * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (var_gem_n_dn10 * (nv11 - 0.0));
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq31_node_derivatives: [f64; 11] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, var_gem_n];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_i_cth_rdn3: f64,
        var_i_cth_rv: f64,
        var_qb1b2: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn10: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn10: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vbc: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbe: f64,
        var_vbe_dn1: f64,
        var_vbe_dn2: f64,
        var_xqex: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn10: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq11_e235_q: f64 = var_i_cth_rv;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (var_i_cth_rdn3),
        );
        let eq13_e245: f64 = (var_qte + var_qbe);
        let eq13_e245_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq13_e245_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq13_e247: f64 = (eq13_e245 + var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + var_qe_dn1);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + var_qe_dn10);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, 0.0, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, 0.0];
        let eq13_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq14_e254_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, 0.0, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, 0.0];
        let eq14_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (var_qtc + var_qbc);
        let eq15_e261_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq15_e261_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq15_e263: f64 = (eq15_e261 + var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + var_qepi_dn1);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + var_qepi_dn10);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, 0.0, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq16_e270_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, 0.0, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, 0.0];
        let eq16_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * var_vbe);
        let eq17_e278_d_n1: f64 = (eq17_e276 * var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * var_vbe_dn2);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq17_e281_d_n1),
            nodes[2],
            multiplicity * (eq17_e281_d_n2),
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * var_vbc_dn1);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq18_e289_d_n0),
            nodes[1],
            multiplicity * (eq18_e289_d_n1),
        );
        let eq21_e305: f64 = (var_xqtex + var_xqex);
        let eq21_e305_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq21_e305_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, 0.0, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (var_qtex + var_qex);
        let eq23_e324_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq23_e324_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq23_e324_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq23_e324_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq23_e324_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq23_e324_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq23_e324_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq23_e324_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq23_e324_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq23_e324_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, 0.0, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (var_taun * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * (nv11 - 0.0));
        let eq30_e368_q: f64 = (var_taun * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (var_taun_dn0 * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (var_taun_dn1 * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (var_taun_dn3 * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (var_taun_dn4 * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (var_taun_dn5 * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (var_taun_dn6 * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (var_taun_dn7 * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (var_taun_dn8 * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (var_taun_dn9 * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (var_taun_dn10 * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, 0.0, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, var_taun];
        let eq30_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
