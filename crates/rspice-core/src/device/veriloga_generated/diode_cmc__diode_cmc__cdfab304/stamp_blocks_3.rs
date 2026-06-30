#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard495: f64,
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

        let (assign30770_e46049, assign30770_e46049_d_n0, assign30770_e46049_d_n1, assign30770_e46049_d_n2, assign30770_e46049_d_n3, assign30770_e46049_d_n4, assign30770_e46049_d_n5, assign30770_e46049_d_b0, assign30770_e46049_d_b1, assign30770_e46049_d_b2, assign30770_e46049_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30770_e46045: f64 = (4.0 * p.p85);
        let assign30770_e46047: f64 = (assign30770_e46045 * 0.01);
        (assign30770_e46047, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30770_e46049;
        var_tmf2_dn0 = assign30770_e46049_d_n0;
        var_tmf2_dn1 = assign30770_e46049_d_n1;
        var_tmf2_dn2 = assign30770_e46049_d_n2;
        var_tmf2_dn3 = assign30770_e46049_d_n3;
        var_tmf2_dn4 = assign30770_e46049_d_n4;
        var_tmf2_dn5 = assign30770_e46049_d_n5;
        var_tmf2_db0 = assign30770_e46049_d_b0;
        var_tmf2_db1 = assign30770_e46049_d_b1;
        var_tmf2_db2 = assign30770_e46049_d_b2;
        var_tmf2_db3 = assign30770_e46049_d_b3;

        let (assign30780_e46067, assign30780_e46067_d_n0, assign30780_e46067_d_n1, assign30780_e46067_d_n2, assign30780_e46067_d_n3, assign30780_e46067_d_n4, assign30780_e46067_d_n5, assign30780_e46067_d_b0, assign30780_e46067_d_b1, assign30780_e46067_d_b2, assign30780_e46067_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n1, assign30780_e46065_d_n2, assign30780_e46065_d_n3, assign30780_e46065_d_n4, assign30780_e46065_d_n5, assign30780_e46065_d_b0, assign30780_e46065_d_b1, assign30780_e46065_d_b2, assign30780_e46065_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30780_e46064: f64 = (-var_tmf2);
                (assign30780_e46064, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n1, assign30780_e46065_d_n2, assign30780_e46065_d_n3, assign30780_e46065_d_n4, assign30780_e46065_d_n5, assign30780_e46065_d_b0, assign30780_e46065_d_b1, assign30780_e46065_d_b2, assign30780_e46065_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30780_e46067;
        var_tmf2_dn0 = assign30780_e46067_d_n0;
        var_tmf2_dn1 = assign30780_e46067_d_n1;
        var_tmf2_dn2 = assign30780_e46067_d_n2;
        var_tmf2_dn3 = assign30780_e46067_d_n3;
        var_tmf2_dn4 = assign30780_e46067_d_n4;
        var_tmf2_dn5 = assign30780_e46067_d_n5;
        var_tmf2_db0 = assign30780_e46067_d_b0;
        var_tmf2_db1 = assign30780_e46067_d_b1;
        var_tmf2_db2 = assign30780_e46067_d_b2;
        var_tmf2_db3 = assign30780_e46067_d_b3;

        let (assign30790_e46084, assign30790_e46084_d_n0, assign30790_e46084_d_n1, assign30790_e46084_d_n2, assign30790_e46084_d_n3, assign30790_e46084_d_n4, assign30790_e46084_d_n5, assign30790_e46084_d_b0, assign30790_e46084_d_b1, assign30790_e46084_d_b2, assign30790_e46084_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30790_e46079: f64 = (var_tmf1 * var_tmf1);
        let assign30790_e46081: f64 = (assign30790_e46079 + var_tmf2);
        let assign30790_e46082: f64 = (assign30790_e46081).sqrt();
        (assign30790_e46082, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30790_e46082)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30790_e46082)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30790_e46082)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30790_e46082)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30790_e46082)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30790_e46084;
        var_tmf2_dn0 = assign30790_e46084_d_n0;
        var_tmf2_dn1 = assign30790_e46084_d_n1;
        var_tmf2_dn2 = assign30790_e46084_d_n2;
        var_tmf2_dn3 = assign30790_e46084_d_n3;
        var_tmf2_dn4 = assign30790_e46084_d_n4;
        var_tmf2_dn5 = assign30790_e46084_d_n5;
        var_tmf2_db0 = assign30790_e46084_d_b0;
        var_tmf2_db1 = assign30790_e46084_d_b1;
        var_tmf2_db2 = assign30790_e46084_d_b2;
        var_tmf2_db3 = assign30790_e46084_d_b3;

        let (assign30800_e46102, assign30800_e46102_d_n0, assign30800_e46102_d_n1, assign30800_e46102_d_n2, assign30800_e46102_d_n3, assign30800_e46102_d_n4, assign30800_e46102_d_n5, assign30800_e46102_d_b0, assign30800_e46102_d_b1, assign30800_e46102_d_b2, assign30800_e46102_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30800_e46098: f64 = (var_tmf1 / var_tmf2);
        let assign30800_e46099: f64 = (1.0 + assign30800_e46098);
        let assign30800_e46100: f64 = (0.5 * assign30800_e46099);
        (assign30800_e46100, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign30800_e46102;
        var_dfn_su_dn0 = assign30800_e46102_d_n0;
        var_dfn_su_dn1 = assign30800_e46102_d_n1;
        var_dfn_su_dn2 = assign30800_e46102_d_n2;
        var_dfn_su_dn3 = assign30800_e46102_d_n3;
        var_dfn_su_dn4 = assign30800_e46102_d_n4;
        var_dfn_su_dn5 = assign30800_e46102_d_n5;
        var_dfn_su_db0 = assign30800_e46102_d_b0;
        var_dfn_su_db1 = assign30800_e46102_d_b1;
        var_dfn_su_db2 = assign30800_e46102_d_b2;
        var_dfn_su_db3 = assign30800_e46102_d_b3;

        let (assign30810_e46120, assign30810_e46120_d_n0, assign30810_e46120_d_n1, assign30810_e46120_d_n2, assign30810_e46120_d_n3, assign30810_e46120_d_n4, assign30810_e46120_d_n5, assign30810_e46120_d_b0, assign30810_e46120_d_b1, assign30810_e46120_d_b2, assign30810_e46120_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30810_e46116: f64 = (var_tmf1 + var_tmf2);
        let assign30810_e46117: f64 = (0.5 * assign30810_e46116);
        let assign30810_e46118: f64 = (p.p85 - assign30810_e46117);
        (assign30810_e46118, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign30810_e46120;
        var_nja11_dn0 = assign30810_e46120_d_n0;
        var_nja11_dn1 = assign30810_e46120_d_n1;
        var_nja11_dn2 = assign30810_e46120_d_n2;
        var_nja11_dn3 = assign30810_e46120_d_n3;
        var_nja11_dn4 = assign30810_e46120_d_n4;
        var_nja11_dn5 = assign30810_e46120_d_n5;
        var_nja11_db0 = assign30810_e46120_d_b0;
        var_nja11_db1 = assign30810_e46120_d_b1;
        var_nja11_db2 = assign30810_e46120_d_b2;
        var_nja11_db3 = assign30810_e46120_d_b3;

        let (assign30820_e46136, assign30820_e46136_d_n0, assign30820_e46136_d_n1, assign30820_e46136_d_n2, assign30820_e46136_d_n3, assign30820_e46136_d_n4, assign30820_e46136_d_n5, assign30820_e46136_d_b0, assign30820_e46136_d_b1, assign30820_e46136_d_b2, assign30820_e46136_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30820_e46132: f64 = (var_nja11 - var_nfasti_i);
        let assign30820_e46134: f64 = (assign30820_e46132 - 0.01);
        (assign30820_e46134, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30820_e46136;
        var_tmf1_dn0 = assign30820_e46136_d_n0;
        var_tmf1_dn1 = assign30820_e46136_d_n1;
        var_tmf1_dn2 = assign30820_e46136_d_n2;
        var_tmf1_dn3 = assign30820_e46136_d_n3;
        var_tmf1_dn4 = assign30820_e46136_d_n4;
        var_tmf1_dn5 = assign30820_e46136_d_n5;
        var_tmf1_db0 = assign30820_e46136_d_b0;
        var_tmf1_db1 = assign30820_e46136_d_b1;
        var_tmf1_db2 = assign30820_e46136_d_b2;
        var_tmf1_db3 = assign30820_e46136_d_b3;

        let (assign30830_e46152, assign30830_e46152_d_n0, assign30830_e46152_d_n1, assign30830_e46152_d_n2, assign30830_e46152_d_n3, assign30830_e46152_d_n4, assign30830_e46152_d_n5, assign30830_e46152_d_b0, assign30830_e46152_d_b1, assign30830_e46152_d_b2, assign30830_e46152_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30830_e46148: f64 = (4.0 * var_nfasti_i);
        let assign30830_e46150: f64 = (assign30830_e46148 * 0.01);
        (assign30830_e46150, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30830_e46152;
        var_tmf2_dn0 = assign30830_e46152_d_n0;
        var_tmf2_dn1 = assign30830_e46152_d_n1;
        var_tmf2_dn2 = assign30830_e46152_d_n2;
        var_tmf2_dn3 = assign30830_e46152_d_n3;
        var_tmf2_dn4 = assign30830_e46152_d_n4;
        var_tmf2_dn5 = assign30830_e46152_d_n5;
        var_tmf2_db0 = assign30830_e46152_d_b0;
        var_tmf2_db1 = assign30830_e46152_d_b1;
        var_tmf2_db2 = assign30830_e46152_d_b2;
        var_tmf2_db3 = assign30830_e46152_d_b3;

        let (assign30840_e46170, assign30840_e46170_d_n0, assign30840_e46170_d_n1, assign30840_e46170_d_n2, assign30840_e46170_d_n3, assign30840_e46170_d_n4, assign30840_e46170_d_n5, assign30840_e46170_d_b0, assign30840_e46170_d_b1, assign30840_e46170_d_b2, assign30840_e46170_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n1, assign30840_e46168_d_n2, assign30840_e46168_d_n3, assign30840_e46168_d_n4, assign30840_e46168_d_n5, assign30840_e46168_d_b0, assign30840_e46168_d_b1, assign30840_e46168_d_b2, assign30840_e46168_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30840_e46167: f64 = (-var_tmf2);
                (assign30840_e46167, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n1, assign30840_e46168_d_n2, assign30840_e46168_d_n3, assign30840_e46168_d_n4, assign30840_e46168_d_n5, assign30840_e46168_d_b0, assign30840_e46168_d_b1, assign30840_e46168_d_b2, assign30840_e46168_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30840_e46170;
        var_tmf2_dn0 = assign30840_e46170_d_n0;
        var_tmf2_dn1 = assign30840_e46170_d_n1;
        var_tmf2_dn2 = assign30840_e46170_d_n2;
        var_tmf2_dn3 = assign30840_e46170_d_n3;
        var_tmf2_dn4 = assign30840_e46170_d_n4;
        var_tmf2_dn5 = assign30840_e46170_d_n5;
        var_tmf2_db0 = assign30840_e46170_d_b0;
        var_tmf2_db1 = assign30840_e46170_d_b1;
        var_tmf2_db2 = assign30840_e46170_d_b2;
        var_tmf2_db3 = assign30840_e46170_d_b3;

        let (assign30850_e46187, assign30850_e46187_d_n0, assign30850_e46187_d_n1, assign30850_e46187_d_n2, assign30850_e46187_d_n3, assign30850_e46187_d_n4, assign30850_e46187_d_n5, assign30850_e46187_d_b0, assign30850_e46187_d_b1, assign30850_e46187_d_b2, assign30850_e46187_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30850_e46182: f64 = (var_tmf1 * var_tmf1);
        let assign30850_e46184: f64 = (assign30850_e46182 + var_tmf2);
        let assign30850_e46185: f64 = (assign30850_e46184).sqrt();
        (assign30850_e46185, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30850_e46185)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30850_e46185)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30850_e46185)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30850_e46185)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30850_e46185)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30850_e46187;
        var_tmf2_dn0 = assign30850_e46187_d_n0;
        var_tmf2_dn1 = assign30850_e46187_d_n1;
        var_tmf2_dn2 = assign30850_e46187_d_n2;
        var_tmf2_dn3 = assign30850_e46187_d_n3;
        var_tmf2_dn4 = assign30850_e46187_d_n4;
        var_tmf2_dn5 = assign30850_e46187_d_n5;
        var_tmf2_db0 = assign30850_e46187_d_b0;
        var_tmf2_db1 = assign30850_e46187_d_b1;
        var_tmf2_db2 = assign30850_e46187_d_b2;
        var_tmf2_db3 = assign30850_e46187_d_b3;

        let (assign30860_e46205, assign30860_e46205_d_n0, assign30860_e46205_d_n1, assign30860_e46205_d_n2, assign30860_e46205_d_n3, assign30860_e46205_d_n4, assign30860_e46205_d_n5, assign30860_e46205_d_b0, assign30860_e46205_d_b1, assign30860_e46205_d_b2, assign30860_e46205_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30860_e46201: f64 = (var_tmf1 / var_tmf2);
        let assign30860_e46202: f64 = (1.0 + assign30860_e46201);
        let assign30860_e46203: f64 = (0.5 * assign30860_e46202);
        (assign30860_e46203, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign30860_e46205;
        var_dfn_sl_dn0 = assign30860_e46205_d_n0;
        var_dfn_sl_dn1 = assign30860_e46205_d_n1;
        var_dfn_sl_dn2 = assign30860_e46205_d_n2;
        var_dfn_sl_dn3 = assign30860_e46205_d_n3;
        var_dfn_sl_dn4 = assign30860_e46205_d_n4;
        var_dfn_sl_dn5 = assign30860_e46205_d_n5;
        var_dfn_sl_db0 = assign30860_e46205_d_b0;
        var_dfn_sl_db1 = assign30860_e46205_d_b1;
        var_dfn_sl_db2 = assign30860_e46205_d_b2;
        var_dfn_sl_db3 = assign30860_e46205_d_b3;

        let (assign30870_e46223, assign30870_e46223_d_n0, assign30870_e46223_d_n1, assign30870_e46223_d_n2, assign30870_e46223_d_n3, assign30870_e46223_d_n4, assign30870_e46223_d_n5, assign30870_e46223_d_b0, assign30870_e46223_d_b1, assign30870_e46223_d_b2, assign30870_e46223_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30870_e46219: f64 = (var_tmf1 + var_tmf2);
        let assign30870_e46220: f64 = (0.5 * assign30870_e46219);
        let assign30870_e46221: f64 = (var_nfasti_i + assign30870_e46220);
        (assign30870_e46221, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign30870_e46223;
        var_nj1_dn0 = assign30870_e46223_d_n0;
        var_nj1_dn1 = assign30870_e46223_d_n1;
        var_nj1_dn2 = assign30870_e46223_d_n2;
        var_nj1_dn3 = assign30870_e46223_d_n3;
        var_nj1_dn4 = assign30870_e46223_d_n4;
        var_nj1_dn5 = assign30870_e46223_d_n5;
        var_nj1_db0 = assign30870_e46223_d_b0;
        var_nj1_db1 = assign30870_e46223_d_b1;
        var_nj1_db2 = assign30870_e46223_d_b2;
        var_nj1_db3 = assign30870_e46223_d_b3;

        let (assign30880_e46239, assign30880_e46239_d_n0, assign30880_e46239_d_n1, assign30880_e46239_d_n2, assign30880_e46239_d_n3, assign30880_e46239_d_n4, assign30880_e46239_d_n5, assign30880_e46239_d_b0, assign30880_e46239_d_b1, assign30880_e46239_d_b2, assign30880_e46239_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30880_e46235: f64 = (p.p85 - var_nj0);
        let assign30880_e46237: f64 = (assign30880_e46235 - 0.01);
        (assign30880_e46237, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30880_e46239;
        var_tmf1_dn0 = assign30880_e46239_d_n0;
        var_tmf1_dn1 = assign30880_e46239_d_n1;
        var_tmf1_dn2 = assign30880_e46239_d_n2;
        var_tmf1_dn3 = assign30880_e46239_d_n3;
        var_tmf1_dn4 = assign30880_e46239_d_n4;
        var_tmf1_dn5 = assign30880_e46239_d_n5;
        var_tmf1_db0 = assign30880_e46239_d_b0;
        var_tmf1_db1 = assign30880_e46239_d_b1;
        var_tmf1_db2 = assign30880_e46239_d_b2;
        var_tmf1_db3 = assign30880_e46239_d_b3;

        let (assign30890_e46255, assign30890_e46255_d_n0, assign30890_e46255_d_n1, assign30890_e46255_d_n2, assign30890_e46255_d_n3, assign30890_e46255_d_n4, assign30890_e46255_d_n5, assign30890_e46255_d_b0, assign30890_e46255_d_b1, assign30890_e46255_d_b2, assign30890_e46255_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30890_e46251: f64 = (4.0 * p.p85);
        let assign30890_e46253: f64 = (assign30890_e46251 * 0.01);
        (assign30890_e46253, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30890_e46255;
        var_tmf2_dn0 = assign30890_e46255_d_n0;
        var_tmf2_dn1 = assign30890_e46255_d_n1;
        var_tmf2_dn2 = assign30890_e46255_d_n2;
        var_tmf2_dn3 = assign30890_e46255_d_n3;
        var_tmf2_dn4 = assign30890_e46255_d_n4;
        var_tmf2_dn5 = assign30890_e46255_d_n5;
        var_tmf2_db0 = assign30890_e46255_d_b0;
        var_tmf2_db1 = assign30890_e46255_d_b1;
        var_tmf2_db2 = assign30890_e46255_d_b2;
        var_tmf2_db3 = assign30890_e46255_d_b3;

        let (assign30900_e46273, assign30900_e46273_d_n0, assign30900_e46273_d_n1, assign30900_e46273_d_n2, assign30900_e46273_d_n3, assign30900_e46273_d_n4, assign30900_e46273_d_n5, assign30900_e46273_d_b0, assign30900_e46273_d_b1, assign30900_e46273_d_b2, assign30900_e46273_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n1, assign30900_e46271_d_n2, assign30900_e46271_d_n3, assign30900_e46271_d_n4, assign30900_e46271_d_n5, assign30900_e46271_d_b0, assign30900_e46271_d_b1, assign30900_e46271_d_b2, assign30900_e46271_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30900_e46270: f64 = (-var_tmf2);
                (assign30900_e46270, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n1, assign30900_e46271_d_n2, assign30900_e46271_d_n3, assign30900_e46271_d_n4, assign30900_e46271_d_n5, assign30900_e46271_d_b0, assign30900_e46271_d_b1, assign30900_e46271_d_b2, assign30900_e46271_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30900_e46273;
        var_tmf2_dn0 = assign30900_e46273_d_n0;
        var_tmf2_dn1 = assign30900_e46273_d_n1;
        var_tmf2_dn2 = assign30900_e46273_d_n2;
        var_tmf2_dn3 = assign30900_e46273_d_n3;
        var_tmf2_dn4 = assign30900_e46273_d_n4;
        var_tmf2_dn5 = assign30900_e46273_d_n5;
        var_tmf2_db0 = assign30900_e46273_d_b0;
        var_tmf2_db1 = assign30900_e46273_d_b1;
        var_tmf2_db2 = assign30900_e46273_d_b2;
        var_tmf2_db3 = assign30900_e46273_d_b3;

        let (assign30910_e46290, assign30910_e46290_d_n0, assign30910_e46290_d_n1, assign30910_e46290_d_n2, assign30910_e46290_d_n3, assign30910_e46290_d_n4, assign30910_e46290_d_n5, assign30910_e46290_d_b0, assign30910_e46290_d_b1, assign30910_e46290_d_b2, assign30910_e46290_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30910_e46285: f64 = (var_tmf1 * var_tmf1);
        let assign30910_e46287: f64 = (assign30910_e46285 + var_tmf2);
        let assign30910_e46288: f64 = (assign30910_e46287).sqrt();
        (assign30910_e46288, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30910_e46288)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30910_e46288)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30910_e46288)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30910_e46288)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30910_e46288)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30910_e46290;
        var_tmf2_dn0 = assign30910_e46290_d_n0;
        var_tmf2_dn1 = assign30910_e46290_d_n1;
        var_tmf2_dn2 = assign30910_e46290_d_n2;
        var_tmf2_dn3 = assign30910_e46290_d_n3;
        var_tmf2_dn4 = assign30910_e46290_d_n4;
        var_tmf2_dn5 = assign30910_e46290_d_n5;
        var_tmf2_db0 = assign30910_e46290_d_b0;
        var_tmf2_db1 = assign30910_e46290_d_b1;
        var_tmf2_db2 = assign30910_e46290_d_b2;
        var_tmf2_db3 = assign30910_e46290_d_b3;

        let (assign30920_e46308, assign30920_e46308_d_n0, assign30920_e46308_d_n1, assign30920_e46308_d_n2, assign30920_e46308_d_n3, assign30920_e46308_d_n4, assign30920_e46308_d_n5, assign30920_e46308_d_b0, assign30920_e46308_d_b1, assign30920_e46308_d_b2, assign30920_e46308_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30920_e46304: f64 = (var_tmf1 + var_tmf2);
        let assign30920_e46305: f64 = (0.5 * assign30920_e46304);
        let assign30920_e46306: f64 = (p.p85 - assign30920_e46305);
        (assign30920_e46306, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30920_e46308;
        var_nj0_dn0 = assign30920_e46308_d_n0;
        var_nj0_dn1 = assign30920_e46308_d_n1;
        var_nj0_dn2 = assign30920_e46308_d_n2;
        var_nj0_dn3 = assign30920_e46308_d_n3;
        var_nj0_dn4 = assign30920_e46308_d_n4;
        var_nj0_dn5 = assign30920_e46308_d_n5;
        var_nj0_db0 = assign30920_e46308_d_b0;
        var_nj0_db1 = assign30920_e46308_d_b1;
        var_nj0_db2 = assign30920_e46308_d_b2;
        var_nj0_db3 = assign30920_e46308_d_b3;

        let (assign30930_e46324, assign30930_e46324_d_n0, assign30930_e46324_d_n1, assign30930_e46324_d_n2, assign30930_e46324_d_n3, assign30930_e46324_d_n4, assign30930_e46324_d_n5, assign30930_e46324_d_b0, assign30930_e46324_d_b1, assign30930_e46324_d_b2, assign30930_e46324_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30930_e46320: f64 = (var_nj0 - var_nfasti_i);
        let assign30930_e46322: f64 = (assign30930_e46320 - 0.01);
        (assign30930_e46322, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign30930_e46324;
        var_tmf1_dn0 = assign30930_e46324_d_n0;
        var_tmf1_dn1 = assign30930_e46324_d_n1;
        var_tmf1_dn2 = assign30930_e46324_d_n2;
        var_tmf1_dn3 = assign30930_e46324_d_n3;
        var_tmf1_dn4 = assign30930_e46324_d_n4;
        var_tmf1_dn5 = assign30930_e46324_d_n5;
        var_tmf1_db0 = assign30930_e46324_d_b0;
        var_tmf1_db1 = assign30930_e46324_d_b1;
        var_tmf1_db2 = assign30930_e46324_d_b2;
        var_tmf1_db3 = assign30930_e46324_d_b3;

        let (assign30940_e46340, assign30940_e46340_d_n0, assign30940_e46340_d_n1, assign30940_e46340_d_n2, assign30940_e46340_d_n3, assign30940_e46340_d_n4, assign30940_e46340_d_n5, assign30940_e46340_d_b0, assign30940_e46340_d_b1, assign30940_e46340_d_b2, assign30940_e46340_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30940_e46336: f64 = (4.0 * var_nfasti_i);
        let assign30940_e46338: f64 = (assign30940_e46336 * 0.01);
        (assign30940_e46338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30940_e46340;
        var_tmf2_dn0 = assign30940_e46340_d_n0;
        var_tmf2_dn1 = assign30940_e46340_d_n1;
        var_tmf2_dn2 = assign30940_e46340_d_n2;
        var_tmf2_dn3 = assign30940_e46340_d_n3;
        var_tmf2_dn4 = assign30940_e46340_d_n4;
        var_tmf2_dn5 = assign30940_e46340_d_n5;
        var_tmf2_db0 = assign30940_e46340_d_b0;
        var_tmf2_db1 = assign30940_e46340_d_b1;
        var_tmf2_db2 = assign30940_e46340_d_b2;
        var_tmf2_db3 = assign30940_e46340_d_b3;

        let (assign30950_e46358, assign30950_e46358_d_n0, assign30950_e46358_d_n1, assign30950_e46358_d_n2, assign30950_e46358_d_n3, assign30950_e46358_d_n4, assign30950_e46358_d_n5, assign30950_e46358_d_b0, assign30950_e46358_d_b1, assign30950_e46358_d_b2, assign30950_e46358_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n1, assign30950_e46356_d_n2, assign30950_e46356_d_n3, assign30950_e46356_d_n4, assign30950_e46356_d_n5, assign30950_e46356_d_b0, assign30950_e46356_d_b1, assign30950_e46356_d_b2, assign30950_e46356_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign30950_e46355: f64 = (-var_tmf2);
                (assign30950_e46355, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n1, assign30950_e46356_d_n2, assign30950_e46356_d_n3, assign30950_e46356_d_n4, assign30950_e46356_d_n5, assign30950_e46356_d_b0, assign30950_e46356_d_b1, assign30950_e46356_d_b2, assign30950_e46356_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30950_e46358;
        var_tmf2_dn0 = assign30950_e46358_d_n0;
        var_tmf2_dn1 = assign30950_e46358_d_n1;
        var_tmf2_dn2 = assign30950_e46358_d_n2;
        var_tmf2_dn3 = assign30950_e46358_d_n3;
        var_tmf2_dn4 = assign30950_e46358_d_n4;
        var_tmf2_dn5 = assign30950_e46358_d_n5;
        var_tmf2_db0 = assign30950_e46358_d_b0;
        var_tmf2_db1 = assign30950_e46358_d_b1;
        var_tmf2_db2 = assign30950_e46358_d_b2;
        var_tmf2_db3 = assign30950_e46358_d_b3;

        let (assign30960_e46375, assign30960_e46375_d_n0, assign30960_e46375_d_n1, assign30960_e46375_d_n2, assign30960_e46375_d_n3, assign30960_e46375_d_n4, assign30960_e46375_d_n5, assign30960_e46375_d_b0, assign30960_e46375_d_b1, assign30960_e46375_d_b2, assign30960_e46375_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30960_e46370: f64 = (var_tmf1 * var_tmf1);
        let assign30960_e46372: f64 = (assign30960_e46370 + var_tmf2);
        let assign30960_e46373: f64 = (assign30960_e46372).sqrt();
        (assign30960_e46373, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign30960_e46373)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign30960_e46373)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign30960_e46373)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign30960_e46373)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign30960_e46373)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign30960_e46375;
        var_tmf2_dn0 = assign30960_e46375_d_n0;
        var_tmf2_dn1 = assign30960_e46375_d_n1;
        var_tmf2_dn2 = assign30960_e46375_d_n2;
        var_tmf2_dn3 = assign30960_e46375_d_n3;
        var_tmf2_dn4 = assign30960_e46375_d_n4;
        var_tmf2_dn5 = assign30960_e46375_d_n5;
        var_tmf2_db0 = assign30960_e46375_d_b0;
        var_tmf2_db1 = assign30960_e46375_d_b1;
        var_tmf2_db2 = assign30960_e46375_d_b2;
        var_tmf2_db3 = assign30960_e46375_d_b3;

        let (assign30970_e46393, assign30970_e46393_d_n0, assign30970_e46393_d_n1, assign30970_e46393_d_n2, assign30970_e46393_d_n3, assign30970_e46393_d_n4, assign30970_e46393_d_n5, assign30970_e46393_d_b0, assign30970_e46393_d_b1, assign30970_e46393_d_b2, assign30970_e46393_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30970_e46389: f64 = (var_tmf1 + var_tmf2);
        let assign30970_e46390: f64 = (0.5 * assign30970_e46389);
        let assign30970_e46391: f64 = (var_nfasti_i + assign30970_e46390);
        (assign30970_e46391, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30970_e46393;
        var_nj0_dn0 = assign30970_e46393_d_n0;
        var_nj0_dn1 = assign30970_e46393_d_n1;
        var_nj0_dn2 = assign30970_e46393_d_n2;
        var_nj0_dn3 = assign30970_e46393_d_n3;
        var_nj0_dn4 = assign30970_e46393_d_n4;
        var_nj0_dn5 = assign30970_e46393_d_n5;
        var_nj0_db0 = assign30970_e46393_d_b0;
        var_nj0_db1 = assign30970_e46393_d_b1;
        var_nj0_db2 = assign30970_e46393_d_b2;
        var_nj0_db3 = assign30970_e46393_d_b3;

        let (assign30980_e46409, assign30980_e46409_d_n0, assign30980_e46409_d_n1, assign30980_e46409_d_n2, assign30980_e46409_d_n3, assign30980_e46409_d_n4, assign30980_e46409_d_n5, assign30980_e46409_d_b0, assign30980_e46409_d_b1, assign30980_e46409_d_b2, assign30980_e46409_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30980_e46405: f64 = (p.p86 * var_dfn_su);
        let assign30980_e46407: f64 = (assign30980_e46405 * var_dfn_sl);
        (assign30980_e46407, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign30980_e46409;
        var_dnj1_dv_dn0 = assign30980_e46409_d_n0;
        var_dnj1_dv_dn1 = assign30980_e46409_d_n1;
        var_dnj1_dv_dn2 = assign30980_e46409_d_n2;
        var_dnj1_dv_dn3 = assign30980_e46409_d_n3;
        var_dnj1_dv_dn4 = assign30980_e46409_d_n4;
        var_dnj1_dv_dn5 = assign30980_e46409_d_n5;
        var_dnj1_dv_db0 = assign30980_e46409_d_b0;
        var_dnj1_dv_db1 = assign30980_e46409_d_b1;
        var_dnj1_dv_db2 = assign30980_e46409_d_b2;
        var_dnj1_dv_db3 = assign30980_e46409_d_b3;


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

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard495: f64,
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
        var_guard498_slot: &mut f64,
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
        let mut var_guard498: f64 = *var_guard498_slot;
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

        let (assign30990_e46422, assign30990_e46422_d_n0, assign30990_e46422_d_n1, assign30990_e46422_d_n2, assign30990_e46422_d_n3, assign30990_e46422_d_n4, assign30990_e46422_d_n5, assign30990_e46422_d_b0, assign30990_e46422_d_b1, assign30990_e46422_d_b2, assign30990_e46422_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign30990_e46422;
        var_nj0_dn0 = assign30990_e46422_d_n0;
        var_nj0_dn1 = assign30990_e46422_d_n1;
        var_nj0_dn2 = assign30990_e46422_d_n2;
        var_nj0_dn3 = assign30990_e46422_d_n3;
        var_nj0_dn4 = assign30990_e46422_d_n4;
        var_nj0_dn5 = assign30990_e46422_d_n5;
        var_nj0_db0 = assign30990_e46422_d_b0;
        var_nj0_db1 = assign30990_e46422_d_b1;
        var_nj0_db2 = assign30990_e46422_d_b2;
        var_nj0_db3 = assign30990_e46422_d_b3;

        let (assign31000_e46435, assign31000_e46435_d_n0, assign31000_e46435_d_n1, assign31000_e46435_d_n2, assign31000_e46435_d_n3, assign31000_e46435_d_n4, assign31000_e46435_d_n5, assign31000_e46435_d_b0, assign31000_e46435_d_b1, assign31000_e46435_d_b2, assign31000_e46435_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31000_e46435;
        var_nj1_dn0 = assign31000_e46435_d_n0;
        var_nj1_dn1 = assign31000_e46435_d_n1;
        var_nj1_dn2 = assign31000_e46435_d_n2;
        var_nj1_dn3 = assign31000_e46435_d_n3;
        var_nj1_dn4 = assign31000_e46435_d_n4;
        var_nj1_dn5 = assign31000_e46435_d_n5;
        var_nj1_db0 = assign31000_e46435_d_b0;
        var_nj1_db1 = assign31000_e46435_d_b1;
        var_nj1_db2 = assign31000_e46435_d_b2;
        var_nj1_db3 = assign31000_e46435_d_b3;

        let (assign31010_e46448, assign31010_e46448_d_n0, assign31010_e46448_d_n1, assign31010_e46448_d_n2, assign31010_e46448_d_n3, assign31010_e46448_d_n4, assign31010_e46448_d_n5, assign31010_e46448_d_b0, assign31010_e46448_d_b1, assign31010_e46448_d_b2, assign31010_e46448_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31010_e46448;
        var_dnj1_dv_dn0 = assign31010_e46448_d_n0;
        var_dnj1_dv_dn1 = assign31010_e46448_d_n1;
        var_dnj1_dv_dn2 = assign31010_e46448_d_n2;
        var_dnj1_dv_dn3 = assign31010_e46448_d_n3;
        var_dnj1_dv_dn4 = assign31010_e46448_d_n4;
        var_dnj1_dv_dn5 = assign31010_e46448_d_n5;
        var_dnj1_dv_db0 = assign31010_e46448_d_b0;
        var_dnj1_dv_db1 = assign31010_e46448_d_b1;
        var_dnj1_dv_db2 = assign31010_e46448_d_b2;
        var_dnj1_dv_db3 = assign31010_e46448_d_b3;

        let (assign31070_e46701, assign31070_e46701_d_n0, assign31070_e46701_d_n1, assign31070_e46701_d_n2, assign31070_e46701_d_n3, assign31070_e46701_d_n4, assign31070_e46701_d_n5, assign31070_e46701_d_b0, assign31070_e46701_d_b1, assign31070_e46701_d_b2, assign31070_e46701_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31070_e46685: f64 = (var_vmax * var_dnj1_dv);
        let assign31070_e46686: f64 = (var_nj1 - assign31070_e46685);
        let assign31070_e46689: f64 = (var_nj1 * var_nj1);
        let assign31070_e46690: f64 = (assign31070_e46686 / assign31070_e46689);
        let assign31070_e46693: f64 = (var_vha1 * var_dnj1_dv);
        let assign31070_e46696: f64 = (var_nj0 * p.p85);
        let assign31070_e46697: f64 = (assign31070_e46693 / assign31070_e46696);
        let assign31070_e46698: f64 = (assign31070_e46690 + assign31070_e46697);
        let assign31070_e46699: f64 = (var_phitdinv * assign31070_e46698);
        (assign31070_e46699, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn0 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn1 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn2 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn3 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn4 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn5 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_db0) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db0 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_db1) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db1 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_db2) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db2 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_db3) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_db3 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign31070_e46701;
        var_dvmax_over_phitd_dv_dn0 = assign31070_e46701_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign31070_e46701_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign31070_e46701_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign31070_e46701_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign31070_e46701_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign31070_e46701_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign31070_e46701_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign31070_e46701_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign31070_e46701_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign31070_e46701_d_b3;

        let (assign31090_e46733,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31090_e46729: f64 = (var_nin * var_nin);
        let assign31090_e46731: f64 = (assign31090_e46729 / var_ndigat_i);
        (assign31090_e46731,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign31090_e46733;

        let (assign31100_e46750,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31100_e46743: f64 = (var_nfagat_i / var_phitdinv);
        let assign31100_e46746: f64 = (var_ndigat_i / var_pnn0);
        let assign31100_e46747: f64 = (assign31100_e46746).ln();
        let assign31100_e46748: f64 = (assign31100_e46743 * assign31100_e46747);
        (assign31100_e46748,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign31100_e46750;

        let assign31110_e46753: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard498 = assign31110_e46753;

        let (assign31120_e46771, assign31120_e46771_d_n0, assign31120_e46771_d_n1, assign31120_e46771_d_n2, assign31120_e46771_d_n3, assign31120_e46771_d_n4, assign31120_e46771_d_n5, assign31120_e46771_d_b0, assign31120_e46771_d_b1, assign31120_e46771_d_b2, assign31120_e46771_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31120_e46766: f64 = (var_vmax - var_vha1);
        let assign31120_e46767: f64 = (p.p86 * assign31120_e46766);
        let assign31120_e46769: f64 = (assign31120_e46767 + var_nfagat_i);
        (assign31120_e46769, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn1, var_nja10_dn2, var_nja10_dn3, var_nja10_dn4, var_nja10_dn5, var_nja10_db0, var_nja10_db1, var_nja10_db2, var_nja10_db3,)
    }
};
        var_nja10 = assign31120_e46771;
        var_nja10_dn0 = assign31120_e46771_d_n0;
        var_nja10_dn1 = assign31120_e46771_d_n1;
        var_nja10_dn2 = assign31120_e46771_d_n2;
        var_nja10_dn3 = assign31120_e46771_d_n3;
        var_nja10_dn4 = assign31120_e46771_d_n4;
        var_nja10_dn5 = assign31120_e46771_d_n5;
        var_nja10_db0 = assign31120_e46771_d_b0;
        var_nja10_db1 = assign31120_e46771_d_b1;
        var_nja10_db2 = assign31120_e46771_d_b2;
        var_nja10_db3 = assign31120_e46771_d_b3;

        let (assign31130_e46787, assign31130_e46787_d_n0, assign31130_e46787_d_n1, assign31130_e46787_d_n2, assign31130_e46787_d_n3, assign31130_e46787_d_n4, assign31130_e46787_d_n5, assign31130_e46787_d_b0, assign31130_e46787_d_b1, assign31130_e46787_d_b2, assign31130_e46787_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31130_e46784: f64 = (p.p86 * var_vha1);
        let assign31130_e46785: f64 = (var_nfagat_i - assign31130_e46784);
        (assign31130_e46785, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31130_e46787;
        var_nj0_dn0 = assign31130_e46787_d_n0;
        var_nj0_dn1 = assign31130_e46787_d_n1;
        var_nj0_dn2 = assign31130_e46787_d_n2;
        var_nj0_dn3 = assign31130_e46787_d_n3;
        var_nj0_dn4 = assign31130_e46787_d_n4;
        var_nj0_dn5 = assign31130_e46787_d_n5;
        var_nj0_db0 = assign31130_e46787_d_b0;
        var_nj0_db1 = assign31130_e46787_d_b1;
        var_nj0_db2 = assign31130_e46787_d_b2;
        var_nj0_db3 = assign31130_e46787_d_b3;

        let (assign31140_e46803, assign31140_e46803_d_n0, assign31140_e46803_d_n1, assign31140_e46803_d_n2, assign31140_e46803_d_n3, assign31140_e46803_d_n4, assign31140_e46803_d_n5, assign31140_e46803_d_b0, assign31140_e46803_d_b1, assign31140_e46803_d_b2, assign31140_e46803_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31140_e46799: f64 = (p.p85 - var_nja10);
        let assign31140_e46801: f64 = (assign31140_e46799 - 0.01);
        (assign31140_e46801, (-var_nja10_dn0), (-var_nja10_dn1), (-var_nja10_dn2), (-var_nja10_dn3), (-var_nja10_dn4), (-var_nja10_dn5), (-var_nja10_db0), (-var_nja10_db1), (-var_nja10_db2), (-var_nja10_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31140_e46803;
        var_tmf1_dn0 = assign31140_e46803_d_n0;
        var_tmf1_dn1 = assign31140_e46803_d_n1;
        var_tmf1_dn2 = assign31140_e46803_d_n2;
        var_tmf1_dn3 = assign31140_e46803_d_n3;
        var_tmf1_dn4 = assign31140_e46803_d_n4;
        var_tmf1_dn5 = assign31140_e46803_d_n5;
        var_tmf1_db0 = assign31140_e46803_d_b0;
        var_tmf1_db1 = assign31140_e46803_d_b1;
        var_tmf1_db2 = assign31140_e46803_d_b2;
        var_tmf1_db3 = assign31140_e46803_d_b3;

        let (assign31150_e46819, assign31150_e46819_d_n0, assign31150_e46819_d_n1, assign31150_e46819_d_n2, assign31150_e46819_d_n3, assign31150_e46819_d_n4, assign31150_e46819_d_n5, assign31150_e46819_d_b0, assign31150_e46819_d_b1, assign31150_e46819_d_b2, assign31150_e46819_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31150_e46815: f64 = (4.0 * p.p85);
        let assign31150_e46817: f64 = (assign31150_e46815 * 0.01);
        (assign31150_e46817, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31150_e46819;
        var_tmf2_dn0 = assign31150_e46819_d_n0;
        var_tmf2_dn1 = assign31150_e46819_d_n1;
        var_tmf2_dn2 = assign31150_e46819_d_n2;
        var_tmf2_dn3 = assign31150_e46819_d_n3;
        var_tmf2_dn4 = assign31150_e46819_d_n4;
        var_tmf2_dn5 = assign31150_e46819_d_n5;
        var_tmf2_db0 = assign31150_e46819_d_b0;
        var_tmf2_db1 = assign31150_e46819_d_b1;
        var_tmf2_db2 = assign31150_e46819_d_b2;
        var_tmf2_db3 = assign31150_e46819_d_b3;

        let (assign31160_e46837, assign31160_e46837_d_n0, assign31160_e46837_d_n1, assign31160_e46837_d_n2, assign31160_e46837_d_n3, assign31160_e46837_d_n4, assign31160_e46837_d_n5, assign31160_e46837_d_b0, assign31160_e46837_d_b1, assign31160_e46837_d_b2, assign31160_e46837_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n1, assign31160_e46835_d_n2, assign31160_e46835_d_n3, assign31160_e46835_d_n4, assign31160_e46835_d_n5, assign31160_e46835_d_b0, assign31160_e46835_d_b1, assign31160_e46835_d_b2, assign31160_e46835_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31160_e46834: f64 = (-var_tmf2);
                (assign31160_e46834, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n1, assign31160_e46835_d_n2, assign31160_e46835_d_n3, assign31160_e46835_d_n4, assign31160_e46835_d_n5, assign31160_e46835_d_b0, assign31160_e46835_d_b1, assign31160_e46835_d_b2, assign31160_e46835_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31160_e46837;
        var_tmf2_dn0 = assign31160_e46837_d_n0;
        var_tmf2_dn1 = assign31160_e46837_d_n1;
        var_tmf2_dn2 = assign31160_e46837_d_n2;
        var_tmf2_dn3 = assign31160_e46837_d_n3;
        var_tmf2_dn4 = assign31160_e46837_d_n4;
        var_tmf2_dn5 = assign31160_e46837_d_n5;
        var_tmf2_db0 = assign31160_e46837_d_b0;
        var_tmf2_db1 = assign31160_e46837_d_b1;
        var_tmf2_db2 = assign31160_e46837_d_b2;
        var_tmf2_db3 = assign31160_e46837_d_b3;

        let (assign31170_e46854, assign31170_e46854_d_n0, assign31170_e46854_d_n1, assign31170_e46854_d_n2, assign31170_e46854_d_n3, assign31170_e46854_d_n4, assign31170_e46854_d_n5, assign31170_e46854_d_b0, assign31170_e46854_d_b1, assign31170_e46854_d_b2, assign31170_e46854_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31170_e46849: f64 = (var_tmf1 * var_tmf1);
        let assign31170_e46851: f64 = (assign31170_e46849 + var_tmf2);
        let assign31170_e46852: f64 = (assign31170_e46851).sqrt();
        (assign31170_e46852, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31170_e46852)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31170_e46852)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31170_e46852)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31170_e46852)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31170_e46852)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31170_e46854;
        var_tmf2_dn0 = assign31170_e46854_d_n0;
        var_tmf2_dn1 = assign31170_e46854_d_n1;
        var_tmf2_dn2 = assign31170_e46854_d_n2;
        var_tmf2_dn3 = assign31170_e46854_d_n3;
        var_tmf2_dn4 = assign31170_e46854_d_n4;
        var_tmf2_dn5 = assign31170_e46854_d_n5;
        var_tmf2_db0 = assign31170_e46854_d_b0;
        var_tmf2_db1 = assign31170_e46854_d_b1;
        var_tmf2_db2 = assign31170_e46854_d_b2;
        var_tmf2_db3 = assign31170_e46854_d_b3;

        let (assign31180_e46872, assign31180_e46872_d_n0, assign31180_e46872_d_n1, assign31180_e46872_d_n2, assign31180_e46872_d_n3, assign31180_e46872_d_n4, assign31180_e46872_d_n5, assign31180_e46872_d_b0, assign31180_e46872_d_b1, assign31180_e46872_d_b2, assign31180_e46872_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31180_e46868: f64 = (var_tmf1 / var_tmf2);
        let assign31180_e46869: f64 = (1.0 + assign31180_e46868);
        let assign31180_e46870: f64 = (0.5 * assign31180_e46869);
        (assign31180_e46870, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn1, var_dfn_su_dn2, var_dfn_su_dn3, var_dfn_su_dn4, var_dfn_su_dn5, var_dfn_su_db0, var_dfn_su_db1, var_dfn_su_db2, var_dfn_su_db3,)
    }
};
        var_dfn_su = assign31180_e46872;
        var_dfn_su_dn0 = assign31180_e46872_d_n0;
        var_dfn_su_dn1 = assign31180_e46872_d_n1;
        var_dfn_su_dn2 = assign31180_e46872_d_n2;
        var_dfn_su_dn3 = assign31180_e46872_d_n3;
        var_dfn_su_dn4 = assign31180_e46872_d_n4;
        var_dfn_su_dn5 = assign31180_e46872_d_n5;
        var_dfn_su_db0 = assign31180_e46872_d_b0;
        var_dfn_su_db1 = assign31180_e46872_d_b1;
        var_dfn_su_db2 = assign31180_e46872_d_b2;
        var_dfn_su_db3 = assign31180_e46872_d_b3;

        let (assign31190_e46890, assign31190_e46890_d_n0, assign31190_e46890_d_n1, assign31190_e46890_d_n2, assign31190_e46890_d_n3, assign31190_e46890_d_n4, assign31190_e46890_d_n5, assign31190_e46890_d_b0, assign31190_e46890_d_b1, assign31190_e46890_d_b2, assign31190_e46890_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31190_e46886: f64 = (var_tmf1 + var_tmf2);
        let assign31190_e46887: f64 = (0.5 * assign31190_e46886);
        let assign31190_e46888: f64 = (p.p85 - assign31190_e46887);
        (assign31190_e46888, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    }
};
        var_nja11 = assign31190_e46890;
        var_nja11_dn0 = assign31190_e46890_d_n0;
        var_nja11_dn1 = assign31190_e46890_d_n1;
        var_nja11_dn2 = assign31190_e46890_d_n2;
        var_nja11_dn3 = assign31190_e46890_d_n3;
        var_nja11_dn4 = assign31190_e46890_d_n4;
        var_nja11_dn5 = assign31190_e46890_d_n5;
        var_nja11_db0 = assign31190_e46890_d_b0;
        var_nja11_db1 = assign31190_e46890_d_b1;
        var_nja11_db2 = assign31190_e46890_d_b2;
        var_nja11_db3 = assign31190_e46890_d_b3;

        let (assign31200_e46906, assign31200_e46906_d_n0, assign31200_e46906_d_n1, assign31200_e46906_d_n2, assign31200_e46906_d_n3, assign31200_e46906_d_n4, assign31200_e46906_d_n5, assign31200_e46906_d_b0, assign31200_e46906_d_b1, assign31200_e46906_d_b2, assign31200_e46906_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31200_e46902: f64 = (var_nja11 - var_nfagat_i);
        let assign31200_e46904: f64 = (assign31200_e46902 - 0.01);
        (assign31200_e46904, var_nja11_dn0, var_nja11_dn1, var_nja11_dn2, var_nja11_dn3, var_nja11_dn4, var_nja11_dn5, var_nja11_db0, var_nja11_db1, var_nja11_db2, var_nja11_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31200_e46906;
        var_tmf1_dn0 = assign31200_e46906_d_n0;
        var_tmf1_dn1 = assign31200_e46906_d_n1;
        var_tmf1_dn2 = assign31200_e46906_d_n2;
        var_tmf1_dn3 = assign31200_e46906_d_n3;
        var_tmf1_dn4 = assign31200_e46906_d_n4;
        var_tmf1_dn5 = assign31200_e46906_d_n5;
        var_tmf1_db0 = assign31200_e46906_d_b0;
        var_tmf1_db1 = assign31200_e46906_d_b1;
        var_tmf1_db2 = assign31200_e46906_d_b2;
        var_tmf1_db3 = assign31200_e46906_d_b3;

        let (assign31210_e46922, assign31210_e46922_d_n0, assign31210_e46922_d_n1, assign31210_e46922_d_n2, assign31210_e46922_d_n3, assign31210_e46922_d_n4, assign31210_e46922_d_n5, assign31210_e46922_d_b0, assign31210_e46922_d_b1, assign31210_e46922_d_b2, assign31210_e46922_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31210_e46918: f64 = (4.0 * var_nfagat_i);
        let assign31210_e46920: f64 = (assign31210_e46918 * 0.01);
        (assign31210_e46920, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31210_e46922;
        var_tmf2_dn0 = assign31210_e46922_d_n0;
        var_tmf2_dn1 = assign31210_e46922_d_n1;
        var_tmf2_dn2 = assign31210_e46922_d_n2;
        var_tmf2_dn3 = assign31210_e46922_d_n3;
        var_tmf2_dn4 = assign31210_e46922_d_n4;
        var_tmf2_dn5 = assign31210_e46922_d_n5;
        var_tmf2_db0 = assign31210_e46922_d_b0;
        var_tmf2_db1 = assign31210_e46922_d_b1;
        var_tmf2_db2 = assign31210_e46922_d_b2;
        var_tmf2_db3 = assign31210_e46922_d_b3;

        let (assign31220_e46940, assign31220_e46940_d_n0, assign31220_e46940_d_n1, assign31220_e46940_d_n2, assign31220_e46940_d_n3, assign31220_e46940_d_n4, assign31220_e46940_d_n5, assign31220_e46940_d_b0, assign31220_e46940_d_b1, assign31220_e46940_d_b2, assign31220_e46940_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n1, assign31220_e46938_d_n2, assign31220_e46938_d_n3, assign31220_e46938_d_n4, assign31220_e46938_d_n5, assign31220_e46938_d_b0, assign31220_e46938_d_b1, assign31220_e46938_d_b2, assign31220_e46938_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31220_e46937: f64 = (-var_tmf2);
                (assign31220_e46937, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n1, assign31220_e46938_d_n2, assign31220_e46938_d_n3, assign31220_e46938_d_n4, assign31220_e46938_d_n5, assign31220_e46938_d_b0, assign31220_e46938_d_b1, assign31220_e46938_d_b2, assign31220_e46938_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31220_e46940;
        var_tmf2_dn0 = assign31220_e46940_d_n0;
        var_tmf2_dn1 = assign31220_e46940_d_n1;
        var_tmf2_dn2 = assign31220_e46940_d_n2;
        var_tmf2_dn3 = assign31220_e46940_d_n3;
        var_tmf2_dn4 = assign31220_e46940_d_n4;
        var_tmf2_dn5 = assign31220_e46940_d_n5;
        var_tmf2_db0 = assign31220_e46940_d_b0;
        var_tmf2_db1 = assign31220_e46940_d_b1;
        var_tmf2_db2 = assign31220_e46940_d_b2;
        var_tmf2_db3 = assign31220_e46940_d_b3;

        let (assign31230_e46957, assign31230_e46957_d_n0, assign31230_e46957_d_n1, assign31230_e46957_d_n2, assign31230_e46957_d_n3, assign31230_e46957_d_n4, assign31230_e46957_d_n5, assign31230_e46957_d_b0, assign31230_e46957_d_b1, assign31230_e46957_d_b2, assign31230_e46957_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31230_e46952: f64 = (var_tmf1 * var_tmf1);
        let assign31230_e46954: f64 = (assign31230_e46952 + var_tmf2);
        let assign31230_e46955: f64 = (assign31230_e46954).sqrt();
        (assign31230_e46955, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31230_e46955)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31230_e46955)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31230_e46955)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31230_e46955)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31230_e46955)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31230_e46957;
        var_tmf2_dn0 = assign31230_e46957_d_n0;
        var_tmf2_dn1 = assign31230_e46957_d_n1;
        var_tmf2_dn2 = assign31230_e46957_d_n2;
        var_tmf2_dn3 = assign31230_e46957_d_n3;
        var_tmf2_dn4 = assign31230_e46957_d_n4;
        var_tmf2_dn5 = assign31230_e46957_d_n5;
        var_tmf2_db0 = assign31230_e46957_d_b0;
        var_tmf2_db1 = assign31230_e46957_d_b1;
        var_tmf2_db2 = assign31230_e46957_d_b2;
        var_tmf2_db3 = assign31230_e46957_d_b3;

        let (assign31240_e46975, assign31240_e46975_d_n0, assign31240_e46975_d_n1, assign31240_e46975_d_n2, assign31240_e46975_d_n3, assign31240_e46975_d_n4, assign31240_e46975_d_n5, assign31240_e46975_d_b0, assign31240_e46975_d_b1, assign31240_e46975_d_b2, assign31240_e46975_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31240_e46971: f64 = (var_tmf1 / var_tmf2);
        let assign31240_e46972: f64 = (1.0 + assign31240_e46971);
        let assign31240_e46973: f64 = (0.5 * assign31240_e46972);
        (assign31240_e46973, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn1 * var_tmf2) - (var_tmf1 * var_tmf2_dn1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn3 * var_tmf2) - (var_tmf1 * var_tmf2_dn3)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db0 * var_tmf2) - (var_tmf1 * var_tmf2_db0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db1 * var_tmf2) - (var_tmf1 * var_tmf2_db1)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db2 * var_tmf2) - (var_tmf1 * var_tmf2_db2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_db3 * var_tmf2) - (var_tmf1 * var_tmf2_db3)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn1, var_dfn_sl_dn2, var_dfn_sl_dn3, var_dfn_sl_dn4, var_dfn_sl_dn5, var_dfn_sl_db0, var_dfn_sl_db1, var_dfn_sl_db2, var_dfn_sl_db3,)
    }
};
        var_dfn_sl = assign31240_e46975;
        var_dfn_sl_dn0 = assign31240_e46975_d_n0;
        var_dfn_sl_dn1 = assign31240_e46975_d_n1;
        var_dfn_sl_dn2 = assign31240_e46975_d_n2;
        var_dfn_sl_dn3 = assign31240_e46975_d_n3;
        var_dfn_sl_dn4 = assign31240_e46975_d_n4;
        var_dfn_sl_dn5 = assign31240_e46975_d_n5;
        var_dfn_sl_db0 = assign31240_e46975_d_b0;
        var_dfn_sl_db1 = assign31240_e46975_d_b1;
        var_dfn_sl_db2 = assign31240_e46975_d_b2;
        var_dfn_sl_db3 = assign31240_e46975_d_b3;

        let (assign31250_e46993, assign31250_e46993_d_n0, assign31250_e46993_d_n1, assign31250_e46993_d_n2, assign31250_e46993_d_n3, assign31250_e46993_d_n4, assign31250_e46993_d_n5, assign31250_e46993_d_b0, assign31250_e46993_d_b1, assign31250_e46993_d_b2, assign31250_e46993_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31250_e46989: f64 = (var_tmf1 + var_tmf2);
        let assign31250_e46990: f64 = (0.5 * assign31250_e46989);
        let assign31250_e46991: f64 = (var_nfagat_i + assign31250_e46990);
        (assign31250_e46991, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31250_e46993;
        var_nj1_dn0 = assign31250_e46993_d_n0;
        var_nj1_dn1 = assign31250_e46993_d_n1;
        var_nj1_dn2 = assign31250_e46993_d_n2;
        var_nj1_dn3 = assign31250_e46993_d_n3;
        var_nj1_dn4 = assign31250_e46993_d_n4;
        var_nj1_dn5 = assign31250_e46993_d_n5;
        var_nj1_db0 = assign31250_e46993_d_b0;
        var_nj1_db1 = assign31250_e46993_d_b1;
        var_nj1_db2 = assign31250_e46993_d_b2;
        var_nj1_db3 = assign31250_e46993_d_b3;

        let (assign31260_e47009, assign31260_e47009_d_n0, assign31260_e47009_d_n1, assign31260_e47009_d_n2, assign31260_e47009_d_n3, assign31260_e47009_d_n4, assign31260_e47009_d_n5, assign31260_e47009_d_b0, assign31260_e47009_d_b1, assign31260_e47009_d_b2, assign31260_e47009_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31260_e47005: f64 = (p.p85 - var_nj0);
        let assign31260_e47007: f64 = (assign31260_e47005 - 0.01);
        (assign31260_e47007, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31260_e47009;
        var_tmf1_dn0 = assign31260_e47009_d_n0;
        var_tmf1_dn1 = assign31260_e47009_d_n1;
        var_tmf1_dn2 = assign31260_e47009_d_n2;
        var_tmf1_dn3 = assign31260_e47009_d_n3;
        var_tmf1_dn4 = assign31260_e47009_d_n4;
        var_tmf1_dn5 = assign31260_e47009_d_n5;
        var_tmf1_db0 = assign31260_e47009_d_b0;
        var_tmf1_db1 = assign31260_e47009_d_b1;
        var_tmf1_db2 = assign31260_e47009_d_b2;
        var_tmf1_db3 = assign31260_e47009_d_b3;

        let (assign31270_e47025, assign31270_e47025_d_n0, assign31270_e47025_d_n1, assign31270_e47025_d_n2, assign31270_e47025_d_n3, assign31270_e47025_d_n4, assign31270_e47025_d_n5, assign31270_e47025_d_b0, assign31270_e47025_d_b1, assign31270_e47025_d_b2, assign31270_e47025_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31270_e47021: f64 = (4.0 * p.p85);
        let assign31270_e47023: f64 = (assign31270_e47021 * 0.01);
        (assign31270_e47023, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31270_e47025;
        var_tmf2_dn0 = assign31270_e47025_d_n0;
        var_tmf2_dn1 = assign31270_e47025_d_n1;
        var_tmf2_dn2 = assign31270_e47025_d_n2;
        var_tmf2_dn3 = assign31270_e47025_d_n3;
        var_tmf2_dn4 = assign31270_e47025_d_n4;
        var_tmf2_dn5 = assign31270_e47025_d_n5;
        var_tmf2_db0 = assign31270_e47025_d_b0;
        var_tmf2_db1 = assign31270_e47025_d_b1;
        var_tmf2_db2 = assign31270_e47025_d_b2;
        var_tmf2_db3 = assign31270_e47025_d_b3;

        let (assign31280_e47043, assign31280_e47043_d_n0, assign31280_e47043_d_n1, assign31280_e47043_d_n2, assign31280_e47043_d_n3, assign31280_e47043_d_n4, assign31280_e47043_d_n5, assign31280_e47043_d_b0, assign31280_e47043_d_b1, assign31280_e47043_d_b2, assign31280_e47043_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n1, assign31280_e47041_d_n2, assign31280_e47041_d_n3, assign31280_e47041_d_n4, assign31280_e47041_d_n5, assign31280_e47041_d_b0, assign31280_e47041_d_b1, assign31280_e47041_d_b2, assign31280_e47041_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31280_e47040: f64 = (-var_tmf2);
                (assign31280_e47040, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n1, assign31280_e47041_d_n2, assign31280_e47041_d_n3, assign31280_e47041_d_n4, assign31280_e47041_d_n5, assign31280_e47041_d_b0, assign31280_e47041_d_b1, assign31280_e47041_d_b2, assign31280_e47041_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31280_e47043;
        var_tmf2_dn0 = assign31280_e47043_d_n0;
        var_tmf2_dn1 = assign31280_e47043_d_n1;
        var_tmf2_dn2 = assign31280_e47043_d_n2;
        var_tmf2_dn3 = assign31280_e47043_d_n3;
        var_tmf2_dn4 = assign31280_e47043_d_n4;
        var_tmf2_dn5 = assign31280_e47043_d_n5;
        var_tmf2_db0 = assign31280_e47043_d_b0;
        var_tmf2_db1 = assign31280_e47043_d_b1;
        var_tmf2_db2 = assign31280_e47043_d_b2;
        var_tmf2_db3 = assign31280_e47043_d_b3;


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
        *var_guard498_slot = var_guard498;
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

    pub(super) fn stamp_transient_block_50(
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
        var_guard498: f64,
        var_nfagat_i: f64,
        var_njl: f64,
        var_phitdinv: f64,
        var_v_hk: f64,
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
        var_vha1: f64,
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
        var_guard558_slot: &mut f64,
        var_guard559_slot: &mut f64,
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
        var_nj_k0_slot: &mut f64,
        var_nj_k0_db0_slot: &mut f64,
        var_nj_k0_db1_slot: &mut f64,
        var_nj_k0_db2_slot: &mut f64,
        var_nj_k0_db3_slot: &mut f64,
        var_nj_k0_dn0_slot: &mut f64,
        var_nj_k0_dn1_slot: &mut f64,
        var_nj_k0_dn2_slot: &mut f64,
        var_nj_k0_dn3_slot: &mut f64,
        var_nj_k0_dn4_slot: &mut f64,
        var_nj_k0_dn5_slot: &mut f64,
        var_nj_k1_slot: &mut f64,
        var_nj_k1_db0_slot: &mut f64,
        var_nj_k1_db1_slot: &mut f64,
        var_nj_k1_db2_slot: &mut f64,
        var_nj_k1_db3_slot: &mut f64,
        var_nj_k1_dn0_slot: &mut f64,
        var_nj_k1_dn1_slot: &mut f64,
        var_nj_k1_dn2_slot: &mut f64,
        var_nj_k1_dn3_slot: &mut f64,
        var_nj_k1_dn4_slot: &mut f64,
        var_nj_k1_dn5_slot: &mut f64,
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
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
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
        let mut var_nj_k0: f64 = *var_nj_k0_slot;
        let mut var_nj_k0_db0: f64 = *var_nj_k0_db0_slot;
        let mut var_nj_k0_db1: f64 = *var_nj_k0_db1_slot;
        let mut var_nj_k0_db2: f64 = *var_nj_k0_db2_slot;
        let mut var_nj_k0_db3: f64 = *var_nj_k0_db3_slot;
        let mut var_nj_k0_dn0: f64 = *var_nj_k0_dn0_slot;
        let mut var_nj_k0_dn1: f64 = *var_nj_k0_dn1_slot;
        let mut var_nj_k0_dn2: f64 = *var_nj_k0_dn2_slot;
        let mut var_nj_k0_dn3: f64 = *var_nj_k0_dn3_slot;
        let mut var_nj_k0_dn4: f64 = *var_nj_k0_dn4_slot;
        let mut var_nj_k0_dn5: f64 = *var_nj_k0_dn5_slot;
        let mut var_nj_k1: f64 = *var_nj_k1_slot;
        let mut var_nj_k1_db0: f64 = *var_nj_k1_db0_slot;
        let mut var_nj_k1_db1: f64 = *var_nj_k1_db1_slot;
        let mut var_nj_k1_db2: f64 = *var_nj_k1_db2_slot;
        let mut var_nj_k1_db3: f64 = *var_nj_k1_db3_slot;
        let mut var_nj_k1_dn0: f64 = *var_nj_k1_dn0_slot;
        let mut var_nj_k1_dn1: f64 = *var_nj_k1_dn1_slot;
        let mut var_nj_k1_dn2: f64 = *var_nj_k1_dn2_slot;
        let mut var_nj_k1_dn3: f64 = *var_nj_k1_dn3_slot;
        let mut var_nj_k1_dn4: f64 = *var_nj_k1_dn4_slot;
        let mut var_nj_k1_dn5: f64 = *var_nj_k1_dn5_slot;
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

        let (assign31290_e47060, assign31290_e47060_d_n0, assign31290_e47060_d_n1, assign31290_e47060_d_n2, assign31290_e47060_d_n3, assign31290_e47060_d_n4, assign31290_e47060_d_n5, assign31290_e47060_d_b0, assign31290_e47060_d_b1, assign31290_e47060_d_b2, assign31290_e47060_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31290_e47055: f64 = (var_tmf1 * var_tmf1);
        let assign31290_e47057: f64 = (assign31290_e47055 + var_tmf2);
        let assign31290_e47058: f64 = (assign31290_e47057).sqrt();
        (assign31290_e47058, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31290_e47058)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31290_e47058)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31290_e47058)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31290_e47058)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31290_e47058)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31290_e47060;
        var_tmf2_dn0 = assign31290_e47060_d_n0;
        var_tmf2_dn1 = assign31290_e47060_d_n1;
        var_tmf2_dn2 = assign31290_e47060_d_n2;
        var_tmf2_dn3 = assign31290_e47060_d_n3;
        var_tmf2_dn4 = assign31290_e47060_d_n4;
        var_tmf2_dn5 = assign31290_e47060_d_n5;
        var_tmf2_db0 = assign31290_e47060_d_b0;
        var_tmf2_db1 = assign31290_e47060_d_b1;
        var_tmf2_db2 = assign31290_e47060_d_b2;
        var_tmf2_db3 = assign31290_e47060_d_b3;

        let (assign31300_e47078, assign31300_e47078_d_n0, assign31300_e47078_d_n1, assign31300_e47078_d_n2, assign31300_e47078_d_n3, assign31300_e47078_d_n4, assign31300_e47078_d_n5, assign31300_e47078_d_b0, assign31300_e47078_d_b1, assign31300_e47078_d_b2, assign31300_e47078_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31300_e47074: f64 = (var_tmf1 + var_tmf2);
        let assign31300_e47075: f64 = (0.5 * assign31300_e47074);
        let assign31300_e47076: f64 = (p.p85 - assign31300_e47075);
        (assign31300_e47076, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31300_e47078;
        var_nj0_dn0 = assign31300_e47078_d_n0;
        var_nj0_dn1 = assign31300_e47078_d_n1;
        var_nj0_dn2 = assign31300_e47078_d_n2;
        var_nj0_dn3 = assign31300_e47078_d_n3;
        var_nj0_dn4 = assign31300_e47078_d_n4;
        var_nj0_dn5 = assign31300_e47078_d_n5;
        var_nj0_db0 = assign31300_e47078_d_b0;
        var_nj0_db1 = assign31300_e47078_d_b1;
        var_nj0_db2 = assign31300_e47078_d_b2;
        var_nj0_db3 = assign31300_e47078_d_b3;

        let (assign31310_e47094, assign31310_e47094_d_n0, assign31310_e47094_d_n1, assign31310_e47094_d_n2, assign31310_e47094_d_n3, assign31310_e47094_d_n4, assign31310_e47094_d_n5, assign31310_e47094_d_b0, assign31310_e47094_d_b1, assign31310_e47094_d_b2, assign31310_e47094_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31310_e47090: f64 = (var_nj0 - var_nfagat_i);
        let assign31310_e47092: f64 = (assign31310_e47090 - 0.01);
        (assign31310_e47092, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign31310_e47094;
        var_tmf1_dn0 = assign31310_e47094_d_n0;
        var_tmf1_dn1 = assign31310_e47094_d_n1;
        var_tmf1_dn2 = assign31310_e47094_d_n2;
        var_tmf1_dn3 = assign31310_e47094_d_n3;
        var_tmf1_dn4 = assign31310_e47094_d_n4;
        var_tmf1_dn5 = assign31310_e47094_d_n5;
        var_tmf1_db0 = assign31310_e47094_d_b0;
        var_tmf1_db1 = assign31310_e47094_d_b1;
        var_tmf1_db2 = assign31310_e47094_d_b2;
        var_tmf1_db3 = assign31310_e47094_d_b3;

        let (assign31320_e47110, assign31320_e47110_d_n0, assign31320_e47110_d_n1, assign31320_e47110_d_n2, assign31320_e47110_d_n3, assign31320_e47110_d_n4, assign31320_e47110_d_n5, assign31320_e47110_d_b0, assign31320_e47110_d_b1, assign31320_e47110_d_b2, assign31320_e47110_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31320_e47106: f64 = (4.0 * var_nfagat_i);
        let assign31320_e47108: f64 = (assign31320_e47106 * 0.01);
        (assign31320_e47108, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31320_e47110;
        var_tmf2_dn0 = assign31320_e47110_d_n0;
        var_tmf2_dn1 = assign31320_e47110_d_n1;
        var_tmf2_dn2 = assign31320_e47110_d_n2;
        var_tmf2_dn3 = assign31320_e47110_d_n3;
        var_tmf2_dn4 = assign31320_e47110_d_n4;
        var_tmf2_dn5 = assign31320_e47110_d_n5;
        var_tmf2_db0 = assign31320_e47110_d_b0;
        var_tmf2_db1 = assign31320_e47110_d_b1;
        var_tmf2_db2 = assign31320_e47110_d_b2;
        var_tmf2_db3 = assign31320_e47110_d_b3;

        let (assign31330_e47128, assign31330_e47128_d_n0, assign31330_e47128_d_n1, assign31330_e47128_d_n2, assign31330_e47128_d_n3, assign31330_e47128_d_n4, assign31330_e47128_d_n5, assign31330_e47128_d_b0, assign31330_e47128_d_b1, assign31330_e47128_d_b2, assign31330_e47128_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n1, assign31330_e47126_d_n2, assign31330_e47126_d_n3, assign31330_e47126_d_n4, assign31330_e47126_d_n5, assign31330_e47126_d_b0, assign31330_e47126_d_b1, assign31330_e47126_d_b2, assign31330_e47126_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign31330_e47125: f64 = (-var_tmf2);
                (assign31330_e47125, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n1, assign31330_e47126_d_n2, assign31330_e47126_d_n3, assign31330_e47126_d_n4, assign31330_e47126_d_n5, assign31330_e47126_d_b0, assign31330_e47126_d_b1, assign31330_e47126_d_b2, assign31330_e47126_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31330_e47128;
        var_tmf2_dn0 = assign31330_e47128_d_n0;
        var_tmf2_dn1 = assign31330_e47128_d_n1;
        var_tmf2_dn2 = assign31330_e47128_d_n2;
        var_tmf2_dn3 = assign31330_e47128_d_n3;
        var_tmf2_dn4 = assign31330_e47128_d_n4;
        var_tmf2_dn5 = assign31330_e47128_d_n5;
        var_tmf2_db0 = assign31330_e47128_d_b0;
        var_tmf2_db1 = assign31330_e47128_d_b1;
        var_tmf2_db2 = assign31330_e47128_d_b2;
        var_tmf2_db3 = assign31330_e47128_d_b3;

        let (assign31340_e47145, assign31340_e47145_d_n0, assign31340_e47145_d_n1, assign31340_e47145_d_n2, assign31340_e47145_d_n3, assign31340_e47145_d_n4, assign31340_e47145_d_n5, assign31340_e47145_d_b0, assign31340_e47145_d_b1, assign31340_e47145_d_b2, assign31340_e47145_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31340_e47140: f64 = (var_tmf1 * var_tmf1);
        let assign31340_e47142: f64 = (assign31340_e47140 + var_tmf2);
        let assign31340_e47143: f64 = (assign31340_e47142).sqrt();
        (assign31340_e47143, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign31340_e47143)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign31340_e47143)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign31340_e47143)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign31340_e47143)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign31340_e47143)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign31340_e47145;
        var_tmf2_dn0 = assign31340_e47145_d_n0;
        var_tmf2_dn1 = assign31340_e47145_d_n1;
        var_tmf2_dn2 = assign31340_e47145_d_n2;
        var_tmf2_dn3 = assign31340_e47145_d_n3;
        var_tmf2_dn4 = assign31340_e47145_d_n4;
        var_tmf2_dn5 = assign31340_e47145_d_n5;
        var_tmf2_db0 = assign31340_e47145_d_b0;
        var_tmf2_db1 = assign31340_e47145_d_b1;
        var_tmf2_db2 = assign31340_e47145_d_b2;
        var_tmf2_db3 = assign31340_e47145_d_b3;

        let (assign31350_e47163, assign31350_e47163_d_n0, assign31350_e47163_d_n1, assign31350_e47163_d_n2, assign31350_e47163_d_n3, assign31350_e47163_d_n4, assign31350_e47163_d_n5, assign31350_e47163_d_b0, assign31350_e47163_d_b1, assign31350_e47163_d_b2, assign31350_e47163_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31350_e47159: f64 = (var_tmf1 + var_tmf2);
        let assign31350_e47160: f64 = (0.5 * assign31350_e47159);
        let assign31350_e47161: f64 = (var_nfagat_i + assign31350_e47160);
        (assign31350_e47161, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31350_e47163;
        var_nj0_dn0 = assign31350_e47163_d_n0;
        var_nj0_dn1 = assign31350_e47163_d_n1;
        var_nj0_dn2 = assign31350_e47163_d_n2;
        var_nj0_dn3 = assign31350_e47163_d_n3;
        var_nj0_dn4 = assign31350_e47163_d_n4;
        var_nj0_dn5 = assign31350_e47163_d_n5;
        var_nj0_db0 = assign31350_e47163_d_b0;
        var_nj0_db1 = assign31350_e47163_d_b1;
        var_nj0_db2 = assign31350_e47163_d_b2;
        var_nj0_db3 = assign31350_e47163_d_b3;

        let (assign31360_e47179, assign31360_e47179_d_n0, assign31360_e47179_d_n1, assign31360_e47179_d_n2, assign31360_e47179_d_n3, assign31360_e47179_d_n4, assign31360_e47179_d_n5, assign31360_e47179_d_b0, assign31360_e47179_d_b1, assign31360_e47179_d_b2, assign31360_e47179_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31360_e47175: f64 = (p.p86 * var_dfn_su);
        let assign31360_e47177: f64 = (assign31360_e47175 * var_dfn_sl);
        (assign31360_e47177, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn1) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn1)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn2)), (((p.p86 * var_dfn_su_dn3) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn3)), (((p.p86 * var_dfn_su_dn4) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn4)), (((p.p86 * var_dfn_su_dn5) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn5)), (((p.p86 * var_dfn_su_db0) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db0)), (((p.p86 * var_dfn_su_db1) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db1)), (((p.p86 * var_dfn_su_db2) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db2)), (((p.p86 * var_dfn_su_db3) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_db3)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31360_e47179;
        var_dnj1_dv_dn0 = assign31360_e47179_d_n0;
        var_dnj1_dv_dn1 = assign31360_e47179_d_n1;
        var_dnj1_dv_dn2 = assign31360_e47179_d_n2;
        var_dnj1_dv_dn3 = assign31360_e47179_d_n3;
        var_dnj1_dv_dn4 = assign31360_e47179_d_n4;
        var_dnj1_dv_dn5 = assign31360_e47179_d_n5;
        var_dnj1_dv_db0 = assign31360_e47179_d_b0;
        var_dnj1_dv_db1 = assign31360_e47179_d_b1;
        var_dnj1_dv_db2 = assign31360_e47179_d_b2;
        var_dnj1_dv_db3 = assign31360_e47179_d_b3;

        let (assign31370_e47192, assign31370_e47192_d_n0, assign31370_e47192_d_n1, assign31370_e47192_d_n2, assign31370_e47192_d_n3, assign31370_e47192_d_n4, assign31370_e47192_d_n5, assign31370_e47192_d_b0, assign31370_e47192_d_b1, assign31370_e47192_d_b2, assign31370_e47192_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign31370_e47192;
        var_nj0_dn0 = assign31370_e47192_d_n0;
        var_nj0_dn1 = assign31370_e47192_d_n1;
        var_nj0_dn2 = assign31370_e47192_d_n2;
        var_nj0_dn3 = assign31370_e47192_d_n3;
        var_nj0_dn4 = assign31370_e47192_d_n4;
        var_nj0_dn5 = assign31370_e47192_d_n5;
        var_nj0_db0 = assign31370_e47192_d_b0;
        var_nj0_db1 = assign31370_e47192_d_b1;
        var_nj0_db2 = assign31370_e47192_d_b2;
        var_nj0_db3 = assign31370_e47192_d_b3;

        let (assign31380_e47205, assign31380_e47205_d_n0, assign31380_e47205_d_n1, assign31380_e47205_d_n2, assign31380_e47205_d_n3, assign31380_e47205_d_n4, assign31380_e47205_d_n5, assign31380_e47205_d_b0, assign31380_e47205_d_b1, assign31380_e47205_d_b2, assign31380_e47205_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn1, var_nj1_dn2, var_nj1_dn3, var_nj1_dn4, var_nj1_dn5, var_nj1_db0, var_nj1_db1, var_nj1_db2, var_nj1_db3,)
    }
};
        var_nj1 = assign31380_e47205;
        var_nj1_dn0 = assign31380_e47205_d_n0;
        var_nj1_dn1 = assign31380_e47205_d_n1;
        var_nj1_dn2 = assign31380_e47205_d_n2;
        var_nj1_dn3 = assign31380_e47205_d_n3;
        var_nj1_dn4 = assign31380_e47205_d_n4;
        var_nj1_dn5 = assign31380_e47205_d_n5;
        var_nj1_db0 = assign31380_e47205_d_b0;
        var_nj1_db1 = assign31380_e47205_d_b1;
        var_nj1_db2 = assign31380_e47205_d_b2;
        var_nj1_db3 = assign31380_e47205_d_b3;

        let (assign31390_e47218, assign31390_e47218_d_n0, assign31390_e47218_d_n1, assign31390_e47218_d_n2, assign31390_e47218_d_n3, assign31390_e47218_d_n4, assign31390_e47218_d_n5, assign31390_e47218_d_b0, assign31390_e47218_d_b1, assign31390_e47218_d_b2, assign31390_e47218_d_b3,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn1, var_dnj1_dv_dn2, var_dnj1_dv_dn3, var_dnj1_dv_dn4, var_dnj1_dv_dn5, var_dnj1_dv_db0, var_dnj1_dv_db1, var_dnj1_dv_db2, var_dnj1_dv_db3,)
    }
};
        var_dnj1_dv = assign31390_e47218;
        var_dnj1_dv_dn0 = assign31390_e47218_d_n0;
        var_dnj1_dv_dn1 = assign31390_e47218_d_n1;
        var_dnj1_dv_dn2 = assign31390_e47218_d_n2;
        var_dnj1_dv_dn3 = assign31390_e47218_d_n3;
        var_dnj1_dv_dn4 = assign31390_e47218_d_n4;
        var_dnj1_dv_dn5 = assign31390_e47218_d_n5;
        var_dnj1_dv_db0 = assign31390_e47218_d_b0;
        var_dnj1_dv_db1 = assign31390_e47218_d_b1;
        var_dnj1_dv_db2 = assign31390_e47218_d_b2;
        var_dnj1_dv_db3 = assign31390_e47218_d_b3;

        let (assign31450_e47471, assign31450_e47471_d_n0, assign31450_e47471_d_n1, assign31450_e47471_d_n2, assign31450_e47471_d_n3, assign31450_e47471_d_n4, assign31450_e47471_d_n5, assign31450_e47471_d_b0, assign31450_e47471_d_b1, assign31450_e47471_d_b2, assign31450_e47471_d_b3,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31450_e47455: f64 = (var_vmax * var_dnj1_dv);
        let assign31450_e47456: f64 = (var_nj1 - assign31450_e47455);
        let assign31450_e47459: f64 = (var_nj1 * var_nj1);
        let assign31450_e47460: f64 = (assign31450_e47456 / assign31450_e47459);
        let assign31450_e47463: f64 = (var_vha1 * var_dnj1_dv);
        let assign31450_e47466: f64 = (var_nj0 * p.p85);
        let assign31450_e47467: f64 = (assign31450_e47463 / assign31450_e47466);
        let assign31450_e47468: f64 = (assign31450_e47460 + assign31450_e47467);
        let assign31450_e47469: f64 = (var_phitdinv * assign31450_e47468);
        (assign31450_e47469, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn0 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn1 - (var_vmax * var_dnj1_dv_dn1)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn1 * var_nj1) + (var_nj1 * var_nj1_dn1)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn1) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn1 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn2 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn3 - (var_vmax * var_dnj1_dv_dn3)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn3 * var_nj1) + (var_nj1 * var_nj1_dn3)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn3) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn3 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn4 - (var_vmax * var_dnj1_dv_dn4)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn4 * var_nj1) + (var_nj1 * var_nj1_dn4)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn4) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn4 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn5 - (var_vmax * var_dnj1_dv_dn5)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn5 * var_nj1) + (var_nj1 * var_nj1_dn5)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn5) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn5 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_db0 - (var_vmax * var_dnj1_dv_db0)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db0 * var_nj1) + (var_nj1 * var_nj1_db0)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_db0) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db0 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_db1 - (var_vmax * var_dnj1_dv_db1)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db1 * var_nj1) + (var_nj1 * var_nj1_db1)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_db1) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db1 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_db2 - (var_vmax * var_dnj1_dv_db2)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db2 * var_nj1) + (var_nj1 * var_nj1_db2)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_db2) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db2 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_db3 - (var_vmax * var_dnj1_dv_db3)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_db3 * var_nj1) + (var_nj1 * var_nj1_db3)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_db3) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_db3 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn1, var_dvmax_over_phitd_dv_dn2, var_dvmax_over_phitd_dv_dn3, var_dvmax_over_phitd_dv_dn4, var_dvmax_over_phitd_dv_dn5, var_dvmax_over_phitd_dv_db0, var_dvmax_over_phitd_dv_db1, var_dvmax_over_phitd_dv_db2, var_dvmax_over_phitd_dv_db3,)
    }
};
        var_dvmax_over_phitd_dv = assign31450_e47471;
        var_dvmax_over_phitd_dv_dn0 = assign31450_e47471_d_n0;
        var_dvmax_over_phitd_dv_dn1 = assign31450_e47471_d_n1;
        var_dvmax_over_phitd_dv_dn2 = assign31450_e47471_d_n2;
        var_dvmax_over_phitd_dv_dn3 = assign31450_e47471_d_n3;
        var_dvmax_over_phitd_dv_dn4 = assign31450_e47471_d_n4;
        var_dvmax_over_phitd_dv_dn5 = assign31450_e47471_d_n5;
        var_dvmax_over_phitd_dv_db0 = assign31450_e47471_d_b0;
        var_dvmax_over_phitd_dv_db1 = assign31450_e47471_d_b1;
        var_dvmax_over_phitd_dv_db2 = assign31450_e47471_d_b2;
        var_dvmax_over_phitd_dv_db3 = assign31450_e47471_d_b3;

        let (assign31470_e47498, assign31470_e47498_d_n0, assign31470_e47498_d_n1, assign31470_e47498_d_n2, assign31470_e47498_d_n3, assign31470_e47498_d_n4, assign31470_e47498_d_n5, assign31470_e47498_d_b0, assign31470_e47498_d_b1, assign31470_e47498_d_b2, assign31470_e47498_d_b3,) = {
    if ((var_guard471 == 0.0) && (var_guard479 != 0.0)) {
        let assign31470_e47496: f64 = (var_idmultbot - 1.0);
        (assign31470_e47496, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign31470_e47498;
        var_idmultbot_dn0 = assign31470_e47498_d_n0;
        var_idmultbot_dn1 = assign31470_e47498_d_n1;
        var_idmultbot_dn2 = assign31470_e47498_d_n2;
        var_idmultbot_dn3 = assign31470_e47498_d_n3;
        var_idmultbot_dn4 = assign31470_e47498_d_n4;
        var_idmultbot_dn5 = assign31470_e47498_d_n5;
        var_idmultbot_db0 = assign31470_e47498_d_b0;
        var_idmultbot_db1 = assign31470_e47498_d_b1;
        var_idmultbot_db2 = assign31470_e47498_d_b2;
        var_idmultbot_db3 = assign31470_e47498_d_b3;

        let (assign31580_e47681, assign31580_e47681_d_n0, assign31580_e47681_d_n1, assign31580_e47681_d_n2, assign31580_e47681_d_n3, assign31580_e47681_d_n4, assign31580_e47681_d_n5, assign31580_e47681_d_b0, assign31580_e47681_d_b1, assign31580_e47681_d_b2, assign31580_e47681_d_b3,) = {
    if ((var_guard471 == 0.0) && (var_guard479 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    }
};
        var_idmultbot = assign31580_e47681;
        var_idmultbot_dn0 = assign31580_e47681_d_n0;
        var_idmultbot_dn1 = assign31580_e47681_d_n1;
        var_idmultbot_dn2 = assign31580_e47681_d_n2;
        var_idmultbot_dn3 = assign31580_e47681_d_n3;
        var_idmultbot_dn4 = assign31580_e47681_d_n4;
        var_idmultbot_dn5 = assign31580_e47681_d_n5;
        var_idmultbot_db0 = assign31580_e47681_d_b0;
        var_idmultbot_db1 = assign31580_e47681_d_b1;
        var_idmultbot_db2 = assign31580_e47681_d_b2;
        var_idmultbot_db3 = assign31580_e47681_d_b3;

        let assign34170_e51465: f64 = if p.p84 > 0.0 { 1.0 } else { 0.0 };
        var_guard558 = assign34170_e51465;

        let assign34180_e51468: f64 = if var_njl < p.p85 { 1.0 } else { 0.0 };
        var_guard559 = assign34180_e51468;

        let (assign34190_e51480, assign34190_e51480_d_n0, assign34190_e51480_d_n1, assign34190_e51480_d_n2, assign34190_e51480_d_n3, assign34190_e51480_d_n4, assign34190_e51480_d_n5, assign34190_e51480_d_b0, assign34190_e51480_d_b1, assign34190_e51480_d_b2, assign34190_e51480_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34190_e51475: f64 = (var_vak - var_v_hk);
        let assign34190_e51476: f64 = (p.p86 * assign34190_e51475);
        let assign34190_e51478: f64 = (assign34190_e51476 + var_njl);
        (assign34190_e51478, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn1), (p.p86 * var_vak_dn2), (p.p86 * var_vak_dn3), (p.p86 * var_vak_dn4), (p.p86 * var_vak_dn5), (p.p86 * var_vak_db0), (p.p86 * var_vak_db1), (p.p86 * var_vak_db2), (p.p86 * var_vak_db3),)
    } else {
        (var_nj_k0, var_nj_k0_dn0, var_nj_k0_dn1, var_nj_k0_dn2, var_nj_k0_dn3, var_nj_k0_dn4, var_nj_k0_dn5, var_nj_k0_db0, var_nj_k0_db1, var_nj_k0_db2, var_nj_k0_db3,)
    }
};
        var_nj_k0 = assign34190_e51480;
        var_nj_k0_dn0 = assign34190_e51480_d_n0;
        var_nj_k0_dn1 = assign34190_e51480_d_n1;
        var_nj_k0_dn2 = assign34190_e51480_d_n2;
        var_nj_k0_dn3 = assign34190_e51480_d_n3;
        var_nj_k0_dn4 = assign34190_e51480_d_n4;
        var_nj_k0_dn5 = assign34190_e51480_d_n5;
        var_nj_k0_db0 = assign34190_e51480_d_b0;
        var_nj_k0_db1 = assign34190_e51480_d_b1;
        var_nj_k0_db2 = assign34190_e51480_d_b2;
        var_nj_k0_db3 = assign34190_e51480_d_b3;

        let (assign34200_e51490, assign34200_e51490_d_n0, assign34200_e51490_d_n1, assign34200_e51490_d_n2, assign34200_e51490_d_n3, assign34200_e51490_d_n4, assign34200_e51490_d_n5, assign34200_e51490_d_b0, assign34200_e51490_d_b1, assign34200_e51490_d_b2, assign34200_e51490_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34200_e51487: f64 = (p.p86 * var_v_hk);
        let assign34200_e51488: f64 = (var_njl - assign34200_e51487);
        (assign34200_e51488, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34200_e51490;
        var_nj0_dn0 = assign34200_e51490_d_n0;
        var_nj0_dn1 = assign34200_e51490_d_n1;
        var_nj0_dn2 = assign34200_e51490_d_n2;
        var_nj0_dn3 = assign34200_e51490_d_n3;
        var_nj0_dn4 = assign34200_e51490_d_n4;
        var_nj0_dn5 = assign34200_e51490_d_n5;
        var_nj0_db0 = assign34200_e51490_d_b0;
        var_nj0_db1 = assign34200_e51490_d_b1;
        var_nj0_db2 = assign34200_e51490_d_b2;
        var_nj0_db3 = assign34200_e51490_d_b3;

        let (assign34210_e51500, assign34210_e51500_d_n0, assign34210_e51500_d_n1, assign34210_e51500_d_n2, assign34210_e51500_d_n3, assign34210_e51500_d_n4, assign34210_e51500_d_n5, assign34210_e51500_d_b0, assign34210_e51500_d_b1, assign34210_e51500_d_b2, assign34210_e51500_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34210_e51496: f64 = (p.p85 - var_nj_k0);
        let assign34210_e51498: f64 = (assign34210_e51496 - 0.01);
        (assign34210_e51498, (-var_nj_k0_dn0), (-var_nj_k0_dn1), (-var_nj_k0_dn2), (-var_nj_k0_dn3), (-var_nj_k0_dn4), (-var_nj_k0_dn5), (-var_nj_k0_db0), (-var_nj_k0_db1), (-var_nj_k0_db2), (-var_nj_k0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34210_e51500;
        var_tmf1_dn0 = assign34210_e51500_d_n0;
        var_tmf1_dn1 = assign34210_e51500_d_n1;
        var_tmf1_dn2 = assign34210_e51500_d_n2;
        var_tmf1_dn3 = assign34210_e51500_d_n3;
        var_tmf1_dn4 = assign34210_e51500_d_n4;
        var_tmf1_dn5 = assign34210_e51500_d_n5;
        var_tmf1_db0 = assign34210_e51500_d_b0;
        var_tmf1_db1 = assign34210_e51500_d_b1;
        var_tmf1_db2 = assign34210_e51500_d_b2;
        var_tmf1_db3 = assign34210_e51500_d_b3;

        let (assign34220_e51510, assign34220_e51510_d_n0, assign34220_e51510_d_n1, assign34220_e51510_d_n2, assign34220_e51510_d_n3, assign34220_e51510_d_n4, assign34220_e51510_d_n5, assign34220_e51510_d_b0, assign34220_e51510_d_b1, assign34220_e51510_d_b2, assign34220_e51510_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34220_e51506: f64 = (4.0 * p.p85);
        let assign34220_e51508: f64 = (assign34220_e51506 * 0.01);
        (assign34220_e51508, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34220_e51510;
        var_tmf2_dn0 = assign34220_e51510_d_n0;
        var_tmf2_dn1 = assign34220_e51510_d_n1;
        var_tmf2_dn2 = assign34220_e51510_d_n2;
        var_tmf2_dn3 = assign34220_e51510_d_n3;
        var_tmf2_dn4 = assign34220_e51510_d_n4;
        var_tmf2_dn5 = assign34220_e51510_d_n5;
        var_tmf2_db0 = assign34220_e51510_d_b0;
        var_tmf2_db1 = assign34220_e51510_d_b1;
        var_tmf2_db2 = assign34220_e51510_d_b2;
        var_tmf2_db3 = assign34220_e51510_d_b3;

        let (assign34230_e51522, assign34230_e51522_d_n0, assign34230_e51522_d_n1, assign34230_e51522_d_n2, assign34230_e51522_d_n3, assign34230_e51522_d_n4, assign34230_e51522_d_n5, assign34230_e51522_d_b0, assign34230_e51522_d_b1, assign34230_e51522_d_b2, assign34230_e51522_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n1, assign34230_e51520_d_n2, assign34230_e51520_d_n3, assign34230_e51520_d_n4, assign34230_e51520_d_n5, assign34230_e51520_d_b0, assign34230_e51520_d_b1, assign34230_e51520_d_b2, assign34230_e51520_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34230_e51519: f64 = (-var_tmf2);
                (assign34230_e51519, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n1, assign34230_e51520_d_n2, assign34230_e51520_d_n3, assign34230_e51520_d_n4, assign34230_e51520_d_n5, assign34230_e51520_d_b0, assign34230_e51520_d_b1, assign34230_e51520_d_b2, assign34230_e51520_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34230_e51522;
        var_tmf2_dn0 = assign34230_e51522_d_n0;
        var_tmf2_dn1 = assign34230_e51522_d_n1;
        var_tmf2_dn2 = assign34230_e51522_d_n2;
        var_tmf2_dn3 = assign34230_e51522_d_n3;
        var_tmf2_dn4 = assign34230_e51522_d_n4;
        var_tmf2_dn5 = assign34230_e51522_d_n5;
        var_tmf2_db0 = assign34230_e51522_d_b0;
        var_tmf2_db1 = assign34230_e51522_d_b1;
        var_tmf2_db2 = assign34230_e51522_d_b2;
        var_tmf2_db3 = assign34230_e51522_d_b3;

        let (assign34240_e51533, assign34240_e51533_d_n0, assign34240_e51533_d_n1, assign34240_e51533_d_n2, assign34240_e51533_d_n3, assign34240_e51533_d_n4, assign34240_e51533_d_n5, assign34240_e51533_d_b0, assign34240_e51533_d_b1, assign34240_e51533_d_b2, assign34240_e51533_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34240_e51528: f64 = (var_tmf1 * var_tmf1);
        let assign34240_e51530: f64 = (assign34240_e51528 + var_tmf2);
        let assign34240_e51531: f64 = (assign34240_e51530).sqrt();
        (assign34240_e51531, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34240_e51531)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34240_e51531)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34240_e51531)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34240_e51531)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34240_e51531)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34240_e51533;
        var_tmf2_dn0 = assign34240_e51533_d_n0;
        var_tmf2_dn1 = assign34240_e51533_d_n1;
        var_tmf2_dn2 = assign34240_e51533_d_n2;
        var_tmf2_dn3 = assign34240_e51533_d_n3;
        var_tmf2_dn4 = assign34240_e51533_d_n4;
        var_tmf2_dn5 = assign34240_e51533_d_n5;
        var_tmf2_db0 = assign34240_e51533_d_b0;
        var_tmf2_db1 = assign34240_e51533_d_b1;
        var_tmf2_db2 = assign34240_e51533_d_b2;
        var_tmf2_db3 = assign34240_e51533_d_b3;

        let (assign34250_e51545, assign34250_e51545_d_n0, assign34250_e51545_d_n1, assign34250_e51545_d_n2, assign34250_e51545_d_n3, assign34250_e51545_d_n4, assign34250_e51545_d_n5, assign34250_e51545_d_b0, assign34250_e51545_d_b1, assign34250_e51545_d_b2, assign34250_e51545_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34250_e51541: f64 = (var_tmf1 + var_tmf2);
        let assign34250_e51542: f64 = (0.5 * assign34250_e51541);
        let assign34250_e51543: f64 = (p.p85 - assign34250_e51542);
        (assign34250_e51543, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj_k1, var_nj_k1_dn0, var_nj_k1_dn1, var_nj_k1_dn2, var_nj_k1_dn3, var_nj_k1_dn4, var_nj_k1_dn5, var_nj_k1_db0, var_nj_k1_db1, var_nj_k1_db2, var_nj_k1_db3,)
    }
};
        var_nj_k1 = assign34250_e51545;
        var_nj_k1_dn0 = assign34250_e51545_d_n0;
        var_nj_k1_dn1 = assign34250_e51545_d_n1;
        var_nj_k1_dn2 = assign34250_e51545_d_n2;
        var_nj_k1_dn3 = assign34250_e51545_d_n3;
        var_nj_k1_dn4 = assign34250_e51545_d_n4;
        var_nj_k1_dn5 = assign34250_e51545_d_n5;
        var_nj_k1_db0 = assign34250_e51545_d_b0;
        var_nj_k1_db1 = assign34250_e51545_d_b1;
        var_nj_k1_db2 = assign34250_e51545_d_b2;
        var_nj_k1_db3 = assign34250_e51545_d_b3;

        let (assign34260_e51555, assign34260_e51555_d_n0, assign34260_e51555_d_n1, assign34260_e51555_d_n2, assign34260_e51555_d_n3, assign34260_e51555_d_n4, assign34260_e51555_d_n5, assign34260_e51555_d_b0, assign34260_e51555_d_b1, assign34260_e51555_d_b2, assign34260_e51555_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34260_e51551: f64 = (var_nj_k1 - var_njl);
        let assign34260_e51553: f64 = (assign34260_e51551 - 0.01);
        (assign34260_e51553, var_nj_k1_dn0, var_nj_k1_dn1, var_nj_k1_dn2, var_nj_k1_dn3, var_nj_k1_dn4, var_nj_k1_dn5, var_nj_k1_db0, var_nj_k1_db1, var_nj_k1_db2, var_nj_k1_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34260_e51555;
        var_tmf1_dn0 = assign34260_e51555_d_n0;
        var_tmf1_dn1 = assign34260_e51555_d_n1;
        var_tmf1_dn2 = assign34260_e51555_d_n2;
        var_tmf1_dn3 = assign34260_e51555_d_n3;
        var_tmf1_dn4 = assign34260_e51555_d_n4;
        var_tmf1_dn5 = assign34260_e51555_d_n5;
        var_tmf1_db0 = assign34260_e51555_d_b0;
        var_tmf1_db1 = assign34260_e51555_d_b1;
        var_tmf1_db2 = assign34260_e51555_d_b2;
        var_tmf1_db3 = assign34260_e51555_d_b3;

        let (assign34270_e51565, assign34270_e51565_d_n0, assign34270_e51565_d_n1, assign34270_e51565_d_n2, assign34270_e51565_d_n3, assign34270_e51565_d_n4, assign34270_e51565_d_n5, assign34270_e51565_d_b0, assign34270_e51565_d_b1, assign34270_e51565_d_b2, assign34270_e51565_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34270_e51561: f64 = (4.0 * var_njl);
        let assign34270_e51563: f64 = (assign34270_e51561 * 0.01);
        (assign34270_e51563, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34270_e51565;
        var_tmf2_dn0 = assign34270_e51565_d_n0;
        var_tmf2_dn1 = assign34270_e51565_d_n1;
        var_tmf2_dn2 = assign34270_e51565_d_n2;
        var_tmf2_dn3 = assign34270_e51565_d_n3;
        var_tmf2_dn4 = assign34270_e51565_d_n4;
        var_tmf2_dn5 = assign34270_e51565_d_n5;
        var_tmf2_db0 = assign34270_e51565_d_b0;
        var_tmf2_db1 = assign34270_e51565_d_b1;
        var_tmf2_db2 = assign34270_e51565_d_b2;
        var_tmf2_db3 = assign34270_e51565_d_b3;


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
        *var_guard558_slot = var_guard558;
        *var_guard559_slot = var_guard559;
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
        *var_nj_k0_slot = var_nj_k0;
        *var_nj_k0_db0_slot = var_nj_k0_db0;
        *var_nj_k0_db1_slot = var_nj_k0_db1;
        *var_nj_k0_db2_slot = var_nj_k0_db2;
        *var_nj_k0_db3_slot = var_nj_k0_db3;
        *var_nj_k0_dn0_slot = var_nj_k0_dn0;
        *var_nj_k0_dn1_slot = var_nj_k0_dn1;
        *var_nj_k0_dn2_slot = var_nj_k0_dn2;
        *var_nj_k0_dn3_slot = var_nj_k0_dn3;
        *var_nj_k0_dn4_slot = var_nj_k0_dn4;
        *var_nj_k0_dn5_slot = var_nj_k0_dn5;
        *var_nj_k1_slot = var_nj_k1;
        *var_nj_k1_db0_slot = var_nj_k1_db0;
        *var_nj_k1_db1_slot = var_nj_k1_db1;
        *var_nj_k1_db2_slot = var_nj_k1_db2;
        *var_nj_k1_db3_slot = var_nj_k1_db3;
        *var_nj_k1_dn0_slot = var_nj_k1_dn0;
        *var_nj_k1_dn1_slot = var_nj_k1_dn1;
        *var_nj_k1_dn2_slot = var_nj_k1_dn2;
        *var_nj_k1_dn3_slot = var_nj_k1_dn3;
        *var_nj_k1_dn4_slot = var_nj_k1_dn4;
        *var_nj_k1_dn5_slot = var_nj_k1_dn5;
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

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        var_guard558: f64,
        var_guard559: f64,
        var_idmultbot: f64,
        var_idmultbot_db0: f64,
        var_idmultbot_db1: f64,
        var_idmultbot_db2: f64,
        var_idmultbot_db3: f64,
        var_idmultbot_dn0: f64,
        var_idmultbot_dn1: f64,
        var_idmultbot_dn2: f64,
        var_idmultbot_dn3: f64,
        var_idmultbot_dn4: f64,
        var_idmultbot_dn5: f64,
        var_njl: f64,
        var_phitdinv: f64,
        var_v_ha: f64,
        var_v_hk: f64,
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
        var_exp_a_slot: &mut f64,
        var_exp_a_db0_slot: &mut f64,
        var_exp_a_db1_slot: &mut f64,
        var_exp_a_db2_slot: &mut f64,
        var_exp_a_db3_slot: &mut f64,
        var_exp_a_dn0_slot: &mut f64,
        var_exp_a_dn1_slot: &mut f64,
        var_exp_a_dn2_slot: &mut f64,
        var_exp_a_dn3_slot: &mut f64,
        var_exp_a_dn4_slot: &mut f64,
        var_exp_a_dn5_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k_db0_slot: &mut f64,
        var_exp_k_db1_slot: &mut f64,
        var_exp_k_db2_slot: &mut f64,
        var_exp_k_db3_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn1_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_exp_k_dn3_slot: &mut f64,
        var_exp_k_dn4_slot: &mut f64,
        var_exp_k_dn5_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
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
        var_nj_k_slot: &mut f64,
        var_nj_k_db0_slot: &mut f64,
        var_nj_k_db1_slot: &mut f64,
        var_nj_k_db2_slot: &mut f64,
        var_nj_k_db3_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn1_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_nj_k_dn3_slot: &mut f64,
        var_nj_k_dn4_slot: &mut f64,
        var_nj_k_dn5_slot: &mut f64,
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
        let mut var_exp_a: f64 = *var_exp_a_slot;
        let mut var_exp_a_db0: f64 = *var_exp_a_db0_slot;
        let mut var_exp_a_db1: f64 = *var_exp_a_db1_slot;
        let mut var_exp_a_db2: f64 = *var_exp_a_db2_slot;
        let mut var_exp_a_db3: f64 = *var_exp_a_db3_slot;
        let mut var_exp_a_dn0: f64 = *var_exp_a_dn0_slot;
        let mut var_exp_a_dn1: f64 = *var_exp_a_dn1_slot;
        let mut var_exp_a_dn2: f64 = *var_exp_a_dn2_slot;
        let mut var_exp_a_dn3: f64 = *var_exp_a_dn3_slot;
        let mut var_exp_a_dn4: f64 = *var_exp_a_dn4_slot;
        let mut var_exp_a_dn5: f64 = *var_exp_a_dn5_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k_db0: f64 = *var_exp_k_db0_slot;
        let mut var_exp_k_db1: f64 = *var_exp_k_db1_slot;
        let mut var_exp_k_db2: f64 = *var_exp_k_db2_slot;
        let mut var_exp_k_db3: f64 = *var_exp_k_db3_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn1: f64 = *var_exp_k_dn1_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_exp_k_dn3: f64 = *var_exp_k_dn3_slot;
        let mut var_exp_k_dn4: f64 = *var_exp_k_dn4_slot;
        let mut var_exp_k_dn5: f64 = *var_exp_k_dn5_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
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
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k_db0: f64 = *var_nj_k_db0_slot;
        let mut var_nj_k_db1: f64 = *var_nj_k_db1_slot;
        let mut var_nj_k_db2: f64 = *var_nj_k_db2_slot;
        let mut var_nj_k_db3: f64 = *var_nj_k_db3_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn1: f64 = *var_nj_k_dn1_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_nj_k_dn3: f64 = *var_nj_k_dn3_slot;
        let mut var_nj_k_dn4: f64 = *var_nj_k_dn4_slot;
        let mut var_nj_k_dn5: f64 = *var_nj_k_dn5_slot;
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

        let (assign34280_e51577, assign34280_e51577_d_n0, assign34280_e51577_d_n1, assign34280_e51577_d_n2, assign34280_e51577_d_n3, assign34280_e51577_d_n4, assign34280_e51577_d_n5, assign34280_e51577_d_b0, assign34280_e51577_d_b1, assign34280_e51577_d_b2, assign34280_e51577_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n1, assign34280_e51575_d_n2, assign34280_e51575_d_n3, assign34280_e51575_d_n4, assign34280_e51575_d_n5, assign34280_e51575_d_b0, assign34280_e51575_d_b1, assign34280_e51575_d_b2, assign34280_e51575_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34280_e51574: f64 = (-var_tmf2);
                (assign34280_e51574, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n1, assign34280_e51575_d_n2, assign34280_e51575_d_n3, assign34280_e51575_d_n4, assign34280_e51575_d_n5, assign34280_e51575_d_b0, assign34280_e51575_d_b1, assign34280_e51575_d_b2, assign34280_e51575_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34280_e51577;
        var_tmf2_dn0 = assign34280_e51577_d_n0;
        var_tmf2_dn1 = assign34280_e51577_d_n1;
        var_tmf2_dn2 = assign34280_e51577_d_n2;
        var_tmf2_dn3 = assign34280_e51577_d_n3;
        var_tmf2_dn4 = assign34280_e51577_d_n4;
        var_tmf2_dn5 = assign34280_e51577_d_n5;
        var_tmf2_db0 = assign34280_e51577_d_b0;
        var_tmf2_db1 = assign34280_e51577_d_b1;
        var_tmf2_db2 = assign34280_e51577_d_b2;
        var_tmf2_db3 = assign34280_e51577_d_b3;

        let (assign34290_e51588, assign34290_e51588_d_n0, assign34290_e51588_d_n1, assign34290_e51588_d_n2, assign34290_e51588_d_n3, assign34290_e51588_d_n4, assign34290_e51588_d_n5, assign34290_e51588_d_b0, assign34290_e51588_d_b1, assign34290_e51588_d_b2, assign34290_e51588_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34290_e51583: f64 = (var_tmf1 * var_tmf1);
        let assign34290_e51585: f64 = (assign34290_e51583 + var_tmf2);
        let assign34290_e51586: f64 = (assign34290_e51585).sqrt();
        (assign34290_e51586, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34290_e51586)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34290_e51586)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34290_e51586)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34290_e51586)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34290_e51586)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34290_e51588;
        var_tmf2_dn0 = assign34290_e51588_d_n0;
        var_tmf2_dn1 = assign34290_e51588_d_n1;
        var_tmf2_dn2 = assign34290_e51588_d_n2;
        var_tmf2_dn3 = assign34290_e51588_d_n3;
        var_tmf2_dn4 = assign34290_e51588_d_n4;
        var_tmf2_dn5 = assign34290_e51588_d_n5;
        var_tmf2_db0 = assign34290_e51588_d_b0;
        var_tmf2_db1 = assign34290_e51588_d_b1;
        var_tmf2_db2 = assign34290_e51588_d_b2;
        var_tmf2_db3 = assign34290_e51588_d_b3;

        let (assign34300_e51600, assign34300_e51600_d_n0, assign34300_e51600_d_n1, assign34300_e51600_d_n2, assign34300_e51600_d_n3, assign34300_e51600_d_n4, assign34300_e51600_d_n5, assign34300_e51600_d_b0, assign34300_e51600_d_b1, assign34300_e51600_d_b2, assign34300_e51600_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34300_e51596: f64 = (var_tmf1 + var_tmf2);
        let assign34300_e51597: f64 = (0.5 * assign34300_e51596);
        let assign34300_e51598: f64 = (var_njl + assign34300_e51597);
        (assign34300_e51598, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn1, var_nj_k_dn2, var_nj_k_dn3, var_nj_k_dn4, var_nj_k_dn5, var_nj_k_db0, var_nj_k_db1, var_nj_k_db2, var_nj_k_db3,)
    }
};
        var_nj_k = assign34300_e51600;
        var_nj_k_dn0 = assign34300_e51600_d_n0;
        var_nj_k_dn1 = assign34300_e51600_d_n1;
        var_nj_k_dn2 = assign34300_e51600_d_n2;
        var_nj_k_dn3 = assign34300_e51600_d_n3;
        var_nj_k_dn4 = assign34300_e51600_d_n4;
        var_nj_k_dn5 = assign34300_e51600_d_n5;
        var_nj_k_db0 = assign34300_e51600_d_b0;
        var_nj_k_db1 = assign34300_e51600_d_b1;
        var_nj_k_db2 = assign34300_e51600_d_b2;
        var_nj_k_db3 = assign34300_e51600_d_b3;

        let (assign34310_e51610, assign34310_e51610_d_n0, assign34310_e51610_d_n1, assign34310_e51610_d_n2, assign34310_e51610_d_n3, assign34310_e51610_d_n4, assign34310_e51610_d_n5, assign34310_e51610_d_b0, assign34310_e51610_d_b1, assign34310_e51610_d_b2, assign34310_e51610_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34310_e51606: f64 = (p.p85 - var_nj0);
        let assign34310_e51608: f64 = (assign34310_e51606 - 0.01);
        (assign34310_e51608, (-var_nj0_dn0), (-var_nj0_dn1), (-var_nj0_dn2), (-var_nj0_dn3), (-var_nj0_dn4), (-var_nj0_dn5), (-var_nj0_db0), (-var_nj0_db1), (-var_nj0_db2), (-var_nj0_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34310_e51610;
        var_tmf1_dn0 = assign34310_e51610_d_n0;
        var_tmf1_dn1 = assign34310_e51610_d_n1;
        var_tmf1_dn2 = assign34310_e51610_d_n2;
        var_tmf1_dn3 = assign34310_e51610_d_n3;
        var_tmf1_dn4 = assign34310_e51610_d_n4;
        var_tmf1_dn5 = assign34310_e51610_d_n5;
        var_tmf1_db0 = assign34310_e51610_d_b0;
        var_tmf1_db1 = assign34310_e51610_d_b1;
        var_tmf1_db2 = assign34310_e51610_d_b2;
        var_tmf1_db3 = assign34310_e51610_d_b3;

        let (assign34320_e51620, assign34320_e51620_d_n0, assign34320_e51620_d_n1, assign34320_e51620_d_n2, assign34320_e51620_d_n3, assign34320_e51620_d_n4, assign34320_e51620_d_n5, assign34320_e51620_d_b0, assign34320_e51620_d_b1, assign34320_e51620_d_b2, assign34320_e51620_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34320_e51616: f64 = (4.0 * p.p85);
        let assign34320_e51618: f64 = (assign34320_e51616 * 0.01);
        (assign34320_e51618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34320_e51620;
        var_tmf2_dn0 = assign34320_e51620_d_n0;
        var_tmf2_dn1 = assign34320_e51620_d_n1;
        var_tmf2_dn2 = assign34320_e51620_d_n2;
        var_tmf2_dn3 = assign34320_e51620_d_n3;
        var_tmf2_dn4 = assign34320_e51620_d_n4;
        var_tmf2_dn5 = assign34320_e51620_d_n5;
        var_tmf2_db0 = assign34320_e51620_d_b0;
        var_tmf2_db1 = assign34320_e51620_d_b1;
        var_tmf2_db2 = assign34320_e51620_d_b2;
        var_tmf2_db3 = assign34320_e51620_d_b3;

        let (assign34330_e51632, assign34330_e51632_d_n0, assign34330_e51632_d_n1, assign34330_e51632_d_n2, assign34330_e51632_d_n3, assign34330_e51632_d_n4, assign34330_e51632_d_n5, assign34330_e51632_d_b0, assign34330_e51632_d_b1, assign34330_e51632_d_b2, assign34330_e51632_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n1, assign34330_e51630_d_n2, assign34330_e51630_d_n3, assign34330_e51630_d_n4, assign34330_e51630_d_n5, assign34330_e51630_d_b0, assign34330_e51630_d_b1, assign34330_e51630_d_b2, assign34330_e51630_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34330_e51629: f64 = (-var_tmf2);
                (assign34330_e51629, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n1, assign34330_e51630_d_n2, assign34330_e51630_d_n3, assign34330_e51630_d_n4, assign34330_e51630_d_n5, assign34330_e51630_d_b0, assign34330_e51630_d_b1, assign34330_e51630_d_b2, assign34330_e51630_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34330_e51632;
        var_tmf2_dn0 = assign34330_e51632_d_n0;
        var_tmf2_dn1 = assign34330_e51632_d_n1;
        var_tmf2_dn2 = assign34330_e51632_d_n2;
        var_tmf2_dn3 = assign34330_e51632_d_n3;
        var_tmf2_dn4 = assign34330_e51632_d_n4;
        var_tmf2_dn5 = assign34330_e51632_d_n5;
        var_tmf2_db0 = assign34330_e51632_d_b0;
        var_tmf2_db1 = assign34330_e51632_d_b1;
        var_tmf2_db2 = assign34330_e51632_d_b2;
        var_tmf2_db3 = assign34330_e51632_d_b3;

        let (assign34340_e51643, assign34340_e51643_d_n0, assign34340_e51643_d_n1, assign34340_e51643_d_n2, assign34340_e51643_d_n3, assign34340_e51643_d_n4, assign34340_e51643_d_n5, assign34340_e51643_d_b0, assign34340_e51643_d_b1, assign34340_e51643_d_b2, assign34340_e51643_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34340_e51638: f64 = (var_tmf1 * var_tmf1);
        let assign34340_e51640: f64 = (assign34340_e51638 + var_tmf2);
        let assign34340_e51641: f64 = (assign34340_e51640).sqrt();
        (assign34340_e51641, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34340_e51641)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34340_e51641)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34340_e51641)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34340_e51641)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34340_e51641)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34340_e51643;
        var_tmf2_dn0 = assign34340_e51643_d_n0;
        var_tmf2_dn1 = assign34340_e51643_d_n1;
        var_tmf2_dn2 = assign34340_e51643_d_n2;
        var_tmf2_dn3 = assign34340_e51643_d_n3;
        var_tmf2_dn4 = assign34340_e51643_d_n4;
        var_tmf2_dn5 = assign34340_e51643_d_n5;
        var_tmf2_db0 = assign34340_e51643_d_b0;
        var_tmf2_db1 = assign34340_e51643_d_b1;
        var_tmf2_db2 = assign34340_e51643_d_b2;
        var_tmf2_db3 = assign34340_e51643_d_b3;

        let (assign34350_e51655, assign34350_e51655_d_n0, assign34350_e51655_d_n1, assign34350_e51655_d_n2, assign34350_e51655_d_n3, assign34350_e51655_d_n4, assign34350_e51655_d_n5, assign34350_e51655_d_b0, assign34350_e51655_d_b1, assign34350_e51655_d_b2, assign34350_e51655_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34350_e51651: f64 = (var_tmf1 + var_tmf2);
        let assign34350_e51652: f64 = (0.5 * assign34350_e51651);
        let assign34350_e51653: f64 = (p.p85 - assign34350_e51652);
        (assign34350_e51653, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34350_e51655;
        var_nj0_dn0 = assign34350_e51655_d_n0;
        var_nj0_dn1 = assign34350_e51655_d_n1;
        var_nj0_dn2 = assign34350_e51655_d_n2;
        var_nj0_dn3 = assign34350_e51655_d_n3;
        var_nj0_dn4 = assign34350_e51655_d_n4;
        var_nj0_dn5 = assign34350_e51655_d_n5;
        var_nj0_db0 = assign34350_e51655_d_b0;
        var_nj0_db1 = assign34350_e51655_d_b1;
        var_nj0_db2 = assign34350_e51655_d_b2;
        var_nj0_db3 = assign34350_e51655_d_b3;

        let (assign34360_e51665, assign34360_e51665_d_n0, assign34360_e51665_d_n1, assign34360_e51665_d_n2, assign34360_e51665_d_n3, assign34360_e51665_d_n4, assign34360_e51665_d_n5, assign34360_e51665_d_b0, assign34360_e51665_d_b1, assign34360_e51665_d_b2, assign34360_e51665_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34360_e51661: f64 = (var_nj0 - var_njl);
        let assign34360_e51663: f64 = (assign34360_e51661 - 0.01);
        (assign34360_e51663, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34360_e51665;
        var_tmf1_dn0 = assign34360_e51665_d_n0;
        var_tmf1_dn1 = assign34360_e51665_d_n1;
        var_tmf1_dn2 = assign34360_e51665_d_n2;
        var_tmf1_dn3 = assign34360_e51665_d_n3;
        var_tmf1_dn4 = assign34360_e51665_d_n4;
        var_tmf1_dn5 = assign34360_e51665_d_n5;
        var_tmf1_db0 = assign34360_e51665_d_b0;
        var_tmf1_db1 = assign34360_e51665_d_b1;
        var_tmf1_db2 = assign34360_e51665_d_b2;
        var_tmf1_db3 = assign34360_e51665_d_b3;

        let (assign34370_e51675, assign34370_e51675_d_n0, assign34370_e51675_d_n1, assign34370_e51675_d_n2, assign34370_e51675_d_n3, assign34370_e51675_d_n4, assign34370_e51675_d_n5, assign34370_e51675_d_b0, assign34370_e51675_d_b1, assign34370_e51675_d_b2, assign34370_e51675_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34370_e51671: f64 = (4.0 * var_njl);
        let assign34370_e51673: f64 = (assign34370_e51671 * 0.01);
        (assign34370_e51673, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34370_e51675;
        var_tmf2_dn0 = assign34370_e51675_d_n0;
        var_tmf2_dn1 = assign34370_e51675_d_n1;
        var_tmf2_dn2 = assign34370_e51675_d_n2;
        var_tmf2_dn3 = assign34370_e51675_d_n3;
        var_tmf2_dn4 = assign34370_e51675_d_n4;
        var_tmf2_dn5 = assign34370_e51675_d_n5;
        var_tmf2_db0 = assign34370_e51675_d_b0;
        var_tmf2_db1 = assign34370_e51675_d_b1;
        var_tmf2_db2 = assign34370_e51675_d_b2;
        var_tmf2_db3 = assign34370_e51675_d_b3;

        let (assign34380_e51687, assign34380_e51687_d_n0, assign34380_e51687_d_n1, assign34380_e51687_d_n2, assign34380_e51687_d_n3, assign34380_e51687_d_n4, assign34380_e51687_d_n5, assign34380_e51687_d_b0, assign34380_e51687_d_b1, assign34380_e51687_d_b2, assign34380_e51687_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n1, assign34380_e51685_d_n2, assign34380_e51685_d_n3, assign34380_e51685_d_n4, assign34380_e51685_d_n5, assign34380_e51685_d_b0, assign34380_e51685_d_b1, assign34380_e51685_d_b2, assign34380_e51685_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34380_e51684: f64 = (-var_tmf2);
                (assign34380_e51684, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n1, assign34380_e51685_d_n2, assign34380_e51685_d_n3, assign34380_e51685_d_n4, assign34380_e51685_d_n5, assign34380_e51685_d_b0, assign34380_e51685_d_b1, assign34380_e51685_d_b2, assign34380_e51685_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34380_e51687;
        var_tmf2_dn0 = assign34380_e51687_d_n0;
        var_tmf2_dn1 = assign34380_e51687_d_n1;
        var_tmf2_dn2 = assign34380_e51687_d_n2;
        var_tmf2_dn3 = assign34380_e51687_d_n3;
        var_tmf2_dn4 = assign34380_e51687_d_n4;
        var_tmf2_dn5 = assign34380_e51687_d_n5;
        var_tmf2_db0 = assign34380_e51687_d_b0;
        var_tmf2_db1 = assign34380_e51687_d_b1;
        var_tmf2_db2 = assign34380_e51687_d_b2;
        var_tmf2_db3 = assign34380_e51687_d_b3;

        let (assign34390_e51698, assign34390_e51698_d_n0, assign34390_e51698_d_n1, assign34390_e51698_d_n2, assign34390_e51698_d_n3, assign34390_e51698_d_n4, assign34390_e51698_d_n5, assign34390_e51698_d_b0, assign34390_e51698_d_b1, assign34390_e51698_d_b2, assign34390_e51698_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34390_e51693: f64 = (var_tmf1 * var_tmf1);
        let assign34390_e51695: f64 = (assign34390_e51693 + var_tmf2);
        let assign34390_e51696: f64 = (assign34390_e51695).sqrt();
        (assign34390_e51696, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34390_e51696)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34390_e51696)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34390_e51696)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34390_e51696)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34390_e51696)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34390_e51698;
        var_tmf2_dn0 = assign34390_e51698_d_n0;
        var_tmf2_dn1 = assign34390_e51698_d_n1;
        var_tmf2_dn2 = assign34390_e51698_d_n2;
        var_tmf2_dn3 = assign34390_e51698_d_n3;
        var_tmf2_dn4 = assign34390_e51698_d_n4;
        var_tmf2_dn5 = assign34390_e51698_d_n5;
        var_tmf2_db0 = assign34390_e51698_d_b0;
        var_tmf2_db1 = assign34390_e51698_d_b1;
        var_tmf2_db2 = assign34390_e51698_d_b2;
        var_tmf2_db3 = assign34390_e51698_d_b3;

        let (assign34400_e51710, assign34400_e51710_d_n0, assign34400_e51710_d_n1, assign34400_e51710_d_n2, assign34400_e51710_d_n3, assign34400_e51710_d_n4, assign34400_e51710_d_n5, assign34400_e51710_d_b0, assign34400_e51710_d_b1, assign34400_e51710_d_b2, assign34400_e51710_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34400_e51706: f64 = (var_tmf1 + var_tmf2);
        let assign34400_e51707: f64 = (0.5 * assign34400_e51706);
        let assign34400_e51708: f64 = (var_njl + assign34400_e51707);
        (assign34400_e51708, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn1 + var_tmf2_dn1)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn3 + var_tmf2_dn3)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_db0 + var_tmf2_db0)), (0.5 * (var_tmf1_db1 + var_tmf2_db1)), (0.5 * (var_tmf1_db2 + var_tmf2_db2)), (0.5 * (var_tmf1_db3 + var_tmf2_db3)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34400_e51710;
        var_nj0_dn0 = assign34400_e51710_d_n0;
        var_nj0_dn1 = assign34400_e51710_d_n1;
        var_nj0_dn2 = assign34400_e51710_d_n2;
        var_nj0_dn3 = assign34400_e51710_d_n3;
        var_nj0_dn4 = assign34400_e51710_d_n4;
        var_nj0_dn5 = assign34400_e51710_d_n5;
        var_nj0_db0 = assign34400_e51710_d_b0;
        var_nj0_db1 = assign34400_e51710_d_b1;
        var_nj0_db2 = assign34400_e51710_d_b2;
        var_nj0_db3 = assign34400_e51710_d_b3;

        let (assign34410_e51717, assign34410_e51717_d_n0, assign34410_e51717_d_n1, assign34410_e51717_d_n2, assign34410_e51717_d_n3, assign34410_e51717_d_n4, assign34410_e51717_d_n5, assign34410_e51717_d_b0, assign34410_e51717_d_b1, assign34410_e51717_d_b2, assign34410_e51717_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn1, var_nj_k_dn2, var_nj_k_dn3, var_nj_k_dn4, var_nj_k_dn5, var_nj_k_db0, var_nj_k_db1, var_nj_k_db2, var_nj_k_db3,)
    }
};
        var_nj_k = assign34410_e51717;
        var_nj_k_dn0 = assign34410_e51717_d_n0;
        var_nj_k_dn1 = assign34410_e51717_d_n1;
        var_nj_k_dn2 = assign34410_e51717_d_n2;
        var_nj_k_dn3 = assign34410_e51717_d_n3;
        var_nj_k_dn4 = assign34410_e51717_d_n4;
        var_nj_k_dn5 = assign34410_e51717_d_n5;
        var_nj_k_db0 = assign34410_e51717_d_b0;
        var_nj_k_db1 = assign34410_e51717_d_b1;
        var_nj_k_db2 = assign34410_e51717_d_b2;
        var_nj_k_db3 = assign34410_e51717_d_b3;

        let (assign34420_e51724, assign34420_e51724_d_n0, assign34420_e51724_d_n1, assign34420_e51724_d_n2, assign34420_e51724_d_n3, assign34420_e51724_d_n4, assign34420_e51724_d_n5, assign34420_e51724_d_b0, assign34420_e51724_d_b1, assign34420_e51724_d_b2, assign34420_e51724_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn1, var_nj0_dn2, var_nj0_dn3, var_nj0_dn4, var_nj0_dn5, var_nj0_db0, var_nj0_db1, var_nj0_db2, var_nj0_db3,)
    }
};
        var_nj0 = assign34420_e51724;
        var_nj0_dn0 = assign34420_e51724_d_n0;
        var_nj0_dn1 = assign34420_e51724_d_n1;
        var_nj0_dn2 = assign34420_e51724_d_n2;
        var_nj0_dn3 = assign34420_e51724_d_n3;
        var_nj0_dn4 = assign34420_e51724_d_n4;
        var_nj0_dn5 = assign34420_e51724_d_n5;
        var_nj0_db0 = assign34420_e51724_d_b0;
        var_nj0_db1 = assign34420_e51724_d_b1;
        var_nj0_db2 = assign34420_e51724_d_b2;
        var_nj0_db3 = assign34420_e51724_d_b3;

        let (assign34430_e51728, assign34430_e51728_d_n0, assign34430_e51728_d_n1, assign34430_e51728_d_n2, assign34430_e51728_d_n3, assign34430_e51728_d_n4, assign34430_e51728_d_n5, assign34430_e51728_d_b0, assign34430_e51728_d_b1, assign34430_e51728_d_b2, assign34430_e51728_d_b3,) = {
    if (var_guard558 != 0.0) {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn1, var_idmultbot_dn2, var_idmultbot_dn3, var_idmultbot_dn4, var_idmultbot_dn5, var_idmultbot_db0, var_idmultbot_db1, var_idmultbot_db2, var_idmultbot_db3,)
    } else {
        (var_exp_a, var_exp_a_dn0, var_exp_a_dn1, var_exp_a_dn2, var_exp_a_dn3, var_exp_a_dn4, var_exp_a_dn5, var_exp_a_db0, var_exp_a_db1, var_exp_a_db2, var_exp_a_db3,)
    }
};
        var_exp_a = assign34430_e51728;
        var_exp_a_dn0 = assign34430_e51728_d_n0;
        var_exp_a_dn1 = assign34430_e51728_d_n1;
        var_exp_a_dn2 = assign34430_e51728_d_n2;
        var_exp_a_dn3 = assign34430_e51728_d_n3;
        var_exp_a_dn4 = assign34430_e51728_d_n4;
        var_exp_a_dn5 = assign34430_e51728_d_n5;
        var_exp_a_db0 = assign34430_e51728_d_b0;
        var_exp_a_db1 = assign34430_e51728_d_b1;
        var_exp_a_db2 = assign34430_e51728_d_b2;
        var_exp_a_db3 = assign34430_e51728_d_b3;

        let assign34440_e51732: f64 = (var_v_hk - var_v_ha);
        let assign34440_e51733: f64 = (var_vak - assign34440_e51732);
        let assign34440_e51735: f64 = if assign34440_e51733 > 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign34440_e51735;

        let assign34450_e51739: f64 = (var_vak / var_nj_k);
        let assign34450_e51742: f64 = (var_v_hk - var_v_ha);
        let assign34450_e51744: f64 = (assign34450_e51742 / var_nj_k);
        let assign34450_e51745: f64 = (assign34450_e51739 - assign34450_e51744);
        let assign34450_e51749: f64 = (var_nj_k - var_nj0);
        let assign34450_e51750: f64 = (var_v_hk * assign34450_e51749);
        let assign34450_e51753: f64 = (var_nj0 * p.p85);
        let assign34450_e51754: f64 = (assign34450_e51750 / assign34450_e51753);
        let assign34450_e51755: f64 = (assign34450_e51745 + assign34450_e51754);
        let assign34450_e51756: f64 = (var_phitdinv * assign34450_e51755);
        let assign34450_e51757: f64 = (assign34450_e51756).abs();
        let assign34450_e51759: f64 = if assign34450_e51757 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard561 = assign34450_e51759;

        let (assign34460_e51788, assign34460_e51788_d_n0, assign34460_e51788_d_n1, assign34460_e51788_d_n2, assign34460_e51788_d_n3, assign34460_e51788_d_n4, assign34460_e51788_d_n5, assign34460_e51788_d_b0, assign34460_e51788_d_b1, assign34460_e51788_d_b2, assign34460_e51788_d_b3,) = {
    if (((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 != 0.0)) {
        let assign34460_e51768: f64 = (var_vak / var_nj_k);
        let assign34460_e51771: f64 = (var_v_hk - var_v_ha);
        let assign34460_e51773: f64 = (assign34460_e51771 / var_nj_k);
        let assign34460_e51774: f64 = (assign34460_e51768 - assign34460_e51773);
        let assign34460_e51778: f64 = (var_nj_k - var_nj0);
        let assign34460_e51779: f64 = (var_v_hk * assign34460_e51778);
        let assign34460_e51782: f64 = (var_nj0 * p.p85);
        let assign34460_e51783: f64 = (assign34460_e51779 / assign34460_e51782);
        let assign34460_e51784: f64 = (assign34460_e51774 + assign34460_e51783);
        let assign34460_e51785: f64 = (var_phitdinv * assign34460_e51784);
        let assign34460_e51786: f64 = (assign34460_e51785).exp();
        (assign34460_e51786, (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn0 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn1 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn2 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn3 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn4 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn5 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db0 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db1 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db2 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_db3 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34460_e51788;
        var_exp_k_dn0 = assign34460_e51788_d_n0;
        var_exp_k_dn1 = assign34460_e51788_d_n1;
        var_exp_k_dn2 = assign34460_e51788_d_n2;
        var_exp_k_dn3 = assign34460_e51788_d_n3;
        var_exp_k_dn4 = assign34460_e51788_d_n4;
        var_exp_k_dn5 = assign34460_e51788_d_n5;
        var_exp_k_db0 = assign34460_e51788_d_b0;
        var_exp_k_db1 = assign34460_e51788_d_b1;
        var_exp_k_db2 = assign34460_e51788_d_b2;
        var_exp_k_db3 = assign34460_e51788_d_b3;

        let assign34470_e51792: f64 = (var_vak / var_nj_k);
        let assign34470_e51795: f64 = (var_v_hk - var_v_ha);
        let assign34470_e51797: f64 = (assign34470_e51795 / var_nj_k);
        let assign34470_e51798: f64 = (assign34470_e51792 - assign34470_e51797);
        let assign34470_e51802: f64 = (var_nj_k - var_nj0);
        let assign34470_e51803: f64 = (var_v_hk * assign34470_e51802);
        let assign34470_e51806: f64 = (var_nj0 * p.p85);
        let assign34470_e51807: f64 = (assign34470_e51803 / assign34470_e51806);
        let assign34470_e51808: f64 = (assign34470_e51798 + assign34470_e51807);
        let assign34470_e51809: f64 = (var_phitdinv * assign34470_e51808);
        let assign34470_e51811: f64 = (-230.25850929940458);
        let assign34470_e51812: f64 = if assign34470_e51809 < assign34470_e51811 { 1.0 } else { 0.0 };
        var_guard562 = assign34470_e51812;

        let (assign34480_e51908, assign34480_e51908_d_n0, assign34480_e51908_d_n1, assign34480_e51908_d_n2, assign34480_e51908_d_n3, assign34480_e51908_d_n4, assign34480_e51908_d_n5, assign34480_e51908_d_b0, assign34480_e51908_d_b1, assign34480_e51908_d_b2, assign34480_e51908_d_b3,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 != 0.0)) {
        let assign34480_e51824: f64 = (-230.25850929940458);
        let assign34480_e51828: f64 = (var_vak / var_nj_k);
        let assign34480_e51831: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51833: f64 = (assign34480_e51831 / var_nj_k);
        let assign34480_e51834: f64 = (assign34480_e51828 - assign34480_e51833);
        let assign34480_e51838: f64 = (var_nj_k - var_nj0);
        let assign34480_e51839: f64 = (var_v_hk * assign34480_e51838);
        let assign34480_e51842: f64 = (var_nj0 * p.p85);
        let assign34480_e51843: f64 = (assign34480_e51839 / assign34480_e51842);
        let assign34480_e51844: f64 = (assign34480_e51834 + assign34480_e51843);
        let assign34480_e51845: f64 = (var_phitdinv * assign34480_e51844);
        let assign34480_e51846: f64 = (assign34480_e51824 - assign34480_e51845);
        let assign34480_e51850: f64 = (-230.25850929940458);
        let assign34480_e51854: f64 = (var_vak / var_nj_k);
        let assign34480_e51857: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51859: f64 = (assign34480_e51857 / var_nj_k);
        let assign34480_e51860: f64 = (assign34480_e51854 - assign34480_e51859);
        let assign34480_e51864: f64 = (var_nj_k - var_nj0);
        let assign34480_e51865: f64 = (var_v_hk * assign34480_e51864);
        let assign34480_e51868: f64 = (var_nj0 * p.p85);
        let assign34480_e51869: f64 = (assign34480_e51865 / assign34480_e51868);
        let assign34480_e51870: f64 = (assign34480_e51860 + assign34480_e51869);
        let assign34480_e51871: f64 = (var_phitdinv * assign34480_e51870);
        let assign34480_e51872: f64 = (assign34480_e51850 - assign34480_e51871);
        let assign34480_e51875: f64 = (-230.25850929940458);
        let assign34480_e51879: f64 = (var_vak / var_nj_k);
        let assign34480_e51882: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51884: f64 = (assign34480_e51882 / var_nj_k);
        let assign34480_e51885: f64 = (assign34480_e51879 - assign34480_e51884);
        let assign34480_e51889: f64 = (var_nj_k - var_nj0);
        let assign34480_e51890: f64 = (var_v_hk * assign34480_e51889);
        let assign34480_e51893: f64 = (var_nj0 * p.p85);
        let assign34480_e51894: f64 = (assign34480_e51890 / assign34480_e51893);
        let assign34480_e51895: f64 = (assign34480_e51885 + assign34480_e51894);
        let assign34480_e51896: f64 = (var_phitdinv * assign34480_e51895);
        let assign34480_e51897: f64 = (assign34480_e51875 - assign34480_e51896);
        let assign34480_e51899: f64 = (assign34480_e51897 * 0.3333333333333333);
        let assign34480_e51900: f64 = (1.0 + assign34480_e51899);
        let assign34480_e51901: f64 = (assign34480_e51872 * assign34480_e51900);
        let assign34480_e51902: f64 = (0.5 * assign34480_e51901);
        let assign34480_e51903: f64 = (1.0 + assign34480_e51902);
        let assign34480_e51904: f64 = (assign34480_e51846 * assign34480_e51903);
        let assign34480_e51905: f64 = (1.0 + assign34480_e51904);
        let assign34480_e51906: f64 = (1e-100 / assign34480_e51905);
        (assign34480_e51906, (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn0 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn0 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn0 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn1 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn1 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn1 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn2 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn2 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn2 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn3 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn3 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn3 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn4 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn4 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn4 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn5 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn5 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn5 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db0 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db0 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db0 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db1 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db1 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db1 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db2 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db2 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db2 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_db3 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_db3 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_db3 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34480_e51908;
        var_exp_k_dn0 = assign34480_e51908_d_n0;
        var_exp_k_dn1 = assign34480_e51908_d_n1;
        var_exp_k_dn2 = assign34480_e51908_d_n2;
        var_exp_k_dn3 = assign34480_e51908_d_n3;
        var_exp_k_dn4 = assign34480_e51908_d_n4;
        var_exp_k_dn5 = assign34480_e51908_d_n5;
        var_exp_k_db0 = assign34480_e51908_d_b0;
        var_exp_k_db1 = assign34480_e51908_d_b1;
        var_exp_k_db2 = assign34480_e51908_d_b2;
        var_exp_k_db3 = assign34480_e51908_d_b3;


        *var_exp_a_slot = var_exp_a;
        *var_exp_a_db0_slot = var_exp_a_db0;
        *var_exp_a_db1_slot = var_exp_a_db1;
        *var_exp_a_db2_slot = var_exp_a_db2;
        *var_exp_a_db3_slot = var_exp_a_db3;
        *var_exp_a_dn0_slot = var_exp_a_dn0;
        *var_exp_a_dn1_slot = var_exp_a_dn1;
        *var_exp_a_dn2_slot = var_exp_a_dn2;
        *var_exp_a_dn3_slot = var_exp_a_dn3;
        *var_exp_a_dn4_slot = var_exp_a_dn4;
        *var_exp_a_dn5_slot = var_exp_a_dn5;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k_db0_slot = var_exp_k_db0;
        *var_exp_k_db1_slot = var_exp_k_db1;
        *var_exp_k_db2_slot = var_exp_k_db2;
        *var_exp_k_db3_slot = var_exp_k_db3;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn1_slot = var_exp_k_dn1;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_exp_k_dn3_slot = var_exp_k_dn3;
        *var_exp_k_dn4_slot = var_exp_k_dn4;
        *var_exp_k_dn5_slot = var_exp_k_dn5;
        *var_guard560_slot = var_guard560;
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
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
        *var_nj_k_slot = var_nj_k;
        *var_nj_k_db0_slot = var_nj_k_db0;
        *var_nj_k_db1_slot = var_nj_k_db1;
        *var_nj_k_db2_slot = var_nj_k_db2;
        *var_nj_k_db3_slot = var_nj_k_db3;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn1_slot = var_nj_k_dn1;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_nj_k_dn3_slot = var_nj_k_dn3;
        *var_nj_k_dn4_slot = var_nj_k_dn4;
        *var_nj_k_dn5_slot = var_nj_k_dn5;
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

    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_exp_a: f64,
        var_exp_a_db0: f64,
        var_exp_a_db1: f64,
        var_exp_a_db2: f64,
        var_exp_a_db3: f64,
        var_exp_a_dn0: f64,
        var_exp_a_dn1: f64,
        var_exp_a_dn2: f64,
        var_exp_a_dn3: f64,
        var_exp_a_dn4: f64,
        var_exp_a_dn5: f64,
        var_guard558: f64,
        var_guard560: f64,
        var_guard561: f64,
        var_guard562: f64,
        var_nj0: f64,
        var_nj0_db0: f64,
        var_nj0_db1: f64,
        var_nj0_db2: f64,
        var_nj0_db3: f64,
        var_nj0_dn0: f64,
        var_nj0_dn1: f64,
        var_nj0_dn2: f64,
        var_nj0_dn3: f64,
        var_nj0_dn4: f64,
        var_nj0_dn5: f64,
        var_nj_k: f64,
        var_nj_k_db0: f64,
        var_nj_k_db1: f64,
        var_nj_k_db2: f64,
        var_nj_k_db3: f64,
        var_nj_k_dn0: f64,
        var_nj_k_dn1: f64,
        var_nj_k_dn2: f64,
        var_nj_k_dn3: f64,
        var_nj_k_dn4: f64,
        var_nj_k_dn5: f64,
        var_phitdinv: f64,
        var_pn0: f64,
        var_q_pex0: f64,
        var_tkd: f64,
        var_tkr: f64,
        var_v_ha: f64,
        var_v_hk: f64,
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
        var_exp_a2_slot: &mut f64,
        var_exp_a2_db0_slot: &mut f64,
        var_exp_a2_db1_slot: &mut f64,
        var_exp_a2_db2_slot: &mut f64,
        var_exp_a2_db3_slot: &mut f64,
        var_exp_a2_dn0_slot: &mut f64,
        var_exp_a2_dn1_slot: &mut f64,
        var_exp_a2_dn2_slot: &mut f64,
        var_exp_a2_dn3_slot: &mut f64,
        var_exp_a2_dn4_slot: &mut f64,
        var_exp_a2_dn5_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k2_slot: &mut f64,
        var_exp_k2_db0_slot: &mut f64,
        var_exp_k2_db1_slot: &mut f64,
        var_exp_k2_db2_slot: &mut f64,
        var_exp_k2_db3_slot: &mut f64,
        var_exp_k2_dn0_slot: &mut f64,
        var_exp_k2_dn1_slot: &mut f64,
        var_exp_k2_dn2_slot: &mut f64,
        var_exp_k2_dn3_slot: &mut f64,
        var_exp_k2_dn4_slot: &mut f64,
        var_exp_k2_dn5_slot: &mut f64,
        var_exp_k_db0_slot: &mut f64,
        var_exp_k_db1_slot: &mut f64,
        var_exp_k_db2_slot: &mut f64,
        var_exp_k_db3_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn1_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_exp_k_dn3_slot: &mut f64,
        var_exp_k_dn4_slot: &mut f64,
        var_exp_k_dn5_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_db0_slot: &mut f64,
        var_inqs0_a_db1_slot: &mut f64,
        var_inqs0_a_db2_slot: &mut f64,
        var_inqs0_a_db3_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn1_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn3_slot: &mut f64,
        var_inqs0_a_dn4_slot: &mut f64,
        var_inqs0_a_dn5_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_db0_slot: &mut f64,
        var_inqs0_k_db1_slot: &mut f64,
        var_inqs0_k_db2_slot: &mut f64,
        var_inqs0_k_db3_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn1_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn3_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_inqs0_k_dn5_slot: &mut f64,
        var_p_na_slot: &mut f64,
        var_p_na_db0_slot: &mut f64,
        var_p_na_db1_slot: &mut f64,
        var_p_na_db2_slot: &mut f64,
        var_p_na_db3_slot: &mut f64,
        var_p_na_dn0_slot: &mut f64,
        var_p_na_dn1_slot: &mut f64,
        var_p_na_dn2_slot: &mut f64,
        var_p_na_dn3_slot: &mut f64,
        var_p_na_dn4_slot: &mut f64,
        var_p_na_dn5_slot: &mut f64,
        var_p_nk_slot: &mut f64,
        var_p_nk_db0_slot: &mut f64,
        var_p_nk_db1_slot: &mut f64,
        var_p_nk_db2_slot: &mut f64,
        var_p_nk_db3_slot: &mut f64,
        var_p_nk_dn0_slot: &mut f64,
        var_p_nk_dn1_slot: &mut f64,
        var_p_nk_dn2_slot: &mut f64,
        var_p_nk_dn3_slot: &mut f64,
        var_p_nk_dn4_slot: &mut f64,
        var_p_nk_dn5_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_db0_slot: &mut f64,
        var_q_nqs_a_db1_slot: &mut f64,
        var_q_nqs_a_db2_slot: &mut f64,
        var_q_nqs_a_db3_slot: &mut f64,
        var_q_nqs_a_dn0_slot: &mut f64,
        var_q_nqs_a_dn1_slot: &mut f64,
        var_q_nqs_a_dn2_slot: &mut f64,
        var_q_nqs_a_dn3_slot: &mut f64,
        var_q_nqs_a_dn4_slot: &mut f64,
        var_q_nqs_a_dn5_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_db0_slot: &mut f64,
        var_q_nqs_k_db1_slot: &mut f64,
        var_q_nqs_k_db2_slot: &mut f64,
        var_q_nqs_k_db3_slot: &mut f64,
        var_q_nqs_k_dn0_slot: &mut f64,
        var_q_nqs_k_dn1_slot: &mut f64,
        var_q_nqs_k_dn2_slot: &mut f64,
        var_q_nqs_k_dn3_slot: &mut f64,
        var_q_nqs_k_dn4_slot: &mut f64,
        var_q_nqs_k_dn5_slot: &mut f64,
        var_q_pexa_slot: &mut f64,
        var_q_pexa_db0_slot: &mut f64,
        var_q_pexa_db1_slot: &mut f64,
        var_q_pexa_db2_slot: &mut f64,
        var_q_pexa_db3_slot: &mut f64,
        var_q_pexa_dn0_slot: &mut f64,
        var_q_pexa_dn1_slot: &mut f64,
        var_q_pexa_dn2_slot: &mut f64,
        var_q_pexa_dn3_slot: &mut f64,
        var_q_pexa_dn4_slot: &mut f64,
        var_q_pexa_dn5_slot: &mut f64,
        var_q_pexk_slot: &mut f64,
        var_q_pexk_db0_slot: &mut f64,
        var_q_pexk_db1_slot: &mut f64,
        var_q_pexk_db2_slot: &mut f64,
        var_q_pexk_db3_slot: &mut f64,
        var_q_pexk_dn0_slot: &mut f64,
        var_q_pexk_dn1_slot: &mut f64,
        var_q_pexk_dn2_slot: &mut f64,
        var_q_pexk_dn3_slot: &mut f64,
        var_q_pexk_dn4_slot: &mut f64,
        var_q_pexk_dn5_slot: &mut f64,
        var_q_qs_a_slot: &mut f64,
        var_q_qs_a_db0_slot: &mut f64,
        var_q_qs_a_db1_slot: &mut f64,
        var_q_qs_a_db2_slot: &mut f64,
        var_q_qs_a_db3_slot: &mut f64,
        var_q_qs_a_dn0_slot: &mut f64,
        var_q_qs_a_dn1_slot: &mut f64,
        var_q_qs_a_dn2_slot: &mut f64,
        var_q_qs_a_dn3_slot: &mut f64,
        var_q_qs_a_dn4_slot: &mut f64,
        var_q_qs_a_dn5_slot: &mut f64,
        var_q_qs_k_slot: &mut f64,
        var_q_qs_k_db0_slot: &mut f64,
        var_q_qs_k_db1_slot: &mut f64,
        var_q_qs_k_db2_slot: &mut f64,
        var_q_qs_k_db3_slot: &mut f64,
        var_q_qs_k_dn0_slot: &mut f64,
        var_q_qs_k_dn1_slot: &mut f64,
        var_q_qs_k_dn2_slot: &mut f64,
        var_q_qs_k_dn3_slot: &mut f64,
        var_q_qs_k_dn4_slot: &mut f64,
        var_q_qs_k_dn5_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_exp_a2: f64 = *var_exp_a2_slot;
        let mut var_exp_a2_db0: f64 = *var_exp_a2_db0_slot;
        let mut var_exp_a2_db1: f64 = *var_exp_a2_db1_slot;
        let mut var_exp_a2_db2: f64 = *var_exp_a2_db2_slot;
        let mut var_exp_a2_db3: f64 = *var_exp_a2_db3_slot;
        let mut var_exp_a2_dn0: f64 = *var_exp_a2_dn0_slot;
        let mut var_exp_a2_dn1: f64 = *var_exp_a2_dn1_slot;
        let mut var_exp_a2_dn2: f64 = *var_exp_a2_dn2_slot;
        let mut var_exp_a2_dn3: f64 = *var_exp_a2_dn3_slot;
        let mut var_exp_a2_dn4: f64 = *var_exp_a2_dn4_slot;
        let mut var_exp_a2_dn5: f64 = *var_exp_a2_dn5_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k2: f64 = *var_exp_k2_slot;
        let mut var_exp_k2_db0: f64 = *var_exp_k2_db0_slot;
        let mut var_exp_k2_db1: f64 = *var_exp_k2_db1_slot;
        let mut var_exp_k2_db2: f64 = *var_exp_k2_db2_slot;
        let mut var_exp_k2_db3: f64 = *var_exp_k2_db3_slot;
        let mut var_exp_k2_dn0: f64 = *var_exp_k2_dn0_slot;
        let mut var_exp_k2_dn1: f64 = *var_exp_k2_dn1_slot;
        let mut var_exp_k2_dn2: f64 = *var_exp_k2_dn2_slot;
        let mut var_exp_k2_dn3: f64 = *var_exp_k2_dn3_slot;
        let mut var_exp_k2_dn4: f64 = *var_exp_k2_dn4_slot;
        let mut var_exp_k2_dn5: f64 = *var_exp_k2_dn5_slot;
        let mut var_exp_k_db0: f64 = *var_exp_k_db0_slot;
        let mut var_exp_k_db1: f64 = *var_exp_k_db1_slot;
        let mut var_exp_k_db2: f64 = *var_exp_k_db2_slot;
        let mut var_exp_k_db3: f64 = *var_exp_k_db3_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn1: f64 = *var_exp_k_dn1_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_exp_k_dn3: f64 = *var_exp_k_dn3_slot;
        let mut var_exp_k_dn4: f64 = *var_exp_k_dn4_slot;
        let mut var_exp_k_dn5: f64 = *var_exp_k_dn5_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_db0: f64 = *var_inqs0_a_db0_slot;
        let mut var_inqs0_a_db1: f64 = *var_inqs0_a_db1_slot;
        let mut var_inqs0_a_db2: f64 = *var_inqs0_a_db2_slot;
        let mut var_inqs0_a_db3: f64 = *var_inqs0_a_db3_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn1: f64 = *var_inqs0_a_dn1_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn3: f64 = *var_inqs0_a_dn3_slot;
        let mut var_inqs0_a_dn4: f64 = *var_inqs0_a_dn4_slot;
        let mut var_inqs0_a_dn5: f64 = *var_inqs0_a_dn5_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_db0: f64 = *var_inqs0_k_db0_slot;
        let mut var_inqs0_k_db1: f64 = *var_inqs0_k_db1_slot;
        let mut var_inqs0_k_db2: f64 = *var_inqs0_k_db2_slot;
        let mut var_inqs0_k_db3: f64 = *var_inqs0_k_db3_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn1: f64 = *var_inqs0_k_dn1_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn3: f64 = *var_inqs0_k_dn3_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_inqs0_k_dn5: f64 = *var_inqs0_k_dn5_slot;
        let mut var_p_na: f64 = *var_p_na_slot;
        let mut var_p_na_db0: f64 = *var_p_na_db0_slot;
        let mut var_p_na_db1: f64 = *var_p_na_db1_slot;
        let mut var_p_na_db2: f64 = *var_p_na_db2_slot;
        let mut var_p_na_db3: f64 = *var_p_na_db3_slot;
        let mut var_p_na_dn0: f64 = *var_p_na_dn0_slot;
        let mut var_p_na_dn1: f64 = *var_p_na_dn1_slot;
        let mut var_p_na_dn2: f64 = *var_p_na_dn2_slot;
        let mut var_p_na_dn3: f64 = *var_p_na_dn3_slot;
        let mut var_p_na_dn4: f64 = *var_p_na_dn4_slot;
        let mut var_p_na_dn5: f64 = *var_p_na_dn5_slot;
        let mut var_p_nk: f64 = *var_p_nk_slot;
        let mut var_p_nk_db0: f64 = *var_p_nk_db0_slot;
        let mut var_p_nk_db1: f64 = *var_p_nk_db1_slot;
        let mut var_p_nk_db2: f64 = *var_p_nk_db2_slot;
        let mut var_p_nk_db3: f64 = *var_p_nk_db3_slot;
        let mut var_p_nk_dn0: f64 = *var_p_nk_dn0_slot;
        let mut var_p_nk_dn1: f64 = *var_p_nk_dn1_slot;
        let mut var_p_nk_dn2: f64 = *var_p_nk_dn2_slot;
        let mut var_p_nk_dn3: f64 = *var_p_nk_dn3_slot;
        let mut var_p_nk_dn4: f64 = *var_p_nk_dn4_slot;
        let mut var_p_nk_dn5: f64 = *var_p_nk_dn5_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_db0: f64 = *var_q_nqs_a_db0_slot;
        let mut var_q_nqs_a_db1: f64 = *var_q_nqs_a_db1_slot;
        let mut var_q_nqs_a_db2: f64 = *var_q_nqs_a_db2_slot;
        let mut var_q_nqs_a_db3: f64 = *var_q_nqs_a_db3_slot;
        let mut var_q_nqs_a_dn0: f64 = *var_q_nqs_a_dn0_slot;
        let mut var_q_nqs_a_dn1: f64 = *var_q_nqs_a_dn1_slot;
        let mut var_q_nqs_a_dn2: f64 = *var_q_nqs_a_dn2_slot;
        let mut var_q_nqs_a_dn3: f64 = *var_q_nqs_a_dn3_slot;
        let mut var_q_nqs_a_dn4: f64 = *var_q_nqs_a_dn4_slot;
        let mut var_q_nqs_a_dn5: f64 = *var_q_nqs_a_dn5_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_db0: f64 = *var_q_nqs_k_db0_slot;
        let mut var_q_nqs_k_db1: f64 = *var_q_nqs_k_db1_slot;
        let mut var_q_nqs_k_db2: f64 = *var_q_nqs_k_db2_slot;
        let mut var_q_nqs_k_db3: f64 = *var_q_nqs_k_db3_slot;
        let mut var_q_nqs_k_dn0: f64 = *var_q_nqs_k_dn0_slot;
        let mut var_q_nqs_k_dn1: f64 = *var_q_nqs_k_dn1_slot;
        let mut var_q_nqs_k_dn2: f64 = *var_q_nqs_k_dn2_slot;
        let mut var_q_nqs_k_dn3: f64 = *var_q_nqs_k_dn3_slot;
        let mut var_q_nqs_k_dn4: f64 = *var_q_nqs_k_dn4_slot;
        let mut var_q_nqs_k_dn5: f64 = *var_q_nqs_k_dn5_slot;
        let mut var_q_pexa: f64 = *var_q_pexa_slot;
        let mut var_q_pexa_db0: f64 = *var_q_pexa_db0_slot;
        let mut var_q_pexa_db1: f64 = *var_q_pexa_db1_slot;
        let mut var_q_pexa_db2: f64 = *var_q_pexa_db2_slot;
        let mut var_q_pexa_db3: f64 = *var_q_pexa_db3_slot;
        let mut var_q_pexa_dn0: f64 = *var_q_pexa_dn0_slot;
        let mut var_q_pexa_dn1: f64 = *var_q_pexa_dn1_slot;
        let mut var_q_pexa_dn2: f64 = *var_q_pexa_dn2_slot;
        let mut var_q_pexa_dn3: f64 = *var_q_pexa_dn3_slot;
        let mut var_q_pexa_dn4: f64 = *var_q_pexa_dn4_slot;
        let mut var_q_pexa_dn5: f64 = *var_q_pexa_dn5_slot;
        let mut var_q_pexk: f64 = *var_q_pexk_slot;
        let mut var_q_pexk_db0: f64 = *var_q_pexk_db0_slot;
        let mut var_q_pexk_db1: f64 = *var_q_pexk_db1_slot;
        let mut var_q_pexk_db2: f64 = *var_q_pexk_db2_slot;
        let mut var_q_pexk_db3: f64 = *var_q_pexk_db3_slot;
        let mut var_q_pexk_dn0: f64 = *var_q_pexk_dn0_slot;
        let mut var_q_pexk_dn1: f64 = *var_q_pexk_dn1_slot;
        let mut var_q_pexk_dn2: f64 = *var_q_pexk_dn2_slot;
        let mut var_q_pexk_dn3: f64 = *var_q_pexk_dn3_slot;
        let mut var_q_pexk_dn4: f64 = *var_q_pexk_dn4_slot;
        let mut var_q_pexk_dn5: f64 = *var_q_pexk_dn5_slot;
        let mut var_q_qs_a: f64 = *var_q_qs_a_slot;
        let mut var_q_qs_a_db0: f64 = *var_q_qs_a_db0_slot;
        let mut var_q_qs_a_db1: f64 = *var_q_qs_a_db1_slot;
        let mut var_q_qs_a_db2: f64 = *var_q_qs_a_db2_slot;
        let mut var_q_qs_a_db3: f64 = *var_q_qs_a_db3_slot;
        let mut var_q_qs_a_dn0: f64 = *var_q_qs_a_dn0_slot;
        let mut var_q_qs_a_dn1: f64 = *var_q_qs_a_dn1_slot;
        let mut var_q_qs_a_dn2: f64 = *var_q_qs_a_dn2_slot;
        let mut var_q_qs_a_dn3: f64 = *var_q_qs_a_dn3_slot;
        let mut var_q_qs_a_dn4: f64 = *var_q_qs_a_dn4_slot;
        let mut var_q_qs_a_dn5: f64 = *var_q_qs_a_dn5_slot;
        let mut var_q_qs_k: f64 = *var_q_qs_k_slot;
        let mut var_q_qs_k_db0: f64 = *var_q_qs_k_db0_slot;
        let mut var_q_qs_k_db1: f64 = *var_q_qs_k_db1_slot;
        let mut var_q_qs_k_db2: f64 = *var_q_qs_k_db2_slot;
        let mut var_q_qs_k_db3: f64 = *var_q_qs_k_db3_slot;
        let mut var_q_qs_k_dn0: f64 = *var_q_qs_k_dn0_slot;
        let mut var_q_qs_k_dn1: f64 = *var_q_qs_k_dn1_slot;
        let mut var_q_qs_k_dn2: f64 = *var_q_qs_k_dn2_slot;
        let mut var_q_qs_k_dn3: f64 = *var_q_qs_k_dn3_slot;
        let mut var_q_qs_k_dn4: f64 = *var_q_qs_k_dn4_slot;
        let mut var_q_qs_k_dn5: f64 = *var_q_qs_k_dn5_slot;

        let (assign34490_e52002, assign34490_e52002_d_n0, assign34490_e52002_d_n1, assign34490_e52002_d_n2, assign34490_e52002_d_n3, assign34490_e52002_d_n4, assign34490_e52002_d_n5, assign34490_e52002_d_b0, assign34490_e52002_d_b1, assign34490_e52002_d_b2, assign34490_e52002_d_b3,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 == 0.0)) {
        let assign34490_e51923: f64 = (var_vak / var_nj_k);
        let assign34490_e51926: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51928: f64 = (assign34490_e51926 / var_nj_k);
        let assign34490_e51929: f64 = (assign34490_e51923 - assign34490_e51928);
        let assign34490_e51933: f64 = (var_nj_k - var_nj0);
        let assign34490_e51934: f64 = (var_v_hk * assign34490_e51933);
        let assign34490_e51937: f64 = (var_nj0 * p.p85);
        let assign34490_e51938: f64 = (assign34490_e51934 / assign34490_e51937);
        let assign34490_e51939: f64 = (assign34490_e51929 + assign34490_e51938);
        let assign34490_e51940: f64 = (var_phitdinv * assign34490_e51939);
        let assign34490_e51942: f64 = (assign34490_e51940 - 230.25850929940458);
        let assign34490_e51948: f64 = (var_vak / var_nj_k);
        let assign34490_e51951: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51953: f64 = (assign34490_e51951 / var_nj_k);
        let assign34490_e51954: f64 = (assign34490_e51948 - assign34490_e51953);
        let assign34490_e51958: f64 = (var_nj_k - var_nj0);
        let assign34490_e51959: f64 = (var_v_hk * assign34490_e51958);
        let assign34490_e51962: f64 = (var_nj0 * p.p85);
        let assign34490_e51963: f64 = (assign34490_e51959 / assign34490_e51962);
        let assign34490_e51964: f64 = (assign34490_e51954 + assign34490_e51963);
        let assign34490_e51965: f64 = (var_phitdinv * assign34490_e51964);
        let assign34490_e51967: f64 = (assign34490_e51965 - 230.25850929940458);
        let assign34490_e51972: f64 = (var_vak / var_nj_k);
        let assign34490_e51975: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51977: f64 = (assign34490_e51975 / var_nj_k);
        let assign34490_e51978: f64 = (assign34490_e51972 - assign34490_e51977);
        let assign34490_e51982: f64 = (var_nj_k - var_nj0);
        let assign34490_e51983: f64 = (var_v_hk * assign34490_e51982);
        let assign34490_e51986: f64 = (var_nj0 * p.p85);
        let assign34490_e51987: f64 = (assign34490_e51983 / assign34490_e51986);
        let assign34490_e51988: f64 = (assign34490_e51978 + assign34490_e51987);
        let assign34490_e51989: f64 = (var_phitdinv * assign34490_e51988);
        let assign34490_e51991: f64 = (assign34490_e51989 - 230.25850929940458);
        let assign34490_e51993: f64 = (assign34490_e51991 * 0.3333333333333333);
        let assign34490_e51994: f64 = (1.0 + assign34490_e51993);
        let assign34490_e51995: f64 = (assign34490_e51967 * assign34490_e51994);
        let assign34490_e51996: f64 = (0.5 * assign34490_e51995);
        let assign34490_e51997: f64 = (1.0 + assign34490_e51996);
        let assign34490_e51998: f64 = (assign34490_e51942 * assign34490_e51997);
        let assign34490_e51999: f64 = (1.0 + assign34490_e51998);
        let assign34490_e52000: f64 = (1e100 * assign34490_e51999);
        (assign34490_e52000, (1e100 * (((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn0 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn0 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn0 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn1 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn1 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn1 * var_nj_k) - (var_vak * var_nj_k_dn1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn1 - var_nj0_dn1)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn1 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn2 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn2 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn2 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn3 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn3 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn3 * var_nj_k) - (var_vak * var_nj_k_dn3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn3 - var_nj0_dn3)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn3 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn4 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn4 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn4 * var_nj_k) - (var_vak * var_nj_k_dn4)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn4) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn4 - var_nj0_dn4)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn4 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn5 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn5 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn5 * var_nj_k) - (var_vak * var_nj_k_dn5)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn5) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn5 - var_nj0_dn5)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn5 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db0 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db0 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_db0 * var_nj_k) - (var_vak * var_nj_k_db0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_db0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db0 - var_nj0_db0)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db0 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db1 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db1 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_db1 * var_nj_k) - (var_vak * var_nj_k_db1)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_db1) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db1 - var_nj0_db1)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db1 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db2 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db2 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_db2 * var_nj_k) - (var_vak * var_nj_k_db2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_db2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db2 - var_nj0_db2)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db2 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_db3 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_db3 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_db3 * var_nj_k) - (var_vak * var_nj_k_db3)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_db3) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_db3 - var_nj0_db3)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_db3 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34490_e52002;
        var_exp_k_dn0 = assign34490_e52002_d_n0;
        var_exp_k_dn1 = assign34490_e52002_d_n1;
        var_exp_k_dn2 = assign34490_e52002_d_n2;
        var_exp_k_dn3 = assign34490_e52002_d_n3;
        var_exp_k_dn4 = assign34490_e52002_d_n4;
        var_exp_k_dn5 = assign34490_e52002_d_n5;
        var_exp_k_db0 = assign34490_e52002_d_b0;
        var_exp_k_db1 = assign34490_e52002_d_b1;
        var_exp_k_db2 = assign34490_e52002_d_b2;
        var_exp_k_db3 = assign34490_e52002_d_b3;

        let (assign34500_e52009, assign34500_e52009_d_n0, assign34500_e52009_d_n1, assign34500_e52009_d_n2, assign34500_e52009_d_n3, assign34500_e52009_d_n4, assign34500_e52009_d_n5, assign34500_e52009_d_b0, assign34500_e52009_d_b1, assign34500_e52009_d_b2, assign34500_e52009_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard560 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn1, var_exp_k_dn2, var_exp_k_dn3, var_exp_k_dn4, var_exp_k_dn5, var_exp_k_db0, var_exp_k_db1, var_exp_k_db2, var_exp_k_db3,)
    }
};
        var_exp_k = assign34500_e52009;
        var_exp_k_dn0 = assign34500_e52009_d_n0;
        var_exp_k_dn1 = assign34500_e52009_d_n1;
        var_exp_k_dn2 = assign34500_e52009_d_n2;
        var_exp_k_dn3 = assign34500_e52009_d_n3;
        var_exp_k_dn4 = assign34500_e52009_d_n4;
        var_exp_k_dn5 = assign34500_e52009_d_n5;
        var_exp_k_db0 = assign34500_e52009_d_b0;
        var_exp_k_db1 = assign34500_e52009_d_b1;
        var_exp_k_db2 = assign34500_e52009_d_b2;
        var_exp_k_db3 = assign34500_e52009_d_b3;

        let assign34510_e52016: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_ha)) { 1.0 } else { 0.0 };
        var_guard563 = assign34510_e52016;

        let (assign34520_e52024, assign34520_e52024_d_n0, assign34520_e52024_d_n1, assign34520_e52024_d_n2, assign34520_e52024_d_n3, assign34520_e52024_d_n4, assign34520_e52024_d_n5, assign34520_e52024_d_b0, assign34520_e52024_d_b1, assign34520_e52024_d_b2, assign34520_e52024_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard563 != 0.0)) {
        let assign34520_e52022: f64 = (var_exp_a * p.p90);
        (assign34520_e52022, (var_exp_a_dn0 * p.p90), (var_exp_a_dn1 * p.p90), (var_exp_a_dn2 * p.p90), (var_exp_a_dn3 * p.p90), (var_exp_a_dn4 * p.p90), (var_exp_a_dn5 * p.p90), (var_exp_a_db0 * p.p90), (var_exp_a_db1 * p.p90), (var_exp_a_db2 * p.p90), (var_exp_a_db3 * p.p90),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34520_e52024;
        var_exp_a2_dn0 = assign34520_e52024_d_n0;
        var_exp_a2_dn1 = assign34520_e52024_d_n1;
        var_exp_a2_dn2 = assign34520_e52024_d_n2;
        var_exp_a2_dn3 = assign34520_e52024_d_n3;
        var_exp_a2_dn4 = assign34520_e52024_d_n4;
        var_exp_a2_dn5 = assign34520_e52024_d_n5;
        var_exp_a2_db0 = assign34520_e52024_d_b0;
        var_exp_a2_db1 = assign34520_e52024_d_b1;
        var_exp_a2_db2 = assign34520_e52024_d_b2;
        var_exp_a2_db3 = assign34520_e52024_d_b3;

        let (assign34530_e52053, assign34530_e52053_d_n0, assign34530_e52053_d_n1, assign34530_e52053_d_n2, assign34530_e52053_d_n3, assign34530_e52053_d_n4, assign34530_e52053_d_n5, assign34530_e52053_d_b0, assign34530_e52053_d_b1, assign34530_e52053_d_b2, assign34530_e52053_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard563 == 0.0)) {
        let assign34530_e52031: f64 = (var_exp_a * p.p90);
        let assign34530_e52033: f64 = (-p.p91);
        let assign34530_e52036: f64 = (var_vak - var_v_ha);
        let assign34530_e52037: f64 = (assign34530_e52033 * assign34530_e52036);
        let assign34530_e52040: f64 = (var_vak - var_v_ha);
        let assign34530_e52041: f64 = (assign34530_e52037 * assign34530_e52040);
        let assign34530_e52045: f64 = (var_tkr / var_tkd);
        let assign34530_e52046: f64 = (assign34530_e52045).ln();
        let assign34530_e52047: f64 = (p.p98 * assign34530_e52046);
        let assign34530_e52048: f64 = (assign34530_e52047).exp();
        let assign34530_e52049: f64 = (assign34530_e52041 * assign34530_e52048);
        let assign34530_e52050: f64 = (assign34530_e52049).exp();
        let assign34530_e52051: f64 = (assign34530_e52031 * assign34530_e52050);
        (assign34530_e52051, (((var_exp_a_dn0 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn0) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn0)) * assign34530_e52048)))), (((var_exp_a_dn1 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn1) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn1)) * assign34530_e52048)))), (((var_exp_a_dn2 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn2) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn2)) * assign34530_e52048)))), (((var_exp_a_dn3 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn3) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn3)) * assign34530_e52048)))), (((var_exp_a_dn4 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn4) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn4)) * assign34530_e52048)))), (((var_exp_a_dn5 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn5) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn5)) * assign34530_e52048)))), (((var_exp_a_db0 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_db0) * assign34530_e52040) + (assign34530_e52037 * var_vak_db0)) * assign34530_e52048)))), (((var_exp_a_db1 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_db1) * assign34530_e52040) + (assign34530_e52037 * var_vak_db1)) * assign34530_e52048)))), (((var_exp_a_db2 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_db2) * assign34530_e52040) + (assign34530_e52037 * var_vak_db2)) * assign34530_e52048)))), (((var_exp_a_db3 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_db3) * assign34530_e52040) + (assign34530_e52037 * var_vak_db3)) * assign34530_e52048)))),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34530_e52053;
        var_exp_a2_dn0 = assign34530_e52053_d_n0;
        var_exp_a2_dn1 = assign34530_e52053_d_n1;
        var_exp_a2_dn2 = assign34530_e52053_d_n2;
        var_exp_a2_dn3 = assign34530_e52053_d_n3;
        var_exp_a2_dn4 = assign34530_e52053_d_n4;
        var_exp_a2_dn5 = assign34530_e52053_d_n5;
        var_exp_a2_db0 = assign34530_e52053_d_b0;
        var_exp_a2_db1 = assign34530_e52053_d_b1;
        var_exp_a2_db2 = assign34530_e52053_d_b2;
        var_exp_a2_db3 = assign34530_e52053_d_b3;

        let (assign34540_e52062, assign34540_e52062_d_n0, assign34540_e52062_d_n1, assign34540_e52062_d_n2, assign34540_e52062_d_n3, assign34540_e52062_d_n4, assign34540_e52062_d_n5, assign34540_e52062_d_b0, assign34540_e52062_d_b1, assign34540_e52062_d_b2, assign34540_e52062_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n1, assign34540_e52060_d_n2, assign34540_e52060_d_n3, assign34540_e52060_d_n4, assign34540_e52060_d_n5, assign34540_e52060_d_b0, assign34540_e52060_d_b1, assign34540_e52060_d_b2, assign34540_e52060_d_b3,) = {
            if (var_exp_a2 > p.p79) {
                (p.p79, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
            }
        };
        (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n1, assign34540_e52060_d_n2, assign34540_e52060_d_n3, assign34540_e52060_d_n4, assign34540_e52060_d_n5, assign34540_e52060_d_b0, assign34540_e52060_d_b1, assign34540_e52060_d_b2, assign34540_e52060_d_b3,)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn1, var_exp_a2_dn2, var_exp_a2_dn3, var_exp_a2_dn4, var_exp_a2_dn5, var_exp_a2_db0, var_exp_a2_db1, var_exp_a2_db2, var_exp_a2_db3,)
    }
};
        var_exp_a2 = assign34540_e52062;
        var_exp_a2_dn0 = assign34540_e52062_d_n0;
        var_exp_a2_dn1 = assign34540_e52062_d_n1;
        var_exp_a2_dn2 = assign34540_e52062_d_n2;
        var_exp_a2_dn3 = assign34540_e52062_d_n3;
        var_exp_a2_dn4 = assign34540_e52062_d_n4;
        var_exp_a2_dn5 = assign34540_e52062_d_n5;
        var_exp_a2_db0 = assign34540_e52062_d_b0;
        var_exp_a2_db1 = assign34540_e52062_d_b1;
        var_exp_a2_db2 = assign34540_e52062_d_b2;
        var_exp_a2_db3 = assign34540_e52062_d_b3;

        let (assign34550_e52068, assign34550_e52068_d_n0, assign34550_e52068_d_n1, assign34550_e52068_d_n2, assign34550_e52068_d_n3, assign34550_e52068_d_n4, assign34550_e52068_d_n5, assign34550_e52068_d_b0, assign34550_e52068_d_b1, assign34550_e52068_d_b2, assign34550_e52068_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34550_e52066: f64 = (var_pn0 * var_exp_a2);
        (assign34550_e52066, (var_pn0 * var_exp_a2_dn0), (var_pn0 * var_exp_a2_dn1), (var_pn0 * var_exp_a2_dn2), (var_pn0 * var_exp_a2_dn3), (var_pn0 * var_exp_a2_dn4), (var_pn0 * var_exp_a2_dn5), (var_pn0 * var_exp_a2_db0), (var_pn0 * var_exp_a2_db1), (var_pn0 * var_exp_a2_db2), (var_pn0 * var_exp_a2_db3),)
    } else {
        (var_p_na, var_p_na_dn0, var_p_na_dn1, var_p_na_dn2, var_p_na_dn3, var_p_na_dn4, var_p_na_dn5, var_p_na_db0, var_p_na_db1, var_p_na_db2, var_p_na_db3,)
    }
};
        var_p_na = assign34550_e52068;
        var_p_na_dn0 = assign34550_e52068_d_n0;
        var_p_na_dn1 = assign34550_e52068_d_n1;
        var_p_na_dn2 = assign34550_e52068_d_n2;
        var_p_na_dn3 = assign34550_e52068_d_n3;
        var_p_na_dn4 = assign34550_e52068_d_n4;
        var_p_na_dn5 = assign34550_e52068_d_n5;
        var_p_na_db0 = assign34550_e52068_d_b0;
        var_p_na_db1 = assign34550_e52068_d_b1;
        var_p_na_db2 = assign34550_e52068_d_b2;
        var_p_na_db3 = assign34550_e52068_d_b3;

        let (assign34560_e52078, assign34560_e52078_d_n0, assign34560_e52078_d_n1, assign34560_e52078_d_n2, assign34560_e52078_d_n3, assign34560_e52078_d_n4, assign34560_e52078_d_n5, assign34560_e52078_d_b0, assign34560_e52078_d_b1, assign34560_e52078_d_b2, assign34560_e52078_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34560_e52072: f64 = (1.6021918e-19 * var_ab_i);
        let assign34560_e52075: f64 = (var_p_na - var_pn0);
        let assign34560_e52076: f64 = (assign34560_e52072 * assign34560_e52075);
        (assign34560_e52076, (assign34560_e52072 * var_p_na_dn0), (assign34560_e52072 * var_p_na_dn1), (assign34560_e52072 * var_p_na_dn2), (assign34560_e52072 * var_p_na_dn3), (assign34560_e52072 * var_p_na_dn4), (assign34560_e52072 * var_p_na_dn5), (assign34560_e52072 * var_p_na_db0), (assign34560_e52072 * var_p_na_db1), (assign34560_e52072 * var_p_na_db2), (assign34560_e52072 * var_p_na_db3),)
    } else {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn1, var_q_pexa_dn2, var_q_pexa_dn3, var_q_pexa_dn4, var_q_pexa_dn5, var_q_pexa_db0, var_q_pexa_db1, var_q_pexa_db2, var_q_pexa_db3,)
    }
};
        var_q_pexa = assign34560_e52078;
        var_q_pexa_dn0 = assign34560_e52078_d_n0;
        var_q_pexa_dn1 = assign34560_e52078_d_n1;
        var_q_pexa_dn2 = assign34560_e52078_d_n2;
        var_q_pexa_dn3 = assign34560_e52078_d_n3;
        var_q_pexa_dn4 = assign34560_e52078_d_n4;
        var_q_pexa_dn5 = assign34560_e52078_d_n5;
        var_q_pexa_db0 = assign34560_e52078_d_b0;
        var_q_pexa_db1 = assign34560_e52078_d_b1;
        var_q_pexa_db2 = assign34560_e52078_d_b2;
        var_q_pexa_db3 = assign34560_e52078_d_b3;

        let assign34570_e52081: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign34570_e52081;

        let (assign34580_e52091, assign34580_e52091_d_n0, assign34580_e52091_d_n1, assign34580_e52091_d_n2, assign34580_e52091_d_n3, assign34580_e52091_d_n4, assign34580_e52091_d_n5, assign34580_e52091_d_b0, assign34580_e52091_d_b1, assign34580_e52091_d_b2, assign34580_e52091_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34580_e52088: f64 = (1e-23 / var_q_pex0);
        let assign34580_e52089: f64 = (var_q_pexa * assign34580_e52088);
        (assign34580_e52089, (var_q_pexa_dn0 * assign34580_e52088), (var_q_pexa_dn1 * assign34580_e52088), (var_q_pexa_dn2 * assign34580_e52088), (var_q_pexa_dn3 * assign34580_e52088), (var_q_pexa_dn4 * assign34580_e52088), (var_q_pexa_dn5 * assign34580_e52088), (var_q_pexa_db0 * assign34580_e52088), (var_q_pexa_db1 * assign34580_e52088), (var_q_pexa_db2 * assign34580_e52088), (var_q_pexa_db3 * assign34580_e52088),)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn1, var_q_qs_a_dn2, var_q_qs_a_dn3, var_q_qs_a_dn4, var_q_qs_a_dn5, var_q_qs_a_db0, var_q_qs_a_db1, var_q_qs_a_db2, var_q_qs_a_db3,)
    }
};
        var_q_qs_a = assign34580_e52091;
        var_q_qs_a_dn0 = assign34580_e52091_d_n0;
        var_q_qs_a_dn1 = assign34580_e52091_d_n1;
        var_q_qs_a_dn2 = assign34580_e52091_d_n2;
        var_q_qs_a_dn3 = assign34580_e52091_d_n3;
        var_q_qs_a_dn4 = assign34580_e52091_d_n4;
        var_q_qs_a_dn5 = assign34580_e52091_d_n5;
        var_q_qs_a_db0 = assign34580_e52091_d_b0;
        var_q_qs_a_db1 = assign34580_e52091_d_b1;
        var_q_qs_a_db2 = assign34580_e52091_d_b2;
        var_q_qs_a_db3 = assign34580_e52091_d_b3;

        let (assign34590_e52099, assign34590_e52099_d_n0, assign34590_e52099_d_n1, assign34590_e52099_d_n2, assign34590_e52099_d_n3, assign34590_e52099_d_n4, assign34590_e52099_d_n5, assign34590_e52099_d_b0, assign34590_e52099_d_b1, assign34590_e52099_d_b2, assign34590_e52099_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34590_e52097: f64 = (nv3 - 0.0);
        (assign34590_e52097, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q_nqs_a, var_q_nqs_a_dn0, var_q_nqs_a_dn1, var_q_nqs_a_dn2, var_q_nqs_a_dn3, var_q_nqs_a_dn4, var_q_nqs_a_dn5, var_q_nqs_a_db0, var_q_nqs_a_db1, var_q_nqs_a_db2, var_q_nqs_a_db3,)
    }
};
        var_q_nqs_a = assign34590_e52099;
        var_q_nqs_a_dn0 = assign34590_e52099_d_n0;
        var_q_nqs_a_dn1 = assign34590_e52099_d_n1;
        var_q_nqs_a_dn2 = assign34590_e52099_d_n2;
        var_q_nqs_a_dn3 = assign34590_e52099_d_n3;
        var_q_nqs_a_dn4 = assign34590_e52099_d_n4;
        var_q_nqs_a_dn5 = assign34590_e52099_d_n5;
        var_q_nqs_a_db0 = assign34590_e52099_d_b0;
        var_q_nqs_a_db1 = assign34590_e52099_d_b1;
        var_q_nqs_a_db2 = assign34590_e52099_d_b2;
        var_q_nqs_a_db3 = assign34590_e52099_d_b3;

        let (assign34600_e52109, assign34600_e52109_d_n0, assign34600_e52109_d_n1, assign34600_e52109_d_n2, assign34600_e52109_d_n3, assign34600_e52109_d_n4, assign34600_e52109_d_n5, assign34600_e52109_d_b0, assign34600_e52109_d_b1, assign34600_e52109_d_b2, assign34600_e52109_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34600_e52105: f64 = (var_q_nqs_a - var_q_qs_a);
        let assign34600_e52107: f64 = (assign34600_e52105 / p.p92);
        (assign34600_e52107, ((var_q_nqs_a_dn0 - var_q_qs_a_dn0) / p.p92), ((var_q_nqs_a_dn1 - var_q_qs_a_dn1) / p.p92), ((var_q_nqs_a_dn2 - var_q_qs_a_dn2) / p.p92), ((var_q_nqs_a_dn3 - var_q_qs_a_dn3) / p.p92), ((var_q_nqs_a_dn4 - var_q_qs_a_dn4) / p.p92), ((var_q_nqs_a_dn5 - var_q_qs_a_dn5) / p.p92), ((var_q_nqs_a_db0 - var_q_qs_a_db0) / p.p92), ((var_q_nqs_a_db1 - var_q_qs_a_db1) / p.p92), ((var_q_nqs_a_db2 - var_q_qs_a_db2) / p.p92), ((var_q_nqs_a_db3 - var_q_qs_a_db3) / p.p92),)
    } else {
        (var_inqs0_a, var_inqs0_a_dn0, var_inqs0_a_dn1, var_inqs0_a_dn2, var_inqs0_a_dn3, var_inqs0_a_dn4, var_inqs0_a_dn5, var_inqs0_a_db0, var_inqs0_a_db1, var_inqs0_a_db2, var_inqs0_a_db3,)
    }
};
        var_inqs0_a = assign34600_e52109;
        var_inqs0_a_dn0 = assign34600_e52109_d_n0;
        var_inqs0_a_dn1 = assign34600_e52109_d_n1;
        var_inqs0_a_dn2 = assign34600_e52109_d_n2;
        var_inqs0_a_dn3 = assign34600_e52109_d_n3;
        var_inqs0_a_dn4 = assign34600_e52109_d_n4;
        var_inqs0_a_dn5 = assign34600_e52109_d_n5;
        var_inqs0_a_db0 = assign34600_e52109_d_b0;
        var_inqs0_a_db1 = assign34600_e52109_d_b1;
        var_inqs0_a_db2 = assign34600_e52109_d_b2;
        var_inqs0_a_db3 = assign34600_e52109_d_b3;

        let (assign34620_e52126, assign34620_e52126_d_n0, assign34620_e52126_d_n1, assign34620_e52126_d_n2, assign34620_e52126_d_n3, assign34620_e52126_d_n4, assign34620_e52126_d_n5, assign34620_e52126_d_b0, assign34620_e52126_d_b1, assign34620_e52126_d_b2, assign34620_e52126_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 == 0.0)) {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn1, var_q_pexa_dn2, var_q_pexa_dn3, var_q_pexa_dn4, var_q_pexa_dn5, var_q_pexa_db0, var_q_pexa_db1, var_q_pexa_db2, var_q_pexa_db3,)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn1, var_q_qs_a_dn2, var_q_qs_a_dn3, var_q_qs_a_dn4, var_q_qs_a_dn5, var_q_qs_a_db0, var_q_qs_a_db1, var_q_qs_a_db2, var_q_qs_a_db3,)
    }
};
        var_q_qs_a = assign34620_e52126;
        var_q_qs_a_dn0 = assign34620_e52126_d_n0;
        var_q_qs_a_dn1 = assign34620_e52126_d_n1;
        var_q_qs_a_dn2 = assign34620_e52126_d_n2;
        var_q_qs_a_dn3 = assign34620_e52126_d_n3;
        var_q_qs_a_dn4 = assign34620_e52126_d_n4;
        var_q_qs_a_dn5 = assign34620_e52126_d_n5;
        var_q_qs_a_db0 = assign34620_e52126_d_b0;
        var_q_qs_a_db1 = assign34620_e52126_d_b1;
        var_q_qs_a_db2 = assign34620_e52126_d_b2;
        var_q_qs_a_db3 = assign34620_e52126_d_b3;

        let assign34640_e52140: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_hk)) { 1.0 } else { 0.0 };
        var_guard565 = assign34640_e52140;

        let (assign34650_e52148, assign34650_e52148_d_n0, assign34650_e52148_d_n1, assign34650_e52148_d_n2, assign34650_e52148_d_n3, assign34650_e52148_d_n4, assign34650_e52148_d_n5, assign34650_e52148_d_b0, assign34650_e52148_d_b1, assign34650_e52148_d_b2, assign34650_e52148_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard565 != 0.0)) {
        let assign34650_e52146: f64 = (var_exp_k * p.p90);
        (assign34650_e52146, (var_exp_k_dn0 * p.p90), (var_exp_k_dn1 * p.p90), (var_exp_k_dn2 * p.p90), (var_exp_k_dn3 * p.p90), (var_exp_k_dn4 * p.p90), (var_exp_k_dn5 * p.p90), (var_exp_k_db0 * p.p90), (var_exp_k_db1 * p.p90), (var_exp_k_db2 * p.p90), (var_exp_k_db3 * p.p90),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34650_e52148;
        var_exp_k2_dn0 = assign34650_e52148_d_n0;
        var_exp_k2_dn1 = assign34650_e52148_d_n1;
        var_exp_k2_dn2 = assign34650_e52148_d_n2;
        var_exp_k2_dn3 = assign34650_e52148_d_n3;
        var_exp_k2_dn4 = assign34650_e52148_d_n4;
        var_exp_k2_dn5 = assign34650_e52148_d_n5;
        var_exp_k2_db0 = assign34650_e52148_d_b0;
        var_exp_k2_db1 = assign34650_e52148_d_b1;
        var_exp_k2_db2 = assign34650_e52148_d_b2;
        var_exp_k2_db3 = assign34650_e52148_d_b3;

        let (assign34660_e52177, assign34660_e52177_d_n0, assign34660_e52177_d_n1, assign34660_e52177_d_n2, assign34660_e52177_d_n3, assign34660_e52177_d_n4, assign34660_e52177_d_n5, assign34660_e52177_d_b0, assign34660_e52177_d_b1, assign34660_e52177_d_b2, assign34660_e52177_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard565 == 0.0)) {
        let assign34660_e52155: f64 = (var_exp_k * p.p90);
        let assign34660_e52157: f64 = (-p.p91);
        let assign34660_e52160: f64 = (var_vak - var_v_hk);
        let assign34660_e52161: f64 = (assign34660_e52157 * assign34660_e52160);
        let assign34660_e52164: f64 = (var_vak - var_v_hk);
        let assign34660_e52165: f64 = (assign34660_e52161 * assign34660_e52164);
        let assign34660_e52169: f64 = (var_tkr / var_tkd);
        let assign34660_e52170: f64 = (assign34660_e52169).ln();
        let assign34660_e52171: f64 = (p.p98 * assign34660_e52170);
        let assign34660_e52172: f64 = (assign34660_e52171).exp();
        let assign34660_e52173: f64 = (assign34660_e52165 * assign34660_e52172);
        let assign34660_e52174: f64 = (assign34660_e52173).exp();
        let assign34660_e52175: f64 = (assign34660_e52155 * assign34660_e52174);
        (assign34660_e52175, (((var_exp_k_dn0 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn0) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn0)) * assign34660_e52172)))), (((var_exp_k_dn1 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn1) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn1)) * assign34660_e52172)))), (((var_exp_k_dn2 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn2) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn2)) * assign34660_e52172)))), (((var_exp_k_dn3 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn3) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn3)) * assign34660_e52172)))), (((var_exp_k_dn4 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn4) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn4)) * assign34660_e52172)))), (((var_exp_k_dn5 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn5) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn5)) * assign34660_e52172)))), (((var_exp_k_db0 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_db0) * assign34660_e52164) + (assign34660_e52161 * var_vak_db0)) * assign34660_e52172)))), (((var_exp_k_db1 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_db1) * assign34660_e52164) + (assign34660_e52161 * var_vak_db1)) * assign34660_e52172)))), (((var_exp_k_db2 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_db2) * assign34660_e52164) + (assign34660_e52161 * var_vak_db2)) * assign34660_e52172)))), (((var_exp_k_db3 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_db3) * assign34660_e52164) + (assign34660_e52161 * var_vak_db3)) * assign34660_e52172)))),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34660_e52177;
        var_exp_k2_dn0 = assign34660_e52177_d_n0;
        var_exp_k2_dn1 = assign34660_e52177_d_n1;
        var_exp_k2_dn2 = assign34660_e52177_d_n2;
        var_exp_k2_dn3 = assign34660_e52177_d_n3;
        var_exp_k2_dn4 = assign34660_e52177_d_n4;
        var_exp_k2_dn5 = assign34660_e52177_d_n5;
        var_exp_k2_db0 = assign34660_e52177_d_b0;
        var_exp_k2_db1 = assign34660_e52177_d_b1;
        var_exp_k2_db2 = assign34660_e52177_d_b2;
        var_exp_k2_db3 = assign34660_e52177_d_b3;

        let (assign34670_e52186, assign34670_e52186_d_n0, assign34670_e52186_d_n1, assign34670_e52186_d_n2, assign34670_e52186_d_n3, assign34670_e52186_d_n4, assign34670_e52186_d_n5, assign34670_e52186_d_b0, assign34670_e52186_d_b1, assign34670_e52186_d_b2, assign34670_e52186_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n1, assign34670_e52184_d_n2, assign34670_e52184_d_n3, assign34670_e52184_d_n4, assign34670_e52184_d_n5, assign34670_e52184_d_b0, assign34670_e52184_d_b1, assign34670_e52184_d_b2, assign34670_e52184_d_b3,) = {
            if (var_exp_k2 > p.p79) {
                (p.p79, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
            }
        };
        (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n1, assign34670_e52184_d_n2, assign34670_e52184_d_n3, assign34670_e52184_d_n4, assign34670_e52184_d_n5, assign34670_e52184_d_b0, assign34670_e52184_d_b1, assign34670_e52184_d_b2, assign34670_e52184_d_b3,)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn1, var_exp_k2_dn2, var_exp_k2_dn3, var_exp_k2_dn4, var_exp_k2_dn5, var_exp_k2_db0, var_exp_k2_db1, var_exp_k2_db2, var_exp_k2_db3,)
    }
};
        var_exp_k2 = assign34670_e52186;
        var_exp_k2_dn0 = assign34670_e52186_d_n0;
        var_exp_k2_dn1 = assign34670_e52186_d_n1;
        var_exp_k2_dn2 = assign34670_e52186_d_n2;
        var_exp_k2_dn3 = assign34670_e52186_d_n3;
        var_exp_k2_dn4 = assign34670_e52186_d_n4;
        var_exp_k2_dn5 = assign34670_e52186_d_n5;
        var_exp_k2_db0 = assign34670_e52186_d_b0;
        var_exp_k2_db1 = assign34670_e52186_d_b1;
        var_exp_k2_db2 = assign34670_e52186_d_b2;
        var_exp_k2_db3 = assign34670_e52186_d_b3;

        let (assign34680_e52192, assign34680_e52192_d_n0, assign34680_e52192_d_n1, assign34680_e52192_d_n2, assign34680_e52192_d_n3, assign34680_e52192_d_n4, assign34680_e52192_d_n5, assign34680_e52192_d_b0, assign34680_e52192_d_b1, assign34680_e52192_d_b2, assign34680_e52192_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34680_e52190: f64 = (var_pn0 * var_exp_k2);
        (assign34680_e52190, (var_pn0 * var_exp_k2_dn0), (var_pn0 * var_exp_k2_dn1), (var_pn0 * var_exp_k2_dn2), (var_pn0 * var_exp_k2_dn3), (var_pn0 * var_exp_k2_dn4), (var_pn0 * var_exp_k2_dn5), (var_pn0 * var_exp_k2_db0), (var_pn0 * var_exp_k2_db1), (var_pn0 * var_exp_k2_db2), (var_pn0 * var_exp_k2_db3),)
    } else {
        (var_p_nk, var_p_nk_dn0, var_p_nk_dn1, var_p_nk_dn2, var_p_nk_dn3, var_p_nk_dn4, var_p_nk_dn5, var_p_nk_db0, var_p_nk_db1, var_p_nk_db2, var_p_nk_db3,)
    }
};
        var_p_nk = assign34680_e52192;
        var_p_nk_dn0 = assign34680_e52192_d_n0;
        var_p_nk_dn1 = assign34680_e52192_d_n1;
        var_p_nk_dn2 = assign34680_e52192_d_n2;
        var_p_nk_dn3 = assign34680_e52192_d_n3;
        var_p_nk_dn4 = assign34680_e52192_d_n4;
        var_p_nk_dn5 = assign34680_e52192_d_n5;
        var_p_nk_db0 = assign34680_e52192_d_b0;
        var_p_nk_db1 = assign34680_e52192_d_b1;
        var_p_nk_db2 = assign34680_e52192_d_b2;
        var_p_nk_db3 = assign34680_e52192_d_b3;

        let (assign34690_e52202, assign34690_e52202_d_n0, assign34690_e52202_d_n1, assign34690_e52202_d_n2, assign34690_e52202_d_n3, assign34690_e52202_d_n4, assign34690_e52202_d_n5, assign34690_e52202_d_b0, assign34690_e52202_d_b1, assign34690_e52202_d_b2, assign34690_e52202_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34690_e52196: f64 = (1.6021918e-19 * var_ab_i);
        let assign34690_e52199: f64 = (var_p_nk - var_pn0);
        let assign34690_e52200: f64 = (assign34690_e52196 * assign34690_e52199);
        (assign34690_e52200, (assign34690_e52196 * var_p_nk_dn0), (assign34690_e52196 * var_p_nk_dn1), (assign34690_e52196 * var_p_nk_dn2), (assign34690_e52196 * var_p_nk_dn3), (assign34690_e52196 * var_p_nk_dn4), (assign34690_e52196 * var_p_nk_dn5), (assign34690_e52196 * var_p_nk_db0), (assign34690_e52196 * var_p_nk_db1), (assign34690_e52196 * var_p_nk_db2), (assign34690_e52196 * var_p_nk_db3),)
    } else {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn1, var_q_pexk_dn2, var_q_pexk_dn3, var_q_pexk_dn4, var_q_pexk_dn5, var_q_pexk_db0, var_q_pexk_db1, var_q_pexk_db2, var_q_pexk_db3,)
    }
};
        var_q_pexk = assign34690_e52202;
        var_q_pexk_dn0 = assign34690_e52202_d_n0;
        var_q_pexk_dn1 = assign34690_e52202_d_n1;
        var_q_pexk_dn2 = assign34690_e52202_d_n2;
        var_q_pexk_dn3 = assign34690_e52202_d_n3;
        var_q_pexk_dn4 = assign34690_e52202_d_n4;
        var_q_pexk_dn5 = assign34690_e52202_d_n5;
        var_q_pexk_db0 = assign34690_e52202_d_b0;
        var_q_pexk_db1 = assign34690_e52202_d_b1;
        var_q_pexk_db2 = assign34690_e52202_d_b2;
        var_q_pexk_db3 = assign34690_e52202_d_b3;

        let assign34700_e52205: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign34700_e52205;

        let (assign34710_e52215, assign34710_e52215_d_n0, assign34710_e52215_d_n1, assign34710_e52215_d_n2, assign34710_e52215_d_n3, assign34710_e52215_d_n4, assign34710_e52215_d_n5, assign34710_e52215_d_b0, assign34710_e52215_d_b1, assign34710_e52215_d_b2, assign34710_e52215_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34710_e52212: f64 = (1e-23 / var_q_pex0);
        let assign34710_e52213: f64 = (var_q_pexk * assign34710_e52212);
        (assign34710_e52213, (var_q_pexk_dn0 * assign34710_e52212), (var_q_pexk_dn1 * assign34710_e52212), (var_q_pexk_dn2 * assign34710_e52212), (var_q_pexk_dn3 * assign34710_e52212), (var_q_pexk_dn4 * assign34710_e52212), (var_q_pexk_dn5 * assign34710_e52212), (var_q_pexk_db0 * assign34710_e52212), (var_q_pexk_db1 * assign34710_e52212), (var_q_pexk_db2 * assign34710_e52212), (var_q_pexk_db3 * assign34710_e52212),)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn1, var_q_qs_k_dn2, var_q_qs_k_dn3, var_q_qs_k_dn4, var_q_qs_k_dn5, var_q_qs_k_db0, var_q_qs_k_db1, var_q_qs_k_db2, var_q_qs_k_db3,)
    }
};
        var_q_qs_k = assign34710_e52215;
        var_q_qs_k_dn0 = assign34710_e52215_d_n0;
        var_q_qs_k_dn1 = assign34710_e52215_d_n1;
        var_q_qs_k_dn2 = assign34710_e52215_d_n2;
        var_q_qs_k_dn3 = assign34710_e52215_d_n3;
        var_q_qs_k_dn4 = assign34710_e52215_d_n4;
        var_q_qs_k_dn5 = assign34710_e52215_d_n5;
        var_q_qs_k_db0 = assign34710_e52215_d_b0;
        var_q_qs_k_db1 = assign34710_e52215_d_b1;
        var_q_qs_k_db2 = assign34710_e52215_d_b2;
        var_q_qs_k_db3 = assign34710_e52215_d_b3;

        let (assign34720_e52223, assign34720_e52223_d_n0, assign34720_e52223_d_n1, assign34720_e52223_d_n2, assign34720_e52223_d_n3, assign34720_e52223_d_n4, assign34720_e52223_d_n5, assign34720_e52223_d_b0, assign34720_e52223_d_b1, assign34720_e52223_d_b2, assign34720_e52223_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34720_e52221: f64 = (nv4 - 0.0);
        (assign34720_e52221, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q_nqs_k, var_q_nqs_k_dn0, var_q_nqs_k_dn1, var_q_nqs_k_dn2, var_q_nqs_k_dn3, var_q_nqs_k_dn4, var_q_nqs_k_dn5, var_q_nqs_k_db0, var_q_nqs_k_db1, var_q_nqs_k_db2, var_q_nqs_k_db3,)
    }
};
        var_q_nqs_k = assign34720_e52223;
        var_q_nqs_k_dn0 = assign34720_e52223_d_n0;
        var_q_nqs_k_dn1 = assign34720_e52223_d_n1;
        var_q_nqs_k_dn2 = assign34720_e52223_d_n2;
        var_q_nqs_k_dn3 = assign34720_e52223_d_n3;
        var_q_nqs_k_dn4 = assign34720_e52223_d_n4;
        var_q_nqs_k_dn5 = assign34720_e52223_d_n5;
        var_q_nqs_k_db0 = assign34720_e52223_d_b0;
        var_q_nqs_k_db1 = assign34720_e52223_d_b1;
        var_q_nqs_k_db2 = assign34720_e52223_d_b2;
        var_q_nqs_k_db3 = assign34720_e52223_d_b3;

        let (assign34730_e52233, assign34730_e52233_d_n0, assign34730_e52233_d_n1, assign34730_e52233_d_n2, assign34730_e52233_d_n3, assign34730_e52233_d_n4, assign34730_e52233_d_n5, assign34730_e52233_d_b0, assign34730_e52233_d_b1, assign34730_e52233_d_b2, assign34730_e52233_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34730_e52229: f64 = (var_q_nqs_k - var_q_qs_k);
        let assign34730_e52231: f64 = (assign34730_e52229 / p.p92);
        (assign34730_e52231, ((var_q_nqs_k_dn0 - var_q_qs_k_dn0) / p.p92), ((var_q_nqs_k_dn1 - var_q_qs_k_dn1) / p.p92), ((var_q_nqs_k_dn2 - var_q_qs_k_dn2) / p.p92), ((var_q_nqs_k_dn3 - var_q_qs_k_dn3) / p.p92), ((var_q_nqs_k_dn4 - var_q_qs_k_dn4) / p.p92), ((var_q_nqs_k_dn5 - var_q_qs_k_dn5) / p.p92), ((var_q_nqs_k_db0 - var_q_qs_k_db0) / p.p92), ((var_q_nqs_k_db1 - var_q_qs_k_db1) / p.p92), ((var_q_nqs_k_db2 - var_q_qs_k_db2) / p.p92), ((var_q_nqs_k_db3 - var_q_qs_k_db3) / p.p92),)
    } else {
        (var_inqs0_k, var_inqs0_k_dn0, var_inqs0_k_dn1, var_inqs0_k_dn2, var_inqs0_k_dn3, var_inqs0_k_dn4, var_inqs0_k_dn5, var_inqs0_k_db0, var_inqs0_k_db1, var_inqs0_k_db2, var_inqs0_k_db3,)
    }
};
        var_inqs0_k = assign34730_e52233;
        var_inqs0_k_dn0 = assign34730_e52233_d_n0;
        var_inqs0_k_dn1 = assign34730_e52233_d_n1;
        var_inqs0_k_dn2 = assign34730_e52233_d_n2;
        var_inqs0_k_dn3 = assign34730_e52233_d_n3;
        var_inqs0_k_dn4 = assign34730_e52233_d_n4;
        var_inqs0_k_dn5 = assign34730_e52233_d_n5;
        var_inqs0_k_db0 = assign34730_e52233_d_b0;
        var_inqs0_k_db1 = assign34730_e52233_d_b1;
        var_inqs0_k_db2 = assign34730_e52233_d_b2;
        var_inqs0_k_db3 = assign34730_e52233_d_b3;

        let (assign34750_e52250, assign34750_e52250_d_n0, assign34750_e52250_d_n1, assign34750_e52250_d_n2, assign34750_e52250_d_n3, assign34750_e52250_d_n4, assign34750_e52250_d_n5, assign34750_e52250_d_b0, assign34750_e52250_d_b1, assign34750_e52250_d_b2, assign34750_e52250_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard566 == 0.0)) {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn1, var_q_pexk_dn2, var_q_pexk_dn3, var_q_pexk_dn4, var_q_pexk_dn5, var_q_pexk_db0, var_q_pexk_db1, var_q_pexk_db2, var_q_pexk_db3,)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn1, var_q_qs_k_dn2, var_q_qs_k_dn3, var_q_qs_k_dn4, var_q_qs_k_dn5, var_q_qs_k_db0, var_q_qs_k_db1, var_q_qs_k_db2, var_q_qs_k_db3,)
    }
};
        var_q_qs_k = assign34750_e52250;
        var_q_qs_k_dn0 = assign34750_e52250_d_n0;
        var_q_qs_k_dn1 = assign34750_e52250_d_n1;
        var_q_qs_k_dn2 = assign34750_e52250_d_n2;
        var_q_qs_k_dn3 = assign34750_e52250_d_n3;
        var_q_qs_k_dn4 = assign34750_e52250_d_n4;
        var_q_qs_k_dn5 = assign34750_e52250_d_n5;
        var_q_qs_k_db0 = assign34750_e52250_d_b0;
        var_q_qs_k_db1 = assign34750_e52250_d_b1;
        var_q_qs_k_db2 = assign34750_e52250_d_b2;
        var_q_qs_k_db3 = assign34750_e52250_d_b3;


        *var_exp_a2_slot = var_exp_a2;
        *var_exp_a2_db0_slot = var_exp_a2_db0;
        *var_exp_a2_db1_slot = var_exp_a2_db1;
        *var_exp_a2_db2_slot = var_exp_a2_db2;
        *var_exp_a2_db3_slot = var_exp_a2_db3;
        *var_exp_a2_dn0_slot = var_exp_a2_dn0;
        *var_exp_a2_dn1_slot = var_exp_a2_dn1;
        *var_exp_a2_dn2_slot = var_exp_a2_dn2;
        *var_exp_a2_dn3_slot = var_exp_a2_dn3;
        *var_exp_a2_dn4_slot = var_exp_a2_dn4;
        *var_exp_a2_dn5_slot = var_exp_a2_dn5;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k2_slot = var_exp_k2;
        *var_exp_k2_db0_slot = var_exp_k2_db0;
        *var_exp_k2_db1_slot = var_exp_k2_db1;
        *var_exp_k2_db2_slot = var_exp_k2_db2;
        *var_exp_k2_db3_slot = var_exp_k2_db3;
        *var_exp_k2_dn0_slot = var_exp_k2_dn0;
        *var_exp_k2_dn1_slot = var_exp_k2_dn1;
        *var_exp_k2_dn2_slot = var_exp_k2_dn2;
        *var_exp_k2_dn3_slot = var_exp_k2_dn3;
        *var_exp_k2_dn4_slot = var_exp_k2_dn4;
        *var_exp_k2_dn5_slot = var_exp_k2_dn5;
        *var_exp_k_db0_slot = var_exp_k_db0;
        *var_exp_k_db1_slot = var_exp_k_db1;
        *var_exp_k_db2_slot = var_exp_k_db2;
        *var_exp_k_db3_slot = var_exp_k_db3;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn1_slot = var_exp_k_dn1;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_exp_k_dn3_slot = var_exp_k_dn3;
        *var_exp_k_dn4_slot = var_exp_k_dn4;
        *var_exp_k_dn5_slot = var_exp_k_dn5;
        *var_guard563_slot = var_guard563;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
        *var_guard566_slot = var_guard566;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_db0_slot = var_inqs0_a_db0;
        *var_inqs0_a_db1_slot = var_inqs0_a_db1;
        *var_inqs0_a_db2_slot = var_inqs0_a_db2;
        *var_inqs0_a_db3_slot = var_inqs0_a_db3;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn1_slot = var_inqs0_a_dn1;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn3_slot = var_inqs0_a_dn3;
        *var_inqs0_a_dn4_slot = var_inqs0_a_dn4;
        *var_inqs0_a_dn5_slot = var_inqs0_a_dn5;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_db0_slot = var_inqs0_k_db0;
        *var_inqs0_k_db1_slot = var_inqs0_k_db1;
        *var_inqs0_k_db2_slot = var_inqs0_k_db2;
        *var_inqs0_k_db3_slot = var_inqs0_k_db3;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn1_slot = var_inqs0_k_dn1;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn3_slot = var_inqs0_k_dn3;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_inqs0_k_dn5_slot = var_inqs0_k_dn5;
        *var_p_na_slot = var_p_na;
        *var_p_na_db0_slot = var_p_na_db0;
        *var_p_na_db1_slot = var_p_na_db1;
        *var_p_na_db2_slot = var_p_na_db2;
        *var_p_na_db3_slot = var_p_na_db3;
        *var_p_na_dn0_slot = var_p_na_dn0;
        *var_p_na_dn1_slot = var_p_na_dn1;
        *var_p_na_dn2_slot = var_p_na_dn2;
        *var_p_na_dn3_slot = var_p_na_dn3;
        *var_p_na_dn4_slot = var_p_na_dn4;
        *var_p_na_dn5_slot = var_p_na_dn5;
        *var_p_nk_slot = var_p_nk;
        *var_p_nk_db0_slot = var_p_nk_db0;
        *var_p_nk_db1_slot = var_p_nk_db1;
        *var_p_nk_db2_slot = var_p_nk_db2;
        *var_p_nk_db3_slot = var_p_nk_db3;
        *var_p_nk_dn0_slot = var_p_nk_dn0;
        *var_p_nk_dn1_slot = var_p_nk_dn1;
        *var_p_nk_dn2_slot = var_p_nk_dn2;
        *var_p_nk_dn3_slot = var_p_nk_dn3;
        *var_p_nk_dn4_slot = var_p_nk_dn4;
        *var_p_nk_dn5_slot = var_p_nk_dn5;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_db0_slot = var_q_nqs_a_db0;
        *var_q_nqs_a_db1_slot = var_q_nqs_a_db1;
        *var_q_nqs_a_db2_slot = var_q_nqs_a_db2;
        *var_q_nqs_a_db3_slot = var_q_nqs_a_db3;
        *var_q_nqs_a_dn0_slot = var_q_nqs_a_dn0;
        *var_q_nqs_a_dn1_slot = var_q_nqs_a_dn1;
        *var_q_nqs_a_dn2_slot = var_q_nqs_a_dn2;
        *var_q_nqs_a_dn3_slot = var_q_nqs_a_dn3;
        *var_q_nqs_a_dn4_slot = var_q_nqs_a_dn4;
        *var_q_nqs_a_dn5_slot = var_q_nqs_a_dn5;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_db0_slot = var_q_nqs_k_db0;
        *var_q_nqs_k_db1_slot = var_q_nqs_k_db1;
        *var_q_nqs_k_db2_slot = var_q_nqs_k_db2;
        *var_q_nqs_k_db3_slot = var_q_nqs_k_db3;
        *var_q_nqs_k_dn0_slot = var_q_nqs_k_dn0;
        *var_q_nqs_k_dn1_slot = var_q_nqs_k_dn1;
        *var_q_nqs_k_dn2_slot = var_q_nqs_k_dn2;
        *var_q_nqs_k_dn3_slot = var_q_nqs_k_dn3;
        *var_q_nqs_k_dn4_slot = var_q_nqs_k_dn4;
        *var_q_nqs_k_dn5_slot = var_q_nqs_k_dn5;
        *var_q_pexa_slot = var_q_pexa;
        *var_q_pexa_db0_slot = var_q_pexa_db0;
        *var_q_pexa_db1_slot = var_q_pexa_db1;
        *var_q_pexa_db2_slot = var_q_pexa_db2;
        *var_q_pexa_db3_slot = var_q_pexa_db3;
        *var_q_pexa_dn0_slot = var_q_pexa_dn0;
        *var_q_pexa_dn1_slot = var_q_pexa_dn1;
        *var_q_pexa_dn2_slot = var_q_pexa_dn2;
        *var_q_pexa_dn3_slot = var_q_pexa_dn3;
        *var_q_pexa_dn4_slot = var_q_pexa_dn4;
        *var_q_pexa_dn5_slot = var_q_pexa_dn5;
        *var_q_pexk_slot = var_q_pexk;
        *var_q_pexk_db0_slot = var_q_pexk_db0;
        *var_q_pexk_db1_slot = var_q_pexk_db1;
        *var_q_pexk_db2_slot = var_q_pexk_db2;
        *var_q_pexk_db3_slot = var_q_pexk_db3;
        *var_q_pexk_dn0_slot = var_q_pexk_dn0;
        *var_q_pexk_dn1_slot = var_q_pexk_dn1;
        *var_q_pexk_dn2_slot = var_q_pexk_dn2;
        *var_q_pexk_dn3_slot = var_q_pexk_dn3;
        *var_q_pexk_dn4_slot = var_q_pexk_dn4;
        *var_q_pexk_dn5_slot = var_q_pexk_dn5;
        *var_q_qs_a_slot = var_q_qs_a;
        *var_q_qs_a_db0_slot = var_q_qs_a_db0;
        *var_q_qs_a_db1_slot = var_q_qs_a_db1;
        *var_q_qs_a_db2_slot = var_q_qs_a_db2;
        *var_q_qs_a_db3_slot = var_q_qs_a_db3;
        *var_q_qs_a_dn0_slot = var_q_qs_a_dn0;
        *var_q_qs_a_dn1_slot = var_q_qs_a_dn1;
        *var_q_qs_a_dn2_slot = var_q_qs_a_dn2;
        *var_q_qs_a_dn3_slot = var_q_qs_a_dn3;
        *var_q_qs_a_dn4_slot = var_q_qs_a_dn4;
        *var_q_qs_a_dn5_slot = var_q_qs_a_dn5;
        *var_q_qs_k_slot = var_q_qs_k;
        *var_q_qs_k_db0_slot = var_q_qs_k_db0;
        *var_q_qs_k_db1_slot = var_q_qs_k_db1;
        *var_q_qs_k_db2_slot = var_q_qs_k_db2;
        *var_q_qs_k_db3_slot = var_q_qs_k_db3;
        *var_q_qs_k_dn0_slot = var_q_qs_k_dn0;
        *var_q_qs_k_dn1_slot = var_q_qs_k_dn1;
        *var_q_qs_k_dn2_slot = var_q_qs_k_dn2;
        *var_q_qs_k_dn3_slot = var_q_qs_k_dn3;
        *var_q_qs_k_dn4_slot = var_q_qs_k_dn4;
        *var_q_qs_k_dn5_slot = var_q_qs_k_dn5;
    }

    pub(super) fn stamp_transient_block_53(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_epssi: f64,
        var_guard558: f64,
        var_juncdlt: f64,
        var_ndi_i: f64,
        var_pb: f64,
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
        var_w_depa0: f64,
        var_w_depa0_db0: f64,
        var_w_depa0_db1: f64,
        var_w_depa0_db2: f64,
        var_w_depa0_db3: f64,
        var_w_depa0_dn0: f64,
        var_w_depa0_dn1: f64,
        var_w_depa0_dn2: f64,
        var_w_depa0_dn3: f64,
        var_w_depa0_dn4: f64,
        var_w_depa0_dn5: f64,
        var_guard567_slot: &mut f64,
        var_guard568_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_db0_slot: &mut f64,
        var_iwnqs0_a_db1_slot: &mut f64,
        var_iwnqs0_a_db2_slot: &mut f64,
        var_iwnqs0_a_db3_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn1_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn3_slot: &mut f64,
        var_iwnqs0_a_dn4_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
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
        var_vjunc_a_slot: &mut f64,
        var_vjunc_a_db0_slot: &mut f64,
        var_vjunc_a_db1_slot: &mut f64,
        var_vjunc_a_db2_slot: &mut f64,
        var_vjunc_a_db3_slot: &mut f64,
        var_vjunc_a_dn0_slot: &mut f64,
        var_vjunc_a_dn1_slot: &mut f64,
        var_vjunc_a_dn2_slot: &mut f64,
        var_vjunc_a_dn3_slot: &mut f64,
        var_vjunc_a_dn4_slot: &mut f64,
        var_vjunc_a_dn5_slot: &mut f64,
        var_w_depa_slot: &mut f64,
        var_w_depa_db0_slot: &mut f64,
        var_w_depa_db1_slot: &mut f64,
        var_w_depa_db2_slot: &mut f64,
        var_w_depa_db3_slot: &mut f64,
        var_w_depa_dn0_slot: &mut f64,
        var_w_depa_dn1_slot: &mut f64,
        var_w_depa_dn2_slot: &mut f64,
        var_w_depa_dn3_slot: &mut f64,
        var_w_depa_dn4_slot: &mut f64,
        var_w_depa_dn5_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_db0_slot: &mut f64,
        var_w_nqs_a_db1_slot: &mut f64,
        var_w_nqs_a_db2_slot: &mut f64,
        var_w_nqs_a_db3_slot: &mut f64,
        var_w_nqs_a_dn0_slot: &mut f64,
        var_w_nqs_a_dn1_slot: &mut f64,
        var_w_nqs_a_dn2_slot: &mut f64,
        var_w_nqs_a_dn3_slot: &mut f64,
        var_w_nqs_a_dn4_slot: &mut f64,
        var_w_nqs_a_dn5_slot: &mut f64,
        var_w_qs_a_slot: &mut f64,
        var_w_qs_a_db0_slot: &mut f64,
        var_w_qs_a_db1_slot: &mut f64,
        var_w_qs_a_db2_slot: &mut f64,
        var_w_qs_a_db3_slot: &mut f64,
        var_w_qs_a_dn0_slot: &mut f64,
        var_w_qs_a_dn1_slot: &mut f64,
        var_w_qs_a_dn2_slot: &mut f64,
        var_w_qs_a_dn3_slot: &mut f64,
        var_w_qs_a_dn4_slot: &mut f64,
        var_w_qs_a_dn5_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_db0: f64 = *var_iwnqs0_a_db0_slot;
        let mut var_iwnqs0_a_db1: f64 = *var_iwnqs0_a_db1_slot;
        let mut var_iwnqs0_a_db2: f64 = *var_iwnqs0_a_db2_slot;
        let mut var_iwnqs0_a_db3: f64 = *var_iwnqs0_a_db3_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn1: f64 = *var_iwnqs0_a_dn1_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn3: f64 = *var_iwnqs0_a_dn3_slot;
        let mut var_iwnqs0_a_dn4: f64 = *var_iwnqs0_a_dn4_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
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
        let mut var_vjunc_a: f64 = *var_vjunc_a_slot;
        let mut var_vjunc_a_db0: f64 = *var_vjunc_a_db0_slot;
        let mut var_vjunc_a_db1: f64 = *var_vjunc_a_db1_slot;
        let mut var_vjunc_a_db2: f64 = *var_vjunc_a_db2_slot;
        let mut var_vjunc_a_db3: f64 = *var_vjunc_a_db3_slot;
        let mut var_vjunc_a_dn0: f64 = *var_vjunc_a_dn0_slot;
        let mut var_vjunc_a_dn1: f64 = *var_vjunc_a_dn1_slot;
        let mut var_vjunc_a_dn2: f64 = *var_vjunc_a_dn2_slot;
        let mut var_vjunc_a_dn3: f64 = *var_vjunc_a_dn3_slot;
        let mut var_vjunc_a_dn4: f64 = *var_vjunc_a_dn4_slot;
        let mut var_vjunc_a_dn5: f64 = *var_vjunc_a_dn5_slot;
        let mut var_w_depa: f64 = *var_w_depa_slot;
        let mut var_w_depa_db0: f64 = *var_w_depa_db0_slot;
        let mut var_w_depa_db1: f64 = *var_w_depa_db1_slot;
        let mut var_w_depa_db2: f64 = *var_w_depa_db2_slot;
        let mut var_w_depa_db3: f64 = *var_w_depa_db3_slot;
        let mut var_w_depa_dn0: f64 = *var_w_depa_dn0_slot;
        let mut var_w_depa_dn1: f64 = *var_w_depa_dn1_slot;
        let mut var_w_depa_dn2: f64 = *var_w_depa_dn2_slot;
        let mut var_w_depa_dn3: f64 = *var_w_depa_dn3_slot;
        let mut var_w_depa_dn4: f64 = *var_w_depa_dn4_slot;
        let mut var_w_depa_dn5: f64 = *var_w_depa_dn5_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_db0: f64 = *var_w_nqs_a_db0_slot;
        let mut var_w_nqs_a_db1: f64 = *var_w_nqs_a_db1_slot;
        let mut var_w_nqs_a_db2: f64 = *var_w_nqs_a_db2_slot;
        let mut var_w_nqs_a_db3: f64 = *var_w_nqs_a_db3_slot;
        let mut var_w_nqs_a_dn0: f64 = *var_w_nqs_a_dn0_slot;
        let mut var_w_nqs_a_dn1: f64 = *var_w_nqs_a_dn1_slot;
        let mut var_w_nqs_a_dn2: f64 = *var_w_nqs_a_dn2_slot;
        let mut var_w_nqs_a_dn3: f64 = *var_w_nqs_a_dn3_slot;
        let mut var_w_nqs_a_dn4: f64 = *var_w_nqs_a_dn4_slot;
        let mut var_w_nqs_a_dn5: f64 = *var_w_nqs_a_dn5_slot;
        let mut var_w_qs_a: f64 = *var_w_qs_a_slot;
        let mut var_w_qs_a_db0: f64 = *var_w_qs_a_db0_slot;
        let mut var_w_qs_a_db1: f64 = *var_w_qs_a_db1_slot;
        let mut var_w_qs_a_db2: f64 = *var_w_qs_a_db2_slot;
        let mut var_w_qs_a_db3: f64 = *var_w_qs_a_db3_slot;
        let mut var_w_qs_a_dn0: f64 = *var_w_qs_a_dn0_slot;
        let mut var_w_qs_a_dn1: f64 = *var_w_qs_a_dn1_slot;
        let mut var_w_qs_a_dn2: f64 = *var_w_qs_a_dn2_slot;
        let mut var_w_qs_a_dn3: f64 = *var_w_qs_a_dn3_slot;
        let mut var_w_qs_a_dn4: f64 = *var_w_qs_a_dn4_slot;
        let mut var_w_qs_a_dn5: f64 = *var_w_qs_a_dn5_slot;

        let (assign34770_e52263, assign34770_e52263_d_n0, assign34770_e52263_d_n1, assign34770_e52263_d_n2, assign34770_e52263_d_n3, assign34770_e52263_d_n4, assign34770_e52263_d_n5, assign34770_e52263_d_b0, assign34770_e52263_d_b1, assign34770_e52263_d_b2, assign34770_e52263_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34770_e52261: f64 = (var_pb - var_vak);
        (assign34770_e52261, (-var_vak_dn0), (-var_vak_dn1), (-var_vak_dn2), (-var_vak_dn3), (-var_vak_dn4), (-var_vak_dn5), (-var_vak_db0), (-var_vak_db1), (-var_vak_db2), (-var_vak_db3),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34770_e52263;
        var_vjunc_a_dn0 = assign34770_e52263_d_n0;
        var_vjunc_a_dn1 = assign34770_e52263_d_n1;
        var_vjunc_a_dn2 = assign34770_e52263_d_n2;
        var_vjunc_a_dn3 = assign34770_e52263_d_n3;
        var_vjunc_a_dn4 = assign34770_e52263_d_n4;
        var_vjunc_a_dn5 = assign34770_e52263_d_n5;
        var_vjunc_a_db0 = assign34770_e52263_d_b0;
        var_vjunc_a_db1 = assign34770_e52263_d_b1;
        var_vjunc_a_db2 = assign34770_e52263_d_b2;
        var_vjunc_a_db3 = assign34770_e52263_d_b3;

        let (assign34780_e52276, assign34780_e52276_d_n0, assign34780_e52276_d_n1, assign34780_e52276_d_n2, assign34780_e52276_d_n3, assign34780_e52276_d_n4, assign34780_e52276_d_n5, assign34780_e52276_d_b0, assign34780_e52276_d_b1, assign34780_e52276_d_b2, assign34780_e52276_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34780_e52267: f64 = (var_vjunc_a * var_vjunc_a);
        let assign34780_e52270: f64 = (4.0 * var_juncdlt);
        let assign34780_e52272: f64 = (assign34780_e52270 * var_juncdlt);
        let assign34780_e52273: f64 = (assign34780_e52267 + assign34780_e52272);
        let assign34780_e52274: f64 = (assign34780_e52273).sqrt();
        (assign34780_e52274, (((var_vjunc_a_dn0 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn0)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn1 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn1)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn2 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn2)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn3 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn3)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn4 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn4)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn5 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn5)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_db0 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db0)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_db1 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db1)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_db2 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db2)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_db3 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_db3)) / (2.0 * assign34780_e52274)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34780_e52276;
        var_tmf2_dn0 = assign34780_e52276_d_n0;
        var_tmf2_dn1 = assign34780_e52276_d_n1;
        var_tmf2_dn2 = assign34780_e52276_d_n2;
        var_tmf2_dn3 = assign34780_e52276_d_n3;
        var_tmf2_dn4 = assign34780_e52276_d_n4;
        var_tmf2_dn5 = assign34780_e52276_d_n5;
        var_tmf2_db0 = assign34780_e52276_d_b0;
        var_tmf2_db1 = assign34780_e52276_d_b1;
        var_tmf2_db2 = assign34780_e52276_d_b2;
        var_tmf2_db3 = assign34780_e52276_d_b3;

        let (assign34790_e52284, assign34790_e52284_d_n0, assign34790_e52284_d_n1, assign34790_e52284_d_n2, assign34790_e52284_d_n3, assign34790_e52284_d_n4, assign34790_e52284_d_n5, assign34790_e52284_d_b0, assign34790_e52284_d_b1, assign34790_e52284_d_b2, assign34790_e52284_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34790_e52281: f64 = (var_vjunc_a + var_tmf2);
        let assign34790_e52282: f64 = (0.5 * assign34790_e52281);
        (assign34790_e52282, (0.5 * (var_vjunc_a_dn0 + var_tmf2_dn0)), (0.5 * (var_vjunc_a_dn1 + var_tmf2_dn1)), (0.5 * (var_vjunc_a_dn2 + var_tmf2_dn2)), (0.5 * (var_vjunc_a_dn3 + var_tmf2_dn3)), (0.5 * (var_vjunc_a_dn4 + var_tmf2_dn4)), (0.5 * (var_vjunc_a_dn5 + var_tmf2_dn5)), (0.5 * (var_vjunc_a_db0 + var_tmf2_db0)), (0.5 * (var_vjunc_a_db1 + var_tmf2_db1)), (0.5 * (var_vjunc_a_db2 + var_tmf2_db2)), (0.5 * (var_vjunc_a_db3 + var_tmf2_db3)),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34790_e52284;
        var_vjunc_a_dn0 = assign34790_e52284_d_n0;
        var_vjunc_a_dn1 = assign34790_e52284_d_n1;
        var_vjunc_a_dn2 = assign34790_e52284_d_n2;
        var_vjunc_a_dn3 = assign34790_e52284_d_n3;
        var_vjunc_a_dn4 = assign34790_e52284_d_n4;
        var_vjunc_a_dn5 = assign34790_e52284_d_n5;
        var_vjunc_a_db0 = assign34790_e52284_d_b0;
        var_vjunc_a_db1 = assign34790_e52284_d_b1;
        var_vjunc_a_db2 = assign34790_e52284_d_b2;
        var_vjunc_a_db3 = assign34790_e52284_d_b3;

        let assign34800_e52287: f64 = if var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign34800_e52287;

        let (assign34810_e52293, assign34810_e52293_d_n0, assign34810_e52293_d_n1, assign34810_e52293_d_n2, assign34810_e52293_d_n3, assign34810_e52293_d_n4, assign34810_e52293_d_n5, assign34810_e52293_d_b0, assign34810_e52293_d_b1, assign34810_e52293_d_b2, assign34810_e52293_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard567 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn1, var_vjunc_a_dn2, var_vjunc_a_dn3, var_vjunc_a_dn4, var_vjunc_a_dn5, var_vjunc_a_db0, var_vjunc_a_db1, var_vjunc_a_db2, var_vjunc_a_db3,)
    }
};
        var_vjunc_a = assign34810_e52293;
        var_vjunc_a_dn0 = assign34810_e52293_d_n0;
        var_vjunc_a_dn1 = assign34810_e52293_d_n1;
        var_vjunc_a_dn2 = assign34810_e52293_d_n2;
        var_vjunc_a_dn3 = assign34810_e52293_d_n3;
        var_vjunc_a_dn4 = assign34810_e52293_d_n4;
        var_vjunc_a_dn5 = assign34810_e52293_d_n5;
        var_vjunc_a_db0 = assign34810_e52293_d_b0;
        var_vjunc_a_db1 = assign34810_e52293_d_b1;
        var_vjunc_a_db2 = assign34810_e52293_d_b2;
        var_vjunc_a_db3 = assign34810_e52293_d_b3;

        let (assign34820_e52306, assign34820_e52306_d_n0, assign34820_e52306_d_n1, assign34820_e52306_d_n2, assign34820_e52306_d_n3, assign34820_e52306_d_n4, assign34820_e52306_d_n5, assign34820_e52306_d_b0, assign34820_e52306_d_b1, assign34820_e52306_d_b2, assign34820_e52306_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34820_e52297: f64 = (2.0 * var_epssi);
        let assign34820_e52299: f64 = (assign34820_e52297 * var_vjunc_a);
        let assign34820_e52302: f64 = (1.6021918e-19 * var_ndi_i);
        let assign34820_e52303: f64 = (assign34820_e52299 / assign34820_e52302);
        let assign34820_e52304: f64 = (assign34820_e52303).sqrt();
        (assign34820_e52304, (((assign34820_e52297 * var_vjunc_a_dn0) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn1) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn2) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn3) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn4) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn5) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_db0) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_db1) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_db2) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_db3) / assign34820_e52302) / (2.0 * assign34820_e52304)),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    }
};
        var_w_depa = assign34820_e52306;
        var_w_depa_dn0 = assign34820_e52306_d_n0;
        var_w_depa_dn1 = assign34820_e52306_d_n1;
        var_w_depa_dn2 = assign34820_e52306_d_n2;
        var_w_depa_dn3 = assign34820_e52306_d_n3;
        var_w_depa_dn4 = assign34820_e52306_d_n4;
        var_w_depa_dn5 = assign34820_e52306_d_n5;
        var_w_depa_db0 = assign34820_e52306_d_b0;
        var_w_depa_db1 = assign34820_e52306_d_b1;
        var_w_depa_db2 = assign34820_e52306_d_b2;
        var_w_depa_db3 = assign34820_e52306_d_b3;

        let (assign34830_e52314, assign34830_e52314_d_n0, assign34830_e52314_d_n1, assign34830_e52314_d_n2, assign34830_e52314_d_n3, assign34830_e52314_d_n4, assign34830_e52314_d_n5, assign34830_e52314_d_b0, assign34830_e52314_d_b1, assign34830_e52314_d_b2, assign34830_e52314_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34830_e52310: f64 = (p.p94 - var_w_depa);
        let assign34830_e52312: f64 = (assign34830_e52310 - 1e-7);
        (assign34830_e52312, (-var_w_depa_dn0), (-var_w_depa_dn1), (-var_w_depa_dn2), (-var_w_depa_dn3), (-var_w_depa_dn4), (-var_w_depa_dn5), (-var_w_depa_db0), (-var_w_depa_db1), (-var_w_depa_db2), (-var_w_depa_db3),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn1, var_tmf1_dn2, var_tmf1_dn3, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_db0, var_tmf1_db1, var_tmf1_db2, var_tmf1_db3,)
    }
};
        var_tmf1 = assign34830_e52314;
        var_tmf1_dn0 = assign34830_e52314_d_n0;
        var_tmf1_dn1 = assign34830_e52314_d_n1;
        var_tmf1_dn2 = assign34830_e52314_d_n2;
        var_tmf1_dn3 = assign34830_e52314_d_n3;
        var_tmf1_dn4 = assign34830_e52314_d_n4;
        var_tmf1_dn5 = assign34830_e52314_d_n5;
        var_tmf1_db0 = assign34830_e52314_d_b0;
        var_tmf1_db1 = assign34830_e52314_d_b1;
        var_tmf1_db2 = assign34830_e52314_d_b2;
        var_tmf1_db3 = assign34830_e52314_d_b3;

        let (assign34840_e52322, assign34840_e52322_d_n0, assign34840_e52322_d_n1, assign34840_e52322_d_n2, assign34840_e52322_d_n3, assign34840_e52322_d_n4, assign34840_e52322_d_n5, assign34840_e52322_d_b0, assign34840_e52322_d_b1, assign34840_e52322_d_b2, assign34840_e52322_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34840_e52318: f64 = (4.0 * p.p94);
        let assign34840_e52320: f64 = (assign34840_e52318 * 1e-7);
        (assign34840_e52320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34840_e52322;
        var_tmf2_dn0 = assign34840_e52322_d_n0;
        var_tmf2_dn1 = assign34840_e52322_d_n1;
        var_tmf2_dn2 = assign34840_e52322_d_n2;
        var_tmf2_dn3 = assign34840_e52322_d_n3;
        var_tmf2_dn4 = assign34840_e52322_d_n4;
        var_tmf2_dn5 = assign34840_e52322_d_n5;
        var_tmf2_db0 = assign34840_e52322_d_b0;
        var_tmf2_db1 = assign34840_e52322_d_b1;
        var_tmf2_db2 = assign34840_e52322_d_b2;
        var_tmf2_db3 = assign34840_e52322_d_b3;

        let (assign34850_e52332, assign34850_e52332_d_n0, assign34850_e52332_d_n1, assign34850_e52332_d_n2, assign34850_e52332_d_n3, assign34850_e52332_d_n4, assign34850_e52332_d_n5, assign34850_e52332_d_b0, assign34850_e52332_d_b1, assign34850_e52332_d_b2, assign34850_e52332_d_b3,) = {
    if (var_guard558 != 0.0) {
        let (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n1, assign34850_e52330_d_n2, assign34850_e52330_d_n3, assign34850_e52330_d_n4, assign34850_e52330_d_n5, assign34850_e52330_d_b0, assign34850_e52330_d_b1, assign34850_e52330_d_b2, assign34850_e52330_d_b3,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
            } else {
                let assign34850_e52329: f64 = (-var_tmf2);
                (assign34850_e52329, (-var_tmf2_dn0), (-var_tmf2_dn1), (-var_tmf2_dn2), (-var_tmf2_dn3), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_db0), (-var_tmf2_db1), (-var_tmf2_db2), (-var_tmf2_db3),)
            }
        };
        (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n1, assign34850_e52330_d_n2, assign34850_e52330_d_n3, assign34850_e52330_d_n4, assign34850_e52330_d_n5, assign34850_e52330_d_b0, assign34850_e52330_d_b1, assign34850_e52330_d_b2, assign34850_e52330_d_b3,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34850_e52332;
        var_tmf2_dn0 = assign34850_e52332_d_n0;
        var_tmf2_dn1 = assign34850_e52332_d_n1;
        var_tmf2_dn2 = assign34850_e52332_d_n2;
        var_tmf2_dn3 = assign34850_e52332_d_n3;
        var_tmf2_dn4 = assign34850_e52332_d_n4;
        var_tmf2_dn5 = assign34850_e52332_d_n5;
        var_tmf2_db0 = assign34850_e52332_d_b0;
        var_tmf2_db1 = assign34850_e52332_d_b1;
        var_tmf2_db2 = assign34850_e52332_d_b2;
        var_tmf2_db3 = assign34850_e52332_d_b3;

        let (assign34860_e52341, assign34860_e52341_d_n0, assign34860_e52341_d_n1, assign34860_e52341_d_n2, assign34860_e52341_d_n3, assign34860_e52341_d_n4, assign34860_e52341_d_n5, assign34860_e52341_d_b0, assign34860_e52341_d_b1, assign34860_e52341_d_b2, assign34860_e52341_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34860_e52336: f64 = (var_tmf1 * var_tmf1);
        let assign34860_e52338: f64 = (assign34860_e52336 + var_tmf2);
        let assign34860_e52339: f64 = (assign34860_e52338).sqrt();
        (assign34860_e52339, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn1 * var_tmf1) + (var_tmf1 * var_tmf1_dn1)) + var_tmf2_dn1) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn3 * var_tmf1) + (var_tmf1 * var_tmf1_dn3)) + var_tmf2_dn3) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign34860_e52339)), ((((var_tmf1_db0 * var_tmf1) + (var_tmf1 * var_tmf1_db0)) + var_tmf2_db0) / (2.0 * assign34860_e52339)), ((((var_tmf1_db1 * var_tmf1) + (var_tmf1 * var_tmf1_db1)) + var_tmf2_db1) / (2.0 * assign34860_e52339)), ((((var_tmf1_db2 * var_tmf1) + (var_tmf1 * var_tmf1_db2)) + var_tmf2_db2) / (2.0 * assign34860_e52339)), ((((var_tmf1_db3 * var_tmf1) + (var_tmf1 * var_tmf1_db3)) + var_tmf2_db3) / (2.0 * assign34860_e52339)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn1, var_tmf2_dn2, var_tmf2_dn3, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_db0, var_tmf2_db1, var_tmf2_db2, var_tmf2_db3,)
    }
};
        var_tmf2 = assign34860_e52341;
        var_tmf2_dn0 = assign34860_e52341_d_n0;
        var_tmf2_dn1 = assign34860_e52341_d_n1;
        var_tmf2_dn2 = assign34860_e52341_d_n2;
        var_tmf2_dn3 = assign34860_e52341_d_n3;
        var_tmf2_dn4 = assign34860_e52341_d_n4;
        var_tmf2_dn5 = assign34860_e52341_d_n5;
        var_tmf2_db0 = assign34860_e52341_d_b0;
        var_tmf2_db1 = assign34860_e52341_d_b1;
        var_tmf2_db2 = assign34860_e52341_d_b2;
        var_tmf2_db3 = assign34860_e52341_d_b3;

        let (assign34870_e52351, assign34870_e52351_d_n0, assign34870_e52351_d_n1, assign34870_e52351_d_n2, assign34870_e52351_d_n3, assign34870_e52351_d_n4, assign34870_e52351_d_n5, assign34870_e52351_d_b0, assign34870_e52351_d_b1, assign34870_e52351_d_b2, assign34870_e52351_d_b3,) = {
    if (var_guard558 != 0.0) {
        let assign34870_e52347: f64 = (var_tmf1 + var_tmf2);
        let assign34870_e52348: f64 = (0.5 * assign34870_e52347);
        let assign34870_e52349: f64 = (p.p94 - assign34870_e52348);
        (assign34870_e52349, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn1 + var_tmf2_dn1))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn3 + var_tmf2_dn3))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_db0 + var_tmf2_db0))), (-(0.5 * (var_tmf1_db1 + var_tmf2_db1))), (-(0.5 * (var_tmf1_db2 + var_tmf2_db2))), (-(0.5 * (var_tmf1_db3 + var_tmf2_db3))),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    }
};
        var_w_depa = assign34870_e52351;
        var_w_depa_dn0 = assign34870_e52351_d_n0;
        var_w_depa_dn1 = assign34870_e52351_d_n1;
        var_w_depa_dn2 = assign34870_e52351_d_n2;
        var_w_depa_dn3 = assign34870_e52351_d_n3;
        var_w_depa_dn4 = assign34870_e52351_d_n4;
        var_w_depa_dn5 = assign34870_e52351_d_n5;
        var_w_depa_db0 = assign34870_e52351_d_b0;
        var_w_depa_db1 = assign34870_e52351_d_b1;
        var_w_depa_db2 = assign34870_e52351_d_b2;
        var_w_depa_db3 = assign34870_e52351_d_b3;

        let assign34880_e52354: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };
        var_guard568 = assign34880_e52354;

        let (assign34890_e52364, assign34890_e52364_d_n0, assign34890_e52364_d_n1, assign34890_e52364_d_n2, assign34890_e52364_d_n3, assign34890_e52364_d_n4, assign34890_e52364_d_n5, assign34890_e52364_d_b0, assign34890_e52364_d_b1, assign34890_e52364_d_b2, assign34890_e52364_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34890_e52361: f64 = (1.0 / var_w_depa0);
        let assign34890_e52362: f64 = (var_w_depa * assign34890_e52361);
        (assign34890_e52362, ((var_w_depa_dn0 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn0 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn1 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn1 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn2 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn2 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn3 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn3 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn4 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn4 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn5 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn5 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db0 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db0 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db1 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db1 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db2 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db2 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_db3 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_db3 / (var_w_depa0 * var_w_depa0))))),)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn1, var_w_qs_a_dn2, var_w_qs_a_dn3, var_w_qs_a_dn4, var_w_qs_a_dn5, var_w_qs_a_db0, var_w_qs_a_db1, var_w_qs_a_db2, var_w_qs_a_db3,)
    }
};
        var_w_qs_a = assign34890_e52364;
        var_w_qs_a_dn0 = assign34890_e52364_d_n0;
        var_w_qs_a_dn1 = assign34890_e52364_d_n1;
        var_w_qs_a_dn2 = assign34890_e52364_d_n2;
        var_w_qs_a_dn3 = assign34890_e52364_d_n3;
        var_w_qs_a_dn4 = assign34890_e52364_d_n4;
        var_w_qs_a_dn5 = assign34890_e52364_d_n5;
        var_w_qs_a_db0 = assign34890_e52364_d_b0;
        var_w_qs_a_db1 = assign34890_e52364_d_b1;
        var_w_qs_a_db2 = assign34890_e52364_d_b2;
        var_w_qs_a_db3 = assign34890_e52364_d_b3;

        let (assign34900_e52372, assign34900_e52372_d_n0, assign34900_e52372_d_n1, assign34900_e52372_d_n2, assign34900_e52372_d_n3, assign34900_e52372_d_n4, assign34900_e52372_d_n5, assign34900_e52372_d_b0, assign34900_e52372_d_b1, assign34900_e52372_d_b2, assign34900_e52372_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34900_e52370: f64 = (nv5 - 0.0);
        (assign34900_e52370, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_w_nqs_a, var_w_nqs_a_dn0, var_w_nqs_a_dn1, var_w_nqs_a_dn2, var_w_nqs_a_dn3, var_w_nqs_a_dn4, var_w_nqs_a_dn5, var_w_nqs_a_db0, var_w_nqs_a_db1, var_w_nqs_a_db2, var_w_nqs_a_db3,)
    }
};
        var_w_nqs_a = assign34900_e52372;
        var_w_nqs_a_dn0 = assign34900_e52372_d_n0;
        var_w_nqs_a_dn1 = assign34900_e52372_d_n1;
        var_w_nqs_a_dn2 = assign34900_e52372_d_n2;
        var_w_nqs_a_dn3 = assign34900_e52372_d_n3;
        var_w_nqs_a_dn4 = assign34900_e52372_d_n4;
        var_w_nqs_a_dn5 = assign34900_e52372_d_n5;
        var_w_nqs_a_db0 = assign34900_e52372_d_b0;
        var_w_nqs_a_db1 = assign34900_e52372_d_b1;
        var_w_nqs_a_db2 = assign34900_e52372_d_b2;
        var_w_nqs_a_db3 = assign34900_e52372_d_b3;

        let (assign34910_e52382, assign34910_e52382_d_n0, assign34910_e52382_d_n1, assign34910_e52382_d_n2, assign34910_e52382_d_n3, assign34910_e52382_d_n4, assign34910_e52382_d_n5, assign34910_e52382_d_b0, assign34910_e52382_d_b1, assign34910_e52382_d_b2, assign34910_e52382_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34910_e52378: f64 = (var_w_nqs_a - var_w_qs_a);
        let assign34910_e52380: f64 = (assign34910_e52378 / p.p95);
        (assign34910_e52380, ((var_w_nqs_a_dn0 - var_w_qs_a_dn0) / p.p95), ((var_w_nqs_a_dn1 - var_w_qs_a_dn1) / p.p95), ((var_w_nqs_a_dn2 - var_w_qs_a_dn2) / p.p95), ((var_w_nqs_a_dn3 - var_w_qs_a_dn3) / p.p95), ((var_w_nqs_a_dn4 - var_w_qs_a_dn4) / p.p95), ((var_w_nqs_a_dn5 - var_w_qs_a_dn5) / p.p95), ((var_w_nqs_a_db0 - var_w_qs_a_db0) / p.p95), ((var_w_nqs_a_db1 - var_w_qs_a_db1) / p.p95), ((var_w_nqs_a_db2 - var_w_qs_a_db2) / p.p95), ((var_w_nqs_a_db3 - var_w_qs_a_db3) / p.p95),)
    } else {
        (var_iwnqs0_a, var_iwnqs0_a_dn0, var_iwnqs0_a_dn1, var_iwnqs0_a_dn2, var_iwnqs0_a_dn3, var_iwnqs0_a_dn4, var_iwnqs0_a_dn5, var_iwnqs0_a_db0, var_iwnqs0_a_db1, var_iwnqs0_a_db2, var_iwnqs0_a_db3,)
    }
};
        var_iwnqs0_a = assign34910_e52382;
        var_iwnqs0_a_dn0 = assign34910_e52382_d_n0;
        var_iwnqs0_a_dn1 = assign34910_e52382_d_n1;
        var_iwnqs0_a_dn2 = assign34910_e52382_d_n2;
        var_iwnqs0_a_dn3 = assign34910_e52382_d_n3;
        var_iwnqs0_a_dn4 = assign34910_e52382_d_n4;
        var_iwnqs0_a_dn5 = assign34910_e52382_d_n5;
        var_iwnqs0_a_db0 = assign34910_e52382_d_b0;
        var_iwnqs0_a_db1 = assign34910_e52382_d_b1;
        var_iwnqs0_a_db2 = assign34910_e52382_d_b2;
        var_iwnqs0_a_db3 = assign34910_e52382_d_b3;

        let (assign34930_e52399, assign34930_e52399_d_n0, assign34930_e52399_d_n1, assign34930_e52399_d_n2, assign34930_e52399_d_n3, assign34930_e52399_d_n4, assign34930_e52399_d_n5, assign34930_e52399_d_b0, assign34930_e52399_d_b1, assign34930_e52399_d_b2, assign34930_e52399_d_b3,) = {
    if ((var_guard558 != 0.0) && (var_guard568 == 0.0)) {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn1, var_w_depa_dn2, var_w_depa_dn3, var_w_depa_dn4, var_w_depa_dn5, var_w_depa_db0, var_w_depa_db1, var_w_depa_db2, var_w_depa_db3,)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn1, var_w_qs_a_dn2, var_w_qs_a_dn3, var_w_qs_a_dn4, var_w_qs_a_dn5, var_w_qs_a_db0, var_w_qs_a_db1, var_w_qs_a_db2, var_w_qs_a_db3,)
    }
};
        var_w_qs_a = assign34930_e52399;
        var_w_qs_a_dn0 = assign34930_e52399_d_n0;
        var_w_qs_a_dn1 = assign34930_e52399_d_n1;
        var_w_qs_a_dn2 = assign34930_e52399_d_n2;
        var_w_qs_a_dn3 = assign34930_e52399_d_n3;
        var_w_qs_a_dn4 = assign34930_e52399_d_n4;
        var_w_qs_a_dn5 = assign34930_e52399_d_n5;
        var_w_qs_a_db0 = assign34930_e52399_d_b0;
        var_w_qs_a_db1 = assign34930_e52399_d_b1;
        var_w_qs_a_db2 = assign34930_e52399_d_b2;
        var_w_qs_a_db3 = assign34930_e52399_d_b3;

        let assign35080_e52535: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };
        var_guard571 = assign35080_e52535;

        let assign35090_e52542: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        var_guard572 = assign35090_e52542;


        *var_guard567_slot = var_guard567;
        *var_guard568_slot = var_guard568;
        *var_guard571_slot = var_guard571;
        *var_guard572_slot = var_guard572;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_db0_slot = var_iwnqs0_a_db0;
        *var_iwnqs0_a_db1_slot = var_iwnqs0_a_db1;
        *var_iwnqs0_a_db2_slot = var_iwnqs0_a_db2;
        *var_iwnqs0_a_db3_slot = var_iwnqs0_a_db3;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn1_slot = var_iwnqs0_a_dn1;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn3_slot = var_iwnqs0_a_dn3;
        *var_iwnqs0_a_dn4_slot = var_iwnqs0_a_dn4;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
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
        *var_vjunc_a_slot = var_vjunc_a;
        *var_vjunc_a_db0_slot = var_vjunc_a_db0;
        *var_vjunc_a_db1_slot = var_vjunc_a_db1;
        *var_vjunc_a_db2_slot = var_vjunc_a_db2;
        *var_vjunc_a_db3_slot = var_vjunc_a_db3;
        *var_vjunc_a_dn0_slot = var_vjunc_a_dn0;
        *var_vjunc_a_dn1_slot = var_vjunc_a_dn1;
        *var_vjunc_a_dn2_slot = var_vjunc_a_dn2;
        *var_vjunc_a_dn3_slot = var_vjunc_a_dn3;
        *var_vjunc_a_dn4_slot = var_vjunc_a_dn4;
        *var_vjunc_a_dn5_slot = var_vjunc_a_dn5;
        *var_w_depa_slot = var_w_depa;
        *var_w_depa_db0_slot = var_w_depa_db0;
        *var_w_depa_db1_slot = var_w_depa_db1;
        *var_w_depa_db2_slot = var_w_depa_db2;
        *var_w_depa_db3_slot = var_w_depa_db3;
        *var_w_depa_dn0_slot = var_w_depa_dn0;
        *var_w_depa_dn1_slot = var_w_depa_dn1;
        *var_w_depa_dn2_slot = var_w_depa_dn2;
        *var_w_depa_dn3_slot = var_w_depa_dn3;
        *var_w_depa_dn4_slot = var_w_depa_dn4;
        *var_w_depa_dn5_slot = var_w_depa_dn5;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_db0_slot = var_w_nqs_a_db0;
        *var_w_nqs_a_db1_slot = var_w_nqs_a_db1;
        *var_w_nqs_a_db2_slot = var_w_nqs_a_db2;
        *var_w_nqs_a_db3_slot = var_w_nqs_a_db3;
        *var_w_nqs_a_dn0_slot = var_w_nqs_a_dn0;
        *var_w_nqs_a_dn1_slot = var_w_nqs_a_dn1;
        *var_w_nqs_a_dn2_slot = var_w_nqs_a_dn2;
        *var_w_nqs_a_dn3_slot = var_w_nqs_a_dn3;
        *var_w_nqs_a_dn4_slot = var_w_nqs_a_dn4;
        *var_w_nqs_a_dn5_slot = var_w_nqs_a_dn5;
        *var_w_qs_a_slot = var_w_qs_a;
        *var_w_qs_a_db0_slot = var_w_qs_a_db0;
        *var_w_qs_a_db1_slot = var_w_qs_a_db1;
        *var_w_qs_a_db2_slot = var_w_qs_a_db2;
        *var_w_qs_a_db3_slot = var_w_qs_a_db3;
        *var_w_qs_a_dn0_slot = var_w_qs_a_dn0;
        *var_w_qs_a_dn1_slot = var_w_qs_a_dn1;
        *var_w_qs_a_dn2_slot = var_w_qs_a_dn2;
        *var_w_qs_a_dn3_slot = var_w_qs_a_dn3;
        *var_w_qs_a_dn4_slot = var_w_qs_a_dn4;
        *var_w_qs_a_dn5_slot = var_w_qs_a_dn5;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scalar(0, (8.8541878176e-12 * 11.8));

        s.store_scalar(1, (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) }));

        s.b[388] = ((!param_given[6]) && param_given[96]);
        s.store_scalar(388, if s.b[388] { 1.0 } else { 0.0 });

        if s.b[388] {
            s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));
        }

        s.store_scalar(2, (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 }));

        s.store_scalar(12, p.p17);

        s.store_scalar(13, p.p18);

        s.store_scalar(14, p.p19);

        s.store_scalar(15, (if (p.p20 > 0.0) { p.p20 } else { 0.0 }));

        s.store_scalar(16, (if (p.p21 > 0.0) { p.p21 } else { 0.0 }));

        s.store_scalar(17, (if (p.p22 > 0.0) { p.p22 } else { 0.0 }));

        s.store_scalar(62, (if (p.p63 > 0.1) { p.p63 } else { 0.1 }));

        s.store_scalar(64, (if (p.p64 > 0.1) { p.p64 } else { 0.1 }));

        s.store_scalar(63, (if (p.p65 > 0.1) { p.p65 } else { 0.1 }));

        s.store_scalar(75, (if (p.p76 > 0.1) { p.p76 } else { 0.1 }));

        s.store_scalar(76, (if (p.p77 > 0.0) { p.p77 } else { 0.0 }));

        s.store_scalar(77, (if (p.p78 > 0.0) { p.p78 } else { 0.0 }));

        s.store_scalar(45, 0.0);

        s.b[389] = (p.p81 > 0.5);
        s.store_scalar(389, if s.b[389] { 1.0 } else { 0.0 });

        if s.b[389] {
            s.store_scalar(45, 1.0);
        }

        if (!s.b[389]) {
            s.store_scalar(45, 0.0);
        }

        s.store_scalar(46, (if (p.p82 > 0.5) { p.p82 } else { 0.5 }));

        s.store_offset(78, 1, 273.15);

        s.store_scalar(79, ((ctx_temp + p.p102)).max((273.15 + (-250.0))));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.store_scalar(81, (1.3806505e-23 / 1.6021918e-19));

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.store_scalar(84, (s.v[81] * s.v[79]));

        s.store_scalar(85, (1.0 / s.v[84]));

        s.store_div_scaled_inputs(89, A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0);

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.store_scalar(90, ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79])));

        s.store_scalar(95, (s.v[12] + s.v[90]));

        s.store_scalar(96, (s.v[13] + s.v[90]));

        s.store_scalar(97, (s.v[14] + s.v[90]));

        s.store_mul_powf_mixed_ai(176, A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))), 80, ((s.v[75] / 2.0) / s.v[62]));

        s.store_mul_powf_mixed_ai(177, A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))), 80, ((s.v[75] / 2.0) / s.v[64]));

        s.store_mul_powf_mixed_ai(178, A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))), 80, ((s.v[75] / 2.0) / s.v[63]));

        s.store_scaled_mul(101, 176, 176, s.v[15]);

        s.store_scaled_mul(102, 177, 177, s.v[16]);

        s.store_scaled_mul(103, 178, 178, s.v[17]);

        s.store_scalar(179, (1.0 - (0.01 * s.v[77])));

        s.store_scalar(308, (p.p87 * 1000000.0));

        s.store_scalar(310, (p.p89 * 1000000.0));

        s.store_scalar(309, (p.p88 * 1000000.0));

        s.store_scalar(307, s.v[308]);

        s.store_scalar(313, s.v[62]);

        s.store_scalar(311, (1450.0 * 0.0001));

        s.store_scalar(312, (500.0 * 0.0001));

        s.store_scalar(368, 0.6);

        s.store_scalar(369, 0.001);

        s.store_scale(318, 176, 1.45e16);

        s.store_scaled_square(319, 318, 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_scaled_product_add_scaled_denominator_indices(322, 320, 321, 2.0, 320, 1.0, 321, 1.0, 1.0);

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_mul(323, 324, 322);

        s.store_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));

        s.store_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));

        s.store_scalar(256, (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]));

        s.store_scalar(257, (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]));

        s.store_scalar(258, (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]));

        s.store_scalar(281, 0.0);

        s.b[393] = ((s.v[101] * s.v[256]) > 0.0);
        s.store_scalar(393, if s.b[393] { 1.0 } else { 0.0 });

        if s.b[393] {
            s.store_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));
        }

        if (!s.b[393]) {
            s.store_scalar(168, 100000000.0);
        }

        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);
        s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });

        if s.b[394] {
            s.store_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));
        }

        if (!s.b[394]) {
            s.store_scalar(169, 100000000.0);
        }

        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);
        s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });

        if s.b[395] {
            s.store_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));
        }

        if (!s.b[395]) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_min3(262, 168, 169, 170);

        s.store_scalar(370, 0.0);

        s.store_scalar(345, 0.0);

        s.store_scalar(338, 0.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(336, 0.0);

        s.store_scalar(337, 0.0);

        s.store_scalar(344, 0.0);

        s.store_scalar(333, (1.6021918e-19 * s.v[256]));

        s.store_scalar(343, ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt());

        s.store_scalar(314, ((p.p94 - s.v[343]) - 1e-7));

        s.store_scalar(315, ((4.0 * p.p94) * 1e-7));

        if (!(s.v[315] > 0.0)) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));

        s.store_sub_from_scalar_ad(343, p.p94, A::scaled_offset(s.ad_value(315), s.v[314], 0.5));

        s.b[413] = (s.v[45] > 0.9);
        s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });

        s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));
        s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });

        if (s.b[413] && s.b[414]) {
            s.store_scalar(45, 0.0);
        }

        s.b[418] = (s.v[45] == 1.0);
        s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });

        if s.b[418] {
            s.store_scalar(277, 0.0);
            s.store_scalar(205, 0.4);
            s.store_scalar(206, 0.65);
            s.store_scalar(207, 0.8);
            s.store_scale(190, 205, (-s.v[46]));
            s.store_scale(191, 206, (-s.v[46]));
            s.store_scale(192, 207, (-s.v[46]));
            s.store_scalar(193, 0.1);
            s.store_scalar(194, 0.2);
        }

        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });

        s.b[464] = (s.v[190] < s.v[262]);
        s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[467] = (s.v[62] < p.p85);
        s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[470] = (s.v[64] < p.p85);
        s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[473] = (s.v[63] < p.p85);
        s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[476] = (s.v[62] < p.p85);
        s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[479] = (s.v[64] < p.p85);
        s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[482] = (s.v[63] < p.p85);
        s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[463]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[463])) {
            s.store_scalar(370, 0.0);
        }

        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });

        s.b[541] = (s.v[191] < s.v[262]);
        s.store_scalar(541, if s.b[541] { 1.0 } else { 0.0 });

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[544] = (s.v[62] < p.p85);
        s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[544])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[545] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(545, if s.b[545] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[545]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[546] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(546, if s.b[546] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && s.b[546]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(191), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[547] = (s.v[64] < p.p85);
        s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[550] = (s.v[63] < p.p85);
        s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_sub_scaled_inputs_indices(360, 191, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[553] = (s.v[62] < p.p85);
        s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[553])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[554] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[554]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[555] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && s.b[555]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[556] = (s.v[64] < p.p85);
        s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[559] = (s.v[63] < p.p85);
        s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[540]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(370, 0.0);
        }

        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });

        s.b[618] = (s.v[192] < s.v[262]);
        s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[621] = (s.v[62] < p.p85);
        s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_sub_scaled_inputs_indices(360, 192, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[621])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[622] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[622]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[623] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && s.b[623]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(192), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[624] = (s.v[64] < p.p85);
        s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_offset_sub_scaled_inputs_indices(360, 192, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[627] = (s.v[63] < p.p85);
        s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_offset_sub_scaled_inputs_indices(360, 192, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[627])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[630] = (s.v[62] < p.p85);
        s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[630])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[631] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[631]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[632] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && s.b[632]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && (!s.b[632])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[633] = (s.v[64] < p.p85);
        s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[633])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[636] = (s.v[63] < p.p85);
        s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[636])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[617]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(370, 0.0);
        }

        s.b[694] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });

        s.b[695] = (s.v[193] < s.v[262]);
        s.store_scalar(695, if s.b[695] { 1.0 } else { 0.0 });

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[698] = (s.v[62] < p.p85);
        s.store_scalar(698, if s.b[698] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_sub_scaled_inputs_indices(360, 193, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[698])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[699] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(699, if s.b[699] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[699]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[700] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(700, if s.b[700] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && s.b[700]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(193), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[701] = (s.v[64] < p.p85);
        s.store_scalar(701, if s.b[701] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_offset_sub_scaled_inputs_indices(360, 193, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[704] = (s.v[63] < p.p85);
        s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_offset_sub_scaled_inputs_indices(360, 193, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[707] = (s.v[62] < p.p85);
        s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[707])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[708] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[708]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[709] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(709, if s.b[709] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && s.b[709]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[710] = (s.v[64] < p.p85);
        s.store_scalar(710, if s.b[710] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[713] = (s.v[63] < p.p85);
        s.store_scalar(713, if s.b[713] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[694]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[694])) {
            s.store_scalar(370, 0.0);
        }

        s.b[771] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });

        s.b[772] = (s.v[194] < s.v[262]);
        s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[775] = (s.v[62] < p.p85);
        s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[775]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[775])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[776] = ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[776]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[777] = ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && s.b[777]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[776])) && (!s.b[777])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(194), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[778] = (s.v[64] < p.p85);
        s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[778]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[778])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if ((s.b[418] && s.b[771]) && s.b[772]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[781] = (s.v[63] < p.p85);
        s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_offset_sub_scaled_inputs_indices(360, 194, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && s.b[781]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[771]) && s.b[772]) && (!s.b[781])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[784] = (s.v[62] < p.p85);
        s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[784]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[784])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[785] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[785]) {
            s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[786] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && s.b[786]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[785])) && (!s.b[786])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_mul_offset_ad_lhs(370, A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[787] = (s.v[64] < p.p85);
        s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[787]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[787])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[790] = (s.v[63] < p.p85);
        s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && s.b[790]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[771]) && (!s.b[772])) && (!s.b[790])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        if ((s.b[418] && s.b[771]) && (!s.b[772])) {
            s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);
        }

        if (s.b[418] && s.b[771]) {
            s.store_offset(370, 370, (-1.0));
        }

        if (s.b[418] && (!s.b[771])) {
            s.store_scalar(370, 0.0);
        }

        s.store_voltage(277, ctx, nodes, Some(0), Some(2));

        s.b[858] = (s.v[45] == 1.0);
        s.store_scalar(858, if s.b[858] { 1.0 } else { 0.0 });

        s.b[866] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.store_scalar(866, if s.b[866] { 1.0 } else { 0.0 });

        s.b[867] = (s.v[277] < s.v[262]);
        s.store_scalar(867, if s.b[867] { 1.0 } else { 0.0 });

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[870] = (s.v[62] < p.p85);
        s.store_scalar(870, if s.b[870] { 1.0 } else { 0.0 });

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[870]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[870])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[871] = ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.store_scalar(871, if s.b[871] { 1.0 } else { 0.0 });

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[871]) {
            s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);
        }

        s.b[872] = ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.store_scalar(872, if s.b[872] { 1.0 } else { 0.0 });

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && s.b[872]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[871])) && (!s.b[872])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(277), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[873] = (s.v[64] < p.p85);
        s.store_scalar(873, if s.b[873] { 1.0 } else { 0.0 });

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[873]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && (!s.b[873])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        if (((!s.b[858]) && s.b[866]) && s.b[867]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[876] = (s.v[63] < p.p85);
        s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_offset_sub_scaled_inputs_indices(360, 277, p.p86, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }

        if ((((!s.b[858]) && s.b[866]) && s.b[867]) && s.b[876]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

    }
}
