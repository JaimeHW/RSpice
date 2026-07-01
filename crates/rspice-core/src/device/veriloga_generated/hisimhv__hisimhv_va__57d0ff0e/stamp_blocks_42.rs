#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_280(
        locals: &mut StampLocals,
    ) {
        let mut assign74240_loop_guard: usize = 0;
        while {
            let assign74240_cond_e112931: f64 = (locals.var_lp_s0_max + 1.0);
            let assign74240_cond_e112933: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_lp_s0 <= assign74240_cond_e112931)) { 1.0 } else { 0.0 };
            assign74240_cond_e112933 != 0.0
        } {
            assign74240_loop_guard += 1;
            assert!(assign74240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign74240_body3_e112960, assign74240_body3_e112960_d_n0, assign74240_body3_e112960_d_n2, assign74240_body3_e112960_d_n4, assign74240_body3_e112960_d_n5, assign74240_body3_e112960_d_n6, assign74240_body3_e112960_d_n7, assign74240_body3_e112960_d_n8, assign74240_body3_e112960_d_n9, assign74240_body3_e112960_d_n10, assign74240_body3_e112960_d_n11, assign74240_body3_e112960_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body3_e112958: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign74240_body3_e112958, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign74240_body3_e112960;
            locals.var_ps0ld_vxb_dn0 = assign74240_body3_e112960_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign74240_body3_e112960_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign74240_body3_e112960_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign74240_body3_e112960_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign74240_body3_e112960_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign74240_body3_e112960_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign74240_body3_e112960_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign74240_body3_e112960_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign74240_body3_e112960_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign74240_body3_e112960_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign74240_body3_e112960_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign74240_body4_e112968, assign74240_body4_e112968_d_n0, assign74240_body4_e112968_d_n2, assign74240_body4_e112968_d_n4, assign74240_body4_e112968_d_n5, assign74240_body4_e112968_d_n6, assign74240_body4_e112968_d_n7, assign74240_body4_e112968_d_n8, assign74240_body4_e112968_d_n9, assign74240_body4_e112968_d_n10, assign74240_body4_e112968_d_n11, assign74240_body4_e112968_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body4_e112966: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign74240_body4_e112966, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign74240_body4_e112968;
            locals.var_chi_dn0 = assign74240_body4_e112968_d_n0;
            locals.var_chi_dn2 = assign74240_body4_e112968_d_n2;
            locals.var_chi_dn4 = assign74240_body4_e112968_d_n4;
            locals.var_chi_dn5 = assign74240_body4_e112968_d_n5;
            locals.var_chi_dn6 = assign74240_body4_e112968_d_n6;
            locals.var_chi_dn7 = assign74240_body4_e112968_d_n7;
            locals.var_chi_dn8 = assign74240_body4_e112968_d_n8;
            locals.var_chi_dn9 = assign74240_body4_e112968_d_n9;
            locals.var_chi_dn10 = assign74240_body4_e112968_d_n10;
            locals.var_chi_dn11 = assign74240_body4_e112968_d_n11;
            locals.var_chi_dn14 = assign74240_body4_e112968_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign74240_body5_e112978, assign74240_body5_e112978_d_n0, assign74240_body5_e112978_d_n2, assign74240_body5_e112978_d_n4, assign74240_body5_e112978_d_n5, assign74240_body5_e112978_d_n6, assign74240_body5_e112978_d_n7, assign74240_body5_e112978_d_n8, assign74240_body5_e112978_d_n9, assign74240_body5_e112978_d_n10, assign74240_body5_e112978_d_n11, assign74240_body5_e112978_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body5_e112975: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign74240_body5_e112976: f64 = (locals.var_c_sb * assign74240_body5_e112975);
        (assign74240_body5_e112976, ((locals.var_c_sb_dn0 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign74240_body5_e112975) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign74240_body5_e112978;
            locals.var_ty_dn0 = assign74240_body5_e112978_d_n0;
            locals.var_ty_dn2 = assign74240_body5_e112978_d_n2;
            locals.var_ty_dn4 = assign74240_body5_e112978_d_n4;
            locals.var_ty_dn5 = assign74240_body5_e112978_d_n5;
            locals.var_ty_dn6 = assign74240_body5_e112978_d_n6;
            locals.var_ty_dn7 = assign74240_body5_e112978_d_n7;
            locals.var_ty_dn8 = assign74240_body5_e112978_d_n8;
            locals.var_ty_dn9 = assign74240_body5_e112978_d_n9;
            locals.var_ty_dn10 = assign74240_body5_e112978_d_n10;
            locals.var_ty_dn11 = assign74240_body5_e112978_d_n11;
            locals.var_ty_dn14 = assign74240_body5_e112978_d_n14;
            locals.var_ty_rv = 0.0;
            let assign74240_body6_e112981: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1731 = assign74240_body6_e112981;
            locals.var_guard1731_rv = 0.0;
            let (assign74240_body7_e112990, assign74240_body7_e112990_d_n0, assign74240_body7_e112990_d_n2, assign74240_body7_e112990_d_n4, assign74240_body7_e112990_d_n5, assign74240_body7_e112990_d_n6, assign74240_body7_e112990_d_n7, assign74240_body7_e112990_d_n8, assign74240_body7_e112990_d_n9, assign74240_body7_e112990_d_n10, assign74240_body7_e112990_d_n11, assign74240_body7_e112990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74240_body7_e112988: f64 = (locals.var_ty).exp();
        (assign74240_body7_e112988, (assign74240_body7_e112988 * locals.var_ty_dn0), (assign74240_body7_e112988 * locals.var_ty_dn2), (assign74240_body7_e112988 * locals.var_ty_dn4), (assign74240_body7_e112988 * locals.var_ty_dn5), (assign74240_body7_e112988 * locals.var_ty_dn6), (assign74240_body7_e112988 * locals.var_ty_dn7), (assign74240_body7_e112988 * locals.var_ty_dn8), (assign74240_body7_e112988 * locals.var_ty_dn9), (assign74240_body7_e112988 * locals.var_ty_dn10), (assign74240_body7_e112988 * locals.var_ty_dn11), (assign74240_body7_e112988 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign74240_body7_e112990;
            locals.var_t1_dn0 = assign74240_body7_e112990_d_n0;
            locals.var_t1_dn2 = assign74240_body7_e112990_d_n2;
            locals.var_t1_dn4 = assign74240_body7_e112990_d_n4;
            locals.var_t1_dn5 = assign74240_body7_e112990_d_n5;
            locals.var_t1_dn6 = assign74240_body7_e112990_d_n6;
            locals.var_t1_dn7 = assign74240_body7_e112990_d_n7;
            locals.var_t1_dn8 = assign74240_body7_e112990_d_n8;
            locals.var_t1_dn9 = assign74240_body7_e112990_d_n9;
            locals.var_t1_dn10 = assign74240_body7_e112990_d_n10;
            locals.var_t1_dn11 = assign74240_body7_e112990_d_n11;
            locals.var_t1_dn14 = assign74240_body7_e112990_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign74240_body8_e113002, assign74240_body8_e113002_d_n0, assign74240_body8_e113002_d_n2, assign74240_body8_e113002_d_n4, assign74240_body8_e113002_d_n5, assign74240_body8_e113002_d_n6, assign74240_body8_e113002_d_n7, assign74240_body8_e113002_d_n8, assign74240_body8_e113002_d_n9, assign74240_body8_e113002_d_n10, assign74240_body8_e113002_d_n11, assign74240_body8_e113002_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74240_body8_e112997: f64 = (-locals.var_c_sb);
        let assign74240_body8_e112999: f64 = (assign74240_body8_e112997 * locals.var_dphi_sb);
        let assign74240_body8_e113000: f64 = (assign74240_body8_e112999).exp();
        (assign74240_body8_e113000, (assign74240_body8_e113000 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn0))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn2))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn4))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn5))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn6))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn7))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn8))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn9))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn10))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn11))), (assign74240_body8_e113000 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign74240_body8_e112997 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign74240_body8_e113002;
            locals.var_t0_dn0 = assign74240_body8_e113002_d_n0;
            locals.var_t0_dn2 = assign74240_body8_e113002_d_n2;
            locals.var_t0_dn4 = assign74240_body8_e113002_d_n4;
            locals.var_t0_dn5 = assign74240_body8_e113002_d_n5;
            locals.var_t0_dn6 = assign74240_body8_e113002_d_n6;
            locals.var_t0_dn7 = assign74240_body8_e113002_d_n7;
            locals.var_t0_dn8 = assign74240_body8_e113002_d_n8;
            locals.var_t0_dn9 = assign74240_body8_e113002_d_n9;
            locals.var_t0_dn10 = assign74240_body8_e113002_d_n10;
            locals.var_t0_dn11 = assign74240_body8_e113002_d_n11;
            locals.var_t0_dn14 = assign74240_body8_e113002_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign74240_body9_e113012, assign74240_body9_e113012_d_n0, assign74240_body9_e113012_d_n2, assign74240_body9_e113012_d_n4, assign74240_body9_e113012_d_n5, assign74240_body9_e113012_d_n6, assign74240_body9_e113012_d_n7, assign74240_body9_e113012_d_n8, assign74240_body9_e113012_d_n9, assign74240_body9_e113012_d_n10, assign74240_body9_e113012_d_n11, assign74240_body9_e113012_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74240_body9_e113010: f64 = (locals.var_t1 - locals.var_t0);
        (assign74240_body9_e113010, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign74240_body9_e113012;
            locals.var_t2_dn0 = assign74240_body9_e113012_d_n0;
            locals.var_t2_dn2 = assign74240_body9_e113012_d_n2;
            locals.var_t2_dn4 = assign74240_body9_e113012_d_n4;
            locals.var_t2_dn5 = assign74240_body9_e113012_d_n5;
            locals.var_t2_dn6 = assign74240_body9_e113012_d_n6;
            locals.var_t2_dn7 = assign74240_body9_e113012_d_n7;
            locals.var_t2_dn8 = assign74240_body9_e113012_d_n8;
            locals.var_t2_dn9 = assign74240_body9_e113012_d_n9;
            locals.var_t2_dn10 = assign74240_body9_e113012_d_n10;
            locals.var_t2_dn11 = assign74240_body9_e113012_d_n11;
            locals.var_t2_dn14 = assign74240_body9_e113012_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign74240_body10_e113025, assign74240_body10_e113025_d_n0, assign74240_body10_e113025_d_n2, assign74240_body10_e113025_d_n4, assign74240_body10_e113025_d_n5, assign74240_body10_e113025_d_n6, assign74240_body10_e113025_d_n7, assign74240_body10_e113025_d_n8, assign74240_body10_e113025_d_n9, assign74240_body10_e113025_d_n10, assign74240_body10_e113025_d_n11, assign74240_body10_e113025_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74240_body10_e113020: f64 = (1.0 + locals.var_t2);
        let assign74240_body10_e113021: f64 = (assign74240_body10_e113020).ln();
        let assign74240_body10_e113023: f64 = (assign74240_body10_e113021 / locals.var_c_sb);
        (assign74240_body10_e113023, ((((locals.var_t2_dn0 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign74240_body10_e113020) * locals.var_c_sb) - (assign74240_body10_e113021 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign74240_body10_e113025;
            locals.var_phi_b_dn0 = assign74240_body10_e113025_d_n0;
            locals.var_phi_b_dn2 = assign74240_body10_e113025_d_n2;
            locals.var_phi_b_dn4 = assign74240_body10_e113025_d_n4;
            locals.var_phi_b_dn5 = assign74240_body10_e113025_d_n5;
            locals.var_phi_b_dn6 = assign74240_body10_e113025_d_n6;
            locals.var_phi_b_dn7 = assign74240_body10_e113025_d_n7;
            locals.var_phi_b_dn8 = assign74240_body10_e113025_d_n8;
            locals.var_phi_b_dn9 = assign74240_body10_e113025_d_n9;
            locals.var_phi_b_dn10 = assign74240_body10_e113025_d_n10;
            locals.var_phi_b_dn11 = assign74240_body10_e113025_d_n11;
            locals.var_phi_b_dn14 = assign74240_body10_e113025_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign74240_body11_e113037, assign74240_body11_e113037_d_n0, assign74240_body11_e113037_d_n2, assign74240_body11_e113037_d_n4, assign74240_body11_e113037_d_n5, assign74240_body11_e113037_d_n6, assign74240_body11_e113037_d_n7, assign74240_body11_e113037_d_n8, assign74240_body11_e113037_d_n9, assign74240_body11_e113037_d_n10, assign74240_body11_e113037_d_n11, assign74240_body11_e113037_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign74240_body11_e113034: f64 = (1.0 + locals.var_t2);
        let assign74240_body11_e113035: f64 = (locals.var_t1 / assign74240_body11_e113034);
        (assign74240_body11_e113035, (((locals.var_t1_dn0 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn0)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn2 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn2)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn4 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn4)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn5 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn5)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn6 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn6)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn7 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn7)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn8 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn8)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn9 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn9)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn10 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn10)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn11 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn11)) / (assign74240_body11_e113034 * assign74240_body11_e113034)), (((locals.var_t1_dn14 * assign74240_body11_e113034) - (locals.var_t1 * locals.var_t2_dn14)) / (assign74240_body11_e113034 * assign74240_body11_e113034)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign74240_body11_e113037;
            locals.var_phi_b_dpss_dn0 = assign74240_body11_e113037_d_n0;
            locals.var_phi_b_dpss_dn2 = assign74240_body11_e113037_d_n2;
            locals.var_phi_b_dpss_dn4 = assign74240_body11_e113037_d_n4;
            locals.var_phi_b_dpss_dn5 = assign74240_body11_e113037_d_n5;
            locals.var_phi_b_dpss_dn6 = assign74240_body11_e113037_d_n6;
            locals.var_phi_b_dpss_dn7 = assign74240_body11_e113037_d_n7;
            locals.var_phi_b_dpss_dn8 = assign74240_body11_e113037_d_n8;
            locals.var_phi_b_dpss_dn9 = assign74240_body11_e113037_d_n9;
            locals.var_phi_b_dpss_dn10 = assign74240_body11_e113037_d_n10;
            locals.var_phi_b_dpss_dn11 = assign74240_body11_e113037_d_n11;
            locals.var_phi_b_dpss_dn14 = assign74240_body11_e113037_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign74240_body12_e113048, assign74240_body12_e113048_d_n0, assign74240_body12_e113048_d_n2, assign74240_body12_e113048_d_n4, assign74240_body12_e113048_d_n5, assign74240_body12_e113048_d_n6, assign74240_body12_e113048_d_n7, assign74240_body12_e113048_d_n8, assign74240_body12_e113048_d_n9, assign74240_body12_e113048_d_n10, assign74240_body12_e113048_d_n11, assign74240_body12_e113048_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 == 0.0)) {
        let assign74240_body12_e113046: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign74240_body12_e113046, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign74240_body12_e113048;
            locals.var_phi_b_dn0 = assign74240_body12_e113048_d_n0;
            locals.var_phi_b_dn2 = assign74240_body12_e113048_d_n2;
            locals.var_phi_b_dn4 = assign74240_body12_e113048_d_n4;
            locals.var_phi_b_dn5 = assign74240_body12_e113048_d_n5;
            locals.var_phi_b_dn6 = assign74240_body12_e113048_d_n6;
            locals.var_phi_b_dn7 = assign74240_body12_e113048_d_n7;
            locals.var_phi_b_dn8 = assign74240_body12_e113048_d_n8;
            locals.var_phi_b_dn9 = assign74240_body12_e113048_d_n9;
            locals.var_phi_b_dn10 = assign74240_body12_e113048_d_n10;
            locals.var_phi_b_dn11 = assign74240_body12_e113048_d_n11;
            locals.var_phi_b_dn14 = assign74240_body12_e113048_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign74240_body13_e113057, assign74240_body13_e113057_d_n0, assign74240_body13_e113057_d_n2, assign74240_body13_e113057_d_n4, assign74240_body13_e113057_d_n5, assign74240_body13_e113057_d_n6, assign74240_body13_e113057_d_n7, assign74240_body13_e113057_d_n8, assign74240_body13_e113057_d_n9, assign74240_body13_e113057_d_n10, assign74240_body13_e113057_d_n11, assign74240_body13_e113057_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1731 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign74240_body13_e113057;
            locals.var_phi_b_dpss_dn0 = assign74240_body13_e113057_d_n0;
            locals.var_phi_b_dpss_dn2 = assign74240_body13_e113057_d_n2;
            locals.var_phi_b_dpss_dn4 = assign74240_body13_e113057_d_n4;
            locals.var_phi_b_dpss_dn5 = assign74240_body13_e113057_d_n5;
            locals.var_phi_b_dpss_dn6 = assign74240_body13_e113057_d_n6;
            locals.var_phi_b_dpss_dn7 = assign74240_body13_e113057_d_n7;
            locals.var_phi_b_dpss_dn8 = assign74240_body13_e113057_d_n8;
            locals.var_phi_b_dpss_dn9 = assign74240_body13_e113057_d_n9;
            locals.var_phi_b_dpss_dn10 = assign74240_body13_e113057_d_n10;
            locals.var_phi_b_dpss_dn11 = assign74240_body13_e113057_d_n11;
            locals.var_phi_b_dpss_dn14 = assign74240_body13_e113057_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign74240_body14_e113065, assign74240_body14_e113065_d_n0, assign74240_body14_e113065_d_n2, assign74240_body14_e113065_d_n4, assign74240_body14_e113065_d_n5, assign74240_body14_e113065_d_n6, assign74240_body14_e113065_d_n7, assign74240_body14_e113065_d_n8, assign74240_body14_e113065_d_n9, assign74240_body14_e113065_d_n10, assign74240_body14_e113065_d_n11, assign74240_body14_e113065_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body14_e113063: f64 = (locals.var_beta * locals.var_phi_b);
        (assign74240_body14_e113063, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign74240_body14_e113065;
            locals.var_chib_dn0 = assign74240_body14_e113065_d_n0;
            locals.var_chib_dn2 = assign74240_body14_e113065_d_n2;
            locals.var_chib_dn4 = assign74240_body14_e113065_d_n4;
            locals.var_chib_dn5 = assign74240_body14_e113065_d_n5;
            locals.var_chib_dn6 = assign74240_body14_e113065_d_n6;
            locals.var_chib_dn7 = assign74240_body14_e113065_d_n7;
            locals.var_chib_dn8 = assign74240_body14_e113065_d_n8;
            locals.var_chib_dn9 = assign74240_body14_e113065_d_n9;
            locals.var_chib_dn10 = assign74240_body14_e113065_d_n10;
            locals.var_chib_dn11 = assign74240_body14_e113065_d_n11;
            locals.var_chib_dn14 = assign74240_body14_e113065_d_n14;
            locals.var_chib_rv = 0.0;
            let assign74240_body15_e113067: f64 = (locals.var_chi).abs();
            let assign74240_body15_e113069: f64 = if assign74240_body15_e113067 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1732 = assign74240_body15_e113069;
            locals.var_guard1732_rv = 0.0;
            let (assign74240_body17_e113115, assign74240_body17_e113115_d_n0, assign74240_body17_e113115_d_n2, assign74240_body17_e113115_d_n4, assign74240_body17_e113115_d_n5, assign74240_body17_e113115_d_n6, assign74240_body17_e113115_d_n7, assign74240_body17_e113115_d_n8, assign74240_body17_e113115_d_n9, assign74240_body17_e113115_d_n10, assign74240_body17_e113115_d_n11, assign74240_body17_e113115_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body17_e113093: f64 = (locals.var_chi * locals.var_chi);
        let assign74240_body17_e113095: f64 = (assign74240_body17_e113093 / 2.0);
        let assign74240_body17_e113099: f64 = (locals.var_chi / 3.0);
        let assign74240_body17_e113103: f64 = (locals.var_chi / 4.0);
        let assign74240_body17_e113107: f64 = (locals.var_chi / 5.0);
        let assign74240_body17_e113108: f64 = (1.0 - assign74240_body17_e113107);
        let assign74240_body17_e113109: f64 = (assign74240_body17_e113103 * assign74240_body17_e113108);
        let assign74240_body17_e113110: f64 = (1.0 - assign74240_body17_e113109);
        let assign74240_body17_e113111: f64 = (assign74240_body17_e113099 * assign74240_body17_e113110);
        let assign74240_body17_e113112: f64 = (1.0 - assign74240_body17_e113111);
        let assign74240_body17_e113113: f64 = (assign74240_body17_e113095 * assign74240_body17_e113112);
        (assign74240_body17_e113113, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn0 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn0 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn2 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn2 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn4 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn4 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn5 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn5 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn6 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn6 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn7 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn7 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn8 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn8 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn9 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn9 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn10 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn10 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn11 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn11 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign74240_body17_e113112) + (assign74240_body17_e113095 * (-(((locals.var_chi_dn14 / 3.0) * assign74240_body17_e113110) + (assign74240_body17_e113099 * (-(((locals.var_chi_dn14 / 4.0) * assign74240_body17_e113108) + (assign74240_body17_e113103 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign74240_body17_e113115;
            locals.var_t0_dn0 = assign74240_body17_e113115_d_n0;
            locals.var_t0_dn2 = assign74240_body17_e113115_d_n2;
            locals.var_t0_dn4 = assign74240_body17_e113115_d_n4;
            locals.var_t0_dn5 = assign74240_body17_e113115_d_n5;
            locals.var_t0_dn6 = assign74240_body17_e113115_d_n6;
            locals.var_t0_dn7 = assign74240_body17_e113115_d_n7;
            locals.var_t0_dn8 = assign74240_body17_e113115_d_n8;
            locals.var_t0_dn9 = assign74240_body17_e113115_d_n9;
            locals.var_t0_dn10 = assign74240_body17_e113115_d_n10;
            locals.var_t0_dn11 = assign74240_body17_e113115_d_n11;
            locals.var_t0_dn14 = assign74240_body17_e113115_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign74240_body18_e113141, assign74240_body18_e113141_d_n0, assign74240_body18_e113141_d_n2, assign74240_body18_e113141_d_n4, assign74240_body18_e113141_d_n5, assign74240_body18_e113141_d_n6, assign74240_body18_e113141_d_n7, assign74240_body18_e113141_d_n8, assign74240_body18_e113141_d_n9, assign74240_body18_e113141_d_n10, assign74240_body18_e113141_d_n11, assign74240_body18_e113141_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body18_e113125: f64 = (locals.var_chi / 2.0);
        let assign74240_body18_e113129: f64 = (locals.var_chi / 3.0);
        let assign74240_body18_e113133: f64 = (locals.var_chi / 4.0);
        let assign74240_body18_e113134: f64 = (1.0 - assign74240_body18_e113133);
        let assign74240_body18_e113135: f64 = (assign74240_body18_e113129 * assign74240_body18_e113134);
        let assign74240_body18_e113136: f64 = (1.0 - assign74240_body18_e113135);
        let assign74240_body18_e113137: f64 = (assign74240_body18_e113125 * assign74240_body18_e113136);
        let assign74240_body18_e113138: f64 = (1.0 - assign74240_body18_e113137);
        let assign74240_body18_e113139: f64 = (locals.var_chi * assign74240_body18_e113138);
        (assign74240_body18_e113139, ((locals.var_chi_dn0 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn0 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn2 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn4 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn5 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn6 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn7 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn8 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn9 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn10 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn11 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign74240_body18_e113138) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign74240_body18_e113136) + (assign74240_body18_e113125 * (-(((locals.var_chi_dn14 / 3.0) * assign74240_body18_e113134) + (assign74240_body18_e113129 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign74240_body18_e113141;
            locals.var_t1_dn0 = assign74240_body18_e113141_d_n0;
            locals.var_t1_dn2 = assign74240_body18_e113141_d_n2;
            locals.var_t1_dn4 = assign74240_body18_e113141_d_n4;
            locals.var_t1_dn5 = assign74240_body18_e113141_d_n5;
            locals.var_t1_dn6 = assign74240_body18_e113141_d_n6;
            locals.var_t1_dn7 = assign74240_body18_e113141_d_n7;
            locals.var_t1_dn8 = assign74240_body18_e113141_d_n8;
            locals.var_t1_dn9 = assign74240_body18_e113141_d_n9;
            locals.var_t1_dn10 = assign74240_body18_e113141_d_n10;
            locals.var_t1_dn11 = assign74240_body18_e113141_d_n11;
            locals.var_t1_dn14 = assign74240_body18_e113141_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign74240_body19_e113171, assign74240_body19_e113171_d_n0, assign74240_body19_e113171_d_n2, assign74240_body19_e113171_d_n4, assign74240_body19_e113171_d_n5, assign74240_body19_e113171_d_n6, assign74240_body19_e113171_d_n7, assign74240_body19_e113171_d_n8, assign74240_body19_e113171_d_n9, assign74240_body19_e113171_d_n10, assign74240_body19_e113171_d_n11, assign74240_body19_e113171_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body19_e113149: f64 = (locals.var_chib * locals.var_chib);
        let assign74240_body19_e113151: f64 = (assign74240_body19_e113149 / 2.0);
        let assign74240_body19_e113155: f64 = (locals.var_chib / 3.0);
        let assign74240_body19_e113159: f64 = (locals.var_chib / 4.0);
        let assign74240_body19_e113163: f64 = (locals.var_chib / 5.0);
        let assign74240_body19_e113164: f64 = (1.0 - assign74240_body19_e113163);
        let assign74240_body19_e113165: f64 = (assign74240_body19_e113159 * assign74240_body19_e113164);
        let assign74240_body19_e113166: f64 = (1.0 - assign74240_body19_e113165);
        let assign74240_body19_e113167: f64 = (assign74240_body19_e113155 * assign74240_body19_e113166);
        let assign74240_body19_e113168: f64 = (1.0 - assign74240_body19_e113167);
        let assign74240_body19_e113169: f64 = (assign74240_body19_e113151 * assign74240_body19_e113168);
        (assign74240_body19_e113169, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn0 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn0 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn2 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn2 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn4 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn4 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn5 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn5 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn6 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn6 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn7 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn7 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn8 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn8 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn9 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn9 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn10 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn10 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn11 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn11 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign74240_body19_e113168) + (assign74240_body19_e113151 * (-(((locals.var_chib_dn14 / 3.0) * assign74240_body19_e113166) + (assign74240_body19_e113155 * (-(((locals.var_chib_dn14 / 4.0) * assign74240_body19_e113164) + (assign74240_body19_e113159 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign74240_body19_e113171;
            locals.var_t2_dn0 = assign74240_body19_e113171_d_n0;
            locals.var_t2_dn2 = assign74240_body19_e113171_d_n2;
            locals.var_t2_dn4 = assign74240_body19_e113171_d_n4;
            locals.var_t2_dn5 = assign74240_body19_e113171_d_n5;
            locals.var_t2_dn6 = assign74240_body19_e113171_d_n6;
            locals.var_t2_dn7 = assign74240_body19_e113171_d_n7;
            locals.var_t2_dn8 = assign74240_body19_e113171_d_n8;
            locals.var_t2_dn9 = assign74240_body19_e113171_d_n9;
            locals.var_t2_dn10 = assign74240_body19_e113171_d_n10;
            locals.var_t2_dn11 = assign74240_body19_e113171_d_n11;
            locals.var_t2_dn14 = assign74240_body19_e113171_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign74240_body20_e113197, assign74240_body20_e113197_d_n0, assign74240_body20_e113197_d_n2, assign74240_body20_e113197_d_n4, assign74240_body20_e113197_d_n5, assign74240_body20_e113197_d_n6, assign74240_body20_e113197_d_n7, assign74240_body20_e113197_d_n8, assign74240_body20_e113197_d_n9, assign74240_body20_e113197_d_n10, assign74240_body20_e113197_d_n11, assign74240_body20_e113197_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body20_e113181: f64 = (locals.var_chib / 2.0);
        let assign74240_body20_e113185: f64 = (locals.var_chib / 3.0);
        let assign74240_body20_e113189: f64 = (locals.var_chib / 4.0);
        let assign74240_body20_e113190: f64 = (1.0 - assign74240_body20_e113189);
        let assign74240_body20_e113191: f64 = (assign74240_body20_e113185 * assign74240_body20_e113190);
        let assign74240_body20_e113192: f64 = (1.0 - assign74240_body20_e113191);
        let assign74240_body20_e113193: f64 = (assign74240_body20_e113181 * assign74240_body20_e113192);
        let assign74240_body20_e113194: f64 = (1.0 - assign74240_body20_e113193);
        let assign74240_body20_e113195: f64 = (locals.var_chib * assign74240_body20_e113194);
        (assign74240_body20_e113195, ((locals.var_chib_dn0 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn0 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn2 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn4 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn5 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn6 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn7 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn8 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn9 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn10 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn11 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign74240_body20_e113194) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign74240_body20_e113192) + (assign74240_body20_e113181 * (-(((locals.var_chib_dn14 / 3.0) * assign74240_body20_e113190) + (assign74240_body20_e113185 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign74240_body20_e113197;
            locals.var_t3_dn0 = assign74240_body20_e113197_d_n0;
            locals.var_t3_dn2 = assign74240_body20_e113197_d_n2;
            locals.var_t3_dn4 = assign74240_body20_e113197_d_n4;
            locals.var_t3_dn5 = assign74240_body20_e113197_d_n5;
            locals.var_t3_dn6 = assign74240_body20_e113197_d_n6;
            locals.var_t3_dn7 = assign74240_body20_e113197_d_n7;
            locals.var_t3_dn8 = assign74240_body20_e113197_d_n8;
            locals.var_t3_dn9 = assign74240_body20_e113197_d_n9;
            locals.var_t3_dn10 = assign74240_body20_e113197_d_n10;
            locals.var_t3_dn11 = assign74240_body20_e113197_d_n11;
            locals.var_t3_dn14 = assign74240_body20_e113197_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign74240_body21_e113207, assign74240_body21_e113207_d_n0, assign74240_body21_e113207_d_n2, assign74240_body21_e113207_d_n4, assign74240_body21_e113207_d_n5, assign74240_body21_e113207_d_n6, assign74240_body21_e113207_d_n7, assign74240_body21_e113207_d_n8, assign74240_body21_e113207_d_n9, assign74240_body21_e113207_d_n10, assign74240_body21_e113207_d_n11, assign74240_body21_e113207_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body21_e113205: f64 = (locals.var_t0 - locals.var_t2);
        (assign74240_body21_e113205, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn11, locals.var_fbsq_dn14,)
    }
};
            locals.var_fbsq = assign74240_body21_e113207;
            locals.var_fbsq_dn0 = assign74240_body21_e113207_d_n0;
            locals.var_fbsq_dn2 = assign74240_body21_e113207_d_n2;
            locals.var_fbsq_dn4 = assign74240_body21_e113207_d_n4;
            locals.var_fbsq_dn5 = assign74240_body21_e113207_d_n5;
            locals.var_fbsq_dn6 = assign74240_body21_e113207_d_n6;
            locals.var_fbsq_dn7 = assign74240_body21_e113207_d_n7;
            locals.var_fbsq_dn8 = assign74240_body21_e113207_d_n8;
            locals.var_fbsq_dn9 = assign74240_body21_e113207_d_n9;
            locals.var_fbsq_dn10 = assign74240_body21_e113207_d_n10;
            locals.var_fbsq_dn11 = assign74240_body21_e113207_d_n11;
            locals.var_fbsq_dn14 = assign74240_body21_e113207_d_n14;
            locals.var_fbsq_rv = 0.0;
            let (assign74240_body22_e113221, assign74240_body22_e113221_d_n0, assign74240_body22_e113221_d_n2, assign74240_body22_e113221_d_n4, assign74240_body22_e113221_d_n5, assign74240_body22_e113221_d_n6, assign74240_body22_e113221_d_n7, assign74240_body22_e113221_d_n8, assign74240_body22_e113221_d_n9, assign74240_body22_e113221_d_n10, assign74240_body22_e113221_d_n11, assign74240_body22_e113221_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign74240_body22_e113217: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign74240_body22_e113218: f64 = (locals.var_t1 - assign74240_body22_e113217);
        let assign74240_body22_e113219: f64 = (locals.var_beta * assign74240_body22_e113218);
        (assign74240_body22_e113219, ((locals.var_beta_dn0 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn11 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))), ((locals.var_beta_dn14 * assign74240_body22_e113218) + (locals.var_beta * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))),)
    } else {
        (locals.var_fbsq_dpss, locals.var_fbsq_dpss_dn0, locals.var_fbsq_dpss_dn2, locals.var_fbsq_dpss_dn4, locals.var_fbsq_dpss_dn5, locals.var_fbsq_dpss_dn6, locals.var_fbsq_dpss_dn7, locals.var_fbsq_dpss_dn8, locals.var_fbsq_dpss_dn9, locals.var_fbsq_dpss_dn10, locals.var_fbsq_dpss_dn11, locals.var_fbsq_dpss_dn14,)
    }
};
            locals.var_fbsq_dpss = assign74240_body22_e113221;
            locals.var_fbsq_dpss_dn0 = assign74240_body22_e113221_d_n0;
            locals.var_fbsq_dpss_dn2 = assign74240_body22_e113221_d_n2;
            locals.var_fbsq_dpss_dn4 = assign74240_body22_e113221_d_n4;
            locals.var_fbsq_dpss_dn5 = assign74240_body22_e113221_d_n5;
            locals.var_fbsq_dpss_dn6 = assign74240_body22_e113221_d_n6;
            locals.var_fbsq_dpss_dn7 = assign74240_body22_e113221_d_n7;
            locals.var_fbsq_dpss_dn8 = assign74240_body22_e113221_d_n8;
            locals.var_fbsq_dpss_dn9 = assign74240_body22_e113221_d_n9;
            locals.var_fbsq_dpss_dn10 = assign74240_body22_e113221_d_n10;
            locals.var_fbsq_dpss_dn11 = assign74240_body22_e113221_d_n11;
            locals.var_fbsq_dpss_dn14 = assign74240_body22_e113221_d_n14;
            locals.var_fbsq_dpss_rv = 0.0;
            let (assign74240_body24_e113249, assign74240_body24_e113249_d_n0, assign74240_body24_e113249_d_n2, assign74240_body24_e113249_d_n4, assign74240_body24_e113249_d_n5, assign74240_body24_e113249_d_n6, assign74240_body24_e113249_d_n7, assign74240_body24_e113249_d_n8, assign74240_body24_e113249_d_n9, assign74240_body24_e113249_d_n10, assign74240_body24_e113249_d_n11, assign74240_body24_e113249_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 == 0.0)) {
        let assign74240_body24_e113246: f64 = (-locals.var_chi);
        let assign74240_body24_e113247: f64 = (assign74240_body24_e113246).exp();
        (assign74240_body24_e113247, (assign74240_body24_e113247 * (-locals.var_chi_dn0)), (assign74240_body24_e113247 * (-locals.var_chi_dn2)), (assign74240_body24_e113247 * (-locals.var_chi_dn4)), (assign74240_body24_e113247 * (-locals.var_chi_dn5)), (assign74240_body24_e113247 * (-locals.var_chi_dn6)), (assign74240_body24_e113247 * (-locals.var_chi_dn7)), (assign74240_body24_e113247 * (-locals.var_chi_dn8)), (assign74240_body24_e113247 * (-locals.var_chi_dn9)), (assign74240_body24_e113247 * (-locals.var_chi_dn10)), (assign74240_body24_e113247 * (-locals.var_chi_dn11)), (assign74240_body24_e113247 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign74240_body24_e113249;
            locals.var_t0_dn0 = assign74240_body24_e113249_d_n0;
            locals.var_t0_dn2 = assign74240_body24_e113249_d_n2;
            locals.var_t0_dn4 = assign74240_body24_e113249_d_n4;
            locals.var_t0_dn5 = assign74240_body24_e113249_d_n5;
            locals.var_t0_dn6 = assign74240_body24_e113249_d_n6;
            locals.var_t0_dn7 = assign74240_body24_e113249_d_n7;
            locals.var_t0_dn8 = assign74240_body24_e113249_d_n8;
            locals.var_t0_dn9 = assign74240_body24_e113249_d_n9;
            locals.var_t0_dn10 = assign74240_body24_e113249_d_n10;
            locals.var_t0_dn11 = assign74240_body24_e113249_d_n11;
            locals.var_t0_dn14 = assign74240_body24_e113249_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign74240_body25_e113260, assign74240_body25_e113260_d_n0, assign74240_body25_e113260_d_n2, assign74240_body25_e113260_d_n4, assign74240_body25_e113260_d_n5, assign74240_body25_e113260_d_n6, assign74240_body25_e113260_d_n7, assign74240_body25_e113260_d_n8, assign74240_body25_e113260_d_n9, assign74240_body25_e113260_d_n10, assign74240_body25_e113260_d_n11, assign74240_body25_e113260_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 == 0.0)) {
        let assign74240_body25_e113257: f64 = (-locals.var_chib);
        let assign74240_body25_e113258: f64 = (assign74240_body25_e113257).exp();
        (assign74240_body25_e113258, (assign74240_body25_e113258 * (-locals.var_chib_dn0)), (assign74240_body25_e113258 * (-locals.var_chib_dn2)), (assign74240_body25_e113258 * (-locals.var_chib_dn4)), (assign74240_body25_e113258 * (-locals.var_chib_dn5)), (assign74240_body25_e113258 * (-locals.var_chib_dn6)), (assign74240_body25_e113258 * (-locals.var_chib_dn7)), (assign74240_body25_e113258 * (-locals.var_chib_dn8)), (assign74240_body25_e113258 * (-locals.var_chib_dn9)), (assign74240_body25_e113258 * (-locals.var_chib_dn10)), (assign74240_body25_e113258 * (-locals.var_chib_dn11)), (assign74240_body25_e113258 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign74240_body25_e113260;
            locals.var_t1_dn0 = assign74240_body25_e113260_d_n0;
            locals.var_t1_dn2 = assign74240_body25_e113260_d_n2;
            locals.var_t1_dn4 = assign74240_body25_e113260_d_n4;
            locals.var_t1_dn5 = assign74240_body25_e113260_d_n5;
            locals.var_t1_dn6 = assign74240_body25_e113260_d_n6;
            locals.var_t1_dn7 = assign74240_body25_e113260_d_n7;
            locals.var_t1_dn8 = assign74240_body25_e113260_d_n8;
            locals.var_t1_dn9 = assign74240_body25_e113260_d_n9;
            locals.var_t1_dn10 = assign74240_body25_e113260_d_n10;
            locals.var_t1_dn11 = assign74240_body25_e113260_d_n11;
            locals.var_t1_dn14 = assign74240_body25_e113260_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign74240_body26_e113275, assign74240_body26_e113275_d_n0, assign74240_body26_e113275_d_n2, assign74240_body26_e113275_d_n4, assign74240_body26_e113275_d_n5, assign74240_body26_e113275_d_n6, assign74240_body26_e113275_d_n7, assign74240_body26_e113275_d_n8, assign74240_body26_e113275_d_n9, assign74240_body26_e113275_d_n10, assign74240_body26_e113275_d_n11, assign74240_body26_e113275_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 == 0.0)) {
        let assign74240_body26_e113269: f64 = (locals.var_chi - locals.var_chib);
        let assign74240_body26_e113272: f64 = (locals.var_t0 - locals.var_t1);
        let assign74240_body26_e113273: f64 = (assign74240_body26_e113269 + assign74240_body26_e113272);
        (assign74240_body26_e113273, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn11, locals.var_fbsq_dn14,)
    }
};
            locals.var_fbsq = assign74240_body26_e113275;
            locals.var_fbsq_dn0 = assign74240_body26_e113275_d_n0;
            locals.var_fbsq_dn2 = assign74240_body26_e113275_d_n2;
            locals.var_fbsq_dn4 = assign74240_body26_e113275_d_n4;
            locals.var_fbsq_dn5 = assign74240_body26_e113275_d_n5;
            locals.var_fbsq_dn6 = assign74240_body26_e113275_d_n6;
            locals.var_fbsq_dn7 = assign74240_body26_e113275_d_n7;
            locals.var_fbsq_dn8 = assign74240_body26_e113275_d_n8;
            locals.var_fbsq_dn9 = assign74240_body26_e113275_d_n9;
            locals.var_fbsq_dn10 = assign74240_body26_e113275_d_n10;
            locals.var_fbsq_dn11 = assign74240_body26_e113275_d_n11;
            locals.var_fbsq_dn14 = assign74240_body26_e113275_d_n14;
            locals.var_fbsq_rv = 0.0;
            let (assign74240_body27_e113294, assign74240_body27_e113294_d_n0, assign74240_body27_e113294_d_n2, assign74240_body27_e113294_d_n4, assign74240_body27_e113294_d_n5, assign74240_body27_e113294_d_n6, assign74240_body27_e113294_d_n7, assign74240_body27_e113294_d_n8, assign74240_body27_e113294_d_n9, assign74240_body27_e113294_d_n10, assign74240_body27_e113294_d_n11, assign74240_body27_e113294_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1732 == 0.0)) {
        let assign74240_body27_e113285: f64 = (1.0 - locals.var_t0);
        let assign74240_body27_e113289: f64 = (1.0 - locals.var_t1);
        let assign74240_body27_e113290: f64 = (locals.var_phi_b_dpss * assign74240_body27_e113289);
        let assign74240_body27_e113291: f64 = (assign74240_body27_e113285 - assign74240_body27_e113290);
        let assign74240_body27_e113292: f64 = (locals.var_beta * assign74240_body27_e113291);
        (assign74240_body27_e113292, ((locals.var_beta_dn0 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn11 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))), ((locals.var_beta_dn14 * assign74240_body27_e113291) + (locals.var_beta * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign74240_body27_e113289) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))),)
    } else {
        (locals.var_fbsq_dpss, locals.var_fbsq_dpss_dn0, locals.var_fbsq_dpss_dn2, locals.var_fbsq_dpss_dn4, locals.var_fbsq_dpss_dn5, locals.var_fbsq_dpss_dn6, locals.var_fbsq_dpss_dn7, locals.var_fbsq_dpss_dn8, locals.var_fbsq_dpss_dn9, locals.var_fbsq_dpss_dn10, locals.var_fbsq_dpss_dn11, locals.var_fbsq_dpss_dn14,)
    }
};
            locals.var_fbsq_dpss = assign74240_body27_e113294;
            locals.var_fbsq_dpss_dn0 = assign74240_body27_e113294_d_n0;
            locals.var_fbsq_dpss_dn2 = assign74240_body27_e113294_d_n2;
            locals.var_fbsq_dpss_dn4 = assign74240_body27_e113294_d_n4;
            locals.var_fbsq_dpss_dn5 = assign74240_body27_e113294_d_n5;
            locals.var_fbsq_dpss_dn6 = assign74240_body27_e113294_d_n6;
            locals.var_fbsq_dpss_dn7 = assign74240_body27_e113294_d_n7;
            locals.var_fbsq_dpss_dn8 = assign74240_body27_e113294_d_n8;
            locals.var_fbsq_dpss_dn9 = assign74240_body27_e113294_d_n9;
            locals.var_fbsq_dpss_dn10 = assign74240_body27_e113294_d_n10;
            locals.var_fbsq_dpss_dn11 = assign74240_body27_e113294_d_n11;
            locals.var_fbsq_dpss_dn14 = assign74240_body27_e113294_d_n14;
            locals.var_fbsq_dpss_rv = 0.0;
            let assign74240_body28_e113296: f64 = (locals.var_chi).abs();
            let assign74240_body28_e113298: f64 = if assign74240_body28_e113296 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1733 = assign74240_body28_e113298;
            locals.var_guard1733_rv = 0.0;
            let (assign74240_body29_e113328, assign74240_body29_e113328_d_n0, assign74240_body29_e113328_d_n2, assign74240_body29_e113328_d_n4, assign74240_body29_e113328_d_n5, assign74240_body29_e113328_d_n6, assign74240_body29_e113328_d_n7, assign74240_body29_e113328_d_n8, assign74240_body29_e113328_d_n9, assign74240_body29_e113328_d_n10, assign74240_body29_e113328_d_n11, assign74240_body29_e113328_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign74240_body29_e113306: f64 = (locals.var_chi * locals.var_chi);
        let assign74240_body29_e113308: f64 = (assign74240_body29_e113306 / 2.0);
        let assign74240_body29_e113312: f64 = (locals.var_chi / 3.0);
        let assign74240_body29_e113316: f64 = (locals.var_chi / 4.0);
        let assign74240_body29_e113320: f64 = (locals.var_chi / 5.0);
        let assign74240_body29_e113321: f64 = (1.0 + assign74240_body29_e113320);
        let assign74240_body29_e113322: f64 = (assign74240_body29_e113316 * assign74240_body29_e113321);
        let assign74240_body29_e113323: f64 = (1.0 + assign74240_body29_e113322);
        let assign74240_body29_e113324: f64 = (assign74240_body29_e113312 * assign74240_body29_e113323);
        let assign74240_body29_e113325: f64 = (1.0 + assign74240_body29_e113324);
        let assign74240_body29_e113326: f64 = (assign74240_body29_e113308 * assign74240_body29_e113325);
        (assign74240_body29_e113326, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn0 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn0 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn2 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn2 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn4 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn4 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn5 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn5 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn6 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn6 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn7 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn7 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn8 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn8 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn9 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn9 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn10 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn10 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn11 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn11 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign74240_body29_e113325) + (assign74240_body29_e113308 * (((locals.var_chi_dn14 / 3.0) * assign74240_body29_e113323) + (assign74240_body29_e113312 * (((locals.var_chi_dn14 / 4.0) * assign74240_body29_e113321) + (assign74240_body29_e113316 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign74240_body29_e113328;
            locals.var_t0_dn0 = assign74240_body29_e113328_d_n0;
            locals.var_t0_dn2 = assign74240_body29_e113328_d_n2;
            locals.var_t0_dn4 = assign74240_body29_e113328_d_n4;
            locals.var_t0_dn5 = assign74240_body29_e113328_d_n5;
            locals.var_t0_dn6 = assign74240_body29_e113328_d_n6;
            locals.var_t0_dn7 = assign74240_body29_e113328_d_n7;
            locals.var_t0_dn8 = assign74240_body29_e113328_d_n8;
            locals.var_t0_dn9 = assign74240_body29_e113328_d_n9;
            locals.var_t0_dn10 = assign74240_body29_e113328_d_n10;
            locals.var_t0_dn11 = assign74240_body29_e113328_d_n11;
            locals.var_t0_dn14 = assign74240_body29_e113328_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign74240_body30_e113354, assign74240_body30_e113354_d_n0, assign74240_body30_e113354_d_n2, assign74240_body30_e113354_d_n4, assign74240_body30_e113354_d_n5, assign74240_body30_e113354_d_n6, assign74240_body30_e113354_d_n7, assign74240_body30_e113354_d_n8, assign74240_body30_e113354_d_n9, assign74240_body30_e113354_d_n10, assign74240_body30_e113354_d_n11, assign74240_body30_e113354_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign74240_body30_e113338: f64 = (locals.var_chi / 2.0);
        let assign74240_body30_e113342: f64 = (locals.var_chi / 3.0);
        let assign74240_body30_e113346: f64 = (locals.var_chi / 4.0);
        let assign74240_body30_e113347: f64 = (1.0 + assign74240_body30_e113346);
        let assign74240_body30_e113348: f64 = (assign74240_body30_e113342 * assign74240_body30_e113347);
        let assign74240_body30_e113349: f64 = (1.0 + assign74240_body30_e113348);
        let assign74240_body30_e113350: f64 = (assign74240_body30_e113338 * assign74240_body30_e113349);
        let assign74240_body30_e113351: f64 = (1.0 + assign74240_body30_e113350);
        let assign74240_body30_e113352: f64 = (locals.var_chi * assign74240_body30_e113351);
        (assign74240_body30_e113352, ((locals.var_chi_dn0 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn0 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn2 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn4 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn5 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn6 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn7 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn8 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn9 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn10 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn11 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign74240_body30_e113351) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign74240_body30_e113349) + (assign74240_body30_e113338 * (((locals.var_chi_dn14 / 3.0) * assign74240_body30_e113347) + (assign74240_body30_e113342 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign74240_body30_e113354;
            locals.var_t1_dn0 = assign74240_body30_e113354_d_n0;
            locals.var_t1_dn2 = assign74240_body30_e113354_d_n2;
            locals.var_t1_dn4 = assign74240_body30_e113354_d_n4;
            locals.var_t1_dn5 = assign74240_body30_e113354_d_n5;
            locals.var_t1_dn6 = assign74240_body30_e113354_d_n6;
            locals.var_t1_dn7 = assign74240_body30_e113354_d_n7;
            locals.var_t1_dn8 = assign74240_body30_e113354_d_n8;
            locals.var_t1_dn9 = assign74240_body30_e113354_d_n9;
            locals.var_t1_dn10 = assign74240_body30_e113354_d_n10;
            locals.var_t1_dn11 = assign74240_body30_e113354_d_n11;
            locals.var_t1_dn14 = assign74240_body30_e113354_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign74240_body31_e113364, assign74240_body31_e113364_d_n0, assign74240_body31_e113364_d_n2, assign74240_body31_e113364_d_n4, assign74240_body31_e113364_d_n5, assign74240_body31_e113364_d_n6, assign74240_body31_e113364_d_n7, assign74240_body31_e113364_d_n8, assign74240_body31_e113364_d_n9, assign74240_body31_e113364_d_n10, assign74240_body31_e113364_d_n11, assign74240_body31_e113364_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign74240_body31_e113362: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign74240_body31_e113362, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign74240_body31_e113364;
            locals.var_fs01_dn0 = assign74240_body31_e113364_d_n0;
            locals.var_fs01_dn2 = assign74240_body31_e113364_d_n2;
            locals.var_fs01_dn4 = assign74240_body31_e113364_d_n4;
            locals.var_fs01_dn5 = assign74240_body31_e113364_d_n5;
            locals.var_fs01_dn6 = assign74240_body31_e113364_d_n6;
            locals.var_fs01_dn7 = assign74240_body31_e113364_d_n7;
            locals.var_fs01_dn8 = assign74240_body31_e113364_d_n8;
            locals.var_fs01_dn9 = assign74240_body31_e113364_d_n9;
            locals.var_fs01_dn10 = assign74240_body31_e113364_d_n10;
            locals.var_fs01_dn11 = assign74240_body31_e113364_d_n11;
            locals.var_fs01_dn14 = assign74240_body31_e113364_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign74240_body32_e113376, assign74240_body32_e113376_d_n0, assign74240_body32_e113376_d_n2, assign74240_body32_e113376_d_n4, assign74240_body32_e113376_d_n5, assign74240_body32_e113376_d_n6, assign74240_body32_e113376_d_n7, assign74240_body32_e113376_d_n8, assign74240_body32_e113376_d_n9, assign74240_body32_e113376_d_n10, assign74240_body32_e113376_d_n11, assign74240_body32_e113376_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign74240_body32_e113372: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign74240_body32_e113374: f64 = (assign74240_body32_e113372 * locals.var_beta);
        (assign74240_body32_e113374, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign74240_body32_e113372 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign74240_body32_e113376;
            locals.var_fs01_dps0_dn0 = assign74240_body32_e113376_d_n0;
            locals.var_fs01_dps0_dn2 = assign74240_body32_e113376_d_n2;
            locals.var_fs01_dps0_dn4 = assign74240_body32_e113376_d_n4;
            locals.var_fs01_dps0_dn5 = assign74240_body32_e113376_d_n5;
            locals.var_fs01_dps0_dn6 = assign74240_body32_e113376_d_n6;
            locals.var_fs01_dps0_dn7 = assign74240_body32_e113376_d_n7;
            locals.var_fs01_dps0_dn8 = assign74240_body32_e113376_d_n8;
            locals.var_fs01_dps0_dn9 = assign74240_body32_e113376_d_n9;
            locals.var_fs01_dps0_dn10 = assign74240_body32_e113376_d_n10;
            locals.var_fs01_dps0_dn11 = assign74240_body32_e113376_d_n11;
            locals.var_fs01_dps0_dn14 = assign74240_body32_e113376_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign74240_body33_e113378: f64 = (locals.var_chi).abs();
            let assign74240_body33_e113380: f64 = if assign74240_body33_e113378 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1734 = assign74240_body33_e113380;
            locals.var_guard1734_rv = 0.0;
            let (assign74240_body35_e113411, assign74240_body35_e113411_d_n0, assign74240_body35_e113411_d_n2, assign74240_body35_e113411_d_n4, assign74240_body35_e113411_d_n5, assign74240_body35_e113411_d_n6, assign74240_body35_e113411_d_n7, assign74240_body35_e113411_d_n8, assign74240_body35_e113411_d_n9, assign74240_body35_e113411_d_n10, assign74240_body35_e113411_d_n11, assign74240_body35_e113411_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        let assign74240_body35_e113409: f64 = (locals.var_chi).exp();
        (assign74240_body35_e113409, (assign74240_body35_e113409 * locals.var_chi_dn0), (assign74240_body35_e113409 * locals.var_chi_dn2), (assign74240_body35_e113409 * locals.var_chi_dn4), (assign74240_body35_e113409 * locals.var_chi_dn5), (assign74240_body35_e113409 * locals.var_chi_dn6), (assign74240_body35_e113409 * locals.var_chi_dn7), (assign74240_body35_e113409 * locals.var_chi_dn8), (assign74240_body35_e113409 * locals.var_chi_dn9), (assign74240_body35_e113409 * locals.var_chi_dn10), (assign74240_body35_e113409 * locals.var_chi_dn11), (assign74240_body35_e113409 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign74240_body35_e113411;
            locals.var_exp_chi_dn0 = assign74240_body35_e113411_d_n0;
            locals.var_exp_chi_dn2 = assign74240_body35_e113411_d_n2;
            locals.var_exp_chi_dn4 = assign74240_body35_e113411_d_n4;
            locals.var_exp_chi_dn5 = assign74240_body35_e113411_d_n5;
            locals.var_exp_chi_dn6 = assign74240_body35_e113411_d_n6;
            locals.var_exp_chi_dn7 = assign74240_body35_e113411_d_n7;
            locals.var_exp_chi_dn8 = assign74240_body35_e113411_d_n8;
            locals.var_exp_chi_dn9 = assign74240_body35_e113411_d_n9;
            locals.var_exp_chi_dn10 = assign74240_body35_e113411_d_n10;
            locals.var_exp_chi_dn11 = assign74240_body35_e113411_d_n11;
            locals.var_exp_chi_dn14 = assign74240_body35_e113411_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign74240_body36_e113424, assign74240_body36_e113424_d_n0, assign74240_body36_e113424_d_n2, assign74240_body36_e113424_d_n4, assign74240_body36_e113424_d_n5, assign74240_body36_e113424_d_n6, assign74240_body36_e113424_d_n7, assign74240_body36_e113424_d_n8, assign74240_body36_e113424_d_n9, assign74240_body36_e113424_d_n10, assign74240_body36_e113424_d_n11, assign74240_body36_e113424_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        let assign74240_body36_e113422: f64 = (locals.var_exp_chi - 1.0);
        (assign74240_body36_e113422, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign74240_body36_e113424;
            locals.var_t1_dn0 = assign74240_body36_e113424_d_n0;
            locals.var_t1_dn2 = assign74240_body36_e113424_d_n2;
            locals.var_t1_dn4 = assign74240_body36_e113424_d_n4;
            locals.var_t1_dn5 = assign74240_body36_e113424_d_n5;
            locals.var_t1_dn6 = assign74240_body36_e113424_d_n6;
            locals.var_t1_dn7 = assign74240_body36_e113424_d_n7;
            locals.var_t1_dn8 = assign74240_body36_e113424_d_n8;
            locals.var_t1_dn9 = assign74240_body36_e113424_d_n9;
            locals.var_t1_dn10 = assign74240_body36_e113424_d_n10;
            locals.var_t1_dn11 = assign74240_body36_e113424_d_n11;
            locals.var_t1_dn14 = assign74240_body36_e113424_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign74240_body37_e113439, assign74240_body37_e113439_d_n0, assign74240_body37_e113439_d_n2, assign74240_body37_e113439_d_n4, assign74240_body37_e113439_d_n5, assign74240_body37_e113439_d_n6, assign74240_body37_e113439_d_n7, assign74240_body37_e113439_d_n8, assign74240_body37_e113439_d_n9, assign74240_body37_e113439_d_n10, assign74240_body37_e113439_d_n11, assign74240_body37_e113439_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        let assign74240_body37_e113436: f64 = (locals.var_t1 - locals.var_chi);
        let assign74240_body37_e113437: f64 = (locals.var_cfs1 * assign74240_body37_e113436);
        (assign74240_body37_e113437, ((locals.var_cfs1_dn0 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign74240_body37_e113436) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign74240_body37_e113439;
            locals.var_fs01_dn0 = assign74240_body37_e113439_d_n0;
            locals.var_fs01_dn2 = assign74240_body37_e113439_d_n2;
            locals.var_fs01_dn4 = assign74240_body37_e113439_d_n4;
            locals.var_fs01_dn5 = assign74240_body37_e113439_d_n5;
            locals.var_fs01_dn6 = assign74240_body37_e113439_d_n6;
            locals.var_fs01_dn7 = assign74240_body37_e113439_d_n7;
            locals.var_fs01_dn8 = assign74240_body37_e113439_d_n8;
            locals.var_fs01_dn9 = assign74240_body37_e113439_d_n9;
            locals.var_fs01_dn10 = assign74240_body37_e113439_d_n10;
            locals.var_fs01_dn11 = assign74240_body37_e113439_d_n11;
            locals.var_fs01_dn14 = assign74240_body37_e113439_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign74240_body38_e113454, assign74240_body38_e113454_d_n0, assign74240_body38_e113454_d_n2, assign74240_body38_e113454_d_n4, assign74240_body38_e113454_d_n5, assign74240_body38_e113454_d_n6, assign74240_body38_e113454_d_n7, assign74240_body38_e113454_d_n8, assign74240_body38_e113454_d_n9, assign74240_body38_e113454_d_n10, assign74240_body38_e113454_d_n11, assign74240_body38_e113454_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        let assign74240_body38_e113450: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign74240_body38_e113452: f64 = (assign74240_body38_e113450 * locals.var_t1);
        (assign74240_body38_e113452, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign74240_body38_e113450 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign74240_body38_e113454;
            locals.var_fs01_dps0_dn0 = assign74240_body38_e113454_d_n0;
            locals.var_fs01_dps0_dn2 = assign74240_body38_e113454_d_n2;
            locals.var_fs01_dps0_dn4 = assign74240_body38_e113454_d_n4;
            locals.var_fs01_dps0_dn5 = assign74240_body38_e113454_d_n5;
            locals.var_fs01_dps0_dn6 = assign74240_body38_e113454_d_n6;
            locals.var_fs01_dps0_dn7 = assign74240_body38_e113454_d_n7;
            locals.var_fs01_dps0_dn8 = assign74240_body38_e113454_d_n8;
            locals.var_fs01_dps0_dn9 = assign74240_body38_e113454_d_n9;
            locals.var_fs01_dps0_dn10 = assign74240_body38_e113454_d_n10;
            locals.var_fs01_dps0_dn11 = assign74240_body38_e113454_d_n11;
            locals.var_fs01_dps0_dn14 = assign74240_body38_e113454_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign74240_body40_e113489, assign74240_body40_e113489_d_n0, assign74240_body40_e113489_d_n2, assign74240_body40_e113489_d_n4, assign74240_body40_e113489_d_n5, assign74240_body40_e113489_d_n6, assign74240_body40_e113489_d_n7, assign74240_body40_e113489_d_n8, assign74240_body40_e113489_d_n9, assign74240_body40_e113489_d_n10, assign74240_body40_e113489_d_n11, assign74240_body40_e113489_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 == 0.0)) {
        let assign74240_body40_e113486: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign74240_body40_e113487: f64 = (assign74240_body40_e113486).exp();
        (assign74240_body40_e113487, (assign74240_body40_e113487 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign74240_body40_e113487 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign74240_body40_e113487 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign74240_body40_e113487 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign74240_body40_e113487 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign74240_body40_e113487 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign74240_body40_e113487 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign74240_body40_e113487 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign74240_body40_e113487 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign74240_body40_e113487 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign74240_body40_e113487 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign74240_body40_e113489;
            locals.var_exp_bps0_dn0 = assign74240_body40_e113489_d_n0;
            locals.var_exp_bps0_dn2 = assign74240_body40_e113489_d_n2;
            locals.var_exp_bps0_dn4 = assign74240_body40_e113489_d_n4;
            locals.var_exp_bps0_dn5 = assign74240_body40_e113489_d_n5;
            locals.var_exp_bps0_dn6 = assign74240_body40_e113489_d_n6;
            locals.var_exp_bps0_dn7 = assign74240_body40_e113489_d_n7;
            locals.var_exp_bps0_dn8 = assign74240_body40_e113489_d_n8;
            locals.var_exp_bps0_dn9 = assign74240_body40_e113489_d_n9;
            locals.var_exp_bps0_dn10 = assign74240_body40_e113489_d_n10;
            locals.var_exp_bps0_dn11 = assign74240_body40_e113489_d_n11;
            locals.var_exp_bps0_dn14 = assign74240_body40_e113489_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign74240_body41_e113509, assign74240_body41_e113509_d_n0, assign74240_body41_e113509_d_n2, assign74240_body41_e113509_d_n4, assign74240_body41_e113509_d_n5, assign74240_body41_e113509_d_n6, assign74240_body41_e113509_d_n7, assign74240_body41_e113509_d_n8, assign74240_body41_e113509_d_n9, assign74240_body41_e113509_d_n10, assign74240_body41_e113509_d_n11, assign74240_body41_e113509_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 == 0.0)) {
        let assign74240_body41_e113504: f64 = (locals.var_chi + 1.0);
        let assign74240_body41_e113505: f64 = (locals.var_exp_bvbs * assign74240_body41_e113504);
        let assign74240_body41_e113506: f64 = (locals.var_exp_bps0 - assign74240_body41_e113505);
        let assign74240_body41_e113507: f64 = (locals.var_cnst1over * assign74240_body41_e113506);
        (assign74240_body41_e113507, ((locals.var_cnst1over_dn0 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign74240_body41_e113506) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign74240_body41_e113504) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign74240_body41_e113509;
            locals.var_fs01_dn0 = assign74240_body41_e113509_d_n0;
            locals.var_fs01_dn2 = assign74240_body41_e113509_d_n2;
            locals.var_fs01_dn4 = assign74240_body41_e113509_d_n4;
            locals.var_fs01_dn5 = assign74240_body41_e113509_d_n5;
            locals.var_fs01_dn6 = assign74240_body41_e113509_d_n6;
            locals.var_fs01_dn7 = assign74240_body41_e113509_d_n7;
            locals.var_fs01_dn8 = assign74240_body41_e113509_d_n8;
            locals.var_fs01_dn9 = assign74240_body41_e113509_d_n9;
            locals.var_fs01_dn10 = assign74240_body41_e113509_d_n10;
            locals.var_fs01_dn11 = assign74240_body41_e113509_d_n11;
            locals.var_fs01_dn14 = assign74240_body41_e113509_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign74240_body42_e113527, assign74240_body42_e113527_d_n0, assign74240_body42_e113527_d_n2, assign74240_body42_e113527_d_n4, assign74240_body42_e113527_d_n5, assign74240_body42_e113527_d_n6, assign74240_body42_e113527_d_n7, assign74240_body42_e113527_d_n8, assign74240_body42_e113527_d_n9, assign74240_body42_e113527_d_n10, assign74240_body42_e113527_d_n11, assign74240_body42_e113527_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1733 == 0.0)) && (locals.var_guard1734 == 0.0)) {
        let assign74240_body42_e113521: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign74240_body42_e113524: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign74240_body42_e113525: f64 = (assign74240_body42_e113521 * assign74240_body42_e113524);
        (assign74240_body42_e113525, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign74240_body42_e113524) + (assign74240_body42_e113521 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign74240_body42_e113527;
            locals.var_fs01_dps0_dn0 = assign74240_body42_e113527_d_n0;
            locals.var_fs01_dps0_dn2 = assign74240_body42_e113527_d_n2;
            locals.var_fs01_dps0_dn4 = assign74240_body42_e113527_d_n4;
            locals.var_fs01_dps0_dn5 = assign74240_body42_e113527_d_n5;
            locals.var_fs01_dps0_dn6 = assign74240_body42_e113527_d_n6;
            locals.var_fs01_dps0_dn7 = assign74240_body42_e113527_d_n7;
            locals.var_fs01_dps0_dn8 = assign74240_body42_e113527_d_n8;
            locals.var_fs01_dps0_dn9 = assign74240_body42_e113527_d_n9;
            locals.var_fs01_dps0_dn10 = assign74240_body42_e113527_d_n10;
            locals.var_fs01_dps0_dn11 = assign74240_body42_e113527_d_n11;
            locals.var_fs01_dps0_dn14 = assign74240_body42_e113527_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign74240_body43_e113530: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1735 = assign74240_body43_e113530;
            locals.var_guard1735_rv = 0.0;
            let (assign74240_body44_e113541, assign74240_body44_e113541_d_n0, assign74240_body44_e113541_d_n2, assign74240_body44_e113541_d_n4, assign74240_body44_e113541_d_n5, assign74240_body44_e113541_d_n6, assign74240_body44_e113541_d_n7, assign74240_body44_e113541_d_n8, assign74240_body44_e113541_d_n9, assign74240_body44_e113541_d_n10, assign74240_body44_e113541_d_n11, assign74240_body44_e113541_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 != 0.0)) {
        let assign74240_body44_e113538: f64 = (locals.var_fbsq + locals.var_fs01);
        let assign74240_body44_e113539: f64 = (assign74240_body44_e113538).sqrt();
        (assign74240_body44_e113539, ((locals.var_fbsq_dn0 + locals.var_fs01_dn0) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn2 + locals.var_fs01_dn2) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn4 + locals.var_fs01_dn4) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn5 + locals.var_fs01_dn5) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn6 + locals.var_fs01_dn6) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn7 + locals.var_fs01_dn7) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn8 + locals.var_fs01_dn8) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn9 + locals.var_fs01_dn9) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn10 + locals.var_fs01_dn10) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn11 + locals.var_fs01_dn11) / (2.0 * assign74240_body44_e113539)), ((locals.var_fbsq_dn14 + locals.var_fs01_dn14) / (2.0 * assign74240_body44_e113539)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign74240_body44_e113541;
            locals.var_fs02_dn0 = assign74240_body44_e113541_d_n0;
            locals.var_fs02_dn2 = assign74240_body44_e113541_d_n2;
            locals.var_fs02_dn4 = assign74240_body44_e113541_d_n4;
            locals.var_fs02_dn5 = assign74240_body44_e113541_d_n5;
            locals.var_fs02_dn6 = assign74240_body44_e113541_d_n6;
            locals.var_fs02_dn7 = assign74240_body44_e113541_d_n7;
            locals.var_fs02_dn8 = assign74240_body44_e113541_d_n8;
            locals.var_fs02_dn9 = assign74240_body44_e113541_d_n9;
            locals.var_fs02_dn10 = assign74240_body44_e113541_d_n10;
            locals.var_fs02_dn11 = assign74240_body44_e113541_d_n11;
            locals.var_fs02_dn14 = assign74240_body44_e113541_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign74240_body45_e113555, assign74240_body45_e113555_d_n0, assign74240_body45_e113555_d_n2, assign74240_body45_e113555_d_n4, assign74240_body45_e113555_d_n5, assign74240_body45_e113555_d_n6, assign74240_body45_e113555_d_n7, assign74240_body45_e113555_d_n8, assign74240_body45_e113555_d_n9, assign74240_body45_e113555_d_n10, assign74240_body45_e113555_d_n11, assign74240_body45_e113555_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 != 0.0)) {
        let assign74240_body45_e113550: f64 = (locals.var_fbsq_dpss + locals.var_fs01_dps0);
        let assign74240_body45_e113551: f64 = (0.5 * assign74240_body45_e113550);
        let assign74240_body45_e113553: f64 = (assign74240_body45_e113551 / locals.var_fs02);
        (assign74240_body45_e113553, ((((0.5 * (locals.var_fbsq_dpss_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn11 + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss_dn14 + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign74240_body45_e113551 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign74240_body45_e113555;
            locals.var_fs02_dps0_dn0 = assign74240_body45_e113555_d_n0;
            locals.var_fs02_dps0_dn2 = assign74240_body45_e113555_d_n2;
            locals.var_fs02_dps0_dn4 = assign74240_body45_e113555_d_n4;
            locals.var_fs02_dps0_dn5 = assign74240_body45_e113555_d_n5;
            locals.var_fs02_dps0_dn6 = assign74240_body45_e113555_d_n6;
            locals.var_fs02_dps0_dn7 = assign74240_body45_e113555_d_n7;
            locals.var_fs02_dps0_dn8 = assign74240_body45_e113555_d_n8;
            locals.var_fs02_dps0_dn9 = assign74240_body45_e113555_d_n9;
            locals.var_fs02_dps0_dn10 = assign74240_body45_e113555_d_n10;
            locals.var_fs02_dps0_dn11 = assign74240_body45_e113555_d_n11;
            locals.var_fs02_dps0_dn14 = assign74240_body45_e113555_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign74240_body46_e113558: f64 = if locals.var_fbsq > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1736 = assign74240_body46_e113558;
            locals.var_guard1736_rv = 0.0;
            let (assign74240_body47_e113570, assign74240_body47_e113570_d_n0, assign74240_body47_e113570_d_n2, assign74240_body47_e113570_d_n4, assign74240_body47_e113570_d_n5, assign74240_body47_e113570_d_n6, assign74240_body47_e113570_d_n7, assign74240_body47_e113570_d_n8, assign74240_body47_e113570_d_n9, assign74240_body47_e113570_d_n10, assign74240_body47_e113570_d_n11, assign74240_body47_e113570_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 == 0.0)) && (locals.var_guard1736 != 0.0)) {
        let assign74240_body47_e113568: f64 = (locals.var_fbsq).sqrt();
        (assign74240_body47_e113568, (locals.var_fbsq_dn0 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn2 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn4 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn5 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn6 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn7 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn8 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn9 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn10 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn11 / (2.0 * assign74240_body47_e113568)), (locals.var_fbsq_dn14 / (2.0 * assign74240_body47_e113568)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign74240_body47_e113570;
            locals.var_fs02_dn0 = assign74240_body47_e113570_d_n0;
            locals.var_fs02_dn2 = assign74240_body47_e113570_d_n2;
            locals.var_fs02_dn4 = assign74240_body47_e113570_d_n4;
            locals.var_fs02_dn5 = assign74240_body47_e113570_d_n5;
            locals.var_fs02_dn6 = assign74240_body47_e113570_d_n6;
            locals.var_fs02_dn7 = assign74240_body47_e113570_d_n7;
            locals.var_fs02_dn8 = assign74240_body47_e113570_d_n8;
            locals.var_fs02_dn9 = assign74240_body47_e113570_d_n9;
            locals.var_fs02_dn10 = assign74240_body47_e113570_d_n10;
            locals.var_fs02_dn11 = assign74240_body47_e113570_d_n11;
            locals.var_fs02_dn14 = assign74240_body47_e113570_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign74240_body48_e113585, assign74240_body48_e113585_d_n0, assign74240_body48_e113585_d_n2, assign74240_body48_e113585_d_n4, assign74240_body48_e113585_d_n5, assign74240_body48_e113585_d_n6, assign74240_body48_e113585_d_n7, assign74240_body48_e113585_d_n8, assign74240_body48_e113585_d_n9, assign74240_body48_e113585_d_n10, assign74240_body48_e113585_d_n11, assign74240_body48_e113585_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 == 0.0)) && (locals.var_guard1736 != 0.0)) {
        let assign74240_body48_e113581: f64 = (0.5 * locals.var_fbsq_dpss);
        let assign74240_body48_e113583: f64 = (assign74240_body48_e113581 / locals.var_fs02);
        (assign74240_body48_e113583, ((((0.5 * locals.var_fbsq_dpss_dn0) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn2) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn4) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn5) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn6) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn7) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn8) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn9) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn10) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn11) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss_dn14) * locals.var_fs02) - (assign74240_body48_e113581 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign74240_body48_e113585;
            locals.var_fs02_dps0_dn0 = assign74240_body48_e113585_d_n0;
            locals.var_fs02_dps0_dn2 = assign74240_body48_e113585_d_n2;
            locals.var_fs02_dps0_dn4 = assign74240_body48_e113585_d_n4;
            locals.var_fs02_dps0_dn5 = assign74240_body48_e113585_d_n5;
            locals.var_fs02_dps0_dn6 = assign74240_body48_e113585_d_n6;
            locals.var_fs02_dps0_dn7 = assign74240_body48_e113585_d_n7;
            locals.var_fs02_dps0_dn8 = assign74240_body48_e113585_d_n8;
            locals.var_fs02_dps0_dn9 = assign74240_body48_e113585_d_n9;
            locals.var_fs02_dps0_dn10 = assign74240_body48_e113585_d_n10;
            locals.var_fs02_dps0_dn11 = assign74240_body48_e113585_d_n11;
            locals.var_fs02_dps0_dn14 = assign74240_body48_e113585_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74240_body49_e113597, assign74240_body49_e113597_d_n0, assign74240_body49_e113597_d_n2, assign74240_body49_e113597_d_n4, assign74240_body49_e113597_d_n5, assign74240_body49_e113597_d_n6, assign74240_body49_e113597_d_n7, assign74240_body49_e113597_d_n8, assign74240_body49_e113597_d_n9, assign74240_body49_e113597_d_n10, assign74240_body49_e113597_d_n11, assign74240_body49_e113597_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 == 0.0)) && (locals.var_guard1736 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign74240_body49_e113597;
            locals.var_fs02_dn0 = assign74240_body49_e113597_d_n0;
            locals.var_fs02_dn2 = assign74240_body49_e113597_d_n2;
            locals.var_fs02_dn4 = assign74240_body49_e113597_d_n4;
            locals.var_fs02_dn5 = assign74240_body49_e113597_d_n5;
            locals.var_fs02_dn6 = assign74240_body49_e113597_d_n6;
            locals.var_fs02_dn7 = assign74240_body49_e113597_d_n7;
            locals.var_fs02_dn8 = assign74240_body49_e113597_d_n8;
            locals.var_fs02_dn9 = assign74240_body49_e113597_d_n9;
            locals.var_fs02_dn10 = assign74240_body49_e113597_d_n10;
            locals.var_fs02_dn11 = assign74240_body49_e113597_d_n11;
            locals.var_fs02_dn14 = assign74240_body49_e113597_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign74240_body50_e113609, assign74240_body50_e113609_d_n0, assign74240_body50_e113609_d_n2, assign74240_body50_e113609_d_n4, assign74240_body50_e113609_d_n5, assign74240_body50_e113609_d_n6, assign74240_body50_e113609_d_n7, assign74240_body50_e113609_d_n8, assign74240_body50_e113609_d_n9, assign74240_body50_e113609_d_n10, assign74240_body50_e113609_d_n11, assign74240_body50_e113609_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1735 == 0.0)) && (locals.var_guard1736 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign74240_body50_e113609;
            locals.var_fs02_dps0_dn0 = assign74240_body50_e113609_d_n0;
            locals.var_fs02_dps0_dn2 = assign74240_body50_e113609_d_n2;
            locals.var_fs02_dps0_dn4 = assign74240_body50_e113609_d_n4;
            locals.var_fs02_dps0_dn5 = assign74240_body50_e113609_d_n5;
            locals.var_fs02_dps0_dn6 = assign74240_body50_e113609_d_n6;
            locals.var_fs02_dps0_dn7 = assign74240_body50_e113609_d_n7;
            locals.var_fs02_dps0_dn8 = assign74240_body50_e113609_d_n8;
            locals.var_fs02_dps0_dn9 = assign74240_body50_e113609_d_n9;
            locals.var_fs02_dps0_dn10 = assign74240_body50_e113609_d_n10;
            locals.var_fs02_dps0_dn11 = assign74240_body50_e113609_d_n11;
            locals.var_fs02_dps0_dn14 = assign74240_body50_e113609_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74240_body51_e113623, assign74240_body51_e113623_d_n0, assign74240_body51_e113623_d_n2, assign74240_body51_e113623_d_n4, assign74240_body51_e113623_d_n5, assign74240_body51_e113623_d_n6, assign74240_body51_e113623_d_n7, assign74240_body51_e113623_d_n8, assign74240_body51_e113623_d_n9, assign74240_body51_e113623_d_n10, assign74240_body51_e113623_d_n11, assign74240_body51_e113623_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let (assign74240_body51_e113619,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign74240_body51_e113618: f64 = (-1.0);
                (assign74240_body51_e113618,)
            }
        };
        let assign74240_body51_e113621: f64 = (assign74240_body51_e113619 * locals.var_fs02);
        (assign74240_body51_e113621, (assign74240_body51_e113619 * locals.var_fs02_dn0), (assign74240_body51_e113619 * locals.var_fs02_dn2), (assign74240_body51_e113619 * locals.var_fs02_dn4), (assign74240_body51_e113619 * locals.var_fs02_dn5), (assign74240_body51_e113619 * locals.var_fs02_dn6), (assign74240_body51_e113619 * locals.var_fs02_dn7), (assign74240_body51_e113619 * locals.var_fs02_dn8), (assign74240_body51_e113619 * locals.var_fs02_dn9), (assign74240_body51_e113619 * locals.var_fs02_dn10), (assign74240_body51_e113619 * locals.var_fs02_dn11), (assign74240_body51_e113619 * locals.var_fs02_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign74240_body51_e113623;
            locals.var_fs02_dn0 = assign74240_body51_e113623_d_n0;
            locals.var_fs02_dn2 = assign74240_body51_e113623_d_n2;
            locals.var_fs02_dn4 = assign74240_body51_e113623_d_n4;
            locals.var_fs02_dn5 = assign74240_body51_e113623_d_n5;
            locals.var_fs02_dn6 = assign74240_body51_e113623_d_n6;
            locals.var_fs02_dn7 = assign74240_body51_e113623_d_n7;
            locals.var_fs02_dn8 = assign74240_body51_e113623_d_n8;
            locals.var_fs02_dn9 = assign74240_body51_e113623_d_n9;
            locals.var_fs02_dn10 = assign74240_body51_e113623_d_n10;
            locals.var_fs02_dn11 = assign74240_body51_e113623_d_n11;
            locals.var_fs02_dn14 = assign74240_body51_e113623_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign74240_body52_e113637, assign74240_body52_e113637_d_n0, assign74240_body52_e113637_d_n2, assign74240_body52_e113637_d_n4, assign74240_body52_e113637_d_n5, assign74240_body52_e113637_d_n6, assign74240_body52_e113637_d_n7, assign74240_body52_e113637_d_n8, assign74240_body52_e113637_d_n9, assign74240_body52_e113637_d_n10, assign74240_body52_e113637_d_n11, assign74240_body52_e113637_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let (assign74240_body52_e113633,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign74240_body52_e113632: f64 = (-1.0);
                (assign74240_body52_e113632,)
            }
        };
        let assign74240_body52_e113635: f64 = (assign74240_body52_e113633 * locals.var_fs02_dps0);
        (assign74240_body52_e113635, (assign74240_body52_e113633 * locals.var_fs02_dps0_dn0), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn2), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn4), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn5), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn6), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn7), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn8), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn9), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn10), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn11), (assign74240_body52_e113633 * locals.var_fs02_dps0_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign74240_body52_e113637;
            locals.var_fs02_dps0_dn0 = assign74240_body52_e113637_d_n0;
            locals.var_fs02_dps0_dn2 = assign74240_body52_e113637_d_n2;
            locals.var_fs02_dps0_dn4 = assign74240_body52_e113637_d_n4;
            locals.var_fs02_dps0_dn5 = assign74240_body52_e113637_d_n5;
            locals.var_fs02_dps0_dn6 = assign74240_body52_e113637_d_n6;
            locals.var_fs02_dps0_dn7 = assign74240_body52_e113637_d_n7;
            locals.var_fs02_dps0_dn8 = assign74240_body52_e113637_d_n8;
            locals.var_fs02_dps0_dn9 = assign74240_body52_e113637_d_n9;
            locals.var_fs02_dps0_dn10 = assign74240_body52_e113637_d_n10;
            locals.var_fs02_dps0_dn11 = assign74240_body52_e113637_d_n11;
            locals.var_fs02_dps0_dn14 = assign74240_body52_e113637_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign74240_body53_e113650, assign74240_body53_e113650_d_n0, assign74240_body53_e113650_d_n2, assign74240_body53_e113650_d_n4, assign74240_body53_e113650_d_n5, assign74240_body53_e113650_d_n6, assign74240_body53_e113650_d_n7, assign74240_body53_e113650_d_n8, assign74240_body53_e113650_d_n9, assign74240_body53_e113650_d_n10, assign74240_body53_e113650_d_n11, assign74240_body53_e113650_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body53_e113642: f64 = (-locals.var_vgpld);
        let assign74240_body53_e113644: f64 = (assign74240_body53_e113642 + locals.var_ps0ld);
        let assign74240_body53_e113647: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign74240_body53_e113648: f64 = (assign74240_body53_e113644 + assign74240_body53_e113647);
        (assign74240_body53_e113648, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign74240_body53_e113650;
            locals.var_fs0_dn0 = assign74240_body53_e113650_d_n0;
            locals.var_fs0_dn2 = assign74240_body53_e113650_d_n2;
            locals.var_fs0_dn4 = assign74240_body53_e113650_d_n4;
            locals.var_fs0_dn5 = assign74240_body53_e113650_d_n5;
            locals.var_fs0_dn6 = assign74240_body53_e113650_d_n6;
            locals.var_fs0_dn7 = assign74240_body53_e113650_d_n7;
            locals.var_fs0_dn8 = assign74240_body53_e113650_d_n8;
            locals.var_fs0_dn9 = assign74240_body53_e113650_d_n9;
            locals.var_fs0_dn10 = assign74240_body53_e113650_d_n10;
            locals.var_fs0_dn11 = assign74240_body53_e113650_d_n11;
            locals.var_fs0_dn14 = assign74240_body53_e113650_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign74240_body54_e113660, assign74240_body54_e113660_d_n0, assign74240_body54_e113660_d_n2, assign74240_body54_e113660_d_n4, assign74240_body54_e113660_d_n5, assign74240_body54_e113660_d_n6, assign74240_body54_e113660_d_n7, assign74240_body54_e113660_d_n8, assign74240_body54_e113660_d_n9, assign74240_body54_e113660_d_n10, assign74240_body54_e113660_d_n11, assign74240_body54_e113660_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body54_e113657: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign74240_body54_e113658: f64 = (1.0 + assign74240_body54_e113657);
        (assign74240_body54_e113658, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign74240_body54_e113660;
            locals.var_fs0_dps0_dn0 = assign74240_body54_e113660_d_n0;
            locals.var_fs0_dps0_dn2 = assign74240_body54_e113660_d_n2;
            locals.var_fs0_dps0_dn4 = assign74240_body54_e113660_d_n4;
            locals.var_fs0_dps0_dn5 = assign74240_body54_e113660_d_n5;
            locals.var_fs0_dps0_dn6 = assign74240_body54_e113660_d_n6;
            locals.var_fs0_dps0_dn7 = assign74240_body54_e113660_d_n7;
            locals.var_fs0_dps0_dn8 = assign74240_body54_e113660_d_n8;
            locals.var_fs0_dps0_dn9 = assign74240_body54_e113660_d_n9;
            locals.var_fs0_dps0_dn10 = assign74240_body54_e113660_d_n10;
            locals.var_fs0_dps0_dn11 = assign74240_body54_e113660_d_n11;
            locals.var_fs0_dps0_dn14 = assign74240_body54_e113660_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign74240_body55_e113663: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1737 = assign74240_body55_e113663;
            locals.var_guard1737_rv = 0.0;
            let (assign74240_body56_e113673,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 != 0.0)) {
        let assign74240_body56_e113671: f64 = (locals.var_lp_s0_max + 1.0);
        (assign74240_body56_e113671,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign74240_body56_e113673;
            locals.var_lp_s0_rv = 0.0;
            let (assign74240_body57_e113685, assign74240_body57_e113685_d_n0, assign74240_body57_e113685_d_n2, assign74240_body57_e113685_d_n4, assign74240_body57_e113685_d_n5, assign74240_body57_e113685_d_n6, assign74240_body57_e113685_d_n7, assign74240_body57_e113685_d_n8, assign74240_body57_e113685_d_n9, assign74240_body57_e113685_d_n10, assign74240_body57_e113685_d_n11, assign74240_body57_e113685_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 == 0.0)) {
        let assign74240_body57_e113681: f64 = (-locals.var_fs0);
        let assign74240_body57_e113683: f64 = (assign74240_body57_e113681 / locals.var_fs0_dps0);
        (assign74240_body57_e113683, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign74240_body57_e113681 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign74240_body57_e113685;
            locals.var_dps0_dn0 = assign74240_body57_e113685_d_n0;
            locals.var_dps0_dn2 = assign74240_body57_e113685_d_n2;
            locals.var_dps0_dn4 = assign74240_body57_e113685_d_n4;
            locals.var_dps0_dn5 = assign74240_body57_e113685_d_n5;
            locals.var_dps0_dn6 = assign74240_body57_e113685_d_n6;
            locals.var_dps0_dn7 = assign74240_body57_e113685_d_n7;
            locals.var_dps0_dn8 = assign74240_body57_e113685_d_n8;
            locals.var_dps0_dn9 = assign74240_body57_e113685_d_n9;
            locals.var_dps0_dn10 = assign74240_body57_e113685_d_n10;
            locals.var_dps0_dn11 = assign74240_body57_e113685_d_n11;
            locals.var_dps0_dn14 = assign74240_body57_e113685_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign74240_body58_e113707, assign74240_body58_e113707_d_n0, assign74240_body58_e113707_d_n2, assign74240_body58_e113707_d_n4, assign74240_body58_e113707_d_n5, assign74240_body58_e113707_d_n6, assign74240_body58_e113707_d_n7, assign74240_body58_e113707_d_n8, assign74240_body58_e113707_d_n9, assign74240_body58_e113707_d_n10, assign74240_body58_e113707_d_n11, assign74240_body58_e113707_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 == 0.0)) {
        let assign74240_body58_e113694: f64 = (0.5 * 0.1);
        let assign74240_body58_e113698: f64 = (locals.var_ps0ld).abs();
        let (assign74240_body58_e113703, assign74240_body58_e113703_d_n0, assign74240_body58_e113703_d_n2, assign74240_body58_e113703_d_n4, assign74240_body58_e113703_d_n5, assign74240_body58_e113703_d_n6, assign74240_body58_e113703_d_n7, assign74240_body58_e113703_d_n8, assign74240_body58_e113703_d_n9, assign74240_body58_e113703_d_n10, assign74240_body58_e113703_d_n11, assign74240_body58_e113703_d_n14,) = {
            if (1.0 >= assign74240_body58_e113698) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign74240_body58_e113702: f64 = (locals.var_ps0ld).abs();
                (assign74240_body58_e113702, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign74240_body58_e113704: f64 = (1.0 + assign74240_body58_e113703);
        let assign74240_body58_e113705: f64 = (assign74240_body58_e113694 * assign74240_body58_e113704);
        (assign74240_body58_e113705, (assign74240_body58_e113694 * assign74240_body58_e113703_d_n0), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n2), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n4), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n5), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n6), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n7), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n8), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n9), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n10), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n11), (assign74240_body58_e113694 * assign74240_body58_e113703_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign74240_body58_e113707;
            locals.var_dplim_dn0 = assign74240_body58_e113707_d_n0;
            locals.var_dplim_dn2 = assign74240_body58_e113707_d_n2;
            locals.var_dplim_dn4 = assign74240_body58_e113707_d_n4;
            locals.var_dplim_dn5 = assign74240_body58_e113707_d_n5;
            locals.var_dplim_dn6 = assign74240_body58_e113707_d_n6;
            locals.var_dplim_dn7 = assign74240_body58_e113707_d_n7;
            locals.var_dplim_dn8 = assign74240_body58_e113707_d_n8;
            locals.var_dplim_dn9 = assign74240_body58_e113707_d_n9;
            locals.var_dplim_dn10 = assign74240_body58_e113707_d_n10;
            locals.var_dplim_dn11 = assign74240_body58_e113707_d_n11;
            locals.var_dplim_dn14 = assign74240_body58_e113707_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign74240_body59_e113709: f64 = (locals.var_dps0).abs();
            let assign74240_body59_e113711: f64 = if assign74240_body59_e113709 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1738 = assign74240_body59_e113711;
            locals.var_guard1738_rv = 0.0;
            let (assign74240_body60_e113730, assign74240_body60_e113730_d_n0, assign74240_body60_e113730_d_n2, assign74240_body60_e113730_d_n4, assign74240_body60_e113730_d_n5, assign74240_body60_e113730_d_n6, assign74240_body60_e113730_d_n7, assign74240_body60_e113730_d_n8, assign74240_body60_e113730_d_n9, assign74240_body60_e113730_d_n10, assign74240_body60_e113730_d_n11, assign74240_body60_e113730_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 == 0.0)) && (locals.var_guard1738 != 0.0)) {
        let (assign74240_body60_e113727,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign74240_body60_e113726: f64 = (-1.0);
                (assign74240_body60_e113726,)
            }
        };
        let assign74240_body60_e113728: f64 = (locals.var_dplim * assign74240_body60_e113727);
        (assign74240_body60_e113728, (locals.var_dplim_dn0 * assign74240_body60_e113727), (locals.var_dplim_dn2 * assign74240_body60_e113727), (locals.var_dplim_dn4 * assign74240_body60_e113727), (locals.var_dplim_dn5 * assign74240_body60_e113727), (locals.var_dplim_dn6 * assign74240_body60_e113727), (locals.var_dplim_dn7 * assign74240_body60_e113727), (locals.var_dplim_dn8 * assign74240_body60_e113727), (locals.var_dplim_dn9 * assign74240_body60_e113727), (locals.var_dplim_dn10 * assign74240_body60_e113727), (locals.var_dplim_dn11 * assign74240_body60_e113727), (locals.var_dplim_dn14 * assign74240_body60_e113727),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign74240_body60_e113730;
            locals.var_dps0_dn0 = assign74240_body60_e113730_d_n0;
            locals.var_dps0_dn2 = assign74240_body60_e113730_d_n2;
            locals.var_dps0_dn4 = assign74240_body60_e113730_d_n4;
            locals.var_dps0_dn5 = assign74240_body60_e113730_d_n5;
            locals.var_dps0_dn6 = assign74240_body60_e113730_d_n6;
            locals.var_dps0_dn7 = assign74240_body60_e113730_d_n7;
            locals.var_dps0_dn8 = assign74240_body60_e113730_d_n8;
            locals.var_dps0_dn9 = assign74240_body60_e113730_d_n9;
            locals.var_dps0_dn10 = assign74240_body60_e113730_d_n10;
            locals.var_dps0_dn11 = assign74240_body60_e113730_d_n11;
            locals.var_dps0_dn14 = assign74240_body60_e113730_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign74240_body61_e113741, assign74240_body61_e113741_d_n0, assign74240_body61_e113741_d_n2, assign74240_body61_e113741_d_n4, assign74240_body61_e113741_d_n5, assign74240_body61_e113741_d_n6, assign74240_body61_e113741_d_n7, assign74240_body61_e113741_d_n8, assign74240_body61_e113741_d_n9, assign74240_body61_e113741_d_n10, assign74240_body61_e113741_d_n11, assign74240_body61_e113741_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 == 0.0)) {
        let assign74240_body61_e113739: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign74240_body61_e113739, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign74240_body61_e113741;
            locals.var_ps0ld_dn0 = assign74240_body61_e113741_d_n0;
            locals.var_ps0ld_dn2 = assign74240_body61_e113741_d_n2;
            locals.var_ps0ld_dn4 = assign74240_body61_e113741_d_n4;
            locals.var_ps0ld_dn5 = assign74240_body61_e113741_d_n5;
            locals.var_ps0ld_dn6 = assign74240_body61_e113741_d_n6;
            locals.var_ps0ld_dn7 = assign74240_body61_e113741_d_n7;
            locals.var_ps0ld_dn8 = assign74240_body61_e113741_d_n8;
            locals.var_ps0ld_dn9 = assign74240_body61_e113741_d_n9;
            locals.var_ps0ld_dn10 = assign74240_body61_e113741_d_n10;
            locals.var_ps0ld_dn11 = assign74240_body61_e113741_d_n11;
            locals.var_ps0ld_dn14 = assign74240_body61_e113741_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign74240_body62_e113743: f64 = (locals.var_dps0).abs();
            let assign74240_body62_e113747: f64 = (locals.var_fs0).abs();
            let assign74240_body62_e113750: f64 = if ((assign74240_body62_e113743 <= 1e-12) && (assign74240_body62_e113747 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1739 = assign74240_body62_e113750;
            locals.var_guard1739_rv = 0.0;
            let (assign74240_body63_e113763,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) && (locals.var_guard1737 == 0.0)) && (locals.var_guard1739 != 0.0)) {
        let assign74240_body63_e113761: f64 = (locals.var_flg_conv + 2.0);
        (assign74240_body63_e113761,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign74240_body63_e113763;
            locals.var_flg_conv_rv = 0.0;
            let (assign74240_body64_e113771,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74240_body64_e113769: f64 = (locals.var_lp_s0 + 1.0);
        (assign74240_body64_e113769,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign74240_body64_e113771;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_281(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74260_e113794, assign74260_e113794_d_n0, assign74260_e113794_d_n2, assign74260_e113794_d_n4, assign74260_e113794_d_n5, assign74260_e113794_d_n6, assign74260_e113794_d_n7, assign74260_e113794_d_n8, assign74260_e113794_d_n9, assign74260_e113794_d_n10, assign74260_e113794_d_n11, assign74260_e113794_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let (assign74260_e113792, assign74260_e113792_d_n0, assign74260_e113792_d_n2, assign74260_e113792_d_n4, assign74260_e113792_d_n5, assign74260_e113792_d_n6, assign74260_e113792_d_n7, assign74260_e113792_d_n8, assign74260_e113792_d_n9, assign74260_e113792_d_n10, assign74260_e113792_d_n11, assign74260_e113792_d_n14,) = {
            if (locals.var_fbsq >= 0.0) {
                let (assign74260_e113787,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign74260_e113786: f64 = (-1.0);
                        (assign74260_e113786,)
                    }
                };
                let assign74260_e113789: f64 = (locals.var_fbsq).sqrt();
                let assign74260_e113790: f64 = (assign74260_e113787 * assign74260_e113789);
                (assign74260_e113790, (assign74260_e113787 * (locals.var_fbsq_dn0 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn2 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn4 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn5 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn6 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn7 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn8 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn9 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn10 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn11 / (2.0 * assign74260_e113789))), (assign74260_e113787 * (locals.var_fbsq_dn14 / (2.0 * assign74260_e113789))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign74260_e113792, assign74260_e113792_d_n0, assign74260_e113792_d_n2, assign74260_e113792_d_n4, assign74260_e113792_d_n5, assign74260_e113792_d_n6, assign74260_e113792_d_n7, assign74260_e113792_d_n8, assign74260_e113792_d_n9, assign74260_e113792_d_n10, assign74260_e113792_d_n11, assign74260_e113792_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign74260_e113794;
        locals.var_fb_dn0 = assign74260_e113794_d_n0;
        locals.var_fb_dn2 = assign74260_e113794_d_n2;
        locals.var_fb_dn4 = assign74260_e113794_d_n4;
        locals.var_fb_dn5 = assign74260_e113794_d_n5;
        locals.var_fb_dn6 = assign74260_e113794_d_n6;
        locals.var_fb_dn7 = assign74260_e113794_d_n7;
        locals.var_fb_dn8 = assign74260_e113794_d_n8;
        locals.var_fb_dn9 = assign74260_e113794_d_n9;
        locals.var_fb_dn10 = assign74260_e113794_d_n10;
        locals.var_fb_dn11 = assign74260_e113794_d_n11;
        locals.var_fb_dn14 = assign74260_e113794_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign74270_e113802, assign74270_e113802_d_n0, assign74270_e113802_d_n2, assign74270_e113802_d_n4, assign74270_e113802_d_n5, assign74270_e113802_d_n6, assign74270_e113802_d_n7, assign74270_e113802_d_n8, assign74270_e113802_d_n9, assign74270_e113802_d_n10, assign74270_e113802_d_n11, assign74270_e113802_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74270_e113800: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign74270_e113800, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld, locals.var_wdld_dn0, locals.var_wdld_dn2, locals.var_wdld_dn4, locals.var_wdld_dn5, locals.var_wdld_dn6, locals.var_wdld_dn7, locals.var_wdld_dn8, locals.var_wdld_dn9, locals.var_wdld_dn10, locals.var_wdld_dn11, locals.var_wdld_dn14,)
    }
};
        locals.var_wdld = assign74270_e113802;
        locals.var_wdld_dn0 = assign74270_e113802_d_n0;
        locals.var_wdld_dn2 = assign74270_e113802_d_n2;
        locals.var_wdld_dn4 = assign74270_e113802_d_n4;
        locals.var_wdld_dn5 = assign74270_e113802_d_n5;
        locals.var_wdld_dn6 = assign74270_e113802_d_n6;
        locals.var_wdld_dn7 = assign74270_e113802_d_n7;
        locals.var_wdld_dn8 = assign74270_e113802_d_n8;
        locals.var_wdld_dn9 = assign74270_e113802_d_n9;
        locals.var_wdld_dn10 = assign74270_e113802_d_n10;
        locals.var_wdld_dn11 = assign74270_e113802_d_n11;
        locals.var_wdld_dn14 = assign74270_e113802_d_n14;
        locals.var_wdld_rv = 0.0;

        let (assign74280_e113810, assign74280_e113810_d_n0, assign74280_e113810_d_n2, assign74280_e113810_d_n4, assign74280_e113810_d_n5, assign74280_e113810_d_n6, assign74280_e113810_d_n7, assign74280_e113810_d_n8, assign74280_e113810_d_n9, assign74280_e113810_d_n10, assign74280_e113810_d_n11, assign74280_e113810_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74280_e113808: f64 = (locals.var_q_nsubld * locals.var_wdld);
        (assign74280_e113808, (locals.var_q_nsubld * locals.var_wdld_dn0), (locals.var_q_nsubld * locals.var_wdld_dn2), (locals.var_q_nsubld * locals.var_wdld_dn4), (locals.var_q_nsubld * locals.var_wdld_dn5), (locals.var_q_nsubld * locals.var_wdld_dn6), (locals.var_q_nsubld * locals.var_wdld_dn7), (locals.var_q_nsubld * locals.var_wdld_dn8), (locals.var_q_nsubld * locals.var_wdld_dn9), (locals.var_q_nsubld * locals.var_wdld_dn10), (locals.var_q_nsubld * locals.var_wdld_dn11), (locals.var_q_nsubld * locals.var_wdld_dn14),)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn11, locals.var_q_dep_ld_dn14,)
    }
};
        locals.var_q_dep_ld = assign74280_e113810;
        locals.var_q_dep_ld_dn0 = assign74280_e113810_d_n0;
        locals.var_q_dep_ld_dn2 = assign74280_e113810_d_n2;
        locals.var_q_dep_ld_dn4 = assign74280_e113810_d_n4;
        locals.var_q_dep_ld_dn5 = assign74280_e113810_d_n5;
        locals.var_q_dep_ld_dn6 = assign74280_e113810_d_n6;
        locals.var_q_dep_ld_dn7 = assign74280_e113810_d_n7;
        locals.var_q_dep_ld_dn8 = assign74280_e113810_d_n8;
        locals.var_q_dep_ld_dn9 = assign74280_e113810_d_n9;
        locals.var_q_dep_ld_dn10 = assign74280_e113810_d_n10;
        locals.var_q_dep_ld_dn11 = assign74280_e113810_d_n11;
        locals.var_q_dep_ld_dn14 = assign74280_e113810_d_n14;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign74290_e113822, assign74290_e113822_d_n0, assign74290_e113822_d_n2, assign74290_e113822_d_n4, assign74290_e113822_d_n5, assign74290_e113822_d_n6, assign74290_e113822_d_n7, assign74290_e113822_d_n8, assign74290_e113822_d_n9, assign74290_e113822_d_n10, assign74290_e113822_d_n11, assign74290_e113822_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74290_e113816: f64 = (locals.var_q_dep_ld / locals.var_cnst0over_func);
        let assign74290_e113819: f64 = (10.0 * 2.220446049250313e-16);
        let assign74290_e113820: f64 = (assign74290_e113816 + assign74290_e113819);
        (assign74290_e113820, (((locals.var_q_dep_ld_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign74290_e113822;
        locals.var_xi0p12_dn0 = assign74290_e113822_d_n0;
        locals.var_xi0p12_dn2 = assign74290_e113822_d_n2;
        locals.var_xi0p12_dn4 = assign74290_e113822_d_n4;
        locals.var_xi0p12_dn5 = assign74290_e113822_d_n5;
        locals.var_xi0p12_dn6 = assign74290_e113822_d_n6;
        locals.var_xi0p12_dn7 = assign74290_e113822_d_n7;
        locals.var_xi0p12_dn8 = assign74290_e113822_d_n8;
        locals.var_xi0p12_dn9 = assign74290_e113822_d_n9;
        locals.var_xi0p12_dn10 = assign74290_e113822_d_n10;
        locals.var_xi0p12_dn11 = assign74290_e113822_d_n11;
        locals.var_xi0p12_dn14 = assign74290_e113822_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign74300_e113830, assign74300_e113830_d_n0, assign74300_e113830_d_n2, assign74300_e113830_d_n4, assign74300_e113830_d_n5, assign74300_e113830_d_n6, assign74300_e113830_d_n7, assign74300_e113830_d_n8, assign74300_e113830_d_n9, assign74300_e113830_d_n10, assign74300_e113830_d_n11, assign74300_e113830_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74300_e113828: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign74300_e113828, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign74300_e113830;
        locals.var_qbuld_dn0 = assign74300_e113830_d_n0;
        locals.var_qbuld_dn2 = assign74300_e113830_d_n2;
        locals.var_qbuld_dn4 = assign74300_e113830_d_n4;
        locals.var_qbuld_dn5 = assign74300_e113830_d_n5;
        locals.var_qbuld_dn6 = assign74300_e113830_d_n6;
        locals.var_qbuld_dn7 = assign74300_e113830_d_n7;
        locals.var_qbuld_dn8 = assign74300_e113830_d_n8;
        locals.var_qbuld_dn9 = assign74300_e113830_d_n9;
        locals.var_qbuld_dn10 = assign74300_e113830_d_n10;
        locals.var_qbuld_dn11 = assign74300_e113830_d_n11;
        locals.var_qbuld_dn14 = assign74300_e113830_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign74310_e113840, assign74310_e113840_d_n0, assign74310_e113840_d_n2, assign74310_e113840_d_n4, assign74310_e113840_d_n5, assign74310_e113840_d_n6, assign74310_e113840_d_n7, assign74310_e113840_d_n8, assign74310_e113840_d_n9, assign74310_e113840_d_n10, assign74310_e113840_d_n11, assign74310_e113840_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74310_e113837: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign74310_e113838: f64 = (1.0 / assign74310_e113837);
        (assign74310_e113838, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign74310_e113837 * assign74310_e113837))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign74310_e113837 * assign74310_e113837))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign74310_e113840;
        locals.var_t1_dn0 = assign74310_e113840_d_n0;
        locals.var_t1_dn2 = assign74310_e113840_d_n2;
        locals.var_t1_dn4 = assign74310_e113840_d_n4;
        locals.var_t1_dn5 = assign74310_e113840_d_n5;
        locals.var_t1_dn6 = assign74310_e113840_d_n6;
        locals.var_t1_dn7 = assign74310_e113840_d_n7;
        locals.var_t1_dn8 = assign74310_e113840_d_n8;
        locals.var_t1_dn9 = assign74310_e113840_d_n9;
        locals.var_t1_dn10 = assign74310_e113840_d_n10;
        locals.var_t1_dn11 = assign74310_e113840_d_n11;
        locals.var_t1_dn14 = assign74310_e113840_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign74320_e113850, assign74320_e113850_d_n0, assign74320_e113850_d_n2, assign74320_e113850_d_n4, assign74320_e113850_d_n5, assign74320_e113850_d_n6, assign74320_e113850_d_n7, assign74320_e113850_d_n8, assign74320_e113850_d_n9, assign74320_e113850_d_n10, assign74320_e113850_d_n11, assign74320_e113850_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74320_e113846: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign74320_e113848: f64 = (assign74320_e113846 * locals.var_t1);
        (assign74320_e113848, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign74320_e113846 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign74320_e113850;
        locals.var_qiuld_dn0 = assign74320_e113850_d_n0;
        locals.var_qiuld_dn2 = assign74320_e113850_d_n2;
        locals.var_qiuld_dn4 = assign74320_e113850_d_n4;
        locals.var_qiuld_dn5 = assign74320_e113850_d_n5;
        locals.var_qiuld_dn6 = assign74320_e113850_d_n6;
        locals.var_qiuld_dn7 = assign74320_e113850_d_n7;
        locals.var_qiuld_dn8 = assign74320_e113850_d_n8;
        locals.var_qiuld_dn9 = assign74320_e113850_d_n9;
        locals.var_qiuld_dn10 = assign74320_e113850_d_n10;
        locals.var_qiuld_dn11 = assign74320_e113850_d_n11;
        locals.var_qiuld_dn14 = assign74320_e113850_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign74330_e113858, assign74330_e113858_d_n0, assign74330_e113858_d_n2, assign74330_e113858_d_n4, assign74330_e113858_d_n5, assign74330_e113858_d_n6, assign74330_e113858_d_n7, assign74330_e113858_d_n8, assign74330_e113858_d_n9, assign74330_e113858_d_n10, assign74330_e113858_d_n11, assign74330_e113858_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1726 != 0.0)) {
        let assign74330_e113856: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign74330_e113856, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign74330_e113858;
        locals.var_qsuld_dn0 = assign74330_e113858_d_n0;
        locals.var_qsuld_dn2 = assign74330_e113858_d_n2;
        locals.var_qsuld_dn4 = assign74330_e113858_d_n4;
        locals.var_qsuld_dn5 = assign74330_e113858_d_n5;
        locals.var_qsuld_dn6 = assign74330_e113858_d_n6;
        locals.var_qsuld_dn7 = assign74330_e113858_d_n7;
        locals.var_qsuld_dn8 = assign74330_e113858_d_n8;
        locals.var_qsuld_dn9 = assign74330_e113858_d_n9;
        locals.var_qsuld_dn10 = assign74330_e113858_d_n10;
        locals.var_qsuld_dn11 = assign74330_e113858_d_n11;
        locals.var_qsuld_dn14 = assign74330_e113858_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign74340_e113864, assign74340_e113864_d_n0, assign74340_e113864_d_n2, assign74340_e113864_d_n4, assign74340_e113864_d_n5, assign74340_e113864_d_n6, assign74340_e113864_d_n7, assign74340_e113864_d_n8, assign74340_e113864_d_n9, assign74340_e113864_d_n10, assign74340_e113864_d_n11, assign74340_e113864_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign74340_e113862: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign74340_e113862, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign74340_e113864;
        locals.var_qiuld_dn0 = assign74340_e113864_d_n0;
        locals.var_qiuld_dn2 = assign74340_e113864_d_n2;
        locals.var_qiuld_dn4 = assign74340_e113864_d_n4;
        locals.var_qiuld_dn5 = assign74340_e113864_d_n5;
        locals.var_qiuld_dn6 = assign74340_e113864_d_n6;
        locals.var_qiuld_dn7 = assign74340_e113864_d_n7;
        locals.var_qiuld_dn8 = assign74340_e113864_d_n8;
        locals.var_qiuld_dn9 = assign74340_e113864_d_n9;
        locals.var_qiuld_dn10 = assign74340_e113864_d_n10;
        locals.var_qiuld_dn11 = assign74340_e113864_d_n11;
        locals.var_qiuld_dn14 = assign74340_e113864_d_n14;
        locals.var_qiuld_rv = 0.0;

        let assign74350_e113867: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1741 = assign74350_e113867;
        locals.var_guard1741_rv = 0.0;

        let (assign74360_e113874, assign74360_e113874_d_n0, assign74360_e113874_d_n2, assign74360_e113874_d_n4, assign74360_e113874_d_n5, assign74360_e113874_d_n6, assign74360_e113874_d_n7, assign74360_e113874_d_n8, assign74360_e113874_d_n9, assign74360_e113874_d_n10, assign74360_e113874_d_n11, assign74360_e113874_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) {
        let assign74360_e113872: f64 = (-locals.var_lover_func);
        (assign74360_e113872, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign74360_e113874;
        locals.var_lover_func_dn0 = assign74360_e113874_d_n0;
        locals.var_lover_func_dn2 = assign74360_e113874_d_n2;
        locals.var_lover_func_dn4 = assign74360_e113874_d_n4;
        locals.var_lover_func_dn5 = assign74360_e113874_d_n5;
        locals.var_lover_func_dn6 = assign74360_e113874_d_n6;
        locals.var_lover_func_dn7 = assign74360_e113874_d_n7;
        locals.var_lover_func_dn8 = assign74360_e113874_d_n8;
        locals.var_lover_func_dn9 = assign74360_e113874_d_n9;
        locals.var_lover_func_dn10 = assign74360_e113874_d_n10;
        locals.var_lover_func_dn11 = assign74360_e113874_d_n11;
        locals.var_lover_func_dn14 = assign74360_e113874_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign74370_e113877: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1742 = assign74370_e113877;
        locals.var_guard1742_rv = 0.0;

        let assign74380_e113880: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1743 = assign74380_e113880;
        locals.var_guard1743_rv = 0.0;

        let (assign74390_e113891, assign74390_e113891_d_n0, assign74390_e113891_d_n2, assign74390_e113891_d_n4, assign74390_e113891_d_n5, assign74390_e113891_d_n6, assign74390_e113891_d_n7, assign74390_e113891_d_n8, assign74390_e113891_d_n9, assign74390_e113891_d_n10, assign74390_e113891_d_n11, assign74390_e113891_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) && (locals.var_guard1743 != 0.0)) {
        let assign74390_e113889: f64 = (-locals.var_ps0ld);
        (assign74390_e113889, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx, locals.var_vx_dn0, locals.var_vx_dn2, locals.var_vx_dn4, locals.var_vx_dn5, locals.var_vx_dn6, locals.var_vx_dn7, locals.var_vx_dn8, locals.var_vx_dn9, locals.var_vx_dn10, locals.var_vx_dn11, locals.var_vx_dn14,)
    }
};
        locals.var_vx = assign74390_e113891;
        locals.var_vx_dn0 = assign74390_e113891_d_n0;
        locals.var_vx_dn2 = assign74390_e113891_d_n2;
        locals.var_vx_dn4 = assign74390_e113891_d_n4;
        locals.var_vx_dn5 = assign74390_e113891_d_n5;
        locals.var_vx_dn6 = assign74390_e113891_d_n6;
        locals.var_vx_dn7 = assign74390_e113891_d_n7;
        locals.var_vx_dn8 = assign74390_e113891_d_n8;
        locals.var_vx_dn9 = assign74390_e113891_d_n9;
        locals.var_vx_dn10 = assign74390_e113891_d_n10;
        locals.var_vx_dn11 = assign74390_e113891_d_n11;
        locals.var_vx_dn14 = assign74390_e113891_d_n14;
        locals.var_vx_rv = 0.0;

        let (assign74400_e113902, assign74400_e113902_d_n0, assign74400_e113902_d_n2, assign74400_e113902_d_n4, assign74400_e113902_d_n5, assign74400_e113902_d_n6, assign74400_e113902_d_n7, assign74400_e113902_d_n8, assign74400_e113902_d_n9, assign74400_e113902_d_n10, assign74400_e113902_d_n11, assign74400_e113902_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) && (locals.var_guard1743 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx, locals.var_vx_dn0, locals.var_vx_dn2, locals.var_vx_dn4, locals.var_vx_dn5, locals.var_vx_dn6, locals.var_vx_dn7, locals.var_vx_dn8, locals.var_vx_dn9, locals.var_vx_dn10, locals.var_vx_dn11, locals.var_vx_dn14,)
    }
};
        locals.var_vx = assign74400_e113902;
        locals.var_vx_dn0 = assign74400_e113902_d_n0;
        locals.var_vx_dn2 = assign74400_e113902_d_n2;
        locals.var_vx_dn4 = assign74400_e113902_d_n4;
        locals.var_vx_dn5 = assign74400_e113902_d_n5;
        locals.var_vx_dn6 = assign74400_e113902_d_n6;
        locals.var_vx_dn7 = assign74400_e113902_d_n7;
        locals.var_vx_dn8 = assign74400_e113902_d_n8;
        locals.var_vx_dn9 = assign74400_e113902_d_n9;
        locals.var_vx_dn10 = assign74400_e113902_d_n10;
        locals.var_vx_dn11 = assign74400_e113902_d_n11;
        locals.var_vx_dn14 = assign74400_e113902_d_n14;
        locals.var_vx_rv = 0.0;

        let (assign74410_e113923, assign74410_e113923_d_n0, assign74410_e113923_d_n2, assign74410_e113923_d_n4, assign74410_e113923_d_n5, assign74410_e113923_d_n6, assign74410_e113923_d_n7, assign74410_e113923_d_n8, assign74410_e113923_d_n9, assign74410_e113923_d_n10, assign74410_e113923_d_n11, assign74410_e113923_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74410_e113910: f64 = (locals.var_vx + p.p137);
        let assign74410_e113913: f64 = (locals.var_vx + p.p137);
        let assign74410_e113914: f64 = (assign74410_e113910 * assign74410_e113913);
        let assign74410_e113917: f64 = (4.0 * 0.1);
        let assign74410_e113919: f64 = (assign74410_e113917 * 0.1);
        let assign74410_e113920: f64 = (assign74410_e113914 + assign74410_e113919);
        let assign74410_e113921: f64 = (assign74410_e113920).sqrt();
        (assign74410_e113921, (((locals.var_vx_dn0 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn0)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn2 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn2)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn4 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn4)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn5 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn5)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn6 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn6)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn7 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn7)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn8 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn8)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn9 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn9)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn10 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn10)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn11 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn11)) / (2.0 * assign74410_e113921)), (((locals.var_vx_dn14 * assign74410_e113913) + (assign74410_e113910 * locals.var_vx_dn14)) / (2.0 * assign74410_e113921)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74410_e113923;
        locals.var_tmf2_dn0 = assign74410_e113923_d_n0;
        locals.var_tmf2_dn2 = assign74410_e113923_d_n2;
        locals.var_tmf2_dn4 = assign74410_e113923_d_n4;
        locals.var_tmf2_dn5 = assign74410_e113923_d_n5;
        locals.var_tmf2_dn6 = assign74410_e113923_d_n6;
        locals.var_tmf2_dn7 = assign74410_e113923_d_n7;
        locals.var_tmf2_dn8 = assign74410_e113923_d_n8;
        locals.var_tmf2_dn9 = assign74410_e113923_d_n9;
        locals.var_tmf2_dn10 = assign74410_e113923_d_n10;
        locals.var_tmf2_dn11 = assign74410_e113923_d_n11;
        locals.var_tmf2_dn14 = assign74410_e113923_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign74420_e113939, assign74420_e113939_d_n0, assign74420_e113939_d_n2, assign74420_e113939_d_n4, assign74420_e113939_d_n5, assign74420_e113939_d_n6, assign74420_e113939_d_n7, assign74420_e113939_d_n8, assign74420_e113939_d_n9, assign74420_e113939_d_n10, assign74420_e113939_d_n11, assign74420_e113939_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74420_e113933: f64 = (locals.var_vx + p.p137);
        let assign74420_e113935: f64 = (assign74420_e113933 / locals.var_tmf2);
        let assign74420_e113936: f64 = (1.0 + assign74420_e113935);
        let assign74420_e113937: f64 = (0.5 * assign74420_e113936);
        (assign74420_e113937, (0.5 * (((locals.var_vx_dn0 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn2 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn4 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn5 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn6 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn7 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn8 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn9 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn10 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn11 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx_dn14 * locals.var_tmf2) - (assign74420_e113933 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74420_e113939;
        locals.var_t9_dn0 = assign74420_e113939_d_n0;
        locals.var_t9_dn2 = assign74420_e113939_d_n2;
        locals.var_t9_dn4 = assign74420_e113939_d_n4;
        locals.var_t9_dn5 = assign74420_e113939_d_n5;
        locals.var_t9_dn6 = assign74420_e113939_d_n6;
        locals.var_t9_dn7 = assign74420_e113939_d_n7;
        locals.var_t9_dn8 = assign74420_e113939_d_n8;
        locals.var_t9_dn9 = assign74420_e113939_d_n9;
        locals.var_t9_dn10 = assign74420_e113939_d_n10;
        locals.var_t9_dn11 = assign74420_e113939_d_n11;
        locals.var_t9_dn14 = assign74420_e113939_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74430_e113953, assign74430_e113953_d_n0, assign74430_e113953_d_n2, assign74430_e113953_d_n4, assign74430_e113953_d_n5, assign74430_e113953_d_n6, assign74430_e113953_d_n7, assign74430_e113953_d_n8, assign74430_e113953_d_n9, assign74430_e113953_d_n10, assign74430_e113953_d_n11, assign74430_e113953_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74430_e113948: f64 = (locals.var_vx + p.p137);
        let assign74430_e113950: f64 = (assign74430_e113948 + locals.var_tmf2);
        let assign74430_e113951: f64 = (0.5 * assign74430_e113950);
        (assign74430_e113951, (0.5 * (locals.var_vx_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74430_e113953;
        locals.var_t2_dn0 = assign74430_e113953_d_n0;
        locals.var_t2_dn2 = assign74430_e113953_d_n2;
        locals.var_t2_dn4 = assign74430_e113953_d_n4;
        locals.var_t2_dn5 = assign74430_e113953_d_n5;
        locals.var_t2_dn6 = assign74430_e113953_d_n6;
        locals.var_t2_dn7 = assign74430_e113953_d_n7;
        locals.var_t2_dn8 = assign74430_e113953_d_n8;
        locals.var_t2_dn9 = assign74430_e113953_d_n9;
        locals.var_t2_dn10 = assign74430_e113953_d_n10;
        locals.var_t2_dn11 = assign74430_e113953_d_n11;
        locals.var_t2_dn14 = assign74430_e113953_d_n14;
        locals.var_t2_rv = 0.0;

        let assign74440_e113956: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1744 = assign74440_e113956;
        locals.var_guard1744_rv = 0.0;

        let (assign74450_e113966, assign74450_e113966_d_n0, assign74450_e113966_d_n2, assign74450_e113966_d_n4, assign74450_e113966_d_n5, assign74450_e113966_d_n6, assign74450_e113966_d_n7, assign74450_e113966_d_n8, assign74450_e113966_d_n9, assign74450_e113966_d_n10, assign74450_e113966_d_n11, assign74450_e113966_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) && (locals.var_guard1744 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74450_e113966;
        locals.var_t2_dn0 = assign74450_e113966_d_n0;
        locals.var_t2_dn2 = assign74450_e113966_d_n2;
        locals.var_t2_dn4 = assign74450_e113966_d_n4;
        locals.var_t2_dn5 = assign74450_e113966_d_n5;
        locals.var_t2_dn6 = assign74450_e113966_d_n6;
        locals.var_t2_dn7 = assign74450_e113966_d_n7;
        locals.var_t2_dn8 = assign74450_e113966_d_n8;
        locals.var_t2_dn9 = assign74450_e113966_d_n9;
        locals.var_t2_dn10 = assign74450_e113966_d_n10;
        locals.var_t2_dn11 = assign74450_e113966_d_n11;
        locals.var_t2_dn14 = assign74450_e113966_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign74460_e113976, assign74460_e113976_d_n0, assign74460_e113976_d_n2, assign74460_e113976_d_n4, assign74460_e113976_d_n5, assign74460_e113976_d_n6, assign74460_e113976_d_n7, assign74460_e113976_d_n8, assign74460_e113976_d_n9, assign74460_e113976_d_n10, assign74460_e113976_d_n11, assign74460_e113976_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) && (locals.var_guard1744 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74460_e113976;
        locals.var_t9_dn0 = assign74460_e113976_d_n0;
        locals.var_t9_dn2 = assign74460_e113976_d_n2;
        locals.var_t9_dn4 = assign74460_e113976_d_n4;
        locals.var_t9_dn5 = assign74460_e113976_d_n5;
        locals.var_t9_dn6 = assign74460_e113976_d_n6;
        locals.var_t9_dn7 = assign74460_e113976_d_n7;
        locals.var_t9_dn8 = assign74460_e113976_d_n8;
        locals.var_t9_dn9 = assign74460_e113976_d_n9;
        locals.var_t9_dn10 = assign74460_e113976_d_n10;
        locals.var_t9_dn11 = assign74460_e113976_d_n11;
        locals.var_t9_dn14 = assign74460_e113976_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74470_e113989, assign74470_e113989_d_n0, assign74470_e113989_d_n2, assign74470_e113989_d_n4, assign74470_e113989_d_n5, assign74470_e113989_d_n6, assign74470_e113989_d_n7, assign74470_e113989_d_n8, assign74470_e113989_d_n9, assign74470_e113989_d_n10, assign74470_e113989_d_n11, assign74470_e113989_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74470_e113984: f64 = (locals.var_kjunc * locals.var_t2);
        let assign74470_e113985: f64 = (assign74470_e113984).sqrt();
        let assign74470_e113987: f64 = (assign74470_e113985 * p.p432);
        (assign74470_e113987, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign74470_e113985)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign74470_e113985)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign74470_e113989;
        locals.var_wjunc0_dn0 = assign74470_e113989_d_n0;
        locals.var_wjunc0_dn2 = assign74470_e113989_d_n2;
        locals.var_wjunc0_dn4 = assign74470_e113989_d_n4;
        locals.var_wjunc0_dn5 = assign74470_e113989_d_n5;
        locals.var_wjunc0_dn6 = assign74470_e113989_d_n6;
        locals.var_wjunc0_dn7 = assign74470_e113989_d_n7;
        locals.var_wjunc0_dn8 = assign74470_e113989_d_n8;
        locals.var_wjunc0_dn9 = assign74470_e113989_d_n9;
        locals.var_wjunc0_dn10 = assign74470_e113989_d_n10;
        locals.var_wjunc0_dn11 = assign74470_e113989_d_n11;
        locals.var_wjunc0_dn14 = assign74470_e113989_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign74480_e114003, assign74480_e114003_d_n0, assign74480_e114003_d_n2, assign74480_e114003_d_n4, assign74480_e114003_d_n5, assign74480_e114003_d_n6, assign74480_e114003_d_n7, assign74480_e114003_d_n8, assign74480_e114003_d_n9, assign74480_e114003_d_n10, assign74480_e114003_d_n11, assign74480_e114003_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74480_e113997: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign74480_e114000: f64 = (0.1 * locals.var_lover_func);
        let assign74480_e114001: f64 = (assign74480_e113997 - assign74480_e114000);
        (assign74480_e114001, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign74480_e114003;
        locals.var_tmf1_dn0 = assign74480_e114003_d_n0;
        locals.var_tmf1_dn2 = assign74480_e114003_d_n2;
        locals.var_tmf1_dn4 = assign74480_e114003_d_n4;
        locals.var_tmf1_dn5 = assign74480_e114003_d_n5;
        locals.var_tmf1_dn6 = assign74480_e114003_d_n6;
        locals.var_tmf1_dn7 = assign74480_e114003_d_n7;
        locals.var_tmf1_dn8 = assign74480_e114003_d_n8;
        locals.var_tmf1_dn9 = assign74480_e114003_d_n9;
        locals.var_tmf1_dn10 = assign74480_e114003_d_n10;
        locals.var_tmf1_dn11 = assign74480_e114003_d_n11;
        locals.var_tmf1_dn14 = assign74480_e114003_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign74490_e114017, assign74490_e114017_d_n0, assign74490_e114017_d_n2, assign74490_e114017_d_n4, assign74490_e114017_d_n5, assign74490_e114017_d_n6, assign74490_e114017_d_n7, assign74490_e114017_d_n8, assign74490_e114017_d_n9, assign74490_e114017_d_n10, assign74490_e114017_d_n11, assign74490_e114017_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74490_e114011: f64 = (4.0 * locals.var_lover_func);
        let assign74490_e114014: f64 = (0.1 * locals.var_lover_func);
        let assign74490_e114015: f64 = (assign74490_e114011 * assign74490_e114014);
        (assign74490_e114015, (((4.0 * locals.var_lover_func_dn0) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign74490_e114014) + (assign74490_e114011 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74490_e114017;
        locals.var_tmf2_dn0 = assign74490_e114017_d_n0;
        locals.var_tmf2_dn2 = assign74490_e114017_d_n2;
        locals.var_tmf2_dn4 = assign74490_e114017_d_n4;
        locals.var_tmf2_dn5 = assign74490_e114017_d_n5;
        locals.var_tmf2_dn6 = assign74490_e114017_d_n6;
        locals.var_tmf2_dn7 = assign74490_e114017_d_n7;
        locals.var_tmf2_dn8 = assign74490_e114017_d_n8;
        locals.var_tmf2_dn9 = assign74490_e114017_d_n9;
        locals.var_tmf2_dn10 = assign74490_e114017_d_n10;
        locals.var_tmf2_dn11 = assign74490_e114017_d_n11;
        locals.var_tmf2_dn14 = assign74490_e114017_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_282(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74500_e114031, assign74500_e114031_d_n0, assign74500_e114031_d_n2, assign74500_e114031_d_n4, assign74500_e114031_d_n5, assign74500_e114031_d_n6, assign74500_e114031_d_n7, assign74500_e114031_d_n8, assign74500_e114031_d_n9, assign74500_e114031_d_n10, assign74500_e114031_d_n11, assign74500_e114031_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let (assign74500_e114029, assign74500_e114029_d_n0, assign74500_e114029_d_n2, assign74500_e114029_d_n4, assign74500_e114029_d_n5, assign74500_e114029_d_n6, assign74500_e114029_d_n7, assign74500_e114029_d_n8, assign74500_e114029_d_n9, assign74500_e114029_d_n10, assign74500_e114029_d_n11, assign74500_e114029_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign74500_e114028: f64 = (-locals.var_tmf2);
                (assign74500_e114028, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign74500_e114029, assign74500_e114029_d_n0, assign74500_e114029_d_n2, assign74500_e114029_d_n4, assign74500_e114029_d_n5, assign74500_e114029_d_n6, assign74500_e114029_d_n7, assign74500_e114029_d_n8, assign74500_e114029_d_n9, assign74500_e114029_d_n10, assign74500_e114029_d_n11, assign74500_e114029_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74500_e114031;
        locals.var_tmf2_dn0 = assign74500_e114031_d_n0;
        locals.var_tmf2_dn2 = assign74500_e114031_d_n2;
        locals.var_tmf2_dn4 = assign74500_e114031_d_n4;
        locals.var_tmf2_dn5 = assign74500_e114031_d_n5;
        locals.var_tmf2_dn6 = assign74500_e114031_d_n6;
        locals.var_tmf2_dn7 = assign74500_e114031_d_n7;
        locals.var_tmf2_dn8 = assign74500_e114031_d_n8;
        locals.var_tmf2_dn9 = assign74500_e114031_d_n9;
        locals.var_tmf2_dn10 = assign74500_e114031_d_n10;
        locals.var_tmf2_dn11 = assign74500_e114031_d_n11;
        locals.var_tmf2_dn14 = assign74500_e114031_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign74510_e114044, assign74510_e114044_d_n0, assign74510_e114044_d_n2, assign74510_e114044_d_n4, assign74510_e114044_d_n5, assign74510_e114044_d_n6, assign74510_e114044_d_n7, assign74510_e114044_d_n8, assign74510_e114044_d_n9, assign74510_e114044_d_n10, assign74510_e114044_d_n11, assign74510_e114044_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74510_e114039: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign74510_e114041: f64 = (assign74510_e114039 + locals.var_tmf2);
        let assign74510_e114042: f64 = (assign74510_e114041).sqrt();
        (assign74510_e114042, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign74510_e114042)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign74510_e114042)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74510_e114044;
        locals.var_tmf2_dn0 = assign74510_e114044_d_n0;
        locals.var_tmf2_dn2 = assign74510_e114044_d_n2;
        locals.var_tmf2_dn4 = assign74510_e114044_d_n4;
        locals.var_tmf2_dn5 = assign74510_e114044_d_n5;
        locals.var_tmf2_dn6 = assign74510_e114044_d_n6;
        locals.var_tmf2_dn7 = assign74510_e114044_d_n7;
        locals.var_tmf2_dn8 = assign74510_e114044_d_n8;
        locals.var_tmf2_dn9 = assign74510_e114044_d_n9;
        locals.var_tmf2_dn10 = assign74510_e114044_d_n10;
        locals.var_tmf2_dn11 = assign74510_e114044_d_n11;
        locals.var_tmf2_dn14 = assign74510_e114044_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign74520_e114058, assign74520_e114058_d_n0, assign74520_e114058_d_n2, assign74520_e114058_d_n4, assign74520_e114058_d_n5, assign74520_e114058_d_n6, assign74520_e114058_d_n7, assign74520_e114058_d_n8, assign74520_e114058_d_n9, assign74520_e114058_d_n10, assign74520_e114058_d_n11, assign74520_e114058_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74520_e114054: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign74520_e114055: f64 = (1.0 + assign74520_e114054);
        let assign74520_e114056: f64 = (0.5 * assign74520_e114055);
        (assign74520_e114056, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign74520_e114058;
        locals.var_t0_dn0 = assign74520_e114058_d_n0;
        locals.var_t0_dn2 = assign74520_e114058_d_n2;
        locals.var_t0_dn4 = assign74520_e114058_d_n4;
        locals.var_t0_dn5 = assign74520_e114058_d_n5;
        locals.var_t0_dn6 = assign74520_e114058_d_n6;
        locals.var_t0_dn7 = assign74520_e114058_d_n7;
        locals.var_t0_dn8 = assign74520_e114058_d_n8;
        locals.var_t0_dn9 = assign74520_e114058_d_n9;
        locals.var_t0_dn10 = assign74520_e114058_d_n10;
        locals.var_t0_dn11 = assign74520_e114058_d_n11;
        locals.var_t0_dn14 = assign74520_e114058_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign74530_e114072, assign74530_e114072_d_n0, assign74530_e114072_d_n2, assign74530_e114072_d_n4, assign74530_e114072_d_n5, assign74530_e114072_d_n6, assign74530_e114072_d_n7, assign74530_e114072_d_n8, assign74530_e114072_d_n9, assign74530_e114072_d_n10, assign74530_e114072_d_n11, assign74530_e114072_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74530_e114068: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign74530_e114069: f64 = (0.5 * assign74530_e114068);
        let assign74530_e114070: f64 = (locals.var_lover_func - assign74530_e114069);
        (assign74530_e114070, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign74530_e114072;
        locals.var_wjuncld_dn0 = assign74530_e114072_d_n0;
        locals.var_wjuncld_dn2 = assign74530_e114072_d_n2;
        locals.var_wjuncld_dn4 = assign74530_e114072_d_n4;
        locals.var_wjuncld_dn5 = assign74530_e114072_d_n5;
        locals.var_wjuncld_dn6 = assign74530_e114072_d_n6;
        locals.var_wjuncld_dn7 = assign74530_e114072_d_n7;
        locals.var_wjuncld_dn8 = assign74530_e114072_d_n8;
        locals.var_wjuncld_dn9 = assign74530_e114072_d_n9;
        locals.var_wjuncld_dn10 = assign74530_e114072_d_n10;
        locals.var_wjuncld_dn11 = assign74530_e114072_d_n11;
        locals.var_wjuncld_dn14 = assign74530_e114072_d_n14;
        locals.var_wjuncld_rv = 0.0;

        let (assign74540_e114082, assign74540_e114082_d_n0, assign74540_e114082_d_n2, assign74540_e114082_d_n4, assign74540_e114082_d_n5, assign74540_e114082_d_n6, assign74540_e114082_d_n7, assign74540_e114082_d_n8, assign74540_e114082_d_n9, assign74540_e114082_d_n10, assign74540_e114082_d_n11, assign74540_e114082_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1741 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign74540_e114080: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign74540_e114080, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign74540_e114082;
        locals.var_lover_func_dn0 = assign74540_e114082_d_n0;
        locals.var_lover_func_dn2 = assign74540_e114082_d_n2;
        locals.var_lover_func_dn4 = assign74540_e114082_d_n4;
        locals.var_lover_func_dn5 = assign74540_e114082_d_n5;
        locals.var_lover_func_dn6 = assign74540_e114082_d_n6;
        locals.var_lover_func_dn7 = assign74540_e114082_d_n7;
        locals.var_lover_func_dn8 = assign74540_e114082_d_n8;
        locals.var_lover_func_dn9 = assign74540_e114082_d_n9;
        locals.var_lover_func_dn10 = assign74540_e114082_d_n10;
        locals.var_lover_func_dn11 = assign74540_e114082_d_n11;
        locals.var_lover_func_dn14 = assign74540_e114082_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign74550_e114085: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1745 = assign74550_e114085;
        locals.var_guard1745_rv = 0.0;

        let assign74560_e114088: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1746 = assign74560_e114088;
        locals.var_guard1746_rv = 0.0;

        let assign74570_e114091: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1747 = assign74570_e114091;
        locals.var_guard1747_rv = 0.0;

        let assign74580_e114094: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1748 = assign74580_e114094;
        locals.var_guard1748_rv = 0.0;

        let assign74590_e114097: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign74590_e114097;
        locals.var_guard1749_rv = 0.0;

        let (assign74600_e114107, assign74600_e114107_d_n0, assign74600_e114107_d_n2, assign74600_e114107_d_n4, assign74600_e114107_d_n5, assign74600_e114107_d_n6, assign74600_e114107_d_n7, assign74600_e114107_d_n8, assign74600_e114107_d_n9, assign74600_e114107_d_n10, assign74600_e114107_d_n11, assign74600_e114107_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1745 != 0.0)) && (locals.var_guard1749 != 0.0)) {
        let assign74600_e114105: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign74600_e114105, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74600_e114107;
        locals.var_t4_dn0 = assign74600_e114107_d_n0;
        locals.var_t4_dn2 = assign74600_e114107_d_n2;
        locals.var_t4_dn4 = assign74600_e114107_d_n4;
        locals.var_t4_dn5 = assign74600_e114107_d_n5;
        locals.var_t4_dn6 = assign74600_e114107_d_n6;
        locals.var_t4_dn7 = assign74600_e114107_d_n7;
        locals.var_t4_dn8 = assign74600_e114107_d_n8;
        locals.var_t4_dn9 = assign74600_e114107_d_n9;
        locals.var_t4_dn10 = assign74600_e114107_d_n10;
        locals.var_t4_dn11 = assign74600_e114107_d_n11;
        locals.var_t4_dn14 = assign74600_e114107_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74610_e114122, assign74610_e114122_d_n0, assign74610_e114122_d_n2, assign74610_e114122_d_n4, assign74610_e114122_d_n5, assign74610_e114122_d_n6, assign74610_e114122_d_n7, assign74610_e114122_d_n8, assign74610_e114122_d_n9, assign74610_e114122_d_n10, assign74610_e114122_d_n11, assign74610_e114122_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1745 != 0.0)) && (locals.var_guard1749 == 0.0)) {
        let assign74610_e114116: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74610_e114119: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign74610_e114120: f64 = (assign74610_e114116 * assign74610_e114119);
        (assign74610_e114120, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign74610_e114119), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign74610_e114119),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74610_e114122;
        locals.var_t4_dn0 = assign74610_e114122_d_n0;
        locals.var_t4_dn2 = assign74610_e114122_d_n2;
        locals.var_t4_dn4 = assign74610_e114122_d_n4;
        locals.var_t4_dn5 = assign74610_e114122_d_n5;
        locals.var_t4_dn6 = assign74610_e114122_d_n6;
        locals.var_t4_dn7 = assign74610_e114122_d_n7;
        locals.var_t4_dn8 = assign74610_e114122_d_n8;
        locals.var_t4_dn9 = assign74610_e114122_d_n9;
        locals.var_t4_dn10 = assign74610_e114122_d_n10;
        locals.var_t4_dn11 = assign74610_e114122_d_n11;
        locals.var_t4_dn14 = assign74610_e114122_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74620_e114130, assign74620_e114130_d_n0, assign74620_e114130_d_n2, assign74620_e114130_d_n4, assign74620_e114130_d_n5, assign74620_e114130_d_n6, assign74620_e114130_d_n7, assign74620_e114130_d_n8, assign74620_e114130_d_n9, assign74620_e114130_d_n10, assign74620_e114130_d_n11, assign74620_e114130_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1745 != 0.0)) {
        let assign74620_e114128: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74620_e114128, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign74620_e114130;
        locals.var_qovs_dn0 = assign74620_e114130_d_n0;
        locals.var_qovs_dn2 = assign74620_e114130_d_n2;
        locals.var_qovs_dn4 = assign74620_e114130_d_n4;
        locals.var_qovs_dn5 = assign74620_e114130_d_n5;
        locals.var_qovs_dn6 = assign74620_e114130_d_n6;
        locals.var_qovs_dn7 = assign74620_e114130_d_n7;
        locals.var_qovs_dn8 = assign74620_e114130_d_n8;
        locals.var_qovs_dn9 = assign74620_e114130_d_n9;
        locals.var_qovs_dn10 = assign74620_e114130_d_n10;
        locals.var_qovs_dn11 = assign74620_e114130_d_n11;
        locals.var_qovs_dn14 = assign74620_e114130_d_n14;
        locals.var_qovs_rv = 0.0;

        let (assign74630_e114138, assign74630_e114138_d_n0, assign74630_e114138_d_n2, assign74630_e114138_d_n4, assign74630_e114138_d_n5, assign74630_e114138_d_n6, assign74630_e114138_d_n7, assign74630_e114138_d_n8, assign74630_e114138_d_n9, assign74630_e114138_d_n10, assign74630_e114138_d_n11, assign74630_e114138_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1745 != 0.0)) {
        let assign74630_e114136: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74630_e114136, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn14,)
    }
};
        locals.var_qbsld = assign74630_e114138;
        locals.var_qbsld_dn0 = assign74630_e114138_d_n0;
        locals.var_qbsld_dn2 = assign74630_e114138_d_n2;
        locals.var_qbsld_dn4 = assign74630_e114138_d_n4;
        locals.var_qbsld_dn5 = assign74630_e114138_d_n5;
        locals.var_qbsld_dn6 = assign74630_e114138_d_n6;
        locals.var_qbsld_dn7 = assign74630_e114138_d_n7;
        locals.var_qbsld_dn8 = assign74630_e114138_d_n8;
        locals.var_qbsld_dn9 = assign74630_e114138_d_n9;
        locals.var_qbsld_dn10 = assign74630_e114138_d_n10;
        locals.var_qbsld_dn11 = assign74630_e114138_d_n11;
        locals.var_qbsld_dn14 = assign74630_e114138_d_n14;
        locals.var_qbsld_rv = 0.0;

        let (assign74660_e114163, assign74660_e114163_d_n0, assign74660_e114163_d_n2, assign74660_e114163_d_n4, assign74660_e114163_d_n5, assign74660_e114163_d_n6, assign74660_e114163_d_n7, assign74660_e114163_d_n8, assign74660_e114163_d_n9, assign74660_e114163_d_n10, assign74660_e114163_d_n11, assign74660_e114163_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1746 != 0.0) && (locals.var_guard1745 == 0.0))) {
        let assign74660_e114159: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74660_e114161: f64 = (assign74660_e114159 * locals.var_uc_cvdsover);
        (assign74660_e114161, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74660_e114163;
        locals.var_t4_dn0 = assign74660_e114163_d_n0;
        locals.var_t4_dn2 = assign74660_e114163_d_n2;
        locals.var_t4_dn4 = assign74660_e114163_d_n4;
        locals.var_t4_dn5 = assign74660_e114163_d_n5;
        locals.var_t4_dn6 = assign74660_e114163_d_n6;
        locals.var_t4_dn7 = assign74660_e114163_d_n7;
        locals.var_t4_dn8 = assign74660_e114163_d_n8;
        locals.var_t4_dn9 = assign74660_e114163_d_n9;
        locals.var_t4_dn10 = assign74660_e114163_d_n10;
        locals.var_t4_dn11 = assign74660_e114163_d_n11;
        locals.var_t4_dn14 = assign74660_e114163_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74670_e114174, assign74670_e114174_d_n0, assign74670_e114174_d_n2, assign74670_e114174_d_n4, assign74670_e114174_d_n5, assign74670_e114174_d_n6, assign74670_e114174_d_n7, assign74670_e114174_d_n8, assign74670_e114174_d_n9, assign74670_e114174_d_n10, assign74670_e114174_d_n11, assign74670_e114174_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1746 != 0.0) && (locals.var_guard1745 == 0.0))) {
        let assign74670_e114172: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74670_e114172, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn11, locals.var_qovsext_dn14,)
    }
};
        locals.var_qovsext = assign74670_e114174;
        locals.var_qovsext_dn0 = assign74670_e114174_d_n0;
        locals.var_qovsext_dn2 = assign74670_e114174_d_n2;
        locals.var_qovsext_dn4 = assign74670_e114174_d_n4;
        locals.var_qovsext_dn5 = assign74670_e114174_d_n5;
        locals.var_qovsext_dn6 = assign74670_e114174_d_n6;
        locals.var_qovsext_dn7 = assign74670_e114174_d_n7;
        locals.var_qovsext_dn8 = assign74670_e114174_d_n8;
        locals.var_qovsext_dn9 = assign74670_e114174_d_n9;
        locals.var_qovsext_dn10 = assign74670_e114174_d_n10;
        locals.var_qovsext_dn11 = assign74670_e114174_d_n11;
        locals.var_qovsext_dn14 = assign74670_e114174_d_n14;
        locals.var_qovsext_rv = 0.0;

        let (assign74680_e114185, assign74680_e114185_d_n0, assign74680_e114185_d_n2, assign74680_e114185_d_n4, assign74680_e114185_d_n5, assign74680_e114185_d_n6, assign74680_e114185_d_n7, assign74680_e114185_d_n8, assign74680_e114185_d_n9, assign74680_e114185_d_n10, assign74680_e114185_d_n11, assign74680_e114185_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1746 != 0.0) && (locals.var_guard1745 == 0.0))) {
        let assign74680_e114183: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74680_e114183, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn11, locals.var_qbsldext_dn14,)
    }
};
        locals.var_qbsldext = assign74680_e114185;
        locals.var_qbsldext_dn0 = assign74680_e114185_d_n0;
        locals.var_qbsldext_dn2 = assign74680_e114185_d_n2;
        locals.var_qbsldext_dn4 = assign74680_e114185_d_n4;
        locals.var_qbsldext_dn5 = assign74680_e114185_d_n5;
        locals.var_qbsldext_dn6 = assign74680_e114185_d_n6;
        locals.var_qbsldext_dn7 = assign74680_e114185_d_n7;
        locals.var_qbsldext_dn8 = assign74680_e114185_d_n8;
        locals.var_qbsldext_dn9 = assign74680_e114185_d_n9;
        locals.var_qbsldext_dn10 = assign74680_e114185_d_n10;
        locals.var_qbsldext_dn11 = assign74680_e114185_d_n11;
        locals.var_qbsldext_dn14 = assign74680_e114185_d_n14;
        locals.var_qbsldext_rv = 0.0;

        let assign74690_e114188: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1750 = assign74690_e114188;
        locals.var_guard1750_rv = 0.0;

        let (assign74700_e114203, assign74700_e114203_d_n0, assign74700_e114203_d_n2, assign74700_e114203_d_n4, assign74700_e114203_d_n5, assign74700_e114203_d_n6, assign74700_e114203_d_n7, assign74700_e114203_d_n8, assign74700_e114203_d_n9, assign74700_e114203_d_n10, assign74700_e114203_d_n11, assign74700_e114203_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) && (locals.var_guard1750 != 0.0)) {
        let assign74700_e114201: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign74700_e114201, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74700_e114203;
        locals.var_t4_dn0 = assign74700_e114203_d_n0;
        locals.var_t4_dn2 = assign74700_e114203_d_n2;
        locals.var_t4_dn4 = assign74700_e114203_d_n4;
        locals.var_t4_dn5 = assign74700_e114203_d_n5;
        locals.var_t4_dn6 = assign74700_e114203_d_n6;
        locals.var_t4_dn7 = assign74700_e114203_d_n7;
        locals.var_t4_dn8 = assign74700_e114203_d_n8;
        locals.var_t4_dn9 = assign74700_e114203_d_n9;
        locals.var_t4_dn10 = assign74700_e114203_d_n10;
        locals.var_t4_dn11 = assign74700_e114203_d_n11;
        locals.var_t4_dn14 = assign74700_e114203_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74710_e114223, assign74710_e114223_d_n0, assign74710_e114223_d_n2, assign74710_e114223_d_n4, assign74710_e114223_d_n5, assign74710_e114223_d_n6, assign74710_e114223_d_n7, assign74710_e114223_d_n8, assign74710_e114223_d_n9, assign74710_e114223_d_n10, assign74710_e114223_d_n11, assign74710_e114223_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) && (locals.var_guard1750 == 0.0)) {
        let assign74710_e114217: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74710_e114220: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign74710_e114221: f64 = (assign74710_e114217 * assign74710_e114220);
        (assign74710_e114221, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign74710_e114220), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign74710_e114220),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74710_e114223;
        locals.var_t4_dn0 = assign74710_e114223_d_n0;
        locals.var_t4_dn2 = assign74710_e114223_d_n2;
        locals.var_t4_dn4 = assign74710_e114223_d_n4;
        locals.var_t4_dn5 = assign74710_e114223_d_n5;
        locals.var_t4_dn6 = assign74710_e114223_d_n6;
        locals.var_t4_dn7 = assign74710_e114223_d_n7;
        locals.var_t4_dn8 = assign74710_e114223_d_n8;
        locals.var_t4_dn9 = assign74710_e114223_d_n9;
        locals.var_t4_dn10 = assign74710_e114223_d_n10;
        locals.var_t4_dn11 = assign74710_e114223_d_n11;
        locals.var_t4_dn14 = assign74710_e114223_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74720_e114234, assign74720_e114234_d_n0, assign74720_e114234_d_n2, assign74720_e114234_d_n4, assign74720_e114234_d_n5, assign74720_e114234_d_n6, assign74720_e114234_d_n7, assign74720_e114234_d_n8, assign74720_e114234_d_n9, assign74720_e114234_d_n10, assign74720_e114234_d_n11, assign74720_e114234_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn11, locals.var_rd_ps0ld_dn14,)
    }
};
        locals.var_rd_ps0ld = assign74720_e114234;
        locals.var_rd_ps0ld_dn0 = assign74720_e114234_d_n0;
        locals.var_rd_ps0ld_dn2 = assign74720_e114234_d_n2;
        locals.var_rd_ps0ld_dn4 = assign74720_e114234_d_n4;
        locals.var_rd_ps0ld_dn5 = assign74720_e114234_d_n5;
        locals.var_rd_ps0ld_dn6 = assign74720_e114234_d_n6;
        locals.var_rd_ps0ld_dn7 = assign74720_e114234_d_n7;
        locals.var_rd_ps0ld_dn8 = assign74720_e114234_d_n8;
        locals.var_rd_ps0ld_dn9 = assign74720_e114234_d_n9;
        locals.var_rd_ps0ld_dn10 = assign74720_e114234_d_n10;
        locals.var_rd_ps0ld_dn11 = assign74720_e114234_d_n11;
        locals.var_rd_ps0ld_dn14 = assign74720_e114234_d_n14;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign74730_e114237: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1751 = assign74730_e114237;
        locals.var_guard1751_rv = 0.0;

        let (assign74740_e114250, assign74740_e114250_d_n0, assign74740_e114250_d_n2, assign74740_e114250_d_n4, assign74740_e114250_d_n5, assign74740_e114250_d_n6, assign74740_e114250_d_n7, assign74740_e114250_d_n8, assign74740_e114250_d_n9, assign74740_e114250_d_n10, assign74740_e114250_d_n11, assign74740_e114250_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) && (locals.var_guard1751 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign74740_e114250;
        locals.var_rd_qbuld_dn0 = assign74740_e114250_d_n0;
        locals.var_rd_qbuld_dn2 = assign74740_e114250_d_n2;
        locals.var_rd_qbuld_dn4 = assign74740_e114250_d_n4;
        locals.var_rd_qbuld_dn5 = assign74740_e114250_d_n5;
        locals.var_rd_qbuld_dn6 = assign74740_e114250_d_n6;
        locals.var_rd_qbuld_dn7 = assign74740_e114250_d_n7;
        locals.var_rd_qbuld_dn8 = assign74740_e114250_d_n8;
        locals.var_rd_qbuld_dn9 = assign74740_e114250_d_n9;
        locals.var_rd_qbuld_dn10 = assign74740_e114250_d_n10;
        locals.var_rd_qbuld_dn11 = assign74740_e114250_d_n11;
        locals.var_rd_qbuld_dn14 = assign74740_e114250_d_n14;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign74750_e114263, assign74750_e114263_d_n0, assign74750_e114263_d_n2, assign74750_e114263_d_n4, assign74750_e114263_d_n5, assign74750_e114263_d_n6, assign74750_e114263_d_n7, assign74750_e114263_d_n8, assign74750_e114263_d_n9, assign74750_e114263_d_n10, assign74750_e114263_d_n11, assign74750_e114263_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) {
        let assign74750_e114261: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74750_e114261, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign74750_e114263;
        locals.var_qovd_dn0 = assign74750_e114263_d_n0;
        locals.var_qovd_dn2 = assign74750_e114263_d_n2;
        locals.var_qovd_dn4 = assign74750_e114263_d_n4;
        locals.var_qovd_dn5 = assign74750_e114263_d_n5;
        locals.var_qovd_dn6 = assign74750_e114263_d_n6;
        locals.var_qovd_dn7 = assign74750_e114263_d_n7;
        locals.var_qovd_dn8 = assign74750_e114263_d_n8;
        locals.var_qovd_dn9 = assign74750_e114263_d_n9;
        locals.var_qovd_dn10 = assign74750_e114263_d_n10;
        locals.var_qovd_dn11 = assign74750_e114263_d_n11;
        locals.var_qovd_dn14 = assign74750_e114263_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign74760_e114276, assign74760_e114276_d_n0, assign74760_e114276_d_n2, assign74760_e114276_d_n4, assign74760_e114276_d_n5, assign74760_e114276_d_n6, assign74760_e114276_d_n7, assign74760_e114276_d_n8, assign74760_e114276_d_n9, assign74760_e114276_d_n10, assign74760_e114276_d_n11, assign74760_e114276_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) {
        let assign74760_e114274: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74760_e114274, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign74760_e114276;
        locals.var_qbdld_dn0 = assign74760_e114276_d_n0;
        locals.var_qbdld_dn2 = assign74760_e114276_d_n2;
        locals.var_qbdld_dn4 = assign74760_e114276_d_n4;
        locals.var_qbdld_dn5 = assign74760_e114276_d_n5;
        locals.var_qbdld_dn6 = assign74760_e114276_d_n6;
        locals.var_qbdld_dn7 = assign74760_e114276_d_n7;
        locals.var_qbdld_dn8 = assign74760_e114276_d_n8;
        locals.var_qbdld_dn9 = assign74760_e114276_d_n9;
        locals.var_qbdld_dn10 = assign74760_e114276_d_n10;
        locals.var_qbdld_dn11 = assign74760_e114276_d_n11;
        locals.var_qbdld_dn14 = assign74760_e114276_d_n14;
        locals.var_qbdld_rv = 0.0;

        let (assign74770_e114287, assign74770_e114287_d_n0, assign74770_e114287_d_n2, assign74770_e114287_d_n4, assign74770_e114287_d_n5, assign74770_e114287_d_n6, assign74770_e114287_d_n7, assign74770_e114287_d_n8, assign74770_e114287_d_n9, assign74770_e114287_d_n10, assign74770_e114287_d_n11, assign74770_e114287_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1747 != 0.0) && (!((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    }
};
        locals.var_qbd_qs = assign74770_e114287;
        locals.var_qbd_qs_dn0 = assign74770_e114287_d_n0;
        locals.var_qbd_qs_dn2 = assign74770_e114287_d_n2;
        locals.var_qbd_qs_dn4 = assign74770_e114287_d_n4;
        locals.var_qbd_qs_dn5 = assign74770_e114287_d_n5;
        locals.var_qbd_qs_dn6 = assign74770_e114287_d_n6;
        locals.var_qbd_qs_dn7 = assign74770_e114287_d_n7;
        locals.var_qbd_qs_dn8 = assign74770_e114287_d_n8;
        locals.var_qbd_qs_dn9 = assign74770_e114287_d_n9;
        locals.var_qbd_qs_dn10 = assign74770_e114287_d_n10;
        locals.var_qbd_qs_dn11 = assign74770_e114287_d_n11;
        locals.var_qbd_qs_dn14 = assign74770_e114287_d_n14;
        locals.var_qbd_qs_rv = 0.0;

        let (assign74780_e114304, assign74780_e114304_d_n0, assign74780_e114304_d_n2, assign74780_e114304_d_n4, assign74780_e114304_d_n5, assign74780_e114304_d_n6, assign74780_e114304_d_n7, assign74780_e114304_d_n8, assign74780_e114304_d_n9, assign74780_e114304_d_n10, assign74780_e114304_d_n11, assign74780_e114304_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1748 != 0.0) && (!(((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0)) || (locals.var_guard1747 != 0.0))))) {
        let assign74780_e114300: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign74780_e114302: f64 = (assign74780_e114300 * locals.var_uc_cvdsover);
        (assign74780_e114302, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign74780_e114304;
        locals.var_t4_dn0 = assign74780_e114304_d_n0;
        locals.var_t4_dn2 = assign74780_e114304_d_n2;
        locals.var_t4_dn4 = assign74780_e114304_d_n4;
        locals.var_t4_dn5 = assign74780_e114304_d_n5;
        locals.var_t4_dn6 = assign74780_e114304_d_n6;
        locals.var_t4_dn7 = assign74780_e114304_d_n7;
        locals.var_t4_dn8 = assign74780_e114304_d_n8;
        locals.var_t4_dn9 = assign74780_e114304_d_n9;
        locals.var_t4_dn10 = assign74780_e114304_d_n10;
        locals.var_t4_dn11 = assign74780_e114304_d_n11;
        locals.var_t4_dn14 = assign74780_e114304_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign74790_e114319, assign74790_e114319_d_n0, assign74790_e114319_d_n2, assign74790_e114319_d_n4, assign74790_e114319_d_n5, assign74790_e114319_d_n6, assign74790_e114319_d_n7, assign74790_e114319_d_n8, assign74790_e114319_d_n9, assign74790_e114319_d_n10, assign74790_e114319_d_n11, assign74790_e114319_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1748 != 0.0) && (!(((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0)) || (locals.var_guard1747 != 0.0))))) {
        let assign74790_e114317: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign74790_e114317, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn11, locals.var_qovdext_dn14,)
    }
};
        locals.var_qovdext = assign74790_e114319;
        locals.var_qovdext_dn0 = assign74790_e114319_d_n0;
        locals.var_qovdext_dn2 = assign74790_e114319_d_n2;
        locals.var_qovdext_dn4 = assign74790_e114319_d_n4;
        locals.var_qovdext_dn5 = assign74790_e114319_d_n5;
        locals.var_qovdext_dn6 = assign74790_e114319_d_n6;
        locals.var_qovdext_dn7 = assign74790_e114319_d_n7;
        locals.var_qovdext_dn8 = assign74790_e114319_d_n8;
        locals.var_qovdext_dn9 = assign74790_e114319_d_n9;
        locals.var_qovdext_dn10 = assign74790_e114319_d_n10;
        locals.var_qovdext_dn11 = assign74790_e114319_d_n11;
        locals.var_qovdext_dn14 = assign74790_e114319_d_n14;
        locals.var_qovdext_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_283(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74800_e114334, assign74800_e114334_d_n0, assign74800_e114334_d_n2, assign74800_e114334_d_n4, assign74800_e114334_d_n5, assign74800_e114334_d_n6, assign74800_e114334_d_n7, assign74800_e114334_d_n8, assign74800_e114334_d_n9, assign74800_e114334_d_n10, assign74800_e114334_d_n11, assign74800_e114334_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1748 != 0.0) && (!(((locals.var_guard1745 != 0.0) || (locals.var_guard1746 != 0.0)) || (locals.var_guard1747 != 0.0))))) {
        let assign74800_e114332: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74800_e114332, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn11, locals.var_qbdldext_dn14,)
    }
};
        locals.var_qbdldext = assign74800_e114334;
        locals.var_qbdldext_dn0 = assign74800_e114334_d_n0;
        locals.var_qbdldext_dn2 = assign74800_e114334_d_n2;
        locals.var_qbdldext_dn4 = assign74800_e114334_d_n4;
        locals.var_qbdldext_dn5 = assign74800_e114334_d_n5;
        locals.var_qbdldext_dn6 = assign74800_e114334_d_n6;
        locals.var_qbdldext_dn7 = assign74800_e114334_d_n7;
        locals.var_qbdldext_dn8 = assign74800_e114334_d_n8;
        locals.var_qbdldext_dn9 = assign74800_e114334_d_n9;
        locals.var_qbdldext_dn10 = assign74800_e114334_d_n10;
        locals.var_qbdldext_dn11 = assign74800_e114334_d_n11;
        locals.var_qbdldext_dn14 = assign74800_e114334_d_n14;
        locals.var_qbdldext_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign74820_e114338: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1752 = assign74820_e114338;
        locals.var_guard1752_rv = 0.0;

        let assign74830_e114341: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1753 = assign74830_e114341;
        locals.var_guard1753_rv = 0.0;

        let assign74840_e114344: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1754 = assign74840_e114344;
        locals.var_guard1754_rv = 0.0;

        let assign74850_e114347: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1755 = assign74850_e114347;
        locals.var_guard1755_rv = 0.0;

        let assign74860_e114358: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1756 = assign74860_e114358;
        locals.var_guard1756_rv = 0.0;

        let (assign74870_e114364,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74870_e114364;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74880_e114370,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign74880_e114370;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign74890_e114378, assign74890_e114378_d_n2, assign74890_e114378_d_n7, assign74890_e114378_d_n8, assign74890_e114378_d_n9,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        let assign74890_e114376: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign74890_e114376, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign74890_e114378;
        locals.var_vgbgmt_dn2 = assign74890_e114378_d_n2;
        locals.var_vgbgmt_dn7 = assign74890_e114378_d_n7;
        locals.var_vgbgmt_dn8 = assign74890_e114378_d_n8;
        locals.var_vgbgmt_dn9 = assign74890_e114378_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74900_e114385, assign74900_e114385_d_n0, assign74900_e114385_d_n2, assign74900_e114385_d_n4, assign74900_e114385_d_n5, assign74900_e114385_d_n6, assign74900_e114385_d_n7, assign74900_e114385_d_n8, assign74900_e114385_d_n9, assign74900_e114385_d_n10, assign74900_e114385_d_n11, assign74900_e114385_d_n14,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        let assign74900_e114383: f64 = (-locals.var_vbsi);
        (assign74900_e114383, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign74900_e114385;
        locals.var_vxbgmt_dn0 = assign74900_e114385_d_n0;
        locals.var_vxbgmt_dn2 = assign74900_e114385_d_n2;
        locals.var_vxbgmt_dn4 = assign74900_e114385_d_n4;
        locals.var_vxbgmt_dn5 = assign74900_e114385_d_n5;
        locals.var_vxbgmt_dn6 = assign74900_e114385_d_n6;
        locals.var_vxbgmt_dn7 = assign74900_e114385_d_n7;
        locals.var_vxbgmt_dn8 = assign74900_e114385_d_n8;
        locals.var_vxbgmt_dn9 = assign74900_e114385_d_n9;
        locals.var_vxbgmt_dn10 = assign74900_e114385_d_n10;
        locals.var_vxbgmt_dn11 = assign74900_e114385_d_n11;
        locals.var_vxbgmt_dn14 = assign74900_e114385_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign74910_e114391,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign74910_e114391;
        locals.var_nover_func_rv = 0.0;

        let (assign74920_e114397, assign74920_e114397_d_n0, assign74920_e114397_d_n2, assign74920_e114397_d_n4, assign74920_e114397_d_n5, assign74920_e114397_d_n6, assign74920_e114397_d_n7, assign74920_e114397_d_n8, assign74920_e114397_d_n9, assign74920_e114397_d_n10, assign74920_e114397_d_n11, assign74920_e114397_d_n14,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign74920_e114397;
        locals.var_lover_func_dn0 = assign74920_e114397_d_n0;
        locals.var_lover_func_dn2 = assign74920_e114397_d_n2;
        locals.var_lover_func_dn4 = assign74920_e114397_d_n4;
        locals.var_lover_func_dn5 = assign74920_e114397_d_n5;
        locals.var_lover_func_dn6 = assign74920_e114397_d_n6;
        locals.var_lover_func_dn7 = assign74920_e114397_d_n7;
        locals.var_lover_func_dn8 = assign74920_e114397_d_n8;
        locals.var_lover_func_dn9 = assign74920_e114397_d_n9;
        locals.var_lover_func_dn10 = assign74920_e114397_d_n10;
        locals.var_lover_func_dn11 = assign74920_e114397_d_n11;
        locals.var_lover_func_dn14 = assign74920_e114397_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign74930_e114403, assign74930_e114403_d_n0, assign74930_e114403_d_n2, assign74930_e114403_d_n4, assign74930_e114403_d_n5, assign74930_e114403_d_n6, assign74930_e114403_d_n7, assign74930_e114403_d_n8, assign74930_e114403_d_n9, assign74930_e114403_d_n10, assign74930_e114403_d_n11, assign74930_e114403_d_n14,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign74930_e114403;
        locals.var_wdep_func_dn0 = assign74930_e114403_d_n0;
        locals.var_wdep_func_dn2 = assign74930_e114403_d_n2;
        locals.var_wdep_func_dn4 = assign74930_e114403_d_n4;
        locals.var_wdep_func_dn5 = assign74930_e114403_d_n5;
        locals.var_wdep_func_dn6 = assign74930_e114403_d_n6;
        locals.var_wdep_func_dn7 = assign74930_e114403_d_n7;
        locals.var_wdep_func_dn8 = assign74930_e114403_d_n8;
        locals.var_wdep_func_dn9 = assign74930_e114403_d_n9;
        locals.var_wdep_func_dn10 = assign74930_e114403_d_n10;
        locals.var_wdep_func_dn11 = assign74930_e114403_d_n11;
        locals.var_wdep_func_dn14 = assign74930_e114403_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign74940_e114409, assign74940_e114409_d_n0, assign74940_e114409_d_n2, assign74940_e114409_d_n4, assign74940_e114409_d_n5, assign74940_e114409_d_n6, assign74940_e114409_d_n7, assign74940_e114409_d_n8, assign74940_e114409_d_n9, assign74940_e114409_d_n10, assign74940_e114409_d_n11, assign74940_e114409_d_n14,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign74940_e114409;
        locals.var_cnst0over_func_dn0 = assign74940_e114409_d_n0;
        locals.var_cnst0over_func_dn2 = assign74940_e114409_d_n2;
        locals.var_cnst0over_func_dn4 = assign74940_e114409_d_n4;
        locals.var_cnst0over_func_dn5 = assign74940_e114409_d_n5;
        locals.var_cnst0over_func_dn6 = assign74940_e114409_d_n6;
        locals.var_cnst0over_func_dn7 = assign74940_e114409_d_n7;
        locals.var_cnst0over_func_dn8 = assign74940_e114409_d_n8;
        locals.var_cnst0over_func_dn9 = assign74940_e114409_d_n9;
        locals.var_cnst0over_func_dn10 = assign74940_e114409_d_n10;
        locals.var_cnst0over_func_dn11 = assign74940_e114409_d_n11;
        locals.var_cnst0over_func_dn14 = assign74940_e114409_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign74950_e114415,) = {
    if ((locals.var_guard1752 != 0.0) && (locals.var_guard1756 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign74950_e114415;
        locals.var_cox0_func_rv = 0.0;

        let assign74960_e114434: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1757 = assign74960_e114434;
        locals.var_guard1757_rv = 0.0;

        let (assign74970_e114443,) = {
    if (((locals.var_guard1753 != 0.0) && (locals.var_guard1752 == 0.0)) && (locals.var_guard1757 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74970_e114443;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74980_e114454, assign74980_e114454_d_n2, assign74980_e114454_d_n7, assign74980_e114454_d_n8, assign74980_e114454_d_n9,) = {
    if (((locals.var_guard1753 != 0.0) && (locals.var_guard1752 == 0.0)) && (locals.var_guard1757 != 0.0)) {
        let assign74980_e114452: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign74980_e114452, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign74980_e114454;
        locals.var_vgbgmt_dn2 = assign74980_e114454_d_n2;
        locals.var_vgbgmt_dn7 = assign74980_e114454_d_n7;
        locals.var_vgbgmt_dn8 = assign74980_e114454_d_n8;
        locals.var_vgbgmt_dn9 = assign74980_e114454_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74990_e114464, assign74990_e114464_d_n0, assign74990_e114464_d_n2, assign74990_e114464_d_n4, assign74990_e114464_d_n5, assign74990_e114464_d_n6, assign74990_e114464_d_n7, assign74990_e114464_d_n8, assign74990_e114464_d_n9, assign74990_e114464_d_n10, assign74990_e114464_d_n11, assign74990_e114464_d_n14,) = {
    if (((locals.var_guard1753 != 0.0) && (locals.var_guard1752 == 0.0)) && (locals.var_guard1757 != 0.0)) {
        let assign74990_e114462: f64 = (-locals.var_vbsei);
        (assign74990_e114462, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign74990_e114464;
        locals.var_vxbgmt_dn0 = assign74990_e114464_d_n0;
        locals.var_vxbgmt_dn2 = assign74990_e114464_d_n2;
        locals.var_vxbgmt_dn4 = assign74990_e114464_d_n4;
        locals.var_vxbgmt_dn5 = assign74990_e114464_d_n5;
        locals.var_vxbgmt_dn6 = assign74990_e114464_d_n6;
        locals.var_vxbgmt_dn7 = assign74990_e114464_d_n7;
        locals.var_vxbgmt_dn8 = assign74990_e114464_d_n8;
        locals.var_vxbgmt_dn9 = assign74990_e114464_d_n9;
        locals.var_vxbgmt_dn10 = assign74990_e114464_d_n10;
        locals.var_vxbgmt_dn11 = assign74990_e114464_d_n11;
        locals.var_vxbgmt_dn14 = assign74990_e114464_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let assign75000_e114475: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1758 = assign75000_e114475;
        locals.var_guard1758_rv = 0.0;

        let (assign75010_e114486,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign75010_e114486;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign75020_e114497,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign75020_e114497;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign75030_e114510, assign75030_e114510_d_n2, assign75030_e114510_d_n7, assign75030_e114510_d_n8, assign75030_e114510_d_n9,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        let assign75030_e114508: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign75030_e114508, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign75030_e114510;
        locals.var_vgbgmt_dn2 = assign75030_e114510_d_n2;
        locals.var_vgbgmt_dn7 = assign75030_e114510_d_n7;
        locals.var_vgbgmt_dn8 = assign75030_e114510_d_n8;
        locals.var_vgbgmt_dn9 = assign75030_e114510_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75040_e114523, assign75040_e114523_d_n0, assign75040_e114523_d_n2, assign75040_e114523_d_n4, assign75040_e114523_d_n5, assign75040_e114523_d_n6, assign75040_e114523_d_n7, assign75040_e114523_d_n8, assign75040_e114523_d_n9, assign75040_e114523_d_n10, assign75040_e114523_d_n11, assign75040_e114523_d_n14,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        let assign75040_e114521: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign75040_e114521, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75040_e114523;
        locals.var_vxbgmt_dn0 = assign75040_e114523_d_n0;
        locals.var_vxbgmt_dn2 = assign75040_e114523_d_n2;
        locals.var_vxbgmt_dn4 = assign75040_e114523_d_n4;
        locals.var_vxbgmt_dn5 = assign75040_e114523_d_n5;
        locals.var_vxbgmt_dn6 = assign75040_e114523_d_n6;
        locals.var_vxbgmt_dn7 = assign75040_e114523_d_n7;
        locals.var_vxbgmt_dn8 = assign75040_e114523_d_n8;
        locals.var_vxbgmt_dn9 = assign75040_e114523_d_n9;
        locals.var_vxbgmt_dn10 = assign75040_e114523_d_n10;
        locals.var_vxbgmt_dn11 = assign75040_e114523_d_n11;
        locals.var_vxbgmt_dn14 = assign75040_e114523_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75050_e114534,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign75050_e114534;
        locals.var_nover_func_rv = 0.0;

        let (assign75060_e114549, assign75060_e114549_d_n0, assign75060_e114549_d_n2, assign75060_e114549_d_n4, assign75060_e114549_d_n5, assign75060_e114549_d_n6, assign75060_e114549_d_n7, assign75060_e114549_d_n8, assign75060_e114549_d_n9, assign75060_e114549_d_n10, assign75060_e114549_d_n11, assign75060_e114549_d_n14,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        let assign75060_e114546: f64 = (p.p64 * p.p55);
        let assign75060_e114547: f64 = (p.p63 + assign75060_e114546);
        (assign75060_e114547, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75060_e114549;
        locals.var_lover_func_dn0 = assign75060_e114549_d_n0;
        locals.var_lover_func_dn2 = assign75060_e114549_d_n2;
        locals.var_lover_func_dn4 = assign75060_e114549_d_n4;
        locals.var_lover_func_dn5 = assign75060_e114549_d_n5;
        locals.var_lover_func_dn6 = assign75060_e114549_d_n6;
        locals.var_lover_func_dn7 = assign75060_e114549_d_n7;
        locals.var_lover_func_dn8 = assign75060_e114549_d_n8;
        locals.var_lover_func_dn9 = assign75060_e114549_d_n9;
        locals.var_lover_func_dn10 = assign75060_e114549_d_n10;
        locals.var_lover_func_dn11 = assign75060_e114549_d_n11;
        locals.var_lover_func_dn14 = assign75060_e114549_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign75070_e114560, assign75070_e114560_d_n0, assign75070_e114560_d_n2, assign75070_e114560_d_n4, assign75070_e114560_d_n5, assign75070_e114560_d_n6, assign75070_e114560_d_n7, assign75070_e114560_d_n8, assign75070_e114560_d_n9, assign75070_e114560_d_n10, assign75070_e114560_d_n11, assign75070_e114560_d_n14,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign75070_e114560;
        locals.var_wdep_func_dn0 = assign75070_e114560_d_n0;
        locals.var_wdep_func_dn2 = assign75070_e114560_d_n2;
        locals.var_wdep_func_dn4 = assign75070_e114560_d_n4;
        locals.var_wdep_func_dn5 = assign75070_e114560_d_n5;
        locals.var_wdep_func_dn6 = assign75070_e114560_d_n6;
        locals.var_wdep_func_dn7 = assign75070_e114560_d_n7;
        locals.var_wdep_func_dn8 = assign75070_e114560_d_n8;
        locals.var_wdep_func_dn9 = assign75070_e114560_d_n9;
        locals.var_wdep_func_dn10 = assign75070_e114560_d_n10;
        locals.var_wdep_func_dn11 = assign75070_e114560_d_n11;
        locals.var_wdep_func_dn14 = assign75070_e114560_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign75080_e114571, assign75080_e114571_d_n0, assign75080_e114571_d_n2, assign75080_e114571_d_n4, assign75080_e114571_d_n5, assign75080_e114571_d_n6, assign75080_e114571_d_n7, assign75080_e114571_d_n8, assign75080_e114571_d_n9, assign75080_e114571_d_n10, assign75080_e114571_d_n11, assign75080_e114571_d_n14,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign75080_e114571;
        locals.var_cnst0over_func_dn0 = assign75080_e114571_d_n0;
        locals.var_cnst0over_func_dn2 = assign75080_e114571_d_n2;
        locals.var_cnst0over_func_dn4 = assign75080_e114571_d_n4;
        locals.var_cnst0over_func_dn5 = assign75080_e114571_d_n5;
        locals.var_cnst0over_func_dn6 = assign75080_e114571_d_n6;
        locals.var_cnst0over_func_dn7 = assign75080_e114571_d_n7;
        locals.var_cnst0over_func_dn8 = assign75080_e114571_d_n8;
        locals.var_cnst0over_func_dn9 = assign75080_e114571_d_n9;
        locals.var_cnst0over_func_dn10 = assign75080_e114571_d_n10;
        locals.var_cnst0over_func_dn11 = assign75080_e114571_d_n11;
        locals.var_cnst0over_func_dn14 = assign75080_e114571_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign75090_e114582,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign75090_e114582;
        locals.var_cox0_func_rv = 0.0;

        let (assign75100_e114594, assign75100_e114594_d_n0, assign75100_e114594_d_n2, assign75100_e114594_d_n4, assign75100_e114594_d_n5, assign75100_e114594_d_n6, assign75100_e114594_d_n7, assign75100_e114594_d_n8, assign75100_e114594_d_n9, assign75100_e114594_d_n10, assign75100_e114594_d_n11, assign75100_e114594_d_n14,) = {
    if (((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) {
        let assign75100_e114592: f64 = (-locals.var_lover_func);
        (assign75100_e114592, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75100_e114594;
        locals.var_lover_func_dn0 = assign75100_e114594_d_n0;
        locals.var_lover_func_dn2 = assign75100_e114594_d_n2;
        locals.var_lover_func_dn4 = assign75100_e114594_d_n4;
        locals.var_lover_func_dn5 = assign75100_e114594_d_n5;
        locals.var_lover_func_dn6 = assign75100_e114594_d_n6;
        locals.var_lover_func_dn7 = assign75100_e114594_d_n7;
        locals.var_lover_func_dn8 = assign75100_e114594_d_n8;
        locals.var_lover_func_dn9 = assign75100_e114594_d_n9;
        locals.var_lover_func_dn10 = assign75100_e114594_d_n10;
        locals.var_lover_func_dn11 = assign75100_e114594_d_n11;
        locals.var_lover_func_dn14 = assign75100_e114594_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign75110_e114605: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1759 = assign75110_e114605;
        locals.var_guard1759_rv = 0.0;

        let (assign75120_e114619, assign75120_e114619_d_n0, assign75120_e114619_d_n2, assign75120_e114619_d_n4, assign75120_e114619_d_n5, assign75120_e114619_d_n6, assign75120_e114619_d_n7, assign75120_e114619_d_n8, assign75120_e114619_d_n9, assign75120_e114619_d_n10, assign75120_e114619_d_n11, assign75120_e114619_d_n14,) = {
    if ((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) {
        let assign75120_e114617: f64 = (-locals.var_lover_func);
        (assign75120_e114617, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75120_e114619;
        locals.var_lover_func_dn0 = assign75120_e114619_d_n0;
        locals.var_lover_func_dn2 = assign75120_e114619_d_n2;
        locals.var_lover_func_dn4 = assign75120_e114619_d_n4;
        locals.var_lover_func_dn5 = assign75120_e114619_d_n5;
        locals.var_lover_func_dn6 = assign75120_e114619_d_n6;
        locals.var_lover_func_dn7 = assign75120_e114619_d_n7;
        locals.var_lover_func_dn8 = assign75120_e114619_d_n8;
        locals.var_lover_func_dn9 = assign75120_e114619_d_n9;
        locals.var_lover_func_dn10 = assign75120_e114619_d_n10;
        locals.var_lover_func_dn11 = assign75120_e114619_d_n11;
        locals.var_lover_func_dn14 = assign75120_e114619_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign75130_e114632, assign75130_e114632_d_n0, assign75130_e114632_d_n2, assign75130_e114632_d_n4, assign75130_e114632_d_n5, assign75130_e114632_d_n6, assign75130_e114632_d_n7, assign75130_e114632_d_n8, assign75130_e114632_d_n9, assign75130_e114632_d_n10, assign75130_e114632_d_n11, assign75130_e114632_d_n14,) = {
    if ((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign75130_e114632;
        locals.var_t1_dn0 = assign75130_e114632_d_n0;
        locals.var_t1_dn2 = assign75130_e114632_d_n2;
        locals.var_t1_dn4 = assign75130_e114632_d_n4;
        locals.var_t1_dn5 = assign75130_e114632_d_n5;
        locals.var_t1_dn6 = assign75130_e114632_d_n6;
        locals.var_t1_dn7 = assign75130_e114632_d_n7;
        locals.var_t1_dn8 = assign75130_e114632_d_n8;
        locals.var_t1_dn9 = assign75130_e114632_d_n9;
        locals.var_t1_dn10 = assign75130_e114632_d_n10;
        locals.var_t1_dn11 = assign75130_e114632_d_n11;
        locals.var_t1_dn14 = assign75130_e114632_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign75140_e114651, assign75140_e114651_d_n0, assign75140_e114651_d_n2, assign75140_e114651_d_n4, assign75140_e114651_d_n5, assign75140_e114651_d_n6, assign75140_e114651_d_n7, assign75140_e114651_d_n8, assign75140_e114651_d_n9, assign75140_e114651_d_n10, assign75140_e114651_d_n11, assign75140_e114651_d_n14,) = {
    if ((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) {
        let assign75140_e114645: f64 = (locals.var_t1 * locals.var_t1);
        let assign75140_e114647: f64 = (assign75140_e114645 / locals.var_kjunc);
        let assign75140_e114649: f64 = (assign75140_e114647 - p.p137);
        (assign75140_e114649, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn11)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) * locals.var_kjunc) - (assign75140_e114645 * locals.var_kjunc_dn14)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn11, locals.var_vxb_lim_dn14,)
    }
};
        locals.var_vxb_lim = assign75140_e114651;
        locals.var_vxb_lim_dn0 = assign75140_e114651_d_n0;
        locals.var_vxb_lim_dn2 = assign75140_e114651_d_n2;
        locals.var_vxb_lim_dn4 = assign75140_e114651_d_n4;
        locals.var_vxb_lim_dn5 = assign75140_e114651_d_n5;
        locals.var_vxb_lim_dn6 = assign75140_e114651_d_n6;
        locals.var_vxb_lim_dn7 = assign75140_e114651_d_n7;
        locals.var_vxb_lim_dn8 = assign75140_e114651_d_n8;
        locals.var_vxb_lim_dn9 = assign75140_e114651_d_n9;
        locals.var_vxb_lim_dn10 = assign75140_e114651_d_n10;
        locals.var_vxb_lim_dn11 = assign75140_e114651_d_n11;
        locals.var_vxb_lim_dn14 = assign75140_e114651_d_n14;
        locals.var_vxb_lim_rv = 0.0;

        let assign75150_e114654: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign75150_e114654;
        locals.var_guard1760_rv = 0.0;

        let assign75160_e114661: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1761 = assign75160_e114661;
        locals.var_guard1761_rv = 0.0;

        let (assign75170_e114678, assign75170_e114678_d_n0, assign75170_e114678_d_n2, assign75170_e114678_d_n4, assign75170_e114678_d_n5, assign75170_e114678_d_n6, assign75170_e114678_d_n7, assign75170_e114678_d_n8, assign75170_e114678_d_n9, assign75170_e114678_d_n10, assign75170_e114678_d_n11, assign75170_e114678_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75170_e114678;
        locals.var_vxbgmt_dn0 = assign75170_e114678_d_n0;
        locals.var_vxbgmt_dn2 = assign75170_e114678_d_n2;
        locals.var_vxbgmt_dn4 = assign75170_e114678_d_n4;
        locals.var_vxbgmt_dn5 = assign75170_e114678_d_n5;
        locals.var_vxbgmt_dn6 = assign75170_e114678_d_n6;
        locals.var_vxbgmt_dn7 = assign75170_e114678_d_n7;
        locals.var_vxbgmt_dn8 = assign75170_e114678_d_n8;
        locals.var_vxbgmt_dn9 = assign75170_e114678_d_n9;
        locals.var_vxbgmt_dn10 = assign75170_e114678_d_n10;
        locals.var_vxbgmt_dn11 = assign75170_e114678_d_n11;
        locals.var_vxbgmt_dn14 = assign75170_e114678_d_n14;
        locals.var_vxbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_284(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign75180_e114702, assign75180_e114702_d_n0, assign75180_e114702_d_n2, assign75180_e114702_d_n4, assign75180_e114702_d_n5, assign75180_e114702_d_n6, assign75180_e114702_d_n7, assign75180_e114702_d_n8, assign75180_e114702_d_n9, assign75180_e114702_d_n10, assign75180_e114702_d_n11, assign75180_e114702_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 == 0.0)) {
        let (assign75180_e114700,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign75180_e114698: f64 = (-1.0);
                (assign75180_e114698,)
            } else {
                (1.0,)
            }
        };
        (assign75180_e114700, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign75180_e114702;
        locals.var_tmf3_dn0 = assign75180_e114702_d_n0;
        locals.var_tmf3_dn2 = assign75180_e114702_d_n2;
        locals.var_tmf3_dn4 = assign75180_e114702_d_n4;
        locals.var_tmf3_dn5 = assign75180_e114702_d_n5;
        locals.var_tmf3_dn6 = assign75180_e114702_d_n6;
        locals.var_tmf3_dn7 = assign75180_e114702_d_n7;
        locals.var_tmf3_dn8 = assign75180_e114702_d_n8;
        locals.var_tmf3_dn9 = assign75180_e114702_d_n9;
        locals.var_tmf3_dn10 = assign75180_e114702_d_n10;
        locals.var_tmf3_dn11 = assign75180_e114702_d_n11;
        locals.var_tmf3_dn14 = assign75180_e114702_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign75190_e114722, assign75190_e114722_d_n0, assign75190_e114722_d_n2, assign75190_e114722_d_n4, assign75190_e114722_d_n5, assign75190_e114722_d_n6, assign75190_e114722_d_n7, assign75190_e114722_d_n8, assign75190_e114722_d_n9, assign75190_e114722_d_n10, assign75190_e114722_d_n11, assign75190_e114722_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 == 0.0)) {
        let assign75190_e114720: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign75190_e114720, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign75190_e114722;
        locals.var_tmf4_dn0 = assign75190_e114722_d_n0;
        locals.var_tmf4_dn2 = assign75190_e114722_d_n2;
        locals.var_tmf4_dn4 = assign75190_e114722_d_n4;
        locals.var_tmf4_dn5 = assign75190_e114722_d_n5;
        locals.var_tmf4_dn6 = assign75190_e114722_d_n6;
        locals.var_tmf4_dn7 = assign75190_e114722_d_n7;
        locals.var_tmf4_dn8 = assign75190_e114722_d_n8;
        locals.var_tmf4_dn9 = assign75190_e114722_d_n9;
        locals.var_tmf4_dn10 = assign75190_e114722_d_n10;
        locals.var_tmf4_dn11 = assign75190_e114722_d_n11;
        locals.var_tmf4_dn14 = assign75190_e114722_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign75200_e114746, assign75200_e114746_d_n0, assign75200_e114746_d_n2, assign75200_e114746_d_n4, assign75200_e114746_d_n5, assign75200_e114746_d_n6, assign75200_e114746_d_n7, assign75200_e114746_d_n8, assign75200_e114746_d_n9, assign75200_e114746_d_n10, assign75200_e114746_d_n11, assign75200_e114746_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 == 0.0)) {
        let assign75200_e114741: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign75200_e114743: f64 = (assign75200_e114741).powf(p.p113);
        let assign75200_e114744: f64 = (1.0 + assign75200_e114743);
        (assign75200_e114744, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75200_e114741).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75200_e114743 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75200_e114741))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75200_e114746;
        locals.var_tmf1_dn0 = assign75200_e114746_d_n0;
        locals.var_tmf1_dn2 = assign75200_e114746_d_n2;
        locals.var_tmf1_dn4 = assign75200_e114746_d_n4;
        locals.var_tmf1_dn5 = assign75200_e114746_d_n5;
        locals.var_tmf1_dn6 = assign75200_e114746_d_n6;
        locals.var_tmf1_dn7 = assign75200_e114746_d_n7;
        locals.var_tmf1_dn8 = assign75200_e114746_d_n8;
        locals.var_tmf1_dn9 = assign75200_e114746_d_n9;
        locals.var_tmf1_dn10 = assign75200_e114746_d_n10;
        locals.var_tmf1_dn11 = assign75200_e114746_d_n11;
        locals.var_tmf1_dn14 = assign75200_e114746_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75210_e114768, assign75210_e114768_d_n0, assign75210_e114768_d_n2, assign75210_e114768_d_n4, assign75210_e114768_d_n5, assign75210_e114768_d_n6, assign75210_e114768_d_n7, assign75210_e114768_d_n8, assign75210_e114768_d_n9, assign75210_e114768_d_n10, assign75210_e114768_d_n11, assign75210_e114768_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 == 0.0)) {
        let assign75210_e114765: f64 = (1.0 / p.p113);
        let assign75210_e114766: f64 = (locals.var_tmf1).powf(assign75210_e114765);
        (assign75210_e114766, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75210_e114765) as f64).is_finite() && ((assign75210_e114765) as f64).fract() == 0.0 { if assign75210_e114765 == 0.0 { 0.0 } else { (assign75210_e114765 * ((locals.var_tmf1).powf(assign75210_e114765 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign75210_e114766 * (assign75210_e114765 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75210_e114768;
        locals.var_tmf2_dn0 = assign75210_e114768_d_n0;
        locals.var_tmf2_dn2 = assign75210_e114768_d_n2;
        locals.var_tmf2_dn4 = assign75210_e114768_d_n4;
        locals.var_tmf2_dn5 = assign75210_e114768_d_n5;
        locals.var_tmf2_dn6 = assign75210_e114768_d_n6;
        locals.var_tmf2_dn7 = assign75210_e114768_d_n7;
        locals.var_tmf2_dn8 = assign75210_e114768_d_n8;
        locals.var_tmf2_dn9 = assign75210_e114768_d_n9;
        locals.var_tmf2_dn10 = assign75210_e114768_d_n10;
        locals.var_tmf2_dn11 = assign75210_e114768_d_n11;
        locals.var_tmf2_dn14 = assign75210_e114768_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75220_e114790, assign75220_e114790_d_n0, assign75220_e114790_d_n2, assign75220_e114790_d_n4, assign75220_e114790_d_n5, assign75220_e114790_d_n6, assign75220_e114790_d_n7, assign75220_e114790_d_n8, assign75220_e114790_d_n9, assign75220_e114790_d_n10, assign75220_e114790_d_n11, assign75220_e114790_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1761 == 0.0)) {
        let assign75220_e114786: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign75220_e114788: f64 = (assign75220_e114786 / locals.var_tmf2);
        (assign75220_e114788, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign75220_e114786 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75220_e114790;
        locals.var_vxbgmt_dn0 = assign75220_e114790_d_n0;
        locals.var_vxbgmt_dn2 = assign75220_e114790_d_n2;
        locals.var_vxbgmt_dn4 = assign75220_e114790_d_n4;
        locals.var_vxbgmt_dn5 = assign75220_e114790_d_n5;
        locals.var_vxbgmt_dn6 = assign75220_e114790_d_n6;
        locals.var_vxbgmt_dn7 = assign75220_e114790_d_n7;
        locals.var_vxbgmt_dn8 = assign75220_e114790_d_n8;
        locals.var_vxbgmt_dn9 = assign75220_e114790_d_n9;
        locals.var_vxbgmt_dn10 = assign75220_e114790_d_n10;
        locals.var_vxbgmt_dn11 = assign75220_e114790_d_n11;
        locals.var_vxbgmt_dn14 = assign75220_e114790_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75230_e114818, assign75230_e114818_d_n0, assign75230_e114818_d_n2, assign75230_e114818_d_n4, assign75230_e114818_d_n5, assign75230_e114818_d_n6, assign75230_e114818_d_n7, assign75230_e114818_d_n8, assign75230_e114818_d_n9, assign75230_e114818_d_n10, assign75230_e114818_d_n11, assign75230_e114818_d_n14,) = {
    if (((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign75230_e114805: f64 = (locals.var_vxbgmt + p.p137);
        let assign75230_e114808: f64 = (locals.var_vxbgmt + p.p137);
        let assign75230_e114809: f64 = (assign75230_e114805 * assign75230_e114808);
        let assign75230_e114812: f64 = (4.0 * 0.1);
        let assign75230_e114814: f64 = (assign75230_e114812 * 0.1);
        let assign75230_e114815: f64 = (assign75230_e114809 + assign75230_e114814);
        let assign75230_e114816: f64 = (assign75230_e114815).sqrt();
        (assign75230_e114816, (((locals.var_vxbgmt_dn0 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn0)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn2 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn2)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn4 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn4)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn5 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn5)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn6 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn6)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn7 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn7)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn8 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn8)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn9 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn9)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn10 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn10)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn11 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn11)) / (2.0 * assign75230_e114816)), (((locals.var_vxbgmt_dn14 * assign75230_e114808) + (assign75230_e114805 * locals.var_vxbgmt_dn14)) / (2.0 * assign75230_e114816)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75230_e114818;
        locals.var_tmf2_dn0 = assign75230_e114818_d_n0;
        locals.var_tmf2_dn2 = assign75230_e114818_d_n2;
        locals.var_tmf2_dn4 = assign75230_e114818_d_n4;
        locals.var_tmf2_dn5 = assign75230_e114818_d_n5;
        locals.var_tmf2_dn6 = assign75230_e114818_d_n6;
        locals.var_tmf2_dn7 = assign75230_e114818_d_n7;
        locals.var_tmf2_dn8 = assign75230_e114818_d_n8;
        locals.var_tmf2_dn9 = assign75230_e114818_d_n9;
        locals.var_tmf2_dn10 = assign75230_e114818_d_n10;
        locals.var_tmf2_dn11 = assign75230_e114818_d_n11;
        locals.var_tmf2_dn14 = assign75230_e114818_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75240_e114841, assign75240_e114841_d_n0, assign75240_e114841_d_n2, assign75240_e114841_d_n4, assign75240_e114841_d_n5, assign75240_e114841_d_n6, assign75240_e114841_d_n7, assign75240_e114841_d_n8, assign75240_e114841_d_n9, assign75240_e114841_d_n10, assign75240_e114841_d_n11, assign75240_e114841_d_n14,) = {
    if (((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign75240_e114835: f64 = (locals.var_vxbgmt + p.p137);
        let assign75240_e114837: f64 = (assign75240_e114835 / locals.var_tmf2);
        let assign75240_e114838: f64 = (1.0 + assign75240_e114837);
        let assign75240_e114839: f64 = (0.5 * assign75240_e114838);
        (assign75240_e114839, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign75240_e114835 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign75240_e114841;
        locals.var_t9_dn0 = assign75240_e114841_d_n0;
        locals.var_t9_dn2 = assign75240_e114841_d_n2;
        locals.var_t9_dn4 = assign75240_e114841_d_n4;
        locals.var_t9_dn5 = assign75240_e114841_d_n5;
        locals.var_t9_dn6 = assign75240_e114841_d_n6;
        locals.var_t9_dn7 = assign75240_e114841_d_n7;
        locals.var_t9_dn8 = assign75240_e114841_d_n8;
        locals.var_t9_dn9 = assign75240_e114841_d_n9;
        locals.var_t9_dn10 = assign75240_e114841_d_n10;
        locals.var_t9_dn11 = assign75240_e114841_d_n11;
        locals.var_t9_dn14 = assign75240_e114841_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign75250_e114862, assign75250_e114862_d_n0, assign75250_e114862_d_n2, assign75250_e114862_d_n4, assign75250_e114862_d_n5, assign75250_e114862_d_n6, assign75250_e114862_d_n7, assign75250_e114862_d_n8, assign75250_e114862_d_n9, assign75250_e114862_d_n10, assign75250_e114862_d_n11, assign75250_e114862_d_n14,) = {
    if (((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign75250_e114857: f64 = (locals.var_vxbgmt + p.p137);
        let assign75250_e114859: f64 = (assign75250_e114857 + locals.var_tmf2);
        let assign75250_e114860: f64 = (0.5 * assign75250_e114859);
        (assign75250_e114860, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75250_e114862;
        locals.var_t2_dn0 = assign75250_e114862_d_n0;
        locals.var_t2_dn2 = assign75250_e114862_d_n2;
        locals.var_t2_dn4 = assign75250_e114862_d_n4;
        locals.var_t2_dn5 = assign75250_e114862_d_n5;
        locals.var_t2_dn6 = assign75250_e114862_d_n6;
        locals.var_t2_dn7 = assign75250_e114862_d_n7;
        locals.var_t2_dn8 = assign75250_e114862_d_n8;
        locals.var_t2_dn9 = assign75250_e114862_d_n9;
        locals.var_t2_dn10 = assign75250_e114862_d_n10;
        locals.var_t2_dn11 = assign75250_e114862_d_n11;
        locals.var_t2_dn14 = assign75250_e114862_d_n14;
        locals.var_t2_rv = 0.0;

        let assign75260_e114865: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1762 = assign75260_e114865;
        locals.var_guard1762_rv = 0.0;

        let (assign75270_e114882, assign75270_e114882_d_n0, assign75270_e114882_d_n2, assign75270_e114882_d_n4, assign75270_e114882_d_n5, assign75270_e114882_d_n6, assign75270_e114882_d_n7, assign75270_e114882_d_n8, assign75270_e114882_d_n9, assign75270_e114882_d_n10, assign75270_e114882_d_n11, assign75270_e114882_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75270_e114882;
        locals.var_t2_dn0 = assign75270_e114882_d_n0;
        locals.var_t2_dn2 = assign75270_e114882_d_n2;
        locals.var_t2_dn4 = assign75270_e114882_d_n4;
        locals.var_t2_dn5 = assign75270_e114882_d_n5;
        locals.var_t2_dn6 = assign75270_e114882_d_n6;
        locals.var_t2_dn7 = assign75270_e114882_d_n7;
        locals.var_t2_dn8 = assign75270_e114882_d_n8;
        locals.var_t2_dn9 = assign75270_e114882_d_n9;
        locals.var_t2_dn10 = assign75270_e114882_d_n10;
        locals.var_t2_dn11 = assign75270_e114882_d_n11;
        locals.var_t2_dn14 = assign75270_e114882_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign75280_e114899, assign75280_e114899_d_n0, assign75280_e114899_d_n2, assign75280_e114899_d_n4, assign75280_e114899_d_n5, assign75280_e114899_d_n6, assign75280_e114899_d_n7, assign75280_e114899_d_n8, assign75280_e114899_d_n9, assign75280_e114899_d_n10, assign75280_e114899_d_n11, assign75280_e114899_d_n14,) = {
    if ((((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) && (locals.var_guard1762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign75280_e114899;
        locals.var_t9_dn0 = assign75280_e114899_d_n0;
        locals.var_t9_dn2 = assign75280_e114899_d_n2;
        locals.var_t9_dn4 = assign75280_e114899_d_n4;
        locals.var_t9_dn5 = assign75280_e114899_d_n5;
        locals.var_t9_dn6 = assign75280_e114899_d_n6;
        locals.var_t9_dn7 = assign75280_e114899_d_n7;
        locals.var_t9_dn8 = assign75280_e114899_d_n8;
        locals.var_t9_dn9 = assign75280_e114899_d_n9;
        locals.var_t9_dn10 = assign75280_e114899_d_n10;
        locals.var_t9_dn11 = assign75280_e114899_d_n11;
        locals.var_t9_dn14 = assign75280_e114899_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign75290_e114919, assign75290_e114919_d_n0, assign75290_e114919_d_n2, assign75290_e114919_d_n4, assign75290_e114919_d_n5, assign75290_e114919_d_n6, assign75290_e114919_d_n7, assign75290_e114919_d_n8, assign75290_e114919_d_n9, assign75290_e114919_d_n10, assign75290_e114919_d_n11, assign75290_e114919_d_n14,) = {
    if (((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign75290_e114914: f64 = (locals.var_kjunc * locals.var_t2);
        let assign75290_e114915: f64 = (assign75290_e114914).sqrt();
        let assign75290_e114917: f64 = (assign75290_e114915 * p.p432);
        (assign75290_e114917, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign75290_e114915)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign75290_e114915)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign75290_e114919;
        locals.var_wjunc0_dn0 = assign75290_e114919_d_n0;
        locals.var_wjunc0_dn2 = assign75290_e114919_d_n2;
        locals.var_wjunc0_dn4 = assign75290_e114919_d_n4;
        locals.var_wjunc0_dn5 = assign75290_e114919_d_n5;
        locals.var_wjunc0_dn6 = assign75290_e114919_d_n6;
        locals.var_wjunc0_dn7 = assign75290_e114919_d_n7;
        locals.var_wjunc0_dn8 = assign75290_e114919_d_n8;
        locals.var_wjunc0_dn9 = assign75290_e114919_d_n9;
        locals.var_wjunc0_dn10 = assign75290_e114919_d_n10;
        locals.var_wjunc0_dn11 = assign75290_e114919_d_n11;
        locals.var_wjunc0_dn14 = assign75290_e114919_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign75300_e114936, assign75300_e114936_d_n0, assign75300_e114936_d_n2, assign75300_e114936_d_n4, assign75300_e114936_d_n5, assign75300_e114936_d_n6, assign75300_e114936_d_n7, assign75300_e114936_d_n8, assign75300_e114936_d_n9, assign75300_e114936_d_n10, assign75300_e114936_d_n11, assign75300_e114936_d_n14,) = {
    if (((((locals.var_guard1754 != 0.0) && (!((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)))) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign75300_e114934: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign75300_e114934, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75300_e114936;
        locals.var_lover_func_dn0 = assign75300_e114936_d_n0;
        locals.var_lover_func_dn2 = assign75300_e114936_d_n2;
        locals.var_lover_func_dn4 = assign75300_e114936_d_n4;
        locals.var_lover_func_dn5 = assign75300_e114936_d_n5;
        locals.var_lover_func_dn6 = assign75300_e114936_d_n6;
        locals.var_lover_func_dn7 = assign75300_e114936_d_n7;
        locals.var_lover_func_dn8 = assign75300_e114936_d_n8;
        locals.var_lover_func_dn9 = assign75300_e114936_d_n9;
        locals.var_lover_func_dn10 = assign75300_e114936_d_n10;
        locals.var_lover_func_dn11 = assign75300_e114936_d_n11;
        locals.var_lover_func_dn14 = assign75300_e114936_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign75310_e114955: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1763 = assign75310_e114955;
        locals.var_guard1763_rv = 0.0;

        let (assign75320_e114968,) = {
    if (((locals.var_guard1755 != 0.0) && (!(((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)) || (locals.var_guard1754 != 0.0)))) && (locals.var_guard1763 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign75320_e114968;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign75330_e114983, assign75330_e114983_d_n2, assign75330_e114983_d_n7, assign75330_e114983_d_n8, assign75330_e114983_d_n9,) = {
    if (((locals.var_guard1755 != 0.0) && (!(((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)) || (locals.var_guard1754 != 0.0)))) && (locals.var_guard1763 != 0.0)) {
        let assign75330_e114981: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign75330_e114981, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign75330_e114983;
        locals.var_vgbgmt_dn2 = assign75330_e114983_d_n2;
        locals.var_vgbgmt_dn7 = assign75330_e114983_d_n7;
        locals.var_vgbgmt_dn8 = assign75330_e114983_d_n8;
        locals.var_vgbgmt_dn9 = assign75330_e114983_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75340_e114998, assign75340_e114998_d_n0, assign75340_e114998_d_n2, assign75340_e114998_d_n4, assign75340_e114998_d_n5, assign75340_e114998_d_n6, assign75340_e114998_d_n7, assign75340_e114998_d_n8, assign75340_e114998_d_n9, assign75340_e114998_d_n10, assign75340_e114998_d_n11, assign75340_e114998_d_n14,) = {
    if (((locals.var_guard1755 != 0.0) && (!(((locals.var_guard1752 != 0.0) || (locals.var_guard1753 != 0.0)) || (locals.var_guard1754 != 0.0)))) && (locals.var_guard1763 != 0.0)) {
        let assign75340_e114996: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign75340_e114996, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75340_e114998;
        locals.var_vxbgmt_dn0 = assign75340_e114998_d_n0;
        locals.var_vxbgmt_dn2 = assign75340_e114998_d_n2;
        locals.var_vxbgmt_dn4 = assign75340_e114998_d_n4;
        locals.var_vxbgmt_dn5 = assign75340_e114998_d_n5;
        locals.var_vxbgmt_dn6 = assign75340_e114998_d_n6;
        locals.var_vxbgmt_dn7 = assign75340_e114998_d_n7;
        locals.var_vxbgmt_dn8 = assign75340_e114998_d_n8;
        locals.var_vxbgmt_dn9 = assign75340_e114998_d_n9;
        locals.var_vxbgmt_dn10 = assign75340_e114998_d_n10;
        locals.var_vxbgmt_dn11 = assign75340_e114998_d_n11;
        locals.var_vxbgmt_dn14 = assign75340_e114998_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75350_e115002, assign75350_e115002_d_n0, assign75350_e115002_d_n2, assign75350_e115002_d_n4, assign75350_e115002_d_n5, assign75350_e115002_d_n6, assign75350_e115002_d_n7, assign75350_e115002_d_n8, assign75350_e115002_d_n9, assign75350_e115002_d_n10, assign75350_e115002_d_n11, assign75350_e115002_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75350_e115002;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75350_e115002_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75350_e115002_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75350_e115002_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75350_e115002_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75350_e115002_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75350_e115002_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75350_e115002_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75350_e115002_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75350_e115002_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75350_e115002_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75350_e115002_d_n14;
        locals.var_vbs_bnd_over__blk1771_rv = 0.0;

        let (assign75370_e115010,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk1772,)
    }
};
        locals.var_flg_fd_mode__blk1772 = assign75370_e115010;
        locals.var_flg_fd_mode__blk1772_rv = 0.0;

        let (assign75380_e115014, assign75380_e115014_d_n0, assign75380_e115014_d_n2, assign75380_e115014_d_n4, assign75380_e115014_d_n5, assign75380_e115014_d_n6, assign75380_e115014_d_n7, assign75380_e115014_d_n8, assign75380_e115014_d_n9, assign75380_e115014_d_n10, assign75380_e115014_d_n11, assign75380_e115014_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign75380_e115014;
        locals.var_fb_dn0 = assign75380_e115014_d_n0;
        locals.var_fb_dn2 = assign75380_e115014_d_n2;
        locals.var_fb_dn4 = assign75380_e115014_d_n4;
        locals.var_fb_dn5 = assign75380_e115014_d_n5;
        locals.var_fb_dn6 = assign75380_e115014_d_n6;
        locals.var_fb_dn7 = assign75380_e115014_d_n7;
        locals.var_fb_dn8 = assign75380_e115014_d_n8;
        locals.var_fb_dn9 = assign75380_e115014_d_n9;
        locals.var_fb_dn10 = assign75380_e115014_d_n10;
        locals.var_fb_dn11 = assign75380_e115014_d_n11;
        locals.var_fb_dn14 = assign75380_e115014_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign75390_e115018, assign75390_e115018_d_n0, assign75390_e115018_d_n2, assign75390_e115018_d_n4, assign75390_e115018_d_n5, assign75390_e115018_d_n6, assign75390_e115018_d_n7, assign75390_e115018_d_n8, assign75390_e115018_d_n9, assign75390_e115018_d_n10, assign75390_e115018_d_n11, assign75390_e115018_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign75390_e115018;
        locals.var_fs01_dn0 = assign75390_e115018_d_n0;
        locals.var_fs01_dn2 = assign75390_e115018_d_n2;
        locals.var_fs01_dn4 = assign75390_e115018_d_n4;
        locals.var_fs01_dn5 = assign75390_e115018_d_n5;
        locals.var_fs01_dn6 = assign75390_e115018_d_n6;
        locals.var_fs01_dn7 = assign75390_e115018_d_n7;
        locals.var_fs01_dn8 = assign75390_e115018_d_n8;
        locals.var_fs01_dn9 = assign75390_e115018_d_n9;
        locals.var_fs01_dn10 = assign75390_e115018_d_n10;
        locals.var_fs01_dn11 = assign75390_e115018_d_n11;
        locals.var_fs01_dn14 = assign75390_e115018_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign75400_e115022, assign75400_e115022_d_n0, assign75400_e115022_d_n2, assign75400_e115022_d_n4, assign75400_e115022_d_n5, assign75400_e115022_d_n6, assign75400_e115022_d_n7, assign75400_e115022_d_n8, assign75400_e115022_d_n9, assign75400_e115022_d_n10, assign75400_e115022_d_n11, assign75400_e115022_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign75400_e115022;
        locals.var_fs02_dn0 = assign75400_e115022_d_n0;
        locals.var_fs02_dn2 = assign75400_e115022_d_n2;
        locals.var_fs02_dn4 = assign75400_e115022_d_n4;
        locals.var_fs02_dn5 = assign75400_e115022_d_n5;
        locals.var_fs02_dn6 = assign75400_e115022_d_n6;
        locals.var_fs02_dn7 = assign75400_e115022_d_n7;
        locals.var_fs02_dn8 = assign75400_e115022_d_n8;
        locals.var_fs02_dn9 = assign75400_e115022_d_n9;
        locals.var_fs02_dn10 = assign75400_e115022_d_n10;
        locals.var_fs02_dn11 = assign75400_e115022_d_n11;
        locals.var_fs02_dn14 = assign75400_e115022_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign75410_e115026, assign75410_e115026_d_n0, assign75410_e115026_d_n2, assign75410_e115026_d_n4, assign75410_e115026_d_n5, assign75410_e115026_d_n6, assign75410_e115026_d_n7, assign75410_e115026_d_n8, assign75410_e115026_d_n9, assign75410_e115026_d_n10, assign75410_e115026_d_n11, assign75410_e115026_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign75410_e115026;
        locals.var_fs0_dn0 = assign75410_e115026_d_n0;
        locals.var_fs0_dn2 = assign75410_e115026_d_n2;
        locals.var_fs0_dn4 = assign75410_e115026_d_n4;
        locals.var_fs0_dn5 = assign75410_e115026_d_n5;
        locals.var_fs0_dn6 = assign75410_e115026_d_n6;
        locals.var_fs0_dn7 = assign75410_e115026_d_n7;
        locals.var_fs0_dn8 = assign75410_e115026_d_n8;
        locals.var_fs0_dn9 = assign75410_e115026_d_n9;
        locals.var_fs0_dn10 = assign75410_e115026_d_n10;
        locals.var_fs0_dn11 = assign75410_e115026_d_n11;
        locals.var_fs0_dn14 = assign75410_e115026_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign75420_e115030, assign75420_e115030_d_n0, assign75420_e115030_d_n2, assign75420_e115030_d_n4, assign75420_e115030_d_n5, assign75420_e115030_d_n6, assign75420_e115030_d_n7, assign75420_e115030_d_n8, assign75420_e115030_d_n9, assign75420_e115030_d_n10, assign75420_e115030_d_n11, assign75420_e115030_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign75420_e115030;
        locals.var_dps0_dn0 = assign75420_e115030_d_n0;
        locals.var_dps0_dn2 = assign75420_e115030_d_n2;
        locals.var_dps0_dn4 = assign75420_e115030_d_n4;
        locals.var_dps0_dn5 = assign75420_e115030_d_n5;
        locals.var_dps0_dn6 = assign75420_e115030_d_n6;
        locals.var_dps0_dn7 = assign75420_e115030_d_n7;
        locals.var_dps0_dn8 = assign75420_e115030_d_n8;
        locals.var_dps0_dn9 = assign75420_e115030_d_n9;
        locals.var_dps0_dn10 = assign75420_e115030_d_n10;
        locals.var_dps0_dn11 = assign75420_e115030_d_n11;
        locals.var_dps0_dn14 = assign75420_e115030_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign75430_e115034, assign75430_e115034_d_n0, assign75430_e115034_d_n2, assign75430_e115034_d_n4, assign75430_e115034_d_n5, assign75430_e115034_d_n6, assign75430_e115034_d_n7, assign75430_e115034_d_n8, assign75430_e115034_d_n9, assign75430_e115034_d_n10, assign75430_e115034_d_n11, assign75430_e115034_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign75430_e115034;
        locals.var_fs0_dps0_dn0 = assign75430_e115034_d_n0;
        locals.var_fs0_dps0_dn2 = assign75430_e115034_d_n2;
        locals.var_fs0_dps0_dn4 = assign75430_e115034_d_n4;
        locals.var_fs0_dps0_dn5 = assign75430_e115034_d_n5;
        locals.var_fs0_dps0_dn6 = assign75430_e115034_d_n6;
        locals.var_fs0_dps0_dn7 = assign75430_e115034_d_n7;
        locals.var_fs0_dps0_dn8 = assign75430_e115034_d_n8;
        locals.var_fs0_dps0_dn9 = assign75430_e115034_d_n9;
        locals.var_fs0_dps0_dn10 = assign75430_e115034_d_n10;
        locals.var_fs0_dps0_dn11 = assign75430_e115034_d_n11;
        locals.var_fs0_dps0_dn14 = assign75430_e115034_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_285(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign75440_e115038, assign75440_e115038_d_n0, assign75440_e115038_d_n2, assign75440_e115038_d_n4, assign75440_e115038_d_n5, assign75440_e115038_d_n6, assign75440_e115038_d_n7, assign75440_e115038_d_n8, assign75440_e115038_d_n9, assign75440_e115038_d_n10, assign75440_e115038_d_n11, assign75440_e115038_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign75440_e115038;
        locals.var_fs02_dps0_dn0 = assign75440_e115038_d_n0;
        locals.var_fs02_dps0_dn2 = assign75440_e115038_d_n2;
        locals.var_fs02_dps0_dn4 = assign75440_e115038_d_n4;
        locals.var_fs02_dps0_dn5 = assign75440_e115038_d_n5;
        locals.var_fs02_dps0_dn6 = assign75440_e115038_d_n6;
        locals.var_fs02_dps0_dn7 = assign75440_e115038_d_n7;
        locals.var_fs02_dps0_dn8 = assign75440_e115038_d_n8;
        locals.var_fs02_dps0_dn9 = assign75440_e115038_d_n9;
        locals.var_fs02_dps0_dn10 = assign75440_e115038_d_n10;
        locals.var_fs02_dps0_dn11 = assign75440_e115038_d_n11;
        locals.var_fs02_dps0_dn14 = assign75440_e115038_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign75450_e115042, assign75450_e115042_d_n0, assign75450_e115042_d_n2, assign75450_e115042_d_n4, assign75450_e115042_d_n5, assign75450_e115042_d_n6, assign75450_e115042_d_n7, assign75450_e115042_d_n8, assign75450_e115042_d_n9, assign75450_e115042_d_n10, assign75450_e115042_d_n11, assign75450_e115042_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign75450_e115042;
        locals.var_fb_dpss_dn0 = assign75450_e115042_d_n0;
        locals.var_fb_dpss_dn2 = assign75450_e115042_d_n2;
        locals.var_fb_dpss_dn4 = assign75450_e115042_d_n4;
        locals.var_fb_dpss_dn5 = assign75450_e115042_d_n5;
        locals.var_fb_dpss_dn6 = assign75450_e115042_d_n6;
        locals.var_fb_dpss_dn7 = assign75450_e115042_d_n7;
        locals.var_fb_dpss_dn8 = assign75450_e115042_d_n8;
        locals.var_fb_dpss_dn9 = assign75450_e115042_d_n9;
        locals.var_fb_dpss_dn10 = assign75450_e115042_d_n10;
        locals.var_fb_dpss_dn11 = assign75450_e115042_d_n11;
        locals.var_fb_dpss_dn14 = assign75450_e115042_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign75460_e115046, assign75460_e115046_d_n0, assign75460_e115046_d_n2, assign75460_e115046_d_n4, assign75460_e115046_d_n5, assign75460_e115046_d_n6, assign75460_e115046_d_n7, assign75460_e115046_d_n8, assign75460_e115046_d_n9, assign75460_e115046_d_n10, assign75460_e115046_d_n11, assign75460_e115046_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign75460_e115046;
        locals.var_fs01_dps0_dn0 = assign75460_e115046_d_n0;
        locals.var_fs01_dps0_dn2 = assign75460_e115046_d_n2;
        locals.var_fs01_dps0_dn4 = assign75460_e115046_d_n4;
        locals.var_fs01_dps0_dn5 = assign75460_e115046_d_n5;
        locals.var_fs01_dps0_dn6 = assign75460_e115046_d_n6;
        locals.var_fs01_dps0_dn7 = assign75460_e115046_d_n7;
        locals.var_fs01_dps0_dn8 = assign75460_e115046_d_n8;
        locals.var_fs01_dps0_dn9 = assign75460_e115046_d_n9;
        locals.var_fs01_dps0_dn10 = assign75460_e115046_d_n10;
        locals.var_fs01_dps0_dn11 = assign75460_e115046_d_n11;
        locals.var_fs01_dps0_dn14 = assign75460_e115046_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign75470_e115050, assign75470_e115050_d_n0, assign75470_e115050_d_n2, assign75470_e115050_d_n4, assign75470_e115050_d_n5, assign75470_e115050_d_n6, assign75470_e115050_d_n7, assign75470_e115050_d_n8, assign75470_e115050_d_n9, assign75470_e115050_d_n10, assign75470_e115050_d_n11, assign75470_e115050_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign75470_e115050;
        locals.var_chi_1_dn0 = assign75470_e115050_d_n0;
        locals.var_chi_1_dn2 = assign75470_e115050_d_n2;
        locals.var_chi_1_dn4 = assign75470_e115050_d_n4;
        locals.var_chi_1_dn5 = assign75470_e115050_d_n5;
        locals.var_chi_1_dn6 = assign75470_e115050_d_n6;
        locals.var_chi_1_dn7 = assign75470_e115050_d_n7;
        locals.var_chi_1_dn8 = assign75470_e115050_d_n8;
        locals.var_chi_1_dn9 = assign75470_e115050_d_n9;
        locals.var_chi_1_dn10 = assign75470_e115050_d_n10;
        locals.var_chi_1_dn11 = assign75470_e115050_d_n11;
        locals.var_chi_1_dn14 = assign75470_e115050_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign75480_e115054, assign75480_e115054_d_n0, assign75480_e115054_d_n2, assign75480_e115054_d_n4, assign75480_e115054_d_n5, assign75480_e115054_d_n6, assign75480_e115054_d_n7, assign75480_e115054_d_n8, assign75480_e115054_d_n9, assign75480_e115054_d_n10, assign75480_e115054_d_n11, assign75480_e115054_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign75480_e115054;
        locals.var_chi_a_dn0 = assign75480_e115054_d_n0;
        locals.var_chi_a_dn2 = assign75480_e115054_d_n2;
        locals.var_chi_a_dn4 = assign75480_e115054_d_n4;
        locals.var_chi_a_dn5 = assign75480_e115054_d_n5;
        locals.var_chi_a_dn6 = assign75480_e115054_d_n6;
        locals.var_chi_a_dn7 = assign75480_e115054_d_n7;
        locals.var_chi_a_dn8 = assign75480_e115054_d_n8;
        locals.var_chi_a_dn9 = assign75480_e115054_d_n9;
        locals.var_chi_a_dn10 = assign75480_e115054_d_n10;
        locals.var_chi_a_dn11 = assign75480_e115054_d_n11;
        locals.var_chi_a_dn14 = assign75480_e115054_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign75490_e115058, assign75490_e115058_d_n0, assign75490_e115058_d_n2, assign75490_e115058_d_n4, assign75490_e115058_d_n5, assign75490_e115058_d_n6, assign75490_e115058_d_n7, assign75490_e115058_d_n8, assign75490_e115058_d_n9, assign75490_e115058_d_n10, assign75490_e115058_d_n11, assign75490_e115058_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign75490_e115058;
        locals.var_chi_b_dn0 = assign75490_e115058_d_n0;
        locals.var_chi_b_dn2 = assign75490_e115058_d_n2;
        locals.var_chi_b_dn4 = assign75490_e115058_d_n4;
        locals.var_chi_b_dn5 = assign75490_e115058_d_n5;
        locals.var_chi_b_dn6 = assign75490_e115058_d_n6;
        locals.var_chi_b_dn7 = assign75490_e115058_d_n7;
        locals.var_chi_b_dn8 = assign75490_e115058_d_n8;
        locals.var_chi_b_dn9 = assign75490_e115058_d_n9;
        locals.var_chi_b_dn10 = assign75490_e115058_d_n10;
        locals.var_chi_b_dn11 = assign75490_e115058_d_n11;
        locals.var_chi_b_dn14 = assign75490_e115058_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign75500_e115063,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75500_e115061: f64 = (-1.0);
        (assign75500_e115061,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign75500_e115063;
        locals.var_flg_conv_rv = 0.0;

        let (assign75510_e115067, assign75510_e115067_d_n0, assign75510_e115067_d_n2, assign75510_e115067_d_n4, assign75510_e115067_d_n5, assign75510_e115067_d_n6, assign75510_e115067_d_n7, assign75510_e115067_d_n8, assign75510_e115067_d_n9, assign75510_e115067_d_n10, assign75510_e115067_d_n11, assign75510_e115067_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1773 = assign75510_e115067;
        locals.var_ps0ld_ini__blk1773_dn0 = assign75510_e115067_d_n0;
        locals.var_ps0ld_ini__blk1773_dn2 = assign75510_e115067_d_n2;
        locals.var_ps0ld_ini__blk1773_dn4 = assign75510_e115067_d_n4;
        locals.var_ps0ld_ini__blk1773_dn5 = assign75510_e115067_d_n5;
        locals.var_ps0ld_ini__blk1773_dn6 = assign75510_e115067_d_n6;
        locals.var_ps0ld_ini__blk1773_dn7 = assign75510_e115067_d_n7;
        locals.var_ps0ld_ini__blk1773_dn8 = assign75510_e115067_d_n8;
        locals.var_ps0ld_ini__blk1773_dn9 = assign75510_e115067_d_n9;
        locals.var_ps0ld_ini__blk1773_dn10 = assign75510_e115067_d_n10;
        locals.var_ps0ld_ini__blk1773_dn11 = assign75510_e115067_d_n11;
        locals.var_ps0ld_ini__blk1773_dn14 = assign75510_e115067_d_n14;
        locals.var_ps0ld_ini__blk1773_rv = 0.0;

        let (assign75520_e115071, assign75520_e115071_d_n0, assign75520_e115071_d_n2, assign75520_e115071_d_n4, assign75520_e115071_d_n5, assign75520_e115071_d_n6, assign75520_e115071_d_n7, assign75520_e115071_d_n8, assign75520_e115071_d_n9, assign75520_e115071_d_n10, assign75520_e115071_d_n11, assign75520_e115071_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1774, locals.var_fbsq__blk1774_dn0, locals.var_fbsq__blk1774_dn2, locals.var_fbsq__blk1774_dn4, locals.var_fbsq__blk1774_dn5, locals.var_fbsq__blk1774_dn6, locals.var_fbsq__blk1774_dn7, locals.var_fbsq__blk1774_dn8, locals.var_fbsq__blk1774_dn9, locals.var_fbsq__blk1774_dn10, locals.var_fbsq__blk1774_dn11, locals.var_fbsq__blk1774_dn14,)
    }
};
        locals.var_fbsq__blk1774 = assign75520_e115071;
        locals.var_fbsq__blk1774_dn0 = assign75520_e115071_d_n0;
        locals.var_fbsq__blk1774_dn2 = assign75520_e115071_d_n2;
        locals.var_fbsq__blk1774_dn4 = assign75520_e115071_d_n4;
        locals.var_fbsq__blk1774_dn5 = assign75520_e115071_d_n5;
        locals.var_fbsq__blk1774_dn6 = assign75520_e115071_d_n6;
        locals.var_fbsq__blk1774_dn7 = assign75520_e115071_d_n7;
        locals.var_fbsq__blk1774_dn8 = assign75520_e115071_d_n8;
        locals.var_fbsq__blk1774_dn9 = assign75520_e115071_d_n9;
        locals.var_fbsq__blk1774_dn10 = assign75520_e115071_d_n10;
        locals.var_fbsq__blk1774_dn11 = assign75520_e115071_d_n11;
        locals.var_fbsq__blk1774_dn14 = assign75520_e115071_d_n14;
        locals.var_fbsq__blk1774_rv = 0.0;

        let (assign75530_e115082, assign75530_e115082_d_n0, assign75530_e115082_d_n2, assign75530_e115082_d_n4, assign75530_e115082_d_n5, assign75530_e115082_d_n6, assign75530_e115082_d_n7, assign75530_e115082_d_n8, assign75530_e115082_d_n9, assign75530_e115082_d_n10, assign75530_e115082_d_n11, assign75530_e115082_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75530_e115075: f64 = (2.0 * locals.var_beta_inv);
        let assign75530_e115078: f64 = (locals.var_nover_func / locals.var_nin);
        let assign75530_e115079: f64 = (assign75530_e115078).ln();
        let assign75530_e115080: f64 = (assign75530_e115075 * assign75530_e115079);
        (assign75530_e115080, (((2.0 * locals.var_beta_inv_dn0) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn2) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn4) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn5) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn6) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn7) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn8) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn9) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn10) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn11) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn14) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))),)
    } else {
        (locals.var_pb2over__blk1769, locals.var_pb2over__blk1769_dn0, locals.var_pb2over__blk1769_dn2, locals.var_pb2over__blk1769_dn4, locals.var_pb2over__blk1769_dn5, locals.var_pb2over__blk1769_dn6, locals.var_pb2over__blk1769_dn7, locals.var_pb2over__blk1769_dn8, locals.var_pb2over__blk1769_dn9, locals.var_pb2over__blk1769_dn10, locals.var_pb2over__blk1769_dn11, locals.var_pb2over__blk1769_dn14,)
    }
};
        locals.var_pb2over__blk1769 = assign75530_e115082;
        locals.var_pb2over__blk1769_dn0 = assign75530_e115082_d_n0;
        locals.var_pb2over__blk1769_dn2 = assign75530_e115082_d_n2;
        locals.var_pb2over__blk1769_dn4 = assign75530_e115082_d_n4;
        locals.var_pb2over__blk1769_dn5 = assign75530_e115082_d_n5;
        locals.var_pb2over__blk1769_dn6 = assign75530_e115082_d_n6;
        locals.var_pb2over__blk1769_dn7 = assign75530_e115082_d_n7;
        locals.var_pb2over__blk1769_dn8 = assign75530_e115082_d_n8;
        locals.var_pb2over__blk1769_dn9 = assign75530_e115082_d_n9;
        locals.var_pb2over__blk1769_dn10 = assign75530_e115082_d_n10;
        locals.var_pb2over__blk1769_dn11 = assign75530_e115082_d_n11;
        locals.var_pb2over__blk1769_dn14 = assign75530_e115082_d_n14;
        locals.var_pb2over__blk1769_rv = 0.0;

        let (assign75540_e115090, assign75540_e115090_d_n0, assign75540_e115090_d_n2, assign75540_e115090_d_n4, assign75540_e115090_d_n5, assign75540_e115090_d_n6, assign75540_e115090_d_n7, assign75540_e115090_d_n8, assign75540_e115090_d_n9, assign75540_e115090_d_n10, assign75540_e115090_d_n11, assign75540_e115090_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75540_e115086: f64 = (0.8 - locals.var_pb2over__blk1769);
        let assign75540_e115088: f64 = (assign75540_e115086 - 0.1);
        (assign75540_e115088, (-locals.var_pb2over__blk1769_dn0), (-locals.var_pb2over__blk1769_dn2), (-locals.var_pb2over__blk1769_dn4), (-locals.var_pb2over__blk1769_dn5), (-locals.var_pb2over__blk1769_dn6), (-locals.var_pb2over__blk1769_dn7), (-locals.var_pb2over__blk1769_dn8), (-locals.var_pb2over__blk1769_dn9), (-locals.var_pb2over__blk1769_dn10), (-locals.var_pb2over__blk1769_dn11), (-locals.var_pb2over__blk1769_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75540_e115090;
        locals.var_tmf1_dn0 = assign75540_e115090_d_n0;
        locals.var_tmf1_dn2 = assign75540_e115090_d_n2;
        locals.var_tmf1_dn4 = assign75540_e115090_d_n4;
        locals.var_tmf1_dn5 = assign75540_e115090_d_n5;
        locals.var_tmf1_dn6 = assign75540_e115090_d_n6;
        locals.var_tmf1_dn7 = assign75540_e115090_d_n7;
        locals.var_tmf1_dn8 = assign75540_e115090_d_n8;
        locals.var_tmf1_dn9 = assign75540_e115090_d_n9;
        locals.var_tmf1_dn10 = assign75540_e115090_d_n10;
        locals.var_tmf1_dn11 = assign75540_e115090_d_n11;
        locals.var_tmf1_dn14 = assign75540_e115090_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75550_e115098, assign75550_e115098_d_n0, assign75550_e115098_d_n2, assign75550_e115098_d_n4, assign75550_e115098_d_n5, assign75550_e115098_d_n6, assign75550_e115098_d_n7, assign75550_e115098_d_n8, assign75550_e115098_d_n9, assign75550_e115098_d_n10, assign75550_e115098_d_n11, assign75550_e115098_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75550_e115094: f64 = (4.0 * 0.8);
        let assign75550_e115096: f64 = (assign75550_e115094 * 0.1);
        (assign75550_e115096, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75550_e115098;
        locals.var_tmf2_dn0 = assign75550_e115098_d_n0;
        locals.var_tmf2_dn2 = assign75550_e115098_d_n2;
        locals.var_tmf2_dn4 = assign75550_e115098_d_n4;
        locals.var_tmf2_dn5 = assign75550_e115098_d_n5;
        locals.var_tmf2_dn6 = assign75550_e115098_d_n6;
        locals.var_tmf2_dn7 = assign75550_e115098_d_n7;
        locals.var_tmf2_dn8 = assign75550_e115098_d_n8;
        locals.var_tmf2_dn9 = assign75550_e115098_d_n9;
        locals.var_tmf2_dn10 = assign75550_e115098_d_n10;
        locals.var_tmf2_dn11 = assign75550_e115098_d_n11;
        locals.var_tmf2_dn14 = assign75550_e115098_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75560_e115108, assign75560_e115108_d_n0, assign75560_e115108_d_n2, assign75560_e115108_d_n4, assign75560_e115108_d_n5, assign75560_e115108_d_n6, assign75560_e115108_d_n7, assign75560_e115108_d_n8, assign75560_e115108_d_n9, assign75560_e115108_d_n10, assign75560_e115108_d_n11, assign75560_e115108_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign75560_e115106, assign75560_e115106_d_n0, assign75560_e115106_d_n2, assign75560_e115106_d_n4, assign75560_e115106_d_n5, assign75560_e115106_d_n6, assign75560_e115106_d_n7, assign75560_e115106_d_n8, assign75560_e115106_d_n9, assign75560_e115106_d_n10, assign75560_e115106_d_n11, assign75560_e115106_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign75560_e115105: f64 = (-locals.var_tmf2);
                (assign75560_e115105, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign75560_e115106, assign75560_e115106_d_n0, assign75560_e115106_d_n2, assign75560_e115106_d_n4, assign75560_e115106_d_n5, assign75560_e115106_d_n6, assign75560_e115106_d_n7, assign75560_e115106_d_n8, assign75560_e115106_d_n9, assign75560_e115106_d_n10, assign75560_e115106_d_n11, assign75560_e115106_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75560_e115108;
        locals.var_tmf2_dn0 = assign75560_e115108_d_n0;
        locals.var_tmf2_dn2 = assign75560_e115108_d_n2;
        locals.var_tmf2_dn4 = assign75560_e115108_d_n4;
        locals.var_tmf2_dn5 = assign75560_e115108_d_n5;
        locals.var_tmf2_dn6 = assign75560_e115108_d_n6;
        locals.var_tmf2_dn7 = assign75560_e115108_d_n7;
        locals.var_tmf2_dn8 = assign75560_e115108_d_n8;
        locals.var_tmf2_dn9 = assign75560_e115108_d_n9;
        locals.var_tmf2_dn10 = assign75560_e115108_d_n10;
        locals.var_tmf2_dn11 = assign75560_e115108_d_n11;
        locals.var_tmf2_dn14 = assign75560_e115108_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75570_e115117, assign75570_e115117_d_n0, assign75570_e115117_d_n2, assign75570_e115117_d_n4, assign75570_e115117_d_n5, assign75570_e115117_d_n6, assign75570_e115117_d_n7, assign75570_e115117_d_n8, assign75570_e115117_d_n9, assign75570_e115117_d_n10, assign75570_e115117_d_n11, assign75570_e115117_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75570_e115112: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign75570_e115114: f64 = (assign75570_e115112 + locals.var_tmf2);
        let assign75570_e115115: f64 = (assign75570_e115114).sqrt();
        (assign75570_e115115, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign75570_e115115)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75570_e115117;
        locals.var_tmf2_dn0 = assign75570_e115117_d_n0;
        locals.var_tmf2_dn2 = assign75570_e115117_d_n2;
        locals.var_tmf2_dn4 = assign75570_e115117_d_n4;
        locals.var_tmf2_dn5 = assign75570_e115117_d_n5;
        locals.var_tmf2_dn6 = assign75570_e115117_d_n6;
        locals.var_tmf2_dn7 = assign75570_e115117_d_n7;
        locals.var_tmf2_dn8 = assign75570_e115117_d_n8;
        locals.var_tmf2_dn9 = assign75570_e115117_d_n9;
        locals.var_tmf2_dn10 = assign75570_e115117_d_n10;
        locals.var_tmf2_dn11 = assign75570_e115117_d_n11;
        locals.var_tmf2_dn14 = assign75570_e115117_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75580_e115127, assign75580_e115127_d_n0, assign75580_e115127_d_n2, assign75580_e115127_d_n4, assign75580_e115127_d_n5, assign75580_e115127_d_n6, assign75580_e115127_d_n7, assign75580_e115127_d_n8, assign75580_e115127_d_n9, assign75580_e115127_d_n10, assign75580_e115127_d_n11, assign75580_e115127_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75580_e115123: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign75580_e115124: f64 = (1.0 + assign75580_e115123);
        let assign75580_e115125: f64 = (0.5 * assign75580_e115124);
        (assign75580_e115125, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75580_e115127;
        locals.var_t0_dn0 = assign75580_e115127_d_n0;
        locals.var_t0_dn2 = assign75580_e115127_d_n2;
        locals.var_t0_dn4 = assign75580_e115127_d_n4;
        locals.var_t0_dn5 = assign75580_e115127_d_n5;
        locals.var_t0_dn6 = assign75580_e115127_d_n6;
        locals.var_t0_dn7 = assign75580_e115127_d_n7;
        locals.var_t0_dn8 = assign75580_e115127_d_n8;
        locals.var_t0_dn9 = assign75580_e115127_d_n9;
        locals.var_t0_dn10 = assign75580_e115127_d_n10;
        locals.var_t0_dn11 = assign75580_e115127_d_n11;
        locals.var_t0_dn14 = assign75580_e115127_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75590_e115137, assign75590_e115137_d_n0, assign75590_e115137_d_n2, assign75590_e115137_d_n4, assign75590_e115137_d_n5, assign75590_e115137_d_n6, assign75590_e115137_d_n7, assign75590_e115137_d_n8, assign75590_e115137_d_n9, assign75590_e115137_d_n10, assign75590_e115137_d_n11, assign75590_e115137_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75590_e115133: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign75590_e115134: f64 = (0.5 * assign75590_e115133);
        let assign75590_e115135: f64 = (0.8 - assign75590_e115134);
        (assign75590_e115135, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk1770, locals.var_vbs_max_over__blk1770_dn0, locals.var_vbs_max_over__blk1770_dn2, locals.var_vbs_max_over__blk1770_dn4, locals.var_vbs_max_over__blk1770_dn5, locals.var_vbs_max_over__blk1770_dn6, locals.var_vbs_max_over__blk1770_dn7, locals.var_vbs_max_over__blk1770_dn8, locals.var_vbs_max_over__blk1770_dn9, locals.var_vbs_max_over__blk1770_dn10, locals.var_vbs_max_over__blk1770_dn11, locals.var_vbs_max_over__blk1770_dn14,)
    }
};
        locals.var_vbs_max_over__blk1770 = assign75590_e115137;
        locals.var_vbs_max_over__blk1770_dn0 = assign75590_e115137_d_n0;
        locals.var_vbs_max_over__blk1770_dn2 = assign75590_e115137_d_n2;
        locals.var_vbs_max_over__blk1770_dn4 = assign75590_e115137_d_n4;
        locals.var_vbs_max_over__blk1770_dn5 = assign75590_e115137_d_n5;
        locals.var_vbs_max_over__blk1770_dn6 = assign75590_e115137_d_n6;
        locals.var_vbs_max_over__blk1770_dn7 = assign75590_e115137_d_n7;
        locals.var_vbs_max_over__blk1770_dn8 = assign75590_e115137_d_n8;
        locals.var_vbs_max_over__blk1770_dn9 = assign75590_e115137_d_n9;
        locals.var_vbs_max_over__blk1770_dn10 = assign75590_e115137_d_n10;
        locals.var_vbs_max_over__blk1770_dn11 = assign75590_e115137_d_n11;
        locals.var_vbs_max_over__blk1770_dn14 = assign75590_e115137_d_n14;
        locals.var_vbs_max_over__blk1770_rv = 0.0;

        let assign75600_e115141: f64 = (locals.var_vbs_max_over__blk1770 * 0.5);
        let assign75600_e115142: f64 = if locals.var_vbs_bnd_over__blk1771 > assign75600_e115141 { 1.0 } else { 0.0 };
        locals.var_guard1776 = assign75600_e115142;
        locals.var_guard1776_rv = 0.0;

        let (assign75610_e115150, assign75610_e115150_d_n0, assign75610_e115150_d_n2, assign75610_e115150_d_n4, assign75610_e115150_d_n5, assign75610_e115150_d_n6, assign75610_e115150_d_n7, assign75610_e115150_d_n8, assign75610_e115150_d_n9, assign75610_e115150_d_n10, assign75610_e115150_d_n11, assign75610_e115150_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1776 != 0.0)) {
        let assign75610_e115148: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75610_e115148, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75610_e115150;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75610_e115150_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75610_e115150_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75610_e115150_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75610_e115150_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75610_e115150_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75610_e115150_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75610_e115150_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75610_e115150_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75610_e115150_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75610_e115150_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75610_e115150_d_n14;
        locals.var_vbs_bnd_over__blk1771_rv = 0.0;

        let assign75620_e115152: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1777 = assign75620_e115152;
        locals.var_guard1777_rv = 0.0;

        let (assign75630_e115158, assign75630_e115158_d_n0, assign75630_e115158_d_n2, assign75630_e115158_d_n4, assign75630_e115158_d_n5, assign75630_e115158_d_n6, assign75630_e115158_d_n7, assign75630_e115158_d_n8, assign75630_e115158_d_n9, assign75630_e115158_d_n10, assign75630_e115158_d_n11, assign75630_e115158_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1770, locals.var_vbs_max_over__blk1770_dn0, locals.var_vbs_max_over__blk1770_dn2, locals.var_vbs_max_over__blk1770_dn4, locals.var_vbs_max_over__blk1770_dn5, locals.var_vbs_max_over__blk1770_dn6, locals.var_vbs_max_over__blk1770_dn7, locals.var_vbs_max_over__blk1770_dn8, locals.var_vbs_max_over__blk1770_dn9, locals.var_vbs_max_over__blk1770_dn10, locals.var_vbs_max_over__blk1770_dn11, locals.var_vbs_max_over__blk1770_dn14,)
    }
};
        locals.var_vbs_max_over__blk1770 = assign75630_e115158;
        locals.var_vbs_max_over__blk1770_dn0 = assign75630_e115158_d_n0;
        locals.var_vbs_max_over__blk1770_dn2 = assign75630_e115158_d_n2;
        locals.var_vbs_max_over__blk1770_dn4 = assign75630_e115158_d_n4;
        locals.var_vbs_max_over__blk1770_dn5 = assign75630_e115158_d_n5;
        locals.var_vbs_max_over__blk1770_dn6 = assign75630_e115158_d_n6;
        locals.var_vbs_max_over__blk1770_dn7 = assign75630_e115158_d_n7;
        locals.var_vbs_max_over__blk1770_dn8 = assign75630_e115158_d_n8;
        locals.var_vbs_max_over__blk1770_dn9 = assign75630_e115158_d_n9;
        locals.var_vbs_max_over__blk1770_dn10 = assign75630_e115158_d_n10;
        locals.var_vbs_max_over__blk1770_dn11 = assign75630_e115158_d_n11;
        locals.var_vbs_max_over__blk1770_dn14 = assign75630_e115158_d_n14;
        locals.var_vbs_max_over__blk1770_rv = 0.0;

        let assign75640_e115160: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1778 = assign75640_e115160;
        locals.var_guard1778_rv = 0.0;

        let (assign75650_e115166, assign75650_e115166_d_n0, assign75650_e115166_d_n2, assign75650_e115166_d_n4, assign75650_e115166_d_n5, assign75650_e115166_d_n6, assign75650_e115166_d_n7, assign75650_e115166_d_n8, assign75650_e115166_d_n9, assign75650_e115166_d_n10, assign75650_e115166_d_n11, assign75650_e115166_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1778 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75650_e115166;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75650_e115166_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75650_e115166_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75650_e115166_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75650_e115166_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75650_e115166_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75650_e115166_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75650_e115166_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75650_e115166_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75650_e115166_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75650_e115166_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75650_e115166_d_n14;
        locals.var_vbs_bnd_over__blk1771_rv = 0.0;

        let assign75660_e115168: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1779 = assign75660_e115168;
        locals.var_guard1779_rv = 0.0;

        let (assign75670_e115179, assign75670_e115179_d_n0, assign75670_e115179_d_n2, assign75670_e115179_d_n4, assign75670_e115179_d_n5, assign75670_e115179_d_n6, assign75670_e115179_d_n7, assign75670_e115179_d_n8, assign75670_e115179_d_n9, assign75670_e115179_d_n10, assign75670_e115179_d_n11, assign75670_e115179_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1778 == 0.0)) && (locals.var_guard1779 != 0.0)) {
        let assign75670_e115177: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75670_e115177, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75670_e115179;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75670_e115179_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75670_e115179_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75670_e115179_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75670_e115179_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75670_e115179_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75670_e115179_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75670_e115179_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75670_e115179_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75670_e115179_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75670_e115179_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75670_e115179_d_n14;
        locals.var_vbs_bnd_over__blk1771_rv = 0.0;

        let assign75680_e115183: f64 = (locals.var_vbs_max_over__blk1770 * 0.5);
        let assign75680_e115184: f64 = if locals.var_vbs_bnd_over__blk1771 > assign75680_e115183 { 1.0 } else { 0.0 };
        locals.var_guard1780 = assign75680_e115184;
        locals.var_guard1780_rv = 0.0;

        let (assign75690_e115192, assign75690_e115192_d_n0, assign75690_e115192_d_n2, assign75690_e115192_d_n4, assign75690_e115192_d_n5, assign75690_e115192_d_n6, assign75690_e115192_d_n7, assign75690_e115192_d_n8, assign75690_e115192_d_n9, assign75690_e115192_d_n10, assign75690_e115192_d_n11, assign75690_e115192_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1780 != 0.0)) {
        let assign75690_e115190: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75690_e115190, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75690_e115192;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75690_e115192_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75690_e115192_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75690_e115192_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75690_e115192_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75690_e115192_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75690_e115192_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75690_e115192_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75690_e115192_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75690_e115192_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75690_e115192_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75690_e115192_d_n14;
        locals.var_vbs_bnd_over__blk1771_rv = 0.0;

        let assign75700_e115195: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1781 = assign75700_e115195;
        locals.var_guard1781_rv = 0.0;

        let (assign75710_e115202, assign75710_e115202_d_n0, assign75710_e115202_d_n2, assign75710_e115202_d_n4, assign75710_e115202_d_n5, assign75710_e115202_d_n6, assign75710_e115202_d_n7, assign75710_e115202_d_n8, assign75710_e115202_d_n9, assign75710_e115202_d_n10, assign75710_e115202_d_n11, assign75710_e115202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        let assign75710_e115200: f64 = (-locals.var_vxbgmt);
        (assign75710_e115200, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75710_e115202;
        locals.var_t0_dn0 = assign75710_e115202_d_n0;
        locals.var_t0_dn2 = assign75710_e115202_d_n2;
        locals.var_t0_dn4 = assign75710_e115202_d_n4;
        locals.var_t0_dn5 = assign75710_e115202_d_n5;
        locals.var_t0_dn6 = assign75710_e115202_d_n6;
        locals.var_t0_dn7 = assign75710_e115202_d_n7;
        locals.var_t0_dn8 = assign75710_e115202_d_n8;
        locals.var_t0_dn9 = assign75710_e115202_d_n9;
        locals.var_t0_dn10 = assign75710_e115202_d_n10;
        locals.var_t0_dn11 = assign75710_e115202_d_n11;
        locals.var_t0_dn14 = assign75710_e115202_d_n14;
        locals.var_t0_rv = 0.0;

        let assign75720_e115205: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1771 { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign75720_e115205;
        locals.var_guard1782_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_286(
        locals: &mut StampLocals,
    ) {
        let (assign75730_e115215, assign75730_e115215_d_n0, assign75730_e115215_d_n2, assign75730_e115215_d_n4, assign75730_e115215_d_n5, assign75730_e115215_d_n6, assign75730_e115215_d_n7, assign75730_e115215_d_n8, assign75730_e115215_d_n9, assign75730_e115215_d_n10, assign75730_e115215_d_n11, assign75730_e115215_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75730_e115213: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1771);
        (assign75730_e115213, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1771_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1771_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1771_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1771_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1771_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1771_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1771_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1771_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1771_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk1771_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk1771_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign75730_e115215;
        locals.var_t1_dn0 = assign75730_e115215_d_n0;
        locals.var_t1_dn2 = assign75730_e115215_d_n2;
        locals.var_t1_dn4 = assign75730_e115215_d_n4;
        locals.var_t1_dn5 = assign75730_e115215_d_n5;
        locals.var_t1_dn6 = assign75730_e115215_d_n6;
        locals.var_t1_dn7 = assign75730_e115215_d_n7;
        locals.var_t1_dn8 = assign75730_e115215_d_n8;
        locals.var_t1_dn9 = assign75730_e115215_d_n9;
        locals.var_t1_dn10 = assign75730_e115215_d_n10;
        locals.var_t1_dn11 = assign75730_e115215_d_n11;
        locals.var_t1_dn14 = assign75730_e115215_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign75740_e115225, assign75740_e115225_d_n0, assign75740_e115225_d_n2, assign75740_e115225_d_n4, assign75740_e115225_d_n5, assign75740_e115225_d_n6, assign75740_e115225_d_n7, assign75740_e115225_d_n8, assign75740_e115225_d_n9, assign75740_e115225_d_n10, assign75740_e115225_d_n11, assign75740_e115225_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75740_e115223: f64 = (locals.var_vbs_max_over__blk1770 - locals.var_vbs_bnd_over__blk1771);
        (assign75740_e115223, (locals.var_vbs_max_over__blk1770_dn0 - locals.var_vbs_bnd_over__blk1771_dn0), (locals.var_vbs_max_over__blk1770_dn2 - locals.var_vbs_bnd_over__blk1771_dn2), (locals.var_vbs_max_over__blk1770_dn4 - locals.var_vbs_bnd_over__blk1771_dn4), (locals.var_vbs_max_over__blk1770_dn5 - locals.var_vbs_bnd_over__blk1771_dn5), (locals.var_vbs_max_over__blk1770_dn6 - locals.var_vbs_bnd_over__blk1771_dn6), (locals.var_vbs_max_over__blk1770_dn7 - locals.var_vbs_bnd_over__blk1771_dn7), (locals.var_vbs_max_over__blk1770_dn8 - locals.var_vbs_bnd_over__blk1771_dn8), (locals.var_vbs_max_over__blk1770_dn9 - locals.var_vbs_bnd_over__blk1771_dn9), (locals.var_vbs_max_over__blk1770_dn10 - locals.var_vbs_bnd_over__blk1771_dn10), (locals.var_vbs_max_over__blk1770_dn11 - locals.var_vbs_bnd_over__blk1771_dn11), (locals.var_vbs_max_over__blk1770_dn14 - locals.var_vbs_bnd_over__blk1771_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75740_e115225;
        locals.var_t2_dn0 = assign75740_e115225_d_n0;
        locals.var_t2_dn2 = assign75740_e115225_d_n2;
        locals.var_t2_dn4 = assign75740_e115225_d_n4;
        locals.var_t2_dn5 = assign75740_e115225_d_n5;
        locals.var_t2_dn6 = assign75740_e115225_d_n6;
        locals.var_t2_dn7 = assign75740_e115225_d_n7;
        locals.var_t2_dn8 = assign75740_e115225_d_n8;
        locals.var_t2_dn9 = assign75740_e115225_d_n9;
        locals.var_t2_dn10 = assign75740_e115225_d_n10;
        locals.var_t2_dn11 = assign75740_e115225_d_n11;
        locals.var_t2_dn14 = assign75740_e115225_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign75750_e115235, assign75750_e115235_d_n0, assign75750_e115235_d_n2, assign75750_e115235_d_n4, assign75750_e115235_d_n5, assign75750_e115235_d_n6, assign75750_e115235_d_n7, assign75750_e115235_d_n8, assign75750_e115235_d_n9, assign75750_e115235_d_n10, assign75750_e115235_d_n11, assign75750_e115235_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75750_e115233: f64 = (locals.var_t1 / locals.var_t2);
        (assign75750_e115233, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75750_e115235;
        locals.var_tmf1_dn0 = assign75750_e115235_d_n0;
        locals.var_tmf1_dn2 = assign75750_e115235_d_n2;
        locals.var_tmf1_dn4 = assign75750_e115235_d_n4;
        locals.var_tmf1_dn5 = assign75750_e115235_d_n5;
        locals.var_tmf1_dn6 = assign75750_e115235_d_n6;
        locals.var_tmf1_dn7 = assign75750_e115235_d_n7;
        locals.var_tmf1_dn8 = assign75750_e115235_d_n8;
        locals.var_tmf1_dn9 = assign75750_e115235_d_n9;
        locals.var_tmf1_dn10 = assign75750_e115235_d_n10;
        locals.var_tmf1_dn11 = assign75750_e115235_d_n11;
        locals.var_tmf1_dn14 = assign75750_e115235_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75760_e115245, assign75760_e115245_d_n0, assign75760_e115245_d_n2, assign75760_e115245_d_n4, assign75760_e115245_d_n5, assign75760_e115245_d_n6, assign75760_e115245_d_n7, assign75760_e115245_d_n8, assign75760_e115245_d_n9, assign75760_e115245_d_n10, assign75760_e115245_d_n11, assign75760_e115245_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75760_e115243: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign75760_e115243, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75760_e115245;
        locals.var_tmf2_dn0 = assign75760_e115245_d_n0;
        locals.var_tmf2_dn2 = assign75760_e115245_d_n2;
        locals.var_tmf2_dn4 = assign75760_e115245_d_n4;
        locals.var_tmf2_dn5 = assign75760_e115245_d_n5;
        locals.var_tmf2_dn6 = assign75760_e115245_d_n6;
        locals.var_tmf2_dn7 = assign75760_e115245_d_n7;
        locals.var_tmf2_dn8 = assign75760_e115245_d_n8;
        locals.var_tmf2_dn9 = assign75760_e115245_d_n9;
        locals.var_tmf2_dn10 = assign75760_e115245_d_n10;
        locals.var_tmf2_dn11 = assign75760_e115245_d_n11;
        locals.var_tmf2_dn14 = assign75760_e115245_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75770_e115255, assign75770_e115255_d_n0, assign75770_e115255_d_n2, assign75770_e115255_d_n4, assign75770_e115255_d_n5, assign75770_e115255_d_n6, assign75770_e115255_d_n7, assign75770_e115255_d_n8, assign75770_e115255_d_n9, assign75770_e115255_d_n10, assign75770_e115255_d_n11, assign75770_e115255_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75770_e115253: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign75770_e115253, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign75770_e115255;
        locals.var_tmf3_dn0 = assign75770_e115255_d_n0;
        locals.var_tmf3_dn2 = assign75770_e115255_d_n2;
        locals.var_tmf3_dn4 = assign75770_e115255_d_n4;
        locals.var_tmf3_dn5 = assign75770_e115255_d_n5;
        locals.var_tmf3_dn6 = assign75770_e115255_d_n6;
        locals.var_tmf3_dn7 = assign75770_e115255_d_n7;
        locals.var_tmf3_dn8 = assign75770_e115255_d_n8;
        locals.var_tmf3_dn9 = assign75770_e115255_d_n9;
        locals.var_tmf3_dn10 = assign75770_e115255_d_n10;
        locals.var_tmf3_dn11 = assign75770_e115255_d_n11;
        locals.var_tmf3_dn14 = assign75770_e115255_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign75780_e115265, assign75780_e115265_d_n0, assign75780_e115265_d_n2, assign75780_e115265_d_n4, assign75780_e115265_d_n5, assign75780_e115265_d_n6, assign75780_e115265_d_n7, assign75780_e115265_d_n8, assign75780_e115265_d_n9, assign75780_e115265_d_n10, assign75780_e115265_d_n11, assign75780_e115265_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75780_e115263: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign75780_e115263, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign75780_e115265;
        locals.var_tmf4_dn0 = assign75780_e115265_d_n0;
        locals.var_tmf4_dn2 = assign75780_e115265_d_n2;
        locals.var_tmf4_dn4 = assign75780_e115265_d_n4;
        locals.var_tmf4_dn5 = assign75780_e115265_d_n5;
        locals.var_tmf4_dn6 = assign75780_e115265_d_n6;
        locals.var_tmf4_dn7 = assign75780_e115265_d_n7;
        locals.var_tmf4_dn8 = assign75780_e115265_d_n8;
        locals.var_tmf4_dn9 = assign75780_e115265_d_n9;
        locals.var_tmf4_dn10 = assign75780_e115265_d_n10;
        locals.var_tmf4_dn11 = assign75780_e115265_d_n11;
        locals.var_tmf4_dn14 = assign75780_e115265_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign75790_e115283, assign75790_e115283_d_n0, assign75790_e115283_d_n2, assign75790_e115283_d_n4, assign75790_e115283_d_n5, assign75790_e115283_d_n6, assign75790_e115283_d_n7, assign75790_e115283_d_n8, assign75790_e115283_d_n9, assign75790_e115283_d_n10, assign75790_e115283_d_n11, assign75790_e115283_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75790_e115274: f64 = (1.0 + locals.var_tmf1);
        let assign75790_e115276: f64 = (assign75790_e115274 + locals.var_tmf2);
        let assign75790_e115278: f64 = (assign75790_e115276 + locals.var_tmf3);
        let assign75790_e115280: f64 = (assign75790_e115278 + locals.var_tmf4);
        let assign75790_e115281: f64 = (1.0 / assign75790_e115280);
        (assign75790_e115281, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign75790_e115280 * assign75790_e115280))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign75790_e115283;
        locals.var_tmf0_dn0 = assign75790_e115283_d_n0;
        locals.var_tmf0_dn2 = assign75790_e115283_d_n2;
        locals.var_tmf0_dn4 = assign75790_e115283_d_n4;
        locals.var_tmf0_dn5 = assign75790_e115283_d_n5;
        locals.var_tmf0_dn6 = assign75790_e115283_d_n6;
        locals.var_tmf0_dn7 = assign75790_e115283_d_n7;
        locals.var_tmf0_dn8 = assign75790_e115283_d_n8;
        locals.var_tmf0_dn9 = assign75790_e115283_d_n9;
        locals.var_tmf0_dn10 = assign75790_e115283_d_n10;
        locals.var_tmf0_dn11 = assign75790_e115283_d_n11;
        locals.var_tmf0_dn14 = assign75790_e115283_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign75800_e115308, assign75800_e115308_d_n0, assign75800_e115308_d_n2, assign75800_e115308_d_n4, assign75800_e115308_d_n5, assign75800_e115308_d_n6, assign75800_e115308_d_n7, assign75800_e115308_d_n8, assign75800_e115308_d_n9, assign75800_e115308_d_n10, assign75800_e115308_d_n11, assign75800_e115308_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75800_e115292: f64 = (2.0 * locals.var_tmf1);
        let assign75800_e115293: f64 = (1.0 + assign75800_e115292);
        let assign75800_e115296: f64 = (3.0 * locals.var_tmf2);
        let assign75800_e115297: f64 = (assign75800_e115293 + assign75800_e115296);
        let assign75800_e115300: f64 = (4.0 * locals.var_tmf3);
        let assign75800_e115301: f64 = (assign75800_e115297 + assign75800_e115300);
        let assign75800_e115302: f64 = (-assign75800_e115301);
        let assign75800_e115304: f64 = (assign75800_e115302 * locals.var_tmf0);
        let assign75800_e115306: f64 = (assign75800_e115304 * locals.var_tmf0);
        (assign75800_e115306, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75800_e115308;
        locals.var_t11_dn0 = assign75800_e115308_d_n0;
        locals.var_t11_dn2 = assign75800_e115308_d_n2;
        locals.var_t11_dn4 = assign75800_e115308_d_n4;
        locals.var_t11_dn5 = assign75800_e115308_d_n5;
        locals.var_t11_dn6 = assign75800_e115308_d_n6;
        locals.var_t11_dn7 = assign75800_e115308_d_n7;
        locals.var_t11_dn8 = assign75800_e115308_d_n8;
        locals.var_t11_dn9 = assign75800_e115308_d_n9;
        locals.var_t11_dn10 = assign75800_e115308_d_n10;
        locals.var_t11_dn11 = assign75800_e115308_d_n11;
        locals.var_t11_dn14 = assign75800_e115308_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign75810_e115320, assign75810_e115320_d_n0, assign75810_e115320_d_n2, assign75810_e115320_d_n4, assign75810_e115320_d_n5, assign75810_e115320_d_n6, assign75810_e115320_d_n7, assign75810_e115320_d_n8, assign75810_e115320_d_n9, assign75810_e115320_d_n10, assign75810_e115320_d_n11, assign75810_e115320_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75810_e115317: f64 = (1.0 - locals.var_tmf0);
        let assign75810_e115318: f64 = (locals.var_t2 * assign75810_e115317);
        (assign75810_e115318, ((locals.var_t2_dn0 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign75810_e115320;
        locals.var_ty_dn0 = assign75810_e115320_d_n0;
        locals.var_ty_dn2 = assign75810_e115320_d_n2;
        locals.var_ty_dn4 = assign75810_e115320_d_n4;
        locals.var_ty_dn5 = assign75810_e115320_d_n5;
        locals.var_ty_dn6 = assign75810_e115320_d_n6;
        locals.var_ty_dn7 = assign75810_e115320_d_n7;
        locals.var_ty_dn8 = assign75810_e115320_d_n8;
        locals.var_ty_dn9 = assign75810_e115320_d_n9;
        locals.var_ty_dn10 = assign75810_e115320_d_n10;
        locals.var_ty_dn11 = assign75810_e115320_d_n11;
        locals.var_ty_dn14 = assign75810_e115320_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign75820_e115334, assign75820_e115334_d_n0, assign75820_e115334_d_n2, assign75820_e115334_d_n4, assign75820_e115334_d_n5, assign75820_e115334_d_n6, assign75820_e115334_d_n7, assign75820_e115334_d_n8, assign75820_e115334_d_n9, assign75820_e115334_d_n10, assign75820_e115334_d_n11, assign75820_e115334_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75820_e115328: f64 = (1.0 - locals.var_tmf0);
        let assign75820_e115331: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign75820_e115332: f64 = (assign75820_e115328 + assign75820_e115331);
        (assign75820_e115332, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75820_e115334;
        locals.var_t0_dn0 = assign75820_e115334_d_n0;
        locals.var_t0_dn2 = assign75820_e115334_d_n2;
        locals.var_t0_dn4 = assign75820_e115334_d_n4;
        locals.var_t0_dn5 = assign75820_e115334_d_n5;
        locals.var_t0_dn6 = assign75820_e115334_d_n6;
        locals.var_t0_dn7 = assign75820_e115334_d_n7;
        locals.var_t0_dn8 = assign75820_e115334_d_n8;
        locals.var_t0_dn9 = assign75820_e115334_d_n9;
        locals.var_t0_dn10 = assign75820_e115334_d_n10;
        locals.var_t0_dn11 = assign75820_e115334_d_n11;
        locals.var_t0_dn14 = assign75820_e115334_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75830_e115343, assign75830_e115343_d_n0, assign75830_e115343_d_n2, assign75830_e115343_d_n4, assign75830_e115343_d_n5, assign75830_e115343_d_n6, assign75830_e115343_d_n7, assign75830_e115343_d_n8, assign75830_e115343_d_n9, assign75830_e115343_d_n10, assign75830_e115343_d_n11, assign75830_e115343_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75830_e115341: f64 = (-locals.var_t11);
        (assign75830_e115341, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75830_e115343;
        locals.var_t11_dn0 = assign75830_e115343_d_n0;
        locals.var_t11_dn2 = assign75830_e115343_d_n2;
        locals.var_t11_dn4 = assign75830_e115343_d_n4;
        locals.var_t11_dn5 = assign75830_e115343_d_n5;
        locals.var_t11_dn6 = assign75830_e115343_d_n6;
        locals.var_t11_dn7 = assign75830_e115343_d_n7;
        locals.var_t11_dn8 = assign75830_e115343_d_n8;
        locals.var_t11_dn9 = assign75830_e115343_d_n9;
        locals.var_t11_dn10 = assign75830_e115343_d_n10;
        locals.var_t11_dn11 = assign75830_e115343_d_n11;
        locals.var_t11_dn14 = assign75830_e115343_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign75840_e115353, assign75840_e115353_d_n0, assign75840_e115353_d_n2, assign75840_e115353_d_n4, assign75840_e115353_d_n5, assign75840_e115353_d_n6, assign75840_e115353_d_n7, assign75840_e115353_d_n8, assign75840_e115353_d_n9, assign75840_e115353_d_n10, assign75840_e115353_d_n11, assign75840_e115353_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75840_e115351: f64 = (locals.var_vbs_bnd_over__blk1771 + locals.var_ty);
        (assign75840_e115351, (locals.var_vbs_bnd_over__blk1771_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1771_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1771_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1771_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1771_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1771_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1771_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1771_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1771_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1771_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk1771_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75840_e115353;
        locals.var_t10_dn0 = assign75840_e115353_d_n0;
        locals.var_t10_dn2 = assign75840_e115353_d_n2;
        locals.var_t10_dn4 = assign75840_e115353_d_n4;
        locals.var_t10_dn5 = assign75840_e115353_d_n5;
        locals.var_t10_dn6 = assign75840_e115353_d_n6;
        locals.var_t10_dn7 = assign75840_e115353_d_n7;
        locals.var_t10_dn8 = assign75840_e115353_d_n8;
        locals.var_t10_dn9 = assign75840_e115353_d_n9;
        locals.var_t10_dn10 = assign75840_e115353_d_n10;
        locals.var_t10_dn11 = assign75840_e115353_d_n11;
        locals.var_t10_dn14 = assign75840_e115353_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign75850_e115362, assign75850_e115362_d_n0, assign75850_e115362_d_n2, assign75850_e115362_d_n4, assign75850_e115362_d_n5, assign75850_e115362_d_n6, assign75850_e115362_d_n7, assign75850_e115362_d_n8, assign75850_e115362_d_n9, assign75850_e115362_d_n10, assign75850_e115362_d_n11, assign75850_e115362_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75850_e115362;
        locals.var_t10_dn0 = assign75850_e115362_d_n0;
        locals.var_t10_dn2 = assign75850_e115362_d_n2;
        locals.var_t10_dn4 = assign75850_e115362_d_n4;
        locals.var_t10_dn5 = assign75850_e115362_d_n5;
        locals.var_t10_dn6 = assign75850_e115362_d_n6;
        locals.var_t10_dn7 = assign75850_e115362_d_n7;
        locals.var_t10_dn8 = assign75850_e115362_d_n8;
        locals.var_t10_dn9 = assign75850_e115362_d_n9;
        locals.var_t10_dn10 = assign75850_e115362_d_n10;
        locals.var_t10_dn11 = assign75850_e115362_d_n11;
        locals.var_t10_dn14 = assign75850_e115362_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign75860_e115369, assign75860_e115369_d_n0, assign75860_e115369_d_n2, assign75860_e115369_d_n4, assign75860_e115369_d_n5, assign75860_e115369_d_n6, assign75860_e115369_d_n7, assign75860_e115369_d_n8, assign75860_e115369_d_n9, assign75860_e115369_d_n10, assign75860_e115369_d_n11, assign75860_e115369_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        let assign75860_e115367: f64 = (-locals.var_t10);
        (assign75860_e115367, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75860_e115369;
        locals.var_vxbgmtcl_dn0 = assign75860_e115369_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75860_e115369_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75860_e115369_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75860_e115369_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75860_e115369_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75860_e115369_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75860_e115369_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75860_e115369_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75860_e115369_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75860_e115369_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75860_e115369_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75870_e115376, assign75870_e115376_d_n0, assign75870_e115376_d_n2, assign75870_e115376_d_n4, assign75870_e115376_d_n5, assign75870_e115376_d_n6, assign75870_e115376_d_n7, assign75870_e115376_d_n8, assign75870_e115376_d_n9, assign75870_e115376_d_n10, assign75870_e115376_d_n11, assign75870_e115376_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75870_e115376;
        locals.var_vxbgmtcl_dn0 = assign75870_e115376_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75870_e115376_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75870_e115376_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75870_e115376_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75870_e115376_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75870_e115376_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75870_e115376_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75870_e115376_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75870_e115376_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75870_e115376_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75870_e115376_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75880_e115382, assign75880_e115382_d_n0, assign75880_e115382_d_n2, assign75880_e115382_d_n4, assign75880_e115382_d_n5, assign75880_e115382_d_n6, assign75880_e115382_d_n7, assign75880_e115382_d_n8, assign75880_e115382_d_n9, assign75880_e115382_d_n10, assign75880_e115382_d_n11, assign75880_e115382_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75880_e115380: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign75880_e115380, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign75880_e115382;
        locals.var_fac1_dn0 = assign75880_e115382_d_n0;
        locals.var_fac1_dn2 = assign75880_e115382_d_n2;
        locals.var_fac1_dn4 = assign75880_e115382_d_n4;
        locals.var_fac1_dn5 = assign75880_e115382_d_n5;
        locals.var_fac1_dn6 = assign75880_e115382_d_n6;
        locals.var_fac1_dn7 = assign75880_e115382_d_n7;
        locals.var_fac1_dn8 = assign75880_e115382_d_n8;
        locals.var_fac1_dn9 = assign75880_e115382_d_n9;
        locals.var_fac1_dn10 = assign75880_e115382_d_n10;
        locals.var_fac1_dn11 = assign75880_e115382_d_n11;
        locals.var_fac1_dn14 = assign75880_e115382_d_n14;
        locals.var_fac1_rv = 0.0;

        let (assign75890_e115388, assign75890_e115388_d_n0, assign75890_e115388_d_n2, assign75890_e115388_d_n4, assign75890_e115388_d_n5, assign75890_e115388_d_n6, assign75890_e115388_d_n7, assign75890_e115388_d_n8, assign75890_e115388_d_n9, assign75890_e115388_d_n10, assign75890_e115388_d_n11, assign75890_e115388_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75890_e115386: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign75890_e115386, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign75890_e115388;
        locals.var_fac1p2_dn0 = assign75890_e115388_d_n0;
        locals.var_fac1p2_dn2 = assign75890_e115388_d_n2;
        locals.var_fac1p2_dn4 = assign75890_e115388_d_n4;
        locals.var_fac1p2_dn5 = assign75890_e115388_d_n5;
        locals.var_fac1p2_dn6 = assign75890_e115388_d_n6;
        locals.var_fac1p2_dn7 = assign75890_e115388_d_n7;
        locals.var_fac1p2_dn8 = assign75890_e115388_d_n8;
        locals.var_fac1p2_dn9 = assign75890_e115388_d_n9;
        locals.var_fac1p2_dn10 = assign75890_e115388_d_n10;
        locals.var_fac1p2_dn11 = assign75890_e115388_d_n11;
        locals.var_fac1p2_dn14 = assign75890_e115388_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign75900_e115395, assign75900_e115395_d_n2, assign75900_e115395_d_n7, assign75900_e115395_d_n8, assign75900_e115395_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75900_e115391: f64 = (-locals.var_vgbgmt);
        let assign75900_e115393: f64 = (assign75900_e115391 + locals.var_uc_vfbover);
        (assign75900_e115393, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign75900_e115395;
        locals.var_vgpld_dn2 = assign75900_e115395_d_n2;
        locals.var_vgpld_dn7 = assign75900_e115395_d_n7;
        locals.var_vgpld_dn8 = assign75900_e115395_d_n8;
        locals.var_vgpld_dn9 = assign75900_e115395_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign75910_e115404, assign75910_e115404_d_n0, assign75910_e115404_d_n2, assign75910_e115404_d_n4, assign75910_e115404_d_n5, assign75910_e115404_d_n6, assign75910_e115404_d_n7, assign75910_e115404_d_n8, assign75910_e115404_d_n9, assign75910_e115404_d_n10, assign75910_e115404_d_n11, assign75910_e115404_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75910_e115398: f64 = (-locals.var_vxbgmtcl);
        let assign75910_e115401: f64 = (10.0 * 2.220446049250313e-16);
        let assign75910_e115402: f64 = (assign75910_e115398 + assign75910_e115401);
        (assign75910_e115402, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign75910_e115404;
        locals.var_vgb_fb_ld_dn0 = assign75910_e115404_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign75910_e115404_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign75910_e115404_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign75910_e115404_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign75910_e115404_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign75910_e115404_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign75910_e115404_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign75910_e115404_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign75910_e115404_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign75910_e115404_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign75910_e115404_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign75920_e115408, assign75920_e115408_d_n0, assign75920_e115408_d_n2, assign75920_e115408_d_n4, assign75920_e115408_d_n5, assign75920_e115408_d_n6, assign75920_e115408_d_n7, assign75920_e115408_d_n8, assign75920_e115408_d_n9, assign75920_e115408_d_n10, assign75920_e115408_d_n11, assign75920_e115408_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1765, locals.var_q_dep_ld__blk1765_dn0, locals.var_q_dep_ld__blk1765_dn2, locals.var_q_dep_ld__blk1765_dn4, locals.var_q_dep_ld__blk1765_dn5, locals.var_q_dep_ld__blk1765_dn6, locals.var_q_dep_ld__blk1765_dn7, locals.var_q_dep_ld__blk1765_dn8, locals.var_q_dep_ld__blk1765_dn9, locals.var_q_dep_ld__blk1765_dn10, locals.var_q_dep_ld__blk1765_dn11, locals.var_q_dep_ld__blk1765_dn14,)
    }
};
        locals.var_q_dep_ld__blk1765 = assign75920_e115408;
        locals.var_q_dep_ld__blk1765_dn0 = assign75920_e115408_d_n0;
        locals.var_q_dep_ld__blk1765_dn2 = assign75920_e115408_d_n2;
        locals.var_q_dep_ld__blk1765_dn4 = assign75920_e115408_d_n4;
        locals.var_q_dep_ld__blk1765_dn5 = assign75920_e115408_d_n5;
        locals.var_q_dep_ld__blk1765_dn6 = assign75920_e115408_d_n6;
        locals.var_q_dep_ld__blk1765_dn7 = assign75920_e115408_d_n7;
        locals.var_q_dep_ld__blk1765_dn8 = assign75920_e115408_d_n8;
        locals.var_q_dep_ld__blk1765_dn9 = assign75920_e115408_d_n9;
        locals.var_q_dep_ld__blk1765_dn10 = assign75920_e115408_d_n10;
        locals.var_q_dep_ld__blk1765_dn11 = assign75920_e115408_d_n11;
        locals.var_q_dep_ld__blk1765_dn14 = assign75920_e115408_d_n14;
        locals.var_q_dep_ld__blk1765_rv = 0.0;

        let (assign75930_e115414,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75930_e115412: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign75930_e115412,)
    } else {
        (locals.var_q_nsubld__blk1766,)
    }
};
        locals.var_q_nsubld__blk1766 = assign75930_e115414;
        locals.var_q_nsubld__blk1766_rv = 0.0;

        let (assign75940_e115420, assign75940_e115420_d_n0, assign75940_e115420_d_n2, assign75940_e115420_d_n4, assign75940_e115420_d_n5, assign75940_e115420_d_n6, assign75940_e115420_d_n7, assign75940_e115420_d_n8, assign75940_e115420_d_n9, assign75940_e115420_d_n10, assign75940_e115420_d_n11, assign75940_e115420_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75940_e115418: f64 = (locals.var_nin / locals.var_nover_func);
        (assign75940_e115418, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75940_e115420;
        locals.var_t0_dn0 = assign75940_e115420_d_n0;
        locals.var_t0_dn2 = assign75940_e115420_d_n2;
        locals.var_t0_dn4 = assign75940_e115420_d_n4;
        locals.var_t0_dn5 = assign75940_e115420_d_n5;
        locals.var_t0_dn6 = assign75940_e115420_d_n6;
        locals.var_t0_dn7 = assign75940_e115420_d_n7;
        locals.var_t0_dn8 = assign75940_e115420_d_n8;
        locals.var_t0_dn9 = assign75940_e115420_d_n9;
        locals.var_t0_dn10 = assign75940_e115420_d_n10;
        locals.var_t0_dn11 = assign75940_e115420_d_n11;
        locals.var_t0_dn14 = assign75940_e115420_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75950_e115426, assign75950_e115426_d_n0, assign75950_e115426_d_n2, assign75950_e115426_d_n4, assign75950_e115426_d_n5, assign75950_e115426_d_n6, assign75950_e115426_d_n7, assign75950_e115426_d_n8, assign75950_e115426_d_n9, assign75950_e115426_d_n10, assign75950_e115426_d_n11, assign75950_e115426_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75950_e115424: f64 = (locals.var_t0 * locals.var_t0);
        (assign75950_e115424, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign75950_e115426;
        locals.var_cnst1over_dn0 = assign75950_e115426_d_n0;
        locals.var_cnst1over_dn2 = assign75950_e115426_d_n2;
        locals.var_cnst1over_dn4 = assign75950_e115426_d_n4;
        locals.var_cnst1over_dn5 = assign75950_e115426_d_n5;
        locals.var_cnst1over_dn6 = assign75950_e115426_d_n6;
        locals.var_cnst1over_dn7 = assign75950_e115426_d_n7;
        locals.var_cnst1over_dn8 = assign75950_e115426_d_n8;
        locals.var_cnst1over_dn9 = assign75950_e115426_d_n9;
        locals.var_cnst1over_dn10 = assign75950_e115426_d_n10;
        locals.var_cnst1over_dn11 = assign75950_e115426_d_n11;
        locals.var_cnst1over_dn14 = assign75950_e115426_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign75960_e115429: f64 = (-locals.var_vxbgmtcl);
        let assign75960_e115430: f64 = (locals.var_beta * assign75960_e115429);
        let assign75960_e115432: f64 = if assign75960_e115430 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1783 = assign75960_e115432;
        locals.var_guard1783_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_287(
        locals: &mut StampLocals,
    ) {
        let (assign75970_e115447, assign75970_e115447_d_n0, assign75970_e115447_d_n2, assign75970_e115447_d_n4, assign75970_e115447_d_n5, assign75970_e115447_d_n6, assign75970_e115447_d_n7, assign75970_e115447_d_n8, assign75970_e115447_d_n9, assign75970_e115447_d_n10, assign75970_e115447_d_n11, assign75970_e115447_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 != 0.0)) {
        let assign75970_e115440: f64 = (-locals.var_vxbgmtcl);
        let assign75970_e115441: f64 = (locals.var_beta * assign75970_e115440);
        let assign75970_e115442: f64 = (1.0 + assign75970_e115441);
        let assign75970_e115444: f64 = (assign75970_e115442 - 500.0);
        let assign75970_e115445: f64 = (1.403592217853e217 * assign75970_e115444);
        (assign75970_e115445, (1.403592217853e217 * ((locals.var_beta_dn0 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign75970_e115447;
        locals.var_exp_bvbs_dn0 = assign75970_e115447_d_n0;
        locals.var_exp_bvbs_dn2 = assign75970_e115447_d_n2;
        locals.var_exp_bvbs_dn4 = assign75970_e115447_d_n4;
        locals.var_exp_bvbs_dn5 = assign75970_e115447_d_n5;
        locals.var_exp_bvbs_dn6 = assign75970_e115447_d_n6;
        locals.var_exp_bvbs_dn7 = assign75970_e115447_d_n7;
        locals.var_exp_bvbs_dn8 = assign75970_e115447_d_n8;
        locals.var_exp_bvbs_dn9 = assign75970_e115447_d_n9;
        locals.var_exp_bvbs_dn10 = assign75970_e115447_d_n10;
        locals.var_exp_bvbs_dn11 = assign75970_e115447_d_n11;
        locals.var_exp_bvbs_dn14 = assign75970_e115447_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign75980_e115453, assign75980_e115453_d_n0, assign75980_e115453_d_n2, assign75980_e115453_d_n4, assign75980_e115453_d_n5, assign75980_e115453_d_n6, assign75980_e115453_d_n7, assign75980_e115453_d_n8, assign75980_e115453_d_n9, assign75980_e115453_d_n10, assign75980_e115453_d_n11, assign75980_e115453_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75980_e115453;
        locals.var_t0_dn0 = assign75980_e115453_d_n0;
        locals.var_t0_dn2 = assign75980_e115453_d_n2;
        locals.var_t0_dn4 = assign75980_e115453_d_n4;
        locals.var_t0_dn5 = assign75980_e115453_d_n5;
        locals.var_t0_dn6 = assign75980_e115453_d_n6;
        locals.var_t0_dn7 = assign75980_e115453_d_n7;
        locals.var_t0_dn8 = assign75980_e115453_d_n8;
        locals.var_t0_dn9 = assign75980_e115453_d_n9;
        locals.var_t0_dn10 = assign75980_e115453_d_n10;
        locals.var_t0_dn11 = assign75980_e115453_d_n11;
        locals.var_t0_dn14 = assign75980_e115453_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75990_e115463, assign75990_e115463_d_n0, assign75990_e115463_d_n2, assign75990_e115463_d_n4, assign75990_e115463_d_n5, assign75990_e115463_d_n6, assign75990_e115463_d_n7, assign75990_e115463_d_n8, assign75990_e115463_d_n9, assign75990_e115463_d_n10, assign75990_e115463_d_n11, assign75990_e115463_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign75990_e115460: f64 = (-locals.var_vxbgmtcl);
        let assign75990_e115461: f64 = (locals.var_beta * assign75990_e115460);
        (assign75990_e115461, ((locals.var_beta_dn0 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75990_e115463;
        locals.var_tmf1_dn0 = assign75990_e115463_d_n0;
        locals.var_tmf1_dn2 = assign75990_e115463_d_n2;
        locals.var_tmf1_dn4 = assign75990_e115463_d_n4;
        locals.var_tmf1_dn5 = assign75990_e115463_d_n5;
        locals.var_tmf1_dn6 = assign75990_e115463_d_n6;
        locals.var_tmf1_dn7 = assign75990_e115463_d_n7;
        locals.var_tmf1_dn8 = assign75990_e115463_d_n8;
        locals.var_tmf1_dn9 = assign75990_e115463_d_n9;
        locals.var_tmf1_dn10 = assign75990_e115463_d_n10;
        locals.var_tmf1_dn11 = assign75990_e115463_d_n11;
        locals.var_tmf1_dn14 = assign75990_e115463_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign76000_e115470, assign76000_e115470_d_n0, assign76000_e115470_d_n2, assign76000_e115470_d_n4, assign76000_e115470_d_n5, assign76000_e115470_d_n6, assign76000_e115470_d_n7, assign76000_e115470_d_n8, assign76000_e115470_d_n9, assign76000_e115470_d_n10, assign76000_e115470_d_n11, assign76000_e115470_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign76000_e115470;
        locals.var_exp_bvbs_dn0 = assign76000_e115470_d_n0;
        locals.var_exp_bvbs_dn2 = assign76000_e115470_d_n2;
        locals.var_exp_bvbs_dn4 = assign76000_e115470_d_n4;
        locals.var_exp_bvbs_dn5 = assign76000_e115470_d_n5;
        locals.var_exp_bvbs_dn6 = assign76000_e115470_d_n6;
        locals.var_exp_bvbs_dn7 = assign76000_e115470_d_n7;
        locals.var_exp_bvbs_dn8 = assign76000_e115470_d_n8;
        locals.var_exp_bvbs_dn9 = assign76000_e115470_d_n9;
        locals.var_exp_bvbs_dn10 = assign76000_e115470_d_n10;
        locals.var_exp_bvbs_dn11 = assign76000_e115470_d_n11;
        locals.var_exp_bvbs_dn14 = assign76000_e115470_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign76010_loop_guard: usize = 0;
        while {
            let assign76010_cond_e115478: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign76010_cond_e115478 != 0.0
        } {
            assign76010_loop_guard += 1;
            assert!(assign76010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76010_body0_e115487, assign76010_body0_e115487_d_n0, assign76010_body0_e115487_d_n2, assign76010_body0_e115487_d_n4, assign76010_body0_e115487_d_n5, assign76010_body0_e115487_d_n6, assign76010_body0_e115487_d_n7, assign76010_body0_e115487_d_n8, assign76010_body0_e115487_d_n9, assign76010_body0_e115487_d_n10, assign76010_body0_e115487_d_n11, assign76010_body0_e115487_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76010_body0_e115485: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign76010_body0_e115485, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign76010_body0_e115487;
            locals.var_exp_bvbs_dn0 = assign76010_body0_e115487_d_n0;
            locals.var_exp_bvbs_dn2 = assign76010_body0_e115487_d_n2;
            locals.var_exp_bvbs_dn4 = assign76010_body0_e115487_d_n4;
            locals.var_exp_bvbs_dn5 = assign76010_body0_e115487_d_n5;
            locals.var_exp_bvbs_dn6 = assign76010_body0_e115487_d_n6;
            locals.var_exp_bvbs_dn7 = assign76010_body0_e115487_d_n7;
            locals.var_exp_bvbs_dn8 = assign76010_body0_e115487_d_n8;
            locals.var_exp_bvbs_dn9 = assign76010_body0_e115487_d_n9;
            locals.var_exp_bvbs_dn10 = assign76010_body0_e115487_d_n10;
            locals.var_exp_bvbs_dn11 = assign76010_body0_e115487_d_n11;
            locals.var_exp_bvbs_dn14 = assign76010_body0_e115487_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign76010_body1_e115496, assign76010_body1_e115496_d_n0, assign76010_body1_e115496_d_n2, assign76010_body1_e115496_d_n4, assign76010_body1_e115496_d_n5, assign76010_body1_e115496_d_n6, assign76010_body1_e115496_d_n7, assign76010_body1_e115496_d_n8, assign76010_body1_e115496_d_n9, assign76010_body1_e115496_d_n10, assign76010_body1_e115496_d_n11, assign76010_body1_e115496_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76010_body1_e115494: f64 = (locals.var_tmf1 - 60.0);
        (assign76010_body1_e115494, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign76010_body1_e115496;
            locals.var_tmf1_dn0 = assign76010_body1_e115496_d_n0;
            locals.var_tmf1_dn2 = assign76010_body1_e115496_d_n2;
            locals.var_tmf1_dn4 = assign76010_body1_e115496_d_n4;
            locals.var_tmf1_dn5 = assign76010_body1_e115496_d_n5;
            locals.var_tmf1_dn6 = assign76010_body1_e115496_d_n6;
            locals.var_tmf1_dn7 = assign76010_body1_e115496_d_n7;
            locals.var_tmf1_dn8 = assign76010_body1_e115496_d_n8;
            locals.var_tmf1_dn9 = assign76010_body1_e115496_d_n9;
            locals.var_tmf1_dn10 = assign76010_body1_e115496_d_n10;
            locals.var_tmf1_dn11 = assign76010_body1_e115496_d_n11;
            locals.var_tmf1_dn14 = assign76010_body1_e115496_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign76020_e115506, assign76020_e115506_d_n0, assign76020_e115506_d_n2, assign76020_e115506_d_n4, assign76020_e115506_d_n5, assign76020_e115506_d_n6, assign76020_e115506_d_n7, assign76020_e115506_d_n8, assign76020_e115506_d_n9, assign76020_e115506_d_n10, assign76020_e115506_d_n11, assign76020_e115506_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76020_e115503: f64 = (locals.var_tmf1).exp();
        let assign76020_e115504: f64 = (locals.var_exp_bvbs * assign76020_e115503);
        (assign76020_e115504, ((locals.var_exp_bvbs_dn0 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign76020_e115506;
        locals.var_exp_bvbs_dn0 = assign76020_e115506_d_n0;
        locals.var_exp_bvbs_dn2 = assign76020_e115506_d_n2;
        locals.var_exp_bvbs_dn4 = assign76020_e115506_d_n4;
        locals.var_exp_bvbs_dn5 = assign76020_e115506_d_n5;
        locals.var_exp_bvbs_dn6 = assign76020_e115506_d_n6;
        locals.var_exp_bvbs_dn7 = assign76020_e115506_d_n7;
        locals.var_exp_bvbs_dn8 = assign76020_e115506_d_n8;
        locals.var_exp_bvbs_dn9 = assign76020_e115506_d_n9;
        locals.var_exp_bvbs_dn10 = assign76020_e115506_d_n10;
        locals.var_exp_bvbs_dn11 = assign76020_e115506_d_n11;
        locals.var_exp_bvbs_dn14 = assign76020_e115506_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign76030_e115513, assign76030_e115513_d_n0, assign76030_e115513_d_n2, assign76030_e115513_d_n4, assign76030_e115513_d_n5, assign76030_e115513_d_n6, assign76030_e115513_d_n7, assign76030_e115513_d_n8, assign76030_e115513_d_n9, assign76030_e115513_d_n10, assign76030_e115513_d_n11, assign76030_e115513_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76030_e115513;
        locals.var_t0_dn0 = assign76030_e115513_d_n0;
        locals.var_t0_dn2 = assign76030_e115513_d_n2;
        locals.var_t0_dn4 = assign76030_e115513_d_n4;
        locals.var_t0_dn5 = assign76030_e115513_d_n5;
        locals.var_t0_dn6 = assign76030_e115513_d_n6;
        locals.var_t0_dn7 = assign76030_e115513_d_n7;
        locals.var_t0_dn8 = assign76030_e115513_d_n8;
        locals.var_t0_dn9 = assign76030_e115513_d_n9;
        locals.var_t0_dn10 = assign76030_e115513_d_n10;
        locals.var_t0_dn11 = assign76030_e115513_d_n11;
        locals.var_t0_dn14 = assign76030_e115513_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76040_e115526, assign76040_e115526_d_n0, assign76040_e115526_d_n2, assign76040_e115526_d_n4, assign76040_e115526_d_n5, assign76040_e115526_d_n6, assign76040_e115526_d_n7, assign76040_e115526_d_n8, assign76040_e115526_d_n9, assign76040_e115526_d_n10, assign76040_e115526_d_n11, assign76040_e115526_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76040_e115518: f64 = (-locals.var_vgpld);
        let assign76040_e115520: f64 = (assign76040_e115518 * 0.5);
        let assign76040_e115522: f64 = (assign76040_e115520 - 0.5);
        let assign76040_e115524: f64 = (assign76040_e115522 - 1.0);
        (assign76040_e115524, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76040_e115526;
        locals.var_tmf1_dn0 = assign76040_e115526_d_n0;
        locals.var_tmf1_dn2 = assign76040_e115526_d_n2;
        locals.var_tmf1_dn4 = assign76040_e115526_d_n4;
        locals.var_tmf1_dn5 = assign76040_e115526_d_n5;
        locals.var_tmf1_dn6 = assign76040_e115526_d_n6;
        locals.var_tmf1_dn7 = assign76040_e115526_d_n7;
        locals.var_tmf1_dn8 = assign76040_e115526_d_n8;
        locals.var_tmf1_dn9 = assign76040_e115526_d_n9;
        locals.var_tmf1_dn10 = assign76040_e115526_d_n10;
        locals.var_tmf1_dn11 = assign76040_e115526_d_n11;
        locals.var_tmf1_dn14 = assign76040_e115526_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign76050_e115536, assign76050_e115536_d_n0, assign76050_e115536_d_n2, assign76050_e115536_d_n4, assign76050_e115536_d_n5, assign76050_e115536_d_n6, assign76050_e115536_d_n7, assign76050_e115536_d_n8, assign76050_e115536_d_n9, assign76050_e115536_d_n10, assign76050_e115536_d_n11, assign76050_e115536_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76050_e115532: f64 = (4.0 * 0.5);
        let assign76050_e115534: f64 = assign76050_e115532;
        (assign76050_e115534, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76050_e115536;
        locals.var_tmf2_dn0 = assign76050_e115536_d_n0;
        locals.var_tmf2_dn2 = assign76050_e115536_d_n2;
        locals.var_tmf2_dn4 = assign76050_e115536_d_n4;
        locals.var_tmf2_dn5 = assign76050_e115536_d_n5;
        locals.var_tmf2_dn6 = assign76050_e115536_d_n6;
        locals.var_tmf2_dn7 = assign76050_e115536_d_n7;
        locals.var_tmf2_dn8 = assign76050_e115536_d_n8;
        locals.var_tmf2_dn9 = assign76050_e115536_d_n9;
        locals.var_tmf2_dn10 = assign76050_e115536_d_n10;
        locals.var_tmf2_dn11 = assign76050_e115536_d_n11;
        locals.var_tmf2_dn14 = assign76050_e115536_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76060_e115548, assign76060_e115548_d_n0, assign76060_e115548_d_n2, assign76060_e115548_d_n4, assign76060_e115548_d_n5, assign76060_e115548_d_n6, assign76060_e115548_d_n7, assign76060_e115548_d_n8, assign76060_e115548_d_n9, assign76060_e115548_d_n10, assign76060_e115548_d_n11, assign76060_e115548_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign76060_e115546, assign76060_e115546_d_n0, assign76060_e115546_d_n2, assign76060_e115546_d_n4, assign76060_e115546_d_n5, assign76060_e115546_d_n6, assign76060_e115546_d_n7, assign76060_e115546_d_n8, assign76060_e115546_d_n9, assign76060_e115546_d_n10, assign76060_e115546_d_n11, assign76060_e115546_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign76060_e115545: f64 = (-locals.var_tmf2);
                (assign76060_e115545, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign76060_e115546, assign76060_e115546_d_n0, assign76060_e115546_d_n2, assign76060_e115546_d_n4, assign76060_e115546_d_n5, assign76060_e115546_d_n6, assign76060_e115546_d_n7, assign76060_e115546_d_n8, assign76060_e115546_d_n9, assign76060_e115546_d_n10, assign76060_e115546_d_n11, assign76060_e115546_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76060_e115548;
        locals.var_tmf2_dn0 = assign76060_e115548_d_n0;
        locals.var_tmf2_dn2 = assign76060_e115548_d_n2;
        locals.var_tmf2_dn4 = assign76060_e115548_d_n4;
        locals.var_tmf2_dn5 = assign76060_e115548_d_n5;
        locals.var_tmf2_dn6 = assign76060_e115548_d_n6;
        locals.var_tmf2_dn7 = assign76060_e115548_d_n7;
        locals.var_tmf2_dn8 = assign76060_e115548_d_n8;
        locals.var_tmf2_dn9 = assign76060_e115548_d_n9;
        locals.var_tmf2_dn10 = assign76060_e115548_d_n10;
        locals.var_tmf2_dn11 = assign76060_e115548_d_n11;
        locals.var_tmf2_dn14 = assign76060_e115548_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76070_e115559, assign76070_e115559_d_n0, assign76070_e115559_d_n2, assign76070_e115559_d_n4, assign76070_e115559_d_n5, assign76070_e115559_d_n6, assign76070_e115559_d_n7, assign76070_e115559_d_n8, assign76070_e115559_d_n9, assign76070_e115559_d_n10, assign76070_e115559_d_n11, assign76070_e115559_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76070_e115554: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign76070_e115556: f64 = (assign76070_e115554 + locals.var_tmf2);
        let assign76070_e115557: f64 = (assign76070_e115556).sqrt();
        (assign76070_e115557, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign76070_e115557)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76070_e115559;
        locals.var_tmf2_dn0 = assign76070_e115559_d_n0;
        locals.var_tmf2_dn2 = assign76070_e115559_d_n2;
        locals.var_tmf2_dn4 = assign76070_e115559_d_n4;
        locals.var_tmf2_dn5 = assign76070_e115559_d_n5;
        locals.var_tmf2_dn6 = assign76070_e115559_d_n6;
        locals.var_tmf2_dn7 = assign76070_e115559_d_n7;
        locals.var_tmf2_dn8 = assign76070_e115559_d_n8;
        locals.var_tmf2_dn9 = assign76070_e115559_d_n9;
        locals.var_tmf2_dn10 = assign76070_e115559_d_n10;
        locals.var_tmf2_dn11 = assign76070_e115559_d_n11;
        locals.var_tmf2_dn14 = assign76070_e115559_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76080_e115571, assign76080_e115571_d_n0, assign76080_e115571_d_n2, assign76080_e115571_d_n4, assign76080_e115571_d_n5, assign76080_e115571_d_n6, assign76080_e115571_d_n7, assign76080_e115571_d_n8, assign76080_e115571_d_n9, assign76080_e115571_d_n10, assign76080_e115571_d_n11, assign76080_e115571_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76080_e115567: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign76080_e115568: f64 = (1.0 + assign76080_e115567);
        let assign76080_e115569: f64 = (0.5 * assign76080_e115568);
        (assign76080_e115569, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76080_e115571;
        locals.var_t0_dn0 = assign76080_e115571_d_n0;
        locals.var_t0_dn2 = assign76080_e115571_d_n2;
        locals.var_t0_dn4 = assign76080_e115571_d_n4;
        locals.var_t0_dn5 = assign76080_e115571_d_n5;
        locals.var_t0_dn6 = assign76080_e115571_d_n6;
        locals.var_t0_dn7 = assign76080_e115571_d_n7;
        locals.var_t0_dn8 = assign76080_e115571_d_n8;
        locals.var_t0_dn9 = assign76080_e115571_d_n9;
        locals.var_t0_dn10 = assign76080_e115571_d_n10;
        locals.var_t0_dn11 = assign76080_e115571_d_n11;
        locals.var_t0_dn14 = assign76080_e115571_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76090_e115583, assign76090_e115583_d_n0, assign76090_e115583_d_n2, assign76090_e115583_d_n4, assign76090_e115583_d_n5, assign76090_e115583_d_n6, assign76090_e115583_d_n7, assign76090_e115583_d_n8, assign76090_e115583_d_n9, assign76090_e115583_d_n10, assign76090_e115583_d_n11, assign76090_e115583_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76090_e115579: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign76090_e115580: f64 = (0.5 * assign76090_e115579);
        let assign76090_e115581: f64 = (0.5 + assign76090_e115580);
        (assign76090_e115581, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76090_e115583;
        locals.var_t1_dn0 = assign76090_e115583_d_n0;
        locals.var_t1_dn2 = assign76090_e115583_d_n2;
        locals.var_t1_dn4 = assign76090_e115583_d_n4;
        locals.var_t1_dn5 = assign76090_e115583_d_n5;
        locals.var_t1_dn6 = assign76090_e115583_d_n6;
        locals.var_t1_dn7 = assign76090_e115583_d_n7;
        locals.var_t1_dn8 = assign76090_e115583_d_n8;
        locals.var_t1_dn9 = assign76090_e115583_d_n9;
        locals.var_t1_dn10 = assign76090_e115583_d_n10;
        locals.var_t1_dn11 = assign76090_e115583_d_n11;
        locals.var_t1_dn14 = assign76090_e115583_d_n14;
        locals.var_t1_rv = 0.0;

        let assign76100_e115586: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76100_e115589: f64 = (-locals.var_t1);
        let assign76100_e115594: f64 = if ((assign76100_e115586 > assign76100_e115589) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1784 = assign76100_e115594;
        locals.var_guard1784_rv = 0.0;

        let (assign76110_e115608, assign76110_e115608_d_n0, assign76110_e115608_d_n2, assign76110_e115608_d_n4, assign76110_e115608_d_n5, assign76110_e115608_d_n6, assign76110_e115608_d_n7, assign76110_e115608_d_n8, assign76110_e115608_d_n9, assign76110_e115608_d_n10, assign76110_e115608_d_n11, assign76110_e115608_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76110_e115602: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76110_e115604: f64 = assign76110_e115602;
        let assign76110_e115606: f64 = (assign76110_e115604 + locals.var_t1);
        (assign76110_e115606, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76110_e115608;
        locals.var_tmf1_dn0 = assign76110_e115608_d_n0;
        locals.var_tmf1_dn2 = assign76110_e115608_d_n2;
        locals.var_tmf1_dn4 = assign76110_e115608_d_n4;
        locals.var_tmf1_dn5 = assign76110_e115608_d_n5;
        locals.var_tmf1_dn6 = assign76110_e115608_d_n6;
        locals.var_tmf1_dn7 = assign76110_e115608_d_n7;
        locals.var_tmf1_dn8 = assign76110_e115608_d_n8;
        locals.var_tmf1_dn9 = assign76110_e115608_d_n9;
        locals.var_tmf1_dn10 = assign76110_e115608_d_n10;
        locals.var_tmf1_dn11 = assign76110_e115608_d_n11;
        locals.var_tmf1_dn14 = assign76110_e115608_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign76120_e115618, assign76120_e115618_d_n0, assign76120_e115618_d_n2, assign76120_e115618_d_n4, assign76120_e115618_d_n5, assign76120_e115618_d_n6, assign76120_e115618_d_n7, assign76120_e115618_d_n8, assign76120_e115618_d_n9, assign76120_e115618_d_n10, assign76120_e115618_d_n11, assign76120_e115618_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76120_e115616: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign76120_e115616, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign76120_e115618;
        locals.var_x2_dn0 = assign76120_e115618_d_n0;
        locals.var_x2_dn2 = assign76120_e115618_d_n2;
        locals.var_x2_dn4 = assign76120_e115618_d_n4;
        locals.var_x2_dn5 = assign76120_e115618_d_n5;
        locals.var_x2_dn6 = assign76120_e115618_d_n6;
        locals.var_x2_dn7 = assign76120_e115618_d_n7;
        locals.var_x2_dn8 = assign76120_e115618_d_n8;
        locals.var_x2_dn9 = assign76120_e115618_d_n9;
        locals.var_x2_dn10 = assign76120_e115618_d_n10;
        locals.var_x2_dn11 = assign76120_e115618_d_n11;
        locals.var_x2_dn14 = assign76120_e115618_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign76130_e115628, assign76130_e115628_d_n0, assign76130_e115628_d_n2, assign76130_e115628_d_n4, assign76130_e115628_d_n5, assign76130_e115628_d_n6, assign76130_e115628_d_n7, assign76130_e115628_d_n8, assign76130_e115628_d_n9, assign76130_e115628_d_n10, assign76130_e115628_d_n11, assign76130_e115628_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76130_e115626: f64 = (locals.var_t1 * locals.var_t1);
        (assign76130_e115626, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign76130_e115628;
        locals.var_xmax2_dn0 = assign76130_e115628_d_n0;
        locals.var_xmax2_dn2 = assign76130_e115628_d_n2;
        locals.var_xmax2_dn4 = assign76130_e115628_d_n4;
        locals.var_xmax2_dn5 = assign76130_e115628_d_n5;
        locals.var_xmax2_dn6 = assign76130_e115628_d_n6;
        locals.var_xmax2_dn7 = assign76130_e115628_d_n7;
        locals.var_xmax2_dn8 = assign76130_e115628_d_n8;
        locals.var_xmax2_dn9 = assign76130_e115628_d_n9;
        locals.var_xmax2_dn10 = assign76130_e115628_d_n10;
        locals.var_xmax2_dn11 = assign76130_e115628_d_n11;
        locals.var_xmax2_dn14 = assign76130_e115628_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign76140_e115636, assign76140_e115636_d_n0, assign76140_e115636_d_n2, assign76140_e115636_d_n4, assign76140_e115636_d_n5, assign76140_e115636_d_n6, assign76140_e115636_d_n7, assign76140_e115636_d_n8, assign76140_e115636_d_n9, assign76140_e115636_d_n10, assign76140_e115636_d_n11, assign76140_e115636_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76140_e115636;
        locals.var_xp_dn0 = assign76140_e115636_d_n0;
        locals.var_xp_dn2 = assign76140_e115636_d_n2;
        locals.var_xp_dn4 = assign76140_e115636_d_n4;
        locals.var_xp_dn5 = assign76140_e115636_d_n5;
        locals.var_xp_dn6 = assign76140_e115636_d_n6;
        locals.var_xp_dn7 = assign76140_e115636_d_n7;
        locals.var_xp_dn8 = assign76140_e115636_d_n8;
        locals.var_xp_dn9 = assign76140_e115636_d_n9;
        locals.var_xp_dn10 = assign76140_e115636_d_n10;
        locals.var_xp_dn11 = assign76140_e115636_d_n11;
        locals.var_xp_dn14 = assign76140_e115636_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign76150_e115644, assign76150_e115644_d_n0, assign76150_e115644_d_n2, assign76150_e115644_d_n4, assign76150_e115644_d_n5, assign76150_e115644_d_n6, assign76150_e115644_d_n7, assign76150_e115644_d_n8, assign76150_e115644_d_n9, assign76150_e115644_d_n10, assign76150_e115644_d_n11, assign76150_e115644_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76150_e115644;
        locals.var_xmp_dn0 = assign76150_e115644_d_n0;
        locals.var_xmp_dn2 = assign76150_e115644_d_n2;
        locals.var_xmp_dn4 = assign76150_e115644_d_n4;
        locals.var_xmp_dn5 = assign76150_e115644_d_n5;
        locals.var_xmp_dn6 = assign76150_e115644_d_n6;
        locals.var_xmp_dn7 = assign76150_e115644_d_n7;
        locals.var_xmp_dn8 = assign76150_e115644_d_n8;
        locals.var_xmp_dn9 = assign76150_e115644_d_n9;
        locals.var_xmp_dn10 = assign76150_e115644_d_n10;
        locals.var_xmp_dn11 = assign76150_e115644_d_n11;
        locals.var_xmp_dn14 = assign76150_e115644_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign76160_e115652,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76160_e115652;
        locals.var_m0_rv = 0.0;

        let (assign76170_e115660,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76170_e115660;
        locals.var_mm_rv = 0.0;

        let (assign76180_e115668, assign76180_e115668_d_n0, assign76180_e115668_d_n2, assign76180_e115668_d_n4, assign76180_e115668_d_n5, assign76180_e115668_d_n6, assign76180_e115668_d_n7, assign76180_e115668_d_n8, assign76180_e115668_d_n9, assign76180_e115668_d_n10, assign76180_e115668_d_n11, assign76180_e115668_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76180_e115668;
        locals.var_arg_dn0 = assign76180_e115668_d_n0;
        locals.var_arg_dn2 = assign76180_e115668_d_n2;
        locals.var_arg_dn4 = assign76180_e115668_d_n4;
        locals.var_arg_dn5 = assign76180_e115668_d_n5;
        locals.var_arg_dn6 = assign76180_e115668_d_n6;
        locals.var_arg_dn7 = assign76180_e115668_d_n7;
        locals.var_arg_dn8 = assign76180_e115668_d_n8;
        locals.var_arg_dn9 = assign76180_e115668_d_n9;
        locals.var_arg_dn10 = assign76180_e115668_d_n10;
        locals.var_arg_dn11 = assign76180_e115668_d_n11;
        locals.var_arg_dn14 = assign76180_e115668_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_288(
        locals: &mut StampLocals,
    ) {
        let (assign76190_e115676, assign76190_e115676_d_n0, assign76190_e115676_d_n2, assign76190_e115676_d_n4, assign76190_e115676_d_n5, assign76190_e115676_d_n6, assign76190_e115676_d_n7, assign76190_e115676_d_n8, assign76190_e115676_d_n9, assign76190_e115676_d_n10, assign76190_e115676_d_n11, assign76190_e115676_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76190_e115676;
        locals.var_dnm_dn0 = assign76190_e115676_d_n0;
        locals.var_dnm_dn2 = assign76190_e115676_d_n2;
        locals.var_dnm_dn4 = assign76190_e115676_d_n4;
        locals.var_dnm_dn5 = assign76190_e115676_d_n5;
        locals.var_dnm_dn6 = assign76190_e115676_d_n6;
        locals.var_dnm_dn7 = assign76190_e115676_d_n7;
        locals.var_dnm_dn8 = assign76190_e115676_d_n8;
        locals.var_dnm_dn9 = assign76190_e115676_d_n9;
        locals.var_dnm_dn10 = assign76190_e115676_d_n10;
        locals.var_dnm_dn11 = assign76190_e115676_d_n11;
        locals.var_dnm_dn14 = assign76190_e115676_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76200_e115686, assign76200_e115686_d_n0, assign76200_e115686_d_n2, assign76200_e115686_d_n4, assign76200_e115686_d_n5, assign76200_e115686_d_n6, assign76200_e115686_d_n7, assign76200_e115686_d_n8, assign76200_e115686_d_n9, assign76200_e115686_d_n10, assign76200_e115686_d_n11, assign76200_e115686_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76200_e115684: f64 = (locals.var_xp * locals.var_x2);
        (assign76200_e115684, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76200_e115686;
        locals.var_xp_dn0 = assign76200_e115686_d_n0;
        locals.var_xp_dn2 = assign76200_e115686_d_n2;
        locals.var_xp_dn4 = assign76200_e115686_d_n4;
        locals.var_xp_dn5 = assign76200_e115686_d_n5;
        locals.var_xp_dn6 = assign76200_e115686_d_n6;
        locals.var_xp_dn7 = assign76200_e115686_d_n7;
        locals.var_xp_dn8 = assign76200_e115686_d_n8;
        locals.var_xp_dn9 = assign76200_e115686_d_n9;
        locals.var_xp_dn10 = assign76200_e115686_d_n10;
        locals.var_xp_dn11 = assign76200_e115686_d_n11;
        locals.var_xp_dn14 = assign76200_e115686_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign76210_e115696, assign76210_e115696_d_n0, assign76210_e115696_d_n2, assign76210_e115696_d_n4, assign76210_e115696_d_n5, assign76210_e115696_d_n6, assign76210_e115696_d_n7, assign76210_e115696_d_n8, assign76210_e115696_d_n9, assign76210_e115696_d_n10, assign76210_e115696_d_n11, assign76210_e115696_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76210_e115694: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign76210_e115694, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76210_e115696;
        locals.var_xmp_dn0 = assign76210_e115696_d_n0;
        locals.var_xmp_dn2 = assign76210_e115696_d_n2;
        locals.var_xmp_dn4 = assign76210_e115696_d_n4;
        locals.var_xmp_dn5 = assign76210_e115696_d_n5;
        locals.var_xmp_dn6 = assign76210_e115696_d_n6;
        locals.var_xmp_dn7 = assign76210_e115696_d_n7;
        locals.var_xmp_dn8 = assign76210_e115696_d_n8;
        locals.var_xmp_dn9 = assign76210_e115696_d_n9;
        locals.var_xmp_dn10 = assign76210_e115696_d_n10;
        locals.var_xmp_dn11 = assign76210_e115696_d_n11;
        locals.var_xmp_dn14 = assign76210_e115696_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign76220_e115706, assign76220_e115706_d_n0, assign76220_e115706_d_n2, assign76220_e115706_d_n4, assign76220_e115706_d_n5, assign76220_e115706_d_n6, assign76220_e115706_d_n7, assign76220_e115706_d_n8, assign76220_e115706_d_n9, assign76220_e115706_d_n10, assign76220_e115706_d_n11, assign76220_e115706_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76220_e115704: f64 = (locals.var_xp + locals.var_xmp);
        (assign76220_e115704, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76220_e115706;
        locals.var_arg_dn0 = assign76220_e115706_d_n0;
        locals.var_arg_dn2 = assign76220_e115706_d_n2;
        locals.var_arg_dn4 = assign76220_e115706_d_n4;
        locals.var_arg_dn5 = assign76220_e115706_d_n5;
        locals.var_arg_dn6 = assign76220_e115706_d_n6;
        locals.var_arg_dn7 = assign76220_e115706_d_n7;
        locals.var_arg_dn8 = assign76220_e115706_d_n8;
        locals.var_arg_dn9 = assign76220_e115706_d_n9;
        locals.var_arg_dn10 = assign76220_e115706_d_n10;
        locals.var_arg_dn11 = assign76220_e115706_d_n11;
        locals.var_arg_dn14 = assign76220_e115706_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign76230_e115714, assign76230_e115714_d_n0, assign76230_e115714_d_n2, assign76230_e115714_d_n4, assign76230_e115714_d_n5, assign76230_e115714_d_n6, assign76230_e115714_d_n7, assign76230_e115714_d_n8, assign76230_e115714_d_n9, assign76230_e115714_d_n10, assign76230_e115714_d_n11, assign76230_e115714_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76230_e115714;
        locals.var_dnm_dn0 = assign76230_e115714_d_n0;
        locals.var_dnm_dn2 = assign76230_e115714_d_n2;
        locals.var_dnm_dn4 = assign76230_e115714_d_n4;
        locals.var_dnm_dn5 = assign76230_e115714_d_n5;
        locals.var_dnm_dn6 = assign76230_e115714_d_n6;
        locals.var_dnm_dn7 = assign76230_e115714_d_n7;
        locals.var_dnm_dn8 = assign76230_e115714_d_n8;
        locals.var_dnm_dn9 = assign76230_e115714_d_n9;
        locals.var_dnm_dn10 = assign76230_e115714_d_n10;
        locals.var_dnm_dn11 = assign76230_e115714_d_n11;
        locals.var_dnm_dn14 = assign76230_e115714_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign76240_e115729: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1785 = assign76240_e115729;
        locals.var_guard1785_rv = 0.0;

        let assign76250_e115732: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1786 = assign76250_e115732;
        locals.var_guard1786_rv = 0.0;

        let (assign76260_e115744,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76260_e115744;
        locals.var_mm_rv = 0.0;

        let assign76270_e115747: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign76270_e115747;
        locals.var_guard1787_rv = 0.0;

        let (assign76280_e115762,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76280_e115762;
        locals.var_mm_rv = 0.0;

        let assign76290_e115765: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1788 = assign76290_e115765;
        locals.var_guard1788_rv = 0.0;

        let (assign76300_e115783,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 == 0.0)) && (locals.var_guard1788 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76300_e115783;
        locals.var_mm_rv = 0.0;

        let assign76310_e115786: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1789 = assign76310_e115786;
        locals.var_guard1789_rv = 0.0;

        let (assign76320_e115807,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 == 0.0)) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76320_e115807;
        locals.var_mm_rv = 0.0;

        let (assign76330_e115817,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76330_e115817;
        locals.var_m0_rv = 0.0;

        let mut assign76340_loop_guard: usize = 0;
        while {
            let assign76340_cond_e115828: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign76340_cond_e115828 != 0.0
        } {
            assign76340_loop_guard += 1;
            assert!(assign76340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76340_body0_e115839, assign76340_body0_e115839_d_n0, assign76340_body0_e115839_d_n2, assign76340_body0_e115839_d_n4, assign76340_body0_e115839_d_n5, assign76340_body0_e115839_d_n6, assign76340_body0_e115839_d_n7, assign76340_body0_e115839_d_n8, assign76340_body0_e115839_d_n9, assign76340_body0_e115839_d_n10, assign76340_body0_e115839_d_n11, assign76340_body0_e115839_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign76340_body0_e115837: f64 = (locals.var_dnm).sqrt();
        (assign76340_body0_e115837, (locals.var_dnm_dn0 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn2 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn4 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn5 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn6 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn7 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn8 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn9 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn10 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn11 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn14 / (2.0 * assign76340_body0_e115837)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign76340_body0_e115839;
            locals.var_dnm_dn0 = assign76340_body0_e115839_d_n0;
            locals.var_dnm_dn2 = assign76340_body0_e115839_d_n2;
            locals.var_dnm_dn4 = assign76340_body0_e115839_d_n4;
            locals.var_dnm_dn5 = assign76340_body0_e115839_d_n5;
            locals.var_dnm_dn6 = assign76340_body0_e115839_d_n6;
            locals.var_dnm_dn7 = assign76340_body0_e115839_d_n7;
            locals.var_dnm_dn8 = assign76340_body0_e115839_d_n8;
            locals.var_dnm_dn9 = assign76340_body0_e115839_d_n9;
            locals.var_dnm_dn10 = assign76340_body0_e115839_d_n10;
            locals.var_dnm_dn11 = assign76340_body0_e115839_d_n11;
            locals.var_dnm_dn14 = assign76340_body0_e115839_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign76340_body1_e115851,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign76340_body1_e115849: f64 = (locals.var_m0 + 1.0);
        (assign76340_body1_e115849,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign76340_body1_e115851;
            locals.var_m0_rv = 0.0;
        }

        let (assign76350_e115873, assign76350_e115873_d_n0, assign76350_e115873_d_n2, assign76350_e115873_d_n4, assign76350_e115873_d_n5, assign76350_e115873_d_n6, assign76350_e115873_d_n7, assign76350_e115873_d_n8, assign76350_e115873_d_n9, assign76350_e115873_d_n10, assign76350_e115873_d_n11, assign76350_e115873_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 == 0.0)) {
        let (assign76350_e115871, assign76350_e115871_d_n0, assign76350_e115871_d_n2, assign76350_e115871_d_n4, assign76350_e115871_d_n5, assign76350_e115871_d_n6, assign76350_e115871_d_n7, assign76350_e115871_d_n8, assign76350_e115871_d_n9, assign76350_e115871_d_n10, assign76350_e115871_d_n11, assign76350_e115871_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign76350_e115868: f64 = 2.0;
                let assign76350_e115869: f64 = (1.0 / assign76350_e115868);
                let assign76350_e115870: f64 = (locals.var_dnm).powf(assign76350_e115869);
                (assign76350_e115870, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn0)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn2)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn4)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn5)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn6)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn7)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn8)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn9)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn10)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn11)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn14)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign76350_e115871, assign76350_e115871_d_n0, assign76350_e115871_d_n2, assign76350_e115871_d_n4, assign76350_e115871_d_n5, assign76350_e115871_d_n6, assign76350_e115871_d_n7, assign76350_e115871_d_n8, assign76350_e115871_d_n9, assign76350_e115871_d_n10, assign76350_e115871_d_n11, assign76350_e115871_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76350_e115873;
        locals.var_dnm_dn0 = assign76350_e115873_d_n0;
        locals.var_dnm_dn2 = assign76350_e115873_d_n2;
        locals.var_dnm_dn4 = assign76350_e115873_d_n4;
        locals.var_dnm_dn5 = assign76350_e115873_d_n5;
        locals.var_dnm_dn6 = assign76350_e115873_d_n6;
        locals.var_dnm_dn7 = assign76350_e115873_d_n7;
        locals.var_dnm_dn8 = assign76350_e115873_d_n8;
        locals.var_dnm_dn9 = assign76350_e115873_d_n9;
        locals.var_dnm_dn10 = assign76350_e115873_d_n10;
        locals.var_dnm_dn11 = assign76350_e115873_d_n11;
        locals.var_dnm_dn14 = assign76350_e115873_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76360_e115883, assign76360_e115883_d_n0, assign76360_e115883_d_n2, assign76360_e115883_d_n4, assign76360_e115883_d_n5, assign76360_e115883_d_n6, assign76360_e115883_d_n7, assign76360_e115883_d_n8, assign76360_e115883_d_n9, assign76360_e115883_d_n10, assign76360_e115883_d_n11, assign76360_e115883_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76360_e115881: f64 = (1.0 / locals.var_dnm);
        (assign76360_e115881, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76360_e115883;
        locals.var_dnm_dn0 = assign76360_e115883_d_n0;
        locals.var_dnm_dn2 = assign76360_e115883_d_n2;
        locals.var_dnm_dn4 = assign76360_e115883_d_n4;
        locals.var_dnm_dn5 = assign76360_e115883_d_n5;
        locals.var_dnm_dn6 = assign76360_e115883_d_n6;
        locals.var_dnm_dn7 = assign76360_e115883_d_n7;
        locals.var_dnm_dn8 = assign76360_e115883_d_n8;
        locals.var_dnm_dn9 = assign76360_e115883_d_n9;
        locals.var_dnm_dn10 = assign76360_e115883_d_n10;
        locals.var_dnm_dn11 = assign76360_e115883_d_n11;
        locals.var_dnm_dn14 = assign76360_e115883_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76370_e115895, assign76370_e115895_d_n0, assign76370_e115895_d_n2, assign76370_e115895_d_n4, assign76370_e115895_d_n5, assign76370_e115895_d_n6, assign76370_e115895_d_n7, assign76370_e115895_d_n8, assign76370_e115895_d_n9, assign76370_e115895_d_n10, assign76370_e115895_d_n11, assign76370_e115895_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76370_e115891: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign76370_e115893: f64 = (assign76370_e115891 * locals.var_dnm);
        (assign76370_e115893, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign76370_e115895;
        locals.var_tmf0_dn0 = assign76370_e115895_d_n0;
        locals.var_tmf0_dn2 = assign76370_e115895_d_n2;
        locals.var_tmf0_dn4 = assign76370_e115895_d_n4;
        locals.var_tmf0_dn5 = assign76370_e115895_d_n5;
        locals.var_tmf0_dn6 = assign76370_e115895_d_n6;
        locals.var_tmf0_dn7 = assign76370_e115895_d_n7;
        locals.var_tmf0_dn8 = assign76370_e115895_d_n8;
        locals.var_tmf0_dn9 = assign76370_e115895_d_n9;
        locals.var_tmf0_dn10 = assign76370_e115895_d_n10;
        locals.var_tmf0_dn11 = assign76370_e115895_d_n11;
        locals.var_tmf0_dn14 = assign76370_e115895_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign76380_e115909, assign76380_e115909_d_n0, assign76380_e115909_d_n2, assign76380_e115909_d_n4, assign76380_e115909_d_n5, assign76380_e115909_d_n6, assign76380_e115909_d_n7, assign76380_e115909_d_n8, assign76380_e115909_d_n9, assign76380_e115909_d_n10, assign76380_e115909_d_n11, assign76380_e115909_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76380_e115903: f64 = (locals.var_t1 * locals.var_xmp);
        let assign76380_e115905: f64 = (assign76380_e115903 * locals.var_dnm);
        let assign76380_e115907: f64 = (assign76380_e115905 / locals.var_arg);
        (assign76380_e115907, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn0)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn2)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn4)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn5)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn6)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn7)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn8)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn9)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn10)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn11)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn14)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76380_e115909;
        locals.var_t0_dn0 = assign76380_e115909_d_n0;
        locals.var_t0_dn2 = assign76380_e115909_d_n2;
        locals.var_t0_dn4 = assign76380_e115909_d_n4;
        locals.var_t0_dn5 = assign76380_e115909_d_n5;
        locals.var_t0_dn6 = assign76380_e115909_d_n6;
        locals.var_t0_dn7 = assign76380_e115909_d_n7;
        locals.var_t0_dn8 = assign76380_e115909_d_n8;
        locals.var_t0_dn9 = assign76380_e115909_d_n9;
        locals.var_t0_dn10 = assign76380_e115909_d_n10;
        locals.var_t0_dn11 = assign76380_e115909_d_n11;
        locals.var_t0_dn14 = assign76380_e115909_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76390_e115921, assign76390_e115921_d_n0, assign76390_e115921_d_n2, assign76390_e115921_d_n4, assign76390_e115921_d_n5, assign76390_e115921_d_n6, assign76390_e115921_d_n7, assign76390_e115921_d_n8, assign76390_e115921_d_n9, assign76390_e115921_d_n10, assign76390_e115921_d_n11, assign76390_e115921_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76390_e115917: f64 = (-locals.var_t1);
        let assign76390_e115919: f64 = (assign76390_e115917 + locals.var_tmf0);
        (assign76390_e115919, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76390_e115921;
        locals.var_t1_dn0 = assign76390_e115921_d_n0;
        locals.var_t1_dn2 = assign76390_e115921_d_n2;
        locals.var_t1_dn4 = assign76390_e115921_d_n4;
        locals.var_t1_dn5 = assign76390_e115921_d_n5;
        locals.var_t1_dn6 = assign76390_e115921_d_n6;
        locals.var_t1_dn7 = assign76390_e115921_d_n7;
        locals.var_t1_dn8 = assign76390_e115921_d_n8;
        locals.var_t1_dn9 = assign76390_e115921_d_n9;
        locals.var_t1_dn10 = assign76390_e115921_d_n10;
        locals.var_t1_dn11 = assign76390_e115921_d_n11;
        locals.var_t1_dn14 = assign76390_e115921_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76400_e115929, assign76400_e115929_d_n0, assign76400_e115929_d_n2, assign76400_e115929_d_n4, assign76400_e115929_d_n5, assign76400_e115929_d_n6, assign76400_e115929_d_n7, assign76400_e115929_d_n8, assign76400_e115929_d_n9, assign76400_e115929_d_n10, assign76400_e115929_d_n11, assign76400_e115929_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76400_e115929;
        locals.var_t0_dn0 = assign76400_e115929_d_n0;
        locals.var_t0_dn2 = assign76400_e115929_d_n2;
        locals.var_t0_dn4 = assign76400_e115929_d_n4;
        locals.var_t0_dn5 = assign76400_e115929_d_n5;
        locals.var_t0_dn6 = assign76400_e115929_d_n6;
        locals.var_t0_dn7 = assign76400_e115929_d_n7;
        locals.var_t0_dn8 = assign76400_e115929_d_n8;
        locals.var_t0_dn9 = assign76400_e115929_d_n9;
        locals.var_t0_dn10 = assign76400_e115929_d_n10;
        locals.var_t0_dn11 = assign76400_e115929_d_n11;
        locals.var_t0_dn14 = assign76400_e115929_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76410_e115940, assign76410_e115940_d_n0, assign76410_e115940_d_n2, assign76410_e115940_d_n4, assign76410_e115940_d_n5, assign76410_e115940_d_n6, assign76410_e115940_d_n7, assign76410_e115940_d_n8, assign76410_e115940_d_n9, assign76410_e115940_d_n10, assign76410_e115940_d_n11, assign76410_e115940_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 == 0.0)) {
        let assign76410_e115938: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign76410_e115938, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76410_e115940;
        locals.var_t1_dn0 = assign76410_e115940_d_n0;
        locals.var_t1_dn2 = assign76410_e115940_d_n2;
        locals.var_t1_dn4 = assign76410_e115940_d_n4;
        locals.var_t1_dn5 = assign76410_e115940_d_n5;
        locals.var_t1_dn6 = assign76410_e115940_d_n6;
        locals.var_t1_dn7 = assign76410_e115940_d_n7;
        locals.var_t1_dn8 = assign76410_e115940_d_n8;
        locals.var_t1_dn9 = assign76410_e115940_d_n9;
        locals.var_t1_dn10 = assign76410_e115940_d_n10;
        locals.var_t1_dn11 = assign76410_e115940_d_n11;
        locals.var_t1_dn14 = assign76410_e115940_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76420_e115949, assign76420_e115949_d_n0, assign76420_e115949_d_n2, assign76420_e115949_d_n4, assign76420_e115949_d_n5, assign76420_e115949_d_n6, assign76420_e115949_d_n7, assign76420_e115949_d_n8, assign76420_e115949_d_n9, assign76420_e115949_d_n10, assign76420_e115949_d_n11, assign76420_e115949_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76420_e115949;
        locals.var_t0_dn0 = assign76420_e115949_d_n0;
        locals.var_t0_dn2 = assign76420_e115949_d_n2;
        locals.var_t0_dn4 = assign76420_e115949_d_n4;
        locals.var_t0_dn5 = assign76420_e115949_d_n5;
        locals.var_t0_dn6 = assign76420_e115949_d_n6;
        locals.var_t0_dn7 = assign76420_e115949_d_n7;
        locals.var_t0_dn8 = assign76420_e115949_d_n8;
        locals.var_t0_dn9 = assign76420_e115949_d_n9;
        locals.var_t0_dn10 = assign76420_e115949_d_n10;
        locals.var_t0_dn11 = assign76420_e115949_d_n11;
        locals.var_t0_dn14 = assign76420_e115949_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76430_e115957, assign76430_e115957_d_n0, assign76430_e115957_d_n2, assign76430_e115957_d_n4, assign76430_e115957_d_n5, assign76430_e115957_d_n6, assign76430_e115957_d_n7, assign76430_e115957_d_n8, assign76430_e115957_d_n9, assign76430_e115957_d_n10, assign76430_e115957_d_n11, assign76430_e115957_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76430_e115955: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign76430_e115955, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign76430_e115957;
        locals.var_vxbgmtcl_dn0 = assign76430_e115957_d_n0;
        locals.var_vxbgmtcl_dn2 = assign76430_e115957_d_n2;
        locals.var_vxbgmtcl_dn4 = assign76430_e115957_d_n4;
        locals.var_vxbgmtcl_dn5 = assign76430_e115957_d_n5;
        locals.var_vxbgmtcl_dn6 = assign76430_e115957_d_n6;
        locals.var_vxbgmtcl_dn7 = assign76430_e115957_d_n7;
        locals.var_vxbgmtcl_dn8 = assign76430_e115957_d_n8;
        locals.var_vxbgmtcl_dn9 = assign76430_e115957_d_n9;
        locals.var_vxbgmtcl_dn10 = assign76430_e115957_d_n10;
        locals.var_vxbgmtcl_dn11 = assign76430_e115957_d_n11;
        locals.var_vxbgmtcl_dn14 = assign76430_e115957_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign76440_e115968, assign76440_e115968_d_n0, assign76440_e115968_d_n2, assign76440_e115968_d_n4, assign76440_e115968_d_n5, assign76440_e115968_d_n6, assign76440_e115968_d_n7, assign76440_e115968_d_n8, assign76440_e115968_d_n9, assign76440_e115968_d_n10, assign76440_e115968_d_n11, assign76440_e115968_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76440_e115962: f64 = (-locals.var_vxbgmtcl);
        let assign76440_e115965: f64 = (10.0 * 2.220446049250313e-16);
        let assign76440_e115966: f64 = (assign76440_e115962 + assign76440_e115965);
        (assign76440_e115966, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign76440_e115968;
        locals.var_vgb_fb_ld_dn0 = assign76440_e115968_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign76440_e115968_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign76440_e115968_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign76440_e115968_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign76440_e115968_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign76440_e115968_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign76440_e115968_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign76440_e115968_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign76440_e115968_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign76440_e115968_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign76440_e115968_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign76450_e115971: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1790 = assign76450_e115971;
        locals.var_guard1790_rv = 0.0;

        let (assign76470_e115992, assign76470_e115992_d_n0, assign76470_e115992_d_n2, assign76470_e115992_d_n4, assign76470_e115992_d_n5, assign76470_e115992_d_n6, assign76470_e115992_d_n7, assign76470_e115992_d_n8, assign76470_e115992_d_n9, assign76470_e115992_d_n10, assign76470_e115992_d_n11, assign76470_e115992_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76470_e115984: f64 = (2.0 * locals.var_beta_inv);
        let assign76470_e115986: f64 = (-locals.var_vgs_min);
        let assign76470_e115988: f64 = (assign76470_e115986 / locals.var_fac1);
        let assign76470_e115989: f64 = (assign76470_e115988).ln();
        let assign76470_e115990: f64 = (assign76470_e115984 * assign76470_e115989);
        (assign76470_e115990, (((2.0 * locals.var_beta_inv_dn0) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn2) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn4) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn5) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn6) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn7) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn8) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn9) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn10) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn11) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn14) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign76470_e115992;
        locals.var_ps0_min_dn0 = assign76470_e115992_d_n0;
        locals.var_ps0_min_dn2 = assign76470_e115992_d_n2;
        locals.var_ps0_min_dn4 = assign76470_e115992_d_n4;
        locals.var_ps0_min_dn5 = assign76470_e115992_d_n5;
        locals.var_ps0_min_dn6 = assign76470_e115992_d_n6;
        locals.var_ps0_min_dn7 = assign76470_e115992_d_n7;
        locals.var_ps0_min_dn8 = assign76470_e115992_d_n8;
        locals.var_ps0_min_dn9 = assign76470_e115992_d_n9;
        locals.var_ps0_min_dn10 = assign76470_e115992_d_n10;
        locals.var_ps0_min_dn11 = assign76470_e115992_d_n11;
        locals.var_ps0_min_dn14 = assign76470_e115992_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign76480_e116002, assign76480_e116002_d_n0, assign76480_e116002_d_n2, assign76480_e116002_d_n4, assign76480_e116002_d_n5, assign76480_e116002_d_n6, assign76480_e116002_d_n7, assign76480_e116002_d_n8, assign76480_e116002_d_n9, assign76480_e116002_d_n10, assign76480_e116002_d_n11, assign76480_e116002_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76480_e115999: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76480_e116000: f64 = (locals.var_beta * assign76480_e115999);
        (assign76480_e116000, ((locals.var_beta_dn0 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76480_e116002;
        locals.var_tx_dn0 = assign76480_e116002_d_n0;
        locals.var_tx_dn2 = assign76480_e116002_d_n2;
        locals.var_tx_dn4 = assign76480_e116002_d_n4;
        locals.var_tx_dn5 = assign76480_e116002_d_n5;
        locals.var_tx_dn6 = assign76480_e116002_d_n6;
        locals.var_tx_dn7 = assign76480_e116002_d_n7;
        locals.var_tx_dn8 = assign76480_e116002_d_n8;
        locals.var_tx_dn9 = assign76480_e116002_d_n9;
        locals.var_tx_dn10 = assign76480_e116002_d_n10;
        locals.var_tx_dn11 = assign76480_e116002_d_n11;
        locals.var_tx_dn14 = assign76480_e116002_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_289(
        locals: &mut StampLocals,
    ) {
        let (assign76490_e116012, assign76490_e116012_d_n0, assign76490_e116012_d_n2, assign76490_e116012_d_n4, assign76490_e116012_d_n5, assign76490_e116012_d_n6, assign76490_e116012_d_n7, assign76490_e116012_d_n8, assign76490_e116012_d_n9, assign76490_e116012_d_n10, assign76490_e116012_d_n11, assign76490_e116012_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76490_e116009: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign76490_e116010: f64 = (1.0 / assign76490_e116009);
        (assign76490_e116010, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign76490_e116009 * assign76490_e116009))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76490_e116012;
        locals.var_t1_dn0 = assign76490_e116012_d_n0;
        locals.var_t1_dn2 = assign76490_e116012_d_n2;
        locals.var_t1_dn4 = assign76490_e116012_d_n4;
        locals.var_t1_dn5 = assign76490_e116012_d_n5;
        locals.var_t1_dn6 = assign76490_e116012_d_n6;
        locals.var_t1_dn7 = assign76490_e116012_d_n7;
        locals.var_t1_dn8 = assign76490_e116012_d_n8;
        locals.var_t1_dn9 = assign76490_e116012_d_n9;
        locals.var_t1_dn10 = assign76490_e116012_d_n10;
        locals.var_t1_dn11 = assign76490_e116012_d_n11;
        locals.var_t1_dn14 = assign76490_e116012_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76500_e116020, assign76500_e116020_d_n0, assign76500_e116020_d_n2, assign76500_e116020_d_n4, assign76500_e116020_d_n5, assign76500_e116020_d_n6, assign76500_e116020_d_n7, assign76500_e116020_d_n8, assign76500_e116020_d_n9, assign76500_e116020_d_n10, assign76500_e116020_d_n11, assign76500_e116020_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76500_e116018: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign76500_e116018, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76500_e116020;
        locals.var_ty_dn0 = assign76500_e116020_d_n0;
        locals.var_ty_dn2 = assign76500_e116020_d_n2;
        locals.var_ty_dn4 = assign76500_e116020_d_n4;
        locals.var_ty_dn5 = assign76500_e116020_d_n5;
        locals.var_ty_dn6 = assign76500_e116020_d_n6;
        locals.var_ty_dn7 = assign76500_e116020_d_n7;
        locals.var_ty_dn8 = assign76500_e116020_d_n8;
        locals.var_ty_dn9 = assign76500_e116020_d_n9;
        locals.var_ty_dn10 = assign76500_e116020_d_n10;
        locals.var_ty_dn11 = assign76500_e116020_d_n11;
        locals.var_ty_dn14 = assign76500_e116020_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76510_e116032, assign76510_e116032_d_n0, assign76510_e116032_d_n2, assign76510_e116032_d_n4, assign76510_e116032_d_n5, assign76510_e116032_d_n6, assign76510_e116032_d_n7, assign76510_e116032_d_n8, assign76510_e116032_d_n9, assign76510_e116032_d_n10, assign76510_e116032_d_n11, assign76510_e116032_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76510_e116027: f64 = (3.0 * 1.414213562373095);
        let assign76510_e116029: f64 = (assign76510_e116027 * locals.var_ty);
        let assign76510_e116030: f64 = (2.0 + assign76510_e116029);
        (assign76510_e116030, (assign76510_e116027 * locals.var_ty_dn0), (assign76510_e116027 * locals.var_ty_dn2), (assign76510_e116027 * locals.var_ty_dn4), (assign76510_e116027 * locals.var_ty_dn5), (assign76510_e116027 * locals.var_ty_dn6), (assign76510_e116027 * locals.var_ty_dn7), (assign76510_e116027 * locals.var_ty_dn8), (assign76510_e116027 * locals.var_ty_dn9), (assign76510_e116027 * locals.var_ty_dn10), (assign76510_e116027 * locals.var_ty_dn11), (assign76510_e116027 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign76510_e116032;
        locals.var_ac41_dn0 = assign76510_e116032_d_n0;
        locals.var_ac41_dn2 = assign76510_e116032_d_n2;
        locals.var_ac41_dn4 = assign76510_e116032_d_n4;
        locals.var_ac41_dn5 = assign76510_e116032_d_n5;
        locals.var_ac41_dn6 = assign76510_e116032_d_n6;
        locals.var_ac41_dn7 = assign76510_e116032_d_n7;
        locals.var_ac41_dn8 = assign76510_e116032_d_n8;
        locals.var_ac41_dn9 = assign76510_e116032_d_n9;
        locals.var_ac41_dn10 = assign76510_e116032_d_n10;
        locals.var_ac41_dn11 = assign76510_e116032_d_n11;
        locals.var_ac41_dn14 = assign76510_e116032_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign76520_e116044, assign76520_e116044_d_n0, assign76520_e116044_d_n2, assign76520_e116044_d_n4, assign76520_e116044_d_n5, assign76520_e116044_d_n6, assign76520_e116044_d_n7, assign76520_e116044_d_n8, assign76520_e116044_d_n9, assign76520_e116044_d_n10, assign76520_e116044_d_n11, assign76520_e116044_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76520_e116038: f64 = (8.0 * locals.var_ac41);
        let assign76520_e116040: f64 = (assign76520_e116038 * locals.var_ac41);
        let assign76520_e116042: f64 = (assign76520_e116040 * locals.var_ac41);
        (assign76520_e116042, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign76520_e116044;
        locals.var_ac4_dn0 = assign76520_e116044_d_n0;
        locals.var_ac4_dn2 = assign76520_e116044_d_n2;
        locals.var_ac4_dn4 = assign76520_e116044_d_n4;
        locals.var_ac4_dn5 = assign76520_e116044_d_n5;
        locals.var_ac4_dn6 = assign76520_e116044_d_n6;
        locals.var_ac4_dn7 = assign76520_e116044_d_n7;
        locals.var_ac4_dn8 = assign76520_e116044_d_n8;
        locals.var_ac4_dn9 = assign76520_e116044_d_n9;
        locals.var_ac4_dn10 = assign76520_e116044_d_n10;
        locals.var_ac4_dn11 = assign76520_e116044_d_n11;
        locals.var_ac4_dn14 = assign76520_e116044_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign76530_e116060, assign76530_e116060_d_n0, assign76530_e116060_d_n2, assign76530_e116060_d_n4, assign76530_e116060_d_n5, assign76530_e116060_d_n6, assign76530_e116060_d_n7, assign76530_e116060_d_n8, assign76530_e116060_d_n9, assign76530_e116060_d_n10, assign76530_e116060_d_n11, assign76530_e116060_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76530_e116050: f64 = (7.0 * 1.414213562373095);
        let assign76530_e116053: f64 = (9.0 * locals.var_ty);
        let assign76530_e116056: f64 = (locals.var_tx - 2.0);
        let assign76530_e116057: f64 = (assign76530_e116053 * assign76530_e116056);
        let assign76530_e116058: f64 = (assign76530_e116050 - assign76530_e116057);
        (assign76530_e116058, (-(((9.0 * locals.var_ty_dn0) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign76530_e116060;
        locals.var_ac31_dn0 = assign76530_e116060_d_n0;
        locals.var_ac31_dn2 = assign76530_e116060_d_n2;
        locals.var_ac31_dn4 = assign76530_e116060_d_n4;
        locals.var_ac31_dn5 = assign76530_e116060_d_n5;
        locals.var_ac31_dn6 = assign76530_e116060_d_n6;
        locals.var_ac31_dn7 = assign76530_e116060_d_n7;
        locals.var_ac31_dn8 = assign76530_e116060_d_n8;
        locals.var_ac31_dn9 = assign76530_e116060_d_n9;
        locals.var_ac31_dn10 = assign76530_e116060_d_n10;
        locals.var_ac31_dn11 = assign76530_e116060_d_n11;
        locals.var_ac31_dn14 = assign76530_e116060_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign76540_e116068, assign76540_e116068_d_n0, assign76540_e116068_d_n2, assign76540_e116068_d_n4, assign76540_e116068_d_n5, assign76540_e116068_d_n6, assign76540_e116068_d_n7, assign76540_e116068_d_n8, assign76540_e116068_d_n9, assign76540_e116068_d_n10, assign76540_e116068_d_n11, assign76540_e116068_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76540_e116066: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign76540_e116066, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign76540_e116068;
        locals.var_ac3_dn0 = assign76540_e116068_d_n0;
        locals.var_ac3_dn2 = assign76540_e116068_d_n2;
        locals.var_ac3_dn4 = assign76540_e116068_d_n4;
        locals.var_ac3_dn5 = assign76540_e116068_d_n5;
        locals.var_ac3_dn6 = assign76540_e116068_d_n6;
        locals.var_ac3_dn7 = assign76540_e116068_d_n7;
        locals.var_ac3_dn8 = assign76540_e116068_d_n8;
        locals.var_ac3_dn9 = assign76540_e116068_d_n9;
        locals.var_ac3_dn10 = assign76540_e116068_d_n10;
        locals.var_ac3_dn11 = assign76540_e116068_d_n11;
        locals.var_ac3_dn14 = assign76540_e116068_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign76550_e116072: f64 = (locals.var_ac3 * 1e-8);
        let assign76550_e116073: f64 = if locals.var_ac4 < assign76550_e116072 { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign76550_e116073;
        locals.var_guard1791_rv = 0.0;

        let (assign76570_e116094, assign76570_e116094_d_n0, assign76570_e116094_d_n2, assign76570_e116094_d_n4, assign76570_e116094_d_n5, assign76570_e116094_d_n6, assign76570_e116094_d_n7, assign76570_e116094_d_n8, assign76570_e116094_d_n9, assign76570_e116094_d_n10, assign76570_e116094_d_n11, assign76570_e116094_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76570_e116090: f64 = (0.5 * locals.var_ac4);
        let assign76570_e116092: f64 = (assign76570_e116090 / locals.var_ac31);
        (assign76570_e116092, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76570_e116094;
        locals.var_ac1_dn0 = assign76570_e116094_d_n0;
        locals.var_ac1_dn2 = assign76570_e116094_d_n2;
        locals.var_ac1_dn4 = assign76570_e116094_d_n4;
        locals.var_ac1_dn5 = assign76570_e116094_d_n5;
        locals.var_ac1_dn6 = assign76570_e116094_d_n6;
        locals.var_ac1_dn7 = assign76570_e116094_d_n7;
        locals.var_ac1_dn8 = assign76570_e116094_d_n8;
        locals.var_ac1_dn9 = assign76570_e116094_d_n9;
        locals.var_ac1_dn10 = assign76570_e116094_d_n10;
        locals.var_ac1_dn11 = assign76570_e116094_d_n11;
        locals.var_ac1_dn14 = assign76570_e116094_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign76580_e116106, assign76580_e116106_d_n0, assign76580_e116106_d_n2, assign76580_e116106_d_n4, assign76580_e116106_d_n5, assign76580_e116106_d_n6, assign76580_e116106_d_n7, assign76580_e116106_d_n8, assign76580_e116106_d_n9, assign76580_e116106_d_n10, assign76580_e116106_d_n11, assign76580_e116106_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76580_e116103: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign76580_e116104: f64 = (assign76580_e116103).sqrt();
        (assign76580_e116104, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign76580_e116104)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign76580_e116106;
        locals.var_ac2_dn0 = assign76580_e116106_d_n0;
        locals.var_ac2_dn2 = assign76580_e116106_d_n2;
        locals.var_ac2_dn4 = assign76580_e116106_d_n4;
        locals.var_ac2_dn5 = assign76580_e116106_d_n5;
        locals.var_ac2_dn6 = assign76580_e116106_d_n6;
        locals.var_ac2_dn7 = assign76580_e116106_d_n7;
        locals.var_ac2_dn8 = assign76580_e116106_d_n8;
        locals.var_ac2_dn9 = assign76580_e116106_d_n9;
        locals.var_ac2_dn10 = assign76580_e116106_d_n10;
        locals.var_ac2_dn11 = assign76580_e116106_d_n11;
        locals.var_ac2_dn14 = assign76580_e116106_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign76590_e116118, assign76590_e116118_d_n0, assign76590_e116118_d_n2, assign76590_e116118_d_n4, assign76590_e116118_d_n5, assign76590_e116118_d_n6, assign76590_e116118_d_n7, assign76590_e116118_d_n8, assign76590_e116118_d_n9, assign76590_e116118_d_n10, assign76590_e116118_d_n11, assign76590_e116118_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76590_e116114: f64 = (-locals.var_ac31);
        let assign76590_e116116: f64 = (assign76590_e116114 + locals.var_ac2);
        (assign76590_e116116, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76590_e116118;
        locals.var_ac1_dn0 = assign76590_e116118_d_n0;
        locals.var_ac1_dn2 = assign76590_e116118_d_n2;
        locals.var_ac1_dn4 = assign76590_e116118_d_n4;
        locals.var_ac1_dn5 = assign76590_e116118_d_n5;
        locals.var_ac1_dn6 = assign76590_e116118_d_n6;
        locals.var_ac1_dn7 = assign76590_e116118_d_n7;
        locals.var_ac1_dn8 = assign76590_e116118_d_n8;
        locals.var_ac1_dn9 = assign76590_e116118_d_n9;
        locals.var_ac1_dn10 = assign76590_e116118_d_n10;
        locals.var_ac1_dn11 = assign76590_e116118_d_n11;
        locals.var_ac1_dn14 = assign76590_e116118_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign76600_e116126, assign76600_e116126_d_n0, assign76600_e116126_d_n2, assign76600_e116126_d_n4, assign76600_e116126_d_n5, assign76600_e116126_d_n6, assign76600_e116126_d_n7, assign76600_e116126_d_n8, assign76600_e116126_d_n9, assign76600_e116126_d_n10, assign76600_e116126_d_n11, assign76600_e116126_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76600_e116124: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign76600_e116124, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign76600_e116126;
        locals.var_acd_dn0 = assign76600_e116126_d_n0;
        locals.var_acd_dn2 = assign76600_e116126_d_n2;
        locals.var_acd_dn4 = assign76600_e116126_d_n4;
        locals.var_acd_dn5 = assign76600_e116126_d_n5;
        locals.var_acd_dn6 = assign76600_e116126_d_n6;
        locals.var_acd_dn7 = assign76600_e116126_d_n7;
        locals.var_acd_dn8 = assign76600_e116126_d_n8;
        locals.var_acd_dn9 = assign76600_e116126_d_n9;
        locals.var_acd_dn10 = assign76600_e116126_d_n10;
        locals.var_acd_dn11 = assign76600_e116126_d_n11;
        locals.var_acd_dn14 = assign76600_e116126_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign76610_e116149, assign76610_e116149_d_n0, assign76610_e116149_d_n2, assign76610_e116149_d_n4, assign76610_e116149_d_n5, assign76610_e116149_d_n6, assign76610_e116149_d_n7, assign76610_e116149_d_n8, assign76610_e116149_d_n9, assign76610_e116149_d_n10, assign76610_e116149_d_n11, assign76610_e116149_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76610_e116131: f64 = (-4.0);
        let assign76610_e116133: f64 = (assign76610_e116131 * 1.414213562373095);
        let assign76610_e116136: f64 = (12.0 * locals.var_ty);
        let assign76610_e116137: f64 = (assign76610_e116133 - assign76610_e116136);
        let assign76610_e116140: f64 = (2.0 * locals.var_acd);
        let assign76610_e116141: f64 = (assign76610_e116137 + assign76610_e116140);
        let assign76610_e116144: f64 = (1.414213562373095 * locals.var_acd);
        let assign76610_e116146: f64 = (assign76610_e116144 * locals.var_acd);
        let assign76610_e116147: f64 = (assign76610_e116141 + assign76610_e116146);
        (assign76610_e116147, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign76610_e116149;
        locals.var_acn_dn0 = assign76610_e116149_d_n0;
        locals.var_acn_dn2 = assign76610_e116149_d_n2;
        locals.var_acn_dn4 = assign76610_e116149_d_n4;
        locals.var_acn_dn5 = assign76610_e116149_d_n5;
        locals.var_acn_dn6 = assign76610_e116149_d_n6;
        locals.var_acn_dn7 = assign76610_e116149_d_n7;
        locals.var_acn_dn8 = assign76610_e116149_d_n8;
        locals.var_acn_dn9 = assign76610_e116149_d_n9;
        locals.var_acn_dn10 = assign76610_e116149_d_n10;
        locals.var_acn_dn11 = assign76610_e116149_d_n11;
        locals.var_acn_dn14 = assign76610_e116149_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign76620_e116157, assign76620_e116157_d_n0, assign76620_e116157_d_n2, assign76620_e116157_d_n4, assign76620_e116157_d_n5, assign76620_e116157_d_n6, assign76620_e116157_d_n7, assign76620_e116157_d_n8, assign76620_e116157_d_n9, assign76620_e116157_d_n10, assign76620_e116157_d_n11, assign76620_e116157_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76620_e116155: f64 = (locals.var_acn / locals.var_acd);
        (assign76620_e116155, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76620_e116157;
        locals.var_chi_dn0 = assign76620_e116157_d_n0;
        locals.var_chi_dn2 = assign76620_e116157_d_n2;
        locals.var_chi_dn4 = assign76620_e116157_d_n4;
        locals.var_chi_dn5 = assign76620_e116157_d_n5;
        locals.var_chi_dn6 = assign76620_e116157_d_n6;
        locals.var_chi_dn7 = assign76620_e116157_d_n7;
        locals.var_chi_dn8 = assign76620_e116157_d_n8;
        locals.var_chi_dn9 = assign76620_e116157_d_n9;
        locals.var_chi_dn10 = assign76620_e116157_d_n10;
        locals.var_chi_dn11 = assign76620_e116157_d_n11;
        locals.var_chi_dn14 = assign76620_e116157_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76630_e116165, assign76630_e116165_d_n0, assign76630_e116165_d_n2, assign76630_e116165_d_n4, assign76630_e116165_d_n5, assign76630_e116165_d_n6, assign76630_e116165_d_n7, assign76630_e116165_d_n8, assign76630_e116165_d_n9, assign76630_e116165_d_n10, assign76630_e116165_d_n11, assign76630_e116165_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76630_e116163: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign76630_e116163, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76630_e116165;
        locals.var_t1_dn0 = assign76630_e116165_d_n0;
        locals.var_t1_dn2 = assign76630_e116165_d_n2;
        locals.var_t1_dn4 = assign76630_e116165_d_n4;
        locals.var_t1_dn5 = assign76630_e116165_d_n5;
        locals.var_t1_dn6 = assign76630_e116165_d_n6;
        locals.var_t1_dn7 = assign76630_e116165_d_n7;
        locals.var_t1_dn8 = assign76630_e116165_d_n8;
        locals.var_t1_dn9 = assign76630_e116165_d_n9;
        locals.var_t1_dn10 = assign76630_e116165_d_n10;
        locals.var_t1_dn11 = assign76630_e116165_d_n11;
        locals.var_t1_dn14 = assign76630_e116165_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76640_e116173, assign76640_e116173_d_n0, assign76640_e116173_d_n2, assign76640_e116173_d_n4, assign76640_e116173_d_n5, assign76640_e116173_d_n6, assign76640_e116173_d_n7, assign76640_e116173_d_n8, assign76640_e116173_d_n9, assign76640_e116173_d_n10, assign76640_e116173_d_n11, assign76640_e116173_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76640_e116171: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign76640_e116171, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76640_e116173;
        locals.var_t2_dn0 = assign76640_e116173_d_n0;
        locals.var_t2_dn2 = assign76640_e116173_d_n2;
        locals.var_t2_dn4 = assign76640_e116173_d_n4;
        locals.var_t2_dn5 = assign76640_e116173_d_n5;
        locals.var_t2_dn6 = assign76640_e116173_d_n6;
        locals.var_t2_dn7 = assign76640_e116173_d_n7;
        locals.var_t2_dn8 = assign76640_e116173_d_n8;
        locals.var_t2_dn9 = assign76640_e116173_d_n9;
        locals.var_t2_dn10 = assign76640_e116173_d_n10;
        locals.var_t2_dn11 = assign76640_e116173_d_n11;
        locals.var_t2_dn14 = assign76640_e116173_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign76650_e116184, assign76650_e116184_d_n0, assign76650_e116184_d_n2, assign76650_e116184_d_n4, assign76650_e116184_d_n5, assign76650_e116184_d_n6, assign76650_e116184_d_n7, assign76650_e116184_d_n8, assign76650_e116184_d_n9, assign76650_e116184_d_n10, assign76650_e116184_d_n11, assign76650_e116184_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76650_e116180: f64 = (locals.var_t2 * locals.var_t2);
        let assign76650_e116181: f64 = (1.0 + assign76650_e116180);
        let assign76650_e116182: f64 = (assign76650_e116181).sqrt();
        (assign76650_e116182, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign76650_e116182)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign76650_e116184;
        locals.var_t3_dn0 = assign76650_e116184_d_n0;
        locals.var_t3_dn2 = assign76650_e116184_d_n2;
        locals.var_t3_dn4 = assign76650_e116184_d_n4;
        locals.var_t3_dn5 = assign76650_e116184_d_n5;
        locals.var_t3_dn6 = assign76650_e116184_d_n6;
        locals.var_t3_dn7 = assign76650_e116184_d_n7;
        locals.var_t3_dn8 = assign76650_e116184_d_n8;
        locals.var_t3_dn9 = assign76650_e116184_d_n9;
        locals.var_t3_dn10 = assign76650_e116184_d_n10;
        locals.var_t3_dn11 = assign76650_e116184_d_n11;
        locals.var_t3_dn14 = assign76650_e116184_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign76660_e116194, assign76660_e116194_d_n0, assign76660_e116194_d_n2, assign76660_e116194_d_n4, assign76660_e116194_d_n5, assign76660_e116194_d_n6, assign76660_e116194_d_n7, assign76660_e116194_d_n8, assign76660_e116194_d_n9, assign76660_e116194_d_n10, assign76660_e116194_d_n11, assign76660_e116194_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76660_e116190: f64 = (locals.var_t1 / locals.var_t3);
        let assign76660_e116192: f64 = (assign76660_e116190 - locals.var_vxbgmtcl);
        (assign76660_e116192, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign76660_e116194;
        locals.var_ps0ld_dn0 = assign76660_e116194_d_n0;
        locals.var_ps0ld_dn2 = assign76660_e116194_d_n2;
        locals.var_ps0ld_dn4 = assign76660_e116194_d_n4;
        locals.var_ps0ld_dn5 = assign76660_e116194_d_n5;
        locals.var_ps0ld_dn6 = assign76660_e116194_d_n6;
        locals.var_ps0ld_dn7 = assign76660_e116194_d_n7;
        locals.var_ps0ld_dn8 = assign76660_e116194_d_n8;
        locals.var_ps0ld_dn9 = assign76660_e116194_d_n9;
        locals.var_ps0ld_dn10 = assign76660_e116194_d_n10;
        locals.var_ps0ld_dn11 = assign76660_e116194_d_n11;
        locals.var_ps0ld_dn14 = assign76660_e116194_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign76670_e116202, assign76670_e116202_d_n0, assign76670_e116202_d_n2, assign76670_e116202_d_n4, assign76670_e116202_d_n5, assign76670_e116202_d_n6, assign76670_e116202_d_n7, assign76670_e116202_d_n8, assign76670_e116202_d_n9, assign76670_e116202_d_n10, assign76670_e116202_d_n11, assign76670_e116202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76670_e116200: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign76670_e116200, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76670_e116202;
        locals.var_t2_dn0 = assign76670_e116202_d_n0;
        locals.var_t2_dn2 = assign76670_e116202_d_n2;
        locals.var_t2_dn4 = assign76670_e116202_d_n4;
        locals.var_t2_dn5 = assign76670_e116202_d_n5;
        locals.var_t2_dn6 = assign76670_e116202_d_n6;
        locals.var_t2_dn7 = assign76670_e116202_d_n7;
        locals.var_t2_dn8 = assign76670_e116202_d_n8;
        locals.var_t2_dn9 = assign76670_e116202_d_n9;
        locals.var_t2_dn10 = assign76670_e116202_d_n10;
        locals.var_t2_dn11 = assign76670_e116202_d_n11;
        locals.var_t2_dn14 = assign76670_e116202_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign76680_e116210, assign76680_e116210_d_n0, assign76680_e116210_d_n2, assign76680_e116210_d_n4, assign76680_e116210_d_n5, assign76680_e116210_d_n6, assign76680_e116210_d_n7, assign76680_e116210_d_n8, assign76680_e116210_d_n9, assign76680_e116210_d_n10, assign76680_e116210_d_n11, assign76680_e116210_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76680_e116208: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign76680_e116208, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign76680_e116210;
        locals.var_qsuld_dn0 = assign76680_e116210_d_n0;
        locals.var_qsuld_dn2 = assign76680_e116210_d_n2;
        locals.var_qsuld_dn4 = assign76680_e116210_d_n4;
        locals.var_qsuld_dn5 = assign76680_e116210_d_n5;
        locals.var_qsuld_dn6 = assign76680_e116210_d_n6;
        locals.var_qsuld_dn7 = assign76680_e116210_d_n7;
        locals.var_qsuld_dn8 = assign76680_e116210_d_n8;
        locals.var_qsuld_dn9 = assign76680_e116210_d_n9;
        locals.var_qsuld_dn10 = assign76680_e116210_d_n10;
        locals.var_qsuld_dn11 = assign76680_e116210_d_n11;
        locals.var_qsuld_dn14 = assign76680_e116210_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign76690_e116216, assign76690_e116216_d_n0, assign76690_e116216_d_n2, assign76690_e116216_d_n4, assign76690_e116216_d_n5, assign76690_e116216_d_n6, assign76690_e116216_d_n7, assign76690_e116216_d_n8, assign76690_e116216_d_n9, assign76690_e116216_d_n10, assign76690_e116216_d_n11, assign76690_e116216_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign76690_e116216;
        locals.var_qbuld_dn0 = assign76690_e116216_d_n0;
        locals.var_qbuld_dn2 = assign76690_e116216_d_n2;
        locals.var_qbuld_dn4 = assign76690_e116216_d_n4;
        locals.var_qbuld_dn5 = assign76690_e116216_d_n5;
        locals.var_qbuld_dn6 = assign76690_e116216_d_n6;
        locals.var_qbuld_dn7 = assign76690_e116216_d_n7;
        locals.var_qbuld_dn8 = assign76690_e116216_d_n8;
        locals.var_qbuld_dn9 = assign76690_e116216_d_n9;
        locals.var_qbuld_dn10 = assign76690_e116216_d_n10;
        locals.var_qbuld_dn11 = assign76690_e116216_d_n11;
        locals.var_qbuld_dn14 = assign76690_e116216_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign76700_e116222, assign76700_e116222_d_n0, assign76700_e116222_d_n2, assign76700_e116222_d_n4, assign76700_e116222_d_n5, assign76700_e116222_d_n6, assign76700_e116222_d_n7, assign76700_e116222_d_n8, assign76700_e116222_d_n9, assign76700_e116222_d_n10, assign76700_e116222_d_n11, assign76700_e116222_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1773 = assign76700_e116222;
        locals.var_ps0ld_ini__blk1773_dn0 = assign76700_e116222_d_n0;
        locals.var_ps0ld_ini__blk1773_dn2 = assign76700_e116222_d_n2;
        locals.var_ps0ld_ini__blk1773_dn4 = assign76700_e116222_d_n4;
        locals.var_ps0ld_ini__blk1773_dn5 = assign76700_e116222_d_n5;
        locals.var_ps0ld_ini__blk1773_dn6 = assign76700_e116222_d_n6;
        locals.var_ps0ld_ini__blk1773_dn7 = assign76700_e116222_d_n7;
        locals.var_ps0ld_ini__blk1773_dn8 = assign76700_e116222_d_n8;
        locals.var_ps0ld_ini__blk1773_dn9 = assign76700_e116222_d_n9;
        locals.var_ps0ld_ini__blk1773_dn10 = assign76700_e116222_d_n10;
        locals.var_ps0ld_ini__blk1773_dn11 = assign76700_e116222_d_n11;
        locals.var_ps0ld_ini__blk1773_dn14 = assign76700_e116222_d_n14;
        locals.var_ps0ld_ini__blk1773_rv = 0.0;

        let assign76710_e116226: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76710_e116227: f64 = (locals.var_beta * assign76710_e116226);
        let assign76710_e116231: f64 = (10.0 * 2.220446049250313e-16);
        let assign76710_e116233: f64 = (assign76710_e116231 - 1.0);
        let assign76710_e116235: f64 = (assign76710_e116233 * locals.var_fac1p2);
        let assign76710_e116237: f64 = (assign76710_e116235 * locals.var_beta2);
        let assign76710_e116239: f64 = (assign76710_e116237 / 4.0);
        let assign76710_e116240: f64 = (1.0 + assign76710_e116239);
        let assign76710_e116241: f64 = if assign76710_e116227 < assign76710_e116240 { 1.0 } else { 0.0 };
        locals.var_guard1792 = assign76710_e116241;
        locals.var_guard1792_rv = 0.0;

        let (assign76720_e116256, assign76720_e116256_d_n0, assign76720_e116256_d_n2, assign76720_e116256_d_n4, assign76720_e116256_d_n5, assign76720_e116256_d_n6, assign76720_e116256_d_n7, assign76720_e116256_d_n8, assign76720_e116256_d_n9, assign76720_e116256_d_n10, assign76720_e116256_d_n11, assign76720_e116256_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign76720_e116251: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76720_e116253: f64 = (assign76720_e116251 / 2.0);
        let assign76720_e116254: f64 = (locals.var_vgpld + assign76720_e116253);
        (assign76720_e116254, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76720_e116256;
        locals.var_ps0_inia_dn0 = assign76720_e116256_d_n0;
        locals.var_ps0_inia_dn2 = assign76720_e116256_d_n2;
        locals.var_ps0_inia_dn4 = assign76720_e116256_d_n4;
        locals.var_ps0_inia_dn5 = assign76720_e116256_d_n5;
        locals.var_ps0_inia_dn6 = assign76720_e116256_d_n6;
        locals.var_ps0_inia_dn7 = assign76720_e116256_d_n7;
        locals.var_ps0_inia_dn8 = assign76720_e116256_d_n8;
        locals.var_ps0_inia_dn9 = assign76720_e116256_d_n9;
        locals.var_ps0_inia_dn10 = assign76720_e116256_d_n10;
        locals.var_ps0_inia_dn11 = assign76720_e116256_d_n11;
        locals.var_ps0_inia_dn14 = assign76720_e116256_d_n14;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_290(
        locals: &mut StampLocals,
    ) {
        let (assign76730_e116280, assign76730_e116280_d_n0, assign76730_e116280_d_n2, assign76730_e116280_d_n4, assign76730_e116280_d_n5, assign76730_e116280_d_n6, assign76730_e116280_d_n7, assign76730_e116280_d_n8, assign76730_e116280_d_n9, assign76730_e116280_d_n10, assign76730_e116280_d_n11, assign76730_e116280_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 == 0.0)) {
        let assign76730_e116269: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76730_e116270: f64 = (locals.var_beta * assign76730_e116269);
        let assign76730_e116272: f64 = (assign76730_e116270 - 1.0);
        let assign76730_e116273: f64 = (4.0 * assign76730_e116272);
        let assign76730_e116276: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76730_e116277: f64 = (assign76730_e116273 / assign76730_e116276);
        let assign76730_e116278: f64 = (1.0 + assign76730_e116277);
        (assign76730_e116278, ((((4.0 * ((locals.var_beta_dn0 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn2 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn4 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn5 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn6 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn7 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn8 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn9 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn10 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn11 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn14 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76730_e116276 * assign76730_e116276)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76730_e116280;
        locals.var_tx_dn0 = assign76730_e116280_d_n0;
        locals.var_tx_dn2 = assign76730_e116280_d_n2;
        locals.var_tx_dn4 = assign76730_e116280_d_n4;
        locals.var_tx_dn5 = assign76730_e116280_d_n5;
        locals.var_tx_dn6 = assign76730_e116280_d_n6;
        locals.var_tx_dn7 = assign76730_e116280_d_n7;
        locals.var_tx_dn8 = assign76730_e116280_d_n8;
        locals.var_tx_dn9 = assign76730_e116280_d_n9;
        locals.var_tx_dn10 = assign76730_e116280_d_n10;
        locals.var_tx_dn11 = assign76730_e116280_d_n11;
        locals.var_tx_dn14 = assign76730_e116280_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76740_e116301, assign76740_e116301_d_n0, assign76740_e116301_d_n2, assign76740_e116301_d_n4, assign76740_e116301_d_n5, assign76740_e116301_d_n6, assign76740_e116301_d_n7, assign76740_e116301_d_n8, assign76740_e116301_d_n9, assign76740_e116301_d_n10, assign76740_e116301_d_n11, assign76740_e116301_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 == 0.0)) {
        let assign76740_e116291: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76740_e116293: f64 = (assign76740_e116291 / 2.0);
        let assign76740_e116296: f64 = (locals.var_tx).sqrt();
        let assign76740_e116297: f64 = (1.0 - assign76740_e116296);
        let assign76740_e116298: f64 = (assign76740_e116293 * assign76740_e116297);
        let assign76740_e116299: f64 = (locals.var_vgpld + assign76740_e116298);
        (assign76740_e116299, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn0 / (2.0 * assign76740_e116296))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn2 / (2.0 * assign76740_e116296)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn4 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn5 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn6 / (2.0 * assign76740_e116296))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn7 / (2.0 * assign76740_e116296)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn8 / (2.0 * assign76740_e116296)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn9 / (2.0 * assign76740_e116296)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn10 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn11 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn14 / (2.0 * assign76740_e116296))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76740_e116301;
        locals.var_ps0_inia_dn0 = assign76740_e116301_d_n0;
        locals.var_ps0_inia_dn2 = assign76740_e116301_d_n2;
        locals.var_ps0_inia_dn4 = assign76740_e116301_d_n4;
        locals.var_ps0_inia_dn5 = assign76740_e116301_d_n5;
        locals.var_ps0_inia_dn6 = assign76740_e116301_d_n6;
        locals.var_ps0_inia_dn7 = assign76740_e116301_d_n7;
        locals.var_ps0_inia_dn8 = assign76740_e116301_d_n8;
        locals.var_ps0_inia_dn9 = assign76740_e116301_d_n9;
        locals.var_ps0_inia_dn10 = assign76740_e116301_d_n10;
        locals.var_ps0_inia_dn11 = assign76740_e116301_d_n11;
        locals.var_ps0_inia_dn14 = assign76740_e116301_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76750_e116312, assign76750_e116312_d_n0, assign76750_e116312_d_n2, assign76750_e116312_d_n4, assign76750_e116312_d_n5, assign76750_e116312_d_n6, assign76750_e116312_d_n7, assign76750_e116312_d_n8, assign76750_e116312_d_n9, assign76750_e116312_d_n10, assign76750_e116312_d_n11, assign76750_e116312_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign76750_e116309: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76750_e116310: f64 = (locals.var_beta * assign76750_e116309);
        (assign76750_e116310, ((locals.var_beta_dn0 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76750_e116312;
        locals.var_chi_dn0 = assign76750_e116312_d_n0;
        locals.var_chi_dn2 = assign76750_e116312_d_n2;
        locals.var_chi_dn4 = assign76750_e116312_d_n4;
        locals.var_chi_dn5 = assign76750_e116312_d_n5;
        locals.var_chi_dn6 = assign76750_e116312_d_n6;
        locals.var_chi_dn7 = assign76750_e116312_d_n7;
        locals.var_chi_dn8 = assign76750_e116312_d_n8;
        locals.var_chi_dn9 = assign76750_e116312_d_n9;
        locals.var_chi_dn10 = assign76750_e116312_d_n10;
        locals.var_chi_dn11 = assign76750_e116312_d_n11;
        locals.var_chi_dn14 = assign76750_e116312_d_n14;
        locals.var_chi_rv = 0.0;

        let assign76760_e116315: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1793 = assign76760_e116315;
        locals.var_guard1793_rv = 0.0;

        let (assign76780_e116335, assign76780_e116335_d_n0, assign76780_e116335_d_n2, assign76780_e116335_d_n4, assign76780_e116335_d_n5, assign76780_e116335_d_n6, assign76780_e116335_d_n7, assign76780_e116335_d_n8, assign76780_e116335_d_n9, assign76780_e116335_d_n10, assign76780_e116335_d_n11, assign76780_e116335_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76780_e116332: f64 = (-locals.var_chi);
        let assign76780_e116333: f64 = (assign76780_e116332).exp();
        (assign76780_e116333, (assign76780_e116333 * (-locals.var_chi_dn0)), (assign76780_e116333 * (-locals.var_chi_dn2)), (assign76780_e116333 * (-locals.var_chi_dn4)), (assign76780_e116333 * (-locals.var_chi_dn5)), (assign76780_e116333 * (-locals.var_chi_dn6)), (assign76780_e116333 * (-locals.var_chi_dn7)), (assign76780_e116333 * (-locals.var_chi_dn8)), (assign76780_e116333 * (-locals.var_chi_dn9)), (assign76780_e116333 * (-locals.var_chi_dn10)), (assign76780_e116333 * (-locals.var_chi_dn11)), (assign76780_e116333 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76780_e116335;
        locals.var_ty_dn0 = assign76780_e116335_d_n0;
        locals.var_ty_dn2 = assign76780_e116335_d_n2;
        locals.var_ty_dn4 = assign76780_e116335_d_n4;
        locals.var_ty_dn5 = assign76780_e116335_d_n5;
        locals.var_ty_dn6 = assign76780_e116335_d_n6;
        locals.var_ty_dn7 = assign76780_e116335_d_n7;
        locals.var_ty_dn8 = assign76780_e116335_d_n8;
        locals.var_ty_dn9 = assign76780_e116335_d_n9;
        locals.var_ty_dn10 = assign76780_e116335_d_n10;
        locals.var_ty_dn11 = assign76780_e116335_d_n11;
        locals.var_ty_dn14 = assign76780_e116335_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76790_e116360, assign76790_e116360_d_n0, assign76790_e116360_d_n2, assign76790_e116360_d_n4, assign76790_e116360_d_n5, assign76790_e116360_d_n6, assign76790_e116360_d_n7, assign76790_e116360_d_n8, assign76790_e116360_d_n9, assign76790_e116360_d_n10, assign76790_e116360_d_n11, assign76790_e116360_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76790_e116347: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76790_e116348: f64 = (locals.var_beta * assign76790_e116347);
        let assign76790_e116350: f64 = (assign76790_e116348 - 1.0);
        let assign76790_e116352: f64 = (assign76790_e116350 + locals.var_ty);
        let assign76790_e116353: f64 = (4.0 * assign76790_e116352);
        let assign76790_e116356: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76790_e116357: f64 = (assign76790_e116353 / assign76790_e116356);
        let assign76790_e116358: f64 = (1.0 + assign76790_e116357);
        (assign76790_e116358, ((((4.0 * (((locals.var_beta_dn0 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn2 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn4 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn5 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn6 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn7 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn8 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn9 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn10 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn11 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn14 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76790_e116356 * assign76790_e116356)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76790_e116360;
        locals.var_tx_dn0 = assign76790_e116360_d_n0;
        locals.var_tx_dn2 = assign76790_e116360_d_n2;
        locals.var_tx_dn4 = assign76790_e116360_d_n4;
        locals.var_tx_dn5 = assign76790_e116360_d_n5;
        locals.var_tx_dn6 = assign76790_e116360_d_n6;
        locals.var_tx_dn7 = assign76790_e116360_d_n7;
        locals.var_tx_dn8 = assign76790_e116360_d_n8;
        locals.var_tx_dn9 = assign76790_e116360_d_n9;
        locals.var_tx_dn10 = assign76790_e116360_d_n10;
        locals.var_tx_dn11 = assign76790_e116360_d_n11;
        locals.var_tx_dn14 = assign76790_e116360_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76800_e116380, assign76800_e116380_d_n0, assign76800_e116380_d_n2, assign76800_e116380_d_n4, assign76800_e116380_d_n5, assign76800_e116380_d_n6, assign76800_e116380_d_n7, assign76800_e116380_d_n8, assign76800_e116380_d_n9, assign76800_e116380_d_n10, assign76800_e116380_d_n11, assign76800_e116380_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76800_e116370: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76800_e116372: f64 = (assign76800_e116370 / 2.0);
        let assign76800_e116375: f64 = (locals.var_tx).sqrt();
        let assign76800_e116376: f64 = (1.0 - assign76800_e116375);
        let assign76800_e116377: f64 = (assign76800_e116372 * assign76800_e116376);
        let assign76800_e116378: f64 = (locals.var_vgpld + assign76800_e116377);
        (assign76800_e116378, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn0 / (2.0 * assign76800_e116375))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn2 / (2.0 * assign76800_e116375)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn4 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn5 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn6 / (2.0 * assign76800_e116375))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn7 / (2.0 * assign76800_e116375)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn8 / (2.0 * assign76800_e116375)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn9 / (2.0 * assign76800_e116375)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn10 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn11 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn14 / (2.0 * assign76800_e116375))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76800_e116380;
        locals.var_ps0_inia_dn0 = assign76800_e116380_d_n0;
        locals.var_ps0_inia_dn2 = assign76800_e116380_d_n2;
        locals.var_ps0_inia_dn4 = assign76800_e116380_d_n4;
        locals.var_ps0_inia_dn5 = assign76800_e116380_d_n5;
        locals.var_ps0_inia_dn6 = assign76800_e116380_d_n6;
        locals.var_ps0_inia_dn7 = assign76800_e116380_d_n7;
        locals.var_ps0_inia_dn8 = assign76800_e116380_d_n8;
        locals.var_ps0_inia_dn9 = assign76800_e116380_d_n9;
        locals.var_ps0_inia_dn10 = assign76800_e116380_d_n10;
        locals.var_ps0_inia_dn11 = assign76800_e116380_d_n11;
        locals.var_ps0_inia_dn14 = assign76800_e116380_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76810_e116393, assign76810_e116393_d_n0, assign76810_e116393_d_n2, assign76810_e116393_d_n4, assign76810_e116393_d_n5, assign76810_e116393_d_n6, assign76810_e116393_d_n7, assign76810_e116393_d_n8, assign76810_e116393_d_n9, assign76810_e116393_d_n10, assign76810_e116393_d_n11, assign76810_e116393_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76810_e116390: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76810_e116391: f64 = (locals.var_beta * assign76810_e116390);
        (assign76810_e116391, ((locals.var_beta_dn0 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76810_e116393;
        locals.var_chi_dn0 = assign76810_e116393_d_n0;
        locals.var_chi_dn2 = assign76810_e116393_d_n2;
        locals.var_chi_dn4 = assign76810_e116393_d_n4;
        locals.var_chi_dn5 = assign76810_e116393_d_n5;
        locals.var_chi_dn6 = assign76810_e116393_d_n6;
        locals.var_chi_dn7 = assign76810_e116393_d_n7;
        locals.var_chi_dn8 = assign76810_e116393_d_n8;
        locals.var_chi_dn9 = assign76810_e116393_d_n9;
        locals.var_chi_dn10 = assign76810_e116393_d_n10;
        locals.var_chi_dn11 = assign76810_e116393_d_n11;
        locals.var_chi_dn14 = assign76810_e116393_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76820_e116404, assign76820_e116404_d_n0, assign76820_e116404_d_n2, assign76820_e116404_d_n4, assign76820_e116404_d_n5, assign76820_e116404_d_n6, assign76820_e116404_d_n7, assign76820_e116404_d_n8, assign76820_e116404_d_n9, assign76820_e116404_d_n10, assign76820_e116404_d_n11, assign76820_e116404_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76820_e116401: f64 = (-locals.var_chi);
        let assign76820_e116402: f64 = (assign76820_e116401).exp();
        (assign76820_e116402, (assign76820_e116402 * (-locals.var_chi_dn0)), (assign76820_e116402 * (-locals.var_chi_dn2)), (assign76820_e116402 * (-locals.var_chi_dn4)), (assign76820_e116402 * (-locals.var_chi_dn5)), (assign76820_e116402 * (-locals.var_chi_dn6)), (assign76820_e116402 * (-locals.var_chi_dn7)), (assign76820_e116402 * (-locals.var_chi_dn8)), (assign76820_e116402 * (-locals.var_chi_dn9)), (assign76820_e116402 * (-locals.var_chi_dn10)), (assign76820_e116402 * (-locals.var_chi_dn11)), (assign76820_e116402 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76820_e116404;
        locals.var_ty_dn0 = assign76820_e116404_d_n0;
        locals.var_ty_dn2 = assign76820_e116404_d_n2;
        locals.var_ty_dn4 = assign76820_e116404_d_n4;
        locals.var_ty_dn5 = assign76820_e116404_d_n5;
        locals.var_ty_dn6 = assign76820_e116404_d_n6;
        locals.var_ty_dn7 = assign76820_e116404_d_n7;
        locals.var_ty_dn8 = assign76820_e116404_d_n8;
        locals.var_ty_dn9 = assign76820_e116404_d_n9;
        locals.var_ty_dn10 = assign76820_e116404_d_n10;
        locals.var_ty_dn11 = assign76820_e116404_d_n11;
        locals.var_ty_dn14 = assign76820_e116404_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76830_e116429, assign76830_e116429_d_n0, assign76830_e116429_d_n2, assign76830_e116429_d_n4, assign76830_e116429_d_n5, assign76830_e116429_d_n6, assign76830_e116429_d_n7, assign76830_e116429_d_n8, assign76830_e116429_d_n9, assign76830_e116429_d_n10, assign76830_e116429_d_n11, assign76830_e116429_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76830_e116416: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76830_e116417: f64 = (locals.var_beta * assign76830_e116416);
        let assign76830_e116419: f64 = (assign76830_e116417 - 1.0);
        let assign76830_e116421: f64 = (assign76830_e116419 + locals.var_ty);
        let assign76830_e116422: f64 = (4.0 * assign76830_e116421);
        let assign76830_e116425: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76830_e116426: f64 = (assign76830_e116422 / assign76830_e116425);
        let assign76830_e116427: f64 = (1.0 + assign76830_e116426);
        (assign76830_e116427, ((((4.0 * (((locals.var_beta_dn0 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn2 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn4 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn5 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn6 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn7 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn8 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn9 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn10 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn11 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn14 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76830_e116425 * assign76830_e116425)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76830_e116429;
        locals.var_tx_dn0 = assign76830_e116429_d_n0;
        locals.var_tx_dn2 = assign76830_e116429_d_n2;
        locals.var_tx_dn4 = assign76830_e116429_d_n4;
        locals.var_tx_dn5 = assign76830_e116429_d_n5;
        locals.var_tx_dn6 = assign76830_e116429_d_n6;
        locals.var_tx_dn7 = assign76830_e116429_d_n7;
        locals.var_tx_dn8 = assign76830_e116429_d_n8;
        locals.var_tx_dn9 = assign76830_e116429_d_n9;
        locals.var_tx_dn10 = assign76830_e116429_d_n10;
        locals.var_tx_dn11 = assign76830_e116429_d_n11;
        locals.var_tx_dn14 = assign76830_e116429_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76840_e116449, assign76840_e116449_d_n0, assign76840_e116449_d_n2, assign76840_e116449_d_n4, assign76840_e116449_d_n5, assign76840_e116449_d_n6, assign76840_e116449_d_n7, assign76840_e116449_d_n8, assign76840_e116449_d_n9, assign76840_e116449_d_n10, assign76840_e116449_d_n11, assign76840_e116449_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76840_e116439: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76840_e116441: f64 = (assign76840_e116439 / 2.0);
        let assign76840_e116444: f64 = (locals.var_tx).sqrt();
        let assign76840_e116445: f64 = (1.0 - assign76840_e116444);
        let assign76840_e116446: f64 = (assign76840_e116441 * assign76840_e116445);
        let assign76840_e116447: f64 = (locals.var_vgpld + assign76840_e116446);
        (assign76840_e116447, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn0 / (2.0 * assign76840_e116444))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn2 / (2.0 * assign76840_e116444)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn4 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn5 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn6 / (2.0 * assign76840_e116444))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn7 / (2.0 * assign76840_e116444)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn8 / (2.0 * assign76840_e116444)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn9 / (2.0 * assign76840_e116444)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn10 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn11 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn14 / (2.0 * assign76840_e116444))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76840_e116449;
        locals.var_ps0_inia_dn0 = assign76840_e116449_d_n0;
        locals.var_ps0_inia_dn2 = assign76840_e116449_d_n2;
        locals.var_ps0_inia_dn4 = assign76840_e116449_d_n4;
        locals.var_ps0_inia_dn5 = assign76840_e116449_d_n5;
        locals.var_ps0_inia_dn6 = assign76840_e116449_d_n6;
        locals.var_ps0_inia_dn7 = assign76840_e116449_d_n7;
        locals.var_ps0_inia_dn8 = assign76840_e116449_d_n8;
        locals.var_ps0_inia_dn9 = assign76840_e116449_d_n9;
        locals.var_ps0_inia_dn10 = assign76840_e116449_d_n10;
        locals.var_ps0_inia_dn11 = assign76840_e116449_d_n11;
        locals.var_ps0_inia_dn14 = assign76840_e116449_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76850_e116462, assign76850_e116462_d_n0, assign76850_e116462_d_n2, assign76850_e116462_d_n4, assign76850_e116462_d_n5, assign76850_e116462_d_n6, assign76850_e116462_d_n7, assign76850_e116462_d_n8, assign76850_e116462_d_n9, assign76850_e116462_d_n10, assign76850_e116462_d_n11, assign76850_e116462_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76850_e116459: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76850_e116460: f64 = (locals.var_beta * assign76850_e116459);
        (assign76850_e116460, ((locals.var_beta_dn0 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76850_e116462;
        locals.var_chi_dn0 = assign76850_e116462_d_n0;
        locals.var_chi_dn2 = assign76850_e116462_d_n2;
        locals.var_chi_dn4 = assign76850_e116462_d_n4;
        locals.var_chi_dn5 = assign76850_e116462_d_n5;
        locals.var_chi_dn6 = assign76850_e116462_d_n6;
        locals.var_chi_dn7 = assign76850_e116462_d_n7;
        locals.var_chi_dn8 = assign76850_e116462_d_n8;
        locals.var_chi_dn9 = assign76850_e116462_d_n9;
        locals.var_chi_dn10 = assign76850_e116462_d_n10;
        locals.var_chi_dn11 = assign76850_e116462_d_n11;
        locals.var_chi_dn14 = assign76850_e116462_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76870_e116504,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76870_e116483: f64 = (2.0_f64).sqrt();
        let assign76870_e116484: f64 = (9.0 * assign76870_e116483);
        let assign76870_e116485: f64 = (1.0 / assign76870_e116484);
        let assign76870_e116489: f64 = (-3.0);
        let assign76870_e116490: f64 = (assign76870_e116489).exp();
        let assign76870_e116491: f64 = (7.0 * assign76870_e116490);
        let assign76870_e116492: f64 = (5.0 + assign76870_e116491);
        let assign76870_e116496: f64 = (-3.0);
        let assign76870_e116497: f64 = (assign76870_e116496).exp();
        let assign76870_e116498: f64 = (2.0 + assign76870_e116497);
        let assign76870_e116499: f64 = (assign76870_e116498).sqrt();
        let assign76870_e116500: f64 = (54.0 * assign76870_e116499);
        let assign76870_e116501: f64 = (assign76870_e116492 / assign76870_e116500);
        let assign76870_e116502: f64 = (assign76870_e116485 - assign76870_e116501);
        (assign76870_e116502,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign76870_e116504;
        locals.var_ta_rv = 0.0;

        let (assign76880_e116532,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76880_e116514: f64 = (-3.0);
        let assign76880_e116515: f64 = (assign76880_e116514).exp();
        let assign76880_e116516: f64 = (1.0 + assign76880_e116515);
        let assign76880_e116520: f64 = (-3.0);
        let assign76880_e116521: f64 = (assign76880_e116520).exp();
        let assign76880_e116522: f64 = (2.0 + assign76880_e116521);
        let assign76880_e116523: f64 = (assign76880_e116522).sqrt();
        let assign76880_e116524: f64 = (2.0 * assign76880_e116523);
        let assign76880_e116525: f64 = (assign76880_e116516 / assign76880_e116524);
        let assign76880_e116527: f64 = (2.0_f64).sqrt();
        let assign76880_e116529: f64 = (assign76880_e116527 / 3.0);
        let assign76880_e116530: f64 = (assign76880_e116525 - assign76880_e116529);
        (assign76880_e116530,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign76880_e116532;
        locals.var_tb_rv = 0.0;

        let (assign76890_e116551, assign76890_e116551_d_n0, assign76890_e116551_d_n2, assign76890_e116551_d_n4, assign76890_e116551_d_n5, assign76890_e116551_d_n6, assign76890_e116551_d_n7, assign76890_e116551_d_n8, assign76890_e116551_d_n9, assign76890_e116551_d_n10, assign76890_e116551_d_n11, assign76890_e116551_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76890_e116542: f64 = (2.0_f64).sqrt();
        let assign76890_e116543: f64 = (1.0 / assign76890_e116542);
        let assign76890_e116547: f64 = (locals.var_beta * locals.var_fac1);
        let assign76890_e116548: f64 = (1.0 / assign76890_e116547);
        let assign76890_e116549: f64 = (assign76890_e116543 + assign76890_e116548);
        (assign76890_e116549, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign76890_e116547 * assign76890_e116547))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign76890_e116551;
        locals.var_tc_dn0 = assign76890_e116551_d_n0;
        locals.var_tc_dn2 = assign76890_e116551_d_n2;
        locals.var_tc_dn4 = assign76890_e116551_d_n4;
        locals.var_tc_dn5 = assign76890_e116551_d_n5;
        locals.var_tc_dn6 = assign76890_e116551_d_n6;
        locals.var_tc_dn7 = assign76890_e116551_d_n7;
        locals.var_tc_dn8 = assign76890_e116551_d_n8;
        locals.var_tc_dn9 = assign76890_e116551_d_n9;
        locals.var_tc_dn10 = assign76890_e116551_d_n10;
        locals.var_tc_dn11 = assign76890_e116551_d_n11;
        locals.var_tc_dn14 = assign76890_e116551_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign76900_e116566, assign76900_e116566_d_n0, assign76900_e116566_d_n2, assign76900_e116566_d_n4, assign76900_e116566_d_n5, assign76900_e116566_d_n6, assign76900_e116566_d_n7, assign76900_e116566_d_n8, assign76900_e116566_d_n9, assign76900_e116566_d_n10, assign76900_e116566_d_n11, assign76900_e116566_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76900_e116561: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76900_e116562: f64 = (-assign76900_e116561);
        let assign76900_e116564: f64 = (assign76900_e116562 / locals.var_fac1);
        (assign76900_e116564, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign76900_e116566;
        locals.var_td_dn0 = assign76900_e116566_d_n0;
        locals.var_td_dn2 = assign76900_e116566_d_n2;
        locals.var_td_dn4 = assign76900_e116566_d_n4;
        locals.var_td_dn5 = assign76900_e116566_d_n5;
        locals.var_td_dn6 = assign76900_e116566_d_n6;
        locals.var_td_dn7 = assign76900_e116566_d_n7;
        locals.var_td_dn8 = assign76900_e116566_d_n8;
        locals.var_td_dn9 = assign76900_e116566_d_n9;
        locals.var_td_dn10 = assign76900_e116566_d_n10;
        locals.var_td_dn11 = assign76900_e116566_d_n11;
        locals.var_td_dn14 = assign76900_e116566_d_n14;
        locals.var_td_rv = 0.0;

        let (assign76910_e116604, assign76910_e116604_d_n0, assign76910_e116604_d_n2, assign76910_e116604_d_n4, assign76910_e116604_d_n5, assign76910_e116604_d_n6, assign76910_e116604_d_n7, assign76910_e116604_d_n8, assign76910_e116604_d_n9, assign76910_e116604_d_n10, assign76910_e116604_d_n11, assign76910_e116604_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76910_e116576: f64 = (locals.var_tb * locals.var_tb);
        let assign76910_e116578: f64 = (assign76910_e116576 * locals.var_tb);
        let assign76910_e116581: f64 = (27.0 * locals.var_ta);
        let assign76910_e116583: f64 = (assign76910_e116581 * locals.var_ta);
        let assign76910_e116585: f64 = (assign76910_e116583 * locals.var_ta);
        let assign76910_e116586: f64 = (assign76910_e116578 / assign76910_e116585);
        let assign76910_e116589: f64 = (locals.var_tb * locals.var_tc);
        let assign76910_e116592: f64 = (6.0 * locals.var_ta);
        let assign76910_e116594: f64 = (assign76910_e116592 * locals.var_ta);
        let assign76910_e116595: f64 = (assign76910_e116589 / assign76910_e116594);
        let assign76910_e116596: f64 = (assign76910_e116586 - assign76910_e116595);
        let assign76910_e116600: f64 = (2.0 * locals.var_ta);
        let assign76910_e116601: f64 = (locals.var_td / assign76910_e116600);
        let assign76910_e116602: f64 = (assign76910_e116596 + assign76910_e116601);
        (assign76910_e116602, ((-((locals.var_tb * locals.var_tc_dn0) / assign76910_e116594)) + (locals.var_td_dn0 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn2) / assign76910_e116594)) + (locals.var_td_dn2 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn4) / assign76910_e116594)) + (locals.var_td_dn4 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn5) / assign76910_e116594)) + (locals.var_td_dn5 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn6) / assign76910_e116594)) + (locals.var_td_dn6 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn7) / assign76910_e116594)) + (locals.var_td_dn7 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn8) / assign76910_e116594)) + (locals.var_td_dn8 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn9) / assign76910_e116594)) + (locals.var_td_dn9 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn10) / assign76910_e116594)) + (locals.var_td_dn10 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn11) / assign76910_e116594)) + (locals.var_td_dn11 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn14) / assign76910_e116594)) + (locals.var_td_dn14 / assign76910_e116600)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign76910_e116604;
        locals.var_tq_dn0 = assign76910_e116604_d_n0;
        locals.var_tq_dn2 = assign76910_e116604_d_n2;
        locals.var_tq_dn4 = assign76910_e116604_d_n4;
        locals.var_tq_dn5 = assign76910_e116604_d_n5;
        locals.var_tq_dn6 = assign76910_e116604_d_n6;
        locals.var_tq_dn7 = assign76910_e116604_d_n7;
        locals.var_tq_dn8 = assign76910_e116604_d_n8;
        locals.var_tq_dn9 = assign76910_e116604_d_n9;
        locals.var_tq_dn10 = assign76910_e116604_d_n10;
        locals.var_tq_dn11 = assign76910_e116604_d_n11;
        locals.var_tq_dn14 = assign76910_e116604_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign76920_e116628, assign76920_e116628_d_n0, assign76920_e116628_d_n2, assign76920_e116628_d_n4, assign76920_e116628_d_n5, assign76920_e116628_d_n6, assign76920_e116628_d_n7, assign76920_e116628_d_n8, assign76920_e116628_d_n9, assign76920_e116628_d_n10, assign76920_e116628_d_n11, assign76920_e116628_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76920_e116614: f64 = (3.0 * locals.var_ta);
        let assign76920_e116616: f64 = (assign76920_e116614 * locals.var_tc);
        let assign76920_e116619: f64 = (locals.var_tb * locals.var_tb);
        let assign76920_e116620: f64 = (assign76920_e116616 - assign76920_e116619);
        let assign76920_e116623: f64 = (9.0 * locals.var_ta);
        let assign76920_e116625: f64 = (assign76920_e116623 * locals.var_ta);
        let assign76920_e116626: f64 = (assign76920_e116620 / assign76920_e116625);
        (assign76920_e116626, ((assign76920_e116614 * locals.var_tc_dn0) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn2) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn4) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn5) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn6) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn7) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn8) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn9) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn10) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn11) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn14) / assign76920_e116625),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign76920_e116628;
        locals.var_tp_dn0 = assign76920_e116628_d_n0;
        locals.var_tp_dn2 = assign76920_e116628_d_n2;
        locals.var_tp_dn4 = assign76920_e116628_d_n4;
        locals.var_tp_dn5 = assign76920_e116628_d_n5;
        locals.var_tp_dn6 = assign76920_e116628_d_n6;
        locals.var_tp_dn7 = assign76920_e116628_d_n7;
        locals.var_tp_dn8 = assign76920_e116628_d_n8;
        locals.var_tp_dn9 = assign76920_e116628_d_n9;
        locals.var_tp_dn10 = assign76920_e116628_d_n10;
        locals.var_tp_dn11 = assign76920_e116628_d_n11;
        locals.var_tp_dn14 = assign76920_e116628_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign76930_e116647, assign76930_e116647_d_n0, assign76930_e116647_d_n2, assign76930_e116647_d_n4, assign76930_e116647_d_n5, assign76930_e116647_d_n6, assign76930_e116647_d_n7, assign76930_e116647_d_n8, assign76930_e116647_d_n9, assign76930_e116647_d_n10, assign76930_e116647_d_n11, assign76930_e116647_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76930_e116638: f64 = (locals.var_tq * locals.var_tq);
        let assign76930_e116641: f64 = (locals.var_tp * locals.var_tp);
        let assign76930_e116643: f64 = (assign76930_e116641 * locals.var_tp);
        let assign76930_e116644: f64 = (assign76930_e116638 + assign76930_e116643);
        let assign76930_e116645: f64 = (assign76930_e116644).sqrt();
        (assign76930_e116645, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn0))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn2))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn4))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn5))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn6))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn7))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn8))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn9))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn10))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn11))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn14))) / (2.0 * assign76930_e116645)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign76930_e116647;
        locals.var_t5_dn0 = assign76930_e116647_d_n0;
        locals.var_t5_dn2 = assign76930_e116647_d_n2;
        locals.var_t5_dn4 = assign76930_e116647_d_n4;
        locals.var_t5_dn5 = assign76930_e116647_d_n5;
        locals.var_t5_dn6 = assign76930_e116647_d_n6;
        locals.var_t5_dn7 = assign76930_e116647_d_n7;
        locals.var_t5_dn8 = assign76930_e116647_d_n8;
        locals.var_t5_dn9 = assign76930_e116647_d_n9;
        locals.var_t5_dn10 = assign76930_e116647_d_n10;
        locals.var_t5_dn11 = assign76930_e116647_d_n11;
        locals.var_t5_dn14 = assign76930_e116647_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign76940_e116662, assign76940_e116662_d_n0, assign76940_e116662_d_n2, assign76940_e116662_d_n4, assign76940_e116662_d_n5, assign76940_e116662_d_n6, assign76940_e116662_d_n7, assign76940_e116662_d_n8, assign76940_e116662_d_n9, assign76940_e116662_d_n10, assign76940_e116662_d_n11, assign76940_e116662_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76940_e116656: f64 = (-locals.var_tq);
        let assign76940_e116658: f64 = (assign76940_e116656 + locals.var_t5);
        let assign76940_e116660: f64 = (assign76940_e116658).powf(0.3333333333333333);
        (assign76940_e116660, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign76940_e116658))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign76940_e116662;
        locals.var_tu_dn0 = assign76940_e116662_d_n0;
        locals.var_tu_dn2 = assign76940_e116662_d_n2;
        locals.var_tu_dn4 = assign76940_e116662_d_n4;
        locals.var_tu_dn5 = assign76940_e116662_d_n5;
        locals.var_tu_dn6 = assign76940_e116662_d_n6;
        locals.var_tu_dn7 = assign76940_e116662_d_n7;
        locals.var_tu_dn8 = assign76940_e116662_d_n8;
        locals.var_tu_dn9 = assign76940_e116662_d_n9;
        locals.var_tu_dn10 = assign76940_e116662_d_n10;
        locals.var_tu_dn11 = assign76940_e116662_d_n11;
        locals.var_tu_dn14 = assign76940_e116662_d_n14;
        locals.var_tu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_291(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign76950_e116677, assign76950_e116677_d_n0, assign76950_e116677_d_n2, assign76950_e116677_d_n4, assign76950_e116677_d_n5, assign76950_e116677_d_n6, assign76950_e116677_d_n7, assign76950_e116677_d_n8, assign76950_e116677_d_n9, assign76950_e116677_d_n10, assign76950_e116677_d_n11, assign76950_e116677_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76950_e116672: f64 = (locals.var_tq + locals.var_t5);
        let assign76950_e116674: f64 = (assign76950_e116672).powf(0.3333333333333333);
        let assign76950_e116675: f64 = (-assign76950_e116674);
        (assign76950_e116675, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign76950_e116672))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign76950_e116677;
        locals.var_tv_dn0 = assign76950_e116677_d_n0;
        locals.var_tv_dn2 = assign76950_e116677_d_n2;
        locals.var_tv_dn4 = assign76950_e116677_d_n4;
        locals.var_tv_dn5 = assign76950_e116677_d_n5;
        locals.var_tv_dn6 = assign76950_e116677_d_n6;
        locals.var_tv_dn7 = assign76950_e116677_d_n7;
        locals.var_tv_dn8 = assign76950_e116677_d_n8;
        locals.var_tv_dn9 = assign76950_e116677_d_n9;
        locals.var_tv_dn10 = assign76950_e116677_d_n10;
        locals.var_tv_dn11 = assign76950_e116677_d_n11;
        locals.var_tv_dn14 = assign76950_e116677_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign76960_e116695, assign76960_e116695_d_n0, assign76960_e116695_d_n2, assign76960_e116695_d_n4, assign76960_e116695_d_n5, assign76960_e116695_d_n6, assign76960_e116695_d_n7, assign76960_e116695_d_n8, assign76960_e116695_d_n9, assign76960_e116695_d_n10, assign76960_e116695_d_n11, assign76960_e116695_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76960_e116687: f64 = (locals.var_tu + locals.var_tv);
        let assign76960_e116691: f64 = (3.0 * locals.var_ta);
        let assign76960_e116692: f64 = (locals.var_tb / assign76960_e116691);
        let assign76960_e116693: f64 = (assign76960_e116687 - assign76960_e116692);
        (assign76960_e116693, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76960_e116695;
        locals.var_chi_dn0 = assign76960_e116695_d_n0;
        locals.var_chi_dn2 = assign76960_e116695_d_n2;
        locals.var_chi_dn4 = assign76960_e116695_d_n4;
        locals.var_chi_dn5 = assign76960_e116695_d_n5;
        locals.var_chi_dn6 = assign76960_e116695_d_n6;
        locals.var_chi_dn7 = assign76960_e116695_d_n7;
        locals.var_chi_dn8 = assign76960_e116695_d_n8;
        locals.var_chi_dn9 = assign76960_e116695_d_n9;
        locals.var_chi_dn10 = assign76960_e116695_d_n10;
        locals.var_chi_dn11 = assign76960_e116695_d_n11;
        locals.var_chi_dn14 = assign76960_e116695_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76970_e116709, assign76970_e116709_d_n0, assign76970_e116709_d_n2, assign76970_e116709_d_n4, assign76970_e116709_d_n5, assign76970_e116709_d_n6, assign76970_e116709_d_n7, assign76970_e116709_d_n8, assign76970_e116709_d_n9, assign76970_e116709_d_n10, assign76970_e116709_d_n11, assign76970_e116709_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76970_e116705: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign76970_e116707: f64 = (assign76970_e116705 - locals.var_vxbgmtcl);
        (assign76970_e116707, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76970_e116709;
        locals.var_ps0_inia_dn0 = assign76970_e116709_d_n0;
        locals.var_ps0_inia_dn2 = assign76970_e116709_d_n2;
        locals.var_ps0_inia_dn4 = assign76970_e116709_d_n4;
        locals.var_ps0_inia_dn5 = assign76970_e116709_d_n5;
        locals.var_ps0_inia_dn6 = assign76970_e116709_d_n6;
        locals.var_ps0_inia_dn7 = assign76970_e116709_d_n7;
        locals.var_ps0_inia_dn8 = assign76970_e116709_d_n8;
        locals.var_ps0_inia_dn9 = assign76970_e116709_d_n9;
        locals.var_ps0_inia_dn10 = assign76970_e116709_d_n10;
        locals.var_ps0_inia_dn11 = assign76970_e116709_d_n11;
        locals.var_ps0_inia_dn14 = assign76970_e116709_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign76980_e116712: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1794 = assign76980_e116712;
        locals.var_guard1794_rv = 0.0;

        let (assign76990_e116725, assign76990_e116725_d_n0, assign76990_e116725_d_n2, assign76990_e116725_d_n4, assign76990_e116725_d_n5, assign76990_e116725_d_n6, assign76990_e116725_d_n7, assign76990_e116725_d_n8, assign76990_e116725_d_n9, assign76990_e116725_d_n10, assign76990_e116725_d_n11, assign76990_e116725_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign76990_e116721: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76990_e116723: f64 = (assign76990_e116721 + 0.1);
        (assign76990_e116723, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign76990_e116725;
        locals.var_vgpld_shift_dn0 = assign76990_e116725_d_n0;
        locals.var_vgpld_shift_dn2 = assign76990_e116725_d_n2;
        locals.var_vgpld_shift_dn4 = assign76990_e116725_d_n4;
        locals.var_vgpld_shift_dn5 = assign76990_e116725_d_n5;
        locals.var_vgpld_shift_dn6 = assign76990_e116725_d_n6;
        locals.var_vgpld_shift_dn7 = assign76990_e116725_d_n7;
        locals.var_vgpld_shift_dn8 = assign76990_e116725_d_n8;
        locals.var_vgpld_shift_dn9 = assign76990_e116725_d_n9;
        locals.var_vgpld_shift_dn10 = assign76990_e116725_d_n10;
        locals.var_vgpld_shift_dn11 = assign76990_e116725_d_n11;
        locals.var_vgpld_shift_dn14 = assign76990_e116725_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign77000_e116736, assign77000_e116736_d_n0, assign77000_e116736_d_n2, assign77000_e116736_d_n4, assign77000_e116736_d_n5, assign77000_e116736_d_n6, assign77000_e116736_d_n7, assign77000_e116736_d_n8, assign77000_e116736_d_n9, assign77000_e116736_d_n10, assign77000_e116736_d_n11, assign77000_e116736_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77000_e116734: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign77000_e116734, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign77000_e116736;
        locals.var_cfs1_dn0 = assign77000_e116736_d_n0;
        locals.var_cfs1_dn2 = assign77000_e116736_d_n2;
        locals.var_cfs1_dn4 = assign77000_e116736_d_n4;
        locals.var_cfs1_dn5 = assign77000_e116736_d_n5;
        locals.var_cfs1_dn6 = assign77000_e116736_d_n6;
        locals.var_cfs1_dn7 = assign77000_e116736_d_n7;
        locals.var_cfs1_dn8 = assign77000_e116736_d_n8;
        locals.var_cfs1_dn9 = assign77000_e116736_d_n9;
        locals.var_cfs1_dn10 = assign77000_e116736_d_n10;
        locals.var_cfs1_dn11 = assign77000_e116736_d_n11;
        locals.var_cfs1_dn14 = assign77000_e116736_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign77010_e116747, assign77010_e116747_d_n0, assign77010_e116747_d_n2, assign77010_e116747_d_n4, assign77010_e116747_d_n5, assign77010_e116747_d_n6, assign77010_e116747_d_n7, assign77010_e116747_d_n8, assign77010_e116747_d_n9, assign77010_e116747_d_n10, assign77010_e116747_d_n11, assign77010_e116747_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77010_e116745: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign77010_e116745, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign77010_e116747;
        locals.var_gammachi_dn0 = assign77010_e116747_d_n0;
        locals.var_gammachi_dn2 = assign77010_e116747_d_n2;
        locals.var_gammachi_dn4 = assign77010_e116747_d_n4;
        locals.var_gammachi_dn5 = assign77010_e116747_d_n5;
        locals.var_gammachi_dn6 = assign77010_e116747_d_n6;
        locals.var_gammachi_dn7 = assign77010_e116747_d_n7;
        locals.var_gammachi_dn8 = assign77010_e116747_d_n8;
        locals.var_gammachi_dn9 = assign77010_e116747_d_n9;
        locals.var_gammachi_dn10 = assign77010_e116747_d_n10;
        locals.var_gammachi_dn11 = assign77010_e116747_d_n11;
        locals.var_gammachi_dn14 = assign77010_e116747_d_n14;
        locals.var_gammachi_rv = 0.0;

        let (assign77020_e116758, assign77020_e116758_d_n0, assign77020_e116758_d_n2, assign77020_e116758_d_n4, assign77020_e116758_d_n5, assign77020_e116758_d_n6, assign77020_e116758_d_n7, assign77020_e116758_d_n8, assign77020_e116758_d_n9, assign77020_e116758_d_n10, assign77020_e116758_d_n11, assign77020_e116758_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77020_e116756: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign77020_e116756, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77020_e116758;
        locals.var_t0_dn0 = assign77020_e116758_d_n0;
        locals.var_t0_dn2 = assign77020_e116758_d_n2;
        locals.var_t0_dn4 = assign77020_e116758_d_n4;
        locals.var_t0_dn5 = assign77020_e116758_d_n5;
        locals.var_t0_dn6 = assign77020_e116758_d_n6;
        locals.var_t0_dn7 = assign77020_e116758_d_n7;
        locals.var_t0_dn8 = assign77020_e116758_d_n8;
        locals.var_t0_dn9 = assign77020_e116758_d_n9;
        locals.var_t0_dn10 = assign77020_e116758_d_n10;
        locals.var_t0_dn11 = assign77020_e116758_d_n11;
        locals.var_t0_dn14 = assign77020_e116758_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77030_e116769, assign77030_e116769_d_n0, assign77030_e116769_d_n2, assign77030_e116769_d_n4, assign77030_e116769_d_n5, assign77030_e116769_d_n6, assign77030_e116769_d_n7, assign77030_e116769_d_n8, assign77030_e116769_d_n9, assign77030_e116769_d_n10, assign77030_e116769_d_n11, assign77030_e116769_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77030_e116767: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign77030_e116767, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77030_e116769;
        locals.var_psi_dn0 = assign77030_e116769_d_n0;
        locals.var_psi_dn2 = assign77030_e116769_d_n2;
        locals.var_psi_dn4 = assign77030_e116769_d_n4;
        locals.var_psi_dn5 = assign77030_e116769_d_n5;
        locals.var_psi_dn6 = assign77030_e116769_d_n6;
        locals.var_psi_dn7 = assign77030_e116769_d_n7;
        locals.var_psi_dn8 = assign77030_e116769_d_n8;
        locals.var_psi_dn9 = assign77030_e116769_d_n9;
        locals.var_psi_dn10 = assign77030_e116769_d_n10;
        locals.var_psi_dn11 = assign77030_e116769_d_n11;
        locals.var_psi_dn14 = assign77030_e116769_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77040_e116794, assign77040_e116794_d_n0, assign77040_e116794_d_n2, assign77040_e116794_d_n4, assign77040_e116794_d_n5, assign77040_e116794_d_n6, assign77040_e116794_d_n7, assign77040_e116794_d_n8, assign77040_e116794_d_n9, assign77040_e116794_d_n10, assign77040_e116794_d_n11, assign77040_e116794_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77040_e116778: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77040_e116781: f64 = (locals.var_psi * locals.var_psi);
        let assign77040_e116782: f64 = (assign77040_e116778 + assign77040_e116781);
        let assign77040_e116783: f64 = (assign77040_e116782).ln();
        let assign77040_e116786: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77040_e116787: f64 = (assign77040_e116786).ln();
        let assign77040_e116788: f64 = (assign77040_e116783 - assign77040_e116787);
        let assign77040_e116791: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77040_e116792: f64 = (assign77040_e116788 + assign77040_e116791);
        (assign77040_e116792, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77040_e116782) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77040_e116786)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77040_e116782) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77040_e116786)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77040_e116782) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77040_e116786)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77040_e116782) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77040_e116786)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77040_e116782) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77040_e116786)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77040_e116782) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77040_e116786)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77040_e116782) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77040_e116786)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77040_e116782) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77040_e116786)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77040_e116782) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77040_e116786)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77040_e116782) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77040_e116786)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77040_e116782) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77040_e116786)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77040_e116794;
        locals.var_chi_1_dn0 = assign77040_e116794_d_n0;
        locals.var_chi_1_dn2 = assign77040_e116794_d_n2;
        locals.var_chi_1_dn4 = assign77040_e116794_d_n4;
        locals.var_chi_1_dn5 = assign77040_e116794_d_n5;
        locals.var_chi_1_dn6 = assign77040_e116794_d_n6;
        locals.var_chi_1_dn7 = assign77040_e116794_d_n7;
        locals.var_chi_1_dn8 = assign77040_e116794_d_n8;
        locals.var_chi_1_dn9 = assign77040_e116794_d_n9;
        locals.var_chi_1_dn10 = assign77040_e116794_d_n10;
        locals.var_chi_1_dn11 = assign77040_e116794_d_n11;
        locals.var_chi_1_dn14 = assign77040_e116794_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign77050_e116797: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1795 = assign77050_e116797;
        locals.var_guard1795_rv = 0.0;

        let (assign77060_e116812, assign77060_e116812_d_n0, assign77060_e116812_d_n2, assign77060_e116812_d_n4, assign77060_e116812_d_n5, assign77060_e116812_d_n6, assign77060_e116812_d_n7, assign77060_e116812_d_n8, assign77060_e116812_d_n9, assign77060_e116812_d_n10, assign77060_e116812_d_n11, assign77060_e116812_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77060_e116808: f64 = (locals.var_psi - locals.var_chi_1);
        let assign77060_e116810: f64 = (assign77060_e116808 - 1.0);
        (assign77060_e116810, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77060_e116812;
        locals.var_tmf1_dn0 = assign77060_e116812_d_n0;
        locals.var_tmf1_dn2 = assign77060_e116812_d_n2;
        locals.var_tmf1_dn4 = assign77060_e116812_d_n4;
        locals.var_tmf1_dn5 = assign77060_e116812_d_n5;
        locals.var_tmf1_dn6 = assign77060_e116812_d_n6;
        locals.var_tmf1_dn7 = assign77060_e116812_d_n7;
        locals.var_tmf1_dn8 = assign77060_e116812_d_n8;
        locals.var_tmf1_dn9 = assign77060_e116812_d_n9;
        locals.var_tmf1_dn10 = assign77060_e116812_d_n10;
        locals.var_tmf1_dn11 = assign77060_e116812_d_n11;
        locals.var_tmf1_dn14 = assign77060_e116812_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign77070_e116827, assign77070_e116827_d_n0, assign77070_e116827_d_n2, assign77070_e116827_d_n4, assign77070_e116827_d_n5, assign77070_e116827_d_n6, assign77070_e116827_d_n7, assign77070_e116827_d_n8, assign77070_e116827_d_n9, assign77070_e116827_d_n10, assign77070_e116827_d_n11, assign77070_e116827_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77070_e116823: f64 = (4.0 * locals.var_psi);
        let assign77070_e116825: f64 = assign77070_e116823;
        (assign77070_e116825, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77070_e116827;
        locals.var_tmf2_dn0 = assign77070_e116827_d_n0;
        locals.var_tmf2_dn2 = assign77070_e116827_d_n2;
        locals.var_tmf2_dn4 = assign77070_e116827_d_n4;
        locals.var_tmf2_dn5 = assign77070_e116827_d_n5;
        locals.var_tmf2_dn6 = assign77070_e116827_d_n6;
        locals.var_tmf2_dn7 = assign77070_e116827_d_n7;
        locals.var_tmf2_dn8 = assign77070_e116827_d_n8;
        locals.var_tmf2_dn9 = assign77070_e116827_d_n9;
        locals.var_tmf2_dn10 = assign77070_e116827_d_n10;
        locals.var_tmf2_dn11 = assign77070_e116827_d_n11;
        locals.var_tmf2_dn14 = assign77070_e116827_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77080_e116844, assign77080_e116844_d_n0, assign77080_e116844_d_n2, assign77080_e116844_d_n4, assign77080_e116844_d_n5, assign77080_e116844_d_n6, assign77080_e116844_d_n7, assign77080_e116844_d_n8, assign77080_e116844_d_n9, assign77080_e116844_d_n10, assign77080_e116844_d_n11, assign77080_e116844_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let (assign77080_e116842, assign77080_e116842_d_n0, assign77080_e116842_d_n2, assign77080_e116842_d_n4, assign77080_e116842_d_n5, assign77080_e116842_d_n6, assign77080_e116842_d_n7, assign77080_e116842_d_n8, assign77080_e116842_d_n9, assign77080_e116842_d_n10, assign77080_e116842_d_n11, assign77080_e116842_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign77080_e116841: f64 = (-locals.var_tmf2);
                (assign77080_e116841, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign77080_e116842, assign77080_e116842_d_n0, assign77080_e116842_d_n2, assign77080_e116842_d_n4, assign77080_e116842_d_n5, assign77080_e116842_d_n6, assign77080_e116842_d_n7, assign77080_e116842_d_n8, assign77080_e116842_d_n9, assign77080_e116842_d_n10, assign77080_e116842_d_n11, assign77080_e116842_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77080_e116844;
        locals.var_tmf2_dn0 = assign77080_e116844_d_n0;
        locals.var_tmf2_dn2 = assign77080_e116844_d_n2;
        locals.var_tmf2_dn4 = assign77080_e116844_d_n4;
        locals.var_tmf2_dn5 = assign77080_e116844_d_n5;
        locals.var_tmf2_dn6 = assign77080_e116844_d_n6;
        locals.var_tmf2_dn7 = assign77080_e116844_d_n7;
        locals.var_tmf2_dn8 = assign77080_e116844_d_n8;
        locals.var_tmf2_dn9 = assign77080_e116844_d_n9;
        locals.var_tmf2_dn10 = assign77080_e116844_d_n10;
        locals.var_tmf2_dn11 = assign77080_e116844_d_n11;
        locals.var_tmf2_dn14 = assign77080_e116844_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77090_e116860, assign77090_e116860_d_n0, assign77090_e116860_d_n2, assign77090_e116860_d_n4, assign77090_e116860_d_n5, assign77090_e116860_d_n6, assign77090_e116860_d_n7, assign77090_e116860_d_n8, assign77090_e116860_d_n9, assign77090_e116860_d_n10, assign77090_e116860_d_n11, assign77090_e116860_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77090_e116855: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign77090_e116857: f64 = (assign77090_e116855 + locals.var_tmf2);
        let assign77090_e116858: f64 = (assign77090_e116857).sqrt();
        (assign77090_e116858, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign77090_e116858)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77090_e116860;
        locals.var_tmf2_dn0 = assign77090_e116860_d_n0;
        locals.var_tmf2_dn2 = assign77090_e116860_d_n2;
        locals.var_tmf2_dn4 = assign77090_e116860_d_n4;
        locals.var_tmf2_dn5 = assign77090_e116860_d_n5;
        locals.var_tmf2_dn6 = assign77090_e116860_d_n6;
        locals.var_tmf2_dn7 = assign77090_e116860_d_n7;
        locals.var_tmf2_dn8 = assign77090_e116860_d_n8;
        locals.var_tmf2_dn9 = assign77090_e116860_d_n9;
        locals.var_tmf2_dn10 = assign77090_e116860_d_n10;
        locals.var_tmf2_dn11 = assign77090_e116860_d_n11;
        locals.var_tmf2_dn14 = assign77090_e116860_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77100_e116877, assign77100_e116877_d_n0, assign77100_e116877_d_n2, assign77100_e116877_d_n4, assign77100_e116877_d_n5, assign77100_e116877_d_n6, assign77100_e116877_d_n7, assign77100_e116877_d_n8, assign77100_e116877_d_n9, assign77100_e116877_d_n10, assign77100_e116877_d_n11, assign77100_e116877_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77100_e116873: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign77100_e116874: f64 = (1.0 + assign77100_e116873);
        let assign77100_e116875: f64 = (0.5 * assign77100_e116874);
        (assign77100_e116875, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77100_e116877;
        locals.var_t1_dn0 = assign77100_e116877_d_n0;
        locals.var_t1_dn2 = assign77100_e116877_d_n2;
        locals.var_t1_dn4 = assign77100_e116877_d_n4;
        locals.var_t1_dn5 = assign77100_e116877_d_n5;
        locals.var_t1_dn6 = assign77100_e116877_d_n6;
        locals.var_t1_dn7 = assign77100_e116877_d_n7;
        locals.var_t1_dn8 = assign77100_e116877_d_n8;
        locals.var_t1_dn9 = assign77100_e116877_d_n9;
        locals.var_t1_dn10 = assign77100_e116877_d_n10;
        locals.var_t1_dn11 = assign77100_e116877_d_n11;
        locals.var_t1_dn14 = assign77100_e116877_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77110_e116894, assign77110_e116894_d_n0, assign77110_e116894_d_n2, assign77110_e116894_d_n4, assign77110_e116894_d_n5, assign77110_e116894_d_n6, assign77110_e116894_d_n7, assign77110_e116894_d_n8, assign77110_e116894_d_n9, assign77110_e116894_d_n10, assign77110_e116894_d_n11, assign77110_e116894_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77110_e116890: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign77110_e116891: f64 = (0.5 * assign77110_e116890);
        let assign77110_e116892: f64 = (locals.var_psi - assign77110_e116891);
        (assign77110_e116892, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77110_e116894;
        locals.var_chi_1_dn0 = assign77110_e116894_d_n0;
        locals.var_chi_1_dn2 = assign77110_e116894_d_n2;
        locals.var_chi_1_dn4 = assign77110_e116894_d_n4;
        locals.var_chi_1_dn5 = assign77110_e116894_d_n5;
        locals.var_chi_1_dn6 = assign77110_e116894_d_n6;
        locals.var_chi_1_dn7 = assign77110_e116894_d_n7;
        locals.var_chi_1_dn8 = assign77110_e116894_d_n8;
        locals.var_chi_1_dn9 = assign77110_e116894_d_n9;
        locals.var_chi_1_dn10 = assign77110_e116894_d_n10;
        locals.var_chi_1_dn11 = assign77110_e116894_d_n11;
        locals.var_chi_1_dn14 = assign77110_e116894_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77120_e116911, assign77120_e116911_d_n0, assign77120_e116911_d_n2, assign77120_e116911_d_n4, assign77120_e116911_d_n5, assign77120_e116911_d_n6, assign77120_e116911_d_n7, assign77120_e116911_d_n8, assign77120_e116911_d_n9, assign77120_e116911_d_n10, assign77120_e116911_d_n11, assign77120_e116911_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) {
        let (assign77120_e116909, assign77120_e116909_d_n0, assign77120_e116909_d_n2, assign77120_e116909_d_n4, assign77120_e116909_d_n5, assign77120_e116909_d_n6, assign77120_e116909_d_n7, assign77120_e116909_d_n8, assign77120_e116909_d_n9, assign77120_e116909_d_n10, assign77120_e116909_d_n11, assign77120_e116909_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign77120_e116909, assign77120_e116909_d_n0, assign77120_e116909_d_n2, assign77120_e116909_d_n4, assign77120_e116909_d_n5, assign77120_e116909_d_n6, assign77120_e116909_d_n7, assign77120_e116909_d_n8, assign77120_e116909_d_n9, assign77120_e116909_d_n10, assign77120_e116909_d_n11, assign77120_e116909_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77120_e116911;
        locals.var_chi_1_dn0 = assign77120_e116911_d_n0;
        locals.var_chi_1_dn2 = assign77120_e116911_d_n2;
        locals.var_chi_1_dn4 = assign77120_e116911_d_n4;
        locals.var_chi_1_dn5 = assign77120_e116911_d_n5;
        locals.var_chi_1_dn6 = assign77120_e116911_d_n6;
        locals.var_chi_1_dn7 = assign77120_e116911_d_n7;
        locals.var_chi_1_dn8 = assign77120_e116911_d_n8;
        locals.var_chi_1_dn9 = assign77120_e116911_d_n9;
        locals.var_chi_1_dn10 = assign77120_e116911_d_n10;
        locals.var_chi_1_dn11 = assign77120_e116911_d_n11;
        locals.var_chi_1_dn14 = assign77120_e116911_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77130_e116925, assign77130_e116925_d_n0, assign77130_e116925_d_n2, assign77130_e116925_d_n4, assign77130_e116925_d_n5, assign77130_e116925_d_n6, assign77130_e116925_d_n7, assign77130_e116925_d_n8, assign77130_e116925_d_n9, assign77130_e116925_d_n10, assign77130_e116925_d_n11, assign77130_e116925_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let (assign77130_e116923, assign77130_e116923_d_n0, assign77130_e116923_d_n2, assign77130_e116923_d_n4, assign77130_e116923_d_n5, assign77130_e116923_d_n6, assign77130_e116923_d_n7, assign77130_e116923_d_n8, assign77130_e116923_d_n9, assign77130_e116923_d_n10, assign77130_e116923_d_n11, assign77130_e116923_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77130_e116923, assign77130_e116923_d_n0, assign77130_e116923_d_n2, assign77130_e116923_d_n4, assign77130_e116923_d_n5, assign77130_e116923_d_n6, assign77130_e116923_d_n7, assign77130_e116923_d_n8, assign77130_e116923_d_n9, assign77130_e116923_d_n10, assign77130_e116923_d_n11, assign77130_e116923_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77130_e116925;
        locals.var_chi_1_dn0 = assign77130_e116925_d_n0;
        locals.var_chi_1_dn2 = assign77130_e116925_d_n2;
        locals.var_chi_1_dn4 = assign77130_e116925_d_n4;
        locals.var_chi_1_dn5 = assign77130_e116925_d_n5;
        locals.var_chi_1_dn6 = assign77130_e116925_d_n6;
        locals.var_chi_1_dn7 = assign77130_e116925_d_n7;
        locals.var_chi_1_dn8 = assign77130_e116925_d_n8;
        locals.var_chi_1_dn9 = assign77130_e116925_d_n9;
        locals.var_chi_1_dn10 = assign77130_e116925_d_n10;
        locals.var_chi_1_dn11 = assign77130_e116925_d_n11;
        locals.var_chi_1_dn14 = assign77130_e116925_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77140_e116936, assign77140_e116936_d_n0, assign77140_e116936_d_n2, assign77140_e116936_d_n4, assign77140_e116936_d_n5, assign77140_e116936_d_n6, assign77140_e116936_d_n7, assign77140_e116936_d_n8, assign77140_e116936_d_n9, assign77140_e116936_d_n10, assign77140_e116936_d_n11, assign77140_e116936_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77140_e116934: f64 = (locals.var_psi - locals.var_chi_1);
        (assign77140_e116934, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77140_e116936;
        locals.var_psi_dn0 = assign77140_e116936_d_n0;
        locals.var_psi_dn2 = assign77140_e116936_d_n2;
        locals.var_psi_dn4 = assign77140_e116936_d_n4;
        locals.var_psi_dn5 = assign77140_e116936_d_n5;
        locals.var_psi_dn6 = assign77140_e116936_d_n6;
        locals.var_psi_dn7 = assign77140_e116936_d_n7;
        locals.var_psi_dn8 = assign77140_e116936_d_n8;
        locals.var_psi_dn9 = assign77140_e116936_d_n9;
        locals.var_psi_dn10 = assign77140_e116936_d_n10;
        locals.var_psi_dn11 = assign77140_e116936_d_n11;
        locals.var_psi_dn14 = assign77140_e116936_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77150_e116949, assign77150_e116949_d_n0, assign77150_e116949_d_n2, assign77150_e116949_d_n4, assign77150_e116949_d_n5, assign77150_e116949_d_n6, assign77150_e116949_d_n7, assign77150_e116949_d_n8, assign77150_e116949_d_n9, assign77150_e116949_d_n10, assign77150_e116949_d_n11, assign77150_e116949_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77150_e116946: f64 = (locals.var_beta * 0.1);
        let assign77150_e116947: f64 = (locals.var_psi + assign77150_e116946);
        (assign77150_e116947, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77150_e116949;
        locals.var_psi_dn0 = assign77150_e116949_d_n0;
        locals.var_psi_dn2 = assign77150_e116949_d_n2;
        locals.var_psi_dn4 = assign77150_e116949_d_n4;
        locals.var_psi_dn5 = assign77150_e116949_d_n5;
        locals.var_psi_dn6 = assign77150_e116949_d_n6;
        locals.var_psi_dn7 = assign77150_e116949_d_n7;
        locals.var_psi_dn8 = assign77150_e116949_d_n8;
        locals.var_psi_dn9 = assign77150_e116949_d_n9;
        locals.var_psi_dn10 = assign77150_e116949_d_n10;
        locals.var_psi_dn11 = assign77150_e116949_d_n11;
        locals.var_psi_dn14 = assign77150_e116949_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77160_e116970, assign77160_e116970_d_n0, assign77160_e116970_d_n2, assign77160_e116970_d_n4, assign77160_e116970_d_n5, assign77160_e116970_d_n6, assign77160_e116970_d_n7, assign77160_e116970_d_n8, assign77160_e116970_d_n9, assign77160_e116970_d_n10, assign77160_e116970_d_n11, assign77160_e116970_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77160_e116958: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77160_e116961: f64 = (locals.var_psi * locals.var_psi);
        let assign77160_e116962: f64 = (assign77160_e116958 + assign77160_e116961);
        let assign77160_e116963: f64 = (assign77160_e116962).ln();
        let assign77160_e116966: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77160_e116967: f64 = (assign77160_e116966).ln();
        let assign77160_e116968: f64 = (assign77160_e116963 - assign77160_e116967);
        (assign77160_e116968, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77160_e116962) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77160_e116966)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77160_e116962) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77160_e116966)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77160_e116962) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77160_e116966)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77160_e116962) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77160_e116966)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77160_e116962) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77160_e116966)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77160_e116962) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77160_e116966)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77160_e116962) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77160_e116966)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77160_e116962) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77160_e116966)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77160_e116962) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77160_e116966)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77160_e116962) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77160_e116966)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77160_e116962) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77160_e116966)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77160_e116970;
        locals.var_t1_dn0 = assign77160_e116970_d_n0;
        locals.var_t1_dn2 = assign77160_e116970_d_n2;
        locals.var_t1_dn4 = assign77160_e116970_d_n4;
        locals.var_t1_dn5 = assign77160_e116970_d_n5;
        locals.var_t1_dn6 = assign77160_e116970_d_n6;
        locals.var_t1_dn7 = assign77160_e116970_d_n7;
        locals.var_t1_dn8 = assign77160_e116970_d_n8;
        locals.var_t1_dn9 = assign77160_e116970_d_n9;
        locals.var_t1_dn10 = assign77160_e116970_d_n10;
        locals.var_t1_dn11 = assign77160_e116970_d_n11;
        locals.var_t1_dn14 = assign77160_e116970_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_292(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77170_e116983, assign77170_e116983_d_n0, assign77170_e116983_d_n2, assign77170_e116983_d_n4, assign77170_e116983_d_n5, assign77170_e116983_d_n6, assign77170_e116983_d_n7, assign77170_e116983_d_n8, assign77170_e116983_d_n9, assign77170_e116983_d_n10, assign77170_e116983_d_n11, assign77170_e116983_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77170_e116980: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77170_e116981: f64 = (locals.var_t1 + assign77170_e116980);
        (assign77170_e116981, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77170_e116983;
        locals.var_chi_b_dn0 = assign77170_e116983_d_n0;
        locals.var_chi_b_dn2 = assign77170_e116983_d_n2;
        locals.var_chi_b_dn4 = assign77170_e116983_d_n4;
        locals.var_chi_b_dn5 = assign77170_e116983_d_n5;
        locals.var_chi_b_dn6 = assign77170_e116983_d_n6;
        locals.var_chi_b_dn7 = assign77170_e116983_d_n7;
        locals.var_chi_b_dn8 = assign77170_e116983_d_n8;
        locals.var_chi_b_dn9 = assign77170_e116983_d_n9;
        locals.var_chi_b_dn10 = assign77170_e116983_d_n10;
        locals.var_chi_b_dn11 = assign77170_e116983_d_n11;
        locals.var_chi_b_dn14 = assign77170_e116983_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign77180_e116997, assign77180_e116997_d_n0, assign77180_e116997_d_n2, assign77180_e116997_d_n4, assign77180_e116997_d_n5, assign77180_e116997_d_n6, assign77180_e116997_d_n7, assign77180_e116997_d_n8, assign77180_e116997_d_n9, assign77180_e116997_d_n10, assign77180_e116997_d_n11, assign77180_e116997_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let (assign77180_e116995, assign77180_e116995_d_n0, assign77180_e116995_d_n2, assign77180_e116995_d_n4, assign77180_e116995_d_n5, assign77180_e116995_d_n6, assign77180_e116995_d_n7, assign77180_e116995_d_n8, assign77180_e116995_d_n9, assign77180_e116995_d_n10, assign77180_e116995_d_n11, assign77180_e116995_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77180_e116995, assign77180_e116995_d_n0, assign77180_e116995_d_n2, assign77180_e116995_d_n4, assign77180_e116995_d_n5, assign77180_e116995_d_n6, assign77180_e116995_d_n7, assign77180_e116995_d_n8, assign77180_e116995_d_n9, assign77180_e116995_d_n10, assign77180_e116995_d_n11, assign77180_e116995_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77180_e116997;
        locals.var_chi_b_dn0 = assign77180_e116997_d_n0;
        locals.var_chi_b_dn2 = assign77180_e116997_d_n2;
        locals.var_chi_b_dn4 = assign77180_e116997_d_n4;
        locals.var_chi_b_dn5 = assign77180_e116997_d_n5;
        locals.var_chi_b_dn6 = assign77180_e116997_d_n6;
        locals.var_chi_b_dn7 = assign77180_e116997_d_n7;
        locals.var_chi_b_dn8 = assign77180_e116997_d_n8;
        locals.var_chi_b_dn9 = assign77180_e116997_d_n9;
        locals.var_chi_b_dn10 = assign77180_e116997_d_n10;
        locals.var_chi_b_dn11 = assign77180_e116997_d_n11;
        locals.var_chi_b_dn14 = assign77180_e116997_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign77190_e117006, assign77190_e117006_d_n0, assign77190_e117006_d_n2, assign77190_e117006_d_n4, assign77190_e117006_d_n5, assign77190_e117006_d_n6, assign77190_e117006_d_n7, assign77190_e117006_d_n8, assign77190_e117006_d_n9, assign77190_e117006_d_n10, assign77190_e117006_d_n11, assign77190_e117006_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign77190_e117006;
        locals.var_chi_a_dn0 = assign77190_e117006_d_n0;
        locals.var_chi_a_dn2 = assign77190_e117006_d_n2;
        locals.var_chi_a_dn4 = assign77190_e117006_d_n4;
        locals.var_chi_a_dn5 = assign77190_e117006_d_n5;
        locals.var_chi_a_dn6 = assign77190_e117006_d_n6;
        locals.var_chi_a_dn7 = assign77190_e117006_d_n7;
        locals.var_chi_a_dn8 = assign77190_e117006_d_n8;
        locals.var_chi_a_dn9 = assign77190_e117006_d_n9;
        locals.var_chi_a_dn10 = assign77190_e117006_d_n10;
        locals.var_chi_a_dn11 = assign77190_e117006_d_n11;
        locals.var_chi_a_dn14 = assign77190_e117006_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign77200_e117009: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1796 = assign77200_e117009;
        locals.var_guard1796_rv = 0.0;

        let assign77210_e117014: f64 = (0.2 * locals.var_chi_b);
        let assign77210_e117015: f64 = (locals.var_chi_b - assign77210_e117014);
        let assign77210_e117019: f64 = (0.2 * locals.var_chi_b);
        let assign77210_e117022: f64 = if ((locals.var_chi_a > assign77210_e117015) && (assign77210_e117019 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1797 = assign77210_e117022;
        locals.var_guard1797_rv = 0.0;

        let (assign77220_e117041, assign77220_e117041_d_n0, assign77220_e117041_d_n2, assign77220_e117041_d_n4, assign77220_e117041_d_n5, assign77220_e117041_d_n6, assign77220_e117041_d_n7, assign77220_e117041_d_n8, assign77220_e117041_d_n9, assign77220_e117041_d_n10, assign77220_e117041_d_n11, assign77220_e117041_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77220_e117035: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign77220_e117038: f64 = (0.2 * locals.var_chi_b);
        let assign77220_e117039: f64 = (assign77220_e117035 + assign77220_e117038);
        (assign77220_e117039, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77220_e117041;
        locals.var_tmf1_dn0 = assign77220_e117041_d_n0;
        locals.var_tmf1_dn2 = assign77220_e117041_d_n2;
        locals.var_tmf1_dn4 = assign77220_e117041_d_n4;
        locals.var_tmf1_dn5 = assign77220_e117041_d_n5;
        locals.var_tmf1_dn6 = assign77220_e117041_d_n6;
        locals.var_tmf1_dn7 = assign77220_e117041_d_n7;
        locals.var_tmf1_dn8 = assign77220_e117041_d_n8;
        locals.var_tmf1_dn9 = assign77220_e117041_d_n9;
        locals.var_tmf1_dn10 = assign77220_e117041_d_n10;
        locals.var_tmf1_dn11 = assign77220_e117041_d_n11;
        locals.var_tmf1_dn14 = assign77220_e117041_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign77230_e117056, assign77230_e117056_d_n0, assign77230_e117056_d_n2, assign77230_e117056_d_n4, assign77230_e117056_d_n5, assign77230_e117056_d_n6, assign77230_e117056_d_n7, assign77230_e117056_d_n8, assign77230_e117056_d_n9, assign77230_e117056_d_n10, assign77230_e117056_d_n11, assign77230_e117056_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77230_e117054: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign77230_e117054, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign77230_e117056;
        locals.var_x2_dn0 = assign77230_e117056_d_n0;
        locals.var_x2_dn2 = assign77230_e117056_d_n2;
        locals.var_x2_dn4 = assign77230_e117056_d_n4;
        locals.var_x2_dn5 = assign77230_e117056_d_n5;
        locals.var_x2_dn6 = assign77230_e117056_d_n6;
        locals.var_x2_dn7 = assign77230_e117056_d_n7;
        locals.var_x2_dn8 = assign77230_e117056_d_n8;
        locals.var_x2_dn9 = assign77230_e117056_d_n9;
        locals.var_x2_dn10 = assign77230_e117056_d_n10;
        locals.var_x2_dn11 = assign77230_e117056_d_n11;
        locals.var_x2_dn14 = assign77230_e117056_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign77240_e117075, assign77240_e117075_d_n0, assign77240_e117075_d_n2, assign77240_e117075_d_n4, assign77240_e117075_d_n5, assign77240_e117075_d_n6, assign77240_e117075_d_n7, assign77240_e117075_d_n8, assign77240_e117075_d_n9, assign77240_e117075_d_n10, assign77240_e117075_d_n11, assign77240_e117075_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77240_e117069: f64 = (0.2 * locals.var_chi_b);
        let assign77240_e117072: f64 = (0.2 * locals.var_chi_b);
        let assign77240_e117073: f64 = (assign77240_e117069 * assign77240_e117072);
        (assign77240_e117073, (((0.2 * locals.var_chi_b_dn0) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign77240_e117075;
        locals.var_xmax2_dn0 = assign77240_e117075_d_n0;
        locals.var_xmax2_dn2 = assign77240_e117075_d_n2;
        locals.var_xmax2_dn4 = assign77240_e117075_d_n4;
        locals.var_xmax2_dn5 = assign77240_e117075_d_n5;
        locals.var_xmax2_dn6 = assign77240_e117075_d_n6;
        locals.var_xmax2_dn7 = assign77240_e117075_d_n7;
        locals.var_xmax2_dn8 = assign77240_e117075_d_n8;
        locals.var_xmax2_dn9 = assign77240_e117075_d_n9;
        locals.var_xmax2_dn10 = assign77240_e117075_d_n10;
        locals.var_xmax2_dn11 = assign77240_e117075_d_n11;
        locals.var_xmax2_dn14 = assign77240_e117075_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign77250_e117088, assign77250_e117088_d_n0, assign77250_e117088_d_n2, assign77250_e117088_d_n4, assign77250_e117088_d_n5, assign77250_e117088_d_n6, assign77250_e117088_d_n7, assign77250_e117088_d_n8, assign77250_e117088_d_n9, assign77250_e117088_d_n10, assign77250_e117088_d_n11, assign77250_e117088_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77250_e117088;
        locals.var_xp_dn0 = assign77250_e117088_d_n0;
        locals.var_xp_dn2 = assign77250_e117088_d_n2;
        locals.var_xp_dn4 = assign77250_e117088_d_n4;
        locals.var_xp_dn5 = assign77250_e117088_d_n5;
        locals.var_xp_dn6 = assign77250_e117088_d_n6;
        locals.var_xp_dn7 = assign77250_e117088_d_n7;
        locals.var_xp_dn8 = assign77250_e117088_d_n8;
        locals.var_xp_dn9 = assign77250_e117088_d_n9;
        locals.var_xp_dn10 = assign77250_e117088_d_n10;
        locals.var_xp_dn11 = assign77250_e117088_d_n11;
        locals.var_xp_dn14 = assign77250_e117088_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77260_e117101, assign77260_e117101_d_n0, assign77260_e117101_d_n2, assign77260_e117101_d_n4, assign77260_e117101_d_n5, assign77260_e117101_d_n6, assign77260_e117101_d_n7, assign77260_e117101_d_n8, assign77260_e117101_d_n9, assign77260_e117101_d_n10, assign77260_e117101_d_n11, assign77260_e117101_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77260_e117101;
        locals.var_xmp_dn0 = assign77260_e117101_d_n0;
        locals.var_xmp_dn2 = assign77260_e117101_d_n2;
        locals.var_xmp_dn4 = assign77260_e117101_d_n4;
        locals.var_xmp_dn5 = assign77260_e117101_d_n5;
        locals.var_xmp_dn6 = assign77260_e117101_d_n6;
        locals.var_xmp_dn7 = assign77260_e117101_d_n7;
        locals.var_xmp_dn8 = assign77260_e117101_d_n8;
        locals.var_xmp_dn9 = assign77260_e117101_d_n9;
        locals.var_xmp_dn10 = assign77260_e117101_d_n10;
        locals.var_xmp_dn11 = assign77260_e117101_d_n11;
        locals.var_xmp_dn14 = assign77260_e117101_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77270_e117114,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77270_e117114;
        locals.var_m0_rv = 0.0;

        let (assign77280_e117127,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77280_e117127;
        locals.var_mm_rv = 0.0;

        let (assign77290_e117140, assign77290_e117140_d_n0, assign77290_e117140_d_n2, assign77290_e117140_d_n4, assign77290_e117140_d_n5, assign77290_e117140_d_n6, assign77290_e117140_d_n7, assign77290_e117140_d_n8, assign77290_e117140_d_n9, assign77290_e117140_d_n10, assign77290_e117140_d_n11, assign77290_e117140_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77290_e117140;
        locals.var_arg_dn0 = assign77290_e117140_d_n0;
        locals.var_arg_dn2 = assign77290_e117140_d_n2;
        locals.var_arg_dn4 = assign77290_e117140_d_n4;
        locals.var_arg_dn5 = assign77290_e117140_d_n5;
        locals.var_arg_dn6 = assign77290_e117140_d_n6;
        locals.var_arg_dn7 = assign77290_e117140_d_n7;
        locals.var_arg_dn8 = assign77290_e117140_d_n8;
        locals.var_arg_dn9 = assign77290_e117140_d_n9;
        locals.var_arg_dn10 = assign77290_e117140_d_n10;
        locals.var_arg_dn11 = assign77290_e117140_d_n11;
        locals.var_arg_dn14 = assign77290_e117140_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign77300_e117153, assign77300_e117153_d_n0, assign77300_e117153_d_n2, assign77300_e117153_d_n4, assign77300_e117153_d_n5, assign77300_e117153_d_n6, assign77300_e117153_d_n7, assign77300_e117153_d_n8, assign77300_e117153_d_n9, assign77300_e117153_d_n10, assign77300_e117153_d_n11, assign77300_e117153_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77300_e117153;
        locals.var_dnm_dn0 = assign77300_e117153_d_n0;
        locals.var_dnm_dn2 = assign77300_e117153_d_n2;
        locals.var_dnm_dn4 = assign77300_e117153_d_n4;
        locals.var_dnm_dn5 = assign77300_e117153_d_n5;
        locals.var_dnm_dn6 = assign77300_e117153_d_n6;
        locals.var_dnm_dn7 = assign77300_e117153_d_n7;
        locals.var_dnm_dn8 = assign77300_e117153_d_n8;
        locals.var_dnm_dn9 = assign77300_e117153_d_n9;
        locals.var_dnm_dn10 = assign77300_e117153_d_n10;
        locals.var_dnm_dn11 = assign77300_e117153_d_n11;
        locals.var_dnm_dn14 = assign77300_e117153_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77310_e117168, assign77310_e117168_d_n0, assign77310_e117168_d_n2, assign77310_e117168_d_n4, assign77310_e117168_d_n5, assign77310_e117168_d_n6, assign77310_e117168_d_n7, assign77310_e117168_d_n8, assign77310_e117168_d_n9, assign77310_e117168_d_n10, assign77310_e117168_d_n11, assign77310_e117168_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77310_e117166: f64 = (locals.var_xp * locals.var_x2);
        (assign77310_e117166, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77310_e117168;
        locals.var_xp_dn0 = assign77310_e117168_d_n0;
        locals.var_xp_dn2 = assign77310_e117168_d_n2;
        locals.var_xp_dn4 = assign77310_e117168_d_n4;
        locals.var_xp_dn5 = assign77310_e117168_d_n5;
        locals.var_xp_dn6 = assign77310_e117168_d_n6;
        locals.var_xp_dn7 = assign77310_e117168_d_n7;
        locals.var_xp_dn8 = assign77310_e117168_d_n8;
        locals.var_xp_dn9 = assign77310_e117168_d_n9;
        locals.var_xp_dn10 = assign77310_e117168_d_n10;
        locals.var_xp_dn11 = assign77310_e117168_d_n11;
        locals.var_xp_dn14 = assign77310_e117168_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77320_e117183, assign77320_e117183_d_n0, assign77320_e117183_d_n2, assign77320_e117183_d_n4, assign77320_e117183_d_n5, assign77320_e117183_d_n6, assign77320_e117183_d_n7, assign77320_e117183_d_n8, assign77320_e117183_d_n9, assign77320_e117183_d_n10, assign77320_e117183_d_n11, assign77320_e117183_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77320_e117181: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77320_e117181, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77320_e117183;
        locals.var_xmp_dn0 = assign77320_e117183_d_n0;
        locals.var_xmp_dn2 = assign77320_e117183_d_n2;
        locals.var_xmp_dn4 = assign77320_e117183_d_n4;
        locals.var_xmp_dn5 = assign77320_e117183_d_n5;
        locals.var_xmp_dn6 = assign77320_e117183_d_n6;
        locals.var_xmp_dn7 = assign77320_e117183_d_n7;
        locals.var_xmp_dn8 = assign77320_e117183_d_n8;
        locals.var_xmp_dn9 = assign77320_e117183_d_n9;
        locals.var_xmp_dn10 = assign77320_e117183_d_n10;
        locals.var_xmp_dn11 = assign77320_e117183_d_n11;
        locals.var_xmp_dn14 = assign77320_e117183_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77330_e117198, assign77330_e117198_d_n0, assign77330_e117198_d_n2, assign77330_e117198_d_n4, assign77330_e117198_d_n5, assign77330_e117198_d_n6, assign77330_e117198_d_n7, assign77330_e117198_d_n8, assign77330_e117198_d_n9, assign77330_e117198_d_n10, assign77330_e117198_d_n11, assign77330_e117198_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77330_e117196: f64 = (locals.var_xp * locals.var_x2);
        (assign77330_e117196, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77330_e117198;
        locals.var_xp_dn0 = assign77330_e117198_d_n0;
        locals.var_xp_dn2 = assign77330_e117198_d_n2;
        locals.var_xp_dn4 = assign77330_e117198_d_n4;
        locals.var_xp_dn5 = assign77330_e117198_d_n5;
        locals.var_xp_dn6 = assign77330_e117198_d_n6;
        locals.var_xp_dn7 = assign77330_e117198_d_n7;
        locals.var_xp_dn8 = assign77330_e117198_d_n8;
        locals.var_xp_dn9 = assign77330_e117198_d_n9;
        locals.var_xp_dn10 = assign77330_e117198_d_n10;
        locals.var_xp_dn11 = assign77330_e117198_d_n11;
        locals.var_xp_dn14 = assign77330_e117198_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77340_e117213, assign77340_e117213_d_n0, assign77340_e117213_d_n2, assign77340_e117213_d_n4, assign77340_e117213_d_n5, assign77340_e117213_d_n6, assign77340_e117213_d_n7, assign77340_e117213_d_n8, assign77340_e117213_d_n9, assign77340_e117213_d_n10, assign77340_e117213_d_n11, assign77340_e117213_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77340_e117211: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77340_e117211, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77340_e117213;
        locals.var_xmp_dn0 = assign77340_e117213_d_n0;
        locals.var_xmp_dn2 = assign77340_e117213_d_n2;
        locals.var_xmp_dn4 = assign77340_e117213_d_n4;
        locals.var_xmp_dn5 = assign77340_e117213_d_n5;
        locals.var_xmp_dn6 = assign77340_e117213_d_n6;
        locals.var_xmp_dn7 = assign77340_e117213_d_n7;
        locals.var_xmp_dn8 = assign77340_e117213_d_n8;
        locals.var_xmp_dn9 = assign77340_e117213_d_n9;
        locals.var_xmp_dn10 = assign77340_e117213_d_n10;
        locals.var_xmp_dn11 = assign77340_e117213_d_n11;
        locals.var_xmp_dn14 = assign77340_e117213_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77350_e117228, assign77350_e117228_d_n0, assign77350_e117228_d_n2, assign77350_e117228_d_n4, assign77350_e117228_d_n5, assign77350_e117228_d_n6, assign77350_e117228_d_n7, assign77350_e117228_d_n8, assign77350_e117228_d_n9, assign77350_e117228_d_n10, assign77350_e117228_d_n11, assign77350_e117228_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77350_e117226: f64 = (locals.var_xp + locals.var_xmp);
        (assign77350_e117226, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77350_e117228;
        locals.var_arg_dn0 = assign77350_e117228_d_n0;
        locals.var_arg_dn2 = assign77350_e117228_d_n2;
        locals.var_arg_dn4 = assign77350_e117228_d_n4;
        locals.var_arg_dn5 = assign77350_e117228_d_n5;
        locals.var_arg_dn6 = assign77350_e117228_d_n6;
        locals.var_arg_dn7 = assign77350_e117228_d_n7;
        locals.var_arg_dn8 = assign77350_e117228_d_n8;
        locals.var_arg_dn9 = assign77350_e117228_d_n9;
        locals.var_arg_dn10 = assign77350_e117228_d_n10;
        locals.var_arg_dn11 = assign77350_e117228_d_n11;
        locals.var_arg_dn14 = assign77350_e117228_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign77360_e117241, assign77360_e117241_d_n0, assign77360_e117241_d_n2, assign77360_e117241_d_n4, assign77360_e117241_d_n5, assign77360_e117241_d_n6, assign77360_e117241_d_n7, assign77360_e117241_d_n8, assign77360_e117241_d_n9, assign77360_e117241_d_n10, assign77360_e117241_d_n11, assign77360_e117241_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77360_e117241;
        locals.var_dnm_dn0 = assign77360_e117241_d_n0;
        locals.var_dnm_dn2 = assign77360_e117241_d_n2;
        locals.var_dnm_dn4 = assign77360_e117241_d_n4;
        locals.var_dnm_dn5 = assign77360_e117241_d_n5;
        locals.var_dnm_dn6 = assign77360_e117241_d_n6;
        locals.var_dnm_dn7 = assign77360_e117241_d_n7;
        locals.var_dnm_dn8 = assign77360_e117241_d_n8;
        locals.var_dnm_dn9 = assign77360_e117241_d_n9;
        locals.var_dnm_dn10 = assign77360_e117241_d_n10;
        locals.var_dnm_dn11 = assign77360_e117241_d_n11;
        locals.var_dnm_dn14 = assign77360_e117241_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign77370_e117256: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1798 = assign77370_e117256;
        locals.var_guard1798_rv = 0.0;

        let assign77380_e117259: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1799 = assign77380_e117259;
        locals.var_guard1799_rv = 0.0;

        let (assign77390_e117276,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77390_e117276;
        locals.var_mm_rv = 0.0;

        let assign77400_e117279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1800 = assign77400_e117279;
        locals.var_guard1800_rv = 0.0;

        let (assign77410_e117299,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77410_e117299;
        locals.var_mm_rv = 0.0;

        let assign77420_e117302: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1801 = assign77420_e117302;
        locals.var_guard1801_rv = 0.0;

        let (assign77430_e117325,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77430_e117325;
        locals.var_mm_rv = 0.0;

        let assign77440_e117328: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1802 = assign77440_e117328;
        locals.var_guard1802_rv = 0.0;

        let (assign77450_e117354,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 == 0.0)) && (locals.var_guard1802 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77450_e117354;
        locals.var_mm_rv = 0.0;

        let (assign77460_e117369,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77460_e117369;
        locals.var_m0_rv = 0.0;

        let mut assign77470_loop_guard: usize = 0;
        while {
            let assign77470_cond_e117385: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign77470_cond_e117385 != 0.0
        } {
            assign77470_loop_guard += 1;
            assert!(assign77470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign77470_body0_e117401, assign77470_body0_e117401_d_n0, assign77470_body0_e117401_d_n2, assign77470_body0_e117401_d_n4, assign77470_body0_e117401_d_n5, assign77470_body0_e117401_d_n6, assign77470_body0_e117401_d_n7, assign77470_body0_e117401_d_n8, assign77470_body0_e117401_d_n9, assign77470_body0_e117401_d_n10, assign77470_body0_e117401_d_n11, assign77470_body0_e117401_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        let assign77470_body0_e117399: f64 = (locals.var_dnm).sqrt();
        (assign77470_body0_e117399, (locals.var_dnm_dn0 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn2 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn4 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn5 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn6 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn7 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn8 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn9 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn10 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn11 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn14 / (2.0 * assign77470_body0_e117399)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign77470_body0_e117401;
            locals.var_dnm_dn0 = assign77470_body0_e117401_d_n0;
            locals.var_dnm_dn2 = assign77470_body0_e117401_d_n2;
            locals.var_dnm_dn4 = assign77470_body0_e117401_d_n4;
            locals.var_dnm_dn5 = assign77470_body0_e117401_d_n5;
            locals.var_dnm_dn6 = assign77470_body0_e117401_d_n6;
            locals.var_dnm_dn7 = assign77470_body0_e117401_d_n7;
            locals.var_dnm_dn8 = assign77470_body0_e117401_d_n8;
            locals.var_dnm_dn9 = assign77470_body0_e117401_d_n9;
            locals.var_dnm_dn10 = assign77470_body0_e117401_d_n10;
            locals.var_dnm_dn11 = assign77470_body0_e117401_d_n11;
            locals.var_dnm_dn14 = assign77470_body0_e117401_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign77470_body1_e117418,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        let assign77470_body1_e117416: f64 = (locals.var_m0 + 1.0);
        (assign77470_body1_e117416,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign77470_body1_e117418;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_293(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77480_e117445, assign77480_e117445_d_n0, assign77480_e117445_d_n2, assign77480_e117445_d_n4, assign77480_e117445_d_n5, assign77480_e117445_d_n6, assign77480_e117445_d_n7, assign77480_e117445_d_n8, assign77480_e117445_d_n9, assign77480_e117445_d_n10, assign77480_e117445_d_n11, assign77480_e117445_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 == 0.0)) {
        let (assign77480_e117443, assign77480_e117443_d_n0, assign77480_e117443_d_n2, assign77480_e117443_d_n4, assign77480_e117443_d_n5, assign77480_e117443_d_n6, assign77480_e117443_d_n7, assign77480_e117443_d_n8, assign77480_e117443_d_n9, assign77480_e117443_d_n10, assign77480_e117443_d_n11, assign77480_e117443_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign77480_e117440: f64 = (2.0 * 2.0);
                let assign77480_e117441: f64 = (1.0 / assign77480_e117440);
                let assign77480_e117442: f64 = (locals.var_dnm).powf(assign77480_e117441);
                (assign77480_e117442, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn0)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn2)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn4)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn5)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn6)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn7)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn8)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn9)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn10)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn11)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn14)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign77480_e117443, assign77480_e117443_d_n0, assign77480_e117443_d_n2, assign77480_e117443_d_n4, assign77480_e117443_d_n5, assign77480_e117443_d_n6, assign77480_e117443_d_n7, assign77480_e117443_d_n8, assign77480_e117443_d_n9, assign77480_e117443_d_n10, assign77480_e117443_d_n11, assign77480_e117443_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77480_e117445;
        locals.var_dnm_dn0 = assign77480_e117445_d_n0;
        locals.var_dnm_dn2 = assign77480_e117445_d_n2;
        locals.var_dnm_dn4 = assign77480_e117445_d_n4;
        locals.var_dnm_dn5 = assign77480_e117445_d_n5;
        locals.var_dnm_dn6 = assign77480_e117445_d_n6;
        locals.var_dnm_dn7 = assign77480_e117445_d_n7;
        locals.var_dnm_dn8 = assign77480_e117445_d_n8;
        locals.var_dnm_dn9 = assign77480_e117445_d_n9;
        locals.var_dnm_dn10 = assign77480_e117445_d_n10;
        locals.var_dnm_dn11 = assign77480_e117445_d_n11;
        locals.var_dnm_dn14 = assign77480_e117445_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77490_e117460, assign77490_e117460_d_n0, assign77490_e117460_d_n2, assign77490_e117460_d_n4, assign77490_e117460_d_n5, assign77490_e117460_d_n6, assign77490_e117460_d_n7, assign77490_e117460_d_n8, assign77490_e117460_d_n9, assign77490_e117460_d_n10, assign77490_e117460_d_n11, assign77490_e117460_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77490_e117458: f64 = (1.0 / locals.var_dnm);
        (assign77490_e117458, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77490_e117460;
        locals.var_dnm_dn0 = assign77490_e117460_d_n0;
        locals.var_dnm_dn2 = assign77490_e117460_d_n2;
        locals.var_dnm_dn4 = assign77490_e117460_d_n4;
        locals.var_dnm_dn5 = assign77490_e117460_d_n5;
        locals.var_dnm_dn6 = assign77490_e117460_d_n6;
        locals.var_dnm_dn7 = assign77490_e117460_d_n7;
        locals.var_dnm_dn8 = assign77490_e117460_d_n8;
        locals.var_dnm_dn9 = assign77490_e117460_d_n9;
        locals.var_dnm_dn10 = assign77490_e117460_d_n10;
        locals.var_dnm_dn11 = assign77490_e117460_d_n11;
        locals.var_dnm_dn14 = assign77490_e117460_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77500_e117479, assign77500_e117479_d_n0, assign77500_e117479_d_n2, assign77500_e117479_d_n4, assign77500_e117479_d_n5, assign77500_e117479_d_n6, assign77500_e117479_d_n7, assign77500_e117479_d_n8, assign77500_e117479_d_n9, assign77500_e117479_d_n10, assign77500_e117479_d_n11, assign77500_e117479_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77500_e117474: f64 = (0.2 * locals.var_chi_b);
        let assign77500_e117475: f64 = (locals.var_tmf1 * assign77500_e117474);
        let assign77500_e117477: f64 = (assign77500_e117475 * locals.var_dnm);
        (assign77500_e117477, ((((locals.var_tmf1_dn0 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign77500_e117479;
        locals.var_tmf0_dn0 = assign77500_e117479_d_n0;
        locals.var_tmf0_dn2 = assign77500_e117479_d_n2;
        locals.var_tmf0_dn4 = assign77500_e117479_d_n4;
        locals.var_tmf0_dn5 = assign77500_e117479_d_n5;
        locals.var_tmf0_dn6 = assign77500_e117479_d_n6;
        locals.var_tmf0_dn7 = assign77500_e117479_d_n7;
        locals.var_tmf0_dn8 = assign77500_e117479_d_n8;
        locals.var_tmf0_dn9 = assign77500_e117479_d_n9;
        locals.var_tmf0_dn10 = assign77500_e117479_d_n10;
        locals.var_tmf0_dn11 = assign77500_e117479_d_n11;
        locals.var_tmf0_dn14 = assign77500_e117479_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign77510_e117500, assign77510_e117500_d_n0, assign77510_e117500_d_n2, assign77510_e117500_d_n4, assign77510_e117500_d_n5, assign77510_e117500_d_n6, assign77510_e117500_d_n7, assign77510_e117500_d_n8, assign77510_e117500_d_n9, assign77510_e117500_d_n10, assign77510_e117500_d_n11, assign77510_e117500_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77510_e117492: f64 = (0.2 * locals.var_chi_b);
        let assign77510_e117494: f64 = (assign77510_e117492 * locals.var_xmp);
        let assign77510_e117496: f64 = (assign77510_e117494 * locals.var_dnm);
        let assign77510_e117498: f64 = (assign77510_e117496 / locals.var_arg);
        (assign77510_e117498, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn0)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn2)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn4)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn5)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn6)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn7)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn8)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn9)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn10)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn11)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn14)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77510_e117500;
        locals.var_t1_dn0 = assign77510_e117500_d_n0;
        locals.var_t1_dn2 = assign77510_e117500_d_n2;
        locals.var_t1_dn4 = assign77510_e117500_d_n4;
        locals.var_t1_dn5 = assign77510_e117500_d_n5;
        locals.var_t1_dn6 = assign77510_e117500_d_n6;
        locals.var_t1_dn7 = assign77510_e117500_d_n7;
        locals.var_t1_dn8 = assign77510_e117500_d_n8;
        locals.var_t1_dn9 = assign77510_e117500_d_n9;
        locals.var_t1_dn10 = assign77510_e117500_d_n10;
        locals.var_t1_dn11 = assign77510_e117500_d_n11;
        locals.var_t1_dn14 = assign77510_e117500_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77520_e117519, assign77520_e117519_d_n0, assign77520_e117519_d_n2, assign77520_e117519_d_n4, assign77520_e117519_d_n5, assign77520_e117519_d_n6, assign77520_e117519_d_n7, assign77520_e117519_d_n8, assign77520_e117519_d_n9, assign77520_e117519_d_n10, assign77520_e117519_d_n11, assign77520_e117519_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77520_e117514: f64 = (0.2 * locals.var_chi_b);
        let assign77520_e117515: f64 = (locals.var_chi_b - assign77520_e117514);
        let assign77520_e117517: f64 = (assign77520_e117515 + locals.var_tmf0);
        (assign77520_e117517, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77520_e117519;
        locals.var_chi_dn0 = assign77520_e117519_d_n0;
        locals.var_chi_dn2 = assign77520_e117519_d_n2;
        locals.var_chi_dn4 = assign77520_e117519_d_n4;
        locals.var_chi_dn5 = assign77520_e117519_d_n5;
        locals.var_chi_dn6 = assign77520_e117519_d_n6;
        locals.var_chi_dn7 = assign77520_e117519_d_n7;
        locals.var_chi_dn8 = assign77520_e117519_d_n8;
        locals.var_chi_dn9 = assign77520_e117519_d_n9;
        locals.var_chi_dn10 = assign77520_e117519_d_n10;
        locals.var_chi_dn11 = assign77520_e117519_d_n11;
        locals.var_chi_dn14 = assign77520_e117519_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77530_e117532, assign77530_e117532_d_n0, assign77530_e117532_d_n2, assign77530_e117532_d_n4, assign77530_e117532_d_n5, assign77530_e117532_d_n6, assign77530_e117532_d_n7, assign77530_e117532_d_n8, assign77530_e117532_d_n9, assign77530_e117532_d_n10, assign77530_e117532_d_n11, assign77530_e117532_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77530_e117532;
        locals.var_t1_dn0 = assign77530_e117532_d_n0;
        locals.var_t1_dn2 = assign77530_e117532_d_n2;
        locals.var_t1_dn4 = assign77530_e117532_d_n4;
        locals.var_t1_dn5 = assign77530_e117532_d_n5;
        locals.var_t1_dn6 = assign77530_e117532_d_n6;
        locals.var_t1_dn7 = assign77530_e117532_d_n7;
        locals.var_t1_dn8 = assign77530_e117532_d_n8;
        locals.var_t1_dn9 = assign77530_e117532_d_n9;
        locals.var_t1_dn10 = assign77530_e117532_d_n10;
        locals.var_t1_dn11 = assign77530_e117532_d_n11;
        locals.var_t1_dn14 = assign77530_e117532_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77540_e117546, assign77540_e117546_d_n0, assign77540_e117546_d_n2, assign77540_e117546_d_n4, assign77540_e117546_d_n5, assign77540_e117546_d_n6, assign77540_e117546_d_n7, assign77540_e117546_d_n8, assign77540_e117546_d_n9, assign77540_e117546_d_n10, assign77540_e117546_d_n11, assign77540_e117546_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77540_e117546;
        locals.var_chi_dn0 = assign77540_e117546_d_n0;
        locals.var_chi_dn2 = assign77540_e117546_d_n2;
        locals.var_chi_dn4 = assign77540_e117546_d_n4;
        locals.var_chi_dn5 = assign77540_e117546_d_n5;
        locals.var_chi_dn6 = assign77540_e117546_d_n6;
        locals.var_chi_dn7 = assign77540_e117546_d_n7;
        locals.var_chi_dn8 = assign77540_e117546_d_n8;
        locals.var_chi_dn9 = assign77540_e117546_d_n9;
        locals.var_chi_dn10 = assign77540_e117546_d_n10;
        locals.var_chi_dn11 = assign77540_e117546_d_n11;
        locals.var_chi_dn14 = assign77540_e117546_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77550_e117560, assign77550_e117560_d_n0, assign77550_e117560_d_n2, assign77550_e117560_d_n4, assign77550_e117560_d_n5, assign77550_e117560_d_n6, assign77550_e117560_d_n7, assign77550_e117560_d_n8, assign77550_e117560_d_n9, assign77550_e117560_d_n10, assign77550_e117560_d_n11, assign77550_e117560_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77550_e117560;
        locals.var_t1_dn0 = assign77550_e117560_d_n0;
        locals.var_t1_dn2 = assign77550_e117560_d_n2;
        locals.var_t1_dn4 = assign77550_e117560_d_n4;
        locals.var_t1_dn5 = assign77550_e117560_d_n5;
        locals.var_t1_dn6 = assign77550_e117560_d_n6;
        locals.var_t1_dn7 = assign77550_e117560_d_n7;
        locals.var_t1_dn8 = assign77550_e117560_d_n8;
        locals.var_t1_dn9 = assign77550_e117560_d_n9;
        locals.var_t1_dn10 = assign77550_e117560_d_n10;
        locals.var_t1_dn11 = assign77550_e117560_d_n11;
        locals.var_t1_dn14 = assign77550_e117560_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77560_e117577, assign77560_e117577_d_n0, assign77560_e117577_d_n2, assign77560_e117577_d_n4, assign77560_e117577_d_n5, assign77560_e117577_d_n6, assign77560_e117577_d_n7, assign77560_e117577_d_n8, assign77560_e117577_d_n9, assign77560_e117577_d_n10, assign77560_e117577_d_n11, assign77560_e117577_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 == 0.0)) {
        let (assign77560_e117575, assign77560_e117575_d_n0, assign77560_e117575_d_n2, assign77560_e117575_d_n4, assign77560_e117575_d_n5, assign77560_e117575_d_n6, assign77560_e117575_d_n7, assign77560_e117575_d_n8, assign77560_e117575_d_n9, assign77560_e117575_d_n10, assign77560_e117575_d_n11, assign77560_e117575_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign77560_e117575, assign77560_e117575_d_n0, assign77560_e117575_d_n2, assign77560_e117575_d_n4, assign77560_e117575_d_n5, assign77560_e117575_d_n6, assign77560_e117575_d_n7, assign77560_e117575_d_n8, assign77560_e117575_d_n9, assign77560_e117575_d_n10, assign77560_e117575_d_n11, assign77560_e117575_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77560_e117577;
        locals.var_chi_dn0 = assign77560_e117577_d_n0;
        locals.var_chi_dn2 = assign77560_e117577_d_n2;
        locals.var_chi_dn4 = assign77560_e117577_d_n4;
        locals.var_chi_dn5 = assign77560_e117577_d_n5;
        locals.var_chi_dn6 = assign77560_e117577_d_n6;
        locals.var_chi_dn7 = assign77560_e117577_d_n7;
        locals.var_chi_dn8 = assign77560_e117577_d_n8;
        locals.var_chi_dn9 = assign77560_e117577_d_n9;
        locals.var_chi_dn10 = assign77560_e117577_d_n10;
        locals.var_chi_dn11 = assign77560_e117577_d_n11;
        locals.var_chi_dn14 = assign77560_e117577_d_n14;
        locals.var_chi_rv = 0.0;

        let assign77570_e117580: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1803 = assign77570_e117580;
        locals.var_guard1803_rv = 0.0;

        let (assign77580_e117593, assign77580_e117593_d_n0, assign77580_e117593_d_n2, assign77580_e117593_d_n4, assign77580_e117593_d_n5, assign77580_e117593_d_n6, assign77580_e117593_d_n7, assign77580_e117593_d_n8, assign77580_e117593_d_n9, assign77580_e117593_d_n10, assign77580_e117593_d_n11, assign77580_e117593_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77580_e117589: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77580_e117591: f64 = (assign77580_e117589 - locals.var_vxbgmtcl);
        (assign77580_e117591, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77580_e117593;
        locals.var_ps0ld_dn0 = assign77580_e117593_d_n0;
        locals.var_ps0ld_dn2 = assign77580_e117593_d_n2;
        locals.var_ps0ld_dn4 = assign77580_e117593_d_n4;
        locals.var_ps0ld_dn5 = assign77580_e117593_d_n5;
        locals.var_ps0ld_dn6 = assign77580_e117593_d_n6;
        locals.var_ps0ld_dn7 = assign77580_e117593_d_n7;
        locals.var_ps0ld_dn8 = assign77580_e117593_d_n8;
        locals.var_ps0ld_dn9 = assign77580_e117593_d_n9;
        locals.var_ps0ld_dn10 = assign77580_e117593_d_n10;
        locals.var_ps0ld_dn11 = assign77580_e117593_d_n11;
        locals.var_ps0ld_dn14 = assign77580_e117593_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign77590_e117596: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1804 = assign77590_e117596;
        locals.var_guard1804_rv = 0.0;

        let (assign77600_e117609, assign77600_e117609_d_n0, assign77600_e117609_d_n2, assign77600_e117609_d_n4, assign77600_e117609_d_n5, assign77600_e117609_d_n6, assign77600_e117609_d_n7, assign77600_e117609_d_n8, assign77600_e117609_d_n9, assign77600_e117609_d_n10, assign77600_e117609_d_n11, assign77600_e117609_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        let assign77600_e117607: f64 = (p.p334 - locals.var_wdep_func);
        (assign77600_e117607, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77600_e117609;
        locals.var_t2_dn0 = assign77600_e117609_d_n0;
        locals.var_t2_dn2 = assign77600_e117609_d_n2;
        locals.var_t2_dn4 = assign77600_e117609_d_n4;
        locals.var_t2_dn5 = assign77600_e117609_d_n5;
        locals.var_t2_dn6 = assign77600_e117609_d_n6;
        locals.var_t2_dn7 = assign77600_e117609_d_n7;
        locals.var_t2_dn8 = assign77600_e117609_d_n8;
        locals.var_t2_dn9 = assign77600_e117609_d_n9;
        locals.var_t2_dn10 = assign77600_e117609_d_n10;
        locals.var_t2_dn11 = assign77600_e117609_d_n11;
        locals.var_t2_dn14 = assign77600_e117609_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77610_e117634, assign77610_e117634_d_n0, assign77610_e117634_d_n2, assign77610_e117634_d_n4, assign77610_e117634_d_n5, assign77610_e117634_d_n6, assign77610_e117634_d_n7, assign77610_e117634_d_n8, assign77610_e117634_d_n9, assign77610_e117634_d_n10, assign77610_e117634_d_n11, assign77610_e117634_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77610_e117621: f64 = (locals.var_vdsi + p.p137);
        let assign77610_e117624: f64 = (locals.var_vdsi + p.p137);
        let assign77610_e117625: f64 = (assign77610_e117621 * assign77610_e117624);
        let assign77610_e117628: f64 = (4.0 * 0.1);
        let assign77610_e117630: f64 = (assign77610_e117628 * 0.1);
        let assign77610_e117631: f64 = (assign77610_e117625 + assign77610_e117630);
        let assign77610_e117632: f64 = (assign77610_e117631).sqrt();
        (assign77610_e117632, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign77610_e117624) + (assign77610_e117621 * locals.var_vdsi_dn6)) / (2.0 * assign77610_e117632)), 0.0, (((locals.var_vdsi_dn8 * assign77610_e117624) + (assign77610_e117621 * locals.var_vdsi_dn8)) / (2.0 * assign77610_e117632)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77610_e117634;
        locals.var_tmf2_dn0 = assign77610_e117634_d_n0;
        locals.var_tmf2_dn2 = assign77610_e117634_d_n2;
        locals.var_tmf2_dn4 = assign77610_e117634_d_n4;
        locals.var_tmf2_dn5 = assign77610_e117634_d_n5;
        locals.var_tmf2_dn6 = assign77610_e117634_d_n6;
        locals.var_tmf2_dn7 = assign77610_e117634_d_n7;
        locals.var_tmf2_dn8 = assign77610_e117634_d_n8;
        locals.var_tmf2_dn9 = assign77610_e117634_d_n9;
        locals.var_tmf2_dn10 = assign77610_e117634_d_n10;
        locals.var_tmf2_dn11 = assign77610_e117634_d_n11;
        locals.var_tmf2_dn14 = assign77610_e117634_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77620_e117654, assign77620_e117654_d_n0, assign77620_e117654_d_n2, assign77620_e117654_d_n4, assign77620_e117654_d_n5, assign77620_e117654_d_n6, assign77620_e117654_d_n7, assign77620_e117654_d_n8, assign77620_e117654_d_n9, assign77620_e117654_d_n10, assign77620_e117654_d_n11, assign77620_e117654_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77620_e117648: f64 = (locals.var_vdsi + p.p137);
        let assign77620_e117650: f64 = (assign77620_e117648 / locals.var_tmf2);
        let assign77620_e117651: f64 = (1.0 + assign77620_e117650);
        let assign77620_e117652: f64 = (0.5 * assign77620_e117651);
        (assign77620_e117652, (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign77620_e117648 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign77620_e117648 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77620_e117654;
        locals.var_t9_dn0 = assign77620_e117654_d_n0;
        locals.var_t9_dn2 = assign77620_e117654_d_n2;
        locals.var_t9_dn4 = assign77620_e117654_d_n4;
        locals.var_t9_dn5 = assign77620_e117654_d_n5;
        locals.var_t9_dn6 = assign77620_e117654_d_n6;
        locals.var_t9_dn7 = assign77620_e117654_d_n7;
        locals.var_t9_dn8 = assign77620_e117654_d_n8;
        locals.var_t9_dn9 = assign77620_e117654_d_n9;
        locals.var_t9_dn10 = assign77620_e117654_d_n10;
        locals.var_t9_dn11 = assign77620_e117654_d_n11;
        locals.var_t9_dn14 = assign77620_e117654_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77630_e117672, assign77630_e117672_d_n0, assign77630_e117672_d_n2, assign77630_e117672_d_n4, assign77630_e117672_d_n5, assign77630_e117672_d_n6, assign77630_e117672_d_n7, assign77630_e117672_d_n8, assign77630_e117672_d_n9, assign77630_e117672_d_n10, assign77630_e117672_d_n11, assign77630_e117672_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77630_e117667: f64 = (locals.var_vdsi + p.p137);
        let assign77630_e117669: f64 = (assign77630_e117667 + locals.var_tmf2);
        let assign77630_e117670: f64 = (0.5 * assign77630_e117669);
        (assign77630_e117670, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77630_e117672;
        locals.var_t2_dn0 = assign77630_e117672_d_n0;
        locals.var_t2_dn2 = assign77630_e117672_d_n2;
        locals.var_t2_dn4 = assign77630_e117672_d_n4;
        locals.var_t2_dn5 = assign77630_e117672_d_n5;
        locals.var_t2_dn6 = assign77630_e117672_d_n6;
        locals.var_t2_dn7 = assign77630_e117672_d_n7;
        locals.var_t2_dn8 = assign77630_e117672_d_n8;
        locals.var_t2_dn9 = assign77630_e117672_d_n9;
        locals.var_t2_dn10 = assign77630_e117672_d_n10;
        locals.var_t2_dn11 = assign77630_e117672_d_n11;
        locals.var_t2_dn14 = assign77630_e117672_d_n14;
        locals.var_t2_rv = 0.0;

        let assign77640_e117675: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1805 = assign77640_e117675;
        locals.var_guard1805_rv = 0.0;

        let (assign77650_e117689, assign77650_e117689_d_n0, assign77650_e117689_d_n2, assign77650_e117689_d_n4, assign77650_e117689_d_n5, assign77650_e117689_d_n6, assign77650_e117689_d_n7, assign77650_e117689_d_n8, assign77650_e117689_d_n9, assign77650_e117689_d_n10, assign77650_e117689_d_n11, assign77650_e117689_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77650_e117689;
        locals.var_t2_dn0 = assign77650_e117689_d_n0;
        locals.var_t2_dn2 = assign77650_e117689_d_n2;
        locals.var_t2_dn4 = assign77650_e117689_d_n4;
        locals.var_t2_dn5 = assign77650_e117689_d_n5;
        locals.var_t2_dn6 = assign77650_e117689_d_n6;
        locals.var_t2_dn7 = assign77650_e117689_d_n7;
        locals.var_t2_dn8 = assign77650_e117689_d_n8;
        locals.var_t2_dn9 = assign77650_e117689_d_n9;
        locals.var_t2_dn10 = assign77650_e117689_d_n10;
        locals.var_t2_dn11 = assign77650_e117689_d_n11;
        locals.var_t2_dn14 = assign77650_e117689_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77660_e117703, assign77660_e117703_d_n0, assign77660_e117703_d_n2, assign77660_e117703_d_n4, assign77660_e117703_d_n5, assign77660_e117703_d_n6, assign77660_e117703_d_n7, assign77660_e117703_d_n8, assign77660_e117703_d_n9, assign77660_e117703_d_n10, assign77660_e117703_d_n11, assign77660_e117703_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77660_e117703;
        locals.var_t9_dn0 = assign77660_e117703_d_n0;
        locals.var_t9_dn2 = assign77660_e117703_d_n2;
        locals.var_t9_dn4 = assign77660_e117703_d_n4;
        locals.var_t9_dn5 = assign77660_e117703_d_n5;
        locals.var_t9_dn6 = assign77660_e117703_d_n6;
        locals.var_t9_dn7 = assign77660_e117703_d_n7;
        locals.var_t9_dn8 = assign77660_e117703_d_n8;
        locals.var_t9_dn9 = assign77660_e117703_d_n9;
        locals.var_t9_dn10 = assign77660_e117703_d_n10;
        locals.var_t9_dn11 = assign77660_e117703_d_n11;
        locals.var_t9_dn14 = assign77660_e117703_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77670_e117720, assign77670_e117720_d_n0, assign77670_e117720_d_n2, assign77670_e117720_d_n4, assign77670_e117720_d_n5, assign77670_e117720_d_n6, assign77670_e117720_d_n7, assign77670_e117720_d_n8, assign77670_e117720_d_n9, assign77670_e117720_d_n10, assign77670_e117720_d_n11, assign77670_e117720_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77670_e117715: f64 = (locals.var_kjunc * locals.var_t2);
        let assign77670_e117716: f64 = (assign77670_e117715).sqrt();
        let assign77670_e117718: f64 = (assign77670_e117716 * p.p432);
        (assign77670_e117718, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign77670_e117716)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign77670_e117720;
        locals.var_wjunc0_dn0 = assign77670_e117720_d_n0;
        locals.var_wjunc0_dn2 = assign77670_e117720_d_n2;
        locals.var_wjunc0_dn4 = assign77670_e117720_d_n4;
        locals.var_wjunc0_dn5 = assign77670_e117720_d_n5;
        locals.var_wjunc0_dn6 = assign77670_e117720_d_n6;
        locals.var_wjunc0_dn7 = assign77670_e117720_d_n7;
        locals.var_wjunc0_dn8 = assign77670_e117720_d_n8;
        locals.var_wjunc0_dn9 = assign77670_e117720_d_n9;
        locals.var_wjunc0_dn10 = assign77670_e117720_d_n10;
        locals.var_wjunc0_dn11 = assign77670_e117720_d_n11;
        locals.var_wjunc0_dn14 = assign77670_e117720_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign77680_e117734, assign77680_e117734_d_n0, assign77680_e117734_d_n2, assign77680_e117734_d_n4, assign77680_e117734_d_n5, assign77680_e117734_d_n6, assign77680_e117734_d_n7, assign77680_e117734_d_n8, assign77680_e117734_d_n9, assign77680_e117734_d_n10, assign77680_e117734_d_n11, assign77680_e117734_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77680_e117732: f64 = (p.p334 - locals.var_wjunc0);
        (assign77680_e117732, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77680_e117734;
        locals.var_t2_dn0 = assign77680_e117734_d_n0;
        locals.var_t2_dn2 = assign77680_e117734_d_n2;
        locals.var_t2_dn4 = assign77680_e117734_d_n4;
        locals.var_t2_dn5 = assign77680_e117734_d_n5;
        locals.var_t2_dn6 = assign77680_e117734_d_n6;
        locals.var_t2_dn7 = assign77680_e117734_d_n7;
        locals.var_t2_dn8 = assign77680_e117734_d_n8;
        locals.var_t2_dn9 = assign77680_e117734_d_n9;
        locals.var_t2_dn10 = assign77680_e117734_d_n10;
        locals.var_t2_dn11 = assign77680_e117734_d_n11;
        locals.var_t2_dn14 = assign77680_e117734_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77690_e117756, assign77690_e117756_d_n0, assign77690_e117756_d_n2, assign77690_e117756_d_n4, assign77690_e117756_d_n5, assign77690_e117756_d_n6, assign77690_e117756_d_n7, assign77690_e117756_d_n8, assign77690_e117756_d_n9, assign77690_e117756_d_n10, assign77690_e117756_d_n11, assign77690_e117756_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77690_e117743: f64 = (locals.var_t2 * locals.var_t2);
        let assign77690_e117747: f64 = (p.p334 * 0.01);
        let assign77690_e117748: f64 = (4.0 * assign77690_e117747);
        let assign77690_e117751: f64 = (p.p334 * 0.01);
        let assign77690_e117752: f64 = (assign77690_e117748 * assign77690_e117751);
        let assign77690_e117753: f64 = (assign77690_e117743 + assign77690_e117752);
        let assign77690_e117754: f64 = (assign77690_e117753).sqrt();
        (assign77690_e117754, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign77690_e117754)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77690_e117756;
        locals.var_tmf2_dn0 = assign77690_e117756_d_n0;
        locals.var_tmf2_dn2 = assign77690_e117756_d_n2;
        locals.var_tmf2_dn4 = assign77690_e117756_d_n4;
        locals.var_tmf2_dn5 = assign77690_e117756_d_n5;
        locals.var_tmf2_dn6 = assign77690_e117756_d_n6;
        locals.var_tmf2_dn7 = assign77690_e117756_d_n7;
        locals.var_tmf2_dn8 = assign77690_e117756_d_n8;
        locals.var_tmf2_dn9 = assign77690_e117756_d_n9;
        locals.var_tmf2_dn10 = assign77690_e117756_d_n10;
        locals.var_tmf2_dn11 = assign77690_e117756_d_n11;
        locals.var_tmf2_dn14 = assign77690_e117756_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77700_e117771, assign77700_e117771_d_n0, assign77700_e117771_d_n2, assign77700_e117771_d_n4, assign77700_e117771_d_n5, assign77700_e117771_d_n6, assign77700_e117771_d_n7, assign77700_e117771_d_n8, assign77700_e117771_d_n9, assign77700_e117771_d_n10, assign77700_e117771_d_n11, assign77700_e117771_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77700_e117767: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign77700_e117768: f64 = (1.0 + assign77700_e117767);
        let assign77700_e117769: f64 = (0.5 * assign77700_e117768);
        (assign77700_e117769, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77700_e117771;
        locals.var_t9_dn0 = assign77700_e117771_d_n0;
        locals.var_t9_dn2 = assign77700_e117771_d_n2;
        locals.var_t9_dn4 = assign77700_e117771_d_n4;
        locals.var_t9_dn5 = assign77700_e117771_d_n5;
        locals.var_t9_dn6 = assign77700_e117771_d_n6;
        locals.var_t9_dn7 = assign77700_e117771_d_n7;
        locals.var_t9_dn8 = assign77700_e117771_d_n8;
        locals.var_t9_dn9 = assign77700_e117771_d_n9;
        locals.var_t9_dn10 = assign77700_e117771_d_n10;
        locals.var_t9_dn11 = assign77700_e117771_d_n11;
        locals.var_t9_dn14 = assign77700_e117771_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_294(
        locals: &mut StampLocals,
    ) {
        let (assign77710_e117784, assign77710_e117784_d_n0, assign77710_e117784_d_n2, assign77710_e117784_d_n4, assign77710_e117784_d_n5, assign77710_e117784_d_n6, assign77710_e117784_d_n7, assign77710_e117784_d_n8, assign77710_e117784_d_n9, assign77710_e117784_d_n10, assign77710_e117784_d_n11, assign77710_e117784_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77710_e117781: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign77710_e117782: f64 = (0.5 * assign77710_e117781);
        (assign77710_e117782, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77710_e117784;
        locals.var_t2_dn0 = assign77710_e117784_d_n0;
        locals.var_t2_dn2 = assign77710_e117784_d_n2;
        locals.var_t2_dn4 = assign77710_e117784_d_n4;
        locals.var_t2_dn5 = assign77710_e117784_d_n5;
        locals.var_t2_dn6 = assign77710_e117784_d_n6;
        locals.var_t2_dn7 = assign77710_e117784_d_n7;
        locals.var_t2_dn8 = assign77710_e117784_d_n8;
        locals.var_t2_dn9 = assign77710_e117784_d_n9;
        locals.var_t2_dn10 = assign77710_e117784_d_n10;
        locals.var_t2_dn11 = assign77710_e117784_d_n11;
        locals.var_t2_dn14 = assign77710_e117784_d_n14;
        locals.var_t2_rv = 0.0;

        let assign77720_e117787: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1806 = assign77720_e117787;
        locals.var_guard1806_rv = 0.0;

        let (assign77730_e117798, assign77730_e117798_d_n0, assign77730_e117798_d_n2, assign77730_e117798_d_n4, assign77730_e117798_d_n5, assign77730_e117798_d_n6, assign77730_e117798_d_n7, assign77730_e117798_d_n8, assign77730_e117798_d_n9, assign77730_e117798_d_n10, assign77730_e117798_d_n11, assign77730_e117798_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77730_e117798;
        locals.var_t2_dn0 = assign77730_e117798_d_n0;
        locals.var_t2_dn2 = assign77730_e117798_d_n2;
        locals.var_t2_dn4 = assign77730_e117798_d_n4;
        locals.var_t2_dn5 = assign77730_e117798_d_n5;
        locals.var_t2_dn6 = assign77730_e117798_d_n6;
        locals.var_t2_dn7 = assign77730_e117798_d_n7;
        locals.var_t2_dn8 = assign77730_e117798_d_n8;
        locals.var_t2_dn9 = assign77730_e117798_d_n9;
        locals.var_t2_dn10 = assign77730_e117798_d_n10;
        locals.var_t2_dn11 = assign77730_e117798_d_n11;
        locals.var_t2_dn14 = assign77730_e117798_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77740_e117809, assign77740_e117809_d_n0, assign77740_e117809_d_n2, assign77740_e117809_d_n4, assign77740_e117809_d_n5, assign77740_e117809_d_n6, assign77740_e117809_d_n7, assign77740_e117809_d_n8, assign77740_e117809_d_n9, assign77740_e117809_d_n10, assign77740_e117809_d_n11, assign77740_e117809_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77740_e117809;
        locals.var_t9_dn0 = assign77740_e117809_d_n0;
        locals.var_t9_dn2 = assign77740_e117809_d_n2;
        locals.var_t9_dn4 = assign77740_e117809_d_n4;
        locals.var_t9_dn5 = assign77740_e117809_d_n5;
        locals.var_t9_dn6 = assign77740_e117809_d_n6;
        locals.var_t9_dn7 = assign77740_e117809_d_n7;
        locals.var_t9_dn8 = assign77740_e117809_d_n8;
        locals.var_t9_dn9 = assign77740_e117809_d_n9;
        locals.var_t9_dn10 = assign77740_e117809_d_n10;
        locals.var_t9_dn11 = assign77740_e117809_d_n11;
        locals.var_t9_dn14 = assign77740_e117809_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77750_e117818, assign77750_e117818_d_n0, assign77750_e117818_d_n2, assign77750_e117818_d_n4, assign77750_e117818_d_n5, assign77750_e117818_d_n6, assign77750_e117818_d_n7, assign77750_e117818_d_n8, assign77750_e117818_d_n9, assign77750_e117818_d_n10, assign77750_e117818_d_n11, assign77750_e117818_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign77750_e117818;
        locals.var_ddriftldc_dn0 = assign77750_e117818_d_n0;
        locals.var_ddriftldc_dn2 = assign77750_e117818_d_n2;
        locals.var_ddriftldc_dn4 = assign77750_e117818_d_n4;
        locals.var_ddriftldc_dn5 = assign77750_e117818_d_n5;
        locals.var_ddriftldc_dn6 = assign77750_e117818_d_n6;
        locals.var_ddriftldc_dn7 = assign77750_e117818_d_n7;
        locals.var_ddriftldc_dn8 = assign77750_e117818_d_n8;
        locals.var_ddriftldc_dn9 = assign77750_e117818_d_n9;
        locals.var_ddriftldc_dn10 = assign77750_e117818_d_n10;
        locals.var_ddriftldc_dn11 = assign77750_e117818_d_n11;
        locals.var_ddriftldc_dn14 = assign77750_e117818_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign77760_e117835, assign77760_e117835_d_n0, assign77760_e117835_d_n2, assign77760_e117835_d_n4, assign77760_e117835_d_n5, assign77760_e117835_d_n6, assign77760_e117835_d_n7, assign77760_e117835_d_n8, assign77760_e117835_d_n9, assign77760_e117835_d_n10, assign77760_e117835_d_n11, assign77760_e117835_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77760_e117827: f64 = (locals.var_q_nsubld__blk1766 * locals.var_ddriftldc);
        let assign77760_e117829: f64 = (assign77760_e117827 * locals.var_ddriftldc);
        let assign77760_e117831: f64 = (assign77760_e117829 / 2.0);
        let assign77760_e117833: f64 = (assign77760_e117831 / 1.034943e-10);
        (assign77760_e117833, (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign77760_e117835;
        locals.var_dphi_sb_dn0 = assign77760_e117835_d_n0;
        locals.var_dphi_sb_dn2 = assign77760_e117835_d_n2;
        locals.var_dphi_sb_dn4 = assign77760_e117835_d_n4;
        locals.var_dphi_sb_dn5 = assign77760_e117835_d_n5;
        locals.var_dphi_sb_dn6 = assign77760_e117835_d_n6;
        locals.var_dphi_sb_dn7 = assign77760_e117835_d_n7;
        locals.var_dphi_sb_dn8 = assign77760_e117835_d_n8;
        locals.var_dphi_sb_dn9 = assign77760_e117835_d_n9;
        locals.var_dphi_sb_dn10 = assign77760_e117835_d_n10;
        locals.var_dphi_sb_dn11 = assign77760_e117835_d_n11;
        locals.var_dphi_sb_dn14 = assign77760_e117835_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign77770_e117849, assign77770_e117849_d_n0, assign77770_e117849_d_n2, assign77770_e117849_d_n4, assign77770_e117849_d_n5, assign77770_e117849_d_n6, assign77770_e117849_d_n7, assign77770_e117849_d_n8, assign77770_e117849_d_n9, assign77770_e117849_d_n10, assign77770_e117849_d_n11, assign77770_e117849_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77770_e117844: f64 = (2.0 * locals.var_beta);
        let assign77770_e117846: f64 = (assign77770_e117844 * locals.var_dphi_sb);
        let assign77770_e117847: f64 = (assign77770_e117846).sqrt();
        (assign77770_e117847, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn0)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn2)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn4)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn5)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn6)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn7)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn8)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn9)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn10)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn11)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn14)) / (2.0 * assign77770_e117847)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77770_e117849;
        locals.var_t0_dn0 = assign77770_e117849_d_n0;
        locals.var_t0_dn2 = assign77770_e117849_d_n2;
        locals.var_t0_dn4 = assign77770_e117849_d_n4;
        locals.var_t0_dn5 = assign77770_e117849_d_n5;
        locals.var_t0_dn6 = assign77770_e117849_d_n6;
        locals.var_t0_dn7 = assign77770_e117849_d_n7;
        locals.var_t0_dn8 = assign77770_e117849_d_n8;
        locals.var_t0_dn9 = assign77770_e117849_d_n9;
        locals.var_t0_dn10 = assign77770_e117849_d_n10;
        locals.var_t0_dn11 = assign77770_e117849_d_n11;
        locals.var_t0_dn14 = assign77770_e117849_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77780_e117865, assign77780_e117865_d_n0, assign77780_e117865_d_n2, assign77780_e117865_d_n4, assign77780_e117865_d_n5, assign77780_e117865_d_n6, assign77780_e117865_d_n7, assign77780_e117865_d_n8, assign77780_e117865_d_n9, assign77780_e117865_d_n10, assign77780_e117865_d_n11, assign77780_e117865_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77780_e117857: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77780_e117859: f64 = (-locals.var_t0);
        let assign77780_e117860: f64 = { let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77780_e117861: f64 = (assign77780_e117857 + assign77780_e117860);
        let assign77780_e117863: f64 = (assign77780_e117861 / 2.0);
        (assign77780_e117863, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77780_e117865;
        locals.var_t1_dn0 = assign77780_e117865_d_n0;
        locals.var_t1_dn2 = assign77780_e117865_d_n2;
        locals.var_t1_dn4 = assign77780_e117865_d_n4;
        locals.var_t1_dn5 = assign77780_e117865_d_n5;
        locals.var_t1_dn6 = assign77780_e117865_d_n6;
        locals.var_t1_dn7 = assign77780_e117865_d_n7;
        locals.var_t1_dn8 = assign77780_e117865_d_n8;
        locals.var_t1_dn9 = assign77780_e117865_d_n9;
        locals.var_t1_dn10 = assign77780_e117865_d_n10;
        locals.var_t1_dn11 = assign77780_e117865_d_n11;
        locals.var_t1_dn14 = assign77780_e117865_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77790_e117877, assign77790_e117877_d_n0, assign77790_e117877_d_n2, assign77790_e117877_d_n4, assign77790_e117877_d_n5, assign77790_e117877_d_n6, assign77790_e117877_d_n7, assign77790_e117877_d_n8, assign77790_e117877_d_n9, assign77790_e117877_d_n10, assign77790_e117877_d_n11, assign77790_e117877_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77790_e117873: f64 = (locals.var_t1).ln();
        let assign77790_e117875: f64 = (assign77790_e117873 / locals.var_dphi_sb);
        (assign77790_e117875, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign77790_e117877;
        locals.var_c_sb_dn0 = assign77790_e117877_d_n0;
        locals.var_c_sb_dn2 = assign77790_e117877_d_n2;
        locals.var_c_sb_dn4 = assign77790_e117877_d_n4;
        locals.var_c_sb_dn5 = assign77790_e117877_d_n5;
        locals.var_c_sb_dn6 = assign77790_e117877_d_n6;
        locals.var_c_sb_dn7 = assign77790_e117877_d_n7;
        locals.var_c_sb_dn8 = assign77790_e117877_d_n8;
        locals.var_c_sb_dn9 = assign77790_e117877_d_n9;
        locals.var_c_sb_dn10 = assign77790_e117877_d_n10;
        locals.var_c_sb_dn11 = assign77790_e117877_d_n11;
        locals.var_c_sb_dn14 = assign77790_e117877_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign77800_e117888, assign77800_e117888_d_n0, assign77800_e117888_d_n2, assign77800_e117888_d_n4, assign77800_e117888_d_n5, assign77800_e117888_d_n6, assign77800_e117888_d_n7, assign77800_e117888_d_n8, assign77800_e117888_d_n9, assign77800_e117888_d_n10, assign77800_e117888_d_n11, assign77800_e117888_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77800_e117886: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign77800_e117886, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign77800_e117888;
        locals.var_ps0ld_vxb_dn0 = assign77800_e117888_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign77800_e117888_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign77800_e117888_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign77800_e117888_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign77800_e117888_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign77800_e117888_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign77800_e117888_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign77800_e117888_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign77800_e117888_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign77800_e117888_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign77800_e117888_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign77810_e117901, assign77810_e117901_d_n0, assign77810_e117901_d_n2, assign77810_e117901_d_n4, assign77810_e117901_d_n5, assign77810_e117901_d_n6, assign77810_e117901_d_n7, assign77810_e117901_d_n8, assign77810_e117901_d_n9, assign77810_e117901_d_n10, assign77810_e117901_d_n11, assign77810_e117901_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77810_e117898: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign77810_e117899: f64 = (locals.var_c_sb * assign77810_e117898);
        (assign77810_e117899, ((locals.var_c_sb_dn0 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign77810_e117901;
        locals.var_ty_dn0 = assign77810_e117901_d_n0;
        locals.var_ty_dn2 = assign77810_e117901_d_n2;
        locals.var_ty_dn4 = assign77810_e117901_d_n4;
        locals.var_ty_dn5 = assign77810_e117901_d_n5;
        locals.var_ty_dn6 = assign77810_e117901_d_n6;
        locals.var_ty_dn7 = assign77810_e117901_d_n7;
        locals.var_ty_dn8 = assign77810_e117901_d_n8;
        locals.var_ty_dn9 = assign77810_e117901_d_n9;
        locals.var_ty_dn10 = assign77810_e117901_d_n10;
        locals.var_ty_dn11 = assign77810_e117901_d_n11;
        locals.var_ty_dn14 = assign77810_e117901_d_n14;
        locals.var_ty_rv = 0.0;

        let assign77820_e117904: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1807 = assign77820_e117904;
        locals.var_guard1807_rv = 0.0;

        let (assign77830_e117916, assign77830_e117916_d_n0, assign77830_e117916_d_n2, assign77830_e117916_d_n4, assign77830_e117916_d_n5, assign77830_e117916_d_n6, assign77830_e117916_d_n7, assign77830_e117916_d_n8, assign77830_e117916_d_n9, assign77830_e117916_d_n10, assign77830_e117916_d_n11, assign77830_e117916_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77830_e117914: f64 = (locals.var_ty).exp();
        (assign77830_e117914, (assign77830_e117914 * locals.var_ty_dn0), (assign77830_e117914 * locals.var_ty_dn2), (assign77830_e117914 * locals.var_ty_dn4), (assign77830_e117914 * locals.var_ty_dn5), (assign77830_e117914 * locals.var_ty_dn6), (assign77830_e117914 * locals.var_ty_dn7), (assign77830_e117914 * locals.var_ty_dn8), (assign77830_e117914 * locals.var_ty_dn9), (assign77830_e117914 * locals.var_ty_dn10), (assign77830_e117914 * locals.var_ty_dn11), (assign77830_e117914 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77830_e117916;
        locals.var_t1_dn0 = assign77830_e117916_d_n0;
        locals.var_t1_dn2 = assign77830_e117916_d_n2;
        locals.var_t1_dn4 = assign77830_e117916_d_n4;
        locals.var_t1_dn5 = assign77830_e117916_d_n5;
        locals.var_t1_dn6 = assign77830_e117916_d_n6;
        locals.var_t1_dn7 = assign77830_e117916_d_n7;
        locals.var_t1_dn8 = assign77830_e117916_d_n8;
        locals.var_t1_dn9 = assign77830_e117916_d_n9;
        locals.var_t1_dn10 = assign77830_e117916_d_n10;
        locals.var_t1_dn11 = assign77830_e117916_d_n11;
        locals.var_t1_dn14 = assign77830_e117916_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77840_e117931, assign77840_e117931_d_n0, assign77840_e117931_d_n2, assign77840_e117931_d_n4, assign77840_e117931_d_n5, assign77840_e117931_d_n6, assign77840_e117931_d_n7, assign77840_e117931_d_n8, assign77840_e117931_d_n9, assign77840_e117931_d_n10, assign77840_e117931_d_n11, assign77840_e117931_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77840_e117926: f64 = (-locals.var_c_sb);
        let assign77840_e117928: f64 = (assign77840_e117926 * locals.var_dphi_sb);
        let assign77840_e117929: f64 = (assign77840_e117928).exp();
        (assign77840_e117929, (assign77840_e117929 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn0))), (assign77840_e117929 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn2))), (assign77840_e117929 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn4))), (assign77840_e117929 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn5))), (assign77840_e117929 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn6))), (assign77840_e117929 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn7))), (assign77840_e117929 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn8))), (assign77840_e117929 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn9))), (assign77840_e117929 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn10))), (assign77840_e117929 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn11))), (assign77840_e117929 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77840_e117931;
        locals.var_t0_dn0 = assign77840_e117931_d_n0;
        locals.var_t0_dn2 = assign77840_e117931_d_n2;
        locals.var_t0_dn4 = assign77840_e117931_d_n4;
        locals.var_t0_dn5 = assign77840_e117931_d_n5;
        locals.var_t0_dn6 = assign77840_e117931_d_n6;
        locals.var_t0_dn7 = assign77840_e117931_d_n7;
        locals.var_t0_dn8 = assign77840_e117931_d_n8;
        locals.var_t0_dn9 = assign77840_e117931_d_n9;
        locals.var_t0_dn10 = assign77840_e117931_d_n10;
        locals.var_t0_dn11 = assign77840_e117931_d_n11;
        locals.var_t0_dn14 = assign77840_e117931_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77850_e117944, assign77850_e117944_d_n0, assign77850_e117944_d_n2, assign77850_e117944_d_n4, assign77850_e117944_d_n5, assign77850_e117944_d_n6, assign77850_e117944_d_n7, assign77850_e117944_d_n8, assign77850_e117944_d_n9, assign77850_e117944_d_n10, assign77850_e117944_d_n11, assign77850_e117944_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77850_e117942: f64 = (locals.var_t1 - locals.var_t0);
        (assign77850_e117942, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77850_e117944;
        locals.var_t2_dn0 = assign77850_e117944_d_n0;
        locals.var_t2_dn2 = assign77850_e117944_d_n2;
        locals.var_t2_dn4 = assign77850_e117944_d_n4;
        locals.var_t2_dn5 = assign77850_e117944_d_n5;
        locals.var_t2_dn6 = assign77850_e117944_d_n6;
        locals.var_t2_dn7 = assign77850_e117944_d_n7;
        locals.var_t2_dn8 = assign77850_e117944_d_n8;
        locals.var_t2_dn9 = assign77850_e117944_d_n9;
        locals.var_t2_dn10 = assign77850_e117944_d_n10;
        locals.var_t2_dn11 = assign77850_e117944_d_n11;
        locals.var_t2_dn14 = assign77850_e117944_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77860_e117960, assign77860_e117960_d_n0, assign77860_e117960_d_n2, assign77860_e117960_d_n4, assign77860_e117960_d_n5, assign77860_e117960_d_n6, assign77860_e117960_d_n7, assign77860_e117960_d_n8, assign77860_e117960_d_n9, assign77860_e117960_d_n10, assign77860_e117960_d_n11, assign77860_e117960_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77860_e117955: f64 = (1.0 + locals.var_t2);
        let assign77860_e117956: f64 = (assign77860_e117955).ln();
        let assign77860_e117958: f64 = (assign77860_e117956 / locals.var_c_sb);
        (assign77860_e117958, ((((locals.var_t2_dn0 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77860_e117960;
        locals.var_phi_b_dn0 = assign77860_e117960_d_n0;
        locals.var_phi_b_dn2 = assign77860_e117960_d_n2;
        locals.var_phi_b_dn4 = assign77860_e117960_d_n4;
        locals.var_phi_b_dn5 = assign77860_e117960_d_n5;
        locals.var_phi_b_dn6 = assign77860_e117960_d_n6;
        locals.var_phi_b_dn7 = assign77860_e117960_d_n7;
        locals.var_phi_b_dn8 = assign77860_e117960_d_n8;
        locals.var_phi_b_dn9 = assign77860_e117960_d_n9;
        locals.var_phi_b_dn10 = assign77860_e117960_d_n10;
        locals.var_phi_b_dn11 = assign77860_e117960_d_n11;
        locals.var_phi_b_dn14 = assign77860_e117960_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign77870_e117974, assign77870_e117974_d_n0, assign77870_e117974_d_n2, assign77870_e117974_d_n4, assign77870_e117974_d_n5, assign77870_e117974_d_n6, assign77870_e117974_d_n7, assign77870_e117974_d_n8, assign77870_e117974_d_n9, assign77870_e117974_d_n10, assign77870_e117974_d_n11, assign77870_e117974_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 == 0.0)) {
        let assign77870_e117972: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign77870_e117972, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77870_e117974;
        locals.var_phi_b_dn0 = assign77870_e117974_d_n0;
        locals.var_phi_b_dn2 = assign77870_e117974_d_n2;
        locals.var_phi_b_dn4 = assign77870_e117974_d_n4;
        locals.var_phi_b_dn5 = assign77870_e117974_d_n5;
        locals.var_phi_b_dn6 = assign77870_e117974_d_n6;
        locals.var_phi_b_dn7 = assign77870_e117974_d_n7;
        locals.var_phi_b_dn8 = assign77870_e117974_d_n8;
        locals.var_phi_b_dn9 = assign77870_e117974_d_n9;
        locals.var_phi_b_dn10 = assign77870_e117974_d_n10;
        locals.var_phi_b_dn11 = assign77870_e117974_d_n11;
        locals.var_phi_b_dn14 = assign77870_e117974_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign77880_e117985, assign77880_e117985_d_n0, assign77880_e117985_d_n2, assign77880_e117985_d_n4, assign77880_e117985_d_n5, assign77880_e117985_d_n6, assign77880_e117985_d_n7, assign77880_e117985_d_n8, assign77880_e117985_d_n9, assign77880_e117985_d_n10, assign77880_e117985_d_n11, assign77880_e117985_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77880_e117983: f64 = (locals.var_beta * locals.var_phi_b);
        (assign77880_e117983, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign77880_e117985;
        locals.var_chib_dn0 = assign77880_e117985_d_n0;
        locals.var_chib_dn2 = assign77880_e117985_d_n2;
        locals.var_chib_dn4 = assign77880_e117985_d_n4;
        locals.var_chib_dn5 = assign77880_e117985_d_n5;
        locals.var_chib_dn6 = assign77880_e117985_d_n6;
        locals.var_chib_dn7 = assign77880_e117985_d_n7;
        locals.var_chib_dn8 = assign77880_e117985_d_n8;
        locals.var_chib_dn9 = assign77880_e117985_d_n9;
        locals.var_chib_dn10 = assign77880_e117985_d_n10;
        locals.var_chib_dn11 = assign77880_e117985_d_n11;
        locals.var_chib_dn14 = assign77880_e117985_d_n14;
        locals.var_chib_rv = 0.0;

        let assign77890_e117989: f64 = (locals.var_chi / 100.0);
        let assign77890_e117994: f64 = if ((locals.var_chib > assign77890_e117989) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1808 = assign77890_e117994;
        locals.var_guard1808_rv = 0.0;

        let (assign77900_e118007,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1808 != 0.0)) {
        let assign77900_e118005: f64 = (locals.var_flg_fd_mode__blk1772 + 1.0);
        (assign77900_e118005,)
    } else {
        (locals.var_flg_fd_mode__blk1772,)
    }
};
        locals.var_flg_fd_mode__blk1772 = assign77900_e118007;
        locals.var_flg_fd_mode__blk1772_rv = 0.0;

        let (assign77910_e118018, assign77910_e118018_d_n0, assign77910_e118018_d_n2, assign77910_e118018_d_n4, assign77910_e118018_d_n5, assign77910_e118018_d_n6, assign77910_e118018_d_n7, assign77910_e118018_d_n8, assign77910_e118018_d_n9, assign77910_e118018_d_n10, assign77910_e118018_d_n11, assign77910_e118018_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1808 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77910_e118018;
        locals.var_chi_dn0 = assign77910_e118018_d_n0;
        locals.var_chi_dn2 = assign77910_e118018_d_n2;
        locals.var_chi_dn4 = assign77910_e118018_d_n4;
        locals.var_chi_dn5 = assign77910_e118018_d_n5;
        locals.var_chi_dn6 = assign77910_e118018_d_n6;
        locals.var_chi_dn7 = assign77910_e118018_d_n7;
        locals.var_chi_dn8 = assign77910_e118018_d_n8;
        locals.var_chi_dn9 = assign77910_e118018_d_n9;
        locals.var_chi_dn10 = assign77910_e118018_d_n10;
        locals.var_chi_dn11 = assign77910_e118018_d_n11;
        locals.var_chi_dn14 = assign77910_e118018_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77920_e118029, assign77920_e118029_d_n0, assign77920_e118029_d_n2, assign77920_e118029_d_n4, assign77920_e118029_d_n5, assign77920_e118029_d_n6, assign77920_e118029_d_n7, assign77920_e118029_d_n8, assign77920_e118029_d_n9, assign77920_e118029_d_n10, assign77920_e118029_d_n11, assign77920_e118029_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77920_e118025: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77920_e118027: f64 = (assign77920_e118025 - locals.var_vxbgmtcl);
        (assign77920_e118027, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77920_e118029;
        locals.var_ps0ld_dn0 = assign77920_e118029_d_n0;
        locals.var_ps0ld_dn2 = assign77920_e118029_d_n2;
        locals.var_ps0ld_dn4 = assign77920_e118029_d_n4;
        locals.var_ps0ld_dn5 = assign77920_e118029_d_n5;
        locals.var_ps0ld_dn6 = assign77920_e118029_d_n6;
        locals.var_ps0ld_dn7 = assign77920_e118029_d_n7;
        locals.var_ps0ld_dn8 = assign77920_e118029_d_n8;
        locals.var_ps0ld_dn9 = assign77920_e118029_d_n9;
        locals.var_ps0ld_dn10 = assign77920_e118029_d_n10;
        locals.var_ps0ld_dn11 = assign77920_e118029_d_n11;
        locals.var_ps0ld_dn14 = assign77920_e118029_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign77930_e118031: f64 = (locals.var_chi).abs();
        let assign77930_e118033: f64 = if assign77930_e118031 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1809 = assign77930_e118033;
        locals.var_guard1809_rv = 0.0;

        let (assign77940_e118048, assign77940_e118048_d_n0, assign77940_e118048_d_n2, assign77940_e118048_d_n4, assign77940_e118048_d_n5, assign77940_e118048_d_n6, assign77940_e118048_d_n7, assign77940_e118048_d_n8, assign77940_e118048_d_n9, assign77940_e118048_d_n10, assign77940_e118048_d_n11, assign77940_e118048_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign77940_e118042: f64 = (locals.var_chi - 1.0);
        let assign77940_e118044: f64 = (-locals.var_chi);
        let assign77940_e118045: f64 = (assign77940_e118044).exp();
        let assign77940_e118046: f64 = (assign77940_e118042 + assign77940_e118045);
        (assign77940_e118046, (locals.var_chi_dn0 + (assign77940_e118045 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign77940_e118045 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign77940_e118045 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign77940_e118045 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign77940_e118045 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign77940_e118045 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign77940_e118045 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign77940_e118045 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign77940_e118045 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign77940_e118045 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign77940_e118045 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77940_e118048;
        locals.var_t1_dn0 = assign77940_e118048_d_n0;
        locals.var_t1_dn2 = assign77940_e118048_d_n2;
        locals.var_t1_dn4 = assign77940_e118048_d_n4;
        locals.var_t1_dn5 = assign77940_e118048_d_n5;
        locals.var_t1_dn6 = assign77940_e118048_d_n6;
        locals.var_t1_dn7 = assign77940_e118048_d_n7;
        locals.var_t1_dn8 = assign77940_e118048_d_n8;
        locals.var_t1_dn9 = assign77940_e118048_d_n9;
        locals.var_t1_dn10 = assign77940_e118048_d_n10;
        locals.var_t1_dn11 = assign77940_e118048_d_n11;
        locals.var_t1_dn14 = assign77940_e118048_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77950_e118058, assign77950_e118058_d_n0, assign77950_e118058_d_n2, assign77950_e118058_d_n4, assign77950_e118058_d_n5, assign77950_e118058_d_n6, assign77950_e118058_d_n7, assign77950_e118058_d_n8, assign77950_e118058_d_n9, assign77950_e118058_d_n10, assign77950_e118058_d_n11, assign77950_e118058_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign77950_e118056: f64 = (locals.var_t1).sqrt();
        (assign77950_e118056, (locals.var_t1_dn0 / (2.0 * assign77950_e118056)), (locals.var_t1_dn2 / (2.0 * assign77950_e118056)), (locals.var_t1_dn4 / (2.0 * assign77950_e118056)), (locals.var_t1_dn5 / (2.0 * assign77950_e118056)), (locals.var_t1_dn6 / (2.0 * assign77950_e118056)), (locals.var_t1_dn7 / (2.0 * assign77950_e118056)), (locals.var_t1_dn8 / (2.0 * assign77950_e118056)), (locals.var_t1_dn9 / (2.0 * assign77950_e118056)), (locals.var_t1_dn10 / (2.0 * assign77950_e118056)), (locals.var_t1_dn11 / (2.0 * assign77950_e118056)), (locals.var_t1_dn14 / (2.0 * assign77950_e118056)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77950_e118058;
        locals.var_t2_dn0 = assign77950_e118058_d_n0;
        locals.var_t2_dn2 = assign77950_e118058_d_n2;
        locals.var_t2_dn4 = assign77950_e118058_d_n4;
        locals.var_t2_dn5 = assign77950_e118058_d_n5;
        locals.var_t2_dn6 = assign77950_e118058_d_n6;
        locals.var_t2_dn7 = assign77950_e118058_d_n7;
        locals.var_t2_dn8 = assign77950_e118058_d_n8;
        locals.var_t2_dn9 = assign77950_e118058_d_n9;
        locals.var_t2_dn10 = assign77950_e118058_d_n10;
        locals.var_t2_dn11 = assign77950_e118058_d_n11;
        locals.var_t2_dn14 = assign77950_e118058_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77970_e118089, assign77970_e118089_d_n0, assign77970_e118089_d_n2, assign77970_e118089_d_n4, assign77970_e118089_d_n5, assign77970_e118089_d_n6, assign77970_e118089_d_n7, assign77970_e118089_d_n8, assign77970_e118089_d_n9, assign77970_e118089_d_n10, assign77970_e118089_d_n11, assign77970_e118089_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 == 0.0)) {
        let assign77970_e118080: f64 = (0.7071067811865475 * locals.var_chi);
        let assign77970_e118084: f64 = (locals.var_chi * 0.3333333333333333);
        let assign77970_e118085: f64 = (1.0 - assign77970_e118084);
        let assign77970_e118086: f64 = (assign77970_e118085).sqrt();
        let assign77970_e118087: f64 = (assign77970_e118080 * assign77970_e118086);
        (assign77970_e118087, (((0.7071067811865475 * locals.var_chi_dn0) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77970_e118089;
        locals.var_t2_dn0 = assign77970_e118089_d_n0;
        locals.var_t2_dn2 = assign77970_e118089_d_n2;
        locals.var_t2_dn4 = assign77970_e118089_d_n4;
        locals.var_t2_dn5 = assign77970_e118089_d_n5;
        locals.var_t2_dn6 = assign77970_e118089_d_n6;
        locals.var_t2_dn7 = assign77970_e118089_d_n7;
        locals.var_t2_dn8 = assign77970_e118089_d_n8;
        locals.var_t2_dn9 = assign77970_e118089_d_n9;
        locals.var_t2_dn10 = assign77970_e118089_d_n10;
        locals.var_t2_dn11 = assign77970_e118089_d_n11;
        locals.var_t2_dn14 = assign77970_e118089_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_295(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77980_e118098, assign77980_e118098_d_n0, assign77980_e118098_d_n2, assign77980_e118098_d_n4, assign77980_e118098_d_n5, assign77980_e118098_d_n6, assign77980_e118098_d_n7, assign77980_e118098_d_n8, assign77980_e118098_d_n9, assign77980_e118098_d_n10, assign77980_e118098_d_n11, assign77980_e118098_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77980_e118096: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign77980_e118096, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign77980_e118098;
        locals.var_qbuld_dn0 = assign77980_e118098_d_n0;
        locals.var_qbuld_dn2 = assign77980_e118098_d_n2;
        locals.var_qbuld_dn4 = assign77980_e118098_d_n4;
        locals.var_qbuld_dn5 = assign77980_e118098_d_n5;
        locals.var_qbuld_dn6 = assign77980_e118098_d_n6;
        locals.var_qbuld_dn7 = assign77980_e118098_d_n7;
        locals.var_qbuld_dn8 = assign77980_e118098_d_n8;
        locals.var_qbuld_dn9 = assign77980_e118098_d_n9;
        locals.var_qbuld_dn10 = assign77980_e118098_d_n10;
        locals.var_qbuld_dn11 = assign77980_e118098_d_n11;
        locals.var_qbuld_dn14 = assign77980_e118098_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign77990_e118109, assign77990_e118109_d_n0, assign77990_e118109_d_n2, assign77990_e118109_d_n4, assign77990_e118109_d_n5, assign77990_e118109_d_n6, assign77990_e118109_d_n7, assign77990_e118109_d_n8, assign77990_e118109_d_n9, assign77990_e118109_d_n10, assign77990_e118109_d_n11, assign77990_e118109_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77990_e118106: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign77990_e118107: f64 = (locals.var_cox0_func * assign77990_e118106);
        (assign77990_e118107, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign77990_e118109;
        locals.var_qsuld_dn0 = assign77990_e118109_d_n0;
        locals.var_qsuld_dn2 = assign77990_e118109_d_n2;
        locals.var_qsuld_dn4 = assign77990_e118109_d_n4;
        locals.var_qsuld_dn5 = assign77990_e118109_d_n5;
        locals.var_qsuld_dn6 = assign77990_e118109_d_n6;
        locals.var_qsuld_dn7 = assign77990_e118109_d_n7;
        locals.var_qsuld_dn8 = assign77990_e118109_d_n8;
        locals.var_qsuld_dn9 = assign77990_e118109_d_n9;
        locals.var_qsuld_dn10 = assign77990_e118109_d_n10;
        locals.var_qsuld_dn11 = assign77990_e118109_d_n11;
        locals.var_qsuld_dn14 = assign77990_e118109_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign78000_e118118, assign78000_e118118_d_n0, assign78000_e118118_d_n2, assign78000_e118118_d_n4, assign78000_e118118_d_n5, assign78000_e118118_d_n6, assign78000_e118118_d_n7, assign78000_e118118_d_n8, assign78000_e118118_d_n9, assign78000_e118118_d_n10, assign78000_e118118_d_n11, assign78000_e118118_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign78000_e118116: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1766);
        (assign78000_e118116, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk1766),)
    } else {
        (locals.var_wdld0__blk1810, locals.var_wdld0__blk1810_dn0, locals.var_wdld0__blk1810_dn2, locals.var_wdld0__blk1810_dn4, locals.var_wdld0__blk1810_dn5, locals.var_wdld0__blk1810_dn6, locals.var_wdld0__blk1810_dn7, locals.var_wdld0__blk1810_dn8, locals.var_wdld0__blk1810_dn9, locals.var_wdld0__blk1810_dn10, locals.var_wdld0__blk1810_dn11, locals.var_wdld0__blk1810_dn14,)
    }
};
        locals.var_wdld0__blk1810 = assign78000_e118118;
        locals.var_wdld0__blk1810_dn0 = assign78000_e118118_d_n0;
        locals.var_wdld0__blk1810_dn2 = assign78000_e118118_d_n2;
        locals.var_wdld0__blk1810_dn4 = assign78000_e118118_d_n4;
        locals.var_wdld0__blk1810_dn5 = assign78000_e118118_d_n5;
        locals.var_wdld0__blk1810_dn6 = assign78000_e118118_d_n6;
        locals.var_wdld0__blk1810_dn7 = assign78000_e118118_d_n7;
        locals.var_wdld0__blk1810_dn8 = assign78000_e118118_d_n8;
        locals.var_wdld0__blk1810_dn9 = assign78000_e118118_d_n9;
        locals.var_wdld0__blk1810_dn10 = assign78000_e118118_d_n10;
        locals.var_wdld0__blk1810_dn11 = assign78000_e118118_d_n11;
        locals.var_wdld0__blk1810_dn14 = assign78000_e118118_d_n14;
        locals.var_wdld0__blk1810_rv = 0.0;

        let assign78010_e118121: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1812 = assign78010_e118121;
        locals.var_guard1812_rv = 0.0;

        let assign78020_e118126: f64 = (locals.var_ddriftldc * 0.1);
        let assign78020_e118127: f64 = (locals.var_ddriftldc - assign78020_e118126);
        let assign78020_e118131: f64 = (locals.var_ddriftldc * 0.1);
        let assign78020_e118134: f64 = if ((locals.var_wdld0__blk1810 > assign78020_e118127) && (assign78020_e118131 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1813 = assign78020_e118134;
        locals.var_guard1813_rv = 0.0;

        let (assign78030_e118151, assign78030_e118151_d_n0, assign78030_e118151_d_n2, assign78030_e118151_d_n4, assign78030_e118151_d_n5, assign78030_e118151_d_n6, assign78030_e118151_d_n7, assign78030_e118151_d_n8, assign78030_e118151_d_n9, assign78030_e118151_d_n10, assign78030_e118151_d_n11, assign78030_e118151_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78030_e118145: f64 = (locals.var_wdld0__blk1810 - locals.var_ddriftldc);
        let assign78030_e118148: f64 = (locals.var_ddriftldc * 0.1);
        let assign78030_e118149: f64 = (assign78030_e118145 + assign78030_e118148);
        (assign78030_e118149, ((locals.var_wdld0__blk1810_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1810_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1810_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1810_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1810_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1810_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1810_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1810_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1810_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1810_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk1810_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign78030_e118151;
        locals.var_tmf1_dn0 = assign78030_e118151_d_n0;
        locals.var_tmf1_dn2 = assign78030_e118151_d_n2;
        locals.var_tmf1_dn4 = assign78030_e118151_d_n4;
        locals.var_tmf1_dn5 = assign78030_e118151_d_n5;
        locals.var_tmf1_dn6 = assign78030_e118151_d_n6;
        locals.var_tmf1_dn7 = assign78030_e118151_d_n7;
        locals.var_tmf1_dn8 = assign78030_e118151_d_n8;
        locals.var_tmf1_dn9 = assign78030_e118151_d_n9;
        locals.var_tmf1_dn10 = assign78030_e118151_d_n10;
        locals.var_tmf1_dn11 = assign78030_e118151_d_n11;
        locals.var_tmf1_dn14 = assign78030_e118151_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign78040_e118164, assign78040_e118164_d_n0, assign78040_e118164_d_n2, assign78040_e118164_d_n4, assign78040_e118164_d_n5, assign78040_e118164_d_n6, assign78040_e118164_d_n7, assign78040_e118164_d_n8, assign78040_e118164_d_n9, assign78040_e118164_d_n10, assign78040_e118164_d_n11, assign78040_e118164_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78040_e118162: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78040_e118162, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign78040_e118164;
        locals.var_x2_dn0 = assign78040_e118164_d_n0;
        locals.var_x2_dn2 = assign78040_e118164_d_n2;
        locals.var_x2_dn4 = assign78040_e118164_d_n4;
        locals.var_x2_dn5 = assign78040_e118164_d_n5;
        locals.var_x2_dn6 = assign78040_e118164_d_n6;
        locals.var_x2_dn7 = assign78040_e118164_d_n7;
        locals.var_x2_dn8 = assign78040_e118164_d_n8;
        locals.var_x2_dn9 = assign78040_e118164_d_n9;
        locals.var_x2_dn10 = assign78040_e118164_d_n10;
        locals.var_x2_dn11 = assign78040_e118164_d_n11;
        locals.var_x2_dn14 = assign78040_e118164_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign78050_e118181, assign78050_e118181_d_n0, assign78050_e118181_d_n2, assign78050_e118181_d_n4, assign78050_e118181_d_n5, assign78050_e118181_d_n6, assign78050_e118181_d_n7, assign78050_e118181_d_n8, assign78050_e118181_d_n9, assign78050_e118181_d_n10, assign78050_e118181_d_n11, assign78050_e118181_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78050_e118175: f64 = (locals.var_ddriftldc * 0.1);
        let assign78050_e118178: f64 = (locals.var_ddriftldc * 0.1);
        let assign78050_e118179: f64 = (assign78050_e118175 * assign78050_e118178);
        (assign78050_e118179, (((locals.var_ddriftldc_dn0 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign78050_e118181;
        locals.var_xmax2_dn0 = assign78050_e118181_d_n0;
        locals.var_xmax2_dn2 = assign78050_e118181_d_n2;
        locals.var_xmax2_dn4 = assign78050_e118181_d_n4;
        locals.var_xmax2_dn5 = assign78050_e118181_d_n5;
        locals.var_xmax2_dn6 = assign78050_e118181_d_n6;
        locals.var_xmax2_dn7 = assign78050_e118181_d_n7;
        locals.var_xmax2_dn8 = assign78050_e118181_d_n8;
        locals.var_xmax2_dn9 = assign78050_e118181_d_n9;
        locals.var_xmax2_dn10 = assign78050_e118181_d_n10;
        locals.var_xmax2_dn11 = assign78050_e118181_d_n11;
        locals.var_xmax2_dn14 = assign78050_e118181_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign78060_e118192, assign78060_e118192_d_n0, assign78060_e118192_d_n2, assign78060_e118192_d_n4, assign78060_e118192_d_n5, assign78060_e118192_d_n6, assign78060_e118192_d_n7, assign78060_e118192_d_n8, assign78060_e118192_d_n9, assign78060_e118192_d_n10, assign78060_e118192_d_n11, assign78060_e118192_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78060_e118192;
        locals.var_xp_dn0 = assign78060_e118192_d_n0;
        locals.var_xp_dn2 = assign78060_e118192_d_n2;
        locals.var_xp_dn4 = assign78060_e118192_d_n4;
        locals.var_xp_dn5 = assign78060_e118192_d_n5;
        locals.var_xp_dn6 = assign78060_e118192_d_n6;
        locals.var_xp_dn7 = assign78060_e118192_d_n7;
        locals.var_xp_dn8 = assign78060_e118192_d_n8;
        locals.var_xp_dn9 = assign78060_e118192_d_n9;
        locals.var_xp_dn10 = assign78060_e118192_d_n10;
        locals.var_xp_dn11 = assign78060_e118192_d_n11;
        locals.var_xp_dn14 = assign78060_e118192_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78070_e118203, assign78070_e118203_d_n0, assign78070_e118203_d_n2, assign78070_e118203_d_n4, assign78070_e118203_d_n5, assign78070_e118203_d_n6, assign78070_e118203_d_n7, assign78070_e118203_d_n8, assign78070_e118203_d_n9, assign78070_e118203_d_n10, assign78070_e118203_d_n11, assign78070_e118203_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78070_e118203;
        locals.var_xmp_dn0 = assign78070_e118203_d_n0;
        locals.var_xmp_dn2 = assign78070_e118203_d_n2;
        locals.var_xmp_dn4 = assign78070_e118203_d_n4;
        locals.var_xmp_dn5 = assign78070_e118203_d_n5;
        locals.var_xmp_dn6 = assign78070_e118203_d_n6;
        locals.var_xmp_dn7 = assign78070_e118203_d_n7;
        locals.var_xmp_dn8 = assign78070_e118203_d_n8;
        locals.var_xmp_dn9 = assign78070_e118203_d_n9;
        locals.var_xmp_dn10 = assign78070_e118203_d_n10;
        locals.var_xmp_dn11 = assign78070_e118203_d_n11;
        locals.var_xmp_dn14 = assign78070_e118203_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78080_e118214,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78080_e118214;
        locals.var_m0_rv = 0.0;

        let (assign78090_e118225,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78090_e118225;
        locals.var_mm_rv = 0.0;

        let (assign78100_e118236, assign78100_e118236_d_n0, assign78100_e118236_d_n2, assign78100_e118236_d_n4, assign78100_e118236_d_n5, assign78100_e118236_d_n6, assign78100_e118236_d_n7, assign78100_e118236_d_n8, assign78100_e118236_d_n9, assign78100_e118236_d_n10, assign78100_e118236_d_n11, assign78100_e118236_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78100_e118236;
        locals.var_arg_dn0 = assign78100_e118236_d_n0;
        locals.var_arg_dn2 = assign78100_e118236_d_n2;
        locals.var_arg_dn4 = assign78100_e118236_d_n4;
        locals.var_arg_dn5 = assign78100_e118236_d_n5;
        locals.var_arg_dn6 = assign78100_e118236_d_n6;
        locals.var_arg_dn7 = assign78100_e118236_d_n7;
        locals.var_arg_dn8 = assign78100_e118236_d_n8;
        locals.var_arg_dn9 = assign78100_e118236_d_n9;
        locals.var_arg_dn10 = assign78100_e118236_d_n10;
        locals.var_arg_dn11 = assign78100_e118236_d_n11;
        locals.var_arg_dn14 = assign78100_e118236_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78110_e118247, assign78110_e118247_d_n0, assign78110_e118247_d_n2, assign78110_e118247_d_n4, assign78110_e118247_d_n5, assign78110_e118247_d_n6, assign78110_e118247_d_n7, assign78110_e118247_d_n8, assign78110_e118247_d_n9, assign78110_e118247_d_n10, assign78110_e118247_d_n11, assign78110_e118247_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78110_e118247;
        locals.var_dnm_dn0 = assign78110_e118247_d_n0;
        locals.var_dnm_dn2 = assign78110_e118247_d_n2;
        locals.var_dnm_dn4 = assign78110_e118247_d_n4;
        locals.var_dnm_dn5 = assign78110_e118247_d_n5;
        locals.var_dnm_dn6 = assign78110_e118247_d_n6;
        locals.var_dnm_dn7 = assign78110_e118247_d_n7;
        locals.var_dnm_dn8 = assign78110_e118247_d_n8;
        locals.var_dnm_dn9 = assign78110_e118247_d_n9;
        locals.var_dnm_dn10 = assign78110_e118247_d_n10;
        locals.var_dnm_dn11 = assign78110_e118247_d_n11;
        locals.var_dnm_dn14 = assign78110_e118247_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78120_e118260, assign78120_e118260_d_n0, assign78120_e118260_d_n2, assign78120_e118260_d_n4, assign78120_e118260_d_n5, assign78120_e118260_d_n6, assign78120_e118260_d_n7, assign78120_e118260_d_n8, assign78120_e118260_d_n9, assign78120_e118260_d_n10, assign78120_e118260_d_n11, assign78120_e118260_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78120_e118258: f64 = (locals.var_xp * locals.var_x2);
        (assign78120_e118258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78120_e118260;
        locals.var_xp_dn0 = assign78120_e118260_d_n0;
        locals.var_xp_dn2 = assign78120_e118260_d_n2;
        locals.var_xp_dn4 = assign78120_e118260_d_n4;
        locals.var_xp_dn5 = assign78120_e118260_d_n5;
        locals.var_xp_dn6 = assign78120_e118260_d_n6;
        locals.var_xp_dn7 = assign78120_e118260_d_n7;
        locals.var_xp_dn8 = assign78120_e118260_d_n8;
        locals.var_xp_dn9 = assign78120_e118260_d_n9;
        locals.var_xp_dn10 = assign78120_e118260_d_n10;
        locals.var_xp_dn11 = assign78120_e118260_d_n11;
        locals.var_xp_dn14 = assign78120_e118260_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78130_e118273, assign78130_e118273_d_n0, assign78130_e118273_d_n2, assign78130_e118273_d_n4, assign78130_e118273_d_n5, assign78130_e118273_d_n6, assign78130_e118273_d_n7, assign78130_e118273_d_n8, assign78130_e118273_d_n9, assign78130_e118273_d_n10, assign78130_e118273_d_n11, assign78130_e118273_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78130_e118271: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78130_e118271, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78130_e118273;
        locals.var_xmp_dn0 = assign78130_e118273_d_n0;
        locals.var_xmp_dn2 = assign78130_e118273_d_n2;
        locals.var_xmp_dn4 = assign78130_e118273_d_n4;
        locals.var_xmp_dn5 = assign78130_e118273_d_n5;
        locals.var_xmp_dn6 = assign78130_e118273_d_n6;
        locals.var_xmp_dn7 = assign78130_e118273_d_n7;
        locals.var_xmp_dn8 = assign78130_e118273_d_n8;
        locals.var_xmp_dn9 = assign78130_e118273_d_n9;
        locals.var_xmp_dn10 = assign78130_e118273_d_n10;
        locals.var_xmp_dn11 = assign78130_e118273_d_n11;
        locals.var_xmp_dn14 = assign78130_e118273_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78140_e118286, assign78140_e118286_d_n0, assign78140_e118286_d_n2, assign78140_e118286_d_n4, assign78140_e118286_d_n5, assign78140_e118286_d_n6, assign78140_e118286_d_n7, assign78140_e118286_d_n8, assign78140_e118286_d_n9, assign78140_e118286_d_n10, assign78140_e118286_d_n11, assign78140_e118286_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78140_e118284: f64 = (locals.var_xp * locals.var_x2);
        (assign78140_e118284, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78140_e118286;
        locals.var_xp_dn0 = assign78140_e118286_d_n0;
        locals.var_xp_dn2 = assign78140_e118286_d_n2;
        locals.var_xp_dn4 = assign78140_e118286_d_n4;
        locals.var_xp_dn5 = assign78140_e118286_d_n5;
        locals.var_xp_dn6 = assign78140_e118286_d_n6;
        locals.var_xp_dn7 = assign78140_e118286_d_n7;
        locals.var_xp_dn8 = assign78140_e118286_d_n8;
        locals.var_xp_dn9 = assign78140_e118286_d_n9;
        locals.var_xp_dn10 = assign78140_e118286_d_n10;
        locals.var_xp_dn11 = assign78140_e118286_d_n11;
        locals.var_xp_dn14 = assign78140_e118286_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78150_e118299, assign78150_e118299_d_n0, assign78150_e118299_d_n2, assign78150_e118299_d_n4, assign78150_e118299_d_n5, assign78150_e118299_d_n6, assign78150_e118299_d_n7, assign78150_e118299_d_n8, assign78150_e118299_d_n9, assign78150_e118299_d_n10, assign78150_e118299_d_n11, assign78150_e118299_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78150_e118297: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78150_e118297, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78150_e118299;
        locals.var_xmp_dn0 = assign78150_e118299_d_n0;
        locals.var_xmp_dn2 = assign78150_e118299_d_n2;
        locals.var_xmp_dn4 = assign78150_e118299_d_n4;
        locals.var_xmp_dn5 = assign78150_e118299_d_n5;
        locals.var_xmp_dn6 = assign78150_e118299_d_n6;
        locals.var_xmp_dn7 = assign78150_e118299_d_n7;
        locals.var_xmp_dn8 = assign78150_e118299_d_n8;
        locals.var_xmp_dn9 = assign78150_e118299_d_n9;
        locals.var_xmp_dn10 = assign78150_e118299_d_n10;
        locals.var_xmp_dn11 = assign78150_e118299_d_n11;
        locals.var_xmp_dn14 = assign78150_e118299_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78160_e118312, assign78160_e118312_d_n0, assign78160_e118312_d_n2, assign78160_e118312_d_n4, assign78160_e118312_d_n5, assign78160_e118312_d_n6, assign78160_e118312_d_n7, assign78160_e118312_d_n8, assign78160_e118312_d_n9, assign78160_e118312_d_n10, assign78160_e118312_d_n11, assign78160_e118312_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78160_e118310: f64 = (locals.var_xp + locals.var_xmp);
        (assign78160_e118310, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78160_e118312;
        locals.var_arg_dn0 = assign78160_e118312_d_n0;
        locals.var_arg_dn2 = assign78160_e118312_d_n2;
        locals.var_arg_dn4 = assign78160_e118312_d_n4;
        locals.var_arg_dn5 = assign78160_e118312_d_n5;
        locals.var_arg_dn6 = assign78160_e118312_d_n6;
        locals.var_arg_dn7 = assign78160_e118312_d_n7;
        locals.var_arg_dn8 = assign78160_e118312_d_n8;
        locals.var_arg_dn9 = assign78160_e118312_d_n9;
        locals.var_arg_dn10 = assign78160_e118312_d_n10;
        locals.var_arg_dn11 = assign78160_e118312_d_n11;
        locals.var_arg_dn14 = assign78160_e118312_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78170_e118323, assign78170_e118323_d_n0, assign78170_e118323_d_n2, assign78170_e118323_d_n4, assign78170_e118323_d_n5, assign78170_e118323_d_n6, assign78170_e118323_d_n7, assign78170_e118323_d_n8, assign78170_e118323_d_n9, assign78170_e118323_d_n10, assign78170_e118323_d_n11, assign78170_e118323_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78170_e118323;
        locals.var_dnm_dn0 = assign78170_e118323_d_n0;
        locals.var_dnm_dn2 = assign78170_e118323_d_n2;
        locals.var_dnm_dn4 = assign78170_e118323_d_n4;
        locals.var_dnm_dn5 = assign78170_e118323_d_n5;
        locals.var_dnm_dn6 = assign78170_e118323_d_n6;
        locals.var_dnm_dn7 = assign78170_e118323_d_n7;
        locals.var_dnm_dn8 = assign78170_e118323_d_n8;
        locals.var_dnm_dn9 = assign78170_e118323_d_n9;
        locals.var_dnm_dn10 = assign78170_e118323_d_n10;
        locals.var_dnm_dn11 = assign78170_e118323_d_n11;
        locals.var_dnm_dn14 = assign78170_e118323_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign78180_e118338: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1814 = assign78180_e118338;
        locals.var_guard1814_rv = 0.0;

        let assign78190_e118341: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1815 = assign78190_e118341;
        locals.var_guard1815_rv = 0.0;

        let (assign78200_e118356,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78200_e118356;
        locals.var_mm_rv = 0.0;

        let assign78210_e118359: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1816 = assign78210_e118359;
        locals.var_guard1816_rv = 0.0;

        let (assign78220_e118377,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78220_e118377;
        locals.var_mm_rv = 0.0;

        let assign78230_e118380: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1817 = assign78230_e118380;
        locals.var_guard1817_rv = 0.0;

        let (assign78240_e118401,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 == 0.0)) && (locals.var_guard1817 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78240_e118401;
        locals.var_mm_rv = 0.0;

        let assign78250_e118404: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1818 = assign78250_e118404;
        locals.var_guard1818_rv = 0.0;

        let (assign78260_e118428,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 == 0.0)) && (locals.var_guard1817 == 0.0)) && (locals.var_guard1818 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78260_e118428;
        locals.var_mm_rv = 0.0;

        let (assign78270_e118441,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78270_e118441;
        locals.var_m0_rv = 0.0;

        let mut assign78280_loop_guard: usize = 0;
        while {
            let assign78280_cond_e118455: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78280_cond_e118455 != 0.0
        } {
            assign78280_loop_guard += 1;
            assert!(assign78280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78280_body0_e118469, assign78280_body0_e118469_d_n0, assign78280_body0_e118469_d_n2, assign78280_body0_e118469_d_n4, assign78280_body0_e118469_d_n5, assign78280_body0_e118469_d_n6, assign78280_body0_e118469_d_n7, assign78280_body0_e118469_d_n8, assign78280_body0_e118469_d_n9, assign78280_body0_e118469_d_n10, assign78280_body0_e118469_d_n11, assign78280_body0_e118469_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        let assign78280_body0_e118467: f64 = (locals.var_dnm).sqrt();
        (assign78280_body0_e118467, (locals.var_dnm_dn0 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn2 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn4 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn5 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn6 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn7 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn8 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn9 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn10 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn11 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn14 / (2.0 * assign78280_body0_e118467)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign78280_body0_e118469;
            locals.var_dnm_dn0 = assign78280_body0_e118469_d_n0;
            locals.var_dnm_dn2 = assign78280_body0_e118469_d_n2;
            locals.var_dnm_dn4 = assign78280_body0_e118469_d_n4;
            locals.var_dnm_dn5 = assign78280_body0_e118469_d_n5;
            locals.var_dnm_dn6 = assign78280_body0_e118469_d_n6;
            locals.var_dnm_dn7 = assign78280_body0_e118469_d_n7;
            locals.var_dnm_dn8 = assign78280_body0_e118469_d_n8;
            locals.var_dnm_dn9 = assign78280_body0_e118469_d_n9;
            locals.var_dnm_dn10 = assign78280_body0_e118469_d_n10;
            locals.var_dnm_dn11 = assign78280_body0_e118469_d_n11;
            locals.var_dnm_dn14 = assign78280_body0_e118469_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign78280_body1_e118484,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        let assign78280_body1_e118482: f64 = (locals.var_m0 + 1.0);
        (assign78280_body1_e118482,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78280_body1_e118484;
            locals.var_m0_rv = 0.0;
        }

    }
}
