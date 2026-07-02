#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17580_e12090, assign17580_e12090_d_n0, assign17580_e12090_d_n2, assign17580_e12090_d_n4, assign17580_e12090_d_n5, assign17580_e12090_d_n6, assign17580_e12090_d_n7, assign17580_e12090_d_n8, assign17580_e12090_d_n9, assign17580_e12090_d_n10, assign17580_e12090_d_n11, assign17580_e12090_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17580_e12082: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_nin;
        let assign17580_e12084: f64 = (assign17580_e12082 * __rspice_inv_cse_0);
        let assign17580_e12086: f64 = (assign17580_e12084 * __rspice_inv_cse_0);
        let assign17580_e12087: f64 = (assign17580_e12086).ln();
        let assign17580_e12088: f64 = (locals.var_beta_inv * assign17580_e12087);
        (assign17580_e12088, ((locals.var_beta_inv_dn0 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn2 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn4 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn5 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn6 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn7 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn8 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn9 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn10 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn11 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))), ((locals.var_beta_inv_dn14 * assign17580_e12087) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign17580_e12082 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17580_e12084 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17580_e12086))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17580_e12090;
        locals.var_vbipn_dn0 = assign17580_e12090_d_n0;
        locals.var_vbipn_dn2 = assign17580_e12090_d_n2;
        locals.var_vbipn_dn4 = assign17580_e12090_d_n4;
        locals.var_vbipn_dn5 = assign17580_e12090_d_n5;
        locals.var_vbipn_dn6 = assign17580_e12090_d_n6;
        locals.var_vbipn_dn7 = assign17580_e12090_d_n7;
        locals.var_vbipn_dn8 = assign17580_e12090_d_n8;
        locals.var_vbipn_dn9 = assign17580_e12090_d_n9;
        locals.var_vbipn_dn10 = assign17580_e12090_d_n10;
        locals.var_vbipn_dn11 = assign17580_e12090_d_n11;
        locals.var_vbipn_dn14 = assign17580_e12090_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign17590_e12102, assign17590_e12102_d_n0, assign17590_e12102_d_n2, assign17590_e12102_d_n4, assign17590_e12102_d_n5, assign17590_e12102_d_n6, assign17590_e12102_d_n7, assign17590_e12102_d_n8, assign17590_e12102_d_n9, assign17590_e12102_d_n10, assign17590_e12102_d_n11, assign17590_e12102_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17590_e12099: f64 = (locals.var_log_tratio * p.p380);
        let assign17590_e12100: f64 = (assign17590_e12099).exp();
        (assign17590_e12100, (assign17590_e12100 * (locals.var_log_tratio_dn0 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn2 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn4 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn5 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn6 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn7 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn8 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn9 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn10 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn11 * p.p380)), (assign17590_e12100 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17590_e12102;
        locals.var_t1_dn0 = assign17590_e12102_d_n0;
        locals.var_t1_dn2 = assign17590_e12102_d_n2;
        locals.var_t1_dn4 = assign17590_e12102_d_n4;
        locals.var_t1_dn5 = assign17590_e12102_d_n5;
        locals.var_t1_dn6 = assign17590_e12102_d_n6;
        locals.var_t1_dn7 = assign17590_e12102_d_n7;
        locals.var_t1_dn8 = assign17590_e12102_d_n8;
        locals.var_t1_dn9 = assign17590_e12102_d_n9;
        locals.var_t1_dn10 = assign17590_e12102_d_n10;
        locals.var_t1_dn11 = assign17590_e12102_d_n11;
        locals.var_t1_dn14 = assign17590_e12102_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17600_e12113, assign17600_e12113_d_n0, assign17600_e12113_d_n2, assign17600_e12113_d_n4, assign17600_e12113_d_n5, assign17600_e12113_d_n6, assign17600_e12113_d_n7, assign17600_e12113_d_n8, assign17600_e12113_d_n9, assign17600_e12113_d_n10, assign17600_e12113_d_n11, assign17600_e12113_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17600_e12111: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17600_e12111, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17600_e12113;
        locals.var_depmphn0_dn0 = assign17600_e12113_d_n0;
        locals.var_depmphn0_dn2 = assign17600_e12113_d_n2;
        locals.var_depmphn0_dn4 = assign17600_e12113_d_n4;
        locals.var_depmphn0_dn5 = assign17600_e12113_d_n5;
        locals.var_depmphn0_dn6 = assign17600_e12113_d_n6;
        locals.var_depmphn0_dn7 = assign17600_e12113_d_n7;
        locals.var_depmphn0_dn8 = assign17600_e12113_d_n8;
        locals.var_depmphn0_dn9 = assign17600_e12113_d_n9;
        locals.var_depmphn0_dn10 = assign17600_e12113_d_n10;
        locals.var_depmphn0_dn11 = assign17600_e12113_d_n11;
        locals.var_depmphn0_dn14 = assign17600_e12113_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign17610_e12138, assign17610_e12138_d_n0, assign17610_e12138_d_n2, assign17610_e12138_d_n4, assign17610_e12138_d_n5, assign17610_e12138_d_n6, assign17610_e12138_d_n7, assign17610_e12138_d_n8, assign17610_e12138_d_n9, assign17610_e12138_d_n10, assign17610_e12138_d_n11, assign17610_e12138_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17610_e12123: f64 = (0.4 * locals.var_tratio);
        let assign17610_e12124: f64 = (1.8 + assign17610_e12123);
        let assign17610_e12127: f64 = (0.1 * locals.var_tratio);
        let assign17610_e12129: f64 = (assign17610_e12127 * locals.var_tratio);
        let assign17610_e12130: f64 = (assign17610_e12124 + assign17610_e12129);
        let assign17610_e12134: f64 = (1.0 - locals.var_tratio);
        let assign17610_e12135: f64 = (p.p379 * assign17610_e12134);
        let assign17610_e12136: f64 = (assign17610_e12130 - assign17610_e12135);
        (assign17610_e12136, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign17610_e12127 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17610_e12138;
        locals.var_t0_dn0 = assign17610_e12138_d_n0;
        locals.var_t0_dn2 = assign17610_e12138_d_n2;
        locals.var_t0_dn4 = assign17610_e12138_d_n4;
        locals.var_t0_dn5 = assign17610_e12138_d_n5;
        locals.var_t0_dn6 = assign17610_e12138_d_n6;
        locals.var_t0_dn7 = assign17610_e12138_d_n7;
        locals.var_t0_dn8 = assign17610_e12138_d_n8;
        locals.var_t0_dn9 = assign17610_e12138_d_n9;
        locals.var_t0_dn10 = assign17610_e12138_d_n10;
        locals.var_t0_dn11 = assign17610_e12138_d_n11;
        locals.var_t0_dn14 = assign17610_e12138_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign17620_e12149, assign17620_e12149_d_n0, assign17620_e12149_d_n2, assign17620_e12149_d_n4, assign17620_e12149_d_n5, assign17620_e12149_d_n6, assign17620_e12149_d_n7, assign17620_e12149_d_n8, assign17620_e12149_d_n9, assign17620_e12149_d_n10, assign17620_e12149_d_n11, assign17620_e12149_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17620_e12147: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17620_e12147, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17620_e12149;
        locals.var_uc_depvmax_dn0 = assign17620_e12149_d_n0;
        locals.var_uc_depvmax_dn2 = assign17620_e12149_d_n2;
        locals.var_uc_depvmax_dn4 = assign17620_e12149_d_n4;
        locals.var_uc_depvmax_dn5 = assign17620_e12149_d_n5;
        locals.var_uc_depvmax_dn6 = assign17620_e12149_d_n6;
        locals.var_uc_depvmax_dn7 = assign17620_e12149_d_n7;
        locals.var_uc_depvmax_dn8 = assign17620_e12149_d_n8;
        locals.var_uc_depvmax_dn9 = assign17620_e12149_d_n9;
        locals.var_uc_depvmax_dn10 = assign17620_e12149_d_n10;
        locals.var_uc_depvmax_dn11 = assign17620_e12149_d_n11;
        locals.var_uc_depvmax_dn14 = assign17620_e12149_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17640_e12157: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard364 = assign17640_e12157;
        locals.var_guard364_rv = 0.0;

        let (assign17650_e12168, assign17650_e12168_d_n0, assign17650_e12168_d_n2, assign17650_e12168_d_n4, assign17650_e12168_d_n5, assign17650_e12168_d_n6, assign17650_e12168_d_n7, assign17650_e12168_d_n8, assign17650_e12168_d_n9, assign17650_e12168_d_n10, assign17650_e12168_d_n11, assign17650_e12168_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) && (locals.var_guard364 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign17650_e12168;
        locals.var_uc_depvmax_dn0 = assign17650_e12168_d_n0;
        locals.var_uc_depvmax_dn2 = assign17650_e12168_d_n2;
        locals.var_uc_depvmax_dn4 = assign17650_e12168_d_n4;
        locals.var_uc_depvmax_dn5 = assign17650_e12168_d_n5;
        locals.var_uc_depvmax_dn6 = assign17650_e12168_d_n6;
        locals.var_uc_depvmax_dn7 = assign17650_e12168_d_n7;
        locals.var_uc_depvmax_dn8 = assign17650_e12168_d_n8;
        locals.var_uc_depvmax_dn9 = assign17650_e12168_d_n9;
        locals.var_uc_depvmax_dn10 = assign17650_e12168_d_n10;
        locals.var_uc_depvmax_dn11 = assign17650_e12168_d_n11;
        locals.var_uc_depvmax_dn14 = assign17650_e12168_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17660_e12181, assign17660_e12181_d_n0, assign17660_e12181_d_n2, assign17660_e12181_d_n4, assign17660_e12181_d_n5, assign17660_e12181_d_n6, assign17660_e12181_d_n7, assign17660_e12181_d_n8, assign17660_e12181_d_n9, assign17660_e12181_d_n10, assign17660_e12181_d_n11, assign17660_e12181_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17660_e12178: f64 = (locals.var_tratio).powf(p.p381);
        let assign17660_e12179: f64 = (locals.var_uc_depmue0 / assign17660_e12178);
        (assign17660_e12179, (((locals.var_uc_depmue0_dn0 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn2 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn4 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn5 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn6 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn7 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn8 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn9 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn10 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn11 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)), (((locals.var_uc_depmue0_dn14 * assign17660_e12178) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign17660_e12178 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign17660_e12178 * assign17660_e12178)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign17660_e12181;
        locals.var_uc_depmue0_dn0 = assign17660_e12181_d_n0;
        locals.var_uc_depmue0_dn2 = assign17660_e12181_d_n2;
        locals.var_uc_depmue0_dn4 = assign17660_e12181_d_n4;
        locals.var_uc_depmue0_dn5 = assign17660_e12181_d_n5;
        locals.var_uc_depmue0_dn6 = assign17660_e12181_d_n6;
        locals.var_uc_depmue0_dn7 = assign17660_e12181_d_n7;
        locals.var_uc_depmue0_dn8 = assign17660_e12181_d_n8;
        locals.var_uc_depmue0_dn9 = assign17660_e12181_d_n9;
        locals.var_uc_depmue0_dn10 = assign17660_e12181_d_n10;
        locals.var_uc_depmue0_dn11 = assign17660_e12181_d_n11;
        locals.var_uc_depmue0_dn14 = assign17660_e12181_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17670_e12196, assign17670_e12196_d_n0, assign17670_e12196_d_n2, assign17670_e12196_d_n4, assign17670_e12196_d_n5, assign17670_e12196_d_n6, assign17670_e12196_d_n7, assign17670_e12196_d_n8, assign17670_e12196_d_n9, assign17670_e12196_d_n10, assign17670_e12196_d_n11, assign17670_e12196_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 != 0.0)) {
        let assign17670_e12192: f64 = (locals.var_tratio - 1.0);
        let assign17670_e12193: f64 = (p.p365 * assign17670_e12192);
        let assign17670_e12194: f64 = (p.p364 + assign17670_e12193);
        (assign17670_e12194, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn11), (p.p365 * locals.var_tratio_dn14),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn11, locals.var_uc_depwlp_dn14,)
    }
};
        locals.var_uc_depwlp = assign17670_e12196;
        locals.var_uc_depwlp_dn0 = assign17670_e12196_d_n0;
        locals.var_uc_depwlp_dn2 = assign17670_e12196_d_n2;
        locals.var_uc_depwlp_dn4 = assign17670_e12196_d_n4;
        locals.var_uc_depwlp_dn5 = assign17670_e12196_d_n5;
        locals.var_uc_depwlp_dn6 = assign17670_e12196_d_n6;
        locals.var_uc_depwlp_dn7 = assign17670_e12196_d_n7;
        locals.var_uc_depwlp_dn8 = assign17670_e12196_d_n8;
        locals.var_uc_depwlp_dn9 = assign17670_e12196_d_n9;
        locals.var_uc_depwlp_dn10 = assign17670_e12196_d_n10;
        locals.var_uc_depwlp_dn11 = assign17670_e12196_d_n11;
        locals.var_uc_depwlp_dn14 = assign17670_e12196_d_n14;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign17680_e12206, assign17680_e12206_d_n0, assign17680_e12206_d_n2, assign17680_e12206_d_n4, assign17680_e12206_d_n5, assign17680_e12206_d_n6, assign17680_e12206_d_n7, assign17680_e12206_d_n8, assign17680_e12206_d_n9, assign17680_e12206_d_n10, assign17680_e12206_d_n11, assign17680_e12206_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign17680_e12206;
        locals.var_pb2n_dn0 = assign17680_e12206_d_n0;
        locals.var_pb2n_dn2 = assign17680_e12206_d_n2;
        locals.var_pb2n_dn4 = assign17680_e12206_d_n4;
        locals.var_pb2n_dn5 = assign17680_e12206_d_n5;
        locals.var_pb2n_dn6 = assign17680_e12206_d_n6;
        locals.var_pb2n_dn7 = assign17680_e12206_d_n7;
        locals.var_pb2n_dn8 = assign17680_e12206_d_n8;
        locals.var_pb2n_dn9 = assign17680_e12206_d_n9;
        locals.var_pb2n_dn10 = assign17680_e12206_d_n10;
        locals.var_pb2n_dn11 = assign17680_e12206_d_n11;
        locals.var_pb2n_dn14 = assign17680_e12206_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign17690_e12225, assign17690_e12225_d_n0, assign17690_e12225_d_n2, assign17690_e12225_d_n4, assign17690_e12225_d_n5, assign17690_e12225_d_n6, assign17690_e12225_d_n7, assign17690_e12225_d_n8, assign17690_e12225_d_n9, assign17690_e12225_d_n10, assign17690_e12225_d_n11, assign17690_e12225_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        let assign17690_e12217: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign17690_e12219: f64 = (assign17690_e12217 * locals.var_nsub);
        let assign17690_e12221: f64 = (assign17690_e12219 / locals.var_nin);
        let assign17690_e12222: f64 = (assign17690_e12221).ln();
        let assign17690_e12223: f64 = (locals.var_beta_inv * assign17690_e12222);
        (assign17690_e12223, ((locals.var_beta_inv_dn0 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn0)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn2 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn2)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn4 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn4)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn5 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn5)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn6 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn6)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn7 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn7)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn8 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn8)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn9 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn9)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn10 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn10)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn11 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn11)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))), ((locals.var_beta_inv_dn14 * assign17690_e12222) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17690_e12217 * locals.var_nsub_dn14)) * locals.var_nin) - (assign17690_e12219 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign17690_e12221))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign17690_e12225;
        locals.var_vbipn_dn0 = assign17690_e12225_d_n0;
        locals.var_vbipn_dn2 = assign17690_e12225_d_n2;
        locals.var_vbipn_dn4 = assign17690_e12225_d_n4;
        locals.var_vbipn_dn5 = assign17690_e12225_d_n5;
        locals.var_vbipn_dn6 = assign17690_e12225_d_n6;
        locals.var_vbipn_dn7 = assign17690_e12225_d_n7;
        locals.var_vbipn_dn8 = assign17690_e12225_d_n8;
        locals.var_vbipn_dn9 = assign17690_e12225_d_n9;
        locals.var_vbipn_dn10 = assign17690_e12225_d_n10;
        locals.var_vbipn_dn11 = assign17690_e12225_d_n11;
        locals.var_vbipn_dn14 = assign17690_e12225_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign17700_e12235, assign17700_e12235_d_n0, assign17700_e12235_d_n2, assign17700_e12235_d_n4, assign17700_e12235_d_n5, assign17700_e12235_d_n6, assign17700_e12235_d_n7, assign17700_e12235_d_n8, assign17700_e12235_d_n9, assign17700_e12235_d_n10, assign17700_e12235_d_n11, assign17700_e12235_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard359 == 0.0)) && (locals.var_guard362 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign17700_e12235;
        locals.var_depmphn0_dn0 = assign17700_e12235_d_n0;
        locals.var_depmphn0_dn2 = assign17700_e12235_d_n2;
        locals.var_depmphn0_dn4 = assign17700_e12235_d_n4;
        locals.var_depmphn0_dn5 = assign17700_e12235_d_n5;
        locals.var_depmphn0_dn6 = assign17700_e12235_d_n6;
        locals.var_depmphn0_dn7 = assign17700_e12235_d_n7;
        locals.var_depmphn0_dn8 = assign17700_e12235_d_n8;
        locals.var_depmphn0_dn9 = assign17700_e12235_d_n9;
        locals.var_depmphn0_dn10 = assign17700_e12235_d_n10;
        locals.var_depmphn0_dn11 = assign17700_e12235_d_n11;
        locals.var_depmphn0_dn14 = assign17700_e12235_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign17710_e12241, assign17710_e12241_d_n0, assign17710_e12241_d_n2, assign17710_e12241_d_n4, assign17710_e12241_d_n5, assign17710_e12241_d_n6, assign17710_e12241_d_n7, assign17710_e12241_d_n8, assign17710_e12241_d_n9, assign17710_e12241_d_n10, assign17710_e12241_d_n11, assign17710_e12241_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17710_e12239: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign17710_e12239, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn11 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn11)), ((locals.var_ptovr0_dn14 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn11, locals.var_ptovr_dn14,)
    }
};
        locals.var_ptovr = assign17710_e12241;
        locals.var_ptovr_dn0 = assign17710_e12241_d_n0;
        locals.var_ptovr_dn2 = assign17710_e12241_d_n2;
        locals.var_ptovr_dn4 = assign17710_e12241_d_n4;
        locals.var_ptovr_dn5 = assign17710_e12241_d_n5;
        locals.var_ptovr_dn6 = assign17710_e12241_d_n6;
        locals.var_ptovr_dn7 = assign17710_e12241_d_n7;
        locals.var_ptovr_dn8 = assign17710_e12241_d_n8;
        locals.var_ptovr_dn9 = assign17710_e12241_d_n9;
        locals.var_ptovr_dn10 = assign17710_e12241_d_n10;
        locals.var_ptovr_dn11 = assign17710_e12241_d_n11;
        locals.var_ptovr_dn14 = assign17710_e12241_d_n14;
        locals.var_ptovr_rv = 0.0;

        let (assign17720_e12247, assign17720_e12247_d_n0, assign17720_e12247_d_n2, assign17720_e12247_d_n4, assign17720_e12247_d_n5, assign17720_e12247_d_n6, assign17720_e12247_d_n7, assign17720_e12247_d_n8, assign17720_e12247_d_n9, assign17720_e12247_d_n10, assign17720_e12247_d_n11, assign17720_e12247_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17720_e12245: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17720_e12245, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17720_e12247;
        locals.var_t1_dn0 = assign17720_e12247_d_n0;
        locals.var_t1_dn2 = assign17720_e12247_d_n2;
        locals.var_t1_dn4 = assign17720_e12247_d_n4;
        locals.var_t1_dn5 = assign17720_e12247_d_n5;
        locals.var_t1_dn6 = assign17720_e12247_d_n6;
        locals.var_t1_dn7 = assign17720_e12247_d_n7;
        locals.var_t1_dn8 = assign17720_e12247_d_n8;
        locals.var_t1_dn9 = assign17720_e12247_d_n9;
        locals.var_t1_dn10 = assign17720_e12247_d_n10;
        locals.var_t1_dn11 = assign17720_e12247_d_n11;
        locals.var_t1_dn14 = assign17720_e12247_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17730_e12267, assign17730_e12267_d_n0, assign17730_e12267_d_n2, assign17730_e12267_d_n4, assign17730_e12267_d_n5, assign17730_e12267_d_n6, assign17730_e12267_d_n7, assign17730_e12267_d_n8, assign17730_e12267_d_n9, assign17730_e12267_d_n10, assign17730_e12267_d_n11, assign17730_e12267_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign17730_e12252: f64 = (0.4 * locals.var_t1);
        let assign17730_e12253: f64 = (1.8 + assign17730_e12252);
        let assign17730_e12256: f64 = (0.1 * locals.var_t1);
        let assign17730_e12258: f64 = (assign17730_e12256 * locals.var_t1);
        let assign17730_e12259: f64 = (assign17730_e12253 + assign17730_e12258);
        let assign17730_e12263: f64 = (1.0 - locals.var_t1);
        let assign17730_e12264: f64 = (locals.var_uc_vtmp * assign17730_e12263);
        let assign17730_e12265: f64 = (assign17730_e12259 - assign17730_e12264);
        (assign17730_e12265, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn11))) - (locals.var_uc_vtmp * (-locals.var_t1_dn11))), (((0.4 * locals.var_t1_dn14) + (((0.1 * locals.var_t1_dn14) * locals.var_t1) + (assign17730_e12256 * locals.var_t1_dn14))) - (locals.var_uc_vtmp * (-locals.var_t1_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17730_e12267;
        locals.var_t0_dn0 = assign17730_e12267_d_n0;
        locals.var_t0_dn2 = assign17730_e12267_d_n2;
        locals.var_t0_dn4 = assign17730_e12267_d_n4;
        locals.var_t0_dn5 = assign17730_e12267_d_n5;
        locals.var_t0_dn6 = assign17730_e12267_d_n6;
        locals.var_t0_dn7 = assign17730_e12267_d_n7;
        locals.var_t0_dn8 = assign17730_e12267_d_n8;
        locals.var_t0_dn9 = assign17730_e12267_d_n9;
        locals.var_t0_dn10 = assign17730_e12267_d_n10;
        locals.var_t0_dn11 = assign17730_e12267_d_n11;
        locals.var_t0_dn14 = assign17730_e12267_d_n14;
        locals.var_t0_rv = 0.0;

        let assign17740_e12270: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign17740_e12270;
        locals.var_guard365_rv = 0.0;

        let (assign17750_e12290, assign17750_e12290_d_n0, assign17750_e12290_d_n2, assign17750_e12290_d_n4, assign17750_e12290_d_n5, assign17750_e12290_d_n6, assign17750_e12290_d_n7, assign17750_e12290_d_n8, assign17750_e12290_d_n9, assign17750_e12290_d_n10, assign17750_e12290_d_n11, assign17750_e12290_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard365 != 0.0)) {
        let assign17750_e12276: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17750_e12278: f64 = (assign17750_e12276 / locals.var_t0);
        let assign17750_e12282: f64 = (p.p90 * locals.var_tdiff0);
        let assign17750_e12283: f64 = (1.0 + assign17750_e12282);
        let assign17750_e12286: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign17750_e12287: f64 = (assign17750_e12283 + assign17750_e12286);
        let assign17750_e12288: f64 = (assign17750_e12278 * assign17750_e12287);
        (assign17750_e12288, (((-((assign17750_e12276 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign17750_e12276 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign17750_e12276 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign17750_e12276 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign17750_e12276 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign17750_e12276 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign17750_e12276 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign17750_e12276 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign17750_e12276 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign17750_e12276 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn11) + (p.p91 * locals.var_tdiff0_2_dn11)))), (((-((assign17750_e12276 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign17750_e12287) + (assign17750_e12278 * ((p.p90 * locals.var_tdiff0_dn14) + (p.p91 * locals.var_tdiff0_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign17750_e12290;
        locals.var_vmaxeff_dn0 = assign17750_e12290_d_n0;
        locals.var_vmaxeff_dn2 = assign17750_e12290_d_n2;
        locals.var_vmaxeff_dn4 = assign17750_e12290_d_n4;
        locals.var_vmaxeff_dn5 = assign17750_e12290_d_n5;
        locals.var_vmaxeff_dn6 = assign17750_e12290_d_n6;
        locals.var_vmaxeff_dn7 = assign17750_e12290_d_n7;
        locals.var_vmaxeff_dn8 = assign17750_e12290_d_n8;
        locals.var_vmaxeff_dn9 = assign17750_e12290_d_n9;
        locals.var_vmaxeff_dn10 = assign17750_e12290_d_n10;
        locals.var_vmaxeff_dn11 = assign17750_e12290_d_n11;
        locals.var_vmaxeff_dn14 = assign17750_e12290_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let (assign17760_e12311, assign17760_e12311_d_n0, assign17760_e12311_d_n2, assign17760_e12311_d_n4, assign17760_e12311_d_n5, assign17760_e12311_d_n6, assign17760_e12311_d_n7, assign17760_e12311_d_n8, assign17760_e12311_d_n9, assign17760_e12311_d_n10, assign17760_e12311_d_n11, assign17760_e12311_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard365 == 0.0)) {
        let assign17760_e12297: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17760_e12299: f64 = (assign17760_e12297 / locals.var_t0);
        let assign17760_e12303: f64 = (p.p90 * locals.var_tdiff);
        let assign17760_e12304: f64 = (1.0 + assign17760_e12303);
        let assign17760_e12307: f64 = (p.p91 * locals.var_tdiff_2);
        let assign17760_e12308: f64 = (assign17760_e12304 + assign17760_e12307);
        let assign17760_e12309: f64 = (assign17760_e12299 * assign17760_e12308);
        (assign17760_e12309, (((-((assign17760_e12297 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign17760_e12297 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign17760_e12297 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign17760_e12297 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign17760_e12297 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign17760_e12297 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign17760_e12297 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign17760_e12297 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign17760_e12297 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign17760_e12297 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn11) + (p.p91 * locals.var_tdiff_2_dn11)))), (((-((assign17760_e12297 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign17760_e12308) + (assign17760_e12299 * ((p.p90 * locals.var_tdiff_dn14) + (p.p91 * locals.var_tdiff_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign17760_e12311;
        locals.var_vmaxeff_dn0 = assign17760_e12311_d_n0;
        locals.var_vmaxeff_dn2 = assign17760_e12311_d_n2;
        locals.var_vmaxeff_dn4 = assign17760_e12311_d_n4;
        locals.var_vmaxeff_dn5 = assign17760_e12311_d_n5;
        locals.var_vmaxeff_dn6 = assign17760_e12311_d_n6;
        locals.var_vmaxeff_dn7 = assign17760_e12311_d_n7;
        locals.var_vmaxeff_dn8 = assign17760_e12311_d_n8;
        locals.var_vmaxeff_dn9 = assign17760_e12311_d_n9;
        locals.var_vmaxeff_dn10 = assign17760_e12311_d_n10;
        locals.var_vmaxeff_dn11 = assign17760_e12311_d_n11;
        locals.var_vmaxeff_dn14 = assign17760_e12311_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let assign17780_e12319: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign17780_e12319;
        locals.var_guard367_rv = 0.0;

        let (assign17790_e12335, assign17790_e12335_d_n0, assign17790_e12335_d_n2, assign17790_e12335_d_n4, assign17790_e12335_d_n5, assign17790_e12335_d_n6, assign17790_e12335_d_n7, assign17790_e12335_d_n8, assign17790_e12335_d_n9, assign17790_e12335_d_n10, assign17790_e12335_d_n11, assign17790_e12335_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17790_e12327: f64 = (p.p324 * locals.var_tdiff0);
        let assign17790_e12328: f64 = (1.0 + assign17790_e12327);
        let assign17790_e12331: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign17790_e12332: f64 = (assign17790_e12328 + assign17790_e12331);
        let assign17790_e12333: f64 = (locals.var_ninvd0 * assign17790_e12332);
        (assign17790_e12333, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn11) + (p.p325 * locals.var_tdiff0_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn14) + (p.p325 * locals.var_tdiff0_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17790_e12335;
        locals.var_ninvde_dn0 = assign17790_e12335_d_n0;
        locals.var_ninvde_dn2 = assign17790_e12335_d_n2;
        locals.var_ninvde_dn4 = assign17790_e12335_d_n4;
        locals.var_ninvde_dn5 = assign17790_e12335_d_n5;
        locals.var_ninvde_dn6 = assign17790_e12335_d_n6;
        locals.var_ninvde_dn7 = assign17790_e12335_d_n7;
        locals.var_ninvde_dn8 = assign17790_e12335_d_n8;
        locals.var_ninvde_dn9 = assign17790_e12335_d_n9;
        locals.var_ninvde_dn10 = assign17790_e12335_d_n10;
        locals.var_ninvde_dn11 = assign17790_e12335_d_n11;
        locals.var_ninvde_dn14 = assign17790_e12335_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign17800_e12349, assign17800_e12349_d_n0, assign17800_e12349_d_n2, assign17800_e12349_d_n4, assign17800_e12349_d_n5, assign17800_e12349_d_n6, assign17800_e12349_d_n7, assign17800_e12349_d_n8, assign17800_e12349_d_n9, assign17800_e12349_d_n10, assign17800_e12349_d_n11, assign17800_e12349_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17800_e12342: f64 = (p.p390 * locals.var_tdiff0);
        let assign17800_e12343: f64 = (1.0 + assign17800_e12342);
        let assign17800_e12346: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign17800_e12347: f64 = (assign17800_e12343 + assign17800_e12346);
        (assign17800_e12347, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn11) + (p.p391 * locals.var_tdiff0_2_dn11)), ((p.p390 * locals.var_tdiff0_dn14) + (p.p391 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17800_e12349;
        locals.var_t1_dn0 = assign17800_e12349_d_n0;
        locals.var_t1_dn2 = assign17800_e12349_d_n2;
        locals.var_t1_dn4 = assign17800_e12349_d_n4;
        locals.var_t1_dn5 = assign17800_e12349_d_n5;
        locals.var_t1_dn6 = assign17800_e12349_d_n6;
        locals.var_t1_dn7 = assign17800_e12349_d_n7;
        locals.var_t1_dn8 = assign17800_e12349_d_n8;
        locals.var_t1_dn9 = assign17800_e12349_d_n9;
        locals.var_t1_dn10 = assign17800_e12349_d_n10;
        locals.var_t1_dn11 = assign17800_e12349_d_n11;
        locals.var_t1_dn14 = assign17800_e12349_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17810_e12357, assign17810_e12357_d_n0, assign17810_e12357_d_n2, assign17810_e12357_d_n4, assign17810_e12357_d_n5, assign17810_e12357_d_n6, assign17810_e12357_d_n7, assign17810_e12357_d_n8, assign17810_e12357_d_n9, assign17810_e12357_d_n10, assign17810_e12357_d_n11, assign17810_e12357_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17810_e12355: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17810_e12355, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17810_e12357;
        locals.var_ninvdecres_dn0 = assign17810_e12357_d_n0;
        locals.var_ninvdecres_dn2 = assign17810_e12357_d_n2;
        locals.var_ninvdecres_dn4 = assign17810_e12357_d_n4;
        locals.var_ninvdecres_dn5 = assign17810_e12357_d_n5;
        locals.var_ninvdecres_dn6 = assign17810_e12357_d_n6;
        locals.var_ninvdecres_dn7 = assign17810_e12357_d_n7;
        locals.var_ninvdecres_dn8 = assign17810_e12357_d_n8;
        locals.var_ninvdecres_dn9 = assign17810_e12357_d_n9;
        locals.var_ninvdecres_dn10 = assign17810_e12357_d_n10;
        locals.var_ninvdecres_dn11 = assign17810_e12357_d_n11;
        locals.var_ninvdecres_dn14 = assign17810_e12357_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17820_e12365, assign17820_e12365_d_n0, assign17820_e12365_d_n2, assign17820_e12365_d_n4, assign17820_e12365_d_n5, assign17820_e12365_d_n6, assign17820_e12365_d_n7, assign17820_e12365_d_n8, assign17820_e12365_d_n9, assign17820_e12365_d_n10, assign17820_e12365_d_n11, assign17820_e12365_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign17820_e12363: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17820_e12363, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17820_e12365;
        locals.var_ninvdehres_dn0 = assign17820_e12365_d_n0;
        locals.var_ninvdehres_dn2 = assign17820_e12365_d_n2;
        locals.var_ninvdehres_dn4 = assign17820_e12365_d_n4;
        locals.var_ninvdehres_dn5 = assign17820_e12365_d_n5;
        locals.var_ninvdehres_dn6 = assign17820_e12365_d_n6;
        locals.var_ninvdehres_dn7 = assign17820_e12365_d_n7;
        locals.var_ninvdehres_dn8 = assign17820_e12365_d_n8;
        locals.var_ninvdehres_dn9 = assign17820_e12365_d_n9;
        locals.var_ninvdehres_dn10 = assign17820_e12365_d_n10;
        locals.var_ninvdehres_dn11 = assign17820_e12365_d_n11;
        locals.var_ninvdehres_dn14 = assign17820_e12365_d_n14;
        locals.var_ninvdehres_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17830_e12382, assign17830_e12382_d_n0, assign17830_e12382_d_n2, assign17830_e12382_d_n4, assign17830_e12382_d_n5, assign17830_e12382_d_n6, assign17830_e12382_d_n7, assign17830_e12382_d_n8, assign17830_e12382_d_n9, assign17830_e12382_d_n10, assign17830_e12382_d_n11, assign17830_e12382_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17830_e12374: f64 = (p.p324 * locals.var_tdiff);
        let assign17830_e12375: f64 = (1.0 + assign17830_e12374);
        let assign17830_e12378: f64 = (p.p325 * locals.var_tdiff_2);
        let assign17830_e12379: f64 = (assign17830_e12375 + assign17830_e12378);
        let assign17830_e12380: f64 = (locals.var_ninvd0 * assign17830_e12379);
        (assign17830_e12380, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn11) + (p.p325 * locals.var_tdiff_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn14) + (p.p325 * locals.var_tdiff_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17830_e12382;
        locals.var_ninvde_dn0 = assign17830_e12382_d_n0;
        locals.var_ninvde_dn2 = assign17830_e12382_d_n2;
        locals.var_ninvde_dn4 = assign17830_e12382_d_n4;
        locals.var_ninvde_dn5 = assign17830_e12382_d_n5;
        locals.var_ninvde_dn6 = assign17830_e12382_d_n6;
        locals.var_ninvde_dn7 = assign17830_e12382_d_n7;
        locals.var_ninvde_dn8 = assign17830_e12382_d_n8;
        locals.var_ninvde_dn9 = assign17830_e12382_d_n9;
        locals.var_ninvde_dn10 = assign17830_e12382_d_n10;
        locals.var_ninvde_dn11 = assign17830_e12382_d_n11;
        locals.var_ninvde_dn14 = assign17830_e12382_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign17840_e12397, assign17840_e12397_d_n0, assign17840_e12397_d_n2, assign17840_e12397_d_n4, assign17840_e12397_d_n5, assign17840_e12397_d_n6, assign17840_e12397_d_n7, assign17840_e12397_d_n8, assign17840_e12397_d_n9, assign17840_e12397_d_n10, assign17840_e12397_d_n11, assign17840_e12397_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17840_e12390: f64 = (p.p390 * locals.var_tdiff);
        let assign17840_e12391: f64 = (1.0 + assign17840_e12390);
        let assign17840_e12394: f64 = (p.p391 * locals.var_tdiff_2);
        let assign17840_e12395: f64 = (assign17840_e12391 + assign17840_e12394);
        (assign17840_e12395, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn11) + (p.p391 * locals.var_tdiff_2_dn11)), ((p.p390 * locals.var_tdiff_dn14) + (p.p391 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign17840_e12397;
        locals.var_t1_dn0 = assign17840_e12397_d_n0;
        locals.var_t1_dn2 = assign17840_e12397_d_n2;
        locals.var_t1_dn4 = assign17840_e12397_d_n4;
        locals.var_t1_dn5 = assign17840_e12397_d_n5;
        locals.var_t1_dn6 = assign17840_e12397_d_n6;
        locals.var_t1_dn7 = assign17840_e12397_d_n7;
        locals.var_t1_dn8 = assign17840_e12397_d_n8;
        locals.var_t1_dn9 = assign17840_e12397_d_n9;
        locals.var_t1_dn10 = assign17840_e12397_d_n10;
        locals.var_t1_dn11 = assign17840_e12397_d_n11;
        locals.var_t1_dn14 = assign17840_e12397_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign17850_e12406, assign17850_e12406_d_n0, assign17850_e12406_d_n2, assign17850_e12406_d_n4, assign17850_e12406_d_n5, assign17850_e12406_d_n6, assign17850_e12406_d_n7, assign17850_e12406_d_n8, assign17850_e12406_d_n9, assign17850_e12406_d_n10, assign17850_e12406_d_n11, assign17850_e12406_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17850_e12404: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17850_e12404, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17850_e12406;
        locals.var_ninvdecres_dn0 = assign17850_e12406_d_n0;
        locals.var_ninvdecres_dn2 = assign17850_e12406_d_n2;
        locals.var_ninvdecres_dn4 = assign17850_e12406_d_n4;
        locals.var_ninvdecres_dn5 = assign17850_e12406_d_n5;
        locals.var_ninvdecres_dn6 = assign17850_e12406_d_n6;
        locals.var_ninvdecres_dn7 = assign17850_e12406_d_n7;
        locals.var_ninvdecres_dn8 = assign17850_e12406_d_n8;
        locals.var_ninvdecres_dn9 = assign17850_e12406_d_n9;
        locals.var_ninvdecres_dn10 = assign17850_e12406_d_n10;
        locals.var_ninvdecres_dn11 = assign17850_e12406_d_n11;
        locals.var_ninvdecres_dn14 = assign17850_e12406_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17860_e12415, assign17860_e12415_d_n0, assign17860_e12415_d_n2, assign17860_e12415_d_n4, assign17860_e12415_d_n5, assign17860_e12415_d_n6, assign17860_e12415_d_n7, assign17860_e12415_d_n8, assign17860_e12415_d_n9, assign17860_e12415_d_n10, assign17860_e12415_d_n11, assign17860_e12415_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign17860_e12413: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17860_e12413, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17860_e12415;
        locals.var_ninvdehres_dn0 = assign17860_e12415_d_n0;
        locals.var_ninvdehres_dn2 = assign17860_e12415_d_n2;
        locals.var_ninvdehres_dn4 = assign17860_e12415_d_n4;
        locals.var_ninvdehres_dn5 = assign17860_e12415_d_n5;
        locals.var_ninvdehres_dn6 = assign17860_e12415_d_n6;
        locals.var_ninvdehres_dn7 = assign17860_e12415_d_n7;
        locals.var_ninvdehres_dn8 = assign17860_e12415_d_n8;
        locals.var_ninvdehres_dn9 = assign17860_e12415_d_n9;
        locals.var_ninvdehres_dn10 = assign17860_e12415_d_n10;
        locals.var_ninvdehres_dn11 = assign17860_e12415_d_n11;
        locals.var_ninvdehres_dn14 = assign17860_e12415_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let assign17880_e12423: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign17880_e12423;
        locals.var_guard369_rv = 0.0;

        let (assign17890_e12429, assign17890_e12429_d_n0, assign17890_e12429_d_n2, assign17890_e12429_d_n4, assign17890_e12429_d_n5, assign17890_e12429_d_n6, assign17890_e12429_d_n7, assign17890_e12429_d_n8, assign17890_e12429_d_n9, assign17890_e12429_d_n10, assign17890_e12429_d_n11, assign17890_e12429_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign17890_e12429;
        locals.var_ninvde_dn0 = assign17890_e12429_d_n0;
        locals.var_ninvde_dn2 = assign17890_e12429_d_n2;
        locals.var_ninvde_dn4 = assign17890_e12429_d_n4;
        locals.var_ninvde_dn5 = assign17890_e12429_d_n5;
        locals.var_ninvde_dn6 = assign17890_e12429_d_n6;
        locals.var_ninvde_dn7 = assign17890_e12429_d_n7;
        locals.var_ninvde_dn8 = assign17890_e12429_d_n8;
        locals.var_ninvde_dn9 = assign17890_e12429_d_n9;
        locals.var_ninvde_dn10 = assign17890_e12429_d_n10;
        locals.var_ninvde_dn11 = assign17890_e12429_d_n11;
        locals.var_ninvde_dn14 = assign17890_e12429_d_n14;
        locals.var_ninvde_rv = 0.0;

        let assign17910_e12437: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign17910_e12437;
        locals.var_guard371_rv = 0.0;

        let (assign17920_e12443, assign17920_e12443_d_n0, assign17920_e12443_d_n2, assign17920_e12443_d_n4, assign17920_e12443_d_n5, assign17920_e12443_d_n6, assign17920_e12443_d_n7, assign17920_e12443_d_n8, assign17920_e12443_d_n9, assign17920_e12443_d_n10, assign17920_e12443_d_n11, assign17920_e12443_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard371 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign17920_e12443;
        locals.var_ninvdecres_dn0 = assign17920_e12443_d_n0;
        locals.var_ninvdecres_dn2 = assign17920_e12443_d_n2;
        locals.var_ninvdecres_dn4 = assign17920_e12443_d_n4;
        locals.var_ninvdecres_dn5 = assign17920_e12443_d_n5;
        locals.var_ninvdecres_dn6 = assign17920_e12443_d_n6;
        locals.var_ninvdecres_dn7 = assign17920_e12443_d_n7;
        locals.var_ninvdecres_dn8 = assign17920_e12443_d_n8;
        locals.var_ninvdecres_dn9 = assign17920_e12443_d_n9;
        locals.var_ninvdecres_dn10 = assign17920_e12443_d_n10;
        locals.var_ninvdecres_dn11 = assign17920_e12443_d_n11;
        locals.var_ninvdecres_dn14 = assign17920_e12443_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let assign17940_e12451: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign17940_e12451;
        locals.var_guard373_rv = 0.0;

        let (assign17950_e12457, assign17950_e12457_d_n0, assign17950_e12457_d_n2, assign17950_e12457_d_n4, assign17950_e12457_d_n5, assign17950_e12457_d_n6, assign17950_e12457_d_n7, assign17950_e12457_d_n8, assign17950_e12457_d_n9, assign17950_e12457_d_n10, assign17950_e12457_d_n11, assign17950_e12457_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard373 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign17950_e12457;
        locals.var_ninvdehres_dn0 = assign17950_e12457_d_n0;
        locals.var_ninvdehres_dn2 = assign17950_e12457_d_n2;
        locals.var_ninvdehres_dn4 = assign17950_e12457_d_n4;
        locals.var_ninvdehres_dn5 = assign17950_e12457_d_n5;
        locals.var_ninvdehres_dn6 = assign17950_e12457_d_n6;
        locals.var_ninvdehres_dn7 = assign17950_e12457_d_n7;
        locals.var_ninvdehres_dn8 = assign17950_e12457_d_n8;
        locals.var_ninvdehres_dn9 = assign17950_e12457_d_n9;
        locals.var_ninvdehres_dn10 = assign17950_e12457_d_n10;
        locals.var_ninvdehres_dn11 = assign17950_e12457_d_n11;
        locals.var_ninvdehres_dn14 = assign17950_e12457_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let (assign17960_e12473, assign17960_e12473_d_n0, assign17960_e12473_d_n2, assign17960_e12473_d_n4, assign17960_e12473_d_n5, assign17960_e12473_d_n6, assign17960_e12473_d_n7, assign17960_e12473_d_n8, assign17960_e12473_d_n9, assign17960_e12473_d_n10, assign17960_e12473_d_n11, assign17960_e12473_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (p.p53 != 0.0)) {
        let assign17960_e12464: f64 = (p.p328 * locals.var_tdiff0);
        let assign17960_e12465: f64 = (locals.var_uc_rth0 + assign17960_e12464);
        let assign17960_e12468: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign17960_e12469: f64 = (assign17960_e12465 + assign17960_e12468);
        let assign17960_e12471: f64 = (assign17960_e12469 * locals.var_rthtemp0);
        (assign17960_e12471, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn11) + (p.p329 * locals.var_tdiff0_2_dn11)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn14) + (p.p329 * locals.var_tdiff0_2_dn14)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign17960_e12473;
        locals.var_rth_dn0 = assign17960_e12473_d_n0;
        locals.var_rth_dn2 = assign17960_e12473_d_n2;
        locals.var_rth_dn4 = assign17960_e12473_d_n4;
        locals.var_rth_dn5 = assign17960_e12473_d_n5;
        locals.var_rth_dn6 = assign17960_e12473_d_n6;
        locals.var_rth_dn7 = assign17960_e12473_d_n7;
        locals.var_rth_dn8 = assign17960_e12473_d_n8;
        locals.var_rth_dn9 = assign17960_e12473_d_n9;
        locals.var_rth_dn10 = assign17960_e12473_d_n10;
        locals.var_rth_dn11 = assign17960_e12473_d_n11;
        locals.var_rth_dn14 = assign17960_e12473_d_n14;
        locals.var_rth_rv = 0.0;

        let assign17980_e12481: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign17980_e12481;
        locals.var_guard375_rv = 0.0;

        let (assign17990_e12489, assign17990_e12489_d_n0, assign17990_e12489_d_n2, assign17990_e12489_d_n4, assign17990_e12489_d_n5, assign17990_e12489_d_n6, assign17990_e12489_d_n7, assign17990_e12489_d_n8, assign17990_e12489_d_n9, assign17990_e12489_d_n10, assign17990_e12489_d_n11, assign17990_e12489_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard375 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign17990_e12489;
        locals.var_rth_dn0 = assign17990_e12489_d_n0;
        locals.var_rth_dn2 = assign17990_e12489_d_n2;
        locals.var_rth_dn4 = assign17990_e12489_d_n4;
        locals.var_rth_dn5 = assign17990_e12489_d_n5;
        locals.var_rth_dn6 = assign17990_e12489_d_n6;
        locals.var_rth_dn7 = assign17990_e12489_d_n7;
        locals.var_rth_dn8 = assign17990_e12489_d_n8;
        locals.var_rth_dn9 = assign17990_e12489_d_n9;
        locals.var_rth_dn10 = assign17990_e12489_d_n10;
        locals.var_rth_dn11 = assign17990_e12489_d_n11;
        locals.var_rth_dn14 = assign17990_e12489_d_n14;
        locals.var_rth_rv = 0.0;

        let (assign18000_e12501, assign18000_e12501_d_n0, assign18000_e12501_d_n2, assign18000_e12501_d_n4, assign18000_e12501_d_n5, assign18000_e12501_d_n6, assign18000_e12501_d_n7, assign18000_e12501_d_n8, assign18000_e12501_d_n9, assign18000_e12501_d_n10, assign18000_e12501_d_n11, assign18000_e12501_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18000_e12494: f64 = (p.p330 * locals.var_tdiff0);
        let assign18000_e12495: f64 = (locals.var_uc_powrat + assign18000_e12494);
        let assign18000_e12498: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign18000_e12499: f64 = (assign18000_e12495 + assign18000_e12498);
        (assign18000_e12499, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn11) + (p.p331 * locals.var_tdiff0_2_dn11)), ((p.p330 * locals.var_tdiff0_dn14) + (p.p331 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18000_e12501;
        locals.var_t2_dn0 = assign18000_e12501_d_n0;
        locals.var_t2_dn2 = assign18000_e12501_d_n2;
        locals.var_t2_dn4 = assign18000_e12501_d_n4;
        locals.var_t2_dn5 = assign18000_e12501_d_n5;
        locals.var_t2_dn6 = assign18000_e12501_d_n6;
        locals.var_t2_dn7 = assign18000_e12501_d_n7;
        locals.var_t2_dn8 = assign18000_e12501_d_n8;
        locals.var_t2_dn9 = assign18000_e12501_d_n9;
        locals.var_t2_dn10 = assign18000_e12501_d_n10;
        locals.var_t2_dn11 = assign18000_e12501_d_n11;
        locals.var_t2_dn14 = assign18000_e12501_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign18010_e12509, assign18010_e12509_d_n0, assign18010_e12509_d_n2, assign18010_e12509_d_n4, assign18010_e12509_d_n5, assign18010_e12509_d_n6, assign18010_e12509_d_n7, assign18010_e12509_d_n8, assign18010_e12509_d_n9, assign18010_e12509_d_n10, assign18010_e12509_d_n11, assign18010_e12509_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18010_e12505: f64 = locals.var_t2;
        let assign18010_e12507: f64 = (assign18010_e12505 - 0.05);
        (assign18010_e12507, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18010_e12509;
        locals.var_tmf1_dn0 = assign18010_e12509_d_n0;
        locals.var_tmf1_dn2 = assign18010_e12509_d_n2;
        locals.var_tmf1_dn4 = assign18010_e12509_d_n4;
        locals.var_tmf1_dn5 = assign18010_e12509_d_n5;
        locals.var_tmf1_dn6 = assign18010_e12509_d_n6;
        locals.var_tmf1_dn7 = assign18010_e12509_d_n7;
        locals.var_tmf1_dn8 = assign18010_e12509_d_n8;
        locals.var_tmf1_dn9 = assign18010_e12509_d_n9;
        locals.var_tmf1_dn10 = assign18010_e12509_d_n10;
        locals.var_tmf1_dn11 = assign18010_e12509_d_n11;
        locals.var_tmf1_dn14 = assign18010_e12509_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18020_e12517, assign18020_e12517_d_n0, assign18020_e12517_d_n2, assign18020_e12517_d_n4, assign18020_e12517_d_n5, assign18020_e12517_d_n6, assign18020_e12517_d_n7, assign18020_e12517_d_n8, assign18020_e12517_d_n9, assign18020_e12517_d_n10, assign18020_e12517_d_n11, assign18020_e12517_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18020_e12517;
        locals.var_tmf2_dn0 = assign18020_e12517_d_n0;
        locals.var_tmf2_dn2 = assign18020_e12517_d_n2;
        locals.var_tmf2_dn4 = assign18020_e12517_d_n4;
        locals.var_tmf2_dn5 = assign18020_e12517_d_n5;
        locals.var_tmf2_dn6 = assign18020_e12517_d_n6;
        locals.var_tmf2_dn7 = assign18020_e12517_d_n7;
        locals.var_tmf2_dn8 = assign18020_e12517_d_n8;
        locals.var_tmf2_dn9 = assign18020_e12517_d_n9;
        locals.var_tmf2_dn10 = assign18020_e12517_d_n10;
        locals.var_tmf2_dn11 = assign18020_e12517_d_n11;
        locals.var_tmf2_dn14 = assign18020_e12517_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18030_e12527, assign18030_e12527_d_n0, assign18030_e12527_d_n2, assign18030_e12527_d_n4, assign18030_e12527_d_n5, assign18030_e12527_d_n6, assign18030_e12527_d_n7, assign18030_e12527_d_n8, assign18030_e12527_d_n9, assign18030_e12527_d_n10, assign18030_e12527_d_n11, assign18030_e12527_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let (assign18030_e12525, assign18030_e12525_d_n0, assign18030_e12525_d_n2, assign18030_e12525_d_n4, assign18030_e12525_d_n5, assign18030_e12525_d_n6, assign18030_e12525_d_n7, assign18030_e12525_d_n8, assign18030_e12525_d_n9, assign18030_e12525_d_n10, assign18030_e12525_d_n11, assign18030_e12525_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18030_e12524: f64 = (-locals.var_tmf2);
                (assign18030_e12524, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18030_e12525, assign18030_e12525_d_n0, assign18030_e12525_d_n2, assign18030_e12525_d_n4, assign18030_e12525_d_n5, assign18030_e12525_d_n6, assign18030_e12525_d_n7, assign18030_e12525_d_n8, assign18030_e12525_d_n9, assign18030_e12525_d_n10, assign18030_e12525_d_n11, assign18030_e12525_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18030_e12527;
        locals.var_tmf2_dn0 = assign18030_e12527_d_n0;
        locals.var_tmf2_dn2 = assign18030_e12527_d_n2;
        locals.var_tmf2_dn4 = assign18030_e12527_d_n4;
        locals.var_tmf2_dn5 = assign18030_e12527_d_n5;
        locals.var_tmf2_dn6 = assign18030_e12527_d_n6;
        locals.var_tmf2_dn7 = assign18030_e12527_d_n7;
        locals.var_tmf2_dn8 = assign18030_e12527_d_n8;
        locals.var_tmf2_dn9 = assign18030_e12527_d_n9;
        locals.var_tmf2_dn10 = assign18030_e12527_d_n10;
        locals.var_tmf2_dn11 = assign18030_e12527_d_n11;
        locals.var_tmf2_dn14 = assign18030_e12527_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18040_e12536, assign18040_e12536_d_n0, assign18040_e12536_d_n2, assign18040_e12536_d_n4, assign18040_e12536_d_n5, assign18040_e12536_d_n6, assign18040_e12536_d_n7, assign18040_e12536_d_n8, assign18040_e12536_d_n9, assign18040_e12536_d_n10, assign18040_e12536_d_n11, assign18040_e12536_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18040_e12531: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18040_e12533: f64 = (assign18040_e12531 + locals.var_tmf2);
        let assign18040_e12534: f64 = (assign18040_e12533).sqrt();
        (assign18040_e12534, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18040_e12534)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18040_e12534)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18040_e12536;
        locals.var_tmf2_dn0 = assign18040_e12536_d_n0;
        locals.var_tmf2_dn2 = assign18040_e12536_d_n2;
        locals.var_tmf2_dn4 = assign18040_e12536_d_n4;
        locals.var_tmf2_dn5 = assign18040_e12536_d_n5;
        locals.var_tmf2_dn6 = assign18040_e12536_d_n6;
        locals.var_tmf2_dn7 = assign18040_e12536_d_n7;
        locals.var_tmf2_dn8 = assign18040_e12536_d_n8;
        locals.var_tmf2_dn9 = assign18040_e12536_d_n9;
        locals.var_tmf2_dn10 = assign18040_e12536_d_n10;
        locals.var_tmf2_dn11 = assign18040_e12536_d_n11;
        locals.var_tmf2_dn14 = assign18040_e12536_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18050_e12546, assign18050_e12546_d_n0, assign18050_e12546_d_n2, assign18050_e12546_d_n4, assign18050_e12546_d_n5, assign18050_e12546_d_n6, assign18050_e12546_d_n7, assign18050_e12546_d_n8, assign18050_e12546_d_n9, assign18050_e12546_d_n10, assign18050_e12546_d_n11, assign18050_e12546_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18050_e12542: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18050_e12543: f64 = (1.0 + assign18050_e12542);
        let assign18050_e12544: f64 = (0.5 * assign18050_e12543);
        (assign18050_e12544, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18050_e12546;
        locals.var_t0_dn0 = assign18050_e12546_d_n0;
        locals.var_t0_dn2 = assign18050_e12546_d_n2;
        locals.var_t0_dn4 = assign18050_e12546_d_n4;
        locals.var_t0_dn5 = assign18050_e12546_d_n5;
        locals.var_t0_dn6 = assign18050_e12546_d_n6;
        locals.var_t0_dn7 = assign18050_e12546_d_n7;
        locals.var_t0_dn8 = assign18050_e12546_d_n8;
        locals.var_t0_dn9 = assign18050_e12546_d_n9;
        locals.var_t0_dn10 = assign18050_e12546_d_n10;
        locals.var_t0_dn11 = assign18050_e12546_d_n11;
        locals.var_t0_dn14 = assign18050_e12546_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18060_e12556, assign18060_e12556_d_n0, assign18060_e12556_d_n2, assign18060_e12556_d_n4, assign18060_e12556_d_n5, assign18060_e12556_d_n6, assign18060_e12556_d_n7, assign18060_e12556_d_n8, assign18060_e12556_d_n9, assign18060_e12556_d_n10, assign18060_e12556_d_n11, assign18060_e12556_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18060_e12552: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18060_e12553: f64 = (0.5 * assign18060_e12552);
        let assign18060_e12554: f64 = assign18060_e12553;
        (assign18060_e12554, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18060_e12556;
        locals.var_t2_dn0 = assign18060_e12556_d_n0;
        locals.var_t2_dn2 = assign18060_e12556_d_n2;
        locals.var_t2_dn4 = assign18060_e12556_d_n4;
        locals.var_t2_dn5 = assign18060_e12556_d_n5;
        locals.var_t2_dn6 = assign18060_e12556_d_n6;
        locals.var_t2_dn7 = assign18060_e12556_d_n7;
        locals.var_t2_dn8 = assign18060_e12556_d_n8;
        locals.var_t2_dn9 = assign18060_e12556_d_n9;
        locals.var_t2_dn10 = assign18060_e12556_d_n10;
        locals.var_t2_dn11 = assign18060_e12556_d_n11;
        locals.var_t2_dn14 = assign18060_e12556_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign18070_e12564, assign18070_e12564_d_n0, assign18070_e12564_d_n2, assign18070_e12564_d_n4, assign18070_e12564_d_n5, assign18070_e12564_d_n6, assign18070_e12564_d_n7, assign18070_e12564_d_n8, assign18070_e12564_d_n9, assign18070_e12564_d_n10, assign18070_e12564_d_n11, assign18070_e12564_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18070_e12560: f64 = (1.0 - locals.var_t2);
        let assign18070_e12562: f64 = (assign18070_e12560 - 0.05);
        (assign18070_e12562, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18070_e12564;
        locals.var_tmf1_dn0 = assign18070_e12564_d_n0;
        locals.var_tmf1_dn2 = assign18070_e12564_d_n2;
        locals.var_tmf1_dn4 = assign18070_e12564_d_n4;
        locals.var_tmf1_dn5 = assign18070_e12564_d_n5;
        locals.var_tmf1_dn6 = assign18070_e12564_d_n6;
        locals.var_tmf1_dn7 = assign18070_e12564_d_n7;
        locals.var_tmf1_dn8 = assign18070_e12564_d_n8;
        locals.var_tmf1_dn9 = assign18070_e12564_d_n9;
        locals.var_tmf1_dn10 = assign18070_e12564_d_n10;
        locals.var_tmf1_dn11 = assign18070_e12564_d_n11;
        locals.var_tmf1_dn14 = assign18070_e12564_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18080_e12572, assign18080_e12572_d_n0, assign18080_e12572_d_n2, assign18080_e12572_d_n4, assign18080_e12572_d_n5, assign18080_e12572_d_n6, assign18080_e12572_d_n7, assign18080_e12572_d_n8, assign18080_e12572_d_n9, assign18080_e12572_d_n10, assign18080_e12572_d_n11, assign18080_e12572_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18080_e12568: f64 = 4.0;
        let assign18080_e12570: f64 = (assign18080_e12568 * 0.05);
        (assign18080_e12570, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18080_e12572;
        locals.var_tmf2_dn0 = assign18080_e12572_d_n0;
        locals.var_tmf2_dn2 = assign18080_e12572_d_n2;
        locals.var_tmf2_dn4 = assign18080_e12572_d_n4;
        locals.var_tmf2_dn5 = assign18080_e12572_d_n5;
        locals.var_tmf2_dn6 = assign18080_e12572_d_n6;
        locals.var_tmf2_dn7 = assign18080_e12572_d_n7;
        locals.var_tmf2_dn8 = assign18080_e12572_d_n8;
        locals.var_tmf2_dn9 = assign18080_e12572_d_n9;
        locals.var_tmf2_dn10 = assign18080_e12572_d_n10;
        locals.var_tmf2_dn11 = assign18080_e12572_d_n11;
        locals.var_tmf2_dn14 = assign18080_e12572_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18090_e12582, assign18090_e12582_d_n0, assign18090_e12582_d_n2, assign18090_e12582_d_n4, assign18090_e12582_d_n5, assign18090_e12582_d_n6, assign18090_e12582_d_n7, assign18090_e12582_d_n8, assign18090_e12582_d_n9, assign18090_e12582_d_n10, assign18090_e12582_d_n11, assign18090_e12582_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let (assign18090_e12580, assign18090_e12580_d_n0, assign18090_e12580_d_n2, assign18090_e12580_d_n4, assign18090_e12580_d_n5, assign18090_e12580_d_n6, assign18090_e12580_d_n7, assign18090_e12580_d_n8, assign18090_e12580_d_n9, assign18090_e12580_d_n10, assign18090_e12580_d_n11, assign18090_e12580_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18090_e12579: f64 = (-locals.var_tmf2);
                (assign18090_e12579, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18090_e12580, assign18090_e12580_d_n0, assign18090_e12580_d_n2, assign18090_e12580_d_n4, assign18090_e12580_d_n5, assign18090_e12580_d_n6, assign18090_e12580_d_n7, assign18090_e12580_d_n8, assign18090_e12580_d_n9, assign18090_e12580_d_n10, assign18090_e12580_d_n11, assign18090_e12580_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18090_e12582;
        locals.var_tmf2_dn0 = assign18090_e12582_d_n0;
        locals.var_tmf2_dn2 = assign18090_e12582_d_n2;
        locals.var_tmf2_dn4 = assign18090_e12582_d_n4;
        locals.var_tmf2_dn5 = assign18090_e12582_d_n5;
        locals.var_tmf2_dn6 = assign18090_e12582_d_n6;
        locals.var_tmf2_dn7 = assign18090_e12582_d_n7;
        locals.var_tmf2_dn8 = assign18090_e12582_d_n8;
        locals.var_tmf2_dn9 = assign18090_e12582_d_n9;
        locals.var_tmf2_dn10 = assign18090_e12582_d_n10;
        locals.var_tmf2_dn11 = assign18090_e12582_d_n11;
        locals.var_tmf2_dn14 = assign18090_e12582_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18100_e12591, assign18100_e12591_d_n0, assign18100_e12591_d_n2, assign18100_e12591_d_n4, assign18100_e12591_d_n5, assign18100_e12591_d_n6, assign18100_e12591_d_n7, assign18100_e12591_d_n8, assign18100_e12591_d_n9, assign18100_e12591_d_n10, assign18100_e12591_d_n11, assign18100_e12591_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18100_e12586: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18100_e12588: f64 = (assign18100_e12586 + locals.var_tmf2);
        let assign18100_e12589: f64 = (assign18100_e12588).sqrt();
        (assign18100_e12589, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18100_e12589)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18100_e12589)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18100_e12591;
        locals.var_tmf2_dn0 = assign18100_e12591_d_n0;
        locals.var_tmf2_dn2 = assign18100_e12591_d_n2;
        locals.var_tmf2_dn4 = assign18100_e12591_d_n4;
        locals.var_tmf2_dn5 = assign18100_e12591_d_n5;
        locals.var_tmf2_dn6 = assign18100_e12591_d_n6;
        locals.var_tmf2_dn7 = assign18100_e12591_d_n7;
        locals.var_tmf2_dn8 = assign18100_e12591_d_n8;
        locals.var_tmf2_dn9 = assign18100_e12591_d_n9;
        locals.var_tmf2_dn10 = assign18100_e12591_d_n10;
        locals.var_tmf2_dn11 = assign18100_e12591_d_n11;
        locals.var_tmf2_dn14 = assign18100_e12591_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18110_e12601, assign18110_e12601_d_n0, assign18110_e12601_d_n2, assign18110_e12601_d_n4, assign18110_e12601_d_n5, assign18110_e12601_d_n6, assign18110_e12601_d_n7, assign18110_e12601_d_n8, assign18110_e12601_d_n9, assign18110_e12601_d_n10, assign18110_e12601_d_n11, assign18110_e12601_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18110_e12597: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18110_e12598: f64 = (1.0 + assign18110_e12597);
        let assign18110_e12599: f64 = (0.5 * assign18110_e12598);
        (assign18110_e12599, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18110_e12601;
        locals.var_t0_dn0 = assign18110_e12601_d_n0;
        locals.var_t0_dn2 = assign18110_e12601_d_n2;
        locals.var_t0_dn4 = assign18110_e12601_d_n4;
        locals.var_t0_dn5 = assign18110_e12601_d_n5;
        locals.var_t0_dn6 = assign18110_e12601_d_n6;
        locals.var_t0_dn7 = assign18110_e12601_d_n7;
        locals.var_t0_dn8 = assign18110_e12601_d_n8;
        locals.var_t0_dn9 = assign18110_e12601_d_n9;
        locals.var_t0_dn10 = assign18110_e12601_d_n10;
        locals.var_t0_dn11 = assign18110_e12601_d_n11;
        locals.var_t0_dn14 = assign18110_e12601_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18120_e12611, assign18120_e12611_d_n0, assign18120_e12611_d_n2, assign18120_e12611_d_n4, assign18120_e12611_d_n5, assign18120_e12611_d_n6, assign18120_e12611_d_n7, assign18120_e12611_d_n8, assign18120_e12611_d_n9, assign18120_e12611_d_n10, assign18120_e12611_d_n11, assign18120_e12611_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18120_e12607: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18120_e12608: f64 = (0.5 * assign18120_e12607);
        let assign18120_e12609: f64 = (1.0 - assign18120_e12608);
        (assign18120_e12609, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign18120_e12611;
        locals.var_powratio_dn0 = assign18120_e12611_d_n0;
        locals.var_powratio_dn2 = assign18120_e12611_d_n2;
        locals.var_powratio_dn4 = assign18120_e12611_d_n4;
        locals.var_powratio_dn5 = assign18120_e12611_d_n5;
        locals.var_powratio_dn6 = assign18120_e12611_d_n6;
        locals.var_powratio_dn7 = assign18120_e12611_d_n7;
        locals.var_powratio_dn8 = assign18120_e12611_d_n8;
        locals.var_powratio_dn9 = assign18120_e12611_d_n9;
        locals.var_powratio_dn10 = assign18120_e12611_d_n10;
        locals.var_powratio_dn11 = assign18120_e12611_d_n11;
        locals.var_powratio_dn14 = assign18120_e12611_d_n14;
        locals.var_powratio_rv = 0.0;

        let (assign18130_e12622, assign18130_e12622_d_n0, assign18130_e12622_d_n2, assign18130_e12622_d_n4, assign18130_e12622_d_n5, assign18130_e12622_d_n6, assign18130_e12622_d_n7, assign18130_e12622_d_n8, assign18130_e12622_d_n9, assign18130_e12622_d_n10, assign18130_e12622_d_n11, assign18130_e12622_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18130_e12615: f64 = (2.0 * locals.var_beta_inv);
        let assign18130_e12618: f64 = (locals.var_nsub / locals.var_nin);
        let assign18130_e12619: f64 = (assign18130_e12618).ln();
        let assign18130_e12620: f64 = (assign18130_e12615 * assign18130_e12619);
        (assign18130_e12620, (((2.0 * locals.var_beta_inv_dn0) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn2) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn4) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn5) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn6) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn7) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn8) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn9) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn10) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn11) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))), (((2.0 * locals.var_beta_inv_dn14) * assign18130_e12619) + (assign18130_e12615 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign18130_e12618))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign18130_e12622;
        locals.var_pb2_dn0 = assign18130_e12622_d_n0;
        locals.var_pb2_dn2 = assign18130_e12622_d_n2;
        locals.var_pb2_dn4 = assign18130_e12622_d_n4;
        locals.var_pb2_dn5 = assign18130_e12622_d_n5;
        locals.var_pb2_dn6 = assign18130_e12622_d_n6;
        locals.var_pb2_dn7 = assign18130_e12622_d_n7;
        locals.var_pb2_dn8 = assign18130_e12622_d_n8;
        locals.var_pb2_dn9 = assign18130_e12622_d_n9;
        locals.var_pb2_dn10 = assign18130_e12622_d_n10;
        locals.var_pb2_dn11 = assign18130_e12622_d_n11;
        locals.var_pb2_dn14 = assign18130_e12622_d_n14;
        locals.var_pb2_rv = 0.0;

        let (assign18140_e12630, assign18140_e12630_d_n0, assign18140_e12630_d_n2, assign18140_e12630_d_n4, assign18140_e12630_d_n5, assign18140_e12630_d_n6, assign18140_e12630_d_n7, assign18140_e12630_d_n8, assign18140_e12630_d_n9, assign18140_e12630_d_n10, assign18140_e12630_d_n11, assign18140_e12630_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18140_e12626: f64 = (2.0 * 1.034943e-10);
        let assign18140_e12628: f64 = (assign18140_e12626 / 1.6021918e-19);
        (assign18140_e12628, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18140_e12630;
        locals.var_t1_dn0 = assign18140_e12630_d_n0;
        locals.var_t1_dn2 = assign18140_e12630_d_n2;
        locals.var_t1_dn4 = assign18140_e12630_d_n4;
        locals.var_t1_dn5 = assign18140_e12630_d_n5;
        locals.var_t1_dn6 = assign18140_e12630_d_n6;
        locals.var_t1_dn7 = assign18140_e12630_d_n7;
        locals.var_t1_dn8 = assign18140_e12630_d_n8;
        locals.var_t1_dn9 = assign18140_e12630_d_n9;
        locals.var_t1_dn10 = assign18140_e12630_d_n10;
        locals.var_t1_dn11 = assign18140_e12630_d_n11;
        locals.var_t1_dn14 = assign18140_e12630_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18150_e12637, assign18150_e12637_d_n0, assign18150_e12637_d_n2, assign18150_e12637_d_n4, assign18150_e12637_d_n5, assign18150_e12637_d_n6, assign18150_e12637_d_n7, assign18150_e12637_d_n8, assign18150_e12637_d_n9, assign18150_e12637_d_n10, assign18150_e12637_d_n11, assign18150_e12637_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18150_e12634: f64 = (locals.var_t1 / locals.var_nsub);
        let assign18150_e12635: f64 = (assign18150_e12634).sqrt();
        (assign18150_e12635, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)), ((((locals.var_t1_dn14 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18150_e12635)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    }
};
        locals.var_wdpl = assign18150_e12637;
        locals.var_wdpl_dn0 = assign18150_e12637_d_n0;
        locals.var_wdpl_dn2 = assign18150_e12637_d_n2;
        locals.var_wdpl_dn4 = assign18150_e12637_d_n4;
        locals.var_wdpl_dn5 = assign18150_e12637_d_n5;
        locals.var_wdpl_dn6 = assign18150_e12637_d_n6;
        locals.var_wdpl_dn7 = assign18150_e12637_d_n7;
        locals.var_wdpl_dn8 = assign18150_e12637_d_n8;
        locals.var_wdpl_dn9 = assign18150_e12637_d_n9;
        locals.var_wdpl_dn10 = assign18150_e12637_d_n10;
        locals.var_wdpl_dn11 = assign18150_e12637_d_n11;
        locals.var_wdpl_dn14 = assign18150_e12637_d_n14;
        locals.var_wdpl_rv = 0.0;

        let (assign18160_e12644, assign18160_e12644_d_n0, assign18160_e12644_d_n2, assign18160_e12644_d_n4, assign18160_e12644_d_n5, assign18160_e12644_d_n6, assign18160_e12644_d_n7, assign18160_e12644_d_n8, assign18160_e12644_d_n9, assign18160_e12644_d_n10, assign18160_e12644_d_n11, assign18160_e12644_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign18160_e12641: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign18160_e12642: f64 = (assign18160_e12641).sqrt();
        (assign18160_e12642, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn11 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn11)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)), ((((locals.var_t1_dn14 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn14)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18160_e12642)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    }
};
        locals.var_wdplp = assign18160_e12644;
        locals.var_wdplp_dn0 = assign18160_e12644_d_n0;
        locals.var_wdplp_dn2 = assign18160_e12644_d_n2;
        locals.var_wdplp_dn4 = assign18160_e12644_d_n4;
        locals.var_wdplp_dn5 = assign18160_e12644_d_n5;
        locals.var_wdplp_dn6 = assign18160_e12644_d_n6;
        locals.var_wdplp_dn7 = assign18160_e12644_d_n7;
        locals.var_wdplp_dn8 = assign18160_e12644_d_n8;
        locals.var_wdplp_dn9 = assign18160_e12644_d_n9;
        locals.var_wdplp_dn10 = assign18160_e12644_d_n10;
        locals.var_wdplp_dn11 = assign18160_e12644_d_n11;
        locals.var_wdplp_dn14 = assign18160_e12644_d_n14;
        locals.var_wdplp_rv = 0.0;

        let assign18170_e12647: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign18170_e12647;
        locals.var_guard376_rv = 0.0;

        let (assign18180_e12662, assign18180_e12662_d_n0, assign18180_e12662_d_n2, assign18180_e12662_d_n4, assign18180_e12662_d_n5, assign18180_e12662_d_n6, assign18180_e12662_d_n7, assign18180_e12662_d_n8, assign18180_e12662_d_n9, assign18180_e12662_d_n10, assign18180_e12662_d_n11, assign18180_e12662_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18180_e12653: f64 = (2.0 * 1.034943e-10);
        let assign18180_e12655: f64 = (assign18180_e12653 * 1.6021918e-19);
        let assign18180_e12657: f64 = (assign18180_e12655 * locals.var_nsub);
        let assign18180_e12659: f64 = (assign18180_e12657 * locals.var_beta_inv);
        let assign18180_e12660: f64 = (assign18180_e12659).sqrt();
        (assign18180_e12660, ((((assign18180_e12655 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn0)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn2)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn4)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn5)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn6)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn7)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn8)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn9)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn10)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn11) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn11)) / (2.0 * assign18180_e12660)), ((((assign18180_e12655 * locals.var_nsub_dn14) * locals.var_beta_inv) + (assign18180_e12657 * locals.var_beta_inv_dn14)) / (2.0 * assign18180_e12660)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign18180_e12662;
        locals.var_cnst0_dn0 = assign18180_e12662_d_n0;
        locals.var_cnst0_dn2 = assign18180_e12662_d_n2;
        locals.var_cnst0_dn4 = assign18180_e12662_d_n4;
        locals.var_cnst0_dn5 = assign18180_e12662_d_n5;
        locals.var_cnst0_dn6 = assign18180_e12662_d_n6;
        locals.var_cnst0_dn7 = assign18180_e12662_d_n7;
        locals.var_cnst0_dn8 = assign18180_e12662_d_n8;
        locals.var_cnst0_dn9 = assign18180_e12662_d_n9;
        locals.var_cnst0_dn10 = assign18180_e12662_d_n10;
        locals.var_cnst0_dn11 = assign18180_e12662_d_n11;
        locals.var_cnst0_dn14 = assign18180_e12662_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign18190_e12670, assign18190_e12670_d_n0, assign18190_e12670_d_n2, assign18190_e12670_d_n4, assign18190_e12670_d_n5, assign18190_e12670_d_n6, assign18190_e12670_d_n7, assign18190_e12670_d_n8, assign18190_e12670_d_n9, assign18190_e12670_d_n10, assign18190_e12670_d_n11, assign18190_e12670_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18190_e12668: f64 = (locals.var_nin / locals.var_nsub);
        (assign18190_e12668, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn14 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18190_e12670;
        locals.var_t1_dn0 = assign18190_e12670_d_n0;
        locals.var_t1_dn2 = assign18190_e12670_d_n2;
        locals.var_t1_dn4 = assign18190_e12670_d_n4;
        locals.var_t1_dn5 = assign18190_e12670_d_n5;
        locals.var_t1_dn6 = assign18190_e12670_d_n6;
        locals.var_t1_dn7 = assign18190_e12670_d_n7;
        locals.var_t1_dn8 = assign18190_e12670_d_n8;
        locals.var_t1_dn9 = assign18190_e12670_d_n9;
        locals.var_t1_dn10 = assign18190_e12670_d_n10;
        locals.var_t1_dn11 = assign18190_e12670_d_n11;
        locals.var_t1_dn14 = assign18190_e12670_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18200_e12678, assign18200_e12678_d_n0, assign18200_e12678_d_n2, assign18200_e12678_d_n4, assign18200_e12678_d_n5, assign18200_e12678_d_n6, assign18200_e12678_d_n7, assign18200_e12678_d_n8, assign18200_e12678_d_n9, assign18200_e12678_d_n10, assign18200_e12678_d_n11, assign18200_e12678_d_n14,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard376 != 0.0)) {
        let assign18200_e12676: f64 = (locals.var_t1 * locals.var_t1);
        (assign18200_e12676, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign18200_e12678;
        locals.var_cnst1_dn0 = assign18200_e12678_d_n0;
        locals.var_cnst1_dn2 = assign18200_e12678_d_n2;
        locals.var_cnst1_dn4 = assign18200_e12678_d_n4;
        locals.var_cnst1_dn5 = assign18200_e12678_d_n5;
        locals.var_cnst1_dn6 = assign18200_e12678_d_n6;
        locals.var_cnst1_dn7 = assign18200_e12678_d_n7;
        locals.var_cnst1_dn8 = assign18200_e12678_d_n8;
        locals.var_cnst1_dn9 = assign18200_e12678_d_n9;
        locals.var_cnst1_dn10 = assign18200_e12678_d_n10;
        locals.var_cnst1_dn11 = assign18200_e12678_d_n11;
        locals.var_cnst1_dn14 = assign18200_e12678_d_n14;
        locals.var_cnst1_rv = 0.0;

        let assign18210_e12681: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign18210_e12681;
        locals.var_guard377_rv = 0.0;

        let assign18220_e12684: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign18220_e12684;
        locals.var_guard378_rv = 0.0;

        let (assign18230_e12697, assign18230_e12697_d_n0, assign18230_e12697_d_n2, assign18230_e12697_d_n4, assign18230_e12697_d_n5, assign18230_e12697_d_n6, assign18230_e12697_d_n7, assign18230_e12697_d_n8, assign18230_e12697_d_n9, assign18230_e12697_d_n10, assign18230_e12697_d_n11, assign18230_e12697_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 != 0.0)) && (locals.var_guard378 != 0.0)) {
        let assign18230_e12693: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign18230_e12694: f64 = (assign18230_e12693).sqrt();
        let assign18230_e12695: f64 = (locals.var_cnst0 * assign18230_e12694);
        (assign18230_e12695, ((locals.var_cnst0_dn0 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn2 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn4 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn5 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn6 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn7 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn8 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn9 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn10 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn11 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))), ((locals.var_cnst0_dn14 * assign18230_e12694) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12694)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18230_e12697;
        locals.var_cnst0over_dn0 = assign18230_e12697_d_n0;
        locals.var_cnst0over_dn2 = assign18230_e12697_d_n2;
        locals.var_cnst0over_dn4 = assign18230_e12697_d_n4;
        locals.var_cnst0over_dn5 = assign18230_e12697_d_n5;
        locals.var_cnst0over_dn6 = assign18230_e12697_d_n6;
        locals.var_cnst0over_dn7 = assign18230_e12697_d_n7;
        locals.var_cnst0over_dn8 = assign18230_e12697_d_n8;
        locals.var_cnst0over_dn9 = assign18230_e12697_d_n9;
        locals.var_cnst0over_dn10 = assign18230_e12697_d_n10;
        locals.var_cnst0over_dn11 = assign18230_e12697_d_n11;
        locals.var_cnst0over_dn14 = assign18230_e12697_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign18240_e12700: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign18240_e12700;
        locals.var_guard379_rv = 0.0;

        let (assign18250_e12713, assign18250_e12713_d_n0, assign18250_e12713_d_n2, assign18250_e12713_d_n4, assign18250_e12713_d_n5, assign18250_e12713_d_n6, assign18250_e12713_d_n7, assign18250_e12713_d_n8, assign18250_e12713_d_n9, assign18250_e12713_d_n10, assign18250_e12713_d_n11, assign18250_e12713_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 != 0.0)) && (locals.var_guard379 != 0.0)) {
        let assign18250_e12709: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign18250_e12710: f64 = (assign18250_e12709).sqrt();
        let assign18250_e12711: f64 = (locals.var_cnst0 * assign18250_e12710);
        (assign18250_e12711, ((locals.var_cnst0_dn0 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn2 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn4 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn5 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn6 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn7 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn8 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn9 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn10 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn11 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))), ((locals.var_cnst0_dn14 * assign18250_e12710) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18250_e12710)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18250_e12713;
        locals.var_cnst0overs_dn0 = assign18250_e12713_d_n0;
        locals.var_cnst0overs_dn2 = assign18250_e12713_d_n2;
        locals.var_cnst0overs_dn4 = assign18250_e12713_d_n4;
        locals.var_cnst0overs_dn5 = assign18250_e12713_d_n5;
        locals.var_cnst0overs_dn6 = assign18250_e12713_d_n6;
        locals.var_cnst0overs_dn7 = assign18250_e12713_d_n7;
        locals.var_cnst0overs_dn8 = assign18250_e12713_d_n8;
        locals.var_cnst0overs_dn9 = assign18250_e12713_d_n9;
        locals.var_cnst0overs_dn10 = assign18250_e12713_d_n10;
        locals.var_cnst0overs_dn11 = assign18250_e12713_d_n11;
        locals.var_cnst0overs_dn14 = assign18250_e12713_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign18260_e12716: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign18260_e12716;
        locals.var_guard380_rv = 0.0;

        let (assign18270_e12730, assign18270_e12730_d_n0, assign18270_e12730_d_n2, assign18270_e12730_d_n4, assign18270_e12730_d_n5, assign18270_e12730_d_n6, assign18270_e12730_d_n7, assign18270_e12730_d_n8, assign18270_e12730_d_n9, assign18270_e12730_d_n10, assign18270_e12730_d_n11, assign18270_e12730_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 == 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18270_e12726: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign18270_e12727: f64 = (assign18270_e12726).sqrt();
        let assign18270_e12728: f64 = (locals.var_cnst0 * assign18270_e12727);
        (assign18270_e12728, ((locals.var_cnst0_dn0 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn2 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn4 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn5 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn6 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn7 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn8 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn9 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn10 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn11 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))), ((locals.var_cnst0_dn14 * assign18270_e12727) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12727)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18270_e12730;
        locals.var_cnst0over_dn0 = assign18270_e12730_d_n0;
        locals.var_cnst0over_dn2 = assign18270_e12730_d_n2;
        locals.var_cnst0over_dn4 = assign18270_e12730_d_n4;
        locals.var_cnst0over_dn5 = assign18270_e12730_d_n5;
        locals.var_cnst0over_dn6 = assign18270_e12730_d_n6;
        locals.var_cnst0over_dn7 = assign18270_e12730_d_n7;
        locals.var_cnst0over_dn8 = assign18270_e12730_d_n8;
        locals.var_cnst0over_dn9 = assign18270_e12730_d_n9;
        locals.var_cnst0over_dn10 = assign18270_e12730_d_n10;
        locals.var_cnst0over_dn11 = assign18270_e12730_d_n11;
        locals.var_cnst0over_dn14 = assign18270_e12730_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign18280_e12733: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign18280_e12733;
        locals.var_guard381_rv = 0.0;

        let (assign18290_e12747, assign18290_e12747_d_n0, assign18290_e12747_d_n2, assign18290_e12747_d_n4, assign18290_e12747_d_n5, assign18290_e12747_d_n6, assign18290_e12747_d_n7, assign18290_e12747_d_n8, assign18290_e12747_d_n9, assign18290_e12747_d_n10, assign18290_e12747_d_n11, assign18290_e12747_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard377 == 0.0)) && (locals.var_guard381 != 0.0)) {
        let assign18290_e12743: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign18290_e12744: f64 = (assign18290_e12743).sqrt();
        let assign18290_e12745: f64 = (locals.var_cnst0 * assign18290_e12744);
        (assign18290_e12745, ((locals.var_cnst0_dn0 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn2 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn4 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn5 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn6 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn7 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn8 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn9 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn10 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn11 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))), ((locals.var_cnst0_dn14 * assign18290_e12744) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18290_e12744)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18290_e12747;
        locals.var_cnst0overs_dn0 = assign18290_e12747_d_n0;
        locals.var_cnst0overs_dn2 = assign18290_e12747_d_n2;
        locals.var_cnst0overs_dn4 = assign18290_e12747_d_n4;
        locals.var_cnst0overs_dn5 = assign18290_e12747_d_n5;
        locals.var_cnst0overs_dn6 = assign18290_e12747_d_n6;
        locals.var_cnst0overs_dn7 = assign18290_e12747_d_n7;
        locals.var_cnst0overs_dn8 = assign18290_e12747_d_n8;
        locals.var_cnst0overs_dn9 = assign18290_e12747_d_n9;
        locals.var_cnst0overs_dn10 = assign18290_e12747_d_n10;
        locals.var_cnst0overs_dn11 = assign18290_e12747_d_n11;
        locals.var_cnst0overs_dn14 = assign18290_e12747_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign18300_e12750: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign18300_e12750;
        locals.var_guard382_rv = 0.0;

        let assign18310_e12753: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign18310_e12753;
        locals.var_guard383_rv = 0.0;

        let (assign18320_e12777, assign18320_e12777_d_n0, assign18320_e12777_d_n2, assign18320_e12777_d_n4, assign18320_e12777_d_n5, assign18320_e12777_d_n6, assign18320_e12777_d_n7, assign18320_e12777_d_n8, assign18320_e12777_d_n9, assign18320_e12777_d_n10, assign18320_e12777_d_n11, assign18320_e12777_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18320_e12762: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18320_e12764: f64 = (assign18320_e12762 * 1000000.0);
        let assign18320_e12766: f64 = (assign18320_e12764 + locals.var_uc_rdict1);
        let assign18320_e12767: f64 = (locals.var_rdtemp0 * assign18320_e12766);
        let assign18320_e12770: f64 = (p.p68 * p.p100);
        let assign18320_e12772: f64 = (assign18320_e12770 * 1000000.0);
        let assign18320_e12774: f64 = (assign18320_e12772 + p.p101);
        let assign18320_e12775: f64 = (assign18320_e12767 * assign18320_e12774);
        (assign18320_e12775, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18320_e12777;
        locals.var_t2_dn0 = assign18320_e12777_d_n0;
        locals.var_t2_dn2 = assign18320_e12777_d_n2;
        locals.var_t2_dn4 = assign18320_e12777_d_n4;
        locals.var_t2_dn5 = assign18320_e12777_d_n5;
        locals.var_t2_dn6 = assign18320_e12777_d_n6;
        locals.var_t2_dn7 = assign18320_e12777_d_n7;
        locals.var_t2_dn8 = assign18320_e12777_d_n8;
        locals.var_t2_dn9 = assign18320_e12777_d_n9;
        locals.var_t2_dn10 = assign18320_e12777_d_n10;
        locals.var_t2_dn11 = assign18320_e12777_d_n11;
        locals.var_t2_dn14 = assign18320_e12777_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18330_e12780: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign18330_e12780;
        locals.var_guard384_rv = 0.0;

        let (assign18340_e12800, assign18340_e12800_d_n0, assign18340_e12800_d_n2, assign18340_e12800_d_n4, assign18340_e12800_d_n5, assign18340_e12800_d_n6, assign18340_e12800_d_n7, assign18340_e12800_d_n8, assign18340_e12800_d_n9, assign18340_e12800_d_n10, assign18340_e12800_d_n11, assign18340_e12800_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18340_e12791: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18340_e12792: f64 = (locals.var_uc_rd + assign18340_e12791);
        let assign18340_e12795: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18340_e12796: f64 = (assign18340_e12792 + assign18340_e12795);
        let assign18340_e12798: f64 = (assign18340_e12796 * locals.var_t2);
        (assign18340_e12798, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18340_e12796 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18340_e12800;
        locals.var_rde_dn0 = assign18340_e12800_d_n0;
        locals.var_rde_dn2 = assign18340_e12800_d_n2;
        locals.var_rde_dn4 = assign18340_e12800_d_n4;
        locals.var_rde_dn5 = assign18340_e12800_d_n5;
        locals.var_rde_dn6 = assign18340_e12800_d_n6;
        locals.var_rde_dn7 = assign18340_e12800_d_n7;
        locals.var_rde_dn8 = assign18340_e12800_d_n8;
        locals.var_rde_dn9 = assign18340_e12800_d_n9;
        locals.var_rde_dn10 = assign18340_e12800_d_n10;
        locals.var_rde_dn11 = assign18340_e12800_d_n11;
        locals.var_rde_dn14 = assign18340_e12800_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18350_e12818, assign18350_e12818_d_n0, assign18350_e12818_d_n2, assign18350_e12818_d_n4, assign18350_e12818_d_n5, assign18350_e12818_d_n6, assign18350_e12818_d_n7, assign18350_e12818_d_n8, assign18350_e12818_d_n9, assign18350_e12818_d_n10, assign18350_e12818_d_n11, assign18350_e12818_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18350_e12811: f64 = (0.005 * locals.var_uc_rd);
        let assign18350_e12812: f64 = (locals.var_rde - assign18350_e12811);
        let assign18350_e12815: f64 = (0.01 * locals.var_uc_rd);
        let assign18350_e12816: f64 = (assign18350_e12812 - assign18350_e12815);
        (assign18350_e12816, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18350_e12818;
        locals.var_tmf1_dn0 = assign18350_e12818_d_n0;
        locals.var_tmf1_dn2 = assign18350_e12818_d_n2;
        locals.var_tmf1_dn4 = assign18350_e12818_d_n4;
        locals.var_tmf1_dn5 = assign18350_e12818_d_n5;
        locals.var_tmf1_dn6 = assign18350_e12818_d_n6;
        locals.var_tmf1_dn7 = assign18350_e12818_d_n7;
        locals.var_tmf1_dn8 = assign18350_e12818_d_n8;
        locals.var_tmf1_dn9 = assign18350_e12818_d_n9;
        locals.var_tmf1_dn10 = assign18350_e12818_d_n10;
        locals.var_tmf1_dn11 = assign18350_e12818_d_n11;
        locals.var_tmf1_dn14 = assign18350_e12818_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18360_e12836, assign18360_e12836_d_n0, assign18360_e12836_d_n2, assign18360_e12836_d_n4, assign18360_e12836_d_n5, assign18360_e12836_d_n6, assign18360_e12836_d_n7, assign18360_e12836_d_n8, assign18360_e12836_d_n9, assign18360_e12836_d_n10, assign18360_e12836_d_n11, assign18360_e12836_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18360_e12829: f64 = (0.005 * locals.var_uc_rd);
        let assign18360_e12830: f64 = (4.0 * assign18360_e12829);
        let assign18360_e12833: f64 = (0.01 * locals.var_uc_rd);
        let assign18360_e12834: f64 = (assign18360_e12830 * assign18360_e12833);
        (assign18360_e12834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18360_e12836;
        locals.var_tmf2_dn0 = assign18360_e12836_d_n0;
        locals.var_tmf2_dn2 = assign18360_e12836_d_n2;
        locals.var_tmf2_dn4 = assign18360_e12836_d_n4;
        locals.var_tmf2_dn5 = assign18360_e12836_d_n5;
        locals.var_tmf2_dn6 = assign18360_e12836_d_n6;
        locals.var_tmf2_dn7 = assign18360_e12836_d_n7;
        locals.var_tmf2_dn8 = assign18360_e12836_d_n8;
        locals.var_tmf2_dn9 = assign18360_e12836_d_n9;
        locals.var_tmf2_dn10 = assign18360_e12836_d_n10;
        locals.var_tmf2_dn11 = assign18360_e12836_d_n11;
        locals.var_tmf2_dn14 = assign18360_e12836_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18370_e12852, assign18370_e12852_d_n0, assign18370_e12852_d_n2, assign18370_e12852_d_n4, assign18370_e12852_d_n5, assign18370_e12852_d_n6, assign18370_e12852_d_n7, assign18370_e12852_d_n8, assign18370_e12852_d_n9, assign18370_e12852_d_n10, assign18370_e12852_d_n11, assign18370_e12852_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let (assign18370_e12850, assign18370_e12850_d_n0, assign18370_e12850_d_n2, assign18370_e12850_d_n4, assign18370_e12850_d_n5, assign18370_e12850_d_n6, assign18370_e12850_d_n7, assign18370_e12850_d_n8, assign18370_e12850_d_n9, assign18370_e12850_d_n10, assign18370_e12850_d_n11, assign18370_e12850_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18370_e12849: f64 = (-locals.var_tmf2);
                (assign18370_e12849, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18370_e12850, assign18370_e12850_d_n0, assign18370_e12850_d_n2, assign18370_e12850_d_n4, assign18370_e12850_d_n5, assign18370_e12850_d_n6, assign18370_e12850_d_n7, assign18370_e12850_d_n8, assign18370_e12850_d_n9, assign18370_e12850_d_n10, assign18370_e12850_d_n11, assign18370_e12850_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18370_e12852;
        locals.var_tmf2_dn0 = assign18370_e12852_d_n0;
        locals.var_tmf2_dn2 = assign18370_e12852_d_n2;
        locals.var_tmf2_dn4 = assign18370_e12852_d_n4;
        locals.var_tmf2_dn5 = assign18370_e12852_d_n5;
        locals.var_tmf2_dn6 = assign18370_e12852_d_n6;
        locals.var_tmf2_dn7 = assign18370_e12852_d_n7;
        locals.var_tmf2_dn8 = assign18370_e12852_d_n8;
        locals.var_tmf2_dn9 = assign18370_e12852_d_n9;
        locals.var_tmf2_dn10 = assign18370_e12852_d_n10;
        locals.var_tmf2_dn11 = assign18370_e12852_d_n11;
        locals.var_tmf2_dn14 = assign18370_e12852_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18380_e12867, assign18380_e12867_d_n0, assign18380_e12867_d_n2, assign18380_e12867_d_n4, assign18380_e12867_d_n5, assign18380_e12867_d_n6, assign18380_e12867_d_n7, assign18380_e12867_d_n8, assign18380_e12867_d_n9, assign18380_e12867_d_n10, assign18380_e12867_d_n11, assign18380_e12867_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18380_e12862: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18380_e12864: f64 = (assign18380_e12862 + locals.var_tmf2);
        let assign18380_e12865: f64 = (assign18380_e12864).sqrt();
        (assign18380_e12865, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18380_e12865)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18380_e12865)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18380_e12867;
        locals.var_tmf2_dn0 = assign18380_e12867_d_n0;
        locals.var_tmf2_dn2 = assign18380_e12867_d_n2;
        locals.var_tmf2_dn4 = assign18380_e12867_d_n4;
        locals.var_tmf2_dn5 = assign18380_e12867_d_n5;
        locals.var_tmf2_dn6 = assign18380_e12867_d_n6;
        locals.var_tmf2_dn7 = assign18380_e12867_d_n7;
        locals.var_tmf2_dn8 = assign18380_e12867_d_n8;
        locals.var_tmf2_dn9 = assign18380_e12867_d_n9;
        locals.var_tmf2_dn10 = assign18380_e12867_d_n10;
        locals.var_tmf2_dn11 = assign18380_e12867_d_n11;
        locals.var_tmf2_dn14 = assign18380_e12867_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18390_e12883, assign18390_e12883_d_n0, assign18390_e12883_d_n2, assign18390_e12883_d_n4, assign18390_e12883_d_n5, assign18390_e12883_d_n6, assign18390_e12883_d_n7, assign18390_e12883_d_n8, assign18390_e12883_d_n9, assign18390_e12883_d_n10, assign18390_e12883_d_n11, assign18390_e12883_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18390_e12879: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18390_e12880: f64 = (1.0 + assign18390_e12879);
        let assign18390_e12881: f64 = (0.5 * assign18390_e12880);
        (assign18390_e12881, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18390_e12883;
        locals.var_t0_dn0 = assign18390_e12883_d_n0;
        locals.var_t0_dn2 = assign18390_e12883_d_n2;
        locals.var_t0_dn4 = assign18390_e12883_d_n4;
        locals.var_t0_dn5 = assign18390_e12883_d_n5;
        locals.var_t0_dn6 = assign18390_e12883_d_n6;
        locals.var_t0_dn7 = assign18390_e12883_d_n7;
        locals.var_t0_dn8 = assign18390_e12883_d_n8;
        locals.var_t0_dn9 = assign18390_e12883_d_n9;
        locals.var_t0_dn10 = assign18390_e12883_d_n10;
        locals.var_t0_dn11 = assign18390_e12883_d_n11;
        locals.var_t0_dn14 = assign18390_e12883_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18400_e12901, assign18400_e12901_d_n0, assign18400_e12901_d_n2, assign18400_e12901_d_n4, assign18400_e12901_d_n5, assign18400_e12901_d_n6, assign18400_e12901_d_n7, assign18400_e12901_d_n8, assign18400_e12901_d_n9, assign18400_e12901_d_n10, assign18400_e12901_d_n11, assign18400_e12901_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18400_e12893: f64 = (0.005 * locals.var_uc_rd);
        let assign18400_e12897: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18400_e12898: f64 = (0.5 * assign18400_e12897);
        let assign18400_e12899: f64 = (assign18400_e12893 + assign18400_e12898);
        (assign18400_e12899, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18400_e12901;
        locals.var_rde_dn0 = assign18400_e12901_d_n0;
        locals.var_rde_dn2 = assign18400_e12901_d_n2;
        locals.var_rde_dn4 = assign18400_e12901_d_n4;
        locals.var_rde_dn5 = assign18400_e12901_d_n5;
        locals.var_rde_dn6 = assign18400_e12901_d_n6;
        locals.var_rde_dn7 = assign18400_e12901_d_n7;
        locals.var_rde_dn8 = assign18400_e12901_d_n8;
        locals.var_rde_dn9 = assign18400_e12901_d_n9;
        locals.var_rde_dn10 = assign18400_e12901_d_n10;
        locals.var_rde_dn11 = assign18400_e12901_d_n11;
        locals.var_rde_dn14 = assign18400_e12901_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18410_e12922, assign18410_e12922_d_n0, assign18410_e12922_d_n2, assign18410_e12922_d_n4, assign18410_e12922_d_n5, assign18410_e12922_d_n6, assign18410_e12922_d_n7, assign18410_e12922_d_n8, assign18410_e12922_d_n9, assign18410_e12922_d_n10, assign18410_e12922_d_n11, assign18410_e12922_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18410_e12913: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18410_e12914: f64 = (locals.var_uc_rd + assign18410_e12913);
        let assign18410_e12917: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18410_e12918: f64 = (assign18410_e12914 + assign18410_e12917);
        let assign18410_e12920: f64 = (assign18410_e12918 * locals.var_t2);
        (assign18410_e12920, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18410_e12918 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18410_e12922;
        locals.var_rde_dn0 = assign18410_e12922_d_n0;
        locals.var_rde_dn2 = assign18410_e12922_d_n2;
        locals.var_rde_dn4 = assign18410_e12922_d_n4;
        locals.var_rde_dn5 = assign18410_e12922_d_n5;
        locals.var_rde_dn6 = assign18410_e12922_d_n6;
        locals.var_rde_dn7 = assign18410_e12922_d_n7;
        locals.var_rde_dn8 = assign18410_e12922_d_n8;
        locals.var_rde_dn9 = assign18410_e12922_d_n9;
        locals.var_rde_dn10 = assign18410_e12922_d_n10;
        locals.var_rde_dn11 = assign18410_e12922_d_n11;
        locals.var_rde_dn14 = assign18410_e12922_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18420_e12941, assign18420_e12941_d_n0, assign18420_e12941_d_n2, assign18420_e12941_d_n4, assign18420_e12941_d_n5, assign18420_e12941_d_n6, assign18420_e12941_d_n7, assign18420_e12941_d_n8, assign18420_e12941_d_n9, assign18420_e12941_d_n10, assign18420_e12941_d_n11, assign18420_e12941_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18420_e12934: f64 = (0.005 * locals.var_uc_rd);
        let assign18420_e12935: f64 = (locals.var_rde - assign18420_e12934);
        let assign18420_e12938: f64 = (0.01 * locals.var_uc_rd);
        let assign18420_e12939: f64 = (assign18420_e12935 - assign18420_e12938);
        (assign18420_e12939, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18420_e12941;
        locals.var_tmf1_dn0 = assign18420_e12941_d_n0;
        locals.var_tmf1_dn2 = assign18420_e12941_d_n2;
        locals.var_tmf1_dn4 = assign18420_e12941_d_n4;
        locals.var_tmf1_dn5 = assign18420_e12941_d_n5;
        locals.var_tmf1_dn6 = assign18420_e12941_d_n6;
        locals.var_tmf1_dn7 = assign18420_e12941_d_n7;
        locals.var_tmf1_dn8 = assign18420_e12941_d_n8;
        locals.var_tmf1_dn9 = assign18420_e12941_d_n9;
        locals.var_tmf1_dn10 = assign18420_e12941_d_n10;
        locals.var_tmf1_dn11 = assign18420_e12941_d_n11;
        locals.var_tmf1_dn14 = assign18420_e12941_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18430_e12960, assign18430_e12960_d_n0, assign18430_e12960_d_n2, assign18430_e12960_d_n4, assign18430_e12960_d_n5, assign18430_e12960_d_n6, assign18430_e12960_d_n7, assign18430_e12960_d_n8, assign18430_e12960_d_n9, assign18430_e12960_d_n10, assign18430_e12960_d_n11, assign18430_e12960_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18430_e12953: f64 = (0.005 * locals.var_uc_rd);
        let assign18430_e12954: f64 = (4.0 * assign18430_e12953);
        let assign18430_e12957: f64 = (0.01 * locals.var_uc_rd);
        let assign18430_e12958: f64 = (assign18430_e12954 * assign18430_e12957);
        (assign18430_e12958, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18430_e12960;
        locals.var_tmf2_dn0 = assign18430_e12960_d_n0;
        locals.var_tmf2_dn2 = assign18430_e12960_d_n2;
        locals.var_tmf2_dn4 = assign18430_e12960_d_n4;
        locals.var_tmf2_dn5 = assign18430_e12960_d_n5;
        locals.var_tmf2_dn6 = assign18430_e12960_d_n6;
        locals.var_tmf2_dn7 = assign18430_e12960_d_n7;
        locals.var_tmf2_dn8 = assign18430_e12960_d_n8;
        locals.var_tmf2_dn9 = assign18430_e12960_d_n9;
        locals.var_tmf2_dn10 = assign18430_e12960_d_n10;
        locals.var_tmf2_dn11 = assign18430_e12960_d_n11;
        locals.var_tmf2_dn14 = assign18430_e12960_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18440_e12977, assign18440_e12977_d_n0, assign18440_e12977_d_n2, assign18440_e12977_d_n4, assign18440_e12977_d_n5, assign18440_e12977_d_n6, assign18440_e12977_d_n7, assign18440_e12977_d_n8, assign18440_e12977_d_n9, assign18440_e12977_d_n10, assign18440_e12977_d_n11, assign18440_e12977_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let (assign18440_e12975, assign18440_e12975_d_n0, assign18440_e12975_d_n2, assign18440_e12975_d_n4, assign18440_e12975_d_n5, assign18440_e12975_d_n6, assign18440_e12975_d_n7, assign18440_e12975_d_n8, assign18440_e12975_d_n9, assign18440_e12975_d_n10, assign18440_e12975_d_n11, assign18440_e12975_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18440_e12974: f64 = (-locals.var_tmf2);
                (assign18440_e12974, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18440_e12975, assign18440_e12975_d_n0, assign18440_e12975_d_n2, assign18440_e12975_d_n4, assign18440_e12975_d_n5, assign18440_e12975_d_n6, assign18440_e12975_d_n7, assign18440_e12975_d_n8, assign18440_e12975_d_n9, assign18440_e12975_d_n10, assign18440_e12975_d_n11, assign18440_e12975_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18440_e12977;
        locals.var_tmf2_dn0 = assign18440_e12977_d_n0;
        locals.var_tmf2_dn2 = assign18440_e12977_d_n2;
        locals.var_tmf2_dn4 = assign18440_e12977_d_n4;
        locals.var_tmf2_dn5 = assign18440_e12977_d_n5;
        locals.var_tmf2_dn6 = assign18440_e12977_d_n6;
        locals.var_tmf2_dn7 = assign18440_e12977_d_n7;
        locals.var_tmf2_dn8 = assign18440_e12977_d_n8;
        locals.var_tmf2_dn9 = assign18440_e12977_d_n9;
        locals.var_tmf2_dn10 = assign18440_e12977_d_n10;
        locals.var_tmf2_dn11 = assign18440_e12977_d_n11;
        locals.var_tmf2_dn14 = assign18440_e12977_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18450_e12993, assign18450_e12993_d_n0, assign18450_e12993_d_n2, assign18450_e12993_d_n4, assign18450_e12993_d_n5, assign18450_e12993_d_n6, assign18450_e12993_d_n7, assign18450_e12993_d_n8, assign18450_e12993_d_n9, assign18450_e12993_d_n10, assign18450_e12993_d_n11, assign18450_e12993_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18450_e12988: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18450_e12990: f64 = (assign18450_e12988 + locals.var_tmf2);
        let assign18450_e12991: f64 = (assign18450_e12990).sqrt();
        (assign18450_e12991, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18450_e12991)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18450_e12991)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18450_e12993;
        locals.var_tmf2_dn0 = assign18450_e12993_d_n0;
        locals.var_tmf2_dn2 = assign18450_e12993_d_n2;
        locals.var_tmf2_dn4 = assign18450_e12993_d_n4;
        locals.var_tmf2_dn5 = assign18450_e12993_d_n5;
        locals.var_tmf2_dn6 = assign18450_e12993_d_n6;
        locals.var_tmf2_dn7 = assign18450_e12993_d_n7;
        locals.var_tmf2_dn8 = assign18450_e12993_d_n8;
        locals.var_tmf2_dn9 = assign18450_e12993_d_n9;
        locals.var_tmf2_dn10 = assign18450_e12993_d_n10;
        locals.var_tmf2_dn11 = assign18450_e12993_d_n11;
        locals.var_tmf2_dn14 = assign18450_e12993_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18460_e13010, assign18460_e13010_d_n0, assign18460_e13010_d_n2, assign18460_e13010_d_n4, assign18460_e13010_d_n5, assign18460_e13010_d_n6, assign18460_e13010_d_n7, assign18460_e13010_d_n8, assign18460_e13010_d_n9, assign18460_e13010_d_n10, assign18460_e13010_d_n11, assign18460_e13010_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18460_e13006: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18460_e13007: f64 = (1.0 + assign18460_e13006);
        let assign18460_e13008: f64 = (0.5 * assign18460_e13007);
        (assign18460_e13008, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18460_e13010;
        locals.var_t0_dn0 = assign18460_e13010_d_n0;
        locals.var_t0_dn2 = assign18460_e13010_d_n2;
        locals.var_t0_dn4 = assign18460_e13010_d_n4;
        locals.var_t0_dn5 = assign18460_e13010_d_n5;
        locals.var_t0_dn6 = assign18460_e13010_d_n6;
        locals.var_t0_dn7 = assign18460_e13010_d_n7;
        locals.var_t0_dn8 = assign18460_e13010_d_n8;
        locals.var_t0_dn9 = assign18460_e13010_d_n9;
        locals.var_t0_dn10 = assign18460_e13010_d_n10;
        locals.var_t0_dn11 = assign18460_e13010_d_n11;
        locals.var_t0_dn14 = assign18460_e13010_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18470_e13029, assign18470_e13029_d_n0, assign18470_e13029_d_n2, assign18470_e13029_d_n4, assign18470_e13029_d_n5, assign18470_e13029_d_n6, assign18470_e13029_d_n7, assign18470_e13029_d_n8, assign18470_e13029_d_n9, assign18470_e13029_d_n10, assign18470_e13029_d_n11, assign18470_e13029_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18470_e13021: f64 = (0.005 * locals.var_uc_rd);
        let assign18470_e13025: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18470_e13026: f64 = (0.5 * assign18470_e13025);
        let assign18470_e13027: f64 = (assign18470_e13021 + assign18470_e13026);
        (assign18470_e13027, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18470_e13029;
        locals.var_rde_dn0 = assign18470_e13029_d_n0;
        locals.var_rde_dn2 = assign18470_e13029_d_n2;
        locals.var_rde_dn4 = assign18470_e13029_d_n4;
        locals.var_rde_dn5 = assign18470_e13029_d_n5;
        locals.var_rde_dn6 = assign18470_e13029_d_n6;
        locals.var_rde_dn7 = assign18470_e13029_d_n7;
        locals.var_rde_dn8 = assign18470_e13029_d_n8;
        locals.var_rde_dn9 = assign18470_e13029_d_n9;
        locals.var_rde_dn10 = assign18470_e13029_d_n10;
        locals.var_rde_dn11 = assign18470_e13029_d_n11;
        locals.var_rde_dn14 = assign18470_e13029_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18480_e13038, assign18480_e13038_d_n0, assign18480_e13038_d_n2, assign18480_e13038_d_n4, assign18480_e13038_d_n5, assign18480_e13038_d_n6, assign18480_e13038_d_n7, assign18480_e13038_d_n8, assign18480_e13038_d_n9, assign18480_e13038_d_n10, assign18480_e13038_d_n11, assign18480_e13038_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18480_e13038;
        locals.var_rde_dn0 = assign18480_e13038_d_n0;
        locals.var_rde_dn2 = assign18480_e13038_d_n2;
        locals.var_rde_dn4 = assign18480_e13038_d_n4;
        locals.var_rde_dn5 = assign18480_e13038_d_n5;
        locals.var_rde_dn6 = assign18480_e13038_d_n6;
        locals.var_rde_dn7 = assign18480_e13038_d_n7;
        locals.var_rde_dn8 = assign18480_e13038_d_n8;
        locals.var_rde_dn9 = assign18480_e13038_d_n9;
        locals.var_rde_dn10 = assign18480_e13038_d_n10;
        locals.var_rde_dn11 = assign18480_e13038_d_n11;
        locals.var_rde_dn14 = assign18480_e13038_d_n14;
        locals.var_rde_rv = 0.0;

        let assign18490_e13041: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign18490_e13041;
        locals.var_guard385_rv = 0.0;

        let (assign18500_e13065, assign18500_e13065_d_n0, assign18500_e13065_d_n2, assign18500_e13065_d_n4, assign18500_e13065_d_n5, assign18500_e13065_d_n6, assign18500_e13065_d_n7, assign18500_e13065_d_n8, assign18500_e13065_d_n9, assign18500_e13065_d_n10, assign18500_e13065_d_n11, assign18500_e13065_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18500_e13050: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign18500_e13052: f64 = (assign18500_e13050 * 1000000.0);
        let assign18500_e13054: f64 = (assign18500_e13052 + locals.var_uc_rdict1);
        let assign18500_e13055: f64 = (locals.var_rdtemp0 * assign18500_e13054);
        let assign18500_e13058: f64 = (p.p70 * p.p100);
        let assign18500_e13060: f64 = (assign18500_e13058 * 1000000.0);
        let assign18500_e13062: f64 = (assign18500_e13060 + p.p101);
        let assign18500_e13063: f64 = (assign18500_e13055 * assign18500_e13062);
        (assign18500_e13063, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18500_e13065;
        locals.var_t2_dn0 = assign18500_e13065_d_n0;
        locals.var_t2_dn2 = assign18500_e13065_d_n2;
        locals.var_t2_dn4 = assign18500_e13065_d_n4;
        locals.var_t2_dn5 = assign18500_e13065_d_n5;
        locals.var_t2_dn6 = assign18500_e13065_d_n6;
        locals.var_t2_dn7 = assign18500_e13065_d_n7;
        locals.var_t2_dn8 = assign18500_e13065_d_n8;
        locals.var_t2_dn9 = assign18500_e13065_d_n9;
        locals.var_t2_dn10 = assign18500_e13065_d_n10;
        locals.var_t2_dn11 = assign18500_e13065_d_n11;
        locals.var_t2_dn14 = assign18500_e13065_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18510_e13068: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign18510_e13068;
        locals.var_guard386_rv = 0.0;

        let (assign18520_e13088, assign18520_e13088_d_n0, assign18520_e13088_d_n2, assign18520_e13088_d_n4, assign18520_e13088_d_n5, assign18520_e13088_d_n6, assign18520_e13088_d_n7, assign18520_e13088_d_n8, assign18520_e13088_d_n9, assign18520_e13088_d_n10, assign18520_e13088_d_n11, assign18520_e13088_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18520_e13079: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18520_e13080: f64 = (locals.var_uc_rs + assign18520_e13079);
        let assign18520_e13083: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18520_e13084: f64 = (assign18520_e13080 + assign18520_e13083);
        let assign18520_e13086: f64 = (assign18520_e13084 * locals.var_t2);
        (assign18520_e13086, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18520_e13084 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18520_e13088;
        locals.var_rse_dn0 = assign18520_e13088_d_n0;
        locals.var_rse_dn2 = assign18520_e13088_d_n2;
        locals.var_rse_dn4 = assign18520_e13088_d_n4;
        locals.var_rse_dn5 = assign18520_e13088_d_n5;
        locals.var_rse_dn6 = assign18520_e13088_d_n6;
        locals.var_rse_dn7 = assign18520_e13088_d_n7;
        locals.var_rse_dn8 = assign18520_e13088_d_n8;
        locals.var_rse_dn9 = assign18520_e13088_d_n9;
        locals.var_rse_dn10 = assign18520_e13088_d_n10;
        locals.var_rse_dn11 = assign18520_e13088_d_n11;
        locals.var_rse_dn14 = assign18520_e13088_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18530_e13106, assign18530_e13106_d_n0, assign18530_e13106_d_n2, assign18530_e13106_d_n4, assign18530_e13106_d_n5, assign18530_e13106_d_n6, assign18530_e13106_d_n7, assign18530_e13106_d_n8, assign18530_e13106_d_n9, assign18530_e13106_d_n10, assign18530_e13106_d_n11, assign18530_e13106_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18530_e13099: f64 = (0.005 * locals.var_uc_rs);
        let assign18530_e13100: f64 = (locals.var_rse - assign18530_e13099);
        let assign18530_e13103: f64 = (0.01 * locals.var_uc_rs);
        let assign18530_e13104: f64 = (assign18530_e13100 - assign18530_e13103);
        (assign18530_e13104, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18530_e13106;
        locals.var_tmf1_dn0 = assign18530_e13106_d_n0;
        locals.var_tmf1_dn2 = assign18530_e13106_d_n2;
        locals.var_tmf1_dn4 = assign18530_e13106_d_n4;
        locals.var_tmf1_dn5 = assign18530_e13106_d_n5;
        locals.var_tmf1_dn6 = assign18530_e13106_d_n6;
        locals.var_tmf1_dn7 = assign18530_e13106_d_n7;
        locals.var_tmf1_dn8 = assign18530_e13106_d_n8;
        locals.var_tmf1_dn9 = assign18530_e13106_d_n9;
        locals.var_tmf1_dn10 = assign18530_e13106_d_n10;
        locals.var_tmf1_dn11 = assign18530_e13106_d_n11;
        locals.var_tmf1_dn14 = assign18530_e13106_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18540_e13124, assign18540_e13124_d_n0, assign18540_e13124_d_n2, assign18540_e13124_d_n4, assign18540_e13124_d_n5, assign18540_e13124_d_n6, assign18540_e13124_d_n7, assign18540_e13124_d_n8, assign18540_e13124_d_n9, assign18540_e13124_d_n10, assign18540_e13124_d_n11, assign18540_e13124_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18540_e13117: f64 = (0.005 * locals.var_uc_rs);
        let assign18540_e13118: f64 = (4.0 * assign18540_e13117);
        let assign18540_e13121: f64 = (0.01 * locals.var_uc_rs);
        let assign18540_e13122: f64 = (assign18540_e13118 * assign18540_e13121);
        (assign18540_e13122, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18540_e13124;
        locals.var_tmf2_dn0 = assign18540_e13124_d_n0;
        locals.var_tmf2_dn2 = assign18540_e13124_d_n2;
        locals.var_tmf2_dn4 = assign18540_e13124_d_n4;
        locals.var_tmf2_dn5 = assign18540_e13124_d_n5;
        locals.var_tmf2_dn6 = assign18540_e13124_d_n6;
        locals.var_tmf2_dn7 = assign18540_e13124_d_n7;
        locals.var_tmf2_dn8 = assign18540_e13124_d_n8;
        locals.var_tmf2_dn9 = assign18540_e13124_d_n9;
        locals.var_tmf2_dn10 = assign18540_e13124_d_n10;
        locals.var_tmf2_dn11 = assign18540_e13124_d_n11;
        locals.var_tmf2_dn14 = assign18540_e13124_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18550_e13140, assign18550_e13140_d_n0, assign18550_e13140_d_n2, assign18550_e13140_d_n4, assign18550_e13140_d_n5, assign18550_e13140_d_n6, assign18550_e13140_d_n7, assign18550_e13140_d_n8, assign18550_e13140_d_n9, assign18550_e13140_d_n10, assign18550_e13140_d_n11, assign18550_e13140_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let (assign18550_e13138, assign18550_e13138_d_n0, assign18550_e13138_d_n2, assign18550_e13138_d_n4, assign18550_e13138_d_n5, assign18550_e13138_d_n6, assign18550_e13138_d_n7, assign18550_e13138_d_n8, assign18550_e13138_d_n9, assign18550_e13138_d_n10, assign18550_e13138_d_n11, assign18550_e13138_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18550_e13137: f64 = (-locals.var_tmf2);
                (assign18550_e13137, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18550_e13138, assign18550_e13138_d_n0, assign18550_e13138_d_n2, assign18550_e13138_d_n4, assign18550_e13138_d_n5, assign18550_e13138_d_n6, assign18550_e13138_d_n7, assign18550_e13138_d_n8, assign18550_e13138_d_n9, assign18550_e13138_d_n10, assign18550_e13138_d_n11, assign18550_e13138_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18550_e13140;
        locals.var_tmf2_dn0 = assign18550_e13140_d_n0;
        locals.var_tmf2_dn2 = assign18550_e13140_d_n2;
        locals.var_tmf2_dn4 = assign18550_e13140_d_n4;
        locals.var_tmf2_dn5 = assign18550_e13140_d_n5;
        locals.var_tmf2_dn6 = assign18550_e13140_d_n6;
        locals.var_tmf2_dn7 = assign18550_e13140_d_n7;
        locals.var_tmf2_dn8 = assign18550_e13140_d_n8;
        locals.var_tmf2_dn9 = assign18550_e13140_d_n9;
        locals.var_tmf2_dn10 = assign18550_e13140_d_n10;
        locals.var_tmf2_dn11 = assign18550_e13140_d_n11;
        locals.var_tmf2_dn14 = assign18550_e13140_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18560_e13155, assign18560_e13155_d_n0, assign18560_e13155_d_n2, assign18560_e13155_d_n4, assign18560_e13155_d_n5, assign18560_e13155_d_n6, assign18560_e13155_d_n7, assign18560_e13155_d_n8, assign18560_e13155_d_n9, assign18560_e13155_d_n10, assign18560_e13155_d_n11, assign18560_e13155_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18560_e13150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18560_e13152: f64 = (assign18560_e13150 + locals.var_tmf2);
        let assign18560_e13153: f64 = (assign18560_e13152).sqrt();
        (assign18560_e13153, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18560_e13153)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18560_e13153)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18560_e13155;
        locals.var_tmf2_dn0 = assign18560_e13155_d_n0;
        locals.var_tmf2_dn2 = assign18560_e13155_d_n2;
        locals.var_tmf2_dn4 = assign18560_e13155_d_n4;
        locals.var_tmf2_dn5 = assign18560_e13155_d_n5;
        locals.var_tmf2_dn6 = assign18560_e13155_d_n6;
        locals.var_tmf2_dn7 = assign18560_e13155_d_n7;
        locals.var_tmf2_dn8 = assign18560_e13155_d_n8;
        locals.var_tmf2_dn9 = assign18560_e13155_d_n9;
        locals.var_tmf2_dn10 = assign18560_e13155_d_n10;
        locals.var_tmf2_dn11 = assign18560_e13155_d_n11;
        locals.var_tmf2_dn14 = assign18560_e13155_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18570_e13171, assign18570_e13171_d_n0, assign18570_e13171_d_n2, assign18570_e13171_d_n4, assign18570_e13171_d_n5, assign18570_e13171_d_n6, assign18570_e13171_d_n7, assign18570_e13171_d_n8, assign18570_e13171_d_n9, assign18570_e13171_d_n10, assign18570_e13171_d_n11, assign18570_e13171_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18570_e13167: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18570_e13168: f64 = (1.0 + assign18570_e13167);
        let assign18570_e13169: f64 = (0.5 * assign18570_e13168);
        (assign18570_e13169, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18570_e13171;
        locals.var_t0_dn0 = assign18570_e13171_d_n0;
        locals.var_t0_dn2 = assign18570_e13171_d_n2;
        locals.var_t0_dn4 = assign18570_e13171_d_n4;
        locals.var_t0_dn5 = assign18570_e13171_d_n5;
        locals.var_t0_dn6 = assign18570_e13171_d_n6;
        locals.var_t0_dn7 = assign18570_e13171_d_n7;
        locals.var_t0_dn8 = assign18570_e13171_d_n8;
        locals.var_t0_dn9 = assign18570_e13171_d_n9;
        locals.var_t0_dn10 = assign18570_e13171_d_n10;
        locals.var_t0_dn11 = assign18570_e13171_d_n11;
        locals.var_t0_dn14 = assign18570_e13171_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18580_e13189, assign18580_e13189_d_n0, assign18580_e13189_d_n2, assign18580_e13189_d_n4, assign18580_e13189_d_n5, assign18580_e13189_d_n6, assign18580_e13189_d_n7, assign18580_e13189_d_n8, assign18580_e13189_d_n9, assign18580_e13189_d_n10, assign18580_e13189_d_n11, assign18580_e13189_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign18580_e13181: f64 = (0.005 * locals.var_uc_rs);
        let assign18580_e13185: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18580_e13186: f64 = (0.5 * assign18580_e13185);
        let assign18580_e13187: f64 = (assign18580_e13181 + assign18580_e13186);
        (assign18580_e13187, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18580_e13189;
        locals.var_rse_dn0 = assign18580_e13189_d_n0;
        locals.var_rse_dn2 = assign18580_e13189_d_n2;
        locals.var_rse_dn4 = assign18580_e13189_d_n4;
        locals.var_rse_dn5 = assign18580_e13189_d_n5;
        locals.var_rse_dn6 = assign18580_e13189_d_n6;
        locals.var_rse_dn7 = assign18580_e13189_d_n7;
        locals.var_rse_dn8 = assign18580_e13189_d_n8;
        locals.var_rse_dn9 = assign18580_e13189_d_n9;
        locals.var_rse_dn10 = assign18580_e13189_d_n10;
        locals.var_rse_dn11 = assign18580_e13189_d_n11;
        locals.var_rse_dn14 = assign18580_e13189_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18590_e13210, assign18590_e13210_d_n0, assign18590_e13210_d_n2, assign18590_e13210_d_n4, assign18590_e13210_d_n5, assign18590_e13210_d_n6, assign18590_e13210_d_n7, assign18590_e13210_d_n8, assign18590_e13210_d_n9, assign18590_e13210_d_n10, assign18590_e13210_d_n11, assign18590_e13210_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18590_e13201: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18590_e13202: f64 = (locals.var_uc_rs + assign18590_e13201);
        let assign18590_e13205: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18590_e13206: f64 = (assign18590_e13202 + assign18590_e13205);
        let assign18590_e13208: f64 = (assign18590_e13206 * locals.var_t2);
        (assign18590_e13208, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18590_e13206 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18590_e13210;
        locals.var_rse_dn0 = assign18590_e13210_d_n0;
        locals.var_rse_dn2 = assign18590_e13210_d_n2;
        locals.var_rse_dn4 = assign18590_e13210_d_n4;
        locals.var_rse_dn5 = assign18590_e13210_d_n5;
        locals.var_rse_dn6 = assign18590_e13210_d_n6;
        locals.var_rse_dn7 = assign18590_e13210_d_n7;
        locals.var_rse_dn8 = assign18590_e13210_d_n8;
        locals.var_rse_dn9 = assign18590_e13210_d_n9;
        locals.var_rse_dn10 = assign18590_e13210_d_n10;
        locals.var_rse_dn11 = assign18590_e13210_d_n11;
        locals.var_rse_dn14 = assign18590_e13210_d_n14;
        locals.var_rse_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18600_e13229, assign18600_e13229_d_n0, assign18600_e13229_d_n2, assign18600_e13229_d_n4, assign18600_e13229_d_n5, assign18600_e13229_d_n6, assign18600_e13229_d_n7, assign18600_e13229_d_n8, assign18600_e13229_d_n9, assign18600_e13229_d_n10, assign18600_e13229_d_n11, assign18600_e13229_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18600_e13222: f64 = (0.005 * locals.var_uc_rs);
        let assign18600_e13223: f64 = (locals.var_rse - assign18600_e13222);
        let assign18600_e13226: f64 = (0.01 * locals.var_uc_rs);
        let assign18600_e13227: f64 = (assign18600_e13223 - assign18600_e13226);
        (assign18600_e13227, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18600_e13229;
        locals.var_tmf1_dn0 = assign18600_e13229_d_n0;
        locals.var_tmf1_dn2 = assign18600_e13229_d_n2;
        locals.var_tmf1_dn4 = assign18600_e13229_d_n4;
        locals.var_tmf1_dn5 = assign18600_e13229_d_n5;
        locals.var_tmf1_dn6 = assign18600_e13229_d_n6;
        locals.var_tmf1_dn7 = assign18600_e13229_d_n7;
        locals.var_tmf1_dn8 = assign18600_e13229_d_n8;
        locals.var_tmf1_dn9 = assign18600_e13229_d_n9;
        locals.var_tmf1_dn10 = assign18600_e13229_d_n10;
        locals.var_tmf1_dn11 = assign18600_e13229_d_n11;
        locals.var_tmf1_dn14 = assign18600_e13229_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18610_e13248, assign18610_e13248_d_n0, assign18610_e13248_d_n2, assign18610_e13248_d_n4, assign18610_e13248_d_n5, assign18610_e13248_d_n6, assign18610_e13248_d_n7, assign18610_e13248_d_n8, assign18610_e13248_d_n9, assign18610_e13248_d_n10, assign18610_e13248_d_n11, assign18610_e13248_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18610_e13241: f64 = (0.005 * locals.var_uc_rs);
        let assign18610_e13242: f64 = (4.0 * assign18610_e13241);
        let assign18610_e13245: f64 = (0.01 * locals.var_uc_rs);
        let assign18610_e13246: f64 = (assign18610_e13242 * assign18610_e13245);
        (assign18610_e13246, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18610_e13248;
        locals.var_tmf2_dn0 = assign18610_e13248_d_n0;
        locals.var_tmf2_dn2 = assign18610_e13248_d_n2;
        locals.var_tmf2_dn4 = assign18610_e13248_d_n4;
        locals.var_tmf2_dn5 = assign18610_e13248_d_n5;
        locals.var_tmf2_dn6 = assign18610_e13248_d_n6;
        locals.var_tmf2_dn7 = assign18610_e13248_d_n7;
        locals.var_tmf2_dn8 = assign18610_e13248_d_n8;
        locals.var_tmf2_dn9 = assign18610_e13248_d_n9;
        locals.var_tmf2_dn10 = assign18610_e13248_d_n10;
        locals.var_tmf2_dn11 = assign18610_e13248_d_n11;
        locals.var_tmf2_dn14 = assign18610_e13248_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18620_e13265, assign18620_e13265_d_n0, assign18620_e13265_d_n2, assign18620_e13265_d_n4, assign18620_e13265_d_n5, assign18620_e13265_d_n6, assign18620_e13265_d_n7, assign18620_e13265_d_n8, assign18620_e13265_d_n9, assign18620_e13265_d_n10, assign18620_e13265_d_n11, assign18620_e13265_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let (assign18620_e13263, assign18620_e13263_d_n0, assign18620_e13263_d_n2, assign18620_e13263_d_n4, assign18620_e13263_d_n5, assign18620_e13263_d_n6, assign18620_e13263_d_n7, assign18620_e13263_d_n8, assign18620_e13263_d_n9, assign18620_e13263_d_n10, assign18620_e13263_d_n11, assign18620_e13263_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18620_e13262: f64 = (-locals.var_tmf2);
                (assign18620_e13262, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18620_e13263, assign18620_e13263_d_n0, assign18620_e13263_d_n2, assign18620_e13263_d_n4, assign18620_e13263_d_n5, assign18620_e13263_d_n6, assign18620_e13263_d_n7, assign18620_e13263_d_n8, assign18620_e13263_d_n9, assign18620_e13263_d_n10, assign18620_e13263_d_n11, assign18620_e13263_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18620_e13265;
        locals.var_tmf2_dn0 = assign18620_e13265_d_n0;
        locals.var_tmf2_dn2 = assign18620_e13265_d_n2;
        locals.var_tmf2_dn4 = assign18620_e13265_d_n4;
        locals.var_tmf2_dn5 = assign18620_e13265_d_n5;
        locals.var_tmf2_dn6 = assign18620_e13265_d_n6;
        locals.var_tmf2_dn7 = assign18620_e13265_d_n7;
        locals.var_tmf2_dn8 = assign18620_e13265_d_n8;
        locals.var_tmf2_dn9 = assign18620_e13265_d_n9;
        locals.var_tmf2_dn10 = assign18620_e13265_d_n10;
        locals.var_tmf2_dn11 = assign18620_e13265_d_n11;
        locals.var_tmf2_dn14 = assign18620_e13265_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18630_e13281, assign18630_e13281_d_n0, assign18630_e13281_d_n2, assign18630_e13281_d_n4, assign18630_e13281_d_n5, assign18630_e13281_d_n6, assign18630_e13281_d_n7, assign18630_e13281_d_n8, assign18630_e13281_d_n9, assign18630_e13281_d_n10, assign18630_e13281_d_n11, assign18630_e13281_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18630_e13276: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18630_e13278: f64 = (assign18630_e13276 + locals.var_tmf2);
        let assign18630_e13279: f64 = (assign18630_e13278).sqrt();
        (assign18630_e13279, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18630_e13279)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18630_e13279)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18630_e13281;
        locals.var_tmf2_dn0 = assign18630_e13281_d_n0;
        locals.var_tmf2_dn2 = assign18630_e13281_d_n2;
        locals.var_tmf2_dn4 = assign18630_e13281_d_n4;
        locals.var_tmf2_dn5 = assign18630_e13281_d_n5;
        locals.var_tmf2_dn6 = assign18630_e13281_d_n6;
        locals.var_tmf2_dn7 = assign18630_e13281_d_n7;
        locals.var_tmf2_dn8 = assign18630_e13281_d_n8;
        locals.var_tmf2_dn9 = assign18630_e13281_d_n9;
        locals.var_tmf2_dn10 = assign18630_e13281_d_n10;
        locals.var_tmf2_dn11 = assign18630_e13281_d_n11;
        locals.var_tmf2_dn14 = assign18630_e13281_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18640_e13298, assign18640_e13298_d_n0, assign18640_e13298_d_n2, assign18640_e13298_d_n4, assign18640_e13298_d_n5, assign18640_e13298_d_n6, assign18640_e13298_d_n7, assign18640_e13298_d_n8, assign18640_e13298_d_n9, assign18640_e13298_d_n10, assign18640_e13298_d_n11, assign18640_e13298_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18640_e13294: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18640_e13295: f64 = (1.0 + assign18640_e13294);
        let assign18640_e13296: f64 = (0.5 * assign18640_e13295);
        (assign18640_e13296, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18640_e13298;
        locals.var_t0_dn0 = assign18640_e13298_d_n0;
        locals.var_t0_dn2 = assign18640_e13298_d_n2;
        locals.var_t0_dn4 = assign18640_e13298_d_n4;
        locals.var_t0_dn5 = assign18640_e13298_d_n5;
        locals.var_t0_dn6 = assign18640_e13298_d_n6;
        locals.var_t0_dn7 = assign18640_e13298_d_n7;
        locals.var_t0_dn8 = assign18640_e13298_d_n8;
        locals.var_t0_dn9 = assign18640_e13298_d_n9;
        locals.var_t0_dn10 = assign18640_e13298_d_n10;
        locals.var_t0_dn11 = assign18640_e13298_d_n11;
        locals.var_t0_dn14 = assign18640_e13298_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18650_e13317, assign18650_e13317_d_n0, assign18650_e13317_d_n2, assign18650_e13317_d_n4, assign18650_e13317_d_n5, assign18650_e13317_d_n6, assign18650_e13317_d_n7, assign18650_e13317_d_n8, assign18650_e13317_d_n9, assign18650_e13317_d_n10, assign18650_e13317_d_n11, assign18650_e13317_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign18650_e13309: f64 = (0.005 * locals.var_uc_rs);
        let assign18650_e13313: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18650_e13314: f64 = (0.5 * assign18650_e13313);
        let assign18650_e13315: f64 = (assign18650_e13309 + assign18650_e13314);
        (assign18650_e13315, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18650_e13317;
        locals.var_rse_dn0 = assign18650_e13317_d_n0;
        locals.var_rse_dn2 = assign18650_e13317_d_n2;
        locals.var_rse_dn4 = assign18650_e13317_d_n4;
        locals.var_rse_dn5 = assign18650_e13317_d_n5;
        locals.var_rse_dn6 = assign18650_e13317_d_n6;
        locals.var_rse_dn7 = assign18650_e13317_d_n7;
        locals.var_rse_dn8 = assign18650_e13317_d_n8;
        locals.var_rse_dn9 = assign18650_e13317_d_n9;
        locals.var_rse_dn10 = assign18650_e13317_d_n10;
        locals.var_rse_dn11 = assign18650_e13317_d_n11;
        locals.var_rse_dn14 = assign18650_e13317_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18660_e13326, assign18660_e13326_d_n0, assign18660_e13326_d_n2, assign18660_e13326_d_n4, assign18660_e13326_d_n5, assign18660_e13326_d_n6, assign18660_e13326_d_n7, assign18660_e13326_d_n8, assign18660_e13326_d_n9, assign18660_e13326_d_n10, assign18660_e13326_d_n11, assign18660_e13326_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18660_e13326;
        locals.var_rse_dn0 = assign18660_e13326_d_n0;
        locals.var_rse_dn2 = assign18660_e13326_d_n2;
        locals.var_rse_dn4 = assign18660_e13326_d_n4;
        locals.var_rse_dn5 = assign18660_e13326_d_n5;
        locals.var_rse_dn6 = assign18660_e13326_d_n6;
        locals.var_rse_dn7 = assign18660_e13326_d_n7;
        locals.var_rse_dn8 = assign18660_e13326_d_n8;
        locals.var_rse_dn9 = assign18660_e13326_d_n9;
        locals.var_rse_dn10 = assign18660_e13326_d_n10;
        locals.var_rse_dn11 = assign18660_e13326_d_n11;
        locals.var_rse_dn14 = assign18660_e13326_d_n14;
        locals.var_rse_rv = 0.0;

        let assign18670_e13329: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign18670_e13329;
        locals.var_guard387_rv = 0.0;

        let (assign18680_e13353, assign18680_e13353_d_n0, assign18680_e13353_d_n2, assign18680_e13353_d_n4, assign18680_e13353_d_n5, assign18680_e13353_d_n6, assign18680_e13353_d_n7, assign18680_e13353_d_n8, assign18680_e13353_d_n9, assign18680_e13353_d_n10, assign18680_e13353_d_n11, assign18680_e13353_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18680_e13338: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18680_e13340: f64 = (assign18680_e13338 * 1000000.0);
        let assign18680_e13342: f64 = (assign18680_e13340 + locals.var_uc_rdict1);
        let assign18680_e13343: f64 = (locals.var_rdvdtemp0 * assign18680_e13342);
        let assign18680_e13346: f64 = (p.p68 * p.p100);
        let assign18680_e13348: f64 = (assign18680_e13346 * 1000000.0);
        let assign18680_e13350: f64 = (assign18680_e13348 + p.p101);
        let assign18680_e13351: f64 = (assign18680_e13343 * assign18680_e13350);
        (assign18680_e13351, ((locals.var_rdvdtemp0_dn0 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn2 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn4 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn5 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn6 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn7 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn8 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn9 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn10 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn11 * assign18680_e13342) * assign18680_e13350), ((locals.var_rdvdtemp0_dn14 * assign18680_e13342) * assign18680_e13350),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign18680_e13353;
        locals.var_t4_dn0 = assign18680_e13353_d_n0;
        locals.var_t4_dn2 = assign18680_e13353_d_n2;
        locals.var_t4_dn4 = assign18680_e13353_d_n4;
        locals.var_t4_dn5 = assign18680_e13353_d_n5;
        locals.var_t4_dn6 = assign18680_e13353_d_n6;
        locals.var_t4_dn7 = assign18680_e13353_d_n7;
        locals.var_t4_dn8 = assign18680_e13353_d_n8;
        locals.var_t4_dn9 = assign18680_e13353_d_n9;
        locals.var_t4_dn10 = assign18680_e13353_d_n10;
        locals.var_t4_dn11 = assign18680_e13353_d_n11;
        locals.var_t4_dn14 = assign18680_e13353_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign18690_e13367, assign18690_e13367_d_n0, assign18690_e13367_d_n2, assign18690_e13367_d_n4, assign18690_e13367_d_n5, assign18690_e13367_d_n6, assign18690_e13367_d_n7, assign18690_e13367_d_n8, assign18690_e13367_d_n9, assign18690_e13367_d_n10, assign18690_e13367_d_n11, assign18690_e13367_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18690_e13361: f64 = (1.0 - locals.var_uc_rdov13);
        let assign18690_e13363: f64 = (assign18690_e13361 * p.p63);
        let assign18690_e13365: f64 = (assign18690_e13363 * 1000000.0);
        (assign18690_e13365, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18690_e13367;
        locals.var_t1_dn0 = assign18690_e13367_d_n0;
        locals.var_t1_dn2 = assign18690_e13367_d_n2;
        locals.var_t1_dn4 = assign18690_e13367_d_n4;
        locals.var_t1_dn5 = assign18690_e13367_d_n5;
        locals.var_t1_dn6 = assign18690_e13367_d_n6;
        locals.var_t1_dn7 = assign18690_e13367_d_n7;
        locals.var_t1_dn8 = assign18690_e13367_d_n8;
        locals.var_t1_dn9 = assign18690_e13367_d_n9;
        locals.var_t1_dn10 = assign18690_e13367_d_n10;
        locals.var_t1_dn11 = assign18690_e13367_d_n11;
        locals.var_t1_dn14 = assign18690_e13367_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18700_e13388, assign18700_e13388_d_n0, assign18700_e13388_d_n2, assign18700_e13388_d_n4, assign18700_e13388_d_n5, assign18700_e13388_d_n6, assign18700_e13388_d_n7, assign18700_e13388_d_n8, assign18700_e13388_d_n9, assign18700_e13388_d_n10, assign18700_e13388_d_n11, assign18700_e13388_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18700_e13375: f64 = (p.p99 * p.p99);
        let assign18700_e13379: f64 = (0.0001 * 0.01);
        let assign18700_e13380: f64 = (4.0 * assign18700_e13379);
        let assign18700_e13383: f64 = (0.0001 * 0.01);
        let assign18700_e13384: f64 = (assign18700_e13380 * assign18700_e13383);
        let assign18700_e13385: f64 = (assign18700_e13375 + assign18700_e13384);
        let assign18700_e13386: f64 = (assign18700_e13385).sqrt();
        (assign18700_e13386, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18700_e13388;
        locals.var_tmf2_dn0 = assign18700_e13388_d_n0;
        locals.var_tmf2_dn2 = assign18700_e13388_d_n2;
        locals.var_tmf2_dn4 = assign18700_e13388_d_n4;
        locals.var_tmf2_dn5 = assign18700_e13388_d_n5;
        locals.var_tmf2_dn6 = assign18700_e13388_d_n6;
        locals.var_tmf2_dn7 = assign18700_e13388_d_n7;
        locals.var_tmf2_dn8 = assign18700_e13388_d_n8;
        locals.var_tmf2_dn9 = assign18700_e13388_d_n9;
        locals.var_tmf2_dn10 = assign18700_e13388_d_n10;
        locals.var_tmf2_dn11 = assign18700_e13388_d_n11;
        locals.var_tmf2_dn14 = assign18700_e13388_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18710_e13402, assign18710_e13402_d_n0, assign18710_e13402_d_n2, assign18710_e13402_d_n4, assign18710_e13402_d_n5, assign18710_e13402_d_n6, assign18710_e13402_d_n7, assign18710_e13402_d_n8, assign18710_e13402_d_n9, assign18710_e13402_d_n10, assign18710_e13402_d_n11, assign18710_e13402_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18710_e13398: f64 = (p.p99 / locals.var_tmf2);
        let assign18710_e13399: f64 = (1.0 + assign18710_e13398);
        let assign18710_e13400: f64 = (0.5 * assign18710_e13399);
        (assign18710_e13400, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18710_e13402;
        locals.var_t0_dn0 = assign18710_e13402_d_n0;
        locals.var_t0_dn2 = assign18710_e13402_d_n2;
        locals.var_t0_dn4 = assign18710_e13402_d_n4;
        locals.var_t0_dn5 = assign18710_e13402_d_n5;
        locals.var_t0_dn6 = assign18710_e13402_d_n6;
        locals.var_t0_dn7 = assign18710_e13402_d_n7;
        locals.var_t0_dn8 = assign18710_e13402_d_n8;
        locals.var_t0_dn9 = assign18710_e13402_d_n9;
        locals.var_t0_dn10 = assign18710_e13402_d_n10;
        locals.var_t0_dn11 = assign18710_e13402_d_n11;
        locals.var_t0_dn14 = assign18710_e13402_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18720_e13414, assign18720_e13414_d_n0, assign18720_e13414_d_n2, assign18720_e13414_d_n4, assign18720_e13414_d_n5, assign18720_e13414_d_n6, assign18720_e13414_d_n7, assign18720_e13414_d_n8, assign18720_e13414_d_n9, assign18720_e13414_d_n10, assign18720_e13414_d_n11, assign18720_e13414_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18720_e13411: f64 = (p.p99 + locals.var_tmf2);
        let assign18720_e13412: f64 = (0.5 * assign18720_e13411);
        (assign18720_e13412, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18720_e13414;
        locals.var_t2_dn0 = assign18720_e13414_d_n0;
        locals.var_t2_dn2 = assign18720_e13414_d_n2;
        locals.var_t2_dn4 = assign18720_e13414_d_n4;
        locals.var_t2_dn5 = assign18720_e13414_d_n5;
        locals.var_t2_dn6 = assign18720_e13414_d_n6;
        locals.var_t2_dn7 = assign18720_e13414_d_n7;
        locals.var_t2_dn8 = assign18720_e13414_d_n8;
        locals.var_t2_dn9 = assign18720_e13414_d_n9;
        locals.var_t2_dn10 = assign18720_e13414_d_n10;
        locals.var_t2_dn11 = assign18720_e13414_d_n11;
        locals.var_t2_dn14 = assign18720_e13414_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18730_e13417: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign18730_e13417;
        locals.var_guard388_rv = 0.0;

        let (assign18740_e13427, assign18740_e13427_d_n0, assign18740_e13427_d_n2, assign18740_e13427_d_n4, assign18740_e13427_d_n5, assign18740_e13427_d_n6, assign18740_e13427_d_n7, assign18740_e13427_d_n8, assign18740_e13427_d_n9, assign18740_e13427_d_n10, assign18740_e13427_d_n11, assign18740_e13427_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18740_e13427;
        locals.var_t2_dn0 = assign18740_e13427_d_n0;
        locals.var_t2_dn2 = assign18740_e13427_d_n2;
        locals.var_t2_dn4 = assign18740_e13427_d_n4;
        locals.var_t2_dn5 = assign18740_e13427_d_n5;
        locals.var_t2_dn6 = assign18740_e13427_d_n6;
        locals.var_t2_dn7 = assign18740_e13427_d_n7;
        locals.var_t2_dn8 = assign18740_e13427_d_n8;
        locals.var_t2_dn9 = assign18740_e13427_d_n9;
        locals.var_t2_dn10 = assign18740_e13427_d_n10;
        locals.var_t2_dn11 = assign18740_e13427_d_n11;
        locals.var_t2_dn14 = assign18740_e13427_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign18750_e13437, assign18750_e13437_d_n0, assign18750_e13437_d_n2, assign18750_e13437_d_n4, assign18750_e13437_d_n5, assign18750_e13437_d_n6, assign18750_e13437_d_n7, assign18750_e13437_d_n8, assign18750_e13437_d_n9, assign18750_e13437_d_n10, assign18750_e13437_d_n11, assign18750_e13437_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18750_e13437;
        locals.var_t0_dn0 = assign18750_e13437_d_n0;
        locals.var_t0_dn2 = assign18750_e13437_d_n2;
        locals.var_t0_dn4 = assign18750_e13437_d_n4;
        locals.var_t0_dn5 = assign18750_e13437_d_n5;
        locals.var_t0_dn6 = assign18750_e13437_d_n6;
        locals.var_t0_dn7 = assign18750_e13437_d_n7;
        locals.var_t0_dn8 = assign18750_e13437_d_n8;
        locals.var_t0_dn9 = assign18750_e13437_d_n9;
        locals.var_t0_dn10 = assign18750_e13437_d_n10;
        locals.var_t0_dn11 = assign18750_e13437_d_n11;
        locals.var_t0_dn14 = assign18750_e13437_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18760_e13448, assign18760_e13448_d_n0, assign18760_e13448_d_n2, assign18760_e13448_d_n4, assign18760_e13448_d_n5, assign18760_e13448_d_n6, assign18760_e13448_d_n7, assign18760_e13448_d_n8, assign18760_e13448_d_n9, assign18760_e13448_d_n10, assign18760_e13448_d_n11, assign18760_e13448_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18760_e13444: f64 = (-p.p98);
        let assign18760_e13446: f64 = (assign18760_e13444 / locals.var_t2);
        (assign18760_e13446, (-((assign18760_e13444 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign18760_e13444 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign18760_e13448;
        locals.var_t8_dn0 = assign18760_e13448_d_n0;
        locals.var_t8_dn2 = assign18760_e13448_d_n2;
        locals.var_t8_dn4 = assign18760_e13448_d_n4;
        locals.var_t8_dn5 = assign18760_e13448_d_n5;
        locals.var_t8_dn6 = assign18760_e13448_d_n6;
        locals.var_t8_dn7 = assign18760_e13448_d_n7;
        locals.var_t8_dn8 = assign18760_e13448_d_n8;
        locals.var_t8_dn9 = assign18760_e13448_d_n9;
        locals.var_t8_dn10 = assign18760_e13448_d_n10;
        locals.var_t8_dn11 = assign18760_e13448_d_n11;
        locals.var_t8_dn14 = assign18760_e13448_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign18770_e13464, assign18770_e13464_d_n0, assign18770_e13464_d_n2, assign18770_e13464_d_n4, assign18770_e13464_d_n5, assign18770_e13464_d_n6, assign18770_e13464_d_n7, assign18770_e13464_d_n8, assign18770_e13464_d_n9, assign18770_e13464_d_n10, assign18770_e13464_d_n11, assign18770_e13464_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18770_e13456: f64 = (locals.var_t8 * p.p63);
        let assign18770_e13458: f64 = (assign18770_e13456 * 1000000.0);
        let assign18770_e13460: f64 = (assign18770_e13458 + 1.0);
        let assign18770_e13462: f64 = (assign18770_e13460 + p.p98);
        (assign18770_e13462, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn11 * p.p63) * 1000000.0), ((locals.var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign18770_e13464;
        locals.var_t3_dn0 = assign18770_e13464_d_n0;
        locals.var_t3_dn2 = assign18770_e13464_d_n2;
        locals.var_t3_dn4 = assign18770_e13464_d_n4;
        locals.var_t3_dn5 = assign18770_e13464_d_n5;
        locals.var_t3_dn6 = assign18770_e13464_d_n6;
        locals.var_t3_dn7 = assign18770_e13464_d_n7;
        locals.var_t3_dn8 = assign18770_e13464_d_n8;
        locals.var_t3_dn9 = assign18770_e13464_d_n9;
        locals.var_t3_dn10 = assign18770_e13464_d_n10;
        locals.var_t3_dn11 = assign18770_e13464_d_n11;
        locals.var_t3_dn14 = assign18770_e13464_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign18780_e13478, assign18780_e13478_d_n0, assign18780_e13478_d_n2, assign18780_e13478_d_n4, assign18780_e13478_d_n5, assign18780_e13478_d_n6, assign18780_e13478_d_n7, assign18780_e13478_d_n8, assign18780_e13478_d_n9, assign18780_e13478_d_n10, assign18780_e13478_d_n11, assign18780_e13478_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18780_e13472: f64 = (locals.var_t3 * locals.var_t4);
        let assign18780_e13474: f64 = (assign18780_e13472 - locals.var_t4);
        let assign18780_e13476: f64 = (assign18780_e13474 - 0.01);
        (assign18780_e13476, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18780_e13478;
        locals.var_tmf1_dn0 = assign18780_e13478_d_n0;
        locals.var_tmf1_dn2 = assign18780_e13478_d_n2;
        locals.var_tmf1_dn4 = assign18780_e13478_d_n4;
        locals.var_tmf1_dn5 = assign18780_e13478_d_n5;
        locals.var_tmf1_dn6 = assign18780_e13478_d_n6;
        locals.var_tmf1_dn7 = assign18780_e13478_d_n7;
        locals.var_tmf1_dn8 = assign18780_e13478_d_n8;
        locals.var_tmf1_dn9 = assign18780_e13478_d_n9;
        locals.var_tmf1_dn10 = assign18780_e13478_d_n10;
        locals.var_tmf1_dn11 = assign18780_e13478_d_n11;
        locals.var_tmf1_dn14 = assign18780_e13478_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18790_e13490, assign18790_e13490_d_n0, assign18790_e13490_d_n2, assign18790_e13490_d_n4, assign18790_e13490_d_n5, assign18790_e13490_d_n6, assign18790_e13490_d_n7, assign18790_e13490_d_n8, assign18790_e13490_d_n9, assign18790_e13490_d_n10, assign18790_e13490_d_n11, assign18790_e13490_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18790_e13486: f64 = (4.0 * locals.var_t4);
        let assign18790_e13488: f64 = (assign18790_e13486 * 0.01);
        (assign18790_e13488, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18790_e13490;
        locals.var_tmf2_dn0 = assign18790_e13490_d_n0;
        locals.var_tmf2_dn2 = assign18790_e13490_d_n2;
        locals.var_tmf2_dn4 = assign18790_e13490_d_n4;
        locals.var_tmf2_dn5 = assign18790_e13490_d_n5;
        locals.var_tmf2_dn6 = assign18790_e13490_d_n6;
        locals.var_tmf2_dn7 = assign18790_e13490_d_n7;
        locals.var_tmf2_dn8 = assign18790_e13490_d_n8;
        locals.var_tmf2_dn9 = assign18790_e13490_d_n9;
        locals.var_tmf2_dn10 = assign18790_e13490_d_n10;
        locals.var_tmf2_dn11 = assign18790_e13490_d_n11;
        locals.var_tmf2_dn14 = assign18790_e13490_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18800_e13504, assign18800_e13504_d_n0, assign18800_e13504_d_n2, assign18800_e13504_d_n4, assign18800_e13504_d_n5, assign18800_e13504_d_n6, assign18800_e13504_d_n7, assign18800_e13504_d_n8, assign18800_e13504_d_n9, assign18800_e13504_d_n10, assign18800_e13504_d_n11, assign18800_e13504_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18800_e13502, assign18800_e13502_d_n0, assign18800_e13502_d_n2, assign18800_e13502_d_n4, assign18800_e13502_d_n5, assign18800_e13502_d_n6, assign18800_e13502_d_n7, assign18800_e13502_d_n8, assign18800_e13502_d_n9, assign18800_e13502_d_n10, assign18800_e13502_d_n11, assign18800_e13502_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18800_e13501: f64 = (-locals.var_tmf2);
                (assign18800_e13501, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18800_e13502, assign18800_e13502_d_n0, assign18800_e13502_d_n2, assign18800_e13502_d_n4, assign18800_e13502_d_n5, assign18800_e13502_d_n6, assign18800_e13502_d_n7, assign18800_e13502_d_n8, assign18800_e13502_d_n9, assign18800_e13502_d_n10, assign18800_e13502_d_n11, assign18800_e13502_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18800_e13504;
        locals.var_tmf2_dn0 = assign18800_e13504_d_n0;
        locals.var_tmf2_dn2 = assign18800_e13504_d_n2;
        locals.var_tmf2_dn4 = assign18800_e13504_d_n4;
        locals.var_tmf2_dn5 = assign18800_e13504_d_n5;
        locals.var_tmf2_dn6 = assign18800_e13504_d_n6;
        locals.var_tmf2_dn7 = assign18800_e13504_d_n7;
        locals.var_tmf2_dn8 = assign18800_e13504_d_n8;
        locals.var_tmf2_dn9 = assign18800_e13504_d_n9;
        locals.var_tmf2_dn10 = assign18800_e13504_d_n10;
        locals.var_tmf2_dn11 = assign18800_e13504_d_n11;
        locals.var_tmf2_dn14 = assign18800_e13504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18810_e13517, assign18810_e13517_d_n0, assign18810_e13517_d_n2, assign18810_e13517_d_n4, assign18810_e13517_d_n5, assign18810_e13517_d_n6, assign18810_e13517_d_n7, assign18810_e13517_d_n8, assign18810_e13517_d_n9, assign18810_e13517_d_n10, assign18810_e13517_d_n11, assign18810_e13517_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18810_e13512: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18810_e13514: f64 = (assign18810_e13512 + locals.var_tmf2);
        let assign18810_e13515: f64 = (assign18810_e13514).sqrt();
        (assign18810_e13515, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18810_e13515)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18810_e13515)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18810_e13517;
        locals.var_tmf2_dn0 = assign18810_e13517_d_n0;
        locals.var_tmf2_dn2 = assign18810_e13517_d_n2;
        locals.var_tmf2_dn4 = assign18810_e13517_d_n4;
        locals.var_tmf2_dn5 = assign18810_e13517_d_n5;
        locals.var_tmf2_dn6 = assign18810_e13517_d_n6;
        locals.var_tmf2_dn7 = assign18810_e13517_d_n7;
        locals.var_tmf2_dn8 = assign18810_e13517_d_n8;
        locals.var_tmf2_dn9 = assign18810_e13517_d_n9;
        locals.var_tmf2_dn10 = assign18810_e13517_d_n10;
        locals.var_tmf2_dn11 = assign18810_e13517_d_n11;
        locals.var_tmf2_dn14 = assign18810_e13517_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18820_e13531, assign18820_e13531_d_n0, assign18820_e13531_d_n2, assign18820_e13531_d_n4, assign18820_e13531_d_n5, assign18820_e13531_d_n6, assign18820_e13531_d_n7, assign18820_e13531_d_n8, assign18820_e13531_d_n9, assign18820_e13531_d_n10, assign18820_e13531_d_n11, assign18820_e13531_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18820_e13527: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18820_e13528: f64 = (1.0 + assign18820_e13527);
        let assign18820_e13529: f64 = (0.5 * assign18820_e13528);
        (assign18820_e13529, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18820_e13531;
        locals.var_t6_dn0 = assign18820_e13531_d_n0;
        locals.var_t6_dn2 = assign18820_e13531_d_n2;
        locals.var_t6_dn4 = assign18820_e13531_d_n4;
        locals.var_t6_dn5 = assign18820_e13531_d_n5;
        locals.var_t6_dn6 = assign18820_e13531_d_n6;
        locals.var_t6_dn7 = assign18820_e13531_d_n7;
        locals.var_t6_dn8 = assign18820_e13531_d_n8;
        locals.var_t6_dn9 = assign18820_e13531_d_n9;
        locals.var_t6_dn10 = assign18820_e13531_d_n10;
        locals.var_t6_dn11 = assign18820_e13531_d_n11;
        locals.var_t6_dn14 = assign18820_e13531_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18830_e13545, assign18830_e13545_d_n0, assign18830_e13545_d_n2, assign18830_e13545_d_n4, assign18830_e13545_d_n5, assign18830_e13545_d_n6, assign18830_e13545_d_n7, assign18830_e13545_d_n8, assign18830_e13545_d_n9, assign18830_e13545_d_n10, assign18830_e13545_d_n11, assign18830_e13545_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18830_e13541: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18830_e13542: f64 = (0.5 * assign18830_e13541);
        let assign18830_e13543: f64 = (locals.var_t4 + assign18830_e13542);
        (assign18830_e13543, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign18830_e13545;
        locals.var_t5_dn0 = assign18830_e13545_d_n0;
        locals.var_t5_dn2 = assign18830_e13545_d_n2;
        locals.var_t5_dn4 = assign18830_e13545_d_n4;
        locals.var_t5_dn5 = assign18830_e13545_d_n5;
        locals.var_t5_dn6 = assign18830_e13545_d_n6;
        locals.var_t5_dn7 = assign18830_e13545_d_n7;
        locals.var_t5_dn8 = assign18830_e13545_d_n8;
        locals.var_t5_dn9 = assign18830_e13545_d_n9;
        locals.var_t5_dn10 = assign18830_e13545_d_n10;
        locals.var_t5_dn11 = assign18830_e13545_d_n11;
        locals.var_t5_dn14 = assign18830_e13545_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign18840_e13561, assign18840_e13561_d_n0, assign18840_e13561_d_n2, assign18840_e13561_d_n4, assign18840_e13561_d_n5, assign18840_e13561_d_n6, assign18840_e13561_d_n7, assign18840_e13561_d_n8, assign18840_e13561_d_n9, assign18840_e13561_d_n10, assign18840_e13561_d_n11, assign18840_e13561_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18840_e13554: f64 = (p.p98 + 1.0);
        let assign18840_e13555: f64 = (locals.var_t4 * assign18840_e13554);
        let assign18840_e13557: f64 = (assign18840_e13555 - locals.var_t5);
        let assign18840_e13559: f64 = (assign18840_e13557 - 5e-5);
        (assign18840_e13559, ((locals.var_t4_dn0 * assign18840_e13554) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign18840_e13554) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign18840_e13554) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign18840_e13554) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign18840_e13554) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign18840_e13554) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign18840_e13554) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign18840_e13554) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign18840_e13554) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign18840_e13554) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign18840_e13554) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18840_e13561;
        locals.var_tmf1_dn0 = assign18840_e13561_d_n0;
        locals.var_tmf1_dn2 = assign18840_e13561_d_n2;
        locals.var_tmf1_dn4 = assign18840_e13561_d_n4;
        locals.var_tmf1_dn5 = assign18840_e13561_d_n5;
        locals.var_tmf1_dn6 = assign18840_e13561_d_n6;
        locals.var_tmf1_dn7 = assign18840_e13561_d_n7;
        locals.var_tmf1_dn8 = assign18840_e13561_d_n8;
        locals.var_tmf1_dn9 = assign18840_e13561_d_n9;
        locals.var_tmf1_dn10 = assign18840_e13561_d_n10;
        locals.var_tmf1_dn11 = assign18840_e13561_d_n11;
        locals.var_tmf1_dn14 = assign18840_e13561_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18850_e13577, assign18850_e13577_d_n0, assign18850_e13577_d_n2, assign18850_e13577_d_n4, assign18850_e13577_d_n5, assign18850_e13577_d_n6, assign18850_e13577_d_n7, assign18850_e13577_d_n8, assign18850_e13577_d_n9, assign18850_e13577_d_n10, assign18850_e13577_d_n11, assign18850_e13577_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18850_e13571: f64 = (p.p98 + 1.0);
        let assign18850_e13572: f64 = (locals.var_t4 * assign18850_e13571);
        let assign18850_e13573: f64 = (4.0 * assign18850_e13572);
        let assign18850_e13575: f64 = (assign18850_e13573 * 5e-5);
        (assign18850_e13575, ((4.0 * (locals.var_t4_dn0 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign18850_e13571)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign18850_e13571)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18850_e13577;
        locals.var_tmf2_dn0 = assign18850_e13577_d_n0;
        locals.var_tmf2_dn2 = assign18850_e13577_d_n2;
        locals.var_tmf2_dn4 = assign18850_e13577_d_n4;
        locals.var_tmf2_dn5 = assign18850_e13577_d_n5;
        locals.var_tmf2_dn6 = assign18850_e13577_d_n6;
        locals.var_tmf2_dn7 = assign18850_e13577_d_n7;
        locals.var_tmf2_dn8 = assign18850_e13577_d_n8;
        locals.var_tmf2_dn9 = assign18850_e13577_d_n9;
        locals.var_tmf2_dn10 = assign18850_e13577_d_n10;
        locals.var_tmf2_dn11 = assign18850_e13577_d_n11;
        locals.var_tmf2_dn14 = assign18850_e13577_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18860_e13591, assign18860_e13591_d_n0, assign18860_e13591_d_n2, assign18860_e13591_d_n4, assign18860_e13591_d_n5, assign18860_e13591_d_n6, assign18860_e13591_d_n7, assign18860_e13591_d_n8, assign18860_e13591_d_n9, assign18860_e13591_d_n10, assign18860_e13591_d_n11, assign18860_e13591_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18860_e13589, assign18860_e13589_d_n0, assign18860_e13589_d_n2, assign18860_e13589_d_n4, assign18860_e13589_d_n5, assign18860_e13589_d_n6, assign18860_e13589_d_n7, assign18860_e13589_d_n8, assign18860_e13589_d_n9, assign18860_e13589_d_n10, assign18860_e13589_d_n11, assign18860_e13589_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18860_e13588: f64 = (-locals.var_tmf2);
                (assign18860_e13588, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18860_e13589, assign18860_e13589_d_n0, assign18860_e13589_d_n2, assign18860_e13589_d_n4, assign18860_e13589_d_n5, assign18860_e13589_d_n6, assign18860_e13589_d_n7, assign18860_e13589_d_n8, assign18860_e13589_d_n9, assign18860_e13589_d_n10, assign18860_e13589_d_n11, assign18860_e13589_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18860_e13591;
        locals.var_tmf2_dn0 = assign18860_e13591_d_n0;
        locals.var_tmf2_dn2 = assign18860_e13591_d_n2;
        locals.var_tmf2_dn4 = assign18860_e13591_d_n4;
        locals.var_tmf2_dn5 = assign18860_e13591_d_n5;
        locals.var_tmf2_dn6 = assign18860_e13591_d_n6;
        locals.var_tmf2_dn7 = assign18860_e13591_d_n7;
        locals.var_tmf2_dn8 = assign18860_e13591_d_n8;
        locals.var_tmf2_dn9 = assign18860_e13591_d_n9;
        locals.var_tmf2_dn10 = assign18860_e13591_d_n10;
        locals.var_tmf2_dn11 = assign18860_e13591_d_n11;
        locals.var_tmf2_dn14 = assign18860_e13591_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18870_e13604, assign18870_e13604_d_n0, assign18870_e13604_d_n2, assign18870_e13604_d_n4, assign18870_e13604_d_n5, assign18870_e13604_d_n6, assign18870_e13604_d_n7, assign18870_e13604_d_n8, assign18870_e13604_d_n9, assign18870_e13604_d_n10, assign18870_e13604_d_n11, assign18870_e13604_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18870_e13599: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18870_e13601: f64 = (assign18870_e13599 + locals.var_tmf2);
        let assign18870_e13602: f64 = (assign18870_e13601).sqrt();
        (assign18870_e13602, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18870_e13602)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18870_e13602)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18870_e13604;
        locals.var_tmf2_dn0 = assign18870_e13604_d_n0;
        locals.var_tmf2_dn2 = assign18870_e13604_d_n2;
        locals.var_tmf2_dn4 = assign18870_e13604_d_n4;
        locals.var_tmf2_dn5 = assign18870_e13604_d_n5;
        locals.var_tmf2_dn6 = assign18870_e13604_d_n6;
        locals.var_tmf2_dn7 = assign18870_e13604_d_n7;
        locals.var_tmf2_dn8 = assign18870_e13604_d_n8;
        locals.var_tmf2_dn9 = assign18870_e13604_d_n9;
        locals.var_tmf2_dn10 = assign18870_e13604_d_n10;
        locals.var_tmf2_dn11 = assign18870_e13604_d_n11;
        locals.var_tmf2_dn14 = assign18870_e13604_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18880_e13618, assign18880_e13618_d_n0, assign18880_e13618_d_n2, assign18880_e13618_d_n4, assign18880_e13618_d_n5, assign18880_e13618_d_n6, assign18880_e13618_d_n7, assign18880_e13618_d_n8, assign18880_e13618_d_n9, assign18880_e13618_d_n10, assign18880_e13618_d_n11, assign18880_e13618_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18880_e13614: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18880_e13615: f64 = (1.0 + assign18880_e13614);
        let assign18880_e13616: f64 = (0.5 * assign18880_e13615);
        (assign18880_e13616, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18880_e13618;
        locals.var_t6_dn0 = assign18880_e13618_d_n0;
        locals.var_t6_dn2 = assign18880_e13618_d_n2;
        locals.var_t6_dn4 = assign18880_e13618_d_n4;
        locals.var_t6_dn5 = assign18880_e13618_d_n5;
        locals.var_t6_dn6 = assign18880_e13618_d_n6;
        locals.var_t6_dn7 = assign18880_e13618_d_n7;
        locals.var_t6_dn8 = assign18880_e13618_d_n8;
        locals.var_t6_dn9 = assign18880_e13618_d_n9;
        locals.var_t6_dn10 = assign18880_e13618_d_n10;
        locals.var_t6_dn11 = assign18880_e13618_d_n11;
        locals.var_t6_dn14 = assign18880_e13618_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18890_e13636, assign18890_e13636_d_n0, assign18890_e13636_d_n2, assign18890_e13636_d_n4, assign18890_e13636_d_n5, assign18890_e13636_d_n6, assign18890_e13636_d_n7, assign18890_e13636_d_n8, assign18890_e13636_d_n9, assign18890_e13636_d_n10, assign18890_e13636_d_n11, assign18890_e13636_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18890_e13627: f64 = (p.p98 + 1.0);
        let assign18890_e13628: f64 = (locals.var_t4 * assign18890_e13627);
        let assign18890_e13632: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18890_e13633: f64 = (0.5 * assign18890_e13632);
        let assign18890_e13634: f64 = (assign18890_e13628 - assign18890_e13633);
        (assign18890_e13634, ((locals.var_t4_dn0 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign18890_e13627) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign18890_e13636;
        locals.var_t7_dn0 = assign18890_e13636_d_n0;
        locals.var_t7_dn2 = assign18890_e13636_d_n2;
        locals.var_t7_dn4 = assign18890_e13636_d_n4;
        locals.var_t7_dn5 = assign18890_e13636_d_n5;
        locals.var_t7_dn6 = assign18890_e13636_d_n6;
        locals.var_t7_dn7 = assign18890_e13636_d_n7;
        locals.var_t7_dn8 = assign18890_e13636_d_n8;
        locals.var_t7_dn9 = assign18890_e13636_d_n9;
        locals.var_t7_dn10 = assign18890_e13636_d_n10;
        locals.var_t7_dn11 = assign18890_e13636_d_n11;
        locals.var_t7_dn14 = assign18890_e13636_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign18900_e13652, assign18900_e13652_d_n0, assign18900_e13652_d_n2, assign18900_e13652_d_n4, assign18900_e13652_d_n5, assign18900_e13652_d_n6, assign18900_e13652_d_n7, assign18900_e13652_d_n8, assign18900_e13652_d_n9, assign18900_e13652_d_n10, assign18900_e13652_d_n11, assign18900_e13652_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18900_e13645: f64 = (locals.var_t1 * locals.var_t4);
        let assign18900_e13646: f64 = (locals.var_t7 + assign18900_e13645);
        let assign18900_e13648: f64 = assign18900_e13646;
        let assign18900_e13650: f64 = (assign18900_e13648 - 5e-5);
        (assign18900_e13650, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18900_e13652;
        locals.var_tmf1_dn0 = assign18900_e13652_d_n0;
        locals.var_tmf1_dn2 = assign18900_e13652_d_n2;
        locals.var_tmf1_dn4 = assign18900_e13652_d_n4;
        locals.var_tmf1_dn5 = assign18900_e13652_d_n5;
        locals.var_tmf1_dn6 = assign18900_e13652_d_n6;
        locals.var_tmf1_dn7 = assign18900_e13652_d_n7;
        locals.var_tmf1_dn8 = assign18900_e13652_d_n8;
        locals.var_tmf1_dn9 = assign18900_e13652_d_n9;
        locals.var_tmf1_dn10 = assign18900_e13652_d_n10;
        locals.var_tmf1_dn11 = assign18900_e13652_d_n11;
        locals.var_tmf1_dn14 = assign18900_e13652_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18910_e13664, assign18910_e13664_d_n0, assign18910_e13664_d_n2, assign18910_e13664_d_n4, assign18910_e13664_d_n5, assign18910_e13664_d_n6, assign18910_e13664_d_n7, assign18910_e13664_d_n8, assign18910_e13664_d_n9, assign18910_e13664_d_n10, assign18910_e13664_d_n11, assign18910_e13664_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18910_e13664;
        locals.var_tmf2_dn0 = assign18910_e13664_d_n0;
        locals.var_tmf2_dn2 = assign18910_e13664_d_n2;
        locals.var_tmf2_dn4 = assign18910_e13664_d_n4;
        locals.var_tmf2_dn5 = assign18910_e13664_d_n5;
        locals.var_tmf2_dn6 = assign18910_e13664_d_n6;
        locals.var_tmf2_dn7 = assign18910_e13664_d_n7;
        locals.var_tmf2_dn8 = assign18910_e13664_d_n8;
        locals.var_tmf2_dn9 = assign18910_e13664_d_n9;
        locals.var_tmf2_dn10 = assign18910_e13664_d_n10;
        locals.var_tmf2_dn11 = assign18910_e13664_d_n11;
        locals.var_tmf2_dn14 = assign18910_e13664_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18920_e13678, assign18920_e13678_d_n0, assign18920_e13678_d_n2, assign18920_e13678_d_n4, assign18920_e13678_d_n5, assign18920_e13678_d_n6, assign18920_e13678_d_n7, assign18920_e13678_d_n8, assign18920_e13678_d_n9, assign18920_e13678_d_n10, assign18920_e13678_d_n11, assign18920_e13678_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18920_e13676, assign18920_e13676_d_n0, assign18920_e13676_d_n2, assign18920_e13676_d_n4, assign18920_e13676_d_n5, assign18920_e13676_d_n6, assign18920_e13676_d_n7, assign18920_e13676_d_n8, assign18920_e13676_d_n9, assign18920_e13676_d_n10, assign18920_e13676_d_n11, assign18920_e13676_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18920_e13675: f64 = (-locals.var_tmf2);
                (assign18920_e13675, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18920_e13676, assign18920_e13676_d_n0, assign18920_e13676_d_n2, assign18920_e13676_d_n4, assign18920_e13676_d_n5, assign18920_e13676_d_n6, assign18920_e13676_d_n7, assign18920_e13676_d_n8, assign18920_e13676_d_n9, assign18920_e13676_d_n10, assign18920_e13676_d_n11, assign18920_e13676_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18920_e13678;
        locals.var_tmf2_dn0 = assign18920_e13678_d_n0;
        locals.var_tmf2_dn2 = assign18920_e13678_d_n2;
        locals.var_tmf2_dn4 = assign18920_e13678_d_n4;
        locals.var_tmf2_dn5 = assign18920_e13678_d_n5;
        locals.var_tmf2_dn6 = assign18920_e13678_d_n6;
        locals.var_tmf2_dn7 = assign18920_e13678_d_n7;
        locals.var_tmf2_dn8 = assign18920_e13678_d_n8;
        locals.var_tmf2_dn9 = assign18920_e13678_d_n9;
        locals.var_tmf2_dn10 = assign18920_e13678_d_n10;
        locals.var_tmf2_dn11 = assign18920_e13678_d_n11;
        locals.var_tmf2_dn14 = assign18920_e13678_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18930_e13691, assign18930_e13691_d_n0, assign18930_e13691_d_n2, assign18930_e13691_d_n4, assign18930_e13691_d_n5, assign18930_e13691_d_n6, assign18930_e13691_d_n7, assign18930_e13691_d_n8, assign18930_e13691_d_n9, assign18930_e13691_d_n10, assign18930_e13691_d_n11, assign18930_e13691_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18930_e13686: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18930_e13688: f64 = (assign18930_e13686 + locals.var_tmf2);
        let assign18930_e13689: f64 = (assign18930_e13688).sqrt();
        (assign18930_e13689, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18930_e13689)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18930_e13689)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18930_e13691;
        locals.var_tmf2_dn0 = assign18930_e13691_d_n0;
        locals.var_tmf2_dn2 = assign18930_e13691_d_n2;
        locals.var_tmf2_dn4 = assign18930_e13691_d_n4;
        locals.var_tmf2_dn5 = assign18930_e13691_d_n5;
        locals.var_tmf2_dn6 = assign18930_e13691_d_n6;
        locals.var_tmf2_dn7 = assign18930_e13691_d_n7;
        locals.var_tmf2_dn8 = assign18930_e13691_d_n8;
        locals.var_tmf2_dn9 = assign18930_e13691_d_n9;
        locals.var_tmf2_dn10 = assign18930_e13691_d_n10;
        locals.var_tmf2_dn11 = assign18930_e13691_d_n11;
        locals.var_tmf2_dn14 = assign18930_e13691_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18940_e13705, assign18940_e13705_d_n0, assign18940_e13705_d_n2, assign18940_e13705_d_n4, assign18940_e13705_d_n5, assign18940_e13705_d_n6, assign18940_e13705_d_n7, assign18940_e13705_d_n8, assign18940_e13705_d_n9, assign18940_e13705_d_n10, assign18940_e13705_d_n11, assign18940_e13705_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18940_e13701: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18940_e13702: f64 = (1.0 + assign18940_e13701);
        let assign18940_e13703: f64 = (0.5 * assign18940_e13702);
        (assign18940_e13703, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18940_e13705;
        locals.var_t6_dn0 = assign18940_e13705_d_n0;
        locals.var_t6_dn2 = assign18940_e13705_d_n2;
        locals.var_t6_dn4 = assign18940_e13705_d_n4;
        locals.var_t6_dn5 = assign18940_e13705_d_n5;
        locals.var_t6_dn6 = assign18940_e13705_d_n6;
        locals.var_t6_dn7 = assign18940_e13705_d_n7;
        locals.var_t6_dn8 = assign18940_e13705_d_n8;
        locals.var_t6_dn9 = assign18940_e13705_d_n9;
        locals.var_t6_dn10 = assign18940_e13705_d_n10;
        locals.var_t6_dn11 = assign18940_e13705_d_n11;
        locals.var_t6_dn14 = assign18940_e13705_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18950_e13719, assign18950_e13719_d_n0, assign18950_e13719_d_n2, assign18950_e13719_d_n4, assign18950_e13719_d_n5, assign18950_e13719_d_n6, assign18950_e13719_d_n7, assign18950_e13719_d_n8, assign18950_e13719_d_n9, assign18950_e13719_d_n10, assign18950_e13719_d_n11, assign18950_e13719_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18950_e13715: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18950_e13716: f64 = (0.5 * assign18950_e13715);
        let assign18950_e13717: f64 = assign18950_e13716;
        (assign18950_e13717, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18950_e13719;
        locals.var_t2_dn0 = assign18950_e13719_d_n0;
        locals.var_t2_dn2 = assign18950_e13719_d_n2;
        locals.var_t2_dn4 = assign18950_e13719_d_n4;
        locals.var_t2_dn5 = assign18950_e13719_d_n5;
        locals.var_t2_dn6 = assign18950_e13719_d_n6;
        locals.var_t2_dn7 = assign18950_e13719_d_n7;
        locals.var_t2_dn8 = assign18950_e13719_d_n8;
        locals.var_t2_dn9 = assign18950_e13719_d_n9;
        locals.var_t2_dn10 = assign18950_e13719_d_n10;
        locals.var_t2_dn11 = assign18950_e13719_d_n11;
        locals.var_t2_dn14 = assign18950_e13719_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18960_e13726: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign18960_e13726;
        locals.var_guard389_rv = 0.0;

        let (assign18970_e13746, assign18970_e13746_d_n0, assign18970_e13746_d_n2, assign18970_e13746_d_n4, assign18970_e13746_d_n5, assign18970_e13746_d_n6, assign18970_e13746_d_n7, assign18970_e13746_d_n8, assign18970_e13746_d_n9, assign18970_e13746_d_n10, assign18970_e13746_d_n11, assign18970_e13746_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18970_e13737: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign18970_e13738: f64 = (locals.var_uc_rdvd + assign18970_e13737);
        let assign18970_e13741: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign18970_e13742: f64 = (assign18970_e13738 + assign18970_e13741);
        let assign18970_e13744: f64 = (assign18970_e13742 * locals.var_t2);
        (assign18970_e13744, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18970_e13742 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign18970_e13746;
        locals.var_rdvde_dn0 = assign18970_e13746_d_n0;
        locals.var_rdvde_dn2 = assign18970_e13746_d_n2;
        locals.var_rdvde_dn4 = assign18970_e13746_d_n4;
        locals.var_rdvde_dn5 = assign18970_e13746_d_n5;
        locals.var_rdvde_dn6 = assign18970_e13746_d_n6;
        locals.var_rdvde_dn7 = assign18970_e13746_d_n7;
        locals.var_rdvde_dn8 = assign18970_e13746_d_n8;
        locals.var_rdvde_dn9 = assign18970_e13746_d_n9;
        locals.var_rdvde_dn10 = assign18970_e13746_d_n10;
        locals.var_rdvde_dn11 = assign18970_e13746_d_n11;
        locals.var_rdvde_dn14 = assign18970_e13746_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign18980_e13764, assign18980_e13764_d_n0, assign18980_e13764_d_n2, assign18980_e13764_d_n4, assign18980_e13764_d_n5, assign18980_e13764_d_n6, assign18980_e13764_d_n7, assign18980_e13764_d_n8, assign18980_e13764_d_n9, assign18980_e13764_d_n10, assign18980_e13764_d_n11, assign18980_e13764_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18980_e13757: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18980_e13758: f64 = (locals.var_rdvde - assign18980_e13757);
        let assign18980_e13761: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18980_e13762: f64 = (assign18980_e13758 - assign18980_e13761);
        (assign18980_e13762, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18980_e13764;
        locals.var_tmf1_dn0 = assign18980_e13764_d_n0;
        locals.var_tmf1_dn2 = assign18980_e13764_d_n2;
        locals.var_tmf1_dn4 = assign18980_e13764_d_n4;
        locals.var_tmf1_dn5 = assign18980_e13764_d_n5;
        locals.var_tmf1_dn6 = assign18980_e13764_d_n6;
        locals.var_tmf1_dn7 = assign18980_e13764_d_n7;
        locals.var_tmf1_dn8 = assign18980_e13764_d_n8;
        locals.var_tmf1_dn9 = assign18980_e13764_d_n9;
        locals.var_tmf1_dn10 = assign18980_e13764_d_n10;
        locals.var_tmf1_dn11 = assign18980_e13764_d_n11;
        locals.var_tmf1_dn14 = assign18980_e13764_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18990_e13782, assign18990_e13782_d_n0, assign18990_e13782_d_n2, assign18990_e13782_d_n4, assign18990_e13782_d_n5, assign18990_e13782_d_n6, assign18990_e13782_d_n7, assign18990_e13782_d_n8, assign18990_e13782_d_n9, assign18990_e13782_d_n10, assign18990_e13782_d_n11, assign18990_e13782_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign18990_e13775: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18990_e13776: f64 = (4.0 * assign18990_e13775);
        let assign18990_e13779: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18990_e13780: f64 = (assign18990_e13776 * assign18990_e13779);
        (assign18990_e13780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18990_e13782;
        locals.var_tmf2_dn0 = assign18990_e13782_d_n0;
        locals.var_tmf2_dn2 = assign18990_e13782_d_n2;
        locals.var_tmf2_dn4 = assign18990_e13782_d_n4;
        locals.var_tmf2_dn5 = assign18990_e13782_d_n5;
        locals.var_tmf2_dn6 = assign18990_e13782_d_n6;
        locals.var_tmf2_dn7 = assign18990_e13782_d_n7;
        locals.var_tmf2_dn8 = assign18990_e13782_d_n8;
        locals.var_tmf2_dn9 = assign18990_e13782_d_n9;
        locals.var_tmf2_dn10 = assign18990_e13782_d_n10;
        locals.var_tmf2_dn11 = assign18990_e13782_d_n11;
        locals.var_tmf2_dn14 = assign18990_e13782_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19000_e13798, assign19000_e13798_d_n0, assign19000_e13798_d_n2, assign19000_e13798_d_n4, assign19000_e13798_d_n5, assign19000_e13798_d_n6, assign19000_e13798_d_n7, assign19000_e13798_d_n8, assign19000_e13798_d_n9, assign19000_e13798_d_n10, assign19000_e13798_d_n11, assign19000_e13798_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let (assign19000_e13796, assign19000_e13796_d_n0, assign19000_e13796_d_n2, assign19000_e13796_d_n4, assign19000_e13796_d_n5, assign19000_e13796_d_n6, assign19000_e13796_d_n7, assign19000_e13796_d_n8, assign19000_e13796_d_n9, assign19000_e13796_d_n10, assign19000_e13796_d_n11, assign19000_e13796_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19000_e13795: f64 = (-locals.var_tmf2);
                (assign19000_e13795, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19000_e13796, assign19000_e13796_d_n0, assign19000_e13796_d_n2, assign19000_e13796_d_n4, assign19000_e13796_d_n5, assign19000_e13796_d_n6, assign19000_e13796_d_n7, assign19000_e13796_d_n8, assign19000_e13796_d_n9, assign19000_e13796_d_n10, assign19000_e13796_d_n11, assign19000_e13796_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19000_e13798;
        locals.var_tmf2_dn0 = assign19000_e13798_d_n0;
        locals.var_tmf2_dn2 = assign19000_e13798_d_n2;
        locals.var_tmf2_dn4 = assign19000_e13798_d_n4;
        locals.var_tmf2_dn5 = assign19000_e13798_d_n5;
        locals.var_tmf2_dn6 = assign19000_e13798_d_n6;
        locals.var_tmf2_dn7 = assign19000_e13798_d_n7;
        locals.var_tmf2_dn8 = assign19000_e13798_d_n8;
        locals.var_tmf2_dn9 = assign19000_e13798_d_n9;
        locals.var_tmf2_dn10 = assign19000_e13798_d_n10;
        locals.var_tmf2_dn11 = assign19000_e13798_d_n11;
        locals.var_tmf2_dn14 = assign19000_e13798_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19010_e13813, assign19010_e13813_d_n0, assign19010_e13813_d_n2, assign19010_e13813_d_n4, assign19010_e13813_d_n5, assign19010_e13813_d_n6, assign19010_e13813_d_n7, assign19010_e13813_d_n8, assign19010_e13813_d_n9, assign19010_e13813_d_n10, assign19010_e13813_d_n11, assign19010_e13813_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19010_e13808: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19010_e13810: f64 = (assign19010_e13808 + locals.var_tmf2);
        let assign19010_e13811: f64 = (assign19010_e13810).sqrt();
        (assign19010_e13811, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19010_e13811)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19010_e13811)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19010_e13813;
        locals.var_tmf2_dn0 = assign19010_e13813_d_n0;
        locals.var_tmf2_dn2 = assign19010_e13813_d_n2;
        locals.var_tmf2_dn4 = assign19010_e13813_d_n4;
        locals.var_tmf2_dn5 = assign19010_e13813_d_n5;
        locals.var_tmf2_dn6 = assign19010_e13813_d_n6;
        locals.var_tmf2_dn7 = assign19010_e13813_d_n7;
        locals.var_tmf2_dn8 = assign19010_e13813_d_n8;
        locals.var_tmf2_dn9 = assign19010_e13813_d_n9;
        locals.var_tmf2_dn10 = assign19010_e13813_d_n10;
        locals.var_tmf2_dn11 = assign19010_e13813_d_n11;
        locals.var_tmf2_dn14 = assign19010_e13813_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19020_e13829, assign19020_e13829_d_n0, assign19020_e13829_d_n2, assign19020_e13829_d_n4, assign19020_e13829_d_n5, assign19020_e13829_d_n6, assign19020_e13829_d_n7, assign19020_e13829_d_n8, assign19020_e13829_d_n9, assign19020_e13829_d_n10, assign19020_e13829_d_n11, assign19020_e13829_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19020_e13825: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19020_e13826: f64 = (1.0 + assign19020_e13825);
        let assign19020_e13827: f64 = (0.5 * assign19020_e13826);
        (assign19020_e13827, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19020_e13829;
        locals.var_t0_dn0 = assign19020_e13829_d_n0;
        locals.var_t0_dn2 = assign19020_e13829_d_n2;
        locals.var_t0_dn4 = assign19020_e13829_d_n4;
        locals.var_t0_dn5 = assign19020_e13829_d_n5;
        locals.var_t0_dn6 = assign19020_e13829_d_n6;
        locals.var_t0_dn7 = assign19020_e13829_d_n7;
        locals.var_t0_dn8 = assign19020_e13829_d_n8;
        locals.var_t0_dn9 = assign19020_e13829_d_n9;
        locals.var_t0_dn10 = assign19020_e13829_d_n10;
        locals.var_t0_dn11 = assign19020_e13829_d_n11;
        locals.var_t0_dn14 = assign19020_e13829_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19030_e13847, assign19030_e13847_d_n0, assign19030_e13847_d_n2, assign19030_e13847_d_n4, assign19030_e13847_d_n5, assign19030_e13847_d_n6, assign19030_e13847_d_n7, assign19030_e13847_d_n8, assign19030_e13847_d_n9, assign19030_e13847_d_n10, assign19030_e13847_d_n11, assign19030_e13847_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign19030_e13839: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19030_e13843: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19030_e13844: f64 = (0.5 * assign19030_e13843);
        let assign19030_e13845: f64 = (assign19030_e13839 + assign19030_e13844);
        (assign19030_e13845, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19030_e13847;
        locals.var_rdvde_dn0 = assign19030_e13847_d_n0;
        locals.var_rdvde_dn2 = assign19030_e13847_d_n2;
        locals.var_rdvde_dn4 = assign19030_e13847_d_n4;
        locals.var_rdvde_dn5 = assign19030_e13847_d_n5;
        locals.var_rdvde_dn6 = assign19030_e13847_d_n6;
        locals.var_rdvde_dn7 = assign19030_e13847_d_n7;
        locals.var_rdvde_dn8 = assign19030_e13847_d_n8;
        locals.var_rdvde_dn9 = assign19030_e13847_d_n9;
        locals.var_rdvde_dn10 = assign19030_e13847_d_n10;
        locals.var_rdvde_dn11 = assign19030_e13847_d_n11;
        locals.var_rdvde_dn14 = assign19030_e13847_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19040_e13868, assign19040_e13868_d_n0, assign19040_e13868_d_n2, assign19040_e13868_d_n4, assign19040_e13868_d_n5, assign19040_e13868_d_n6, assign19040_e13868_d_n7, assign19040_e13868_d_n8, assign19040_e13868_d_n9, assign19040_e13868_d_n10, assign19040_e13868_d_n11, assign19040_e13868_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19040_e13859: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19040_e13860: f64 = (locals.var_uc_rdvd + assign19040_e13859);
        let assign19040_e13863: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19040_e13864: f64 = (assign19040_e13860 + assign19040_e13863);
        let assign19040_e13866: f64 = (assign19040_e13864 * locals.var_t2);
        (assign19040_e13866, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19040_e13864 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19040_e13868;
        locals.var_rdvde_dn0 = assign19040_e13868_d_n0;
        locals.var_rdvde_dn2 = assign19040_e13868_d_n2;
        locals.var_rdvde_dn4 = assign19040_e13868_d_n4;
        locals.var_rdvde_dn5 = assign19040_e13868_d_n5;
        locals.var_rdvde_dn6 = assign19040_e13868_d_n6;
        locals.var_rdvde_dn7 = assign19040_e13868_d_n7;
        locals.var_rdvde_dn8 = assign19040_e13868_d_n8;
        locals.var_rdvde_dn9 = assign19040_e13868_d_n9;
        locals.var_rdvde_dn10 = assign19040_e13868_d_n10;
        locals.var_rdvde_dn11 = assign19040_e13868_d_n11;
        locals.var_rdvde_dn14 = assign19040_e13868_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19050_e13887, assign19050_e13887_d_n0, assign19050_e13887_d_n2, assign19050_e13887_d_n4, assign19050_e13887_d_n5, assign19050_e13887_d_n6, assign19050_e13887_d_n7, assign19050_e13887_d_n8, assign19050_e13887_d_n9, assign19050_e13887_d_n10, assign19050_e13887_d_n11, assign19050_e13887_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19050_e13880: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19050_e13881: f64 = (locals.var_rdvde - assign19050_e13880);
        let assign19050_e13884: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19050_e13885: f64 = (assign19050_e13881 - assign19050_e13884);
        (assign19050_e13885, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19050_e13887;
        locals.var_tmf1_dn0 = assign19050_e13887_d_n0;
        locals.var_tmf1_dn2 = assign19050_e13887_d_n2;
        locals.var_tmf1_dn4 = assign19050_e13887_d_n4;
        locals.var_tmf1_dn5 = assign19050_e13887_d_n5;
        locals.var_tmf1_dn6 = assign19050_e13887_d_n6;
        locals.var_tmf1_dn7 = assign19050_e13887_d_n7;
        locals.var_tmf1_dn8 = assign19050_e13887_d_n8;
        locals.var_tmf1_dn9 = assign19050_e13887_d_n9;
        locals.var_tmf1_dn10 = assign19050_e13887_d_n10;
        locals.var_tmf1_dn11 = assign19050_e13887_d_n11;
        locals.var_tmf1_dn14 = assign19050_e13887_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19060_e13906, assign19060_e13906_d_n0, assign19060_e13906_d_n2, assign19060_e13906_d_n4, assign19060_e13906_d_n5, assign19060_e13906_d_n6, assign19060_e13906_d_n7, assign19060_e13906_d_n8, assign19060_e13906_d_n9, assign19060_e13906_d_n10, assign19060_e13906_d_n11, assign19060_e13906_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19060_e13899: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19060_e13900: f64 = (4.0 * assign19060_e13899);
        let assign19060_e13903: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19060_e13904: f64 = (assign19060_e13900 * assign19060_e13903);
        (assign19060_e13904, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19060_e13906;
        locals.var_tmf2_dn0 = assign19060_e13906_d_n0;
        locals.var_tmf2_dn2 = assign19060_e13906_d_n2;
        locals.var_tmf2_dn4 = assign19060_e13906_d_n4;
        locals.var_tmf2_dn5 = assign19060_e13906_d_n5;
        locals.var_tmf2_dn6 = assign19060_e13906_d_n6;
        locals.var_tmf2_dn7 = assign19060_e13906_d_n7;
        locals.var_tmf2_dn8 = assign19060_e13906_d_n8;
        locals.var_tmf2_dn9 = assign19060_e13906_d_n9;
        locals.var_tmf2_dn10 = assign19060_e13906_d_n10;
        locals.var_tmf2_dn11 = assign19060_e13906_d_n11;
        locals.var_tmf2_dn14 = assign19060_e13906_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19070_e13923, assign19070_e13923_d_n0, assign19070_e13923_d_n2, assign19070_e13923_d_n4, assign19070_e13923_d_n5, assign19070_e13923_d_n6, assign19070_e13923_d_n7, assign19070_e13923_d_n8, assign19070_e13923_d_n9, assign19070_e13923_d_n10, assign19070_e13923_d_n11, assign19070_e13923_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let (assign19070_e13921, assign19070_e13921_d_n0, assign19070_e13921_d_n2, assign19070_e13921_d_n4, assign19070_e13921_d_n5, assign19070_e13921_d_n6, assign19070_e13921_d_n7, assign19070_e13921_d_n8, assign19070_e13921_d_n9, assign19070_e13921_d_n10, assign19070_e13921_d_n11, assign19070_e13921_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19070_e13920: f64 = (-locals.var_tmf2);
                (assign19070_e13920, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19070_e13921, assign19070_e13921_d_n0, assign19070_e13921_d_n2, assign19070_e13921_d_n4, assign19070_e13921_d_n5, assign19070_e13921_d_n6, assign19070_e13921_d_n7, assign19070_e13921_d_n8, assign19070_e13921_d_n9, assign19070_e13921_d_n10, assign19070_e13921_d_n11, assign19070_e13921_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19070_e13923;
        locals.var_tmf2_dn0 = assign19070_e13923_d_n0;
        locals.var_tmf2_dn2 = assign19070_e13923_d_n2;
        locals.var_tmf2_dn4 = assign19070_e13923_d_n4;
        locals.var_tmf2_dn5 = assign19070_e13923_d_n5;
        locals.var_tmf2_dn6 = assign19070_e13923_d_n6;
        locals.var_tmf2_dn7 = assign19070_e13923_d_n7;
        locals.var_tmf2_dn8 = assign19070_e13923_d_n8;
        locals.var_tmf2_dn9 = assign19070_e13923_d_n9;
        locals.var_tmf2_dn10 = assign19070_e13923_d_n10;
        locals.var_tmf2_dn11 = assign19070_e13923_d_n11;
        locals.var_tmf2_dn14 = assign19070_e13923_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19080_e13939, assign19080_e13939_d_n0, assign19080_e13939_d_n2, assign19080_e13939_d_n4, assign19080_e13939_d_n5, assign19080_e13939_d_n6, assign19080_e13939_d_n7, assign19080_e13939_d_n8, assign19080_e13939_d_n9, assign19080_e13939_d_n10, assign19080_e13939_d_n11, assign19080_e13939_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19080_e13934: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19080_e13936: f64 = (assign19080_e13934 + locals.var_tmf2);
        let assign19080_e13937: f64 = (assign19080_e13936).sqrt();
        (assign19080_e13937, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19080_e13937)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19080_e13937)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19080_e13939;
        locals.var_tmf2_dn0 = assign19080_e13939_d_n0;
        locals.var_tmf2_dn2 = assign19080_e13939_d_n2;
        locals.var_tmf2_dn4 = assign19080_e13939_d_n4;
        locals.var_tmf2_dn5 = assign19080_e13939_d_n5;
        locals.var_tmf2_dn6 = assign19080_e13939_d_n6;
        locals.var_tmf2_dn7 = assign19080_e13939_d_n7;
        locals.var_tmf2_dn8 = assign19080_e13939_d_n8;
        locals.var_tmf2_dn9 = assign19080_e13939_d_n9;
        locals.var_tmf2_dn10 = assign19080_e13939_d_n10;
        locals.var_tmf2_dn11 = assign19080_e13939_d_n11;
        locals.var_tmf2_dn14 = assign19080_e13939_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19090_e13956, assign19090_e13956_d_n0, assign19090_e13956_d_n2, assign19090_e13956_d_n4, assign19090_e13956_d_n5, assign19090_e13956_d_n6, assign19090_e13956_d_n7, assign19090_e13956_d_n8, assign19090_e13956_d_n9, assign19090_e13956_d_n10, assign19090_e13956_d_n11, assign19090_e13956_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19090_e13952: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19090_e13953: f64 = (1.0 + assign19090_e13952);
        let assign19090_e13954: f64 = (0.5 * assign19090_e13953);
        (assign19090_e13954, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19090_e13956;
        locals.var_t0_dn0 = assign19090_e13956_d_n0;
        locals.var_t0_dn2 = assign19090_e13956_d_n2;
        locals.var_t0_dn4 = assign19090_e13956_d_n4;
        locals.var_t0_dn5 = assign19090_e13956_d_n5;
        locals.var_t0_dn6 = assign19090_e13956_d_n6;
        locals.var_t0_dn7 = assign19090_e13956_d_n7;
        locals.var_t0_dn8 = assign19090_e13956_d_n8;
        locals.var_t0_dn9 = assign19090_e13956_d_n9;
        locals.var_t0_dn10 = assign19090_e13956_d_n10;
        locals.var_t0_dn11 = assign19090_e13956_d_n11;
        locals.var_t0_dn14 = assign19090_e13956_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19100_e13975, assign19100_e13975_d_n0, assign19100_e13975_d_n2, assign19100_e13975_d_n4, assign19100_e13975_d_n5, assign19100_e13975_d_n6, assign19100_e13975_d_n7, assign19100_e13975_d_n8, assign19100_e13975_d_n9, assign19100_e13975_d_n10, assign19100_e13975_d_n11, assign19100_e13975_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign19100_e13967: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19100_e13971: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19100_e13972: f64 = (0.5 * assign19100_e13971);
        let assign19100_e13973: f64 = (assign19100_e13967 + assign19100_e13972);
        (assign19100_e13973, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19100_e13975;
        locals.var_rdvde_dn0 = assign19100_e13975_d_n0;
        locals.var_rdvde_dn2 = assign19100_e13975_d_n2;
        locals.var_rdvde_dn4 = assign19100_e13975_d_n4;
        locals.var_rdvde_dn5 = assign19100_e13975_d_n5;
        locals.var_rdvde_dn6 = assign19100_e13975_d_n6;
        locals.var_rdvde_dn7 = assign19100_e13975_d_n7;
        locals.var_rdvde_dn8 = assign19100_e13975_d_n8;
        locals.var_rdvde_dn9 = assign19100_e13975_d_n9;
        locals.var_rdvde_dn10 = assign19100_e13975_d_n10;
        locals.var_rdvde_dn11 = assign19100_e13975_d_n11;
        locals.var_rdvde_dn14 = assign19100_e13975_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19110_e13999, assign19110_e13999_d_n0, assign19110_e13999_d_n2, assign19110_e13999_d_n4, assign19110_e13999_d_n5, assign19110_e13999_d_n6, assign19110_e13999_d_n7, assign19110_e13999_d_n8, assign19110_e13999_d_n9, assign19110_e13999_d_n10, assign19110_e13999_d_n11, assign19110_e13999_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19110_e13984: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign19110_e13986: f64 = (assign19110_e13984 * 1000000.0);
        let assign19110_e13988: f64 = (assign19110_e13986 + locals.var_uc_rdict1);
        let assign19110_e13989: f64 = (locals.var_rdvdtemp0 * assign19110_e13988);
        let assign19110_e13992: f64 = (p.p70 * p.p100);
        let assign19110_e13994: f64 = (assign19110_e13992 * 1000000.0);
        let assign19110_e13996: f64 = (assign19110_e13994 + p.p101);
        let assign19110_e13997: f64 = (assign19110_e13989 * assign19110_e13996);
        (assign19110_e13997, ((locals.var_rdvdtemp0_dn0 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn2 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn4 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn5 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn6 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn7 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn8 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn9 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn10 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn11 * assign19110_e13988) * assign19110_e13996), ((locals.var_rdvdtemp0_dn14 * assign19110_e13988) * assign19110_e13996),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign19110_e13999;
        locals.var_t4_dn0 = assign19110_e13999_d_n0;
        locals.var_t4_dn2 = assign19110_e13999_d_n2;
        locals.var_t4_dn4 = assign19110_e13999_d_n4;
        locals.var_t4_dn5 = assign19110_e13999_d_n5;
        locals.var_t4_dn6 = assign19110_e13999_d_n6;
        locals.var_t4_dn7 = assign19110_e13999_d_n7;
        locals.var_t4_dn8 = assign19110_e13999_d_n8;
        locals.var_t4_dn9 = assign19110_e13999_d_n9;
        locals.var_t4_dn10 = assign19110_e13999_d_n10;
        locals.var_t4_dn11 = assign19110_e13999_d_n11;
        locals.var_t4_dn14 = assign19110_e13999_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign19120_e14013, assign19120_e14013_d_n0, assign19120_e14013_d_n2, assign19120_e14013_d_n4, assign19120_e14013_d_n5, assign19120_e14013_d_n6, assign19120_e14013_d_n7, assign19120_e14013_d_n8, assign19120_e14013_d_n9, assign19120_e14013_d_n10, assign19120_e14013_d_n11, assign19120_e14013_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19120_e14007: f64 = (1.0 - locals.var_uc_rdov13);
        let assign19120_e14009: f64 = (assign19120_e14007 * p.p66);
        let assign19120_e14011: f64 = (assign19120_e14009 * 1000000.0);
        (assign19120_e14011, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign19120_e14013;
        locals.var_t1_dn0 = assign19120_e14013_d_n0;
        locals.var_t1_dn2 = assign19120_e14013_d_n2;
        locals.var_t1_dn4 = assign19120_e14013_d_n4;
        locals.var_t1_dn5 = assign19120_e14013_d_n5;
        locals.var_t1_dn6 = assign19120_e14013_d_n6;
        locals.var_t1_dn7 = assign19120_e14013_d_n7;
        locals.var_t1_dn8 = assign19120_e14013_d_n8;
        locals.var_t1_dn9 = assign19120_e14013_d_n9;
        locals.var_t1_dn10 = assign19120_e14013_d_n10;
        locals.var_t1_dn11 = assign19120_e14013_d_n11;
        locals.var_t1_dn14 = assign19120_e14013_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign19130_e14029, assign19130_e14029_d_n0, assign19130_e14029_d_n2, assign19130_e14029_d_n4, assign19130_e14029_d_n5, assign19130_e14029_d_n6, assign19130_e14029_d_n7, assign19130_e14029_d_n8, assign19130_e14029_d_n9, assign19130_e14029_d_n10, assign19130_e14029_d_n11, assign19130_e14029_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19130_e14021: f64 = (locals.var_t8 * p.p66);
        let assign19130_e14023: f64 = (assign19130_e14021 * 1000000.0);
        let assign19130_e14025: f64 = (assign19130_e14023 + 1.0);
        let assign19130_e14027: f64 = (assign19130_e14025 + p.p98);
        (assign19130_e14027, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn11 * p.p66) * 1000000.0), ((locals.var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign19130_e14029;
        locals.var_t3_dn0 = assign19130_e14029_d_n0;
        locals.var_t3_dn2 = assign19130_e14029_d_n2;
        locals.var_t3_dn4 = assign19130_e14029_d_n4;
        locals.var_t3_dn5 = assign19130_e14029_d_n5;
        locals.var_t3_dn6 = assign19130_e14029_d_n6;
        locals.var_t3_dn7 = assign19130_e14029_d_n7;
        locals.var_t3_dn8 = assign19130_e14029_d_n8;
        locals.var_t3_dn9 = assign19130_e14029_d_n9;
        locals.var_t3_dn10 = assign19130_e14029_d_n10;
        locals.var_t3_dn11 = assign19130_e14029_d_n11;
        locals.var_t3_dn14 = assign19130_e14029_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign19140_e14043, assign19140_e14043_d_n0, assign19140_e14043_d_n2, assign19140_e14043_d_n4, assign19140_e14043_d_n5, assign19140_e14043_d_n6, assign19140_e14043_d_n7, assign19140_e14043_d_n8, assign19140_e14043_d_n9, assign19140_e14043_d_n10, assign19140_e14043_d_n11, assign19140_e14043_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19140_e14037: f64 = (locals.var_t3 * locals.var_t4);
        let assign19140_e14039: f64 = (assign19140_e14037 - locals.var_t4);
        let assign19140_e14041: f64 = (assign19140_e14039 - 0.01);
        (assign19140_e14041, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19140_e14043;
        locals.var_tmf1_dn0 = assign19140_e14043_d_n0;
        locals.var_tmf1_dn2 = assign19140_e14043_d_n2;
        locals.var_tmf1_dn4 = assign19140_e14043_d_n4;
        locals.var_tmf1_dn5 = assign19140_e14043_d_n5;
        locals.var_tmf1_dn6 = assign19140_e14043_d_n6;
        locals.var_tmf1_dn7 = assign19140_e14043_d_n7;
        locals.var_tmf1_dn8 = assign19140_e14043_d_n8;
        locals.var_tmf1_dn9 = assign19140_e14043_d_n9;
        locals.var_tmf1_dn10 = assign19140_e14043_d_n10;
        locals.var_tmf1_dn11 = assign19140_e14043_d_n11;
        locals.var_tmf1_dn14 = assign19140_e14043_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19150_e14055, assign19150_e14055_d_n0, assign19150_e14055_d_n2, assign19150_e14055_d_n4, assign19150_e14055_d_n5, assign19150_e14055_d_n6, assign19150_e14055_d_n7, assign19150_e14055_d_n8, assign19150_e14055_d_n9, assign19150_e14055_d_n10, assign19150_e14055_d_n11, assign19150_e14055_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19150_e14051: f64 = (4.0 * locals.var_t4);
        let assign19150_e14053: f64 = (assign19150_e14051 * 0.01);
        (assign19150_e14053, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19150_e14055;
        locals.var_tmf2_dn0 = assign19150_e14055_d_n0;
        locals.var_tmf2_dn2 = assign19150_e14055_d_n2;
        locals.var_tmf2_dn4 = assign19150_e14055_d_n4;
        locals.var_tmf2_dn5 = assign19150_e14055_d_n5;
        locals.var_tmf2_dn6 = assign19150_e14055_d_n6;
        locals.var_tmf2_dn7 = assign19150_e14055_d_n7;
        locals.var_tmf2_dn8 = assign19150_e14055_d_n8;
        locals.var_tmf2_dn9 = assign19150_e14055_d_n9;
        locals.var_tmf2_dn10 = assign19150_e14055_d_n10;
        locals.var_tmf2_dn11 = assign19150_e14055_d_n11;
        locals.var_tmf2_dn14 = assign19150_e14055_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19160_e14069, assign19160_e14069_d_n0, assign19160_e14069_d_n2, assign19160_e14069_d_n4, assign19160_e14069_d_n5, assign19160_e14069_d_n6, assign19160_e14069_d_n7, assign19160_e14069_d_n8, assign19160_e14069_d_n9, assign19160_e14069_d_n10, assign19160_e14069_d_n11, assign19160_e14069_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19160_e14067, assign19160_e14067_d_n0, assign19160_e14067_d_n2, assign19160_e14067_d_n4, assign19160_e14067_d_n5, assign19160_e14067_d_n6, assign19160_e14067_d_n7, assign19160_e14067_d_n8, assign19160_e14067_d_n9, assign19160_e14067_d_n10, assign19160_e14067_d_n11, assign19160_e14067_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19160_e14066: f64 = (-locals.var_tmf2);
                (assign19160_e14066, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19160_e14067, assign19160_e14067_d_n0, assign19160_e14067_d_n2, assign19160_e14067_d_n4, assign19160_e14067_d_n5, assign19160_e14067_d_n6, assign19160_e14067_d_n7, assign19160_e14067_d_n8, assign19160_e14067_d_n9, assign19160_e14067_d_n10, assign19160_e14067_d_n11, assign19160_e14067_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19160_e14069;
        locals.var_tmf2_dn0 = assign19160_e14069_d_n0;
        locals.var_tmf2_dn2 = assign19160_e14069_d_n2;
        locals.var_tmf2_dn4 = assign19160_e14069_d_n4;
        locals.var_tmf2_dn5 = assign19160_e14069_d_n5;
        locals.var_tmf2_dn6 = assign19160_e14069_d_n6;
        locals.var_tmf2_dn7 = assign19160_e14069_d_n7;
        locals.var_tmf2_dn8 = assign19160_e14069_d_n8;
        locals.var_tmf2_dn9 = assign19160_e14069_d_n9;
        locals.var_tmf2_dn10 = assign19160_e14069_d_n10;
        locals.var_tmf2_dn11 = assign19160_e14069_d_n11;
        locals.var_tmf2_dn14 = assign19160_e14069_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19170_e14082, assign19170_e14082_d_n0, assign19170_e14082_d_n2, assign19170_e14082_d_n4, assign19170_e14082_d_n5, assign19170_e14082_d_n6, assign19170_e14082_d_n7, assign19170_e14082_d_n8, assign19170_e14082_d_n9, assign19170_e14082_d_n10, assign19170_e14082_d_n11, assign19170_e14082_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19170_e14077: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19170_e14079: f64 = (assign19170_e14077 + locals.var_tmf2);
        let assign19170_e14080: f64 = (assign19170_e14079).sqrt();
        (assign19170_e14080, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19170_e14080)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19170_e14080)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19170_e14082;
        locals.var_tmf2_dn0 = assign19170_e14082_d_n0;
        locals.var_tmf2_dn2 = assign19170_e14082_d_n2;
        locals.var_tmf2_dn4 = assign19170_e14082_d_n4;
        locals.var_tmf2_dn5 = assign19170_e14082_d_n5;
        locals.var_tmf2_dn6 = assign19170_e14082_d_n6;
        locals.var_tmf2_dn7 = assign19170_e14082_d_n7;
        locals.var_tmf2_dn8 = assign19170_e14082_d_n8;
        locals.var_tmf2_dn9 = assign19170_e14082_d_n9;
        locals.var_tmf2_dn10 = assign19170_e14082_d_n10;
        locals.var_tmf2_dn11 = assign19170_e14082_d_n11;
        locals.var_tmf2_dn14 = assign19170_e14082_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19180_e14096, assign19180_e14096_d_n0, assign19180_e14096_d_n2, assign19180_e14096_d_n4, assign19180_e14096_d_n5, assign19180_e14096_d_n6, assign19180_e14096_d_n7, assign19180_e14096_d_n8, assign19180_e14096_d_n9, assign19180_e14096_d_n10, assign19180_e14096_d_n11, assign19180_e14096_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19180_e14092: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19180_e14093: f64 = (1.0 + assign19180_e14092);
        let assign19180_e14094: f64 = (0.5 * assign19180_e14093);
        (assign19180_e14094, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19180_e14096;
        locals.var_t6_dn0 = assign19180_e14096_d_n0;
        locals.var_t6_dn2 = assign19180_e14096_d_n2;
        locals.var_t6_dn4 = assign19180_e14096_d_n4;
        locals.var_t6_dn5 = assign19180_e14096_d_n5;
        locals.var_t6_dn6 = assign19180_e14096_d_n6;
        locals.var_t6_dn7 = assign19180_e14096_d_n7;
        locals.var_t6_dn8 = assign19180_e14096_d_n8;
        locals.var_t6_dn9 = assign19180_e14096_d_n9;
        locals.var_t6_dn10 = assign19180_e14096_d_n10;
        locals.var_t6_dn11 = assign19180_e14096_d_n11;
        locals.var_t6_dn14 = assign19180_e14096_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19190_e14110, assign19190_e14110_d_n0, assign19190_e14110_d_n2, assign19190_e14110_d_n4, assign19190_e14110_d_n5, assign19190_e14110_d_n6, assign19190_e14110_d_n7, assign19190_e14110_d_n8, assign19190_e14110_d_n9, assign19190_e14110_d_n10, assign19190_e14110_d_n11, assign19190_e14110_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19190_e14106: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19190_e14107: f64 = (0.5 * assign19190_e14106);
        let assign19190_e14108: f64 = (locals.var_t4 + assign19190_e14107);
        (assign19190_e14108, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign19190_e14110;
        locals.var_t5_dn0 = assign19190_e14110_d_n0;
        locals.var_t5_dn2 = assign19190_e14110_d_n2;
        locals.var_t5_dn4 = assign19190_e14110_d_n4;
        locals.var_t5_dn5 = assign19190_e14110_d_n5;
        locals.var_t5_dn6 = assign19190_e14110_d_n6;
        locals.var_t5_dn7 = assign19190_e14110_d_n7;
        locals.var_t5_dn8 = assign19190_e14110_d_n8;
        locals.var_t5_dn9 = assign19190_e14110_d_n9;
        locals.var_t5_dn10 = assign19190_e14110_d_n10;
        locals.var_t5_dn11 = assign19190_e14110_d_n11;
        locals.var_t5_dn14 = assign19190_e14110_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign19200_e14126, assign19200_e14126_d_n0, assign19200_e14126_d_n2, assign19200_e14126_d_n4, assign19200_e14126_d_n5, assign19200_e14126_d_n6, assign19200_e14126_d_n7, assign19200_e14126_d_n8, assign19200_e14126_d_n9, assign19200_e14126_d_n10, assign19200_e14126_d_n11, assign19200_e14126_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19200_e14119: f64 = (p.p98 + 1.0);
        let assign19200_e14120: f64 = (locals.var_t4 * assign19200_e14119);
        let assign19200_e14122: f64 = (assign19200_e14120 - locals.var_t5);
        let assign19200_e14124: f64 = (assign19200_e14122 - 5e-5);
        (assign19200_e14124, ((locals.var_t4_dn0 * assign19200_e14119) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign19200_e14119) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign19200_e14119) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign19200_e14119) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign19200_e14119) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign19200_e14119) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign19200_e14119) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign19200_e14119) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign19200_e14119) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign19200_e14119) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign19200_e14119) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19200_e14126;
        locals.var_tmf1_dn0 = assign19200_e14126_d_n0;
        locals.var_tmf1_dn2 = assign19200_e14126_d_n2;
        locals.var_tmf1_dn4 = assign19200_e14126_d_n4;
        locals.var_tmf1_dn5 = assign19200_e14126_d_n5;
        locals.var_tmf1_dn6 = assign19200_e14126_d_n6;
        locals.var_tmf1_dn7 = assign19200_e14126_d_n7;
        locals.var_tmf1_dn8 = assign19200_e14126_d_n8;
        locals.var_tmf1_dn9 = assign19200_e14126_d_n9;
        locals.var_tmf1_dn10 = assign19200_e14126_d_n10;
        locals.var_tmf1_dn11 = assign19200_e14126_d_n11;
        locals.var_tmf1_dn14 = assign19200_e14126_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19210_e14142, assign19210_e14142_d_n0, assign19210_e14142_d_n2, assign19210_e14142_d_n4, assign19210_e14142_d_n5, assign19210_e14142_d_n6, assign19210_e14142_d_n7, assign19210_e14142_d_n8, assign19210_e14142_d_n9, assign19210_e14142_d_n10, assign19210_e14142_d_n11, assign19210_e14142_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19210_e14136: f64 = (p.p98 + 1.0);
        let assign19210_e14137: f64 = (locals.var_t4 * assign19210_e14136);
        let assign19210_e14138: f64 = (4.0 * assign19210_e14137);
        let assign19210_e14140: f64 = (assign19210_e14138 * 5e-5);
        (assign19210_e14140, ((4.0 * (locals.var_t4_dn0 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign19210_e14136)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign19210_e14136)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19210_e14142;
        locals.var_tmf2_dn0 = assign19210_e14142_d_n0;
        locals.var_tmf2_dn2 = assign19210_e14142_d_n2;
        locals.var_tmf2_dn4 = assign19210_e14142_d_n4;
        locals.var_tmf2_dn5 = assign19210_e14142_d_n5;
        locals.var_tmf2_dn6 = assign19210_e14142_d_n6;
        locals.var_tmf2_dn7 = assign19210_e14142_d_n7;
        locals.var_tmf2_dn8 = assign19210_e14142_d_n8;
        locals.var_tmf2_dn9 = assign19210_e14142_d_n9;
        locals.var_tmf2_dn10 = assign19210_e14142_d_n10;
        locals.var_tmf2_dn11 = assign19210_e14142_d_n11;
        locals.var_tmf2_dn14 = assign19210_e14142_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19220_e14156, assign19220_e14156_d_n0, assign19220_e14156_d_n2, assign19220_e14156_d_n4, assign19220_e14156_d_n5, assign19220_e14156_d_n6, assign19220_e14156_d_n7, assign19220_e14156_d_n8, assign19220_e14156_d_n9, assign19220_e14156_d_n10, assign19220_e14156_d_n11, assign19220_e14156_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19220_e14154, assign19220_e14154_d_n0, assign19220_e14154_d_n2, assign19220_e14154_d_n4, assign19220_e14154_d_n5, assign19220_e14154_d_n6, assign19220_e14154_d_n7, assign19220_e14154_d_n8, assign19220_e14154_d_n9, assign19220_e14154_d_n10, assign19220_e14154_d_n11, assign19220_e14154_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19220_e14153: f64 = (-locals.var_tmf2);
                (assign19220_e14153, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19220_e14154, assign19220_e14154_d_n0, assign19220_e14154_d_n2, assign19220_e14154_d_n4, assign19220_e14154_d_n5, assign19220_e14154_d_n6, assign19220_e14154_d_n7, assign19220_e14154_d_n8, assign19220_e14154_d_n9, assign19220_e14154_d_n10, assign19220_e14154_d_n11, assign19220_e14154_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19220_e14156;
        locals.var_tmf2_dn0 = assign19220_e14156_d_n0;
        locals.var_tmf2_dn2 = assign19220_e14156_d_n2;
        locals.var_tmf2_dn4 = assign19220_e14156_d_n4;
        locals.var_tmf2_dn5 = assign19220_e14156_d_n5;
        locals.var_tmf2_dn6 = assign19220_e14156_d_n6;
        locals.var_tmf2_dn7 = assign19220_e14156_d_n7;
        locals.var_tmf2_dn8 = assign19220_e14156_d_n8;
        locals.var_tmf2_dn9 = assign19220_e14156_d_n9;
        locals.var_tmf2_dn10 = assign19220_e14156_d_n10;
        locals.var_tmf2_dn11 = assign19220_e14156_d_n11;
        locals.var_tmf2_dn14 = assign19220_e14156_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19230_e14169, assign19230_e14169_d_n0, assign19230_e14169_d_n2, assign19230_e14169_d_n4, assign19230_e14169_d_n5, assign19230_e14169_d_n6, assign19230_e14169_d_n7, assign19230_e14169_d_n8, assign19230_e14169_d_n9, assign19230_e14169_d_n10, assign19230_e14169_d_n11, assign19230_e14169_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19230_e14164: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19230_e14166: f64 = (assign19230_e14164 + locals.var_tmf2);
        let assign19230_e14167: f64 = (assign19230_e14166).sqrt();
        (assign19230_e14167, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19230_e14167)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19230_e14167)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19230_e14169;
        locals.var_tmf2_dn0 = assign19230_e14169_d_n0;
        locals.var_tmf2_dn2 = assign19230_e14169_d_n2;
        locals.var_tmf2_dn4 = assign19230_e14169_d_n4;
        locals.var_tmf2_dn5 = assign19230_e14169_d_n5;
        locals.var_tmf2_dn6 = assign19230_e14169_d_n6;
        locals.var_tmf2_dn7 = assign19230_e14169_d_n7;
        locals.var_tmf2_dn8 = assign19230_e14169_d_n8;
        locals.var_tmf2_dn9 = assign19230_e14169_d_n9;
        locals.var_tmf2_dn10 = assign19230_e14169_d_n10;
        locals.var_tmf2_dn11 = assign19230_e14169_d_n11;
        locals.var_tmf2_dn14 = assign19230_e14169_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19240_e14183, assign19240_e14183_d_n0, assign19240_e14183_d_n2, assign19240_e14183_d_n4, assign19240_e14183_d_n5, assign19240_e14183_d_n6, assign19240_e14183_d_n7, assign19240_e14183_d_n8, assign19240_e14183_d_n9, assign19240_e14183_d_n10, assign19240_e14183_d_n11, assign19240_e14183_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19240_e14179: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19240_e14180: f64 = (1.0 + assign19240_e14179);
        let assign19240_e14181: f64 = (0.5 * assign19240_e14180);
        (assign19240_e14181, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19240_e14183;
        locals.var_t6_dn0 = assign19240_e14183_d_n0;
        locals.var_t6_dn2 = assign19240_e14183_d_n2;
        locals.var_t6_dn4 = assign19240_e14183_d_n4;
        locals.var_t6_dn5 = assign19240_e14183_d_n5;
        locals.var_t6_dn6 = assign19240_e14183_d_n6;
        locals.var_t6_dn7 = assign19240_e14183_d_n7;
        locals.var_t6_dn8 = assign19240_e14183_d_n8;
        locals.var_t6_dn9 = assign19240_e14183_d_n9;
        locals.var_t6_dn10 = assign19240_e14183_d_n10;
        locals.var_t6_dn11 = assign19240_e14183_d_n11;
        locals.var_t6_dn14 = assign19240_e14183_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19250_e14201, assign19250_e14201_d_n0, assign19250_e14201_d_n2, assign19250_e14201_d_n4, assign19250_e14201_d_n5, assign19250_e14201_d_n6, assign19250_e14201_d_n7, assign19250_e14201_d_n8, assign19250_e14201_d_n9, assign19250_e14201_d_n10, assign19250_e14201_d_n11, assign19250_e14201_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19250_e14192: f64 = (p.p98 + 1.0);
        let assign19250_e14193: f64 = (locals.var_t4 * assign19250_e14192);
        let assign19250_e14197: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19250_e14198: f64 = (0.5 * assign19250_e14197);
        let assign19250_e14199: f64 = (assign19250_e14193 - assign19250_e14198);
        (assign19250_e14199, ((locals.var_t4_dn0 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign19250_e14192) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign19250_e14201;
        locals.var_t7_dn0 = assign19250_e14201_d_n0;
        locals.var_t7_dn2 = assign19250_e14201_d_n2;
        locals.var_t7_dn4 = assign19250_e14201_d_n4;
        locals.var_t7_dn5 = assign19250_e14201_d_n5;
        locals.var_t7_dn6 = assign19250_e14201_d_n6;
        locals.var_t7_dn7 = assign19250_e14201_d_n7;
        locals.var_t7_dn8 = assign19250_e14201_d_n8;
        locals.var_t7_dn9 = assign19250_e14201_d_n9;
        locals.var_t7_dn10 = assign19250_e14201_d_n10;
        locals.var_t7_dn11 = assign19250_e14201_d_n11;
        locals.var_t7_dn14 = assign19250_e14201_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign19260_e14217, assign19260_e14217_d_n0, assign19260_e14217_d_n2, assign19260_e14217_d_n4, assign19260_e14217_d_n5, assign19260_e14217_d_n6, assign19260_e14217_d_n7, assign19260_e14217_d_n8, assign19260_e14217_d_n9, assign19260_e14217_d_n10, assign19260_e14217_d_n11, assign19260_e14217_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19260_e14210: f64 = (locals.var_t1 * locals.var_t4);
        let assign19260_e14211: f64 = (locals.var_t7 + assign19260_e14210);
        let assign19260_e14213: f64 = assign19260_e14211;
        let assign19260_e14215: f64 = (assign19260_e14213 - 5e-5);
        (assign19260_e14215, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19260_e14217;
        locals.var_tmf1_dn0 = assign19260_e14217_d_n0;
        locals.var_tmf1_dn2 = assign19260_e14217_d_n2;
        locals.var_tmf1_dn4 = assign19260_e14217_d_n4;
        locals.var_tmf1_dn5 = assign19260_e14217_d_n5;
        locals.var_tmf1_dn6 = assign19260_e14217_d_n6;
        locals.var_tmf1_dn7 = assign19260_e14217_d_n7;
        locals.var_tmf1_dn8 = assign19260_e14217_d_n8;
        locals.var_tmf1_dn9 = assign19260_e14217_d_n9;
        locals.var_tmf1_dn10 = assign19260_e14217_d_n10;
        locals.var_tmf1_dn11 = assign19260_e14217_d_n11;
        locals.var_tmf1_dn14 = assign19260_e14217_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19270_e14229, assign19270_e14229_d_n0, assign19270_e14229_d_n2, assign19270_e14229_d_n4, assign19270_e14229_d_n5, assign19270_e14229_d_n6, assign19270_e14229_d_n7, assign19270_e14229_d_n8, assign19270_e14229_d_n9, assign19270_e14229_d_n10, assign19270_e14229_d_n11, assign19270_e14229_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19270_e14229;
        locals.var_tmf2_dn0 = assign19270_e14229_d_n0;
        locals.var_tmf2_dn2 = assign19270_e14229_d_n2;
        locals.var_tmf2_dn4 = assign19270_e14229_d_n4;
        locals.var_tmf2_dn5 = assign19270_e14229_d_n5;
        locals.var_tmf2_dn6 = assign19270_e14229_d_n6;
        locals.var_tmf2_dn7 = assign19270_e14229_d_n7;
        locals.var_tmf2_dn8 = assign19270_e14229_d_n8;
        locals.var_tmf2_dn9 = assign19270_e14229_d_n9;
        locals.var_tmf2_dn10 = assign19270_e14229_d_n10;
        locals.var_tmf2_dn11 = assign19270_e14229_d_n11;
        locals.var_tmf2_dn14 = assign19270_e14229_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19280_e14243, assign19280_e14243_d_n0, assign19280_e14243_d_n2, assign19280_e14243_d_n4, assign19280_e14243_d_n5, assign19280_e14243_d_n6, assign19280_e14243_d_n7, assign19280_e14243_d_n8, assign19280_e14243_d_n9, assign19280_e14243_d_n10, assign19280_e14243_d_n11, assign19280_e14243_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign19280_e14241, assign19280_e14241_d_n0, assign19280_e14241_d_n2, assign19280_e14241_d_n4, assign19280_e14241_d_n5, assign19280_e14241_d_n6, assign19280_e14241_d_n7, assign19280_e14241_d_n8, assign19280_e14241_d_n9, assign19280_e14241_d_n10, assign19280_e14241_d_n11, assign19280_e14241_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19280_e14240: f64 = (-locals.var_tmf2);
                (assign19280_e14240, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19280_e14241, assign19280_e14241_d_n0, assign19280_e14241_d_n2, assign19280_e14241_d_n4, assign19280_e14241_d_n5, assign19280_e14241_d_n6, assign19280_e14241_d_n7, assign19280_e14241_d_n8, assign19280_e14241_d_n9, assign19280_e14241_d_n10, assign19280_e14241_d_n11, assign19280_e14241_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19280_e14243;
        locals.var_tmf2_dn0 = assign19280_e14243_d_n0;
        locals.var_tmf2_dn2 = assign19280_e14243_d_n2;
        locals.var_tmf2_dn4 = assign19280_e14243_d_n4;
        locals.var_tmf2_dn5 = assign19280_e14243_d_n5;
        locals.var_tmf2_dn6 = assign19280_e14243_d_n6;
        locals.var_tmf2_dn7 = assign19280_e14243_d_n7;
        locals.var_tmf2_dn8 = assign19280_e14243_d_n8;
        locals.var_tmf2_dn9 = assign19280_e14243_d_n9;
        locals.var_tmf2_dn10 = assign19280_e14243_d_n10;
        locals.var_tmf2_dn11 = assign19280_e14243_d_n11;
        locals.var_tmf2_dn14 = assign19280_e14243_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19290_e14256, assign19290_e14256_d_n0, assign19290_e14256_d_n2, assign19290_e14256_d_n4, assign19290_e14256_d_n5, assign19290_e14256_d_n6, assign19290_e14256_d_n7, assign19290_e14256_d_n8, assign19290_e14256_d_n9, assign19290_e14256_d_n10, assign19290_e14256_d_n11, assign19290_e14256_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19290_e14251: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19290_e14253: f64 = (assign19290_e14251 + locals.var_tmf2);
        let assign19290_e14254: f64 = (assign19290_e14253).sqrt();
        (assign19290_e14254, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19290_e14254)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19290_e14254)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19290_e14256;
        locals.var_tmf2_dn0 = assign19290_e14256_d_n0;
        locals.var_tmf2_dn2 = assign19290_e14256_d_n2;
        locals.var_tmf2_dn4 = assign19290_e14256_d_n4;
        locals.var_tmf2_dn5 = assign19290_e14256_d_n5;
        locals.var_tmf2_dn6 = assign19290_e14256_d_n6;
        locals.var_tmf2_dn7 = assign19290_e14256_d_n7;
        locals.var_tmf2_dn8 = assign19290_e14256_d_n8;
        locals.var_tmf2_dn9 = assign19290_e14256_d_n9;
        locals.var_tmf2_dn10 = assign19290_e14256_d_n10;
        locals.var_tmf2_dn11 = assign19290_e14256_d_n11;
        locals.var_tmf2_dn14 = assign19290_e14256_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19300_e14270, assign19300_e14270_d_n0, assign19300_e14270_d_n2, assign19300_e14270_d_n4, assign19300_e14270_d_n5, assign19300_e14270_d_n6, assign19300_e14270_d_n7, assign19300_e14270_d_n8, assign19300_e14270_d_n9, assign19300_e14270_d_n10, assign19300_e14270_d_n11, assign19300_e14270_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19300_e14266: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19300_e14267: f64 = (1.0 + assign19300_e14266);
        let assign19300_e14268: f64 = (0.5 * assign19300_e14267);
        (assign19300_e14268, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19300_e14270;
        locals.var_t6_dn0 = assign19300_e14270_d_n0;
        locals.var_t6_dn2 = assign19300_e14270_d_n2;
        locals.var_t6_dn4 = assign19300_e14270_d_n4;
        locals.var_t6_dn5 = assign19300_e14270_d_n5;
        locals.var_t6_dn6 = assign19300_e14270_d_n6;
        locals.var_t6_dn7 = assign19300_e14270_d_n7;
        locals.var_t6_dn8 = assign19300_e14270_d_n8;
        locals.var_t6_dn9 = assign19300_e14270_d_n9;
        locals.var_t6_dn10 = assign19300_e14270_d_n10;
        locals.var_t6_dn11 = assign19300_e14270_d_n11;
        locals.var_t6_dn14 = assign19300_e14270_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19310_e14284, assign19310_e14284_d_n0, assign19310_e14284_d_n2, assign19310_e14284_d_n4, assign19310_e14284_d_n5, assign19310_e14284_d_n6, assign19310_e14284_d_n7, assign19310_e14284_d_n8, assign19310_e14284_d_n9, assign19310_e14284_d_n10, assign19310_e14284_d_n11, assign19310_e14284_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19310_e14280: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19310_e14281: f64 = (0.5 * assign19310_e14280);
        let assign19310_e14282: f64 = assign19310_e14281;
        (assign19310_e14282, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19310_e14284;
        locals.var_t2_dn0 = assign19310_e14284_d_n0;
        locals.var_t2_dn2 = assign19310_e14284_d_n2;
        locals.var_t2_dn4 = assign19310_e14284_d_n4;
        locals.var_t2_dn5 = assign19310_e14284_d_n5;
        locals.var_t2_dn6 = assign19310_e14284_d_n6;
        locals.var_t2_dn7 = assign19310_e14284_d_n7;
        locals.var_t2_dn8 = assign19310_e14284_d_n8;
        locals.var_t2_dn9 = assign19310_e14284_d_n9;
        locals.var_t2_dn10 = assign19310_e14284_d_n10;
        locals.var_t2_dn11 = assign19310_e14284_d_n11;
        locals.var_t2_dn14 = assign19310_e14284_d_n14;
        locals.var_t2_rv = 0.0;

        let assign19320_e14291: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard390 = assign19320_e14291;
        locals.var_guard390_rv = 0.0;

        let (assign19330_e14311, assign19330_e14311_d_n0, assign19330_e14311_d_n2, assign19330_e14311_d_n4, assign19330_e14311_d_n5, assign19330_e14311_d_n6, assign19330_e14311_d_n7, assign19330_e14311_d_n8, assign19330_e14311_d_n9, assign19330_e14311_d_n10, assign19330_e14311_d_n11, assign19330_e14311_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19330_e14302: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign19330_e14303: f64 = (locals.var_uc_rdvd + assign19330_e14302);
        let assign19330_e14306: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign19330_e14307: f64 = (assign19330_e14303 + assign19330_e14306);
        let assign19330_e14309: f64 = (assign19330_e14307 * locals.var_t2);
        (assign19330_e14309, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign19330_e14307 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19330_e14311;
        locals.var_rsvde_dn0 = assign19330_e14311_d_n0;
        locals.var_rsvde_dn2 = assign19330_e14311_d_n2;
        locals.var_rsvde_dn4 = assign19330_e14311_d_n4;
        locals.var_rsvde_dn5 = assign19330_e14311_d_n5;
        locals.var_rsvde_dn6 = assign19330_e14311_d_n6;
        locals.var_rsvde_dn7 = assign19330_e14311_d_n7;
        locals.var_rsvde_dn8 = assign19330_e14311_d_n8;
        locals.var_rsvde_dn9 = assign19330_e14311_d_n9;
        locals.var_rsvde_dn10 = assign19330_e14311_d_n10;
        locals.var_rsvde_dn11 = assign19330_e14311_d_n11;
        locals.var_rsvde_dn14 = assign19330_e14311_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19340_e14329, assign19340_e14329_d_n0, assign19340_e14329_d_n2, assign19340_e14329_d_n4, assign19340_e14329_d_n5, assign19340_e14329_d_n6, assign19340_e14329_d_n7, assign19340_e14329_d_n8, assign19340_e14329_d_n9, assign19340_e14329_d_n10, assign19340_e14329_d_n11, assign19340_e14329_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19340_e14322: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19340_e14323: f64 = (locals.var_rsvde - assign19340_e14322);
        let assign19340_e14326: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19340_e14327: f64 = (assign19340_e14323 - assign19340_e14326);
        (assign19340_e14327, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19340_e14329;
        locals.var_tmf1_dn0 = assign19340_e14329_d_n0;
        locals.var_tmf1_dn2 = assign19340_e14329_d_n2;
        locals.var_tmf1_dn4 = assign19340_e14329_d_n4;
        locals.var_tmf1_dn5 = assign19340_e14329_d_n5;
        locals.var_tmf1_dn6 = assign19340_e14329_d_n6;
        locals.var_tmf1_dn7 = assign19340_e14329_d_n7;
        locals.var_tmf1_dn8 = assign19340_e14329_d_n8;
        locals.var_tmf1_dn9 = assign19340_e14329_d_n9;
        locals.var_tmf1_dn10 = assign19340_e14329_d_n10;
        locals.var_tmf1_dn11 = assign19340_e14329_d_n11;
        locals.var_tmf1_dn14 = assign19340_e14329_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19350_e14347, assign19350_e14347_d_n0, assign19350_e14347_d_n2, assign19350_e14347_d_n4, assign19350_e14347_d_n5, assign19350_e14347_d_n6, assign19350_e14347_d_n7, assign19350_e14347_d_n8, assign19350_e14347_d_n9, assign19350_e14347_d_n10, assign19350_e14347_d_n11, assign19350_e14347_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19350_e14340: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19350_e14341: f64 = (4.0 * assign19350_e14340);
        let assign19350_e14344: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19350_e14345: f64 = (assign19350_e14341 * assign19350_e14344);
        (assign19350_e14345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19350_e14347;
        locals.var_tmf2_dn0 = assign19350_e14347_d_n0;
        locals.var_tmf2_dn2 = assign19350_e14347_d_n2;
        locals.var_tmf2_dn4 = assign19350_e14347_d_n4;
        locals.var_tmf2_dn5 = assign19350_e14347_d_n5;
        locals.var_tmf2_dn6 = assign19350_e14347_d_n6;
        locals.var_tmf2_dn7 = assign19350_e14347_d_n7;
        locals.var_tmf2_dn8 = assign19350_e14347_d_n8;
        locals.var_tmf2_dn9 = assign19350_e14347_d_n9;
        locals.var_tmf2_dn10 = assign19350_e14347_d_n10;
        locals.var_tmf2_dn11 = assign19350_e14347_d_n11;
        locals.var_tmf2_dn14 = assign19350_e14347_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19360_e14363, assign19360_e14363_d_n0, assign19360_e14363_d_n2, assign19360_e14363_d_n4, assign19360_e14363_d_n5, assign19360_e14363_d_n6, assign19360_e14363_d_n7, assign19360_e14363_d_n8, assign19360_e14363_d_n9, assign19360_e14363_d_n10, assign19360_e14363_d_n11, assign19360_e14363_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let (assign19360_e14361, assign19360_e14361_d_n0, assign19360_e14361_d_n2, assign19360_e14361_d_n4, assign19360_e14361_d_n5, assign19360_e14361_d_n6, assign19360_e14361_d_n7, assign19360_e14361_d_n8, assign19360_e14361_d_n9, assign19360_e14361_d_n10, assign19360_e14361_d_n11, assign19360_e14361_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19360_e14360: f64 = (-locals.var_tmf2);
                (assign19360_e14360, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19360_e14361, assign19360_e14361_d_n0, assign19360_e14361_d_n2, assign19360_e14361_d_n4, assign19360_e14361_d_n5, assign19360_e14361_d_n6, assign19360_e14361_d_n7, assign19360_e14361_d_n8, assign19360_e14361_d_n9, assign19360_e14361_d_n10, assign19360_e14361_d_n11, assign19360_e14361_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19360_e14363;
        locals.var_tmf2_dn0 = assign19360_e14363_d_n0;
        locals.var_tmf2_dn2 = assign19360_e14363_d_n2;
        locals.var_tmf2_dn4 = assign19360_e14363_d_n4;
        locals.var_tmf2_dn5 = assign19360_e14363_d_n5;
        locals.var_tmf2_dn6 = assign19360_e14363_d_n6;
        locals.var_tmf2_dn7 = assign19360_e14363_d_n7;
        locals.var_tmf2_dn8 = assign19360_e14363_d_n8;
        locals.var_tmf2_dn9 = assign19360_e14363_d_n9;
        locals.var_tmf2_dn10 = assign19360_e14363_d_n10;
        locals.var_tmf2_dn11 = assign19360_e14363_d_n11;
        locals.var_tmf2_dn14 = assign19360_e14363_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19370_e14378, assign19370_e14378_d_n0, assign19370_e14378_d_n2, assign19370_e14378_d_n4, assign19370_e14378_d_n5, assign19370_e14378_d_n6, assign19370_e14378_d_n7, assign19370_e14378_d_n8, assign19370_e14378_d_n9, assign19370_e14378_d_n10, assign19370_e14378_d_n11, assign19370_e14378_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19370_e14373: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19370_e14375: f64 = (assign19370_e14373 + locals.var_tmf2);
        let assign19370_e14376: f64 = (assign19370_e14375).sqrt();
        (assign19370_e14376, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19370_e14376)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19370_e14376)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19370_e14378;
        locals.var_tmf2_dn0 = assign19370_e14378_d_n0;
        locals.var_tmf2_dn2 = assign19370_e14378_d_n2;
        locals.var_tmf2_dn4 = assign19370_e14378_d_n4;
        locals.var_tmf2_dn5 = assign19370_e14378_d_n5;
        locals.var_tmf2_dn6 = assign19370_e14378_d_n6;
        locals.var_tmf2_dn7 = assign19370_e14378_d_n7;
        locals.var_tmf2_dn8 = assign19370_e14378_d_n8;
        locals.var_tmf2_dn9 = assign19370_e14378_d_n9;
        locals.var_tmf2_dn10 = assign19370_e14378_d_n10;
        locals.var_tmf2_dn11 = assign19370_e14378_d_n11;
        locals.var_tmf2_dn14 = assign19370_e14378_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19380_e14394, assign19380_e14394_d_n0, assign19380_e14394_d_n2, assign19380_e14394_d_n4, assign19380_e14394_d_n5, assign19380_e14394_d_n6, assign19380_e14394_d_n7, assign19380_e14394_d_n8, assign19380_e14394_d_n9, assign19380_e14394_d_n10, assign19380_e14394_d_n11, assign19380_e14394_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19380_e14390: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19380_e14391: f64 = (1.0 + assign19380_e14390);
        let assign19380_e14392: f64 = (0.5 * assign19380_e14391);
        (assign19380_e14392, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19380_e14394;
        locals.var_t0_dn0 = assign19380_e14394_d_n0;
        locals.var_t0_dn2 = assign19380_e14394_d_n2;
        locals.var_t0_dn4 = assign19380_e14394_d_n4;
        locals.var_t0_dn5 = assign19380_e14394_d_n5;
        locals.var_t0_dn6 = assign19380_e14394_d_n6;
        locals.var_t0_dn7 = assign19380_e14394_d_n7;
        locals.var_t0_dn8 = assign19380_e14394_d_n8;
        locals.var_t0_dn9 = assign19380_e14394_d_n9;
        locals.var_t0_dn10 = assign19380_e14394_d_n10;
        locals.var_t0_dn11 = assign19380_e14394_d_n11;
        locals.var_t0_dn14 = assign19380_e14394_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19390_e14412, assign19390_e14412_d_n0, assign19390_e14412_d_n2, assign19390_e14412_d_n4, assign19390_e14412_d_n5, assign19390_e14412_d_n6, assign19390_e14412_d_n7, assign19390_e14412_d_n8, assign19390_e14412_d_n9, assign19390_e14412_d_n10, assign19390_e14412_d_n11, assign19390_e14412_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign19390_e14404: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19390_e14408: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19390_e14409: f64 = (0.5 * assign19390_e14408);
        let assign19390_e14410: f64 = (assign19390_e14404 + assign19390_e14409);
        (assign19390_e14410, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19390_e14412;
        locals.var_rsvde_dn0 = assign19390_e14412_d_n0;
        locals.var_rsvde_dn2 = assign19390_e14412_d_n2;
        locals.var_rsvde_dn4 = assign19390_e14412_d_n4;
        locals.var_rsvde_dn5 = assign19390_e14412_d_n5;
        locals.var_rsvde_dn6 = assign19390_e14412_d_n6;
        locals.var_rsvde_dn7 = assign19390_e14412_d_n7;
        locals.var_rsvde_dn8 = assign19390_e14412_d_n8;
        locals.var_rsvde_dn9 = assign19390_e14412_d_n9;
        locals.var_rsvde_dn10 = assign19390_e14412_d_n10;
        locals.var_rsvde_dn11 = assign19390_e14412_d_n11;
        locals.var_rsvde_dn14 = assign19390_e14412_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19400_e14433, assign19400_e14433_d_n0, assign19400_e14433_d_n2, assign19400_e14433_d_n4, assign19400_e14433_d_n5, assign19400_e14433_d_n6, assign19400_e14433_d_n7, assign19400_e14433_d_n8, assign19400_e14433_d_n9, assign19400_e14433_d_n10, assign19400_e14433_d_n11, assign19400_e14433_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19400_e14424: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19400_e14425: f64 = (locals.var_uc_rdvd + assign19400_e14424);
        let assign19400_e14428: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19400_e14429: f64 = (assign19400_e14425 + assign19400_e14428);
        let assign19400_e14431: f64 = (assign19400_e14429 * locals.var_t2);
        (assign19400_e14431, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19400_e14429 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19400_e14433;
        locals.var_rsvde_dn0 = assign19400_e14433_d_n0;
        locals.var_rsvde_dn2 = assign19400_e14433_d_n2;
        locals.var_rsvde_dn4 = assign19400_e14433_d_n4;
        locals.var_rsvde_dn5 = assign19400_e14433_d_n5;
        locals.var_rsvde_dn6 = assign19400_e14433_d_n6;
        locals.var_rsvde_dn7 = assign19400_e14433_d_n7;
        locals.var_rsvde_dn8 = assign19400_e14433_d_n8;
        locals.var_rsvde_dn9 = assign19400_e14433_d_n9;
        locals.var_rsvde_dn10 = assign19400_e14433_d_n10;
        locals.var_rsvde_dn11 = assign19400_e14433_d_n11;
        locals.var_rsvde_dn14 = assign19400_e14433_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19410_e14452, assign19410_e14452_d_n0, assign19410_e14452_d_n2, assign19410_e14452_d_n4, assign19410_e14452_d_n5, assign19410_e14452_d_n6, assign19410_e14452_d_n7, assign19410_e14452_d_n8, assign19410_e14452_d_n9, assign19410_e14452_d_n10, assign19410_e14452_d_n11, assign19410_e14452_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19410_e14445: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19410_e14446: f64 = (locals.var_rsvde - assign19410_e14445);
        let assign19410_e14449: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19410_e14450: f64 = (assign19410_e14446 - assign19410_e14449);
        (assign19410_e14450, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19410_e14452;
        locals.var_tmf1_dn0 = assign19410_e14452_d_n0;
        locals.var_tmf1_dn2 = assign19410_e14452_d_n2;
        locals.var_tmf1_dn4 = assign19410_e14452_d_n4;
        locals.var_tmf1_dn5 = assign19410_e14452_d_n5;
        locals.var_tmf1_dn6 = assign19410_e14452_d_n6;
        locals.var_tmf1_dn7 = assign19410_e14452_d_n7;
        locals.var_tmf1_dn8 = assign19410_e14452_d_n8;
        locals.var_tmf1_dn9 = assign19410_e14452_d_n9;
        locals.var_tmf1_dn10 = assign19410_e14452_d_n10;
        locals.var_tmf1_dn11 = assign19410_e14452_d_n11;
        locals.var_tmf1_dn14 = assign19410_e14452_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19420_e14471, assign19420_e14471_d_n0, assign19420_e14471_d_n2, assign19420_e14471_d_n4, assign19420_e14471_d_n5, assign19420_e14471_d_n6, assign19420_e14471_d_n7, assign19420_e14471_d_n8, assign19420_e14471_d_n9, assign19420_e14471_d_n10, assign19420_e14471_d_n11, assign19420_e14471_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19420_e14464: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19420_e14465: f64 = (4.0 * assign19420_e14464);
        let assign19420_e14468: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19420_e14469: f64 = (assign19420_e14465 * assign19420_e14468);
        (assign19420_e14469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19420_e14471;
        locals.var_tmf2_dn0 = assign19420_e14471_d_n0;
        locals.var_tmf2_dn2 = assign19420_e14471_d_n2;
        locals.var_tmf2_dn4 = assign19420_e14471_d_n4;
        locals.var_tmf2_dn5 = assign19420_e14471_d_n5;
        locals.var_tmf2_dn6 = assign19420_e14471_d_n6;
        locals.var_tmf2_dn7 = assign19420_e14471_d_n7;
        locals.var_tmf2_dn8 = assign19420_e14471_d_n8;
        locals.var_tmf2_dn9 = assign19420_e14471_d_n9;
        locals.var_tmf2_dn10 = assign19420_e14471_d_n10;
        locals.var_tmf2_dn11 = assign19420_e14471_d_n11;
        locals.var_tmf2_dn14 = assign19420_e14471_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19430_e14488, assign19430_e14488_d_n0, assign19430_e14488_d_n2, assign19430_e14488_d_n4, assign19430_e14488_d_n5, assign19430_e14488_d_n6, assign19430_e14488_d_n7, assign19430_e14488_d_n8, assign19430_e14488_d_n9, assign19430_e14488_d_n10, assign19430_e14488_d_n11, assign19430_e14488_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let (assign19430_e14486, assign19430_e14486_d_n0, assign19430_e14486_d_n2, assign19430_e14486_d_n4, assign19430_e14486_d_n5, assign19430_e14486_d_n6, assign19430_e14486_d_n7, assign19430_e14486_d_n8, assign19430_e14486_d_n9, assign19430_e14486_d_n10, assign19430_e14486_d_n11, assign19430_e14486_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19430_e14485: f64 = (-locals.var_tmf2);
                (assign19430_e14485, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19430_e14486, assign19430_e14486_d_n0, assign19430_e14486_d_n2, assign19430_e14486_d_n4, assign19430_e14486_d_n5, assign19430_e14486_d_n6, assign19430_e14486_d_n7, assign19430_e14486_d_n8, assign19430_e14486_d_n9, assign19430_e14486_d_n10, assign19430_e14486_d_n11, assign19430_e14486_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19430_e14488;
        locals.var_tmf2_dn0 = assign19430_e14488_d_n0;
        locals.var_tmf2_dn2 = assign19430_e14488_d_n2;
        locals.var_tmf2_dn4 = assign19430_e14488_d_n4;
        locals.var_tmf2_dn5 = assign19430_e14488_d_n5;
        locals.var_tmf2_dn6 = assign19430_e14488_d_n6;
        locals.var_tmf2_dn7 = assign19430_e14488_d_n7;
        locals.var_tmf2_dn8 = assign19430_e14488_d_n8;
        locals.var_tmf2_dn9 = assign19430_e14488_d_n9;
        locals.var_tmf2_dn10 = assign19430_e14488_d_n10;
        locals.var_tmf2_dn11 = assign19430_e14488_d_n11;
        locals.var_tmf2_dn14 = assign19430_e14488_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign19440_e14504, assign19440_e14504_d_n0, assign19440_e14504_d_n2, assign19440_e14504_d_n4, assign19440_e14504_d_n5, assign19440_e14504_d_n6, assign19440_e14504_d_n7, assign19440_e14504_d_n8, assign19440_e14504_d_n9, assign19440_e14504_d_n10, assign19440_e14504_d_n11, assign19440_e14504_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19440_e14499: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19440_e14501: f64 = (assign19440_e14499 + locals.var_tmf2);
        let assign19440_e14502: f64 = (assign19440_e14501).sqrt();
        (assign19440_e14502, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19440_e14502)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19440_e14502)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19440_e14504;
        locals.var_tmf2_dn0 = assign19440_e14504_d_n0;
        locals.var_tmf2_dn2 = assign19440_e14504_d_n2;
        locals.var_tmf2_dn4 = assign19440_e14504_d_n4;
        locals.var_tmf2_dn5 = assign19440_e14504_d_n5;
        locals.var_tmf2_dn6 = assign19440_e14504_d_n6;
        locals.var_tmf2_dn7 = assign19440_e14504_d_n7;
        locals.var_tmf2_dn8 = assign19440_e14504_d_n8;
        locals.var_tmf2_dn9 = assign19440_e14504_d_n9;
        locals.var_tmf2_dn10 = assign19440_e14504_d_n10;
        locals.var_tmf2_dn11 = assign19440_e14504_d_n11;
        locals.var_tmf2_dn14 = assign19440_e14504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19450_e14521, assign19450_e14521_d_n0, assign19450_e14521_d_n2, assign19450_e14521_d_n4, assign19450_e14521_d_n5, assign19450_e14521_d_n6, assign19450_e14521_d_n7, assign19450_e14521_d_n8, assign19450_e14521_d_n9, assign19450_e14521_d_n10, assign19450_e14521_d_n11, assign19450_e14521_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19450_e14517: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19450_e14518: f64 = (1.0 + assign19450_e14517);
        let assign19450_e14519: f64 = (0.5 * assign19450_e14518);
        (assign19450_e14519, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19450_e14521;
        locals.var_t0_dn0 = assign19450_e14521_d_n0;
        locals.var_t0_dn2 = assign19450_e14521_d_n2;
        locals.var_t0_dn4 = assign19450_e14521_d_n4;
        locals.var_t0_dn5 = assign19450_e14521_d_n5;
        locals.var_t0_dn6 = assign19450_e14521_d_n6;
        locals.var_t0_dn7 = assign19450_e14521_d_n7;
        locals.var_t0_dn8 = assign19450_e14521_d_n8;
        locals.var_t0_dn9 = assign19450_e14521_d_n9;
        locals.var_t0_dn10 = assign19450_e14521_d_n10;
        locals.var_t0_dn11 = assign19450_e14521_d_n11;
        locals.var_t0_dn14 = assign19450_e14521_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19460_e14540, assign19460_e14540_d_n0, assign19460_e14540_d_n2, assign19460_e14540_d_n4, assign19460_e14540_d_n5, assign19460_e14540_d_n6, assign19460_e14540_d_n7, assign19460_e14540_d_n8, assign19460_e14540_d_n9, assign19460_e14540_d_n10, assign19460_e14540_d_n11, assign19460_e14540_d_n14,) = {
    if ((((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 != 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign19460_e14532: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19460_e14536: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19460_e14537: f64 = (0.5 * assign19460_e14536);
        let assign19460_e14538: f64 = (assign19460_e14532 + assign19460_e14537);
        (assign19460_e14538, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19460_e14540;
        locals.var_rsvde_dn0 = assign19460_e14540_d_n0;
        locals.var_rsvde_dn2 = assign19460_e14540_d_n2;
        locals.var_rsvde_dn4 = assign19460_e14540_d_n4;
        locals.var_rsvde_dn5 = assign19460_e14540_d_n5;
        locals.var_rsvde_dn6 = assign19460_e14540_d_n6;
        locals.var_rsvde_dn7 = assign19460_e14540_d_n7;
        locals.var_rsvde_dn8 = assign19460_e14540_d_n8;
        locals.var_rsvde_dn9 = assign19460_e14540_d_n9;
        locals.var_rsvde_dn10 = assign19460_e14540_d_n10;
        locals.var_rsvde_dn11 = assign19460_e14540_d_n11;
        locals.var_rsvde_dn14 = assign19460_e14540_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19470_e14549, assign19470_e14549_d_n0, assign19470_e14549_d_n2, assign19470_e14549_d_n4, assign19470_e14549_d_n5, assign19470_e14549_d_n6, assign19470_e14549_d_n7, assign19470_e14549_d_n8, assign19470_e14549_d_n9, assign19470_e14549_d_n10, assign19470_e14549_d_n11, assign19470_e14549_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19470_e14549;
        locals.var_rdvde_dn0 = assign19470_e14549_d_n0;
        locals.var_rdvde_dn2 = assign19470_e14549_d_n2;
        locals.var_rdvde_dn4 = assign19470_e14549_d_n4;
        locals.var_rdvde_dn5 = assign19470_e14549_d_n5;
        locals.var_rdvde_dn6 = assign19470_e14549_d_n6;
        locals.var_rdvde_dn7 = assign19470_e14549_d_n7;
        locals.var_rdvde_dn8 = assign19470_e14549_d_n8;
        locals.var_rdvde_dn9 = assign19470_e14549_d_n9;
        locals.var_rdvde_dn10 = assign19470_e14549_d_n10;
        locals.var_rdvde_dn11 = assign19470_e14549_d_n11;
        locals.var_rdvde_dn14 = assign19470_e14549_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19480_e14558, assign19480_e14558_d_n0, assign19480_e14558_d_n2, assign19480_e14558_d_n4, assign19480_e14558_d_n5, assign19480_e14558_d_n6, assign19480_e14558_d_n7, assign19480_e14558_d_n8, assign19480_e14558_d_n9, assign19480_e14558_d_n10, assign19480_e14558_d_n11, assign19480_e14558_d_n14,) = {
    if (((locals.var_guard356 != 0.0) && (locals.var_guard382 != 0.0)) && (locals.var_guard387 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19480_e14558;
        locals.var_rsvde_dn0 = assign19480_e14558_d_n0;
        locals.var_rsvde_dn2 = assign19480_e14558_d_n2;
        locals.var_rsvde_dn4 = assign19480_e14558_d_n4;
        locals.var_rsvde_dn5 = assign19480_e14558_d_n5;
        locals.var_rsvde_dn6 = assign19480_e14558_d_n6;
        locals.var_rsvde_dn7 = assign19480_e14558_d_n7;
        locals.var_rsvde_dn8 = assign19480_e14558_d_n8;
        locals.var_rsvde_dn9 = assign19480_e14558_d_n9;
        locals.var_rsvde_dn10 = assign19480_e14558_d_n10;
        locals.var_rsvde_dn11 = assign19480_e14558_d_n11;
        locals.var_rsvde_dn14 = assign19480_e14558_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19490_e14565, assign19490_e14565_d_n0, assign19490_e14565_d_n2, assign19490_e14565_d_n4, assign19490_e14565_d_n5, assign19490_e14565_d_n6, assign19490_e14565_d_n7, assign19490_e14565_d_n8, assign19490_e14565_d_n9, assign19490_e14565_d_n10, assign19490_e14565_d_n11, assign19490_e14565_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19490_e14562: f64 = (locals.var_beta_inv).sqrt();
        let assign19490_e14563: f64 = (locals.var_costi00 * assign19490_e14562);
        (assign19490_e14563, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn11 / (2.0 * assign19490_e14562))), (locals.var_costi00 * (locals.var_beta_inv_dn14 / (2.0 * assign19490_e14562))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn14,)
    }
};
        locals.var_costi0 = assign19490_e14565;
        locals.var_costi0_dn0 = assign19490_e14565_d_n0;
        locals.var_costi0_dn2 = assign19490_e14565_d_n2;
        locals.var_costi0_dn4 = assign19490_e14565_d_n4;
        locals.var_costi0_dn5 = assign19490_e14565_d_n5;
        locals.var_costi0_dn6 = assign19490_e14565_d_n6;
        locals.var_costi0_dn7 = assign19490_e14565_d_n7;
        locals.var_costi0_dn8 = assign19490_e14565_d_n8;
        locals.var_costi0_dn9 = assign19490_e14565_d_n9;
        locals.var_costi0_dn10 = assign19490_e14565_d_n10;
        locals.var_costi0_dn11 = assign19490_e14565_d_n11;
        locals.var_costi0_dn14 = assign19490_e14565_d_n14;
        locals.var_costi0_rv = 0.0;

        let (assign19500_e14571, assign19500_e14571_d_n0, assign19500_e14571_d_n2, assign19500_e14571_d_n4, assign19500_e14571_d_n5, assign19500_e14571_d_n6, assign19500_e14571_d_n7, assign19500_e14571_d_n8, assign19500_e14571_d_n9, assign19500_e14571_d_n10, assign19500_e14571_d_n11, assign19500_e14571_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19500_e14569: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign19500_e14569, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)), ((locals.var_costi0_dn14 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn14)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn11, locals.var_costi0_p2_dn14,)
    }
};
        locals.var_costi0_p2 = assign19500_e14571;
        locals.var_costi0_p2_dn0 = assign19500_e14571_d_n0;
        locals.var_costi0_p2_dn2 = assign19500_e14571_d_n2;
        locals.var_costi0_p2_dn4 = assign19500_e14571_d_n4;
        locals.var_costi0_p2_dn5 = assign19500_e14571_d_n5;
        locals.var_costi0_p2_dn6 = assign19500_e14571_d_n6;
        locals.var_costi0_p2_dn7 = assign19500_e14571_d_n7;
        locals.var_costi0_p2_dn8 = assign19500_e14571_d_n8;
        locals.var_costi0_p2_dn9 = assign19500_e14571_d_n9;
        locals.var_costi0_p2_dn10 = assign19500_e14571_d_n10;
        locals.var_costi0_p2_dn11 = assign19500_e14571_d_n11;
        locals.var_costi0_p2_dn14 = assign19500_e14571_d_n14;
        locals.var_costi0_p2_rv = 0.0;

        let (assign19510_e14579, assign19510_e14579_d_n0, assign19510_e14579_d_n2, assign19510_e14579_d_n4, assign19510_e14579_d_n5, assign19510_e14579_d_n6, assign19510_e14579_d_n7, assign19510_e14579_d_n8, assign19510_e14579_d_n9, assign19510_e14579_d_n10, assign19510_e14579_d_n11, assign19510_e14579_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19510_e14575: f64 = (locals.var_nin * locals.var_nin);
        let assign19510_e14577: f64 = (assign19510_e14575 * locals.var_nsti_p2);
        (assign19510_e14577, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2), (((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn14,)
    }
};
        locals.var_costi1 = assign19510_e14579;
        locals.var_costi1_dn0 = assign19510_e14579_d_n0;
        locals.var_costi1_dn2 = assign19510_e14579_d_n2;
        locals.var_costi1_dn4 = assign19510_e14579_d_n4;
        locals.var_costi1_dn5 = assign19510_e14579_d_n5;
        locals.var_costi1_dn6 = assign19510_e14579_d_n6;
        locals.var_costi1_dn7 = assign19510_e14579_d_n7;
        locals.var_costi1_dn8 = assign19510_e14579_d_n8;
        locals.var_costi1_dn9 = assign19510_e14579_d_n9;
        locals.var_costi1_dn10 = assign19510_e14579_d_n10;
        locals.var_costi1_dn11 = assign19510_e14579_d_n11;
        locals.var_costi1_dn14 = assign19510_e14579_d_n14;
        locals.var_costi1_rv = 0.0;

        let (assign19520_e14587, assign19520_e14587_d_n0, assign19520_e14587_d_n2, assign19520_e14587_d_n4, assign19520_e14587_d_n5, assign19520_e14587_d_n6, assign19520_e14587_d_n7, assign19520_e14587_d_n8, assign19520_e14587_d_n9, assign19520_e14587_d_n10, assign19520_e14587_d_n11, assign19520_e14587_d_n14,) = {
    if (locals.var_guard356 != 0.0) {
        let assign19520_e14584: f64 = (p.p448 * locals.var_tdiff);
        let assign19520_e14585: f64 = (p.p447 + assign19520_e14584);
        (assign19520_e14585, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn11), (p.p448 * locals.var_tdiff_dn14),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    }
};
        locals.var_hbdceff = assign19520_e14587;
        locals.var_hbdceff_dn0 = assign19520_e14587_d_n0;
        locals.var_hbdceff_dn2 = assign19520_e14587_d_n2;
        locals.var_hbdceff_dn4 = assign19520_e14587_d_n4;
        locals.var_hbdceff_dn5 = assign19520_e14587_d_n5;
        locals.var_hbdceff_dn6 = assign19520_e14587_d_n6;
        locals.var_hbdceff_dn7 = assign19520_e14587_d_n7;
        locals.var_hbdceff_dn8 = assign19520_e14587_d_n8;
        locals.var_hbdceff_dn9 = assign19520_e14587_d_n9;
        locals.var_hbdceff_dn10 = assign19520_e14587_d_n10;
        locals.var_hbdceff_dn11 = assign19520_e14587_d_n11;
        locals.var_hbdceff_dn14 = assign19520_e14587_d_n14;
        locals.var_hbdceff_rv = 0.0;

        let (assign19530_e14591,) = {
    if (locals.var_guard356 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19530_e14591;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19560_e14604: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign19560_e14604;
        locals.var_guard393_rv = 0.0;

        let (assign19570_e14610,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19570_e14610;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19580_e14613: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign19580_e14613;
        locals.var_guard394_rv = 0.0;

        let (assign19590_e14619,) = {
    if ((locals.var_guard356 != 0.0) && (locals.var_guard394 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19590_e14619;
        locals.var_uc_subtmp_rv = 0.0;

        let (assign19600_e14626, assign19600_e14626_d_n0, assign19600_e14626_d_n2, assign19600_e14626_d_n4, assign19600_e14626_d_n5, assign19600_e14626_d_n6, assign19600_e14626_d_n7, assign19600_e14626_d_n8, assign19600_e14626_d_n9, assign19600_e14626_d_n10, assign19600_e14626_d_n11, assign19600_e14626_d_n14,) = {
    if (locals.var_guard356 == 0.0) {
        let assign19600_e14622: f64 = ctx_temp;
        let assign19600_e14624: f64 = (assign19600_e14622 + p.p11);
        (assign19600_e14624, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign19600_e14626;
        locals.var_ttemp_dn0 = assign19600_e14626_d_n0;
        locals.var_ttemp_dn2 = assign19600_e14626_d_n2;
        locals.var_ttemp_dn4 = assign19600_e14626_d_n4;
        locals.var_ttemp_dn5 = assign19600_e14626_d_n5;
        locals.var_ttemp_dn6 = assign19600_e14626_d_n6;
        locals.var_ttemp_dn7 = assign19600_e14626_d_n7;
        locals.var_ttemp_dn8 = assign19600_e14626_d_n8;
        locals.var_ttemp_dn9 = assign19600_e14626_d_n9;
        locals.var_ttemp_dn10 = assign19600_e14626_d_n10;
        locals.var_ttemp_dn11 = assign19600_e14626_d_n11;
        locals.var_ttemp_dn14 = assign19600_e14626_d_n14;
        locals.var_ttemp_rv = 0.0;

        let assign19610_e14629: f64 = (locals.var_weff_ld * p.p7);
        locals.var_weffld_nf = assign19610_e14629;
        locals.var_weffld_nf_rv = 0.0;

        let assign19620_e14632: f64 = (p.p67 + p.p68);
        locals.var_ldrift0 = assign19620_e14632;
        locals.var_ldrift0_rv = 0.0;

        locals.var_vfb = locals.var_uc_vfbc;
        locals.var_vfb_rv = 0.0;

        locals.var_vmaxe = locals.var_vmaxeff;
        locals.var_vmaxe_dn0 = locals.var_vmaxeff_dn0;
        locals.var_vmaxe_dn2 = locals.var_vmaxeff_dn2;
        locals.var_vmaxe_dn4 = locals.var_vmaxeff_dn4;
        locals.var_vmaxe_dn5 = locals.var_vmaxeff_dn5;
        locals.var_vmaxe_dn6 = locals.var_vmaxeff_dn6;
        locals.var_vmaxe_dn7 = locals.var_vmaxeff_dn7;
        locals.var_vmaxe_dn8 = locals.var_vmaxeff_dn8;
        locals.var_vmaxe_dn9 = locals.var_vmaxeff_dn9;
        locals.var_vmaxe_dn10 = locals.var_vmaxeff_dn10;
        locals.var_vmaxe_dn11 = locals.var_vmaxeff_dn11;
        locals.var_vmaxe_dn14 = locals.var_vmaxeff_dn14;
        locals.var_vmaxe_rv = 0.0;

        locals.var_c_eox = locals.var_cecox;
        locals.var_c_eox_rv = 0.0;

        locals.var_tox0 = p.p95;
        locals.var_tox0_rv = 0.0;

        let assign19670_e14639: f64 = (locals.var_c_eox / locals.var_tox0);
        locals.var_cox0 = assign19670_e14639;
        locals.var_cox0_rv = 0.0;

        let assign19680_e14642: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign19680_e14642;
        locals.var_cox0_inv_rv = 0.0;

        let assign19690_e14645: f64 = (locals.var_c_eox / locals.var_uc_toxb);
        locals.var_coxb0 = assign19690_e14645;
        locals.var_coxb0_rv = 0.0;

        let assign19700_e14648: f64 = (p.p87 * p.p434);
        locals.var_vgs_min = assign19700_e14648;
        locals.var_vgs_min_rv = 0.0;

        let assign19710_e14652: f64 = (locals.var_pb2 - p.p262);
        let assign19710_e14653: f64 = (0.8 - assign19710_e14652);
        let assign19710_e14655: f64 = (assign19710_e14653 - 0.1);
        locals.var_tmf1 = assign19710_e14655;
        locals.var_tmf1_dn0 = (-locals.var_pb2_dn0);
        locals.var_tmf1_dn2 = (-locals.var_pb2_dn2);
        locals.var_tmf1_dn4 = (-locals.var_pb2_dn4);
        locals.var_tmf1_dn5 = (-locals.var_pb2_dn5);
        locals.var_tmf1_dn6 = (-locals.var_pb2_dn6);
        locals.var_tmf1_dn7 = (-locals.var_pb2_dn7);
        locals.var_tmf1_dn8 = (-locals.var_pb2_dn8);
        locals.var_tmf1_dn9 = (-locals.var_pb2_dn9);
        locals.var_tmf1_dn10 = (-locals.var_pb2_dn10);
        locals.var_tmf1_dn11 = (-locals.var_pb2_dn11);
        locals.var_tmf1_dn14 = (-locals.var_pb2_dn14);
        locals.var_tmf1_rv = 0.0;

        let assign19720_e14658: f64 = (4.0 * 0.8);
        let assign19720_e14660: f64 = (assign19720_e14658 * 0.1);
        locals.var_tmf2 = assign19720_e14660;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign19730_e14667, assign19730_e14667_d_n0, assign19730_e14667_d_n2, assign19730_e14667_d_n4, assign19730_e14667_d_n5, assign19730_e14667_d_n6, assign19730_e14667_d_n7, assign19730_e14667_d_n8, assign19730_e14667_d_n9, assign19730_e14667_d_n10, assign19730_e14667_d_n11, assign19730_e14667_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign19730_e14666: f64 = (-locals.var_tmf2);
        (assign19730_e14666, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign19730_e14667;
        locals.var_tmf2_dn0 = assign19730_e14667_d_n0;
        locals.var_tmf2_dn2 = assign19730_e14667_d_n2;
        locals.var_tmf2_dn4 = assign19730_e14667_d_n4;
        locals.var_tmf2_dn5 = assign19730_e14667_d_n5;
        locals.var_tmf2_dn6 = assign19730_e14667_d_n6;
        locals.var_tmf2_dn7 = assign19730_e14667_d_n7;
        locals.var_tmf2_dn8 = assign19730_e14667_d_n8;
        locals.var_tmf2_dn9 = assign19730_e14667_d_n9;
        locals.var_tmf2_dn10 = assign19730_e14667_d_n10;
        locals.var_tmf2_dn11 = assign19730_e14667_d_n11;
        locals.var_tmf2_dn14 = assign19730_e14667_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign19740_e14670: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19740_e14672: f64 = (assign19740_e14670 + locals.var_tmf2);
        let assign19740_e14673: f64 = (assign19740_e14672).sqrt();
        locals.var_tmf2 = assign19740_e14673;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19740_e14673));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19740_e14673));
        locals.var_tmf2_rv = 0.0;

        let assign19750_e14678: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19750_e14679: f64 = (1.0 + assign19750_e14678);
        let assign19750_e14680: f64 = (0.5 * assign19750_e14679);
        locals.var_t0 = assign19750_e14680;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign19760_e14685: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19760_e14686: f64 = (0.5 * assign19760_e14685);
        let assign19760_e14687: f64 = (0.8 - assign19760_e14686);
        locals.var_t1 = assign19760_e14687;
        locals.var_t1_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn9 = (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)));
        locals.var_t1_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn14 = (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)));
        locals.var_t1_rv = 0.0;

        locals.var_vbs_max = locals.var_t1;
        locals.var_vbs_max_dn0 = locals.var_t1_dn0;
        locals.var_vbs_max_dn2 = locals.var_t1_dn2;
        locals.var_vbs_max_dn4 = locals.var_t1_dn4;
        locals.var_vbs_max_dn5 = locals.var_t1_dn5;
        locals.var_vbs_max_dn6 = locals.var_t1_dn6;
        locals.var_vbs_max_dn7 = locals.var_t1_dn7;
        locals.var_vbs_max_dn8 = locals.var_t1_dn8;
        locals.var_vbs_max_dn9 = locals.var_t1_dn9;
        locals.var_vbs_max_dn10 = locals.var_t1_dn10;
        locals.var_vbs_max_dn11 = locals.var_t1_dn11;
        locals.var_vbs_max_dn14 = locals.var_t1_dn14;
        locals.var_vbs_max_rv = 0.0;

        let assign19780_e14691: f64 = (locals.var_pb20 - p.p262);
        let assign19780_e14693: f64 = if assign19780_e14691 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard395 = assign19780_e14693;
        locals.var_guard395_rv = 0.0;

        let (assign19790_e14699, assign19790_e14699_d_n0, assign19790_e14699_d_n2, assign19790_e14699_d_n4, assign19790_e14699_d_n5, assign19790_e14699_d_n6, assign19790_e14699_d_n7, assign19790_e14699_d_n8, assign19790_e14699_d_n9, assign19790_e14699_d_n10, assign19790_e14699_d_n11, assign19790_e14699_d_n14,) = {
    if (locals.var_guard395 != 0.0) {
        let assign19790_e14697: f64 = (locals.var_pb20 - p.p262);
        (assign19790_e14697, locals.var_pb20_dn0, locals.var_pb20_dn2, locals.var_pb20_dn4, locals.var_pb20_dn5, locals.var_pb20_dn6, locals.var_pb20_dn7, locals.var_pb20_dn8, locals.var_pb20_dn9, locals.var_pb20_dn10, locals.var_pb20_dn11, locals.var_pb20_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19790_e14699;
        locals.var_vbs_max_dn0 = assign19790_e14699_d_n0;
        locals.var_vbs_max_dn2 = assign19790_e14699_d_n2;
        locals.var_vbs_max_dn4 = assign19790_e14699_d_n4;
        locals.var_vbs_max_dn5 = assign19790_e14699_d_n5;
        locals.var_vbs_max_dn6 = assign19790_e14699_d_n6;
        locals.var_vbs_max_dn7 = assign19790_e14699_d_n7;
        locals.var_vbs_max_dn8 = assign19790_e14699_d_n8;
        locals.var_vbs_max_dn9 = assign19790_e14699_d_n9;
        locals.var_vbs_max_dn10 = assign19790_e14699_d_n10;
        locals.var_vbs_max_dn11 = assign19790_e14699_d_n11;
        locals.var_vbs_max_dn14 = assign19790_e14699_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19800_e14702: f64 = (locals.var_pb2c - p.p262);
        let assign19800_e14704: f64 = if assign19800_e14702 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard396 = assign19800_e14704;
        locals.var_guard396_rv = 0.0;

        let (assign19810_e14710, assign19810_e14710_d_n0, assign19810_e14710_d_n2, assign19810_e14710_d_n4, assign19810_e14710_d_n5, assign19810_e14710_d_n6, assign19810_e14710_d_n7, assign19810_e14710_d_n8, assign19810_e14710_d_n9, assign19810_e14710_d_n10, assign19810_e14710_d_n11, assign19810_e14710_d_n14,) = {
    if (locals.var_guard396 != 0.0) {
        let assign19810_e14708: f64 = (locals.var_pb2c - p.p262);
        (assign19810_e14708, locals.var_pb2c_dn0, locals.var_pb2c_dn2, locals.var_pb2c_dn4, locals.var_pb2c_dn5, locals.var_pb2c_dn6, locals.var_pb2c_dn7, locals.var_pb2c_dn8, locals.var_pb2c_dn9, locals.var_pb2c_dn10, locals.var_pb2c_dn11, locals.var_pb2c_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19810_e14710;
        locals.var_vbs_max_dn0 = assign19810_e14710_d_n0;
        locals.var_vbs_max_dn2 = assign19810_e14710_d_n2;
        locals.var_vbs_max_dn4 = assign19810_e14710_d_n4;
        locals.var_vbs_max_dn5 = assign19810_e14710_d_n5;
        locals.var_vbs_max_dn6 = assign19810_e14710_d_n6;
        locals.var_vbs_max_dn7 = assign19810_e14710_d_n7;
        locals.var_vbs_max_dn8 = assign19810_e14710_d_n8;
        locals.var_vbs_max_dn9 = assign19810_e14710_d_n9;
        locals.var_vbs_max_dn10 = assign19810_e14710_d_n10;
        locals.var_vbs_max_dn11 = assign19810_e14710_d_n11;
        locals.var_vbs_max_dn14 = assign19810_e14710_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19820_e14717: f64 = if ((locals.var_uc_codep > 0.0) && (locals.var_uc_codep <= 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign19820_e14717;
        locals.var_guard397_rv = 0.0;

        let assign19830_e14720: f64 = (locals.var_pb2n - p.p262);
        let assign19830_e14722: f64 = if assign19830_e14720 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard398 = assign19830_e14722;
        locals.var_guard398_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign19840_e14730, assign19840_e14730_d_n0, assign19840_e14730_d_n2, assign19840_e14730_d_n4, assign19840_e14730_d_n5, assign19840_e14730_d_n6, assign19840_e14730_d_n7, assign19840_e14730_d_n8, assign19840_e14730_d_n9, assign19840_e14730_d_n10, assign19840_e14730_d_n11, assign19840_e14730_d_n14,) = {
    if ((locals.var_guard397 != 0.0) && (locals.var_guard398 != 0.0)) {
        let assign19840_e14728: f64 = (locals.var_pb2n - p.p262);
        (assign19840_e14728, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19840_e14730;
        locals.var_vbs_max_dn0 = assign19840_e14730_d_n0;
        locals.var_vbs_max_dn2 = assign19840_e14730_d_n2;
        locals.var_vbs_max_dn4 = assign19840_e14730_d_n4;
        locals.var_vbs_max_dn5 = assign19840_e14730_d_n5;
        locals.var_vbs_max_dn6 = assign19840_e14730_d_n6;
        locals.var_vbs_max_dn7 = assign19840_e14730_d_n7;
        locals.var_vbs_max_dn8 = assign19840_e14730_d_n8;
        locals.var_vbs_max_dn9 = assign19840_e14730_d_n9;
        locals.var_vbs_max_dn10 = assign19840_e14730_d_n10;
        locals.var_vbs_max_dn11 = assign19840_e14730_d_n11;
        locals.var_vbs_max_dn14 = assign19840_e14730_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19850_e14733: f64 = (locals.var_vbipn - p.p262);
        let assign19850_e14735: f64 = if assign19850_e14733 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard399 = assign19850_e14735;
        locals.var_guard399_rv = 0.0;

        let (assign19860_e14743, assign19860_e14743_d_n0, assign19860_e14743_d_n2, assign19860_e14743_d_n4, assign19860_e14743_d_n5, assign19860_e14743_d_n6, assign19860_e14743_d_n7, assign19860_e14743_d_n8, assign19860_e14743_d_n9, assign19860_e14743_d_n10, assign19860_e14743_d_n11, assign19860_e14743_d_n14,) = {
    if ((locals.var_guard397 != 0.0) && (locals.var_guard399 != 0.0)) {
        let assign19860_e14741: f64 = (locals.var_vbipn - p.p262);
        (assign19860_e14741, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19860_e14743;
        locals.var_vbs_max_dn0 = assign19860_e14743_d_n0;
        locals.var_vbs_max_dn2 = assign19860_e14743_d_n2;
        locals.var_vbs_max_dn4 = assign19860_e14743_d_n4;
        locals.var_vbs_max_dn5 = assign19860_e14743_d_n5;
        locals.var_vbs_max_dn6 = assign19860_e14743_d_n6;
        locals.var_vbs_max_dn7 = assign19860_e14743_d_n7;
        locals.var_vbs_max_dn8 = assign19860_e14743_d_n8;
        locals.var_vbs_max_dn9 = assign19860_e14743_d_n9;
        locals.var_vbs_max_dn10 = assign19860_e14743_d_n10;
        locals.var_vbs_max_dn11 = assign19860_e14743_d_n11;
        locals.var_vbs_max_dn14 = assign19860_e14743_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19870_e14747: f64 = (locals.var_vbs_max * 0.5);
        let assign19870_e14748: f64 = if locals.var_vbs_bnd > assign19870_e14747 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign19870_e14748;
        locals.var_guard400_rv = 0.0;

        let (assign19880_e14754, assign19880_e14754_d_n0, assign19880_e14754_d_n2, assign19880_e14754_d_n4, assign19880_e14754_d_n5, assign19880_e14754_d_n6, assign19880_e14754_d_n7, assign19880_e14754_d_n8, assign19880_e14754_d_n9, assign19880_e14754_d_n10, assign19880_e14754_d_n11, assign19880_e14754_d_n14,) = {
    if (locals.var_guard400 != 0.0) {
        let assign19880_e14752: f64 = (0.5 * locals.var_vbs_max);
        (assign19880_e14752, (0.5 * locals.var_vbs_max_dn0), (0.5 * locals.var_vbs_max_dn2), (0.5 * locals.var_vbs_max_dn4), (0.5 * locals.var_vbs_max_dn5), (0.5 * locals.var_vbs_max_dn6), (0.5 * locals.var_vbs_max_dn7), (0.5 * locals.var_vbs_max_dn8), (0.5 * locals.var_vbs_max_dn9), (0.5 * locals.var_vbs_max_dn10), (0.5 * locals.var_vbs_max_dn11), (0.5 * locals.var_vbs_max_dn14),)
    } else {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    }
};
        locals.var_vbs_bnd = assign19880_e14754;
        locals.var_vbs_bnd_dn0 = assign19880_e14754_d_n0;
        locals.var_vbs_bnd_dn2 = assign19880_e14754_d_n2;
        locals.var_vbs_bnd_dn4 = assign19880_e14754_d_n4;
        locals.var_vbs_bnd_dn5 = assign19880_e14754_d_n5;
        locals.var_vbs_bnd_dn6 = assign19880_e14754_d_n6;
        locals.var_vbs_bnd_dn7 = assign19880_e14754_d_n7;
        locals.var_vbs_bnd_dn8 = assign19880_e14754_d_n8;
        locals.var_vbs_bnd_dn9 = assign19880_e14754_d_n9;
        locals.var_vbs_bnd_dn10 = assign19880_e14754_d_n10;
        locals.var_vbs_bnd_dn11 = assign19880_e14754_d_n11;
        locals.var_vbs_bnd_dn14 = assign19880_e14754_d_n14;
        locals.var_vbs_bnd_rv = 0.0;

        let assign19890_e14756: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard401 = assign19890_e14756;
        locals.var_guard401_rv = 0.0;

        let (assign19900_e14760, assign19900_e14760_d_n0, assign19900_e14760_d_n2, assign19900_e14760_d_n4, assign19900_e14760_d_n5, assign19900_e14760_d_n6, assign19900_e14760_d_n7, assign19900_e14760_d_n8, assign19900_e14760_d_n9, assign19900_e14760_d_n10, assign19900_e14760_d_n11, assign19900_e14760_d_n14,) = {
    if (locals.var_guard401 != 0.0) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19900_e14760;
        locals.var_vbs_max_local_dn0 = assign19900_e14760_d_n0;
        locals.var_vbs_max_local_dn2 = assign19900_e14760_d_n2;
        locals.var_vbs_max_local_dn4 = assign19900_e14760_d_n4;
        locals.var_vbs_max_local_dn5 = assign19900_e14760_d_n5;
        locals.var_vbs_max_local_dn6 = assign19900_e14760_d_n6;
        locals.var_vbs_max_local_dn7 = assign19900_e14760_d_n7;
        locals.var_vbs_max_local_dn8 = assign19900_e14760_d_n8;
        locals.var_vbs_max_local_dn9 = assign19900_e14760_d_n9;
        locals.var_vbs_max_local_dn10 = assign19900_e14760_d_n10;
        locals.var_vbs_max_local_dn11 = assign19900_e14760_d_n11;
        locals.var_vbs_max_local_dn14 = assign19900_e14760_d_n14;
        locals.var_vbs_max_local_rv = 0.0;

        let (assign19910_e14765, assign19910_e14765_d_n0, assign19910_e14765_d_n2, assign19910_e14765_d_n4, assign19910_e14765_d_n5, assign19910_e14765_d_n6, assign19910_e14765_d_n7, assign19910_e14765_d_n8, assign19910_e14765_d_n9, assign19910_e14765_d_n10, assign19910_e14765_d_n11, assign19910_e14765_d_n14,) = {
    if (locals.var_guard401 == 0.0) {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19910_e14765;
        locals.var_vbs_max_local_dn0 = assign19910_e14765_d_n0;
        locals.var_vbs_max_local_dn2 = assign19910_e14765_d_n2;
        locals.var_vbs_max_local_dn4 = assign19910_e14765_d_n4;
        locals.var_vbs_max_local_dn5 = assign19910_e14765_d_n5;
        locals.var_vbs_max_local_dn6 = assign19910_e14765_d_n6;
        locals.var_vbs_max_local_dn7 = assign19910_e14765_d_n7;
        locals.var_vbs_max_local_dn8 = assign19910_e14765_d_n8;
        locals.var_vbs_max_local_dn9 = assign19910_e14765_d_n9;
        locals.var_vbs_max_local_dn10 = assign19910_e14765_d_n10;
        locals.var_vbs_max_local_dn11 = assign19910_e14765_d_n11;
        locals.var_vbs_max_local_dn14 = assign19910_e14765_d_n14;
        locals.var_vbs_max_local_rv = 0.0;

        let assign19920_e14767: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard402 = assign19920_e14767;
        locals.var_guard402_rv = 0.0;

        let (assign19930_e14771, assign19930_e14771_d_n0, assign19930_e14771_d_n2, assign19930_e14771_d_n4, assign19930_e14771_d_n5, assign19930_e14771_d_n6, assign19930_e14771_d_n7, assign19930_e14771_d_n8, assign19930_e14771_d_n9, assign19930_e14771_d_n10, assign19930_e14771_d_n11, assign19930_e14771_d_n14,) = {
    if (locals.var_guard402 != 0.0) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19930_e14771;
        locals.var_vbs_bnd_local_dn0 = assign19930_e14771_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19930_e14771_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19930_e14771_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19930_e14771_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19930_e14771_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19930_e14771_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19930_e14771_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19930_e14771_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19930_e14771_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19930_e14771_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19930_e14771_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19940_e14773: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard403 = assign19940_e14773;
        locals.var_guard403_rv = 0.0;

        let (assign19950_e14782, assign19950_e14782_d_n0, assign19950_e14782_d_n2, assign19950_e14782_d_n4, assign19950_e14782_d_n5, assign19950_e14782_d_n6, assign19950_e14782_d_n7, assign19950_e14782_d_n8, assign19950_e14782_d_n9, assign19950_e14782_d_n10, assign19950_e14782_d_n11, assign19950_e14782_d_n14,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 != 0.0)) {
        let assign19950_e14780: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19950_e14780, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19950_e14782;
        locals.var_vbs_bnd_local_dn0 = assign19950_e14782_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19950_e14782_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19950_e14782_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19950_e14782_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19950_e14782_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19950_e14782_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19950_e14782_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19950_e14782_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19950_e14782_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19950_e14782_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19950_e14782_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let (assign19960_e14790, assign19960_e14790_d_n0, assign19960_e14790_d_n2, assign19960_e14790_d_n4, assign19960_e14790_d_n5, assign19960_e14790_d_n6, assign19960_e14790_d_n7, assign19960_e14790_d_n8, assign19960_e14790_d_n9, assign19960_e14790_d_n10, assign19960_e14790_d_n11, assign19960_e14790_d_n14,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 == 0.0)) {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19960_e14790;
        locals.var_vbs_bnd_local_dn0 = assign19960_e14790_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19960_e14790_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19960_e14790_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19960_e14790_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19960_e14790_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19960_e14790_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19960_e14790_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19960_e14790_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19960_e14790_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19960_e14790_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19960_e14790_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19970_e14794: f64 = (locals.var_vbs_max_local * 0.5);
        let assign19970_e14795: f64 = if locals.var_vbs_bnd_local > assign19970_e14794 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign19970_e14795;
        locals.var_guard404_rv = 0.0;

        let (assign19980_e14801, assign19980_e14801_d_n0, assign19980_e14801_d_n2, assign19980_e14801_d_n4, assign19980_e14801_d_n5, assign19980_e14801_d_n6, assign19980_e14801_d_n7, assign19980_e14801_d_n8, assign19980_e14801_d_n9, assign19980_e14801_d_n10, assign19980_e14801_d_n11, assign19980_e14801_d_n14,) = {
    if (locals.var_guard404 != 0.0) {
        let assign19980_e14799: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19980_e14799, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19980_e14801;
        locals.var_vbs_bnd_local_dn0 = assign19980_e14801_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19980_e14801_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19980_e14801_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19980_e14801_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19980_e14801_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19980_e14801_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19980_e14801_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19980_e14801_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19980_e14801_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19980_e14801_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19980_e14801_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19990_e14808: f64 = if ((locals.var_rse > 0.0) || (locals.var_rde > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign19990_e14808;
        locals.var_guard405_rv = 0.0;

        let assign20000_e14811: f64 = if locals.var_uc_corsrd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign20000_e14811;
        locals.var_guard406_rv = 0.0;

        let (assign20010_e14817,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20010_e14817;
        locals.var_flg_rsrd_rv = 0.0;

        let assign20020_e14820: f64 = if locals.var_uc_corsrd == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign20020_e14820;
        locals.var_guard407_rv = 0.0;

        let (assign20030_e14826,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard407 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20030_e14826;
        locals.var_flg_rsrd_rv = 0.0;

        let assign20040_e14829: f64 = if locals.var_uc_corsrd == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign20040_e14829;
        locals.var_guard408_rv = 0.0;

        let (assign20050_e14835,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20050_e14835;
        locals.var_flg_rsrd_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        let assign20070_e14847: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20070_e14848: f64 = (locals.var_uc_nover * assign20070_e14847);
        let assign20070_e14851: f64 = if (((locals.var_uc_cordrift == 1.0) && (p.p54 == 1.0)) && (assign20070_e14848 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20070_e14851;
        locals.var_guard409_rv = 0.0;

        let (assign20080_e14855, assign20080_e14855_d_n0, assign20080_e14855_d_n2,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20080_e14855;
        locals.var_vdsegmt_dn0 = assign20080_e14855_d_n0;
        locals.var_vdsegmt_dn2 = assign20080_e14855_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let assign20090_e14858: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20090_e14858;
        locals.var_guard410_rv = 0.0;

        let (assign20100_e14864, assign20100_e14864_d_n0, assign20100_e14864_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20100_e14864;
        locals.var_vdserev_dn0 = assign20100_e14864_d_n0;
        locals.var_vdserev_dn2 = assign20100_e14864_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20110_e14870, assign20110_e14870_d_n0, assign20110_e14870_d_n2, assign20110_e14870_d_n4,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20110_e14870;
        locals.var_vsubsrev_dn0 = assign20110_e14870_d_n0;
        locals.var_vsubsrev_dn2 = assign20110_e14870_d_n2;
        locals.var_vsubsrev_dn4 = assign20110_e14870_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20120_e14878, assign20120_e14878_d_n0, assign20120_e14878_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20120_e14876: f64 = (-locals.var_vdsegmt);
        (assign20120_e14876, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20120_e14878;
        locals.var_vdserev_dn0 = assign20120_e14878_d_n0;
        locals.var_vdserev_dn2 = assign20120_e14878_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20130_e14887, assign20130_e14887_d_n0, assign20130_e14887_d_n2, assign20130_e14887_d_n4,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20130_e14885: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20130_e14885, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20130_e14887;
        locals.var_vsubsrev_dn0 = assign20130_e14887_d_n0;
        locals.var_vsubsrev_dn2 = assign20130_e14887_d_n2;
        locals.var_vsubsrev_dn4 = assign20130_e14887_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20140_e14897, assign20140_e14897_d_n0, assign20140_e14897_d_n2, assign20140_e14897_d_n4, assign20140_e14897_d_n5, assign20140_e14897_d_n6, assign20140_e14897_d_n7, assign20140_e14897_d_n8, assign20140_e14897_d_n9, assign20140_e14897_d_n10, assign20140_e14897_d_n11, assign20140_e14897_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20140_e14892: f64 = (locals.var_vdserev / 2.0);
        let assign20140_e14893: f64 = (2.0 * assign20140_e14892);
        let assign20140_e14895: f64 = (assign20140_e14893 / p.p262);
        (assign20140_e14895, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20140_e14897;
        locals.var_tmf1_dn0 = assign20140_e14897_d_n0;
        locals.var_tmf1_dn2 = assign20140_e14897_d_n2;
        locals.var_tmf1_dn4 = assign20140_e14897_d_n4;
        locals.var_tmf1_dn5 = assign20140_e14897_d_n5;
        locals.var_tmf1_dn6 = assign20140_e14897_d_n6;
        locals.var_tmf1_dn7 = assign20140_e14897_d_n7;
        locals.var_tmf1_dn8 = assign20140_e14897_d_n8;
        locals.var_tmf1_dn9 = assign20140_e14897_d_n9;
        locals.var_tmf1_dn10 = assign20140_e14897_d_n10;
        locals.var_tmf1_dn11 = assign20140_e14897_d_n11;
        locals.var_tmf1_dn14 = assign20140_e14897_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20150_e14937, assign20150_e14937_d_n0, assign20150_e14937_d_n2, assign20150_e14937_d_n4, assign20150_e14937_d_n5, assign20150_e14937_d_n6, assign20150_e14937_d_n7, assign20150_e14937_d_n8, assign20150_e14937_d_n9, assign20150_e14937_d_n10, assign20150_e14937_d_n11, assign20150_e14937_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20150_e14903: f64 = (1.0 / 2.0);
        let assign20150_e14907: f64 = (1.0 / 6.0);
        let assign20150_e14911: f64 = (1.0 / 24.0);
        let assign20150_e14915: f64 = (1.0 / 120.0);
        let assign20150_e14919: f64 = (1.0 / 720.0);
        let assign20150_e14923: f64 = (1.0 / 5040.0);
        let assign20150_e14924: f64 = (locals.var_tmf1 * assign20150_e14923);
        let assign20150_e14925: f64 = (assign20150_e14919 + assign20150_e14924);
        let assign20150_e14926: f64 = (locals.var_tmf1 * assign20150_e14925);
        let assign20150_e14927: f64 = (assign20150_e14915 + assign20150_e14926);
        let assign20150_e14928: f64 = (locals.var_tmf1 * assign20150_e14927);
        let assign20150_e14929: f64 = (assign20150_e14911 + assign20150_e14928);
        let assign20150_e14930: f64 = (locals.var_tmf1 * assign20150_e14929);
        let assign20150_e14931: f64 = (assign20150_e14907 + assign20150_e14930);
        let assign20150_e14932: f64 = (locals.var_tmf1 * assign20150_e14931);
        let assign20150_e14933: f64 = (assign20150_e14903 + assign20150_e14932);
        let assign20150_e14934: f64 = (locals.var_tmf1 * assign20150_e14933);
        let assign20150_e14935: f64 = (1.0 + assign20150_e14934);
        (assign20150_e14935, ((locals.var_tmf1_dn0 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn2 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn4 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn5 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn6 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn7 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn8 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn9 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn10 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn11 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20150_e14923))))))))))), ((locals.var_tmf1_dn14 * assign20150_e14933) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14931) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14929) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14927) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20150_e14925) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20150_e14923))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20150_e14937;
        locals.var_tmf2_dn0 = assign20150_e14937_d_n0;
        locals.var_tmf2_dn2 = assign20150_e14937_d_n2;
        locals.var_tmf2_dn4 = assign20150_e14937_d_n4;
        locals.var_tmf2_dn5 = assign20150_e14937_d_n5;
        locals.var_tmf2_dn6 = assign20150_e14937_d_n6;
        locals.var_tmf2_dn7 = assign20150_e14937_d_n7;
        locals.var_tmf2_dn8 = assign20150_e14937_d_n8;
        locals.var_tmf2_dn9 = assign20150_e14937_d_n9;
        locals.var_tmf2_dn10 = assign20150_e14937_d_n10;
        locals.var_tmf2_dn11 = assign20150_e14937_d_n11;
        locals.var_tmf2_dn14 = assign20150_e14937_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20160_e14973, assign20160_e14973_d_n0, assign20160_e14973_d_n2, assign20160_e14973_d_n4, assign20160_e14973_d_n5, assign20160_e14973_d_n6, assign20160_e14973_d_n7, assign20160_e14973_d_n8, assign20160_e14973_d_n9, assign20160_e14973_d_n10, assign20160_e14973_d_n11, assign20160_e14973_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20160_e14941: f64 = (1.0 / 2.0);
        let assign20160_e14945: f64 = (1.0 / 3.0);
        let assign20160_e14949: f64 = (1.0 / 8.0);
        let assign20160_e14953: f64 = (1.0 / 30.0);
        let assign20160_e14957: f64 = (1.0 / 144.0);
        let assign20160_e14961: f64 = (1.0 / 840.0);
        let assign20160_e14962: f64 = (locals.var_tmf1 * assign20160_e14961);
        let assign20160_e14963: f64 = (assign20160_e14957 + assign20160_e14962);
        let assign20160_e14964: f64 = (locals.var_tmf1 * assign20160_e14963);
        let assign20160_e14965: f64 = (assign20160_e14953 + assign20160_e14964);
        let assign20160_e14966: f64 = (locals.var_tmf1 * assign20160_e14965);
        let assign20160_e14967: f64 = (assign20160_e14949 + assign20160_e14966);
        let assign20160_e14968: f64 = (locals.var_tmf1 * assign20160_e14967);
        let assign20160_e14969: f64 = (assign20160_e14945 + assign20160_e14968);
        let assign20160_e14970: f64 = (locals.var_tmf1 * assign20160_e14969);
        let assign20160_e14971: f64 = (assign20160_e14941 + assign20160_e14970);
        (assign20160_e14971, ((locals.var_tmf1_dn0 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20160_e14961))))))))), ((locals.var_tmf1_dn2 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20160_e14961))))))))), ((locals.var_tmf1_dn4 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20160_e14961))))))))), ((locals.var_tmf1_dn5 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20160_e14961))))))))), ((locals.var_tmf1_dn6 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20160_e14961))))))))), ((locals.var_tmf1_dn7 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20160_e14961))))))))), ((locals.var_tmf1_dn8 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20160_e14961))))))))), ((locals.var_tmf1_dn9 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20160_e14961))))))))), ((locals.var_tmf1_dn10 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20160_e14961))))))))), ((locals.var_tmf1_dn11 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20160_e14961))))))))), ((locals.var_tmf1_dn14 * assign20160_e14969) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14967) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14965) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20160_e14963) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20160_e14961))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20160_e14973;
        locals.var_tmf3_dn0 = assign20160_e14973_d_n0;
        locals.var_tmf3_dn2 = assign20160_e14973_d_n2;
        locals.var_tmf3_dn4 = assign20160_e14973_d_n4;
        locals.var_tmf3_dn5 = assign20160_e14973_d_n5;
        locals.var_tmf3_dn6 = assign20160_e14973_d_n6;
        locals.var_tmf3_dn7 = assign20160_e14973_d_n7;
        locals.var_tmf3_dn8 = assign20160_e14973_d_n8;
        locals.var_tmf3_dn9 = assign20160_e14973_d_n9;
        locals.var_tmf3_dn10 = assign20160_e14973_d_n10;
        locals.var_tmf3_dn11 = assign20160_e14973_d_n11;
        locals.var_tmf3_dn14 = assign20160_e14973_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign20170_e14979, assign20170_e14979_d_n0, assign20170_e14979_d_n2, assign20170_e14979_d_n4, assign20170_e14979_d_n5, assign20170_e14979_d_n6, assign20170_e14979_d_n7, assign20170_e14979_d_n8, assign20170_e14979_d_n9, assign20170_e14979_d_n10, assign20170_e14979_d_n11, assign20170_e14979_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20170_e14977: f64 = (p.p262 / locals.var_tmf2);
        (assign20170_e14977, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20170_e14979;
        locals.var_vzadd_dn0 = assign20170_e14979_d_n0;
        locals.var_vzadd_dn2 = assign20170_e14979_d_n2;
        locals.var_vzadd_dn4 = assign20170_e14979_d_n4;
        locals.var_vzadd_dn5 = assign20170_e14979_d_n5;
        locals.var_vzadd_dn6 = assign20170_e14979_d_n6;
        locals.var_vzadd_dn7 = assign20170_e14979_d_n7;
        locals.var_vzadd_dn8 = assign20170_e14979_d_n8;
        locals.var_vzadd_dn9 = assign20170_e14979_d_n9;
        locals.var_vzadd_dn10 = assign20170_e14979_d_n10;
        locals.var_vzadd_dn11 = assign20170_e14979_d_n11;
        locals.var_vzadd_dn14 = assign20170_e14979_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20180_e14990, assign20180_e14990_d_n0, assign20180_e14990_d_n2, assign20180_e14990_d_n4, assign20180_e14990_d_n5, assign20180_e14990_d_n6, assign20180_e14990_d_n7, assign20180_e14990_d_n8, assign20180_e14990_d_n9, assign20180_e14990_d_n10, assign20180_e14990_d_n11, assign20180_e14990_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20180_e14982: f64 = (-2.0);
        let assign20180_e14984: f64 = (assign20180_e14982 * locals.var_tmf3);
        let assign20180_e14987: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20180_e14988: f64 = (assign20180_e14984 / assign20180_e14987);
        (assign20180_e14988, ((((assign20180_e14982 * locals.var_tmf3_dn0) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn2) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn4) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn5) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn6) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn7) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn8) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn9) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn10) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn11) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20180_e14987 * assign20180_e14987)), ((((assign20180_e14982 * locals.var_tmf3_dn14) * assign20180_e14987) - (assign20180_e14984 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20180_e14987 * assign20180_e14987)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20180_e14990;
        locals.var_t2_dn0 = assign20180_e14990_d_n0;
        locals.var_t2_dn2 = assign20180_e14990_d_n2;
        locals.var_t2_dn4 = assign20180_e14990_d_n4;
        locals.var_t2_dn5 = assign20180_e14990_d_n5;
        locals.var_t2_dn6 = assign20180_e14990_d_n6;
        locals.var_t2_dn7 = assign20180_e14990_d_n7;
        locals.var_t2_dn8 = assign20180_e14990_d_n8;
        locals.var_t2_dn9 = assign20180_e14990_d_n9;
        locals.var_t2_dn10 = assign20180_e14990_d_n10;
        locals.var_t2_dn11 = assign20180_e14990_d_n11;
        locals.var_t2_dn14 = assign20180_e14990_d_n14;
        locals.var_t2_rv = 0.0;

        let assign20190_e14993: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20190_e14993;
        locals.var_guard411_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20200_e14999, assign20200_e14999_d_n0, assign20200_e14999_d_n2, assign20200_e14999_d_n4, assign20200_e14999_d_n5, assign20200_e14999_d_n6, assign20200_e14999_d_n7, assign20200_e14999_d_n8, assign20200_e14999_d_n9, assign20200_e14999_d_n10, assign20200_e14999_d_n11, assign20200_e14999_d_n14,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20200_e14999;
        locals.var_vzadd_dn0 = assign20200_e14999_d_n0;
        locals.var_vzadd_dn2 = assign20200_e14999_d_n2;
        locals.var_vzadd_dn4 = assign20200_e14999_d_n4;
        locals.var_vzadd_dn5 = assign20200_e14999_d_n5;
        locals.var_vzadd_dn6 = assign20200_e14999_d_n6;
        locals.var_vzadd_dn7 = assign20200_e14999_d_n7;
        locals.var_vzadd_dn8 = assign20200_e14999_d_n8;
        locals.var_vzadd_dn9 = assign20200_e14999_d_n9;
        locals.var_vzadd_dn10 = assign20200_e14999_d_n10;
        locals.var_vzadd_dn11 = assign20200_e14999_d_n11;
        locals.var_vzadd_dn14 = assign20200_e14999_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20210_e15007, assign20210_e15007_d_n0, assign20210_e15007_d_n2, assign20210_e15007_d_n4, assign20210_e15007_d_n5, assign20210_e15007_d_n6, assign20210_e15007_d_n7, assign20210_e15007_d_n8, assign20210_e15007_d_n9, assign20210_e15007_d_n10, assign20210_e15007_d_n11, assign20210_e15007_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20210_e15004: f64 = (2.0 * locals.var_vzadd);
        let assign20210_e15005: f64 = (locals.var_vdserev + assign20210_e15004);
        (assign20210_e15005, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20210_e15007;
        locals.var_vdserevz_dn0 = assign20210_e15007_d_n0;
        locals.var_vdserevz_dn2 = assign20210_e15007_d_n2;
        locals.var_vdserevz_dn4 = assign20210_e15007_d_n4;
        locals.var_vdserevz_dn5 = assign20210_e15007_d_n5;
        locals.var_vdserevz_dn6 = assign20210_e15007_d_n6;
        locals.var_vdserevz_dn7 = assign20210_e15007_d_n7;
        locals.var_vdserevz_dn8 = assign20210_e15007_d_n8;
        locals.var_vdserevz_dn9 = assign20210_e15007_d_n9;
        locals.var_vdserevz_dn10 = assign20210_e15007_d_n10;
        locals.var_vdserevz_dn11 = assign20210_e15007_d_n11;
        locals.var_vdserevz_dn14 = assign20210_e15007_d_n14;
        locals.var_vdserevz_rv = 0.0;

        let (assign20220_e15019, assign20220_e15019_d_n0, assign20220_e15019_d_n2, assign20220_e15019_d_n4, assign20220_e15019_d_n5, assign20220_e15019_d_n6, assign20220_e15019_d_n7, assign20220_e15019_d_n8, assign20220_e15019_d_n9, assign20220_e15019_d_n10, assign20220_e15019_d_n11, assign20220_e15019_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20220_e15012: f64 = (p.p333 * locals.var_vdserevz);
        let assign20220_e15013: f64 = (p.p335 - assign20220_e15012);
        let assign20220_e15016: f64 = (p.p332 * locals.var_vsubsrev);
        let assign20220_e15017: f64 = (assign20220_e15013 - assign20220_e15016);
        (assign20220_e15017, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20220_e15019;
        locals.var_t0_dn0 = assign20220_e15019_d_n0;
        locals.var_t0_dn2 = assign20220_e15019_d_n2;
        locals.var_t0_dn4 = assign20220_e15019_d_n4;
        locals.var_t0_dn5 = assign20220_e15019_d_n5;
        locals.var_t0_dn6 = assign20220_e15019_d_n6;
        locals.var_t0_dn7 = assign20220_e15019_d_n7;
        locals.var_t0_dn8 = assign20220_e15019_d_n8;
        locals.var_t0_dn9 = assign20220_e15019_d_n9;
        locals.var_t0_dn10 = assign20220_e15019_d_n10;
        locals.var_t0_dn11 = assign20220_e15019_d_n11;
        locals.var_t0_dn14 = assign20220_e15019_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20230_e15032, assign20230_e15032_d_n0, assign20230_e15032_d_n2, assign20230_e15032_d_n4, assign20230_e15032_d_n5, assign20230_e15032_d_n6, assign20230_e15032_d_n7, assign20230_e15032_d_n8, assign20230_e15032_d_n9, assign20230_e15032_d_n10, assign20230_e15032_d_n11, assign20230_e15032_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20230_e15023: f64 = (locals.var_t0 * locals.var_t0);
        let assign20230_e15026: f64 = (4.0 * 10.0);
        let assign20230_e15028: f64 = (assign20230_e15026 * 10.0);
        let assign20230_e15029: f64 = (assign20230_e15023 + assign20230_e15028);
        let assign20230_e15030: f64 = (assign20230_e15029).sqrt();
        (assign20230_e15030, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign20230_e15030)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign20230_e15030)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20230_e15032;
        locals.var_tmf2_dn0 = assign20230_e15032_d_n0;
        locals.var_tmf2_dn2 = assign20230_e15032_d_n2;
        locals.var_tmf2_dn4 = assign20230_e15032_d_n4;
        locals.var_tmf2_dn5 = assign20230_e15032_d_n5;
        locals.var_tmf2_dn6 = assign20230_e15032_d_n6;
        locals.var_tmf2_dn7 = assign20230_e15032_d_n7;
        locals.var_tmf2_dn8 = assign20230_e15032_d_n8;
        locals.var_tmf2_dn9 = assign20230_e15032_d_n9;
        locals.var_tmf2_dn10 = assign20230_e15032_d_n10;
        locals.var_tmf2_dn11 = assign20230_e15032_d_n11;
        locals.var_tmf2_dn14 = assign20230_e15032_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20240_e15042, assign20240_e15042_d_n0, assign20240_e15042_d_n2, assign20240_e15042_d_n4, assign20240_e15042_d_n5, assign20240_e15042_d_n6, assign20240_e15042_d_n7, assign20240_e15042_d_n8, assign20240_e15042_d_n9, assign20240_e15042_d_n10, assign20240_e15042_d_n11, assign20240_e15042_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20240_e15038: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign20240_e15039: f64 = (1.0 + assign20240_e15038);
        let assign20240_e15040: f64 = (0.5 * assign20240_e15039);
        (assign20240_e15040, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20240_e15042;
        locals.var_t2_dn0 = assign20240_e15042_d_n0;
        locals.var_t2_dn2 = assign20240_e15042_d_n2;
        locals.var_t2_dn4 = assign20240_e15042_d_n4;
        locals.var_t2_dn5 = assign20240_e15042_d_n5;
        locals.var_t2_dn6 = assign20240_e15042_d_n6;
        locals.var_t2_dn7 = assign20240_e15042_d_n7;
        locals.var_t2_dn8 = assign20240_e15042_d_n8;
        locals.var_t2_dn9 = assign20240_e15042_d_n9;
        locals.var_t2_dn10 = assign20240_e15042_d_n10;
        locals.var_t2_dn11 = assign20240_e15042_d_n11;
        locals.var_t2_dn14 = assign20240_e15042_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20250_e15050, assign20250_e15050_d_n0, assign20250_e15050_d_n2, assign20250_e15050_d_n4, assign20250_e15050_d_n5, assign20250_e15050_d_n6, assign20250_e15050_d_n7, assign20250_e15050_d_n8, assign20250_e15050_d_n9, assign20250_e15050_d_n10, assign20250_e15050_d_n11, assign20250_e15050_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20250_e15047: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign20250_e15048: f64 = (0.5 * assign20250_e15047);
        (assign20250_e15048, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20250_e15050;
        locals.var_t1_dn0 = assign20250_e15050_d_n0;
        locals.var_t1_dn2 = assign20250_e15050_d_n2;
        locals.var_t1_dn4 = assign20250_e15050_d_n4;
        locals.var_t1_dn5 = assign20250_e15050_d_n5;
        locals.var_t1_dn6 = assign20250_e15050_d_n6;
        locals.var_t1_dn7 = assign20250_e15050_d_n7;
        locals.var_t1_dn8 = assign20250_e15050_d_n8;
        locals.var_t1_dn9 = assign20250_e15050_d_n9;
        locals.var_t1_dn10 = assign20250_e15050_d_n10;
        locals.var_t1_dn11 = assign20250_e15050_d_n11;
        locals.var_t1_dn14 = assign20250_e15050_d_n14;
        locals.var_t1_rv = 0.0;

        let assign20260_e15053: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign20260_e15053;
        locals.var_guard412_rv = 0.0;

        let (assign20270_e15059, assign20270_e15059_d_n0, assign20270_e15059_d_n2, assign20270_e15059_d_n4, assign20270_e15059_d_n5, assign20270_e15059_d_n6, assign20270_e15059_d_n7, assign20270_e15059_d_n8, assign20270_e15059_d_n9, assign20270_e15059_d_n10, assign20270_e15059_d_n11, assign20270_e15059_d_n14,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard412 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20270_e15059;
        locals.var_t1_dn0 = assign20270_e15059_d_n0;
        locals.var_t1_dn2 = assign20270_e15059_d_n2;
        locals.var_t1_dn4 = assign20270_e15059_d_n4;
        locals.var_t1_dn5 = assign20270_e15059_d_n5;
        locals.var_t1_dn6 = assign20270_e15059_d_n6;
        locals.var_t1_dn7 = assign20270_e15059_d_n7;
        locals.var_t1_dn8 = assign20270_e15059_d_n8;
        locals.var_t1_dn9 = assign20270_e15059_d_n9;
        locals.var_t1_dn10 = assign20270_e15059_d_n10;
        locals.var_t1_dn11 = assign20270_e15059_d_n11;
        locals.var_t1_dn14 = assign20270_e15059_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20280_e15065, assign20280_e15065_d_n0, assign20280_e15065_d_n2, assign20280_e15065_d_n4, assign20280_e15065_d_n5, assign20280_e15065_d_n6, assign20280_e15065_d_n7, assign20280_e15065_d_n8, assign20280_e15065_d_n9, assign20280_e15065_d_n10, assign20280_e15065_d_n11, assign20280_e15065_d_n14,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard412 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20280_e15065;
        locals.var_t2_dn0 = assign20280_e15065_d_n0;
        locals.var_t2_dn2 = assign20280_e15065_d_n2;
        locals.var_t2_dn4 = assign20280_e15065_d_n4;
        locals.var_t2_dn5 = assign20280_e15065_d_n5;
        locals.var_t2_dn6 = assign20280_e15065_d_n6;
        locals.var_t2_dn7 = assign20280_e15065_d_n7;
        locals.var_t2_dn8 = assign20280_e15065_d_n8;
        locals.var_t2_dn9 = assign20280_e15065_d_n9;
        locals.var_t2_dn10 = assign20280_e15065_d_n10;
        locals.var_t2_dn11 = assign20280_e15065_d_n11;
        locals.var_t2_dn14 = assign20280_e15065_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20290_e15073, assign20290_e15073_d_n0, assign20290_e15073_d_n2, assign20290_e15073_d_n4, assign20290_e15073_d_n5, assign20290_e15073_d_n6, assign20290_e15073_d_n7, assign20290_e15073_d_n8, assign20290_e15073_d_n9, assign20290_e15073_d_n10, assign20290_e15073_d_n11, assign20290_e15073_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20290_e15070: f64 = (10.0 * 2.220446049250313e-16);
        let assign20290_e15071: f64 = (locals.var_t1 + assign20290_e15070);
        (assign20290_e15071, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20290_e15073;
        locals.var_t1_dn0 = assign20290_e15073_d_n0;
        locals.var_t1_dn2 = assign20290_e15073_d_n2;
        locals.var_t1_dn4 = assign20290_e15073_d_n4;
        locals.var_t1_dn5 = assign20290_e15073_d_n5;
        locals.var_t1_dn6 = assign20290_e15073_d_n6;
        locals.var_t1_dn7 = assign20290_e15073_d_n7;
        locals.var_t1_dn8 = assign20290_e15073_d_n8;
        locals.var_t1_dn9 = assign20290_e15073_d_n9;
        locals.var_t1_dn10 = assign20290_e15073_d_n10;
        locals.var_t1_dn11 = assign20290_e15073_d_n11;
        locals.var_t1_dn14 = assign20290_e15073_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20300_e15083, assign20300_e15083_d_n0, assign20300_e15083_d_n2, assign20300_e15083_d_n4, assign20300_e15083_d_n5, assign20300_e15083_d_n6, assign20300_e15083_d_n7, assign20300_e15083_d_n8, assign20300_e15083_d_n9, assign20300_e15083_d_n10, assign20300_e15083_d_n11, assign20300_e15083_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20300_e15079: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20300_e15080: f64 = (locals.var_uc_nover * assign20300_e15079);
        let assign20300_e15081: f64 = (locals.var_mks_nsubsub / assign20300_e15080);
        (assign20300_e15081, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20300_e15083;
        locals.var_t0_dn0 = assign20300_e15083_d_n0;
        locals.var_t0_dn2 = assign20300_e15083_d_n2;
        locals.var_t0_dn4 = assign20300_e15083_d_n4;
        locals.var_t0_dn5 = assign20300_e15083_d_n5;
        locals.var_t0_dn6 = assign20300_e15083_d_n6;
        locals.var_t0_dn7 = assign20300_e15083_d_n7;
        locals.var_t0_dn8 = assign20300_e15083_d_n8;
        locals.var_t0_dn9 = assign20300_e15083_d_n9;
        locals.var_t0_dn10 = assign20300_e15083_d_n10;
        locals.var_t0_dn11 = assign20300_e15083_d_n11;
        locals.var_t0_dn14 = assign20300_e15083_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20310_e15093, assign20310_e15093_d_n0, assign20310_e15093_d_n2, assign20310_e15093_d_n4, assign20310_e15093_d_n5, assign20310_e15093_d_n6, assign20310_e15093_d_n7, assign20310_e15093_d_n8, assign20310_e15093_d_n9, assign20310_e15093_d_n10, assign20310_e15093_d_n11, assign20310_e15093_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20310_e15087: f64 = (2.0 * 1.034943e-10);
        let assign20310_e15089: f64 = (assign20310_e15087 / 1.6021918e-19);
        let assign20310_e15091: f64 = (assign20310_e15089 * locals.var_t0);
        (assign20310_e15091, (assign20310_e15089 * locals.var_t0_dn0), (assign20310_e15089 * locals.var_t0_dn2), (assign20310_e15089 * locals.var_t0_dn4), (assign20310_e15089 * locals.var_t0_dn5), (assign20310_e15089 * locals.var_t0_dn6), (assign20310_e15089 * locals.var_t0_dn7), (assign20310_e15089 * locals.var_t0_dn8), (assign20310_e15089 * locals.var_t0_dn9), (assign20310_e15089 * locals.var_t0_dn10), (assign20310_e15089 * locals.var_t0_dn11), (assign20310_e15089 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20310_e15093;
        locals.var_t4_dn0 = assign20310_e15093_d_n0;
        locals.var_t4_dn2 = assign20310_e15093_d_n2;
        locals.var_t4_dn4 = assign20310_e15093_d_n4;
        locals.var_t4_dn5 = assign20310_e15093_d_n5;
        locals.var_t4_dn6 = assign20310_e15093_d_n6;
        locals.var_t4_dn7 = assign20310_e15093_d_n7;
        locals.var_t4_dn8 = assign20310_e15093_d_n8;
        locals.var_t4_dn9 = assign20310_e15093_d_n9;
        locals.var_t4_dn10 = assign20310_e15093_d_n10;
        locals.var_t4_dn11 = assign20310_e15093_d_n11;
        locals.var_t4_dn14 = assign20310_e15093_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20320_e15102, assign20320_e15102_d_n0, assign20320_e15102_d_n2, assign20320_e15102_d_n4, assign20320_e15102_d_n5, assign20320_e15102_d_n6, assign20320_e15102_d_n7, assign20320_e15102_d_n8, assign20320_e15102_d_n9, assign20320_e15102_d_n10, assign20320_e15102_d_n11, assign20320_e15102_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20320_e15097: f64 = (locals.var_t4 * locals.var_t1);
        let assign20320_e15098: f64 = (assign20320_e15097).sqrt();
        let assign20320_e15100: f64 = (assign20320_e15098 + 1e-25);
        (assign20320_e15100, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign20320_e15098)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20320_e15102;
        locals.var_wdep_dn0 = assign20320_e15102_d_n0;
        locals.var_wdep_dn2 = assign20320_e15102_d_n2;
        locals.var_wdep_dn4 = assign20320_e15102_d_n4;
        locals.var_wdep_dn5 = assign20320_e15102_d_n5;
        locals.var_wdep_dn6 = assign20320_e15102_d_n6;
        locals.var_wdep_dn7 = assign20320_e15102_d_n7;
        locals.var_wdep_dn8 = assign20320_e15102_d_n8;
        locals.var_wdep_dn9 = assign20320_e15102_d_n9;
        locals.var_wdep_dn10 = assign20320_e15102_d_n10;
        locals.var_wdep_dn11 = assign20320_e15102_d_n11;
        locals.var_wdep_dn14 = assign20320_e15102_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign20330_e15112, assign20330_e15112_d_n0, assign20330_e15112_d_n2, assign20330_e15112_d_n4, assign20330_e15112_d_n5, assign20330_e15112_d_n6, assign20330_e15112_d_n7, assign20330_e15112_d_n8, assign20330_e15112_d_n9, assign20330_e15112_d_n10, assign20330_e15112_d_n11, assign20330_e15112_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20330_e15106: f64 = (p.p334 - locals.var_wdep);
        let assign20330_e15109: f64 = (0.1 * p.p334);
        let assign20330_e15110: f64 = (assign20330_e15106 - assign20330_e15109);
        (assign20330_e15110, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20330_e15112;
        locals.var_tmf1_dn0 = assign20330_e15112_d_n0;
        locals.var_tmf1_dn2 = assign20330_e15112_d_n2;
        locals.var_tmf1_dn4 = assign20330_e15112_d_n4;
        locals.var_tmf1_dn5 = assign20330_e15112_d_n5;
        locals.var_tmf1_dn6 = assign20330_e15112_d_n6;
        locals.var_tmf1_dn7 = assign20330_e15112_d_n7;
        locals.var_tmf1_dn8 = assign20330_e15112_d_n8;
        locals.var_tmf1_dn9 = assign20330_e15112_d_n9;
        locals.var_tmf1_dn10 = assign20330_e15112_d_n10;
        locals.var_tmf1_dn11 = assign20330_e15112_d_n11;
        locals.var_tmf1_dn14 = assign20330_e15112_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20340_e15122, assign20340_e15122_d_n0, assign20340_e15122_d_n2, assign20340_e15122_d_n4, assign20340_e15122_d_n5, assign20340_e15122_d_n6, assign20340_e15122_d_n7, assign20340_e15122_d_n8, assign20340_e15122_d_n9, assign20340_e15122_d_n10, assign20340_e15122_d_n11, assign20340_e15122_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20340_e15116: f64 = (4.0 * p.p334);
        let assign20340_e15119: f64 = (0.1 * p.p334);
        let assign20340_e15120: f64 = (assign20340_e15116 * assign20340_e15119);
        (assign20340_e15120, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20340_e15122;
        locals.var_tmf2_dn0 = assign20340_e15122_d_n0;
        locals.var_tmf2_dn2 = assign20340_e15122_d_n2;
        locals.var_tmf2_dn4 = assign20340_e15122_d_n4;
        locals.var_tmf2_dn5 = assign20340_e15122_d_n5;
        locals.var_tmf2_dn6 = assign20340_e15122_d_n6;
        locals.var_tmf2_dn7 = assign20340_e15122_d_n7;
        locals.var_tmf2_dn8 = assign20340_e15122_d_n8;
        locals.var_tmf2_dn9 = assign20340_e15122_d_n9;
        locals.var_tmf2_dn10 = assign20340_e15122_d_n10;
        locals.var_tmf2_dn11 = assign20340_e15122_d_n11;
        locals.var_tmf2_dn14 = assign20340_e15122_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20350_e15132, assign20350_e15132_d_n0, assign20350_e15132_d_n2, assign20350_e15132_d_n4, assign20350_e15132_d_n5, assign20350_e15132_d_n6, assign20350_e15132_d_n7, assign20350_e15132_d_n8, assign20350_e15132_d_n9, assign20350_e15132_d_n10, assign20350_e15132_d_n11, assign20350_e15132_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let (assign20350_e15130, assign20350_e15130_d_n0, assign20350_e15130_d_n2, assign20350_e15130_d_n4, assign20350_e15130_d_n5, assign20350_e15130_d_n6, assign20350_e15130_d_n7, assign20350_e15130_d_n8, assign20350_e15130_d_n9, assign20350_e15130_d_n10, assign20350_e15130_d_n11, assign20350_e15130_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20350_e15129: f64 = (-locals.var_tmf2);
                (assign20350_e15129, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20350_e15130, assign20350_e15130_d_n0, assign20350_e15130_d_n2, assign20350_e15130_d_n4, assign20350_e15130_d_n5, assign20350_e15130_d_n6, assign20350_e15130_d_n7, assign20350_e15130_d_n8, assign20350_e15130_d_n9, assign20350_e15130_d_n10, assign20350_e15130_d_n11, assign20350_e15130_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20350_e15132;
        locals.var_tmf2_dn0 = assign20350_e15132_d_n0;
        locals.var_tmf2_dn2 = assign20350_e15132_d_n2;
        locals.var_tmf2_dn4 = assign20350_e15132_d_n4;
        locals.var_tmf2_dn5 = assign20350_e15132_d_n5;
        locals.var_tmf2_dn6 = assign20350_e15132_d_n6;
        locals.var_tmf2_dn7 = assign20350_e15132_d_n7;
        locals.var_tmf2_dn8 = assign20350_e15132_d_n8;
        locals.var_tmf2_dn9 = assign20350_e15132_d_n9;
        locals.var_tmf2_dn10 = assign20350_e15132_d_n10;
        locals.var_tmf2_dn11 = assign20350_e15132_d_n11;
        locals.var_tmf2_dn14 = assign20350_e15132_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20360_e15141, assign20360_e15141_d_n0, assign20360_e15141_d_n2, assign20360_e15141_d_n4, assign20360_e15141_d_n5, assign20360_e15141_d_n6, assign20360_e15141_d_n7, assign20360_e15141_d_n8, assign20360_e15141_d_n9, assign20360_e15141_d_n10, assign20360_e15141_d_n11, assign20360_e15141_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20360_e15136: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20360_e15138: f64 = (assign20360_e15136 + locals.var_tmf2);
        let assign20360_e15139: f64 = (assign20360_e15138).sqrt();
        (assign20360_e15139, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20360_e15139)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20360_e15141;
        locals.var_tmf2_dn0 = assign20360_e15141_d_n0;
        locals.var_tmf2_dn2 = assign20360_e15141_d_n2;
        locals.var_tmf2_dn4 = assign20360_e15141_d_n4;
        locals.var_tmf2_dn5 = assign20360_e15141_d_n5;
        locals.var_tmf2_dn6 = assign20360_e15141_d_n6;
        locals.var_tmf2_dn7 = assign20360_e15141_d_n7;
        locals.var_tmf2_dn8 = assign20360_e15141_d_n8;
        locals.var_tmf2_dn9 = assign20360_e15141_d_n9;
        locals.var_tmf2_dn10 = assign20360_e15141_d_n10;
        locals.var_tmf2_dn11 = assign20360_e15141_d_n11;
        locals.var_tmf2_dn14 = assign20360_e15141_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20370_e15151, assign20370_e15151_d_n0, assign20370_e15151_d_n2, assign20370_e15151_d_n4, assign20370_e15151_d_n5, assign20370_e15151_d_n6, assign20370_e15151_d_n7, assign20370_e15151_d_n8, assign20370_e15151_d_n9, assign20370_e15151_d_n10, assign20370_e15151_d_n11, assign20370_e15151_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20370_e15147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20370_e15148: f64 = (1.0 + assign20370_e15147);
        let assign20370_e15149: f64 = (0.5 * assign20370_e15148);
        (assign20370_e15149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20370_e15151;
        locals.var_t0_dn0 = assign20370_e15151_d_n0;
        locals.var_t0_dn2 = assign20370_e15151_d_n2;
        locals.var_t0_dn4 = assign20370_e15151_d_n4;
        locals.var_t0_dn5 = assign20370_e15151_d_n5;
        locals.var_t0_dn6 = assign20370_e15151_d_n6;
        locals.var_t0_dn7 = assign20370_e15151_d_n7;
        locals.var_t0_dn8 = assign20370_e15151_d_n8;
        locals.var_t0_dn9 = assign20370_e15151_d_n9;
        locals.var_t0_dn10 = assign20370_e15151_d_n10;
        locals.var_t0_dn11 = assign20370_e15151_d_n11;
        locals.var_t0_dn14 = assign20370_e15151_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20380_e15161, assign20380_e15161_d_n0, assign20380_e15161_d_n2, assign20380_e15161_d_n4, assign20380_e15161_d_n5, assign20380_e15161_d_n6, assign20380_e15161_d_n7, assign20380_e15161_d_n8, assign20380_e15161_d_n9, assign20380_e15161_d_n10, assign20380_e15161_d_n11, assign20380_e15161_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20380_e15157: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20380_e15158: f64 = (0.5 * assign20380_e15157);
        let assign20380_e15159: f64 = (p.p334 - assign20380_e15158);
        (assign20380_e15159, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20380_e15161;
        locals.var_wdep_dn0 = assign20380_e15161_d_n0;
        locals.var_wdep_dn2 = assign20380_e15161_d_n2;
        locals.var_wdep_dn4 = assign20380_e15161_d_n4;
        locals.var_wdep_dn5 = assign20380_e15161_d_n5;
        locals.var_wdep_dn6 = assign20380_e15161_d_n6;
        locals.var_wdep_dn7 = assign20380_e15161_d_n7;
        locals.var_wdep_dn8 = assign20380_e15161_d_n8;
        locals.var_wdep_dn9 = assign20380_e15161_d_n9;
        locals.var_wdep_dn10 = assign20380_e15161_d_n10;
        locals.var_wdep_dn11 = assign20380_e15161_d_n11;
        locals.var_wdep_dn14 = assign20380_e15161_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign20390_e15166, assign20390_e15166_d_n0, assign20390_e15166_d_n2, assign20390_e15166_d_n4, assign20390_e15166_d_n5, assign20390_e15166_d_n6, assign20390_e15166_d_n7, assign20390_e15166_d_n8, assign20390_e15166_d_n9, assign20390_e15166_d_n10, assign20390_e15166_d_n11, assign20390_e15166_d_n14,) = {
    if (locals.var_guard409 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20390_e15166;
        locals.var_wdep_dn0 = assign20390_e15166_d_n0;
        locals.var_wdep_dn2 = assign20390_e15166_d_n2;
        locals.var_wdep_dn4 = assign20390_e15166_d_n4;
        locals.var_wdep_dn5 = assign20390_e15166_d_n5;
        locals.var_wdep_dn6 = assign20390_e15166_d_n6;
        locals.var_wdep_dn7 = assign20390_e15166_d_n7;
        locals.var_wdep_dn8 = assign20390_e15166_d_n8;
        locals.var_wdep_dn9 = assign20390_e15166_d_n9;
        locals.var_wdep_dn10 = assign20390_e15166_d_n10;
        locals.var_wdep_dn11 = assign20390_e15166_d_n11;
        locals.var_wdep_dn14 = assign20390_e15166_d_n14;
        locals.var_wdep_rv = 0.0;

        let assign20400_e15173: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20400_e15173;
        locals.var_guard413_rv = 0.0;

        let (assign20410_e15177, assign20410_e15177_d_n0, assign20410_e15177_d_n2,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20410_e15177;
        locals.var_vdsegmt_dn0 = assign20410_e15177_d_n0;
        locals.var_vdsegmt_dn2 = assign20410_e15177_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let (assign20420_e15181, assign20420_e15181_d_n2, assign20420_e15181_d_n7,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vgsei, locals.var_vgsei_dn2, locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgsegmt, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    }
};
        locals.var_vgsegmt = assign20420_e15181;
        locals.var_vgsegmt_dn2 = assign20420_e15181_d_n2;
        locals.var_vgsegmt_dn7 = assign20420_e15181_d_n7;
        locals.var_vgsegmt_rv = 0.0;

        let (assign20430_e15185, assign20430_e15185_d_n2, assign20430_e15185_d_n9,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vbsei, locals.var_vbsei_dn2, locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbsegmt, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    }
};
        locals.var_vbsegmt = assign20430_e15185;
        locals.var_vbsegmt_dn2 = assign20430_e15185_d_n2;
        locals.var_vbsegmt_dn9 = assign20430_e15185_d_n9;
        locals.var_vbsegmt_rv = 0.0;

        let assign20440_e15188: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20440_e15188;
        locals.var_guard414_rv = 0.0;

        let (assign20450_e15194,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20450_e15194;
        locals.var_vdsemodenml_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20460_e15200,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20460_e15200;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20470_e15206, assign20470_e15206_d_n0, assign20470_e15206_d_n2,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20470_e15206;
        locals.var_vdserev_dn0 = assign20470_e15206_d_n0;
        locals.var_vdserev_dn2 = assign20470_e15206_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20480_e15212, assign20480_e15212_d_n0, assign20480_e15212_d_n2, assign20480_e15212_d_n7,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vgsegmt, 0.0, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20480_e15212;
        locals.var_vgserev_dn0 = assign20480_e15212_d_n0;
        locals.var_vgserev_dn2 = assign20480_e15212_d_n2;
        locals.var_vgserev_dn7 = assign20480_e15212_d_n7;
        locals.var_vgserev_rv = 0.0;

        let (assign20490_e15218, assign20490_e15218_d_n0, assign20490_e15218_d_n2, assign20490_e15218_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vbsegmt, 0.0, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20490_e15218;
        locals.var_vbserev_dn0 = assign20490_e15218_d_n0;
        locals.var_vbserev_dn2 = assign20490_e15218_d_n2;
        locals.var_vbserev_dn9 = assign20490_e15218_d_n9;
        locals.var_vbserev_rv = 0.0;

        let (assign20500_e15224, assign20500_e15224_d_n0, assign20500_e15224_d_n2, assign20500_e15224_d_n4,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20500_e15224;
        locals.var_vsubsrev_dn0 = assign20500_e15224_d_n0;
        locals.var_vsubsrev_dn2 = assign20500_e15224_d_n2;
        locals.var_vsubsrev_dn4 = assign20500_e15224_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20510_e15231,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20510_e15231;
        locals.var_vdsemodenml_rv = 0.0;

        let (assign20520_e15238,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20520_e15238;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20530_e15246, assign20530_e15246_d_n0, assign20530_e15246_d_n2,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20530_e15244: f64 = (-locals.var_vdsegmt);
        (assign20530_e15244, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20530_e15246;
        locals.var_vdserev_dn0 = assign20530_e15246_d_n0;
        locals.var_vdserev_dn2 = assign20530_e15246_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20540_e15255, assign20540_e15255_d_n0, assign20540_e15255_d_n2, assign20540_e15255_d_n7,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20540_e15253: f64 = (locals.var_vgsegmt - locals.var_vdsegmt);
        (assign20540_e15253, (-locals.var_vdsegmt_dn0), (locals.var_vgsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20540_e15255;
        locals.var_vgserev_dn0 = assign20540_e15255_d_n0;
        locals.var_vgserev_dn2 = assign20540_e15255_d_n2;
        locals.var_vgserev_dn7 = assign20540_e15255_d_n7;
        locals.var_vgserev_rv = 0.0;

        let (assign20550_e15264, assign20550_e15264_d_n0, assign20550_e15264_d_n2, assign20550_e15264_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20550_e15262: f64 = (locals.var_vbsegmt - locals.var_vdsegmt);
        (assign20550_e15262, (-locals.var_vdsegmt_dn0), (locals.var_vbsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20550_e15264;
        locals.var_vbserev_dn0 = assign20550_e15264_d_n0;
        locals.var_vbserev_dn2 = assign20550_e15264_d_n2;
        locals.var_vbserev_dn9 = assign20550_e15264_d_n9;
        locals.var_vbserev_rv = 0.0;

        let (assign20560_e15273, assign20560_e15273_d_n0, assign20560_e15273_d_n2, assign20560_e15273_d_n4,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20560_e15271: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20560_e15271, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20560_e15273;
        locals.var_vsubsrev_dn0 = assign20560_e15273_d_n0;
        locals.var_vsubsrev_dn2 = assign20560_e15273_d_n2;
        locals.var_vsubsrev_dn4 = assign20560_e15273_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let assign20570_e15292: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign20570_e15292;
        locals.var_guard415_rv = 0.0;

        let (assign20580_e15304, assign20580_e15304_d_n0, assign20580_e15304_d_n2, assign20580_e15304_d_n4, assign20580_e15304_d_n5, assign20580_e15304_d_n6, assign20580_e15304_d_n7, assign20580_e15304_d_n8, assign20580_e15304_d_n9, assign20580_e15304_d_n10, assign20580_e15304_d_n11, assign20580_e15304_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20580_e15299: f64 = (locals.var_vdserev / 2.0);
        let assign20580_e15300: f64 = (2.0 * assign20580_e15299);
        let assign20580_e15302: f64 = (assign20580_e15300 / p.p262);
        (assign20580_e15302, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20580_e15304;
        locals.var_tmf1_dn0 = assign20580_e15304_d_n0;
        locals.var_tmf1_dn2 = assign20580_e15304_d_n2;
        locals.var_tmf1_dn4 = assign20580_e15304_d_n4;
        locals.var_tmf1_dn5 = assign20580_e15304_d_n5;
        locals.var_tmf1_dn6 = assign20580_e15304_d_n6;
        locals.var_tmf1_dn7 = assign20580_e15304_d_n7;
        locals.var_tmf1_dn8 = assign20580_e15304_d_n8;
        locals.var_tmf1_dn9 = assign20580_e15304_d_n9;
        locals.var_tmf1_dn10 = assign20580_e15304_d_n10;
        locals.var_tmf1_dn11 = assign20580_e15304_d_n11;
        locals.var_tmf1_dn14 = assign20580_e15304_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20590_e15346, assign20590_e15346_d_n0, assign20590_e15346_d_n2, assign20590_e15346_d_n4, assign20590_e15346_d_n5, assign20590_e15346_d_n6, assign20590_e15346_d_n7, assign20590_e15346_d_n8, assign20590_e15346_d_n9, assign20590_e15346_d_n10, assign20590_e15346_d_n11, assign20590_e15346_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20590_e15312: f64 = (1.0 / 2.0);
        let assign20590_e15316: f64 = (1.0 / 6.0);
        let assign20590_e15320: f64 = (1.0 / 24.0);
        let assign20590_e15324: f64 = (1.0 / 120.0);
        let assign20590_e15328: f64 = (1.0 / 720.0);
        let assign20590_e15332: f64 = (1.0 / 5040.0);
        let assign20590_e15333: f64 = (locals.var_tmf1 * assign20590_e15332);
        let assign20590_e15334: f64 = (assign20590_e15328 + assign20590_e15333);
        let assign20590_e15335: f64 = (locals.var_tmf1 * assign20590_e15334);
        let assign20590_e15336: f64 = (assign20590_e15324 + assign20590_e15335);
        let assign20590_e15337: f64 = (locals.var_tmf1 * assign20590_e15336);
        let assign20590_e15338: f64 = (assign20590_e15320 + assign20590_e15337);
        let assign20590_e15339: f64 = (locals.var_tmf1 * assign20590_e15338);
        let assign20590_e15340: f64 = (assign20590_e15316 + assign20590_e15339);
        let assign20590_e15341: f64 = (locals.var_tmf1 * assign20590_e15340);
        let assign20590_e15342: f64 = (assign20590_e15312 + assign20590_e15341);
        let assign20590_e15343: f64 = (locals.var_tmf1 * assign20590_e15342);
        let assign20590_e15344: f64 = (1.0 + assign20590_e15343);
        (assign20590_e15344, ((locals.var_tmf1_dn0 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn2 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn4 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn5 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn6 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn7 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn8 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn9 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn10 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn11 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn14 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20590_e15332))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20590_e15346;
        locals.var_tmf2_dn0 = assign20590_e15346_d_n0;
        locals.var_tmf2_dn2 = assign20590_e15346_d_n2;
        locals.var_tmf2_dn4 = assign20590_e15346_d_n4;
        locals.var_tmf2_dn5 = assign20590_e15346_d_n5;
        locals.var_tmf2_dn6 = assign20590_e15346_d_n6;
        locals.var_tmf2_dn7 = assign20590_e15346_d_n7;
        locals.var_tmf2_dn8 = assign20590_e15346_d_n8;
        locals.var_tmf2_dn9 = assign20590_e15346_d_n9;
        locals.var_tmf2_dn10 = assign20590_e15346_d_n10;
        locals.var_tmf2_dn11 = assign20590_e15346_d_n11;
        locals.var_tmf2_dn14 = assign20590_e15346_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20600_e15384, assign20600_e15384_d_n0, assign20600_e15384_d_n2, assign20600_e15384_d_n4, assign20600_e15384_d_n5, assign20600_e15384_d_n6, assign20600_e15384_d_n7, assign20600_e15384_d_n8, assign20600_e15384_d_n9, assign20600_e15384_d_n10, assign20600_e15384_d_n11, assign20600_e15384_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20600_e15352: f64 = (1.0 / 2.0);
        let assign20600_e15356: f64 = (1.0 / 3.0);
        let assign20600_e15360: f64 = (1.0 / 8.0);
        let assign20600_e15364: f64 = (1.0 / 30.0);
        let assign20600_e15368: f64 = (1.0 / 144.0);
        let assign20600_e15372: f64 = (1.0 / 840.0);
        let assign20600_e15373: f64 = (locals.var_tmf1 * assign20600_e15372);
        let assign20600_e15374: f64 = (assign20600_e15368 + assign20600_e15373);
        let assign20600_e15375: f64 = (locals.var_tmf1 * assign20600_e15374);
        let assign20600_e15376: f64 = (assign20600_e15364 + assign20600_e15375);
        let assign20600_e15377: f64 = (locals.var_tmf1 * assign20600_e15376);
        let assign20600_e15378: f64 = (assign20600_e15360 + assign20600_e15377);
        let assign20600_e15379: f64 = (locals.var_tmf1 * assign20600_e15378);
        let assign20600_e15380: f64 = (assign20600_e15356 + assign20600_e15379);
        let assign20600_e15381: f64 = (locals.var_tmf1 * assign20600_e15380);
        let assign20600_e15382: f64 = (assign20600_e15352 + assign20600_e15381);
        (assign20600_e15382, ((locals.var_tmf1_dn0 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20600_e15372))))))))), ((locals.var_tmf1_dn2 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20600_e15372))))))))), ((locals.var_tmf1_dn4 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20600_e15372))))))))), ((locals.var_tmf1_dn5 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20600_e15372))))))))), ((locals.var_tmf1_dn6 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20600_e15372))))))))), ((locals.var_tmf1_dn7 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20600_e15372))))))))), ((locals.var_tmf1_dn8 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20600_e15372))))))))), ((locals.var_tmf1_dn9 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20600_e15372))))))))), ((locals.var_tmf1_dn10 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20600_e15372))))))))), ((locals.var_tmf1_dn11 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20600_e15372))))))))), ((locals.var_tmf1_dn14 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20600_e15372))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20600_e15384;
        locals.var_tmf3_dn0 = assign20600_e15384_d_n0;
        locals.var_tmf3_dn2 = assign20600_e15384_d_n2;
        locals.var_tmf3_dn4 = assign20600_e15384_d_n4;
        locals.var_tmf3_dn5 = assign20600_e15384_d_n5;
        locals.var_tmf3_dn6 = assign20600_e15384_d_n6;
        locals.var_tmf3_dn7 = assign20600_e15384_d_n7;
        locals.var_tmf3_dn8 = assign20600_e15384_d_n8;
        locals.var_tmf3_dn9 = assign20600_e15384_d_n9;
        locals.var_tmf3_dn10 = assign20600_e15384_d_n10;
        locals.var_tmf3_dn11 = assign20600_e15384_d_n11;
        locals.var_tmf3_dn14 = assign20600_e15384_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign20610_e15392, assign20610_e15392_d_n0, assign20610_e15392_d_n2, assign20610_e15392_d_n4, assign20610_e15392_d_n5, assign20610_e15392_d_n6, assign20610_e15392_d_n7, assign20610_e15392_d_n8, assign20610_e15392_d_n9, assign20610_e15392_d_n10, assign20610_e15392_d_n11, assign20610_e15392_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20610_e15390: f64 = (p.p262 / locals.var_tmf2);
        (assign20610_e15390, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20610_e15392;
        locals.var_vzadd_dn0 = assign20610_e15392_d_n0;
        locals.var_vzadd_dn2 = assign20610_e15392_d_n2;
        locals.var_vzadd_dn4 = assign20610_e15392_d_n4;
        locals.var_vzadd_dn5 = assign20610_e15392_d_n5;
        locals.var_vzadd_dn6 = assign20610_e15392_d_n6;
        locals.var_vzadd_dn7 = assign20610_e15392_d_n7;
        locals.var_vzadd_dn8 = assign20610_e15392_d_n8;
        locals.var_vzadd_dn9 = assign20610_e15392_d_n9;
        locals.var_vzadd_dn10 = assign20610_e15392_d_n10;
        locals.var_vzadd_dn11 = assign20610_e15392_d_n11;
        locals.var_vzadd_dn14 = assign20610_e15392_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20620_e15405, assign20620_e15405_d_n0, assign20620_e15405_d_n2, assign20620_e15405_d_n4, assign20620_e15405_d_n5, assign20620_e15405_d_n6, assign20620_e15405_d_n7, assign20620_e15405_d_n8, assign20620_e15405_d_n9, assign20620_e15405_d_n10, assign20620_e15405_d_n11, assign20620_e15405_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20620_e15397: f64 = (-2.0);
        let assign20620_e15399: f64 = (assign20620_e15397 * locals.var_tmf3);
        let assign20620_e15402: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20620_e15403: f64 = (assign20620_e15399 / assign20620_e15402);
        (assign20620_e15403, ((((assign20620_e15397 * locals.var_tmf3_dn0) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn2) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn4) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn5) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn6) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn7) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn8) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn9) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn10) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn11) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn14) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20620_e15402 * assign20620_e15402)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20620_e15405;
        locals.var_t2_dn0 = assign20620_e15405_d_n0;
        locals.var_t2_dn2 = assign20620_e15405_d_n2;
        locals.var_t2_dn4 = assign20620_e15405_d_n4;
        locals.var_t2_dn5 = assign20620_e15405_d_n5;
        locals.var_t2_dn6 = assign20620_e15405_d_n6;
        locals.var_t2_dn7 = assign20620_e15405_d_n7;
        locals.var_t2_dn8 = assign20620_e15405_d_n8;
        locals.var_t2_dn9 = assign20620_e15405_d_n9;
        locals.var_t2_dn10 = assign20620_e15405_d_n10;
        locals.var_t2_dn11 = assign20620_e15405_d_n11;
        locals.var_t2_dn14 = assign20620_e15405_d_n14;
        locals.var_t2_rv = 0.0;

        let assign20630_e15408: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign20630_e15408;
        locals.var_guard416_rv = 0.0;

        let (assign20640_e15416, assign20640_e15416_d_n0, assign20640_e15416_d_n2, assign20640_e15416_d_n4, assign20640_e15416_d_n5, assign20640_e15416_d_n6, assign20640_e15416_d_n7, assign20640_e15416_d_n8, assign20640_e15416_d_n9, assign20640_e15416_d_n10, assign20640_e15416_d_n11, assign20640_e15416_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20640_e15416;
        locals.var_vzadd_dn0 = assign20640_e15416_d_n0;
        locals.var_vzadd_dn2 = assign20640_e15416_d_n2;
        locals.var_vzadd_dn4 = assign20640_e15416_d_n4;
        locals.var_vzadd_dn5 = assign20640_e15416_d_n5;
        locals.var_vzadd_dn6 = assign20640_e15416_d_n6;
        locals.var_vzadd_dn7 = assign20640_e15416_d_n7;
        locals.var_vzadd_dn8 = assign20640_e15416_d_n8;
        locals.var_vzadd_dn9 = assign20640_e15416_d_n9;
        locals.var_vzadd_dn10 = assign20640_e15416_d_n10;
        locals.var_vzadd_dn11 = assign20640_e15416_d_n11;
        locals.var_vzadd_dn14 = assign20640_e15416_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20650_e15426, assign20650_e15426_d_n0, assign20650_e15426_d_n2, assign20650_e15426_d_n4, assign20650_e15426_d_n5, assign20650_e15426_d_n6, assign20650_e15426_d_n7, assign20650_e15426_d_n8, assign20650_e15426_d_n9, assign20650_e15426_d_n10, assign20650_e15426_d_n11, assign20650_e15426_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20650_e15423: f64 = (2.0 * locals.var_vzadd);
        let assign20650_e15424: f64 = (locals.var_vdserev + assign20650_e15423);
        (assign20650_e15424, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20650_e15426;
        locals.var_vdserevz_dn0 = assign20650_e15426_d_n0;
        locals.var_vdserevz_dn2 = assign20650_e15426_d_n2;
        locals.var_vdserevz_dn4 = assign20650_e15426_d_n4;
        locals.var_vdserevz_dn5 = assign20650_e15426_d_n5;
        locals.var_vdserevz_dn6 = assign20650_e15426_d_n6;
        locals.var_vdserevz_dn7 = assign20650_e15426_d_n7;
        locals.var_vdserevz_dn8 = assign20650_e15426_d_n8;
        locals.var_vdserevz_dn9 = assign20650_e15426_d_n9;
        locals.var_vdserevz_dn10 = assign20650_e15426_d_n10;
        locals.var_vdserevz_dn11 = assign20650_e15426_d_n11;
        locals.var_vdserevz_dn14 = assign20650_e15426_d_n14;
        locals.var_vdserevz_rv = 0.0;

        let (assign20660_e15434, assign20660_e15434_d_n0, assign20660_e15434_d_n2, assign20660_e15434_d_n4, assign20660_e15434_d_n5, assign20660_e15434_d_n6, assign20660_e15434_d_n7, assign20660_e15434_d_n8, assign20660_e15434_d_n9, assign20660_e15434_d_n10, assign20660_e15434_d_n11, assign20660_e15434_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20660_e15432: f64 = (locals.var_vgserev + locals.var_vzadd);
        (assign20660_e15432, (locals.var_vgserev_dn0 + locals.var_vzadd_dn0), (locals.var_vgserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, (locals.var_vgserev_dn7 + locals.var_vzadd_dn7), locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vgserevz, locals.var_vgserevz_dn0, locals.var_vgserevz_dn2, locals.var_vgserevz_dn4, locals.var_vgserevz_dn5, locals.var_vgserevz_dn6, locals.var_vgserevz_dn7, locals.var_vgserevz_dn8, locals.var_vgserevz_dn9, locals.var_vgserevz_dn10, locals.var_vgserevz_dn11, locals.var_vgserevz_dn14,)
    }
};
        locals.var_vgserevz = assign20660_e15434;
        locals.var_vgserevz_dn0 = assign20660_e15434_d_n0;
        locals.var_vgserevz_dn2 = assign20660_e15434_d_n2;
        locals.var_vgserevz_dn4 = assign20660_e15434_d_n4;
        locals.var_vgserevz_dn5 = assign20660_e15434_d_n5;
        locals.var_vgserevz_dn6 = assign20660_e15434_d_n6;
        locals.var_vgserevz_dn7 = assign20660_e15434_d_n7;
        locals.var_vgserevz_dn8 = assign20660_e15434_d_n8;
        locals.var_vgserevz_dn9 = assign20660_e15434_d_n9;
        locals.var_vgserevz_dn10 = assign20660_e15434_d_n10;
        locals.var_vgserevz_dn11 = assign20660_e15434_d_n11;
        locals.var_vgserevz_dn14 = assign20660_e15434_d_n14;
        locals.var_vgserevz_rv = 0.0;

        let (assign20670_e15442, assign20670_e15442_d_n0, assign20670_e15442_d_n2, assign20670_e15442_d_n4, assign20670_e15442_d_n5, assign20670_e15442_d_n6, assign20670_e15442_d_n7, assign20670_e15442_d_n8, assign20670_e15442_d_n9, assign20670_e15442_d_n10, assign20670_e15442_d_n11, assign20670_e15442_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20670_e15440: f64 = (locals.var_vbserev + locals.var_vzadd);
        (assign20670_e15440, (locals.var_vbserev_dn0 + locals.var_vzadd_dn0), (locals.var_vbserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, (locals.var_vbserev_dn9 + locals.var_vzadd_dn9), locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vbserevz, locals.var_vbserevz_dn0, locals.var_vbserevz_dn2, locals.var_vbserevz_dn4, locals.var_vbserevz_dn5, locals.var_vbserevz_dn6, locals.var_vbserevz_dn7, locals.var_vbserevz_dn8, locals.var_vbserevz_dn9, locals.var_vbserevz_dn10, locals.var_vbserevz_dn11, locals.var_vbserevz_dn14,)
    }
};
        locals.var_vbserevz = assign20670_e15442;
        locals.var_vbserevz_dn0 = assign20670_e15442_d_n0;
        locals.var_vbserevz_dn2 = assign20670_e15442_d_n2;
        locals.var_vbserevz_dn4 = assign20670_e15442_d_n4;
        locals.var_vbserevz_dn5 = assign20670_e15442_d_n5;
        locals.var_vbserevz_dn6 = assign20670_e15442_d_n6;
        locals.var_vbserevz_dn7 = assign20670_e15442_d_n7;
        locals.var_vbserevz_dn8 = assign20670_e15442_d_n8;
        locals.var_vbserevz_dn9 = assign20670_e15442_d_n9;
        locals.var_vbserevz_dn10 = assign20670_e15442_d_n10;
        locals.var_vbserevz_dn11 = assign20670_e15442_d_n11;
        locals.var_vbserevz_dn14 = assign20670_e15442_d_n14;
        locals.var_vbserevz_rv = 0.0;

        let assign20680_e15449: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard417 = assign20680_e15449;
        locals.var_guard417_rv = 0.0;

        let (assign20690_e15463, assign20690_e15463_d_n0, assign20690_e15463_d_n2, assign20690_e15463_d_n4, assign20690_e15463_d_n5, assign20690_e15463_d_n6, assign20690_e15463_d_n7, assign20690_e15463_d_n8, assign20690_e15463_d_n9, assign20690_e15463_d_n10, assign20690_e15463_d_n11, assign20690_e15463_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20690_e15457: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign20690_e15460: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign20690_e15461: f64 = (assign20690_e15457 + assign20690_e15460);
        (assign20690_e15461, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20690_e15463;
        locals.var_t1_dn0 = assign20690_e15463_d_n0;
        locals.var_t1_dn2 = assign20690_e15463_d_n2;
        locals.var_t1_dn4 = assign20690_e15463_d_n4;
        locals.var_t1_dn5 = assign20690_e15463_d_n5;
        locals.var_t1_dn6 = assign20690_e15463_d_n6;
        locals.var_t1_dn7 = assign20690_e15463_d_n7;
        locals.var_t1_dn8 = assign20690_e15463_d_n8;
        locals.var_t1_dn9 = assign20690_e15463_d_n9;
        locals.var_t1_dn10 = assign20690_e15463_d_n10;
        locals.var_t1_dn11 = assign20690_e15463_d_n11;
        locals.var_t1_dn14 = assign20690_e15463_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20700_e15477, assign20700_e15477_d_n0, assign20700_e15477_d_n2, assign20700_e15477_d_n4, assign20700_e15477_d_n5, assign20700_e15477_d_n6, assign20700_e15477_d_n7, assign20700_e15477_d_n8, assign20700_e15477_d_n9, assign20700_e15477_d_n10, assign20700_e15477_d_n11, assign20700_e15477_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20700_e15471: f64 = (locals.var_vdsemodenml * locals.var_rdvde);
        let assign20700_e15474: f64 = (locals.var_vdsemodervs * locals.var_rsvde);
        let assign20700_e15475: f64 = (assign20700_e15471 + assign20700_e15474);
        (assign20700_e15475, ((locals.var_vdsemodenml * locals.var_rdvde_dn0) + (locals.var_vdsemodervs * locals.var_rsvde_dn0)), ((locals.var_vdsemodenml * locals.var_rdvde_dn2) + (locals.var_vdsemodervs * locals.var_rsvde_dn2)), ((locals.var_vdsemodenml * locals.var_rdvde_dn4) + (locals.var_vdsemodervs * locals.var_rsvde_dn4)), ((locals.var_vdsemodenml * locals.var_rdvde_dn5) + (locals.var_vdsemodervs * locals.var_rsvde_dn5)), ((locals.var_vdsemodenml * locals.var_rdvde_dn6) + (locals.var_vdsemodervs * locals.var_rsvde_dn6)), ((locals.var_vdsemodenml * locals.var_rdvde_dn7) + (locals.var_vdsemodervs * locals.var_rsvde_dn7)), ((locals.var_vdsemodenml * locals.var_rdvde_dn8) + (locals.var_vdsemodervs * locals.var_rsvde_dn8)), ((locals.var_vdsemodenml * locals.var_rdvde_dn9) + (locals.var_vdsemodervs * locals.var_rsvde_dn9)), ((locals.var_vdsemodenml * locals.var_rdvde_dn10) + (locals.var_vdsemodervs * locals.var_rsvde_dn10)), ((locals.var_vdsemodenml * locals.var_rdvde_dn11) + (locals.var_vdsemodervs * locals.var_rsvde_dn11)), ((locals.var_vdsemodenml * locals.var_rdvde_dn14) + (locals.var_vdsemodervs * locals.var_rsvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20700_e15477;
        locals.var_t0_dn0 = assign20700_e15477_d_n0;
        locals.var_t0_dn2 = assign20700_e15477_d_n2;
        locals.var_t0_dn4 = assign20700_e15477_d_n4;
        locals.var_t0_dn5 = assign20700_e15477_d_n5;
        locals.var_t0_dn6 = assign20700_e15477_d_n6;
        locals.var_t0_dn7 = assign20700_e15477_d_n7;
        locals.var_t0_dn8 = assign20700_e15477_d_n8;
        locals.var_t0_dn9 = assign20700_e15477_d_n9;
        locals.var_t0_dn10 = assign20700_e15477_d_n10;
        locals.var_t0_dn11 = assign20700_e15477_d_n11;
        locals.var_t0_dn14 = assign20700_e15477_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20710_e15489, assign20710_e15489_d_n0, assign20710_e15489_d_n2, assign20710_e15489_d_n4, assign20710_e15489_d_n5, assign20710_e15489_d_n6, assign20710_e15489_d_n7, assign20710_e15489_d_n8, assign20710_e15489_d_n9, assign20710_e15489_d_n10, assign20710_e15489_d_n11, assign20710_e15489_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20710_e15486: f64 = (locals.var_t0 * locals.var_vdserevz);
        let assign20710_e15487: f64 = (locals.var_t1 + assign20710_e15486);
        (assign20710_e15487, (locals.var_t1_dn0 + ((locals.var_t0_dn0 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn0))), (locals.var_t1_dn2 + ((locals.var_t0_dn2 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn2))), (locals.var_t1_dn4 + ((locals.var_t0_dn4 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn4))), (locals.var_t1_dn5 + ((locals.var_t0_dn5 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn5))), (locals.var_t1_dn6 + ((locals.var_t0_dn6 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn6))), (locals.var_t1_dn7 + ((locals.var_t0_dn7 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn7))), (locals.var_t1_dn8 + ((locals.var_t0_dn8 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn8))), (locals.var_t1_dn9 + ((locals.var_t0_dn9 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn9))), (locals.var_t1_dn10 + ((locals.var_t0_dn10 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn10))), (locals.var_t1_dn11 + ((locals.var_t0_dn11 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn11))), (locals.var_t1_dn14 + ((locals.var_t0_dn14 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20710_e15489;
        locals.var_t4_dn0 = assign20710_e15489_d_n0;
        locals.var_t4_dn2 = assign20710_e15489_d_n2;
        locals.var_t4_dn4 = assign20710_e15489_d_n4;
        locals.var_t4_dn5 = assign20710_e15489_d_n5;
        locals.var_t4_dn6 = assign20710_e15489_d_n6;
        locals.var_t4_dn7 = assign20710_e15489_d_n7;
        locals.var_t4_dn8 = assign20710_e15489_d_n8;
        locals.var_t4_dn9 = assign20710_e15489_d_n9;
        locals.var_t4_dn10 = assign20710_e15489_d_n10;
        locals.var_t4_dn11 = assign20710_e15489_d_n11;
        locals.var_t4_dn14 = assign20710_e15489_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20720_e15510, assign20720_e15510_d_n0, assign20720_e15510_d_n2, assign20720_e15510_d_n4, assign20720_e15510_d_n5, assign20720_e15510_d_n6, assign20720_e15510_d_n7, assign20720_e15510_d_n8, assign20720_e15510_d_n9, assign20720_e15510_d_n10, assign20720_e15510_d_n11, assign20720_e15510_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20720_e15497: f64 = (p.p292 * p.p292);
        let assign20720_e15501: f64 = (0.0001 * 0.01);
        let assign20720_e15502: f64 = (4.0 * assign20720_e15501);
        let assign20720_e15505: f64 = (0.0001 * 0.01);
        let assign20720_e15506: f64 = (assign20720_e15502 * assign20720_e15505);
        let assign20720_e15507: f64 = (assign20720_e15497 + assign20720_e15506);
        let assign20720_e15508: f64 = (assign20720_e15507).sqrt();
        (assign20720_e15508, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20720_e15510;
        locals.var_tmf2_dn0 = assign20720_e15510_d_n0;
        locals.var_tmf2_dn2 = assign20720_e15510_d_n2;
        locals.var_tmf2_dn4 = assign20720_e15510_d_n4;
        locals.var_tmf2_dn5 = assign20720_e15510_d_n5;
        locals.var_tmf2_dn6 = assign20720_e15510_d_n6;
        locals.var_tmf2_dn7 = assign20720_e15510_d_n7;
        locals.var_tmf2_dn8 = assign20720_e15510_d_n8;
        locals.var_tmf2_dn9 = assign20720_e15510_d_n9;
        locals.var_tmf2_dn10 = assign20720_e15510_d_n10;
        locals.var_tmf2_dn11 = assign20720_e15510_d_n11;
        locals.var_tmf2_dn14 = assign20720_e15510_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20730_e15524, assign20730_e15524_d_n0, assign20730_e15524_d_n2, assign20730_e15524_d_n4, assign20730_e15524_d_n5, assign20730_e15524_d_n6, assign20730_e15524_d_n7, assign20730_e15524_d_n8, assign20730_e15524_d_n9, assign20730_e15524_d_n10, assign20730_e15524_d_n11, assign20730_e15524_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20730_e15520: f64 = (p.p292 / locals.var_tmf2);
        let assign20730_e15521: f64 = (1.0 + assign20730_e15520);
        let assign20730_e15522: f64 = (0.5 * assign20730_e15521);
        (assign20730_e15522, (0.5 * (-((p.p292 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20730_e15524;
        locals.var_t0_dn0 = assign20730_e15524_d_n0;
        locals.var_t0_dn2 = assign20730_e15524_d_n2;
        locals.var_t0_dn4 = assign20730_e15524_d_n4;
        locals.var_t0_dn5 = assign20730_e15524_d_n5;
        locals.var_t0_dn6 = assign20730_e15524_d_n6;
        locals.var_t0_dn7 = assign20730_e15524_d_n7;
        locals.var_t0_dn8 = assign20730_e15524_d_n8;
        locals.var_t0_dn9 = assign20730_e15524_d_n9;
        locals.var_t0_dn10 = assign20730_e15524_d_n10;
        locals.var_t0_dn11 = assign20730_e15524_d_n11;
        locals.var_t0_dn14 = assign20730_e15524_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20740_e15536, assign20740_e15536_d_n0, assign20740_e15536_d_n2, assign20740_e15536_d_n4, assign20740_e15536_d_n5, assign20740_e15536_d_n6, assign20740_e15536_d_n7, assign20740_e15536_d_n8, assign20740_e15536_d_n9, assign20740_e15536_d_n10, assign20740_e15536_d_n11, assign20740_e15536_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20740_e15533: f64 = (p.p292 + locals.var_tmf2);
        let assign20740_e15534: f64 = (0.5 * assign20740_e15533);
        (assign20740_e15534, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20740_e15536;
        locals.var_t10_dn0 = assign20740_e15536_d_n0;
        locals.var_t10_dn2 = assign20740_e15536_d_n2;
        locals.var_t10_dn4 = assign20740_e15536_d_n4;
        locals.var_t10_dn5 = assign20740_e15536_d_n5;
        locals.var_t10_dn6 = assign20740_e15536_d_n6;
        locals.var_t10_dn7 = assign20740_e15536_d_n7;
        locals.var_t10_dn8 = assign20740_e15536_d_n8;
        locals.var_t10_dn9 = assign20740_e15536_d_n9;
        locals.var_t10_dn10 = assign20740_e15536_d_n10;
        locals.var_t10_dn11 = assign20740_e15536_d_n11;
        locals.var_t10_dn14 = assign20740_e15536_d_n14;
        locals.var_t10_rv = 0.0;

        let assign20750_e15539: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign20750_e15539;
        locals.var_guard418_rv = 0.0;

        let (assign20760_e15549, assign20760_e15549_d_n0, assign20760_e15549_d_n2, assign20760_e15549_d_n4, assign20760_e15549_d_n5, assign20760_e15549_d_n6, assign20760_e15549_d_n7, assign20760_e15549_d_n8, assign20760_e15549_d_n9, assign20760_e15549_d_n10, assign20760_e15549_d_n11, assign20760_e15549_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20760_e15549;
        locals.var_t10_dn0 = assign20760_e15549_d_n0;
        locals.var_t10_dn2 = assign20760_e15549_d_n2;
        locals.var_t10_dn4 = assign20760_e15549_d_n4;
        locals.var_t10_dn5 = assign20760_e15549_d_n5;
        locals.var_t10_dn6 = assign20760_e15549_d_n6;
        locals.var_t10_dn7 = assign20760_e15549_d_n7;
        locals.var_t10_dn8 = assign20760_e15549_d_n8;
        locals.var_t10_dn9 = assign20760_e15549_d_n9;
        locals.var_t10_dn10 = assign20760_e15549_d_n10;
        locals.var_t10_dn11 = assign20760_e15549_d_n11;
        locals.var_t10_dn14 = assign20760_e15549_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign20770_e15559, assign20770_e15559_d_n0, assign20770_e15559_d_n2, assign20770_e15559_d_n4, assign20770_e15559_d_n5, assign20770_e15559_d_n6, assign20770_e15559_d_n7, assign20770_e15559_d_n8, assign20770_e15559_d_n9, assign20770_e15559_d_n10, assign20770_e15559_d_n11, assign20770_e15559_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20770_e15559;
        locals.var_t0_dn0 = assign20770_e15559_d_n0;
        locals.var_t0_dn2 = assign20770_e15559_d_n2;
        locals.var_t0_dn4 = assign20770_e15559_d_n4;
        locals.var_t0_dn5 = assign20770_e15559_d_n5;
        locals.var_t0_dn6 = assign20770_e15559_d_n6;
        locals.var_t0_dn7 = assign20770_e15559_d_n7;
        locals.var_t0_dn8 = assign20770_e15559_d_n8;
        locals.var_t0_dn9 = assign20770_e15559_d_n9;
        locals.var_t0_dn10 = assign20770_e15559_d_n10;
        locals.var_t0_dn11 = assign20770_e15559_d_n11;
        locals.var_t0_dn14 = assign20770_e15559_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20780_e15577, assign20780_e15577_d_n0, assign20780_e15577_d_n2, assign20780_e15577_d_n4, assign20780_e15577_d_n5, assign20780_e15577_d_n6, assign20780_e15577_d_n7, assign20780_e15577_d_n8, assign20780_e15577_d_n9, assign20780_e15577_d_n10, assign20780_e15577_d_n11, assign20780_e15577_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20780_e15571: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign20780_e15572: f64 = (1.0 - assign20780_e15571);
        let assign20780_e15573: f64 = (locals.var_uc_rdvg11 * assign20780_e15572);
        let assign20780_e15574: f64 = (1.0 + assign20780_e15573);
        let assign20780_e15575: f64 = (locals.var_t4 * assign20780_e15574);
        (assign20780_e15575, ((locals.var_t4_dn0 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20780_e15577;
        locals.var_t1_dn0 = assign20780_e15577_d_n0;
        locals.var_t1_dn2 = assign20780_e15577_d_n2;
        locals.var_t1_dn4 = assign20780_e15577_d_n4;
        locals.var_t1_dn5 = assign20780_e15577_d_n5;
        locals.var_t1_dn6 = assign20780_e15577_d_n6;
        locals.var_t1_dn7 = assign20780_e15577_d_n7;
        locals.var_t1_dn8 = assign20780_e15577_d_n8;
        locals.var_t1_dn9 = assign20780_e15577_d_n9;
        locals.var_t1_dn10 = assign20780_e15577_d_n10;
        locals.var_t1_dn11 = assign20780_e15577_d_n11;
        locals.var_t1_dn14 = assign20780_e15577_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20790_e15591, assign20790_e15591_d_n0, assign20790_e15591_d_n2, assign20790_e15591_d_n4, assign20790_e15591_d_n5, assign20790_e15591_d_n6, assign20790_e15591_d_n7, assign20790_e15591_d_n8, assign20790_e15591_d_n9, assign20790_e15591_d_n10, assign20790_e15591_d_n11, assign20790_e15591_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20790_e15585: f64 = (locals.var_t1 - locals.var_t4);
        let assign20790_e15588: f64 = (0.01 * 0.01);
        let assign20790_e15589: f64 = (assign20790_e15585 - assign20790_e15588);
        (assign20790_e15589, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20790_e15591;
        locals.var_tmf1_dn0 = assign20790_e15591_d_n0;
        locals.var_tmf1_dn2 = assign20790_e15591_d_n2;
        locals.var_tmf1_dn4 = assign20790_e15591_d_n4;
        locals.var_tmf1_dn5 = assign20790_e15591_d_n5;
        locals.var_tmf1_dn6 = assign20790_e15591_d_n6;
        locals.var_tmf1_dn7 = assign20790_e15591_d_n7;
        locals.var_tmf1_dn8 = assign20790_e15591_d_n8;
        locals.var_tmf1_dn9 = assign20790_e15591_d_n9;
        locals.var_tmf1_dn10 = assign20790_e15591_d_n10;
        locals.var_tmf1_dn11 = assign20790_e15591_d_n11;
        locals.var_tmf1_dn14 = assign20790_e15591_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20800_e15605, assign20800_e15605_d_n0, assign20800_e15605_d_n2, assign20800_e15605_d_n4, assign20800_e15605_d_n5, assign20800_e15605_d_n6, assign20800_e15605_d_n7, assign20800_e15605_d_n8, assign20800_e15605_d_n9, assign20800_e15605_d_n10, assign20800_e15605_d_n11, assign20800_e15605_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20800_e15599: f64 = (4.0 * locals.var_t4);
        let assign20800_e15602: f64 = (0.01 * 0.01);
        let assign20800_e15603: f64 = (assign20800_e15599 * assign20800_e15602);
        (assign20800_e15603, ((4.0 * locals.var_t4_dn0) * assign20800_e15602), ((4.0 * locals.var_t4_dn2) * assign20800_e15602), ((4.0 * locals.var_t4_dn4) * assign20800_e15602), ((4.0 * locals.var_t4_dn5) * assign20800_e15602), ((4.0 * locals.var_t4_dn6) * assign20800_e15602), ((4.0 * locals.var_t4_dn7) * assign20800_e15602), ((4.0 * locals.var_t4_dn8) * assign20800_e15602), ((4.0 * locals.var_t4_dn9) * assign20800_e15602), ((4.0 * locals.var_t4_dn10) * assign20800_e15602), ((4.0 * locals.var_t4_dn11) * assign20800_e15602), ((4.0 * locals.var_t4_dn14) * assign20800_e15602),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20800_e15605;
        locals.var_tmf2_dn0 = assign20800_e15605_d_n0;
        locals.var_tmf2_dn2 = assign20800_e15605_d_n2;
        locals.var_tmf2_dn4 = assign20800_e15605_d_n4;
        locals.var_tmf2_dn5 = assign20800_e15605_d_n5;
        locals.var_tmf2_dn6 = assign20800_e15605_d_n6;
        locals.var_tmf2_dn7 = assign20800_e15605_d_n7;
        locals.var_tmf2_dn8 = assign20800_e15605_d_n8;
        locals.var_tmf2_dn9 = assign20800_e15605_d_n9;
        locals.var_tmf2_dn10 = assign20800_e15605_d_n10;
        locals.var_tmf2_dn11 = assign20800_e15605_d_n11;
        locals.var_tmf2_dn14 = assign20800_e15605_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20810_e15619, assign20810_e15619_d_n0, assign20810_e15619_d_n2, assign20810_e15619_d_n4, assign20810_e15619_d_n5, assign20810_e15619_d_n6, assign20810_e15619_d_n7, assign20810_e15619_d_n8, assign20810_e15619_d_n9, assign20810_e15619_d_n10, assign20810_e15619_d_n11, assign20810_e15619_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let (assign20810_e15617, assign20810_e15617_d_n0, assign20810_e15617_d_n2, assign20810_e15617_d_n4, assign20810_e15617_d_n5, assign20810_e15617_d_n6, assign20810_e15617_d_n7, assign20810_e15617_d_n8, assign20810_e15617_d_n9, assign20810_e15617_d_n10, assign20810_e15617_d_n11, assign20810_e15617_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20810_e15616: f64 = (-locals.var_tmf2);
                (assign20810_e15616, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20810_e15617, assign20810_e15617_d_n0, assign20810_e15617_d_n2, assign20810_e15617_d_n4, assign20810_e15617_d_n5, assign20810_e15617_d_n6, assign20810_e15617_d_n7, assign20810_e15617_d_n8, assign20810_e15617_d_n9, assign20810_e15617_d_n10, assign20810_e15617_d_n11, assign20810_e15617_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20810_e15619;
        locals.var_tmf2_dn0 = assign20810_e15619_d_n0;
        locals.var_tmf2_dn2 = assign20810_e15619_d_n2;
        locals.var_tmf2_dn4 = assign20810_e15619_d_n4;
        locals.var_tmf2_dn5 = assign20810_e15619_d_n5;
        locals.var_tmf2_dn6 = assign20810_e15619_d_n6;
        locals.var_tmf2_dn7 = assign20810_e15619_d_n7;
        locals.var_tmf2_dn8 = assign20810_e15619_d_n8;
        locals.var_tmf2_dn9 = assign20810_e15619_d_n9;
        locals.var_tmf2_dn10 = assign20810_e15619_d_n10;
        locals.var_tmf2_dn11 = assign20810_e15619_d_n11;
        locals.var_tmf2_dn14 = assign20810_e15619_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20820_e15632, assign20820_e15632_d_n0, assign20820_e15632_d_n2, assign20820_e15632_d_n4, assign20820_e15632_d_n5, assign20820_e15632_d_n6, assign20820_e15632_d_n7, assign20820_e15632_d_n8, assign20820_e15632_d_n9, assign20820_e15632_d_n10, assign20820_e15632_d_n11, assign20820_e15632_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20820_e15627: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20820_e15629: f64 = (assign20820_e15627 + locals.var_tmf2);
        let assign20820_e15630: f64 = (assign20820_e15629).sqrt();
        (assign20820_e15630, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20820_e15630)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20820_e15632;
        locals.var_tmf2_dn0 = assign20820_e15632_d_n0;
        locals.var_tmf2_dn2 = assign20820_e15632_d_n2;
        locals.var_tmf2_dn4 = assign20820_e15632_d_n4;
        locals.var_tmf2_dn5 = assign20820_e15632_d_n5;
        locals.var_tmf2_dn6 = assign20820_e15632_d_n6;
        locals.var_tmf2_dn7 = assign20820_e15632_d_n7;
        locals.var_tmf2_dn8 = assign20820_e15632_d_n8;
        locals.var_tmf2_dn9 = assign20820_e15632_d_n9;
        locals.var_tmf2_dn10 = assign20820_e15632_d_n10;
        locals.var_tmf2_dn11 = assign20820_e15632_d_n11;
        locals.var_tmf2_dn14 = assign20820_e15632_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20830_e15646, assign20830_e15646_d_n0, assign20830_e15646_d_n2, assign20830_e15646_d_n4, assign20830_e15646_d_n5, assign20830_e15646_d_n6, assign20830_e15646_d_n7, assign20830_e15646_d_n8, assign20830_e15646_d_n9, assign20830_e15646_d_n10, assign20830_e15646_d_n11, assign20830_e15646_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20830_e15642: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20830_e15643: f64 = (1.0 + assign20830_e15642);
        let assign20830_e15644: f64 = (0.5 * assign20830_e15643);
        (assign20830_e15644, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20830_e15646;
        locals.var_t0_dn0 = assign20830_e15646_d_n0;
        locals.var_t0_dn2 = assign20830_e15646_d_n2;
        locals.var_t0_dn4 = assign20830_e15646_d_n4;
        locals.var_t0_dn5 = assign20830_e15646_d_n5;
        locals.var_t0_dn6 = assign20830_e15646_d_n6;
        locals.var_t0_dn7 = assign20830_e15646_d_n7;
        locals.var_t0_dn8 = assign20830_e15646_d_n8;
        locals.var_t0_dn9 = assign20830_e15646_d_n9;
        locals.var_t0_dn10 = assign20830_e15646_d_n10;
        locals.var_t0_dn11 = assign20830_e15646_d_n11;
        locals.var_t0_dn14 = assign20830_e15646_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20840_e15666, assign20840_e15666_d_n0, assign20840_e15666_d_n2, assign20840_e15666_d_n4, assign20840_e15666_d_n5, assign20840_e15666_d_n6, assign20840_e15666_d_n7, assign20840_e15666_d_n8, assign20840_e15666_d_n9, assign20840_e15666_d_n10, assign20840_e15666_d_n11, assign20840_e15666_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20840_e15657: f64 = (2.0 * 0.01);
        let assign20840_e15659: f64 = (assign20840_e15657 * 0.01);
        let assign20840_e15660: f64 = (locals.var_tmf1 - assign20840_e15659);
        let assign20840_e15662: f64 = (assign20840_e15660 / locals.var_tmf2);
        let assign20840_e15663: f64 = (1.0 - assign20840_e15662);
        let assign20840_e15664: f64 = (0.5 * assign20840_e15663);
        (assign20840_e15664, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20840_e15666;
        locals.var_t5_dn0 = assign20840_e15666_d_n0;
        locals.var_t5_dn2 = assign20840_e15666_d_n2;
        locals.var_t5_dn4 = assign20840_e15666_d_n4;
        locals.var_t5_dn5 = assign20840_e15666_d_n5;
        locals.var_t5_dn6 = assign20840_e15666_d_n6;
        locals.var_t5_dn7 = assign20840_e15666_d_n7;
        locals.var_t5_dn8 = assign20840_e15666_d_n8;
        locals.var_t5_dn9 = assign20840_e15666_d_n9;
        locals.var_t5_dn10 = assign20840_e15666_d_n10;
        locals.var_t5_dn11 = assign20840_e15666_d_n11;
        locals.var_t5_dn14 = assign20840_e15666_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign20850_e15680, assign20850_e15680_d_n0, assign20850_e15680_d_n2, assign20850_e15680_d_n4, assign20850_e15680_d_n5, assign20850_e15680_d_n6, assign20850_e15680_d_n7, assign20850_e15680_d_n8, assign20850_e15680_d_n9, assign20850_e15680_d_n10, assign20850_e15680_d_n11, assign20850_e15680_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20850_e15676: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20850_e15677: f64 = (0.5 * assign20850_e15676);
        let assign20850_e15678: f64 = (locals.var_t4 + assign20850_e15677);
        (assign20850_e15678, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20850_e15680;
        locals.var_t2_dn0 = assign20850_e15680_d_n0;
        locals.var_t2_dn2 = assign20850_e15680_d_n2;
        locals.var_t2_dn4 = assign20850_e15680_d_n4;
        locals.var_t2_dn5 = assign20850_e15680_d_n5;
        locals.var_t2_dn6 = assign20850_e15680_d_n6;
        locals.var_t2_dn7 = assign20850_e15680_d_n7;
        locals.var_t2_dn8 = assign20850_e15680_d_n8;
        locals.var_t2_dn9 = assign20850_e15680_d_n9;
        locals.var_t2_dn10 = assign20850_e15680_d_n10;
        locals.var_t2_dn11 = assign20850_e15680_d_n11;
        locals.var_t2_dn14 = assign20850_e15680_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20860_e15692, assign20860_e15692_d_n0, assign20860_e15692_d_n2, assign20860_e15692_d_n4, assign20860_e15692_d_n5, assign20860_e15692_d_n6, assign20860_e15692_d_n7, assign20860_e15692_d_n8, assign20860_e15692_d_n9, assign20860_e15692_d_n10, assign20860_e15692_d_n11, assign20860_e15692_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20860_e15689: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign20860_e15690: f64 = (locals.var_t4 * assign20860_e15689);
        (assign20860_e15690, (locals.var_t4_dn0 * assign20860_e15689), (locals.var_t4_dn2 * assign20860_e15689), (locals.var_t4_dn4 * assign20860_e15689), (locals.var_t4_dn5 * assign20860_e15689), (locals.var_t4_dn6 * assign20860_e15689), (locals.var_t4_dn7 * assign20860_e15689), (locals.var_t4_dn8 * assign20860_e15689), (locals.var_t4_dn9 * assign20860_e15689), (locals.var_t4_dn10 * assign20860_e15689), (locals.var_t4_dn11 * assign20860_e15689), (locals.var_t4_dn14 * assign20860_e15689),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20860_e15692;
        locals.var_t3_dn0 = assign20860_e15692_d_n0;
        locals.var_t3_dn2 = assign20860_e15692_d_n2;
        locals.var_t3_dn4 = assign20860_e15692_d_n4;
        locals.var_t3_dn5 = assign20860_e15692_d_n5;
        locals.var_t3_dn6 = assign20860_e15692_d_n6;
        locals.var_t3_dn7 = assign20860_e15692_d_n7;
        locals.var_t3_dn8 = assign20860_e15692_d_n8;
        locals.var_t3_dn9 = assign20860_e15692_d_n9;
        locals.var_t3_dn10 = assign20860_e15692_d_n10;
        locals.var_t3_dn11 = assign20860_e15692_d_n11;
        locals.var_t3_dn14 = assign20860_e15692_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign20870_e15706, assign20870_e15706_d_n0, assign20870_e15706_d_n2, assign20870_e15706_d_n4, assign20870_e15706_d_n5, assign20870_e15706_d_n6, assign20870_e15706_d_n7, assign20870_e15706_d_n8, assign20870_e15706_d_n9, assign20870_e15706_d_n10, assign20870_e15706_d_n11, assign20870_e15706_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20870_e15700: f64 = (locals.var_t3 - locals.var_t2);
        let assign20870_e15703: f64 = (5e-5 * 0.01);
        let assign20870_e15704: f64 = (assign20870_e15700 - assign20870_e15703);
        (assign20870_e15704, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20870_e15706;
        locals.var_tmf1_dn0 = assign20870_e15706_d_n0;
        locals.var_tmf1_dn2 = assign20870_e15706_d_n2;
        locals.var_tmf1_dn4 = assign20870_e15706_d_n4;
        locals.var_tmf1_dn5 = assign20870_e15706_d_n5;
        locals.var_tmf1_dn6 = assign20870_e15706_d_n6;
        locals.var_tmf1_dn7 = assign20870_e15706_d_n7;
        locals.var_tmf1_dn8 = assign20870_e15706_d_n8;
        locals.var_tmf1_dn9 = assign20870_e15706_d_n9;
        locals.var_tmf1_dn10 = assign20870_e15706_d_n10;
        locals.var_tmf1_dn11 = assign20870_e15706_d_n11;
        locals.var_tmf1_dn14 = assign20870_e15706_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20880_e15720, assign20880_e15720_d_n0, assign20880_e15720_d_n2, assign20880_e15720_d_n4, assign20880_e15720_d_n5, assign20880_e15720_d_n6, assign20880_e15720_d_n7, assign20880_e15720_d_n8, assign20880_e15720_d_n9, assign20880_e15720_d_n10, assign20880_e15720_d_n11, assign20880_e15720_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20880_e15714: f64 = (4.0 * locals.var_t3);
        let assign20880_e15717: f64 = (5e-5 * 0.01);
        let assign20880_e15718: f64 = (assign20880_e15714 * assign20880_e15717);
        (assign20880_e15718, ((4.0 * locals.var_t3_dn0) * assign20880_e15717), ((4.0 * locals.var_t3_dn2) * assign20880_e15717), ((4.0 * locals.var_t3_dn4) * assign20880_e15717), ((4.0 * locals.var_t3_dn5) * assign20880_e15717), ((4.0 * locals.var_t3_dn6) * assign20880_e15717), ((4.0 * locals.var_t3_dn7) * assign20880_e15717), ((4.0 * locals.var_t3_dn8) * assign20880_e15717), ((4.0 * locals.var_t3_dn9) * assign20880_e15717), ((4.0 * locals.var_t3_dn10) * assign20880_e15717), ((4.0 * locals.var_t3_dn11) * assign20880_e15717), ((4.0 * locals.var_t3_dn14) * assign20880_e15717),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20880_e15720;
        locals.var_tmf2_dn0 = assign20880_e15720_d_n0;
        locals.var_tmf2_dn2 = assign20880_e15720_d_n2;
        locals.var_tmf2_dn4 = assign20880_e15720_d_n4;
        locals.var_tmf2_dn5 = assign20880_e15720_d_n5;
        locals.var_tmf2_dn6 = assign20880_e15720_d_n6;
        locals.var_tmf2_dn7 = assign20880_e15720_d_n7;
        locals.var_tmf2_dn8 = assign20880_e15720_d_n8;
        locals.var_tmf2_dn9 = assign20880_e15720_d_n9;
        locals.var_tmf2_dn10 = assign20880_e15720_d_n10;
        locals.var_tmf2_dn11 = assign20880_e15720_d_n11;
        locals.var_tmf2_dn14 = assign20880_e15720_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20890_e15734, assign20890_e15734_d_n0, assign20890_e15734_d_n2, assign20890_e15734_d_n4, assign20890_e15734_d_n5, assign20890_e15734_d_n6, assign20890_e15734_d_n7, assign20890_e15734_d_n8, assign20890_e15734_d_n9, assign20890_e15734_d_n10, assign20890_e15734_d_n11, assign20890_e15734_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let (assign20890_e15732, assign20890_e15732_d_n0, assign20890_e15732_d_n2, assign20890_e15732_d_n4, assign20890_e15732_d_n5, assign20890_e15732_d_n6, assign20890_e15732_d_n7, assign20890_e15732_d_n8, assign20890_e15732_d_n9, assign20890_e15732_d_n10, assign20890_e15732_d_n11, assign20890_e15732_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20890_e15731: f64 = (-locals.var_tmf2);
                (assign20890_e15731, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20890_e15732, assign20890_e15732_d_n0, assign20890_e15732_d_n2, assign20890_e15732_d_n4, assign20890_e15732_d_n5, assign20890_e15732_d_n6, assign20890_e15732_d_n7, assign20890_e15732_d_n8, assign20890_e15732_d_n9, assign20890_e15732_d_n10, assign20890_e15732_d_n11, assign20890_e15732_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20890_e15734;
        locals.var_tmf2_dn0 = assign20890_e15734_d_n0;
        locals.var_tmf2_dn2 = assign20890_e15734_d_n2;
        locals.var_tmf2_dn4 = assign20890_e15734_d_n4;
        locals.var_tmf2_dn5 = assign20890_e15734_d_n5;
        locals.var_tmf2_dn6 = assign20890_e15734_d_n6;
        locals.var_tmf2_dn7 = assign20890_e15734_d_n7;
        locals.var_tmf2_dn8 = assign20890_e15734_d_n8;
        locals.var_tmf2_dn9 = assign20890_e15734_d_n9;
        locals.var_tmf2_dn10 = assign20890_e15734_d_n10;
        locals.var_tmf2_dn11 = assign20890_e15734_d_n11;
        locals.var_tmf2_dn14 = assign20890_e15734_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20900_e15747, assign20900_e15747_d_n0, assign20900_e15747_d_n2, assign20900_e15747_d_n4, assign20900_e15747_d_n5, assign20900_e15747_d_n6, assign20900_e15747_d_n7, assign20900_e15747_d_n8, assign20900_e15747_d_n9, assign20900_e15747_d_n10, assign20900_e15747_d_n11, assign20900_e15747_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20900_e15742: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20900_e15744: f64 = (assign20900_e15742 + locals.var_tmf2);
        let assign20900_e15745: f64 = (assign20900_e15744).sqrt();
        (assign20900_e15745, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20900_e15745)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20900_e15747;
        locals.var_tmf2_dn0 = assign20900_e15747_d_n0;
        locals.var_tmf2_dn2 = assign20900_e15747_d_n2;
        locals.var_tmf2_dn4 = assign20900_e15747_d_n4;
        locals.var_tmf2_dn5 = assign20900_e15747_d_n5;
        locals.var_tmf2_dn6 = assign20900_e15747_d_n6;
        locals.var_tmf2_dn7 = assign20900_e15747_d_n7;
        locals.var_tmf2_dn8 = assign20900_e15747_d_n8;
        locals.var_tmf2_dn9 = assign20900_e15747_d_n9;
        locals.var_tmf2_dn10 = assign20900_e15747_d_n10;
        locals.var_tmf2_dn11 = assign20900_e15747_d_n11;
        locals.var_tmf2_dn14 = assign20900_e15747_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20910_e15761, assign20910_e15761_d_n0, assign20910_e15761_d_n2, assign20910_e15761_d_n4, assign20910_e15761_d_n5, assign20910_e15761_d_n6, assign20910_e15761_d_n7, assign20910_e15761_d_n8, assign20910_e15761_d_n9, assign20910_e15761_d_n10, assign20910_e15761_d_n11, assign20910_e15761_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20910_e15757: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20910_e15758: f64 = (1.0 + assign20910_e15757);
        let assign20910_e15759: f64 = (0.5 * assign20910_e15758);
        (assign20910_e15759, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20910_e15761;
        locals.var_t0_dn0 = assign20910_e15761_d_n0;
        locals.var_t0_dn2 = assign20910_e15761_d_n2;
        locals.var_t0_dn4 = assign20910_e15761_d_n4;
        locals.var_t0_dn5 = assign20910_e15761_d_n5;
        locals.var_t0_dn6 = assign20910_e15761_d_n6;
        locals.var_t0_dn7 = assign20910_e15761_d_n7;
        locals.var_t0_dn8 = assign20910_e15761_d_n8;
        locals.var_t0_dn9 = assign20910_e15761_d_n9;
        locals.var_t0_dn10 = assign20910_e15761_d_n10;
        locals.var_t0_dn11 = assign20910_e15761_d_n11;
        locals.var_t0_dn14 = assign20910_e15761_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20920_e15781, assign20920_e15781_d_n0, assign20920_e15781_d_n2, assign20920_e15781_d_n4, assign20920_e15781_d_n5, assign20920_e15781_d_n6, assign20920_e15781_d_n7, assign20920_e15781_d_n8, assign20920_e15781_d_n9, assign20920_e15781_d_n10, assign20920_e15781_d_n11, assign20920_e15781_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20920_e15772: f64 = (2.0 * 5e-5);
        let assign20920_e15774: f64 = (assign20920_e15772 * 0.01);
        let assign20920_e15775: f64 = (locals.var_tmf1 + assign20920_e15774);
        let assign20920_e15777: f64 = (assign20920_e15775 / locals.var_tmf2);
        let assign20920_e15778: f64 = (1.0 - assign20920_e15777);
        let assign20920_e15779: f64 = (0.5 * assign20920_e15778);
        (assign20920_e15779, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20920_e15781;
        locals.var_t5_dn0 = assign20920_e15781_d_n0;
        locals.var_t5_dn2 = assign20920_e15781_d_n2;
        locals.var_t5_dn4 = assign20920_e15781_d_n4;
        locals.var_t5_dn5 = assign20920_e15781_d_n5;
        locals.var_t5_dn6 = assign20920_e15781_d_n6;
        locals.var_t5_dn7 = assign20920_e15781_d_n7;
        locals.var_t5_dn8 = assign20920_e15781_d_n8;
        locals.var_t5_dn9 = assign20920_e15781_d_n9;
        locals.var_t5_dn10 = assign20920_e15781_d_n10;
        locals.var_t5_dn11 = assign20920_e15781_d_n11;
        locals.var_t5_dn14 = assign20920_e15781_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign20930_e15795, assign20930_e15795_d_n0, assign20930_e15795_d_n2, assign20930_e15795_d_n4, assign20930_e15795_d_n5, assign20930_e15795_d_n6, assign20930_e15795_d_n7, assign20930_e15795_d_n8, assign20930_e15795_d_n9, assign20930_e15795_d_n10, assign20930_e15795_d_n11, assign20930_e15795_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20930_e15791: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20930_e15792: f64 = (0.5 * assign20930_e15791);
        let assign20930_e15793: f64 = (locals.var_t3 - assign20930_e15792);
        (assign20930_e15793, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign20930_e15795;
        locals.var_rdrift_dn0 = assign20930_e15795_d_n0;
        locals.var_rdrift_dn2 = assign20930_e15795_d_n2;
        locals.var_rdrift_dn4 = assign20930_e15795_d_n4;
        locals.var_rdrift_dn5 = assign20930_e15795_d_n5;
        locals.var_rdrift_dn6 = assign20930_e15795_d_n6;
        locals.var_rdrift_dn7 = assign20930_e15795_d_n7;
        locals.var_rdrift_dn8 = assign20930_e15795_d_n8;
        locals.var_rdrift_dn9 = assign20930_e15795_d_n9;
        locals.var_rdrift_dn10 = assign20930_e15795_d_n10;
        locals.var_rdrift_dn11 = assign20930_e15795_d_n11;
        locals.var_rdrift_dn14 = assign20930_e15795_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign20940_e15807, assign20940_e15807_d_n0, assign20940_e15807_d_n2, assign20940_e15807_d_n4, assign20940_e15807_d_n5, assign20940_e15807_d_n6, assign20940_e15807_d_n7, assign20940_e15807_d_n8, assign20940_e15807_d_n9, assign20940_e15807_d_n10, assign20940_e15807_d_n11, assign20940_e15807_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20940_e15804: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign20940_e15805: f64 = (1.0 - assign20940_e15804);
        (assign20940_e15805, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20940_e15807;
        locals.var_t1_dn0 = assign20940_e15807_d_n0;
        locals.var_t1_dn2 = assign20940_e15807_d_n2;
        locals.var_t1_dn4 = assign20940_e15807_d_n4;
        locals.var_t1_dn5 = assign20940_e15807_d_n5;
        locals.var_t1_dn6 = assign20940_e15807_d_n6;
        locals.var_t1_dn7 = assign20940_e15807_d_n7;
        locals.var_t1_dn8 = assign20940_e15807_d_n8;
        locals.var_t1_dn9 = assign20940_e15807_d_n9;
        locals.var_t1_dn10 = assign20940_e15807_d_n10;
        locals.var_t1_dn11 = assign20940_e15807_d_n11;
        locals.var_t1_dn14 = assign20940_e15807_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20950_e15828, assign20950_e15828_d_n0, assign20950_e15828_d_n2, assign20950_e15828_d_n4, assign20950_e15828_d_n5, assign20950_e15828_d_n6, assign20950_e15828_d_n7, assign20950_e15828_d_n8, assign20950_e15828_d_n9, assign20950_e15828_d_n10, assign20950_e15828_d_n11, assign20950_e15828_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20950_e15815: f64 = (locals.var_t1 * locals.var_t1);
        let assign20950_e15819: f64 = (0.0001 * 0.01);
        let assign20950_e15820: f64 = (4.0 * assign20950_e15819);
        let assign20950_e15823: f64 = (0.0001 * 0.01);
        let assign20950_e15824: f64 = (assign20950_e15820 * assign20950_e15823);
        let assign20950_e15825: f64 = (assign20950_e15815 + assign20950_e15824);
        let assign20950_e15826: f64 = (assign20950_e15825).sqrt();
        (assign20950_e15826, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign20950_e15826)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20950_e15828;
        locals.var_tmf2_dn0 = assign20950_e15828_d_n0;
        locals.var_tmf2_dn2 = assign20950_e15828_d_n2;
        locals.var_tmf2_dn4 = assign20950_e15828_d_n4;
        locals.var_tmf2_dn5 = assign20950_e15828_d_n5;
        locals.var_tmf2_dn6 = assign20950_e15828_d_n6;
        locals.var_tmf2_dn7 = assign20950_e15828_d_n7;
        locals.var_tmf2_dn8 = assign20950_e15828_d_n8;
        locals.var_tmf2_dn9 = assign20950_e15828_d_n9;
        locals.var_tmf2_dn10 = assign20950_e15828_d_n10;
        locals.var_tmf2_dn11 = assign20950_e15828_d_n11;
        locals.var_tmf2_dn14 = assign20950_e15828_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20960_e15842, assign20960_e15842_d_n0, assign20960_e15842_d_n2, assign20960_e15842_d_n4, assign20960_e15842_d_n5, assign20960_e15842_d_n6, assign20960_e15842_d_n7, assign20960_e15842_d_n8, assign20960_e15842_d_n9, assign20960_e15842_d_n10, assign20960_e15842_d_n11, assign20960_e15842_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20960_e15838: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign20960_e15839: f64 = (1.0 + assign20960_e15838);
        let assign20960_e15840: f64 = (0.5 * assign20960_e15839);
        (assign20960_e15840, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20960_e15842;
        locals.var_t4_dn0 = assign20960_e15842_d_n0;
        locals.var_t4_dn2 = assign20960_e15842_d_n2;
        locals.var_t4_dn4 = assign20960_e15842_d_n4;
        locals.var_t4_dn5 = assign20960_e15842_d_n5;
        locals.var_t4_dn6 = assign20960_e15842_d_n6;
        locals.var_t4_dn7 = assign20960_e15842_d_n7;
        locals.var_t4_dn8 = assign20960_e15842_d_n8;
        locals.var_t4_dn9 = assign20960_e15842_d_n9;
        locals.var_t4_dn10 = assign20960_e15842_d_n10;
        locals.var_t4_dn11 = assign20960_e15842_d_n11;
        locals.var_t4_dn14 = assign20960_e15842_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20970_e15854, assign20970_e15854_d_n0, assign20970_e15854_d_n2, assign20970_e15854_d_n4, assign20970_e15854_d_n5, assign20970_e15854_d_n6, assign20970_e15854_d_n7, assign20970_e15854_d_n8, assign20970_e15854_d_n9, assign20970_e15854_d_n10, assign20970_e15854_d_n11, assign20970_e15854_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20970_e15851: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign20970_e15852: f64 = (0.5 * assign20970_e15851);
        (assign20970_e15852, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20970_e15854;
        locals.var_t3_dn0 = assign20970_e15854_d_n0;
        locals.var_t3_dn2 = assign20970_e15854_d_n2;
        locals.var_t3_dn4 = assign20970_e15854_d_n4;
        locals.var_t3_dn5 = assign20970_e15854_d_n5;
        locals.var_t3_dn6 = assign20970_e15854_d_n6;
        locals.var_t3_dn7 = assign20970_e15854_d_n7;
        locals.var_t3_dn8 = assign20970_e15854_d_n8;
        locals.var_t3_dn9 = assign20970_e15854_d_n9;
        locals.var_t3_dn10 = assign20970_e15854_d_n10;
        locals.var_t3_dn11 = assign20970_e15854_d_n11;
        locals.var_t3_dn14 = assign20970_e15854_d_n14;
        locals.var_t3_rv = 0.0;

        let assign20980_e15857: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign20980_e15857;
        locals.var_guard419_rv = 0.0;

        let (assign20990_e15867, assign20990_e15867_d_n0, assign20990_e15867_d_n2, assign20990_e15867_d_n4, assign20990_e15867_d_n5, assign20990_e15867_d_n6, assign20990_e15867_d_n7, assign20990_e15867_d_n8, assign20990_e15867_d_n9, assign20990_e15867_d_n10, assign20990_e15867_d_n11, assign20990_e15867_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20990_e15867;
        locals.var_t3_dn0 = assign20990_e15867_d_n0;
        locals.var_t3_dn2 = assign20990_e15867_d_n2;
        locals.var_t3_dn4 = assign20990_e15867_d_n4;
        locals.var_t3_dn5 = assign20990_e15867_d_n5;
        locals.var_t3_dn6 = assign20990_e15867_d_n6;
        locals.var_t3_dn7 = assign20990_e15867_d_n7;
        locals.var_t3_dn8 = assign20990_e15867_d_n8;
        locals.var_t3_dn9 = assign20990_e15867_d_n9;
        locals.var_t3_dn10 = assign20990_e15867_d_n10;
        locals.var_t3_dn11 = assign20990_e15867_d_n11;
        locals.var_t3_dn14 = assign20990_e15867_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21000_e15877, assign21000_e15877_d_n0, assign21000_e15877_d_n2, assign21000_e15877_d_n4, assign21000_e15877_d_n5, assign21000_e15877_d_n6, assign21000_e15877_d_n7, assign21000_e15877_d_n8, assign21000_e15877_d_n9, assign21000_e15877_d_n10, assign21000_e15877_d_n11, assign21000_e15877_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21000_e15877;
        locals.var_t4_dn0 = assign21000_e15877_d_n0;
        locals.var_t4_dn2 = assign21000_e15877_d_n2;
        locals.var_t4_dn4 = assign21000_e15877_d_n4;
        locals.var_t4_dn5 = assign21000_e15877_d_n5;
        locals.var_t4_dn6 = assign21000_e15877_d_n6;
        locals.var_t4_dn7 = assign21000_e15877_d_n7;
        locals.var_t4_dn8 = assign21000_e15877_d_n8;
        locals.var_t4_dn9 = assign21000_e15877_d_n9;
        locals.var_t4_dn10 = assign21000_e15877_d_n10;
        locals.var_t4_dn11 = assign21000_e15877_d_n11;
        locals.var_t4_dn14 = assign21000_e15877_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21010_e15887, assign21010_e15887_d_n0, assign21010_e15887_d_n2, assign21010_e15887_d_n4, assign21010_e15887_d_n5, assign21010_e15887_d_n6, assign21010_e15887_d_n7, assign21010_e15887_d_n8, assign21010_e15887_d_n9, assign21010_e15887_d_n10, assign21010_e15887_d_n11, assign21010_e15887_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign21010_e15885: f64 = (locals.var_t3 + 1e-25);
        (assign21010_e15885, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21010_e15887;
        locals.var_t3_dn0 = assign21010_e15887_d_n0;
        locals.var_t3_dn2 = assign21010_e15887_d_n2;
        locals.var_t3_dn4 = assign21010_e15887_d_n4;
        locals.var_t3_dn5 = assign21010_e15887_d_n5;
        locals.var_t3_dn6 = assign21010_e15887_d_n6;
        locals.var_t3_dn7 = assign21010_e15887_d_n7;
        locals.var_t3_dn8 = assign21010_e15887_d_n8;
        locals.var_t3_dn9 = assign21010_e15887_d_n9;
        locals.var_t3_dn10 = assign21010_e15887_d_n10;
        locals.var_t3_dn11 = assign21010_e15887_d_n11;
        locals.var_t3_dn14 = assign21010_e15887_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21020_e15895, assign21020_e15895_d_n0, assign21020_e15895_d_n2, assign21020_e15895_d_n4, assign21020_e15895_d_n5, assign21020_e15895_d_n6, assign21020_e15895_d_n7, assign21020_e15895_d_n8, assign21020_e15895_d_n9, assign21020_e15895_d_n10, assign21020_e15895_d_n11, assign21020_e15895_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21020_e15895;
        locals.var_t0_dn0 = assign21020_e15895_d_n0;
        locals.var_t0_dn2 = assign21020_e15895_d_n2;
        locals.var_t0_dn4 = assign21020_e15895_d_n4;
        locals.var_t0_dn5 = assign21020_e15895_d_n5;
        locals.var_t0_dn6 = assign21020_e15895_d_n6;
        locals.var_t0_dn7 = assign21020_e15895_d_n7;
        locals.var_t0_dn8 = assign21020_e15895_d_n8;
        locals.var_t0_dn9 = assign21020_e15895_d_n9;
        locals.var_t0_dn10 = assign21020_e15895_d_n10;
        locals.var_t0_dn11 = assign21020_e15895_d_n11;
        locals.var_t0_dn14 = assign21020_e15895_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21030_e15905, assign21030_e15905_d_n0, assign21030_e15905_d_n2, assign21030_e15905_d_n4, assign21030_e15905_d_n5, assign21030_e15905_d_n6, assign21030_e15905_d_n7, assign21030_e15905_d_n8, assign21030_e15905_d_n9, assign21030_e15905_d_n10, assign21030_e15905_d_n11, assign21030_e15905_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign21030_e15903: f64 = (locals.var_rdrift * locals.var_t3);
        (assign21030_e15903, ((locals.var_rdrift_dn0 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn0)), ((locals.var_rdrift_dn2 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn2)), ((locals.var_rdrift_dn4 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn4)), ((locals.var_rdrift_dn5 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn5)), ((locals.var_rdrift_dn6 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn6)), ((locals.var_rdrift_dn7 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn7)), ((locals.var_rdrift_dn8 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn8)), ((locals.var_rdrift_dn9 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn9)), ((locals.var_rdrift_dn10 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn10)), ((locals.var_rdrift_dn11 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn11)), ((locals.var_rdrift_dn14 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21030_e15905;
        locals.var_rdrift_dn0 = assign21030_e15905_d_n0;
        locals.var_rdrift_dn2 = assign21030_e15905_d_n2;
        locals.var_rdrift_dn4 = assign21030_e15905_d_n4;
        locals.var_rdrift_dn5 = assign21030_e15905_d_n5;
        locals.var_rdrift_dn6 = assign21030_e15905_d_n6;
        locals.var_rdrift_dn7 = assign21030_e15905_d_n7;
        locals.var_rdrift_dn8 = assign21030_e15905_d_n8;
        locals.var_rdrift_dn9 = assign21030_e15905_d_n9;
        locals.var_rdrift_dn10 = assign21030_e15905_d_n10;
        locals.var_rdrift_dn11 = assign21030_e15905_d_n11;
        locals.var_rdrift_dn14 = assign21030_e15905_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21040_e15914, assign21040_e15914_d_n0, assign21040_e15914_d_n2, assign21040_e15914_d_n4, assign21040_e15914_d_n5, assign21040_e15914_d_n6, assign21040_e15914_d_n7, assign21040_e15914_d_n8, assign21040_e15914_d_n9, assign21040_e15914_d_n10, assign21040_e15914_d_n11, assign21040_e15914_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21040_e15914;
        locals.var_rdrift_dn0 = assign21040_e15914_d_n0;
        locals.var_rdrift_dn2 = assign21040_e15914_d_n2;
        locals.var_rdrift_dn4 = assign21040_e15914_d_n4;
        locals.var_rdrift_dn5 = assign21040_e15914_d_n5;
        locals.var_rdrift_dn6 = assign21040_e15914_d_n6;
        locals.var_rdrift_dn7 = assign21040_e15914_d_n7;
        locals.var_rdrift_dn8 = assign21040_e15914_d_n8;
        locals.var_rdrift_dn9 = assign21040_e15914_d_n9;
        locals.var_rdrift_dn10 = assign21040_e15914_d_n10;
        locals.var_rdrift_dn11 = assign21040_e15914_d_n11;
        locals.var_rdrift_dn14 = assign21040_e15914_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21050_e15926, assign21050_e15926_d_n0, assign21050_e15926_d_n2, assign21050_e15926_d_n4, assign21050_e15926_d_n5, assign21050_e15926_d_n6, assign21050_e15926_d_n7, assign21050_e15926_d_n8, assign21050_e15926_d_n9, assign21050_e15926_d_n10, assign21050_e15926_d_n11, assign21050_e15926_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign21050_e15920: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21050_e15923: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21050_e15924: f64 = (assign21050_e15920 + assign21050_e15923);
        (assign21050_e15924, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21050_e15926;
        locals.var_t4_dn0 = assign21050_e15926_d_n0;
        locals.var_t4_dn2 = assign21050_e15926_d_n2;
        locals.var_t4_dn4 = assign21050_e15926_d_n4;
        locals.var_t4_dn5 = assign21050_e15926_d_n5;
        locals.var_t4_dn6 = assign21050_e15926_d_n6;
        locals.var_t4_dn7 = assign21050_e15926_d_n7;
        locals.var_t4_dn8 = assign21050_e15926_d_n8;
        locals.var_t4_dn9 = assign21050_e15926_d_n9;
        locals.var_t4_dn10 = assign21050_e15926_d_n10;
        locals.var_t4_dn11 = assign21050_e15926_d_n11;
        locals.var_t4_dn14 = assign21050_e15926_d_n14;
        locals.var_t4_rv = 0.0;

        let assign21060_e15933: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodervs == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard420 = assign21060_e15933;
        locals.var_guard420_rv = 0.0;

        let (assign21070_e15947, assign21070_e15947_d_n0, assign21070_e15947_d_n2, assign21070_e15947_d_n4, assign21070_e15947_d_n5, assign21070_e15947_d_n6, assign21070_e15947_d_n7, assign21070_e15947_d_n8, assign21070_e15947_d_n9, assign21070_e15947_d_n10, assign21070_e15947_d_n11, assign21070_e15947_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21070_e15941: f64 = (locals.var_vdsemodenml * locals.var_rsvde);
        let assign21070_e15944: f64 = (locals.var_vdsemodervs * locals.var_rdvde);
        let assign21070_e15945: f64 = (assign21070_e15941 + assign21070_e15944);
        (assign21070_e15945, ((locals.var_vdsemodenml * locals.var_rsvde_dn0) + (locals.var_vdsemodervs * locals.var_rdvde_dn0)), ((locals.var_vdsemodenml * locals.var_rsvde_dn2) + (locals.var_vdsemodervs * locals.var_rdvde_dn2)), ((locals.var_vdsemodenml * locals.var_rsvde_dn4) + (locals.var_vdsemodervs * locals.var_rdvde_dn4)), ((locals.var_vdsemodenml * locals.var_rsvde_dn5) + (locals.var_vdsemodervs * locals.var_rdvde_dn5)), ((locals.var_vdsemodenml * locals.var_rsvde_dn6) + (locals.var_vdsemodervs * locals.var_rdvde_dn6)), ((locals.var_vdsemodenml * locals.var_rsvde_dn7) + (locals.var_vdsemodervs * locals.var_rdvde_dn7)), ((locals.var_vdsemodenml * locals.var_rsvde_dn8) + (locals.var_vdsemodervs * locals.var_rdvde_dn8)), ((locals.var_vdsemodenml * locals.var_rsvde_dn9) + (locals.var_vdsemodervs * locals.var_rdvde_dn9)), ((locals.var_vdsemodenml * locals.var_rsvde_dn10) + (locals.var_vdsemodervs * locals.var_rdvde_dn10)), ((locals.var_vdsemodenml * locals.var_rsvde_dn11) + (locals.var_vdsemodervs * locals.var_rdvde_dn11)), ((locals.var_vdsemodenml * locals.var_rsvde_dn14) + (locals.var_vdsemodervs * locals.var_rdvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21070_e15947;
        locals.var_t0_dn0 = assign21070_e15947_d_n0;
        locals.var_t0_dn2 = assign21070_e15947_d_n2;
        locals.var_t0_dn4 = assign21070_e15947_d_n4;
        locals.var_t0_dn5 = assign21070_e15947_d_n5;
        locals.var_t0_dn6 = assign21070_e15947_d_n6;
        locals.var_t0_dn7 = assign21070_e15947_d_n7;
        locals.var_t0_dn8 = assign21070_e15947_d_n8;
        locals.var_t0_dn9 = assign21070_e15947_d_n9;
        locals.var_t0_dn10 = assign21070_e15947_d_n10;
        locals.var_t0_dn11 = assign21070_e15947_d_n11;
        locals.var_t0_dn14 = assign21070_e15947_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21080_e15961, assign21080_e15961_d_n0, assign21080_e15961_d_n2, assign21080_e15961_d_n4, assign21080_e15961_d_n5, assign21080_e15961_d_n6, assign21080_e15961_d_n7, assign21080_e15961_d_n8, assign21080_e15961_d_n9, assign21080_e15961_d_n10, assign21080_e15961_d_n11, assign21080_e15961_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21080_e15957: f64 = (2.0 * p.p262);
        let assign21080_e15958: f64 = (locals.var_t0 * assign21080_e15957);
        let assign21080_e15959: f64 = (locals.var_t4 + assign21080_e15958);
        (assign21080_e15959, (locals.var_t4_dn0 + (locals.var_t0_dn0 * assign21080_e15957)), (locals.var_t4_dn2 + (locals.var_t0_dn2 * assign21080_e15957)), (locals.var_t4_dn4 + (locals.var_t0_dn4 * assign21080_e15957)), (locals.var_t4_dn5 + (locals.var_t0_dn5 * assign21080_e15957)), (locals.var_t4_dn6 + (locals.var_t0_dn6 * assign21080_e15957)), (locals.var_t4_dn7 + (locals.var_t0_dn7 * assign21080_e15957)), (locals.var_t4_dn8 + (locals.var_t0_dn8 * assign21080_e15957)), (locals.var_t4_dn9 + (locals.var_t0_dn9 * assign21080_e15957)), (locals.var_t4_dn10 + (locals.var_t0_dn10 * assign21080_e15957)), (locals.var_t4_dn11 + (locals.var_t0_dn11 * assign21080_e15957)), (locals.var_t4_dn14 + (locals.var_t0_dn14 * assign21080_e15957)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21080_e15961;
        locals.var_t4_dn0 = assign21080_e15961_d_n0;
        locals.var_t4_dn2 = assign21080_e15961_d_n2;
        locals.var_t4_dn4 = assign21080_e15961_d_n4;
        locals.var_t4_dn5 = assign21080_e15961_d_n5;
        locals.var_t4_dn6 = assign21080_e15961_d_n6;
        locals.var_t4_dn7 = assign21080_e15961_d_n7;
        locals.var_t4_dn8 = assign21080_e15961_d_n8;
        locals.var_t4_dn9 = assign21080_e15961_d_n9;
        locals.var_t4_dn10 = assign21080_e15961_d_n10;
        locals.var_t4_dn11 = assign21080_e15961_d_n11;
        locals.var_t4_dn14 = assign21080_e15961_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21090_e15971, assign21090_e15971_d_n0, assign21090_e15971_d_n2, assign21090_e15971_d_n4, assign21090_e15971_d_n5, assign21090_e15971_d_n6, assign21090_e15971_d_n7, assign21090_e15971_d_n8, assign21090_e15971_d_n9, assign21090_e15971_d_n10, assign21090_e15971_d_n11, assign21090_e15971_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21090_e15969: f64 = (p.p292 + 1e-25);
        (assign21090_e15969, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign21090_e15971;
        locals.var_t10_dn0 = assign21090_e15971_d_n0;
        locals.var_t10_dn2 = assign21090_e15971_d_n2;
        locals.var_t10_dn4 = assign21090_e15971_d_n4;
        locals.var_t10_dn5 = assign21090_e15971_d_n5;
        locals.var_t10_dn6 = assign21090_e15971_d_n6;
        locals.var_t10_dn7 = assign21090_e15971_d_n7;
        locals.var_t10_dn8 = assign21090_e15971_d_n8;
        locals.var_t10_dn9 = assign21090_e15971_d_n9;
        locals.var_t10_dn10 = assign21090_e15971_d_n10;
        locals.var_t10_dn11 = assign21090_e15971_d_n11;
        locals.var_t10_dn14 = assign21090_e15971_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign21100_e15989, assign21100_e15989_d_n0, assign21100_e15989_d_n2, assign21100_e15989_d_n4, assign21100_e15989_d_n5, assign21100_e15989_d_n6, assign21100_e15989_d_n7, assign21100_e15989_d_n8, assign21100_e15989_d_n9, assign21100_e15989_d_n10, assign21100_e15989_d_n11, assign21100_e15989_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21100_e15983: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign21100_e15984: f64 = (1.0 - assign21100_e15983);
        let assign21100_e15985: f64 = (locals.var_uc_rdvg11 * assign21100_e15984);
        let assign21100_e15986: f64 = (1.0 + assign21100_e15985);
        let assign21100_e15987: f64 = (locals.var_t4 * assign21100_e15986);
        (assign21100_e15987, ((locals.var_t4_dn0 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21100_e15989;
        locals.var_t1_dn0 = assign21100_e15989_d_n0;
        locals.var_t1_dn2 = assign21100_e15989_d_n2;
        locals.var_t1_dn4 = assign21100_e15989_d_n4;
        locals.var_t1_dn5 = assign21100_e15989_d_n5;
        locals.var_t1_dn6 = assign21100_e15989_d_n6;
        locals.var_t1_dn7 = assign21100_e15989_d_n7;
        locals.var_t1_dn8 = assign21100_e15989_d_n8;
        locals.var_t1_dn9 = assign21100_e15989_d_n9;
        locals.var_t1_dn10 = assign21100_e15989_d_n10;
        locals.var_t1_dn11 = assign21100_e15989_d_n11;
        locals.var_t1_dn14 = assign21100_e15989_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21110_e16003, assign21110_e16003_d_n0, assign21110_e16003_d_n2, assign21110_e16003_d_n4, assign21110_e16003_d_n5, assign21110_e16003_d_n6, assign21110_e16003_d_n7, assign21110_e16003_d_n8, assign21110_e16003_d_n9, assign21110_e16003_d_n10, assign21110_e16003_d_n11, assign21110_e16003_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21110_e15997: f64 = (locals.var_t1 - locals.var_t4);
        let assign21110_e16000: f64 = (0.01 * 0.01);
        let assign21110_e16001: f64 = (assign21110_e15997 - assign21110_e16000);
        (assign21110_e16001, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21110_e16003;
        locals.var_tmf1_dn0 = assign21110_e16003_d_n0;
        locals.var_tmf1_dn2 = assign21110_e16003_d_n2;
        locals.var_tmf1_dn4 = assign21110_e16003_d_n4;
        locals.var_tmf1_dn5 = assign21110_e16003_d_n5;
        locals.var_tmf1_dn6 = assign21110_e16003_d_n6;
        locals.var_tmf1_dn7 = assign21110_e16003_d_n7;
        locals.var_tmf1_dn8 = assign21110_e16003_d_n8;
        locals.var_tmf1_dn9 = assign21110_e16003_d_n9;
        locals.var_tmf1_dn10 = assign21110_e16003_d_n10;
        locals.var_tmf1_dn11 = assign21110_e16003_d_n11;
        locals.var_tmf1_dn14 = assign21110_e16003_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21120_e16017, assign21120_e16017_d_n0, assign21120_e16017_d_n2, assign21120_e16017_d_n4, assign21120_e16017_d_n5, assign21120_e16017_d_n6, assign21120_e16017_d_n7, assign21120_e16017_d_n8, assign21120_e16017_d_n9, assign21120_e16017_d_n10, assign21120_e16017_d_n11, assign21120_e16017_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21120_e16011: f64 = (4.0 * locals.var_t4);
        let assign21120_e16014: f64 = (0.01 * 0.01);
        let assign21120_e16015: f64 = (assign21120_e16011 * assign21120_e16014);
        (assign21120_e16015, ((4.0 * locals.var_t4_dn0) * assign21120_e16014), ((4.0 * locals.var_t4_dn2) * assign21120_e16014), ((4.0 * locals.var_t4_dn4) * assign21120_e16014), ((4.0 * locals.var_t4_dn5) * assign21120_e16014), ((4.0 * locals.var_t4_dn6) * assign21120_e16014), ((4.0 * locals.var_t4_dn7) * assign21120_e16014), ((4.0 * locals.var_t4_dn8) * assign21120_e16014), ((4.0 * locals.var_t4_dn9) * assign21120_e16014), ((4.0 * locals.var_t4_dn10) * assign21120_e16014), ((4.0 * locals.var_t4_dn11) * assign21120_e16014), ((4.0 * locals.var_t4_dn14) * assign21120_e16014),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21120_e16017;
        locals.var_tmf2_dn0 = assign21120_e16017_d_n0;
        locals.var_tmf2_dn2 = assign21120_e16017_d_n2;
        locals.var_tmf2_dn4 = assign21120_e16017_d_n4;
        locals.var_tmf2_dn5 = assign21120_e16017_d_n5;
        locals.var_tmf2_dn6 = assign21120_e16017_d_n6;
        locals.var_tmf2_dn7 = assign21120_e16017_d_n7;
        locals.var_tmf2_dn8 = assign21120_e16017_d_n8;
        locals.var_tmf2_dn9 = assign21120_e16017_d_n9;
        locals.var_tmf2_dn10 = assign21120_e16017_d_n10;
        locals.var_tmf2_dn11 = assign21120_e16017_d_n11;
        locals.var_tmf2_dn14 = assign21120_e16017_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21130_e16031, assign21130_e16031_d_n0, assign21130_e16031_d_n2, assign21130_e16031_d_n4, assign21130_e16031_d_n5, assign21130_e16031_d_n6, assign21130_e16031_d_n7, assign21130_e16031_d_n8, assign21130_e16031_d_n9, assign21130_e16031_d_n10, assign21130_e16031_d_n11, assign21130_e16031_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let (assign21130_e16029, assign21130_e16029_d_n0, assign21130_e16029_d_n2, assign21130_e16029_d_n4, assign21130_e16029_d_n5, assign21130_e16029_d_n6, assign21130_e16029_d_n7, assign21130_e16029_d_n8, assign21130_e16029_d_n9, assign21130_e16029_d_n10, assign21130_e16029_d_n11, assign21130_e16029_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21130_e16028: f64 = (-locals.var_tmf2);
                (assign21130_e16028, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21130_e16029, assign21130_e16029_d_n0, assign21130_e16029_d_n2, assign21130_e16029_d_n4, assign21130_e16029_d_n5, assign21130_e16029_d_n6, assign21130_e16029_d_n7, assign21130_e16029_d_n8, assign21130_e16029_d_n9, assign21130_e16029_d_n10, assign21130_e16029_d_n11, assign21130_e16029_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21130_e16031;
        locals.var_tmf2_dn0 = assign21130_e16031_d_n0;
        locals.var_tmf2_dn2 = assign21130_e16031_d_n2;
        locals.var_tmf2_dn4 = assign21130_e16031_d_n4;
        locals.var_tmf2_dn5 = assign21130_e16031_d_n5;
        locals.var_tmf2_dn6 = assign21130_e16031_d_n6;
        locals.var_tmf2_dn7 = assign21130_e16031_d_n7;
        locals.var_tmf2_dn8 = assign21130_e16031_d_n8;
        locals.var_tmf2_dn9 = assign21130_e16031_d_n9;
        locals.var_tmf2_dn10 = assign21130_e16031_d_n10;
        locals.var_tmf2_dn11 = assign21130_e16031_d_n11;
        locals.var_tmf2_dn14 = assign21130_e16031_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21140_e16044, assign21140_e16044_d_n0, assign21140_e16044_d_n2, assign21140_e16044_d_n4, assign21140_e16044_d_n5, assign21140_e16044_d_n6, assign21140_e16044_d_n7, assign21140_e16044_d_n8, assign21140_e16044_d_n9, assign21140_e16044_d_n10, assign21140_e16044_d_n11, assign21140_e16044_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21140_e16039: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21140_e16041: f64 = (assign21140_e16039 + locals.var_tmf2);
        let assign21140_e16042: f64 = (assign21140_e16041).sqrt();
        (assign21140_e16042, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21140_e16042)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21140_e16044;
        locals.var_tmf2_dn0 = assign21140_e16044_d_n0;
        locals.var_tmf2_dn2 = assign21140_e16044_d_n2;
        locals.var_tmf2_dn4 = assign21140_e16044_d_n4;
        locals.var_tmf2_dn5 = assign21140_e16044_d_n5;
        locals.var_tmf2_dn6 = assign21140_e16044_d_n6;
        locals.var_tmf2_dn7 = assign21140_e16044_d_n7;
        locals.var_tmf2_dn8 = assign21140_e16044_d_n8;
        locals.var_tmf2_dn9 = assign21140_e16044_d_n9;
        locals.var_tmf2_dn10 = assign21140_e16044_d_n10;
        locals.var_tmf2_dn11 = assign21140_e16044_d_n11;
        locals.var_tmf2_dn14 = assign21140_e16044_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21150_e16058, assign21150_e16058_d_n0, assign21150_e16058_d_n2, assign21150_e16058_d_n4, assign21150_e16058_d_n5, assign21150_e16058_d_n6, assign21150_e16058_d_n7, assign21150_e16058_d_n8, assign21150_e16058_d_n9, assign21150_e16058_d_n10, assign21150_e16058_d_n11, assign21150_e16058_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21150_e16054: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21150_e16055: f64 = (1.0 + assign21150_e16054);
        let assign21150_e16056: f64 = (0.5 * assign21150_e16055);
        (assign21150_e16056, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21150_e16058;
        locals.var_t0_dn0 = assign21150_e16058_d_n0;
        locals.var_t0_dn2 = assign21150_e16058_d_n2;
        locals.var_t0_dn4 = assign21150_e16058_d_n4;
        locals.var_t0_dn5 = assign21150_e16058_d_n5;
        locals.var_t0_dn6 = assign21150_e16058_d_n6;
        locals.var_t0_dn7 = assign21150_e16058_d_n7;
        locals.var_t0_dn8 = assign21150_e16058_d_n8;
        locals.var_t0_dn9 = assign21150_e16058_d_n9;
        locals.var_t0_dn10 = assign21150_e16058_d_n10;
        locals.var_t0_dn11 = assign21150_e16058_d_n11;
        locals.var_t0_dn14 = assign21150_e16058_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21160_e16078, assign21160_e16078_d_n0, assign21160_e16078_d_n2, assign21160_e16078_d_n4, assign21160_e16078_d_n5, assign21160_e16078_d_n6, assign21160_e16078_d_n7, assign21160_e16078_d_n8, assign21160_e16078_d_n9, assign21160_e16078_d_n10, assign21160_e16078_d_n11, assign21160_e16078_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21160_e16069: f64 = (2.0 * 0.01);
        let assign21160_e16071: f64 = (assign21160_e16069 * 0.01);
        let assign21160_e16072: f64 = (locals.var_tmf1 - assign21160_e16071);
        let assign21160_e16074: f64 = (assign21160_e16072 / locals.var_tmf2);
        let assign21160_e16075: f64 = (1.0 - assign21160_e16074);
        let assign21160_e16076: f64 = (0.5 * assign21160_e16075);
        (assign21160_e16076, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21160_e16078;
        locals.var_t5_dn0 = assign21160_e16078_d_n0;
        locals.var_t5_dn2 = assign21160_e16078_d_n2;
        locals.var_t5_dn4 = assign21160_e16078_d_n4;
        locals.var_t5_dn5 = assign21160_e16078_d_n5;
        locals.var_t5_dn6 = assign21160_e16078_d_n6;
        locals.var_t5_dn7 = assign21160_e16078_d_n7;
        locals.var_t5_dn8 = assign21160_e16078_d_n8;
        locals.var_t5_dn9 = assign21160_e16078_d_n9;
        locals.var_t5_dn10 = assign21160_e16078_d_n10;
        locals.var_t5_dn11 = assign21160_e16078_d_n11;
        locals.var_t5_dn14 = assign21160_e16078_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21170_e16092, assign21170_e16092_d_n0, assign21170_e16092_d_n2, assign21170_e16092_d_n4, assign21170_e16092_d_n5, assign21170_e16092_d_n6, assign21170_e16092_d_n7, assign21170_e16092_d_n8, assign21170_e16092_d_n9, assign21170_e16092_d_n10, assign21170_e16092_d_n11, assign21170_e16092_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21170_e16088: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21170_e16089: f64 = (0.5 * assign21170_e16088);
        let assign21170_e16090: f64 = (locals.var_t4 + assign21170_e16089);
        (assign21170_e16090, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21170_e16092;
        locals.var_t2_dn0 = assign21170_e16092_d_n0;
        locals.var_t2_dn2 = assign21170_e16092_d_n2;
        locals.var_t2_dn4 = assign21170_e16092_d_n4;
        locals.var_t2_dn5 = assign21170_e16092_d_n5;
        locals.var_t2_dn6 = assign21170_e16092_d_n6;
        locals.var_t2_dn7 = assign21170_e16092_d_n7;
        locals.var_t2_dn8 = assign21170_e16092_d_n8;
        locals.var_t2_dn9 = assign21170_e16092_d_n9;
        locals.var_t2_dn10 = assign21170_e16092_d_n10;
        locals.var_t2_dn11 = assign21170_e16092_d_n11;
        locals.var_t2_dn14 = assign21170_e16092_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21180_e16104, assign21180_e16104_d_n0, assign21180_e16104_d_n2, assign21180_e16104_d_n4, assign21180_e16104_d_n5, assign21180_e16104_d_n6, assign21180_e16104_d_n7, assign21180_e16104_d_n8, assign21180_e16104_d_n9, assign21180_e16104_d_n10, assign21180_e16104_d_n11, assign21180_e16104_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21180_e16101: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign21180_e16102: f64 = (locals.var_t4 * assign21180_e16101);
        (assign21180_e16102, (locals.var_t4_dn0 * assign21180_e16101), (locals.var_t4_dn2 * assign21180_e16101), (locals.var_t4_dn4 * assign21180_e16101), (locals.var_t4_dn5 * assign21180_e16101), (locals.var_t4_dn6 * assign21180_e16101), (locals.var_t4_dn7 * assign21180_e16101), (locals.var_t4_dn8 * assign21180_e16101), (locals.var_t4_dn9 * assign21180_e16101), (locals.var_t4_dn10 * assign21180_e16101), (locals.var_t4_dn11 * assign21180_e16101), (locals.var_t4_dn14 * assign21180_e16101),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21180_e16104;
        locals.var_t3_dn0 = assign21180_e16104_d_n0;
        locals.var_t3_dn2 = assign21180_e16104_d_n2;
        locals.var_t3_dn4 = assign21180_e16104_d_n4;
        locals.var_t3_dn5 = assign21180_e16104_d_n5;
        locals.var_t3_dn6 = assign21180_e16104_d_n6;
        locals.var_t3_dn7 = assign21180_e16104_d_n7;
        locals.var_t3_dn8 = assign21180_e16104_d_n8;
        locals.var_t3_dn9 = assign21180_e16104_d_n9;
        locals.var_t3_dn10 = assign21180_e16104_d_n10;
        locals.var_t3_dn11 = assign21180_e16104_d_n11;
        locals.var_t3_dn14 = assign21180_e16104_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21190_e16118, assign21190_e16118_d_n0, assign21190_e16118_d_n2, assign21190_e16118_d_n4, assign21190_e16118_d_n5, assign21190_e16118_d_n6, assign21190_e16118_d_n7, assign21190_e16118_d_n8, assign21190_e16118_d_n9, assign21190_e16118_d_n10, assign21190_e16118_d_n11, assign21190_e16118_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21190_e16112: f64 = (locals.var_t3 - locals.var_t2);
        let assign21190_e16115: f64 = (5e-5 * 0.01);
        let assign21190_e16116: f64 = (assign21190_e16112 - assign21190_e16115);
        (assign21190_e16116, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21190_e16118;
        locals.var_tmf1_dn0 = assign21190_e16118_d_n0;
        locals.var_tmf1_dn2 = assign21190_e16118_d_n2;
        locals.var_tmf1_dn4 = assign21190_e16118_d_n4;
        locals.var_tmf1_dn5 = assign21190_e16118_d_n5;
        locals.var_tmf1_dn6 = assign21190_e16118_d_n6;
        locals.var_tmf1_dn7 = assign21190_e16118_d_n7;
        locals.var_tmf1_dn8 = assign21190_e16118_d_n8;
        locals.var_tmf1_dn9 = assign21190_e16118_d_n9;
        locals.var_tmf1_dn10 = assign21190_e16118_d_n10;
        locals.var_tmf1_dn11 = assign21190_e16118_d_n11;
        locals.var_tmf1_dn14 = assign21190_e16118_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21200_e16132, assign21200_e16132_d_n0, assign21200_e16132_d_n2, assign21200_e16132_d_n4, assign21200_e16132_d_n5, assign21200_e16132_d_n6, assign21200_e16132_d_n7, assign21200_e16132_d_n8, assign21200_e16132_d_n9, assign21200_e16132_d_n10, assign21200_e16132_d_n11, assign21200_e16132_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21200_e16126: f64 = (4.0 * locals.var_t3);
        let assign21200_e16129: f64 = (5e-5 * 0.01);
        let assign21200_e16130: f64 = (assign21200_e16126 * assign21200_e16129);
        (assign21200_e16130, ((4.0 * locals.var_t3_dn0) * assign21200_e16129), ((4.0 * locals.var_t3_dn2) * assign21200_e16129), ((4.0 * locals.var_t3_dn4) * assign21200_e16129), ((4.0 * locals.var_t3_dn5) * assign21200_e16129), ((4.0 * locals.var_t3_dn6) * assign21200_e16129), ((4.0 * locals.var_t3_dn7) * assign21200_e16129), ((4.0 * locals.var_t3_dn8) * assign21200_e16129), ((4.0 * locals.var_t3_dn9) * assign21200_e16129), ((4.0 * locals.var_t3_dn10) * assign21200_e16129), ((4.0 * locals.var_t3_dn11) * assign21200_e16129), ((4.0 * locals.var_t3_dn14) * assign21200_e16129),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21200_e16132;
        locals.var_tmf2_dn0 = assign21200_e16132_d_n0;
        locals.var_tmf2_dn2 = assign21200_e16132_d_n2;
        locals.var_tmf2_dn4 = assign21200_e16132_d_n4;
        locals.var_tmf2_dn5 = assign21200_e16132_d_n5;
        locals.var_tmf2_dn6 = assign21200_e16132_d_n6;
        locals.var_tmf2_dn7 = assign21200_e16132_d_n7;
        locals.var_tmf2_dn8 = assign21200_e16132_d_n8;
        locals.var_tmf2_dn9 = assign21200_e16132_d_n9;
        locals.var_tmf2_dn10 = assign21200_e16132_d_n10;
        locals.var_tmf2_dn11 = assign21200_e16132_d_n11;
        locals.var_tmf2_dn14 = assign21200_e16132_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21210_e16146, assign21210_e16146_d_n0, assign21210_e16146_d_n2, assign21210_e16146_d_n4, assign21210_e16146_d_n5, assign21210_e16146_d_n6, assign21210_e16146_d_n7, assign21210_e16146_d_n8, assign21210_e16146_d_n9, assign21210_e16146_d_n10, assign21210_e16146_d_n11, assign21210_e16146_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let (assign21210_e16144, assign21210_e16144_d_n0, assign21210_e16144_d_n2, assign21210_e16144_d_n4, assign21210_e16144_d_n5, assign21210_e16144_d_n6, assign21210_e16144_d_n7, assign21210_e16144_d_n8, assign21210_e16144_d_n9, assign21210_e16144_d_n10, assign21210_e16144_d_n11, assign21210_e16144_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21210_e16143: f64 = (-locals.var_tmf2);
                (assign21210_e16143, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21210_e16144, assign21210_e16144_d_n0, assign21210_e16144_d_n2, assign21210_e16144_d_n4, assign21210_e16144_d_n5, assign21210_e16144_d_n6, assign21210_e16144_d_n7, assign21210_e16144_d_n8, assign21210_e16144_d_n9, assign21210_e16144_d_n10, assign21210_e16144_d_n11, assign21210_e16144_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21210_e16146;
        locals.var_tmf2_dn0 = assign21210_e16146_d_n0;
        locals.var_tmf2_dn2 = assign21210_e16146_d_n2;
        locals.var_tmf2_dn4 = assign21210_e16146_d_n4;
        locals.var_tmf2_dn5 = assign21210_e16146_d_n5;
        locals.var_tmf2_dn6 = assign21210_e16146_d_n6;
        locals.var_tmf2_dn7 = assign21210_e16146_d_n7;
        locals.var_tmf2_dn8 = assign21210_e16146_d_n8;
        locals.var_tmf2_dn9 = assign21210_e16146_d_n9;
        locals.var_tmf2_dn10 = assign21210_e16146_d_n10;
        locals.var_tmf2_dn11 = assign21210_e16146_d_n11;
        locals.var_tmf2_dn14 = assign21210_e16146_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21220_e16159, assign21220_e16159_d_n0, assign21220_e16159_d_n2, assign21220_e16159_d_n4, assign21220_e16159_d_n5, assign21220_e16159_d_n6, assign21220_e16159_d_n7, assign21220_e16159_d_n8, assign21220_e16159_d_n9, assign21220_e16159_d_n10, assign21220_e16159_d_n11, assign21220_e16159_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21220_e16154: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21220_e16156: f64 = (assign21220_e16154 + locals.var_tmf2);
        let assign21220_e16157: f64 = (assign21220_e16156).sqrt();
        (assign21220_e16157, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21220_e16157)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21220_e16159;
        locals.var_tmf2_dn0 = assign21220_e16159_d_n0;
        locals.var_tmf2_dn2 = assign21220_e16159_d_n2;
        locals.var_tmf2_dn4 = assign21220_e16159_d_n4;
        locals.var_tmf2_dn5 = assign21220_e16159_d_n5;
        locals.var_tmf2_dn6 = assign21220_e16159_d_n6;
        locals.var_tmf2_dn7 = assign21220_e16159_d_n7;
        locals.var_tmf2_dn8 = assign21220_e16159_d_n8;
        locals.var_tmf2_dn9 = assign21220_e16159_d_n9;
        locals.var_tmf2_dn10 = assign21220_e16159_d_n10;
        locals.var_tmf2_dn11 = assign21220_e16159_d_n11;
        locals.var_tmf2_dn14 = assign21220_e16159_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21230_e16173, assign21230_e16173_d_n0, assign21230_e16173_d_n2, assign21230_e16173_d_n4, assign21230_e16173_d_n5, assign21230_e16173_d_n6, assign21230_e16173_d_n7, assign21230_e16173_d_n8, assign21230_e16173_d_n9, assign21230_e16173_d_n10, assign21230_e16173_d_n11, assign21230_e16173_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21230_e16169: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21230_e16170: f64 = (1.0 + assign21230_e16169);
        let assign21230_e16171: f64 = (0.5 * assign21230_e16170);
        (assign21230_e16171, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21230_e16173;
        locals.var_t0_dn0 = assign21230_e16173_d_n0;
        locals.var_t0_dn2 = assign21230_e16173_d_n2;
        locals.var_t0_dn4 = assign21230_e16173_d_n4;
        locals.var_t0_dn5 = assign21230_e16173_d_n5;
        locals.var_t0_dn6 = assign21230_e16173_d_n6;
        locals.var_t0_dn7 = assign21230_e16173_d_n7;
        locals.var_t0_dn8 = assign21230_e16173_d_n8;
        locals.var_t0_dn9 = assign21230_e16173_d_n9;
        locals.var_t0_dn10 = assign21230_e16173_d_n10;
        locals.var_t0_dn11 = assign21230_e16173_d_n11;
        locals.var_t0_dn14 = assign21230_e16173_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21240_e16193, assign21240_e16193_d_n0, assign21240_e16193_d_n2, assign21240_e16193_d_n4, assign21240_e16193_d_n5, assign21240_e16193_d_n6, assign21240_e16193_d_n7, assign21240_e16193_d_n8, assign21240_e16193_d_n9, assign21240_e16193_d_n10, assign21240_e16193_d_n11, assign21240_e16193_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21240_e16184: f64 = (2.0 * 5e-5);
        let assign21240_e16186: f64 = (assign21240_e16184 * 0.01);
        let assign21240_e16187: f64 = (locals.var_tmf1 + assign21240_e16186);
        let assign21240_e16189: f64 = (assign21240_e16187 / locals.var_tmf2);
        let assign21240_e16190: f64 = (1.0 - assign21240_e16189);
        let assign21240_e16191: f64 = (0.5 * assign21240_e16190);
        (assign21240_e16191, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21240_e16193;
        locals.var_t5_dn0 = assign21240_e16193_d_n0;
        locals.var_t5_dn2 = assign21240_e16193_d_n2;
        locals.var_t5_dn4 = assign21240_e16193_d_n4;
        locals.var_t5_dn5 = assign21240_e16193_d_n5;
        locals.var_t5_dn6 = assign21240_e16193_d_n6;
        locals.var_t5_dn7 = assign21240_e16193_d_n7;
        locals.var_t5_dn8 = assign21240_e16193_d_n8;
        locals.var_t5_dn9 = assign21240_e16193_d_n9;
        locals.var_t5_dn10 = assign21240_e16193_d_n10;
        locals.var_t5_dn11 = assign21240_e16193_d_n11;
        locals.var_t5_dn14 = assign21240_e16193_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21250_e16207, assign21250_e16207_d_n0, assign21250_e16207_d_n2, assign21250_e16207_d_n4, assign21250_e16207_d_n5, assign21250_e16207_d_n6, assign21250_e16207_d_n7, assign21250_e16207_d_n8, assign21250_e16207_d_n9, assign21250_e16207_d_n10, assign21250_e16207_d_n11, assign21250_e16207_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21250_e16203: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21250_e16204: f64 = (0.5 * assign21250_e16203);
        let assign21250_e16205: f64 = (locals.var_t3 - assign21250_e16204);
        (assign21250_e16205, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21250_e16207;
        locals.var_rsdrift_dn0 = assign21250_e16207_d_n0;
        locals.var_rsdrift_dn2 = assign21250_e16207_d_n2;
        locals.var_rsdrift_dn4 = assign21250_e16207_d_n4;
        locals.var_rsdrift_dn5 = assign21250_e16207_d_n5;
        locals.var_rsdrift_dn6 = assign21250_e16207_d_n6;
        locals.var_rsdrift_dn7 = assign21250_e16207_d_n7;
        locals.var_rsdrift_dn8 = assign21250_e16207_d_n8;
        locals.var_rsdrift_dn9 = assign21250_e16207_d_n9;
        locals.var_rsdrift_dn10 = assign21250_e16207_d_n10;
        locals.var_rsdrift_dn11 = assign21250_e16207_d_n11;
        locals.var_rsdrift_dn14 = assign21250_e16207_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21260_e16219, assign21260_e16219_d_n0, assign21260_e16219_d_n2, assign21260_e16219_d_n4, assign21260_e16219_d_n5, assign21260_e16219_d_n6, assign21260_e16219_d_n7, assign21260_e16219_d_n8, assign21260_e16219_d_n9, assign21260_e16219_d_n10, assign21260_e16219_d_n11, assign21260_e16219_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21260_e16216: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign21260_e16217: f64 = (1.0 - assign21260_e16216);
        (assign21260_e16217, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21260_e16219;
        locals.var_t1_dn0 = assign21260_e16219_d_n0;
        locals.var_t1_dn2 = assign21260_e16219_d_n2;
        locals.var_t1_dn4 = assign21260_e16219_d_n4;
        locals.var_t1_dn5 = assign21260_e16219_d_n5;
        locals.var_t1_dn6 = assign21260_e16219_d_n6;
        locals.var_t1_dn7 = assign21260_e16219_d_n7;
        locals.var_t1_dn8 = assign21260_e16219_d_n8;
        locals.var_t1_dn9 = assign21260_e16219_d_n9;
        locals.var_t1_dn10 = assign21260_e16219_d_n10;
        locals.var_t1_dn11 = assign21260_e16219_d_n11;
        locals.var_t1_dn14 = assign21260_e16219_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21270_e16240, assign21270_e16240_d_n0, assign21270_e16240_d_n2, assign21270_e16240_d_n4, assign21270_e16240_d_n5, assign21270_e16240_d_n6, assign21270_e16240_d_n7, assign21270_e16240_d_n8, assign21270_e16240_d_n9, assign21270_e16240_d_n10, assign21270_e16240_d_n11, assign21270_e16240_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21270_e16227: f64 = (locals.var_t1 * locals.var_t1);
        let assign21270_e16231: f64 = (0.0001 * 0.01);
        let assign21270_e16232: f64 = (4.0 * assign21270_e16231);
        let assign21270_e16235: f64 = (0.0001 * 0.01);
        let assign21270_e16236: f64 = (assign21270_e16232 * assign21270_e16235);
        let assign21270_e16237: f64 = (assign21270_e16227 + assign21270_e16236);
        let assign21270_e16238: f64 = (assign21270_e16237).sqrt();
        (assign21270_e16238, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign21270_e16238)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21270_e16240;
        locals.var_tmf2_dn0 = assign21270_e16240_d_n0;
        locals.var_tmf2_dn2 = assign21270_e16240_d_n2;
        locals.var_tmf2_dn4 = assign21270_e16240_d_n4;
        locals.var_tmf2_dn5 = assign21270_e16240_d_n5;
        locals.var_tmf2_dn6 = assign21270_e16240_d_n6;
        locals.var_tmf2_dn7 = assign21270_e16240_d_n7;
        locals.var_tmf2_dn8 = assign21270_e16240_d_n8;
        locals.var_tmf2_dn9 = assign21270_e16240_d_n9;
        locals.var_tmf2_dn10 = assign21270_e16240_d_n10;
        locals.var_tmf2_dn11 = assign21270_e16240_d_n11;
        locals.var_tmf2_dn14 = assign21270_e16240_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21280_e16254, assign21280_e16254_d_n0, assign21280_e16254_d_n2, assign21280_e16254_d_n4, assign21280_e16254_d_n5, assign21280_e16254_d_n6, assign21280_e16254_d_n7, assign21280_e16254_d_n8, assign21280_e16254_d_n9, assign21280_e16254_d_n10, assign21280_e16254_d_n11, assign21280_e16254_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21280_e16250: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign21280_e16251: f64 = (1.0 + assign21280_e16250);
        let assign21280_e16252: f64 = (0.5 * assign21280_e16251);
        (assign21280_e16252, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21280_e16254;
        locals.var_t4_dn0 = assign21280_e16254_d_n0;
        locals.var_t4_dn2 = assign21280_e16254_d_n2;
        locals.var_t4_dn4 = assign21280_e16254_d_n4;
        locals.var_t4_dn5 = assign21280_e16254_d_n5;
        locals.var_t4_dn6 = assign21280_e16254_d_n6;
        locals.var_t4_dn7 = assign21280_e16254_d_n7;
        locals.var_t4_dn8 = assign21280_e16254_d_n8;
        locals.var_t4_dn9 = assign21280_e16254_d_n9;
        locals.var_t4_dn10 = assign21280_e16254_d_n10;
        locals.var_t4_dn11 = assign21280_e16254_d_n11;
        locals.var_t4_dn14 = assign21280_e16254_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21290_e16266, assign21290_e16266_d_n0, assign21290_e16266_d_n2, assign21290_e16266_d_n4, assign21290_e16266_d_n5, assign21290_e16266_d_n6, assign21290_e16266_d_n7, assign21290_e16266_d_n8, assign21290_e16266_d_n9, assign21290_e16266_d_n10, assign21290_e16266_d_n11, assign21290_e16266_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21290_e16263: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign21290_e16264: f64 = (0.5 * assign21290_e16263);
        (assign21290_e16264, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21290_e16266;
        locals.var_t3_dn0 = assign21290_e16266_d_n0;
        locals.var_t3_dn2 = assign21290_e16266_d_n2;
        locals.var_t3_dn4 = assign21290_e16266_d_n4;
        locals.var_t3_dn5 = assign21290_e16266_d_n5;
        locals.var_t3_dn6 = assign21290_e16266_d_n6;
        locals.var_t3_dn7 = assign21290_e16266_d_n7;
        locals.var_t3_dn8 = assign21290_e16266_d_n8;
        locals.var_t3_dn9 = assign21290_e16266_d_n9;
        locals.var_t3_dn10 = assign21290_e16266_d_n10;
        locals.var_t3_dn11 = assign21290_e16266_d_n11;
        locals.var_t3_dn14 = assign21290_e16266_d_n14;
        locals.var_t3_rv = 0.0;

        let assign21300_e16269: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign21300_e16269;
        locals.var_guard421_rv = 0.0;

        let (assign21310_e16279, assign21310_e16279_d_n0, assign21310_e16279_d_n2, assign21310_e16279_d_n4, assign21310_e16279_d_n5, assign21310_e16279_d_n6, assign21310_e16279_d_n7, assign21310_e16279_d_n8, assign21310_e16279_d_n9, assign21310_e16279_d_n10, assign21310_e16279_d_n11, assign21310_e16279_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21310_e16279;
        locals.var_t3_dn0 = assign21310_e16279_d_n0;
        locals.var_t3_dn2 = assign21310_e16279_d_n2;
        locals.var_t3_dn4 = assign21310_e16279_d_n4;
        locals.var_t3_dn5 = assign21310_e16279_d_n5;
        locals.var_t3_dn6 = assign21310_e16279_d_n6;
        locals.var_t3_dn7 = assign21310_e16279_d_n7;
        locals.var_t3_dn8 = assign21310_e16279_d_n8;
        locals.var_t3_dn9 = assign21310_e16279_d_n9;
        locals.var_t3_dn10 = assign21310_e16279_d_n10;
        locals.var_t3_dn11 = assign21310_e16279_d_n11;
        locals.var_t3_dn14 = assign21310_e16279_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21320_e16289, assign21320_e16289_d_n0, assign21320_e16289_d_n2, assign21320_e16289_d_n4, assign21320_e16289_d_n5, assign21320_e16289_d_n6, assign21320_e16289_d_n7, assign21320_e16289_d_n8, assign21320_e16289_d_n9, assign21320_e16289_d_n10, assign21320_e16289_d_n11, assign21320_e16289_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21320_e16289;
        locals.var_t4_dn0 = assign21320_e16289_d_n0;
        locals.var_t4_dn2 = assign21320_e16289_d_n2;
        locals.var_t4_dn4 = assign21320_e16289_d_n4;
        locals.var_t4_dn5 = assign21320_e16289_d_n5;
        locals.var_t4_dn6 = assign21320_e16289_d_n6;
        locals.var_t4_dn7 = assign21320_e16289_d_n7;
        locals.var_t4_dn8 = assign21320_e16289_d_n8;
        locals.var_t4_dn9 = assign21320_e16289_d_n9;
        locals.var_t4_dn10 = assign21320_e16289_d_n10;
        locals.var_t4_dn11 = assign21320_e16289_d_n11;
        locals.var_t4_dn14 = assign21320_e16289_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21330_e16299, assign21330_e16299_d_n0, assign21330_e16299_d_n2, assign21330_e16299_d_n4, assign21330_e16299_d_n5, assign21330_e16299_d_n6, assign21330_e16299_d_n7, assign21330_e16299_d_n8, assign21330_e16299_d_n9, assign21330_e16299_d_n10, assign21330_e16299_d_n11, assign21330_e16299_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21330_e16297: f64 = (locals.var_t3 + 1e-25);
        (assign21330_e16297, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21330_e16299;
        locals.var_t3_dn0 = assign21330_e16299_d_n0;
        locals.var_t3_dn2 = assign21330_e16299_d_n2;
        locals.var_t3_dn4 = assign21330_e16299_d_n4;
        locals.var_t3_dn5 = assign21330_e16299_d_n5;
        locals.var_t3_dn6 = assign21330_e16299_d_n6;
        locals.var_t3_dn7 = assign21330_e16299_d_n7;
        locals.var_t3_dn8 = assign21330_e16299_d_n8;
        locals.var_t3_dn9 = assign21330_e16299_d_n9;
        locals.var_t3_dn10 = assign21330_e16299_d_n10;
        locals.var_t3_dn11 = assign21330_e16299_d_n11;
        locals.var_t3_dn14 = assign21330_e16299_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21340_e16307, assign21340_e16307_d_n0, assign21340_e16307_d_n2, assign21340_e16307_d_n4, assign21340_e16307_d_n5, assign21340_e16307_d_n6, assign21340_e16307_d_n7, assign21340_e16307_d_n8, assign21340_e16307_d_n9, assign21340_e16307_d_n10, assign21340_e16307_d_n11, assign21340_e16307_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21340_e16307;
        locals.var_t0_dn0 = assign21340_e16307_d_n0;
        locals.var_t0_dn2 = assign21340_e16307_d_n2;
        locals.var_t0_dn4 = assign21340_e16307_d_n4;
        locals.var_t0_dn5 = assign21340_e16307_d_n5;
        locals.var_t0_dn6 = assign21340_e16307_d_n6;
        locals.var_t0_dn7 = assign21340_e16307_d_n7;
        locals.var_t0_dn8 = assign21340_e16307_d_n8;
        locals.var_t0_dn9 = assign21340_e16307_d_n9;
        locals.var_t0_dn10 = assign21340_e16307_d_n10;
        locals.var_t0_dn11 = assign21340_e16307_d_n11;
        locals.var_t0_dn14 = assign21340_e16307_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21350_e16317, assign21350_e16317_d_n0, assign21350_e16317_d_n2, assign21350_e16317_d_n4, assign21350_e16317_d_n5, assign21350_e16317_d_n6, assign21350_e16317_d_n7, assign21350_e16317_d_n8, assign21350_e16317_d_n9, assign21350_e16317_d_n10, assign21350_e16317_d_n11, assign21350_e16317_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21350_e16315: f64 = (locals.var_rsdrift * locals.var_t3);
        (assign21350_e16315, ((locals.var_rsdrift_dn0 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21350_e16317;
        locals.var_rsdrift_dn0 = assign21350_e16317_d_n0;
        locals.var_rsdrift_dn2 = assign21350_e16317_d_n2;
        locals.var_rsdrift_dn4 = assign21350_e16317_d_n4;
        locals.var_rsdrift_dn5 = assign21350_e16317_d_n5;
        locals.var_rsdrift_dn6 = assign21350_e16317_d_n6;
        locals.var_rsdrift_dn7 = assign21350_e16317_d_n7;
        locals.var_rsdrift_dn8 = assign21350_e16317_d_n8;
        locals.var_rsdrift_dn9 = assign21350_e16317_d_n9;
        locals.var_rsdrift_dn10 = assign21350_e16317_d_n10;
        locals.var_rsdrift_dn11 = assign21350_e16317_d_n11;
        locals.var_rsdrift_dn14 = assign21350_e16317_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21360_e16326, assign21360_e16326_d_n0, assign21360_e16326_d_n2, assign21360_e16326_d_n4, assign21360_e16326_d_n5, assign21360_e16326_d_n6, assign21360_e16326_d_n7, assign21360_e16326_d_n8, assign21360_e16326_d_n9, assign21360_e16326_d_n10, assign21360_e16326_d_n11, assign21360_e16326_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21360_e16326;
        locals.var_rsdrift_dn0 = assign21360_e16326_d_n0;
        locals.var_rsdrift_dn2 = assign21360_e16326_d_n2;
        locals.var_rsdrift_dn4 = assign21360_e16326_d_n4;
        locals.var_rsdrift_dn5 = assign21360_e16326_d_n5;
        locals.var_rsdrift_dn6 = assign21360_e16326_d_n6;
        locals.var_rsdrift_dn7 = assign21360_e16326_d_n7;
        locals.var_rsdrift_dn8 = assign21360_e16326_d_n8;
        locals.var_rsdrift_dn9 = assign21360_e16326_d_n9;
        locals.var_rsdrift_dn10 = assign21360_e16326_d_n10;
        locals.var_rsdrift_dn11 = assign21360_e16326_d_n11;
        locals.var_rsdrift_dn14 = assign21360_e16326_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let assign21370_e16337: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21370_e16338: f64 = (locals.var_uc_nover * assign21370_e16337);
        let assign21370_e16341: f64 = if (((p.p54 == 1.0) && (p.p34 == 0.0)) && (assign21370_e16338 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard422 = assign21370_e16341;
        locals.var_guard422_rv = 0.0;

        let (assign21380_e16357, assign21380_e16357_d_n0, assign21380_e16357_d_n2, assign21380_e16357_d_n4, assign21380_e16357_d_n5, assign21380_e16357_d_n6, assign21380_e16357_d_n7, assign21380_e16357_d_n8, assign21380_e16357_d_n9, assign21380_e16357_d_n10, assign21380_e16357_d_n11, assign21380_e16357_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21380_e16350: f64 = (p.p333 * locals.var_vdserevz);
        let assign21380_e16351: f64 = (p.p335 - assign21380_e16350);
        let assign21380_e16354: f64 = (p.p332 * locals.var_vsubsrev);
        let assign21380_e16355: f64 = (assign21380_e16351 - assign21380_e16354);
        (assign21380_e16355, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21380_e16357;
        locals.var_t0_dn0 = assign21380_e16357_d_n0;
        locals.var_t0_dn2 = assign21380_e16357_d_n2;
        locals.var_t0_dn4 = assign21380_e16357_d_n4;
        locals.var_t0_dn5 = assign21380_e16357_d_n5;
        locals.var_t0_dn6 = assign21380_e16357_d_n6;
        locals.var_t0_dn7 = assign21380_e16357_d_n7;
        locals.var_t0_dn8 = assign21380_e16357_d_n8;
        locals.var_t0_dn9 = assign21380_e16357_d_n9;
        locals.var_t0_dn10 = assign21380_e16357_d_n10;
        locals.var_t0_dn11 = assign21380_e16357_d_n11;
        locals.var_t0_dn14 = assign21380_e16357_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21390_e16374, assign21390_e16374_d_n0, assign21390_e16374_d_n2, assign21390_e16374_d_n4, assign21390_e16374_d_n5, assign21390_e16374_d_n6, assign21390_e16374_d_n7, assign21390_e16374_d_n8, assign21390_e16374_d_n9, assign21390_e16374_d_n10, assign21390_e16374_d_n11, assign21390_e16374_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21390_e16365: f64 = (locals.var_t0 * locals.var_t0);
        let assign21390_e16368: f64 = (4.0 * 10.0);
        let assign21390_e16370: f64 = (assign21390_e16368 * 10.0);
        let assign21390_e16371: f64 = (assign21390_e16365 + assign21390_e16370);
        let assign21390_e16372: f64 = (assign21390_e16371).sqrt();
        (assign21390_e16372, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign21390_e16372)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21390_e16374;
        locals.var_tmf2_dn0 = assign21390_e16374_d_n0;
        locals.var_tmf2_dn2 = assign21390_e16374_d_n2;
        locals.var_tmf2_dn4 = assign21390_e16374_d_n4;
        locals.var_tmf2_dn5 = assign21390_e16374_d_n5;
        locals.var_tmf2_dn6 = assign21390_e16374_d_n6;
        locals.var_tmf2_dn7 = assign21390_e16374_d_n7;
        locals.var_tmf2_dn8 = assign21390_e16374_d_n8;
        locals.var_tmf2_dn9 = assign21390_e16374_d_n9;
        locals.var_tmf2_dn10 = assign21390_e16374_d_n10;
        locals.var_tmf2_dn11 = assign21390_e16374_d_n11;
        locals.var_tmf2_dn14 = assign21390_e16374_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21400_e16388, assign21400_e16388_d_n0, assign21400_e16388_d_n2, assign21400_e16388_d_n4, assign21400_e16388_d_n5, assign21400_e16388_d_n6, assign21400_e16388_d_n7, assign21400_e16388_d_n8, assign21400_e16388_d_n9, assign21400_e16388_d_n10, assign21400_e16388_d_n11, assign21400_e16388_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21400_e16384: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign21400_e16385: f64 = (1.0 + assign21400_e16384);
        let assign21400_e16386: f64 = (0.5 * assign21400_e16385);
        (assign21400_e16386, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21400_e16388;
        locals.var_t2_dn0 = assign21400_e16388_d_n0;
        locals.var_t2_dn2 = assign21400_e16388_d_n2;
        locals.var_t2_dn4 = assign21400_e16388_d_n4;
        locals.var_t2_dn5 = assign21400_e16388_d_n5;
        locals.var_t2_dn6 = assign21400_e16388_d_n6;
        locals.var_t2_dn7 = assign21400_e16388_d_n7;
        locals.var_t2_dn8 = assign21400_e16388_d_n8;
        locals.var_t2_dn9 = assign21400_e16388_d_n9;
        locals.var_t2_dn10 = assign21400_e16388_d_n10;
        locals.var_t2_dn11 = assign21400_e16388_d_n11;
        locals.var_t2_dn14 = assign21400_e16388_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21410_e16400, assign21410_e16400_d_n0, assign21410_e16400_d_n2, assign21410_e16400_d_n4, assign21410_e16400_d_n5, assign21410_e16400_d_n6, assign21410_e16400_d_n7, assign21410_e16400_d_n8, assign21410_e16400_d_n9, assign21410_e16400_d_n10, assign21410_e16400_d_n11, assign21410_e16400_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21410_e16397: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign21410_e16398: f64 = (0.5 * assign21410_e16397);
        (assign21410_e16398, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21410_e16400;
        locals.var_t1_dn0 = assign21410_e16400_d_n0;
        locals.var_t1_dn2 = assign21410_e16400_d_n2;
        locals.var_t1_dn4 = assign21410_e16400_d_n4;
        locals.var_t1_dn5 = assign21410_e16400_d_n5;
        locals.var_t1_dn6 = assign21410_e16400_d_n6;
        locals.var_t1_dn7 = assign21410_e16400_d_n7;
        locals.var_t1_dn8 = assign21410_e16400_d_n8;
        locals.var_t1_dn9 = assign21410_e16400_d_n9;
        locals.var_t1_dn10 = assign21410_e16400_d_n10;
        locals.var_t1_dn11 = assign21410_e16400_d_n11;
        locals.var_t1_dn14 = assign21410_e16400_d_n14;
        locals.var_t1_rv = 0.0;

        let assign21420_e16403: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign21420_e16403;
        locals.var_guard423_rv = 0.0;

        let (assign21430_e16413, assign21430_e16413_d_n0, assign21430_e16413_d_n2, assign21430_e16413_d_n4, assign21430_e16413_d_n5, assign21430_e16413_d_n6, assign21430_e16413_d_n7, assign21430_e16413_d_n8, assign21430_e16413_d_n9, assign21430_e16413_d_n10, assign21430_e16413_d_n11, assign21430_e16413_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21430_e16413;
        locals.var_t1_dn0 = assign21430_e16413_d_n0;
        locals.var_t1_dn2 = assign21430_e16413_d_n2;
        locals.var_t1_dn4 = assign21430_e16413_d_n4;
        locals.var_t1_dn5 = assign21430_e16413_d_n5;
        locals.var_t1_dn6 = assign21430_e16413_d_n6;
        locals.var_t1_dn7 = assign21430_e16413_d_n7;
        locals.var_t1_dn8 = assign21430_e16413_d_n8;
        locals.var_t1_dn9 = assign21430_e16413_d_n9;
        locals.var_t1_dn10 = assign21430_e16413_d_n10;
        locals.var_t1_dn11 = assign21430_e16413_d_n11;
        locals.var_t1_dn14 = assign21430_e16413_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21440_e16423, assign21440_e16423_d_n0, assign21440_e16423_d_n2, assign21440_e16423_d_n4, assign21440_e16423_d_n5, assign21440_e16423_d_n6, assign21440_e16423_d_n7, assign21440_e16423_d_n8, assign21440_e16423_d_n9, assign21440_e16423_d_n10, assign21440_e16423_d_n11, assign21440_e16423_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21440_e16423;
        locals.var_t2_dn0 = assign21440_e16423_d_n0;
        locals.var_t2_dn2 = assign21440_e16423_d_n2;
        locals.var_t2_dn4 = assign21440_e16423_d_n4;
        locals.var_t2_dn5 = assign21440_e16423_d_n5;
        locals.var_t2_dn6 = assign21440_e16423_d_n6;
        locals.var_t2_dn7 = assign21440_e16423_d_n7;
        locals.var_t2_dn8 = assign21440_e16423_d_n8;
        locals.var_t2_dn9 = assign21440_e16423_d_n9;
        locals.var_t2_dn10 = assign21440_e16423_d_n10;
        locals.var_t2_dn11 = assign21440_e16423_d_n11;
        locals.var_t2_dn14 = assign21440_e16423_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21450_e16435, assign21450_e16435_d_n0, assign21450_e16435_d_n2, assign21450_e16435_d_n4, assign21450_e16435_d_n5, assign21450_e16435_d_n6, assign21450_e16435_d_n7, assign21450_e16435_d_n8, assign21450_e16435_d_n9, assign21450_e16435_d_n10, assign21450_e16435_d_n11, assign21450_e16435_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21450_e16432: f64 = (10.0 * 2.220446049250313e-16);
        let assign21450_e16433: f64 = (locals.var_t1 + assign21450_e16432);
        (assign21450_e16433, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21450_e16435;
        locals.var_t1_dn0 = assign21450_e16435_d_n0;
        locals.var_t1_dn2 = assign21450_e16435_d_n2;
        locals.var_t1_dn4 = assign21450_e16435_d_n4;
        locals.var_t1_dn5 = assign21450_e16435_d_n5;
        locals.var_t1_dn6 = assign21450_e16435_d_n6;
        locals.var_t1_dn7 = assign21450_e16435_d_n7;
        locals.var_t1_dn8 = assign21450_e16435_d_n8;
        locals.var_t1_dn9 = assign21450_e16435_d_n9;
        locals.var_t1_dn10 = assign21450_e16435_d_n10;
        locals.var_t1_dn11 = assign21450_e16435_d_n11;
        locals.var_t1_dn14 = assign21450_e16435_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21460_e16449, assign21460_e16449_d_n0, assign21460_e16449_d_n2, assign21460_e16449_d_n4, assign21460_e16449_d_n5, assign21460_e16449_d_n6, assign21460_e16449_d_n7, assign21460_e16449_d_n8, assign21460_e16449_d_n9, assign21460_e16449_d_n10, assign21460_e16449_d_n11, assign21460_e16449_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21460_e16445: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21460_e16446: f64 = (locals.var_uc_nover * assign21460_e16445);
        let assign21460_e16447: f64 = (locals.var_mks_nsubsub / assign21460_e16446);
        (assign21460_e16447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21460_e16449;
        locals.var_t0_dn0 = assign21460_e16449_d_n0;
        locals.var_t0_dn2 = assign21460_e16449_d_n2;
        locals.var_t0_dn4 = assign21460_e16449_d_n4;
        locals.var_t0_dn5 = assign21460_e16449_d_n5;
        locals.var_t0_dn6 = assign21460_e16449_d_n6;
        locals.var_t0_dn7 = assign21460_e16449_d_n7;
        locals.var_t0_dn8 = assign21460_e16449_d_n8;
        locals.var_t0_dn9 = assign21460_e16449_d_n9;
        locals.var_t0_dn10 = assign21460_e16449_d_n10;
        locals.var_t0_dn11 = assign21460_e16449_d_n11;
        locals.var_t0_dn14 = assign21460_e16449_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21470_e16463, assign21470_e16463_d_n0, assign21470_e16463_d_n2, assign21470_e16463_d_n4, assign21470_e16463_d_n5, assign21470_e16463_d_n6, assign21470_e16463_d_n7, assign21470_e16463_d_n8, assign21470_e16463_d_n9, assign21470_e16463_d_n10, assign21470_e16463_d_n11, assign21470_e16463_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21470_e16457: f64 = (2.0 * 1.034943e-10);
        let assign21470_e16459: f64 = (assign21470_e16457 / 1.6021918e-19);
        let assign21470_e16461: f64 = (assign21470_e16459 * locals.var_t0);
        (assign21470_e16461, (assign21470_e16459 * locals.var_t0_dn0), (assign21470_e16459 * locals.var_t0_dn2), (assign21470_e16459 * locals.var_t0_dn4), (assign21470_e16459 * locals.var_t0_dn5), (assign21470_e16459 * locals.var_t0_dn6), (assign21470_e16459 * locals.var_t0_dn7), (assign21470_e16459 * locals.var_t0_dn8), (assign21470_e16459 * locals.var_t0_dn9), (assign21470_e16459 * locals.var_t0_dn10), (assign21470_e16459 * locals.var_t0_dn11), (assign21470_e16459 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21470_e16463;
        locals.var_t4_dn0 = assign21470_e16463_d_n0;
        locals.var_t4_dn2 = assign21470_e16463_d_n2;
        locals.var_t4_dn4 = assign21470_e16463_d_n4;
        locals.var_t4_dn5 = assign21470_e16463_d_n5;
        locals.var_t4_dn6 = assign21470_e16463_d_n6;
        locals.var_t4_dn7 = assign21470_e16463_d_n7;
        locals.var_t4_dn8 = assign21470_e16463_d_n8;
        locals.var_t4_dn9 = assign21470_e16463_d_n9;
        locals.var_t4_dn10 = assign21470_e16463_d_n10;
        locals.var_t4_dn11 = assign21470_e16463_d_n11;
        locals.var_t4_dn14 = assign21470_e16463_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21480_e16476, assign21480_e16476_d_n0, assign21480_e16476_d_n2, assign21480_e16476_d_n4, assign21480_e16476_d_n5, assign21480_e16476_d_n6, assign21480_e16476_d_n7, assign21480_e16476_d_n8, assign21480_e16476_d_n9, assign21480_e16476_d_n10, assign21480_e16476_d_n11, assign21480_e16476_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21480_e16471: f64 = (locals.var_t4 * locals.var_t1);
        let assign21480_e16472: f64 = (assign21480_e16471).sqrt();
        let assign21480_e16474: f64 = (assign21480_e16472 + 1e-25);
        (assign21480_e16474, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign21480_e16472)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign21480_e16472)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21480_e16476;
        locals.var_wdep_dn0 = assign21480_e16476_d_n0;
        locals.var_wdep_dn2 = assign21480_e16476_d_n2;
        locals.var_wdep_dn4 = assign21480_e16476_d_n4;
        locals.var_wdep_dn5 = assign21480_e16476_d_n5;
        locals.var_wdep_dn6 = assign21480_e16476_d_n6;
        locals.var_wdep_dn7 = assign21480_e16476_d_n7;
        locals.var_wdep_dn8 = assign21480_e16476_d_n8;
        locals.var_wdep_dn9 = assign21480_e16476_d_n9;
        locals.var_wdep_dn10 = assign21480_e16476_d_n10;
        locals.var_wdep_dn11 = assign21480_e16476_d_n11;
        locals.var_wdep_dn14 = assign21480_e16476_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21490_e16490, assign21490_e16490_d_n0, assign21490_e16490_d_n2, assign21490_e16490_d_n4, assign21490_e16490_d_n5, assign21490_e16490_d_n6, assign21490_e16490_d_n7, assign21490_e16490_d_n8, assign21490_e16490_d_n9, assign21490_e16490_d_n10, assign21490_e16490_d_n11, assign21490_e16490_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21490_e16484: f64 = (p.p334 - locals.var_wdep);
        let assign21490_e16487: f64 = (0.1 * p.p334);
        let assign21490_e16488: f64 = (assign21490_e16484 - assign21490_e16487);
        (assign21490_e16488, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21490_e16490;
        locals.var_tmf1_dn0 = assign21490_e16490_d_n0;
        locals.var_tmf1_dn2 = assign21490_e16490_d_n2;
        locals.var_tmf1_dn4 = assign21490_e16490_d_n4;
        locals.var_tmf1_dn5 = assign21490_e16490_d_n5;
        locals.var_tmf1_dn6 = assign21490_e16490_d_n6;
        locals.var_tmf1_dn7 = assign21490_e16490_d_n7;
        locals.var_tmf1_dn8 = assign21490_e16490_d_n8;
        locals.var_tmf1_dn9 = assign21490_e16490_d_n9;
        locals.var_tmf1_dn10 = assign21490_e16490_d_n10;
        locals.var_tmf1_dn11 = assign21490_e16490_d_n11;
        locals.var_tmf1_dn14 = assign21490_e16490_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21500_e16504, assign21500_e16504_d_n0, assign21500_e16504_d_n2, assign21500_e16504_d_n4, assign21500_e16504_d_n5, assign21500_e16504_d_n6, assign21500_e16504_d_n7, assign21500_e16504_d_n8, assign21500_e16504_d_n9, assign21500_e16504_d_n10, assign21500_e16504_d_n11, assign21500_e16504_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21500_e16498: f64 = (4.0 * p.p334);
        let assign21500_e16501: f64 = (0.1 * p.p334);
        let assign21500_e16502: f64 = (assign21500_e16498 * assign21500_e16501);
        (assign21500_e16502, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21500_e16504;
        locals.var_tmf2_dn0 = assign21500_e16504_d_n0;
        locals.var_tmf2_dn2 = assign21500_e16504_d_n2;
        locals.var_tmf2_dn4 = assign21500_e16504_d_n4;
        locals.var_tmf2_dn5 = assign21500_e16504_d_n5;
        locals.var_tmf2_dn6 = assign21500_e16504_d_n6;
        locals.var_tmf2_dn7 = assign21500_e16504_d_n7;
        locals.var_tmf2_dn8 = assign21500_e16504_d_n8;
        locals.var_tmf2_dn9 = assign21500_e16504_d_n9;
        locals.var_tmf2_dn10 = assign21500_e16504_d_n10;
        locals.var_tmf2_dn11 = assign21500_e16504_d_n11;
        locals.var_tmf2_dn14 = assign21500_e16504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21510_e16518, assign21510_e16518_d_n0, assign21510_e16518_d_n2, assign21510_e16518_d_n4, assign21510_e16518_d_n5, assign21510_e16518_d_n6, assign21510_e16518_d_n7, assign21510_e16518_d_n8, assign21510_e16518_d_n9, assign21510_e16518_d_n10, assign21510_e16518_d_n11, assign21510_e16518_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let (assign21510_e16516, assign21510_e16516_d_n0, assign21510_e16516_d_n2, assign21510_e16516_d_n4, assign21510_e16516_d_n5, assign21510_e16516_d_n6, assign21510_e16516_d_n7, assign21510_e16516_d_n8, assign21510_e16516_d_n9, assign21510_e16516_d_n10, assign21510_e16516_d_n11, assign21510_e16516_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21510_e16515: f64 = (-locals.var_tmf2);
                (assign21510_e16515, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21510_e16516, assign21510_e16516_d_n0, assign21510_e16516_d_n2, assign21510_e16516_d_n4, assign21510_e16516_d_n5, assign21510_e16516_d_n6, assign21510_e16516_d_n7, assign21510_e16516_d_n8, assign21510_e16516_d_n9, assign21510_e16516_d_n10, assign21510_e16516_d_n11, assign21510_e16516_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21510_e16518;
        locals.var_tmf2_dn0 = assign21510_e16518_d_n0;
        locals.var_tmf2_dn2 = assign21510_e16518_d_n2;
        locals.var_tmf2_dn4 = assign21510_e16518_d_n4;
        locals.var_tmf2_dn5 = assign21510_e16518_d_n5;
        locals.var_tmf2_dn6 = assign21510_e16518_d_n6;
        locals.var_tmf2_dn7 = assign21510_e16518_d_n7;
        locals.var_tmf2_dn8 = assign21510_e16518_d_n8;
        locals.var_tmf2_dn9 = assign21510_e16518_d_n9;
        locals.var_tmf2_dn10 = assign21510_e16518_d_n10;
        locals.var_tmf2_dn11 = assign21510_e16518_d_n11;
        locals.var_tmf2_dn14 = assign21510_e16518_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21520_e16531, assign21520_e16531_d_n0, assign21520_e16531_d_n2, assign21520_e16531_d_n4, assign21520_e16531_d_n5, assign21520_e16531_d_n6, assign21520_e16531_d_n7, assign21520_e16531_d_n8, assign21520_e16531_d_n9, assign21520_e16531_d_n10, assign21520_e16531_d_n11, assign21520_e16531_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21520_e16526: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21520_e16528: f64 = (assign21520_e16526 + locals.var_tmf2);
        let assign21520_e16529: f64 = (assign21520_e16528).sqrt();
        (assign21520_e16529, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21520_e16529)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21520_e16529)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21520_e16531;
        locals.var_tmf2_dn0 = assign21520_e16531_d_n0;
        locals.var_tmf2_dn2 = assign21520_e16531_d_n2;
        locals.var_tmf2_dn4 = assign21520_e16531_d_n4;
        locals.var_tmf2_dn5 = assign21520_e16531_d_n5;
        locals.var_tmf2_dn6 = assign21520_e16531_d_n6;
        locals.var_tmf2_dn7 = assign21520_e16531_d_n7;
        locals.var_tmf2_dn8 = assign21520_e16531_d_n8;
        locals.var_tmf2_dn9 = assign21520_e16531_d_n9;
        locals.var_tmf2_dn10 = assign21520_e16531_d_n10;
        locals.var_tmf2_dn11 = assign21520_e16531_d_n11;
        locals.var_tmf2_dn14 = assign21520_e16531_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21530_e16545, assign21530_e16545_d_n0, assign21530_e16545_d_n2, assign21530_e16545_d_n4, assign21530_e16545_d_n5, assign21530_e16545_d_n6, assign21530_e16545_d_n7, assign21530_e16545_d_n8, assign21530_e16545_d_n9, assign21530_e16545_d_n10, assign21530_e16545_d_n11, assign21530_e16545_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21530_e16541: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21530_e16542: f64 = (1.0 + assign21530_e16541);
        let assign21530_e16543: f64 = (0.5 * assign21530_e16542);
        (assign21530_e16543, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21530_e16545;
        locals.var_t0_dn0 = assign21530_e16545_d_n0;
        locals.var_t0_dn2 = assign21530_e16545_d_n2;
        locals.var_t0_dn4 = assign21530_e16545_d_n4;
        locals.var_t0_dn5 = assign21530_e16545_d_n5;
        locals.var_t0_dn6 = assign21530_e16545_d_n6;
        locals.var_t0_dn7 = assign21530_e16545_d_n7;
        locals.var_t0_dn8 = assign21530_e16545_d_n8;
        locals.var_t0_dn9 = assign21530_e16545_d_n9;
        locals.var_t0_dn10 = assign21530_e16545_d_n10;
        locals.var_t0_dn11 = assign21530_e16545_d_n11;
        locals.var_t0_dn14 = assign21530_e16545_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21540_e16559, assign21540_e16559_d_n0, assign21540_e16559_d_n2, assign21540_e16559_d_n4, assign21540_e16559_d_n5, assign21540_e16559_d_n6, assign21540_e16559_d_n7, assign21540_e16559_d_n8, assign21540_e16559_d_n9, assign21540_e16559_d_n10, assign21540_e16559_d_n11, assign21540_e16559_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21540_e16555: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21540_e16556: f64 = (0.5 * assign21540_e16555);
        let assign21540_e16557: f64 = (p.p334 - assign21540_e16556);
        (assign21540_e16557, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21540_e16559;
        locals.var_wdep_dn0 = assign21540_e16559_d_n0;
        locals.var_wdep_dn2 = assign21540_e16559_d_n2;
        locals.var_wdep_dn4 = assign21540_e16559_d_n4;
        locals.var_wdep_dn5 = assign21540_e16559_d_n5;
        locals.var_wdep_dn6 = assign21540_e16559_d_n6;
        locals.var_wdep_dn7 = assign21540_e16559_d_n7;
        locals.var_wdep_dn8 = assign21540_e16559_d_n8;
        locals.var_wdep_dn9 = assign21540_e16559_d_n9;
        locals.var_wdep_dn10 = assign21540_e16559_d_n10;
        locals.var_wdep_dn11 = assign21540_e16559_d_n11;
        locals.var_wdep_dn14 = assign21540_e16559_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21550_e16571, assign21550_e16571_d_n0, assign21550_e16571_d_n2, assign21550_e16571_d_n4, assign21550_e16571_d_n5, assign21550_e16571_d_n6, assign21550_e16571_d_n7, assign21550_e16571_d_n8, assign21550_e16571_d_n9, assign21550_e16571_d_n10, assign21550_e16571_d_n11, assign21550_e16571_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21550_e16568: f64 = (p.p334 - locals.var_wdep);
        let assign21550_e16569: f64 = (locals.var_ldrift0 / assign21550_e16568);
        (assign21550_e16569, (-((locals.var_ldrift0 * (-locals.var_wdep_dn0)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn2)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn4)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn5)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn6)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn7)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn8)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn9)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn10)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn11)) / (assign21550_e16568 * assign21550_e16568))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn14)) / (assign21550_e16568 * assign21550_e16568))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign21550_e16571;
        locals.var_t6_dn0 = assign21550_e16571_d_n0;
        locals.var_t6_dn2 = assign21550_e16571_d_n2;
        locals.var_t6_dn4 = assign21550_e16571_d_n4;
        locals.var_t6_dn5 = assign21550_e16571_d_n5;
        locals.var_t6_dn6 = assign21550_e16571_d_n6;
        locals.var_t6_dn7 = assign21550_e16571_d_n7;
        locals.var_t6_dn8 = assign21550_e16571_d_n8;
        locals.var_t6_dn9 = assign21550_e16571_d_n9;
        locals.var_t6_dn10 = assign21550_e16571_d_n10;
        locals.var_t6_dn11 = assign21550_e16571_d_n11;
        locals.var_t6_dn14 = assign21550_e16571_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign21560_e16581, assign21560_e16581_d_n0, assign21560_e16581_d_n2, assign21560_e16581_d_n4, assign21560_e16581_d_n5, assign21560_e16581_d_n6, assign21560_e16581_d_n7, assign21560_e16581_d_n8, assign21560_e16581_d_n9, assign21560_e16581_d_n10, assign21560_e16581_d_n11, assign21560_e16581_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21560_e16579: f64 = (locals.var_rdrift * locals.var_t6);
        (assign21560_e16579, ((locals.var_rdrift_dn0 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn0)), ((locals.var_rdrift_dn2 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn2)), ((locals.var_rdrift_dn4 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn4)), ((locals.var_rdrift_dn5 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn5)), ((locals.var_rdrift_dn6 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn6)), ((locals.var_rdrift_dn7 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn7)), ((locals.var_rdrift_dn8 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn8)), ((locals.var_rdrift_dn9 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn9)), ((locals.var_rdrift_dn10 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn10)), ((locals.var_rdrift_dn11 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn11)), ((locals.var_rdrift_dn14 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21560_e16581;
        locals.var_t0_dn0 = assign21560_e16581_d_n0;
        locals.var_t0_dn2 = assign21560_e16581_d_n2;
        locals.var_t0_dn4 = assign21560_e16581_d_n4;
        locals.var_t0_dn5 = assign21560_e16581_d_n5;
        locals.var_t0_dn6 = assign21560_e16581_d_n6;
        locals.var_t0_dn7 = assign21560_e16581_d_n7;
        locals.var_t0_dn8 = assign21560_e16581_d_n8;
        locals.var_t0_dn9 = assign21560_e16581_d_n9;
        locals.var_t0_dn10 = assign21560_e16581_d_n10;
        locals.var_t0_dn11 = assign21560_e16581_d_n11;
        locals.var_t0_dn14 = assign21560_e16581_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21570_e16591, assign21570_e16591_d_n0, assign21570_e16591_d_n2, assign21570_e16591_d_n4, assign21570_e16591_d_n5, assign21570_e16591_d_n6, assign21570_e16591_d_n7, assign21570_e16591_d_n8, assign21570_e16591_d_n9, assign21570_e16591_d_n10, assign21570_e16591_d_n11, assign21570_e16591_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21570_e16589: f64 = (locals.var_rsdrift * locals.var_t6);
        (assign21570_e16589, ((locals.var_rsdrift_dn0 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21570_e16591;
        locals.var_t1_dn0 = assign21570_e16591_d_n0;
        locals.var_t1_dn2 = assign21570_e16591_d_n2;
        locals.var_t1_dn4 = assign21570_e16591_d_n4;
        locals.var_t1_dn5 = assign21570_e16591_d_n5;
        locals.var_t1_dn6 = assign21570_e16591_d_n6;
        locals.var_t1_dn7 = assign21570_e16591_d_n7;
        locals.var_t1_dn8 = assign21570_e16591_d_n8;
        locals.var_t1_dn9 = assign21570_e16591_d_n9;
        locals.var_t1_dn10 = assign21570_e16591_d_n10;
        locals.var_t1_dn11 = assign21570_e16591_d_n11;
        locals.var_t1_dn14 = assign21570_e16591_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21580_e16605, assign21580_e16605_d_n0, assign21580_e16605_d_n2, assign21580_e16605_d_n4, assign21580_e16605_d_n5, assign21580_e16605_d_n6, assign21580_e16605_d_n7, assign21580_e16605_d_n8, assign21580_e16605_d_n9, assign21580_e16605_d_n10, assign21580_e16605_d_n11, assign21580_e16605_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21580_e16599: f64 = (locals.var_t0 * locals.var_vdsemodenml);
        let assign21580_e16602: f64 = (locals.var_rdrift * locals.var_vdsemodervs);
        let assign21580_e16603: f64 = (assign21580_e16599 + assign21580_e16602);
        (assign21580_e16603, ((locals.var_t0_dn0 * locals.var_vdsemodenml) + (locals.var_rdrift_dn0 * locals.var_vdsemodervs)), ((locals.var_t0_dn2 * locals.var_vdsemodenml) + (locals.var_rdrift_dn2 * locals.var_vdsemodervs)), ((locals.var_t0_dn4 * locals.var_vdsemodenml) + (locals.var_rdrift_dn4 * locals.var_vdsemodervs)), ((locals.var_t0_dn5 * locals.var_vdsemodenml) + (locals.var_rdrift_dn5 * locals.var_vdsemodervs)), ((locals.var_t0_dn6 * locals.var_vdsemodenml) + (locals.var_rdrift_dn6 * locals.var_vdsemodervs)), ((locals.var_t0_dn7 * locals.var_vdsemodenml) + (locals.var_rdrift_dn7 * locals.var_vdsemodervs)), ((locals.var_t0_dn8 * locals.var_vdsemodenml) + (locals.var_rdrift_dn8 * locals.var_vdsemodervs)), ((locals.var_t0_dn9 * locals.var_vdsemodenml) + (locals.var_rdrift_dn9 * locals.var_vdsemodervs)), ((locals.var_t0_dn10 * locals.var_vdsemodenml) + (locals.var_rdrift_dn10 * locals.var_vdsemodervs)), ((locals.var_t0_dn11 * locals.var_vdsemodenml) + (locals.var_rdrift_dn11 * locals.var_vdsemodervs)), ((locals.var_t0_dn14 * locals.var_vdsemodenml) + (locals.var_rdrift_dn14 * locals.var_vdsemodervs)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21580_e16605;
        locals.var_rdrift_dn0 = assign21580_e16605_d_n0;
        locals.var_rdrift_dn2 = assign21580_e16605_d_n2;
        locals.var_rdrift_dn4 = assign21580_e16605_d_n4;
        locals.var_rdrift_dn5 = assign21580_e16605_d_n5;
        locals.var_rdrift_dn6 = assign21580_e16605_d_n6;
        locals.var_rdrift_dn7 = assign21580_e16605_d_n7;
        locals.var_rdrift_dn8 = assign21580_e16605_d_n8;
        locals.var_rdrift_dn9 = assign21580_e16605_d_n9;
        locals.var_rdrift_dn10 = assign21580_e16605_d_n10;
        locals.var_rdrift_dn11 = assign21580_e16605_d_n11;
        locals.var_rdrift_dn14 = assign21580_e16605_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21590_e16619, assign21590_e16619_d_n0, assign21590_e16619_d_n2, assign21590_e16619_d_n4, assign21590_e16619_d_n5, assign21590_e16619_d_n6, assign21590_e16619_d_n7, assign21590_e16619_d_n8, assign21590_e16619_d_n9, assign21590_e16619_d_n10, assign21590_e16619_d_n11, assign21590_e16619_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21590_e16613: f64 = (locals.var_t1 * locals.var_vdsemodervs);
        let assign21590_e16616: f64 = (locals.var_rsdrift * locals.var_vdsemodenml);
        let assign21590_e16617: f64 = (assign21590_e16613 + assign21590_e16616);
        (assign21590_e16617, ((locals.var_t1_dn0 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn0 * locals.var_vdsemodenml)), ((locals.var_t1_dn2 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn2 * locals.var_vdsemodenml)), ((locals.var_t1_dn4 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn4 * locals.var_vdsemodenml)), ((locals.var_t1_dn5 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn5 * locals.var_vdsemodenml)), ((locals.var_t1_dn6 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn6 * locals.var_vdsemodenml)), ((locals.var_t1_dn7 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn7 * locals.var_vdsemodenml)), ((locals.var_t1_dn8 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn8 * locals.var_vdsemodenml)), ((locals.var_t1_dn9 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn9 * locals.var_vdsemodenml)), ((locals.var_t1_dn10 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn10 * locals.var_vdsemodenml)), ((locals.var_t1_dn11 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn11 * locals.var_vdsemodenml)), ((locals.var_t1_dn14 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn14 * locals.var_vdsemodenml)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21590_e16619;
        locals.var_rsdrift_dn0 = assign21590_e16619_d_n0;
        locals.var_rsdrift_dn2 = assign21590_e16619_d_n2;
        locals.var_rsdrift_dn4 = assign21590_e16619_d_n4;
        locals.var_rsdrift_dn5 = assign21590_e16619_d_n5;
        locals.var_rsdrift_dn6 = assign21590_e16619_d_n6;
        locals.var_rsdrift_dn7 = assign21590_e16619_d_n7;
        locals.var_rsdrift_dn8 = assign21590_e16619_d_n8;
        locals.var_rsdrift_dn9 = assign21590_e16619_d_n9;
        locals.var_rsdrift_dn10 = assign21590_e16619_d_n10;
        locals.var_rsdrift_dn11 = assign21590_e16619_d_n11;
        locals.var_rsdrift_dn14 = assign21590_e16619_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21600_e16628, assign21600_e16628_d_n0, assign21600_e16628_d_n2, assign21600_e16628_d_n4, assign21600_e16628_d_n5, assign21600_e16628_d_n6, assign21600_e16628_d_n7, assign21600_e16628_d_n8, assign21600_e16628_d_n9, assign21600_e16628_d_n10, assign21600_e16628_d_n11, assign21600_e16628_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21600_e16628;
        locals.var_wdep_dn0 = assign21600_e16628_d_n0;
        locals.var_wdep_dn2 = assign21600_e16628_d_n2;
        locals.var_wdep_dn4 = assign21600_e16628_d_n4;
        locals.var_wdep_dn5 = assign21600_e16628_d_n5;
        locals.var_wdep_dn6 = assign21600_e16628_d_n6;
        locals.var_wdep_dn7 = assign21600_e16628_d_n7;
        locals.var_wdep_dn8 = assign21600_e16628_d_n8;
        locals.var_wdep_dn9 = assign21600_e16628_d_n9;
        locals.var_wdep_dn10 = assign21600_e16628_d_n10;
        locals.var_wdep_dn11 = assign21600_e16628_d_n11;
        locals.var_wdep_dn14 = assign21600_e16628_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21610_e16634, assign21610_e16634_d_n0, assign21610_e16634_d_n2, assign21610_e16634_d_n4, assign21610_e16634_d_n5, assign21610_e16634_d_n6, assign21610_e16634_d_n7, assign21610_e16634_d_n8, assign21610_e16634_d_n9, assign21610_e16634_d_n10, assign21610_e16634_d_n11, assign21610_e16634_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21610_e16634;
        locals.var_rdd_dn0 = assign21610_e16634_d_n0;
        locals.var_rdd_dn2 = assign21610_e16634_d_n2;
        locals.var_rdd_dn4 = assign21610_e16634_d_n4;
        locals.var_rdd_dn5 = assign21610_e16634_d_n5;
        locals.var_rdd_dn6 = assign21610_e16634_d_n6;
        locals.var_rdd_dn7 = assign21610_e16634_d_n7;
        locals.var_rdd_dn8 = assign21610_e16634_d_n8;
        locals.var_rdd_dn9 = assign21610_e16634_d_n9;
        locals.var_rdd_dn10 = assign21610_e16634_d_n10;
        locals.var_rdd_dn11 = assign21610_e16634_d_n11;
        locals.var_rdd_dn14 = assign21610_e16634_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21620_e16640, assign21620_e16640_d_n0, assign21620_e16640_d_n2, assign21620_e16640_d_n4, assign21620_e16640_d_n5, assign21620_e16640_d_n6, assign21620_e16640_d_n7, assign21620_e16640_d_n8, assign21620_e16640_d_n9, assign21620_e16640_d_n10, assign21620_e16640_d_n11, assign21620_e16640_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21620_e16640;
        locals.var_rsd_dn0 = assign21620_e16640_d_n0;
        locals.var_rsd_dn2 = assign21620_e16640_d_n2;
        locals.var_rsd_dn4 = assign21620_e16640_d_n4;
        locals.var_rsd_dn5 = assign21620_e16640_d_n5;
        locals.var_rsd_dn6 = assign21620_e16640_d_n6;
        locals.var_rsd_dn7 = assign21620_e16640_d_n7;
        locals.var_rsd_dn8 = assign21620_e16640_d_n8;
        locals.var_rsd_dn9 = assign21620_e16640_d_n9;
        locals.var_rsd_dn10 = assign21620_e16640_d_n10;
        locals.var_rsd_dn11 = assign21620_e16640_d_n11;
        locals.var_rsd_dn14 = assign21620_e16640_d_n14;
        locals.var_rsd_rv = 0.0;

    }
}
