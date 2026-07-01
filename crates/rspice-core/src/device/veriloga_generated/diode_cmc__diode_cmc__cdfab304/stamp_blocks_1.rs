#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_db0: f64,
        var_dfn_sl_db1: f64,
        var_dfn_sl_db2: f64,
        var_dfn_sl_db3: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn1: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_sl_dn3: f64,
        var_dfn_sl_dn4: f64,
        var_dfn_sl_dn5: f64,
        var_guard153: f64,
        var_guard154: f64,
        var_guard169: f64,
        var_guard31: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign10430_e13270, assign10430_e13270_d_n0, assign10430_e13270_d_n1, assign10430_e13270_d_n2, assign10430_e13270_d_n3, assign10430_e13270_d_n4, assign10430_e13270_d_n5, assign10430_e13270_d_b0, assign10430_e13270_d_b1, assign10430_e13270_d_b2, assign10430_e13270_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10430_e13265: f64 = (var_tmf1 * var_tmf1);
        let assign10430_e13267: f64 = (assign10430_e13265 + var_tmf2);
        let assign10430_e13268: f64 = (assign10430_e13267).sqrt();
        (assign10430_e13268, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10430_e13268)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10430_e13268)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10430_e13268)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10430_e13268)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10430_e13268)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10430_e13270;
        var_tmf2_dn0 = assign10430_e13270_d_n0;
        var_tmf2_dn1 = assign10430_e13270_d_n1;
        var_tmf2_dn2 = assign10430_e13270_d_n2;
        var_tmf2_dn3 = assign10430_e13270_d_n3;
        var_tmf2_dn4 = assign10430_e13270_d_n4;
        var_tmf2_dn5 = assign10430_e13270_d_n5;
        var_tmf2_db0 = assign10430_e13270_d_b0;
        var_tmf2_db1 = assign10430_e13270_d_b1;
        var_tmf2_db2 = assign10430_e13270_d_b2;
        var_tmf2_db3 = assign10430_e13270_d_b3;

        let (assign10440_e13287, assign10440_e13287_d_n0, assign10440_e13287_d_n1, assign10440_e13287_d_n2, assign10440_e13287_d_n3, assign10440_e13287_d_n4, assign10440_e13287_d_n5, assign10440_e13287_d_b0, assign10440_e13287_d_b1, assign10440_e13287_d_b2, assign10440_e13287_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10440_e13283: f64 = (var_tmf1 + var_tmf2);
        let assign10440_e13284: f64 = (0.5 * assign10440_e13283);
        let assign10440_e13285: f64 = (p.p85 - assign10440_e13284);
        (assign10440_e13285, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10440_e13287;
        var_nj0_dn0 = assign10440_e13287_d_n0;
        var_nj0_dn1 = assign10440_e13287_d_n1;
        var_nj0_dn2 = assign10440_e13287_d_n2;
        var_nj0_dn3 = assign10440_e13287_d_n3;
        var_nj0_dn4 = assign10440_e13287_d_n4;
        var_nj0_dn5 = assign10440_e13287_d_n5;
        var_nj0_db0 = assign10440_e13287_d_b0;
        var_nj0_db1 = assign10440_e13287_d_b1;
        var_nj0_db2 = assign10440_e13287_d_b2;
        var_nj0_db3 = assign10440_e13287_d_b3;

        let (assign10450_e13302, assign10450_e13302_d_n0, assign10450_e13302_d_n1, assign10450_e13302_d_n2, assign10450_e13302_d_n3, assign10450_e13302_d_n4, assign10450_e13302_d_n5, assign10450_e13302_d_b0, assign10450_e13302_d_b1, assign10450_e13302_d_b2, assign10450_e13302_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10450_e13298: f64 = (var_nj0 - var_nfasti_i);
        let assign10450_e13300: f64 = (assign10450_e13298 - 0.01);
        (assign10450_e13300, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign10450_e13302;
        var_tmf1_dn0 = assign10450_e13302_d_n0;
        var_tmf1_dn1 = assign10450_e13302_d_n1;
        var_tmf1_dn2 = assign10450_e13302_d_n2;
        var_tmf1_dn3 = assign10450_e13302_d_n3;
        var_tmf1_dn4 = assign10450_e13302_d_n4;
        var_tmf1_dn5 = assign10450_e13302_d_n5;
        var_tmf1_db0 = assign10450_e13302_d_b0;
        var_tmf1_db1 = assign10450_e13302_d_b1;
        var_tmf1_db2 = assign10450_e13302_d_b2;
        var_tmf1_db3 = assign10450_e13302_d_b3;

        let (assign10460_e13317, assign10460_e13317_d_n0, assign10460_e13317_d_n1, assign10460_e13317_d_n2, assign10460_e13317_d_n3, assign10460_e13317_d_n4, assign10460_e13317_d_n5, assign10460_e13317_d_b0, assign10460_e13317_d_b1, assign10460_e13317_d_b2, assign10460_e13317_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10460_e13313: f64 = (4.0 * var_nfasti_i);
        let assign10460_e13315: f64 = (assign10460_e13313 * 0.01);
        (assign10460_e13315, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10460_e13317;
        var_tmf2_dn0 = assign10460_e13317_d_n0;
        var_tmf2_dn1 = assign10460_e13317_d_n1;
        var_tmf2_dn2 = assign10460_e13317_d_n2;
        var_tmf2_dn3 = assign10460_e13317_d_n3;
        var_tmf2_dn4 = assign10460_e13317_d_n4;
        var_tmf2_dn5 = assign10460_e13317_d_n5;
        var_tmf2_db0 = assign10460_e13317_d_b0;
        var_tmf2_db1 = assign10460_e13317_d_b1;
        var_tmf2_db2 = assign10460_e13317_d_b2;
        var_tmf2_db3 = assign10460_e13317_d_b3;

        let (assign10470_e13334, assign10470_e13334_d_n0, assign10470_e13334_d_n1, assign10470_e13334_d_n2, assign10470_e13334_d_n3, assign10470_e13334_d_n4, assign10470_e13334_d_n5, assign10470_e13334_d_b0, assign10470_e13334_d_b1, assign10470_e13334_d_b2, assign10470_e13334_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n1, assign10470_e13332_d_n2, assign10470_e13332_d_n3, assign10470_e13332_d_n4, assign10470_e13332_d_n5, assign10470_e13332_d_b0, assign10470_e13332_d_b1, assign10470_e13332_d_b2, assign10470_e13332_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign10470_e13331: f64 = (-var_tmf2);
                (assign10470_e13331, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n1, assign10470_e13332_d_n2, assign10470_e13332_d_n3, assign10470_e13332_d_n4, assign10470_e13332_d_n5, assign10470_e13332_d_b0, assign10470_e13332_d_b1, assign10470_e13332_d_b2, assign10470_e13332_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10470_e13334;
        var_tmf2_dn0 = assign10470_e13334_d_n0;
        var_tmf2_dn1 = assign10470_e13334_d_n1;
        var_tmf2_dn2 = assign10470_e13334_d_n2;
        var_tmf2_dn3 = assign10470_e13334_d_n3;
        var_tmf2_dn4 = assign10470_e13334_d_n4;
        var_tmf2_dn5 = assign10470_e13334_d_n5;
        var_tmf2_db0 = assign10470_e13334_d_b0;
        var_tmf2_db1 = assign10470_e13334_d_b1;
        var_tmf2_db2 = assign10470_e13334_d_b2;
        var_tmf2_db3 = assign10470_e13334_d_b3;

        let (assign10480_e13350, assign10480_e13350_d_n0, assign10480_e13350_d_n1, assign10480_e13350_d_n2, assign10480_e13350_d_n3, assign10480_e13350_d_n4, assign10480_e13350_d_n5, assign10480_e13350_d_b0, assign10480_e13350_d_b1, assign10480_e13350_d_b2, assign10480_e13350_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10480_e13345: f64 = (var_tmf1 * var_tmf1);
        let assign10480_e13347: f64 = (assign10480_e13345 + var_tmf2);
        let assign10480_e13348: f64 = (assign10480_e13347).sqrt();
        (assign10480_e13348, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10480_e13348)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10480_e13348)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10480_e13348)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10480_e13348)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10480_e13348)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10480_e13348)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10480_e13350;
        var_tmf2_dn0 = assign10480_e13350_d_n0;
        var_tmf2_dn1 = assign10480_e13350_d_n1;
        var_tmf2_dn2 = assign10480_e13350_d_n2;
        var_tmf2_dn3 = assign10480_e13350_d_n3;
        var_tmf2_dn4 = assign10480_e13350_d_n4;
        var_tmf2_dn5 = assign10480_e13350_d_n5;
        var_tmf2_db0 = assign10480_e13350_d_b0;
        var_tmf2_db1 = assign10480_e13350_d_b1;
        var_tmf2_db2 = assign10480_e13350_d_b2;
        var_tmf2_db3 = assign10480_e13350_d_b3;

        let (assign10490_e13367, assign10490_e13367_d_n0, assign10490_e13367_d_n1, assign10490_e13367_d_n2, assign10490_e13367_d_n3, assign10490_e13367_d_n4, assign10490_e13367_d_n5, assign10490_e13367_d_b0, assign10490_e13367_d_b1, assign10490_e13367_d_b2, assign10490_e13367_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10490_e13363: f64 = (var_tmf1 + var_tmf2);
        let assign10490_e13364: f64 = (0.5 * assign10490_e13363);
        let assign10490_e13365: f64 = (var_nfasti_i + assign10490_e13364);
        (assign10490_e13365, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10490_e13367;
        var_nj0_dn0 = assign10490_e13367_d_n0;
        var_nj0_dn1 = assign10490_e13367_d_n1;
        var_nj0_dn2 = assign10490_e13367_d_n2;
        var_nj0_dn3 = assign10490_e13367_d_n3;
        var_nj0_dn4 = assign10490_e13367_d_n4;
        var_nj0_dn5 = assign10490_e13367_d_n5;
        var_nj0_db0 = assign10490_e13367_d_b0;
        var_nj0_db1 = assign10490_e13367_d_b1;
        var_nj0_db2 = assign10490_e13367_d_b2;
        var_nj0_db3 = assign10490_e13367_d_b3;

        let (assign10500_e13382, assign10500_e13382_d_n0, assign10500_e13382_d_n1, assign10500_e13382_d_n2, assign10500_e13382_d_n3, assign10500_e13382_d_n4, assign10500_e13382_d_n5, assign10500_e13382_d_b0, assign10500_e13382_d_b1, assign10500_e13382_d_b2, assign10500_e13382_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10500_e13378: f64 = (p.p86 * var_dfn_su);
        let assign10500_e13380: f64 = (assign10500_e13378 * var_dfn_sl);
        (assign10500_e13380, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign10500_e13378 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign10500_e13382;
        var_dnj1_dv_dn0 = assign10500_e13382_d_n0;
        var_dnj1_dv_dn1 = assign10500_e13382_d_n1;
        var_dnj1_dv_dn2 = assign10500_e13382_d_n2;
        var_dnj1_dv_dn3 = assign10500_e13382_d_n3;
        var_dnj1_dv_dn4 = assign10500_e13382_d_n4;
        var_dnj1_dv_dn5 = assign10500_e13382_d_n5;
        var_dnj1_dv_db0 = assign10500_e13382_d_b0;
        var_dnj1_dv_db1 = assign10500_e13382_d_b1;
        var_dnj1_dv_db2 = assign10500_e13382_d_b2;
        var_dnj1_dv_db3 = assign10500_e13382_d_b3;

        let (assign10510_e13394, assign10510_e13394_d_n0, assign10510_e13394_d_n1, assign10510_e13394_d_n2, assign10510_e13394_d_n3, assign10510_e13394_d_n4, assign10510_e13394_d_n5, assign10510_e13394_d_b0, assign10510_e13394_d_b1, assign10510_e13394_d_b2, assign10510_e13394_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10510_e13394;
        var_nj0_dn0 = assign10510_e13394_d_n0;
        var_nj0_dn1 = assign10510_e13394_d_n1;
        var_nj0_dn2 = assign10510_e13394_d_n2;
        var_nj0_dn3 = assign10510_e13394_d_n3;
        var_nj0_dn4 = assign10510_e13394_d_n4;
        var_nj0_dn5 = assign10510_e13394_d_n5;
        var_nj0_db0 = assign10510_e13394_d_b0;
        var_nj0_db1 = assign10510_e13394_d_b1;
        var_nj0_db2 = assign10510_e13394_d_b2;
        var_nj0_db3 = assign10510_e13394_d_b3;

        let (assign10520_e13406, assign10520_e13406_d_n0, assign10520_e13406_d_n1, assign10520_e13406_d_n2, assign10520_e13406_d_n3, assign10520_e13406_d_n4, assign10520_e13406_d_n5, assign10520_e13406_d_b0, assign10520_e13406_d_b1, assign10520_e13406_d_b2, assign10520_e13406_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign10520_e13406;
        var_nj1_dn0 = assign10520_e13406_d_n0;
        var_nj1_dn1 = assign10520_e13406_d_n1;
        var_nj1_dn2 = assign10520_e13406_d_n2;
        var_nj1_dn3 = assign10520_e13406_d_n3;
        var_nj1_dn4 = assign10520_e13406_d_n4;
        var_nj1_dn5 = assign10520_e13406_d_n5;
        var_nj1_db0 = assign10520_e13406_d_b0;
        var_nj1_db1 = assign10520_e13406_d_b1;
        var_nj1_db2 = assign10520_e13406_d_b2;
        var_nj1_db3 = assign10520_e13406_d_b3;

        let (assign10530_e13418, assign10530_e13418_d_n0, assign10530_e13418_d_n1, assign10530_e13418_d_n2, assign10530_e13418_d_n3, assign10530_e13418_d_n4, assign10530_e13418_d_n5, assign10530_e13418_d_b0, assign10530_e13418_d_b1, assign10530_e13418_d_b2, assign10530_e13418_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign10530_e13418;
        var_dnj1_dv_dn0 = assign10530_e13418_d_n0;
        var_dnj1_dv_dn1 = assign10530_e13418_d_n1;
        var_dnj1_dv_dn2 = assign10530_e13418_d_n2;
        var_dnj1_dv_dn3 = assign10530_e13418_d_n3;
        var_dnj1_dv_dn4 = assign10530_e13418_d_n4;
        var_dnj1_dv_dn5 = assign10530_e13418_d_n5;
        var_dnj1_dv_db0 = assign10530_e13418_d_b0;
        var_dnj1_dv_db1 = assign10530_e13418_d_b1;
        var_dnj1_dv_db2 = assign10530_e13418_d_b2;
        var_dnj1_dv_db3 = assign10530_e13418_d_b3;

        let (assign10590_e13667, assign10590_e13667_d_n0, assign10590_e13667_d_n1, assign10590_e13667_d_n2, assign10590_e13667_d_n3, assign10590_e13667_d_n4, assign10590_e13667_d_n5, assign10590_e13667_d_b0, assign10590_e13667_d_b1, assign10590_e13667_d_b2, assign10590_e13667_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10590_e13651: f64 = (var_vmax * var_dnj1_dv);
        let assign10590_e13652: f64 = (var_nj1 - assign10590_e13651);
        let assign10590_e13655: f64 = (var_nj1 * var_nj1);
        let assign10590_e13656: f64 = (assign10590_e13652 / assign10590_e13655);
        let assign10590_e13659: f64 = (var_vha1 * var_dnj1_dv);
        let assign10590_e13662: f64 = (var_nj0 * p.p85);
        let assign10590_e13663: f64 = (assign10590_e13659 / assign10590_e13662);
        let assign10590_e13664: f64 = (assign10590_e13656 + assign10590_e13663);
        let assign10590_e13665: f64 = (var_phitdinv * assign10590_e13664);
        (assign10590_e13665, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn0 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn1 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn2 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn3 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn4 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_dn5 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_db0) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_db0 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_db1) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_db1 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_db2) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_db2 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign10590_e13655) - (assign10590_e13652 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign10590_e13655 * assign10590_e13655)) + ((((var_vha1 * var_dnj1_dv_db3) * assign10590_e13662) - (assign10590_e13659 * (var_nj0_db3 * p.p85))) / (assign10590_e13662 * assign10590_e13662)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign10590_e13667;
        var_dvmax_over_phitd_dv_dn0 = assign10590_e13667_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign10590_e13667_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign10590_e13667_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign10590_e13667_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign10590_e13667_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign10590_e13667_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign10590_e13667_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign10590_e13667_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign10590_e13667_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign10590_e13667_d_b3;

        let (assign10610_e13697,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10610_e13693: f64 = (var_nin * var_nin);
        let assign10610_e13695: f64 = (assign10610_e13693 / var_ndigat_i);
        (assign10610_e13695,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign10610_e13697;

        let (assign10620_e13713,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10620_e13706: f64 = (var_nfagat_i / var_phitdinv);
        let assign10620_e13709: f64 = (var_ndigat_i / var_pnn0);
        let assign10620_e13710: f64 = (assign10620_e13709).ln();
        let assign10620_e13711: f64 = (assign10620_e13706 * assign10620_e13710);
        (assign10620_e13711,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign10620_e13713;

        let assign10630_e13716: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard172 = assign10630_e13716;

        let (assign10640_e13733, assign10640_e13733_d_n0, assign10640_e13733_d_n1, assign10640_e13733_d_n2, assign10640_e13733_d_n3, assign10640_e13733_d_n4, assign10640_e13733_d_n5, assign10640_e13733_d_b0, assign10640_e13733_d_b1, assign10640_e13733_d_b2, assign10640_e13733_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10640_e13728: f64 = (var_vmax - var_vha1);
        let assign10640_e13729: f64 = (p.p86 * assign10640_e13728);
        let assign10640_e13731: f64 = (assign10640_e13729 + var_nfagat_i);
        (assign10640_e13731, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign10640_e13733;
        var_nja10_dn0 = assign10640_e13733_d_n0;
        var_nja10_dn1 = assign10640_e13733_d_n1;
        var_nja10_dn2 = assign10640_e13733_d_n2;
        var_nja10_dn3 = assign10640_e13733_d_n3;
        var_nja10_dn4 = assign10640_e13733_d_n4;
        var_nja10_dn5 = assign10640_e13733_d_n5;
        var_nja10_db0 = assign10640_e13733_d_b0;
        var_nja10_db1 = assign10640_e13733_d_b1;
        var_nja10_db2 = assign10640_e13733_d_b2;
        var_nja10_db3 = assign10640_e13733_d_b3;

        let (assign10650_e13748, assign10650_e13748_d_n0, assign10650_e13748_d_n1, assign10650_e13748_d_n2, assign10650_e13748_d_n3, assign10650_e13748_d_n4, assign10650_e13748_d_n5, assign10650_e13748_d_b0, assign10650_e13748_d_b1, assign10650_e13748_d_b2, assign10650_e13748_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10650_e13745: f64 = (p.p86 * var_vha1);
        let assign10650_e13746: f64 = (var_nfagat_i - assign10650_e13745);
        (assign10650_e13746, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10650_e13748;
        var_nj0_dn0 = assign10650_e13748_d_n0;
        var_nj0_dn1 = assign10650_e13748_d_n1;
        var_nj0_dn2 = assign10650_e13748_d_n2;
        var_nj0_dn3 = assign10650_e13748_d_n3;
        var_nj0_dn4 = assign10650_e13748_d_n4;
        var_nj0_dn5 = assign10650_e13748_d_n5;
        var_nj0_db0 = assign10650_e13748_d_b0;
        var_nj0_db1 = assign10650_e13748_d_b1;
        var_nj0_db2 = assign10650_e13748_d_b2;
        var_nj0_db3 = assign10650_e13748_d_b3;

        let (assign10660_e13763, assign10660_e13763_d_n0, assign10660_e13763_d_n1, assign10660_e13763_d_n2, assign10660_e13763_d_n3, assign10660_e13763_d_n4, assign10660_e13763_d_n5, assign10660_e13763_d_b0, assign10660_e13763_d_b1, assign10660_e13763_d_b2, assign10660_e13763_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10660_e13759: f64 = (p.p85 - var_nja10);
        let assign10660_e13761: f64 = (assign10660_e13759 - 0.01);
        (assign10660_e13761, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign10660_e13763;
        var_tmf1_dn0 = assign10660_e13763_d_n0;
        var_tmf1_dn1 = assign10660_e13763_d_n1;
        var_tmf1_dn2 = assign10660_e13763_d_n2;
        var_tmf1_dn3 = assign10660_e13763_d_n3;
        var_tmf1_dn4 = assign10660_e13763_d_n4;
        var_tmf1_dn5 = assign10660_e13763_d_n5;
        var_tmf1_db0 = assign10660_e13763_d_b0;
        var_tmf1_db1 = assign10660_e13763_d_b1;
        var_tmf1_db2 = assign10660_e13763_d_b2;
        var_tmf1_db3 = assign10660_e13763_d_b3;

        let (assign10670_e13778, assign10670_e13778_d_n0, assign10670_e13778_d_n1, assign10670_e13778_d_n2, assign10670_e13778_d_n3, assign10670_e13778_d_n4, assign10670_e13778_d_n5, assign10670_e13778_d_b0, assign10670_e13778_d_b1, assign10670_e13778_d_b2, assign10670_e13778_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10670_e13774: f64 = (4.0 * p.p85);
        let assign10670_e13776: f64 = (assign10670_e13774 * 0.01);
        (assign10670_e13776, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10670_e13778;
        var_tmf2_dn0 = assign10670_e13778_d_n0;
        var_tmf2_dn1 = assign10670_e13778_d_n1;
        var_tmf2_dn2 = assign10670_e13778_d_n2;
        var_tmf2_dn3 = assign10670_e13778_d_n3;
        var_tmf2_dn4 = assign10670_e13778_d_n4;
        var_tmf2_dn5 = assign10670_e13778_d_n5;
        var_tmf2_db0 = assign10670_e13778_d_b0;
        var_tmf2_db1 = assign10670_e13778_d_b1;
        var_tmf2_db2 = assign10670_e13778_d_b2;
        var_tmf2_db3 = assign10670_e13778_d_b3;

        let (assign10680_e13795, assign10680_e13795_d_n0, assign10680_e13795_d_n1, assign10680_e13795_d_n2, assign10680_e13795_d_n3, assign10680_e13795_d_n4, assign10680_e13795_d_n5, assign10680_e13795_d_b0, assign10680_e13795_d_b1, assign10680_e13795_d_b2, assign10680_e13795_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n1, assign10680_e13793_d_n2, assign10680_e13793_d_n3, assign10680_e13793_d_n4, assign10680_e13793_d_n5, assign10680_e13793_d_b0, assign10680_e13793_d_b1, assign10680_e13793_d_b2, assign10680_e13793_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign10680_e13792: f64 = (-var_tmf2);
                (assign10680_e13792, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign10680_e13793, assign10680_e13793_d_n0, assign10680_e13793_d_n1, assign10680_e13793_d_n2, assign10680_e13793_d_n3, assign10680_e13793_d_n4, assign10680_e13793_d_n5, assign10680_e13793_d_b0, assign10680_e13793_d_b1, assign10680_e13793_d_b2, assign10680_e13793_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10680_e13795;
        var_tmf2_dn0 = assign10680_e13795_d_n0;
        var_tmf2_dn1 = assign10680_e13795_d_n1;
        var_tmf2_dn2 = assign10680_e13795_d_n2;
        var_tmf2_dn3 = assign10680_e13795_d_n3;
        var_tmf2_dn4 = assign10680_e13795_d_n4;
        var_tmf2_dn5 = assign10680_e13795_d_n5;
        var_tmf2_db0 = assign10680_e13795_d_b0;
        var_tmf2_db1 = assign10680_e13795_d_b1;
        var_tmf2_db2 = assign10680_e13795_d_b2;
        var_tmf2_db3 = assign10680_e13795_d_b3;

        let (assign10690_e13811, assign10690_e13811_d_n0, assign10690_e13811_d_n1, assign10690_e13811_d_n2, assign10690_e13811_d_n3, assign10690_e13811_d_n4, assign10690_e13811_d_n5, assign10690_e13811_d_b0, assign10690_e13811_d_b1, assign10690_e13811_d_b2, assign10690_e13811_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10690_e13806: f64 = (var_tmf1 * var_tmf1);
        let assign10690_e13808: f64 = (assign10690_e13806 + var_tmf2);
        let assign10690_e13809: f64 = (assign10690_e13808).sqrt();
        (assign10690_e13809, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10690_e13809)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10690_e13809)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10690_e13809)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10690_e13809)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10690_e13809)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10690_e13809)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10690_e13811;
        var_tmf2_dn0 = assign10690_e13811_d_n0;
        var_tmf2_dn1 = assign10690_e13811_d_n1;
        var_tmf2_dn2 = assign10690_e13811_d_n2;
        var_tmf2_dn3 = assign10690_e13811_d_n3;
        var_tmf2_dn4 = assign10690_e13811_d_n4;
        var_tmf2_dn5 = assign10690_e13811_d_n5;
        var_tmf2_db0 = assign10690_e13811_d_b0;
        var_tmf2_db1 = assign10690_e13811_d_b1;
        var_tmf2_db2 = assign10690_e13811_d_b2;
        var_tmf2_db3 = assign10690_e13811_d_b3;

        let (assign10700_e13828, assign10700_e13828_d_n0, assign10700_e13828_d_n1, assign10700_e13828_d_n2, assign10700_e13828_d_n3, assign10700_e13828_d_n4, assign10700_e13828_d_n5, assign10700_e13828_d_b0, assign10700_e13828_d_b1, assign10700_e13828_d_b2, assign10700_e13828_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10700_e13824: f64 = (var_tmf1 / var_tmf2);
        let assign10700_e13825: f64 = (1.0 + assign10700_e13824);
        let assign10700_e13826: f64 = (0.5 * assign10700_e13825);
        (assign10700_e13826, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign10700_e13828;
        var_dfn_su_dn0 = assign10700_e13828_d_n0;
        var_dfn_su_dn1 = assign10700_e13828_d_n1;
        var_dfn_su_dn2 = assign10700_e13828_d_n2;
        var_dfn_su_dn3 = assign10700_e13828_d_n3;
        var_dfn_su_dn4 = assign10700_e13828_d_n4;
        var_dfn_su_dn5 = assign10700_e13828_d_n5;
        var_dfn_su_db0 = assign10700_e13828_d_b0;
        var_dfn_su_db1 = assign10700_e13828_d_b1;
        var_dfn_su_db2 = assign10700_e13828_d_b2;
        var_dfn_su_db3 = assign10700_e13828_d_b3;

        let (assign10710_e13845, assign10710_e13845_d_n0, assign10710_e13845_d_n1, assign10710_e13845_d_n2, assign10710_e13845_d_n3, assign10710_e13845_d_n4, assign10710_e13845_d_n5, assign10710_e13845_d_b0, assign10710_e13845_d_b1, assign10710_e13845_d_b2, assign10710_e13845_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10710_e13841: f64 = (var_tmf1 + var_tmf2);
        let assign10710_e13842: f64 = (0.5 * assign10710_e13841);
        let assign10710_e13843: f64 = (p.p85 - assign10710_e13842);
        (assign10710_e13843, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign10710_e13845;
        var_nja11_dn0 = assign10710_e13845_d_n0;
        var_nja11_dn1 = assign10710_e13845_d_n1;
        var_nja11_dn2 = assign10710_e13845_d_n2;
        var_nja11_dn3 = assign10710_e13845_d_n3;
        var_nja11_dn4 = assign10710_e13845_d_n4;
        var_nja11_dn5 = assign10710_e13845_d_n5;
        var_nja11_db0 = assign10710_e13845_d_b0;
        var_nja11_db1 = assign10710_e13845_d_b1;
        var_nja11_db2 = assign10710_e13845_d_b2;
        var_nja11_db3 = assign10710_e13845_d_b3;

        let (assign10720_e13860, assign10720_e13860_d_n0, assign10720_e13860_d_n1, assign10720_e13860_d_n2, assign10720_e13860_d_n3, assign10720_e13860_d_n4, assign10720_e13860_d_n5, assign10720_e13860_d_b0, assign10720_e13860_d_b1, assign10720_e13860_d_b2, assign10720_e13860_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10720_e13856: f64 = (var_nja11 - var_nfagat_i);
        let assign10720_e13858: f64 = (assign10720_e13856 - 0.01);
        (assign10720_e13858, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign10720_e13860;
        var_tmf1_dn0 = assign10720_e13860_d_n0;
        var_tmf1_dn1 = assign10720_e13860_d_n1;
        var_tmf1_dn2 = assign10720_e13860_d_n2;
        var_tmf1_dn3 = assign10720_e13860_d_n3;
        var_tmf1_dn4 = assign10720_e13860_d_n4;
        var_tmf1_dn5 = assign10720_e13860_d_n5;
        var_tmf1_db0 = assign10720_e13860_d_b0;
        var_tmf1_db1 = assign10720_e13860_d_b1;
        var_tmf1_db2 = assign10720_e13860_d_b2;
        var_tmf1_db3 = assign10720_e13860_d_b3;

        let (assign10730_e13875, assign10730_e13875_d_n0, assign10730_e13875_d_n1, assign10730_e13875_d_n2, assign10730_e13875_d_n3, assign10730_e13875_d_n4, assign10730_e13875_d_n5, assign10730_e13875_d_b0, assign10730_e13875_d_b1, assign10730_e13875_d_b2, assign10730_e13875_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10730_e13871: f64 = (4.0 * var_nfagat_i);
        let assign10730_e13873: f64 = (assign10730_e13871 * 0.01);
        (assign10730_e13873, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10730_e13875;
        var_tmf2_dn0 = assign10730_e13875_d_n0;
        var_tmf2_dn1 = assign10730_e13875_d_n1;
        var_tmf2_dn2 = assign10730_e13875_d_n2;
        var_tmf2_dn3 = assign10730_e13875_d_n3;
        var_tmf2_dn4 = assign10730_e13875_d_n4;
        var_tmf2_dn5 = assign10730_e13875_d_n5;
        var_tmf2_db0 = assign10730_e13875_d_b0;
        var_tmf2_db1 = assign10730_e13875_d_b1;
        var_tmf2_db2 = assign10730_e13875_d_b2;
        var_tmf2_db3 = assign10730_e13875_d_b3;

        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_guard172_slot = var_guard172;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_ab_i: f64,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard153: f64,
        var_guard154: f64,
        var_guard172: f64,
        var_guard31: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_guard230_slot: &mut f64,
        var_guard231_slot: &mut f64,
        var_guard234_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_guard230: f64 = *var_guard230_slot;
        let mut var_guard231: f64 = *var_guard231_slot;
        let mut var_guard234: f64 = *var_guard234_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign10740_e13892, assign10740_e13892_d_n0, assign10740_e13892_d_n1, assign10740_e13892_d_n2, assign10740_e13892_d_n3, assign10740_e13892_d_n4, assign10740_e13892_d_n5, assign10740_e13892_d_b0, assign10740_e13892_d_b1, assign10740_e13892_d_b2, assign10740_e13892_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n1, assign10740_e13890_d_n2, assign10740_e13890_d_n3, assign10740_e13890_d_n4, assign10740_e13890_d_n5, assign10740_e13890_d_b0, assign10740_e13890_d_b1, assign10740_e13890_d_b2, assign10740_e13890_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign10740_e13889: f64 = (-var_tmf2);
                (assign10740_e13889, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign10740_e13890, assign10740_e13890_d_n0, assign10740_e13890_d_n1, assign10740_e13890_d_n2, assign10740_e13890_d_n3, assign10740_e13890_d_n4, assign10740_e13890_d_n5, assign10740_e13890_d_b0, assign10740_e13890_d_b1, assign10740_e13890_d_b2, assign10740_e13890_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10740_e13892;
        var_tmf2_dn0 = assign10740_e13892_d_n0;
        var_tmf2_dn1 = assign10740_e13892_d_n1;
        var_tmf2_dn2 = assign10740_e13892_d_n2;
        var_tmf2_dn3 = assign10740_e13892_d_n3;
        var_tmf2_dn4 = assign10740_e13892_d_n4;
        var_tmf2_dn5 = assign10740_e13892_d_n5;
        var_tmf2_db0 = assign10740_e13892_d_b0;
        var_tmf2_db1 = assign10740_e13892_d_b1;
        var_tmf2_db2 = assign10740_e13892_d_b2;
        var_tmf2_db3 = assign10740_e13892_d_b3;

        let (assign10750_e13908, assign10750_e13908_d_n0, assign10750_e13908_d_n1, assign10750_e13908_d_n2, assign10750_e13908_d_n3, assign10750_e13908_d_n4, assign10750_e13908_d_n5, assign10750_e13908_d_b0, assign10750_e13908_d_b1, assign10750_e13908_d_b2, assign10750_e13908_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10750_e13903: f64 = (var_tmf1 * var_tmf1);
        let assign10750_e13905: f64 = (assign10750_e13903 + var_tmf2);
        let assign10750_e13906: f64 = (assign10750_e13905).sqrt();
        (assign10750_e13906, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10750_e13906)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10750_e13906)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10750_e13906)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10750_e13906)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10750_e13906)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10750_e13906)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10750_e13908;
        var_tmf2_dn0 = assign10750_e13908_d_n0;
        var_tmf2_dn1 = assign10750_e13908_d_n1;
        var_tmf2_dn2 = assign10750_e13908_d_n2;
        var_tmf2_dn3 = assign10750_e13908_d_n3;
        var_tmf2_dn4 = assign10750_e13908_d_n4;
        var_tmf2_dn5 = assign10750_e13908_d_n5;
        var_tmf2_db0 = assign10750_e13908_d_b0;
        var_tmf2_db1 = assign10750_e13908_d_b1;
        var_tmf2_db2 = assign10750_e13908_d_b2;
        var_tmf2_db3 = assign10750_e13908_d_b3;

        let (assign10760_e13925, assign10760_e13925_d_n0, assign10760_e13925_d_n1, assign10760_e13925_d_n2, assign10760_e13925_d_n3, assign10760_e13925_d_n4, assign10760_e13925_d_n5, assign10760_e13925_d_b0, assign10760_e13925_d_b1, assign10760_e13925_d_b2, assign10760_e13925_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10760_e13921: f64 = (var_tmf1 / var_tmf2);
        let assign10760_e13922: f64 = (1.0 + assign10760_e13921);
        let assign10760_e13923: f64 = (0.5 * assign10760_e13922);
        (assign10760_e13923, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign10760_e13925;
        var_dfn_sl_dn0 = assign10760_e13925_d_n0;
        var_dfn_sl_dn1 = assign10760_e13925_d_n1;
        var_dfn_sl_dn2 = assign10760_e13925_d_n2;
        var_dfn_sl_dn3 = assign10760_e13925_d_n3;
        var_dfn_sl_dn4 = assign10760_e13925_d_n4;
        var_dfn_sl_dn5 = assign10760_e13925_d_n5;
        var_dfn_sl_db0 = assign10760_e13925_d_b0;
        var_dfn_sl_db1 = assign10760_e13925_d_b1;
        var_dfn_sl_db2 = assign10760_e13925_d_b2;
        var_dfn_sl_db3 = assign10760_e13925_d_b3;

        let (assign10770_e13942, assign10770_e13942_d_n0, assign10770_e13942_d_n1, assign10770_e13942_d_n2, assign10770_e13942_d_n3, assign10770_e13942_d_n4, assign10770_e13942_d_n5, assign10770_e13942_d_b0, assign10770_e13942_d_b1, assign10770_e13942_d_b2, assign10770_e13942_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10770_e13938: f64 = (var_tmf1 + var_tmf2);
        let assign10770_e13939: f64 = (0.5 * assign10770_e13938);
        let assign10770_e13940: f64 = (var_nfagat_i + assign10770_e13939);
        (assign10770_e13940, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign10770_e13942;
        var_nj1_dn0 = assign10770_e13942_d_n0;
        var_nj1_dn1 = assign10770_e13942_d_n1;
        var_nj1_dn2 = assign10770_e13942_d_n2;
        var_nj1_dn3 = assign10770_e13942_d_n3;
        var_nj1_dn4 = assign10770_e13942_d_n4;
        var_nj1_dn5 = assign10770_e13942_d_n5;
        var_nj1_db0 = assign10770_e13942_d_b0;
        var_nj1_db1 = assign10770_e13942_d_b1;
        var_nj1_db2 = assign10770_e13942_d_b2;
        var_nj1_db3 = assign10770_e13942_d_b3;

        let (assign10780_e13957, assign10780_e13957_d_n0, assign10780_e13957_d_n1, assign10780_e13957_d_n2, assign10780_e13957_d_n3, assign10780_e13957_d_n4, assign10780_e13957_d_n5, assign10780_e13957_d_b0, assign10780_e13957_d_b1, assign10780_e13957_d_b2, assign10780_e13957_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10780_e13953: f64 = (p.p85 - var_nj0);
        let assign10780_e13955: f64 = (assign10780_e13953 - 0.01);
        (assign10780_e13955, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign10780_e13957;
        var_tmf1_dn0 = assign10780_e13957_d_n0;
        var_tmf1_dn1 = assign10780_e13957_d_n1;
        var_tmf1_dn2 = assign10780_e13957_d_n2;
        var_tmf1_dn3 = assign10780_e13957_d_n3;
        var_tmf1_dn4 = assign10780_e13957_d_n4;
        var_tmf1_dn5 = assign10780_e13957_d_n5;
        var_tmf1_db0 = assign10780_e13957_d_b0;
        var_tmf1_db1 = assign10780_e13957_d_b1;
        var_tmf1_db2 = assign10780_e13957_d_b2;
        var_tmf1_db3 = assign10780_e13957_d_b3;

        let (assign10790_e13972, assign10790_e13972_d_n0, assign10790_e13972_d_n1, assign10790_e13972_d_n2, assign10790_e13972_d_n3, assign10790_e13972_d_n4, assign10790_e13972_d_n5, assign10790_e13972_d_b0, assign10790_e13972_d_b1, assign10790_e13972_d_b2, assign10790_e13972_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10790_e13968: f64 = (4.0 * p.p85);
        let assign10790_e13970: f64 = (assign10790_e13968 * 0.01);
        (assign10790_e13970, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10790_e13972;
        var_tmf2_dn0 = assign10790_e13972_d_n0;
        var_tmf2_dn1 = assign10790_e13972_d_n1;
        var_tmf2_dn2 = assign10790_e13972_d_n2;
        var_tmf2_dn3 = assign10790_e13972_d_n3;
        var_tmf2_dn4 = assign10790_e13972_d_n4;
        var_tmf2_dn5 = assign10790_e13972_d_n5;
        var_tmf2_db0 = assign10790_e13972_d_b0;
        var_tmf2_db1 = assign10790_e13972_d_b1;
        var_tmf2_db2 = assign10790_e13972_d_b2;
        var_tmf2_db3 = assign10790_e13972_d_b3;

        let (assign10800_e13989, assign10800_e13989_d_n0, assign10800_e13989_d_n1, assign10800_e13989_d_n2, assign10800_e13989_d_n3, assign10800_e13989_d_n4, assign10800_e13989_d_n5, assign10800_e13989_d_b0, assign10800_e13989_d_b1, assign10800_e13989_d_b2, assign10800_e13989_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n1, assign10800_e13987_d_n2, assign10800_e13987_d_n3, assign10800_e13987_d_n4, assign10800_e13987_d_n5, assign10800_e13987_d_b0, assign10800_e13987_d_b1, assign10800_e13987_d_b2, assign10800_e13987_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign10800_e13986: f64 = (-var_tmf2);
                (assign10800_e13986, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign10800_e13987, assign10800_e13987_d_n0, assign10800_e13987_d_n1, assign10800_e13987_d_n2, assign10800_e13987_d_n3, assign10800_e13987_d_n4, assign10800_e13987_d_n5, assign10800_e13987_d_b0, assign10800_e13987_d_b1, assign10800_e13987_d_b2, assign10800_e13987_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10800_e13989;
        var_tmf2_dn0 = assign10800_e13989_d_n0;
        var_tmf2_dn1 = assign10800_e13989_d_n1;
        var_tmf2_dn2 = assign10800_e13989_d_n2;
        var_tmf2_dn3 = assign10800_e13989_d_n3;
        var_tmf2_dn4 = assign10800_e13989_d_n4;
        var_tmf2_dn5 = assign10800_e13989_d_n5;
        var_tmf2_db0 = assign10800_e13989_d_b0;
        var_tmf2_db1 = assign10800_e13989_d_b1;
        var_tmf2_db2 = assign10800_e13989_d_b2;
        var_tmf2_db3 = assign10800_e13989_d_b3;

        let (assign10810_e14005, assign10810_e14005_d_n0, assign10810_e14005_d_n1, assign10810_e14005_d_n2, assign10810_e14005_d_n3, assign10810_e14005_d_n4, assign10810_e14005_d_n5, assign10810_e14005_d_b0, assign10810_e14005_d_b1, assign10810_e14005_d_b2, assign10810_e14005_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10810_e14000: f64 = (var_tmf1 * var_tmf1);
        let assign10810_e14002: f64 = (assign10810_e14000 + var_tmf2);
        let assign10810_e14003: f64 = (assign10810_e14002).sqrt();
        (assign10810_e14003, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10810_e14003)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10810_e14003)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10810_e14003)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10810_e14003)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10810_e14003)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10810_e14003)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10810_e14005;
        var_tmf2_dn0 = assign10810_e14005_d_n0;
        var_tmf2_dn1 = assign10810_e14005_d_n1;
        var_tmf2_dn2 = assign10810_e14005_d_n2;
        var_tmf2_dn3 = assign10810_e14005_d_n3;
        var_tmf2_dn4 = assign10810_e14005_d_n4;
        var_tmf2_dn5 = assign10810_e14005_d_n5;
        var_tmf2_db0 = assign10810_e14005_d_b0;
        var_tmf2_db1 = assign10810_e14005_d_b1;
        var_tmf2_db2 = assign10810_e14005_d_b2;
        var_tmf2_db3 = assign10810_e14005_d_b3;

        let (assign10820_e14022, assign10820_e14022_d_n0, assign10820_e14022_d_n1, assign10820_e14022_d_n2, assign10820_e14022_d_n3, assign10820_e14022_d_n4, assign10820_e14022_d_n5, assign10820_e14022_d_b0, assign10820_e14022_d_b1, assign10820_e14022_d_b2, assign10820_e14022_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10820_e14018: f64 = (var_tmf1 + var_tmf2);
        let assign10820_e14019: f64 = (0.5 * assign10820_e14018);
        let assign10820_e14020: f64 = (p.p85 - assign10820_e14019);
        (assign10820_e14020, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10820_e14022;
        var_nj0_dn0 = assign10820_e14022_d_n0;
        var_nj0_dn1 = assign10820_e14022_d_n1;
        var_nj0_dn2 = assign10820_e14022_d_n2;
        var_nj0_dn3 = assign10820_e14022_d_n3;
        var_nj0_dn4 = assign10820_e14022_d_n4;
        var_nj0_dn5 = assign10820_e14022_d_n5;
        var_nj0_db0 = assign10820_e14022_d_b0;
        var_nj0_db1 = assign10820_e14022_d_b1;
        var_nj0_db2 = assign10820_e14022_d_b2;
        var_nj0_db3 = assign10820_e14022_d_b3;

        let (assign10830_e14037, assign10830_e14037_d_n0, assign10830_e14037_d_n1, assign10830_e14037_d_n2, assign10830_e14037_d_n3, assign10830_e14037_d_n4, assign10830_e14037_d_n5, assign10830_e14037_d_b0, assign10830_e14037_d_b1, assign10830_e14037_d_b2, assign10830_e14037_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10830_e14033: f64 = (var_nj0 - var_nfagat_i);
        let assign10830_e14035: f64 = (assign10830_e14033 - 0.01);
        (assign10830_e14035, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign10830_e14037;
        var_tmf1_dn0 = assign10830_e14037_d_n0;
        var_tmf1_dn1 = assign10830_e14037_d_n1;
        var_tmf1_dn2 = assign10830_e14037_d_n2;
        var_tmf1_dn3 = assign10830_e14037_d_n3;
        var_tmf1_dn4 = assign10830_e14037_d_n4;
        var_tmf1_dn5 = assign10830_e14037_d_n5;
        var_tmf1_db0 = assign10830_e14037_d_b0;
        var_tmf1_db1 = assign10830_e14037_d_b1;
        var_tmf1_db2 = assign10830_e14037_d_b2;
        var_tmf1_db3 = assign10830_e14037_d_b3;

        let (assign10840_e14052, assign10840_e14052_d_n0, assign10840_e14052_d_n1, assign10840_e14052_d_n2, assign10840_e14052_d_n3, assign10840_e14052_d_n4, assign10840_e14052_d_n5, assign10840_e14052_d_b0, assign10840_e14052_d_b1, assign10840_e14052_d_b2, assign10840_e14052_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10840_e14048: f64 = (4.0 * var_nfagat_i);
        let assign10840_e14050: f64 = (assign10840_e14048 * 0.01);
        (assign10840_e14050, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10840_e14052;
        var_tmf2_dn0 = assign10840_e14052_d_n0;
        var_tmf2_dn1 = assign10840_e14052_d_n1;
        var_tmf2_dn2 = assign10840_e14052_d_n2;
        var_tmf2_dn3 = assign10840_e14052_d_n3;
        var_tmf2_dn4 = assign10840_e14052_d_n4;
        var_tmf2_dn5 = assign10840_e14052_d_n5;
        var_tmf2_db0 = assign10840_e14052_d_b0;
        var_tmf2_db1 = assign10840_e14052_d_b1;
        var_tmf2_db2 = assign10840_e14052_d_b2;
        var_tmf2_db3 = assign10840_e14052_d_b3;

        let (assign10850_e14069, assign10850_e14069_d_n0, assign10850_e14069_d_n1, assign10850_e14069_d_n2, assign10850_e14069_d_n3, assign10850_e14069_d_n4, assign10850_e14069_d_n5, assign10850_e14069_d_b0, assign10850_e14069_d_b1, assign10850_e14069_d_b2, assign10850_e14069_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n1, assign10850_e14067_d_n2, assign10850_e14067_d_n3, assign10850_e14067_d_n4, assign10850_e14067_d_n5, assign10850_e14067_d_b0, assign10850_e14067_d_b1, assign10850_e14067_d_b2, assign10850_e14067_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign10850_e14066: f64 = (-var_tmf2);
                (assign10850_e14066, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign10850_e14067, assign10850_e14067_d_n0, assign10850_e14067_d_n1, assign10850_e14067_d_n2, assign10850_e14067_d_n3, assign10850_e14067_d_n4, assign10850_e14067_d_n5, assign10850_e14067_d_b0, assign10850_e14067_d_b1, assign10850_e14067_d_b2, assign10850_e14067_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10850_e14069;
        var_tmf2_dn0 = assign10850_e14069_d_n0;
        var_tmf2_dn1 = assign10850_e14069_d_n1;
        var_tmf2_dn2 = assign10850_e14069_d_n2;
        var_tmf2_dn3 = assign10850_e14069_d_n3;
        var_tmf2_dn4 = assign10850_e14069_d_n4;
        var_tmf2_dn5 = assign10850_e14069_d_n5;
        var_tmf2_db0 = assign10850_e14069_d_b0;
        var_tmf2_db1 = assign10850_e14069_d_b1;
        var_tmf2_db2 = assign10850_e14069_d_b2;
        var_tmf2_db3 = assign10850_e14069_d_b3;

        let (assign10860_e14085, assign10860_e14085_d_n0, assign10860_e14085_d_n1, assign10860_e14085_d_n2, assign10860_e14085_d_n3, assign10860_e14085_d_n4, assign10860_e14085_d_n5, assign10860_e14085_d_b0, assign10860_e14085_d_b1, assign10860_e14085_d_b2, assign10860_e14085_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10860_e14080: f64 = (var_tmf1 * var_tmf1);
        let assign10860_e14082: f64 = (assign10860_e14080 + var_tmf2);
        let assign10860_e14083: f64 = (assign10860_e14082).sqrt();
        (assign10860_e14083, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign10860_e14083)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign10860_e14083)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign10860_e14083)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign10860_e14083)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign10860_e14083)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign10860_e14083)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign10860_e14085;
        var_tmf2_dn0 = assign10860_e14085_d_n0;
        var_tmf2_dn1 = assign10860_e14085_d_n1;
        var_tmf2_dn2 = assign10860_e14085_d_n2;
        var_tmf2_dn3 = assign10860_e14085_d_n3;
        var_tmf2_dn4 = assign10860_e14085_d_n4;
        var_tmf2_dn5 = assign10860_e14085_d_n5;
        var_tmf2_db0 = assign10860_e14085_d_b0;
        var_tmf2_db1 = assign10860_e14085_d_b1;
        var_tmf2_db2 = assign10860_e14085_d_b2;
        var_tmf2_db3 = assign10860_e14085_d_b3;

        let (assign10870_e14102, assign10870_e14102_d_n0, assign10870_e14102_d_n1, assign10870_e14102_d_n2, assign10870_e14102_d_n3, assign10870_e14102_d_n4, assign10870_e14102_d_n5, assign10870_e14102_d_b0, assign10870_e14102_d_b1, assign10870_e14102_d_b2, assign10870_e14102_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10870_e14098: f64 = (var_tmf1 + var_tmf2);
        let assign10870_e14099: f64 = (0.5 * assign10870_e14098);
        let assign10870_e14100: f64 = (var_nfagat_i + assign10870_e14099);
        (assign10870_e14100, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10870_e14102;
        var_nj0_dn0 = assign10870_e14102_d_n0;
        var_nj0_dn1 = assign10870_e14102_d_n1;
        var_nj0_dn2 = assign10870_e14102_d_n2;
        var_nj0_dn3 = assign10870_e14102_d_n3;
        var_nj0_dn4 = assign10870_e14102_d_n4;
        var_nj0_dn5 = assign10870_e14102_d_n5;
        var_nj0_db0 = assign10870_e14102_d_b0;
        var_nj0_db1 = assign10870_e14102_d_b1;
        var_nj0_db2 = assign10870_e14102_d_b2;
        var_nj0_db3 = assign10870_e14102_d_b3;

        let (assign10880_e14117, assign10880_e14117_d_n0, assign10880_e14117_d_n1, assign10880_e14117_d_n2, assign10880_e14117_d_n3, assign10880_e14117_d_n4, assign10880_e14117_d_n5, assign10880_e14117_d_b0, assign10880_e14117_d_b1, assign10880_e14117_d_b2, assign10880_e14117_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 != 0.0)) {
        let assign10880_e14113: f64 = (p.p86 * var_dfn_su);
        let assign10880_e14115: f64 = (assign10880_e14113 * var_dfn_sl);
        (assign10880_e14115, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign10880_e14113 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign10880_e14117;
        var_dnj1_dv_dn0 = assign10880_e14117_d_n0;
        var_dnj1_dv_dn1 = assign10880_e14117_d_n1;
        var_dnj1_dv_dn2 = assign10880_e14117_d_n2;
        var_dnj1_dv_dn3 = assign10880_e14117_d_n3;
        var_dnj1_dv_dn4 = assign10880_e14117_d_n4;
        var_dnj1_dv_dn5 = assign10880_e14117_d_n5;
        var_dnj1_dv_db0 = assign10880_e14117_d_b0;
        var_dnj1_dv_db1 = assign10880_e14117_d_b1;
        var_dnj1_dv_db2 = assign10880_e14117_d_b2;
        var_dnj1_dv_db3 = assign10880_e14117_d_b3;

        let (assign10890_e14129, assign10890_e14129_d_n0, assign10890_e14129_d_n1, assign10890_e14129_d_n2, assign10890_e14129_d_n3, assign10890_e14129_d_n4, assign10890_e14129_d_n5, assign10890_e14129_d_b0, assign10890_e14129_d_b1, assign10890_e14129_d_b2, assign10890_e14129_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign10890_e14129;
        var_nj0_dn0 = assign10890_e14129_d_n0;
        var_nj0_dn1 = assign10890_e14129_d_n1;
        var_nj0_dn2 = assign10890_e14129_d_n2;
        var_nj0_dn3 = assign10890_e14129_d_n3;
        var_nj0_dn4 = assign10890_e14129_d_n4;
        var_nj0_dn5 = assign10890_e14129_d_n5;
        var_nj0_db0 = assign10890_e14129_d_b0;
        var_nj0_db1 = assign10890_e14129_d_b1;
        var_nj0_db2 = assign10890_e14129_d_b2;
        var_nj0_db3 = assign10890_e14129_d_b3;

        let (assign10900_e14141, assign10900_e14141_d_n0, assign10900_e14141_d_n1, assign10900_e14141_d_n2, assign10900_e14141_d_n3, assign10900_e14141_d_n4, assign10900_e14141_d_n5, assign10900_e14141_d_b0, assign10900_e14141_d_b1, assign10900_e14141_d_b2, assign10900_e14141_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign10900_e14141;
        var_nj1_dn0 = assign10900_e14141_d_n0;
        var_nj1_dn1 = assign10900_e14141_d_n1;
        var_nj1_dn2 = assign10900_e14141_d_n2;
        var_nj1_dn3 = assign10900_e14141_d_n3;
        var_nj1_dn4 = assign10900_e14141_d_n4;
        var_nj1_dn5 = assign10900_e14141_d_n5;
        var_nj1_db0 = assign10900_e14141_d_b0;
        var_nj1_db1 = assign10900_e14141_d_b1;
        var_nj1_db2 = assign10900_e14141_d_b2;
        var_nj1_db3 = assign10900_e14141_d_b3;

        let (assign10910_e14153, assign10910_e14153_d_n0, assign10910_e14153_d_n1, assign10910_e14153_d_n2, assign10910_e14153_d_n3, assign10910_e14153_d_n4, assign10910_e14153_d_n5, assign10910_e14153_d_b0, assign10910_e14153_d_b1, assign10910_e14153_d_b2, assign10910_e14153_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard172 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign10910_e14153;
        var_dnj1_dv_dn0 = assign10910_e14153_d_n0;
        var_dnj1_dv_dn1 = assign10910_e14153_d_n1;
        var_dnj1_dv_dn2 = assign10910_e14153_d_n2;
        var_dnj1_dv_dn3 = assign10910_e14153_d_n3;
        var_dnj1_dv_dn4 = assign10910_e14153_d_n4;
        var_dnj1_dv_dn5 = assign10910_e14153_d_n5;
        var_dnj1_dv_db0 = assign10910_e14153_d_b0;
        var_dnj1_dv_db1 = assign10910_e14153_d_b1;
        var_dnj1_dv_db2 = assign10910_e14153_d_b2;
        var_dnj1_dv_db3 = assign10910_e14153_d_b3;

        let (assign10970_e14402, assign10970_e14402_d_n0, assign10970_e14402_d_n1, assign10970_e14402_d_n2, assign10970_e14402_d_n3, assign10970_e14402_d_n4, assign10970_e14402_d_n5, assign10970_e14402_d_b0, assign10970_e14402_d_b1, assign10970_e14402_d_b2, assign10970_e14402_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10970_e14386: f64 = (var_vmax * var_dnj1_dv);
        let assign10970_e14387: f64 = (var_nj1 - assign10970_e14386);
        let assign10970_e14390: f64 = (var_nj1 * var_nj1);
        let assign10970_e14391: f64 = (assign10970_e14387 / assign10970_e14390);
        let assign10970_e14394: f64 = (var_vha1 * var_dnj1_dv);
        let assign10970_e14397: f64 = (var_nj0 * p.p85);
        let assign10970_e14398: f64 = (assign10970_e14394 / assign10970_e14397);
        let assign10970_e14399: f64 = (assign10970_e14391 + assign10970_e14398);
        let assign10970_e14400: f64 = (var_phitdinv * assign10970_e14399);
        (assign10970_e14400, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn0 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn1 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn2 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn3 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn4 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_dn5 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_db0) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_db0 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_db1) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_db1 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_db2) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_db2 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign10970_e14390) - (assign10970_e14387 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign10970_e14390 * assign10970_e14390)) + ((((var_vha1 * var_dnj1_dv_db3) * assign10970_e14397) - (assign10970_e14394 * (var_nj0_db3 * p.p85))) / (assign10970_e14397 * assign10970_e14397)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign10970_e14402;
        var_dvmax_over_phitd_dv_dn0 = assign10970_e14402_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign10970_e14402_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign10970_e14402_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign10970_e14402_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign10970_e14402_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign10970_e14402_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign10970_e14402_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign10970_e14402_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign10970_e14402_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign10970_e14402_d_b3;

        let (assign10990_e14427, assign10990_e14427_d_n0, assign10990_e14427_d_n1, assign10990_e14427_d_n2, assign10990_e14427_d_n3, assign10990_e14427_d_n4, assign10990_e14427_d_n5, assign10990_e14427_d_b0, assign10990_e14427_d_b1, assign10990_e14427_d_b2, assign10990_e14427_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard153 != 0.0)) {
        let assign10990_e14425: f64 = (var_idmultbot - 1.0);
        (assign10990_e14425, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign10990_e14427;
        var_idmultbot_dn0 = assign10990_e14427_d_n0;
        var_idmultbot_dn1 = assign10990_e14427_d_n1;
        var_idmultbot_dn2 = assign10990_e14427_d_n2;
        var_idmultbot_dn3 = assign10990_e14427_d_n3;
        var_idmultbot_dn4 = assign10990_e14427_d_n4;
        var_idmultbot_dn5 = assign10990_e14427_d_n5;
        var_idmultbot_db0 = assign10990_e14427_d_b0;
        var_idmultbot_db1 = assign10990_e14427_d_b1;
        var_idmultbot_db2 = assign10990_e14427_d_b2;
        var_idmultbot_db3 = assign10990_e14427_d_b3;

        let (assign11100_e14600, assign11100_e14600_d_n0, assign11100_e14600_d_n1, assign11100_e14600_d_n2, assign11100_e14600_d_n3, assign11100_e14600_d_n4, assign11100_e14600_d_n5, assign11100_e14600_d_b0, assign11100_e14600_d_b1, assign11100_e14600_d_b2, assign11100_e14600_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard153 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign11100_e14600;
        var_idmultbot_dn0 = assign11100_e14600_d_n0;
        var_idmultbot_dn1 = assign11100_e14600_d_n1;
        var_idmultbot_dn2 = assign11100_e14600_d_n2;
        var_idmultbot_dn3 = assign11100_e14600_d_n3;
        var_idmultbot_dn4 = assign11100_e14600_d_n4;
        var_idmultbot_dn5 = assign11100_e14600_d_n5;
        var_idmultbot_db0 = assign11100_e14600_d_b0;
        var_idmultbot_db1 = assign11100_e14600_d_b1;
        var_idmultbot_db2 = assign11100_e14600_d_b2;
        var_idmultbot_db3 = assign11100_e14600_d_b3;

        let assign13630_e18150: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard230 = assign13630_e18150;

        let assign13710_e18222: f64 = if var_v3 < var_vmax { 1.0 } else { 0.0 };
        var_guard231 = assign13710_e18222;

        let (assign13770_e18363,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign13770_e18359: f64 = (var_nin * var_nin);
        let assign13770_e18361: f64 = (assign13770_e18359 / var_ndibot_i);
        (assign13770_e18361,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign13770_e18363;

        let (assign13780_e18378,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign13780_e18371: f64 = (var_nfabot_i / var_phitdinv);
        let assign13780_e18374: f64 = (var_ndibot_i / var_pnn0);
        let assign13780_e18375: f64 = (assign13780_e18374).ln();
        let assign13780_e18376: f64 = (assign13780_e18371 * assign13780_e18375);
        (assign13780_e18376,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign13780_e18378;

        let assign13790_e18381: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard234 = assign13790_e18381;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_guard230_slot = var_guard230;
        *var_guard231_slot = var_guard231;
        *var_guard234_slot = var_guard234;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard234: f64,
        var_guard31: f64,
        var_nfabot_i: f64,
        var_v3: f64,
        var_vha1: f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
    ) {
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;

        let (assign13800_e18397, assign13800_e18397_d_n0, assign13800_e18397_d_n1, assign13800_e18397_d_n2, assign13800_e18397_d_n3, assign13800_e18397_d_n4, assign13800_e18397_d_n5, assign13800_e18397_d_b0, assign13800_e18397_d_b1, assign13800_e18397_d_b2, assign13800_e18397_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13800_e18392: f64 = (var_v3 - var_vha1);
        let assign13800_e18393: f64 = (p.p86 * assign13800_e18392);
        let assign13800_e18395: f64 = (assign13800_e18393 + var_nfabot_i);
        (assign13800_e18395, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign13800_e18397;
        var_nja10_dn0 = assign13800_e18397_d_n0;
        var_nja10_dn1 = assign13800_e18397_d_n1;
        var_nja10_dn2 = assign13800_e18397_d_n2;
        var_nja10_dn3 = assign13800_e18397_d_n3;
        var_nja10_dn4 = assign13800_e18397_d_n4;
        var_nja10_dn5 = assign13800_e18397_d_n5;
        var_nja10_db0 = assign13800_e18397_d_b0;
        var_nja10_db1 = assign13800_e18397_d_b1;
        var_nja10_db2 = assign13800_e18397_d_b2;
        var_nja10_db3 = assign13800_e18397_d_b3;

        let (assign13810_e18411, assign13810_e18411_d_n0, assign13810_e18411_d_n1, assign13810_e18411_d_n2, assign13810_e18411_d_n3, assign13810_e18411_d_n4, assign13810_e18411_d_n5, assign13810_e18411_d_b0, assign13810_e18411_d_b1, assign13810_e18411_d_b2, assign13810_e18411_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13810_e18408: f64 = (p.p86 * var_vha1);
        let assign13810_e18409: f64 = (var_nfabot_i - assign13810_e18408);
        (assign13810_e18409, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign13810_e18411;
        var_nj0_dn0 = assign13810_e18411_d_n0;
        var_nj0_dn1 = assign13810_e18411_d_n1;
        var_nj0_dn2 = assign13810_e18411_d_n2;
        var_nj0_dn3 = assign13810_e18411_d_n3;
        var_nj0_dn4 = assign13810_e18411_d_n4;
        var_nj0_dn5 = assign13810_e18411_d_n5;
        var_nj0_db0 = assign13810_e18411_d_b0;
        var_nj0_db1 = assign13810_e18411_d_b1;
        var_nj0_db2 = assign13810_e18411_d_b2;
        var_nj0_db3 = assign13810_e18411_d_b3;

        let (assign13820_e18425, assign13820_e18425_d_n0, assign13820_e18425_d_n1, assign13820_e18425_d_n2, assign13820_e18425_d_n3, assign13820_e18425_d_n4, assign13820_e18425_d_n5, assign13820_e18425_d_b0, assign13820_e18425_d_b1, assign13820_e18425_d_b2, assign13820_e18425_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13820_e18421: f64 = (p.p85 - var_nja10);
        let assign13820_e18423: f64 = (assign13820_e18421 - 0.01);
        (assign13820_e18423, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign13820_e18425;
        var_tmf1_dn0 = assign13820_e18425_d_n0;
        var_tmf1_dn1 = assign13820_e18425_d_n1;
        var_tmf1_dn2 = assign13820_e18425_d_n2;
        var_tmf1_dn3 = assign13820_e18425_d_n3;
        var_tmf1_dn4 = assign13820_e18425_d_n4;
        var_tmf1_dn5 = assign13820_e18425_d_n5;
        var_tmf1_db0 = assign13820_e18425_d_b0;
        var_tmf1_db1 = assign13820_e18425_d_b1;
        var_tmf1_db2 = assign13820_e18425_d_b2;
        var_tmf1_db3 = assign13820_e18425_d_b3;

        let (assign13830_e18439, assign13830_e18439_d_n0, assign13830_e18439_d_n1, assign13830_e18439_d_n2, assign13830_e18439_d_n3, assign13830_e18439_d_n4, assign13830_e18439_d_n5, assign13830_e18439_d_b0, assign13830_e18439_d_b1, assign13830_e18439_d_b2, assign13830_e18439_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13830_e18435: f64 = (4.0 * p.p85);
        let assign13830_e18437: f64 = (assign13830_e18435 * 0.01);
        (assign13830_e18437, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13830_e18439;
        var_tmf2_dn0 = assign13830_e18439_d_n0;
        var_tmf2_dn1 = assign13830_e18439_d_n1;
        var_tmf2_dn2 = assign13830_e18439_d_n2;
        var_tmf2_dn3 = assign13830_e18439_d_n3;
        var_tmf2_dn4 = assign13830_e18439_d_n4;
        var_tmf2_dn5 = assign13830_e18439_d_n5;
        var_tmf2_db0 = assign13830_e18439_d_b0;
        var_tmf2_db1 = assign13830_e18439_d_b1;
        var_tmf2_db2 = assign13830_e18439_d_b2;
        var_tmf2_db3 = assign13830_e18439_d_b3;

        let (assign13840_e18455, assign13840_e18455_d_n0, assign13840_e18455_d_n1, assign13840_e18455_d_n2, assign13840_e18455_d_n3, assign13840_e18455_d_n4, assign13840_e18455_d_n5, assign13840_e18455_d_b0, assign13840_e18455_d_b1, assign13840_e18455_d_b2, assign13840_e18455_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n1, assign13840_e18453_d_n2, assign13840_e18453_d_n3, assign13840_e18453_d_n4, assign13840_e18453_d_n5, assign13840_e18453_d_b0, assign13840_e18453_d_b1, assign13840_e18453_d_b2, assign13840_e18453_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign13840_e18452: f64 = (-var_tmf2);
                (assign13840_e18452, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign13840_e18453, assign13840_e18453_d_n0, assign13840_e18453_d_n1, assign13840_e18453_d_n2, assign13840_e18453_d_n3, assign13840_e18453_d_n4, assign13840_e18453_d_n5, assign13840_e18453_d_b0, assign13840_e18453_d_b1, assign13840_e18453_d_b2, assign13840_e18453_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13840_e18455;
        var_tmf2_dn0 = assign13840_e18455_d_n0;
        var_tmf2_dn1 = assign13840_e18455_d_n1;
        var_tmf2_dn2 = assign13840_e18455_d_n2;
        var_tmf2_dn3 = assign13840_e18455_d_n3;
        var_tmf2_dn4 = assign13840_e18455_d_n4;
        var_tmf2_dn5 = assign13840_e18455_d_n5;
        var_tmf2_db0 = assign13840_e18455_d_b0;
        var_tmf2_db1 = assign13840_e18455_d_b1;
        var_tmf2_db2 = assign13840_e18455_d_b2;
        var_tmf2_db3 = assign13840_e18455_d_b3;

        let (assign13850_e18470, assign13850_e18470_d_n0, assign13850_e18470_d_n1, assign13850_e18470_d_n2, assign13850_e18470_d_n3, assign13850_e18470_d_n4, assign13850_e18470_d_n5, assign13850_e18470_d_b0, assign13850_e18470_d_b1, assign13850_e18470_d_b2, assign13850_e18470_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13850_e18465: f64 = (var_tmf1 * var_tmf1);
        let assign13850_e18467: f64 = (assign13850_e18465 + var_tmf2);
        let assign13850_e18468: f64 = (assign13850_e18467).sqrt();
        (assign13850_e18468, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign13850_e18468)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign13850_e18468)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign13850_e18468)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign13850_e18468)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign13850_e18468)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign13850_e18468)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13850_e18470;
        var_tmf2_dn0 = assign13850_e18470_d_n0;
        var_tmf2_dn1 = assign13850_e18470_d_n1;
        var_tmf2_dn2 = assign13850_e18470_d_n2;
        var_tmf2_dn3 = assign13850_e18470_d_n3;
        var_tmf2_dn4 = assign13850_e18470_d_n4;
        var_tmf2_dn5 = assign13850_e18470_d_n5;
        var_tmf2_db0 = assign13850_e18470_d_b0;
        var_tmf2_db1 = assign13850_e18470_d_b1;
        var_tmf2_db2 = assign13850_e18470_d_b2;
        var_tmf2_db3 = assign13850_e18470_d_b3;

        let (assign13860_e18486, assign13860_e18486_d_n0, assign13860_e18486_d_n1, assign13860_e18486_d_n2, assign13860_e18486_d_n3, assign13860_e18486_d_n4, assign13860_e18486_d_n5, assign13860_e18486_d_b0, assign13860_e18486_d_b1, assign13860_e18486_d_b2, assign13860_e18486_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13860_e18482: f64 = (var_tmf1 + var_tmf2);
        let assign13860_e18483: f64 = (0.5 * assign13860_e18482);
        let assign13860_e18484: f64 = (p.p85 - assign13860_e18483);
        (assign13860_e18484, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign13860_e18486;
        var_nja11_dn0 = assign13860_e18486_d_n0;
        var_nja11_dn1 = assign13860_e18486_d_n1;
        var_nja11_dn2 = assign13860_e18486_d_n2;
        var_nja11_dn3 = assign13860_e18486_d_n3;
        var_nja11_dn4 = assign13860_e18486_d_n4;
        var_nja11_dn5 = assign13860_e18486_d_n5;
        var_nja11_db0 = assign13860_e18486_d_b0;
        var_nja11_db1 = assign13860_e18486_d_b1;
        var_nja11_db2 = assign13860_e18486_d_b2;
        var_nja11_db3 = assign13860_e18486_d_b3;

        let (assign13870_e18500, assign13870_e18500_d_n0, assign13870_e18500_d_n1, assign13870_e18500_d_n2, assign13870_e18500_d_n3, assign13870_e18500_d_n4, assign13870_e18500_d_n5, assign13870_e18500_d_b0, assign13870_e18500_d_b1, assign13870_e18500_d_b2, assign13870_e18500_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13870_e18496: f64 = (var_nja11 - var_nfabot_i);
        let assign13870_e18498: f64 = (assign13870_e18496 - 0.01);
        (assign13870_e18498, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign13870_e18500;
        var_tmf1_dn0 = assign13870_e18500_d_n0;
        var_tmf1_dn1 = assign13870_e18500_d_n1;
        var_tmf1_dn2 = assign13870_e18500_d_n2;
        var_tmf1_dn3 = assign13870_e18500_d_n3;
        var_tmf1_dn4 = assign13870_e18500_d_n4;
        var_tmf1_dn5 = assign13870_e18500_d_n5;
        var_tmf1_db0 = assign13870_e18500_d_b0;
        var_tmf1_db1 = assign13870_e18500_d_b1;
        var_tmf1_db2 = assign13870_e18500_d_b2;
        var_tmf1_db3 = assign13870_e18500_d_b3;

        let (assign13880_e18514, assign13880_e18514_d_n0, assign13880_e18514_d_n1, assign13880_e18514_d_n2, assign13880_e18514_d_n3, assign13880_e18514_d_n4, assign13880_e18514_d_n5, assign13880_e18514_d_b0, assign13880_e18514_d_b1, assign13880_e18514_d_b2, assign13880_e18514_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13880_e18510: f64 = (4.0 * var_nfabot_i);
        let assign13880_e18512: f64 = (assign13880_e18510 * 0.01);
        (assign13880_e18512, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13880_e18514;
        var_tmf2_dn0 = assign13880_e18514_d_n0;
        var_tmf2_dn1 = assign13880_e18514_d_n1;
        var_tmf2_dn2 = assign13880_e18514_d_n2;
        var_tmf2_dn3 = assign13880_e18514_d_n3;
        var_tmf2_dn4 = assign13880_e18514_d_n4;
        var_tmf2_dn5 = assign13880_e18514_d_n5;
        var_tmf2_db0 = assign13880_e18514_d_b0;
        var_tmf2_db1 = assign13880_e18514_d_b1;
        var_tmf2_db2 = assign13880_e18514_d_b2;
        var_tmf2_db3 = assign13880_e18514_d_b3;

        let (assign13890_e18530, assign13890_e18530_d_n0, assign13890_e18530_d_n1, assign13890_e18530_d_n2, assign13890_e18530_d_n3, assign13890_e18530_d_n4, assign13890_e18530_d_n5, assign13890_e18530_d_b0, assign13890_e18530_d_b1, assign13890_e18530_d_b2, assign13890_e18530_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n1, assign13890_e18528_d_n2, assign13890_e18528_d_n3, assign13890_e18528_d_n4, assign13890_e18528_d_n5, assign13890_e18528_d_b0, assign13890_e18528_d_b1, assign13890_e18528_d_b2, assign13890_e18528_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign13890_e18527: f64 = (-var_tmf2);
                (assign13890_e18527, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign13890_e18528, assign13890_e18528_d_n0, assign13890_e18528_d_n1, assign13890_e18528_d_n2, assign13890_e18528_d_n3, assign13890_e18528_d_n4, assign13890_e18528_d_n5, assign13890_e18528_d_b0, assign13890_e18528_d_b1, assign13890_e18528_d_b2, assign13890_e18528_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13890_e18530;
        var_tmf2_dn0 = assign13890_e18530_d_n0;
        var_tmf2_dn1 = assign13890_e18530_d_n1;
        var_tmf2_dn2 = assign13890_e18530_d_n2;
        var_tmf2_dn3 = assign13890_e18530_d_n3;
        var_tmf2_dn4 = assign13890_e18530_d_n4;
        var_tmf2_dn5 = assign13890_e18530_d_n5;
        var_tmf2_db0 = assign13890_e18530_d_b0;
        var_tmf2_db1 = assign13890_e18530_d_b1;
        var_tmf2_db2 = assign13890_e18530_d_b2;
        var_tmf2_db3 = assign13890_e18530_d_b3;

        let (assign13900_e18545, assign13900_e18545_d_n0, assign13900_e18545_d_n1, assign13900_e18545_d_n2, assign13900_e18545_d_n3, assign13900_e18545_d_n4, assign13900_e18545_d_n5, assign13900_e18545_d_b0, assign13900_e18545_d_b1, assign13900_e18545_d_b2, assign13900_e18545_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13900_e18540: f64 = (var_tmf1 * var_tmf1);
        let assign13900_e18542: f64 = (assign13900_e18540 + var_tmf2);
        let assign13900_e18543: f64 = (assign13900_e18542).sqrt();
        (assign13900_e18543, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign13900_e18543)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign13900_e18543)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign13900_e18543)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign13900_e18543)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign13900_e18543)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign13900_e18543)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13900_e18545;
        var_tmf2_dn0 = assign13900_e18545_d_n0;
        var_tmf2_dn1 = assign13900_e18545_d_n1;
        var_tmf2_dn2 = assign13900_e18545_d_n2;
        var_tmf2_dn3 = assign13900_e18545_d_n3;
        var_tmf2_dn4 = assign13900_e18545_d_n4;
        var_tmf2_dn5 = assign13900_e18545_d_n5;
        var_tmf2_db0 = assign13900_e18545_d_b0;
        var_tmf2_db1 = assign13900_e18545_d_b1;
        var_tmf2_db2 = assign13900_e18545_d_b2;
        var_tmf2_db3 = assign13900_e18545_d_b3;

        let (assign13910_e18561, assign13910_e18561_d_n0, assign13910_e18561_d_n1, assign13910_e18561_d_n2, assign13910_e18561_d_n3, assign13910_e18561_d_n4, assign13910_e18561_d_n5, assign13910_e18561_d_b0, assign13910_e18561_d_b1, assign13910_e18561_d_b2, assign13910_e18561_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13910_e18557: f64 = (var_tmf1 + var_tmf2);
        let assign13910_e18558: f64 = (0.5 * assign13910_e18557);
        let assign13910_e18559: f64 = (var_nfabot_i + assign13910_e18558);
        (assign13910_e18559, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign13910_e18561;
        var_nj1_dn0 = assign13910_e18561_d_n0;
        var_nj1_dn1 = assign13910_e18561_d_n1;
        var_nj1_dn2 = assign13910_e18561_d_n2;
        var_nj1_dn3 = assign13910_e18561_d_n3;
        var_nj1_dn4 = assign13910_e18561_d_n4;
        var_nj1_dn5 = assign13910_e18561_d_n5;
        var_nj1_db0 = assign13910_e18561_d_b0;
        var_nj1_db1 = assign13910_e18561_d_b1;
        var_nj1_db2 = assign13910_e18561_d_b2;
        var_nj1_db3 = assign13910_e18561_d_b3;

        let (assign13920_e18575, assign13920_e18575_d_n0, assign13920_e18575_d_n1, assign13920_e18575_d_n2, assign13920_e18575_d_n3, assign13920_e18575_d_n4, assign13920_e18575_d_n5, assign13920_e18575_d_b0, assign13920_e18575_d_b1, assign13920_e18575_d_b2, assign13920_e18575_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13920_e18571: f64 = (p.p85 - var_nj0);
        let assign13920_e18573: f64 = (assign13920_e18571 - 0.01);
        (assign13920_e18573, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign13920_e18575;
        var_tmf1_dn0 = assign13920_e18575_d_n0;
        var_tmf1_dn1 = assign13920_e18575_d_n1;
        var_tmf1_dn2 = assign13920_e18575_d_n2;
        var_tmf1_dn3 = assign13920_e18575_d_n3;
        var_tmf1_dn4 = assign13920_e18575_d_n4;
        var_tmf1_dn5 = assign13920_e18575_d_n5;
        var_tmf1_db0 = assign13920_e18575_d_b0;
        var_tmf1_db1 = assign13920_e18575_d_b1;
        var_tmf1_db2 = assign13920_e18575_d_b2;
        var_tmf1_db3 = assign13920_e18575_d_b3;

        let (assign13930_e18589, assign13930_e18589_d_n0, assign13930_e18589_d_n1, assign13930_e18589_d_n2, assign13930_e18589_d_n3, assign13930_e18589_d_n4, assign13930_e18589_d_n5, assign13930_e18589_d_b0, assign13930_e18589_d_b1, assign13930_e18589_d_b2, assign13930_e18589_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13930_e18585: f64 = (4.0 * p.p85);
        let assign13930_e18587: f64 = (assign13930_e18585 * 0.01);
        (assign13930_e18587, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13930_e18589;
        var_tmf2_dn0 = assign13930_e18589_d_n0;
        var_tmf2_dn1 = assign13930_e18589_d_n1;
        var_tmf2_dn2 = assign13930_e18589_d_n2;
        var_tmf2_dn3 = assign13930_e18589_d_n3;
        var_tmf2_dn4 = assign13930_e18589_d_n4;
        var_tmf2_dn5 = assign13930_e18589_d_n5;
        var_tmf2_db0 = assign13930_e18589_d_b0;
        var_tmf2_db1 = assign13930_e18589_d_b1;
        var_tmf2_db2 = assign13930_e18589_d_b2;
        var_tmf2_db3 = assign13930_e18589_d_b3;

        let (assign13940_e18605, assign13940_e18605_d_n0, assign13940_e18605_d_n1, assign13940_e18605_d_n2, assign13940_e18605_d_n3, assign13940_e18605_d_n4, assign13940_e18605_d_n5, assign13940_e18605_d_b0, assign13940_e18605_d_b1, assign13940_e18605_d_b2, assign13940_e18605_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n1, assign13940_e18603_d_n2, assign13940_e18603_d_n3, assign13940_e18603_d_n4, assign13940_e18603_d_n5, assign13940_e18603_d_b0, assign13940_e18603_d_b1, assign13940_e18603_d_b2, assign13940_e18603_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign13940_e18602: f64 = (-var_tmf2);
                (assign13940_e18602, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign13940_e18603, assign13940_e18603_d_n0, assign13940_e18603_d_n1, assign13940_e18603_d_n2, assign13940_e18603_d_n3, assign13940_e18603_d_n4, assign13940_e18603_d_n5, assign13940_e18603_d_b0, assign13940_e18603_d_b1, assign13940_e18603_d_b2, assign13940_e18603_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13940_e18605;
        var_tmf2_dn0 = assign13940_e18605_d_n0;
        var_tmf2_dn1 = assign13940_e18605_d_n1;
        var_tmf2_dn2 = assign13940_e18605_d_n2;
        var_tmf2_dn3 = assign13940_e18605_d_n3;
        var_tmf2_dn4 = assign13940_e18605_d_n4;
        var_tmf2_dn5 = assign13940_e18605_d_n5;
        var_tmf2_db0 = assign13940_e18605_d_b0;
        var_tmf2_db1 = assign13940_e18605_d_b1;
        var_tmf2_db2 = assign13940_e18605_d_b2;
        var_tmf2_db3 = assign13940_e18605_d_b3;

        let (assign13950_e18620, assign13950_e18620_d_n0, assign13950_e18620_d_n1, assign13950_e18620_d_n2, assign13950_e18620_d_n3, assign13950_e18620_d_n4, assign13950_e18620_d_n5, assign13950_e18620_d_b0, assign13950_e18620_d_b1, assign13950_e18620_d_b2, assign13950_e18620_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13950_e18615: f64 = (var_tmf1 * var_tmf1);
        let assign13950_e18617: f64 = (assign13950_e18615 + var_tmf2);
        let assign13950_e18618: f64 = (assign13950_e18617).sqrt();
        (assign13950_e18618, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign13950_e18618)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign13950_e18618)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign13950_e18618)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign13950_e18618)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign13950_e18618)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign13950_e18618)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13950_e18620;
        var_tmf2_dn0 = assign13950_e18620_d_n0;
        var_tmf2_dn1 = assign13950_e18620_d_n1;
        var_tmf2_dn2 = assign13950_e18620_d_n2;
        var_tmf2_dn3 = assign13950_e18620_d_n3;
        var_tmf2_dn4 = assign13950_e18620_d_n4;
        var_tmf2_dn5 = assign13950_e18620_d_n5;
        var_tmf2_db0 = assign13950_e18620_d_b0;
        var_tmf2_db1 = assign13950_e18620_d_b1;
        var_tmf2_db2 = assign13950_e18620_d_b2;
        var_tmf2_db3 = assign13950_e18620_d_b3;

        let (assign13960_e18636, assign13960_e18636_d_n0, assign13960_e18636_d_n1, assign13960_e18636_d_n2, assign13960_e18636_d_n3, assign13960_e18636_d_n4, assign13960_e18636_d_n5, assign13960_e18636_d_b0, assign13960_e18636_d_b1, assign13960_e18636_d_b2, assign13960_e18636_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13960_e18632: f64 = (var_tmf1 + var_tmf2);
        let assign13960_e18633: f64 = (0.5 * assign13960_e18632);
        let assign13960_e18634: f64 = (p.p85 - assign13960_e18633);
        (assign13960_e18634, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign13960_e18636;
        var_nj0_dn0 = assign13960_e18636_d_n0;
        var_nj0_dn1 = assign13960_e18636_d_n1;
        var_nj0_dn2 = assign13960_e18636_d_n2;
        var_nj0_dn3 = assign13960_e18636_d_n3;
        var_nj0_dn4 = assign13960_e18636_d_n4;
        var_nj0_dn5 = assign13960_e18636_d_n5;
        var_nj0_db0 = assign13960_e18636_d_b0;
        var_nj0_db1 = assign13960_e18636_d_b1;
        var_nj0_db2 = assign13960_e18636_d_b2;
        var_nj0_db3 = assign13960_e18636_d_b3;

        let (assign13970_e18650, assign13970_e18650_d_n0, assign13970_e18650_d_n1, assign13970_e18650_d_n2, assign13970_e18650_d_n3, assign13970_e18650_d_n4, assign13970_e18650_d_n5, assign13970_e18650_d_b0, assign13970_e18650_d_b1, assign13970_e18650_d_b2, assign13970_e18650_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13970_e18646: f64 = (var_nj0 - var_nfabot_i);
        let assign13970_e18648: f64 = (assign13970_e18646 - 0.01);
        (assign13970_e18648, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign13970_e18650;
        var_tmf1_dn0 = assign13970_e18650_d_n0;
        var_tmf1_dn1 = assign13970_e18650_d_n1;
        var_tmf1_dn2 = assign13970_e18650_d_n2;
        var_tmf1_dn3 = assign13970_e18650_d_n3;
        var_tmf1_dn4 = assign13970_e18650_d_n4;
        var_tmf1_dn5 = assign13970_e18650_d_n5;
        var_tmf1_db0 = assign13970_e18650_d_b0;
        var_tmf1_db1 = assign13970_e18650_d_b1;
        var_tmf1_db2 = assign13970_e18650_d_b2;
        var_tmf1_db3 = assign13970_e18650_d_b3;

        let (assign13980_e18664, assign13980_e18664_d_n0, assign13980_e18664_d_n1, assign13980_e18664_d_n2, assign13980_e18664_d_n3, assign13980_e18664_d_n4, assign13980_e18664_d_n5, assign13980_e18664_d_b0, assign13980_e18664_d_b1, assign13980_e18664_d_b2, assign13980_e18664_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign13980_e18660: f64 = (4.0 * var_nfabot_i);
        let assign13980_e18662: f64 = (assign13980_e18660 * 0.01);
        (assign13980_e18662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13980_e18664;
        var_tmf2_dn0 = assign13980_e18664_d_n0;
        var_tmf2_dn1 = assign13980_e18664_d_n1;
        var_tmf2_dn2 = assign13980_e18664_d_n2;
        var_tmf2_dn3 = assign13980_e18664_d_n3;
        var_tmf2_dn4 = assign13980_e18664_d_n4;
        var_tmf2_dn5 = assign13980_e18664_d_n5;
        var_tmf2_db0 = assign13980_e18664_d_b0;
        var_tmf2_db1 = assign13980_e18664_d_b1;
        var_tmf2_db2 = assign13980_e18664_d_b2;
        var_tmf2_db3 = assign13980_e18664_d_b3;

        let (assign13990_e18680, assign13990_e18680_d_n0, assign13990_e18680_d_n1, assign13990_e18680_d_n2, assign13990_e18680_d_n3, assign13990_e18680_d_n4, assign13990_e18680_d_n5, assign13990_e18680_d_b0, assign13990_e18680_d_b1, assign13990_e18680_d_b2, assign13990_e18680_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n1, assign13990_e18678_d_n2, assign13990_e18678_d_n3, assign13990_e18678_d_n4, assign13990_e18678_d_n5, assign13990_e18678_d_b0, assign13990_e18678_d_b1, assign13990_e18678_d_b2, assign13990_e18678_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign13990_e18677: f64 = (-var_tmf2);
                (assign13990_e18677, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign13990_e18678, assign13990_e18678_d_n0, assign13990_e18678_d_n1, assign13990_e18678_d_n2, assign13990_e18678_d_n3, assign13990_e18678_d_n4, assign13990_e18678_d_n5, assign13990_e18678_d_b0, assign13990_e18678_d_b1, assign13990_e18678_d_b2, assign13990_e18678_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign13990_e18680;
        var_tmf2_dn0 = assign13990_e18680_d_n0;
        var_tmf2_dn1 = assign13990_e18680_d_n1;
        var_tmf2_dn2 = assign13990_e18680_d_n2;
        var_tmf2_dn3 = assign13990_e18680_d_n3;
        var_tmf2_dn4 = assign13990_e18680_d_n4;
        var_tmf2_dn5 = assign13990_e18680_d_n5;
        var_tmf2_db0 = assign13990_e18680_d_b0;
        var_tmf2_db1 = assign13990_e18680_d_b1;
        var_tmf2_db2 = assign13990_e18680_d_b2;
        var_tmf2_db3 = assign13990_e18680_d_b3;

        let (assign14000_e18695, assign14000_e18695_d_n0, assign14000_e18695_d_n1, assign14000_e18695_d_n2, assign14000_e18695_d_n3, assign14000_e18695_d_n4, assign14000_e18695_d_n5, assign14000_e18695_d_b0, assign14000_e18695_d_b1, assign14000_e18695_d_b2, assign14000_e18695_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign14000_e18690: f64 = (var_tmf1 * var_tmf1);
        let assign14000_e18692: f64 = (assign14000_e18690 + var_tmf2);
        let assign14000_e18693: f64 = (assign14000_e18692).sqrt();
        (assign14000_e18693, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14000_e18693)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14000_e18693)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14000_e18693)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14000_e18693)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14000_e18693)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14000_e18693)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14000_e18695;
        var_tmf2_dn0 = assign14000_e18695_d_n0;
        var_tmf2_dn1 = assign14000_e18695_d_n1;
        var_tmf2_dn2 = assign14000_e18695_d_n2;
        var_tmf2_dn3 = assign14000_e18695_d_n3;
        var_tmf2_dn4 = assign14000_e18695_d_n4;
        var_tmf2_dn5 = assign14000_e18695_d_n5;
        var_tmf2_db0 = assign14000_e18695_d_b0;
        var_tmf2_db1 = assign14000_e18695_d_b1;
        var_tmf2_db2 = assign14000_e18695_d_b2;
        var_tmf2_db3 = assign14000_e18695_d_b3;

        let (assign14010_e18711, assign14010_e18711_d_n0, assign14010_e18711_d_n1, assign14010_e18711_d_n2, assign14010_e18711_d_n3, assign14010_e18711_d_n4, assign14010_e18711_d_n5, assign14010_e18711_d_b0, assign14010_e18711_d_b1, assign14010_e18711_d_b2, assign14010_e18711_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 != 0.0)) {
        let assign14010_e18707: f64 = (var_tmf1 + var_tmf2);
        let assign14010_e18708: f64 = (0.5 * assign14010_e18707);
        let assign14010_e18709: f64 = (var_nfabot_i + assign14010_e18708);
        (assign14010_e18709, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14010_e18711;
        var_nj0_dn0 = assign14010_e18711_d_n0;
        var_nj0_dn1 = assign14010_e18711_d_n1;
        var_nj0_dn2 = assign14010_e18711_d_n2;
        var_nj0_dn3 = assign14010_e18711_d_n3;
        var_nj0_dn4 = assign14010_e18711_d_n4;
        var_nj0_dn5 = assign14010_e18711_d_n5;
        var_nj0_db0 = assign14010_e18711_d_b0;
        var_nj0_db1 = assign14010_e18711_d_b1;
        var_nj0_db2 = assign14010_e18711_d_b2;
        var_nj0_db3 = assign14010_e18711_d_b3;

        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard234: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_guard235_slot: &mut f64,
        var_guard236_slot: &mut f64,
        var_guard237_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard235: f64 = *var_guard235_slot;
        let mut var_guard236: f64 = *var_guard236_slot;
        let mut var_guard237: f64 = *var_guard237_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign14020_e18722, assign14020_e18722_d_n0, assign14020_e18722_d_n1, assign14020_e18722_d_n2, assign14020_e18722_d_n3, assign14020_e18722_d_n4, assign14020_e18722_d_n5, assign14020_e18722_d_b0, assign14020_e18722_d_b1, assign14020_e18722_d_b2, assign14020_e18722_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14020_e18722;
        var_nj0_dn0 = assign14020_e18722_d_n0;
        var_nj0_dn1 = assign14020_e18722_d_n1;
        var_nj0_dn2 = assign14020_e18722_d_n2;
        var_nj0_dn3 = assign14020_e18722_d_n3;
        var_nj0_dn4 = assign14020_e18722_d_n4;
        var_nj0_dn5 = assign14020_e18722_d_n5;
        var_nj0_db0 = assign14020_e18722_d_b0;
        var_nj0_db1 = assign14020_e18722_d_b1;
        var_nj0_db2 = assign14020_e18722_d_b2;
        var_nj0_db3 = assign14020_e18722_d_b3;

        let (assign14030_e18733, assign14030_e18733_d_n0, assign14030_e18733_d_n1, assign14030_e18733_d_n2, assign14030_e18733_d_n3, assign14030_e18733_d_n4, assign14030_e18733_d_n5, assign14030_e18733_d_b0, assign14030_e18733_d_b1, assign14030_e18733_d_b2, assign14030_e18733_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard234 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14030_e18733;
        var_nj1_dn0 = assign14030_e18733_d_n0;
        var_nj1_dn1 = assign14030_e18733_d_n1;
        var_nj1_dn2 = assign14030_e18733_d_n2;
        var_nj1_dn3 = assign14030_e18733_d_n3;
        var_nj1_dn4 = assign14030_e18733_d_n4;
        var_nj1_dn5 = assign14030_e18733_d_n5;
        var_nj1_db0 = assign14030_e18733_d_b0;
        var_nj1_db1 = assign14030_e18733_d_b1;
        var_nj1_db2 = assign14030_e18733_d_b2;
        var_nj1_db3 = assign14030_e18733_d_b3;

        let assign14040_e18737: f64 = (var_v3 / var_nj1);
        let assign14040_e18741: f64 = (var_nj1 - var_nj0);
        let assign14040_e18742: f64 = (var_vha1 * assign14040_e18741);
        let assign14040_e18745: f64 = (var_nj0 * p.p85);
        let assign14040_e18746: f64 = (assign14040_e18742 / assign14040_e18745);
        let assign14040_e18747: f64 = (assign14040_e18737 + assign14040_e18746);
        let assign14040_e18748: f64 = (var_phitdinv * assign14040_e18747);
        let assign14040_e18749: f64 = (assign14040_e18748).abs();
        let assign14040_e18751: f64 = if assign14040_e18749 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard235 = assign14040_e18751;

        let (assign14050_e18776, assign14050_e18776_d_n0, assign14050_e18776_d_n1, assign14050_e18776_d_n2, assign14050_e18776_d_n3, assign14050_e18776_d_n4, assign14050_e18776_d_n5, assign14050_e18776_d_b0, assign14050_e18776_d_b1, assign14050_e18776_d_b2, assign14050_e18776_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 != 0.0)) {
        let assign14050_e18762: f64 = (var_v3 / var_nj1);
        let assign14050_e18766: f64 = (var_nj1 - var_nj0);
        let assign14050_e18767: f64 = (var_vha1 * assign14050_e18766);
        let assign14050_e18770: f64 = (var_nj0 * p.p85);
        let assign14050_e18771: f64 = (assign14050_e18767 / assign14050_e18770);
        let assign14050_e18772: f64 = (assign14050_e18762 + assign14050_e18771);
        let assign14050_e18773: f64 = (var_phitdinv * assign14050_e18772);
        let assign14050_e18774: f64 = (assign14050_e18773).exp();
        (assign14050_e18774, (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn0 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn1 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn2 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn3 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn4 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_dn5 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_db0 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_db1 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_db2 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))), (assign14050_e18774 * (var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14050_e18770) - (assign14050_e18767 * (var_nj0_db3 * p.p85))) / (assign14050_e18770 * assign14050_e18770))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign14050_e18776;
        var_idmultbot_dn0 = assign14050_e18776_d_n0;
        var_idmultbot_dn1 = assign14050_e18776_d_n1;
        var_idmultbot_dn2 = assign14050_e18776_d_n2;
        var_idmultbot_dn3 = assign14050_e18776_d_n3;
        var_idmultbot_dn4 = assign14050_e18776_d_n4;
        var_idmultbot_dn5 = assign14050_e18776_d_n5;
        var_idmultbot_db0 = assign14050_e18776_d_b0;
        var_idmultbot_db1 = assign14050_e18776_d_b1;
        var_idmultbot_db2 = assign14050_e18776_d_b2;
        var_idmultbot_db3 = assign14050_e18776_d_b3;

        let assign14060_e18780: f64 = (var_v3 / var_nj1);
        let assign14060_e18784: f64 = (var_nj1 - var_nj0);
        let assign14060_e18785: f64 = (var_vha1 * assign14060_e18784);
        let assign14060_e18788: f64 = (var_nj0 * p.p85);
        let assign14060_e18789: f64 = (assign14060_e18785 / assign14060_e18788);
        let assign14060_e18790: f64 = (assign14060_e18780 + assign14060_e18789);
        let assign14060_e18791: f64 = (var_phitdinv * assign14060_e18790);
        let assign14060_e18793: f64 = (-230.25850929940458);
        let assign14060_e18794: f64 = if assign14060_e18791 < assign14060_e18793 { 1.0 } else { 0.0 };
        var_guard236 = assign14060_e18794;

        let (assign14070_e18874, assign14070_e18874_d_n0, assign14070_e18874_d_n1, assign14070_e18874_d_n2, assign14070_e18874_d_n3, assign14070_e18874_d_n4, assign14070_e18874_d_n5, assign14070_e18874_d_b0, assign14070_e18874_d_b1, assign14070_e18874_d_b2, assign14070_e18874_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 == 0.0)) && (var_guard236 != 0.0)) {
        let assign14070_e18808: f64 = (-230.25850929940458);
        let assign14070_e18812: f64 = (var_v3 / var_nj1);
        let assign14070_e18816: f64 = (var_nj1 - var_nj0);
        let assign14070_e18817: f64 = (var_vha1 * assign14070_e18816);
        let assign14070_e18820: f64 = (var_nj0 * p.p85);
        let assign14070_e18821: f64 = (assign14070_e18817 / assign14070_e18820);
        let assign14070_e18822: f64 = (assign14070_e18812 + assign14070_e18821);
        let assign14070_e18823: f64 = (var_phitdinv * assign14070_e18822);
        let assign14070_e18824: f64 = (assign14070_e18808 - assign14070_e18823);
        let assign14070_e18828: f64 = (-230.25850929940458);
        let assign14070_e18832: f64 = (var_v3 / var_nj1);
        let assign14070_e18836: f64 = (var_nj1 - var_nj0);
        let assign14070_e18837: f64 = (var_vha1 * assign14070_e18836);
        let assign14070_e18840: f64 = (var_nj0 * p.p85);
        let assign14070_e18841: f64 = (assign14070_e18837 / assign14070_e18840);
        let assign14070_e18842: f64 = (assign14070_e18832 + assign14070_e18841);
        let assign14070_e18843: f64 = (var_phitdinv * assign14070_e18842);
        let assign14070_e18844: f64 = (assign14070_e18828 - assign14070_e18843);
        let assign14070_e18847: f64 = (-230.25850929940458);
        let assign14070_e18851: f64 = (var_v3 / var_nj1);
        let assign14070_e18855: f64 = (var_nj1 - var_nj0);
        let assign14070_e18856: f64 = (var_vha1 * assign14070_e18855);
        let assign14070_e18859: f64 = (var_nj0 * p.p85);
        let assign14070_e18860: f64 = (assign14070_e18856 / assign14070_e18859);
        let assign14070_e18861: f64 = (assign14070_e18851 + assign14070_e18860);
        let assign14070_e18862: f64 = (var_phitdinv * assign14070_e18861);
        let assign14070_e18863: f64 = (assign14070_e18847 - assign14070_e18862);
        let assign14070_e18865: f64 = (assign14070_e18863 * 0.3333333333333333);
        let assign14070_e18866: f64 = (1.0 + assign14070_e18865);
        let assign14070_e18867: f64 = (assign14070_e18844 * assign14070_e18866);
        let assign14070_e18868: f64 = (0.5 * assign14070_e18867);
        let assign14070_e18869: f64 = (1.0 + assign14070_e18868);
        let assign14070_e18870: f64 = (assign14070_e18824 * assign14070_e18869);
        let assign14070_e18871: f64 = (1.0 + assign14070_e18870);
        let assign14070_e18872: f64 = (1e-100 / assign14070_e18871);
        (assign14070_e18872, (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn0 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn0 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn0 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn1 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn1 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn1 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn2 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn2 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn2 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn3 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn3 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn3 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn4 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn4 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn4 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_dn5 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_dn5 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_dn5 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_db0 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_db0 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_db0 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_db1 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_db1 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_db1 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_db2 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_db2 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_db2 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14070_e18820) - (assign14070_e18817 * (var_nj0_db3 * p.p85))) / (assign14070_e18820 * assign14070_e18820))))) * assign14070_e18869) + (assign14070_e18824 * (0.5 * (((-(var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14070_e18840) - (assign14070_e18837 * (var_nj0_db3 * p.p85))) / (assign14070_e18840 * assign14070_e18840))))) * assign14070_e18866) + (assign14070_e18844 * ((-(var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14070_e18859) - (assign14070_e18856 * (var_nj0_db3 * p.p85))) / (assign14070_e18859 * assign14070_e18859))))) * 0.3333333333333333))))))) / (assign14070_e18871 * assign14070_e18871))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign14070_e18874;
        var_idmultbot_dn0 = assign14070_e18874_d_n0;
        var_idmultbot_dn1 = assign14070_e18874_d_n1;
        var_idmultbot_dn2 = assign14070_e18874_d_n2;
        var_idmultbot_dn3 = assign14070_e18874_d_n3;
        var_idmultbot_dn4 = assign14070_e18874_d_n4;
        var_idmultbot_dn5 = assign14070_e18874_d_n5;
        var_idmultbot_db0 = assign14070_e18874_d_b0;
        var_idmultbot_db1 = assign14070_e18874_d_b1;
        var_idmultbot_db2 = assign14070_e18874_d_b2;
        var_idmultbot_db3 = assign14070_e18874_d_b3;

        let (assign14080_e18952, assign14080_e18952_d_n0, assign14080_e18952_d_n1, assign14080_e18952_d_n2, assign14080_e18952_d_n3, assign14080_e18952_d_n4, assign14080_e18952_d_n5, assign14080_e18952_d_b0, assign14080_e18952_d_b1, assign14080_e18952_d_b2, assign14080_e18952_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard235 == 0.0)) && (var_guard236 == 0.0)) {
        let assign14080_e18891: f64 = (var_v3 / var_nj1);
        let assign14080_e18895: f64 = (var_nj1 - var_nj0);
        let assign14080_e18896: f64 = (var_vha1 * assign14080_e18895);
        let assign14080_e18899: f64 = (var_nj0 * p.p85);
        let assign14080_e18900: f64 = (assign14080_e18896 / assign14080_e18899);
        let assign14080_e18901: f64 = (assign14080_e18891 + assign14080_e18900);
        let assign14080_e18902: f64 = (var_phitdinv * assign14080_e18901);
        let assign14080_e18904: f64 = (assign14080_e18902 - 230.25850929940458);
        let assign14080_e18910: f64 = (var_v3 / var_nj1);
        let assign14080_e18914: f64 = (var_nj1 - var_nj0);
        let assign14080_e18915: f64 = (var_vha1 * assign14080_e18914);
        let assign14080_e18918: f64 = (var_nj0 * p.p85);
        let assign14080_e18919: f64 = (assign14080_e18915 / assign14080_e18918);
        let assign14080_e18920: f64 = (assign14080_e18910 + assign14080_e18919);
        let assign14080_e18921: f64 = (var_phitdinv * assign14080_e18920);
        let assign14080_e18923: f64 = (assign14080_e18921 - 230.25850929940458);
        let assign14080_e18928: f64 = (var_v3 / var_nj1);
        let assign14080_e18932: f64 = (var_nj1 - var_nj0);
        let assign14080_e18933: f64 = (var_vha1 * assign14080_e18932);
        let assign14080_e18936: f64 = (var_nj0 * p.p85);
        let assign14080_e18937: f64 = (assign14080_e18933 / assign14080_e18936);
        let assign14080_e18938: f64 = (assign14080_e18928 + assign14080_e18937);
        let assign14080_e18939: f64 = (var_phitdinv * assign14080_e18938);
        let assign14080_e18941: f64 = (assign14080_e18939 - 230.25850929940458);
        let assign14080_e18943: f64 = (assign14080_e18941 * 0.3333333333333333);
        let assign14080_e18944: f64 = (1.0 + assign14080_e18943);
        let assign14080_e18945: f64 = (assign14080_e18923 * assign14080_e18944);
        let assign14080_e18946: f64 = (0.5 * assign14080_e18945);
        let assign14080_e18947: f64 = (1.0 + assign14080_e18946);
        let assign14080_e18948: f64 = (assign14080_e18904 * assign14080_e18947);
        let assign14080_e18949: f64 = (1.0 + assign14080_e18948);
        let assign14080_e18950: f64 = (1e100 * assign14080_e18949);
        (assign14080_e18950, (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn0 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn0 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn0 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn1 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn1 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn1 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn2 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn2 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn2 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn3 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn3 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn3 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn4 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn4 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn4 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_dn5 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_dn5 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_dn5 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_db0 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_db0 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_db0 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_db1 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_db1 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_db1 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_db2 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_db2 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_db2 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14080_e18899) - (assign14080_e18896 * (var_nj0_db3 * p.p85))) / (assign14080_e18899 * assign14080_e18899)))) * assign14080_e18947) + (assign14080_e18904 * (0.5 * (((var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14080_e18918) - (assign14080_e18915 * (var_nj0_db3 * p.p85))) / (assign14080_e18918 * assign14080_e18918)))) * assign14080_e18944) + (assign14080_e18923 * ((var_phitdinv * ((-((var_v3 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign14080_e18936) - (assign14080_e18933 * (var_nj0_db3 * p.p85))) / (assign14080_e18936 * assign14080_e18936)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign14080_e18952;
        var_idmultbot_dn0 = assign14080_e18952_d_n0;
        var_idmultbot_dn1 = assign14080_e18952_d_n1;
        var_idmultbot_dn2 = assign14080_e18952_d_n2;
        var_idmultbot_dn3 = assign14080_e18952_d_n3;
        var_idmultbot_dn4 = assign14080_e18952_d_n4;
        var_idmultbot_dn5 = assign14080_e18952_d_n5;
        var_idmultbot_db0 = assign14080_e18952_d_b0;
        var_idmultbot_db1 = assign14080_e18952_d_b1;
        var_idmultbot_db2 = assign14080_e18952_d_b2;
        var_idmultbot_db3 = assign14080_e18952_d_b3;

        let (assign14090_e18964,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14090_e18960: f64 = (var_nin * var_nin);
        let assign14090_e18962: f64 = (assign14090_e18960 / var_ndisti_i);
        (assign14090_e18962,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14090_e18964;

        let (assign14100_e18979,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14100_e18972: f64 = (var_nfasti_i / var_phitdinv);
        let assign14100_e18975: f64 = (var_ndisti_i / var_pnn0);
        let assign14100_e18976: f64 = (assign14100_e18975).ln();
        let assign14100_e18977: f64 = (assign14100_e18972 * assign14100_e18976);
        (assign14100_e18977,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14100_e18979;

        let assign14110_e18982: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard237 = assign14110_e18982;

        let (assign14120_e18998, assign14120_e18998_d_n0, assign14120_e18998_d_n1, assign14120_e18998_d_n2, assign14120_e18998_d_n3, assign14120_e18998_d_n4, assign14120_e18998_d_n5, assign14120_e18998_d_b0, assign14120_e18998_d_b1, assign14120_e18998_d_b2, assign14120_e18998_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14120_e18993: f64 = (var_v3 - var_vha1);
        let assign14120_e18994: f64 = (p.p86 * assign14120_e18993);
        let assign14120_e18996: f64 = (assign14120_e18994 + var_nfasti_i);
        (assign14120_e18996, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign14120_e18998;
        var_nja10_dn0 = assign14120_e18998_d_n0;
        var_nja10_dn1 = assign14120_e18998_d_n1;
        var_nja10_dn2 = assign14120_e18998_d_n2;
        var_nja10_dn3 = assign14120_e18998_d_n3;
        var_nja10_dn4 = assign14120_e18998_d_n4;
        var_nja10_dn5 = assign14120_e18998_d_n5;
        var_nja10_db0 = assign14120_e18998_d_b0;
        var_nja10_db1 = assign14120_e18998_d_b1;
        var_nja10_db2 = assign14120_e18998_d_b2;
        var_nja10_db3 = assign14120_e18998_d_b3;

        let (assign14130_e19012, assign14130_e19012_d_n0, assign14130_e19012_d_n1, assign14130_e19012_d_n2, assign14130_e19012_d_n3, assign14130_e19012_d_n4, assign14130_e19012_d_n5, assign14130_e19012_d_b0, assign14130_e19012_d_b1, assign14130_e19012_d_b2, assign14130_e19012_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14130_e19009: f64 = (p.p86 * var_vha1);
        let assign14130_e19010: f64 = (var_nfasti_i - assign14130_e19009);
        (assign14130_e19010, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14130_e19012;
        var_nj0_dn0 = assign14130_e19012_d_n0;
        var_nj0_dn1 = assign14130_e19012_d_n1;
        var_nj0_dn2 = assign14130_e19012_d_n2;
        var_nj0_dn3 = assign14130_e19012_d_n3;
        var_nj0_dn4 = assign14130_e19012_d_n4;
        var_nj0_dn5 = assign14130_e19012_d_n5;
        var_nj0_db0 = assign14130_e19012_d_b0;
        var_nj0_db1 = assign14130_e19012_d_b1;
        var_nj0_db2 = assign14130_e19012_d_b2;
        var_nj0_db3 = assign14130_e19012_d_b3;

        let (assign14140_e19026, assign14140_e19026_d_n0, assign14140_e19026_d_n1, assign14140_e19026_d_n2, assign14140_e19026_d_n3, assign14140_e19026_d_n4, assign14140_e19026_d_n5, assign14140_e19026_d_b0, assign14140_e19026_d_b1, assign14140_e19026_d_b2, assign14140_e19026_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14140_e19022: f64 = (p.p85 - var_nja10);
        let assign14140_e19024: f64 = (assign14140_e19022 - 0.01);
        (assign14140_e19024, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14140_e19026;
        var_tmf1_dn0 = assign14140_e19026_d_n0;
        var_tmf1_dn1 = assign14140_e19026_d_n1;
        var_tmf1_dn2 = assign14140_e19026_d_n2;
        var_tmf1_dn3 = assign14140_e19026_d_n3;
        var_tmf1_dn4 = assign14140_e19026_d_n4;
        var_tmf1_dn5 = assign14140_e19026_d_n5;
        var_tmf1_db0 = assign14140_e19026_d_b0;
        var_tmf1_db1 = assign14140_e19026_d_b1;
        var_tmf1_db2 = assign14140_e19026_d_b2;
        var_tmf1_db3 = assign14140_e19026_d_b3;

        let (assign14150_e19040, assign14150_e19040_d_n0, assign14150_e19040_d_n1, assign14150_e19040_d_n2, assign14150_e19040_d_n3, assign14150_e19040_d_n4, assign14150_e19040_d_n5, assign14150_e19040_d_b0, assign14150_e19040_d_b1, assign14150_e19040_d_b2, assign14150_e19040_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14150_e19036: f64 = (4.0 * p.p85);
        let assign14150_e19038: f64 = (assign14150_e19036 * 0.01);
        (assign14150_e19038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14150_e19040;
        var_tmf2_dn0 = assign14150_e19040_d_n0;
        var_tmf2_dn1 = assign14150_e19040_d_n1;
        var_tmf2_dn2 = assign14150_e19040_d_n2;
        var_tmf2_dn3 = assign14150_e19040_d_n3;
        var_tmf2_dn4 = assign14150_e19040_d_n4;
        var_tmf2_dn5 = assign14150_e19040_d_n5;
        var_tmf2_db0 = assign14150_e19040_d_b0;
        var_tmf2_db1 = assign14150_e19040_d_b1;
        var_tmf2_db2 = assign14150_e19040_d_b2;
        var_tmf2_db3 = assign14150_e19040_d_b3;

        let (assign14160_e19056, assign14160_e19056_d_n0, assign14160_e19056_d_n1, assign14160_e19056_d_n2, assign14160_e19056_d_n3, assign14160_e19056_d_n4, assign14160_e19056_d_n5, assign14160_e19056_d_b0, assign14160_e19056_d_b1, assign14160_e19056_d_b2, assign14160_e19056_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n1, assign14160_e19054_d_n2, assign14160_e19054_d_n3, assign14160_e19054_d_n4, assign14160_e19054_d_n5, assign14160_e19054_d_b0, assign14160_e19054_d_b1, assign14160_e19054_d_b2, assign14160_e19054_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14160_e19053: f64 = (-var_tmf2);
                (assign14160_e19053, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14160_e19054, assign14160_e19054_d_n0, assign14160_e19054_d_n1, assign14160_e19054_d_n2, assign14160_e19054_d_n3, assign14160_e19054_d_n4, assign14160_e19054_d_n5, assign14160_e19054_d_b0, assign14160_e19054_d_b1, assign14160_e19054_d_b2, assign14160_e19054_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14160_e19056;
        var_tmf2_dn0 = assign14160_e19056_d_n0;
        var_tmf2_dn1 = assign14160_e19056_d_n1;
        var_tmf2_dn2 = assign14160_e19056_d_n2;
        var_tmf2_dn3 = assign14160_e19056_d_n3;
        var_tmf2_dn4 = assign14160_e19056_d_n4;
        var_tmf2_dn5 = assign14160_e19056_d_n5;
        var_tmf2_db0 = assign14160_e19056_d_b0;
        var_tmf2_db1 = assign14160_e19056_d_b1;
        var_tmf2_db2 = assign14160_e19056_d_b2;
        var_tmf2_db3 = assign14160_e19056_d_b3;

        let (assign14170_e19071, assign14170_e19071_d_n0, assign14170_e19071_d_n1, assign14170_e19071_d_n2, assign14170_e19071_d_n3, assign14170_e19071_d_n4, assign14170_e19071_d_n5, assign14170_e19071_d_b0, assign14170_e19071_d_b1, assign14170_e19071_d_b2, assign14170_e19071_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14170_e19066: f64 = (var_tmf1 * var_tmf1);
        let assign14170_e19068: f64 = (assign14170_e19066 + var_tmf2);
        let assign14170_e19069: f64 = (assign14170_e19068).sqrt();
        (assign14170_e19069, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14170_e19069)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14170_e19069)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14170_e19069)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14170_e19069)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14170_e19069)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14170_e19069)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14170_e19071;
        var_tmf2_dn0 = assign14170_e19071_d_n0;
        var_tmf2_dn1 = assign14170_e19071_d_n1;
        var_tmf2_dn2 = assign14170_e19071_d_n2;
        var_tmf2_dn3 = assign14170_e19071_d_n3;
        var_tmf2_dn4 = assign14170_e19071_d_n4;
        var_tmf2_dn5 = assign14170_e19071_d_n5;
        var_tmf2_db0 = assign14170_e19071_d_b0;
        var_tmf2_db1 = assign14170_e19071_d_b1;
        var_tmf2_db2 = assign14170_e19071_d_b2;
        var_tmf2_db3 = assign14170_e19071_d_b3;

        let (assign14180_e19087, assign14180_e19087_d_n0, assign14180_e19087_d_n1, assign14180_e19087_d_n2, assign14180_e19087_d_n3, assign14180_e19087_d_n4, assign14180_e19087_d_n5, assign14180_e19087_d_b0, assign14180_e19087_d_b1, assign14180_e19087_d_b2, assign14180_e19087_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14180_e19083: f64 = (var_tmf1 + var_tmf2);
        let assign14180_e19084: f64 = (0.5 * assign14180_e19083);
        let assign14180_e19085: f64 = (p.p85 - assign14180_e19084);
        (assign14180_e19085, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign14180_e19087;
        var_nja11_dn0 = assign14180_e19087_d_n0;
        var_nja11_dn1 = assign14180_e19087_d_n1;
        var_nja11_dn2 = assign14180_e19087_d_n2;
        var_nja11_dn3 = assign14180_e19087_d_n3;
        var_nja11_dn4 = assign14180_e19087_d_n4;
        var_nja11_dn5 = assign14180_e19087_d_n5;
        var_nja11_db0 = assign14180_e19087_d_b0;
        var_nja11_db1 = assign14180_e19087_d_b1;
        var_nja11_db2 = assign14180_e19087_d_b2;
        var_nja11_db3 = assign14180_e19087_d_b3;

        let (assign14190_e19101, assign14190_e19101_d_n0, assign14190_e19101_d_n1, assign14190_e19101_d_n2, assign14190_e19101_d_n3, assign14190_e19101_d_n4, assign14190_e19101_d_n5, assign14190_e19101_d_b0, assign14190_e19101_d_b1, assign14190_e19101_d_b2, assign14190_e19101_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14190_e19097: f64 = (var_nja11 - var_nfasti_i);
        let assign14190_e19099: f64 = (assign14190_e19097 - 0.01);
        (assign14190_e19099, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14190_e19101;
        var_tmf1_dn0 = assign14190_e19101_d_n0;
        var_tmf1_dn1 = assign14190_e19101_d_n1;
        var_tmf1_dn2 = assign14190_e19101_d_n2;
        var_tmf1_dn3 = assign14190_e19101_d_n3;
        var_tmf1_dn4 = assign14190_e19101_d_n4;
        var_tmf1_dn5 = assign14190_e19101_d_n5;
        var_tmf1_db0 = assign14190_e19101_d_b0;
        var_tmf1_db1 = assign14190_e19101_d_b1;
        var_tmf1_db2 = assign14190_e19101_d_b2;
        var_tmf1_db3 = assign14190_e19101_d_b3;

        let (assign14200_e19115, assign14200_e19115_d_n0, assign14200_e19115_d_n1, assign14200_e19115_d_n2, assign14200_e19115_d_n3, assign14200_e19115_d_n4, assign14200_e19115_d_n5, assign14200_e19115_d_b0, assign14200_e19115_d_b1, assign14200_e19115_d_b2, assign14200_e19115_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14200_e19111: f64 = (4.0 * var_nfasti_i);
        let assign14200_e19113: f64 = (assign14200_e19111 * 0.01);
        (assign14200_e19113, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14200_e19115;
        var_tmf2_dn0 = assign14200_e19115_d_n0;
        var_tmf2_dn1 = assign14200_e19115_d_n1;
        var_tmf2_dn2 = assign14200_e19115_d_n2;
        var_tmf2_dn3 = assign14200_e19115_d_n3;
        var_tmf2_dn4 = assign14200_e19115_d_n4;
        var_tmf2_dn5 = assign14200_e19115_d_n5;
        var_tmf2_db0 = assign14200_e19115_d_b0;
        var_tmf2_db1 = assign14200_e19115_d_b1;
        var_tmf2_db2 = assign14200_e19115_d_b2;
        var_tmf2_db3 = assign14200_e19115_d_b3;

        let (assign14210_e19131, assign14210_e19131_d_n0, assign14210_e19131_d_n1, assign14210_e19131_d_n2, assign14210_e19131_d_n3, assign14210_e19131_d_n4, assign14210_e19131_d_n5, assign14210_e19131_d_b0, assign14210_e19131_d_b1, assign14210_e19131_d_b2, assign14210_e19131_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n1, assign14210_e19129_d_n2, assign14210_e19129_d_n3, assign14210_e19129_d_n4, assign14210_e19129_d_n5, assign14210_e19129_d_b0, assign14210_e19129_d_b1, assign14210_e19129_d_b2, assign14210_e19129_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14210_e19128: f64 = (-var_tmf2);
                (assign14210_e19128, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14210_e19129, assign14210_e19129_d_n0, assign14210_e19129_d_n1, assign14210_e19129_d_n2, assign14210_e19129_d_n3, assign14210_e19129_d_n4, assign14210_e19129_d_n5, assign14210_e19129_d_b0, assign14210_e19129_d_b1, assign14210_e19129_d_b2, assign14210_e19129_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14210_e19131;
        var_tmf2_dn0 = assign14210_e19131_d_n0;
        var_tmf2_dn1 = assign14210_e19131_d_n1;
        var_tmf2_dn2 = assign14210_e19131_d_n2;
        var_tmf2_dn3 = assign14210_e19131_d_n3;
        var_tmf2_dn4 = assign14210_e19131_d_n4;
        var_tmf2_dn5 = assign14210_e19131_d_n5;
        var_tmf2_db0 = assign14210_e19131_d_b0;
        var_tmf2_db1 = assign14210_e19131_d_b1;
        var_tmf2_db2 = assign14210_e19131_d_b2;
        var_tmf2_db3 = assign14210_e19131_d_b3;

        let (assign14220_e19146, assign14220_e19146_d_n0, assign14220_e19146_d_n1, assign14220_e19146_d_n2, assign14220_e19146_d_n3, assign14220_e19146_d_n4, assign14220_e19146_d_n5, assign14220_e19146_d_b0, assign14220_e19146_d_b1, assign14220_e19146_d_b2, assign14220_e19146_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14220_e19141: f64 = (var_tmf1 * var_tmf1);
        let assign14220_e19143: f64 = (assign14220_e19141 + var_tmf2);
        let assign14220_e19144: f64 = (assign14220_e19143).sqrt();
        (assign14220_e19144, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14220_e19144)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14220_e19144)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14220_e19144)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14220_e19144)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14220_e19144)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14220_e19144)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14220_e19146;
        var_tmf2_dn0 = assign14220_e19146_d_n0;
        var_tmf2_dn1 = assign14220_e19146_d_n1;
        var_tmf2_dn2 = assign14220_e19146_d_n2;
        var_tmf2_dn3 = assign14220_e19146_d_n3;
        var_tmf2_dn4 = assign14220_e19146_d_n4;
        var_tmf2_dn5 = assign14220_e19146_d_n5;
        var_tmf2_db0 = assign14220_e19146_d_b0;
        var_tmf2_db1 = assign14220_e19146_d_b1;
        var_tmf2_db2 = assign14220_e19146_d_b2;
        var_tmf2_db3 = assign14220_e19146_d_b3;

        let (assign14230_e19162, assign14230_e19162_d_n0, assign14230_e19162_d_n1, assign14230_e19162_d_n2, assign14230_e19162_d_n3, assign14230_e19162_d_n4, assign14230_e19162_d_n5, assign14230_e19162_d_b0, assign14230_e19162_d_b1, assign14230_e19162_d_b2, assign14230_e19162_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14230_e19158: f64 = (var_tmf1 + var_tmf2);
        let assign14230_e19159: f64 = (0.5 * assign14230_e19158);
        let assign14230_e19160: f64 = (var_nfasti_i + assign14230_e19159);
        (assign14230_e19160, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14230_e19162;
        var_nj1_dn0 = assign14230_e19162_d_n0;
        var_nj1_dn1 = assign14230_e19162_d_n1;
        var_nj1_dn2 = assign14230_e19162_d_n2;
        var_nj1_dn3 = assign14230_e19162_d_n3;
        var_nj1_dn4 = assign14230_e19162_d_n4;
        var_nj1_dn5 = assign14230_e19162_d_n5;
        var_nj1_db0 = assign14230_e19162_d_b0;
        var_nj1_db1 = assign14230_e19162_d_b1;
        var_nj1_db2 = assign14230_e19162_d_b2;
        var_nj1_db3 = assign14230_e19162_d_b3;

        let (assign14240_e19176, assign14240_e19176_d_n0, assign14240_e19176_d_n1, assign14240_e19176_d_n2, assign14240_e19176_d_n3, assign14240_e19176_d_n4, assign14240_e19176_d_n5, assign14240_e19176_d_b0, assign14240_e19176_d_b1, assign14240_e19176_d_b2, assign14240_e19176_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14240_e19172: f64 = (p.p85 - var_nj0);
        let assign14240_e19174: f64 = (assign14240_e19172 - 0.01);
        (assign14240_e19174, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14240_e19176;
        var_tmf1_dn0 = assign14240_e19176_d_n0;
        var_tmf1_dn1 = assign14240_e19176_d_n1;
        var_tmf1_dn2 = assign14240_e19176_d_n2;
        var_tmf1_dn3 = assign14240_e19176_d_n3;
        var_tmf1_dn4 = assign14240_e19176_d_n4;
        var_tmf1_dn5 = assign14240_e19176_d_n5;
        var_tmf1_db0 = assign14240_e19176_d_b0;
        var_tmf1_db1 = assign14240_e19176_d_b1;
        var_tmf1_db2 = assign14240_e19176_d_b2;
        var_tmf1_db3 = assign14240_e19176_d_b3;

        *var_guard235_slot = var_guard235;
        *var_guard236_slot = var_guard236;
        *var_guard237_slot = var_guard237;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard237: f64,
        var_guard31: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_guard240_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard240: f64 = *var_guard240_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign14250_e19190, assign14250_e19190_d_n0, assign14250_e19190_d_n1, assign14250_e19190_d_n2, assign14250_e19190_d_n3, assign14250_e19190_d_n4, assign14250_e19190_d_n5, assign14250_e19190_d_b0, assign14250_e19190_d_b1, assign14250_e19190_d_b2, assign14250_e19190_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14250_e19186: f64 = (4.0 * p.p85);
        let assign14250_e19188: f64 = (assign14250_e19186 * 0.01);
        (assign14250_e19188, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14250_e19190;
        var_tmf2_dn0 = assign14250_e19190_d_n0;
        var_tmf2_dn1 = assign14250_e19190_d_n1;
        var_tmf2_dn2 = assign14250_e19190_d_n2;
        var_tmf2_dn3 = assign14250_e19190_d_n3;
        var_tmf2_dn4 = assign14250_e19190_d_n4;
        var_tmf2_dn5 = assign14250_e19190_d_n5;
        var_tmf2_db0 = assign14250_e19190_d_b0;
        var_tmf2_db1 = assign14250_e19190_d_b1;
        var_tmf2_db2 = assign14250_e19190_d_b2;
        var_tmf2_db3 = assign14250_e19190_d_b3;

        let (assign14260_e19206, assign14260_e19206_d_n0, assign14260_e19206_d_n1, assign14260_e19206_d_n2, assign14260_e19206_d_n3, assign14260_e19206_d_n4, assign14260_e19206_d_n5, assign14260_e19206_d_b0, assign14260_e19206_d_b1, assign14260_e19206_d_b2, assign14260_e19206_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n1, assign14260_e19204_d_n2, assign14260_e19204_d_n3, assign14260_e19204_d_n4, assign14260_e19204_d_n5, assign14260_e19204_d_b0, assign14260_e19204_d_b1, assign14260_e19204_d_b2, assign14260_e19204_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14260_e19203: f64 = (-var_tmf2);
                (assign14260_e19203, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14260_e19204, assign14260_e19204_d_n0, assign14260_e19204_d_n1, assign14260_e19204_d_n2, assign14260_e19204_d_n3, assign14260_e19204_d_n4, assign14260_e19204_d_n5, assign14260_e19204_d_b0, assign14260_e19204_d_b1, assign14260_e19204_d_b2, assign14260_e19204_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14260_e19206;
        var_tmf2_dn0 = assign14260_e19206_d_n0;
        var_tmf2_dn1 = assign14260_e19206_d_n1;
        var_tmf2_dn2 = assign14260_e19206_d_n2;
        var_tmf2_dn3 = assign14260_e19206_d_n3;
        var_tmf2_dn4 = assign14260_e19206_d_n4;
        var_tmf2_dn5 = assign14260_e19206_d_n5;
        var_tmf2_db0 = assign14260_e19206_d_b0;
        var_tmf2_db1 = assign14260_e19206_d_b1;
        var_tmf2_db2 = assign14260_e19206_d_b2;
        var_tmf2_db3 = assign14260_e19206_d_b3;

        let (assign14270_e19221, assign14270_e19221_d_n0, assign14270_e19221_d_n1, assign14270_e19221_d_n2, assign14270_e19221_d_n3, assign14270_e19221_d_n4, assign14270_e19221_d_n5, assign14270_e19221_d_b0, assign14270_e19221_d_b1, assign14270_e19221_d_b2, assign14270_e19221_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14270_e19216: f64 = (var_tmf1 * var_tmf1);
        let assign14270_e19218: f64 = (assign14270_e19216 + var_tmf2);
        let assign14270_e19219: f64 = (assign14270_e19218).sqrt();
        (assign14270_e19219, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14270_e19219)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14270_e19219)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14270_e19219)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14270_e19219)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14270_e19219)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14270_e19219)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14270_e19221;
        var_tmf2_dn0 = assign14270_e19221_d_n0;
        var_tmf2_dn1 = assign14270_e19221_d_n1;
        var_tmf2_dn2 = assign14270_e19221_d_n2;
        var_tmf2_dn3 = assign14270_e19221_d_n3;
        var_tmf2_dn4 = assign14270_e19221_d_n4;
        var_tmf2_dn5 = assign14270_e19221_d_n5;
        var_tmf2_db0 = assign14270_e19221_d_b0;
        var_tmf2_db1 = assign14270_e19221_d_b1;
        var_tmf2_db2 = assign14270_e19221_d_b2;
        var_tmf2_db3 = assign14270_e19221_d_b3;

        let (assign14280_e19237, assign14280_e19237_d_n0, assign14280_e19237_d_n1, assign14280_e19237_d_n2, assign14280_e19237_d_n3, assign14280_e19237_d_n4, assign14280_e19237_d_n5, assign14280_e19237_d_b0, assign14280_e19237_d_b1, assign14280_e19237_d_b2, assign14280_e19237_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14280_e19233: f64 = (var_tmf1 + var_tmf2);
        let assign14280_e19234: f64 = (0.5 * assign14280_e19233);
        let assign14280_e19235: f64 = (p.p85 - assign14280_e19234);
        (assign14280_e19235, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14280_e19237;
        var_nj0_dn0 = assign14280_e19237_d_n0;
        var_nj0_dn1 = assign14280_e19237_d_n1;
        var_nj0_dn2 = assign14280_e19237_d_n2;
        var_nj0_dn3 = assign14280_e19237_d_n3;
        var_nj0_dn4 = assign14280_e19237_d_n4;
        var_nj0_dn5 = assign14280_e19237_d_n5;
        var_nj0_db0 = assign14280_e19237_d_b0;
        var_nj0_db1 = assign14280_e19237_d_b1;
        var_nj0_db2 = assign14280_e19237_d_b2;
        var_nj0_db3 = assign14280_e19237_d_b3;

        let (assign14290_e19251, assign14290_e19251_d_n0, assign14290_e19251_d_n1, assign14290_e19251_d_n2, assign14290_e19251_d_n3, assign14290_e19251_d_n4, assign14290_e19251_d_n5, assign14290_e19251_d_b0, assign14290_e19251_d_b1, assign14290_e19251_d_b2, assign14290_e19251_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14290_e19247: f64 = (var_nj0 - var_nfasti_i);
        let assign14290_e19249: f64 = (assign14290_e19247 - 0.01);
        (assign14290_e19249, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14290_e19251;
        var_tmf1_dn0 = assign14290_e19251_d_n0;
        var_tmf1_dn1 = assign14290_e19251_d_n1;
        var_tmf1_dn2 = assign14290_e19251_d_n2;
        var_tmf1_dn3 = assign14290_e19251_d_n3;
        var_tmf1_dn4 = assign14290_e19251_d_n4;
        var_tmf1_dn5 = assign14290_e19251_d_n5;
        var_tmf1_db0 = assign14290_e19251_d_b0;
        var_tmf1_db1 = assign14290_e19251_d_b1;
        var_tmf1_db2 = assign14290_e19251_d_b2;
        var_tmf1_db3 = assign14290_e19251_d_b3;

        let (assign14300_e19265, assign14300_e19265_d_n0, assign14300_e19265_d_n1, assign14300_e19265_d_n2, assign14300_e19265_d_n3, assign14300_e19265_d_n4, assign14300_e19265_d_n5, assign14300_e19265_d_b0, assign14300_e19265_d_b1, assign14300_e19265_d_b2, assign14300_e19265_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14300_e19261: f64 = (4.0 * var_nfasti_i);
        let assign14300_e19263: f64 = (assign14300_e19261 * 0.01);
        (assign14300_e19263, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14300_e19265;
        var_tmf2_dn0 = assign14300_e19265_d_n0;
        var_tmf2_dn1 = assign14300_e19265_d_n1;
        var_tmf2_dn2 = assign14300_e19265_d_n2;
        var_tmf2_dn3 = assign14300_e19265_d_n3;
        var_tmf2_dn4 = assign14300_e19265_d_n4;
        var_tmf2_dn5 = assign14300_e19265_d_n5;
        var_tmf2_db0 = assign14300_e19265_d_b0;
        var_tmf2_db1 = assign14300_e19265_d_b1;
        var_tmf2_db2 = assign14300_e19265_d_b2;
        var_tmf2_db3 = assign14300_e19265_d_b3;

        let (assign14310_e19281, assign14310_e19281_d_n0, assign14310_e19281_d_n1, assign14310_e19281_d_n2, assign14310_e19281_d_n3, assign14310_e19281_d_n4, assign14310_e19281_d_n5, assign14310_e19281_d_b0, assign14310_e19281_d_b1, assign14310_e19281_d_b2, assign14310_e19281_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n1, assign14310_e19279_d_n2, assign14310_e19279_d_n3, assign14310_e19279_d_n4, assign14310_e19279_d_n5, assign14310_e19279_d_b0, assign14310_e19279_d_b1, assign14310_e19279_d_b2, assign14310_e19279_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14310_e19278: f64 = (-var_tmf2);
                (assign14310_e19278, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14310_e19279, assign14310_e19279_d_n0, assign14310_e19279_d_n1, assign14310_e19279_d_n2, assign14310_e19279_d_n3, assign14310_e19279_d_n4, assign14310_e19279_d_n5, assign14310_e19279_d_b0, assign14310_e19279_d_b1, assign14310_e19279_d_b2, assign14310_e19279_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14310_e19281;
        var_tmf2_dn0 = assign14310_e19281_d_n0;
        var_tmf2_dn1 = assign14310_e19281_d_n1;
        var_tmf2_dn2 = assign14310_e19281_d_n2;
        var_tmf2_dn3 = assign14310_e19281_d_n3;
        var_tmf2_dn4 = assign14310_e19281_d_n4;
        var_tmf2_dn5 = assign14310_e19281_d_n5;
        var_tmf2_db0 = assign14310_e19281_d_b0;
        var_tmf2_db1 = assign14310_e19281_d_b1;
        var_tmf2_db2 = assign14310_e19281_d_b2;
        var_tmf2_db3 = assign14310_e19281_d_b3;

        let (assign14320_e19296, assign14320_e19296_d_n0, assign14320_e19296_d_n1, assign14320_e19296_d_n2, assign14320_e19296_d_n3, assign14320_e19296_d_n4, assign14320_e19296_d_n5, assign14320_e19296_d_b0, assign14320_e19296_d_b1, assign14320_e19296_d_b2, assign14320_e19296_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14320_e19291: f64 = (var_tmf1 * var_tmf1);
        let assign14320_e19293: f64 = (assign14320_e19291 + var_tmf2);
        let assign14320_e19294: f64 = (assign14320_e19293).sqrt();
        (assign14320_e19294, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14320_e19294)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14320_e19294)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14320_e19294)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14320_e19294)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14320_e19294)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14320_e19294)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14320_e19296;
        var_tmf2_dn0 = assign14320_e19296_d_n0;
        var_tmf2_dn1 = assign14320_e19296_d_n1;
        var_tmf2_dn2 = assign14320_e19296_d_n2;
        var_tmf2_dn3 = assign14320_e19296_d_n3;
        var_tmf2_dn4 = assign14320_e19296_d_n4;
        var_tmf2_dn5 = assign14320_e19296_d_n5;
        var_tmf2_db0 = assign14320_e19296_d_b0;
        var_tmf2_db1 = assign14320_e19296_d_b1;
        var_tmf2_db2 = assign14320_e19296_d_b2;
        var_tmf2_db3 = assign14320_e19296_d_b3;

        let (assign14330_e19312, assign14330_e19312_d_n0, assign14330_e19312_d_n1, assign14330_e19312_d_n2, assign14330_e19312_d_n3, assign14330_e19312_d_n4, assign14330_e19312_d_n5, assign14330_e19312_d_b0, assign14330_e19312_d_b1, assign14330_e19312_d_b2, assign14330_e19312_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 != 0.0)) {
        let assign14330_e19308: f64 = (var_tmf1 + var_tmf2);
        let assign14330_e19309: f64 = (0.5 * assign14330_e19308);
        let assign14330_e19310: f64 = (var_nfasti_i + assign14330_e19309);
        (assign14330_e19310, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14330_e19312;
        var_nj0_dn0 = assign14330_e19312_d_n0;
        var_nj0_dn1 = assign14330_e19312_d_n1;
        var_nj0_dn2 = assign14330_e19312_d_n2;
        var_nj0_dn3 = assign14330_e19312_d_n3;
        var_nj0_dn4 = assign14330_e19312_d_n4;
        var_nj0_dn5 = assign14330_e19312_d_n5;
        var_nj0_db0 = assign14330_e19312_d_b0;
        var_nj0_db1 = assign14330_e19312_d_b1;
        var_nj0_db2 = assign14330_e19312_d_b2;
        var_nj0_db3 = assign14330_e19312_d_b3;

        let (assign14340_e19323, assign14340_e19323_d_n0, assign14340_e19323_d_n1, assign14340_e19323_d_n2, assign14340_e19323_d_n3, assign14340_e19323_d_n4, assign14340_e19323_d_n5, assign14340_e19323_d_b0, assign14340_e19323_d_b1, assign14340_e19323_d_b2, assign14340_e19323_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14340_e19323;
        var_nj0_dn0 = assign14340_e19323_d_n0;
        var_nj0_dn1 = assign14340_e19323_d_n1;
        var_nj0_dn2 = assign14340_e19323_d_n2;
        var_nj0_dn3 = assign14340_e19323_d_n3;
        var_nj0_dn4 = assign14340_e19323_d_n4;
        var_nj0_dn5 = assign14340_e19323_d_n5;
        var_nj0_db0 = assign14340_e19323_d_b0;
        var_nj0_db1 = assign14340_e19323_d_b1;
        var_nj0_db2 = assign14340_e19323_d_b2;
        var_nj0_db3 = assign14340_e19323_d_b3;

        let (assign14350_e19334, assign14350_e19334_d_n0, assign14350_e19334_d_n1, assign14350_e19334_d_n2, assign14350_e19334_d_n3, assign14350_e19334_d_n4, assign14350_e19334_d_n5, assign14350_e19334_d_b0, assign14350_e19334_d_b1, assign14350_e19334_d_b2, assign14350_e19334_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard237 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14350_e19334;
        var_nj1_dn0 = assign14350_e19334_d_n0;
        var_nj1_dn1 = assign14350_e19334_d_n1;
        var_nj1_dn2 = assign14350_e19334_d_n2;
        var_nj1_dn3 = assign14350_e19334_d_n3;
        var_nj1_dn4 = assign14350_e19334_d_n4;
        var_nj1_dn5 = assign14350_e19334_d_n5;
        var_nj1_db0 = assign14350_e19334_d_b0;
        var_nj1_db1 = assign14350_e19334_d_b1;
        var_nj1_db2 = assign14350_e19334_d_b2;
        var_nj1_db3 = assign14350_e19334_d_b3;

        let (assign14410_e19565,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14410_e19561: f64 = (var_nin * var_nin);
        let assign14410_e19563: f64 = (assign14410_e19561 / var_ndigat_i);
        (assign14410_e19563,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14410_e19565;

        let (assign14420_e19580,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) {
        let assign14420_e19573: f64 = (var_nfagat_i / var_phitdinv);
        let assign14420_e19576: f64 = (var_ndigat_i / var_pnn0);
        let assign14420_e19577: f64 = (assign14420_e19576).ln();
        let assign14420_e19578: f64 = (assign14420_e19573 * assign14420_e19577);
        (assign14420_e19578,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14420_e19580;

        let assign14430_e19583: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard240 = assign14430_e19583;

        let (assign14440_e19599, assign14440_e19599_d_n0, assign14440_e19599_d_n1, assign14440_e19599_d_n2, assign14440_e19599_d_n3, assign14440_e19599_d_n4, assign14440_e19599_d_n5, assign14440_e19599_d_b0, assign14440_e19599_d_b1, assign14440_e19599_d_b2, assign14440_e19599_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14440_e19594: f64 = (var_v3 - var_vha1);
        let assign14440_e19595: f64 = (p.p86 * assign14440_e19594);
        let assign14440_e19597: f64 = (assign14440_e19595 + var_nfagat_i);
        (assign14440_e19597, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign14440_e19599;
        var_nja10_dn0 = assign14440_e19599_d_n0;
        var_nja10_dn1 = assign14440_e19599_d_n1;
        var_nja10_dn2 = assign14440_e19599_d_n2;
        var_nja10_dn3 = assign14440_e19599_d_n3;
        var_nja10_dn4 = assign14440_e19599_d_n4;
        var_nja10_dn5 = assign14440_e19599_d_n5;
        var_nja10_db0 = assign14440_e19599_d_b0;
        var_nja10_db1 = assign14440_e19599_d_b1;
        var_nja10_db2 = assign14440_e19599_d_b2;
        var_nja10_db3 = assign14440_e19599_d_b3;

        let (assign14450_e19613, assign14450_e19613_d_n0, assign14450_e19613_d_n1, assign14450_e19613_d_n2, assign14450_e19613_d_n3, assign14450_e19613_d_n4, assign14450_e19613_d_n5, assign14450_e19613_d_b0, assign14450_e19613_d_b1, assign14450_e19613_d_b2, assign14450_e19613_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14450_e19610: f64 = (p.p86 * var_vha1);
        let assign14450_e19611: f64 = (var_nfagat_i - assign14450_e19610);
        (assign14450_e19611, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14450_e19613;
        var_nj0_dn0 = assign14450_e19613_d_n0;
        var_nj0_dn1 = assign14450_e19613_d_n1;
        var_nj0_dn2 = assign14450_e19613_d_n2;
        var_nj0_dn3 = assign14450_e19613_d_n3;
        var_nj0_dn4 = assign14450_e19613_d_n4;
        var_nj0_dn5 = assign14450_e19613_d_n5;
        var_nj0_db0 = assign14450_e19613_d_b0;
        var_nj0_db1 = assign14450_e19613_d_b1;
        var_nj0_db2 = assign14450_e19613_d_b2;
        var_nj0_db3 = assign14450_e19613_d_b3;

        let (assign14460_e19627, assign14460_e19627_d_n0, assign14460_e19627_d_n1, assign14460_e19627_d_n2, assign14460_e19627_d_n3, assign14460_e19627_d_n4, assign14460_e19627_d_n5, assign14460_e19627_d_b0, assign14460_e19627_d_b1, assign14460_e19627_d_b2, assign14460_e19627_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14460_e19623: f64 = (p.p85 - var_nja10);
        let assign14460_e19625: f64 = (assign14460_e19623 - 0.01);
        (assign14460_e19625, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14460_e19627;
        var_tmf1_dn0 = assign14460_e19627_d_n0;
        var_tmf1_dn1 = assign14460_e19627_d_n1;
        var_tmf1_dn2 = assign14460_e19627_d_n2;
        var_tmf1_dn3 = assign14460_e19627_d_n3;
        var_tmf1_dn4 = assign14460_e19627_d_n4;
        var_tmf1_dn5 = assign14460_e19627_d_n5;
        var_tmf1_db0 = assign14460_e19627_d_b0;
        var_tmf1_db1 = assign14460_e19627_d_b1;
        var_tmf1_db2 = assign14460_e19627_d_b2;
        var_tmf1_db3 = assign14460_e19627_d_b3;

        let (assign14470_e19641, assign14470_e19641_d_n0, assign14470_e19641_d_n1, assign14470_e19641_d_n2, assign14470_e19641_d_n3, assign14470_e19641_d_n4, assign14470_e19641_d_n5, assign14470_e19641_d_b0, assign14470_e19641_d_b1, assign14470_e19641_d_b2, assign14470_e19641_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14470_e19637: f64 = (4.0 * p.p85);
        let assign14470_e19639: f64 = (assign14470_e19637 * 0.01);
        (assign14470_e19639, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14470_e19641;
        var_tmf2_dn0 = assign14470_e19641_d_n0;
        var_tmf2_dn1 = assign14470_e19641_d_n1;
        var_tmf2_dn2 = assign14470_e19641_d_n2;
        var_tmf2_dn3 = assign14470_e19641_d_n3;
        var_tmf2_dn4 = assign14470_e19641_d_n4;
        var_tmf2_dn5 = assign14470_e19641_d_n5;
        var_tmf2_db0 = assign14470_e19641_d_b0;
        var_tmf2_db1 = assign14470_e19641_d_b1;
        var_tmf2_db2 = assign14470_e19641_d_b2;
        var_tmf2_db3 = assign14470_e19641_d_b3;

        let (assign14480_e19657, assign14480_e19657_d_n0, assign14480_e19657_d_n1, assign14480_e19657_d_n2, assign14480_e19657_d_n3, assign14480_e19657_d_n4, assign14480_e19657_d_n5, assign14480_e19657_d_b0, assign14480_e19657_d_b1, assign14480_e19657_d_b2, assign14480_e19657_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n1, assign14480_e19655_d_n2, assign14480_e19655_d_n3, assign14480_e19655_d_n4, assign14480_e19655_d_n5, assign14480_e19655_d_b0, assign14480_e19655_d_b1, assign14480_e19655_d_b2, assign14480_e19655_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14480_e19654: f64 = (-var_tmf2);
                (assign14480_e19654, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14480_e19655, assign14480_e19655_d_n0, assign14480_e19655_d_n1, assign14480_e19655_d_n2, assign14480_e19655_d_n3, assign14480_e19655_d_n4, assign14480_e19655_d_n5, assign14480_e19655_d_b0, assign14480_e19655_d_b1, assign14480_e19655_d_b2, assign14480_e19655_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14480_e19657;
        var_tmf2_dn0 = assign14480_e19657_d_n0;
        var_tmf2_dn1 = assign14480_e19657_d_n1;
        var_tmf2_dn2 = assign14480_e19657_d_n2;
        var_tmf2_dn3 = assign14480_e19657_d_n3;
        var_tmf2_dn4 = assign14480_e19657_d_n4;
        var_tmf2_dn5 = assign14480_e19657_d_n5;
        var_tmf2_db0 = assign14480_e19657_d_b0;
        var_tmf2_db1 = assign14480_e19657_d_b1;
        var_tmf2_db2 = assign14480_e19657_d_b2;
        var_tmf2_db3 = assign14480_e19657_d_b3;

        let (assign14490_e19672, assign14490_e19672_d_n0, assign14490_e19672_d_n1, assign14490_e19672_d_n2, assign14490_e19672_d_n3, assign14490_e19672_d_n4, assign14490_e19672_d_n5, assign14490_e19672_d_b0, assign14490_e19672_d_b1, assign14490_e19672_d_b2, assign14490_e19672_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14490_e19667: f64 = (var_tmf1 * var_tmf1);
        let assign14490_e19669: f64 = (assign14490_e19667 + var_tmf2);
        let assign14490_e19670: f64 = (assign14490_e19669).sqrt();
        (assign14490_e19670, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14490_e19670)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14490_e19670)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14490_e19670)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14490_e19670)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14490_e19670)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14490_e19670)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14490_e19672;
        var_tmf2_dn0 = assign14490_e19672_d_n0;
        var_tmf2_dn1 = assign14490_e19672_d_n1;
        var_tmf2_dn2 = assign14490_e19672_d_n2;
        var_tmf2_dn3 = assign14490_e19672_d_n3;
        var_tmf2_dn4 = assign14490_e19672_d_n4;
        var_tmf2_dn5 = assign14490_e19672_d_n5;
        var_tmf2_db0 = assign14490_e19672_d_b0;
        var_tmf2_db1 = assign14490_e19672_d_b1;
        var_tmf2_db2 = assign14490_e19672_d_b2;
        var_tmf2_db3 = assign14490_e19672_d_b3;

        let (assign14500_e19688, assign14500_e19688_d_n0, assign14500_e19688_d_n1, assign14500_e19688_d_n2, assign14500_e19688_d_n3, assign14500_e19688_d_n4, assign14500_e19688_d_n5, assign14500_e19688_d_b0, assign14500_e19688_d_b1, assign14500_e19688_d_b2, assign14500_e19688_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14500_e19684: f64 = (var_tmf1 + var_tmf2);
        let assign14500_e19685: f64 = (0.5 * assign14500_e19684);
        let assign14500_e19686: f64 = (p.p85 - assign14500_e19685);
        (assign14500_e19686, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign14500_e19688;
        var_nja11_dn0 = assign14500_e19688_d_n0;
        var_nja11_dn1 = assign14500_e19688_d_n1;
        var_nja11_dn2 = assign14500_e19688_d_n2;
        var_nja11_dn3 = assign14500_e19688_d_n3;
        var_nja11_dn4 = assign14500_e19688_d_n4;
        var_nja11_dn5 = assign14500_e19688_d_n5;
        var_nja11_db0 = assign14500_e19688_d_b0;
        var_nja11_db1 = assign14500_e19688_d_b1;
        var_nja11_db2 = assign14500_e19688_d_b2;
        var_nja11_db3 = assign14500_e19688_d_b3;

        let (assign14510_e19702, assign14510_e19702_d_n0, assign14510_e19702_d_n1, assign14510_e19702_d_n2, assign14510_e19702_d_n3, assign14510_e19702_d_n4, assign14510_e19702_d_n5, assign14510_e19702_d_b0, assign14510_e19702_d_b1, assign14510_e19702_d_b2, assign14510_e19702_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14510_e19698: f64 = (var_nja11 - var_nfagat_i);
        let assign14510_e19700: f64 = (assign14510_e19698 - 0.01);
        (assign14510_e19700, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14510_e19702;
        var_tmf1_dn0 = assign14510_e19702_d_n0;
        var_tmf1_dn1 = assign14510_e19702_d_n1;
        var_tmf1_dn2 = assign14510_e19702_d_n2;
        var_tmf1_dn3 = assign14510_e19702_d_n3;
        var_tmf1_dn4 = assign14510_e19702_d_n4;
        var_tmf1_dn5 = assign14510_e19702_d_n5;
        var_tmf1_db0 = assign14510_e19702_d_b0;
        var_tmf1_db1 = assign14510_e19702_d_b1;
        var_tmf1_db2 = assign14510_e19702_d_b2;
        var_tmf1_db3 = assign14510_e19702_d_b3;

        let (assign14520_e19716, assign14520_e19716_d_n0, assign14520_e19716_d_n1, assign14520_e19716_d_n2, assign14520_e19716_d_n3, assign14520_e19716_d_n4, assign14520_e19716_d_n5, assign14520_e19716_d_b0, assign14520_e19716_d_b1, assign14520_e19716_d_b2, assign14520_e19716_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14520_e19712: f64 = (4.0 * var_nfagat_i);
        let assign14520_e19714: f64 = (assign14520_e19712 * 0.01);
        (assign14520_e19714, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14520_e19716;
        var_tmf2_dn0 = assign14520_e19716_d_n0;
        var_tmf2_dn1 = assign14520_e19716_d_n1;
        var_tmf2_dn2 = assign14520_e19716_d_n2;
        var_tmf2_dn3 = assign14520_e19716_d_n3;
        var_tmf2_dn4 = assign14520_e19716_d_n4;
        var_tmf2_dn5 = assign14520_e19716_d_n5;
        var_tmf2_db0 = assign14520_e19716_d_b0;
        var_tmf2_db1 = assign14520_e19716_d_b1;
        var_tmf2_db2 = assign14520_e19716_d_b2;
        var_tmf2_db3 = assign14520_e19716_d_b3;

        let (assign14530_e19732, assign14530_e19732_d_n0, assign14530_e19732_d_n1, assign14530_e19732_d_n2, assign14530_e19732_d_n3, assign14530_e19732_d_n4, assign14530_e19732_d_n5, assign14530_e19732_d_b0, assign14530_e19732_d_b1, assign14530_e19732_d_b2, assign14530_e19732_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n1, assign14530_e19730_d_n2, assign14530_e19730_d_n3, assign14530_e19730_d_n4, assign14530_e19730_d_n5, assign14530_e19730_d_b0, assign14530_e19730_d_b1, assign14530_e19730_d_b2, assign14530_e19730_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14530_e19729: f64 = (-var_tmf2);
                (assign14530_e19729, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14530_e19730, assign14530_e19730_d_n0, assign14530_e19730_d_n1, assign14530_e19730_d_n2, assign14530_e19730_d_n3, assign14530_e19730_d_n4, assign14530_e19730_d_n5, assign14530_e19730_d_b0, assign14530_e19730_d_b1, assign14530_e19730_d_b2, assign14530_e19730_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14530_e19732;
        var_tmf2_dn0 = assign14530_e19732_d_n0;
        var_tmf2_dn1 = assign14530_e19732_d_n1;
        var_tmf2_dn2 = assign14530_e19732_d_n2;
        var_tmf2_dn3 = assign14530_e19732_d_n3;
        var_tmf2_dn4 = assign14530_e19732_d_n4;
        var_tmf2_dn5 = assign14530_e19732_d_n5;
        var_tmf2_db0 = assign14530_e19732_d_b0;
        var_tmf2_db1 = assign14530_e19732_d_b1;
        var_tmf2_db2 = assign14530_e19732_d_b2;
        var_tmf2_db3 = assign14530_e19732_d_b3;

        *var_guard240_slot = var_guard240;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard240: f64,
        var_guard31: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_guard243_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_guard243: f64 = *var_guard243_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign14540_e19747, assign14540_e19747_d_n0, assign14540_e19747_d_n1, assign14540_e19747_d_n2, assign14540_e19747_d_n3, assign14540_e19747_d_n4, assign14540_e19747_d_n5, assign14540_e19747_d_b0, assign14540_e19747_d_b1, assign14540_e19747_d_b2, assign14540_e19747_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14540_e19742: f64 = (var_tmf1 * var_tmf1);
        let assign14540_e19744: f64 = (assign14540_e19742 + var_tmf2);
        let assign14540_e19745: f64 = (assign14540_e19744).sqrt();
        (assign14540_e19745, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14540_e19745)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14540_e19745)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14540_e19745)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14540_e19745)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14540_e19745)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14540_e19745)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14540_e19747;
        var_tmf2_dn0 = assign14540_e19747_d_n0;
        var_tmf2_dn1 = assign14540_e19747_d_n1;
        var_tmf2_dn2 = assign14540_e19747_d_n2;
        var_tmf2_dn3 = assign14540_e19747_d_n3;
        var_tmf2_dn4 = assign14540_e19747_d_n4;
        var_tmf2_dn5 = assign14540_e19747_d_n5;
        var_tmf2_db0 = assign14540_e19747_d_b0;
        var_tmf2_db1 = assign14540_e19747_d_b1;
        var_tmf2_db2 = assign14540_e19747_d_b2;
        var_tmf2_db3 = assign14540_e19747_d_b3;

        let (assign14550_e19763, assign14550_e19763_d_n0, assign14550_e19763_d_n1, assign14550_e19763_d_n2, assign14550_e19763_d_n3, assign14550_e19763_d_n4, assign14550_e19763_d_n5, assign14550_e19763_d_b0, assign14550_e19763_d_b1, assign14550_e19763_d_b2, assign14550_e19763_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14550_e19759: f64 = (var_tmf1 + var_tmf2);
        let assign14550_e19760: f64 = (0.5 * assign14550_e19759);
        let assign14550_e19761: f64 = (var_nfagat_i + assign14550_e19760);
        (assign14550_e19761, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14550_e19763;
        var_nj1_dn0 = assign14550_e19763_d_n0;
        var_nj1_dn1 = assign14550_e19763_d_n1;
        var_nj1_dn2 = assign14550_e19763_d_n2;
        var_nj1_dn3 = assign14550_e19763_d_n3;
        var_nj1_dn4 = assign14550_e19763_d_n4;
        var_nj1_dn5 = assign14550_e19763_d_n5;
        var_nj1_db0 = assign14550_e19763_d_b0;
        var_nj1_db1 = assign14550_e19763_d_b1;
        var_nj1_db2 = assign14550_e19763_d_b2;
        var_nj1_db3 = assign14550_e19763_d_b3;

        let (assign14560_e19777, assign14560_e19777_d_n0, assign14560_e19777_d_n1, assign14560_e19777_d_n2, assign14560_e19777_d_n3, assign14560_e19777_d_n4, assign14560_e19777_d_n5, assign14560_e19777_d_b0, assign14560_e19777_d_b1, assign14560_e19777_d_b2, assign14560_e19777_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14560_e19773: f64 = (p.p85 - var_nj0);
        let assign14560_e19775: f64 = (assign14560_e19773 - 0.01);
        (assign14560_e19775, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14560_e19777;
        var_tmf1_dn0 = assign14560_e19777_d_n0;
        var_tmf1_dn1 = assign14560_e19777_d_n1;
        var_tmf1_dn2 = assign14560_e19777_d_n2;
        var_tmf1_dn3 = assign14560_e19777_d_n3;
        var_tmf1_dn4 = assign14560_e19777_d_n4;
        var_tmf1_dn5 = assign14560_e19777_d_n5;
        var_tmf1_db0 = assign14560_e19777_d_b0;
        var_tmf1_db1 = assign14560_e19777_d_b1;
        var_tmf1_db2 = assign14560_e19777_d_b2;
        var_tmf1_db3 = assign14560_e19777_d_b3;

        let (assign14570_e19791, assign14570_e19791_d_n0, assign14570_e19791_d_n1, assign14570_e19791_d_n2, assign14570_e19791_d_n3, assign14570_e19791_d_n4, assign14570_e19791_d_n5, assign14570_e19791_d_b0, assign14570_e19791_d_b1, assign14570_e19791_d_b2, assign14570_e19791_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14570_e19787: f64 = (4.0 * p.p85);
        let assign14570_e19789: f64 = (assign14570_e19787 * 0.01);
        (assign14570_e19789, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14570_e19791;
        var_tmf2_dn0 = assign14570_e19791_d_n0;
        var_tmf2_dn1 = assign14570_e19791_d_n1;
        var_tmf2_dn2 = assign14570_e19791_d_n2;
        var_tmf2_dn3 = assign14570_e19791_d_n3;
        var_tmf2_dn4 = assign14570_e19791_d_n4;
        var_tmf2_dn5 = assign14570_e19791_d_n5;
        var_tmf2_db0 = assign14570_e19791_d_b0;
        var_tmf2_db1 = assign14570_e19791_d_b1;
        var_tmf2_db2 = assign14570_e19791_d_b2;
        var_tmf2_db3 = assign14570_e19791_d_b3;

        let (assign14580_e19807, assign14580_e19807_d_n0, assign14580_e19807_d_n1, assign14580_e19807_d_n2, assign14580_e19807_d_n3, assign14580_e19807_d_n4, assign14580_e19807_d_n5, assign14580_e19807_d_b0, assign14580_e19807_d_b1, assign14580_e19807_d_b2, assign14580_e19807_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n1, assign14580_e19805_d_n2, assign14580_e19805_d_n3, assign14580_e19805_d_n4, assign14580_e19805_d_n5, assign14580_e19805_d_b0, assign14580_e19805_d_b1, assign14580_e19805_d_b2, assign14580_e19805_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14580_e19804: f64 = (-var_tmf2);
                (assign14580_e19804, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14580_e19805, assign14580_e19805_d_n0, assign14580_e19805_d_n1, assign14580_e19805_d_n2, assign14580_e19805_d_n3, assign14580_e19805_d_n4, assign14580_e19805_d_n5, assign14580_e19805_d_b0, assign14580_e19805_d_b1, assign14580_e19805_d_b2, assign14580_e19805_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14580_e19807;
        var_tmf2_dn0 = assign14580_e19807_d_n0;
        var_tmf2_dn1 = assign14580_e19807_d_n1;
        var_tmf2_dn2 = assign14580_e19807_d_n2;
        var_tmf2_dn3 = assign14580_e19807_d_n3;
        var_tmf2_dn4 = assign14580_e19807_d_n4;
        var_tmf2_dn5 = assign14580_e19807_d_n5;
        var_tmf2_db0 = assign14580_e19807_d_b0;
        var_tmf2_db1 = assign14580_e19807_d_b1;
        var_tmf2_db2 = assign14580_e19807_d_b2;
        var_tmf2_db3 = assign14580_e19807_d_b3;

        let (assign14590_e19822, assign14590_e19822_d_n0, assign14590_e19822_d_n1, assign14590_e19822_d_n2, assign14590_e19822_d_n3, assign14590_e19822_d_n4, assign14590_e19822_d_n5, assign14590_e19822_d_b0, assign14590_e19822_d_b1, assign14590_e19822_d_b2, assign14590_e19822_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14590_e19817: f64 = (var_tmf1 * var_tmf1);
        let assign14590_e19819: f64 = (assign14590_e19817 + var_tmf2);
        let assign14590_e19820: f64 = (assign14590_e19819).sqrt();
        (assign14590_e19820, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14590_e19820)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14590_e19820)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14590_e19820)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14590_e19820)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14590_e19820)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14590_e19820)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14590_e19822;
        var_tmf2_dn0 = assign14590_e19822_d_n0;
        var_tmf2_dn1 = assign14590_e19822_d_n1;
        var_tmf2_dn2 = assign14590_e19822_d_n2;
        var_tmf2_dn3 = assign14590_e19822_d_n3;
        var_tmf2_dn4 = assign14590_e19822_d_n4;
        var_tmf2_dn5 = assign14590_e19822_d_n5;
        var_tmf2_db0 = assign14590_e19822_d_b0;
        var_tmf2_db1 = assign14590_e19822_d_b1;
        var_tmf2_db2 = assign14590_e19822_d_b2;
        var_tmf2_db3 = assign14590_e19822_d_b3;

        let (assign14600_e19838, assign14600_e19838_d_n0, assign14600_e19838_d_n1, assign14600_e19838_d_n2, assign14600_e19838_d_n3, assign14600_e19838_d_n4, assign14600_e19838_d_n5, assign14600_e19838_d_b0, assign14600_e19838_d_b1, assign14600_e19838_d_b2, assign14600_e19838_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14600_e19834: f64 = (var_tmf1 + var_tmf2);
        let assign14600_e19835: f64 = (0.5 * assign14600_e19834);
        let assign14600_e19836: f64 = (p.p85 - assign14600_e19835);
        (assign14600_e19836, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14600_e19838;
        var_nj0_dn0 = assign14600_e19838_d_n0;
        var_nj0_dn1 = assign14600_e19838_d_n1;
        var_nj0_dn2 = assign14600_e19838_d_n2;
        var_nj0_dn3 = assign14600_e19838_d_n3;
        var_nj0_dn4 = assign14600_e19838_d_n4;
        var_nj0_dn5 = assign14600_e19838_d_n5;
        var_nj0_db0 = assign14600_e19838_d_b0;
        var_nj0_db1 = assign14600_e19838_d_b1;
        var_nj0_db2 = assign14600_e19838_d_b2;
        var_nj0_db3 = assign14600_e19838_d_b3;

        let (assign14610_e19852, assign14610_e19852_d_n0, assign14610_e19852_d_n1, assign14610_e19852_d_n2, assign14610_e19852_d_n3, assign14610_e19852_d_n4, assign14610_e19852_d_n5, assign14610_e19852_d_b0, assign14610_e19852_d_b1, assign14610_e19852_d_b2, assign14610_e19852_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14610_e19848: f64 = (var_nj0 - var_nfagat_i);
        let assign14610_e19850: f64 = (assign14610_e19848 - 0.01);
        (assign14610_e19850, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14610_e19852;
        var_tmf1_dn0 = assign14610_e19852_d_n0;
        var_tmf1_dn1 = assign14610_e19852_d_n1;
        var_tmf1_dn2 = assign14610_e19852_d_n2;
        var_tmf1_dn3 = assign14610_e19852_d_n3;
        var_tmf1_dn4 = assign14610_e19852_d_n4;
        var_tmf1_dn5 = assign14610_e19852_d_n5;
        var_tmf1_db0 = assign14610_e19852_d_b0;
        var_tmf1_db1 = assign14610_e19852_d_b1;
        var_tmf1_db2 = assign14610_e19852_d_b2;
        var_tmf1_db3 = assign14610_e19852_d_b3;

        let (assign14620_e19866, assign14620_e19866_d_n0, assign14620_e19866_d_n1, assign14620_e19866_d_n2, assign14620_e19866_d_n3, assign14620_e19866_d_n4, assign14620_e19866_d_n5, assign14620_e19866_d_b0, assign14620_e19866_d_b1, assign14620_e19866_d_b2, assign14620_e19866_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14620_e19862: f64 = (4.0 * var_nfagat_i);
        let assign14620_e19864: f64 = (assign14620_e19862 * 0.01);
        (assign14620_e19864, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14620_e19866;
        var_tmf2_dn0 = assign14620_e19866_d_n0;
        var_tmf2_dn1 = assign14620_e19866_d_n1;
        var_tmf2_dn2 = assign14620_e19866_d_n2;
        var_tmf2_dn3 = assign14620_e19866_d_n3;
        var_tmf2_dn4 = assign14620_e19866_d_n4;
        var_tmf2_dn5 = assign14620_e19866_d_n5;
        var_tmf2_db0 = assign14620_e19866_d_b0;
        var_tmf2_db1 = assign14620_e19866_d_b1;
        var_tmf2_db2 = assign14620_e19866_d_b2;
        var_tmf2_db3 = assign14620_e19866_d_b3;

        let (assign14630_e19882, assign14630_e19882_d_n0, assign14630_e19882_d_n1, assign14630_e19882_d_n2, assign14630_e19882_d_n3, assign14630_e19882_d_n4, assign14630_e19882_d_n5, assign14630_e19882_d_b0, assign14630_e19882_d_b1, assign14630_e19882_d_b2, assign14630_e19882_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n1, assign14630_e19880_d_n2, assign14630_e19880_d_n3, assign14630_e19880_d_n4, assign14630_e19880_d_n5, assign14630_e19880_d_b0, assign14630_e19880_d_b1, assign14630_e19880_d_b2, assign14630_e19880_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14630_e19879: f64 = (-var_tmf2);
                (assign14630_e19879, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14630_e19880, assign14630_e19880_d_n0, assign14630_e19880_d_n1, assign14630_e19880_d_n2, assign14630_e19880_d_n3, assign14630_e19880_d_n4, assign14630_e19880_d_n5, assign14630_e19880_d_b0, assign14630_e19880_d_b1, assign14630_e19880_d_b2, assign14630_e19880_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14630_e19882;
        var_tmf2_dn0 = assign14630_e19882_d_n0;
        var_tmf2_dn1 = assign14630_e19882_d_n1;
        var_tmf2_dn2 = assign14630_e19882_d_n2;
        var_tmf2_dn3 = assign14630_e19882_d_n3;
        var_tmf2_dn4 = assign14630_e19882_d_n4;
        var_tmf2_dn5 = assign14630_e19882_d_n5;
        var_tmf2_db0 = assign14630_e19882_d_b0;
        var_tmf2_db1 = assign14630_e19882_d_b1;
        var_tmf2_db2 = assign14630_e19882_d_b2;
        var_tmf2_db3 = assign14630_e19882_d_b3;

        let (assign14640_e19897, assign14640_e19897_d_n0, assign14640_e19897_d_n1, assign14640_e19897_d_n2, assign14640_e19897_d_n3, assign14640_e19897_d_n4, assign14640_e19897_d_n5, assign14640_e19897_d_b0, assign14640_e19897_d_b1, assign14640_e19897_d_b2, assign14640_e19897_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14640_e19892: f64 = (var_tmf1 * var_tmf1);
        let assign14640_e19894: f64 = (assign14640_e19892 + var_tmf2);
        let assign14640_e19895: f64 = (assign14640_e19894).sqrt();
        (assign14640_e19895, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14640_e19895)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14640_e19895)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14640_e19895)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14640_e19895)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14640_e19895)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14640_e19895)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14640_e19897;
        var_tmf2_dn0 = assign14640_e19897_d_n0;
        var_tmf2_dn1 = assign14640_e19897_d_n1;
        var_tmf2_dn2 = assign14640_e19897_d_n2;
        var_tmf2_dn3 = assign14640_e19897_d_n3;
        var_tmf2_dn4 = assign14640_e19897_d_n4;
        var_tmf2_dn5 = assign14640_e19897_d_n5;
        var_tmf2_db0 = assign14640_e19897_d_b0;
        var_tmf2_db1 = assign14640_e19897_d_b1;
        var_tmf2_db2 = assign14640_e19897_d_b2;
        var_tmf2_db3 = assign14640_e19897_d_b3;

        let (assign14650_e19913, assign14650_e19913_d_n0, assign14650_e19913_d_n1, assign14650_e19913_d_n2, assign14650_e19913_d_n3, assign14650_e19913_d_n4, assign14650_e19913_d_n5, assign14650_e19913_d_b0, assign14650_e19913_d_b1, assign14650_e19913_d_b2, assign14650_e19913_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 != 0.0)) {
        let assign14650_e19909: f64 = (var_tmf1 + var_tmf2);
        let assign14650_e19910: f64 = (0.5 * assign14650_e19909);
        let assign14650_e19911: f64 = (var_nfagat_i + assign14650_e19910);
        (assign14650_e19911, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14650_e19913;
        var_nj0_dn0 = assign14650_e19913_d_n0;
        var_nj0_dn1 = assign14650_e19913_d_n1;
        var_nj0_dn2 = assign14650_e19913_d_n2;
        var_nj0_dn3 = assign14650_e19913_d_n3;
        var_nj0_dn4 = assign14650_e19913_d_n4;
        var_nj0_dn5 = assign14650_e19913_d_n5;
        var_nj0_db0 = assign14650_e19913_d_b0;
        var_nj0_db1 = assign14650_e19913_d_b1;
        var_nj0_db2 = assign14650_e19913_d_b2;
        var_nj0_db3 = assign14650_e19913_d_b3;

        let (assign14660_e19924, assign14660_e19924_d_n0, assign14660_e19924_d_n1, assign14660_e19924_d_n2, assign14660_e19924_d_n3, assign14660_e19924_d_n4, assign14660_e19924_d_n5, assign14660_e19924_d_b0, assign14660_e19924_d_b1, assign14660_e19924_d_b2, assign14660_e19924_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14660_e19924;
        var_nj0_dn0 = assign14660_e19924_d_n0;
        var_nj0_dn1 = assign14660_e19924_d_n1;
        var_nj0_dn2 = assign14660_e19924_d_n2;
        var_nj0_dn3 = assign14660_e19924_d_n3;
        var_nj0_dn4 = assign14660_e19924_d_n4;
        var_nj0_dn5 = assign14660_e19924_d_n5;
        var_nj0_db0 = assign14660_e19924_d_b0;
        var_nj0_db1 = assign14660_e19924_d_b1;
        var_nj0_db2 = assign14660_e19924_d_b2;
        var_nj0_db3 = assign14660_e19924_d_b3;

        let (assign14670_e19935, assign14670_e19935_d_n0, assign14670_e19935_d_n1, assign14670_e19935_d_n2, assign14670_e19935_d_n3, assign14670_e19935_d_n4, assign14670_e19935_d_n5, assign14670_e19935_d_b0, assign14670_e19935_d_b1, assign14670_e19935_d_b2, assign14670_e19935_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 != 0.0)) && (var_guard240 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14670_e19935;
        var_nj1_dn0 = assign14670_e19935_d_n0;
        var_nj1_dn1 = assign14670_e19935_d_n1;
        var_nj1_dn2 = assign14670_e19935_d_n2;
        var_nj1_dn3 = assign14670_e19935_d_n3;
        var_nj1_dn4 = assign14670_e19935_d_n4;
        var_nj1_dn5 = assign14670_e19935_d_n5;
        var_nj1_db0 = assign14670_e19935_d_b0;
        var_nj1_db1 = assign14670_e19935_d_b1;
        var_nj1_db2 = assign14670_e19935_d_b2;
        var_nj1_db3 = assign14670_e19935_d_b3;

        let (assign14740_e20185,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign14740_e20181: f64 = (var_nin * var_nin);
        let assign14740_e20183: f64 = (assign14740_e20181 / var_ndibot_i);
        (assign14740_e20183,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign14740_e20185;

        let (assign14750_e20201,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign14750_e20194: f64 = (var_nfabot_i / var_phitdinv);
        let assign14750_e20197: f64 = (var_ndibot_i / var_pnn0);
        let assign14750_e20198: f64 = (assign14750_e20197).ln();
        let assign14750_e20199: f64 = (assign14750_e20194 * assign14750_e20198);
        (assign14750_e20199,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign14750_e20201;

        let assign14760_e20204: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard243 = assign14760_e20204;

        let (assign14770_e20221, assign14770_e20221_d_n0, assign14770_e20221_d_n1, assign14770_e20221_d_n2, assign14770_e20221_d_n3, assign14770_e20221_d_n4, assign14770_e20221_d_n5, assign14770_e20221_d_b0, assign14770_e20221_d_b1, assign14770_e20221_d_b2, assign14770_e20221_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14770_e20216: f64 = (var_vmax - var_vha1);
        let assign14770_e20217: f64 = (p.p86 * assign14770_e20216);
        let assign14770_e20219: f64 = (assign14770_e20217 + var_nfabot_i);
        (assign14770_e20219, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign14770_e20221;
        var_nja10_dn0 = assign14770_e20221_d_n0;
        var_nja10_dn1 = assign14770_e20221_d_n1;
        var_nja10_dn2 = assign14770_e20221_d_n2;
        var_nja10_dn3 = assign14770_e20221_d_n3;
        var_nja10_dn4 = assign14770_e20221_d_n4;
        var_nja10_dn5 = assign14770_e20221_d_n5;
        var_nja10_db0 = assign14770_e20221_d_b0;
        var_nja10_db1 = assign14770_e20221_d_b1;
        var_nja10_db2 = assign14770_e20221_d_b2;
        var_nja10_db3 = assign14770_e20221_d_b3;

        let (assign14780_e20236, assign14780_e20236_d_n0, assign14780_e20236_d_n1, assign14780_e20236_d_n2, assign14780_e20236_d_n3, assign14780_e20236_d_n4, assign14780_e20236_d_n5, assign14780_e20236_d_b0, assign14780_e20236_d_b1, assign14780_e20236_d_b2, assign14780_e20236_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14780_e20233: f64 = (p.p86 * var_vha1);
        let assign14780_e20234: f64 = (var_nfabot_i - assign14780_e20233);
        (assign14780_e20234, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14780_e20236;
        var_nj0_dn0 = assign14780_e20236_d_n0;
        var_nj0_dn1 = assign14780_e20236_d_n1;
        var_nj0_dn2 = assign14780_e20236_d_n2;
        var_nj0_dn3 = assign14780_e20236_d_n3;
        var_nj0_dn4 = assign14780_e20236_d_n4;
        var_nj0_dn5 = assign14780_e20236_d_n5;
        var_nj0_db0 = assign14780_e20236_d_b0;
        var_nj0_db1 = assign14780_e20236_d_b1;
        var_nj0_db2 = assign14780_e20236_d_b2;
        var_nj0_db3 = assign14780_e20236_d_b3;

        let (assign14790_e20251, assign14790_e20251_d_n0, assign14790_e20251_d_n1, assign14790_e20251_d_n2, assign14790_e20251_d_n3, assign14790_e20251_d_n4, assign14790_e20251_d_n5, assign14790_e20251_d_b0, assign14790_e20251_d_b1, assign14790_e20251_d_b2, assign14790_e20251_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14790_e20247: f64 = (p.p85 - var_nja10);
        let assign14790_e20249: f64 = (assign14790_e20247 - 0.01);
        (assign14790_e20249, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14790_e20251;
        var_tmf1_dn0 = assign14790_e20251_d_n0;
        var_tmf1_dn1 = assign14790_e20251_d_n1;
        var_tmf1_dn2 = assign14790_e20251_d_n2;
        var_tmf1_dn3 = assign14790_e20251_d_n3;
        var_tmf1_dn4 = assign14790_e20251_d_n4;
        var_tmf1_dn5 = assign14790_e20251_d_n5;
        var_tmf1_db0 = assign14790_e20251_d_b0;
        var_tmf1_db1 = assign14790_e20251_d_b1;
        var_tmf1_db2 = assign14790_e20251_d_b2;
        var_tmf1_db3 = assign14790_e20251_d_b3;

        let (assign14800_e20266, assign14800_e20266_d_n0, assign14800_e20266_d_n1, assign14800_e20266_d_n2, assign14800_e20266_d_n3, assign14800_e20266_d_n4, assign14800_e20266_d_n5, assign14800_e20266_d_b0, assign14800_e20266_d_b1, assign14800_e20266_d_b2, assign14800_e20266_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14800_e20262: f64 = (4.0 * p.p85);
        let assign14800_e20264: f64 = (assign14800_e20262 * 0.01);
        (assign14800_e20264, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14800_e20266;
        var_tmf2_dn0 = assign14800_e20266_d_n0;
        var_tmf2_dn1 = assign14800_e20266_d_n1;
        var_tmf2_dn2 = assign14800_e20266_d_n2;
        var_tmf2_dn3 = assign14800_e20266_d_n3;
        var_tmf2_dn4 = assign14800_e20266_d_n4;
        var_tmf2_dn5 = assign14800_e20266_d_n5;
        var_tmf2_db0 = assign14800_e20266_d_b0;
        var_tmf2_db1 = assign14800_e20266_d_b1;
        var_tmf2_db2 = assign14800_e20266_d_b2;
        var_tmf2_db3 = assign14800_e20266_d_b3;

        let (assign14810_e20283, assign14810_e20283_d_n0, assign14810_e20283_d_n1, assign14810_e20283_d_n2, assign14810_e20283_d_n3, assign14810_e20283_d_n4, assign14810_e20283_d_n5, assign14810_e20283_d_b0, assign14810_e20283_d_b1, assign14810_e20283_d_b2, assign14810_e20283_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n1, assign14810_e20281_d_n2, assign14810_e20281_d_n3, assign14810_e20281_d_n4, assign14810_e20281_d_n5, assign14810_e20281_d_b0, assign14810_e20281_d_b1, assign14810_e20281_d_b2, assign14810_e20281_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14810_e20280: f64 = (-var_tmf2);
                (assign14810_e20280, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14810_e20281, assign14810_e20281_d_n0, assign14810_e20281_d_n1, assign14810_e20281_d_n2, assign14810_e20281_d_n3, assign14810_e20281_d_n4, assign14810_e20281_d_n5, assign14810_e20281_d_b0, assign14810_e20281_d_b1, assign14810_e20281_d_b2, assign14810_e20281_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14810_e20283;
        var_tmf2_dn0 = assign14810_e20283_d_n0;
        var_tmf2_dn1 = assign14810_e20283_d_n1;
        var_tmf2_dn2 = assign14810_e20283_d_n2;
        var_tmf2_dn3 = assign14810_e20283_d_n3;
        var_tmf2_dn4 = assign14810_e20283_d_n4;
        var_tmf2_dn5 = assign14810_e20283_d_n5;
        var_tmf2_db0 = assign14810_e20283_d_b0;
        var_tmf2_db1 = assign14810_e20283_d_b1;
        var_tmf2_db2 = assign14810_e20283_d_b2;
        var_tmf2_db3 = assign14810_e20283_d_b3;

        let (assign14820_e20299, assign14820_e20299_d_n0, assign14820_e20299_d_n1, assign14820_e20299_d_n2, assign14820_e20299_d_n3, assign14820_e20299_d_n4, assign14820_e20299_d_n5, assign14820_e20299_d_b0, assign14820_e20299_d_b1, assign14820_e20299_d_b2, assign14820_e20299_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14820_e20294: f64 = (var_tmf1 * var_tmf1);
        let assign14820_e20296: f64 = (assign14820_e20294 + var_tmf2);
        let assign14820_e20297: f64 = (assign14820_e20296).sqrt();
        (assign14820_e20297, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14820_e20297)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14820_e20297)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14820_e20297)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14820_e20297)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14820_e20297)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14820_e20297)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14820_e20299;
        var_tmf2_dn0 = assign14820_e20299_d_n0;
        var_tmf2_dn1 = assign14820_e20299_d_n1;
        var_tmf2_dn2 = assign14820_e20299_d_n2;
        var_tmf2_dn3 = assign14820_e20299_d_n3;
        var_tmf2_dn4 = assign14820_e20299_d_n4;
        var_tmf2_dn5 = assign14820_e20299_d_n5;
        var_tmf2_db0 = assign14820_e20299_d_b0;
        var_tmf2_db1 = assign14820_e20299_d_b1;
        var_tmf2_db2 = assign14820_e20299_d_b2;
        var_tmf2_db3 = assign14820_e20299_d_b3;

        let (assign14830_e20316, assign14830_e20316_d_n0, assign14830_e20316_d_n1, assign14830_e20316_d_n2, assign14830_e20316_d_n3, assign14830_e20316_d_n4, assign14830_e20316_d_n5, assign14830_e20316_d_b0, assign14830_e20316_d_b1, assign14830_e20316_d_b2, assign14830_e20316_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14830_e20312: f64 = (var_tmf1 / var_tmf2);
        let assign14830_e20313: f64 = (1.0 + assign14830_e20312);
        let assign14830_e20314: f64 = (0.5 * assign14830_e20313);
        (assign14830_e20314, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign14830_e20316;
        var_dfn_su_dn0 = assign14830_e20316_d_n0;
        var_dfn_su_dn1 = assign14830_e20316_d_n1;
        var_dfn_su_dn2 = assign14830_e20316_d_n2;
        var_dfn_su_dn3 = assign14830_e20316_d_n3;
        var_dfn_su_dn4 = assign14830_e20316_d_n4;
        var_dfn_su_dn5 = assign14830_e20316_d_n5;
        var_dfn_su_db0 = assign14830_e20316_d_b0;
        var_dfn_su_db1 = assign14830_e20316_d_b1;
        var_dfn_su_db2 = assign14830_e20316_d_b2;
        var_dfn_su_db3 = assign14830_e20316_d_b3;

        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_guard243_slot = var_guard243;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard230: f64,
        var_guard231: f64,
        var_guard243: f64,
        var_guard31: f64,
        var_nfabot_i: f64,
        var_phitdinv: f64,
        var_vha1: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn5_slot: &mut f64,
        var_guard244_slot: &mut f64,
        var_guard245_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_db0: f64 = *var_exp_vmax_over_phitd_bot_db0_slot;
        let mut var_exp_vmax_over_phitd_bot_db1: f64 = *var_exp_vmax_over_phitd_bot_db1_slot;
        let mut var_exp_vmax_over_phitd_bot_db2: f64 = *var_exp_vmax_over_phitd_bot_db2_slot;
        let mut var_exp_vmax_over_phitd_bot_db3: f64 = *var_exp_vmax_over_phitd_bot_db3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn1: f64 = *var_exp_vmax_over_phitd_bot_dn1_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_dn3: f64 = *var_exp_vmax_over_phitd_bot_dn3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn4: f64 = *var_exp_vmax_over_phitd_bot_dn4_slot;
        let mut var_exp_vmax_over_phitd_bot_dn5: f64 = *var_exp_vmax_over_phitd_bot_dn5_slot;
        let mut var_guard244: f64 = *var_guard244_slot;
        let mut var_guard245: f64 = *var_guard245_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;

        let (assign14840_e20333, assign14840_e20333_d_n0, assign14840_e20333_d_n1, assign14840_e20333_d_n2, assign14840_e20333_d_n3, assign14840_e20333_d_n4, assign14840_e20333_d_n5, assign14840_e20333_d_b0, assign14840_e20333_d_b1, assign14840_e20333_d_b2, assign14840_e20333_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14840_e20329: f64 = (var_tmf1 + var_tmf2);
        let assign14840_e20330: f64 = (0.5 * assign14840_e20329);
        let assign14840_e20331: f64 = (p.p85 - assign14840_e20330);
        (assign14840_e20331, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign14840_e20333;
        var_nja11_dn0 = assign14840_e20333_d_n0;
        var_nja11_dn1 = assign14840_e20333_d_n1;
        var_nja11_dn2 = assign14840_e20333_d_n2;
        var_nja11_dn3 = assign14840_e20333_d_n3;
        var_nja11_dn4 = assign14840_e20333_d_n4;
        var_nja11_dn5 = assign14840_e20333_d_n5;
        var_nja11_db0 = assign14840_e20333_d_b0;
        var_nja11_db1 = assign14840_e20333_d_b1;
        var_nja11_db2 = assign14840_e20333_d_b2;
        var_nja11_db3 = assign14840_e20333_d_b3;

        let (assign14850_e20348, assign14850_e20348_d_n0, assign14850_e20348_d_n1, assign14850_e20348_d_n2, assign14850_e20348_d_n3, assign14850_e20348_d_n4, assign14850_e20348_d_n5, assign14850_e20348_d_b0, assign14850_e20348_d_b1, assign14850_e20348_d_b2, assign14850_e20348_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14850_e20344: f64 = (var_nja11 - var_nfabot_i);
        let assign14850_e20346: f64 = (assign14850_e20344 - 0.01);
        (assign14850_e20346, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14850_e20348;
        var_tmf1_dn0 = assign14850_e20348_d_n0;
        var_tmf1_dn1 = assign14850_e20348_d_n1;
        var_tmf1_dn2 = assign14850_e20348_d_n2;
        var_tmf1_dn3 = assign14850_e20348_d_n3;
        var_tmf1_dn4 = assign14850_e20348_d_n4;
        var_tmf1_dn5 = assign14850_e20348_d_n5;
        var_tmf1_db0 = assign14850_e20348_d_b0;
        var_tmf1_db1 = assign14850_e20348_d_b1;
        var_tmf1_db2 = assign14850_e20348_d_b2;
        var_tmf1_db3 = assign14850_e20348_d_b3;

        let (assign14860_e20363, assign14860_e20363_d_n0, assign14860_e20363_d_n1, assign14860_e20363_d_n2, assign14860_e20363_d_n3, assign14860_e20363_d_n4, assign14860_e20363_d_n5, assign14860_e20363_d_b0, assign14860_e20363_d_b1, assign14860_e20363_d_b2, assign14860_e20363_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14860_e20359: f64 = (4.0 * var_nfabot_i);
        let assign14860_e20361: f64 = (assign14860_e20359 * 0.01);
        (assign14860_e20361, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14860_e20363;
        var_tmf2_dn0 = assign14860_e20363_d_n0;
        var_tmf2_dn1 = assign14860_e20363_d_n1;
        var_tmf2_dn2 = assign14860_e20363_d_n2;
        var_tmf2_dn3 = assign14860_e20363_d_n3;
        var_tmf2_dn4 = assign14860_e20363_d_n4;
        var_tmf2_dn5 = assign14860_e20363_d_n5;
        var_tmf2_db0 = assign14860_e20363_d_b0;
        var_tmf2_db1 = assign14860_e20363_d_b1;
        var_tmf2_db2 = assign14860_e20363_d_b2;
        var_tmf2_db3 = assign14860_e20363_d_b3;

        let (assign14870_e20380, assign14870_e20380_d_n0, assign14870_e20380_d_n1, assign14870_e20380_d_n2, assign14870_e20380_d_n3, assign14870_e20380_d_n4, assign14870_e20380_d_n5, assign14870_e20380_d_b0, assign14870_e20380_d_b1, assign14870_e20380_d_b2, assign14870_e20380_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n1, assign14870_e20378_d_n2, assign14870_e20378_d_n3, assign14870_e20378_d_n4, assign14870_e20378_d_n5, assign14870_e20378_d_b0, assign14870_e20378_d_b1, assign14870_e20378_d_b2, assign14870_e20378_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14870_e20377: f64 = (-var_tmf2);
                (assign14870_e20377, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14870_e20378, assign14870_e20378_d_n0, assign14870_e20378_d_n1, assign14870_e20378_d_n2, assign14870_e20378_d_n3, assign14870_e20378_d_n4, assign14870_e20378_d_n5, assign14870_e20378_d_b0, assign14870_e20378_d_b1, assign14870_e20378_d_b2, assign14870_e20378_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14870_e20380;
        var_tmf2_dn0 = assign14870_e20380_d_n0;
        var_tmf2_dn1 = assign14870_e20380_d_n1;
        var_tmf2_dn2 = assign14870_e20380_d_n2;
        var_tmf2_dn3 = assign14870_e20380_d_n3;
        var_tmf2_dn4 = assign14870_e20380_d_n4;
        var_tmf2_dn5 = assign14870_e20380_d_n5;
        var_tmf2_db0 = assign14870_e20380_d_b0;
        var_tmf2_db1 = assign14870_e20380_d_b1;
        var_tmf2_db2 = assign14870_e20380_d_b2;
        var_tmf2_db3 = assign14870_e20380_d_b3;

        let (assign14880_e20396, assign14880_e20396_d_n0, assign14880_e20396_d_n1, assign14880_e20396_d_n2, assign14880_e20396_d_n3, assign14880_e20396_d_n4, assign14880_e20396_d_n5, assign14880_e20396_d_b0, assign14880_e20396_d_b1, assign14880_e20396_d_b2, assign14880_e20396_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14880_e20391: f64 = (var_tmf1 * var_tmf1);
        let assign14880_e20393: f64 = (assign14880_e20391 + var_tmf2);
        let assign14880_e20394: f64 = (assign14880_e20393).sqrt();
        (assign14880_e20394, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14880_e20394)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14880_e20394)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14880_e20394)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14880_e20394)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14880_e20394)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14880_e20394)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14880_e20396;
        var_tmf2_dn0 = assign14880_e20396_d_n0;
        var_tmf2_dn1 = assign14880_e20396_d_n1;
        var_tmf2_dn2 = assign14880_e20396_d_n2;
        var_tmf2_dn3 = assign14880_e20396_d_n3;
        var_tmf2_dn4 = assign14880_e20396_d_n4;
        var_tmf2_dn5 = assign14880_e20396_d_n5;
        var_tmf2_db0 = assign14880_e20396_d_b0;
        var_tmf2_db1 = assign14880_e20396_d_b1;
        var_tmf2_db2 = assign14880_e20396_d_b2;
        var_tmf2_db3 = assign14880_e20396_d_b3;

        let (assign14890_e20413, assign14890_e20413_d_n0, assign14890_e20413_d_n1, assign14890_e20413_d_n2, assign14890_e20413_d_n3, assign14890_e20413_d_n4, assign14890_e20413_d_n5, assign14890_e20413_d_b0, assign14890_e20413_d_b1, assign14890_e20413_d_b2, assign14890_e20413_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14890_e20409: f64 = (var_tmf1 / var_tmf2);
        let assign14890_e20410: f64 = (1.0 + assign14890_e20409);
        let assign14890_e20411: f64 = (0.5 * assign14890_e20410);
        (assign14890_e20411, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign14890_e20413;
        var_dfn_sl_dn0 = assign14890_e20413_d_n0;
        var_dfn_sl_dn1 = assign14890_e20413_d_n1;
        var_dfn_sl_dn2 = assign14890_e20413_d_n2;
        var_dfn_sl_dn3 = assign14890_e20413_d_n3;
        var_dfn_sl_dn4 = assign14890_e20413_d_n4;
        var_dfn_sl_dn5 = assign14890_e20413_d_n5;
        var_dfn_sl_db0 = assign14890_e20413_d_b0;
        var_dfn_sl_db1 = assign14890_e20413_d_b1;
        var_dfn_sl_db2 = assign14890_e20413_d_b2;
        var_dfn_sl_db3 = assign14890_e20413_d_b3;

        let (assign14900_e20430, assign14900_e20430_d_n0, assign14900_e20430_d_n1, assign14900_e20430_d_n2, assign14900_e20430_d_n3, assign14900_e20430_d_n4, assign14900_e20430_d_n5, assign14900_e20430_d_b0, assign14900_e20430_d_b1, assign14900_e20430_d_b2, assign14900_e20430_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14900_e20426: f64 = (var_tmf1 + var_tmf2);
        let assign14900_e20427: f64 = (0.5 * assign14900_e20426);
        let assign14900_e20428: f64 = (var_nfabot_i + assign14900_e20427);
        (assign14900_e20428, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign14900_e20430;
        var_nj1_dn0 = assign14900_e20430_d_n0;
        var_nj1_dn1 = assign14900_e20430_d_n1;
        var_nj1_dn2 = assign14900_e20430_d_n2;
        var_nj1_dn3 = assign14900_e20430_d_n3;
        var_nj1_dn4 = assign14900_e20430_d_n4;
        var_nj1_dn5 = assign14900_e20430_d_n5;
        var_nj1_db0 = assign14900_e20430_d_b0;
        var_nj1_db1 = assign14900_e20430_d_b1;
        var_nj1_db2 = assign14900_e20430_d_b2;
        var_nj1_db3 = assign14900_e20430_d_b3;

        let (assign14910_e20445, assign14910_e20445_d_n0, assign14910_e20445_d_n1, assign14910_e20445_d_n2, assign14910_e20445_d_n3, assign14910_e20445_d_n4, assign14910_e20445_d_n5, assign14910_e20445_d_b0, assign14910_e20445_d_b1, assign14910_e20445_d_b2, assign14910_e20445_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14910_e20441: f64 = (p.p85 - var_nj0);
        let assign14910_e20443: f64 = (assign14910_e20441 - 0.01);
        (assign14910_e20443, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14910_e20445;
        var_tmf1_dn0 = assign14910_e20445_d_n0;
        var_tmf1_dn1 = assign14910_e20445_d_n1;
        var_tmf1_dn2 = assign14910_e20445_d_n2;
        var_tmf1_dn3 = assign14910_e20445_d_n3;
        var_tmf1_dn4 = assign14910_e20445_d_n4;
        var_tmf1_dn5 = assign14910_e20445_d_n5;
        var_tmf1_db0 = assign14910_e20445_d_b0;
        var_tmf1_db1 = assign14910_e20445_d_b1;
        var_tmf1_db2 = assign14910_e20445_d_b2;
        var_tmf1_db3 = assign14910_e20445_d_b3;

        let (assign14920_e20460, assign14920_e20460_d_n0, assign14920_e20460_d_n1, assign14920_e20460_d_n2, assign14920_e20460_d_n3, assign14920_e20460_d_n4, assign14920_e20460_d_n5, assign14920_e20460_d_b0, assign14920_e20460_d_b1, assign14920_e20460_d_b2, assign14920_e20460_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14920_e20456: f64 = (4.0 * p.p85);
        let assign14920_e20458: f64 = (assign14920_e20456 * 0.01);
        (assign14920_e20458, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14920_e20460;
        var_tmf2_dn0 = assign14920_e20460_d_n0;
        var_tmf2_dn1 = assign14920_e20460_d_n1;
        var_tmf2_dn2 = assign14920_e20460_d_n2;
        var_tmf2_dn3 = assign14920_e20460_d_n3;
        var_tmf2_dn4 = assign14920_e20460_d_n4;
        var_tmf2_dn5 = assign14920_e20460_d_n5;
        var_tmf2_db0 = assign14920_e20460_d_b0;
        var_tmf2_db1 = assign14920_e20460_d_b1;
        var_tmf2_db2 = assign14920_e20460_d_b2;
        var_tmf2_db3 = assign14920_e20460_d_b3;

        let (assign14930_e20477, assign14930_e20477_d_n0, assign14930_e20477_d_n1, assign14930_e20477_d_n2, assign14930_e20477_d_n3, assign14930_e20477_d_n4, assign14930_e20477_d_n5, assign14930_e20477_d_b0, assign14930_e20477_d_b1, assign14930_e20477_d_b2, assign14930_e20477_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n1, assign14930_e20475_d_n2, assign14930_e20475_d_n3, assign14930_e20475_d_n4, assign14930_e20475_d_n5, assign14930_e20475_d_b0, assign14930_e20475_d_b1, assign14930_e20475_d_b2, assign14930_e20475_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14930_e20474: f64 = (-var_tmf2);
                (assign14930_e20474, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14930_e20475, assign14930_e20475_d_n0, assign14930_e20475_d_n1, assign14930_e20475_d_n2, assign14930_e20475_d_n3, assign14930_e20475_d_n4, assign14930_e20475_d_n5, assign14930_e20475_d_b0, assign14930_e20475_d_b1, assign14930_e20475_d_b2, assign14930_e20475_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14930_e20477;
        var_tmf2_dn0 = assign14930_e20477_d_n0;
        var_tmf2_dn1 = assign14930_e20477_d_n1;
        var_tmf2_dn2 = assign14930_e20477_d_n2;
        var_tmf2_dn3 = assign14930_e20477_d_n3;
        var_tmf2_dn4 = assign14930_e20477_d_n4;
        var_tmf2_dn5 = assign14930_e20477_d_n5;
        var_tmf2_db0 = assign14930_e20477_d_b0;
        var_tmf2_db1 = assign14930_e20477_d_b1;
        var_tmf2_db2 = assign14930_e20477_d_b2;
        var_tmf2_db3 = assign14930_e20477_d_b3;

        let (assign14940_e20493, assign14940_e20493_d_n0, assign14940_e20493_d_n1, assign14940_e20493_d_n2, assign14940_e20493_d_n3, assign14940_e20493_d_n4, assign14940_e20493_d_n5, assign14940_e20493_d_b0, assign14940_e20493_d_b1, assign14940_e20493_d_b2, assign14940_e20493_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14940_e20488: f64 = (var_tmf1 * var_tmf1);
        let assign14940_e20490: f64 = (assign14940_e20488 + var_tmf2);
        let assign14940_e20491: f64 = (assign14940_e20490).sqrt();
        (assign14940_e20491, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14940_e20491)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14940_e20491)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14940_e20491)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14940_e20491)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14940_e20491)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14940_e20491)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14940_e20493;
        var_tmf2_dn0 = assign14940_e20493_d_n0;
        var_tmf2_dn1 = assign14940_e20493_d_n1;
        var_tmf2_dn2 = assign14940_e20493_d_n2;
        var_tmf2_dn3 = assign14940_e20493_d_n3;
        var_tmf2_dn4 = assign14940_e20493_d_n4;
        var_tmf2_dn5 = assign14940_e20493_d_n5;
        var_tmf2_db0 = assign14940_e20493_d_b0;
        var_tmf2_db1 = assign14940_e20493_d_b1;
        var_tmf2_db2 = assign14940_e20493_d_b2;
        var_tmf2_db3 = assign14940_e20493_d_b3;

        let (assign14950_e20510, assign14950_e20510_d_n0, assign14950_e20510_d_n1, assign14950_e20510_d_n2, assign14950_e20510_d_n3, assign14950_e20510_d_n4, assign14950_e20510_d_n5, assign14950_e20510_d_b0, assign14950_e20510_d_b1, assign14950_e20510_d_b2, assign14950_e20510_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14950_e20506: f64 = (var_tmf1 + var_tmf2);
        let assign14950_e20507: f64 = (0.5 * assign14950_e20506);
        let assign14950_e20508: f64 = (p.p85 - assign14950_e20507);
        (assign14950_e20508, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign14950_e20510;
        var_nj0_dn0 = assign14950_e20510_d_n0;
        var_nj0_dn1 = assign14950_e20510_d_n1;
        var_nj0_dn2 = assign14950_e20510_d_n2;
        var_nj0_dn3 = assign14950_e20510_d_n3;
        var_nj0_dn4 = assign14950_e20510_d_n4;
        var_nj0_dn5 = assign14950_e20510_d_n5;
        var_nj0_db0 = assign14950_e20510_d_b0;
        var_nj0_db1 = assign14950_e20510_d_b1;
        var_nj0_db2 = assign14950_e20510_d_b2;
        var_nj0_db3 = assign14950_e20510_d_b3;

        let (assign14960_e20525, assign14960_e20525_d_n0, assign14960_e20525_d_n1, assign14960_e20525_d_n2, assign14960_e20525_d_n3, assign14960_e20525_d_n4, assign14960_e20525_d_n5, assign14960_e20525_d_b0, assign14960_e20525_d_b1, assign14960_e20525_d_b2, assign14960_e20525_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14960_e20521: f64 = (var_nj0 - var_nfabot_i);
        let assign14960_e20523: f64 = (assign14960_e20521 - 0.01);
        (assign14960_e20523, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign14960_e20525;
        var_tmf1_dn0 = assign14960_e20525_d_n0;
        var_tmf1_dn1 = assign14960_e20525_d_n1;
        var_tmf1_dn2 = assign14960_e20525_d_n2;
        var_tmf1_dn3 = assign14960_e20525_d_n3;
        var_tmf1_dn4 = assign14960_e20525_d_n4;
        var_tmf1_dn5 = assign14960_e20525_d_n5;
        var_tmf1_db0 = assign14960_e20525_d_b0;
        var_tmf1_db1 = assign14960_e20525_d_b1;
        var_tmf1_db2 = assign14960_e20525_d_b2;
        var_tmf1_db3 = assign14960_e20525_d_b3;

        let (assign14970_e20540, assign14970_e20540_d_n0, assign14970_e20540_d_n1, assign14970_e20540_d_n2, assign14970_e20540_d_n3, assign14970_e20540_d_n4, assign14970_e20540_d_n5, assign14970_e20540_d_b0, assign14970_e20540_d_b1, assign14970_e20540_d_b2, assign14970_e20540_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14970_e20536: f64 = (4.0 * var_nfabot_i);
        let assign14970_e20538: f64 = (assign14970_e20536 * 0.01);
        (assign14970_e20538, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14970_e20540;
        var_tmf2_dn0 = assign14970_e20540_d_n0;
        var_tmf2_dn1 = assign14970_e20540_d_n1;
        var_tmf2_dn2 = assign14970_e20540_d_n2;
        var_tmf2_dn3 = assign14970_e20540_d_n3;
        var_tmf2_dn4 = assign14970_e20540_d_n4;
        var_tmf2_dn5 = assign14970_e20540_d_n5;
        var_tmf2_db0 = assign14970_e20540_d_b0;
        var_tmf2_db1 = assign14970_e20540_d_b1;
        var_tmf2_db2 = assign14970_e20540_d_b2;
        var_tmf2_db3 = assign14970_e20540_d_b3;

        let (assign14980_e20557, assign14980_e20557_d_n0, assign14980_e20557_d_n1, assign14980_e20557_d_n2, assign14980_e20557_d_n3, assign14980_e20557_d_n4, assign14980_e20557_d_n5, assign14980_e20557_d_b0, assign14980_e20557_d_b1, assign14980_e20557_d_b2, assign14980_e20557_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n1, assign14980_e20555_d_n2, assign14980_e20555_d_n3, assign14980_e20555_d_n4, assign14980_e20555_d_n5, assign14980_e20555_d_b0, assign14980_e20555_d_b1, assign14980_e20555_d_b2, assign14980_e20555_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign14980_e20554: f64 = (-var_tmf2);
                (assign14980_e20554, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign14980_e20555, assign14980_e20555_d_n0, assign14980_e20555_d_n1, assign14980_e20555_d_n2, assign14980_e20555_d_n3, assign14980_e20555_d_n4, assign14980_e20555_d_n5, assign14980_e20555_d_b0, assign14980_e20555_d_b1, assign14980_e20555_d_b2, assign14980_e20555_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14980_e20557;
        var_tmf2_dn0 = assign14980_e20557_d_n0;
        var_tmf2_dn1 = assign14980_e20557_d_n1;
        var_tmf2_dn2 = assign14980_e20557_d_n2;
        var_tmf2_dn3 = assign14980_e20557_d_n3;
        var_tmf2_dn4 = assign14980_e20557_d_n4;
        var_tmf2_dn5 = assign14980_e20557_d_n5;
        var_tmf2_db0 = assign14980_e20557_d_b0;
        var_tmf2_db1 = assign14980_e20557_d_b1;
        var_tmf2_db2 = assign14980_e20557_d_b2;
        var_tmf2_db3 = assign14980_e20557_d_b3;

        let (assign14990_e20573, assign14990_e20573_d_n0, assign14990_e20573_d_n1, assign14990_e20573_d_n2, assign14990_e20573_d_n3, assign14990_e20573_d_n4, assign14990_e20573_d_n5, assign14990_e20573_d_b0, assign14990_e20573_d_b1, assign14990_e20573_d_b2, assign14990_e20573_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign14990_e20568: f64 = (var_tmf1 * var_tmf1);
        let assign14990_e20570: f64 = (assign14990_e20568 + var_tmf2);
        let assign14990_e20571: f64 = (assign14990_e20570).sqrt();
        (assign14990_e20571, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14990_e20571)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14990_e20571)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign14990_e20571)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign14990_e20571)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign14990_e20571)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign14990_e20571)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign14990_e20573;
        var_tmf2_dn0 = assign14990_e20573_d_n0;
        var_tmf2_dn1 = assign14990_e20573_d_n1;
        var_tmf2_dn2 = assign14990_e20573_d_n2;
        var_tmf2_dn3 = assign14990_e20573_d_n3;
        var_tmf2_dn4 = assign14990_e20573_d_n4;
        var_tmf2_dn5 = assign14990_e20573_d_n5;
        var_tmf2_db0 = assign14990_e20573_d_b0;
        var_tmf2_db1 = assign14990_e20573_d_b1;
        var_tmf2_db2 = assign14990_e20573_d_b2;
        var_tmf2_db3 = assign14990_e20573_d_b3;

        let (assign15000_e20590, assign15000_e20590_d_n0, assign15000_e20590_d_n1, assign15000_e20590_d_n2, assign15000_e20590_d_n3, assign15000_e20590_d_n4, assign15000_e20590_d_n5, assign15000_e20590_d_b0, assign15000_e20590_d_b1, assign15000_e20590_d_b2, assign15000_e20590_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign15000_e20586: f64 = (var_tmf1 + var_tmf2);
        let assign15000_e20587: f64 = (0.5 * assign15000_e20586);
        let assign15000_e20588: f64 = (var_nfabot_i + assign15000_e20587);
        (assign15000_e20588, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15000_e20590;
        var_nj0_dn0 = assign15000_e20590_d_n0;
        var_nj0_dn1 = assign15000_e20590_d_n1;
        var_nj0_dn2 = assign15000_e20590_d_n2;
        var_nj0_dn3 = assign15000_e20590_d_n3;
        var_nj0_dn4 = assign15000_e20590_d_n4;
        var_nj0_dn5 = assign15000_e20590_d_n5;
        var_nj0_db0 = assign15000_e20590_d_b0;
        var_nj0_db1 = assign15000_e20590_d_b1;
        var_nj0_db2 = assign15000_e20590_d_b2;
        var_nj0_db3 = assign15000_e20590_d_b3;

        let (assign15010_e20605, assign15010_e20605_d_n0, assign15010_e20605_d_n1, assign15010_e20605_d_n2, assign15010_e20605_d_n3, assign15010_e20605_d_n4, assign15010_e20605_d_n5, assign15010_e20605_d_b0, assign15010_e20605_d_b1, assign15010_e20605_d_b2, assign15010_e20605_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 != 0.0)) {
        let assign15010_e20601: f64 = (p.p86 * var_dfn_su);
        let assign15010_e20603: f64 = (assign15010_e20601 * var_dfn_sl);
        (assign15010_e20603, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign15010_e20601 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15010_e20605;
        var_dnj1_dv_dn0 = assign15010_e20605_d_n0;
        var_dnj1_dv_dn1 = assign15010_e20605_d_n1;
        var_dnj1_dv_dn2 = assign15010_e20605_d_n2;
        var_dnj1_dv_dn3 = assign15010_e20605_d_n3;
        var_dnj1_dv_dn4 = assign15010_e20605_d_n4;
        var_dnj1_dv_dn5 = assign15010_e20605_d_n5;
        var_dnj1_dv_db0 = assign15010_e20605_d_b0;
        var_dnj1_dv_db1 = assign15010_e20605_d_b1;
        var_dnj1_dv_db2 = assign15010_e20605_d_b2;
        var_dnj1_dv_db3 = assign15010_e20605_d_b3;

        let (assign15020_e20617, assign15020_e20617_d_n0, assign15020_e20617_d_n1, assign15020_e20617_d_n2, assign15020_e20617_d_n3, assign15020_e20617_d_n4, assign15020_e20617_d_n5, assign15020_e20617_d_b0, assign15020_e20617_d_b1, assign15020_e20617_d_b2, assign15020_e20617_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15020_e20617;
        var_nj0_dn0 = assign15020_e20617_d_n0;
        var_nj0_dn1 = assign15020_e20617_d_n1;
        var_nj0_dn2 = assign15020_e20617_d_n2;
        var_nj0_dn3 = assign15020_e20617_d_n3;
        var_nj0_dn4 = assign15020_e20617_d_n4;
        var_nj0_dn5 = assign15020_e20617_d_n5;
        var_nj0_db0 = assign15020_e20617_d_b0;
        var_nj0_db1 = assign15020_e20617_d_b1;
        var_nj0_db2 = assign15020_e20617_d_b2;
        var_nj0_db3 = assign15020_e20617_d_b3;

        let (assign15030_e20629, assign15030_e20629_d_n0, assign15030_e20629_d_n1, assign15030_e20629_d_n2, assign15030_e20629_d_n3, assign15030_e20629_d_n4, assign15030_e20629_d_n5, assign15030_e20629_d_b0, assign15030_e20629_d_b1, assign15030_e20629_d_b2, assign15030_e20629_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign15030_e20629;
        var_nj1_dn0 = assign15030_e20629_d_n0;
        var_nj1_dn1 = assign15030_e20629_d_n1;
        var_nj1_dn2 = assign15030_e20629_d_n2;
        var_nj1_dn3 = assign15030_e20629_d_n3;
        var_nj1_dn4 = assign15030_e20629_d_n4;
        var_nj1_dn5 = assign15030_e20629_d_n5;
        var_nj1_db0 = assign15030_e20629_d_b0;
        var_nj1_db1 = assign15030_e20629_d_b1;
        var_nj1_db2 = assign15030_e20629_d_b2;
        var_nj1_db3 = assign15030_e20629_d_b3;

        let (assign15040_e20641, assign15040_e20641_d_n0, assign15040_e20641_d_n1, assign15040_e20641_d_n2, assign15040_e20641_d_n3, assign15040_e20641_d_n4, assign15040_e20641_d_n5, assign15040_e20641_d_b0, assign15040_e20641_d_b1, assign15040_e20641_d_b2, assign15040_e20641_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard243 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15040_e20641;
        var_dnj1_dv_dn0 = assign15040_e20641_d_n0;
        var_dnj1_dv_dn1 = assign15040_e20641_d_n1;
        var_dnj1_dv_dn2 = assign15040_e20641_d_n2;
        var_dnj1_dv_dn3 = assign15040_e20641_d_n3;
        var_dnj1_dv_dn4 = assign15040_e20641_d_n4;
        var_dnj1_dv_dn5 = assign15040_e20641_d_n5;
        var_dnj1_dv_db0 = assign15040_e20641_d_b0;
        var_dnj1_dv_db1 = assign15040_e20641_d_b1;
        var_dnj1_dv_db2 = assign15040_e20641_d_b2;
        var_dnj1_dv_db3 = assign15040_e20641_d_b3;

        let assign15050_e20645: f64 = (var_vmax / var_nj1);
        let assign15050_e20649: f64 = (var_nj1 - var_nj0);
        let assign15050_e20650: f64 = (var_vha1 * assign15050_e20649);
        let assign15050_e20653: f64 = (var_nj0 * p.p85);
        let assign15050_e20654: f64 = (assign15050_e20650 / assign15050_e20653);
        let assign15050_e20655: f64 = (assign15050_e20645 + assign15050_e20654);
        let assign15050_e20656: f64 = (var_phitdinv * assign15050_e20655);
        let assign15050_e20657: f64 = (assign15050_e20656).abs();
        let assign15050_e20659: f64 = if assign15050_e20657 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard244 = assign15050_e20659;

        let (assign15060_e20685, assign15060_e20685_d_n0, assign15060_e20685_d_n1, assign15060_e20685_d_n2, assign15060_e20685_d_n3, assign15060_e20685_d_n4, assign15060_e20685_d_n5, assign15060_e20685_d_b0, assign15060_e20685_d_b1, assign15060_e20685_d_b2, assign15060_e20685_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 != 0.0)) {
        let assign15060_e20671: f64 = (var_vmax / var_nj1);
        let assign15060_e20675: f64 = (var_nj1 - var_nj0);
        let assign15060_e20676: f64 = (var_vha1 * assign15060_e20675);
        let assign15060_e20679: f64 = (var_nj0 * p.p85);
        let assign15060_e20680: f64 = (assign15060_e20676 / assign15060_e20679);
        let assign15060_e20681: f64 = (assign15060_e20671 + assign15060_e20680);
        let assign15060_e20682: f64 = (var_phitdinv * assign15060_e20681);
        let assign15060_e20683: f64 = (assign15060_e20682).exp();
        (assign15060_e20683, (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn0 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn1 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn2 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn3 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn4 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_dn5 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_db0 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_db1 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_db2 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))), (assign15060_e20683 * (var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15060_e20679) - (assign15060_e20676 * (var_nj0_db3 * p.p85))) / (assign15060_e20679 * assign15060_e20679))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15060_e20685;
        var_exp_vmax_over_phitd_bot_dn0 = assign15060_e20685_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign15060_e20685_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign15060_e20685_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign15060_e20685_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign15060_e20685_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign15060_e20685_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign15060_e20685_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign15060_e20685_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign15060_e20685_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign15060_e20685_d_b3;

        let assign15070_e20689: f64 = (var_vmax / var_nj1);
        let assign15070_e20693: f64 = (var_nj1 - var_nj0);
        let assign15070_e20694: f64 = (var_vha1 * assign15070_e20693);
        let assign15070_e20697: f64 = (var_nj0 * p.p85);
        let assign15070_e20698: f64 = (assign15070_e20694 / assign15070_e20697);
        let assign15070_e20699: f64 = (assign15070_e20689 + assign15070_e20698);
        let assign15070_e20700: f64 = (var_phitdinv * assign15070_e20699);
        let assign15070_e20702: f64 = (-230.25850929940458);
        let assign15070_e20703: f64 = if assign15070_e20700 < assign15070_e20702 { 1.0 } else { 0.0 };
        var_guard245 = assign15070_e20703;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_db0_slot = var_exp_vmax_over_phitd_bot_db0;
        *var_exp_vmax_over_phitd_bot_db1_slot = var_exp_vmax_over_phitd_bot_db1;
        *var_exp_vmax_over_phitd_bot_db2_slot = var_exp_vmax_over_phitd_bot_db2;
        *var_exp_vmax_over_phitd_bot_db3_slot = var_exp_vmax_over_phitd_bot_db3;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn1_slot = var_exp_vmax_over_phitd_bot_dn1;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_dn3_slot = var_exp_vmax_over_phitd_bot_dn3;
        *var_exp_vmax_over_phitd_bot_dn4_slot = var_exp_vmax_over_phitd_bot_dn4;
        *var_exp_vmax_over_phitd_bot_dn5_slot = var_exp_vmax_over_phitd_bot_dn5;
        *var_guard244_slot = var_guard244;
        *var_guard245_slot = var_guard245;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        var_dnj1_dv: f64,
        var_dnj1_dv_db0: f64,
        var_dnj1_dv_db1: f64,
        var_dnj1_dv_db2: f64,
        var_dnj1_dv_db3: f64,
        var_dnj1_dv_dn0: f64,
        var_dnj1_dv_dn1: f64,
        var_dnj1_dv_dn2: f64,
        var_dnj1_dv_dn3: f64,
        var_dnj1_dv_dn4: f64,
        var_dnj1_dv_dn5: f64,
        var_guard230: f64,
        var_guard231: f64,
        var_guard244: f64,
        var_guard245: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v3: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn5_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_db0: f64 = *var_exp_vmax_over_phitd_bot_db0_slot;
        let mut var_exp_vmax_over_phitd_bot_db1: f64 = *var_exp_vmax_over_phitd_bot_db1_slot;
        let mut var_exp_vmax_over_phitd_bot_db2: f64 = *var_exp_vmax_over_phitd_bot_db2_slot;
        let mut var_exp_vmax_over_phitd_bot_db3: f64 = *var_exp_vmax_over_phitd_bot_db3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn1: f64 = *var_exp_vmax_over_phitd_bot_dn1_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_dn3: f64 = *var_exp_vmax_over_phitd_bot_dn3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn4: f64 = *var_exp_vmax_over_phitd_bot_dn4_slot;
        let mut var_exp_vmax_over_phitd_bot_dn5: f64 = *var_exp_vmax_over_phitd_bot_dn5_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign15080_e20784, assign15080_e20784_d_n0, assign15080_e20784_d_n1, assign15080_e20784_d_n2, assign15080_e20784_d_n3, assign15080_e20784_d_n4, assign15080_e20784_d_n5, assign15080_e20784_d_b0, assign15080_e20784_d_b1, assign15080_e20784_d_b2, assign15080_e20784_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 == 0.0)) && (var_guard245 != 0.0)) {
        let assign15080_e20718: f64 = (-230.25850929940458);
        let assign15080_e20722: f64 = (var_vmax / var_nj1);
        let assign15080_e20726: f64 = (var_nj1 - var_nj0);
        let assign15080_e20727: f64 = (var_vha1 * assign15080_e20726);
        let assign15080_e20730: f64 = (var_nj0 * p.p85);
        let assign15080_e20731: f64 = (assign15080_e20727 / assign15080_e20730);
        let assign15080_e20732: f64 = (assign15080_e20722 + assign15080_e20731);
        let assign15080_e20733: f64 = (var_phitdinv * assign15080_e20732);
        let assign15080_e20734: f64 = (assign15080_e20718 - assign15080_e20733);
        let assign15080_e20738: f64 = (-230.25850929940458);
        let assign15080_e20742: f64 = (var_vmax / var_nj1);
        let assign15080_e20746: f64 = (var_nj1 - var_nj0);
        let assign15080_e20747: f64 = (var_vha1 * assign15080_e20746);
        let assign15080_e20750: f64 = (var_nj0 * p.p85);
        let assign15080_e20751: f64 = (assign15080_e20747 / assign15080_e20750);
        let assign15080_e20752: f64 = (assign15080_e20742 + assign15080_e20751);
        let assign15080_e20753: f64 = (var_phitdinv * assign15080_e20752);
        let assign15080_e20754: f64 = (assign15080_e20738 - assign15080_e20753);
        let assign15080_e20757: f64 = (-230.25850929940458);
        let assign15080_e20761: f64 = (var_vmax / var_nj1);
        let assign15080_e20765: f64 = (var_nj1 - var_nj0);
        let assign15080_e20766: f64 = (var_vha1 * assign15080_e20765);
        let assign15080_e20769: f64 = (var_nj0 * p.p85);
        let assign15080_e20770: f64 = (assign15080_e20766 / assign15080_e20769);
        let assign15080_e20771: f64 = (assign15080_e20761 + assign15080_e20770);
        let assign15080_e20772: f64 = (var_phitdinv * assign15080_e20771);
        let assign15080_e20773: f64 = (assign15080_e20757 - assign15080_e20772);
        let assign15080_e20775: f64 = (assign15080_e20773 * 0.3333333333333333);
        let assign15080_e20776: f64 = (1.0 + assign15080_e20775);
        let assign15080_e20777: f64 = (assign15080_e20754 * assign15080_e20776);
        let assign15080_e20778: f64 = (0.5 * assign15080_e20777);
        let assign15080_e20779: f64 = (1.0 + assign15080_e20778);
        let assign15080_e20780: f64 = (assign15080_e20734 * assign15080_e20779);
        let assign15080_e20781: f64 = (1.0 + assign15080_e20780);
        let assign15080_e20782: f64 = (1e-100 / assign15080_e20781);
        (assign15080_e20782, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn0 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn0 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn0 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn1 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn1 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn1 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn2 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn2 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn2 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn3 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn3 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn3 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn4 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn4 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn4 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_dn5 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_dn5 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_dn5 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_db0 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_db0 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_db0 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_db1 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_db1 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_db1 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_db2 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_db2 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_db2 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15080_e20730) - (assign15080_e20727 * (var_nj0_db3 * p.p85))) / (assign15080_e20730 * assign15080_e20730))))) * assign15080_e20779) + (assign15080_e20734 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15080_e20750) - (assign15080_e20747 * (var_nj0_db3 * p.p85))) / (assign15080_e20750 * assign15080_e20750))))) * assign15080_e20776) + (assign15080_e20754 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15080_e20769) - (assign15080_e20766 * (var_nj0_db3 * p.p85))) / (assign15080_e20769 * assign15080_e20769))))) * 0.3333333333333333))))))) / (assign15080_e20781 * assign15080_e20781))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15080_e20784;
        var_exp_vmax_over_phitd_bot_dn0 = assign15080_e20784_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign15080_e20784_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign15080_e20784_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign15080_e20784_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign15080_e20784_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign15080_e20784_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign15080_e20784_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign15080_e20784_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign15080_e20784_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign15080_e20784_d_b3;

        let (assign15090_e20863, assign15090_e20863_d_n0, assign15090_e20863_d_n1, assign15090_e20863_d_n2, assign15090_e20863_d_n3, assign15090_e20863_d_n4, assign15090_e20863_d_n5, assign15090_e20863_d_b0, assign15090_e20863_d_b1, assign15090_e20863_d_b2, assign15090_e20863_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard244 == 0.0)) && (var_guard245 == 0.0)) {
        let assign15090_e20802: f64 = (var_vmax / var_nj1);
        let assign15090_e20806: f64 = (var_nj1 - var_nj0);
        let assign15090_e20807: f64 = (var_vha1 * assign15090_e20806);
        let assign15090_e20810: f64 = (var_nj0 * p.p85);
        let assign15090_e20811: f64 = (assign15090_e20807 / assign15090_e20810);
        let assign15090_e20812: f64 = (assign15090_e20802 + assign15090_e20811);
        let assign15090_e20813: f64 = (var_phitdinv * assign15090_e20812);
        let assign15090_e20815: f64 = (assign15090_e20813 - 230.25850929940458);
        let assign15090_e20821: f64 = (var_vmax / var_nj1);
        let assign15090_e20825: f64 = (var_nj1 - var_nj0);
        let assign15090_e20826: f64 = (var_vha1 * assign15090_e20825);
        let assign15090_e20829: f64 = (var_nj0 * p.p85);
        let assign15090_e20830: f64 = (assign15090_e20826 / assign15090_e20829);
        let assign15090_e20831: f64 = (assign15090_e20821 + assign15090_e20830);
        let assign15090_e20832: f64 = (var_phitdinv * assign15090_e20831);
        let assign15090_e20834: f64 = (assign15090_e20832 - 230.25850929940458);
        let assign15090_e20839: f64 = (var_vmax / var_nj1);
        let assign15090_e20843: f64 = (var_nj1 - var_nj0);
        let assign15090_e20844: f64 = (var_vha1 * assign15090_e20843);
        let assign15090_e20847: f64 = (var_nj0 * p.p85);
        let assign15090_e20848: f64 = (assign15090_e20844 / assign15090_e20847);
        let assign15090_e20849: f64 = (assign15090_e20839 + assign15090_e20848);
        let assign15090_e20850: f64 = (var_phitdinv * assign15090_e20849);
        let assign15090_e20852: f64 = (assign15090_e20850 - 230.25850929940458);
        let assign15090_e20854: f64 = (assign15090_e20852 * 0.3333333333333333);
        let assign15090_e20855: f64 = (1.0 + assign15090_e20854);
        let assign15090_e20856: f64 = (assign15090_e20834 * assign15090_e20855);
        let assign15090_e20857: f64 = (0.5 * assign15090_e20856);
        let assign15090_e20858: f64 = (1.0 + assign15090_e20857);
        let assign15090_e20859: f64 = (assign15090_e20815 * assign15090_e20858);
        let assign15090_e20860: f64 = (1.0 + assign15090_e20859);
        let assign15090_e20861: f64 = (1e100 * assign15090_e20860);
        (assign15090_e20861, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn0 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn0 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn0 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn1 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn1 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn1 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn2 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn2 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn2 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn3 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn3 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn3 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn4 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn4 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn4 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_dn5 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_dn5 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_dn5 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_db0 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_db0 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_db0 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_db1 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_db1 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_db1 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_db2 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_db2 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_db2 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15090_e20810) - (assign15090_e20807 * (var_nj0_db3 * p.p85))) / (assign15090_e20810 * assign15090_e20810)))) * assign15090_e20858) + (assign15090_e20815 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15090_e20829) - (assign15090_e20826 * (var_nj0_db3 * p.p85))) / (assign15090_e20829 * assign15090_e20829)))) * assign15090_e20855) + (assign15090_e20834 * ((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign15090_e20847) - (assign15090_e20844 * (var_nj0_db3 * p.p85))) / (assign15090_e20847 * assign15090_e20847)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign15090_e20863;
        var_exp_vmax_over_phitd_bot_dn0 = assign15090_e20863_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign15090_e20863_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign15090_e20863_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign15090_e20863_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign15090_e20863_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign15090_e20863_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign15090_e20863_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign15090_e20863_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign15090_e20863_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign15090_e20863_d_b3;

        let (assign15100_e20890, assign15100_e20890_d_n0, assign15100_e20890_d_n1, assign15100_e20890_d_n2, assign15100_e20890_d_n3, assign15100_e20890_d_n4, assign15100_e20890_d_n5, assign15100_e20890_d_b0, assign15100_e20890_d_b1, assign15100_e20890_d_b2, assign15100_e20890_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15100_e20874: f64 = (var_vmax * var_dnj1_dv);
        let assign15100_e20875: f64 = (var_nj1 - assign15100_e20874);
        let assign15100_e20878: f64 = (var_nj1 * var_nj1);
        let assign15100_e20879: f64 = (assign15100_e20875 / assign15100_e20878);
        let assign15100_e20882: f64 = (var_vha1 * var_dnj1_dv);
        let assign15100_e20885: f64 = (var_nj0 * p.p85);
        let assign15100_e20886: f64 = (assign15100_e20882 / assign15100_e20885);
        let assign15100_e20887: f64 = (assign15100_e20879 + assign15100_e20886);
        let assign15100_e20888: f64 = (var_phitdinv * assign15100_e20887);
        (assign15100_e20888, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn0 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn1 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn2 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn3 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn4 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_dn5 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_db0) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_db0 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_db1) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_db1 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_db2) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_db2 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign15100_e20878) - (assign15100_e20875 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign15100_e20878 * assign15100_e20878)) + ((((var_vha1 * var_dnj1_dv_db3) * assign15100_e20885) - (assign15100_e20882 * (var_nj0_db3 * p.p85))) / (assign15100_e20885 * assign15100_e20885)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign15100_e20890;
        var_dvmax_over_phitd_dv_dn0 = assign15100_e20890_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign15100_e20890_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign15100_e20890_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign15100_e20890_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign15100_e20890_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign15100_e20890_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign15100_e20890_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign15100_e20890_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign15100_e20890_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign15100_e20890_d_b3;

        let (assign15110_e20907, assign15110_e20907_d_n0, assign15110_e20907_d_n1, assign15110_e20907_d_n2, assign15110_e20907_d_n3, assign15110_e20907_d_n4, assign15110_e20907_d_n5, assign15110_e20907_d_b0, assign15110_e20907_d_b1, assign15110_e20907_d_b2, assign15110_e20907_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15110_e20900: f64 = (var_v3 - var_vmax);
        let assign15110_e20902: f64 = (assign15110_e20900 * var_dvmax_over_phitd_dv);
        let assign15110_e20903: f64 = (1.0 + assign15110_e20902);
        let assign15110_e20905: f64 = (assign15110_e20903 * var_exp_vmax_over_phitd_bot);
        (assign15110_e20905, (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn0)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn1) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn1)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn2)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn3) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn3)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn4) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn4)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_dn5) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_dn5)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_db0) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_db0)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_db1) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_db1)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_db2) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_db2)), (((assign15110_e20900 * var_dvmax_over_phitd_dv_db3) * var_exp_vmax_over_phitd_bot) + (assign15110_e20903 * var_exp_vmax_over_phitd_bot_db3)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign15110_e20907;
        var_idmultbot_dn0 = assign15110_e20907_d_n0;
        var_idmultbot_dn1 = assign15110_e20907_d_n1;
        var_idmultbot_dn2 = assign15110_e20907_d_n2;
        var_idmultbot_dn3 = assign15110_e20907_d_n3;
        var_idmultbot_dn4 = assign15110_e20907_d_n4;
        var_idmultbot_dn5 = assign15110_e20907_d_n5;
        var_idmultbot_db0 = assign15110_e20907_d_b0;
        var_idmultbot_db1 = assign15110_e20907_d_b1;
        var_idmultbot_db2 = assign15110_e20907_d_b2;
        var_idmultbot_db3 = assign15110_e20907_d_b3;

        let (assign15120_e20920,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15120_e20916: f64 = (var_nin * var_nin);
        let assign15120_e20918: f64 = (assign15120_e20916 / var_ndisti_i);
        (assign15120_e20918,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign15120_e20920;

        let (assign15130_e20936,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15130_e20929: f64 = (var_nfasti_i / var_phitdinv);
        let assign15130_e20932: f64 = (var_ndisti_i / var_pnn0);
        let assign15130_e20933: f64 = (assign15130_e20932).ln();
        let assign15130_e20934: f64 = (assign15130_e20929 * assign15130_e20933);
        (assign15130_e20934,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign15130_e20936;

        let assign15140_e20939: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard246 = assign15140_e20939;

        let (assign15150_e20956, assign15150_e20956_d_n0, assign15150_e20956_d_n1, assign15150_e20956_d_n2, assign15150_e20956_d_n3, assign15150_e20956_d_n4, assign15150_e20956_d_n5, assign15150_e20956_d_b0, assign15150_e20956_d_b1, assign15150_e20956_d_b2, assign15150_e20956_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15150_e20951: f64 = (var_vmax - var_vha1);
        let assign15150_e20952: f64 = (p.p86 * assign15150_e20951);
        let assign15150_e20954: f64 = (assign15150_e20952 + var_nfasti_i);
        (assign15150_e20954, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign15150_e20956;
        var_nja10_dn0 = assign15150_e20956_d_n0;
        var_nja10_dn1 = assign15150_e20956_d_n1;
        var_nja10_dn2 = assign15150_e20956_d_n2;
        var_nja10_dn3 = assign15150_e20956_d_n3;
        var_nja10_dn4 = assign15150_e20956_d_n4;
        var_nja10_dn5 = assign15150_e20956_d_n5;
        var_nja10_db0 = assign15150_e20956_d_b0;
        var_nja10_db1 = assign15150_e20956_d_b1;
        var_nja10_db2 = assign15150_e20956_d_b2;
        var_nja10_db3 = assign15150_e20956_d_b3;

        let (assign15160_e20971, assign15160_e20971_d_n0, assign15160_e20971_d_n1, assign15160_e20971_d_n2, assign15160_e20971_d_n3, assign15160_e20971_d_n4, assign15160_e20971_d_n5, assign15160_e20971_d_b0, assign15160_e20971_d_b1, assign15160_e20971_d_b2, assign15160_e20971_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15160_e20968: f64 = (p.p86 * var_vha1);
        let assign15160_e20969: f64 = (var_nfasti_i - assign15160_e20968);
        (assign15160_e20969, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15160_e20971;
        var_nj0_dn0 = assign15160_e20971_d_n0;
        var_nj0_dn1 = assign15160_e20971_d_n1;
        var_nj0_dn2 = assign15160_e20971_d_n2;
        var_nj0_dn3 = assign15160_e20971_d_n3;
        var_nj0_dn4 = assign15160_e20971_d_n4;
        var_nj0_dn5 = assign15160_e20971_d_n5;
        var_nj0_db0 = assign15160_e20971_d_b0;
        var_nj0_db1 = assign15160_e20971_d_b1;
        var_nj0_db2 = assign15160_e20971_d_b2;
        var_nj0_db3 = assign15160_e20971_d_b3;

        let (assign15170_e20986, assign15170_e20986_d_n0, assign15170_e20986_d_n1, assign15170_e20986_d_n2, assign15170_e20986_d_n3, assign15170_e20986_d_n4, assign15170_e20986_d_n5, assign15170_e20986_d_b0, assign15170_e20986_d_b1, assign15170_e20986_d_b2, assign15170_e20986_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15170_e20982: f64 = (p.p85 - var_nja10);
        let assign15170_e20984: f64 = (assign15170_e20982 - 0.01);
        (assign15170_e20984, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15170_e20986;
        var_tmf1_dn0 = assign15170_e20986_d_n0;
        var_tmf1_dn1 = assign15170_e20986_d_n1;
        var_tmf1_dn2 = assign15170_e20986_d_n2;
        var_tmf1_dn3 = assign15170_e20986_d_n3;
        var_tmf1_dn4 = assign15170_e20986_d_n4;
        var_tmf1_dn5 = assign15170_e20986_d_n5;
        var_tmf1_db0 = assign15170_e20986_d_b0;
        var_tmf1_db1 = assign15170_e20986_d_b1;
        var_tmf1_db2 = assign15170_e20986_d_b2;
        var_tmf1_db3 = assign15170_e20986_d_b3;

        let (assign15180_e21001, assign15180_e21001_d_n0, assign15180_e21001_d_n1, assign15180_e21001_d_n2, assign15180_e21001_d_n3, assign15180_e21001_d_n4, assign15180_e21001_d_n5, assign15180_e21001_d_b0, assign15180_e21001_d_b1, assign15180_e21001_d_b2, assign15180_e21001_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15180_e20997: f64 = (4.0 * p.p85);
        let assign15180_e20999: f64 = (assign15180_e20997 * 0.01);
        (assign15180_e20999, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15180_e21001;
        var_tmf2_dn0 = assign15180_e21001_d_n0;
        var_tmf2_dn1 = assign15180_e21001_d_n1;
        var_tmf2_dn2 = assign15180_e21001_d_n2;
        var_tmf2_dn3 = assign15180_e21001_d_n3;
        var_tmf2_dn4 = assign15180_e21001_d_n4;
        var_tmf2_dn5 = assign15180_e21001_d_n5;
        var_tmf2_db0 = assign15180_e21001_d_b0;
        var_tmf2_db1 = assign15180_e21001_d_b1;
        var_tmf2_db2 = assign15180_e21001_d_b2;
        var_tmf2_db3 = assign15180_e21001_d_b3;

        let (assign15190_e21018, assign15190_e21018_d_n0, assign15190_e21018_d_n1, assign15190_e21018_d_n2, assign15190_e21018_d_n3, assign15190_e21018_d_n4, assign15190_e21018_d_n5, assign15190_e21018_d_b0, assign15190_e21018_d_b1, assign15190_e21018_d_b2, assign15190_e21018_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n1, assign15190_e21016_d_n2, assign15190_e21016_d_n3, assign15190_e21016_d_n4, assign15190_e21016_d_n5, assign15190_e21016_d_b0, assign15190_e21016_d_b1, assign15190_e21016_d_b2, assign15190_e21016_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15190_e21015: f64 = (-var_tmf2);
                (assign15190_e21015, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15190_e21016, assign15190_e21016_d_n0, assign15190_e21016_d_n1, assign15190_e21016_d_n2, assign15190_e21016_d_n3, assign15190_e21016_d_n4, assign15190_e21016_d_n5, assign15190_e21016_d_b0, assign15190_e21016_d_b1, assign15190_e21016_d_b2, assign15190_e21016_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15190_e21018;
        var_tmf2_dn0 = assign15190_e21018_d_n0;
        var_tmf2_dn1 = assign15190_e21018_d_n1;
        var_tmf2_dn2 = assign15190_e21018_d_n2;
        var_tmf2_dn3 = assign15190_e21018_d_n3;
        var_tmf2_dn4 = assign15190_e21018_d_n4;
        var_tmf2_dn5 = assign15190_e21018_d_n5;
        var_tmf2_db0 = assign15190_e21018_d_b0;
        var_tmf2_db1 = assign15190_e21018_d_b1;
        var_tmf2_db2 = assign15190_e21018_d_b2;
        var_tmf2_db3 = assign15190_e21018_d_b3;

        let (assign15200_e21034, assign15200_e21034_d_n0, assign15200_e21034_d_n1, assign15200_e21034_d_n2, assign15200_e21034_d_n3, assign15200_e21034_d_n4, assign15200_e21034_d_n5, assign15200_e21034_d_b0, assign15200_e21034_d_b1, assign15200_e21034_d_b2, assign15200_e21034_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15200_e21029: f64 = (var_tmf1 * var_tmf1);
        let assign15200_e21031: f64 = (assign15200_e21029 + var_tmf2);
        let assign15200_e21032: f64 = (assign15200_e21031).sqrt();
        (assign15200_e21032, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15200_e21032)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15200_e21032)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15200_e21032)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15200_e21032)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15200_e21032)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15200_e21032)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15200_e21034;
        var_tmf2_dn0 = assign15200_e21034_d_n0;
        var_tmf2_dn1 = assign15200_e21034_d_n1;
        var_tmf2_dn2 = assign15200_e21034_d_n2;
        var_tmf2_dn3 = assign15200_e21034_d_n3;
        var_tmf2_dn4 = assign15200_e21034_d_n4;
        var_tmf2_dn5 = assign15200_e21034_d_n5;
        var_tmf2_db0 = assign15200_e21034_d_b0;
        var_tmf2_db1 = assign15200_e21034_d_b1;
        var_tmf2_db2 = assign15200_e21034_d_b2;
        var_tmf2_db3 = assign15200_e21034_d_b3;

        let (assign15210_e21051, assign15210_e21051_d_n0, assign15210_e21051_d_n1, assign15210_e21051_d_n2, assign15210_e21051_d_n3, assign15210_e21051_d_n4, assign15210_e21051_d_n5, assign15210_e21051_d_b0, assign15210_e21051_d_b1, assign15210_e21051_d_b2, assign15210_e21051_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15210_e21047: f64 = (var_tmf1 / var_tmf2);
        let assign15210_e21048: f64 = (1.0 + assign15210_e21047);
        let assign15210_e21049: f64 = (0.5 * assign15210_e21048);
        (assign15210_e21049, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign15210_e21051;
        var_dfn_su_dn0 = assign15210_e21051_d_n0;
        var_dfn_su_dn1 = assign15210_e21051_d_n1;
        var_dfn_su_dn2 = assign15210_e21051_d_n2;
        var_dfn_su_dn3 = assign15210_e21051_d_n3;
        var_dfn_su_dn4 = assign15210_e21051_d_n4;
        var_dfn_su_dn5 = assign15210_e21051_d_n5;
        var_dfn_su_db0 = assign15210_e21051_d_b0;
        var_dfn_su_db1 = assign15210_e21051_d_b1;
        var_dfn_su_db2 = assign15210_e21051_d_b2;
        var_dfn_su_db3 = assign15210_e21051_d_b3;

        let (assign15220_e21068, assign15220_e21068_d_n0, assign15220_e21068_d_n1, assign15220_e21068_d_n2, assign15220_e21068_d_n3, assign15220_e21068_d_n4, assign15220_e21068_d_n5, assign15220_e21068_d_b0, assign15220_e21068_d_b1, assign15220_e21068_d_b2, assign15220_e21068_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15220_e21064: f64 = (var_tmf1 + var_tmf2);
        let assign15220_e21065: f64 = (0.5 * assign15220_e21064);
        let assign15220_e21066: f64 = (p.p85 - assign15220_e21065);
        (assign15220_e21066, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign15220_e21068;
        var_nja11_dn0 = assign15220_e21068_d_n0;
        var_nja11_dn1 = assign15220_e21068_d_n1;
        var_nja11_dn2 = assign15220_e21068_d_n2;
        var_nja11_dn3 = assign15220_e21068_d_n3;
        var_nja11_dn4 = assign15220_e21068_d_n4;
        var_nja11_dn5 = assign15220_e21068_d_n5;
        var_nja11_db0 = assign15220_e21068_d_b0;
        var_nja11_db1 = assign15220_e21068_d_b1;
        var_nja11_db2 = assign15220_e21068_d_b2;
        var_nja11_db3 = assign15220_e21068_d_b3;

        let (assign15230_e21083, assign15230_e21083_d_n0, assign15230_e21083_d_n1, assign15230_e21083_d_n2, assign15230_e21083_d_n3, assign15230_e21083_d_n4, assign15230_e21083_d_n5, assign15230_e21083_d_b0, assign15230_e21083_d_b1, assign15230_e21083_d_b2, assign15230_e21083_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15230_e21079: f64 = (var_nja11 - var_nfasti_i);
        let assign15230_e21081: f64 = (assign15230_e21079 - 0.01);
        (assign15230_e21081, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15230_e21083;
        var_tmf1_dn0 = assign15230_e21083_d_n0;
        var_tmf1_dn1 = assign15230_e21083_d_n1;
        var_tmf1_dn2 = assign15230_e21083_d_n2;
        var_tmf1_dn3 = assign15230_e21083_d_n3;
        var_tmf1_dn4 = assign15230_e21083_d_n4;
        var_tmf1_dn5 = assign15230_e21083_d_n5;
        var_tmf1_db0 = assign15230_e21083_d_b0;
        var_tmf1_db1 = assign15230_e21083_d_b1;
        var_tmf1_db2 = assign15230_e21083_d_b2;
        var_tmf1_db3 = assign15230_e21083_d_b3;

        let (assign15240_e21098, assign15240_e21098_d_n0, assign15240_e21098_d_n1, assign15240_e21098_d_n2, assign15240_e21098_d_n3, assign15240_e21098_d_n4, assign15240_e21098_d_n5, assign15240_e21098_d_b0, assign15240_e21098_d_b1, assign15240_e21098_d_b2, assign15240_e21098_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15240_e21094: f64 = (4.0 * var_nfasti_i);
        let assign15240_e21096: f64 = (assign15240_e21094 * 0.01);
        (assign15240_e21096, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15240_e21098;
        var_tmf2_dn0 = assign15240_e21098_d_n0;
        var_tmf2_dn1 = assign15240_e21098_d_n1;
        var_tmf2_dn2 = assign15240_e21098_d_n2;
        var_tmf2_dn3 = assign15240_e21098_d_n3;
        var_tmf2_dn4 = assign15240_e21098_d_n4;
        var_tmf2_dn5 = assign15240_e21098_d_n5;
        var_tmf2_db0 = assign15240_e21098_d_b0;
        var_tmf2_db1 = assign15240_e21098_d_b1;
        var_tmf2_db2 = assign15240_e21098_d_b2;
        var_tmf2_db3 = assign15240_e21098_d_b3;

        let (assign15250_e21115, assign15250_e21115_d_n0, assign15250_e21115_d_n1, assign15250_e21115_d_n2, assign15250_e21115_d_n3, assign15250_e21115_d_n4, assign15250_e21115_d_n5, assign15250_e21115_d_b0, assign15250_e21115_d_b1, assign15250_e21115_d_b2, assign15250_e21115_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n1, assign15250_e21113_d_n2, assign15250_e21113_d_n3, assign15250_e21113_d_n4, assign15250_e21113_d_n5, assign15250_e21113_d_b0, assign15250_e21113_d_b1, assign15250_e21113_d_b2, assign15250_e21113_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15250_e21112: f64 = (-var_tmf2);
                (assign15250_e21112, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15250_e21113, assign15250_e21113_d_n0, assign15250_e21113_d_n1, assign15250_e21113_d_n2, assign15250_e21113_d_n3, assign15250_e21113_d_n4, assign15250_e21113_d_n5, assign15250_e21113_d_b0, assign15250_e21113_d_b1, assign15250_e21113_d_b2, assign15250_e21113_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15250_e21115;
        var_tmf2_dn0 = assign15250_e21115_d_n0;
        var_tmf2_dn1 = assign15250_e21115_d_n1;
        var_tmf2_dn2 = assign15250_e21115_d_n2;
        var_tmf2_dn3 = assign15250_e21115_d_n3;
        var_tmf2_dn4 = assign15250_e21115_d_n4;
        var_tmf2_dn5 = assign15250_e21115_d_n5;
        var_tmf2_db0 = assign15250_e21115_d_b0;
        var_tmf2_db1 = assign15250_e21115_d_b1;
        var_tmf2_db2 = assign15250_e21115_d_b2;
        var_tmf2_db3 = assign15250_e21115_d_b3;

        let (assign15260_e21131, assign15260_e21131_d_n0, assign15260_e21131_d_n1, assign15260_e21131_d_n2, assign15260_e21131_d_n3, assign15260_e21131_d_n4, assign15260_e21131_d_n5, assign15260_e21131_d_b0, assign15260_e21131_d_b1, assign15260_e21131_d_b2, assign15260_e21131_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15260_e21126: f64 = (var_tmf1 * var_tmf1);
        let assign15260_e21128: f64 = (assign15260_e21126 + var_tmf2);
        let assign15260_e21129: f64 = (assign15260_e21128).sqrt();
        (assign15260_e21129, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15260_e21129)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15260_e21129)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15260_e21129)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15260_e21129)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15260_e21129)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15260_e21129)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15260_e21131;
        var_tmf2_dn0 = assign15260_e21131_d_n0;
        var_tmf2_dn1 = assign15260_e21131_d_n1;
        var_tmf2_dn2 = assign15260_e21131_d_n2;
        var_tmf2_dn3 = assign15260_e21131_d_n3;
        var_tmf2_dn4 = assign15260_e21131_d_n4;
        var_tmf2_dn5 = assign15260_e21131_d_n5;
        var_tmf2_db0 = assign15260_e21131_d_b0;
        var_tmf2_db1 = assign15260_e21131_d_b1;
        var_tmf2_db2 = assign15260_e21131_d_b2;
        var_tmf2_db3 = assign15260_e21131_d_b3;

        let (assign15270_e21148, assign15270_e21148_d_n0, assign15270_e21148_d_n1, assign15270_e21148_d_n2, assign15270_e21148_d_n3, assign15270_e21148_d_n4, assign15270_e21148_d_n5, assign15270_e21148_d_b0, assign15270_e21148_d_b1, assign15270_e21148_d_b2, assign15270_e21148_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15270_e21144: f64 = (var_tmf1 / var_tmf2);
        let assign15270_e21145: f64 = (1.0 + assign15270_e21144);
        let assign15270_e21146: f64 = (0.5 * assign15270_e21145);
        (assign15270_e21146, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign15270_e21148;
        var_dfn_sl_dn0 = assign15270_e21148_d_n0;
        var_dfn_sl_dn1 = assign15270_e21148_d_n1;
        var_dfn_sl_dn2 = assign15270_e21148_d_n2;
        var_dfn_sl_dn3 = assign15270_e21148_d_n3;
        var_dfn_sl_dn4 = assign15270_e21148_d_n4;
        var_dfn_sl_dn5 = assign15270_e21148_d_n5;
        var_dfn_sl_db0 = assign15270_e21148_d_b0;
        var_dfn_sl_db1 = assign15270_e21148_d_b1;
        var_dfn_sl_db2 = assign15270_e21148_d_b2;
        var_dfn_sl_db3 = assign15270_e21148_d_b3;

        let (assign15280_e21165, assign15280_e21165_d_n0, assign15280_e21165_d_n1, assign15280_e21165_d_n2, assign15280_e21165_d_n3, assign15280_e21165_d_n4, assign15280_e21165_d_n5, assign15280_e21165_d_b0, assign15280_e21165_d_b1, assign15280_e21165_d_b2, assign15280_e21165_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15280_e21161: f64 = (var_tmf1 + var_tmf2);
        let assign15280_e21162: f64 = (0.5 * assign15280_e21161);
        let assign15280_e21163: f64 = (var_nfasti_i + assign15280_e21162);
        (assign15280_e21163, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign15280_e21165;
        var_nj1_dn0 = assign15280_e21165_d_n0;
        var_nj1_dn1 = assign15280_e21165_d_n1;
        var_nj1_dn2 = assign15280_e21165_d_n2;
        var_nj1_dn3 = assign15280_e21165_d_n3;
        var_nj1_dn4 = assign15280_e21165_d_n4;
        var_nj1_dn5 = assign15280_e21165_d_n5;
        var_nj1_db0 = assign15280_e21165_d_b0;
        var_nj1_db1 = assign15280_e21165_d_b1;
        var_nj1_db2 = assign15280_e21165_d_b2;
        var_nj1_db3 = assign15280_e21165_d_b3;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_db0_slot = var_exp_vmax_over_phitd_bot_db0;
        *var_exp_vmax_over_phitd_bot_db1_slot = var_exp_vmax_over_phitd_bot_db1;
        *var_exp_vmax_over_phitd_bot_db2_slot = var_exp_vmax_over_phitd_bot_db2;
        *var_exp_vmax_over_phitd_bot_db3_slot = var_exp_vmax_over_phitd_bot_db3;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn1_slot = var_exp_vmax_over_phitd_bot_dn1;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_dn3_slot = var_exp_vmax_over_phitd_bot_dn3;
        *var_exp_vmax_over_phitd_bot_dn4_slot = var_exp_vmax_over_phitd_bot_dn4;
        *var_exp_vmax_over_phitd_bot_dn5_slot = var_exp_vmax_over_phitd_bot_dn5;
        *var_guard246_slot = var_guard246;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_db0: f64,
        var_dfn_sl_db1: f64,
        var_dfn_sl_db2: f64,
        var_dfn_sl_db3: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn1: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_sl_dn3: f64,
        var_dfn_sl_dn4: f64,
        var_dfn_sl_dn5: f64,
        var_dfn_su: f64,
        var_dfn_su_db0: f64,
        var_dfn_su_db1: f64,
        var_dfn_su_db2: f64,
        var_dfn_su_db3: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn1: f64,
        var_dfn_su_dn2: f64,
        var_dfn_su_dn3: f64,
        var_dfn_su_dn4: f64,
        var_dfn_su_dn5: f64,
        var_guard230: f64,
        var_guard231: f64,
        var_guard246: f64,
        var_guard31: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign15290_e21180, assign15290_e21180_d_n0, assign15290_e21180_d_n1, assign15290_e21180_d_n2, assign15290_e21180_d_n3, assign15290_e21180_d_n4, assign15290_e21180_d_n5, assign15290_e21180_d_b0, assign15290_e21180_d_b1, assign15290_e21180_d_b2, assign15290_e21180_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15290_e21176: f64 = (p.p85 - var_nj0);
        let assign15290_e21178: f64 = (assign15290_e21176 - 0.01);
        (assign15290_e21178, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15290_e21180;
        var_tmf1_dn0 = assign15290_e21180_d_n0;
        var_tmf1_dn1 = assign15290_e21180_d_n1;
        var_tmf1_dn2 = assign15290_e21180_d_n2;
        var_tmf1_dn3 = assign15290_e21180_d_n3;
        var_tmf1_dn4 = assign15290_e21180_d_n4;
        var_tmf1_dn5 = assign15290_e21180_d_n5;
        var_tmf1_db0 = assign15290_e21180_d_b0;
        var_tmf1_db1 = assign15290_e21180_d_b1;
        var_tmf1_db2 = assign15290_e21180_d_b2;
        var_tmf1_db3 = assign15290_e21180_d_b3;

        let (assign15300_e21195, assign15300_e21195_d_n0, assign15300_e21195_d_n1, assign15300_e21195_d_n2, assign15300_e21195_d_n3, assign15300_e21195_d_n4, assign15300_e21195_d_n5, assign15300_e21195_d_b0, assign15300_e21195_d_b1, assign15300_e21195_d_b2, assign15300_e21195_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15300_e21191: f64 = (4.0 * p.p85);
        let assign15300_e21193: f64 = (assign15300_e21191 * 0.01);
        (assign15300_e21193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15300_e21195;
        var_tmf2_dn0 = assign15300_e21195_d_n0;
        var_tmf2_dn1 = assign15300_e21195_d_n1;
        var_tmf2_dn2 = assign15300_e21195_d_n2;
        var_tmf2_dn3 = assign15300_e21195_d_n3;
        var_tmf2_dn4 = assign15300_e21195_d_n4;
        var_tmf2_dn5 = assign15300_e21195_d_n5;
        var_tmf2_db0 = assign15300_e21195_d_b0;
        var_tmf2_db1 = assign15300_e21195_d_b1;
        var_tmf2_db2 = assign15300_e21195_d_b2;
        var_tmf2_db3 = assign15300_e21195_d_b3;

        let (assign15310_e21212, assign15310_e21212_d_n0, assign15310_e21212_d_n1, assign15310_e21212_d_n2, assign15310_e21212_d_n3, assign15310_e21212_d_n4, assign15310_e21212_d_n5, assign15310_e21212_d_b0, assign15310_e21212_d_b1, assign15310_e21212_d_b2, assign15310_e21212_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n1, assign15310_e21210_d_n2, assign15310_e21210_d_n3, assign15310_e21210_d_n4, assign15310_e21210_d_n5, assign15310_e21210_d_b0, assign15310_e21210_d_b1, assign15310_e21210_d_b2, assign15310_e21210_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15310_e21209: f64 = (-var_tmf2);
                (assign15310_e21209, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15310_e21210, assign15310_e21210_d_n0, assign15310_e21210_d_n1, assign15310_e21210_d_n2, assign15310_e21210_d_n3, assign15310_e21210_d_n4, assign15310_e21210_d_n5, assign15310_e21210_d_b0, assign15310_e21210_d_b1, assign15310_e21210_d_b2, assign15310_e21210_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15310_e21212;
        var_tmf2_dn0 = assign15310_e21212_d_n0;
        var_tmf2_dn1 = assign15310_e21212_d_n1;
        var_tmf2_dn2 = assign15310_e21212_d_n2;
        var_tmf2_dn3 = assign15310_e21212_d_n3;
        var_tmf2_dn4 = assign15310_e21212_d_n4;
        var_tmf2_dn5 = assign15310_e21212_d_n5;
        var_tmf2_db0 = assign15310_e21212_d_b0;
        var_tmf2_db1 = assign15310_e21212_d_b1;
        var_tmf2_db2 = assign15310_e21212_d_b2;
        var_tmf2_db3 = assign15310_e21212_d_b3;

        let (assign15320_e21228, assign15320_e21228_d_n0, assign15320_e21228_d_n1, assign15320_e21228_d_n2, assign15320_e21228_d_n3, assign15320_e21228_d_n4, assign15320_e21228_d_n5, assign15320_e21228_d_b0, assign15320_e21228_d_b1, assign15320_e21228_d_b2, assign15320_e21228_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15320_e21223: f64 = (var_tmf1 * var_tmf1);
        let assign15320_e21225: f64 = (assign15320_e21223 + var_tmf2);
        let assign15320_e21226: f64 = (assign15320_e21225).sqrt();
        (assign15320_e21226, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15320_e21226)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15320_e21226)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15320_e21226)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15320_e21226)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15320_e21226)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15320_e21226)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15320_e21228;
        var_tmf2_dn0 = assign15320_e21228_d_n0;
        var_tmf2_dn1 = assign15320_e21228_d_n1;
        var_tmf2_dn2 = assign15320_e21228_d_n2;
        var_tmf2_dn3 = assign15320_e21228_d_n3;
        var_tmf2_dn4 = assign15320_e21228_d_n4;
        var_tmf2_dn5 = assign15320_e21228_d_n5;
        var_tmf2_db0 = assign15320_e21228_d_b0;
        var_tmf2_db1 = assign15320_e21228_d_b1;
        var_tmf2_db2 = assign15320_e21228_d_b2;
        var_tmf2_db3 = assign15320_e21228_d_b3;

        let (assign15330_e21245, assign15330_e21245_d_n0, assign15330_e21245_d_n1, assign15330_e21245_d_n2, assign15330_e21245_d_n3, assign15330_e21245_d_n4, assign15330_e21245_d_n5, assign15330_e21245_d_b0, assign15330_e21245_d_b1, assign15330_e21245_d_b2, assign15330_e21245_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15330_e21241: f64 = (var_tmf1 + var_tmf2);
        let assign15330_e21242: f64 = (0.5 * assign15330_e21241);
        let assign15330_e21243: f64 = (p.p85 - assign15330_e21242);
        (assign15330_e21243, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15330_e21245;
        var_nj0_dn0 = assign15330_e21245_d_n0;
        var_nj0_dn1 = assign15330_e21245_d_n1;
        var_nj0_dn2 = assign15330_e21245_d_n2;
        var_nj0_dn3 = assign15330_e21245_d_n3;
        var_nj0_dn4 = assign15330_e21245_d_n4;
        var_nj0_dn5 = assign15330_e21245_d_n5;
        var_nj0_db0 = assign15330_e21245_d_b0;
        var_nj0_db1 = assign15330_e21245_d_b1;
        var_nj0_db2 = assign15330_e21245_d_b2;
        var_nj0_db3 = assign15330_e21245_d_b3;

        let (assign15340_e21260, assign15340_e21260_d_n0, assign15340_e21260_d_n1, assign15340_e21260_d_n2, assign15340_e21260_d_n3, assign15340_e21260_d_n4, assign15340_e21260_d_n5, assign15340_e21260_d_b0, assign15340_e21260_d_b1, assign15340_e21260_d_b2, assign15340_e21260_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15340_e21256: f64 = (var_nj0 - var_nfasti_i);
        let assign15340_e21258: f64 = (assign15340_e21256 - 0.01);
        (assign15340_e21258, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15340_e21260;
        var_tmf1_dn0 = assign15340_e21260_d_n0;
        var_tmf1_dn1 = assign15340_e21260_d_n1;
        var_tmf1_dn2 = assign15340_e21260_d_n2;
        var_tmf1_dn3 = assign15340_e21260_d_n3;
        var_tmf1_dn4 = assign15340_e21260_d_n4;
        var_tmf1_dn5 = assign15340_e21260_d_n5;
        var_tmf1_db0 = assign15340_e21260_d_b0;
        var_tmf1_db1 = assign15340_e21260_d_b1;
        var_tmf1_db2 = assign15340_e21260_d_b2;
        var_tmf1_db3 = assign15340_e21260_d_b3;

        let (assign15350_e21275, assign15350_e21275_d_n0, assign15350_e21275_d_n1, assign15350_e21275_d_n2, assign15350_e21275_d_n3, assign15350_e21275_d_n4, assign15350_e21275_d_n5, assign15350_e21275_d_b0, assign15350_e21275_d_b1, assign15350_e21275_d_b2, assign15350_e21275_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15350_e21271: f64 = (4.0 * var_nfasti_i);
        let assign15350_e21273: f64 = (assign15350_e21271 * 0.01);
        (assign15350_e21273, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15350_e21275;
        var_tmf2_dn0 = assign15350_e21275_d_n0;
        var_tmf2_dn1 = assign15350_e21275_d_n1;
        var_tmf2_dn2 = assign15350_e21275_d_n2;
        var_tmf2_dn3 = assign15350_e21275_d_n3;
        var_tmf2_dn4 = assign15350_e21275_d_n4;
        var_tmf2_dn5 = assign15350_e21275_d_n5;
        var_tmf2_db0 = assign15350_e21275_d_b0;
        var_tmf2_db1 = assign15350_e21275_d_b1;
        var_tmf2_db2 = assign15350_e21275_d_b2;
        var_tmf2_db3 = assign15350_e21275_d_b3;

        let (assign15360_e21292, assign15360_e21292_d_n0, assign15360_e21292_d_n1, assign15360_e21292_d_n2, assign15360_e21292_d_n3, assign15360_e21292_d_n4, assign15360_e21292_d_n5, assign15360_e21292_d_b0, assign15360_e21292_d_b1, assign15360_e21292_d_b2, assign15360_e21292_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n1, assign15360_e21290_d_n2, assign15360_e21290_d_n3, assign15360_e21290_d_n4, assign15360_e21290_d_n5, assign15360_e21290_d_b0, assign15360_e21290_d_b1, assign15360_e21290_d_b2, assign15360_e21290_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15360_e21289: f64 = (-var_tmf2);
                (assign15360_e21289, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15360_e21290, assign15360_e21290_d_n0, assign15360_e21290_d_n1, assign15360_e21290_d_n2, assign15360_e21290_d_n3, assign15360_e21290_d_n4, assign15360_e21290_d_n5, assign15360_e21290_d_b0, assign15360_e21290_d_b1, assign15360_e21290_d_b2, assign15360_e21290_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15360_e21292;
        var_tmf2_dn0 = assign15360_e21292_d_n0;
        var_tmf2_dn1 = assign15360_e21292_d_n1;
        var_tmf2_dn2 = assign15360_e21292_d_n2;
        var_tmf2_dn3 = assign15360_e21292_d_n3;
        var_tmf2_dn4 = assign15360_e21292_d_n4;
        var_tmf2_dn5 = assign15360_e21292_d_n5;
        var_tmf2_db0 = assign15360_e21292_d_b0;
        var_tmf2_db1 = assign15360_e21292_d_b1;
        var_tmf2_db2 = assign15360_e21292_d_b2;
        var_tmf2_db3 = assign15360_e21292_d_b3;

        let (assign15370_e21308, assign15370_e21308_d_n0, assign15370_e21308_d_n1, assign15370_e21308_d_n2, assign15370_e21308_d_n3, assign15370_e21308_d_n4, assign15370_e21308_d_n5, assign15370_e21308_d_b0, assign15370_e21308_d_b1, assign15370_e21308_d_b2, assign15370_e21308_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15370_e21303: f64 = (var_tmf1 * var_tmf1);
        let assign15370_e21305: f64 = (assign15370_e21303 + var_tmf2);
        let assign15370_e21306: f64 = (assign15370_e21305).sqrt();
        (assign15370_e21306, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15370_e21306)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15370_e21306)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15370_e21306)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15370_e21306)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15370_e21306)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15370_e21306)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15370_e21308;
        var_tmf2_dn0 = assign15370_e21308_d_n0;
        var_tmf2_dn1 = assign15370_e21308_d_n1;
        var_tmf2_dn2 = assign15370_e21308_d_n2;
        var_tmf2_dn3 = assign15370_e21308_d_n3;
        var_tmf2_dn4 = assign15370_e21308_d_n4;
        var_tmf2_dn5 = assign15370_e21308_d_n5;
        var_tmf2_db0 = assign15370_e21308_d_b0;
        var_tmf2_db1 = assign15370_e21308_d_b1;
        var_tmf2_db2 = assign15370_e21308_d_b2;
        var_tmf2_db3 = assign15370_e21308_d_b3;

        let (assign15380_e21325, assign15380_e21325_d_n0, assign15380_e21325_d_n1, assign15380_e21325_d_n2, assign15380_e21325_d_n3, assign15380_e21325_d_n4, assign15380_e21325_d_n5, assign15380_e21325_d_b0, assign15380_e21325_d_b1, assign15380_e21325_d_b2, assign15380_e21325_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15380_e21321: f64 = (var_tmf1 + var_tmf2);
        let assign15380_e21322: f64 = (0.5 * assign15380_e21321);
        let assign15380_e21323: f64 = (var_nfasti_i + assign15380_e21322);
        (assign15380_e21323, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15380_e21325;
        var_nj0_dn0 = assign15380_e21325_d_n0;
        var_nj0_dn1 = assign15380_e21325_d_n1;
        var_nj0_dn2 = assign15380_e21325_d_n2;
        var_nj0_dn3 = assign15380_e21325_d_n3;
        var_nj0_dn4 = assign15380_e21325_d_n4;
        var_nj0_dn5 = assign15380_e21325_d_n5;
        var_nj0_db0 = assign15380_e21325_d_b0;
        var_nj0_db1 = assign15380_e21325_d_b1;
        var_nj0_db2 = assign15380_e21325_d_b2;
        var_nj0_db3 = assign15380_e21325_d_b3;

        let (assign15390_e21340, assign15390_e21340_d_n0, assign15390_e21340_d_n1, assign15390_e21340_d_n2, assign15390_e21340_d_n3, assign15390_e21340_d_n4, assign15390_e21340_d_n5, assign15390_e21340_d_b0, assign15390_e21340_d_b1, assign15390_e21340_d_b2, assign15390_e21340_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 != 0.0)) {
        let assign15390_e21336: f64 = (p.p86 * var_dfn_su);
        let assign15390_e21338: f64 = (assign15390_e21336 * var_dfn_sl);
        (assign15390_e21338, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign15390_e21336 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15390_e21340;
        var_dnj1_dv_dn0 = assign15390_e21340_d_n0;
        var_dnj1_dv_dn1 = assign15390_e21340_d_n1;
        var_dnj1_dv_dn2 = assign15390_e21340_d_n2;
        var_dnj1_dv_dn3 = assign15390_e21340_d_n3;
        var_dnj1_dv_dn4 = assign15390_e21340_d_n4;
        var_dnj1_dv_dn5 = assign15390_e21340_d_n5;
        var_dnj1_dv_db0 = assign15390_e21340_d_b0;
        var_dnj1_dv_db1 = assign15390_e21340_d_b1;
        var_dnj1_dv_db2 = assign15390_e21340_d_b2;
        var_dnj1_dv_db3 = assign15390_e21340_d_b3;

        let (assign15400_e21352, assign15400_e21352_d_n0, assign15400_e21352_d_n1, assign15400_e21352_d_n2, assign15400_e21352_d_n3, assign15400_e21352_d_n4, assign15400_e21352_d_n5, assign15400_e21352_d_b0, assign15400_e21352_d_b1, assign15400_e21352_d_b2, assign15400_e21352_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15400_e21352;
        var_nj0_dn0 = assign15400_e21352_d_n0;
        var_nj0_dn1 = assign15400_e21352_d_n1;
        var_nj0_dn2 = assign15400_e21352_d_n2;
        var_nj0_dn3 = assign15400_e21352_d_n3;
        var_nj0_dn4 = assign15400_e21352_d_n4;
        var_nj0_dn5 = assign15400_e21352_d_n5;
        var_nj0_db0 = assign15400_e21352_d_b0;
        var_nj0_db1 = assign15400_e21352_d_b1;
        var_nj0_db2 = assign15400_e21352_d_b2;
        var_nj0_db3 = assign15400_e21352_d_b3;

        let (assign15410_e21364, assign15410_e21364_d_n0, assign15410_e21364_d_n1, assign15410_e21364_d_n2, assign15410_e21364_d_n3, assign15410_e21364_d_n4, assign15410_e21364_d_n5, assign15410_e21364_d_b0, assign15410_e21364_d_b1, assign15410_e21364_d_b2, assign15410_e21364_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign15410_e21364;
        var_nj1_dn0 = assign15410_e21364_d_n0;
        var_nj1_dn1 = assign15410_e21364_d_n1;
        var_nj1_dn2 = assign15410_e21364_d_n2;
        var_nj1_dn3 = assign15410_e21364_d_n3;
        var_nj1_dn4 = assign15410_e21364_d_n4;
        var_nj1_dn5 = assign15410_e21364_d_n5;
        var_nj1_db0 = assign15410_e21364_d_b0;
        var_nj1_db1 = assign15410_e21364_d_b1;
        var_nj1_db2 = assign15410_e21364_d_b2;
        var_nj1_db3 = assign15410_e21364_d_b3;

        let (assign15420_e21376, assign15420_e21376_d_n0, assign15420_e21376_d_n1, assign15420_e21376_d_n2, assign15420_e21376_d_n3, assign15420_e21376_d_n4, assign15420_e21376_d_n5, assign15420_e21376_d_b0, assign15420_e21376_d_b1, assign15420_e21376_d_b2, assign15420_e21376_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard246 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15420_e21376;
        var_dnj1_dv_dn0 = assign15420_e21376_d_n0;
        var_dnj1_dv_dn1 = assign15420_e21376_d_n1;
        var_dnj1_dv_dn2 = assign15420_e21376_d_n2;
        var_dnj1_dv_dn3 = assign15420_e21376_d_n3;
        var_dnj1_dv_dn4 = assign15420_e21376_d_n4;
        var_dnj1_dv_dn5 = assign15420_e21376_d_n5;
        var_dnj1_dv_db0 = assign15420_e21376_d_b0;
        var_dnj1_dv_db1 = assign15420_e21376_d_b1;
        var_dnj1_dv_db2 = assign15420_e21376_d_b2;
        var_dnj1_dv_db3 = assign15420_e21376_d_b3;

        let (assign15480_e21625, assign15480_e21625_d_n0, assign15480_e21625_d_n1, assign15480_e21625_d_n2, assign15480_e21625_d_n3, assign15480_e21625_d_n4, assign15480_e21625_d_n5, assign15480_e21625_d_b0, assign15480_e21625_d_b1, assign15480_e21625_d_b2, assign15480_e21625_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15480_e21609: f64 = (var_vmax * var_dnj1_dv);
        let assign15480_e21610: f64 = (var_nj1 - assign15480_e21609);
        let assign15480_e21613: f64 = (var_nj1 * var_nj1);
        let assign15480_e21614: f64 = (assign15480_e21610 / assign15480_e21613);
        let assign15480_e21617: f64 = (var_vha1 * var_dnj1_dv);
        let assign15480_e21620: f64 = (var_nj0 * p.p85);
        let assign15480_e21621: f64 = (assign15480_e21617 / assign15480_e21620);
        let assign15480_e21622: f64 = (assign15480_e21614 + assign15480_e21621);
        let assign15480_e21623: f64 = (var_phitdinv * assign15480_e21622);
        (assign15480_e21623, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn0 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn1 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn2 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn3 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn4 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_dn5 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_db0) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_db0 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_db1) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_db1 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_db2) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_db2 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign15480_e21613) - (assign15480_e21610 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign15480_e21613 * assign15480_e21613)) + ((((var_vha1 * var_dnj1_dv_db3) * assign15480_e21620) - (assign15480_e21617 * (var_nj0_db3 * p.p85))) / (assign15480_e21620 * assign15480_e21620)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign15480_e21625;
        var_dvmax_over_phitd_dv_dn0 = assign15480_e21625_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign15480_e21625_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign15480_e21625_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign15480_e21625_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign15480_e21625_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign15480_e21625_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign15480_e21625_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign15480_e21625_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign15480_e21625_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign15480_e21625_d_b3;

        let (assign15500_e21655,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15500_e21651: f64 = (var_nin * var_nin);
        let assign15500_e21653: f64 = (assign15500_e21651 / var_ndigat_i);
        (assign15500_e21653,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign15500_e21655;

        let (assign15510_e21671,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15510_e21664: f64 = (var_nfagat_i / var_phitdinv);
        let assign15510_e21667: f64 = (var_ndigat_i / var_pnn0);
        let assign15510_e21668: f64 = (assign15510_e21667).ln();
        let assign15510_e21669: f64 = (assign15510_e21664 * assign15510_e21668);
        (assign15510_e21669,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign15510_e21671;

        let assign15520_e21674: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard249 = assign15520_e21674;

        let (assign15530_e21691, assign15530_e21691_d_n0, assign15530_e21691_d_n1, assign15530_e21691_d_n2, assign15530_e21691_d_n3, assign15530_e21691_d_n4, assign15530_e21691_d_n5, assign15530_e21691_d_b0, assign15530_e21691_d_b1, assign15530_e21691_d_b2, assign15530_e21691_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15530_e21686: f64 = (var_vmax - var_vha1);
        let assign15530_e21687: f64 = (p.p86 * assign15530_e21686);
        let assign15530_e21689: f64 = (assign15530_e21687 + var_nfagat_i);
        (assign15530_e21689, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign15530_e21691;
        var_nja10_dn0 = assign15530_e21691_d_n0;
        var_nja10_dn1 = assign15530_e21691_d_n1;
        var_nja10_dn2 = assign15530_e21691_d_n2;
        var_nja10_dn3 = assign15530_e21691_d_n3;
        var_nja10_dn4 = assign15530_e21691_d_n4;
        var_nja10_dn5 = assign15530_e21691_d_n5;
        var_nja10_db0 = assign15530_e21691_d_b0;
        var_nja10_db1 = assign15530_e21691_d_b1;
        var_nja10_db2 = assign15530_e21691_d_b2;
        var_nja10_db3 = assign15530_e21691_d_b3;

        let (assign15540_e21706, assign15540_e21706_d_n0, assign15540_e21706_d_n1, assign15540_e21706_d_n2, assign15540_e21706_d_n3, assign15540_e21706_d_n4, assign15540_e21706_d_n5, assign15540_e21706_d_b0, assign15540_e21706_d_b1, assign15540_e21706_d_b2, assign15540_e21706_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15540_e21703: f64 = (p.p86 * var_vha1);
        let assign15540_e21704: f64 = (var_nfagat_i - assign15540_e21703);
        (assign15540_e21704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15540_e21706;
        var_nj0_dn0 = assign15540_e21706_d_n0;
        var_nj0_dn1 = assign15540_e21706_d_n1;
        var_nj0_dn2 = assign15540_e21706_d_n2;
        var_nj0_dn3 = assign15540_e21706_d_n3;
        var_nj0_dn4 = assign15540_e21706_d_n4;
        var_nj0_dn5 = assign15540_e21706_d_n5;
        var_nj0_db0 = assign15540_e21706_d_b0;
        var_nj0_db1 = assign15540_e21706_d_b1;
        var_nj0_db2 = assign15540_e21706_d_b2;
        var_nj0_db3 = assign15540_e21706_d_b3;

        let (assign15550_e21721, assign15550_e21721_d_n0, assign15550_e21721_d_n1, assign15550_e21721_d_n2, assign15550_e21721_d_n3, assign15550_e21721_d_n4, assign15550_e21721_d_n5, assign15550_e21721_d_b0, assign15550_e21721_d_b1, assign15550_e21721_d_b2, assign15550_e21721_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15550_e21717: f64 = (p.p85 - var_nja10);
        let assign15550_e21719: f64 = (assign15550_e21717 - 0.01);
        (assign15550_e21719, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15550_e21721;
        var_tmf1_dn0 = assign15550_e21721_d_n0;
        var_tmf1_dn1 = assign15550_e21721_d_n1;
        var_tmf1_dn2 = assign15550_e21721_d_n2;
        var_tmf1_dn3 = assign15550_e21721_d_n3;
        var_tmf1_dn4 = assign15550_e21721_d_n4;
        var_tmf1_dn5 = assign15550_e21721_d_n5;
        var_tmf1_db0 = assign15550_e21721_d_b0;
        var_tmf1_db1 = assign15550_e21721_d_b1;
        var_tmf1_db2 = assign15550_e21721_d_b2;
        var_tmf1_db3 = assign15550_e21721_d_b3;

        let (assign15560_e21736, assign15560_e21736_d_n0, assign15560_e21736_d_n1, assign15560_e21736_d_n2, assign15560_e21736_d_n3, assign15560_e21736_d_n4, assign15560_e21736_d_n5, assign15560_e21736_d_b0, assign15560_e21736_d_b1, assign15560_e21736_d_b2, assign15560_e21736_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15560_e21732: f64 = (4.0 * p.p85);
        let assign15560_e21734: f64 = (assign15560_e21732 * 0.01);
        (assign15560_e21734, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15560_e21736;
        var_tmf2_dn0 = assign15560_e21736_d_n0;
        var_tmf2_dn1 = assign15560_e21736_d_n1;
        var_tmf2_dn2 = assign15560_e21736_d_n2;
        var_tmf2_dn3 = assign15560_e21736_d_n3;
        var_tmf2_dn4 = assign15560_e21736_d_n4;
        var_tmf2_dn5 = assign15560_e21736_d_n5;
        var_tmf2_db0 = assign15560_e21736_d_b0;
        var_tmf2_db1 = assign15560_e21736_d_b1;
        var_tmf2_db2 = assign15560_e21736_d_b2;
        var_tmf2_db3 = assign15560_e21736_d_b3;

        let (assign15570_e21753, assign15570_e21753_d_n0, assign15570_e21753_d_n1, assign15570_e21753_d_n2, assign15570_e21753_d_n3, assign15570_e21753_d_n4, assign15570_e21753_d_n5, assign15570_e21753_d_b0, assign15570_e21753_d_b1, assign15570_e21753_d_b2, assign15570_e21753_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n1, assign15570_e21751_d_n2, assign15570_e21751_d_n3, assign15570_e21751_d_n4, assign15570_e21751_d_n5, assign15570_e21751_d_b0, assign15570_e21751_d_b1, assign15570_e21751_d_b2, assign15570_e21751_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15570_e21750: f64 = (-var_tmf2);
                (assign15570_e21750, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15570_e21751, assign15570_e21751_d_n0, assign15570_e21751_d_n1, assign15570_e21751_d_n2, assign15570_e21751_d_n3, assign15570_e21751_d_n4, assign15570_e21751_d_n5, assign15570_e21751_d_b0, assign15570_e21751_d_b1, assign15570_e21751_d_b2, assign15570_e21751_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15570_e21753;
        var_tmf2_dn0 = assign15570_e21753_d_n0;
        var_tmf2_dn1 = assign15570_e21753_d_n1;
        var_tmf2_dn2 = assign15570_e21753_d_n2;
        var_tmf2_dn3 = assign15570_e21753_d_n3;
        var_tmf2_dn4 = assign15570_e21753_d_n4;
        var_tmf2_dn5 = assign15570_e21753_d_n5;
        var_tmf2_db0 = assign15570_e21753_d_b0;
        var_tmf2_db1 = assign15570_e21753_d_b1;
        var_tmf2_db2 = assign15570_e21753_d_b2;
        var_tmf2_db3 = assign15570_e21753_d_b3;

        let (assign15580_e21769, assign15580_e21769_d_n0, assign15580_e21769_d_n1, assign15580_e21769_d_n2, assign15580_e21769_d_n3, assign15580_e21769_d_n4, assign15580_e21769_d_n5, assign15580_e21769_d_b0, assign15580_e21769_d_b1, assign15580_e21769_d_b2, assign15580_e21769_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15580_e21764: f64 = (var_tmf1 * var_tmf1);
        let assign15580_e21766: f64 = (assign15580_e21764 + var_tmf2);
        let assign15580_e21767: f64 = (assign15580_e21766).sqrt();
        (assign15580_e21767, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15580_e21767)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15580_e21767)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15580_e21767)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15580_e21767)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15580_e21767)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15580_e21767)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15580_e21769;
        var_tmf2_dn0 = assign15580_e21769_d_n0;
        var_tmf2_dn1 = assign15580_e21769_d_n1;
        var_tmf2_dn2 = assign15580_e21769_d_n2;
        var_tmf2_dn3 = assign15580_e21769_d_n3;
        var_tmf2_dn4 = assign15580_e21769_d_n4;
        var_tmf2_dn5 = assign15580_e21769_d_n5;
        var_tmf2_db0 = assign15580_e21769_d_b0;
        var_tmf2_db1 = assign15580_e21769_d_b1;
        var_tmf2_db2 = assign15580_e21769_d_b2;
        var_tmf2_db3 = assign15580_e21769_d_b3;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_guard249_slot = var_guard249;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        var_guard230: f64,
        var_guard231: f64,
        var_guard249: f64,
        var_guard31: f64,
        var_nfagat_i: f64,
        var_phitdinv: f64,
        var_vha1: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;

        let (assign15590_e21786, assign15590_e21786_d_n0, assign15590_e21786_d_n1, assign15590_e21786_d_n2, assign15590_e21786_d_n3, assign15590_e21786_d_n4, assign15590_e21786_d_n5, assign15590_e21786_d_b0, assign15590_e21786_d_b1, assign15590_e21786_d_b2, assign15590_e21786_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15590_e21782: f64 = (var_tmf1 / var_tmf2);
        let assign15590_e21783: f64 = (1.0 + assign15590_e21782);
        let assign15590_e21784: f64 = (0.5 * assign15590_e21783);
        (assign15590_e21784, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign15590_e21786;
        var_dfn_su_dn0 = assign15590_e21786_d_n0;
        var_dfn_su_dn1 = assign15590_e21786_d_n1;
        var_dfn_su_dn2 = assign15590_e21786_d_n2;
        var_dfn_su_dn3 = assign15590_e21786_d_n3;
        var_dfn_su_dn4 = assign15590_e21786_d_n4;
        var_dfn_su_dn5 = assign15590_e21786_d_n5;
        var_dfn_su_db0 = assign15590_e21786_d_b0;
        var_dfn_su_db1 = assign15590_e21786_d_b1;
        var_dfn_su_db2 = assign15590_e21786_d_b2;
        var_dfn_su_db3 = assign15590_e21786_d_b3;

        let (assign15600_e21803, assign15600_e21803_d_n0, assign15600_e21803_d_n1, assign15600_e21803_d_n2, assign15600_e21803_d_n3, assign15600_e21803_d_n4, assign15600_e21803_d_n5, assign15600_e21803_d_b0, assign15600_e21803_d_b1, assign15600_e21803_d_b2, assign15600_e21803_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15600_e21799: f64 = (var_tmf1 + var_tmf2);
        let assign15600_e21800: f64 = (0.5 * assign15600_e21799);
        let assign15600_e21801: f64 = (p.p85 - assign15600_e21800);
        (assign15600_e21801, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign15600_e21803;
        var_nja11_dn0 = assign15600_e21803_d_n0;
        var_nja11_dn1 = assign15600_e21803_d_n1;
        var_nja11_dn2 = assign15600_e21803_d_n2;
        var_nja11_dn3 = assign15600_e21803_d_n3;
        var_nja11_dn4 = assign15600_e21803_d_n4;
        var_nja11_dn5 = assign15600_e21803_d_n5;
        var_nja11_db0 = assign15600_e21803_d_b0;
        var_nja11_db1 = assign15600_e21803_d_b1;
        var_nja11_db2 = assign15600_e21803_d_b2;
        var_nja11_db3 = assign15600_e21803_d_b3;

        let (assign15610_e21818, assign15610_e21818_d_n0, assign15610_e21818_d_n1, assign15610_e21818_d_n2, assign15610_e21818_d_n3, assign15610_e21818_d_n4, assign15610_e21818_d_n5, assign15610_e21818_d_b0, assign15610_e21818_d_b1, assign15610_e21818_d_b2, assign15610_e21818_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15610_e21814: f64 = (var_nja11 - var_nfagat_i);
        let assign15610_e21816: f64 = (assign15610_e21814 - 0.01);
        (assign15610_e21816, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15610_e21818;
        var_tmf1_dn0 = assign15610_e21818_d_n0;
        var_tmf1_dn1 = assign15610_e21818_d_n1;
        var_tmf1_dn2 = assign15610_e21818_d_n2;
        var_tmf1_dn3 = assign15610_e21818_d_n3;
        var_tmf1_dn4 = assign15610_e21818_d_n4;
        var_tmf1_dn5 = assign15610_e21818_d_n5;
        var_tmf1_db0 = assign15610_e21818_d_b0;
        var_tmf1_db1 = assign15610_e21818_d_b1;
        var_tmf1_db2 = assign15610_e21818_d_b2;
        var_tmf1_db3 = assign15610_e21818_d_b3;

        let (assign15620_e21833, assign15620_e21833_d_n0, assign15620_e21833_d_n1, assign15620_e21833_d_n2, assign15620_e21833_d_n3, assign15620_e21833_d_n4, assign15620_e21833_d_n5, assign15620_e21833_d_b0, assign15620_e21833_d_b1, assign15620_e21833_d_b2, assign15620_e21833_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15620_e21829: f64 = (4.0 * var_nfagat_i);
        let assign15620_e21831: f64 = (assign15620_e21829 * 0.01);
        (assign15620_e21831, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15620_e21833;
        var_tmf2_dn0 = assign15620_e21833_d_n0;
        var_tmf2_dn1 = assign15620_e21833_d_n1;
        var_tmf2_dn2 = assign15620_e21833_d_n2;
        var_tmf2_dn3 = assign15620_e21833_d_n3;
        var_tmf2_dn4 = assign15620_e21833_d_n4;
        var_tmf2_dn5 = assign15620_e21833_d_n5;
        var_tmf2_db0 = assign15620_e21833_d_b0;
        var_tmf2_db1 = assign15620_e21833_d_b1;
        var_tmf2_db2 = assign15620_e21833_d_b2;
        var_tmf2_db3 = assign15620_e21833_d_b3;

        let (assign15630_e21850, assign15630_e21850_d_n0, assign15630_e21850_d_n1, assign15630_e21850_d_n2, assign15630_e21850_d_n3, assign15630_e21850_d_n4, assign15630_e21850_d_n5, assign15630_e21850_d_b0, assign15630_e21850_d_b1, assign15630_e21850_d_b2, assign15630_e21850_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n1, assign15630_e21848_d_n2, assign15630_e21848_d_n3, assign15630_e21848_d_n4, assign15630_e21848_d_n5, assign15630_e21848_d_b0, assign15630_e21848_d_b1, assign15630_e21848_d_b2, assign15630_e21848_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15630_e21847: f64 = (-var_tmf2);
                (assign15630_e21847, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15630_e21848, assign15630_e21848_d_n0, assign15630_e21848_d_n1, assign15630_e21848_d_n2, assign15630_e21848_d_n3, assign15630_e21848_d_n4, assign15630_e21848_d_n5, assign15630_e21848_d_b0, assign15630_e21848_d_b1, assign15630_e21848_d_b2, assign15630_e21848_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15630_e21850;
        var_tmf2_dn0 = assign15630_e21850_d_n0;
        var_tmf2_dn1 = assign15630_e21850_d_n1;
        var_tmf2_dn2 = assign15630_e21850_d_n2;
        var_tmf2_dn3 = assign15630_e21850_d_n3;
        var_tmf2_dn4 = assign15630_e21850_d_n4;
        var_tmf2_dn5 = assign15630_e21850_d_n5;
        var_tmf2_db0 = assign15630_e21850_d_b0;
        var_tmf2_db1 = assign15630_e21850_d_b1;
        var_tmf2_db2 = assign15630_e21850_d_b2;
        var_tmf2_db3 = assign15630_e21850_d_b3;

        let (assign15640_e21866, assign15640_e21866_d_n0, assign15640_e21866_d_n1, assign15640_e21866_d_n2, assign15640_e21866_d_n3, assign15640_e21866_d_n4, assign15640_e21866_d_n5, assign15640_e21866_d_b0, assign15640_e21866_d_b1, assign15640_e21866_d_b2, assign15640_e21866_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15640_e21861: f64 = (var_tmf1 * var_tmf1);
        let assign15640_e21863: f64 = (assign15640_e21861 + var_tmf2);
        let assign15640_e21864: f64 = (assign15640_e21863).sqrt();
        (assign15640_e21864, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15640_e21864)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15640_e21864)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15640_e21864)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15640_e21864)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15640_e21864)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15640_e21864)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15640_e21866;
        var_tmf2_dn0 = assign15640_e21866_d_n0;
        var_tmf2_dn1 = assign15640_e21866_d_n1;
        var_tmf2_dn2 = assign15640_e21866_d_n2;
        var_tmf2_dn3 = assign15640_e21866_d_n3;
        var_tmf2_dn4 = assign15640_e21866_d_n4;
        var_tmf2_dn5 = assign15640_e21866_d_n5;
        var_tmf2_db0 = assign15640_e21866_d_b0;
        var_tmf2_db1 = assign15640_e21866_d_b1;
        var_tmf2_db2 = assign15640_e21866_d_b2;
        var_tmf2_db3 = assign15640_e21866_d_b3;

        let (assign15650_e21883, assign15650_e21883_d_n0, assign15650_e21883_d_n1, assign15650_e21883_d_n2, assign15650_e21883_d_n3, assign15650_e21883_d_n4, assign15650_e21883_d_n5, assign15650_e21883_d_b0, assign15650_e21883_d_b1, assign15650_e21883_d_b2, assign15650_e21883_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15650_e21879: f64 = (var_tmf1 / var_tmf2);
        let assign15650_e21880: f64 = (1.0 + assign15650_e21879);
        let assign15650_e21881: f64 = (0.5 * assign15650_e21880);
        (assign15650_e21881, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign15650_e21883;
        var_dfn_sl_dn0 = assign15650_e21883_d_n0;
        var_dfn_sl_dn1 = assign15650_e21883_d_n1;
        var_dfn_sl_dn2 = assign15650_e21883_d_n2;
        var_dfn_sl_dn3 = assign15650_e21883_d_n3;
        var_dfn_sl_dn4 = assign15650_e21883_d_n4;
        var_dfn_sl_dn5 = assign15650_e21883_d_n5;
        var_dfn_sl_db0 = assign15650_e21883_d_b0;
        var_dfn_sl_db1 = assign15650_e21883_d_b1;
        var_dfn_sl_db2 = assign15650_e21883_d_b2;
        var_dfn_sl_db3 = assign15650_e21883_d_b3;

        let (assign15660_e21900, assign15660_e21900_d_n0, assign15660_e21900_d_n1, assign15660_e21900_d_n2, assign15660_e21900_d_n3, assign15660_e21900_d_n4, assign15660_e21900_d_n5, assign15660_e21900_d_b0, assign15660_e21900_d_b1, assign15660_e21900_d_b2, assign15660_e21900_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15660_e21896: f64 = (var_tmf1 + var_tmf2);
        let assign15660_e21897: f64 = (0.5 * assign15660_e21896);
        let assign15660_e21898: f64 = (var_nfagat_i + assign15660_e21897);
        (assign15660_e21898, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign15660_e21900;
        var_nj1_dn0 = assign15660_e21900_d_n0;
        var_nj1_dn1 = assign15660_e21900_d_n1;
        var_nj1_dn2 = assign15660_e21900_d_n2;
        var_nj1_dn3 = assign15660_e21900_d_n3;
        var_nj1_dn4 = assign15660_e21900_d_n4;
        var_nj1_dn5 = assign15660_e21900_d_n5;
        var_nj1_db0 = assign15660_e21900_d_b0;
        var_nj1_db1 = assign15660_e21900_d_b1;
        var_nj1_db2 = assign15660_e21900_d_b2;
        var_nj1_db3 = assign15660_e21900_d_b3;

        let (assign15670_e21915, assign15670_e21915_d_n0, assign15670_e21915_d_n1, assign15670_e21915_d_n2, assign15670_e21915_d_n3, assign15670_e21915_d_n4, assign15670_e21915_d_n5, assign15670_e21915_d_b0, assign15670_e21915_d_b1, assign15670_e21915_d_b2, assign15670_e21915_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15670_e21911: f64 = (p.p85 - var_nj0);
        let assign15670_e21913: f64 = (assign15670_e21911 - 0.01);
        (assign15670_e21913, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15670_e21915;
        var_tmf1_dn0 = assign15670_e21915_d_n0;
        var_tmf1_dn1 = assign15670_e21915_d_n1;
        var_tmf1_dn2 = assign15670_e21915_d_n2;
        var_tmf1_dn3 = assign15670_e21915_d_n3;
        var_tmf1_dn4 = assign15670_e21915_d_n4;
        var_tmf1_dn5 = assign15670_e21915_d_n5;
        var_tmf1_db0 = assign15670_e21915_d_b0;
        var_tmf1_db1 = assign15670_e21915_d_b1;
        var_tmf1_db2 = assign15670_e21915_d_b2;
        var_tmf1_db3 = assign15670_e21915_d_b3;

        let (assign15680_e21930, assign15680_e21930_d_n0, assign15680_e21930_d_n1, assign15680_e21930_d_n2, assign15680_e21930_d_n3, assign15680_e21930_d_n4, assign15680_e21930_d_n5, assign15680_e21930_d_b0, assign15680_e21930_d_b1, assign15680_e21930_d_b2, assign15680_e21930_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15680_e21926: f64 = (4.0 * p.p85);
        let assign15680_e21928: f64 = (assign15680_e21926 * 0.01);
        (assign15680_e21928, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15680_e21930;
        var_tmf2_dn0 = assign15680_e21930_d_n0;
        var_tmf2_dn1 = assign15680_e21930_d_n1;
        var_tmf2_dn2 = assign15680_e21930_d_n2;
        var_tmf2_dn3 = assign15680_e21930_d_n3;
        var_tmf2_dn4 = assign15680_e21930_d_n4;
        var_tmf2_dn5 = assign15680_e21930_d_n5;
        var_tmf2_db0 = assign15680_e21930_d_b0;
        var_tmf2_db1 = assign15680_e21930_d_b1;
        var_tmf2_db2 = assign15680_e21930_d_b2;
        var_tmf2_db3 = assign15680_e21930_d_b3;

        let (assign15690_e21947, assign15690_e21947_d_n0, assign15690_e21947_d_n1, assign15690_e21947_d_n2, assign15690_e21947_d_n3, assign15690_e21947_d_n4, assign15690_e21947_d_n5, assign15690_e21947_d_b0, assign15690_e21947_d_b1, assign15690_e21947_d_b2, assign15690_e21947_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n1, assign15690_e21945_d_n2, assign15690_e21945_d_n3, assign15690_e21945_d_n4, assign15690_e21945_d_n5, assign15690_e21945_d_b0, assign15690_e21945_d_b1, assign15690_e21945_d_b2, assign15690_e21945_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15690_e21944: f64 = (-var_tmf2);
                (assign15690_e21944, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15690_e21945, assign15690_e21945_d_n0, assign15690_e21945_d_n1, assign15690_e21945_d_n2, assign15690_e21945_d_n3, assign15690_e21945_d_n4, assign15690_e21945_d_n5, assign15690_e21945_d_b0, assign15690_e21945_d_b1, assign15690_e21945_d_b2, assign15690_e21945_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15690_e21947;
        var_tmf2_dn0 = assign15690_e21947_d_n0;
        var_tmf2_dn1 = assign15690_e21947_d_n1;
        var_tmf2_dn2 = assign15690_e21947_d_n2;
        var_tmf2_dn3 = assign15690_e21947_d_n3;
        var_tmf2_dn4 = assign15690_e21947_d_n4;
        var_tmf2_dn5 = assign15690_e21947_d_n5;
        var_tmf2_db0 = assign15690_e21947_d_b0;
        var_tmf2_db1 = assign15690_e21947_d_b1;
        var_tmf2_db2 = assign15690_e21947_d_b2;
        var_tmf2_db3 = assign15690_e21947_d_b3;

        let (assign15700_e21963, assign15700_e21963_d_n0, assign15700_e21963_d_n1, assign15700_e21963_d_n2, assign15700_e21963_d_n3, assign15700_e21963_d_n4, assign15700_e21963_d_n5, assign15700_e21963_d_b0, assign15700_e21963_d_b1, assign15700_e21963_d_b2, assign15700_e21963_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15700_e21958: f64 = (var_tmf1 * var_tmf1);
        let assign15700_e21960: f64 = (assign15700_e21958 + var_tmf2);
        let assign15700_e21961: f64 = (assign15700_e21960).sqrt();
        (assign15700_e21961, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15700_e21961)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15700_e21961)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15700_e21961)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15700_e21961)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15700_e21961)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15700_e21961)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15700_e21963;
        var_tmf2_dn0 = assign15700_e21963_d_n0;
        var_tmf2_dn1 = assign15700_e21963_d_n1;
        var_tmf2_dn2 = assign15700_e21963_d_n2;
        var_tmf2_dn3 = assign15700_e21963_d_n3;
        var_tmf2_dn4 = assign15700_e21963_d_n4;
        var_tmf2_dn5 = assign15700_e21963_d_n5;
        var_tmf2_db0 = assign15700_e21963_d_b0;
        var_tmf2_db1 = assign15700_e21963_d_b1;
        var_tmf2_db2 = assign15700_e21963_d_b2;
        var_tmf2_db3 = assign15700_e21963_d_b3;

        let (assign15710_e21980, assign15710_e21980_d_n0, assign15710_e21980_d_n1, assign15710_e21980_d_n2, assign15710_e21980_d_n3, assign15710_e21980_d_n4, assign15710_e21980_d_n5, assign15710_e21980_d_b0, assign15710_e21980_d_b1, assign15710_e21980_d_b2, assign15710_e21980_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15710_e21976: f64 = (var_tmf1 + var_tmf2);
        let assign15710_e21977: f64 = (0.5 * assign15710_e21976);
        let assign15710_e21978: f64 = (p.p85 - assign15710_e21977);
        (assign15710_e21978, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15710_e21980;
        var_nj0_dn0 = assign15710_e21980_d_n0;
        var_nj0_dn1 = assign15710_e21980_d_n1;
        var_nj0_dn2 = assign15710_e21980_d_n2;
        var_nj0_dn3 = assign15710_e21980_d_n3;
        var_nj0_dn4 = assign15710_e21980_d_n4;
        var_nj0_dn5 = assign15710_e21980_d_n5;
        var_nj0_db0 = assign15710_e21980_d_b0;
        var_nj0_db1 = assign15710_e21980_d_b1;
        var_nj0_db2 = assign15710_e21980_d_b2;
        var_nj0_db3 = assign15710_e21980_d_b3;

        let (assign15720_e21995, assign15720_e21995_d_n0, assign15720_e21995_d_n1, assign15720_e21995_d_n2, assign15720_e21995_d_n3, assign15720_e21995_d_n4, assign15720_e21995_d_n5, assign15720_e21995_d_b0, assign15720_e21995_d_b1, assign15720_e21995_d_b2, assign15720_e21995_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15720_e21991: f64 = (var_nj0 - var_nfagat_i);
        let assign15720_e21993: f64 = (assign15720_e21991 - 0.01);
        (assign15720_e21993, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign15720_e21995;
        var_tmf1_dn0 = assign15720_e21995_d_n0;
        var_tmf1_dn1 = assign15720_e21995_d_n1;
        var_tmf1_dn2 = assign15720_e21995_d_n2;
        var_tmf1_dn3 = assign15720_e21995_d_n3;
        var_tmf1_dn4 = assign15720_e21995_d_n4;
        var_tmf1_dn5 = assign15720_e21995_d_n5;
        var_tmf1_db0 = assign15720_e21995_d_b0;
        var_tmf1_db1 = assign15720_e21995_d_b1;
        var_tmf1_db2 = assign15720_e21995_d_b2;
        var_tmf1_db3 = assign15720_e21995_d_b3;

        let (assign15730_e22010, assign15730_e22010_d_n0, assign15730_e22010_d_n1, assign15730_e22010_d_n2, assign15730_e22010_d_n3, assign15730_e22010_d_n4, assign15730_e22010_d_n5, assign15730_e22010_d_b0, assign15730_e22010_d_b1, assign15730_e22010_d_b2, assign15730_e22010_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15730_e22006: f64 = (4.0 * var_nfagat_i);
        let assign15730_e22008: f64 = (assign15730_e22006 * 0.01);
        (assign15730_e22008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15730_e22010;
        var_tmf2_dn0 = assign15730_e22010_d_n0;
        var_tmf2_dn1 = assign15730_e22010_d_n1;
        var_tmf2_dn2 = assign15730_e22010_d_n2;
        var_tmf2_dn3 = assign15730_e22010_d_n3;
        var_tmf2_dn4 = assign15730_e22010_d_n4;
        var_tmf2_dn5 = assign15730_e22010_d_n5;
        var_tmf2_db0 = assign15730_e22010_d_b0;
        var_tmf2_db1 = assign15730_e22010_d_b1;
        var_tmf2_db2 = assign15730_e22010_d_b2;
        var_tmf2_db3 = assign15730_e22010_d_b3;

        let (assign15740_e22027, assign15740_e22027_d_n0, assign15740_e22027_d_n1, assign15740_e22027_d_n2, assign15740_e22027_d_n3, assign15740_e22027_d_n4, assign15740_e22027_d_n5, assign15740_e22027_d_b0, assign15740_e22027_d_b1, assign15740_e22027_d_b2, assign15740_e22027_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n1, assign15740_e22025_d_n2, assign15740_e22025_d_n3, assign15740_e22025_d_n4, assign15740_e22025_d_n5, assign15740_e22025_d_b0, assign15740_e22025_d_b1, assign15740_e22025_d_b2, assign15740_e22025_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign15740_e22024: f64 = (-var_tmf2);
                (assign15740_e22024, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign15740_e22025, assign15740_e22025_d_n0, assign15740_e22025_d_n1, assign15740_e22025_d_n2, assign15740_e22025_d_n3, assign15740_e22025_d_n4, assign15740_e22025_d_n5, assign15740_e22025_d_b0, assign15740_e22025_d_b1, assign15740_e22025_d_b2, assign15740_e22025_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15740_e22027;
        var_tmf2_dn0 = assign15740_e22027_d_n0;
        var_tmf2_dn1 = assign15740_e22027_d_n1;
        var_tmf2_dn2 = assign15740_e22027_d_n2;
        var_tmf2_dn3 = assign15740_e22027_d_n3;
        var_tmf2_dn4 = assign15740_e22027_d_n4;
        var_tmf2_dn5 = assign15740_e22027_d_n5;
        var_tmf2_db0 = assign15740_e22027_d_b0;
        var_tmf2_db1 = assign15740_e22027_d_b1;
        var_tmf2_db2 = assign15740_e22027_d_b2;
        var_tmf2_db3 = assign15740_e22027_d_b3;

        let (assign15750_e22043, assign15750_e22043_d_n0, assign15750_e22043_d_n1, assign15750_e22043_d_n2, assign15750_e22043_d_n3, assign15750_e22043_d_n4, assign15750_e22043_d_n5, assign15750_e22043_d_b0, assign15750_e22043_d_b1, assign15750_e22043_d_b2, assign15750_e22043_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15750_e22038: f64 = (var_tmf1 * var_tmf1);
        let assign15750_e22040: f64 = (assign15750_e22038 + var_tmf2);
        let assign15750_e22041: f64 = (assign15750_e22040).sqrt();
        (assign15750_e22041, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15750_e22041)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15750_e22041)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign15750_e22041)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign15750_e22041)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign15750_e22041)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign15750_e22041)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign15750_e22043;
        var_tmf2_dn0 = assign15750_e22043_d_n0;
        var_tmf2_dn1 = assign15750_e22043_d_n1;
        var_tmf2_dn2 = assign15750_e22043_d_n2;
        var_tmf2_dn3 = assign15750_e22043_d_n3;
        var_tmf2_dn4 = assign15750_e22043_d_n4;
        var_tmf2_dn5 = assign15750_e22043_d_n5;
        var_tmf2_db0 = assign15750_e22043_d_b0;
        var_tmf2_db1 = assign15750_e22043_d_b1;
        var_tmf2_db2 = assign15750_e22043_d_b2;
        var_tmf2_db3 = assign15750_e22043_d_b3;

        let (assign15760_e22060, assign15760_e22060_d_n0, assign15760_e22060_d_n1, assign15760_e22060_d_n2, assign15760_e22060_d_n3, assign15760_e22060_d_n4, assign15760_e22060_d_n5, assign15760_e22060_d_b0, assign15760_e22060_d_b1, assign15760_e22060_d_b2, assign15760_e22060_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15760_e22056: f64 = (var_tmf1 + var_tmf2);
        let assign15760_e22057: f64 = (0.5 * assign15760_e22056);
        let assign15760_e22058: f64 = (var_nfagat_i + assign15760_e22057);
        (assign15760_e22058, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15760_e22060;
        var_nj0_dn0 = assign15760_e22060_d_n0;
        var_nj0_dn1 = assign15760_e22060_d_n1;
        var_nj0_dn2 = assign15760_e22060_d_n2;
        var_nj0_dn3 = assign15760_e22060_d_n3;
        var_nj0_dn4 = assign15760_e22060_d_n4;
        var_nj0_dn5 = assign15760_e22060_d_n5;
        var_nj0_db0 = assign15760_e22060_d_b0;
        var_nj0_db1 = assign15760_e22060_d_b1;
        var_nj0_db2 = assign15760_e22060_d_b2;
        var_nj0_db3 = assign15760_e22060_d_b3;

        let (assign15770_e22075, assign15770_e22075_d_n0, assign15770_e22075_d_n1, assign15770_e22075_d_n2, assign15770_e22075_d_n3, assign15770_e22075_d_n4, assign15770_e22075_d_n5, assign15770_e22075_d_b0, assign15770_e22075_d_b1, assign15770_e22075_d_b2, assign15770_e22075_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 != 0.0)) {
        let assign15770_e22071: f64 = (p.p86 * var_dfn_su);
        let assign15770_e22073: f64 = (assign15770_e22071 * var_dfn_sl);
        (assign15770_e22073, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign15770_e22071 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15770_e22075;
        var_dnj1_dv_dn0 = assign15770_e22075_d_n0;
        var_dnj1_dv_dn1 = assign15770_e22075_d_n1;
        var_dnj1_dv_dn2 = assign15770_e22075_d_n2;
        var_dnj1_dv_dn3 = assign15770_e22075_d_n3;
        var_dnj1_dv_dn4 = assign15770_e22075_d_n4;
        var_dnj1_dv_dn5 = assign15770_e22075_d_n5;
        var_dnj1_dv_db0 = assign15770_e22075_d_b0;
        var_dnj1_dv_db1 = assign15770_e22075_d_b1;
        var_dnj1_dv_db2 = assign15770_e22075_d_b2;
        var_dnj1_dv_db3 = assign15770_e22075_d_b3;

        let (assign15780_e22087, assign15780_e22087_d_n0, assign15780_e22087_d_n1, assign15780_e22087_d_n2, assign15780_e22087_d_n3, assign15780_e22087_d_n4, assign15780_e22087_d_n5, assign15780_e22087_d_b0, assign15780_e22087_d_b1, assign15780_e22087_d_b2, assign15780_e22087_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign15780_e22087;
        var_nj0_dn0 = assign15780_e22087_d_n0;
        var_nj0_dn1 = assign15780_e22087_d_n1;
        var_nj0_dn2 = assign15780_e22087_d_n2;
        var_nj0_dn3 = assign15780_e22087_d_n3;
        var_nj0_dn4 = assign15780_e22087_d_n4;
        var_nj0_dn5 = assign15780_e22087_d_n5;
        var_nj0_db0 = assign15780_e22087_d_b0;
        var_nj0_db1 = assign15780_e22087_d_b1;
        var_nj0_db2 = assign15780_e22087_d_b2;
        var_nj0_db3 = assign15780_e22087_d_b3;

        let (assign15790_e22099, assign15790_e22099_d_n0, assign15790_e22099_d_n1, assign15790_e22099_d_n2, assign15790_e22099_d_n3, assign15790_e22099_d_n4, assign15790_e22099_d_n5, assign15790_e22099_d_b0, assign15790_e22099_d_b1, assign15790_e22099_d_b2, assign15790_e22099_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign15790_e22099;
        var_nj1_dn0 = assign15790_e22099_d_n0;
        var_nj1_dn1 = assign15790_e22099_d_n1;
        var_nj1_dn2 = assign15790_e22099_d_n2;
        var_nj1_dn3 = assign15790_e22099_d_n3;
        var_nj1_dn4 = assign15790_e22099_d_n4;
        var_nj1_dn5 = assign15790_e22099_d_n5;
        var_nj1_db0 = assign15790_e22099_d_b0;
        var_nj1_db1 = assign15790_e22099_d_b1;
        var_nj1_db2 = assign15790_e22099_d_b2;
        var_nj1_db3 = assign15790_e22099_d_b3;

        let (assign15800_e22111, assign15800_e22111_d_n0, assign15800_e22111_d_n1, assign15800_e22111_d_n2, assign15800_e22111_d_n3, assign15800_e22111_d_n4, assign15800_e22111_d_n5, assign15800_e22111_d_b0, assign15800_e22111_d_b1, assign15800_e22111_d_b2, assign15800_e22111_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) && (var_guard249 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign15800_e22111;
        var_dnj1_dv_dn0 = assign15800_e22111_d_n0;
        var_dnj1_dv_dn1 = assign15800_e22111_d_n1;
        var_dnj1_dv_dn2 = assign15800_e22111_d_n2;
        var_dnj1_dv_dn3 = assign15800_e22111_d_n3;
        var_dnj1_dv_dn4 = assign15800_e22111_d_n4;
        var_dnj1_dv_dn5 = assign15800_e22111_d_n5;
        var_dnj1_dv_db0 = assign15800_e22111_d_b0;
        var_dnj1_dv_db1 = assign15800_e22111_d_b1;
        var_dnj1_dv_db2 = assign15800_e22111_d_b2;
        var_dnj1_dv_db3 = assign15800_e22111_d_b3;

        let (assign15860_e22360, assign15860_e22360_d_n0, assign15860_e22360_d_n1, assign15860_e22360_d_n2, assign15860_e22360_d_n3, assign15860_e22360_d_n4, assign15860_e22360_d_n5, assign15860_e22360_d_b0, assign15860_e22360_d_b1, assign15860_e22360_d_b2, assign15860_e22360_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard230 != 0.0)) && (var_guard231 == 0.0)) {
        let assign15860_e22344: f64 = (var_vmax * var_dnj1_dv);
        let assign15860_e22345: f64 = (var_nj1 - assign15860_e22344);
        let assign15860_e22348: f64 = (var_nj1 * var_nj1);
        let assign15860_e22349: f64 = (assign15860_e22345 / assign15860_e22348);
        let assign15860_e22352: f64 = (var_vha1 * var_dnj1_dv);
        let assign15860_e22355: f64 = (var_nj0 * p.p85);
        let assign15860_e22356: f64 = (assign15860_e22352 / assign15860_e22355);
        let assign15860_e22357: f64 = (assign15860_e22349 + assign15860_e22356);
        let assign15860_e22358: f64 = (var_phitdinv * assign15860_e22357);
        (assign15860_e22358, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn0 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn1 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn2 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn3 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn4 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_dn5 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_db0) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_db0 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_db1) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_db1 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_db2) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_db2 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign15860_e22348) - (assign15860_e22345 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign15860_e22348 * assign15860_e22348)) + ((((var_vha1 * var_dnj1_dv_db3) * assign15860_e22355) - (assign15860_e22352 * (var_nj0_db3 * p.p85))) / (assign15860_e22355 * assign15860_e22355)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign15860_e22360;
        var_dvmax_over_phitd_dv_dn0 = assign15860_e22360_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign15860_e22360_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign15860_e22360_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign15860_e22360_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign15860_e22360_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign15860_e22360_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign15860_e22360_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign15860_e22360_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign15860_e22360_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign15860_e22360_d_b3;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        var_ab_i: f64,
        var_guard230: f64,
        var_guard31: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_vmax: f64,
        var_guard307_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign15880_e22385, assign15880_e22385_d_n0, assign15880_e22385_d_n1, assign15880_e22385_d_n2, assign15880_e22385_d_n3, assign15880_e22385_d_n4, assign15880_e22385_d_n5, assign15880_e22385_d_b0, assign15880_e22385_d_b1, assign15880_e22385_d_b2, assign15880_e22385_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard230 != 0.0)) {
        let assign15880_e22383: f64 = (var_idmultbot - 1.0);
        (assign15880_e22383, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign15880_e22385;
        var_idmultbot_dn0 = assign15880_e22385_d_n0;
        var_idmultbot_dn1 = assign15880_e22385_d_n1;
        var_idmultbot_dn2 = assign15880_e22385_d_n2;
        var_idmultbot_dn3 = assign15880_e22385_d_n3;
        var_idmultbot_dn4 = assign15880_e22385_d_n4;
        var_idmultbot_dn5 = assign15880_e22385_d_n5;
        var_idmultbot_db0 = assign15880_e22385_d_b0;
        var_idmultbot_db1 = assign15880_e22385_d_b1;
        var_idmultbot_db2 = assign15880_e22385_d_b2;
        var_idmultbot_db3 = assign15880_e22385_d_b3;

        let (assign15990_e22558, assign15990_e22558_d_n0, assign15990_e22558_d_n1, assign15990_e22558_d_n2, assign15990_e22558_d_n3, assign15990_e22558_d_n4, assign15990_e22558_d_n5, assign15990_e22558_d_b0, assign15990_e22558_d_b1, assign15990_e22558_d_b2, assign15990_e22558_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard230 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign15990_e22558;
        var_idmultbot_dn0 = assign15990_e22558_d_n0;
        var_idmultbot_dn1 = assign15990_e22558_d_n1;
        var_idmultbot_dn2 = assign15990_e22558_d_n2;
        var_idmultbot_dn3 = assign15990_e22558_d_n3;
        var_idmultbot_dn4 = assign15990_e22558_d_n4;
        var_idmultbot_dn5 = assign15990_e22558_d_n5;
        var_idmultbot_db0 = assign15990_e22558_d_b0;
        var_idmultbot_db1 = assign15990_e22558_d_b1;
        var_idmultbot_db2 = assign15990_e22558_d_b2;
        var_idmultbot_db3 = assign15990_e22558_d_b3;

        let assign18520_e26108: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard307 = assign18520_e26108;

        let assign18600_e26180: f64 = if var_v4 < var_vmax { 1.0 } else { 0.0 };
        var_guard308 = assign18600_e26180;

        let (assign18660_e26321,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18660_e26317: f64 = (var_nin * var_nin);
        let assign18660_e26319: f64 = (assign18660_e26317 / var_ndibot_i);
        (assign18660_e26319,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign18660_e26321;

        let (assign18670_e26336,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18670_e26329: f64 = (var_nfabot_i / var_phitdinv);
        let assign18670_e26332: f64 = (var_ndibot_i / var_pnn0);
        let assign18670_e26333: f64 = (assign18670_e26332).ln();
        let assign18670_e26334: f64 = (assign18670_e26329 * assign18670_e26333);
        (assign18670_e26334,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign18670_e26336;

        let assign18680_e26339: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard311 = assign18680_e26339;

        let (assign18690_e26355, assign18690_e26355_d_n0, assign18690_e26355_d_n1, assign18690_e26355_d_n2, assign18690_e26355_d_n3, assign18690_e26355_d_n4, assign18690_e26355_d_n5, assign18690_e26355_d_b0, assign18690_e26355_d_b1, assign18690_e26355_d_b2, assign18690_e26355_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18690_e26350: f64 = (var_v4 - var_vha1);
        let assign18690_e26351: f64 = (p.p86 * assign18690_e26350);
        let assign18690_e26353: f64 = (assign18690_e26351 + var_nfabot_i);
        (assign18690_e26353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign18690_e26355;
        var_nja10_dn0 = assign18690_e26355_d_n0;
        var_nja10_dn1 = assign18690_e26355_d_n1;
        var_nja10_dn2 = assign18690_e26355_d_n2;
        var_nja10_dn3 = assign18690_e26355_d_n3;
        var_nja10_dn4 = assign18690_e26355_d_n4;
        var_nja10_dn5 = assign18690_e26355_d_n5;
        var_nja10_db0 = assign18690_e26355_d_b0;
        var_nja10_db1 = assign18690_e26355_d_b1;
        var_nja10_db2 = assign18690_e26355_d_b2;
        var_nja10_db3 = assign18690_e26355_d_b3;

        let (assign18700_e26369, assign18700_e26369_d_n0, assign18700_e26369_d_n1, assign18700_e26369_d_n2, assign18700_e26369_d_n3, assign18700_e26369_d_n4, assign18700_e26369_d_n5, assign18700_e26369_d_b0, assign18700_e26369_d_b1, assign18700_e26369_d_b2, assign18700_e26369_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18700_e26366: f64 = (p.p86 * var_vha1);
        let assign18700_e26367: f64 = (var_nfabot_i - assign18700_e26366);
        (assign18700_e26367, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign18700_e26369;
        var_nj0_dn0 = assign18700_e26369_d_n0;
        var_nj0_dn1 = assign18700_e26369_d_n1;
        var_nj0_dn2 = assign18700_e26369_d_n2;
        var_nj0_dn3 = assign18700_e26369_d_n3;
        var_nj0_dn4 = assign18700_e26369_d_n4;
        var_nj0_dn5 = assign18700_e26369_d_n5;
        var_nj0_db0 = assign18700_e26369_d_b0;
        var_nj0_db1 = assign18700_e26369_d_b1;
        var_nj0_db2 = assign18700_e26369_d_b2;
        var_nj0_db3 = assign18700_e26369_d_b3;

        let (assign18710_e26383, assign18710_e26383_d_n0, assign18710_e26383_d_n1, assign18710_e26383_d_n2, assign18710_e26383_d_n3, assign18710_e26383_d_n4, assign18710_e26383_d_n5, assign18710_e26383_d_b0, assign18710_e26383_d_b1, assign18710_e26383_d_b2, assign18710_e26383_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18710_e26379: f64 = (p.p85 - var_nja10);
        let assign18710_e26381: f64 = (assign18710_e26379 - 0.01);
        (assign18710_e26381, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign18710_e26383;
        var_tmf1_dn0 = assign18710_e26383_d_n0;
        var_tmf1_dn1 = assign18710_e26383_d_n1;
        var_tmf1_dn2 = assign18710_e26383_d_n2;
        var_tmf1_dn3 = assign18710_e26383_d_n3;
        var_tmf1_dn4 = assign18710_e26383_d_n4;
        var_tmf1_dn5 = assign18710_e26383_d_n5;
        var_tmf1_db0 = assign18710_e26383_d_b0;
        var_tmf1_db1 = assign18710_e26383_d_b1;
        var_tmf1_db2 = assign18710_e26383_d_b2;
        var_tmf1_db3 = assign18710_e26383_d_b3;

        let (assign18720_e26397, assign18720_e26397_d_n0, assign18720_e26397_d_n1, assign18720_e26397_d_n2, assign18720_e26397_d_n3, assign18720_e26397_d_n4, assign18720_e26397_d_n5, assign18720_e26397_d_b0, assign18720_e26397_d_b1, assign18720_e26397_d_b2, assign18720_e26397_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18720_e26393: f64 = (4.0 * p.p85);
        let assign18720_e26395: f64 = (assign18720_e26393 * 0.01);
        (assign18720_e26395, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18720_e26397;
        var_tmf2_dn0 = assign18720_e26397_d_n0;
        var_tmf2_dn1 = assign18720_e26397_d_n1;
        var_tmf2_dn2 = assign18720_e26397_d_n2;
        var_tmf2_dn3 = assign18720_e26397_d_n3;
        var_tmf2_dn4 = assign18720_e26397_d_n4;
        var_tmf2_dn5 = assign18720_e26397_d_n5;
        var_tmf2_db0 = assign18720_e26397_d_b0;
        var_tmf2_db1 = assign18720_e26397_d_b1;
        var_tmf2_db2 = assign18720_e26397_d_b2;
        var_tmf2_db3 = assign18720_e26397_d_b3;

        let (assign18730_e26413, assign18730_e26413_d_n0, assign18730_e26413_d_n1, assign18730_e26413_d_n2, assign18730_e26413_d_n3, assign18730_e26413_d_n4, assign18730_e26413_d_n5, assign18730_e26413_d_b0, assign18730_e26413_d_b1, assign18730_e26413_d_b2, assign18730_e26413_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n1, assign18730_e26411_d_n2, assign18730_e26411_d_n3, assign18730_e26411_d_n4, assign18730_e26411_d_n5, assign18730_e26411_d_b0, assign18730_e26411_d_b1, assign18730_e26411_d_b2, assign18730_e26411_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign18730_e26410: f64 = (-var_tmf2);
                (assign18730_e26410, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign18730_e26411, assign18730_e26411_d_n0, assign18730_e26411_d_n1, assign18730_e26411_d_n2, assign18730_e26411_d_n3, assign18730_e26411_d_n4, assign18730_e26411_d_n5, assign18730_e26411_d_b0, assign18730_e26411_d_b1, assign18730_e26411_d_b2, assign18730_e26411_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18730_e26413;
        var_tmf2_dn0 = assign18730_e26413_d_n0;
        var_tmf2_dn1 = assign18730_e26413_d_n1;
        var_tmf2_dn2 = assign18730_e26413_d_n2;
        var_tmf2_dn3 = assign18730_e26413_d_n3;
        var_tmf2_dn4 = assign18730_e26413_d_n4;
        var_tmf2_dn5 = assign18730_e26413_d_n5;
        var_tmf2_db0 = assign18730_e26413_d_b0;
        var_tmf2_db1 = assign18730_e26413_d_b1;
        var_tmf2_db2 = assign18730_e26413_d_b2;
        var_tmf2_db3 = assign18730_e26413_d_b3;

        let (assign18740_e26428, assign18740_e26428_d_n0, assign18740_e26428_d_n1, assign18740_e26428_d_n2, assign18740_e26428_d_n3, assign18740_e26428_d_n4, assign18740_e26428_d_n5, assign18740_e26428_d_b0, assign18740_e26428_d_b1, assign18740_e26428_d_b2, assign18740_e26428_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18740_e26423: f64 = (var_tmf1 * var_tmf1);
        let assign18740_e26425: f64 = (assign18740_e26423 + var_tmf2);
        let assign18740_e26426: f64 = (assign18740_e26425).sqrt();
        (assign18740_e26426, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign18740_e26426)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign18740_e26426)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign18740_e26426)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign18740_e26426)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign18740_e26426)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign18740_e26426)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18740_e26428;
        var_tmf2_dn0 = assign18740_e26428_d_n0;
        var_tmf2_dn1 = assign18740_e26428_d_n1;
        var_tmf2_dn2 = assign18740_e26428_d_n2;
        var_tmf2_dn3 = assign18740_e26428_d_n3;
        var_tmf2_dn4 = assign18740_e26428_d_n4;
        var_tmf2_dn5 = assign18740_e26428_d_n5;
        var_tmf2_db0 = assign18740_e26428_d_b0;
        var_tmf2_db1 = assign18740_e26428_d_b1;
        var_tmf2_db2 = assign18740_e26428_d_b2;
        var_tmf2_db3 = assign18740_e26428_d_b3;

        let (assign18750_e26444, assign18750_e26444_d_n0, assign18750_e26444_d_n1, assign18750_e26444_d_n2, assign18750_e26444_d_n3, assign18750_e26444_d_n4, assign18750_e26444_d_n5, assign18750_e26444_d_b0, assign18750_e26444_d_b1, assign18750_e26444_d_b2, assign18750_e26444_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18750_e26440: f64 = (var_tmf1 + var_tmf2);
        let assign18750_e26441: f64 = (0.5 * assign18750_e26440);
        let assign18750_e26442: f64 = (p.p85 - assign18750_e26441);
        (assign18750_e26442, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign18750_e26444;
        var_nja11_dn0 = assign18750_e26444_d_n0;
        var_nja11_dn1 = assign18750_e26444_d_n1;
        var_nja11_dn2 = assign18750_e26444_d_n2;
        var_nja11_dn3 = assign18750_e26444_d_n3;
        var_nja11_dn4 = assign18750_e26444_d_n4;
        var_nja11_dn5 = assign18750_e26444_d_n5;
        var_nja11_db0 = assign18750_e26444_d_b0;
        var_nja11_db1 = assign18750_e26444_d_b1;
        var_nja11_db2 = assign18750_e26444_d_b2;
        var_nja11_db3 = assign18750_e26444_d_b3;

        let (assign18760_e26458, assign18760_e26458_d_n0, assign18760_e26458_d_n1, assign18760_e26458_d_n2, assign18760_e26458_d_n3, assign18760_e26458_d_n4, assign18760_e26458_d_n5, assign18760_e26458_d_b0, assign18760_e26458_d_b1, assign18760_e26458_d_b2, assign18760_e26458_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18760_e26454: f64 = (var_nja11 - var_nfabot_i);
        let assign18760_e26456: f64 = (assign18760_e26454 - 0.01);
        (assign18760_e26456, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign18760_e26458;
        var_tmf1_dn0 = assign18760_e26458_d_n0;
        var_tmf1_dn1 = assign18760_e26458_d_n1;
        var_tmf1_dn2 = assign18760_e26458_d_n2;
        var_tmf1_dn3 = assign18760_e26458_d_n3;
        var_tmf1_dn4 = assign18760_e26458_d_n4;
        var_tmf1_dn5 = assign18760_e26458_d_n5;
        var_tmf1_db0 = assign18760_e26458_d_b0;
        var_tmf1_db1 = assign18760_e26458_d_b1;
        var_tmf1_db2 = assign18760_e26458_d_b2;
        var_tmf1_db3 = assign18760_e26458_d_b3;

        let (assign18770_e26472, assign18770_e26472_d_n0, assign18770_e26472_d_n1, assign18770_e26472_d_n2, assign18770_e26472_d_n3, assign18770_e26472_d_n4, assign18770_e26472_d_n5, assign18770_e26472_d_b0, assign18770_e26472_d_b1, assign18770_e26472_d_b2, assign18770_e26472_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18770_e26468: f64 = (4.0 * var_nfabot_i);
        let assign18770_e26470: f64 = (assign18770_e26468 * 0.01);
        (assign18770_e26470, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18770_e26472;
        var_tmf2_dn0 = assign18770_e26472_d_n0;
        var_tmf2_dn1 = assign18770_e26472_d_n1;
        var_tmf2_dn2 = assign18770_e26472_d_n2;
        var_tmf2_dn3 = assign18770_e26472_d_n3;
        var_tmf2_dn4 = assign18770_e26472_d_n4;
        var_tmf2_dn5 = assign18770_e26472_d_n5;
        var_tmf2_db0 = assign18770_e26472_d_b0;
        var_tmf2_db1 = assign18770_e26472_d_b1;
        var_tmf2_db2 = assign18770_e26472_d_b2;
        var_tmf2_db3 = assign18770_e26472_d_b3;

        let (assign18780_e26488, assign18780_e26488_d_n0, assign18780_e26488_d_n1, assign18780_e26488_d_n2, assign18780_e26488_d_n3, assign18780_e26488_d_n4, assign18780_e26488_d_n5, assign18780_e26488_d_b0, assign18780_e26488_d_b1, assign18780_e26488_d_b2, assign18780_e26488_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n1, assign18780_e26486_d_n2, assign18780_e26486_d_n3, assign18780_e26486_d_n4, assign18780_e26486_d_n5, assign18780_e26486_d_b0, assign18780_e26486_d_b1, assign18780_e26486_d_b2, assign18780_e26486_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign18780_e26485: f64 = (-var_tmf2);
                (assign18780_e26485, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign18780_e26486, assign18780_e26486_d_n0, assign18780_e26486_d_n1, assign18780_e26486_d_n2, assign18780_e26486_d_n3, assign18780_e26486_d_n4, assign18780_e26486_d_n5, assign18780_e26486_d_b0, assign18780_e26486_d_b1, assign18780_e26486_d_b2, assign18780_e26486_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18780_e26488;
        var_tmf2_dn0 = assign18780_e26488_d_n0;
        var_tmf2_dn1 = assign18780_e26488_d_n1;
        var_tmf2_dn2 = assign18780_e26488_d_n2;
        var_tmf2_dn3 = assign18780_e26488_d_n3;
        var_tmf2_dn4 = assign18780_e26488_d_n4;
        var_tmf2_dn5 = assign18780_e26488_d_n5;
        var_tmf2_db0 = assign18780_e26488_d_b0;
        var_tmf2_db1 = assign18780_e26488_d_b1;
        var_tmf2_db2 = assign18780_e26488_d_b2;
        var_tmf2_db3 = assign18780_e26488_d_b3;

        let (assign18790_e26503, assign18790_e26503_d_n0, assign18790_e26503_d_n1, assign18790_e26503_d_n2, assign18790_e26503_d_n3, assign18790_e26503_d_n4, assign18790_e26503_d_n5, assign18790_e26503_d_b0, assign18790_e26503_d_b1, assign18790_e26503_d_b2, assign18790_e26503_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18790_e26498: f64 = (var_tmf1 * var_tmf1);
        let assign18790_e26500: f64 = (assign18790_e26498 + var_tmf2);
        let assign18790_e26501: f64 = (assign18790_e26500).sqrt();
        (assign18790_e26501, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign18790_e26501)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign18790_e26501)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign18790_e26501)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign18790_e26501)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign18790_e26501)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign18790_e26501)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18790_e26503;
        var_tmf2_dn0 = assign18790_e26503_d_n0;
        var_tmf2_dn1 = assign18790_e26503_d_n1;
        var_tmf2_dn2 = assign18790_e26503_d_n2;
        var_tmf2_dn3 = assign18790_e26503_d_n3;
        var_tmf2_dn4 = assign18790_e26503_d_n4;
        var_tmf2_dn5 = assign18790_e26503_d_n5;
        var_tmf2_db0 = assign18790_e26503_d_b0;
        var_tmf2_db1 = assign18790_e26503_d_b1;
        var_tmf2_db2 = assign18790_e26503_d_b2;
        var_tmf2_db3 = assign18790_e26503_d_b3;

        let (assign18800_e26519, assign18800_e26519_d_n0, assign18800_e26519_d_n1, assign18800_e26519_d_n2, assign18800_e26519_d_n3, assign18800_e26519_d_n4, assign18800_e26519_d_n5, assign18800_e26519_d_b0, assign18800_e26519_d_b1, assign18800_e26519_d_b2, assign18800_e26519_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18800_e26515: f64 = (var_tmf1 + var_tmf2);
        let assign18800_e26516: f64 = (0.5 * assign18800_e26515);
        let assign18800_e26517: f64 = (var_nfabot_i + assign18800_e26516);
        (assign18800_e26517, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign18800_e26519;
        var_nj1_dn0 = assign18800_e26519_d_n0;
        var_nj1_dn1 = assign18800_e26519_d_n1;
        var_nj1_dn2 = assign18800_e26519_d_n2;
        var_nj1_dn3 = assign18800_e26519_d_n3;
        var_nj1_dn4 = assign18800_e26519_d_n4;
        var_nj1_dn5 = assign18800_e26519_d_n5;
        var_nj1_db0 = assign18800_e26519_d_b0;
        var_nj1_db1 = assign18800_e26519_d_b1;
        var_nj1_db2 = assign18800_e26519_d_b2;
        var_nj1_db3 = assign18800_e26519_d_b3;

        let (assign18810_e26533, assign18810_e26533_d_n0, assign18810_e26533_d_n1, assign18810_e26533_d_n2, assign18810_e26533_d_n3, assign18810_e26533_d_n4, assign18810_e26533_d_n5, assign18810_e26533_d_b0, assign18810_e26533_d_b1, assign18810_e26533_d_b2, assign18810_e26533_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18810_e26529: f64 = (p.p85 - var_nj0);
        let assign18810_e26531: f64 = (assign18810_e26529 - 0.01);
        (assign18810_e26531, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign18810_e26533;
        var_tmf1_dn0 = assign18810_e26533_d_n0;
        var_tmf1_dn1 = assign18810_e26533_d_n1;
        var_tmf1_dn2 = assign18810_e26533_d_n2;
        var_tmf1_dn3 = assign18810_e26533_d_n3;
        var_tmf1_dn4 = assign18810_e26533_d_n4;
        var_tmf1_dn5 = assign18810_e26533_d_n5;
        var_tmf1_db0 = assign18810_e26533_d_b0;
        var_tmf1_db1 = assign18810_e26533_d_b1;
        var_tmf1_db2 = assign18810_e26533_d_b2;
        var_tmf1_db3 = assign18810_e26533_d_b3;

        let (assign18820_e26547, assign18820_e26547_d_n0, assign18820_e26547_d_n1, assign18820_e26547_d_n2, assign18820_e26547_d_n3, assign18820_e26547_d_n4, assign18820_e26547_d_n5, assign18820_e26547_d_b0, assign18820_e26547_d_b1, assign18820_e26547_d_b2, assign18820_e26547_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18820_e26543: f64 = (4.0 * p.p85);
        let assign18820_e26545: f64 = (assign18820_e26543 * 0.01);
        (assign18820_e26545, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18820_e26547;
        var_tmf2_dn0 = assign18820_e26547_d_n0;
        var_tmf2_dn1 = assign18820_e26547_d_n1;
        var_tmf2_dn2 = assign18820_e26547_d_n2;
        var_tmf2_dn3 = assign18820_e26547_d_n3;
        var_tmf2_dn4 = assign18820_e26547_d_n4;
        var_tmf2_dn5 = assign18820_e26547_d_n5;
        var_tmf2_db0 = assign18820_e26547_d_b0;
        var_tmf2_db1 = assign18820_e26547_d_b1;
        var_tmf2_db2 = assign18820_e26547_d_b2;
        var_tmf2_db3 = assign18820_e26547_d_b3;

        let (assign18830_e26563, assign18830_e26563_d_n0, assign18830_e26563_d_n1, assign18830_e26563_d_n2, assign18830_e26563_d_n3, assign18830_e26563_d_n4, assign18830_e26563_d_n5, assign18830_e26563_d_b0, assign18830_e26563_d_b1, assign18830_e26563_d_b2, assign18830_e26563_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n1, assign18830_e26561_d_n2, assign18830_e26561_d_n3, assign18830_e26561_d_n4, assign18830_e26561_d_n5, assign18830_e26561_d_b0, assign18830_e26561_d_b1, assign18830_e26561_d_b2, assign18830_e26561_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign18830_e26560: f64 = (-var_tmf2);
                (assign18830_e26560, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign18830_e26561, assign18830_e26561_d_n0, assign18830_e26561_d_n1, assign18830_e26561_d_n2, assign18830_e26561_d_n3, assign18830_e26561_d_n4, assign18830_e26561_d_n5, assign18830_e26561_d_b0, assign18830_e26561_d_b1, assign18830_e26561_d_b2, assign18830_e26561_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18830_e26563;
        var_tmf2_dn0 = assign18830_e26563_d_n0;
        var_tmf2_dn1 = assign18830_e26563_d_n1;
        var_tmf2_dn2 = assign18830_e26563_d_n2;
        var_tmf2_dn3 = assign18830_e26563_d_n3;
        var_tmf2_dn4 = assign18830_e26563_d_n4;
        var_tmf2_dn5 = assign18830_e26563_d_n5;
        var_tmf2_db0 = assign18830_e26563_d_b0;
        var_tmf2_db1 = assign18830_e26563_d_b1;
        var_tmf2_db2 = assign18830_e26563_d_b2;
        var_tmf2_db3 = assign18830_e26563_d_b3;

        let (assign18840_e26578, assign18840_e26578_d_n0, assign18840_e26578_d_n1, assign18840_e26578_d_n2, assign18840_e26578_d_n3, assign18840_e26578_d_n4, assign18840_e26578_d_n5, assign18840_e26578_d_b0, assign18840_e26578_d_b1, assign18840_e26578_d_b2, assign18840_e26578_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18840_e26573: f64 = (var_tmf1 * var_tmf1);
        let assign18840_e26575: f64 = (assign18840_e26573 + var_tmf2);
        let assign18840_e26576: f64 = (assign18840_e26575).sqrt();
        (assign18840_e26576, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign18840_e26576)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign18840_e26576)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign18840_e26576)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign18840_e26576)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign18840_e26576)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign18840_e26576)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18840_e26578;
        var_tmf2_dn0 = assign18840_e26578_d_n0;
        var_tmf2_dn1 = assign18840_e26578_d_n1;
        var_tmf2_dn2 = assign18840_e26578_d_n2;
        var_tmf2_dn3 = assign18840_e26578_d_n3;
        var_tmf2_dn4 = assign18840_e26578_d_n4;
        var_tmf2_dn5 = assign18840_e26578_d_n5;
        var_tmf2_db0 = assign18840_e26578_d_b0;
        var_tmf2_db1 = assign18840_e26578_d_b1;
        var_tmf2_db2 = assign18840_e26578_d_b2;
        var_tmf2_db3 = assign18840_e26578_d_b3;

        let (assign18850_e26594, assign18850_e26594_d_n0, assign18850_e26594_d_n1, assign18850_e26594_d_n2, assign18850_e26594_d_n3, assign18850_e26594_d_n4, assign18850_e26594_d_n5, assign18850_e26594_d_b0, assign18850_e26594_d_b1, assign18850_e26594_d_b2, assign18850_e26594_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18850_e26590: f64 = (var_tmf1 + var_tmf2);
        let assign18850_e26591: f64 = (0.5 * assign18850_e26590);
        let assign18850_e26592: f64 = (p.p85 - assign18850_e26591);
        (assign18850_e26592, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign18850_e26594;
        var_nj0_dn0 = assign18850_e26594_d_n0;
        var_nj0_dn1 = assign18850_e26594_d_n1;
        var_nj0_dn2 = assign18850_e26594_d_n2;
        var_nj0_dn3 = assign18850_e26594_d_n3;
        var_nj0_dn4 = assign18850_e26594_d_n4;
        var_nj0_dn5 = assign18850_e26594_d_n5;
        var_nj0_db0 = assign18850_e26594_d_b0;
        var_nj0_db1 = assign18850_e26594_d_b1;
        var_nj0_db2 = assign18850_e26594_d_b2;
        var_nj0_db3 = assign18850_e26594_d_b3;

        let (assign18860_e26608, assign18860_e26608_d_n0, assign18860_e26608_d_n1, assign18860_e26608_d_n2, assign18860_e26608_d_n3, assign18860_e26608_d_n4, assign18860_e26608_d_n5, assign18860_e26608_d_b0, assign18860_e26608_d_b1, assign18860_e26608_d_b2, assign18860_e26608_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18860_e26604: f64 = (var_nj0 - var_nfabot_i);
        let assign18860_e26606: f64 = (assign18860_e26604 - 0.01);
        (assign18860_e26606, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign18860_e26608;
        var_tmf1_dn0 = assign18860_e26608_d_n0;
        var_tmf1_dn1 = assign18860_e26608_d_n1;
        var_tmf1_dn2 = assign18860_e26608_d_n2;
        var_tmf1_dn3 = assign18860_e26608_d_n3;
        var_tmf1_dn4 = assign18860_e26608_d_n4;
        var_tmf1_dn5 = assign18860_e26608_d_n5;
        var_tmf1_db0 = assign18860_e26608_d_b0;
        var_tmf1_db1 = assign18860_e26608_d_b1;
        var_tmf1_db2 = assign18860_e26608_d_b2;
        var_tmf1_db3 = assign18860_e26608_d_b3;

        let (assign18870_e26622, assign18870_e26622_d_n0, assign18870_e26622_d_n1, assign18870_e26622_d_n2, assign18870_e26622_d_n3, assign18870_e26622_d_n4, assign18870_e26622_d_n5, assign18870_e26622_d_b0, assign18870_e26622_d_b1, assign18870_e26622_d_b2, assign18870_e26622_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18870_e26618: f64 = (4.0 * var_nfabot_i);
        let assign18870_e26620: f64 = (assign18870_e26618 * 0.01);
        (assign18870_e26620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18870_e26622;
        var_tmf2_dn0 = assign18870_e26622_d_n0;
        var_tmf2_dn1 = assign18870_e26622_d_n1;
        var_tmf2_dn2 = assign18870_e26622_d_n2;
        var_tmf2_dn3 = assign18870_e26622_d_n3;
        var_tmf2_dn4 = assign18870_e26622_d_n4;
        var_tmf2_dn5 = assign18870_e26622_d_n5;
        var_tmf2_db0 = assign18870_e26622_d_b0;
        var_tmf2_db1 = assign18870_e26622_d_b1;
        var_tmf2_db2 = assign18870_e26622_d_b2;
        var_tmf2_db3 = assign18870_e26622_d_b3;

        *var_guard307_slot = var_guard307;
        *var_guard308_slot = var_guard308;
        *var_guard311_slot = var_guard311;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard311: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_guard312_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign18880_e26638, assign18880_e26638_d_n0, assign18880_e26638_d_n1, assign18880_e26638_d_n2, assign18880_e26638_d_n3, assign18880_e26638_d_n4, assign18880_e26638_d_n5, assign18880_e26638_d_b0, assign18880_e26638_d_b1, assign18880_e26638_d_b2, assign18880_e26638_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n1, assign18880_e26636_d_n2, assign18880_e26636_d_n3, assign18880_e26636_d_n4, assign18880_e26636_d_n5, assign18880_e26636_d_b0, assign18880_e26636_d_b1, assign18880_e26636_d_b2, assign18880_e26636_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign18880_e26635: f64 = (-var_tmf2);
                (assign18880_e26635, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign18880_e26636, assign18880_e26636_d_n0, assign18880_e26636_d_n1, assign18880_e26636_d_n2, assign18880_e26636_d_n3, assign18880_e26636_d_n4, assign18880_e26636_d_n5, assign18880_e26636_d_b0, assign18880_e26636_d_b1, assign18880_e26636_d_b2, assign18880_e26636_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18880_e26638;
        var_tmf2_dn0 = assign18880_e26638_d_n0;
        var_tmf2_dn1 = assign18880_e26638_d_n1;
        var_tmf2_dn2 = assign18880_e26638_d_n2;
        var_tmf2_dn3 = assign18880_e26638_d_n3;
        var_tmf2_dn4 = assign18880_e26638_d_n4;
        var_tmf2_dn5 = assign18880_e26638_d_n5;
        var_tmf2_db0 = assign18880_e26638_d_b0;
        var_tmf2_db1 = assign18880_e26638_d_b1;
        var_tmf2_db2 = assign18880_e26638_d_b2;
        var_tmf2_db3 = assign18880_e26638_d_b3;

        let (assign18890_e26653, assign18890_e26653_d_n0, assign18890_e26653_d_n1, assign18890_e26653_d_n2, assign18890_e26653_d_n3, assign18890_e26653_d_n4, assign18890_e26653_d_n5, assign18890_e26653_d_b0, assign18890_e26653_d_b1, assign18890_e26653_d_b2, assign18890_e26653_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18890_e26648: f64 = (var_tmf1 * var_tmf1);
        let assign18890_e26650: f64 = (assign18890_e26648 + var_tmf2);
        let assign18890_e26651: f64 = (assign18890_e26650).sqrt();
        (assign18890_e26651, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign18890_e26651)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign18890_e26651)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign18890_e26651)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign18890_e26651)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign18890_e26651)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign18890_e26651)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign18890_e26653;
        var_tmf2_dn0 = assign18890_e26653_d_n0;
        var_tmf2_dn1 = assign18890_e26653_d_n1;
        var_tmf2_dn2 = assign18890_e26653_d_n2;
        var_tmf2_dn3 = assign18890_e26653_d_n3;
        var_tmf2_dn4 = assign18890_e26653_d_n4;
        var_tmf2_dn5 = assign18890_e26653_d_n5;
        var_tmf2_db0 = assign18890_e26653_d_b0;
        var_tmf2_db1 = assign18890_e26653_d_b1;
        var_tmf2_db2 = assign18890_e26653_d_b2;
        var_tmf2_db3 = assign18890_e26653_d_b3;

        let (assign18900_e26669, assign18900_e26669_d_n0, assign18900_e26669_d_n1, assign18900_e26669_d_n2, assign18900_e26669_d_n3, assign18900_e26669_d_n4, assign18900_e26669_d_n5, assign18900_e26669_d_b0, assign18900_e26669_d_b1, assign18900_e26669_d_b2, assign18900_e26669_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 != 0.0)) {
        let assign18900_e26665: f64 = (var_tmf1 + var_tmf2);
        let assign18900_e26666: f64 = (0.5 * assign18900_e26665);
        let assign18900_e26667: f64 = (var_nfabot_i + assign18900_e26666);
        (assign18900_e26667, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign18900_e26669;
        var_nj0_dn0 = assign18900_e26669_d_n0;
        var_nj0_dn1 = assign18900_e26669_d_n1;
        var_nj0_dn2 = assign18900_e26669_d_n2;
        var_nj0_dn3 = assign18900_e26669_d_n3;
        var_nj0_dn4 = assign18900_e26669_d_n4;
        var_nj0_dn5 = assign18900_e26669_d_n5;
        var_nj0_db0 = assign18900_e26669_d_b0;
        var_nj0_db1 = assign18900_e26669_d_b1;
        var_nj0_db2 = assign18900_e26669_d_b2;
        var_nj0_db3 = assign18900_e26669_d_b3;

        let (assign18910_e26680, assign18910_e26680_d_n0, assign18910_e26680_d_n1, assign18910_e26680_d_n2, assign18910_e26680_d_n3, assign18910_e26680_d_n4, assign18910_e26680_d_n5, assign18910_e26680_d_b0, assign18910_e26680_d_b1, assign18910_e26680_d_b2, assign18910_e26680_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign18910_e26680;
        var_nj0_dn0 = assign18910_e26680_d_n0;
        var_nj0_dn1 = assign18910_e26680_d_n1;
        var_nj0_dn2 = assign18910_e26680_d_n2;
        var_nj0_dn3 = assign18910_e26680_d_n3;
        var_nj0_dn4 = assign18910_e26680_d_n4;
        var_nj0_dn5 = assign18910_e26680_d_n5;
        var_nj0_db0 = assign18910_e26680_d_b0;
        var_nj0_db1 = assign18910_e26680_d_b1;
        var_nj0_db2 = assign18910_e26680_d_b2;
        var_nj0_db3 = assign18910_e26680_d_b3;

        let (assign18920_e26691, assign18920_e26691_d_n0, assign18920_e26691_d_n1, assign18920_e26691_d_n2, assign18920_e26691_d_n3, assign18920_e26691_d_n4, assign18920_e26691_d_n5, assign18920_e26691_d_b0, assign18920_e26691_d_b1, assign18920_e26691_d_b2, assign18920_e26691_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard311 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign18920_e26691;
        var_nj1_dn0 = assign18920_e26691_d_n0;
        var_nj1_dn1 = assign18920_e26691_d_n1;
        var_nj1_dn2 = assign18920_e26691_d_n2;
        var_nj1_dn3 = assign18920_e26691_d_n3;
        var_nj1_dn4 = assign18920_e26691_d_n4;
        var_nj1_dn5 = assign18920_e26691_d_n5;
        var_nj1_db0 = assign18920_e26691_d_b0;
        var_nj1_db1 = assign18920_e26691_d_b1;
        var_nj1_db2 = assign18920_e26691_d_b2;
        var_nj1_db3 = assign18920_e26691_d_b3;

        let assign18930_e26695: f64 = (var_v4 / var_nj1);
        let assign18930_e26699: f64 = (var_nj1 - var_nj0);
        let assign18930_e26700: f64 = (var_vha1 * assign18930_e26699);
        let assign18930_e26703: f64 = (var_nj0 * p.p85);
        let assign18930_e26704: f64 = (assign18930_e26700 / assign18930_e26703);
        let assign18930_e26705: f64 = (assign18930_e26695 + assign18930_e26704);
        let assign18930_e26706: f64 = (var_phitdinv * assign18930_e26705);
        let assign18930_e26707: f64 = (assign18930_e26706).abs();
        let assign18930_e26709: f64 = if assign18930_e26707 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard312 = assign18930_e26709;

        let (assign18940_e26734, assign18940_e26734_d_n0, assign18940_e26734_d_n1, assign18940_e26734_d_n2, assign18940_e26734_d_n3, assign18940_e26734_d_n4, assign18940_e26734_d_n5, assign18940_e26734_d_b0, assign18940_e26734_d_b1, assign18940_e26734_d_b2, assign18940_e26734_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 != 0.0)) {
        let assign18940_e26720: f64 = (var_v4 / var_nj1);
        let assign18940_e26724: f64 = (var_nj1 - var_nj0);
        let assign18940_e26725: f64 = (var_vha1 * assign18940_e26724);
        let assign18940_e26728: f64 = (var_nj0 * p.p85);
        let assign18940_e26729: f64 = (assign18940_e26725 / assign18940_e26728);
        let assign18940_e26730: f64 = (assign18940_e26720 + assign18940_e26729);
        let assign18940_e26731: f64 = (var_phitdinv * assign18940_e26730);
        let assign18940_e26732: f64 = (assign18940_e26731).exp();
        (assign18940_e26732, (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn0 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn1 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn2 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn3 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn4 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_dn5 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_db0 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_db1 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_db2 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))), (assign18940_e26732 * (var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18940_e26728) - (assign18940_e26725 * (var_nj0_db3 * p.p85))) / (assign18940_e26728 * assign18940_e26728))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign18940_e26734;
        var_idmultbot_dn0 = assign18940_e26734_d_n0;
        var_idmultbot_dn1 = assign18940_e26734_d_n1;
        var_idmultbot_dn2 = assign18940_e26734_d_n2;
        var_idmultbot_dn3 = assign18940_e26734_d_n3;
        var_idmultbot_dn4 = assign18940_e26734_d_n4;
        var_idmultbot_dn5 = assign18940_e26734_d_n5;
        var_idmultbot_db0 = assign18940_e26734_d_b0;
        var_idmultbot_db1 = assign18940_e26734_d_b1;
        var_idmultbot_db2 = assign18940_e26734_d_b2;
        var_idmultbot_db3 = assign18940_e26734_d_b3;

        let assign18950_e26738: f64 = (var_v4 / var_nj1);
        let assign18950_e26742: f64 = (var_nj1 - var_nj0);
        let assign18950_e26743: f64 = (var_vha1 * assign18950_e26742);
        let assign18950_e26746: f64 = (var_nj0 * p.p85);
        let assign18950_e26747: f64 = (assign18950_e26743 / assign18950_e26746);
        let assign18950_e26748: f64 = (assign18950_e26738 + assign18950_e26747);
        let assign18950_e26749: f64 = (var_phitdinv * assign18950_e26748);
        let assign18950_e26751: f64 = (-230.25850929940458);
        let assign18950_e26752: f64 = if assign18950_e26749 < assign18950_e26751 { 1.0 } else { 0.0 };
        var_guard313 = assign18950_e26752;

        let (assign18960_e26832, assign18960_e26832_d_n0, assign18960_e26832_d_n1, assign18960_e26832_d_n2, assign18960_e26832_d_n3, assign18960_e26832_d_n4, assign18960_e26832_d_n5, assign18960_e26832_d_b0, assign18960_e26832_d_b1, assign18960_e26832_d_b2, assign18960_e26832_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 == 0.0)) && (var_guard313 != 0.0)) {
        let assign18960_e26766: f64 = (-230.25850929940458);
        let assign18960_e26770: f64 = (var_v4 / var_nj1);
        let assign18960_e26774: f64 = (var_nj1 - var_nj0);
        let assign18960_e26775: f64 = (var_vha1 * assign18960_e26774);
        let assign18960_e26778: f64 = (var_nj0 * p.p85);
        let assign18960_e26779: f64 = (assign18960_e26775 / assign18960_e26778);
        let assign18960_e26780: f64 = (assign18960_e26770 + assign18960_e26779);
        let assign18960_e26781: f64 = (var_phitdinv * assign18960_e26780);
        let assign18960_e26782: f64 = (assign18960_e26766 - assign18960_e26781);
        let assign18960_e26786: f64 = (-230.25850929940458);
        let assign18960_e26790: f64 = (var_v4 / var_nj1);
        let assign18960_e26794: f64 = (var_nj1 - var_nj0);
        let assign18960_e26795: f64 = (var_vha1 * assign18960_e26794);
        let assign18960_e26798: f64 = (var_nj0 * p.p85);
        let assign18960_e26799: f64 = (assign18960_e26795 / assign18960_e26798);
        let assign18960_e26800: f64 = (assign18960_e26790 + assign18960_e26799);
        let assign18960_e26801: f64 = (var_phitdinv * assign18960_e26800);
        let assign18960_e26802: f64 = (assign18960_e26786 - assign18960_e26801);
        let assign18960_e26805: f64 = (-230.25850929940458);
        let assign18960_e26809: f64 = (var_v4 / var_nj1);
        let assign18960_e26813: f64 = (var_nj1 - var_nj0);
        let assign18960_e26814: f64 = (var_vha1 * assign18960_e26813);
        let assign18960_e26817: f64 = (var_nj0 * p.p85);
        let assign18960_e26818: f64 = (assign18960_e26814 / assign18960_e26817);
        let assign18960_e26819: f64 = (assign18960_e26809 + assign18960_e26818);
        let assign18960_e26820: f64 = (var_phitdinv * assign18960_e26819);
        let assign18960_e26821: f64 = (assign18960_e26805 - assign18960_e26820);
        let assign18960_e26823: f64 = (assign18960_e26821 * 0.3333333333333333);
        let assign18960_e26824: f64 = (1.0 + assign18960_e26823);
        let assign18960_e26825: f64 = (assign18960_e26802 * assign18960_e26824);
        let assign18960_e26826: f64 = (0.5 * assign18960_e26825);
        let assign18960_e26827: f64 = (1.0 + assign18960_e26826);
        let assign18960_e26828: f64 = (assign18960_e26782 * assign18960_e26827);
        let assign18960_e26829: f64 = (1.0 + assign18960_e26828);
        let assign18960_e26830: f64 = (1e-100 / assign18960_e26829);
        (assign18960_e26830, (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn0 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn0 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn0 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn1 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn1 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn1 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn2 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn2 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn2 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn3 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn3 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn3 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn4 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn4 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn4 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_dn5 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_dn5 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_dn5 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_db0 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_db0 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_db0 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_db1 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_db1 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_db1 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_db2 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_db2 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_db2 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18960_e26778) - (assign18960_e26775 * (var_nj0_db3 * p.p85))) / (assign18960_e26778 * assign18960_e26778))))) * assign18960_e26827) + (assign18960_e26782 * (0.5 * (((-(var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18960_e26798) - (assign18960_e26795 * (var_nj0_db3 * p.p85))) / (assign18960_e26798 * assign18960_e26798))))) * assign18960_e26824) + (assign18960_e26802 * ((-(var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18960_e26817) - (assign18960_e26814 * (var_nj0_db3 * p.p85))) / (assign18960_e26817 * assign18960_e26817))))) * 0.3333333333333333))))))) / (assign18960_e26829 * assign18960_e26829))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign18960_e26832;
        var_idmultbot_dn0 = assign18960_e26832_d_n0;
        var_idmultbot_dn1 = assign18960_e26832_d_n1;
        var_idmultbot_dn2 = assign18960_e26832_d_n2;
        var_idmultbot_dn3 = assign18960_e26832_d_n3;
        var_idmultbot_dn4 = assign18960_e26832_d_n4;
        var_idmultbot_dn5 = assign18960_e26832_d_n5;
        var_idmultbot_db0 = assign18960_e26832_d_b0;
        var_idmultbot_db1 = assign18960_e26832_d_b1;
        var_idmultbot_db2 = assign18960_e26832_d_b2;
        var_idmultbot_db3 = assign18960_e26832_d_b3;

        let (assign18970_e26910, assign18970_e26910_d_n0, assign18970_e26910_d_n1, assign18970_e26910_d_n2, assign18970_e26910_d_n3, assign18970_e26910_d_n4, assign18970_e26910_d_n5, assign18970_e26910_d_b0, assign18970_e26910_d_b1, assign18970_e26910_d_b2, assign18970_e26910_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard312 == 0.0)) && (var_guard313 == 0.0)) {
        let assign18970_e26849: f64 = (var_v4 / var_nj1);
        let assign18970_e26853: f64 = (var_nj1 - var_nj0);
        let assign18970_e26854: f64 = (var_vha1 * assign18970_e26853);
        let assign18970_e26857: f64 = (var_nj0 * p.p85);
        let assign18970_e26858: f64 = (assign18970_e26854 / assign18970_e26857);
        let assign18970_e26859: f64 = (assign18970_e26849 + assign18970_e26858);
        let assign18970_e26860: f64 = (var_phitdinv * assign18970_e26859);
        let assign18970_e26862: f64 = (assign18970_e26860 - 230.25850929940458);
        let assign18970_e26868: f64 = (var_v4 / var_nj1);
        let assign18970_e26872: f64 = (var_nj1 - var_nj0);
        let assign18970_e26873: f64 = (var_vha1 * assign18970_e26872);
        let assign18970_e26876: f64 = (var_nj0 * p.p85);
        let assign18970_e26877: f64 = (assign18970_e26873 / assign18970_e26876);
        let assign18970_e26878: f64 = (assign18970_e26868 + assign18970_e26877);
        let assign18970_e26879: f64 = (var_phitdinv * assign18970_e26878);
        let assign18970_e26881: f64 = (assign18970_e26879 - 230.25850929940458);
        let assign18970_e26886: f64 = (var_v4 / var_nj1);
        let assign18970_e26890: f64 = (var_nj1 - var_nj0);
        let assign18970_e26891: f64 = (var_vha1 * assign18970_e26890);
        let assign18970_e26894: f64 = (var_nj0 * p.p85);
        let assign18970_e26895: f64 = (assign18970_e26891 / assign18970_e26894);
        let assign18970_e26896: f64 = (assign18970_e26886 + assign18970_e26895);
        let assign18970_e26897: f64 = (var_phitdinv * assign18970_e26896);
        let assign18970_e26899: f64 = (assign18970_e26897 - 230.25850929940458);
        let assign18970_e26901: f64 = (assign18970_e26899 * 0.3333333333333333);
        let assign18970_e26902: f64 = (1.0 + assign18970_e26901);
        let assign18970_e26903: f64 = (assign18970_e26881 * assign18970_e26902);
        let assign18970_e26904: f64 = (0.5 * assign18970_e26903);
        let assign18970_e26905: f64 = (1.0 + assign18970_e26904);
        let assign18970_e26906: f64 = (assign18970_e26862 * assign18970_e26905);
        let assign18970_e26907: f64 = (1.0 + assign18970_e26906);
        let assign18970_e26908: f64 = (1e100 * assign18970_e26907);
        (assign18970_e26908, (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn0 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn0 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn0 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn1 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn1 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn1 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn2 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn2 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn2 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn3 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn3 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn3 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn4 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn4 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn4 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_dn5 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_dn5 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_dn5 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_db0 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_db0 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_db0 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_db1 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_db1 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_db1 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_db2 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_db2 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_db2 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18970_e26857) - (assign18970_e26854 * (var_nj0_db3 * p.p85))) / (assign18970_e26857 * assign18970_e26857)))) * assign18970_e26905) + (assign18970_e26862 * (0.5 * (((var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18970_e26876) - (assign18970_e26873 * (var_nj0_db3 * p.p85))) / (assign18970_e26876 * assign18970_e26876)))) * assign18970_e26902) + (assign18970_e26881 * ((var_phitdinv * ((-((var_v4 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign18970_e26894) - (assign18970_e26891 * (var_nj0_db3 * p.p85))) / (assign18970_e26894 * assign18970_e26894)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign18970_e26910;
        var_idmultbot_dn0 = assign18970_e26910_d_n0;
        var_idmultbot_dn1 = assign18970_e26910_d_n1;
        var_idmultbot_dn2 = assign18970_e26910_d_n2;
        var_idmultbot_dn3 = assign18970_e26910_d_n3;
        var_idmultbot_dn4 = assign18970_e26910_d_n4;
        var_idmultbot_dn5 = assign18970_e26910_d_n5;
        var_idmultbot_db0 = assign18970_e26910_d_b0;
        var_idmultbot_db1 = assign18970_e26910_d_b1;
        var_idmultbot_db2 = assign18970_e26910_d_b2;
        var_idmultbot_db3 = assign18970_e26910_d_b3;

        let (assign18980_e26922,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18980_e26918: f64 = (var_nin * var_nin);
        let assign18980_e26920: f64 = (assign18980_e26918 / var_ndisti_i);
        (assign18980_e26920,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign18980_e26922;

        let (assign18990_e26937,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign18990_e26930: f64 = (var_nfasti_i / var_phitdinv);
        let assign18990_e26933: f64 = (var_ndisti_i / var_pnn0);
        let assign18990_e26934: f64 = (assign18990_e26933).ln();
        let assign18990_e26935: f64 = (assign18990_e26930 * assign18990_e26934);
        (assign18990_e26935,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign18990_e26937;

        let assign19000_e26940: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard314 = assign19000_e26940;

        let (assign19010_e26956, assign19010_e26956_d_n0, assign19010_e26956_d_n1, assign19010_e26956_d_n2, assign19010_e26956_d_n3, assign19010_e26956_d_n4, assign19010_e26956_d_n5, assign19010_e26956_d_b0, assign19010_e26956_d_b1, assign19010_e26956_d_b2, assign19010_e26956_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19010_e26951: f64 = (var_v4 - var_vha1);
        let assign19010_e26952: f64 = (p.p86 * assign19010_e26951);
        let assign19010_e26954: f64 = (assign19010_e26952 + var_nfasti_i);
        (assign19010_e26954, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign19010_e26956;
        var_nja10_dn0 = assign19010_e26956_d_n0;
        var_nja10_dn1 = assign19010_e26956_d_n1;
        var_nja10_dn2 = assign19010_e26956_d_n2;
        var_nja10_dn3 = assign19010_e26956_d_n3;
        var_nja10_dn4 = assign19010_e26956_d_n4;
        var_nja10_dn5 = assign19010_e26956_d_n5;
        var_nja10_db0 = assign19010_e26956_d_b0;
        var_nja10_db1 = assign19010_e26956_d_b1;
        var_nja10_db2 = assign19010_e26956_d_b2;
        var_nja10_db3 = assign19010_e26956_d_b3;

        let (assign19020_e26970, assign19020_e26970_d_n0, assign19020_e26970_d_n1, assign19020_e26970_d_n2, assign19020_e26970_d_n3, assign19020_e26970_d_n4, assign19020_e26970_d_n5, assign19020_e26970_d_b0, assign19020_e26970_d_b1, assign19020_e26970_d_b2, assign19020_e26970_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19020_e26967: f64 = (p.p86 * var_vha1);
        let assign19020_e26968: f64 = (var_nfasti_i - assign19020_e26967);
        (assign19020_e26968, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19020_e26970;
        var_nj0_dn0 = assign19020_e26970_d_n0;
        var_nj0_dn1 = assign19020_e26970_d_n1;
        var_nj0_dn2 = assign19020_e26970_d_n2;
        var_nj0_dn3 = assign19020_e26970_d_n3;
        var_nj0_dn4 = assign19020_e26970_d_n4;
        var_nj0_dn5 = assign19020_e26970_d_n5;
        var_nj0_db0 = assign19020_e26970_d_b0;
        var_nj0_db1 = assign19020_e26970_d_b1;
        var_nj0_db2 = assign19020_e26970_d_b2;
        var_nj0_db3 = assign19020_e26970_d_b3;

        let (assign19030_e26984, assign19030_e26984_d_n0, assign19030_e26984_d_n1, assign19030_e26984_d_n2, assign19030_e26984_d_n3, assign19030_e26984_d_n4, assign19030_e26984_d_n5, assign19030_e26984_d_b0, assign19030_e26984_d_b1, assign19030_e26984_d_b2, assign19030_e26984_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19030_e26980: f64 = (p.p85 - var_nja10);
        let assign19030_e26982: f64 = (assign19030_e26980 - 0.01);
        (assign19030_e26982, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19030_e26984;
        var_tmf1_dn0 = assign19030_e26984_d_n0;
        var_tmf1_dn1 = assign19030_e26984_d_n1;
        var_tmf1_dn2 = assign19030_e26984_d_n2;
        var_tmf1_dn3 = assign19030_e26984_d_n3;
        var_tmf1_dn4 = assign19030_e26984_d_n4;
        var_tmf1_dn5 = assign19030_e26984_d_n5;
        var_tmf1_db0 = assign19030_e26984_d_b0;
        var_tmf1_db1 = assign19030_e26984_d_b1;
        var_tmf1_db2 = assign19030_e26984_d_b2;
        var_tmf1_db3 = assign19030_e26984_d_b3;

        let (assign19040_e26998, assign19040_e26998_d_n0, assign19040_e26998_d_n1, assign19040_e26998_d_n2, assign19040_e26998_d_n3, assign19040_e26998_d_n4, assign19040_e26998_d_n5, assign19040_e26998_d_b0, assign19040_e26998_d_b1, assign19040_e26998_d_b2, assign19040_e26998_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19040_e26994: f64 = (4.0 * p.p85);
        let assign19040_e26996: f64 = (assign19040_e26994 * 0.01);
        (assign19040_e26996, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19040_e26998;
        var_tmf2_dn0 = assign19040_e26998_d_n0;
        var_tmf2_dn1 = assign19040_e26998_d_n1;
        var_tmf2_dn2 = assign19040_e26998_d_n2;
        var_tmf2_dn3 = assign19040_e26998_d_n3;
        var_tmf2_dn4 = assign19040_e26998_d_n4;
        var_tmf2_dn5 = assign19040_e26998_d_n5;
        var_tmf2_db0 = assign19040_e26998_d_b0;
        var_tmf2_db1 = assign19040_e26998_d_b1;
        var_tmf2_db2 = assign19040_e26998_d_b2;
        var_tmf2_db3 = assign19040_e26998_d_b3;

        let (assign19050_e27014, assign19050_e27014_d_n0, assign19050_e27014_d_n1, assign19050_e27014_d_n2, assign19050_e27014_d_n3, assign19050_e27014_d_n4, assign19050_e27014_d_n5, assign19050_e27014_d_b0, assign19050_e27014_d_b1, assign19050_e27014_d_b2, assign19050_e27014_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n1, assign19050_e27012_d_n2, assign19050_e27012_d_n3, assign19050_e27012_d_n4, assign19050_e27012_d_n5, assign19050_e27012_d_b0, assign19050_e27012_d_b1, assign19050_e27012_d_b2, assign19050_e27012_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19050_e27011: f64 = (-var_tmf2);
                (assign19050_e27011, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19050_e27012, assign19050_e27012_d_n0, assign19050_e27012_d_n1, assign19050_e27012_d_n2, assign19050_e27012_d_n3, assign19050_e27012_d_n4, assign19050_e27012_d_n5, assign19050_e27012_d_b0, assign19050_e27012_d_b1, assign19050_e27012_d_b2, assign19050_e27012_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19050_e27014;
        var_tmf2_dn0 = assign19050_e27014_d_n0;
        var_tmf2_dn1 = assign19050_e27014_d_n1;
        var_tmf2_dn2 = assign19050_e27014_d_n2;
        var_tmf2_dn3 = assign19050_e27014_d_n3;
        var_tmf2_dn4 = assign19050_e27014_d_n4;
        var_tmf2_dn5 = assign19050_e27014_d_n5;
        var_tmf2_db0 = assign19050_e27014_d_b0;
        var_tmf2_db1 = assign19050_e27014_d_b1;
        var_tmf2_db2 = assign19050_e27014_d_b2;
        var_tmf2_db3 = assign19050_e27014_d_b3;

        let (assign19060_e27029, assign19060_e27029_d_n0, assign19060_e27029_d_n1, assign19060_e27029_d_n2, assign19060_e27029_d_n3, assign19060_e27029_d_n4, assign19060_e27029_d_n5, assign19060_e27029_d_b0, assign19060_e27029_d_b1, assign19060_e27029_d_b2, assign19060_e27029_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19060_e27024: f64 = (var_tmf1 * var_tmf1);
        let assign19060_e27026: f64 = (assign19060_e27024 + var_tmf2);
        let assign19060_e27027: f64 = (assign19060_e27026).sqrt();
        (assign19060_e27027, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19060_e27027)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19060_e27027)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19060_e27027)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19060_e27027)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19060_e27027)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19060_e27027)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19060_e27029;
        var_tmf2_dn0 = assign19060_e27029_d_n0;
        var_tmf2_dn1 = assign19060_e27029_d_n1;
        var_tmf2_dn2 = assign19060_e27029_d_n2;
        var_tmf2_dn3 = assign19060_e27029_d_n3;
        var_tmf2_dn4 = assign19060_e27029_d_n4;
        var_tmf2_dn5 = assign19060_e27029_d_n5;
        var_tmf2_db0 = assign19060_e27029_d_b0;
        var_tmf2_db1 = assign19060_e27029_d_b1;
        var_tmf2_db2 = assign19060_e27029_d_b2;
        var_tmf2_db3 = assign19060_e27029_d_b3;

        let (assign19070_e27045, assign19070_e27045_d_n0, assign19070_e27045_d_n1, assign19070_e27045_d_n2, assign19070_e27045_d_n3, assign19070_e27045_d_n4, assign19070_e27045_d_n5, assign19070_e27045_d_b0, assign19070_e27045_d_b1, assign19070_e27045_d_b2, assign19070_e27045_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19070_e27041: f64 = (var_tmf1 + var_tmf2);
        let assign19070_e27042: f64 = (0.5 * assign19070_e27041);
        let assign19070_e27043: f64 = (p.p85 - assign19070_e27042);
        (assign19070_e27043, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign19070_e27045;
        var_nja11_dn0 = assign19070_e27045_d_n0;
        var_nja11_dn1 = assign19070_e27045_d_n1;
        var_nja11_dn2 = assign19070_e27045_d_n2;
        var_nja11_dn3 = assign19070_e27045_d_n3;
        var_nja11_dn4 = assign19070_e27045_d_n4;
        var_nja11_dn5 = assign19070_e27045_d_n5;
        var_nja11_db0 = assign19070_e27045_d_b0;
        var_nja11_db1 = assign19070_e27045_d_b1;
        var_nja11_db2 = assign19070_e27045_d_b2;
        var_nja11_db3 = assign19070_e27045_d_b3;

        let (assign19080_e27059, assign19080_e27059_d_n0, assign19080_e27059_d_n1, assign19080_e27059_d_n2, assign19080_e27059_d_n3, assign19080_e27059_d_n4, assign19080_e27059_d_n5, assign19080_e27059_d_b0, assign19080_e27059_d_b1, assign19080_e27059_d_b2, assign19080_e27059_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19080_e27055: f64 = (var_nja11 - var_nfasti_i);
        let assign19080_e27057: f64 = (assign19080_e27055 - 0.01);
        (assign19080_e27057, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19080_e27059;
        var_tmf1_dn0 = assign19080_e27059_d_n0;
        var_tmf1_dn1 = assign19080_e27059_d_n1;
        var_tmf1_dn2 = assign19080_e27059_d_n2;
        var_tmf1_dn3 = assign19080_e27059_d_n3;
        var_tmf1_dn4 = assign19080_e27059_d_n4;
        var_tmf1_dn5 = assign19080_e27059_d_n5;
        var_tmf1_db0 = assign19080_e27059_d_b0;
        var_tmf1_db1 = assign19080_e27059_d_b1;
        var_tmf1_db2 = assign19080_e27059_d_b2;
        var_tmf1_db3 = assign19080_e27059_d_b3;

        let (assign19090_e27073, assign19090_e27073_d_n0, assign19090_e27073_d_n1, assign19090_e27073_d_n2, assign19090_e27073_d_n3, assign19090_e27073_d_n4, assign19090_e27073_d_n5, assign19090_e27073_d_b0, assign19090_e27073_d_b1, assign19090_e27073_d_b2, assign19090_e27073_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19090_e27069: f64 = (4.0 * var_nfasti_i);
        let assign19090_e27071: f64 = (assign19090_e27069 * 0.01);
        (assign19090_e27071, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19090_e27073;
        var_tmf2_dn0 = assign19090_e27073_d_n0;
        var_tmf2_dn1 = assign19090_e27073_d_n1;
        var_tmf2_dn2 = assign19090_e27073_d_n2;
        var_tmf2_dn3 = assign19090_e27073_d_n3;
        var_tmf2_dn4 = assign19090_e27073_d_n4;
        var_tmf2_dn5 = assign19090_e27073_d_n5;
        var_tmf2_db0 = assign19090_e27073_d_b0;
        var_tmf2_db1 = assign19090_e27073_d_b1;
        var_tmf2_db2 = assign19090_e27073_d_b2;
        var_tmf2_db3 = assign19090_e27073_d_b3;

        *var_guard312_slot = var_guard312;
        *var_guard313_slot = var_guard313;
        *var_guard314_slot = var_guard314;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard314: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_guard317_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign19100_e27089, assign19100_e27089_d_n0, assign19100_e27089_d_n1, assign19100_e27089_d_n2, assign19100_e27089_d_n3, assign19100_e27089_d_n4, assign19100_e27089_d_n5, assign19100_e27089_d_b0, assign19100_e27089_d_b1, assign19100_e27089_d_b2, assign19100_e27089_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n1, assign19100_e27087_d_n2, assign19100_e27087_d_n3, assign19100_e27087_d_n4, assign19100_e27087_d_n5, assign19100_e27087_d_b0, assign19100_e27087_d_b1, assign19100_e27087_d_b2, assign19100_e27087_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19100_e27086: f64 = (-var_tmf2);
                (assign19100_e27086, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19100_e27087, assign19100_e27087_d_n0, assign19100_e27087_d_n1, assign19100_e27087_d_n2, assign19100_e27087_d_n3, assign19100_e27087_d_n4, assign19100_e27087_d_n5, assign19100_e27087_d_b0, assign19100_e27087_d_b1, assign19100_e27087_d_b2, assign19100_e27087_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19100_e27089;
        var_tmf2_dn0 = assign19100_e27089_d_n0;
        var_tmf2_dn1 = assign19100_e27089_d_n1;
        var_tmf2_dn2 = assign19100_e27089_d_n2;
        var_tmf2_dn3 = assign19100_e27089_d_n3;
        var_tmf2_dn4 = assign19100_e27089_d_n4;
        var_tmf2_dn5 = assign19100_e27089_d_n5;
        var_tmf2_db0 = assign19100_e27089_d_b0;
        var_tmf2_db1 = assign19100_e27089_d_b1;
        var_tmf2_db2 = assign19100_e27089_d_b2;
        var_tmf2_db3 = assign19100_e27089_d_b3;

        let (assign19110_e27104, assign19110_e27104_d_n0, assign19110_e27104_d_n1, assign19110_e27104_d_n2, assign19110_e27104_d_n3, assign19110_e27104_d_n4, assign19110_e27104_d_n5, assign19110_e27104_d_b0, assign19110_e27104_d_b1, assign19110_e27104_d_b2, assign19110_e27104_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19110_e27099: f64 = (var_tmf1 * var_tmf1);
        let assign19110_e27101: f64 = (assign19110_e27099 + var_tmf2);
        let assign19110_e27102: f64 = (assign19110_e27101).sqrt();
        (assign19110_e27102, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19110_e27102)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19110_e27102)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19110_e27102)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19110_e27102)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19110_e27102)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19110_e27102)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19110_e27104;
        var_tmf2_dn0 = assign19110_e27104_d_n0;
        var_tmf2_dn1 = assign19110_e27104_d_n1;
        var_tmf2_dn2 = assign19110_e27104_d_n2;
        var_tmf2_dn3 = assign19110_e27104_d_n3;
        var_tmf2_dn4 = assign19110_e27104_d_n4;
        var_tmf2_dn5 = assign19110_e27104_d_n5;
        var_tmf2_db0 = assign19110_e27104_d_b0;
        var_tmf2_db1 = assign19110_e27104_d_b1;
        var_tmf2_db2 = assign19110_e27104_d_b2;
        var_tmf2_db3 = assign19110_e27104_d_b3;

        let (assign19120_e27120, assign19120_e27120_d_n0, assign19120_e27120_d_n1, assign19120_e27120_d_n2, assign19120_e27120_d_n3, assign19120_e27120_d_n4, assign19120_e27120_d_n5, assign19120_e27120_d_b0, assign19120_e27120_d_b1, assign19120_e27120_d_b2, assign19120_e27120_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19120_e27116: f64 = (var_tmf1 + var_tmf2);
        let assign19120_e27117: f64 = (0.5 * assign19120_e27116);
        let assign19120_e27118: f64 = (var_nfasti_i + assign19120_e27117);
        (assign19120_e27118, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19120_e27120;
        var_nj1_dn0 = assign19120_e27120_d_n0;
        var_nj1_dn1 = assign19120_e27120_d_n1;
        var_nj1_dn2 = assign19120_e27120_d_n2;
        var_nj1_dn3 = assign19120_e27120_d_n3;
        var_nj1_dn4 = assign19120_e27120_d_n4;
        var_nj1_dn5 = assign19120_e27120_d_n5;
        var_nj1_db0 = assign19120_e27120_d_b0;
        var_nj1_db1 = assign19120_e27120_d_b1;
        var_nj1_db2 = assign19120_e27120_d_b2;
        var_nj1_db3 = assign19120_e27120_d_b3;

        let (assign19130_e27134, assign19130_e27134_d_n0, assign19130_e27134_d_n1, assign19130_e27134_d_n2, assign19130_e27134_d_n3, assign19130_e27134_d_n4, assign19130_e27134_d_n5, assign19130_e27134_d_b0, assign19130_e27134_d_b1, assign19130_e27134_d_b2, assign19130_e27134_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19130_e27130: f64 = (p.p85 - var_nj0);
        let assign19130_e27132: f64 = (assign19130_e27130 - 0.01);
        (assign19130_e27132, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19130_e27134;
        var_tmf1_dn0 = assign19130_e27134_d_n0;
        var_tmf1_dn1 = assign19130_e27134_d_n1;
        var_tmf1_dn2 = assign19130_e27134_d_n2;
        var_tmf1_dn3 = assign19130_e27134_d_n3;
        var_tmf1_dn4 = assign19130_e27134_d_n4;
        var_tmf1_dn5 = assign19130_e27134_d_n5;
        var_tmf1_db0 = assign19130_e27134_d_b0;
        var_tmf1_db1 = assign19130_e27134_d_b1;
        var_tmf1_db2 = assign19130_e27134_d_b2;
        var_tmf1_db3 = assign19130_e27134_d_b3;

        let (assign19140_e27148, assign19140_e27148_d_n0, assign19140_e27148_d_n1, assign19140_e27148_d_n2, assign19140_e27148_d_n3, assign19140_e27148_d_n4, assign19140_e27148_d_n5, assign19140_e27148_d_b0, assign19140_e27148_d_b1, assign19140_e27148_d_b2, assign19140_e27148_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19140_e27144: f64 = (4.0 * p.p85);
        let assign19140_e27146: f64 = (assign19140_e27144 * 0.01);
        (assign19140_e27146, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19140_e27148;
        var_tmf2_dn0 = assign19140_e27148_d_n0;
        var_tmf2_dn1 = assign19140_e27148_d_n1;
        var_tmf2_dn2 = assign19140_e27148_d_n2;
        var_tmf2_dn3 = assign19140_e27148_d_n3;
        var_tmf2_dn4 = assign19140_e27148_d_n4;
        var_tmf2_dn5 = assign19140_e27148_d_n5;
        var_tmf2_db0 = assign19140_e27148_d_b0;
        var_tmf2_db1 = assign19140_e27148_d_b1;
        var_tmf2_db2 = assign19140_e27148_d_b2;
        var_tmf2_db3 = assign19140_e27148_d_b3;

        let (assign19150_e27164, assign19150_e27164_d_n0, assign19150_e27164_d_n1, assign19150_e27164_d_n2, assign19150_e27164_d_n3, assign19150_e27164_d_n4, assign19150_e27164_d_n5, assign19150_e27164_d_b0, assign19150_e27164_d_b1, assign19150_e27164_d_b2, assign19150_e27164_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n1, assign19150_e27162_d_n2, assign19150_e27162_d_n3, assign19150_e27162_d_n4, assign19150_e27162_d_n5, assign19150_e27162_d_b0, assign19150_e27162_d_b1, assign19150_e27162_d_b2, assign19150_e27162_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19150_e27161: f64 = (-var_tmf2);
                (assign19150_e27161, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19150_e27162, assign19150_e27162_d_n0, assign19150_e27162_d_n1, assign19150_e27162_d_n2, assign19150_e27162_d_n3, assign19150_e27162_d_n4, assign19150_e27162_d_n5, assign19150_e27162_d_b0, assign19150_e27162_d_b1, assign19150_e27162_d_b2, assign19150_e27162_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19150_e27164;
        var_tmf2_dn0 = assign19150_e27164_d_n0;
        var_tmf2_dn1 = assign19150_e27164_d_n1;
        var_tmf2_dn2 = assign19150_e27164_d_n2;
        var_tmf2_dn3 = assign19150_e27164_d_n3;
        var_tmf2_dn4 = assign19150_e27164_d_n4;
        var_tmf2_dn5 = assign19150_e27164_d_n5;
        var_tmf2_db0 = assign19150_e27164_d_b0;
        var_tmf2_db1 = assign19150_e27164_d_b1;
        var_tmf2_db2 = assign19150_e27164_d_b2;
        var_tmf2_db3 = assign19150_e27164_d_b3;

        let (assign19160_e27179, assign19160_e27179_d_n0, assign19160_e27179_d_n1, assign19160_e27179_d_n2, assign19160_e27179_d_n3, assign19160_e27179_d_n4, assign19160_e27179_d_n5, assign19160_e27179_d_b0, assign19160_e27179_d_b1, assign19160_e27179_d_b2, assign19160_e27179_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19160_e27174: f64 = (var_tmf1 * var_tmf1);
        let assign19160_e27176: f64 = (assign19160_e27174 + var_tmf2);
        let assign19160_e27177: f64 = (assign19160_e27176).sqrt();
        (assign19160_e27177, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19160_e27177)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19160_e27177)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19160_e27177)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19160_e27177)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19160_e27177)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19160_e27177)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19160_e27179;
        var_tmf2_dn0 = assign19160_e27179_d_n0;
        var_tmf2_dn1 = assign19160_e27179_d_n1;
        var_tmf2_dn2 = assign19160_e27179_d_n2;
        var_tmf2_dn3 = assign19160_e27179_d_n3;
        var_tmf2_dn4 = assign19160_e27179_d_n4;
        var_tmf2_dn5 = assign19160_e27179_d_n5;
        var_tmf2_db0 = assign19160_e27179_d_b0;
        var_tmf2_db1 = assign19160_e27179_d_b1;
        var_tmf2_db2 = assign19160_e27179_d_b2;
        var_tmf2_db3 = assign19160_e27179_d_b3;

        let (assign19170_e27195, assign19170_e27195_d_n0, assign19170_e27195_d_n1, assign19170_e27195_d_n2, assign19170_e27195_d_n3, assign19170_e27195_d_n4, assign19170_e27195_d_n5, assign19170_e27195_d_b0, assign19170_e27195_d_b1, assign19170_e27195_d_b2, assign19170_e27195_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19170_e27191: f64 = (var_tmf1 + var_tmf2);
        let assign19170_e27192: f64 = (0.5 * assign19170_e27191);
        let assign19170_e27193: f64 = (p.p85 - assign19170_e27192);
        (assign19170_e27193, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19170_e27195;
        var_nj0_dn0 = assign19170_e27195_d_n0;
        var_nj0_dn1 = assign19170_e27195_d_n1;
        var_nj0_dn2 = assign19170_e27195_d_n2;
        var_nj0_dn3 = assign19170_e27195_d_n3;
        var_nj0_dn4 = assign19170_e27195_d_n4;
        var_nj0_dn5 = assign19170_e27195_d_n5;
        var_nj0_db0 = assign19170_e27195_d_b0;
        var_nj0_db1 = assign19170_e27195_d_b1;
        var_nj0_db2 = assign19170_e27195_d_b2;
        var_nj0_db3 = assign19170_e27195_d_b3;

        let (assign19180_e27209, assign19180_e27209_d_n0, assign19180_e27209_d_n1, assign19180_e27209_d_n2, assign19180_e27209_d_n3, assign19180_e27209_d_n4, assign19180_e27209_d_n5, assign19180_e27209_d_b0, assign19180_e27209_d_b1, assign19180_e27209_d_b2, assign19180_e27209_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19180_e27205: f64 = (var_nj0 - var_nfasti_i);
        let assign19180_e27207: f64 = (assign19180_e27205 - 0.01);
        (assign19180_e27207, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19180_e27209;
        var_tmf1_dn0 = assign19180_e27209_d_n0;
        var_tmf1_dn1 = assign19180_e27209_d_n1;
        var_tmf1_dn2 = assign19180_e27209_d_n2;
        var_tmf1_dn3 = assign19180_e27209_d_n3;
        var_tmf1_dn4 = assign19180_e27209_d_n4;
        var_tmf1_dn5 = assign19180_e27209_d_n5;
        var_tmf1_db0 = assign19180_e27209_d_b0;
        var_tmf1_db1 = assign19180_e27209_d_b1;
        var_tmf1_db2 = assign19180_e27209_d_b2;
        var_tmf1_db3 = assign19180_e27209_d_b3;

        let (assign19190_e27223, assign19190_e27223_d_n0, assign19190_e27223_d_n1, assign19190_e27223_d_n2, assign19190_e27223_d_n3, assign19190_e27223_d_n4, assign19190_e27223_d_n5, assign19190_e27223_d_b0, assign19190_e27223_d_b1, assign19190_e27223_d_b2, assign19190_e27223_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19190_e27219: f64 = (4.0 * var_nfasti_i);
        let assign19190_e27221: f64 = (assign19190_e27219 * 0.01);
        (assign19190_e27221, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19190_e27223;
        var_tmf2_dn0 = assign19190_e27223_d_n0;
        var_tmf2_dn1 = assign19190_e27223_d_n1;
        var_tmf2_dn2 = assign19190_e27223_d_n2;
        var_tmf2_dn3 = assign19190_e27223_d_n3;
        var_tmf2_dn4 = assign19190_e27223_d_n4;
        var_tmf2_dn5 = assign19190_e27223_d_n5;
        var_tmf2_db0 = assign19190_e27223_d_b0;
        var_tmf2_db1 = assign19190_e27223_d_b1;
        var_tmf2_db2 = assign19190_e27223_d_b2;
        var_tmf2_db3 = assign19190_e27223_d_b3;

        let (assign19200_e27239, assign19200_e27239_d_n0, assign19200_e27239_d_n1, assign19200_e27239_d_n2, assign19200_e27239_d_n3, assign19200_e27239_d_n4, assign19200_e27239_d_n5, assign19200_e27239_d_b0, assign19200_e27239_d_b1, assign19200_e27239_d_b2, assign19200_e27239_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n1, assign19200_e27237_d_n2, assign19200_e27237_d_n3, assign19200_e27237_d_n4, assign19200_e27237_d_n5, assign19200_e27237_d_b0, assign19200_e27237_d_b1, assign19200_e27237_d_b2, assign19200_e27237_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19200_e27236: f64 = (-var_tmf2);
                (assign19200_e27236, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19200_e27237, assign19200_e27237_d_n0, assign19200_e27237_d_n1, assign19200_e27237_d_n2, assign19200_e27237_d_n3, assign19200_e27237_d_n4, assign19200_e27237_d_n5, assign19200_e27237_d_b0, assign19200_e27237_d_b1, assign19200_e27237_d_b2, assign19200_e27237_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19200_e27239;
        var_tmf2_dn0 = assign19200_e27239_d_n0;
        var_tmf2_dn1 = assign19200_e27239_d_n1;
        var_tmf2_dn2 = assign19200_e27239_d_n2;
        var_tmf2_dn3 = assign19200_e27239_d_n3;
        var_tmf2_dn4 = assign19200_e27239_d_n4;
        var_tmf2_dn5 = assign19200_e27239_d_n5;
        var_tmf2_db0 = assign19200_e27239_d_b0;
        var_tmf2_db1 = assign19200_e27239_d_b1;
        var_tmf2_db2 = assign19200_e27239_d_b2;
        var_tmf2_db3 = assign19200_e27239_d_b3;

        let (assign19210_e27254, assign19210_e27254_d_n0, assign19210_e27254_d_n1, assign19210_e27254_d_n2, assign19210_e27254_d_n3, assign19210_e27254_d_n4, assign19210_e27254_d_n5, assign19210_e27254_d_b0, assign19210_e27254_d_b1, assign19210_e27254_d_b2, assign19210_e27254_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19210_e27249: f64 = (var_tmf1 * var_tmf1);
        let assign19210_e27251: f64 = (assign19210_e27249 + var_tmf2);
        let assign19210_e27252: f64 = (assign19210_e27251).sqrt();
        (assign19210_e27252, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19210_e27252)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19210_e27252)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19210_e27252)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19210_e27252)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19210_e27252)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19210_e27252)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19210_e27254;
        var_tmf2_dn0 = assign19210_e27254_d_n0;
        var_tmf2_dn1 = assign19210_e27254_d_n1;
        var_tmf2_dn2 = assign19210_e27254_d_n2;
        var_tmf2_dn3 = assign19210_e27254_d_n3;
        var_tmf2_dn4 = assign19210_e27254_d_n4;
        var_tmf2_dn5 = assign19210_e27254_d_n5;
        var_tmf2_db0 = assign19210_e27254_d_b0;
        var_tmf2_db1 = assign19210_e27254_d_b1;
        var_tmf2_db2 = assign19210_e27254_d_b2;
        var_tmf2_db3 = assign19210_e27254_d_b3;

        let (assign19220_e27270, assign19220_e27270_d_n0, assign19220_e27270_d_n1, assign19220_e27270_d_n2, assign19220_e27270_d_n3, assign19220_e27270_d_n4, assign19220_e27270_d_n5, assign19220_e27270_d_b0, assign19220_e27270_d_b1, assign19220_e27270_d_b2, assign19220_e27270_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 != 0.0)) {
        let assign19220_e27266: f64 = (var_tmf1 + var_tmf2);
        let assign19220_e27267: f64 = (0.5 * assign19220_e27266);
        let assign19220_e27268: f64 = (var_nfasti_i + assign19220_e27267);
        (assign19220_e27268, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19220_e27270;
        var_nj0_dn0 = assign19220_e27270_d_n0;
        var_nj0_dn1 = assign19220_e27270_d_n1;
        var_nj0_dn2 = assign19220_e27270_d_n2;
        var_nj0_dn3 = assign19220_e27270_d_n3;
        var_nj0_dn4 = assign19220_e27270_d_n4;
        var_nj0_dn5 = assign19220_e27270_d_n5;
        var_nj0_db0 = assign19220_e27270_d_b0;
        var_nj0_db1 = assign19220_e27270_d_b1;
        var_nj0_db2 = assign19220_e27270_d_b2;
        var_nj0_db3 = assign19220_e27270_d_b3;

        let (assign19230_e27281, assign19230_e27281_d_n0, assign19230_e27281_d_n1, assign19230_e27281_d_n2, assign19230_e27281_d_n3, assign19230_e27281_d_n4, assign19230_e27281_d_n5, assign19230_e27281_d_b0, assign19230_e27281_d_b1, assign19230_e27281_d_b2, assign19230_e27281_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19230_e27281;
        var_nj0_dn0 = assign19230_e27281_d_n0;
        var_nj0_dn1 = assign19230_e27281_d_n1;
        var_nj0_dn2 = assign19230_e27281_d_n2;
        var_nj0_dn3 = assign19230_e27281_d_n3;
        var_nj0_dn4 = assign19230_e27281_d_n4;
        var_nj0_dn5 = assign19230_e27281_d_n5;
        var_nj0_db0 = assign19230_e27281_d_b0;
        var_nj0_db1 = assign19230_e27281_d_b1;
        var_nj0_db2 = assign19230_e27281_d_b2;
        var_nj0_db3 = assign19230_e27281_d_b3;

        let (assign19240_e27292, assign19240_e27292_d_n0, assign19240_e27292_d_n1, assign19240_e27292_d_n2, assign19240_e27292_d_n3, assign19240_e27292_d_n4, assign19240_e27292_d_n5, assign19240_e27292_d_b0, assign19240_e27292_d_b1, assign19240_e27292_d_b2, assign19240_e27292_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard314 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19240_e27292;
        var_nj1_dn0 = assign19240_e27292_d_n0;
        var_nj1_dn1 = assign19240_e27292_d_n1;
        var_nj1_dn2 = assign19240_e27292_d_n2;
        var_nj1_dn3 = assign19240_e27292_d_n3;
        var_nj1_dn4 = assign19240_e27292_d_n4;
        var_nj1_dn5 = assign19240_e27292_d_n5;
        var_nj1_db0 = assign19240_e27292_d_b0;
        var_nj1_db1 = assign19240_e27292_d_b1;
        var_nj1_db2 = assign19240_e27292_d_b2;
        var_nj1_db3 = assign19240_e27292_d_b3;

        let (assign19300_e27523,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign19300_e27519: f64 = (var_nin * var_nin);
        let assign19300_e27521: f64 = (assign19300_e27519 / var_ndigat_i);
        (assign19300_e27521,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign19300_e27523;

        let (assign19310_e27538,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign19310_e27531: f64 = (var_nfagat_i / var_phitdinv);
        let assign19310_e27534: f64 = (var_ndigat_i / var_pnn0);
        let assign19310_e27535: f64 = (assign19310_e27534).ln();
        let assign19310_e27536: f64 = (assign19310_e27531 * assign19310_e27535);
        (assign19310_e27536,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign19310_e27538;

        let assign19320_e27541: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard317 = assign19320_e27541;

        let (assign19330_e27557, assign19330_e27557_d_n0, assign19330_e27557_d_n1, assign19330_e27557_d_n2, assign19330_e27557_d_n3, assign19330_e27557_d_n4, assign19330_e27557_d_n5, assign19330_e27557_d_b0, assign19330_e27557_d_b1, assign19330_e27557_d_b2, assign19330_e27557_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19330_e27552: f64 = (var_v4 - var_vha1);
        let assign19330_e27553: f64 = (p.p86 * assign19330_e27552);
        let assign19330_e27555: f64 = (assign19330_e27553 + var_nfagat_i);
        (assign19330_e27555, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign19330_e27557;
        var_nja10_dn0 = assign19330_e27557_d_n0;
        var_nja10_dn1 = assign19330_e27557_d_n1;
        var_nja10_dn2 = assign19330_e27557_d_n2;
        var_nja10_dn3 = assign19330_e27557_d_n3;
        var_nja10_dn4 = assign19330_e27557_d_n4;
        var_nja10_dn5 = assign19330_e27557_d_n5;
        var_nja10_db0 = assign19330_e27557_d_b0;
        var_nja10_db1 = assign19330_e27557_d_b1;
        var_nja10_db2 = assign19330_e27557_d_b2;
        var_nja10_db3 = assign19330_e27557_d_b3;

        let (assign19340_e27571, assign19340_e27571_d_n0, assign19340_e27571_d_n1, assign19340_e27571_d_n2, assign19340_e27571_d_n3, assign19340_e27571_d_n4, assign19340_e27571_d_n5, assign19340_e27571_d_b0, assign19340_e27571_d_b1, assign19340_e27571_d_b2, assign19340_e27571_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19340_e27568: f64 = (p.p86 * var_vha1);
        let assign19340_e27569: f64 = (var_nfagat_i - assign19340_e27568);
        (assign19340_e27569, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19340_e27571;
        var_nj0_dn0 = assign19340_e27571_d_n0;
        var_nj0_dn1 = assign19340_e27571_d_n1;
        var_nj0_dn2 = assign19340_e27571_d_n2;
        var_nj0_dn3 = assign19340_e27571_d_n3;
        var_nj0_dn4 = assign19340_e27571_d_n4;
        var_nj0_dn5 = assign19340_e27571_d_n5;
        var_nj0_db0 = assign19340_e27571_d_b0;
        var_nj0_db1 = assign19340_e27571_d_b1;
        var_nj0_db2 = assign19340_e27571_d_b2;
        var_nj0_db3 = assign19340_e27571_d_b3;

        let (assign19350_e27585, assign19350_e27585_d_n0, assign19350_e27585_d_n1, assign19350_e27585_d_n2, assign19350_e27585_d_n3, assign19350_e27585_d_n4, assign19350_e27585_d_n5, assign19350_e27585_d_b0, assign19350_e27585_d_b1, assign19350_e27585_d_b2, assign19350_e27585_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19350_e27581: f64 = (p.p85 - var_nja10);
        let assign19350_e27583: f64 = (assign19350_e27581 - 0.01);
        (assign19350_e27583, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19350_e27585;
        var_tmf1_dn0 = assign19350_e27585_d_n0;
        var_tmf1_dn1 = assign19350_e27585_d_n1;
        var_tmf1_dn2 = assign19350_e27585_d_n2;
        var_tmf1_dn3 = assign19350_e27585_d_n3;
        var_tmf1_dn4 = assign19350_e27585_d_n4;
        var_tmf1_dn5 = assign19350_e27585_d_n5;
        var_tmf1_db0 = assign19350_e27585_d_b0;
        var_tmf1_db1 = assign19350_e27585_d_b1;
        var_tmf1_db2 = assign19350_e27585_d_b2;
        var_tmf1_db3 = assign19350_e27585_d_b3;

        let (assign19360_e27599, assign19360_e27599_d_n0, assign19360_e27599_d_n1, assign19360_e27599_d_n2, assign19360_e27599_d_n3, assign19360_e27599_d_n4, assign19360_e27599_d_n5, assign19360_e27599_d_b0, assign19360_e27599_d_b1, assign19360_e27599_d_b2, assign19360_e27599_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19360_e27595: f64 = (4.0 * p.p85);
        let assign19360_e27597: f64 = (assign19360_e27595 * 0.01);
        (assign19360_e27597, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19360_e27599;
        var_tmf2_dn0 = assign19360_e27599_d_n0;
        var_tmf2_dn1 = assign19360_e27599_d_n1;
        var_tmf2_dn2 = assign19360_e27599_d_n2;
        var_tmf2_dn3 = assign19360_e27599_d_n3;
        var_tmf2_dn4 = assign19360_e27599_d_n4;
        var_tmf2_dn5 = assign19360_e27599_d_n5;
        var_tmf2_db0 = assign19360_e27599_d_b0;
        var_tmf2_db1 = assign19360_e27599_d_b1;
        var_tmf2_db2 = assign19360_e27599_d_b2;
        var_tmf2_db3 = assign19360_e27599_d_b3;

        let (assign19370_e27615, assign19370_e27615_d_n0, assign19370_e27615_d_n1, assign19370_e27615_d_n2, assign19370_e27615_d_n3, assign19370_e27615_d_n4, assign19370_e27615_d_n5, assign19370_e27615_d_b0, assign19370_e27615_d_b1, assign19370_e27615_d_b2, assign19370_e27615_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n1, assign19370_e27613_d_n2, assign19370_e27613_d_n3, assign19370_e27613_d_n4, assign19370_e27613_d_n5, assign19370_e27613_d_b0, assign19370_e27613_d_b1, assign19370_e27613_d_b2, assign19370_e27613_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19370_e27612: f64 = (-var_tmf2);
                (assign19370_e27612, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19370_e27613, assign19370_e27613_d_n0, assign19370_e27613_d_n1, assign19370_e27613_d_n2, assign19370_e27613_d_n3, assign19370_e27613_d_n4, assign19370_e27613_d_n5, assign19370_e27613_d_b0, assign19370_e27613_d_b1, assign19370_e27613_d_b2, assign19370_e27613_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19370_e27615;
        var_tmf2_dn0 = assign19370_e27615_d_n0;
        var_tmf2_dn1 = assign19370_e27615_d_n1;
        var_tmf2_dn2 = assign19370_e27615_d_n2;
        var_tmf2_dn3 = assign19370_e27615_d_n3;
        var_tmf2_dn4 = assign19370_e27615_d_n4;
        var_tmf2_dn5 = assign19370_e27615_d_n5;
        var_tmf2_db0 = assign19370_e27615_d_b0;
        var_tmf2_db1 = assign19370_e27615_d_b1;
        var_tmf2_db2 = assign19370_e27615_d_b2;
        var_tmf2_db3 = assign19370_e27615_d_b3;

        let (assign19380_e27630, assign19380_e27630_d_n0, assign19380_e27630_d_n1, assign19380_e27630_d_n2, assign19380_e27630_d_n3, assign19380_e27630_d_n4, assign19380_e27630_d_n5, assign19380_e27630_d_b0, assign19380_e27630_d_b1, assign19380_e27630_d_b2, assign19380_e27630_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19380_e27625: f64 = (var_tmf1 * var_tmf1);
        let assign19380_e27627: f64 = (assign19380_e27625 + var_tmf2);
        let assign19380_e27628: f64 = (assign19380_e27627).sqrt();
        (assign19380_e27628, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19380_e27628)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19380_e27628)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19380_e27628)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19380_e27628)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19380_e27628)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19380_e27628)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19380_e27630;
        var_tmf2_dn0 = assign19380_e27630_d_n0;
        var_tmf2_dn1 = assign19380_e27630_d_n1;
        var_tmf2_dn2 = assign19380_e27630_d_n2;
        var_tmf2_dn3 = assign19380_e27630_d_n3;
        var_tmf2_dn4 = assign19380_e27630_d_n4;
        var_tmf2_dn5 = assign19380_e27630_d_n5;
        var_tmf2_db0 = assign19380_e27630_d_b0;
        var_tmf2_db1 = assign19380_e27630_d_b1;
        var_tmf2_db2 = assign19380_e27630_d_b2;
        var_tmf2_db3 = assign19380_e27630_d_b3;

        *var_guard317_slot = var_guard317;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard317: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_guard320_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign19390_e27646, assign19390_e27646_d_n0, assign19390_e27646_d_n1, assign19390_e27646_d_n2, assign19390_e27646_d_n3, assign19390_e27646_d_n4, assign19390_e27646_d_n5, assign19390_e27646_d_b0, assign19390_e27646_d_b1, assign19390_e27646_d_b2, assign19390_e27646_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19390_e27642: f64 = (var_tmf1 + var_tmf2);
        let assign19390_e27643: f64 = (0.5 * assign19390_e27642);
        let assign19390_e27644: f64 = (p.p85 - assign19390_e27643);
        (assign19390_e27644, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign19390_e27646;
        var_nja11_dn0 = assign19390_e27646_d_n0;
        var_nja11_dn1 = assign19390_e27646_d_n1;
        var_nja11_dn2 = assign19390_e27646_d_n2;
        var_nja11_dn3 = assign19390_e27646_d_n3;
        var_nja11_dn4 = assign19390_e27646_d_n4;
        var_nja11_dn5 = assign19390_e27646_d_n5;
        var_nja11_db0 = assign19390_e27646_d_b0;
        var_nja11_db1 = assign19390_e27646_d_b1;
        var_nja11_db2 = assign19390_e27646_d_b2;
        var_nja11_db3 = assign19390_e27646_d_b3;

        let (assign19400_e27660, assign19400_e27660_d_n0, assign19400_e27660_d_n1, assign19400_e27660_d_n2, assign19400_e27660_d_n3, assign19400_e27660_d_n4, assign19400_e27660_d_n5, assign19400_e27660_d_b0, assign19400_e27660_d_b1, assign19400_e27660_d_b2, assign19400_e27660_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19400_e27656: f64 = (var_nja11 - var_nfagat_i);
        let assign19400_e27658: f64 = (assign19400_e27656 - 0.01);
        (assign19400_e27658, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19400_e27660;
        var_tmf1_dn0 = assign19400_e27660_d_n0;
        var_tmf1_dn1 = assign19400_e27660_d_n1;
        var_tmf1_dn2 = assign19400_e27660_d_n2;
        var_tmf1_dn3 = assign19400_e27660_d_n3;
        var_tmf1_dn4 = assign19400_e27660_d_n4;
        var_tmf1_dn5 = assign19400_e27660_d_n5;
        var_tmf1_db0 = assign19400_e27660_d_b0;
        var_tmf1_db1 = assign19400_e27660_d_b1;
        var_tmf1_db2 = assign19400_e27660_d_b2;
        var_tmf1_db3 = assign19400_e27660_d_b3;

        let (assign19410_e27674, assign19410_e27674_d_n0, assign19410_e27674_d_n1, assign19410_e27674_d_n2, assign19410_e27674_d_n3, assign19410_e27674_d_n4, assign19410_e27674_d_n5, assign19410_e27674_d_b0, assign19410_e27674_d_b1, assign19410_e27674_d_b2, assign19410_e27674_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19410_e27670: f64 = (4.0 * var_nfagat_i);
        let assign19410_e27672: f64 = (assign19410_e27670 * 0.01);
        (assign19410_e27672, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19410_e27674;
        var_tmf2_dn0 = assign19410_e27674_d_n0;
        var_tmf2_dn1 = assign19410_e27674_d_n1;
        var_tmf2_dn2 = assign19410_e27674_d_n2;
        var_tmf2_dn3 = assign19410_e27674_d_n3;
        var_tmf2_dn4 = assign19410_e27674_d_n4;
        var_tmf2_dn5 = assign19410_e27674_d_n5;
        var_tmf2_db0 = assign19410_e27674_d_b0;
        var_tmf2_db1 = assign19410_e27674_d_b1;
        var_tmf2_db2 = assign19410_e27674_d_b2;
        var_tmf2_db3 = assign19410_e27674_d_b3;

        let (assign19420_e27690, assign19420_e27690_d_n0, assign19420_e27690_d_n1, assign19420_e27690_d_n2, assign19420_e27690_d_n3, assign19420_e27690_d_n4, assign19420_e27690_d_n5, assign19420_e27690_d_b0, assign19420_e27690_d_b1, assign19420_e27690_d_b2, assign19420_e27690_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n1, assign19420_e27688_d_n2, assign19420_e27688_d_n3, assign19420_e27688_d_n4, assign19420_e27688_d_n5, assign19420_e27688_d_b0, assign19420_e27688_d_b1, assign19420_e27688_d_b2, assign19420_e27688_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19420_e27687: f64 = (-var_tmf2);
                (assign19420_e27687, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19420_e27688, assign19420_e27688_d_n0, assign19420_e27688_d_n1, assign19420_e27688_d_n2, assign19420_e27688_d_n3, assign19420_e27688_d_n4, assign19420_e27688_d_n5, assign19420_e27688_d_b0, assign19420_e27688_d_b1, assign19420_e27688_d_b2, assign19420_e27688_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19420_e27690;
        var_tmf2_dn0 = assign19420_e27690_d_n0;
        var_tmf2_dn1 = assign19420_e27690_d_n1;
        var_tmf2_dn2 = assign19420_e27690_d_n2;
        var_tmf2_dn3 = assign19420_e27690_d_n3;
        var_tmf2_dn4 = assign19420_e27690_d_n4;
        var_tmf2_dn5 = assign19420_e27690_d_n5;
        var_tmf2_db0 = assign19420_e27690_d_b0;
        var_tmf2_db1 = assign19420_e27690_d_b1;
        var_tmf2_db2 = assign19420_e27690_d_b2;
        var_tmf2_db3 = assign19420_e27690_d_b3;

        let (assign19430_e27705, assign19430_e27705_d_n0, assign19430_e27705_d_n1, assign19430_e27705_d_n2, assign19430_e27705_d_n3, assign19430_e27705_d_n4, assign19430_e27705_d_n5, assign19430_e27705_d_b0, assign19430_e27705_d_b1, assign19430_e27705_d_b2, assign19430_e27705_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19430_e27700: f64 = (var_tmf1 * var_tmf1);
        let assign19430_e27702: f64 = (assign19430_e27700 + var_tmf2);
        let assign19430_e27703: f64 = (assign19430_e27702).sqrt();
        (assign19430_e27703, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19430_e27703)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19430_e27703)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19430_e27703)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19430_e27703)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19430_e27703)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19430_e27703)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19430_e27705;
        var_tmf2_dn0 = assign19430_e27705_d_n0;
        var_tmf2_dn1 = assign19430_e27705_d_n1;
        var_tmf2_dn2 = assign19430_e27705_d_n2;
        var_tmf2_dn3 = assign19430_e27705_d_n3;
        var_tmf2_dn4 = assign19430_e27705_d_n4;
        var_tmf2_dn5 = assign19430_e27705_d_n5;
        var_tmf2_db0 = assign19430_e27705_d_b0;
        var_tmf2_db1 = assign19430_e27705_d_b1;
        var_tmf2_db2 = assign19430_e27705_d_b2;
        var_tmf2_db3 = assign19430_e27705_d_b3;

        let (assign19440_e27721, assign19440_e27721_d_n0, assign19440_e27721_d_n1, assign19440_e27721_d_n2, assign19440_e27721_d_n3, assign19440_e27721_d_n4, assign19440_e27721_d_n5, assign19440_e27721_d_b0, assign19440_e27721_d_b1, assign19440_e27721_d_b2, assign19440_e27721_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19440_e27717: f64 = (var_tmf1 + var_tmf2);
        let assign19440_e27718: f64 = (0.5 * assign19440_e27717);
        let assign19440_e27719: f64 = (var_nfagat_i + assign19440_e27718);
        (assign19440_e27719, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19440_e27721;
        var_nj1_dn0 = assign19440_e27721_d_n0;
        var_nj1_dn1 = assign19440_e27721_d_n1;
        var_nj1_dn2 = assign19440_e27721_d_n2;
        var_nj1_dn3 = assign19440_e27721_d_n3;
        var_nj1_dn4 = assign19440_e27721_d_n4;
        var_nj1_dn5 = assign19440_e27721_d_n5;
        var_nj1_db0 = assign19440_e27721_d_b0;
        var_nj1_db1 = assign19440_e27721_d_b1;
        var_nj1_db2 = assign19440_e27721_d_b2;
        var_nj1_db3 = assign19440_e27721_d_b3;

        let (assign19450_e27735, assign19450_e27735_d_n0, assign19450_e27735_d_n1, assign19450_e27735_d_n2, assign19450_e27735_d_n3, assign19450_e27735_d_n4, assign19450_e27735_d_n5, assign19450_e27735_d_b0, assign19450_e27735_d_b1, assign19450_e27735_d_b2, assign19450_e27735_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19450_e27731: f64 = (p.p85 - var_nj0);
        let assign19450_e27733: f64 = (assign19450_e27731 - 0.01);
        (assign19450_e27733, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19450_e27735;
        var_tmf1_dn0 = assign19450_e27735_d_n0;
        var_tmf1_dn1 = assign19450_e27735_d_n1;
        var_tmf1_dn2 = assign19450_e27735_d_n2;
        var_tmf1_dn3 = assign19450_e27735_d_n3;
        var_tmf1_dn4 = assign19450_e27735_d_n4;
        var_tmf1_dn5 = assign19450_e27735_d_n5;
        var_tmf1_db0 = assign19450_e27735_d_b0;
        var_tmf1_db1 = assign19450_e27735_d_b1;
        var_tmf1_db2 = assign19450_e27735_d_b2;
        var_tmf1_db3 = assign19450_e27735_d_b3;

        let (assign19460_e27749, assign19460_e27749_d_n0, assign19460_e27749_d_n1, assign19460_e27749_d_n2, assign19460_e27749_d_n3, assign19460_e27749_d_n4, assign19460_e27749_d_n5, assign19460_e27749_d_b0, assign19460_e27749_d_b1, assign19460_e27749_d_b2, assign19460_e27749_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19460_e27745: f64 = (4.0 * p.p85);
        let assign19460_e27747: f64 = (assign19460_e27745 * 0.01);
        (assign19460_e27747, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19460_e27749;
        var_tmf2_dn0 = assign19460_e27749_d_n0;
        var_tmf2_dn1 = assign19460_e27749_d_n1;
        var_tmf2_dn2 = assign19460_e27749_d_n2;
        var_tmf2_dn3 = assign19460_e27749_d_n3;
        var_tmf2_dn4 = assign19460_e27749_d_n4;
        var_tmf2_dn5 = assign19460_e27749_d_n5;
        var_tmf2_db0 = assign19460_e27749_d_b0;
        var_tmf2_db1 = assign19460_e27749_d_b1;
        var_tmf2_db2 = assign19460_e27749_d_b2;
        var_tmf2_db3 = assign19460_e27749_d_b3;

        let (assign19470_e27765, assign19470_e27765_d_n0, assign19470_e27765_d_n1, assign19470_e27765_d_n2, assign19470_e27765_d_n3, assign19470_e27765_d_n4, assign19470_e27765_d_n5, assign19470_e27765_d_b0, assign19470_e27765_d_b1, assign19470_e27765_d_b2, assign19470_e27765_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n1, assign19470_e27763_d_n2, assign19470_e27763_d_n3, assign19470_e27763_d_n4, assign19470_e27763_d_n5, assign19470_e27763_d_b0, assign19470_e27763_d_b1, assign19470_e27763_d_b2, assign19470_e27763_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19470_e27762: f64 = (-var_tmf2);
                (assign19470_e27762, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19470_e27763, assign19470_e27763_d_n0, assign19470_e27763_d_n1, assign19470_e27763_d_n2, assign19470_e27763_d_n3, assign19470_e27763_d_n4, assign19470_e27763_d_n5, assign19470_e27763_d_b0, assign19470_e27763_d_b1, assign19470_e27763_d_b2, assign19470_e27763_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19470_e27765;
        var_tmf2_dn0 = assign19470_e27765_d_n0;
        var_tmf2_dn1 = assign19470_e27765_d_n1;
        var_tmf2_dn2 = assign19470_e27765_d_n2;
        var_tmf2_dn3 = assign19470_e27765_d_n3;
        var_tmf2_dn4 = assign19470_e27765_d_n4;
        var_tmf2_dn5 = assign19470_e27765_d_n5;
        var_tmf2_db0 = assign19470_e27765_d_b0;
        var_tmf2_db1 = assign19470_e27765_d_b1;
        var_tmf2_db2 = assign19470_e27765_d_b2;
        var_tmf2_db3 = assign19470_e27765_d_b3;

        let (assign19480_e27780, assign19480_e27780_d_n0, assign19480_e27780_d_n1, assign19480_e27780_d_n2, assign19480_e27780_d_n3, assign19480_e27780_d_n4, assign19480_e27780_d_n5, assign19480_e27780_d_b0, assign19480_e27780_d_b1, assign19480_e27780_d_b2, assign19480_e27780_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19480_e27775: f64 = (var_tmf1 * var_tmf1);
        let assign19480_e27777: f64 = (assign19480_e27775 + var_tmf2);
        let assign19480_e27778: f64 = (assign19480_e27777).sqrt();
        (assign19480_e27778, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19480_e27778)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19480_e27778)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19480_e27778)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19480_e27778)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19480_e27778)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19480_e27778)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19480_e27780;
        var_tmf2_dn0 = assign19480_e27780_d_n0;
        var_tmf2_dn1 = assign19480_e27780_d_n1;
        var_tmf2_dn2 = assign19480_e27780_d_n2;
        var_tmf2_dn3 = assign19480_e27780_d_n3;
        var_tmf2_dn4 = assign19480_e27780_d_n4;
        var_tmf2_dn5 = assign19480_e27780_d_n5;
        var_tmf2_db0 = assign19480_e27780_d_b0;
        var_tmf2_db1 = assign19480_e27780_d_b1;
        var_tmf2_db2 = assign19480_e27780_d_b2;
        var_tmf2_db3 = assign19480_e27780_d_b3;

        let (assign19490_e27796, assign19490_e27796_d_n0, assign19490_e27796_d_n1, assign19490_e27796_d_n2, assign19490_e27796_d_n3, assign19490_e27796_d_n4, assign19490_e27796_d_n5, assign19490_e27796_d_b0, assign19490_e27796_d_b1, assign19490_e27796_d_b2, assign19490_e27796_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19490_e27792: f64 = (var_tmf1 + var_tmf2);
        let assign19490_e27793: f64 = (0.5 * assign19490_e27792);
        let assign19490_e27794: f64 = (p.p85 - assign19490_e27793);
        (assign19490_e27794, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19490_e27796;
        var_nj0_dn0 = assign19490_e27796_d_n0;
        var_nj0_dn1 = assign19490_e27796_d_n1;
        var_nj0_dn2 = assign19490_e27796_d_n2;
        var_nj0_dn3 = assign19490_e27796_d_n3;
        var_nj0_dn4 = assign19490_e27796_d_n4;
        var_nj0_dn5 = assign19490_e27796_d_n5;
        var_nj0_db0 = assign19490_e27796_d_b0;
        var_nj0_db1 = assign19490_e27796_d_b1;
        var_nj0_db2 = assign19490_e27796_d_b2;
        var_nj0_db3 = assign19490_e27796_d_b3;

        let (assign19500_e27810, assign19500_e27810_d_n0, assign19500_e27810_d_n1, assign19500_e27810_d_n2, assign19500_e27810_d_n3, assign19500_e27810_d_n4, assign19500_e27810_d_n5, assign19500_e27810_d_b0, assign19500_e27810_d_b1, assign19500_e27810_d_b2, assign19500_e27810_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19500_e27806: f64 = (var_nj0 - var_nfagat_i);
        let assign19500_e27808: f64 = (assign19500_e27806 - 0.01);
        (assign19500_e27808, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19500_e27810;
        var_tmf1_dn0 = assign19500_e27810_d_n0;
        var_tmf1_dn1 = assign19500_e27810_d_n1;
        var_tmf1_dn2 = assign19500_e27810_d_n2;
        var_tmf1_dn3 = assign19500_e27810_d_n3;
        var_tmf1_dn4 = assign19500_e27810_d_n4;
        var_tmf1_dn5 = assign19500_e27810_d_n5;
        var_tmf1_db0 = assign19500_e27810_d_b0;
        var_tmf1_db1 = assign19500_e27810_d_b1;
        var_tmf1_db2 = assign19500_e27810_d_b2;
        var_tmf1_db3 = assign19500_e27810_d_b3;

        let (assign19510_e27824, assign19510_e27824_d_n0, assign19510_e27824_d_n1, assign19510_e27824_d_n2, assign19510_e27824_d_n3, assign19510_e27824_d_n4, assign19510_e27824_d_n5, assign19510_e27824_d_b0, assign19510_e27824_d_b1, assign19510_e27824_d_b2, assign19510_e27824_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19510_e27820: f64 = (4.0 * var_nfagat_i);
        let assign19510_e27822: f64 = (assign19510_e27820 * 0.01);
        (assign19510_e27822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19510_e27824;
        var_tmf2_dn0 = assign19510_e27824_d_n0;
        var_tmf2_dn1 = assign19510_e27824_d_n1;
        var_tmf2_dn2 = assign19510_e27824_d_n2;
        var_tmf2_dn3 = assign19510_e27824_d_n3;
        var_tmf2_dn4 = assign19510_e27824_d_n4;
        var_tmf2_dn5 = assign19510_e27824_d_n5;
        var_tmf2_db0 = assign19510_e27824_d_b0;
        var_tmf2_db1 = assign19510_e27824_d_b1;
        var_tmf2_db2 = assign19510_e27824_d_b2;
        var_tmf2_db3 = assign19510_e27824_d_b3;

        let (assign19520_e27840, assign19520_e27840_d_n0, assign19520_e27840_d_n1, assign19520_e27840_d_n2, assign19520_e27840_d_n3, assign19520_e27840_d_n4, assign19520_e27840_d_n5, assign19520_e27840_d_b0, assign19520_e27840_d_b1, assign19520_e27840_d_b2, assign19520_e27840_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n1, assign19520_e27838_d_n2, assign19520_e27838_d_n3, assign19520_e27838_d_n4, assign19520_e27838_d_n5, assign19520_e27838_d_b0, assign19520_e27838_d_b1, assign19520_e27838_d_b2, assign19520_e27838_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19520_e27837: f64 = (-var_tmf2);
                (assign19520_e27837, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19520_e27838, assign19520_e27838_d_n0, assign19520_e27838_d_n1, assign19520_e27838_d_n2, assign19520_e27838_d_n3, assign19520_e27838_d_n4, assign19520_e27838_d_n5, assign19520_e27838_d_b0, assign19520_e27838_d_b1, assign19520_e27838_d_b2, assign19520_e27838_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19520_e27840;
        var_tmf2_dn0 = assign19520_e27840_d_n0;
        var_tmf2_dn1 = assign19520_e27840_d_n1;
        var_tmf2_dn2 = assign19520_e27840_d_n2;
        var_tmf2_dn3 = assign19520_e27840_d_n3;
        var_tmf2_dn4 = assign19520_e27840_d_n4;
        var_tmf2_dn5 = assign19520_e27840_d_n5;
        var_tmf2_db0 = assign19520_e27840_d_b0;
        var_tmf2_db1 = assign19520_e27840_d_b1;
        var_tmf2_db2 = assign19520_e27840_d_b2;
        var_tmf2_db3 = assign19520_e27840_d_b3;

        let (assign19530_e27855, assign19530_e27855_d_n0, assign19530_e27855_d_n1, assign19530_e27855_d_n2, assign19530_e27855_d_n3, assign19530_e27855_d_n4, assign19530_e27855_d_n5, assign19530_e27855_d_b0, assign19530_e27855_d_b1, assign19530_e27855_d_b2, assign19530_e27855_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19530_e27850: f64 = (var_tmf1 * var_tmf1);
        let assign19530_e27852: f64 = (assign19530_e27850 + var_tmf2);
        let assign19530_e27853: f64 = (assign19530_e27852).sqrt();
        (assign19530_e27853, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19530_e27853)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19530_e27853)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19530_e27853)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19530_e27853)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19530_e27853)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19530_e27853)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19530_e27855;
        var_tmf2_dn0 = assign19530_e27855_d_n0;
        var_tmf2_dn1 = assign19530_e27855_d_n1;
        var_tmf2_dn2 = assign19530_e27855_d_n2;
        var_tmf2_dn3 = assign19530_e27855_d_n3;
        var_tmf2_dn4 = assign19530_e27855_d_n4;
        var_tmf2_dn5 = assign19530_e27855_d_n5;
        var_tmf2_db0 = assign19530_e27855_d_b0;
        var_tmf2_db1 = assign19530_e27855_d_b1;
        var_tmf2_db2 = assign19530_e27855_d_b2;
        var_tmf2_db3 = assign19530_e27855_d_b3;

        let (assign19540_e27871, assign19540_e27871_d_n0, assign19540_e27871_d_n1, assign19540_e27871_d_n2, assign19540_e27871_d_n3, assign19540_e27871_d_n4, assign19540_e27871_d_n5, assign19540_e27871_d_b0, assign19540_e27871_d_b1, assign19540_e27871_d_b2, assign19540_e27871_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 != 0.0)) {
        let assign19540_e27867: f64 = (var_tmf1 + var_tmf2);
        let assign19540_e27868: f64 = (0.5 * assign19540_e27867);
        let assign19540_e27869: f64 = (var_nfagat_i + assign19540_e27868);
        (assign19540_e27869, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19540_e27871;
        var_nj0_dn0 = assign19540_e27871_d_n0;
        var_nj0_dn1 = assign19540_e27871_d_n1;
        var_nj0_dn2 = assign19540_e27871_d_n2;
        var_nj0_dn3 = assign19540_e27871_d_n3;
        var_nj0_dn4 = assign19540_e27871_d_n4;
        var_nj0_dn5 = assign19540_e27871_d_n5;
        var_nj0_db0 = assign19540_e27871_d_b0;
        var_nj0_db1 = assign19540_e27871_d_b1;
        var_nj0_db2 = assign19540_e27871_d_b2;
        var_nj0_db3 = assign19540_e27871_d_b3;

        let (assign19550_e27882, assign19550_e27882_d_n0, assign19550_e27882_d_n1, assign19550_e27882_d_n2, assign19550_e27882_d_n3, assign19550_e27882_d_n4, assign19550_e27882_d_n5, assign19550_e27882_d_b0, assign19550_e27882_d_b1, assign19550_e27882_d_b2, assign19550_e27882_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19550_e27882;
        var_nj0_dn0 = assign19550_e27882_d_n0;
        var_nj0_dn1 = assign19550_e27882_d_n1;
        var_nj0_dn2 = assign19550_e27882_d_n2;
        var_nj0_dn3 = assign19550_e27882_d_n3;
        var_nj0_dn4 = assign19550_e27882_d_n4;
        var_nj0_dn5 = assign19550_e27882_d_n5;
        var_nj0_db0 = assign19550_e27882_d_b0;
        var_nj0_db1 = assign19550_e27882_d_b1;
        var_nj0_db2 = assign19550_e27882_d_b2;
        var_nj0_db3 = assign19550_e27882_d_b3;

        let (assign19560_e27893, assign19560_e27893_d_n0, assign19560_e27893_d_n1, assign19560_e27893_d_n2, assign19560_e27893_d_n3, assign19560_e27893_d_n4, assign19560_e27893_d_n5, assign19560_e27893_d_b0, assign19560_e27893_d_b1, assign19560_e27893_d_b2, assign19560_e27893_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) && (var_guard317 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19560_e27893;
        var_nj1_dn0 = assign19560_e27893_d_n0;
        var_nj1_dn1 = assign19560_e27893_d_n1;
        var_nj1_dn2 = assign19560_e27893_d_n2;
        var_nj1_dn3 = assign19560_e27893_d_n3;
        var_nj1_dn4 = assign19560_e27893_d_n4;
        var_nj1_dn5 = assign19560_e27893_d_n5;
        var_nj1_db0 = assign19560_e27893_d_b0;
        var_nj1_db1 = assign19560_e27893_d_b1;
        var_nj1_db2 = assign19560_e27893_d_b2;
        var_nj1_db3 = assign19560_e27893_d_b3;

        let (assign19630_e28143,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19630_e28139: f64 = (var_nin * var_nin);
        let assign19630_e28141: f64 = (assign19630_e28139 / var_ndibot_i);
        (assign19630_e28141,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign19630_e28143;

        let (assign19640_e28159,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19640_e28152: f64 = (var_nfabot_i / var_phitdinv);
        let assign19640_e28155: f64 = (var_ndibot_i / var_pnn0);
        let assign19640_e28156: f64 = (assign19640_e28155).ln();
        let assign19640_e28157: f64 = (assign19640_e28152 * assign19640_e28156);
        (assign19640_e28157,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign19640_e28159;

        let assign19650_e28162: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard320 = assign19650_e28162;

        let (assign19660_e28179, assign19660_e28179_d_n0, assign19660_e28179_d_n1, assign19660_e28179_d_n2, assign19660_e28179_d_n3, assign19660_e28179_d_n4, assign19660_e28179_d_n5, assign19660_e28179_d_b0, assign19660_e28179_d_b1, assign19660_e28179_d_b2, assign19660_e28179_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19660_e28174: f64 = (var_vmax - var_vha1);
        let assign19660_e28175: f64 = (p.p86 * assign19660_e28174);
        let assign19660_e28177: f64 = (assign19660_e28175 + var_nfabot_i);
        (assign19660_e28177, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign19660_e28179;
        var_nja10_dn0 = assign19660_e28179_d_n0;
        var_nja10_dn1 = assign19660_e28179_d_n1;
        var_nja10_dn2 = assign19660_e28179_d_n2;
        var_nja10_dn3 = assign19660_e28179_d_n3;
        var_nja10_dn4 = assign19660_e28179_d_n4;
        var_nja10_dn5 = assign19660_e28179_d_n5;
        var_nja10_db0 = assign19660_e28179_d_b0;
        var_nja10_db1 = assign19660_e28179_d_b1;
        var_nja10_db2 = assign19660_e28179_d_b2;
        var_nja10_db3 = assign19660_e28179_d_b3;

        let (assign19670_e28194, assign19670_e28194_d_n0, assign19670_e28194_d_n1, assign19670_e28194_d_n2, assign19670_e28194_d_n3, assign19670_e28194_d_n4, assign19670_e28194_d_n5, assign19670_e28194_d_b0, assign19670_e28194_d_b1, assign19670_e28194_d_b2, assign19670_e28194_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19670_e28191: f64 = (p.p86 * var_vha1);
        let assign19670_e28192: f64 = (var_nfabot_i - assign19670_e28191);
        (assign19670_e28192, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19670_e28194;
        var_nj0_dn0 = assign19670_e28194_d_n0;
        var_nj0_dn1 = assign19670_e28194_d_n1;
        var_nj0_dn2 = assign19670_e28194_d_n2;
        var_nj0_dn3 = assign19670_e28194_d_n3;
        var_nj0_dn4 = assign19670_e28194_d_n4;
        var_nj0_dn5 = assign19670_e28194_d_n5;
        var_nj0_db0 = assign19670_e28194_d_b0;
        var_nj0_db1 = assign19670_e28194_d_b1;
        var_nj0_db2 = assign19670_e28194_d_b2;
        var_nj0_db3 = assign19670_e28194_d_b3;

        let (assign19680_e28209, assign19680_e28209_d_n0, assign19680_e28209_d_n1, assign19680_e28209_d_n2, assign19680_e28209_d_n3, assign19680_e28209_d_n4, assign19680_e28209_d_n5, assign19680_e28209_d_b0, assign19680_e28209_d_b1, assign19680_e28209_d_b2, assign19680_e28209_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19680_e28205: f64 = (p.p85 - var_nja10);
        let assign19680_e28207: f64 = (assign19680_e28205 - 0.01);
        (assign19680_e28207, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19680_e28209;
        var_tmf1_dn0 = assign19680_e28209_d_n0;
        var_tmf1_dn1 = assign19680_e28209_d_n1;
        var_tmf1_dn2 = assign19680_e28209_d_n2;
        var_tmf1_dn3 = assign19680_e28209_d_n3;
        var_tmf1_dn4 = assign19680_e28209_d_n4;
        var_tmf1_dn5 = assign19680_e28209_d_n5;
        var_tmf1_db0 = assign19680_e28209_d_b0;
        var_tmf1_db1 = assign19680_e28209_d_b1;
        var_tmf1_db2 = assign19680_e28209_d_b2;
        var_tmf1_db3 = assign19680_e28209_d_b3;

        let (assign19690_e28224, assign19690_e28224_d_n0, assign19690_e28224_d_n1, assign19690_e28224_d_n2, assign19690_e28224_d_n3, assign19690_e28224_d_n4, assign19690_e28224_d_n5, assign19690_e28224_d_b0, assign19690_e28224_d_b1, assign19690_e28224_d_b2, assign19690_e28224_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19690_e28220: f64 = (4.0 * p.p85);
        let assign19690_e28222: f64 = (assign19690_e28220 * 0.01);
        (assign19690_e28222, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19690_e28224;
        var_tmf2_dn0 = assign19690_e28224_d_n0;
        var_tmf2_dn1 = assign19690_e28224_d_n1;
        var_tmf2_dn2 = assign19690_e28224_d_n2;
        var_tmf2_dn3 = assign19690_e28224_d_n3;
        var_tmf2_dn4 = assign19690_e28224_d_n4;
        var_tmf2_dn5 = assign19690_e28224_d_n5;
        var_tmf2_db0 = assign19690_e28224_d_b0;
        var_tmf2_db1 = assign19690_e28224_d_b1;
        var_tmf2_db2 = assign19690_e28224_d_b2;
        var_tmf2_db3 = assign19690_e28224_d_b3;

        *var_guard320_slot = var_guard320;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard320: f64,
        var_nfabot_i: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_db0_slot: &mut f64,
        var_dfn_sl_db1_slot: &mut f64,
        var_dfn_sl_db2_slot: &mut f64,
        var_dfn_sl_db3_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn1_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_dn3_slot: &mut f64,
        var_dfn_sl_dn4_slot: &mut f64,
        var_dfn_sl_dn5_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_db0: f64 = *var_dfn_sl_db0_slot;
        let mut var_dfn_sl_db1: f64 = *var_dfn_sl_db1_slot;
        let mut var_dfn_sl_db2: f64 = *var_dfn_sl_db2_slot;
        let mut var_dfn_sl_db3: f64 = *var_dfn_sl_db3_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn1: f64 = *var_dfn_sl_dn1_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_dn3: f64 = *var_dfn_sl_dn3_slot;
        let mut var_dfn_sl_dn4: f64 = *var_dfn_sl_dn4_slot;
        let mut var_dfn_sl_dn5: f64 = *var_dfn_sl_dn5_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;

        let (assign19700_e28241, assign19700_e28241_d_n0, assign19700_e28241_d_n1, assign19700_e28241_d_n2, assign19700_e28241_d_n3, assign19700_e28241_d_n4, assign19700_e28241_d_n5, assign19700_e28241_d_b0, assign19700_e28241_d_b1, assign19700_e28241_d_b2, assign19700_e28241_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n1, assign19700_e28239_d_n2, assign19700_e28239_d_n3, assign19700_e28239_d_n4, assign19700_e28239_d_n5, assign19700_e28239_d_b0, assign19700_e28239_d_b1, assign19700_e28239_d_b2, assign19700_e28239_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19700_e28238: f64 = (-var_tmf2);
                (assign19700_e28238, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19700_e28239, assign19700_e28239_d_n0, assign19700_e28239_d_n1, assign19700_e28239_d_n2, assign19700_e28239_d_n3, assign19700_e28239_d_n4, assign19700_e28239_d_n5, assign19700_e28239_d_b0, assign19700_e28239_d_b1, assign19700_e28239_d_b2, assign19700_e28239_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19700_e28241;
        var_tmf2_dn0 = assign19700_e28241_d_n0;
        var_tmf2_dn1 = assign19700_e28241_d_n1;
        var_tmf2_dn2 = assign19700_e28241_d_n2;
        var_tmf2_dn3 = assign19700_e28241_d_n3;
        var_tmf2_dn4 = assign19700_e28241_d_n4;
        var_tmf2_dn5 = assign19700_e28241_d_n5;
        var_tmf2_db0 = assign19700_e28241_d_b0;
        var_tmf2_db1 = assign19700_e28241_d_b1;
        var_tmf2_db2 = assign19700_e28241_d_b2;
        var_tmf2_db3 = assign19700_e28241_d_b3;

        let (assign19710_e28257, assign19710_e28257_d_n0, assign19710_e28257_d_n1, assign19710_e28257_d_n2, assign19710_e28257_d_n3, assign19710_e28257_d_n4, assign19710_e28257_d_n5, assign19710_e28257_d_b0, assign19710_e28257_d_b1, assign19710_e28257_d_b2, assign19710_e28257_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19710_e28252: f64 = (var_tmf1 * var_tmf1);
        let assign19710_e28254: f64 = (assign19710_e28252 + var_tmf2);
        let assign19710_e28255: f64 = (assign19710_e28254).sqrt();
        (assign19710_e28255, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19710_e28255)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19710_e28255)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19710_e28255)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19710_e28255)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19710_e28255)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19710_e28255)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19710_e28257;
        var_tmf2_dn0 = assign19710_e28257_d_n0;
        var_tmf2_dn1 = assign19710_e28257_d_n1;
        var_tmf2_dn2 = assign19710_e28257_d_n2;
        var_tmf2_dn3 = assign19710_e28257_d_n3;
        var_tmf2_dn4 = assign19710_e28257_d_n4;
        var_tmf2_dn5 = assign19710_e28257_d_n5;
        var_tmf2_db0 = assign19710_e28257_d_b0;
        var_tmf2_db1 = assign19710_e28257_d_b1;
        var_tmf2_db2 = assign19710_e28257_d_b2;
        var_tmf2_db3 = assign19710_e28257_d_b3;

        let (assign19720_e28274, assign19720_e28274_d_n0, assign19720_e28274_d_n1, assign19720_e28274_d_n2, assign19720_e28274_d_n3, assign19720_e28274_d_n4, assign19720_e28274_d_n5, assign19720_e28274_d_b0, assign19720_e28274_d_b1, assign19720_e28274_d_b2, assign19720_e28274_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19720_e28270: f64 = (var_tmf1 / var_tmf2);
        let assign19720_e28271: f64 = (1.0 + assign19720_e28270);
        let assign19720_e28272: f64 = (0.5 * assign19720_e28271);
        (assign19720_e28272, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign19720_e28274;
        var_dfn_su_dn0 = assign19720_e28274_d_n0;
        var_dfn_su_dn1 = assign19720_e28274_d_n1;
        var_dfn_su_dn2 = assign19720_e28274_d_n2;
        var_dfn_su_dn3 = assign19720_e28274_d_n3;
        var_dfn_su_dn4 = assign19720_e28274_d_n4;
        var_dfn_su_dn5 = assign19720_e28274_d_n5;
        var_dfn_su_db0 = assign19720_e28274_d_b0;
        var_dfn_su_db1 = assign19720_e28274_d_b1;
        var_dfn_su_db2 = assign19720_e28274_d_b2;
        var_dfn_su_db3 = assign19720_e28274_d_b3;

        let (assign19730_e28291, assign19730_e28291_d_n0, assign19730_e28291_d_n1, assign19730_e28291_d_n2, assign19730_e28291_d_n3, assign19730_e28291_d_n4, assign19730_e28291_d_n5, assign19730_e28291_d_b0, assign19730_e28291_d_b1, assign19730_e28291_d_b2, assign19730_e28291_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19730_e28287: f64 = (var_tmf1 + var_tmf2);
        let assign19730_e28288: f64 = (0.5 * assign19730_e28287);
        let assign19730_e28289: f64 = (p.p85 - assign19730_e28288);
        (assign19730_e28289, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign19730_e28291;
        var_nja11_dn0 = assign19730_e28291_d_n0;
        var_nja11_dn1 = assign19730_e28291_d_n1;
        var_nja11_dn2 = assign19730_e28291_d_n2;
        var_nja11_dn3 = assign19730_e28291_d_n3;
        var_nja11_dn4 = assign19730_e28291_d_n4;
        var_nja11_dn5 = assign19730_e28291_d_n5;
        var_nja11_db0 = assign19730_e28291_d_b0;
        var_nja11_db1 = assign19730_e28291_d_b1;
        var_nja11_db2 = assign19730_e28291_d_b2;
        var_nja11_db3 = assign19730_e28291_d_b3;

        let (assign19740_e28306, assign19740_e28306_d_n0, assign19740_e28306_d_n1, assign19740_e28306_d_n2, assign19740_e28306_d_n3, assign19740_e28306_d_n4, assign19740_e28306_d_n5, assign19740_e28306_d_b0, assign19740_e28306_d_b1, assign19740_e28306_d_b2, assign19740_e28306_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19740_e28302: f64 = (var_nja11 - var_nfabot_i);
        let assign19740_e28304: f64 = (assign19740_e28302 - 0.01);
        (assign19740_e28304, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19740_e28306;
        var_tmf1_dn0 = assign19740_e28306_d_n0;
        var_tmf1_dn1 = assign19740_e28306_d_n1;
        var_tmf1_dn2 = assign19740_e28306_d_n2;
        var_tmf1_dn3 = assign19740_e28306_d_n3;
        var_tmf1_dn4 = assign19740_e28306_d_n4;
        var_tmf1_dn5 = assign19740_e28306_d_n5;
        var_tmf1_db0 = assign19740_e28306_d_b0;
        var_tmf1_db1 = assign19740_e28306_d_b1;
        var_tmf1_db2 = assign19740_e28306_d_b2;
        var_tmf1_db3 = assign19740_e28306_d_b3;

        let (assign19750_e28321, assign19750_e28321_d_n0, assign19750_e28321_d_n1, assign19750_e28321_d_n2, assign19750_e28321_d_n3, assign19750_e28321_d_n4, assign19750_e28321_d_n5, assign19750_e28321_d_b0, assign19750_e28321_d_b1, assign19750_e28321_d_b2, assign19750_e28321_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19750_e28317: f64 = (4.0 * var_nfabot_i);
        let assign19750_e28319: f64 = (assign19750_e28317 * 0.01);
        (assign19750_e28319, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19750_e28321;
        var_tmf2_dn0 = assign19750_e28321_d_n0;
        var_tmf2_dn1 = assign19750_e28321_d_n1;
        var_tmf2_dn2 = assign19750_e28321_d_n2;
        var_tmf2_dn3 = assign19750_e28321_d_n3;
        var_tmf2_dn4 = assign19750_e28321_d_n4;
        var_tmf2_dn5 = assign19750_e28321_d_n5;
        var_tmf2_db0 = assign19750_e28321_d_b0;
        var_tmf2_db1 = assign19750_e28321_d_b1;
        var_tmf2_db2 = assign19750_e28321_d_b2;
        var_tmf2_db3 = assign19750_e28321_d_b3;

        let (assign19760_e28338, assign19760_e28338_d_n0, assign19760_e28338_d_n1, assign19760_e28338_d_n2, assign19760_e28338_d_n3, assign19760_e28338_d_n4, assign19760_e28338_d_n5, assign19760_e28338_d_b0, assign19760_e28338_d_b1, assign19760_e28338_d_b2, assign19760_e28338_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n1, assign19760_e28336_d_n2, assign19760_e28336_d_n3, assign19760_e28336_d_n4, assign19760_e28336_d_n5, assign19760_e28336_d_b0, assign19760_e28336_d_b1, assign19760_e28336_d_b2, assign19760_e28336_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19760_e28335: f64 = (-var_tmf2);
                (assign19760_e28335, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19760_e28336, assign19760_e28336_d_n0, assign19760_e28336_d_n1, assign19760_e28336_d_n2, assign19760_e28336_d_n3, assign19760_e28336_d_n4, assign19760_e28336_d_n5, assign19760_e28336_d_b0, assign19760_e28336_d_b1, assign19760_e28336_d_b2, assign19760_e28336_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19760_e28338;
        var_tmf2_dn0 = assign19760_e28338_d_n0;
        var_tmf2_dn1 = assign19760_e28338_d_n1;
        var_tmf2_dn2 = assign19760_e28338_d_n2;
        var_tmf2_dn3 = assign19760_e28338_d_n3;
        var_tmf2_dn4 = assign19760_e28338_d_n4;
        var_tmf2_dn5 = assign19760_e28338_d_n5;
        var_tmf2_db0 = assign19760_e28338_d_b0;
        var_tmf2_db1 = assign19760_e28338_d_b1;
        var_tmf2_db2 = assign19760_e28338_d_b2;
        var_tmf2_db3 = assign19760_e28338_d_b3;

        let (assign19770_e28354, assign19770_e28354_d_n0, assign19770_e28354_d_n1, assign19770_e28354_d_n2, assign19770_e28354_d_n3, assign19770_e28354_d_n4, assign19770_e28354_d_n5, assign19770_e28354_d_b0, assign19770_e28354_d_b1, assign19770_e28354_d_b2, assign19770_e28354_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19770_e28349: f64 = (var_tmf1 * var_tmf1);
        let assign19770_e28351: f64 = (assign19770_e28349 + var_tmf2);
        let assign19770_e28352: f64 = (assign19770_e28351).sqrt();
        (assign19770_e28352, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19770_e28352)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19770_e28352)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19770_e28352)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19770_e28352)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19770_e28352)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19770_e28352)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19770_e28354;
        var_tmf2_dn0 = assign19770_e28354_d_n0;
        var_tmf2_dn1 = assign19770_e28354_d_n1;
        var_tmf2_dn2 = assign19770_e28354_d_n2;
        var_tmf2_dn3 = assign19770_e28354_d_n3;
        var_tmf2_dn4 = assign19770_e28354_d_n4;
        var_tmf2_dn5 = assign19770_e28354_d_n5;
        var_tmf2_db0 = assign19770_e28354_d_b0;
        var_tmf2_db1 = assign19770_e28354_d_b1;
        var_tmf2_db2 = assign19770_e28354_d_b2;
        var_tmf2_db3 = assign19770_e28354_d_b3;

        let (assign19780_e28371, assign19780_e28371_d_n0, assign19780_e28371_d_n1, assign19780_e28371_d_n2, assign19780_e28371_d_n3, assign19780_e28371_d_n4, assign19780_e28371_d_n5, assign19780_e28371_d_b0, assign19780_e28371_d_b1, assign19780_e28371_d_b2, assign19780_e28371_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19780_e28367: f64 = (var_tmf1 / var_tmf2);
        let assign19780_e28368: f64 = (1.0 + assign19780_e28367);
        let assign19780_e28369: f64 = (0.5 * assign19780_e28368);
        (assign19780_e28369, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign19780_e28371;
        var_dfn_sl_dn0 = assign19780_e28371_d_n0;
        var_dfn_sl_dn1 = assign19780_e28371_d_n1;
        var_dfn_sl_dn2 = assign19780_e28371_d_n2;
        var_dfn_sl_dn3 = assign19780_e28371_d_n3;
        var_dfn_sl_dn4 = assign19780_e28371_d_n4;
        var_dfn_sl_dn5 = assign19780_e28371_d_n5;
        var_dfn_sl_db0 = assign19780_e28371_d_b0;
        var_dfn_sl_db1 = assign19780_e28371_d_b1;
        var_dfn_sl_db2 = assign19780_e28371_d_b2;
        var_dfn_sl_db3 = assign19780_e28371_d_b3;

        let (assign19790_e28388, assign19790_e28388_d_n0, assign19790_e28388_d_n1, assign19790_e28388_d_n2, assign19790_e28388_d_n3, assign19790_e28388_d_n4, assign19790_e28388_d_n5, assign19790_e28388_d_b0, assign19790_e28388_d_b1, assign19790_e28388_d_b2, assign19790_e28388_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19790_e28384: f64 = (var_tmf1 + var_tmf2);
        let assign19790_e28385: f64 = (0.5 * assign19790_e28384);
        let assign19790_e28386: f64 = (var_nfabot_i + assign19790_e28385);
        (assign19790_e28386, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19790_e28388;
        var_nj1_dn0 = assign19790_e28388_d_n0;
        var_nj1_dn1 = assign19790_e28388_d_n1;
        var_nj1_dn2 = assign19790_e28388_d_n2;
        var_nj1_dn3 = assign19790_e28388_d_n3;
        var_nj1_dn4 = assign19790_e28388_d_n4;
        var_nj1_dn5 = assign19790_e28388_d_n5;
        var_nj1_db0 = assign19790_e28388_d_b0;
        var_nj1_db1 = assign19790_e28388_d_b1;
        var_nj1_db2 = assign19790_e28388_d_b2;
        var_nj1_db3 = assign19790_e28388_d_b3;

        let (assign19800_e28403, assign19800_e28403_d_n0, assign19800_e28403_d_n1, assign19800_e28403_d_n2, assign19800_e28403_d_n3, assign19800_e28403_d_n4, assign19800_e28403_d_n5, assign19800_e28403_d_b0, assign19800_e28403_d_b1, assign19800_e28403_d_b2, assign19800_e28403_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19800_e28399: f64 = (p.p85 - var_nj0);
        let assign19800_e28401: f64 = (assign19800_e28399 - 0.01);
        (assign19800_e28401, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19800_e28403;
        var_tmf1_dn0 = assign19800_e28403_d_n0;
        var_tmf1_dn1 = assign19800_e28403_d_n1;
        var_tmf1_dn2 = assign19800_e28403_d_n2;
        var_tmf1_dn3 = assign19800_e28403_d_n3;
        var_tmf1_dn4 = assign19800_e28403_d_n4;
        var_tmf1_dn5 = assign19800_e28403_d_n5;
        var_tmf1_db0 = assign19800_e28403_d_b0;
        var_tmf1_db1 = assign19800_e28403_d_b1;
        var_tmf1_db2 = assign19800_e28403_d_b2;
        var_tmf1_db3 = assign19800_e28403_d_b3;

        let (assign19810_e28418, assign19810_e28418_d_n0, assign19810_e28418_d_n1, assign19810_e28418_d_n2, assign19810_e28418_d_n3, assign19810_e28418_d_n4, assign19810_e28418_d_n5, assign19810_e28418_d_b0, assign19810_e28418_d_b1, assign19810_e28418_d_b2, assign19810_e28418_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19810_e28414: f64 = (4.0 * p.p85);
        let assign19810_e28416: f64 = (assign19810_e28414 * 0.01);
        (assign19810_e28416, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19810_e28418;
        var_tmf2_dn0 = assign19810_e28418_d_n0;
        var_tmf2_dn1 = assign19810_e28418_d_n1;
        var_tmf2_dn2 = assign19810_e28418_d_n2;
        var_tmf2_dn3 = assign19810_e28418_d_n3;
        var_tmf2_dn4 = assign19810_e28418_d_n4;
        var_tmf2_dn5 = assign19810_e28418_d_n5;
        var_tmf2_db0 = assign19810_e28418_d_b0;
        var_tmf2_db1 = assign19810_e28418_d_b1;
        var_tmf2_db2 = assign19810_e28418_d_b2;
        var_tmf2_db3 = assign19810_e28418_d_b3;

        let (assign19820_e28435, assign19820_e28435_d_n0, assign19820_e28435_d_n1, assign19820_e28435_d_n2, assign19820_e28435_d_n3, assign19820_e28435_d_n4, assign19820_e28435_d_n5, assign19820_e28435_d_b0, assign19820_e28435_d_b1, assign19820_e28435_d_b2, assign19820_e28435_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n1, assign19820_e28433_d_n2, assign19820_e28433_d_n3, assign19820_e28433_d_n4, assign19820_e28433_d_n5, assign19820_e28433_d_b0, assign19820_e28433_d_b1, assign19820_e28433_d_b2, assign19820_e28433_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19820_e28432: f64 = (-var_tmf2);
                (assign19820_e28432, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19820_e28433, assign19820_e28433_d_n0, assign19820_e28433_d_n1, assign19820_e28433_d_n2, assign19820_e28433_d_n3, assign19820_e28433_d_n4, assign19820_e28433_d_n5, assign19820_e28433_d_b0, assign19820_e28433_d_b1, assign19820_e28433_d_b2, assign19820_e28433_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19820_e28435;
        var_tmf2_dn0 = assign19820_e28435_d_n0;
        var_tmf2_dn1 = assign19820_e28435_d_n1;
        var_tmf2_dn2 = assign19820_e28435_d_n2;
        var_tmf2_dn3 = assign19820_e28435_d_n3;
        var_tmf2_dn4 = assign19820_e28435_d_n4;
        var_tmf2_dn5 = assign19820_e28435_d_n5;
        var_tmf2_db0 = assign19820_e28435_d_b0;
        var_tmf2_db1 = assign19820_e28435_d_b1;
        var_tmf2_db2 = assign19820_e28435_d_b2;
        var_tmf2_db3 = assign19820_e28435_d_b3;

        let (assign19830_e28451, assign19830_e28451_d_n0, assign19830_e28451_d_n1, assign19830_e28451_d_n2, assign19830_e28451_d_n3, assign19830_e28451_d_n4, assign19830_e28451_d_n5, assign19830_e28451_d_b0, assign19830_e28451_d_b1, assign19830_e28451_d_b2, assign19830_e28451_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19830_e28446: f64 = (var_tmf1 * var_tmf1);
        let assign19830_e28448: f64 = (assign19830_e28446 + var_tmf2);
        let assign19830_e28449: f64 = (assign19830_e28448).sqrt();
        (assign19830_e28449, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19830_e28449)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19830_e28449)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19830_e28449)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19830_e28449)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19830_e28449)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19830_e28449)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19830_e28451;
        var_tmf2_dn0 = assign19830_e28451_d_n0;
        var_tmf2_dn1 = assign19830_e28451_d_n1;
        var_tmf2_dn2 = assign19830_e28451_d_n2;
        var_tmf2_dn3 = assign19830_e28451_d_n3;
        var_tmf2_dn4 = assign19830_e28451_d_n4;
        var_tmf2_dn5 = assign19830_e28451_d_n5;
        var_tmf2_db0 = assign19830_e28451_d_b0;
        var_tmf2_db1 = assign19830_e28451_d_b1;
        var_tmf2_db2 = assign19830_e28451_d_b2;
        var_tmf2_db3 = assign19830_e28451_d_b3;

        let (assign19840_e28468, assign19840_e28468_d_n0, assign19840_e28468_d_n1, assign19840_e28468_d_n2, assign19840_e28468_d_n3, assign19840_e28468_d_n4, assign19840_e28468_d_n5, assign19840_e28468_d_b0, assign19840_e28468_d_b1, assign19840_e28468_d_b2, assign19840_e28468_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19840_e28464: f64 = (var_tmf1 + var_tmf2);
        let assign19840_e28465: f64 = (0.5 * assign19840_e28464);
        let assign19840_e28466: f64 = (p.p85 - assign19840_e28465);
        (assign19840_e28466, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19840_e28468;
        var_nj0_dn0 = assign19840_e28468_d_n0;
        var_nj0_dn1 = assign19840_e28468_d_n1;
        var_nj0_dn2 = assign19840_e28468_d_n2;
        var_nj0_dn3 = assign19840_e28468_d_n3;
        var_nj0_dn4 = assign19840_e28468_d_n4;
        var_nj0_dn5 = assign19840_e28468_d_n5;
        var_nj0_db0 = assign19840_e28468_d_b0;
        var_nj0_db1 = assign19840_e28468_d_b1;
        var_nj0_db2 = assign19840_e28468_d_b2;
        var_nj0_db3 = assign19840_e28468_d_b3;

        let (assign19850_e28483, assign19850_e28483_d_n0, assign19850_e28483_d_n1, assign19850_e28483_d_n2, assign19850_e28483_d_n3, assign19850_e28483_d_n4, assign19850_e28483_d_n5, assign19850_e28483_d_b0, assign19850_e28483_d_b1, assign19850_e28483_d_b2, assign19850_e28483_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19850_e28479: f64 = (var_nj0 - var_nfabot_i);
        let assign19850_e28481: f64 = (assign19850_e28479 - 0.01);
        (assign19850_e28481, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign19850_e28483;
        var_tmf1_dn0 = assign19850_e28483_d_n0;
        var_tmf1_dn1 = assign19850_e28483_d_n1;
        var_tmf1_dn2 = assign19850_e28483_d_n2;
        var_tmf1_dn3 = assign19850_e28483_d_n3;
        var_tmf1_dn4 = assign19850_e28483_d_n4;
        var_tmf1_dn5 = assign19850_e28483_d_n5;
        var_tmf1_db0 = assign19850_e28483_d_b0;
        var_tmf1_db1 = assign19850_e28483_d_b1;
        var_tmf1_db2 = assign19850_e28483_d_b2;
        var_tmf1_db3 = assign19850_e28483_d_b3;

        let (assign19860_e28498, assign19860_e28498_d_n0, assign19860_e28498_d_n1, assign19860_e28498_d_n2, assign19860_e28498_d_n3, assign19860_e28498_d_n4, assign19860_e28498_d_n5, assign19860_e28498_d_b0, assign19860_e28498_d_b1, assign19860_e28498_d_b2, assign19860_e28498_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19860_e28494: f64 = (4.0 * var_nfabot_i);
        let assign19860_e28496: f64 = (assign19860_e28494 * 0.01);
        (assign19860_e28496, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19860_e28498;
        var_tmf2_dn0 = assign19860_e28498_d_n0;
        var_tmf2_dn1 = assign19860_e28498_d_n1;
        var_tmf2_dn2 = assign19860_e28498_d_n2;
        var_tmf2_dn3 = assign19860_e28498_d_n3;
        var_tmf2_dn4 = assign19860_e28498_d_n4;
        var_tmf2_dn5 = assign19860_e28498_d_n5;
        var_tmf2_db0 = assign19860_e28498_d_b0;
        var_tmf2_db1 = assign19860_e28498_d_b1;
        var_tmf2_db2 = assign19860_e28498_d_b2;
        var_tmf2_db3 = assign19860_e28498_d_b3;

        let (assign19870_e28515, assign19870_e28515_d_n0, assign19870_e28515_d_n1, assign19870_e28515_d_n2, assign19870_e28515_d_n3, assign19870_e28515_d_n4, assign19870_e28515_d_n5, assign19870_e28515_d_b0, assign19870_e28515_d_b1, assign19870_e28515_d_b2, assign19870_e28515_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n1, assign19870_e28513_d_n2, assign19870_e28513_d_n3, assign19870_e28513_d_n4, assign19870_e28513_d_n5, assign19870_e28513_d_b0, assign19870_e28513_d_b1, assign19870_e28513_d_b2, assign19870_e28513_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign19870_e28512: f64 = (-var_tmf2);
                (assign19870_e28512, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign19870_e28513, assign19870_e28513_d_n0, assign19870_e28513_d_n1, assign19870_e28513_d_n2, assign19870_e28513_d_n3, assign19870_e28513_d_n4, assign19870_e28513_d_n5, assign19870_e28513_d_b0, assign19870_e28513_d_b1, assign19870_e28513_d_b2, assign19870_e28513_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19870_e28515;
        var_tmf2_dn0 = assign19870_e28515_d_n0;
        var_tmf2_dn1 = assign19870_e28515_d_n1;
        var_tmf2_dn2 = assign19870_e28515_d_n2;
        var_tmf2_dn3 = assign19870_e28515_d_n3;
        var_tmf2_dn4 = assign19870_e28515_d_n4;
        var_tmf2_dn5 = assign19870_e28515_d_n5;
        var_tmf2_db0 = assign19870_e28515_d_b0;
        var_tmf2_db1 = assign19870_e28515_d_b1;
        var_tmf2_db2 = assign19870_e28515_d_b2;
        var_tmf2_db3 = assign19870_e28515_d_b3;

        let (assign19880_e28531, assign19880_e28531_d_n0, assign19880_e28531_d_n1, assign19880_e28531_d_n2, assign19880_e28531_d_n3, assign19880_e28531_d_n4, assign19880_e28531_d_n5, assign19880_e28531_d_b0, assign19880_e28531_d_b1, assign19880_e28531_d_b2, assign19880_e28531_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19880_e28526: f64 = (var_tmf1 * var_tmf1);
        let assign19880_e28528: f64 = (assign19880_e28526 + var_tmf2);
        let assign19880_e28529: f64 = (assign19880_e28528).sqrt();
        (assign19880_e28529, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign19880_e28529)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign19880_e28529)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign19880_e28529)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign19880_e28529)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign19880_e28529)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign19880_e28529)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign19880_e28531;
        var_tmf2_dn0 = assign19880_e28531_d_n0;
        var_tmf2_dn1 = assign19880_e28531_d_n1;
        var_tmf2_dn2 = assign19880_e28531_d_n2;
        var_tmf2_dn3 = assign19880_e28531_d_n3;
        var_tmf2_dn4 = assign19880_e28531_d_n4;
        var_tmf2_dn5 = assign19880_e28531_d_n5;
        var_tmf2_db0 = assign19880_e28531_d_b0;
        var_tmf2_db1 = assign19880_e28531_d_b1;
        var_tmf2_db2 = assign19880_e28531_d_b2;
        var_tmf2_db3 = assign19880_e28531_d_b3;

        let (assign19890_e28548, assign19890_e28548_d_n0, assign19890_e28548_d_n1, assign19890_e28548_d_n2, assign19890_e28548_d_n3, assign19890_e28548_d_n4, assign19890_e28548_d_n5, assign19890_e28548_d_b0, assign19890_e28548_d_b1, assign19890_e28548_d_b2, assign19890_e28548_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19890_e28544: f64 = (var_tmf1 + var_tmf2);
        let assign19890_e28545: f64 = (0.5 * assign19890_e28544);
        let assign19890_e28546: f64 = (var_nfabot_i + assign19890_e28545);
        (assign19890_e28546, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19890_e28548;
        var_nj0_dn0 = assign19890_e28548_d_n0;
        var_nj0_dn1 = assign19890_e28548_d_n1;
        var_nj0_dn2 = assign19890_e28548_d_n2;
        var_nj0_dn3 = assign19890_e28548_d_n3;
        var_nj0_dn4 = assign19890_e28548_d_n4;
        var_nj0_dn5 = assign19890_e28548_d_n5;
        var_nj0_db0 = assign19890_e28548_d_b0;
        var_nj0_db1 = assign19890_e28548_d_b1;
        var_nj0_db2 = assign19890_e28548_d_b2;
        var_nj0_db3 = assign19890_e28548_d_b3;

        let (assign19900_e28563, assign19900_e28563_d_n0, assign19900_e28563_d_n1, assign19900_e28563_d_n2, assign19900_e28563_d_n3, assign19900_e28563_d_n4, assign19900_e28563_d_n5, assign19900_e28563_d_b0, assign19900_e28563_d_b1, assign19900_e28563_d_b2, assign19900_e28563_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 != 0.0)) {
        let assign19900_e28559: f64 = (p.p86 * var_dfn_su);
        let assign19900_e28561: f64 = (assign19900_e28559 * var_dfn_sl);
        (assign19900_e28561, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign19900_e28559 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign19900_e28563;
        var_dnj1_dv_dn0 = assign19900_e28563_d_n0;
        var_dnj1_dv_dn1 = assign19900_e28563_d_n1;
        var_dnj1_dv_dn2 = assign19900_e28563_d_n2;
        var_dnj1_dv_dn3 = assign19900_e28563_d_n3;
        var_dnj1_dv_dn4 = assign19900_e28563_d_n4;
        var_dnj1_dv_dn5 = assign19900_e28563_d_n5;
        var_dnj1_dv_db0 = assign19900_e28563_d_b0;
        var_dnj1_dv_db1 = assign19900_e28563_d_b1;
        var_dnj1_dv_db2 = assign19900_e28563_d_b2;
        var_dnj1_dv_db3 = assign19900_e28563_d_b3;

        let (assign19910_e28575, assign19910_e28575_d_n0, assign19910_e28575_d_n1, assign19910_e28575_d_n2, assign19910_e28575_d_n3, assign19910_e28575_d_n4, assign19910_e28575_d_n5, assign19910_e28575_d_b0, assign19910_e28575_d_b1, assign19910_e28575_d_b2, assign19910_e28575_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign19910_e28575;
        var_nj0_dn0 = assign19910_e28575_d_n0;
        var_nj0_dn1 = assign19910_e28575_d_n1;
        var_nj0_dn2 = assign19910_e28575_d_n2;
        var_nj0_dn3 = assign19910_e28575_d_n3;
        var_nj0_dn4 = assign19910_e28575_d_n4;
        var_nj0_dn5 = assign19910_e28575_d_n5;
        var_nj0_db0 = assign19910_e28575_d_b0;
        var_nj0_db1 = assign19910_e28575_d_b1;
        var_nj0_db2 = assign19910_e28575_d_b2;
        var_nj0_db3 = assign19910_e28575_d_b3;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_db0_slot = var_dfn_sl_db0;
        *var_dfn_sl_db1_slot = var_dfn_sl_db1;
        *var_dfn_sl_db2_slot = var_dfn_sl_db2;
        *var_dfn_sl_db3_slot = var_dfn_sl_db3;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn1_slot = var_dfn_sl_dn1;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_dn3_slot = var_dfn_sl_dn3;
        *var_dfn_sl_dn4_slot = var_dfn_sl_dn4;
        *var_dfn_sl_dn5_slot = var_dfn_sl_dn5;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard320: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v4: f64,
        var_vmax: f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_db0_slot: &mut f64,
        var_dfn_su_db1_slot: &mut f64,
        var_dfn_su_db2_slot: &mut f64,
        var_dfn_su_db3_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn1_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_dn3_slot: &mut f64,
        var_dfn_su_dn4_slot: &mut f64,
        var_dfn_su_dn5_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_db0_slot: &mut f64,
        var_dnj1_dv_db1_slot: &mut f64,
        var_dnj1_dv_db2_slot: &mut f64,
        var_dnj1_dv_db3_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn1_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_dn3_slot: &mut f64,
        var_dnj1_dv_dn4_slot: &mut f64,
        var_dnj1_dv_dn5_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_db0_slot: &mut f64,
        var_dvmax_over_phitd_dv_db1_slot: &mut f64,
        var_dvmax_over_phitd_dv_db2_slot: &mut f64,
        var_dvmax_over_phitd_dv_db3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn1_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn3_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn4_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn5_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_db3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn1_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn3_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn4_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn5_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_db0_slot: &mut f64,
        var_idmultbot_db1_slot: &mut f64,
        var_idmultbot_db2_slot: &mut f64,
        var_idmultbot_db3_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn1_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_dn3_slot: &mut f64,
        var_idmultbot_dn4_slot: &mut f64,
        var_idmultbot_dn5_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_db0_slot: &mut f64,
        var_nj0_db1_slot: &mut f64,
        var_nj0_db2_slot: &mut f64,
        var_nj0_db3_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn1_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_dn3_slot: &mut f64,
        var_nj0_dn4_slot: &mut f64,
        var_nj0_dn5_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_db0_slot: &mut f64,
        var_nj1_db1_slot: &mut f64,
        var_nj1_db2_slot: &mut f64,
        var_nj1_db3_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn1_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_dn3_slot: &mut f64,
        var_nj1_dn4_slot: &mut f64,
        var_nj1_dn5_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_db0_slot: &mut f64,
        var_nja10_db1_slot: &mut f64,
        var_nja10_db2_slot: &mut f64,
        var_nja10_db3_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn1_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_dn3_slot: &mut f64,
        var_nja10_dn4_slot: &mut f64,
        var_nja10_dn5_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_db0_slot: &mut f64,
        var_nja11_db1_slot: &mut f64,
        var_nja11_db2_slot: &mut f64,
        var_nja11_db3_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn1_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_dn3_slot: &mut f64,
        var_nja11_dn4_slot: &mut f64,
        var_nja11_dn5_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_db0_slot: &mut f64,
        var_tmf1_db1_slot: &mut f64,
        var_tmf1_db2_slot: &mut f64,
        var_tmf1_db3_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn1_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn3_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_db0_slot: &mut f64,
        var_tmf2_db1_slot: &mut f64,
        var_tmf2_db2_slot: &mut f64,
        var_tmf2_db3_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn1_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn3_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_db0: f64 = *var_dfn_su_db0_slot;
        let mut var_dfn_su_db1: f64 = *var_dfn_su_db1_slot;
        let mut var_dfn_su_db2: f64 = *var_dfn_su_db2_slot;
        let mut var_dfn_su_db3: f64 = *var_dfn_su_db3_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn1: f64 = *var_dfn_su_dn1_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_dn3: f64 = *var_dfn_su_dn3_slot;
        let mut var_dfn_su_dn4: f64 = *var_dfn_su_dn4_slot;
        let mut var_dfn_su_dn5: f64 = *var_dfn_su_dn5_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_db0: f64 = *var_dnj1_dv_db0_slot;
        let mut var_dnj1_dv_db1: f64 = *var_dnj1_dv_db1_slot;
        let mut var_dnj1_dv_db2: f64 = *var_dnj1_dv_db2_slot;
        let mut var_dnj1_dv_db3: f64 = *var_dnj1_dv_db3_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn1: f64 = *var_dnj1_dv_dn1_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_dn3: f64 = *var_dnj1_dv_dn3_slot;
        let mut var_dnj1_dv_dn4: f64 = *var_dnj1_dv_dn4_slot;
        let mut var_dnj1_dv_dn5: f64 = *var_dnj1_dv_dn5_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_db0: f64 = *var_dvmax_over_phitd_dv_db0_slot;
        let mut var_dvmax_over_phitd_dv_db1: f64 = *var_dvmax_over_phitd_dv_db1_slot;
        let mut var_dvmax_over_phitd_dv_db2: f64 = *var_dvmax_over_phitd_dv_db2_slot;
        let mut var_dvmax_over_phitd_dv_db3: f64 = *var_dvmax_over_phitd_dv_db3_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn1: f64 = *var_dvmax_over_phitd_dv_dn1_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_dn3: f64 = *var_dvmax_over_phitd_dv_dn3_slot;
        let mut var_dvmax_over_phitd_dv_dn4: f64 = *var_dvmax_over_phitd_dv_dn4_slot;
        let mut var_dvmax_over_phitd_dv_dn5: f64 = *var_dvmax_over_phitd_dv_dn5_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_db0: f64 = *var_exp_vmax_over_phitd_bot_db0_slot;
        let mut var_exp_vmax_over_phitd_bot_db1: f64 = *var_exp_vmax_over_phitd_bot_db1_slot;
        let mut var_exp_vmax_over_phitd_bot_db2: f64 = *var_exp_vmax_over_phitd_bot_db2_slot;
        let mut var_exp_vmax_over_phitd_bot_db3: f64 = *var_exp_vmax_over_phitd_bot_db3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn1: f64 = *var_exp_vmax_over_phitd_bot_dn1_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_dn3: f64 = *var_exp_vmax_over_phitd_bot_dn3_slot;
        let mut var_exp_vmax_over_phitd_bot_dn4: f64 = *var_exp_vmax_over_phitd_bot_dn4_slot;
        let mut var_exp_vmax_over_phitd_bot_dn5: f64 = *var_exp_vmax_over_phitd_bot_dn5_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_db0: f64 = *var_idmultbot_db0_slot;
        let mut var_idmultbot_db1: f64 = *var_idmultbot_db1_slot;
        let mut var_idmultbot_db2: f64 = *var_idmultbot_db2_slot;
        let mut var_idmultbot_db3: f64 = *var_idmultbot_db3_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn1: f64 = *var_idmultbot_dn1_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_dn3: f64 = *var_idmultbot_dn3_slot;
        let mut var_idmultbot_dn4: f64 = *var_idmultbot_dn4_slot;
        let mut var_idmultbot_dn5: f64 = *var_idmultbot_dn5_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_db0: f64 = *var_nj0_db0_slot;
        let mut var_nj0_db1: f64 = *var_nj0_db1_slot;
        let mut var_nj0_db2: f64 = *var_nj0_db2_slot;
        let mut var_nj0_db3: f64 = *var_nj0_db3_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn1: f64 = *var_nj0_dn1_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_dn3: f64 = *var_nj0_dn3_slot;
        let mut var_nj0_dn4: f64 = *var_nj0_dn4_slot;
        let mut var_nj0_dn5: f64 = *var_nj0_dn5_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_db0: f64 = *var_nj1_db0_slot;
        let mut var_nj1_db1: f64 = *var_nj1_db1_slot;
        let mut var_nj1_db2: f64 = *var_nj1_db2_slot;
        let mut var_nj1_db3: f64 = *var_nj1_db3_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn1: f64 = *var_nj1_dn1_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_dn3: f64 = *var_nj1_dn3_slot;
        let mut var_nj1_dn4: f64 = *var_nj1_dn4_slot;
        let mut var_nj1_dn5: f64 = *var_nj1_dn5_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_db0: f64 = *var_nja10_db0_slot;
        let mut var_nja10_db1: f64 = *var_nja10_db1_slot;
        let mut var_nja10_db2: f64 = *var_nja10_db2_slot;
        let mut var_nja10_db3: f64 = *var_nja10_db3_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn1: f64 = *var_nja10_dn1_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_dn3: f64 = *var_nja10_dn3_slot;
        let mut var_nja10_dn4: f64 = *var_nja10_dn4_slot;
        let mut var_nja10_dn5: f64 = *var_nja10_dn5_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_db0: f64 = *var_nja11_db0_slot;
        let mut var_nja11_db1: f64 = *var_nja11_db1_slot;
        let mut var_nja11_db2: f64 = *var_nja11_db2_slot;
        let mut var_nja11_db3: f64 = *var_nja11_db3_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn1: f64 = *var_nja11_dn1_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_dn3: f64 = *var_nja11_dn3_slot;
        let mut var_nja11_dn4: f64 = *var_nja11_dn4_slot;
        let mut var_nja11_dn5: f64 = *var_nja11_dn5_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_db0: f64 = *var_tmf1_db0_slot;
        let mut var_tmf1_db1: f64 = *var_tmf1_db1_slot;
        let mut var_tmf1_db2: f64 = *var_tmf1_db2_slot;
        let mut var_tmf1_db3: f64 = *var_tmf1_db3_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn1: f64 = *var_tmf1_dn1_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn3: f64 = *var_tmf1_dn3_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_db0: f64 = *var_tmf2_db0_slot;
        let mut var_tmf2_db1: f64 = *var_tmf2_db1_slot;
        let mut var_tmf2_db2: f64 = *var_tmf2_db2_slot;
        let mut var_tmf2_db3: f64 = *var_tmf2_db3_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn1: f64 = *var_tmf2_dn1_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn3: f64 = *var_tmf2_dn3_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign19920_e28587, assign19920_e28587_d_n0, assign19920_e28587_d_n1, assign19920_e28587_d_n2, assign19920_e28587_d_n3, assign19920_e28587_d_n4, assign19920_e28587_d_n5, assign19920_e28587_d_b0, assign19920_e28587_d_b1, assign19920_e28587_d_b2, assign19920_e28587_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign19920_e28587;
        var_nj1_dn0 = assign19920_e28587_d_n0;
        var_nj1_dn1 = assign19920_e28587_d_n1;
        var_nj1_dn2 = assign19920_e28587_d_n2;
        var_nj1_dn3 = assign19920_e28587_d_n3;
        var_nj1_dn4 = assign19920_e28587_d_n4;
        var_nj1_dn5 = assign19920_e28587_d_n5;
        var_nj1_db0 = assign19920_e28587_d_b0;
        var_nj1_db1 = assign19920_e28587_d_b1;
        var_nj1_db2 = assign19920_e28587_d_b2;
        var_nj1_db3 = assign19920_e28587_d_b3;

        let (assign19930_e28599, assign19930_e28599_d_n0, assign19930_e28599_d_n1, assign19930_e28599_d_n2, assign19930_e28599_d_n3, assign19930_e28599_d_n4, assign19930_e28599_d_n5, assign19930_e28599_d_b0, assign19930_e28599_d_b1, assign19930_e28599_d_b2, assign19930_e28599_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign19930_e28599;
        var_dnj1_dv_dn0 = assign19930_e28599_d_n0;
        var_dnj1_dv_dn1 = assign19930_e28599_d_n1;
        var_dnj1_dv_dn2 = assign19930_e28599_d_n2;
        var_dnj1_dv_dn3 = assign19930_e28599_d_n3;
        var_dnj1_dv_dn4 = assign19930_e28599_d_n4;
        var_dnj1_dv_dn5 = assign19930_e28599_d_n5;
        var_dnj1_dv_db0 = assign19930_e28599_d_b0;
        var_dnj1_dv_db1 = assign19930_e28599_d_b1;
        var_dnj1_dv_db2 = assign19930_e28599_d_b2;
        var_dnj1_dv_db3 = assign19930_e28599_d_b3;

        let assign19940_e28603: f64 = (var_vmax / var_nj1);
        let assign19940_e28607: f64 = (var_nj1 - var_nj0);
        let assign19940_e28608: f64 = (var_vha1 * assign19940_e28607);
        let assign19940_e28611: f64 = (var_nj0 * p.p85);
        let assign19940_e28612: f64 = (assign19940_e28608 / assign19940_e28611);
        let assign19940_e28613: f64 = (assign19940_e28603 + assign19940_e28612);
        let assign19940_e28614: f64 = (var_phitdinv * assign19940_e28613);
        let assign19940_e28615: f64 = (assign19940_e28614).abs();
        let assign19940_e28617: f64 = if assign19940_e28615 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard321 = assign19940_e28617;

        let (assign19950_e28643, assign19950_e28643_d_n0, assign19950_e28643_d_n1, assign19950_e28643_d_n2, assign19950_e28643_d_n3, assign19950_e28643_d_n4, assign19950_e28643_d_n5, assign19950_e28643_d_b0, assign19950_e28643_d_b1, assign19950_e28643_d_b2, assign19950_e28643_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 != 0.0)) {
        let assign19950_e28629: f64 = (var_vmax / var_nj1);
        let assign19950_e28633: f64 = (var_nj1 - var_nj0);
        let assign19950_e28634: f64 = (var_vha1 * assign19950_e28633);
        let assign19950_e28637: f64 = (var_nj0 * p.p85);
        let assign19950_e28638: f64 = (assign19950_e28634 / assign19950_e28637);
        let assign19950_e28639: f64 = (assign19950_e28629 + assign19950_e28638);
        let assign19950_e28640: f64 = (var_phitdinv * assign19950_e28639);
        let assign19950_e28641: f64 = (assign19950_e28640).exp();
        (assign19950_e28641, (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn0 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn1 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn2 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn3 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn4 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_dn5 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_db0 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_db1 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_db2 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))), (assign19950_e28641 * (var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19950_e28637) - (assign19950_e28634 * (var_nj0_db3 * p.p85))) / (assign19950_e28637 * assign19950_e28637))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19950_e28643;
        var_exp_vmax_over_phitd_bot_dn0 = assign19950_e28643_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign19950_e28643_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign19950_e28643_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign19950_e28643_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign19950_e28643_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign19950_e28643_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign19950_e28643_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign19950_e28643_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign19950_e28643_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign19950_e28643_d_b3;

        let assign19960_e28647: f64 = (var_vmax / var_nj1);
        let assign19960_e28651: f64 = (var_nj1 - var_nj0);
        let assign19960_e28652: f64 = (var_vha1 * assign19960_e28651);
        let assign19960_e28655: f64 = (var_nj0 * p.p85);
        let assign19960_e28656: f64 = (assign19960_e28652 / assign19960_e28655);
        let assign19960_e28657: f64 = (assign19960_e28647 + assign19960_e28656);
        let assign19960_e28658: f64 = (var_phitdinv * assign19960_e28657);
        let assign19960_e28660: f64 = (-230.25850929940458);
        let assign19960_e28661: f64 = if assign19960_e28658 < assign19960_e28660 { 1.0 } else { 0.0 };
        var_guard322 = assign19960_e28661;

        let (assign19970_e28742, assign19970_e28742_d_n0, assign19970_e28742_d_n1, assign19970_e28742_d_n2, assign19970_e28742_d_n3, assign19970_e28742_d_n4, assign19970_e28742_d_n5, assign19970_e28742_d_b0, assign19970_e28742_d_b1, assign19970_e28742_d_b2, assign19970_e28742_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 != 0.0)) {
        let assign19970_e28676: f64 = (-230.25850929940458);
        let assign19970_e28680: f64 = (var_vmax / var_nj1);
        let assign19970_e28684: f64 = (var_nj1 - var_nj0);
        let assign19970_e28685: f64 = (var_vha1 * assign19970_e28684);
        let assign19970_e28688: f64 = (var_nj0 * p.p85);
        let assign19970_e28689: f64 = (assign19970_e28685 / assign19970_e28688);
        let assign19970_e28690: f64 = (assign19970_e28680 + assign19970_e28689);
        let assign19970_e28691: f64 = (var_phitdinv * assign19970_e28690);
        let assign19970_e28692: f64 = (assign19970_e28676 - assign19970_e28691);
        let assign19970_e28696: f64 = (-230.25850929940458);
        let assign19970_e28700: f64 = (var_vmax / var_nj1);
        let assign19970_e28704: f64 = (var_nj1 - var_nj0);
        let assign19970_e28705: f64 = (var_vha1 * assign19970_e28704);
        let assign19970_e28708: f64 = (var_nj0 * p.p85);
        let assign19970_e28709: f64 = (assign19970_e28705 / assign19970_e28708);
        let assign19970_e28710: f64 = (assign19970_e28700 + assign19970_e28709);
        let assign19970_e28711: f64 = (var_phitdinv * assign19970_e28710);
        let assign19970_e28712: f64 = (assign19970_e28696 - assign19970_e28711);
        let assign19970_e28715: f64 = (-230.25850929940458);
        let assign19970_e28719: f64 = (var_vmax / var_nj1);
        let assign19970_e28723: f64 = (var_nj1 - var_nj0);
        let assign19970_e28724: f64 = (var_vha1 * assign19970_e28723);
        let assign19970_e28727: f64 = (var_nj0 * p.p85);
        let assign19970_e28728: f64 = (assign19970_e28724 / assign19970_e28727);
        let assign19970_e28729: f64 = (assign19970_e28719 + assign19970_e28728);
        let assign19970_e28730: f64 = (var_phitdinv * assign19970_e28729);
        let assign19970_e28731: f64 = (assign19970_e28715 - assign19970_e28730);
        let assign19970_e28733: f64 = (assign19970_e28731 * 0.3333333333333333);
        let assign19970_e28734: f64 = (1.0 + assign19970_e28733);
        let assign19970_e28735: f64 = (assign19970_e28712 * assign19970_e28734);
        let assign19970_e28736: f64 = (0.5 * assign19970_e28735);
        let assign19970_e28737: f64 = (1.0 + assign19970_e28736);
        let assign19970_e28738: f64 = (assign19970_e28692 * assign19970_e28737);
        let assign19970_e28739: f64 = (1.0 + assign19970_e28738);
        let assign19970_e28740: f64 = (1e-100 / assign19970_e28739);
        (assign19970_e28740, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn0 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn0 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn0 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn1 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn1 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn1 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn2 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn2 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn2 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn3 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn3 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn3 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn4 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn4 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn4 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_dn5 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_dn5 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_dn5 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_db0 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_db0 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_db0 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_db1 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_db1 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_db1 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_db2 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_db2 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_db2 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19970_e28688) - (assign19970_e28685 * (var_nj0_db3 * p.p85))) / (assign19970_e28688 * assign19970_e28688))))) * assign19970_e28737) + (assign19970_e28692 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19970_e28708) - (assign19970_e28705 * (var_nj0_db3 * p.p85))) / (assign19970_e28708 * assign19970_e28708))))) * assign19970_e28734) + (assign19970_e28712 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19970_e28727) - (assign19970_e28724 * (var_nj0_db3 * p.p85))) / (assign19970_e28727 * assign19970_e28727))))) * 0.3333333333333333))))))) / (assign19970_e28739 * assign19970_e28739))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19970_e28742;
        var_exp_vmax_over_phitd_bot_dn0 = assign19970_e28742_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign19970_e28742_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign19970_e28742_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign19970_e28742_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign19970_e28742_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign19970_e28742_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign19970_e28742_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign19970_e28742_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign19970_e28742_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign19970_e28742_d_b3;

        let (assign19980_e28821, assign19980_e28821_d_n0, assign19980_e28821_d_n1, assign19980_e28821_d_n2, assign19980_e28821_d_n3, assign19980_e28821_d_n4, assign19980_e28821_d_n5, assign19980_e28821_d_b0, assign19980_e28821_d_b1, assign19980_e28821_d_b2, assign19980_e28821_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard321 == 0.0)) && (var_guard322 == 0.0)) {
        let assign19980_e28760: f64 = (var_vmax / var_nj1);
        let assign19980_e28764: f64 = (var_nj1 - var_nj0);
        let assign19980_e28765: f64 = (var_vha1 * assign19980_e28764);
        let assign19980_e28768: f64 = (var_nj0 * p.p85);
        let assign19980_e28769: f64 = (assign19980_e28765 / assign19980_e28768);
        let assign19980_e28770: f64 = (assign19980_e28760 + assign19980_e28769);
        let assign19980_e28771: f64 = (var_phitdinv * assign19980_e28770);
        let assign19980_e28773: f64 = (assign19980_e28771 - 230.25850929940458);
        let assign19980_e28779: f64 = (var_vmax / var_nj1);
        let assign19980_e28783: f64 = (var_nj1 - var_nj0);
        let assign19980_e28784: f64 = (var_vha1 * assign19980_e28783);
        let assign19980_e28787: f64 = (var_nj0 * p.p85);
        let assign19980_e28788: f64 = (assign19980_e28784 / assign19980_e28787);
        let assign19980_e28789: f64 = (assign19980_e28779 + assign19980_e28788);
        let assign19980_e28790: f64 = (var_phitdinv * assign19980_e28789);
        let assign19980_e28792: f64 = (assign19980_e28790 - 230.25850929940458);
        let assign19980_e28797: f64 = (var_vmax / var_nj1);
        let assign19980_e28801: f64 = (var_nj1 - var_nj0);
        let assign19980_e28802: f64 = (var_vha1 * assign19980_e28801);
        let assign19980_e28805: f64 = (var_nj0 * p.p85);
        let assign19980_e28806: f64 = (assign19980_e28802 / assign19980_e28805);
        let assign19980_e28807: f64 = (assign19980_e28797 + assign19980_e28806);
        let assign19980_e28808: f64 = (var_phitdinv * assign19980_e28807);
        let assign19980_e28810: f64 = (assign19980_e28808 - 230.25850929940458);
        let assign19980_e28812: f64 = (assign19980_e28810 * 0.3333333333333333);
        let assign19980_e28813: f64 = (1.0 + assign19980_e28812);
        let assign19980_e28814: f64 = (assign19980_e28792 * assign19980_e28813);
        let assign19980_e28815: f64 = (0.5 * assign19980_e28814);
        let assign19980_e28816: f64 = (1.0 + assign19980_e28815);
        let assign19980_e28817: f64 = (assign19980_e28773 * assign19980_e28816);
        let assign19980_e28818: f64 = (1.0 + assign19980_e28817);
        let assign19980_e28819: f64 = (1e100 * assign19980_e28818);
        (assign19980_e28819, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn0 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn0 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn0 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn1 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn1 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn1 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn2 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn2 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn2 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn3 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn3 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn3 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn4 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn4 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn4 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_dn5 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_dn5 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_dn5 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_db0 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_db0 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_db0 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_db1 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_db1 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_db1 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_db2 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_db2 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_db2 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19980_e28768) - (assign19980_e28765 * (var_nj0_db3 * p.p85))) / (assign19980_e28768 * assign19980_e28768)))) * assign19980_e28816) + (assign19980_e28773 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19980_e28787) - (assign19980_e28784 * (var_nj0_db3 * p.p85))) / (assign19980_e28787 * assign19980_e28787)))) * assign19980_e28813) + (assign19980_e28792 * ((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign19980_e28805) - (assign19980_e28802 * (var_nj0_db3 * p.p85))) / (assign19980_e28805 * assign19980_e28805)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign19980_e28821;
        var_exp_vmax_over_phitd_bot_dn0 = assign19980_e28821_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign19980_e28821_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign19980_e28821_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign19980_e28821_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign19980_e28821_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign19980_e28821_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign19980_e28821_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign19980_e28821_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign19980_e28821_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign19980_e28821_d_b3;

        let (assign19990_e28848, assign19990_e28848_d_n0, assign19990_e28848_d_n1, assign19990_e28848_d_n2, assign19990_e28848_d_n3, assign19990_e28848_d_n4, assign19990_e28848_d_n5, assign19990_e28848_d_b0, assign19990_e28848_d_b1, assign19990_e28848_d_b2, assign19990_e28848_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign19990_e28832: f64 = (var_vmax * var_dnj1_dv);
        let assign19990_e28833: f64 = (var_nj1 - assign19990_e28832);
        let assign19990_e28836: f64 = (var_nj1 * var_nj1);
        let assign19990_e28837: f64 = (assign19990_e28833 / assign19990_e28836);
        let assign19990_e28840: f64 = (var_vha1 * var_dnj1_dv);
        let assign19990_e28843: f64 = (var_nj0 * p.p85);
        let assign19990_e28844: f64 = (assign19990_e28840 / assign19990_e28843);
        let assign19990_e28845: f64 = (assign19990_e28837 + assign19990_e28844);
        let assign19990_e28846: f64 = (var_phitdinv * assign19990_e28845);
        (assign19990_e28846, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn0 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn1 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn2 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn3 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn4 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_dn5 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_db0) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_db0 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_db1) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_db1 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_db2) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_db2 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign19990_e28836) - (assign19990_e28833 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign19990_e28836 * assign19990_e28836)) + ((((var_vha1 * var_dnj1_dv_db3) * assign19990_e28843) - (assign19990_e28840 * (var_nj0_db3 * p.p85))) / (assign19990_e28843 * assign19990_e28843)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign19990_e28848;
        var_dvmax_over_phitd_dv_dn0 = assign19990_e28848_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign19990_e28848_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign19990_e28848_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign19990_e28848_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign19990_e28848_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign19990_e28848_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign19990_e28848_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign19990_e28848_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign19990_e28848_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign19990_e28848_d_b3;

        let (assign20000_e28865, assign20000_e28865_d_n0, assign20000_e28865_d_n1, assign20000_e28865_d_n2, assign20000_e28865_d_n3, assign20000_e28865_d_n4, assign20000_e28865_d_n5, assign20000_e28865_d_b0, assign20000_e28865_d_b1, assign20000_e28865_d_b2, assign20000_e28865_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20000_e28858: f64 = (var_v4 - var_vmax);
        let assign20000_e28860: f64 = (assign20000_e28858 * var_dvmax_over_phitd_dv);
        let assign20000_e28861: f64 = (1.0 + assign20000_e28860);
        let assign20000_e28863: f64 = (assign20000_e28861 * var_exp_vmax_over_phitd_bot);
        (assign20000_e28863, (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn0)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn1) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn1)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn2)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn3) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn3)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn4) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn4)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_dn5) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_dn5)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_db0) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_db0)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_db1) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_db1)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_db2) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_db2)), (((assign20000_e28858 * var_dvmax_over_phitd_dv_db3) * var_exp_vmax_over_phitd_bot) + (assign20000_e28861 * var_exp_vmax_over_phitd_bot_db3)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign20000_e28865;
        var_idmultbot_dn0 = assign20000_e28865_d_n0;
        var_idmultbot_dn1 = assign20000_e28865_d_n1;
        var_idmultbot_dn2 = assign20000_e28865_d_n2;
        var_idmultbot_dn3 = assign20000_e28865_d_n3;
        var_idmultbot_dn4 = assign20000_e28865_d_n4;
        var_idmultbot_dn5 = assign20000_e28865_d_n5;
        var_idmultbot_db0 = assign20000_e28865_d_b0;
        var_idmultbot_db1 = assign20000_e28865_d_b1;
        var_idmultbot_db2 = assign20000_e28865_d_b2;
        var_idmultbot_db3 = assign20000_e28865_d_b3;

        let (assign20010_e28878,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20010_e28874: f64 = (var_nin * var_nin);
        let assign20010_e28876: f64 = (assign20010_e28874 / var_ndisti_i);
        (assign20010_e28876,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign20010_e28878;

        let (assign20020_e28894,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20020_e28887: f64 = (var_nfasti_i / var_phitdinv);
        let assign20020_e28890: f64 = (var_ndisti_i / var_pnn0);
        let assign20020_e28891: f64 = (assign20020_e28890).ln();
        let assign20020_e28892: f64 = (assign20020_e28887 * assign20020_e28891);
        (assign20020_e28892,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign20020_e28894;

        let assign20030_e28897: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard323 = assign20030_e28897;

        let (assign20040_e28914, assign20040_e28914_d_n0, assign20040_e28914_d_n1, assign20040_e28914_d_n2, assign20040_e28914_d_n3, assign20040_e28914_d_n4, assign20040_e28914_d_n5, assign20040_e28914_d_b0, assign20040_e28914_d_b1, assign20040_e28914_d_b2, assign20040_e28914_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20040_e28909: f64 = (var_vmax - var_vha1);
        let assign20040_e28910: f64 = (p.p86 * assign20040_e28909);
        let assign20040_e28912: f64 = (assign20040_e28910 + var_nfasti_i);
        (assign20040_e28912, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign20040_e28914;
        var_nja10_dn0 = assign20040_e28914_d_n0;
        var_nja10_dn1 = assign20040_e28914_d_n1;
        var_nja10_dn2 = assign20040_e28914_d_n2;
        var_nja10_dn3 = assign20040_e28914_d_n3;
        var_nja10_dn4 = assign20040_e28914_d_n4;
        var_nja10_dn5 = assign20040_e28914_d_n5;
        var_nja10_db0 = assign20040_e28914_d_b0;
        var_nja10_db1 = assign20040_e28914_d_b1;
        var_nja10_db2 = assign20040_e28914_d_b2;
        var_nja10_db3 = assign20040_e28914_d_b3;

        let (assign20050_e28929, assign20050_e28929_d_n0, assign20050_e28929_d_n1, assign20050_e28929_d_n2, assign20050_e28929_d_n3, assign20050_e28929_d_n4, assign20050_e28929_d_n5, assign20050_e28929_d_b0, assign20050_e28929_d_b1, assign20050_e28929_d_b2, assign20050_e28929_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20050_e28926: f64 = (p.p86 * var_vha1);
        let assign20050_e28927: f64 = (var_nfasti_i - assign20050_e28926);
        (assign20050_e28927, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20050_e28929;
        var_nj0_dn0 = assign20050_e28929_d_n0;
        var_nj0_dn1 = assign20050_e28929_d_n1;
        var_nj0_dn2 = assign20050_e28929_d_n2;
        var_nj0_dn3 = assign20050_e28929_d_n3;
        var_nj0_dn4 = assign20050_e28929_d_n4;
        var_nj0_dn5 = assign20050_e28929_d_n5;
        var_nj0_db0 = assign20050_e28929_d_b0;
        var_nj0_db1 = assign20050_e28929_d_b1;
        var_nj0_db2 = assign20050_e28929_d_b2;
        var_nj0_db3 = assign20050_e28929_d_b3;

        let (assign20060_e28944, assign20060_e28944_d_n0, assign20060_e28944_d_n1, assign20060_e28944_d_n2, assign20060_e28944_d_n3, assign20060_e28944_d_n4, assign20060_e28944_d_n5, assign20060_e28944_d_b0, assign20060_e28944_d_b1, assign20060_e28944_d_b2, assign20060_e28944_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20060_e28940: f64 = (p.p85 - var_nja10);
        let assign20060_e28942: f64 = (assign20060_e28940 - 0.01);
        (assign20060_e28942, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20060_e28944;
        var_tmf1_dn0 = assign20060_e28944_d_n0;
        var_tmf1_dn1 = assign20060_e28944_d_n1;
        var_tmf1_dn2 = assign20060_e28944_d_n2;
        var_tmf1_dn3 = assign20060_e28944_d_n3;
        var_tmf1_dn4 = assign20060_e28944_d_n4;
        var_tmf1_dn5 = assign20060_e28944_d_n5;
        var_tmf1_db0 = assign20060_e28944_d_b0;
        var_tmf1_db1 = assign20060_e28944_d_b1;
        var_tmf1_db2 = assign20060_e28944_d_b2;
        var_tmf1_db3 = assign20060_e28944_d_b3;

        let (assign20070_e28959, assign20070_e28959_d_n0, assign20070_e28959_d_n1, assign20070_e28959_d_n2, assign20070_e28959_d_n3, assign20070_e28959_d_n4, assign20070_e28959_d_n5, assign20070_e28959_d_b0, assign20070_e28959_d_b1, assign20070_e28959_d_b2, assign20070_e28959_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20070_e28955: f64 = (4.0 * p.p85);
        let assign20070_e28957: f64 = (assign20070_e28955 * 0.01);
        (assign20070_e28957, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20070_e28959;
        var_tmf2_dn0 = assign20070_e28959_d_n0;
        var_tmf2_dn1 = assign20070_e28959_d_n1;
        var_tmf2_dn2 = assign20070_e28959_d_n2;
        var_tmf2_dn3 = assign20070_e28959_d_n3;
        var_tmf2_dn4 = assign20070_e28959_d_n4;
        var_tmf2_dn5 = assign20070_e28959_d_n5;
        var_tmf2_db0 = assign20070_e28959_d_b0;
        var_tmf2_db1 = assign20070_e28959_d_b1;
        var_tmf2_db2 = assign20070_e28959_d_b2;
        var_tmf2_db3 = assign20070_e28959_d_b3;

        let (assign20080_e28976, assign20080_e28976_d_n0, assign20080_e28976_d_n1, assign20080_e28976_d_n2, assign20080_e28976_d_n3, assign20080_e28976_d_n4, assign20080_e28976_d_n5, assign20080_e28976_d_b0, assign20080_e28976_d_b1, assign20080_e28976_d_b2, assign20080_e28976_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n1, assign20080_e28974_d_n2, assign20080_e28974_d_n3, assign20080_e28974_d_n4, assign20080_e28974_d_n5, assign20080_e28974_d_b0, assign20080_e28974_d_b1, assign20080_e28974_d_b2, assign20080_e28974_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20080_e28973: f64 = (-var_tmf2);
                (assign20080_e28973, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20080_e28974, assign20080_e28974_d_n0, assign20080_e28974_d_n1, assign20080_e28974_d_n2, assign20080_e28974_d_n3, assign20080_e28974_d_n4, assign20080_e28974_d_n5, assign20080_e28974_d_b0, assign20080_e28974_d_b1, assign20080_e28974_d_b2, assign20080_e28974_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20080_e28976;
        var_tmf2_dn0 = assign20080_e28976_d_n0;
        var_tmf2_dn1 = assign20080_e28976_d_n1;
        var_tmf2_dn2 = assign20080_e28976_d_n2;
        var_tmf2_dn3 = assign20080_e28976_d_n3;
        var_tmf2_dn4 = assign20080_e28976_d_n4;
        var_tmf2_dn5 = assign20080_e28976_d_n5;
        var_tmf2_db0 = assign20080_e28976_d_b0;
        var_tmf2_db1 = assign20080_e28976_d_b1;
        var_tmf2_db2 = assign20080_e28976_d_b2;
        var_tmf2_db3 = assign20080_e28976_d_b3;

        let (assign20090_e28992, assign20090_e28992_d_n0, assign20090_e28992_d_n1, assign20090_e28992_d_n2, assign20090_e28992_d_n3, assign20090_e28992_d_n4, assign20090_e28992_d_n5, assign20090_e28992_d_b0, assign20090_e28992_d_b1, assign20090_e28992_d_b2, assign20090_e28992_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20090_e28987: f64 = (var_tmf1 * var_tmf1);
        let assign20090_e28989: f64 = (assign20090_e28987 + var_tmf2);
        let assign20090_e28990: f64 = (assign20090_e28989).sqrt();
        (assign20090_e28990, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20090_e28990)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20090_e28990)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20090_e28990)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20090_e28990)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20090_e28990)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20090_e28990)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20090_e28992;
        var_tmf2_dn0 = assign20090_e28992_d_n0;
        var_tmf2_dn1 = assign20090_e28992_d_n1;
        var_tmf2_dn2 = assign20090_e28992_d_n2;
        var_tmf2_dn3 = assign20090_e28992_d_n3;
        var_tmf2_dn4 = assign20090_e28992_d_n4;
        var_tmf2_dn5 = assign20090_e28992_d_n5;
        var_tmf2_db0 = assign20090_e28992_d_b0;
        var_tmf2_db1 = assign20090_e28992_d_b1;
        var_tmf2_db2 = assign20090_e28992_d_b2;
        var_tmf2_db3 = assign20090_e28992_d_b3;

        let (assign20100_e29009, assign20100_e29009_d_n0, assign20100_e29009_d_n1, assign20100_e29009_d_n2, assign20100_e29009_d_n3, assign20100_e29009_d_n4, assign20100_e29009_d_n5, assign20100_e29009_d_b0, assign20100_e29009_d_b1, assign20100_e29009_d_b2, assign20100_e29009_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20100_e29005: f64 = (var_tmf1 / var_tmf2);
        let assign20100_e29006: f64 = (1.0 + assign20100_e29005);
        let assign20100_e29007: f64 = (0.5 * assign20100_e29006);
        (assign20100_e29007, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign20100_e29009;
        var_dfn_su_dn0 = assign20100_e29009_d_n0;
        var_dfn_su_dn1 = assign20100_e29009_d_n1;
        var_dfn_su_dn2 = assign20100_e29009_d_n2;
        var_dfn_su_dn3 = assign20100_e29009_d_n3;
        var_dfn_su_dn4 = assign20100_e29009_d_n4;
        var_dfn_su_dn5 = assign20100_e29009_d_n5;
        var_dfn_su_db0 = assign20100_e29009_d_b0;
        var_dfn_su_db1 = assign20100_e29009_d_b1;
        var_dfn_su_db2 = assign20100_e29009_d_b2;
        var_dfn_su_db3 = assign20100_e29009_d_b3;

        let (assign20110_e29026, assign20110_e29026_d_n0, assign20110_e29026_d_n1, assign20110_e29026_d_n2, assign20110_e29026_d_n3, assign20110_e29026_d_n4, assign20110_e29026_d_n5, assign20110_e29026_d_b0, assign20110_e29026_d_b1, assign20110_e29026_d_b2, assign20110_e29026_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20110_e29022: f64 = (var_tmf1 + var_tmf2);
        let assign20110_e29023: f64 = (0.5 * assign20110_e29022);
        let assign20110_e29024: f64 = (p.p85 - assign20110_e29023);
        (assign20110_e29024, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign20110_e29026;
        var_nja11_dn0 = assign20110_e29026_d_n0;
        var_nja11_dn1 = assign20110_e29026_d_n1;
        var_nja11_dn2 = assign20110_e29026_d_n2;
        var_nja11_dn3 = assign20110_e29026_d_n3;
        var_nja11_dn4 = assign20110_e29026_d_n4;
        var_nja11_dn5 = assign20110_e29026_d_n5;
        var_nja11_db0 = assign20110_e29026_d_b0;
        var_nja11_db1 = assign20110_e29026_d_b1;
        var_nja11_db2 = assign20110_e29026_d_b2;
        var_nja11_db3 = assign20110_e29026_d_b3;

        let (assign20120_e29041, assign20120_e29041_d_n0, assign20120_e29041_d_n1, assign20120_e29041_d_n2, assign20120_e29041_d_n3, assign20120_e29041_d_n4, assign20120_e29041_d_n5, assign20120_e29041_d_b0, assign20120_e29041_d_b1, assign20120_e29041_d_b2, assign20120_e29041_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20120_e29037: f64 = (var_nja11 - var_nfasti_i);
        let assign20120_e29039: f64 = (assign20120_e29037 - 0.01);
        (assign20120_e29039, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20120_e29041;
        var_tmf1_dn0 = assign20120_e29041_d_n0;
        var_tmf1_dn1 = assign20120_e29041_d_n1;
        var_tmf1_dn2 = assign20120_e29041_d_n2;
        var_tmf1_dn3 = assign20120_e29041_d_n3;
        var_tmf1_dn4 = assign20120_e29041_d_n4;
        var_tmf1_dn5 = assign20120_e29041_d_n5;
        var_tmf1_db0 = assign20120_e29041_d_b0;
        var_tmf1_db1 = assign20120_e29041_d_b1;
        var_tmf1_db2 = assign20120_e29041_d_b2;
        var_tmf1_db3 = assign20120_e29041_d_b3;

        let (assign20130_e29056, assign20130_e29056_d_n0, assign20130_e29056_d_n1, assign20130_e29056_d_n2, assign20130_e29056_d_n3, assign20130_e29056_d_n4, assign20130_e29056_d_n5, assign20130_e29056_d_b0, assign20130_e29056_d_b1, assign20130_e29056_d_b2, assign20130_e29056_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20130_e29052: f64 = (4.0 * var_nfasti_i);
        let assign20130_e29054: f64 = (assign20130_e29052 * 0.01);
        (assign20130_e29054, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20130_e29056;
        var_tmf2_dn0 = assign20130_e29056_d_n0;
        var_tmf2_dn1 = assign20130_e29056_d_n1;
        var_tmf2_dn2 = assign20130_e29056_d_n2;
        var_tmf2_dn3 = assign20130_e29056_d_n3;
        var_tmf2_dn4 = assign20130_e29056_d_n4;
        var_tmf2_dn5 = assign20130_e29056_d_n5;
        var_tmf2_db0 = assign20130_e29056_d_b0;
        var_tmf2_db1 = assign20130_e29056_d_b1;
        var_tmf2_db2 = assign20130_e29056_d_b2;
        var_tmf2_db3 = assign20130_e29056_d_b3;

        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_db0_slot = var_dfn_su_db0;
        *var_dfn_su_db1_slot = var_dfn_su_db1;
        *var_dfn_su_db2_slot = var_dfn_su_db2;
        *var_dfn_su_db3_slot = var_dfn_su_db3;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn1_slot = var_dfn_su_dn1;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_dn3_slot = var_dfn_su_dn3;
        *var_dfn_su_dn4_slot = var_dfn_su_dn4;
        *var_dfn_su_dn5_slot = var_dfn_su_dn5;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_db0_slot = var_dnj1_dv_db0;
        *var_dnj1_dv_db1_slot = var_dnj1_dv_db1;
        *var_dnj1_dv_db2_slot = var_dnj1_dv_db2;
        *var_dnj1_dv_db3_slot = var_dnj1_dv_db3;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn1_slot = var_dnj1_dv_dn1;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_dn3_slot = var_dnj1_dv_dn3;
        *var_dnj1_dv_dn4_slot = var_dnj1_dv_dn4;
        *var_dnj1_dv_dn5_slot = var_dnj1_dv_dn5;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_db0_slot = var_dvmax_over_phitd_dv_db0;
        *var_dvmax_over_phitd_dv_db1_slot = var_dvmax_over_phitd_dv_db1;
        *var_dvmax_over_phitd_dv_db2_slot = var_dvmax_over_phitd_dv_db2;
        *var_dvmax_over_phitd_dv_db3_slot = var_dvmax_over_phitd_dv_db3;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn1_slot = var_dvmax_over_phitd_dv_dn1;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_dn3_slot = var_dvmax_over_phitd_dv_dn3;
        *var_dvmax_over_phitd_dv_dn4_slot = var_dvmax_over_phitd_dv_dn4;
        *var_dvmax_over_phitd_dv_dn5_slot = var_dvmax_over_phitd_dv_dn5;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_db0_slot = var_exp_vmax_over_phitd_bot_db0;
        *var_exp_vmax_over_phitd_bot_db1_slot = var_exp_vmax_over_phitd_bot_db1;
        *var_exp_vmax_over_phitd_bot_db2_slot = var_exp_vmax_over_phitd_bot_db2;
        *var_exp_vmax_over_phitd_bot_db3_slot = var_exp_vmax_over_phitd_bot_db3;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn1_slot = var_exp_vmax_over_phitd_bot_dn1;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_dn3_slot = var_exp_vmax_over_phitd_bot_dn3;
        *var_exp_vmax_over_phitd_bot_dn4_slot = var_exp_vmax_over_phitd_bot_dn4;
        *var_exp_vmax_over_phitd_bot_dn5_slot = var_exp_vmax_over_phitd_bot_dn5;
        *var_guard321_slot = var_guard321;
        *var_guard322_slot = var_guard322;
        *var_guard323_slot = var_guard323;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_db0_slot = var_idmultbot_db0;
        *var_idmultbot_db1_slot = var_idmultbot_db1;
        *var_idmultbot_db2_slot = var_idmultbot_db2;
        *var_idmultbot_db3_slot = var_idmultbot_db3;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn1_slot = var_idmultbot_dn1;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_dn3_slot = var_idmultbot_dn3;
        *var_idmultbot_dn4_slot = var_idmultbot_dn4;
        *var_idmultbot_dn5_slot = var_idmultbot_dn5;
        *var_nj0_slot = var_nj0;
        *var_nj0_db0_slot = var_nj0_db0;
        *var_nj0_db1_slot = var_nj0_db1;
        *var_nj0_db2_slot = var_nj0_db2;
        *var_nj0_db3_slot = var_nj0_db3;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn1_slot = var_nj0_dn1;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_dn3_slot = var_nj0_dn3;
        *var_nj0_dn4_slot = var_nj0_dn4;
        *var_nj0_dn5_slot = var_nj0_dn5;
        *var_nj1_slot = var_nj1;
        *var_nj1_db0_slot = var_nj1_db0;
        *var_nj1_db1_slot = var_nj1_db1;
        *var_nj1_db2_slot = var_nj1_db2;
        *var_nj1_db3_slot = var_nj1_db3;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn1_slot = var_nj1_dn1;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_dn3_slot = var_nj1_dn3;
        *var_nj1_dn4_slot = var_nj1_dn4;
        *var_nj1_dn5_slot = var_nj1_dn5;
        *var_nja10_slot = var_nja10;
        *var_nja10_db0_slot = var_nja10_db0;
        *var_nja10_db1_slot = var_nja10_db1;
        *var_nja10_db2_slot = var_nja10_db2;
        *var_nja10_db3_slot = var_nja10_db3;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn1_slot = var_nja10_dn1;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_dn3_slot = var_nja10_dn3;
        *var_nja10_dn4_slot = var_nja10_dn4;
        *var_nja10_dn5_slot = var_nja10_dn5;
        *var_nja11_slot = var_nja11;
        *var_nja11_db0_slot = var_nja11_db0;
        *var_nja11_db1_slot = var_nja11_db1;
        *var_nja11_db2_slot = var_nja11_db2;
        *var_nja11_db3_slot = var_nja11_db3;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn1_slot = var_nja11_dn1;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_dn3_slot = var_nja11_dn3;
        *var_nja11_dn4_slot = var_nja11_dn4;
        *var_nja11_dn5_slot = var_nja11_dn5;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_db0_slot = var_tmf1_db0;
        *var_tmf1_db1_slot = var_tmf1_db1;
        *var_tmf1_db2_slot = var_tmf1_db2;
        *var_tmf1_db3_slot = var_tmf1_db3;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn1_slot = var_tmf1_dn1;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn3_slot = var_tmf1_dn3;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_db0_slot = var_tmf2_db0;
        *var_tmf2_db1_slot = var_tmf2_db1;
        *var_tmf2_db2_slot = var_tmf2_db2;
        *var_tmf2_db3_slot = var_tmf2_db3;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn1_slot = var_tmf2_dn1;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn3_slot = var_tmf2_dn3;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_vha1_slot = var_vha1;
    }
}
