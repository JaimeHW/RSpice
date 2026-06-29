#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
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
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard323: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
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
        var_guard326_slot: &mut f64,
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
        let mut var_guard326: f64 = *var_guard326_slot;
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

        let (assign20140_e29073, assign20140_e29073_d_n0, assign20140_e29073_d_n1, assign20140_e29073_d_n2, assign20140_e29073_d_n3, assign20140_e29073_d_n4, assign20140_e29073_d_n5, assign20140_e29073_d_b0, assign20140_e29073_d_b1, assign20140_e29073_d_b2, assign20140_e29073_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n1, assign20140_e29071_d_n2, assign20140_e29071_d_n3, assign20140_e29071_d_n4, assign20140_e29071_d_n5, assign20140_e29071_d_b0, assign20140_e29071_d_b1, assign20140_e29071_d_b2, assign20140_e29071_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20140_e29070: f64 = (-var_tmf2);
                (assign20140_e29070, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20140_e29071, assign20140_e29071_d_n0, assign20140_e29071_d_n1, assign20140_e29071_d_n2, assign20140_e29071_d_n3, assign20140_e29071_d_n4, assign20140_e29071_d_n5, assign20140_e29071_d_b0, assign20140_e29071_d_b1, assign20140_e29071_d_b2, assign20140_e29071_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20140_e29073;
        var_tmf2_dn0 = assign20140_e29073_d_n0;
        var_tmf2_dn1 = assign20140_e29073_d_n1;
        var_tmf2_dn2 = assign20140_e29073_d_n2;
        var_tmf2_dn3 = assign20140_e29073_d_n3;
        var_tmf2_dn4 = assign20140_e29073_d_n4;
        var_tmf2_dn5 = assign20140_e29073_d_n5;
        var_tmf2_db0 = assign20140_e29073_d_b0;
        var_tmf2_db1 = assign20140_e29073_d_b1;
        var_tmf2_db2 = assign20140_e29073_d_b2;
        var_tmf2_db3 = assign20140_e29073_d_b3;

        let (assign20150_e29089, assign20150_e29089_d_n0, assign20150_e29089_d_n1, assign20150_e29089_d_n2, assign20150_e29089_d_n3, assign20150_e29089_d_n4, assign20150_e29089_d_n5, assign20150_e29089_d_b0, assign20150_e29089_d_b1, assign20150_e29089_d_b2, assign20150_e29089_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20150_e29084: f64 = (var_tmf1 * var_tmf1);
        let assign20150_e29086: f64 = (assign20150_e29084 + var_tmf2);
        let assign20150_e29087: f64 = (assign20150_e29086).sqrt();
        (assign20150_e29087, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20150_e29087)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20150_e29087)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20150_e29087)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20150_e29087)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20150_e29087)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20150_e29087)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20150_e29089;
        var_tmf2_dn0 = assign20150_e29089_d_n0;
        var_tmf2_dn1 = assign20150_e29089_d_n1;
        var_tmf2_dn2 = assign20150_e29089_d_n2;
        var_tmf2_dn3 = assign20150_e29089_d_n3;
        var_tmf2_dn4 = assign20150_e29089_d_n4;
        var_tmf2_dn5 = assign20150_e29089_d_n5;
        var_tmf2_db0 = assign20150_e29089_d_b0;
        var_tmf2_db1 = assign20150_e29089_d_b1;
        var_tmf2_db2 = assign20150_e29089_d_b2;
        var_tmf2_db3 = assign20150_e29089_d_b3;

        let (assign20160_e29106, assign20160_e29106_d_n0, assign20160_e29106_d_n1, assign20160_e29106_d_n2, assign20160_e29106_d_n3, assign20160_e29106_d_n4, assign20160_e29106_d_n5, assign20160_e29106_d_b0, assign20160_e29106_d_b1, assign20160_e29106_d_b2, assign20160_e29106_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20160_e29102: f64 = (var_tmf1 / var_tmf2);
        let assign20160_e29103: f64 = (1.0 + assign20160_e29102);
        let assign20160_e29104: f64 = (0.5 * assign20160_e29103);
        (assign20160_e29104, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign20160_e29106;
        var_dfn_sl_dn0 = assign20160_e29106_d_n0;
        var_dfn_sl_dn1 = assign20160_e29106_d_n1;
        var_dfn_sl_dn2 = assign20160_e29106_d_n2;
        var_dfn_sl_dn3 = assign20160_e29106_d_n3;
        var_dfn_sl_dn4 = assign20160_e29106_d_n4;
        var_dfn_sl_dn5 = assign20160_e29106_d_n5;
        var_dfn_sl_db0 = assign20160_e29106_d_b0;
        var_dfn_sl_db1 = assign20160_e29106_d_b1;
        var_dfn_sl_db2 = assign20160_e29106_d_b2;
        var_dfn_sl_db3 = assign20160_e29106_d_b3;

        let (assign20170_e29123, assign20170_e29123_d_n0, assign20170_e29123_d_n1, assign20170_e29123_d_n2, assign20170_e29123_d_n3, assign20170_e29123_d_n4, assign20170_e29123_d_n5, assign20170_e29123_d_b0, assign20170_e29123_d_b1, assign20170_e29123_d_b2, assign20170_e29123_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20170_e29119: f64 = (var_tmf1 + var_tmf2);
        let assign20170_e29120: f64 = (0.5 * assign20170_e29119);
        let assign20170_e29121: f64 = (var_nfasti_i + assign20170_e29120);
        (assign20170_e29121, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign20170_e29123;
        var_nj1_dn0 = assign20170_e29123_d_n0;
        var_nj1_dn1 = assign20170_e29123_d_n1;
        var_nj1_dn2 = assign20170_e29123_d_n2;
        var_nj1_dn3 = assign20170_e29123_d_n3;
        var_nj1_dn4 = assign20170_e29123_d_n4;
        var_nj1_dn5 = assign20170_e29123_d_n5;
        var_nj1_db0 = assign20170_e29123_d_b0;
        var_nj1_db1 = assign20170_e29123_d_b1;
        var_nj1_db2 = assign20170_e29123_d_b2;
        var_nj1_db3 = assign20170_e29123_d_b3;

        let (assign20180_e29138, assign20180_e29138_d_n0, assign20180_e29138_d_n1, assign20180_e29138_d_n2, assign20180_e29138_d_n3, assign20180_e29138_d_n4, assign20180_e29138_d_n5, assign20180_e29138_d_b0, assign20180_e29138_d_b1, assign20180_e29138_d_b2, assign20180_e29138_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20180_e29134: f64 = (p.p85 - var_nj0);
        let assign20180_e29136: f64 = (assign20180_e29134 - 0.01);
        (assign20180_e29136, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20180_e29138;
        var_tmf1_dn0 = assign20180_e29138_d_n0;
        var_tmf1_dn1 = assign20180_e29138_d_n1;
        var_tmf1_dn2 = assign20180_e29138_d_n2;
        var_tmf1_dn3 = assign20180_e29138_d_n3;
        var_tmf1_dn4 = assign20180_e29138_d_n4;
        var_tmf1_dn5 = assign20180_e29138_d_n5;
        var_tmf1_db0 = assign20180_e29138_d_b0;
        var_tmf1_db1 = assign20180_e29138_d_b1;
        var_tmf1_db2 = assign20180_e29138_d_b2;
        var_tmf1_db3 = assign20180_e29138_d_b3;

        let (assign20190_e29153, assign20190_e29153_d_n0, assign20190_e29153_d_n1, assign20190_e29153_d_n2, assign20190_e29153_d_n3, assign20190_e29153_d_n4, assign20190_e29153_d_n5, assign20190_e29153_d_b0, assign20190_e29153_d_b1, assign20190_e29153_d_b2, assign20190_e29153_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20190_e29149: f64 = (4.0 * p.p85);
        let assign20190_e29151: f64 = (assign20190_e29149 * 0.01);
        (assign20190_e29151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20190_e29153;
        var_tmf2_dn0 = assign20190_e29153_d_n0;
        var_tmf2_dn1 = assign20190_e29153_d_n1;
        var_tmf2_dn2 = assign20190_e29153_d_n2;
        var_tmf2_dn3 = assign20190_e29153_d_n3;
        var_tmf2_dn4 = assign20190_e29153_d_n4;
        var_tmf2_dn5 = assign20190_e29153_d_n5;
        var_tmf2_db0 = assign20190_e29153_d_b0;
        var_tmf2_db1 = assign20190_e29153_d_b1;
        var_tmf2_db2 = assign20190_e29153_d_b2;
        var_tmf2_db3 = assign20190_e29153_d_b3;

        let (assign20200_e29170, assign20200_e29170_d_n0, assign20200_e29170_d_n1, assign20200_e29170_d_n2, assign20200_e29170_d_n3, assign20200_e29170_d_n4, assign20200_e29170_d_n5, assign20200_e29170_d_b0, assign20200_e29170_d_b1, assign20200_e29170_d_b2, assign20200_e29170_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n1, assign20200_e29168_d_n2, assign20200_e29168_d_n3, assign20200_e29168_d_n4, assign20200_e29168_d_n5, assign20200_e29168_d_b0, assign20200_e29168_d_b1, assign20200_e29168_d_b2, assign20200_e29168_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20200_e29167: f64 = (-var_tmf2);
                (assign20200_e29167, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20200_e29168, assign20200_e29168_d_n0, assign20200_e29168_d_n1, assign20200_e29168_d_n2, assign20200_e29168_d_n3, assign20200_e29168_d_n4, assign20200_e29168_d_n5, assign20200_e29168_d_b0, assign20200_e29168_d_b1, assign20200_e29168_d_b2, assign20200_e29168_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20200_e29170;
        var_tmf2_dn0 = assign20200_e29170_d_n0;
        var_tmf2_dn1 = assign20200_e29170_d_n1;
        var_tmf2_dn2 = assign20200_e29170_d_n2;
        var_tmf2_dn3 = assign20200_e29170_d_n3;
        var_tmf2_dn4 = assign20200_e29170_d_n4;
        var_tmf2_dn5 = assign20200_e29170_d_n5;
        var_tmf2_db0 = assign20200_e29170_d_b0;
        var_tmf2_db1 = assign20200_e29170_d_b1;
        var_tmf2_db2 = assign20200_e29170_d_b2;
        var_tmf2_db3 = assign20200_e29170_d_b3;

        let (assign20210_e29186, assign20210_e29186_d_n0, assign20210_e29186_d_n1, assign20210_e29186_d_n2, assign20210_e29186_d_n3, assign20210_e29186_d_n4, assign20210_e29186_d_n5, assign20210_e29186_d_b0, assign20210_e29186_d_b1, assign20210_e29186_d_b2, assign20210_e29186_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20210_e29181: f64 = (var_tmf1 * var_tmf1);
        let assign20210_e29183: f64 = (assign20210_e29181 + var_tmf2);
        let assign20210_e29184: f64 = (assign20210_e29183).sqrt();
        (assign20210_e29184, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20210_e29184)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20210_e29184)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20210_e29184)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20210_e29184)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20210_e29184)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20210_e29184)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20210_e29186;
        var_tmf2_dn0 = assign20210_e29186_d_n0;
        var_tmf2_dn1 = assign20210_e29186_d_n1;
        var_tmf2_dn2 = assign20210_e29186_d_n2;
        var_tmf2_dn3 = assign20210_e29186_d_n3;
        var_tmf2_dn4 = assign20210_e29186_d_n4;
        var_tmf2_dn5 = assign20210_e29186_d_n5;
        var_tmf2_db0 = assign20210_e29186_d_b0;
        var_tmf2_db1 = assign20210_e29186_d_b1;
        var_tmf2_db2 = assign20210_e29186_d_b2;
        var_tmf2_db3 = assign20210_e29186_d_b3;

        let (assign20220_e29203, assign20220_e29203_d_n0, assign20220_e29203_d_n1, assign20220_e29203_d_n2, assign20220_e29203_d_n3, assign20220_e29203_d_n4, assign20220_e29203_d_n5, assign20220_e29203_d_b0, assign20220_e29203_d_b1, assign20220_e29203_d_b2, assign20220_e29203_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20220_e29199: f64 = (var_tmf1 + var_tmf2);
        let assign20220_e29200: f64 = (0.5 * assign20220_e29199);
        let assign20220_e29201: f64 = (p.p85 - assign20220_e29200);
        (assign20220_e29201, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20220_e29203;
        var_nj0_dn0 = assign20220_e29203_d_n0;
        var_nj0_dn1 = assign20220_e29203_d_n1;
        var_nj0_dn2 = assign20220_e29203_d_n2;
        var_nj0_dn3 = assign20220_e29203_d_n3;
        var_nj0_dn4 = assign20220_e29203_d_n4;
        var_nj0_dn5 = assign20220_e29203_d_n5;
        var_nj0_db0 = assign20220_e29203_d_b0;
        var_nj0_db1 = assign20220_e29203_d_b1;
        var_nj0_db2 = assign20220_e29203_d_b2;
        var_nj0_db3 = assign20220_e29203_d_b3;

        let (assign20230_e29218, assign20230_e29218_d_n0, assign20230_e29218_d_n1, assign20230_e29218_d_n2, assign20230_e29218_d_n3, assign20230_e29218_d_n4, assign20230_e29218_d_n5, assign20230_e29218_d_b0, assign20230_e29218_d_b1, assign20230_e29218_d_b2, assign20230_e29218_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20230_e29214: f64 = (var_nj0 - var_nfasti_i);
        let assign20230_e29216: f64 = (assign20230_e29214 - 0.01);
        (assign20230_e29216, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20230_e29218;
        var_tmf1_dn0 = assign20230_e29218_d_n0;
        var_tmf1_dn1 = assign20230_e29218_d_n1;
        var_tmf1_dn2 = assign20230_e29218_d_n2;
        var_tmf1_dn3 = assign20230_e29218_d_n3;
        var_tmf1_dn4 = assign20230_e29218_d_n4;
        var_tmf1_dn5 = assign20230_e29218_d_n5;
        var_tmf1_db0 = assign20230_e29218_d_b0;
        var_tmf1_db1 = assign20230_e29218_d_b1;
        var_tmf1_db2 = assign20230_e29218_d_b2;
        var_tmf1_db3 = assign20230_e29218_d_b3;

        let (assign20240_e29233, assign20240_e29233_d_n0, assign20240_e29233_d_n1, assign20240_e29233_d_n2, assign20240_e29233_d_n3, assign20240_e29233_d_n4, assign20240_e29233_d_n5, assign20240_e29233_d_b0, assign20240_e29233_d_b1, assign20240_e29233_d_b2, assign20240_e29233_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20240_e29229: f64 = (4.0 * var_nfasti_i);
        let assign20240_e29231: f64 = (assign20240_e29229 * 0.01);
        (assign20240_e29231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20240_e29233;
        var_tmf2_dn0 = assign20240_e29233_d_n0;
        var_tmf2_dn1 = assign20240_e29233_d_n1;
        var_tmf2_dn2 = assign20240_e29233_d_n2;
        var_tmf2_dn3 = assign20240_e29233_d_n3;
        var_tmf2_dn4 = assign20240_e29233_d_n4;
        var_tmf2_dn5 = assign20240_e29233_d_n5;
        var_tmf2_db0 = assign20240_e29233_d_b0;
        var_tmf2_db1 = assign20240_e29233_d_b1;
        var_tmf2_db2 = assign20240_e29233_d_b2;
        var_tmf2_db3 = assign20240_e29233_d_b3;

        let (assign20250_e29250, assign20250_e29250_d_n0, assign20250_e29250_d_n1, assign20250_e29250_d_n2, assign20250_e29250_d_n3, assign20250_e29250_d_n4, assign20250_e29250_d_n5, assign20250_e29250_d_b0, assign20250_e29250_d_b1, assign20250_e29250_d_b2, assign20250_e29250_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n1, assign20250_e29248_d_n2, assign20250_e29248_d_n3, assign20250_e29248_d_n4, assign20250_e29248_d_n5, assign20250_e29248_d_b0, assign20250_e29248_d_b1, assign20250_e29248_d_b2, assign20250_e29248_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20250_e29247: f64 = (-var_tmf2);
                (assign20250_e29247, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20250_e29248, assign20250_e29248_d_n0, assign20250_e29248_d_n1, assign20250_e29248_d_n2, assign20250_e29248_d_n3, assign20250_e29248_d_n4, assign20250_e29248_d_n5, assign20250_e29248_d_b0, assign20250_e29248_d_b1, assign20250_e29248_d_b2, assign20250_e29248_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20250_e29250;
        var_tmf2_dn0 = assign20250_e29250_d_n0;
        var_tmf2_dn1 = assign20250_e29250_d_n1;
        var_tmf2_dn2 = assign20250_e29250_d_n2;
        var_tmf2_dn3 = assign20250_e29250_d_n3;
        var_tmf2_dn4 = assign20250_e29250_d_n4;
        var_tmf2_dn5 = assign20250_e29250_d_n5;
        var_tmf2_db0 = assign20250_e29250_d_b0;
        var_tmf2_db1 = assign20250_e29250_d_b1;
        var_tmf2_db2 = assign20250_e29250_d_b2;
        var_tmf2_db3 = assign20250_e29250_d_b3;

        let (assign20260_e29266, assign20260_e29266_d_n0, assign20260_e29266_d_n1, assign20260_e29266_d_n2, assign20260_e29266_d_n3, assign20260_e29266_d_n4, assign20260_e29266_d_n5, assign20260_e29266_d_b0, assign20260_e29266_d_b1, assign20260_e29266_d_b2, assign20260_e29266_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20260_e29261: f64 = (var_tmf1 * var_tmf1);
        let assign20260_e29263: f64 = (assign20260_e29261 + var_tmf2);
        let assign20260_e29264: f64 = (assign20260_e29263).sqrt();
        (assign20260_e29264, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20260_e29264)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20260_e29264)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20260_e29264)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20260_e29264)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20260_e29264)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20260_e29264)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20260_e29266;
        var_tmf2_dn0 = assign20260_e29266_d_n0;
        var_tmf2_dn1 = assign20260_e29266_d_n1;
        var_tmf2_dn2 = assign20260_e29266_d_n2;
        var_tmf2_dn3 = assign20260_e29266_d_n3;
        var_tmf2_dn4 = assign20260_e29266_d_n4;
        var_tmf2_dn5 = assign20260_e29266_d_n5;
        var_tmf2_db0 = assign20260_e29266_d_b0;
        var_tmf2_db1 = assign20260_e29266_d_b1;
        var_tmf2_db2 = assign20260_e29266_d_b2;
        var_tmf2_db3 = assign20260_e29266_d_b3;

        let (assign20270_e29283, assign20270_e29283_d_n0, assign20270_e29283_d_n1, assign20270_e29283_d_n2, assign20270_e29283_d_n3, assign20270_e29283_d_n4, assign20270_e29283_d_n5, assign20270_e29283_d_b0, assign20270_e29283_d_b1, assign20270_e29283_d_b2, assign20270_e29283_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20270_e29279: f64 = (var_tmf1 + var_tmf2);
        let assign20270_e29280: f64 = (0.5 * assign20270_e29279);
        let assign20270_e29281: f64 = (var_nfasti_i + assign20270_e29280);
        (assign20270_e29281, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20270_e29283;
        var_nj0_dn0 = assign20270_e29283_d_n0;
        var_nj0_dn1 = assign20270_e29283_d_n1;
        var_nj0_dn2 = assign20270_e29283_d_n2;
        var_nj0_dn3 = assign20270_e29283_d_n3;
        var_nj0_dn4 = assign20270_e29283_d_n4;
        var_nj0_dn5 = assign20270_e29283_d_n5;
        var_nj0_db0 = assign20270_e29283_d_b0;
        var_nj0_db1 = assign20270_e29283_d_b1;
        var_nj0_db2 = assign20270_e29283_d_b2;
        var_nj0_db3 = assign20270_e29283_d_b3;

        let (assign20280_e29298, assign20280_e29298_d_n0, assign20280_e29298_d_n1, assign20280_e29298_d_n2, assign20280_e29298_d_n3, assign20280_e29298_d_n4, assign20280_e29298_d_n5, assign20280_e29298_d_b0, assign20280_e29298_d_b1, assign20280_e29298_d_b2, assign20280_e29298_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 != 0.0)) {
        let assign20280_e29294: f64 = (p.p86 * var_dfn_su);
        let assign20280_e29296: f64 = (assign20280_e29294 * var_dfn_sl);
        (assign20280_e29296, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign20280_e29294 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign20280_e29298;
        var_dnj1_dv_dn0 = assign20280_e29298_d_n0;
        var_dnj1_dv_dn1 = assign20280_e29298_d_n1;
        var_dnj1_dv_dn2 = assign20280_e29298_d_n2;
        var_dnj1_dv_dn3 = assign20280_e29298_d_n3;
        var_dnj1_dv_dn4 = assign20280_e29298_d_n4;
        var_dnj1_dv_dn5 = assign20280_e29298_d_n5;
        var_dnj1_dv_db0 = assign20280_e29298_d_b0;
        var_dnj1_dv_db1 = assign20280_e29298_d_b1;
        var_dnj1_dv_db2 = assign20280_e29298_d_b2;
        var_dnj1_dv_db3 = assign20280_e29298_d_b3;

        let (assign20290_e29310, assign20290_e29310_d_n0, assign20290_e29310_d_n1, assign20290_e29310_d_n2, assign20290_e29310_d_n3, assign20290_e29310_d_n4, assign20290_e29310_d_n5, assign20290_e29310_d_b0, assign20290_e29310_d_b1, assign20290_e29310_d_b2, assign20290_e29310_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20290_e29310;
        var_nj0_dn0 = assign20290_e29310_d_n0;
        var_nj0_dn1 = assign20290_e29310_d_n1;
        var_nj0_dn2 = assign20290_e29310_d_n2;
        var_nj0_dn3 = assign20290_e29310_d_n3;
        var_nj0_dn4 = assign20290_e29310_d_n4;
        var_nj0_dn5 = assign20290_e29310_d_n5;
        var_nj0_db0 = assign20290_e29310_d_b0;
        var_nj0_db1 = assign20290_e29310_d_b1;
        var_nj0_db2 = assign20290_e29310_d_b2;
        var_nj0_db3 = assign20290_e29310_d_b3;

        let (assign20300_e29322, assign20300_e29322_d_n0, assign20300_e29322_d_n1, assign20300_e29322_d_n2, assign20300_e29322_d_n3, assign20300_e29322_d_n4, assign20300_e29322_d_n5, assign20300_e29322_d_b0, assign20300_e29322_d_b1, assign20300_e29322_d_b2, assign20300_e29322_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign20300_e29322;
        var_nj1_dn0 = assign20300_e29322_d_n0;
        var_nj1_dn1 = assign20300_e29322_d_n1;
        var_nj1_dn2 = assign20300_e29322_d_n2;
        var_nj1_dn3 = assign20300_e29322_d_n3;
        var_nj1_dn4 = assign20300_e29322_d_n4;
        var_nj1_dn5 = assign20300_e29322_d_n5;
        var_nj1_db0 = assign20300_e29322_d_b0;
        var_nj1_db1 = assign20300_e29322_d_b1;
        var_nj1_db2 = assign20300_e29322_d_b2;
        var_nj1_db3 = assign20300_e29322_d_b3;

        let (assign20310_e29334, assign20310_e29334_d_n0, assign20310_e29334_d_n1, assign20310_e29334_d_n2, assign20310_e29334_d_n3, assign20310_e29334_d_n4, assign20310_e29334_d_n5, assign20310_e29334_d_b0, assign20310_e29334_d_b1, assign20310_e29334_d_b2, assign20310_e29334_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard323 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign20310_e29334;
        var_dnj1_dv_dn0 = assign20310_e29334_d_n0;
        var_dnj1_dv_dn1 = assign20310_e29334_d_n1;
        var_dnj1_dv_dn2 = assign20310_e29334_d_n2;
        var_dnj1_dv_dn3 = assign20310_e29334_d_n3;
        var_dnj1_dv_dn4 = assign20310_e29334_d_n4;
        var_dnj1_dv_dn5 = assign20310_e29334_d_n5;
        var_dnj1_dv_db0 = assign20310_e29334_d_b0;
        var_dnj1_dv_db1 = assign20310_e29334_d_b1;
        var_dnj1_dv_db2 = assign20310_e29334_d_b2;
        var_dnj1_dv_db3 = assign20310_e29334_d_b3;

        let (assign20370_e29583, assign20370_e29583_d_n0, assign20370_e29583_d_n1, assign20370_e29583_d_n2, assign20370_e29583_d_n3, assign20370_e29583_d_n4, assign20370_e29583_d_n5, assign20370_e29583_d_b0, assign20370_e29583_d_b1, assign20370_e29583_d_b2, assign20370_e29583_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20370_e29567: f64 = (var_vmax * var_dnj1_dv);
        let assign20370_e29568: f64 = (var_nj1 - assign20370_e29567);
        let assign20370_e29571: f64 = (var_nj1 * var_nj1);
        let assign20370_e29572: f64 = (assign20370_e29568 / assign20370_e29571);
        let assign20370_e29575: f64 = (var_vha1 * var_dnj1_dv);
        let assign20370_e29578: f64 = (var_nj0 * p.p85);
        let assign20370_e29579: f64 = (assign20370_e29575 / assign20370_e29578);
        let assign20370_e29580: f64 = (assign20370_e29572 + assign20370_e29579);
        let assign20370_e29581: f64 = (var_phitdinv * assign20370_e29580);
        (assign20370_e29581, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn0 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn1 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn2 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn3 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn4 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_dn5 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_db0) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_db0 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_db1) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_db1 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_db2) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_db2 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign20370_e29571) - (assign20370_e29568 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign20370_e29571 * assign20370_e29571)) + ((((var_vha1 * var_dnj1_dv_db3) * assign20370_e29578) - (assign20370_e29575 * (var_nj0_db3 * p.p85))) / (assign20370_e29578 * assign20370_e29578)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign20370_e29583;
        var_dvmax_over_phitd_dv_dn0 = assign20370_e29583_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign20370_e29583_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign20370_e29583_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign20370_e29583_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign20370_e29583_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign20370_e29583_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign20370_e29583_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign20370_e29583_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign20370_e29583_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign20370_e29583_d_b3;

        let (assign20390_e29613,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20390_e29609: f64 = (var_nin * var_nin);
        let assign20390_e29611: f64 = (assign20390_e29609 / var_ndigat_i);
        (assign20390_e29611,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign20390_e29613;

        let (assign20400_e29629,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20400_e29622: f64 = (var_nfagat_i / var_phitdinv);
        let assign20400_e29625: f64 = (var_ndigat_i / var_pnn0);
        let assign20400_e29626: f64 = (assign20400_e29625).ln();
        let assign20400_e29627: f64 = (assign20400_e29622 * assign20400_e29626);
        (assign20400_e29627,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign20400_e29629;

        let assign20410_e29632: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard326 = assign20410_e29632;

        let (assign20420_e29649, assign20420_e29649_d_n0, assign20420_e29649_d_n1, assign20420_e29649_d_n2, assign20420_e29649_d_n3, assign20420_e29649_d_n4, assign20420_e29649_d_n5, assign20420_e29649_d_b0, assign20420_e29649_d_b1, assign20420_e29649_d_b2, assign20420_e29649_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20420_e29644: f64 = (var_vmax - var_vha1);
        let assign20420_e29645: f64 = (p.p86 * assign20420_e29644);
        let assign20420_e29647: f64 = (assign20420_e29645 + var_nfagat_i);
        (assign20420_e29647, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign20420_e29649;
        var_nja10_dn0 = assign20420_e29649_d_n0;
        var_nja10_dn1 = assign20420_e29649_d_n1;
        var_nja10_dn2 = assign20420_e29649_d_n2;
        var_nja10_dn3 = assign20420_e29649_d_n3;
        var_nja10_dn4 = assign20420_e29649_d_n4;
        var_nja10_dn5 = assign20420_e29649_d_n5;
        var_nja10_db0 = assign20420_e29649_d_b0;
        var_nja10_db1 = assign20420_e29649_d_b1;
        var_nja10_db2 = assign20420_e29649_d_b2;
        var_nja10_db3 = assign20420_e29649_d_b3;

        let (assign20430_e29664, assign20430_e29664_d_n0, assign20430_e29664_d_n1, assign20430_e29664_d_n2, assign20430_e29664_d_n3, assign20430_e29664_d_n4, assign20430_e29664_d_n5, assign20430_e29664_d_b0, assign20430_e29664_d_b1, assign20430_e29664_d_b2, assign20430_e29664_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20430_e29661: f64 = (p.p86 * var_vha1);
        let assign20430_e29662: f64 = (var_nfagat_i - assign20430_e29661);
        (assign20430_e29662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20430_e29664;
        var_nj0_dn0 = assign20430_e29664_d_n0;
        var_nj0_dn1 = assign20430_e29664_d_n1;
        var_nj0_dn2 = assign20430_e29664_d_n2;
        var_nj0_dn3 = assign20430_e29664_d_n3;
        var_nj0_dn4 = assign20430_e29664_d_n4;
        var_nj0_dn5 = assign20430_e29664_d_n5;
        var_nj0_db0 = assign20430_e29664_d_b0;
        var_nj0_db1 = assign20430_e29664_d_b1;
        var_nj0_db2 = assign20430_e29664_d_b2;
        var_nj0_db3 = assign20430_e29664_d_b3;


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
        *var_guard326_slot = var_guard326;
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

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard326: f64,
        var_nfagat_i: f64,
        var_nja10: f64,
        var_nja10_db0: f64,
        var_nja10_db1: f64,
        var_nja10_db2: f64,
        var_nja10_db3: f64,
        var_nja10_dn0: f64,
        var_nja10_dn1: f64,
        var_nja10_dn2: f64,
        var_nja10_dn3: f64,
        var_nja10_dn4: f64,
        var_nja10_dn5: f64,
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

        let (assign20440_e29679, assign20440_e29679_d_n0, assign20440_e29679_d_n1, assign20440_e29679_d_n2, assign20440_e29679_d_n3, assign20440_e29679_d_n4, assign20440_e29679_d_n5, assign20440_e29679_d_b0, assign20440_e29679_d_b1, assign20440_e29679_d_b2, assign20440_e29679_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20440_e29675: f64 = (p.p85 - var_nja10);
        let assign20440_e29677: f64 = (assign20440_e29675 - 0.01);
        (assign20440_e29677, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20440_e29679;
        var_tmf1_dn0 = assign20440_e29679_d_n0;
        var_tmf1_dn1 = assign20440_e29679_d_n1;
        var_tmf1_dn2 = assign20440_e29679_d_n2;
        var_tmf1_dn3 = assign20440_e29679_d_n3;
        var_tmf1_dn4 = assign20440_e29679_d_n4;
        var_tmf1_dn5 = assign20440_e29679_d_n5;
        var_tmf1_db0 = assign20440_e29679_d_b0;
        var_tmf1_db1 = assign20440_e29679_d_b1;
        var_tmf1_db2 = assign20440_e29679_d_b2;
        var_tmf1_db3 = assign20440_e29679_d_b3;

        let (assign20450_e29694, assign20450_e29694_d_n0, assign20450_e29694_d_n1, assign20450_e29694_d_n2, assign20450_e29694_d_n3, assign20450_e29694_d_n4, assign20450_e29694_d_n5, assign20450_e29694_d_b0, assign20450_e29694_d_b1, assign20450_e29694_d_b2, assign20450_e29694_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20450_e29690: f64 = (4.0 * p.p85);
        let assign20450_e29692: f64 = (assign20450_e29690 * 0.01);
        (assign20450_e29692, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20450_e29694;
        var_tmf2_dn0 = assign20450_e29694_d_n0;
        var_tmf2_dn1 = assign20450_e29694_d_n1;
        var_tmf2_dn2 = assign20450_e29694_d_n2;
        var_tmf2_dn3 = assign20450_e29694_d_n3;
        var_tmf2_dn4 = assign20450_e29694_d_n4;
        var_tmf2_dn5 = assign20450_e29694_d_n5;
        var_tmf2_db0 = assign20450_e29694_d_b0;
        var_tmf2_db1 = assign20450_e29694_d_b1;
        var_tmf2_db2 = assign20450_e29694_d_b2;
        var_tmf2_db3 = assign20450_e29694_d_b3;

        let (assign20460_e29711, assign20460_e29711_d_n0, assign20460_e29711_d_n1, assign20460_e29711_d_n2, assign20460_e29711_d_n3, assign20460_e29711_d_n4, assign20460_e29711_d_n5, assign20460_e29711_d_b0, assign20460_e29711_d_b1, assign20460_e29711_d_b2, assign20460_e29711_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n1, assign20460_e29709_d_n2, assign20460_e29709_d_n3, assign20460_e29709_d_n4, assign20460_e29709_d_n5, assign20460_e29709_d_b0, assign20460_e29709_d_b1, assign20460_e29709_d_b2, assign20460_e29709_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20460_e29708: f64 = (-var_tmf2);
                (assign20460_e29708, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20460_e29709, assign20460_e29709_d_n0, assign20460_e29709_d_n1, assign20460_e29709_d_n2, assign20460_e29709_d_n3, assign20460_e29709_d_n4, assign20460_e29709_d_n5, assign20460_e29709_d_b0, assign20460_e29709_d_b1, assign20460_e29709_d_b2, assign20460_e29709_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20460_e29711;
        var_tmf2_dn0 = assign20460_e29711_d_n0;
        var_tmf2_dn1 = assign20460_e29711_d_n1;
        var_tmf2_dn2 = assign20460_e29711_d_n2;
        var_tmf2_dn3 = assign20460_e29711_d_n3;
        var_tmf2_dn4 = assign20460_e29711_d_n4;
        var_tmf2_dn5 = assign20460_e29711_d_n5;
        var_tmf2_db0 = assign20460_e29711_d_b0;
        var_tmf2_db1 = assign20460_e29711_d_b1;
        var_tmf2_db2 = assign20460_e29711_d_b2;
        var_tmf2_db3 = assign20460_e29711_d_b3;

        let (assign20470_e29727, assign20470_e29727_d_n0, assign20470_e29727_d_n1, assign20470_e29727_d_n2, assign20470_e29727_d_n3, assign20470_e29727_d_n4, assign20470_e29727_d_n5, assign20470_e29727_d_b0, assign20470_e29727_d_b1, assign20470_e29727_d_b2, assign20470_e29727_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20470_e29722: f64 = (var_tmf1 * var_tmf1);
        let assign20470_e29724: f64 = (assign20470_e29722 + var_tmf2);
        let assign20470_e29725: f64 = (assign20470_e29724).sqrt();
        (assign20470_e29725, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20470_e29725)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20470_e29725)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20470_e29725)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20470_e29725)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20470_e29725)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20470_e29725)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20470_e29727;
        var_tmf2_dn0 = assign20470_e29727_d_n0;
        var_tmf2_dn1 = assign20470_e29727_d_n1;
        var_tmf2_dn2 = assign20470_e29727_d_n2;
        var_tmf2_dn3 = assign20470_e29727_d_n3;
        var_tmf2_dn4 = assign20470_e29727_d_n4;
        var_tmf2_dn5 = assign20470_e29727_d_n5;
        var_tmf2_db0 = assign20470_e29727_d_b0;
        var_tmf2_db1 = assign20470_e29727_d_b1;
        var_tmf2_db2 = assign20470_e29727_d_b2;
        var_tmf2_db3 = assign20470_e29727_d_b3;

        let (assign20480_e29744, assign20480_e29744_d_n0, assign20480_e29744_d_n1, assign20480_e29744_d_n2, assign20480_e29744_d_n3, assign20480_e29744_d_n4, assign20480_e29744_d_n5, assign20480_e29744_d_b0, assign20480_e29744_d_b1, assign20480_e29744_d_b2, assign20480_e29744_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20480_e29740: f64 = (var_tmf1 / var_tmf2);
        let assign20480_e29741: f64 = (1.0 + assign20480_e29740);
        let assign20480_e29742: f64 = (0.5 * assign20480_e29741);
        (assign20480_e29742, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign20480_e29744;
        var_dfn_su_dn0 = assign20480_e29744_d_n0;
        var_dfn_su_dn1 = assign20480_e29744_d_n1;
        var_dfn_su_dn2 = assign20480_e29744_d_n2;
        var_dfn_su_dn3 = assign20480_e29744_d_n3;
        var_dfn_su_dn4 = assign20480_e29744_d_n4;
        var_dfn_su_dn5 = assign20480_e29744_d_n5;
        var_dfn_su_db0 = assign20480_e29744_d_b0;
        var_dfn_su_db1 = assign20480_e29744_d_b1;
        var_dfn_su_db2 = assign20480_e29744_d_b2;
        var_dfn_su_db3 = assign20480_e29744_d_b3;

        let (assign20490_e29761, assign20490_e29761_d_n0, assign20490_e29761_d_n1, assign20490_e29761_d_n2, assign20490_e29761_d_n3, assign20490_e29761_d_n4, assign20490_e29761_d_n5, assign20490_e29761_d_b0, assign20490_e29761_d_b1, assign20490_e29761_d_b2, assign20490_e29761_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20490_e29757: f64 = (var_tmf1 + var_tmf2);
        let assign20490_e29758: f64 = (0.5 * assign20490_e29757);
        let assign20490_e29759: f64 = (p.p85 - assign20490_e29758);
        (assign20490_e29759, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign20490_e29761;
        var_nja11_dn0 = assign20490_e29761_d_n0;
        var_nja11_dn1 = assign20490_e29761_d_n1;
        var_nja11_dn2 = assign20490_e29761_d_n2;
        var_nja11_dn3 = assign20490_e29761_d_n3;
        var_nja11_dn4 = assign20490_e29761_d_n4;
        var_nja11_dn5 = assign20490_e29761_d_n5;
        var_nja11_db0 = assign20490_e29761_d_b0;
        var_nja11_db1 = assign20490_e29761_d_b1;
        var_nja11_db2 = assign20490_e29761_d_b2;
        var_nja11_db3 = assign20490_e29761_d_b3;

        let (assign20500_e29776, assign20500_e29776_d_n0, assign20500_e29776_d_n1, assign20500_e29776_d_n2, assign20500_e29776_d_n3, assign20500_e29776_d_n4, assign20500_e29776_d_n5, assign20500_e29776_d_b0, assign20500_e29776_d_b1, assign20500_e29776_d_b2, assign20500_e29776_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20500_e29772: f64 = (var_nja11 - var_nfagat_i);
        let assign20500_e29774: f64 = (assign20500_e29772 - 0.01);
        (assign20500_e29774, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20500_e29776;
        var_tmf1_dn0 = assign20500_e29776_d_n0;
        var_tmf1_dn1 = assign20500_e29776_d_n1;
        var_tmf1_dn2 = assign20500_e29776_d_n2;
        var_tmf1_dn3 = assign20500_e29776_d_n3;
        var_tmf1_dn4 = assign20500_e29776_d_n4;
        var_tmf1_dn5 = assign20500_e29776_d_n5;
        var_tmf1_db0 = assign20500_e29776_d_b0;
        var_tmf1_db1 = assign20500_e29776_d_b1;
        var_tmf1_db2 = assign20500_e29776_d_b2;
        var_tmf1_db3 = assign20500_e29776_d_b3;

        let (assign20510_e29791, assign20510_e29791_d_n0, assign20510_e29791_d_n1, assign20510_e29791_d_n2, assign20510_e29791_d_n3, assign20510_e29791_d_n4, assign20510_e29791_d_n5, assign20510_e29791_d_b0, assign20510_e29791_d_b1, assign20510_e29791_d_b2, assign20510_e29791_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20510_e29787: f64 = (4.0 * var_nfagat_i);
        let assign20510_e29789: f64 = (assign20510_e29787 * 0.01);
        (assign20510_e29789, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20510_e29791;
        var_tmf2_dn0 = assign20510_e29791_d_n0;
        var_tmf2_dn1 = assign20510_e29791_d_n1;
        var_tmf2_dn2 = assign20510_e29791_d_n2;
        var_tmf2_dn3 = assign20510_e29791_d_n3;
        var_tmf2_dn4 = assign20510_e29791_d_n4;
        var_tmf2_dn5 = assign20510_e29791_d_n5;
        var_tmf2_db0 = assign20510_e29791_d_b0;
        var_tmf2_db1 = assign20510_e29791_d_b1;
        var_tmf2_db2 = assign20510_e29791_d_b2;
        var_tmf2_db3 = assign20510_e29791_d_b3;

        let (assign20520_e29808, assign20520_e29808_d_n0, assign20520_e29808_d_n1, assign20520_e29808_d_n2, assign20520_e29808_d_n3, assign20520_e29808_d_n4, assign20520_e29808_d_n5, assign20520_e29808_d_b0, assign20520_e29808_d_b1, assign20520_e29808_d_b2, assign20520_e29808_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n1, assign20520_e29806_d_n2, assign20520_e29806_d_n3, assign20520_e29806_d_n4, assign20520_e29806_d_n5, assign20520_e29806_d_b0, assign20520_e29806_d_b1, assign20520_e29806_d_b2, assign20520_e29806_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20520_e29805: f64 = (-var_tmf2);
                (assign20520_e29805, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20520_e29806, assign20520_e29806_d_n0, assign20520_e29806_d_n1, assign20520_e29806_d_n2, assign20520_e29806_d_n3, assign20520_e29806_d_n4, assign20520_e29806_d_n5, assign20520_e29806_d_b0, assign20520_e29806_d_b1, assign20520_e29806_d_b2, assign20520_e29806_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20520_e29808;
        var_tmf2_dn0 = assign20520_e29808_d_n0;
        var_tmf2_dn1 = assign20520_e29808_d_n1;
        var_tmf2_dn2 = assign20520_e29808_d_n2;
        var_tmf2_dn3 = assign20520_e29808_d_n3;
        var_tmf2_dn4 = assign20520_e29808_d_n4;
        var_tmf2_dn5 = assign20520_e29808_d_n5;
        var_tmf2_db0 = assign20520_e29808_d_b0;
        var_tmf2_db1 = assign20520_e29808_d_b1;
        var_tmf2_db2 = assign20520_e29808_d_b2;
        var_tmf2_db3 = assign20520_e29808_d_b3;

        let (assign20530_e29824, assign20530_e29824_d_n0, assign20530_e29824_d_n1, assign20530_e29824_d_n2, assign20530_e29824_d_n3, assign20530_e29824_d_n4, assign20530_e29824_d_n5, assign20530_e29824_d_b0, assign20530_e29824_d_b1, assign20530_e29824_d_b2, assign20530_e29824_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20530_e29819: f64 = (var_tmf1 * var_tmf1);
        let assign20530_e29821: f64 = (assign20530_e29819 + var_tmf2);
        let assign20530_e29822: f64 = (assign20530_e29821).sqrt();
        (assign20530_e29822, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20530_e29822)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20530_e29822)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20530_e29822)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20530_e29822)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20530_e29822)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20530_e29822)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20530_e29824;
        var_tmf2_dn0 = assign20530_e29824_d_n0;
        var_tmf2_dn1 = assign20530_e29824_d_n1;
        var_tmf2_dn2 = assign20530_e29824_d_n2;
        var_tmf2_dn3 = assign20530_e29824_d_n3;
        var_tmf2_dn4 = assign20530_e29824_d_n4;
        var_tmf2_dn5 = assign20530_e29824_d_n5;
        var_tmf2_db0 = assign20530_e29824_d_b0;
        var_tmf2_db1 = assign20530_e29824_d_b1;
        var_tmf2_db2 = assign20530_e29824_d_b2;
        var_tmf2_db3 = assign20530_e29824_d_b3;

        let (assign20540_e29841, assign20540_e29841_d_n0, assign20540_e29841_d_n1, assign20540_e29841_d_n2, assign20540_e29841_d_n3, assign20540_e29841_d_n4, assign20540_e29841_d_n5, assign20540_e29841_d_b0, assign20540_e29841_d_b1, assign20540_e29841_d_b2, assign20540_e29841_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20540_e29837: f64 = (var_tmf1 / var_tmf2);
        let assign20540_e29838: f64 = (1.0 + assign20540_e29837);
        let assign20540_e29839: f64 = (0.5 * assign20540_e29838);
        (assign20540_e29839, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign20540_e29841;
        var_dfn_sl_dn0 = assign20540_e29841_d_n0;
        var_dfn_sl_dn1 = assign20540_e29841_d_n1;
        var_dfn_sl_dn2 = assign20540_e29841_d_n2;
        var_dfn_sl_dn3 = assign20540_e29841_d_n3;
        var_dfn_sl_dn4 = assign20540_e29841_d_n4;
        var_dfn_sl_dn5 = assign20540_e29841_d_n5;
        var_dfn_sl_db0 = assign20540_e29841_d_b0;
        var_dfn_sl_db1 = assign20540_e29841_d_b1;
        var_dfn_sl_db2 = assign20540_e29841_d_b2;
        var_dfn_sl_db3 = assign20540_e29841_d_b3;

        let (assign20550_e29858, assign20550_e29858_d_n0, assign20550_e29858_d_n1, assign20550_e29858_d_n2, assign20550_e29858_d_n3, assign20550_e29858_d_n4, assign20550_e29858_d_n5, assign20550_e29858_d_b0, assign20550_e29858_d_b1, assign20550_e29858_d_b2, assign20550_e29858_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20550_e29854: f64 = (var_tmf1 + var_tmf2);
        let assign20550_e29855: f64 = (0.5 * assign20550_e29854);
        let assign20550_e29856: f64 = (var_nfagat_i + assign20550_e29855);
        (assign20550_e29856, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign20550_e29858;
        var_nj1_dn0 = assign20550_e29858_d_n0;
        var_nj1_dn1 = assign20550_e29858_d_n1;
        var_nj1_dn2 = assign20550_e29858_d_n2;
        var_nj1_dn3 = assign20550_e29858_d_n3;
        var_nj1_dn4 = assign20550_e29858_d_n4;
        var_nj1_dn5 = assign20550_e29858_d_n5;
        var_nj1_db0 = assign20550_e29858_d_b0;
        var_nj1_db1 = assign20550_e29858_d_b1;
        var_nj1_db2 = assign20550_e29858_d_b2;
        var_nj1_db3 = assign20550_e29858_d_b3;

        let (assign20560_e29873, assign20560_e29873_d_n0, assign20560_e29873_d_n1, assign20560_e29873_d_n2, assign20560_e29873_d_n3, assign20560_e29873_d_n4, assign20560_e29873_d_n5, assign20560_e29873_d_b0, assign20560_e29873_d_b1, assign20560_e29873_d_b2, assign20560_e29873_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20560_e29869: f64 = (p.p85 - var_nj0);
        let assign20560_e29871: f64 = (assign20560_e29869 - 0.01);
        (assign20560_e29871, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20560_e29873;
        var_tmf1_dn0 = assign20560_e29873_d_n0;
        var_tmf1_dn1 = assign20560_e29873_d_n1;
        var_tmf1_dn2 = assign20560_e29873_d_n2;
        var_tmf1_dn3 = assign20560_e29873_d_n3;
        var_tmf1_dn4 = assign20560_e29873_d_n4;
        var_tmf1_dn5 = assign20560_e29873_d_n5;
        var_tmf1_db0 = assign20560_e29873_d_b0;
        var_tmf1_db1 = assign20560_e29873_d_b1;
        var_tmf1_db2 = assign20560_e29873_d_b2;
        var_tmf1_db3 = assign20560_e29873_d_b3;

        let (assign20570_e29888, assign20570_e29888_d_n0, assign20570_e29888_d_n1, assign20570_e29888_d_n2, assign20570_e29888_d_n3, assign20570_e29888_d_n4, assign20570_e29888_d_n5, assign20570_e29888_d_b0, assign20570_e29888_d_b1, assign20570_e29888_d_b2, assign20570_e29888_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20570_e29884: f64 = (4.0 * p.p85);
        let assign20570_e29886: f64 = (assign20570_e29884 * 0.01);
        (assign20570_e29886, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20570_e29888;
        var_tmf2_dn0 = assign20570_e29888_d_n0;
        var_tmf2_dn1 = assign20570_e29888_d_n1;
        var_tmf2_dn2 = assign20570_e29888_d_n2;
        var_tmf2_dn3 = assign20570_e29888_d_n3;
        var_tmf2_dn4 = assign20570_e29888_d_n4;
        var_tmf2_dn5 = assign20570_e29888_d_n5;
        var_tmf2_db0 = assign20570_e29888_d_b0;
        var_tmf2_db1 = assign20570_e29888_d_b1;
        var_tmf2_db2 = assign20570_e29888_d_b2;
        var_tmf2_db3 = assign20570_e29888_d_b3;

        let (assign20580_e29905, assign20580_e29905_d_n0, assign20580_e29905_d_n1, assign20580_e29905_d_n2, assign20580_e29905_d_n3, assign20580_e29905_d_n4, assign20580_e29905_d_n5, assign20580_e29905_d_b0, assign20580_e29905_d_b1, assign20580_e29905_d_b2, assign20580_e29905_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n1, assign20580_e29903_d_n2, assign20580_e29903_d_n3, assign20580_e29903_d_n4, assign20580_e29903_d_n5, assign20580_e29903_d_b0, assign20580_e29903_d_b1, assign20580_e29903_d_b2, assign20580_e29903_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20580_e29902: f64 = (-var_tmf2);
                (assign20580_e29902, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20580_e29903, assign20580_e29903_d_n0, assign20580_e29903_d_n1, assign20580_e29903_d_n2, assign20580_e29903_d_n3, assign20580_e29903_d_n4, assign20580_e29903_d_n5, assign20580_e29903_d_b0, assign20580_e29903_d_b1, assign20580_e29903_d_b2, assign20580_e29903_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20580_e29905;
        var_tmf2_dn0 = assign20580_e29905_d_n0;
        var_tmf2_dn1 = assign20580_e29905_d_n1;
        var_tmf2_dn2 = assign20580_e29905_d_n2;
        var_tmf2_dn3 = assign20580_e29905_d_n3;
        var_tmf2_dn4 = assign20580_e29905_d_n4;
        var_tmf2_dn5 = assign20580_e29905_d_n5;
        var_tmf2_db0 = assign20580_e29905_d_b0;
        var_tmf2_db1 = assign20580_e29905_d_b1;
        var_tmf2_db2 = assign20580_e29905_d_b2;
        var_tmf2_db3 = assign20580_e29905_d_b3;

        let (assign20590_e29921, assign20590_e29921_d_n0, assign20590_e29921_d_n1, assign20590_e29921_d_n2, assign20590_e29921_d_n3, assign20590_e29921_d_n4, assign20590_e29921_d_n5, assign20590_e29921_d_b0, assign20590_e29921_d_b1, assign20590_e29921_d_b2, assign20590_e29921_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20590_e29916: f64 = (var_tmf1 * var_tmf1);
        let assign20590_e29918: f64 = (assign20590_e29916 + var_tmf2);
        let assign20590_e29919: f64 = (assign20590_e29918).sqrt();
        (assign20590_e29919, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20590_e29919)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20590_e29919)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20590_e29919)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20590_e29919)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20590_e29919)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20590_e29919)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20590_e29921;
        var_tmf2_dn0 = assign20590_e29921_d_n0;
        var_tmf2_dn1 = assign20590_e29921_d_n1;
        var_tmf2_dn2 = assign20590_e29921_d_n2;
        var_tmf2_dn3 = assign20590_e29921_d_n3;
        var_tmf2_dn4 = assign20590_e29921_d_n4;
        var_tmf2_dn5 = assign20590_e29921_d_n5;
        var_tmf2_db0 = assign20590_e29921_d_b0;
        var_tmf2_db1 = assign20590_e29921_d_b1;
        var_tmf2_db2 = assign20590_e29921_d_b2;
        var_tmf2_db3 = assign20590_e29921_d_b3;

        let (assign20600_e29938, assign20600_e29938_d_n0, assign20600_e29938_d_n1, assign20600_e29938_d_n2, assign20600_e29938_d_n3, assign20600_e29938_d_n4, assign20600_e29938_d_n5, assign20600_e29938_d_b0, assign20600_e29938_d_b1, assign20600_e29938_d_b2, assign20600_e29938_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20600_e29934: f64 = (var_tmf1 + var_tmf2);
        let assign20600_e29935: f64 = (0.5 * assign20600_e29934);
        let assign20600_e29936: f64 = (p.p85 - assign20600_e29935);
        (assign20600_e29936, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20600_e29938;
        var_nj0_dn0 = assign20600_e29938_d_n0;
        var_nj0_dn1 = assign20600_e29938_d_n1;
        var_nj0_dn2 = assign20600_e29938_d_n2;
        var_nj0_dn3 = assign20600_e29938_d_n3;
        var_nj0_dn4 = assign20600_e29938_d_n4;
        var_nj0_dn5 = assign20600_e29938_d_n5;
        var_nj0_db0 = assign20600_e29938_d_b0;
        var_nj0_db1 = assign20600_e29938_d_b1;
        var_nj0_db2 = assign20600_e29938_d_b2;
        var_nj0_db3 = assign20600_e29938_d_b3;

        let (assign20610_e29953, assign20610_e29953_d_n0, assign20610_e29953_d_n1, assign20610_e29953_d_n2, assign20610_e29953_d_n3, assign20610_e29953_d_n4, assign20610_e29953_d_n5, assign20610_e29953_d_b0, assign20610_e29953_d_b1, assign20610_e29953_d_b2, assign20610_e29953_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20610_e29949: f64 = (var_nj0 - var_nfagat_i);
        let assign20610_e29951: f64 = (assign20610_e29949 - 0.01);
        (assign20610_e29951, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign20610_e29953;
        var_tmf1_dn0 = assign20610_e29953_d_n0;
        var_tmf1_dn1 = assign20610_e29953_d_n1;
        var_tmf1_dn2 = assign20610_e29953_d_n2;
        var_tmf1_dn3 = assign20610_e29953_d_n3;
        var_tmf1_dn4 = assign20610_e29953_d_n4;
        var_tmf1_dn5 = assign20610_e29953_d_n5;
        var_tmf1_db0 = assign20610_e29953_d_b0;
        var_tmf1_db1 = assign20610_e29953_d_b1;
        var_tmf1_db2 = assign20610_e29953_d_b2;
        var_tmf1_db3 = assign20610_e29953_d_b3;

        let (assign20620_e29968, assign20620_e29968_d_n0, assign20620_e29968_d_n1, assign20620_e29968_d_n2, assign20620_e29968_d_n3, assign20620_e29968_d_n4, assign20620_e29968_d_n5, assign20620_e29968_d_b0, assign20620_e29968_d_b1, assign20620_e29968_d_b2, assign20620_e29968_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20620_e29964: f64 = (4.0 * var_nfagat_i);
        let assign20620_e29966: f64 = (assign20620_e29964 * 0.01);
        (assign20620_e29966, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20620_e29968;
        var_tmf2_dn0 = assign20620_e29968_d_n0;
        var_tmf2_dn1 = assign20620_e29968_d_n1;
        var_tmf2_dn2 = assign20620_e29968_d_n2;
        var_tmf2_dn3 = assign20620_e29968_d_n3;
        var_tmf2_dn4 = assign20620_e29968_d_n4;
        var_tmf2_dn5 = assign20620_e29968_d_n5;
        var_tmf2_db0 = assign20620_e29968_d_b0;
        var_tmf2_db1 = assign20620_e29968_d_b1;
        var_tmf2_db2 = assign20620_e29968_d_b2;
        var_tmf2_db3 = assign20620_e29968_d_b3;

        let (assign20630_e29985, assign20630_e29985_d_n0, assign20630_e29985_d_n1, assign20630_e29985_d_n2, assign20630_e29985_d_n3, assign20630_e29985_d_n4, assign20630_e29985_d_n5, assign20630_e29985_d_b0, assign20630_e29985_d_b1, assign20630_e29985_d_b2, assign20630_e29985_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n1, assign20630_e29983_d_n2, assign20630_e29983_d_n3, assign20630_e29983_d_n4, assign20630_e29983_d_n5, assign20630_e29983_d_b0, assign20630_e29983_d_b1, assign20630_e29983_d_b2, assign20630_e29983_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign20630_e29982: f64 = (-var_tmf2);
                (assign20630_e29982, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign20630_e29983, assign20630_e29983_d_n0, assign20630_e29983_d_n1, assign20630_e29983_d_n2, assign20630_e29983_d_n3, assign20630_e29983_d_n4, assign20630_e29983_d_n5, assign20630_e29983_d_b0, assign20630_e29983_d_b1, assign20630_e29983_d_b2, assign20630_e29983_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20630_e29985;
        var_tmf2_dn0 = assign20630_e29985_d_n0;
        var_tmf2_dn1 = assign20630_e29985_d_n1;
        var_tmf2_dn2 = assign20630_e29985_d_n2;
        var_tmf2_dn3 = assign20630_e29985_d_n3;
        var_tmf2_dn4 = assign20630_e29985_d_n4;
        var_tmf2_dn5 = assign20630_e29985_d_n5;
        var_tmf2_db0 = assign20630_e29985_d_b0;
        var_tmf2_db1 = assign20630_e29985_d_b1;
        var_tmf2_db2 = assign20630_e29985_d_b2;
        var_tmf2_db3 = assign20630_e29985_d_b3;

        let (assign20640_e30001, assign20640_e30001_d_n0, assign20640_e30001_d_n1, assign20640_e30001_d_n2, assign20640_e30001_d_n3, assign20640_e30001_d_n4, assign20640_e30001_d_n5, assign20640_e30001_d_b0, assign20640_e30001_d_b1, assign20640_e30001_d_b2, assign20640_e30001_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20640_e29996: f64 = (var_tmf1 * var_tmf1);
        let assign20640_e29998: f64 = (assign20640_e29996 + var_tmf2);
        let assign20640_e29999: f64 = (assign20640_e29998).sqrt();
        (assign20640_e29999, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign20640_e29999)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign20640_e29999)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign20640_e29999)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign20640_e29999)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign20640_e29999)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign20640_e29999)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign20640_e30001;
        var_tmf2_dn0 = assign20640_e30001_d_n0;
        var_tmf2_dn1 = assign20640_e30001_d_n1;
        var_tmf2_dn2 = assign20640_e30001_d_n2;
        var_tmf2_dn3 = assign20640_e30001_d_n3;
        var_tmf2_dn4 = assign20640_e30001_d_n4;
        var_tmf2_dn5 = assign20640_e30001_d_n5;
        var_tmf2_db0 = assign20640_e30001_d_b0;
        var_tmf2_db1 = assign20640_e30001_d_b1;
        var_tmf2_db2 = assign20640_e30001_d_b2;
        var_tmf2_db3 = assign20640_e30001_d_b3;

        let (assign20650_e30018, assign20650_e30018_d_n0, assign20650_e30018_d_n1, assign20650_e30018_d_n2, assign20650_e30018_d_n3, assign20650_e30018_d_n4, assign20650_e30018_d_n5, assign20650_e30018_d_b0, assign20650_e30018_d_b1, assign20650_e30018_d_b2, assign20650_e30018_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20650_e30014: f64 = (var_tmf1 + var_tmf2);
        let assign20650_e30015: f64 = (0.5 * assign20650_e30014);
        let assign20650_e30016: f64 = (var_nfagat_i + assign20650_e30015);
        (assign20650_e30016, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20650_e30018;
        var_nj0_dn0 = assign20650_e30018_d_n0;
        var_nj0_dn1 = assign20650_e30018_d_n1;
        var_nj0_dn2 = assign20650_e30018_d_n2;
        var_nj0_dn3 = assign20650_e30018_d_n3;
        var_nj0_dn4 = assign20650_e30018_d_n4;
        var_nj0_dn5 = assign20650_e30018_d_n5;
        var_nj0_db0 = assign20650_e30018_d_b0;
        var_nj0_db1 = assign20650_e30018_d_b1;
        var_nj0_db2 = assign20650_e30018_d_b2;
        var_nj0_db3 = assign20650_e30018_d_b3;


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

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        var_ab_i: f64,
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
        var_guard307: f64,
        var_guard308: f64,
        var_guard31: f64,
        var_guard326: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
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
        var_guard384_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_guard388_slot: &mut f64,
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
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_guard388: f64 = *var_guard388_slot;
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

        let (assign20660_e30033, assign20660_e30033_d_n0, assign20660_e30033_d_n1, assign20660_e30033_d_n2, assign20660_e30033_d_n3, assign20660_e30033_d_n4, assign20660_e30033_d_n5, assign20660_e30033_d_b0, assign20660_e30033_d_b1, assign20660_e30033_d_b2, assign20660_e30033_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 != 0.0)) {
        let assign20660_e30029: f64 = (p.p86 * var_dfn_su);
        let assign20660_e30031: f64 = (assign20660_e30029 * var_dfn_sl);
        (assign20660_e30031, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign20660_e30029 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign20660_e30033;
        var_dnj1_dv_dn0 = assign20660_e30033_d_n0;
        var_dnj1_dv_dn1 = assign20660_e30033_d_n1;
        var_dnj1_dv_dn2 = assign20660_e30033_d_n2;
        var_dnj1_dv_dn3 = assign20660_e30033_d_n3;
        var_dnj1_dv_dn4 = assign20660_e30033_d_n4;
        var_dnj1_dv_dn5 = assign20660_e30033_d_n5;
        var_dnj1_dv_db0 = assign20660_e30033_d_b0;
        var_dnj1_dv_db1 = assign20660_e30033_d_b1;
        var_dnj1_dv_db2 = assign20660_e30033_d_b2;
        var_dnj1_dv_db3 = assign20660_e30033_d_b3;

        let (assign20670_e30045, assign20670_e30045_d_n0, assign20670_e30045_d_n1, assign20670_e30045_d_n2, assign20670_e30045_d_n3, assign20670_e30045_d_n4, assign20670_e30045_d_n5, assign20670_e30045_d_b0, assign20670_e30045_d_b1, assign20670_e30045_d_b2, assign20670_e30045_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign20670_e30045;
        var_nj0_dn0 = assign20670_e30045_d_n0;
        var_nj0_dn1 = assign20670_e30045_d_n1;
        var_nj0_dn2 = assign20670_e30045_d_n2;
        var_nj0_dn3 = assign20670_e30045_d_n3;
        var_nj0_dn4 = assign20670_e30045_d_n4;
        var_nj0_dn5 = assign20670_e30045_d_n5;
        var_nj0_db0 = assign20670_e30045_d_b0;
        var_nj0_db1 = assign20670_e30045_d_b1;
        var_nj0_db2 = assign20670_e30045_d_b2;
        var_nj0_db3 = assign20670_e30045_d_b3;

        let (assign20680_e30057, assign20680_e30057_d_n0, assign20680_e30057_d_n1, assign20680_e30057_d_n2, assign20680_e30057_d_n3, assign20680_e30057_d_n4, assign20680_e30057_d_n5, assign20680_e30057_d_b0, assign20680_e30057_d_b1, assign20680_e30057_d_b2, assign20680_e30057_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign20680_e30057;
        var_nj1_dn0 = assign20680_e30057_d_n0;
        var_nj1_dn1 = assign20680_e30057_d_n1;
        var_nj1_dn2 = assign20680_e30057_d_n2;
        var_nj1_dn3 = assign20680_e30057_d_n3;
        var_nj1_dn4 = assign20680_e30057_d_n4;
        var_nj1_dn5 = assign20680_e30057_d_n5;
        var_nj1_db0 = assign20680_e30057_d_b0;
        var_nj1_db1 = assign20680_e30057_d_b1;
        var_nj1_db2 = assign20680_e30057_d_b2;
        var_nj1_db3 = assign20680_e30057_d_b3;

        let (assign20690_e30069, assign20690_e30069_d_n0, assign20690_e30069_d_n1, assign20690_e30069_d_n2, assign20690_e30069_d_n3, assign20690_e30069_d_n4, assign20690_e30069_d_n5, assign20690_e30069_d_b0, assign20690_e30069_d_b1, assign20690_e30069_d_b2, assign20690_e30069_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) && (var_guard326 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign20690_e30069;
        var_dnj1_dv_dn0 = assign20690_e30069_d_n0;
        var_dnj1_dv_dn1 = assign20690_e30069_d_n1;
        var_dnj1_dv_dn2 = assign20690_e30069_d_n2;
        var_dnj1_dv_dn3 = assign20690_e30069_d_n3;
        var_dnj1_dv_dn4 = assign20690_e30069_d_n4;
        var_dnj1_dv_dn5 = assign20690_e30069_d_n5;
        var_dnj1_dv_db0 = assign20690_e30069_d_b0;
        var_dnj1_dv_db1 = assign20690_e30069_d_b1;
        var_dnj1_dv_db2 = assign20690_e30069_d_b2;
        var_dnj1_dv_db3 = assign20690_e30069_d_b3;

        let (assign20750_e30318, assign20750_e30318_d_n0, assign20750_e30318_d_n1, assign20750_e30318_d_n2, assign20750_e30318_d_n3, assign20750_e30318_d_n4, assign20750_e30318_d_n5, assign20750_e30318_d_b0, assign20750_e30318_d_b1, assign20750_e30318_d_b2, assign20750_e30318_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign20750_e30302: f64 = (var_vmax * var_dnj1_dv);
        let assign20750_e30303: f64 = (var_nj1 - assign20750_e30302);
        let assign20750_e30306: f64 = (var_nj1 * var_nj1);
        let assign20750_e30307: f64 = (assign20750_e30303 / assign20750_e30306);
        let assign20750_e30310: f64 = (var_vha1 * var_dnj1_dv);
        let assign20750_e30313: f64 = (var_nj0 * p.p85);
        let assign20750_e30314: f64 = (assign20750_e30310 / assign20750_e30313);
        let assign20750_e30315: f64 = (assign20750_e30307 + assign20750_e30314);
        let assign20750_e30316: f64 = (var_phitdinv * assign20750_e30315);
        (assign20750_e30316, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn0 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn1 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn2 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn3 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn4 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_dn5 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_db0) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_db0 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_db1) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_db1 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_db2) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_db2 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign20750_e30306) - (assign20750_e30303 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign20750_e30306 * assign20750_e30306)) + ((((var_vha1 * var_dnj1_dv_db3) * assign20750_e30313) - (assign20750_e30310 * (var_nj0_db3 * p.p85))) / (assign20750_e30313 * assign20750_e30313)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign20750_e30318;
        var_dvmax_over_phitd_dv_dn0 = assign20750_e30318_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign20750_e30318_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign20750_e30318_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign20750_e30318_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign20750_e30318_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign20750_e30318_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign20750_e30318_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign20750_e30318_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign20750_e30318_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign20750_e30318_d_b3;

        let (assign20770_e30343, assign20770_e30343_d_n0, assign20770_e30343_d_n1, assign20770_e30343_d_n2, assign20770_e30343_d_n3, assign20770_e30343_d_n4, assign20770_e30343_d_n5, assign20770_e30343_d_b0, assign20770_e30343_d_b1, assign20770_e30343_d_b2, assign20770_e30343_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard307 != 0.0)) {
        let assign20770_e30341: f64 = (var_idmultbot - 1.0);
        (assign20770_e30341, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign20770_e30343;
        var_idmultbot_dn0 = assign20770_e30343_d_n0;
        var_idmultbot_dn1 = assign20770_e30343_d_n1;
        var_idmultbot_dn2 = assign20770_e30343_d_n2;
        var_idmultbot_dn3 = assign20770_e30343_d_n3;
        var_idmultbot_dn4 = assign20770_e30343_d_n4;
        var_idmultbot_dn5 = assign20770_e30343_d_n5;
        var_idmultbot_db0 = assign20770_e30343_d_b0;
        var_idmultbot_db1 = assign20770_e30343_d_b1;
        var_idmultbot_db2 = assign20770_e30343_d_b2;
        var_idmultbot_db3 = assign20770_e30343_d_b3;

        let (assign20880_e30516, assign20880_e30516_d_n0, assign20880_e30516_d_n1, assign20880_e30516_d_n2, assign20880_e30516_d_n3, assign20880_e30516_d_n4, assign20880_e30516_d_n5, assign20880_e30516_d_b0, assign20880_e30516_d_b1, assign20880_e30516_d_b2, assign20880_e30516_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard307 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign20880_e30516;
        var_idmultbot_dn0 = assign20880_e30516_d_n0;
        var_idmultbot_dn1 = assign20880_e30516_d_n1;
        var_idmultbot_dn2 = assign20880_e30516_d_n2;
        var_idmultbot_dn3 = assign20880_e30516_d_n3;
        var_idmultbot_dn4 = assign20880_e30516_d_n4;
        var_idmultbot_dn5 = assign20880_e30516_d_n5;
        var_idmultbot_db0 = assign20880_e30516_d_b0;
        var_idmultbot_db1 = assign20880_e30516_d_b1;
        var_idmultbot_db2 = assign20880_e30516_d_b2;
        var_idmultbot_db3 = assign20880_e30516_d_b3;

        let assign23410_e34066: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard384 = assign23410_e34066;

        let assign23490_e34138: f64 = if var_v5 < var_vmax { 1.0 } else { 0.0 };
        var_guard385 = assign23490_e34138;

        let (assign23550_e34279,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23550_e34275: f64 = (var_nin * var_nin);
        let assign23550_e34277: f64 = (assign23550_e34275 / var_ndibot_i);
        (assign23550_e34277,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign23550_e34279;

        let (assign23560_e34294,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23560_e34287: f64 = (var_nfabot_i / var_phitdinv);
        let assign23560_e34290: f64 = (var_ndibot_i / var_pnn0);
        let assign23560_e34291: f64 = (assign23560_e34290).ln();
        let assign23560_e34292: f64 = (assign23560_e34287 * assign23560_e34291);
        (assign23560_e34292,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign23560_e34294;

        let assign23570_e34297: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard388 = assign23570_e34297;

        let (assign23580_e34313, assign23580_e34313_d_n0, assign23580_e34313_d_n1, assign23580_e34313_d_n2, assign23580_e34313_d_n3, assign23580_e34313_d_n4, assign23580_e34313_d_n5, assign23580_e34313_d_b0, assign23580_e34313_d_b1, assign23580_e34313_d_b2, assign23580_e34313_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23580_e34308: f64 = (var_v5 - var_vha1);
        let assign23580_e34309: f64 = (p.p86 * assign23580_e34308);
        let assign23580_e34311: f64 = (assign23580_e34309 + var_nfabot_i);
        (assign23580_e34311, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign23580_e34313;
        var_nja10_dn0 = assign23580_e34313_d_n0;
        var_nja10_dn1 = assign23580_e34313_d_n1;
        var_nja10_dn2 = assign23580_e34313_d_n2;
        var_nja10_dn3 = assign23580_e34313_d_n3;
        var_nja10_dn4 = assign23580_e34313_d_n4;
        var_nja10_dn5 = assign23580_e34313_d_n5;
        var_nja10_db0 = assign23580_e34313_d_b0;
        var_nja10_db1 = assign23580_e34313_d_b1;
        var_nja10_db2 = assign23580_e34313_d_b2;
        var_nja10_db3 = assign23580_e34313_d_b3;

        let (assign23590_e34327, assign23590_e34327_d_n0, assign23590_e34327_d_n1, assign23590_e34327_d_n2, assign23590_e34327_d_n3, assign23590_e34327_d_n4, assign23590_e34327_d_n5, assign23590_e34327_d_b0, assign23590_e34327_d_b1, assign23590_e34327_d_b2, assign23590_e34327_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23590_e34324: f64 = (p.p86 * var_vha1);
        let assign23590_e34325: f64 = (var_nfabot_i - assign23590_e34324);
        (assign23590_e34325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign23590_e34327;
        var_nj0_dn0 = assign23590_e34327_d_n0;
        var_nj0_dn1 = assign23590_e34327_d_n1;
        var_nj0_dn2 = assign23590_e34327_d_n2;
        var_nj0_dn3 = assign23590_e34327_d_n3;
        var_nj0_dn4 = assign23590_e34327_d_n4;
        var_nj0_dn5 = assign23590_e34327_d_n5;
        var_nj0_db0 = assign23590_e34327_d_b0;
        var_nj0_db1 = assign23590_e34327_d_b1;
        var_nj0_db2 = assign23590_e34327_d_b2;
        var_nj0_db3 = assign23590_e34327_d_b3;

        let (assign23600_e34341, assign23600_e34341_d_n0, assign23600_e34341_d_n1, assign23600_e34341_d_n2, assign23600_e34341_d_n3, assign23600_e34341_d_n4, assign23600_e34341_d_n5, assign23600_e34341_d_b0, assign23600_e34341_d_b1, assign23600_e34341_d_b2, assign23600_e34341_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23600_e34337: f64 = (p.p85 - var_nja10);
        let assign23600_e34339: f64 = (assign23600_e34337 - 0.01);
        (assign23600_e34339, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23600_e34341;
        var_tmf1_dn0 = assign23600_e34341_d_n0;
        var_tmf1_dn1 = assign23600_e34341_d_n1;
        var_tmf1_dn2 = assign23600_e34341_d_n2;
        var_tmf1_dn3 = assign23600_e34341_d_n3;
        var_tmf1_dn4 = assign23600_e34341_d_n4;
        var_tmf1_dn5 = assign23600_e34341_d_n5;
        var_tmf1_db0 = assign23600_e34341_d_b0;
        var_tmf1_db1 = assign23600_e34341_d_b1;
        var_tmf1_db2 = assign23600_e34341_d_b2;
        var_tmf1_db3 = assign23600_e34341_d_b3;

        let (assign23610_e34355, assign23610_e34355_d_n0, assign23610_e34355_d_n1, assign23610_e34355_d_n2, assign23610_e34355_d_n3, assign23610_e34355_d_n4, assign23610_e34355_d_n5, assign23610_e34355_d_b0, assign23610_e34355_d_b1, assign23610_e34355_d_b2, assign23610_e34355_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23610_e34351: f64 = (4.0 * p.p85);
        let assign23610_e34353: f64 = (assign23610_e34351 * 0.01);
        (assign23610_e34353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23610_e34355;
        var_tmf2_dn0 = assign23610_e34355_d_n0;
        var_tmf2_dn1 = assign23610_e34355_d_n1;
        var_tmf2_dn2 = assign23610_e34355_d_n2;
        var_tmf2_dn3 = assign23610_e34355_d_n3;
        var_tmf2_dn4 = assign23610_e34355_d_n4;
        var_tmf2_dn5 = assign23610_e34355_d_n5;
        var_tmf2_db0 = assign23610_e34355_d_b0;
        var_tmf2_db1 = assign23610_e34355_d_b1;
        var_tmf2_db2 = assign23610_e34355_d_b2;
        var_tmf2_db3 = assign23610_e34355_d_b3;

        let (assign23620_e34371, assign23620_e34371_d_n0, assign23620_e34371_d_n1, assign23620_e34371_d_n2, assign23620_e34371_d_n3, assign23620_e34371_d_n4, assign23620_e34371_d_n5, assign23620_e34371_d_b0, assign23620_e34371_d_b1, assign23620_e34371_d_b2, assign23620_e34371_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n1, assign23620_e34369_d_n2, assign23620_e34369_d_n3, assign23620_e34369_d_n4, assign23620_e34369_d_n5, assign23620_e34369_d_b0, assign23620_e34369_d_b1, assign23620_e34369_d_b2, assign23620_e34369_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23620_e34368: f64 = (-var_tmf2);
                (assign23620_e34368, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23620_e34369, assign23620_e34369_d_n0, assign23620_e34369_d_n1, assign23620_e34369_d_n2, assign23620_e34369_d_n3, assign23620_e34369_d_n4, assign23620_e34369_d_n5, assign23620_e34369_d_b0, assign23620_e34369_d_b1, assign23620_e34369_d_b2, assign23620_e34369_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23620_e34371;
        var_tmf2_dn0 = assign23620_e34371_d_n0;
        var_tmf2_dn1 = assign23620_e34371_d_n1;
        var_tmf2_dn2 = assign23620_e34371_d_n2;
        var_tmf2_dn3 = assign23620_e34371_d_n3;
        var_tmf2_dn4 = assign23620_e34371_d_n4;
        var_tmf2_dn5 = assign23620_e34371_d_n5;
        var_tmf2_db0 = assign23620_e34371_d_b0;
        var_tmf2_db1 = assign23620_e34371_d_b1;
        var_tmf2_db2 = assign23620_e34371_d_b2;
        var_tmf2_db3 = assign23620_e34371_d_b3;

        let (assign23630_e34386, assign23630_e34386_d_n0, assign23630_e34386_d_n1, assign23630_e34386_d_n2, assign23630_e34386_d_n3, assign23630_e34386_d_n4, assign23630_e34386_d_n5, assign23630_e34386_d_b0, assign23630_e34386_d_b1, assign23630_e34386_d_b2, assign23630_e34386_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23630_e34381: f64 = (var_tmf1 * var_tmf1);
        let assign23630_e34383: f64 = (assign23630_e34381 + var_tmf2);
        let assign23630_e34384: f64 = (assign23630_e34383).sqrt();
        (assign23630_e34384, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign23630_e34384)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign23630_e34384)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign23630_e34384)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign23630_e34384)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign23630_e34384)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign23630_e34384)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23630_e34386;
        var_tmf2_dn0 = assign23630_e34386_d_n0;
        var_tmf2_dn1 = assign23630_e34386_d_n1;
        var_tmf2_dn2 = assign23630_e34386_d_n2;
        var_tmf2_dn3 = assign23630_e34386_d_n3;
        var_tmf2_dn4 = assign23630_e34386_d_n4;
        var_tmf2_dn5 = assign23630_e34386_d_n5;
        var_tmf2_db0 = assign23630_e34386_d_b0;
        var_tmf2_db1 = assign23630_e34386_d_b1;
        var_tmf2_db2 = assign23630_e34386_d_b2;
        var_tmf2_db3 = assign23630_e34386_d_b3;

        let (assign23640_e34402, assign23640_e34402_d_n0, assign23640_e34402_d_n1, assign23640_e34402_d_n2, assign23640_e34402_d_n3, assign23640_e34402_d_n4, assign23640_e34402_d_n5, assign23640_e34402_d_b0, assign23640_e34402_d_b1, assign23640_e34402_d_b2, assign23640_e34402_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23640_e34398: f64 = (var_tmf1 + var_tmf2);
        let assign23640_e34399: f64 = (0.5 * assign23640_e34398);
        let assign23640_e34400: f64 = (p.p85 - assign23640_e34399);
        (assign23640_e34400, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign23640_e34402;
        var_nja11_dn0 = assign23640_e34402_d_n0;
        var_nja11_dn1 = assign23640_e34402_d_n1;
        var_nja11_dn2 = assign23640_e34402_d_n2;
        var_nja11_dn3 = assign23640_e34402_d_n3;
        var_nja11_dn4 = assign23640_e34402_d_n4;
        var_nja11_dn5 = assign23640_e34402_d_n5;
        var_nja11_db0 = assign23640_e34402_d_b0;
        var_nja11_db1 = assign23640_e34402_d_b1;
        var_nja11_db2 = assign23640_e34402_d_b2;
        var_nja11_db3 = assign23640_e34402_d_b3;

        let (assign23650_e34416, assign23650_e34416_d_n0, assign23650_e34416_d_n1, assign23650_e34416_d_n2, assign23650_e34416_d_n3, assign23650_e34416_d_n4, assign23650_e34416_d_n5, assign23650_e34416_d_b0, assign23650_e34416_d_b1, assign23650_e34416_d_b2, assign23650_e34416_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23650_e34412: f64 = (var_nja11 - var_nfabot_i);
        let assign23650_e34414: f64 = (assign23650_e34412 - 0.01);
        (assign23650_e34414, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23650_e34416;
        var_tmf1_dn0 = assign23650_e34416_d_n0;
        var_tmf1_dn1 = assign23650_e34416_d_n1;
        var_tmf1_dn2 = assign23650_e34416_d_n2;
        var_tmf1_dn3 = assign23650_e34416_d_n3;
        var_tmf1_dn4 = assign23650_e34416_d_n4;
        var_tmf1_dn5 = assign23650_e34416_d_n5;
        var_tmf1_db0 = assign23650_e34416_d_b0;
        var_tmf1_db1 = assign23650_e34416_d_b1;
        var_tmf1_db2 = assign23650_e34416_d_b2;
        var_tmf1_db3 = assign23650_e34416_d_b3;

        let (assign23660_e34430, assign23660_e34430_d_n0, assign23660_e34430_d_n1, assign23660_e34430_d_n2, assign23660_e34430_d_n3, assign23660_e34430_d_n4, assign23660_e34430_d_n5, assign23660_e34430_d_b0, assign23660_e34430_d_b1, assign23660_e34430_d_b2, assign23660_e34430_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23660_e34426: f64 = (4.0 * var_nfabot_i);
        let assign23660_e34428: f64 = (assign23660_e34426 * 0.01);
        (assign23660_e34428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23660_e34430;
        var_tmf2_dn0 = assign23660_e34430_d_n0;
        var_tmf2_dn1 = assign23660_e34430_d_n1;
        var_tmf2_dn2 = assign23660_e34430_d_n2;
        var_tmf2_dn3 = assign23660_e34430_d_n3;
        var_tmf2_dn4 = assign23660_e34430_d_n4;
        var_tmf2_dn5 = assign23660_e34430_d_n5;
        var_tmf2_db0 = assign23660_e34430_d_b0;
        var_tmf2_db1 = assign23660_e34430_d_b1;
        var_tmf2_db2 = assign23660_e34430_d_b2;
        var_tmf2_db3 = assign23660_e34430_d_b3;

        let (assign23670_e34446, assign23670_e34446_d_n0, assign23670_e34446_d_n1, assign23670_e34446_d_n2, assign23670_e34446_d_n3, assign23670_e34446_d_n4, assign23670_e34446_d_n5, assign23670_e34446_d_b0, assign23670_e34446_d_b1, assign23670_e34446_d_b2, assign23670_e34446_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n1, assign23670_e34444_d_n2, assign23670_e34444_d_n3, assign23670_e34444_d_n4, assign23670_e34444_d_n5, assign23670_e34444_d_b0, assign23670_e34444_d_b1, assign23670_e34444_d_b2, assign23670_e34444_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23670_e34443: f64 = (-var_tmf2);
                (assign23670_e34443, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23670_e34444, assign23670_e34444_d_n0, assign23670_e34444_d_n1, assign23670_e34444_d_n2, assign23670_e34444_d_n3, assign23670_e34444_d_n4, assign23670_e34444_d_n5, assign23670_e34444_d_b0, assign23670_e34444_d_b1, assign23670_e34444_d_b2, assign23670_e34444_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23670_e34446;
        var_tmf2_dn0 = assign23670_e34446_d_n0;
        var_tmf2_dn1 = assign23670_e34446_d_n1;
        var_tmf2_dn2 = assign23670_e34446_d_n2;
        var_tmf2_dn3 = assign23670_e34446_d_n3;
        var_tmf2_dn4 = assign23670_e34446_d_n4;
        var_tmf2_dn5 = assign23670_e34446_d_n5;
        var_tmf2_db0 = assign23670_e34446_d_b0;
        var_tmf2_db1 = assign23670_e34446_d_b1;
        var_tmf2_db2 = assign23670_e34446_d_b2;
        var_tmf2_db3 = assign23670_e34446_d_b3;

        let (assign23680_e34461, assign23680_e34461_d_n0, assign23680_e34461_d_n1, assign23680_e34461_d_n2, assign23680_e34461_d_n3, assign23680_e34461_d_n4, assign23680_e34461_d_n5, assign23680_e34461_d_b0, assign23680_e34461_d_b1, assign23680_e34461_d_b2, assign23680_e34461_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23680_e34456: f64 = (var_tmf1 * var_tmf1);
        let assign23680_e34458: f64 = (assign23680_e34456 + var_tmf2);
        let assign23680_e34459: f64 = (assign23680_e34458).sqrt();
        (assign23680_e34459, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign23680_e34459)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign23680_e34459)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign23680_e34459)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign23680_e34459)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign23680_e34459)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign23680_e34459)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23680_e34461;
        var_tmf2_dn0 = assign23680_e34461_d_n0;
        var_tmf2_dn1 = assign23680_e34461_d_n1;
        var_tmf2_dn2 = assign23680_e34461_d_n2;
        var_tmf2_dn3 = assign23680_e34461_d_n3;
        var_tmf2_dn4 = assign23680_e34461_d_n4;
        var_tmf2_dn5 = assign23680_e34461_d_n5;
        var_tmf2_db0 = assign23680_e34461_d_b0;
        var_tmf2_db1 = assign23680_e34461_d_b1;
        var_tmf2_db2 = assign23680_e34461_d_b2;
        var_tmf2_db3 = assign23680_e34461_d_b3;

        let (assign23690_e34477, assign23690_e34477_d_n0, assign23690_e34477_d_n1, assign23690_e34477_d_n2, assign23690_e34477_d_n3, assign23690_e34477_d_n4, assign23690_e34477_d_n5, assign23690_e34477_d_b0, assign23690_e34477_d_b1, assign23690_e34477_d_b2, assign23690_e34477_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23690_e34473: f64 = (var_tmf1 + var_tmf2);
        let assign23690_e34474: f64 = (0.5 * assign23690_e34473);
        let assign23690_e34475: f64 = (var_nfabot_i + assign23690_e34474);
        (assign23690_e34475, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign23690_e34477;
        var_nj1_dn0 = assign23690_e34477_d_n0;
        var_nj1_dn1 = assign23690_e34477_d_n1;
        var_nj1_dn2 = assign23690_e34477_d_n2;
        var_nj1_dn3 = assign23690_e34477_d_n3;
        var_nj1_dn4 = assign23690_e34477_d_n4;
        var_nj1_dn5 = assign23690_e34477_d_n5;
        var_nj1_db0 = assign23690_e34477_d_b0;
        var_nj1_db1 = assign23690_e34477_d_b1;
        var_nj1_db2 = assign23690_e34477_d_b2;
        var_nj1_db3 = assign23690_e34477_d_b3;

        let (assign23700_e34491, assign23700_e34491_d_n0, assign23700_e34491_d_n1, assign23700_e34491_d_n2, assign23700_e34491_d_n3, assign23700_e34491_d_n4, assign23700_e34491_d_n5, assign23700_e34491_d_b0, assign23700_e34491_d_b1, assign23700_e34491_d_b2, assign23700_e34491_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23700_e34487: f64 = (p.p85 - var_nj0);
        let assign23700_e34489: f64 = (assign23700_e34487 - 0.01);
        (assign23700_e34489, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23700_e34491;
        var_tmf1_dn0 = assign23700_e34491_d_n0;
        var_tmf1_dn1 = assign23700_e34491_d_n1;
        var_tmf1_dn2 = assign23700_e34491_d_n2;
        var_tmf1_dn3 = assign23700_e34491_d_n3;
        var_tmf1_dn4 = assign23700_e34491_d_n4;
        var_tmf1_dn5 = assign23700_e34491_d_n5;
        var_tmf1_db0 = assign23700_e34491_d_b0;
        var_tmf1_db1 = assign23700_e34491_d_b1;
        var_tmf1_db2 = assign23700_e34491_d_b2;
        var_tmf1_db3 = assign23700_e34491_d_b3;

        let (assign23710_e34505, assign23710_e34505_d_n0, assign23710_e34505_d_n1, assign23710_e34505_d_n2, assign23710_e34505_d_n3, assign23710_e34505_d_n4, assign23710_e34505_d_n5, assign23710_e34505_d_b0, assign23710_e34505_d_b1, assign23710_e34505_d_b2, assign23710_e34505_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23710_e34501: f64 = (4.0 * p.p85);
        let assign23710_e34503: f64 = (assign23710_e34501 * 0.01);
        (assign23710_e34503, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23710_e34505;
        var_tmf2_dn0 = assign23710_e34505_d_n0;
        var_tmf2_dn1 = assign23710_e34505_d_n1;
        var_tmf2_dn2 = assign23710_e34505_d_n2;
        var_tmf2_dn3 = assign23710_e34505_d_n3;
        var_tmf2_dn4 = assign23710_e34505_d_n4;
        var_tmf2_dn5 = assign23710_e34505_d_n5;
        var_tmf2_db0 = assign23710_e34505_d_b0;
        var_tmf2_db1 = assign23710_e34505_d_b1;
        var_tmf2_db2 = assign23710_e34505_d_b2;
        var_tmf2_db3 = assign23710_e34505_d_b3;


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
        *var_guard384_slot = var_guard384;
        *var_guard385_slot = var_guard385;
        *var_guard388_slot = var_guard388;
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

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard388: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_guard389_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard391_slot: &mut f64,
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
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
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

        let (assign23720_e34521, assign23720_e34521_d_n0, assign23720_e34521_d_n1, assign23720_e34521_d_n2, assign23720_e34521_d_n3, assign23720_e34521_d_n4, assign23720_e34521_d_n5, assign23720_e34521_d_b0, assign23720_e34521_d_b1, assign23720_e34521_d_b2, assign23720_e34521_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n1, assign23720_e34519_d_n2, assign23720_e34519_d_n3, assign23720_e34519_d_n4, assign23720_e34519_d_n5, assign23720_e34519_d_b0, assign23720_e34519_d_b1, assign23720_e34519_d_b2, assign23720_e34519_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23720_e34518: f64 = (-var_tmf2);
                (assign23720_e34518, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23720_e34519, assign23720_e34519_d_n0, assign23720_e34519_d_n1, assign23720_e34519_d_n2, assign23720_e34519_d_n3, assign23720_e34519_d_n4, assign23720_e34519_d_n5, assign23720_e34519_d_b0, assign23720_e34519_d_b1, assign23720_e34519_d_b2, assign23720_e34519_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23720_e34521;
        var_tmf2_dn0 = assign23720_e34521_d_n0;
        var_tmf2_dn1 = assign23720_e34521_d_n1;
        var_tmf2_dn2 = assign23720_e34521_d_n2;
        var_tmf2_dn3 = assign23720_e34521_d_n3;
        var_tmf2_dn4 = assign23720_e34521_d_n4;
        var_tmf2_dn5 = assign23720_e34521_d_n5;
        var_tmf2_db0 = assign23720_e34521_d_b0;
        var_tmf2_db1 = assign23720_e34521_d_b1;
        var_tmf2_db2 = assign23720_e34521_d_b2;
        var_tmf2_db3 = assign23720_e34521_d_b3;

        let (assign23730_e34536, assign23730_e34536_d_n0, assign23730_e34536_d_n1, assign23730_e34536_d_n2, assign23730_e34536_d_n3, assign23730_e34536_d_n4, assign23730_e34536_d_n5, assign23730_e34536_d_b0, assign23730_e34536_d_b1, assign23730_e34536_d_b2, assign23730_e34536_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23730_e34531: f64 = (var_tmf1 * var_tmf1);
        let assign23730_e34533: f64 = (assign23730_e34531 + var_tmf2);
        let assign23730_e34534: f64 = (assign23730_e34533).sqrt();
        (assign23730_e34534, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign23730_e34534)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign23730_e34534)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign23730_e34534)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign23730_e34534)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign23730_e34534)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign23730_e34534)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23730_e34536;
        var_tmf2_dn0 = assign23730_e34536_d_n0;
        var_tmf2_dn1 = assign23730_e34536_d_n1;
        var_tmf2_dn2 = assign23730_e34536_d_n2;
        var_tmf2_dn3 = assign23730_e34536_d_n3;
        var_tmf2_dn4 = assign23730_e34536_d_n4;
        var_tmf2_dn5 = assign23730_e34536_d_n5;
        var_tmf2_db0 = assign23730_e34536_d_b0;
        var_tmf2_db1 = assign23730_e34536_d_b1;
        var_tmf2_db2 = assign23730_e34536_d_b2;
        var_tmf2_db3 = assign23730_e34536_d_b3;

        let (assign23740_e34552, assign23740_e34552_d_n0, assign23740_e34552_d_n1, assign23740_e34552_d_n2, assign23740_e34552_d_n3, assign23740_e34552_d_n4, assign23740_e34552_d_n5, assign23740_e34552_d_b0, assign23740_e34552_d_b1, assign23740_e34552_d_b2, assign23740_e34552_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23740_e34548: f64 = (var_tmf1 + var_tmf2);
        let assign23740_e34549: f64 = (0.5 * assign23740_e34548);
        let assign23740_e34550: f64 = (p.p85 - assign23740_e34549);
        (assign23740_e34550, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign23740_e34552;
        var_nj0_dn0 = assign23740_e34552_d_n0;
        var_nj0_dn1 = assign23740_e34552_d_n1;
        var_nj0_dn2 = assign23740_e34552_d_n2;
        var_nj0_dn3 = assign23740_e34552_d_n3;
        var_nj0_dn4 = assign23740_e34552_d_n4;
        var_nj0_dn5 = assign23740_e34552_d_n5;
        var_nj0_db0 = assign23740_e34552_d_b0;
        var_nj0_db1 = assign23740_e34552_d_b1;
        var_nj0_db2 = assign23740_e34552_d_b2;
        var_nj0_db3 = assign23740_e34552_d_b3;

        let (assign23750_e34566, assign23750_e34566_d_n0, assign23750_e34566_d_n1, assign23750_e34566_d_n2, assign23750_e34566_d_n3, assign23750_e34566_d_n4, assign23750_e34566_d_n5, assign23750_e34566_d_b0, assign23750_e34566_d_b1, assign23750_e34566_d_b2, assign23750_e34566_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23750_e34562: f64 = (var_nj0 - var_nfabot_i);
        let assign23750_e34564: f64 = (assign23750_e34562 - 0.01);
        (assign23750_e34564, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23750_e34566;
        var_tmf1_dn0 = assign23750_e34566_d_n0;
        var_tmf1_dn1 = assign23750_e34566_d_n1;
        var_tmf1_dn2 = assign23750_e34566_d_n2;
        var_tmf1_dn3 = assign23750_e34566_d_n3;
        var_tmf1_dn4 = assign23750_e34566_d_n4;
        var_tmf1_dn5 = assign23750_e34566_d_n5;
        var_tmf1_db0 = assign23750_e34566_d_b0;
        var_tmf1_db1 = assign23750_e34566_d_b1;
        var_tmf1_db2 = assign23750_e34566_d_b2;
        var_tmf1_db3 = assign23750_e34566_d_b3;

        let (assign23760_e34580, assign23760_e34580_d_n0, assign23760_e34580_d_n1, assign23760_e34580_d_n2, assign23760_e34580_d_n3, assign23760_e34580_d_n4, assign23760_e34580_d_n5, assign23760_e34580_d_b0, assign23760_e34580_d_b1, assign23760_e34580_d_b2, assign23760_e34580_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23760_e34576: f64 = (4.0 * var_nfabot_i);
        let assign23760_e34578: f64 = (assign23760_e34576 * 0.01);
        (assign23760_e34578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23760_e34580;
        var_tmf2_dn0 = assign23760_e34580_d_n0;
        var_tmf2_dn1 = assign23760_e34580_d_n1;
        var_tmf2_dn2 = assign23760_e34580_d_n2;
        var_tmf2_dn3 = assign23760_e34580_d_n3;
        var_tmf2_dn4 = assign23760_e34580_d_n4;
        var_tmf2_dn5 = assign23760_e34580_d_n5;
        var_tmf2_db0 = assign23760_e34580_d_b0;
        var_tmf2_db1 = assign23760_e34580_d_b1;
        var_tmf2_db2 = assign23760_e34580_d_b2;
        var_tmf2_db3 = assign23760_e34580_d_b3;

        let (assign23770_e34596, assign23770_e34596_d_n0, assign23770_e34596_d_n1, assign23770_e34596_d_n2, assign23770_e34596_d_n3, assign23770_e34596_d_n4, assign23770_e34596_d_n5, assign23770_e34596_d_b0, assign23770_e34596_d_b1, assign23770_e34596_d_b2, assign23770_e34596_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n1, assign23770_e34594_d_n2, assign23770_e34594_d_n3, assign23770_e34594_d_n4, assign23770_e34594_d_n5, assign23770_e34594_d_b0, assign23770_e34594_d_b1, assign23770_e34594_d_b2, assign23770_e34594_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23770_e34593: f64 = (-var_tmf2);
                (assign23770_e34593, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23770_e34594, assign23770_e34594_d_n0, assign23770_e34594_d_n1, assign23770_e34594_d_n2, assign23770_e34594_d_n3, assign23770_e34594_d_n4, assign23770_e34594_d_n5, assign23770_e34594_d_b0, assign23770_e34594_d_b1, assign23770_e34594_d_b2, assign23770_e34594_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23770_e34596;
        var_tmf2_dn0 = assign23770_e34596_d_n0;
        var_tmf2_dn1 = assign23770_e34596_d_n1;
        var_tmf2_dn2 = assign23770_e34596_d_n2;
        var_tmf2_dn3 = assign23770_e34596_d_n3;
        var_tmf2_dn4 = assign23770_e34596_d_n4;
        var_tmf2_dn5 = assign23770_e34596_d_n5;
        var_tmf2_db0 = assign23770_e34596_d_b0;
        var_tmf2_db1 = assign23770_e34596_d_b1;
        var_tmf2_db2 = assign23770_e34596_d_b2;
        var_tmf2_db3 = assign23770_e34596_d_b3;

        let (assign23780_e34611, assign23780_e34611_d_n0, assign23780_e34611_d_n1, assign23780_e34611_d_n2, assign23780_e34611_d_n3, assign23780_e34611_d_n4, assign23780_e34611_d_n5, assign23780_e34611_d_b0, assign23780_e34611_d_b1, assign23780_e34611_d_b2, assign23780_e34611_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23780_e34606: f64 = (var_tmf1 * var_tmf1);
        let assign23780_e34608: f64 = (assign23780_e34606 + var_tmf2);
        let assign23780_e34609: f64 = (assign23780_e34608).sqrt();
        (assign23780_e34609, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign23780_e34609)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign23780_e34609)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign23780_e34609)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign23780_e34609)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign23780_e34609)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign23780_e34609)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23780_e34611;
        var_tmf2_dn0 = assign23780_e34611_d_n0;
        var_tmf2_dn1 = assign23780_e34611_d_n1;
        var_tmf2_dn2 = assign23780_e34611_d_n2;
        var_tmf2_dn3 = assign23780_e34611_d_n3;
        var_tmf2_dn4 = assign23780_e34611_d_n4;
        var_tmf2_dn5 = assign23780_e34611_d_n5;
        var_tmf2_db0 = assign23780_e34611_d_b0;
        var_tmf2_db1 = assign23780_e34611_d_b1;
        var_tmf2_db2 = assign23780_e34611_d_b2;
        var_tmf2_db3 = assign23780_e34611_d_b3;

        let (assign23790_e34627, assign23790_e34627_d_n0, assign23790_e34627_d_n1, assign23790_e34627_d_n2, assign23790_e34627_d_n3, assign23790_e34627_d_n4, assign23790_e34627_d_n5, assign23790_e34627_d_b0, assign23790_e34627_d_b1, assign23790_e34627_d_b2, assign23790_e34627_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 != 0.0)) {
        let assign23790_e34623: f64 = (var_tmf1 + var_tmf2);
        let assign23790_e34624: f64 = (0.5 * assign23790_e34623);
        let assign23790_e34625: f64 = (var_nfabot_i + assign23790_e34624);
        (assign23790_e34625, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign23790_e34627;
        var_nj0_dn0 = assign23790_e34627_d_n0;
        var_nj0_dn1 = assign23790_e34627_d_n1;
        var_nj0_dn2 = assign23790_e34627_d_n2;
        var_nj0_dn3 = assign23790_e34627_d_n3;
        var_nj0_dn4 = assign23790_e34627_d_n4;
        var_nj0_dn5 = assign23790_e34627_d_n5;
        var_nj0_db0 = assign23790_e34627_d_b0;
        var_nj0_db1 = assign23790_e34627_d_b1;
        var_nj0_db2 = assign23790_e34627_d_b2;
        var_nj0_db3 = assign23790_e34627_d_b3;

        let (assign23800_e34638, assign23800_e34638_d_n0, assign23800_e34638_d_n1, assign23800_e34638_d_n2, assign23800_e34638_d_n3, assign23800_e34638_d_n4, assign23800_e34638_d_n5, assign23800_e34638_d_b0, assign23800_e34638_d_b1, assign23800_e34638_d_b2, assign23800_e34638_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign23800_e34638;
        var_nj0_dn0 = assign23800_e34638_d_n0;
        var_nj0_dn1 = assign23800_e34638_d_n1;
        var_nj0_dn2 = assign23800_e34638_d_n2;
        var_nj0_dn3 = assign23800_e34638_d_n3;
        var_nj0_dn4 = assign23800_e34638_d_n4;
        var_nj0_dn5 = assign23800_e34638_d_n5;
        var_nj0_db0 = assign23800_e34638_d_b0;
        var_nj0_db1 = assign23800_e34638_d_b1;
        var_nj0_db2 = assign23800_e34638_d_b2;
        var_nj0_db3 = assign23800_e34638_d_b3;

        let (assign23810_e34649, assign23810_e34649_d_n0, assign23810_e34649_d_n1, assign23810_e34649_d_n2, assign23810_e34649_d_n3, assign23810_e34649_d_n4, assign23810_e34649_d_n5, assign23810_e34649_d_b0, assign23810_e34649_d_b1, assign23810_e34649_d_b2, assign23810_e34649_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard388 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign23810_e34649;
        var_nj1_dn0 = assign23810_e34649_d_n0;
        var_nj1_dn1 = assign23810_e34649_d_n1;
        var_nj1_dn2 = assign23810_e34649_d_n2;
        var_nj1_dn3 = assign23810_e34649_d_n3;
        var_nj1_dn4 = assign23810_e34649_d_n4;
        var_nj1_dn5 = assign23810_e34649_d_n5;
        var_nj1_db0 = assign23810_e34649_d_b0;
        var_nj1_db1 = assign23810_e34649_d_b1;
        var_nj1_db2 = assign23810_e34649_d_b2;
        var_nj1_db3 = assign23810_e34649_d_b3;

        let assign23820_e34653: f64 = (var_v5 / var_nj1);
        let assign23820_e34657: f64 = (var_nj1 - var_nj0);
        let assign23820_e34658: f64 = (var_vha1 * assign23820_e34657);
        let assign23820_e34661: f64 = (var_nj0 * p.p85);
        let assign23820_e34662: f64 = (assign23820_e34658 / assign23820_e34661);
        let assign23820_e34663: f64 = (assign23820_e34653 + assign23820_e34662);
        let assign23820_e34664: f64 = (var_phitdinv * assign23820_e34663);
        let assign23820_e34665: f64 = (assign23820_e34664).abs();
        let assign23820_e34667: f64 = if assign23820_e34665 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard389 = assign23820_e34667;

        let (assign23830_e34692, assign23830_e34692_d_n0, assign23830_e34692_d_n1, assign23830_e34692_d_n2, assign23830_e34692_d_n3, assign23830_e34692_d_n4, assign23830_e34692_d_n5, assign23830_e34692_d_b0, assign23830_e34692_d_b1, assign23830_e34692_d_b2, assign23830_e34692_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 != 0.0)) {
        let assign23830_e34678: f64 = (var_v5 / var_nj1);
        let assign23830_e34682: f64 = (var_nj1 - var_nj0);
        let assign23830_e34683: f64 = (var_vha1 * assign23830_e34682);
        let assign23830_e34686: f64 = (var_nj0 * p.p85);
        let assign23830_e34687: f64 = (assign23830_e34683 / assign23830_e34686);
        let assign23830_e34688: f64 = (assign23830_e34678 + assign23830_e34687);
        let assign23830_e34689: f64 = (var_phitdinv * assign23830_e34688);
        let assign23830_e34690: f64 = (assign23830_e34689).exp();
        (assign23830_e34690, (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn0 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn1 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn2 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn3 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn4 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_dn5 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_db0 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_db1 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_db2 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))), (assign23830_e34690 * (var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23830_e34686) - (assign23830_e34683 * (var_nj0_db3 * p.p85))) / (assign23830_e34686 * assign23830_e34686))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign23830_e34692;
        var_idmultbot_dn0 = assign23830_e34692_d_n0;
        var_idmultbot_dn1 = assign23830_e34692_d_n1;
        var_idmultbot_dn2 = assign23830_e34692_d_n2;
        var_idmultbot_dn3 = assign23830_e34692_d_n3;
        var_idmultbot_dn4 = assign23830_e34692_d_n4;
        var_idmultbot_dn5 = assign23830_e34692_d_n5;
        var_idmultbot_db0 = assign23830_e34692_d_b0;
        var_idmultbot_db1 = assign23830_e34692_d_b1;
        var_idmultbot_db2 = assign23830_e34692_d_b2;
        var_idmultbot_db3 = assign23830_e34692_d_b3;

        let assign23840_e34696: f64 = (var_v5 / var_nj1);
        let assign23840_e34700: f64 = (var_nj1 - var_nj0);
        let assign23840_e34701: f64 = (var_vha1 * assign23840_e34700);
        let assign23840_e34704: f64 = (var_nj0 * p.p85);
        let assign23840_e34705: f64 = (assign23840_e34701 / assign23840_e34704);
        let assign23840_e34706: f64 = (assign23840_e34696 + assign23840_e34705);
        let assign23840_e34707: f64 = (var_phitdinv * assign23840_e34706);
        let assign23840_e34709: f64 = (-230.25850929940458);
        let assign23840_e34710: f64 = if assign23840_e34707 < assign23840_e34709 { 1.0 } else { 0.0 };
        var_guard390 = assign23840_e34710;

        let (assign23850_e34790, assign23850_e34790_d_n0, assign23850_e34790_d_n1, assign23850_e34790_d_n2, assign23850_e34790_d_n3, assign23850_e34790_d_n4, assign23850_e34790_d_n5, assign23850_e34790_d_b0, assign23850_e34790_d_b1, assign23850_e34790_d_b2, assign23850_e34790_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 == 0.0)) && (var_guard390 != 0.0)) {
        let assign23850_e34724: f64 = (-230.25850929940458);
        let assign23850_e34728: f64 = (var_v5 / var_nj1);
        let assign23850_e34732: f64 = (var_nj1 - var_nj0);
        let assign23850_e34733: f64 = (var_vha1 * assign23850_e34732);
        let assign23850_e34736: f64 = (var_nj0 * p.p85);
        let assign23850_e34737: f64 = (assign23850_e34733 / assign23850_e34736);
        let assign23850_e34738: f64 = (assign23850_e34728 + assign23850_e34737);
        let assign23850_e34739: f64 = (var_phitdinv * assign23850_e34738);
        let assign23850_e34740: f64 = (assign23850_e34724 - assign23850_e34739);
        let assign23850_e34744: f64 = (-230.25850929940458);
        let assign23850_e34748: f64 = (var_v5 / var_nj1);
        let assign23850_e34752: f64 = (var_nj1 - var_nj0);
        let assign23850_e34753: f64 = (var_vha1 * assign23850_e34752);
        let assign23850_e34756: f64 = (var_nj0 * p.p85);
        let assign23850_e34757: f64 = (assign23850_e34753 / assign23850_e34756);
        let assign23850_e34758: f64 = (assign23850_e34748 + assign23850_e34757);
        let assign23850_e34759: f64 = (var_phitdinv * assign23850_e34758);
        let assign23850_e34760: f64 = (assign23850_e34744 - assign23850_e34759);
        let assign23850_e34763: f64 = (-230.25850929940458);
        let assign23850_e34767: f64 = (var_v5 / var_nj1);
        let assign23850_e34771: f64 = (var_nj1 - var_nj0);
        let assign23850_e34772: f64 = (var_vha1 * assign23850_e34771);
        let assign23850_e34775: f64 = (var_nj0 * p.p85);
        let assign23850_e34776: f64 = (assign23850_e34772 / assign23850_e34775);
        let assign23850_e34777: f64 = (assign23850_e34767 + assign23850_e34776);
        let assign23850_e34778: f64 = (var_phitdinv * assign23850_e34777);
        let assign23850_e34779: f64 = (assign23850_e34763 - assign23850_e34778);
        let assign23850_e34781: f64 = (assign23850_e34779 * 0.3333333333333333);
        let assign23850_e34782: f64 = (1.0 + assign23850_e34781);
        let assign23850_e34783: f64 = (assign23850_e34760 * assign23850_e34782);
        let assign23850_e34784: f64 = (0.5 * assign23850_e34783);
        let assign23850_e34785: f64 = (1.0 + assign23850_e34784);
        let assign23850_e34786: f64 = (assign23850_e34740 * assign23850_e34785);
        let assign23850_e34787: f64 = (1.0 + assign23850_e34786);
        let assign23850_e34788: f64 = (1e-100 / assign23850_e34787);
        (assign23850_e34788, (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn0 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn0 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn0 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn1 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn1 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn1 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn2 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn2 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn2 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn3 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn3 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn3 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn4 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn4 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn4 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_dn5 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_dn5 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_dn5 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_db0 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_db0 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_db0 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_db1 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_db1 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_db1 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_db2 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_db2 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_db2 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23850_e34736) - (assign23850_e34733 * (var_nj0_db3 * p.p85))) / (assign23850_e34736 * assign23850_e34736))))) * assign23850_e34785) + (assign23850_e34740 * (0.5 * (((-(var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23850_e34756) - (assign23850_e34753 * (var_nj0_db3 * p.p85))) / (assign23850_e34756 * assign23850_e34756))))) * assign23850_e34782) + (assign23850_e34760 * ((-(var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23850_e34775) - (assign23850_e34772 * (var_nj0_db3 * p.p85))) / (assign23850_e34775 * assign23850_e34775))))) * 0.3333333333333333))))))) / (assign23850_e34787 * assign23850_e34787))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign23850_e34790;
        var_idmultbot_dn0 = assign23850_e34790_d_n0;
        var_idmultbot_dn1 = assign23850_e34790_d_n1;
        var_idmultbot_dn2 = assign23850_e34790_d_n2;
        var_idmultbot_dn3 = assign23850_e34790_d_n3;
        var_idmultbot_dn4 = assign23850_e34790_d_n4;
        var_idmultbot_dn5 = assign23850_e34790_d_n5;
        var_idmultbot_db0 = assign23850_e34790_d_b0;
        var_idmultbot_db1 = assign23850_e34790_d_b1;
        var_idmultbot_db2 = assign23850_e34790_d_b2;
        var_idmultbot_db3 = assign23850_e34790_d_b3;

        let (assign23860_e34868, assign23860_e34868_d_n0, assign23860_e34868_d_n1, assign23860_e34868_d_n2, assign23860_e34868_d_n3, assign23860_e34868_d_n4, assign23860_e34868_d_n5, assign23860_e34868_d_b0, assign23860_e34868_d_b1, assign23860_e34868_d_b2, assign23860_e34868_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard389 == 0.0)) && (var_guard390 == 0.0)) {
        let assign23860_e34807: f64 = (var_v5 / var_nj1);
        let assign23860_e34811: f64 = (var_nj1 - var_nj0);
        let assign23860_e34812: f64 = (var_vha1 * assign23860_e34811);
        let assign23860_e34815: f64 = (var_nj0 * p.p85);
        let assign23860_e34816: f64 = (assign23860_e34812 / assign23860_e34815);
        let assign23860_e34817: f64 = (assign23860_e34807 + assign23860_e34816);
        let assign23860_e34818: f64 = (var_phitdinv * assign23860_e34817);
        let assign23860_e34820: f64 = (assign23860_e34818 - 230.25850929940458);
        let assign23860_e34826: f64 = (var_v5 / var_nj1);
        let assign23860_e34830: f64 = (var_nj1 - var_nj0);
        let assign23860_e34831: f64 = (var_vha1 * assign23860_e34830);
        let assign23860_e34834: f64 = (var_nj0 * p.p85);
        let assign23860_e34835: f64 = (assign23860_e34831 / assign23860_e34834);
        let assign23860_e34836: f64 = (assign23860_e34826 + assign23860_e34835);
        let assign23860_e34837: f64 = (var_phitdinv * assign23860_e34836);
        let assign23860_e34839: f64 = (assign23860_e34837 - 230.25850929940458);
        let assign23860_e34844: f64 = (var_v5 / var_nj1);
        let assign23860_e34848: f64 = (var_nj1 - var_nj0);
        let assign23860_e34849: f64 = (var_vha1 * assign23860_e34848);
        let assign23860_e34852: f64 = (var_nj0 * p.p85);
        let assign23860_e34853: f64 = (assign23860_e34849 / assign23860_e34852);
        let assign23860_e34854: f64 = (assign23860_e34844 + assign23860_e34853);
        let assign23860_e34855: f64 = (var_phitdinv * assign23860_e34854);
        let assign23860_e34857: f64 = (assign23860_e34855 - 230.25850929940458);
        let assign23860_e34859: f64 = (assign23860_e34857 * 0.3333333333333333);
        let assign23860_e34860: f64 = (1.0 + assign23860_e34859);
        let assign23860_e34861: f64 = (assign23860_e34839 * assign23860_e34860);
        let assign23860_e34862: f64 = (0.5 * assign23860_e34861);
        let assign23860_e34863: f64 = (1.0 + assign23860_e34862);
        let assign23860_e34864: f64 = (assign23860_e34820 * assign23860_e34863);
        let assign23860_e34865: f64 = (1.0 + assign23860_e34864);
        let assign23860_e34866: f64 = (1e100 * assign23860_e34865);
        (assign23860_e34866, (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn0 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn0 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn0 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn1 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn1 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn1 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn2 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn2 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn2 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn3 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn3 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn3 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn4 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn4 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn4 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_dn5 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_dn5 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_dn5 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_db0 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_db0 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_db0 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_db1 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_db1 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_db1 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_db2 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_db2 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_db2 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23860_e34815) - (assign23860_e34812 * (var_nj0_db3 * p.p85))) / (assign23860_e34815 * assign23860_e34815)))) * assign23860_e34863) + (assign23860_e34820 * (0.5 * (((var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23860_e34834) - (assign23860_e34831 * (var_nj0_db3 * p.p85))) / (assign23860_e34834 * assign23860_e34834)))) * assign23860_e34860) + (assign23860_e34839 * ((var_phitdinv * ((-((var_v5 * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign23860_e34852) - (assign23860_e34849 * (var_nj0_db3 * p.p85))) / (assign23860_e34852 * assign23860_e34852)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign23860_e34868;
        var_idmultbot_dn0 = assign23860_e34868_d_n0;
        var_idmultbot_dn1 = assign23860_e34868_d_n1;
        var_idmultbot_dn2 = assign23860_e34868_d_n2;
        var_idmultbot_dn3 = assign23860_e34868_d_n3;
        var_idmultbot_dn4 = assign23860_e34868_d_n4;
        var_idmultbot_dn5 = assign23860_e34868_d_n5;
        var_idmultbot_db0 = assign23860_e34868_d_b0;
        var_idmultbot_db1 = assign23860_e34868_d_b1;
        var_idmultbot_db2 = assign23860_e34868_d_b2;
        var_idmultbot_db3 = assign23860_e34868_d_b3;

        let (assign23870_e34880,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23870_e34876: f64 = (var_nin * var_nin);
        let assign23870_e34878: f64 = (assign23870_e34876 / var_ndisti_i);
        (assign23870_e34878,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign23870_e34880;

        let (assign23880_e34895,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign23880_e34888: f64 = (var_nfasti_i / var_phitdinv);
        let assign23880_e34891: f64 = (var_ndisti_i / var_pnn0);
        let assign23880_e34892: f64 = (assign23880_e34891).ln();
        let assign23880_e34893: f64 = (assign23880_e34888 * assign23880_e34892);
        (assign23880_e34893,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign23880_e34895;

        let assign23890_e34898: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard391 = assign23890_e34898;

        let (assign23900_e34914, assign23900_e34914_d_n0, assign23900_e34914_d_n1, assign23900_e34914_d_n2, assign23900_e34914_d_n3, assign23900_e34914_d_n4, assign23900_e34914_d_n5, assign23900_e34914_d_b0, assign23900_e34914_d_b1, assign23900_e34914_d_b2, assign23900_e34914_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23900_e34909: f64 = (var_v5 - var_vha1);
        let assign23900_e34910: f64 = (p.p86 * assign23900_e34909);
        let assign23900_e34912: f64 = (assign23900_e34910 + var_nfasti_i);
        (assign23900_e34912, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign23900_e34914;
        var_nja10_dn0 = assign23900_e34914_d_n0;
        var_nja10_dn1 = assign23900_e34914_d_n1;
        var_nja10_dn2 = assign23900_e34914_d_n2;
        var_nja10_dn3 = assign23900_e34914_d_n3;
        var_nja10_dn4 = assign23900_e34914_d_n4;
        var_nja10_dn5 = assign23900_e34914_d_n5;
        var_nja10_db0 = assign23900_e34914_d_b0;
        var_nja10_db1 = assign23900_e34914_d_b1;
        var_nja10_db2 = assign23900_e34914_d_b2;
        var_nja10_db3 = assign23900_e34914_d_b3;

        let (assign23910_e34928, assign23910_e34928_d_n0, assign23910_e34928_d_n1, assign23910_e34928_d_n2, assign23910_e34928_d_n3, assign23910_e34928_d_n4, assign23910_e34928_d_n5, assign23910_e34928_d_b0, assign23910_e34928_d_b1, assign23910_e34928_d_b2, assign23910_e34928_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23910_e34925: f64 = (p.p86 * var_vha1);
        let assign23910_e34926: f64 = (var_nfasti_i - assign23910_e34925);
        (assign23910_e34926, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign23910_e34928;
        var_nj0_dn0 = assign23910_e34928_d_n0;
        var_nj0_dn1 = assign23910_e34928_d_n1;
        var_nj0_dn2 = assign23910_e34928_d_n2;
        var_nj0_dn3 = assign23910_e34928_d_n3;
        var_nj0_dn4 = assign23910_e34928_d_n4;
        var_nj0_dn5 = assign23910_e34928_d_n5;
        var_nj0_db0 = assign23910_e34928_d_b0;
        var_nj0_db1 = assign23910_e34928_d_b1;
        var_nj0_db2 = assign23910_e34928_d_b2;
        var_nj0_db3 = assign23910_e34928_d_b3;

        let (assign23920_e34942, assign23920_e34942_d_n0, assign23920_e34942_d_n1, assign23920_e34942_d_n2, assign23920_e34942_d_n3, assign23920_e34942_d_n4, assign23920_e34942_d_n5, assign23920_e34942_d_b0, assign23920_e34942_d_b1, assign23920_e34942_d_b2, assign23920_e34942_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23920_e34938: f64 = (p.p85 - var_nja10);
        let assign23920_e34940: f64 = (assign23920_e34938 - 0.01);
        (assign23920_e34940, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23920_e34942;
        var_tmf1_dn0 = assign23920_e34942_d_n0;
        var_tmf1_dn1 = assign23920_e34942_d_n1;
        var_tmf1_dn2 = assign23920_e34942_d_n2;
        var_tmf1_dn3 = assign23920_e34942_d_n3;
        var_tmf1_dn4 = assign23920_e34942_d_n4;
        var_tmf1_dn5 = assign23920_e34942_d_n5;
        var_tmf1_db0 = assign23920_e34942_d_b0;
        var_tmf1_db1 = assign23920_e34942_d_b1;
        var_tmf1_db2 = assign23920_e34942_d_b2;
        var_tmf1_db3 = assign23920_e34942_d_b3;

        let (assign23930_e34956, assign23930_e34956_d_n0, assign23930_e34956_d_n1, assign23930_e34956_d_n2, assign23930_e34956_d_n3, assign23930_e34956_d_n4, assign23930_e34956_d_n5, assign23930_e34956_d_b0, assign23930_e34956_d_b1, assign23930_e34956_d_b2, assign23930_e34956_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23930_e34952: f64 = (4.0 * p.p85);
        let assign23930_e34954: f64 = (assign23930_e34952 * 0.01);
        (assign23930_e34954, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23930_e34956;
        var_tmf2_dn0 = assign23930_e34956_d_n0;
        var_tmf2_dn1 = assign23930_e34956_d_n1;
        var_tmf2_dn2 = assign23930_e34956_d_n2;
        var_tmf2_dn3 = assign23930_e34956_d_n3;
        var_tmf2_dn4 = assign23930_e34956_d_n4;
        var_tmf2_dn5 = assign23930_e34956_d_n5;
        var_tmf2_db0 = assign23930_e34956_d_b0;
        var_tmf2_db1 = assign23930_e34956_d_b1;
        var_tmf2_db2 = assign23930_e34956_d_b2;
        var_tmf2_db3 = assign23930_e34956_d_b3;


        *var_guard389_slot = var_guard389;
        *var_guard390_slot = var_guard390;
        *var_guard391_slot = var_guard391;
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

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard391: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_guard394_slot: &mut f64,
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
        let mut var_guard394: f64 = *var_guard394_slot;
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

        let (assign23940_e34972, assign23940_e34972_d_n0, assign23940_e34972_d_n1, assign23940_e34972_d_n2, assign23940_e34972_d_n3, assign23940_e34972_d_n4, assign23940_e34972_d_n5, assign23940_e34972_d_b0, assign23940_e34972_d_b1, assign23940_e34972_d_b2, assign23940_e34972_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n1, assign23940_e34970_d_n2, assign23940_e34970_d_n3, assign23940_e34970_d_n4, assign23940_e34970_d_n5, assign23940_e34970_d_b0, assign23940_e34970_d_b1, assign23940_e34970_d_b2, assign23940_e34970_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23940_e34969: f64 = (-var_tmf2);
                (assign23940_e34969, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23940_e34970, assign23940_e34970_d_n0, assign23940_e34970_d_n1, assign23940_e34970_d_n2, assign23940_e34970_d_n3, assign23940_e34970_d_n4, assign23940_e34970_d_n5, assign23940_e34970_d_b0, assign23940_e34970_d_b1, assign23940_e34970_d_b2, assign23940_e34970_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23940_e34972;
        var_tmf2_dn0 = assign23940_e34972_d_n0;
        var_tmf2_dn1 = assign23940_e34972_d_n1;
        var_tmf2_dn2 = assign23940_e34972_d_n2;
        var_tmf2_dn3 = assign23940_e34972_d_n3;
        var_tmf2_dn4 = assign23940_e34972_d_n4;
        var_tmf2_dn5 = assign23940_e34972_d_n5;
        var_tmf2_db0 = assign23940_e34972_d_b0;
        var_tmf2_db1 = assign23940_e34972_d_b1;
        var_tmf2_db2 = assign23940_e34972_d_b2;
        var_tmf2_db3 = assign23940_e34972_d_b3;

        let (assign23950_e34987, assign23950_e34987_d_n0, assign23950_e34987_d_n1, assign23950_e34987_d_n2, assign23950_e34987_d_n3, assign23950_e34987_d_n4, assign23950_e34987_d_n5, assign23950_e34987_d_b0, assign23950_e34987_d_b1, assign23950_e34987_d_b2, assign23950_e34987_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23950_e34982: f64 = (var_tmf1 * var_tmf1);
        let assign23950_e34984: f64 = (assign23950_e34982 + var_tmf2);
        let assign23950_e34985: f64 = (assign23950_e34984).sqrt();
        (assign23950_e34985, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign23950_e34985)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign23950_e34985)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign23950_e34985)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign23950_e34985)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign23950_e34985)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign23950_e34985)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23950_e34987;
        var_tmf2_dn0 = assign23950_e34987_d_n0;
        var_tmf2_dn1 = assign23950_e34987_d_n1;
        var_tmf2_dn2 = assign23950_e34987_d_n2;
        var_tmf2_dn3 = assign23950_e34987_d_n3;
        var_tmf2_dn4 = assign23950_e34987_d_n4;
        var_tmf2_dn5 = assign23950_e34987_d_n5;
        var_tmf2_db0 = assign23950_e34987_d_b0;
        var_tmf2_db1 = assign23950_e34987_d_b1;
        var_tmf2_db2 = assign23950_e34987_d_b2;
        var_tmf2_db3 = assign23950_e34987_d_b3;

        let (assign23960_e35003, assign23960_e35003_d_n0, assign23960_e35003_d_n1, assign23960_e35003_d_n2, assign23960_e35003_d_n3, assign23960_e35003_d_n4, assign23960_e35003_d_n5, assign23960_e35003_d_b0, assign23960_e35003_d_b1, assign23960_e35003_d_b2, assign23960_e35003_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23960_e34999: f64 = (var_tmf1 + var_tmf2);
        let assign23960_e35000: f64 = (0.5 * assign23960_e34999);
        let assign23960_e35001: f64 = (p.p85 - assign23960_e35000);
        (assign23960_e35001, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign23960_e35003;
        var_nja11_dn0 = assign23960_e35003_d_n0;
        var_nja11_dn1 = assign23960_e35003_d_n1;
        var_nja11_dn2 = assign23960_e35003_d_n2;
        var_nja11_dn3 = assign23960_e35003_d_n3;
        var_nja11_dn4 = assign23960_e35003_d_n4;
        var_nja11_dn5 = assign23960_e35003_d_n5;
        var_nja11_db0 = assign23960_e35003_d_b0;
        var_nja11_db1 = assign23960_e35003_d_b1;
        var_nja11_db2 = assign23960_e35003_d_b2;
        var_nja11_db3 = assign23960_e35003_d_b3;

        let (assign23970_e35017, assign23970_e35017_d_n0, assign23970_e35017_d_n1, assign23970_e35017_d_n2, assign23970_e35017_d_n3, assign23970_e35017_d_n4, assign23970_e35017_d_n5, assign23970_e35017_d_b0, assign23970_e35017_d_b1, assign23970_e35017_d_b2, assign23970_e35017_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23970_e35013: f64 = (var_nja11 - var_nfasti_i);
        let assign23970_e35015: f64 = (assign23970_e35013 - 0.01);
        (assign23970_e35015, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign23970_e35017;
        var_tmf1_dn0 = assign23970_e35017_d_n0;
        var_tmf1_dn1 = assign23970_e35017_d_n1;
        var_tmf1_dn2 = assign23970_e35017_d_n2;
        var_tmf1_dn3 = assign23970_e35017_d_n3;
        var_tmf1_dn4 = assign23970_e35017_d_n4;
        var_tmf1_dn5 = assign23970_e35017_d_n5;
        var_tmf1_db0 = assign23970_e35017_d_b0;
        var_tmf1_db1 = assign23970_e35017_d_b1;
        var_tmf1_db2 = assign23970_e35017_d_b2;
        var_tmf1_db3 = assign23970_e35017_d_b3;

        let (assign23980_e35031, assign23980_e35031_d_n0, assign23980_e35031_d_n1, assign23980_e35031_d_n2, assign23980_e35031_d_n3, assign23980_e35031_d_n4, assign23980_e35031_d_n5, assign23980_e35031_d_b0, assign23980_e35031_d_b1, assign23980_e35031_d_b2, assign23980_e35031_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign23980_e35027: f64 = (4.0 * var_nfasti_i);
        let assign23980_e35029: f64 = (assign23980_e35027 * 0.01);
        (assign23980_e35029, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23980_e35031;
        var_tmf2_dn0 = assign23980_e35031_d_n0;
        var_tmf2_dn1 = assign23980_e35031_d_n1;
        var_tmf2_dn2 = assign23980_e35031_d_n2;
        var_tmf2_dn3 = assign23980_e35031_d_n3;
        var_tmf2_dn4 = assign23980_e35031_d_n4;
        var_tmf2_dn5 = assign23980_e35031_d_n5;
        var_tmf2_db0 = assign23980_e35031_d_b0;
        var_tmf2_db1 = assign23980_e35031_d_b1;
        var_tmf2_db2 = assign23980_e35031_d_b2;
        var_tmf2_db3 = assign23980_e35031_d_b3;

        let (assign23990_e35047, assign23990_e35047_d_n0, assign23990_e35047_d_n1, assign23990_e35047_d_n2, assign23990_e35047_d_n3, assign23990_e35047_d_n4, assign23990_e35047_d_n5, assign23990_e35047_d_b0, assign23990_e35047_d_b1, assign23990_e35047_d_b2, assign23990_e35047_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n1, assign23990_e35045_d_n2, assign23990_e35045_d_n3, assign23990_e35045_d_n4, assign23990_e35045_d_n5, assign23990_e35045_d_b0, assign23990_e35045_d_b1, assign23990_e35045_d_b2, assign23990_e35045_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign23990_e35044: f64 = (-var_tmf2);
                (assign23990_e35044, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign23990_e35045, assign23990_e35045_d_n0, assign23990_e35045_d_n1, assign23990_e35045_d_n2, assign23990_e35045_d_n3, assign23990_e35045_d_n4, assign23990_e35045_d_n5, assign23990_e35045_d_b0, assign23990_e35045_d_b1, assign23990_e35045_d_b2, assign23990_e35045_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign23990_e35047;
        var_tmf2_dn0 = assign23990_e35047_d_n0;
        var_tmf2_dn1 = assign23990_e35047_d_n1;
        var_tmf2_dn2 = assign23990_e35047_d_n2;
        var_tmf2_dn3 = assign23990_e35047_d_n3;
        var_tmf2_dn4 = assign23990_e35047_d_n4;
        var_tmf2_dn5 = assign23990_e35047_d_n5;
        var_tmf2_db0 = assign23990_e35047_d_b0;
        var_tmf2_db1 = assign23990_e35047_d_b1;
        var_tmf2_db2 = assign23990_e35047_d_b2;
        var_tmf2_db3 = assign23990_e35047_d_b3;

        let (assign24000_e35062, assign24000_e35062_d_n0, assign24000_e35062_d_n1, assign24000_e35062_d_n2, assign24000_e35062_d_n3, assign24000_e35062_d_n4, assign24000_e35062_d_n5, assign24000_e35062_d_b0, assign24000_e35062_d_b1, assign24000_e35062_d_b2, assign24000_e35062_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24000_e35057: f64 = (var_tmf1 * var_tmf1);
        let assign24000_e35059: f64 = (assign24000_e35057 + var_tmf2);
        let assign24000_e35060: f64 = (assign24000_e35059).sqrt();
        (assign24000_e35060, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24000_e35060)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24000_e35060)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24000_e35060)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24000_e35060)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24000_e35060)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24000_e35060)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24000_e35062;
        var_tmf2_dn0 = assign24000_e35062_d_n0;
        var_tmf2_dn1 = assign24000_e35062_d_n1;
        var_tmf2_dn2 = assign24000_e35062_d_n2;
        var_tmf2_dn3 = assign24000_e35062_d_n3;
        var_tmf2_dn4 = assign24000_e35062_d_n4;
        var_tmf2_dn5 = assign24000_e35062_d_n5;
        var_tmf2_db0 = assign24000_e35062_d_b0;
        var_tmf2_db1 = assign24000_e35062_d_b1;
        var_tmf2_db2 = assign24000_e35062_d_b2;
        var_tmf2_db3 = assign24000_e35062_d_b3;

        let (assign24010_e35078, assign24010_e35078_d_n0, assign24010_e35078_d_n1, assign24010_e35078_d_n2, assign24010_e35078_d_n3, assign24010_e35078_d_n4, assign24010_e35078_d_n5, assign24010_e35078_d_b0, assign24010_e35078_d_b1, assign24010_e35078_d_b2, assign24010_e35078_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24010_e35074: f64 = (var_tmf1 + var_tmf2);
        let assign24010_e35075: f64 = (0.5 * assign24010_e35074);
        let assign24010_e35076: f64 = (var_nfasti_i + assign24010_e35075);
        (assign24010_e35076, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24010_e35078;
        var_nj1_dn0 = assign24010_e35078_d_n0;
        var_nj1_dn1 = assign24010_e35078_d_n1;
        var_nj1_dn2 = assign24010_e35078_d_n2;
        var_nj1_dn3 = assign24010_e35078_d_n3;
        var_nj1_dn4 = assign24010_e35078_d_n4;
        var_nj1_dn5 = assign24010_e35078_d_n5;
        var_nj1_db0 = assign24010_e35078_d_b0;
        var_nj1_db1 = assign24010_e35078_d_b1;
        var_nj1_db2 = assign24010_e35078_d_b2;
        var_nj1_db3 = assign24010_e35078_d_b3;

        let (assign24020_e35092, assign24020_e35092_d_n0, assign24020_e35092_d_n1, assign24020_e35092_d_n2, assign24020_e35092_d_n3, assign24020_e35092_d_n4, assign24020_e35092_d_n5, assign24020_e35092_d_b0, assign24020_e35092_d_b1, assign24020_e35092_d_b2, assign24020_e35092_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24020_e35088: f64 = (p.p85 - var_nj0);
        let assign24020_e35090: f64 = (assign24020_e35088 - 0.01);
        (assign24020_e35090, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24020_e35092;
        var_tmf1_dn0 = assign24020_e35092_d_n0;
        var_tmf1_dn1 = assign24020_e35092_d_n1;
        var_tmf1_dn2 = assign24020_e35092_d_n2;
        var_tmf1_dn3 = assign24020_e35092_d_n3;
        var_tmf1_dn4 = assign24020_e35092_d_n4;
        var_tmf1_dn5 = assign24020_e35092_d_n5;
        var_tmf1_db0 = assign24020_e35092_d_b0;
        var_tmf1_db1 = assign24020_e35092_d_b1;
        var_tmf1_db2 = assign24020_e35092_d_b2;
        var_tmf1_db3 = assign24020_e35092_d_b3;

        let (assign24030_e35106, assign24030_e35106_d_n0, assign24030_e35106_d_n1, assign24030_e35106_d_n2, assign24030_e35106_d_n3, assign24030_e35106_d_n4, assign24030_e35106_d_n5, assign24030_e35106_d_b0, assign24030_e35106_d_b1, assign24030_e35106_d_b2, assign24030_e35106_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24030_e35102: f64 = (4.0 * p.p85);
        let assign24030_e35104: f64 = (assign24030_e35102 * 0.01);
        (assign24030_e35104, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24030_e35106;
        var_tmf2_dn0 = assign24030_e35106_d_n0;
        var_tmf2_dn1 = assign24030_e35106_d_n1;
        var_tmf2_dn2 = assign24030_e35106_d_n2;
        var_tmf2_dn3 = assign24030_e35106_d_n3;
        var_tmf2_dn4 = assign24030_e35106_d_n4;
        var_tmf2_dn5 = assign24030_e35106_d_n5;
        var_tmf2_db0 = assign24030_e35106_d_b0;
        var_tmf2_db1 = assign24030_e35106_d_b1;
        var_tmf2_db2 = assign24030_e35106_d_b2;
        var_tmf2_db3 = assign24030_e35106_d_b3;

        let (assign24040_e35122, assign24040_e35122_d_n0, assign24040_e35122_d_n1, assign24040_e35122_d_n2, assign24040_e35122_d_n3, assign24040_e35122_d_n4, assign24040_e35122_d_n5, assign24040_e35122_d_b0, assign24040_e35122_d_b1, assign24040_e35122_d_b2, assign24040_e35122_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n1, assign24040_e35120_d_n2, assign24040_e35120_d_n3, assign24040_e35120_d_n4, assign24040_e35120_d_n5, assign24040_e35120_d_b0, assign24040_e35120_d_b1, assign24040_e35120_d_b2, assign24040_e35120_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24040_e35119: f64 = (-var_tmf2);
                (assign24040_e35119, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24040_e35120, assign24040_e35120_d_n0, assign24040_e35120_d_n1, assign24040_e35120_d_n2, assign24040_e35120_d_n3, assign24040_e35120_d_n4, assign24040_e35120_d_n5, assign24040_e35120_d_b0, assign24040_e35120_d_b1, assign24040_e35120_d_b2, assign24040_e35120_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24040_e35122;
        var_tmf2_dn0 = assign24040_e35122_d_n0;
        var_tmf2_dn1 = assign24040_e35122_d_n1;
        var_tmf2_dn2 = assign24040_e35122_d_n2;
        var_tmf2_dn3 = assign24040_e35122_d_n3;
        var_tmf2_dn4 = assign24040_e35122_d_n4;
        var_tmf2_dn5 = assign24040_e35122_d_n5;
        var_tmf2_db0 = assign24040_e35122_d_b0;
        var_tmf2_db1 = assign24040_e35122_d_b1;
        var_tmf2_db2 = assign24040_e35122_d_b2;
        var_tmf2_db3 = assign24040_e35122_d_b3;

        let (assign24050_e35137, assign24050_e35137_d_n0, assign24050_e35137_d_n1, assign24050_e35137_d_n2, assign24050_e35137_d_n3, assign24050_e35137_d_n4, assign24050_e35137_d_n5, assign24050_e35137_d_b0, assign24050_e35137_d_b1, assign24050_e35137_d_b2, assign24050_e35137_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24050_e35132: f64 = (var_tmf1 * var_tmf1);
        let assign24050_e35134: f64 = (assign24050_e35132 + var_tmf2);
        let assign24050_e35135: f64 = (assign24050_e35134).sqrt();
        (assign24050_e35135, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24050_e35135)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24050_e35135)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24050_e35135)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24050_e35135)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24050_e35135)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24050_e35135)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24050_e35137;
        var_tmf2_dn0 = assign24050_e35137_d_n0;
        var_tmf2_dn1 = assign24050_e35137_d_n1;
        var_tmf2_dn2 = assign24050_e35137_d_n2;
        var_tmf2_dn3 = assign24050_e35137_d_n3;
        var_tmf2_dn4 = assign24050_e35137_d_n4;
        var_tmf2_dn5 = assign24050_e35137_d_n5;
        var_tmf2_db0 = assign24050_e35137_d_b0;
        var_tmf2_db1 = assign24050_e35137_d_b1;
        var_tmf2_db2 = assign24050_e35137_d_b2;
        var_tmf2_db3 = assign24050_e35137_d_b3;

        let (assign24060_e35153, assign24060_e35153_d_n0, assign24060_e35153_d_n1, assign24060_e35153_d_n2, assign24060_e35153_d_n3, assign24060_e35153_d_n4, assign24060_e35153_d_n5, assign24060_e35153_d_b0, assign24060_e35153_d_b1, assign24060_e35153_d_b2, assign24060_e35153_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24060_e35149: f64 = (var_tmf1 + var_tmf2);
        let assign24060_e35150: f64 = (0.5 * assign24060_e35149);
        let assign24060_e35151: f64 = (p.p85 - assign24060_e35150);
        (assign24060_e35151, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24060_e35153;
        var_nj0_dn0 = assign24060_e35153_d_n0;
        var_nj0_dn1 = assign24060_e35153_d_n1;
        var_nj0_dn2 = assign24060_e35153_d_n2;
        var_nj0_dn3 = assign24060_e35153_d_n3;
        var_nj0_dn4 = assign24060_e35153_d_n4;
        var_nj0_dn5 = assign24060_e35153_d_n5;
        var_nj0_db0 = assign24060_e35153_d_b0;
        var_nj0_db1 = assign24060_e35153_d_b1;
        var_nj0_db2 = assign24060_e35153_d_b2;
        var_nj0_db3 = assign24060_e35153_d_b3;

        let (assign24070_e35167, assign24070_e35167_d_n0, assign24070_e35167_d_n1, assign24070_e35167_d_n2, assign24070_e35167_d_n3, assign24070_e35167_d_n4, assign24070_e35167_d_n5, assign24070_e35167_d_b0, assign24070_e35167_d_b1, assign24070_e35167_d_b2, assign24070_e35167_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24070_e35163: f64 = (var_nj0 - var_nfasti_i);
        let assign24070_e35165: f64 = (assign24070_e35163 - 0.01);
        (assign24070_e35165, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24070_e35167;
        var_tmf1_dn0 = assign24070_e35167_d_n0;
        var_tmf1_dn1 = assign24070_e35167_d_n1;
        var_tmf1_dn2 = assign24070_e35167_d_n2;
        var_tmf1_dn3 = assign24070_e35167_d_n3;
        var_tmf1_dn4 = assign24070_e35167_d_n4;
        var_tmf1_dn5 = assign24070_e35167_d_n5;
        var_tmf1_db0 = assign24070_e35167_d_b0;
        var_tmf1_db1 = assign24070_e35167_d_b1;
        var_tmf1_db2 = assign24070_e35167_d_b2;
        var_tmf1_db3 = assign24070_e35167_d_b3;

        let (assign24080_e35181, assign24080_e35181_d_n0, assign24080_e35181_d_n1, assign24080_e35181_d_n2, assign24080_e35181_d_n3, assign24080_e35181_d_n4, assign24080_e35181_d_n5, assign24080_e35181_d_b0, assign24080_e35181_d_b1, assign24080_e35181_d_b2, assign24080_e35181_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24080_e35177: f64 = (4.0 * var_nfasti_i);
        let assign24080_e35179: f64 = (assign24080_e35177 * 0.01);
        (assign24080_e35179, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24080_e35181;
        var_tmf2_dn0 = assign24080_e35181_d_n0;
        var_tmf2_dn1 = assign24080_e35181_d_n1;
        var_tmf2_dn2 = assign24080_e35181_d_n2;
        var_tmf2_dn3 = assign24080_e35181_d_n3;
        var_tmf2_dn4 = assign24080_e35181_d_n4;
        var_tmf2_dn5 = assign24080_e35181_d_n5;
        var_tmf2_db0 = assign24080_e35181_d_b0;
        var_tmf2_db1 = assign24080_e35181_d_b1;
        var_tmf2_db2 = assign24080_e35181_d_b2;
        var_tmf2_db3 = assign24080_e35181_d_b3;

        let (assign24090_e35197, assign24090_e35197_d_n0, assign24090_e35197_d_n1, assign24090_e35197_d_n2, assign24090_e35197_d_n3, assign24090_e35197_d_n4, assign24090_e35197_d_n5, assign24090_e35197_d_b0, assign24090_e35197_d_b1, assign24090_e35197_d_b2, assign24090_e35197_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n1, assign24090_e35195_d_n2, assign24090_e35195_d_n3, assign24090_e35195_d_n4, assign24090_e35195_d_n5, assign24090_e35195_d_b0, assign24090_e35195_d_b1, assign24090_e35195_d_b2, assign24090_e35195_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24090_e35194: f64 = (-var_tmf2);
                (assign24090_e35194, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24090_e35195, assign24090_e35195_d_n0, assign24090_e35195_d_n1, assign24090_e35195_d_n2, assign24090_e35195_d_n3, assign24090_e35195_d_n4, assign24090_e35195_d_n5, assign24090_e35195_d_b0, assign24090_e35195_d_b1, assign24090_e35195_d_b2, assign24090_e35195_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24090_e35197;
        var_tmf2_dn0 = assign24090_e35197_d_n0;
        var_tmf2_dn1 = assign24090_e35197_d_n1;
        var_tmf2_dn2 = assign24090_e35197_d_n2;
        var_tmf2_dn3 = assign24090_e35197_d_n3;
        var_tmf2_dn4 = assign24090_e35197_d_n4;
        var_tmf2_dn5 = assign24090_e35197_d_n5;
        var_tmf2_db0 = assign24090_e35197_d_b0;
        var_tmf2_db1 = assign24090_e35197_d_b1;
        var_tmf2_db2 = assign24090_e35197_d_b2;
        var_tmf2_db3 = assign24090_e35197_d_b3;

        let (assign24100_e35212, assign24100_e35212_d_n0, assign24100_e35212_d_n1, assign24100_e35212_d_n2, assign24100_e35212_d_n3, assign24100_e35212_d_n4, assign24100_e35212_d_n5, assign24100_e35212_d_b0, assign24100_e35212_d_b1, assign24100_e35212_d_b2, assign24100_e35212_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24100_e35207: f64 = (var_tmf1 * var_tmf1);
        let assign24100_e35209: f64 = (assign24100_e35207 + var_tmf2);
        let assign24100_e35210: f64 = (assign24100_e35209).sqrt();
        (assign24100_e35210, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24100_e35210)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24100_e35210)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24100_e35210)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24100_e35210)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24100_e35210)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24100_e35210)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24100_e35212;
        var_tmf2_dn0 = assign24100_e35212_d_n0;
        var_tmf2_dn1 = assign24100_e35212_d_n1;
        var_tmf2_dn2 = assign24100_e35212_d_n2;
        var_tmf2_dn3 = assign24100_e35212_d_n3;
        var_tmf2_dn4 = assign24100_e35212_d_n4;
        var_tmf2_dn5 = assign24100_e35212_d_n5;
        var_tmf2_db0 = assign24100_e35212_d_b0;
        var_tmf2_db1 = assign24100_e35212_d_b1;
        var_tmf2_db2 = assign24100_e35212_d_b2;
        var_tmf2_db3 = assign24100_e35212_d_b3;

        let (assign24110_e35228, assign24110_e35228_d_n0, assign24110_e35228_d_n1, assign24110_e35228_d_n2, assign24110_e35228_d_n3, assign24110_e35228_d_n4, assign24110_e35228_d_n5, assign24110_e35228_d_b0, assign24110_e35228_d_b1, assign24110_e35228_d_b2, assign24110_e35228_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 != 0.0)) {
        let assign24110_e35224: f64 = (var_tmf1 + var_tmf2);
        let assign24110_e35225: f64 = (0.5 * assign24110_e35224);
        let assign24110_e35226: f64 = (var_nfasti_i + assign24110_e35225);
        (assign24110_e35226, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24110_e35228;
        var_nj0_dn0 = assign24110_e35228_d_n0;
        var_nj0_dn1 = assign24110_e35228_d_n1;
        var_nj0_dn2 = assign24110_e35228_d_n2;
        var_nj0_dn3 = assign24110_e35228_d_n3;
        var_nj0_dn4 = assign24110_e35228_d_n4;
        var_nj0_dn5 = assign24110_e35228_d_n5;
        var_nj0_db0 = assign24110_e35228_d_b0;
        var_nj0_db1 = assign24110_e35228_d_b1;
        var_nj0_db2 = assign24110_e35228_d_b2;
        var_nj0_db3 = assign24110_e35228_d_b3;

        let (assign24120_e35239, assign24120_e35239_d_n0, assign24120_e35239_d_n1, assign24120_e35239_d_n2, assign24120_e35239_d_n3, assign24120_e35239_d_n4, assign24120_e35239_d_n5, assign24120_e35239_d_b0, assign24120_e35239_d_b1, assign24120_e35239_d_b2, assign24120_e35239_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24120_e35239;
        var_nj0_dn0 = assign24120_e35239_d_n0;
        var_nj0_dn1 = assign24120_e35239_d_n1;
        var_nj0_dn2 = assign24120_e35239_d_n2;
        var_nj0_dn3 = assign24120_e35239_d_n3;
        var_nj0_dn4 = assign24120_e35239_d_n4;
        var_nj0_dn5 = assign24120_e35239_d_n5;
        var_nj0_db0 = assign24120_e35239_d_b0;
        var_nj0_db1 = assign24120_e35239_d_b1;
        var_nj0_db2 = assign24120_e35239_d_b2;
        var_nj0_db3 = assign24120_e35239_d_b3;

        let (assign24130_e35250, assign24130_e35250_d_n0, assign24130_e35250_d_n1, assign24130_e35250_d_n2, assign24130_e35250_d_n3, assign24130_e35250_d_n4, assign24130_e35250_d_n5, assign24130_e35250_d_b0, assign24130_e35250_d_b1, assign24130_e35250_d_b2, assign24130_e35250_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard391 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24130_e35250;
        var_nj1_dn0 = assign24130_e35250_d_n0;
        var_nj1_dn1 = assign24130_e35250_d_n1;
        var_nj1_dn2 = assign24130_e35250_d_n2;
        var_nj1_dn3 = assign24130_e35250_d_n3;
        var_nj1_dn4 = assign24130_e35250_d_n4;
        var_nj1_dn5 = assign24130_e35250_d_n5;
        var_nj1_db0 = assign24130_e35250_d_b0;
        var_nj1_db1 = assign24130_e35250_d_b1;
        var_nj1_db2 = assign24130_e35250_d_b2;
        var_nj1_db3 = assign24130_e35250_d_b3;

        let (assign24190_e35481,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign24190_e35477: f64 = (var_nin * var_nin);
        let assign24190_e35479: f64 = (assign24190_e35477 / var_ndigat_i);
        (assign24190_e35479,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24190_e35481;

        let (assign24200_e35496,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) {
        let assign24200_e35489: f64 = (var_nfagat_i / var_phitdinv);
        let assign24200_e35492: f64 = (var_ndigat_i / var_pnn0);
        let assign24200_e35493: f64 = (assign24200_e35492).ln();
        let assign24200_e35494: f64 = (assign24200_e35489 * assign24200_e35493);
        (assign24200_e35494,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24200_e35496;

        let assign24210_e35499: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard394 = assign24210_e35499;

        let (assign24220_e35515, assign24220_e35515_d_n0, assign24220_e35515_d_n1, assign24220_e35515_d_n2, assign24220_e35515_d_n3, assign24220_e35515_d_n4, assign24220_e35515_d_n5, assign24220_e35515_d_b0, assign24220_e35515_d_b1, assign24220_e35515_d_b2, assign24220_e35515_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24220_e35510: f64 = (var_v5 - var_vha1);
        let assign24220_e35511: f64 = (p.p86 * assign24220_e35510);
        let assign24220_e35513: f64 = (assign24220_e35511 + var_nfagat_i);
        (assign24220_e35513, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign24220_e35515;
        var_nja10_dn0 = assign24220_e35515_d_n0;
        var_nja10_dn1 = assign24220_e35515_d_n1;
        var_nja10_dn2 = assign24220_e35515_d_n2;
        var_nja10_dn3 = assign24220_e35515_d_n3;
        var_nja10_dn4 = assign24220_e35515_d_n4;
        var_nja10_dn5 = assign24220_e35515_d_n5;
        var_nja10_db0 = assign24220_e35515_d_b0;
        var_nja10_db1 = assign24220_e35515_d_b1;
        var_nja10_db2 = assign24220_e35515_d_b2;
        var_nja10_db3 = assign24220_e35515_d_b3;


        *var_guard394_slot = var_guard394;
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

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard394: f64,
        var_nfagat_i: f64,
        var_nja10: f64,
        var_nja10_db0: f64,
        var_nja10_db1: f64,
        var_nja10_db2: f64,
        var_nja10_db3: f64,
        var_nja10_dn0: f64,
        var_nja10_dn1: f64,
        var_nja10_dn2: f64,
        var_nja10_dn3: f64,
        var_nja10_dn4: f64,
        var_nja10_dn5: f64,
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

        let (assign24230_e35529, assign24230_e35529_d_n0, assign24230_e35529_d_n1, assign24230_e35529_d_n2, assign24230_e35529_d_n3, assign24230_e35529_d_n4, assign24230_e35529_d_n5, assign24230_e35529_d_b0, assign24230_e35529_d_b1, assign24230_e35529_d_b2, assign24230_e35529_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24230_e35526: f64 = (p.p86 * var_vha1);
        let assign24230_e35527: f64 = (var_nfagat_i - assign24230_e35526);
        (assign24230_e35527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24230_e35529;
        var_nj0_dn0 = assign24230_e35529_d_n0;
        var_nj0_dn1 = assign24230_e35529_d_n1;
        var_nj0_dn2 = assign24230_e35529_d_n2;
        var_nj0_dn3 = assign24230_e35529_d_n3;
        var_nj0_dn4 = assign24230_e35529_d_n4;
        var_nj0_dn5 = assign24230_e35529_d_n5;
        var_nj0_db0 = assign24230_e35529_d_b0;
        var_nj0_db1 = assign24230_e35529_d_b1;
        var_nj0_db2 = assign24230_e35529_d_b2;
        var_nj0_db3 = assign24230_e35529_d_b3;

        let (assign24240_e35543, assign24240_e35543_d_n0, assign24240_e35543_d_n1, assign24240_e35543_d_n2, assign24240_e35543_d_n3, assign24240_e35543_d_n4, assign24240_e35543_d_n5, assign24240_e35543_d_b0, assign24240_e35543_d_b1, assign24240_e35543_d_b2, assign24240_e35543_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24240_e35539: f64 = (p.p85 - var_nja10);
        let assign24240_e35541: f64 = (assign24240_e35539 - 0.01);
        (assign24240_e35541, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24240_e35543;
        var_tmf1_dn0 = assign24240_e35543_d_n0;
        var_tmf1_dn1 = assign24240_e35543_d_n1;
        var_tmf1_dn2 = assign24240_e35543_d_n2;
        var_tmf1_dn3 = assign24240_e35543_d_n3;
        var_tmf1_dn4 = assign24240_e35543_d_n4;
        var_tmf1_dn5 = assign24240_e35543_d_n5;
        var_tmf1_db0 = assign24240_e35543_d_b0;
        var_tmf1_db1 = assign24240_e35543_d_b1;
        var_tmf1_db2 = assign24240_e35543_d_b2;
        var_tmf1_db3 = assign24240_e35543_d_b3;

        let (assign24250_e35557, assign24250_e35557_d_n0, assign24250_e35557_d_n1, assign24250_e35557_d_n2, assign24250_e35557_d_n3, assign24250_e35557_d_n4, assign24250_e35557_d_n5, assign24250_e35557_d_b0, assign24250_e35557_d_b1, assign24250_e35557_d_b2, assign24250_e35557_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24250_e35553: f64 = (4.0 * p.p85);
        let assign24250_e35555: f64 = (assign24250_e35553 * 0.01);
        (assign24250_e35555, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24250_e35557;
        var_tmf2_dn0 = assign24250_e35557_d_n0;
        var_tmf2_dn1 = assign24250_e35557_d_n1;
        var_tmf2_dn2 = assign24250_e35557_d_n2;
        var_tmf2_dn3 = assign24250_e35557_d_n3;
        var_tmf2_dn4 = assign24250_e35557_d_n4;
        var_tmf2_dn5 = assign24250_e35557_d_n5;
        var_tmf2_db0 = assign24250_e35557_d_b0;
        var_tmf2_db1 = assign24250_e35557_d_b1;
        var_tmf2_db2 = assign24250_e35557_d_b2;
        var_tmf2_db3 = assign24250_e35557_d_b3;

        let (assign24260_e35573, assign24260_e35573_d_n0, assign24260_e35573_d_n1, assign24260_e35573_d_n2, assign24260_e35573_d_n3, assign24260_e35573_d_n4, assign24260_e35573_d_n5, assign24260_e35573_d_b0, assign24260_e35573_d_b1, assign24260_e35573_d_b2, assign24260_e35573_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n1, assign24260_e35571_d_n2, assign24260_e35571_d_n3, assign24260_e35571_d_n4, assign24260_e35571_d_n5, assign24260_e35571_d_b0, assign24260_e35571_d_b1, assign24260_e35571_d_b2, assign24260_e35571_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24260_e35570: f64 = (-var_tmf2);
                (assign24260_e35570, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24260_e35571, assign24260_e35571_d_n0, assign24260_e35571_d_n1, assign24260_e35571_d_n2, assign24260_e35571_d_n3, assign24260_e35571_d_n4, assign24260_e35571_d_n5, assign24260_e35571_d_b0, assign24260_e35571_d_b1, assign24260_e35571_d_b2, assign24260_e35571_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24260_e35573;
        var_tmf2_dn0 = assign24260_e35573_d_n0;
        var_tmf2_dn1 = assign24260_e35573_d_n1;
        var_tmf2_dn2 = assign24260_e35573_d_n2;
        var_tmf2_dn3 = assign24260_e35573_d_n3;
        var_tmf2_dn4 = assign24260_e35573_d_n4;
        var_tmf2_dn5 = assign24260_e35573_d_n5;
        var_tmf2_db0 = assign24260_e35573_d_b0;
        var_tmf2_db1 = assign24260_e35573_d_b1;
        var_tmf2_db2 = assign24260_e35573_d_b2;
        var_tmf2_db3 = assign24260_e35573_d_b3;

        let (assign24270_e35588, assign24270_e35588_d_n0, assign24270_e35588_d_n1, assign24270_e35588_d_n2, assign24270_e35588_d_n3, assign24270_e35588_d_n4, assign24270_e35588_d_n5, assign24270_e35588_d_b0, assign24270_e35588_d_b1, assign24270_e35588_d_b2, assign24270_e35588_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24270_e35583: f64 = (var_tmf1 * var_tmf1);
        let assign24270_e35585: f64 = (assign24270_e35583 + var_tmf2);
        let assign24270_e35586: f64 = (assign24270_e35585).sqrt();
        (assign24270_e35586, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24270_e35586)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24270_e35586)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24270_e35586)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24270_e35586)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24270_e35586)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24270_e35586)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24270_e35588;
        var_tmf2_dn0 = assign24270_e35588_d_n0;
        var_tmf2_dn1 = assign24270_e35588_d_n1;
        var_tmf2_dn2 = assign24270_e35588_d_n2;
        var_tmf2_dn3 = assign24270_e35588_d_n3;
        var_tmf2_dn4 = assign24270_e35588_d_n4;
        var_tmf2_dn5 = assign24270_e35588_d_n5;
        var_tmf2_db0 = assign24270_e35588_d_b0;
        var_tmf2_db1 = assign24270_e35588_d_b1;
        var_tmf2_db2 = assign24270_e35588_d_b2;
        var_tmf2_db3 = assign24270_e35588_d_b3;

        let (assign24280_e35604, assign24280_e35604_d_n0, assign24280_e35604_d_n1, assign24280_e35604_d_n2, assign24280_e35604_d_n3, assign24280_e35604_d_n4, assign24280_e35604_d_n5, assign24280_e35604_d_b0, assign24280_e35604_d_b1, assign24280_e35604_d_b2, assign24280_e35604_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24280_e35600: f64 = (var_tmf1 + var_tmf2);
        let assign24280_e35601: f64 = (0.5 * assign24280_e35600);
        let assign24280_e35602: f64 = (p.p85 - assign24280_e35601);
        (assign24280_e35602, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign24280_e35604;
        var_nja11_dn0 = assign24280_e35604_d_n0;
        var_nja11_dn1 = assign24280_e35604_d_n1;
        var_nja11_dn2 = assign24280_e35604_d_n2;
        var_nja11_dn3 = assign24280_e35604_d_n3;
        var_nja11_dn4 = assign24280_e35604_d_n4;
        var_nja11_dn5 = assign24280_e35604_d_n5;
        var_nja11_db0 = assign24280_e35604_d_b0;
        var_nja11_db1 = assign24280_e35604_d_b1;
        var_nja11_db2 = assign24280_e35604_d_b2;
        var_nja11_db3 = assign24280_e35604_d_b3;

        let (assign24290_e35618, assign24290_e35618_d_n0, assign24290_e35618_d_n1, assign24290_e35618_d_n2, assign24290_e35618_d_n3, assign24290_e35618_d_n4, assign24290_e35618_d_n5, assign24290_e35618_d_b0, assign24290_e35618_d_b1, assign24290_e35618_d_b2, assign24290_e35618_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24290_e35614: f64 = (var_nja11 - var_nfagat_i);
        let assign24290_e35616: f64 = (assign24290_e35614 - 0.01);
        (assign24290_e35616, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24290_e35618;
        var_tmf1_dn0 = assign24290_e35618_d_n0;
        var_tmf1_dn1 = assign24290_e35618_d_n1;
        var_tmf1_dn2 = assign24290_e35618_d_n2;
        var_tmf1_dn3 = assign24290_e35618_d_n3;
        var_tmf1_dn4 = assign24290_e35618_d_n4;
        var_tmf1_dn5 = assign24290_e35618_d_n5;
        var_tmf1_db0 = assign24290_e35618_d_b0;
        var_tmf1_db1 = assign24290_e35618_d_b1;
        var_tmf1_db2 = assign24290_e35618_d_b2;
        var_tmf1_db3 = assign24290_e35618_d_b3;

        let (assign24300_e35632, assign24300_e35632_d_n0, assign24300_e35632_d_n1, assign24300_e35632_d_n2, assign24300_e35632_d_n3, assign24300_e35632_d_n4, assign24300_e35632_d_n5, assign24300_e35632_d_b0, assign24300_e35632_d_b1, assign24300_e35632_d_b2, assign24300_e35632_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24300_e35628: f64 = (4.0 * var_nfagat_i);
        let assign24300_e35630: f64 = (assign24300_e35628 * 0.01);
        (assign24300_e35630, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24300_e35632;
        var_tmf2_dn0 = assign24300_e35632_d_n0;
        var_tmf2_dn1 = assign24300_e35632_d_n1;
        var_tmf2_dn2 = assign24300_e35632_d_n2;
        var_tmf2_dn3 = assign24300_e35632_d_n3;
        var_tmf2_dn4 = assign24300_e35632_d_n4;
        var_tmf2_dn5 = assign24300_e35632_d_n5;
        var_tmf2_db0 = assign24300_e35632_d_b0;
        var_tmf2_db1 = assign24300_e35632_d_b1;
        var_tmf2_db2 = assign24300_e35632_d_b2;
        var_tmf2_db3 = assign24300_e35632_d_b3;

        let (assign24310_e35648, assign24310_e35648_d_n0, assign24310_e35648_d_n1, assign24310_e35648_d_n2, assign24310_e35648_d_n3, assign24310_e35648_d_n4, assign24310_e35648_d_n5, assign24310_e35648_d_b0, assign24310_e35648_d_b1, assign24310_e35648_d_b2, assign24310_e35648_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n1, assign24310_e35646_d_n2, assign24310_e35646_d_n3, assign24310_e35646_d_n4, assign24310_e35646_d_n5, assign24310_e35646_d_b0, assign24310_e35646_d_b1, assign24310_e35646_d_b2, assign24310_e35646_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24310_e35645: f64 = (-var_tmf2);
                (assign24310_e35645, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24310_e35646, assign24310_e35646_d_n0, assign24310_e35646_d_n1, assign24310_e35646_d_n2, assign24310_e35646_d_n3, assign24310_e35646_d_n4, assign24310_e35646_d_n5, assign24310_e35646_d_b0, assign24310_e35646_d_b1, assign24310_e35646_d_b2, assign24310_e35646_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24310_e35648;
        var_tmf2_dn0 = assign24310_e35648_d_n0;
        var_tmf2_dn1 = assign24310_e35648_d_n1;
        var_tmf2_dn2 = assign24310_e35648_d_n2;
        var_tmf2_dn3 = assign24310_e35648_d_n3;
        var_tmf2_dn4 = assign24310_e35648_d_n4;
        var_tmf2_dn5 = assign24310_e35648_d_n5;
        var_tmf2_db0 = assign24310_e35648_d_b0;
        var_tmf2_db1 = assign24310_e35648_d_b1;
        var_tmf2_db2 = assign24310_e35648_d_b2;
        var_tmf2_db3 = assign24310_e35648_d_b3;

        let (assign24320_e35663, assign24320_e35663_d_n0, assign24320_e35663_d_n1, assign24320_e35663_d_n2, assign24320_e35663_d_n3, assign24320_e35663_d_n4, assign24320_e35663_d_n5, assign24320_e35663_d_b0, assign24320_e35663_d_b1, assign24320_e35663_d_b2, assign24320_e35663_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24320_e35658: f64 = (var_tmf1 * var_tmf1);
        let assign24320_e35660: f64 = (assign24320_e35658 + var_tmf2);
        let assign24320_e35661: f64 = (assign24320_e35660).sqrt();
        (assign24320_e35661, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24320_e35661)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24320_e35661)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24320_e35661)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24320_e35661)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24320_e35661)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24320_e35661)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24320_e35663;
        var_tmf2_dn0 = assign24320_e35663_d_n0;
        var_tmf2_dn1 = assign24320_e35663_d_n1;
        var_tmf2_dn2 = assign24320_e35663_d_n2;
        var_tmf2_dn3 = assign24320_e35663_d_n3;
        var_tmf2_dn4 = assign24320_e35663_d_n4;
        var_tmf2_dn5 = assign24320_e35663_d_n5;
        var_tmf2_db0 = assign24320_e35663_d_b0;
        var_tmf2_db1 = assign24320_e35663_d_b1;
        var_tmf2_db2 = assign24320_e35663_d_b2;
        var_tmf2_db3 = assign24320_e35663_d_b3;

        let (assign24330_e35679, assign24330_e35679_d_n0, assign24330_e35679_d_n1, assign24330_e35679_d_n2, assign24330_e35679_d_n3, assign24330_e35679_d_n4, assign24330_e35679_d_n5, assign24330_e35679_d_b0, assign24330_e35679_d_b1, assign24330_e35679_d_b2, assign24330_e35679_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24330_e35675: f64 = (var_tmf1 + var_tmf2);
        let assign24330_e35676: f64 = (0.5 * assign24330_e35675);
        let assign24330_e35677: f64 = (var_nfagat_i + assign24330_e35676);
        (assign24330_e35677, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24330_e35679;
        var_nj1_dn0 = assign24330_e35679_d_n0;
        var_nj1_dn1 = assign24330_e35679_d_n1;
        var_nj1_dn2 = assign24330_e35679_d_n2;
        var_nj1_dn3 = assign24330_e35679_d_n3;
        var_nj1_dn4 = assign24330_e35679_d_n4;
        var_nj1_dn5 = assign24330_e35679_d_n5;
        var_nj1_db0 = assign24330_e35679_d_b0;
        var_nj1_db1 = assign24330_e35679_d_b1;
        var_nj1_db2 = assign24330_e35679_d_b2;
        var_nj1_db3 = assign24330_e35679_d_b3;

        let (assign24340_e35693, assign24340_e35693_d_n0, assign24340_e35693_d_n1, assign24340_e35693_d_n2, assign24340_e35693_d_n3, assign24340_e35693_d_n4, assign24340_e35693_d_n5, assign24340_e35693_d_b0, assign24340_e35693_d_b1, assign24340_e35693_d_b2, assign24340_e35693_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24340_e35689: f64 = (p.p85 - var_nj0);
        let assign24340_e35691: f64 = (assign24340_e35689 - 0.01);
        (assign24340_e35691, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24340_e35693;
        var_tmf1_dn0 = assign24340_e35693_d_n0;
        var_tmf1_dn1 = assign24340_e35693_d_n1;
        var_tmf1_dn2 = assign24340_e35693_d_n2;
        var_tmf1_dn3 = assign24340_e35693_d_n3;
        var_tmf1_dn4 = assign24340_e35693_d_n4;
        var_tmf1_dn5 = assign24340_e35693_d_n5;
        var_tmf1_db0 = assign24340_e35693_d_b0;
        var_tmf1_db1 = assign24340_e35693_d_b1;
        var_tmf1_db2 = assign24340_e35693_d_b2;
        var_tmf1_db3 = assign24340_e35693_d_b3;

        let (assign24350_e35707, assign24350_e35707_d_n0, assign24350_e35707_d_n1, assign24350_e35707_d_n2, assign24350_e35707_d_n3, assign24350_e35707_d_n4, assign24350_e35707_d_n5, assign24350_e35707_d_b0, assign24350_e35707_d_b1, assign24350_e35707_d_b2, assign24350_e35707_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24350_e35703: f64 = (4.0 * p.p85);
        let assign24350_e35705: f64 = (assign24350_e35703 * 0.01);
        (assign24350_e35705, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24350_e35707;
        var_tmf2_dn0 = assign24350_e35707_d_n0;
        var_tmf2_dn1 = assign24350_e35707_d_n1;
        var_tmf2_dn2 = assign24350_e35707_d_n2;
        var_tmf2_dn3 = assign24350_e35707_d_n3;
        var_tmf2_dn4 = assign24350_e35707_d_n4;
        var_tmf2_dn5 = assign24350_e35707_d_n5;
        var_tmf2_db0 = assign24350_e35707_d_b0;
        var_tmf2_db1 = assign24350_e35707_d_b1;
        var_tmf2_db2 = assign24350_e35707_d_b2;
        var_tmf2_db3 = assign24350_e35707_d_b3;

        let (assign24360_e35723, assign24360_e35723_d_n0, assign24360_e35723_d_n1, assign24360_e35723_d_n2, assign24360_e35723_d_n3, assign24360_e35723_d_n4, assign24360_e35723_d_n5, assign24360_e35723_d_b0, assign24360_e35723_d_b1, assign24360_e35723_d_b2, assign24360_e35723_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n1, assign24360_e35721_d_n2, assign24360_e35721_d_n3, assign24360_e35721_d_n4, assign24360_e35721_d_n5, assign24360_e35721_d_b0, assign24360_e35721_d_b1, assign24360_e35721_d_b2, assign24360_e35721_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24360_e35720: f64 = (-var_tmf2);
                (assign24360_e35720, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24360_e35721, assign24360_e35721_d_n0, assign24360_e35721_d_n1, assign24360_e35721_d_n2, assign24360_e35721_d_n3, assign24360_e35721_d_n4, assign24360_e35721_d_n5, assign24360_e35721_d_b0, assign24360_e35721_d_b1, assign24360_e35721_d_b2, assign24360_e35721_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24360_e35723;
        var_tmf2_dn0 = assign24360_e35723_d_n0;
        var_tmf2_dn1 = assign24360_e35723_d_n1;
        var_tmf2_dn2 = assign24360_e35723_d_n2;
        var_tmf2_dn3 = assign24360_e35723_d_n3;
        var_tmf2_dn4 = assign24360_e35723_d_n4;
        var_tmf2_dn5 = assign24360_e35723_d_n5;
        var_tmf2_db0 = assign24360_e35723_d_b0;
        var_tmf2_db1 = assign24360_e35723_d_b1;
        var_tmf2_db2 = assign24360_e35723_d_b2;
        var_tmf2_db3 = assign24360_e35723_d_b3;

        let (assign24370_e35738, assign24370_e35738_d_n0, assign24370_e35738_d_n1, assign24370_e35738_d_n2, assign24370_e35738_d_n3, assign24370_e35738_d_n4, assign24370_e35738_d_n5, assign24370_e35738_d_b0, assign24370_e35738_d_b1, assign24370_e35738_d_b2, assign24370_e35738_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24370_e35733: f64 = (var_tmf1 * var_tmf1);
        let assign24370_e35735: f64 = (assign24370_e35733 + var_tmf2);
        let assign24370_e35736: f64 = (assign24370_e35735).sqrt();
        (assign24370_e35736, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24370_e35736)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24370_e35736)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24370_e35736)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24370_e35736)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24370_e35736)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24370_e35736)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24370_e35738;
        var_tmf2_dn0 = assign24370_e35738_d_n0;
        var_tmf2_dn1 = assign24370_e35738_d_n1;
        var_tmf2_dn2 = assign24370_e35738_d_n2;
        var_tmf2_dn3 = assign24370_e35738_d_n3;
        var_tmf2_dn4 = assign24370_e35738_d_n4;
        var_tmf2_dn5 = assign24370_e35738_d_n5;
        var_tmf2_db0 = assign24370_e35738_d_b0;
        var_tmf2_db1 = assign24370_e35738_d_b1;
        var_tmf2_db2 = assign24370_e35738_d_b2;
        var_tmf2_db3 = assign24370_e35738_d_b3;

        let (assign24380_e35754, assign24380_e35754_d_n0, assign24380_e35754_d_n1, assign24380_e35754_d_n2, assign24380_e35754_d_n3, assign24380_e35754_d_n4, assign24380_e35754_d_n5, assign24380_e35754_d_b0, assign24380_e35754_d_b1, assign24380_e35754_d_b2, assign24380_e35754_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24380_e35750: f64 = (var_tmf1 + var_tmf2);
        let assign24380_e35751: f64 = (0.5 * assign24380_e35750);
        let assign24380_e35752: f64 = (p.p85 - assign24380_e35751);
        (assign24380_e35752, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24380_e35754;
        var_nj0_dn0 = assign24380_e35754_d_n0;
        var_nj0_dn1 = assign24380_e35754_d_n1;
        var_nj0_dn2 = assign24380_e35754_d_n2;
        var_nj0_dn3 = assign24380_e35754_d_n3;
        var_nj0_dn4 = assign24380_e35754_d_n4;
        var_nj0_dn5 = assign24380_e35754_d_n5;
        var_nj0_db0 = assign24380_e35754_d_b0;
        var_nj0_db1 = assign24380_e35754_d_b1;
        var_nj0_db2 = assign24380_e35754_d_b2;
        var_nj0_db3 = assign24380_e35754_d_b3;

        let (assign24390_e35768, assign24390_e35768_d_n0, assign24390_e35768_d_n1, assign24390_e35768_d_n2, assign24390_e35768_d_n3, assign24390_e35768_d_n4, assign24390_e35768_d_n5, assign24390_e35768_d_b0, assign24390_e35768_d_b1, assign24390_e35768_d_b2, assign24390_e35768_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24390_e35764: f64 = (var_nj0 - var_nfagat_i);
        let assign24390_e35766: f64 = (assign24390_e35764 - 0.01);
        (assign24390_e35766, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24390_e35768;
        var_tmf1_dn0 = assign24390_e35768_d_n0;
        var_tmf1_dn1 = assign24390_e35768_d_n1;
        var_tmf1_dn2 = assign24390_e35768_d_n2;
        var_tmf1_dn3 = assign24390_e35768_d_n3;
        var_tmf1_dn4 = assign24390_e35768_d_n4;
        var_tmf1_dn5 = assign24390_e35768_d_n5;
        var_tmf1_db0 = assign24390_e35768_d_b0;
        var_tmf1_db1 = assign24390_e35768_d_b1;
        var_tmf1_db2 = assign24390_e35768_d_b2;
        var_tmf1_db3 = assign24390_e35768_d_b3;

        let (assign24400_e35782, assign24400_e35782_d_n0, assign24400_e35782_d_n1, assign24400_e35782_d_n2, assign24400_e35782_d_n3, assign24400_e35782_d_n4, assign24400_e35782_d_n5, assign24400_e35782_d_b0, assign24400_e35782_d_b1, assign24400_e35782_d_b2, assign24400_e35782_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24400_e35778: f64 = (4.0 * var_nfagat_i);
        let assign24400_e35780: f64 = (assign24400_e35778 * 0.01);
        (assign24400_e35780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24400_e35782;
        var_tmf2_dn0 = assign24400_e35782_d_n0;
        var_tmf2_dn1 = assign24400_e35782_d_n1;
        var_tmf2_dn2 = assign24400_e35782_d_n2;
        var_tmf2_dn3 = assign24400_e35782_d_n3;
        var_tmf2_dn4 = assign24400_e35782_d_n4;
        var_tmf2_dn5 = assign24400_e35782_d_n5;
        var_tmf2_db0 = assign24400_e35782_d_b0;
        var_tmf2_db1 = assign24400_e35782_d_b1;
        var_tmf2_db2 = assign24400_e35782_d_b2;
        var_tmf2_db3 = assign24400_e35782_d_b3;

        let (assign24410_e35798, assign24410_e35798_d_n0, assign24410_e35798_d_n1, assign24410_e35798_d_n2, assign24410_e35798_d_n3, assign24410_e35798_d_n4, assign24410_e35798_d_n5, assign24410_e35798_d_b0, assign24410_e35798_d_b1, assign24410_e35798_d_b2, assign24410_e35798_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n1, assign24410_e35796_d_n2, assign24410_e35796_d_n3, assign24410_e35796_d_n4, assign24410_e35796_d_n5, assign24410_e35796_d_b0, assign24410_e35796_d_b1, assign24410_e35796_d_b2, assign24410_e35796_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24410_e35795: f64 = (-var_tmf2);
                (assign24410_e35795, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24410_e35796, assign24410_e35796_d_n0, assign24410_e35796_d_n1, assign24410_e35796_d_n2, assign24410_e35796_d_n3, assign24410_e35796_d_n4, assign24410_e35796_d_n5, assign24410_e35796_d_b0, assign24410_e35796_d_b1, assign24410_e35796_d_b2, assign24410_e35796_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24410_e35798;
        var_tmf2_dn0 = assign24410_e35798_d_n0;
        var_tmf2_dn1 = assign24410_e35798_d_n1;
        var_tmf2_dn2 = assign24410_e35798_d_n2;
        var_tmf2_dn3 = assign24410_e35798_d_n3;
        var_tmf2_dn4 = assign24410_e35798_d_n4;
        var_tmf2_dn5 = assign24410_e35798_d_n5;
        var_tmf2_db0 = assign24410_e35798_d_b0;
        var_tmf2_db1 = assign24410_e35798_d_b1;
        var_tmf2_db2 = assign24410_e35798_d_b2;
        var_tmf2_db3 = assign24410_e35798_d_b3;

        let (assign24420_e35813, assign24420_e35813_d_n0, assign24420_e35813_d_n1, assign24420_e35813_d_n2, assign24420_e35813_d_n3, assign24420_e35813_d_n4, assign24420_e35813_d_n5, assign24420_e35813_d_b0, assign24420_e35813_d_b1, assign24420_e35813_d_b2, assign24420_e35813_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24420_e35808: f64 = (var_tmf1 * var_tmf1);
        let assign24420_e35810: f64 = (assign24420_e35808 + var_tmf2);
        let assign24420_e35811: f64 = (assign24420_e35810).sqrt();
        (assign24420_e35811, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24420_e35811)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24420_e35811)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24420_e35811)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24420_e35811)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24420_e35811)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24420_e35811)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24420_e35813;
        var_tmf2_dn0 = assign24420_e35813_d_n0;
        var_tmf2_dn1 = assign24420_e35813_d_n1;
        var_tmf2_dn2 = assign24420_e35813_d_n2;
        var_tmf2_dn3 = assign24420_e35813_d_n3;
        var_tmf2_dn4 = assign24420_e35813_d_n4;
        var_tmf2_dn5 = assign24420_e35813_d_n5;
        var_tmf2_db0 = assign24420_e35813_d_b0;
        var_tmf2_db1 = assign24420_e35813_d_b1;
        var_tmf2_db2 = assign24420_e35813_d_b2;
        var_tmf2_db3 = assign24420_e35813_d_b3;

        let (assign24430_e35829, assign24430_e35829_d_n0, assign24430_e35829_d_n1, assign24430_e35829_d_n2, assign24430_e35829_d_n3, assign24430_e35829_d_n4, assign24430_e35829_d_n5, assign24430_e35829_d_b0, assign24430_e35829_d_b1, assign24430_e35829_d_b2, assign24430_e35829_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 != 0.0)) {
        let assign24430_e35825: f64 = (var_tmf1 + var_tmf2);
        let assign24430_e35826: f64 = (0.5 * assign24430_e35825);
        let assign24430_e35827: f64 = (var_nfagat_i + assign24430_e35826);
        (assign24430_e35827, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24430_e35829;
        var_nj0_dn0 = assign24430_e35829_d_n0;
        var_nj0_dn1 = assign24430_e35829_d_n1;
        var_nj0_dn2 = assign24430_e35829_d_n2;
        var_nj0_dn3 = assign24430_e35829_d_n3;
        var_nj0_dn4 = assign24430_e35829_d_n4;
        var_nj0_dn5 = assign24430_e35829_d_n5;
        var_nj0_db0 = assign24430_e35829_d_b0;
        var_nj0_db1 = assign24430_e35829_d_b1;
        var_nj0_db2 = assign24430_e35829_d_b2;
        var_nj0_db3 = assign24430_e35829_d_b3;

        let (assign24440_e35840, assign24440_e35840_d_n0, assign24440_e35840_d_n1, assign24440_e35840_d_n2, assign24440_e35840_d_n3, assign24440_e35840_d_n4, assign24440_e35840_d_n5, assign24440_e35840_d_b0, assign24440_e35840_d_b1, assign24440_e35840_d_b2, assign24440_e35840_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24440_e35840;
        var_nj0_dn0 = assign24440_e35840_d_n0;
        var_nj0_dn1 = assign24440_e35840_d_n1;
        var_nj0_dn2 = assign24440_e35840_d_n2;
        var_nj0_dn3 = assign24440_e35840_d_n3;
        var_nj0_dn4 = assign24440_e35840_d_n4;
        var_nj0_dn5 = assign24440_e35840_d_n5;
        var_nj0_db0 = assign24440_e35840_d_b0;
        var_nj0_db1 = assign24440_e35840_d_b1;
        var_nj0_db2 = assign24440_e35840_d_b2;
        var_nj0_db3 = assign24440_e35840_d_b3;

        let (assign24450_e35851, assign24450_e35851_d_n0, assign24450_e35851_d_n1, assign24450_e35851_d_n2, assign24450_e35851_d_n3, assign24450_e35851_d_n4, assign24450_e35851_d_n5, assign24450_e35851_d_b0, assign24450_e35851_d_b1, assign24450_e35851_d_b2, assign24450_e35851_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 != 0.0)) && (var_guard394 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24450_e35851;
        var_nj1_dn0 = assign24450_e35851_d_n0;
        var_nj1_dn1 = assign24450_e35851_d_n1;
        var_nj1_dn2 = assign24450_e35851_d_n2;
        var_nj1_dn3 = assign24450_e35851_d_n3;
        var_nj1_dn4 = assign24450_e35851_d_n4;
        var_nj1_dn5 = assign24450_e35851_d_n5;
        var_nj1_db0 = assign24450_e35851_d_b0;
        var_nj1_db1 = assign24450_e35851_d_b1;
        var_nj1_db2 = assign24450_e35851_d_b2;
        var_nj1_db3 = assign24450_e35851_d_b3;


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

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
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
        var_guard397_slot: &mut f64,
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
        let mut var_guard397: f64 = *var_guard397_slot;
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

        let (assign24520_e36101,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24520_e36097: f64 = (var_nin * var_nin);
        let assign24520_e36099: f64 = (assign24520_e36097 / var_ndibot_i);
        (assign24520_e36099,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24520_e36101;

        let (assign24530_e36117,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24530_e36110: f64 = (var_nfabot_i / var_phitdinv);
        let assign24530_e36113: f64 = (var_ndibot_i / var_pnn0);
        let assign24530_e36114: f64 = (assign24530_e36113).ln();
        let assign24530_e36115: f64 = (assign24530_e36110 * assign24530_e36114);
        (assign24530_e36115,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24530_e36117;

        let assign24540_e36120: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard397 = assign24540_e36120;

        let (assign24550_e36137, assign24550_e36137_d_n0, assign24550_e36137_d_n1, assign24550_e36137_d_n2, assign24550_e36137_d_n3, assign24550_e36137_d_n4, assign24550_e36137_d_n5, assign24550_e36137_d_b0, assign24550_e36137_d_b1, assign24550_e36137_d_b2, assign24550_e36137_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24550_e36132: f64 = (var_vmax - var_vha1);
        let assign24550_e36133: f64 = (p.p86 * assign24550_e36132);
        let assign24550_e36135: f64 = (assign24550_e36133 + var_nfabot_i);
        (assign24550_e36135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign24550_e36137;
        var_nja10_dn0 = assign24550_e36137_d_n0;
        var_nja10_dn1 = assign24550_e36137_d_n1;
        var_nja10_dn2 = assign24550_e36137_d_n2;
        var_nja10_dn3 = assign24550_e36137_d_n3;
        var_nja10_dn4 = assign24550_e36137_d_n4;
        var_nja10_dn5 = assign24550_e36137_d_n5;
        var_nja10_db0 = assign24550_e36137_d_b0;
        var_nja10_db1 = assign24550_e36137_d_b1;
        var_nja10_db2 = assign24550_e36137_d_b2;
        var_nja10_db3 = assign24550_e36137_d_b3;

        let (assign24560_e36152, assign24560_e36152_d_n0, assign24560_e36152_d_n1, assign24560_e36152_d_n2, assign24560_e36152_d_n3, assign24560_e36152_d_n4, assign24560_e36152_d_n5, assign24560_e36152_d_b0, assign24560_e36152_d_b1, assign24560_e36152_d_b2, assign24560_e36152_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24560_e36149: f64 = (p.p86 * var_vha1);
        let assign24560_e36150: f64 = (var_nfabot_i - assign24560_e36149);
        (assign24560_e36150, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24560_e36152;
        var_nj0_dn0 = assign24560_e36152_d_n0;
        var_nj0_dn1 = assign24560_e36152_d_n1;
        var_nj0_dn2 = assign24560_e36152_d_n2;
        var_nj0_dn3 = assign24560_e36152_d_n3;
        var_nj0_dn4 = assign24560_e36152_d_n4;
        var_nj0_dn5 = assign24560_e36152_d_n5;
        var_nj0_db0 = assign24560_e36152_d_b0;
        var_nj0_db1 = assign24560_e36152_d_b1;
        var_nj0_db2 = assign24560_e36152_d_b2;
        var_nj0_db3 = assign24560_e36152_d_b3;

        let (assign24570_e36167, assign24570_e36167_d_n0, assign24570_e36167_d_n1, assign24570_e36167_d_n2, assign24570_e36167_d_n3, assign24570_e36167_d_n4, assign24570_e36167_d_n5, assign24570_e36167_d_b0, assign24570_e36167_d_b1, assign24570_e36167_d_b2, assign24570_e36167_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24570_e36163: f64 = (p.p85 - var_nja10);
        let assign24570_e36165: f64 = (assign24570_e36163 - 0.01);
        (assign24570_e36165, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24570_e36167;
        var_tmf1_dn0 = assign24570_e36167_d_n0;
        var_tmf1_dn1 = assign24570_e36167_d_n1;
        var_tmf1_dn2 = assign24570_e36167_d_n2;
        var_tmf1_dn3 = assign24570_e36167_d_n3;
        var_tmf1_dn4 = assign24570_e36167_d_n4;
        var_tmf1_dn5 = assign24570_e36167_d_n5;
        var_tmf1_db0 = assign24570_e36167_d_b0;
        var_tmf1_db1 = assign24570_e36167_d_b1;
        var_tmf1_db2 = assign24570_e36167_d_b2;
        var_tmf1_db3 = assign24570_e36167_d_b3;

        let (assign24580_e36182, assign24580_e36182_d_n0, assign24580_e36182_d_n1, assign24580_e36182_d_n2, assign24580_e36182_d_n3, assign24580_e36182_d_n4, assign24580_e36182_d_n5, assign24580_e36182_d_b0, assign24580_e36182_d_b1, assign24580_e36182_d_b2, assign24580_e36182_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24580_e36178: f64 = (4.0 * p.p85);
        let assign24580_e36180: f64 = (assign24580_e36178 * 0.01);
        (assign24580_e36180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24580_e36182;
        var_tmf2_dn0 = assign24580_e36182_d_n0;
        var_tmf2_dn1 = assign24580_e36182_d_n1;
        var_tmf2_dn2 = assign24580_e36182_d_n2;
        var_tmf2_dn3 = assign24580_e36182_d_n3;
        var_tmf2_dn4 = assign24580_e36182_d_n4;
        var_tmf2_dn5 = assign24580_e36182_d_n5;
        var_tmf2_db0 = assign24580_e36182_d_b0;
        var_tmf2_db1 = assign24580_e36182_d_b1;
        var_tmf2_db2 = assign24580_e36182_d_b2;
        var_tmf2_db3 = assign24580_e36182_d_b3;

        let (assign24590_e36199, assign24590_e36199_d_n0, assign24590_e36199_d_n1, assign24590_e36199_d_n2, assign24590_e36199_d_n3, assign24590_e36199_d_n4, assign24590_e36199_d_n5, assign24590_e36199_d_b0, assign24590_e36199_d_b1, assign24590_e36199_d_b2, assign24590_e36199_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n1, assign24590_e36197_d_n2, assign24590_e36197_d_n3, assign24590_e36197_d_n4, assign24590_e36197_d_n5, assign24590_e36197_d_b0, assign24590_e36197_d_b1, assign24590_e36197_d_b2, assign24590_e36197_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24590_e36196: f64 = (-var_tmf2);
                (assign24590_e36196, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24590_e36197, assign24590_e36197_d_n0, assign24590_e36197_d_n1, assign24590_e36197_d_n2, assign24590_e36197_d_n3, assign24590_e36197_d_n4, assign24590_e36197_d_n5, assign24590_e36197_d_b0, assign24590_e36197_d_b1, assign24590_e36197_d_b2, assign24590_e36197_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24590_e36199;
        var_tmf2_dn0 = assign24590_e36199_d_n0;
        var_tmf2_dn1 = assign24590_e36199_d_n1;
        var_tmf2_dn2 = assign24590_e36199_d_n2;
        var_tmf2_dn3 = assign24590_e36199_d_n3;
        var_tmf2_dn4 = assign24590_e36199_d_n4;
        var_tmf2_dn5 = assign24590_e36199_d_n5;
        var_tmf2_db0 = assign24590_e36199_d_b0;
        var_tmf2_db1 = assign24590_e36199_d_b1;
        var_tmf2_db2 = assign24590_e36199_d_b2;
        var_tmf2_db3 = assign24590_e36199_d_b3;

        let (assign24600_e36215, assign24600_e36215_d_n0, assign24600_e36215_d_n1, assign24600_e36215_d_n2, assign24600_e36215_d_n3, assign24600_e36215_d_n4, assign24600_e36215_d_n5, assign24600_e36215_d_b0, assign24600_e36215_d_b1, assign24600_e36215_d_b2, assign24600_e36215_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24600_e36210: f64 = (var_tmf1 * var_tmf1);
        let assign24600_e36212: f64 = (assign24600_e36210 + var_tmf2);
        let assign24600_e36213: f64 = (assign24600_e36212).sqrt();
        (assign24600_e36213, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24600_e36213)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24600_e36213)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24600_e36213)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24600_e36213)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24600_e36213)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24600_e36213)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24600_e36215;
        var_tmf2_dn0 = assign24600_e36215_d_n0;
        var_tmf2_dn1 = assign24600_e36215_d_n1;
        var_tmf2_dn2 = assign24600_e36215_d_n2;
        var_tmf2_dn3 = assign24600_e36215_d_n3;
        var_tmf2_dn4 = assign24600_e36215_d_n4;
        var_tmf2_dn5 = assign24600_e36215_d_n5;
        var_tmf2_db0 = assign24600_e36215_d_b0;
        var_tmf2_db1 = assign24600_e36215_d_b1;
        var_tmf2_db2 = assign24600_e36215_d_b2;
        var_tmf2_db3 = assign24600_e36215_d_b3;

        let (assign24610_e36232, assign24610_e36232_d_n0, assign24610_e36232_d_n1, assign24610_e36232_d_n2, assign24610_e36232_d_n3, assign24610_e36232_d_n4, assign24610_e36232_d_n5, assign24610_e36232_d_b0, assign24610_e36232_d_b1, assign24610_e36232_d_b2, assign24610_e36232_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24610_e36228: f64 = (var_tmf1 / var_tmf2);
        let assign24610_e36229: f64 = (1.0 + assign24610_e36228);
        let assign24610_e36230: f64 = (0.5 * assign24610_e36229);
        (assign24610_e36230, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign24610_e36232;
        var_dfn_su_dn0 = assign24610_e36232_d_n0;
        var_dfn_su_dn1 = assign24610_e36232_d_n1;
        var_dfn_su_dn2 = assign24610_e36232_d_n2;
        var_dfn_su_dn3 = assign24610_e36232_d_n3;
        var_dfn_su_dn4 = assign24610_e36232_d_n4;
        var_dfn_su_dn5 = assign24610_e36232_d_n5;
        var_dfn_su_db0 = assign24610_e36232_d_b0;
        var_dfn_su_db1 = assign24610_e36232_d_b1;
        var_dfn_su_db2 = assign24610_e36232_d_b2;
        var_dfn_su_db3 = assign24610_e36232_d_b3;

        let (assign24620_e36249, assign24620_e36249_d_n0, assign24620_e36249_d_n1, assign24620_e36249_d_n2, assign24620_e36249_d_n3, assign24620_e36249_d_n4, assign24620_e36249_d_n5, assign24620_e36249_d_b0, assign24620_e36249_d_b1, assign24620_e36249_d_b2, assign24620_e36249_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24620_e36245: f64 = (var_tmf1 + var_tmf2);
        let assign24620_e36246: f64 = (0.5 * assign24620_e36245);
        let assign24620_e36247: f64 = (p.p85 - assign24620_e36246);
        (assign24620_e36247, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign24620_e36249;
        var_nja11_dn0 = assign24620_e36249_d_n0;
        var_nja11_dn1 = assign24620_e36249_d_n1;
        var_nja11_dn2 = assign24620_e36249_d_n2;
        var_nja11_dn3 = assign24620_e36249_d_n3;
        var_nja11_dn4 = assign24620_e36249_d_n4;
        var_nja11_dn5 = assign24620_e36249_d_n5;
        var_nja11_db0 = assign24620_e36249_d_b0;
        var_nja11_db1 = assign24620_e36249_d_b1;
        var_nja11_db2 = assign24620_e36249_d_b2;
        var_nja11_db3 = assign24620_e36249_d_b3;

        let (assign24630_e36264, assign24630_e36264_d_n0, assign24630_e36264_d_n1, assign24630_e36264_d_n2, assign24630_e36264_d_n3, assign24630_e36264_d_n4, assign24630_e36264_d_n5, assign24630_e36264_d_b0, assign24630_e36264_d_b1, assign24630_e36264_d_b2, assign24630_e36264_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24630_e36260: f64 = (var_nja11 - var_nfabot_i);
        let assign24630_e36262: f64 = (assign24630_e36260 - 0.01);
        (assign24630_e36262, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24630_e36264;
        var_tmf1_dn0 = assign24630_e36264_d_n0;
        var_tmf1_dn1 = assign24630_e36264_d_n1;
        var_tmf1_dn2 = assign24630_e36264_d_n2;
        var_tmf1_dn3 = assign24630_e36264_d_n3;
        var_tmf1_dn4 = assign24630_e36264_d_n4;
        var_tmf1_dn5 = assign24630_e36264_d_n5;
        var_tmf1_db0 = assign24630_e36264_d_b0;
        var_tmf1_db1 = assign24630_e36264_d_b1;
        var_tmf1_db2 = assign24630_e36264_d_b2;
        var_tmf1_db3 = assign24630_e36264_d_b3;

        let (assign24640_e36279, assign24640_e36279_d_n0, assign24640_e36279_d_n1, assign24640_e36279_d_n2, assign24640_e36279_d_n3, assign24640_e36279_d_n4, assign24640_e36279_d_n5, assign24640_e36279_d_b0, assign24640_e36279_d_b1, assign24640_e36279_d_b2, assign24640_e36279_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24640_e36275: f64 = (4.0 * var_nfabot_i);
        let assign24640_e36277: f64 = (assign24640_e36275 * 0.01);
        (assign24640_e36277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24640_e36279;
        var_tmf2_dn0 = assign24640_e36279_d_n0;
        var_tmf2_dn1 = assign24640_e36279_d_n1;
        var_tmf2_dn2 = assign24640_e36279_d_n2;
        var_tmf2_dn3 = assign24640_e36279_d_n3;
        var_tmf2_dn4 = assign24640_e36279_d_n4;
        var_tmf2_dn5 = assign24640_e36279_d_n5;
        var_tmf2_db0 = assign24640_e36279_d_b0;
        var_tmf2_db1 = assign24640_e36279_d_b1;
        var_tmf2_db2 = assign24640_e36279_d_b2;
        var_tmf2_db3 = assign24640_e36279_d_b3;

        let (assign24650_e36296, assign24650_e36296_d_n0, assign24650_e36296_d_n1, assign24650_e36296_d_n2, assign24650_e36296_d_n3, assign24650_e36296_d_n4, assign24650_e36296_d_n5, assign24650_e36296_d_b0, assign24650_e36296_d_b1, assign24650_e36296_d_b2, assign24650_e36296_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n1, assign24650_e36294_d_n2, assign24650_e36294_d_n3, assign24650_e36294_d_n4, assign24650_e36294_d_n5, assign24650_e36294_d_b0, assign24650_e36294_d_b1, assign24650_e36294_d_b2, assign24650_e36294_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24650_e36293: f64 = (-var_tmf2);
                (assign24650_e36293, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24650_e36294, assign24650_e36294_d_n0, assign24650_e36294_d_n1, assign24650_e36294_d_n2, assign24650_e36294_d_n3, assign24650_e36294_d_n4, assign24650_e36294_d_n5, assign24650_e36294_d_b0, assign24650_e36294_d_b1, assign24650_e36294_d_b2, assign24650_e36294_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24650_e36296;
        var_tmf2_dn0 = assign24650_e36296_d_n0;
        var_tmf2_dn1 = assign24650_e36296_d_n1;
        var_tmf2_dn2 = assign24650_e36296_d_n2;
        var_tmf2_dn3 = assign24650_e36296_d_n3;
        var_tmf2_dn4 = assign24650_e36296_d_n4;
        var_tmf2_dn5 = assign24650_e36296_d_n5;
        var_tmf2_db0 = assign24650_e36296_d_b0;
        var_tmf2_db1 = assign24650_e36296_d_b1;
        var_tmf2_db2 = assign24650_e36296_d_b2;
        var_tmf2_db3 = assign24650_e36296_d_b3;

        let (assign24660_e36312, assign24660_e36312_d_n0, assign24660_e36312_d_n1, assign24660_e36312_d_n2, assign24660_e36312_d_n3, assign24660_e36312_d_n4, assign24660_e36312_d_n5, assign24660_e36312_d_b0, assign24660_e36312_d_b1, assign24660_e36312_d_b2, assign24660_e36312_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24660_e36307: f64 = (var_tmf1 * var_tmf1);
        let assign24660_e36309: f64 = (assign24660_e36307 + var_tmf2);
        let assign24660_e36310: f64 = (assign24660_e36309).sqrt();
        (assign24660_e36310, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24660_e36310)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24660_e36310)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24660_e36310)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24660_e36310)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24660_e36310)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24660_e36310)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24660_e36312;
        var_tmf2_dn0 = assign24660_e36312_d_n0;
        var_tmf2_dn1 = assign24660_e36312_d_n1;
        var_tmf2_dn2 = assign24660_e36312_d_n2;
        var_tmf2_dn3 = assign24660_e36312_d_n3;
        var_tmf2_dn4 = assign24660_e36312_d_n4;
        var_tmf2_dn5 = assign24660_e36312_d_n5;
        var_tmf2_db0 = assign24660_e36312_d_b0;
        var_tmf2_db1 = assign24660_e36312_d_b1;
        var_tmf2_db2 = assign24660_e36312_d_b2;
        var_tmf2_db3 = assign24660_e36312_d_b3;

        let (assign24670_e36329, assign24670_e36329_d_n0, assign24670_e36329_d_n1, assign24670_e36329_d_n2, assign24670_e36329_d_n3, assign24670_e36329_d_n4, assign24670_e36329_d_n5, assign24670_e36329_d_b0, assign24670_e36329_d_b1, assign24670_e36329_d_b2, assign24670_e36329_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24670_e36325: f64 = (var_tmf1 / var_tmf2);
        let assign24670_e36326: f64 = (1.0 + assign24670_e36325);
        let assign24670_e36327: f64 = (0.5 * assign24670_e36326);
        (assign24670_e36327, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign24670_e36329;
        var_dfn_sl_dn0 = assign24670_e36329_d_n0;
        var_dfn_sl_dn1 = assign24670_e36329_d_n1;
        var_dfn_sl_dn2 = assign24670_e36329_d_n2;
        var_dfn_sl_dn3 = assign24670_e36329_d_n3;
        var_dfn_sl_dn4 = assign24670_e36329_d_n4;
        var_dfn_sl_dn5 = assign24670_e36329_d_n5;
        var_dfn_sl_db0 = assign24670_e36329_d_b0;
        var_dfn_sl_db1 = assign24670_e36329_d_b1;
        var_dfn_sl_db2 = assign24670_e36329_d_b2;
        var_dfn_sl_db3 = assign24670_e36329_d_b3;

        let (assign24680_e36346, assign24680_e36346_d_n0, assign24680_e36346_d_n1, assign24680_e36346_d_n2, assign24680_e36346_d_n3, assign24680_e36346_d_n4, assign24680_e36346_d_n5, assign24680_e36346_d_b0, assign24680_e36346_d_b1, assign24680_e36346_d_b2, assign24680_e36346_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24680_e36342: f64 = (var_tmf1 + var_tmf2);
        let assign24680_e36343: f64 = (0.5 * assign24680_e36342);
        let assign24680_e36344: f64 = (var_nfabot_i + assign24680_e36343);
        (assign24680_e36344, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24680_e36346;
        var_nj1_dn0 = assign24680_e36346_d_n0;
        var_nj1_dn1 = assign24680_e36346_d_n1;
        var_nj1_dn2 = assign24680_e36346_d_n2;
        var_nj1_dn3 = assign24680_e36346_d_n3;
        var_nj1_dn4 = assign24680_e36346_d_n4;
        var_nj1_dn5 = assign24680_e36346_d_n5;
        var_nj1_db0 = assign24680_e36346_d_b0;
        var_nj1_db1 = assign24680_e36346_d_b1;
        var_nj1_db2 = assign24680_e36346_d_b2;
        var_nj1_db3 = assign24680_e36346_d_b3;

        let (assign24690_e36361, assign24690_e36361_d_n0, assign24690_e36361_d_n1, assign24690_e36361_d_n2, assign24690_e36361_d_n3, assign24690_e36361_d_n4, assign24690_e36361_d_n5, assign24690_e36361_d_b0, assign24690_e36361_d_b1, assign24690_e36361_d_b2, assign24690_e36361_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24690_e36357: f64 = (p.p85 - var_nj0);
        let assign24690_e36359: f64 = (assign24690_e36357 - 0.01);
        (assign24690_e36359, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24690_e36361;
        var_tmf1_dn0 = assign24690_e36361_d_n0;
        var_tmf1_dn1 = assign24690_e36361_d_n1;
        var_tmf1_dn2 = assign24690_e36361_d_n2;
        var_tmf1_dn3 = assign24690_e36361_d_n3;
        var_tmf1_dn4 = assign24690_e36361_d_n4;
        var_tmf1_dn5 = assign24690_e36361_d_n5;
        var_tmf1_db0 = assign24690_e36361_d_b0;
        var_tmf1_db1 = assign24690_e36361_d_b1;
        var_tmf1_db2 = assign24690_e36361_d_b2;
        var_tmf1_db3 = assign24690_e36361_d_b3;

        let (assign24700_e36376, assign24700_e36376_d_n0, assign24700_e36376_d_n1, assign24700_e36376_d_n2, assign24700_e36376_d_n3, assign24700_e36376_d_n4, assign24700_e36376_d_n5, assign24700_e36376_d_b0, assign24700_e36376_d_b1, assign24700_e36376_d_b2, assign24700_e36376_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24700_e36372: f64 = (4.0 * p.p85);
        let assign24700_e36374: f64 = (assign24700_e36372 * 0.01);
        (assign24700_e36374, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24700_e36376;
        var_tmf2_dn0 = assign24700_e36376_d_n0;
        var_tmf2_dn1 = assign24700_e36376_d_n1;
        var_tmf2_dn2 = assign24700_e36376_d_n2;
        var_tmf2_dn3 = assign24700_e36376_d_n3;
        var_tmf2_dn4 = assign24700_e36376_d_n4;
        var_tmf2_dn5 = assign24700_e36376_d_n5;
        var_tmf2_db0 = assign24700_e36376_d_b0;
        var_tmf2_db1 = assign24700_e36376_d_b1;
        var_tmf2_db2 = assign24700_e36376_d_b2;
        var_tmf2_db3 = assign24700_e36376_d_b3;

        let (assign24710_e36393, assign24710_e36393_d_n0, assign24710_e36393_d_n1, assign24710_e36393_d_n2, assign24710_e36393_d_n3, assign24710_e36393_d_n4, assign24710_e36393_d_n5, assign24710_e36393_d_b0, assign24710_e36393_d_b1, assign24710_e36393_d_b2, assign24710_e36393_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n1, assign24710_e36391_d_n2, assign24710_e36391_d_n3, assign24710_e36391_d_n4, assign24710_e36391_d_n5, assign24710_e36391_d_b0, assign24710_e36391_d_b1, assign24710_e36391_d_b2, assign24710_e36391_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24710_e36390: f64 = (-var_tmf2);
                (assign24710_e36390, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n1, assign24710_e36391_d_n2, assign24710_e36391_d_n3, assign24710_e36391_d_n4, assign24710_e36391_d_n5, assign24710_e36391_d_b0, assign24710_e36391_d_b1, assign24710_e36391_d_b2, assign24710_e36391_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24710_e36393;
        var_tmf2_dn0 = assign24710_e36393_d_n0;
        var_tmf2_dn1 = assign24710_e36393_d_n1;
        var_tmf2_dn2 = assign24710_e36393_d_n2;
        var_tmf2_dn3 = assign24710_e36393_d_n3;
        var_tmf2_dn4 = assign24710_e36393_d_n4;
        var_tmf2_dn5 = assign24710_e36393_d_n5;
        var_tmf2_db0 = assign24710_e36393_d_b0;
        var_tmf2_db1 = assign24710_e36393_d_b1;
        var_tmf2_db2 = assign24710_e36393_d_b2;
        var_tmf2_db3 = assign24710_e36393_d_b3;

        let (assign24720_e36409, assign24720_e36409_d_n0, assign24720_e36409_d_n1, assign24720_e36409_d_n2, assign24720_e36409_d_n3, assign24720_e36409_d_n4, assign24720_e36409_d_n5, assign24720_e36409_d_b0, assign24720_e36409_d_b1, assign24720_e36409_d_b2, assign24720_e36409_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24720_e36404: f64 = (var_tmf1 * var_tmf1);
        let assign24720_e36406: f64 = (assign24720_e36404 + var_tmf2);
        let assign24720_e36407: f64 = (assign24720_e36406).sqrt();
        (assign24720_e36407, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24720_e36407)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24720_e36407)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24720_e36407)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24720_e36407)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24720_e36407)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24720_e36409;
        var_tmf2_dn0 = assign24720_e36409_d_n0;
        var_tmf2_dn1 = assign24720_e36409_d_n1;
        var_tmf2_dn2 = assign24720_e36409_d_n2;
        var_tmf2_dn3 = assign24720_e36409_d_n3;
        var_tmf2_dn4 = assign24720_e36409_d_n4;
        var_tmf2_dn5 = assign24720_e36409_d_n5;
        var_tmf2_db0 = assign24720_e36409_d_b0;
        var_tmf2_db1 = assign24720_e36409_d_b1;
        var_tmf2_db2 = assign24720_e36409_d_b2;
        var_tmf2_db3 = assign24720_e36409_d_b3;

        let (assign24730_e36426, assign24730_e36426_d_n0, assign24730_e36426_d_n1, assign24730_e36426_d_n2, assign24730_e36426_d_n3, assign24730_e36426_d_n4, assign24730_e36426_d_n5, assign24730_e36426_d_b0, assign24730_e36426_d_b1, assign24730_e36426_d_b2, assign24730_e36426_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24730_e36422: f64 = (var_tmf1 + var_tmf2);
        let assign24730_e36423: f64 = (0.5 * assign24730_e36422);
        let assign24730_e36424: f64 = (p.p85 - assign24730_e36423);
        (assign24730_e36424, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24730_e36426;
        var_nj0_dn0 = assign24730_e36426_d_n0;
        var_nj0_dn1 = assign24730_e36426_d_n1;
        var_nj0_dn2 = assign24730_e36426_d_n2;
        var_nj0_dn3 = assign24730_e36426_d_n3;
        var_nj0_dn4 = assign24730_e36426_d_n4;
        var_nj0_dn5 = assign24730_e36426_d_n5;
        var_nj0_db0 = assign24730_e36426_d_b0;
        var_nj0_db1 = assign24730_e36426_d_b1;
        var_nj0_db2 = assign24730_e36426_d_b2;
        var_nj0_db3 = assign24730_e36426_d_b3;

        let (assign24740_e36441, assign24740_e36441_d_n0, assign24740_e36441_d_n1, assign24740_e36441_d_n2, assign24740_e36441_d_n3, assign24740_e36441_d_n4, assign24740_e36441_d_n5, assign24740_e36441_d_b0, assign24740_e36441_d_b1, assign24740_e36441_d_b2, assign24740_e36441_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24740_e36437: f64 = (var_nj0 - var_nfabot_i);
        let assign24740_e36439: f64 = (assign24740_e36437 - 0.01);
        (assign24740_e36439, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24740_e36441;
        var_tmf1_dn0 = assign24740_e36441_d_n0;
        var_tmf1_dn1 = assign24740_e36441_d_n1;
        var_tmf1_dn2 = assign24740_e36441_d_n2;
        var_tmf1_dn3 = assign24740_e36441_d_n3;
        var_tmf1_dn4 = assign24740_e36441_d_n4;
        var_tmf1_dn5 = assign24740_e36441_d_n5;
        var_tmf1_db0 = assign24740_e36441_d_b0;
        var_tmf1_db1 = assign24740_e36441_d_b1;
        var_tmf1_db2 = assign24740_e36441_d_b2;
        var_tmf1_db3 = assign24740_e36441_d_b3;

        let (assign24750_e36456, assign24750_e36456_d_n0, assign24750_e36456_d_n1, assign24750_e36456_d_n2, assign24750_e36456_d_n3, assign24750_e36456_d_n4, assign24750_e36456_d_n5, assign24750_e36456_d_b0, assign24750_e36456_d_b1, assign24750_e36456_d_b2, assign24750_e36456_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24750_e36452: f64 = (4.0 * var_nfabot_i);
        let assign24750_e36454: f64 = (assign24750_e36452 * 0.01);
        (assign24750_e36454, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24750_e36456;
        var_tmf2_dn0 = assign24750_e36456_d_n0;
        var_tmf2_dn1 = assign24750_e36456_d_n1;
        var_tmf2_dn2 = assign24750_e36456_d_n2;
        var_tmf2_dn3 = assign24750_e36456_d_n3;
        var_tmf2_dn4 = assign24750_e36456_d_n4;
        var_tmf2_dn5 = assign24750_e36456_d_n5;
        var_tmf2_db0 = assign24750_e36456_d_b0;
        var_tmf2_db1 = assign24750_e36456_d_b1;
        var_tmf2_db2 = assign24750_e36456_d_b2;
        var_tmf2_db3 = assign24750_e36456_d_b3;


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
        *var_guard397_slot = var_guard397;
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

    pub(super) fn stamp_transient_block_39(
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
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard397: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
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
        var_guard398_slot: &mut f64,
        var_guard399_slot: &mut f64,
        var_guard400_slot: &mut f64,
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
        let mut var_guard398: f64 = *var_guard398_slot;
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
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

        let (assign24760_e36473, assign24760_e36473_d_n0, assign24760_e36473_d_n1, assign24760_e36473_d_n2, assign24760_e36473_d_n3, assign24760_e36473_d_n4, assign24760_e36473_d_n5, assign24760_e36473_d_b0, assign24760_e36473_d_b1, assign24760_e36473_d_b2, assign24760_e36473_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n1, assign24760_e36471_d_n2, assign24760_e36471_d_n3, assign24760_e36471_d_n4, assign24760_e36471_d_n5, assign24760_e36471_d_b0, assign24760_e36471_d_b1, assign24760_e36471_d_b2, assign24760_e36471_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24760_e36470: f64 = (-var_tmf2);
                (assign24760_e36470, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n1, assign24760_e36471_d_n2, assign24760_e36471_d_n3, assign24760_e36471_d_n4, assign24760_e36471_d_n5, assign24760_e36471_d_b0, assign24760_e36471_d_b1, assign24760_e36471_d_b2, assign24760_e36471_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24760_e36473;
        var_tmf2_dn0 = assign24760_e36473_d_n0;
        var_tmf2_dn1 = assign24760_e36473_d_n1;
        var_tmf2_dn2 = assign24760_e36473_d_n2;
        var_tmf2_dn3 = assign24760_e36473_d_n3;
        var_tmf2_dn4 = assign24760_e36473_d_n4;
        var_tmf2_dn5 = assign24760_e36473_d_n5;
        var_tmf2_db0 = assign24760_e36473_d_b0;
        var_tmf2_db1 = assign24760_e36473_d_b1;
        var_tmf2_db2 = assign24760_e36473_d_b2;
        var_tmf2_db3 = assign24760_e36473_d_b3;

        let (assign24770_e36489, assign24770_e36489_d_n0, assign24770_e36489_d_n1, assign24770_e36489_d_n2, assign24770_e36489_d_n3, assign24770_e36489_d_n4, assign24770_e36489_d_n5, assign24770_e36489_d_b0, assign24770_e36489_d_b1, assign24770_e36489_d_b2, assign24770_e36489_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24770_e36484: f64 = (var_tmf1 * var_tmf1);
        let assign24770_e36486: f64 = (assign24770_e36484 + var_tmf2);
        let assign24770_e36487: f64 = (assign24770_e36486).sqrt();
        (assign24770_e36487, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24770_e36487)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24770_e36487)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24770_e36487)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24770_e36487)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24770_e36487)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24770_e36489;
        var_tmf2_dn0 = assign24770_e36489_d_n0;
        var_tmf2_dn1 = assign24770_e36489_d_n1;
        var_tmf2_dn2 = assign24770_e36489_d_n2;
        var_tmf2_dn3 = assign24770_e36489_d_n3;
        var_tmf2_dn4 = assign24770_e36489_d_n4;
        var_tmf2_dn5 = assign24770_e36489_d_n5;
        var_tmf2_db0 = assign24770_e36489_d_b0;
        var_tmf2_db1 = assign24770_e36489_d_b1;
        var_tmf2_db2 = assign24770_e36489_d_b2;
        var_tmf2_db3 = assign24770_e36489_d_b3;

        let (assign24780_e36506, assign24780_e36506_d_n0, assign24780_e36506_d_n1, assign24780_e36506_d_n2, assign24780_e36506_d_n3, assign24780_e36506_d_n4, assign24780_e36506_d_n5, assign24780_e36506_d_b0, assign24780_e36506_d_b1, assign24780_e36506_d_b2, assign24780_e36506_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24780_e36502: f64 = (var_tmf1 + var_tmf2);
        let assign24780_e36503: f64 = (0.5 * assign24780_e36502);
        let assign24780_e36504: f64 = (var_nfabot_i + assign24780_e36503);
        (assign24780_e36504, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24780_e36506;
        var_nj0_dn0 = assign24780_e36506_d_n0;
        var_nj0_dn1 = assign24780_e36506_d_n1;
        var_nj0_dn2 = assign24780_e36506_d_n2;
        var_nj0_dn3 = assign24780_e36506_d_n3;
        var_nj0_dn4 = assign24780_e36506_d_n4;
        var_nj0_dn5 = assign24780_e36506_d_n5;
        var_nj0_db0 = assign24780_e36506_d_b0;
        var_nj0_db1 = assign24780_e36506_d_b1;
        var_nj0_db2 = assign24780_e36506_d_b2;
        var_nj0_db3 = assign24780_e36506_d_b3;

        let (assign24790_e36521, assign24790_e36521_d_n0, assign24790_e36521_d_n1, assign24790_e36521_d_n2, assign24790_e36521_d_n3, assign24790_e36521_d_n4, assign24790_e36521_d_n5, assign24790_e36521_d_b0, assign24790_e36521_d_b1, assign24790_e36521_d_b2, assign24790_e36521_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24790_e36517: f64 = (p.p86 * var_dfn_su);
        let assign24790_e36519: f64 = (assign24790_e36517 * var_dfn_sl);
        (assign24790_e36519, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign24790_e36521;
        var_dnj1_dv_dn0 = assign24790_e36521_d_n0;
        var_dnj1_dv_dn1 = assign24790_e36521_d_n1;
        var_dnj1_dv_dn2 = assign24790_e36521_d_n2;
        var_dnj1_dv_dn3 = assign24790_e36521_d_n3;
        var_dnj1_dv_dn4 = assign24790_e36521_d_n4;
        var_dnj1_dv_dn5 = assign24790_e36521_d_n5;
        var_dnj1_dv_db0 = assign24790_e36521_d_b0;
        var_dnj1_dv_db1 = assign24790_e36521_d_b1;
        var_dnj1_dv_db2 = assign24790_e36521_d_b2;
        var_dnj1_dv_db3 = assign24790_e36521_d_b3;

        let (assign24800_e36533, assign24800_e36533_d_n0, assign24800_e36533_d_n1, assign24800_e36533_d_n2, assign24800_e36533_d_n3, assign24800_e36533_d_n4, assign24800_e36533_d_n5, assign24800_e36533_d_b0, assign24800_e36533_d_b1, assign24800_e36533_d_b2, assign24800_e36533_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24800_e36533;
        var_nj0_dn0 = assign24800_e36533_d_n0;
        var_nj0_dn1 = assign24800_e36533_d_n1;
        var_nj0_dn2 = assign24800_e36533_d_n2;
        var_nj0_dn3 = assign24800_e36533_d_n3;
        var_nj0_dn4 = assign24800_e36533_d_n4;
        var_nj0_dn5 = assign24800_e36533_d_n5;
        var_nj0_db0 = assign24800_e36533_d_b0;
        var_nj0_db1 = assign24800_e36533_d_b1;
        var_nj0_db2 = assign24800_e36533_d_b2;
        var_nj0_db3 = assign24800_e36533_d_b3;

        let (assign24810_e36545, assign24810_e36545_d_n0, assign24810_e36545_d_n1, assign24810_e36545_d_n2, assign24810_e36545_d_n3, assign24810_e36545_d_n4, assign24810_e36545_d_n5, assign24810_e36545_d_b0, assign24810_e36545_d_b1, assign24810_e36545_d_b2, assign24810_e36545_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign24810_e36545;
        var_nj1_dn0 = assign24810_e36545_d_n0;
        var_nj1_dn1 = assign24810_e36545_d_n1;
        var_nj1_dn2 = assign24810_e36545_d_n2;
        var_nj1_dn3 = assign24810_e36545_d_n3;
        var_nj1_dn4 = assign24810_e36545_d_n4;
        var_nj1_dn5 = assign24810_e36545_d_n5;
        var_nj1_db0 = assign24810_e36545_d_b0;
        var_nj1_db1 = assign24810_e36545_d_b1;
        var_nj1_db2 = assign24810_e36545_d_b2;
        var_nj1_db3 = assign24810_e36545_d_b3;

        let (assign24820_e36557, assign24820_e36557_d_n0, assign24820_e36557_d_n1, assign24820_e36557_d_n2, assign24820_e36557_d_n3, assign24820_e36557_d_n4, assign24820_e36557_d_n5, assign24820_e36557_d_b0, assign24820_e36557_d_b1, assign24820_e36557_d_b2, assign24820_e36557_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign24820_e36557;
        var_dnj1_dv_dn0 = assign24820_e36557_d_n0;
        var_dnj1_dv_dn1 = assign24820_e36557_d_n1;
        var_dnj1_dv_dn2 = assign24820_e36557_d_n2;
        var_dnj1_dv_dn3 = assign24820_e36557_d_n3;
        var_dnj1_dv_dn4 = assign24820_e36557_d_n4;
        var_dnj1_dv_dn5 = assign24820_e36557_d_n5;
        var_dnj1_dv_db0 = assign24820_e36557_d_b0;
        var_dnj1_dv_db1 = assign24820_e36557_d_b1;
        var_dnj1_dv_db2 = assign24820_e36557_d_b2;
        var_dnj1_dv_db3 = assign24820_e36557_d_b3;

        let assign24830_e36561: f64 = (var_vmax / var_nj1);
        let assign24830_e36565: f64 = (var_nj1 - var_nj0);
        let assign24830_e36566: f64 = (var_vha1 * assign24830_e36565);
        let assign24830_e36569: f64 = (var_nj0 * p.p85);
        let assign24830_e36570: f64 = (assign24830_e36566 / assign24830_e36569);
        let assign24830_e36571: f64 = (assign24830_e36561 + assign24830_e36570);
        let assign24830_e36572: f64 = (var_phitdinv * assign24830_e36571);
        let assign24830_e36573: f64 = (assign24830_e36572).abs();
        let assign24830_e36575: f64 = if assign24830_e36573 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard398 = assign24830_e36575;

        let (assign24840_e36601, assign24840_e36601_d_n0, assign24840_e36601_d_n1, assign24840_e36601_d_n2, assign24840_e36601_d_n3, assign24840_e36601_d_n4, assign24840_e36601_d_n5, assign24840_e36601_d_b0, assign24840_e36601_d_b1, assign24840_e36601_d_b2, assign24840_e36601_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 != 0.0)) {
        let assign24840_e36587: f64 = (var_vmax / var_nj1);
        let assign24840_e36591: f64 = (var_nj1 - var_nj0);
        let assign24840_e36592: f64 = (var_vha1 * assign24840_e36591);
        let assign24840_e36595: f64 = (var_nj0 * p.p85);
        let assign24840_e36596: f64 = (assign24840_e36592 / assign24840_e36595);
        let assign24840_e36597: f64 = (assign24840_e36587 + assign24840_e36596);
        let assign24840_e36598: f64 = (var_phitdinv * assign24840_e36597);
        let assign24840_e36599: f64 = (assign24840_e36598).exp();
        (assign24840_e36599, (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn0 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn1 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn2 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn3 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn4 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn5 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_db0 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_db1 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_db2 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_db3 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24840_e36601;
        var_exp_vmax_over_phitd_bot_dn0 = assign24840_e36601_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign24840_e36601_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign24840_e36601_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign24840_e36601_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign24840_e36601_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign24840_e36601_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign24840_e36601_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign24840_e36601_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign24840_e36601_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign24840_e36601_d_b3;

        let assign24850_e36605: f64 = (var_vmax / var_nj1);
        let assign24850_e36609: f64 = (var_nj1 - var_nj0);
        let assign24850_e36610: f64 = (var_vha1 * assign24850_e36609);
        let assign24850_e36613: f64 = (var_nj0 * p.p85);
        let assign24850_e36614: f64 = (assign24850_e36610 / assign24850_e36613);
        let assign24850_e36615: f64 = (assign24850_e36605 + assign24850_e36614);
        let assign24850_e36616: f64 = (var_phitdinv * assign24850_e36615);
        let assign24850_e36618: f64 = (-230.25850929940458);
        let assign24850_e36619: f64 = if assign24850_e36616 < assign24850_e36618 { 1.0 } else { 0.0 };
        var_guard399 = assign24850_e36619;

        let (assign24860_e36700, assign24860_e36700_d_n0, assign24860_e36700_d_n1, assign24860_e36700_d_n2, assign24860_e36700_d_n3, assign24860_e36700_d_n4, assign24860_e36700_d_n5, assign24860_e36700_d_b0, assign24860_e36700_d_b1, assign24860_e36700_d_b2, assign24860_e36700_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 == 0.0)) && (var_guard399 != 0.0)) {
        let assign24860_e36634: f64 = (-230.25850929940458);
        let assign24860_e36638: f64 = (var_vmax / var_nj1);
        let assign24860_e36642: f64 = (var_nj1 - var_nj0);
        let assign24860_e36643: f64 = (var_vha1 * assign24860_e36642);
        let assign24860_e36646: f64 = (var_nj0 * p.p85);
        let assign24860_e36647: f64 = (assign24860_e36643 / assign24860_e36646);
        let assign24860_e36648: f64 = (assign24860_e36638 + assign24860_e36647);
        let assign24860_e36649: f64 = (var_phitdinv * assign24860_e36648);
        let assign24860_e36650: f64 = (assign24860_e36634 - assign24860_e36649);
        let assign24860_e36654: f64 = (-230.25850929940458);
        let assign24860_e36658: f64 = (var_vmax / var_nj1);
        let assign24860_e36662: f64 = (var_nj1 - var_nj0);
        let assign24860_e36663: f64 = (var_vha1 * assign24860_e36662);
        let assign24860_e36666: f64 = (var_nj0 * p.p85);
        let assign24860_e36667: f64 = (assign24860_e36663 / assign24860_e36666);
        let assign24860_e36668: f64 = (assign24860_e36658 + assign24860_e36667);
        let assign24860_e36669: f64 = (var_phitdinv * assign24860_e36668);
        let assign24860_e36670: f64 = (assign24860_e36654 - assign24860_e36669);
        let assign24860_e36673: f64 = (-230.25850929940458);
        let assign24860_e36677: f64 = (var_vmax / var_nj1);
        let assign24860_e36681: f64 = (var_nj1 - var_nj0);
        let assign24860_e36682: f64 = (var_vha1 * assign24860_e36681);
        let assign24860_e36685: f64 = (var_nj0 * p.p85);
        let assign24860_e36686: f64 = (assign24860_e36682 / assign24860_e36685);
        let assign24860_e36687: f64 = (assign24860_e36677 + assign24860_e36686);
        let assign24860_e36688: f64 = (var_phitdinv * assign24860_e36687);
        let assign24860_e36689: f64 = (assign24860_e36673 - assign24860_e36688);
        let assign24860_e36691: f64 = (assign24860_e36689 * 0.3333333333333333);
        let assign24860_e36692: f64 = (1.0 + assign24860_e36691);
        let assign24860_e36693: f64 = (assign24860_e36670 * assign24860_e36692);
        let assign24860_e36694: f64 = (0.5 * assign24860_e36693);
        let assign24860_e36695: f64 = (1.0 + assign24860_e36694);
        let assign24860_e36696: f64 = (assign24860_e36650 * assign24860_e36695);
        let assign24860_e36697: f64 = (1.0 + assign24860_e36696);
        let assign24860_e36698: f64 = (1e-100 / assign24860_e36697);
        (assign24860_e36698, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn0 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn0 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn0 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn1 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn1 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn1 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn2 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn2 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn2 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn3 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn3 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn3 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn4 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn4 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn4 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn5 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn5 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn5 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_db0 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_db0 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_db0 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_db1 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_db1 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_db1 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_db2 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_db2 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_db2 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_db3 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_db3 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_db3 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24860_e36700;
        var_exp_vmax_over_phitd_bot_dn0 = assign24860_e36700_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign24860_e36700_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign24860_e36700_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign24860_e36700_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign24860_e36700_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign24860_e36700_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign24860_e36700_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign24860_e36700_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign24860_e36700_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign24860_e36700_d_b3;

        let (assign24870_e36779, assign24870_e36779_d_n0, assign24870_e36779_d_n1, assign24870_e36779_d_n2, assign24870_e36779_d_n3, assign24870_e36779_d_n4, assign24870_e36779_d_n5, assign24870_e36779_d_b0, assign24870_e36779_d_b1, assign24870_e36779_d_b2, assign24870_e36779_d_b3,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 == 0.0)) && (var_guard399 == 0.0)) {
        let assign24870_e36718: f64 = (var_vmax / var_nj1);
        let assign24870_e36722: f64 = (var_nj1 - var_nj0);
        let assign24870_e36723: f64 = (var_vha1 * assign24870_e36722);
        let assign24870_e36726: f64 = (var_nj0 * p.p85);
        let assign24870_e36727: f64 = (assign24870_e36723 / assign24870_e36726);
        let assign24870_e36728: f64 = (assign24870_e36718 + assign24870_e36727);
        let assign24870_e36729: f64 = (var_phitdinv * assign24870_e36728);
        let assign24870_e36731: f64 = (assign24870_e36729 - 230.25850929940458);
        let assign24870_e36737: f64 = (var_vmax / var_nj1);
        let assign24870_e36741: f64 = (var_nj1 - var_nj0);
        let assign24870_e36742: f64 = (var_vha1 * assign24870_e36741);
        let assign24870_e36745: f64 = (var_nj0 * p.p85);
        let assign24870_e36746: f64 = (assign24870_e36742 / assign24870_e36745);
        let assign24870_e36747: f64 = (assign24870_e36737 + assign24870_e36746);
        let assign24870_e36748: f64 = (var_phitdinv * assign24870_e36747);
        let assign24870_e36750: f64 = (assign24870_e36748 - 230.25850929940458);
        let assign24870_e36755: f64 = (var_vmax / var_nj1);
        let assign24870_e36759: f64 = (var_nj1 - var_nj0);
        let assign24870_e36760: f64 = (var_vha1 * assign24870_e36759);
        let assign24870_e36763: f64 = (var_nj0 * p.p85);
        let assign24870_e36764: f64 = (assign24870_e36760 / assign24870_e36763);
        let assign24870_e36765: f64 = (assign24870_e36755 + assign24870_e36764);
        let assign24870_e36766: f64 = (var_phitdinv * assign24870_e36765);
        let assign24870_e36768: f64 = (assign24870_e36766 - 230.25850929940458);
        let assign24870_e36770: f64 = (assign24870_e36768 * 0.3333333333333333);
        let assign24870_e36771: f64 = (1.0 + assign24870_e36770);
        let assign24870_e36772: f64 = (assign24870_e36750 * assign24870_e36771);
        let assign24870_e36773: f64 = (0.5 * assign24870_e36772);
        let assign24870_e36774: f64 = (1.0 + assign24870_e36773);
        let assign24870_e36775: f64 = (assign24870_e36731 * assign24870_e36774);
        let assign24870_e36776: f64 = (1.0 + assign24870_e36775);
        let assign24870_e36777: f64 = (1e100 * assign24870_e36776);
        (assign24870_e36777, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn0 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn0 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn0 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn1 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn1 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn1 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn2 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn2 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn2 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn3 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn3 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn3 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn4 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn4 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn4 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn5 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn5 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn5 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_db0 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_db0 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_db0 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_db1 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_db1 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_db1 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_db2 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_db2 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_db2 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_db3 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_db3 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_db3 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24870_e36779;
        var_exp_vmax_over_phitd_bot_dn0 = assign24870_e36779_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign24870_e36779_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign24870_e36779_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign24870_e36779_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign24870_e36779_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign24870_e36779_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign24870_e36779_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign24870_e36779_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign24870_e36779_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign24870_e36779_d_b3;

        let (assign24880_e36806, assign24880_e36806_d_n0, assign24880_e36806_d_n1, assign24880_e36806_d_n2, assign24880_e36806_d_n3, assign24880_e36806_d_n4, assign24880_e36806_d_n5, assign24880_e36806_d_b0, assign24880_e36806_d_b1, assign24880_e36806_d_b2, assign24880_e36806_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24880_e36790: f64 = (var_vmax * var_dnj1_dv);
        let assign24880_e36791: f64 = (var_nj1 - assign24880_e36790);
        let assign24880_e36794: f64 = (var_nj1 * var_nj1);
        let assign24880_e36795: f64 = (assign24880_e36791 / assign24880_e36794);
        let assign24880_e36798: f64 = (var_vha1 * var_dnj1_dv);
        let assign24880_e36801: f64 = (var_nj0 * p.p85);
        let assign24880_e36802: f64 = (assign24880_e36798 / assign24880_e36801);
        let assign24880_e36803: f64 = (assign24880_e36795 + assign24880_e36802);
        let assign24880_e36804: f64 = (var_phitdinv * assign24880_e36803);
        (assign24880_e36804, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn0 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn1 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn2 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn3 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn4 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn5 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_db0) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_db0 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_db1) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_db1 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_db2) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_db2 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_db3) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_db3 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign24880_e36806;
        var_dvmax_over_phitd_dv_dn0 = assign24880_e36806_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign24880_e36806_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign24880_e36806_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign24880_e36806_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign24880_e36806_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign24880_e36806_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign24880_e36806_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign24880_e36806_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign24880_e36806_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign24880_e36806_d_b3;

        let (assign24890_e36823, assign24890_e36823_d_n0, assign24890_e36823_d_n1, assign24890_e36823_d_n2, assign24890_e36823_d_n3, assign24890_e36823_d_n4, assign24890_e36823_d_n5, assign24890_e36823_d_b0, assign24890_e36823_d_b1, assign24890_e36823_d_b2, assign24890_e36823_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24890_e36816: f64 = (var_v5 - var_vmax);
        let assign24890_e36818: f64 = (assign24890_e36816 * var_dvmax_over_phitd_dv);
        let assign24890_e36819: f64 = (1.0 + assign24890_e36818);
        let assign24890_e36821: f64 = (assign24890_e36819 * var_exp_vmax_over_phitd_bot);
        (assign24890_e36821, (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn0)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn1) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn1)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn2)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn3) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn3)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn4) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn4)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn5) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn5)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_db0) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_db0)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_db1) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_db1)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_db2) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_db2)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_db3) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_db3)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign24890_e36823;
        var_idmultbot_dn0 = assign24890_e36823_d_n0;
        var_idmultbot_dn1 = assign24890_e36823_d_n1;
        var_idmultbot_dn2 = assign24890_e36823_d_n2;
        var_idmultbot_dn3 = assign24890_e36823_d_n3;
        var_idmultbot_dn4 = assign24890_e36823_d_n4;
        var_idmultbot_dn5 = assign24890_e36823_d_n5;
        var_idmultbot_db0 = assign24890_e36823_d_b0;
        var_idmultbot_db1 = assign24890_e36823_d_b1;
        var_idmultbot_db2 = assign24890_e36823_d_b2;
        var_idmultbot_db3 = assign24890_e36823_d_b3;

        let (assign24900_e36836,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24900_e36832: f64 = (var_nin * var_nin);
        let assign24900_e36834: f64 = (assign24900_e36832 / var_ndisti_i);
        (assign24900_e36834,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24900_e36836;

        let (assign24910_e36852,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24910_e36845: f64 = (var_nfasti_i / var_phitdinv);
        let assign24910_e36848: f64 = (var_ndisti_i / var_pnn0);
        let assign24910_e36849: f64 = (assign24910_e36848).ln();
        let assign24910_e36850: f64 = (assign24910_e36845 * assign24910_e36849);
        (assign24910_e36850,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24910_e36852;

        let assign24920_e36855: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard400 = assign24920_e36855;

        let (assign24930_e36872, assign24930_e36872_d_n0, assign24930_e36872_d_n1, assign24930_e36872_d_n2, assign24930_e36872_d_n3, assign24930_e36872_d_n4, assign24930_e36872_d_n5, assign24930_e36872_d_b0, assign24930_e36872_d_b1, assign24930_e36872_d_b2, assign24930_e36872_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24930_e36867: f64 = (var_vmax - var_vha1);
        let assign24930_e36868: f64 = (p.p86 * assign24930_e36867);
        let assign24930_e36870: f64 = (assign24930_e36868 + var_nfasti_i);
        (assign24930_e36870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign24930_e36872;
        var_nja10_dn0 = assign24930_e36872_d_n0;
        var_nja10_dn1 = assign24930_e36872_d_n1;
        var_nja10_dn2 = assign24930_e36872_d_n2;
        var_nja10_dn3 = assign24930_e36872_d_n3;
        var_nja10_dn4 = assign24930_e36872_d_n4;
        var_nja10_dn5 = assign24930_e36872_d_n5;
        var_nja10_db0 = assign24930_e36872_d_b0;
        var_nja10_db1 = assign24930_e36872_d_b1;
        var_nja10_db2 = assign24930_e36872_d_b2;
        var_nja10_db3 = assign24930_e36872_d_b3;

        let (assign24940_e36887, assign24940_e36887_d_n0, assign24940_e36887_d_n1, assign24940_e36887_d_n2, assign24940_e36887_d_n3, assign24940_e36887_d_n4, assign24940_e36887_d_n5, assign24940_e36887_d_b0, assign24940_e36887_d_b1, assign24940_e36887_d_b2, assign24940_e36887_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24940_e36884: f64 = (p.p86 * var_vha1);
        let assign24940_e36885: f64 = (var_nfasti_i - assign24940_e36884);
        (assign24940_e36885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign24940_e36887;
        var_nj0_dn0 = assign24940_e36887_d_n0;
        var_nj0_dn1 = assign24940_e36887_d_n1;
        var_nj0_dn2 = assign24940_e36887_d_n2;
        var_nj0_dn3 = assign24940_e36887_d_n3;
        var_nj0_dn4 = assign24940_e36887_d_n4;
        var_nj0_dn5 = assign24940_e36887_d_n5;
        var_nj0_db0 = assign24940_e36887_d_b0;
        var_nj0_db1 = assign24940_e36887_d_b1;
        var_nj0_db2 = assign24940_e36887_d_b2;
        var_nj0_db3 = assign24940_e36887_d_b3;

        let (assign24950_e36902, assign24950_e36902_d_n0, assign24950_e36902_d_n1, assign24950_e36902_d_n2, assign24950_e36902_d_n3, assign24950_e36902_d_n4, assign24950_e36902_d_n5, assign24950_e36902_d_b0, assign24950_e36902_d_b1, assign24950_e36902_d_b2, assign24950_e36902_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24950_e36898: f64 = (p.p85 - var_nja10);
        let assign24950_e36900: f64 = (assign24950_e36898 - 0.01);
        (assign24950_e36900, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign24950_e36902;
        var_tmf1_dn0 = assign24950_e36902_d_n0;
        var_tmf1_dn1 = assign24950_e36902_d_n1;
        var_tmf1_dn2 = assign24950_e36902_d_n2;
        var_tmf1_dn3 = assign24950_e36902_d_n3;
        var_tmf1_dn4 = assign24950_e36902_d_n4;
        var_tmf1_dn5 = assign24950_e36902_d_n5;
        var_tmf1_db0 = assign24950_e36902_d_b0;
        var_tmf1_db1 = assign24950_e36902_d_b1;
        var_tmf1_db2 = assign24950_e36902_d_b2;
        var_tmf1_db3 = assign24950_e36902_d_b3;

        let (assign24960_e36917, assign24960_e36917_d_n0, assign24960_e36917_d_n1, assign24960_e36917_d_n2, assign24960_e36917_d_n3, assign24960_e36917_d_n4, assign24960_e36917_d_n5, assign24960_e36917_d_b0, assign24960_e36917_d_b1, assign24960_e36917_d_b2, assign24960_e36917_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24960_e36913: f64 = (4.0 * p.p85);
        let assign24960_e36915: f64 = (assign24960_e36913 * 0.01);
        (assign24960_e36915, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24960_e36917;
        var_tmf2_dn0 = assign24960_e36917_d_n0;
        var_tmf2_dn1 = assign24960_e36917_d_n1;
        var_tmf2_dn2 = assign24960_e36917_d_n2;
        var_tmf2_dn3 = assign24960_e36917_d_n3;
        var_tmf2_dn4 = assign24960_e36917_d_n4;
        var_tmf2_dn5 = assign24960_e36917_d_n5;
        var_tmf2_db0 = assign24960_e36917_d_b0;
        var_tmf2_db1 = assign24960_e36917_d_b1;
        var_tmf2_db2 = assign24960_e36917_d_b2;
        var_tmf2_db3 = assign24960_e36917_d_b3;

        let (assign24970_e36934, assign24970_e36934_d_n0, assign24970_e36934_d_n1, assign24970_e36934_d_n2, assign24970_e36934_d_n3, assign24970_e36934_d_n4, assign24970_e36934_d_n5, assign24970_e36934_d_b0, assign24970_e36934_d_b1, assign24970_e36934_d_b2, assign24970_e36934_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n1, assign24970_e36932_d_n2, assign24970_e36932_d_n3, assign24970_e36932_d_n4, assign24970_e36932_d_n5, assign24970_e36932_d_b0, assign24970_e36932_d_b1, assign24970_e36932_d_b2, assign24970_e36932_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign24970_e36931: f64 = (-var_tmf2);
                (assign24970_e36931, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n1, assign24970_e36932_d_n2, assign24970_e36932_d_n3, assign24970_e36932_d_n4, assign24970_e36932_d_n5, assign24970_e36932_d_b0, assign24970_e36932_d_b1, assign24970_e36932_d_b2, assign24970_e36932_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24970_e36934;
        var_tmf2_dn0 = assign24970_e36934_d_n0;
        var_tmf2_dn1 = assign24970_e36934_d_n1;
        var_tmf2_dn2 = assign24970_e36934_d_n2;
        var_tmf2_dn3 = assign24970_e36934_d_n3;
        var_tmf2_dn4 = assign24970_e36934_d_n4;
        var_tmf2_dn5 = assign24970_e36934_d_n5;
        var_tmf2_db0 = assign24970_e36934_d_b0;
        var_tmf2_db1 = assign24970_e36934_d_b1;
        var_tmf2_db2 = assign24970_e36934_d_b2;
        var_tmf2_db3 = assign24970_e36934_d_b3;


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
        *var_guard398_slot = var_guard398;
        *var_guard399_slot = var_guard399;
        *var_guard400_slot = var_guard400;
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

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard400: f64,
        var_nfasti_i: f64,
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

        let (assign24980_e36950, assign24980_e36950_d_n0, assign24980_e36950_d_n1, assign24980_e36950_d_n2, assign24980_e36950_d_n3, assign24980_e36950_d_n4, assign24980_e36950_d_n5, assign24980_e36950_d_b0, assign24980_e36950_d_b1, assign24980_e36950_d_b2, assign24980_e36950_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24980_e36945: f64 = (var_tmf1 * var_tmf1);
        let assign24980_e36947: f64 = (assign24980_e36945 + var_tmf2);
        let assign24980_e36948: f64 = (assign24980_e36947).sqrt();
        (assign24980_e36948, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign24980_e36948)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign24980_e36948)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign24980_e36948)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign24980_e36948)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign24980_e36948)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign24980_e36950;
        var_tmf2_dn0 = assign24980_e36950_d_n0;
        var_tmf2_dn1 = assign24980_e36950_d_n1;
        var_tmf2_dn2 = assign24980_e36950_d_n2;
        var_tmf2_dn3 = assign24980_e36950_d_n3;
        var_tmf2_dn4 = assign24980_e36950_d_n4;
        var_tmf2_dn5 = assign24980_e36950_d_n5;
        var_tmf2_db0 = assign24980_e36950_d_b0;
        var_tmf2_db1 = assign24980_e36950_d_b1;
        var_tmf2_db2 = assign24980_e36950_d_b2;
        var_tmf2_db3 = assign24980_e36950_d_b3;

        let (assign24990_e36967, assign24990_e36967_d_n0, assign24990_e36967_d_n1, assign24990_e36967_d_n2, assign24990_e36967_d_n3, assign24990_e36967_d_n4, assign24990_e36967_d_n5, assign24990_e36967_d_b0, assign24990_e36967_d_b1, assign24990_e36967_d_b2, assign24990_e36967_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24990_e36963: f64 = (var_tmf1 / var_tmf2);
        let assign24990_e36964: f64 = (1.0 + assign24990_e36963);
        let assign24990_e36965: f64 = (0.5 * assign24990_e36964);
        (assign24990_e36965, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign24990_e36967;
        var_dfn_su_dn0 = assign24990_e36967_d_n0;
        var_dfn_su_dn1 = assign24990_e36967_d_n1;
        var_dfn_su_dn2 = assign24990_e36967_d_n2;
        var_dfn_su_dn3 = assign24990_e36967_d_n3;
        var_dfn_su_dn4 = assign24990_e36967_d_n4;
        var_dfn_su_dn5 = assign24990_e36967_d_n5;
        var_dfn_su_db0 = assign24990_e36967_d_b0;
        var_dfn_su_db1 = assign24990_e36967_d_b1;
        var_dfn_su_db2 = assign24990_e36967_d_b2;
        var_dfn_su_db3 = assign24990_e36967_d_b3;

        let (assign25000_e36984, assign25000_e36984_d_n0, assign25000_e36984_d_n1, assign25000_e36984_d_n2, assign25000_e36984_d_n3, assign25000_e36984_d_n4, assign25000_e36984_d_n5, assign25000_e36984_d_b0, assign25000_e36984_d_b1, assign25000_e36984_d_b2, assign25000_e36984_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25000_e36980: f64 = (var_tmf1 + var_tmf2);
        let assign25000_e36981: f64 = (0.5 * assign25000_e36980);
        let assign25000_e36982: f64 = (p.p85 - assign25000_e36981);
        (assign25000_e36982, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign25000_e36984;
        var_nja11_dn0 = assign25000_e36984_d_n0;
        var_nja11_dn1 = assign25000_e36984_d_n1;
        var_nja11_dn2 = assign25000_e36984_d_n2;
        var_nja11_dn3 = assign25000_e36984_d_n3;
        var_nja11_dn4 = assign25000_e36984_d_n4;
        var_nja11_dn5 = assign25000_e36984_d_n5;
        var_nja11_db0 = assign25000_e36984_d_b0;
        var_nja11_db1 = assign25000_e36984_d_b1;
        var_nja11_db2 = assign25000_e36984_d_b2;
        var_nja11_db3 = assign25000_e36984_d_b3;

        let (assign25010_e36999, assign25010_e36999_d_n0, assign25010_e36999_d_n1, assign25010_e36999_d_n2, assign25010_e36999_d_n3, assign25010_e36999_d_n4, assign25010_e36999_d_n5, assign25010_e36999_d_b0, assign25010_e36999_d_b1, assign25010_e36999_d_b2, assign25010_e36999_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25010_e36995: f64 = (var_nja11 - var_nfasti_i);
        let assign25010_e36997: f64 = (assign25010_e36995 - 0.01);
        (assign25010_e36997, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25010_e36999;
        var_tmf1_dn0 = assign25010_e36999_d_n0;
        var_tmf1_dn1 = assign25010_e36999_d_n1;
        var_tmf1_dn2 = assign25010_e36999_d_n2;
        var_tmf1_dn3 = assign25010_e36999_d_n3;
        var_tmf1_dn4 = assign25010_e36999_d_n4;
        var_tmf1_dn5 = assign25010_e36999_d_n5;
        var_tmf1_db0 = assign25010_e36999_d_b0;
        var_tmf1_db1 = assign25010_e36999_d_b1;
        var_tmf1_db2 = assign25010_e36999_d_b2;
        var_tmf1_db3 = assign25010_e36999_d_b3;

        let (assign25020_e37014, assign25020_e37014_d_n0, assign25020_e37014_d_n1, assign25020_e37014_d_n2, assign25020_e37014_d_n3, assign25020_e37014_d_n4, assign25020_e37014_d_n5, assign25020_e37014_d_b0, assign25020_e37014_d_b1, assign25020_e37014_d_b2, assign25020_e37014_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25020_e37010: f64 = (4.0 * var_nfasti_i);
        let assign25020_e37012: f64 = (assign25020_e37010 * 0.01);
        (assign25020_e37012, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25020_e37014;
        var_tmf2_dn0 = assign25020_e37014_d_n0;
        var_tmf2_dn1 = assign25020_e37014_d_n1;
        var_tmf2_dn2 = assign25020_e37014_d_n2;
        var_tmf2_dn3 = assign25020_e37014_d_n3;
        var_tmf2_dn4 = assign25020_e37014_d_n4;
        var_tmf2_dn5 = assign25020_e37014_d_n5;
        var_tmf2_db0 = assign25020_e37014_d_b0;
        var_tmf2_db1 = assign25020_e37014_d_b1;
        var_tmf2_db2 = assign25020_e37014_d_b2;
        var_tmf2_db3 = assign25020_e37014_d_b3;

        let (assign25030_e37031, assign25030_e37031_d_n0, assign25030_e37031_d_n1, assign25030_e37031_d_n2, assign25030_e37031_d_n3, assign25030_e37031_d_n4, assign25030_e37031_d_n5, assign25030_e37031_d_b0, assign25030_e37031_d_b1, assign25030_e37031_d_b2, assign25030_e37031_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n1, assign25030_e37029_d_n2, assign25030_e37029_d_n3, assign25030_e37029_d_n4, assign25030_e37029_d_n5, assign25030_e37029_d_b0, assign25030_e37029_d_b1, assign25030_e37029_d_b2, assign25030_e37029_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25030_e37028: f64 = (-var_tmf2);
                (assign25030_e37028, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n1, assign25030_e37029_d_n2, assign25030_e37029_d_n3, assign25030_e37029_d_n4, assign25030_e37029_d_n5, assign25030_e37029_d_b0, assign25030_e37029_d_b1, assign25030_e37029_d_b2, assign25030_e37029_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25030_e37031;
        var_tmf2_dn0 = assign25030_e37031_d_n0;
        var_tmf2_dn1 = assign25030_e37031_d_n1;
        var_tmf2_dn2 = assign25030_e37031_d_n2;
        var_tmf2_dn3 = assign25030_e37031_d_n3;
        var_tmf2_dn4 = assign25030_e37031_d_n4;
        var_tmf2_dn5 = assign25030_e37031_d_n5;
        var_tmf2_db0 = assign25030_e37031_d_b0;
        var_tmf2_db1 = assign25030_e37031_d_b1;
        var_tmf2_db2 = assign25030_e37031_d_b2;
        var_tmf2_db3 = assign25030_e37031_d_b3;

        let (assign25040_e37047, assign25040_e37047_d_n0, assign25040_e37047_d_n1, assign25040_e37047_d_n2, assign25040_e37047_d_n3, assign25040_e37047_d_n4, assign25040_e37047_d_n5, assign25040_e37047_d_b0, assign25040_e37047_d_b1, assign25040_e37047_d_b2, assign25040_e37047_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25040_e37042: f64 = (var_tmf1 * var_tmf1);
        let assign25040_e37044: f64 = (assign25040_e37042 + var_tmf2);
        let assign25040_e37045: f64 = (assign25040_e37044).sqrt();
        (assign25040_e37045, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25040_e37045)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25040_e37045)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25040_e37045)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25040_e37045)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25040_e37045)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25040_e37047;
        var_tmf2_dn0 = assign25040_e37047_d_n0;
        var_tmf2_dn1 = assign25040_e37047_d_n1;
        var_tmf2_dn2 = assign25040_e37047_d_n2;
        var_tmf2_dn3 = assign25040_e37047_d_n3;
        var_tmf2_dn4 = assign25040_e37047_d_n4;
        var_tmf2_dn5 = assign25040_e37047_d_n5;
        var_tmf2_db0 = assign25040_e37047_d_b0;
        var_tmf2_db1 = assign25040_e37047_d_b1;
        var_tmf2_db2 = assign25040_e37047_d_b2;
        var_tmf2_db3 = assign25040_e37047_d_b3;

        let (assign25050_e37064, assign25050_e37064_d_n0, assign25050_e37064_d_n1, assign25050_e37064_d_n2, assign25050_e37064_d_n3, assign25050_e37064_d_n4, assign25050_e37064_d_n5, assign25050_e37064_d_b0, assign25050_e37064_d_b1, assign25050_e37064_d_b2, assign25050_e37064_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25050_e37060: f64 = (var_tmf1 / var_tmf2);
        let assign25050_e37061: f64 = (1.0 + assign25050_e37060);
        let assign25050_e37062: f64 = (0.5 * assign25050_e37061);
        (assign25050_e37062, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign25050_e37064;
        var_dfn_sl_dn0 = assign25050_e37064_d_n0;
        var_dfn_sl_dn1 = assign25050_e37064_d_n1;
        var_dfn_sl_dn2 = assign25050_e37064_d_n2;
        var_dfn_sl_dn3 = assign25050_e37064_d_n3;
        var_dfn_sl_dn4 = assign25050_e37064_d_n4;
        var_dfn_sl_dn5 = assign25050_e37064_d_n5;
        var_dfn_sl_db0 = assign25050_e37064_d_b0;
        var_dfn_sl_db1 = assign25050_e37064_d_b1;
        var_dfn_sl_db2 = assign25050_e37064_d_b2;
        var_dfn_sl_db3 = assign25050_e37064_d_b3;

        let (assign25060_e37081, assign25060_e37081_d_n0, assign25060_e37081_d_n1, assign25060_e37081_d_n2, assign25060_e37081_d_n3, assign25060_e37081_d_n4, assign25060_e37081_d_n5, assign25060_e37081_d_b0, assign25060_e37081_d_b1, assign25060_e37081_d_b2, assign25060_e37081_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25060_e37077: f64 = (var_tmf1 + var_tmf2);
        let assign25060_e37078: f64 = (0.5 * assign25060_e37077);
        let assign25060_e37079: f64 = (var_nfasti_i + assign25060_e37078);
        (assign25060_e37079, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign25060_e37081;
        var_nj1_dn0 = assign25060_e37081_d_n0;
        var_nj1_dn1 = assign25060_e37081_d_n1;
        var_nj1_dn2 = assign25060_e37081_d_n2;
        var_nj1_dn3 = assign25060_e37081_d_n3;
        var_nj1_dn4 = assign25060_e37081_d_n4;
        var_nj1_dn5 = assign25060_e37081_d_n5;
        var_nj1_db0 = assign25060_e37081_d_b0;
        var_nj1_db1 = assign25060_e37081_d_b1;
        var_nj1_db2 = assign25060_e37081_d_b2;
        var_nj1_db3 = assign25060_e37081_d_b3;

        let (assign25070_e37096, assign25070_e37096_d_n0, assign25070_e37096_d_n1, assign25070_e37096_d_n2, assign25070_e37096_d_n3, assign25070_e37096_d_n4, assign25070_e37096_d_n5, assign25070_e37096_d_b0, assign25070_e37096_d_b1, assign25070_e37096_d_b2, assign25070_e37096_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25070_e37092: f64 = (p.p85 - var_nj0);
        let assign25070_e37094: f64 = (assign25070_e37092 - 0.01);
        (assign25070_e37094, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25070_e37096;
        var_tmf1_dn0 = assign25070_e37096_d_n0;
        var_tmf1_dn1 = assign25070_e37096_d_n1;
        var_tmf1_dn2 = assign25070_e37096_d_n2;
        var_tmf1_dn3 = assign25070_e37096_d_n3;
        var_tmf1_dn4 = assign25070_e37096_d_n4;
        var_tmf1_dn5 = assign25070_e37096_d_n5;
        var_tmf1_db0 = assign25070_e37096_d_b0;
        var_tmf1_db1 = assign25070_e37096_d_b1;
        var_tmf1_db2 = assign25070_e37096_d_b2;
        var_tmf1_db3 = assign25070_e37096_d_b3;

        let (assign25080_e37111, assign25080_e37111_d_n0, assign25080_e37111_d_n1, assign25080_e37111_d_n2, assign25080_e37111_d_n3, assign25080_e37111_d_n4, assign25080_e37111_d_n5, assign25080_e37111_d_b0, assign25080_e37111_d_b1, assign25080_e37111_d_b2, assign25080_e37111_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25080_e37107: f64 = (4.0 * p.p85);
        let assign25080_e37109: f64 = (assign25080_e37107 * 0.01);
        (assign25080_e37109, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25080_e37111;
        var_tmf2_dn0 = assign25080_e37111_d_n0;
        var_tmf2_dn1 = assign25080_e37111_d_n1;
        var_tmf2_dn2 = assign25080_e37111_d_n2;
        var_tmf2_dn3 = assign25080_e37111_d_n3;
        var_tmf2_dn4 = assign25080_e37111_d_n4;
        var_tmf2_dn5 = assign25080_e37111_d_n5;
        var_tmf2_db0 = assign25080_e37111_d_b0;
        var_tmf2_db1 = assign25080_e37111_d_b1;
        var_tmf2_db2 = assign25080_e37111_d_b2;
        var_tmf2_db3 = assign25080_e37111_d_b3;

        let (assign25090_e37128, assign25090_e37128_d_n0, assign25090_e37128_d_n1, assign25090_e37128_d_n2, assign25090_e37128_d_n3, assign25090_e37128_d_n4, assign25090_e37128_d_n5, assign25090_e37128_d_b0, assign25090_e37128_d_b1, assign25090_e37128_d_b2, assign25090_e37128_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n1, assign25090_e37126_d_n2, assign25090_e37126_d_n3, assign25090_e37126_d_n4, assign25090_e37126_d_n5, assign25090_e37126_d_b0, assign25090_e37126_d_b1, assign25090_e37126_d_b2, assign25090_e37126_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25090_e37125: f64 = (-var_tmf2);
                (assign25090_e37125, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n1, assign25090_e37126_d_n2, assign25090_e37126_d_n3, assign25090_e37126_d_n4, assign25090_e37126_d_n5, assign25090_e37126_d_b0, assign25090_e37126_d_b1, assign25090_e37126_d_b2, assign25090_e37126_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25090_e37128;
        var_tmf2_dn0 = assign25090_e37128_d_n0;
        var_tmf2_dn1 = assign25090_e37128_d_n1;
        var_tmf2_dn2 = assign25090_e37128_d_n2;
        var_tmf2_dn3 = assign25090_e37128_d_n3;
        var_tmf2_dn4 = assign25090_e37128_d_n4;
        var_tmf2_dn5 = assign25090_e37128_d_n5;
        var_tmf2_db0 = assign25090_e37128_d_b0;
        var_tmf2_db1 = assign25090_e37128_d_b1;
        var_tmf2_db2 = assign25090_e37128_d_b2;
        var_tmf2_db3 = assign25090_e37128_d_b3;

        let (assign25100_e37144, assign25100_e37144_d_n0, assign25100_e37144_d_n1, assign25100_e37144_d_n2, assign25100_e37144_d_n3, assign25100_e37144_d_n4, assign25100_e37144_d_n5, assign25100_e37144_d_b0, assign25100_e37144_d_b1, assign25100_e37144_d_b2, assign25100_e37144_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25100_e37139: f64 = (var_tmf1 * var_tmf1);
        let assign25100_e37141: f64 = (assign25100_e37139 + var_tmf2);
        let assign25100_e37142: f64 = (assign25100_e37141).sqrt();
        (assign25100_e37142, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25100_e37142)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25100_e37142)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25100_e37142)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25100_e37142)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25100_e37142)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25100_e37144;
        var_tmf2_dn0 = assign25100_e37144_d_n0;
        var_tmf2_dn1 = assign25100_e37144_d_n1;
        var_tmf2_dn2 = assign25100_e37144_d_n2;
        var_tmf2_dn3 = assign25100_e37144_d_n3;
        var_tmf2_dn4 = assign25100_e37144_d_n4;
        var_tmf2_dn5 = assign25100_e37144_d_n5;
        var_tmf2_db0 = assign25100_e37144_d_b0;
        var_tmf2_db1 = assign25100_e37144_d_b1;
        var_tmf2_db2 = assign25100_e37144_d_b2;
        var_tmf2_db3 = assign25100_e37144_d_b3;

        let (assign25110_e37161, assign25110_e37161_d_n0, assign25110_e37161_d_n1, assign25110_e37161_d_n2, assign25110_e37161_d_n3, assign25110_e37161_d_n4, assign25110_e37161_d_n5, assign25110_e37161_d_b0, assign25110_e37161_d_b1, assign25110_e37161_d_b2, assign25110_e37161_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25110_e37157: f64 = (var_tmf1 + var_tmf2);
        let assign25110_e37158: f64 = (0.5 * assign25110_e37157);
        let assign25110_e37159: f64 = (p.p85 - assign25110_e37158);
        (assign25110_e37159, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25110_e37161;
        var_nj0_dn0 = assign25110_e37161_d_n0;
        var_nj0_dn1 = assign25110_e37161_d_n1;
        var_nj0_dn2 = assign25110_e37161_d_n2;
        var_nj0_dn3 = assign25110_e37161_d_n3;
        var_nj0_dn4 = assign25110_e37161_d_n4;
        var_nj0_dn5 = assign25110_e37161_d_n5;
        var_nj0_db0 = assign25110_e37161_d_b0;
        var_nj0_db1 = assign25110_e37161_d_b1;
        var_nj0_db2 = assign25110_e37161_d_b2;
        var_nj0_db3 = assign25110_e37161_d_b3;

        let (assign25120_e37176, assign25120_e37176_d_n0, assign25120_e37176_d_n1, assign25120_e37176_d_n2, assign25120_e37176_d_n3, assign25120_e37176_d_n4, assign25120_e37176_d_n5, assign25120_e37176_d_b0, assign25120_e37176_d_b1, assign25120_e37176_d_b2, assign25120_e37176_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25120_e37172: f64 = (var_nj0 - var_nfasti_i);
        let assign25120_e37174: f64 = (assign25120_e37172 - 0.01);
        (assign25120_e37174, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25120_e37176;
        var_tmf1_dn0 = assign25120_e37176_d_n0;
        var_tmf1_dn1 = assign25120_e37176_d_n1;
        var_tmf1_dn2 = assign25120_e37176_d_n2;
        var_tmf1_dn3 = assign25120_e37176_d_n3;
        var_tmf1_dn4 = assign25120_e37176_d_n4;
        var_tmf1_dn5 = assign25120_e37176_d_n5;
        var_tmf1_db0 = assign25120_e37176_d_b0;
        var_tmf1_db1 = assign25120_e37176_d_b1;
        var_tmf1_db2 = assign25120_e37176_d_b2;
        var_tmf1_db3 = assign25120_e37176_d_b3;

        let (assign25130_e37191, assign25130_e37191_d_n0, assign25130_e37191_d_n1, assign25130_e37191_d_n2, assign25130_e37191_d_n3, assign25130_e37191_d_n4, assign25130_e37191_d_n5, assign25130_e37191_d_b0, assign25130_e37191_d_b1, assign25130_e37191_d_b2, assign25130_e37191_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25130_e37187: f64 = (4.0 * var_nfasti_i);
        let assign25130_e37189: f64 = (assign25130_e37187 * 0.01);
        (assign25130_e37189, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25130_e37191;
        var_tmf2_dn0 = assign25130_e37191_d_n0;
        var_tmf2_dn1 = assign25130_e37191_d_n1;
        var_tmf2_dn2 = assign25130_e37191_d_n2;
        var_tmf2_dn3 = assign25130_e37191_d_n3;
        var_tmf2_dn4 = assign25130_e37191_d_n4;
        var_tmf2_dn5 = assign25130_e37191_d_n5;
        var_tmf2_db0 = assign25130_e37191_d_b0;
        var_tmf2_db1 = assign25130_e37191_d_b1;
        var_tmf2_db2 = assign25130_e37191_d_b2;
        var_tmf2_db3 = assign25130_e37191_d_b3;

        let (assign25140_e37208, assign25140_e37208_d_n0, assign25140_e37208_d_n1, assign25140_e37208_d_n2, assign25140_e37208_d_n3, assign25140_e37208_d_n4, assign25140_e37208_d_n5, assign25140_e37208_d_b0, assign25140_e37208_d_b1, assign25140_e37208_d_b2, assign25140_e37208_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n1, assign25140_e37206_d_n2, assign25140_e37206_d_n3, assign25140_e37206_d_n4, assign25140_e37206_d_n5, assign25140_e37206_d_b0, assign25140_e37206_d_b1, assign25140_e37206_d_b2, assign25140_e37206_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25140_e37205: f64 = (-var_tmf2);
                (assign25140_e37205, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n1, assign25140_e37206_d_n2, assign25140_e37206_d_n3, assign25140_e37206_d_n4, assign25140_e37206_d_n5, assign25140_e37206_d_b0, assign25140_e37206_d_b1, assign25140_e37206_d_b2, assign25140_e37206_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25140_e37208;
        var_tmf2_dn0 = assign25140_e37208_d_n0;
        var_tmf2_dn1 = assign25140_e37208_d_n1;
        var_tmf2_dn2 = assign25140_e37208_d_n2;
        var_tmf2_dn3 = assign25140_e37208_d_n3;
        var_tmf2_dn4 = assign25140_e37208_d_n4;
        var_tmf2_dn5 = assign25140_e37208_d_n5;
        var_tmf2_db0 = assign25140_e37208_d_b0;
        var_tmf2_db1 = assign25140_e37208_d_b1;
        var_tmf2_db2 = assign25140_e37208_d_b2;
        var_tmf2_db3 = assign25140_e37208_d_b3;

        let (assign25150_e37224, assign25150_e37224_d_n0, assign25150_e37224_d_n1, assign25150_e37224_d_n2, assign25150_e37224_d_n3, assign25150_e37224_d_n4, assign25150_e37224_d_n5, assign25150_e37224_d_b0, assign25150_e37224_d_b1, assign25150_e37224_d_b2, assign25150_e37224_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25150_e37219: f64 = (var_tmf1 * var_tmf1);
        let assign25150_e37221: f64 = (assign25150_e37219 + var_tmf2);
        let assign25150_e37222: f64 = (assign25150_e37221).sqrt();
        (assign25150_e37222, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25150_e37222)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25150_e37222)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25150_e37222)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25150_e37222)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25150_e37222)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25150_e37224;
        var_tmf2_dn0 = assign25150_e37224_d_n0;
        var_tmf2_dn1 = assign25150_e37224_d_n1;
        var_tmf2_dn2 = assign25150_e37224_d_n2;
        var_tmf2_dn3 = assign25150_e37224_d_n3;
        var_tmf2_dn4 = assign25150_e37224_d_n4;
        var_tmf2_dn5 = assign25150_e37224_d_n5;
        var_tmf2_db0 = assign25150_e37224_d_b0;
        var_tmf2_db1 = assign25150_e37224_d_b1;
        var_tmf2_db2 = assign25150_e37224_d_b2;
        var_tmf2_db3 = assign25150_e37224_d_b3;

        let (assign25160_e37241, assign25160_e37241_d_n0, assign25160_e37241_d_n1, assign25160_e37241_d_n2, assign25160_e37241_d_n3, assign25160_e37241_d_n4, assign25160_e37241_d_n5, assign25160_e37241_d_b0, assign25160_e37241_d_b1, assign25160_e37241_d_b2, assign25160_e37241_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25160_e37237: f64 = (var_tmf1 + var_tmf2);
        let assign25160_e37238: f64 = (0.5 * assign25160_e37237);
        let assign25160_e37239: f64 = (var_nfasti_i + assign25160_e37238);
        (assign25160_e37239, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25160_e37241;
        var_nj0_dn0 = assign25160_e37241_d_n0;
        var_nj0_dn1 = assign25160_e37241_d_n1;
        var_nj0_dn2 = assign25160_e37241_d_n2;
        var_nj0_dn3 = assign25160_e37241_d_n3;
        var_nj0_dn4 = assign25160_e37241_d_n4;
        var_nj0_dn5 = assign25160_e37241_d_n5;
        var_nj0_db0 = assign25160_e37241_d_b0;
        var_nj0_db1 = assign25160_e37241_d_b1;
        var_nj0_db2 = assign25160_e37241_d_b2;
        var_nj0_db3 = assign25160_e37241_d_b3;

        let (assign25170_e37256, assign25170_e37256_d_n0, assign25170_e37256_d_n1, assign25170_e37256_d_n2, assign25170_e37256_d_n3, assign25170_e37256_d_n4, assign25170_e37256_d_n5, assign25170_e37256_d_b0, assign25170_e37256_d_b1, assign25170_e37256_d_b2, assign25170_e37256_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25170_e37252: f64 = (p.p86 * var_dfn_su);
        let assign25170_e37254: f64 = (assign25170_e37252 * var_dfn_sl);
        (assign25170_e37254, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign25170_e37256;
        var_dnj1_dv_dn0 = assign25170_e37256_d_n0;
        var_dnj1_dv_dn1 = assign25170_e37256_d_n1;
        var_dnj1_dv_dn2 = assign25170_e37256_d_n2;
        var_dnj1_dv_dn3 = assign25170_e37256_d_n3;
        var_dnj1_dv_dn4 = assign25170_e37256_d_n4;
        var_dnj1_dv_dn5 = assign25170_e37256_d_n5;
        var_dnj1_dv_db0 = assign25170_e37256_d_b0;
        var_dnj1_dv_db1 = assign25170_e37256_d_b1;
        var_dnj1_dv_db2 = assign25170_e37256_d_b2;
        var_dnj1_dv_db3 = assign25170_e37256_d_b3;

        let (assign25180_e37268, assign25180_e37268_d_n0, assign25180_e37268_d_n1, assign25180_e37268_d_n2, assign25180_e37268_d_n3, assign25180_e37268_d_n4, assign25180_e37268_d_n5, assign25180_e37268_d_b0, assign25180_e37268_d_b1, assign25180_e37268_d_b2, assign25180_e37268_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25180_e37268;
        var_nj0_dn0 = assign25180_e37268_d_n0;
        var_nj0_dn1 = assign25180_e37268_d_n1;
        var_nj0_dn2 = assign25180_e37268_d_n2;
        var_nj0_dn3 = assign25180_e37268_d_n3;
        var_nj0_dn4 = assign25180_e37268_d_n4;
        var_nj0_dn5 = assign25180_e37268_d_n5;
        var_nj0_db0 = assign25180_e37268_d_b0;
        var_nj0_db1 = assign25180_e37268_d_b1;
        var_nj0_db2 = assign25180_e37268_d_b2;
        var_nj0_db3 = assign25180_e37268_d_b3;

        let (assign25190_e37280, assign25190_e37280_d_n0, assign25190_e37280_d_n1, assign25190_e37280_d_n2, assign25190_e37280_d_n3, assign25190_e37280_d_n4, assign25190_e37280_d_n5, assign25190_e37280_d_b0, assign25190_e37280_d_b1, assign25190_e37280_d_b2, assign25190_e37280_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign25190_e37280;
        var_nj1_dn0 = assign25190_e37280_d_n0;
        var_nj1_dn1 = assign25190_e37280_d_n1;
        var_nj1_dn2 = assign25190_e37280_d_n2;
        var_nj1_dn3 = assign25190_e37280_d_n3;
        var_nj1_dn4 = assign25190_e37280_d_n4;
        var_nj1_dn5 = assign25190_e37280_d_n5;
        var_nj1_db0 = assign25190_e37280_d_b0;
        var_nj1_db1 = assign25190_e37280_d_b1;
        var_nj1_db2 = assign25190_e37280_d_b2;
        var_nj1_db3 = assign25190_e37280_d_b3;

        let (assign25200_e37292, assign25200_e37292_d_n0, assign25200_e37292_d_n1, assign25200_e37292_d_n2, assign25200_e37292_d_n3, assign25200_e37292_d_n4, assign25200_e37292_d_n5, assign25200_e37292_d_b0, assign25200_e37292_d_b1, assign25200_e37292_d_b2, assign25200_e37292_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign25200_e37292;
        var_dnj1_dv_dn0 = assign25200_e37292_d_n0;
        var_dnj1_dv_dn1 = assign25200_e37292_d_n1;
        var_dnj1_dv_dn2 = assign25200_e37292_d_n2;
        var_dnj1_dv_dn3 = assign25200_e37292_d_n3;
        var_dnj1_dv_dn4 = assign25200_e37292_d_n4;
        var_dnj1_dv_dn5 = assign25200_e37292_d_n5;
        var_dnj1_dv_db0 = assign25200_e37292_d_b0;
        var_dnj1_dv_db1 = assign25200_e37292_d_b1;
        var_dnj1_dv_db2 = assign25200_e37292_d_b2;
        var_dnj1_dv_db3 = assign25200_e37292_d_b3;


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

    pub(super) fn stamp_transient_block_41(
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
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
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
        var_guard403_slot: &mut f64,
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
        let mut var_guard403: f64 = *var_guard403_slot;
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

        let (assign25260_e37541, assign25260_e37541_d_n0, assign25260_e37541_d_n1, assign25260_e37541_d_n2, assign25260_e37541_d_n3, assign25260_e37541_d_n4, assign25260_e37541_d_n5, assign25260_e37541_d_b0, assign25260_e37541_d_b1, assign25260_e37541_d_b2, assign25260_e37541_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25260_e37525: f64 = (var_vmax * var_dnj1_dv);
        let assign25260_e37526: f64 = (var_nj1 - assign25260_e37525);
        let assign25260_e37529: f64 = (var_nj1 * var_nj1);
        let assign25260_e37530: f64 = (assign25260_e37526 / assign25260_e37529);
        let assign25260_e37533: f64 = (var_vha1 * var_dnj1_dv);
        let assign25260_e37536: f64 = (var_nj0 * p.p85);
        let assign25260_e37537: f64 = (assign25260_e37533 / assign25260_e37536);
        let assign25260_e37538: f64 = (assign25260_e37530 + assign25260_e37537);
        let assign25260_e37539: f64 = (var_phitdinv * assign25260_e37538);
        (assign25260_e37539, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn0 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn1 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn2 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn3 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn4 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn5 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_db0) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_db0 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_db1) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_db1 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_db2) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_db2 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_db3) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_db3 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign25260_e37541;
        var_dvmax_over_phitd_dv_dn0 = assign25260_e37541_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign25260_e37541_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign25260_e37541_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign25260_e37541_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign25260_e37541_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign25260_e37541_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign25260_e37541_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign25260_e37541_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign25260_e37541_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign25260_e37541_d_b3;

        let (assign25280_e37571,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25280_e37567: f64 = (var_nin * var_nin);
        let assign25280_e37569: f64 = (assign25280_e37567 / var_ndigat_i);
        (assign25280_e37569,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign25280_e37571;

        let (assign25290_e37587,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25290_e37580: f64 = (var_nfagat_i / var_phitdinv);
        let assign25290_e37583: f64 = (var_ndigat_i / var_pnn0);
        let assign25290_e37584: f64 = (assign25290_e37583).ln();
        let assign25290_e37585: f64 = (assign25290_e37580 * assign25290_e37584);
        (assign25290_e37585,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign25290_e37587;

        let assign25300_e37590: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard403 = assign25300_e37590;

        let (assign25310_e37607, assign25310_e37607_d_n0, assign25310_e37607_d_n1, assign25310_e37607_d_n2, assign25310_e37607_d_n3, assign25310_e37607_d_n4, assign25310_e37607_d_n5, assign25310_e37607_d_b0, assign25310_e37607_d_b1, assign25310_e37607_d_b2, assign25310_e37607_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25310_e37602: f64 = (var_vmax - var_vha1);
        let assign25310_e37603: f64 = (p.p86 * assign25310_e37602);
        let assign25310_e37605: f64 = (assign25310_e37603 + var_nfagat_i);
        (assign25310_e37605, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign25310_e37607;
        var_nja10_dn0 = assign25310_e37607_d_n0;
        var_nja10_dn1 = assign25310_e37607_d_n1;
        var_nja10_dn2 = assign25310_e37607_d_n2;
        var_nja10_dn3 = assign25310_e37607_d_n3;
        var_nja10_dn4 = assign25310_e37607_d_n4;
        var_nja10_dn5 = assign25310_e37607_d_n5;
        var_nja10_db0 = assign25310_e37607_d_b0;
        var_nja10_db1 = assign25310_e37607_d_b1;
        var_nja10_db2 = assign25310_e37607_d_b2;
        var_nja10_db3 = assign25310_e37607_d_b3;

        let (assign25320_e37622, assign25320_e37622_d_n0, assign25320_e37622_d_n1, assign25320_e37622_d_n2, assign25320_e37622_d_n3, assign25320_e37622_d_n4, assign25320_e37622_d_n5, assign25320_e37622_d_b0, assign25320_e37622_d_b1, assign25320_e37622_d_b2, assign25320_e37622_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25320_e37619: f64 = (p.p86 * var_vha1);
        let assign25320_e37620: f64 = (var_nfagat_i - assign25320_e37619);
        (assign25320_e37620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25320_e37622;
        var_nj0_dn0 = assign25320_e37622_d_n0;
        var_nj0_dn1 = assign25320_e37622_d_n1;
        var_nj0_dn2 = assign25320_e37622_d_n2;
        var_nj0_dn3 = assign25320_e37622_d_n3;
        var_nj0_dn4 = assign25320_e37622_d_n4;
        var_nj0_dn5 = assign25320_e37622_d_n5;
        var_nj0_db0 = assign25320_e37622_d_b0;
        var_nj0_db1 = assign25320_e37622_d_b1;
        var_nj0_db2 = assign25320_e37622_d_b2;
        var_nj0_db3 = assign25320_e37622_d_b3;

        let (assign25330_e37637, assign25330_e37637_d_n0, assign25330_e37637_d_n1, assign25330_e37637_d_n2, assign25330_e37637_d_n3, assign25330_e37637_d_n4, assign25330_e37637_d_n5, assign25330_e37637_d_b0, assign25330_e37637_d_b1, assign25330_e37637_d_b2, assign25330_e37637_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25330_e37633: f64 = (p.p85 - var_nja10);
        let assign25330_e37635: f64 = (assign25330_e37633 - 0.01);
        (assign25330_e37635, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25330_e37637;
        var_tmf1_dn0 = assign25330_e37637_d_n0;
        var_tmf1_dn1 = assign25330_e37637_d_n1;
        var_tmf1_dn2 = assign25330_e37637_d_n2;
        var_tmf1_dn3 = assign25330_e37637_d_n3;
        var_tmf1_dn4 = assign25330_e37637_d_n4;
        var_tmf1_dn5 = assign25330_e37637_d_n5;
        var_tmf1_db0 = assign25330_e37637_d_b0;
        var_tmf1_db1 = assign25330_e37637_d_b1;
        var_tmf1_db2 = assign25330_e37637_d_b2;
        var_tmf1_db3 = assign25330_e37637_d_b3;

        let (assign25340_e37652, assign25340_e37652_d_n0, assign25340_e37652_d_n1, assign25340_e37652_d_n2, assign25340_e37652_d_n3, assign25340_e37652_d_n4, assign25340_e37652_d_n5, assign25340_e37652_d_b0, assign25340_e37652_d_b1, assign25340_e37652_d_b2, assign25340_e37652_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25340_e37648: f64 = (4.0 * p.p85);
        let assign25340_e37650: f64 = (assign25340_e37648 * 0.01);
        (assign25340_e37650, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25340_e37652;
        var_tmf2_dn0 = assign25340_e37652_d_n0;
        var_tmf2_dn1 = assign25340_e37652_d_n1;
        var_tmf2_dn2 = assign25340_e37652_d_n2;
        var_tmf2_dn3 = assign25340_e37652_d_n3;
        var_tmf2_dn4 = assign25340_e37652_d_n4;
        var_tmf2_dn5 = assign25340_e37652_d_n5;
        var_tmf2_db0 = assign25340_e37652_d_b0;
        var_tmf2_db1 = assign25340_e37652_d_b1;
        var_tmf2_db2 = assign25340_e37652_d_b2;
        var_tmf2_db3 = assign25340_e37652_d_b3;

        let (assign25350_e37669, assign25350_e37669_d_n0, assign25350_e37669_d_n1, assign25350_e37669_d_n2, assign25350_e37669_d_n3, assign25350_e37669_d_n4, assign25350_e37669_d_n5, assign25350_e37669_d_b0, assign25350_e37669_d_b1, assign25350_e37669_d_b2, assign25350_e37669_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n1, assign25350_e37667_d_n2, assign25350_e37667_d_n3, assign25350_e37667_d_n4, assign25350_e37667_d_n5, assign25350_e37667_d_b0, assign25350_e37667_d_b1, assign25350_e37667_d_b2, assign25350_e37667_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25350_e37666: f64 = (-var_tmf2);
                (assign25350_e37666, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n1, assign25350_e37667_d_n2, assign25350_e37667_d_n3, assign25350_e37667_d_n4, assign25350_e37667_d_n5, assign25350_e37667_d_b0, assign25350_e37667_d_b1, assign25350_e37667_d_b2, assign25350_e37667_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25350_e37669;
        var_tmf2_dn0 = assign25350_e37669_d_n0;
        var_tmf2_dn1 = assign25350_e37669_d_n1;
        var_tmf2_dn2 = assign25350_e37669_d_n2;
        var_tmf2_dn3 = assign25350_e37669_d_n3;
        var_tmf2_dn4 = assign25350_e37669_d_n4;
        var_tmf2_dn5 = assign25350_e37669_d_n5;
        var_tmf2_db0 = assign25350_e37669_d_b0;
        var_tmf2_db1 = assign25350_e37669_d_b1;
        var_tmf2_db2 = assign25350_e37669_d_b2;
        var_tmf2_db3 = assign25350_e37669_d_b3;

        let (assign25360_e37685, assign25360_e37685_d_n0, assign25360_e37685_d_n1, assign25360_e37685_d_n2, assign25360_e37685_d_n3, assign25360_e37685_d_n4, assign25360_e37685_d_n5, assign25360_e37685_d_b0, assign25360_e37685_d_b1, assign25360_e37685_d_b2, assign25360_e37685_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25360_e37680: f64 = (var_tmf1 * var_tmf1);
        let assign25360_e37682: f64 = (assign25360_e37680 + var_tmf2);
        let assign25360_e37683: f64 = (assign25360_e37682).sqrt();
        (assign25360_e37683, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25360_e37683)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25360_e37683)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25360_e37683)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25360_e37683)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25360_e37683)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25360_e37685;
        var_tmf2_dn0 = assign25360_e37685_d_n0;
        var_tmf2_dn1 = assign25360_e37685_d_n1;
        var_tmf2_dn2 = assign25360_e37685_d_n2;
        var_tmf2_dn3 = assign25360_e37685_d_n3;
        var_tmf2_dn4 = assign25360_e37685_d_n4;
        var_tmf2_dn5 = assign25360_e37685_d_n5;
        var_tmf2_db0 = assign25360_e37685_d_b0;
        var_tmf2_db1 = assign25360_e37685_d_b1;
        var_tmf2_db2 = assign25360_e37685_d_b2;
        var_tmf2_db3 = assign25360_e37685_d_b3;

        let (assign25370_e37702, assign25370_e37702_d_n0, assign25370_e37702_d_n1, assign25370_e37702_d_n2, assign25370_e37702_d_n3, assign25370_e37702_d_n4, assign25370_e37702_d_n5, assign25370_e37702_d_b0, assign25370_e37702_d_b1, assign25370_e37702_d_b2, assign25370_e37702_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25370_e37698: f64 = (var_tmf1 / var_tmf2);
        let assign25370_e37699: f64 = (1.0 + assign25370_e37698);
        let assign25370_e37700: f64 = (0.5 * assign25370_e37699);
        (assign25370_e37700, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign25370_e37702;
        var_dfn_su_dn0 = assign25370_e37702_d_n0;
        var_dfn_su_dn1 = assign25370_e37702_d_n1;
        var_dfn_su_dn2 = assign25370_e37702_d_n2;
        var_dfn_su_dn3 = assign25370_e37702_d_n3;
        var_dfn_su_dn4 = assign25370_e37702_d_n4;
        var_dfn_su_dn5 = assign25370_e37702_d_n5;
        var_dfn_su_db0 = assign25370_e37702_d_b0;
        var_dfn_su_db1 = assign25370_e37702_d_b1;
        var_dfn_su_db2 = assign25370_e37702_d_b2;
        var_dfn_su_db3 = assign25370_e37702_d_b3;

        let (assign25380_e37719, assign25380_e37719_d_n0, assign25380_e37719_d_n1, assign25380_e37719_d_n2, assign25380_e37719_d_n3, assign25380_e37719_d_n4, assign25380_e37719_d_n5, assign25380_e37719_d_b0, assign25380_e37719_d_b1, assign25380_e37719_d_b2, assign25380_e37719_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25380_e37715: f64 = (var_tmf1 + var_tmf2);
        let assign25380_e37716: f64 = (0.5 * assign25380_e37715);
        let assign25380_e37717: f64 = (p.p85 - assign25380_e37716);
        (assign25380_e37717, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign25380_e37719;
        var_nja11_dn0 = assign25380_e37719_d_n0;
        var_nja11_dn1 = assign25380_e37719_d_n1;
        var_nja11_dn2 = assign25380_e37719_d_n2;
        var_nja11_dn3 = assign25380_e37719_d_n3;
        var_nja11_dn4 = assign25380_e37719_d_n4;
        var_nja11_dn5 = assign25380_e37719_d_n5;
        var_nja11_db0 = assign25380_e37719_d_b0;
        var_nja11_db1 = assign25380_e37719_d_b1;
        var_nja11_db2 = assign25380_e37719_d_b2;
        var_nja11_db3 = assign25380_e37719_d_b3;

        let (assign25390_e37734, assign25390_e37734_d_n0, assign25390_e37734_d_n1, assign25390_e37734_d_n2, assign25390_e37734_d_n3, assign25390_e37734_d_n4, assign25390_e37734_d_n5, assign25390_e37734_d_b0, assign25390_e37734_d_b1, assign25390_e37734_d_b2, assign25390_e37734_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25390_e37730: f64 = (var_nja11 - var_nfagat_i);
        let assign25390_e37732: f64 = (assign25390_e37730 - 0.01);
        (assign25390_e37732, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25390_e37734;
        var_tmf1_dn0 = assign25390_e37734_d_n0;
        var_tmf1_dn1 = assign25390_e37734_d_n1;
        var_tmf1_dn2 = assign25390_e37734_d_n2;
        var_tmf1_dn3 = assign25390_e37734_d_n3;
        var_tmf1_dn4 = assign25390_e37734_d_n4;
        var_tmf1_dn5 = assign25390_e37734_d_n5;
        var_tmf1_db0 = assign25390_e37734_d_b0;
        var_tmf1_db1 = assign25390_e37734_d_b1;
        var_tmf1_db2 = assign25390_e37734_d_b2;
        var_tmf1_db3 = assign25390_e37734_d_b3;

        let (assign25400_e37749, assign25400_e37749_d_n0, assign25400_e37749_d_n1, assign25400_e37749_d_n2, assign25400_e37749_d_n3, assign25400_e37749_d_n4, assign25400_e37749_d_n5, assign25400_e37749_d_b0, assign25400_e37749_d_b1, assign25400_e37749_d_b2, assign25400_e37749_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25400_e37745: f64 = (4.0 * var_nfagat_i);
        let assign25400_e37747: f64 = (assign25400_e37745 * 0.01);
        (assign25400_e37747, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25400_e37749;
        var_tmf2_dn0 = assign25400_e37749_d_n0;
        var_tmf2_dn1 = assign25400_e37749_d_n1;
        var_tmf2_dn2 = assign25400_e37749_d_n2;
        var_tmf2_dn3 = assign25400_e37749_d_n3;
        var_tmf2_dn4 = assign25400_e37749_d_n4;
        var_tmf2_dn5 = assign25400_e37749_d_n5;
        var_tmf2_db0 = assign25400_e37749_d_b0;
        var_tmf2_db1 = assign25400_e37749_d_b1;
        var_tmf2_db2 = assign25400_e37749_d_b2;
        var_tmf2_db3 = assign25400_e37749_d_b3;

        let (assign25410_e37766, assign25410_e37766_d_n0, assign25410_e37766_d_n1, assign25410_e37766_d_n2, assign25410_e37766_d_n3, assign25410_e37766_d_n4, assign25410_e37766_d_n5, assign25410_e37766_d_b0, assign25410_e37766_d_b1, assign25410_e37766_d_b2, assign25410_e37766_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n1, assign25410_e37764_d_n2, assign25410_e37764_d_n3, assign25410_e37764_d_n4, assign25410_e37764_d_n5, assign25410_e37764_d_b0, assign25410_e37764_d_b1, assign25410_e37764_d_b2, assign25410_e37764_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25410_e37763: f64 = (-var_tmf2);
                (assign25410_e37763, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n1, assign25410_e37764_d_n2, assign25410_e37764_d_n3, assign25410_e37764_d_n4, assign25410_e37764_d_n5, assign25410_e37764_d_b0, assign25410_e37764_d_b1, assign25410_e37764_d_b2, assign25410_e37764_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25410_e37766;
        var_tmf2_dn0 = assign25410_e37766_d_n0;
        var_tmf2_dn1 = assign25410_e37766_d_n1;
        var_tmf2_dn2 = assign25410_e37766_d_n2;
        var_tmf2_dn3 = assign25410_e37766_d_n3;
        var_tmf2_dn4 = assign25410_e37766_d_n4;
        var_tmf2_dn5 = assign25410_e37766_d_n5;
        var_tmf2_db0 = assign25410_e37766_d_b0;
        var_tmf2_db1 = assign25410_e37766_d_b1;
        var_tmf2_db2 = assign25410_e37766_d_b2;
        var_tmf2_db3 = assign25410_e37766_d_b3;

        let (assign25420_e37782, assign25420_e37782_d_n0, assign25420_e37782_d_n1, assign25420_e37782_d_n2, assign25420_e37782_d_n3, assign25420_e37782_d_n4, assign25420_e37782_d_n5, assign25420_e37782_d_b0, assign25420_e37782_d_b1, assign25420_e37782_d_b2, assign25420_e37782_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25420_e37777: f64 = (var_tmf1 * var_tmf1);
        let assign25420_e37779: f64 = (assign25420_e37777 + var_tmf2);
        let assign25420_e37780: f64 = (assign25420_e37779).sqrt();
        (assign25420_e37780, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25420_e37780)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25420_e37780)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25420_e37780)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25420_e37780)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25420_e37780)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25420_e37782;
        var_tmf2_dn0 = assign25420_e37782_d_n0;
        var_tmf2_dn1 = assign25420_e37782_d_n1;
        var_tmf2_dn2 = assign25420_e37782_d_n2;
        var_tmf2_dn3 = assign25420_e37782_d_n3;
        var_tmf2_dn4 = assign25420_e37782_d_n4;
        var_tmf2_dn5 = assign25420_e37782_d_n5;
        var_tmf2_db0 = assign25420_e37782_d_b0;
        var_tmf2_db1 = assign25420_e37782_d_b1;
        var_tmf2_db2 = assign25420_e37782_d_b2;
        var_tmf2_db3 = assign25420_e37782_d_b3;

        let (assign25430_e37799, assign25430_e37799_d_n0, assign25430_e37799_d_n1, assign25430_e37799_d_n2, assign25430_e37799_d_n3, assign25430_e37799_d_n4, assign25430_e37799_d_n5, assign25430_e37799_d_b0, assign25430_e37799_d_b1, assign25430_e37799_d_b2, assign25430_e37799_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25430_e37795: f64 = (var_tmf1 / var_tmf2);
        let assign25430_e37796: f64 = (1.0 + assign25430_e37795);
        let assign25430_e37797: f64 = (0.5 * assign25430_e37796);
        (assign25430_e37797, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign25430_e37799;
        var_dfn_sl_dn0 = assign25430_e37799_d_n0;
        var_dfn_sl_dn1 = assign25430_e37799_d_n1;
        var_dfn_sl_dn2 = assign25430_e37799_d_n2;
        var_dfn_sl_dn3 = assign25430_e37799_d_n3;
        var_dfn_sl_dn4 = assign25430_e37799_d_n4;
        var_dfn_sl_dn5 = assign25430_e37799_d_n5;
        var_dfn_sl_db0 = assign25430_e37799_d_b0;
        var_dfn_sl_db1 = assign25430_e37799_d_b1;
        var_dfn_sl_db2 = assign25430_e37799_d_b2;
        var_dfn_sl_db3 = assign25430_e37799_d_b3;

        let (assign25440_e37816, assign25440_e37816_d_n0, assign25440_e37816_d_n1, assign25440_e37816_d_n2, assign25440_e37816_d_n3, assign25440_e37816_d_n4, assign25440_e37816_d_n5, assign25440_e37816_d_b0, assign25440_e37816_d_b1, assign25440_e37816_d_b2, assign25440_e37816_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25440_e37812: f64 = (var_tmf1 + var_tmf2);
        let assign25440_e37813: f64 = (0.5 * assign25440_e37812);
        let assign25440_e37814: f64 = (var_nfagat_i + assign25440_e37813);
        (assign25440_e37814, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign25440_e37816;
        var_nj1_dn0 = assign25440_e37816_d_n0;
        var_nj1_dn1 = assign25440_e37816_d_n1;
        var_nj1_dn2 = assign25440_e37816_d_n2;
        var_nj1_dn3 = assign25440_e37816_d_n3;
        var_nj1_dn4 = assign25440_e37816_d_n4;
        var_nj1_dn5 = assign25440_e37816_d_n5;
        var_nj1_db0 = assign25440_e37816_d_b0;
        var_nj1_db1 = assign25440_e37816_d_b1;
        var_nj1_db2 = assign25440_e37816_d_b2;
        var_nj1_db3 = assign25440_e37816_d_b3;

        let (assign25450_e37831, assign25450_e37831_d_n0, assign25450_e37831_d_n1, assign25450_e37831_d_n2, assign25450_e37831_d_n3, assign25450_e37831_d_n4, assign25450_e37831_d_n5, assign25450_e37831_d_b0, assign25450_e37831_d_b1, assign25450_e37831_d_b2, assign25450_e37831_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25450_e37827: f64 = (p.p85 - var_nj0);
        let assign25450_e37829: f64 = (assign25450_e37827 - 0.01);
        (assign25450_e37829, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25450_e37831;
        var_tmf1_dn0 = assign25450_e37831_d_n0;
        var_tmf1_dn1 = assign25450_e37831_d_n1;
        var_tmf1_dn2 = assign25450_e37831_d_n2;
        var_tmf1_dn3 = assign25450_e37831_d_n3;
        var_tmf1_dn4 = assign25450_e37831_d_n4;
        var_tmf1_dn5 = assign25450_e37831_d_n5;
        var_tmf1_db0 = assign25450_e37831_d_b0;
        var_tmf1_db1 = assign25450_e37831_d_b1;
        var_tmf1_db2 = assign25450_e37831_d_b2;
        var_tmf1_db3 = assign25450_e37831_d_b3;

        let (assign25460_e37846, assign25460_e37846_d_n0, assign25460_e37846_d_n1, assign25460_e37846_d_n2, assign25460_e37846_d_n3, assign25460_e37846_d_n4, assign25460_e37846_d_n5, assign25460_e37846_d_b0, assign25460_e37846_d_b1, assign25460_e37846_d_b2, assign25460_e37846_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25460_e37842: f64 = (4.0 * p.p85);
        let assign25460_e37844: f64 = (assign25460_e37842 * 0.01);
        (assign25460_e37844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25460_e37846;
        var_tmf2_dn0 = assign25460_e37846_d_n0;
        var_tmf2_dn1 = assign25460_e37846_d_n1;
        var_tmf2_dn2 = assign25460_e37846_d_n2;
        var_tmf2_dn3 = assign25460_e37846_d_n3;
        var_tmf2_dn4 = assign25460_e37846_d_n4;
        var_tmf2_dn5 = assign25460_e37846_d_n5;
        var_tmf2_db0 = assign25460_e37846_d_b0;
        var_tmf2_db1 = assign25460_e37846_d_b1;
        var_tmf2_db2 = assign25460_e37846_d_b2;
        var_tmf2_db3 = assign25460_e37846_d_b3;

        let (assign25470_e37863, assign25470_e37863_d_n0, assign25470_e37863_d_n1, assign25470_e37863_d_n2, assign25470_e37863_d_n3, assign25470_e37863_d_n4, assign25470_e37863_d_n5, assign25470_e37863_d_b0, assign25470_e37863_d_b1, assign25470_e37863_d_b2, assign25470_e37863_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n1, assign25470_e37861_d_n2, assign25470_e37861_d_n3, assign25470_e37861_d_n4, assign25470_e37861_d_n5, assign25470_e37861_d_b0, assign25470_e37861_d_b1, assign25470_e37861_d_b2, assign25470_e37861_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25470_e37860: f64 = (-var_tmf2);
                (assign25470_e37860, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n1, assign25470_e37861_d_n2, assign25470_e37861_d_n3, assign25470_e37861_d_n4, assign25470_e37861_d_n5, assign25470_e37861_d_b0, assign25470_e37861_d_b1, assign25470_e37861_d_b2, assign25470_e37861_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25470_e37863;
        var_tmf2_dn0 = assign25470_e37863_d_n0;
        var_tmf2_dn1 = assign25470_e37863_d_n1;
        var_tmf2_dn2 = assign25470_e37863_d_n2;
        var_tmf2_dn3 = assign25470_e37863_d_n3;
        var_tmf2_dn4 = assign25470_e37863_d_n4;
        var_tmf2_dn5 = assign25470_e37863_d_n5;
        var_tmf2_db0 = assign25470_e37863_d_b0;
        var_tmf2_db1 = assign25470_e37863_d_b1;
        var_tmf2_db2 = assign25470_e37863_d_b2;
        var_tmf2_db3 = assign25470_e37863_d_b3;

        let (assign25480_e37879, assign25480_e37879_d_n0, assign25480_e37879_d_n1, assign25480_e37879_d_n2, assign25480_e37879_d_n3, assign25480_e37879_d_n4, assign25480_e37879_d_n5, assign25480_e37879_d_b0, assign25480_e37879_d_b1, assign25480_e37879_d_b2, assign25480_e37879_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25480_e37874: f64 = (var_tmf1 * var_tmf1);
        let assign25480_e37876: f64 = (assign25480_e37874 + var_tmf2);
        let assign25480_e37877: f64 = (assign25480_e37876).sqrt();
        (assign25480_e37877, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25480_e37877)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25480_e37877)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25480_e37877)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25480_e37877)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25480_e37877)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25480_e37879;
        var_tmf2_dn0 = assign25480_e37879_d_n0;
        var_tmf2_dn1 = assign25480_e37879_d_n1;
        var_tmf2_dn2 = assign25480_e37879_d_n2;
        var_tmf2_dn3 = assign25480_e37879_d_n3;
        var_tmf2_dn4 = assign25480_e37879_d_n4;
        var_tmf2_dn5 = assign25480_e37879_d_n5;
        var_tmf2_db0 = assign25480_e37879_d_b0;
        var_tmf2_db1 = assign25480_e37879_d_b1;
        var_tmf2_db2 = assign25480_e37879_d_b2;
        var_tmf2_db3 = assign25480_e37879_d_b3;

        let (assign25490_e37896, assign25490_e37896_d_n0, assign25490_e37896_d_n1, assign25490_e37896_d_n2, assign25490_e37896_d_n3, assign25490_e37896_d_n4, assign25490_e37896_d_n5, assign25490_e37896_d_b0, assign25490_e37896_d_b1, assign25490_e37896_d_b2, assign25490_e37896_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25490_e37892: f64 = (var_tmf1 + var_tmf2);
        let assign25490_e37893: f64 = (0.5 * assign25490_e37892);
        let assign25490_e37894: f64 = (p.p85 - assign25490_e37893);
        (assign25490_e37894, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25490_e37896;
        var_nj0_dn0 = assign25490_e37896_d_n0;
        var_nj0_dn1 = assign25490_e37896_d_n1;
        var_nj0_dn2 = assign25490_e37896_d_n2;
        var_nj0_dn3 = assign25490_e37896_d_n3;
        var_nj0_dn4 = assign25490_e37896_d_n4;
        var_nj0_dn5 = assign25490_e37896_d_n5;
        var_nj0_db0 = assign25490_e37896_d_b0;
        var_nj0_db1 = assign25490_e37896_d_b1;
        var_nj0_db2 = assign25490_e37896_d_b2;
        var_nj0_db3 = assign25490_e37896_d_b3;

        let (assign25500_e37911, assign25500_e37911_d_n0, assign25500_e37911_d_n1, assign25500_e37911_d_n2, assign25500_e37911_d_n3, assign25500_e37911_d_n4, assign25500_e37911_d_n5, assign25500_e37911_d_b0, assign25500_e37911_d_b1, assign25500_e37911_d_b2, assign25500_e37911_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25500_e37907: f64 = (var_nj0 - var_nfagat_i);
        let assign25500_e37909: f64 = (assign25500_e37907 - 0.01);
        (assign25500_e37909, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign25500_e37911;
        var_tmf1_dn0 = assign25500_e37911_d_n0;
        var_tmf1_dn1 = assign25500_e37911_d_n1;
        var_tmf1_dn2 = assign25500_e37911_d_n2;
        var_tmf1_dn3 = assign25500_e37911_d_n3;
        var_tmf1_dn4 = assign25500_e37911_d_n4;
        var_tmf1_dn5 = assign25500_e37911_d_n5;
        var_tmf1_db0 = assign25500_e37911_d_b0;
        var_tmf1_db1 = assign25500_e37911_d_b1;
        var_tmf1_db2 = assign25500_e37911_d_b2;
        var_tmf1_db3 = assign25500_e37911_d_b3;


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
        *var_guard403_slot = var_guard403;
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

    pub(super) fn stamp_transient_block_42(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
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
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard403: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_swjunexp_i: f64,
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
        var_guard471_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard483_slot: &mut f64,
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
        var_vak_slot: &mut f64,
        var_vak_db0_slot: &mut f64,
        var_vak_db1_slot: &mut f64,
        var_vak_db2_slot: &mut f64,
        var_vak_db3_slot: &mut f64,
        var_vak_dn0_slot: &mut f64,
        var_vak_dn1_slot: &mut f64,
        var_vak_dn2_slot: &mut f64,
        var_vak_dn3_slot: &mut f64,
        var_vak_dn4_slot: &mut f64,
        var_vak_dn5_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
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
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard483: f64 = *var_guard483_slot;
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
        let mut var_vak: f64 = *var_vak_slot;
        let mut var_vak_db0: f64 = *var_vak_db0_slot;
        let mut var_vak_db1: f64 = *var_vak_db1_slot;
        let mut var_vak_db2: f64 = *var_vak_db2_slot;
        let mut var_vak_db3: f64 = *var_vak_db3_slot;
        let mut var_vak_dn0: f64 = *var_vak_dn0_slot;
        let mut var_vak_dn1: f64 = *var_vak_dn1_slot;
        let mut var_vak_dn2: f64 = *var_vak_dn2_slot;
        let mut var_vak_dn3: f64 = *var_vak_dn3_slot;
        let mut var_vak_dn4: f64 = *var_vak_dn4_slot;
        let mut var_vak_dn5: f64 = *var_vak_dn5_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign25510_e37926, assign25510_e37926_d_n0, assign25510_e37926_d_n1, assign25510_e37926_d_n2, assign25510_e37926_d_n3, assign25510_e37926_d_n4, assign25510_e37926_d_n5, assign25510_e37926_d_b0, assign25510_e37926_d_b1, assign25510_e37926_d_b2, assign25510_e37926_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25510_e37922: f64 = (4.0 * var_nfagat_i);
        let assign25510_e37924: f64 = (assign25510_e37922 * 0.01);
        (assign25510_e37924, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25510_e37926;
        var_tmf2_dn0 = assign25510_e37926_d_n0;
        var_tmf2_dn1 = assign25510_e37926_d_n1;
        var_tmf2_dn2 = assign25510_e37926_d_n2;
        var_tmf2_dn3 = assign25510_e37926_d_n3;
        var_tmf2_dn4 = assign25510_e37926_d_n4;
        var_tmf2_dn5 = assign25510_e37926_d_n5;
        var_tmf2_db0 = assign25510_e37926_d_b0;
        var_tmf2_db1 = assign25510_e37926_d_b1;
        var_tmf2_db2 = assign25510_e37926_d_b2;
        var_tmf2_db3 = assign25510_e37926_d_b3;

        let (assign25520_e37943, assign25520_e37943_d_n0, assign25520_e37943_d_n1, assign25520_e37943_d_n2, assign25520_e37943_d_n3, assign25520_e37943_d_n4, assign25520_e37943_d_n5, assign25520_e37943_d_b0, assign25520_e37943_d_b1, assign25520_e37943_d_b2, assign25520_e37943_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n1, assign25520_e37941_d_n2, assign25520_e37941_d_n3, assign25520_e37941_d_n4, assign25520_e37941_d_n5, assign25520_e37941_d_b0, assign25520_e37941_d_b1, assign25520_e37941_d_b2, assign25520_e37941_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign25520_e37940: f64 = (-var_tmf2);
                (assign25520_e37940, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n1, assign25520_e37941_d_n2, assign25520_e37941_d_n3, assign25520_e37941_d_n4, assign25520_e37941_d_n5, assign25520_e37941_d_b0, assign25520_e37941_d_b1, assign25520_e37941_d_b2, assign25520_e37941_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25520_e37943;
        var_tmf2_dn0 = assign25520_e37943_d_n0;
        var_tmf2_dn1 = assign25520_e37943_d_n1;
        var_tmf2_dn2 = assign25520_e37943_d_n2;
        var_tmf2_dn3 = assign25520_e37943_d_n3;
        var_tmf2_dn4 = assign25520_e37943_d_n4;
        var_tmf2_dn5 = assign25520_e37943_d_n5;
        var_tmf2_db0 = assign25520_e37943_d_b0;
        var_tmf2_db1 = assign25520_e37943_d_b1;
        var_tmf2_db2 = assign25520_e37943_d_b2;
        var_tmf2_db3 = assign25520_e37943_d_b3;

        let (assign25530_e37959, assign25530_e37959_d_n0, assign25530_e37959_d_n1, assign25530_e37959_d_n2, assign25530_e37959_d_n3, assign25530_e37959_d_n4, assign25530_e37959_d_n5, assign25530_e37959_d_b0, assign25530_e37959_d_b1, assign25530_e37959_d_b2, assign25530_e37959_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25530_e37954: f64 = (var_tmf1 * var_tmf1);
        let assign25530_e37956: f64 = (assign25530_e37954 + var_tmf2);
        let assign25530_e37957: f64 = (assign25530_e37956).sqrt();
        (assign25530_e37957, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign25530_e37957)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign25530_e37957)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign25530_e37957)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign25530_e37957)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign25530_e37957)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign25530_e37959;
        var_tmf2_dn0 = assign25530_e37959_d_n0;
        var_tmf2_dn1 = assign25530_e37959_d_n1;
        var_tmf2_dn2 = assign25530_e37959_d_n2;
        var_tmf2_dn3 = assign25530_e37959_d_n3;
        var_tmf2_dn4 = assign25530_e37959_d_n4;
        var_tmf2_dn5 = assign25530_e37959_d_n5;
        var_tmf2_db0 = assign25530_e37959_d_b0;
        var_tmf2_db1 = assign25530_e37959_d_b1;
        var_tmf2_db2 = assign25530_e37959_d_b2;
        var_tmf2_db3 = assign25530_e37959_d_b3;

        let (assign25540_e37976, assign25540_e37976_d_n0, assign25540_e37976_d_n1, assign25540_e37976_d_n2, assign25540_e37976_d_n3, assign25540_e37976_d_n4, assign25540_e37976_d_n5, assign25540_e37976_d_b0, assign25540_e37976_d_b1, assign25540_e37976_d_b2, assign25540_e37976_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25540_e37972: f64 = (var_tmf1 + var_tmf2);
        let assign25540_e37973: f64 = (0.5 * assign25540_e37972);
        let assign25540_e37974: f64 = (var_nfagat_i + assign25540_e37973);
        (assign25540_e37974, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25540_e37976;
        var_nj0_dn0 = assign25540_e37976_d_n0;
        var_nj0_dn1 = assign25540_e37976_d_n1;
        var_nj0_dn2 = assign25540_e37976_d_n2;
        var_nj0_dn3 = assign25540_e37976_d_n3;
        var_nj0_dn4 = assign25540_e37976_d_n4;
        var_nj0_dn5 = assign25540_e37976_d_n5;
        var_nj0_db0 = assign25540_e37976_d_b0;
        var_nj0_db1 = assign25540_e37976_d_b1;
        var_nj0_db2 = assign25540_e37976_d_b2;
        var_nj0_db3 = assign25540_e37976_d_b3;

        let (assign25550_e37991, assign25550_e37991_d_n0, assign25550_e37991_d_n1, assign25550_e37991_d_n2, assign25550_e37991_d_n3, assign25550_e37991_d_n4, assign25550_e37991_d_n5, assign25550_e37991_d_b0, assign25550_e37991_d_b1, assign25550_e37991_d_b2, assign25550_e37991_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25550_e37987: f64 = (p.p86 * var_dfn_su);
        let assign25550_e37989: f64 = (assign25550_e37987 * var_dfn_sl);
        (assign25550_e37989, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign25550_e37991;
        var_dnj1_dv_dn0 = assign25550_e37991_d_n0;
        var_dnj1_dv_dn1 = assign25550_e37991_d_n1;
        var_dnj1_dv_dn2 = assign25550_e37991_d_n2;
        var_dnj1_dv_dn3 = assign25550_e37991_d_n3;
        var_dnj1_dv_dn4 = assign25550_e37991_d_n4;
        var_dnj1_dv_dn5 = assign25550_e37991_d_n5;
        var_dnj1_dv_db0 = assign25550_e37991_d_b0;
        var_dnj1_dv_db1 = assign25550_e37991_d_b1;
        var_dnj1_dv_db2 = assign25550_e37991_d_b2;
        var_dnj1_dv_db3 = assign25550_e37991_d_b3;

        let (assign25560_e38003, assign25560_e38003_d_n0, assign25560_e38003_d_n1, assign25560_e38003_d_n2, assign25560_e38003_d_n3, assign25560_e38003_d_n4, assign25560_e38003_d_n5, assign25560_e38003_d_b0, assign25560_e38003_d_b1, assign25560_e38003_d_b2, assign25560_e38003_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign25560_e38003;
        var_nj0_dn0 = assign25560_e38003_d_n0;
        var_nj0_dn1 = assign25560_e38003_d_n1;
        var_nj0_dn2 = assign25560_e38003_d_n2;
        var_nj0_dn3 = assign25560_e38003_d_n3;
        var_nj0_dn4 = assign25560_e38003_d_n4;
        var_nj0_dn5 = assign25560_e38003_d_n5;
        var_nj0_db0 = assign25560_e38003_d_b0;
        var_nj0_db1 = assign25560_e38003_d_b1;
        var_nj0_db2 = assign25560_e38003_d_b2;
        var_nj0_db3 = assign25560_e38003_d_b3;

        let (assign25570_e38015, assign25570_e38015_d_n0, assign25570_e38015_d_n1, assign25570_e38015_d_n2, assign25570_e38015_d_n3, assign25570_e38015_d_n4, assign25570_e38015_d_n5, assign25570_e38015_d_b0, assign25570_e38015_d_b1, assign25570_e38015_d_b2, assign25570_e38015_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign25570_e38015;
        var_nj1_dn0 = assign25570_e38015_d_n0;
        var_nj1_dn1 = assign25570_e38015_d_n1;
        var_nj1_dn2 = assign25570_e38015_d_n2;
        var_nj1_dn3 = assign25570_e38015_d_n3;
        var_nj1_dn4 = assign25570_e38015_d_n4;
        var_nj1_dn5 = assign25570_e38015_d_n5;
        var_nj1_db0 = assign25570_e38015_d_b0;
        var_nj1_db1 = assign25570_e38015_d_b1;
        var_nj1_db2 = assign25570_e38015_d_b2;
        var_nj1_db3 = assign25570_e38015_d_b3;

        let (assign25580_e38027, assign25580_e38027_d_n0, assign25580_e38027_d_n1, assign25580_e38027_d_n2, assign25580_e38027_d_n3, assign25580_e38027_d_n4, assign25580_e38027_d_n5, assign25580_e38027_d_b0, assign25580_e38027_d_b1, assign25580_e38027_d_b2, assign25580_e38027_d_b3,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign25580_e38027;
        var_dnj1_dv_dn0 = assign25580_e38027_d_n0;
        var_dnj1_dv_dn1 = assign25580_e38027_d_n1;
        var_dnj1_dv_dn2 = assign25580_e38027_d_n2;
        var_dnj1_dv_dn3 = assign25580_e38027_d_n3;
        var_dnj1_dv_dn4 = assign25580_e38027_d_n4;
        var_dnj1_dv_dn5 = assign25580_e38027_d_n5;
        var_dnj1_dv_db0 = assign25580_e38027_d_b0;
        var_dnj1_dv_db1 = assign25580_e38027_d_b1;
        var_dnj1_dv_db2 = assign25580_e38027_d_b2;
        var_dnj1_dv_db3 = assign25580_e38027_d_b3;

        let (assign25640_e38276, assign25640_e38276_d_n0, assign25640_e38276_d_n1, assign25640_e38276_d_n2, assign25640_e38276_d_n3, assign25640_e38276_d_n4, assign25640_e38276_d_n5, assign25640_e38276_d_b0, assign25640_e38276_d_b1, assign25640_e38276_d_b2, assign25640_e38276_d_b3,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25640_e38260: f64 = (var_vmax * var_dnj1_dv);
        let assign25640_e38261: f64 = (var_nj1 - assign25640_e38260);
        let assign25640_e38264: f64 = (var_nj1 * var_nj1);
        let assign25640_e38265: f64 = (assign25640_e38261 / assign25640_e38264);
        let assign25640_e38268: f64 = (var_vha1 * var_dnj1_dv);
        let assign25640_e38271: f64 = (var_nj0 * p.p85);
        let assign25640_e38272: f64 = (assign25640_e38268 / assign25640_e38271);
        let assign25640_e38273: f64 = (assign25640_e38265 + assign25640_e38272);
        let assign25640_e38274: f64 = (var_phitdinv * assign25640_e38273);
        (assign25640_e38274, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn0 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn1 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn2 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn3 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn4 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn5 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_db0) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_db0 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_db1) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_db1 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_db2) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_db2 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_db3) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_db3 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign25640_e38276;
        var_dvmax_over_phitd_dv_dn0 = assign25640_e38276_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign25640_e38276_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign25640_e38276_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign25640_e38276_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign25640_e38276_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign25640_e38276_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign25640_e38276_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign25640_e38276_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign25640_e38276_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign25640_e38276_d_b3;

        let (assign25660_e38301, assign25660_e38301_d_n0, assign25660_e38301_d_n1, assign25660_e38301_d_n2, assign25660_e38301_d_n3, assign25660_e38301_d_n4, assign25660_e38301_d_n5, assign25660_e38301_d_b0, assign25660_e38301_d_b1, assign25660_e38301_d_b2, assign25660_e38301_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard384 != 0.0)) {
        let assign25660_e38299: f64 = (var_idmultbot - 1.0);
        (assign25660_e38299, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign25660_e38301;
        var_idmultbot_dn0 = assign25660_e38301_d_n0;
        var_idmultbot_dn1 = assign25660_e38301_d_n1;
        var_idmultbot_dn2 = assign25660_e38301_d_n2;
        var_idmultbot_dn3 = assign25660_e38301_d_n3;
        var_idmultbot_dn4 = assign25660_e38301_d_n4;
        var_idmultbot_dn5 = assign25660_e38301_d_n5;
        var_idmultbot_db0 = assign25660_e38301_d_b0;
        var_idmultbot_db1 = assign25660_e38301_d_b1;
        var_idmultbot_db2 = assign25660_e38301_d_b2;
        var_idmultbot_db3 = assign25660_e38301_d_b3;

        let (assign25770_e38474, assign25770_e38474_d_n0, assign25770_e38474_d_n1, assign25770_e38474_d_n2, assign25770_e38474_d_n3, assign25770_e38474_d_n4, assign25770_e38474_d_n5, assign25770_e38474_d_b0, assign25770_e38474_d_b1, assign25770_e38474_d_b2, assign25770_e38474_d_b3,) = {
    if ((var_guard31 != 0.0) && (var_guard384 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign25770_e38474;
        var_idmultbot_dn0 = assign25770_e38474_d_n0;
        var_idmultbot_dn1 = assign25770_e38474_d_n1;
        var_idmultbot_dn2 = assign25770_e38474_d_n2;
        var_idmultbot_dn3 = assign25770_e38474_d_n3;
        var_idmultbot_dn4 = assign25770_e38474_d_n4;
        var_idmultbot_dn5 = assign25770_e38474_d_n5;
        var_idmultbot_db0 = assign25770_e38474_d_b0;
        var_idmultbot_db1 = assign25770_e38474_d_b1;
        var_idmultbot_db2 = assign25770_e38474_d_b2;
        var_idmultbot_db3 = assign25770_e38474_d_b3;

        var_vak = (nv0 - nv2);
        var_vak_dn0 = 1.0;
        var_vak_dn1 = 0.0;
        var_vak_dn2 = -1.0;
        var_vak_dn3 = 0.0;
        var_vak_dn4 = 0.0;
        var_vak_dn5 = 0.0;
        var_vak_db0 = 0.0;
        var_vak_db1 = 0.0;
        var_vak_db2 = 0.0;
        var_vak_db3 = 0.0;

        let assign28760_e42618: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard471 = assign28760_e42618;

        let assign29220_e43059: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard479 = assign29220_e43059;

        let assign29300_e43138: f64 = if var_vak < var_vmax { 1.0 } else { 0.0 };
        var_guard480 = assign29300_e43138;

        let (assign29360_e43283,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29360_e43279: f64 = (var_nin * var_nin);
        let assign29360_e43281: f64 = (assign29360_e43279 / var_ndibot_i);
        (assign29360_e43281,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign29360_e43283;

        let (assign29370_e43299,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29370_e43292: f64 = (var_nfabot_i / var_phitdinv);
        let assign29370_e43295: f64 = (var_ndibot_i / var_pnn0);
        let assign29370_e43296: f64 = (assign29370_e43295).ln();
        let assign29370_e43297: f64 = (assign29370_e43292 * assign29370_e43296);
        (assign29370_e43297,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign29370_e43299;

        let assign29380_e43302: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard483 = assign29380_e43302;

        let (assign29390_e43319, assign29390_e43319_d_n0, assign29390_e43319_d_n1, assign29390_e43319_d_n2, assign29390_e43319_d_n3, assign29390_e43319_d_n4, assign29390_e43319_d_n5, assign29390_e43319_d_b0, assign29390_e43319_d_b1, assign29390_e43319_d_b2, assign29390_e43319_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29390_e43314: f64 = (var_vak - var_vha1);
        let assign29390_e43315: f64 = (p.p86 * assign29390_e43314);
        let assign29390_e43317: f64 = (assign29390_e43315 + var_nfabot_i);
        (assign29390_e43317, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn1), (p.p86 * var_vak_dn2), (p.p86 * var_vak_dn3), (p.p86 * var_vak_dn4), (p.p86 * var_vak_dn5), (p.p86 * var_vak_db0), (p.p86 * var_vak_db1), (p.p86 * var_vak_db2), (p.p86 * var_vak_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign29390_e43319;
        var_nja10_dn0 = assign29390_e43319_d_n0;
        var_nja10_dn1 = assign29390_e43319_d_n1;
        var_nja10_dn2 = assign29390_e43319_d_n2;
        var_nja10_dn3 = assign29390_e43319_d_n3;
        var_nja10_dn4 = assign29390_e43319_d_n4;
        var_nja10_dn5 = assign29390_e43319_d_n5;
        var_nja10_db0 = assign29390_e43319_d_b0;
        var_nja10_db1 = assign29390_e43319_d_b1;
        var_nja10_db2 = assign29390_e43319_d_b2;
        var_nja10_db3 = assign29390_e43319_d_b3;

        let (assign29400_e43334, assign29400_e43334_d_n0, assign29400_e43334_d_n1, assign29400_e43334_d_n2, assign29400_e43334_d_n3, assign29400_e43334_d_n4, assign29400_e43334_d_n5, assign29400_e43334_d_b0, assign29400_e43334_d_b1, assign29400_e43334_d_b2, assign29400_e43334_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29400_e43331: f64 = (p.p86 * var_vha1);
        let assign29400_e43332: f64 = (var_nfabot_i - assign29400_e43331);
        (assign29400_e43332, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29400_e43334;
        var_nj0_dn0 = assign29400_e43334_d_n0;
        var_nj0_dn1 = assign29400_e43334_d_n1;
        var_nj0_dn2 = assign29400_e43334_d_n2;
        var_nj0_dn3 = assign29400_e43334_d_n3;
        var_nj0_dn4 = assign29400_e43334_d_n4;
        var_nj0_dn5 = assign29400_e43334_d_n5;
        var_nj0_db0 = assign29400_e43334_d_b0;
        var_nj0_db1 = assign29400_e43334_d_b1;
        var_nj0_db2 = assign29400_e43334_d_b2;
        var_nj0_db3 = assign29400_e43334_d_b3;

        let (assign29410_e43349, assign29410_e43349_d_n0, assign29410_e43349_d_n1, assign29410_e43349_d_n2, assign29410_e43349_d_n3, assign29410_e43349_d_n4, assign29410_e43349_d_n5, assign29410_e43349_d_b0, assign29410_e43349_d_b1, assign29410_e43349_d_b2, assign29410_e43349_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29410_e43345: f64 = (p.p85 - var_nja10);
        let assign29410_e43347: f64 = (assign29410_e43345 - 0.01);
        (assign29410_e43347, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29410_e43349;
        var_tmf1_dn0 = assign29410_e43349_d_n0;
        var_tmf1_dn1 = assign29410_e43349_d_n1;
        var_tmf1_dn2 = assign29410_e43349_d_n2;
        var_tmf1_dn3 = assign29410_e43349_d_n3;
        var_tmf1_dn4 = assign29410_e43349_d_n4;
        var_tmf1_dn5 = assign29410_e43349_d_n5;
        var_tmf1_db0 = assign29410_e43349_d_b0;
        var_tmf1_db1 = assign29410_e43349_d_b1;
        var_tmf1_db2 = assign29410_e43349_d_b2;
        var_tmf1_db3 = assign29410_e43349_d_b3;

        let (assign29420_e43364, assign29420_e43364_d_n0, assign29420_e43364_d_n1, assign29420_e43364_d_n2, assign29420_e43364_d_n3, assign29420_e43364_d_n4, assign29420_e43364_d_n5, assign29420_e43364_d_b0, assign29420_e43364_d_b1, assign29420_e43364_d_b2, assign29420_e43364_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29420_e43360: f64 = (4.0 * p.p85);
        let assign29420_e43362: f64 = (assign29420_e43360 * 0.01);
        (assign29420_e43362, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29420_e43364;
        var_tmf2_dn0 = assign29420_e43364_d_n0;
        var_tmf2_dn1 = assign29420_e43364_d_n1;
        var_tmf2_dn2 = assign29420_e43364_d_n2;
        var_tmf2_dn3 = assign29420_e43364_d_n3;
        var_tmf2_dn4 = assign29420_e43364_d_n4;
        var_tmf2_dn5 = assign29420_e43364_d_n5;
        var_tmf2_db0 = assign29420_e43364_d_b0;
        var_tmf2_db1 = assign29420_e43364_d_b1;
        var_tmf2_db2 = assign29420_e43364_d_b2;
        var_tmf2_db3 = assign29420_e43364_d_b3;

        let (assign29430_e43381, assign29430_e43381_d_n0, assign29430_e43381_d_n1, assign29430_e43381_d_n2, assign29430_e43381_d_n3, assign29430_e43381_d_n4, assign29430_e43381_d_n5, assign29430_e43381_d_b0, assign29430_e43381_d_b1, assign29430_e43381_d_b2, assign29430_e43381_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n1, assign29430_e43379_d_n2, assign29430_e43379_d_n3, assign29430_e43379_d_n4, assign29430_e43379_d_n5, assign29430_e43379_d_b0, assign29430_e43379_d_b1, assign29430_e43379_d_b2, assign29430_e43379_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29430_e43378: f64 = (-var_tmf2);
                (assign29430_e43378, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n1, assign29430_e43379_d_n2, assign29430_e43379_d_n3, assign29430_e43379_d_n4, assign29430_e43379_d_n5, assign29430_e43379_d_b0, assign29430_e43379_d_b1, assign29430_e43379_d_b2, assign29430_e43379_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29430_e43381;
        var_tmf2_dn0 = assign29430_e43381_d_n0;
        var_tmf2_dn1 = assign29430_e43381_d_n1;
        var_tmf2_dn2 = assign29430_e43381_d_n2;
        var_tmf2_dn3 = assign29430_e43381_d_n3;
        var_tmf2_dn4 = assign29430_e43381_d_n4;
        var_tmf2_dn5 = assign29430_e43381_d_n5;
        var_tmf2_db0 = assign29430_e43381_d_b0;
        var_tmf2_db1 = assign29430_e43381_d_b1;
        var_tmf2_db2 = assign29430_e43381_d_b2;
        var_tmf2_db3 = assign29430_e43381_d_b3;

        let (assign29440_e43397, assign29440_e43397_d_n0, assign29440_e43397_d_n1, assign29440_e43397_d_n2, assign29440_e43397_d_n3, assign29440_e43397_d_n4, assign29440_e43397_d_n5, assign29440_e43397_d_b0, assign29440_e43397_d_b1, assign29440_e43397_d_b2, assign29440_e43397_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29440_e43392: f64 = (var_tmf1 * var_tmf1);
        let assign29440_e43394: f64 = (assign29440_e43392 + var_tmf2);
        let assign29440_e43395: f64 = (assign29440_e43394).sqrt();
        (assign29440_e43395, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29440_e43395)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29440_e43395)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29440_e43395)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29440_e43395)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29440_e43395)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29440_e43397;
        var_tmf2_dn0 = assign29440_e43397_d_n0;
        var_tmf2_dn1 = assign29440_e43397_d_n1;
        var_tmf2_dn2 = assign29440_e43397_d_n2;
        var_tmf2_dn3 = assign29440_e43397_d_n3;
        var_tmf2_dn4 = assign29440_e43397_d_n4;
        var_tmf2_dn5 = assign29440_e43397_d_n5;
        var_tmf2_db0 = assign29440_e43397_d_b0;
        var_tmf2_db1 = assign29440_e43397_d_b1;
        var_tmf2_db2 = assign29440_e43397_d_b2;
        var_tmf2_db3 = assign29440_e43397_d_b3;

        let (assign29450_e43414, assign29450_e43414_d_n0, assign29450_e43414_d_n1, assign29450_e43414_d_n2, assign29450_e43414_d_n3, assign29450_e43414_d_n4, assign29450_e43414_d_n5, assign29450_e43414_d_b0, assign29450_e43414_d_b1, assign29450_e43414_d_b2, assign29450_e43414_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29450_e43410: f64 = (var_tmf1 + var_tmf2);
        let assign29450_e43411: f64 = (0.5 * assign29450_e43410);
        let assign29450_e43412: f64 = (p.p85 - assign29450_e43411);
        (assign29450_e43412, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign29450_e43414;
        var_nja11_dn0 = assign29450_e43414_d_n0;
        var_nja11_dn1 = assign29450_e43414_d_n1;
        var_nja11_dn2 = assign29450_e43414_d_n2;
        var_nja11_dn3 = assign29450_e43414_d_n3;
        var_nja11_dn4 = assign29450_e43414_d_n4;
        var_nja11_dn5 = assign29450_e43414_d_n5;
        var_nja11_db0 = assign29450_e43414_d_b0;
        var_nja11_db1 = assign29450_e43414_d_b1;
        var_nja11_db2 = assign29450_e43414_d_b2;
        var_nja11_db3 = assign29450_e43414_d_b3;

        let (assign29460_e43429, assign29460_e43429_d_n0, assign29460_e43429_d_n1, assign29460_e43429_d_n2, assign29460_e43429_d_n3, assign29460_e43429_d_n4, assign29460_e43429_d_n5, assign29460_e43429_d_b0, assign29460_e43429_d_b1, assign29460_e43429_d_b2, assign29460_e43429_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29460_e43425: f64 = (var_nja11 - var_nfabot_i);
        let assign29460_e43427: f64 = (assign29460_e43425 - 0.01);
        (assign29460_e43427, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29460_e43429;
        var_tmf1_dn0 = assign29460_e43429_d_n0;
        var_tmf1_dn1 = assign29460_e43429_d_n1;
        var_tmf1_dn2 = assign29460_e43429_d_n2;
        var_tmf1_dn3 = assign29460_e43429_d_n3;
        var_tmf1_dn4 = assign29460_e43429_d_n4;
        var_tmf1_dn5 = assign29460_e43429_d_n5;
        var_tmf1_db0 = assign29460_e43429_d_b0;
        var_tmf1_db1 = assign29460_e43429_d_b1;
        var_tmf1_db2 = assign29460_e43429_d_b2;
        var_tmf1_db3 = assign29460_e43429_d_b3;

        let (assign29470_e43444, assign29470_e43444_d_n0, assign29470_e43444_d_n1, assign29470_e43444_d_n2, assign29470_e43444_d_n3, assign29470_e43444_d_n4, assign29470_e43444_d_n5, assign29470_e43444_d_b0, assign29470_e43444_d_b1, assign29470_e43444_d_b2, assign29470_e43444_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29470_e43440: f64 = (4.0 * var_nfabot_i);
        let assign29470_e43442: f64 = (assign29470_e43440 * 0.01);
        (assign29470_e43442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29470_e43444;
        var_tmf2_dn0 = assign29470_e43444_d_n0;
        var_tmf2_dn1 = assign29470_e43444_d_n1;
        var_tmf2_dn2 = assign29470_e43444_d_n2;
        var_tmf2_dn3 = assign29470_e43444_d_n3;
        var_tmf2_dn4 = assign29470_e43444_d_n4;
        var_tmf2_dn5 = assign29470_e43444_d_n5;
        var_tmf2_db0 = assign29470_e43444_d_b0;
        var_tmf2_db1 = assign29470_e43444_d_b1;
        var_tmf2_db2 = assign29470_e43444_d_b2;
        var_tmf2_db3 = assign29470_e43444_d_b3;

        let (assign29480_e43461, assign29480_e43461_d_n0, assign29480_e43461_d_n1, assign29480_e43461_d_n2, assign29480_e43461_d_n3, assign29480_e43461_d_n4, assign29480_e43461_d_n5, assign29480_e43461_d_b0, assign29480_e43461_d_b1, assign29480_e43461_d_b2, assign29480_e43461_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n1, assign29480_e43459_d_n2, assign29480_e43459_d_n3, assign29480_e43459_d_n4, assign29480_e43459_d_n5, assign29480_e43459_d_b0, assign29480_e43459_d_b1, assign29480_e43459_d_b2, assign29480_e43459_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29480_e43458: f64 = (-var_tmf2);
                (assign29480_e43458, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n1, assign29480_e43459_d_n2, assign29480_e43459_d_n3, assign29480_e43459_d_n4, assign29480_e43459_d_n5, assign29480_e43459_d_b0, assign29480_e43459_d_b1, assign29480_e43459_d_b2, assign29480_e43459_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29480_e43461;
        var_tmf2_dn0 = assign29480_e43461_d_n0;
        var_tmf2_dn1 = assign29480_e43461_d_n1;
        var_tmf2_dn2 = assign29480_e43461_d_n2;
        var_tmf2_dn3 = assign29480_e43461_d_n3;
        var_tmf2_dn4 = assign29480_e43461_d_n4;
        var_tmf2_dn5 = assign29480_e43461_d_n5;
        var_tmf2_db0 = assign29480_e43461_d_b0;
        var_tmf2_db1 = assign29480_e43461_d_b1;
        var_tmf2_db2 = assign29480_e43461_d_b2;
        var_tmf2_db3 = assign29480_e43461_d_b3;


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
        *var_guard471_slot = var_guard471;
        *var_guard479_slot = var_guard479;
        *var_guard480_slot = var_guard480;
        *var_guard483_slot = var_guard483;
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
        *var_vak_slot = var_vak;
        *var_vak_db0_slot = var_vak_db0;
        *var_vak_db1_slot = var_vak_db1;
        *var_vak_db2_slot = var_vak_db2;
        *var_vak_db3_slot = var_vak_db3;
        *var_vak_dn0_slot = var_vak_dn0;
        *var_vak_dn1_slot = var_vak_dn1;
        *var_vak_dn2_slot = var_vak_dn2;
        *var_vak_dn3_slot = var_vak_dn3;
        *var_vak_dn4_slot = var_vak_dn4;
        *var_vak_dn5_slot = var_vak_dn5;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard483: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_guard484_slot: &mut f64,
        var_guard485_slot: &mut f64,
        var_guard486_slot: &mut f64,
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
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
        let mut var_guard486: f64 = *var_guard486_slot;
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

        let (assign29490_e43477, assign29490_e43477_d_n0, assign29490_e43477_d_n1, assign29490_e43477_d_n2, assign29490_e43477_d_n3, assign29490_e43477_d_n4, assign29490_e43477_d_n5, assign29490_e43477_d_b0, assign29490_e43477_d_b1, assign29490_e43477_d_b2, assign29490_e43477_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29490_e43472: f64 = (var_tmf1 * var_tmf1);
        let assign29490_e43474: f64 = (assign29490_e43472 + var_tmf2);
        let assign29490_e43475: f64 = (assign29490_e43474).sqrt();
        (assign29490_e43475, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29490_e43475)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29490_e43475)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29490_e43475)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29490_e43475)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29490_e43475)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29490_e43477;
        var_tmf2_dn0 = assign29490_e43477_d_n0;
        var_tmf2_dn1 = assign29490_e43477_d_n1;
        var_tmf2_dn2 = assign29490_e43477_d_n2;
        var_tmf2_dn3 = assign29490_e43477_d_n3;
        var_tmf2_dn4 = assign29490_e43477_d_n4;
        var_tmf2_dn5 = assign29490_e43477_d_n5;
        var_tmf2_db0 = assign29490_e43477_d_b0;
        var_tmf2_db1 = assign29490_e43477_d_b1;
        var_tmf2_db2 = assign29490_e43477_d_b2;
        var_tmf2_db3 = assign29490_e43477_d_b3;

        let (assign29500_e43494, assign29500_e43494_d_n0, assign29500_e43494_d_n1, assign29500_e43494_d_n2, assign29500_e43494_d_n3, assign29500_e43494_d_n4, assign29500_e43494_d_n5, assign29500_e43494_d_b0, assign29500_e43494_d_b1, assign29500_e43494_d_b2, assign29500_e43494_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29500_e43490: f64 = (var_tmf1 + var_tmf2);
        let assign29500_e43491: f64 = (0.5 * assign29500_e43490);
        let assign29500_e43492: f64 = (var_nfabot_i + assign29500_e43491);
        (assign29500_e43492, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign29500_e43494;
        var_nj1_dn0 = assign29500_e43494_d_n0;
        var_nj1_dn1 = assign29500_e43494_d_n1;
        var_nj1_dn2 = assign29500_e43494_d_n2;
        var_nj1_dn3 = assign29500_e43494_d_n3;
        var_nj1_dn4 = assign29500_e43494_d_n4;
        var_nj1_dn5 = assign29500_e43494_d_n5;
        var_nj1_db0 = assign29500_e43494_d_b0;
        var_nj1_db1 = assign29500_e43494_d_b1;
        var_nj1_db2 = assign29500_e43494_d_b2;
        var_nj1_db3 = assign29500_e43494_d_b3;

        let (assign29510_e43509, assign29510_e43509_d_n0, assign29510_e43509_d_n1, assign29510_e43509_d_n2, assign29510_e43509_d_n3, assign29510_e43509_d_n4, assign29510_e43509_d_n5, assign29510_e43509_d_b0, assign29510_e43509_d_b1, assign29510_e43509_d_b2, assign29510_e43509_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29510_e43505: f64 = (p.p85 - var_nj0);
        let assign29510_e43507: f64 = (assign29510_e43505 - 0.01);
        (assign29510_e43507, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29510_e43509;
        var_tmf1_dn0 = assign29510_e43509_d_n0;
        var_tmf1_dn1 = assign29510_e43509_d_n1;
        var_tmf1_dn2 = assign29510_e43509_d_n2;
        var_tmf1_dn3 = assign29510_e43509_d_n3;
        var_tmf1_dn4 = assign29510_e43509_d_n4;
        var_tmf1_dn5 = assign29510_e43509_d_n5;
        var_tmf1_db0 = assign29510_e43509_d_b0;
        var_tmf1_db1 = assign29510_e43509_d_b1;
        var_tmf1_db2 = assign29510_e43509_d_b2;
        var_tmf1_db3 = assign29510_e43509_d_b3;

        let (assign29520_e43524, assign29520_e43524_d_n0, assign29520_e43524_d_n1, assign29520_e43524_d_n2, assign29520_e43524_d_n3, assign29520_e43524_d_n4, assign29520_e43524_d_n5, assign29520_e43524_d_b0, assign29520_e43524_d_b1, assign29520_e43524_d_b2, assign29520_e43524_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29520_e43520: f64 = (4.0 * p.p85);
        let assign29520_e43522: f64 = (assign29520_e43520 * 0.01);
        (assign29520_e43522, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29520_e43524;
        var_tmf2_dn0 = assign29520_e43524_d_n0;
        var_tmf2_dn1 = assign29520_e43524_d_n1;
        var_tmf2_dn2 = assign29520_e43524_d_n2;
        var_tmf2_dn3 = assign29520_e43524_d_n3;
        var_tmf2_dn4 = assign29520_e43524_d_n4;
        var_tmf2_dn5 = assign29520_e43524_d_n5;
        var_tmf2_db0 = assign29520_e43524_d_b0;
        var_tmf2_db1 = assign29520_e43524_d_b1;
        var_tmf2_db2 = assign29520_e43524_d_b2;
        var_tmf2_db3 = assign29520_e43524_d_b3;

        let (assign29530_e43541, assign29530_e43541_d_n0, assign29530_e43541_d_n1, assign29530_e43541_d_n2, assign29530_e43541_d_n3, assign29530_e43541_d_n4, assign29530_e43541_d_n5, assign29530_e43541_d_b0, assign29530_e43541_d_b1, assign29530_e43541_d_b2, assign29530_e43541_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n1, assign29530_e43539_d_n2, assign29530_e43539_d_n3, assign29530_e43539_d_n4, assign29530_e43539_d_n5, assign29530_e43539_d_b0, assign29530_e43539_d_b1, assign29530_e43539_d_b2, assign29530_e43539_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29530_e43538: f64 = (-var_tmf2);
                (assign29530_e43538, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n1, assign29530_e43539_d_n2, assign29530_e43539_d_n3, assign29530_e43539_d_n4, assign29530_e43539_d_n5, assign29530_e43539_d_b0, assign29530_e43539_d_b1, assign29530_e43539_d_b2, assign29530_e43539_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29530_e43541;
        var_tmf2_dn0 = assign29530_e43541_d_n0;
        var_tmf2_dn1 = assign29530_e43541_d_n1;
        var_tmf2_dn2 = assign29530_e43541_d_n2;
        var_tmf2_dn3 = assign29530_e43541_d_n3;
        var_tmf2_dn4 = assign29530_e43541_d_n4;
        var_tmf2_dn5 = assign29530_e43541_d_n5;
        var_tmf2_db0 = assign29530_e43541_d_b0;
        var_tmf2_db1 = assign29530_e43541_d_b1;
        var_tmf2_db2 = assign29530_e43541_d_b2;
        var_tmf2_db3 = assign29530_e43541_d_b3;

        let (assign29540_e43557, assign29540_e43557_d_n0, assign29540_e43557_d_n1, assign29540_e43557_d_n2, assign29540_e43557_d_n3, assign29540_e43557_d_n4, assign29540_e43557_d_n5, assign29540_e43557_d_b0, assign29540_e43557_d_b1, assign29540_e43557_d_b2, assign29540_e43557_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29540_e43552: f64 = (var_tmf1 * var_tmf1);
        let assign29540_e43554: f64 = (assign29540_e43552 + var_tmf2);
        let assign29540_e43555: f64 = (assign29540_e43554).sqrt();
        (assign29540_e43555, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29540_e43555)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29540_e43555)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29540_e43555)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29540_e43555)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29540_e43555)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29540_e43557;
        var_tmf2_dn0 = assign29540_e43557_d_n0;
        var_tmf2_dn1 = assign29540_e43557_d_n1;
        var_tmf2_dn2 = assign29540_e43557_d_n2;
        var_tmf2_dn3 = assign29540_e43557_d_n3;
        var_tmf2_dn4 = assign29540_e43557_d_n4;
        var_tmf2_dn5 = assign29540_e43557_d_n5;
        var_tmf2_db0 = assign29540_e43557_d_b0;
        var_tmf2_db1 = assign29540_e43557_d_b1;
        var_tmf2_db2 = assign29540_e43557_d_b2;
        var_tmf2_db3 = assign29540_e43557_d_b3;

        let (assign29550_e43574, assign29550_e43574_d_n0, assign29550_e43574_d_n1, assign29550_e43574_d_n2, assign29550_e43574_d_n3, assign29550_e43574_d_n4, assign29550_e43574_d_n5, assign29550_e43574_d_b0, assign29550_e43574_d_b1, assign29550_e43574_d_b2, assign29550_e43574_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29550_e43570: f64 = (var_tmf1 + var_tmf2);
        let assign29550_e43571: f64 = (0.5 * assign29550_e43570);
        let assign29550_e43572: f64 = (p.p85 - assign29550_e43571);
        (assign29550_e43572, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29550_e43574;
        var_nj0_dn0 = assign29550_e43574_d_n0;
        var_nj0_dn1 = assign29550_e43574_d_n1;
        var_nj0_dn2 = assign29550_e43574_d_n2;
        var_nj0_dn3 = assign29550_e43574_d_n3;
        var_nj0_dn4 = assign29550_e43574_d_n4;
        var_nj0_dn5 = assign29550_e43574_d_n5;
        var_nj0_db0 = assign29550_e43574_d_b0;
        var_nj0_db1 = assign29550_e43574_d_b1;
        var_nj0_db2 = assign29550_e43574_d_b2;
        var_nj0_db3 = assign29550_e43574_d_b3;

        let (assign29560_e43589, assign29560_e43589_d_n0, assign29560_e43589_d_n1, assign29560_e43589_d_n2, assign29560_e43589_d_n3, assign29560_e43589_d_n4, assign29560_e43589_d_n5, assign29560_e43589_d_b0, assign29560_e43589_d_b1, assign29560_e43589_d_b2, assign29560_e43589_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29560_e43585: f64 = (var_nj0 - var_nfabot_i);
        let assign29560_e43587: f64 = (assign29560_e43585 - 0.01);
        (assign29560_e43587, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29560_e43589;
        var_tmf1_dn0 = assign29560_e43589_d_n0;
        var_tmf1_dn1 = assign29560_e43589_d_n1;
        var_tmf1_dn2 = assign29560_e43589_d_n2;
        var_tmf1_dn3 = assign29560_e43589_d_n3;
        var_tmf1_dn4 = assign29560_e43589_d_n4;
        var_tmf1_dn5 = assign29560_e43589_d_n5;
        var_tmf1_db0 = assign29560_e43589_d_b0;
        var_tmf1_db1 = assign29560_e43589_d_b1;
        var_tmf1_db2 = assign29560_e43589_d_b2;
        var_tmf1_db3 = assign29560_e43589_d_b3;

        let (assign29570_e43604, assign29570_e43604_d_n0, assign29570_e43604_d_n1, assign29570_e43604_d_n2, assign29570_e43604_d_n3, assign29570_e43604_d_n4, assign29570_e43604_d_n5, assign29570_e43604_d_b0, assign29570_e43604_d_b1, assign29570_e43604_d_b2, assign29570_e43604_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29570_e43600: f64 = (4.0 * var_nfabot_i);
        let assign29570_e43602: f64 = (assign29570_e43600 * 0.01);
        (assign29570_e43602, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29570_e43604;
        var_tmf2_dn0 = assign29570_e43604_d_n0;
        var_tmf2_dn1 = assign29570_e43604_d_n1;
        var_tmf2_dn2 = assign29570_e43604_d_n2;
        var_tmf2_dn3 = assign29570_e43604_d_n3;
        var_tmf2_dn4 = assign29570_e43604_d_n4;
        var_tmf2_dn5 = assign29570_e43604_d_n5;
        var_tmf2_db0 = assign29570_e43604_d_b0;
        var_tmf2_db1 = assign29570_e43604_d_b1;
        var_tmf2_db2 = assign29570_e43604_d_b2;
        var_tmf2_db3 = assign29570_e43604_d_b3;

        let (assign29580_e43621, assign29580_e43621_d_n0, assign29580_e43621_d_n1, assign29580_e43621_d_n2, assign29580_e43621_d_n3, assign29580_e43621_d_n4, assign29580_e43621_d_n5, assign29580_e43621_d_b0, assign29580_e43621_d_b1, assign29580_e43621_d_b2, assign29580_e43621_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n1, assign29580_e43619_d_n2, assign29580_e43619_d_n3, assign29580_e43619_d_n4, assign29580_e43619_d_n5, assign29580_e43619_d_b0, assign29580_e43619_d_b1, assign29580_e43619_d_b2, assign29580_e43619_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29580_e43618: f64 = (-var_tmf2);
                (assign29580_e43618, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n1, assign29580_e43619_d_n2, assign29580_e43619_d_n3, assign29580_e43619_d_n4, assign29580_e43619_d_n5, assign29580_e43619_d_b0, assign29580_e43619_d_b1, assign29580_e43619_d_b2, assign29580_e43619_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29580_e43621;
        var_tmf2_dn0 = assign29580_e43621_d_n0;
        var_tmf2_dn1 = assign29580_e43621_d_n1;
        var_tmf2_dn2 = assign29580_e43621_d_n2;
        var_tmf2_dn3 = assign29580_e43621_d_n3;
        var_tmf2_dn4 = assign29580_e43621_d_n4;
        var_tmf2_dn5 = assign29580_e43621_d_n5;
        var_tmf2_db0 = assign29580_e43621_d_b0;
        var_tmf2_db1 = assign29580_e43621_d_b1;
        var_tmf2_db2 = assign29580_e43621_d_b2;
        var_tmf2_db3 = assign29580_e43621_d_b3;

        let (assign29590_e43637, assign29590_e43637_d_n0, assign29590_e43637_d_n1, assign29590_e43637_d_n2, assign29590_e43637_d_n3, assign29590_e43637_d_n4, assign29590_e43637_d_n5, assign29590_e43637_d_b0, assign29590_e43637_d_b1, assign29590_e43637_d_b2, assign29590_e43637_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29590_e43632: f64 = (var_tmf1 * var_tmf1);
        let assign29590_e43634: f64 = (assign29590_e43632 + var_tmf2);
        let assign29590_e43635: f64 = (assign29590_e43634).sqrt();
        (assign29590_e43635, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29590_e43635)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29590_e43635)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29590_e43635)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29590_e43635)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29590_e43635)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29590_e43637;
        var_tmf2_dn0 = assign29590_e43637_d_n0;
        var_tmf2_dn1 = assign29590_e43637_d_n1;
        var_tmf2_dn2 = assign29590_e43637_d_n2;
        var_tmf2_dn3 = assign29590_e43637_d_n3;
        var_tmf2_dn4 = assign29590_e43637_d_n4;
        var_tmf2_dn5 = assign29590_e43637_d_n5;
        var_tmf2_db0 = assign29590_e43637_d_b0;
        var_tmf2_db1 = assign29590_e43637_d_b1;
        var_tmf2_db2 = assign29590_e43637_d_b2;
        var_tmf2_db3 = assign29590_e43637_d_b3;

        let (assign29600_e43654, assign29600_e43654_d_n0, assign29600_e43654_d_n1, assign29600_e43654_d_n2, assign29600_e43654_d_n3, assign29600_e43654_d_n4, assign29600_e43654_d_n5, assign29600_e43654_d_b0, assign29600_e43654_d_b1, assign29600_e43654_d_b2, assign29600_e43654_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29600_e43650: f64 = (var_tmf1 + var_tmf2);
        let assign29600_e43651: f64 = (0.5 * assign29600_e43650);
        let assign29600_e43652: f64 = (var_nfabot_i + assign29600_e43651);
        (assign29600_e43652, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29600_e43654;
        var_nj0_dn0 = assign29600_e43654_d_n0;
        var_nj0_dn1 = assign29600_e43654_d_n1;
        var_nj0_dn2 = assign29600_e43654_d_n2;
        var_nj0_dn3 = assign29600_e43654_d_n3;
        var_nj0_dn4 = assign29600_e43654_d_n4;
        var_nj0_dn5 = assign29600_e43654_d_n5;
        var_nj0_db0 = assign29600_e43654_d_b0;
        var_nj0_db1 = assign29600_e43654_d_b1;
        var_nj0_db2 = assign29600_e43654_d_b2;
        var_nj0_db3 = assign29600_e43654_d_b3;

        let (assign29610_e43666, assign29610_e43666_d_n0, assign29610_e43666_d_n1, assign29610_e43666_d_n2, assign29610_e43666_d_n3, assign29610_e43666_d_n4, assign29610_e43666_d_n5, assign29610_e43666_d_b0, assign29610_e43666_d_b1, assign29610_e43666_d_b2, assign29610_e43666_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29610_e43666;
        var_nj0_dn0 = assign29610_e43666_d_n0;
        var_nj0_dn1 = assign29610_e43666_d_n1;
        var_nj0_dn2 = assign29610_e43666_d_n2;
        var_nj0_dn3 = assign29610_e43666_d_n3;
        var_nj0_dn4 = assign29610_e43666_d_n4;
        var_nj0_dn5 = assign29610_e43666_d_n5;
        var_nj0_db0 = assign29610_e43666_d_b0;
        var_nj0_db1 = assign29610_e43666_d_b1;
        var_nj0_db2 = assign29610_e43666_d_b2;
        var_nj0_db3 = assign29610_e43666_d_b3;

        let (assign29620_e43678, assign29620_e43678_d_n0, assign29620_e43678_d_n1, assign29620_e43678_d_n2, assign29620_e43678_d_n3, assign29620_e43678_d_n4, assign29620_e43678_d_n5, assign29620_e43678_d_b0, assign29620_e43678_d_b1, assign29620_e43678_d_b2, assign29620_e43678_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign29620_e43678;
        var_nj1_dn0 = assign29620_e43678_d_n0;
        var_nj1_dn1 = assign29620_e43678_d_n1;
        var_nj1_dn2 = assign29620_e43678_d_n2;
        var_nj1_dn3 = assign29620_e43678_d_n3;
        var_nj1_dn4 = assign29620_e43678_d_n4;
        var_nj1_dn5 = assign29620_e43678_d_n5;
        var_nj1_db0 = assign29620_e43678_d_b0;
        var_nj1_db1 = assign29620_e43678_d_b1;
        var_nj1_db2 = assign29620_e43678_d_b2;
        var_nj1_db3 = assign29620_e43678_d_b3;

        let assign29630_e43682: f64 = (var_vak / var_nj1);
        let assign29630_e43686: f64 = (var_nj1 - var_nj0);
        let assign29630_e43687: f64 = (var_vha1 * assign29630_e43686);
        let assign29630_e43690: f64 = (var_nj0 * p.p85);
        let assign29630_e43691: f64 = (assign29630_e43687 / assign29630_e43690);
        let assign29630_e43692: f64 = (assign29630_e43682 + assign29630_e43691);
        let assign29630_e43693: f64 = (var_phitdinv * assign29630_e43692);
        let assign29630_e43694: f64 = (assign29630_e43693).abs();
        let assign29630_e43696: f64 = if assign29630_e43694 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard484 = assign29630_e43696;

        let (assign29640_e43722, assign29640_e43722_d_n0, assign29640_e43722_d_n1, assign29640_e43722_d_n2, assign29640_e43722_d_n3, assign29640_e43722_d_n4, assign29640_e43722_d_n5, assign29640_e43722_d_b0, assign29640_e43722_d_b1, assign29640_e43722_d_b2, assign29640_e43722_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 != 0.0)) {
        let assign29640_e43708: f64 = (var_vak / var_nj1);
        let assign29640_e43712: f64 = (var_nj1 - var_nj0);
        let assign29640_e43713: f64 = (var_vha1 * assign29640_e43712);
        let assign29640_e43716: f64 = (var_nj0 * p.p85);
        let assign29640_e43717: f64 = (assign29640_e43713 / assign29640_e43716);
        let assign29640_e43718: f64 = (assign29640_e43708 + assign29640_e43717);
        let assign29640_e43719: f64 = (var_phitdinv * assign29640_e43718);
        let assign29640_e43720: f64 = (assign29640_e43719).exp();
        (assign29640_e43720, (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn0 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn1 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn2 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn3 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn4 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn5 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_db0 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_db1 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_db2 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_db3 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign29640_e43722;
        var_idmultbot_dn0 = assign29640_e43722_d_n0;
        var_idmultbot_dn1 = assign29640_e43722_d_n1;
        var_idmultbot_dn2 = assign29640_e43722_d_n2;
        var_idmultbot_dn3 = assign29640_e43722_d_n3;
        var_idmultbot_dn4 = assign29640_e43722_d_n4;
        var_idmultbot_dn5 = assign29640_e43722_d_n5;
        var_idmultbot_db0 = assign29640_e43722_d_b0;
        var_idmultbot_db1 = assign29640_e43722_d_b1;
        var_idmultbot_db2 = assign29640_e43722_d_b2;
        var_idmultbot_db3 = assign29640_e43722_d_b3;

        let assign29650_e43726: f64 = (var_vak / var_nj1);
        let assign29650_e43730: f64 = (var_nj1 - var_nj0);
        let assign29650_e43731: f64 = (var_vha1 * assign29650_e43730);
        let assign29650_e43734: f64 = (var_nj0 * p.p85);
        let assign29650_e43735: f64 = (assign29650_e43731 / assign29650_e43734);
        let assign29650_e43736: f64 = (assign29650_e43726 + assign29650_e43735);
        let assign29650_e43737: f64 = (var_phitdinv * assign29650_e43736);
        let assign29650_e43739: f64 = (-230.25850929940458);
        let assign29650_e43740: f64 = if assign29650_e43737 < assign29650_e43739 { 1.0 } else { 0.0 };
        var_guard485 = assign29650_e43740;

        let (assign29660_e43821, assign29660_e43821_d_n0, assign29660_e43821_d_n1, assign29660_e43821_d_n2, assign29660_e43821_d_n3, assign29660_e43821_d_n4, assign29660_e43821_d_n5, assign29660_e43821_d_b0, assign29660_e43821_d_b1, assign29660_e43821_d_b2, assign29660_e43821_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 == 0.0)) && (var_guard485 != 0.0)) {
        let assign29660_e43755: f64 = (-230.25850929940458);
        let assign29660_e43759: f64 = (var_vak / var_nj1);
        let assign29660_e43763: f64 = (var_nj1 - var_nj0);
        let assign29660_e43764: f64 = (var_vha1 * assign29660_e43763);
        let assign29660_e43767: f64 = (var_nj0 * p.p85);
        let assign29660_e43768: f64 = (assign29660_e43764 / assign29660_e43767);
        let assign29660_e43769: f64 = (assign29660_e43759 + assign29660_e43768);
        let assign29660_e43770: f64 = (var_phitdinv * assign29660_e43769);
        let assign29660_e43771: f64 = (assign29660_e43755 - assign29660_e43770);
        let assign29660_e43775: f64 = (-230.25850929940458);
        let assign29660_e43779: f64 = (var_vak / var_nj1);
        let assign29660_e43783: f64 = (var_nj1 - var_nj0);
        let assign29660_e43784: f64 = (var_vha1 * assign29660_e43783);
        let assign29660_e43787: f64 = (var_nj0 * p.p85);
        let assign29660_e43788: f64 = (assign29660_e43784 / assign29660_e43787);
        let assign29660_e43789: f64 = (assign29660_e43779 + assign29660_e43788);
        let assign29660_e43790: f64 = (var_phitdinv * assign29660_e43789);
        let assign29660_e43791: f64 = (assign29660_e43775 - assign29660_e43790);
        let assign29660_e43794: f64 = (-230.25850929940458);
        let assign29660_e43798: f64 = (var_vak / var_nj1);
        let assign29660_e43802: f64 = (var_nj1 - var_nj0);
        let assign29660_e43803: f64 = (var_vha1 * assign29660_e43802);
        let assign29660_e43806: f64 = (var_nj0 * p.p85);
        let assign29660_e43807: f64 = (assign29660_e43803 / assign29660_e43806);
        let assign29660_e43808: f64 = (assign29660_e43798 + assign29660_e43807);
        let assign29660_e43809: f64 = (var_phitdinv * assign29660_e43808);
        let assign29660_e43810: f64 = (assign29660_e43794 - assign29660_e43809);
        let assign29660_e43812: f64 = (assign29660_e43810 * 0.3333333333333333);
        let assign29660_e43813: f64 = (1.0 + assign29660_e43812);
        let assign29660_e43814: f64 = (assign29660_e43791 * assign29660_e43813);
        let assign29660_e43815: f64 = (0.5 * assign29660_e43814);
        let assign29660_e43816: f64 = (1.0 + assign29660_e43815);
        let assign29660_e43817: f64 = (assign29660_e43771 * assign29660_e43816);
        let assign29660_e43818: f64 = (1.0 + assign29660_e43817);
        let assign29660_e43819: f64 = (1e-100 / assign29660_e43818);
        (assign29660_e43819, (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn0 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn0 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn0 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn1 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn1 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn1 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn2 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn2 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn2 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn3 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn3 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn3 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn4 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn4 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn4 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn5 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn5 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn5 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_db0 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_db0 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_db0 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_db1 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_db1 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_db1 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_db2 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_db2 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_db2 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_db3 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_db3 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_db3 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign29660_e43821;
        var_idmultbot_dn0 = assign29660_e43821_d_n0;
        var_idmultbot_dn1 = assign29660_e43821_d_n1;
        var_idmultbot_dn2 = assign29660_e43821_d_n2;
        var_idmultbot_dn3 = assign29660_e43821_d_n3;
        var_idmultbot_dn4 = assign29660_e43821_d_n4;
        var_idmultbot_dn5 = assign29660_e43821_d_n5;
        var_idmultbot_db0 = assign29660_e43821_d_b0;
        var_idmultbot_db1 = assign29660_e43821_d_b1;
        var_idmultbot_db2 = assign29660_e43821_d_b2;
        var_idmultbot_db3 = assign29660_e43821_d_b3;

        let (assign29670_e43900, assign29670_e43900_d_n0, assign29670_e43900_d_n1, assign29670_e43900_d_n2, assign29670_e43900_d_n3, assign29670_e43900_d_n4, assign29670_e43900_d_n5, assign29670_e43900_d_b0, assign29670_e43900_d_b1, assign29670_e43900_d_b2, assign29670_e43900_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 == 0.0)) && (var_guard485 == 0.0)) {
        let assign29670_e43839: f64 = (var_vak / var_nj1);
        let assign29670_e43843: f64 = (var_nj1 - var_nj0);
        let assign29670_e43844: f64 = (var_vha1 * assign29670_e43843);
        let assign29670_e43847: f64 = (var_nj0 * p.p85);
        let assign29670_e43848: f64 = (assign29670_e43844 / assign29670_e43847);
        let assign29670_e43849: f64 = (assign29670_e43839 + assign29670_e43848);
        let assign29670_e43850: f64 = (var_phitdinv * assign29670_e43849);
        let assign29670_e43852: f64 = (assign29670_e43850 - 230.25850929940458);
        let assign29670_e43858: f64 = (var_vak / var_nj1);
        let assign29670_e43862: f64 = (var_nj1 - var_nj0);
        let assign29670_e43863: f64 = (var_vha1 * assign29670_e43862);
        let assign29670_e43866: f64 = (var_nj0 * p.p85);
        let assign29670_e43867: f64 = (assign29670_e43863 / assign29670_e43866);
        let assign29670_e43868: f64 = (assign29670_e43858 + assign29670_e43867);
        let assign29670_e43869: f64 = (var_phitdinv * assign29670_e43868);
        let assign29670_e43871: f64 = (assign29670_e43869 - 230.25850929940458);
        let assign29670_e43876: f64 = (var_vak / var_nj1);
        let assign29670_e43880: f64 = (var_nj1 - var_nj0);
        let assign29670_e43881: f64 = (var_vha1 * assign29670_e43880);
        let assign29670_e43884: f64 = (var_nj0 * p.p85);
        let assign29670_e43885: f64 = (assign29670_e43881 / assign29670_e43884);
        let assign29670_e43886: f64 = (assign29670_e43876 + assign29670_e43885);
        let assign29670_e43887: f64 = (var_phitdinv * assign29670_e43886);
        let assign29670_e43889: f64 = (assign29670_e43887 - 230.25850929940458);
        let assign29670_e43891: f64 = (assign29670_e43889 * 0.3333333333333333);
        let assign29670_e43892: f64 = (1.0 + assign29670_e43891);
        let assign29670_e43893: f64 = (assign29670_e43871 * assign29670_e43892);
        let assign29670_e43894: f64 = (0.5 * assign29670_e43893);
        let assign29670_e43895: f64 = (1.0 + assign29670_e43894);
        let assign29670_e43896: f64 = (assign29670_e43852 * assign29670_e43895);
        let assign29670_e43897: f64 = (1.0 + assign29670_e43896);
        let assign29670_e43898: f64 = (1e100 * assign29670_e43897);
        (assign29670_e43898, (1e100 * (((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn0 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn0 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn0 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn1 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn1 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn1 * var_nj1) - (var_vak * var_nj1_dn1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn1 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn2 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn2 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn2 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn3 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn3 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn3 * var_nj1) - (var_vak * var_nj1_dn3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn3 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn4 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn4 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn4 * var_nj1) - (var_vak * var_nj1_dn4)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn4 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn5 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn5 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn5 * var_nj1) - (var_vak * var_nj1_dn5)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn5 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_db0 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_db0 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_db0 * var_nj1) - (var_vak * var_nj1_db0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_db0 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_db1 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_db1 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_db1 * var_nj1) - (var_vak * var_nj1_db1)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_db1 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_db2 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_db2 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_db2 * var_nj1) - (var_vak * var_nj1_db2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_db2 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_db3 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_db3 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_db3 * var_nj1) - (var_vak * var_nj1_db3)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_db3 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign29670_e43900;
        var_idmultbot_dn0 = assign29670_e43900_d_n0;
        var_idmultbot_dn1 = assign29670_e43900_d_n1;
        var_idmultbot_dn2 = assign29670_e43900_d_n2;
        var_idmultbot_dn3 = assign29670_e43900_d_n3;
        var_idmultbot_dn4 = assign29670_e43900_d_n4;
        var_idmultbot_dn5 = assign29670_e43900_d_n5;
        var_idmultbot_db0 = assign29670_e43900_d_b0;
        var_idmultbot_db1 = assign29670_e43900_d_b1;
        var_idmultbot_db2 = assign29670_e43900_d_b2;
        var_idmultbot_db3 = assign29670_e43900_d_b3;

        let (assign29680_e43913,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29680_e43909: f64 = (var_nin * var_nin);
        let assign29680_e43911: f64 = (assign29680_e43909 / var_ndisti_i);
        (assign29680_e43911,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign29680_e43913;

        let (assign29690_e43929,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29690_e43922: f64 = (var_nfasti_i / var_phitdinv);
        let assign29690_e43925: f64 = (var_ndisti_i / var_pnn0);
        let assign29690_e43926: f64 = (assign29690_e43925).ln();
        let assign29690_e43927: f64 = (assign29690_e43922 * assign29690_e43926);
        (assign29690_e43927,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign29690_e43929;

        let assign29700_e43932: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard486 = assign29700_e43932;

        let (assign29710_e43949, assign29710_e43949_d_n0, assign29710_e43949_d_n1, assign29710_e43949_d_n2, assign29710_e43949_d_n3, assign29710_e43949_d_n4, assign29710_e43949_d_n5, assign29710_e43949_d_b0, assign29710_e43949_d_b1, assign29710_e43949_d_b2, assign29710_e43949_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29710_e43944: f64 = (var_vak - var_vha1);
        let assign29710_e43945: f64 = (p.p86 * assign29710_e43944);
        let assign29710_e43947: f64 = (assign29710_e43945 + var_nfasti_i);
        (assign29710_e43947, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn1), (p.p86 * var_vak_dn2), (p.p86 * var_vak_dn3), (p.p86 * var_vak_dn4), (p.p86 * var_vak_dn5), (p.p86 * var_vak_db0), (p.p86 * var_vak_db1), (p.p86 * var_vak_db2), (p.p86 * var_vak_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign29710_e43949;
        var_nja10_dn0 = assign29710_e43949_d_n0;
        var_nja10_dn1 = assign29710_e43949_d_n1;
        var_nja10_dn2 = assign29710_e43949_d_n2;
        var_nja10_dn3 = assign29710_e43949_d_n3;
        var_nja10_dn4 = assign29710_e43949_d_n4;
        var_nja10_dn5 = assign29710_e43949_d_n5;
        var_nja10_db0 = assign29710_e43949_d_b0;
        var_nja10_db1 = assign29710_e43949_d_b1;
        var_nja10_db2 = assign29710_e43949_d_b2;
        var_nja10_db3 = assign29710_e43949_d_b3;


        *var_guard484_slot = var_guard484;
        *var_guard485_slot = var_guard485;
        *var_guard486_slot = var_guard486;
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

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard486: f64,
        var_nfasti_i: f64,
        var_nja10: f64,
        var_nja10_db0: f64,
        var_nja10_db1: f64,
        var_nja10_db2: f64,
        var_nja10_db3: f64,
        var_nja10_dn0: f64,
        var_nja10_dn1: f64,
        var_nja10_dn2: f64,
        var_nja10_dn3: f64,
        var_nja10_dn4: f64,
        var_nja10_dn5: f64,
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

        let (assign29720_e43964, assign29720_e43964_d_n0, assign29720_e43964_d_n1, assign29720_e43964_d_n2, assign29720_e43964_d_n3, assign29720_e43964_d_n4, assign29720_e43964_d_n5, assign29720_e43964_d_b0, assign29720_e43964_d_b1, assign29720_e43964_d_b2, assign29720_e43964_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29720_e43961: f64 = (p.p86 * var_vha1);
        let assign29720_e43962: f64 = (var_nfasti_i - assign29720_e43961);
        (assign29720_e43962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29720_e43964;
        var_nj0_dn0 = assign29720_e43964_d_n0;
        var_nj0_dn1 = assign29720_e43964_d_n1;
        var_nj0_dn2 = assign29720_e43964_d_n2;
        var_nj0_dn3 = assign29720_e43964_d_n3;
        var_nj0_dn4 = assign29720_e43964_d_n4;
        var_nj0_dn5 = assign29720_e43964_d_n5;
        var_nj0_db0 = assign29720_e43964_d_b0;
        var_nj0_db1 = assign29720_e43964_d_b1;
        var_nj0_db2 = assign29720_e43964_d_b2;
        var_nj0_db3 = assign29720_e43964_d_b3;

        let (assign29730_e43979, assign29730_e43979_d_n0, assign29730_e43979_d_n1, assign29730_e43979_d_n2, assign29730_e43979_d_n3, assign29730_e43979_d_n4, assign29730_e43979_d_n5, assign29730_e43979_d_b0, assign29730_e43979_d_b1, assign29730_e43979_d_b2, assign29730_e43979_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29730_e43975: f64 = (p.p85 - var_nja10);
        let assign29730_e43977: f64 = (assign29730_e43975 - 0.01);
        (assign29730_e43977, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29730_e43979;
        var_tmf1_dn0 = assign29730_e43979_d_n0;
        var_tmf1_dn1 = assign29730_e43979_d_n1;
        var_tmf1_dn2 = assign29730_e43979_d_n2;
        var_tmf1_dn3 = assign29730_e43979_d_n3;
        var_tmf1_dn4 = assign29730_e43979_d_n4;
        var_tmf1_dn5 = assign29730_e43979_d_n5;
        var_tmf1_db0 = assign29730_e43979_d_b0;
        var_tmf1_db1 = assign29730_e43979_d_b1;
        var_tmf1_db2 = assign29730_e43979_d_b2;
        var_tmf1_db3 = assign29730_e43979_d_b3;

        let (assign29740_e43994, assign29740_e43994_d_n0, assign29740_e43994_d_n1, assign29740_e43994_d_n2, assign29740_e43994_d_n3, assign29740_e43994_d_n4, assign29740_e43994_d_n5, assign29740_e43994_d_b0, assign29740_e43994_d_b1, assign29740_e43994_d_b2, assign29740_e43994_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29740_e43990: f64 = (4.0 * p.p85);
        let assign29740_e43992: f64 = (assign29740_e43990 * 0.01);
        (assign29740_e43992, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29740_e43994;
        var_tmf2_dn0 = assign29740_e43994_d_n0;
        var_tmf2_dn1 = assign29740_e43994_d_n1;
        var_tmf2_dn2 = assign29740_e43994_d_n2;
        var_tmf2_dn3 = assign29740_e43994_d_n3;
        var_tmf2_dn4 = assign29740_e43994_d_n4;
        var_tmf2_dn5 = assign29740_e43994_d_n5;
        var_tmf2_db0 = assign29740_e43994_d_b0;
        var_tmf2_db1 = assign29740_e43994_d_b1;
        var_tmf2_db2 = assign29740_e43994_d_b2;
        var_tmf2_db3 = assign29740_e43994_d_b3;

        let (assign29750_e44011, assign29750_e44011_d_n0, assign29750_e44011_d_n1, assign29750_e44011_d_n2, assign29750_e44011_d_n3, assign29750_e44011_d_n4, assign29750_e44011_d_n5, assign29750_e44011_d_b0, assign29750_e44011_d_b1, assign29750_e44011_d_b2, assign29750_e44011_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n1, assign29750_e44009_d_n2, assign29750_e44009_d_n3, assign29750_e44009_d_n4, assign29750_e44009_d_n5, assign29750_e44009_d_b0, assign29750_e44009_d_b1, assign29750_e44009_d_b2, assign29750_e44009_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29750_e44008: f64 = (-var_tmf2);
                (assign29750_e44008, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n1, assign29750_e44009_d_n2, assign29750_e44009_d_n3, assign29750_e44009_d_n4, assign29750_e44009_d_n5, assign29750_e44009_d_b0, assign29750_e44009_d_b1, assign29750_e44009_d_b2, assign29750_e44009_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29750_e44011;
        var_tmf2_dn0 = assign29750_e44011_d_n0;
        var_tmf2_dn1 = assign29750_e44011_d_n1;
        var_tmf2_dn2 = assign29750_e44011_d_n2;
        var_tmf2_dn3 = assign29750_e44011_d_n3;
        var_tmf2_dn4 = assign29750_e44011_d_n4;
        var_tmf2_dn5 = assign29750_e44011_d_n5;
        var_tmf2_db0 = assign29750_e44011_d_b0;
        var_tmf2_db1 = assign29750_e44011_d_b1;
        var_tmf2_db2 = assign29750_e44011_d_b2;
        var_tmf2_db3 = assign29750_e44011_d_b3;

        let (assign29760_e44027, assign29760_e44027_d_n0, assign29760_e44027_d_n1, assign29760_e44027_d_n2, assign29760_e44027_d_n3, assign29760_e44027_d_n4, assign29760_e44027_d_n5, assign29760_e44027_d_b0, assign29760_e44027_d_b1, assign29760_e44027_d_b2, assign29760_e44027_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29760_e44022: f64 = (var_tmf1 * var_tmf1);
        let assign29760_e44024: f64 = (assign29760_e44022 + var_tmf2);
        let assign29760_e44025: f64 = (assign29760_e44024).sqrt();
        (assign29760_e44025, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29760_e44025)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29760_e44025)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29760_e44025)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29760_e44025)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29760_e44025)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29760_e44027;
        var_tmf2_dn0 = assign29760_e44027_d_n0;
        var_tmf2_dn1 = assign29760_e44027_d_n1;
        var_tmf2_dn2 = assign29760_e44027_d_n2;
        var_tmf2_dn3 = assign29760_e44027_d_n3;
        var_tmf2_dn4 = assign29760_e44027_d_n4;
        var_tmf2_dn5 = assign29760_e44027_d_n5;
        var_tmf2_db0 = assign29760_e44027_d_b0;
        var_tmf2_db1 = assign29760_e44027_d_b1;
        var_tmf2_db2 = assign29760_e44027_d_b2;
        var_tmf2_db3 = assign29760_e44027_d_b3;

        let (assign29770_e44044, assign29770_e44044_d_n0, assign29770_e44044_d_n1, assign29770_e44044_d_n2, assign29770_e44044_d_n3, assign29770_e44044_d_n4, assign29770_e44044_d_n5, assign29770_e44044_d_b0, assign29770_e44044_d_b1, assign29770_e44044_d_b2, assign29770_e44044_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29770_e44040: f64 = (var_tmf1 + var_tmf2);
        let assign29770_e44041: f64 = (0.5 * assign29770_e44040);
        let assign29770_e44042: f64 = (p.p85 - assign29770_e44041);
        (assign29770_e44042, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign29770_e44044;
        var_nja11_dn0 = assign29770_e44044_d_n0;
        var_nja11_dn1 = assign29770_e44044_d_n1;
        var_nja11_dn2 = assign29770_e44044_d_n2;
        var_nja11_dn3 = assign29770_e44044_d_n3;
        var_nja11_dn4 = assign29770_e44044_d_n4;
        var_nja11_dn5 = assign29770_e44044_d_n5;
        var_nja11_db0 = assign29770_e44044_d_b0;
        var_nja11_db1 = assign29770_e44044_d_b1;
        var_nja11_db2 = assign29770_e44044_d_b2;
        var_nja11_db3 = assign29770_e44044_d_b3;

        let (assign29780_e44059, assign29780_e44059_d_n0, assign29780_e44059_d_n1, assign29780_e44059_d_n2, assign29780_e44059_d_n3, assign29780_e44059_d_n4, assign29780_e44059_d_n5, assign29780_e44059_d_b0, assign29780_e44059_d_b1, assign29780_e44059_d_b2, assign29780_e44059_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29780_e44055: f64 = (var_nja11 - var_nfasti_i);
        let assign29780_e44057: f64 = (assign29780_e44055 - 0.01);
        (assign29780_e44057, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29780_e44059;
        var_tmf1_dn0 = assign29780_e44059_d_n0;
        var_tmf1_dn1 = assign29780_e44059_d_n1;
        var_tmf1_dn2 = assign29780_e44059_d_n2;
        var_tmf1_dn3 = assign29780_e44059_d_n3;
        var_tmf1_dn4 = assign29780_e44059_d_n4;
        var_tmf1_dn5 = assign29780_e44059_d_n5;
        var_tmf1_db0 = assign29780_e44059_d_b0;
        var_tmf1_db1 = assign29780_e44059_d_b1;
        var_tmf1_db2 = assign29780_e44059_d_b2;
        var_tmf1_db3 = assign29780_e44059_d_b3;

        let (assign29790_e44074, assign29790_e44074_d_n0, assign29790_e44074_d_n1, assign29790_e44074_d_n2, assign29790_e44074_d_n3, assign29790_e44074_d_n4, assign29790_e44074_d_n5, assign29790_e44074_d_b0, assign29790_e44074_d_b1, assign29790_e44074_d_b2, assign29790_e44074_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29790_e44070: f64 = (4.0 * var_nfasti_i);
        let assign29790_e44072: f64 = (assign29790_e44070 * 0.01);
        (assign29790_e44072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29790_e44074;
        var_tmf2_dn0 = assign29790_e44074_d_n0;
        var_tmf2_dn1 = assign29790_e44074_d_n1;
        var_tmf2_dn2 = assign29790_e44074_d_n2;
        var_tmf2_dn3 = assign29790_e44074_d_n3;
        var_tmf2_dn4 = assign29790_e44074_d_n4;
        var_tmf2_dn5 = assign29790_e44074_d_n5;
        var_tmf2_db0 = assign29790_e44074_d_b0;
        var_tmf2_db1 = assign29790_e44074_d_b1;
        var_tmf2_db2 = assign29790_e44074_d_b2;
        var_tmf2_db3 = assign29790_e44074_d_b3;

        let (assign29800_e44091, assign29800_e44091_d_n0, assign29800_e44091_d_n1, assign29800_e44091_d_n2, assign29800_e44091_d_n3, assign29800_e44091_d_n4, assign29800_e44091_d_n5, assign29800_e44091_d_b0, assign29800_e44091_d_b1, assign29800_e44091_d_b2, assign29800_e44091_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n1, assign29800_e44089_d_n2, assign29800_e44089_d_n3, assign29800_e44089_d_n4, assign29800_e44089_d_n5, assign29800_e44089_d_b0, assign29800_e44089_d_b1, assign29800_e44089_d_b2, assign29800_e44089_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29800_e44088: f64 = (-var_tmf2);
                (assign29800_e44088, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n1, assign29800_e44089_d_n2, assign29800_e44089_d_n3, assign29800_e44089_d_n4, assign29800_e44089_d_n5, assign29800_e44089_d_b0, assign29800_e44089_d_b1, assign29800_e44089_d_b2, assign29800_e44089_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29800_e44091;
        var_tmf2_dn0 = assign29800_e44091_d_n0;
        var_tmf2_dn1 = assign29800_e44091_d_n1;
        var_tmf2_dn2 = assign29800_e44091_d_n2;
        var_tmf2_dn3 = assign29800_e44091_d_n3;
        var_tmf2_dn4 = assign29800_e44091_d_n4;
        var_tmf2_dn5 = assign29800_e44091_d_n5;
        var_tmf2_db0 = assign29800_e44091_d_b0;
        var_tmf2_db1 = assign29800_e44091_d_b1;
        var_tmf2_db2 = assign29800_e44091_d_b2;
        var_tmf2_db3 = assign29800_e44091_d_b3;

        let (assign29810_e44107, assign29810_e44107_d_n0, assign29810_e44107_d_n1, assign29810_e44107_d_n2, assign29810_e44107_d_n3, assign29810_e44107_d_n4, assign29810_e44107_d_n5, assign29810_e44107_d_b0, assign29810_e44107_d_b1, assign29810_e44107_d_b2, assign29810_e44107_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29810_e44102: f64 = (var_tmf1 * var_tmf1);
        let assign29810_e44104: f64 = (assign29810_e44102 + var_tmf2);
        let assign29810_e44105: f64 = (assign29810_e44104).sqrt();
        (assign29810_e44105, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29810_e44105)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29810_e44105)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29810_e44105)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29810_e44105)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29810_e44105)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29810_e44107;
        var_tmf2_dn0 = assign29810_e44107_d_n0;
        var_tmf2_dn1 = assign29810_e44107_d_n1;
        var_tmf2_dn2 = assign29810_e44107_d_n2;
        var_tmf2_dn3 = assign29810_e44107_d_n3;
        var_tmf2_dn4 = assign29810_e44107_d_n4;
        var_tmf2_dn5 = assign29810_e44107_d_n5;
        var_tmf2_db0 = assign29810_e44107_d_b0;
        var_tmf2_db1 = assign29810_e44107_d_b1;
        var_tmf2_db2 = assign29810_e44107_d_b2;
        var_tmf2_db3 = assign29810_e44107_d_b3;

        let (assign29820_e44124, assign29820_e44124_d_n0, assign29820_e44124_d_n1, assign29820_e44124_d_n2, assign29820_e44124_d_n3, assign29820_e44124_d_n4, assign29820_e44124_d_n5, assign29820_e44124_d_b0, assign29820_e44124_d_b1, assign29820_e44124_d_b2, assign29820_e44124_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29820_e44120: f64 = (var_tmf1 + var_tmf2);
        let assign29820_e44121: f64 = (0.5 * assign29820_e44120);
        let assign29820_e44122: f64 = (var_nfasti_i + assign29820_e44121);
        (assign29820_e44122, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign29820_e44124;
        var_nj1_dn0 = assign29820_e44124_d_n0;
        var_nj1_dn1 = assign29820_e44124_d_n1;
        var_nj1_dn2 = assign29820_e44124_d_n2;
        var_nj1_dn3 = assign29820_e44124_d_n3;
        var_nj1_dn4 = assign29820_e44124_d_n4;
        var_nj1_dn5 = assign29820_e44124_d_n5;
        var_nj1_db0 = assign29820_e44124_d_b0;
        var_nj1_db1 = assign29820_e44124_d_b1;
        var_nj1_db2 = assign29820_e44124_d_b2;
        var_nj1_db3 = assign29820_e44124_d_b3;

        let (assign29830_e44139, assign29830_e44139_d_n0, assign29830_e44139_d_n1, assign29830_e44139_d_n2, assign29830_e44139_d_n3, assign29830_e44139_d_n4, assign29830_e44139_d_n5, assign29830_e44139_d_b0, assign29830_e44139_d_b1, assign29830_e44139_d_b2, assign29830_e44139_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29830_e44135: f64 = (p.p85 - var_nj0);
        let assign29830_e44137: f64 = (assign29830_e44135 - 0.01);
        (assign29830_e44137, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29830_e44139;
        var_tmf1_dn0 = assign29830_e44139_d_n0;
        var_tmf1_dn1 = assign29830_e44139_d_n1;
        var_tmf1_dn2 = assign29830_e44139_d_n2;
        var_tmf1_dn3 = assign29830_e44139_d_n3;
        var_tmf1_dn4 = assign29830_e44139_d_n4;
        var_tmf1_dn5 = assign29830_e44139_d_n5;
        var_tmf1_db0 = assign29830_e44139_d_b0;
        var_tmf1_db1 = assign29830_e44139_d_b1;
        var_tmf1_db2 = assign29830_e44139_d_b2;
        var_tmf1_db3 = assign29830_e44139_d_b3;

        let (assign29840_e44154, assign29840_e44154_d_n0, assign29840_e44154_d_n1, assign29840_e44154_d_n2, assign29840_e44154_d_n3, assign29840_e44154_d_n4, assign29840_e44154_d_n5, assign29840_e44154_d_b0, assign29840_e44154_d_b1, assign29840_e44154_d_b2, assign29840_e44154_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29840_e44150: f64 = (4.0 * p.p85);
        let assign29840_e44152: f64 = (assign29840_e44150 * 0.01);
        (assign29840_e44152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29840_e44154;
        var_tmf2_dn0 = assign29840_e44154_d_n0;
        var_tmf2_dn1 = assign29840_e44154_d_n1;
        var_tmf2_dn2 = assign29840_e44154_d_n2;
        var_tmf2_dn3 = assign29840_e44154_d_n3;
        var_tmf2_dn4 = assign29840_e44154_d_n4;
        var_tmf2_dn5 = assign29840_e44154_d_n5;
        var_tmf2_db0 = assign29840_e44154_d_b0;
        var_tmf2_db1 = assign29840_e44154_d_b1;
        var_tmf2_db2 = assign29840_e44154_d_b2;
        var_tmf2_db3 = assign29840_e44154_d_b3;

        let (assign29850_e44171, assign29850_e44171_d_n0, assign29850_e44171_d_n1, assign29850_e44171_d_n2, assign29850_e44171_d_n3, assign29850_e44171_d_n4, assign29850_e44171_d_n5, assign29850_e44171_d_b0, assign29850_e44171_d_b1, assign29850_e44171_d_b2, assign29850_e44171_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n1, assign29850_e44169_d_n2, assign29850_e44169_d_n3, assign29850_e44169_d_n4, assign29850_e44169_d_n5, assign29850_e44169_d_b0, assign29850_e44169_d_b1, assign29850_e44169_d_b2, assign29850_e44169_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29850_e44168: f64 = (-var_tmf2);
                (assign29850_e44168, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n1, assign29850_e44169_d_n2, assign29850_e44169_d_n3, assign29850_e44169_d_n4, assign29850_e44169_d_n5, assign29850_e44169_d_b0, assign29850_e44169_d_b1, assign29850_e44169_d_b2, assign29850_e44169_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29850_e44171;
        var_tmf2_dn0 = assign29850_e44171_d_n0;
        var_tmf2_dn1 = assign29850_e44171_d_n1;
        var_tmf2_dn2 = assign29850_e44171_d_n2;
        var_tmf2_dn3 = assign29850_e44171_d_n3;
        var_tmf2_dn4 = assign29850_e44171_d_n4;
        var_tmf2_dn5 = assign29850_e44171_d_n5;
        var_tmf2_db0 = assign29850_e44171_d_b0;
        var_tmf2_db1 = assign29850_e44171_d_b1;
        var_tmf2_db2 = assign29850_e44171_d_b2;
        var_tmf2_db3 = assign29850_e44171_d_b3;

        let (assign29860_e44187, assign29860_e44187_d_n0, assign29860_e44187_d_n1, assign29860_e44187_d_n2, assign29860_e44187_d_n3, assign29860_e44187_d_n4, assign29860_e44187_d_n5, assign29860_e44187_d_b0, assign29860_e44187_d_b1, assign29860_e44187_d_b2, assign29860_e44187_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29860_e44182: f64 = (var_tmf1 * var_tmf1);
        let assign29860_e44184: f64 = (assign29860_e44182 + var_tmf2);
        let assign29860_e44185: f64 = (assign29860_e44184).sqrt();
        (assign29860_e44185, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29860_e44185)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29860_e44185)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29860_e44185)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29860_e44185)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29860_e44185)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29860_e44187;
        var_tmf2_dn0 = assign29860_e44187_d_n0;
        var_tmf2_dn1 = assign29860_e44187_d_n1;
        var_tmf2_dn2 = assign29860_e44187_d_n2;
        var_tmf2_dn3 = assign29860_e44187_d_n3;
        var_tmf2_dn4 = assign29860_e44187_d_n4;
        var_tmf2_dn5 = assign29860_e44187_d_n5;
        var_tmf2_db0 = assign29860_e44187_d_b0;
        var_tmf2_db1 = assign29860_e44187_d_b1;
        var_tmf2_db2 = assign29860_e44187_d_b2;
        var_tmf2_db3 = assign29860_e44187_d_b3;

        let (assign29870_e44204, assign29870_e44204_d_n0, assign29870_e44204_d_n1, assign29870_e44204_d_n2, assign29870_e44204_d_n3, assign29870_e44204_d_n4, assign29870_e44204_d_n5, assign29870_e44204_d_b0, assign29870_e44204_d_b1, assign29870_e44204_d_b2, assign29870_e44204_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29870_e44200: f64 = (var_tmf1 + var_tmf2);
        let assign29870_e44201: f64 = (0.5 * assign29870_e44200);
        let assign29870_e44202: f64 = (p.p85 - assign29870_e44201);
        (assign29870_e44202, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29870_e44204;
        var_nj0_dn0 = assign29870_e44204_d_n0;
        var_nj0_dn1 = assign29870_e44204_d_n1;
        var_nj0_dn2 = assign29870_e44204_d_n2;
        var_nj0_dn3 = assign29870_e44204_d_n3;
        var_nj0_dn4 = assign29870_e44204_d_n4;
        var_nj0_dn5 = assign29870_e44204_d_n5;
        var_nj0_db0 = assign29870_e44204_d_b0;
        var_nj0_db1 = assign29870_e44204_d_b1;
        var_nj0_db2 = assign29870_e44204_d_b2;
        var_nj0_db3 = assign29870_e44204_d_b3;

        let (assign29880_e44219, assign29880_e44219_d_n0, assign29880_e44219_d_n1, assign29880_e44219_d_n2, assign29880_e44219_d_n3, assign29880_e44219_d_n4, assign29880_e44219_d_n5, assign29880_e44219_d_b0, assign29880_e44219_d_b1, assign29880_e44219_d_b2, assign29880_e44219_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29880_e44215: f64 = (var_nj0 - var_nfasti_i);
        let assign29880_e44217: f64 = (assign29880_e44215 - 0.01);
        (assign29880_e44217, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign29880_e44219;
        var_tmf1_dn0 = assign29880_e44219_d_n0;
        var_tmf1_dn1 = assign29880_e44219_d_n1;
        var_tmf1_dn2 = assign29880_e44219_d_n2;
        var_tmf1_dn3 = assign29880_e44219_d_n3;
        var_tmf1_dn4 = assign29880_e44219_d_n4;
        var_tmf1_dn5 = assign29880_e44219_d_n5;
        var_tmf1_db0 = assign29880_e44219_d_b0;
        var_tmf1_db1 = assign29880_e44219_d_b1;
        var_tmf1_db2 = assign29880_e44219_d_b2;
        var_tmf1_db3 = assign29880_e44219_d_b3;

        let (assign29890_e44234, assign29890_e44234_d_n0, assign29890_e44234_d_n1, assign29890_e44234_d_n2, assign29890_e44234_d_n3, assign29890_e44234_d_n4, assign29890_e44234_d_n5, assign29890_e44234_d_b0, assign29890_e44234_d_b1, assign29890_e44234_d_b2, assign29890_e44234_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29890_e44230: f64 = (4.0 * var_nfasti_i);
        let assign29890_e44232: f64 = (assign29890_e44230 * 0.01);
        (assign29890_e44232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29890_e44234;
        var_tmf2_dn0 = assign29890_e44234_d_n0;
        var_tmf2_dn1 = assign29890_e44234_d_n1;
        var_tmf2_dn2 = assign29890_e44234_d_n2;
        var_tmf2_dn3 = assign29890_e44234_d_n3;
        var_tmf2_dn4 = assign29890_e44234_d_n4;
        var_tmf2_dn5 = assign29890_e44234_d_n5;
        var_tmf2_db0 = assign29890_e44234_d_b0;
        var_tmf2_db1 = assign29890_e44234_d_b1;
        var_tmf2_db2 = assign29890_e44234_d_b2;
        var_tmf2_db3 = assign29890_e44234_d_b3;

        let (assign29900_e44251, assign29900_e44251_d_n0, assign29900_e44251_d_n1, assign29900_e44251_d_n2, assign29900_e44251_d_n3, assign29900_e44251_d_n4, assign29900_e44251_d_n5, assign29900_e44251_d_b0, assign29900_e44251_d_b1, assign29900_e44251_d_b2, assign29900_e44251_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n1, assign29900_e44249_d_n2, assign29900_e44249_d_n3, assign29900_e44249_d_n4, assign29900_e44249_d_n5, assign29900_e44249_d_b0, assign29900_e44249_d_b1, assign29900_e44249_d_b2, assign29900_e44249_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign29900_e44248: f64 = (-var_tmf2);
                (assign29900_e44248, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n1, assign29900_e44249_d_n2, assign29900_e44249_d_n3, assign29900_e44249_d_n4, assign29900_e44249_d_n5, assign29900_e44249_d_b0, assign29900_e44249_d_b1, assign29900_e44249_d_b2, assign29900_e44249_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29900_e44251;
        var_tmf2_dn0 = assign29900_e44251_d_n0;
        var_tmf2_dn1 = assign29900_e44251_d_n1;
        var_tmf2_dn2 = assign29900_e44251_d_n2;
        var_tmf2_dn3 = assign29900_e44251_d_n3;
        var_tmf2_dn4 = assign29900_e44251_d_n4;
        var_tmf2_dn5 = assign29900_e44251_d_n5;
        var_tmf2_db0 = assign29900_e44251_d_b0;
        var_tmf2_db1 = assign29900_e44251_d_b1;
        var_tmf2_db2 = assign29900_e44251_d_b2;
        var_tmf2_db3 = assign29900_e44251_d_b3;

        let (assign29910_e44267, assign29910_e44267_d_n0, assign29910_e44267_d_n1, assign29910_e44267_d_n2, assign29910_e44267_d_n3, assign29910_e44267_d_n4, assign29910_e44267_d_n5, assign29910_e44267_d_b0, assign29910_e44267_d_b1, assign29910_e44267_d_b2, assign29910_e44267_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29910_e44262: f64 = (var_tmf1 * var_tmf1);
        let assign29910_e44264: f64 = (assign29910_e44262 + var_tmf2);
        let assign29910_e44265: f64 = (assign29910_e44264).sqrt();
        (assign29910_e44265, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign29910_e44265)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign29910_e44265)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign29910_e44265)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign29910_e44265)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign29910_e44265)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign29910_e44267;
        var_tmf2_dn0 = assign29910_e44267_d_n0;
        var_tmf2_dn1 = assign29910_e44267_d_n1;
        var_tmf2_dn2 = assign29910_e44267_d_n2;
        var_tmf2_dn3 = assign29910_e44267_d_n3;
        var_tmf2_dn4 = assign29910_e44267_d_n4;
        var_tmf2_dn5 = assign29910_e44267_d_n5;
        var_tmf2_db0 = assign29910_e44267_d_b0;
        var_tmf2_db1 = assign29910_e44267_d_b1;
        var_tmf2_db2 = assign29910_e44267_d_b2;
        var_tmf2_db3 = assign29910_e44267_d_b3;

        let (assign29920_e44284, assign29920_e44284_d_n0, assign29920_e44284_d_n1, assign29920_e44284_d_n2, assign29920_e44284_d_n3, assign29920_e44284_d_n4, assign29920_e44284_d_n5, assign29920_e44284_d_b0, assign29920_e44284_d_b1, assign29920_e44284_d_b2, assign29920_e44284_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29920_e44280: f64 = (var_tmf1 + var_tmf2);
        let assign29920_e44281: f64 = (0.5 * assign29920_e44280);
        let assign29920_e44282: f64 = (var_nfasti_i + assign29920_e44281);
        (assign29920_e44282, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29920_e44284;
        var_nj0_dn0 = assign29920_e44284_d_n0;
        var_nj0_dn1 = assign29920_e44284_d_n1;
        var_nj0_dn2 = assign29920_e44284_d_n2;
        var_nj0_dn3 = assign29920_e44284_d_n3;
        var_nj0_dn4 = assign29920_e44284_d_n4;
        var_nj0_dn5 = assign29920_e44284_d_n5;
        var_nj0_db0 = assign29920_e44284_d_b0;
        var_nj0_db1 = assign29920_e44284_d_b1;
        var_nj0_db2 = assign29920_e44284_d_b2;
        var_nj0_db3 = assign29920_e44284_d_b3;

        let (assign29930_e44296, assign29930_e44296_d_n0, assign29930_e44296_d_n1, assign29930_e44296_d_n2, assign29930_e44296_d_n3, assign29930_e44296_d_n4, assign29930_e44296_d_n5, assign29930_e44296_d_b0, assign29930_e44296_d_b1, assign29930_e44296_d_b2, assign29930_e44296_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign29930_e44296;
        var_nj0_dn0 = assign29930_e44296_d_n0;
        var_nj0_dn1 = assign29930_e44296_d_n1;
        var_nj0_dn2 = assign29930_e44296_d_n2;
        var_nj0_dn3 = assign29930_e44296_d_n3;
        var_nj0_dn4 = assign29930_e44296_d_n4;
        var_nj0_dn5 = assign29930_e44296_d_n5;
        var_nj0_db0 = assign29930_e44296_d_b0;
        var_nj0_db1 = assign29930_e44296_d_b1;
        var_nj0_db2 = assign29930_e44296_d_b2;
        var_nj0_db3 = assign29930_e44296_d_b3;

        let (assign29940_e44308, assign29940_e44308_d_n0, assign29940_e44308_d_n1, assign29940_e44308_d_n2, assign29940_e44308_d_n3, assign29940_e44308_d_n4, assign29940_e44308_d_n5, assign29940_e44308_d_b0, assign29940_e44308_d_b1, assign29940_e44308_d_b2, assign29940_e44308_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign29940_e44308;
        var_nj1_dn0 = assign29940_e44308_d_n0;
        var_nj1_dn1 = assign29940_e44308_d_n1;
        var_nj1_dn2 = assign29940_e44308_d_n2;
        var_nj1_dn3 = assign29940_e44308_d_n3;
        var_nj1_dn4 = assign29940_e44308_d_n4;
        var_nj1_dn5 = assign29940_e44308_d_n5;
        var_nj1_db0 = assign29940_e44308_d_b0;
        var_nj1_db1 = assign29940_e44308_d_b1;
        var_nj1_db2 = assign29940_e44308_d_b2;
        var_nj1_db3 = assign29940_e44308_d_b3;


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

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
        var_guard489_slot: &mut f64,
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
        let mut var_guard489: f64 = *var_guard489_slot;
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

        let (assign30000_e44543,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign30000_e44539: f64 = (var_nin * var_nin);
        let assign30000_e44541: f64 = (assign30000_e44539 / var_ndigat_i);
        (assign30000_e44541,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30000_e44543;

        let (assign30010_e44559,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign30010_e44552: f64 = (var_nfagat_i / var_phitdinv);
        let assign30010_e44555: f64 = (var_ndigat_i / var_pnn0);
        let assign30010_e44556: f64 = (assign30010_e44555).ln();
        let assign30010_e44557: f64 = (assign30010_e44552 * assign30010_e44556);
        (assign30010_e44557,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30010_e44559;

        let assign30020_e44562: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard489 = assign30020_e44562;

        let (assign30030_e44579, assign30030_e44579_d_n0, assign30030_e44579_d_n1, assign30030_e44579_d_n2, assign30030_e44579_d_n3, assign30030_e44579_d_n4, assign30030_e44579_d_n5, assign30030_e44579_d_b0, assign30030_e44579_d_b1, assign30030_e44579_d_b2, assign30030_e44579_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30030_e44574: f64 = (var_vak - var_vha1);
        let assign30030_e44575: f64 = (p.p86 * assign30030_e44574);
        let assign30030_e44577: f64 = (assign30030_e44575 + var_nfagat_i);
        (assign30030_e44577, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn1), (p.p86 * var_vak_dn2), (p.p86 * var_vak_dn3), (p.p86 * var_vak_dn4), (p.p86 * var_vak_dn5), (p.p86 * var_vak_db0), (p.p86 * var_vak_db1), (p.p86 * var_vak_db2), (p.p86 * var_vak_db3),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign30030_e44579;
        var_nja10_dn0 = assign30030_e44579_d_n0;
        var_nja10_dn1 = assign30030_e44579_d_n1;
        var_nja10_dn2 = assign30030_e44579_d_n2;
        var_nja10_dn3 = assign30030_e44579_d_n3;
        var_nja10_dn4 = assign30030_e44579_d_n4;
        var_nja10_dn5 = assign30030_e44579_d_n5;
        var_nja10_db0 = assign30030_e44579_d_b0;
        var_nja10_db1 = assign30030_e44579_d_b1;
        var_nja10_db2 = assign30030_e44579_d_b2;
        var_nja10_db3 = assign30030_e44579_d_b3;

        let (assign30040_e44594, assign30040_e44594_d_n0, assign30040_e44594_d_n1, assign30040_e44594_d_n2, assign30040_e44594_d_n3, assign30040_e44594_d_n4, assign30040_e44594_d_n5, assign30040_e44594_d_b0, assign30040_e44594_d_b1, assign30040_e44594_d_b2, assign30040_e44594_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30040_e44591: f64 = (p.p86 * var_vha1);
        let assign30040_e44592: f64 = (var_nfagat_i - assign30040_e44591);
        (assign30040_e44592, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30040_e44594;
        var_nj0_dn0 = assign30040_e44594_d_n0;
        var_nj0_dn1 = assign30040_e44594_d_n1;
        var_nj0_dn2 = assign30040_e44594_d_n2;
        var_nj0_dn3 = assign30040_e44594_d_n3;
        var_nj0_dn4 = assign30040_e44594_d_n4;
        var_nj0_dn5 = assign30040_e44594_d_n5;
        var_nj0_db0 = assign30040_e44594_d_b0;
        var_nj0_db1 = assign30040_e44594_d_b1;
        var_nj0_db2 = assign30040_e44594_d_b2;
        var_nj0_db3 = assign30040_e44594_d_b3;

        let (assign30050_e44609, assign30050_e44609_d_n0, assign30050_e44609_d_n1, assign30050_e44609_d_n2, assign30050_e44609_d_n3, assign30050_e44609_d_n4, assign30050_e44609_d_n5, assign30050_e44609_d_b0, assign30050_e44609_d_b1, assign30050_e44609_d_b2, assign30050_e44609_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30050_e44605: f64 = (p.p85 - var_nja10);
        let assign30050_e44607: f64 = (assign30050_e44605 - 0.01);
        (assign30050_e44607, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30050_e44609;
        var_tmf1_dn0 = assign30050_e44609_d_n0;
        var_tmf1_dn1 = assign30050_e44609_d_n1;
        var_tmf1_dn2 = assign30050_e44609_d_n2;
        var_tmf1_dn3 = assign30050_e44609_d_n3;
        var_tmf1_dn4 = assign30050_e44609_d_n4;
        var_tmf1_dn5 = assign30050_e44609_d_n5;
        var_tmf1_db0 = assign30050_e44609_d_b0;
        var_tmf1_db1 = assign30050_e44609_d_b1;
        var_tmf1_db2 = assign30050_e44609_d_b2;
        var_tmf1_db3 = assign30050_e44609_d_b3;

        let (assign30060_e44624, assign30060_e44624_d_n0, assign30060_e44624_d_n1, assign30060_e44624_d_n2, assign30060_e44624_d_n3, assign30060_e44624_d_n4, assign30060_e44624_d_n5, assign30060_e44624_d_b0, assign30060_e44624_d_b1, assign30060_e44624_d_b2, assign30060_e44624_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30060_e44620: f64 = (4.0 * p.p85);
        let assign30060_e44622: f64 = (assign30060_e44620 * 0.01);
        (assign30060_e44622, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30060_e44624;
        var_tmf2_dn0 = assign30060_e44624_d_n0;
        var_tmf2_dn1 = assign30060_e44624_d_n1;
        var_tmf2_dn2 = assign30060_e44624_d_n2;
        var_tmf2_dn3 = assign30060_e44624_d_n3;
        var_tmf2_dn4 = assign30060_e44624_d_n4;
        var_tmf2_dn5 = assign30060_e44624_d_n5;
        var_tmf2_db0 = assign30060_e44624_d_b0;
        var_tmf2_db1 = assign30060_e44624_d_b1;
        var_tmf2_db2 = assign30060_e44624_d_b2;
        var_tmf2_db3 = assign30060_e44624_d_b3;

        let (assign30070_e44641, assign30070_e44641_d_n0, assign30070_e44641_d_n1, assign30070_e44641_d_n2, assign30070_e44641_d_n3, assign30070_e44641_d_n4, assign30070_e44641_d_n5, assign30070_e44641_d_b0, assign30070_e44641_d_b1, assign30070_e44641_d_b2, assign30070_e44641_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n1, assign30070_e44639_d_n2, assign30070_e44639_d_n3, assign30070_e44639_d_n4, assign30070_e44639_d_n5, assign30070_e44639_d_b0, assign30070_e44639_d_b1, assign30070_e44639_d_b2, assign30070_e44639_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30070_e44638: f64 = (-var_tmf2);
                (assign30070_e44638, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n1, assign30070_e44639_d_n2, assign30070_e44639_d_n3, assign30070_e44639_d_n4, assign30070_e44639_d_n5, assign30070_e44639_d_b0, assign30070_e44639_d_b1, assign30070_e44639_d_b2, assign30070_e44639_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30070_e44641;
        var_tmf2_dn0 = assign30070_e44641_d_n0;
        var_tmf2_dn1 = assign30070_e44641_d_n1;
        var_tmf2_dn2 = assign30070_e44641_d_n2;
        var_tmf2_dn3 = assign30070_e44641_d_n3;
        var_tmf2_dn4 = assign30070_e44641_d_n4;
        var_tmf2_dn5 = assign30070_e44641_d_n5;
        var_tmf2_db0 = assign30070_e44641_d_b0;
        var_tmf2_db1 = assign30070_e44641_d_b1;
        var_tmf2_db2 = assign30070_e44641_d_b2;
        var_tmf2_db3 = assign30070_e44641_d_b3;

        let (assign30080_e44657, assign30080_e44657_d_n0, assign30080_e44657_d_n1, assign30080_e44657_d_n2, assign30080_e44657_d_n3, assign30080_e44657_d_n4, assign30080_e44657_d_n5, assign30080_e44657_d_b0, assign30080_e44657_d_b1, assign30080_e44657_d_b2, assign30080_e44657_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30080_e44652: f64 = (var_tmf1 * var_tmf1);
        let assign30080_e44654: f64 = (assign30080_e44652 + var_tmf2);
        let assign30080_e44655: f64 = (assign30080_e44654).sqrt();
        (assign30080_e44655, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30080_e44655)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30080_e44655)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30080_e44655)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30080_e44655)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30080_e44655)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30080_e44657;
        var_tmf2_dn0 = assign30080_e44657_d_n0;
        var_tmf2_dn1 = assign30080_e44657_d_n1;
        var_tmf2_dn2 = assign30080_e44657_d_n2;
        var_tmf2_dn3 = assign30080_e44657_d_n3;
        var_tmf2_dn4 = assign30080_e44657_d_n4;
        var_tmf2_dn5 = assign30080_e44657_d_n5;
        var_tmf2_db0 = assign30080_e44657_d_b0;
        var_tmf2_db1 = assign30080_e44657_d_b1;
        var_tmf2_db2 = assign30080_e44657_d_b2;
        var_tmf2_db3 = assign30080_e44657_d_b3;

        let (assign30090_e44674, assign30090_e44674_d_n0, assign30090_e44674_d_n1, assign30090_e44674_d_n2, assign30090_e44674_d_n3, assign30090_e44674_d_n4, assign30090_e44674_d_n5, assign30090_e44674_d_b0, assign30090_e44674_d_b1, assign30090_e44674_d_b2, assign30090_e44674_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30090_e44670: f64 = (var_tmf1 + var_tmf2);
        let assign30090_e44671: f64 = (0.5 * assign30090_e44670);
        let assign30090_e44672: f64 = (p.p85 - assign30090_e44671);
        (assign30090_e44672, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign30090_e44674;
        var_nja11_dn0 = assign30090_e44674_d_n0;
        var_nja11_dn1 = assign30090_e44674_d_n1;
        var_nja11_dn2 = assign30090_e44674_d_n2;
        var_nja11_dn3 = assign30090_e44674_d_n3;
        var_nja11_dn4 = assign30090_e44674_d_n4;
        var_nja11_dn5 = assign30090_e44674_d_n5;
        var_nja11_db0 = assign30090_e44674_d_b0;
        var_nja11_db1 = assign30090_e44674_d_b1;
        var_nja11_db2 = assign30090_e44674_d_b2;
        var_nja11_db3 = assign30090_e44674_d_b3;

        let (assign30100_e44689, assign30100_e44689_d_n0, assign30100_e44689_d_n1, assign30100_e44689_d_n2, assign30100_e44689_d_n3, assign30100_e44689_d_n4, assign30100_e44689_d_n5, assign30100_e44689_d_b0, assign30100_e44689_d_b1, assign30100_e44689_d_b2, assign30100_e44689_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30100_e44685: f64 = (var_nja11 - var_nfagat_i);
        let assign30100_e44687: f64 = (assign30100_e44685 - 0.01);
        (assign30100_e44687, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30100_e44689;
        var_tmf1_dn0 = assign30100_e44689_d_n0;
        var_tmf1_dn1 = assign30100_e44689_d_n1;
        var_tmf1_dn2 = assign30100_e44689_d_n2;
        var_tmf1_dn3 = assign30100_e44689_d_n3;
        var_tmf1_dn4 = assign30100_e44689_d_n4;
        var_tmf1_dn5 = assign30100_e44689_d_n5;
        var_tmf1_db0 = assign30100_e44689_d_b0;
        var_tmf1_db1 = assign30100_e44689_d_b1;
        var_tmf1_db2 = assign30100_e44689_d_b2;
        var_tmf1_db3 = assign30100_e44689_d_b3;

        let (assign30110_e44704, assign30110_e44704_d_n0, assign30110_e44704_d_n1, assign30110_e44704_d_n2, assign30110_e44704_d_n3, assign30110_e44704_d_n4, assign30110_e44704_d_n5, assign30110_e44704_d_b0, assign30110_e44704_d_b1, assign30110_e44704_d_b2, assign30110_e44704_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30110_e44700: f64 = (4.0 * var_nfagat_i);
        let assign30110_e44702: f64 = (assign30110_e44700 * 0.01);
        (assign30110_e44702, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30110_e44704;
        var_tmf2_dn0 = assign30110_e44704_d_n0;
        var_tmf2_dn1 = assign30110_e44704_d_n1;
        var_tmf2_dn2 = assign30110_e44704_d_n2;
        var_tmf2_dn3 = assign30110_e44704_d_n3;
        var_tmf2_dn4 = assign30110_e44704_d_n4;
        var_tmf2_dn5 = assign30110_e44704_d_n5;
        var_tmf2_db0 = assign30110_e44704_d_b0;
        var_tmf2_db1 = assign30110_e44704_d_b1;
        var_tmf2_db2 = assign30110_e44704_d_b2;
        var_tmf2_db3 = assign30110_e44704_d_b3;

        let (assign30120_e44721, assign30120_e44721_d_n0, assign30120_e44721_d_n1, assign30120_e44721_d_n2, assign30120_e44721_d_n3, assign30120_e44721_d_n4, assign30120_e44721_d_n5, assign30120_e44721_d_b0, assign30120_e44721_d_b1, assign30120_e44721_d_b2, assign30120_e44721_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n1, assign30120_e44719_d_n2, assign30120_e44719_d_n3, assign30120_e44719_d_n4, assign30120_e44719_d_n5, assign30120_e44719_d_b0, assign30120_e44719_d_b1, assign30120_e44719_d_b2, assign30120_e44719_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30120_e44718: f64 = (-var_tmf2);
                (assign30120_e44718, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n1, assign30120_e44719_d_n2, assign30120_e44719_d_n3, assign30120_e44719_d_n4, assign30120_e44719_d_n5, assign30120_e44719_d_b0, assign30120_e44719_d_b1, assign30120_e44719_d_b2, assign30120_e44719_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30120_e44721;
        var_tmf2_dn0 = assign30120_e44721_d_n0;
        var_tmf2_dn1 = assign30120_e44721_d_n1;
        var_tmf2_dn2 = assign30120_e44721_d_n2;
        var_tmf2_dn3 = assign30120_e44721_d_n3;
        var_tmf2_dn4 = assign30120_e44721_d_n4;
        var_tmf2_dn5 = assign30120_e44721_d_n5;
        var_tmf2_db0 = assign30120_e44721_d_b0;
        var_tmf2_db1 = assign30120_e44721_d_b1;
        var_tmf2_db2 = assign30120_e44721_d_b2;
        var_tmf2_db3 = assign30120_e44721_d_b3;

        let (assign30130_e44737, assign30130_e44737_d_n0, assign30130_e44737_d_n1, assign30130_e44737_d_n2, assign30130_e44737_d_n3, assign30130_e44737_d_n4, assign30130_e44737_d_n5, assign30130_e44737_d_b0, assign30130_e44737_d_b1, assign30130_e44737_d_b2, assign30130_e44737_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30130_e44732: f64 = (var_tmf1 * var_tmf1);
        let assign30130_e44734: f64 = (assign30130_e44732 + var_tmf2);
        let assign30130_e44735: f64 = (assign30130_e44734).sqrt();
        (assign30130_e44735, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30130_e44735)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30130_e44735)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30130_e44735)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30130_e44735)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30130_e44735)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30130_e44737;
        var_tmf2_dn0 = assign30130_e44737_d_n0;
        var_tmf2_dn1 = assign30130_e44737_d_n1;
        var_tmf2_dn2 = assign30130_e44737_d_n2;
        var_tmf2_dn3 = assign30130_e44737_d_n3;
        var_tmf2_dn4 = assign30130_e44737_d_n4;
        var_tmf2_dn5 = assign30130_e44737_d_n5;
        var_tmf2_db0 = assign30130_e44737_d_b0;
        var_tmf2_db1 = assign30130_e44737_d_b1;
        var_tmf2_db2 = assign30130_e44737_d_b2;
        var_tmf2_db3 = assign30130_e44737_d_b3;

        let (assign30140_e44754, assign30140_e44754_d_n0, assign30140_e44754_d_n1, assign30140_e44754_d_n2, assign30140_e44754_d_n3, assign30140_e44754_d_n4, assign30140_e44754_d_n5, assign30140_e44754_d_b0, assign30140_e44754_d_b1, assign30140_e44754_d_b2, assign30140_e44754_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30140_e44750: f64 = (var_tmf1 + var_tmf2);
        let assign30140_e44751: f64 = (0.5 * assign30140_e44750);
        let assign30140_e44752: f64 = (var_nfagat_i + assign30140_e44751);
        (assign30140_e44752, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30140_e44754;
        var_nj1_dn0 = assign30140_e44754_d_n0;
        var_nj1_dn1 = assign30140_e44754_d_n1;
        var_nj1_dn2 = assign30140_e44754_d_n2;
        var_nj1_dn3 = assign30140_e44754_d_n3;
        var_nj1_dn4 = assign30140_e44754_d_n4;
        var_nj1_dn5 = assign30140_e44754_d_n5;
        var_nj1_db0 = assign30140_e44754_d_b0;
        var_nj1_db1 = assign30140_e44754_d_b1;
        var_nj1_db2 = assign30140_e44754_d_b2;
        var_nj1_db3 = assign30140_e44754_d_b3;

        let (assign30150_e44769, assign30150_e44769_d_n0, assign30150_e44769_d_n1, assign30150_e44769_d_n2, assign30150_e44769_d_n3, assign30150_e44769_d_n4, assign30150_e44769_d_n5, assign30150_e44769_d_b0, assign30150_e44769_d_b1, assign30150_e44769_d_b2, assign30150_e44769_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30150_e44765: f64 = (p.p85 - var_nj0);
        let assign30150_e44767: f64 = (assign30150_e44765 - 0.01);
        (assign30150_e44767, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30150_e44769;
        var_tmf1_dn0 = assign30150_e44769_d_n0;
        var_tmf1_dn1 = assign30150_e44769_d_n1;
        var_tmf1_dn2 = assign30150_e44769_d_n2;
        var_tmf1_dn3 = assign30150_e44769_d_n3;
        var_tmf1_dn4 = assign30150_e44769_d_n4;
        var_tmf1_dn5 = assign30150_e44769_d_n5;
        var_tmf1_db0 = assign30150_e44769_d_b0;
        var_tmf1_db1 = assign30150_e44769_d_b1;
        var_tmf1_db2 = assign30150_e44769_d_b2;
        var_tmf1_db3 = assign30150_e44769_d_b3;

        let (assign30160_e44784, assign30160_e44784_d_n0, assign30160_e44784_d_n1, assign30160_e44784_d_n2, assign30160_e44784_d_n3, assign30160_e44784_d_n4, assign30160_e44784_d_n5, assign30160_e44784_d_b0, assign30160_e44784_d_b1, assign30160_e44784_d_b2, assign30160_e44784_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30160_e44780: f64 = (4.0 * p.p85);
        let assign30160_e44782: f64 = (assign30160_e44780 * 0.01);
        (assign30160_e44782, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30160_e44784;
        var_tmf2_dn0 = assign30160_e44784_d_n0;
        var_tmf2_dn1 = assign30160_e44784_d_n1;
        var_tmf2_dn2 = assign30160_e44784_d_n2;
        var_tmf2_dn3 = assign30160_e44784_d_n3;
        var_tmf2_dn4 = assign30160_e44784_d_n4;
        var_tmf2_dn5 = assign30160_e44784_d_n5;
        var_tmf2_db0 = assign30160_e44784_d_b0;
        var_tmf2_db1 = assign30160_e44784_d_b1;
        var_tmf2_db2 = assign30160_e44784_d_b2;
        var_tmf2_db3 = assign30160_e44784_d_b3;

        let (assign30170_e44801, assign30170_e44801_d_n0, assign30170_e44801_d_n1, assign30170_e44801_d_n2, assign30170_e44801_d_n3, assign30170_e44801_d_n4, assign30170_e44801_d_n5, assign30170_e44801_d_b0, assign30170_e44801_d_b1, assign30170_e44801_d_b2, assign30170_e44801_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n1, assign30170_e44799_d_n2, assign30170_e44799_d_n3, assign30170_e44799_d_n4, assign30170_e44799_d_n5, assign30170_e44799_d_b0, assign30170_e44799_d_b1, assign30170_e44799_d_b2, assign30170_e44799_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30170_e44798: f64 = (-var_tmf2);
                (assign30170_e44798, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n1, assign30170_e44799_d_n2, assign30170_e44799_d_n3, assign30170_e44799_d_n4, assign30170_e44799_d_n5, assign30170_e44799_d_b0, assign30170_e44799_d_b1, assign30170_e44799_d_b2, assign30170_e44799_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30170_e44801;
        var_tmf2_dn0 = assign30170_e44801_d_n0;
        var_tmf2_dn1 = assign30170_e44801_d_n1;
        var_tmf2_dn2 = assign30170_e44801_d_n2;
        var_tmf2_dn3 = assign30170_e44801_d_n3;
        var_tmf2_dn4 = assign30170_e44801_d_n4;
        var_tmf2_dn5 = assign30170_e44801_d_n5;
        var_tmf2_db0 = assign30170_e44801_d_b0;
        var_tmf2_db1 = assign30170_e44801_d_b1;
        var_tmf2_db2 = assign30170_e44801_d_b2;
        var_tmf2_db3 = assign30170_e44801_d_b3;

        let (assign30180_e44817, assign30180_e44817_d_n0, assign30180_e44817_d_n1, assign30180_e44817_d_n2, assign30180_e44817_d_n3, assign30180_e44817_d_n4, assign30180_e44817_d_n5, assign30180_e44817_d_b0, assign30180_e44817_d_b1, assign30180_e44817_d_b2, assign30180_e44817_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30180_e44812: f64 = (var_tmf1 * var_tmf1);
        let assign30180_e44814: f64 = (assign30180_e44812 + var_tmf2);
        let assign30180_e44815: f64 = (assign30180_e44814).sqrt();
        (assign30180_e44815, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30180_e44815)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30180_e44815)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30180_e44815)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30180_e44815)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30180_e44815)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30180_e44817;
        var_tmf2_dn0 = assign30180_e44817_d_n0;
        var_tmf2_dn1 = assign30180_e44817_d_n1;
        var_tmf2_dn2 = assign30180_e44817_d_n2;
        var_tmf2_dn3 = assign30180_e44817_d_n3;
        var_tmf2_dn4 = assign30180_e44817_d_n4;
        var_tmf2_dn5 = assign30180_e44817_d_n5;
        var_tmf2_db0 = assign30180_e44817_d_b0;
        var_tmf2_db1 = assign30180_e44817_d_b1;
        var_tmf2_db2 = assign30180_e44817_d_b2;
        var_tmf2_db3 = assign30180_e44817_d_b3;

        let (assign30190_e44834, assign30190_e44834_d_n0, assign30190_e44834_d_n1, assign30190_e44834_d_n2, assign30190_e44834_d_n3, assign30190_e44834_d_n4, assign30190_e44834_d_n5, assign30190_e44834_d_b0, assign30190_e44834_d_b1, assign30190_e44834_d_b2, assign30190_e44834_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30190_e44830: f64 = (var_tmf1 + var_tmf2);
        let assign30190_e44831: f64 = (0.5 * assign30190_e44830);
        let assign30190_e44832: f64 = (p.p85 - assign30190_e44831);
        (assign30190_e44832, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30190_e44834;
        var_nj0_dn0 = assign30190_e44834_d_n0;
        var_nj0_dn1 = assign30190_e44834_d_n1;
        var_nj0_dn2 = assign30190_e44834_d_n2;
        var_nj0_dn3 = assign30190_e44834_d_n3;
        var_nj0_dn4 = assign30190_e44834_d_n4;
        var_nj0_dn5 = assign30190_e44834_d_n5;
        var_nj0_db0 = assign30190_e44834_d_b0;
        var_nj0_db1 = assign30190_e44834_d_b1;
        var_nj0_db2 = assign30190_e44834_d_b2;
        var_nj0_db3 = assign30190_e44834_d_b3;

        let (assign30200_e44849, assign30200_e44849_d_n0, assign30200_e44849_d_n1, assign30200_e44849_d_n2, assign30200_e44849_d_n3, assign30200_e44849_d_n4, assign30200_e44849_d_n5, assign30200_e44849_d_b0, assign30200_e44849_d_b1, assign30200_e44849_d_b2, assign30200_e44849_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30200_e44845: f64 = (var_nj0 - var_nfagat_i);
        let assign30200_e44847: f64 = (assign30200_e44845 - 0.01);
        (assign30200_e44847, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30200_e44849;
        var_tmf1_dn0 = assign30200_e44849_d_n0;
        var_tmf1_dn1 = assign30200_e44849_d_n1;
        var_tmf1_dn2 = assign30200_e44849_d_n2;
        var_tmf1_dn3 = assign30200_e44849_d_n3;
        var_tmf1_dn4 = assign30200_e44849_d_n4;
        var_tmf1_dn5 = assign30200_e44849_d_n5;
        var_tmf1_db0 = assign30200_e44849_d_b0;
        var_tmf1_db1 = assign30200_e44849_d_b1;
        var_tmf1_db2 = assign30200_e44849_d_b2;
        var_tmf1_db3 = assign30200_e44849_d_b3;

        let (assign30210_e44864, assign30210_e44864_d_n0, assign30210_e44864_d_n1, assign30210_e44864_d_n2, assign30210_e44864_d_n3, assign30210_e44864_d_n4, assign30210_e44864_d_n5, assign30210_e44864_d_b0, assign30210_e44864_d_b1, assign30210_e44864_d_b2, assign30210_e44864_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30210_e44860: f64 = (4.0 * var_nfagat_i);
        let assign30210_e44862: f64 = (assign30210_e44860 * 0.01);
        (assign30210_e44862, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30210_e44864;
        var_tmf2_dn0 = assign30210_e44864_d_n0;
        var_tmf2_dn1 = assign30210_e44864_d_n1;
        var_tmf2_dn2 = assign30210_e44864_d_n2;
        var_tmf2_dn3 = assign30210_e44864_d_n3;
        var_tmf2_dn4 = assign30210_e44864_d_n4;
        var_tmf2_dn5 = assign30210_e44864_d_n5;
        var_tmf2_db0 = assign30210_e44864_d_b0;
        var_tmf2_db1 = assign30210_e44864_d_b1;
        var_tmf2_db2 = assign30210_e44864_d_b2;
        var_tmf2_db3 = assign30210_e44864_d_b3;

        let (assign30220_e44881, assign30220_e44881_d_n0, assign30220_e44881_d_n1, assign30220_e44881_d_n2, assign30220_e44881_d_n3, assign30220_e44881_d_n4, assign30220_e44881_d_n5, assign30220_e44881_d_b0, assign30220_e44881_d_b1, assign30220_e44881_d_b2, assign30220_e44881_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n1, assign30220_e44879_d_n2, assign30220_e44879_d_n3, assign30220_e44879_d_n4, assign30220_e44879_d_n5, assign30220_e44879_d_b0, assign30220_e44879_d_b1, assign30220_e44879_d_b2, assign30220_e44879_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30220_e44878: f64 = (-var_tmf2);
                (assign30220_e44878, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n1, assign30220_e44879_d_n2, assign30220_e44879_d_n3, assign30220_e44879_d_n4, assign30220_e44879_d_n5, assign30220_e44879_d_b0, assign30220_e44879_d_b1, assign30220_e44879_d_b2, assign30220_e44879_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30220_e44881;
        var_tmf2_dn0 = assign30220_e44881_d_n0;
        var_tmf2_dn1 = assign30220_e44881_d_n1;
        var_tmf2_dn2 = assign30220_e44881_d_n2;
        var_tmf2_dn3 = assign30220_e44881_d_n3;
        var_tmf2_dn4 = assign30220_e44881_d_n4;
        var_tmf2_dn5 = assign30220_e44881_d_n5;
        var_tmf2_db0 = assign30220_e44881_d_b0;
        var_tmf2_db1 = assign30220_e44881_d_b1;
        var_tmf2_db2 = assign30220_e44881_d_b2;
        var_tmf2_db3 = assign30220_e44881_d_b3;

        let (assign30230_e44897, assign30230_e44897_d_n0, assign30230_e44897_d_n1, assign30230_e44897_d_n2, assign30230_e44897_d_n3, assign30230_e44897_d_n4, assign30230_e44897_d_n5, assign30230_e44897_d_b0, assign30230_e44897_d_b1, assign30230_e44897_d_b2, assign30230_e44897_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30230_e44892: f64 = (var_tmf1 * var_tmf1);
        let assign30230_e44894: f64 = (assign30230_e44892 + var_tmf2);
        let assign30230_e44895: f64 = (assign30230_e44894).sqrt();
        (assign30230_e44895, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30230_e44895)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30230_e44895)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30230_e44895)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30230_e44895)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30230_e44895)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30230_e44897;
        var_tmf2_dn0 = assign30230_e44897_d_n0;
        var_tmf2_dn1 = assign30230_e44897_d_n1;
        var_tmf2_dn2 = assign30230_e44897_d_n2;
        var_tmf2_dn3 = assign30230_e44897_d_n3;
        var_tmf2_dn4 = assign30230_e44897_d_n4;
        var_tmf2_dn5 = assign30230_e44897_d_n5;
        var_tmf2_db0 = assign30230_e44897_d_b0;
        var_tmf2_db1 = assign30230_e44897_d_b1;
        var_tmf2_db2 = assign30230_e44897_d_b2;
        var_tmf2_db3 = assign30230_e44897_d_b3;


        *var_guard489_slot = var_guard489;
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

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard489: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
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
        var_guard492_slot: &mut f64,
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
        let mut var_guard492: f64 = *var_guard492_slot;
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

        let (assign30240_e44914, assign30240_e44914_d_n0, assign30240_e44914_d_n1, assign30240_e44914_d_n2, assign30240_e44914_d_n3, assign30240_e44914_d_n4, assign30240_e44914_d_n5, assign30240_e44914_d_b0, assign30240_e44914_d_b1, assign30240_e44914_d_b2, assign30240_e44914_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30240_e44910: f64 = (var_tmf1 + var_tmf2);
        let assign30240_e44911: f64 = (0.5 * assign30240_e44910);
        let assign30240_e44912: f64 = (var_nfagat_i + assign30240_e44911);
        (assign30240_e44912, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30240_e44914;
        var_nj0_dn0 = assign30240_e44914_d_n0;
        var_nj0_dn1 = assign30240_e44914_d_n1;
        var_nj0_dn2 = assign30240_e44914_d_n2;
        var_nj0_dn3 = assign30240_e44914_d_n3;
        var_nj0_dn4 = assign30240_e44914_d_n4;
        var_nj0_dn5 = assign30240_e44914_d_n5;
        var_nj0_db0 = assign30240_e44914_d_b0;
        var_nj0_db1 = assign30240_e44914_d_b1;
        var_nj0_db2 = assign30240_e44914_d_b2;
        var_nj0_db3 = assign30240_e44914_d_b3;

        let (assign30250_e44926, assign30250_e44926_d_n0, assign30250_e44926_d_n1, assign30250_e44926_d_n2, assign30250_e44926_d_n3, assign30250_e44926_d_n4, assign30250_e44926_d_n5, assign30250_e44926_d_b0, assign30250_e44926_d_b1, assign30250_e44926_d_b2, assign30250_e44926_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30250_e44926;
        var_nj0_dn0 = assign30250_e44926_d_n0;
        var_nj0_dn1 = assign30250_e44926_d_n1;
        var_nj0_dn2 = assign30250_e44926_d_n2;
        var_nj0_dn3 = assign30250_e44926_d_n3;
        var_nj0_dn4 = assign30250_e44926_d_n4;
        var_nj0_dn5 = assign30250_e44926_d_n5;
        var_nj0_db0 = assign30250_e44926_d_b0;
        var_nj0_db1 = assign30250_e44926_d_b1;
        var_nj0_db2 = assign30250_e44926_d_b2;
        var_nj0_db3 = assign30250_e44926_d_b3;

        let (assign30260_e44938, assign30260_e44938_d_n0, assign30260_e44938_d_n1, assign30260_e44938_d_n2, assign30260_e44938_d_n3, assign30260_e44938_d_n4, assign30260_e44938_d_n5, assign30260_e44938_d_b0, assign30260_e44938_d_b1, assign30260_e44938_d_b2, assign30260_e44938_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30260_e44938;
        var_nj1_dn0 = assign30260_e44938_d_n0;
        var_nj1_dn1 = assign30260_e44938_d_n1;
        var_nj1_dn2 = assign30260_e44938_d_n2;
        var_nj1_dn3 = assign30260_e44938_d_n3;
        var_nj1_dn4 = assign30260_e44938_d_n4;
        var_nj1_dn5 = assign30260_e44938_d_n5;
        var_nj1_db0 = assign30260_e44938_d_b0;
        var_nj1_db1 = assign30260_e44938_d_b1;
        var_nj1_db2 = assign30260_e44938_d_b2;
        var_nj1_db3 = assign30260_e44938_d_b3;

        let (assign30330_e45193,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30330_e45189: f64 = (var_nin * var_nin);
        let assign30330_e45191: f64 = (assign30330_e45189 / var_ndibot_i);
        (assign30330_e45191,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30330_e45193;

        let (assign30340_e45210,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30340_e45203: f64 = (var_nfabot_i / var_phitdinv);
        let assign30340_e45206: f64 = (var_ndibot_i / var_pnn0);
        let assign30340_e45207: f64 = (assign30340_e45206).ln();
        let assign30340_e45208: f64 = (assign30340_e45203 * assign30340_e45207);
        (assign30340_e45208,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30340_e45210;

        let assign30350_e45213: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard492 = assign30350_e45213;

        let (assign30360_e45231, assign30360_e45231_d_n0, assign30360_e45231_d_n1, assign30360_e45231_d_n2, assign30360_e45231_d_n3, assign30360_e45231_d_n4, assign30360_e45231_d_n5, assign30360_e45231_d_b0, assign30360_e45231_d_b1, assign30360_e45231_d_b2, assign30360_e45231_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30360_e45226: f64 = (var_vmax - var_vha1);
        let assign30360_e45227: f64 = (p.p86 * assign30360_e45226);
        let assign30360_e45229: f64 = (assign30360_e45227 + var_nfabot_i);
        (assign30360_e45229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign30360_e45231;
        var_nja10_dn0 = assign30360_e45231_d_n0;
        var_nja10_dn1 = assign30360_e45231_d_n1;
        var_nja10_dn2 = assign30360_e45231_d_n2;
        var_nja10_dn3 = assign30360_e45231_d_n3;
        var_nja10_dn4 = assign30360_e45231_d_n4;
        var_nja10_dn5 = assign30360_e45231_d_n5;
        var_nja10_db0 = assign30360_e45231_d_b0;
        var_nja10_db1 = assign30360_e45231_d_b1;
        var_nja10_db2 = assign30360_e45231_d_b2;
        var_nja10_db3 = assign30360_e45231_d_b3;

        let (assign30370_e45247, assign30370_e45247_d_n0, assign30370_e45247_d_n1, assign30370_e45247_d_n2, assign30370_e45247_d_n3, assign30370_e45247_d_n4, assign30370_e45247_d_n5, assign30370_e45247_d_b0, assign30370_e45247_d_b1, assign30370_e45247_d_b2, assign30370_e45247_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30370_e45244: f64 = (p.p86 * var_vha1);
        let assign30370_e45245: f64 = (var_nfabot_i - assign30370_e45244);
        (assign30370_e45245, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30370_e45247;
        var_nj0_dn0 = assign30370_e45247_d_n0;
        var_nj0_dn1 = assign30370_e45247_d_n1;
        var_nj0_dn2 = assign30370_e45247_d_n2;
        var_nj0_dn3 = assign30370_e45247_d_n3;
        var_nj0_dn4 = assign30370_e45247_d_n4;
        var_nj0_dn5 = assign30370_e45247_d_n5;
        var_nj0_db0 = assign30370_e45247_d_b0;
        var_nj0_db1 = assign30370_e45247_d_b1;
        var_nj0_db2 = assign30370_e45247_d_b2;
        var_nj0_db3 = assign30370_e45247_d_b3;

        let (assign30380_e45263, assign30380_e45263_d_n0, assign30380_e45263_d_n1, assign30380_e45263_d_n2, assign30380_e45263_d_n3, assign30380_e45263_d_n4, assign30380_e45263_d_n5, assign30380_e45263_d_b0, assign30380_e45263_d_b1, assign30380_e45263_d_b2, assign30380_e45263_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30380_e45259: f64 = (p.p85 - var_nja10);
        let assign30380_e45261: f64 = (assign30380_e45259 - 0.01);
        (assign30380_e45261, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30380_e45263;
        var_tmf1_dn0 = assign30380_e45263_d_n0;
        var_tmf1_dn1 = assign30380_e45263_d_n1;
        var_tmf1_dn2 = assign30380_e45263_d_n2;
        var_tmf1_dn3 = assign30380_e45263_d_n3;
        var_tmf1_dn4 = assign30380_e45263_d_n4;
        var_tmf1_dn5 = assign30380_e45263_d_n5;
        var_tmf1_db0 = assign30380_e45263_d_b0;
        var_tmf1_db1 = assign30380_e45263_d_b1;
        var_tmf1_db2 = assign30380_e45263_d_b2;
        var_tmf1_db3 = assign30380_e45263_d_b3;

        let (assign30390_e45279, assign30390_e45279_d_n0, assign30390_e45279_d_n1, assign30390_e45279_d_n2, assign30390_e45279_d_n3, assign30390_e45279_d_n4, assign30390_e45279_d_n5, assign30390_e45279_d_b0, assign30390_e45279_d_b1, assign30390_e45279_d_b2, assign30390_e45279_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30390_e45275: f64 = (4.0 * p.p85);
        let assign30390_e45277: f64 = (assign30390_e45275 * 0.01);
        (assign30390_e45277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30390_e45279;
        var_tmf2_dn0 = assign30390_e45279_d_n0;
        var_tmf2_dn1 = assign30390_e45279_d_n1;
        var_tmf2_dn2 = assign30390_e45279_d_n2;
        var_tmf2_dn3 = assign30390_e45279_d_n3;
        var_tmf2_dn4 = assign30390_e45279_d_n4;
        var_tmf2_dn5 = assign30390_e45279_d_n5;
        var_tmf2_db0 = assign30390_e45279_d_b0;
        var_tmf2_db1 = assign30390_e45279_d_b1;
        var_tmf2_db2 = assign30390_e45279_d_b2;
        var_tmf2_db3 = assign30390_e45279_d_b3;

        let (assign30400_e45297, assign30400_e45297_d_n0, assign30400_e45297_d_n1, assign30400_e45297_d_n2, assign30400_e45297_d_n3, assign30400_e45297_d_n4, assign30400_e45297_d_n5, assign30400_e45297_d_b0, assign30400_e45297_d_b1, assign30400_e45297_d_b2, assign30400_e45297_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n1, assign30400_e45295_d_n2, assign30400_e45295_d_n3, assign30400_e45295_d_n4, assign30400_e45295_d_n5, assign30400_e45295_d_b0, assign30400_e45295_d_b1, assign30400_e45295_d_b2, assign30400_e45295_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30400_e45294: f64 = (-var_tmf2);
                (assign30400_e45294, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n1, assign30400_e45295_d_n2, assign30400_e45295_d_n3, assign30400_e45295_d_n4, assign30400_e45295_d_n5, assign30400_e45295_d_b0, assign30400_e45295_d_b1, assign30400_e45295_d_b2, assign30400_e45295_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30400_e45297;
        var_tmf2_dn0 = assign30400_e45297_d_n0;
        var_tmf2_dn1 = assign30400_e45297_d_n1;
        var_tmf2_dn2 = assign30400_e45297_d_n2;
        var_tmf2_dn3 = assign30400_e45297_d_n3;
        var_tmf2_dn4 = assign30400_e45297_d_n4;
        var_tmf2_dn5 = assign30400_e45297_d_n5;
        var_tmf2_db0 = assign30400_e45297_d_b0;
        var_tmf2_db1 = assign30400_e45297_d_b1;
        var_tmf2_db2 = assign30400_e45297_d_b2;
        var_tmf2_db3 = assign30400_e45297_d_b3;

        let (assign30410_e45314, assign30410_e45314_d_n0, assign30410_e45314_d_n1, assign30410_e45314_d_n2, assign30410_e45314_d_n3, assign30410_e45314_d_n4, assign30410_e45314_d_n5, assign30410_e45314_d_b0, assign30410_e45314_d_b1, assign30410_e45314_d_b2, assign30410_e45314_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30410_e45309: f64 = (var_tmf1 * var_tmf1);
        let assign30410_e45311: f64 = (assign30410_e45309 + var_tmf2);
        let assign30410_e45312: f64 = (assign30410_e45311).sqrt();
        (assign30410_e45312, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30410_e45312)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30410_e45312)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30410_e45312)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30410_e45312)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30410_e45312)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30410_e45314;
        var_tmf2_dn0 = assign30410_e45314_d_n0;
        var_tmf2_dn1 = assign30410_e45314_d_n1;
        var_tmf2_dn2 = assign30410_e45314_d_n2;
        var_tmf2_dn3 = assign30410_e45314_d_n3;
        var_tmf2_dn4 = assign30410_e45314_d_n4;
        var_tmf2_dn5 = assign30410_e45314_d_n5;
        var_tmf2_db0 = assign30410_e45314_d_b0;
        var_tmf2_db1 = assign30410_e45314_d_b1;
        var_tmf2_db2 = assign30410_e45314_d_b2;
        var_tmf2_db3 = assign30410_e45314_d_b3;

        let (assign30420_e45332, assign30420_e45332_d_n0, assign30420_e45332_d_n1, assign30420_e45332_d_n2, assign30420_e45332_d_n3, assign30420_e45332_d_n4, assign30420_e45332_d_n5, assign30420_e45332_d_b0, assign30420_e45332_d_b1, assign30420_e45332_d_b2, assign30420_e45332_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30420_e45328: f64 = (var_tmf1 / var_tmf2);
        let assign30420_e45329: f64 = (1.0 + assign30420_e45328);
        let assign30420_e45330: f64 = (0.5 * assign30420_e45329);
        (assign30420_e45330, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign30420_e45332;
        var_dfn_su_dn0 = assign30420_e45332_d_n0;
        var_dfn_su_dn1 = assign30420_e45332_d_n1;
        var_dfn_su_dn2 = assign30420_e45332_d_n2;
        var_dfn_su_dn3 = assign30420_e45332_d_n3;
        var_dfn_su_dn4 = assign30420_e45332_d_n4;
        var_dfn_su_dn5 = assign30420_e45332_d_n5;
        var_dfn_su_db0 = assign30420_e45332_d_b0;
        var_dfn_su_db1 = assign30420_e45332_d_b1;
        var_dfn_su_db2 = assign30420_e45332_d_b2;
        var_dfn_su_db3 = assign30420_e45332_d_b3;

        let (assign30430_e45350, assign30430_e45350_d_n0, assign30430_e45350_d_n1, assign30430_e45350_d_n2, assign30430_e45350_d_n3, assign30430_e45350_d_n4, assign30430_e45350_d_n5, assign30430_e45350_d_b0, assign30430_e45350_d_b1, assign30430_e45350_d_b2, assign30430_e45350_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30430_e45346: f64 = (var_tmf1 + var_tmf2);
        let assign30430_e45347: f64 = (0.5 * assign30430_e45346);
        let assign30430_e45348: f64 = (p.p85 - assign30430_e45347);
        (assign30430_e45348, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign30430_e45350;
        var_nja11_dn0 = assign30430_e45350_d_n0;
        var_nja11_dn1 = assign30430_e45350_d_n1;
        var_nja11_dn2 = assign30430_e45350_d_n2;
        var_nja11_dn3 = assign30430_e45350_d_n3;
        var_nja11_dn4 = assign30430_e45350_d_n4;
        var_nja11_dn5 = assign30430_e45350_d_n5;
        var_nja11_db0 = assign30430_e45350_d_b0;
        var_nja11_db1 = assign30430_e45350_d_b1;
        var_nja11_db2 = assign30430_e45350_d_b2;
        var_nja11_db3 = assign30430_e45350_d_b3;

        let (assign30440_e45366, assign30440_e45366_d_n0, assign30440_e45366_d_n1, assign30440_e45366_d_n2, assign30440_e45366_d_n3, assign30440_e45366_d_n4, assign30440_e45366_d_n5, assign30440_e45366_d_b0, assign30440_e45366_d_b1, assign30440_e45366_d_b2, assign30440_e45366_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30440_e45362: f64 = (var_nja11 - var_nfabot_i);
        let assign30440_e45364: f64 = (assign30440_e45362 - 0.01);
        (assign30440_e45364, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30440_e45366;
        var_tmf1_dn0 = assign30440_e45366_d_n0;
        var_tmf1_dn1 = assign30440_e45366_d_n1;
        var_tmf1_dn2 = assign30440_e45366_d_n2;
        var_tmf1_dn3 = assign30440_e45366_d_n3;
        var_tmf1_dn4 = assign30440_e45366_d_n4;
        var_tmf1_dn5 = assign30440_e45366_d_n5;
        var_tmf1_db0 = assign30440_e45366_d_b0;
        var_tmf1_db1 = assign30440_e45366_d_b1;
        var_tmf1_db2 = assign30440_e45366_d_b2;
        var_tmf1_db3 = assign30440_e45366_d_b3;

        let (assign30450_e45382, assign30450_e45382_d_n0, assign30450_e45382_d_n1, assign30450_e45382_d_n2, assign30450_e45382_d_n3, assign30450_e45382_d_n4, assign30450_e45382_d_n5, assign30450_e45382_d_b0, assign30450_e45382_d_b1, assign30450_e45382_d_b2, assign30450_e45382_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30450_e45378: f64 = (4.0 * var_nfabot_i);
        let assign30450_e45380: f64 = (assign30450_e45378 * 0.01);
        (assign30450_e45380, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30450_e45382;
        var_tmf2_dn0 = assign30450_e45382_d_n0;
        var_tmf2_dn1 = assign30450_e45382_d_n1;
        var_tmf2_dn2 = assign30450_e45382_d_n2;
        var_tmf2_dn3 = assign30450_e45382_d_n3;
        var_tmf2_dn4 = assign30450_e45382_d_n4;
        var_tmf2_dn5 = assign30450_e45382_d_n5;
        var_tmf2_db0 = assign30450_e45382_d_b0;
        var_tmf2_db1 = assign30450_e45382_d_b1;
        var_tmf2_db2 = assign30450_e45382_d_b2;
        var_tmf2_db3 = assign30450_e45382_d_b3;

        let (assign30460_e45400, assign30460_e45400_d_n0, assign30460_e45400_d_n1, assign30460_e45400_d_n2, assign30460_e45400_d_n3, assign30460_e45400_d_n4, assign30460_e45400_d_n5, assign30460_e45400_d_b0, assign30460_e45400_d_b1, assign30460_e45400_d_b2, assign30460_e45400_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n1, assign30460_e45398_d_n2, assign30460_e45398_d_n3, assign30460_e45398_d_n4, assign30460_e45398_d_n5, assign30460_e45398_d_b0, assign30460_e45398_d_b1, assign30460_e45398_d_b2, assign30460_e45398_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30460_e45397: f64 = (-var_tmf2);
                (assign30460_e45397, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n1, assign30460_e45398_d_n2, assign30460_e45398_d_n3, assign30460_e45398_d_n4, assign30460_e45398_d_n5, assign30460_e45398_d_b0, assign30460_e45398_d_b1, assign30460_e45398_d_b2, assign30460_e45398_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30460_e45400;
        var_tmf2_dn0 = assign30460_e45400_d_n0;
        var_tmf2_dn1 = assign30460_e45400_d_n1;
        var_tmf2_dn2 = assign30460_e45400_d_n2;
        var_tmf2_dn3 = assign30460_e45400_d_n3;
        var_tmf2_dn4 = assign30460_e45400_d_n4;
        var_tmf2_dn5 = assign30460_e45400_d_n5;
        var_tmf2_db0 = assign30460_e45400_d_b0;
        var_tmf2_db1 = assign30460_e45400_d_b1;
        var_tmf2_db2 = assign30460_e45400_d_b2;
        var_tmf2_db3 = assign30460_e45400_d_b3;

        let (assign30470_e45417, assign30470_e45417_d_n0, assign30470_e45417_d_n1, assign30470_e45417_d_n2, assign30470_e45417_d_n3, assign30470_e45417_d_n4, assign30470_e45417_d_n5, assign30470_e45417_d_b0, assign30470_e45417_d_b1, assign30470_e45417_d_b2, assign30470_e45417_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30470_e45412: f64 = (var_tmf1 * var_tmf1);
        let assign30470_e45414: f64 = (assign30470_e45412 + var_tmf2);
        let assign30470_e45415: f64 = (assign30470_e45414).sqrt();
        (assign30470_e45415, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30470_e45415)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30470_e45415)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30470_e45415)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30470_e45415)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30470_e45415)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30470_e45417;
        var_tmf2_dn0 = assign30470_e45417_d_n0;
        var_tmf2_dn1 = assign30470_e45417_d_n1;
        var_tmf2_dn2 = assign30470_e45417_d_n2;
        var_tmf2_dn3 = assign30470_e45417_d_n3;
        var_tmf2_dn4 = assign30470_e45417_d_n4;
        var_tmf2_dn5 = assign30470_e45417_d_n5;
        var_tmf2_db0 = assign30470_e45417_d_b0;
        var_tmf2_db1 = assign30470_e45417_d_b1;
        var_tmf2_db2 = assign30470_e45417_d_b2;
        var_tmf2_db3 = assign30470_e45417_d_b3;

        let (assign30480_e45435, assign30480_e45435_d_n0, assign30480_e45435_d_n1, assign30480_e45435_d_n2, assign30480_e45435_d_n3, assign30480_e45435_d_n4, assign30480_e45435_d_n5, assign30480_e45435_d_b0, assign30480_e45435_d_b1, assign30480_e45435_d_b2, assign30480_e45435_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30480_e45431: f64 = (var_tmf1 / var_tmf2);
        let assign30480_e45432: f64 = (1.0 + assign30480_e45431);
        let assign30480_e45433: f64 = (0.5 * assign30480_e45432);
        (assign30480_e45433, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign30480_e45435;
        var_dfn_sl_dn0 = assign30480_e45435_d_n0;
        var_dfn_sl_dn1 = assign30480_e45435_d_n1;
        var_dfn_sl_dn2 = assign30480_e45435_d_n2;
        var_dfn_sl_dn3 = assign30480_e45435_d_n3;
        var_dfn_sl_dn4 = assign30480_e45435_d_n4;
        var_dfn_sl_dn5 = assign30480_e45435_d_n5;
        var_dfn_sl_db0 = assign30480_e45435_d_b0;
        var_dfn_sl_db1 = assign30480_e45435_d_b1;
        var_dfn_sl_db2 = assign30480_e45435_d_b2;
        var_dfn_sl_db3 = assign30480_e45435_d_b3;

        let (assign30490_e45453, assign30490_e45453_d_n0, assign30490_e45453_d_n1, assign30490_e45453_d_n2, assign30490_e45453_d_n3, assign30490_e45453_d_n4, assign30490_e45453_d_n5, assign30490_e45453_d_b0, assign30490_e45453_d_b1, assign30490_e45453_d_b2, assign30490_e45453_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30490_e45449: f64 = (var_tmf1 + var_tmf2);
        let assign30490_e45450: f64 = (0.5 * assign30490_e45449);
        let assign30490_e45451: f64 = (var_nfabot_i + assign30490_e45450);
        (assign30490_e45451, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30490_e45453;
        var_nj1_dn0 = assign30490_e45453_d_n0;
        var_nj1_dn1 = assign30490_e45453_d_n1;
        var_nj1_dn2 = assign30490_e45453_d_n2;
        var_nj1_dn3 = assign30490_e45453_d_n3;
        var_nj1_dn4 = assign30490_e45453_d_n4;
        var_nj1_dn5 = assign30490_e45453_d_n5;
        var_nj1_db0 = assign30490_e45453_d_b0;
        var_nj1_db1 = assign30490_e45453_d_b1;
        var_nj1_db2 = assign30490_e45453_d_b2;
        var_nj1_db3 = assign30490_e45453_d_b3;

        let (assign30500_e45469, assign30500_e45469_d_n0, assign30500_e45469_d_n1, assign30500_e45469_d_n2, assign30500_e45469_d_n3, assign30500_e45469_d_n4, assign30500_e45469_d_n5, assign30500_e45469_d_b0, assign30500_e45469_d_b1, assign30500_e45469_d_b2, assign30500_e45469_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30500_e45465: f64 = (p.p85 - var_nj0);
        let assign30500_e45467: f64 = (assign30500_e45465 - 0.01);
        (assign30500_e45467, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30500_e45469;
        var_tmf1_dn0 = assign30500_e45469_d_n0;
        var_tmf1_dn1 = assign30500_e45469_d_n1;
        var_tmf1_dn2 = assign30500_e45469_d_n2;
        var_tmf1_dn3 = assign30500_e45469_d_n3;
        var_tmf1_dn4 = assign30500_e45469_d_n4;
        var_tmf1_dn5 = assign30500_e45469_d_n5;
        var_tmf1_db0 = assign30500_e45469_d_b0;
        var_tmf1_db1 = assign30500_e45469_d_b1;
        var_tmf1_db2 = assign30500_e45469_d_b2;
        var_tmf1_db3 = assign30500_e45469_d_b3;

        let (assign30510_e45485, assign30510_e45485_d_n0, assign30510_e45485_d_n1, assign30510_e45485_d_n2, assign30510_e45485_d_n3, assign30510_e45485_d_n4, assign30510_e45485_d_n5, assign30510_e45485_d_b0, assign30510_e45485_d_b1, assign30510_e45485_d_b2, assign30510_e45485_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30510_e45481: f64 = (4.0 * p.p85);
        let assign30510_e45483: f64 = (assign30510_e45481 * 0.01);
        (assign30510_e45483, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30510_e45485;
        var_tmf2_dn0 = assign30510_e45485_d_n0;
        var_tmf2_dn1 = assign30510_e45485_d_n1;
        var_tmf2_dn2 = assign30510_e45485_d_n2;
        var_tmf2_dn3 = assign30510_e45485_d_n3;
        var_tmf2_dn4 = assign30510_e45485_d_n4;
        var_tmf2_dn5 = assign30510_e45485_d_n5;
        var_tmf2_db0 = assign30510_e45485_d_b0;
        var_tmf2_db1 = assign30510_e45485_d_b1;
        var_tmf2_db2 = assign30510_e45485_d_b2;
        var_tmf2_db3 = assign30510_e45485_d_b3;

        let (assign30520_e45503, assign30520_e45503_d_n0, assign30520_e45503_d_n1, assign30520_e45503_d_n2, assign30520_e45503_d_n3, assign30520_e45503_d_n4, assign30520_e45503_d_n5, assign30520_e45503_d_b0, assign30520_e45503_d_b1, assign30520_e45503_d_b2, assign30520_e45503_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n1, assign30520_e45501_d_n2, assign30520_e45501_d_n3, assign30520_e45501_d_n4, assign30520_e45501_d_n5, assign30520_e45501_d_b0, assign30520_e45501_d_b1, assign30520_e45501_d_b2, assign30520_e45501_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30520_e45500: f64 = (-var_tmf2);
                (assign30520_e45500, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n1, assign30520_e45501_d_n2, assign30520_e45501_d_n3, assign30520_e45501_d_n4, assign30520_e45501_d_n5, assign30520_e45501_d_b0, assign30520_e45501_d_b1, assign30520_e45501_d_b2, assign30520_e45501_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30520_e45503;
        var_tmf2_dn0 = assign30520_e45503_d_n0;
        var_tmf2_dn1 = assign30520_e45503_d_n1;
        var_tmf2_dn2 = assign30520_e45503_d_n2;
        var_tmf2_dn3 = assign30520_e45503_d_n3;
        var_tmf2_dn4 = assign30520_e45503_d_n4;
        var_tmf2_dn5 = assign30520_e45503_d_n5;
        var_tmf2_db0 = assign30520_e45503_d_b0;
        var_tmf2_db1 = assign30520_e45503_d_b1;
        var_tmf2_db2 = assign30520_e45503_d_b2;
        var_tmf2_db3 = assign30520_e45503_d_b3;

        let (assign30530_e45520, assign30530_e45520_d_n0, assign30530_e45520_d_n1, assign30530_e45520_d_n2, assign30530_e45520_d_n3, assign30530_e45520_d_n4, assign30530_e45520_d_n5, assign30530_e45520_d_b0, assign30530_e45520_d_b1, assign30530_e45520_d_b2, assign30530_e45520_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30530_e45515: f64 = (var_tmf1 * var_tmf1);
        let assign30530_e45517: f64 = (assign30530_e45515 + var_tmf2);
        let assign30530_e45518: f64 = (assign30530_e45517).sqrt();
        (assign30530_e45518, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30530_e45518)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30530_e45518)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30530_e45518)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30530_e45518)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30530_e45518)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30530_e45520;
        var_tmf2_dn0 = assign30530_e45520_d_n0;
        var_tmf2_dn1 = assign30530_e45520_d_n1;
        var_tmf2_dn2 = assign30530_e45520_d_n2;
        var_tmf2_dn3 = assign30530_e45520_d_n3;
        var_tmf2_dn4 = assign30530_e45520_d_n4;
        var_tmf2_dn5 = assign30530_e45520_d_n5;
        var_tmf2_db0 = assign30530_e45520_d_b0;
        var_tmf2_db1 = assign30530_e45520_d_b1;
        var_tmf2_db2 = assign30530_e45520_d_b2;
        var_tmf2_db3 = assign30530_e45520_d_b3;


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
        *var_guard492_slot = var_guard492;
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

    pub(super) fn stamp_transient_block_47(
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
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard492: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_db0: f64,
        var_vak_db1: f64,
        var_vak_db2: f64,
        var_vak_db3: f64,
        var_vak_dn0: f64,
        var_vak_dn1: f64,
        var_vak_dn2: f64,
        var_vak_dn3: f64,
        var_vak_dn4: f64,
        var_vak_dn5: f64,
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
        var_guard493_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard495_slot: &mut f64,
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
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
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

        let (assign30540_e45538, assign30540_e45538_d_n0, assign30540_e45538_d_n1, assign30540_e45538_d_n2, assign30540_e45538_d_n3, assign30540_e45538_d_n4, assign30540_e45538_d_n5, assign30540_e45538_d_b0, assign30540_e45538_d_b1, assign30540_e45538_d_b2, assign30540_e45538_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30540_e45534: f64 = (var_tmf1 + var_tmf2);
        let assign30540_e45535: f64 = (0.5 * assign30540_e45534);
        let assign30540_e45536: f64 = (p.p85 - assign30540_e45535);
        (assign30540_e45536, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30540_e45538;
        var_nj0_dn0 = assign30540_e45538_d_n0;
        var_nj0_dn1 = assign30540_e45538_d_n1;
        var_nj0_dn2 = assign30540_e45538_d_n2;
        var_nj0_dn3 = assign30540_e45538_d_n3;
        var_nj0_dn4 = assign30540_e45538_d_n4;
        var_nj0_dn5 = assign30540_e45538_d_n5;
        var_nj0_db0 = assign30540_e45538_d_b0;
        var_nj0_db1 = assign30540_e45538_d_b1;
        var_nj0_db2 = assign30540_e45538_d_b2;
        var_nj0_db3 = assign30540_e45538_d_b3;

        let (assign30550_e45554, assign30550_e45554_d_n0, assign30550_e45554_d_n1, assign30550_e45554_d_n2, assign30550_e45554_d_n3, assign30550_e45554_d_n4, assign30550_e45554_d_n5, assign30550_e45554_d_b0, assign30550_e45554_d_b1, assign30550_e45554_d_b2, assign30550_e45554_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30550_e45550: f64 = (var_nj0 - var_nfabot_i);
        let assign30550_e45552: f64 = (assign30550_e45550 - 0.01);
        (assign30550_e45552, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30550_e45554;
        var_tmf1_dn0 = assign30550_e45554_d_n0;
        var_tmf1_dn1 = assign30550_e45554_d_n1;
        var_tmf1_dn2 = assign30550_e45554_d_n2;
        var_tmf1_dn3 = assign30550_e45554_d_n3;
        var_tmf1_dn4 = assign30550_e45554_d_n4;
        var_tmf1_dn5 = assign30550_e45554_d_n5;
        var_tmf1_db0 = assign30550_e45554_d_b0;
        var_tmf1_db1 = assign30550_e45554_d_b1;
        var_tmf1_db2 = assign30550_e45554_d_b2;
        var_tmf1_db3 = assign30550_e45554_d_b3;

        let (assign30560_e45570, assign30560_e45570_d_n0, assign30560_e45570_d_n1, assign30560_e45570_d_n2, assign30560_e45570_d_n3, assign30560_e45570_d_n4, assign30560_e45570_d_n5, assign30560_e45570_d_b0, assign30560_e45570_d_b1, assign30560_e45570_d_b2, assign30560_e45570_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30560_e45566: f64 = (4.0 * var_nfabot_i);
        let assign30560_e45568: f64 = (assign30560_e45566 * 0.01);
        (assign30560_e45568, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30560_e45570;
        var_tmf2_dn0 = assign30560_e45570_d_n0;
        var_tmf2_dn1 = assign30560_e45570_d_n1;
        var_tmf2_dn2 = assign30560_e45570_d_n2;
        var_tmf2_dn3 = assign30560_e45570_d_n3;
        var_tmf2_dn4 = assign30560_e45570_d_n4;
        var_tmf2_dn5 = assign30560_e45570_d_n5;
        var_tmf2_db0 = assign30560_e45570_d_b0;
        var_tmf2_db1 = assign30560_e45570_d_b1;
        var_tmf2_db2 = assign30560_e45570_d_b2;
        var_tmf2_db3 = assign30560_e45570_d_b3;

        let (assign30570_e45588, assign30570_e45588_d_n0, assign30570_e45588_d_n1, assign30570_e45588_d_n2, assign30570_e45588_d_n3, assign30570_e45588_d_n4, assign30570_e45588_d_n5, assign30570_e45588_d_b0, assign30570_e45588_d_b1, assign30570_e45588_d_b2, assign30570_e45588_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n1, assign30570_e45586_d_n2, assign30570_e45586_d_n3, assign30570_e45586_d_n4, assign30570_e45586_d_n5, assign30570_e45586_d_b0, assign30570_e45586_d_b1, assign30570_e45586_d_b2, assign30570_e45586_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30570_e45585: f64 = (-var_tmf2);
                (assign30570_e45585, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n1, assign30570_e45586_d_n2, assign30570_e45586_d_n3, assign30570_e45586_d_n4, assign30570_e45586_d_n5, assign30570_e45586_d_b0, assign30570_e45586_d_b1, assign30570_e45586_d_b2, assign30570_e45586_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30570_e45588;
        var_tmf2_dn0 = assign30570_e45588_d_n0;
        var_tmf2_dn1 = assign30570_e45588_d_n1;
        var_tmf2_dn2 = assign30570_e45588_d_n2;
        var_tmf2_dn3 = assign30570_e45588_d_n3;
        var_tmf2_dn4 = assign30570_e45588_d_n4;
        var_tmf2_dn5 = assign30570_e45588_d_n5;
        var_tmf2_db0 = assign30570_e45588_d_b0;
        var_tmf2_db1 = assign30570_e45588_d_b1;
        var_tmf2_db2 = assign30570_e45588_d_b2;
        var_tmf2_db3 = assign30570_e45588_d_b3;

        let (assign30580_e45605, assign30580_e45605_d_n0, assign30580_e45605_d_n1, assign30580_e45605_d_n2, assign30580_e45605_d_n3, assign30580_e45605_d_n4, assign30580_e45605_d_n5, assign30580_e45605_d_b0, assign30580_e45605_d_b1, assign30580_e45605_d_b2, assign30580_e45605_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30580_e45600: f64 = (var_tmf1 * var_tmf1);
        let assign30580_e45602: f64 = (assign30580_e45600 + var_tmf2);
        let assign30580_e45603: f64 = (assign30580_e45602).sqrt();
        (assign30580_e45603, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30580_e45603)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30580_e45603)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30580_e45603)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30580_e45603)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30580_e45603)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30580_e45605;
        var_tmf2_dn0 = assign30580_e45605_d_n0;
        var_tmf2_dn1 = assign30580_e45605_d_n1;
        var_tmf2_dn2 = assign30580_e45605_d_n2;
        var_tmf2_dn3 = assign30580_e45605_d_n3;
        var_tmf2_dn4 = assign30580_e45605_d_n4;
        var_tmf2_dn5 = assign30580_e45605_d_n5;
        var_tmf2_db0 = assign30580_e45605_d_b0;
        var_tmf2_db1 = assign30580_e45605_d_b1;
        var_tmf2_db2 = assign30580_e45605_d_b2;
        var_tmf2_db3 = assign30580_e45605_d_b3;

        let (assign30590_e45623, assign30590_e45623_d_n0, assign30590_e45623_d_n1, assign30590_e45623_d_n2, assign30590_e45623_d_n3, assign30590_e45623_d_n4, assign30590_e45623_d_n5, assign30590_e45623_d_b0, assign30590_e45623_d_b1, assign30590_e45623_d_b2, assign30590_e45623_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30590_e45619: f64 = (var_tmf1 + var_tmf2);
        let assign30590_e45620: f64 = (0.5 * assign30590_e45619);
        let assign30590_e45621: f64 = (var_nfabot_i + assign30590_e45620);
        (assign30590_e45621, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30590_e45623;
        var_nj0_dn0 = assign30590_e45623_d_n0;
        var_nj0_dn1 = assign30590_e45623_d_n1;
        var_nj0_dn2 = assign30590_e45623_d_n2;
        var_nj0_dn3 = assign30590_e45623_d_n3;
        var_nj0_dn4 = assign30590_e45623_d_n4;
        var_nj0_dn5 = assign30590_e45623_d_n5;
        var_nj0_db0 = assign30590_e45623_d_b0;
        var_nj0_db1 = assign30590_e45623_d_b1;
        var_nj0_db2 = assign30590_e45623_d_b2;
        var_nj0_db3 = assign30590_e45623_d_b3;

        let (assign30600_e45639, assign30600_e45639_d_n0, assign30600_e45639_d_n1, assign30600_e45639_d_n2, assign30600_e45639_d_n3, assign30600_e45639_d_n4, assign30600_e45639_d_n5, assign30600_e45639_d_b0, assign30600_e45639_d_b1, assign30600_e45639_d_b2, assign30600_e45639_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30600_e45635: f64 = (p.p86 * var_dfn_su);
        let assign30600_e45637: f64 = (assign30600_e45635 * var_dfn_sl);
        (assign30600_e45637, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30600_e45639;
        var_dnj1_dv_dn0 = assign30600_e45639_d_n0;
        var_dnj1_dv_dn1 = assign30600_e45639_d_n1;
        var_dnj1_dv_dn2 = assign30600_e45639_d_n2;
        var_dnj1_dv_dn3 = assign30600_e45639_d_n3;
        var_dnj1_dv_dn4 = assign30600_e45639_d_n4;
        var_dnj1_dv_dn5 = assign30600_e45639_d_n5;
        var_dnj1_dv_db0 = assign30600_e45639_d_b0;
        var_dnj1_dv_db1 = assign30600_e45639_d_b1;
        var_dnj1_dv_db2 = assign30600_e45639_d_b2;
        var_dnj1_dv_db3 = assign30600_e45639_d_b3;

        let (assign30610_e45652, assign30610_e45652_d_n0, assign30610_e45652_d_n1, assign30610_e45652_d_n2, assign30610_e45652_d_n3, assign30610_e45652_d_n4, assign30610_e45652_d_n5, assign30610_e45652_d_b0, assign30610_e45652_d_b1, assign30610_e45652_d_b2, assign30610_e45652_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30610_e45652;
        var_nj0_dn0 = assign30610_e45652_d_n0;
        var_nj0_dn1 = assign30610_e45652_d_n1;
        var_nj0_dn2 = assign30610_e45652_d_n2;
        var_nj0_dn3 = assign30610_e45652_d_n3;
        var_nj0_dn4 = assign30610_e45652_d_n4;
        var_nj0_dn5 = assign30610_e45652_d_n5;
        var_nj0_db0 = assign30610_e45652_d_b0;
        var_nj0_db1 = assign30610_e45652_d_b1;
        var_nj0_db2 = assign30610_e45652_d_b2;
        var_nj0_db3 = assign30610_e45652_d_b3;

        let (assign30620_e45665, assign30620_e45665_d_n0, assign30620_e45665_d_n1, assign30620_e45665_d_n2, assign30620_e45665_d_n3, assign30620_e45665_d_n4, assign30620_e45665_d_n5, assign30620_e45665_d_b0, assign30620_e45665_d_b1, assign30620_e45665_d_b2, assign30620_e45665_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30620_e45665;
        var_nj1_dn0 = assign30620_e45665_d_n0;
        var_nj1_dn1 = assign30620_e45665_d_n1;
        var_nj1_dn2 = assign30620_e45665_d_n2;
        var_nj1_dn3 = assign30620_e45665_d_n3;
        var_nj1_dn4 = assign30620_e45665_d_n4;
        var_nj1_dn5 = assign30620_e45665_d_n5;
        var_nj1_db0 = assign30620_e45665_d_b0;
        var_nj1_db1 = assign30620_e45665_d_b1;
        var_nj1_db2 = assign30620_e45665_d_b2;
        var_nj1_db3 = assign30620_e45665_d_b3;

        let (assign30630_e45678, assign30630_e45678_d_n0, assign30630_e45678_d_n1, assign30630_e45678_d_n2, assign30630_e45678_d_n3, assign30630_e45678_d_n4, assign30630_e45678_d_n5, assign30630_e45678_d_b0, assign30630_e45678_d_b1, assign30630_e45678_d_b2, assign30630_e45678_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30630_e45678;
        var_dnj1_dv_dn0 = assign30630_e45678_d_n0;
        var_dnj1_dv_dn1 = assign30630_e45678_d_n1;
        var_dnj1_dv_dn2 = assign30630_e45678_d_n2;
        var_dnj1_dv_dn3 = assign30630_e45678_d_n3;
        var_dnj1_dv_dn4 = assign30630_e45678_d_n4;
        var_dnj1_dv_dn5 = assign30630_e45678_d_n5;
        var_dnj1_dv_db0 = assign30630_e45678_d_b0;
        var_dnj1_dv_db1 = assign30630_e45678_d_b1;
        var_dnj1_dv_db2 = assign30630_e45678_d_b2;
        var_dnj1_dv_db3 = assign30630_e45678_d_b3;

        let assign30640_e45682: f64 = (var_vmax / var_nj1);
        let assign30640_e45686: f64 = (var_nj1 - var_nj0);
        let assign30640_e45687: f64 = (var_vha1 * assign30640_e45686);
        let assign30640_e45690: f64 = (var_nj0 * p.p85);
        let assign30640_e45691: f64 = (assign30640_e45687 / assign30640_e45690);
        let assign30640_e45692: f64 = (assign30640_e45682 + assign30640_e45691);
        let assign30640_e45693: f64 = (var_phitdinv * assign30640_e45692);
        let assign30640_e45694: f64 = (assign30640_e45693).abs();
        let assign30640_e45696: f64 = if assign30640_e45694 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard493 = assign30640_e45696;

        let (assign30650_e45723, assign30650_e45723_d_n0, assign30650_e45723_d_n1, assign30650_e45723_d_n2, assign30650_e45723_d_n3, assign30650_e45723_d_n4, assign30650_e45723_d_n5, assign30650_e45723_d_b0, assign30650_e45723_d_b1, assign30650_e45723_d_b2, assign30650_e45723_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 != 0.0)) {
        let assign30650_e45709: f64 = (var_vmax / var_nj1);
        let assign30650_e45713: f64 = (var_nj1 - var_nj0);
        let assign30650_e45714: f64 = (var_vha1 * assign30650_e45713);
        let assign30650_e45717: f64 = (var_nj0 * p.p85);
        let assign30650_e45718: f64 = (assign30650_e45714 / assign30650_e45717);
        let assign30650_e45719: f64 = (assign30650_e45709 + assign30650_e45718);
        let assign30650_e45720: f64 = (var_phitdinv * assign30650_e45719);
        let assign30650_e45721: f64 = (assign30650_e45720).exp();
        (assign30650_e45721, (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn0 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn1 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn2 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn3 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn4 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn5 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db0 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db1 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db2 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_db3 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30650_e45723;
        var_exp_vmax_over_phitd_bot_dn0 = assign30650_e45723_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30650_e45723_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30650_e45723_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30650_e45723_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30650_e45723_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30650_e45723_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30650_e45723_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30650_e45723_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30650_e45723_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30650_e45723_d_b3;

        let assign30660_e45727: f64 = (var_vmax / var_nj1);
        let assign30660_e45731: f64 = (var_nj1 - var_nj0);
        let assign30660_e45732: f64 = (var_vha1 * assign30660_e45731);
        let assign30660_e45735: f64 = (var_nj0 * p.p85);
        let assign30660_e45736: f64 = (assign30660_e45732 / assign30660_e45735);
        let assign30660_e45737: f64 = (assign30660_e45727 + assign30660_e45736);
        let assign30660_e45738: f64 = (var_phitdinv * assign30660_e45737);
        let assign30660_e45740: f64 = (-230.25850929940458);
        let assign30660_e45741: f64 = if assign30660_e45738 < assign30660_e45740 { 1.0 } else { 0.0 };
        var_guard494 = assign30660_e45741;

        let (assign30670_e45823, assign30670_e45823_d_n0, assign30670_e45823_d_n1, assign30670_e45823_d_n2, assign30670_e45823_d_n3, assign30670_e45823_d_n4, assign30670_e45823_d_n5, assign30670_e45823_d_b0, assign30670_e45823_d_b1, assign30670_e45823_d_b2, assign30670_e45823_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 != 0.0)) {
        let assign30670_e45757: f64 = (-230.25850929940458);
        let assign30670_e45761: f64 = (var_vmax / var_nj1);
        let assign30670_e45765: f64 = (var_nj1 - var_nj0);
        let assign30670_e45766: f64 = (var_vha1 * assign30670_e45765);
        let assign30670_e45769: f64 = (var_nj0 * p.p85);
        let assign30670_e45770: f64 = (assign30670_e45766 / assign30670_e45769);
        let assign30670_e45771: f64 = (assign30670_e45761 + assign30670_e45770);
        let assign30670_e45772: f64 = (var_phitdinv * assign30670_e45771);
        let assign30670_e45773: f64 = (assign30670_e45757 - assign30670_e45772);
        let assign30670_e45777: f64 = (-230.25850929940458);
        let assign30670_e45781: f64 = (var_vmax / var_nj1);
        let assign30670_e45785: f64 = (var_nj1 - var_nj0);
        let assign30670_e45786: f64 = (var_vha1 * assign30670_e45785);
        let assign30670_e45789: f64 = (var_nj0 * p.p85);
        let assign30670_e45790: f64 = (assign30670_e45786 / assign30670_e45789);
        let assign30670_e45791: f64 = (assign30670_e45781 + assign30670_e45790);
        let assign30670_e45792: f64 = (var_phitdinv * assign30670_e45791);
        let assign30670_e45793: f64 = (assign30670_e45777 - assign30670_e45792);
        let assign30670_e45796: f64 = (-230.25850929940458);
        let assign30670_e45800: f64 = (var_vmax / var_nj1);
        let assign30670_e45804: f64 = (var_nj1 - var_nj0);
        let assign30670_e45805: f64 = (var_vha1 * assign30670_e45804);
        let assign30670_e45808: f64 = (var_nj0 * p.p85);
        let assign30670_e45809: f64 = (assign30670_e45805 / assign30670_e45808);
        let assign30670_e45810: f64 = (assign30670_e45800 + assign30670_e45809);
        let assign30670_e45811: f64 = (var_phitdinv * assign30670_e45810);
        let assign30670_e45812: f64 = (assign30670_e45796 - assign30670_e45811);
        let assign30670_e45814: f64 = (assign30670_e45812 * 0.3333333333333333);
        let assign30670_e45815: f64 = (1.0 + assign30670_e45814);
        let assign30670_e45816: f64 = (assign30670_e45793 * assign30670_e45815);
        let assign30670_e45817: f64 = (0.5 * assign30670_e45816);
        let assign30670_e45818: f64 = (1.0 + assign30670_e45817);
        let assign30670_e45819: f64 = (assign30670_e45773 * assign30670_e45818);
        let assign30670_e45820: f64 = (1.0 + assign30670_e45819);
        let assign30670_e45821: f64 = (1e-100 / assign30670_e45820);
        (assign30670_e45821, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn0 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn0 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn0 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn1 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn1 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn1 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn2 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn2 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn2 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn3 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn3 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn3 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn4 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn4 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn4 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn5 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn5 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn5 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db0 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db0 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db0 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db1 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db1 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db1 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db2 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db2 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db2 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_db3 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_db3 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_db3 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30670_e45823;
        var_exp_vmax_over_phitd_bot_dn0 = assign30670_e45823_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30670_e45823_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30670_e45823_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30670_e45823_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30670_e45823_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30670_e45823_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30670_e45823_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30670_e45823_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30670_e45823_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30670_e45823_d_b3;

        let (assign30680_e45903, assign30680_e45903_d_n0, assign30680_e45903_d_n1, assign30680_e45903_d_n2, assign30680_e45903_d_n3, assign30680_e45903_d_n4, assign30680_e45903_d_n5, assign30680_e45903_d_b0, assign30680_e45903_d_b1, assign30680_e45903_d_b2, assign30680_e45903_d_b3,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 == 0.0)) {
        let assign30680_e45842: f64 = (var_vmax / var_nj1);
        let assign30680_e45846: f64 = (var_nj1 - var_nj0);
        let assign30680_e45847: f64 = (var_vha1 * assign30680_e45846);
        let assign30680_e45850: f64 = (var_nj0 * p.p85);
        let assign30680_e45851: f64 = (assign30680_e45847 / assign30680_e45850);
        let assign30680_e45852: f64 = (assign30680_e45842 + assign30680_e45851);
        let assign30680_e45853: f64 = (var_phitdinv * assign30680_e45852);
        let assign30680_e45855: f64 = (assign30680_e45853 - 230.25850929940458);
        let assign30680_e45861: f64 = (var_vmax / var_nj1);
        let assign30680_e45865: f64 = (var_nj1 - var_nj0);
        let assign30680_e45866: f64 = (var_vha1 * assign30680_e45865);
        let assign30680_e45869: f64 = (var_nj0 * p.p85);
        let assign30680_e45870: f64 = (assign30680_e45866 / assign30680_e45869);
        let assign30680_e45871: f64 = (assign30680_e45861 + assign30680_e45870);
        let assign30680_e45872: f64 = (var_phitdinv * assign30680_e45871);
        let assign30680_e45874: f64 = (assign30680_e45872 - 230.25850929940458);
        let assign30680_e45879: f64 = (var_vmax / var_nj1);
        let assign30680_e45883: f64 = (var_nj1 - var_nj0);
        let assign30680_e45884: f64 = (var_vha1 * assign30680_e45883);
        let assign30680_e45887: f64 = (var_nj0 * p.p85);
        let assign30680_e45888: f64 = (assign30680_e45884 / assign30680_e45887);
        let assign30680_e45889: f64 = (assign30680_e45879 + assign30680_e45888);
        let assign30680_e45890: f64 = (var_phitdinv * assign30680_e45889);
        let assign30680_e45892: f64 = (assign30680_e45890 - 230.25850929940458);
        let assign30680_e45894: f64 = (assign30680_e45892 * 0.3333333333333333);
        let assign30680_e45895: f64 = (1.0 + assign30680_e45894);
        let assign30680_e45896: f64 = (assign30680_e45874 * assign30680_e45895);
        let assign30680_e45897: f64 = (0.5 * assign30680_e45896);
        let assign30680_e45898: f64 = (1.0 + assign30680_e45897);
        let assign30680_e45899: f64 = (assign30680_e45855 * assign30680_e45898);
        let assign30680_e45900: f64 = (1.0 + assign30680_e45899);
        let assign30680_e45901: f64 = (1e100 * assign30680_e45900);
        (assign30680_e45901, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn0 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn0 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn0 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn1 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn1 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn1 - var_nj0_dn1)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn1 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn2 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn2 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn2 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn3 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn3 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn3 - var_nj0_dn3)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn3 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn4 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn4 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn4) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn4 - var_nj0_dn4)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn4 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn5 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn5 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn5) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn5 - var_nj0_dn5)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn5 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db0 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db0 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_db0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db0 - var_nj0_db0)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db0 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db1 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db1 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_db1) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db1 - var_nj0_db1)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db1 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db2 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db2 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_db2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db2 - var_nj0_db2)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db2 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_db3 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_db3 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_db3) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_db3 - var_nj0_db3)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_db3 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn1, var_exp_vmax_over_phitd_bot_dn2, var_exp_vmax_over_phitd_bot_dn3, var_exp_vmax_over_phitd_bot_dn4, var_exp_vmax_over_phitd_bot_dn5, var_exp_vmax_over_phitd_bot_db0, var_exp_vmax_over_phitd_bot_db1, var_exp_vmax_over_phitd_bot_db2, var_exp_vmax_over_phitd_bot_db3,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30680_e45903;
        var_exp_vmax_over_phitd_bot_dn0 = assign30680_e45903_d_n0;
        var_exp_vmax_over_phitd_bot_dn1 = assign30680_e45903_d_n1;
        var_exp_vmax_over_phitd_bot_dn2 = assign30680_e45903_d_n2;
        var_exp_vmax_over_phitd_bot_dn3 = assign30680_e45903_d_n3;
        var_exp_vmax_over_phitd_bot_dn4 = assign30680_e45903_d_n4;
        var_exp_vmax_over_phitd_bot_dn5 = assign30680_e45903_d_n5;
        var_exp_vmax_over_phitd_bot_db0 = assign30680_e45903_d_b0;
        var_exp_vmax_over_phitd_bot_db1 = assign30680_e45903_d_b1;
        var_exp_vmax_over_phitd_bot_db2 = assign30680_e45903_d_b2;
        var_exp_vmax_over_phitd_bot_db3 = assign30680_e45903_d_b3;

        let (assign30690_e45931, assign30690_e45931_d_n0, assign30690_e45931_d_n1, assign30690_e45931_d_n2, assign30690_e45931_d_n3, assign30690_e45931_d_n4, assign30690_e45931_d_n5, assign30690_e45931_d_b0, assign30690_e45931_d_b1, assign30690_e45931_d_b2, assign30690_e45931_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30690_e45915: f64 = (var_vmax * var_dnj1_dv);
        let assign30690_e45916: f64 = (var_nj1 - assign30690_e45915);
        let assign30690_e45919: f64 = (var_nj1 * var_nj1);
        let assign30690_e45920: f64 = (assign30690_e45916 / assign30690_e45919);
        let assign30690_e45923: f64 = (var_vha1 * var_dnj1_dv);
        let assign30690_e45926: f64 = (var_nj0 * p.p85);
        let assign30690_e45927: f64 = (assign30690_e45923 / assign30690_e45926);
        let assign30690_e45928: f64 = (assign30690_e45920 + assign30690_e45927);
        let assign30690_e45929: f64 = (var_phitdinv * assign30690_e45928);
        (assign30690_e45929, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn0 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn1 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn2 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn3 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn4 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn5 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_db0) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db0 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_db1) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db1 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_db2) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db2 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_db3) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_db3 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign30690_e45931;
        var_dvmax_over_phitd_dv_dn0 = assign30690_e45931_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign30690_e45931_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign30690_e45931_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign30690_e45931_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign30690_e45931_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign30690_e45931_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign30690_e45931_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign30690_e45931_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign30690_e45931_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign30690_e45931_d_b3;

        let (assign30700_e45949, assign30700_e45949_d_n0, assign30700_e45949_d_n1, assign30700_e45949_d_n2, assign30700_e45949_d_n3, assign30700_e45949_d_n4, assign30700_e45949_d_n5, assign30700_e45949_d_b0, assign30700_e45949_d_b1, assign30700_e45949_d_b2, assign30700_e45949_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30700_e45942: f64 = (var_vak - var_vmax);
        let assign30700_e45944: f64 = (assign30700_e45942 * var_dvmax_over_phitd_dv);
        let assign30700_e45945: f64 = (1.0 + assign30700_e45944);
        let assign30700_e45947: f64 = (assign30700_e45945 * var_exp_vmax_over_phitd_bot);
        (assign30700_e45947, ((((var_vak_dn0 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn0)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn0)), ((((var_vak_dn1 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn1)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn1)), ((((var_vak_dn2 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn2)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn2)), ((((var_vak_dn3 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn3)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn3)), ((((var_vak_dn4 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn4)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn4)), ((((var_vak_dn5 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn5)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn5)), ((((var_vak_db0 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db0)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db0)), ((((var_vak_db1 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db1)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db1)), ((((var_vak_db2 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db2)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db2)), ((((var_vak_db3 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_db3)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_db3)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign30700_e45949;
        var_idmultbot_dn0 = assign30700_e45949_d_n0;
        var_idmultbot_dn1 = assign30700_e45949_d_n1;
        var_idmultbot_dn2 = assign30700_e45949_d_n2;
        var_idmultbot_dn3 = assign30700_e45949_d_n3;
        var_idmultbot_dn4 = assign30700_e45949_d_n4;
        var_idmultbot_dn5 = assign30700_e45949_d_n5;
        var_idmultbot_db0 = assign30700_e45949_d_b0;
        var_idmultbot_db1 = assign30700_e45949_d_b1;
        var_idmultbot_db2 = assign30700_e45949_d_b2;
        var_idmultbot_db3 = assign30700_e45949_d_b3;

        let (assign30710_e45963,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30710_e45959: f64 = (var_nin * var_nin);
        let assign30710_e45961: f64 = (assign30710_e45959 / var_ndisti_i);
        (assign30710_e45961,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30710_e45963;

        let (assign30720_e45980,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30720_e45973: f64 = (var_nfasti_i / var_phitdinv);
        let assign30720_e45976: f64 = (var_ndisti_i / var_pnn0);
        let assign30720_e45977: f64 = (assign30720_e45976).ln();
        let assign30720_e45978: f64 = (assign30720_e45973 * assign30720_e45977);
        (assign30720_e45978,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30720_e45980;

        let assign30730_e45983: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard495 = assign30730_e45983;

        let (assign30740_e46001, assign30740_e46001_d_n0, assign30740_e46001_d_n1, assign30740_e46001_d_n2, assign30740_e46001_d_n3, assign30740_e46001_d_n4, assign30740_e46001_d_n5, assign30740_e46001_d_b0, assign30740_e46001_d_b1, assign30740_e46001_d_b2, assign30740_e46001_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30740_e45996: f64 = (var_vmax - var_vha1);
        let assign30740_e45997: f64 = (p.p86 * assign30740_e45996);
        let assign30740_e45999: f64 = (assign30740_e45997 + var_nfasti_i);
        (assign30740_e45999, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign30740_e46001;
        var_nja10_dn0 = assign30740_e46001_d_n0;
        var_nja10_dn1 = assign30740_e46001_d_n1;
        var_nja10_dn2 = assign30740_e46001_d_n2;
        var_nja10_dn3 = assign30740_e46001_d_n3;
        var_nja10_dn4 = assign30740_e46001_d_n4;
        var_nja10_dn5 = assign30740_e46001_d_n5;
        var_nja10_db0 = assign30740_e46001_d_b0;
        var_nja10_db1 = assign30740_e46001_d_b1;
        var_nja10_db2 = assign30740_e46001_d_b2;
        var_nja10_db3 = assign30740_e46001_d_b3;

        let (assign30750_e46017, assign30750_e46017_d_n0, assign30750_e46017_d_n1, assign30750_e46017_d_n2, assign30750_e46017_d_n3, assign30750_e46017_d_n4, assign30750_e46017_d_n5, assign30750_e46017_d_b0, assign30750_e46017_d_b1, assign30750_e46017_d_b2, assign30750_e46017_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30750_e46014: f64 = (p.p86 * var_vha1);
        let assign30750_e46015: f64 = (var_nfasti_i - assign30750_e46014);
        (assign30750_e46015, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30750_e46017;
        var_nj0_dn0 = assign30750_e46017_d_n0;
        var_nj0_dn1 = assign30750_e46017_d_n1;
        var_nj0_dn2 = assign30750_e46017_d_n2;
        var_nj0_dn3 = assign30750_e46017_d_n3;
        var_nj0_dn4 = assign30750_e46017_d_n4;
        var_nj0_dn5 = assign30750_e46017_d_n5;
        var_nj0_db0 = assign30750_e46017_d_b0;
        var_nj0_db1 = assign30750_e46017_d_b1;
        var_nj0_db2 = assign30750_e46017_d_b2;
        var_nj0_db3 = assign30750_e46017_d_b3;

        let (assign30760_e46033, assign30760_e46033_d_n0, assign30760_e46033_d_n1, assign30760_e46033_d_n2, assign30760_e46033_d_n3, assign30760_e46033_d_n4, assign30760_e46033_d_n5, assign30760_e46033_d_b0, assign30760_e46033_d_b1, assign30760_e46033_d_b2, assign30760_e46033_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30760_e46029: f64 = (p.p85 - var_nja10);
        let assign30760_e46031: f64 = (assign30760_e46029 - 0.01);
        (assign30760_e46031, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30760_e46033;
        var_tmf1_dn0 = assign30760_e46033_d_n0;
        var_tmf1_dn1 = assign30760_e46033_d_n1;
        var_tmf1_dn2 = assign30760_e46033_d_n2;
        var_tmf1_dn3 = assign30760_e46033_d_n3;
        var_tmf1_dn4 = assign30760_e46033_d_n4;
        var_tmf1_dn5 = assign30760_e46033_d_n5;
        var_tmf1_db0 = assign30760_e46033_d_b0;
        var_tmf1_db1 = assign30760_e46033_d_b1;
        var_tmf1_db2 = assign30760_e46033_d_b2;
        var_tmf1_db3 = assign30760_e46033_d_b3;


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
        *var_guard493_slot = var_guard493;
        *var_guard494_slot = var_guard494;
        *var_guard495_slot = var_guard495;
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
