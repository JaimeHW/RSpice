#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_idspt1 = 0.0;
        locals.var_idspt1_dn0 = 0.0;
        locals.var_idspt1_dn2 = 0.0;
        locals.var_idspt1_dn4 = 0.0;
        locals.var_idspt1_dn5 = 0.0;
        locals.var_idspt1_dn6 = 0.0;
        locals.var_idspt1_dn7 = 0.0;
        locals.var_idspt1_dn8 = 0.0;
        locals.var_idspt1_dn9 = 0.0;
        locals.var_idspt1_dn10 = 0.0;
        locals.var_idspt1_dn11 = 0.0;
        locals.var_idspt1_dn14 = 0.0;
        locals.var_idspt1_rv = 0.0;

        locals.var_cox0_func = 0.0;
        locals.var_cox0_func_rv = 0.0;

        locals.var_iwnqs0_a = 0.0;
        locals.var_iwnqs0_a_dn0 = 0.0;
        locals.var_iwnqs0_a_dn2 = 0.0;
        locals.var_iwnqs0_a_dn4 = 0.0;
        locals.var_iwnqs0_a_dn5 = 0.0;
        locals.var_iwnqs0_a_dn6 = 0.0;
        locals.var_iwnqs0_a_dn7 = 0.0;
        locals.var_iwnqs0_a_dn8 = 0.0;
        locals.var_iwnqs0_a_dn9 = 0.0;
        locals.var_iwnqs0_a_dn10 = 0.0;
        locals.var_iwnqs0_a_dn11 = 0.0;
        locals.var_iwnqs0_a_dn14 = 0.0;
        locals.var_iwnqs0_a_dn18 = 0.0;
        locals.var_iwnqs0_a_rv = 0.0;

        locals.var_inqs0_a = 0.0;
        locals.var_inqs0_a_dn0 = 0.0;
        locals.var_inqs0_a_dn2 = 0.0;
        locals.var_inqs0_a_dn4 = 0.0;
        locals.var_inqs0_a_dn5 = 0.0;
        locals.var_inqs0_a_dn6 = 0.0;
        locals.var_inqs0_a_dn7 = 0.0;
        locals.var_inqs0_a_dn8 = 0.0;
        locals.var_inqs0_a_dn9 = 0.0;
        locals.var_inqs0_a_dn10 = 0.0;
        locals.var_inqs0_a_dn11 = 0.0;
        locals.var_inqs0_a_dn14 = 0.0;
        locals.var_inqs0_a_dn16 = 0.0;
        locals.var_inqs0_a_rv = 0.0;

        locals.var_inqs0_k = 0.0;
        locals.var_inqs0_k_dn0 = 0.0;
        locals.var_inqs0_k_dn2 = 0.0;
        locals.var_inqs0_k_dn4 = 0.0;
        locals.var_inqs0_k_dn5 = 0.0;
        locals.var_inqs0_k_dn6 = 0.0;
        locals.var_inqs0_k_dn7 = 0.0;
        locals.var_inqs0_k_dn8 = 0.0;
        locals.var_inqs0_k_dn9 = 0.0;
        locals.var_inqs0_k_dn10 = 0.0;
        locals.var_inqs0_k_dn11 = 0.0;
        locals.var_inqs0_k_dn14 = 0.0;
        locals.var_inqs0_k_dn17 = 0.0;
        locals.var_inqs0_k_rv = 0.0;

        locals.var_isubibpc = 0.0;
        locals.var_isubibpc_dn0 = 0.0;
        locals.var_isubibpc_dn2 = 0.0;
        locals.var_isubibpc_dn4 = 0.0;
        locals.var_isubibpc_dn5 = 0.0;
        locals.var_isubibpc_dn6 = 0.0;
        locals.var_isubibpc_dn7 = 0.0;
        locals.var_isubibpc_dn8 = 0.0;
        locals.var_isubibpc_dn9 = 0.0;
        locals.var_isubibpc_dn10 = 0.0;
        locals.var_isubibpc_dn11 = 0.0;
        locals.var_isubibpc_dn14 = 0.0;
        locals.var_isubibpc_rv = 0.0;

        locals.var_lover_func = 0.0;
        locals.var_lover_func_dn0 = 0.0;
        locals.var_lover_func_dn2 = 0.0;
        locals.var_lover_func_dn4 = 0.0;
        locals.var_lover_func_dn5 = 0.0;
        locals.var_lover_func_dn6 = 0.0;
        locals.var_lover_func_dn7 = 0.0;
        locals.var_lover_func_dn8 = 0.0;
        locals.var_lover_func_dn9 = 0.0;
        locals.var_lover_func_dn10 = 0.0;
        locals.var_lover_func_dn11 = 0.0;
        locals.var_lover_func_dn14 = 0.0;
        locals.var_lover_func_rv = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn16 = 0.0;
        locals.var_q_nqs_a_rv = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn17 = 0.0;
        locals.var_q_nqs_k_rv = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn18 = 0.0;
        locals.var_w_nqs_a_rv = 0.0;

        locals.var_w_res = 0.0;
        locals.var_w_res_dn0 = 0.0;
        locals.var_w_res_dn2 = 0.0;
        locals.var_w_res_dn4 = 0.0;
        locals.var_w_res_dn5 = 0.0;
        locals.var_w_res_dn6 = 0.0;
        locals.var_w_res_dn7 = 0.0;
        locals.var_w_res_dn8 = 0.0;
        locals.var_w_res_dn9 = 0.0;
        locals.var_w_res_dn10 = 0.0;
        locals.var_w_res_dn11 = 0.0;
        locals.var_w_res_dn14 = 0.0;
        locals.var_w_res_rv = 0.0;

        locals.var_wdep_func = 0.0;
        locals.var_wdep_func_dn0 = 0.0;
        locals.var_wdep_func_dn2 = 0.0;
        locals.var_wdep_func_dn4 = 0.0;
        locals.var_wdep_func_dn5 = 0.0;
        locals.var_wdep_func_dn6 = 0.0;
        locals.var_wdep_func_dn7 = 0.0;
        locals.var_wdep_func_dn8 = 0.0;
        locals.var_wdep_func_dn9 = 0.0;
        locals.var_wdep_func_dn10 = 0.0;
        locals.var_wdep_func_dn11 = 0.0;
        locals.var_wdep_func_dn14 = 0.0;
        locals.var_wdep_func_rv = 0.0;

        locals.var_wk_ii = 0.0;
        locals.var_wk_ii_dn0 = 0.0;
        locals.var_wk_ii_dn2 = 0.0;
        locals.var_wk_ii_dn4 = 0.0;
        locals.var_wk_ii_dn5 = 0.0;
        locals.var_wk_ii_dn6 = 0.0;
        locals.var_wk_ii_dn7 = 0.0;
        locals.var_wk_ii_dn8 = 0.0;
        locals.var_wk_ii_dn9 = 0.0;
        locals.var_wk_ii_dn10 = 0.0;
        locals.var_wk_ii_dn11 = 0.0;
        locals.var_wk_ii_dn14 = 0.0;
        locals.var_wk_ii_rv = 0.0;

        let (assign5340_e1947,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        locals.var_uc_corsrd = assign5340_e1947;
        locals.var_uc_corsrd_rv = 0.0;

        locals.var_uc_xpdv = p.p104;
        locals.var_uc_xpdv_rv = 0.0;

        locals.var_uc_xldld = p.p294;
        locals.var_uc_xldld_rv = 0.0;

        locals.var_uc_scp22 = p.p222;
        locals.var_uc_scp22_rv = 0.0;

        locals.var_uc_rdrcx = p.p420;
        locals.var_uc_rdrcx_rv = 0.0;

        locals.var_mfactor = 1.0;
        locals.var_mfactor_rv = 0.0;

        let assign5500_e1990: f64 = if locals.var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign5500_e1990;
        locals.var_guard10_rv = 0.0;

        let (assign5510_e1994,) = {
    if (locals.var_guard10 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5510_e1994;
        locals.var_uc_scp22_rv = 0.0;

        let assign5520_e1997: f64 = if locals.var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign5520_e1997;
        locals.var_guard11_rv = 0.0;

        let (assign5530_e2001,) = {
    if (locals.var_guard11 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5530_e2001;
        locals.var_uc_scp22_rv = 0.0;

        let assign5550_e2009: f64 = if locals.var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign5550_e2009;
        locals.var_guard13_rv = 0.0;

        let (assign5560_e2013,) = {
    if (locals.var_guard13 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_xldld,)
    }
};
        locals.var_uc_xldld = assign5560_e2013;
        locals.var_uc_xldld_rv = 0.0;

        let assign5590_e2026: f64 = if locals.var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign5590_e2026;
        locals.var_guard16_rv = 0.0;

        let (assign5600_e2030,) = {
    if (locals.var_guard16 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5600_e2030;
        locals.var_uc_rdrcx_rv = 0.0;

        let assign5610_e2033: f64 = if locals.var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign5610_e2033;
        locals.var_guard17_rv = 0.0;

        let (assign5620_e2037,) = {
    if (locals.var_guard17 != 0.0) {
        (1.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5620_e2037;
        locals.var_uc_rdrcx_rv = 0.0;

        locals.var_uc_ndepm = p.p340;
        locals.var_uc_ndepm_dn0 = 0.0;
        locals.var_uc_ndepm_dn2 = 0.0;
        locals.var_uc_ndepm_dn4 = 0.0;
        locals.var_uc_ndepm_dn5 = 0.0;
        locals.var_uc_ndepm_dn6 = 0.0;
        locals.var_uc_ndepm_dn7 = 0.0;
        locals.var_uc_ndepm_dn8 = 0.0;
        locals.var_uc_ndepm_dn9 = 0.0;
        locals.var_uc_ndepm_dn10 = 0.0;
        locals.var_uc_ndepm_dn11 = 0.0;
        locals.var_uc_ndepm_dn14 = 0.0;
        locals.var_uc_ndepm_rv = 0.0;

        locals.var_uc_depthn = p.p343;
        locals.var_uc_depthn_dn0 = 0.0;
        locals.var_uc_depthn_dn2 = 0.0;
        locals.var_uc_depthn_dn4 = 0.0;
        locals.var_uc_depthn_dn5 = 0.0;
        locals.var_uc_depthn_dn6 = 0.0;
        locals.var_uc_depthn_dn7 = 0.0;
        locals.var_uc_depthn_dn8 = 0.0;
        locals.var_uc_depthn_dn9 = 0.0;
        locals.var_uc_depthn_dn10 = 0.0;
        locals.var_uc_depthn_dn11 = 0.0;
        locals.var_uc_depthn_dn14 = 0.0;
        locals.var_uc_depthn_rv = 0.0;

        locals.var_uc_codep = p.p42;
        locals.var_uc_codep_rv = 0.0;

        locals.var_uc_depmueback0 = p.p354;
        locals.var_uc_depmueback0_dn0 = 0.0;
        locals.var_uc_depmueback0_dn2 = 0.0;
        locals.var_uc_depmueback0_dn4 = 0.0;
        locals.var_uc_depmueback0_dn5 = 0.0;
        locals.var_uc_depmueback0_dn6 = 0.0;
        locals.var_uc_depmueback0_dn7 = 0.0;
        locals.var_uc_depmueback0_dn8 = 0.0;
        locals.var_uc_depmueback0_dn9 = 0.0;
        locals.var_uc_depmueback0_dn10 = 0.0;
        locals.var_uc_depmueback0_dn11 = 0.0;
        locals.var_uc_depmueback0_dn14 = 0.0;
        locals.var_uc_depmueback0_rv = 0.0;

        locals.var_uc_depmueback1 = p.p355;
        locals.var_uc_depmueback1_dn0 = 0.0;
        locals.var_uc_depmueback1_dn2 = 0.0;
        locals.var_uc_depmueback1_dn4 = 0.0;
        locals.var_uc_depmueback1_dn5 = 0.0;
        locals.var_uc_depmueback1_dn6 = 0.0;
        locals.var_uc_depmueback1_dn7 = 0.0;
        locals.var_uc_depmueback1_dn8 = 0.0;
        locals.var_uc_depmueback1_dn9 = 0.0;
        locals.var_uc_depmueback1_dn10 = 0.0;
        locals.var_uc_depmueback1_dn11 = 0.0;
        locals.var_uc_depmueback1_dn14 = 0.0;
        locals.var_uc_depmueback1_rv = 0.0;

        locals.var_uc_depmue0 = p.p346;
        locals.var_uc_depmue0_dn0 = 0.0;
        locals.var_uc_depmue0_dn2 = 0.0;
        locals.var_uc_depmue0_dn4 = 0.0;
        locals.var_uc_depmue0_dn5 = 0.0;
        locals.var_uc_depmue0_dn6 = 0.0;
        locals.var_uc_depmue0_dn7 = 0.0;
        locals.var_uc_depmue0_dn8 = 0.0;
        locals.var_uc_depmue0_dn9 = 0.0;
        locals.var_uc_depmue0_dn10 = 0.0;
        locals.var_uc_depmue0_dn11 = 0.0;
        locals.var_uc_depmue0_dn14 = 0.0;
        locals.var_uc_depmue0_rv = 0.0;

        locals.var_uc_depmue1 = p.p349;
        locals.var_uc_depmue1_dn0 = 0.0;
        locals.var_uc_depmue1_dn2 = 0.0;
        locals.var_uc_depmue1_dn4 = 0.0;
        locals.var_uc_depmue1_dn5 = 0.0;
        locals.var_uc_depmue1_dn6 = 0.0;
        locals.var_uc_depmue1_dn7 = 0.0;
        locals.var_uc_depmue1_dn8 = 0.0;
        locals.var_uc_depmue1_dn9 = 0.0;
        locals.var_uc_depmue1_dn10 = 0.0;
        locals.var_uc_depmue1_dn11 = 0.0;
        locals.var_uc_depmue1_dn14 = 0.0;
        locals.var_uc_depmue1_rv = 0.0;

        locals.var_uc_depmue2 = p.p352;
        locals.var_uc_depmue2_dn0 = 0.0;
        locals.var_uc_depmue2_dn2 = 0.0;
        locals.var_uc_depmue2_dn4 = 0.0;
        locals.var_uc_depmue2_dn5 = 0.0;
        locals.var_uc_depmue2_dn6 = 0.0;
        locals.var_uc_depmue2_dn7 = 0.0;
        locals.var_uc_depmue2_dn8 = 0.0;
        locals.var_uc_depmue2_dn9 = 0.0;
        locals.var_uc_depmue2_dn10 = 0.0;
        locals.var_uc_depmue2_dn11 = 0.0;
        locals.var_uc_depmue2_dn14 = 0.0;
        locals.var_uc_depmue2_rv = 0.0;

        locals.var_uc_depleak = p.p360;
        locals.var_uc_depleak_dn0 = 0.0;
        locals.var_uc_depleak_dn2 = 0.0;
        locals.var_uc_depleak_dn4 = 0.0;
        locals.var_uc_depleak_dn5 = 0.0;
        locals.var_uc_depleak_dn6 = 0.0;
        locals.var_uc_depleak_dn7 = 0.0;
        locals.var_uc_depleak_dn8 = 0.0;
        locals.var_uc_depleak_dn9 = 0.0;
        locals.var_uc_depleak_dn10 = 0.0;
        locals.var_uc_depleak_dn11 = 0.0;
        locals.var_uc_depleak_dn14 = 0.0;
        locals.var_uc_depleak_rv = 0.0;

        locals.var_uc_depvmax = p.p367;
        locals.var_uc_depvmax_dn0 = 0.0;
        locals.var_uc_depvmax_dn2 = 0.0;
        locals.var_uc_depvmax_dn4 = 0.0;
        locals.var_uc_depvmax_dn5 = 0.0;
        locals.var_uc_depvmax_dn6 = 0.0;
        locals.var_uc_depvmax_dn7 = 0.0;
        locals.var_uc_depvmax_dn8 = 0.0;
        locals.var_uc_depvmax_dn9 = 0.0;
        locals.var_uc_depvmax_dn10 = 0.0;
        locals.var_uc_depvmax_dn11 = 0.0;
        locals.var_uc_depvmax_dn14 = 0.0;
        locals.var_uc_depvmax_rv = 0.0;

        locals.var_uc_depwlp = p.p364;
        locals.var_uc_depwlp_dn0 = 0.0;
        locals.var_uc_depwlp_dn2 = 0.0;
        locals.var_uc_depwlp_dn4 = 0.0;
        locals.var_uc_depwlp_dn5 = 0.0;
        locals.var_uc_depwlp_dn6 = 0.0;
        locals.var_uc_depwlp_dn7 = 0.0;
        locals.var_uc_depwlp_dn8 = 0.0;
        locals.var_uc_depwlp_dn9 = 0.0;
        locals.var_uc_depwlp_dn10 = 0.0;
        locals.var_uc_depwlp_dn11 = 0.0;
        locals.var_uc_depwlp_dn14 = 0.0;
        locals.var_uc_depwlp_rv = 0.0;

        locals.var_uc_depmueph1 = p.p377;
        locals.var_uc_depmueph1_rv = 0.0;

        locals.var_uc_depvdsef1 = p.p370;
        locals.var_uc_depvdsef1_dn0 = 0.0;
        locals.var_uc_depvdsef1_dn2 = 0.0;
        locals.var_uc_depvdsef1_dn4 = 0.0;
        locals.var_uc_depvdsef1_dn5 = 0.0;
        locals.var_uc_depvdsef1_dn6 = 0.0;
        locals.var_uc_depvdsef1_dn7 = 0.0;
        locals.var_uc_depvdsef1_dn8 = 0.0;
        locals.var_uc_depvdsef1_dn9 = 0.0;
        locals.var_uc_depvdsef1_dn10 = 0.0;
        locals.var_uc_depvdsef1_dn11 = 0.0;
        locals.var_uc_depvdsef1_dn14 = 0.0;
        locals.var_uc_depvdsef1_rv = 0.0;

        locals.var_uc_depvdsef2 = p.p371;
        locals.var_uc_depvdsef2_dn0 = 0.0;
        locals.var_uc_depvdsef2_dn2 = 0.0;
        locals.var_uc_depvdsef2_dn4 = 0.0;
        locals.var_uc_depvdsef2_dn5 = 0.0;
        locals.var_uc_depvdsef2_dn6 = 0.0;
        locals.var_uc_depvdsef2_dn7 = 0.0;
        locals.var_uc_depvdsef2_dn8 = 0.0;
        locals.var_uc_depvdsef2_dn9 = 0.0;
        locals.var_uc_depvdsef2_dn10 = 0.0;
        locals.var_uc_depvdsef2_dn11 = 0.0;
        locals.var_uc_depvdsef2_dn14 = 0.0;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign6710_e2710: f64 = if ((locals.var_uc_codep < 3.0) && (locals.var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6710_e2710;
        locals.var_guard112_rv = 0.0;

        let assign6740_e2723: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6740_e2723;
        locals.var_guard115_rv = 0.0;

        let (assign6750_e2729, assign6750_e2729_d_n0, assign6750_e2729_d_n2, assign6750_e2729_d_n4, assign6750_e2729_d_n5, assign6750_e2729_d_n6, assign6750_e2729_d_n7, assign6750_e2729_d_n8, assign6750_e2729_d_n9, assign6750_e2729_d_n10, assign6750_e2729_d_n11, assign6750_e2729_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard115 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6750_e2729;
        locals.var_uc_ndepm_dn0 = assign6750_e2729_d_n0;
        locals.var_uc_ndepm_dn2 = assign6750_e2729_d_n2;
        locals.var_uc_ndepm_dn4 = assign6750_e2729_d_n4;
        locals.var_uc_ndepm_dn5 = assign6750_e2729_d_n5;
        locals.var_uc_ndepm_dn6 = assign6750_e2729_d_n6;
        locals.var_uc_ndepm_dn7 = assign6750_e2729_d_n7;
        locals.var_uc_ndepm_dn8 = assign6750_e2729_d_n8;
        locals.var_uc_ndepm_dn9 = assign6750_e2729_d_n9;
        locals.var_uc_ndepm_dn10 = assign6750_e2729_d_n10;
        locals.var_uc_ndepm_dn11 = assign6750_e2729_d_n11;
        locals.var_uc_ndepm_dn14 = assign6750_e2729_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6760_e2732: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6760_e2732;
        locals.var_guard116_rv = 0.0;

        let (assign6770_e2738, assign6770_e2738_d_n0, assign6770_e2738_d_n2, assign6770_e2738_d_n4, assign6770_e2738_d_n5, assign6770_e2738_d_n6, assign6770_e2738_d_n7, assign6770_e2738_d_n8, assign6770_e2738_d_n9, assign6770_e2738_d_n10, assign6770_e2738_d_n11, assign6770_e2738_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard116 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6770_e2738;
        locals.var_uc_ndepm_dn0 = assign6770_e2738_d_n0;
        locals.var_uc_ndepm_dn2 = assign6770_e2738_d_n2;
        locals.var_uc_ndepm_dn4 = assign6770_e2738_d_n4;
        locals.var_uc_ndepm_dn5 = assign6770_e2738_d_n5;
        locals.var_uc_ndepm_dn6 = assign6770_e2738_d_n6;
        locals.var_uc_ndepm_dn7 = assign6770_e2738_d_n7;
        locals.var_uc_ndepm_dn8 = assign6770_e2738_d_n8;
        locals.var_uc_ndepm_dn9 = assign6770_e2738_d_n9;
        locals.var_uc_ndepm_dn10 = assign6770_e2738_d_n10;
        locals.var_uc_ndepm_dn11 = assign6770_e2738_d_n11;
        locals.var_uc_ndepm_dn14 = assign6770_e2738_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6800_e2751: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6800_e2751;
        locals.var_guard119_rv = 0.0;

        let (assign6810_e2757, assign6810_e2757_d_n0, assign6810_e2757_d_n2, assign6810_e2757_d_n4, assign6810_e2757_d_n5, assign6810_e2757_d_n6, assign6810_e2757_d_n7, assign6810_e2757_d_n8, assign6810_e2757_d_n9, assign6810_e2757_d_n10, assign6810_e2757_d_n11, assign6810_e2757_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard119 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6810_e2757;
        locals.var_uc_depthn_dn0 = assign6810_e2757_d_n0;
        locals.var_uc_depthn_dn2 = assign6810_e2757_d_n2;
        locals.var_uc_depthn_dn4 = assign6810_e2757_d_n4;
        locals.var_uc_depthn_dn5 = assign6810_e2757_d_n5;
        locals.var_uc_depthn_dn6 = assign6810_e2757_d_n6;
        locals.var_uc_depthn_dn7 = assign6810_e2757_d_n7;
        locals.var_uc_depthn_dn8 = assign6810_e2757_d_n8;
        locals.var_uc_depthn_dn9 = assign6810_e2757_d_n9;
        locals.var_uc_depthn_dn10 = assign6810_e2757_d_n10;
        locals.var_uc_depthn_dn11 = assign6810_e2757_d_n11;
        locals.var_uc_depthn_dn14 = assign6810_e2757_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign6820_e2760: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6820_e2760;
        locals.var_guard120_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6830_e2766, assign6830_e2766_d_n0, assign6830_e2766_d_n2, assign6830_e2766_d_n4, assign6830_e2766_d_n5, assign6830_e2766_d_n6, assign6830_e2766_d_n7, assign6830_e2766_d_n8, assign6830_e2766_d_n9, assign6830_e2766_d_n10, assign6830_e2766_d_n11, assign6830_e2766_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard120 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6830_e2766;
        locals.var_uc_depthn_dn0 = assign6830_e2766_d_n0;
        locals.var_uc_depthn_dn2 = assign6830_e2766_d_n2;
        locals.var_uc_depthn_dn4 = assign6830_e2766_d_n4;
        locals.var_uc_depthn_dn5 = assign6830_e2766_d_n5;
        locals.var_uc_depthn_dn6 = assign6830_e2766_d_n6;
        locals.var_uc_depthn_dn7 = assign6830_e2766_d_n7;
        locals.var_uc_depthn_dn8 = assign6830_e2766_d_n8;
        locals.var_uc_depthn_dn9 = assign6830_e2766_d_n9;
        locals.var_uc_depthn_dn10 = assign6830_e2766_d_n10;
        locals.var_uc_depthn_dn11 = assign6830_e2766_d_n11;
        locals.var_uc_depthn_dn14 = assign6830_e2766_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign6860_e2779: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign6860_e2779;
        locals.var_guard123_rv = 0.0;

        let (assign6870_e2785, assign6870_e2785_d_n0, assign6870_e2785_d_n2, assign6870_e2785_d_n4, assign6870_e2785_d_n5, assign6870_e2785_d_n6, assign6870_e2785_d_n7, assign6870_e2785_d_n8, assign6870_e2785_d_n9, assign6870_e2785_d_n10, assign6870_e2785_d_n11, assign6870_e2785_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard123 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6870_e2785;
        locals.var_uc_depmue0_dn0 = assign6870_e2785_d_n0;
        locals.var_uc_depmue0_dn2 = assign6870_e2785_d_n2;
        locals.var_uc_depmue0_dn4 = assign6870_e2785_d_n4;
        locals.var_uc_depmue0_dn5 = assign6870_e2785_d_n5;
        locals.var_uc_depmue0_dn6 = assign6870_e2785_d_n6;
        locals.var_uc_depmue0_dn7 = assign6870_e2785_d_n7;
        locals.var_uc_depmue0_dn8 = assign6870_e2785_d_n8;
        locals.var_uc_depmue0_dn9 = assign6870_e2785_d_n9;
        locals.var_uc_depmue0_dn10 = assign6870_e2785_d_n10;
        locals.var_uc_depmue0_dn11 = assign6870_e2785_d_n11;
        locals.var_uc_depmue0_dn14 = assign6870_e2785_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6880_e2788: f64 = if locals.var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign6880_e2788;
        locals.var_guard124_rv = 0.0;

        let (assign6890_e2794, assign6890_e2794_d_n0, assign6890_e2794_d_n2, assign6890_e2794_d_n4, assign6890_e2794_d_n5, assign6890_e2794_d_n6, assign6890_e2794_d_n7, assign6890_e2794_d_n8, assign6890_e2794_d_n9, assign6890_e2794_d_n10, assign6890_e2794_d_n11, assign6890_e2794_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard124 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6890_e2794;
        locals.var_uc_depmue0_dn0 = assign6890_e2794_d_n0;
        locals.var_uc_depmue0_dn2 = assign6890_e2794_d_n2;
        locals.var_uc_depmue0_dn4 = assign6890_e2794_d_n4;
        locals.var_uc_depmue0_dn5 = assign6890_e2794_d_n5;
        locals.var_uc_depmue0_dn6 = assign6890_e2794_d_n6;
        locals.var_uc_depmue0_dn7 = assign6890_e2794_d_n7;
        locals.var_uc_depmue0_dn8 = assign6890_e2794_d_n8;
        locals.var_uc_depmue0_dn9 = assign6890_e2794_d_n9;
        locals.var_uc_depmue0_dn10 = assign6890_e2794_d_n10;
        locals.var_uc_depmue0_dn11 = assign6890_e2794_d_n11;
        locals.var_uc_depmue0_dn14 = assign6890_e2794_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6920_e2807: f64 = if locals.var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6920_e2807;
        locals.var_guard127_rv = 0.0;

        let (assign6930_e2813, assign6930_e2813_d_n0, assign6930_e2813_d_n2, assign6930_e2813_d_n4, assign6930_e2813_d_n5, assign6930_e2813_d_n6, assign6930_e2813_d_n7, assign6930_e2813_d_n8, assign6930_e2813_d_n9, assign6930_e2813_d_n10, assign6930_e2813_d_n11, assign6930_e2813_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard127 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6930_e2813;
        locals.var_uc_depmueback0_dn0 = assign6930_e2813_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6930_e2813_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6930_e2813_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6930_e2813_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6930_e2813_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6930_e2813_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6930_e2813_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6930_e2813_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6930_e2813_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6930_e2813_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6930_e2813_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign6940_e2816: f64 = if locals.var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6940_e2816;
        locals.var_guard128_rv = 0.0;

        let (assign6950_e2822, assign6950_e2822_d_n0, assign6950_e2822_d_n2, assign6950_e2822_d_n4, assign6950_e2822_d_n5, assign6950_e2822_d_n6, assign6950_e2822_d_n7, assign6950_e2822_d_n8, assign6950_e2822_d_n9, assign6950_e2822_d_n10, assign6950_e2822_d_n11, assign6950_e2822_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard128 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6950_e2822;
        locals.var_uc_depmueback0_dn0 = assign6950_e2822_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6950_e2822_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6950_e2822_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6950_e2822_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6950_e2822_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6950_e2822_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6950_e2822_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6950_e2822_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6950_e2822_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6950_e2822_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6950_e2822_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign6980_e2835: f64 = if locals.var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6980_e2835;
        locals.var_guard131_rv = 0.0;

        let (assign6990_e2841,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard131 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6990_e2841;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7000_e2844: f64 = if locals.var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7000_e2844;
        locals.var_guard132_rv = 0.0;

        let (assign7010_e2850,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard132 != 0.0)) {
        (100000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7010_e2850;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7040_e2863: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign7040_e2863;
        locals.var_guard135_rv = 0.0;

        let (assign7050_e2869, assign7050_e2869_d_n0, assign7050_e2869_d_n2, assign7050_e2869_d_n4, assign7050_e2869_d_n5, assign7050_e2869_d_n6, assign7050_e2869_d_n7, assign7050_e2869_d_n8, assign7050_e2869_d_n9, assign7050_e2869_d_n10, assign7050_e2869_d_n11, assign7050_e2869_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard135 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7050_e2869;
        locals.var_uc_depvdsef2_dn0 = assign7050_e2869_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7050_e2869_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7050_e2869_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7050_e2869_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7050_e2869_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7050_e2869_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7050_e2869_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7050_e2869_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7050_e2869_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7050_e2869_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7050_e2869_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7060_e2872: f64 = if locals.var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign7060_e2872;
        locals.var_guard136_rv = 0.0;

        let (assign7070_e2878, assign7070_e2878_d_n0, assign7070_e2878_d_n2, assign7070_e2878_d_n4, assign7070_e2878_d_n5, assign7070_e2878_d_n6, assign7070_e2878_d_n7, assign7070_e2878_d_n8, assign7070_e2878_d_n9, assign7070_e2878_d_n10, assign7070_e2878_d_n11, assign7070_e2878_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard136 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7070_e2878;
        locals.var_uc_depvdsef2_dn0 = assign7070_e2878_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7070_e2878_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7070_e2878_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7070_e2878_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7070_e2878_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7070_e2878_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7070_e2878_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7070_e2878_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7070_e2878_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7070_e2878_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7070_e2878_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7100_e2891: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7100_e2891;
        locals.var_guard139_rv = 0.0;

        let (assign7110_e2897, assign7110_e2897_d_n0, assign7110_e2897_d_n2, assign7110_e2897_d_n4, assign7110_e2897_d_n5, assign7110_e2897_d_n6, assign7110_e2897_d_n7, assign7110_e2897_d_n8, assign7110_e2897_d_n9, assign7110_e2897_d_n10, assign7110_e2897_d_n11, assign7110_e2897_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard139 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7110_e2897;
        locals.var_uc_depleak_dn0 = assign7110_e2897_d_n0;
        locals.var_uc_depleak_dn2 = assign7110_e2897_d_n2;
        locals.var_uc_depleak_dn4 = assign7110_e2897_d_n4;
        locals.var_uc_depleak_dn5 = assign7110_e2897_d_n5;
        locals.var_uc_depleak_dn6 = assign7110_e2897_d_n6;
        locals.var_uc_depleak_dn7 = assign7110_e2897_d_n7;
        locals.var_uc_depleak_dn8 = assign7110_e2897_d_n8;
        locals.var_uc_depleak_dn9 = assign7110_e2897_d_n9;
        locals.var_uc_depleak_dn10 = assign7110_e2897_d_n10;
        locals.var_uc_depleak_dn11 = assign7110_e2897_d_n11;
        locals.var_uc_depleak_dn14 = assign7110_e2897_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7120_e2900: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign7120_e2900;
        locals.var_guard140_rv = 0.0;

        let (assign7130_e2906, assign7130_e2906_d_n0, assign7130_e2906_d_n2, assign7130_e2906_d_n4, assign7130_e2906_d_n5, assign7130_e2906_d_n6, assign7130_e2906_d_n7, assign7130_e2906_d_n8, assign7130_e2906_d_n9, assign7130_e2906_d_n10, assign7130_e2906_d_n11, assign7130_e2906_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard140 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7130_e2906;
        locals.var_uc_depleak_dn0 = assign7130_e2906_d_n0;
        locals.var_uc_depleak_dn2 = assign7130_e2906_d_n2;
        locals.var_uc_depleak_dn4 = assign7130_e2906_d_n4;
        locals.var_uc_depleak_dn5 = assign7130_e2906_d_n5;
        locals.var_uc_depleak_dn6 = assign7130_e2906_d_n6;
        locals.var_uc_depleak_dn7 = assign7130_e2906_d_n7;
        locals.var_uc_depleak_dn8 = assign7130_e2906_d_n8;
        locals.var_uc_depleak_dn9 = assign7130_e2906_d_n9;
        locals.var_uc_depleak_dn10 = assign7130_e2906_d_n10;
        locals.var_uc_depleak_dn11 = assign7130_e2906_d_n11;
        locals.var_uc_depleak_dn14 = assign7130_e2906_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7140_e2909: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign7140_e2909;
        locals.var_guard141_rv = 0.0;

        let assign7170_e2922: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7170_e2922;
        locals.var_guard144_rv = 0.0;

        let (assign7180_e2931, assign7180_e2931_d_n0, assign7180_e2931_d_n2, assign7180_e2931_d_n4, assign7180_e2931_d_n5, assign7180_e2931_d_n6, assign7180_e2931_d_n7, assign7180_e2931_d_n8, assign7180_e2931_d_n9, assign7180_e2931_d_n10, assign7180_e2931_d_n11, assign7180_e2931_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard144 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7180_e2931;
        locals.var_uc_ndepm_dn0 = assign7180_e2931_d_n0;
        locals.var_uc_ndepm_dn2 = assign7180_e2931_d_n2;
        locals.var_uc_ndepm_dn4 = assign7180_e2931_d_n4;
        locals.var_uc_ndepm_dn5 = assign7180_e2931_d_n5;
        locals.var_uc_ndepm_dn6 = assign7180_e2931_d_n6;
        locals.var_uc_ndepm_dn7 = assign7180_e2931_d_n7;
        locals.var_uc_ndepm_dn8 = assign7180_e2931_d_n8;
        locals.var_uc_ndepm_dn9 = assign7180_e2931_d_n9;
        locals.var_uc_ndepm_dn10 = assign7180_e2931_d_n10;
        locals.var_uc_ndepm_dn11 = assign7180_e2931_d_n11;
        locals.var_uc_ndepm_dn14 = assign7180_e2931_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7190_e2934: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7190_e2934;
        locals.var_guard145_rv = 0.0;

        let (assign7200_e2943, assign7200_e2943_d_n0, assign7200_e2943_d_n2, assign7200_e2943_d_n4, assign7200_e2943_d_n5, assign7200_e2943_d_n6, assign7200_e2943_d_n7, assign7200_e2943_d_n8, assign7200_e2943_d_n9, assign7200_e2943_d_n10, assign7200_e2943_d_n11, assign7200_e2943_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard145 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7200_e2943;
        locals.var_uc_ndepm_dn0 = assign7200_e2943_d_n0;
        locals.var_uc_ndepm_dn2 = assign7200_e2943_d_n2;
        locals.var_uc_ndepm_dn4 = assign7200_e2943_d_n4;
        locals.var_uc_ndepm_dn5 = assign7200_e2943_d_n5;
        locals.var_uc_ndepm_dn6 = assign7200_e2943_d_n6;
        locals.var_uc_ndepm_dn7 = assign7200_e2943_d_n7;
        locals.var_uc_ndepm_dn8 = assign7200_e2943_d_n8;
        locals.var_uc_ndepm_dn9 = assign7200_e2943_d_n9;
        locals.var_uc_ndepm_dn10 = assign7200_e2943_d_n10;
        locals.var_uc_ndepm_dn11 = assign7200_e2943_d_n11;
        locals.var_uc_ndepm_dn14 = assign7200_e2943_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7230_e2956: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7230_e2956;
        locals.var_guard148_rv = 0.0;

        let (assign7240_e2965, assign7240_e2965_d_n0, assign7240_e2965_d_n2, assign7240_e2965_d_n4, assign7240_e2965_d_n5, assign7240_e2965_d_n6, assign7240_e2965_d_n7, assign7240_e2965_d_n8, assign7240_e2965_d_n9, assign7240_e2965_d_n10, assign7240_e2965_d_n11, assign7240_e2965_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard148 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7240_e2965;
        locals.var_uc_depthn_dn0 = assign7240_e2965_d_n0;
        locals.var_uc_depthn_dn2 = assign7240_e2965_d_n2;
        locals.var_uc_depthn_dn4 = assign7240_e2965_d_n4;
        locals.var_uc_depthn_dn5 = assign7240_e2965_d_n5;
        locals.var_uc_depthn_dn6 = assign7240_e2965_d_n6;
        locals.var_uc_depthn_dn7 = assign7240_e2965_d_n7;
        locals.var_uc_depthn_dn8 = assign7240_e2965_d_n8;
        locals.var_uc_depthn_dn9 = assign7240_e2965_d_n9;
        locals.var_uc_depthn_dn10 = assign7240_e2965_d_n10;
        locals.var_uc_depthn_dn11 = assign7240_e2965_d_n11;
        locals.var_uc_depthn_dn14 = assign7240_e2965_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign7250_e2968: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7250_e2968;
        locals.var_guard149_rv = 0.0;

        let (assign7260_e2977, assign7260_e2977_d_n0, assign7260_e2977_d_n2, assign7260_e2977_d_n4, assign7260_e2977_d_n5, assign7260_e2977_d_n6, assign7260_e2977_d_n7, assign7260_e2977_d_n8, assign7260_e2977_d_n9, assign7260_e2977_d_n10, assign7260_e2977_d_n11, assign7260_e2977_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard149 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7260_e2977;
        locals.var_uc_depthn_dn0 = assign7260_e2977_d_n0;
        locals.var_uc_depthn_dn2 = assign7260_e2977_d_n2;
        locals.var_uc_depthn_dn4 = assign7260_e2977_d_n4;
        locals.var_uc_depthn_dn5 = assign7260_e2977_d_n5;
        locals.var_uc_depthn_dn6 = assign7260_e2977_d_n6;
        locals.var_uc_depthn_dn7 = assign7260_e2977_d_n7;
        locals.var_uc_depthn_dn8 = assign7260_e2977_d_n8;
        locals.var_uc_depthn_dn9 = assign7260_e2977_d_n9;
        locals.var_uc_depthn_dn10 = assign7260_e2977_d_n10;
        locals.var_uc_depthn_dn11 = assign7260_e2977_d_n11;
        locals.var_uc_depthn_dn14 = assign7260_e2977_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign7290_e2990: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7290_e2990;
        locals.var_guard152_rv = 0.0;

        let (assign7300_e2999, assign7300_e2999_d_n0, assign7300_e2999_d_n2, assign7300_e2999_d_n4, assign7300_e2999_d_n5, assign7300_e2999_d_n6, assign7300_e2999_d_n7, assign7300_e2999_d_n8, assign7300_e2999_d_n9, assign7300_e2999_d_n10, assign7300_e2999_d_n11, assign7300_e2999_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7300_e2999;
        locals.var_uc_depmue0_dn0 = assign7300_e2999_d_n0;
        locals.var_uc_depmue0_dn2 = assign7300_e2999_d_n2;
        locals.var_uc_depmue0_dn4 = assign7300_e2999_d_n4;
        locals.var_uc_depmue0_dn5 = assign7300_e2999_d_n5;
        locals.var_uc_depmue0_dn6 = assign7300_e2999_d_n6;
        locals.var_uc_depmue0_dn7 = assign7300_e2999_d_n7;
        locals.var_uc_depmue0_dn8 = assign7300_e2999_d_n8;
        locals.var_uc_depmue0_dn9 = assign7300_e2999_d_n9;
        locals.var_uc_depmue0_dn10 = assign7300_e2999_d_n10;
        locals.var_uc_depmue0_dn11 = assign7300_e2999_d_n11;
        locals.var_uc_depmue0_dn14 = assign7300_e2999_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7310_e3002: f64 = if locals.var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign7310_e3002;
        locals.var_guard153_rv = 0.0;

        let (assign7320_e3011, assign7320_e3011_d_n0, assign7320_e3011_d_n2, assign7320_e3011_d_n4, assign7320_e3011_d_n5, assign7320_e3011_d_n6, assign7320_e3011_d_n7, assign7320_e3011_d_n8, assign7320_e3011_d_n9, assign7320_e3011_d_n10, assign7320_e3011_d_n11, assign7320_e3011_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard153 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7320_e3011;
        locals.var_uc_depmue0_dn0 = assign7320_e3011_d_n0;
        locals.var_uc_depmue0_dn2 = assign7320_e3011_d_n2;
        locals.var_uc_depmue0_dn4 = assign7320_e3011_d_n4;
        locals.var_uc_depmue0_dn5 = assign7320_e3011_d_n5;
        locals.var_uc_depmue0_dn6 = assign7320_e3011_d_n6;
        locals.var_uc_depmue0_dn7 = assign7320_e3011_d_n7;
        locals.var_uc_depmue0_dn8 = assign7320_e3011_d_n8;
        locals.var_uc_depmue0_dn9 = assign7320_e3011_d_n9;
        locals.var_uc_depmue0_dn10 = assign7320_e3011_d_n10;
        locals.var_uc_depmue0_dn11 = assign7320_e3011_d_n11;
        locals.var_uc_depmue0_dn14 = assign7320_e3011_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7350_e3024: f64 = if locals.var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign7350_e3024;
        locals.var_guard156_rv = 0.0;

        let (assign7360_e3033,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard156 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7360_e3033;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7370_e3036: f64 = if locals.var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign7370_e3036;
        locals.var_guard157_rv = 0.0;

        let (assign7380_e3045,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard157 != 0.0)) {
        (2000000000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7380_e3045;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7410_e3058: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign7410_e3058;
        locals.var_guard160_rv = 0.0;

        let (assign7420_e3067, assign7420_e3067_d_n0, assign7420_e3067_d_n2, assign7420_e3067_d_n4, assign7420_e3067_d_n5, assign7420_e3067_d_n6, assign7420_e3067_d_n7, assign7420_e3067_d_n8, assign7420_e3067_d_n9, assign7420_e3067_d_n10, assign7420_e3067_d_n11, assign7420_e3067_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard160 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7420_e3067;
        locals.var_uc_depleak_dn0 = assign7420_e3067_d_n0;
        locals.var_uc_depleak_dn2 = assign7420_e3067_d_n2;
        locals.var_uc_depleak_dn4 = assign7420_e3067_d_n4;
        locals.var_uc_depleak_dn5 = assign7420_e3067_d_n5;
        locals.var_uc_depleak_dn6 = assign7420_e3067_d_n6;
        locals.var_uc_depleak_dn7 = assign7420_e3067_d_n7;
        locals.var_uc_depleak_dn8 = assign7420_e3067_d_n8;
        locals.var_uc_depleak_dn9 = assign7420_e3067_d_n9;
        locals.var_uc_depleak_dn10 = assign7420_e3067_d_n10;
        locals.var_uc_depleak_dn11 = assign7420_e3067_d_n11;
        locals.var_uc_depleak_dn14 = assign7420_e3067_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7430_e3070: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign7430_e3070;
        locals.var_guard161_rv = 0.0;

        let (assign7440_e3079, assign7440_e3079_d_n0, assign7440_e3079_d_n2, assign7440_e3079_d_n4, assign7440_e3079_d_n5, assign7440_e3079_d_n6, assign7440_e3079_d_n7, assign7440_e3079_d_n8, assign7440_e3079_d_n9, assign7440_e3079_d_n10, assign7440_e3079_d_n11, assign7440_e3079_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard161 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7440_e3079;
        locals.var_uc_depleak_dn0 = assign7440_e3079_d_n0;
        locals.var_uc_depleak_dn2 = assign7440_e3079_d_n2;
        locals.var_uc_depleak_dn4 = assign7440_e3079_d_n4;
        locals.var_uc_depleak_dn5 = assign7440_e3079_d_n5;
        locals.var_uc_depleak_dn6 = assign7440_e3079_d_n6;
        locals.var_uc_depleak_dn7 = assign7440_e3079_d_n7;
        locals.var_uc_depleak_dn8 = assign7440_e3079_d_n8;
        locals.var_uc_depleak_dn9 = assign7440_e3079_d_n9;
        locals.var_uc_depleak_dn10 = assign7440_e3079_d_n10;
        locals.var_uc_depleak_dn11 = assign7440_e3079_d_n11;
        locals.var_uc_depleak_dn14 = assign7440_e3079_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        locals.var_uc_toxb = p.p96;
        locals.var_uc_toxb_rv = 0.0;

        let assign7540_e3117: f64 = if locals.var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign7540_e3117;
        locals.var_guard170_rv = 0.0;

        let (assign7550_e3121,) = {
    if (locals.var_guard170 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7550_e3121;
        locals.var_uc_toxb_rv = 0.0;

        let assign7560_e3124: f64 = if locals.var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign7560_e3124;
        locals.var_guard171_rv = 0.0;

        let (assign7570_e3128,) = {
    if (locals.var_guard171 != 0.0) {
        (5e-7,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7570_e3128;
        locals.var_uc_toxb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7580_e3132: f64 = (100.0_f64).powf(p.p122);
        let assign7580_e3133: f64 = (p.p120 / assign7580_e3132);
        locals.var_mks_ll = assign7580_e3133;
        locals.var_mks_ll_rv = 0.0;

        let assign7590_e3137: f64 = (100.0_f64).powf(p.p129);
        let assign7590_e3138: f64 = (p.p123 / assign7590_e3137);
        locals.var_mks_wl = assign7590_e3138;
        locals.var_mks_wl_rv = 0.0;

        let assign7600_e3142: f64 = (100.0_f64).powf(p.p199);
        let assign7600_e3143: f64 = (p.p198 / assign7600_e3142);
        locals.var_mks_svgsl = assign7600_e3143;
        locals.var_mks_svgsl_rv = 0.0;

        let assign7610_e3147: f64 = (100.0_f64).powf(p.p201);
        let assign7610_e3148: f64 = (p.p200 / assign7610_e3147);
        locals.var_mks_svgsw = assign7610_e3148;
        locals.var_mks_svgsw_rv = 0.0;

        let assign7620_e3152: f64 = (100.0_f64).powf(p.p184);
        let assign7620_e3153: f64 = (p.p183 / assign7620_e3152);
        locals.var_mks_svbsl = assign7620_e3153;
        locals.var_mks_svbsl_rv = 0.0;

        let assign7630_e3157: f64 = (100.0_f64).powf(p.p203);
        let assign7630_e3158: f64 = (p.p202 / assign7630_e3157);
        locals.var_mks_slgl = assign7630_e3158;
        locals.var_mks_slgl_rv = 0.0;

        let assign7640_e3162: f64 = (100.0_f64).powf(p.p191);
        let assign7640_e3163: f64 = (p.p190 / assign7640_e3162);
        locals.var_mks_sub1l = assign7640_e3163;
        locals.var_mks_sub1l_rv = 0.0;

        let assign7650_e3166: f64 = (p.p186 / 100.0);
        locals.var_mks_slg = assign7650_e3166;
        locals.var_mks_slg_rv = 0.0;

        let assign7660_e3169: f64 = (p.p192 / 100.0);
        locals.var_mks_sub2l = assign7660_e3169;
        locals.var_mks_sub2l_rv = 0.0;

        let assign7670_e3172: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign7670_e3172;
        locals.var_mks_subld2_rv = 0.0;

        let assign7680_e3175: f64 = (p.p311 / 100.0);
        locals.var_mks_rdtemp1 = assign7680_e3175;
        locals.var_mks_rdtemp1_rv = 0.0;

        let assign7690_e3178: f64 = (p.p312 / 100.0);
        locals.var_mks_rdtemp2 = assign7690_e3178;
        locals.var_mks_rdtemp2_rv = 0.0;

        let assign7700_e3181: f64 = (p.p313 / 100.0);
        locals.var_mks_rdvdtemp1 = assign7700_e3181;
        locals.var_mks_rdvdtemp1_rv = 0.0;

        let assign7710_e3184: f64 = (p.p314 / 100.0);
        locals.var_mks_rdvdtemp2 = assign7710_e3184;
        locals.var_mks_rdvdtemp2_rv = 0.0;

        let assign7720_e3187: f64 = (p.p336 / 1e-6);
        locals.var_mks_nsubsub = assign7720_e3187;
        locals.var_mks_nsubsub_rv = 0.0;

        let assign7730_e3190: f64 = (p.p255 * 100.0);
        locals.var_mks_glksd3 = assign7730_e3190;
        locals.var_mks_glksd3_rv = 0.0;

        let assign7740_e3193: f64 = (p.p248 * 100.0);
        locals.var_mks_gleak4 = assign7740_e3193;
        locals.var_mks_gleak4_rv = 0.0;

        let assign7750_e3196: f64 = (p.p249 * 100.0);
        locals.var_mks_gleak5 = assign7750_e3196;
        locals.var_mks_gleak5_rv = 0.0;

        let assign7760_e3199: f64 = (p.p251 / 10000.0);
        locals.var_mks_gleak7 = assign7760_e3199;
        locals.var_mks_gleak7_rv = 0.0;

        let assign7770_e3202: f64 = (p.p266 * 10000.0);
        locals.var_mks_cit = assign7770_e3202;
        locals.var_mks_cit_rv = 0.0;

        let assign7780_e3205: f64 = (p.p275 / 100.0);
        locals.var_mks_ovslp = assign7780_e3205;
        locals.var_mks_ovslp_rv = 0.0;

        let assign7790_e3208: f64 = (p.p272 / 10000.0);
        locals.var_mks_dly3 = assign7790_e3208;
        locals.var_mks_dly3_rv = 0.0;

        let assign7800_e3211: f64 = (p.p273 / 10000.0);
        locals.var_mks_dlyov = assign7800_e3211;
        locals.var_mks_dlyov_dn0 = 0.0;
        locals.var_mks_dlyov_dn2 = 0.0;
        locals.var_mks_dlyov_dn4 = 0.0;
        locals.var_mks_dlyov_dn5 = 0.0;
        locals.var_mks_dlyov_dn6 = 0.0;
        locals.var_mks_dlyov_dn7 = 0.0;
        locals.var_mks_dlyov_dn8 = 0.0;
        locals.var_mks_dlyov_dn9 = 0.0;
        locals.var_mks_dlyov_dn10 = 0.0;
        locals.var_mks_dlyov_dn11 = 0.0;
        locals.var_mks_dlyov_dn14 = 0.0;
        locals.var_mks_dlyov_rv = 0.0;

        let assign7820_e3217: f64 = (p.p409 / 10000.0);
        locals.var_mks_rdrmue = assign7820_e3217;
        locals.var_mks_rdrmue_rv = 0.0;

        let assign7830_e3220: f64 = (p.p412 / 100.0);
        locals.var_mks_rdrvmax = assign7830_e3220;
        locals.var_mks_rdrvmax_rv = 0.0;

        let assign7840_e3223: f64 = (p.p413 / 10000.0);
        locals.var_mks_rdrmues = assign7840_e3223;
        locals.var_mks_rdrmues_rv = 0.0;

        let assign7850_e3226: f64 = (p.p414 / 100.0);
        locals.var_mks_rdrvmaxs = assign7850_e3226;
        locals.var_mks_rdrvmaxs_rv = 0.0;

        let assign7860_e3229: f64 = (locals.var_uc_ndepm / 1e-6);
        locals.var_uc_ndepm = assign7860_e3229;
        locals.var_uc_ndepm_dn0 = (locals.var_uc_ndepm_dn0 / 1e-6);
        locals.var_uc_ndepm_dn2 = (locals.var_uc_ndepm_dn2 / 1e-6);
        locals.var_uc_ndepm_dn4 = (locals.var_uc_ndepm_dn4 / 1e-6);
        locals.var_uc_ndepm_dn5 = (locals.var_uc_ndepm_dn5 / 1e-6);
        locals.var_uc_ndepm_dn6 = (locals.var_uc_ndepm_dn6 / 1e-6);
        locals.var_uc_ndepm_dn7 = (locals.var_uc_ndepm_dn7 / 1e-6);
        locals.var_uc_ndepm_dn8 = (locals.var_uc_ndepm_dn8 / 1e-6);
        locals.var_uc_ndepm_dn9 = (locals.var_uc_ndepm_dn9 / 1e-6);
        locals.var_uc_ndepm_dn10 = (locals.var_uc_ndepm_dn10 / 1e-6);
        locals.var_uc_ndepm_dn11 = (locals.var_uc_ndepm_dn11 / 1e-6);
        locals.var_uc_ndepm_dn14 = (locals.var_uc_ndepm_dn14 / 1e-6);
        locals.var_uc_ndepm_rv = 0.0;

        let assign7870_e3232: f64 = (p.p453 / 1e-6);
        locals.var_uc_njunc = assign7870_e3232;
        locals.var_uc_njunc_rv = 0.0;

        let assign7880_e3235: f64 = (p.p274 + 273.15);
        locals.var_ktnom = assign7880_e3235;
        locals.var_ktnom_rv = 0.0;

        let assign7930_e3258: f64 = (p.p0 + p.p116);
        locals.var_lgate = assign7930_e3258;
        locals.var_lgate_rv = 0.0;

        let assign7940_e3261: f64 = (p.p1 / p.p7);
        let assign7940_e3263: f64 = (assign7940_e3261 + p.p117);
        locals.var_wgate = assign7940_e3263;
        locals.var_wgate_rv = 0.0;

        let assign8090_e3363: f64 = (locals.var_lgate * 1000000.0);
        locals.var_lg = assign8090_e3363;
        locals.var_lg_rv = 0.0;

        let assign8100_e3366: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign8100_e3366;
        locals.var_wg_rv = 0.0;

        let assign8110_e3369: f64 = (locals.var_lg).powf(p.p553);
        locals.var_lbin = assign8110_e3369;
        locals.var_lbin_rv = 0.0;

        let assign8120_e3372: f64 = (locals.var_wg).powf(p.p554);
        locals.var_wbin = assign8120_e3372;
        locals.var_wbin_rv = 0.0;

        let assign8130_e3375: f64 = (locals.var_lbin * locals.var_wbin);
        locals.var_lwbin = assign8130_e3375;
        locals.var_lwbin_rv = 0.0;

        let assign8140_e3379: f64 = (p.p555 / locals.var_lbin);
        let assign8140_e3380: f64 = (p.p89 + assign8140_e3379);
        let assign8140_e3383: f64 = (p.p643 / locals.var_wbin);
        let assign8140_e3384: f64 = (assign8140_e3380 + assign8140_e3383);
        let assign8140_e3387: f64 = (p.p731 / locals.var_lwbin);
        let assign8140_e3388: f64 = (assign8140_e3384 + assign8140_e3387);
        locals.var_uc_vmax = assign8140_e3388;
        locals.var_uc_vmax_rv = 0.0;

        let assign8150_e3392: f64 = (p.p556 / locals.var_lbin);
        let assign8150_e3393: f64 = (p.p92 + assign8150_e3392);
        let assign8150_e3396: f64 = (p.p644 / locals.var_wbin);
        let assign8150_e3397: f64 = (assign8150_e3393 + assign8150_e3396);
        let assign8150_e3400: f64 = (p.p732 / locals.var_lwbin);
        let assign8150_e3401: f64 = (assign8150_e3397 + assign8150_e3400);
        locals.var_uc_bgtmp1 = assign8150_e3401;
        locals.var_uc_bgtmp1_rv = 0.0;

        let assign8160_e3405: f64 = (p.p557 / locals.var_lbin);
        let assign8160_e3406: f64 = (p.p93 + assign8160_e3405);
        let assign8160_e3409: f64 = (p.p645 / locals.var_wbin);
        let assign8160_e3410: f64 = (assign8160_e3406 + assign8160_e3409);
        let assign8160_e3413: f64 = (p.p733 / locals.var_lwbin);
        let assign8160_e3414: f64 = (assign8160_e3410 + assign8160_e3413);
        locals.var_uc_bgtmp2 = assign8160_e3414;
        locals.var_uc_bgtmp2_rv = 0.0;

        let assign8170_e3418: f64 = (p.p558 / locals.var_lbin);
        let assign8170_e3419: f64 = (p.p94 + assign8170_e3418);
        let assign8170_e3422: f64 = (p.p646 / locals.var_wbin);
        let assign8170_e3423: f64 = (assign8170_e3419 + assign8170_e3422);
        let assign8170_e3426: f64 = (p.p734 / locals.var_lwbin);
        let assign8170_e3427: f64 = (assign8170_e3423 + assign8170_e3426);
        locals.var_uc_eg0 = assign8170_e3427;
        locals.var_uc_eg0_rv = 0.0;

        let assign8180_e3431: f64 = (p.p559 / locals.var_lbin);
        let assign8180_e3432: f64 = (p.p110 + assign8180_e3431);
        let assign8180_e3435: f64 = (p.p647 / locals.var_wbin);
        let assign8180_e3436: f64 = (assign8180_e3432 + assign8180_e3435);
        let assign8180_e3439: f64 = (p.p735 / locals.var_lwbin);
        let assign8180_e3440: f64 = (assign8180_e3436 + assign8180_e3439);
        locals.var_uc_vfbover = assign8180_e3440;
        locals.var_uc_vfbover_rv = 0.0;

        let assign8190_e3444: f64 = (p.p560 / locals.var_lbin);
        let assign8190_e3445: f64 = (p.p111 + assign8190_e3444);
        let assign8190_e3448: f64 = (p.p648 / locals.var_wbin);
        let assign8190_e3449: f64 = (assign8190_e3445 + assign8190_e3448);
        let assign8190_e3452: f64 = (p.p736 / locals.var_lwbin);
        let assign8190_e3453: f64 = (assign8190_e3449 + assign8190_e3452);
        locals.var_uc_nover = assign8190_e3453;
        locals.var_uc_nover_rv = 0.0;

        let assign8200_e3457: f64 = (p.p561 / locals.var_lbin);
        let assign8200_e3458: f64 = (p.p112 + assign8200_e3457);
        let assign8200_e3461: f64 = (p.p649 / locals.var_wbin);
        let assign8200_e3462: f64 = (assign8200_e3458 + assign8200_e3461);
        let assign8200_e3465: f64 = (p.p737 / locals.var_lwbin);
        let assign8200_e3466: f64 = (assign8200_e3462 + assign8200_e3465);
        locals.var_uc_novers = assign8200_e3466;
        locals.var_uc_novers_rv = 0.0;

        let assign8210_e3470: f64 = (p.p562 / locals.var_lbin);
        let assign8210_e3471: f64 = (p.p126 + assign8210_e3470);
        let assign8210_e3474: f64 = (p.p650 / locals.var_wbin);
        let assign8210_e3475: f64 = (assign8210_e3471 + assign8210_e3474);
        let assign8210_e3478: f64 = (p.p738 / locals.var_lwbin);
        let assign8210_e3479: f64 = (assign8210_e3475 + assign8210_e3478);
        locals.var_uc_wl2 = assign8210_e3479;
        locals.var_uc_wl2_rv = 0.0;

        let assign8220_e3483: f64 = (p.p563 / locals.var_lbin);
        let assign8220_e3484: f64 = (p.p136 + assign8220_e3483);
        let assign8220_e3487: f64 = (p.p651 / locals.var_wbin);
        let assign8220_e3488: f64 = (assign8220_e3484 + assign8220_e3487);
        let assign8220_e3491: f64 = (p.p739 / locals.var_lwbin);
        let assign8220_e3492: f64 = (assign8220_e3488 + assign8220_e3491);
        locals.var_uc_vfbc = assign8220_e3492;
        locals.var_uc_vfbc_rv = 0.0;

        let assign8230_e3496: f64 = (p.p564 / locals.var_lbin);
        let assign8230_e3497: f64 = (p.p138 + assign8230_e3496);
        let assign8230_e3500: f64 = (p.p652 / locals.var_wbin);
        let assign8230_e3501: f64 = (assign8230_e3497 + assign8230_e3500);
        let assign8230_e3504: f64 = (p.p740 / locals.var_lwbin);
        let assign8230_e3505: f64 = (assign8230_e3501 + assign8230_e3504);
        locals.var_uc_nsubc = assign8230_e3505;
        locals.var_uc_nsubc_rv = 0.0;

        let assign8240_e3509: f64 = (p.p565 / locals.var_lbin);
        let assign8240_e3510: f64 = (p.p141 + assign8240_e3509);
        let assign8240_e3513: f64 = (p.p653 / locals.var_wbin);
        let assign8240_e3514: f64 = (assign8240_e3510 + assign8240_e3513);
        let assign8240_e3517: f64 = (p.p741 / locals.var_lwbin);
        let assign8240_e3518: f64 = (assign8240_e3514 + assign8240_e3517);
        locals.var_uc_nsubp = assign8240_e3518;
        locals.var_uc_nsubp_rv = 0.0;

        let assign8250_e3522: f64 = (p.p566 / locals.var_lbin);
        let assign8250_e3523: f64 = (p.p144 + assign8250_e3522);
        let assign8250_e3526: f64 = (p.p654 / locals.var_wbin);
        let assign8250_e3527: f64 = (assign8250_e3523 + assign8250_e3526);
        let assign8250_e3530: f64 = (p.p742 / locals.var_lwbin);
        let assign8250_e3531: f64 = (assign8250_e3527 + assign8250_e3530);
        locals.var_uc_scp1 = assign8250_e3531;
        locals.var_uc_scp1_rv = 0.0;

        let assign8260_e3535: f64 = (p.p567 / locals.var_lbin);
        let assign8260_e3536: f64 = (p.p145 + assign8260_e3535);
        let assign8260_e3539: f64 = (p.p655 / locals.var_wbin);
        let assign8260_e3540: f64 = (assign8260_e3536 + assign8260_e3539);
        let assign8260_e3543: f64 = (p.p743 / locals.var_lwbin);
        let assign8260_e3544: f64 = (assign8260_e3540 + assign8260_e3543);
        locals.var_uc_scp2 = assign8260_e3544;
        locals.var_uc_scp2_rv = 0.0;

        let assign8270_e3548: f64 = (p.p568 / locals.var_lbin);
        let assign8270_e3549: f64 = (p.p146 + assign8270_e3548);
        let assign8270_e3552: f64 = (p.p656 / locals.var_wbin);
        let assign8270_e3553: f64 = (assign8270_e3549 + assign8270_e3552);
        let assign8270_e3556: f64 = (p.p744 / locals.var_lwbin);
        let assign8270_e3557: f64 = (assign8270_e3553 + assign8270_e3556);
        locals.var_uc_scp3 = assign8270_e3557;
        locals.var_uc_scp3_rv = 0.0;

        let assign8280_e3561: f64 = (p.p569 / locals.var_lbin);
        let assign8280_e3562: f64 = (p.p147 + assign8280_e3561);
        let assign8280_e3565: f64 = (p.p657 / locals.var_wbin);
        let assign8280_e3566: f64 = (assign8280_e3562 + assign8280_e3565);
        let assign8280_e3569: f64 = (p.p745 / locals.var_lwbin);
        let assign8280_e3570: f64 = (assign8280_e3566 + assign8280_e3569);
        locals.var_uc_sc1 = assign8280_e3570;
        locals.var_uc_sc1_rv = 0.0;

        let assign8290_e3574: f64 = (p.p570 / locals.var_lbin);
        let assign8290_e3575: f64 = (p.p148 + assign8290_e3574);
        let assign8290_e3578: f64 = (p.p658 / locals.var_wbin);
        let assign8290_e3579: f64 = (assign8290_e3575 + assign8290_e3578);
        let assign8290_e3582: f64 = (p.p746 / locals.var_lwbin);
        let assign8290_e3583: f64 = (assign8290_e3579 + assign8290_e3582);
        locals.var_uc_sc2 = assign8290_e3583;
        locals.var_uc_sc2_rv = 0.0;

        let assign8300_e3587: f64 = (p.p571 / locals.var_lbin);
        let assign8300_e3588: f64 = (p.p149 + assign8300_e3587);
        let assign8300_e3591: f64 = (p.p659 / locals.var_wbin);
        let assign8300_e3592: f64 = (assign8300_e3588 + assign8300_e3591);
        let assign8300_e3595: f64 = (p.p747 / locals.var_lwbin);
        let assign8300_e3596: f64 = (assign8300_e3592 + assign8300_e3595);
        locals.var_uc_sc3 = assign8300_e3596;
        locals.var_uc_sc3_rv = 0.0;

        let assign8310_e3600: f64 = (p.p572 / locals.var_lbin);
        let assign8310_e3601: f64 = (p.p151 + assign8310_e3600);
        let assign8310_e3604: f64 = (p.p660 / locals.var_wbin);
        let assign8310_e3605: f64 = (assign8310_e3601 + assign8310_e3604);
        let assign8310_e3608: f64 = (p.p748 / locals.var_lwbin);
        let assign8310_e3609: f64 = (assign8310_e3605 + assign8310_e3608);
        locals.var_uc_pgd1 = assign8310_e3609;
        locals.var_uc_pgd1_rv = 0.0;

        let assign8320_e3613: f64 = (p.p573 / locals.var_lbin);
        let assign8320_e3614: f64 = (p.p154 + assign8320_e3613);
        let assign8320_e3617: f64 = (p.p661 / locals.var_wbin);
        let assign8320_e3618: f64 = (assign8320_e3614 + assign8320_e3617);
        let assign8320_e3621: f64 = (p.p749 / locals.var_lwbin);
        let assign8320_e3622: f64 = (assign8320_e3618 + assign8320_e3621);
        locals.var_uc_ndep = assign8320_e3622;
        locals.var_uc_ndep_rv = 0.0;

        let assign8330_e3626: f64 = (p.p574 / locals.var_lbin);
        let assign8330_e3627: f64 = (p.p157 + assign8330_e3626);
        let assign8330_e3630: f64 = (p.p662 / locals.var_wbin);
        let assign8330_e3631: f64 = (assign8330_e3627 + assign8330_e3630);
        let assign8330_e3634: f64 = (p.p750 / locals.var_lwbin);
        let assign8330_e3635: f64 = (assign8330_e3631 + assign8330_e3634);
        locals.var_uc_ninv = assign8330_e3635;
        locals.var_uc_ninv_rv = 0.0;

        let assign8340_e3639: f64 = (p.p575 / locals.var_lbin);
        let assign8340_e3640: f64 = (p.p158 + assign8340_e3639);
        let assign8340_e3643: f64 = (p.p663 / locals.var_wbin);
        let assign8340_e3644: f64 = (assign8340_e3640 + assign8340_e3643);
        let assign8340_e3647: f64 = (p.p751 / locals.var_lwbin);
        let assign8340_e3648: f64 = (assign8340_e3644 + assign8340_e3647);
        locals.var_uc_muecb0 = assign8340_e3648;
        locals.var_uc_muecb0_rv = 0.0;

        let assign8350_e3652: f64 = (p.p576 / locals.var_lbin);
        let assign8350_e3653: f64 = (p.p159 + assign8350_e3652);
        let assign8350_e3656: f64 = (p.p664 / locals.var_wbin);
        let assign8350_e3657: f64 = (assign8350_e3653 + assign8350_e3656);
        let assign8350_e3660: f64 = (p.p752 / locals.var_lwbin);
        let assign8350_e3661: f64 = (assign8350_e3657 + assign8350_e3660);
        locals.var_uc_muecb1 = assign8350_e3661;
        locals.var_uc_muecb1_rv = 0.0;

        let assign8360_e3665: f64 = (p.p577 / locals.var_lbin);
        let assign8360_e3666: f64 = (p.p161 + assign8360_e3665);
        let assign8360_e3669: f64 = (p.p665 / locals.var_wbin);
        let assign8360_e3670: f64 = (assign8360_e3666 + assign8360_e3669);
        let assign8360_e3673: f64 = (p.p753 / locals.var_lwbin);
        let assign8360_e3674: f64 = (assign8360_e3670 + assign8360_e3673);
        locals.var_uc_mueph1 = assign8360_e3674;
        locals.var_uc_mueph1_rv = 0.0;

        let assign8370_e3678: f64 = (p.p578 / locals.var_lbin);
        let assign8370_e3679: f64 = (p.p169 + assign8370_e3678);
        let assign8370_e3682: f64 = (p.p666 / locals.var_wbin);
        let assign8370_e3683: f64 = (assign8370_e3679 + assign8370_e3682);
        let assign8370_e3686: f64 = (p.p754 / locals.var_lwbin);
        let assign8370_e3687: f64 = (assign8370_e3683 + assign8370_e3686);
        locals.var_uc_vtmp = assign8370_e3687;
        locals.var_uc_vtmp_rv = 0.0;

        let assign8380_e3691: f64 = (p.p579 / locals.var_lbin);
        let assign8380_e3692: f64 = (p.p170 + assign8380_e3691);
        let assign8380_e3695: f64 = (p.p667 / locals.var_wbin);
        let assign8380_e3696: f64 = (assign8380_e3692 + assign8380_e3695);
        let assign8380_e3699: f64 = (p.p755 / locals.var_lwbin);
        let assign8380_e3700: f64 = (assign8380_e3696 + assign8380_e3699);
        locals.var_uc_wvth0 = assign8380_e3700;
        locals.var_uc_wvth0_rv = 0.0;

        let assign8390_e3704: f64 = (p.p580 / locals.var_lbin);
        let assign8390_e3705: f64 = (p.p172 + assign8390_e3704);
        let assign8390_e3708: f64 = (p.p668 / locals.var_wbin);
        let assign8390_e3709: f64 = (assign8390_e3705 + assign8390_e3708);
        let assign8390_e3712: f64 = (p.p756 / locals.var_lwbin);
        let assign8390_e3713: f64 = (assign8390_e3709 + assign8390_e3712);
        locals.var_uc_muesr1 = assign8390_e3713;
        locals.var_uc_muesr1_rv = 0.0;

        let assign8400_e3717: f64 = (p.p581 / locals.var_lbin);
        let assign8400_e3718: f64 = (p.p177 + assign8400_e3717);
        let assign8400_e3721: f64 = (p.p669 / locals.var_wbin);
        let assign8400_e3722: f64 = (assign8400_e3718 + assign8400_e3721);
        let assign8400_e3725: f64 = (p.p757 / locals.var_lwbin);
        let assign8400_e3726: f64 = (assign8400_e3722 + assign8400_e3725);
        locals.var_uc_muetmp = assign8400_e3726;
        locals.var_uc_muetmp_rv = 0.0;

        let assign8410_e3730: f64 = (p.p582 / locals.var_lbin);
        let assign8410_e3731: f64 = (p.p179 + assign8410_e3730);
        let assign8410_e3734: f64 = (p.p670 / locals.var_wbin);
        let assign8410_e3735: f64 = (assign8410_e3731 + assign8410_e3734);
        let assign8410_e3738: f64 = (p.p758 / locals.var_lwbin);
        let assign8410_e3739: f64 = (assign8410_e3735 + assign8410_e3738);
        locals.var_uc_sub1 = assign8410_e3739;
        locals.var_uc_sub1_rv = 0.0;

        let assign8420_e3743: f64 = (p.p583 / locals.var_lbin);
        let assign8420_e3744: f64 = (p.p180 + assign8420_e3743);
        let assign8420_e3747: f64 = (p.p671 / locals.var_wbin);
        let assign8420_e3748: f64 = (assign8420_e3744 + assign8420_e3747);
        let assign8420_e3751: f64 = (p.p759 / locals.var_lwbin);
        let assign8420_e3752: f64 = (assign8420_e3748 + assign8420_e3751);
        locals.var_uc_sub2 = assign8420_e3752;
        locals.var_uc_sub2_rv = 0.0;

        let assign8430_e3756: f64 = (p.p584 / locals.var_lbin);
        let assign8430_e3757: f64 = (p.p185 + assign8430_e3756);
        let assign8430_e3760: f64 = (p.p672 / locals.var_wbin);
        let assign8430_e3761: f64 = (assign8430_e3757 + assign8430_e3760);
        let assign8430_e3764: f64 = (p.p760 / locals.var_lwbin);
        let assign8430_e3765: f64 = (assign8430_e3761 + assign8430_e3764);
        locals.var_uc_svds = assign8430_e3765;
        locals.var_uc_svds_rv = 0.0;

        let assign8440_e3769: f64 = (p.p585 / locals.var_lbin);
        let assign8440_e3770: f64 = (p.p182 + assign8440_e3769);
        let assign8440_e3773: f64 = (p.p673 / locals.var_wbin);
        let assign8440_e3774: f64 = (assign8440_e3770 + assign8440_e3773);
        let assign8440_e3777: f64 = (p.p761 / locals.var_lwbin);
        let assign8440_e3778: f64 = (assign8440_e3774 + assign8440_e3777);
        locals.var_uc_svbs = assign8440_e3778;
        locals.var_uc_svbs_rv = 0.0;

        let assign8450_e3782: f64 = (p.p586 / locals.var_lbin);
        let assign8450_e3783: f64 = (p.p181 + assign8450_e3782);
        let assign8450_e3786: f64 = (p.p674 / locals.var_wbin);
        let assign8450_e3787: f64 = (assign8450_e3783 + assign8450_e3786);
        let assign8450_e3790: f64 = (p.p762 / locals.var_lwbin);
        let assign8450_e3791: f64 = (assign8450_e3787 + assign8450_e3790);
        locals.var_uc_svgs = assign8450_e3791;
        locals.var_uc_svgs_rv = 0.0;

        let assign8460_e3795: f64 = (p.p587 / locals.var_lbin);
        let assign8460_e3796: f64 = (p.p187 + assign8460_e3795);
        let assign8460_e3799: f64 = (p.p675 / locals.var_wbin);
        let assign8460_e3800: f64 = (assign8460_e3796 + assign8460_e3799);
        let assign8460_e3803: f64 = (p.p763 / locals.var_lwbin);
        let assign8460_e3804: f64 = (assign8460_e3800 + assign8460_e3803);
        locals.var_uc_sub1snp = assign8460_e3804;
        locals.var_uc_sub1snp_rv = 0.0;

        let assign8470_e3808: f64 = (p.p588 / locals.var_lbin);
        let assign8470_e3809: f64 = (p.p188 + assign8470_e3808);
        let assign8470_e3812: f64 = (p.p676 / locals.var_wbin);
        let assign8470_e3813: f64 = (assign8470_e3809 + assign8470_e3812);
        let assign8470_e3816: f64 = (p.p764 / locals.var_lwbin);
        let assign8470_e3817: f64 = (assign8470_e3813 + assign8470_e3816);
        locals.var_uc_sub2snp = assign8470_e3817;
        locals.var_uc_sub2snp_rv = 0.0;

        let assign8480_e3821: f64 = (p.p589 / locals.var_lbin);
        let assign8480_e3822: f64 = (p.p189 + assign8480_e3821);
        let assign8480_e3825: f64 = (p.p677 / locals.var_wbin);
        let assign8480_e3826: f64 = (assign8480_e3822 + assign8480_e3825);
        let assign8480_e3829: f64 = (p.p765 / locals.var_lwbin);
        let assign8480_e3830: f64 = (assign8480_e3826 + assign8480_e3829);
        locals.var_uc_svdssnp = assign8480_e3830;
        locals.var_uc_svdssnp_rv = 0.0;

        let assign8490_e3834: f64 = (p.p590 / locals.var_lbin);
        let assign8490_e3835: f64 = (p.p194 + assign8490_e3834);
        let assign8490_e3838: f64 = (p.p678 / locals.var_wbin);
        let assign8490_e3839: f64 = (assign8490_e3835 + assign8490_e3838);
        let assign8490_e3842: f64 = (p.p766 / locals.var_lwbin);
        let assign8490_e3843: f64 = (assign8490_e3839 + assign8490_e3842);
        locals.var_uc_fn1 = assign8490_e3843;
        locals.var_uc_fn1_rv = 0.0;

        let assign8500_e3847: f64 = (p.p591 / locals.var_lbin);
        let assign8500_e3848: f64 = (p.p195 + assign8500_e3847);
        let assign8500_e3851: f64 = (p.p679 / locals.var_wbin);
        let assign8500_e3852: f64 = (assign8500_e3848 + assign8500_e3851);
        let assign8500_e3855: f64 = (p.p767 / locals.var_lwbin);
        let assign8500_e3856: f64 = (assign8500_e3852 + assign8500_e3855);
        locals.var_uc_fn2 = assign8500_e3856;
        locals.var_uc_fn2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8510_e3860: f64 = (p.p592 / locals.var_lbin);
        let assign8510_e3861: f64 = (p.p196 + assign8510_e3860);
        let assign8510_e3864: f64 = (p.p680 / locals.var_wbin);
        let assign8510_e3865: f64 = (assign8510_e3861 + assign8510_e3864);
        let assign8510_e3868: f64 = (p.p768 / locals.var_lwbin);
        let assign8510_e3869: f64 = (assign8510_e3865 + assign8510_e3868);
        locals.var_uc_fn3 = assign8510_e3869;
        locals.var_uc_fn3_rv = 0.0;

        let assign8520_e3873: f64 = (p.p593 / locals.var_lbin);
        let assign8520_e3874: f64 = (p.p197 + assign8520_e3873);
        let assign8520_e3877: f64 = (p.p681 / locals.var_wbin);
        let assign8520_e3878: f64 = (assign8520_e3874 + assign8520_e3877);
        let assign8520_e3881: f64 = (p.p769 / locals.var_lwbin);
        let assign8520_e3882: f64 = (assign8520_e3878 + assign8520_e3881);
        locals.var_uc_fvbs = assign8520_e3882;
        locals.var_uc_fvbs_rv = 0.0;

        let assign8530_e3886: f64 = (p.p594 / locals.var_lbin);
        let assign8530_e3887: f64 = (p.p204 + assign8530_e3886);
        let assign8530_e3890: f64 = (p.p682 / locals.var_wbin);
        let assign8530_e3891: f64 = (assign8530_e3887 + assign8530_e3890);
        let assign8530_e3894: f64 = (p.p770 / locals.var_lwbin);
        let assign8530_e3895: f64 = (assign8530_e3891 + assign8530_e3894);
        locals.var_uc_nsti = assign8530_e3895;
        locals.var_uc_nsti_rv = 0.0;

        let assign8540_e3899: f64 = (p.p595 / locals.var_lbin);
        let assign8540_e3900: f64 = (p.p205 + assign8540_e3899);
        let assign8540_e3903: f64 = (p.p683 / locals.var_wbin);
        let assign8540_e3904: f64 = (assign8540_e3900 + assign8540_e3903);
        let assign8540_e3907: f64 = (p.p771 / locals.var_lwbin);
        let assign8540_e3908: f64 = (assign8540_e3904 + assign8540_e3907);
        locals.var_uc_wsti = assign8540_e3908;
        locals.var_uc_wsti_dn0 = 0.0;
        locals.var_uc_wsti_dn2 = 0.0;
        locals.var_uc_wsti_dn4 = 0.0;
        locals.var_uc_wsti_dn5 = 0.0;
        locals.var_uc_wsti_dn6 = 0.0;
        locals.var_uc_wsti_dn7 = 0.0;
        locals.var_uc_wsti_dn8 = 0.0;
        locals.var_uc_wsti_dn9 = 0.0;
        locals.var_uc_wsti_dn10 = 0.0;
        locals.var_uc_wsti_dn11 = 0.0;
        locals.var_uc_wsti_dn14 = 0.0;
        locals.var_uc_wsti_rv = 0.0;

        let assign8550_e3912: f64 = (p.p596 / locals.var_lbin);
        let assign8550_e3913: f64 = (p.p210 + assign8550_e3912);
        let assign8550_e3916: f64 = (p.p684 / locals.var_wbin);
        let assign8550_e3917: f64 = (assign8550_e3913 + assign8550_e3916);
        let assign8550_e3920: f64 = (p.p772 / locals.var_lwbin);
        let assign8550_e3921: f64 = (assign8550_e3917 + assign8550_e3920);
        locals.var_uc_scsti1 = assign8550_e3921;
        locals.var_uc_scsti1_rv = 0.0;

        let assign8560_e3925: f64 = (p.p597 / locals.var_lbin);
        let assign8560_e3926: f64 = (p.p211 + assign8560_e3925);
        let assign8560_e3929: f64 = (p.p685 / locals.var_wbin);
        let assign8560_e3930: f64 = (assign8560_e3926 + assign8560_e3929);
        let assign8560_e3933: f64 = (p.p773 / locals.var_lwbin);
        let assign8560_e3934: f64 = (assign8560_e3930 + assign8560_e3933);
        locals.var_uc_scsti2 = assign8560_e3934;
        locals.var_uc_scsti2_rv = 0.0;

        let assign8570_e3938: f64 = (p.p598 / locals.var_lbin);
        let assign8570_e3939: f64 = (p.p212 + assign8570_e3938);
        let assign8570_e3942: f64 = (p.p686 / locals.var_wbin);
        let assign8570_e3943: f64 = (assign8570_e3939 + assign8570_e3942);
        let assign8570_e3946: f64 = (p.p774 / locals.var_lwbin);
        let assign8570_e3947: f64 = (assign8570_e3943 + assign8570_e3946);
        locals.var_uc_vthsti = assign8570_e3947;
        locals.var_uc_vthsti_rv = 0.0;

        let assign8580_e3951: f64 = (p.p599 / locals.var_lbin);
        let assign8580_e3952: f64 = (p.p214 + assign8580_e3951);
        let assign8580_e3955: f64 = (p.p687 / locals.var_wbin);
        let assign8580_e3956: f64 = (assign8580_e3952 + assign8580_e3955);
        let assign8580_e3959: f64 = (p.p775 / locals.var_lwbin);
        let assign8580_e3960: f64 = (assign8580_e3956 + assign8580_e3959);
        locals.var_uc_muesti1 = assign8580_e3960;
        locals.var_uc_muesti1_rv = 0.0;

        let assign8590_e3964: f64 = (p.p600 / locals.var_lbin);
        let assign8590_e3965: f64 = (p.p215 + assign8590_e3964);
        let assign8590_e3968: f64 = (p.p688 / locals.var_wbin);
        let assign8590_e3969: f64 = (assign8590_e3965 + assign8590_e3968);
        let assign8590_e3972: f64 = (p.p776 / locals.var_lwbin);
        let assign8590_e3973: f64 = (assign8590_e3969 + assign8590_e3972);
        locals.var_uc_muesti2 = assign8590_e3973;
        locals.var_uc_muesti2_rv = 0.0;

        let assign8600_e3977: f64 = (p.p601 / locals.var_lbin);
        let assign8600_e3978: f64 = (p.p216 + assign8600_e3977);
        let assign8600_e3981: f64 = (p.p689 / locals.var_wbin);
        let assign8600_e3982: f64 = (assign8600_e3978 + assign8600_e3981);
        let assign8600_e3985: f64 = (p.p777 / locals.var_lwbin);
        let assign8600_e3986: f64 = (assign8600_e3982 + assign8600_e3985);
        locals.var_uc_muesti3 = assign8600_e3986;
        locals.var_uc_muesti3_rv = 0.0;

        let assign8610_e3990: f64 = (p.p602 / locals.var_lbin);
        let assign8610_e3991: f64 = (p.p217 + assign8610_e3990);
        let assign8610_e3994: f64 = (p.p690 / locals.var_wbin);
        let assign8610_e3995: f64 = (assign8610_e3991 + assign8610_e3994);
        let assign8610_e3998: f64 = (p.p778 / locals.var_lwbin);
        let assign8610_e3999: f64 = (assign8610_e3995 + assign8610_e3998);
        locals.var_uc_nsubpsti1 = assign8610_e3999;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign8620_e4003: f64 = (p.p603 / locals.var_lbin);
        let assign8620_e4004: f64 = (p.p218 + assign8620_e4003);
        let assign8620_e4007: f64 = (p.p691 / locals.var_wbin);
        let assign8620_e4008: f64 = (assign8620_e4004 + assign8620_e4007);
        let assign8620_e4011: f64 = (p.p779 / locals.var_lwbin);
        let assign8620_e4012: f64 = (assign8620_e4008 + assign8620_e4011);
        locals.var_uc_nsubpsti2 = assign8620_e4012;
        locals.var_uc_nsubpsti2_rv = 0.0;

        let assign8630_e4016: f64 = (p.p604 / locals.var_lbin);
        let assign8630_e4017: f64 = (p.p219 + assign8630_e4016);
        let assign8630_e4020: f64 = (p.p692 / locals.var_wbin);
        let assign8630_e4021: f64 = (assign8630_e4017 + assign8630_e4020);
        let assign8630_e4024: f64 = (p.p780 / locals.var_lwbin);
        let assign8630_e4025: f64 = (assign8630_e4021 + assign8630_e4024);
        locals.var_uc_nsubpsti3 = assign8630_e4025;
        locals.var_uc_nsubpsti3_rv = 0.0;

        let assign8640_e4029: f64 = (p.p605 / locals.var_lbin);
        let assign8640_e4030: f64 = (p.p269 + assign8640_e4029);
        let assign8640_e4033: f64 = (p.p693 / locals.var_wbin);
        let assign8640_e4034: f64 = (assign8640_e4030 + assign8640_e4033);
        let assign8640_e4037: f64 = (p.p781 / locals.var_lwbin);
        let assign8640_e4038: f64 = (assign8640_e4034 + assign8640_e4037);
        locals.var_uc_cgso = assign8640_e4038;
        locals.var_uc_cgso_rv = 0.0;

        let assign8650_e4042: f64 = (p.p606 / locals.var_lbin);
        let assign8650_e4043: f64 = (p.p268 + assign8650_e4042);
        let assign8650_e4046: f64 = (p.p694 / locals.var_wbin);
        let assign8650_e4047: f64 = (assign8650_e4043 + assign8650_e4046);
        let assign8650_e4050: f64 = (p.p782 / locals.var_lwbin);
        let assign8650_e4051: f64 = (assign8650_e4047 + assign8650_e4050);
        locals.var_uc_cgdo = assign8650_e4051;
        locals.var_uc_cgdo_rv = 0.0;

        let assign8660_e4055: f64 = (p.p607 / locals.var_lbin);
        let assign8660_e4056: f64 = (p.p226 + assign8660_e4055);
        let assign8660_e4059: f64 = (p.p695 / locals.var_wbin);
        let assign8660_e4060: f64 = (assign8660_e4056 + assign8660_e4059);
        let assign8660_e4063: f64 = (p.p783 / locals.var_lwbin);
        let assign8660_e4064: f64 = (assign8660_e4060 + assign8660_e4063);
        locals.var_uc_clm1 = assign8660_e4064;
        locals.var_uc_clm1_rv = 0.0;

        let assign8670_e4068: f64 = (p.p608 / locals.var_lbin);
        let assign8670_e4069: f64 = (p.p227 + assign8670_e4068);
        let assign8670_e4072: f64 = (p.p696 / locals.var_wbin);
        let assign8670_e4073: f64 = (assign8670_e4069 + assign8670_e4072);
        let assign8670_e4076: f64 = (p.p784 / locals.var_lwbin);
        let assign8670_e4077: f64 = (assign8670_e4073 + assign8670_e4076);
        locals.var_uc_clm2 = assign8670_e4077;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn4 = 0.0;
        locals.var_uc_clm2_dn5 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn8 = 0.0;
        locals.var_uc_clm2_dn9 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn11 = 0.0;
        locals.var_uc_clm2_dn14 = 0.0;
        locals.var_uc_clm2_rv = 0.0;

        let assign8680_e4081: f64 = (p.p609 / locals.var_lbin);
        let assign8680_e4082: f64 = (p.p228 + assign8680_e4081);
        let assign8680_e4085: f64 = (p.p697 / locals.var_wbin);
        let assign8680_e4086: f64 = (assign8680_e4082 + assign8680_e4085);
        let assign8680_e4089: f64 = (p.p785 / locals.var_lwbin);
        let assign8680_e4090: f64 = (assign8680_e4086 + assign8680_e4089);
        locals.var_uc_clm3 = assign8680_e4090;
        locals.var_uc_clm3_rv = 0.0;

        let assign8690_e4094: f64 = (p.p610 / locals.var_lbin);
        let assign8690_e4095: f64 = (p.p232 + assign8690_e4094);
        let assign8690_e4098: f64 = (p.p698 / locals.var_wbin);
        let assign8690_e4099: f64 = (assign8690_e4095 + assign8690_e4098);
        let assign8690_e4102: f64 = (p.p786 / locals.var_lwbin);
        let assign8690_e4103: f64 = (assign8690_e4099 + assign8690_e4102);
        locals.var_uc_wfc = assign8690_e4103;
        locals.var_uc_wfc_rv = 0.0;

        let assign8700_e4107: f64 = (p.p611 / locals.var_lbin);
        let assign8700_e4108: f64 = (p.p240 + assign8700_e4107);
        let assign8700_e4111: f64 = (p.p699 / locals.var_wbin);
        let assign8700_e4112: f64 = (assign8700_e4108 + assign8700_e4111);
        let assign8700_e4115: f64 = (p.p787 / locals.var_lwbin);
        let assign8700_e4116: f64 = (assign8700_e4112 + assign8700_e4115);
        locals.var_uc_gidl1 = assign8700_e4116;
        locals.var_uc_gidl1_rv = 0.0;

        let assign8710_e4120: f64 = (p.p612 / locals.var_lbin);
        let assign8710_e4121: f64 = (p.p241 + assign8710_e4120);
        let assign8710_e4124: f64 = (p.p700 / locals.var_wbin);
        let assign8710_e4125: f64 = (assign8710_e4121 + assign8710_e4124);
        let assign8710_e4128: f64 = (p.p788 / locals.var_lwbin);
        let assign8710_e4129: f64 = (assign8710_e4125 + assign8710_e4128);
        locals.var_uc_gidl2 = assign8710_e4129;
        locals.var_uc_gidl2_rv = 0.0;

        let assign8720_e4133: f64 = (p.p613 / locals.var_lbin);
        let assign8720_e4134: f64 = (p.p245 + assign8720_e4133);
        let assign8720_e4137: f64 = (p.p701 / locals.var_wbin);
        let assign8720_e4138: f64 = (assign8720_e4134 + assign8720_e4137);
        let assign8720_e4141: f64 = (p.p789 / locals.var_lwbin);
        let assign8720_e4142: f64 = (assign8720_e4138 + assign8720_e4141);
        locals.var_uc_gleak1 = assign8720_e4142;
        locals.var_uc_gleak1_rv = 0.0;

        let assign8730_e4146: f64 = (p.p614 / locals.var_lbin);
        let assign8730_e4147: f64 = (p.p246 + assign8730_e4146);
        let assign8730_e4150: f64 = (p.p702 / locals.var_wbin);
        let assign8730_e4151: f64 = (assign8730_e4147 + assign8730_e4150);
        let assign8730_e4154: f64 = (p.p790 / locals.var_lwbin);
        let assign8730_e4155: f64 = (assign8730_e4151 + assign8730_e4154);
        locals.var_uc_gleak2 = assign8730_e4155;
        locals.var_uc_gleak2_rv = 0.0;

        let assign8740_e4159: f64 = (p.p615 / locals.var_lbin);
        let assign8740_e4160: f64 = (p.p247 + assign8740_e4159);
        let assign8740_e4163: f64 = (p.p703 / locals.var_wbin);
        let assign8740_e4164: f64 = (assign8740_e4160 + assign8740_e4163);
        let assign8740_e4167: f64 = (p.p791 / locals.var_lwbin);
        let assign8740_e4168: f64 = (assign8740_e4164 + assign8740_e4167);
        locals.var_uc_gleak3 = assign8740_e4168;
        locals.var_uc_gleak3_rv = 0.0;

        let assign8750_e4172: f64 = (p.p616 / locals.var_lbin);
        let assign8750_e4173: f64 = (p.p250 + assign8750_e4172);
        let assign8750_e4176: f64 = (p.p704 / locals.var_wbin);
        let assign8750_e4177: f64 = (assign8750_e4173 + assign8750_e4176);
        let assign8750_e4180: f64 = (p.p792 / locals.var_lwbin);
        let assign8750_e4181: f64 = (assign8750_e4177 + assign8750_e4180);
        locals.var_uc_gleak6 = assign8750_e4181;
        locals.var_uc_gleak6_rv = 0.0;

        let assign8760_e4185: f64 = (p.p617 / locals.var_lbin);
        let assign8760_e4186: f64 = (p.p253 + assign8760_e4185);
        let assign8760_e4189: f64 = (p.p705 / locals.var_wbin);
        let assign8760_e4190: f64 = (assign8760_e4186 + assign8760_e4189);
        let assign8760_e4193: f64 = (p.p793 / locals.var_lwbin);
        let assign8760_e4194: f64 = (assign8760_e4190 + assign8760_e4193);
        locals.var_uc_glksd1 = assign8760_e4194;
        locals.var_uc_glksd1_rv = 0.0;

        let assign8770_e4198: f64 = (p.p618 / locals.var_lbin);
        let assign8770_e4199: f64 = (p.p254 + assign8770_e4198);
        let assign8770_e4202: f64 = (p.p706 / locals.var_wbin);
        let assign8770_e4203: f64 = (assign8770_e4199 + assign8770_e4202);
        let assign8770_e4206: f64 = (p.p794 / locals.var_lwbin);
        let assign8770_e4207: f64 = (assign8770_e4203 + assign8770_e4206);
        locals.var_uc_glksd2 = assign8770_e4207;
        locals.var_uc_glksd2_rv = 0.0;

        let assign8780_e4211: f64 = (p.p619 / locals.var_lbin);
        let assign8780_e4212: f64 = (p.p256 + assign8780_e4211);
        let assign8780_e4215: f64 = (p.p707 / locals.var_wbin);
        let assign8780_e4216: f64 = (assign8780_e4212 + assign8780_e4215);
        let assign8780_e4219: f64 = (p.p795 / locals.var_lwbin);
        let assign8780_e4220: f64 = (assign8780_e4216 + assign8780_e4219);
        locals.var_uc_glkb1 = assign8780_e4220;
        locals.var_uc_glkb1_rv = 0.0;

        let assign8790_e4224: f64 = (p.p620 / locals.var_lbin);
        let assign8790_e4225: f64 = (p.p257 + assign8790_e4224);
        let assign8790_e4228: f64 = (p.p708 / locals.var_wbin);
        let assign8790_e4229: f64 = (assign8790_e4225 + assign8790_e4228);
        let assign8790_e4232: f64 = (p.p796 / locals.var_lwbin);
        let assign8790_e4233: f64 = (assign8790_e4229 + assign8790_e4232);
        locals.var_uc_glkb2 = assign8790_e4233;
        locals.var_uc_glkb2_rv = 0.0;

        let assign8810_e4250: f64 = (p.p622 / locals.var_lbin);
        let assign8810_e4251: f64 = (p.p265 + assign8810_e4250);
        let assign8810_e4254: f64 = (p.p710 / locals.var_wbin);
        let assign8810_e4255: f64 = (assign8810_e4251 + assign8810_e4254);
        let assign8810_e4258: f64 = (p.p798 / locals.var_lwbin);
        let assign8810_e4259: f64 = (assign8810_e4255 + assign8810_e4258);
        locals.var_uc_nfalp = assign8810_e4259;
        locals.var_uc_nfalp_rv = 0.0;

        let assign8820_e4263: f64 = (p.p623 / locals.var_lbin);
        let assign8820_e4264: f64 = (p.p278 + assign8820_e4263);
        let assign8820_e4267: f64 = (p.p711 / locals.var_wbin);
        let assign8820_e4268: f64 = (assign8820_e4264 + assign8820_e4267);
        let assign8820_e4271: f64 = (p.p799 / locals.var_lwbin);
        let assign8820_e4272: f64 = (assign8820_e4268 + assign8820_e4271);
        locals.var_uc_ibpc1 = assign8820_e4272;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign8830_e4276: f64 = (p.p624 / locals.var_lbin);
        let assign8830_e4277: f64 = (p.p281 + assign8830_e4276);
        let assign8830_e4280: f64 = (p.p712 / locals.var_wbin);
        let assign8830_e4281: f64 = (assign8830_e4277 + assign8830_e4280);
        let assign8830_e4284: f64 = (p.p800 / locals.var_lwbin);
        let assign8830_e4285: f64 = (assign8830_e4281 + assign8830_e4284);
        locals.var_uc_ibpc2 = assign8830_e4285;
        locals.var_uc_ibpc2_rv = 0.0;

        let assign8840_e4289: f64 = (p.p625 / locals.var_lbin);
        let assign8840_e4290: f64 = (p.p79 + assign8840_e4289);
        let assign8840_e4293: f64 = (p.p713 / locals.var_wbin);
        let assign8840_e4294: f64 = (assign8840_e4290 + assign8840_e4293);
        let assign8840_e4297: f64 = (p.p801 / locals.var_lwbin);
        let assign8840_e4298: f64 = (assign8840_e4294 + assign8840_e4297);
        locals.var_uc_cgbo = assign8840_e4298;
        locals.var_uc_cgbo_rv = 0.0;

        let assign8850_e4302: f64 = (p.p626 / locals.var_lbin);
        let assign8850_e4303: f64 = (p.p86 + assign8850_e4302);
        let assign8850_e4306: f64 = (p.p714 / locals.var_wbin);
        let assign8850_e4307: f64 = (assign8850_e4303 + assign8850_e4306);
        let assign8850_e4310: f64 = (p.p802 / locals.var_lwbin);
        let assign8850_e4311: f64 = (assign8850_e4307 + assign8850_e4310);
        locals.var_uc_cvdsover = assign8850_e4311;
        locals.var_uc_cvdsover_rv = 0.0;

        let assign8870_e4328: f64 = (p.p628 / locals.var_lbin);
        let assign8870_e4329: f64 = (p.p76 + assign8870_e4328);
        let assign8870_e4332: f64 = (p.p716 / locals.var_wbin);
        let assign8870_e4333: f64 = (assign8870_e4329 + assign8870_e4332);
        let assign8870_e4336: f64 = (p.p804 / locals.var_lwbin);
        let assign8870_e4337: f64 = (assign8870_e4333 + assign8870_e4336);
        locals.var_uc_npext = assign8870_e4337;
        locals.var_uc_npext_rv = 0.0;

        let assign8880_e4341: f64 = (p.p629 / locals.var_lbin);
        let assign8880_e4342: f64 = (p.p81 + assign8880_e4341);
        let assign8880_e4345: f64 = (p.p717 / locals.var_wbin);
        let assign8880_e4346: f64 = (assign8880_e4342 + assign8880_e4345);
        let assign8880_e4349: f64 = (p.p805 / locals.var_lwbin);
        let assign8880_e4350: f64 = (assign8880_e4346 + assign8880_e4349);
        locals.var_uc_powrat = assign8880_e4350;
        locals.var_uc_powrat_rv = 0.0;

        let assign8890_e4354: f64 = (p.p630 / locals.var_lbin);
        let assign8890_e4355: f64 = (p.p74 + assign8890_e4354);
        let assign8890_e4358: f64 = (p.p718 / locals.var_wbin);
        let assign8890_e4359: f64 = (assign8890_e4355 + assign8890_e4358);
        let assign8890_e4362: f64 = (p.p806 / locals.var_lwbin);
        let assign8890_e4363: f64 = (assign8890_e4359 + assign8890_e4362);
        locals.var_uc_rd = assign8890_e4363;
        locals.var_uc_rd_rv = 0.0;

        let assign8900_e4367: f64 = (p.p631 / locals.var_lbin);
        let assign8900_e4368: f64 = (p.p298 + assign8900_e4367);
        let assign8900_e4371: f64 = (p.p719 / locals.var_wbin);
        let assign8900_e4372: f64 = (assign8900_e4368 + assign8900_e4371);
        let assign8900_e4375: f64 = (p.p807 / locals.var_lwbin);
        let assign8900_e4376: f64 = (assign8900_e4372 + assign8900_e4375);
        locals.var_uc_rd22 = assign8900_e4376;
        locals.var_uc_rd22_rv = 0.0;

        let assign8910_e4380: f64 = (p.p632 / locals.var_lbin);
        let assign8910_e4381: f64 = (p.p83 + assign8910_e4380);
        let assign8910_e4384: f64 = (p.p720 / locals.var_wbin);
        let assign8910_e4385: f64 = (assign8910_e4381 + assign8910_e4384);
        let assign8910_e4388: f64 = (p.p808 / locals.var_lwbin);
        let assign8910_e4389: f64 = (assign8910_e4385 + assign8910_e4388);
        locals.var_uc_rd23 = assign8910_e4389;
        locals.var_uc_rd23_rv = 0.0;

        let assign8920_e4393: f64 = (p.p633 / locals.var_lbin);
        let assign8920_e4394: f64 = (p.p84 + assign8920_e4393);
        let assign8920_e4397: f64 = (p.p721 / locals.var_wbin);
        let assign8920_e4398: f64 = (assign8920_e4394 + assign8920_e4397);
        let assign8920_e4401: f64 = (p.p809 / locals.var_lwbin);
        let assign8920_e4402: f64 = (assign8920_e4398 + assign8920_e4401);
        locals.var_uc_rd24 = assign8920_e4402;
        locals.var_uc_rd24_rv = 0.0;

        let assign8930_e4406: f64 = (p.p634 / locals.var_lbin);
        let assign8930_e4407: f64 = (p.p62 + assign8930_e4406);
        let assign8930_e4410: f64 = (p.p722 / locals.var_wbin);
        let assign8930_e4411: f64 = (assign8930_e4407 + assign8930_e4410);
        let assign8930_e4414: f64 = (p.p810 / locals.var_lwbin);
        let assign8930_e4415: f64 = (assign8930_e4411 + assign8930_e4414);
        locals.var_uc_rdict1 = assign8930_e4415;
        locals.var_uc_rdict1_rv = 0.0;

        let assign8940_e4419: f64 = (p.p635 / locals.var_lbin);
        let assign8940_e4420: f64 = (p.p59 + assign8940_e4419);
        let assign8940_e4423: f64 = (p.p723 / locals.var_wbin);
        let assign8940_e4424: f64 = (assign8940_e4420 + assign8940_e4423);
        let assign8940_e4427: f64 = (p.p811 / locals.var_lwbin);
        let assign8940_e4428: f64 = (assign8940_e4424 + assign8940_e4427);
        locals.var_uc_rdov13 = assign8940_e4428;
        locals.var_uc_rdov13_rv = 0.0;

        let assign8950_e4432: f64 = (p.p636 / locals.var_lbin);
        let assign8950_e4433: f64 = (p.p60 + assign8950_e4432);
        let assign8950_e4436: f64 = (p.p724 / locals.var_wbin);
        let assign8950_e4437: f64 = (assign8950_e4433 + assign8950_e4436);
        let assign8950_e4440: f64 = (p.p812 / locals.var_lwbin);
        let assign8950_e4441: f64 = (assign8950_e4437 + assign8950_e4440);
        locals.var_uc_rdslp1 = assign8950_e4441;
        locals.var_uc_rdslp1_rv = 0.0;

        let assign8960_e4445: f64 = (p.p637 / locals.var_lbin);
        let assign8960_e4446: f64 = (p.p85 + assign8960_e4445);
        let assign8960_e4449: f64 = (p.p725 / locals.var_wbin);
        let assign8960_e4450: f64 = (assign8960_e4446 + assign8960_e4449);
        let assign8960_e4453: f64 = (p.p813 / locals.var_lwbin);
        let assign8960_e4454: f64 = (assign8960_e4450 + assign8960_e4453);
        locals.var_uc_rdvb = assign8960_e4454;
        locals.var_uc_rdvb_rv = 0.0;

        let assign8970_e4458: f64 = (p.p638 / locals.var_lbin);
        let assign8970_e4459: f64 = (p.p82 + assign8970_e4458);
        let assign8970_e4462: f64 = (p.p726 / locals.var_wbin);
        let assign8970_e4463: f64 = (assign8970_e4459 + assign8970_e4462);
        let assign8970_e4466: f64 = (p.p814 / locals.var_lwbin);
        let assign8970_e4467: f64 = (assign8970_e4463 + assign8970_e4466);
        locals.var_uc_rdvd = assign8970_e4467;
        locals.var_uc_rdvd_rv = 0.0;

        let assign8980_e4471: f64 = (p.p639 / locals.var_lbin);
        let assign8980_e4472: f64 = (p.p61 + assign8980_e4471);
        let assign8980_e4475: f64 = (p.p727 / locals.var_wbin);
        let assign8980_e4476: f64 = (assign8980_e4472 + assign8980_e4475);
        let assign8980_e4479: f64 = (p.p815 / locals.var_lwbin);
        let assign8980_e4480: f64 = (assign8980_e4476 + assign8980_e4479);
        locals.var_uc_rdvg11 = assign8980_e4480;
        locals.var_uc_rdvg11_rv = 0.0;

        let assign8990_e4484: f64 = (p.p640 / locals.var_lbin);
        let assign8990_e4485: f64 = (p.p75 + assign8990_e4484);
        let assign8990_e4488: f64 = (p.p728 / locals.var_wbin);
        let assign8990_e4489: f64 = (assign8990_e4485 + assign8990_e4488);
        let assign8990_e4492: f64 = (p.p816 / locals.var_lwbin);
        let assign8990_e4493: f64 = (assign8990_e4489 + assign8990_e4492);
        locals.var_uc_rs = assign8990_e4493;
        locals.var_uc_rs_rv = 0.0;

        let assign9000_e4497: f64 = (p.p641 / locals.var_lbin);
        let assign9000_e4498: f64 = (p.p80 + assign9000_e4497);
        let assign9000_e4501: f64 = (p.p729 / locals.var_wbin);
        let assign9000_e4502: f64 = (assign9000_e4498 + assign9000_e4501);
        let assign9000_e4505: f64 = (p.p817 / locals.var_lwbin);
        let assign9000_e4506: f64 = (assign9000_e4502 + assign9000_e4505);
        locals.var_uc_rth0 = assign9000_e4506;
        locals.var_uc_rth0_rv = 0.0;

        let assign9010_e4510: f64 = (p.p642 / locals.var_lbin);
        let assign9010_e4511: f64 = (p.p77 + assign9010_e4510);
        let assign9010_e4514: f64 = (p.p730 / locals.var_wbin);
        let assign9010_e4515: f64 = (assign9010_e4511 + assign9010_e4514);
        let assign9010_e4518: f64 = (p.p818 / locals.var_lwbin);
        let assign9010_e4519: f64 = (assign9010_e4515 + assign9010_e4518);
        locals.var_uc_vover = assign9010_e4519;
        locals.var_uc_vover_rv = 0.0;

        let assign9020_e4523: f64 = (p.p824 / locals.var_lbin);
        let assign9020_e4524: f64 = (p.p493 + assign9020_e4523);
        let assign9020_e4527: f64 = (p.p839 / locals.var_wbin);
        let assign9020_e4528: f64 = (assign9020_e4524 + assign9020_e4527);
        let assign9020_e4531: f64 = (p.p854 / locals.var_lwbin);
        let assign9020_e4532: f64 = (assign9020_e4528 + assign9020_e4531);
        locals.var_uc_js0d = assign9020_e4532;
        locals.var_uc_js0d_rv = 0.0;

        let assign9030_e4536: f64 = (p.p825 / locals.var_lbin);
        let assign9030_e4537: f64 = (p.p494 + assign9030_e4536);
        let assign9030_e4540: f64 = (p.p840 / locals.var_wbin);
        let assign9030_e4541: f64 = (assign9030_e4537 + assign9030_e4540);
        let assign9030_e4544: f64 = (p.p855 / locals.var_lwbin);
        let assign9030_e4545: f64 = (assign9030_e4541 + assign9030_e4544);
        locals.var_uc_js0swd = assign9030_e4545;
        locals.var_uc_js0swd_rv = 0.0;

        let assign9040_e4549: f64 = (p.p826 / locals.var_lbin);
        let assign9040_e4550: f64 = (p.p496 + assign9040_e4549);
        let assign9040_e4553: f64 = (p.p841 / locals.var_wbin);
        let assign9040_e4554: f64 = (assign9040_e4550 + assign9040_e4553);
        let assign9040_e4557: f64 = (p.p856 / locals.var_lwbin);
        let assign9040_e4558: f64 = (assign9040_e4554 + assign9040_e4557);
        locals.var_uc_njd = assign9040_e4558;
        locals.var_uc_njd_rv = 0.0;

        let assign9060_e4575: f64 = (p.p828 / locals.var_lbin);
        let assign9060_e4576: f64 = (p.p515 + assign9060_e4575);
        let assign9060_e4579: f64 = (p.p843 / locals.var_wbin);
        let assign9060_e4580: f64 = (assign9060_e4576 + assign9060_e4579);
        let assign9060_e4583: f64 = (p.p858 / locals.var_lwbin);
        let assign9060_e4584: f64 = (assign9060_e4580 + assign9060_e4583);
        locals.var_uc_vdiffjd = assign9060_e4584;
        locals.var_uc_vdiffjd_rv = 0.0;

        let assign9070_e4588: f64 = (p.p829 / locals.var_lbin);
        let assign9070_e4589: f64 = (p.p516 + assign9070_e4588);
        let assign9070_e4592: f64 = (p.p844 / locals.var_wbin);
        let assign9070_e4593: f64 = (assign9070_e4589 + assign9070_e4592);
        let assign9070_e4596: f64 = (p.p859 / locals.var_lwbin);
        let assign9070_e4597: f64 = (assign9070_e4593 + assign9070_e4596);
        locals.var_uc_js0s = assign9070_e4597;
        locals.var_uc_js0s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9080_e4601: f64 = (p.p830 / locals.var_lbin);
        let assign9080_e4602: f64 = (p.p517 + assign9080_e4601);
        let assign9080_e4605: f64 = (p.p845 / locals.var_wbin);
        let assign9080_e4606: f64 = (assign9080_e4602 + assign9080_e4605);
        let assign9080_e4609: f64 = (p.p860 / locals.var_lwbin);
        let assign9080_e4610: f64 = (assign9080_e4606 + assign9080_e4609);
        locals.var_uc_js0sws = assign9080_e4610;
        locals.var_uc_js0sws_rv = 0.0;

        let assign9090_e4614: f64 = (p.p831 / locals.var_lbin);
        let assign9090_e4615: f64 = (p.p519 + assign9090_e4614);
        let assign9090_e4618: f64 = (p.p846 / locals.var_wbin);
        let assign9090_e4619: f64 = (assign9090_e4615 + assign9090_e4618);
        let assign9090_e4622: f64 = (p.p861 / locals.var_lwbin);
        let assign9090_e4623: f64 = (assign9090_e4619 + assign9090_e4622);
        locals.var_uc_njs = assign9090_e4623;
        locals.var_uc_njs_rv = 0.0;

        let assign9110_e4640: f64 = (p.p833 / locals.var_lbin);
        let assign9110_e4641: f64 = (p.p538 + assign9110_e4640);
        let assign9110_e4644: f64 = (p.p848 / locals.var_wbin);
        let assign9110_e4645: f64 = (assign9110_e4641 + assign9110_e4644);
        let assign9110_e4648: f64 = (p.p863 / locals.var_lwbin);
        let assign9110_e4649: f64 = (assign9110_e4645 + assign9110_e4648);
        locals.var_uc_vdiffjs = assign9110_e4649;
        locals.var_uc_vdiffjs_rv = 0.0;

        let assign9210_e4700: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign9210_e4700;
        locals.var_guard187_rv = 0.0;

        let (assign9220_e4706, assign9220_e4706_d_n0, assign9220_e4706_d_n2, assign9220_e4706_d_n4, assign9220_e4706_d_n5, assign9220_e4706_d_n6, assign9220_e4706_d_n7, assign9220_e4706_d_n8, assign9220_e4706_d_n9, assign9220_e4706_d_n10, assign9220_e4706_d_n11, assign9220_e4706_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9220_e4704: f64 = (locals.var_lg).powf(p.p342);
        (assign9220_e4704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9220_e4706;
        locals.var_t3_dn0 = assign9220_e4706_d_n0;
        locals.var_t3_dn2 = assign9220_e4706_d_n2;
        locals.var_t3_dn4 = assign9220_e4706_d_n4;
        locals.var_t3_dn5 = assign9220_e4706_d_n5;
        locals.var_t3_dn6 = assign9220_e4706_d_n6;
        locals.var_t3_dn7 = assign9220_e4706_d_n7;
        locals.var_t3_dn8 = assign9220_e4706_d_n8;
        locals.var_t3_dn9 = assign9220_e4706_d_n9;
        locals.var_t3_dn10 = assign9220_e4706_d_n10;
        locals.var_t3_dn11 = assign9220_e4706_d_n11;
        locals.var_t3_dn14 = assign9220_e4706_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9230_e4716, assign9230_e4716_d_n0, assign9230_e4716_d_n2, assign9230_e4716_d_n4, assign9230_e4716_d_n5, assign9230_e4716_d_n6, assign9230_e4716_d_n7, assign9230_e4716_d_n8, assign9230_e4716_d_n9, assign9230_e4716_d_n10, assign9230_e4716_d_n11, assign9230_e4716_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9230_e4712: f64 = (p.p341 / locals.var_t3);
        let assign9230_e4713: f64 = (1.0 + assign9230_e4712);
        let assign9230_e4714: f64 = (locals.var_uc_ndepm * assign9230_e4713);
        (assign9230_e4714, ((locals.var_uc_ndepm_dn0 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn2 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn4 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn5 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn6 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn7 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn8 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn9 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn10 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn11 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn14 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9230_e4716;
        locals.var_uc_ndepm_dn0 = assign9230_e4716_d_n0;
        locals.var_uc_ndepm_dn2 = assign9230_e4716_d_n2;
        locals.var_uc_ndepm_dn4 = assign9230_e4716_d_n4;
        locals.var_uc_ndepm_dn5 = assign9230_e4716_d_n5;
        locals.var_uc_ndepm_dn6 = assign9230_e4716_d_n6;
        locals.var_uc_ndepm_dn7 = assign9230_e4716_d_n7;
        locals.var_uc_ndepm_dn8 = assign9230_e4716_d_n8;
        locals.var_uc_ndepm_dn9 = assign9230_e4716_d_n9;
        locals.var_uc_ndepm_dn10 = assign9230_e4716_d_n10;
        locals.var_uc_ndepm_dn11 = assign9230_e4716_d_n11;
        locals.var_uc_ndepm_dn14 = assign9230_e4716_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign9240_e4719: f64 = if locals.var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign9240_e4719;
        locals.var_guard188_rv = 0.0;

        let (assign9250_e4725, assign9250_e4725_d_n0, assign9250_e4725_d_n2, assign9250_e4725_d_n4, assign9250_e4725_d_n5, assign9250_e4725_d_n6, assign9250_e4725_d_n7, assign9250_e4725_d_n8, assign9250_e4725_d_n9, assign9250_e4725_d_n10, assign9250_e4725_d_n11, assign9250_e4725_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard188 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9250_e4725;
        locals.var_uc_ndepm_dn0 = assign9250_e4725_d_n0;
        locals.var_uc_ndepm_dn2 = assign9250_e4725_d_n2;
        locals.var_uc_ndepm_dn4 = assign9250_e4725_d_n4;
        locals.var_uc_ndepm_dn5 = assign9250_e4725_d_n5;
        locals.var_uc_ndepm_dn6 = assign9250_e4725_d_n6;
        locals.var_uc_ndepm_dn7 = assign9250_e4725_d_n7;
        locals.var_uc_ndepm_dn8 = assign9250_e4725_d_n8;
        locals.var_uc_ndepm_dn9 = assign9250_e4725_d_n9;
        locals.var_uc_ndepm_dn10 = assign9250_e4725_d_n10;
        locals.var_uc_ndepm_dn11 = assign9250_e4725_d_n11;
        locals.var_uc_ndepm_dn14 = assign9250_e4725_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9260_e4731, assign9260_e4731_d_n0, assign9260_e4731_d_n2, assign9260_e4731_d_n4, assign9260_e4731_d_n5, assign9260_e4731_d_n6, assign9260_e4731_d_n7, assign9260_e4731_d_n8, assign9260_e4731_d_n9, assign9260_e4731_d_n10, assign9260_e4731_d_n11, assign9260_e4731_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9260_e4729: f64 = (locals.var_lg).powf(p.p369);
        (assign9260_e4729, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9260_e4731;
        locals.var_t3_dn0 = assign9260_e4731_d_n0;
        locals.var_t3_dn2 = assign9260_e4731_d_n2;
        locals.var_t3_dn4 = assign9260_e4731_d_n4;
        locals.var_t3_dn5 = assign9260_e4731_d_n5;
        locals.var_t3_dn6 = assign9260_e4731_d_n6;
        locals.var_t3_dn7 = assign9260_e4731_d_n7;
        locals.var_t3_dn8 = assign9260_e4731_d_n8;
        locals.var_t3_dn9 = assign9260_e4731_d_n9;
        locals.var_t3_dn10 = assign9260_e4731_d_n10;
        locals.var_t3_dn11 = assign9260_e4731_d_n11;
        locals.var_t3_dn14 = assign9260_e4731_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9270_e4741, assign9270_e4741_d_n0, assign9270_e4741_d_n2, assign9270_e4741_d_n4, assign9270_e4741_d_n5, assign9270_e4741_d_n6, assign9270_e4741_d_n7, assign9270_e4741_d_n8, assign9270_e4741_d_n9, assign9270_e4741_d_n10, assign9270_e4741_d_n11, assign9270_e4741_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9270_e4737: f64 = (p.p368 / locals.var_t3);
        let assign9270_e4738: f64 = (1.0 + assign9270_e4737);
        let assign9270_e4739: f64 = (locals.var_uc_depvmax * assign9270_e4738);
        (assign9270_e4739, ((locals.var_uc_depvmax_dn0 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn2 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn4 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn5 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn6 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn7 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn8 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn9 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn10 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn11 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn14 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9270_e4741;
        locals.var_uc_depvmax_dn0 = assign9270_e4741_d_n0;
        locals.var_uc_depvmax_dn2 = assign9270_e4741_d_n2;
        locals.var_uc_depvmax_dn4 = assign9270_e4741_d_n4;
        locals.var_uc_depvmax_dn5 = assign9270_e4741_d_n5;
        locals.var_uc_depvmax_dn6 = assign9270_e4741_d_n6;
        locals.var_uc_depvmax_dn7 = assign9270_e4741_d_n7;
        locals.var_uc_depvmax_dn8 = assign9270_e4741_d_n8;
        locals.var_uc_depvmax_dn9 = assign9270_e4741_d_n9;
        locals.var_uc_depvmax_dn10 = assign9270_e4741_d_n10;
        locals.var_uc_depvmax_dn11 = assign9270_e4741_d_n11;
        locals.var_uc_depvmax_dn14 = assign9270_e4741_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9280_e4747, assign9280_e4747_d_n0, assign9280_e4747_d_n2, assign9280_e4747_d_n4, assign9280_e4747_d_n5, assign9280_e4747_d_n6, assign9280_e4747_d_n7, assign9280_e4747_d_n8, assign9280_e4747_d_n9, assign9280_e4747_d_n10, assign9280_e4747_d_n11, assign9280_e4747_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9280_e4745: f64 = (locals.var_lg).powf(p.p362);
        (assign9280_e4745, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9280_e4747;
        locals.var_t3_dn0 = assign9280_e4747_d_n0;
        locals.var_t3_dn2 = assign9280_e4747_d_n2;
        locals.var_t3_dn4 = assign9280_e4747_d_n4;
        locals.var_t3_dn5 = assign9280_e4747_d_n5;
        locals.var_t3_dn6 = assign9280_e4747_d_n6;
        locals.var_t3_dn7 = assign9280_e4747_d_n7;
        locals.var_t3_dn8 = assign9280_e4747_d_n8;
        locals.var_t3_dn9 = assign9280_e4747_d_n9;
        locals.var_t3_dn10 = assign9280_e4747_d_n10;
        locals.var_t3_dn11 = assign9280_e4747_d_n11;
        locals.var_t3_dn14 = assign9280_e4747_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9290_e4757, assign9290_e4757_d_n0, assign9290_e4757_d_n2, assign9290_e4757_d_n4, assign9290_e4757_d_n5, assign9290_e4757_d_n6, assign9290_e4757_d_n7, assign9290_e4757_d_n8, assign9290_e4757_d_n9, assign9290_e4757_d_n10, assign9290_e4757_d_n11, assign9290_e4757_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9290_e4753: f64 = (p.p361 / locals.var_t3);
        let assign9290_e4754: f64 = (1.0 + assign9290_e4753);
        let assign9290_e4755: f64 = (p.p360 * assign9290_e4754);
        (assign9290_e4755, (p.p360 * (-((p.p361 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9290_e4757;
        locals.var_uc_depleak_dn0 = assign9290_e4757_d_n0;
        locals.var_uc_depleak_dn2 = assign9290_e4757_d_n2;
        locals.var_uc_depleak_dn4 = assign9290_e4757_d_n4;
        locals.var_uc_depleak_dn5 = assign9290_e4757_d_n5;
        locals.var_uc_depleak_dn6 = assign9290_e4757_d_n6;
        locals.var_uc_depleak_dn7 = assign9290_e4757_d_n7;
        locals.var_uc_depleak_dn8 = assign9290_e4757_d_n8;
        locals.var_uc_depleak_dn9 = assign9290_e4757_d_n9;
        locals.var_uc_depleak_dn10 = assign9290_e4757_d_n10;
        locals.var_uc_depleak_dn11 = assign9290_e4757_d_n11;
        locals.var_uc_depleak_dn14 = assign9290_e4757_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign9300_e4760: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9300_e4760;
        locals.var_guard189_rv = 0.0;

        let (assign9310_e4766, assign9310_e4766_d_n0, assign9310_e4766_d_n2, assign9310_e4766_d_n4, assign9310_e4766_d_n5, assign9310_e4766_d_n6, assign9310_e4766_d_n7, assign9310_e4766_d_n8, assign9310_e4766_d_n9, assign9310_e4766_d_n10, assign9310_e4766_d_n11, assign9310_e4766_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9310_e4766;
        locals.var_uc_depleak_dn0 = assign9310_e4766_d_n0;
        locals.var_uc_depleak_dn2 = assign9310_e4766_d_n2;
        locals.var_uc_depleak_dn4 = assign9310_e4766_d_n4;
        locals.var_uc_depleak_dn5 = assign9310_e4766_d_n5;
        locals.var_uc_depleak_dn6 = assign9310_e4766_d_n6;
        locals.var_uc_depleak_dn7 = assign9310_e4766_d_n7;
        locals.var_uc_depleak_dn8 = assign9310_e4766_d_n8;
        locals.var_uc_depleak_dn9 = assign9310_e4766_d_n9;
        locals.var_uc_depleak_dn10 = assign9310_e4766_d_n10;
        locals.var_uc_depleak_dn11 = assign9310_e4766_d_n11;
        locals.var_uc_depleak_dn14 = assign9310_e4766_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9320_e4772, assign9320_e4772_d_n0, assign9320_e4772_d_n2, assign9320_e4772_d_n4, assign9320_e4772_d_n5, assign9320_e4772_d_n6, assign9320_e4772_d_n7, assign9320_e4772_d_n8, assign9320_e4772_d_n9, assign9320_e4772_d_n10, assign9320_e4772_d_n11, assign9320_e4772_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9320_e4770: f64 = (locals.var_lg).powf(p.p348);
        (assign9320_e4770, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9320_e4772;
        locals.var_t3_dn0 = assign9320_e4772_d_n0;
        locals.var_t3_dn2 = assign9320_e4772_d_n2;
        locals.var_t3_dn4 = assign9320_e4772_d_n4;
        locals.var_t3_dn5 = assign9320_e4772_d_n5;
        locals.var_t3_dn6 = assign9320_e4772_d_n6;
        locals.var_t3_dn7 = assign9320_e4772_d_n7;
        locals.var_t3_dn8 = assign9320_e4772_d_n8;
        locals.var_t3_dn9 = assign9320_e4772_d_n9;
        locals.var_t3_dn10 = assign9320_e4772_d_n10;
        locals.var_t3_dn11 = assign9320_e4772_d_n11;
        locals.var_t3_dn14 = assign9320_e4772_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9330_e4782, assign9330_e4782_d_n0, assign9330_e4782_d_n2, assign9330_e4782_d_n4, assign9330_e4782_d_n5, assign9330_e4782_d_n6, assign9330_e4782_d_n7, assign9330_e4782_d_n8, assign9330_e4782_d_n9, assign9330_e4782_d_n10, assign9330_e4782_d_n11, assign9330_e4782_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9330_e4778: f64 = (p.p347 / locals.var_t3);
        let assign9330_e4779: f64 = (1.0 + assign9330_e4778);
        let assign9330_e4780: f64 = (p.p346 * assign9330_e4779);
        (assign9330_e4780, (p.p346 * (-((p.p347 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9330_e4782;
        locals.var_uc_depmue0_dn0 = assign9330_e4782_d_n0;
        locals.var_uc_depmue0_dn2 = assign9330_e4782_d_n2;
        locals.var_uc_depmue0_dn4 = assign9330_e4782_d_n4;
        locals.var_uc_depmue0_dn5 = assign9330_e4782_d_n5;
        locals.var_uc_depmue0_dn6 = assign9330_e4782_d_n6;
        locals.var_uc_depmue0_dn7 = assign9330_e4782_d_n7;
        locals.var_uc_depmue0_dn8 = assign9330_e4782_d_n8;
        locals.var_uc_depmue0_dn9 = assign9330_e4782_d_n9;
        locals.var_uc_depmue0_dn10 = assign9330_e4782_d_n10;
        locals.var_uc_depmue0_dn11 = assign9330_e4782_d_n11;
        locals.var_uc_depmue0_dn14 = assign9330_e4782_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign9340_e4785: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9340_e4785;
        locals.var_guard190_rv = 0.0;

        let (assign9350_e4791, assign9350_e4791_d_n0, assign9350_e4791_d_n2, assign9350_e4791_d_n4, assign9350_e4791_d_n5, assign9350_e4791_d_n6, assign9350_e4791_d_n7, assign9350_e4791_d_n8, assign9350_e4791_d_n9, assign9350_e4791_d_n10, assign9350_e4791_d_n11, assign9350_e4791_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard190 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9350_e4791;
        locals.var_uc_depmue0_dn0 = assign9350_e4791_d_n0;
        locals.var_uc_depmue0_dn2 = assign9350_e4791_d_n2;
        locals.var_uc_depmue0_dn4 = assign9350_e4791_d_n4;
        locals.var_uc_depmue0_dn5 = assign9350_e4791_d_n5;
        locals.var_uc_depmue0_dn6 = assign9350_e4791_d_n6;
        locals.var_uc_depmue0_dn7 = assign9350_e4791_d_n7;
        locals.var_uc_depmue0_dn8 = assign9350_e4791_d_n8;
        locals.var_uc_depmue0_dn9 = assign9350_e4791_d_n9;
        locals.var_uc_depmue0_dn10 = assign9350_e4791_d_n10;
        locals.var_uc_depmue0_dn11 = assign9350_e4791_d_n11;
        locals.var_uc_depmue0_dn14 = assign9350_e4791_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9360_e4797, assign9360_e4797_d_n0, assign9360_e4797_d_n2, assign9360_e4797_d_n4, assign9360_e4797_d_n5, assign9360_e4797_d_n6, assign9360_e4797_d_n7, assign9360_e4797_d_n8, assign9360_e4797_d_n9, assign9360_e4797_d_n10, assign9360_e4797_d_n11, assign9360_e4797_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9360_e4795: f64 = (locals.var_lg).powf(p.p351);
        (assign9360_e4795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9360_e4797;
        locals.var_t3_dn0 = assign9360_e4797_d_n0;
        locals.var_t3_dn2 = assign9360_e4797_d_n2;
        locals.var_t3_dn4 = assign9360_e4797_d_n4;
        locals.var_t3_dn5 = assign9360_e4797_d_n5;
        locals.var_t3_dn6 = assign9360_e4797_d_n6;
        locals.var_t3_dn7 = assign9360_e4797_d_n7;
        locals.var_t3_dn8 = assign9360_e4797_d_n8;
        locals.var_t3_dn9 = assign9360_e4797_d_n9;
        locals.var_t3_dn10 = assign9360_e4797_d_n10;
        locals.var_t3_dn11 = assign9360_e4797_d_n11;
        locals.var_t3_dn14 = assign9360_e4797_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9370_e4807, assign9370_e4807_d_n0, assign9370_e4807_d_n2, assign9370_e4807_d_n4, assign9370_e4807_d_n5, assign9370_e4807_d_n6, assign9370_e4807_d_n7, assign9370_e4807_d_n8, assign9370_e4807_d_n9, assign9370_e4807_d_n10, assign9370_e4807_d_n11, assign9370_e4807_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9370_e4803: f64 = (p.p350 / locals.var_t3);
        let assign9370_e4804: f64 = (1.0 + assign9370_e4803);
        let assign9370_e4805: f64 = (p.p349 * assign9370_e4804);
        (assign9370_e4805, (p.p349 * (-((p.p350 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9370_e4807;
        locals.var_uc_depmue1_dn0 = assign9370_e4807_d_n0;
        locals.var_uc_depmue1_dn2 = assign9370_e4807_d_n2;
        locals.var_uc_depmue1_dn4 = assign9370_e4807_d_n4;
        locals.var_uc_depmue1_dn5 = assign9370_e4807_d_n5;
        locals.var_uc_depmue1_dn6 = assign9370_e4807_d_n6;
        locals.var_uc_depmue1_dn7 = assign9370_e4807_d_n7;
        locals.var_uc_depmue1_dn8 = assign9370_e4807_d_n8;
        locals.var_uc_depmue1_dn9 = assign9370_e4807_d_n9;
        locals.var_uc_depmue1_dn10 = assign9370_e4807_d_n10;
        locals.var_uc_depmue1_dn11 = assign9370_e4807_d_n11;
        locals.var_uc_depmue1_dn14 = assign9370_e4807_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let assign9380_e4810: f64 = if locals.var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9380_e4810;
        locals.var_guard191_rv = 0.0;

        let (assign9390_e4816, assign9390_e4816_d_n0, assign9390_e4816_d_n2, assign9390_e4816_d_n4, assign9390_e4816_d_n5, assign9390_e4816_d_n6, assign9390_e4816_d_n7, assign9390_e4816_d_n8, assign9390_e4816_d_n9, assign9390_e4816_d_n10, assign9390_e4816_d_n11, assign9390_e4816_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9390_e4816;
        locals.var_uc_depmue1_dn0 = assign9390_e4816_d_n0;
        locals.var_uc_depmue1_dn2 = assign9390_e4816_d_n2;
        locals.var_uc_depmue1_dn4 = assign9390_e4816_d_n4;
        locals.var_uc_depmue1_dn5 = assign9390_e4816_d_n5;
        locals.var_uc_depmue1_dn6 = assign9390_e4816_d_n6;
        locals.var_uc_depmue1_dn7 = assign9390_e4816_d_n7;
        locals.var_uc_depmue1_dn8 = assign9390_e4816_d_n8;
        locals.var_uc_depmue1_dn9 = assign9390_e4816_d_n9;
        locals.var_uc_depmue1_dn10 = assign9390_e4816_d_n10;
        locals.var_uc_depmue1_dn11 = assign9390_e4816_d_n11;
        locals.var_uc_depmue1_dn14 = assign9390_e4816_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9400_e4822, assign9400_e4822_d_n0, assign9400_e4822_d_n2, assign9400_e4822_d_n4, assign9400_e4822_d_n5, assign9400_e4822_d_n6, assign9400_e4822_d_n7, assign9400_e4822_d_n8, assign9400_e4822_d_n9, assign9400_e4822_d_n10, assign9400_e4822_d_n11, assign9400_e4822_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9400_e4820: f64 = (locals.var_lg).powf(p.p357);
        (assign9400_e4820, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9400_e4822;
        locals.var_t3_dn0 = assign9400_e4822_d_n0;
        locals.var_t3_dn2 = assign9400_e4822_d_n2;
        locals.var_t3_dn4 = assign9400_e4822_d_n4;
        locals.var_t3_dn5 = assign9400_e4822_d_n5;
        locals.var_t3_dn6 = assign9400_e4822_d_n6;
        locals.var_t3_dn7 = assign9400_e4822_d_n7;
        locals.var_t3_dn8 = assign9400_e4822_d_n8;
        locals.var_t3_dn9 = assign9400_e4822_d_n9;
        locals.var_t3_dn10 = assign9400_e4822_d_n10;
        locals.var_t3_dn11 = assign9400_e4822_d_n11;
        locals.var_t3_dn14 = assign9400_e4822_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9410_e4832, assign9410_e4832_d_n0, assign9410_e4832_d_n2, assign9410_e4832_d_n4, assign9410_e4832_d_n5, assign9410_e4832_d_n6, assign9410_e4832_d_n7, assign9410_e4832_d_n8, assign9410_e4832_d_n9, assign9410_e4832_d_n10, assign9410_e4832_d_n11, assign9410_e4832_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9410_e4828: f64 = (p.p356 / locals.var_t3);
        let assign9410_e4829: f64 = (1.0 + assign9410_e4828);
        let assign9410_e4830: f64 = (p.p354 * assign9410_e4829);
        (assign9410_e4830, (p.p354 * (-((p.p356 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9410_e4832;
        locals.var_uc_depmueback0_dn0 = assign9410_e4832_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9410_e4832_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9410_e4832_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9410_e4832_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9410_e4832_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9410_e4832_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9410_e4832_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9410_e4832_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9410_e4832_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9410_e4832_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9410_e4832_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign9420_e4835: f64 = if locals.var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9420_e4835;
        locals.var_guard192_rv = 0.0;

        let (assign9430_e4841, assign9430_e4841_d_n0, assign9430_e4841_d_n2, assign9430_e4841_d_n4, assign9430_e4841_d_n5, assign9430_e4841_d_n6, assign9430_e4841_d_n7, assign9430_e4841_d_n8, assign9430_e4841_d_n9, assign9430_e4841_d_n10, assign9430_e4841_d_n11, assign9430_e4841_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9430_e4841;
        locals.var_uc_depmueback0_dn0 = assign9430_e4841_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9430_e4841_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9430_e4841_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9430_e4841_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9430_e4841_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9430_e4841_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9430_e4841_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9430_e4841_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9430_e4841_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9430_e4841_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9430_e4841_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let (assign9440_e4847, assign9440_e4847_d_n0, assign9440_e4847_d_n2, assign9440_e4847_d_n4, assign9440_e4847_d_n5, assign9440_e4847_d_n6, assign9440_e4847_d_n7, assign9440_e4847_d_n8, assign9440_e4847_d_n9, assign9440_e4847_d_n10, assign9440_e4847_d_n11, assign9440_e4847_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9440_e4845: f64 = (locals.var_lg).powf(p.p359);
        (assign9440_e4845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9440_e4847;
        locals.var_t3_dn0 = assign9440_e4847_d_n0;
        locals.var_t3_dn2 = assign9440_e4847_d_n2;
        locals.var_t3_dn4 = assign9440_e4847_d_n4;
        locals.var_t3_dn5 = assign9440_e4847_d_n5;
        locals.var_t3_dn6 = assign9440_e4847_d_n6;
        locals.var_t3_dn7 = assign9440_e4847_d_n7;
        locals.var_t3_dn8 = assign9440_e4847_d_n8;
        locals.var_t3_dn9 = assign9440_e4847_d_n9;
        locals.var_t3_dn10 = assign9440_e4847_d_n10;
        locals.var_t3_dn11 = assign9440_e4847_d_n11;
        locals.var_t3_dn14 = assign9440_e4847_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9450_e4857, assign9450_e4857_d_n0, assign9450_e4857_d_n2, assign9450_e4857_d_n4, assign9450_e4857_d_n5, assign9450_e4857_d_n6, assign9450_e4857_d_n7, assign9450_e4857_d_n8, assign9450_e4857_d_n9, assign9450_e4857_d_n10, assign9450_e4857_d_n11, assign9450_e4857_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9450_e4853: f64 = (p.p358 / locals.var_t3);
        let assign9450_e4854: f64 = (1.0 + assign9450_e4853);
        let assign9450_e4855: f64 = (p.p355 * assign9450_e4854);
        (assign9450_e4855, (p.p355 * (-((p.p358 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9450_e4857;
        locals.var_uc_depmueback1_dn0 = assign9450_e4857_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9450_e4857_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9450_e4857_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9450_e4857_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9450_e4857_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9450_e4857_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9450_e4857_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9450_e4857_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9450_e4857_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9450_e4857_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9450_e4857_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

        let assign9460_e4860: f64 = if locals.var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign9460_e4860;
        locals.var_guard193_rv = 0.0;

        let (assign9470_e4866, assign9470_e4866_d_n0, assign9470_e4866_d_n2, assign9470_e4866_d_n4, assign9470_e4866_d_n5, assign9470_e4866_d_n6, assign9470_e4866_d_n7, assign9470_e4866_d_n8, assign9470_e4866_d_n9, assign9470_e4866_d_n10, assign9470_e4866_d_n11, assign9470_e4866_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard193 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9470_e4866;
        locals.var_uc_depmueback1_dn0 = assign9470_e4866_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9470_e4866_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9470_e4866_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9470_e4866_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9470_e4866_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9470_e4866_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9470_e4866_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9470_e4866_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9470_e4866_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9470_e4866_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9470_e4866_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9480_e4872, assign9480_e4872_d_n0, assign9480_e4872_d_n2, assign9480_e4872_d_n4, assign9480_e4872_d_n5, assign9480_e4872_d_n6, assign9480_e4872_d_n7, assign9480_e4872_d_n8, assign9480_e4872_d_n9, assign9480_e4872_d_n10, assign9480_e4872_d_n11, assign9480_e4872_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9480_e4870: f64 = (locals.var_lg).powf(p.p373);
        (assign9480_e4870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9480_e4872;
        locals.var_t3_dn0 = assign9480_e4872_d_n0;
        locals.var_t3_dn2 = assign9480_e4872_d_n2;
        locals.var_t3_dn4 = assign9480_e4872_d_n4;
        locals.var_t3_dn5 = assign9480_e4872_d_n5;
        locals.var_t3_dn6 = assign9480_e4872_d_n6;
        locals.var_t3_dn7 = assign9480_e4872_d_n7;
        locals.var_t3_dn8 = assign9480_e4872_d_n8;
        locals.var_t3_dn9 = assign9480_e4872_d_n9;
        locals.var_t3_dn10 = assign9480_e4872_d_n10;
        locals.var_t3_dn11 = assign9480_e4872_d_n11;
        locals.var_t3_dn14 = assign9480_e4872_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9490_e4882, assign9490_e4882_d_n0, assign9490_e4882_d_n2, assign9490_e4882_d_n4, assign9490_e4882_d_n5, assign9490_e4882_d_n6, assign9490_e4882_d_n7, assign9490_e4882_d_n8, assign9490_e4882_d_n9, assign9490_e4882_d_n10, assign9490_e4882_d_n11, assign9490_e4882_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9490_e4878: f64 = (p.p372 / locals.var_t3);
        let assign9490_e4879: f64 = (1.0 + assign9490_e4878);
        let assign9490_e4880: f64 = (locals.var_uc_depvdsef1 * assign9490_e4879);
        (assign9490_e4880, ((locals.var_uc_depvdsef1_dn0 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn2 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn4 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn5 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn6 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn7 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn8 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn9 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn10 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn11 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn14 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9490_e4882;
        locals.var_uc_depvdsef1_dn0 = assign9490_e4882_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9490_e4882_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9490_e4882_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9490_e4882_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9490_e4882_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9490_e4882_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9490_e4882_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9490_e4882_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9490_e4882_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9490_e4882_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9490_e4882_d_n14;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9500_e4888, assign9500_e4888_d_n0, assign9500_e4888_d_n2, assign9500_e4888_d_n4, assign9500_e4888_d_n5, assign9500_e4888_d_n6, assign9500_e4888_d_n7, assign9500_e4888_d_n8, assign9500_e4888_d_n9, assign9500_e4888_d_n10, assign9500_e4888_d_n11, assign9500_e4888_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9500_e4886: f64 = (locals.var_lg).powf(p.p375);
        (assign9500_e4886, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9500_e4888;
        locals.var_t3_dn0 = assign9500_e4888_d_n0;
        locals.var_t3_dn2 = assign9500_e4888_d_n2;
        locals.var_t3_dn4 = assign9500_e4888_d_n4;
        locals.var_t3_dn5 = assign9500_e4888_d_n5;
        locals.var_t3_dn6 = assign9500_e4888_d_n6;
        locals.var_t3_dn7 = assign9500_e4888_d_n7;
        locals.var_t3_dn8 = assign9500_e4888_d_n8;
        locals.var_t3_dn9 = assign9500_e4888_d_n9;
        locals.var_t3_dn10 = assign9500_e4888_d_n10;
        locals.var_t3_dn11 = assign9500_e4888_d_n11;
        locals.var_t3_dn14 = assign9500_e4888_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9510_e4898, assign9510_e4898_d_n0, assign9510_e4898_d_n2, assign9510_e4898_d_n4, assign9510_e4898_d_n5, assign9510_e4898_d_n6, assign9510_e4898_d_n7, assign9510_e4898_d_n8, assign9510_e4898_d_n9, assign9510_e4898_d_n10, assign9510_e4898_d_n11, assign9510_e4898_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9510_e4894: f64 = (p.p374 / locals.var_t3);
        let assign9510_e4895: f64 = (1.0 + assign9510_e4894);
        let assign9510_e4896: f64 = (locals.var_uc_depvdsef2 * assign9510_e4895);
        (assign9510_e4896, ((locals.var_uc_depvdsef2_dn0 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn2 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn4 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn5 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn6 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn7 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn8 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn9 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn10 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn11 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn14 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9510_e4898;
        locals.var_uc_depvdsef2_dn0 = assign9510_e4898_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9510_e4898_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9510_e4898_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9510_e4898_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9510_e4898_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9510_e4898_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9510_e4898_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9510_e4898_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9510_e4898_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9510_e4898_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9510_e4898_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign9520_e4901: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign9520_e4901;
        locals.var_guard194_rv = 0.0;

        let (assign9530_e4907, assign9530_e4907_d_n0, assign9530_e4907_d_n2, assign9530_e4907_d_n4, assign9530_e4907_d_n5, assign9530_e4907_d_n6, assign9530_e4907_d_n7, assign9530_e4907_d_n8, assign9530_e4907_d_n9, assign9530_e4907_d_n10, assign9530_e4907_d_n11, assign9530_e4907_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard194 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9530_e4907;
        locals.var_uc_depvdsef2_dn0 = assign9530_e4907_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9530_e4907_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9530_e4907_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9530_e4907_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9530_e4907_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9530_e4907_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9530_e4907_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9530_e4907_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9530_e4907_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9530_e4907_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9530_e4907_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let (assign9540_e4912, assign9540_e4912_d_n0, assign9540_e4912_d_n2, assign9540_e4912_d_n4, assign9540_e4912_d_n5, assign9540_e4912_d_n6, assign9540_e4912_d_n7, assign9540_e4912_d_n8, assign9540_e4912_d_n9, assign9540_e4912_d_n10, assign9540_e4912_d_n11, assign9540_e4912_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9540_e4912;
        locals.var_uc_ndepm_dn0 = assign9540_e4912_d_n0;
        locals.var_uc_ndepm_dn2 = assign9540_e4912_d_n2;
        locals.var_uc_ndepm_dn4 = assign9540_e4912_d_n4;
        locals.var_uc_ndepm_dn5 = assign9540_e4912_d_n5;
        locals.var_uc_ndepm_dn6 = assign9540_e4912_d_n6;
        locals.var_uc_ndepm_dn7 = assign9540_e4912_d_n7;
        locals.var_uc_ndepm_dn8 = assign9540_e4912_d_n8;
        locals.var_uc_ndepm_dn9 = assign9540_e4912_d_n9;
        locals.var_uc_ndepm_dn10 = assign9540_e4912_d_n10;
        locals.var_uc_ndepm_dn11 = assign9540_e4912_d_n11;
        locals.var_uc_ndepm_dn14 = assign9540_e4912_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9550_e4917, assign9550_e4917_d_n0, assign9550_e4917_d_n2, assign9550_e4917_d_n4, assign9550_e4917_d_n5, assign9550_e4917_d_n6, assign9550_e4917_d_n7, assign9550_e4917_d_n8, assign9550_e4917_d_n9, assign9550_e4917_d_n10, assign9550_e4917_d_n11, assign9550_e4917_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9550_e4917;
        locals.var_uc_depvmax_dn0 = assign9550_e4917_d_n0;
        locals.var_uc_depvmax_dn2 = assign9550_e4917_d_n2;
        locals.var_uc_depvmax_dn4 = assign9550_e4917_d_n4;
        locals.var_uc_depvmax_dn5 = assign9550_e4917_d_n5;
        locals.var_uc_depvmax_dn6 = assign9550_e4917_d_n6;
        locals.var_uc_depvmax_dn7 = assign9550_e4917_d_n7;
        locals.var_uc_depvmax_dn8 = assign9550_e4917_d_n8;
        locals.var_uc_depvmax_dn9 = assign9550_e4917_d_n9;
        locals.var_uc_depvmax_dn10 = assign9550_e4917_d_n10;
        locals.var_uc_depvmax_dn11 = assign9550_e4917_d_n11;
        locals.var_uc_depvmax_dn14 = assign9550_e4917_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9560_e4922, assign9560_e4922_d_n0, assign9560_e4922_d_n2, assign9560_e4922_d_n4, assign9560_e4922_d_n5, assign9560_e4922_d_n6, assign9560_e4922_d_n7, assign9560_e4922_d_n8, assign9560_e4922_d_n9, assign9560_e4922_d_n10, assign9560_e4922_d_n11, assign9560_e4922_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9560_e4922;
        locals.var_uc_depleak_dn0 = assign9560_e4922_d_n0;
        locals.var_uc_depleak_dn2 = assign9560_e4922_d_n2;
        locals.var_uc_depleak_dn4 = assign9560_e4922_d_n4;
        locals.var_uc_depleak_dn5 = assign9560_e4922_d_n5;
        locals.var_uc_depleak_dn6 = assign9560_e4922_d_n6;
        locals.var_uc_depleak_dn7 = assign9560_e4922_d_n7;
        locals.var_uc_depleak_dn8 = assign9560_e4922_d_n8;
        locals.var_uc_depleak_dn9 = assign9560_e4922_d_n9;
        locals.var_uc_depleak_dn10 = assign9560_e4922_d_n10;
        locals.var_uc_depleak_dn11 = assign9560_e4922_d_n11;
        locals.var_uc_depleak_dn14 = assign9560_e4922_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9570_e4927, assign9570_e4927_d_n0, assign9570_e4927_d_n2, assign9570_e4927_d_n4, assign9570_e4927_d_n5, assign9570_e4927_d_n6, assign9570_e4927_d_n7, assign9570_e4927_d_n8, assign9570_e4927_d_n9, assign9570_e4927_d_n10, assign9570_e4927_d_n11, assign9570_e4927_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9570_e4927;
        locals.var_uc_depmue0_dn0 = assign9570_e4927_d_n0;
        locals.var_uc_depmue0_dn2 = assign9570_e4927_d_n2;
        locals.var_uc_depmue0_dn4 = assign9570_e4927_d_n4;
        locals.var_uc_depmue0_dn5 = assign9570_e4927_d_n5;
        locals.var_uc_depmue0_dn6 = assign9570_e4927_d_n6;
        locals.var_uc_depmue0_dn7 = assign9570_e4927_d_n7;
        locals.var_uc_depmue0_dn8 = assign9570_e4927_d_n8;
        locals.var_uc_depmue0_dn9 = assign9570_e4927_d_n9;
        locals.var_uc_depmue0_dn10 = assign9570_e4927_d_n10;
        locals.var_uc_depmue0_dn11 = assign9570_e4927_d_n11;
        locals.var_uc_depmue0_dn14 = assign9570_e4927_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9580_e4932, assign9580_e4932_d_n0, assign9580_e4932_d_n2, assign9580_e4932_d_n4, assign9580_e4932_d_n5, assign9580_e4932_d_n6, assign9580_e4932_d_n7, assign9580_e4932_d_n8, assign9580_e4932_d_n9, assign9580_e4932_d_n10, assign9580_e4932_d_n11, assign9580_e4932_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9580_e4932;
        locals.var_uc_depmue1_dn0 = assign9580_e4932_d_n0;
        locals.var_uc_depmue1_dn2 = assign9580_e4932_d_n2;
        locals.var_uc_depmue1_dn4 = assign9580_e4932_d_n4;
        locals.var_uc_depmue1_dn5 = assign9580_e4932_d_n5;
        locals.var_uc_depmue1_dn6 = assign9580_e4932_d_n6;
        locals.var_uc_depmue1_dn7 = assign9580_e4932_d_n7;
        locals.var_uc_depmue1_dn8 = assign9580_e4932_d_n8;
        locals.var_uc_depmue1_dn9 = assign9580_e4932_d_n9;
        locals.var_uc_depmue1_dn10 = assign9580_e4932_d_n10;
        locals.var_uc_depmue1_dn11 = assign9580_e4932_d_n11;
        locals.var_uc_depmue1_dn14 = assign9580_e4932_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9590_e4937, assign9590_e4937_d_n0, assign9590_e4937_d_n2, assign9590_e4937_d_n4, assign9590_e4937_d_n5, assign9590_e4937_d_n6, assign9590_e4937_d_n7, assign9590_e4937_d_n8, assign9590_e4937_d_n9, assign9590_e4937_d_n10, assign9590_e4937_d_n11, assign9590_e4937_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9590_e4937;
        locals.var_uc_depmueback0_dn0 = assign9590_e4937_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9590_e4937_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9590_e4937_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9590_e4937_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9590_e4937_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9590_e4937_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9590_e4937_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9590_e4937_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9590_e4937_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9590_e4937_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9590_e4937_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let (assign9600_e4942, assign9600_e4942_d_n0, assign9600_e4942_d_n2, assign9600_e4942_d_n4, assign9600_e4942_d_n5, assign9600_e4942_d_n6, assign9600_e4942_d_n7, assign9600_e4942_d_n8, assign9600_e4942_d_n9, assign9600_e4942_d_n10, assign9600_e4942_d_n11, assign9600_e4942_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9600_e4942;
        locals.var_uc_depmueback1_dn0 = assign9600_e4942_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9600_e4942_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9600_e4942_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9600_e4942_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9600_e4942_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9600_e4942_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9600_e4942_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9600_e4942_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9600_e4942_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9600_e4942_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9600_e4942_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

        let (assign9610_e4947, assign9610_e4947_d_n0, assign9610_e4947_d_n2, assign9610_e4947_d_n4, assign9610_e4947_d_n5, assign9610_e4947_d_n6, assign9610_e4947_d_n7, assign9610_e4947_d_n8, assign9610_e4947_d_n9, assign9610_e4947_d_n10, assign9610_e4947_d_n11, assign9610_e4947_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9610_e4947;
        locals.var_uc_depvdsef1_dn0 = assign9610_e4947_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9610_e4947_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9610_e4947_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9610_e4947_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9610_e4947_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9610_e4947_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9610_e4947_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9610_e4947_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9610_e4947_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9610_e4947_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9610_e4947_d_n14;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9620_e4952, assign9620_e4952_d_n0, assign9620_e4952_d_n2, assign9620_e4952_d_n4, assign9620_e4952_d_n5, assign9620_e4952_d_n6, assign9620_e4952_d_n7, assign9620_e4952_d_n8, assign9620_e4952_d_n9, assign9620_e4952_d_n10, assign9620_e4952_d_n11, assign9620_e4952_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9620_e4952;
        locals.var_uc_depvdsef2_dn0 = assign9620_e4952_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9620_e4952_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9620_e4952_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9620_e4952_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9620_e4952_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9620_e4952_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9620_e4952_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9620_e4952_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9620_e4952_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9620_e4952_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9620_e4952_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign10140_e5325: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign10140_e5327: f64 = if assign10140_e5325 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign10140_e5327;
        locals.var_guard246_rv = 0.0;

        let (assign10150_e5333,) = {
    if (locals.var_guard246 != 0.0) {
        let assign10150_e5331: f64 = (1.0 / locals.var_uc_xldld);
        (assign10150_e5331,)
    } else {
        (locals.var_uc_xpdv,)
    }
};
        locals.var_uc_xpdv = assign10150_e5333;
        locals.var_uc_xpdv_rv = 0.0;

        let assign10170_e5361: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (locals.var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (locals.var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign10170_e5361;
        locals.var_guard248_rv = 0.0;

        let (assign10180_e5365,) = {
    if (locals.var_guard248 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10180_e5365;
        locals.var_uc_cordrift_rv = 0.0;

        let (assign10190_e5370,) = {
    if (locals.var_guard248 == 0.0) {
        (p.p40,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10190_e5370;
        locals.var_uc_cordrift_rv = 0.0;

        let assign10200_e5373: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign10200_e5373;
        locals.var_guard249_rv = 0.0;

        let (assign10210_e5382,) = {
    if (locals.var_guard249 != 0.0) {
        let (assign10210_e5380,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10210_e5380,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10210_e5382;
        locals.var_flg_rd_rv = 0.0;

        let (assign10220_e5391,) = {
    if (locals.var_guard249 != 0.0) {
        let (assign10220_e5389,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10220_e5389,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10220_e5391;
        locals.var_flg_rs_rv = 0.0;

        let assign10230_e5398: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign10230_e5398;
        locals.var_guard250_rv = 0.0;

        let (assign10240_e5405,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10240_e5405;
        locals.var_flg_rd_rv = 0.0;

        let (assign10250_e5412,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10250_e5412;
        locals.var_flg_rs_rv = 0.0;

        let (assign10260_e5444, assign10260_e5444_d_n0, assign10260_e5444_d_n2, assign10260_e5444_d_n4, assign10260_e5444_d_n5, assign10260_e5444_d_n6, assign10260_e5444_d_n7, assign10260_e5444_d_n8, assign10260_e5444_d_n9, assign10260_e5444_d_n10, assign10260_e5444_d_n11, assign10260_e5444_d_n14,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let assign10260_e5420: f64 = (p.p130 * p.p2);
        let assign10260_e5422: f64 = (assign10260_e5420 * p.p7);
        let assign10260_e5425: f64 = (locals.var_uc_rd + locals.var_uc_rdvd);
        let assign10260_e5428: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign10260_e5430: f64 = (assign10260_e5428 * 1000000.0);
        let assign10260_e5432: f64 = (assign10260_e5430 + locals.var_uc_rdict1);
        let assign10260_e5433: f64 = (assign10260_e5425 * assign10260_e5432);
        let assign10260_e5436: f64 = (p.p68 * p.p100);
        let assign10260_e5438: f64 = (assign10260_e5436 * 1000000.0);
        let assign10260_e5440: f64 = (assign10260_e5438 + p.p101);
        let assign10260_e5441: f64 = (assign10260_e5433 * assign10260_e5440);
        let assign10260_e5442: f64 = (assign10260_e5422 + assign10260_e5441);
        (assign10260_e5442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10260_e5444;
        locals.var_t1_dn0 = assign10260_e5444_d_n0;
        locals.var_t1_dn2 = assign10260_e5444_d_n2;
        locals.var_t1_dn4 = assign10260_e5444_d_n4;
        locals.var_t1_dn5 = assign10260_e5444_d_n5;
        locals.var_t1_dn6 = assign10260_e5444_d_n6;
        locals.var_t1_dn7 = assign10260_e5444_d_n7;
        locals.var_t1_dn8 = assign10260_e5444_d_n8;
        locals.var_t1_dn9 = assign10260_e5444_d_n9;
        locals.var_t1_dn10 = assign10260_e5444_d_n10;
        locals.var_t1_dn11 = assign10260_e5444_d_n11;
        locals.var_t1_dn14 = assign10260_e5444_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10270_e5457,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let (assign10270_e5455,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5455,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10270_e5457;
        locals.var_flg_rd_rv = 0.0;

        let (assign10280_e5487, assign10280_e5487_d_n0, assign10280_e5487_d_n2, assign10280_e5487_d_n4, assign10280_e5487_d_n5, assign10280_e5487_d_n6, assign10280_e5487_d_n7, assign10280_e5487_d_n8, assign10280_e5487_d_n9, assign10280_e5487_d_n10, assign10280_e5487_d_n11, assign10280_e5487_d_n14,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let assign10280_e5465: f64 = (p.p131 * p.p3);
        let assign10280_e5467: f64 = (assign10280_e5465 * p.p7);
        let assign10280_e5471: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign10280_e5473: f64 = (assign10280_e5471 * 1000000.0);
        let assign10280_e5475: f64 = (assign10280_e5473 + locals.var_uc_rdict1);
        let assign10280_e5476: f64 = (locals.var_uc_rs * assign10280_e5475);
        let assign10280_e5479: f64 = (p.p70 * p.p100);
        let assign10280_e5481: f64 = (assign10280_e5479 * 1000000.0);
        let assign10280_e5483: f64 = (assign10280_e5481 + p.p101);
        let assign10280_e5484: f64 = (assign10280_e5476 * assign10280_e5483);
        let assign10280_e5485: f64 = (assign10280_e5467 + assign10280_e5484);
        (assign10280_e5485, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10280_e5487;
        locals.var_t1_dn0 = assign10280_e5487_d_n0;
        locals.var_t1_dn2 = assign10280_e5487_d_n2;
        locals.var_t1_dn4 = assign10280_e5487_d_n4;
        locals.var_t1_dn5 = assign10280_e5487_d_n5;
        locals.var_t1_dn6 = assign10280_e5487_d_n6;
        locals.var_t1_dn7 = assign10280_e5487_d_n7;
        locals.var_t1_dn8 = assign10280_e5487_d_n8;
        locals.var_t1_dn9 = assign10280_e5487_d_n9;
        locals.var_t1_dn10 = assign10280_e5487_d_n10;
        locals.var_t1_dn11 = assign10280_e5487_d_n11;
        locals.var_t1_dn14 = assign10280_e5487_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10290_e5500,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let (assign10290_e5498,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10290_e5498,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10290_e5500;
        locals.var_flg_rs_rv = 0.0;

        let assign10300_e5503: f64 = (p.p12 / 1e-6);
        locals.var_mks_nsubcdfm = assign10300_e5503;
        locals.var_mks_nsubcdfm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10310_e5506: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign10310_e5506;
        locals.var_mks_subld2_rv = 0.0;

        let assign10320_e5509: f64 = (locals.var_uc_nsubc / 1e-6);
        locals.var_uc_nsubc = assign10320_e5509;
        locals.var_uc_nsubc_rv = 0.0;

        let assign10330_e5512: f64 = (locals.var_uc_nsubp / 1e-6);
        locals.var_uc_nsubp = assign10330_e5512;
        locals.var_uc_nsubp_rv = 0.0;

        let assign10340_e5515: f64 = (locals.var_uc_nsti / 1e-6);
        locals.var_uc_nsti = assign10340_e5515;
        locals.var_uc_nsti_rv = 0.0;

        let assign10350_e5518: f64 = (locals.var_uc_nover / 1e-6);
        locals.var_uc_nover = assign10350_e5518;
        locals.var_uc_nover_rv = 0.0;

        let assign10360_e5521: f64 = (locals.var_uc_novers / 1e-6);
        locals.var_uc_novers = assign10360_e5521;
        locals.var_uc_novers_rv = 0.0;

        let assign10370_e5524: f64 = (locals.var_uc_nsubpsti1 / 100.0);
        locals.var_uc_nsubpsti1 = assign10370_e5524;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign10380_e5527: f64 = (locals.var_uc_muesti1 / 100.0);
        locals.var_uc_muesti1 = assign10380_e5527;
        locals.var_uc_muesti1_rv = 0.0;

        let assign10390_e5530: f64 = (locals.var_uc_vmax / 100.0);
        locals.var_uc_vmax = assign10390_e5530;
        locals.var_uc_vmax_rv = 0.0;

        let assign10400_e5533: f64 = (locals.var_uc_wfc * 10000.0);
        locals.var_uc_wfc = assign10400_e5533;
        locals.var_uc_wfc_rv = 0.0;

        let assign10410_e5536: f64 = (locals.var_uc_glksd1 / 100.0);
        locals.var_uc_glksd1 = assign10410_e5536;
        locals.var_uc_glksd1_rv = 0.0;

        let assign10420_e5539: f64 = (locals.var_uc_glksd2 * 100.0);
        locals.var_uc_glksd2 = assign10420_e5539;
        locals.var_uc_glksd2_rv = 0.0;

        let assign10430_e5542: f64 = (locals.var_uc_gleak2 * 100.0);
        locals.var_uc_gleak2 = assign10430_e5542;
        locals.var_uc_gleak2_rv = 0.0;

        let assign10440_e5545: f64 = (locals.var_uc_glkb2 * 100.0);
        locals.var_uc_glkb2 = assign10440_e5545;
        locals.var_uc_glkb2_rv = 0.0;

        let assign10450_e5548: f64 = (locals.var_uc_fn2 * 100.0);
        locals.var_uc_fn2 = assign10450_e5548;
        locals.var_uc_fn2_rv = 0.0;

        let assign10460_e5551: f64 = (locals.var_uc_gidl1 / 10.0);
        locals.var_uc_gidl1 = assign10460_e5551;
        locals.var_uc_gidl1_rv = 0.0;

        let assign10470_e5554: f64 = (locals.var_uc_gidl2 * 100.0);
        locals.var_uc_gidl2 = assign10470_e5554;
        locals.var_uc_gidl2_rv = 0.0;

        let assign10480_e5557: f64 = (locals.var_uc_nfalp / 100.0);
        locals.var_uc_nfalp = assign10480_e5557;
        locals.var_uc_nfalp_rv = 0.0;

        let assign10500_e5563: f64 = (locals.var_uc_npext / 1e-6);
        locals.var_uc_npext = assign10500_e5563;
        locals.var_uc_npext_rv = 0.0;

        let assign10510_e5566: f64 = (locals.var_uc_rd22 / 100.0);
        locals.var_uc_rd22 = assign10510_e5566;
        locals.var_uc_rd22_rv = 0.0;

        let assign10520_e5569: f64 = (locals.var_uc_rd23 / 100.0);
        locals.var_uc_rd23 = assign10520_e5569;
        locals.var_uc_rd23_rv = 0.0;

        let assign10530_e5572: f64 = (locals.var_uc_rd24 / 100.0);
        locals.var_uc_rd24 = assign10530_e5572;
        locals.var_uc_rd24_rv = 0.0;

        let assign10540_e5575: f64 = (locals.var_uc_rdvd / 100.0);
        locals.var_uc_rdvd = assign10540_e5575;
        locals.var_uc_rdvd_rv = 0.0;

        let assign10550_e5578: f64 = (locals.var_uc_rth0 / 100.0);
        locals.var_uc_rth0 = assign10550_e5578;
        locals.var_uc_rth0_rv = 0.0;

        let assign10560_e5580: f64 = (-locals.var_uc_vfbover);
        locals.var_uc_vfbover = assign10560_e5580;
        locals.var_uc_vfbover_rv = 0.0;

        let assign10570_e5583: f64 = (locals.var_uc_depvmax / 100.0);
        locals.var_uc_depvmax = assign10570_e5583;
        locals.var_uc_depvmax_dn0 = (locals.var_uc_depvmax_dn0 / 100.0);
        locals.var_uc_depvmax_dn2 = (locals.var_uc_depvmax_dn2 / 100.0);
        locals.var_uc_depvmax_dn4 = (locals.var_uc_depvmax_dn4 / 100.0);
        locals.var_uc_depvmax_dn5 = (locals.var_uc_depvmax_dn5 / 100.0);
        locals.var_uc_depvmax_dn6 = (locals.var_uc_depvmax_dn6 / 100.0);
        locals.var_uc_depvmax_dn7 = (locals.var_uc_depvmax_dn7 / 100.0);
        locals.var_uc_depvmax_dn8 = (locals.var_uc_depvmax_dn8 / 100.0);
        locals.var_uc_depvmax_dn9 = (locals.var_uc_depvmax_dn9 / 100.0);
        locals.var_uc_depvmax_dn10 = (locals.var_uc_depvmax_dn10 / 100.0);
        locals.var_uc_depvmax_dn11 = (locals.var_uc_depvmax_dn11 / 100.0);
        locals.var_uc_depvmax_dn14 = (locals.var_uc_depvmax_dn14 / 100.0);
        locals.var_uc_depvmax_rv = 0.0;

        locals.var_flg_nqs = p.p28;
        locals.var_flg_nqs_rv = 0.0;

        let (assign10590_e5594,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_flg_qy = assign10590_e5594;
        locals.var_flg_qy_rv = 0.0;

        let assign10610_e5608: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard252 = assign10610_e5608;
        locals.var_guard252_rv = 0.0;

        let (assign10620_e5612,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10620_e5612;
        locals.var_flg_qmetemp_rv = 0.0;

        let (assign10630_e5617,) = {
    if (locals.var_guard252 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10630_e5617;
        locals.var_flg_qmetemp_rv = 0.0;

        let assign10640_e5620: f64 = (locals.var_wg * locals.var_lg);
        locals.var_wlg = assign10640_e5620;
        locals.var_wlg_rv = 0.0;

        let assign10650_e5623: f64 = (p.p289 * 1000000.0);
        locals.var_uc_gdld = assign10650_e5623;
        locals.var_uc_gdld_rv = 0.0;

        let assign10660_e5629: f64 = (locals.var_ktnom * 1e-7);
        let assign10660_e5630: f64 = (9.025e-5 + assign10660_e5629);
        let assign10660_e5631: f64 = (locals.var_ktnom * assign10660_e5630);
        let assign10660_e5632: f64 = (locals.var_uc_eg0 - assign10660_e5631);
        locals.var_egtnom = assign10660_e5632;
        locals.var_egtnom_rv = 0.0;

        let assign10670_e5635: f64 = (8.8541878e-12 * p.p267);
        locals.var_cecox = assign10670_e5635;
        locals.var_cecox_rv = 0.0;

        locals.var_msc = locals.var_uc_scp22;
        locals.var_msc_rv = 0.0;

        let assign10690_e5639: f64 = if locals.var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign10690_e5639;
        locals.var_guard253_rv = 0.0;

        let (assign10700_e5643,) = {
    if (locals.var_guard253 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10700_e5643;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10710_e5647,) = {
    if (locals.var_guard253 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10710_e5647;
        locals.var_cnstpgd_rv = 0.0;

        let (assign10720_e5652,) = {
    if (locals.var_guard253 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10720_e5652;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10730_e5665,) = {
    if (locals.var_guard253 == 0.0) {
        let assign10730_e5658: f64 = (1.0 / locals.var_lg);
        let assign10730_e5659: f64 = (1.0 + assign10730_e5658);
        let assign10730_e5661: f64 = (assign10730_e5659).powf(p.p153);
        let assign10730_e5663: f64 = (assign10730_e5661 * locals.var_uc_pgd1);
        (assign10730_e5663,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10730_e5665;
        locals.var_cnstpgd_rv = 0.0;

        let assign10740_e5669: f64 = (locals.var_lg).powf(p.p229);
        let assign10740_e5671: f64 = (assign10740_e5669 * p.p230);
        let assign10740_e5672: f64 = (1.0 + assign10740_e5671);
        locals.var_clmmod = assign10740_e5672;
        locals.var_clmmod_rv = 0.0;

        let assign10750_e5677: f64 = (0.5 * p.p0);
        let assign10750_e5678: f64 = (p.p118 + assign10750_e5677);
        let assign10750_e5679: f64 = (1.0 / assign10750_e5678);
        let assign10750_e5684: f64 = (0.5 * p.p0);
        let assign10750_e5685: f64 = (p.p119 + assign10750_e5684);
        let assign10750_e5686: f64 = (1.0 / assign10750_e5685);
        let assign10750_e5687: f64 = (assign10750_e5679 + assign10750_e5686);
        locals.var_t1 = assign10750_e5687;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign10760_e5690: f64 = (2.0 / locals.var_t1);
        locals.var_lod_half_ref = assign10760_e5690;
        locals.var_lod_half_ref_dn0 = (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn2 = (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn4 = (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn5 = (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn6 = (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn7 = (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn8 = (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn9 = (-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn10 = (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn11 = (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn14 = (-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_rv = 0.0;

        let assign10770_e5709: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard254 = assign10770_e5709;
        locals.var_guard254_rv = 0.0;

        let (assign10780_e5713, assign10780_e5713_d_n0, assign10780_e5713_d_n2, assign10780_e5713_d_n4, assign10780_e5713_d_n5, assign10780_e5713_d_n6, assign10780_e5713_d_n7, assign10780_e5713_d_n8, assign10780_e5713_d_n9, assign10780_e5713_d_n10, assign10780_e5713_d_n11, assign10780_e5713_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10780_e5713;
        locals.var_t1_dn0 = assign10780_e5713_d_n0;
        locals.var_t1_dn2 = assign10780_e5713_d_n2;
        locals.var_t1_dn4 = assign10780_e5713_d_n4;
        locals.var_t1_dn5 = assign10780_e5713_d_n5;
        locals.var_t1_dn6 = assign10780_e5713_d_n6;
        locals.var_t1_dn7 = assign10780_e5713_d_n7;
        locals.var_t1_dn8 = assign10780_e5713_d_n8;
        locals.var_t1_dn9 = assign10780_e5713_d_n9;
        locals.var_t1_dn10 = assign10780_e5713_d_n10;
        locals.var_t1_dn11 = assign10780_e5713_d_n11;
        locals.var_t1_dn14 = assign10780_e5713_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10790_e5717,) = {
    if (locals.var_guard254 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10790_e5717;
        locals.var_i_rv = 0.0;

        let mut assign10800_loop_guard: usize = 0;
        while {
            let assign10800_cond_e5722: f64 = if ((locals.var_guard254 != 0.0) && (locals.var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10800_cond_e5722 != 0.0
        } {
            assign10800_loop_guard += 1;
            assert!(assign10800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10800_body0_e5754, assign10800_body0_e5754_d_n0, assign10800_body0_e5754_d_n2, assign10800_body0_e5754_d_n4, assign10800_body0_e5754_d_n5, assign10800_body0_e5754_d_n6, assign10800_body0_e5754_d_n7, assign10800_body0_e5754_d_n8, assign10800_body0_e5754_d_n9, assign10800_body0_e5754_d_n10, assign10800_body0_e5754_d_n11, assign10800_body0_e5754_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10800_body0_e5729: f64 = (0.5 * p.p0);
        let assign10800_body0_e5730: f64 = (p.p8 + assign10800_body0_e5729);
        let assign10800_body0_e5734: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5735: f64 = (locals.var_i * assign10800_body0_e5734);
        let assign10800_body0_e5736: f64 = (assign10800_body0_e5730 + assign10800_body0_e5735);
        let assign10800_body0_e5737: f64 = (1.0 / assign10800_body0_e5736);
        let assign10800_body0_e5738: f64 = (locals.var_t1 + assign10800_body0_e5737);
        let assign10800_body0_e5743: f64 = (0.5 * p.p0);
        let assign10800_body0_e5744: f64 = (p.p9 + assign10800_body0_e5743);
        let assign10800_body0_e5748: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5749: f64 = (locals.var_i * assign10800_body0_e5748);
        let assign10800_body0_e5750: f64 = (assign10800_body0_e5744 + assign10800_body0_e5749);
        let assign10800_body0_e5751: f64 = (1.0 / assign10800_body0_e5750);
        let assign10800_body0_e5752: f64 = (assign10800_body0_e5738 + assign10800_body0_e5751);
        (assign10800_body0_e5752, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign10800_body0_e5754;
            locals.var_t1_dn0 = assign10800_body0_e5754_d_n0;
            locals.var_t1_dn2 = assign10800_body0_e5754_d_n2;
            locals.var_t1_dn4 = assign10800_body0_e5754_d_n4;
            locals.var_t1_dn5 = assign10800_body0_e5754_d_n5;
            locals.var_t1_dn6 = assign10800_body0_e5754_d_n6;
            locals.var_t1_dn7 = assign10800_body0_e5754_d_n7;
            locals.var_t1_dn8 = assign10800_body0_e5754_d_n8;
            locals.var_t1_dn9 = assign10800_body0_e5754_d_n9;
            locals.var_t1_dn10 = assign10800_body0_e5754_d_n10;
            locals.var_t1_dn11 = assign10800_body0_e5754_d_n11;
            locals.var_t1_dn14 = assign10800_body0_e5754_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign10800_body1_e5760,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10800_body1_e5758: f64 = (locals.var_i + 1.0);
        (assign10800_body1_e5758,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10800_body1_e5760;
            locals.var_i_rv = 0.0;
        }

        let (assign10810_e5768, assign10810_e5768_d_n0, assign10810_e5768_d_n2, assign10810_e5768_d_n4, assign10810_e5768_d_n5, assign10810_e5768_d_n6, assign10810_e5768_d_n7, assign10810_e5768_d_n8, assign10810_e5768_d_n9, assign10810_e5768_d_n10, assign10810_e5768_d_n11, assign10810_e5768_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10810_e5764: f64 = (2.0 * p.p7);
        let assign10810_e5766: f64 = (assign10810_e5764 / locals.var_t1);
        (assign10810_e5766, (-((assign10810_e5764 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10810_e5768;
        locals.var_lod_half_dn0 = assign10810_e5768_d_n0;
        locals.var_lod_half_dn2 = assign10810_e5768_d_n2;
        locals.var_lod_half_dn4 = assign10810_e5768_d_n4;
        locals.var_lod_half_dn5 = assign10810_e5768_d_n5;
        locals.var_lod_half_dn6 = assign10810_e5768_d_n6;
        locals.var_lod_half_dn7 = assign10810_e5768_d_n7;
        locals.var_lod_half_dn8 = assign10810_e5768_d_n8;
        locals.var_lod_half_dn9 = assign10810_e5768_d_n9;
        locals.var_lod_half_dn10 = assign10810_e5768_d_n10;
        locals.var_lod_half_dn11 = assign10810_e5768_d_n11;
        locals.var_lod_half_dn14 = assign10810_e5768_d_n14;
        locals.var_lod_half_rv = 0.0;

        let (assign10820_e5773, assign10820_e5773_d_n0, assign10820_e5773_d_n2, assign10820_e5773_d_n4, assign10820_e5773_d_n5, assign10820_e5773_d_n6, assign10820_e5773_d_n7, assign10820_e5773_d_n8, assign10820_e5773_d_n9, assign10820_e5773_d_n10, assign10820_e5773_d_n11, assign10820_e5773_d_n14,) = {
    if (locals.var_guard254 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10820_e5773;
        locals.var_lod_half_dn0 = assign10820_e5773_d_n0;
        locals.var_lod_half_dn2 = assign10820_e5773_d_n2;
        locals.var_lod_half_dn4 = assign10820_e5773_d_n4;
        locals.var_lod_half_dn5 = assign10820_e5773_d_n5;
        locals.var_lod_half_dn6 = assign10820_e5773_d_n6;
        locals.var_lod_half_dn7 = assign10820_e5773_d_n7;
        locals.var_lod_half_dn8 = assign10820_e5773_d_n8;
        locals.var_lod_half_dn9 = assign10820_e5773_d_n9;
        locals.var_lod_half_dn10 = assign10820_e5773_d_n10;
        locals.var_lod_half_dn11 = assign10820_e5773_d_n11;
        locals.var_lod_half_dn14 = assign10820_e5773_d_n14;
        locals.var_lod_half_rv = 0.0;

        locals.var_npexte = locals.var_uc_npext;
        locals.var_npexte_dn0 = 0.0;
        locals.var_npexte_dn2 = 0.0;
        locals.var_npexte_dn4 = 0.0;
        locals.var_npexte_dn5 = 0.0;
        locals.var_npexte_dn6 = 0.0;
        locals.var_npexte_dn7 = 0.0;
        locals.var_npexte_dn8 = 0.0;
        locals.var_npexte_dn9 = 0.0;
        locals.var_npexte_dn10 = 0.0;
        locals.var_npexte_dn11 = 0.0;
        locals.var_npexte_dn14 = 0.0;
        locals.var_npexte_rv = 0.0;

        locals.var_ef_mueph1 = locals.var_uc_mueph1;
        locals.var_ef_mueph1_dn0 = 0.0;
        locals.var_ef_mueph1_dn2 = 0.0;
        locals.var_ef_mueph1_dn4 = 0.0;
        locals.var_ef_mueph1_dn5 = 0.0;
        locals.var_ef_mueph1_dn6 = 0.0;
        locals.var_ef_mueph1_dn7 = 0.0;
        locals.var_ef_mueph1_dn8 = 0.0;
        locals.var_ef_mueph1_dn9 = 0.0;
        locals.var_ef_mueph1_dn10 = 0.0;
        locals.var_ef_mueph1_dn11 = 0.0;
        locals.var_ef_mueph1_dn14 = 0.0;
        locals.var_ef_mueph1_rv = 0.0;

        locals.var_ef_nsubp = locals.var_uc_nsubp;
        locals.var_ef_nsubp_dn0 = 0.0;
        locals.var_ef_nsubp_dn2 = 0.0;
        locals.var_ef_nsubp_dn4 = 0.0;
        locals.var_ef_nsubp_dn5 = 0.0;
        locals.var_ef_nsubp_dn6 = 0.0;
        locals.var_ef_nsubp_dn7 = 0.0;
        locals.var_ef_nsubp_dn8 = 0.0;
        locals.var_ef_nsubp_dn9 = 0.0;
        locals.var_ef_nsubp_dn10 = 0.0;
        locals.var_ef_nsubp_dn11 = 0.0;
        locals.var_ef_nsubp_dn14 = 0.0;
        locals.var_ef_nsubp_rv = 0.0;

        locals.var_ef_nsubc = locals.var_uc_nsubc;
        locals.var_ef_nsubc_dn0 = 0.0;
        locals.var_ef_nsubc_dn2 = 0.0;
        locals.var_ef_nsubc_dn4 = 0.0;
        locals.var_ef_nsubc_dn5 = 0.0;
        locals.var_ef_nsubc_dn6 = 0.0;
        locals.var_ef_nsubc_dn7 = 0.0;
        locals.var_ef_nsubc_dn8 = 0.0;
        locals.var_ef_nsubc_dn9 = 0.0;
        locals.var_ef_nsubc_dn10 = 0.0;
        locals.var_ef_nsubc_dn11 = 0.0;
        locals.var_ef_nsubc_dn14 = 0.0;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10870_e5782: f64 = if ((p.p32 == 1.0) && (locals.var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard255 = assign10870_e5782;
        locals.var_guard255_rv = 0.0;

        let (assign10890_e5803, assign10890_e5803_d_n0, assign10890_e5803_d_n2, assign10890_e5803_d_n4, assign10890_e5803_d_n5, assign10890_e5803_d_n6, assign10890_e5803_d_n7, assign10890_e5803_d_n8, assign10890_e5803_d_n9, assign10890_e5803_d_n10, assign10890_e5803_d_n11, assign10890_e5803_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10890_e5794: f64 = (locals.var_mks_nsubcdfm).ln();
        let assign10890_e5796: f64 = (locals.var_ef_nsubc).ln();
        let assign10890_e5797: f64 = (assign10890_e5794 - assign10890_e5796);
        let assign10890_e5798: f64 = (p.p282 * assign10890_e5797);
        let assign10890_e5800: f64 = (assign10890_e5798 + 1.0);
        let assign10890_e5801: f64 = (locals.var_ef_mueph1 * assign10890_e5800);
        (assign10890_e5801, ((locals.var_ef_mueph1_dn0 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn0 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn2 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn2 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn4 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn4 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn5 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn5 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn6 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn6 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn7 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn7 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn8 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn8 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn9 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn9 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn10 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn10 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn11 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn11 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn14 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn14 / locals.var_ef_nsubc))))),)
    } else {
        (locals.var_ef_mueph1, locals.var_ef_mueph1_dn0, locals.var_ef_mueph1_dn2, locals.var_ef_mueph1_dn4, locals.var_ef_mueph1_dn5, locals.var_ef_mueph1_dn6, locals.var_ef_mueph1_dn7, locals.var_ef_mueph1_dn8, locals.var_ef_mueph1_dn9, locals.var_ef_mueph1_dn10, locals.var_ef_mueph1_dn11, locals.var_ef_mueph1_dn14,)
    }
};
        locals.var_ef_mueph1 = assign10890_e5803;
        locals.var_ef_mueph1_dn0 = assign10890_e5803_d_n0;
        locals.var_ef_mueph1_dn2 = assign10890_e5803_d_n2;
        locals.var_ef_mueph1_dn4 = assign10890_e5803_d_n4;
        locals.var_ef_mueph1_dn5 = assign10890_e5803_d_n5;
        locals.var_ef_mueph1_dn6 = assign10890_e5803_d_n6;
        locals.var_ef_mueph1_dn7 = assign10890_e5803_d_n7;
        locals.var_ef_mueph1_dn8 = assign10890_e5803_d_n8;
        locals.var_ef_mueph1_dn9 = assign10890_e5803_d_n9;
        locals.var_ef_mueph1_dn10 = assign10890_e5803_d_n10;
        locals.var_ef_mueph1_dn11 = assign10890_e5803_d_n11;
        locals.var_ef_mueph1_dn14 = assign10890_e5803_d_n14;
        locals.var_ef_mueph1_rv = 0.0;

        let (assign10900_e5811, assign10900_e5811_d_n0, assign10900_e5811_d_n2, assign10900_e5811_d_n4, assign10900_e5811_d_n5, assign10900_e5811_d_n6, assign10900_e5811_d_n7, assign10900_e5811_d_n8, assign10900_e5811_d_n9, assign10900_e5811_d_n10, assign10900_e5811_d_n11, assign10900_e5811_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10900_e5807: f64 = (locals.var_ef_nsubp + locals.var_mks_nsubcdfm);
        let assign10900_e5809: f64 = (assign10900_e5807 - locals.var_ef_nsubc);
        (assign10900_e5809, (locals.var_ef_nsubp_dn0 - locals.var_ef_nsubc_dn0), (locals.var_ef_nsubp_dn2 - locals.var_ef_nsubc_dn2), (locals.var_ef_nsubp_dn4 - locals.var_ef_nsubc_dn4), (locals.var_ef_nsubp_dn5 - locals.var_ef_nsubc_dn5), (locals.var_ef_nsubp_dn6 - locals.var_ef_nsubc_dn6), (locals.var_ef_nsubp_dn7 - locals.var_ef_nsubc_dn7), (locals.var_ef_nsubp_dn8 - locals.var_ef_nsubc_dn8), (locals.var_ef_nsubp_dn9 - locals.var_ef_nsubc_dn9), (locals.var_ef_nsubp_dn10 - locals.var_ef_nsubc_dn10), (locals.var_ef_nsubp_dn11 - locals.var_ef_nsubc_dn11), (locals.var_ef_nsubp_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_ef_nsubp, locals.var_ef_nsubp_dn0, locals.var_ef_nsubp_dn2, locals.var_ef_nsubp_dn4, locals.var_ef_nsubp_dn5, locals.var_ef_nsubp_dn6, locals.var_ef_nsubp_dn7, locals.var_ef_nsubp_dn8, locals.var_ef_nsubp_dn9, locals.var_ef_nsubp_dn10, locals.var_ef_nsubp_dn11, locals.var_ef_nsubp_dn14,)
    }
};
        locals.var_ef_nsubp = assign10900_e5811;
        locals.var_ef_nsubp_dn0 = assign10900_e5811_d_n0;
        locals.var_ef_nsubp_dn2 = assign10900_e5811_d_n2;
        locals.var_ef_nsubp_dn4 = assign10900_e5811_d_n4;
        locals.var_ef_nsubp_dn5 = assign10900_e5811_d_n5;
        locals.var_ef_nsubp_dn6 = assign10900_e5811_d_n6;
        locals.var_ef_nsubp_dn7 = assign10900_e5811_d_n7;
        locals.var_ef_nsubp_dn8 = assign10900_e5811_d_n8;
        locals.var_ef_nsubp_dn9 = assign10900_e5811_d_n9;
        locals.var_ef_nsubp_dn10 = assign10900_e5811_d_n10;
        locals.var_ef_nsubp_dn11 = assign10900_e5811_d_n11;
        locals.var_ef_nsubp_dn14 = assign10900_e5811_d_n14;
        locals.var_ef_nsubp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10910_e5819, assign10910_e5819_d_n0, assign10910_e5819_d_n2, assign10910_e5819_d_n4, assign10910_e5819_d_n5, assign10910_e5819_d_n6, assign10910_e5819_d_n7, assign10910_e5819_d_n8, assign10910_e5819_d_n9, assign10910_e5819_d_n10, assign10910_e5819_d_n11, assign10910_e5819_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10910_e5815: f64 = (locals.var_npexte + locals.var_mks_nsubcdfm);
        let assign10910_e5817: f64 = (assign10910_e5815 - locals.var_ef_nsubc);
        (assign10910_e5817, (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0), (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2), (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4), (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5), (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6), (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7), (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8), (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9), (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10), (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11), (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_npexte, locals.var_npexte_dn0, locals.var_npexte_dn2, locals.var_npexte_dn4, locals.var_npexte_dn5, locals.var_npexte_dn6, locals.var_npexte_dn7, locals.var_npexte_dn8, locals.var_npexte_dn9, locals.var_npexte_dn10, locals.var_npexte_dn11, locals.var_npexte_dn14,)
    }
};
        locals.var_npexte = assign10910_e5819;
        locals.var_npexte_dn0 = assign10910_e5819_d_n0;
        locals.var_npexte_dn2 = assign10910_e5819_d_n2;
        locals.var_npexte_dn4 = assign10910_e5819_d_n4;
        locals.var_npexte_dn5 = assign10910_e5819_d_n5;
        locals.var_npexte_dn6 = assign10910_e5819_d_n6;
        locals.var_npexte_dn7 = assign10910_e5819_d_n7;
        locals.var_npexte_dn8 = assign10910_e5819_d_n8;
        locals.var_npexte_dn9 = assign10910_e5819_d_n9;
        locals.var_npexte_dn10 = assign10910_e5819_d_n10;
        locals.var_npexte_dn11 = assign10910_e5819_d_n11;
        locals.var_npexte_dn14 = assign10910_e5819_d_n14;
        locals.var_npexte_rv = 0.0;

        let (assign10920_e5823, assign10920_e5823_d_n0, assign10920_e5823_d_n2, assign10920_e5823_d_n4, assign10920_e5823_d_n5, assign10920_e5823_d_n6, assign10920_e5823_d_n7, assign10920_e5823_d_n8, assign10920_e5823_d_n9, assign10920_e5823_d_n10, assign10920_e5823_d_n11, assign10920_e5823_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        (locals.var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ef_nsubc, locals.var_ef_nsubc_dn0, locals.var_ef_nsubc_dn2, locals.var_ef_nsubc_dn4, locals.var_ef_nsubc_dn5, locals.var_ef_nsubc_dn6, locals.var_ef_nsubc_dn7, locals.var_ef_nsubc_dn8, locals.var_ef_nsubc_dn9, locals.var_ef_nsubc_dn10, locals.var_ef_nsubc_dn11, locals.var_ef_nsubc_dn14,)
    }
};
        locals.var_ef_nsubc = assign10920_e5823;
        locals.var_ef_nsubc_dn0 = assign10920_e5823_d_n0;
        locals.var_ef_nsubc_dn2 = assign10920_e5823_d_n2;
        locals.var_ef_nsubc_dn4 = assign10920_e5823_d_n4;
        locals.var_ef_nsubc_dn5 = assign10920_e5823_d_n5;
        locals.var_ef_nsubc_dn6 = assign10920_e5823_d_n6;
        locals.var_ef_nsubc_dn7 = assign10920_e5823_d_n7;
        locals.var_ef_nsubc_dn8 = assign10920_e5823_d_n8;
        locals.var_ef_nsubc_dn9 = assign10920_e5823_d_n9;
        locals.var_ef_nsubc_dn10 = assign10920_e5823_d_n10;
        locals.var_ef_nsubc_dn11 = assign10920_e5823_d_n11;
        locals.var_ef_nsubc_dn14 = assign10920_e5823_d_n14;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10930_e5829: f64 = (locals.var_wg).powf(p.p163);
        let assign10930_e5830: f64 = (p.p162 / assign10930_e5829);
        let assign10930_e5831: f64 = (1.0 + assign10930_e5830);
        let assign10930_e5832: f64 = (locals.var_ef_mueph1 * assign10930_e5831);
        let assign10930_e5837: f64 = (locals.var_lg).powf(p.p165);
        let assign10930_e5838: f64 = (p.p164 / assign10930_e5837);
        let assign10930_e5839: f64 = (1.0 + assign10930_e5838);
        let assign10930_e5840: f64 = (assign10930_e5832 * assign10930_e5839);
        let assign10930_e5845: f64 = (locals.var_wlg).powf(p.p168);
        let assign10930_e5846: f64 = (p.p167 / assign10930_e5845);
        let assign10930_e5847: f64 = (1.0 + assign10930_e5846);
        let assign10930_e5848: f64 = (assign10930_e5840 * assign10930_e5847);
        locals.var_mueph = assign10930_e5848;
        locals.var_mueph_dn0 = (((locals.var_ef_mueph1_dn0 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn2 = (((locals.var_ef_mueph1_dn2 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn4 = (((locals.var_ef_mueph1_dn4 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn5 = (((locals.var_ef_mueph1_dn5 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn6 = (((locals.var_ef_mueph1_dn6 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn7 = (((locals.var_ef_mueph1_dn7 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn8 = (((locals.var_ef_mueph1_dn8 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn9 = (((locals.var_ef_mueph1_dn9 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn10 = (((locals.var_ef_mueph1_dn10 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn11 = (((locals.var_ef_mueph1_dn11 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn14 = (((locals.var_ef_mueph1_dn14 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_rv = 0.0;

        let assign10940_e5851: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign10940_e5851;
        locals.var_guard257_rv = 0.0;

        let (assign10950_e5859, assign10950_e5859_d_n0, assign10950_e5859_d_n2, assign10950_e5859_d_n4, assign10950_e5859_d_n5, assign10950_e5859_d_n6, assign10950_e5859_d_n7, assign10950_e5859_d_n8, assign10950_e5859_d_n9, assign10950_e5859_d_n10, assign10950_e5859_d_n11, assign10950_e5859_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10950_e5856: f64 = (1.0 + locals.var_uc_muesti2);
        let assign10950_e5857: f64 = (1.0 / assign10950_e5856);
        (assign10950_e5857, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10950_e5859;
        locals.var_t1_dn0 = assign10950_e5859_d_n0;
        locals.var_t1_dn2 = assign10950_e5859_d_n2;
        locals.var_t1_dn4 = assign10950_e5859_d_n4;
        locals.var_t1_dn5 = assign10950_e5859_d_n5;
        locals.var_t1_dn6 = assign10950_e5859_d_n6;
        locals.var_t1_dn7 = assign10950_e5859_d_n7;
        locals.var_t1_dn8 = assign10950_e5859_d_n8;
        locals.var_t1_dn9 = assign10950_e5859_d_n9;
        locals.var_t1_dn10 = assign10950_e5859_d_n10;
        locals.var_t1_dn11 = assign10950_e5859_d_n11;
        locals.var_t1_dn14 = assign10950_e5859_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10960_e5867, assign10960_e5867_d_n0, assign10960_e5867_d_n2, assign10960_e5867_d_n4, assign10960_e5867_d_n5, assign10960_e5867_d_n6, assign10960_e5867_d_n7, assign10960_e5867_d_n8, assign10960_e5867_d_n9, assign10960_e5867_d_n10, assign10960_e5867_d_n11, assign10960_e5867_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10960_e5863: f64 = (locals.var_uc_muesti1 / locals.var_lod_half);
        let assign10960_e5865: f64 = (assign10960_e5863).powf(locals.var_uc_muesti3);
        (assign10960_e5865, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign10960_e5867;
        locals.var_t2_dn0 = assign10960_e5867_d_n0;
        locals.var_t2_dn2 = assign10960_e5867_d_n2;
        locals.var_t2_dn4 = assign10960_e5867_d_n4;
        locals.var_t2_dn5 = assign10960_e5867_d_n5;
        locals.var_t2_dn6 = assign10960_e5867_d_n6;
        locals.var_t2_dn7 = assign10960_e5867_d_n7;
        locals.var_t2_dn8 = assign10960_e5867_d_n8;
        locals.var_t2_dn9 = assign10960_e5867_d_n9;
        locals.var_t2_dn10 = assign10960_e5867_d_n10;
        locals.var_t2_dn11 = assign10960_e5867_d_n11;
        locals.var_t2_dn14 = assign10960_e5867_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign10970_e5875, assign10970_e5875_d_n0, assign10970_e5875_d_n2, assign10970_e5875_d_n4, assign10970_e5875_d_n5, assign10970_e5875_d_n6, assign10970_e5875_d_n7, assign10970_e5875_d_n8, assign10970_e5875_d_n9, assign10970_e5875_d_n10, assign10970_e5875_d_n11, assign10970_e5875_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10970_e5871: f64 = (locals.var_uc_muesti1 / locals.var_lod_half_ref);
        let assign10970_e5873: f64 = (assign10970_e5871).powf(locals.var_uc_muesti3);
        (assign10970_e5873, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign10970_e5875;
        locals.var_t3_dn0 = assign10970_e5875_d_n0;
        locals.var_t3_dn2 = assign10970_e5875_d_n2;
        locals.var_t3_dn4 = assign10970_e5875_d_n4;
        locals.var_t3_dn5 = assign10970_e5875_d_n5;
        locals.var_t3_dn6 = assign10970_e5875_d_n6;
        locals.var_t3_dn7 = assign10970_e5875_d_n7;
        locals.var_t3_dn8 = assign10970_e5875_d_n8;
        locals.var_t3_dn9 = assign10970_e5875_d_n9;
        locals.var_t3_dn10 = assign10970_e5875_d_n10;
        locals.var_t3_dn11 = assign10970_e5875_d_n11;
        locals.var_t3_dn14 = assign10970_e5875_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign10980_e5891, assign10980_e5891_d_n0, assign10980_e5891_d_n2, assign10980_e5891_d_n4, assign10980_e5891_d_n5, assign10980_e5891_d_n6, assign10980_e5891_d_n7, assign10980_e5891_d_n8, assign10980_e5891_d_n9, assign10980_e5891_d_n10, assign10980_e5891_d_n11, assign10980_e5891_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10980_e5881: f64 = (locals.var_t1 * locals.var_t2);
        let assign10980_e5882: f64 = (1.0 + assign10980_e5881);
        let assign10980_e5883: f64 = (locals.var_mueph * assign10980_e5882);
        let assign10980_e5887: f64 = (locals.var_t1 * locals.var_t3);
        let assign10980_e5888: f64 = (1.0 + assign10980_e5887);
        let assign10980_e5889: f64 = (assign10980_e5883 / assign10980_e5888);
        (assign10980_e5889, (((((locals.var_mueph_dn0 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn2 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn4 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn5 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn6 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn7 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn8 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn9 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn10 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn11 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn14 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign10980_e5888 * assign10980_e5888)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign10980_e5891;
        locals.var_mueph_dn0 = assign10980_e5891_d_n0;
        locals.var_mueph_dn2 = assign10980_e5891_d_n2;
        locals.var_mueph_dn4 = assign10980_e5891_d_n4;
        locals.var_mueph_dn5 = assign10980_e5891_d_n5;
        locals.var_mueph_dn6 = assign10980_e5891_d_n6;
        locals.var_mueph_dn7 = assign10980_e5891_d_n7;
        locals.var_mueph_dn8 = assign10980_e5891_d_n8;
        locals.var_mueph_dn9 = assign10980_e5891_d_n9;
        locals.var_mueph_dn10 = assign10980_e5891_d_n10;
        locals.var_mueph_dn11 = assign10980_e5891_d_n11;
        locals.var_mueph_dn14 = assign10980_e5891_d_n14;
        locals.var_mueph_rv = 0.0;

        let assign10990_e5897: f64 = (locals.var_lg).powf(p.p176);
        let assign10990_e5898: f64 = (p.p173 / assign10990_e5897);
        let assign10990_e5899: f64 = (1.0 + assign10990_e5898);
        let assign10990_e5900: f64 = (p.p171 * assign10990_e5899);
        let assign10990_e5905: f64 = (locals.var_wg).powf(p.p175);
        let assign10990_e5906: f64 = (p.p174 / assign10990_e5905);
        let assign10990_e5907: f64 = (1.0 + assign10990_e5906);
        let assign10990_e5908: f64 = (assign10990_e5900 * assign10990_e5907);
        locals.var_muesr = assign10990_e5908;
        locals.var_muesr_rv = 0.0;

        let (assign11020_e5932, assign11020_e5932_d_n0, assign11020_e5932_d_n2, assign11020_e5932_d_n4, assign11020_e5932_d_n5, assign11020_e5932_d_n6, assign11020_e5932_d_n7, assign11020_e5932_d_n8, assign11020_e5932_d_n9, assign11020_e5932_d_n10, assign11020_e5932_d_n11, assign11020_e5932_d_n14,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign11020_e5932;
        locals.var_mueph_dn0 = assign11020_e5932_d_n0;
        locals.var_mueph_dn2 = assign11020_e5932_d_n2;
        locals.var_mueph_dn4 = assign11020_e5932_d_n4;
        locals.var_mueph_dn5 = assign11020_e5932_d_n5;
        locals.var_mueph_dn6 = assign11020_e5932_d_n6;
        locals.var_mueph_dn7 = assign11020_e5932_d_n7;
        locals.var_mueph_dn8 = assign11020_e5932_d_n8;
        locals.var_mueph_dn9 = assign11020_e5932_d_n9;
        locals.var_mueph_dn10 = assign11020_e5932_d_n10;
        locals.var_mueph_dn11 = assign11020_e5932_d_n11;
        locals.var_mueph_dn14 = assign11020_e5932_d_n14;
        locals.var_mueph_rv = 0.0;

        let (assign11030_e5938,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11030_e5938;
        locals.var_muesr_rv = 0.0;

        let assign11040_e5941: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11040_e5941;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11050_e5944: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11050_e5947: f64 = (locals.var_t1 + p.p155);
        let assign11050_e5948: f64 = (assign11050_e5944 / assign11050_e5947);
        let assign11050_e5950: f64 = (assign11050_e5948 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11050_e5950;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn0)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn2)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn4)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn5)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn6)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn7)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn8)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn9)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn10)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn11 = (((((locals.var_uc_ndep * locals.var_t1_dn11) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn11)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn14 = (((((locals.var_uc_ndep * locals.var_t1_dn14) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn14)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_rv = 0.0;

        let assign11060_e5953: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11060_e5953;
        locals.var_ninv_o_esi_rv = 0.0;

        let assign11070_e5959: f64 = (locals.var_lg).powf(p.p321);
        let assign11070_e5960: f64 = (p.p320 / assign11070_e5959);
        let assign11070_e5961: f64 = (1.0 + assign11070_e5960);
        let assign11070_e5962: f64 = (p.p319 * assign11070_e5961);
        let assign11070_e5967: f64 = (locals.var_wg).powf(p.p323);
        let assign11070_e5968: f64 = (p.p322 / assign11070_e5967);
        let assign11070_e5969: f64 = (1.0 + assign11070_e5968);
        let assign11070_e5970: f64 = (assign11070_e5962 * assign11070_e5969);
        locals.var_ninvd0 = assign11070_e5970;
        locals.var_ninvd0_rv = 0.0;

        let assign11080_e5975: f64 = (locals.var_lg).powf(p.p387);
        let assign11080_e5976: f64 = (p.p386 / assign11080_e5975);
        let assign11080_e5977: f64 = (1.0 + assign11080_e5976);
        let assign11080_e5982: f64 = (locals.var_wg).powf(p.p389);
        let assign11080_e5983: f64 = (p.p388 / assign11080_e5982);
        let assign11080_e5984: f64 = (1.0 + assign11080_e5983);
        let assign11080_e5985: f64 = (assign11080_e5977 * assign11080_e5984);
        locals.var_t1 = assign11080_e5985;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11090_e5988: f64 = (p.p384 * locals.var_t1);
        locals.var_ninvd0cres = assign11090_e5988;
        locals.var_ninvd0cres_dn0 = (p.p384 * locals.var_t1_dn0);
        locals.var_ninvd0cres_dn2 = (p.p384 * locals.var_t1_dn2);
        locals.var_ninvd0cres_dn4 = (p.p384 * locals.var_t1_dn4);
        locals.var_ninvd0cres_dn5 = (p.p384 * locals.var_t1_dn5);
        locals.var_ninvd0cres_dn6 = (p.p384 * locals.var_t1_dn6);
        locals.var_ninvd0cres_dn7 = (p.p384 * locals.var_t1_dn7);
        locals.var_ninvd0cres_dn8 = (p.p384 * locals.var_t1_dn8);
        locals.var_ninvd0cres_dn9 = (p.p384 * locals.var_t1_dn9);
        locals.var_ninvd0cres_dn10 = (p.p384 * locals.var_t1_dn10);
        locals.var_ninvd0cres_dn11 = (p.p384 * locals.var_t1_dn11);
        locals.var_ninvd0cres_dn14 = (p.p384 * locals.var_t1_dn14);
        locals.var_ninvd0cres_rv = 0.0;

        let assign11100_e5991: f64 = (p.p385 * locals.var_t1);
        locals.var_ninvd0hres = assign11100_e5991;
        locals.var_ninvd0hres_dn0 = (p.p385 * locals.var_t1_dn0);
        locals.var_ninvd0hres_dn2 = (p.p385 * locals.var_t1_dn2);
        locals.var_ninvd0hres_dn4 = (p.p385 * locals.var_t1_dn4);
        locals.var_ninvd0hres_dn5 = (p.p385 * locals.var_t1_dn5);
        locals.var_ninvd0hres_dn6 = (p.p385 * locals.var_t1_dn6);
        locals.var_ninvd0hres_dn7 = (p.p385 * locals.var_t1_dn7);
        locals.var_ninvd0hres_dn8 = (p.p385 * locals.var_t1_dn8);
        locals.var_ninvd0hres_dn9 = (p.p385 * locals.var_t1_dn9);
        locals.var_ninvd0hres_dn10 = (p.p385 * locals.var_t1_dn10);
        locals.var_ninvd0hres_dn11 = (p.p385 * locals.var_t1_dn11);
        locals.var_ninvd0hres_dn14 = (p.p385 * locals.var_t1_dn14);
        locals.var_ninvd0hres_rv = 0.0;

        let assign11110_e5996: f64 = (locals.var_lgate + p.p121);
        let assign11110_e5998: f64 = (assign11110_e5996).powf(p.p122);
        let assign11110_e5999: f64 = (locals.var_mks_ll / assign11110_e5998);
        let assign11110_e6000: f64 = (p.p97 + assign11110_e5999);
        locals.var_dl = assign11110_e6000;
        locals.var_dl_rv = 0.0;

        let assign11120_e6005: f64 = (locals.var_lgate + p.p121);
        let assign11120_e6007: f64 = (assign11120_e6005).powf(p.p122);
        let assign11120_e6008: f64 = (locals.var_mks_ll / assign11120_e6007);
        let assign11120_e6009: f64 = (locals.var_uc_xldld + assign11120_e6008);
        locals.var_dlld = assign11120_e6009;
        locals.var_dlld_rv = 0.0;

        let assign11130_e6014: f64 = (locals.var_wgate + p.p128);
        let assign11130_e6016: f64 = (assign11130_e6014).powf(p.p129);
        let assign11130_e6017: f64 = (locals.var_mks_wl / assign11130_e6016);
        let assign11130_e6018: f64 = (p.p114 + assign11130_e6017);
        locals.var_dw = assign11130_e6018;
        locals.var_dw_rv = 0.0;

        let assign11140_e6023: f64 = (locals.var_wgate + p.p128);
        let assign11140_e6025: f64 = (assign11140_e6023).powf(p.p129);
        let assign11140_e6026: f64 = (locals.var_mks_wl / assign11140_e6025);
        let assign11140_e6027: f64 = (p.p295 + assign11140_e6026);
        locals.var_dwld = assign11140_e6027;
        locals.var_dwld_rv = 0.0;

        let assign11150_e6032: f64 = (locals.var_wgate + p.p128);
        let assign11150_e6034: f64 = (assign11150_e6032).powf(p.p129);
        let assign11150_e6035: f64 = (locals.var_mks_wl / assign11150_e6034);
        let assign11150_e6036: f64 = (p.p115 + assign11150_e6035);
        locals.var_dwcv = assign11150_e6036;
        locals.var_dwcv_rv = 0.0;

        let assign11160_e6040: f64 = (locals.var_dl + locals.var_dlld);
        let assign11160_e6041: f64 = (locals.var_lgate - assign11160_e6040);
        locals.var_leff = assign11160_e6041;
        locals.var_leff_rv = 0.0;

        let assign11190_e6053: f64 = (locals.var_wlg).powf(p.p125);
        let assign11190_e6054: f64 = (p.p124 / assign11190_e6053);
        let assign11190_e6055: f64 = (locals.var_lgate + assign11190_e6054);
        locals.var_lgatesm = assign11190_e6055;
        locals.var_lgatesm_rv = 0.0;

        let assign11200_e6059: f64 = (locals.var_wlg).powf(p.p127);
        let assign11200_e6060: f64 = (locals.var_uc_wl2 / assign11200_e6059);
        locals.var_dvthsm = assign11200_e6060;
        locals.var_dvthsm_rv = 0.0;

        let assign11210_e6065: f64 = (locals.var_lgatesm * 1000000.0);
        let assign11210_e6067: f64 = (assign11210_e6065).powf(p.p207);
        let assign11210_e6068: f64 = (p.p206 / assign11210_e6067);
        let assign11210_e6069: f64 = (1.0 + assign11210_e6068);
        locals.var_t1 = assign11210_e6069;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11220_e6074: f64 = (locals.var_wg).powf(p.p209);
        let assign11220_e6075: f64 = (p.p208 / assign11220_e6074);
        let assign11220_e6076: f64 = (1.0 + assign11220_e6075);
        locals.var_t2 = assign11220_e6076;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign11230_e6079: f64 = (locals.var_uc_wsti * locals.var_t1);
        let assign11230_e6081: f64 = (assign11230_e6079 * locals.var_t2);
        locals.var_uc_wsti = assign11230_e6081;
        locals.var_uc_wsti_dn0 = ((((locals.var_uc_wsti_dn0 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn0)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn0));
        locals.var_uc_wsti_dn2 = ((((locals.var_uc_wsti_dn2 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn2)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn2));
        locals.var_uc_wsti_dn4 = ((((locals.var_uc_wsti_dn4 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn4)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn4));
        locals.var_uc_wsti_dn5 = ((((locals.var_uc_wsti_dn5 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn5)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn5));
        locals.var_uc_wsti_dn6 = ((((locals.var_uc_wsti_dn6 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn6)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn6));
        locals.var_uc_wsti_dn7 = ((((locals.var_uc_wsti_dn7 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn7)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn7));
        locals.var_uc_wsti_dn8 = ((((locals.var_uc_wsti_dn8 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn8)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn8));
        locals.var_uc_wsti_dn9 = ((((locals.var_uc_wsti_dn9 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn9)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn9));
        locals.var_uc_wsti_dn10 = ((((locals.var_uc_wsti_dn10 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn10)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn10));
        locals.var_uc_wsti_dn11 = ((((locals.var_uc_wsti_dn11 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn11)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn11));
        locals.var_uc_wsti_dn14 = ((((locals.var_uc_wsti_dn14 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn14)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn14));
        locals.var_uc_wsti_rv = 0.0;

        let assign11240_e6085: f64 = (2.0 * locals.var_dw);
        let assign11240_e6086: f64 = (locals.var_wgate - assign11240_e6085);
        locals.var_weff = assign11240_e6086;
        locals.var_weff_rv = 0.0;

        let assign11250_e6090: f64 = (2.0 * locals.var_dwld);
        let assign11250_e6091: f64 = (locals.var_wgate - assign11250_e6090);
        locals.var_weff_ld = assign11250_e6091;
        locals.var_weff_ld_rv = 0.0;

        let assign11260_e6095: f64 = (2.0 * locals.var_dwcv);
        let assign11260_e6096: f64 = (locals.var_wgate - assign11260_e6095);
        locals.var_weff_cv = assign11260_e6096;
        locals.var_weff_cv_rv = 0.0;

        let assign11330_e6120: f64 = (locals.var_weff * p.p7);
        locals.var_weff_nf = assign11330_e6120;
        locals.var_weff_nf_rv = 0.0;

        let assign11340_e6123: f64 = (locals.var_weff_cv * p.p7);
        locals.var_weffcv_nf = assign11340_e6123;
        locals.var_weffcv_nf_rv = 0.0;

        let assign11350_e6129: f64 = (locals.var_wg).powf(p.p143);
        let assign11350_e6130: f64 = (p.p142 / assign11350_e6129);
        let assign11350_e6131: f64 = (1.0 + assign11350_e6130);
        let assign11350_e6132: f64 = (locals.var_ef_nsubp * assign11350_e6131);
        locals.var_nsubpp = assign11350_e6132;
        locals.var_nsubpp_dn0 = (locals.var_ef_nsubp_dn0 * assign11350_e6131);
        locals.var_nsubpp_dn2 = (locals.var_ef_nsubp_dn2 * assign11350_e6131);
        locals.var_nsubpp_dn4 = (locals.var_ef_nsubp_dn4 * assign11350_e6131);
        locals.var_nsubpp_dn5 = (locals.var_ef_nsubp_dn5 * assign11350_e6131);
        locals.var_nsubpp_dn6 = (locals.var_ef_nsubp_dn6 * assign11350_e6131);
        locals.var_nsubpp_dn7 = (locals.var_ef_nsubp_dn7 * assign11350_e6131);
        locals.var_nsubpp_dn8 = (locals.var_ef_nsubp_dn8 * assign11350_e6131);
        locals.var_nsubpp_dn9 = (locals.var_ef_nsubp_dn9 * assign11350_e6131);
        locals.var_nsubpp_dn10 = (locals.var_ef_nsubp_dn10 * assign11350_e6131);
        locals.var_nsubpp_dn11 = (locals.var_ef_nsubp_dn11 * assign11350_e6131);
        locals.var_nsubpp_dn14 = (locals.var_ef_nsubp_dn14 * assign11350_e6131);
        locals.var_nsubpp_rv = 0.0;

        let assign11360_e6138: f64 = (locals.var_wg).powf(p.p234);
        let assign11360_e6139: f64 = (p.p233 / assign11360_e6138);
        let assign11360_e6140: f64 = (1.0 + assign11360_e6139);
        let assign11360_e6141: f64 = (locals.var_ef_nsubc * assign11360_e6140);
        locals.var_ef_nsubc = assign11360_e6141;
        locals.var_ef_nsubc_dn0 = (locals.var_ef_nsubc_dn0 * assign11360_e6140);
        locals.var_ef_nsubc_dn2 = (locals.var_ef_nsubc_dn2 * assign11360_e6140);
        locals.var_ef_nsubc_dn4 = (locals.var_ef_nsubc_dn4 * assign11360_e6140);
        locals.var_ef_nsubc_dn5 = (locals.var_ef_nsubc_dn5 * assign11360_e6140);
        locals.var_ef_nsubc_dn6 = (locals.var_ef_nsubc_dn6 * assign11360_e6140);
        locals.var_ef_nsubc_dn7 = (locals.var_ef_nsubc_dn7 * assign11360_e6140);
        locals.var_ef_nsubc_dn8 = (locals.var_ef_nsubc_dn8 * assign11360_e6140);
        locals.var_ef_nsubc_dn9 = (locals.var_ef_nsubc_dn9 * assign11360_e6140);
        locals.var_ef_nsubc_dn10 = (locals.var_ef_nsubc_dn10 * assign11360_e6140);
        locals.var_ef_nsubc_dn11 = (locals.var_ef_nsubc_dn11 * assign11360_e6140);
        locals.var_ef_nsubc_dn14 = (locals.var_ef_nsubc_dn14 * assign11360_e6140);
        locals.var_ef_nsubc_rv = 0.0;

        let assign11370_e6144: f64 = (locals.var_ef_nsubc * 1e-6);
        locals.var_t1 = assign11370_e6144;
        locals.var_t1_dn0 = (locals.var_ef_nsubc_dn0 * 1e-6);
        locals.var_t1_dn2 = (locals.var_ef_nsubc_dn2 * 1e-6);
        locals.var_t1_dn4 = (locals.var_ef_nsubc_dn4 * 1e-6);
        locals.var_t1_dn5 = (locals.var_ef_nsubc_dn5 * 1e-6);
        locals.var_t1_dn6 = (locals.var_ef_nsubc_dn6 * 1e-6);
        locals.var_t1_dn7 = (locals.var_ef_nsubc_dn7 * 1e-6);
        locals.var_t1_dn8 = (locals.var_ef_nsubc_dn8 * 1e-6);
        locals.var_t1_dn9 = (locals.var_ef_nsubc_dn9 * 1e-6);
        locals.var_t1_dn10 = (locals.var_ef_nsubc_dn10 * 1e-6);
        locals.var_t1_dn11 = (locals.var_ef_nsubc_dn11 * 1e-6);
        locals.var_t1_dn14 = (locals.var_ef_nsubc_dn14 * 1e-6);
        locals.var_t1_rv = 0.0;

        let assign11380_e6147: f64 = (locals.var_nsubpp * 1e-6);
        locals.var_t2 = assign11380_e6147;
        locals.var_t2_dn0 = (locals.var_nsubpp_dn0 * 1e-6);
        locals.var_t2_dn2 = (locals.var_nsubpp_dn2 * 1e-6);
        locals.var_t2_dn4 = (locals.var_nsubpp_dn4 * 1e-6);
        locals.var_t2_dn5 = (locals.var_nsubpp_dn5 * 1e-6);
        locals.var_t2_dn6 = (locals.var_nsubpp_dn6 * 1e-6);
        locals.var_t2_dn7 = (locals.var_nsubpp_dn7 * 1e-6);
        locals.var_t2_dn8 = (locals.var_nsubpp_dn8 * 1e-6);
        locals.var_t2_dn9 = (locals.var_nsubpp_dn9 * 1e-6);
        locals.var_t2_dn10 = (locals.var_nsubpp_dn10 * 1e-6);
        locals.var_t2_dn11 = (locals.var_nsubpp_dn11 * 1e-6);
        locals.var_t2_dn14 = (locals.var_nsubpp_dn14 * 1e-6);
        locals.var_t2_rv = 0.0;

        let assign11400_e6155: f64 = if locals.var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign11400_e6155;
        locals.var_guard265_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11410_e6159, assign11410_e6159_d_n0, assign11410_e6159_d_n2, assign11410_e6159_d_n4, assign11410_e6159_d_n5, assign11410_e6159_d_n6, assign11410_e6159_d_n7, assign11410_e6159_d_n8, assign11410_e6159_d_n9, assign11410_e6159_d_n10, assign11410_e6159_d_n11, assign11410_e6159_d_n14,) = {
    if (locals.var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11410_e6159;
        locals.var_t1_dn0 = assign11410_e6159_d_n0;
        locals.var_t1_dn2 = assign11410_e6159_d_n2;
        locals.var_t1_dn4 = assign11410_e6159_d_n4;
        locals.var_t1_dn5 = assign11410_e6159_d_n5;
        locals.var_t1_dn6 = assign11410_e6159_d_n6;
        locals.var_t1_dn7 = assign11410_e6159_d_n7;
        locals.var_t1_dn8 = assign11410_e6159_d_n8;
        locals.var_t1_dn9 = assign11410_e6159_d_n9;
        locals.var_t1_dn10 = assign11410_e6159_d_n10;
        locals.var_t1_dn11 = assign11410_e6159_d_n11;
        locals.var_t1_dn14 = assign11410_e6159_d_n14;
        locals.var_t1_rv = 0.0;

        let assign11420_e6162: f64 = (locals.var_t1 / 1e-6);
        locals.var_ef_nsubc = assign11420_e6162;
        locals.var_ef_nsubc_dn0 = (locals.var_t1_dn0 / 1e-6);
        locals.var_ef_nsubc_dn2 = (locals.var_t1_dn2 / 1e-6);
        locals.var_ef_nsubc_dn4 = (locals.var_t1_dn4 / 1e-6);
        locals.var_ef_nsubc_dn5 = (locals.var_t1_dn5 / 1e-6);
        locals.var_ef_nsubc_dn6 = (locals.var_t1_dn6 / 1e-6);
        locals.var_ef_nsubc_dn7 = (locals.var_t1_dn7 / 1e-6);
        locals.var_ef_nsubc_dn8 = (locals.var_t1_dn8 / 1e-6);
        locals.var_ef_nsubc_dn9 = (locals.var_t1_dn9 / 1e-6);
        locals.var_ef_nsubc_dn10 = (locals.var_t1_dn10 / 1e-6);
        locals.var_ef_nsubc_dn11 = (locals.var_t1_dn11 / 1e-6);
        locals.var_ef_nsubc_dn14 = (locals.var_t1_dn14 / 1e-6);
        locals.var_ef_nsubc_rv = 0.0;

        let assign11440_e6170: f64 = if locals.var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign11440_e6170;
        locals.var_guard267_rv = 0.0;

        let (assign11450_e6174, assign11450_e6174_d_n0, assign11450_e6174_d_n2, assign11450_e6174_d_n4, assign11450_e6174_d_n5, assign11450_e6174_d_n6, assign11450_e6174_d_n7, assign11450_e6174_d_n8, assign11450_e6174_d_n9, assign11450_e6174_d_n10, assign11450_e6174_d_n11, assign11450_e6174_d_n14,) = {
    if (locals.var_guard267 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11450_e6174;
        locals.var_t2_dn0 = assign11450_e6174_d_n0;
        locals.var_t2_dn2 = assign11450_e6174_d_n2;
        locals.var_t2_dn4 = assign11450_e6174_d_n4;
        locals.var_t2_dn5 = assign11450_e6174_d_n5;
        locals.var_t2_dn6 = assign11450_e6174_d_n6;
        locals.var_t2_dn7 = assign11450_e6174_d_n7;
        locals.var_t2_dn8 = assign11450_e6174_d_n8;
        locals.var_t2_dn9 = assign11450_e6174_d_n9;
        locals.var_t2_dn10 = assign11450_e6174_d_n10;
        locals.var_t2_dn11 = assign11450_e6174_d_n11;
        locals.var_t2_dn14 = assign11450_e6174_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11460_e6177: f64 = (locals.var_t2 / 1e-6);
        locals.var_nsubpp = assign11460_e6177;
        locals.var_nsubpp_dn0 = (locals.var_t2_dn0 / 1e-6);
        locals.var_nsubpp_dn2 = (locals.var_t2_dn2 / 1e-6);
        locals.var_nsubpp_dn4 = (locals.var_t2_dn4 / 1e-6);
        locals.var_nsubpp_dn5 = (locals.var_t2_dn5 / 1e-6);
        locals.var_nsubpp_dn6 = (locals.var_t2_dn6 / 1e-6);
        locals.var_nsubpp_dn7 = (locals.var_t2_dn7 / 1e-6);
        locals.var_nsubpp_dn8 = (locals.var_t2_dn8 / 1e-6);
        locals.var_nsubpp_dn9 = (locals.var_t2_dn9 / 1e-6);
        locals.var_nsubpp_dn10 = (locals.var_t2_dn10 / 1e-6);
        locals.var_nsubpp_dn11 = (locals.var_t2_dn11 / 1e-6);
        locals.var_nsubpp_dn14 = (locals.var_t2_dn14 / 1e-6);
        locals.var_nsubpp_rv = 0.0;

        let assign11470_e6180: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign11470_e6180;
        locals.var_guard268_rv = 0.0;

        let (assign11480_e6188, assign11480_e6188_d_n0, assign11480_e6188_d_n2, assign11480_e6188_d_n4, assign11480_e6188_d_n5, assign11480_e6188_d_n6, assign11480_e6188_d_n7, assign11480_e6188_d_n8, assign11480_e6188_d_n9, assign11480_e6188_d_n10, assign11480_e6188_d_n11, assign11480_e6188_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11480_e6185: f64 = (1.0 + locals.var_uc_nsubpsti2);
        let assign11480_e6186: f64 = (1.0 / assign11480_e6185);
        (assign11480_e6186, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11480_e6188;
        locals.var_t1_dn0 = assign11480_e6188_d_n0;
        locals.var_t1_dn2 = assign11480_e6188_d_n2;
        locals.var_t1_dn4 = assign11480_e6188_d_n4;
        locals.var_t1_dn5 = assign11480_e6188_d_n5;
        locals.var_t1_dn6 = assign11480_e6188_d_n6;
        locals.var_t1_dn7 = assign11480_e6188_d_n7;
        locals.var_t1_dn8 = assign11480_e6188_d_n8;
        locals.var_t1_dn9 = assign11480_e6188_d_n9;
        locals.var_t1_dn10 = assign11480_e6188_d_n10;
        locals.var_t1_dn11 = assign11480_e6188_d_n11;
        locals.var_t1_dn14 = assign11480_e6188_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign11490_e6196, assign11490_e6196_d_n0, assign11490_e6196_d_n2, assign11490_e6196_d_n4, assign11490_e6196_d_n5, assign11490_e6196_d_n6, assign11490_e6196_d_n7, assign11490_e6196_d_n8, assign11490_e6196_d_n9, assign11490_e6196_d_n10, assign11490_e6196_d_n11, assign11490_e6196_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11490_e6192: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half);
        let assign11490_e6194: f64 = (assign11490_e6192).powf(locals.var_uc_nsubpsti3);
        (assign11490_e6194, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11490_e6196;
        locals.var_t2_dn0 = assign11490_e6196_d_n0;
        locals.var_t2_dn2 = assign11490_e6196_d_n2;
        locals.var_t2_dn4 = assign11490_e6196_d_n4;
        locals.var_t2_dn5 = assign11490_e6196_d_n5;
        locals.var_t2_dn6 = assign11490_e6196_d_n6;
        locals.var_t2_dn7 = assign11490_e6196_d_n7;
        locals.var_t2_dn8 = assign11490_e6196_d_n8;
        locals.var_t2_dn9 = assign11490_e6196_d_n9;
        locals.var_t2_dn10 = assign11490_e6196_d_n10;
        locals.var_t2_dn11 = assign11490_e6196_d_n11;
        locals.var_t2_dn14 = assign11490_e6196_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign11500_e6204, assign11500_e6204_d_n0, assign11500_e6204_d_n2, assign11500_e6204_d_n4, assign11500_e6204_d_n5, assign11500_e6204_d_n6, assign11500_e6204_d_n7, assign11500_e6204_d_n8, assign11500_e6204_d_n9, assign11500_e6204_d_n10, assign11500_e6204_d_n11, assign11500_e6204_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11500_e6200: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half_ref);
        let assign11500_e6202: f64 = (assign11500_e6200).powf(locals.var_uc_nsubpsti3);
        (assign11500_e6202, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign11500_e6204;
        locals.var_t3_dn0 = assign11500_e6204_d_n0;
        locals.var_t3_dn2 = assign11500_e6204_d_n2;
        locals.var_t3_dn4 = assign11500_e6204_d_n4;
        locals.var_t3_dn5 = assign11500_e6204_d_n5;
        locals.var_t3_dn6 = assign11500_e6204_d_n6;
        locals.var_t3_dn7 = assign11500_e6204_d_n7;
        locals.var_t3_dn8 = assign11500_e6204_d_n8;
        locals.var_t3_dn9 = assign11500_e6204_d_n9;
        locals.var_t3_dn10 = assign11500_e6204_d_n10;
        locals.var_t3_dn11 = assign11500_e6204_d_n11;
        locals.var_t3_dn14 = assign11500_e6204_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign11510_e6220, assign11510_e6220_d_n0, assign11510_e6220_d_n2, assign11510_e6220_d_n4, assign11510_e6220_d_n5, assign11510_e6220_d_n6, assign11510_e6220_d_n7, assign11510_e6220_d_n8, assign11510_e6220_d_n9, assign11510_e6220_d_n10, assign11510_e6220_d_n11, assign11510_e6220_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11510_e6210: f64 = (locals.var_t1 * locals.var_t2);
        let assign11510_e6211: f64 = (1.0 + assign11510_e6210);
        let assign11510_e6212: f64 = (locals.var_nsubpp * assign11510_e6211);
        let assign11510_e6216: f64 = (locals.var_t1 * locals.var_t3);
        let assign11510_e6217: f64 = (1.0 + assign11510_e6216);
        let assign11510_e6218: f64 = (assign11510_e6212 / assign11510_e6217);
        (assign11510_e6218, (((((locals.var_nsubpp_dn0 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn2 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn4 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn5 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn6 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn7 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn8 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn9 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn10 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn11 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn14 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign11510_e6217 * assign11510_e6217)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11510_e6220;
        locals.var_nsubps_dn0 = assign11510_e6220_d_n0;
        locals.var_nsubps_dn2 = assign11510_e6220_d_n2;
        locals.var_nsubps_dn4 = assign11510_e6220_d_n4;
        locals.var_nsubps_dn5 = assign11510_e6220_d_n5;
        locals.var_nsubps_dn6 = assign11510_e6220_d_n6;
        locals.var_nsubps_dn7 = assign11510_e6220_d_n7;
        locals.var_nsubps_dn8 = assign11510_e6220_d_n8;
        locals.var_nsubps_dn9 = assign11510_e6220_d_n9;
        locals.var_nsubps_dn10 = assign11510_e6220_d_n10;
        locals.var_nsubps_dn11 = assign11510_e6220_d_n11;
        locals.var_nsubps_dn14 = assign11510_e6220_d_n14;
        locals.var_nsubps_rv = 0.0;

        let (assign11520_e6225, assign11520_e6225_d_n0, assign11520_e6225_d_n2, assign11520_e6225_d_n4, assign11520_e6225_d_n5, assign11520_e6225_d_n6, assign11520_e6225_d_n7, assign11520_e6225_d_n8, assign11520_e6225_d_n9, assign11520_e6225_d_n10, assign11520_e6225_d_n11, assign11520_e6225_d_n14,) = {
    if (locals.var_guard268 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn7, locals.var_nsubpp_dn8, locals.var_nsubpp_dn9, locals.var_nsubpp_dn10, locals.var_nsubpp_dn11, locals.var_nsubpp_dn14,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11520_e6225;
        locals.var_nsubps_dn0 = assign11520_e6225_d_n0;
        locals.var_nsubps_dn2 = assign11520_e6225_d_n2;
        locals.var_nsubps_dn4 = assign11520_e6225_d_n4;
        locals.var_nsubps_dn5 = assign11520_e6225_d_n5;
        locals.var_nsubps_dn6 = assign11520_e6225_d_n6;
        locals.var_nsubps_dn7 = assign11520_e6225_d_n7;
        locals.var_nsubps_dn8 = assign11520_e6225_d_n8;
        locals.var_nsubps_dn9 = assign11520_e6225_d_n9;
        locals.var_nsubps_dn10 = assign11520_e6225_d_n10;
        locals.var_nsubps_dn11 = assign11520_e6225_d_n11;
        locals.var_nsubps_dn14 = assign11520_e6225_d_n14;
        locals.var_nsubps_rv = 0.0;

        let assign11530_e6232: f64 = if ((locals.var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard269 = assign11530_e6232;
        locals.var_guard269_rv = 0.0;

        let (assign11540_e6246, assign11540_e6246_d_n0, assign11540_e6246_d_n2, assign11540_e6246_d_n4, assign11540_e6246_d_n5, assign11540_e6246_d_n6, assign11540_e6246_d_n7, assign11540_e6246_d_n8, assign11540_e6246_d_n9, assign11540_e6246_d_n10, assign11540_e6246_d_n11, assign11540_e6246_d_n14,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11540_e6237: f64 = (locals.var_lgate - p.p140);
        let assign11540_e6238: f64 = (locals.var_ef_nsubc * assign11540_e6237);
        let assign11540_e6241: f64 = (locals.var_nsubps * p.p140);
        let assign11540_e6242: f64 = (assign11540_e6238 + assign11540_e6241);
        let assign11540_e6244: f64 = (assign11540_e6242 / locals.var_lgate);
        (assign11540_e6244, (((locals.var_ef_nsubc_dn0 * assign11540_e6237) + (locals.var_nsubps_dn0 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn2 * assign11540_e6237) + (locals.var_nsubps_dn2 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn4 * assign11540_e6237) + (locals.var_nsubps_dn4 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn5 * assign11540_e6237) + (locals.var_nsubps_dn5 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn6 * assign11540_e6237) + (locals.var_nsubps_dn6 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn7 * assign11540_e6237) + (locals.var_nsubps_dn7 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn8 * assign11540_e6237) + (locals.var_nsubps_dn8 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn9 * assign11540_e6237) + (locals.var_nsubps_dn9 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn10 * assign11540_e6237) + (locals.var_nsubps_dn10 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn11 * assign11540_e6237) + (locals.var_nsubps_dn11 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn14 * assign11540_e6237) + (locals.var_nsubps_dn14 * p.p140)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11540_e6246;
        locals.var_nsub_dn0 = assign11540_e6246_d_n0;
        locals.var_nsub_dn2 = assign11540_e6246_d_n2;
        locals.var_nsub_dn4 = assign11540_e6246_d_n4;
        locals.var_nsub_dn5 = assign11540_e6246_d_n5;
        locals.var_nsub_dn6 = assign11540_e6246_d_n6;
        locals.var_nsub_dn7 = assign11540_e6246_d_n7;
        locals.var_nsub_dn8 = assign11540_e6246_d_n8;
        locals.var_nsub_dn9 = assign11540_e6246_d_n9;
        locals.var_nsub_dn10 = assign11540_e6246_d_n10;
        locals.var_nsub_dn11 = assign11540_e6246_d_n11;
        locals.var_nsub_dn14 = assign11540_e6246_d_n14;
        locals.var_nsub_rv = 0.0;

        let (assign11550_e6261, assign11550_e6261_d_n0, assign11550_e6261_d_n2, assign11550_e6261_d_n4, assign11550_e6261_d_n5, assign11550_e6261_d_n6, assign11550_e6261_d_n7, assign11550_e6261_d_n8, assign11550_e6261_d_n9, assign11550_e6261_d_n10, assign11550_e6261_d_n11, assign11550_e6261_d_n14,) = {
    if (locals.var_guard269 == 0.0) {
        let assign11550_e6252: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11550_e6255: f64 = (p.p140 - locals.var_lgate);
        let assign11550_e6256: f64 = (assign11550_e6252 * assign11550_e6255);
        let assign11550_e6258: f64 = (assign11550_e6256 / p.p140);
        let assign11550_e6259: f64 = (locals.var_nsubps + assign11550_e6258);
        (assign11550_e6259, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn9 + (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn14 + (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * assign11550_e6255) / p.p140)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11550_e6261;
        locals.var_nsub_dn0 = assign11550_e6261_d_n0;
        locals.var_nsub_dn2 = assign11550_e6261_d_n2;
        locals.var_nsub_dn4 = assign11550_e6261_d_n4;
        locals.var_nsub_dn5 = assign11550_e6261_d_n5;
        locals.var_nsub_dn6 = assign11550_e6261_d_n6;
        locals.var_nsub_dn7 = assign11550_e6261_d_n7;
        locals.var_nsub_dn8 = assign11550_e6261_d_n8;
        locals.var_nsub_dn9 = assign11550_e6261_d_n9;
        locals.var_nsub_dn10 = assign11550_e6261_d_n10;
        locals.var_nsub_dn11 = assign11550_e6261_d_n11;
        locals.var_nsub_dn14 = assign11550_e6261_d_n14;
        locals.var_nsub_rv = 0.0;

        let assign11560_e6264: f64 = (0.5 * locals.var_lgate);
        let assign11560_e6266: f64 = (assign11560_e6264 - p.p140);
        locals.var_t3 = assign11560_e6266;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign11570_e6269: f64 = (locals.var_t3 - 1e-9);
        let assign11570_e6271: f64 = (assign11570_e6269 - 1e-10);
        locals.var_tmf1 = assign11570_e6271;
        locals.var_tmf1_dn0 = locals.var_t3_dn0;
        locals.var_tmf1_dn2 = locals.var_t3_dn2;
        locals.var_tmf1_dn4 = locals.var_t3_dn4;
        locals.var_tmf1_dn5 = locals.var_t3_dn5;
        locals.var_tmf1_dn6 = locals.var_t3_dn6;
        locals.var_tmf1_dn7 = locals.var_t3_dn7;
        locals.var_tmf1_dn8 = locals.var_t3_dn8;
        locals.var_tmf1_dn9 = locals.var_t3_dn9;
        locals.var_tmf1_dn10 = locals.var_t3_dn10;
        locals.var_tmf1_dn11 = locals.var_t3_dn11;
        locals.var_tmf1_dn14 = locals.var_t3_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign11580_e6274: f64 = (4.0 * 1e-9);
        let assign11580_e6276: f64 = (assign11580_e6274 * 1e-10);
        locals.var_tmf2 = assign11580_e6276;
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

        let (assign11590_e6283, assign11590_e6283_d_n0, assign11590_e6283_d_n2, assign11590_e6283_d_n4, assign11590_e6283_d_n5, assign11590_e6283_d_n6, assign11590_e6283_d_n7, assign11590_e6283_d_n8, assign11590_e6283_d_n9, assign11590_e6283_d_n10, assign11590_e6283_d_n11, assign11590_e6283_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign11590_e6282: f64 = (-locals.var_tmf2);
        (assign11590_e6282, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign11590_e6283;
        locals.var_tmf2_dn0 = assign11590_e6283_d_n0;
        locals.var_tmf2_dn2 = assign11590_e6283_d_n2;
        locals.var_tmf2_dn4 = assign11590_e6283_d_n4;
        locals.var_tmf2_dn5 = assign11590_e6283_d_n5;
        locals.var_tmf2_dn6 = assign11590_e6283_d_n6;
        locals.var_tmf2_dn7 = assign11590_e6283_d_n7;
        locals.var_tmf2_dn8 = assign11590_e6283_d_n8;
        locals.var_tmf2_dn9 = assign11590_e6283_d_n9;
        locals.var_tmf2_dn10 = assign11590_e6283_d_n10;
        locals.var_tmf2_dn11 = assign11590_e6283_d_n11;
        locals.var_tmf2_dn14 = assign11590_e6283_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign11600_e6286: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign11600_e6288: f64 = (assign11600_e6286 + locals.var_tmf2);
        let assign11600_e6289: f64 = (assign11600_e6288).sqrt();
        locals.var_tmf2 = assign11600_e6289;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign11600_e6289));
        locals.var_tmf2_rv = 0.0;

        let assign11610_e6294: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign11610_e6295: f64 = (1.0 + assign11610_e6294);
        let assign11610_e6296: f64 = (0.5 * assign11610_e6295);
        locals.var_t0 = assign11610_e6296;
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

        let assign11620_e6301: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign11620_e6302: f64 = (0.5 * assign11620_e6301);
        let assign11620_e6303: f64 = (1e-9 + assign11620_e6302);
        locals.var_t3 = assign11620_e6303;
        locals.var_t3_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_t3_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_t3_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_t3_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));
        locals.var_t3_rv = 0.0;

        let assign11630_e6307: f64 = (1.0 / locals.var_t3);
        let assign11630_e6310: f64 = (1.0 / p.p220);
        let assign11630_e6311: f64 = (assign11630_e6307 + assign11630_e6310);
        let assign11630_e6312: f64 = (1.0 / assign11630_e6311);
        locals.var_t1 = assign11630_e6312;
        locals.var_t1_dn0 = (-((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn2 = (-((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn4 = (-((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn5 = (-((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn6 = (-((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn7 = (-((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn8 = (-((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn9 = (-((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn10 = (-((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn11 = (-((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn14 = (-((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_rv = 0.0;

        let (assign11640_e6318, assign11640_e6318_d_n0, assign11640_e6318_d_n2, assign11640_e6318_d_n4, assign11640_e6318_d_n5, assign11640_e6318_d_n6, assign11640_e6318_d_n7, assign11640_e6318_d_n8, assign11640_e6318_d_n9, assign11640_e6318_d_n10, assign11640_e6318_d_n11, assign11640_e6318_d_n14,) = {
    if (0.0 >= locals.var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t2 = assign11640_e6318;
        locals.var_t2_dn0 = assign11640_e6318_d_n0;
        locals.var_t2_dn2 = assign11640_e6318_d_n2;
        locals.var_t2_dn4 = assign11640_e6318_d_n4;
        locals.var_t2_dn5 = assign11640_e6318_d_n5;
        locals.var_t2_dn6 = assign11640_e6318_d_n6;
        locals.var_t2_dn7 = assign11640_e6318_d_n7;
        locals.var_t2_dn8 = assign11640_e6318_d_n8;
        locals.var_t2_dn9 = assign11640_e6318_d_n9;
        locals.var_t2_dn10 = assign11640_e6318_d_n10;
        locals.var_t2_dn11 = assign11640_e6318_d_n11;
        locals.var_t2_dn14 = assign11640_e6318_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11650_e6323: f64 = (locals.var_npexte - locals.var_ef_nsubc);
        let assign11650_e6324: f64 = (locals.var_t2 * assign11650_e6323);
        let assign11650_e6326: f64 = (assign11650_e6324 / locals.var_lgate);
        let assign11650_e6327: f64 = (locals.var_nsub + assign11650_e6326);
        locals.var_nsub = assign11650_e6327;
        locals.var_nsub_dn0 = (locals.var_nsub_dn0 + (((locals.var_t2_dn0 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0))) / locals.var_lgate));
        locals.var_nsub_dn2 = (locals.var_nsub_dn2 + (((locals.var_t2_dn2 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2))) / locals.var_lgate));
        locals.var_nsub_dn4 = (locals.var_nsub_dn4 + (((locals.var_t2_dn4 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4))) / locals.var_lgate));
        locals.var_nsub_dn5 = (locals.var_nsub_dn5 + (((locals.var_t2_dn5 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5))) / locals.var_lgate));
        locals.var_nsub_dn6 = (locals.var_nsub_dn6 + (((locals.var_t2_dn6 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6))) / locals.var_lgate));
        locals.var_nsub_dn7 = (locals.var_nsub_dn7 + (((locals.var_t2_dn7 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7))) / locals.var_lgate));
        locals.var_nsub_dn8 = (locals.var_nsub_dn8 + (((locals.var_t2_dn8 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8))) / locals.var_lgate));
        locals.var_nsub_dn9 = (locals.var_nsub_dn9 + (((locals.var_t2_dn9 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9))) / locals.var_lgate));
        locals.var_nsub_dn10 = (locals.var_nsub_dn10 + (((locals.var_t2_dn10 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10))) / locals.var_lgate));
        locals.var_nsub_dn11 = (locals.var_nsub_dn11 + (((locals.var_t2_dn11 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11))) / locals.var_lgate));
        locals.var_nsub_dn14 = (locals.var_nsub_dn14 + (((locals.var_t2_dn14 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14))) / locals.var_lgate));
        locals.var_nsub_rv = 0.0;

        let assign11660_e6330: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign11660_e6330;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn9 = (1.6021918e-19 * locals.var_nsub_dn9);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn14 = (1.6021918e-19 * locals.var_nsub_dn14);
        locals.var_q_nsub_rv = 0.0;

        let assign11670_e6333: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign11670_e6333;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn9 = (locals.var_q_nsub_dn9 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn14 = (locals.var_q_nsub_dn14 * 1.034943e-10);
        locals.var_qnsub_esi_rv = 0.0;

        let assign11680_e6336: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign11680_e6336;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn9 = (2.0 * locals.var_qnsub_esi_dn9);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn14 = (2.0 * locals.var_qnsub_esi_dn14);
        locals.var_qnsub_esi2_rv = 0.0;

        let assign11690_e6340: f64 = (2.0 * p.p140);
        let assign11690_e6345: f64 = if ((locals.var_lgate <= assign11690_e6340) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign11690_e6345;
        locals.var_guard270_rv = 0.0;

        let (assign11700_e6361, assign11700_e6361_d_n0, assign11700_e6361_d_n2, assign11700_e6361_d_n4, assign11700_e6361_d_n5, assign11700_e6361_d_n6, assign11700_e6361_d_n7, assign11700_e6361_d_n8, assign11700_e6361_d_n9, assign11700_e6361_d_n10, assign11700_e6361_d_n11, assign11700_e6361_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11700_e6349: f64 = (2.0 * locals.var_nsubps);
        let assign11700_e6352: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11700_e6354: f64 = (assign11700_e6352 * locals.var_lgate);
        let assign11700_e6356: f64 = (assign11700_e6354 / p.p140);
        let assign11700_e6357: f64 = (assign11700_e6349 - assign11700_e6356);
        let assign11700_e6359: f64 = (assign11700_e6357 - locals.var_ef_nsubc);
        (assign11700_e6359, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn7), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn8), (((2.0 * locals.var_nsubps_dn9) - (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn9), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn11), (((2.0 * locals.var_nsubps_dn14) - (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_nsubb, locals.var_nsubb_dn0, locals.var_nsubb_dn2, locals.var_nsubb_dn4, locals.var_nsubb_dn5, locals.var_nsubb_dn6, locals.var_nsubb_dn7, locals.var_nsubb_dn8, locals.var_nsubb_dn9, locals.var_nsubb_dn10, locals.var_nsubb_dn11, locals.var_nsubb_dn14,)
    }
};
        locals.var_nsubb = assign11700_e6361;
        locals.var_nsubb_dn0 = assign11700_e6361_d_n0;
        locals.var_nsubb_dn2 = assign11700_e6361_d_n2;
        locals.var_nsubb_dn4 = assign11700_e6361_d_n4;
        locals.var_nsubb_dn5 = assign11700_e6361_d_n5;
        locals.var_nsubb_dn6 = assign11700_e6361_d_n6;
        locals.var_nsubb_dn7 = assign11700_e6361_d_n7;
        locals.var_nsubb_dn8 = assign11700_e6361_d_n8;
        locals.var_nsubb_dn9 = assign11700_e6361_d_n9;
        locals.var_nsubb_dn10 = assign11700_e6361_d_n10;
        locals.var_nsubb_dn11 = assign11700_e6361_d_n11;
        locals.var_nsubb_dn14 = assign11700_e6361_d_n14;
        locals.var_nsubb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11710_e6368, assign11710_e6368_d_n0, assign11710_e6368_d_n2, assign11710_e6368_d_n4, assign11710_e6368_d_n5, assign11710_e6368_d_n6, assign11710_e6368_d_n7, assign11710_e6368_d_n8, assign11710_e6368_d_n9, assign11710_e6368_d_n10, assign11710_e6368_d_n11, assign11710_e6368_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11710_e6365: f64 = (locals.var_nsubb / locals.var_ef_nsubc);
        let assign11710_e6366: f64 = (assign11710_e6365).ln();
        (assign11710_e6366, ((((locals.var_nsubb_dn0 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn2 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn4 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn5 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn6 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn7 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn8 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn9 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn10 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn11 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn14 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11710_e6368;
        locals.var_ptovr0_dn0 = assign11710_e6368_d_n0;
        locals.var_ptovr0_dn2 = assign11710_e6368_d_n2;
        locals.var_ptovr0_dn4 = assign11710_e6368_d_n4;
        locals.var_ptovr0_dn5 = assign11710_e6368_d_n5;
        locals.var_ptovr0_dn6 = assign11710_e6368_d_n6;
        locals.var_ptovr0_dn7 = assign11710_e6368_d_n7;
        locals.var_ptovr0_dn8 = assign11710_e6368_d_n8;
        locals.var_ptovr0_dn9 = assign11710_e6368_d_n9;
        locals.var_ptovr0_dn10 = assign11710_e6368_d_n10;
        locals.var_ptovr0_dn11 = assign11710_e6368_d_n11;
        locals.var_ptovr0_dn14 = assign11710_e6368_d_n14;
        locals.var_ptovr0_rv = 0.0;

        let (assign11720_e6373, assign11720_e6373_d_n0, assign11720_e6373_d_n2, assign11720_e6373_d_n4, assign11720_e6373_d_n5, assign11720_e6373_d_n6, assign11720_e6373_d_n7, assign11720_e6373_d_n8, assign11720_e6373_d_n9, assign11720_e6373_d_n10, assign11720_e6373_d_n11, assign11720_e6373_d_n14,) = {
    if (locals.var_guard270 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11720_e6373;
        locals.var_ptovr0_dn0 = assign11720_e6373_d_n0;
        locals.var_ptovr0_dn2 = assign11720_e6373_d_n2;
        locals.var_ptovr0_dn4 = assign11720_e6373_d_n4;
        locals.var_ptovr0_dn5 = assign11720_e6373_d_n5;
        locals.var_ptovr0_dn6 = assign11720_e6373_d_n6;
        locals.var_ptovr0_dn7 = assign11720_e6373_d_n7;
        locals.var_ptovr0_dn8 = assign11720_e6373_d_n8;
        locals.var_ptovr0_dn9 = assign11720_e6373_d_n9;
        locals.var_ptovr0_dn10 = assign11720_e6373_d_n10;
        locals.var_ptovr0_dn11 = assign11720_e6373_d_n11;
        locals.var_ptovr0_dn14 = assign11720_e6373_d_n14;
        locals.var_ptovr0_rv = 0.0;

        let assign11730_e6376: f64 = (2.0 * 1.6021918e-19);
        let assign11730_e6378: f64 = (assign11730_e6376 * locals.var_uc_nsti);
        let assign11730_e6380: f64 = (assign11730_e6378 * 1.034943e-10);
        let assign11730_e6381: f64 = (assign11730_e6380).sqrt();
        locals.var_costi00 = assign11730_e6381;
        locals.var_costi00_rv = 0.0;

        let assign11740_e6385: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign11740_e6386: f64 = (1.0 / assign11740_e6385);
        locals.var_nsti_p2 = assign11740_e6386;
        locals.var_nsti_p2_rv = 0.0;

        let assign11750_e6391: f64 = (locals.var_lg).powf(p.p231);
        let assign11750_e6392: f64 = (locals.var_uc_vover / assign11750_e6391);
        let assign11750_e6393: f64 = (1.0 + assign11750_e6392);
        let assign11750_e6398: f64 = (locals.var_wlg).powf(p.p239);
        let assign11750_e6399: f64 = (p.p238 / assign11750_e6398);
        let assign11750_e6400: f64 = (1.0 + assign11750_e6399);
        let assign11750_e6401: f64 = (assign11750_e6393 * assign11750_e6400);
        locals.var_vmax0 = assign11750_e6401;
        locals.var_vmax0_rv = 0.0;

        let assign11760_e6404: f64 = (2.0 / 38.68283);
        let assign11760_e6407: f64 = (locals.var_nsub / 1.04e16);
        let assign11760_e6408: f64 = (assign11760_e6407).ln();
        let assign11760_e6409: f64 = (assign11760_e6404 * assign11760_e6408);
        locals.var_pb20 = assign11760_e6409;
        locals.var_pb20_dn0 = (assign11760_e6404 * ((locals.var_nsub_dn0 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn2 = (assign11760_e6404 * ((locals.var_nsub_dn2 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn4 = (assign11760_e6404 * ((locals.var_nsub_dn4 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn5 = (assign11760_e6404 * ((locals.var_nsub_dn5 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn6 = (assign11760_e6404 * ((locals.var_nsub_dn6 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn7 = (assign11760_e6404 * ((locals.var_nsub_dn7 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn8 = (assign11760_e6404 * ((locals.var_nsub_dn8 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn9 = (assign11760_e6404 * ((locals.var_nsub_dn9 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn10 = (assign11760_e6404 * ((locals.var_nsub_dn10 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn11 = (assign11760_e6404 * ((locals.var_nsub_dn11 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn14 = (assign11760_e6404 * ((locals.var_nsub_dn14 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_rv = 0.0;

        let assign11770_e6412: f64 = (2.0 / 38.68283);
        let assign11770_e6415: f64 = (locals.var_ef_nsubc / 1.04e16);
        let assign11770_e6416: f64 = (assign11770_e6415).ln();
        let assign11770_e6417: f64 = (assign11770_e6412 * assign11770_e6416);
        locals.var_pb2c = assign11770_e6417;
        locals.var_pb2c_dn0 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn0 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn2 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn2 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn4 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn4 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn5 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn5 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn6 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn6 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn7 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn7 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn8 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn8 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn9 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn9 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn10 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn10 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn11 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn11 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn14 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn14 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_rv = 0.0;

        let assign11780_e6420: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign11780_e6420;
        locals.var_guard271_rv = 0.0;

        let (assign11790_e6430, assign11790_e6430_d_n0, assign11790_e6430_d_n2, assign11790_e6430_d_n4, assign11790_e6430_d_n5, assign11790_e6430_d_n6, assign11790_e6430_d_n7, assign11790_e6430_d_n8, assign11790_e6430_d_n9, assign11790_e6430_d_n10, assign11790_e6430_d_n11, assign11790_e6430_d_n14,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11790_e6426: f64 = (3.0 * p.p4);
        let assign11790_e6427: f64 = (locals.var_weff / assign11790_e6426);
        let assign11790_e6428: f64 = (p.p5 + assign11790_e6427);
        (assign11790_e6428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11790_e6430;
        locals.var_t1_dn0 = assign11790_e6430_d_n0;
        locals.var_t1_dn2 = assign11790_e6430_d_n2;
        locals.var_t1_dn4 = assign11790_e6430_d_n4;
        locals.var_t1_dn5 = assign11790_e6430_d_n5;
        locals.var_t1_dn6 = assign11790_e6430_d_n6;
        locals.var_t1_dn7 = assign11790_e6430_d_n7;
        locals.var_t1_dn8 = assign11790_e6430_d_n8;
        locals.var_t1_dn9 = assign11790_e6430_d_n9;
        locals.var_t1_dn10 = assign11790_e6430_d_n10;
        locals.var_t1_dn11 = assign11790_e6430_d_n11;
        locals.var_t1_dn14 = assign11790_e6430_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign11800_e6436, assign11800_e6436_d_n0, assign11800_e6436_d_n2, assign11800_e6436_d_n4, assign11800_e6436_d_n5, assign11800_e6436_d_n6, assign11800_e6436_d_n7, assign11800_e6436_d_n8, assign11800_e6436_d_n9, assign11800_e6436_d_n10, assign11800_e6436_d_n11, assign11800_e6436_d_n14,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11800_e6434: f64 = (locals.var_lgate - p.p6);
        (assign11800_e6434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11800_e6436;
        locals.var_t2_dn0 = assign11800_e6436_d_n0;
        locals.var_t2_dn2 = assign11800_e6436_d_n2;
        locals.var_t2_dn4 = assign11800_e6436_d_n4;
        locals.var_t2_dn5 = assign11800_e6436_d_n5;
        locals.var_t2_dn6 = assign11800_e6436_d_n6;
        locals.var_t2_dn7 = assign11800_e6436_d_n7;
        locals.var_t2_dn8 = assign11800_e6436_d_n8;
        locals.var_t2_dn9 = assign11800_e6436_d_n9;
        locals.var_t2_dn10 = assign11800_e6436_d_n10;
        locals.var_t2_dn11 = assign11800_e6436_d_n11;
        locals.var_t2_dn14 = assign11800_e6436_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11860_e6478: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign11860_e6478;
        locals.var_guard273_rv = 0.0;

        let (assign11870_e6484,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11870_e6482: f64 = (p.p130 * p.p2);
        (assign11870_e6482,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11870_e6484;
        locals.var_rd0_rv = 0.0;

        let (assign11880_e6490,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11880_e6488: f64 = (p.p130 * p.p3);
        (assign11880_e6488,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11880_e6490;
        locals.var_rs0_rv = 0.0;

        let (assign11890_e6495,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11890_e6495;
        locals.var_rd0_rv = 0.0;

        let (assign11900_e6500,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11900_e6500;
        locals.var_rs0_rv = 0.0;

        let assign11910_e6503: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign11910_e6503;
        locals.var_guard274_rv = 0.0;

        let (assign11920_e6509,) = {
    if (locals.var_guard274 != 0.0) {
        let assign11920_e6507: f64 = (p.p131 * p.p3);
        (assign11920_e6507,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11920_e6509;
        locals.var_rs0_rv = 0.0;

        let (assign11930_e6514,) = {
    if (locals.var_guard274 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11930_e6514;
        locals.var_rs0_rv = 0.0;

        let assign11940_e6517: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign11940_e6517;
        locals.var_guard275_rv = 0.0;

        let assign11950_e6524: f64 = if ((locals.var_uc_rd > 0.0) || (locals.var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard276 = assign11950_e6524;
        locals.var_guard276_rv = 0.0;

        let (assign11960_e6536,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) {
        let assign11960_e6532: f64 = (locals.var_wlg).powf(p.p310);
        let assign11960_e6533: f64 = (p.p309 / assign11960_e6532);
        let assign11960_e6534: f64 = (1.0 + assign11960_e6533);
        (assign11960_e6534,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign11960_e6536;
        locals.var_rdtemp0_rv = 0.0;

        let assign11970_e6539: f64 = if locals.var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign11970_e6539;
        locals.var_guard277_rv = 0.0;

        let (assign11980_e6553, assign11980_e6553_d_n0, assign11980_e6553_d_n2, assign11980_e6553_d_n4, assign11980_e6553_d_n5, assign11980_e6553_d_n6, assign11980_e6553_d_n7, assign11980_e6553_d_n8, assign11980_e6553_d_n9, assign11980_e6553_d_n10, assign11980_e6553_d_n11, assign11980_e6553_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign11980_e6549: f64 = (locals.var_wlg).powf(p.p304);
        let assign11980_e6550: f64 = (p.p303 / assign11980_e6549);
        let assign11980_e6551: f64 = (1.0 + assign11980_e6550);
        (assign11980_e6551, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign11980_e6553;
        locals.var_t7_dn0 = assign11980_e6553_d_n0;
        locals.var_t7_dn2 = assign11980_e6553_d_n2;
        locals.var_t7_dn4 = assign11980_e6553_d_n4;
        locals.var_t7_dn5 = assign11980_e6553_d_n5;
        locals.var_t7_dn6 = assign11980_e6553_d_n6;
        locals.var_t7_dn7 = assign11980_e6553_d_n7;
        locals.var_t7_dn8 = assign11980_e6553_d_n8;
        locals.var_t7_dn9 = assign11980_e6553_d_n9;
        locals.var_t7_dn10 = assign11980_e6553_d_n10;
        locals.var_t7_dn11 = assign11980_e6553_d_n11;
        locals.var_t7_dn14 = assign11980_e6553_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign11990_e6566, assign11990_e6566_d_n0, assign11990_e6566_d_n2, assign11990_e6566_d_n4, assign11990_e6566_d_n5, assign11990_e6566_d_n6, assign11990_e6566_d_n7, assign11990_e6566_d_n8, assign11990_e6566_d_n9, assign11990_e6566_d_n10, assign11990_e6566_d_n11, assign11990_e6566_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign11990_e6560: f64 = (-p.p301);
        let assign11990_e6563: f64 = (locals.var_lg).powf(p.p302);
        let assign11990_e6564: f64 = (assign11990_e6560 * assign11990_e6563);
        (assign11990_e6564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign11990_e6566;
        locals.var_t6_dn0 = assign11990_e6566_d_n0;
        locals.var_t6_dn2 = assign11990_e6566_d_n2;
        locals.var_t6_dn4 = assign11990_e6566_d_n4;
        locals.var_t6_dn5 = assign11990_e6566_d_n5;
        locals.var_t6_dn6 = assign11990_e6566_d_n6;
        locals.var_t6_dn7 = assign11990_e6566_d_n7;
        locals.var_t6_dn8 = assign11990_e6566_d_n8;
        locals.var_t6_dn9 = assign11990_e6566_d_n9;
        locals.var_t6_dn10 = assign11990_e6566_d_n10;
        locals.var_t6_dn11 = assign11990_e6566_d_n11;
        locals.var_t6_dn14 = assign11990_e6566_d_n14;
        locals.var_t6_rv = 0.0;

        let assign12000_e6569: f64 = if locals.var_t6 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign12000_e6569;
        locals.var_guard278_rv = 0.0;

        let (assign12010_e6579, assign12010_e6579_d_n0, assign12010_e6579_d_n2, assign12010_e6579_d_n4, assign12010_e6579_d_n5, assign12010_e6579_d_n6, assign12010_e6579_d_n7, assign12010_e6579_d_n8, assign12010_e6579_d_n9, assign12010_e6579_d_n10, assign12010_e6579_d_n11, assign12010_e6579_d_n14,) = {
    if ((((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12010_e6579;
        locals.var_t6_dn0 = assign12010_e6579_d_n0;
        locals.var_t6_dn2 = assign12010_e6579_d_n2;
        locals.var_t6_dn4 = assign12010_e6579_d_n4;
        locals.var_t6_dn5 = assign12010_e6579_d_n5;
        locals.var_t6_dn6 = assign12010_e6579_d_n6;
        locals.var_t6_dn7 = assign12010_e6579_d_n7;
        locals.var_t6_dn8 = assign12010_e6579_d_n8;
        locals.var_t6_dn9 = assign12010_e6579_d_n9;
        locals.var_t6_dn10 = assign12010_e6579_d_n10;
        locals.var_t6_dn11 = assign12010_e6579_d_n11;
        locals.var_t6_dn14 = assign12010_e6579_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign12020_e6588, assign12020_e6588_d_n0, assign12020_e6588_d_n2, assign12020_e6588_d_n4, assign12020_e6588_d_n5, assign12020_e6588_d_n6, assign12020_e6588_d_n7, assign12020_e6588_d_n8, assign12020_e6588_d_n9, assign12020_e6588_d_n10, assign12020_e6588_d_n11, assign12020_e6588_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign12020_e6586: f64 = (locals.var_t6).exp();
        (assign12020_e6586, (assign12020_e6586 * locals.var_t6_dn0), (assign12020_e6586 * locals.var_t6_dn2), (assign12020_e6586 * locals.var_t6_dn4), (assign12020_e6586 * locals.var_t6_dn5), (assign12020_e6586 * locals.var_t6_dn6), (assign12020_e6586 * locals.var_t6_dn7), (assign12020_e6586 * locals.var_t6_dn8), (assign12020_e6586 * locals.var_t6_dn9), (assign12020_e6586 * locals.var_t6_dn10), (assign12020_e6586 * locals.var_t6_dn11), (assign12020_e6586 * locals.var_t6_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12020_e6588;
        locals.var_t6_dn0 = assign12020_e6588_d_n0;
        locals.var_t6_dn2 = assign12020_e6588_d_n2;
        locals.var_t6_dn4 = assign12020_e6588_d_n4;
        locals.var_t6_dn5 = assign12020_e6588_d_n5;
        locals.var_t6_dn6 = assign12020_e6588_d_n6;
        locals.var_t6_dn7 = assign12020_e6588_d_n7;
        locals.var_t6_dn8 = assign12020_e6588_d_n8;
        locals.var_t6_dn9 = assign12020_e6588_d_n9;
        locals.var_t6_dn10 = assign12020_e6588_d_n10;
        locals.var_t6_dn11 = assign12020_e6588_d_n11;
        locals.var_t6_dn14 = assign12020_e6588_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign12030_e6598, assign12030_e6598_d_n0, assign12030_e6598_d_n2, assign12030_e6598_d_n4, assign12030_e6598_d_n5, assign12030_e6598_d_n6, assign12030_e6598_d_n7, assign12030_e6598_d_n8, assign12030_e6598_d_n9, assign12030_e6598_d_n10, assign12030_e6598_d_n11, assign12030_e6598_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign12030_e6596: f64 = (locals.var_t6 * locals.var_t7);
        (assign12030_e6596, ((locals.var_t6_dn0 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn0)), ((locals.var_t6_dn2 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn2)), ((locals.var_t6_dn4 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn4)), ((locals.var_t6_dn5 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn5)), ((locals.var_t6_dn6 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn6)), ((locals.var_t6_dn7 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn7)), ((locals.var_t6_dn8 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn8)), ((locals.var_t6_dn9 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn9)), ((locals.var_t6_dn10 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn10)), ((locals.var_t6_dn11 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn11)), ((locals.var_t6_dn14 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn14)),)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12030_e6598;
        locals.var_rdvdtemp0_dn0 = assign12030_e6598_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12030_e6598_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12030_e6598_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12030_e6598_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12030_e6598_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12030_e6598_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12030_e6598_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12030_e6598_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12030_e6598_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12030_e6598_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12030_e6598_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12040_e6607, assign12040_e6607_d_n0, assign12040_e6607_d_n2, assign12040_e6607_d_n4, assign12040_e6607_d_n5, assign12040_e6607_d_n6, assign12040_e6607_d_n7, assign12040_e6607_d_n8, assign12040_e6607_d_n9, assign12040_e6607_d_n10, assign12040_e6607_d_n11, assign12040_e6607_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12040_e6607;
        locals.var_rdvdtemp0_dn0 = assign12040_e6607_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12040_e6607_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12040_e6607_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12040_e6607_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12040_e6607_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12040_e6607_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12040_e6607_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12040_e6607_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12040_e6607_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12040_e6607_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12040_e6607_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12050_e6614,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12050_e6614;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12060_e6621, assign12060_e6621_d_n0, assign12060_e6621_d_n2, assign12060_e6621_d_n4, assign12060_e6621_d_n5, assign12060_e6621_d_n6, assign12060_e6621_d_n7, assign12060_e6621_d_n8, assign12060_e6621_d_n9, assign12060_e6621_d_n10, assign12060_e6621_d_n11, assign12060_e6621_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12060_e6621;
        locals.var_rdvdtemp0_dn0 = assign12060_e6621_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12060_e6621_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12060_e6621_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12060_e6621_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12060_e6621_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12060_e6621_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12060_e6621_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12060_e6621_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12060_e6621_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12060_e6621_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12060_e6621_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let assign12070_e6624: f64 = if locals.var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign12070_e6624;
        locals.var_guard279_rv = 0.0;

        let (assign12080_e6636, assign12080_e6636_d_n0, assign12080_e6636_d_n2, assign12080_e6636_d_n4, assign12080_e6636_d_n5, assign12080_e6636_d_n6, assign12080_e6636_d_n7, assign12080_e6636_d_n8, assign12080_e6636_d_n9, assign12080_e6636_d_n10, assign12080_e6636_d_n11, assign12080_e6636_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12080_e6632: f64 = (locals.var_wlg).powf(p.p308);
        let assign12080_e6633: f64 = (p.p307 / assign12080_e6632);
        let assign12080_e6634: f64 = (1.0 + assign12080_e6633);
        (assign12080_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12080_e6636;
        locals.var_t2_dn0 = assign12080_e6636_d_n0;
        locals.var_t2_dn2 = assign12080_e6636_d_n2;
        locals.var_t2_dn4 = assign12080_e6636_d_n4;
        locals.var_t2_dn5 = assign12080_e6636_d_n5;
        locals.var_t2_dn6 = assign12080_e6636_d_n6;
        locals.var_t2_dn7 = assign12080_e6636_d_n7;
        locals.var_t2_dn8 = assign12080_e6636_d_n8;
        locals.var_t2_dn9 = assign12080_e6636_d_n9;
        locals.var_t2_dn10 = assign12080_e6636_d_n10;
        locals.var_t2_dn11 = assign12080_e6636_d_n11;
        locals.var_t2_dn14 = assign12080_e6636_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n11, assign12090_e6647_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12090_e6641: f64 = (-p.p305);
        let assign12090_e6644: f64 = (locals.var_lg).powf(p.p306);
        let assign12090_e6645: f64 = (assign12090_e6641 * assign12090_e6644);
        (assign12090_e6645, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12090_e6647;
        locals.var_t1_dn0 = assign12090_e6647_d_n0;
        locals.var_t1_dn2 = assign12090_e6647_d_n2;
        locals.var_t1_dn4 = assign12090_e6647_d_n4;
        locals.var_t1_dn5 = assign12090_e6647_d_n5;
        locals.var_t1_dn6 = assign12090_e6647_d_n6;
        locals.var_t1_dn7 = assign12090_e6647_d_n7;
        locals.var_t1_dn8 = assign12090_e6647_d_n8;
        locals.var_t1_dn9 = assign12090_e6647_d_n9;
        locals.var_t1_dn10 = assign12090_e6647_d_n10;
        locals.var_t1_dn11 = assign12090_e6647_d_n11;
        locals.var_t1_dn14 = assign12090_e6647_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12100_e6650: f64 = if locals.var_t1 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign12100_e6650;
        locals.var_guard280_rv = 0.0;

        let (assign12110_e6658, assign12110_e6658_d_n0, assign12110_e6658_d_n2, assign12110_e6658_d_n4, assign12110_e6658_d_n5, assign12110_e6658_d_n6, assign12110_e6658_d_n7, assign12110_e6658_d_n8, assign12110_e6658_d_n9, assign12110_e6658_d_n10, assign12110_e6658_d_n11, assign12110_e6658_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) && (locals.var_guard280 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12110_e6658;
        locals.var_t1_dn0 = assign12110_e6658_d_n0;
        locals.var_t1_dn2 = assign12110_e6658_d_n2;
        locals.var_t1_dn4 = assign12110_e6658_d_n4;
        locals.var_t1_dn5 = assign12110_e6658_d_n5;
        locals.var_t1_dn6 = assign12110_e6658_d_n6;
        locals.var_t1_dn7 = assign12110_e6658_d_n7;
        locals.var_t1_dn8 = assign12110_e6658_d_n8;
        locals.var_t1_dn9 = assign12110_e6658_d_n9;
        locals.var_t1_dn10 = assign12110_e6658_d_n10;
        locals.var_t1_dn11 = assign12110_e6658_d_n11;
        locals.var_t1_dn14 = assign12110_e6658_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12120_e6665, assign12120_e6665_d_n0, assign12120_e6665_d_n2, assign12120_e6665_d_n4, assign12120_e6665_d_n5, assign12120_e6665_d_n6, assign12120_e6665_d_n7, assign12120_e6665_d_n8, assign12120_e6665_d_n9, assign12120_e6665_d_n10, assign12120_e6665_d_n11, assign12120_e6665_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12120_e6663: f64 = (locals.var_t1).exp();
        (assign12120_e6663, (assign12120_e6663 * locals.var_t1_dn0), (assign12120_e6663 * locals.var_t1_dn2), (assign12120_e6663 * locals.var_t1_dn4), (assign12120_e6663 * locals.var_t1_dn5), (assign12120_e6663 * locals.var_t1_dn6), (assign12120_e6663 * locals.var_t1_dn7), (assign12120_e6663 * locals.var_t1_dn8), (assign12120_e6663 * locals.var_t1_dn9), (assign12120_e6663 * locals.var_t1_dn10), (assign12120_e6663 * locals.var_t1_dn11), (assign12120_e6663 * locals.var_t1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12120_e6665;
        locals.var_t1_dn0 = assign12120_e6665_d_n0;
        locals.var_t1_dn2 = assign12120_e6665_d_n2;
        locals.var_t1_dn4 = assign12120_e6665_d_n4;
        locals.var_t1_dn5 = assign12120_e6665_d_n5;
        locals.var_t1_dn6 = assign12120_e6665_d_n6;
        locals.var_t1_dn7 = assign12120_e6665_d_n7;
        locals.var_t1_dn8 = assign12120_e6665_d_n8;
        locals.var_t1_dn9 = assign12120_e6665_d_n9;
        locals.var_t1_dn10 = assign12120_e6665_d_n10;
        locals.var_t1_dn11 = assign12120_e6665_d_n11;
        locals.var_t1_dn14 = assign12120_e6665_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign12130_e6675, assign12130_e6675_d_n0, assign12130_e6675_d_n2, assign12130_e6675_d_n4, assign12130_e6675_d_n5, assign12130_e6675_d_n6, assign12130_e6675_d_n7, assign12130_e6675_d_n8, assign12130_e6675_d_n9, assign12130_e6675_d_n10, assign12130_e6675_d_n11, assign12130_e6675_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12130_e6671: f64 = (locals.var_uc_rd23 * locals.var_t2);
        let assign12130_e6673: f64 = (assign12130_e6671 * locals.var_t1);
        (assign12130_e6673, (((locals.var_uc_rd23 * locals.var_t2_dn0) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn0)), (((locals.var_uc_rd23 * locals.var_t2_dn2) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn2)), (((locals.var_uc_rd23 * locals.var_t2_dn4) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn4)), (((locals.var_uc_rd23 * locals.var_t2_dn5) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn5)), (((locals.var_uc_rd23 * locals.var_t2_dn6) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn6)), (((locals.var_uc_rd23 * locals.var_t2_dn7) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn7)), (((locals.var_uc_rd23 * locals.var_t2_dn8) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn8)), (((locals.var_uc_rd23 * locals.var_t2_dn9) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn9)), (((locals.var_uc_rd23 * locals.var_t2_dn10) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn10)), (((locals.var_uc_rd23 * locals.var_t2_dn11) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn11)), (((locals.var_uc_rd23 * locals.var_t2_dn14) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign12130_e6675;
        locals.var_t3_dn0 = assign12130_e6675_d_n0;
        locals.var_t3_dn2 = assign12130_e6675_d_n2;
        locals.var_t3_dn4 = assign12130_e6675_d_n4;
        locals.var_t3_dn5 = assign12130_e6675_d_n5;
        locals.var_t3_dn6 = assign12130_e6675_d_n6;
        locals.var_t3_dn7 = assign12130_e6675_d_n7;
        locals.var_t3_dn8 = assign12130_e6675_d_n8;
        locals.var_t3_dn9 = assign12130_e6675_d_n9;
        locals.var_t3_dn10 = assign12130_e6675_d_n10;
        locals.var_t3_dn11 = assign12130_e6675_d_n11;
        locals.var_t3_dn14 = assign12130_e6675_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign12140_e6698, assign12140_e6698_d_n0, assign12140_e6698_d_n2, assign12140_e6698_d_n4, assign12140_e6698_d_n5, assign12140_e6698_d_n6, assign12140_e6698_d_n7, assign12140_e6698_d_n8, assign12140_e6698_d_n9, assign12140_e6698_d_n10, assign12140_e6698_d_n11, assign12140_e6698_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12140_e6683: f64 = (locals.var_t3 * locals.var_t3);
        let assign12140_e6686: f64 = (4.0 * 1e-6);
        let assign12140_e6688: f64 = (assign12140_e6686 / 100.0);
        let assign12140_e6690: f64 = (assign12140_e6688 * 1e-6);
        let assign12140_e6692: f64 = (assign12140_e6690 / 100.0);
        let assign12140_e6693: f64 = (assign12140_e6683 + assign12140_e6692);
        let assign12140_e6694: f64 = (assign12140_e6693).sqrt();
        let assign12140_e6695: f64 = (locals.var_t3 + assign12140_e6694);
        let assign12140_e6696: f64 = (0.5 * assign12140_e6695);
        (assign12140_e6696, (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign12140_e6694)))),)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12140_e6698;
        locals.var_rd23e_dn0 = assign12140_e6698_d_n0;
        locals.var_rd23e_dn2 = assign12140_e6698_d_n2;
        locals.var_rd23e_dn4 = assign12140_e6698_d_n4;
        locals.var_rd23e_dn5 = assign12140_e6698_d_n5;
        locals.var_rd23e_dn6 = assign12140_e6698_d_n6;
        locals.var_rd23e_dn7 = assign12140_e6698_d_n7;
        locals.var_rd23e_dn8 = assign12140_e6698_d_n8;
        locals.var_rd23e_dn9 = assign12140_e6698_d_n9;
        locals.var_rd23e_dn10 = assign12140_e6698_d_n10;
        locals.var_rd23e_dn11 = assign12140_e6698_d_n11;
        locals.var_rd23e_dn14 = assign12140_e6698_d_n14;
        locals.var_rd23e_rv = 0.0;

        let (assign12150_e6705, assign12150_e6705_d_n0, assign12150_e6705_d_n2, assign12150_e6705_d_n4, assign12150_e6705_d_n5, assign12150_e6705_d_n6, assign12150_e6705_d_n7, assign12150_e6705_d_n8, assign12150_e6705_d_n9, assign12150_e6705_d_n10, assign12150_e6705_d_n11, assign12150_e6705_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12150_e6705;
        locals.var_rd23e_dn0 = assign12150_e6705_d_n0;
        locals.var_rd23e_dn2 = assign12150_e6705_d_n2;
        locals.var_rd23e_dn4 = assign12150_e6705_d_n4;
        locals.var_rd23e_dn5 = assign12150_e6705_d_n5;
        locals.var_rd23e_dn6 = assign12150_e6705_d_n6;
        locals.var_rd23e_dn7 = assign12150_e6705_d_n7;
        locals.var_rd23e_dn8 = assign12150_e6705_d_n8;
        locals.var_rd23e_dn9 = assign12150_e6705_d_n9;
        locals.var_rd23e_dn10 = assign12150_e6705_d_n10;
        locals.var_rd23e_dn11 = assign12150_e6705_d_n11;
        locals.var_rd23e_dn14 = assign12150_e6705_d_n14;
        locals.var_rd23e_rv = 0.0;

        let (assign12160_e6709,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12160_e6709;
        locals.var_xmax_rv = 0.0;

        let (assign12170_e6713,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12170_e6713;
        locals.var_xmax_s_rv = 0.0;

        let (assign12180_e6717,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12180_e6717;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12190_e6721,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12190_e6721;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12200_e6725,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12200_e6725;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12210_e6729, assign12210_e6729_d_n0, assign12210_e6729_d_n2, assign12210_e6729_d_n4, assign12210_e6729_d_n5, assign12210_e6729_d_n6, assign12210_e6729_d_n7, assign12210_e6729_d_n8, assign12210_e6729_d_n9, assign12210_e6729_d_n10, assign12210_e6729_d_n11, assign12210_e6729_d_n14,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12210_e6729;
        locals.var_rdrmuevbs_dn0 = assign12210_e6729_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12210_e6729_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12210_e6729_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12210_e6729_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12210_e6729_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12210_e6729_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12210_e6729_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12210_e6729_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12210_e6729_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12210_e6729_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12210_e6729_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12220_e6741,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12220_e6734: f64 = (p.p419 * p.p419);
        let assign12220_e6737: f64 = (locals.var_uc_xldld * locals.var_uc_xldld);
        let assign12220_e6738: f64 = (assign12220_e6734 + assign12220_e6737);
        let assign12220_e6739: f64 = (assign12220_e6738).sqrt();
        (assign12220_e6739,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12220_e6741;
        locals.var_xmax_rv = 0.0;

        let (assign12230_e6753,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12230_e6746: f64 = (p.p419 * p.p419);
        let assign12230_e6749: f64 = (p.p97 * p.p97);
        let assign12230_e6750: f64 = (assign12230_e6746 + assign12230_e6749);
        let assign12230_e6751: f64 = (assign12230_e6750).sqrt();
        (assign12230_e6751,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12230_e6753;
        locals.var_xmax_s_rv = 0.0;

        let (assign12240_e6764,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12240_e6760: f64 = (locals.var_wg).powf(p.p425);
        let assign12240_e6761: f64 = (p.p424 / assign12240_e6760);
        let assign12240_e6762: f64 = (1.0 + assign12240_e6761);
        (assign12240_e6762,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12240_e6764;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12250_e6775,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12250_e6771: f64 = (locals.var_lg).powf(p.p427);
        let assign12250_e6772: f64 = (p.p426 / assign12250_e6771);
        let assign12250_e6773: f64 = (1.0 + assign12250_e6772);
        (assign12250_e6773,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12250_e6775;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12260_e6786,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12260_e6782: f64 = (locals.var_lg).powf(p.p429);
        let assign12260_e6783: f64 = (p.p428 / assign12260_e6782);
        let assign12260_e6784: f64 = (1.0 + assign12260_e6783);
        (assign12260_e6784,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12260_e6786;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12270_e6791, assign12270_e6791_d_n0, assign12270_e6791_d_n2, assign12270_e6791_d_n4, assign12270_e6791_d_n5, assign12270_e6791_d_n6, assign12270_e6791_d_n7, assign12270_e6791_d_n8, assign12270_e6791_d_n9, assign12270_e6791_d_n10, assign12270_e6791_d_n11, assign12270_e6791_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12270_e6791;
        locals.var_rdrmuevbs_dn0 = assign12270_e6791_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12270_e6791_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12270_e6791_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12270_e6791_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12270_e6791_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12270_e6791_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12270_e6791_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12270_e6791_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12270_e6791_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12270_e6791_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12270_e6791_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12280_e6796,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12280_e6796;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12290_e6801, assign12290_e6801_d_n0, assign12290_e6801_d_n2, assign12290_e6801_d_n4, assign12290_e6801_d_n5, assign12290_e6801_d_n6, assign12290_e6801_d_n7, assign12290_e6801_d_n8, assign12290_e6801_d_n9, assign12290_e6801_d_n10, assign12290_e6801_d_n11, assign12290_e6801_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12290_e6801;
        locals.var_rdvdtemp0_dn0 = assign12290_e6801_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12290_e6801_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12290_e6801_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12290_e6801_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12290_e6801_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12290_e6801_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12290_e6801_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12290_e6801_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12290_e6801_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12290_e6801_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12290_e6801_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12300_e6806, assign12300_e6806_d_n0, assign12300_e6806_d_n2, assign12300_e6806_d_n4, assign12300_e6806_d_n5, assign12300_e6806_d_n6, assign12300_e6806_d_n7, assign12300_e6806_d_n8, assign12300_e6806_d_n9, assign12300_e6806_d_n10, assign12300_e6806_d_n11, assign12300_e6806_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12300_e6806;
        locals.var_rd23e_dn0 = assign12300_e6806_d_n0;
        locals.var_rd23e_dn2 = assign12300_e6806_d_n2;
        locals.var_rd23e_dn4 = assign12300_e6806_d_n4;
        locals.var_rd23e_dn5 = assign12300_e6806_d_n5;
        locals.var_rd23e_dn6 = assign12300_e6806_d_n6;
        locals.var_rd23e_dn7 = assign12300_e6806_d_n7;
        locals.var_rd23e_dn8 = assign12300_e6806_d_n8;
        locals.var_rd23e_dn9 = assign12300_e6806_d_n9;
        locals.var_rd23e_dn10 = assign12300_e6806_d_n10;
        locals.var_rd23e_dn11 = assign12300_e6806_d_n11;
        locals.var_rd23e_dn14 = assign12300_e6806_d_n14;
        locals.var_rd23e_rv = 0.0;

        let assign12310_e6809: f64 = if locals.var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign12310_e6809;
        locals.var_guard281_rv = 0.0;

        let (assign12320_e6819,) = {
    if (locals.var_guard281 != 0.0) {
        let assign12320_e6813: f64 = (2.0 * 1.034943e-10);
        let assign12320_e6816: f64 = (1.6021918e-19 * locals.var_uc_nover);
        let assign12320_e6817: f64 = (assign12320_e6813 / assign12320_e6816);
        (assign12320_e6817,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12320_e6819;
        locals.var_kdep_rv = 0.0;

        let (assign12330_e6835, assign12330_e6835_d_n0, assign12330_e6835_d_n2, assign12330_e6835_d_n4, assign12330_e6835_d_n5, assign12330_e6835_d_n6, assign12330_e6835_d_n7, assign12330_e6835_d_n8, assign12330_e6835_d_n9, assign12330_e6835_d_n10, assign12330_e6835_d_n11, assign12330_e6835_d_n14,) = {
    if (locals.var_guard281 != 0.0) {
        let assign12330_e6823: f64 = (2.0 * 1.034943e-10);
        let assign12330_e6825: f64 = (assign12330_e6823 / 1.6021918e-19);
        let assign12330_e6827: f64 = (assign12330_e6825 * locals.var_ef_nsubc);
        let assign12330_e6830: f64 = (locals.var_uc_nover + locals.var_ef_nsubc);
        let assign12330_e6831: f64 = (assign12330_e6827 / assign12330_e6830);
        let assign12330_e6833: f64 = (assign12330_e6831 / locals.var_uc_nover);
        (assign12330_e6833, (((((assign12330_e6825 * locals.var_ef_nsubc_dn0) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn0)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn2) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn2)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn4) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn4)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn5) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn5)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn6) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn6)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn7) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn7)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn8) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn8)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn9) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn9)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn10) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn10)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn11) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn11)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn14) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn14)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover),)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12330_e6835;
        locals.var_kjunc_dn0 = assign12330_e6835_d_n0;
        locals.var_kjunc_dn2 = assign12330_e6835_d_n2;
        locals.var_kjunc_dn4 = assign12330_e6835_d_n4;
        locals.var_kjunc_dn5 = assign12330_e6835_d_n5;
        locals.var_kjunc_dn6 = assign12330_e6835_d_n6;
        locals.var_kjunc_dn7 = assign12330_e6835_d_n7;
        locals.var_kjunc_dn8 = assign12330_e6835_d_n8;
        locals.var_kjunc_dn9 = assign12330_e6835_d_n9;
        locals.var_kjunc_dn10 = assign12330_e6835_d_n10;
        locals.var_kjunc_dn11 = assign12330_e6835_d_n11;
        locals.var_kjunc_dn14 = assign12330_e6835_d_n14;
        locals.var_kjunc_rv = 0.0;

        let (assign12340_e6840,) = {
    if (locals.var_guard281 == 0.0) {
        (0.0,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12340_e6840;
        locals.var_kdep_rv = 0.0;

        let (assign12350_e6845, assign12350_e6845_d_n0, assign12350_e6845_d_n2, assign12350_e6845_d_n4, assign12350_e6845_d_n5, assign12350_e6845_d_n6, assign12350_e6845_d_n7, assign12350_e6845_d_n8, assign12350_e6845_d_n9, assign12350_e6845_d_n10, assign12350_e6845_d_n11, assign12350_e6845_d_n14,) = {
    if (locals.var_guard281 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12350_e6845;
        locals.var_kjunc_dn0 = assign12350_e6845_d_n0;
        locals.var_kjunc_dn2 = assign12350_e6845_d_n2;
        locals.var_kjunc_dn4 = assign12350_e6845_d_n4;
        locals.var_kjunc_dn5 = assign12350_e6845_d_n5;
        locals.var_kjunc_dn6 = assign12350_e6845_d_n6;
        locals.var_kjunc_dn7 = assign12350_e6845_d_n7;
        locals.var_kjunc_dn8 = assign12350_e6845_d_n8;
        locals.var_kjunc_dn9 = assign12350_e6845_d_n9;
        locals.var_kjunc_dn10 = assign12350_e6845_d_n10;
        locals.var_kjunc_dn11 = assign12350_e6845_d_n11;
        locals.var_kjunc_dn14 = assign12350_e6845_d_n14;
        locals.var_kjunc_rv = 0.0;

        let assign12490_e6940: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign12490_e6940;
        locals.var_guard286_rv = 0.0;

        let (assign12500_e6948, assign12500_e6948_d_n0, assign12500_e6948_d_n2, assign12500_e6948_d_n4, assign12500_e6948_d_n5, assign12500_e6948_d_n6, assign12500_e6948_d_n7, assign12500_e6948_d_n8, assign12500_e6948_d_n9, assign12500_e6948_d_n10, assign12500_e6948_d_n11, assign12500_e6948_d_n14,) = {
    if (locals.var_guard286 != 0.0) {
        let assign12500_e6944: f64 = (p.p108 * locals.var_lg);
        let assign12500_e6946: f64 = (assign12500_e6944 + p.p109);
        (assign12500_e6946, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12500_e6948;
        locals.var_t1_dn0 = assign12500_e6948_d_n0;
        locals.var_t1_dn2 = assign12500_e6948_d_n2;
        locals.var_t1_dn4 = assign12500_e6948_d_n4;
        locals.var_t1_dn5 = assign12500_e6948_d_n5;
        locals.var_t1_dn6 = assign12500_e6948_d_n6;
        locals.var_t1_dn7 = assign12500_e6948_d_n7;
        locals.var_t1_dn8 = assign12500_e6948_d_n8;
        locals.var_t1_dn9 = assign12500_e6948_d_n9;
        locals.var_t1_dn10 = assign12500_e6948_d_n10;
        locals.var_t1_dn11 = assign12500_e6948_d_n11;
        locals.var_t1_dn14 = assign12500_e6948_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12510_e6951: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign12510_e6951;
        locals.var_guard287_rv = 0.0;

        let (assign12520_e6957, assign12520_e6957_d_n0, assign12520_e6957_d_n2, assign12520_e6957_d_n4, assign12520_e6957_d_n5, assign12520_e6957_d_n6, assign12520_e6957_d_n7, assign12520_e6957_d_n8, assign12520_e6957_d_n9, assign12520_e6957_d_n10, assign12520_e6957_d_n11, assign12520_e6957_d_n14,) = {
    if ((locals.var_guard286 != 0.0) && (locals.var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12520_e6957;
        locals.var_t1_dn0 = assign12520_e6957_d_n0;
        locals.var_t1_dn2 = assign12520_e6957_d_n2;
        locals.var_t1_dn4 = assign12520_e6957_d_n4;
        locals.var_t1_dn5 = assign12520_e6957_d_n5;
        locals.var_t1_dn6 = assign12520_e6957_d_n6;
        locals.var_t1_dn7 = assign12520_e6957_d_n7;
        locals.var_t1_dn8 = assign12520_e6957_d_n8;
        locals.var_t1_dn9 = assign12520_e6957_d_n9;
        locals.var_t1_dn10 = assign12520_e6957_d_n10;
        locals.var_t1_dn11 = assign12520_e6957_d_n11;
        locals.var_t1_dn14 = assign12520_e6957_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign12530_e6969, assign12530_e6969_d_n0, assign12530_e6969_d_n2, assign12530_e6969_d_n4, assign12530_e6969_d_n5, assign12530_e6969_d_n6, assign12530_e6969_d_n7, assign12530_e6969_d_n8, assign12530_e6969_d_n9, assign12530_e6969_d_n10, assign12530_e6969_d_n11, assign12530_e6969_d_n14,) = {
    if (locals.var_guard286 != 0.0) {
        let assign12530_e6961: f64 = (locals.var_t1 * p.p107);
        let assign12530_e6964: f64 = (locals.var_t1 + p.p107);
        let assign12530_e6965: f64 = (assign12530_e6961 / assign12530_e6964);
        let assign12530_e6967: f64 = (assign12530_e6965 + 1.0);
        (assign12530_e6967, ((((locals.var_t1_dn0 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn0)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn2 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn2)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn4 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn4)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn5 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn5)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn6 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn6)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn7 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn7)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn8 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn8)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn9 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn9)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn10 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn10)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn11 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn11)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn14 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn14)) / (assign12530_e6964 * assign12530_e6964)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12530_e6969;
        locals.var_ddlte_dn0 = assign12530_e6969_d_n0;
        locals.var_ddlte_dn2 = assign12530_e6969_d_n2;
        locals.var_ddlte_dn4 = assign12530_e6969_d_n4;
        locals.var_ddlte_dn5 = assign12530_e6969_d_n5;
        locals.var_ddlte_dn6 = assign12530_e6969_d_n6;
        locals.var_ddlte_dn7 = assign12530_e6969_d_n7;
        locals.var_ddlte_dn8 = assign12530_e6969_d_n8;
        locals.var_ddlte_dn9 = assign12530_e6969_d_n9;
        locals.var_ddlte_dn10 = assign12530_e6969_d_n10;
        locals.var_ddlte_dn11 = assign12530_e6969_d_n11;
        locals.var_ddlte_dn14 = assign12530_e6969_d_n14;
        locals.var_ddlte_rv = 0.0;

        let (assign12540_e6976, assign12540_e6976_d_n0, assign12540_e6976_d_n2, assign12540_e6976_d_n4, assign12540_e6976_d_n5, assign12540_e6976_d_n6, assign12540_e6976_d_n7, assign12540_e6976_d_n8, assign12540_e6976_d_n9, assign12540_e6976_d_n10, assign12540_e6976_d_n11, assign12540_e6976_d_n14,) = {
    if (locals.var_guard286 == 0.0) {
        let assign12540_e6974: f64 = (p.p108 * locals.var_lg);
        (assign12540_e6974, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12540_e6976;
        locals.var_t1_dn0 = assign12540_e6976_d_n0;
        locals.var_t1_dn2 = assign12540_e6976_d_n2;
        locals.var_t1_dn4 = assign12540_e6976_d_n4;
        locals.var_t1_dn5 = assign12540_e6976_d_n5;
        locals.var_t1_dn6 = assign12540_e6976_d_n6;
        locals.var_t1_dn7 = assign12540_e6976_d_n7;
        locals.var_t1_dn8 = assign12540_e6976_d_n8;
        locals.var_t1_dn9 = assign12540_e6976_d_n9;
        locals.var_t1_dn10 = assign12540_e6976_d_n10;
        locals.var_t1_dn11 = assign12540_e6976_d_n11;
        locals.var_t1_dn14 = assign12540_e6976_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12550_e6979: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign12550_e6979;
        locals.var_guard288_rv = 0.0;

        let (assign12560_e6986, assign12560_e6986_d_n0, assign12560_e6986_d_n2, assign12560_e6986_d_n4, assign12560_e6986_d_n5, assign12560_e6986_d_n6, assign12560_e6986_d_n7, assign12560_e6986_d_n8, assign12560_e6986_d_n9, assign12560_e6986_d_n10, assign12560_e6986_d_n11, assign12560_e6986_d_n14,) = {
    if ((locals.var_guard286 == 0.0) && (locals.var_guard288 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12560_e6986;
        locals.var_t1_dn0 = assign12560_e6986_d_n0;
        locals.var_t1_dn2 = assign12560_e6986_d_n2;
        locals.var_t1_dn4 = assign12560_e6986_d_n4;
        locals.var_t1_dn5 = assign12560_e6986_d_n5;
        locals.var_t1_dn6 = assign12560_e6986_d_n6;
        locals.var_t1_dn7 = assign12560_e6986_d_n7;
        locals.var_t1_dn8 = assign12560_e6986_d_n8;
        locals.var_t1_dn9 = assign12560_e6986_d_n9;
        locals.var_t1_dn10 = assign12560_e6986_d_n10;
        locals.var_t1_dn11 = assign12560_e6986_d_n11;
        locals.var_t1_dn14 = assign12560_e6986_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12570_e7001, assign12570_e7001_d_n0, assign12570_e7001_d_n2, assign12570_e7001_d_n4, assign12570_e7001_d_n5, assign12570_e7001_d_n6, assign12570_e7001_d_n7, assign12570_e7001_d_n8, assign12570_e7001_d_n9, assign12570_e7001_d_n10, assign12570_e7001_d_n11, assign12570_e7001_d_n14,) = {
    if (locals.var_guard286 == 0.0) {
        let assign12570_e6991: f64 = (locals.var_t1 * p.p107);
        let assign12570_e6994: f64 = (locals.var_t1 + p.p107);
        let assign12570_e6995: f64 = (assign12570_e6991 / assign12570_e6994);
        let assign12570_e6997: f64 = (assign12570_e6995 + p.p109);
        let assign12570_e6999: f64 = (assign12570_e6997 + 1e-25);
        (assign12570_e6999, ((((locals.var_t1_dn0 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn0)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn2 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn2)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn4 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn4)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn5 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn5)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn6 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn6)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn7 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn7)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn8 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn8)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn9 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn9)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn10 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn10)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn11 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn11)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn14 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn14)) / (assign12570_e6994 * assign12570_e6994)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12570_e7001;
        locals.var_ddlte_dn0 = assign12570_e7001_d_n0;
        locals.var_ddlte_dn2 = assign12570_e7001_d_n2;
        locals.var_ddlte_dn4 = assign12570_e7001_d_n4;
        locals.var_ddlte_dn5 = assign12570_e7001_d_n5;
        locals.var_ddlte_dn6 = assign12570_e7001_d_n6;
        locals.var_ddlte_dn7 = assign12570_e7001_d_n7;
        locals.var_ddlte_dn8 = assign12570_e7001_d_n8;
        locals.var_ddlte_dn9 = assign12570_e7001_d_n9;
        locals.var_ddlte_dn10 = assign12570_e7001_d_n10;
        locals.var_ddlte_dn11 = assign12570_e7001_d_n11;
        locals.var_ddlte_dn14 = assign12570_e7001_d_n14;
        locals.var_ddlte_rv = 0.0;

        let assign12590_e7009: f64 = if locals.var_ddlte < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign12590_e7009;
        locals.var_guard290_rv = 0.0;

        let (assign12600_e7013, assign12600_e7013_d_n0, assign12600_e7013_d_n2, assign12600_e7013_d_n4, assign12600_e7013_d_n5, assign12600_e7013_d_n6, assign12600_e7013_d_n7, assign12600_e7013_d_n8, assign12600_e7013_d_n9, assign12600_e7013_d_n10, assign12600_e7013_d_n11, assign12600_e7013_d_n14,) = {
    if (locals.var_guard290 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12600_e7013;
        locals.var_ddlte_dn0 = assign12600_e7013_d_n0;
        locals.var_ddlte_dn2 = assign12600_e7013_d_n2;
        locals.var_ddlte_dn4 = assign12600_e7013_d_n4;
        locals.var_ddlte_dn5 = assign12600_e7013_d_n5;
        locals.var_ddlte_dn6 = assign12600_e7013_d_n6;
        locals.var_ddlte_dn7 = assign12600_e7013_d_n7;
        locals.var_ddlte_dn8 = assign12600_e7013_d_n8;
        locals.var_ddlte_dn9 = assign12600_e7013_d_n9;
        locals.var_ddlte_dn10 = assign12600_e7013_d_n10;
        locals.var_ddlte_dn11 = assign12600_e7013_d_n11;
        locals.var_ddlte_dn14 = assign12600_e7013_d_n14;
        locals.var_ddlte_rv = 0.0;

        let (assign12610_e7019, assign12610_e7019_d_n0, assign12610_e7019_d_n2, assign12610_e7019_d_n4, assign12610_e7019_d_n5, assign12610_e7019_d_n6, assign12610_e7019_d_n7, assign12610_e7019_d_n8, assign12610_e7019_d_n9, assign12610_e7019_d_n10, assign12610_e7019_d_n11, assign12610_e7019_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7017: f64 = (locals.var_weff).powf(p.p201);
        (assign12610_e7017, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12610_e7019;
        locals.var_t2_dn0 = assign12610_e7019_d_n0;
        locals.var_t2_dn2 = assign12610_e7019_d_n2;
        locals.var_t2_dn4 = assign12610_e7019_d_n4;
        locals.var_t2_dn5 = assign12610_e7019_d_n5;
        locals.var_t2_dn6 = assign12610_e7019_d_n6;
        locals.var_t2_dn7 = assign12610_e7019_d_n7;
        locals.var_t2_dn8 = assign12610_e7019_d_n8;
        locals.var_t2_dn9 = assign12610_e7019_d_n9;
        locals.var_t2_dn10 = assign12610_e7019_d_n10;
        locals.var_t2_dn11 = assign12610_e7019_d_n11;
        locals.var_t2_dn14 = assign12610_e7019_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign12620_e7037, assign12620_e7037_d_n0, assign12620_e7037_d_n2, assign12620_e7037_d_n4, assign12620_e7037_d_n5, assign12620_e7037_d_n6, assign12620_e7037_d_n7, assign12620_e7037_d_n8, assign12620_e7037_d_n9, assign12620_e7037_d_n10, assign12620_e7037_d_n11, assign12620_e7037_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7026: f64 = (locals.var_lgate).powf(p.p199);
        let assign12620_e7027: f64 = (locals.var_mks_svgsl / assign12620_e7026);
        let assign12620_e7028: f64 = (1.0 + assign12620_e7027);
        let assign12620_e7029: f64 = (locals.var_uc_svgs * assign12620_e7028);
        let assign12620_e7033: f64 = (locals.var_t2 + locals.var_mks_svgsw);
        let assign12620_e7034: f64 = (locals.var_t2 / assign12620_e7033);
        let assign12620_e7035: f64 = (assign12620_e7029 * assign12620_e7034);
        (assign12620_e7035, (assign12620_e7029 * (((locals.var_t2_dn0 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn0)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn2 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn2)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn4 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn4)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn5 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn5)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn6 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn6)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn7 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn7)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn8 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn8)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn9 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn9)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn10 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn10)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn11 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn11)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn14 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn14)) / (assign12620_e7033 * assign12620_e7033))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12620_e7037;
        locals.var_vg2const_dn0 = assign12620_e7037_d_n0;
        locals.var_vg2const_dn2 = assign12620_e7037_d_n2;
        locals.var_vg2const_dn4 = assign12620_e7037_d_n4;
        locals.var_vg2const_dn5 = assign12620_e7037_d_n5;
        locals.var_vg2const_dn6 = assign12620_e7037_d_n6;
        locals.var_vg2const_dn7 = assign12620_e7037_d_n7;
        locals.var_vg2const_dn8 = assign12620_e7037_d_n8;
        locals.var_vg2const_dn9 = assign12620_e7037_d_n9;
        locals.var_vg2const_dn10 = assign12620_e7037_d_n10;
        locals.var_vg2const_dn11 = assign12620_e7037_d_n11;
        locals.var_vg2const_dn14 = assign12620_e7037_d_n14;
        locals.var_vg2const_rv = 0.0;

        let (assign12630_e7049,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7044: f64 = (locals.var_lgate).powf(p.p184);
        let assign12630_e7045: f64 = (locals.var_mks_svbsl / assign12630_e7044);
        let assign12630_e7046: f64 = (1.0 + assign12630_e7045);
        let assign12630_e7047: f64 = (locals.var_uc_svbs * assign12630_e7046);
        (assign12630_e7047,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12630_e7049;
        locals.var_xvbs_rv = 0.0;

        let (assign12640_e7061,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7056: f64 = (locals.var_lgate).powf(p.p203);
        let assign12640_e7057: f64 = (locals.var_mks_slgl / assign12640_e7056);
        let assign12640_e7058: f64 = (1.0 + assign12640_e7057);
        let assign12640_e7059: f64 = (locals.var_mks_slg * assign12640_e7058);
        (assign12640_e7059,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12640_e7061;
        locals.var_xgate_rv = 0.0;

        let (assign12650_e7073,) = {
    if (p.p23 != 0.0) {
        let assign12650_e7068: f64 = (locals.var_lgate).powf(p.p191);
        let assign12650_e7069: f64 = (locals.var_mks_sub1l / assign12650_e7068);
        let assign12650_e7070: f64 = (1.0 + assign12650_e7069);
        let assign12650_e7071: f64 = (locals.var_uc_sub1 * assign12650_e7070);
        (assign12650_e7071,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12650_e7073;
        locals.var_xsub1_rv = 0.0;

        let (assign12660_e7083,) = {
    if (p.p23 != 0.0) {
        let assign12660_e7079: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12660_e7080: f64 = (1.0 + assign12660_e7079);
        let assign12660_e7081: f64 = (locals.var_uc_sub2 * assign12660_e7080);
        (assign12660_e7081,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12660_e7083;
        locals.var_xsub2_rv = 0.0;

        let (assign12670_e7087,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub1,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12670_e7087;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12680_e7091,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub2,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12680_e7091;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12690_e7095, assign12690_e7095_d_n0, assign12690_e7095_d_n2, assign12690_e7095_d_n4, assign12690_e7095_d_n5, assign12690_e7095_d_n6, assign12690_e7095_d_n7, assign12690_e7095_d_n8, assign12690_e7095_d_n9, assign12690_e7095_d_n10, assign12690_e7095_d_n11, assign12690_e7095_d_n14,) = {
    if (p.p23 != 0.0) {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12690_e7095;
        locals.var_vg2const_1_dn0 = assign12690_e7095_d_n0;
        locals.var_vg2const_1_dn2 = assign12690_e7095_d_n2;
        locals.var_vg2const_1_dn4 = assign12690_e7095_d_n4;
        locals.var_vg2const_1_dn5 = assign12690_e7095_d_n5;
        locals.var_vg2const_1_dn6 = assign12690_e7095_d_n6;
        locals.var_vg2const_1_dn7 = assign12690_e7095_d_n7;
        locals.var_vg2const_1_dn8 = assign12690_e7095_d_n8;
        locals.var_vg2const_1_dn9 = assign12690_e7095_d_n9;
        locals.var_vg2const_1_dn10 = assign12690_e7095_d_n10;
        locals.var_vg2const_1_dn11 = assign12690_e7095_d_n11;
        locals.var_vg2const_1_dn14 = assign12690_e7095_d_n14;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12700_e7099,) = {
    if (p.p23 != 0.0) {
        (locals.var_xvbs,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12700_e7099;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12710_e7103,) = {
    if (p.p23 != 0.0) {
        (locals.var_xgate,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12710_e7103;
        locals.var_xgate_1_rv = 0.0;

        let (assign12720_e7117,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12720_e7112: f64 = (locals.var_lgate).powf(p.p191);
        let assign12720_e7113: f64 = (locals.var_mks_sub1l / assign12720_e7112);
        let assign12720_e7114: f64 = (1.0 + assign12720_e7113);
        let assign12720_e7115: f64 = (locals.var_uc_sub1snp * assign12720_e7114);
        (assign12720_e7115,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12720_e7117;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12730_e7129,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12730_e7125: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12730_e7126: f64 = (1.0 + assign12730_e7125);
        let assign12730_e7127: f64 = (locals.var_uc_sub2snp * assign12730_e7126);
        (assign12730_e7127,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12730_e7129;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12740_e7141,) = {
    if (p.p23 != 0.0) {
        let assign12740_e7136: f64 = (locals.var_lg).powf(p.p103);
        let assign12740_e7137: f64 = (p.p102 / assign12740_e7136);
        let assign12740_e7138: f64 = (1.0 + assign12740_e7137);
        let assign12740_e7139: f64 = (p.p72 * assign12740_e7138);
        (assign12740_e7139,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12740_e7141;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12750_e7146, assign12750_e7146_d_n0, assign12750_e7146_d_n2, assign12750_e7146_d_n4, assign12750_e7146_d_n5, assign12750_e7146_d_n6, assign12750_e7146_d_n7, assign12750_e7146_d_n8, assign12750_e7146_d_n9, assign12750_e7146_d_n10, assign12750_e7146_d_n11, assign12750_e7146_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12750_e7146;
        locals.var_vg2const_dn0 = assign12750_e7146_d_n0;
        locals.var_vg2const_dn2 = assign12750_e7146_d_n2;
        locals.var_vg2const_dn4 = assign12750_e7146_d_n4;
        locals.var_vg2const_dn5 = assign12750_e7146_d_n5;
        locals.var_vg2const_dn6 = assign12750_e7146_d_n6;
        locals.var_vg2const_dn7 = assign12750_e7146_d_n7;
        locals.var_vg2const_dn8 = assign12750_e7146_d_n8;
        locals.var_vg2const_dn9 = assign12750_e7146_d_n9;
        locals.var_vg2const_dn10 = assign12750_e7146_d_n10;
        locals.var_vg2const_dn11 = assign12750_e7146_d_n11;
        locals.var_vg2const_dn14 = assign12750_e7146_d_n14;
        locals.var_vg2const_rv = 0.0;

        let (assign12760_e7151,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12760_e7151;
        locals.var_xvbs_rv = 0.0;

        let (assign12770_e7156,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12770_e7156;
        locals.var_xgate_rv = 0.0;

        let (assign12780_e7161,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12780_e7161;
        locals.var_xsub1_rv = 0.0;

        let (assign12790_e7166,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12790_e7166;
        locals.var_xsub2_rv = 0.0;

        let (assign12800_e7171,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12800_e7171;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12810_e7176, assign12810_e7176_d_n0, assign12810_e7176_d_n2, assign12810_e7176_d_n4, assign12810_e7176_d_n5, assign12810_e7176_d_n6, assign12810_e7176_d_n7, assign12810_e7176_d_n8, assign12810_e7176_d_n9, assign12810_e7176_d_n10, assign12810_e7176_d_n11, assign12810_e7176_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12810_e7176;
        locals.var_vg2const_1_dn0 = assign12810_e7176_d_n0;
        locals.var_vg2const_1_dn2 = assign12810_e7176_d_n2;
        locals.var_vg2const_1_dn4 = assign12810_e7176_d_n4;
        locals.var_vg2const_1_dn5 = assign12810_e7176_d_n5;
        locals.var_vg2const_1_dn6 = assign12810_e7176_d_n6;
        locals.var_vg2const_1_dn7 = assign12810_e7176_d_n7;
        locals.var_vg2const_1_dn8 = assign12810_e7176_d_n8;
        locals.var_vg2const_1_dn9 = assign12810_e7176_d_n9;
        locals.var_vg2const_1_dn10 = assign12810_e7176_d_n10;
        locals.var_vg2const_1_dn11 = assign12810_e7176_d_n11;
        locals.var_vg2const_1_dn14 = assign12810_e7176_d_n14;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12820_e7181,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12820_e7181;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12830_e7186,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12830_e7186;
        locals.var_xgate_1_rv = 0.0;

        let (assign12840_e7191,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12840_e7191;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12850_e7196,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12850_e7196;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12860_e7210,) = {
    if (locals.var_uc_ibpc1 != 0.0) {
        let assign12860_e7205: f64 = (locals.var_lg).powf(p.p280);
        let assign12860_e7206: f64 = (p.p279 / assign12860_e7205);
        let assign12860_e7207: f64 = (1.0 + assign12860_e7206);
        let assign12860_e7208: f64 = (locals.var_uc_ibpc1 * assign12860_e7207);
        (assign12860_e7208,)
    } else {
        (0.0,)
    }
};
        locals.var_uc_ibpc1 = assign12860_e7210;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign12870_e7214: f64 = (3.141592653589793 / 2.0);
        let assign12870_e7215: f64 = (3.453133e-11 / assign12870_e7214);
        let assign12870_e7217: f64 = (assign12870_e7215 * locals.var_weffcv_nf);
        let assign12870_e7221: f64 = (p.p225 / p.p95);
        let assign12870_e7222: f64 = (1.0 + assign12870_e7221);
        let assign12870_e7223: f64 = (assign12870_e7222).ln();
        let assign12870_e7224: f64 = (assign12870_e7217 * assign12870_e7223);
        locals.var_cfrng = assign12870_e7224;
        locals.var_cfrng_rv = 0.0;

        let (assign12880_e7238,) = {
    if (p.p134 != 0.0) {
        let assign12880_e7230: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign12880_e7232: f64 = (assign12880_e7230 * p.p134);
        let assign12880_e7235: f64 = (locals.var_lg).powf(p.p135);
        let assign12880_e7236: f64 = (assign12880_e7232 / assign12880_e7235);
        (assign12880_e7236,)
    } else {
        (0.0,)
    }
};
        locals.var_cqyb0 = assign12880_e7238;
        locals.var_cqyb0_rv = 0.0;

        let assign12890_e7242: f64 = (-p.p286);
        let assign12890_e7243: f64 = (locals.var_lg).powf(assign12890_e7242);
        let assign12890_e7244: f64 = (p.p283 * assign12890_e7243);
        locals.var_ptl0 = assign12890_e7244;
        locals.var_ptl0_rv = 0.0;

        let assign12900_e7248: f64 = (-p.p291);
        let assign12900_e7249: f64 = (locals.var_lg).powf(assign12900_e7248);
        let assign12900_e7250: f64 = (p.p290 * assign12900_e7249);
        locals.var_pt40 = assign12900_e7250;
        locals.var_pt40_rv = 0.0;

        let assign12910_e7254: f64 = (locals.var_lg + locals.var_uc_gdld);
        let assign12910_e7256: f64 = (-p.p288);
        let assign12910_e7257: f64 = (assign12910_e7254).powf(assign12910_e7256);
        let assign12910_e7258: f64 = (p.p287 * assign12910_e7257);
        locals.var_gdl0 = assign12910_e7258;
        locals.var_gdl0_rv = 0.0;

        let assign12920_e7262: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12920_e7263: f64 = (locals.var_uc_rth0 / assign12920_e7262);
        let assign12920_e7268: f64 = (locals.var_lg).powf(p.p318);
        let assign12920_e7269: f64 = (p.p317 / assign12920_e7268);
        let assign12920_e7270: f64 = (1.0 + assign12920_e7269);
        let assign12920_e7271: f64 = (assign12920_e7263 * assign12920_e7270);
        let assign12920_e7276: f64 = (locals.var_wg).powf(p.p316);
        let assign12920_e7277: f64 = (p.p315 / assign12920_e7276);
        let assign12920_e7278: f64 = (1.0 + assign12920_e7277);
        let assign12920_e7279: f64 = (assign12920_e7271 * assign12920_e7278);
        locals.var_rth = assign12920_e7279;
        locals.var_rth_dn0 = 0.0;
        locals.var_rth_dn2 = 0.0;
        locals.var_rth_dn4 = 0.0;
        locals.var_rth_dn5 = 0.0;
        locals.var_rth_dn6 = 0.0;
        locals.var_rth_dn7 = 0.0;
        locals.var_rth_dn8 = 0.0;
        locals.var_rth_dn9 = 0.0;
        locals.var_rth_dn10 = 0.0;
        locals.var_rth_dn11 = 0.0;
        locals.var_rth_dn14 = 0.0;
        locals.var_rth_rv = 0.0;

        let assign12940_e7289: f64 = (p.p7).powf(p.p327);
        let assign12940_e7290: f64 = (1.0 / assign12940_e7289);
        let assign12940_e7291: f64 = (locals.var_rth * assign12940_e7290);
        locals.var_rth = assign12940_e7291;
        locals.var_rth_dn0 = (locals.var_rth_dn0 * assign12940_e7290);
        locals.var_rth_dn2 = (locals.var_rth_dn2 * assign12940_e7290);
        locals.var_rth_dn4 = (locals.var_rth_dn4 * assign12940_e7290);
        locals.var_rth_dn5 = (locals.var_rth_dn5 * assign12940_e7290);
        locals.var_rth_dn6 = (locals.var_rth_dn6 * assign12940_e7290);
        locals.var_rth_dn7 = (locals.var_rth_dn7 * assign12940_e7290);
        locals.var_rth_dn8 = (locals.var_rth_dn8 * assign12940_e7290);
        locals.var_rth_dn9 = (locals.var_rth_dn9 * assign12940_e7290);
        locals.var_rth_dn10 = (locals.var_rth_dn10 * assign12940_e7290);
        locals.var_rth_dn11 = (locals.var_rth_dn11 * assign12940_e7290);
        locals.var_rth_dn14 = (locals.var_rth_dn14 * assign12940_e7290);
        locals.var_rth_rv = 0.0;

        let assign12950_e7295: f64 = (p.p7).powf(p.p327);
        let assign12950_e7296: f64 = (1.0 / assign12950_e7295);
        let assign12950_e7299: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12950_e7300: f64 = (assign12950_e7296 / assign12950_e7299);
        let assign12950_e7305: f64 = (locals.var_lg).powf(p.p318);
        let assign12950_e7306: f64 = (p.p317 / assign12950_e7305);
        let assign12950_e7307: f64 = (1.0 + assign12950_e7306);
        let assign12950_e7308: f64 = (assign12950_e7300 * assign12950_e7307);
        let assign12950_e7313: f64 = (locals.var_wg).powf(p.p316);
        let assign12950_e7314: f64 = (p.p315 / assign12950_e7313);
        let assign12950_e7315: f64 = (1.0 + assign12950_e7314);
        let assign12950_e7316: f64 = (assign12950_e7308 * assign12950_e7315);
        locals.var_rthtemp0 = assign12950_e7316;
        locals.var_rthtemp0_rv = 0.0;

        let assign12960_e7323: f64 = if ((p.p53 == 0.0) || (locals.var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard291 = assign12960_e7323;
        locals.var_guard291_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign12970_e7327, assign12970_e7327_d_n0, assign12970_e7327_d_n2, assign12970_e7327_d_n4, assign12970_e7327_d_n5, assign12970_e7327_d_n6, assign12970_e7327_d_n7, assign12970_e7327_d_n8, assign12970_e7327_d_n9, assign12970_e7327_d_n10, assign12970_e7327_d_n11, assign12970_e7327_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign12970_e7327;
        locals.var_cnst0over_dn0 = assign12970_e7327_d_n0;
        locals.var_cnst0over_dn2 = assign12970_e7327_d_n2;
        locals.var_cnst0over_dn4 = assign12970_e7327_d_n4;
        locals.var_cnst0over_dn5 = assign12970_e7327_d_n5;
        locals.var_cnst0over_dn6 = assign12970_e7327_d_n6;
        locals.var_cnst0over_dn7 = assign12970_e7327_d_n7;
        locals.var_cnst0over_dn8 = assign12970_e7327_d_n8;
        locals.var_cnst0over_dn9 = assign12970_e7327_d_n9;
        locals.var_cnst0over_dn10 = assign12970_e7327_d_n10;
        locals.var_cnst0over_dn11 = assign12970_e7327_d_n11;
        locals.var_cnst0over_dn14 = assign12970_e7327_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let (assign12980_e7331, assign12980_e7331_d_n0, assign12980_e7331_d_n2, assign12980_e7331_d_n4, assign12980_e7331_d_n5, assign12980_e7331_d_n6, assign12980_e7331_d_n7, assign12980_e7331_d_n8, assign12980_e7331_d_n9, assign12980_e7331_d_n10, assign12980_e7331_d_n11, assign12980_e7331_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign12980_e7331;
        locals.var_cnst0overs_dn0 = assign12980_e7331_d_n0;
        locals.var_cnst0overs_dn2 = assign12980_e7331_d_n2;
        locals.var_cnst0overs_dn4 = assign12980_e7331_d_n4;
        locals.var_cnst0overs_dn5 = assign12980_e7331_d_n5;
        locals.var_cnst0overs_dn6 = assign12980_e7331_d_n6;
        locals.var_cnst0overs_dn7 = assign12980_e7331_d_n7;
        locals.var_cnst0overs_dn8 = assign12980_e7331_d_n8;
        locals.var_cnst0overs_dn9 = assign12980_e7331_d_n9;
        locals.var_cnst0overs_dn10 = assign12980_e7331_d_n10;
        locals.var_cnst0overs_dn11 = assign12980_e7331_d_n11;
        locals.var_cnst0overs_dn14 = assign12980_e7331_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let (assign12990_e7337, assign12990_e7337_d_n0, assign12990_e7337_d_n2, assign12990_e7337_d_n4, assign12990_e7337_d_n5, assign12990_e7337_d_n6, assign12990_e7337_d_n7, assign12990_e7337_d_n8, assign12990_e7337_d_n9, assign12990_e7337_d_n10, assign12990_e7337_d_n11, assign12990_e7337_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign12990_e7333: f64 = ctx_temp;
        let assign12990_e7335: f64 = (assign12990_e7333 + p.p11);
        (assign12990_e7335, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign12990_e7337;
        locals.var_ttemp_dn0 = assign12990_e7337_d_n0;
        locals.var_ttemp_dn2 = assign12990_e7337_d_n2;
        locals.var_ttemp_dn4 = assign12990_e7337_d_n4;
        locals.var_ttemp_dn5 = assign12990_e7337_d_n5;
        locals.var_ttemp_dn6 = assign12990_e7337_d_n6;
        locals.var_ttemp_dn7 = assign12990_e7337_d_n7;
        locals.var_ttemp_dn8 = assign12990_e7337_d_n8;
        locals.var_ttemp_dn9 = assign12990_e7337_d_n9;
        locals.var_ttemp_dn10 = assign12990_e7337_d_n10;
        locals.var_ttemp_dn11 = assign12990_e7337_d_n11;
        locals.var_ttemp_dn14 = assign12990_e7337_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign13000_e7341, assign13000_e7341_d_n0, assign13000_e7341_d_n2, assign13000_e7341_d_n4, assign13000_e7341_d_n5, assign13000_e7341_d_n6, assign13000_e7341_d_n7, assign13000_e7341_d_n8, assign13000_e7341_d_n9, assign13000_e7341_d_n10, assign13000_e7341_d_n11, assign13000_e7341_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    }
};
        locals.var_ttemp0 = assign13000_e7341;
        locals.var_ttemp0_dn0 = assign13000_e7341_d_n0;
        locals.var_ttemp0_dn2 = assign13000_e7341_d_n2;
        locals.var_ttemp0_dn4 = assign13000_e7341_d_n4;
        locals.var_ttemp0_dn5 = assign13000_e7341_d_n5;
        locals.var_ttemp0_dn6 = assign13000_e7341_d_n6;
        locals.var_ttemp0_dn7 = assign13000_e7341_d_n7;
        locals.var_ttemp0_dn8 = assign13000_e7341_d_n8;
        locals.var_ttemp0_dn9 = assign13000_e7341_d_n9;
        locals.var_ttemp0_dn10 = assign13000_e7341_d_n10;
        locals.var_ttemp0_dn11 = assign13000_e7341_d_n11;
        locals.var_ttemp0_dn14 = assign13000_e7341_d_n14;
        locals.var_ttemp0_rv = 0.0;

        let (assign13010_e7347, assign13010_e7347_d_n0, assign13010_e7347_d_n2, assign13010_e7347_d_n4, assign13010_e7347_d_n5, assign13010_e7347_d_n6, assign13010_e7347_d_n7, assign13010_e7347_d_n8, assign13010_e7347_d_n9, assign13010_e7347_d_n10, assign13010_e7347_d_n11, assign13010_e7347_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13010_e7345: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign13010_e7345, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn11 + locals.var_deltemp_dn11), (locals.var_ttemp_dn14 + locals.var_deltemp_dn14),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign13010_e7347;
        locals.var_ttemp_dn0 = assign13010_e7347_d_n0;
        locals.var_ttemp_dn2 = assign13010_e7347_d_n2;
        locals.var_ttemp_dn4 = assign13010_e7347_d_n4;
        locals.var_ttemp_dn5 = assign13010_e7347_d_n5;
        locals.var_ttemp_dn6 = assign13010_e7347_d_n6;
        locals.var_ttemp_dn7 = assign13010_e7347_d_n7;
        locals.var_ttemp_dn8 = assign13010_e7347_d_n8;
        locals.var_ttemp_dn9 = assign13010_e7347_d_n9;
        locals.var_ttemp_dn10 = assign13010_e7347_d_n10;
        locals.var_ttemp_dn11 = assign13010_e7347_d_n11;
        locals.var_ttemp_dn14 = assign13010_e7347_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign13020_e7353, assign13020_e7353_d_n0, assign13020_e7353_d_n2, assign13020_e7353_d_n4, assign13020_e7353_d_n5, assign13020_e7353_d_n6, assign13020_e7353_d_n7, assign13020_e7353_d_n8, assign13020_e7353_d_n9, assign13020_e7353_d_n10, assign13020_e7353_d_n11, assign13020_e7353_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13020_e7351: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign13020_e7351, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn11, locals.var_tdiff0_dn14,)
    }
};
        locals.var_tdiff0 = assign13020_e7353;
        locals.var_tdiff0_dn0 = assign13020_e7353_d_n0;
        locals.var_tdiff0_dn2 = assign13020_e7353_d_n2;
        locals.var_tdiff0_dn4 = assign13020_e7353_d_n4;
        locals.var_tdiff0_dn5 = assign13020_e7353_d_n5;
        locals.var_tdiff0_dn6 = assign13020_e7353_d_n6;
        locals.var_tdiff0_dn7 = assign13020_e7353_d_n7;
        locals.var_tdiff0_dn8 = assign13020_e7353_d_n8;
        locals.var_tdiff0_dn9 = assign13020_e7353_d_n9;
        locals.var_tdiff0_dn10 = assign13020_e7353_d_n10;
        locals.var_tdiff0_dn11 = assign13020_e7353_d_n11;
        locals.var_tdiff0_dn14 = assign13020_e7353_d_n14;
        locals.var_tdiff0_rv = 0.0;

        let (assign13030_e7363, assign13030_e7363_d_n0, assign13030_e7363_d_n2, assign13030_e7363_d_n4, assign13030_e7363_d_n5, assign13030_e7363_d_n6, assign13030_e7363_d_n7, assign13030_e7363_d_n8, assign13030_e7363_d_n9, assign13030_e7363_d_n10, assign13030_e7363_d_n11, assign13030_e7363_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13030_e7357: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign13030_e7360: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13030_e7361: f64 = (assign13030_e7357 - assign13030_e7360);
        (assign13030_e7361, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn11 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn11)), ((locals.var_ttemp0_dn14 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn14)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn11, locals.var_tdiff0_2_dn14,)
    }
};
        locals.var_tdiff0_2 = assign13030_e7363;
        locals.var_tdiff0_2_dn0 = assign13030_e7363_d_n0;
        locals.var_tdiff0_2_dn2 = assign13030_e7363_d_n2;
        locals.var_tdiff0_2_dn4 = assign13030_e7363_d_n4;
        locals.var_tdiff0_2_dn5 = assign13030_e7363_d_n5;
        locals.var_tdiff0_2_dn6 = assign13030_e7363_d_n6;
        locals.var_tdiff0_2_dn7 = assign13030_e7363_d_n7;
        locals.var_tdiff0_2_dn8 = assign13030_e7363_d_n8;
        locals.var_tdiff0_2_dn9 = assign13030_e7363_d_n9;
        locals.var_tdiff0_2_dn10 = assign13030_e7363_d_n10;
        locals.var_tdiff0_2_dn11 = assign13030_e7363_d_n11;
        locals.var_tdiff0_2_dn14 = assign13030_e7363_d_n14;
        locals.var_tdiff0_2_rv = 0.0;

        let (assign13040_e7369, assign13040_e7369_d_n0, assign13040_e7369_d_n2, assign13040_e7369_d_n4, assign13040_e7369_d_n5, assign13040_e7369_d_n6, assign13040_e7369_d_n7, assign13040_e7369_d_n8, assign13040_e7369_d_n9, assign13040_e7369_d_n10, assign13040_e7369_d_n11, assign13040_e7369_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13040_e7367: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign13040_e7367, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn11, locals.var_tdiff_dn14,)
    }
};
        locals.var_tdiff = assign13040_e7369;
        locals.var_tdiff_dn0 = assign13040_e7369_d_n0;
        locals.var_tdiff_dn2 = assign13040_e7369_d_n2;
        locals.var_tdiff_dn4 = assign13040_e7369_d_n4;
        locals.var_tdiff_dn5 = assign13040_e7369_d_n5;
        locals.var_tdiff_dn6 = assign13040_e7369_d_n6;
        locals.var_tdiff_dn7 = assign13040_e7369_d_n7;
        locals.var_tdiff_dn8 = assign13040_e7369_d_n8;
        locals.var_tdiff_dn9 = assign13040_e7369_d_n9;
        locals.var_tdiff_dn10 = assign13040_e7369_d_n10;
        locals.var_tdiff_dn11 = assign13040_e7369_d_n11;
        locals.var_tdiff_dn14 = assign13040_e7369_d_n14;
        locals.var_tdiff_rv = 0.0;

        let (assign13050_e7379, assign13050_e7379_d_n0, assign13050_e7379_d_n2, assign13050_e7379_d_n4, assign13050_e7379_d_n5, assign13050_e7379_d_n6, assign13050_e7379_d_n7, assign13050_e7379_d_n8, assign13050_e7379_d_n9, assign13050_e7379_d_n10, assign13050_e7379_d_n11, assign13050_e7379_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13050_e7373: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign13050_e7376: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13050_e7377: f64 = (assign13050_e7373 - assign13050_e7376);
        (assign13050_e7377, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn11 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn11)), ((locals.var_ttemp_dn14 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn11, locals.var_tdiff_2_dn14,)
    }
};
        locals.var_tdiff_2 = assign13050_e7379;
        locals.var_tdiff_2_dn0 = assign13050_e7379_d_n0;
        locals.var_tdiff_2_dn2 = assign13050_e7379_d_n2;
        locals.var_tdiff_2_dn4 = assign13050_e7379_d_n4;
        locals.var_tdiff_2_dn5 = assign13050_e7379_d_n5;
        locals.var_tdiff_2_dn6 = assign13050_e7379_d_n6;
        locals.var_tdiff_2_dn7 = assign13050_e7379_d_n7;
        locals.var_tdiff_2_dn8 = assign13050_e7379_d_n8;
        locals.var_tdiff_2_dn9 = assign13050_e7379_d_n9;
        locals.var_tdiff_2_dn10 = assign13050_e7379_d_n10;
        locals.var_tdiff_2_dn11 = assign13050_e7379_d_n11;
        locals.var_tdiff_2_dn14 = assign13050_e7379_d_n14;
        locals.var_tdiff_2_rv = 0.0;

        let (assign13060_e7385, assign13060_e7385_d_n0, assign13060_e7385_d_n2, assign13060_e7385_d_n4, assign13060_e7385_d_n5, assign13060_e7385_d_n6, assign13060_e7385_d_n7, assign13060_e7385_d_n8, assign13060_e7385_d_n9, assign13060_e7385_d_n10, assign13060_e7385_d_n11, assign13060_e7385_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13060_e7383: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13060_e7383, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn11, locals.var_tratio_dn14,)
    }
};
        locals.var_tratio = assign13060_e7385;
        locals.var_tratio_dn0 = assign13060_e7385_d_n0;
        locals.var_tratio_dn2 = assign13060_e7385_d_n2;
        locals.var_tratio_dn4 = assign13060_e7385_d_n4;
        locals.var_tratio_dn5 = assign13060_e7385_d_n5;
        locals.var_tratio_dn6 = assign13060_e7385_d_n6;
        locals.var_tratio_dn7 = assign13060_e7385_d_n7;
        locals.var_tratio_dn8 = assign13060_e7385_d_n8;
        locals.var_tratio_dn9 = assign13060_e7385_d_n9;
        locals.var_tratio_dn10 = assign13060_e7385_d_n10;
        locals.var_tratio_dn11 = assign13060_e7385_d_n11;
        locals.var_tratio_dn14 = assign13060_e7385_d_n14;
        locals.var_tratio_rv = 0.0;

        let (assign13070_e7390, assign13070_e7390_d_n0, assign13070_e7390_d_n2, assign13070_e7390_d_n4, assign13070_e7390_d_n5, assign13070_e7390_d_n6, assign13070_e7390_d_n7, assign13070_e7390_d_n8, assign13070_e7390_d_n9, assign13070_e7390_d_n10, assign13070_e7390_d_n11, assign13070_e7390_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13070_e7388: f64 = (locals.var_tratio).ln();
        (assign13070_e7388, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn11 / locals.var_tratio), (locals.var_tratio_dn14 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn11, locals.var_log_tratio_dn14,)
    }
};
        locals.var_log_tratio = assign13070_e7390;
        locals.var_log_tratio_dn0 = assign13070_e7390_d_n0;
        locals.var_log_tratio_dn2 = assign13070_e7390_d_n2;
        locals.var_log_tratio_dn4 = assign13070_e7390_d_n4;
        locals.var_log_tratio_dn5 = assign13070_e7390_d_n5;
        locals.var_log_tratio_dn6 = assign13070_e7390_d_n6;
        locals.var_log_tratio_dn7 = assign13070_e7390_d_n7;
        locals.var_log_tratio_dn8 = assign13070_e7390_d_n8;
        locals.var_log_tratio_dn9 = assign13070_e7390_d_n9;
        locals.var_log_tratio_dn10 = assign13070_e7390_d_n10;
        locals.var_log_tratio_dn11 = assign13070_e7390_d_n11;
        locals.var_log_tratio_dn14 = assign13070_e7390_d_n14;
        locals.var_log_tratio_rv = 0.0;

        let (assign13080_e7402, assign13080_e7402_d_n0, assign13080_e7402_d_n2, assign13080_e7402_d_n4, assign13080_e7402_d_n5, assign13080_e7402_d_n6, assign13080_e7402_d_n7, assign13080_e7402_d_n8, assign13080_e7402_d_n9, assign13080_e7402_d_n10, assign13080_e7402_d_n11, assign13080_e7402_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13080_e7395: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign13080_e7396: f64 = (locals.var_egtnom - assign13080_e7395);
        let assign13080_e7399: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign13080_e7400: f64 = (assign13080_e7396 - assign13080_e7399);
        (assign13080_e7400, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn11)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn11)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn14)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn11, locals.var_eg_dn14,)
    }
};
        locals.var_eg = assign13080_e7402;
        locals.var_eg_dn0 = assign13080_e7402_d_n0;
        locals.var_eg_dn2 = assign13080_e7402_d_n2;
        locals.var_eg_dn4 = assign13080_e7402_d_n4;
        locals.var_eg_dn5 = assign13080_e7402_d_n5;
        locals.var_eg_dn6 = assign13080_e7402_d_n6;
        locals.var_eg_dn7 = assign13080_e7402_d_n7;
        locals.var_eg_dn8 = assign13080_e7402_d_n8;
        locals.var_eg_dn9 = assign13080_e7402_d_n9;
        locals.var_eg_dn10 = assign13080_e7402_d_n10;
        locals.var_eg_dn11 = assign13080_e7402_d_n11;
        locals.var_eg_dn14 = assign13080_e7402_d_n14;
        locals.var_eg_rv = 0.0;

        let (assign13090_e7407, assign13090_e7407_d_n0, assign13090_e7407_d_n2, assign13090_e7407_d_n4, assign13090_e7407_d_n5, assign13090_e7407_d_n6, assign13090_e7407_d_n7, assign13090_e7407_d_n8, assign13090_e7407_d_n9, assign13090_e7407_d_n10, assign13090_e7407_d_n11, assign13090_e7407_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13090_e7405: f64 = (locals.var_eg).sqrt();
        (assign13090_e7405, (locals.var_eg_dn0 / (2.0 * assign13090_e7405)), (locals.var_eg_dn2 / (2.0 * assign13090_e7405)), (locals.var_eg_dn4 / (2.0 * assign13090_e7405)), (locals.var_eg_dn5 / (2.0 * assign13090_e7405)), (locals.var_eg_dn6 / (2.0 * assign13090_e7405)), (locals.var_eg_dn7 / (2.0 * assign13090_e7405)), (locals.var_eg_dn8 / (2.0 * assign13090_e7405)), (locals.var_eg_dn9 / (2.0 * assign13090_e7405)), (locals.var_eg_dn10 / (2.0 * assign13090_e7405)), (locals.var_eg_dn11 / (2.0 * assign13090_e7405)), (locals.var_eg_dn14 / (2.0 * assign13090_e7405)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    }
};
        locals.var_sqrt_eg = assign13090_e7407;
        locals.var_sqrt_eg_dn0 = assign13090_e7407_d_n0;
        locals.var_sqrt_eg_dn2 = assign13090_e7407_d_n2;
        locals.var_sqrt_eg_dn4 = assign13090_e7407_d_n4;
        locals.var_sqrt_eg_dn5 = assign13090_e7407_d_n5;
        locals.var_sqrt_eg_dn6 = assign13090_e7407_d_n6;
        locals.var_sqrt_eg_dn7 = assign13090_e7407_d_n7;
        locals.var_sqrt_eg_dn8 = assign13090_e7407_d_n8;
        locals.var_sqrt_eg_dn9 = assign13090_e7407_d_n9;
        locals.var_sqrt_eg_dn10 = assign13090_e7407_d_n10;
        locals.var_sqrt_eg_dn11 = assign13090_e7407_d_n11;
        locals.var_sqrt_eg_dn14 = assign13090_e7407_d_n14;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign13100_e7413, assign13100_e7413_d_n0, assign13100_e7413_d_n2, assign13100_e7413_d_n4, assign13100_e7413_d_n5, assign13100_e7413_d_n6, assign13100_e7413_d_n7, assign13100_e7413_d_n8, assign13100_e7413_d_n9, assign13100_e7413_d_n10, assign13100_e7413_d_n11, assign13100_e7413_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13100_e7411: f64 = (1.0 / locals.var_ttemp);
        (assign13100_e7411, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn11 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn14 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13100_e7413;
        locals.var_t1_dn0 = assign13100_e7413_d_n0;
        locals.var_t1_dn2 = assign13100_e7413_d_n2;
        locals.var_t1_dn4 = assign13100_e7413_d_n4;
        locals.var_t1_dn5 = assign13100_e7413_d_n5;
        locals.var_t1_dn6 = assign13100_e7413_d_n6;
        locals.var_t1_dn7 = assign13100_e7413_d_n7;
        locals.var_t1_dn8 = assign13100_e7413_d_n8;
        locals.var_t1_dn9 = assign13100_e7413_d_n9;
        locals.var_t1_dn10 = assign13100_e7413_d_n10;
        locals.var_t1_dn11 = assign13100_e7413_d_n11;
        locals.var_t1_dn14 = assign13100_e7413_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13110_e7419, assign13110_e7419_d_n0, assign13110_e7419_d_n2, assign13110_e7419_d_n4, assign13110_e7419_d_n5, assign13110_e7419_d_n6, assign13110_e7419_d_n7, assign13110_e7419_d_n8, assign13110_e7419_d_n9, assign13110_e7419_d_n10, assign13110_e7419_d_n11, assign13110_e7419_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13110_e7417: f64 = (1.0 / locals.var_ktnom);
        (assign13110_e7417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13110_e7419;
        locals.var_t2_dn0 = assign13110_e7419_d_n0;
        locals.var_t2_dn2 = assign13110_e7419_d_n2;
        locals.var_t2_dn4 = assign13110_e7419_d_n4;
        locals.var_t2_dn5 = assign13110_e7419_d_n5;
        locals.var_t2_dn6 = assign13110_e7419_d_n6;
        locals.var_t2_dn7 = assign13110_e7419_d_n7;
        locals.var_t2_dn8 = assign13110_e7419_d_n8;
        locals.var_t2_dn9 = assign13110_e7419_d_n9;
        locals.var_t2_dn10 = assign13110_e7419_d_n10;
        locals.var_t2_dn11 = assign13110_e7419_d_n11;
        locals.var_t2_dn14 = assign13110_e7419_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13120_e7441, assign13120_e7441_d_n0, assign13120_e7441_d_n2, assign13120_e7441_d_n4, assign13120_e7441_d_n5, assign13120_e7441_d_n6, assign13120_e7441_d_n7, assign13120_e7441_d_n8, assign13120_e7441_d_n9, assign13120_e7441_d_n10, assign13120_e7441_d_n11, assign13120_e7441_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13120_e7423: f64 = (locals.var_egtnom + p.p259);
        let assign13120_e7427: f64 = (locals.var_t1 - locals.var_t2);
        let assign13120_e7428: f64 = (p.p260 * assign13120_e7427);
        let assign13120_e7429: f64 = (assign13120_e7423 + assign13120_e7428);
        let assign13120_e7433: f64 = (locals.var_t1 * locals.var_t1);
        let assign13120_e7436: f64 = (locals.var_t2 * locals.var_t2);
        let assign13120_e7437: f64 = (assign13120_e7433 - assign13120_e7436);
        let assign13120_e7438: f64 = (p.p261 * assign13120_e7437);
        let assign13120_e7439: f64 = (assign13120_e7429 + assign13120_e7438);
        (assign13120_e7439, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn11 - locals.var_t2_dn11)) + (p.p261 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) - ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))))), ((p.p260 * (locals.var_t1_dn14 - locals.var_t2_dn14)) + (p.p261 * (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) - ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign13120_e7441;
        locals.var_t3_dn0 = assign13120_e7441_d_n0;
        locals.var_t3_dn2 = assign13120_e7441_d_n2;
        locals.var_t3_dn4 = assign13120_e7441_d_n4;
        locals.var_t3_dn5 = assign13120_e7441_d_n5;
        locals.var_t3_dn6 = assign13120_e7441_d_n6;
        locals.var_t3_dn7 = assign13120_e7441_d_n7;
        locals.var_t3_dn8 = assign13120_e7441_d_n8;
        locals.var_t3_dn9 = assign13120_e7441_d_n9;
        locals.var_t3_dn10 = assign13120_e7441_d_n10;
        locals.var_t3_dn11 = assign13120_e7441_d_n11;
        locals.var_t3_dn14 = assign13120_e7441_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign13130_e7446, assign13130_e7446_d_n0, assign13130_e7446_d_n2, assign13130_e7446_d_n4, assign13130_e7446_d_n5, assign13130_e7446_d_n6, assign13130_e7446_d_n7, assign13130_e7446_d_n8, assign13130_e7446_d_n9, assign13130_e7446_d_n10, assign13130_e7446_d_n11, assign13130_e7446_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13130_e7444: f64 = (locals.var_t3).sqrt();
        (assign13130_e7444, (locals.var_t3_dn0 / (2.0 * assign13130_e7444)), (locals.var_t3_dn2 / (2.0 * assign13130_e7444)), (locals.var_t3_dn4 / (2.0 * assign13130_e7444)), (locals.var_t3_dn5 / (2.0 * assign13130_e7444)), (locals.var_t3_dn6 / (2.0 * assign13130_e7444)), (locals.var_t3_dn7 / (2.0 * assign13130_e7444)), (locals.var_t3_dn8 / (2.0 * assign13130_e7444)), (locals.var_t3_dn9 / (2.0 * assign13130_e7444)), (locals.var_t3_dn10 / (2.0 * assign13130_e7444)), (locals.var_t3_dn11 / (2.0 * assign13130_e7444)), (locals.var_t3_dn14 / (2.0 * assign13130_e7444)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn11, locals.var_egp12_dn14,)
    }
};
        locals.var_egp12 = assign13130_e7446;
        locals.var_egp12_dn0 = assign13130_e7446_d_n0;
        locals.var_egp12_dn2 = assign13130_e7446_d_n2;
        locals.var_egp12_dn4 = assign13130_e7446_d_n4;
        locals.var_egp12_dn5 = assign13130_e7446_d_n5;
        locals.var_egp12_dn6 = assign13130_e7446_d_n6;
        locals.var_egp12_dn7 = assign13130_e7446_d_n7;
        locals.var_egp12_dn8 = assign13130_e7446_d_n8;
        locals.var_egp12_dn9 = assign13130_e7446_d_n9;
        locals.var_egp12_dn10 = assign13130_e7446_d_n10;
        locals.var_egp12_dn11 = assign13130_e7446_d_n11;
        locals.var_egp12_dn14 = assign13130_e7446_d_n14;
        locals.var_egp12_rv = 0.0;

        let (assign13140_e7452, assign13140_e7452_d_n0, assign13140_e7452_d_n2, assign13140_e7452_d_n4, assign13140_e7452_d_n5, assign13140_e7452_d_n6, assign13140_e7452_d_n7, assign13140_e7452_d_n8, assign13140_e7452_d_n9, assign13140_e7452_d_n10, assign13140_e7452_d_n11, assign13140_e7452_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13140_e7450: f64 = (locals.var_t3 * locals.var_egp12);
        (assign13140_e7450, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn11 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn11)), ((locals.var_t3_dn14 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn14)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn11, locals.var_egp32_dn14,)
    }
};
        locals.var_egp32 = assign13140_e7452;
        locals.var_egp32_dn0 = assign13140_e7452_d_n0;
        locals.var_egp32_dn2 = assign13140_e7452_d_n2;
        locals.var_egp32_dn4 = assign13140_e7452_d_n4;
        locals.var_egp32_dn5 = assign13140_e7452_d_n5;
        locals.var_egp32_dn6 = assign13140_e7452_d_n6;
        locals.var_egp32_dn7 = assign13140_e7452_d_n7;
        locals.var_egp32_dn8 = assign13140_e7452_d_n8;
        locals.var_egp32_dn9 = assign13140_e7452_d_n9;
        locals.var_egp32_dn10 = assign13140_e7452_d_n10;
        locals.var_egp32_dn11 = assign13140_e7452_d_n11;
        locals.var_egp32_dn14 = assign13140_e7452_d_n14;
        locals.var_egp32_rv = 0.0;

        let (assign13150_e7460, assign13150_e7460_d_n0, assign13150_e7460_d_n2, assign13150_e7460_d_n4, assign13150_e7460_d_n5, assign13150_e7460_d_n6, assign13150_e7460_d_n7, assign13150_e7460_d_n8, assign13150_e7460_d_n9, assign13150_e7460_d_n10, assign13150_e7460_d_n11, assign13150_e7460_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13150_e7457: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign13150_e7458: f64 = (1.6021918e-19 / assign13150_e7457);
        (assign13150_e7458, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn11)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn14)) / (assign13150_e7457 * assign13150_e7457))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    }
};
        locals.var_beta = assign13150_e7460;
        locals.var_beta_dn0 = assign13150_e7460_d_n0;
        locals.var_beta_dn2 = assign13150_e7460_d_n2;
        locals.var_beta_dn4 = assign13150_e7460_d_n4;
        locals.var_beta_dn5 = assign13150_e7460_d_n5;
        locals.var_beta_dn6 = assign13150_e7460_d_n6;
        locals.var_beta_dn7 = assign13150_e7460_d_n7;
        locals.var_beta_dn8 = assign13150_e7460_d_n8;
        locals.var_beta_dn9 = assign13150_e7460_d_n9;
        locals.var_beta_dn10 = assign13150_e7460_d_n10;
        locals.var_beta_dn11 = assign13150_e7460_d_n11;
        locals.var_beta_dn14 = assign13150_e7460_d_n14;
        locals.var_beta_rv = 0.0;

        let (assign13160_e7466, assign13160_e7466_d_n0, assign13160_e7466_d_n2, assign13160_e7466_d_n4, assign13160_e7466_d_n5, assign13160_e7466_d_n6, assign13160_e7466_d_n7, assign13160_e7466_d_n8, assign13160_e7466_d_n9, assign13160_e7466_d_n10, assign13160_e7466_d_n11, assign13160_e7466_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13160_e7464: f64 = (1.0 / locals.var_beta);
        (assign13160_e7464, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn11 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn14 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn11, locals.var_beta_inv_dn14,)
    }
};
        locals.var_beta_inv = assign13160_e7466;
        locals.var_beta_inv_dn0 = assign13160_e7466_d_n0;
        locals.var_beta_inv_dn2 = assign13160_e7466_d_n2;
        locals.var_beta_inv_dn4 = assign13160_e7466_d_n4;
        locals.var_beta_inv_dn5 = assign13160_e7466_d_n5;
        locals.var_beta_inv_dn6 = assign13160_e7466_d_n6;
        locals.var_beta_inv_dn7 = assign13160_e7466_d_n7;
        locals.var_beta_inv_dn8 = assign13160_e7466_d_n8;
        locals.var_beta_inv_dn9 = assign13160_e7466_d_n9;
        locals.var_beta_inv_dn10 = assign13160_e7466_d_n10;
        locals.var_beta_inv_dn11 = assign13160_e7466_d_n11;
        locals.var_beta_inv_dn14 = assign13160_e7466_d_n14;
        locals.var_beta_inv_rv = 0.0;

        let (assign13170_e7472, assign13170_e7472_d_n0, assign13170_e7472_d_n2, assign13170_e7472_d_n4, assign13170_e7472_d_n5, assign13170_e7472_d_n6, assign13170_e7472_d_n7, assign13170_e7472_d_n8, assign13170_e7472_d_n9, assign13170_e7472_d_n10, assign13170_e7472_d_n11, assign13170_e7472_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13170_e7470: f64 = (locals.var_beta * locals.var_beta);
        (assign13170_e7470, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn11 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn11)), ((locals.var_beta_dn14 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn14)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn11, locals.var_beta2_dn14,)
    }
};
        locals.var_beta2 = assign13170_e7472;
        locals.var_beta2_dn0 = assign13170_e7472_d_n0;
        locals.var_beta2_dn2 = assign13170_e7472_d_n2;
        locals.var_beta2_dn4 = assign13170_e7472_d_n4;
        locals.var_beta2_dn5 = assign13170_e7472_d_n5;
        locals.var_beta2_dn6 = assign13170_e7472_d_n6;
        locals.var_beta2_dn7 = assign13170_e7472_d_n7;
        locals.var_beta2_dn8 = assign13170_e7472_d_n8;
        locals.var_beta2_dn9 = assign13170_e7472_d_n9;
        locals.var_beta2_dn10 = assign13170_e7472_d_n10;
        locals.var_beta2_dn11 = assign13170_e7472_d_n11;
        locals.var_beta2_dn14 = assign13170_e7472_d_n14;
        locals.var_beta2_rv = 0.0;

        let (assign13180_e7480,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13180_e7477: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign13180_e7478: f64 = (1.6021918e-19 / assign13180_e7477);
        (assign13180_e7478,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign13180_e7480;
        locals.var_betatnom_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13190_e7503, assign13190_e7503_d_n0, assign13190_e7503_d_n2, assign13190_e7503_d_n4, assign13190_e7503_d_n5, assign13190_e7503_d_n6, assign13190_e7503_d_n7, assign13190_e7503_d_n8, assign13190_e7503_d_n9, assign13190_e7503_d_n10, assign13190_e7503_d_n11, assign13190_e7503_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13190_e7485: f64 = (locals.var_log_tratio * 1.5);
        let assign13190_e7486: f64 = (assign13190_e7485).exp();
        let assign13190_e7487: f64 = (1.04e16 * assign13190_e7486);
        let assign13190_e7489: f64 = (-locals.var_eg);
        let assign13190_e7491: f64 = (assign13190_e7489 / 2.0);
        let assign13190_e7493: f64 = (assign13190_e7491 * locals.var_beta);
        let assign13190_e7496: f64 = (locals.var_egtnom / 2.0);
        let assign13190_e7498: f64 = (assign13190_e7496 * locals.var_betatnom);
        let assign13190_e7499: f64 = (assign13190_e7493 + assign13190_e7498);
        let assign13190_e7500: f64 = (assign13190_e7499).exp();
        let assign13190_e7501: f64 = (assign13190_e7487 * assign13190_e7500);
        (assign13190_e7501, (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn0 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn0))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn2 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn2))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn4 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn4))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn5 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn5))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn6 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn6))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn7 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn7))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn8 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn8))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn9 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn9))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn10 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn10))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn11 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn11) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn11))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn14 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn14) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn14))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn11, locals.var_nin_dn14,)
    }
};
        locals.var_nin = assign13190_e7503;
        locals.var_nin_dn0 = assign13190_e7503_d_n0;
        locals.var_nin_dn2 = assign13190_e7503_d_n2;
        locals.var_nin_dn4 = assign13190_e7503_d_n4;
        locals.var_nin_dn5 = assign13190_e7503_d_n5;
        locals.var_nin_dn6 = assign13190_e7503_d_n6;
        locals.var_nin_dn7 = assign13190_e7503_d_n7;
        locals.var_nin_dn8 = assign13190_e7503_d_n8;
        locals.var_nin_dn9 = assign13190_e7503_d_n9;
        locals.var_nin_dn10 = assign13190_e7503_d_n10;
        locals.var_nin_dn11 = assign13190_e7503_d_n11;
        locals.var_nin_dn14 = assign13190_e7503_d_n14;
        locals.var_nin_rv = 0.0;

        let (assign13200_e7510, assign13200_e7510_d_n0, assign13200_e7510_d_n2, assign13200_e7510_d_n4, assign13200_e7510_d_n5, assign13200_e7510_d_n6, assign13200_e7510_d_n7, assign13200_e7510_d_n8, assign13200_e7510_d_n9, assign13200_e7510_d_n10, assign13200_e7510_d_n11, assign13200_e7510_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13200_e7507: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign13200_e7508: f64 = (assign13200_e7507).exp();
        (assign13200_e7508, (assign13200_e7508 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn11 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn14 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13200_e7510;
        locals.var_t1_dn0 = assign13200_e7510_d_n0;
        locals.var_t1_dn2 = assign13200_e7510_d_n2;
        locals.var_t1_dn4 = assign13200_e7510_d_n4;
        locals.var_t1_dn5 = assign13200_e7510_d_n5;
        locals.var_t1_dn6 = assign13200_e7510_d_n6;
        locals.var_t1_dn7 = assign13200_e7510_d_n7;
        locals.var_t1_dn8 = assign13200_e7510_d_n8;
        locals.var_t1_dn9 = assign13200_e7510_d_n9;
        locals.var_t1_dn10 = assign13200_e7510_d_n10;
        locals.var_t1_dn11 = assign13200_e7510_d_n11;
        locals.var_t1_dn14 = assign13200_e7510_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13210_e7516, assign13210_e7516_d_n0, assign13210_e7516_d_n2, assign13210_e7516_d_n4, assign13210_e7516_d_n5, assign13210_e7516_d_n6, assign13210_e7516_d_n7, assign13210_e7516_d_n8, assign13210_e7516_d_n9, assign13210_e7516_d_n10, assign13210_e7516_d_n11, assign13210_e7516_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13210_e7514: f64 = (locals.var_t1 / locals.var_mueph);
        (assign13210_e7514, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn11 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn11)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn14 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn14)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn11, locals.var_mphn0_dn14,)
    }
};
        locals.var_mphn0 = assign13210_e7516;
        locals.var_mphn0_dn0 = assign13210_e7516_d_n0;
        locals.var_mphn0_dn2 = assign13210_e7516_d_n2;
        locals.var_mphn0_dn4 = assign13210_e7516_d_n4;
        locals.var_mphn0_dn5 = assign13210_e7516_d_n5;
        locals.var_mphn0_dn6 = assign13210_e7516_d_n6;
        locals.var_mphn0_dn7 = assign13210_e7516_d_n7;
        locals.var_mphn0_dn8 = assign13210_e7516_d_n8;
        locals.var_mphn0_dn9 = assign13210_e7516_d_n9;
        locals.var_mphn0_dn10 = assign13210_e7516_d_n10;
        locals.var_mphn0_dn11 = assign13210_e7516_d_n11;
        locals.var_mphn0_dn14 = assign13210_e7516_d_n14;
        locals.var_mphn0_rv = 0.0;

        let assign13220_e7523: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard292 = assign13220_e7523;
        locals.var_guard292_rv = 0.0;

        let (assign13230_e7538, assign13230_e7538_d_n0, assign13230_e7538_d_n2, assign13230_e7538_d_n4, assign13230_e7538_d_n5, assign13230_e7538_d_n6, assign13230_e7538_d_n7, assign13230_e7538_d_n8, assign13230_e7538_d_n9, assign13230_e7538_d_n10, assign13230_e7538_d_n11, assign13230_e7538_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13230_e7529: f64 = (2.0 * 1.034943e-10);
        let assign13230_e7531: f64 = (assign13230_e7529 * 1.6021918e-19);
        let assign13230_e7533: f64 = (assign13230_e7531 * locals.var_uc_ndepm);
        let assign13230_e7535: f64 = (assign13230_e7533 * locals.var_beta_inv);
        let assign13230_e7536: f64 = (assign13230_e7535).sqrt();
        (assign13230_e7536, ((((assign13230_e7531 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn0)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn2)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn4)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn5)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn6)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn7)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn8)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn9)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn10)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn11)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn14)) / (2.0 * assign13230_e7536)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign13230_e7538;
        locals.var_cnst0_dn0 = assign13230_e7538_d_n0;
        locals.var_cnst0_dn2 = assign13230_e7538_d_n2;
        locals.var_cnst0_dn4 = assign13230_e7538_d_n4;
        locals.var_cnst0_dn5 = assign13230_e7538_d_n5;
        locals.var_cnst0_dn6 = assign13230_e7538_d_n6;
        locals.var_cnst0_dn7 = assign13230_e7538_d_n7;
        locals.var_cnst0_dn8 = assign13230_e7538_d_n8;
        locals.var_cnst0_dn9 = assign13230_e7538_d_n9;
        locals.var_cnst0_dn10 = assign13230_e7538_d_n10;
        locals.var_cnst0_dn11 = assign13230_e7538_d_n11;
        locals.var_cnst0_dn14 = assign13230_e7538_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign13240_e7550, assign13240_e7550_d_n0, assign13240_e7550_d_n2, assign13240_e7550_d_n4, assign13240_e7550_d_n5, assign13240_e7550_d_n6, assign13240_e7550_d_n7, assign13240_e7550_d_n8, assign13240_e7550_d_n9, assign13240_e7550_d_n10, assign13240_e7550_d_n11, assign13240_e7550_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13240_e7544: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13240_e7546: f64 = (assign13240_e7544 * __rspice_inv_cse_0);
        let assign13240_e7548: f64 = (assign13240_e7546 * __rspice_inv_cse_0);
        (assign13240_e7548, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign13240_e7550;
        locals.var_cnst1_dn0 = assign13240_e7550_d_n0;
        locals.var_cnst1_dn2 = assign13240_e7550_d_n2;
        locals.var_cnst1_dn4 = assign13240_e7550_d_n4;
        locals.var_cnst1_dn5 = assign13240_e7550_d_n5;
        locals.var_cnst1_dn6 = assign13240_e7550_d_n6;
        locals.var_cnst1_dn7 = assign13240_e7550_d_n7;
        locals.var_cnst1_dn8 = assign13240_e7550_d_n8;
        locals.var_cnst1_dn9 = assign13240_e7550_d_n9;
        locals.var_cnst1_dn10 = assign13240_e7550_d_n10;
        locals.var_cnst1_dn11 = assign13240_e7550_d_n11;
        locals.var_cnst1_dn14 = assign13240_e7550_d_n14;
        locals.var_cnst1_rv = 0.0;

        let (assign13250_e7563, assign13250_e7563_d_n0, assign13250_e7563_d_n2, assign13250_e7563_d_n4, assign13250_e7563_d_n5, assign13250_e7563_d_n6, assign13250_e7563_d_n7, assign13250_e7563_d_n8, assign13250_e7563_d_n9, assign13250_e7563_d_n10, assign13250_e7563_d_n11, assign13250_e7563_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13250_e7556: f64 = (2.0 * locals.var_beta_inv);
        let assign13250_e7559: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13250_e7560: f64 = (assign13250_e7559).ln();
        let assign13250_e7561: f64 = (assign13250_e7556 * assign13250_e7560);
        (assign13250_e7561, (((2.0 * locals.var_beta_inv_dn0) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn2) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn4) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn5) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn6) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn7) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn8) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn9) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn10) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn11) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn14) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13250_e7563;
        locals.var_pb2n_dn0 = assign13250_e7563_d_n0;
        locals.var_pb2n_dn2 = assign13250_e7563_d_n2;
        locals.var_pb2n_dn4 = assign13250_e7563_d_n4;
        locals.var_pb2n_dn5 = assign13250_e7563_d_n5;
        locals.var_pb2n_dn6 = assign13250_e7563_d_n6;
        locals.var_pb2n_dn7 = assign13250_e7563_d_n7;
        locals.var_pb2n_dn8 = assign13250_e7563_d_n8;
        locals.var_pb2n_dn9 = assign13250_e7563_d_n9;
        locals.var_pb2n_dn10 = assign13250_e7563_d_n10;
        locals.var_pb2n_dn11 = assign13250_e7563_d_n11;
        locals.var_pb2n_dn14 = assign13250_e7563_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign13260_e7578, assign13260_e7578_d_n0, assign13260_e7578_d_n2, assign13260_e7578_d_n4, assign13260_e7578_d_n5, assign13260_e7578_d_n6, assign13260_e7578_d_n7, assign13260_e7578_d_n8, assign13260_e7578_d_n9, assign13260_e7578_d_n10, assign13260_e7578_d_n11, assign13260_e7578_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13260_e7570: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign13260_e7572: f64 = (assign13260_e7570 * __rspice_inv_cse_1);
        let assign13260_e7574: f64 = (assign13260_e7572 * __rspice_inv_cse_1);
        let assign13260_e7575: f64 = (assign13260_e7574).ln();
        let assign13260_e7576: f64 = (locals.var_beta_inv * assign13260_e7575);
        (assign13260_e7576, ((locals.var_beta_inv_dn0 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn2 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn4 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn5 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn6 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn7 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn8 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn9 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn10 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn11 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn14 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13260_e7578;
        locals.var_vbipn_dn0 = assign13260_e7578_d_n0;
        locals.var_vbipn_dn2 = assign13260_e7578_d_n2;
        locals.var_vbipn_dn4 = assign13260_e7578_d_n4;
        locals.var_vbipn_dn5 = assign13260_e7578_d_n5;
        locals.var_vbipn_dn6 = assign13260_e7578_d_n6;
        locals.var_vbipn_dn7 = assign13260_e7578_d_n7;
        locals.var_vbipn_dn8 = assign13260_e7578_d_n8;
        locals.var_vbipn_dn9 = assign13260_e7578_d_n9;
        locals.var_vbipn_dn10 = assign13260_e7578_d_n10;
        locals.var_vbipn_dn11 = assign13260_e7578_d_n11;
        locals.var_vbipn_dn14 = assign13260_e7578_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign13270_e7587, assign13270_e7587_d_n0, assign13270_e7587_d_n2, assign13270_e7587_d_n4, assign13270_e7587_d_n5, assign13270_e7587_d_n6, assign13270_e7587_d_n7, assign13270_e7587_d_n8, assign13270_e7587_d_n9, assign13270_e7587_d_n10, assign13270_e7587_d_n11, assign13270_e7587_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13270_e7584: f64 = (locals.var_log_tratio * p.p380);
        let assign13270_e7585: f64 = (assign13270_e7584).exp();
        (assign13270_e7585, (assign13270_e7585 * (locals.var_log_tratio_dn0 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn2 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn4 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn5 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn6 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn7 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn8 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn9 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn10 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn11 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13270_e7587;
        locals.var_t1_dn0 = assign13270_e7587_d_n0;
        locals.var_t1_dn2 = assign13270_e7587_d_n2;
        locals.var_t1_dn4 = assign13270_e7587_d_n4;
        locals.var_t1_dn5 = assign13270_e7587_d_n5;
        locals.var_t1_dn6 = assign13270_e7587_d_n6;
        locals.var_t1_dn7 = assign13270_e7587_d_n7;
        locals.var_t1_dn8 = assign13270_e7587_d_n8;
        locals.var_t1_dn9 = assign13270_e7587_d_n9;
        locals.var_t1_dn10 = assign13270_e7587_d_n10;
        locals.var_t1_dn11 = assign13270_e7587_d_n11;
        locals.var_t1_dn14 = assign13270_e7587_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13280_e7595, assign13280_e7595_d_n0, assign13280_e7595_d_n2, assign13280_e7595_d_n4, assign13280_e7595_d_n5, assign13280_e7595_d_n6, assign13280_e7595_d_n7, assign13280_e7595_d_n8, assign13280_e7595_d_n9, assign13280_e7595_d_n10, assign13280_e7595_d_n11, assign13280_e7595_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13280_e7593: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13280_e7593, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13280_e7595;
        locals.var_depmphn0_dn0 = assign13280_e7595_d_n0;
        locals.var_depmphn0_dn2 = assign13280_e7595_d_n2;
        locals.var_depmphn0_dn4 = assign13280_e7595_d_n4;
        locals.var_depmphn0_dn5 = assign13280_e7595_d_n5;
        locals.var_depmphn0_dn6 = assign13280_e7595_d_n6;
        locals.var_depmphn0_dn7 = assign13280_e7595_d_n7;
        locals.var_depmphn0_dn8 = assign13280_e7595_d_n8;
        locals.var_depmphn0_dn9 = assign13280_e7595_d_n9;
        locals.var_depmphn0_dn10 = assign13280_e7595_d_n10;
        locals.var_depmphn0_dn11 = assign13280_e7595_d_n11;
        locals.var_depmphn0_dn14 = assign13280_e7595_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign13290_e7617, assign13290_e7617_d_n0, assign13290_e7617_d_n2, assign13290_e7617_d_n4, assign13290_e7617_d_n5, assign13290_e7617_d_n6, assign13290_e7617_d_n7, assign13290_e7617_d_n8, assign13290_e7617_d_n9, assign13290_e7617_d_n10, assign13290_e7617_d_n11, assign13290_e7617_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13290_e7602: f64 = (0.4 * locals.var_tratio);
        let assign13290_e7603: f64 = (1.8 + assign13290_e7602);
        let assign13290_e7606: f64 = (0.1 * locals.var_tratio);
        let assign13290_e7608: f64 = (assign13290_e7606 * locals.var_tratio);
        let assign13290_e7609: f64 = (assign13290_e7603 + assign13290_e7608);
        let assign13290_e7613: f64 = (1.0 - locals.var_tratio);
        let assign13290_e7614: f64 = (p.p379 * assign13290_e7613);
        let assign13290_e7615: f64 = (assign13290_e7609 - assign13290_e7614);
        (assign13290_e7615, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13290_e7617;
        locals.var_t0_dn0 = assign13290_e7617_d_n0;
        locals.var_t0_dn2 = assign13290_e7617_d_n2;
        locals.var_t0_dn4 = assign13290_e7617_d_n4;
        locals.var_t0_dn5 = assign13290_e7617_d_n5;
        locals.var_t0_dn6 = assign13290_e7617_d_n6;
        locals.var_t0_dn7 = assign13290_e7617_d_n7;
        locals.var_t0_dn8 = assign13290_e7617_d_n8;
        locals.var_t0_dn9 = assign13290_e7617_d_n9;
        locals.var_t0_dn10 = assign13290_e7617_d_n10;
        locals.var_t0_dn11 = assign13290_e7617_d_n11;
        locals.var_t0_dn14 = assign13290_e7617_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13300_e7625, assign13300_e7625_d_n0, assign13300_e7625_d_n2, assign13300_e7625_d_n4, assign13300_e7625_d_n5, assign13300_e7625_d_n6, assign13300_e7625_d_n7, assign13300_e7625_d_n8, assign13300_e7625_d_n9, assign13300_e7625_d_n10, assign13300_e7625_d_n11, assign13300_e7625_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13300_e7623: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13300_e7623, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13300_e7625;
        locals.var_uc_depvmax_dn0 = assign13300_e7625_d_n0;
        locals.var_uc_depvmax_dn2 = assign13300_e7625_d_n2;
        locals.var_uc_depvmax_dn4 = assign13300_e7625_d_n4;
        locals.var_uc_depvmax_dn5 = assign13300_e7625_d_n5;
        locals.var_uc_depvmax_dn6 = assign13300_e7625_d_n6;
        locals.var_uc_depvmax_dn7 = assign13300_e7625_d_n7;
        locals.var_uc_depvmax_dn8 = assign13300_e7625_d_n8;
        locals.var_uc_depvmax_dn9 = assign13300_e7625_d_n9;
        locals.var_uc_depvmax_dn10 = assign13300_e7625_d_n10;
        locals.var_uc_depvmax_dn11 = assign13300_e7625_d_n11;
        locals.var_uc_depvmax_dn14 = assign13300_e7625_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let assign13320_e7633: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign13320_e7633;
        locals.var_guard294_rv = 0.0;

        let (assign13330_e7641, assign13330_e7641_d_n0, assign13330_e7641_d_n2, assign13330_e7641_d_n4, assign13330_e7641_d_n5, assign13330_e7641_d_n6, assign13330_e7641_d_n7, assign13330_e7641_d_n8, assign13330_e7641_d_n9, assign13330_e7641_d_n10, assign13330_e7641_d_n11, assign13330_e7641_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) && (locals.var_guard294 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13330_e7641;
        locals.var_uc_depvmax_dn0 = assign13330_e7641_d_n0;
        locals.var_uc_depvmax_dn2 = assign13330_e7641_d_n2;
        locals.var_uc_depvmax_dn4 = assign13330_e7641_d_n4;
        locals.var_uc_depvmax_dn5 = assign13330_e7641_d_n5;
        locals.var_uc_depvmax_dn6 = assign13330_e7641_d_n6;
        locals.var_uc_depvmax_dn7 = assign13330_e7641_d_n7;
        locals.var_uc_depvmax_dn8 = assign13330_e7641_d_n8;
        locals.var_uc_depvmax_dn9 = assign13330_e7641_d_n9;
        locals.var_uc_depvmax_dn10 = assign13330_e7641_d_n10;
        locals.var_uc_depvmax_dn11 = assign13330_e7641_d_n11;
        locals.var_uc_depvmax_dn14 = assign13330_e7641_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign13340_e7651, assign13340_e7651_d_n0, assign13340_e7651_d_n2, assign13340_e7651_d_n4, assign13340_e7651_d_n5, assign13340_e7651_d_n6, assign13340_e7651_d_n7, assign13340_e7651_d_n8, assign13340_e7651_d_n9, assign13340_e7651_d_n10, assign13340_e7651_d_n11, assign13340_e7651_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13340_e7648: f64 = (locals.var_tratio).powf(p.p381);
        let assign13340_e7649: f64 = (locals.var_uc_depmue0 / assign13340_e7648);
        (assign13340_e7649, (((locals.var_uc_depmue0_dn0 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn2 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn4 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn5 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn6 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn7 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn8 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn9 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn10 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn11 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn14 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign13340_e7651;
        locals.var_uc_depmue0_dn0 = assign13340_e7651_d_n0;
        locals.var_uc_depmue0_dn2 = assign13340_e7651_d_n2;
        locals.var_uc_depmue0_dn4 = assign13340_e7651_d_n4;
        locals.var_uc_depmue0_dn5 = assign13340_e7651_d_n5;
        locals.var_uc_depmue0_dn6 = assign13340_e7651_d_n6;
        locals.var_uc_depmue0_dn7 = assign13340_e7651_d_n7;
        locals.var_uc_depmue0_dn8 = assign13340_e7651_d_n8;
        locals.var_uc_depmue0_dn9 = assign13340_e7651_d_n9;
        locals.var_uc_depmue0_dn10 = assign13340_e7651_d_n10;
        locals.var_uc_depmue0_dn11 = assign13340_e7651_d_n11;
        locals.var_uc_depmue0_dn14 = assign13340_e7651_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign13350_e7661, assign13350_e7661_d_n0, assign13350_e7661_d_n2, assign13350_e7661_d_n4, assign13350_e7661_d_n5, assign13350_e7661_d_n6, assign13350_e7661_d_n7, assign13350_e7661_d_n8, assign13350_e7661_d_n9, assign13350_e7661_d_n10, assign13350_e7661_d_n11, assign13350_e7661_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13350_e7658: f64 = (locals.var_tratio).powf(p.p382);
        let assign13350_e7659: f64 = (locals.var_uc_depmue2 / assign13350_e7658);
        (assign13350_e7659, (((locals.var_uc_depmue2_dn0 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn2 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn4 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn5 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn6 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn7 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn8 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn9 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn10 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn11 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn14 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn11, locals.var_uc_depmue2_dn14,)
    }
};
        locals.var_uc_depmue2 = assign13350_e7661;
        locals.var_uc_depmue2_dn0 = assign13350_e7661_d_n0;
        locals.var_uc_depmue2_dn2 = assign13350_e7661_d_n2;
        locals.var_uc_depmue2_dn4 = assign13350_e7661_d_n4;
        locals.var_uc_depmue2_dn5 = assign13350_e7661_d_n5;
        locals.var_uc_depmue2_dn6 = assign13350_e7661_d_n6;
        locals.var_uc_depmue2_dn7 = assign13350_e7661_d_n7;
        locals.var_uc_depmue2_dn8 = assign13350_e7661_d_n8;
        locals.var_uc_depmue2_dn9 = assign13350_e7661_d_n9;
        locals.var_uc_depmue2_dn10 = assign13350_e7661_d_n10;
        locals.var_uc_depmue2_dn11 = assign13350_e7661_d_n11;
        locals.var_uc_depmue2_dn14 = assign13350_e7661_d_n14;
        locals.var_uc_depmue2_rv = 0.0;

        let assign13360_e7664: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign13360_e7664;
        locals.var_guard295_rv = 0.0;

        let (assign13370_e7682, assign13370_e7682_d_n0, assign13370_e7682_d_n2, assign13370_e7682_d_n4, assign13370_e7682_d_n5, assign13370_e7682_d_n6, assign13370_e7682_d_n7, assign13370_e7682_d_n8, assign13370_e7682_d_n9, assign13370_e7682_d_n10, assign13370_e7682_d_n11, assign13370_e7682_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13370_e7673: f64 = (2.0 * 1.034943e-10);
        let assign13370_e7675: f64 = (assign13370_e7673 * 1.6021918e-19);
        let assign13370_e7677: f64 = (assign13370_e7675 * locals.var_uc_ndepm);
        let assign13370_e7679: f64 = (assign13370_e7677 * locals.var_beta_inv);
        let assign13370_e7680: f64 = (assign13370_e7679).sqrt();
        (assign13370_e7680, ((((assign13370_e7675 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn0)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn2)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn4)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn5)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn6)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn7)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn8)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn9)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn10)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn11)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn14)) / (2.0 * assign13370_e7680)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign13370_e7682;
        locals.var_cnst0_dn0 = assign13370_e7682_d_n0;
        locals.var_cnst0_dn2 = assign13370_e7682_d_n2;
        locals.var_cnst0_dn4 = assign13370_e7682_d_n4;
        locals.var_cnst0_dn5 = assign13370_e7682_d_n5;
        locals.var_cnst0_dn6 = assign13370_e7682_d_n6;
        locals.var_cnst0_dn7 = assign13370_e7682_d_n7;
        locals.var_cnst0_dn8 = assign13370_e7682_d_n8;
        locals.var_cnst0_dn9 = assign13370_e7682_d_n9;
        locals.var_cnst0_dn10 = assign13370_e7682_d_n10;
        locals.var_cnst0_dn11 = assign13370_e7682_d_n11;
        locals.var_cnst0_dn14 = assign13370_e7682_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign13380_e7697, assign13380_e7697_d_n0, assign13380_e7697_d_n2, assign13380_e7697_d_n4, assign13380_e7697_d_n5, assign13380_e7697_d_n6, assign13380_e7697_d_n7, assign13380_e7697_d_n8, assign13380_e7697_d_n9, assign13380_e7697_d_n10, assign13380_e7697_d_n11, assign13380_e7697_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13380_e7691: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13380_e7693: f64 = (assign13380_e7691 * __rspice_inv_cse_2);
        let assign13380_e7695: f64 = (assign13380_e7693 * __rspice_inv_cse_2);
        (assign13380_e7695, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign13380_e7697;
        locals.var_cnst1_dn0 = assign13380_e7697_d_n0;
        locals.var_cnst1_dn2 = assign13380_e7697_d_n2;
        locals.var_cnst1_dn4 = assign13380_e7697_d_n4;
        locals.var_cnst1_dn5 = assign13380_e7697_d_n5;
        locals.var_cnst1_dn6 = assign13380_e7697_d_n6;
        locals.var_cnst1_dn7 = assign13380_e7697_d_n7;
        locals.var_cnst1_dn8 = assign13380_e7697_d_n8;
        locals.var_cnst1_dn9 = assign13380_e7697_d_n9;
        locals.var_cnst1_dn10 = assign13380_e7697_d_n10;
        locals.var_cnst1_dn11 = assign13380_e7697_d_n11;
        locals.var_cnst1_dn14 = assign13380_e7697_d_n14;
        locals.var_cnst1_rv = 0.0;

        let (assign13390_e7713, assign13390_e7713_d_n0, assign13390_e7713_d_n2, assign13390_e7713_d_n4, assign13390_e7713_d_n5, assign13390_e7713_d_n6, assign13390_e7713_d_n7, assign13390_e7713_d_n8, assign13390_e7713_d_n9, assign13390_e7713_d_n10, assign13390_e7713_d_n11, assign13390_e7713_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13390_e7706: f64 = (2.0 * locals.var_beta_inv);
        let assign13390_e7709: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13390_e7710: f64 = (assign13390_e7709).ln();
        let assign13390_e7711: f64 = (assign13390_e7706 * assign13390_e7710);
        (assign13390_e7711, (((2.0 * locals.var_beta_inv_dn0) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn2) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn4) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn5) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn6) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn7) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn8) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn9) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn10) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn11) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn14) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13390_e7713;
        locals.var_pb2n_dn0 = assign13390_e7713_d_n0;
        locals.var_pb2n_dn2 = assign13390_e7713_d_n2;
        locals.var_pb2n_dn4 = assign13390_e7713_d_n4;
        locals.var_pb2n_dn5 = assign13390_e7713_d_n5;
        locals.var_pb2n_dn6 = assign13390_e7713_d_n6;
        locals.var_pb2n_dn7 = assign13390_e7713_d_n7;
        locals.var_pb2n_dn8 = assign13390_e7713_d_n8;
        locals.var_pb2n_dn9 = assign13390_e7713_d_n9;
        locals.var_pb2n_dn10 = assign13390_e7713_d_n10;
        locals.var_pb2n_dn11 = assign13390_e7713_d_n11;
        locals.var_pb2n_dn14 = assign13390_e7713_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign13400_e7731, assign13400_e7731_d_n0, assign13400_e7731_d_n2, assign13400_e7731_d_n4, assign13400_e7731_d_n5, assign13400_e7731_d_n6, assign13400_e7731_d_n7, assign13400_e7731_d_n8, assign13400_e7731_d_n9, assign13400_e7731_d_n10, assign13400_e7731_d_n11, assign13400_e7731_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13400_e7723: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_nin;
        let assign13400_e7725: f64 = (assign13400_e7723 * __rspice_inv_cse_3);
        let assign13400_e7727: f64 = (assign13400_e7725 * __rspice_inv_cse_3);
        let assign13400_e7728: f64 = (assign13400_e7727).ln();
        let assign13400_e7729: f64 = (locals.var_beta_inv * assign13400_e7728);
        (assign13400_e7729, ((locals.var_beta_inv_dn0 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn2 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn4 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn5 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn6 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn7 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn8 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn9 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn10 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn11 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn14 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13400_e7731;
        locals.var_vbipn_dn0 = assign13400_e7731_d_n0;
        locals.var_vbipn_dn2 = assign13400_e7731_d_n2;
        locals.var_vbipn_dn4 = assign13400_e7731_d_n4;
        locals.var_vbipn_dn5 = assign13400_e7731_d_n5;
        locals.var_vbipn_dn6 = assign13400_e7731_d_n6;
        locals.var_vbipn_dn7 = assign13400_e7731_d_n7;
        locals.var_vbipn_dn8 = assign13400_e7731_d_n8;
        locals.var_vbipn_dn9 = assign13400_e7731_d_n9;
        locals.var_vbipn_dn10 = assign13400_e7731_d_n10;
        locals.var_vbipn_dn11 = assign13400_e7731_d_n11;
        locals.var_vbipn_dn14 = assign13400_e7731_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign13410_e7743, assign13410_e7743_d_n0, assign13410_e7743_d_n2, assign13410_e7743_d_n4, assign13410_e7743_d_n5, assign13410_e7743_d_n6, assign13410_e7743_d_n7, assign13410_e7743_d_n8, assign13410_e7743_d_n9, assign13410_e7743_d_n10, assign13410_e7743_d_n11, assign13410_e7743_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13410_e7740: f64 = (locals.var_log_tratio * p.p380);
        let assign13410_e7741: f64 = (assign13410_e7740).exp();
        (assign13410_e7741, (assign13410_e7741 * (locals.var_log_tratio_dn0 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn2 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn4 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn5 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn6 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn7 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn8 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn9 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn10 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn11 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13410_e7743;
        locals.var_t1_dn0 = assign13410_e7743_d_n0;
        locals.var_t1_dn2 = assign13410_e7743_d_n2;
        locals.var_t1_dn4 = assign13410_e7743_d_n4;
        locals.var_t1_dn5 = assign13410_e7743_d_n5;
        locals.var_t1_dn6 = assign13410_e7743_d_n6;
        locals.var_t1_dn7 = assign13410_e7743_d_n7;
        locals.var_t1_dn8 = assign13410_e7743_d_n8;
        locals.var_t1_dn9 = assign13410_e7743_d_n9;
        locals.var_t1_dn10 = assign13410_e7743_d_n10;
        locals.var_t1_dn11 = assign13410_e7743_d_n11;
        locals.var_t1_dn14 = assign13410_e7743_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13420_e7754, assign13420_e7754_d_n0, assign13420_e7754_d_n2, assign13420_e7754_d_n4, assign13420_e7754_d_n5, assign13420_e7754_d_n6, assign13420_e7754_d_n7, assign13420_e7754_d_n8, assign13420_e7754_d_n9, assign13420_e7754_d_n10, assign13420_e7754_d_n11, assign13420_e7754_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13420_e7752: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13420_e7752, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13420_e7754;
        locals.var_depmphn0_dn0 = assign13420_e7754_d_n0;
        locals.var_depmphn0_dn2 = assign13420_e7754_d_n2;
        locals.var_depmphn0_dn4 = assign13420_e7754_d_n4;
        locals.var_depmphn0_dn5 = assign13420_e7754_d_n5;
        locals.var_depmphn0_dn6 = assign13420_e7754_d_n6;
        locals.var_depmphn0_dn7 = assign13420_e7754_d_n7;
        locals.var_depmphn0_dn8 = assign13420_e7754_d_n8;
        locals.var_depmphn0_dn9 = assign13420_e7754_d_n9;
        locals.var_depmphn0_dn10 = assign13420_e7754_d_n10;
        locals.var_depmphn0_dn11 = assign13420_e7754_d_n11;
        locals.var_depmphn0_dn14 = assign13420_e7754_d_n14;
        locals.var_depmphn0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13430_e7779, assign13430_e7779_d_n0, assign13430_e7779_d_n2, assign13430_e7779_d_n4, assign13430_e7779_d_n5, assign13430_e7779_d_n6, assign13430_e7779_d_n7, assign13430_e7779_d_n8, assign13430_e7779_d_n9, assign13430_e7779_d_n10, assign13430_e7779_d_n11, assign13430_e7779_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13430_e7764: f64 = (0.4 * locals.var_tratio);
        let assign13430_e7765: f64 = (1.8 + assign13430_e7764);
        let assign13430_e7768: f64 = (0.1 * locals.var_tratio);
        let assign13430_e7770: f64 = (assign13430_e7768 * locals.var_tratio);
        let assign13430_e7771: f64 = (assign13430_e7765 + assign13430_e7770);
        let assign13430_e7775: f64 = (1.0 - locals.var_tratio);
        let assign13430_e7776: f64 = (p.p379 * assign13430_e7775);
        let assign13430_e7777: f64 = (assign13430_e7771 - assign13430_e7776);
        (assign13430_e7777, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13430_e7779;
        locals.var_t0_dn0 = assign13430_e7779_d_n0;
        locals.var_t0_dn2 = assign13430_e7779_d_n2;
        locals.var_t0_dn4 = assign13430_e7779_d_n4;
        locals.var_t0_dn5 = assign13430_e7779_d_n5;
        locals.var_t0_dn6 = assign13430_e7779_d_n6;
        locals.var_t0_dn7 = assign13430_e7779_d_n7;
        locals.var_t0_dn8 = assign13430_e7779_d_n8;
        locals.var_t0_dn9 = assign13430_e7779_d_n9;
        locals.var_t0_dn10 = assign13430_e7779_d_n10;
        locals.var_t0_dn11 = assign13430_e7779_d_n11;
        locals.var_t0_dn14 = assign13430_e7779_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13440_e7790, assign13440_e7790_d_n0, assign13440_e7790_d_n2, assign13440_e7790_d_n4, assign13440_e7790_d_n5, assign13440_e7790_d_n6, assign13440_e7790_d_n7, assign13440_e7790_d_n8, assign13440_e7790_d_n9, assign13440_e7790_d_n10, assign13440_e7790_d_n11, assign13440_e7790_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13440_e7788: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13440_e7788, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13440_e7790;
        locals.var_uc_depvmax_dn0 = assign13440_e7790_d_n0;
        locals.var_uc_depvmax_dn2 = assign13440_e7790_d_n2;
        locals.var_uc_depvmax_dn4 = assign13440_e7790_d_n4;
        locals.var_uc_depvmax_dn5 = assign13440_e7790_d_n5;
        locals.var_uc_depvmax_dn6 = assign13440_e7790_d_n6;
        locals.var_uc_depvmax_dn7 = assign13440_e7790_d_n7;
        locals.var_uc_depvmax_dn8 = assign13440_e7790_d_n8;
        locals.var_uc_depvmax_dn9 = assign13440_e7790_d_n9;
        locals.var_uc_depvmax_dn10 = assign13440_e7790_d_n10;
        locals.var_uc_depvmax_dn11 = assign13440_e7790_d_n11;
        locals.var_uc_depvmax_dn14 = assign13440_e7790_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let assign13460_e7798: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign13460_e7798;
        locals.var_guard297_rv = 0.0;

        let (assign13470_e7809, assign13470_e7809_d_n0, assign13470_e7809_d_n2, assign13470_e7809_d_n4, assign13470_e7809_d_n5, assign13470_e7809_d_n6, assign13470_e7809_d_n7, assign13470_e7809_d_n8, assign13470_e7809_d_n9, assign13470_e7809_d_n10, assign13470_e7809_d_n11, assign13470_e7809_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13470_e7809;
        locals.var_uc_depvmax_dn0 = assign13470_e7809_d_n0;
        locals.var_uc_depvmax_dn2 = assign13470_e7809_d_n2;
        locals.var_uc_depvmax_dn4 = assign13470_e7809_d_n4;
        locals.var_uc_depvmax_dn5 = assign13470_e7809_d_n5;
        locals.var_uc_depvmax_dn6 = assign13470_e7809_d_n6;
        locals.var_uc_depvmax_dn7 = assign13470_e7809_d_n7;
        locals.var_uc_depvmax_dn8 = assign13470_e7809_d_n8;
        locals.var_uc_depvmax_dn9 = assign13470_e7809_d_n9;
        locals.var_uc_depvmax_dn10 = assign13470_e7809_d_n10;
        locals.var_uc_depvmax_dn11 = assign13470_e7809_d_n11;
        locals.var_uc_depvmax_dn14 = assign13470_e7809_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign13480_e7822, assign13480_e7822_d_n0, assign13480_e7822_d_n2, assign13480_e7822_d_n4, assign13480_e7822_d_n5, assign13480_e7822_d_n6, assign13480_e7822_d_n7, assign13480_e7822_d_n8, assign13480_e7822_d_n9, assign13480_e7822_d_n10, assign13480_e7822_d_n11, assign13480_e7822_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13480_e7819: f64 = (locals.var_tratio).powf(p.p381);
        let assign13480_e7820: f64 = (locals.var_uc_depmue0 / assign13480_e7819);
        (assign13480_e7820, (((locals.var_uc_depmue0_dn0 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn2 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn4 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn5 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn6 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn7 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn8 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn9 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn10 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn11 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn14 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign13480_e7822;
        locals.var_uc_depmue0_dn0 = assign13480_e7822_d_n0;
        locals.var_uc_depmue0_dn2 = assign13480_e7822_d_n2;
        locals.var_uc_depmue0_dn4 = assign13480_e7822_d_n4;
        locals.var_uc_depmue0_dn5 = assign13480_e7822_d_n5;
        locals.var_uc_depmue0_dn6 = assign13480_e7822_d_n6;
        locals.var_uc_depmue0_dn7 = assign13480_e7822_d_n7;
        locals.var_uc_depmue0_dn8 = assign13480_e7822_d_n8;
        locals.var_uc_depmue0_dn9 = assign13480_e7822_d_n9;
        locals.var_uc_depmue0_dn10 = assign13480_e7822_d_n10;
        locals.var_uc_depmue0_dn11 = assign13480_e7822_d_n11;
        locals.var_uc_depmue0_dn14 = assign13480_e7822_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign13490_e7837, assign13490_e7837_d_n0, assign13490_e7837_d_n2, assign13490_e7837_d_n4, assign13490_e7837_d_n5, assign13490_e7837_d_n6, assign13490_e7837_d_n7, assign13490_e7837_d_n8, assign13490_e7837_d_n9, assign13490_e7837_d_n10, assign13490_e7837_d_n11, assign13490_e7837_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13490_e7833: f64 = (locals.var_tratio - 1.0);
        let assign13490_e7834: f64 = (p.p365 * assign13490_e7833);
        let assign13490_e7835: f64 = (p.p364 + assign13490_e7834);
        (assign13490_e7835, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn11), (p.p365 * locals.var_tratio_dn14),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn11, locals.var_uc_depwlp_dn14,)
    }
};
        locals.var_uc_depwlp = assign13490_e7837;
        locals.var_uc_depwlp_dn0 = assign13490_e7837_d_n0;
        locals.var_uc_depwlp_dn2 = assign13490_e7837_d_n2;
        locals.var_uc_depwlp_dn4 = assign13490_e7837_d_n4;
        locals.var_uc_depwlp_dn5 = assign13490_e7837_d_n5;
        locals.var_uc_depwlp_dn6 = assign13490_e7837_d_n6;
        locals.var_uc_depwlp_dn7 = assign13490_e7837_d_n7;
        locals.var_uc_depwlp_dn8 = assign13490_e7837_d_n8;
        locals.var_uc_depwlp_dn9 = assign13490_e7837_d_n9;
        locals.var_uc_depwlp_dn10 = assign13490_e7837_d_n10;
        locals.var_uc_depwlp_dn11 = assign13490_e7837_d_n11;
        locals.var_uc_depwlp_dn14 = assign13490_e7837_d_n14;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign13500_e7847, assign13500_e7847_d_n0, assign13500_e7847_d_n2, assign13500_e7847_d_n4, assign13500_e7847_d_n5, assign13500_e7847_d_n6, assign13500_e7847_d_n7, assign13500_e7847_d_n8, assign13500_e7847_d_n9, assign13500_e7847_d_n10, assign13500_e7847_d_n11, assign13500_e7847_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13500_e7847;
        locals.var_pb2n_dn0 = assign13500_e7847_d_n0;
        locals.var_pb2n_dn2 = assign13500_e7847_d_n2;
        locals.var_pb2n_dn4 = assign13500_e7847_d_n4;
        locals.var_pb2n_dn5 = assign13500_e7847_d_n5;
        locals.var_pb2n_dn6 = assign13500_e7847_d_n6;
        locals.var_pb2n_dn7 = assign13500_e7847_d_n7;
        locals.var_pb2n_dn8 = assign13500_e7847_d_n8;
        locals.var_pb2n_dn9 = assign13500_e7847_d_n9;
        locals.var_pb2n_dn10 = assign13500_e7847_d_n10;
        locals.var_pb2n_dn11 = assign13500_e7847_d_n11;
        locals.var_pb2n_dn14 = assign13500_e7847_d_n14;
        locals.var_pb2n_rv = 0.0;

        let (assign13510_e7866, assign13510_e7866_d_n0, assign13510_e7866_d_n2, assign13510_e7866_d_n4, assign13510_e7866_d_n5, assign13510_e7866_d_n6, assign13510_e7866_d_n7, assign13510_e7866_d_n8, assign13510_e7866_d_n9, assign13510_e7866_d_n10, assign13510_e7866_d_n11, assign13510_e7866_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        let assign13510_e7858: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign13510_e7860: f64 = (assign13510_e7858 * locals.var_nsub);
        let assign13510_e7862: f64 = (assign13510_e7860 / locals.var_nin);
        let assign13510_e7863: f64 = (assign13510_e7862).ln();
        let assign13510_e7864: f64 = (locals.var_beta_inv * assign13510_e7863);
        (assign13510_e7864, ((locals.var_beta_inv_dn0 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn0)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn2 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn2)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn4 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn4)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn5 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn5)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn6 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn6)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn7 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn7)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn8 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn8)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn9 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn9)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn10 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn10)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn11 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn11)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn14 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn14)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13510_e7866;
        locals.var_vbipn_dn0 = assign13510_e7866_d_n0;
        locals.var_vbipn_dn2 = assign13510_e7866_d_n2;
        locals.var_vbipn_dn4 = assign13510_e7866_d_n4;
        locals.var_vbipn_dn5 = assign13510_e7866_d_n5;
        locals.var_vbipn_dn6 = assign13510_e7866_d_n6;
        locals.var_vbipn_dn7 = assign13510_e7866_d_n7;
        locals.var_vbipn_dn8 = assign13510_e7866_d_n8;
        locals.var_vbipn_dn9 = assign13510_e7866_d_n9;
        locals.var_vbipn_dn10 = assign13510_e7866_d_n10;
        locals.var_vbipn_dn11 = assign13510_e7866_d_n11;
        locals.var_vbipn_dn14 = assign13510_e7866_d_n14;
        locals.var_vbipn_rv = 0.0;

        let (assign13520_e7876, assign13520_e7876_d_n0, assign13520_e7876_d_n2, assign13520_e7876_d_n4, assign13520_e7876_d_n5, assign13520_e7876_d_n6, assign13520_e7876_d_n7, assign13520_e7876_d_n8, assign13520_e7876_d_n9, assign13520_e7876_d_n10, assign13520_e7876_d_n11, assign13520_e7876_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13520_e7876;
        locals.var_depmphn0_dn0 = assign13520_e7876_d_n0;
        locals.var_depmphn0_dn2 = assign13520_e7876_d_n2;
        locals.var_depmphn0_dn4 = assign13520_e7876_d_n4;
        locals.var_depmphn0_dn5 = assign13520_e7876_d_n5;
        locals.var_depmphn0_dn6 = assign13520_e7876_d_n6;
        locals.var_depmphn0_dn7 = assign13520_e7876_d_n7;
        locals.var_depmphn0_dn8 = assign13520_e7876_d_n8;
        locals.var_depmphn0_dn9 = assign13520_e7876_d_n9;
        locals.var_depmphn0_dn10 = assign13520_e7876_d_n10;
        locals.var_depmphn0_dn11 = assign13520_e7876_d_n11;
        locals.var_depmphn0_dn14 = assign13520_e7876_d_n14;
        locals.var_depmphn0_rv = 0.0;

        let (assign13530_e7882, assign13530_e7882_d_n0, assign13530_e7882_d_n2, assign13530_e7882_d_n4, assign13530_e7882_d_n5, assign13530_e7882_d_n6, assign13530_e7882_d_n7, assign13530_e7882_d_n8, assign13530_e7882_d_n9, assign13530_e7882_d_n10, assign13530_e7882_d_n11, assign13530_e7882_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13530_e7880: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign13530_e7880, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn11 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn11)), ((locals.var_ptovr0_dn14 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn11, locals.var_ptovr_dn14,)
    }
};
        locals.var_ptovr = assign13530_e7882;
        locals.var_ptovr_dn0 = assign13530_e7882_d_n0;
        locals.var_ptovr_dn2 = assign13530_e7882_d_n2;
        locals.var_ptovr_dn4 = assign13530_e7882_d_n4;
        locals.var_ptovr_dn5 = assign13530_e7882_d_n5;
        locals.var_ptovr_dn6 = assign13530_e7882_d_n6;
        locals.var_ptovr_dn7 = assign13530_e7882_d_n7;
        locals.var_ptovr_dn8 = assign13530_e7882_d_n8;
        locals.var_ptovr_dn9 = assign13530_e7882_d_n9;
        locals.var_ptovr_dn10 = assign13530_e7882_d_n10;
        locals.var_ptovr_dn11 = assign13530_e7882_d_n11;
        locals.var_ptovr_dn14 = assign13530_e7882_d_n14;
        locals.var_ptovr_rv = 0.0;

        let (assign13540_e7888, assign13540_e7888_d_n0, assign13540_e7888_d_n2, assign13540_e7888_d_n4, assign13540_e7888_d_n5, assign13540_e7888_d_n6, assign13540_e7888_d_n7, assign13540_e7888_d_n8, assign13540_e7888_d_n9, assign13540_e7888_d_n10, assign13540_e7888_d_n11, assign13540_e7888_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13540_e7886: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13540_e7886, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13540_e7888;
        locals.var_t1_dn0 = assign13540_e7888_d_n0;
        locals.var_t1_dn2 = assign13540_e7888_d_n2;
        locals.var_t1_dn4 = assign13540_e7888_d_n4;
        locals.var_t1_dn5 = assign13540_e7888_d_n5;
        locals.var_t1_dn6 = assign13540_e7888_d_n6;
        locals.var_t1_dn7 = assign13540_e7888_d_n7;
        locals.var_t1_dn8 = assign13540_e7888_d_n8;
        locals.var_t1_dn9 = assign13540_e7888_d_n9;
        locals.var_t1_dn10 = assign13540_e7888_d_n10;
        locals.var_t1_dn11 = assign13540_e7888_d_n11;
        locals.var_t1_dn14 = assign13540_e7888_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13550_e7908, assign13550_e7908_d_n0, assign13550_e7908_d_n2, assign13550_e7908_d_n4, assign13550_e7908_d_n5, assign13550_e7908_d_n6, assign13550_e7908_d_n7, assign13550_e7908_d_n8, assign13550_e7908_d_n9, assign13550_e7908_d_n10, assign13550_e7908_d_n11, assign13550_e7908_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13550_e7893: f64 = (0.4 * locals.var_t1);
        let assign13550_e7894: f64 = (1.8 + assign13550_e7893);
        let assign13550_e7897: f64 = (0.1 * locals.var_t1);
        let assign13550_e7899: f64 = (assign13550_e7897 * locals.var_t1);
        let assign13550_e7900: f64 = (assign13550_e7894 + assign13550_e7899);
        let assign13550_e7904: f64 = (1.0 - locals.var_t1);
        let assign13550_e7905: f64 = (locals.var_uc_vtmp * assign13550_e7904);
        let assign13550_e7906: f64 = (assign13550_e7900 - assign13550_e7905);
        (assign13550_e7906, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn11))) - (locals.var_uc_vtmp * (-locals.var_t1_dn11))), (((0.4 * locals.var_t1_dn14) + (((0.1 * locals.var_t1_dn14) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn14))) - (locals.var_uc_vtmp * (-locals.var_t1_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13550_e7908;
        locals.var_t0_dn0 = assign13550_e7908_d_n0;
        locals.var_t0_dn2 = assign13550_e7908_d_n2;
        locals.var_t0_dn4 = assign13550_e7908_d_n4;
        locals.var_t0_dn5 = assign13550_e7908_d_n5;
        locals.var_t0_dn6 = assign13550_e7908_d_n6;
        locals.var_t0_dn7 = assign13550_e7908_d_n7;
        locals.var_t0_dn8 = assign13550_e7908_d_n8;
        locals.var_t0_dn9 = assign13550_e7908_d_n9;
        locals.var_t0_dn10 = assign13550_e7908_d_n10;
        locals.var_t0_dn11 = assign13550_e7908_d_n11;
        locals.var_t0_dn14 = assign13550_e7908_d_n14;
        locals.var_t0_rv = 0.0;

        let assign13560_e7911: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign13560_e7911;
        locals.var_guard298_rv = 0.0;

        let (assign13570_e7931, assign13570_e7931_d_n0, assign13570_e7931_d_n2, assign13570_e7931_d_n4, assign13570_e7931_d_n5, assign13570_e7931_d_n6, assign13570_e7931_d_n7, assign13570_e7931_d_n8, assign13570_e7931_d_n9, assign13570_e7931_d_n10, assign13570_e7931_d_n11, assign13570_e7931_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13570_e7917: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13570_e7919: f64 = (assign13570_e7917 / locals.var_t0);
        let assign13570_e7923: f64 = (p.p90 * locals.var_tdiff0);
        let assign13570_e7924: f64 = (1.0 + assign13570_e7923);
        let assign13570_e7927: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign13570_e7928: f64 = (assign13570_e7924 + assign13570_e7927);
        let assign13570_e7929: f64 = (assign13570_e7919 * assign13570_e7928);
        (assign13570_e7929, (((-((assign13570_e7917 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign13570_e7917 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign13570_e7917 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign13570_e7917 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign13570_e7917 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign13570_e7917 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign13570_e7917 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign13570_e7917 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign13570_e7917 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign13570_e7917 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn11) + (p.p91 * locals.var_tdiff0_2_dn11)))), (((-((assign13570_e7917 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn14) + (p.p91 * locals.var_tdiff0_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13570_e7931;
        locals.var_vmaxeff_dn0 = assign13570_e7931_d_n0;
        locals.var_vmaxeff_dn2 = assign13570_e7931_d_n2;
        locals.var_vmaxeff_dn4 = assign13570_e7931_d_n4;
        locals.var_vmaxeff_dn5 = assign13570_e7931_d_n5;
        locals.var_vmaxeff_dn6 = assign13570_e7931_d_n6;
        locals.var_vmaxeff_dn7 = assign13570_e7931_d_n7;
        locals.var_vmaxeff_dn8 = assign13570_e7931_d_n8;
        locals.var_vmaxeff_dn9 = assign13570_e7931_d_n9;
        locals.var_vmaxeff_dn10 = assign13570_e7931_d_n10;
        locals.var_vmaxeff_dn11 = assign13570_e7931_d_n11;
        locals.var_vmaxeff_dn14 = assign13570_e7931_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let (assign13580_e7952, assign13580_e7952_d_n0, assign13580_e7952_d_n2, assign13580_e7952_d_n4, assign13580_e7952_d_n5, assign13580_e7952_d_n6, assign13580_e7952_d_n7, assign13580_e7952_d_n8, assign13580_e7952_d_n9, assign13580_e7952_d_n10, assign13580_e7952_d_n11, assign13580_e7952_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13580_e7938: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13580_e7940: f64 = (assign13580_e7938 / locals.var_t0);
        let assign13580_e7944: f64 = (p.p90 * locals.var_tdiff);
        let assign13580_e7945: f64 = (1.0 + assign13580_e7944);
        let assign13580_e7948: f64 = (p.p91 * locals.var_tdiff_2);
        let assign13580_e7949: f64 = (assign13580_e7945 + assign13580_e7948);
        let assign13580_e7950: f64 = (assign13580_e7940 * assign13580_e7949);
        (assign13580_e7950, (((-((assign13580_e7938 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign13580_e7938 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign13580_e7938 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign13580_e7938 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign13580_e7938 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign13580_e7938 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign13580_e7938 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign13580_e7938 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign13580_e7938 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign13580_e7938 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn11) + (p.p91 * locals.var_tdiff_2_dn11)))), (((-((assign13580_e7938 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn14) + (p.p91 * locals.var_tdiff_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13580_e7952;
        locals.var_vmaxeff_dn0 = assign13580_e7952_d_n0;
        locals.var_vmaxeff_dn2 = assign13580_e7952_d_n2;
        locals.var_vmaxeff_dn4 = assign13580_e7952_d_n4;
        locals.var_vmaxeff_dn5 = assign13580_e7952_d_n5;
        locals.var_vmaxeff_dn6 = assign13580_e7952_d_n6;
        locals.var_vmaxeff_dn7 = assign13580_e7952_d_n7;
        locals.var_vmaxeff_dn8 = assign13580_e7952_d_n8;
        locals.var_vmaxeff_dn9 = assign13580_e7952_d_n9;
        locals.var_vmaxeff_dn10 = assign13580_e7952_d_n10;
        locals.var_vmaxeff_dn11 = assign13580_e7952_d_n11;
        locals.var_vmaxeff_dn14 = assign13580_e7952_d_n14;
        locals.var_vmaxeff_rv = 0.0;

        let assign13600_e7960: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign13600_e7960;
        locals.var_guard300_rv = 0.0;

        let (assign13610_e7976, assign13610_e7976_d_n0, assign13610_e7976_d_n2, assign13610_e7976_d_n4, assign13610_e7976_d_n5, assign13610_e7976_d_n6, assign13610_e7976_d_n7, assign13610_e7976_d_n8, assign13610_e7976_d_n9, assign13610_e7976_d_n10, assign13610_e7976_d_n11, assign13610_e7976_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13610_e7968: f64 = (p.p324 * locals.var_tdiff0);
        let assign13610_e7969: f64 = (1.0 + assign13610_e7968);
        let assign13610_e7972: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign13610_e7973: f64 = (assign13610_e7969 + assign13610_e7972);
        let assign13610_e7974: f64 = (locals.var_ninvd0 * assign13610_e7973);
        (assign13610_e7974, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn11) + (p.p325 * locals.var_tdiff0_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn14) + (p.p325 * locals.var_tdiff0_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13610_e7976;
        locals.var_ninvde_dn0 = assign13610_e7976_d_n0;
        locals.var_ninvde_dn2 = assign13610_e7976_d_n2;
        locals.var_ninvde_dn4 = assign13610_e7976_d_n4;
        locals.var_ninvde_dn5 = assign13610_e7976_d_n5;
        locals.var_ninvde_dn6 = assign13610_e7976_d_n6;
        locals.var_ninvde_dn7 = assign13610_e7976_d_n7;
        locals.var_ninvde_dn8 = assign13610_e7976_d_n8;
        locals.var_ninvde_dn9 = assign13610_e7976_d_n9;
        locals.var_ninvde_dn10 = assign13610_e7976_d_n10;
        locals.var_ninvde_dn11 = assign13610_e7976_d_n11;
        locals.var_ninvde_dn14 = assign13610_e7976_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign13620_e7990, assign13620_e7990_d_n0, assign13620_e7990_d_n2, assign13620_e7990_d_n4, assign13620_e7990_d_n5, assign13620_e7990_d_n6, assign13620_e7990_d_n7, assign13620_e7990_d_n8, assign13620_e7990_d_n9, assign13620_e7990_d_n10, assign13620_e7990_d_n11, assign13620_e7990_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13620_e7983: f64 = (p.p390 * locals.var_tdiff0);
        let assign13620_e7984: f64 = (1.0 + assign13620_e7983);
        let assign13620_e7987: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign13620_e7988: f64 = (assign13620_e7984 + assign13620_e7987);
        (assign13620_e7988, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn11) + (p.p391 * locals.var_tdiff0_2_dn11)), ((p.p390 * locals.var_tdiff0_dn14) + (p.p391 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13620_e7990;
        locals.var_t1_dn0 = assign13620_e7990_d_n0;
        locals.var_t1_dn2 = assign13620_e7990_d_n2;
        locals.var_t1_dn4 = assign13620_e7990_d_n4;
        locals.var_t1_dn5 = assign13620_e7990_d_n5;
        locals.var_t1_dn6 = assign13620_e7990_d_n6;
        locals.var_t1_dn7 = assign13620_e7990_d_n7;
        locals.var_t1_dn8 = assign13620_e7990_d_n8;
        locals.var_t1_dn9 = assign13620_e7990_d_n9;
        locals.var_t1_dn10 = assign13620_e7990_d_n10;
        locals.var_t1_dn11 = assign13620_e7990_d_n11;
        locals.var_t1_dn14 = assign13620_e7990_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13630_e7998, assign13630_e7998_d_n0, assign13630_e7998_d_n2, assign13630_e7998_d_n4, assign13630_e7998_d_n5, assign13630_e7998_d_n6, assign13630_e7998_d_n7, assign13630_e7998_d_n8, assign13630_e7998_d_n9, assign13630_e7998_d_n10, assign13630_e7998_d_n11, assign13630_e7998_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13630_e7996: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13630_e7996, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13630_e7998;
        locals.var_ninvdecres_dn0 = assign13630_e7998_d_n0;
        locals.var_ninvdecres_dn2 = assign13630_e7998_d_n2;
        locals.var_ninvdecres_dn4 = assign13630_e7998_d_n4;
        locals.var_ninvdecres_dn5 = assign13630_e7998_d_n5;
        locals.var_ninvdecres_dn6 = assign13630_e7998_d_n6;
        locals.var_ninvdecres_dn7 = assign13630_e7998_d_n7;
        locals.var_ninvdecres_dn8 = assign13630_e7998_d_n8;
        locals.var_ninvdecres_dn9 = assign13630_e7998_d_n9;
        locals.var_ninvdecres_dn10 = assign13630_e7998_d_n10;
        locals.var_ninvdecres_dn11 = assign13630_e7998_d_n11;
        locals.var_ninvdecres_dn14 = assign13630_e7998_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let (assign13640_e8006, assign13640_e8006_d_n0, assign13640_e8006_d_n2, assign13640_e8006_d_n4, assign13640_e8006_d_n5, assign13640_e8006_d_n6, assign13640_e8006_d_n7, assign13640_e8006_d_n8, assign13640_e8006_d_n9, assign13640_e8006_d_n10, assign13640_e8006_d_n11, assign13640_e8006_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13640_e8004: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13640_e8004, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13640_e8006;
        locals.var_ninvdehres_dn0 = assign13640_e8006_d_n0;
        locals.var_ninvdehres_dn2 = assign13640_e8006_d_n2;
        locals.var_ninvdehres_dn4 = assign13640_e8006_d_n4;
        locals.var_ninvdehres_dn5 = assign13640_e8006_d_n5;
        locals.var_ninvdehres_dn6 = assign13640_e8006_d_n6;
        locals.var_ninvdehres_dn7 = assign13640_e8006_d_n7;
        locals.var_ninvdehres_dn8 = assign13640_e8006_d_n8;
        locals.var_ninvdehres_dn9 = assign13640_e8006_d_n9;
        locals.var_ninvdehres_dn10 = assign13640_e8006_d_n10;
        locals.var_ninvdehres_dn11 = assign13640_e8006_d_n11;
        locals.var_ninvdehres_dn14 = assign13640_e8006_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13650_e8023, assign13650_e8023_d_n0, assign13650_e8023_d_n2, assign13650_e8023_d_n4, assign13650_e8023_d_n5, assign13650_e8023_d_n6, assign13650_e8023_d_n7, assign13650_e8023_d_n8, assign13650_e8023_d_n9, assign13650_e8023_d_n10, assign13650_e8023_d_n11, assign13650_e8023_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13650_e8015: f64 = (p.p324 * locals.var_tdiff);
        let assign13650_e8016: f64 = (1.0 + assign13650_e8015);
        let assign13650_e8019: f64 = (p.p325 * locals.var_tdiff_2);
        let assign13650_e8020: f64 = (assign13650_e8016 + assign13650_e8019);
        let assign13650_e8021: f64 = (locals.var_ninvd0 * assign13650_e8020);
        (assign13650_e8021, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn11) + (p.p325 * locals.var_tdiff_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn14) + (p.p325 * locals.var_tdiff_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13650_e8023;
        locals.var_ninvde_dn0 = assign13650_e8023_d_n0;
        locals.var_ninvde_dn2 = assign13650_e8023_d_n2;
        locals.var_ninvde_dn4 = assign13650_e8023_d_n4;
        locals.var_ninvde_dn5 = assign13650_e8023_d_n5;
        locals.var_ninvde_dn6 = assign13650_e8023_d_n6;
        locals.var_ninvde_dn7 = assign13650_e8023_d_n7;
        locals.var_ninvde_dn8 = assign13650_e8023_d_n8;
        locals.var_ninvde_dn9 = assign13650_e8023_d_n9;
        locals.var_ninvde_dn10 = assign13650_e8023_d_n10;
        locals.var_ninvde_dn11 = assign13650_e8023_d_n11;
        locals.var_ninvde_dn14 = assign13650_e8023_d_n14;
        locals.var_ninvde_rv = 0.0;

        let (assign13660_e8038, assign13660_e8038_d_n0, assign13660_e8038_d_n2, assign13660_e8038_d_n4, assign13660_e8038_d_n5, assign13660_e8038_d_n6, assign13660_e8038_d_n7, assign13660_e8038_d_n8, assign13660_e8038_d_n9, assign13660_e8038_d_n10, assign13660_e8038_d_n11, assign13660_e8038_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13660_e8031: f64 = (p.p390 * locals.var_tdiff);
        let assign13660_e8032: f64 = (1.0 + assign13660_e8031);
        let assign13660_e8035: f64 = (p.p391 * locals.var_tdiff_2);
        let assign13660_e8036: f64 = (assign13660_e8032 + assign13660_e8035);
        (assign13660_e8036, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn11) + (p.p391 * locals.var_tdiff_2_dn11)), ((p.p390 * locals.var_tdiff_dn14) + (p.p391 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13660_e8038;
        locals.var_t1_dn0 = assign13660_e8038_d_n0;
        locals.var_t1_dn2 = assign13660_e8038_d_n2;
        locals.var_t1_dn4 = assign13660_e8038_d_n4;
        locals.var_t1_dn5 = assign13660_e8038_d_n5;
        locals.var_t1_dn6 = assign13660_e8038_d_n6;
        locals.var_t1_dn7 = assign13660_e8038_d_n7;
        locals.var_t1_dn8 = assign13660_e8038_d_n8;
        locals.var_t1_dn9 = assign13660_e8038_d_n9;
        locals.var_t1_dn10 = assign13660_e8038_d_n10;
        locals.var_t1_dn11 = assign13660_e8038_d_n11;
        locals.var_t1_dn14 = assign13660_e8038_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13670_e8047, assign13670_e8047_d_n0, assign13670_e8047_d_n2, assign13670_e8047_d_n4, assign13670_e8047_d_n5, assign13670_e8047_d_n6, assign13670_e8047_d_n7, assign13670_e8047_d_n8, assign13670_e8047_d_n9, assign13670_e8047_d_n10, assign13670_e8047_d_n11, assign13670_e8047_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13670_e8045: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13670_e8045, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13670_e8047;
        locals.var_ninvdecres_dn0 = assign13670_e8047_d_n0;
        locals.var_ninvdecres_dn2 = assign13670_e8047_d_n2;
        locals.var_ninvdecres_dn4 = assign13670_e8047_d_n4;
        locals.var_ninvdecres_dn5 = assign13670_e8047_d_n5;
        locals.var_ninvdecres_dn6 = assign13670_e8047_d_n6;
        locals.var_ninvdecres_dn7 = assign13670_e8047_d_n7;
        locals.var_ninvdecres_dn8 = assign13670_e8047_d_n8;
        locals.var_ninvdecres_dn9 = assign13670_e8047_d_n9;
        locals.var_ninvdecres_dn10 = assign13670_e8047_d_n10;
        locals.var_ninvdecres_dn11 = assign13670_e8047_d_n11;
        locals.var_ninvdecres_dn14 = assign13670_e8047_d_n14;
        locals.var_ninvdecres_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13680_e8056, assign13680_e8056_d_n0, assign13680_e8056_d_n2, assign13680_e8056_d_n4, assign13680_e8056_d_n5, assign13680_e8056_d_n6, assign13680_e8056_d_n7, assign13680_e8056_d_n8, assign13680_e8056_d_n9, assign13680_e8056_d_n10, assign13680_e8056_d_n11, assign13680_e8056_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13680_e8054: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13680_e8054, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13680_e8056;
        locals.var_ninvdehres_dn0 = assign13680_e8056_d_n0;
        locals.var_ninvdehres_dn2 = assign13680_e8056_d_n2;
        locals.var_ninvdehres_dn4 = assign13680_e8056_d_n4;
        locals.var_ninvdehres_dn5 = assign13680_e8056_d_n5;
        locals.var_ninvdehres_dn6 = assign13680_e8056_d_n6;
        locals.var_ninvdehres_dn7 = assign13680_e8056_d_n7;
        locals.var_ninvdehres_dn8 = assign13680_e8056_d_n8;
        locals.var_ninvdehres_dn9 = assign13680_e8056_d_n9;
        locals.var_ninvdehres_dn10 = assign13680_e8056_d_n10;
        locals.var_ninvdehres_dn11 = assign13680_e8056_d_n11;
        locals.var_ninvdehres_dn14 = assign13680_e8056_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let assign13700_e8064: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign13700_e8064;
        locals.var_guard302_rv = 0.0;

        let (assign13710_e8070, assign13710_e8070_d_n0, assign13710_e8070_d_n2, assign13710_e8070_d_n4, assign13710_e8070_d_n5, assign13710_e8070_d_n6, assign13710_e8070_d_n7, assign13710_e8070_d_n8, assign13710_e8070_d_n9, assign13710_e8070_d_n10, assign13710_e8070_d_n11, assign13710_e8070_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13710_e8070;
        locals.var_ninvde_dn0 = assign13710_e8070_d_n0;
        locals.var_ninvde_dn2 = assign13710_e8070_d_n2;
        locals.var_ninvde_dn4 = assign13710_e8070_d_n4;
        locals.var_ninvde_dn5 = assign13710_e8070_d_n5;
        locals.var_ninvde_dn6 = assign13710_e8070_d_n6;
        locals.var_ninvde_dn7 = assign13710_e8070_d_n7;
        locals.var_ninvde_dn8 = assign13710_e8070_d_n8;
        locals.var_ninvde_dn9 = assign13710_e8070_d_n9;
        locals.var_ninvde_dn10 = assign13710_e8070_d_n10;
        locals.var_ninvde_dn11 = assign13710_e8070_d_n11;
        locals.var_ninvde_dn14 = assign13710_e8070_d_n14;
        locals.var_ninvde_rv = 0.0;

        let assign13730_e8078: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign13730_e8078;
        locals.var_guard304_rv = 0.0;

        let (assign13740_e8084, assign13740_e8084_d_n0, assign13740_e8084_d_n2, assign13740_e8084_d_n4, assign13740_e8084_d_n5, assign13740_e8084_d_n6, assign13740_e8084_d_n7, assign13740_e8084_d_n8, assign13740_e8084_d_n9, assign13740_e8084_d_n10, assign13740_e8084_d_n11, assign13740_e8084_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13740_e8084;
        locals.var_ninvdecres_dn0 = assign13740_e8084_d_n0;
        locals.var_ninvdecres_dn2 = assign13740_e8084_d_n2;
        locals.var_ninvdecres_dn4 = assign13740_e8084_d_n4;
        locals.var_ninvdecres_dn5 = assign13740_e8084_d_n5;
        locals.var_ninvdecres_dn6 = assign13740_e8084_d_n6;
        locals.var_ninvdecres_dn7 = assign13740_e8084_d_n7;
        locals.var_ninvdecres_dn8 = assign13740_e8084_d_n8;
        locals.var_ninvdecres_dn9 = assign13740_e8084_d_n9;
        locals.var_ninvdecres_dn10 = assign13740_e8084_d_n10;
        locals.var_ninvdecres_dn11 = assign13740_e8084_d_n11;
        locals.var_ninvdecres_dn14 = assign13740_e8084_d_n14;
        locals.var_ninvdecres_rv = 0.0;

        let assign13760_e8092: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign13760_e8092;
        locals.var_guard306_rv = 0.0;

        let (assign13770_e8098, assign13770_e8098_d_n0, assign13770_e8098_d_n2, assign13770_e8098_d_n4, assign13770_e8098_d_n5, assign13770_e8098_d_n6, assign13770_e8098_d_n7, assign13770_e8098_d_n8, assign13770_e8098_d_n9, assign13770_e8098_d_n10, assign13770_e8098_d_n11, assign13770_e8098_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard306 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13770_e8098;
        locals.var_ninvdehres_dn0 = assign13770_e8098_d_n0;
        locals.var_ninvdehres_dn2 = assign13770_e8098_d_n2;
        locals.var_ninvdehres_dn4 = assign13770_e8098_d_n4;
        locals.var_ninvdehres_dn5 = assign13770_e8098_d_n5;
        locals.var_ninvdehres_dn6 = assign13770_e8098_d_n6;
        locals.var_ninvdehres_dn7 = assign13770_e8098_d_n7;
        locals.var_ninvdehres_dn8 = assign13770_e8098_d_n8;
        locals.var_ninvdehres_dn9 = assign13770_e8098_d_n9;
        locals.var_ninvdehres_dn10 = assign13770_e8098_d_n10;
        locals.var_ninvdehres_dn11 = assign13770_e8098_d_n11;
        locals.var_ninvdehres_dn14 = assign13770_e8098_d_n14;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13780_e8114, assign13780_e8114_d_n0, assign13780_e8114_d_n2, assign13780_e8114_d_n4, assign13780_e8114_d_n5, assign13780_e8114_d_n6, assign13780_e8114_d_n7, assign13780_e8114_d_n8, assign13780_e8114_d_n9, assign13780_e8114_d_n10, assign13780_e8114_d_n11, assign13780_e8114_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) {
        let assign13780_e8105: f64 = (p.p328 * locals.var_tdiff0);
        let assign13780_e8106: f64 = (locals.var_uc_rth0 + assign13780_e8105);
        let assign13780_e8109: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign13780_e8110: f64 = (assign13780_e8106 + assign13780_e8109);
        let assign13780_e8112: f64 = (assign13780_e8110 * locals.var_rthtemp0);
        (assign13780_e8112, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn11) + (p.p329 * locals.var_tdiff0_2_dn11)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn14) + (p.p329 * locals.var_tdiff0_2_dn14)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13780_e8114;
        locals.var_rth_dn0 = assign13780_e8114_d_n0;
        locals.var_rth_dn2 = assign13780_e8114_d_n2;
        locals.var_rth_dn4 = assign13780_e8114_d_n4;
        locals.var_rth_dn5 = assign13780_e8114_d_n5;
        locals.var_rth_dn6 = assign13780_e8114_d_n6;
        locals.var_rth_dn7 = assign13780_e8114_d_n7;
        locals.var_rth_dn8 = assign13780_e8114_d_n8;
        locals.var_rth_dn9 = assign13780_e8114_d_n9;
        locals.var_rth_dn10 = assign13780_e8114_d_n10;
        locals.var_rth_dn11 = assign13780_e8114_d_n11;
        locals.var_rth_dn14 = assign13780_e8114_d_n14;
        locals.var_rth_rv = 0.0;

        let assign13800_e8122: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign13800_e8122;
        locals.var_guard308_rv = 0.0;

        let (assign13810_e8130, assign13810_e8130_d_n0, assign13810_e8130_d_n2, assign13810_e8130_d_n4, assign13810_e8130_d_n5, assign13810_e8130_d_n6, assign13810_e8130_d_n7, assign13810_e8130_d_n8, assign13810_e8130_d_n9, assign13810_e8130_d_n10, assign13810_e8130_d_n11, assign13810_e8130_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13810_e8130;
        locals.var_rth_dn0 = assign13810_e8130_d_n0;
        locals.var_rth_dn2 = assign13810_e8130_d_n2;
        locals.var_rth_dn4 = assign13810_e8130_d_n4;
        locals.var_rth_dn5 = assign13810_e8130_d_n5;
        locals.var_rth_dn6 = assign13810_e8130_d_n6;
        locals.var_rth_dn7 = assign13810_e8130_d_n7;
        locals.var_rth_dn8 = assign13810_e8130_d_n8;
        locals.var_rth_dn9 = assign13810_e8130_d_n9;
        locals.var_rth_dn10 = assign13810_e8130_d_n10;
        locals.var_rth_dn11 = assign13810_e8130_d_n11;
        locals.var_rth_dn14 = assign13810_e8130_d_n14;
        locals.var_rth_rv = 0.0;

        let (assign13820_e8142, assign13820_e8142_d_n0, assign13820_e8142_d_n2, assign13820_e8142_d_n4, assign13820_e8142_d_n5, assign13820_e8142_d_n6, assign13820_e8142_d_n7, assign13820_e8142_d_n8, assign13820_e8142_d_n9, assign13820_e8142_d_n10, assign13820_e8142_d_n11, assign13820_e8142_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13820_e8135: f64 = (p.p330 * locals.var_tdiff0);
        let assign13820_e8136: f64 = (locals.var_uc_powrat + assign13820_e8135);
        let assign13820_e8139: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign13820_e8140: f64 = (assign13820_e8136 + assign13820_e8139);
        (assign13820_e8140, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn11) + (p.p331 * locals.var_tdiff0_2_dn11)), ((p.p330 * locals.var_tdiff0_dn14) + (p.p331 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13820_e8142;
        locals.var_t2_dn0 = assign13820_e8142_d_n0;
        locals.var_t2_dn2 = assign13820_e8142_d_n2;
        locals.var_t2_dn4 = assign13820_e8142_d_n4;
        locals.var_t2_dn5 = assign13820_e8142_d_n5;
        locals.var_t2_dn6 = assign13820_e8142_d_n6;
        locals.var_t2_dn7 = assign13820_e8142_d_n7;
        locals.var_t2_dn8 = assign13820_e8142_d_n8;
        locals.var_t2_dn9 = assign13820_e8142_d_n9;
        locals.var_t2_dn10 = assign13820_e8142_d_n10;
        locals.var_t2_dn11 = assign13820_e8142_d_n11;
        locals.var_t2_dn14 = assign13820_e8142_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13830_e8150, assign13830_e8150_d_n0, assign13830_e8150_d_n2, assign13830_e8150_d_n4, assign13830_e8150_d_n5, assign13830_e8150_d_n6, assign13830_e8150_d_n7, assign13830_e8150_d_n8, assign13830_e8150_d_n9, assign13830_e8150_d_n10, assign13830_e8150_d_n11, assign13830_e8150_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13830_e8146: f64 = locals.var_t2;
        let assign13830_e8148: f64 = (assign13830_e8146 - 0.05);
        (assign13830_e8148, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13830_e8150;
        locals.var_tmf1_dn0 = assign13830_e8150_d_n0;
        locals.var_tmf1_dn2 = assign13830_e8150_d_n2;
        locals.var_tmf1_dn4 = assign13830_e8150_d_n4;
        locals.var_tmf1_dn5 = assign13830_e8150_d_n5;
        locals.var_tmf1_dn6 = assign13830_e8150_d_n6;
        locals.var_tmf1_dn7 = assign13830_e8150_d_n7;
        locals.var_tmf1_dn8 = assign13830_e8150_d_n8;
        locals.var_tmf1_dn9 = assign13830_e8150_d_n9;
        locals.var_tmf1_dn10 = assign13830_e8150_d_n10;
        locals.var_tmf1_dn11 = assign13830_e8150_d_n11;
        locals.var_tmf1_dn14 = assign13830_e8150_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign13840_e8158, assign13840_e8158_d_n0, assign13840_e8158_d_n2, assign13840_e8158_d_n4, assign13840_e8158_d_n5, assign13840_e8158_d_n6, assign13840_e8158_d_n7, assign13840_e8158_d_n8, assign13840_e8158_d_n9, assign13840_e8158_d_n10, assign13840_e8158_d_n11, assign13840_e8158_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13840_e8158;
        locals.var_tmf2_dn0 = assign13840_e8158_d_n0;
        locals.var_tmf2_dn2 = assign13840_e8158_d_n2;
        locals.var_tmf2_dn4 = assign13840_e8158_d_n4;
        locals.var_tmf2_dn5 = assign13840_e8158_d_n5;
        locals.var_tmf2_dn6 = assign13840_e8158_d_n6;
        locals.var_tmf2_dn7 = assign13840_e8158_d_n7;
        locals.var_tmf2_dn8 = assign13840_e8158_d_n8;
        locals.var_tmf2_dn9 = assign13840_e8158_d_n9;
        locals.var_tmf2_dn10 = assign13840_e8158_d_n10;
        locals.var_tmf2_dn11 = assign13840_e8158_d_n11;
        locals.var_tmf2_dn14 = assign13840_e8158_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13850_e8168, assign13850_e8168_d_n0, assign13850_e8168_d_n2, assign13850_e8168_d_n4, assign13850_e8168_d_n5, assign13850_e8168_d_n6, assign13850_e8168_d_n7, assign13850_e8168_d_n8, assign13850_e8168_d_n9, assign13850_e8168_d_n10, assign13850_e8168_d_n11, assign13850_e8168_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13850_e8165: f64 = (-locals.var_tmf2);
                (assign13850_e8165, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13850_e8168;
        locals.var_tmf2_dn0 = assign13850_e8168_d_n0;
        locals.var_tmf2_dn2 = assign13850_e8168_d_n2;
        locals.var_tmf2_dn4 = assign13850_e8168_d_n4;
        locals.var_tmf2_dn5 = assign13850_e8168_d_n5;
        locals.var_tmf2_dn6 = assign13850_e8168_d_n6;
        locals.var_tmf2_dn7 = assign13850_e8168_d_n7;
        locals.var_tmf2_dn8 = assign13850_e8168_d_n8;
        locals.var_tmf2_dn9 = assign13850_e8168_d_n9;
        locals.var_tmf2_dn10 = assign13850_e8168_d_n10;
        locals.var_tmf2_dn11 = assign13850_e8168_d_n11;
        locals.var_tmf2_dn14 = assign13850_e8168_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13860_e8177, assign13860_e8177_d_n0, assign13860_e8177_d_n2, assign13860_e8177_d_n4, assign13860_e8177_d_n5, assign13860_e8177_d_n6, assign13860_e8177_d_n7, assign13860_e8177_d_n8, assign13860_e8177_d_n9, assign13860_e8177_d_n10, assign13860_e8177_d_n11, assign13860_e8177_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13860_e8172: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13860_e8174: f64 = (assign13860_e8172 + locals.var_tmf2);
        let assign13860_e8175: f64 = (assign13860_e8174).sqrt();
        (assign13860_e8175, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13860_e8175)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13860_e8177;
        locals.var_tmf2_dn0 = assign13860_e8177_d_n0;
        locals.var_tmf2_dn2 = assign13860_e8177_d_n2;
        locals.var_tmf2_dn4 = assign13860_e8177_d_n4;
        locals.var_tmf2_dn5 = assign13860_e8177_d_n5;
        locals.var_tmf2_dn6 = assign13860_e8177_d_n6;
        locals.var_tmf2_dn7 = assign13860_e8177_d_n7;
        locals.var_tmf2_dn8 = assign13860_e8177_d_n8;
        locals.var_tmf2_dn9 = assign13860_e8177_d_n9;
        locals.var_tmf2_dn10 = assign13860_e8177_d_n10;
        locals.var_tmf2_dn11 = assign13860_e8177_d_n11;
        locals.var_tmf2_dn14 = assign13860_e8177_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13870_e8187, assign13870_e8187_d_n0, assign13870_e8187_d_n2, assign13870_e8187_d_n4, assign13870_e8187_d_n5, assign13870_e8187_d_n6, assign13870_e8187_d_n7, assign13870_e8187_d_n8, assign13870_e8187_d_n9, assign13870_e8187_d_n10, assign13870_e8187_d_n11, assign13870_e8187_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13870_e8183: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13870_e8184: f64 = (1.0 + assign13870_e8183);
        let assign13870_e8185: f64 = (0.5 * assign13870_e8184);
        (assign13870_e8185, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13870_e8187;
        locals.var_t0_dn0 = assign13870_e8187_d_n0;
        locals.var_t0_dn2 = assign13870_e8187_d_n2;
        locals.var_t0_dn4 = assign13870_e8187_d_n4;
        locals.var_t0_dn5 = assign13870_e8187_d_n5;
        locals.var_t0_dn6 = assign13870_e8187_d_n6;
        locals.var_t0_dn7 = assign13870_e8187_d_n7;
        locals.var_t0_dn8 = assign13870_e8187_d_n8;
        locals.var_t0_dn9 = assign13870_e8187_d_n9;
        locals.var_t0_dn10 = assign13870_e8187_d_n10;
        locals.var_t0_dn11 = assign13870_e8187_d_n11;
        locals.var_t0_dn14 = assign13870_e8187_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13880_e8197, assign13880_e8197_d_n0, assign13880_e8197_d_n2, assign13880_e8197_d_n4, assign13880_e8197_d_n5, assign13880_e8197_d_n6, assign13880_e8197_d_n7, assign13880_e8197_d_n8, assign13880_e8197_d_n9, assign13880_e8197_d_n10, assign13880_e8197_d_n11, assign13880_e8197_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13880_e8193: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13880_e8194: f64 = (0.5 * assign13880_e8193);
        let assign13880_e8195: f64 = assign13880_e8194;
        (assign13880_e8195, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13880_e8197;
        locals.var_t2_dn0 = assign13880_e8197_d_n0;
        locals.var_t2_dn2 = assign13880_e8197_d_n2;
        locals.var_t2_dn4 = assign13880_e8197_d_n4;
        locals.var_t2_dn5 = assign13880_e8197_d_n5;
        locals.var_t2_dn6 = assign13880_e8197_d_n6;
        locals.var_t2_dn7 = assign13880_e8197_d_n7;
        locals.var_t2_dn8 = assign13880_e8197_d_n8;
        locals.var_t2_dn9 = assign13880_e8197_d_n9;
        locals.var_t2_dn10 = assign13880_e8197_d_n10;
        locals.var_t2_dn11 = assign13880_e8197_d_n11;
        locals.var_t2_dn14 = assign13880_e8197_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13890_e8205, assign13890_e8205_d_n0, assign13890_e8205_d_n2, assign13890_e8205_d_n4, assign13890_e8205_d_n5, assign13890_e8205_d_n6, assign13890_e8205_d_n7, assign13890_e8205_d_n8, assign13890_e8205_d_n9, assign13890_e8205_d_n10, assign13890_e8205_d_n11, assign13890_e8205_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13890_e8201: f64 = (1.0 - locals.var_t2);
        let assign13890_e8203: f64 = (assign13890_e8201 - 0.05);
        (assign13890_e8203, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13890_e8205;
        locals.var_tmf1_dn0 = assign13890_e8205_d_n0;
        locals.var_tmf1_dn2 = assign13890_e8205_d_n2;
        locals.var_tmf1_dn4 = assign13890_e8205_d_n4;
        locals.var_tmf1_dn5 = assign13890_e8205_d_n5;
        locals.var_tmf1_dn6 = assign13890_e8205_d_n6;
        locals.var_tmf1_dn7 = assign13890_e8205_d_n7;
        locals.var_tmf1_dn8 = assign13890_e8205_d_n8;
        locals.var_tmf1_dn9 = assign13890_e8205_d_n9;
        locals.var_tmf1_dn10 = assign13890_e8205_d_n10;
        locals.var_tmf1_dn11 = assign13890_e8205_d_n11;
        locals.var_tmf1_dn14 = assign13890_e8205_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign13900_e8213, assign13900_e8213_d_n0, assign13900_e8213_d_n2, assign13900_e8213_d_n4, assign13900_e8213_d_n5, assign13900_e8213_d_n6, assign13900_e8213_d_n7, assign13900_e8213_d_n8, assign13900_e8213_d_n9, assign13900_e8213_d_n10, assign13900_e8213_d_n11, assign13900_e8213_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13900_e8209: f64 = 4.0;
        let assign13900_e8211: f64 = (assign13900_e8209 * 0.05);
        (assign13900_e8211, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13900_e8213;
        locals.var_tmf2_dn0 = assign13900_e8213_d_n0;
        locals.var_tmf2_dn2 = assign13900_e8213_d_n2;
        locals.var_tmf2_dn4 = assign13900_e8213_d_n4;
        locals.var_tmf2_dn5 = assign13900_e8213_d_n5;
        locals.var_tmf2_dn6 = assign13900_e8213_d_n6;
        locals.var_tmf2_dn7 = assign13900_e8213_d_n7;
        locals.var_tmf2_dn8 = assign13900_e8213_d_n8;
        locals.var_tmf2_dn9 = assign13900_e8213_d_n9;
        locals.var_tmf2_dn10 = assign13900_e8213_d_n10;
        locals.var_tmf2_dn11 = assign13900_e8213_d_n11;
        locals.var_tmf2_dn14 = assign13900_e8213_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13910_e8223, assign13910_e8223_d_n0, assign13910_e8223_d_n2, assign13910_e8223_d_n4, assign13910_e8223_d_n5, assign13910_e8223_d_n6, assign13910_e8223_d_n7, assign13910_e8223_d_n8, assign13910_e8223_d_n9, assign13910_e8223_d_n10, assign13910_e8223_d_n11, assign13910_e8223_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13910_e8220: f64 = (-locals.var_tmf2);
                (assign13910_e8220, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13910_e8223;
        locals.var_tmf2_dn0 = assign13910_e8223_d_n0;
        locals.var_tmf2_dn2 = assign13910_e8223_d_n2;
        locals.var_tmf2_dn4 = assign13910_e8223_d_n4;
        locals.var_tmf2_dn5 = assign13910_e8223_d_n5;
        locals.var_tmf2_dn6 = assign13910_e8223_d_n6;
        locals.var_tmf2_dn7 = assign13910_e8223_d_n7;
        locals.var_tmf2_dn8 = assign13910_e8223_d_n8;
        locals.var_tmf2_dn9 = assign13910_e8223_d_n9;
        locals.var_tmf2_dn10 = assign13910_e8223_d_n10;
        locals.var_tmf2_dn11 = assign13910_e8223_d_n11;
        locals.var_tmf2_dn14 = assign13910_e8223_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13920_e8232, assign13920_e8232_d_n0, assign13920_e8232_d_n2, assign13920_e8232_d_n4, assign13920_e8232_d_n5, assign13920_e8232_d_n6, assign13920_e8232_d_n7, assign13920_e8232_d_n8, assign13920_e8232_d_n9, assign13920_e8232_d_n10, assign13920_e8232_d_n11, assign13920_e8232_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13920_e8227: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13920_e8229: f64 = (assign13920_e8227 + locals.var_tmf2);
        let assign13920_e8230: f64 = (assign13920_e8229).sqrt();
        (assign13920_e8230, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13920_e8230)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13920_e8232;
        locals.var_tmf2_dn0 = assign13920_e8232_d_n0;
        locals.var_tmf2_dn2 = assign13920_e8232_d_n2;
        locals.var_tmf2_dn4 = assign13920_e8232_d_n4;
        locals.var_tmf2_dn5 = assign13920_e8232_d_n5;
        locals.var_tmf2_dn6 = assign13920_e8232_d_n6;
        locals.var_tmf2_dn7 = assign13920_e8232_d_n7;
        locals.var_tmf2_dn8 = assign13920_e8232_d_n8;
        locals.var_tmf2_dn9 = assign13920_e8232_d_n9;
        locals.var_tmf2_dn10 = assign13920_e8232_d_n10;
        locals.var_tmf2_dn11 = assign13920_e8232_d_n11;
        locals.var_tmf2_dn14 = assign13920_e8232_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign13930_e8242, assign13930_e8242_d_n0, assign13930_e8242_d_n2, assign13930_e8242_d_n4, assign13930_e8242_d_n5, assign13930_e8242_d_n6, assign13930_e8242_d_n7, assign13930_e8242_d_n8, assign13930_e8242_d_n9, assign13930_e8242_d_n10, assign13930_e8242_d_n11, assign13930_e8242_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13930_e8238: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13930_e8239: f64 = (1.0 + assign13930_e8238);
        let assign13930_e8240: f64 = (0.5 * assign13930_e8239);
        (assign13930_e8240, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13930_e8242;
        locals.var_t0_dn0 = assign13930_e8242_d_n0;
        locals.var_t0_dn2 = assign13930_e8242_d_n2;
        locals.var_t0_dn4 = assign13930_e8242_d_n4;
        locals.var_t0_dn5 = assign13930_e8242_d_n5;
        locals.var_t0_dn6 = assign13930_e8242_d_n6;
        locals.var_t0_dn7 = assign13930_e8242_d_n7;
        locals.var_t0_dn8 = assign13930_e8242_d_n8;
        locals.var_t0_dn9 = assign13930_e8242_d_n9;
        locals.var_t0_dn10 = assign13930_e8242_d_n10;
        locals.var_t0_dn11 = assign13930_e8242_d_n11;
        locals.var_t0_dn14 = assign13930_e8242_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign13940_e8252, assign13940_e8252_d_n0, assign13940_e8252_d_n2, assign13940_e8252_d_n4, assign13940_e8252_d_n5, assign13940_e8252_d_n6, assign13940_e8252_d_n7, assign13940_e8252_d_n8, assign13940_e8252_d_n9, assign13940_e8252_d_n10, assign13940_e8252_d_n11, assign13940_e8252_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13940_e8248: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13940_e8249: f64 = (0.5 * assign13940_e8248);
        let assign13940_e8250: f64 = (1.0 - assign13940_e8249);
        (assign13940_e8250, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign13940_e8252;
        locals.var_powratio_dn0 = assign13940_e8252_d_n0;
        locals.var_powratio_dn2 = assign13940_e8252_d_n2;
        locals.var_powratio_dn4 = assign13940_e8252_d_n4;
        locals.var_powratio_dn5 = assign13940_e8252_d_n5;
        locals.var_powratio_dn6 = assign13940_e8252_d_n6;
        locals.var_powratio_dn7 = assign13940_e8252_d_n7;
        locals.var_powratio_dn8 = assign13940_e8252_d_n8;
        locals.var_powratio_dn9 = assign13940_e8252_d_n9;
        locals.var_powratio_dn10 = assign13940_e8252_d_n10;
        locals.var_powratio_dn11 = assign13940_e8252_d_n11;
        locals.var_powratio_dn14 = assign13940_e8252_d_n14;
        locals.var_powratio_rv = 0.0;

        let (assign13950_e8263, assign13950_e8263_d_n0, assign13950_e8263_d_n2, assign13950_e8263_d_n4, assign13950_e8263_d_n5, assign13950_e8263_d_n6, assign13950_e8263_d_n7, assign13950_e8263_d_n8, assign13950_e8263_d_n9, assign13950_e8263_d_n10, assign13950_e8263_d_n11, assign13950_e8263_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13950_e8256: f64 = (2.0 * locals.var_beta_inv);
        let assign13950_e8259: f64 = (locals.var_nsub / locals.var_nin);
        let assign13950_e8260: f64 = (assign13950_e8259).ln();
        let assign13950_e8261: f64 = (assign13950_e8256 * assign13950_e8260);
        (assign13950_e8261, (((2.0 * locals.var_beta_inv_dn0) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn2) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn4) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn5) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn6) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn7) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn8) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn9) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn10) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn11) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn14) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign13950_e8263;
        locals.var_pb2_dn0 = assign13950_e8263_d_n0;
        locals.var_pb2_dn2 = assign13950_e8263_d_n2;
        locals.var_pb2_dn4 = assign13950_e8263_d_n4;
        locals.var_pb2_dn5 = assign13950_e8263_d_n5;
        locals.var_pb2_dn6 = assign13950_e8263_d_n6;
        locals.var_pb2_dn7 = assign13950_e8263_d_n7;
        locals.var_pb2_dn8 = assign13950_e8263_d_n8;
        locals.var_pb2_dn9 = assign13950_e8263_d_n9;
        locals.var_pb2_dn10 = assign13950_e8263_d_n10;
        locals.var_pb2_dn11 = assign13950_e8263_d_n11;
        locals.var_pb2_dn14 = assign13950_e8263_d_n14;
        locals.var_pb2_rv = 0.0;

    }
}
