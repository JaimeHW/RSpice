#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_208(
        locals: &mut StampLocals,
    ) {
        let mut assign59370_loop_guard: usize = 0;
        while {
            let assign59370_cond_e92326: f64 = (40.0 + 1.0);
            let assign59370_cond_e92328: f64 = if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_lp_sl <= assign59370_cond_e92326)) { 1.0 } else { 0.0 };
            assign59370_cond_e92328 != 0.0
        } {
            assign59370_loop_guard += 1;
            assert!(assign59370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign59370_body1_e92348, assign59370_body1_e92348_d_n0, assign59370_body1_e92348_d_n2, assign59370_body1_e92348_d_n4, assign59370_body1_e92348_d_n5, assign59370_body1_e92348_d_n6, assign59370_body1_e92348_d_n7, assign59370_body1_e92348_d_n8, assign59370_body1_e92348_d_n9, assign59370_body1_e92348_d_n10, assign59370_body1_e92348_d_n11, assign59370_body1_e92348_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59370_body1_e92345: f64 = (locals.var_psl - locals.var_vbscl__blk437);
        let assign59370_body1_e92346: f64 = (locals.var_beta * assign59370_body1_e92345);
        (assign59370_body1_e92346, ((locals.var_beta_dn0 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vbscl__blk437_dn0))), ((locals.var_beta_dn2 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vbscl__blk437_dn2))), ((locals.var_beta_dn4 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vbscl__blk437_dn4))), ((locals.var_beta_dn5 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vbscl__blk437_dn5))), ((locals.var_beta_dn6 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vbscl__blk437_dn6))), ((locals.var_beta_dn7 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vbscl__blk437_dn7))), ((locals.var_beta_dn8 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vbscl__blk437_dn8))), ((locals.var_beta_dn9 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vbscl__blk437_dn9))), ((locals.var_beta_dn10 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vbscl__blk437_dn10))), ((locals.var_beta_dn11 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn11 - locals.var_vbscl__blk437_dn11))), ((locals.var_beta_dn14 * assign59370_body1_e92345) + (locals.var_beta * (locals.var_psl_dn14 - locals.var_vbscl__blk437_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign59370_body1_e92348;
            locals.var_chi_dn0 = assign59370_body1_e92348_d_n0;
            locals.var_chi_dn2 = assign59370_body1_e92348_d_n2;
            locals.var_chi_dn4 = assign59370_body1_e92348_d_n4;
            locals.var_chi_dn5 = assign59370_body1_e92348_d_n5;
            locals.var_chi_dn6 = assign59370_body1_e92348_d_n6;
            locals.var_chi_dn7 = assign59370_body1_e92348_d_n7;
            locals.var_chi_dn8 = assign59370_body1_e92348_d_n8;
            locals.var_chi_dn9 = assign59370_body1_e92348_d_n9;
            locals.var_chi_dn10 = assign59370_body1_e92348_d_n10;
            locals.var_chi_dn11 = assign59370_body1_e92348_d_n11;
            locals.var_chi_dn14 = assign59370_body1_e92348_d_n14;
            let assign59370_body2_e92351: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1455 = assign59370_body2_e92351;
            let (assign59370_body3_e92375, assign59370_body3_e92375_d_n0, assign59370_body3_e92375_d_n2, assign59370_body3_e92375_d_n4, assign59370_body3_e92375_d_n5, assign59370_body3_e92375_d_n6, assign59370_body3_e92375_d_n7, assign59370_body3_e92375_d_n8, assign59370_body3_e92375_d_n9, assign59370_body3_e92375_d_n10, assign59370_body3_e92375_d_n11, assign59370_body3_e92375_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body3_e92360: f64 = (locals.var_chi * locals.var_chi);
        let assign59370_body3_e92362: f64 = (assign59370_body3_e92360 * locals.var_chi);
        let assign59370_body3_e92366: f64 = (-0.07053654284009761);
        let assign59370_body3_e92369: f64 = (locals.var_chi * 0.006115288895133179);
        let assign59370_body3_e92370: f64 = (assign59370_body3_e92366 + assign59370_body3_e92369);
        let assign59370_body3_e92371: f64 = (locals.var_chi * assign59370_body3_e92370);
        let assign59370_body3_e92372: f64 = (0.29693154855771 + assign59370_body3_e92371);
        let assign59370_body3_e92373: f64 = (assign59370_body3_e92362 * assign59370_body3_e92372);
        (assign59370_body3_e92373, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn0)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn0 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn2)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn2 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn4)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn4 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn5)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn5 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn6)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn6 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn7)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn7 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn8)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn8 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn9)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn9 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn10)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn10 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn11)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn11 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * locals.var_chi) + (assign59370_body3_e92360 * locals.var_chi_dn14)) * assign59370_body3_e92372) + (assign59370_body3_e92362 * ((locals.var_chi_dn14 * assign59370_body3_e92370) + (locals.var_chi * (locals.var_chi_dn14 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn14,)
    }
};
            locals.var_fi = assign59370_body3_e92375;
            locals.var_fi_dn0 = assign59370_body3_e92375_d_n0;
            locals.var_fi_dn2 = assign59370_body3_e92375_d_n2;
            locals.var_fi_dn4 = assign59370_body3_e92375_d_n4;
            locals.var_fi_dn5 = assign59370_body3_e92375_d_n5;
            locals.var_fi_dn6 = assign59370_body3_e92375_d_n6;
            locals.var_fi_dn7 = assign59370_body3_e92375_d_n7;
            locals.var_fi_dn8 = assign59370_body3_e92375_d_n8;
            locals.var_fi_dn9 = assign59370_body3_e92375_d_n9;
            locals.var_fi_dn10 = assign59370_body3_e92375_d_n10;
            locals.var_fi_dn11 = assign59370_body3_e92375_d_n11;
            locals.var_fi_dn14 = assign59370_body3_e92375_d_n14;
            let (assign59370_body4_e92403, assign59370_body4_e92403_d_n0, assign59370_body4_e92403_d_n2, assign59370_body4_e92403_d_n4, assign59370_body4_e92403_d_n5, assign59370_body4_e92403_d_n6, assign59370_body4_e92403_d_n7, assign59370_body4_e92403_d_n8, assign59370_body4_e92403_d_n9, assign59370_body4_e92403_d_n10, assign59370_body4_e92403_d_n11, assign59370_body4_e92403_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body4_e92384: f64 = (locals.var_chi * locals.var_chi);
        let assign59370_body4_e92387: f64 = (3.0 * 0.29693154855771);
        let assign59370_body4_e92391: f64 = (-0.07053654284009761);
        let assign59370_body4_e92392: f64 = (4.0 * assign59370_body4_e92391);
        let assign59370_body4_e92395: f64 = (locals.var_chi * 5.0);
        let assign59370_body4_e92397: f64 = (assign59370_body4_e92395 * 0.006115288895133179);
        let assign59370_body4_e92398: f64 = (assign59370_body4_e92392 + assign59370_body4_e92397);
        let assign59370_body4_e92399: f64 = (locals.var_chi * assign59370_body4_e92398);
        let assign59370_body4_e92400: f64 = (assign59370_body4_e92387 + assign59370_body4_e92399);
        let assign59370_body4_e92401: f64 = (assign59370_body4_e92384 * assign59370_body4_e92400);
        (assign59370_body4_e92401, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn0 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn2 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn4 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn5 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn6 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn7 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn8 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn9 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn10 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn11 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * assign59370_body4_e92400) + (assign59370_body4_e92384 * ((locals.var_chi_dn14 * assign59370_body4_e92398) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn14,)
    }
};
            locals.var_fi_dchi = assign59370_body4_e92403;
            locals.var_fi_dchi_dn0 = assign59370_body4_e92403_d_n0;
            locals.var_fi_dchi_dn2 = assign59370_body4_e92403_d_n2;
            locals.var_fi_dchi_dn4 = assign59370_body4_e92403_d_n4;
            locals.var_fi_dchi_dn5 = assign59370_body4_e92403_d_n5;
            locals.var_fi_dchi_dn6 = assign59370_body4_e92403_d_n6;
            locals.var_fi_dchi_dn7 = assign59370_body4_e92403_d_n7;
            locals.var_fi_dchi_dn8 = assign59370_body4_e92403_d_n8;
            locals.var_fi_dchi_dn9 = assign59370_body4_e92403_d_n9;
            locals.var_fi_dchi_dn10 = assign59370_body4_e92403_d_n10;
            locals.var_fi_dchi_dn11 = assign59370_body4_e92403_d_n11;
            locals.var_fi_dchi_dn14 = assign59370_body4_e92403_d_n14;
            let (assign59370_body5_e92414, assign59370_body5_e92414_d_n0, assign59370_body5_e92414_d_n2, assign59370_body5_e92414_d_n4, assign59370_body5_e92414_d_n5, assign59370_body5_e92414_d_n6, assign59370_body5_e92414_d_n7, assign59370_body5_e92414_d_n8, assign59370_body5_e92414_d_n9, assign59370_body5_e92414_d_n10, assign59370_body5_e92414_d_n11, assign59370_body5_e92414_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body5_e92412: f64 = (locals.var_cnst1 * locals.var_exp_bvbsvds);
        (assign59370_body5_e92412, ((locals.var_cnst1_dn0 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn10)), ((locals.var_cnst1_dn11 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn11)), ((locals.var_cnst1_dn14 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
            locals.var_cfs1 = assign59370_body5_e92414;
            locals.var_cfs1_dn0 = assign59370_body5_e92414_d_n0;
            locals.var_cfs1_dn2 = assign59370_body5_e92414_d_n2;
            locals.var_cfs1_dn4 = assign59370_body5_e92414_d_n4;
            locals.var_cfs1_dn5 = assign59370_body5_e92414_d_n5;
            locals.var_cfs1_dn6 = assign59370_body5_e92414_d_n6;
            locals.var_cfs1_dn7 = assign59370_body5_e92414_d_n7;
            locals.var_cfs1_dn8 = assign59370_body5_e92414_d_n8;
            locals.var_cfs1_dn9 = assign59370_body5_e92414_d_n9;
            locals.var_cfs1_dn10 = assign59370_body5_e92414_d_n10;
            locals.var_cfs1_dn11 = assign59370_body5_e92414_d_n11;
            locals.var_cfs1_dn14 = assign59370_body5_e92414_d_n14;
            let (assign59370_body6_e92427, assign59370_body6_e92427_d_n0, assign59370_body6_e92427_d_n2, assign59370_body6_e92427_d_n4, assign59370_body6_e92427_d_n5, assign59370_body6_e92427_d_n6, assign59370_body6_e92427_d_n7, assign59370_body6_e92427_d_n8, assign59370_body6_e92427_d_n9, assign59370_body6_e92427_d_n10, assign59370_body6_e92427_d_n11, assign59370_body6_e92427_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body6_e92423: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign59370_body6_e92425: f64 = (assign59370_body6_e92423 * locals.var_fi);
        (assign59370_body6_e92425, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn14)) * locals.var_fi) + (assign59370_body6_e92423 * locals.var_fi_dn14)),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn14,)
    }
};
            locals.var_fsl1 = assign59370_body6_e92427;
            locals.var_fsl1_dn0 = assign59370_body6_e92427_d_n0;
            locals.var_fsl1_dn2 = assign59370_body6_e92427_d_n2;
            locals.var_fsl1_dn4 = assign59370_body6_e92427_d_n4;
            locals.var_fsl1_dn5 = assign59370_body6_e92427_d_n5;
            locals.var_fsl1_dn6 = assign59370_body6_e92427_d_n6;
            locals.var_fsl1_dn7 = assign59370_body6_e92427_d_n7;
            locals.var_fsl1_dn8 = assign59370_body6_e92427_d_n8;
            locals.var_fsl1_dn9 = assign59370_body6_e92427_d_n9;
            locals.var_fsl1_dn10 = assign59370_body6_e92427_d_n10;
            locals.var_fsl1_dn11 = assign59370_body6_e92427_d_n11;
            locals.var_fsl1_dn14 = assign59370_body6_e92427_d_n14;
            let (assign59370_body7_e92444, assign59370_body7_e92444_d_n0, assign59370_body7_e92444_d_n2, assign59370_body7_e92444_d_n4, assign59370_body7_e92444_d_n5, assign59370_body7_e92444_d_n6, assign59370_body7_e92444_d_n7, assign59370_body7_e92444_d_n8, assign59370_body7_e92444_d_n9, assign59370_body7_e92444_d_n10, assign59370_body7_e92444_d_n11, assign59370_body7_e92444_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body7_e92436: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign59370_body7_e92438: f64 = (assign59370_body7_e92436 * 2.0);
        let assign59370_body7_e92440: f64 = (assign59370_body7_e92438 * locals.var_fi);
        let assign59370_body7_e92442: f64 = (assign59370_body7_e92440 * locals.var_fi_dchi);
        (assign59370_body7_e92442, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn11)), (((((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * 2.0) * locals.var_fi) + (assign59370_body7_e92438 * locals.var_fi_dn14)) * locals.var_fi_dchi) + (assign59370_body7_e92440 * locals.var_fi_dchi_dn14)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn14,)
    }
};
            locals.var_fsl1_dpsl = assign59370_body7_e92444;
            locals.var_fsl1_dpsl_dn0 = assign59370_body7_e92444_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59370_body7_e92444_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59370_body7_e92444_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59370_body7_e92444_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59370_body7_e92444_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59370_body7_e92444_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59370_body7_e92444_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59370_body7_e92444_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59370_body7_e92444_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign59370_body7_e92444_d_n11;
            locals.var_fsl1_dpsl_dn14 = assign59370_body7_e92444_d_n14;
            let (assign59370_body8_e92473, assign59370_body8_e92473_d_n0, assign59370_body8_e92473_d_n2, assign59370_body8_e92473_d_n4, assign59370_body8_e92473_d_n5, assign59370_body8_e92473_d_n6, assign59370_body8_e92473_d_n7, assign59370_body8_e92473_d_n8, assign59370_body8_e92473_d_n9, assign59370_body8_e92473_d_n10, assign59370_body8_e92473_d_n11, assign59370_body8_e92473_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body8_e92455: f64 = (-0.117851130197758);
        let assign59370_body8_e92460: f64 = (-0.00163730162779191);
        let assign59370_body8_e92463: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign59370_body8_e92464: f64 = (assign59370_body8_e92460 + assign59370_body8_e92463);
        let assign59370_body8_e92465: f64 = (locals.var_chi * assign59370_body8_e92464);
        let assign59370_body8_e92466: f64 = (0.0178800506338833 + assign59370_body8_e92465);
        let assign59370_body8_e92467: f64 = (locals.var_chi * assign59370_body8_e92466);
        let assign59370_body8_e92468: f64 = (assign59370_body8_e92455 + assign59370_body8_e92467);
        let assign59370_body8_e92469: f64 = (locals.var_chi * assign59370_body8_e92468);
        let assign59370_body8_e92470: f64 = (0.707106781186548 + assign59370_body8_e92469);
        let assign59370_body8_e92471: f64 = (locals.var_chi * assign59370_body8_e92470);
        (assign59370_body8_e92471, ((locals.var_chi_dn0 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn0 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn0 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn0 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn2 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn2 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn2 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn4 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn4 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn4 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn5 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn5 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn5 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn6 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn6 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn6 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn7 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn7 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn7 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn8 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn8 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn8 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn9 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn9 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn9 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn10 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn10 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn10 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn11 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn11 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn11 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn14 * assign59370_body8_e92470) + (locals.var_chi * ((locals.var_chi_dn14 * assign59370_body8_e92468) + (locals.var_chi * ((locals.var_chi_dn14 * assign59370_body8_e92466) + (locals.var_chi * ((locals.var_chi_dn14 * assign59370_body8_e92464) + (locals.var_chi * (locals.var_chi_dn14 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign59370_body8_e92473;
            locals.var_fb_dn0 = assign59370_body8_e92473_d_n0;
            locals.var_fb_dn2 = assign59370_body8_e92473_d_n2;
            locals.var_fb_dn4 = assign59370_body8_e92473_d_n4;
            locals.var_fb_dn5 = assign59370_body8_e92473_d_n5;
            locals.var_fb_dn6 = assign59370_body8_e92473_d_n6;
            locals.var_fb_dn7 = assign59370_body8_e92473_d_n7;
            locals.var_fb_dn8 = assign59370_body8_e92473_d_n8;
            locals.var_fb_dn9 = assign59370_body8_e92473_d_n9;
            locals.var_fb_dn10 = assign59370_body8_e92473_d_n10;
            locals.var_fb_dn11 = assign59370_body8_e92473_d_n11;
            locals.var_fb_dn14 = assign59370_body8_e92473_d_n14;
            let (assign59370_body9_e92508, assign59370_body9_e92508_d_n0, assign59370_body9_e92508_d_n2, assign59370_body9_e92508_d_n4, assign59370_body9_e92508_d_n5, assign59370_body9_e92508_d_n6, assign59370_body9_e92508_d_n7, assign59370_body9_e92508_d_n8, assign59370_body9_e92508_d_n9, assign59370_body9_e92508_d_n10, assign59370_body9_e92508_d_n11, assign59370_body9_e92508_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body9_e92484: f64 = (-0.117851130197758);
        let assign59370_body9_e92485: f64 = (2.0 * assign59370_body9_e92484);
        let assign59370_body9_e92489: f64 = (3.0 * 0.0178800506338833);
        let assign59370_body9_e92493: f64 = (-0.00163730162779191);
        let assign59370_body9_e92494: f64 = (4.0 * assign59370_body9_e92493);
        let assign59370_body9_e92497: f64 = (locals.var_chi * 5.0);
        let assign59370_body9_e92499: f64 = (assign59370_body9_e92497 * 6.36964918866352e-5);
        let assign59370_body9_e92500: f64 = (assign59370_body9_e92494 + assign59370_body9_e92499);
        let assign59370_body9_e92501: f64 = (locals.var_chi * assign59370_body9_e92500);
        let assign59370_body9_e92502: f64 = (assign59370_body9_e92489 + assign59370_body9_e92501);
        let assign59370_body9_e92503: f64 = (locals.var_chi * assign59370_body9_e92502);
        let assign59370_body9_e92504: f64 = (assign59370_body9_e92485 + assign59370_body9_e92503);
        let assign59370_body9_e92505: f64 = (locals.var_chi * assign59370_body9_e92504);
        let assign59370_body9_e92506: f64 = (0.707106781186548 + assign59370_body9_e92505);
        (assign59370_body9_e92506, ((locals.var_chi_dn0 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn0 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn0 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn2 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn2 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn4 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn4 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn5 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn5 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn6 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn6 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn7 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn7 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn8 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn8 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn9 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn9 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn10 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn10 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn11 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn11 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn14 * assign59370_body9_e92504) + (locals.var_chi * ((locals.var_chi_dn14 * assign59370_body9_e92502) + (locals.var_chi * ((locals.var_chi_dn14 * assign59370_body9_e92500) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn14,)
    }
};
            locals.var_fb_dchi = assign59370_body9_e92508;
            locals.var_fb_dchi_dn0 = assign59370_body9_e92508_d_n0;
            locals.var_fb_dchi_dn2 = assign59370_body9_e92508_d_n2;
            locals.var_fb_dchi_dn4 = assign59370_body9_e92508_d_n4;
            locals.var_fb_dchi_dn5 = assign59370_body9_e92508_d_n5;
            locals.var_fb_dchi_dn6 = assign59370_body9_e92508_d_n6;
            locals.var_fb_dchi_dn7 = assign59370_body9_e92508_d_n7;
            locals.var_fb_dchi_dn8 = assign59370_body9_e92508_d_n8;
            locals.var_fb_dchi_dn9 = assign59370_body9_e92508_d_n9;
            locals.var_fb_dchi_dn10 = assign59370_body9_e92508_d_n10;
            locals.var_fb_dchi_dn11 = assign59370_body9_e92508_d_n11;
            locals.var_fb_dchi_dn14 = assign59370_body9_e92508_d_n14;
            let (assign59370_body10_e92522, assign59370_body10_e92522_d_n0, assign59370_body10_e92522_d_n2, assign59370_body10_e92522_d_n4, assign59370_body10_e92522_d_n5, assign59370_body10_e92522_d_n6, assign59370_body10_e92522_d_n7, assign59370_body10_e92522_d_n8, assign59370_body10_e92522_d_n9, assign59370_body10_e92522_d_n10, assign59370_body10_e92522_d_n11, assign59370_body10_e92522_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body10_e92517: f64 = (locals.var_fb * locals.var_fb);
        let assign59370_body10_e92519: f64 = (assign59370_body10_e92517 + locals.var_fsl1);
        let assign59370_body10_e92520: f64 = (assign59370_body10_e92519).sqrt();
        (assign59370_body10_e92520, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fsl1_dn4) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fsl1_dn5) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fsl1_dn8) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fsl1_dn9) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign59370_body10_e92520)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fsl1_dn14) / (2.0 * assign59370_body10_e92520)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn14,)
    }
};
            locals.var_fsl2 = assign59370_body10_e92522;
            locals.var_fsl2_dn0 = assign59370_body10_e92522_d_n0;
            locals.var_fsl2_dn2 = assign59370_body10_e92522_d_n2;
            locals.var_fsl2_dn4 = assign59370_body10_e92522_d_n4;
            locals.var_fsl2_dn5 = assign59370_body10_e92522_d_n5;
            locals.var_fsl2_dn6 = assign59370_body10_e92522_d_n6;
            locals.var_fsl2_dn7 = assign59370_body10_e92522_d_n7;
            locals.var_fsl2_dn8 = assign59370_body10_e92522_d_n8;
            locals.var_fsl2_dn9 = assign59370_body10_e92522_d_n9;
            locals.var_fsl2_dn10 = assign59370_body10_e92522_d_n10;
            locals.var_fsl2_dn11 = assign59370_body10_e92522_d_n11;
            locals.var_fsl2_dn14 = assign59370_body10_e92522_d_n14;
            let (assign59370_body11_e92543, assign59370_body11_e92543_d_n0, assign59370_body11_e92543_d_n2, assign59370_body11_e92543_d_n4, assign59370_body11_e92543_d_n5, assign59370_body11_e92543_d_n6, assign59370_body11_e92543_d_n7, assign59370_body11_e92543_d_n8, assign59370_body11_e92543_d_n9, assign59370_body11_e92543_d_n10, assign59370_body11_e92543_d_n11, assign59370_body11_e92543_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        let assign59370_body11_e92531: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign59370_body11_e92533: f64 = (assign59370_body11_e92531 * 2.0);
        let assign59370_body11_e92535: f64 = (assign59370_body11_e92533 * locals.var_fb);
        let assign59370_body11_e92537: f64 = (assign59370_body11_e92535 + locals.var_fsl1_dpsl);
        let assign59370_body11_e92540: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59370_body11_e92541: f64 = (assign59370_body11_e92537 / assign59370_body11_e92540);
        (assign59370_body11_e92541, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn4)) + locals.var_fsl1_dpsl_dn4) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn5)) + locals.var_fsl1_dpsl_dn5) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn8)) + locals.var_fsl1_dpsl_dn8) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn9)) + locals.var_fsl1_dpsl_dn9) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn11 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn11)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn11 + locals.var_fsl2_dn11))) / (assign59370_body11_e92540 * assign59370_body11_e92540)), (((((((((locals.var_beta_dn14 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn14)) * 2.0) * locals.var_fb) + (assign59370_body11_e92533 * locals.var_fb_dn14)) + locals.var_fsl1_dpsl_dn14) * assign59370_body11_e92540) - (assign59370_body11_e92537 * (locals.var_fsl2_dn14 + locals.var_fsl2_dn14))) / (assign59370_body11_e92540 * assign59370_body11_e92540)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn14,)
    }
};
            locals.var_fsl2_dpsl = assign59370_body11_e92543;
            locals.var_fsl2_dpsl_dn0 = assign59370_body11_e92543_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59370_body11_e92543_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59370_body11_e92543_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59370_body11_e92543_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59370_body11_e92543_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59370_body11_e92543_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59370_body11_e92543_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59370_body11_e92543_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59370_body11_e92543_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign59370_body11_e92543_d_n11;
            locals.var_fsl2_dpsl_dn14 = assign59370_body11_e92543_d_n14;
            let (assign59370_body12_e92557, assign59370_body12_e92557_d_n0, assign59370_body12_e92557_d_n2, assign59370_body12_e92557_d_n4, assign59370_body12_e92557_d_n5, assign59370_body12_e92557_d_n6, assign59370_body12_e92557_d_n7, assign59370_body12_e92557_d_n8, assign59370_body12_e92557_d_n9, assign59370_body12_e92557_d_n10, assign59370_body12_e92557_d_n11, assign59370_body12_e92557_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body12_e92554: f64 = (locals.var_psl - locals.var_vds);
        let assign59370_body12_e92555: f64 = (locals.var_beta * assign59370_body12_e92554);
        (assign59370_body12_e92555, ((locals.var_beta_dn0 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vds_dn0))), ((locals.var_beta_dn2 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vds_dn2))), ((locals.var_beta_dn4 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vds_dn4))), ((locals.var_beta_dn5 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vds_dn5))), ((locals.var_beta_dn6 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vds_dn6))), ((locals.var_beta_dn7 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vds_dn7))), ((locals.var_beta_dn8 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vds_dn8))), ((locals.var_beta_dn9 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vds_dn9))), ((locals.var_beta_dn10 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vds_dn10))), ((locals.var_beta_dn11 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn11 - locals.var_vds_dn11))), ((locals.var_beta_dn14 * assign59370_body12_e92554) + (locals.var_beta * (locals.var_psl_dn14 - locals.var_vds_dn14))),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn14,)
    }
};
            locals.var_rho = assign59370_body12_e92557;
            locals.var_rho_dn0 = assign59370_body12_e92557_d_n0;
            locals.var_rho_dn2 = assign59370_body12_e92557_d_n2;
            locals.var_rho_dn4 = assign59370_body12_e92557_d_n4;
            locals.var_rho_dn5 = assign59370_body12_e92557_d_n5;
            locals.var_rho_dn6 = assign59370_body12_e92557_d_n6;
            locals.var_rho_dn7 = assign59370_body12_e92557_d_n7;
            locals.var_rho_dn8 = assign59370_body12_e92557_d_n8;
            locals.var_rho_dn9 = assign59370_body12_e92557_d_n9;
            locals.var_rho_dn10 = assign59370_body12_e92557_d_n10;
            locals.var_rho_dn11 = assign59370_body12_e92557_d_n11;
            locals.var_rho_dn14 = assign59370_body12_e92557_d_n14;
            let (assign59370_body13_e92568, assign59370_body13_e92568_d_n0, assign59370_body13_e92568_d_n2, assign59370_body13_e92568_d_n4, assign59370_body13_e92568_d_n5, assign59370_body13_e92568_d_n6, assign59370_body13_e92568_d_n7, assign59370_body13_e92568_d_n8, assign59370_body13_e92568_d_n9, assign59370_body13_e92568_d_n10, assign59370_body13_e92568_d_n11, assign59370_body13_e92568_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body13_e92566: f64 = (locals.var_rho).exp();
        (assign59370_body13_e92566, (assign59370_body13_e92566 * locals.var_rho_dn0), (assign59370_body13_e92566 * locals.var_rho_dn2), (assign59370_body13_e92566 * locals.var_rho_dn4), (assign59370_body13_e92566 * locals.var_rho_dn5), (assign59370_body13_e92566 * locals.var_rho_dn6), (assign59370_body13_e92566 * locals.var_rho_dn7), (assign59370_body13_e92566 * locals.var_rho_dn8), (assign59370_body13_e92566 * locals.var_rho_dn9), (assign59370_body13_e92566 * locals.var_rho_dn10), (assign59370_body13_e92566 * locals.var_rho_dn11), (assign59370_body13_e92566 * locals.var_rho_dn14),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn14,)
    }
};
            locals.var_exp_rho = assign59370_body13_e92568;
            locals.var_exp_rho_dn0 = assign59370_body13_e92568_d_n0;
            locals.var_exp_rho_dn2 = assign59370_body13_e92568_d_n2;
            locals.var_exp_rho_dn4 = assign59370_body13_e92568_d_n4;
            locals.var_exp_rho_dn5 = assign59370_body13_e92568_d_n5;
            locals.var_exp_rho_dn6 = assign59370_body13_e92568_d_n6;
            locals.var_exp_rho_dn7 = assign59370_body13_e92568_d_n7;
            locals.var_exp_rho_dn8 = assign59370_body13_e92568_d_n8;
            locals.var_exp_rho_dn9 = assign59370_body13_e92568_d_n9;
            locals.var_exp_rho_dn10 = assign59370_body13_e92568_d_n10;
            locals.var_exp_rho_dn11 = assign59370_body13_e92568_d_n11;
            locals.var_exp_rho_dn14 = assign59370_body13_e92568_d_n14;
            let (assign59370_body14_e92582, assign59370_body14_e92582_d_n0, assign59370_body14_e92582_d_n2, assign59370_body14_e92582_d_n4, assign59370_body14_e92582_d_n5, assign59370_body14_e92582_d_n6, assign59370_body14_e92582_d_n7, assign59370_body14_e92582_d_n8, assign59370_body14_e92582_d_n9, assign59370_body14_e92582_d_n10, assign59370_body14_e92582_d_n11, assign59370_body14_e92582_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body14_e92579: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign59370_body14_e92580: f64 = (locals.var_cnst1 * assign59370_body14_e92579);
        (assign59370_body14_e92580, ((locals.var_cnst1_dn0 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((locals.var_cnst1_dn2 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((locals.var_cnst1_dn4 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((locals.var_cnst1_dn5 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((locals.var_cnst1_dn6 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((locals.var_cnst1_dn7 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((locals.var_cnst1_dn8 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((locals.var_cnst1_dn9 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((locals.var_cnst1_dn10 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((locals.var_cnst1_dn11 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), ((locals.var_cnst1_dn14 * assign59370_body14_e92579) + (locals.var_cnst1 * (locals.var_exp_rho_dn14 - locals.var_exp_bvbsvds_dn14))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn14,)
    }
};
            locals.var_fsl1 = assign59370_body14_e92582;
            locals.var_fsl1_dn0 = assign59370_body14_e92582_d_n0;
            locals.var_fsl1_dn2 = assign59370_body14_e92582_d_n2;
            locals.var_fsl1_dn4 = assign59370_body14_e92582_d_n4;
            locals.var_fsl1_dn5 = assign59370_body14_e92582_d_n5;
            locals.var_fsl1_dn6 = assign59370_body14_e92582_d_n6;
            locals.var_fsl1_dn7 = assign59370_body14_e92582_d_n7;
            locals.var_fsl1_dn8 = assign59370_body14_e92582_d_n8;
            locals.var_fsl1_dn9 = assign59370_body14_e92582_d_n9;
            locals.var_fsl1_dn10 = assign59370_body14_e92582_d_n10;
            locals.var_fsl1_dn11 = assign59370_body14_e92582_d_n11;
            locals.var_fsl1_dn14 = assign59370_body14_e92582_d_n14;
            let (assign59370_body15_e92596, assign59370_body15_e92596_d_n0, assign59370_body15_e92596_d_n2, assign59370_body15_e92596_d_n4, assign59370_body15_e92596_d_n5, assign59370_body15_e92596_d_n6, assign59370_body15_e92596_d_n7, assign59370_body15_e92596_d_n8, assign59370_body15_e92596_d_n9, assign59370_body15_e92596_d_n10, assign59370_body15_e92596_d_n11, assign59370_body15_e92596_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body15_e92592: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign59370_body15_e92594: f64 = (assign59370_body15_e92592 * locals.var_exp_rho);
        (assign59370_body15_e92594, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn10)), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn11)), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * locals.var_exp_rho) + (assign59370_body15_e92592 * locals.var_exp_rho_dn14)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn14,)
    }
};
            locals.var_fsl1_dpsl = assign59370_body15_e92596;
            locals.var_fsl1_dpsl_dn0 = assign59370_body15_e92596_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59370_body15_e92596_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59370_body15_e92596_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59370_body15_e92596_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59370_body15_e92596_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59370_body15_e92596_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59370_body15_e92596_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59370_body15_e92596_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59370_body15_e92596_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign59370_body15_e92596_d_n11;
            locals.var_fsl1_dpsl_dn14 = assign59370_body15_e92596_d_n14;
            let (assign59370_body16_e92608, assign59370_body16_e92608_d_n0, assign59370_body16_e92608_d_n2, assign59370_body16_e92608_d_n4, assign59370_body16_e92608_d_n5, assign59370_body16_e92608_d_n6, assign59370_body16_e92608_d_n7, assign59370_body16_e92608_d_n8, assign59370_body16_e92608_d_n9, assign59370_body16_e92608_d_n10, assign59370_body16_e92608_d_n11, assign59370_body16_e92608_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body16_e92606: f64 = (locals.var_chi - 1.0);
        (assign59370_body16_e92606, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
            locals.var_xil = assign59370_body16_e92608;
            locals.var_xil_dn0 = assign59370_body16_e92608_d_n0;
            locals.var_xil_dn2 = assign59370_body16_e92608_d_n2;
            locals.var_xil_dn4 = assign59370_body16_e92608_d_n4;
            locals.var_xil_dn5 = assign59370_body16_e92608_d_n5;
            locals.var_xil_dn6 = assign59370_body16_e92608_d_n6;
            locals.var_xil_dn7 = assign59370_body16_e92608_d_n7;
            locals.var_xil_dn8 = assign59370_body16_e92608_d_n8;
            locals.var_xil_dn9 = assign59370_body16_e92608_d_n9;
            locals.var_xil_dn10 = assign59370_body16_e92608_d_n10;
            locals.var_xil_dn11 = assign59370_body16_e92608_d_n11;
            locals.var_xil_dn14 = assign59370_body16_e92608_d_n14;
            let (assign59370_body17_e92621, assign59370_body17_e92621_d_n0, assign59370_body17_e92621_d_n2, assign59370_body17_e92621_d_n4, assign59370_body17_e92621_d_n5, assign59370_body17_e92621_d_n6, assign59370_body17_e92621_d_n7, assign59370_body17_e92621_d_n8, assign59370_body17_e92621_d_n9, assign59370_body17_e92621_d_n10, assign59370_body17_e92621_d_n11, assign59370_body17_e92621_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body17_e92618: f64 = (locals.var_xil + locals.var_fsl1);
        let assign59370_body17_e92619: f64 = (assign59370_body17_e92618).sqrt();
        (assign59370_body17_e92619, ((locals.var_xil_dn0 + locals.var_fsl1_dn0) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn2 + locals.var_fsl1_dn2) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn4 + locals.var_fsl1_dn4) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn5 + locals.var_fsl1_dn5) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn6 + locals.var_fsl1_dn6) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn7 + locals.var_fsl1_dn7) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn8 + locals.var_fsl1_dn8) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn9 + locals.var_fsl1_dn9) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn10 + locals.var_fsl1_dn10) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn11 + locals.var_fsl1_dn11) / (2.0 * assign59370_body17_e92619)), ((locals.var_xil_dn14 + locals.var_fsl1_dn14) / (2.0 * assign59370_body17_e92619)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn14,)
    }
};
            locals.var_fsl2 = assign59370_body17_e92621;
            locals.var_fsl2_dn0 = assign59370_body17_e92621_d_n0;
            locals.var_fsl2_dn2 = assign59370_body17_e92621_d_n2;
            locals.var_fsl2_dn4 = assign59370_body17_e92621_d_n4;
            locals.var_fsl2_dn5 = assign59370_body17_e92621_d_n5;
            locals.var_fsl2_dn6 = assign59370_body17_e92621_d_n6;
            locals.var_fsl2_dn7 = assign59370_body17_e92621_d_n7;
            locals.var_fsl2_dn8 = assign59370_body17_e92621_d_n8;
            locals.var_fsl2_dn9 = assign59370_body17_e92621_d_n9;
            locals.var_fsl2_dn10 = assign59370_body17_e92621_d_n10;
            locals.var_fsl2_dn11 = assign59370_body17_e92621_d_n11;
            locals.var_fsl2_dn14 = assign59370_body17_e92621_d_n14;
            let (assign59370_body18_e92637, assign59370_body18_e92637_d_n0, assign59370_body18_e92637_d_n2, assign59370_body18_e92637_d_n4, assign59370_body18_e92637_d_n5, assign59370_body18_e92637_d_n6, assign59370_body18_e92637_d_n7, assign59370_body18_e92637_d_n8, assign59370_body18_e92637_d_n9, assign59370_body18_e92637_d_n10, assign59370_body18_e92637_d_n11, assign59370_body18_e92637_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1455 == 0.0)) {
        let assign59370_body18_e92631: f64 = (locals.var_beta + locals.var_fsl1_dpsl);
        let assign59370_body18_e92634: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59370_body18_e92635: f64 = (assign59370_body18_e92631 / assign59370_body18_e92634);
        (assign59370_body18_e92635, ((((locals.var_beta_dn0 + locals.var_fsl1_dpsl_dn0) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn2 + locals.var_fsl1_dpsl_dn2) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn4 + locals.var_fsl1_dpsl_dn4) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn5 + locals.var_fsl1_dpsl_dn5) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn6 + locals.var_fsl1_dpsl_dn6) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn7 + locals.var_fsl1_dpsl_dn7) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn8 + locals.var_fsl1_dpsl_dn8) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn9 + locals.var_fsl1_dpsl_dn9) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn10 + locals.var_fsl1_dpsl_dn10) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn11 + locals.var_fsl1_dpsl_dn11) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn11 + locals.var_fsl2_dn11))) / (assign59370_body18_e92634 * assign59370_body18_e92634)), ((((locals.var_beta_dn14 + locals.var_fsl1_dpsl_dn14) * assign59370_body18_e92634) - (assign59370_body18_e92631 * (locals.var_fsl2_dn14 + locals.var_fsl2_dn14))) / (assign59370_body18_e92634 * assign59370_body18_e92634)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn14,)
    }
};
            locals.var_fsl2_dpsl = assign59370_body18_e92637;
            locals.var_fsl2_dpsl_dn0 = assign59370_body18_e92637_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59370_body18_e92637_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59370_body18_e92637_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59370_body18_e92637_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59370_body18_e92637_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59370_body18_e92637_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59370_body18_e92637_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59370_body18_e92637_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59370_body18_e92637_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign59370_body18_e92637_d_n11;
            locals.var_fsl2_dpsl_dn14 = assign59370_body18_e92637_d_n14;
            let (assign59370_body19_e92650, assign59370_body19_e92650_d_n0, assign59370_body19_e92650_d_n2, assign59370_body19_e92650_d_n4, assign59370_body19_e92650_d_n5, assign59370_body19_e92650_d_n6, assign59370_body19_e92650_d_n7, assign59370_body19_e92650_d_n8, assign59370_body19_e92650_d_n9, assign59370_body19_e92650_d_n10, assign59370_body19_e92650_d_n11, assign59370_body19_e92650_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59370_body19_e92644: f64 = (locals.var_vgp - locals.var_psl);
        let assign59370_body19_e92647: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign59370_body19_e92648: f64 = (assign59370_body19_e92644 - assign59370_body19_e92647);
        (assign59370_body19_e92648, ((locals.var_vgp_dn0 - locals.var_psl_dn0) - ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))), ((locals.var_vgp_dn2 - locals.var_psl_dn2) - ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))), ((locals.var_vgp_dn4 - locals.var_psl_dn4) - ((locals.var_fac1_dn4 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn4))), ((locals.var_vgp_dn5 - locals.var_psl_dn5) - ((locals.var_fac1_dn5 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn5))), ((locals.var_vgp_dn6 - locals.var_psl_dn6) - ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))), ((locals.var_vgp_dn7 - locals.var_psl_dn7) - ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))), ((locals.var_vgp_dn8 - locals.var_psl_dn8) - ((locals.var_fac1_dn8 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn8))), ((locals.var_vgp_dn9 - locals.var_psl_dn9) - ((locals.var_fac1_dn9 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn9))), ((locals.var_vgp_dn10 - locals.var_psl_dn10) - ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))), ((locals.var_vgp_dn11 - locals.var_psl_dn11) - ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))), ((locals.var_vgp_dn14 - locals.var_psl_dn14) - ((locals.var_fac1_dn14 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn14))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn4, locals.var_fsl_dn5, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn8, locals.var_fsl_dn9, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn14,)
    }
};
            locals.var_fsl = assign59370_body19_e92650;
            locals.var_fsl_dn0 = assign59370_body19_e92650_d_n0;
            locals.var_fsl_dn2 = assign59370_body19_e92650_d_n2;
            locals.var_fsl_dn4 = assign59370_body19_e92650_d_n4;
            locals.var_fsl_dn5 = assign59370_body19_e92650_d_n5;
            locals.var_fsl_dn6 = assign59370_body19_e92650_d_n6;
            locals.var_fsl_dn7 = assign59370_body19_e92650_d_n7;
            locals.var_fsl_dn8 = assign59370_body19_e92650_d_n8;
            locals.var_fsl_dn9 = assign59370_body19_e92650_d_n9;
            locals.var_fsl_dn10 = assign59370_body19_e92650_d_n10;
            locals.var_fsl_dn11 = assign59370_body19_e92650_d_n11;
            locals.var_fsl_dn14 = assign59370_body19_e92650_d_n14;
            let (assign59370_body20_e92662, assign59370_body20_e92662_d_n0, assign59370_body20_e92662_d_n2, assign59370_body20_e92662_d_n4, assign59370_body20_e92662_d_n5, assign59370_body20_e92662_d_n6, assign59370_body20_e92662_d_n7, assign59370_body20_e92662_d_n8, assign59370_body20_e92662_d_n9, assign59370_body20_e92662_d_n10, assign59370_body20_e92662_d_n11, assign59370_body20_e92662_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59370_body20_e92656: f64 = (-1.0);
        let assign59370_body20_e92659: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign59370_body20_e92660: f64 = (assign59370_body20_e92656 - assign59370_body20_e92659);
        (assign59370_body20_e92660, (-((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0))), (-((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2))), (-((locals.var_fac1_dn4 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn4))), (-((locals.var_fac1_dn5 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn5))), (-((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6))), (-((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7))), (-((locals.var_fac1_dn8 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn8))), (-((locals.var_fac1_dn9 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn9))), (-((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10))), (-((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11))), (-((locals.var_fac1_dn14 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn14))),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn4, locals.var_fsl_dpsl_dn5, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn8, locals.var_fsl_dpsl_dn9, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn14,)
    }
};
            locals.var_fsl_dpsl = assign59370_body20_e92662;
            locals.var_fsl_dpsl_dn0 = assign59370_body20_e92662_d_n0;
            locals.var_fsl_dpsl_dn2 = assign59370_body20_e92662_d_n2;
            locals.var_fsl_dpsl_dn4 = assign59370_body20_e92662_d_n4;
            locals.var_fsl_dpsl_dn5 = assign59370_body20_e92662_d_n5;
            locals.var_fsl_dpsl_dn6 = assign59370_body20_e92662_d_n6;
            locals.var_fsl_dpsl_dn7 = assign59370_body20_e92662_d_n7;
            locals.var_fsl_dpsl_dn8 = assign59370_body20_e92662_d_n8;
            locals.var_fsl_dpsl_dn9 = assign59370_body20_e92662_d_n9;
            locals.var_fsl_dpsl_dn10 = assign59370_body20_e92662_d_n10;
            locals.var_fsl_dpsl_dn11 = assign59370_body20_e92662_d_n11;
            locals.var_fsl_dpsl_dn14 = assign59370_body20_e92662_d_n14;
            let assign59370_body21_e92665: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1456 = assign59370_body21_e92665;
            let (assign59370_body22_e92674,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1456 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59370_body22_e92674;
            let assign59370_body23_e92677: f64 = if locals.var_flg_brk2 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1457 = assign59370_body23_e92677;
            let (assign59370_body24_e92689, assign59370_body24_e92689_d_n0, assign59370_body24_e92689_d_n2, assign59370_body24_e92689_d_n4, assign59370_body24_e92689_d_n5, assign59370_body24_e92689_d_n6, assign59370_body24_e92689_d_n7, assign59370_body24_e92689_d_n8, assign59370_body24_e92689_d_n9, assign59370_body24_e92689_d_n10, assign59370_body24_e92689_d_n11, assign59370_body24_e92689_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59370_body24_e92685: f64 = (-locals.var_fsl);
        let assign59370_body24_e92687: f64 = (assign59370_body24_e92685 / locals.var_fsl_dpsl);
        (assign59370_body24_e92687, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn4) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn4)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn5) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn5)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn8) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn8)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn9) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn9)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn14) * locals.var_fsl_dpsl) - (assign59370_body24_e92685 * locals.var_fsl_dpsl_dn14)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn14,)
    }
};
            locals.var_dpsl = assign59370_body24_e92689;
            locals.var_dpsl_dn0 = assign59370_body24_e92689_d_n0;
            locals.var_dpsl_dn2 = assign59370_body24_e92689_d_n2;
            locals.var_dpsl_dn4 = assign59370_body24_e92689_d_n4;
            locals.var_dpsl_dn5 = assign59370_body24_e92689_d_n5;
            locals.var_dpsl_dn6 = assign59370_body24_e92689_d_n6;
            locals.var_dpsl_dn7 = assign59370_body24_e92689_d_n7;
            locals.var_dpsl_dn8 = assign59370_body24_e92689_d_n8;
            locals.var_dpsl_dn9 = assign59370_body24_e92689_d_n9;
            locals.var_dpsl_dn10 = assign59370_body24_e92689_d_n10;
            locals.var_dpsl_dn11 = assign59370_body24_e92689_d_n11;
            locals.var_dpsl_dn14 = assign59370_body24_e92689_d_n14;
            let (assign59370_body25_e92711, assign59370_body25_e92711_d_n0, assign59370_body25_e92711_d_n2, assign59370_body25_e92711_d_n4, assign59370_body25_e92711_d_n5, assign59370_body25_e92711_d_n6, assign59370_body25_e92711_d_n7, assign59370_body25_e92711_d_n8, assign59370_body25_e92711_d_n9, assign59370_body25_e92711_d_n10, assign59370_body25_e92711_d_n11, assign59370_body25_e92711_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59370_body25_e92698: f64 = (0.5 * 0.1);
        let assign59370_body25_e92702: f64 = (locals.var_psl).abs();
        let (assign59370_body25_e92707, assign59370_body25_e92707_d_n0, assign59370_body25_e92707_d_n2, assign59370_body25_e92707_d_n4, assign59370_body25_e92707_d_n5, assign59370_body25_e92707_d_n6, assign59370_body25_e92707_d_n7, assign59370_body25_e92707_d_n8, assign59370_body25_e92707_d_n9, assign59370_body25_e92707_d_n10, assign59370_body25_e92707_d_n11, assign59370_body25_e92707_d_n14,) = {
            if (1.0 >= assign59370_body25_e92702) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59370_body25_e92706: f64 = (locals.var_psl).abs();
                (assign59370_body25_e92706, if locals.var_psl >= 0.0 { locals.var_psl_dn0 } else { (-locals.var_psl_dn0) }, if locals.var_psl >= 0.0 { locals.var_psl_dn2 } else { (-locals.var_psl_dn2) }, if locals.var_psl >= 0.0 { locals.var_psl_dn4 } else { (-locals.var_psl_dn4) }, if locals.var_psl >= 0.0 { locals.var_psl_dn5 } else { (-locals.var_psl_dn5) }, if locals.var_psl >= 0.0 { locals.var_psl_dn6 } else { (-locals.var_psl_dn6) }, if locals.var_psl >= 0.0 { locals.var_psl_dn7 } else { (-locals.var_psl_dn7) }, if locals.var_psl >= 0.0 { locals.var_psl_dn8 } else { (-locals.var_psl_dn8) }, if locals.var_psl >= 0.0 { locals.var_psl_dn9 } else { (-locals.var_psl_dn9) }, if locals.var_psl >= 0.0 { locals.var_psl_dn10 } else { (-locals.var_psl_dn10) }, if locals.var_psl >= 0.0 { locals.var_psl_dn11 } else { (-locals.var_psl_dn11) }, if locals.var_psl >= 0.0 { locals.var_psl_dn14 } else { (-locals.var_psl_dn14) },)
            }
        };
        let assign59370_body25_e92708: f64 = (1.0 + assign59370_body25_e92707);
        let assign59370_body25_e92709: f64 = (assign59370_body25_e92698 * assign59370_body25_e92708);
        (assign59370_body25_e92709, (assign59370_body25_e92698 * assign59370_body25_e92707_d_n0), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n2), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n4), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n5), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n6), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n7), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n8), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n9), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n10), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n11), (assign59370_body25_e92698 * assign59370_body25_e92707_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign59370_body25_e92711;
            locals.var_dplim_dn0 = assign59370_body25_e92711_d_n0;
            locals.var_dplim_dn2 = assign59370_body25_e92711_d_n2;
            locals.var_dplim_dn4 = assign59370_body25_e92711_d_n4;
            locals.var_dplim_dn5 = assign59370_body25_e92711_d_n5;
            locals.var_dplim_dn6 = assign59370_body25_e92711_d_n6;
            locals.var_dplim_dn7 = assign59370_body25_e92711_d_n7;
            locals.var_dplim_dn8 = assign59370_body25_e92711_d_n8;
            locals.var_dplim_dn9 = assign59370_body25_e92711_d_n9;
            locals.var_dplim_dn10 = assign59370_body25_e92711_d_n10;
            locals.var_dplim_dn11 = assign59370_body25_e92711_d_n11;
            locals.var_dplim_dn14 = assign59370_body25_e92711_d_n14;
            let assign59370_body26_e92713: f64 = (locals.var_dpsl).abs();
            let assign59370_body26_e92715: f64 = if assign59370_body26_e92713 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1458 = assign59370_body26_e92715;
            let (assign59370_body27_e92734, assign59370_body27_e92734_d_n0, assign59370_body27_e92734_d_n2, assign59370_body27_e92734_d_n4, assign59370_body27_e92734_d_n5, assign59370_body27_e92734_d_n6, assign59370_body27_e92734_d_n7, assign59370_body27_e92734_d_n8, assign59370_body27_e92734_d_n9, assign59370_body27_e92734_d_n10, assign59370_body27_e92734_d_n11, assign59370_body27_e92734_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        let (assign59370_body27_e92731,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign59370_body27_e92730: f64 = (-1.0);
                (assign59370_body27_e92730,)
            }
        };
        let assign59370_body27_e92732: f64 = (locals.var_dplim * assign59370_body27_e92731);
        (assign59370_body27_e92732, (locals.var_dplim_dn0 * assign59370_body27_e92731), (locals.var_dplim_dn2 * assign59370_body27_e92731), (locals.var_dplim_dn4 * assign59370_body27_e92731), (locals.var_dplim_dn5 * assign59370_body27_e92731), (locals.var_dplim_dn6 * assign59370_body27_e92731), (locals.var_dplim_dn7 * assign59370_body27_e92731), (locals.var_dplim_dn8 * assign59370_body27_e92731), (locals.var_dplim_dn9 * assign59370_body27_e92731), (locals.var_dplim_dn10 * assign59370_body27_e92731), (locals.var_dplim_dn11 * assign59370_body27_e92731), (locals.var_dplim_dn14 * assign59370_body27_e92731),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn14,)
    }
};
            locals.var_dpsl = assign59370_body27_e92734;
            locals.var_dpsl_dn0 = assign59370_body27_e92734_d_n0;
            locals.var_dpsl_dn2 = assign59370_body27_e92734_d_n2;
            locals.var_dpsl_dn4 = assign59370_body27_e92734_d_n4;
            locals.var_dpsl_dn5 = assign59370_body27_e92734_d_n5;
            locals.var_dpsl_dn6 = assign59370_body27_e92734_d_n6;
            locals.var_dpsl_dn7 = assign59370_body27_e92734_d_n7;
            locals.var_dpsl_dn8 = assign59370_body27_e92734_d_n8;
            locals.var_dpsl_dn9 = assign59370_body27_e92734_d_n9;
            locals.var_dpsl_dn10 = assign59370_body27_e92734_d_n10;
            locals.var_dpsl_dn11 = assign59370_body27_e92734_d_n11;
            locals.var_dpsl_dn14 = assign59370_body27_e92734_d_n14;
            let (assign59370_body28_e92745, assign59370_body28_e92745_d_n0, assign59370_body28_e92745_d_n2, assign59370_body28_e92745_d_n4, assign59370_body28_e92745_d_n5, assign59370_body28_e92745_d_n6, assign59370_body28_e92745_d_n7, assign59370_body28_e92745_d_n8, assign59370_body28_e92745_d_n9, assign59370_body28_e92745_d_n10, assign59370_body28_e92745_d_n11, assign59370_body28_e92745_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59370_body28_e92743: f64 = (locals.var_psl + locals.var_dpsl);
        (assign59370_body28_e92743, (locals.var_psl_dn0 + locals.var_dpsl_dn0), (locals.var_psl_dn2 + locals.var_dpsl_dn2), (locals.var_psl_dn4 + locals.var_dpsl_dn4), (locals.var_psl_dn5 + locals.var_dpsl_dn5), (locals.var_psl_dn6 + locals.var_dpsl_dn6), (locals.var_psl_dn7 + locals.var_dpsl_dn7), (locals.var_psl_dn8 + locals.var_dpsl_dn8), (locals.var_psl_dn9 + locals.var_dpsl_dn9), (locals.var_psl_dn10 + locals.var_dpsl_dn10), (locals.var_psl_dn11 + locals.var_dpsl_dn11), (locals.var_psl_dn14 + locals.var_dpsl_dn14),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
            locals.var_psl = assign59370_body28_e92745;
            locals.var_psl_dn0 = assign59370_body28_e92745_d_n0;
            locals.var_psl_dn2 = assign59370_body28_e92745_d_n2;
            locals.var_psl_dn4 = assign59370_body28_e92745_d_n4;
            locals.var_psl_dn5 = assign59370_body28_e92745_d_n5;
            locals.var_psl_dn6 = assign59370_body28_e92745_d_n6;
            locals.var_psl_dn7 = assign59370_body28_e92745_d_n7;
            locals.var_psl_dn8 = assign59370_body28_e92745_d_n8;
            locals.var_psl_dn9 = assign59370_body28_e92745_d_n9;
            locals.var_psl_dn10 = assign59370_body28_e92745_d_n10;
            locals.var_psl_dn11 = assign59370_body28_e92745_d_n11;
            locals.var_psl_dn14 = assign59370_body28_e92745_d_n14;
            let assign59370_body29_e92747: f64 = (locals.var_dpsl).abs();
            let assign59370_body29_e92751: f64 = (locals.var_fsl).abs();
            let assign59370_body29_e92754: f64 = if ((assign59370_body29_e92747 <= 1e-12) && (assign59370_body29_e92751 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1459 = assign59370_body29_e92754;
            let (assign59370_body30_e92765,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign59370_body30_e92765;
            let (assign59370_body31_e92776,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_flg_brk2 != 0.0)) {
        let assign59370_body31_e92774: f64 = (40.0 + 1.0);
        (assign59370_body31_e92774,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59370_body31_e92776;
            let (assign59370_body32_e92783,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59370_body32_e92783;
            let (assign59370_body33_e92792,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59370_body33_e92790: f64 = (locals.var_lp_sl + 1.0);
        (assign59370_body33_e92790,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59370_body33_e92792;
        }

    }

    pub(super) fn stamp_transient_block_209(
        locals: &mut StampLocals,
    ) {
        let (assign59380_e92801,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59380_e92799: f64 = (locals.var_lp_sl - 1.0);
        (assign59380_e92799,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign59380_e92801;

        let assign59400_e92807: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign59400_e92807;

        let (assign59410_e92822, assign59410_e92822_d_n0, assign59410_e92822_d_n2, assign59410_e92822_d_n4, assign59410_e92822_d_n5, assign59410_e92822_d_n6, assign59410_e92822_d_n7, assign59410_e92822_d_n8, assign59410_e92822_d_n9, assign59410_e92822_d_n10, assign59410_e92822_d_n11, assign59410_e92822_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 != 0.0)) {
        let assign59410_e92816: f64 = (locals.var_fb * locals.var_fb);
        let assign59410_e92819: f64 = (10.0 * 2.220446049250313e-16);
        let assign59410_e92820: f64 = (assign59410_e92816 + assign59410_e92819);
        (assign59410_e92820, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
        locals.var_xil = assign59410_e92822;
        locals.var_xil_dn0 = assign59410_e92822_d_n0;
        locals.var_xil_dn2 = assign59410_e92822_d_n2;
        locals.var_xil_dn4 = assign59410_e92822_d_n4;
        locals.var_xil_dn5 = assign59410_e92822_d_n5;
        locals.var_xil_dn6 = assign59410_e92822_d_n6;
        locals.var_xil_dn7 = assign59410_e92822_d_n7;
        locals.var_xil_dn8 = assign59410_e92822_d_n8;
        locals.var_xil_dn9 = assign59410_e92822_d_n9;
        locals.var_xil_dn10 = assign59410_e92822_d_n10;
        locals.var_xil_dn11 = assign59410_e92822_d_n11;
        locals.var_xil_dn14 = assign59410_e92822_d_n14;

        let (assign59420_e92835, assign59420_e92835_d_n0, assign59420_e92835_d_n2, assign59420_e92835_d_n4, assign59420_e92835_d_n5, assign59420_e92835_d_n6, assign59420_e92835_d_n7, assign59420_e92835_d_n8, assign59420_e92835_d_n9, assign59420_e92835_d_n10, assign59420_e92835_d_n11, assign59420_e92835_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 != 0.0)) {
        let assign59420_e92832: f64 = (10.0 * 2.220446049250313e-16);
        let assign59420_e92833: f64 = (locals.var_fb + assign59420_e92832);
        (assign59420_e92833, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn14,)
    }
};
        locals.var_xilp12 = assign59420_e92835;
        locals.var_xilp12_dn0 = assign59420_e92835_d_n0;
        locals.var_xilp12_dn2 = assign59420_e92835_d_n2;
        locals.var_xilp12_dn4 = assign59420_e92835_d_n4;
        locals.var_xilp12_dn5 = assign59420_e92835_d_n5;
        locals.var_xilp12_dn6 = assign59420_e92835_d_n6;
        locals.var_xilp12_dn7 = assign59420_e92835_d_n7;
        locals.var_xilp12_dn8 = assign59420_e92835_d_n8;
        locals.var_xilp12_dn9 = assign59420_e92835_d_n9;
        locals.var_xilp12_dn10 = assign59420_e92835_d_n10;
        locals.var_xilp12_dn11 = assign59420_e92835_d_n11;
        locals.var_xilp12_dn14 = assign59420_e92835_d_n14;

        let (assign59430_e92852, assign59430_e92852_d_n0, assign59430_e92852_d_n2, assign59430_e92852_d_n4, assign59430_e92852_d_n5, assign59430_e92852_d_n6, assign59430_e92852_d_n7, assign59430_e92852_d_n8, assign59430_e92852_d_n9, assign59430_e92852_d_n10, assign59430_e92852_d_n11, assign59430_e92852_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 != 0.0)) {
        let assign59430_e92844: f64 = (locals.var_fb * locals.var_fb);
        let assign59430_e92846: f64 = (assign59430_e92844 * locals.var_fb);
        let assign59430_e92849: f64 = (10.0 * 2.220446049250313e-16);
        let assign59430_e92850: f64 = (assign59430_e92846 + assign59430_e92849);
        (assign59430_e92850, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn0)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn2)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn4)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn5)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn6)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn7)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn8)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn9)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn10)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn11)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) * locals.var_fb) + (assign59430_e92844 * locals.var_fb_dn14)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn14,)
    }
};
        locals.var_xilp32 = assign59430_e92852;
        locals.var_xilp32_dn0 = assign59430_e92852_d_n0;
        locals.var_xilp32_dn2 = assign59430_e92852_d_n2;
        locals.var_xilp32_dn4 = assign59430_e92852_d_n4;
        locals.var_xilp32_dn5 = assign59430_e92852_d_n5;
        locals.var_xilp32_dn6 = assign59430_e92852_d_n6;
        locals.var_xilp32_dn7 = assign59430_e92852_d_n7;
        locals.var_xilp32_dn8 = assign59430_e92852_d_n8;
        locals.var_xilp32_dn9 = assign59430_e92852_d_n9;
        locals.var_xilp32_dn10 = assign59430_e92852_d_n10;
        locals.var_xilp32_dn11 = assign59430_e92852_d_n11;
        locals.var_xilp32_dn14 = assign59430_e92852_d_n14;

        let (assign59440_e92864, assign59440_e92864_d_n0, assign59440_e92864_d_n2, assign59440_e92864_d_n4, assign59440_e92864_d_n5, assign59440_e92864_d_n6, assign59440_e92864_d_n7, assign59440_e92864_d_n8, assign59440_e92864_d_n9, assign59440_e92864_d_n10, assign59440_e92864_d_n11, assign59440_e92864_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 == 0.0)) {
        let assign59440_e92862: f64 = (locals.var_chi - 1.0);
        (assign59440_e92862, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
        locals.var_xil = assign59440_e92864;
        locals.var_xil_dn0 = assign59440_e92864_d_n0;
        locals.var_xil_dn2 = assign59440_e92864_d_n2;
        locals.var_xil_dn4 = assign59440_e92864_d_n4;
        locals.var_xil_dn5 = assign59440_e92864_d_n5;
        locals.var_xil_dn6 = assign59440_e92864_d_n6;
        locals.var_xil_dn7 = assign59440_e92864_d_n7;
        locals.var_xil_dn8 = assign59440_e92864_d_n8;
        locals.var_xil_dn9 = assign59440_e92864_d_n9;
        locals.var_xil_dn10 = assign59440_e92864_d_n10;
        locals.var_xil_dn11 = assign59440_e92864_d_n11;
        locals.var_xil_dn14 = assign59440_e92864_d_n14;

        let (assign59450_e92875, assign59450_e92875_d_n0, assign59450_e92875_d_n2, assign59450_e92875_d_n4, assign59450_e92875_d_n5, assign59450_e92875_d_n6, assign59450_e92875_d_n7, assign59450_e92875_d_n8, assign59450_e92875_d_n9, assign59450_e92875_d_n10, assign59450_e92875_d_n11, assign59450_e92875_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 == 0.0)) {
        let assign59450_e92873: f64 = (locals.var_xil).sqrt();
        (assign59450_e92873, (locals.var_xil_dn0 / (2.0 * assign59450_e92873)), (locals.var_xil_dn2 / (2.0 * assign59450_e92873)), (locals.var_xil_dn4 / (2.0 * assign59450_e92873)), (locals.var_xil_dn5 / (2.0 * assign59450_e92873)), (locals.var_xil_dn6 / (2.0 * assign59450_e92873)), (locals.var_xil_dn7 / (2.0 * assign59450_e92873)), (locals.var_xil_dn8 / (2.0 * assign59450_e92873)), (locals.var_xil_dn9 / (2.0 * assign59450_e92873)), (locals.var_xil_dn10 / (2.0 * assign59450_e92873)), (locals.var_xil_dn11 / (2.0 * assign59450_e92873)), (locals.var_xil_dn14 / (2.0 * assign59450_e92873)),)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn14,)
    }
};
        locals.var_xilp12 = assign59450_e92875;
        locals.var_xilp12_dn0 = assign59450_e92875_d_n0;
        locals.var_xilp12_dn2 = assign59450_e92875_d_n2;
        locals.var_xilp12_dn4 = assign59450_e92875_d_n4;
        locals.var_xilp12_dn5 = assign59450_e92875_d_n5;
        locals.var_xilp12_dn6 = assign59450_e92875_d_n6;
        locals.var_xilp12_dn7 = assign59450_e92875_d_n7;
        locals.var_xilp12_dn8 = assign59450_e92875_d_n8;
        locals.var_xilp12_dn9 = assign59450_e92875_d_n9;
        locals.var_xilp12_dn10 = assign59450_e92875_d_n10;
        locals.var_xilp12_dn11 = assign59450_e92875_d_n11;
        locals.var_xilp12_dn14 = assign59450_e92875_d_n14;

        let (assign59460_e92887, assign59460_e92887_d_n0, assign59460_e92887_d_n2, assign59460_e92887_d_n4, assign59460_e92887_d_n5, assign59460_e92887_d_n6, assign59460_e92887_d_n7, assign59460_e92887_d_n8, assign59460_e92887_d_n9, assign59460_e92887_d_n10, assign59460_e92887_d_n11, assign59460_e92887_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1461 == 0.0)) {
        let assign59460_e92885: f64 = (locals.var_xil * locals.var_xilp12);
        (assign59460_e92885, ((locals.var_xil_dn0 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn2)), ((locals.var_xil_dn4 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn4)), ((locals.var_xil_dn5 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn5)), ((locals.var_xil_dn6 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn6)), ((locals.var_xil_dn7 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn7)), ((locals.var_xil_dn8 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn8)), ((locals.var_xil_dn9 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn9)), ((locals.var_xil_dn10 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn10)), ((locals.var_xil_dn11 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn11)), ((locals.var_xil_dn14 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn14)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn14,)
    }
};
        locals.var_xilp32 = assign59460_e92887;
        locals.var_xilp32_dn0 = assign59460_e92887_d_n0;
        locals.var_xilp32_dn2 = assign59460_e92887_d_n2;
        locals.var_xilp32_dn4 = assign59460_e92887_d_n4;
        locals.var_xilp32_dn5 = assign59460_e92887_d_n5;
        locals.var_xilp32_dn6 = assign59460_e92887_d_n6;
        locals.var_xilp32_dn7 = assign59460_e92887_d_n7;
        locals.var_xilp32_dn8 = assign59460_e92887_d_n8;
        locals.var_xilp32_dn9 = assign59460_e92887_d_n9;
        locals.var_xilp32_dn10 = assign59460_e92887_d_n10;
        locals.var_xilp32_dn11 = assign59460_e92887_d_n11;
        locals.var_xilp32_dn14 = assign59460_e92887_d_n14;

        let (assign59470_e92896, assign59470_e92896_d_n0, assign59470_e92896_d_n2, assign59470_e92896_d_n4, assign59470_e92896_d_n5, assign59470_e92896_d_n6, assign59470_e92896_d_n7, assign59470_e92896_d_n8, assign59470_e92896_d_n9, assign59470_e92896_d_n10, assign59470_e92896_d_n11, assign59470_e92896_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59470_e92894: f64 = (locals.var_psl - locals.var_ps0);
        (assign59470_e92894, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn4 - locals.var_ps0_dn4), (locals.var_psl_dn5 - locals.var_ps0_dn5), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn8 - locals.var_ps0_dn8), (locals.var_psl_dn9 - locals.var_ps0_dn9), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn14 - locals.var_ps0_dn14),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59470_e92896;
        locals.var_pds_dn0 = assign59470_e92896_d_n0;
        locals.var_pds_dn2 = assign59470_e92896_d_n2;
        locals.var_pds_dn4 = assign59470_e92896_d_n4;
        locals.var_pds_dn5 = assign59470_e92896_d_n5;
        locals.var_pds_dn6 = assign59470_e92896_d_n6;
        locals.var_pds_dn7 = assign59470_e92896_d_n7;
        locals.var_pds_dn8 = assign59470_e92896_d_n8;
        locals.var_pds_dn9 = assign59470_e92896_d_n9;
        locals.var_pds_dn10 = assign59470_e92896_d_n10;
        locals.var_pds_dn11 = assign59470_e92896_d_n11;
        locals.var_pds_dn14 = assign59470_e92896_d_n14;

        let (assign59480_e92903, assign59480_e92903_d_n0, assign59480_e92903_d_n2, assign59480_e92903_d_n4, assign59480_e92903_d_n5, assign59480_e92903_d_n6, assign59480_e92903_d_n7, assign59480_e92903_d_n8, assign59480_e92903_d_n9, assign59480_e92903_d_n10, assign59480_e92903_d_n11, assign59480_e92903_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign59480_e92903;
        locals.var_vds_dn0 = assign59480_e92903_d_n0;
        locals.var_vds_dn2 = assign59480_e92903_d_n2;
        locals.var_vds_dn4 = assign59480_e92903_d_n4;
        locals.var_vds_dn5 = assign59480_e92903_d_n5;
        locals.var_vds_dn6 = assign59480_e92903_d_n6;
        locals.var_vds_dn7 = assign59480_e92903_d_n7;
        locals.var_vds_dn8 = assign59480_e92903_d_n8;
        locals.var_vds_dn9 = assign59480_e92903_d_n9;
        locals.var_vds_dn10 = assign59480_e92903_d_n10;
        locals.var_vds_dn11 = assign59480_e92903_d_n11;
        locals.var_vds_dn14 = assign59480_e92903_d_n14;

        let (assign59490_e92912, assign59490_e92912_d_n0, assign59490_e92912_d_n2, assign59490_e92912_d_n4, assign59490_e92912_d_n5, assign59490_e92912_d_n6, assign59490_e92912_d_n7, assign59490_e92912_d_n8, assign59490_e92912_d_n9, assign59490_e92912_d_n10, assign59490_e92912_d_n11, assign59490_e92912_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59490_e92910: f64 = (locals.var_beta / locals.var_xi0);
        (assign59490_e92910, (((locals.var_beta_dn0 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn0)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn2 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn2)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn4 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn4)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn5 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn5)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn6 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn6)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn7 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn7)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn8 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn8)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn9 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn9)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn11 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn11)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn14 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn14)) / (locals.var_xi0 * locals.var_xi0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59490_e92912;
        locals.var_t1_dn0 = assign59490_e92912_d_n0;
        locals.var_t1_dn2 = assign59490_e92912_d_n2;
        locals.var_t1_dn4 = assign59490_e92912_d_n4;
        locals.var_t1_dn5 = assign59490_e92912_d_n5;
        locals.var_t1_dn6 = assign59490_e92912_d_n6;
        locals.var_t1_dn7 = assign59490_e92912_d_n7;
        locals.var_t1_dn8 = assign59490_e92912_d_n8;
        locals.var_t1_dn9 = assign59490_e92912_d_n9;
        locals.var_t1_dn10 = assign59490_e92912_d_n10;
        locals.var_t1_dn11 = assign59490_e92912_d_n11;
        locals.var_t1_dn14 = assign59490_e92912_d_n14;

        let (assign59500_e92921, assign59500_e92921_d_n0, assign59500_e92921_d_n2, assign59500_e92921_d_n4, assign59500_e92921_d_n5, assign59500_e92921_d_n6, assign59500_e92921_d_n7, assign59500_e92921_d_n8, assign59500_e92921_d_n9, assign59500_e92921_d_n10, assign59500_e92921_d_n11, assign59500_e92921_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59500_e92919: f64 = (locals.var_t1 * locals.var_pds);
        (assign59500_e92919, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn4 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn4)), ((locals.var_t1_dn5 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn5)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn8 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn8)), ((locals.var_t1_dn9 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn9)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn14 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn14)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn14,)
    }
};
        locals.var_eta = assign59500_e92921;
        locals.var_eta_dn0 = assign59500_e92921_d_n0;
        locals.var_eta_dn2 = assign59500_e92921_d_n2;
        locals.var_eta_dn4 = assign59500_e92921_d_n4;
        locals.var_eta_dn5 = assign59500_e92921_d_n5;
        locals.var_eta_dn6 = assign59500_e92921_d_n6;
        locals.var_eta_dn7 = assign59500_e92921_d_n7;
        locals.var_eta_dn8 = assign59500_e92921_d_n8;
        locals.var_eta_dn9 = assign59500_e92921_d_n9;
        locals.var_eta_dn10 = assign59500_e92921_d_n10;
        locals.var_eta_dn11 = assign59500_e92921_d_n11;
        locals.var_eta_dn14 = assign59500_e92921_d_n14;

        let (assign59510_e92930, assign59510_e92930_d_n0, assign59510_e92930_d_n2, assign59510_e92930_d_n4, assign59510_e92930_d_n5, assign59510_e92930_d_n6, assign59510_e92930_d_n7, assign59510_e92930_d_n8, assign59510_e92930_d_n9, assign59510_e92930_d_n10, assign59510_e92930_d_n11, assign59510_e92930_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59510_e92928: f64 = (locals.var_eta + 1.0);
        (assign59510_e92928, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn14,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn4, locals.var_eta1_dn5, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn8, locals.var_eta1_dn9, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn14,)
    }
};
        locals.var_eta1 = assign59510_e92930;
        locals.var_eta1_dn0 = assign59510_e92930_d_n0;
        locals.var_eta1_dn2 = assign59510_e92930_d_n2;
        locals.var_eta1_dn4 = assign59510_e92930_d_n4;
        locals.var_eta1_dn5 = assign59510_e92930_d_n5;
        locals.var_eta1_dn6 = assign59510_e92930_d_n6;
        locals.var_eta1_dn7 = assign59510_e92930_d_n7;
        locals.var_eta1_dn8 = assign59510_e92930_d_n8;
        locals.var_eta1_dn9 = assign59510_e92930_d_n9;
        locals.var_eta1_dn10 = assign59510_e92930_d_n10;
        locals.var_eta1_dn11 = assign59510_e92930_d_n11;
        locals.var_eta1_dn14 = assign59510_e92930_d_n14;

        let (assign59520_e92938, assign59520_e92938_d_n0, assign59520_e92938_d_n2, assign59520_e92938_d_n4, assign59520_e92938_d_n5, assign59520_e92938_d_n6, assign59520_e92938_d_n7, assign59520_e92938_d_n8, assign59520_e92938_d_n9, assign59520_e92938_d_n10, assign59520_e92938_d_n11, assign59520_e92938_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59520_e92936: f64 = (locals.var_eta1).sqrt();
        (assign59520_e92936, (locals.var_eta1_dn0 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn2 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn4 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn5 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn6 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn7 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn8 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn9 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn10 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn11 / (2.0 * assign59520_e92936)), (locals.var_eta1_dn14 / (2.0 * assign59520_e92936)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn4, locals.var_eta1p12_dn5, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn8, locals.var_eta1p12_dn9, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn14,)
    }
};
        locals.var_eta1p12 = assign59520_e92938;
        locals.var_eta1p12_dn0 = assign59520_e92938_d_n0;
        locals.var_eta1p12_dn2 = assign59520_e92938_d_n2;
        locals.var_eta1p12_dn4 = assign59520_e92938_d_n4;
        locals.var_eta1p12_dn5 = assign59520_e92938_d_n5;
        locals.var_eta1p12_dn6 = assign59520_e92938_d_n6;
        locals.var_eta1p12_dn7 = assign59520_e92938_d_n7;
        locals.var_eta1p12_dn8 = assign59520_e92938_d_n8;
        locals.var_eta1p12_dn9 = assign59520_e92938_d_n9;
        locals.var_eta1p12_dn10 = assign59520_e92938_d_n10;
        locals.var_eta1p12_dn11 = assign59520_e92938_d_n11;
        locals.var_eta1p12_dn14 = assign59520_e92938_d_n14;

        let (assign59530_e92947, assign59530_e92947_d_n0, assign59530_e92947_d_n2, assign59530_e92947_d_n4, assign59530_e92947_d_n5, assign59530_e92947_d_n6, assign59530_e92947_d_n7, assign59530_e92947_d_n8, assign59530_e92947_d_n9, assign59530_e92947_d_n10, assign59530_e92947_d_n11, assign59530_e92947_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59530_e92945: f64 = (locals.var_eta1p12 * locals.var_eta1);
        (assign59530_e92945, ((locals.var_eta1p12_dn0 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn0)), ((locals.var_eta1p12_dn2 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn2)), ((locals.var_eta1p12_dn4 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn4)), ((locals.var_eta1p12_dn5 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn5)), ((locals.var_eta1p12_dn6 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn6)), ((locals.var_eta1p12_dn7 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn7)), ((locals.var_eta1p12_dn8 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn8)), ((locals.var_eta1p12_dn9 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn9)), ((locals.var_eta1p12_dn10 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn10)), ((locals.var_eta1p12_dn11 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn11)), ((locals.var_eta1p12_dn14 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn14)),)
    } else {
        (locals.var_eta1p32, locals.var_eta1p32_dn0, locals.var_eta1p32_dn2, locals.var_eta1p32_dn4, locals.var_eta1p32_dn5, locals.var_eta1p32_dn6, locals.var_eta1p32_dn7, locals.var_eta1p32_dn8, locals.var_eta1p32_dn9, locals.var_eta1p32_dn10, locals.var_eta1p32_dn11, locals.var_eta1p32_dn14,)
    }
};
        locals.var_eta1p32 = assign59530_e92947;
        locals.var_eta1p32_dn0 = assign59530_e92947_d_n0;
        locals.var_eta1p32_dn2 = assign59530_e92947_d_n2;
        locals.var_eta1p32_dn4 = assign59530_e92947_d_n4;
        locals.var_eta1p32_dn5 = assign59530_e92947_d_n5;
        locals.var_eta1p32_dn6 = assign59530_e92947_d_n6;
        locals.var_eta1p32_dn7 = assign59530_e92947_d_n7;
        locals.var_eta1p32_dn8 = assign59530_e92947_d_n8;
        locals.var_eta1p32_dn9 = assign59530_e92947_d_n9;
        locals.var_eta1p32_dn10 = assign59530_e92947_d_n10;
        locals.var_eta1p32_dn11 = assign59530_e92947_d_n11;
        locals.var_eta1p32_dn14 = assign59530_e92947_d_n14;

        let (assign59540_e92956, assign59540_e92956_d_n0, assign59540_e92956_d_n2, assign59540_e92956_d_n4, assign59540_e92956_d_n5, assign59540_e92956_d_n6, assign59540_e92956_d_n7, assign59540_e92956_d_n8, assign59540_e92956_d_n9, assign59540_e92956_d_n10, assign59540_e92956_d_n11, assign59540_e92956_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59540_e92954: f64 = (locals.var_eta1p32 * locals.var_eta1);
        (assign59540_e92954, ((locals.var_eta1p32_dn0 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn0)), ((locals.var_eta1p32_dn2 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn2)), ((locals.var_eta1p32_dn4 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn4)), ((locals.var_eta1p32_dn5 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn5)), ((locals.var_eta1p32_dn6 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn6)), ((locals.var_eta1p32_dn7 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn7)), ((locals.var_eta1p32_dn8 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn8)), ((locals.var_eta1p32_dn9 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn9)), ((locals.var_eta1p32_dn10 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn10)), ((locals.var_eta1p32_dn11 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn11)), ((locals.var_eta1p32_dn14 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn14)),)
    } else {
        (locals.var_eta1p52, locals.var_eta1p52_dn0, locals.var_eta1p52_dn2, locals.var_eta1p52_dn4, locals.var_eta1p52_dn5, locals.var_eta1p52_dn6, locals.var_eta1p52_dn7, locals.var_eta1p52_dn8, locals.var_eta1p52_dn9, locals.var_eta1p52_dn10, locals.var_eta1p52_dn11, locals.var_eta1p52_dn14,)
    }
};
        locals.var_eta1p52 = assign59540_e92956;
        locals.var_eta1p52_dn0 = assign59540_e92956_d_n0;
        locals.var_eta1p52_dn2 = assign59540_e92956_d_n2;
        locals.var_eta1p52_dn4 = assign59540_e92956_d_n4;
        locals.var_eta1p52_dn5 = assign59540_e92956_d_n5;
        locals.var_eta1p52_dn6 = assign59540_e92956_d_n6;
        locals.var_eta1p52_dn7 = assign59540_e92956_d_n7;
        locals.var_eta1p52_dn8 = assign59540_e92956_d_n8;
        locals.var_eta1p52_dn9 = assign59540_e92956_d_n9;
        locals.var_eta1p52_dn10 = assign59540_e92956_d_n10;
        locals.var_eta1p52_dn11 = assign59540_e92956_d_n11;
        locals.var_eta1p52_dn14 = assign59540_e92956_d_n14;

        let (assign59550_e92967, assign59550_e92967_d_n0, assign59550_e92967_d_n2, assign59550_e92967_d_n4, assign59550_e92967_d_n5, assign59550_e92967_d_n6, assign59550_e92967_d_n7, assign59550_e92967_d_n8, assign59550_e92967_d_n9, assign59550_e92967_d_n10, assign59550_e92967_d_n11, assign59550_e92967_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59550_e92964: f64 = (locals.var_eta1p12 + 1.0);
        let assign59550_e92965: f64 = (1.0 / assign59550_e92964);
        (assign59550_e92965, (-(locals.var_eta1p12_dn0 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn2 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn4 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn5 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn6 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn7 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn8 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn9 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn10 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn11 / (assign59550_e92964 * assign59550_e92964))), (-(locals.var_eta1p12_dn14 / (assign59550_e92964 * assign59550_e92964))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn4, locals.var_zeta12_dn5, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn8, locals.var_zeta12_dn9, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn14,)
    }
};
        locals.var_zeta12 = assign59550_e92967;
        locals.var_zeta12_dn0 = assign59550_e92967_d_n0;
        locals.var_zeta12_dn2 = assign59550_e92967_d_n2;
        locals.var_zeta12_dn4 = assign59550_e92967_d_n4;
        locals.var_zeta12_dn5 = assign59550_e92967_d_n5;
        locals.var_zeta12_dn6 = assign59550_e92967_d_n6;
        locals.var_zeta12_dn7 = assign59550_e92967_d_n7;
        locals.var_zeta12_dn8 = assign59550_e92967_d_n8;
        locals.var_zeta12_dn9 = assign59550_e92967_d_n9;
        locals.var_zeta12_dn10 = assign59550_e92967_d_n10;
        locals.var_zeta12_dn11 = assign59550_e92967_d_n11;
        locals.var_zeta12_dn14 = assign59550_e92967_d_n14;

        let (assign59560_e92978, assign59560_e92978_d_n0, assign59560_e92978_d_n2, assign59560_e92978_d_n4, assign59560_e92978_d_n5, assign59560_e92978_d_n6, assign59560_e92978_d_n7, assign59560_e92978_d_n8, assign59560_e92978_d_n9, assign59560_e92978_d_n10, assign59560_e92978_d_n11, assign59560_e92978_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59560_e92975: f64 = (locals.var_eta1p32 + 1.0);
        let assign59560_e92976: f64 = (1.0 / assign59560_e92975);
        (assign59560_e92976, (-(locals.var_eta1p32_dn0 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn2 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn4 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn5 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn6 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn7 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn8 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn9 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn10 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn11 / (assign59560_e92975 * assign59560_e92975))), (-(locals.var_eta1p32_dn14 / (assign59560_e92975 * assign59560_e92975))),)
    } else {
        (locals.var_zeta32, locals.var_zeta32_dn0, locals.var_zeta32_dn2, locals.var_zeta32_dn4, locals.var_zeta32_dn5, locals.var_zeta32_dn6, locals.var_zeta32_dn7, locals.var_zeta32_dn8, locals.var_zeta32_dn9, locals.var_zeta32_dn10, locals.var_zeta32_dn11, locals.var_zeta32_dn14,)
    }
};
        locals.var_zeta32 = assign59560_e92978;
        locals.var_zeta32_dn0 = assign59560_e92978_d_n0;
        locals.var_zeta32_dn2 = assign59560_e92978_d_n2;
        locals.var_zeta32_dn4 = assign59560_e92978_d_n4;
        locals.var_zeta32_dn5 = assign59560_e92978_d_n5;
        locals.var_zeta32_dn6 = assign59560_e92978_d_n6;
        locals.var_zeta32_dn7 = assign59560_e92978_d_n7;
        locals.var_zeta32_dn8 = assign59560_e92978_d_n8;
        locals.var_zeta32_dn9 = assign59560_e92978_d_n9;
        locals.var_zeta32_dn10 = assign59560_e92978_d_n10;
        locals.var_zeta32_dn11 = assign59560_e92978_d_n11;
        locals.var_zeta32_dn14 = assign59560_e92978_d_n14;

        let (assign59570_e92989, assign59570_e92989_d_n0, assign59570_e92989_d_n2, assign59570_e92989_d_n4, assign59570_e92989_d_n5, assign59570_e92989_d_n6, assign59570_e92989_d_n7, assign59570_e92989_d_n8, assign59570_e92989_d_n9, assign59570_e92989_d_n10, assign59570_e92989_d_n11, assign59570_e92989_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59570_e92986: f64 = (locals.var_eta1p52 + 1.0);
        let assign59570_e92987: f64 = (1.0 / assign59570_e92986);
        (assign59570_e92987, (-(locals.var_eta1p52_dn0 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn2 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn4 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn5 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn6 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn7 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn8 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn9 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn10 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn11 / (assign59570_e92986 * assign59570_e92986))), (-(locals.var_eta1p52_dn14 / (assign59570_e92986 * assign59570_e92986))),)
    } else {
        (locals.var_zeta52, locals.var_zeta52_dn0, locals.var_zeta52_dn2, locals.var_zeta52_dn4, locals.var_zeta52_dn5, locals.var_zeta52_dn6, locals.var_zeta52_dn7, locals.var_zeta52_dn8, locals.var_zeta52_dn9, locals.var_zeta52_dn10, locals.var_zeta52_dn11, locals.var_zeta52_dn14,)
    }
};
        locals.var_zeta52 = assign59570_e92989;
        locals.var_zeta52_dn0 = assign59570_e92989_d_n0;
        locals.var_zeta52_dn2 = assign59570_e92989_d_n2;
        locals.var_zeta52_dn4 = assign59570_e92989_d_n4;
        locals.var_zeta52_dn5 = assign59570_e92989_d_n5;
        locals.var_zeta52_dn6 = assign59570_e92989_d_n6;
        locals.var_zeta52_dn7 = assign59570_e92989_d_n7;
        locals.var_zeta52_dn8 = assign59570_e92989_d_n8;
        locals.var_zeta52_dn9 = assign59570_e92989_d_n9;
        locals.var_zeta52_dn10 = assign59570_e92989_d_n10;
        locals.var_zeta52_dn11 = assign59570_e92989_d_n11;
        locals.var_zeta52_dn14 = assign59570_e92989_d_n14;

        let (assign59580_e92998, assign59580_e92998_d_n0, assign59580_e92998_d_n2, assign59580_e92998_d_n4, assign59580_e92998_d_n5, assign59580_e92998_d_n6, assign59580_e92998_d_n7, assign59580_e92998_d_n8, assign59580_e92998_d_n9, assign59580_e92998_d_n10, assign59580_e92998_d_n11, assign59580_e92998_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59580_e92996: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign59580_e92996, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn4 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn4)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn5 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn5)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn8 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn8)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn9 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn9)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn14 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn14)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn4, locals.var_f00_dn5, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn8, locals.var_f00_dn9, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn14,)
    }
};
        locals.var_f00 = assign59580_e92998;
        locals.var_f00_dn0 = assign59580_e92998_d_n0;
        locals.var_f00_dn2 = assign59580_e92998_d_n2;
        locals.var_f00_dn4 = assign59580_e92998_d_n4;
        locals.var_f00_dn5 = assign59580_e92998_d_n5;
        locals.var_f00_dn6 = assign59580_e92998_d_n6;
        locals.var_f00_dn7 = assign59580_e92998_d_n7;
        locals.var_f00_dn8 = assign59580_e92998_d_n8;
        locals.var_f00_dn9 = assign59580_e92998_d_n9;
        locals.var_f00_dn10 = assign59580_e92998_d_n10;
        locals.var_f00_dn11 = assign59580_e92998_d_n11;
        locals.var_f00_dn14 = assign59580_e92998_d_n14;

        let (assign59590_e93011, assign59590_e93011_d_n0, assign59590_e93011_d_n2, assign59590_e93011_d_n4, assign59590_e93011_d_n5, assign59590_e93011_d_n6, assign59590_e93011_d_n7, assign59590_e93011_d_n8, assign59590_e93011_d_n9, assign59590_e93011_d_n10, assign59590_e93011_d_n11, assign59590_e93011_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59590_e93007: f64 = (3.0 + locals.var_eta);
        let assign59590_e93008: f64 = (locals.var_eta * assign59590_e93007);
        let assign59590_e93009: f64 = (3.0 + assign59590_e93008);
        (assign59590_e93009, ((locals.var_eta_dn0 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn0)), ((locals.var_eta_dn2 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn2)), ((locals.var_eta_dn4 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn4)), ((locals.var_eta_dn5 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn5)), ((locals.var_eta_dn6 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn6)), ((locals.var_eta_dn7 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn7)), ((locals.var_eta_dn8 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn8)), ((locals.var_eta_dn9 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn9)), ((locals.var_eta_dn10 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn10)), ((locals.var_eta_dn11 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn11)), ((locals.var_eta_dn14 * assign59590_e93007) + (locals.var_eta * locals.var_eta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59590_e93011;
        locals.var_t1_dn0 = assign59590_e93011_d_n0;
        locals.var_t1_dn2 = assign59590_e93011_d_n2;
        locals.var_t1_dn4 = assign59590_e93011_d_n4;
        locals.var_t1_dn5 = assign59590_e93011_d_n5;
        locals.var_t1_dn6 = assign59590_e93011_d_n6;
        locals.var_t1_dn7 = assign59590_e93011_d_n7;
        locals.var_t1_dn8 = assign59590_e93011_d_n8;
        locals.var_t1_dn9 = assign59590_e93011_d_n9;
        locals.var_t1_dn10 = assign59590_e93011_d_n10;
        locals.var_t1_dn11 = assign59590_e93011_d_n11;
        locals.var_t1_dn14 = assign59590_e93011_d_n14;

        let (assign59600_e93024, assign59600_e93024_d_n0, assign59600_e93024_d_n2, assign59600_e93024_d_n4, assign59600_e93024_d_n5, assign59600_e93024_d_n6, assign59600_e93024_d_n7, assign59600_e93024_d_n8, assign59600_e93024_d_n9, assign59600_e93024_d_n10, assign59600_e93024_d_n11, assign59600_e93024_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59600_e93018: f64 = (0.6666666666666667 * locals.var_xi0p12);
        let assign59600_e93020: f64 = (assign59600_e93018 * locals.var_zeta32);
        let assign59600_e93022: f64 = (assign59600_e93020 * locals.var_t1);
        (assign59600_e93022, (((((0.6666666666666667 * locals.var_xi0p12_dn0) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn0)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn0)), (((((0.6666666666666667 * locals.var_xi0p12_dn2) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn2)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn2)), (((((0.6666666666666667 * locals.var_xi0p12_dn4) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn4)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn4)), (((((0.6666666666666667 * locals.var_xi0p12_dn5) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn5)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn5)), (((((0.6666666666666667 * locals.var_xi0p12_dn6) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn6)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn6)), (((((0.6666666666666667 * locals.var_xi0p12_dn7) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn7)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn7)), (((((0.6666666666666667 * locals.var_xi0p12_dn8) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn8)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn8)), (((((0.6666666666666667 * locals.var_xi0p12_dn9) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn9)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn9)), (((((0.6666666666666667 * locals.var_xi0p12_dn10) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn10)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn10)), (((((0.6666666666666667 * locals.var_xi0p12_dn11) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn11)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn11)), (((((0.6666666666666667 * locals.var_xi0p12_dn14) * locals.var_zeta32) + (assign59600_e93018 * locals.var_zeta32_dn14)) * locals.var_t1) + (assign59600_e93020 * locals.var_t1_dn14)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn4, locals.var_f10_dn5, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn8, locals.var_f10_dn9, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn14,)
    }
};
        locals.var_f10 = assign59600_e93024;
        locals.var_f10_dn0 = assign59600_e93024_d_n0;
        locals.var_f10_dn2 = assign59600_e93024_d_n2;
        locals.var_f10_dn4 = assign59600_e93024_d_n4;
        locals.var_f10_dn5 = assign59600_e93024_d_n5;
        locals.var_f10_dn6 = assign59600_e93024_d_n6;
        locals.var_f10_dn7 = assign59600_e93024_d_n7;
        locals.var_f10_dn8 = assign59600_e93024_d_n8;
        locals.var_f10_dn9 = assign59600_e93024_d_n9;
        locals.var_f10_dn10 = assign59600_e93024_d_n10;
        locals.var_f10_dn11 = assign59600_e93024_d_n11;
        locals.var_f10_dn14 = assign59600_e93024_d_n14;

        let (assign59610_e93045, assign59610_e93045_d_n0, assign59610_e93045_d_n2, assign59610_e93045_d_n4, assign59610_e93045_d_n5, assign59610_e93045_d_n6, assign59610_e93045_d_n7, assign59610_e93045_d_n8, assign59610_e93045_d_n9, assign59610_e93045_d_n10, assign59610_e93045_d_n11, assign59610_e93045_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59610_e93037: f64 = (5.0 + locals.var_eta);
        let assign59610_e93038: f64 = (locals.var_eta * assign59610_e93037);
        let assign59610_e93039: f64 = (10.0 + assign59610_e93038);
        let assign59610_e93040: f64 = (locals.var_eta * assign59610_e93039);
        let assign59610_e93041: f64 = (10.0 + assign59610_e93040);
        let assign59610_e93042: f64 = (locals.var_eta * assign59610_e93041);
        let assign59610_e93043: f64 = (5.0 + assign59610_e93042);
        (assign59610_e93043, ((locals.var_eta_dn0 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn0 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn0 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn0)))))), ((locals.var_eta_dn2 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn2 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn2 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn2)))))), ((locals.var_eta_dn4 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn4 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn4 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn4)))))), ((locals.var_eta_dn5 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn5 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn5 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn5)))))), ((locals.var_eta_dn6 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn6 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn6 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn6)))))), ((locals.var_eta_dn7 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn7 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn7 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn7)))))), ((locals.var_eta_dn8 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn8 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn8 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn8)))))), ((locals.var_eta_dn9 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn9 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn9 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn9)))))), ((locals.var_eta_dn10 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn10 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn10 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn10)))))), ((locals.var_eta_dn11 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn11 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn11 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn11)))))), ((locals.var_eta_dn14 * assign59610_e93041) + (locals.var_eta * ((locals.var_eta_dn14 * assign59610_e93039) + (locals.var_eta * ((locals.var_eta_dn14 * assign59610_e93037) + (locals.var_eta * locals.var_eta_dn14)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59610_e93045;
        locals.var_t1_dn0 = assign59610_e93045_d_n0;
        locals.var_t1_dn2 = assign59610_e93045_d_n2;
        locals.var_t1_dn4 = assign59610_e93045_d_n4;
        locals.var_t1_dn5 = assign59610_e93045_d_n5;
        locals.var_t1_dn6 = assign59610_e93045_d_n6;
        locals.var_t1_dn7 = assign59610_e93045_d_n7;
        locals.var_t1_dn8 = assign59610_e93045_d_n8;
        locals.var_t1_dn9 = assign59610_e93045_d_n9;
        locals.var_t1_dn10 = assign59610_e93045_d_n10;
        locals.var_t1_dn11 = assign59610_e93045_d_n11;
        locals.var_t1_dn14 = assign59610_e93045_d_n14;

        let (assign59620_e93062, assign59620_e93062_d_n0, assign59620_e93062_d_n2, assign59620_e93062_d_n4, assign59620_e93062_d_n5, assign59620_e93062_d_n6, assign59620_e93062_d_n7, assign59620_e93062_d_n8, assign59620_e93062_d_n9, assign59620_e93062_d_n10, assign59620_e93062_d_n11, assign59620_e93062_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59620_e93053: f64 = (15.0 * locals.var_beta);
        let assign59620_e93054: f64 = (4.0 / assign59620_e93053);
        let assign59620_e93056: f64 = (assign59620_e93054 * locals.var_xi0p32);
        let assign59620_e93058: f64 = (assign59620_e93056 * locals.var_zeta52);
        let assign59620_e93060: f64 = (assign59620_e93058 * locals.var_t1);
        (assign59620_e93060, (((((((-((4.0 * (15.0 * locals.var_beta_dn0)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn0)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn0)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn0)), (((((((-((4.0 * (15.0 * locals.var_beta_dn2)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn2)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn2)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn2)), (((((((-((4.0 * (15.0 * locals.var_beta_dn4)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn4)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn4)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn4)), (((((((-((4.0 * (15.0 * locals.var_beta_dn5)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn5)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn5)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn5)), (((((((-((4.0 * (15.0 * locals.var_beta_dn6)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn6)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn6)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn6)), (((((((-((4.0 * (15.0 * locals.var_beta_dn7)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn7)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn7)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn7)), (((((((-((4.0 * (15.0 * locals.var_beta_dn8)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn8)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn8)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn8)), (((((((-((4.0 * (15.0 * locals.var_beta_dn9)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn9)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn9)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn9)), (((((((-((4.0 * (15.0 * locals.var_beta_dn10)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn10)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn10)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn10)), (((((((-((4.0 * (15.0 * locals.var_beta_dn11)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn11)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn11)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn11)), (((((((-((4.0 * (15.0 * locals.var_beta_dn14)) / (assign59620_e93053 * assign59620_e93053))) * locals.var_xi0p32) + (assign59620_e93054 * locals.var_xi0p32_dn14)) * locals.var_zeta52) + (assign59620_e93056 * locals.var_zeta52_dn14)) * locals.var_t1) + (assign59620_e93058 * locals.var_t1_dn14)),)
    } else {
        (locals.var_f30, locals.var_f30_dn0, locals.var_f30_dn2, locals.var_f30_dn4, locals.var_f30_dn5, locals.var_f30_dn6, locals.var_f30_dn7, locals.var_f30_dn8, locals.var_f30_dn9, locals.var_f30_dn10, locals.var_f30_dn11, locals.var_f30_dn14,)
    }
};
        locals.var_f30 = assign59620_e93062;
        locals.var_f30_dn0 = assign59620_e93062_d_n0;
        locals.var_f30_dn2 = assign59620_e93062_d_n2;
        locals.var_f30_dn4 = assign59620_e93062_d_n4;
        locals.var_f30_dn5 = assign59620_e93062_d_n5;
        locals.var_f30_dn6 = assign59620_e93062_d_n6;
        locals.var_f30_dn7 = assign59620_e93062_d_n7;
        locals.var_f30_dn8 = assign59620_e93062_d_n8;
        locals.var_f30_dn9 = assign59620_e93062_d_n9;
        locals.var_f30_dn10 = assign59620_e93062_d_n10;
        locals.var_f30_dn11 = assign59620_e93062_d_n11;
        locals.var_f30_dn14 = assign59620_e93062_d_n14;

    }

    pub(super) fn stamp_transient_block_210(
        locals: &mut StampLocals,
    ) {
        let (assign59630_e93079, assign59630_e93079_d_n0, assign59630_e93079_d_n2, assign59630_e93079_d_n4, assign59630_e93079_d_n5, assign59630_e93079_d_n6, assign59630_e93079_d_n7, assign59630_e93079_d_n8, assign59630_e93079_d_n9, assign59630_e93079_d_n10, assign59630_e93079_d_n11, assign59630_e93079_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59630_e93069: f64 = (locals.var_ps0 * locals.var_f10);
        let assign59630_e93072: f64 = (0.6666666666666667 * locals.var_beta_inv);
        let assign59630_e93074: f64 = (assign59630_e93072 * locals.var_xilp32);
        let assign59630_e93075: f64 = (assign59630_e93069 + assign59630_e93074);
        let assign59630_e93077: f64 = (assign59630_e93075 - locals.var_f30);
        (assign59630_e93077, ((((locals.var_ps0_dn0 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn0)) + (((0.6666666666666667 * locals.var_beta_inv_dn0) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn0))) - locals.var_f30_dn0), ((((locals.var_ps0_dn2 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn2)) + (((0.6666666666666667 * locals.var_beta_inv_dn2) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn2))) - locals.var_f30_dn2), ((((locals.var_ps0_dn4 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn4)) + (((0.6666666666666667 * locals.var_beta_inv_dn4) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn4))) - locals.var_f30_dn4), ((((locals.var_ps0_dn5 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn5)) + (((0.6666666666666667 * locals.var_beta_inv_dn5) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn5))) - locals.var_f30_dn5), ((((locals.var_ps0_dn6 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn6)) + (((0.6666666666666667 * locals.var_beta_inv_dn6) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn6))) - locals.var_f30_dn6), ((((locals.var_ps0_dn7 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn7)) + (((0.6666666666666667 * locals.var_beta_inv_dn7) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn7))) - locals.var_f30_dn7), ((((locals.var_ps0_dn8 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn8)) + (((0.6666666666666667 * locals.var_beta_inv_dn8) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn8))) - locals.var_f30_dn8), ((((locals.var_ps0_dn9 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn9)) + (((0.6666666666666667 * locals.var_beta_inv_dn9) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn9))) - locals.var_f30_dn9), ((((locals.var_ps0_dn10 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn10)) + (((0.6666666666666667 * locals.var_beta_inv_dn10) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn10))) - locals.var_f30_dn10), ((((locals.var_ps0_dn11 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn11)) + (((0.6666666666666667 * locals.var_beta_inv_dn11) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn11))) - locals.var_f30_dn11), ((((locals.var_ps0_dn14 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn14)) + (((0.6666666666666667 * locals.var_beta_inv_dn14) * locals.var_xilp32) + (assign59630_e93072 * locals.var_xilp32_dn14))) - locals.var_f30_dn14),)
    } else {
        (locals.var_f11, locals.var_f11_dn0, locals.var_f11_dn2, locals.var_f11_dn4, locals.var_f11_dn5, locals.var_f11_dn6, locals.var_f11_dn7, locals.var_f11_dn8, locals.var_f11_dn9, locals.var_f11_dn10, locals.var_f11_dn11, locals.var_f11_dn14,)
    }
};
        locals.var_f11 = assign59630_e93079;
        locals.var_f11_dn0 = assign59630_e93079_d_n0;
        locals.var_f11_dn2 = assign59630_e93079_d_n2;
        locals.var_f11_dn4 = assign59630_e93079_d_n4;
        locals.var_f11_dn5 = assign59630_e93079_d_n5;
        locals.var_f11_dn6 = assign59630_e93079_d_n6;
        locals.var_f11_dn7 = assign59630_e93079_d_n7;
        locals.var_f11_dn8 = assign59630_e93079_d_n8;
        locals.var_f11_dn9 = assign59630_e93079_d_n9;
        locals.var_f11_dn10 = assign59630_e93079_d_n10;
        locals.var_f11_dn11 = assign59630_e93079_d_n11;
        locals.var_f11_dn14 = assign59630_e93079_d_n14;

        let (assign59640_e93096, assign59640_e93096_d_n0, assign59640_e93096_d_n2, assign59640_e93096_d_n4, assign59640_e93096_d_n5, assign59640_e93096_d_n6, assign59640_e93096_d_n7, assign59640_e93096_d_n8, assign59640_e93096_d_n9, assign59640_e93096_d_n10, assign59640_e93096_d_n11, assign59640_e93096_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59640_e93086: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign59640_e93090: f64 = (2.0 * locals.var_ps0);
        let assign59640_e93092: f64 = (assign59640_e93090 + locals.var_pds);
        let assign59640_e93093: f64 = (0.5 * assign59640_e93092);
        let assign59640_e93094: f64 = (assign59640_e93086 - assign59640_e93093);
        (assign59640_e93094, ((locals.var_vgp_dn0 + locals.var_beta_inv_dn0) - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), ((locals.var_vgp_dn2 + locals.var_beta_inv_dn2) - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), ((locals.var_vgp_dn4 + locals.var_beta_inv_dn4) - (0.5 * ((2.0 * locals.var_ps0_dn4) + locals.var_pds_dn4))), ((locals.var_vgp_dn5 + locals.var_beta_inv_dn5) - (0.5 * ((2.0 * locals.var_ps0_dn5) + locals.var_pds_dn5))), ((locals.var_vgp_dn6 + locals.var_beta_inv_dn6) - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), ((locals.var_vgp_dn7 + locals.var_beta_inv_dn7) - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn8 + locals.var_beta_inv_dn8) - (0.5 * ((2.0 * locals.var_ps0_dn8) + locals.var_pds_dn8))), ((locals.var_vgp_dn9 + locals.var_beta_inv_dn9) - (0.5 * ((2.0 * locals.var_ps0_dn9) + locals.var_pds_dn9))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), ((locals.var_vgp_dn11 + locals.var_beta_inv_dn11) - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), ((locals.var_vgp_dn14 + locals.var_beta_inv_dn14) - (0.5 * ((2.0 * locals.var_ps0_dn14) + locals.var_pds_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59640_e93096;
        locals.var_t1_dn0 = assign59640_e93096_d_n0;
        locals.var_t1_dn2 = assign59640_e93096_d_n2;
        locals.var_t1_dn4 = assign59640_e93096_d_n4;
        locals.var_t1_dn5 = assign59640_e93096_d_n5;
        locals.var_t1_dn6 = assign59640_e93096_d_n6;
        locals.var_t1_dn7 = assign59640_e93096_d_n7;
        locals.var_t1_dn8 = assign59640_e93096_d_n8;
        locals.var_t1_dn9 = assign59640_e93096_d_n9;
        locals.var_t1_dn10 = assign59640_e93096_d_n10;
        locals.var_t1_dn11 = assign59640_e93096_d_n11;
        locals.var_t1_dn14 = assign59640_e93096_d_n14;

        let (assign59650_e93106, assign59650_e93106_d_n0, assign59650_e93106_d_n2, assign59650_e93106_d_n4, assign59650_e93106_d_n5, assign59650_e93106_d_n6, assign59650_e93106_d_n7, assign59650_e93106_d_n8, assign59650_e93106_d_n9, assign59650_e93106_d_n10, assign59650_e93106_d_n11, assign59650_e93106_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59650_e93102: f64 = (-locals.var_f10);
        let assign59650_e93104: f64 = (assign59650_e93102 + locals.var_f00);
        (assign59650_e93104, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn4) + locals.var_f00_dn4), ((-locals.var_f10_dn5) + locals.var_f00_dn5), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn8) + locals.var_f00_dn8), ((-locals.var_f10_dn9) + locals.var_f00_dn9), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn14) + locals.var_f00_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign59650_e93106;
        locals.var_t2_dn0 = assign59650_e93106_d_n0;
        locals.var_t2_dn2 = assign59650_e93106_d_n2;
        locals.var_t2_dn4 = assign59650_e93106_d_n4;
        locals.var_t2_dn5 = assign59650_e93106_d_n5;
        locals.var_t2_dn6 = assign59650_e93106_d_n6;
        locals.var_t2_dn7 = assign59650_e93106_d_n7;
        locals.var_t2_dn8 = assign59650_e93106_d_n8;
        locals.var_t2_dn9 = assign59650_e93106_d_n9;
        locals.var_t2_dn10 = assign59650_e93106_d_n10;
        locals.var_t2_dn11 = assign59650_e93106_d_n11;
        locals.var_t2_dn14 = assign59650_e93106_d_n14;

        let (assign59660_e93115, assign59660_e93115_d_n0, assign59660_e93115_d_n2, assign59660_e93115_d_n4, assign59660_e93115_d_n5, assign59660_e93115_d_n6, assign59660_e93115_d_n7, assign59660_e93115_d_n8, assign59660_e93115_d_n9, assign59660_e93115_d_n10, assign59660_e93115_d_n11, assign59660_e93115_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59660_e93113: f64 = (locals.var_beta * locals.var_cox);
        (assign59660_e93113, ((locals.var_beta_dn0 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn0)), ((locals.var_beta_dn2 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn2)), ((locals.var_beta_dn4 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn4)), ((locals.var_beta_dn5 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn5)), ((locals.var_beta_dn6 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn6)), ((locals.var_beta_dn7 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn7)), ((locals.var_beta_dn8 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn8)), ((locals.var_beta_dn9 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn9)), ((locals.var_beta_dn10 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn10)), ((locals.var_beta_dn11 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn11)), ((locals.var_beta_dn14 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign59660_e93115;
        locals.var_t3_dn0 = assign59660_e93115_d_n0;
        locals.var_t3_dn2 = assign59660_e93115_d_n2;
        locals.var_t3_dn4 = assign59660_e93115_d_n4;
        locals.var_t3_dn5 = assign59660_e93115_d_n5;
        locals.var_t3_dn6 = assign59660_e93115_d_n6;
        locals.var_t3_dn7 = assign59660_e93115_d_n7;
        locals.var_t3_dn8 = assign59660_e93115_d_n8;
        locals.var_t3_dn9 = assign59660_e93115_d_n9;
        locals.var_t3_dn10 = assign59660_e93115_d_n10;
        locals.var_t3_dn11 = assign59660_e93115_d_n11;
        locals.var_t3_dn14 = assign59660_e93115_d_n14;

        let (assign59670_e93124, assign59670_e93124_d_n0, assign59670_e93124_d_n2, assign59670_e93124_d_n4, assign59670_e93124_d_n5, assign59670_e93124_d_n6, assign59670_e93124_d_n7, assign59670_e93124_d_n8, assign59670_e93124_d_n9, assign59670_e93124_d_n10, assign59670_e93124_d_n11, assign59670_e93124_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59670_e93122: f64 = (locals.var_beta * locals.var_cnst0);
        (assign59670_e93122, ((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)), ((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)), ((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)), ((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)), ((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)), ((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)), ((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)), ((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)), ((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)), ((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)), ((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign59670_e93124;
        locals.var_t4_dn0 = assign59670_e93124_d_n0;
        locals.var_t4_dn2 = assign59670_e93124_d_n2;
        locals.var_t4_dn4 = assign59670_e93124_d_n4;
        locals.var_t4_dn5 = assign59670_e93124_d_n5;
        locals.var_t4_dn6 = assign59670_e93124_d_n6;
        locals.var_t4_dn7 = assign59670_e93124_d_n7;
        locals.var_t4_dn8 = assign59670_e93124_d_n8;
        locals.var_t4_dn9 = assign59670_e93124_d_n9;
        locals.var_t4_dn10 = assign59670_e93124_d_n10;
        locals.var_t4_dn11 = assign59670_e93124_d_n11;
        locals.var_t4_dn14 = assign59670_e93124_d_n14;

        let (assign59680_e93137, assign59680_e93137_d_n0, assign59680_e93137_d_n2, assign59680_e93137_d_n4, assign59680_e93137_d_n5, assign59680_e93137_d_n6, assign59680_e93137_d_n7, assign59680_e93137_d_n8, assign59680_e93137_d_n9, assign59680_e93137_d_n10, assign59680_e93137_d_n11, assign59680_e93137_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59680_e93131: f64 = (locals.var_t3 * locals.var_t1);
        let assign59680_e93134: f64 = (locals.var_t4 * locals.var_t2);
        let assign59680_e93135: f64 = (assign59680_e93131 + assign59680_e93134);
        (assign59680_e93135, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn4 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4))), (((locals.var_t3_dn5 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn8 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8))), (((locals.var_t3_dn9 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn14 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn14))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn4, locals.var_fdd_dn5, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn8, locals.var_fdd_dn9, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn14,)
    }
};
        locals.var_fdd = assign59680_e93137;
        locals.var_fdd_dn0 = assign59680_e93137_d_n0;
        locals.var_fdd_dn2 = assign59680_e93137_d_n2;
        locals.var_fdd_dn4 = assign59680_e93137_d_n4;
        locals.var_fdd_dn5 = assign59680_e93137_d_n5;
        locals.var_fdd_dn6 = assign59680_e93137_d_n6;
        locals.var_fdd_dn7 = assign59680_e93137_d_n7;
        locals.var_fdd_dn8 = assign59680_e93137_d_n8;
        locals.var_fdd_dn9 = assign59680_e93137_d_n9;
        locals.var_fdd_dn10 = assign59680_e93137_d_n10;
        locals.var_fdd_dn11 = assign59680_e93137_d_n11;
        locals.var_fdd_dn14 = assign59680_e93137_d_n14;

        let (assign59690_e93146, assign59690_e93146_d_n0, assign59690_e93146_d_n2, assign59690_e93146_d_n4, assign59690_e93146_d_n5, assign59690_e93146_d_n6, assign59690_e93146_d_n7, assign59690_e93146_d_n8, assign59690_e93146_d_n9, assign59690_e93146_d_n10, assign59690_e93146_d_n11, assign59690_e93146_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign59690_e93144: f64 = (locals.var_pds * locals.var_fdd);
        (assign59690_e93144, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn4 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn4)), ((locals.var_pds_dn5 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn5)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn8 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn8)), ((locals.var_pds_dn9 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn9)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn14 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn14)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign59690_e93146;
        locals.var_idd_dn0 = assign59690_e93146_d_n0;
        locals.var_idd_dn2 = assign59690_e93146_d_n2;
        locals.var_idd_dn4 = assign59690_e93146_d_n4;
        locals.var_idd_dn5 = assign59690_e93146_d_n5;
        locals.var_idd_dn6 = assign59690_e93146_d_n6;
        locals.var_idd_dn7 = assign59690_e93146_d_n7;
        locals.var_idd_dn8 = assign59690_e93146_d_n8;
        locals.var_idd_dn9 = assign59690_e93146_d_n9;
        locals.var_idd_dn10 = assign59690_e93146_d_n10;
        locals.var_idd_dn11 = assign59690_e93146_d_n11;
        locals.var_idd_dn14 = assign59690_e93146_d_n14;

        let assign59700_e93149: f64 = if locals.var_flg_zone == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign59700_e93149;

        let (assign59710_e93158,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1462 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign59710_e93158;

        let assign59720_e93161: f64 = if locals.var_start_of_mobility == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign59720_e93161;

        let assign59730_e93165: f64 = (10.0 * 2.220446049250313e-16);
        let assign59730_e93170: f64 = (10.0 * 2.220446049250313e-16);
        let assign59730_e93172: f64 = if ((locals.var_uc_clm2 < assign59730_e93165) && (locals.var_uc_clm3 < assign59730_e93170)) { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign59730_e93172;

        let (assign59740_e93183, assign59740_e93183_d_n0, assign59740_e93183_d_n2, assign59740_e93183_d_n4, assign59740_e93183_d_n5, assign59740_e93183_d_n6, assign59740_e93183_d_n7, assign59740_e93183_d_n8, assign59740_e93183_d_n9, assign59740_e93183_d_n10, assign59740_e93183_d_n11, assign59740_e93183_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign59740_e93183;
        locals.var_lred_dn0 = assign59740_e93183_d_n0;
        locals.var_lred_dn2 = assign59740_e93183_d_n2;
        locals.var_lred_dn4 = assign59740_e93183_d_n4;
        locals.var_lred_dn5 = assign59740_e93183_d_n5;
        locals.var_lred_dn6 = assign59740_e93183_d_n6;
        locals.var_lred_dn7 = assign59740_e93183_d_n7;
        locals.var_lred_dn8 = assign59740_e93183_d_n8;
        locals.var_lred_dn9 = assign59740_e93183_d_n9;
        locals.var_lred_dn10 = assign59740_e93183_d_n10;
        locals.var_lred_dn11 = assign59740_e93183_d_n11;
        locals.var_lred_dn14 = assign59740_e93183_d_n14;

        let (assign59750_e93194, assign59750_e93194_d_n0, assign59750_e93194_d_n2, assign59750_e93194_d_n4, assign59750_e93194_d_n5, assign59750_e93194_d_n6, assign59750_e93194_d_n7, assign59750_e93194_d_n8, assign59750_e93194_d_n9, assign59750_e93194_d_n10, assign59750_e93194_d_n11, assign59750_e93194_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign59750_e93194;
        locals.var_psdl_dn0 = assign59750_e93194_d_n0;
        locals.var_psdl_dn2 = assign59750_e93194_d_n2;
        locals.var_psdl_dn4 = assign59750_e93194_d_n4;
        locals.var_psdl_dn5 = assign59750_e93194_d_n5;
        locals.var_psdl_dn6 = assign59750_e93194_d_n6;
        locals.var_psdl_dn7 = assign59750_e93194_d_n7;
        locals.var_psdl_dn8 = assign59750_e93194_d_n8;
        locals.var_psdl_dn9 = assign59750_e93194_d_n9;
        locals.var_psdl_dn10 = assign59750_e93194_d_n10;
        locals.var_psdl_dn11 = assign59750_e93194_d_n11;
        locals.var_psdl_dn14 = assign59750_e93194_d_n14;

        let assign59760_e93198: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59760_e93201: f64 = (10.0 * 2.220446049250313e-16);
        let assign59760_e93202: f64 = (assign59760_e93198 - assign59760_e93201);
        let assign59760_e93205: f64 = (10.0 * 2.220446049250313e-16);
        let assign59760_e93206: f64 = (assign59760_e93202 - assign59760_e93205);
        let assign59760_e93210: f64 = (10.0 * 2.220446049250313e-16);
        let assign59760_e93213: f64 = if ((locals.var_psdl > assign59760_e93206) && (assign59760_e93210 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign59760_e93213;

        let (assign59770_e93238, assign59770_e93238_d_n0, assign59770_e93238_d_n2, assign59770_e93238_d_n4, assign59770_e93238_d_n5, assign59770_e93238_d_n6, assign59770_e93238_d_n7, assign59770_e93238_d_n8, assign59770_e93238_d_n9, assign59770_e93238_d_n10, assign59770_e93238_d_n11, assign59770_e93238_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59770_e93227: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59770_e93230: f64 = (10.0 * 2.220446049250313e-16);
        let assign59770_e93231: f64 = (assign59770_e93227 - assign59770_e93230);
        let assign59770_e93232: f64 = (locals.var_psdl - assign59770_e93231);
        let assign59770_e93235: f64 = (10.0 * 2.220446049250313e-16);
        let assign59770_e93236: f64 = (assign59770_e93232 + assign59770_e93235);
        (assign59770_e93236, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign59770_e93238;
        locals.var_tmf1_dn0 = assign59770_e93238_d_n0;
        locals.var_tmf1_dn2 = assign59770_e93238_d_n2;
        locals.var_tmf1_dn4 = assign59770_e93238_d_n4;
        locals.var_tmf1_dn5 = assign59770_e93238_d_n5;
        locals.var_tmf1_dn6 = assign59770_e93238_d_n6;
        locals.var_tmf1_dn7 = assign59770_e93238_d_n7;
        locals.var_tmf1_dn8 = assign59770_e93238_d_n8;
        locals.var_tmf1_dn9 = assign59770_e93238_d_n9;
        locals.var_tmf1_dn10 = assign59770_e93238_d_n10;
        locals.var_tmf1_dn11 = assign59770_e93238_d_n11;
        locals.var_tmf1_dn14 = assign59770_e93238_d_n14;

        let (assign59780_e93253, assign59780_e93253_d_n0, assign59780_e93253_d_n2, assign59780_e93253_d_n4, assign59780_e93253_d_n5, assign59780_e93253_d_n6, assign59780_e93253_d_n7, assign59780_e93253_d_n8, assign59780_e93253_d_n9, assign59780_e93253_d_n10, assign59780_e93253_d_n11, assign59780_e93253_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59780_e93251: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign59780_e93251, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign59780_e93253;
        locals.var_x2_dn0 = assign59780_e93253_d_n0;
        locals.var_x2_dn2 = assign59780_e93253_d_n2;
        locals.var_x2_dn4 = assign59780_e93253_d_n4;
        locals.var_x2_dn5 = assign59780_e93253_d_n5;
        locals.var_x2_dn6 = assign59780_e93253_d_n6;
        locals.var_x2_dn7 = assign59780_e93253_d_n7;
        locals.var_x2_dn8 = assign59780_e93253_d_n8;
        locals.var_x2_dn9 = assign59780_e93253_d_n9;
        locals.var_x2_dn10 = assign59780_e93253_d_n10;
        locals.var_x2_dn11 = assign59780_e93253_d_n11;
        locals.var_x2_dn14 = assign59780_e93253_d_n14;

        let (assign59790_e93272, assign59790_e93272_d_n0, assign59790_e93272_d_n2, assign59790_e93272_d_n4, assign59790_e93272_d_n5, assign59790_e93272_d_n6, assign59790_e93272_d_n7, assign59790_e93272_d_n8, assign59790_e93272_d_n9, assign59790_e93272_d_n10, assign59790_e93272_d_n11, assign59790_e93272_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59790_e93266: f64 = (10.0 * 2.220446049250313e-16);
        let assign59790_e93269: f64 = (10.0 * 2.220446049250313e-16);
        let assign59790_e93270: f64 = (assign59790_e93266 * assign59790_e93269);
        (assign59790_e93270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign59790_e93272;
        locals.var_xmax2_dn0 = assign59790_e93272_d_n0;
        locals.var_xmax2_dn2 = assign59790_e93272_d_n2;
        locals.var_xmax2_dn4 = assign59790_e93272_d_n4;
        locals.var_xmax2_dn5 = assign59790_e93272_d_n5;
        locals.var_xmax2_dn6 = assign59790_e93272_d_n6;
        locals.var_xmax2_dn7 = assign59790_e93272_d_n7;
        locals.var_xmax2_dn8 = assign59790_e93272_d_n8;
        locals.var_xmax2_dn9 = assign59790_e93272_d_n9;
        locals.var_xmax2_dn10 = assign59790_e93272_d_n10;
        locals.var_xmax2_dn11 = assign59790_e93272_d_n11;
        locals.var_xmax2_dn14 = assign59790_e93272_d_n14;

        let (assign59800_e93285, assign59800_e93285_d_n0, assign59800_e93285_d_n2, assign59800_e93285_d_n4, assign59800_e93285_d_n5, assign59800_e93285_d_n6, assign59800_e93285_d_n7, assign59800_e93285_d_n8, assign59800_e93285_d_n9, assign59800_e93285_d_n10, assign59800_e93285_d_n11, assign59800_e93285_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59800_e93285;
        locals.var_xp_dn0 = assign59800_e93285_d_n0;
        locals.var_xp_dn2 = assign59800_e93285_d_n2;
        locals.var_xp_dn4 = assign59800_e93285_d_n4;
        locals.var_xp_dn5 = assign59800_e93285_d_n5;
        locals.var_xp_dn6 = assign59800_e93285_d_n6;
        locals.var_xp_dn7 = assign59800_e93285_d_n7;
        locals.var_xp_dn8 = assign59800_e93285_d_n8;
        locals.var_xp_dn9 = assign59800_e93285_d_n9;
        locals.var_xp_dn10 = assign59800_e93285_d_n10;
        locals.var_xp_dn11 = assign59800_e93285_d_n11;
        locals.var_xp_dn14 = assign59800_e93285_d_n14;

        let (assign59810_e93298, assign59810_e93298_d_n0, assign59810_e93298_d_n2, assign59810_e93298_d_n4, assign59810_e93298_d_n5, assign59810_e93298_d_n6, assign59810_e93298_d_n7, assign59810_e93298_d_n8, assign59810_e93298_d_n9, assign59810_e93298_d_n10, assign59810_e93298_d_n11, assign59810_e93298_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59810_e93298;
        locals.var_xmp_dn0 = assign59810_e93298_d_n0;
        locals.var_xmp_dn2 = assign59810_e93298_d_n2;
        locals.var_xmp_dn4 = assign59810_e93298_d_n4;
        locals.var_xmp_dn5 = assign59810_e93298_d_n5;
        locals.var_xmp_dn6 = assign59810_e93298_d_n6;
        locals.var_xmp_dn7 = assign59810_e93298_d_n7;
        locals.var_xmp_dn8 = assign59810_e93298_d_n8;
        locals.var_xmp_dn9 = assign59810_e93298_d_n9;
        locals.var_xmp_dn10 = assign59810_e93298_d_n10;
        locals.var_xmp_dn11 = assign59810_e93298_d_n11;
        locals.var_xmp_dn14 = assign59810_e93298_d_n14;

        let (assign59820_e93311,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign59820_e93311;

        let (assign59830_e93324,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59830_e93324;

        let (assign59840_e93337, assign59840_e93337_d_n0, assign59840_e93337_d_n2, assign59840_e93337_d_n4, assign59840_e93337_d_n5, assign59840_e93337_d_n6, assign59840_e93337_d_n7, assign59840_e93337_d_n8, assign59840_e93337_d_n9, assign59840_e93337_d_n10, assign59840_e93337_d_n11, assign59840_e93337_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign59840_e93337;
        locals.var_arg_dn0 = assign59840_e93337_d_n0;
        locals.var_arg_dn2 = assign59840_e93337_d_n2;
        locals.var_arg_dn4 = assign59840_e93337_d_n4;
        locals.var_arg_dn5 = assign59840_e93337_d_n5;
        locals.var_arg_dn6 = assign59840_e93337_d_n6;
        locals.var_arg_dn7 = assign59840_e93337_d_n7;
        locals.var_arg_dn8 = assign59840_e93337_d_n8;
        locals.var_arg_dn9 = assign59840_e93337_d_n9;
        locals.var_arg_dn10 = assign59840_e93337_d_n10;
        locals.var_arg_dn11 = assign59840_e93337_d_n11;
        locals.var_arg_dn14 = assign59840_e93337_d_n14;

        let (assign59850_e93350, assign59850_e93350_d_n0, assign59850_e93350_d_n2, assign59850_e93350_d_n4, assign59850_e93350_d_n5, assign59850_e93350_d_n6, assign59850_e93350_d_n7, assign59850_e93350_d_n8, assign59850_e93350_d_n9, assign59850_e93350_d_n10, assign59850_e93350_d_n11, assign59850_e93350_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign59850_e93350;
        locals.var_dnm_dn0 = assign59850_e93350_d_n0;
        locals.var_dnm_dn2 = assign59850_e93350_d_n2;
        locals.var_dnm_dn4 = assign59850_e93350_d_n4;
        locals.var_dnm_dn5 = assign59850_e93350_d_n5;
        locals.var_dnm_dn6 = assign59850_e93350_d_n6;
        locals.var_dnm_dn7 = assign59850_e93350_d_n7;
        locals.var_dnm_dn8 = assign59850_e93350_d_n8;
        locals.var_dnm_dn9 = assign59850_e93350_d_n9;
        locals.var_dnm_dn10 = assign59850_e93350_d_n10;
        locals.var_dnm_dn11 = assign59850_e93350_d_n11;
        locals.var_dnm_dn14 = assign59850_e93350_d_n14;

        let (assign59860_e93365, assign59860_e93365_d_n0, assign59860_e93365_d_n2, assign59860_e93365_d_n4, assign59860_e93365_d_n5, assign59860_e93365_d_n6, assign59860_e93365_d_n7, assign59860_e93365_d_n8, assign59860_e93365_d_n9, assign59860_e93365_d_n10, assign59860_e93365_d_n11, assign59860_e93365_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59860_e93363: f64 = (locals.var_xp * locals.var_x2);
        (assign59860_e93363, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59860_e93365;
        locals.var_xp_dn0 = assign59860_e93365_d_n0;
        locals.var_xp_dn2 = assign59860_e93365_d_n2;
        locals.var_xp_dn4 = assign59860_e93365_d_n4;
        locals.var_xp_dn5 = assign59860_e93365_d_n5;
        locals.var_xp_dn6 = assign59860_e93365_d_n6;
        locals.var_xp_dn7 = assign59860_e93365_d_n7;
        locals.var_xp_dn8 = assign59860_e93365_d_n8;
        locals.var_xp_dn9 = assign59860_e93365_d_n9;
        locals.var_xp_dn10 = assign59860_e93365_d_n10;
        locals.var_xp_dn11 = assign59860_e93365_d_n11;
        locals.var_xp_dn14 = assign59860_e93365_d_n14;

        let (assign59870_e93380, assign59870_e93380_d_n0, assign59870_e93380_d_n2, assign59870_e93380_d_n4, assign59870_e93380_d_n5, assign59870_e93380_d_n6, assign59870_e93380_d_n7, assign59870_e93380_d_n8, assign59870_e93380_d_n9, assign59870_e93380_d_n10, assign59870_e93380_d_n11, assign59870_e93380_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59870_e93378: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59870_e93378, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59870_e93380;
        locals.var_xmp_dn0 = assign59870_e93380_d_n0;
        locals.var_xmp_dn2 = assign59870_e93380_d_n2;
        locals.var_xmp_dn4 = assign59870_e93380_d_n4;
        locals.var_xmp_dn5 = assign59870_e93380_d_n5;
        locals.var_xmp_dn6 = assign59870_e93380_d_n6;
        locals.var_xmp_dn7 = assign59870_e93380_d_n7;
        locals.var_xmp_dn8 = assign59870_e93380_d_n8;
        locals.var_xmp_dn9 = assign59870_e93380_d_n9;
        locals.var_xmp_dn10 = assign59870_e93380_d_n10;
        locals.var_xmp_dn11 = assign59870_e93380_d_n11;
        locals.var_xmp_dn14 = assign59870_e93380_d_n14;

        let (assign59880_e93395, assign59880_e93395_d_n0, assign59880_e93395_d_n2, assign59880_e93395_d_n4, assign59880_e93395_d_n5, assign59880_e93395_d_n6, assign59880_e93395_d_n7, assign59880_e93395_d_n8, assign59880_e93395_d_n9, assign59880_e93395_d_n10, assign59880_e93395_d_n11, assign59880_e93395_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59880_e93393: f64 = (locals.var_xp * locals.var_x2);
        (assign59880_e93393, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59880_e93395;
        locals.var_xp_dn0 = assign59880_e93395_d_n0;
        locals.var_xp_dn2 = assign59880_e93395_d_n2;
        locals.var_xp_dn4 = assign59880_e93395_d_n4;
        locals.var_xp_dn5 = assign59880_e93395_d_n5;
        locals.var_xp_dn6 = assign59880_e93395_d_n6;
        locals.var_xp_dn7 = assign59880_e93395_d_n7;
        locals.var_xp_dn8 = assign59880_e93395_d_n8;
        locals.var_xp_dn9 = assign59880_e93395_d_n9;
        locals.var_xp_dn10 = assign59880_e93395_d_n10;
        locals.var_xp_dn11 = assign59880_e93395_d_n11;
        locals.var_xp_dn14 = assign59880_e93395_d_n14;

        let (assign59890_e93410, assign59890_e93410_d_n0, assign59890_e93410_d_n2, assign59890_e93410_d_n4, assign59890_e93410_d_n5, assign59890_e93410_d_n6, assign59890_e93410_d_n7, assign59890_e93410_d_n8, assign59890_e93410_d_n9, assign59890_e93410_d_n10, assign59890_e93410_d_n11, assign59890_e93410_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59890_e93408: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59890_e93408, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59890_e93410;
        locals.var_xmp_dn0 = assign59890_e93410_d_n0;
        locals.var_xmp_dn2 = assign59890_e93410_d_n2;
        locals.var_xmp_dn4 = assign59890_e93410_d_n4;
        locals.var_xmp_dn5 = assign59890_e93410_d_n5;
        locals.var_xmp_dn6 = assign59890_e93410_d_n6;
        locals.var_xmp_dn7 = assign59890_e93410_d_n7;
        locals.var_xmp_dn8 = assign59890_e93410_d_n8;
        locals.var_xmp_dn9 = assign59890_e93410_d_n9;
        locals.var_xmp_dn10 = assign59890_e93410_d_n10;
        locals.var_xmp_dn11 = assign59890_e93410_d_n11;
        locals.var_xmp_dn14 = assign59890_e93410_d_n14;

        let (assign59900_e93425, assign59900_e93425_d_n0, assign59900_e93425_d_n2, assign59900_e93425_d_n4, assign59900_e93425_d_n5, assign59900_e93425_d_n6, assign59900_e93425_d_n7, assign59900_e93425_d_n8, assign59900_e93425_d_n9, assign59900_e93425_d_n10, assign59900_e93425_d_n11, assign59900_e93425_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign59900_e93423: f64 = (locals.var_xp + locals.var_xmp);
        (assign59900_e93423, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign59900_e93425;
        locals.var_arg_dn0 = assign59900_e93425_d_n0;
        locals.var_arg_dn2 = assign59900_e93425_d_n2;
        locals.var_arg_dn4 = assign59900_e93425_d_n4;
        locals.var_arg_dn5 = assign59900_e93425_d_n5;
        locals.var_arg_dn6 = assign59900_e93425_d_n6;
        locals.var_arg_dn7 = assign59900_e93425_d_n7;
        locals.var_arg_dn8 = assign59900_e93425_d_n8;
        locals.var_arg_dn9 = assign59900_e93425_d_n9;
        locals.var_arg_dn10 = assign59900_e93425_d_n10;
        locals.var_arg_dn11 = assign59900_e93425_d_n11;
        locals.var_arg_dn14 = assign59900_e93425_d_n14;

    }

    pub(super) fn stamp_transient_block_211(
        locals: &mut StampLocals,
    ) {
        let (assign59910_e93438, assign59910_e93438_d_n0, assign59910_e93438_d_n2, assign59910_e93438_d_n4, assign59910_e93438_d_n5, assign59910_e93438_d_n6, assign59910_e93438_d_n7, assign59910_e93438_d_n8, assign59910_e93438_d_n9, assign59910_e93438_d_n10, assign59910_e93438_d_n11, assign59910_e93438_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign59910_e93438;
        locals.var_dnm_dn0 = assign59910_e93438_d_n0;
        locals.var_dnm_dn2 = assign59910_e93438_d_n2;
        locals.var_dnm_dn4 = assign59910_e93438_d_n4;
        locals.var_dnm_dn5 = assign59910_e93438_d_n5;
        locals.var_dnm_dn6 = assign59910_e93438_d_n6;
        locals.var_dnm_dn7 = assign59910_e93438_d_n7;
        locals.var_dnm_dn8 = assign59910_e93438_d_n8;
        locals.var_dnm_dn9 = assign59910_e93438_d_n9;
        locals.var_dnm_dn10 = assign59910_e93438_d_n10;
        locals.var_dnm_dn11 = assign59910_e93438_d_n11;
        locals.var_dnm_dn14 = assign59910_e93438_d_n14;

        let assign59920_e93453: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign59920_e93453;

        let assign59930_e93456: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign59930_e93456;

        let (assign59940_e93473,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59940_e93473;

        let assign59950_e93476: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign59950_e93476;

        let (assign59960_e93496,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 == 0.0)) && (locals.var_guard1468 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59960_e93496;

        let assign59970_e93499: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign59970_e93499;

        let (assign59980_e93522,) = {
    if (((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 == 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59980_e93522;

        let assign59990_e93525: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign59990_e93525;

        let (assign60000_e93551,) = {
    if ((((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 == 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1470 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60000_e93551;

        let (assign60010_e93566,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60010_e93566;

        let mut assign60020_loop_guard: usize = 0;
        while {
            let assign60020_cond_e93582: f64 = if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60020_cond_e93582 != 0.0
        } {
            assign60020_loop_guard += 1;
            assert!(assign60020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60020_body0_e93598, assign60020_body0_e93598_d_n0, assign60020_body0_e93598_d_n2, assign60020_body0_e93598_d_n4, assign60020_body0_e93598_d_n5, assign60020_body0_e93598_d_n6, assign60020_body0_e93598_d_n7, assign60020_body0_e93598_d_n8, assign60020_body0_e93598_d_n9, assign60020_body0_e93598_d_n10, assign60020_body0_e93598_d_n11, assign60020_body0_e93598_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        let assign60020_body0_e93596: f64 = (locals.var_dnm).sqrt();
        (assign60020_body0_e93596, (locals.var_dnm_dn0 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn2 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn4 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn5 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn6 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn7 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn8 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn9 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn10 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn11 / (2.0 * assign60020_body0_e93596)), (locals.var_dnm_dn14 / (2.0 * assign60020_body0_e93596)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign60020_body0_e93598;
            locals.var_dnm_dn0 = assign60020_body0_e93598_d_n0;
            locals.var_dnm_dn2 = assign60020_body0_e93598_d_n2;
            locals.var_dnm_dn4 = assign60020_body0_e93598_d_n4;
            locals.var_dnm_dn5 = assign60020_body0_e93598_d_n5;
            locals.var_dnm_dn6 = assign60020_body0_e93598_d_n6;
            locals.var_dnm_dn7 = assign60020_body0_e93598_d_n7;
            locals.var_dnm_dn8 = assign60020_body0_e93598_d_n8;
            locals.var_dnm_dn9 = assign60020_body0_e93598_d_n9;
            locals.var_dnm_dn10 = assign60020_body0_e93598_d_n10;
            locals.var_dnm_dn11 = assign60020_body0_e93598_d_n11;
            locals.var_dnm_dn14 = assign60020_body0_e93598_d_n14;
            let (assign60020_body1_e93615,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        let assign60020_body1_e93613: f64 = (locals.var_m0 + 1.0);
        (assign60020_body1_e93613,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60020_body1_e93615;
        }

        let (assign60030_e93642, assign60030_e93642_d_n0, assign60030_e93642_d_n2, assign60030_e93642_d_n4, assign60030_e93642_d_n5, assign60030_e93642_d_n6, assign60030_e93642_d_n7, assign60030_e93642_d_n8, assign60030_e93642_d_n9, assign60030_e93642_d_n10, assign60030_e93642_d_n11, assign60030_e93642_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let (assign60030_e93640, assign60030_e93640_d_n0, assign60030_e93640_d_n2, assign60030_e93640_d_n4, assign60030_e93640_d_n5, assign60030_e93640_d_n6, assign60030_e93640_d_n7, assign60030_e93640_d_n8, assign60030_e93640_d_n9, assign60030_e93640_d_n10, assign60030_e93640_d_n11, assign60030_e93640_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60030_e93637: f64 = (2.0 * 2.0);
                let assign60030_e93638: f64 = (1.0 / assign60030_e93637);
                let assign60030_e93639: f64 = (locals.var_dnm).powf(assign60030_e93638);
                (assign60030_e93639, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn11)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60030_e93638) as f64).is_finite() && ((assign60030_e93638) as f64).fract() == 0.0 { if assign60030_e93638 == 0.0 { 0.0 } else { (assign60030_e93638 * ((locals.var_dnm).powf(assign60030_e93638 - 1.0) * locals.var_dnm_dn14)) } } else { (assign60030_e93639 * (assign60030_e93638 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign60030_e93640, assign60030_e93640_d_n0, assign60030_e93640_d_n2, assign60030_e93640_d_n4, assign60030_e93640_d_n5, assign60030_e93640_d_n6, assign60030_e93640_d_n7, assign60030_e93640_d_n8, assign60030_e93640_d_n9, assign60030_e93640_d_n10, assign60030_e93640_d_n11, assign60030_e93640_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60030_e93642;
        locals.var_dnm_dn0 = assign60030_e93642_d_n0;
        locals.var_dnm_dn2 = assign60030_e93642_d_n2;
        locals.var_dnm_dn4 = assign60030_e93642_d_n4;
        locals.var_dnm_dn5 = assign60030_e93642_d_n5;
        locals.var_dnm_dn6 = assign60030_e93642_d_n6;
        locals.var_dnm_dn7 = assign60030_e93642_d_n7;
        locals.var_dnm_dn8 = assign60030_e93642_d_n8;
        locals.var_dnm_dn9 = assign60030_e93642_d_n9;
        locals.var_dnm_dn10 = assign60030_e93642_d_n10;
        locals.var_dnm_dn11 = assign60030_e93642_d_n11;
        locals.var_dnm_dn14 = assign60030_e93642_d_n14;

        let (assign60040_e93657, assign60040_e93657_d_n0, assign60040_e93657_d_n2, assign60040_e93657_d_n4, assign60040_e93657_d_n5, assign60040_e93657_d_n6, assign60040_e93657_d_n7, assign60040_e93657_d_n8, assign60040_e93657_d_n9, assign60040_e93657_d_n10, assign60040_e93657_d_n11, assign60040_e93657_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60040_e93655: f64 = (1.0 / locals.var_dnm);
        (assign60040_e93655, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60040_e93657;
        locals.var_dnm_dn0 = assign60040_e93657_d_n0;
        locals.var_dnm_dn2 = assign60040_e93657_d_n2;
        locals.var_dnm_dn4 = assign60040_e93657_d_n4;
        locals.var_dnm_dn5 = assign60040_e93657_d_n5;
        locals.var_dnm_dn6 = assign60040_e93657_d_n6;
        locals.var_dnm_dn7 = assign60040_e93657_d_n7;
        locals.var_dnm_dn8 = assign60040_e93657_d_n8;
        locals.var_dnm_dn9 = assign60040_e93657_d_n9;
        locals.var_dnm_dn10 = assign60040_e93657_d_n10;
        locals.var_dnm_dn11 = assign60040_e93657_d_n11;
        locals.var_dnm_dn14 = assign60040_e93657_d_n14;

        let (assign60050_e93676, assign60050_e93676_d_n0, assign60050_e93676_d_n2, assign60050_e93676_d_n4, assign60050_e93676_d_n5, assign60050_e93676_d_n6, assign60050_e93676_d_n7, assign60050_e93676_d_n8, assign60050_e93676_d_n9, assign60050_e93676_d_n10, assign60050_e93676_d_n11, assign60050_e93676_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60050_e93671: f64 = (10.0 * 2.220446049250313e-16);
        let assign60050_e93672: f64 = (locals.var_tmf1 * assign60050_e93671);
        let assign60050_e93674: f64 = (assign60050_e93672 * locals.var_dnm);
        (assign60050_e93674, (((locals.var_tmf1_dn0 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign60050_e93671) * locals.var_dnm) + (assign60050_e93672 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign60050_e93676;
        locals.var_tmf0_dn0 = assign60050_e93676_d_n0;
        locals.var_tmf0_dn2 = assign60050_e93676_d_n2;
        locals.var_tmf0_dn4 = assign60050_e93676_d_n4;
        locals.var_tmf0_dn5 = assign60050_e93676_d_n5;
        locals.var_tmf0_dn6 = assign60050_e93676_d_n6;
        locals.var_tmf0_dn7 = assign60050_e93676_d_n7;
        locals.var_tmf0_dn8 = assign60050_e93676_d_n8;
        locals.var_tmf0_dn9 = assign60050_e93676_d_n9;
        locals.var_tmf0_dn10 = assign60050_e93676_d_n10;
        locals.var_tmf0_dn11 = assign60050_e93676_d_n11;
        locals.var_tmf0_dn14 = assign60050_e93676_d_n14;

        let (assign60060_e93697, assign60060_e93697_d_n0, assign60060_e93697_d_n2, assign60060_e93697_d_n4, assign60060_e93697_d_n5, assign60060_e93697_d_n6, assign60060_e93697_d_n7, assign60060_e93697_d_n8, assign60060_e93697_d_n9, assign60060_e93697_d_n10, assign60060_e93697_d_n11, assign60060_e93697_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60060_e93689: f64 = (10.0 * 2.220446049250313e-16);
        let assign60060_e93691: f64 = (assign60060_e93689 * locals.var_xmp);
        let assign60060_e93693: f64 = (assign60060_e93691 * locals.var_dnm);
        let assign60060_e93695: f64 = (assign60060_e93693 / locals.var_arg);
        (assign60060_e93695, ((((((assign60060_e93689 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn11) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn11)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign60060_e93689 * locals.var_xmp_dn14) * locals.var_dnm) + (assign60060_e93691 * locals.var_dnm_dn14)) * locals.var_arg) - (assign60060_e93693 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60060_e93697;
        locals.var_t0_dn0 = assign60060_e93697_d_n0;
        locals.var_t0_dn2 = assign60060_e93697_d_n2;
        locals.var_t0_dn4 = assign60060_e93697_d_n4;
        locals.var_t0_dn5 = assign60060_e93697_d_n5;
        locals.var_t0_dn6 = assign60060_e93697_d_n6;
        locals.var_t0_dn7 = assign60060_e93697_d_n7;
        locals.var_t0_dn8 = assign60060_e93697_d_n8;
        locals.var_t0_dn9 = assign60060_e93697_d_n9;
        locals.var_t0_dn10 = assign60060_e93697_d_n10;
        locals.var_t0_dn11 = assign60060_e93697_d_n11;
        locals.var_t0_dn14 = assign60060_e93697_d_n14;

        let (assign60070_e93722, assign60070_e93722_d_n0, assign60070_e93722_d_n2, assign60070_e93722_d_n4, assign60070_e93722_d_n5, assign60070_e93722_d_n6, assign60070_e93722_d_n7, assign60070_e93722_d_n8, assign60070_e93722_d_n9, assign60070_e93722_d_n10, assign60070_e93722_d_n11, assign60070_e93722_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60070_e93710: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60070_e93713: f64 = (10.0 * 2.220446049250313e-16);
        let assign60070_e93714: f64 = (assign60070_e93710 - assign60070_e93713);
        let assign60070_e93717: f64 = (10.0 * 2.220446049250313e-16);
        let assign60070_e93718: f64 = (assign60070_e93714 - assign60070_e93717);
        let assign60070_e93720: f64 = (assign60070_e93718 + locals.var_tmf0);
        (assign60070_e93720, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60070_e93722;
        locals.var_psdl_dn0 = assign60070_e93722_d_n0;
        locals.var_psdl_dn2 = assign60070_e93722_d_n2;
        locals.var_psdl_dn4 = assign60070_e93722_d_n4;
        locals.var_psdl_dn5 = assign60070_e93722_d_n5;
        locals.var_psdl_dn6 = assign60070_e93722_d_n6;
        locals.var_psdl_dn7 = assign60070_e93722_d_n7;
        locals.var_psdl_dn8 = assign60070_e93722_d_n8;
        locals.var_psdl_dn9 = assign60070_e93722_d_n9;
        locals.var_psdl_dn10 = assign60070_e93722_d_n10;
        locals.var_psdl_dn11 = assign60070_e93722_d_n11;
        locals.var_psdl_dn14 = assign60070_e93722_d_n14;

        let (assign60080_e93735, assign60080_e93735_d_n0, assign60080_e93735_d_n2, assign60080_e93735_d_n4, assign60080_e93735_d_n5, assign60080_e93735_d_n6, assign60080_e93735_d_n7, assign60080_e93735_d_n8, assign60080_e93735_d_n9, assign60080_e93735_d_n10, assign60080_e93735_d_n11, assign60080_e93735_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60080_e93735;
        locals.var_t0_dn0 = assign60080_e93735_d_n0;
        locals.var_t0_dn2 = assign60080_e93735_d_n2;
        locals.var_t0_dn4 = assign60080_e93735_d_n4;
        locals.var_t0_dn5 = assign60080_e93735_d_n5;
        locals.var_t0_dn6 = assign60080_e93735_d_n6;
        locals.var_t0_dn7 = assign60080_e93735_d_n7;
        locals.var_t0_dn8 = assign60080_e93735_d_n8;
        locals.var_t0_dn9 = assign60080_e93735_d_n9;
        locals.var_t0_dn10 = assign60080_e93735_d_n10;
        locals.var_t0_dn11 = assign60080_e93735_d_n11;
        locals.var_t0_dn14 = assign60080_e93735_d_n14;

        let (assign60090_e93749, assign60090_e93749_d_n0, assign60090_e93749_d_n2, assign60090_e93749_d_n4, assign60090_e93749_d_n5, assign60090_e93749_d_n6, assign60090_e93749_d_n7, assign60090_e93749_d_n8, assign60090_e93749_d_n9, assign60090_e93749_d_n10, assign60090_e93749_d_n11, assign60090_e93749_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60090_e93749;
        locals.var_psdl_dn0 = assign60090_e93749_d_n0;
        locals.var_psdl_dn2 = assign60090_e93749_d_n2;
        locals.var_psdl_dn4 = assign60090_e93749_d_n4;
        locals.var_psdl_dn5 = assign60090_e93749_d_n5;
        locals.var_psdl_dn6 = assign60090_e93749_d_n6;
        locals.var_psdl_dn7 = assign60090_e93749_d_n7;
        locals.var_psdl_dn8 = assign60090_e93749_d_n8;
        locals.var_psdl_dn9 = assign60090_e93749_d_n9;
        locals.var_psdl_dn10 = assign60090_e93749_d_n10;
        locals.var_psdl_dn11 = assign60090_e93749_d_n11;
        locals.var_psdl_dn14 = assign60090_e93749_d_n14;

        let (assign60100_e93763, assign60100_e93763_d_n0, assign60100_e93763_d_n2, assign60100_e93763_d_n4, assign60100_e93763_d_n5, assign60100_e93763_d_n6, assign60100_e93763_d_n7, assign60100_e93763_d_n8, assign60100_e93763_d_n9, assign60100_e93763_d_n10, assign60100_e93763_d_n11, assign60100_e93763_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60100_e93763;
        locals.var_t0_dn0 = assign60100_e93763_d_n0;
        locals.var_t0_dn2 = assign60100_e93763_d_n2;
        locals.var_t0_dn4 = assign60100_e93763_d_n4;
        locals.var_t0_dn5 = assign60100_e93763_d_n5;
        locals.var_t0_dn6 = assign60100_e93763_d_n6;
        locals.var_t0_dn7 = assign60100_e93763_d_n7;
        locals.var_t0_dn8 = assign60100_e93763_d_n8;
        locals.var_t0_dn9 = assign60100_e93763_d_n9;
        locals.var_t0_dn10 = assign60100_e93763_d_n10;
        locals.var_t0_dn11 = assign60100_e93763_d_n11;
        locals.var_t0_dn14 = assign60100_e93763_d_n14;

        let (assign60110_e93775, assign60110_e93775_d_n0, assign60110_e93775_d_n2, assign60110_e93775_d_n4, assign60110_e93775_d_n5, assign60110_e93775_d_n6, assign60110_e93775_d_n7, assign60110_e93775_d_n8, assign60110_e93775_d_n9, assign60110_e93775_d_n10, assign60110_e93775_d_n11, assign60110_e93775_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60110_e93775;
        locals.var_t1_dn0 = assign60110_e93775_d_n0;
        locals.var_t1_dn2 = assign60110_e93775_d_n2;
        locals.var_t1_dn4 = assign60110_e93775_d_n4;
        locals.var_t1_dn5 = assign60110_e93775_d_n5;
        locals.var_t1_dn6 = assign60110_e93775_d_n6;
        locals.var_t1_dn7 = assign60110_e93775_d_n7;
        locals.var_t1_dn8 = assign60110_e93775_d_n8;
        locals.var_t1_dn9 = assign60110_e93775_d_n9;
        locals.var_t1_dn10 = assign60110_e93775_d_n10;
        locals.var_t1_dn11 = assign60110_e93775_d_n11;
        locals.var_t1_dn14 = assign60110_e93775_d_n14;

        let (assign60120_e93790, assign60120_e93790_d_n0, assign60120_e93790_d_n2, assign60120_e93790_d_n4, assign60120_e93790_d_n5, assign60120_e93790_d_n6, assign60120_e93790_d_n7, assign60120_e93790_d_n8, assign60120_e93790_d_n9, assign60120_e93790_d_n10, assign60120_e93790_d_n11, assign60120_e93790_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60120_e93787: f64 = (locals.var_psl - locals.var_vbscl__blk437);
        let assign60120_e93788: f64 = (assign60120_e93787).sqrt();
        (assign60120_e93788, ((locals.var_psl_dn0 - locals.var_vbscl__blk437_dn0) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn2 - locals.var_vbscl__blk437_dn2) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn4 - locals.var_vbscl__blk437_dn4) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn5 - locals.var_vbscl__blk437_dn5) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn6 - locals.var_vbscl__blk437_dn6) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn7 - locals.var_vbscl__blk437_dn7) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn8 - locals.var_vbscl__blk437_dn8) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn9 - locals.var_vbscl__blk437_dn9) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn10 - locals.var_vbscl__blk437_dn10) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn11 - locals.var_vbscl__blk437_dn11) / (2.0 * assign60120_e93788)), ((locals.var_psl_dn14 - locals.var_vbscl__blk437_dn14) / (2.0 * assign60120_e93788)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign60120_e93790;
        locals.var_t8_dn0 = assign60120_e93790_d_n0;
        locals.var_t8_dn2 = assign60120_e93790_d_n2;
        locals.var_t8_dn4 = assign60120_e93790_d_n4;
        locals.var_t8_dn5 = assign60120_e93790_d_n5;
        locals.var_t8_dn6 = assign60120_e93790_d_n6;
        locals.var_t8_dn7 = assign60120_e93790_d_n7;
        locals.var_t8_dn8 = assign60120_e93790_d_n8;
        locals.var_t8_dn9 = assign60120_e93790_d_n9;
        locals.var_t8_dn10 = assign60120_e93790_d_n10;
        locals.var_t8_dn11 = assign60120_e93790_d_n11;
        locals.var_t8_dn14 = assign60120_e93790_d_n14;

        let (assign60130_e93804, assign60130_e93804_d_n0, assign60130_e93804_d_n2, assign60130_e93804_d_n4, assign60130_e93804_d_n5, assign60130_e93804_d_n6, assign60130_e93804_d_n7, assign60130_e93804_d_n8, assign60130_e93804_d_n9, assign60130_e93804_d_n10, assign60130_e93804_d_n11, assign60130_e93804_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60130_e93802: f64 = (locals.var_t1 * locals.var_t8);
        (assign60130_e93802, ((locals.var_t1_dn0 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn0)), ((locals.var_t1_dn2 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn2)), ((locals.var_t1_dn4 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn4)), ((locals.var_t1_dn5 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn5)), ((locals.var_t1_dn6 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn6)), ((locals.var_t1_dn7 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn7)), ((locals.var_t1_dn8 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn8)), ((locals.var_t1_dn9 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn9)), ((locals.var_t1_dn10 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn10)), ((locals.var_t1_dn11 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn11)), ((locals.var_t1_dn14 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn14)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn2, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn11, locals.var_wd_dn14,)
    }
};
        locals.var_wd = assign60130_e93804;
        locals.var_wd_dn0 = assign60130_e93804_d_n0;
        locals.var_wd_dn2 = assign60130_e93804_d_n2;
        locals.var_wd_dn4 = assign60130_e93804_d_n4;
        locals.var_wd_dn5 = assign60130_e93804_d_n5;
        locals.var_wd_dn6 = assign60130_e93804_d_n6;
        locals.var_wd_dn7 = assign60130_e93804_d_n7;
        locals.var_wd_dn8 = assign60130_e93804_d_n8;
        locals.var_wd_dn9 = assign60130_e93804_d_n9;
        locals.var_wd_dn10 = assign60130_e93804_d_n10;
        locals.var_wd_dn11 = assign60130_e93804_d_n11;
        locals.var_wd_dn14 = assign60130_e93804_d_n14;

        let (assign60140_e93820, assign60140_e93820_d_n0, assign60140_e93820_d_n2, assign60140_e93820_d_n4, assign60140_e93820_d_n5, assign60140_e93820_d_n6, assign60140_e93820_d_n7, assign60140_e93820_d_n8, assign60140_e93820_d_n9, assign60140_e93820_d_n10, assign60140_e93820_d_n11, assign60140_e93820_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60140_e93816: f64 = (0.5 * locals.var_t1);
        let assign60140_e93818: f64 = (assign60140_e93816 / locals.var_t8);
        (assign60140_e93818, ((((0.5 * locals.var_t1_dn0) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn0)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn2) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn2)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn4) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn5) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn6) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn7) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn8) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn9) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn10) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn11) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn14) * locals.var_t8) - (assign60140_e93816 * locals.var_t8_dn14)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign60140_e93820;
        locals.var_t9_dn0 = assign60140_e93820_d_n0;
        locals.var_t9_dn2 = assign60140_e93820_d_n2;
        locals.var_t9_dn4 = assign60140_e93820_d_n4;
        locals.var_t9_dn5 = assign60140_e93820_d_n5;
        locals.var_t9_dn6 = assign60140_e93820_d_n6;
        locals.var_t9_dn7 = assign60140_e93820_d_n7;
        locals.var_t9_dn8 = assign60140_e93820_d_n8;
        locals.var_t9_dn9 = assign60140_e93820_d_n9;
        locals.var_t9_dn10 = assign60140_e93820_d_n10;
        locals.var_t9_dn11 = assign60140_e93820_d_n11;
        locals.var_t9_dn14 = assign60140_e93820_d_n14;

        let (assign60150_e93834, assign60150_e93834_d_n0, assign60150_e93834_d_n2, assign60150_e93834_d_n4, assign60150_e93834_d_n5, assign60150_e93834_d_n6, assign60150_e93834_d_n7, assign60150_e93834_d_n8, assign60150_e93834_d_n9, assign60150_e93834_d_n10, assign60150_e93834_d_n11, assign60150_e93834_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60150_e93832: f64 = (1.0 / locals.var_wd);
        (assign60150_e93832, (-(locals.var_wd_dn0 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn2 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn4 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn5 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn6 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn7 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn8 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn9 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn10 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn11 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn14 / (locals.var_wd * locals.var_wd))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60150_e93834;
        locals.var_t0_dn0 = assign60150_e93834_d_n0;
        locals.var_t0_dn2 = assign60150_e93834_d_n2;
        locals.var_t0_dn4 = assign60150_e93834_d_n4;
        locals.var_t0_dn5 = assign60150_e93834_d_n5;
        locals.var_t0_dn6 = assign60150_e93834_d_n6;
        locals.var_t0_dn7 = assign60150_e93834_d_n7;
        locals.var_t0_dn8 = assign60150_e93834_d_n8;
        locals.var_t0_dn9 = assign60150_e93834_d_n9;
        locals.var_t0_dn10 = assign60150_e93834_d_n10;
        locals.var_t0_dn11 = assign60150_e93834_d_n11;
        locals.var_t0_dn14 = assign60150_e93834_d_n14;

        let (assign60160_e93848, assign60160_e93848_d_n0, assign60160_e93848_d_n2, assign60160_e93848_d_n4, assign60160_e93848_d_n5, assign60160_e93848_d_n6, assign60160_e93848_d_n7, assign60160_e93848_d_n8, assign60160_e93848_d_n9, assign60160_e93848_d_n10, assign60160_e93848_d_n11, assign60160_e93848_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60160_e93846: f64 = (locals.var_qn0 * locals.var_t0);
        (assign60160_e93846, ((locals.var_qn0_dn0 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn0)), ((locals.var_qn0_dn2 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn2)), ((locals.var_qn0_dn4 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn4)), ((locals.var_qn0_dn5 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn5)), ((locals.var_qn0_dn6 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn6)), ((locals.var_qn0_dn7 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn7)), ((locals.var_qn0_dn8 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn8)), ((locals.var_qn0_dn9 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn9)), ((locals.var_qn0_dn10 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn10)), ((locals.var_qn0_dn11 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn11)), ((locals.var_qn0_dn14 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60160_e93848;
        locals.var_t1_dn0 = assign60160_e93848_d_n0;
        locals.var_t1_dn2 = assign60160_e93848_d_n2;
        locals.var_t1_dn4 = assign60160_e93848_d_n4;
        locals.var_t1_dn5 = assign60160_e93848_d_n5;
        locals.var_t1_dn6 = assign60160_e93848_d_n6;
        locals.var_t1_dn7 = assign60160_e93848_d_n7;
        locals.var_t1_dn8 = assign60160_e93848_d_n8;
        locals.var_t1_dn9 = assign60160_e93848_d_n9;
        locals.var_t1_dn10 = assign60160_e93848_d_n10;
        locals.var_t1_dn11 = assign60160_e93848_d_n11;
        locals.var_t1_dn14 = assign60160_e93848_d_n14;

        let (assign60170_e93862, assign60170_e93862_d_n0, assign60170_e93862_d_n2, assign60170_e93862_d_n4, assign60170_e93862_d_n5, assign60170_e93862_d_n6, assign60170_e93862_d_n7, assign60170_e93862_d_n8, assign60170_e93862_d_n9, assign60170_e93862_d_n10, assign60170_e93862_d_n11, assign60170_e93862_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60170_e93860: f64 = (locals.var_uc_clm3 * locals.var_t1);
        (assign60170_e93860, (locals.var_uc_clm3 * locals.var_t1_dn0), (locals.var_uc_clm3 * locals.var_t1_dn2), (locals.var_uc_clm3 * locals.var_t1_dn4), (locals.var_uc_clm3 * locals.var_t1_dn5), (locals.var_uc_clm3 * locals.var_t1_dn6), (locals.var_uc_clm3 * locals.var_t1_dn7), (locals.var_uc_clm3 * locals.var_t1_dn8), (locals.var_uc_clm3 * locals.var_t1_dn9), (locals.var_uc_clm3 * locals.var_t1_dn10), (locals.var_uc_clm3 * locals.var_t1_dn11), (locals.var_uc_clm3 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60170_e93862;
        locals.var_t2_dn0 = assign60170_e93862_d_n0;
        locals.var_t2_dn2 = assign60170_e93862_d_n2;
        locals.var_t2_dn4 = assign60170_e93862_d_n4;
        locals.var_t2_dn5 = assign60170_e93862_d_n5;
        locals.var_t2_dn6 = assign60170_e93862_d_n6;
        locals.var_t2_dn7 = assign60170_e93862_d_n7;
        locals.var_t2_dn8 = assign60170_e93862_d_n8;
        locals.var_t2_dn9 = assign60170_e93862_d_n9;
        locals.var_t2_dn10 = assign60170_e93862_d_n10;
        locals.var_t2_dn11 = assign60170_e93862_d_n11;
        locals.var_t2_dn14 = assign60170_e93862_d_n14;

        let (assign60180_e93876, assign60180_e93876_d_n0, assign60180_e93876_d_n2, assign60180_e93876_d_n4, assign60180_e93876_d_n5, assign60180_e93876_d_n6, assign60180_e93876_d_n7, assign60180_e93876_d_n8, assign60180_e93876_d_n9, assign60180_e93876_d_n10, assign60180_e93876_d_n11, assign60180_e93876_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60180_e93874: f64 = (locals.var_uc_clm3 * locals.var_t0);
        (assign60180_e93874, (locals.var_uc_clm3 * locals.var_t0_dn0), (locals.var_uc_clm3 * locals.var_t0_dn2), (locals.var_uc_clm3 * locals.var_t0_dn4), (locals.var_uc_clm3 * locals.var_t0_dn5), (locals.var_uc_clm3 * locals.var_t0_dn6), (locals.var_uc_clm3 * locals.var_t0_dn7), (locals.var_uc_clm3 * locals.var_t0_dn8), (locals.var_uc_clm3 * locals.var_t0_dn9), (locals.var_uc_clm3 * locals.var_t0_dn10), (locals.var_uc_clm3 * locals.var_t0_dn11), (locals.var_uc_clm3 * locals.var_t0_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60180_e93876;
        locals.var_t3_dn0 = assign60180_e93876_d_n0;
        locals.var_t3_dn2 = assign60180_e93876_d_n2;
        locals.var_t3_dn4 = assign60180_e93876_d_n4;
        locals.var_t3_dn5 = assign60180_e93876_d_n5;
        locals.var_t3_dn6 = assign60180_e93876_d_n6;
        locals.var_t3_dn7 = assign60180_e93876_d_n7;
        locals.var_t3_dn8 = assign60180_e93876_d_n8;
        locals.var_t3_dn9 = assign60180_e93876_d_n9;
        locals.var_t3_dn10 = assign60180_e93876_d_n10;
        locals.var_t3_dn11 = assign60180_e93876_d_n11;
        locals.var_t3_dn14 = assign60180_e93876_d_n14;

        let (assign60190_e93892, assign60190_e93892_d_n0, assign60190_e93892_d_n2, assign60190_e93892_d_n4, assign60190_e93892_d_n5, assign60190_e93892_d_n6, assign60190_e93892_d_n7, assign60190_e93892_d_n8, assign60190_e93892_d_n9, assign60190_e93892_d_n10, assign60190_e93892_d_n11, assign60190_e93892_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60190_e93888: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign60190_e93890: f64 = (assign60190_e93888 + locals.var_t2);
        (assign60190_e93890, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2_dn2), (((locals.var_uc_clm2_dn4 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn4)) + locals.var_t2_dn4), (((locals.var_uc_clm2_dn5 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn5)) + locals.var_t2_dn5), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2_dn7), (((locals.var_uc_clm2_dn8 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn8)) + locals.var_t2_dn8), (((locals.var_uc_clm2_dn9 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn9)) + locals.var_t2_dn9), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2_dn11), (((locals.var_uc_clm2_dn14 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn14)) + locals.var_t2_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign60190_e93892;
        locals.var_t5_dn0 = assign60190_e93892_d_n0;
        locals.var_t5_dn2 = assign60190_e93892_d_n2;
        locals.var_t5_dn4 = assign60190_e93892_d_n4;
        locals.var_t5_dn5 = assign60190_e93892_d_n5;
        locals.var_t5_dn6 = assign60190_e93892_d_n6;
        locals.var_t5_dn7 = assign60190_e93892_d_n7;
        locals.var_t5_dn8 = assign60190_e93892_d_n8;
        locals.var_t5_dn9 = assign60190_e93892_d_n9;
        locals.var_t5_dn10 = assign60190_e93892_d_n10;
        locals.var_t5_dn11 = assign60190_e93892_d_n11;
        locals.var_t5_dn14 = assign60190_e93892_d_n14;

    }

    pub(super) fn stamp_transient_block_212(
        locals: &mut StampLocals,
    ) {
        let (assign60200_e93906, assign60200_e93906_d_n0, assign60200_e93906_d_n2, assign60200_e93906_d_n4, assign60200_e93906_d_n5, assign60200_e93906_d_n6, assign60200_e93906_d_n7, assign60200_e93906_d_n8, assign60200_e93906_d_n9, assign60200_e93906_d_n10, assign60200_e93906_d_n11, assign60200_e93906_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60200_e93904: f64 = (1.0 / locals.var_t5);
        (assign60200_e93904, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn14 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60200_e93906;
        locals.var_t1_dn0 = assign60200_e93906_d_n0;
        locals.var_t1_dn2 = assign60200_e93906_d_n2;
        locals.var_t1_dn4 = assign60200_e93906_d_n4;
        locals.var_t1_dn5 = assign60200_e93906_d_n5;
        locals.var_t1_dn6 = assign60200_e93906_d_n6;
        locals.var_t1_dn7 = assign60200_e93906_d_n7;
        locals.var_t1_dn8 = assign60200_e93906_d_n8;
        locals.var_t1_dn9 = assign60200_e93906_d_n9;
        locals.var_t1_dn10 = assign60200_e93906_d_n10;
        locals.var_t1_dn11 = assign60200_e93906_d_n11;
        locals.var_t1_dn14 = assign60200_e93906_d_n14;

        let (assign60210_e93920, assign60210_e93920_d_n0, assign60210_e93920_d_n2, assign60210_e93920_d_n4, assign60210_e93920_d_n5, assign60210_e93920_d_n6, assign60210_e93920_d_n7, assign60210_e93920_d_n8, assign60210_e93920_d_n9, assign60210_e93920_d_n10, assign60210_e93920_d_n11, assign60210_e93920_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60210_e93918: f64 = (1.034943e-10 * locals.var_t1);
        (assign60210_e93918, (1.034943e-10 * locals.var_t1_dn0), (1.034943e-10 * locals.var_t1_dn2), (1.034943e-10 * locals.var_t1_dn4), (1.034943e-10 * locals.var_t1_dn5), (1.034943e-10 * locals.var_t1_dn6), (1.034943e-10 * locals.var_t1_dn7), (1.034943e-10 * locals.var_t1_dn8), (1.034943e-10 * locals.var_t1_dn9), (1.034943e-10 * locals.var_t1_dn10), (1.034943e-10 * locals.var_t1_dn11), (1.034943e-10 * locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign60210_e93920;
        locals.var_t4_dn0 = assign60210_e93920_d_n0;
        locals.var_t4_dn2 = assign60210_e93920_d_n2;
        locals.var_t4_dn4 = assign60210_e93920_d_n4;
        locals.var_t4_dn5 = assign60210_e93920_d_n5;
        locals.var_t4_dn6 = assign60210_e93920_d_n6;
        locals.var_t4_dn7 = assign60210_e93920_d_n7;
        locals.var_t4_dn8 = assign60210_e93920_d_n8;
        locals.var_t4_dn9 = assign60210_e93920_d_n9;
        locals.var_t4_dn10 = assign60210_e93920_d_n10;
        locals.var_t4_dn11 = assign60210_e93920_d_n11;
        locals.var_t4_dn14 = assign60210_e93920_d_n14;

        let (assign60220_e93934, assign60220_e93934_d_n0, assign60220_e93934_d_n2, assign60220_e93934_d_n4, assign60220_e93934_d_n5, assign60220_e93934_d_n6, assign60220_e93934_d_n7, assign60220_e93934_d_n8, assign60220_e93934_d_n9, assign60220_e93934_d_n10, assign60220_e93934_d_n11, assign60220_e93934_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60220_e93932: f64 = (1.0 - locals.var_uc_clm1);
        (assign60220_e93932, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60220_e93934;
        locals.var_t1_dn0 = assign60220_e93934_d_n0;
        locals.var_t1_dn2 = assign60220_e93934_d_n2;
        locals.var_t1_dn4 = assign60220_e93934_d_n4;
        locals.var_t1_dn5 = assign60220_e93934_d_n5;
        locals.var_t1_dn6 = assign60220_e93934_d_n6;
        locals.var_t1_dn7 = assign60220_e93934_d_n7;
        locals.var_t1_dn8 = assign60220_e93934_d_n8;
        locals.var_t1_dn9 = assign60220_e93934_d_n9;
        locals.var_t1_dn10 = assign60220_e93934_d_n10;
        locals.var_t1_dn11 = assign60220_e93934_d_n11;
        locals.var_t1_dn14 = assign60220_e93934_d_n14;

        let (assign60230_e93954, assign60230_e93954_d_n0, assign60230_e93954_d_n2, assign60230_e93954_d_n4, assign60230_e93954_d_n5, assign60230_e93954_d_n6, assign60230_e93954_d_n7, assign60230_e93954_d_n8, assign60230_e93954_d_n9, assign60230_e93954_d_n10, assign60230_e93954_d_n11, assign60230_e93954_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60230_e93947: f64 = (locals.var_vds + locals.var_ps0);
        let assign60230_e93948: f64 = (locals.var_uc_clm1 * assign60230_e93947);
        let assign60230_e93951: f64 = (locals.var_t1 * locals.var_psl);
        let assign60230_e93952: f64 = (assign60230_e93948 + assign60230_e93951);
        (assign60230_e93952, ((locals.var_uc_clm1 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1_dn0 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn0))), ((locals.var_uc_clm1 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1_dn2 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn2))), ((locals.var_uc_clm1 * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + ((locals.var_t1_dn4 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn4))), ((locals.var_uc_clm1 * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + ((locals.var_t1_dn5 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn5))), ((locals.var_uc_clm1 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1_dn6 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn6))), ((locals.var_uc_clm1 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1_dn7 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn7))), ((locals.var_uc_clm1 * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + ((locals.var_t1_dn8 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn8))), ((locals.var_uc_clm1 * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + ((locals.var_t1_dn9 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn9))), ((locals.var_uc_clm1 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1_dn10 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn10))), ((locals.var_uc_clm1 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1_dn11 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn11))), ((locals.var_uc_clm1 * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + ((locals.var_t1_dn14 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn14))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60230_e93954;
        locals.var_psdl_dn0 = assign60230_e93954_d_n0;
        locals.var_psdl_dn2 = assign60230_e93954_d_n2;
        locals.var_psdl_dn4 = assign60230_e93954_d_n4;
        locals.var_psdl_dn5 = assign60230_e93954_d_n5;
        locals.var_psdl_dn6 = assign60230_e93954_d_n6;
        locals.var_psdl_dn7 = assign60230_e93954_d_n7;
        locals.var_psdl_dn8 = assign60230_e93954_d_n8;
        locals.var_psdl_dn9 = assign60230_e93954_d_n9;
        locals.var_psdl_dn10 = assign60230_e93954_d_n10;
        locals.var_psdl_dn11 = assign60230_e93954_d_n11;
        locals.var_psdl_dn14 = assign60230_e93954_d_n14;

        let assign60240_e93958: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60240_e93961: f64 = (10.0 * 2.220446049250313e-16);
        let assign60240_e93962: f64 = (assign60240_e93958 - assign60240_e93961);
        let assign60240_e93965: f64 = (10.0 * 2.220446049250313e-16);
        let assign60240_e93966: f64 = (assign60240_e93962 - assign60240_e93965);
        let assign60240_e93970: f64 = (10.0 * 2.220446049250313e-16);
        let assign60240_e93973: f64 = if ((locals.var_psdl > assign60240_e93966) && (assign60240_e93970 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign60240_e93973;

        let (assign60250_e93999, assign60250_e93999_d_n0, assign60250_e93999_d_n2, assign60250_e93999_d_n4, assign60250_e93999_d_n5, assign60250_e93999_d_n6, assign60250_e93999_d_n7, assign60250_e93999_d_n8, assign60250_e93999_d_n9, assign60250_e93999_d_n10, assign60250_e93999_d_n11, assign60250_e93999_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60250_e93988: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60250_e93991: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e93992: f64 = (assign60250_e93988 - assign60250_e93991);
        let assign60250_e93993: f64 = (locals.var_psdl - assign60250_e93992);
        let assign60250_e93996: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e93997: f64 = (assign60250_e93993 + assign60250_e93996);
        (assign60250_e93997, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign60250_e93999;
        locals.var_tmf1_dn0 = assign60250_e93999_d_n0;
        locals.var_tmf1_dn2 = assign60250_e93999_d_n2;
        locals.var_tmf1_dn4 = assign60250_e93999_d_n4;
        locals.var_tmf1_dn5 = assign60250_e93999_d_n5;
        locals.var_tmf1_dn6 = assign60250_e93999_d_n6;
        locals.var_tmf1_dn7 = assign60250_e93999_d_n7;
        locals.var_tmf1_dn8 = assign60250_e93999_d_n8;
        locals.var_tmf1_dn9 = assign60250_e93999_d_n9;
        locals.var_tmf1_dn10 = assign60250_e93999_d_n10;
        locals.var_tmf1_dn11 = assign60250_e93999_d_n11;
        locals.var_tmf1_dn14 = assign60250_e93999_d_n14;

        let (assign60260_e94015, assign60260_e94015_d_n0, assign60260_e94015_d_n2, assign60260_e94015_d_n4, assign60260_e94015_d_n5, assign60260_e94015_d_n6, assign60260_e94015_d_n7, assign60260_e94015_d_n8, assign60260_e94015_d_n9, assign60260_e94015_d_n10, assign60260_e94015_d_n11, assign60260_e94015_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60260_e94013: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign60260_e94013, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign60260_e94015;
        locals.var_x2_dn0 = assign60260_e94015_d_n0;
        locals.var_x2_dn2 = assign60260_e94015_d_n2;
        locals.var_x2_dn4 = assign60260_e94015_d_n4;
        locals.var_x2_dn5 = assign60260_e94015_d_n5;
        locals.var_x2_dn6 = assign60260_e94015_d_n6;
        locals.var_x2_dn7 = assign60260_e94015_d_n7;
        locals.var_x2_dn8 = assign60260_e94015_d_n8;
        locals.var_x2_dn9 = assign60260_e94015_d_n9;
        locals.var_x2_dn10 = assign60260_e94015_d_n10;
        locals.var_x2_dn11 = assign60260_e94015_d_n11;
        locals.var_x2_dn14 = assign60260_e94015_d_n14;

        let (assign60270_e94035, assign60270_e94035_d_n0, assign60270_e94035_d_n2, assign60270_e94035_d_n4, assign60270_e94035_d_n5, assign60270_e94035_d_n6, assign60270_e94035_d_n7, assign60270_e94035_d_n8, assign60270_e94035_d_n9, assign60270_e94035_d_n10, assign60270_e94035_d_n11, assign60270_e94035_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60270_e94029: f64 = (10.0 * 2.220446049250313e-16);
        let assign60270_e94032: f64 = (10.0 * 2.220446049250313e-16);
        let assign60270_e94033: f64 = (assign60270_e94029 * assign60270_e94032);
        (assign60270_e94033, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign60270_e94035;
        locals.var_xmax2_dn0 = assign60270_e94035_d_n0;
        locals.var_xmax2_dn2 = assign60270_e94035_d_n2;
        locals.var_xmax2_dn4 = assign60270_e94035_d_n4;
        locals.var_xmax2_dn5 = assign60270_e94035_d_n5;
        locals.var_xmax2_dn6 = assign60270_e94035_d_n6;
        locals.var_xmax2_dn7 = assign60270_e94035_d_n7;
        locals.var_xmax2_dn8 = assign60270_e94035_d_n8;
        locals.var_xmax2_dn9 = assign60270_e94035_d_n9;
        locals.var_xmax2_dn10 = assign60270_e94035_d_n10;
        locals.var_xmax2_dn11 = assign60270_e94035_d_n11;
        locals.var_xmax2_dn14 = assign60270_e94035_d_n14;

        let (assign60280_e94049, assign60280_e94049_d_n0, assign60280_e94049_d_n2, assign60280_e94049_d_n4, assign60280_e94049_d_n5, assign60280_e94049_d_n6, assign60280_e94049_d_n7, assign60280_e94049_d_n8, assign60280_e94049_d_n9, assign60280_e94049_d_n10, assign60280_e94049_d_n11, assign60280_e94049_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60280_e94049;
        locals.var_xp_dn0 = assign60280_e94049_d_n0;
        locals.var_xp_dn2 = assign60280_e94049_d_n2;
        locals.var_xp_dn4 = assign60280_e94049_d_n4;
        locals.var_xp_dn5 = assign60280_e94049_d_n5;
        locals.var_xp_dn6 = assign60280_e94049_d_n6;
        locals.var_xp_dn7 = assign60280_e94049_d_n7;
        locals.var_xp_dn8 = assign60280_e94049_d_n8;
        locals.var_xp_dn9 = assign60280_e94049_d_n9;
        locals.var_xp_dn10 = assign60280_e94049_d_n10;
        locals.var_xp_dn11 = assign60280_e94049_d_n11;
        locals.var_xp_dn14 = assign60280_e94049_d_n14;

        let (assign60290_e94063, assign60290_e94063_d_n0, assign60290_e94063_d_n2, assign60290_e94063_d_n4, assign60290_e94063_d_n5, assign60290_e94063_d_n6, assign60290_e94063_d_n7, assign60290_e94063_d_n8, assign60290_e94063_d_n9, assign60290_e94063_d_n10, assign60290_e94063_d_n11, assign60290_e94063_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60290_e94063;
        locals.var_xmp_dn0 = assign60290_e94063_d_n0;
        locals.var_xmp_dn2 = assign60290_e94063_d_n2;
        locals.var_xmp_dn4 = assign60290_e94063_d_n4;
        locals.var_xmp_dn5 = assign60290_e94063_d_n5;
        locals.var_xmp_dn6 = assign60290_e94063_d_n6;
        locals.var_xmp_dn7 = assign60290_e94063_d_n7;
        locals.var_xmp_dn8 = assign60290_e94063_d_n8;
        locals.var_xmp_dn9 = assign60290_e94063_d_n9;
        locals.var_xmp_dn10 = assign60290_e94063_d_n10;
        locals.var_xmp_dn11 = assign60290_e94063_d_n11;
        locals.var_xmp_dn14 = assign60290_e94063_d_n14;

        let (assign60300_e94077,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60300_e94077;

        let (assign60310_e94091,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60310_e94091;

        let (assign60320_e94105, assign60320_e94105_d_n0, assign60320_e94105_d_n2, assign60320_e94105_d_n4, assign60320_e94105_d_n5, assign60320_e94105_d_n6, assign60320_e94105_d_n7, assign60320_e94105_d_n8, assign60320_e94105_d_n9, assign60320_e94105_d_n10, assign60320_e94105_d_n11, assign60320_e94105_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign60320_e94105;
        locals.var_arg_dn0 = assign60320_e94105_d_n0;
        locals.var_arg_dn2 = assign60320_e94105_d_n2;
        locals.var_arg_dn4 = assign60320_e94105_d_n4;
        locals.var_arg_dn5 = assign60320_e94105_d_n5;
        locals.var_arg_dn6 = assign60320_e94105_d_n6;
        locals.var_arg_dn7 = assign60320_e94105_d_n7;
        locals.var_arg_dn8 = assign60320_e94105_d_n8;
        locals.var_arg_dn9 = assign60320_e94105_d_n9;
        locals.var_arg_dn10 = assign60320_e94105_d_n10;
        locals.var_arg_dn11 = assign60320_e94105_d_n11;
        locals.var_arg_dn14 = assign60320_e94105_d_n14;

        let (assign60330_e94119, assign60330_e94119_d_n0, assign60330_e94119_d_n2, assign60330_e94119_d_n4, assign60330_e94119_d_n5, assign60330_e94119_d_n6, assign60330_e94119_d_n7, assign60330_e94119_d_n8, assign60330_e94119_d_n9, assign60330_e94119_d_n10, assign60330_e94119_d_n11, assign60330_e94119_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60330_e94119;
        locals.var_dnm_dn0 = assign60330_e94119_d_n0;
        locals.var_dnm_dn2 = assign60330_e94119_d_n2;
        locals.var_dnm_dn4 = assign60330_e94119_d_n4;
        locals.var_dnm_dn5 = assign60330_e94119_d_n5;
        locals.var_dnm_dn6 = assign60330_e94119_d_n6;
        locals.var_dnm_dn7 = assign60330_e94119_d_n7;
        locals.var_dnm_dn8 = assign60330_e94119_d_n8;
        locals.var_dnm_dn9 = assign60330_e94119_d_n9;
        locals.var_dnm_dn10 = assign60330_e94119_d_n10;
        locals.var_dnm_dn11 = assign60330_e94119_d_n11;
        locals.var_dnm_dn14 = assign60330_e94119_d_n14;

        let (assign60340_e94135, assign60340_e94135_d_n0, assign60340_e94135_d_n2, assign60340_e94135_d_n4, assign60340_e94135_d_n5, assign60340_e94135_d_n6, assign60340_e94135_d_n7, assign60340_e94135_d_n8, assign60340_e94135_d_n9, assign60340_e94135_d_n10, assign60340_e94135_d_n11, assign60340_e94135_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60340_e94133: f64 = (locals.var_xp * locals.var_x2);
        (assign60340_e94133, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60340_e94135;
        locals.var_xp_dn0 = assign60340_e94135_d_n0;
        locals.var_xp_dn2 = assign60340_e94135_d_n2;
        locals.var_xp_dn4 = assign60340_e94135_d_n4;
        locals.var_xp_dn5 = assign60340_e94135_d_n5;
        locals.var_xp_dn6 = assign60340_e94135_d_n6;
        locals.var_xp_dn7 = assign60340_e94135_d_n7;
        locals.var_xp_dn8 = assign60340_e94135_d_n8;
        locals.var_xp_dn9 = assign60340_e94135_d_n9;
        locals.var_xp_dn10 = assign60340_e94135_d_n10;
        locals.var_xp_dn11 = assign60340_e94135_d_n11;
        locals.var_xp_dn14 = assign60340_e94135_d_n14;

        let (assign60350_e94151, assign60350_e94151_d_n0, assign60350_e94151_d_n2, assign60350_e94151_d_n4, assign60350_e94151_d_n5, assign60350_e94151_d_n6, assign60350_e94151_d_n7, assign60350_e94151_d_n8, assign60350_e94151_d_n9, assign60350_e94151_d_n10, assign60350_e94151_d_n11, assign60350_e94151_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60350_e94149: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60350_e94149, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60350_e94151;
        locals.var_xmp_dn0 = assign60350_e94151_d_n0;
        locals.var_xmp_dn2 = assign60350_e94151_d_n2;
        locals.var_xmp_dn4 = assign60350_e94151_d_n4;
        locals.var_xmp_dn5 = assign60350_e94151_d_n5;
        locals.var_xmp_dn6 = assign60350_e94151_d_n6;
        locals.var_xmp_dn7 = assign60350_e94151_d_n7;
        locals.var_xmp_dn8 = assign60350_e94151_d_n8;
        locals.var_xmp_dn9 = assign60350_e94151_d_n9;
        locals.var_xmp_dn10 = assign60350_e94151_d_n10;
        locals.var_xmp_dn11 = assign60350_e94151_d_n11;
        locals.var_xmp_dn14 = assign60350_e94151_d_n14;

        let (assign60360_e94167, assign60360_e94167_d_n0, assign60360_e94167_d_n2, assign60360_e94167_d_n4, assign60360_e94167_d_n5, assign60360_e94167_d_n6, assign60360_e94167_d_n7, assign60360_e94167_d_n8, assign60360_e94167_d_n9, assign60360_e94167_d_n10, assign60360_e94167_d_n11, assign60360_e94167_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60360_e94165: f64 = (locals.var_xp * locals.var_x2);
        (assign60360_e94165, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60360_e94167;
        locals.var_xp_dn0 = assign60360_e94167_d_n0;
        locals.var_xp_dn2 = assign60360_e94167_d_n2;
        locals.var_xp_dn4 = assign60360_e94167_d_n4;
        locals.var_xp_dn5 = assign60360_e94167_d_n5;
        locals.var_xp_dn6 = assign60360_e94167_d_n6;
        locals.var_xp_dn7 = assign60360_e94167_d_n7;
        locals.var_xp_dn8 = assign60360_e94167_d_n8;
        locals.var_xp_dn9 = assign60360_e94167_d_n9;
        locals.var_xp_dn10 = assign60360_e94167_d_n10;
        locals.var_xp_dn11 = assign60360_e94167_d_n11;
        locals.var_xp_dn14 = assign60360_e94167_d_n14;

        let (assign60370_e94183, assign60370_e94183_d_n0, assign60370_e94183_d_n2, assign60370_e94183_d_n4, assign60370_e94183_d_n5, assign60370_e94183_d_n6, assign60370_e94183_d_n7, assign60370_e94183_d_n8, assign60370_e94183_d_n9, assign60370_e94183_d_n10, assign60370_e94183_d_n11, assign60370_e94183_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60370_e94181: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60370_e94181, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60370_e94183;
        locals.var_xmp_dn0 = assign60370_e94183_d_n0;
        locals.var_xmp_dn2 = assign60370_e94183_d_n2;
        locals.var_xmp_dn4 = assign60370_e94183_d_n4;
        locals.var_xmp_dn5 = assign60370_e94183_d_n5;
        locals.var_xmp_dn6 = assign60370_e94183_d_n6;
        locals.var_xmp_dn7 = assign60370_e94183_d_n7;
        locals.var_xmp_dn8 = assign60370_e94183_d_n8;
        locals.var_xmp_dn9 = assign60370_e94183_d_n9;
        locals.var_xmp_dn10 = assign60370_e94183_d_n10;
        locals.var_xmp_dn11 = assign60370_e94183_d_n11;
        locals.var_xmp_dn14 = assign60370_e94183_d_n14;

        let (assign60380_e94199, assign60380_e94199_d_n0, assign60380_e94199_d_n2, assign60380_e94199_d_n4, assign60380_e94199_d_n5, assign60380_e94199_d_n6, assign60380_e94199_d_n7, assign60380_e94199_d_n8, assign60380_e94199_d_n9, assign60380_e94199_d_n10, assign60380_e94199_d_n11, assign60380_e94199_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60380_e94197: f64 = (locals.var_xp + locals.var_xmp);
        (assign60380_e94197, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign60380_e94199;
        locals.var_arg_dn0 = assign60380_e94199_d_n0;
        locals.var_arg_dn2 = assign60380_e94199_d_n2;
        locals.var_arg_dn4 = assign60380_e94199_d_n4;
        locals.var_arg_dn5 = assign60380_e94199_d_n5;
        locals.var_arg_dn6 = assign60380_e94199_d_n6;
        locals.var_arg_dn7 = assign60380_e94199_d_n7;
        locals.var_arg_dn8 = assign60380_e94199_d_n8;
        locals.var_arg_dn9 = assign60380_e94199_d_n9;
        locals.var_arg_dn10 = assign60380_e94199_d_n10;
        locals.var_arg_dn11 = assign60380_e94199_d_n11;
        locals.var_arg_dn14 = assign60380_e94199_d_n14;

        let (assign60390_e94213, assign60390_e94213_d_n0, assign60390_e94213_d_n2, assign60390_e94213_d_n4, assign60390_e94213_d_n5, assign60390_e94213_d_n6, assign60390_e94213_d_n7, assign60390_e94213_d_n8, assign60390_e94213_d_n9, assign60390_e94213_d_n10, assign60390_e94213_d_n11, assign60390_e94213_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60390_e94213;
        locals.var_dnm_dn0 = assign60390_e94213_d_n0;
        locals.var_dnm_dn2 = assign60390_e94213_d_n2;
        locals.var_dnm_dn4 = assign60390_e94213_d_n4;
        locals.var_dnm_dn5 = assign60390_e94213_d_n5;
        locals.var_dnm_dn6 = assign60390_e94213_d_n6;
        locals.var_dnm_dn7 = assign60390_e94213_d_n7;
        locals.var_dnm_dn8 = assign60390_e94213_d_n8;
        locals.var_dnm_dn9 = assign60390_e94213_d_n9;
        locals.var_dnm_dn10 = assign60390_e94213_d_n10;
        locals.var_dnm_dn11 = assign60390_e94213_d_n11;
        locals.var_dnm_dn14 = assign60390_e94213_d_n14;

        let assign60400_e94228: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign60400_e94228;

        let assign60410_e94231: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign60410_e94231;

        let (assign60420_e94249,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) && (locals.var_guard1473 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60420_e94249;

        let assign60430_e94252: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign60430_e94252;

        let (assign60440_e94273,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60440_e94273;

        let assign60450_e94276: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign60450_e94276;

        let (assign60460_e94300,) = {
    if (((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 == 0.0)) && (locals.var_guard1475 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60460_e94300;

        let assign60470_e94303: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign60470_e94303;

        let (assign60480_e94330,) = {
    if ((((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 == 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60480_e94330;

        let (assign60490_e94346,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60490_e94346;

        let mut assign60500_loop_guard: usize = 0;
        while {
            let assign60500_cond_e94363: f64 = if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60500_cond_e94363 != 0.0
        } {
            assign60500_loop_guard += 1;
            assert!(assign60500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60500_body0_e94380, assign60500_body0_e94380_d_n0, assign60500_body0_e94380_d_n2, assign60500_body0_e94380_d_n4, assign60500_body0_e94380_d_n5, assign60500_body0_e94380_d_n6, assign60500_body0_e94380_d_n7, assign60500_body0_e94380_d_n8, assign60500_body0_e94380_d_n9, assign60500_body0_e94380_d_n10, assign60500_body0_e94380_d_n11, assign60500_body0_e94380_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign60500_body0_e94378: f64 = (locals.var_dnm).sqrt();
        (assign60500_body0_e94378, (locals.var_dnm_dn0 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn2 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn4 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn5 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn6 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn7 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn8 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn9 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn10 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn11 / (2.0 * assign60500_body0_e94378)), (locals.var_dnm_dn14 / (2.0 * assign60500_body0_e94378)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign60500_body0_e94380;
            locals.var_dnm_dn0 = assign60500_body0_e94380_d_n0;
            locals.var_dnm_dn2 = assign60500_body0_e94380_d_n2;
            locals.var_dnm_dn4 = assign60500_body0_e94380_d_n4;
            locals.var_dnm_dn5 = assign60500_body0_e94380_d_n5;
            locals.var_dnm_dn6 = assign60500_body0_e94380_d_n6;
            locals.var_dnm_dn7 = assign60500_body0_e94380_d_n7;
            locals.var_dnm_dn8 = assign60500_body0_e94380_d_n8;
            locals.var_dnm_dn9 = assign60500_body0_e94380_d_n9;
            locals.var_dnm_dn10 = assign60500_body0_e94380_d_n10;
            locals.var_dnm_dn11 = assign60500_body0_e94380_d_n11;
            locals.var_dnm_dn14 = assign60500_body0_e94380_d_n14;
            let (assign60500_body1_e94398,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 != 0.0)) {
        let assign60500_body1_e94396: f64 = (locals.var_m0 + 1.0);
        (assign60500_body1_e94396,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60500_body1_e94398;
        }

    }

    pub(super) fn stamp_transient_block_213(
        locals: &mut StampLocals,
    ) {
        let (assign60510_e94426, assign60510_e94426_d_n0, assign60510_e94426_d_n2, assign60510_e94426_d_n4, assign60510_e94426_d_n5, assign60510_e94426_d_n6, assign60510_e94426_d_n7, assign60510_e94426_d_n8, assign60510_e94426_d_n9, assign60510_e94426_d_n10, assign60510_e94426_d_n11, assign60510_e94426_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) && (locals.var_guard1472 == 0.0)) {
        let (assign60510_e94424, assign60510_e94424_d_n0, assign60510_e94424_d_n2, assign60510_e94424_d_n4, assign60510_e94424_d_n5, assign60510_e94424_d_n6, assign60510_e94424_d_n7, assign60510_e94424_d_n8, assign60510_e94424_d_n9, assign60510_e94424_d_n10, assign60510_e94424_d_n11, assign60510_e94424_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60510_e94421: f64 = (2.0 * 2.0);
                let assign60510_e94422: f64 = (1.0 / assign60510_e94421);
                let assign60510_e94423: f64 = (locals.var_dnm).powf(assign60510_e94422);
                (assign60510_e94423, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn11)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60510_e94422) as f64).is_finite() && ((assign60510_e94422) as f64).fract() == 0.0 { if assign60510_e94422 == 0.0 { 0.0 } else { (assign60510_e94422 * ((locals.var_dnm).powf(assign60510_e94422 - 1.0) * locals.var_dnm_dn14)) } } else { (assign60510_e94423 * (assign60510_e94422 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign60510_e94424, assign60510_e94424_d_n0, assign60510_e94424_d_n2, assign60510_e94424_d_n4, assign60510_e94424_d_n5, assign60510_e94424_d_n6, assign60510_e94424_d_n7, assign60510_e94424_d_n8, assign60510_e94424_d_n9, assign60510_e94424_d_n10, assign60510_e94424_d_n11, assign60510_e94424_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60510_e94426;
        locals.var_dnm_dn0 = assign60510_e94426_d_n0;
        locals.var_dnm_dn2 = assign60510_e94426_d_n2;
        locals.var_dnm_dn4 = assign60510_e94426_d_n4;
        locals.var_dnm_dn5 = assign60510_e94426_d_n5;
        locals.var_dnm_dn6 = assign60510_e94426_d_n6;
        locals.var_dnm_dn7 = assign60510_e94426_d_n7;
        locals.var_dnm_dn8 = assign60510_e94426_d_n8;
        locals.var_dnm_dn9 = assign60510_e94426_d_n9;
        locals.var_dnm_dn10 = assign60510_e94426_d_n10;
        locals.var_dnm_dn11 = assign60510_e94426_d_n11;
        locals.var_dnm_dn14 = assign60510_e94426_d_n14;

        let (assign60520_e94442, assign60520_e94442_d_n0, assign60520_e94442_d_n2, assign60520_e94442_d_n4, assign60520_e94442_d_n5, assign60520_e94442_d_n6, assign60520_e94442_d_n7, assign60520_e94442_d_n8, assign60520_e94442_d_n9, assign60520_e94442_d_n10, assign60520_e94442_d_n11, assign60520_e94442_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60520_e94440: f64 = (1.0 / locals.var_dnm);
        (assign60520_e94440, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60520_e94442;
        locals.var_dnm_dn0 = assign60520_e94442_d_n0;
        locals.var_dnm_dn2 = assign60520_e94442_d_n2;
        locals.var_dnm_dn4 = assign60520_e94442_d_n4;
        locals.var_dnm_dn5 = assign60520_e94442_d_n5;
        locals.var_dnm_dn6 = assign60520_e94442_d_n6;
        locals.var_dnm_dn7 = assign60520_e94442_d_n7;
        locals.var_dnm_dn8 = assign60520_e94442_d_n8;
        locals.var_dnm_dn9 = assign60520_e94442_d_n9;
        locals.var_dnm_dn10 = assign60520_e94442_d_n10;
        locals.var_dnm_dn11 = assign60520_e94442_d_n11;
        locals.var_dnm_dn14 = assign60520_e94442_d_n14;

        let (assign60530_e94462, assign60530_e94462_d_n0, assign60530_e94462_d_n2, assign60530_e94462_d_n4, assign60530_e94462_d_n5, assign60530_e94462_d_n6, assign60530_e94462_d_n7, assign60530_e94462_d_n8, assign60530_e94462_d_n9, assign60530_e94462_d_n10, assign60530_e94462_d_n11, assign60530_e94462_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60530_e94457: f64 = (10.0 * 2.220446049250313e-16);
        let assign60530_e94458: f64 = (locals.var_tmf1 * assign60530_e94457);
        let assign60530_e94460: f64 = (assign60530_e94458 * locals.var_dnm);
        (assign60530_e94460, (((locals.var_tmf1_dn0 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign60530_e94457) * locals.var_dnm) + (assign60530_e94458 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign60530_e94462;
        locals.var_tmf0_dn0 = assign60530_e94462_d_n0;
        locals.var_tmf0_dn2 = assign60530_e94462_d_n2;
        locals.var_tmf0_dn4 = assign60530_e94462_d_n4;
        locals.var_tmf0_dn5 = assign60530_e94462_d_n5;
        locals.var_tmf0_dn6 = assign60530_e94462_d_n6;
        locals.var_tmf0_dn7 = assign60530_e94462_d_n7;
        locals.var_tmf0_dn8 = assign60530_e94462_d_n8;
        locals.var_tmf0_dn9 = assign60530_e94462_d_n9;
        locals.var_tmf0_dn10 = assign60530_e94462_d_n10;
        locals.var_tmf0_dn11 = assign60530_e94462_d_n11;
        locals.var_tmf0_dn14 = assign60530_e94462_d_n14;

        let (assign60540_e94484, assign60540_e94484_d_n0, assign60540_e94484_d_n2, assign60540_e94484_d_n4, assign60540_e94484_d_n5, assign60540_e94484_d_n6, assign60540_e94484_d_n7, assign60540_e94484_d_n8, assign60540_e94484_d_n9, assign60540_e94484_d_n10, assign60540_e94484_d_n11, assign60540_e94484_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60540_e94476: f64 = (10.0 * 2.220446049250313e-16);
        let assign60540_e94478: f64 = (assign60540_e94476 * locals.var_xmp);
        let assign60540_e94480: f64 = (assign60540_e94478 * locals.var_dnm);
        let assign60540_e94482: f64 = (assign60540_e94480 / locals.var_arg);
        (assign60540_e94482, ((((((assign60540_e94476 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn11) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn11)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign60540_e94476 * locals.var_xmp_dn14) * locals.var_dnm) + (assign60540_e94478 * locals.var_dnm_dn14)) * locals.var_arg) - (assign60540_e94480 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60540_e94484;
        locals.var_t0_dn0 = assign60540_e94484_d_n0;
        locals.var_t0_dn2 = assign60540_e94484_d_n2;
        locals.var_t0_dn4 = assign60540_e94484_d_n4;
        locals.var_t0_dn5 = assign60540_e94484_d_n5;
        locals.var_t0_dn6 = assign60540_e94484_d_n6;
        locals.var_t0_dn7 = assign60540_e94484_d_n7;
        locals.var_t0_dn8 = assign60540_e94484_d_n8;
        locals.var_t0_dn9 = assign60540_e94484_d_n9;
        locals.var_t0_dn10 = assign60540_e94484_d_n10;
        locals.var_t0_dn11 = assign60540_e94484_d_n11;
        locals.var_t0_dn14 = assign60540_e94484_d_n14;

        let (assign60550_e94510, assign60550_e94510_d_n0, assign60550_e94510_d_n2, assign60550_e94510_d_n4, assign60550_e94510_d_n5, assign60550_e94510_d_n6, assign60550_e94510_d_n7, assign60550_e94510_d_n8, assign60550_e94510_d_n9, assign60550_e94510_d_n10, assign60550_e94510_d_n11, assign60550_e94510_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        let assign60550_e94498: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60550_e94501: f64 = (10.0 * 2.220446049250313e-16);
        let assign60550_e94502: f64 = (assign60550_e94498 - assign60550_e94501);
        let assign60550_e94505: f64 = (10.0 * 2.220446049250313e-16);
        let assign60550_e94506: f64 = (assign60550_e94502 - assign60550_e94505);
        let assign60550_e94508: f64 = (assign60550_e94506 + locals.var_tmf0);
        (assign60550_e94508, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60550_e94510;
        locals.var_psdl_dn0 = assign60550_e94510_d_n0;
        locals.var_psdl_dn2 = assign60550_e94510_d_n2;
        locals.var_psdl_dn4 = assign60550_e94510_d_n4;
        locals.var_psdl_dn5 = assign60550_e94510_d_n5;
        locals.var_psdl_dn6 = assign60550_e94510_d_n6;
        locals.var_psdl_dn7 = assign60550_e94510_d_n7;
        locals.var_psdl_dn8 = assign60550_e94510_d_n8;
        locals.var_psdl_dn9 = assign60550_e94510_d_n9;
        locals.var_psdl_dn10 = assign60550_e94510_d_n10;
        locals.var_psdl_dn11 = assign60550_e94510_d_n11;
        locals.var_psdl_dn14 = assign60550_e94510_d_n14;

        let (assign60560_e94524, assign60560_e94524_d_n0, assign60560_e94524_d_n2, assign60560_e94524_d_n4, assign60560_e94524_d_n5, assign60560_e94524_d_n6, assign60560_e94524_d_n7, assign60560_e94524_d_n8, assign60560_e94524_d_n9, assign60560_e94524_d_n10, assign60560_e94524_d_n11, assign60560_e94524_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60560_e94524;
        locals.var_t0_dn0 = assign60560_e94524_d_n0;
        locals.var_t0_dn2 = assign60560_e94524_d_n2;
        locals.var_t0_dn4 = assign60560_e94524_d_n4;
        locals.var_t0_dn5 = assign60560_e94524_d_n5;
        locals.var_t0_dn6 = assign60560_e94524_d_n6;
        locals.var_t0_dn7 = assign60560_e94524_d_n7;
        locals.var_t0_dn8 = assign60560_e94524_d_n8;
        locals.var_t0_dn9 = assign60560_e94524_d_n9;
        locals.var_t0_dn10 = assign60560_e94524_d_n10;
        locals.var_t0_dn11 = assign60560_e94524_d_n11;
        locals.var_t0_dn14 = assign60560_e94524_d_n14;

        let (assign60570_e94539, assign60570_e94539_d_n0, assign60570_e94539_d_n2, assign60570_e94539_d_n4, assign60570_e94539_d_n5, assign60570_e94539_d_n6, assign60570_e94539_d_n7, assign60570_e94539_d_n8, assign60570_e94539_d_n9, assign60570_e94539_d_n10, assign60570_e94539_d_n11, assign60570_e94539_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60570_e94539;
        locals.var_psdl_dn0 = assign60570_e94539_d_n0;
        locals.var_psdl_dn2 = assign60570_e94539_d_n2;
        locals.var_psdl_dn4 = assign60570_e94539_d_n4;
        locals.var_psdl_dn5 = assign60570_e94539_d_n5;
        locals.var_psdl_dn6 = assign60570_e94539_d_n6;
        locals.var_psdl_dn7 = assign60570_e94539_d_n7;
        locals.var_psdl_dn8 = assign60570_e94539_d_n8;
        locals.var_psdl_dn9 = assign60570_e94539_d_n9;
        locals.var_psdl_dn10 = assign60570_e94539_d_n10;
        locals.var_psdl_dn11 = assign60570_e94539_d_n11;
        locals.var_psdl_dn14 = assign60570_e94539_d_n14;

        let (assign60580_e94554, assign60580_e94554_d_n0, assign60580_e94554_d_n2, assign60580_e94554_d_n4, assign60580_e94554_d_n5, assign60580_e94554_d_n6, assign60580_e94554_d_n7, assign60580_e94554_d_n8, assign60580_e94554_d_n9, assign60580_e94554_d_n10, assign60580_e94554_d_n11, assign60580_e94554_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1471 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60580_e94554;
        locals.var_t0_dn0 = assign60580_e94554_d_n0;
        locals.var_t0_dn2 = assign60580_e94554_d_n2;
        locals.var_t0_dn4 = assign60580_e94554_d_n4;
        locals.var_t0_dn5 = assign60580_e94554_d_n5;
        locals.var_t0_dn6 = assign60580_e94554_d_n6;
        locals.var_t0_dn7 = assign60580_e94554_d_n7;
        locals.var_t0_dn8 = assign60580_e94554_d_n8;
        locals.var_t0_dn9 = assign60580_e94554_d_n9;
        locals.var_t0_dn10 = assign60580_e94554_d_n10;
        locals.var_t0_dn11 = assign60580_e94554_d_n11;
        locals.var_t0_dn14 = assign60580_e94554_d_n14;

        let (assign60590_e94568, assign60590_e94568_d_n0, assign60590_e94568_d_n2, assign60590_e94568_d_n4, assign60590_e94568_d_n5, assign60590_e94568_d_n6, assign60590_e94568_d_n7, assign60590_e94568_d_n8, assign60590_e94568_d_n9, assign60590_e94568_d_n10, assign60590_e94568_d_n11, assign60590_e94568_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60590_e94566: f64 = (locals.var_psdl - locals.var_psl);
        (assign60590_e94566, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn4 - locals.var_psl_dn4), (locals.var_psdl_dn5 - locals.var_psl_dn5), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn8 - locals.var_psl_dn8), (locals.var_psdl_dn9 - locals.var_psl_dn9), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign60590_e94568;
        locals.var_t6_dn0 = assign60590_e94568_d_n0;
        locals.var_t6_dn2 = assign60590_e94568_d_n2;
        locals.var_t6_dn4 = assign60590_e94568_d_n4;
        locals.var_t6_dn5 = assign60590_e94568_d_n5;
        locals.var_t6_dn6 = assign60590_e94568_d_n6;
        locals.var_t6_dn7 = assign60590_e94568_d_n7;
        locals.var_t6_dn8 = assign60590_e94568_d_n8;
        locals.var_t6_dn9 = assign60590_e94568_d_n9;
        locals.var_t6_dn10 = assign60590_e94568_d_n10;
        locals.var_t6_dn11 = assign60590_e94568_d_n11;
        locals.var_t6_dn14 = assign60590_e94568_d_n14;

        let (assign60600_e94582, assign60600_e94582_d_n0, assign60600_e94582_d_n2, assign60600_e94582_d_n4, assign60600_e94582_d_n5, assign60600_e94582_d_n6, assign60600_e94582_d_n7, assign60600_e94582_d_n8, assign60600_e94582_d_n9, assign60600_e94582_d_n10, assign60600_e94582_d_n11, assign60600_e94582_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60600_e94580: f64 = (locals.var_beta * locals.var_qn0);
        (assign60600_e94580, ((locals.var_beta_dn0 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn0)), ((locals.var_beta_dn2 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn2)), ((locals.var_beta_dn4 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn4)), ((locals.var_beta_dn5 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn5)), ((locals.var_beta_dn6 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn6)), ((locals.var_beta_dn7 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn7)), ((locals.var_beta_dn8 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn8)), ((locals.var_beta_dn9 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn9)), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), ((locals.var_beta_dn11 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn11)), ((locals.var_beta_dn14 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60600_e94582;
        locals.var_t3_dn0 = assign60600_e94582_d_n0;
        locals.var_t3_dn2 = assign60600_e94582_d_n2;
        locals.var_t3_dn4 = assign60600_e94582_d_n4;
        locals.var_t3_dn5 = assign60600_e94582_d_n5;
        locals.var_t3_dn6 = assign60600_e94582_d_n6;
        locals.var_t3_dn7 = assign60600_e94582_d_n7;
        locals.var_t3_dn8 = assign60600_e94582_d_n8;
        locals.var_t3_dn9 = assign60600_e94582_d_n9;
        locals.var_t3_dn10 = assign60600_e94582_d_n10;
        locals.var_t3_dn11 = assign60600_e94582_d_n11;
        locals.var_t3_dn14 = assign60600_e94582_d_n14;

        let (assign60610_e94596, assign60610_e94596_d_n0, assign60610_e94596_d_n2, assign60610_e94596_d_n4, assign60610_e94596_d_n5, assign60610_e94596_d_n6, assign60610_e94596_d_n7, assign60610_e94596_d_n8, assign60610_e94596_d_n9, assign60610_e94596_d_n10, assign60610_e94596_d_n11, assign60610_e94596_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60610_e94594: f64 = (1.0 / locals.var_t3);
        (assign60610_e94594, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60610_e94596;
        locals.var_t1_dn0 = assign60610_e94596_d_n0;
        locals.var_t1_dn2 = assign60610_e94596_d_n2;
        locals.var_t1_dn4 = assign60610_e94596_d_n4;
        locals.var_t1_dn5 = assign60610_e94596_d_n5;
        locals.var_t1_dn6 = assign60610_e94596_d_n6;
        locals.var_t1_dn7 = assign60610_e94596_d_n7;
        locals.var_t1_dn8 = assign60610_e94596_d_n8;
        locals.var_t1_dn9 = assign60610_e94596_d_n9;
        locals.var_t1_dn10 = assign60610_e94596_d_n10;
        locals.var_t1_dn11 = assign60610_e94596_d_n11;
        locals.var_t1_dn14 = assign60610_e94596_d_n14;

        let (assign60620_e94616, assign60620_e94616_d_n0, assign60620_e94616_d_n2, assign60620_e94616_d_n4, assign60620_e94616_d_n5, assign60620_e94616_d_n6, assign60620_e94616_d_n7, assign60620_e94616_d_n8, assign60620_e94616_d_n9, assign60620_e94616_d_n10, assign60620_e94616_d_n11, assign60620_e94616_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60620_e94609: f64 = (10.0 * 2.220446049250313e-16);
        let assign60620_e94610: f64 = (locals.var_pds + assign60620_e94609);
        let assign60620_e94612: f64 = (assign60620_e94610 * locals.var_fdd);
        let assign60620_e94614: f64 = (assign60620_e94612 * locals.var_t1);
        (assign60620_e94614, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn0)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn2)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn4)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn5)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn6)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn7)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn8)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn9)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn10)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn10)), ((((locals.var_pds_dn11 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn11)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn11)), ((((locals.var_pds_dn14 * locals.var_fdd) + (assign60620_e94610 * locals.var_fdd_dn14)) * locals.var_t1) + (assign60620_e94612 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign60620_e94616;
        locals.var_t5_dn0 = assign60620_e94616_d_n0;
        locals.var_t5_dn2 = assign60620_e94616_d_n2;
        locals.var_t5_dn4 = assign60620_e94616_d_n4;
        locals.var_t5_dn5 = assign60620_e94616_d_n5;
        locals.var_t5_dn6 = assign60620_e94616_d_n6;
        locals.var_t5_dn7 = assign60620_e94616_d_n7;
        locals.var_t5_dn8 = assign60620_e94616_d_n8;
        locals.var_t5_dn9 = assign60620_e94616_d_n9;
        locals.var_t5_dn10 = assign60620_e94616_d_n10;
        locals.var_t5_dn11 = assign60620_e94616_d_n11;
        locals.var_t5_dn14 = assign60620_e94616_d_n14;

        let (assign60630_e94630, assign60630_e94630_d_n0, assign60630_e94630_d_n2, assign60630_e94630_d_n4, assign60630_e94630_d_n5, assign60630_e94630_d_n6, assign60630_e94630_d_n7, assign60630_e94630_d_n8, assign60630_e94630_d_n9, assign60630_e94630_d_n10, assign60630_e94630_d_n11, assign60630_e94630_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60630_e94628: f64 = (locals.var_t5 * locals.var_beta);
        (assign60630_e94628, ((locals.var_t5_dn0 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn0)), ((locals.var_t5_dn2 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn2)), ((locals.var_t5_dn4 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn4)), ((locals.var_t5_dn5 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn5)), ((locals.var_t5_dn6 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn6)), ((locals.var_t5_dn7 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn7)), ((locals.var_t5_dn8 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn8)), ((locals.var_t5_dn9 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn9)), ((locals.var_t5_dn10 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn10)), ((locals.var_t5_dn11 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn11)), ((locals.var_t5_dn14 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60630_e94630;
        locals.var_t2_dn0 = assign60630_e94630_d_n0;
        locals.var_t2_dn2 = assign60630_e94630_d_n2;
        locals.var_t2_dn4 = assign60630_e94630_d_n4;
        locals.var_t2_dn5 = assign60630_e94630_d_n5;
        locals.var_t2_dn6 = assign60630_e94630_d_n6;
        locals.var_t2_dn7 = assign60630_e94630_d_n7;
        locals.var_t2_dn8 = assign60630_e94630_d_n8;
        locals.var_t2_dn9 = assign60630_e94630_d_n9;
        locals.var_t2_dn10 = assign60630_e94630_d_n10;
        locals.var_t2_dn11 = assign60630_e94630_d_n11;
        locals.var_t2_dn14 = assign60630_e94630_d_n14;

        let (assign60640_e94644, assign60640_e94644_d_n0, assign60640_e94644_d_n2, assign60640_e94644_d_n4, assign60640_e94644_d_n5, assign60640_e94644_d_n6, assign60640_e94644_d_n7, assign60640_e94644_d_n8, assign60640_e94644_d_n9, assign60640_e94644_d_n10, assign60640_e94644_d_n11, assign60640_e94644_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60640_e94642: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign60640_e94642, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn4 / 1.034943e-10), (locals.var_q_nsub_dn5 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn8 / 1.034943e-10), (locals.var_q_nsub_dn9 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn14 / 1.034943e-10),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign60640_e94644;
        locals.var_t10_dn0 = assign60640_e94644_d_n0;
        locals.var_t10_dn2 = assign60640_e94644_d_n2;
        locals.var_t10_dn4 = assign60640_e94644_d_n4;
        locals.var_t10_dn5 = assign60640_e94644_d_n5;
        locals.var_t10_dn6 = assign60640_e94644_d_n6;
        locals.var_t10_dn7 = assign60640_e94644_d_n7;
        locals.var_t10_dn8 = assign60640_e94644_d_n8;
        locals.var_t10_dn9 = assign60640_e94644_d_n9;
        locals.var_t10_dn10 = assign60640_e94644_d_n10;
        locals.var_t10_dn11 = assign60640_e94644_d_n11;
        locals.var_t10_dn14 = assign60640_e94644_d_n14;

        let (assign60650_e94656, assign60650_e94656_d_n0, assign60650_e94656_d_n2, assign60650_e94656_d_n4, assign60650_e94656_d_n5, assign60650_e94656_d_n6, assign60650_e94656_d_n7, assign60650_e94656_d_n8, assign60650_e94656_d_n9, assign60650_e94656_d_n10, assign60650_e94656_d_n11, assign60650_e94656_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60650_e94656;
        locals.var_t1_dn0 = assign60650_e94656_d_n0;
        locals.var_t1_dn2 = assign60650_e94656_d_n2;
        locals.var_t1_dn4 = assign60650_e94656_d_n4;
        locals.var_t1_dn5 = assign60650_e94656_d_n5;
        locals.var_t1_dn6 = assign60650_e94656_d_n6;
        locals.var_t1_dn7 = assign60650_e94656_d_n7;
        locals.var_t1_dn8 = assign60650_e94656_d_n8;
        locals.var_t1_dn9 = assign60650_e94656_d_n9;
        locals.var_t1_dn10 = assign60650_e94656_d_n10;
        locals.var_t1_dn11 = assign60650_e94656_d_n11;
        locals.var_t1_dn14 = assign60650_e94656_d_n14;

        let (assign60660_e94670, assign60660_e94670_d_n0, assign60660_e94670_d_n2, assign60660_e94670_d_n4, assign60660_e94670_d_n5, assign60660_e94670_d_n6, assign60660_e94670_d_n7, assign60660_e94670_d_n8, assign60660_e94670_d_n9, assign60660_e94670_d_n10, assign60660_e94670_d_n11, assign60660_e94670_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60660_e94668: f64 = (1.0 / locals.var_leff);
        (assign60660_e94668, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60660_e94670;
        locals.var_t2_dn0 = assign60660_e94670_d_n0;
        locals.var_t2_dn2 = assign60660_e94670_d_n2;
        locals.var_t2_dn4 = assign60660_e94670_d_n4;
        locals.var_t2_dn5 = assign60660_e94670_d_n5;
        locals.var_t2_dn6 = assign60660_e94670_d_n6;
        locals.var_t2_dn7 = assign60660_e94670_d_n7;
        locals.var_t2_dn8 = assign60660_e94670_d_n8;
        locals.var_t2_dn9 = assign60660_e94670_d_n9;
        locals.var_t2_dn10 = assign60660_e94670_d_n10;
        locals.var_t2_dn11 = assign60660_e94670_d_n11;
        locals.var_t2_dn14 = assign60660_e94670_d_n14;

        let (assign60670_e94698, assign60670_e94698_d_n0, assign60670_e94698_d_n2, assign60670_e94698_d_n4, assign60670_e94698_d_n5, assign60670_e94698_d_n6, assign60670_e94698_d_n7, assign60670_e94698_d_n8, assign60670_e94698_d_n9, assign60670_e94698_d_n10, assign60670_e94698_d_n11, assign60670_e94698_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60670_e94682: f64 = (2.0 * locals.var_t5);
        let assign60670_e94685: f64 = (2.0 * locals.var_t10);
        let assign60670_e94687: f64 = (assign60670_e94685 * locals.var_t6);
        let assign60670_e94689: f64 = (assign60670_e94687 * locals.var_t4);
        let assign60670_e94690: f64 = (assign60670_e94682 + assign60670_e94689);
        let assign60670_e94693: f64 = (locals.var_t1 * locals.var_t4);
        let assign60670_e94694: f64 = (assign60670_e94690 + assign60670_e94693);
        let assign60670_e94696: f64 = (assign60670_e94694 * locals.var_t2);
        (assign60670_e94696, (((((2.0 * locals.var_t5_dn0) + (((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn0)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn0))) + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn0)), (((((2.0 * locals.var_t5_dn2) + (((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn2)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn2))) + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn2)), (((((2.0 * locals.var_t5_dn4) + (((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn4)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn4))) + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn4)), (((((2.0 * locals.var_t5_dn5) + (((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn5)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn5))) + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn5)), (((((2.0 * locals.var_t5_dn6) + (((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn6)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn6))) + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn6)), (((((2.0 * locals.var_t5_dn7) + (((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn7)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn7))) + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn7)), (((((2.0 * locals.var_t5_dn8) + (((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn8)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn8))) + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn8)), (((((2.0 * locals.var_t5_dn9) + (((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn9)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn9))) + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn9)), (((((2.0 * locals.var_t5_dn10) + (((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn10)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn10))) + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn10)), (((((2.0 * locals.var_t5_dn11) + (((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn11)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn11))) + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn11)), (((((2.0 * locals.var_t5_dn14) + (((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign60670_e94685 * locals.var_t6_dn14)) * locals.var_t4) + (assign60670_e94687 * locals.var_t4_dn14))) + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))) * locals.var_t2) + (assign60670_e94694 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign60670_e94698;
        locals.var_t11_dn0 = assign60670_e94698_d_n0;
        locals.var_t11_dn2 = assign60670_e94698_d_n2;
        locals.var_t11_dn4 = assign60670_e94698_d_n4;
        locals.var_t11_dn5 = assign60670_e94698_d_n5;
        locals.var_t11_dn6 = assign60670_e94698_d_n6;
        locals.var_t11_dn7 = assign60670_e94698_d_n7;
        locals.var_t11_dn8 = assign60670_e94698_d_n8;
        locals.var_t11_dn9 = assign60670_e94698_d_n9;
        locals.var_t11_dn10 = assign60670_e94698_d_n10;
        locals.var_t11_dn11 = assign60670_e94698_d_n11;
        locals.var_t11_dn14 = assign60670_e94698_d_n14;

        let (assign60680_e94712, assign60680_e94712_d_n0, assign60680_e94712_d_n2, assign60680_e94712_d_n4, assign60680_e94712_d_n5, assign60680_e94712_d_n6, assign60680_e94712_d_n7, assign60680_e94712_d_n8, assign60680_e94712_d_n9, assign60680_e94712_d_n10, assign60680_e94712_d_n11, assign60680_e94712_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60680_e94710: f64 = (locals.var_t2 * locals.var_t4);
        (assign60680_e94710, ((locals.var_t2_dn0 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn0)), ((locals.var_t2_dn2 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn2)), ((locals.var_t2_dn4 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn4)), ((locals.var_t2_dn5 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn5)), ((locals.var_t2_dn6 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn6)), ((locals.var_t2_dn7 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn7)), ((locals.var_t2_dn8 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn8)), ((locals.var_t2_dn9 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn9)), ((locals.var_t2_dn10 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn10)), ((locals.var_t2_dn11 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn11)), ((locals.var_t2_dn14 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60680_e94712;
        locals.var_t3_dn0 = assign60680_e94712_d_n0;
        locals.var_t3_dn2 = assign60680_e94712_d_n2;
        locals.var_t3_dn4 = assign60680_e94712_d_n4;
        locals.var_t3_dn5 = assign60680_e94712_d_n5;
        locals.var_t3_dn6 = assign60680_e94712_d_n6;
        locals.var_t3_dn7 = assign60680_e94712_d_n7;
        locals.var_t3_dn8 = assign60680_e94712_d_n8;
        locals.var_t3_dn9 = assign60680_e94712_d_n9;
        locals.var_t3_dn10 = assign60680_e94712_d_n10;
        locals.var_t3_dn11 = assign60680_e94712_d_n11;
        locals.var_t3_dn14 = assign60680_e94712_d_n14;

        let (assign60690_e94726, assign60690_e94726_d_n0, assign60690_e94726_d_n2, assign60690_e94726_d_n4, assign60690_e94726_d_n5, assign60690_e94726_d_n6, assign60690_e94726_d_n7, assign60690_e94726_d_n8, assign60690_e94726_d_n9, assign60690_e94726_d_n10, assign60690_e94726_d_n11, assign60690_e94726_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60690_e94724: f64 = (locals.var_t11 * locals.var_t4);
        (assign60690_e94724, ((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)), ((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)), ((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)), ((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)), ((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)), ((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)), ((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)), ((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)), ((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)), ((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)), ((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign60690_e94726;
        locals.var_t7_dn0 = assign60690_e94726_d_n0;
        locals.var_t7_dn2 = assign60690_e94726_d_n2;
        locals.var_t7_dn4 = assign60690_e94726_d_n4;
        locals.var_t7_dn5 = assign60690_e94726_d_n5;
        locals.var_t7_dn6 = assign60690_e94726_d_n6;
        locals.var_t7_dn7 = assign60690_e94726_d_n7;
        locals.var_t7_dn8 = assign60690_e94726_d_n8;
        locals.var_t7_dn9 = assign60690_e94726_d_n9;
        locals.var_t7_dn10 = assign60690_e94726_d_n10;
        locals.var_t7_dn11 = assign60690_e94726_d_n11;
        locals.var_t7_dn14 = assign60690_e94726_d_n14;

        let (assign60700_e94746, assign60700_e94746_d_n0, assign60700_e94746_d_n2, assign60700_e94746_d_n4, assign60700_e94746_d_n5, assign60700_e94746_d_n6, assign60700_e94746_d_n7, assign60700_e94746_d_n8, assign60700_e94746_d_n9, assign60700_e94746_d_n10, assign60700_e94746_d_n11, assign60700_e94746_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60700_e94739: f64 = (2.0 * locals.var_t10);
        let assign60700_e94741: f64 = (assign60700_e94739 * locals.var_t6);
        let assign60700_e94743: f64 = (assign60700_e94741 + locals.var_t1);
        let assign60700_e94744: f64 = (4.0 * assign60700_e94743);
        (assign60700_e94744, (4.0 * ((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn0)) + locals.var_t1_dn0)), (4.0 * ((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn2)) + locals.var_t1_dn2)), (4.0 * ((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn4)) + locals.var_t1_dn4)), (4.0 * ((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn5)) + locals.var_t1_dn5)), (4.0 * ((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn6)) + locals.var_t1_dn6)), (4.0 * ((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn7)) + locals.var_t1_dn7)), (4.0 * ((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn8)) + locals.var_t1_dn8)), (4.0 * ((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn9)) + locals.var_t1_dn9)), (4.0 * ((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn10)) + locals.var_t1_dn10)), (4.0 * ((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn11)) + locals.var_t1_dn11)), (4.0 * ((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign60700_e94739 * locals.var_t6_dn14)) + locals.var_t1_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign60700_e94746;
        locals.var_t11_dn0 = assign60700_e94746_d_n0;
        locals.var_t11_dn2 = assign60700_e94746_d_n2;
        locals.var_t11_dn4 = assign60700_e94746_d_n4;
        locals.var_t11_dn5 = assign60700_e94746_d_n5;
        locals.var_t11_dn6 = assign60700_e94746_d_n6;
        locals.var_t11_dn7 = assign60700_e94746_d_n7;
        locals.var_t11_dn8 = assign60700_e94746_d_n8;
        locals.var_t11_dn9 = assign60700_e94746_d_n9;
        locals.var_t11_dn10 = assign60700_e94746_d_n10;
        locals.var_t11_dn11 = assign60700_e94746_d_n11;
        locals.var_t11_dn14 = assign60700_e94746_d_n14;

        let (assign60710_e94764, assign60710_e94764_d_n0, assign60710_e94764_d_n2, assign60710_e94764_d_n4, assign60710_e94764_d_n5, assign60710_e94764_d_n6, assign60710_e94764_d_n7, assign60710_e94764_d_n8, assign60710_e94764_d_n9, assign60710_e94764_d_n10, assign60710_e94764_d_n11, assign60710_e94764_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60710_e94758: f64 = (8.0 * locals.var_t10);
        let assign60710_e94760: f64 = (assign60710_e94758 * locals.var_t4);
        let assign60710_e94762: f64 = (assign60710_e94760 * locals.var_t4);
        (assign60710_e94762, (((((8.0 * locals.var_t10_dn0) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn0)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn0)), (((((8.0 * locals.var_t10_dn2) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn2)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn2)), (((((8.0 * locals.var_t10_dn4) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn4)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn4)), (((((8.0 * locals.var_t10_dn5) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn5)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn5)), (((((8.0 * locals.var_t10_dn6) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn6)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn6)), (((((8.0 * locals.var_t10_dn7) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn7)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn7)), (((((8.0 * locals.var_t10_dn8) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn8)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn8)), (((((8.0 * locals.var_t10_dn9) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn9)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn9)), (((((8.0 * locals.var_t10_dn10) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn10)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn10)), (((((8.0 * locals.var_t10_dn11) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn11)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn11)), (((((8.0 * locals.var_t10_dn14) * locals.var_t4) + (assign60710_e94758 * locals.var_t4_dn14)) * locals.var_t4) + (assign60710_e94760 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60710_e94764;
        locals.var_t1_dn0 = assign60710_e94764_d_n0;
        locals.var_t1_dn2 = assign60710_e94764_d_n2;
        locals.var_t1_dn4 = assign60710_e94764_d_n4;
        locals.var_t1_dn5 = assign60710_e94764_d_n5;
        locals.var_t1_dn6 = assign60710_e94764_d_n6;
        locals.var_t1_dn7 = assign60710_e94764_d_n7;
        locals.var_t1_dn8 = assign60710_e94764_d_n8;
        locals.var_t1_dn9 = assign60710_e94764_d_n9;
        locals.var_t1_dn10 = assign60710_e94764_d_n10;
        locals.var_t1_dn11 = assign60710_e94764_d_n11;
        locals.var_t1_dn14 = assign60710_e94764_d_n14;

        let (assign60720_e94780, assign60720_e94780_d_n0, assign60720_e94780_d_n2, assign60720_e94780_d_n4, assign60720_e94780_d_n5, assign60720_e94780_d_n6, assign60720_e94780_d_n7, assign60720_e94780_d_n8, assign60720_e94780_d_n9, assign60720_e94780_d_n10, assign60720_e94780_d_n11, assign60720_e94780_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60720_e94776: f64 = (2.0 * locals.var_t11);
        let assign60720_e94778: f64 = (assign60720_e94776 * locals.var_t4);
        (assign60720_e94778, (((2.0 * locals.var_t11_dn0) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn0)), (((2.0 * locals.var_t11_dn2) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn2)), (((2.0 * locals.var_t11_dn4) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn4)), (((2.0 * locals.var_t11_dn5) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn5)), (((2.0 * locals.var_t11_dn6) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn6)), (((2.0 * locals.var_t11_dn7) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn7)), (((2.0 * locals.var_t11_dn8) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn8)), (((2.0 * locals.var_t11_dn9) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn9)), (((2.0 * locals.var_t11_dn10) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn10)), (((2.0 * locals.var_t11_dn11) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn11)), (((2.0 * locals.var_t11_dn14) * locals.var_t4) + (assign60720_e94776 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60720_e94780;
        locals.var_t2_dn0 = assign60720_e94780_d_n0;
        locals.var_t2_dn2 = assign60720_e94780_d_n2;
        locals.var_t2_dn4 = assign60720_e94780_d_n4;
        locals.var_t2_dn5 = assign60720_e94780_d_n5;
        locals.var_t2_dn6 = assign60720_e94780_d_n6;
        locals.var_t2_dn7 = assign60720_e94780_d_n7;
        locals.var_t2_dn8 = assign60720_e94780_d_n8;
        locals.var_t2_dn9 = assign60720_e94780_d_n9;
        locals.var_t2_dn10 = assign60720_e94780_d_n10;
        locals.var_t2_dn11 = assign60720_e94780_d_n11;
        locals.var_t2_dn14 = assign60720_e94780_d_n14;

    }

    pub(super) fn stamp_transient_block_214(
        locals: &mut StampLocals,
    ) {
        let (assign60730_e94796, assign60730_e94796_d_n0, assign60730_e94796_d_n2, assign60730_e94796_d_n4, assign60730_e94796_d_n5, assign60730_e94796_d_n6, assign60730_e94796_d_n7, assign60730_e94796_d_n8, assign60730_e94796_d_n9, assign60730_e94796_d_n10, assign60730_e94796_d_n11, assign60730_e94796_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60730_e94792: f64 = (locals.var_t11 * locals.var_t4);
        let assign60730_e94794: f64 = (assign60730_e94792 * locals.var_t4);
        (assign60730_e94794, ((((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn0)), ((((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn2)), ((((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn4)), ((((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn5)), ((((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn6)), ((((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn7)), ((((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn8)), ((((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn9)), ((((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn10)), ((((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn11)), ((((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)) * locals.var_t4) + (assign60730_e94792 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign60730_e94796;
        locals.var_t8_dn0 = assign60730_e94796_d_n0;
        locals.var_t8_dn2 = assign60730_e94796_d_n2;
        locals.var_t8_dn4 = assign60730_e94796_d_n4;
        locals.var_t8_dn5 = assign60730_e94796_d_n5;
        locals.var_t8_dn6 = assign60730_e94796_d_n6;
        locals.var_t8_dn7 = assign60730_e94796_d_n7;
        locals.var_t8_dn8 = assign60730_e94796_d_n8;
        locals.var_t8_dn9 = assign60730_e94796_d_n9;
        locals.var_t8_dn10 = assign60730_e94796_d_n10;
        locals.var_t8_dn11 = assign60730_e94796_d_n11;
        locals.var_t8_dn14 = assign60730_e94796_d_n14;

        let (assign60740_e94813, assign60740_e94813_d_n0, assign60740_e94813_d_n2, assign60740_e94813_d_n4, assign60740_e94813_d_n5, assign60740_e94813_d_n6, assign60740_e94813_d_n7, assign60740_e94813_d_n8, assign60740_e94813_d_n9, assign60740_e94813_d_n10, assign60740_e94813_d_n11, assign60740_e94813_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60740_e94808: f64 = (locals.var_t7 * locals.var_t7);
        let assign60740_e94810: f64 = (assign60740_e94808 + locals.var_t8);
        let assign60740_e94811: f64 = (assign60740_e94810).sqrt();
        (assign60740_e94811, ((((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) + locals.var_t8_dn0) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) + locals.var_t8_dn2) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) + locals.var_t8_dn4) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) + locals.var_t8_dn5) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) + locals.var_t8_dn6) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) + locals.var_t8_dn7) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) + locals.var_t8_dn8) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) + locals.var_t8_dn9) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) + locals.var_t8_dn10) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) + locals.var_t8_dn11) / (2.0 * assign60740_e94811)), ((((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) + locals.var_t8_dn14) / (2.0 * assign60740_e94811)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign60740_e94813;
        locals.var_t9_dn0 = assign60740_e94813_d_n0;
        locals.var_t9_dn2 = assign60740_e94813_d_n2;
        locals.var_t9_dn4 = assign60740_e94813_d_n4;
        locals.var_t9_dn5 = assign60740_e94813_d_n5;
        locals.var_t9_dn6 = assign60740_e94813_d_n6;
        locals.var_t9_dn7 = assign60740_e94813_d_n7;
        locals.var_t9_dn8 = assign60740_e94813_d_n8;
        locals.var_t9_dn9 = assign60740_e94813_d_n9;
        locals.var_t9_dn10 = assign60740_e94813_d_n10;
        locals.var_t9_dn11 = assign60740_e94813_d_n11;
        locals.var_t9_dn14 = assign60740_e94813_d_n14;

        let (assign60750_e94830, assign60750_e94830_d_n0, assign60750_e94830_d_n2, assign60750_e94830_d_n4, assign60750_e94830_d_n5, assign60750_e94830_d_n6, assign60750_e94830_d_n7, assign60750_e94830_d_n8, assign60750_e94830_d_n9, assign60750_e94830_d_n10, assign60750_e94830_d_n11, assign60750_e94830_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60750_e94825: f64 = (-locals.var_t7);
        let assign60750_e94827: f64 = (assign60750_e94825 + locals.var_t9);
        let assign60750_e94828: f64 = (0.5 * assign60750_e94827);
        (assign60750_e94828, (0.5 * ((-locals.var_t7_dn0) + locals.var_t9_dn0)), (0.5 * ((-locals.var_t7_dn2) + locals.var_t9_dn2)), (0.5 * ((-locals.var_t7_dn4) + locals.var_t9_dn4)), (0.5 * ((-locals.var_t7_dn5) + locals.var_t9_dn5)), (0.5 * ((-locals.var_t7_dn6) + locals.var_t9_dn6)), (0.5 * ((-locals.var_t7_dn7) + locals.var_t9_dn7)), (0.5 * ((-locals.var_t7_dn8) + locals.var_t9_dn8)), (0.5 * ((-locals.var_t7_dn9) + locals.var_t9_dn9)), (0.5 * ((-locals.var_t7_dn10) + locals.var_t9_dn10)), (0.5 * ((-locals.var_t7_dn11) + locals.var_t9_dn11)), (0.5 * ((-locals.var_t7_dn14) + locals.var_t9_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60750_e94830;
        locals.var_lred_dn0 = assign60750_e94830_d_n0;
        locals.var_lred_dn2 = assign60750_e94830_d_n2;
        locals.var_lred_dn4 = assign60750_e94830_d_n4;
        locals.var_lred_dn5 = assign60750_e94830_d_n5;
        locals.var_lred_dn6 = assign60750_e94830_d_n6;
        locals.var_lred_dn7 = assign60750_e94830_d_n7;
        locals.var_lred_dn8 = assign60750_e94830_d_n8;
        locals.var_lred_dn9 = assign60750_e94830_d_n9;
        locals.var_lred_dn10 = assign60750_e94830_d_n10;
        locals.var_lred_dn11 = assign60750_e94830_d_n11;
        locals.var_lred_dn14 = assign60750_e94830_d_n14;

        let (assign60760_e94842, assign60760_e94842_d_n0, assign60760_e94842_d_n2, assign60760_e94842_d_n4, assign60760_e94842_d_n5, assign60760_e94842_d_n6, assign60760_e94842_d_n7, assign60760_e94842_d_n8, assign60760_e94842_d_n9, assign60760_e94842_d_n10, assign60760_e94842_d_n11, assign60760_e94842_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60760_e94842;
        locals.var_t1_dn0 = assign60760_e94842_d_n0;
        locals.var_t1_dn2 = assign60760_e94842_d_n2;
        locals.var_t1_dn4 = assign60760_e94842_d_n4;
        locals.var_t1_dn5 = assign60760_e94842_d_n5;
        locals.var_t1_dn6 = assign60760_e94842_d_n6;
        locals.var_t1_dn7 = assign60760_e94842_d_n7;
        locals.var_t1_dn8 = assign60760_e94842_d_n8;
        locals.var_t1_dn9 = assign60760_e94842_d_n9;
        locals.var_t1_dn10 = assign60760_e94842_d_n10;
        locals.var_t1_dn11 = assign60760_e94842_d_n11;
        locals.var_t1_dn14 = assign60760_e94842_d_n14;

        let (assign60770_e94856, assign60770_e94856_d_n0, assign60770_e94856_d_n2, assign60770_e94856_d_n4, assign60770_e94856_d_n5, assign60770_e94856_d_n6, assign60770_e94856_d_n7, assign60770_e94856_d_n8, assign60770_e94856_d_n9, assign60770_e94856_d_n10, assign60770_e94856_d_n11, assign60770_e94856_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1464 == 0.0)) {
        let assign60770_e94854: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign60770_e94854, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn7 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn7)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn9 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn9)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn11 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn11)), ((locals.var_fmdvds_dn14 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60770_e94856;
        locals.var_lred_dn0 = assign60770_e94856_d_n0;
        locals.var_lred_dn2 = assign60770_e94856_d_n2;
        locals.var_lred_dn4 = assign60770_e94856_d_n4;
        locals.var_lred_dn5 = assign60770_e94856_d_n5;
        locals.var_lred_dn6 = assign60770_e94856_d_n6;
        locals.var_lred_dn7 = assign60770_e94856_d_n7;
        locals.var_lred_dn8 = assign60770_e94856_d_n8;
        locals.var_lred_dn9 = assign60770_e94856_d_n9;
        locals.var_lred_dn10 = assign60770_e94856_d_n10;
        locals.var_lred_dn11 = assign60770_e94856_d_n11;
        locals.var_lred_dn14 = assign60770_e94856_d_n14;

        let (assign60780_e94867, assign60780_e94867_d_n0, assign60780_e94867_d_n2, assign60780_e94867_d_n4, assign60780_e94867_d_n5, assign60780_e94867_d_n6, assign60780_e94867_d_n7, assign60780_e94867_d_n8, assign60780_e94867_d_n9, assign60780_e94867_d_n10, assign60780_e94867_d_n11, assign60780_e94867_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60780_e94865: f64 = (locals.var_lred * locals.var_clmmod);
        (assign60780_e94865, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn9 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn14 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60780_e94867;
        locals.var_lred_dn0 = assign60780_e94867_d_n0;
        locals.var_lred_dn2 = assign60780_e94867_d_n2;
        locals.var_lred_dn4 = assign60780_e94867_d_n4;
        locals.var_lred_dn5 = assign60780_e94867_d_n5;
        locals.var_lred_dn6 = assign60780_e94867_d_n6;
        locals.var_lred_dn7 = assign60780_e94867_d_n7;
        locals.var_lred_dn8 = assign60780_e94867_d_n8;
        locals.var_lred_dn9 = assign60780_e94867_d_n9;
        locals.var_lred_dn10 = assign60780_e94867_d_n10;
        locals.var_lred_dn11 = assign60780_e94867_d_n11;
        locals.var_lred_dn14 = assign60780_e94867_d_n14;

        let (assign60790_e94878, assign60790_e94878_d_n0, assign60790_e94878_d_n2, assign60790_e94878_d_n4, assign60790_e94878_d_n5, assign60790_e94878_d_n6, assign60790_e94878_d_n7, assign60790_e94878_d_n8, assign60790_e94878_d_n9, assign60790_e94878_d_n10, assign60790_e94878_d_n11, assign60790_e94878_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60790_e94876: f64 = (locals.var_vgp + locals.var_beta_inv);
        (assign60790_e94876, (locals.var_vgp_dn0 + locals.var_beta_inv_dn0), (locals.var_vgp_dn2 + locals.var_beta_inv_dn2), (locals.var_vgp_dn4 + locals.var_beta_inv_dn4), (locals.var_vgp_dn5 + locals.var_beta_inv_dn5), (locals.var_vgp_dn6 + locals.var_beta_inv_dn6), (locals.var_vgp_dn7 + locals.var_beta_inv_dn7), (locals.var_vgp_dn8 + locals.var_beta_inv_dn8), (locals.var_vgp_dn9 + locals.var_beta_inv_dn9), (locals.var_vgp_dn10 + locals.var_beta_inv_dn10), (locals.var_vgp_dn11 + locals.var_beta_inv_dn11), (locals.var_vgp_dn14 + locals.var_beta_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60790_e94878;
        locals.var_t1_dn0 = assign60790_e94878_d_n0;
        locals.var_t1_dn2 = assign60790_e94878_d_n2;
        locals.var_t1_dn4 = assign60790_e94878_d_n4;
        locals.var_t1_dn5 = assign60790_e94878_d_n5;
        locals.var_t1_dn6 = assign60790_e94878_d_n6;
        locals.var_t1_dn7 = assign60790_e94878_d_n7;
        locals.var_t1_dn8 = assign60790_e94878_d_n8;
        locals.var_t1_dn9 = assign60790_e94878_d_n9;
        locals.var_t1_dn10 = assign60790_e94878_d_n10;
        locals.var_t1_dn11 = assign60790_e94878_d_n11;
        locals.var_t1_dn14 = assign60790_e94878_d_n14;

        let (assign60800_e94891, assign60800_e94891_d_n0, assign60800_e94891_d_n2, assign60800_e94891_d_n4, assign60800_e94891_d_n5, assign60800_e94891_d_n6, assign60800_e94891_d_n7, assign60800_e94891_d_n8, assign60800_e94891_d_n9, assign60800_e94891_d_n10, assign60800_e94891_d_n11, assign60800_e94891_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60800_e94887: f64 = (locals.var_t1 * locals.var_f10);
        let assign60800_e94889: f64 = (assign60800_e94887 - locals.var_f11);
        (assign60800_e94889, (((locals.var_t1_dn0 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn0)) - locals.var_f11_dn0), (((locals.var_t1_dn2 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn2)) - locals.var_f11_dn2), (((locals.var_t1_dn4 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn4)) - locals.var_f11_dn4), (((locals.var_t1_dn5 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn5)) - locals.var_f11_dn5), (((locals.var_t1_dn6 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn6)) - locals.var_f11_dn6), (((locals.var_t1_dn7 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn7)) - locals.var_f11_dn7), (((locals.var_t1_dn8 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn8)) - locals.var_f11_dn8), (((locals.var_t1_dn9 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn9)) - locals.var_f11_dn9), (((locals.var_t1_dn10 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn10)) - locals.var_f11_dn10), (((locals.var_t1_dn11 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn11)) - locals.var_f11_dn11), (((locals.var_t1_dn14 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn14)) - locals.var_f11_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60800_e94891;
        locals.var_t2_dn0 = assign60800_e94891_d_n0;
        locals.var_t2_dn2 = assign60800_e94891_d_n2;
        locals.var_t2_dn4 = assign60800_e94891_d_n4;
        locals.var_t2_dn5 = assign60800_e94891_d_n5;
        locals.var_t2_dn6 = assign60800_e94891_d_n6;
        locals.var_t2_dn7 = assign60800_e94891_d_n7;
        locals.var_t2_dn8 = assign60800_e94891_d_n8;
        locals.var_t2_dn9 = assign60800_e94891_d_n9;
        locals.var_t2_dn10 = assign60800_e94891_d_n10;
        locals.var_t2_dn11 = assign60800_e94891_d_n11;
        locals.var_t2_dn14 = assign60800_e94891_d_n14;

        let (assign60810_e94918, assign60810_e94918_d_n0, assign60810_e94918_d_n2, assign60810_e94918_d_n4, assign60810_e94918_d_n5, assign60810_e94918_d_n6, assign60810_e94918_d_n7, assign60810_e94918_d_n8, assign60810_e94918_d_n9, assign60810_e94918_d_n10, assign60810_e94918_d_n11, assign60810_e94918_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60810_e94903: f64 = (locals.var_xi0 + 1.0);
        let assign60810_e94904: f64 = (1.5 - assign60810_e94903);
        let assign60810_e94907: f64 = (0.5 * locals.var_beta);
        let assign60810_e94909: f64 = (assign60810_e94907 * locals.var_pds);
        let assign60810_e94910: f64 = (assign60810_e94904 - assign60810_e94909);
        let assign60810_e94911: f64 = (locals.var_cnst0 * assign60810_e94910);
        let assign60810_e94914: f64 = (locals.var_cox * locals.var_t2);
        let assign60810_e94915: f64 = (assign60810_e94911 + assign60810_e94914);
        let assign60810_e94916: f64 = (locals.var_cnst0 * assign60810_e94915);
        (assign60810_e94916, ((locals.var_cnst0_dn0 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn0 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn0) - (((0.5 * locals.var_beta_dn0) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn0))))) + ((locals.var_cox_dn0 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn0))))), ((locals.var_cnst0_dn2 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn2 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn2) - (((0.5 * locals.var_beta_dn2) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn2))))) + ((locals.var_cox_dn2 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn2))))), ((locals.var_cnst0_dn4 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn4 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn4) - (((0.5 * locals.var_beta_dn4) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn4))))) + ((locals.var_cox_dn4 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn4))))), ((locals.var_cnst0_dn5 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn5 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn5) - (((0.5 * locals.var_beta_dn5) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn5))))) + ((locals.var_cox_dn5 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn5))))), ((locals.var_cnst0_dn6 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn6 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn6) - (((0.5 * locals.var_beta_dn6) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn6))))) + ((locals.var_cox_dn6 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn6))))), ((locals.var_cnst0_dn7 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn7 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn7) - (((0.5 * locals.var_beta_dn7) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn7))))) + ((locals.var_cox_dn7 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn7))))), ((locals.var_cnst0_dn8 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn8 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn8) - (((0.5 * locals.var_beta_dn8) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn8))))) + ((locals.var_cox_dn8 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn8))))), ((locals.var_cnst0_dn9 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn9 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn9) - (((0.5 * locals.var_beta_dn9) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn9))))) + ((locals.var_cox_dn9 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn9))))), ((locals.var_cnst0_dn10 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn10 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn10) - (((0.5 * locals.var_beta_dn10) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn10))))) + ((locals.var_cox_dn10 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn10))))), ((locals.var_cnst0_dn11 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn11 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn11) - (((0.5 * locals.var_beta_dn11) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn11))))) + ((locals.var_cox_dn11 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn11))))), ((locals.var_cnst0_dn14 * assign60810_e94915) + (locals.var_cnst0 * (((locals.var_cnst0_dn14 * assign60810_e94910) + (locals.var_cnst0 * ((-locals.var_xi0_dn14) - (((0.5 * locals.var_beta_dn14) * locals.var_pds) + (assign60810_e94907 * locals.var_pds_dn14))))) + ((locals.var_cox_dn14 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn14))))),)
    } else {
        (locals.var_qbnm, locals.var_qbnm_dn0, locals.var_qbnm_dn2, locals.var_qbnm_dn4, locals.var_qbnm_dn5, locals.var_qbnm_dn6, locals.var_qbnm_dn7, locals.var_qbnm_dn8, locals.var_qbnm_dn9, locals.var_qbnm_dn10, locals.var_qbnm_dn11, locals.var_qbnm_dn14,)
    }
};
        locals.var_qbnm = assign60810_e94918;
        locals.var_qbnm_dn0 = assign60810_e94918_d_n0;
        locals.var_qbnm_dn2 = assign60810_e94918_d_n2;
        locals.var_qbnm_dn4 = assign60810_e94918_d_n4;
        locals.var_qbnm_dn5 = assign60810_e94918_d_n5;
        locals.var_qbnm_dn6 = assign60810_e94918_d_n6;
        locals.var_qbnm_dn7 = assign60810_e94918_d_n7;
        locals.var_qbnm_dn8 = assign60810_e94918_d_n8;
        locals.var_qbnm_dn9 = assign60810_e94918_d_n9;
        locals.var_qbnm_dn10 = assign60810_e94918_d_n10;
        locals.var_qbnm_dn11 = assign60810_e94918_d_n11;
        locals.var_qbnm_dn14 = assign60810_e94918_d_n14;

        let (assign60820_e94927, assign60820_e94927_d_n0, assign60820_e94927_d_n2, assign60820_e94927_d_n4, assign60820_e94927_d_n5, assign60820_e94927_d_n6, assign60820_e94927_d_n7, assign60820_e94927_d_n8, assign60820_e94927_d_n9, assign60820_e94927_d_n10, assign60820_e94927_d_n11, assign60820_e94927_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60820_e94927;
        locals.var_t1_dn0 = assign60820_e94927_d_n0;
        locals.var_t1_dn2 = assign60820_e94927_d_n2;
        locals.var_t1_dn4 = assign60820_e94927_d_n4;
        locals.var_t1_dn5 = assign60820_e94927_d_n5;
        locals.var_t1_dn6 = assign60820_e94927_d_n6;
        locals.var_t1_dn7 = assign60820_e94927_d_n7;
        locals.var_t1_dn8 = assign60820_e94927_d_n8;
        locals.var_t1_dn9 = assign60820_e94927_d_n9;
        locals.var_t1_dn10 = assign60820_e94927_d_n10;
        locals.var_t1_dn11 = assign60820_e94927_d_n11;
        locals.var_t1_dn14 = assign60820_e94927_d_n14;

        let (assign60830_e94940, assign60830_e94940_d_n0, assign60830_e94940_d_n2, assign60830_e94940_d_n4, assign60830_e94940_d_n5, assign60830_e94940_d_n6, assign60830_e94940_d_n7, assign60830_e94940_d_n8, assign60830_e94940_d_n9, assign60830_e94940_d_n10, assign60830_e94940_d_n11, assign60830_e94940_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60830_e94936: f64 = (locals.var_t1 * locals.var_qbnm);
        let assign60830_e94938: f64 = (assign60830_e94936 / locals.var_fdd);
        (assign60830_e94938, (((((locals.var_t1_dn0 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn0)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn0)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn2 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn2)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn2)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn4 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn4)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn4)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn5 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn5)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn5)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn6 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn6)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn6)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn7 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn7)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn7)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn8 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn8)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn8)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn9 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn9)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn9)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn10 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn10)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn10)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn11 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn11)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn11)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn14 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn14)) * locals.var_fdd) - (assign60830_e94936 * locals.var_fdd_dn14)) / (locals.var_fdd * locals.var_fdd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign60830_e94940;
        locals.var_qbu_dn0 = assign60830_e94940_d_n0;
        locals.var_qbu_dn2 = assign60830_e94940_d_n2;
        locals.var_qbu_dn4 = assign60830_e94940_d_n4;
        locals.var_qbu_dn5 = assign60830_e94940_d_n5;
        locals.var_qbu_dn6 = assign60830_e94940_d_n6;
        locals.var_qbu_dn7 = assign60830_e94940_d_n7;
        locals.var_qbu_dn8 = assign60830_e94940_d_n8;
        locals.var_qbu_dn9 = assign60830_e94940_d_n9;
        locals.var_qbu_dn10 = assign60830_e94940_d_n10;
        locals.var_qbu_dn11 = assign60830_e94940_d_n11;
        locals.var_qbu_dn14 = assign60830_e94940_d_n14;

        let (assign60840_e94951, assign60840_e94951_d_n0, assign60840_e94951_d_n2, assign60840_e94951_d_n4, assign60840_e94951_d_n5, assign60840_e94951_d_n6, assign60840_e94951_d_n7, assign60840_e94951_d_n8, assign60840_e94951_d_n9, assign60840_e94951_d_n10, assign60840_e94951_d_n11, assign60840_e94951_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60840_e94949: f64 = (2.0 * locals.var_fac1);
        (assign60840_e94949, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn4), (2.0 * locals.var_fac1_dn5), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn8), (2.0 * locals.var_fac1_dn9), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60840_e94951;
        locals.var_t1_dn0 = assign60840_e94951_d_n0;
        locals.var_t1_dn2 = assign60840_e94951_d_n2;
        locals.var_t1_dn4 = assign60840_e94951_d_n4;
        locals.var_t1_dn5 = assign60840_e94951_d_n5;
        locals.var_t1_dn6 = assign60840_e94951_d_n6;
        locals.var_t1_dn7 = assign60840_e94951_d_n7;
        locals.var_t1_dn8 = assign60840_e94951_d_n8;
        locals.var_t1_dn9 = assign60840_e94951_d_n9;
        locals.var_t1_dn10 = assign60840_e94951_d_n10;
        locals.var_t1_dn11 = assign60840_e94951_d_n11;
        locals.var_t1_dn14 = assign60840_e94951_d_n14;

        let (assign60850_e94964, assign60850_e94964_d_n0, assign60850_e94964_d_n2, assign60850_e94964_d_n4, assign60850_e94964_d_n5, assign60850_e94964_d_n6, assign60850_e94964_d_n7, assign60850_e94964_d_n8, assign60850_e94964_d_n9, assign60850_e94964_d_n10, assign60850_e94964_d_n11, assign60850_e94964_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60850_e94961: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign60850_e94962: f64 = (locals.var_t1 * assign60850_e94961);
        (assign60850_e94962, ((locals.var_t1_dn0 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn4 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn4 - locals.var_xi0p12_dn4))), ((locals.var_t1_dn5 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn5 - locals.var_xi0p12_dn5))), ((locals.var_t1_dn6 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn8 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn8 - locals.var_xi0p12_dn8))), ((locals.var_t1_dn9 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn9 - locals.var_xi0p12_dn9))), ((locals.var_t1_dn10 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn14 * assign60850_e94961) + (locals.var_t1 * (locals.var_f10_dn14 - locals.var_xi0p12_dn14))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn4, locals.var_dtpds_dn5, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn8, locals.var_dtpds_dn9, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn14,)
    }
};
        locals.var_dtpds = assign60850_e94964;
        locals.var_dtpds_dn0 = assign60850_e94964_d_n0;
        locals.var_dtpds_dn2 = assign60850_e94964_d_n2;
        locals.var_dtpds_dn4 = assign60850_e94964_d_n4;
        locals.var_dtpds_dn5 = assign60850_e94964_d_n5;
        locals.var_dtpds_dn6 = assign60850_e94964_d_n6;
        locals.var_dtpds_dn7 = assign60850_e94964_d_n7;
        locals.var_dtpds_dn8 = assign60850_e94964_d_n8;
        locals.var_dtpds_dn9 = assign60850_e94964_d_n9;
        locals.var_dtpds_dn10 = assign60850_e94964_d_n10;
        locals.var_dtpds_dn11 = assign60850_e94964_d_n11;
        locals.var_dtpds_dn14 = assign60850_e94964_d_n14;

        let (assign60860_e94977, assign60860_e94977_d_n0, assign60860_e94977_d_n2, assign60860_e94977_d_n4, assign60860_e94977_d_n5, assign60860_e94977_d_n6, assign60860_e94977_d_n7, assign60860_e94977_d_n8, assign60860_e94977_d_n9, assign60860_e94977_d_n10, assign60860_e94977_d_n11, assign60860_e94977_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60860_e94974: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign60860_e94975: f64 = (2.0 * assign60860_e94974);
        (assign60860_e94975, (2.0 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0)), (2.0 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2)), (2.0 * (locals.var_f10_dn4 - locals.var_xi0p12_dn4)), (2.0 * (locals.var_f10_dn5 - locals.var_xi0p12_dn5)), (2.0 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6)), (2.0 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7)), (2.0 * (locals.var_f10_dn8 - locals.var_xi0p12_dn8)), (2.0 * (locals.var_f10_dn9 - locals.var_xi0p12_dn9)), (2.0 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10)), (2.0 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11)), (2.0 * (locals.var_f10_dn14 - locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60860_e94977;
        locals.var_t2_dn0 = assign60860_e94977_d_n0;
        locals.var_t2_dn2 = assign60860_e94977_d_n2;
        locals.var_t2_dn4 = assign60860_e94977_d_n4;
        locals.var_t2_dn5 = assign60860_e94977_d_n5;
        locals.var_t2_dn6 = assign60860_e94977_d_n6;
        locals.var_t2_dn7 = assign60860_e94977_d_n7;
        locals.var_t2_dn8 = assign60860_e94977_d_n8;
        locals.var_t2_dn9 = assign60860_e94977_d_n9;
        locals.var_t2_dn10 = assign60860_e94977_d_n10;
        locals.var_t2_dn11 = assign60860_e94977_d_n11;
        locals.var_t2_dn14 = assign60860_e94977_d_n14;

        let (assign60870_e94988, assign60870_e94988_d_n0, assign60870_e94988_d_n2, assign60870_e94988_d_n4, assign60870_e94988_d_n5, assign60870_e94988_d_n6, assign60870_e94988_d_n7, assign60870_e94988_d_n8, assign60870_e94988_d_n9, assign60870_e94988_d_n10, assign60870_e94988_d_n11, assign60870_e94988_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60870_e94986: f64 = (locals.var_pds + locals.var_dtpds);
        (assign60870_e94986, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn4 + locals.var_dtpds_dn4), (locals.var_pds_dn5 + locals.var_dtpds_dn5), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn8 + locals.var_dtpds_dn8), (locals.var_pds_dn9 + locals.var_dtpds_dn9), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn14 + locals.var_dtpds_dn14),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn4, locals.var_achi_dn5, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn8, locals.var_achi_dn9, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn14,)
    }
};
        locals.var_achi = assign60870_e94988;
        locals.var_achi_dn0 = assign60870_e94988_d_n0;
        locals.var_achi_dn2 = assign60870_e94988_d_n2;
        locals.var_achi_dn4 = assign60870_e94988_d_n4;
        locals.var_achi_dn5 = assign60870_e94988_d_n5;
        locals.var_achi_dn6 = assign60870_e94988_d_n6;
        locals.var_achi_dn7 = assign60870_e94988_d_n7;
        locals.var_achi_dn8 = assign60870_e94988_d_n8;
        locals.var_achi_dn9 = assign60870_e94988_d_n9;
        locals.var_achi_dn10 = assign60870_e94988_d_n10;
        locals.var_achi_dn11 = assign60870_e94988_d_n11;
        locals.var_achi_dn14 = assign60870_e94988_d_n14;

        let (assign60880_e94999, assign60880_e94999_d_n0, assign60880_e94999_d_n2, assign60880_e94999_d_n4, assign60880_e94999_d_n5, assign60880_e94999_d_n6, assign60880_e94999_d_n7, assign60880_e94999_d_n8, assign60880_e94999_d_n9, assign60880_e94999_d_n10, assign60880_e94999_d_n11, assign60880_e94999_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60880_e94997: f64 = (1.0 / locals.var_vgvt);
        (assign60880_e94997, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn4 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn5 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn8 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn9 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn14 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60880_e94999;
        locals.var_t1_dn0 = assign60880_e94999_d_n0;
        locals.var_t1_dn2 = assign60880_e94999_d_n2;
        locals.var_t1_dn4 = assign60880_e94999_d_n4;
        locals.var_t1_dn5 = assign60880_e94999_d_n5;
        locals.var_t1_dn6 = assign60880_e94999_d_n6;
        locals.var_t1_dn7 = assign60880_e94999_d_n7;
        locals.var_t1_dn8 = assign60880_e94999_d_n8;
        locals.var_t1_dn9 = assign60880_e94999_d_n9;
        locals.var_t1_dn10 = assign60880_e94999_d_n10;
        locals.var_t1_dn11 = assign60880_e94999_d_n11;
        locals.var_t1_dn14 = assign60880_e94999_d_n14;

        let (assign60890_e95010, assign60890_e95010_d_n0, assign60890_e95010_d_n2, assign60890_e95010_d_n4, assign60890_e95010_d_n5, assign60890_e95010_d_n6, assign60890_e95010_d_n7, assign60890_e95010_d_n8, assign60890_e95010_d_n9, assign60890_e95010_d_n10, assign60890_e95010_d_n11, assign60890_e95010_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60890_e95008: f64 = (locals.var_achi * locals.var_t1);
        (assign60890_e95008, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn4 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn4)), ((locals.var_achi_dn5 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn5)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn8 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn8)), ((locals.var_achi_dn9 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn9)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn14 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60890_e95010;
        locals.var_t2_dn0 = assign60890_e95010_d_n0;
        locals.var_t2_dn2 = assign60890_e95010_d_n2;
        locals.var_t2_dn4 = assign60890_e95010_d_n4;
        locals.var_t2_dn5 = assign60890_e95010_d_n5;
        locals.var_t2_dn6 = assign60890_e95010_d_n6;
        locals.var_t2_dn7 = assign60890_e95010_d_n7;
        locals.var_t2_dn8 = assign60890_e95010_d_n8;
        locals.var_t2_dn9 = assign60890_e95010_d_n9;
        locals.var_t2_dn10 = assign60890_e95010_d_n10;
        locals.var_t2_dn11 = assign60890_e95010_d_n11;
        locals.var_t2_dn14 = assign60890_e95010_d_n14;

        let (assign60900_e95021, assign60900_e95021_d_n0, assign60900_e95021_d_n2, assign60900_e95021_d_n4, assign60900_e95021_d_n5, assign60900_e95021_d_n6, assign60900_e95021_d_n7, assign60900_e95021_d_n8, assign60900_e95021_d_n9, assign60900_e95021_d_n10, assign60900_e95021_d_n11, assign60900_e95021_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60900_e95019: f64 = (1.0 - locals.var_t2);
        (assign60900_e95019, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60900_e95021;
        locals.var_t3_dn0 = assign60900_e95021_d_n0;
        locals.var_t3_dn2 = assign60900_e95021_d_n2;
        locals.var_t3_dn4 = assign60900_e95021_d_n4;
        locals.var_t3_dn5 = assign60900_e95021_d_n5;
        locals.var_t3_dn6 = assign60900_e95021_d_n6;
        locals.var_t3_dn7 = assign60900_e95021_d_n7;
        locals.var_t3_dn8 = assign60900_e95021_d_n8;
        locals.var_t3_dn9 = assign60900_e95021_d_n9;
        locals.var_t3_dn10 = assign60900_e95021_d_n10;
        locals.var_t3_dn11 = assign60900_e95021_d_n11;
        locals.var_t3_dn14 = assign60900_e95021_d_n14;

        let (assign60910_e95032, assign60910_e95032_d_n0, assign60910_e95032_d_n2, assign60910_e95032_d_n4, assign60910_e95032_d_n5, assign60910_e95032_d_n6, assign60910_e95032_d_n7, assign60910_e95032_d_n8, assign60910_e95032_d_n9, assign60910_e95032_d_n10, assign60910_e95032_d_n11, assign60910_e95032_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60910_e95030: f64 = (1.0 - locals.var_t3);
        (assign60910_e95030, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn4), (-locals.var_t3_dn5), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn8), (-locals.var_t3_dn9), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign60910_e95032;
        locals.var_tx_dn0 = assign60910_e95032_d_n0;
        locals.var_tx_dn2 = assign60910_e95032_d_n2;
        locals.var_tx_dn4 = assign60910_e95032_d_n4;
        locals.var_tx_dn5 = assign60910_e95032_d_n5;
        locals.var_tx_dn6 = assign60910_e95032_d_n6;
        locals.var_tx_dn7 = assign60910_e95032_d_n7;
        locals.var_tx_dn8 = assign60910_e95032_d_n8;
        locals.var_tx_dn9 = assign60910_e95032_d_n9;
        locals.var_tx_dn10 = assign60910_e95032_d_n10;
        locals.var_tx_dn11 = assign60910_e95032_d_n11;
        locals.var_tx_dn14 = assign60910_e95032_d_n14;

        let (assign60920_e95043, assign60920_e95043_d_n0, assign60920_e95043_d_n2, assign60920_e95043_d_n4, assign60920_e95043_d_n5, assign60920_e95043_d_n6, assign60920_e95043_d_n7, assign60920_e95043_d_n8, assign60920_e95043_d_n9, assign60920_e95043_d_n10, assign60920_e95043_d_n11, assign60920_e95043_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60920_e95041: f64 = (locals.var_tx * locals.var_tx);
        (assign60920_e95041, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign60920_e95043;
        locals.var_x2_dn0 = assign60920_e95043_d_n0;
        locals.var_x2_dn2 = assign60920_e95043_d_n2;
        locals.var_x2_dn4 = assign60920_e95043_d_n4;
        locals.var_x2_dn5 = assign60920_e95043_d_n5;
        locals.var_x2_dn6 = assign60920_e95043_d_n6;
        locals.var_x2_dn7 = assign60920_e95043_d_n7;
        locals.var_x2_dn8 = assign60920_e95043_d_n8;
        locals.var_x2_dn9 = assign60920_e95043_d_n9;
        locals.var_x2_dn10 = assign60920_e95043_d_n10;
        locals.var_x2_dn11 = assign60920_e95043_d_n11;
        locals.var_x2_dn14 = assign60920_e95043_d_n14;

        let (assign60930_e95054, assign60930_e95054_d_n0, assign60930_e95054_d_n2, assign60930_e95054_d_n4, assign60930_e95054_d_n5, assign60930_e95054_d_n6, assign60930_e95054_d_n7, assign60930_e95054_d_n8, assign60930_e95054_d_n9, assign60930_e95054_d_n10, assign60930_e95054_d_n11, assign60930_e95054_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign60930_e95052: f64 = 1.0;
        (assign60930_e95052, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign60930_e95054;
        locals.var_xmax2_dn0 = assign60930_e95054_d_n0;
        locals.var_xmax2_dn2 = assign60930_e95054_d_n2;
        locals.var_xmax2_dn4 = assign60930_e95054_d_n4;
        locals.var_xmax2_dn5 = assign60930_e95054_d_n5;
        locals.var_xmax2_dn6 = assign60930_e95054_d_n6;
        locals.var_xmax2_dn7 = assign60930_e95054_d_n7;
        locals.var_xmax2_dn8 = assign60930_e95054_d_n8;
        locals.var_xmax2_dn9 = assign60930_e95054_d_n9;
        locals.var_xmax2_dn10 = assign60930_e95054_d_n10;
        locals.var_xmax2_dn11 = assign60930_e95054_d_n11;
        locals.var_xmax2_dn14 = assign60930_e95054_d_n14;

        let (assign60940_e95063, assign60940_e95063_d_n0, assign60940_e95063_d_n2, assign60940_e95063_d_n4, assign60940_e95063_d_n5, assign60940_e95063_d_n6, assign60940_e95063_d_n7, assign60940_e95063_d_n8, assign60940_e95063_d_n9, assign60940_e95063_d_n10, assign60940_e95063_d_n11, assign60940_e95063_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60940_e95063;
        locals.var_xp_dn0 = assign60940_e95063_d_n0;
        locals.var_xp_dn2 = assign60940_e95063_d_n2;
        locals.var_xp_dn4 = assign60940_e95063_d_n4;
        locals.var_xp_dn5 = assign60940_e95063_d_n5;
        locals.var_xp_dn6 = assign60940_e95063_d_n6;
        locals.var_xp_dn7 = assign60940_e95063_d_n7;
        locals.var_xp_dn8 = assign60940_e95063_d_n8;
        locals.var_xp_dn9 = assign60940_e95063_d_n9;
        locals.var_xp_dn10 = assign60940_e95063_d_n10;
        locals.var_xp_dn11 = assign60940_e95063_d_n11;
        locals.var_xp_dn14 = assign60940_e95063_d_n14;

        let (assign60950_e95072, assign60950_e95072_d_n0, assign60950_e95072_d_n2, assign60950_e95072_d_n4, assign60950_e95072_d_n5, assign60950_e95072_d_n6, assign60950_e95072_d_n7, assign60950_e95072_d_n8, assign60950_e95072_d_n9, assign60950_e95072_d_n10, assign60950_e95072_d_n11, assign60950_e95072_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60950_e95072;
        locals.var_xmp_dn0 = assign60950_e95072_d_n0;
        locals.var_xmp_dn2 = assign60950_e95072_d_n2;
        locals.var_xmp_dn4 = assign60950_e95072_d_n4;
        locals.var_xmp_dn5 = assign60950_e95072_d_n5;
        locals.var_xmp_dn6 = assign60950_e95072_d_n6;
        locals.var_xmp_dn7 = assign60950_e95072_d_n7;
        locals.var_xmp_dn8 = assign60950_e95072_d_n8;
        locals.var_xmp_dn9 = assign60950_e95072_d_n9;
        locals.var_xmp_dn10 = assign60950_e95072_d_n10;
        locals.var_xmp_dn11 = assign60950_e95072_d_n11;
        locals.var_xmp_dn14 = assign60950_e95072_d_n14;

        let (assign60960_e95081,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60960_e95081;

    }

    pub(super) fn stamp_transient_block_215(
        locals: &mut StampLocals,
    ) {
        let (assign60970_e95090,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60970_e95090;

        let (assign60980_e95099, assign60980_e95099_d_n0, assign60980_e95099_d_n2, assign60980_e95099_d_n4, assign60980_e95099_d_n5, assign60980_e95099_d_n6, assign60980_e95099_d_n7, assign60980_e95099_d_n8, assign60980_e95099_d_n9, assign60980_e95099_d_n10, assign60980_e95099_d_n11, assign60980_e95099_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign60980_e95099;
        locals.var_arg_dn0 = assign60980_e95099_d_n0;
        locals.var_arg_dn2 = assign60980_e95099_d_n2;
        locals.var_arg_dn4 = assign60980_e95099_d_n4;
        locals.var_arg_dn5 = assign60980_e95099_d_n5;
        locals.var_arg_dn6 = assign60980_e95099_d_n6;
        locals.var_arg_dn7 = assign60980_e95099_d_n7;
        locals.var_arg_dn8 = assign60980_e95099_d_n8;
        locals.var_arg_dn9 = assign60980_e95099_d_n9;
        locals.var_arg_dn10 = assign60980_e95099_d_n10;
        locals.var_arg_dn11 = assign60980_e95099_d_n11;
        locals.var_arg_dn14 = assign60980_e95099_d_n14;

        let (assign60990_e95108, assign60990_e95108_d_n0, assign60990_e95108_d_n2, assign60990_e95108_d_n4, assign60990_e95108_d_n5, assign60990_e95108_d_n6, assign60990_e95108_d_n7, assign60990_e95108_d_n8, assign60990_e95108_d_n9, assign60990_e95108_d_n10, assign60990_e95108_d_n11, assign60990_e95108_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60990_e95108;
        locals.var_dnm_dn0 = assign60990_e95108_d_n0;
        locals.var_dnm_dn2 = assign60990_e95108_d_n2;
        locals.var_dnm_dn4 = assign60990_e95108_d_n4;
        locals.var_dnm_dn5 = assign60990_e95108_d_n5;
        locals.var_dnm_dn6 = assign60990_e95108_d_n6;
        locals.var_dnm_dn7 = assign60990_e95108_d_n7;
        locals.var_dnm_dn8 = assign60990_e95108_d_n8;
        locals.var_dnm_dn9 = assign60990_e95108_d_n9;
        locals.var_dnm_dn10 = assign60990_e95108_d_n10;
        locals.var_dnm_dn11 = assign60990_e95108_d_n11;
        locals.var_dnm_dn14 = assign60990_e95108_d_n14;

        let (assign61000_e95119, assign61000_e95119_d_n0, assign61000_e95119_d_n2, assign61000_e95119_d_n4, assign61000_e95119_d_n5, assign61000_e95119_d_n6, assign61000_e95119_d_n7, assign61000_e95119_d_n8, assign61000_e95119_d_n9, assign61000_e95119_d_n10, assign61000_e95119_d_n11, assign61000_e95119_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61000_e95117: f64 = (locals.var_xp * locals.var_x2);
        (assign61000_e95117, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61000_e95119;
        locals.var_xp_dn0 = assign61000_e95119_d_n0;
        locals.var_xp_dn2 = assign61000_e95119_d_n2;
        locals.var_xp_dn4 = assign61000_e95119_d_n4;
        locals.var_xp_dn5 = assign61000_e95119_d_n5;
        locals.var_xp_dn6 = assign61000_e95119_d_n6;
        locals.var_xp_dn7 = assign61000_e95119_d_n7;
        locals.var_xp_dn8 = assign61000_e95119_d_n8;
        locals.var_xp_dn9 = assign61000_e95119_d_n9;
        locals.var_xp_dn10 = assign61000_e95119_d_n10;
        locals.var_xp_dn11 = assign61000_e95119_d_n11;
        locals.var_xp_dn14 = assign61000_e95119_d_n14;

        let (assign61010_e95130, assign61010_e95130_d_n0, assign61010_e95130_d_n2, assign61010_e95130_d_n4, assign61010_e95130_d_n5, assign61010_e95130_d_n6, assign61010_e95130_d_n7, assign61010_e95130_d_n8, assign61010_e95130_d_n9, assign61010_e95130_d_n10, assign61010_e95130_d_n11, assign61010_e95130_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61010_e95128: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61010_e95128, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61010_e95130;
        locals.var_xmp_dn0 = assign61010_e95130_d_n0;
        locals.var_xmp_dn2 = assign61010_e95130_d_n2;
        locals.var_xmp_dn4 = assign61010_e95130_d_n4;
        locals.var_xmp_dn5 = assign61010_e95130_d_n5;
        locals.var_xmp_dn6 = assign61010_e95130_d_n6;
        locals.var_xmp_dn7 = assign61010_e95130_d_n7;
        locals.var_xmp_dn8 = assign61010_e95130_d_n8;
        locals.var_xmp_dn9 = assign61010_e95130_d_n9;
        locals.var_xmp_dn10 = assign61010_e95130_d_n10;
        locals.var_xmp_dn11 = assign61010_e95130_d_n11;
        locals.var_xmp_dn14 = assign61010_e95130_d_n14;

        let (assign61020_e95141, assign61020_e95141_d_n0, assign61020_e95141_d_n2, assign61020_e95141_d_n4, assign61020_e95141_d_n5, assign61020_e95141_d_n6, assign61020_e95141_d_n7, assign61020_e95141_d_n8, assign61020_e95141_d_n9, assign61020_e95141_d_n10, assign61020_e95141_d_n11, assign61020_e95141_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61020_e95139: f64 = (locals.var_xp * locals.var_x2);
        (assign61020_e95139, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61020_e95141;
        locals.var_xp_dn0 = assign61020_e95141_d_n0;
        locals.var_xp_dn2 = assign61020_e95141_d_n2;
        locals.var_xp_dn4 = assign61020_e95141_d_n4;
        locals.var_xp_dn5 = assign61020_e95141_d_n5;
        locals.var_xp_dn6 = assign61020_e95141_d_n6;
        locals.var_xp_dn7 = assign61020_e95141_d_n7;
        locals.var_xp_dn8 = assign61020_e95141_d_n8;
        locals.var_xp_dn9 = assign61020_e95141_d_n9;
        locals.var_xp_dn10 = assign61020_e95141_d_n10;
        locals.var_xp_dn11 = assign61020_e95141_d_n11;
        locals.var_xp_dn14 = assign61020_e95141_d_n14;

        let (assign61030_e95152, assign61030_e95152_d_n0, assign61030_e95152_d_n2, assign61030_e95152_d_n4, assign61030_e95152_d_n5, assign61030_e95152_d_n6, assign61030_e95152_d_n7, assign61030_e95152_d_n8, assign61030_e95152_d_n9, assign61030_e95152_d_n10, assign61030_e95152_d_n11, assign61030_e95152_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61030_e95150: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61030_e95150, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61030_e95152;
        locals.var_xmp_dn0 = assign61030_e95152_d_n0;
        locals.var_xmp_dn2 = assign61030_e95152_d_n2;
        locals.var_xmp_dn4 = assign61030_e95152_d_n4;
        locals.var_xmp_dn5 = assign61030_e95152_d_n5;
        locals.var_xmp_dn6 = assign61030_e95152_d_n6;
        locals.var_xmp_dn7 = assign61030_e95152_d_n7;
        locals.var_xmp_dn8 = assign61030_e95152_d_n8;
        locals.var_xmp_dn9 = assign61030_e95152_d_n9;
        locals.var_xmp_dn10 = assign61030_e95152_d_n10;
        locals.var_xmp_dn11 = assign61030_e95152_d_n11;
        locals.var_xmp_dn14 = assign61030_e95152_d_n14;

        let (assign61040_e95163, assign61040_e95163_d_n0, assign61040_e95163_d_n2, assign61040_e95163_d_n4, assign61040_e95163_d_n5, assign61040_e95163_d_n6, assign61040_e95163_d_n7, assign61040_e95163_d_n8, assign61040_e95163_d_n9, assign61040_e95163_d_n10, assign61040_e95163_d_n11, assign61040_e95163_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61040_e95161: f64 = (locals.var_xp * locals.var_x2);
        (assign61040_e95161, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61040_e95163;
        locals.var_xp_dn0 = assign61040_e95163_d_n0;
        locals.var_xp_dn2 = assign61040_e95163_d_n2;
        locals.var_xp_dn4 = assign61040_e95163_d_n4;
        locals.var_xp_dn5 = assign61040_e95163_d_n5;
        locals.var_xp_dn6 = assign61040_e95163_d_n6;
        locals.var_xp_dn7 = assign61040_e95163_d_n7;
        locals.var_xp_dn8 = assign61040_e95163_d_n8;
        locals.var_xp_dn9 = assign61040_e95163_d_n9;
        locals.var_xp_dn10 = assign61040_e95163_d_n10;
        locals.var_xp_dn11 = assign61040_e95163_d_n11;
        locals.var_xp_dn14 = assign61040_e95163_d_n14;

        let (assign61050_e95174, assign61050_e95174_d_n0, assign61050_e95174_d_n2, assign61050_e95174_d_n4, assign61050_e95174_d_n5, assign61050_e95174_d_n6, assign61050_e95174_d_n7, assign61050_e95174_d_n8, assign61050_e95174_d_n9, assign61050_e95174_d_n10, assign61050_e95174_d_n11, assign61050_e95174_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61050_e95172: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61050_e95172, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61050_e95174;
        locals.var_xmp_dn0 = assign61050_e95174_d_n0;
        locals.var_xmp_dn2 = assign61050_e95174_d_n2;
        locals.var_xmp_dn4 = assign61050_e95174_d_n4;
        locals.var_xmp_dn5 = assign61050_e95174_d_n5;
        locals.var_xmp_dn6 = assign61050_e95174_d_n6;
        locals.var_xmp_dn7 = assign61050_e95174_d_n7;
        locals.var_xmp_dn8 = assign61050_e95174_d_n8;
        locals.var_xmp_dn9 = assign61050_e95174_d_n9;
        locals.var_xmp_dn10 = assign61050_e95174_d_n10;
        locals.var_xmp_dn11 = assign61050_e95174_d_n11;
        locals.var_xmp_dn14 = assign61050_e95174_d_n14;

        let (assign61060_e95185, assign61060_e95185_d_n0, assign61060_e95185_d_n2, assign61060_e95185_d_n4, assign61060_e95185_d_n5, assign61060_e95185_d_n6, assign61060_e95185_d_n7, assign61060_e95185_d_n8, assign61060_e95185_d_n9, assign61060_e95185_d_n10, assign61060_e95185_d_n11, assign61060_e95185_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61060_e95183: f64 = (locals.var_xp * locals.var_x2);
        (assign61060_e95183, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61060_e95185;
        locals.var_xp_dn0 = assign61060_e95185_d_n0;
        locals.var_xp_dn2 = assign61060_e95185_d_n2;
        locals.var_xp_dn4 = assign61060_e95185_d_n4;
        locals.var_xp_dn5 = assign61060_e95185_d_n5;
        locals.var_xp_dn6 = assign61060_e95185_d_n6;
        locals.var_xp_dn7 = assign61060_e95185_d_n7;
        locals.var_xp_dn8 = assign61060_e95185_d_n8;
        locals.var_xp_dn9 = assign61060_e95185_d_n9;
        locals.var_xp_dn10 = assign61060_e95185_d_n10;
        locals.var_xp_dn11 = assign61060_e95185_d_n11;
        locals.var_xp_dn14 = assign61060_e95185_d_n14;

        let (assign61070_e95196, assign61070_e95196_d_n0, assign61070_e95196_d_n2, assign61070_e95196_d_n4, assign61070_e95196_d_n5, assign61070_e95196_d_n6, assign61070_e95196_d_n7, assign61070_e95196_d_n8, assign61070_e95196_d_n9, assign61070_e95196_d_n10, assign61070_e95196_d_n11, assign61070_e95196_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61070_e95194: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61070_e95194, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61070_e95196;
        locals.var_xmp_dn0 = assign61070_e95196_d_n0;
        locals.var_xmp_dn2 = assign61070_e95196_d_n2;
        locals.var_xmp_dn4 = assign61070_e95196_d_n4;
        locals.var_xmp_dn5 = assign61070_e95196_d_n5;
        locals.var_xmp_dn6 = assign61070_e95196_d_n6;
        locals.var_xmp_dn7 = assign61070_e95196_d_n7;
        locals.var_xmp_dn8 = assign61070_e95196_d_n8;
        locals.var_xmp_dn9 = assign61070_e95196_d_n9;
        locals.var_xmp_dn10 = assign61070_e95196_d_n10;
        locals.var_xmp_dn11 = assign61070_e95196_d_n11;
        locals.var_xmp_dn14 = assign61070_e95196_d_n14;

        let (assign61080_e95207, assign61080_e95207_d_n0, assign61080_e95207_d_n2, assign61080_e95207_d_n4, assign61080_e95207_d_n5, assign61080_e95207_d_n6, assign61080_e95207_d_n7, assign61080_e95207_d_n8, assign61080_e95207_d_n9, assign61080_e95207_d_n10, assign61080_e95207_d_n11, assign61080_e95207_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61080_e95205: f64 = (locals.var_xp + locals.var_xmp);
        (assign61080_e95205, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61080_e95207;
        locals.var_arg_dn0 = assign61080_e95207_d_n0;
        locals.var_arg_dn2 = assign61080_e95207_d_n2;
        locals.var_arg_dn4 = assign61080_e95207_d_n4;
        locals.var_arg_dn5 = assign61080_e95207_d_n5;
        locals.var_arg_dn6 = assign61080_e95207_d_n6;
        locals.var_arg_dn7 = assign61080_e95207_d_n7;
        locals.var_arg_dn8 = assign61080_e95207_d_n8;
        locals.var_arg_dn9 = assign61080_e95207_d_n9;
        locals.var_arg_dn10 = assign61080_e95207_d_n10;
        locals.var_arg_dn11 = assign61080_e95207_d_n11;
        locals.var_arg_dn14 = assign61080_e95207_d_n14;

        let (assign61090_e95216, assign61090_e95216_d_n0, assign61090_e95216_d_n2, assign61090_e95216_d_n4, assign61090_e95216_d_n5, assign61090_e95216_d_n6, assign61090_e95216_d_n7, assign61090_e95216_d_n8, assign61090_e95216_d_n9, assign61090_e95216_d_n10, assign61090_e95216_d_n11, assign61090_e95216_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61090_e95216;
        locals.var_dnm_dn0 = assign61090_e95216_d_n0;
        locals.var_dnm_dn2 = assign61090_e95216_d_n2;
        locals.var_dnm_dn4 = assign61090_e95216_d_n4;
        locals.var_dnm_dn5 = assign61090_e95216_d_n5;
        locals.var_dnm_dn6 = assign61090_e95216_d_n6;
        locals.var_dnm_dn7 = assign61090_e95216_d_n7;
        locals.var_dnm_dn8 = assign61090_e95216_d_n8;
        locals.var_dnm_dn9 = assign61090_e95216_d_n9;
        locals.var_dnm_dn10 = assign61090_e95216_d_n10;
        locals.var_dnm_dn11 = assign61090_e95216_d_n11;
        locals.var_dnm_dn14 = assign61090_e95216_d_n14;

        let assign61100_e95231: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign61100_e95231;

        let assign61110_e95234: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign61110_e95234;

        let (assign61120_e95247,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) && (locals.var_guard1478 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61120_e95247;

        let assign61130_e95250: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign61130_e95250;

        let (assign61140_e95266,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) && (locals.var_guard1478 == 0.0)) && (locals.var_guard1479 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61140_e95266;

        let assign61150_e95269: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign61150_e95269;

        let (assign61160_e95288,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) && (locals.var_guard1478 == 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61160_e95288;

        let assign61170_e95291: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign61170_e95291;

        let (assign61180_e95313,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) && (locals.var_guard1478 == 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 == 0.0)) && (locals.var_guard1481 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61180_e95313;

        let (assign61190_e95324,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61190_e95324;

        let mut assign61200_loop_guard: usize = 0;
        while {
            let assign61200_cond_e95336: f64 = if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign61200_cond_e95336 != 0.0
        } {
            assign61200_loop_guard += 1;
            assert!(assign61200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61200_body0_e95348, assign61200_body0_e95348_d_n0, assign61200_body0_e95348_d_n2, assign61200_body0_e95348_d_n4, assign61200_body0_e95348_d_n5, assign61200_body0_e95348_d_n6, assign61200_body0_e95348_d_n7, assign61200_body0_e95348_d_n8, assign61200_body0_e95348_d_n9, assign61200_body0_e95348_d_n10, assign61200_body0_e95348_d_n11, assign61200_body0_e95348_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign61200_body0_e95346: f64 = (locals.var_dnm).sqrt();
        (assign61200_body0_e95346, (locals.var_dnm_dn0 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn2 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn4 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn5 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn6 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn7 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn8 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn9 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn10 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn11 / (2.0 * assign61200_body0_e95346)), (locals.var_dnm_dn14 / (2.0 * assign61200_body0_e95346)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign61200_body0_e95348;
            locals.var_dnm_dn0 = assign61200_body0_e95348_d_n0;
            locals.var_dnm_dn2 = assign61200_body0_e95348_d_n2;
            locals.var_dnm_dn4 = assign61200_body0_e95348_d_n4;
            locals.var_dnm_dn5 = assign61200_body0_e95348_d_n5;
            locals.var_dnm_dn6 = assign61200_body0_e95348_d_n6;
            locals.var_dnm_dn7 = assign61200_body0_e95348_d_n7;
            locals.var_dnm_dn8 = assign61200_body0_e95348_d_n8;
            locals.var_dnm_dn9 = assign61200_body0_e95348_d_n9;
            locals.var_dnm_dn10 = assign61200_body0_e95348_d_n10;
            locals.var_dnm_dn11 = assign61200_body0_e95348_d_n11;
            locals.var_dnm_dn14 = assign61200_body0_e95348_d_n14;
            let (assign61200_body1_e95361,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign61200_body1_e95359: f64 = (locals.var_m0 + 1.0);
        (assign61200_body1_e95359,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign61200_body1_e95361;
        }

        let (assign61210_e95384, assign61210_e95384_d_n0, assign61210_e95384_d_n2, assign61210_e95384_d_n4, assign61210_e95384_d_n5, assign61210_e95384_d_n6, assign61210_e95384_d_n7, assign61210_e95384_d_n8, assign61210_e95384_d_n9, assign61210_e95384_d_n10, assign61210_e95384_d_n11, assign61210_e95384_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let (assign61210_e95382, assign61210_e95382_d_n0, assign61210_e95382_d_n2, assign61210_e95382_d_n4, assign61210_e95382_d_n5, assign61210_e95382_d_n6, assign61210_e95382_d_n7, assign61210_e95382_d_n8, assign61210_e95382_d_n9, assign61210_e95382_d_n10, assign61210_e95382_d_n11, assign61210_e95382_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61210_e95379: f64 = (2.0 * 4.0);
                let assign61210_e95380: f64 = (1.0 / assign61210_e95379);
                let assign61210_e95381: f64 = (locals.var_dnm).powf(assign61210_e95380);
                (assign61210_e95381, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn0)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn2)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn4)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn5)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn6)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn7)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn8)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn9)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn10)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn11)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61210_e95380) as f64).is_finite() && ((assign61210_e95380) as f64).fract() == 0.0 { if assign61210_e95380 == 0.0 { 0.0 } else { (assign61210_e95380 * ((locals.var_dnm).powf(assign61210_e95380 - 1.0) * locals.var_dnm_dn14)) } } else { (assign61210_e95381 * (assign61210_e95380 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign61210_e95382, assign61210_e95382_d_n0, assign61210_e95382_d_n2, assign61210_e95382_d_n4, assign61210_e95382_d_n5, assign61210_e95382_d_n6, assign61210_e95382_d_n7, assign61210_e95382_d_n8, assign61210_e95382_d_n9, assign61210_e95382_d_n10, assign61210_e95382_d_n11, assign61210_e95382_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61210_e95384;
        locals.var_dnm_dn0 = assign61210_e95384_d_n0;
        locals.var_dnm_dn2 = assign61210_e95384_d_n2;
        locals.var_dnm_dn4 = assign61210_e95384_d_n4;
        locals.var_dnm_dn5 = assign61210_e95384_d_n5;
        locals.var_dnm_dn6 = assign61210_e95384_d_n6;
        locals.var_dnm_dn7 = assign61210_e95384_d_n7;
        locals.var_dnm_dn8 = assign61210_e95384_d_n8;
        locals.var_dnm_dn9 = assign61210_e95384_d_n9;
        locals.var_dnm_dn10 = assign61210_e95384_d_n10;
        locals.var_dnm_dn11 = assign61210_e95384_d_n11;
        locals.var_dnm_dn14 = assign61210_e95384_d_n14;

        let (assign61220_e95395, assign61220_e95395_d_n0, assign61220_e95395_d_n2, assign61220_e95395_d_n4, assign61220_e95395_d_n5, assign61220_e95395_d_n6, assign61220_e95395_d_n7, assign61220_e95395_d_n8, assign61220_e95395_d_n9, assign61220_e95395_d_n10, assign61220_e95395_d_n11, assign61220_e95395_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61220_e95393: f64 = (1.0 / locals.var_dnm);
        (assign61220_e95393, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61220_e95395;
        locals.var_dnm_dn0 = assign61220_e95395_d_n0;
        locals.var_dnm_dn2 = assign61220_e95395_d_n2;
        locals.var_dnm_dn4 = assign61220_e95395_d_n4;
        locals.var_dnm_dn5 = assign61220_e95395_d_n5;
        locals.var_dnm_dn6 = assign61220_e95395_d_n6;
        locals.var_dnm_dn7 = assign61220_e95395_d_n7;
        locals.var_dnm_dn8 = assign61220_e95395_d_n8;
        locals.var_dnm_dn9 = assign61220_e95395_d_n9;
        locals.var_dnm_dn10 = assign61220_e95395_d_n10;
        locals.var_dnm_dn11 = assign61220_e95395_d_n11;
        locals.var_dnm_dn14 = assign61220_e95395_d_n14;

        let (assign61230_e95408, assign61230_e95408_d_n0, assign61230_e95408_d_n2, assign61230_e95408_d_n4, assign61230_e95408_d_n5, assign61230_e95408_d_n6, assign61230_e95408_d_n7, assign61230_e95408_d_n8, assign61230_e95408_d_n9, assign61230_e95408_d_n10, assign61230_e95408_d_n11, assign61230_e95408_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61230_e95404: f64 = locals.var_tx;
        let assign61230_e95406: f64 = (assign61230_e95404 * locals.var_dnm);
        (assign61230_e95406, ((locals.var_tx_dn0 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn11)), ((locals.var_tx_dn14 * locals.var_dnm) + (assign61230_e95404 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign61230_e95408;
        locals.var_ty_dn0 = assign61230_e95408_d_n0;
        locals.var_ty_dn2 = assign61230_e95408_d_n2;
        locals.var_ty_dn4 = assign61230_e95408_d_n4;
        locals.var_ty_dn5 = assign61230_e95408_d_n5;
        locals.var_ty_dn6 = assign61230_e95408_d_n6;
        locals.var_ty_dn7 = assign61230_e95408_d_n7;
        locals.var_ty_dn8 = assign61230_e95408_d_n8;
        locals.var_ty_dn9 = assign61230_e95408_d_n9;
        locals.var_ty_dn10 = assign61230_e95408_d_n10;
        locals.var_ty_dn11 = assign61230_e95408_d_n11;
        locals.var_ty_dn14 = assign61230_e95408_d_n14;

        let (assign61240_e95423, assign61240_e95423_d_n0, assign61240_e95423_d_n2, assign61240_e95423_d_n4, assign61240_e95423_d_n5, assign61240_e95423_d_n6, assign61240_e95423_d_n7, assign61240_e95423_d_n8, assign61240_e95423_d_n9, assign61240_e95423_d_n10, assign61240_e95423_d_n11, assign61240_e95423_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61240_e95417: f64 = locals.var_xmp;
        let assign61240_e95419: f64 = (assign61240_e95417 * locals.var_dnm);
        let assign61240_e95421: f64 = (assign61240_e95419 / locals.var_arg);
        (assign61240_e95421, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn0)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn2)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn4)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn5)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn6)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn7)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn8)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn9)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn10)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn11)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign61240_e95417 * locals.var_dnm_dn14)) * locals.var_arg) - (assign61240_e95419 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign61240_e95423;
        locals.var_t4_dn0 = assign61240_e95423_d_n0;
        locals.var_t4_dn2 = assign61240_e95423_d_n2;
        locals.var_t4_dn4 = assign61240_e95423_d_n4;
        locals.var_t4_dn5 = assign61240_e95423_d_n5;
        locals.var_t4_dn6 = assign61240_e95423_d_n6;
        locals.var_t4_dn7 = assign61240_e95423_d_n7;
        locals.var_t4_dn8 = assign61240_e95423_d_n8;
        locals.var_t4_dn9 = assign61240_e95423_d_n9;
        locals.var_t4_dn10 = assign61240_e95423_d_n10;
        locals.var_t4_dn11 = assign61240_e95423_d_n11;
        locals.var_t4_dn14 = assign61240_e95423_d_n14;

        let (assign61250_e95434, assign61250_e95434_d_n0, assign61250_e95434_d_n2, assign61250_e95434_d_n4, assign61250_e95434_d_n5, assign61250_e95434_d_n6, assign61250_e95434_d_n7, assign61250_e95434_d_n8, assign61250_e95434_d_n9, assign61250_e95434_d_n10, assign61250_e95434_d_n11, assign61250_e95434_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61250_e95432: f64 = (1.0 - locals.var_ty);
        (assign61250_e95432, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn4), (-locals.var_ty_dn5), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn8), (-locals.var_ty_dn9), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn14),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    }
};
        locals.var_alpha = assign61250_e95434;
        locals.var_alpha_dn0 = assign61250_e95434_d_n0;
        locals.var_alpha_dn2 = assign61250_e95434_d_n2;
        locals.var_alpha_dn4 = assign61250_e95434_d_n4;
        locals.var_alpha_dn5 = assign61250_e95434_d_n5;
        locals.var_alpha_dn6 = assign61250_e95434_d_n6;
        locals.var_alpha_dn7 = assign61250_e95434_d_n7;
        locals.var_alpha_dn8 = assign61250_e95434_d_n8;
        locals.var_alpha_dn9 = assign61250_e95434_d_n9;
        locals.var_alpha_dn10 = assign61250_e95434_d_n10;
        locals.var_alpha_dn11 = assign61250_e95434_d_n11;
        locals.var_alpha_dn14 = assign61250_e95434_d_n14;

        let (assign61260_e95449, assign61260_e95449_d_n0, assign61260_e95449_d_n2, assign61260_e95449_d_n4, assign61260_e95449_d_n5, assign61260_e95449_d_n6, assign61260_e95449_d_n7, assign61260_e95449_d_n8, assign61260_e95449_d_n9, assign61260_e95449_d_n10, assign61260_e95449_d_n11, assign61260_e95449_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61260_e95445: f64 = (1.0 + locals.var_alpha);
        let assign61260_e95446: f64 = (locals.var_alpha * assign61260_e95445);
        let assign61260_e95447: f64 = (1.0 + assign61260_e95446);
        (assign61260_e95447, ((locals.var_alpha_dn0 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * assign61260_e95445) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn4, locals.var_qinm_dn5, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn8, locals.var_qinm_dn9, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn14,)
    }
};
        locals.var_qinm = assign61260_e95449;
        locals.var_qinm_dn0 = assign61260_e95449_d_n0;
        locals.var_qinm_dn2 = assign61260_e95449_d_n2;
        locals.var_qinm_dn4 = assign61260_e95449_d_n4;
        locals.var_qinm_dn5 = assign61260_e95449_d_n5;
        locals.var_qinm_dn6 = assign61260_e95449_d_n6;
        locals.var_qinm_dn7 = assign61260_e95449_d_n7;
        locals.var_qinm_dn8 = assign61260_e95449_d_n8;
        locals.var_qinm_dn9 = assign61260_e95449_d_n9;
        locals.var_qinm_dn10 = assign61260_e95449_d_n10;
        locals.var_qinm_dn11 = assign61260_e95449_d_n11;
        locals.var_qinm_dn14 = assign61260_e95449_d_n14;

        let assign61270_e95452: f64 = (1.0 + locals.var_alpha);
        let assign61270_e95455: f64 = (10.0 * 2.220446049250313e-16);
        let assign61270_e95458: f64 = (10.0 * 2.220446049250313e-16);
        let assign61270_e95459: f64 = (assign61270_e95455 + assign61270_e95458);
        let assign61270_e95463: f64 = (10.0 * 2.220446049250313e-16);
        let assign61270_e95466: f64 = if ((assign61270_e95452 < assign61270_e95459) && (assign61270_e95463 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign61270_e95466;

    }

    pub(super) fn stamp_transient_block_216(
        locals: &mut StampLocals,
    ) {
        let (assign61280_e95487, assign61280_e95487_d_n0, assign61280_e95487_d_n2, assign61280_e95487_d_n4, assign61280_e95487_d_n5, assign61280_e95487_d_n6, assign61280_e95487_d_n7, assign61280_e95487_d_n8, assign61280_e95487_d_n9, assign61280_e95487_d_n10, assign61280_e95487_d_n11, assign61280_e95487_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61280_e95477: f64 = (10.0 * 2.220446049250313e-16);
        let assign61280_e95480: f64 = (10.0 * 2.220446049250313e-16);
        let assign61280_e95481: f64 = (assign61280_e95477 + assign61280_e95480);
        let assign61280_e95484: f64 = (1.0 + locals.var_alpha);
        let assign61280_e95485: f64 = (assign61280_e95481 - assign61280_e95484);
        (assign61280_e95485, (-locals.var_alpha_dn0), (-locals.var_alpha_dn2), (-locals.var_alpha_dn4), (-locals.var_alpha_dn5), (-locals.var_alpha_dn6), (-locals.var_alpha_dn7), (-locals.var_alpha_dn8), (-locals.var_alpha_dn9), (-locals.var_alpha_dn10), (-locals.var_alpha_dn11), (-locals.var_alpha_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign61280_e95487;
        locals.var_tmf1_dn0 = assign61280_e95487_d_n0;
        locals.var_tmf1_dn2 = assign61280_e95487_d_n2;
        locals.var_tmf1_dn4 = assign61280_e95487_d_n4;
        locals.var_tmf1_dn5 = assign61280_e95487_d_n5;
        locals.var_tmf1_dn6 = assign61280_e95487_d_n6;
        locals.var_tmf1_dn7 = assign61280_e95487_d_n7;
        locals.var_tmf1_dn8 = assign61280_e95487_d_n8;
        locals.var_tmf1_dn9 = assign61280_e95487_d_n9;
        locals.var_tmf1_dn10 = assign61280_e95487_d_n10;
        locals.var_tmf1_dn11 = assign61280_e95487_d_n11;
        locals.var_tmf1_dn14 = assign61280_e95487_d_n14;

        let (assign61290_e95500, assign61290_e95500_d_n0, assign61290_e95500_d_n2, assign61290_e95500_d_n4, assign61290_e95500_d_n5, assign61290_e95500_d_n6, assign61290_e95500_d_n7, assign61290_e95500_d_n8, assign61290_e95500_d_n9, assign61290_e95500_d_n10, assign61290_e95500_d_n11, assign61290_e95500_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61290_e95498: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign61290_e95498, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign61290_e95500;
        locals.var_x2_dn0 = assign61290_e95500_d_n0;
        locals.var_x2_dn2 = assign61290_e95500_d_n2;
        locals.var_x2_dn4 = assign61290_e95500_d_n4;
        locals.var_x2_dn5 = assign61290_e95500_d_n5;
        locals.var_x2_dn6 = assign61290_e95500_d_n6;
        locals.var_x2_dn7 = assign61290_e95500_d_n7;
        locals.var_x2_dn8 = assign61290_e95500_d_n8;
        locals.var_x2_dn9 = assign61290_e95500_d_n9;
        locals.var_x2_dn10 = assign61290_e95500_d_n10;
        locals.var_x2_dn11 = assign61290_e95500_d_n11;
        locals.var_x2_dn14 = assign61290_e95500_d_n14;

        let (assign61300_e95517, assign61300_e95517_d_n0, assign61300_e95517_d_n2, assign61300_e95517_d_n4, assign61300_e95517_d_n5, assign61300_e95517_d_n6, assign61300_e95517_d_n7, assign61300_e95517_d_n8, assign61300_e95517_d_n9, assign61300_e95517_d_n10, assign61300_e95517_d_n11, assign61300_e95517_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61300_e95511: f64 = (10.0 * 2.220446049250313e-16);
        let assign61300_e95514: f64 = (10.0 * 2.220446049250313e-16);
        let assign61300_e95515: f64 = (assign61300_e95511 * assign61300_e95514);
        (assign61300_e95515, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign61300_e95517;
        locals.var_xmax2_dn0 = assign61300_e95517_d_n0;
        locals.var_xmax2_dn2 = assign61300_e95517_d_n2;
        locals.var_xmax2_dn4 = assign61300_e95517_d_n4;
        locals.var_xmax2_dn5 = assign61300_e95517_d_n5;
        locals.var_xmax2_dn6 = assign61300_e95517_d_n6;
        locals.var_xmax2_dn7 = assign61300_e95517_d_n7;
        locals.var_xmax2_dn8 = assign61300_e95517_d_n8;
        locals.var_xmax2_dn9 = assign61300_e95517_d_n9;
        locals.var_xmax2_dn10 = assign61300_e95517_d_n10;
        locals.var_xmax2_dn11 = assign61300_e95517_d_n11;
        locals.var_xmax2_dn14 = assign61300_e95517_d_n14;

        let (assign61310_e95528, assign61310_e95528_d_n0, assign61310_e95528_d_n2, assign61310_e95528_d_n4, assign61310_e95528_d_n5, assign61310_e95528_d_n6, assign61310_e95528_d_n7, assign61310_e95528_d_n8, assign61310_e95528_d_n9, assign61310_e95528_d_n10, assign61310_e95528_d_n11, assign61310_e95528_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61310_e95528;
        locals.var_xp_dn0 = assign61310_e95528_d_n0;
        locals.var_xp_dn2 = assign61310_e95528_d_n2;
        locals.var_xp_dn4 = assign61310_e95528_d_n4;
        locals.var_xp_dn5 = assign61310_e95528_d_n5;
        locals.var_xp_dn6 = assign61310_e95528_d_n6;
        locals.var_xp_dn7 = assign61310_e95528_d_n7;
        locals.var_xp_dn8 = assign61310_e95528_d_n8;
        locals.var_xp_dn9 = assign61310_e95528_d_n9;
        locals.var_xp_dn10 = assign61310_e95528_d_n10;
        locals.var_xp_dn11 = assign61310_e95528_d_n11;
        locals.var_xp_dn14 = assign61310_e95528_d_n14;

        let (assign61320_e95539, assign61320_e95539_d_n0, assign61320_e95539_d_n2, assign61320_e95539_d_n4, assign61320_e95539_d_n5, assign61320_e95539_d_n6, assign61320_e95539_d_n7, assign61320_e95539_d_n8, assign61320_e95539_d_n9, assign61320_e95539_d_n10, assign61320_e95539_d_n11, assign61320_e95539_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61320_e95539;
        locals.var_xmp_dn0 = assign61320_e95539_d_n0;
        locals.var_xmp_dn2 = assign61320_e95539_d_n2;
        locals.var_xmp_dn4 = assign61320_e95539_d_n4;
        locals.var_xmp_dn5 = assign61320_e95539_d_n5;
        locals.var_xmp_dn6 = assign61320_e95539_d_n6;
        locals.var_xmp_dn7 = assign61320_e95539_d_n7;
        locals.var_xmp_dn8 = assign61320_e95539_d_n8;
        locals.var_xmp_dn9 = assign61320_e95539_d_n9;
        locals.var_xmp_dn10 = assign61320_e95539_d_n10;
        locals.var_xmp_dn11 = assign61320_e95539_d_n11;
        locals.var_xmp_dn14 = assign61320_e95539_d_n14;

        let (assign61330_e95550,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61330_e95550;

        let (assign61340_e95561,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61340_e95561;

        let (assign61350_e95572, assign61350_e95572_d_n0, assign61350_e95572_d_n2, assign61350_e95572_d_n4, assign61350_e95572_d_n5, assign61350_e95572_d_n6, assign61350_e95572_d_n7, assign61350_e95572_d_n8, assign61350_e95572_d_n9, assign61350_e95572_d_n10, assign61350_e95572_d_n11, assign61350_e95572_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61350_e95572;
        locals.var_arg_dn0 = assign61350_e95572_d_n0;
        locals.var_arg_dn2 = assign61350_e95572_d_n2;
        locals.var_arg_dn4 = assign61350_e95572_d_n4;
        locals.var_arg_dn5 = assign61350_e95572_d_n5;
        locals.var_arg_dn6 = assign61350_e95572_d_n6;
        locals.var_arg_dn7 = assign61350_e95572_d_n7;
        locals.var_arg_dn8 = assign61350_e95572_d_n8;
        locals.var_arg_dn9 = assign61350_e95572_d_n9;
        locals.var_arg_dn10 = assign61350_e95572_d_n10;
        locals.var_arg_dn11 = assign61350_e95572_d_n11;
        locals.var_arg_dn14 = assign61350_e95572_d_n14;

        let (assign61360_e95583, assign61360_e95583_d_n0, assign61360_e95583_d_n2, assign61360_e95583_d_n4, assign61360_e95583_d_n5, assign61360_e95583_d_n6, assign61360_e95583_d_n7, assign61360_e95583_d_n8, assign61360_e95583_d_n9, assign61360_e95583_d_n10, assign61360_e95583_d_n11, assign61360_e95583_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61360_e95583;
        locals.var_dnm_dn0 = assign61360_e95583_d_n0;
        locals.var_dnm_dn2 = assign61360_e95583_d_n2;
        locals.var_dnm_dn4 = assign61360_e95583_d_n4;
        locals.var_dnm_dn5 = assign61360_e95583_d_n5;
        locals.var_dnm_dn6 = assign61360_e95583_d_n6;
        locals.var_dnm_dn7 = assign61360_e95583_d_n7;
        locals.var_dnm_dn8 = assign61360_e95583_d_n8;
        locals.var_dnm_dn9 = assign61360_e95583_d_n9;
        locals.var_dnm_dn10 = assign61360_e95583_d_n10;
        locals.var_dnm_dn11 = assign61360_e95583_d_n11;
        locals.var_dnm_dn14 = assign61360_e95583_d_n14;

        let (assign61370_e95596, assign61370_e95596_d_n0, assign61370_e95596_d_n2, assign61370_e95596_d_n4, assign61370_e95596_d_n5, assign61370_e95596_d_n6, assign61370_e95596_d_n7, assign61370_e95596_d_n8, assign61370_e95596_d_n9, assign61370_e95596_d_n10, assign61370_e95596_d_n11, assign61370_e95596_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61370_e95594: f64 = (locals.var_xp * locals.var_x2);
        (assign61370_e95594, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61370_e95596;
        locals.var_xp_dn0 = assign61370_e95596_d_n0;
        locals.var_xp_dn2 = assign61370_e95596_d_n2;
        locals.var_xp_dn4 = assign61370_e95596_d_n4;
        locals.var_xp_dn5 = assign61370_e95596_d_n5;
        locals.var_xp_dn6 = assign61370_e95596_d_n6;
        locals.var_xp_dn7 = assign61370_e95596_d_n7;
        locals.var_xp_dn8 = assign61370_e95596_d_n8;
        locals.var_xp_dn9 = assign61370_e95596_d_n9;
        locals.var_xp_dn10 = assign61370_e95596_d_n10;
        locals.var_xp_dn11 = assign61370_e95596_d_n11;
        locals.var_xp_dn14 = assign61370_e95596_d_n14;

        let (assign61380_e95609, assign61380_e95609_d_n0, assign61380_e95609_d_n2, assign61380_e95609_d_n4, assign61380_e95609_d_n5, assign61380_e95609_d_n6, assign61380_e95609_d_n7, assign61380_e95609_d_n8, assign61380_e95609_d_n9, assign61380_e95609_d_n10, assign61380_e95609_d_n11, assign61380_e95609_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61380_e95607: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61380_e95607, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61380_e95609;
        locals.var_xmp_dn0 = assign61380_e95609_d_n0;
        locals.var_xmp_dn2 = assign61380_e95609_d_n2;
        locals.var_xmp_dn4 = assign61380_e95609_d_n4;
        locals.var_xmp_dn5 = assign61380_e95609_d_n5;
        locals.var_xmp_dn6 = assign61380_e95609_d_n6;
        locals.var_xmp_dn7 = assign61380_e95609_d_n7;
        locals.var_xmp_dn8 = assign61380_e95609_d_n8;
        locals.var_xmp_dn9 = assign61380_e95609_d_n9;
        locals.var_xmp_dn10 = assign61380_e95609_d_n10;
        locals.var_xmp_dn11 = assign61380_e95609_d_n11;
        locals.var_xmp_dn14 = assign61380_e95609_d_n14;

        let (assign61390_e95622, assign61390_e95622_d_n0, assign61390_e95622_d_n2, assign61390_e95622_d_n4, assign61390_e95622_d_n5, assign61390_e95622_d_n6, assign61390_e95622_d_n7, assign61390_e95622_d_n8, assign61390_e95622_d_n9, assign61390_e95622_d_n10, assign61390_e95622_d_n11, assign61390_e95622_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61390_e95620: f64 = (locals.var_xp * locals.var_x2);
        (assign61390_e95620, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61390_e95622;
        locals.var_xp_dn0 = assign61390_e95622_d_n0;
        locals.var_xp_dn2 = assign61390_e95622_d_n2;
        locals.var_xp_dn4 = assign61390_e95622_d_n4;
        locals.var_xp_dn5 = assign61390_e95622_d_n5;
        locals.var_xp_dn6 = assign61390_e95622_d_n6;
        locals.var_xp_dn7 = assign61390_e95622_d_n7;
        locals.var_xp_dn8 = assign61390_e95622_d_n8;
        locals.var_xp_dn9 = assign61390_e95622_d_n9;
        locals.var_xp_dn10 = assign61390_e95622_d_n10;
        locals.var_xp_dn11 = assign61390_e95622_d_n11;
        locals.var_xp_dn14 = assign61390_e95622_d_n14;

        let (assign61400_e95635, assign61400_e95635_d_n0, assign61400_e95635_d_n2, assign61400_e95635_d_n4, assign61400_e95635_d_n5, assign61400_e95635_d_n6, assign61400_e95635_d_n7, assign61400_e95635_d_n8, assign61400_e95635_d_n9, assign61400_e95635_d_n10, assign61400_e95635_d_n11, assign61400_e95635_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61400_e95633: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61400_e95633, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61400_e95635;
        locals.var_xmp_dn0 = assign61400_e95635_d_n0;
        locals.var_xmp_dn2 = assign61400_e95635_d_n2;
        locals.var_xmp_dn4 = assign61400_e95635_d_n4;
        locals.var_xmp_dn5 = assign61400_e95635_d_n5;
        locals.var_xmp_dn6 = assign61400_e95635_d_n6;
        locals.var_xmp_dn7 = assign61400_e95635_d_n7;
        locals.var_xmp_dn8 = assign61400_e95635_d_n8;
        locals.var_xmp_dn9 = assign61400_e95635_d_n9;
        locals.var_xmp_dn10 = assign61400_e95635_d_n10;
        locals.var_xmp_dn11 = assign61400_e95635_d_n11;
        locals.var_xmp_dn14 = assign61400_e95635_d_n14;

        let (assign61410_e95648, assign61410_e95648_d_n0, assign61410_e95648_d_n2, assign61410_e95648_d_n4, assign61410_e95648_d_n5, assign61410_e95648_d_n6, assign61410_e95648_d_n7, assign61410_e95648_d_n8, assign61410_e95648_d_n9, assign61410_e95648_d_n10, assign61410_e95648_d_n11, assign61410_e95648_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61410_e95646: f64 = (locals.var_xp + locals.var_xmp);
        (assign61410_e95646, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61410_e95648;
        locals.var_arg_dn0 = assign61410_e95648_d_n0;
        locals.var_arg_dn2 = assign61410_e95648_d_n2;
        locals.var_arg_dn4 = assign61410_e95648_d_n4;
        locals.var_arg_dn5 = assign61410_e95648_d_n5;
        locals.var_arg_dn6 = assign61410_e95648_d_n6;
        locals.var_arg_dn7 = assign61410_e95648_d_n7;
        locals.var_arg_dn8 = assign61410_e95648_d_n8;
        locals.var_arg_dn9 = assign61410_e95648_d_n9;
        locals.var_arg_dn10 = assign61410_e95648_d_n10;
        locals.var_arg_dn11 = assign61410_e95648_d_n11;
        locals.var_arg_dn14 = assign61410_e95648_d_n14;

        let (assign61420_e95659, assign61420_e95659_d_n0, assign61420_e95659_d_n2, assign61420_e95659_d_n4, assign61420_e95659_d_n5, assign61420_e95659_d_n6, assign61420_e95659_d_n7, assign61420_e95659_d_n8, assign61420_e95659_d_n9, assign61420_e95659_d_n10, assign61420_e95659_d_n11, assign61420_e95659_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61420_e95659;
        locals.var_dnm_dn0 = assign61420_e95659_d_n0;
        locals.var_dnm_dn2 = assign61420_e95659_d_n2;
        locals.var_dnm_dn4 = assign61420_e95659_d_n4;
        locals.var_dnm_dn5 = assign61420_e95659_d_n5;
        locals.var_dnm_dn6 = assign61420_e95659_d_n6;
        locals.var_dnm_dn7 = assign61420_e95659_d_n7;
        locals.var_dnm_dn8 = assign61420_e95659_d_n8;
        locals.var_dnm_dn9 = assign61420_e95659_d_n9;
        locals.var_dnm_dn10 = assign61420_e95659_d_n10;
        locals.var_dnm_dn11 = assign61420_e95659_d_n11;
        locals.var_dnm_dn14 = assign61420_e95659_d_n14;

        let assign61430_e95674: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign61430_e95674;

        let assign61440_e95677: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign61440_e95677;

        let (assign61450_e95692,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61450_e95692;

        let assign61460_e95695: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign61460_e95695;

        let (assign61470_e95713,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) && (locals.var_guard1484 == 0.0)) && (locals.var_guard1485 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61470_e95713;

        let assign61480_e95716: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign61480_e95716;

        let (assign61490_e95737,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) && (locals.var_guard1484 == 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61490_e95737;

        let assign61500_e95740: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign61500_e95740;

        let (assign61510_e95764,) = {
    if (((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) && (locals.var_guard1484 == 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1487 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61510_e95764;

        let (assign61520_e95777,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61520_e95777;

        let mut assign61530_loop_guard: usize = 0;
        while {
            let assign61530_cond_e95791: f64 = if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign61530_cond_e95791 != 0.0
        } {
            assign61530_loop_guard += 1;
            assert!(assign61530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61530_body0_e95805, assign61530_body0_e95805_d_n0, assign61530_body0_e95805_d_n2, assign61530_body0_e95805_d_n4, assign61530_body0_e95805_d_n5, assign61530_body0_e95805_d_n6, assign61530_body0_e95805_d_n7, assign61530_body0_e95805_d_n8, assign61530_body0_e95805_d_n9, assign61530_body0_e95805_d_n10, assign61530_body0_e95805_d_n11, assign61530_body0_e95805_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) {
        let assign61530_body0_e95803: f64 = (locals.var_dnm).sqrt();
        (assign61530_body0_e95803, (locals.var_dnm_dn0 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn2 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn4 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn5 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn6 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn7 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn8 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn9 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn10 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn11 / (2.0 * assign61530_body0_e95803)), (locals.var_dnm_dn14 / (2.0 * assign61530_body0_e95803)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign61530_body0_e95805;
            locals.var_dnm_dn0 = assign61530_body0_e95805_d_n0;
            locals.var_dnm_dn2 = assign61530_body0_e95805_d_n2;
            locals.var_dnm_dn4 = assign61530_body0_e95805_d_n4;
            locals.var_dnm_dn5 = assign61530_body0_e95805_d_n5;
            locals.var_dnm_dn6 = assign61530_body0_e95805_d_n6;
            locals.var_dnm_dn7 = assign61530_body0_e95805_d_n7;
            locals.var_dnm_dn8 = assign61530_body0_e95805_d_n8;
            locals.var_dnm_dn9 = assign61530_body0_e95805_d_n9;
            locals.var_dnm_dn10 = assign61530_body0_e95805_d_n10;
            locals.var_dnm_dn11 = assign61530_body0_e95805_d_n11;
            locals.var_dnm_dn14 = assign61530_body0_e95805_d_n14;
            let (assign61530_body1_e95820,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 != 0.0)) {
        let assign61530_body1_e95818: f64 = (locals.var_m0 + 1.0);
        (assign61530_body1_e95818,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign61530_body1_e95820;
        }

        let (assign61540_e95845, assign61540_e95845_d_n0, assign61540_e95845_d_n2, assign61540_e95845_d_n4, assign61540_e95845_d_n5, assign61540_e95845_d_n6, assign61540_e95845_d_n7, assign61540_e95845_d_n8, assign61540_e95845_d_n9, assign61540_e95845_d_n10, assign61540_e95845_d_n11, assign61540_e95845_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) && (locals.var_guard1483 == 0.0)) {
        let (assign61540_e95843, assign61540_e95843_d_n0, assign61540_e95843_d_n2, assign61540_e95843_d_n4, assign61540_e95843_d_n5, assign61540_e95843_d_n6, assign61540_e95843_d_n7, assign61540_e95843_d_n8, assign61540_e95843_d_n9, assign61540_e95843_d_n10, assign61540_e95843_d_n11, assign61540_e95843_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61540_e95840: f64 = (2.0 * 2.0);
                let assign61540_e95841: f64 = (1.0 / assign61540_e95840);
                let assign61540_e95842: f64 = (locals.var_dnm).powf(assign61540_e95841);
                (assign61540_e95842, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn0)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn2)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn4)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn5)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn6)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn7)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn8)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn9)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn10)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn11)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61540_e95841) as f64).is_finite() && ((assign61540_e95841) as f64).fract() == 0.0 { if assign61540_e95841 == 0.0 { 0.0 } else { (assign61540_e95841 * ((locals.var_dnm).powf(assign61540_e95841 - 1.0) * locals.var_dnm_dn14)) } } else { (assign61540_e95842 * (assign61540_e95841 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign61540_e95843, assign61540_e95843_d_n0, assign61540_e95843_d_n2, assign61540_e95843_d_n4, assign61540_e95843_d_n5, assign61540_e95843_d_n6, assign61540_e95843_d_n7, assign61540_e95843_d_n8, assign61540_e95843_d_n9, assign61540_e95843_d_n10, assign61540_e95843_d_n11, assign61540_e95843_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61540_e95845;
        locals.var_dnm_dn0 = assign61540_e95845_d_n0;
        locals.var_dnm_dn2 = assign61540_e95845_d_n2;
        locals.var_dnm_dn4 = assign61540_e95845_d_n4;
        locals.var_dnm_dn5 = assign61540_e95845_d_n5;
        locals.var_dnm_dn6 = assign61540_e95845_d_n6;
        locals.var_dnm_dn7 = assign61540_e95845_d_n7;
        locals.var_dnm_dn8 = assign61540_e95845_d_n8;
        locals.var_dnm_dn9 = assign61540_e95845_d_n9;
        locals.var_dnm_dn10 = assign61540_e95845_d_n10;
        locals.var_dnm_dn11 = assign61540_e95845_d_n11;
        locals.var_dnm_dn14 = assign61540_e95845_d_n14;

        let (assign61550_e95858, assign61550_e95858_d_n0, assign61550_e95858_d_n2, assign61550_e95858_d_n4, assign61550_e95858_d_n5, assign61550_e95858_d_n6, assign61550_e95858_d_n7, assign61550_e95858_d_n8, assign61550_e95858_d_n9, assign61550_e95858_d_n10, assign61550_e95858_d_n11, assign61550_e95858_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61550_e95856: f64 = (1.0 / locals.var_dnm);
        (assign61550_e95856, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61550_e95858;
        locals.var_dnm_dn0 = assign61550_e95858_d_n0;
        locals.var_dnm_dn2 = assign61550_e95858_d_n2;
        locals.var_dnm_dn4 = assign61550_e95858_d_n4;
        locals.var_dnm_dn5 = assign61550_e95858_d_n5;
        locals.var_dnm_dn6 = assign61550_e95858_d_n6;
        locals.var_dnm_dn7 = assign61550_e95858_d_n7;
        locals.var_dnm_dn8 = assign61550_e95858_d_n8;
        locals.var_dnm_dn9 = assign61550_e95858_d_n9;
        locals.var_dnm_dn10 = assign61550_e95858_d_n10;
        locals.var_dnm_dn11 = assign61550_e95858_d_n11;
        locals.var_dnm_dn14 = assign61550_e95858_d_n14;

        let (assign61560_e95875, assign61560_e95875_d_n0, assign61560_e95875_d_n2, assign61560_e95875_d_n4, assign61560_e95875_d_n5, assign61560_e95875_d_n6, assign61560_e95875_d_n7, assign61560_e95875_d_n8, assign61560_e95875_d_n9, assign61560_e95875_d_n10, assign61560_e95875_d_n11, assign61560_e95875_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61560_e95870: f64 = (10.0 * 2.220446049250313e-16);
        let assign61560_e95871: f64 = (locals.var_tmf1 * assign61560_e95870);
        let assign61560_e95873: f64 = (assign61560_e95871 * locals.var_dnm);
        (assign61560_e95873, (((locals.var_tmf1_dn0 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign61560_e95870) * locals.var_dnm) + (assign61560_e95871 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign61560_e95875;
        locals.var_tmf0_dn0 = assign61560_e95875_d_n0;
        locals.var_tmf0_dn2 = assign61560_e95875_d_n2;
        locals.var_tmf0_dn4 = assign61560_e95875_d_n4;
        locals.var_tmf0_dn5 = assign61560_e95875_d_n5;
        locals.var_tmf0_dn6 = assign61560_e95875_d_n6;
        locals.var_tmf0_dn7 = assign61560_e95875_d_n7;
        locals.var_tmf0_dn8 = assign61560_e95875_d_n8;
        locals.var_tmf0_dn9 = assign61560_e95875_d_n9;
        locals.var_tmf0_dn10 = assign61560_e95875_d_n10;
        locals.var_tmf0_dn11 = assign61560_e95875_d_n11;
        locals.var_tmf0_dn14 = assign61560_e95875_d_n14;

        let (assign61570_e95894, assign61570_e95894_d_n0, assign61570_e95894_d_n2, assign61570_e95894_d_n4, assign61570_e95894_d_n5, assign61570_e95894_d_n6, assign61570_e95894_d_n7, assign61570_e95894_d_n8, assign61570_e95894_d_n9, assign61570_e95894_d_n10, assign61570_e95894_d_n11, assign61570_e95894_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61570_e95886: f64 = (10.0 * 2.220446049250313e-16);
        let assign61570_e95888: f64 = (assign61570_e95886 * locals.var_xmp);
        let assign61570_e95890: f64 = (assign61570_e95888 * locals.var_dnm);
        let assign61570_e95892: f64 = (assign61570_e95890 / locals.var_arg);
        (assign61570_e95892, ((((((assign61570_e95886 * locals.var_xmp_dn0) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn0)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn2) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn2)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn4) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn4)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn5) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn5)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn6) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn6)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn7) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn7)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn8) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn8)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn9) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn9)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn10) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn10)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn11) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn11)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign61570_e95886 * locals.var_xmp_dn14) * locals.var_dnm) + (assign61570_e95888 * locals.var_dnm_dn14)) * locals.var_arg) - (assign61570_e95890 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61570_e95894;
        locals.var_t0_dn0 = assign61570_e95894_d_n0;
        locals.var_t0_dn2 = assign61570_e95894_d_n2;
        locals.var_t0_dn4 = assign61570_e95894_d_n4;
        locals.var_t0_dn5 = assign61570_e95894_d_n5;
        locals.var_t0_dn6 = assign61570_e95894_d_n6;
        locals.var_t0_dn7 = assign61570_e95894_d_n7;
        locals.var_t0_dn8 = assign61570_e95894_d_n8;
        locals.var_t0_dn9 = assign61570_e95894_d_n9;
        locals.var_t0_dn10 = assign61570_e95894_d_n10;
        locals.var_t0_dn11 = assign61570_e95894_d_n11;
        locals.var_t0_dn14 = assign61570_e95894_d_n14;

        let (assign61580_e95913, assign61580_e95913_d_n0, assign61580_e95913_d_n2, assign61580_e95913_d_n4, assign61580_e95913_d_n5, assign61580_e95913_d_n6, assign61580_e95913_d_n7, assign61580_e95913_d_n8, assign61580_e95913_d_n9, assign61580_e95913_d_n10, assign61580_e95913_d_n11, assign61580_e95913_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign61580_e95905: f64 = (10.0 * 2.220446049250313e-16);
        let assign61580_e95908: f64 = (10.0 * 2.220446049250313e-16);
        let assign61580_e95909: f64 = (assign61580_e95905 + assign61580_e95908);
        let assign61580_e95911: f64 = (assign61580_e95909 - locals.var_tmf0);
        (assign61580_e95911, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn4, locals.var_qidn_dn5, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn8, locals.var_qidn_dn9, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn14,)
    }
};
        locals.var_qidn = assign61580_e95913;
        locals.var_qidn_dn0 = assign61580_e95913_d_n0;
        locals.var_qidn_dn2 = assign61580_e95913_d_n2;
        locals.var_qidn_dn4 = assign61580_e95913_d_n4;
        locals.var_qidn_dn5 = assign61580_e95913_d_n5;
        locals.var_qidn_dn6 = assign61580_e95913_d_n6;
        locals.var_qidn_dn7 = assign61580_e95913_d_n7;
        locals.var_qidn_dn8 = assign61580_e95913_d_n8;
        locals.var_qidn_dn9 = assign61580_e95913_d_n9;
        locals.var_qidn_dn10 = assign61580_e95913_d_n10;
        locals.var_qidn_dn11 = assign61580_e95913_d_n11;
        locals.var_qidn_dn14 = assign61580_e95913_d_n14;

    }

    pub(super) fn stamp_transient_block_217(
        locals: &mut StampLocals,
    ) {
        let (assign61590_e95924, assign61590_e95924_d_n0, assign61590_e95924_d_n2, assign61590_e95924_d_n4, assign61590_e95924_d_n5, assign61590_e95924_d_n6, assign61590_e95924_d_n7, assign61590_e95924_d_n8, assign61590_e95924_d_n9, assign61590_e95924_d_n10, assign61590_e95924_d_n11, assign61590_e95924_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61590_e95924;
        locals.var_t0_dn0 = assign61590_e95924_d_n0;
        locals.var_t0_dn2 = assign61590_e95924_d_n2;
        locals.var_t0_dn4 = assign61590_e95924_d_n4;
        locals.var_t0_dn5 = assign61590_e95924_d_n5;
        locals.var_t0_dn6 = assign61590_e95924_d_n6;
        locals.var_t0_dn7 = assign61590_e95924_d_n7;
        locals.var_t0_dn8 = assign61590_e95924_d_n8;
        locals.var_t0_dn9 = assign61590_e95924_d_n9;
        locals.var_t0_dn10 = assign61590_e95924_d_n10;
        locals.var_t0_dn11 = assign61590_e95924_d_n11;
        locals.var_t0_dn14 = assign61590_e95924_d_n14;

        let (assign61600_e95938, assign61600_e95938_d_n0, assign61600_e95938_d_n2, assign61600_e95938_d_n4, assign61600_e95938_d_n5, assign61600_e95938_d_n6, assign61600_e95938_d_n7, assign61600_e95938_d_n8, assign61600_e95938_d_n9, assign61600_e95938_d_n10, assign61600_e95938_d_n11, assign61600_e95938_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        let assign61600_e95936: f64 = (1.0 + locals.var_alpha);
        (assign61600_e95936, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn4, locals.var_qidn_dn5, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn8, locals.var_qidn_dn9, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn14,)
    }
};
        locals.var_qidn = assign61600_e95938;
        locals.var_qidn_dn0 = assign61600_e95938_d_n0;
        locals.var_qidn_dn2 = assign61600_e95938_d_n2;
        locals.var_qidn_dn4 = assign61600_e95938_d_n4;
        locals.var_qidn_dn5 = assign61600_e95938_d_n5;
        locals.var_qidn_dn6 = assign61600_e95938_d_n6;
        locals.var_qidn_dn7 = assign61600_e95938_d_n7;
        locals.var_qidn_dn8 = assign61600_e95938_d_n8;
        locals.var_qidn_dn9 = assign61600_e95938_d_n9;
        locals.var_qidn_dn10 = assign61600_e95938_d_n10;
        locals.var_qidn_dn11 = assign61600_e95938_d_n11;
        locals.var_qidn_dn14 = assign61600_e95938_d_n14;

        let (assign61610_e95950, assign61610_e95950_d_n0, assign61610_e95950_d_n2, assign61610_e95950_d_n4, assign61610_e95950_d_n5, assign61610_e95950_d_n6, assign61610_e95950_d_n7, assign61610_e95950_d_n8, assign61610_e95950_d_n9, assign61610_e95950_d_n10, assign61610_e95950_d_n11, assign61610_e95950_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61610_e95950;
        locals.var_t0_dn0 = assign61610_e95950_d_n0;
        locals.var_t0_dn2 = assign61610_e95950_d_n2;
        locals.var_t0_dn4 = assign61610_e95950_d_n4;
        locals.var_t0_dn5 = assign61610_e95950_d_n5;
        locals.var_t0_dn6 = assign61610_e95950_d_n6;
        locals.var_t0_dn7 = assign61610_e95950_d_n7;
        locals.var_t0_dn8 = assign61610_e95950_d_n8;
        locals.var_t0_dn9 = assign61610_e95950_d_n9;
        locals.var_t0_dn10 = assign61610_e95950_d_n10;
        locals.var_t0_dn11 = assign61610_e95950_d_n11;
        locals.var_t0_dn14 = assign61610_e95950_d_n14;

        let (assign61620_e95965, assign61620_e95965_d_n0, assign61620_e95965_d_n2, assign61620_e95965_d_n4, assign61620_e95965_d_n5, assign61620_e95965_d_n6, assign61620_e95965_d_n7, assign61620_e95965_d_n8, assign61620_e95965_d_n9, assign61620_e95965_d_n10, assign61620_e95965_d_n11, assign61620_e95965_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61620_e95959: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign61620_e95961: f64 = (assign61620_e95959 * locals.var_qinm);
        let assign61620_e95963: f64 = (assign61620_e95961 / locals.var_qidn);
        (assign61620_e95963, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn4) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn4)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn4)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn5) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn5)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn5)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn8) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn8)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn8)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn9) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn9)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn9)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn14) * locals.var_qinm) + (assign61620_e95959 * locals.var_qinm_dn14)) * locals.var_qidn) - (assign61620_e95961 * locals.var_qidn_dn14)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61620_e95965;
        locals.var_t1_dn0 = assign61620_e95965_d_n0;
        locals.var_t1_dn2 = assign61620_e95965_d_n2;
        locals.var_t1_dn4 = assign61620_e95965_d_n4;
        locals.var_t1_dn5 = assign61620_e95965_d_n5;
        locals.var_t1_dn6 = assign61620_e95965_d_n6;
        locals.var_t1_dn7 = assign61620_e95965_d_n7;
        locals.var_t1_dn8 = assign61620_e95965_d_n8;
        locals.var_t1_dn9 = assign61620_e95965_d_n9;
        locals.var_t1_dn10 = assign61620_e95965_d_n10;
        locals.var_t1_dn11 = assign61620_e95965_d_n11;
        locals.var_t1_dn14 = assign61620_e95965_d_n14;

        let (assign61630_e95976, assign61630_e95976_d_n0, assign61630_e95976_d_n2, assign61630_e95976_d_n4, assign61630_e95976_d_n5, assign61630_e95976_d_n6, assign61630_e95976_d_n7, assign61630_e95976_d_n8, assign61630_e95976_d_n9, assign61630_e95976_d_n10, assign61630_e95976_d_n11, assign61630_e95976_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61630_e95974: f64 = (locals.var_t1 * locals.var_cox);
        (assign61630_e95974, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61630_e95976;
        locals.var_qiu_dn0 = assign61630_e95976_d_n0;
        locals.var_qiu_dn2 = assign61630_e95976_d_n2;
        locals.var_qiu_dn4 = assign61630_e95976_d_n4;
        locals.var_qiu_dn5 = assign61630_e95976_d_n5;
        locals.var_qiu_dn6 = assign61630_e95976_d_n6;
        locals.var_qiu_dn7 = assign61630_e95976_d_n7;
        locals.var_qiu_dn8 = assign61630_e95976_d_n8;
        locals.var_qiu_dn9 = assign61630_e95976_d_n9;
        locals.var_qiu_dn10 = assign61630_e95976_d_n10;
        locals.var_qiu_dn11 = assign61630_e95976_d_n11;
        locals.var_qiu_dn14 = assign61630_e95976_d_n14;

        let (assign61640_e95987, assign61640_e95987_d_n0, assign61640_e95987_d_n2, assign61640_e95987_d_n4, assign61640_e95987_d_n5, assign61640_e95987_d_n6, assign61640_e95987_d_n7, assign61640_e95987_d_n8, assign61640_e95987_d_n9, assign61640_e95987_d_n10, assign61640_e95987_d_n11, assign61640_e95987_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61640_e95985: f64 = (0.5 + locals.var_alpha);
        (assign61640_e95985, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn4, locals.var_qdnm_dn5, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn8, locals.var_qdnm_dn9, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn14,)
    }
};
        locals.var_qdnm = assign61640_e95987;
        locals.var_qdnm_dn0 = assign61640_e95987_d_n0;
        locals.var_qdnm_dn2 = assign61640_e95987_d_n2;
        locals.var_qdnm_dn4 = assign61640_e95987_d_n4;
        locals.var_qdnm_dn5 = assign61640_e95987_d_n5;
        locals.var_qdnm_dn6 = assign61640_e95987_d_n6;
        locals.var_qdnm_dn7 = assign61640_e95987_d_n7;
        locals.var_qdnm_dn8 = assign61640_e95987_d_n8;
        locals.var_qdnm_dn9 = assign61640_e95987_d_n9;
        locals.var_qdnm_dn10 = assign61640_e95987_d_n10;
        locals.var_qdnm_dn11 = assign61640_e95987_d_n11;
        locals.var_qdnm_dn14 = assign61640_e95987_d_n14;

        let (assign61650_e95998, assign61650_e95998_d_n0, assign61650_e95998_d_n2, assign61650_e95998_d_n4, assign61650_e95998_d_n5, assign61650_e95998_d_n6, assign61650_e95998_d_n7, assign61650_e95998_d_n8, assign61650_e95998_d_n9, assign61650_e95998_d_n10, assign61650_e95998_d_n11, assign61650_e95998_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61650_e95996: f64 = (locals.var_qidn * locals.var_qinm);
        (assign61650_e95996, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn4 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn4)), ((locals.var_qidn_dn5 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn5)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn8 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn8)), ((locals.var_qidn_dn9 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn9)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn14 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn14)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn4, locals.var_qddn_dn5, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn8, locals.var_qddn_dn9, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn14,)
    }
};
        locals.var_qddn = assign61650_e95998;
        locals.var_qddn_dn0 = assign61650_e95998_d_n0;
        locals.var_qddn_dn2 = assign61650_e95998_d_n2;
        locals.var_qddn_dn4 = assign61650_e95998_d_n4;
        locals.var_qddn_dn5 = assign61650_e95998_d_n5;
        locals.var_qddn_dn6 = assign61650_e95998_d_n6;
        locals.var_qddn_dn7 = assign61650_e95998_d_n7;
        locals.var_qddn_dn8 = assign61650_e95998_d_n8;
        locals.var_qddn_dn9 = assign61650_e95998_d_n9;
        locals.var_qddn_dn10 = assign61650_e95998_d_n10;
        locals.var_qddn_dn11 = assign61650_e95998_d_n11;
        locals.var_qddn_dn14 = assign61650_e95998_d_n14;

        let (assign61660_e96011, assign61660_e96011_d_n0, assign61660_e96011_d_n2, assign61660_e96011_d_n4, assign61660_e96011_d_n5, assign61660_e96011_d_n6, assign61660_e96011_d_n7, assign61660_e96011_d_n8, assign61660_e96011_d_n9, assign61660_e96011_d_n10, assign61660_e96011_d_n11, assign61660_e96011_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61660_e96007: f64 = (0.4 * locals.var_qdnm);
        let assign61660_e96009: f64 = (assign61660_e96007 / locals.var_qddn);
        (assign61660_e96009, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn4) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn4)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn5) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn5)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn8) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn8)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn9) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn9)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn14) * locals.var_qddn) - (assign61660_e96007 * locals.var_qddn_dn14)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn4, locals.var_quot_dn5, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn8, locals.var_quot_dn9, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn14,)
    }
};
        locals.var_quot = assign61660_e96011;
        locals.var_quot_dn0 = assign61660_e96011_d_n0;
        locals.var_quot_dn2 = assign61660_e96011_d_n2;
        locals.var_quot_dn4 = assign61660_e96011_d_n4;
        locals.var_quot_dn5 = assign61660_e96011_d_n5;
        locals.var_quot_dn6 = assign61660_e96011_d_n6;
        locals.var_quot_dn7 = assign61660_e96011_d_n7;
        locals.var_quot_dn8 = assign61660_e96011_d_n8;
        locals.var_quot_dn9 = assign61660_e96011_d_n9;
        locals.var_quot_dn10 = assign61660_e96011_d_n10;
        locals.var_quot_dn11 = assign61660_e96011_d_n11;
        locals.var_quot_dn14 = assign61660_e96011_d_n14;

        let (assign61670_e96022, assign61670_e96022_d_n0, assign61670_e96022_d_n2, assign61670_e96022_d_n4, assign61670_e96022_d_n5, assign61670_e96022_d_n6, assign61670_e96022_d_n7, assign61670_e96022_d_n8, assign61670_e96022_d_n9, assign61670_e96022_d_n10, assign61670_e96022_d_n11, assign61670_e96022_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign61670_e96020: f64 = (0.6 - locals.var_quot);
        (assign61670_e96020, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn4), (-locals.var_quot_dn5), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn8), (-locals.var_quot_dn9), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn14),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61670_e96022;
        locals.var_qdrat_dn0 = assign61670_e96022_d_n0;
        locals.var_qdrat_dn2 = assign61670_e96022_d_n2;
        locals.var_qdrat_dn4 = assign61670_e96022_d_n4;
        locals.var_qdrat_dn5 = assign61670_e96022_d_n5;
        locals.var_qdrat_dn6 = assign61670_e96022_d_n6;
        locals.var_qdrat_dn7 = assign61670_e96022_d_n7;
        locals.var_qdrat_dn8 = assign61670_e96022_d_n8;
        locals.var_qdrat_dn9 = assign61670_e96022_d_n9;
        locals.var_qdrat_dn10 = assign61670_e96022_d_n10;
        locals.var_qdrat_dn11 = assign61670_e96022_d_n11;
        locals.var_qdrat_dn14 = assign61670_e96022_d_n14;

        let assign61680_e96025: f64 = if locals.var_qdrat > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign61680_e96025;

        let (assign61690_e96036, assign61690_e96036_d_n0, assign61690_e96036_d_n2, assign61690_e96036_d_n4, assign61690_e96036_d_n5, assign61690_e96036_d_n6, assign61690_e96036_d_n7, assign61690_e96036_d_n8, assign61690_e96036_d_n9, assign61690_e96036_d_n10, assign61690_e96036_d_n11, assign61690_e96036_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1488 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61690_e96036;
        locals.var_qdrat_dn0 = assign61690_e96036_d_n0;
        locals.var_qdrat_dn2 = assign61690_e96036_d_n2;
        locals.var_qdrat_dn4 = assign61690_e96036_d_n4;
        locals.var_qdrat_dn5 = assign61690_e96036_d_n5;
        locals.var_qdrat_dn6 = assign61690_e96036_d_n6;
        locals.var_qdrat_dn7 = assign61690_e96036_d_n7;
        locals.var_qdrat_dn8 = assign61690_e96036_d_n8;
        locals.var_qdrat_dn9 = assign61690_e96036_d_n9;
        locals.var_qdrat_dn10 = assign61690_e96036_d_n10;
        locals.var_qdrat_dn11 = assign61690_e96036_d_n11;
        locals.var_qdrat_dn14 = assign61690_e96036_d_n14;

        let assign61700_e96039: f64 = if locals.var_flg_zone == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign61700_e96039;

        let (assign61710_e96050, assign61710_e96050_d_n0, assign61710_e96050_d_n2, assign61710_e96050_d_n4, assign61710_e96050_d_n5, assign61710_e96050_d_n6, assign61710_e96050_d_n7, assign61710_e96050_d_n8, assign61710_e96050_d_n9, assign61710_e96050_d_n10, assign61710_e96050_d_n11, assign61710_e96050_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61710_e96050;
        locals.var_t1_dn0 = assign61710_e96050_d_n0;
        locals.var_t1_dn2 = assign61710_e96050_d_n2;
        locals.var_t1_dn4 = assign61710_e96050_d_n4;
        locals.var_t1_dn5 = assign61710_e96050_d_n5;
        locals.var_t1_dn6 = assign61710_e96050_d_n6;
        locals.var_t1_dn7 = assign61710_e96050_d_n7;
        locals.var_t1_dn8 = assign61710_e96050_d_n8;
        locals.var_t1_dn9 = assign61710_e96050_d_n9;
        locals.var_t1_dn10 = assign61710_e96050_d_n10;
        locals.var_t1_dn11 = assign61710_e96050_d_n11;
        locals.var_t1_dn14 = assign61710_e96050_d_n14;

        let (assign61720_e96069, assign61720_e96069_d_n0, assign61720_e96069_d_n2, assign61720_e96069_d_n4, assign61720_e96069_d_n5, assign61720_e96069_d_n6, assign61720_e96069_d_n7, assign61720_e96069_d_n8, assign61720_e96069_d_n9, assign61720_e96069_d_n10, assign61720_e96069_d_n11, assign61720_e96069_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign61720_e96061: f64 = (locals.var_fd2 * locals.var_qbu);
        let assign61720_e96064: f64 = (1.0 - locals.var_fd2);
        let assign61720_e96066: f64 = (assign61720_e96064 * locals.var_qb0);
        let assign61720_e96067: f64 = (assign61720_e96061 + assign61720_e96066);
        (assign61720_e96067, (((locals.var_fd2_dn0 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn0))), (((locals.var_fd2_dn2 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn2))), (((locals.var_fd2_dn4 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn4))), (((locals.var_fd2_dn5 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn5))), (((locals.var_fd2_dn6 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn6))), (((locals.var_fd2_dn7 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn7))), (((locals.var_fd2_dn8 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn8))), (((locals.var_fd2_dn9 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn9))), (((locals.var_fd2_dn10 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn10))), (((locals.var_fd2_dn11 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn11)) + (((-locals.var_fd2_dn11) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn11))), (((locals.var_fd2_dn14 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn14)) + (((-locals.var_fd2_dn14) * locals.var_qb0) + (assign61720_e96064 * locals.var_qb0_dn14))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign61720_e96069;
        locals.var_qbu_dn0 = assign61720_e96069_d_n0;
        locals.var_qbu_dn2 = assign61720_e96069_d_n2;
        locals.var_qbu_dn4 = assign61720_e96069_d_n4;
        locals.var_qbu_dn5 = assign61720_e96069_d_n5;
        locals.var_qbu_dn6 = assign61720_e96069_d_n6;
        locals.var_qbu_dn7 = assign61720_e96069_d_n7;
        locals.var_qbu_dn8 = assign61720_e96069_d_n8;
        locals.var_qbu_dn9 = assign61720_e96069_d_n9;
        locals.var_qbu_dn10 = assign61720_e96069_d_n10;
        locals.var_qbu_dn11 = assign61720_e96069_d_n11;
        locals.var_qbu_dn14 = assign61720_e96069_d_n14;

        let assign61730_e96072: f64 = if locals.var_qbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign61730_e96072;

        let (assign61740_e96085, assign61740_e96085_d_n0, assign61740_e96085_d_n2, assign61740_e96085_d_n4, assign61740_e96085_d_n5, assign61740_e96085_d_n6, assign61740_e96085_d_n7, assign61740_e96085_d_n8, assign61740_e96085_d_n9, assign61740_e96085_d_n10, assign61740_e96085_d_n11, assign61740_e96085_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) && (locals.var_guard1490 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign61740_e96085;
        locals.var_qbu_dn0 = assign61740_e96085_d_n0;
        locals.var_qbu_dn2 = assign61740_e96085_d_n2;
        locals.var_qbu_dn4 = assign61740_e96085_d_n4;
        locals.var_qbu_dn5 = assign61740_e96085_d_n5;
        locals.var_qbu_dn6 = assign61740_e96085_d_n6;
        locals.var_qbu_dn7 = assign61740_e96085_d_n7;
        locals.var_qbu_dn8 = assign61740_e96085_d_n8;
        locals.var_qbu_dn9 = assign61740_e96085_d_n9;
        locals.var_qbu_dn10 = assign61740_e96085_d_n10;
        locals.var_qbu_dn11 = assign61740_e96085_d_n11;
        locals.var_qbu_dn14 = assign61740_e96085_d_n14;

        let (assign61750_e96096, assign61750_e96096_d_n0, assign61750_e96096_d_n2, assign61750_e96096_d_n4, assign61750_e96096_d_n5, assign61750_e96096_d_n6, assign61750_e96096_d_n7, assign61750_e96096_d_n8, assign61750_e96096_d_n9, assign61750_e96096_d_n10, assign61750_e96096_d_n11, assign61750_e96096_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61750_e96096;
        locals.var_t1_dn0 = assign61750_e96096_d_n0;
        locals.var_t1_dn2 = assign61750_e96096_d_n2;
        locals.var_t1_dn4 = assign61750_e96096_d_n4;
        locals.var_t1_dn5 = assign61750_e96096_d_n5;
        locals.var_t1_dn6 = assign61750_e96096_d_n6;
        locals.var_t1_dn7 = assign61750_e96096_d_n7;
        locals.var_t1_dn8 = assign61750_e96096_d_n8;
        locals.var_t1_dn9 = assign61750_e96096_d_n9;
        locals.var_t1_dn10 = assign61750_e96096_d_n10;
        locals.var_t1_dn11 = assign61750_e96096_d_n11;
        locals.var_t1_dn14 = assign61750_e96096_d_n14;

        let (assign61760_e96115, assign61760_e96115_d_n0, assign61760_e96115_d_n2, assign61760_e96115_d_n4, assign61760_e96115_d_n5, assign61760_e96115_d_n6, assign61760_e96115_d_n7, assign61760_e96115_d_n8, assign61760_e96115_d_n9, assign61760_e96115_d_n10, assign61760_e96115_d_n11, assign61760_e96115_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign61760_e96107: f64 = (locals.var_fd2 * locals.var_qiu);
        let assign61760_e96110: f64 = (1.0 - locals.var_fd2);
        let assign61760_e96112: f64 = (assign61760_e96110 * locals.var_qn0);
        let assign61760_e96113: f64 = (assign61760_e96107 + assign61760_e96112);
        (assign61760_e96113, (((locals.var_fd2_dn0 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn0))), (((locals.var_fd2_dn2 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn2))), (((locals.var_fd2_dn4 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn4))), (((locals.var_fd2_dn5 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn5))), (((locals.var_fd2_dn6 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn6))), (((locals.var_fd2_dn7 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn7))), (((locals.var_fd2_dn8 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn8))), (((locals.var_fd2_dn9 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn9))), (((locals.var_fd2_dn10 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn10))), (((locals.var_fd2_dn11 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn11)) + (((-locals.var_fd2_dn11) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn11))), (((locals.var_fd2_dn14 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn14)) + (((-locals.var_fd2_dn14) * locals.var_qn0) + (assign61760_e96110 * locals.var_qn0_dn14))),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61760_e96115;
        locals.var_qiu_dn0 = assign61760_e96115_d_n0;
        locals.var_qiu_dn2 = assign61760_e96115_d_n2;
        locals.var_qiu_dn4 = assign61760_e96115_d_n4;
        locals.var_qiu_dn5 = assign61760_e96115_d_n5;
        locals.var_qiu_dn6 = assign61760_e96115_d_n6;
        locals.var_qiu_dn7 = assign61760_e96115_d_n7;
        locals.var_qiu_dn8 = assign61760_e96115_d_n8;
        locals.var_qiu_dn9 = assign61760_e96115_d_n9;
        locals.var_qiu_dn10 = assign61760_e96115_d_n10;
        locals.var_qiu_dn11 = assign61760_e96115_d_n11;
        locals.var_qiu_dn14 = assign61760_e96115_d_n14;

        let assign61770_e96118: f64 = if locals.var_qiu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign61770_e96118;

        let (assign61780_e96131, assign61780_e96131_d_n0, assign61780_e96131_d_n2, assign61780_e96131_d_n4, assign61780_e96131_d_n5, assign61780_e96131_d_n6, assign61780_e96131_d_n7, assign61780_e96131_d_n8, assign61780_e96131_d_n9, assign61780_e96131_d_n10, assign61780_e96131_d_n11, assign61780_e96131_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61780_e96131;
        locals.var_qiu_dn0 = assign61780_e96131_d_n0;
        locals.var_qiu_dn2 = assign61780_e96131_d_n2;
        locals.var_qiu_dn4 = assign61780_e96131_d_n4;
        locals.var_qiu_dn5 = assign61780_e96131_d_n5;
        locals.var_qiu_dn6 = assign61780_e96131_d_n6;
        locals.var_qiu_dn7 = assign61780_e96131_d_n7;
        locals.var_qiu_dn8 = assign61780_e96131_d_n8;
        locals.var_qiu_dn9 = assign61780_e96131_d_n9;
        locals.var_qiu_dn10 = assign61780_e96131_d_n10;
        locals.var_qiu_dn11 = assign61780_e96131_d_n11;
        locals.var_qiu_dn14 = assign61780_e96131_d_n14;

        let (assign61790_e96142, assign61790_e96142_d_n0, assign61790_e96142_d_n2, assign61790_e96142_d_n4, assign61790_e96142_d_n5, assign61790_e96142_d_n6, assign61790_e96142_d_n7, assign61790_e96142_d_n8, assign61790_e96142_d_n9, assign61790_e96142_d_n10, assign61790_e96142_d_n11, assign61790_e96142_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61790_e96142;
        locals.var_t1_dn0 = assign61790_e96142_d_n0;
        locals.var_t1_dn2 = assign61790_e96142_d_n2;
        locals.var_t1_dn4 = assign61790_e96142_d_n4;
        locals.var_t1_dn5 = assign61790_e96142_d_n5;
        locals.var_t1_dn6 = assign61790_e96142_d_n6;
        locals.var_t1_dn7 = assign61790_e96142_d_n7;
        locals.var_t1_dn8 = assign61790_e96142_d_n8;
        locals.var_t1_dn9 = assign61790_e96142_d_n9;
        locals.var_t1_dn10 = assign61790_e96142_d_n10;
        locals.var_t1_dn11 = assign61790_e96142_d_n11;
        locals.var_t1_dn14 = assign61790_e96142_d_n14;

        let (assign61800_e96161, assign61800_e96161_d_n0, assign61800_e96161_d_n2, assign61800_e96161_d_n4, assign61800_e96161_d_n5, assign61800_e96161_d_n6, assign61800_e96161_d_n7, assign61800_e96161_d_n8, assign61800_e96161_d_n9, assign61800_e96161_d_n10, assign61800_e96161_d_n11, assign61800_e96161_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign61800_e96153: f64 = (locals.var_fd2 * locals.var_qdrat);
        let assign61800_e96156: f64 = (1.0 - locals.var_fd2);
        let assign61800_e96158: f64 = (assign61800_e96156 * 0.5);
        let assign61800_e96159: f64 = (assign61800_e96153 + assign61800_e96158);
        (assign61800_e96159, (((locals.var_fd2_dn0 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn0)) + ((-locals.var_fd2_dn0) * 0.5)), (((locals.var_fd2_dn2 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn2)) + ((-locals.var_fd2_dn2) * 0.5)), (((locals.var_fd2_dn4 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn4)) + ((-locals.var_fd2_dn4) * 0.5)), (((locals.var_fd2_dn5 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn5)) + ((-locals.var_fd2_dn5) * 0.5)), (((locals.var_fd2_dn6 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn6)) + ((-locals.var_fd2_dn6) * 0.5)), (((locals.var_fd2_dn7 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn7)) + ((-locals.var_fd2_dn7) * 0.5)), (((locals.var_fd2_dn8 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn8)) + ((-locals.var_fd2_dn8) * 0.5)), (((locals.var_fd2_dn9 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn9)) + ((-locals.var_fd2_dn9) * 0.5)), (((locals.var_fd2_dn10 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn10)) + ((-locals.var_fd2_dn10) * 0.5)), (((locals.var_fd2_dn11 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn11)) + ((-locals.var_fd2_dn11) * 0.5)), (((locals.var_fd2_dn14 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn14)) + ((-locals.var_fd2_dn14) * 0.5)),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61800_e96161;
        locals.var_qdrat_dn0 = assign61800_e96161_d_n0;
        locals.var_qdrat_dn2 = assign61800_e96161_d_n2;
        locals.var_qdrat_dn4 = assign61800_e96161_d_n4;
        locals.var_qdrat_dn5 = assign61800_e96161_d_n5;
        locals.var_qdrat_dn6 = assign61800_e96161_d_n6;
        locals.var_qdrat_dn7 = assign61800_e96161_d_n7;
        locals.var_qdrat_dn8 = assign61800_e96161_d_n8;
        locals.var_qdrat_dn9 = assign61800_e96161_d_n9;
        locals.var_qdrat_dn10 = assign61800_e96161_d_n10;
        locals.var_qdrat_dn11 = assign61800_e96161_d_n11;
        locals.var_qdrat_dn14 = assign61800_e96161_d_n14;

        let (assign61810_e96172, assign61810_e96172_d_n0, assign61810_e96172_d_n2, assign61810_e96172_d_n4, assign61810_e96172_d_n5, assign61810_e96172_d_n6, assign61810_e96172_d_n7, assign61810_e96172_d_n8, assign61810_e96172_d_n9, assign61810_e96172_d_n10, assign61810_e96172_d_n11, assign61810_e96172_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61810_e96172;
        locals.var_t1_dn0 = assign61810_e96172_d_n0;
        locals.var_t1_dn2 = assign61810_e96172_d_n2;
        locals.var_t1_dn4 = assign61810_e96172_d_n4;
        locals.var_t1_dn5 = assign61810_e96172_d_n5;
        locals.var_t1_dn6 = assign61810_e96172_d_n6;
        locals.var_t1_dn7 = assign61810_e96172_d_n7;
        locals.var_t1_dn8 = assign61810_e96172_d_n8;
        locals.var_t1_dn9 = assign61810_e96172_d_n9;
        locals.var_t1_dn10 = assign61810_e96172_d_n10;
        locals.var_t1_dn11 = assign61810_e96172_d_n11;
        locals.var_t1_dn14 = assign61810_e96172_d_n14;

        let (assign61820_e96185, assign61820_e96185_d_n0, assign61820_e96185_d_n2, assign61820_e96185_d_n4, assign61820_e96185_d_n5, assign61820_e96185_d_n6, assign61820_e96185_d_n7, assign61820_e96185_d_n8, assign61820_e96185_d_n9, assign61820_e96185_d_n10, assign61820_e96185_d_n11, assign61820_e96185_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1463 != 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign61820_e96183: f64 = (locals.var_fd2 * locals.var_lred);
        (assign61820_e96183, ((locals.var_fd2_dn0 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn0)), ((locals.var_fd2_dn2 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn2)), ((locals.var_fd2_dn4 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn4)), ((locals.var_fd2_dn5 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn5)), ((locals.var_fd2_dn6 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn6)), ((locals.var_fd2_dn7 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn7)), ((locals.var_fd2_dn8 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn8)), ((locals.var_fd2_dn9 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn9)), ((locals.var_fd2_dn10 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn10)), ((locals.var_fd2_dn11 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn11)), ((locals.var_fd2_dn14 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign61820_e96185;
        locals.var_lred_dn0 = assign61820_e96185_d_n0;
        locals.var_lred_dn2 = assign61820_e96185_d_n2;
        locals.var_lred_dn4 = assign61820_e96185_d_n4;
        locals.var_lred_dn5 = assign61820_e96185_d_n5;
        locals.var_lred_dn6 = assign61820_e96185_d_n6;
        locals.var_lred_dn7 = assign61820_e96185_d_n7;
        locals.var_lred_dn8 = assign61820_e96185_d_n8;
        locals.var_lred_dn9 = assign61820_e96185_d_n9;
        locals.var_lred_dn10 = assign61820_e96185_d_n10;
        locals.var_lred_dn11 = assign61820_e96185_d_n11;
        locals.var_lred_dn14 = assign61820_e96185_d_n14;

        let (assign61830_e96194,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_start_of_mobility != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign61830_e96194;

        let (assign61840_e96203, assign61840_e96203_d_n0, assign61840_e96203_d_n2, assign61840_e96203_d_n4, assign61840_e96203_d_n5, assign61840_e96203_d_n6, assign61840_e96203_d_n7, assign61840_e96203_d_n8, assign61840_e96203_d_n9, assign61840_e96203_d_n10, assign61840_e96203_d_n11, assign61840_e96203_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61840_e96201: f64 = (locals.var_leff - locals.var_lred);
        (assign61840_e96201, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn4), (-locals.var_lred_dn5), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn8), (-locals.var_lred_dn9), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn14),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign61840_e96203;
        locals.var_lch_dn0 = assign61840_e96203_d_n0;
        locals.var_lch_dn2 = assign61840_e96203_d_n2;
        locals.var_lch_dn4 = assign61840_e96203_d_n4;
        locals.var_lch_dn5 = assign61840_e96203_d_n5;
        locals.var_lch_dn6 = assign61840_e96203_d_n6;
        locals.var_lch_dn7 = assign61840_e96203_d_n7;
        locals.var_lch_dn8 = assign61840_e96203_d_n8;
        locals.var_lch_dn9 = assign61840_e96203_d_n9;
        locals.var_lch_dn10 = assign61840_e96203_d_n10;
        locals.var_lch_dn11 = assign61840_e96203_d_n11;
        locals.var_lch_dn14 = assign61840_e96203_d_n14;

        let assign61850_e96206: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign61850_e96206;

        let (assign61860_e96215, assign61860_e96215_d_n0, assign61860_e96215_d_n2, assign61860_e96215_d_n4, assign61860_e96215_d_n5, assign61860_e96215_d_n6, assign61860_e96215_d_n7, assign61860_e96215_d_n8, assign61860_e96215_d_n9, assign61860_e96215_d_n10, assign61860_e96215_d_n11, assign61860_e96215_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign61860_e96215;
        locals.var_lch_dn0 = assign61860_e96215_d_n0;
        locals.var_lch_dn2 = assign61860_e96215_d_n2;
        locals.var_lch_dn4 = assign61860_e96215_d_n4;
        locals.var_lch_dn5 = assign61860_e96215_d_n5;
        locals.var_lch_dn6 = assign61860_e96215_d_n6;
        locals.var_lch_dn7 = assign61860_e96215_d_n7;
        locals.var_lch_dn8 = assign61860_e96215_d_n8;
        locals.var_lch_dn9 = assign61860_e96215_d_n9;
        locals.var_lch_dn10 = assign61860_e96215_d_n10;
        locals.var_lch_dn11 = assign61860_e96215_d_n11;
        locals.var_lch_dn14 = assign61860_e96215_d_n14;

        let (assign61870_e96224, assign61870_e96224_d_n0, assign61870_e96224_d_n2, assign61870_e96224_d_n4, assign61870_e96224_d_n5, assign61870_e96224_d_n6, assign61870_e96224_d_n7, assign61870_e96224_d_n8, assign61870_e96224_d_n9, assign61870_e96224_d_n10, assign61870_e96224_d_n11, assign61870_e96224_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61870_e96222: f64 = (locals.var_ndep_o_esi / 100.0);
        (assign61870_e96222, (locals.var_ndep_o_esi_dn0 / 100.0), (locals.var_ndep_o_esi_dn2 / 100.0), (locals.var_ndep_o_esi_dn4 / 100.0), (locals.var_ndep_o_esi_dn5 / 100.0), (locals.var_ndep_o_esi_dn6 / 100.0), (locals.var_ndep_o_esi_dn7 / 100.0), (locals.var_ndep_o_esi_dn8 / 100.0), (locals.var_ndep_o_esi_dn9 / 100.0), (locals.var_ndep_o_esi_dn10 / 100.0), (locals.var_ndep_o_esi_dn11 / 100.0), (locals.var_ndep_o_esi_dn14 / 100.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61870_e96224;
        locals.var_t1_dn0 = assign61870_e96224_d_n0;
        locals.var_t1_dn2 = assign61870_e96224_d_n2;
        locals.var_t1_dn4 = assign61870_e96224_d_n4;
        locals.var_t1_dn5 = assign61870_e96224_d_n5;
        locals.var_t1_dn6 = assign61870_e96224_d_n6;
        locals.var_t1_dn7 = assign61870_e96224_d_n7;
        locals.var_t1_dn8 = assign61870_e96224_d_n8;
        locals.var_t1_dn9 = assign61870_e96224_d_n9;
        locals.var_t1_dn10 = assign61870_e96224_d_n10;
        locals.var_t1_dn11 = assign61870_e96224_d_n11;
        locals.var_t1_dn14 = assign61870_e96224_d_n14;

    }

    pub(super) fn stamp_transient_block_218(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign61880_e96233, assign61880_e96233_d_n0, assign61880_e96233_d_n2, assign61880_e96233_d_n4, assign61880_e96233_d_n5, assign61880_e96233_d_n6, assign61880_e96233_d_n7, assign61880_e96233_d_n8, assign61880_e96233_d_n9, assign61880_e96233_d_n10, assign61880_e96233_d_n11, assign61880_e96233_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61880_e96231: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign61880_e96231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign61880_e96233;
        locals.var_t2_dn0 = assign61880_e96233_d_n0;
        locals.var_t2_dn2 = assign61880_e96233_d_n2;
        locals.var_t2_dn4 = assign61880_e96233_d_n4;
        locals.var_t2_dn5 = assign61880_e96233_d_n5;
        locals.var_t2_dn6 = assign61880_e96233_d_n6;
        locals.var_t2_dn7 = assign61880_e96233_d_n7;
        locals.var_t2_dn8 = assign61880_e96233_d_n8;
        locals.var_t2_dn9 = assign61880_e96233_d_n9;
        locals.var_t2_dn10 = assign61880_e96233_d_n10;
        locals.var_t2_dn11 = assign61880_e96233_d_n11;
        locals.var_t2_dn14 = assign61880_e96233_d_n14;

        let (assign61890_e96240, assign61890_e96240_d_n0, assign61890_e96240_d_n2, assign61890_e96240_d_n4, assign61890_e96240_d_n5, assign61890_e96240_d_n6, assign61890_e96240_d_n7, assign61890_e96240_d_n8, assign61890_e96240_d_n9, assign61890_e96240_d_n10, assign61890_e96240_d_n11, assign61890_e96240_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61890_e96240;
        locals.var_t0_dn0 = assign61890_e96240_d_n0;
        locals.var_t0_dn2 = assign61890_e96240_d_n2;
        locals.var_t0_dn4 = assign61890_e96240_d_n4;
        locals.var_t0_dn5 = assign61890_e96240_d_n5;
        locals.var_t0_dn6 = assign61890_e96240_d_n6;
        locals.var_t0_dn7 = assign61890_e96240_d_n7;
        locals.var_t0_dn8 = assign61890_e96240_d_n8;
        locals.var_t0_dn9 = assign61890_e96240_d_n9;
        locals.var_t0_dn10 = assign61890_e96240_d_n10;
        locals.var_t0_dn11 = assign61890_e96240_d_n11;
        locals.var_t0_dn14 = assign61890_e96240_d_n14;

        let (assign61900_e96253, assign61900_e96253_d_n0, assign61900_e96253_d_n2, assign61900_e96253_d_n4, assign61900_e96253_d_n5, assign61900_e96253_d_n6, assign61900_e96253_d_n7, assign61900_e96253_d_n8, assign61900_e96253_d_n9, assign61900_e96253_d_n10, assign61900_e96253_d_n11, assign61900_e96253_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61900_e96248: f64 = (locals.var_psl - locals.var_ps0);
        let assign61900_e96250: f64 = (assign61900_e96248 * locals.var_t0);
        let assign61900_e96251: f64 = (1.0 + assign61900_e96250);
        (assign61900_e96251, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn2)), (((locals.var_psl_dn4 - locals.var_ps0_dn4) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn4)), (((locals.var_psl_dn5 - locals.var_ps0_dn5) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn5)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn7)), (((locals.var_psl_dn8 - locals.var_ps0_dn8) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn8)), (((locals.var_psl_dn9 - locals.var_ps0_dn9) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn9)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn10)), (((locals.var_psl_dn11 - locals.var_ps0_dn11) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn11)), (((locals.var_psl_dn14 - locals.var_ps0_dn14) * locals.var_t0) + (assign61900_e96248 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign61900_e96253;
        locals.var_t4_dn0 = assign61900_e96253_d_n0;
        locals.var_t4_dn2 = assign61900_e96253_d_n2;
        locals.var_t4_dn4 = assign61900_e96253_d_n4;
        locals.var_t4_dn5 = assign61900_e96253_d_n5;
        locals.var_t4_dn6 = assign61900_e96253_d_n6;
        locals.var_t4_dn7 = assign61900_e96253_d_n7;
        locals.var_t4_dn8 = assign61900_e96253_d_n8;
        locals.var_t4_dn9 = assign61900_e96253_d_n9;
        locals.var_t4_dn10 = assign61900_e96253_d_n10;
        locals.var_t4_dn11 = assign61900_e96253_d_n11;
        locals.var_t4_dn14 = assign61900_e96253_d_n14;

        let (assign61910_e96266, assign61910_e96266_d_n0, assign61910_e96266_d_n2, assign61910_e96266_d_n4, assign61910_e96266_d_n5, assign61910_e96266_d_n6, assign61910_e96266_d_n7, assign61910_e96266_d_n8, assign61910_e96266_d_n9, assign61910_e96266_d_n10, assign61910_e96266_d_n11, assign61910_e96266_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61910_e96260: f64 = (locals.var_t1 * locals.var_qbu);
        let assign61910_e96263: f64 = (locals.var_t2 * locals.var_qiu);
        let assign61910_e96264: f64 = (assign61910_e96260 + assign61910_e96263);
        (assign61910_e96264, (((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)) + ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0))), (((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)) + ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2))), (((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4)) + ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4))), (((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5)) + ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5))), (((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)) + ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6))), (((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)) + ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7))), (((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8)) + ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8))), (((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9)) + ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9))), (((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)) + ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10))), (((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)) + ((locals.var_t2_dn11 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn11))), (((locals.var_t1_dn14 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn14)) + ((locals.var_t2_dn14 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign61910_e96266;
        locals.var_t5_dn0 = assign61910_e96266_d_n0;
        locals.var_t5_dn2 = assign61910_e96266_d_n2;
        locals.var_t5_dn4 = assign61910_e96266_d_n4;
        locals.var_t5_dn5 = assign61910_e96266_d_n5;
        locals.var_t5_dn6 = assign61910_e96266_d_n6;
        locals.var_t5_dn7 = assign61910_e96266_d_n7;
        locals.var_t5_dn8 = assign61910_e96266_d_n8;
        locals.var_t5_dn9 = assign61910_e96266_d_n9;
        locals.var_t5_dn10 = assign61910_e96266_d_n10;
        locals.var_t5_dn11 = assign61910_e96266_d_n11;
        locals.var_t5_dn14 = assign61910_e96266_d_n14;

        let (assign61920_e96275, assign61920_e96275_d_n0, assign61920_e96275_d_n2, assign61920_e96275_d_n4, assign61920_e96275_d_n5, assign61920_e96275_d_n6, assign61920_e96275_d_n7, assign61920_e96275_d_n8, assign61920_e96275_d_n9, assign61920_e96275_d_n10, assign61920_e96275_d_n11, assign61920_e96275_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61920_e96273: f64 = (locals.var_t5 / locals.var_t4);
        (assign61920_e96273, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign61920_e96275;
        locals.var_t3_dn0 = assign61920_e96275_d_n0;
        locals.var_t3_dn2 = assign61920_e96275_d_n2;
        locals.var_t3_dn4 = assign61920_e96275_d_n4;
        locals.var_t3_dn5 = assign61920_e96275_d_n5;
        locals.var_t3_dn6 = assign61920_e96275_d_n6;
        locals.var_t3_dn7 = assign61920_e96275_d_n7;
        locals.var_t3_dn8 = assign61920_e96275_d_n8;
        locals.var_t3_dn9 = assign61920_e96275_d_n9;
        locals.var_t3_dn10 = assign61920_e96275_d_n10;
        locals.var_t3_dn11 = assign61920_e96275_d_n11;
        locals.var_t3_dn14 = assign61920_e96275_d_n14;

        let (assign61930_e96288, assign61930_e96288_d_n0, assign61930_e96288_d_n2, assign61930_e96288_d_n4, assign61930_e96288_d_n5, assign61930_e96288_d_n6, assign61930_e96288_d_n7, assign61930_e96288_d_n8, assign61930_e96288_d_n9, assign61930_e96288_d_n10, assign61930_e96288_d_n11, assign61930_e96288_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61930_e96284: f64 = (p.p166 * locals.var_vbsz__blk440);
        let assign61930_e96285: f64 = (1.0 + assign61930_e96284);
        let assign61930_e96286: f64 = (locals.var_t3 * assign61930_e96285);
        (assign61930_e96286, ((locals.var_t3_dn0 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn0))), ((locals.var_t3_dn2 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn2))), ((locals.var_t3_dn4 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn4))), ((locals.var_t3_dn5 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn5))), ((locals.var_t3_dn6 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn6))), ((locals.var_t3_dn7 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn7))), ((locals.var_t3_dn8 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn8))), ((locals.var_t3_dn9 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn9))), ((locals.var_t3_dn10 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn10))), ((locals.var_t3_dn11 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn11))), ((locals.var_t3_dn14 * assign61930_e96285) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk440_dn14))),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign61930_e96288;
        locals.var_eeff_dn0 = assign61930_e96288_d_n0;
        locals.var_eeff_dn2 = assign61930_e96288_d_n2;
        locals.var_eeff_dn4 = assign61930_e96288_d_n4;
        locals.var_eeff_dn5 = assign61930_e96288_d_n5;
        locals.var_eeff_dn6 = assign61930_e96288_d_n6;
        locals.var_eeff_dn7 = assign61930_e96288_d_n7;
        locals.var_eeff_dn8 = assign61930_e96288_d_n8;
        locals.var_eeff_dn9 = assign61930_e96288_d_n9;
        locals.var_eeff_dn10 = assign61930_e96288_d_n10;
        locals.var_eeff_dn11 = assign61930_e96288_d_n11;
        locals.var_eeff_dn14 = assign61930_e96288_d_n14;

        let (assign61940_e96304, assign61940_e96304_d_n0, assign61940_e96304_d_n2, assign61940_e96304_d_n4, assign61940_e96304_d_n5, assign61940_e96304_d_n6, assign61940_e96304_d_n7, assign61940_e96304_d_n8, assign61940_e96304_d_n9, assign61940_e96304_d_n10, assign61940_e96304_d_n11, assign61940_e96304_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let (assign61940_e96302, assign61940_e96302_d_n0, assign61940_e96302_d_n2, assign61940_e96302_d_n4, assign61940_e96302_d_n5, assign61940_e96302_d_n6, assign61940_e96302_d_n7, assign61940_e96302_d_n8, assign61940_e96302_d_n9, assign61940_e96302_d_n10, assign61940_e96302_d_n11, assign61940_e96302_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61940_e96300: f64 = (p.p160 - 1.0);
                let assign61940_e96301: f64 = (locals.var_eeff).powf(assign61940_e96300);
                (assign61940_e96301, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn11)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96300) as f64).is_finite() && ((assign61940_e96300) as f64).fract() == 0.0 { if assign61940_e96300 == 0.0 { 0.0 } else { (assign61940_e96300 * ((locals.var_eeff).powf(assign61940_e96300 - 1.0) * locals.var_eeff_dn14)) } } else { (assign61940_e96301 * (assign61940_e96300 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign61940_e96302, assign61940_e96302_d_n0, assign61940_e96302_d_n2, assign61940_e96302_d_n4, assign61940_e96302_d_n5, assign61940_e96302_d_n6, assign61940_e96302_d_n7, assign61940_e96302_d_n8, assign61940_e96302_d_n9, assign61940_e96302_d_n10, assign61940_e96302_d_n11, assign61940_e96302_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign61940_e96304;
        locals.var_t5_dn0 = assign61940_e96304_d_n0;
        locals.var_t5_dn2 = assign61940_e96304_d_n2;
        locals.var_t5_dn4 = assign61940_e96304_d_n4;
        locals.var_t5_dn5 = assign61940_e96304_d_n5;
        locals.var_t5_dn6 = assign61940_e96304_d_n6;
        locals.var_t5_dn7 = assign61940_e96304_d_n7;
        locals.var_t5_dn8 = assign61940_e96304_d_n8;
        locals.var_t5_dn9 = assign61940_e96304_d_n9;
        locals.var_t5_dn10 = assign61940_e96304_d_n10;
        locals.var_t5_dn11 = assign61940_e96304_d_n11;
        locals.var_t5_dn14 = assign61940_e96304_d_n14;

        let (assign61950_e96313, assign61950_e96313_d_n0, assign61950_e96313_d_n2, assign61950_e96313_d_n4, assign61950_e96313_d_n5, assign61950_e96313_d_n6, assign61950_e96313_d_n7, assign61950_e96313_d_n8, assign61950_e96313_d_n9, assign61950_e96313_d_n10, assign61950_e96313_d_n11, assign61950_e96313_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61950_e96311: f64 = (locals.var_t5 * locals.var_eeff);
        (assign61950_e96311, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign61950_e96313;
        locals.var_t8_dn0 = assign61950_e96313_d_n0;
        locals.var_t8_dn2 = assign61950_e96313_d_n2;
        locals.var_t8_dn4 = assign61950_e96313_d_n4;
        locals.var_t8_dn5 = assign61950_e96313_d_n5;
        locals.var_t8_dn6 = assign61950_e96313_d_n6;
        locals.var_t8_dn7 = assign61950_e96313_d_n7;
        locals.var_t8_dn8 = assign61950_e96313_d_n8;
        locals.var_t8_dn9 = assign61950_e96313_d_n9;
        locals.var_t8_dn10 = assign61950_e96313_d_n10;
        locals.var_t8_dn11 = assign61950_e96313_d_n11;
        locals.var_t8_dn14 = assign61950_e96313_d_n14;

        let (assign61960_e96329, assign61960_e96329_d_n0, assign61960_e96329_d_n2, assign61960_e96329_d_n4, assign61960_e96329_d_n5, assign61960_e96329_d_n6, assign61960_e96329_d_n7, assign61960_e96329_d_n8, assign61960_e96329_d_n9, assign61960_e96329_d_n10, assign61960_e96329_d_n11, assign61960_e96329_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let (assign61960_e96327, assign61960_e96327_d_n0, assign61960_e96327_d_n2, assign61960_e96327_d_n4, assign61960_e96327_d_n5, assign61960_e96327_d_n6, assign61960_e96327_d_n7, assign61960_e96327_d_n8, assign61960_e96327_d_n9, assign61960_e96327_d_n10, assign61960_e96327_d_n11, assign61960_e96327_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61960_e96325: f64 = (locals.var_muesr - 1.0);
                let assign61960_e96326: f64 = (locals.var_eeff).powf(assign61960_e96325);
                (assign61960_e96326, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn11)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96325) as f64).is_finite() && ((assign61960_e96325) as f64).fract() == 0.0 { if assign61960_e96325 == 0.0 { 0.0 } else { (assign61960_e96325 * ((locals.var_eeff).powf(assign61960_e96325 - 1.0) * locals.var_eeff_dn14)) } } else { (assign61960_e96326 * (assign61960_e96325 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign61960_e96327, assign61960_e96327_d_n0, assign61960_e96327_d_n2, assign61960_e96327_d_n4, assign61960_e96327_d_n5, assign61960_e96327_d_n6, assign61960_e96327_d_n7, assign61960_e96327_d_n8, assign61960_e96327_d_n9, assign61960_e96327_d_n10, assign61960_e96327_d_n11, assign61960_e96327_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign61960_e96329;
        locals.var_t7_dn0 = assign61960_e96329_d_n0;
        locals.var_t7_dn2 = assign61960_e96329_d_n2;
        locals.var_t7_dn4 = assign61960_e96329_d_n4;
        locals.var_t7_dn5 = assign61960_e96329_d_n5;
        locals.var_t7_dn6 = assign61960_e96329_d_n6;
        locals.var_t7_dn7 = assign61960_e96329_d_n7;
        locals.var_t7_dn8 = assign61960_e96329_d_n8;
        locals.var_t7_dn9 = assign61960_e96329_d_n9;
        locals.var_t7_dn10 = assign61960_e96329_d_n10;
        locals.var_t7_dn11 = assign61960_e96329_d_n11;
        locals.var_t7_dn14 = assign61960_e96329_d_n14;

        let (assign61970_e96338, assign61970_e96338_d_n0, assign61970_e96338_d_n2, assign61970_e96338_d_n4, assign61970_e96338_d_n5, assign61970_e96338_d_n6, assign61970_e96338_d_n7, assign61970_e96338_d_n8, assign61970_e96338_d_n9, assign61970_e96338_d_n10, assign61970_e96338_d_n11, assign61970_e96338_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61970_e96336: f64 = (locals.var_t7 * locals.var_eeff);
        (assign61970_e96336, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign61970_e96338;
        locals.var_t6_dn0 = assign61970_e96338_d_n0;
        locals.var_t6_dn2 = assign61970_e96338_d_n2;
        locals.var_t6_dn4 = assign61970_e96338_d_n4;
        locals.var_t6_dn5 = assign61970_e96338_d_n5;
        locals.var_t6_dn6 = assign61970_e96338_d_n6;
        locals.var_t6_dn7 = assign61970_e96338_d_n7;
        locals.var_t6_dn8 = assign61970_e96338_d_n8;
        locals.var_t6_dn9 = assign61970_e96338_d_n9;
        locals.var_t6_dn10 = assign61970_e96338_d_n10;
        locals.var_t6_dn11 = assign61970_e96338_d_n11;
        locals.var_t6_dn14 = assign61970_e96338_d_n14;

        let (assign61980_e96347, assign61980_e96347_d_n0, assign61980_e96347_d_n2, assign61980_e96347_d_n4, assign61980_e96347_d_n5, assign61980_e96347_d_n6, assign61980_e96347_d_n7, assign61980_e96347_d_n8, assign61980_e96347_d_n9, assign61980_e96347_d_n10, assign61980_e96347_d_n11, assign61980_e96347_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61980_e96345: f64 = (1.6021918e-19 * 10000.0);
        (assign61980_e96345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign61980_e96347;
        locals.var_t9_dn0 = assign61980_e96347_d_n0;
        locals.var_t9_dn2 = assign61980_e96347_d_n2;
        locals.var_t9_dn4 = assign61980_e96347_d_n4;
        locals.var_t9_dn5 = assign61980_e96347_d_n5;
        locals.var_t9_dn6 = assign61980_e96347_d_n6;
        locals.var_t9_dn7 = assign61980_e96347_d_n7;
        locals.var_t9_dn8 = assign61980_e96347_d_n8;
        locals.var_t9_dn9 = assign61980_e96347_d_n9;
        locals.var_t9_dn10 = assign61980_e96347_d_n10;
        locals.var_t9_dn11 = assign61980_e96347_d_n11;
        locals.var_t9_dn14 = assign61980_e96347_d_n14;

        let (assign61990_e96356, assign61990_e96356_d_n0, assign61990_e96356_d_n2, assign61990_e96356_d_n4, assign61990_e96356_d_n5, assign61990_e96356_d_n6, assign61990_e96356_d_n7, assign61990_e96356_d_n8, assign61990_e96356_d_n9, assign61990_e96356_d_n10, assign61990_e96356_d_n11, assign61990_e96356_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign61990_e96354: f64 = (locals.var_qiu / locals.var_t9);
        (assign61990_e96354, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign61990_e96356;
        locals.var_rns_dn0 = assign61990_e96356_d_n0;
        locals.var_rns_dn2 = assign61990_e96356_d_n2;
        locals.var_rns_dn4 = assign61990_e96356_d_n4;
        locals.var_rns_dn5 = assign61990_e96356_d_n5;
        locals.var_rns_dn6 = assign61990_e96356_d_n6;
        locals.var_rns_dn7 = assign61990_e96356_d_n7;
        locals.var_rns_dn8 = assign61990_e96356_d_n8;
        locals.var_rns_dn9 = assign61990_e96356_d_n9;
        locals.var_rns_dn10 = assign61990_e96356_d_n10;
        locals.var_rns_dn11 = assign61990_e96356_d_n11;
        locals.var_rns_dn14 = assign61990_e96356_d_n14;

        let (assign62000_e96379, assign62000_e96379_d_n0, assign62000_e96379_d_n2, assign62000_e96379_d_n4, assign62000_e96379_d_n5, assign62000_e96379_d_n6, assign62000_e96379_d_n7, assign62000_e96379_d_n8, assign62000_e96379_d_n9, assign62000_e96379_d_n10, assign62000_e96379_d_n11, assign62000_e96379_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62000_e96365: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign62000_e96367: f64 = (assign62000_e96365 / 100000000000.0);
        let assign62000_e96368: f64 = (locals.var_uc_muecb0 + assign62000_e96367);
        let assign62000_e96369: f64 = (1.0 / assign62000_e96368);
        let assign62000_e96372: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign62000_e96373: f64 = (assign62000_e96369 + assign62000_e96372);
        let assign62000_e96376: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign62000_e96377: f64 = (assign62000_e96373 + assign62000_e96376);
        (assign62000_e96377, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0) / (assign62000_e96368 * assign62000_e96368))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62000_e96379;
        locals.var_t1_dn0 = assign62000_e96379_d_n0;
        locals.var_t1_dn2 = assign62000_e96379_d_n2;
        locals.var_t1_dn4 = assign62000_e96379_d_n4;
        locals.var_t1_dn5 = assign62000_e96379_d_n5;
        locals.var_t1_dn6 = assign62000_e96379_d_n6;
        locals.var_t1_dn7 = assign62000_e96379_d_n7;
        locals.var_t1_dn8 = assign62000_e96379_d_n8;
        locals.var_t1_dn9 = assign62000_e96379_d_n9;
        locals.var_t1_dn10 = assign62000_e96379_d_n10;
        locals.var_t1_dn11 = assign62000_e96379_d_n11;
        locals.var_t1_dn14 = assign62000_e96379_d_n14;

        let (assign62010_e96388, assign62010_e96388_d_n0, assign62010_e96388_d_n2, assign62010_e96388_d_n4, assign62010_e96388_d_n5, assign62010_e96388_d_n6, assign62010_e96388_d_n7, assign62010_e96388_d_n8, assign62010_e96388_d_n9, assign62010_e96388_d_n10, assign62010_e96388_d_n11, assign62010_e96388_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62010_e96386: f64 = (1.0 / locals.var_t1);
        (assign62010_e96386, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign62010_e96388;
        locals.var_muun_dn0 = assign62010_e96388_d_n0;
        locals.var_muun_dn2 = assign62010_e96388_d_n2;
        locals.var_muun_dn4 = assign62010_e96388_d_n4;
        locals.var_muun_dn5 = assign62010_e96388_d_n5;
        locals.var_muun_dn6 = assign62010_e96388_d_n6;
        locals.var_muun_dn7 = assign62010_e96388_d_n7;
        locals.var_muun_dn8 = assign62010_e96388_d_n8;
        locals.var_muun_dn9 = assign62010_e96388_d_n9;
        locals.var_muun_dn10 = assign62010_e96388_d_n10;
        locals.var_muun_dn11 = assign62010_e96388_d_n11;
        locals.var_muun_dn14 = assign62010_e96388_d_n14;

        let (assign62020_e96397, assign62020_e96397_d_n0, assign62020_e96397_d_n2, assign62020_e96397_d_n4, assign62020_e96397_d_n5, assign62020_e96397_d_n6, assign62020_e96397_d_n7, assign62020_e96397_d_n8, assign62020_e96397_d_n9, assign62020_e96397_d_n10, assign62020_e96397_d_n11, assign62020_e96397_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62020_e96395: f64 = (locals.var_muun / 10000.0);
        (assign62020_e96395, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign62020_e96397;
        locals.var_muun_dn0 = assign62020_e96397_d_n0;
        locals.var_muun_dn2 = assign62020_e96397_d_n2;
        locals.var_muun_dn4 = assign62020_e96397_d_n4;
        locals.var_muun_dn5 = assign62020_e96397_d_n5;
        locals.var_muun_dn6 = assign62020_e96397_d_n6;
        locals.var_muun_dn7 = assign62020_e96397_d_n7;
        locals.var_muun_dn8 = assign62020_e96397_d_n8;
        locals.var_muun_dn9 = assign62020_e96397_d_n9;
        locals.var_muun_dn10 = assign62020_e96397_d_n10;
        locals.var_muun_dn11 = assign62020_e96397_d_n11;
        locals.var_muun_dn14 = assign62020_e96397_d_n14;

        let (assign62030_e96410, assign62030_e96410_d_n0, assign62030_e96410_d_n2, assign62030_e96410_d_n4, assign62030_e96410_d_n5, assign62030_e96410_d_n6, assign62030_e96410_d_n7, assign62030_e96410_d_n8, assign62030_e96410_d_n9, assign62030_e96410_d_n10, assign62030_e96410_d_n11, assign62030_e96410_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62030_e96405: f64 = (locals.var_qn0 + 1e-25);
        let assign62030_e96406: f64 = (locals.var_beta * assign62030_e96405);
        let assign62030_e96408: f64 = (assign62030_e96406 * locals.var_lch);
        (assign62030_e96408, ((((locals.var_beta_dn0 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign62030_e96405) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign62030_e96406 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62030_e96410;
        locals.var_t2_dn0 = assign62030_e96410_d_n0;
        locals.var_t2_dn2 = assign62030_e96410_d_n2;
        locals.var_t2_dn4 = assign62030_e96410_d_n4;
        locals.var_t2_dn5 = assign62030_e96410_d_n5;
        locals.var_t2_dn6 = assign62030_e96410_d_n6;
        locals.var_t2_dn7 = assign62030_e96410_d_n7;
        locals.var_t2_dn8 = assign62030_e96410_d_n8;
        locals.var_t2_dn9 = assign62030_e96410_d_n9;
        locals.var_t2_dn10 = assign62030_e96410_d_n10;
        locals.var_t2_dn11 = assign62030_e96410_d_n11;
        locals.var_t2_dn14 = assign62030_e96410_d_n14;

        let (assign62040_e96419, assign62040_e96419_d_n0, assign62040_e96419_d_n2, assign62040_e96419_d_n4, assign62040_e96419_d_n5, assign62040_e96419_d_n6, assign62040_e96419_d_n7, assign62040_e96419_d_n8, assign62040_e96419_d_n9, assign62040_e96419_d_n10, assign62040_e96419_d_n11, assign62040_e96419_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62040_e96417: f64 = (1.0 / locals.var_t2);
        (assign62040_e96417, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62040_e96419;
        locals.var_t1_dn0 = assign62040_e96419_d_n0;
        locals.var_t1_dn2 = assign62040_e96419_d_n2;
        locals.var_t1_dn4 = assign62040_e96419_d_n4;
        locals.var_t1_dn5 = assign62040_e96419_d_n5;
        locals.var_t1_dn6 = assign62040_e96419_d_n6;
        locals.var_t1_dn7 = assign62040_e96419_d_n7;
        locals.var_t1_dn8 = assign62040_e96419_d_n8;
        locals.var_t1_dn9 = assign62040_e96419_d_n9;
        locals.var_t1_dn10 = assign62040_e96419_d_n10;
        locals.var_t1_dn11 = assign62040_e96419_d_n11;
        locals.var_t1_dn14 = assign62040_e96419_d_n14;

        let (assign62050_e96428, assign62050_e96428_d_n0, assign62050_e96428_d_n2, assign62050_e96428_d_n4, assign62050_e96428_d_n5, assign62050_e96428_d_n6, assign62050_e96428_d_n7, assign62050_e96428_d_n8, assign62050_e96428_d_n9, assign62050_e96428_d_n10, assign62050_e96428_d_n11, assign62050_e96428_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62050_e96426: f64 = (locals.var_t1 * locals.var_t1);
        (assign62050_e96426, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62050_e96428;
        locals.var_t3_dn0 = assign62050_e96428_d_n0;
        locals.var_t3_dn2 = assign62050_e96428_d_n2;
        locals.var_t3_dn4 = assign62050_e96428_d_n4;
        locals.var_t3_dn5 = assign62050_e96428_d_n5;
        locals.var_t3_dn6 = assign62050_e96428_d_n6;
        locals.var_t3_dn7 = assign62050_e96428_d_n7;
        locals.var_t3_dn8 = assign62050_e96428_d_n8;
        locals.var_t3_dn9 = assign62050_e96428_d_n9;
        locals.var_t3_dn10 = assign62050_e96428_d_n10;
        locals.var_t3_dn11 = assign62050_e96428_d_n11;
        locals.var_t3_dn14 = assign62050_e96428_d_n14;

        let (assign62060_e96438, assign62060_e96438_d_n0, assign62060_e96438_d_n2, assign62060_e96438_d_n4, assign62060_e96438_d_n5, assign62060_e96438_d_n6, assign62060_e96438_d_n7, assign62060_e96438_d_n8, assign62060_e96438_d_n9, assign62060_e96438_d_n10, assign62060_e96438_d_n11, assign62060_e96438_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62060_e96434: f64 = (-locals.var_beta);
        let assign62060_e96436: f64 = (assign62060_e96434 * locals.var_t3);
        (assign62060_e96436, (((-locals.var_beta_dn0) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn0)), (((-locals.var_beta_dn2) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn2)), (((-locals.var_beta_dn4) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn4)), (((-locals.var_beta_dn5) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn5)), (((-locals.var_beta_dn6) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn6)), (((-locals.var_beta_dn7) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn7)), (((-locals.var_beta_dn8) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn8)), (((-locals.var_beta_dn9) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn9)), (((-locals.var_beta_dn10) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn10)), (((-locals.var_beta_dn11) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn11)), (((-locals.var_beta_dn14) * locals.var_t3) + (assign62060_e96434 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62060_e96438;
        locals.var_t4_dn0 = assign62060_e96438_d_n0;
        locals.var_t4_dn2 = assign62060_e96438_d_n2;
        locals.var_t4_dn4 = assign62060_e96438_d_n4;
        locals.var_t4_dn5 = assign62060_e96438_d_n5;
        locals.var_t4_dn6 = assign62060_e96438_d_n6;
        locals.var_t4_dn7 = assign62060_e96438_d_n7;
        locals.var_t4_dn8 = assign62060_e96438_d_n8;
        locals.var_t4_dn9 = assign62060_e96438_d_n9;
        locals.var_t4_dn10 = assign62060_e96438_d_n10;
        locals.var_t4_dn11 = assign62060_e96438_d_n11;
        locals.var_t4_dn14 = assign62060_e96438_d_n14;

        let (assign62070_e96447, assign62070_e96447_d_n0, assign62070_e96447_d_n2, assign62070_e96447_d_n4, assign62070_e96447_d_n5, assign62070_e96447_d_n6, assign62070_e96447_d_n7, assign62070_e96447_d_n8, assign62070_e96447_d_n9, assign62070_e96447_d_n10, assign62070_e96447_d_n11, assign62070_e96447_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62070_e96445: f64 = (locals.var_t4 * locals.var_lch);
        (assign62070_e96445, ((locals.var_t4_dn0 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn0)), ((locals.var_t4_dn2 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn2)), ((locals.var_t4_dn4 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn4)), ((locals.var_t4_dn5 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn5)), ((locals.var_t4_dn6 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn6)), ((locals.var_t4_dn7 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn7)), ((locals.var_t4_dn8 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn8)), ((locals.var_t4_dn9 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn9)), ((locals.var_t4_dn10 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn10)), ((locals.var_t4_dn11 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn11)), ((locals.var_t4_dn14 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62070_e96447;
        locals.var_t5_dn0 = assign62070_e96447_d_n0;
        locals.var_t5_dn2 = assign62070_e96447_d_n2;
        locals.var_t5_dn4 = assign62070_e96447_d_n4;
        locals.var_t5_dn5 = assign62070_e96447_d_n5;
        locals.var_t5_dn6 = assign62070_e96447_d_n6;
        locals.var_t5_dn7 = assign62070_e96447_d_n7;
        locals.var_t5_dn8 = assign62070_e96447_d_n8;
        locals.var_t5_dn9 = assign62070_e96447_d_n9;
        locals.var_t5_dn10 = assign62070_e96447_d_n10;
        locals.var_t5_dn11 = assign62070_e96447_d_n11;
        locals.var_t5_dn14 = assign62070_e96447_d_n14;

        let (assign62080_e96458, assign62080_e96458_d_n0, assign62080_e96458_d_n2, assign62080_e96458_d_n4, assign62080_e96458_d_n5, assign62080_e96458_d_n6, assign62080_e96458_d_n7, assign62080_e96458_d_n8, assign62080_e96458_d_n9, assign62080_e96458_d_n10, assign62080_e96458_d_n11, assign62080_e96458_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62080_e96455: f64 = (locals.var_qn0 + 1e-25);
        let assign62080_e96456: f64 = (locals.var_t4 * assign62080_e96455);
        (assign62080_e96456, ((locals.var_t4_dn0 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn0)), ((locals.var_t4_dn2 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn2)), ((locals.var_t4_dn4 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn4)), ((locals.var_t4_dn5 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn5)), ((locals.var_t4_dn6 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn6)), ((locals.var_t4_dn7 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn7)), ((locals.var_t4_dn8 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn8)), ((locals.var_t4_dn9 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn9)), ((locals.var_t4_dn10 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn10)), ((locals.var_t4_dn11 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn11)), ((locals.var_t4_dn14 * assign62080_e96455) + (locals.var_t4 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62080_e96458;
        locals.var_t6_dn0 = assign62080_e96458_d_n0;
        locals.var_t6_dn2 = assign62080_e96458_d_n2;
        locals.var_t6_dn4 = assign62080_e96458_d_n4;
        locals.var_t6_dn5 = assign62080_e96458_d_n5;
        locals.var_t6_dn6 = assign62080_e96458_d_n6;
        locals.var_t6_dn7 = assign62080_e96458_d_n7;
        locals.var_t6_dn8 = assign62080_e96458_d_n8;
        locals.var_t6_dn9 = assign62080_e96458_d_n9;
        locals.var_t6_dn10 = assign62080_e96458_d_n10;
        locals.var_t6_dn11 = assign62080_e96458_d_n11;
        locals.var_t6_dn14 = assign62080_e96458_d_n14;

        let (assign62090_e96473, assign62090_e96473_d_n0, assign62090_e96473_d_n2, assign62090_e96473_d_n4, assign62090_e96473_d_n5, assign62090_e96473_d_n6, assign62090_e96473_d_n7, assign62090_e96473_d_n8, assign62090_e96473_d_n9, assign62090_e96473_d_n10, assign62090_e96473_d_n11, assign62090_e96473_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62090_e96466: f64 = (10.0 * 2.220446049250313e-16);
        let assign62090_e96467: f64 = (locals.var_pds + assign62090_e96466);
        let assign62090_e96469: f64 = (assign62090_e96467 * locals.var_fdd);
        let assign62090_e96471: f64 = (assign62090_e96469 * locals.var_t1);
        (assign62090_e96471, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn0)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn2)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn4)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn5)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn6)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn7)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn8)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn9)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn10)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn10)), ((((locals.var_pds_dn11 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn11)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn11)), ((((locals.var_pds_dn14 * locals.var_fdd) + (assign62090_e96467 * locals.var_fdd_dn14)) * locals.var_t1) + (assign62090_e96469 * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign62090_e96473;
        locals.var_ty_dn0 = assign62090_e96473_d_n0;
        locals.var_ty_dn2 = assign62090_e96473_d_n2;
        locals.var_ty_dn4 = assign62090_e96473_d_n4;
        locals.var_ty_dn5 = assign62090_e96473_d_n5;
        locals.var_ty_dn6 = assign62090_e96473_d_n6;
        locals.var_ty_dn7 = assign62090_e96473_d_n7;
        locals.var_ty_dn8 = assign62090_e96473_d_n8;
        locals.var_ty_dn9 = assign62090_e96473_d_n9;
        locals.var_ty_dn10 = assign62090_e96473_d_n10;
        locals.var_ty_dn11 = assign62090_e96473_d_n11;
        locals.var_ty_dn14 = assign62090_e96473_d_n14;

    }

    pub(super) fn stamp_transient_block_219(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62100_e96484, assign62100_e96484_d_n0, assign62100_e96484_d_n2, assign62100_e96484_d_n4, assign62100_e96484_d_n5, assign62100_e96484_d_n6, assign62100_e96484_d_n7, assign62100_e96484_d_n8, assign62100_e96484_d_n9, assign62100_e96484_d_n10, assign62100_e96484_d_n11, assign62100_e96484_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62100_e96480: f64 = (0.2 * locals.var_vmaxe);
        let assign62100_e96482: f64 = (assign62100_e96480 / locals.var_muun);
        (assign62100_e96482, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign62100_e96480 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62100_e96484;
        locals.var_t2_dn0 = assign62100_e96484_d_n0;
        locals.var_t2_dn2 = assign62100_e96484_d_n2;
        locals.var_t2_dn4 = assign62100_e96484_d_n4;
        locals.var_t2_dn5 = assign62100_e96484_d_n5;
        locals.var_t2_dn6 = assign62100_e96484_d_n6;
        locals.var_t2_dn7 = assign62100_e96484_d_n7;
        locals.var_t2_dn8 = assign62100_e96484_d_n8;
        locals.var_t2_dn9 = assign62100_e96484_d_n9;
        locals.var_t2_dn10 = assign62100_e96484_d_n10;
        locals.var_t2_dn11 = assign62100_e96484_d_n11;
        locals.var_t2_dn14 = assign62100_e96484_d_n14;

        let (assign62110_e96494, assign62110_e96494_d_n0, assign62110_e96494_d_n2, assign62110_e96494_d_n4, assign62110_e96494_d_n5, assign62110_e96494_d_n6, assign62110_e96494_d_n7, assign62110_e96494_d_n8, assign62110_e96494_d_n9, assign62110_e96494_d_n10, assign62110_e96494_d_n11, assign62110_e96494_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62110_e96490: f64 = (-locals.var_t2);
        let assign62110_e96492: f64 = (assign62110_e96490 / locals.var_muun);
        (assign62110_e96492, ((((-locals.var_t2_dn0) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn2) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn4) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn5) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn6) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn7) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn8) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn9) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn10) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn11) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn14) * locals.var_muun) - (assign62110_e96490 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62110_e96494;
        locals.var_t3_dn0 = assign62110_e96494_d_n0;
        locals.var_t3_dn2 = assign62110_e96494_d_n2;
        locals.var_t3_dn4 = assign62110_e96494_d_n4;
        locals.var_t3_dn5 = assign62110_e96494_d_n5;
        locals.var_t3_dn6 = assign62110_e96494_d_n6;
        locals.var_t3_dn7 = assign62110_e96494_d_n7;
        locals.var_t3_dn8 = assign62110_e96494_d_n8;
        locals.var_t3_dn9 = assign62110_e96494_d_n9;
        locals.var_t3_dn10 = assign62110_e96494_d_n10;
        locals.var_t3_dn11 = assign62110_e96494_d_n11;
        locals.var_t3_dn14 = assign62110_e96494_d_n14;

        let (assign62120_e96508, assign62120_e96508_d_n0, assign62120_e96508_d_n2, assign62120_e96508_d_n4, assign62120_e96508_d_n5, assign62120_e96508_d_n6, assign62120_e96508_d_n7, assign62120_e96508_d_n8, assign62120_e96508_d_n9, assign62120_e96508_d_n10, assign62120_e96508_d_n11, assign62120_e96508_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62120_e96501: f64 = (locals.var_ty * locals.var_ty);
        let assign62120_e96504: f64 = (locals.var_t2 * locals.var_t2);
        let assign62120_e96505: f64 = (assign62120_e96501 + assign62120_e96504);
        let assign62120_e96506: f64 = (assign62120_e96505).sqrt();
        (assign62120_e96506, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign62120_e96506)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign62120_e96506)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign62120_e96508;
        locals.var_ey_dn0 = assign62120_e96508_d_n0;
        locals.var_ey_dn2 = assign62120_e96508_d_n2;
        locals.var_ey_dn4 = assign62120_e96508_d_n4;
        locals.var_ey_dn5 = assign62120_e96508_d_n5;
        locals.var_ey_dn6 = assign62120_e96508_d_n6;
        locals.var_ey_dn7 = assign62120_e96508_d_n7;
        locals.var_ey_dn8 = assign62120_e96508_d_n8;
        locals.var_ey_dn9 = assign62120_e96508_d_n9;
        locals.var_ey_dn10 = assign62120_e96508_d_n10;
        locals.var_ey_dn11 = assign62120_e96508_d_n11;
        locals.var_ey_dn14 = assign62120_e96508_d_n14;

        let (assign62130_e96517, assign62130_e96517_d_n0, assign62130_e96517_d_n2, assign62130_e96517_d_n4, assign62130_e96517_d_n5, assign62130_e96517_d_n6, assign62130_e96517_d_n7, assign62130_e96517_d_n8, assign62130_e96517_d_n9, assign62130_e96517_d_n10, assign62130_e96517_d_n11, assign62130_e96517_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62130_e96515: f64 = (1.0 / locals.var_ey);
        (assign62130_e96515, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62130_e96517;
        locals.var_t4_dn0 = assign62130_e96517_d_n0;
        locals.var_t4_dn2 = assign62130_e96517_d_n2;
        locals.var_t4_dn4 = assign62130_e96517_d_n4;
        locals.var_t4_dn5 = assign62130_e96517_d_n5;
        locals.var_t4_dn6 = assign62130_e96517_d_n6;
        locals.var_t4_dn7 = assign62130_e96517_d_n7;
        locals.var_t4_dn8 = assign62130_e96517_d_n8;
        locals.var_t4_dn9 = assign62130_e96517_d_n9;
        locals.var_t4_dn10 = assign62130_e96517_d_n10;
        locals.var_t4_dn11 = assign62130_e96517_d_n11;
        locals.var_t4_dn14 = assign62130_e96517_d_n14;

        let (assign62140_e96526, assign62140_e96526_d_n0, assign62140_e96526_d_n2, assign62140_e96526_d_n4, assign62140_e96526_d_n5, assign62140_e96526_d_n6, assign62140_e96526_d_n7, assign62140_e96526_d_n8, assign62140_e96526_d_n9, assign62140_e96526_d_n10, assign62140_e96526_d_n11, assign62140_e96526_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62140_e96524: f64 = (locals.var_muun * locals.var_ey);
        (assign62140_e96524, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign62140_e96526;
        locals.var_em_dn0 = assign62140_e96526_d_n0;
        locals.var_em_dn2 = assign62140_e96526_d_n2;
        locals.var_em_dn4 = assign62140_e96526_d_n4;
        locals.var_em_dn5 = assign62140_e96526_d_n5;
        locals.var_em_dn6 = assign62140_e96526_d_n6;
        locals.var_em_dn7 = assign62140_e96526_d_n7;
        locals.var_em_dn8 = assign62140_e96526_d_n8;
        locals.var_em_dn9 = assign62140_e96526_d_n9;
        locals.var_em_dn10 = assign62140_e96526_d_n10;
        locals.var_em_dn11 = assign62140_e96526_d_n11;
        locals.var_em_dn14 = assign62140_e96526_d_n14;

        let (assign62150_e96535, assign62150_e96535_d_n0, assign62150_e96535_d_n2, assign62150_e96535_d_n4, assign62150_e96535_d_n5, assign62150_e96535_d_n6, assign62150_e96535_d_n7, assign62150_e96535_d_n8, assign62150_e96535_d_n9, assign62150_e96535_d_n10, assign62150_e96535_d_n11, assign62150_e96535_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62150_e96533: f64 = (locals.var_em / locals.var_vmaxe);
        (assign62150_e96533, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62150_e96535;
        locals.var_t1_dn0 = assign62150_e96535_d_n0;
        locals.var_t1_dn2 = assign62150_e96535_d_n2;
        locals.var_t1_dn4 = assign62150_e96535_d_n4;
        locals.var_t1_dn5 = assign62150_e96535_d_n5;
        locals.var_t1_dn6 = assign62150_e96535_d_n6;
        locals.var_t1_dn7 = assign62150_e96535_d_n7;
        locals.var_t1_dn8 = assign62150_e96535_d_n8;
        locals.var_t1_dn9 = assign62150_e96535_d_n9;
        locals.var_t1_dn10 = assign62150_e96535_d_n10;
        locals.var_t1_dn11 = assign62150_e96535_d_n11;
        locals.var_t1_dn14 = assign62150_e96535_d_n14;

        let assign62160_e96539: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96540: f64 = (1.0 - assign62160_e96539);
        let assign62160_e96547: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96548: f64 = (1.0 + assign62160_e96547);
        let assign62160_e96550: f64 = if ((assign62160_e96540 <= p.p178) && (p.p178 <= assign62160_e96548)) { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign62160_e96550;

        let (assign62170_e96559, assign62170_e96559_d_n0, assign62170_e96559_d_n2, assign62170_e96559_d_n4, assign62170_e96559_d_n5, assign62170_e96559_d_n6, assign62170_e96559_d_n7, assign62170_e96559_d_n8, assign62170_e96559_d_n9, assign62170_e96559_d_n10, assign62170_e96559_d_n11, assign62170_e96559_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1493 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62170_e96559;
        locals.var_t3_dn0 = assign62170_e96559_d_n0;
        locals.var_t3_dn2 = assign62170_e96559_d_n2;
        locals.var_t3_dn4 = assign62170_e96559_d_n4;
        locals.var_t3_dn5 = assign62170_e96559_d_n5;
        locals.var_t3_dn6 = assign62170_e96559_d_n6;
        locals.var_t3_dn7 = assign62170_e96559_d_n7;
        locals.var_t3_dn8 = assign62170_e96559_d_n8;
        locals.var_t3_dn9 = assign62170_e96559_d_n9;
        locals.var_t3_dn10 = assign62170_e96559_d_n10;
        locals.var_t3_dn11 = assign62170_e96559_d_n11;
        locals.var_t3_dn14 = assign62170_e96559_d_n14;

        let assign62180_e96563: f64 = (10.0 * 2.220446049250313e-16);
        let assign62180_e96564: f64 = (2.0 - assign62180_e96563);
        let assign62180_e96571: f64 = (10.0 * 2.220446049250313e-16);
        let assign62180_e96572: f64 = (2.0 + assign62180_e96571);
        let assign62180_e96574: f64 = if ((assign62180_e96564 <= p.p178) && (p.p178 <= assign62180_e96572)) { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign62180_e96574;

        let (assign62190_e96586, assign62190_e96586_d_n0, assign62190_e96586_d_n2, assign62190_e96586_d_n4, assign62190_e96586_d_n5, assign62190_e96586_d_n6, assign62190_e96586_d_n7, assign62190_e96586_d_n8, assign62190_e96586_d_n9, assign62190_e96586_d_n10, assign62190_e96586_d_n11, assign62190_e96586_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62190_e96586;
        locals.var_t3_dn0 = assign62190_e96586_d_n0;
        locals.var_t3_dn2 = assign62190_e96586_d_n2;
        locals.var_t3_dn4 = assign62190_e96586_d_n4;
        locals.var_t3_dn5 = assign62190_e96586_d_n5;
        locals.var_t3_dn6 = assign62190_e96586_d_n6;
        locals.var_t3_dn7 = assign62190_e96586_d_n7;
        locals.var_t3_dn8 = assign62190_e96586_d_n8;
        locals.var_t3_dn9 = assign62190_e96586_d_n9;
        locals.var_t3_dn10 = assign62190_e96586_d_n10;
        locals.var_t3_dn11 = assign62190_e96586_d_n11;
        locals.var_t3_dn14 = assign62190_e96586_d_n14;

        let (assign62200_e96608, assign62200_e96608_d_n0, assign62200_e96608_d_n2, assign62200_e96608_d_n4, assign62200_e96608_d_n5, assign62200_e96608_d_n6, assign62200_e96608_d_n7, assign62200_e96608_d_n8, assign62200_e96608_d_n9, assign62200_e96608_d_n10, assign62200_e96608_d_n11, assign62200_e96608_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
        let (assign62200_e96606, assign62200_e96606_d_n0, assign62200_e96606_d_n2, assign62200_e96606_d_n4, assign62200_e96606_d_n5, assign62200_e96606_d_n6, assign62200_e96606_d_n7, assign62200_e96606_d_n8, assign62200_e96606_d_n9, assign62200_e96606_d_n10, assign62200_e96606_d_n11, assign62200_e96606_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62200_e96604: f64 = (p.p178 - 1.0);
                let assign62200_e96605: f64 = (locals.var_t1).powf(assign62200_e96604);
                (assign62200_e96605, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn0)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn2)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn4)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn5)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn6)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn7)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn8)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn9)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn10)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn11)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62200_e96604) as f64).is_finite() && ((assign62200_e96604) as f64).fract() == 0.0 { if assign62200_e96604 == 0.0 { 0.0 } else { (assign62200_e96604 * ((locals.var_t1).powf(assign62200_e96604 - 1.0) * locals.var_t1_dn14)) } } else { (assign62200_e96605 * (assign62200_e96604 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign62200_e96606, assign62200_e96606_d_n0, assign62200_e96606_d_n2, assign62200_e96606_d_n4, assign62200_e96606_d_n5, assign62200_e96606_d_n6, assign62200_e96606_d_n7, assign62200_e96606_d_n8, assign62200_e96606_d_n9, assign62200_e96606_d_n10, assign62200_e96606_d_n11, assign62200_e96606_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62200_e96608;
        locals.var_t3_dn0 = assign62200_e96608_d_n0;
        locals.var_t3_dn2 = assign62200_e96608_d_n2;
        locals.var_t3_dn4 = assign62200_e96608_d_n4;
        locals.var_t3_dn5 = assign62200_e96608_d_n5;
        locals.var_t3_dn6 = assign62200_e96608_d_n6;
        locals.var_t3_dn7 = assign62200_e96608_d_n7;
        locals.var_t3_dn8 = assign62200_e96608_d_n8;
        locals.var_t3_dn9 = assign62200_e96608_d_n9;
        locals.var_t3_dn10 = assign62200_e96608_d_n10;
        locals.var_t3_dn11 = assign62200_e96608_d_n11;
        locals.var_t3_dn14 = assign62200_e96608_d_n14;

        let (assign62210_e96617, assign62210_e96617_d_n0, assign62210_e96617_d_n2, assign62210_e96617_d_n4, assign62210_e96617_d_n5, assign62210_e96617_d_n6, assign62210_e96617_d_n7, assign62210_e96617_d_n8, assign62210_e96617_d_n9, assign62210_e96617_d_n10, assign62210_e96617_d_n11, assign62210_e96617_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62210_e96615: f64 = (locals.var_t1 * locals.var_t3);
        (assign62210_e96615, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62210_e96617;
        locals.var_t2_dn0 = assign62210_e96617_d_n0;
        locals.var_t2_dn2 = assign62210_e96617_d_n2;
        locals.var_t2_dn4 = assign62210_e96617_d_n4;
        locals.var_t2_dn5 = assign62210_e96617_d_n5;
        locals.var_t2_dn6 = assign62210_e96617_d_n6;
        locals.var_t2_dn7 = assign62210_e96617_d_n7;
        locals.var_t2_dn8 = assign62210_e96617_d_n8;
        locals.var_t2_dn9 = assign62210_e96617_d_n9;
        locals.var_t2_dn10 = assign62210_e96617_d_n10;
        locals.var_t2_dn11 = assign62210_e96617_d_n11;
        locals.var_t2_dn14 = assign62210_e96617_d_n14;

        let (assign62220_e96626, assign62220_e96626_d_n0, assign62220_e96626_d_n2, assign62220_e96626_d_n4, assign62220_e96626_d_n5, assign62220_e96626_d_n6, assign62220_e96626_d_n7, assign62220_e96626_d_n8, assign62220_e96626_d_n9, assign62220_e96626_d_n10, assign62220_e96626_d_n11, assign62220_e96626_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62220_e96624: f64 = (1.0 + locals.var_t2);
        (assign62220_e96624, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62220_e96626;
        locals.var_t4_dn0 = assign62220_e96626_d_n0;
        locals.var_t4_dn2 = assign62220_e96626_d_n2;
        locals.var_t4_dn4 = assign62220_e96626_d_n4;
        locals.var_t4_dn5 = assign62220_e96626_d_n5;
        locals.var_t4_dn6 = assign62220_e96626_d_n6;
        locals.var_t4_dn7 = assign62220_e96626_d_n7;
        locals.var_t4_dn8 = assign62220_e96626_d_n8;
        locals.var_t4_dn9 = assign62220_e96626_d_n9;
        locals.var_t4_dn10 = assign62220_e96626_d_n10;
        locals.var_t4_dn11 = assign62220_e96626_d_n11;
        locals.var_t4_dn14 = assign62220_e96626_d_n14;

        let assign62230_e96630: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96631: f64 = (1.0 - assign62230_e96630);
        let assign62230_e96638: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96639: f64 = (1.0 + assign62230_e96638);
        let assign62230_e96641: f64 = if ((assign62230_e96631 <= p.p178) && (p.p178 <= assign62230_e96639)) { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign62230_e96641;

        let (assign62240_e96652, assign62240_e96652_d_n0, assign62240_e96652_d_n2, assign62240_e96652_d_n4, assign62240_e96652_d_n5, assign62240_e96652_d_n6, assign62240_e96652_d_n7, assign62240_e96652_d_n8, assign62240_e96652_d_n9, assign62240_e96652_d_n10, assign62240_e96652_d_n11, assign62240_e96652_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign62240_e96650: f64 = (1.0 / locals.var_t4);
        (assign62240_e96650, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62240_e96652;
        locals.var_t5_dn0 = assign62240_e96652_d_n0;
        locals.var_t5_dn2 = assign62240_e96652_d_n2;
        locals.var_t5_dn4 = assign62240_e96652_d_n4;
        locals.var_t5_dn5 = assign62240_e96652_d_n5;
        locals.var_t5_dn6 = assign62240_e96652_d_n6;
        locals.var_t5_dn7 = assign62240_e96652_d_n7;
        locals.var_t5_dn8 = assign62240_e96652_d_n8;
        locals.var_t5_dn9 = assign62240_e96652_d_n9;
        locals.var_t5_dn10 = assign62240_e96652_d_n10;
        locals.var_t5_dn11 = assign62240_e96652_d_n11;
        locals.var_t5_dn14 = assign62240_e96652_d_n14;

        let assign62250_e96656: f64 = (10.0 * 2.220446049250313e-16);
        let assign62250_e96657: f64 = (2.0 - assign62250_e96656);
        let assign62250_e96664: f64 = (10.0 * 2.220446049250313e-16);
        let assign62250_e96665: f64 = (2.0 + assign62250_e96664);
        let assign62250_e96667: f64 = if ((assign62250_e96657 <= p.p178) && (p.p178 <= assign62250_e96665)) { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign62250_e96667;

        let (assign62260_e96682, assign62260_e96682_d_n0, assign62260_e96682_d_n2, assign62260_e96682_d_n4, assign62260_e96682_d_n5, assign62260_e96682_d_n6, assign62260_e96682_d_n7, assign62260_e96682_d_n8, assign62260_e96682_d_n9, assign62260_e96682_d_n10, assign62260_e96682_d_n11, assign62260_e96682_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1495 == 0.0)) && (locals.var_guard1496 != 0.0)) {
        let assign62260_e96679: f64 = (locals.var_t4).sqrt();
        let assign62260_e96680: f64 = (1.0 / assign62260_e96679);
        (assign62260_e96680, (-((locals.var_t4_dn0 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn2 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn4 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn5 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn6 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn7 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn8 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn9 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn10 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn11 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))), (-((locals.var_t4_dn14 / (2.0 * assign62260_e96679)) / (assign62260_e96679 * assign62260_e96679))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62260_e96682;
        locals.var_t5_dn0 = assign62260_e96682_d_n0;
        locals.var_t5_dn2 = assign62260_e96682_d_n2;
        locals.var_t5_dn4 = assign62260_e96682_d_n4;
        locals.var_t5_dn5 = assign62260_e96682_d_n5;
        locals.var_t5_dn6 = assign62260_e96682_d_n6;
        locals.var_t5_dn7 = assign62260_e96682_d_n7;
        locals.var_t5_dn8 = assign62260_e96682_d_n8;
        locals.var_t5_dn9 = assign62260_e96682_d_n9;
        locals.var_t5_dn10 = assign62260_e96682_d_n10;
        locals.var_t5_dn11 = assign62260_e96682_d_n11;
        locals.var_t5_dn14 = assign62260_e96682_d_n14;

        let (assign62270_e96707, assign62270_e96707_d_n0, assign62270_e96707_d_n2, assign62270_e96707_d_n4, assign62270_e96707_d_n5, assign62270_e96707_d_n6, assign62270_e96707_d_n7, assign62270_e96707_d_n8, assign62270_e96707_d_n9, assign62270_e96707_d_n10, assign62270_e96707_d_n11, assign62270_e96707_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1495 == 0.0)) && (locals.var_guard1496 == 0.0)) {
        let (assign62270_e96705, assign62270_e96705_d_n0, assign62270_e96705_d_n2, assign62270_e96705_d_n4, assign62270_e96705_d_n5, assign62270_e96705_d_n6, assign62270_e96705_d_n7, assign62270_e96705_d_n8, assign62270_e96705_d_n9, assign62270_e96705_d_n10, assign62270_e96705_d_n11, assign62270_e96705_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62270_e96699: f64 = (-1.0);
                let assign62270_e96701: f64 = (assign62270_e96699 / p.p178);
                let assign62270_e96703: f64 = (assign62270_e96701 - 1.0);
                let assign62270_e96704: f64 = (locals.var_t4).powf(assign62270_e96703);
                (assign62270_e96704, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn0)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn2)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn4)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn5)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn6)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn7)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn8)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn9)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn10)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn11)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62270_e96703) as f64).is_finite() && ((assign62270_e96703) as f64).fract() == 0.0 { if assign62270_e96703 == 0.0 { 0.0 } else { (assign62270_e96703 * ((locals.var_t4).powf(assign62270_e96703 - 1.0) * locals.var_t4_dn14)) } } else { (assign62270_e96704 * (assign62270_e96703 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign62270_e96705, assign62270_e96705_d_n0, assign62270_e96705_d_n2, assign62270_e96705_d_n4, assign62270_e96705_d_n5, assign62270_e96705_d_n6, assign62270_e96705_d_n7, assign62270_e96705_d_n8, assign62270_e96705_d_n9, assign62270_e96705_d_n10, assign62270_e96705_d_n11, assign62270_e96705_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62270_e96707;
        locals.var_t6_dn0 = assign62270_e96707_d_n0;
        locals.var_t6_dn2 = assign62270_e96707_d_n2;
        locals.var_t6_dn4 = assign62270_e96707_d_n4;
        locals.var_t6_dn5 = assign62270_e96707_d_n5;
        locals.var_t6_dn6 = assign62270_e96707_d_n6;
        locals.var_t6_dn7 = assign62270_e96707_d_n7;
        locals.var_t6_dn8 = assign62270_e96707_d_n8;
        locals.var_t6_dn9 = assign62270_e96707_d_n9;
        locals.var_t6_dn10 = assign62270_e96707_d_n10;
        locals.var_t6_dn11 = assign62270_e96707_d_n11;
        locals.var_t6_dn14 = assign62270_e96707_d_n14;

        let (assign62280_e96722, assign62280_e96722_d_n0, assign62280_e96722_d_n2, assign62280_e96722_d_n4, assign62280_e96722_d_n5, assign62280_e96722_d_n6, assign62280_e96722_d_n7, assign62280_e96722_d_n8, assign62280_e96722_d_n9, assign62280_e96722_d_n10, assign62280_e96722_d_n11, assign62280_e96722_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1495 == 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign62280_e96720: f64 = (locals.var_t4 * locals.var_t6);
        (assign62280_e96720, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62280_e96722;
        locals.var_t5_dn0 = assign62280_e96722_d_n0;
        locals.var_t5_dn2 = assign62280_e96722_d_n2;
        locals.var_t5_dn4 = assign62280_e96722_d_n4;
        locals.var_t5_dn5 = assign62280_e96722_d_n5;
        locals.var_t5_dn6 = assign62280_e96722_d_n6;
        locals.var_t5_dn7 = assign62280_e96722_d_n7;
        locals.var_t5_dn8 = assign62280_e96722_d_n8;
        locals.var_t5_dn9 = assign62280_e96722_d_n9;
        locals.var_t5_dn10 = assign62280_e96722_d_n10;
        locals.var_t5_dn11 = assign62280_e96722_d_n11;
        locals.var_t5_dn14 = assign62280_e96722_d_n14;

        let (assign62290_e96731, assign62290_e96731_d_n0, assign62290_e96731_d_n2, assign62290_e96731_d_n4, assign62290_e96731_d_n5, assign62290_e96731_d_n6, assign62290_e96731_d_n7, assign62290_e96731_d_n8, assign62290_e96731_d_n9, assign62290_e96731_d_n10, assign62290_e96731_d_n11, assign62290_e96731_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62290_e96729: f64 = (locals.var_muun * locals.var_t5);
        (assign62290_e96729, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign62290_e96731;
        locals.var_mu_dn0 = assign62290_e96731_d_n0;
        locals.var_mu_dn2 = assign62290_e96731_d_n2;
        locals.var_mu_dn4 = assign62290_e96731_d_n4;
        locals.var_mu_dn5 = assign62290_e96731_d_n5;
        locals.var_mu_dn6 = assign62290_e96731_d_n6;
        locals.var_mu_dn7 = assign62290_e96731_d_n7;
        locals.var_mu_dn8 = assign62290_e96731_d_n8;
        locals.var_mu_dn9 = assign62290_e96731_d_n9;
        locals.var_mu_dn10 = assign62290_e96731_d_n10;
        locals.var_mu_dn11 = assign62290_e96731_d_n11;
        locals.var_mu_dn14 = assign62290_e96731_d_n14;

        let (assign62300_e96742, assign62300_e96742_d_n0, assign62300_e96742_d_n2, assign62300_e96742_d_n4, assign62300_e96742_d_n5, assign62300_e96742_d_n6, assign62300_e96742_d_n7, assign62300_e96742_d_n8, assign62300_e96742_d_n9, assign62300_e96742_d_n10, assign62300_e96742_d_n11, assign62300_e96742_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62300_e96738: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign62300_e96740: f64 = (assign62300_e96738 / locals.var_lch);
        (assign62300_e96740, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign62300_e96738 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign62300_e96742;
        locals.var_betawl_dn0 = assign62300_e96742_d_n0;
        locals.var_betawl_dn2 = assign62300_e96742_d_n2;
        locals.var_betawl_dn4 = assign62300_e96742_d_n4;
        locals.var_betawl_dn5 = assign62300_e96742_d_n5;
        locals.var_betawl_dn6 = assign62300_e96742_d_n6;
        locals.var_betawl_dn7 = assign62300_e96742_d_n7;
        locals.var_betawl_dn8 = assign62300_e96742_d_n8;
        locals.var_betawl_dn9 = assign62300_e96742_d_n9;
        locals.var_betawl_dn10 = assign62300_e96742_d_n10;
        locals.var_betawl_dn11 = assign62300_e96742_d_n11;
        locals.var_betawl_dn14 = assign62300_e96742_d_n14;

        let (assign62310_e96752, assign62310_e96752_d_n0, assign62310_e96752_d_n2, assign62310_e96752_d_n4, assign62310_e96752_d_n5, assign62310_e96752_d_n6, assign62310_e96752_d_n7, assign62310_e96752_d_n8, assign62310_e96752_d_n9, assign62310_e96752_d_n10, assign62310_e96752_d_n11, assign62310_e96752_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62310_e96748: f64 = (-locals.var_betawl);
        let assign62310_e96750: f64 = (assign62310_e96748 / locals.var_lch);
        (assign62310_e96750, ((((-locals.var_betawl_dn0) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn2) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn4) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn5) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn6) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn7) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn8) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn9) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn10) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn11) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn14) * locals.var_lch) - (assign62310_e96748 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62310_e96752;
        locals.var_t1_dn0 = assign62310_e96752_d_n0;
        locals.var_t1_dn2 = assign62310_e96752_d_n2;
        locals.var_t1_dn4 = assign62310_e96752_d_n4;
        locals.var_t1_dn5 = assign62310_e96752_d_n5;
        locals.var_t1_dn6 = assign62310_e96752_d_n6;
        locals.var_t1_dn7 = assign62310_e96752_d_n7;
        locals.var_t1_dn8 = assign62310_e96752_d_n8;
        locals.var_t1_dn9 = assign62310_e96752_d_n9;
        locals.var_t1_dn10 = assign62310_e96752_d_n10;
        locals.var_t1_dn11 = assign62310_e96752_d_n11;
        locals.var_t1_dn14 = assign62310_e96752_d_n14;

        let (assign62320_e96763, assign62320_e96763_d_n0, assign62320_e96763_d_n2, assign62320_e96763_d_n4, assign62320_e96763_d_n5, assign62320_e96763_d_n6, assign62320_e96763_d_n7, assign62320_e96763_d_n8, assign62320_e96763_d_n9, assign62320_e96763_d_n10, assign62320_e96763_d_n11, assign62320_e96763_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) {
        let assign62320_e96759: f64 = (locals.var_betawl * locals.var_idd);
        let assign62320_e96761: f64 = (assign62320_e96759 * locals.var_mu);
        (assign62320_e96761, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign62320_e96759 * locals.var_mu_dn14)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62320_e96763;
        locals.var_ids0_dn0 = assign62320_e96763_d_n0;
        locals.var_ids0_dn2 = assign62320_e96763_d_n2;
        locals.var_ids0_dn4 = assign62320_e96763_d_n4;
        locals.var_ids0_dn5 = assign62320_e96763_d_n5;
        locals.var_ids0_dn6 = assign62320_e96763_d_n6;
        locals.var_ids0_dn7 = assign62320_e96763_d_n7;
        locals.var_ids0_dn8 = assign62320_e96763_d_n8;
        locals.var_ids0_dn9 = assign62320_e96763_d_n9;
        locals.var_ids0_dn10 = assign62320_e96763_d_n10;
        locals.var_ids0_dn11 = assign62320_e96763_d_n11;
        locals.var_ids0_dn14 = assign62320_e96763_d_n14;

        let assign62330_e96766: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign62330_e96766;

        let (assign62340_e96779, assign62340_e96779_d_n0, assign62340_e96779_d_n2, assign62340_e96779_d_n4, assign62340_e96779_d_n5, assign62340_e96779_d_n6, assign62340_e96779_d_n7, assign62340_e96779_d_n8, assign62340_e96779_d_n9, assign62340_e96779_d_n10, assign62340_e96779_d_n11, assign62340_e96779_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62340_e96776: f64 = (locals.var_vds - locals.var_pds);
        let assign62340_e96777: f64 = (0.5 * assign62340_e96776);
        (assign62340_e96777, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62340_e96779;
        locals.var_t1_dn0 = assign62340_e96779_d_n0;
        locals.var_t1_dn2 = assign62340_e96779_d_n2;
        locals.var_t1_dn4 = assign62340_e96779_d_n4;
        locals.var_t1_dn5 = assign62340_e96779_d_n5;
        locals.var_t1_dn6 = assign62340_e96779_d_n6;
        locals.var_t1_dn7 = assign62340_e96779_d_n7;
        locals.var_t1_dn8 = assign62340_e96779_d_n8;
        locals.var_t1_dn9 = assign62340_e96779_d_n9;
        locals.var_t1_dn10 = assign62340_e96779_d_n10;
        locals.var_t1_dn11 = assign62340_e96779_d_n11;
        locals.var_t1_dn14 = assign62340_e96779_d_n14;

        let (assign62350_e96792, assign62350_e96792_d_n0, assign62350_e96792_d_n2, assign62350_e96792_d_n4, assign62350_e96792_d_n5, assign62350_e96792_d_n6, assign62350_e96792_d_n7, assign62350_e96792_d_n8, assign62350_e96792_d_n9, assign62350_e96792_d_n10, assign62350_e96792_d_n11, assign62350_e96792_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62350_e96788: f64 = (2.0 * locals.var_t1);
        let assign62350_e96790: f64 = (assign62350_e96788 / 0.01);
        (assign62350_e96790, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62350_e96792;
        locals.var_tmf1_dn0 = assign62350_e96792_d_n0;
        locals.var_tmf1_dn2 = assign62350_e96792_d_n2;
        locals.var_tmf1_dn4 = assign62350_e96792_d_n4;
        locals.var_tmf1_dn5 = assign62350_e96792_d_n5;
        locals.var_tmf1_dn6 = assign62350_e96792_d_n6;
        locals.var_tmf1_dn7 = assign62350_e96792_d_n7;
        locals.var_tmf1_dn8 = assign62350_e96792_d_n8;
        locals.var_tmf1_dn9 = assign62350_e96792_d_n9;
        locals.var_tmf1_dn10 = assign62350_e96792_d_n10;
        locals.var_tmf1_dn11 = assign62350_e96792_d_n11;
        locals.var_tmf1_dn14 = assign62350_e96792_d_n14;

    }

    pub(super) fn stamp_transient_block_220(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62360_e96837, assign62360_e96837_d_n0, assign62360_e96837_d_n2, assign62360_e96837_d_n4, assign62360_e96837_d_n5, assign62360_e96837_d_n6, assign62360_e96837_d_n7, assign62360_e96837_d_n8, assign62360_e96837_d_n9, assign62360_e96837_d_n10, assign62360_e96837_d_n11, assign62360_e96837_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62360_e96803: f64 = (1.0 / 2.0);
        let assign62360_e96807: f64 = (1.0 / 6.0);
        let assign62360_e96811: f64 = (1.0 / 24.0);
        let assign62360_e96815: f64 = (1.0 / 120.0);
        let assign62360_e96819: f64 = (1.0 / 720.0);
        let assign62360_e96823: f64 = (1.0 / 5040.0);
        let assign62360_e96824: f64 = (locals.var_tmf1 * assign62360_e96823);
        let assign62360_e96825: f64 = (assign62360_e96819 + assign62360_e96824);
        let assign62360_e96826: f64 = (locals.var_tmf1 * assign62360_e96825);
        let assign62360_e96827: f64 = (assign62360_e96815 + assign62360_e96826);
        let assign62360_e96828: f64 = (locals.var_tmf1 * assign62360_e96827);
        let assign62360_e96829: f64 = (assign62360_e96811 + assign62360_e96828);
        let assign62360_e96830: f64 = (locals.var_tmf1 * assign62360_e96829);
        let assign62360_e96831: f64 = (assign62360_e96807 + assign62360_e96830);
        let assign62360_e96832: f64 = (locals.var_tmf1 * assign62360_e96831);
        let assign62360_e96833: f64 = (assign62360_e96803 + assign62360_e96832);
        let assign62360_e96834: f64 = (locals.var_tmf1 * assign62360_e96833);
        let assign62360_e96835: f64 = (1.0 + assign62360_e96834);
        (assign62360_e96835, ((locals.var_tmf1_dn0 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn2 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn4 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn5 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn6 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn7 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn8 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn9 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn10 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn11 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62360_e96823))))))))))), ((locals.var_tmf1_dn14 * assign62360_e96833) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62360_e96831) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62360_e96829) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62360_e96827) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62360_e96825) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62360_e96823))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62360_e96837;
        locals.var_tmf2_dn0 = assign62360_e96837_d_n0;
        locals.var_tmf2_dn2 = assign62360_e96837_d_n2;
        locals.var_tmf2_dn4 = assign62360_e96837_d_n4;
        locals.var_tmf2_dn5 = assign62360_e96837_d_n5;
        locals.var_tmf2_dn6 = assign62360_e96837_d_n6;
        locals.var_tmf2_dn7 = assign62360_e96837_d_n7;
        locals.var_tmf2_dn8 = assign62360_e96837_d_n8;
        locals.var_tmf2_dn9 = assign62360_e96837_d_n9;
        locals.var_tmf2_dn10 = assign62360_e96837_d_n10;
        locals.var_tmf2_dn11 = assign62360_e96837_d_n11;
        locals.var_tmf2_dn14 = assign62360_e96837_d_n14;

        let (assign62370_e96878, assign62370_e96878_d_n0, assign62370_e96878_d_n2, assign62370_e96878_d_n4, assign62370_e96878_d_n5, assign62370_e96878_d_n6, assign62370_e96878_d_n7, assign62370_e96878_d_n8, assign62370_e96878_d_n9, assign62370_e96878_d_n10, assign62370_e96878_d_n11, assign62370_e96878_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62370_e96846: f64 = (1.0 / 2.0);
        let assign62370_e96850: f64 = (1.0 / 3.0);
        let assign62370_e96854: f64 = (1.0 / 8.0);
        let assign62370_e96858: f64 = (1.0 / 30.0);
        let assign62370_e96862: f64 = (1.0 / 144.0);
        let assign62370_e96866: f64 = (1.0 / 840.0);
        let assign62370_e96867: f64 = (locals.var_tmf1 * assign62370_e96866);
        let assign62370_e96868: f64 = (assign62370_e96862 + assign62370_e96867);
        let assign62370_e96869: f64 = (locals.var_tmf1 * assign62370_e96868);
        let assign62370_e96870: f64 = (assign62370_e96858 + assign62370_e96869);
        let assign62370_e96871: f64 = (locals.var_tmf1 * assign62370_e96870);
        let assign62370_e96872: f64 = (assign62370_e96854 + assign62370_e96871);
        let assign62370_e96873: f64 = (locals.var_tmf1 * assign62370_e96872);
        let assign62370_e96874: f64 = (assign62370_e96850 + assign62370_e96873);
        let assign62370_e96875: f64 = (locals.var_tmf1 * assign62370_e96874);
        let assign62370_e96876: f64 = (assign62370_e96846 + assign62370_e96875);
        (assign62370_e96876, ((locals.var_tmf1_dn0 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62370_e96866))))))))), ((locals.var_tmf1_dn2 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62370_e96866))))))))), ((locals.var_tmf1_dn4 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62370_e96866))))))))), ((locals.var_tmf1_dn5 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62370_e96866))))))))), ((locals.var_tmf1_dn6 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62370_e96866))))))))), ((locals.var_tmf1_dn7 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62370_e96866))))))))), ((locals.var_tmf1_dn8 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62370_e96866))))))))), ((locals.var_tmf1_dn9 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62370_e96866))))))))), ((locals.var_tmf1_dn10 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62370_e96866))))))))), ((locals.var_tmf1_dn11 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62370_e96866))))))))), ((locals.var_tmf1_dn14 * assign62370_e96874) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62370_e96872) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62370_e96870) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62370_e96868) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62370_e96866))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign62370_e96878;
        locals.var_tmf3_dn0 = assign62370_e96878_d_n0;
        locals.var_tmf3_dn2 = assign62370_e96878_d_n2;
        locals.var_tmf3_dn4 = assign62370_e96878_d_n4;
        locals.var_tmf3_dn5 = assign62370_e96878_d_n5;
        locals.var_tmf3_dn6 = assign62370_e96878_d_n6;
        locals.var_tmf3_dn7 = assign62370_e96878_d_n7;
        locals.var_tmf3_dn8 = assign62370_e96878_d_n8;
        locals.var_tmf3_dn9 = assign62370_e96878_d_n9;
        locals.var_tmf3_dn10 = assign62370_e96878_d_n10;
        locals.var_tmf3_dn11 = assign62370_e96878_d_n11;
        locals.var_tmf3_dn14 = assign62370_e96878_d_n14;

        let (assign62380_e96889, assign62380_e96889_d_n0, assign62380_e96889_d_n2, assign62380_e96889_d_n4, assign62380_e96889_d_n5, assign62380_e96889_d_n6, assign62380_e96889_d_n7, assign62380_e96889_d_n8, assign62380_e96889_d_n9, assign62380_e96889_d_n10, assign62380_e96889_d_n11, assign62380_e96889_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62380_e96887: f64 = (0.01 / locals.var_tmf2);
        (assign62380_e96887, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62380_e96889;
        locals.var_t6_dn0 = assign62380_e96889_d_n0;
        locals.var_t6_dn2 = assign62380_e96889_d_n2;
        locals.var_t6_dn4 = assign62380_e96889_d_n4;
        locals.var_t6_dn5 = assign62380_e96889_d_n5;
        locals.var_t6_dn6 = assign62380_e96889_d_n6;
        locals.var_t6_dn7 = assign62380_e96889_d_n7;
        locals.var_t6_dn8 = assign62380_e96889_d_n8;
        locals.var_t6_dn9 = assign62380_e96889_d_n9;
        locals.var_t6_dn10 = assign62380_e96889_d_n10;
        locals.var_t6_dn11 = assign62380_e96889_d_n11;
        locals.var_t6_dn14 = assign62380_e96889_d_n14;

        let (assign62390_e96905, assign62390_e96905_d_n0, assign62390_e96905_d_n2, assign62390_e96905_d_n4, assign62390_e96905_d_n5, assign62390_e96905_d_n6, assign62390_e96905_d_n7, assign62390_e96905_d_n8, assign62390_e96905_d_n9, assign62390_e96905_d_n10, assign62390_e96905_d_n11, assign62390_e96905_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62390_e96897: f64 = (-2.0);
        let assign62390_e96899: f64 = (assign62390_e96897 * locals.var_tmf3);
        let assign62390_e96902: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign62390_e96903: f64 = (assign62390_e96899 / assign62390_e96902);
        (assign62390_e96903, ((((assign62390_e96897 * locals.var_tmf3_dn0) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn2) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn4) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn5) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn6) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn7) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn8) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn9) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn10) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn11) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign62390_e96902 * assign62390_e96902)), ((((assign62390_e96897 * locals.var_tmf3_dn14) * assign62390_e96902) - (assign62390_e96899 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign62390_e96902 * assign62390_e96902)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62390_e96905;
        locals.var_t2_dn0 = assign62390_e96905_d_n0;
        locals.var_t2_dn2 = assign62390_e96905_d_n2;
        locals.var_t2_dn4 = assign62390_e96905_d_n4;
        locals.var_t2_dn5 = assign62390_e96905_d_n5;
        locals.var_t2_dn6 = assign62390_e96905_d_n6;
        locals.var_t2_dn7 = assign62390_e96905_d_n7;
        locals.var_t2_dn8 = assign62390_e96905_d_n8;
        locals.var_t2_dn9 = assign62390_e96905_d_n9;
        locals.var_t2_dn10 = assign62390_e96905_d_n10;
        locals.var_t2_dn11 = assign62390_e96905_d_n11;
        locals.var_t2_dn14 = assign62390_e96905_d_n14;

        let (assign62400_e96916, assign62400_e96916_d_n0, assign62400_e96916_d_n2, assign62400_e96916_d_n4, assign62400_e96916_d_n5, assign62400_e96916_d_n6, assign62400_e96916_d_n7, assign62400_e96916_d_n8, assign62400_e96916_d_n9, assign62400_e96916_d_n10, assign62400_e96916_d_n11, assign62400_e96916_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62400_e96914: f64 = (locals.var_t2 * 0.5);
        (assign62400_e96914, (locals.var_t2_dn0 * 0.5), (locals.var_t2_dn2 * 0.5), (locals.var_t2_dn4 * 0.5), (locals.var_t2_dn5 * 0.5), (locals.var_t2_dn6 * 0.5), (locals.var_t2_dn7 * 0.5), (locals.var_t2_dn8 * 0.5), (locals.var_t2_dn9 * 0.5), (locals.var_t2_dn10 * 0.5), (locals.var_t2_dn11 * 0.5), (locals.var_t2_dn14 * 0.5),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62400_e96916;
        locals.var_t2_dn0 = assign62400_e96916_d_n0;
        locals.var_t2_dn2 = assign62400_e96916_d_n2;
        locals.var_t2_dn4 = assign62400_e96916_d_n4;
        locals.var_t2_dn5 = assign62400_e96916_d_n5;
        locals.var_t2_dn6 = assign62400_e96916_d_n6;
        locals.var_t2_dn7 = assign62400_e96916_d_n7;
        locals.var_t2_dn8 = assign62400_e96916_d_n8;
        locals.var_t2_dn9 = assign62400_e96916_d_n9;
        locals.var_t2_dn10 = assign62400_e96916_d_n10;
        locals.var_t2_dn11 = assign62400_e96916_d_n11;
        locals.var_t2_dn14 = assign62400_e96916_d_n14;

        let (assign62410_e96929, assign62410_e96929_d_n0, assign62410_e96929_d_n2, assign62410_e96929_d_n4, assign62410_e96929_d_n5, assign62410_e96929_d_n6, assign62410_e96929_d_n7, assign62410_e96929_d_n8, assign62410_e96929_d_n9, assign62410_e96929_d_n10, assign62410_e96929_d_n11, assign62410_e96929_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62410_e96926: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62410_e96927: f64 = (1.1 - assign62410_e96926);
        (assign62410_e96927, (-(locals.var_ps0_dn0 + locals.var_t6_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6_dn2)), (-(locals.var_ps0_dn4 + locals.var_t6_dn4)), (-(locals.var_ps0_dn5 + locals.var_t6_dn5)), (-(locals.var_ps0_dn6 + locals.var_t6_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6_dn7)), (-(locals.var_ps0_dn8 + locals.var_t6_dn8)), (-(locals.var_ps0_dn9 + locals.var_t6_dn9)), (-(locals.var_ps0_dn10 + locals.var_t6_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6_dn11)), (-(locals.var_ps0_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62410_e96929;
        locals.var_t1_dn0 = assign62410_e96929_d_n0;
        locals.var_t1_dn2 = assign62410_e96929_d_n2;
        locals.var_t1_dn4 = assign62410_e96929_d_n4;
        locals.var_t1_dn5 = assign62410_e96929_d_n5;
        locals.var_t1_dn6 = assign62410_e96929_d_n6;
        locals.var_t1_dn7 = assign62410_e96929_d_n7;
        locals.var_t1_dn8 = assign62410_e96929_d_n8;
        locals.var_t1_dn9 = assign62410_e96929_d_n9;
        locals.var_t1_dn10 = assign62410_e96929_d_n10;
        locals.var_t1_dn11 = assign62410_e96929_d_n11;
        locals.var_t1_dn14 = assign62410_e96929_d_n14;

        let (assign62420_e96947, assign62420_e96947_d_n0, assign62420_e96947_d_n2, assign62420_e96947_d_n4, assign62420_e96947_d_n5, assign62420_e96947_d_n6, assign62420_e96947_d_n7, assign62420_e96947_d_n8, assign62420_e96947_d_n9, assign62420_e96947_d_n10, assign62420_e96947_d_n11, assign62420_e96947_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62420_e96938: f64 = (locals.var_t1 * locals.var_t1);
        let assign62420_e96941: f64 = (4.0 * 0.05);
        let assign62420_e96943: f64 = (assign62420_e96941 * 0.05);
        let assign62420_e96944: f64 = (assign62420_e96938 + assign62420_e96943);
        let assign62420_e96945: f64 = (assign62420_e96944).sqrt();
        (assign62420_e96945, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign62420_e96945)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign62420_e96945)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62420_e96947;
        locals.var_tmf2_dn0 = assign62420_e96947_d_n0;
        locals.var_tmf2_dn2 = assign62420_e96947_d_n2;
        locals.var_tmf2_dn4 = assign62420_e96947_d_n4;
        locals.var_tmf2_dn5 = assign62420_e96947_d_n5;
        locals.var_tmf2_dn6 = assign62420_e96947_d_n6;
        locals.var_tmf2_dn7 = assign62420_e96947_d_n7;
        locals.var_tmf2_dn8 = assign62420_e96947_d_n8;
        locals.var_tmf2_dn9 = assign62420_e96947_d_n9;
        locals.var_tmf2_dn10 = assign62420_e96947_d_n10;
        locals.var_tmf2_dn11 = assign62420_e96947_d_n11;
        locals.var_tmf2_dn14 = assign62420_e96947_d_n14;

        let (assign62430_e96962, assign62430_e96962_d_n0, assign62430_e96962_d_n2, assign62430_e96962_d_n4, assign62430_e96962_d_n5, assign62430_e96962_d_n6, assign62430_e96962_d_n7, assign62430_e96962_d_n8, assign62430_e96962_d_n9, assign62430_e96962_d_n10, assign62430_e96962_d_n11, assign62430_e96962_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62430_e96958: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign62430_e96959: f64 = (1.0 + assign62430_e96958);
        let assign62430_e96960: f64 = (0.5 * assign62430_e96959);
        (assign62430_e96960, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62430_e96962;
        locals.var_t0_dn0 = assign62430_e96962_d_n0;
        locals.var_t0_dn2 = assign62430_e96962_d_n2;
        locals.var_t0_dn4 = assign62430_e96962_d_n4;
        locals.var_t0_dn5 = assign62430_e96962_d_n5;
        locals.var_t0_dn6 = assign62430_e96962_d_n6;
        locals.var_t0_dn7 = assign62430_e96962_d_n7;
        locals.var_t0_dn8 = assign62430_e96962_d_n8;
        locals.var_t0_dn9 = assign62430_e96962_d_n9;
        locals.var_t0_dn10 = assign62430_e96962_d_n10;
        locals.var_t0_dn11 = assign62430_e96962_d_n11;
        locals.var_t0_dn14 = assign62430_e96962_d_n14;

        let (assign62440_e96975, assign62440_e96975_d_n0, assign62440_e96975_d_n2, assign62440_e96975_d_n4, assign62440_e96975_d_n5, assign62440_e96975_d_n6, assign62440_e96975_d_n7, assign62440_e96975_d_n8, assign62440_e96975_d_n9, assign62440_e96975_d_n10, assign62440_e96975_d_n11, assign62440_e96975_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62440_e96972: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign62440_e96973: f64 = (0.5 * assign62440_e96972);
        (assign62440_e96973, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62440_e96975;
        locals.var_t2_dn0 = assign62440_e96975_d_n0;
        locals.var_t2_dn2 = assign62440_e96975_d_n2;
        locals.var_t2_dn4 = assign62440_e96975_d_n4;
        locals.var_t2_dn5 = assign62440_e96975_d_n5;
        locals.var_t2_dn6 = assign62440_e96975_d_n6;
        locals.var_t2_dn7 = assign62440_e96975_d_n7;
        locals.var_t2_dn8 = assign62440_e96975_d_n8;
        locals.var_t2_dn9 = assign62440_e96975_d_n9;
        locals.var_t2_dn10 = assign62440_e96975_d_n10;
        locals.var_t2_dn11 = assign62440_e96975_d_n11;
        locals.var_t2_dn14 = assign62440_e96975_d_n14;

        let assign62450_e96978: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign62450_e96978;

        let (assign62460_e96989, assign62460_e96989_d_n0, assign62460_e96989_d_n2, assign62460_e96989_d_n4, assign62460_e96989_d_n5, assign62460_e96989_d_n6, assign62460_e96989_d_n7, assign62460_e96989_d_n8, assign62460_e96989_d_n9, assign62460_e96989_d_n10, assign62460_e96989_d_n11, assign62460_e96989_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62460_e96989;
        locals.var_t2_dn0 = assign62460_e96989_d_n0;
        locals.var_t2_dn2 = assign62460_e96989_d_n2;
        locals.var_t2_dn4 = assign62460_e96989_d_n4;
        locals.var_t2_dn5 = assign62460_e96989_d_n5;
        locals.var_t2_dn6 = assign62460_e96989_d_n6;
        locals.var_t2_dn7 = assign62460_e96989_d_n7;
        locals.var_t2_dn8 = assign62460_e96989_d_n8;
        locals.var_t2_dn9 = assign62460_e96989_d_n9;
        locals.var_t2_dn10 = assign62460_e96989_d_n10;
        locals.var_t2_dn11 = assign62460_e96989_d_n11;
        locals.var_t2_dn14 = assign62460_e96989_d_n14;

        let (assign62470_e97000, assign62470_e97000_d_n0, assign62470_e97000_d_n2, assign62470_e97000_d_n4, assign62470_e97000_d_n5, assign62470_e97000_d_n6, assign62470_e97000_d_n7, assign62470_e97000_d_n8, assign62470_e97000_d_n9, assign62470_e97000_d_n10, assign62470_e97000_d_n11, assign62470_e97000_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62470_e97000;
        locals.var_t0_dn0 = assign62470_e97000_d_n0;
        locals.var_t0_dn2 = assign62470_e97000_d_n2;
        locals.var_t0_dn4 = assign62470_e97000_d_n4;
        locals.var_t0_dn5 = assign62470_e97000_d_n5;
        locals.var_t0_dn6 = assign62470_e97000_d_n6;
        locals.var_t0_dn7 = assign62470_e97000_d_n7;
        locals.var_t0_dn8 = assign62470_e97000_d_n8;
        locals.var_t0_dn9 = assign62470_e97000_d_n9;
        locals.var_t0_dn10 = assign62470_e97000_d_n10;
        locals.var_t0_dn11 = assign62470_e97000_d_n11;
        locals.var_t0_dn14 = assign62470_e97000_d_n14;

        let (assign62480_e97011, assign62480_e97011_d_n0, assign62480_e97011_d_n2, assign62480_e97011_d_n4, assign62480_e97011_d_n5, assign62480_e97011_d_n6, assign62480_e97011_d_n7, assign62480_e97011_d_n8, assign62480_e97011_d_n9, assign62480_e97011_d_n10, assign62480_e97011_d_n11, assign62480_e97011_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62480_e97009: f64 = (locals.var_t2 + 1e-25);
        (assign62480_e97009, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62480_e97011;
        locals.var_t2_dn0 = assign62480_e97011_d_n0;
        locals.var_t2_dn2 = assign62480_e97011_d_n2;
        locals.var_t2_dn4 = assign62480_e97011_d_n4;
        locals.var_t2_dn5 = assign62480_e97011_d_n5;
        locals.var_t2_dn6 = assign62480_e97011_d_n6;
        locals.var_t2_dn7 = assign62480_e97011_d_n7;
        locals.var_t2_dn8 = assign62480_e97011_d_n8;
        locals.var_t2_dn9 = assign62480_e97011_d_n9;
        locals.var_t2_dn10 = assign62480_e97011_d_n10;
        locals.var_t2_dn11 = assign62480_e97011_d_n11;
        locals.var_t2_dn14 = assign62480_e97011_d_n14;

        let (assign62490_e97022, assign62490_e97022_d_n0, assign62490_e97022_d_n2, assign62490_e97022_d_n4, assign62490_e97022_d_n5, assign62490_e97022_d_n6, assign62490_e97022_d_n7, assign62490_e97022_d_n8, assign62490_e97022_d_n9, assign62490_e97022_d_n10, assign62490_e97022_d_n11, assign62490_e97022_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62490_e97020: f64 = (locals.var_beta * locals.var_ptl0);
        (assign62490_e97020, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62490_e97022;
        locals.var_t0_dn0 = assign62490_e97022_d_n0;
        locals.var_t0_dn2 = assign62490_e97022_d_n2;
        locals.var_t0_dn4 = assign62490_e97022_d_n4;
        locals.var_t0_dn5 = assign62490_e97022_d_n5;
        locals.var_t0_dn6 = assign62490_e97022_d_n6;
        locals.var_t0_dn7 = assign62490_e97022_d_n7;
        locals.var_t0_dn8 = assign62490_e97022_d_n8;
        locals.var_t0_dn9 = assign62490_e97022_d_n9;
        locals.var_t0_dn10 = assign62490_e97022_d_n10;
        locals.var_t0_dn11 = assign62490_e97022_d_n11;
        locals.var_t0_dn14 = assign62490_e97022_d_n14;

        let (assign62500_e97033, assign62500_e97033_d_n0, assign62500_e97033_d_n2, assign62500_e97033_d_n4, assign62500_e97033_d_n5, assign62500_e97033_d_n6, assign62500_e97033_d_n7, assign62500_e97033_d_n8, assign62500_e97033_d_n9, assign62500_e97033_d_n10, assign62500_e97033_d_n11, assign62500_e97033_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62500_e97031: f64 = (locals.var_cox * locals.var_t0);
        (assign62500_e97031, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62500_e97033;
        locals.var_t3_dn0 = assign62500_e97033_d_n0;
        locals.var_t3_dn2 = assign62500_e97033_d_n2;
        locals.var_t3_dn4 = assign62500_e97033_d_n4;
        locals.var_t3_dn5 = assign62500_e97033_d_n5;
        locals.var_t3_dn6 = assign62500_e97033_d_n6;
        locals.var_t3_dn7 = assign62500_e97033_d_n7;
        locals.var_t3_dn8 = assign62500_e97033_d_n8;
        locals.var_t3_dn9 = assign62500_e97033_d_n9;
        locals.var_t3_dn10 = assign62500_e97033_d_n10;
        locals.var_t3_dn11 = assign62500_e97033_d_n11;
        locals.var_t3_dn14 = assign62500_e97033_d_n14;

        let (assign62510_e97044, assign62510_e97044_d_n0, assign62510_e97044_d_n2, assign62510_e97044_d_n4, assign62510_e97044_d_n5, assign62510_e97044_d_n6, assign62510_e97044_d_n7, assign62510_e97044_d_n8, assign62510_e97044_d_n9, assign62510_e97044_d_n10, assign62510_e97044_d_n11, assign62510_e97044_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62510_e97042: f64 = (locals.var_t2).powf(p.p284);
        (assign62510_e97042, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62510_e97044;
        locals.var_t0_dn0 = assign62510_e97044_d_n0;
        locals.var_t0_dn2 = assign62510_e97044_d_n2;
        locals.var_t0_dn4 = assign62510_e97044_d_n4;
        locals.var_t0_dn5 = assign62510_e97044_d_n5;
        locals.var_t0_dn6 = assign62510_e97044_d_n6;
        locals.var_t0_dn7 = assign62510_e97044_d_n7;
        locals.var_t0_dn8 = assign62510_e97044_d_n8;
        locals.var_t0_dn9 = assign62510_e97044_d_n9;
        locals.var_t0_dn10 = assign62510_e97044_d_n10;
        locals.var_t0_dn11 = assign62510_e97044_d_n11;
        locals.var_t0_dn14 = assign62510_e97044_d_n14;

        let (assign62520_e97055, assign62520_e97055_d_n0, assign62520_e97055_d_n2, assign62520_e97055_d_n4, assign62520_e97055_d_n5, assign62520_e97055_d_n6, assign62520_e97055_d_n7, assign62520_e97055_d_n8, assign62520_e97055_d_n9, assign62520_e97055_d_n10, assign62520_e97055_d_n11, assign62520_e97055_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62520_e97053: f64 = (locals.var_t3 * locals.var_t0);
        (assign62520_e97053, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62520_e97055;
        locals.var_t9_dn0 = assign62520_e97055_d_n0;
        locals.var_t9_dn2 = assign62520_e97055_d_n2;
        locals.var_t9_dn4 = assign62520_e97055_d_n4;
        locals.var_t9_dn5 = assign62520_e97055_d_n5;
        locals.var_t9_dn6 = assign62520_e97055_d_n6;
        locals.var_t9_dn7 = assign62520_e97055_d_n7;
        locals.var_t9_dn8 = assign62520_e97055_d_n8;
        locals.var_t9_dn9 = assign62520_e97055_d_n9;
        locals.var_t9_dn10 = assign62520_e97055_d_n10;
        locals.var_t9_dn11 = assign62520_e97055_d_n11;
        locals.var_t9_dn14 = assign62520_e97055_d_n14;

        let (assign62530_e97068, assign62530_e97068_d_n0, assign62530_e97068_d_n2, assign62530_e97068_d_n4, assign62530_e97068_d_n5, assign62530_e97068_d_n6, assign62530_e97068_d_n7, assign62530_e97068_d_n8, assign62530_e97068_d_n9, assign62530_e97068_d_n10, assign62530_e97068_d_n11, assign62530_e97068_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62530_e97065: f64 = (locals.var_vdsz__blk441 * p.p285);
        let assign62530_e97066: f64 = (1.0 + assign62530_e97065);
        (assign62530_e97066, (locals.var_vdsz__blk441_dn0 * p.p285), (locals.var_vdsz__blk441_dn2 * p.p285), (locals.var_vdsz__blk441_dn4 * p.p285), (locals.var_vdsz__blk441_dn5 * p.p285), (locals.var_vdsz__blk441_dn6 * p.p285), (locals.var_vdsz__blk441_dn7 * p.p285), (locals.var_vdsz__blk441_dn8 * p.p285), (locals.var_vdsz__blk441_dn9 * p.p285), (locals.var_vdsz__blk441_dn10 * p.p285), (locals.var_vdsz__blk441_dn11 * p.p285), (locals.var_vdsz__blk441_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62530_e97068;
        locals.var_t4_dn0 = assign62530_e97068_d_n0;
        locals.var_t4_dn2 = assign62530_e97068_d_n2;
        locals.var_t4_dn4 = assign62530_e97068_d_n4;
        locals.var_t4_dn5 = assign62530_e97068_d_n5;
        locals.var_t4_dn6 = assign62530_e97068_d_n6;
        locals.var_t4_dn7 = assign62530_e97068_d_n7;
        locals.var_t4_dn8 = assign62530_e97068_d_n8;
        locals.var_t4_dn9 = assign62530_e97068_d_n9;
        locals.var_t4_dn10 = assign62530_e97068_d_n10;
        locals.var_t4_dn11 = assign62530_e97068_d_n11;
        locals.var_t4_dn14 = assign62530_e97068_d_n14;

        let (assign62540_e97077, assign62540_e97077_d_n0, assign62540_e97077_d_n2, assign62540_e97077_d_n4, assign62540_e97077_d_n5, assign62540_e97077_d_n6, assign62540_e97077_d_n7, assign62540_e97077_d_n8, assign62540_e97077_d_n9, assign62540_e97077_d_n10, assign62540_e97077_d_n11, assign62540_e97077_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62540_e97077;
        locals.var_t0_dn0 = assign62540_e97077_d_n0;
        locals.var_t0_dn2 = assign62540_e97077_d_n2;
        locals.var_t0_dn4 = assign62540_e97077_d_n4;
        locals.var_t0_dn5 = assign62540_e97077_d_n5;
        locals.var_t0_dn6 = assign62540_e97077_d_n6;
        locals.var_t0_dn7 = assign62540_e97077_d_n7;
        locals.var_t0_dn8 = assign62540_e97077_d_n8;
        locals.var_t0_dn9 = assign62540_e97077_d_n9;
        locals.var_t0_dn10 = assign62540_e97077_d_n10;
        locals.var_t0_dn11 = assign62540_e97077_d_n11;
        locals.var_t0_dn14 = assign62540_e97077_d_n14;

        let (assign62550_e97090, assign62550_e97090_d_n0, assign62550_e97090_d_n2, assign62550_e97090_d_n4, assign62550_e97090_d_n5, assign62550_e97090_d_n6, assign62550_e97090_d_n7, assign62550_e97090_d_n8, assign62550_e97090_d_n9, assign62550_e97090_d_n10, assign62550_e97090_d_n11, assign62550_e97090_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62550_e97086: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62550_e97088: f64 = (assign62550_e97086 - locals.var_vbsz__blk440);
        (assign62550_e97088, ((locals.var_ps0_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_ps0_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_ps0_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_ps0_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_ps0_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_ps0_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_ps0_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_ps0_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_ps0_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_ps0_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_ps0_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62550_e97090;
        locals.var_t5_dn0 = assign62550_e97090_d_n0;
        locals.var_t5_dn2 = assign62550_e97090_d_n2;
        locals.var_t5_dn4 = assign62550_e97090_d_n4;
        locals.var_t5_dn5 = assign62550_e97090_d_n5;
        locals.var_t5_dn6 = assign62550_e97090_d_n6;
        locals.var_t5_dn7 = assign62550_e97090_d_n7;
        locals.var_t5_dn8 = assign62550_e97090_d_n8;
        locals.var_t5_dn9 = assign62550_e97090_d_n9;
        locals.var_t5_dn10 = assign62550_e97090_d_n10;
        locals.var_t5_dn11 = assign62550_e97090_d_n11;
        locals.var_t5_dn14 = assign62550_e97090_d_n14;

        let (assign62560_e97105, assign62560_e97105_d_n0, assign62560_e97105_d_n2, assign62560_e97105_d_n4, assign62560_e97105_d_n5, assign62560_e97105_d_n6, assign62560_e97105_d_n7, assign62560_e97105_d_n8, assign62560_e97105_d_n9, assign62560_e97105_d_n10, assign62560_e97105_d_n11, assign62560_e97105_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62560_e97100: f64 = (locals.var_vdsz__blk441 * locals.var_t0);
        let assign62560_e97102: f64 = (assign62560_e97100 * locals.var_t5);
        let assign62560_e97103: f64 = (locals.var_t4 + assign62560_e97102);
        (assign62560_e97103, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk441_dn0 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn0)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk441_dn2 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn2)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk441_dn4 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn4)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk441_dn5 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn5)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk441_dn6 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn6)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk441_dn7 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn7)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk441_dn8 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn8)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk441_dn9 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn9)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk441_dn10 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn10)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk441_dn11 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn11)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk441_dn14 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn14)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62560_e97105;
        locals.var_t4_dn0 = assign62560_e97105_d_n0;
        locals.var_t4_dn2 = assign62560_e97105_d_n2;
        locals.var_t4_dn4 = assign62560_e97105_d_n4;
        locals.var_t4_dn5 = assign62560_e97105_d_n5;
        locals.var_t4_dn6 = assign62560_e97105_d_n6;
        locals.var_t4_dn7 = assign62560_e97105_d_n7;
        locals.var_t4_dn8 = assign62560_e97105_d_n8;
        locals.var_t4_dn9 = assign62560_e97105_d_n9;
        locals.var_t4_dn10 = assign62560_e97105_d_n10;
        locals.var_t4_dn11 = assign62560_e97105_d_n11;
        locals.var_t4_dn14 = assign62560_e97105_d_n14;

        let (assign62570_e97116, assign62570_e97116_d_n0, assign62570_e97116_d_n2, assign62570_e97116_d_n4, assign62570_e97116_d_n5, assign62570_e97116_d_n6, assign62570_e97116_d_n7, assign62570_e97116_d_n8, assign62570_e97116_d_n9, assign62570_e97116_d_n10, assign62570_e97116_d_n11, assign62570_e97116_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62570_e97114: f64 = (locals.var_t9 * locals.var_t4);
        (assign62570_e97114, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62570_e97116;
        locals.var_t6_dn0 = assign62570_e97116_d_n0;
        locals.var_t6_dn2 = assign62570_e97116_d_n2;
        locals.var_t6_dn4 = assign62570_e97116_d_n4;
        locals.var_t6_dn5 = assign62570_e97116_d_n5;
        locals.var_t6_dn6 = assign62570_e97116_d_n6;
        locals.var_t6_dn7 = assign62570_e97116_d_n7;
        locals.var_t6_dn8 = assign62570_e97116_d_n8;
        locals.var_t6_dn9 = assign62570_e97116_d_n9;
        locals.var_t6_dn10 = assign62570_e97116_d_n10;
        locals.var_t6_dn11 = assign62570_e97116_d_n11;
        locals.var_t6_dn14 = assign62570_e97116_d_n14;

        let (assign62580_e97125, assign62580_e97125_d_n0, assign62580_e97125_d_n2, assign62580_e97125_d_n4, assign62580_e97125_d_n5, assign62580_e97125_d_n6, assign62580_e97125_d_n7, assign62580_e97125_d_n8, assign62580_e97125_d_n9, assign62580_e97125_d_n10, assign62580_e97125_d_n11, assign62580_e97125_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62580_e97125;
        locals.var_t9_dn0 = assign62580_e97125_d_n0;
        locals.var_t9_dn2 = assign62580_e97125_d_n2;
        locals.var_t9_dn4 = assign62580_e97125_d_n4;
        locals.var_t9_dn5 = assign62580_e97125_d_n5;
        locals.var_t9_dn6 = assign62580_e97125_d_n6;
        locals.var_t9_dn7 = assign62580_e97125_d_n7;
        locals.var_t9_dn8 = assign62580_e97125_d_n8;
        locals.var_t9_dn9 = assign62580_e97125_d_n9;
        locals.var_t9_dn10 = assign62580_e97125_d_n10;
        locals.var_t9_dn11 = assign62580_e97125_d_n11;
        locals.var_t9_dn14 = assign62580_e97125_d_n14;

    }

    pub(super) fn stamp_transient_block_221(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62590_e97135, assign62590_e97135_d_n0, assign62590_e97135_d_n2, assign62590_e97135_d_n4, assign62590_e97135_d_n5, assign62590_e97135_d_n6, assign62590_e97135_d_n7, assign62590_e97135_d_n8, assign62590_e97135_d_n9, assign62590_e97135_d_n10, assign62590_e97135_d_n11, assign62590_e97135_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62590_e97135;
        locals.var_t9_dn0 = assign62590_e97135_d_n0;
        locals.var_t9_dn2 = assign62590_e97135_d_n2;
        locals.var_t9_dn4 = assign62590_e97135_d_n4;
        locals.var_t9_dn5 = assign62590_e97135_d_n5;
        locals.var_t9_dn6 = assign62590_e97135_d_n6;
        locals.var_t9_dn7 = assign62590_e97135_d_n7;
        locals.var_t9_dn8 = assign62590_e97135_d_n8;
        locals.var_t9_dn9 = assign62590_e97135_d_n9;
        locals.var_t9_dn10 = assign62590_e97135_d_n10;
        locals.var_t9_dn11 = assign62590_e97135_d_n11;
        locals.var_t9_dn14 = assign62590_e97135_d_n14;

        let assign62600_e97138: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign62600_e97138;

        let (assign62610_e97149, assign62610_e97149_d_n0, assign62610_e97149_d_n2, assign62610_e97149_d_n4, assign62610_e97149_d_n5, assign62610_e97149_d_n6, assign62610_e97149_d_n7, assign62610_e97149_d_n8, assign62610_e97149_d_n9, assign62610_e97149_d_n10, assign62610_e97149_d_n11, assign62610_e97149_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62610_e97147: f64 = (locals.var_beta * locals.var_gdl0);
        (assign62610_e97147, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62610_e97149;
        locals.var_t1_dn0 = assign62610_e97149_d_n0;
        locals.var_t1_dn2 = assign62610_e97149_d_n2;
        locals.var_t1_dn4 = assign62610_e97149_d_n4;
        locals.var_t1_dn5 = assign62610_e97149_d_n5;
        locals.var_t1_dn6 = assign62610_e97149_d_n6;
        locals.var_t1_dn7 = assign62610_e97149_d_n7;
        locals.var_t1_dn8 = assign62610_e97149_d_n8;
        locals.var_t1_dn9 = assign62610_e97149_d_n9;
        locals.var_t1_dn10 = assign62610_e97149_d_n10;
        locals.var_t1_dn11 = assign62610_e97149_d_n11;
        locals.var_t1_dn14 = assign62610_e97149_d_n14;

        let (assign62620_e97160, assign62620_e97160_d_n0, assign62620_e97160_d_n2, assign62620_e97160_d_n4, assign62620_e97160_d_n5, assign62620_e97160_d_n6, assign62620_e97160_d_n7, assign62620_e97160_d_n8, assign62620_e97160_d_n9, assign62620_e97160_d_n10, assign62620_e97160_d_n11, assign62620_e97160_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62620_e97158: f64 = (locals.var_cox * locals.var_t1);
        (assign62620_e97158, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62620_e97160;
        locals.var_t2_dn0 = assign62620_e97160_d_n0;
        locals.var_t2_dn2 = assign62620_e97160_d_n2;
        locals.var_t2_dn4 = assign62620_e97160_d_n4;
        locals.var_t2_dn5 = assign62620_e97160_d_n5;
        locals.var_t2_dn6 = assign62620_e97160_d_n6;
        locals.var_t2_dn7 = assign62620_e97160_d_n7;
        locals.var_t2_dn8 = assign62620_e97160_d_n8;
        locals.var_t2_dn9 = assign62620_e97160_d_n9;
        locals.var_t2_dn10 = assign62620_e97160_d_n10;
        locals.var_t2_dn11 = assign62620_e97160_d_n11;
        locals.var_t2_dn14 = assign62620_e97160_d_n14;

        let (assign62630_e97171, assign62630_e97171_d_n0, assign62630_e97171_d_n2, assign62630_e97171_d_n4, assign62630_e97171_d_n5, assign62630_e97171_d_n6, assign62630_e97171_d_n7, assign62630_e97171_d_n8, assign62630_e97171_d_n9, assign62630_e97171_d_n10, assign62630_e97171_d_n11, assign62630_e97171_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62630_e97169: f64 = (locals.var_t2 * locals.var_vdsz__blk441);
        (assign62630_e97169, ((locals.var_t2_dn0 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62630_e97171;
        locals.var_t8_dn0 = assign62630_e97171_d_n0;
        locals.var_t8_dn2 = assign62630_e97171_d_n2;
        locals.var_t8_dn4 = assign62630_e97171_d_n4;
        locals.var_t8_dn5 = assign62630_e97171_d_n5;
        locals.var_t8_dn6 = assign62630_e97171_d_n6;
        locals.var_t8_dn7 = assign62630_e97171_d_n7;
        locals.var_t8_dn8 = assign62630_e97171_d_n8;
        locals.var_t8_dn9 = assign62630_e97171_d_n9;
        locals.var_t8_dn10 = assign62630_e97171_d_n10;
        locals.var_t8_dn11 = assign62630_e97171_d_n11;
        locals.var_t8_dn14 = assign62630_e97171_d_n14;

        let (assign62640_e97181, assign62640_e97181_d_n0, assign62640_e97181_d_n2, assign62640_e97181_d_n4, assign62640_e97181_d_n5, assign62640_e97181_d_n6, assign62640_e97181_d_n7, assign62640_e97181_d_n8, assign62640_e97181_d_n9, assign62640_e97181_d_n10, assign62640_e97181_d_n11, assign62640_e97181_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62640_e97181;
        locals.var_t8_dn0 = assign62640_e97181_d_n0;
        locals.var_t8_dn2 = assign62640_e97181_d_n2;
        locals.var_t8_dn4 = assign62640_e97181_d_n4;
        locals.var_t8_dn5 = assign62640_e97181_d_n5;
        locals.var_t8_dn6 = assign62640_e97181_d_n6;
        locals.var_t8_dn7 = assign62640_e97181_d_n7;
        locals.var_t8_dn8 = assign62640_e97181_d_n8;
        locals.var_t8_dn9 = assign62640_e97181_d_n9;
        locals.var_t8_dn10 = assign62640_e97181_d_n10;
        locals.var_t8_dn11 = assign62640_e97181_d_n11;
        locals.var_t8_dn14 = assign62640_e97181_d_n14;

        let assign62650_e97184: f64 = (locals.var_t9 + locals.var_t8);
        let assign62650_e97186: f64 = if assign62650_e97184 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign62650_e97186;

        let (assign62660_e97199, assign62660_e97199_d_n0, assign62660_e97199_d_n2, assign62660_e97199_d_n4, assign62660_e97199_d_n5, assign62660_e97199_d_n6, assign62660_e97199_d_n7, assign62660_e97199_d_n8, assign62660_e97199_d_n9, assign62660_e97199_d_n10, assign62660_e97199_d_n11, assign62660_e97199_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62660_e97196: f64 = (locals.var_t9 + locals.var_t8);
        let assign62660_e97197: f64 = (locals.var_pds * assign62660_e97196);
        (assign62660_e97197, ((locals.var_pds_dn0 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign62660_e97199;
        locals.var_idd1_dn0 = assign62660_e97199_d_n0;
        locals.var_idd1_dn2 = assign62660_e97199_d_n2;
        locals.var_idd1_dn4 = assign62660_e97199_d_n4;
        locals.var_idd1_dn5 = assign62660_e97199_d_n5;
        locals.var_idd1_dn6 = assign62660_e97199_d_n6;
        locals.var_idd1_dn7 = assign62660_e97199_d_n7;
        locals.var_idd1_dn8 = assign62660_e97199_d_n8;
        locals.var_idd1_dn9 = assign62660_e97199_d_n9;
        locals.var_idd1_dn10 = assign62660_e97199_d_n10;
        locals.var_idd1_dn11 = assign62660_e97199_d_n11;
        locals.var_idd1_dn14 = assign62660_e97199_d_n14;

        let (assign62670_e97212, assign62670_e97212_d_n0, assign62670_e97212_d_n2, assign62670_e97212_d_n4, assign62670_e97212_d_n5, assign62670_e97212_d_n6, assign62670_e97212_d_n7, assign62670_e97212_d_n8, assign62670_e97212_d_n9, assign62670_e97212_d_n10, assign62670_e97212_d_n11, assign62670_e97212_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62670_e97208: f64 = (locals.var_betawl * locals.var_idd1);
        let assign62670_e97210: f64 = (assign62670_e97208 * locals.var_mu);
        (assign62670_e97210, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn14)),)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62670_e97212;
        locals.var_idspt0_dn0 = assign62670_e97212_d_n0;
        locals.var_idspt0_dn2 = assign62670_e97212_d_n2;
        locals.var_idspt0_dn4 = assign62670_e97212_d_n4;
        locals.var_idspt0_dn5 = assign62670_e97212_d_n5;
        locals.var_idspt0_dn6 = assign62670_e97212_d_n6;
        locals.var_idspt0_dn7 = assign62670_e97212_d_n7;
        locals.var_idspt0_dn8 = assign62670_e97212_d_n8;
        locals.var_idspt0_dn9 = assign62670_e97212_d_n9;
        locals.var_idspt0_dn10 = assign62670_e97212_d_n10;
        locals.var_idspt0_dn11 = assign62670_e97212_d_n11;
        locals.var_idspt0_dn14 = assign62670_e97212_d_n14;

        let (assign62680_e97223, assign62680_e97223_d_n0, assign62680_e97223_d_n2, assign62680_e97223_d_n4, assign62680_e97223_d_n5, assign62680_e97223_d_n6, assign62680_e97223_d_n7, assign62680_e97223_d_n8, assign62680_e97223_d_n9, assign62680_e97223_d_n10, assign62680_e97223_d_n11, assign62680_e97223_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62680_e97221: f64 = (locals.var_ids0 + locals.var_idspt0);
        (assign62680_e97221, (locals.var_ids0_dn0 + locals.var_idspt0_dn0), (locals.var_ids0_dn2 + locals.var_idspt0_dn2), (locals.var_ids0_dn4 + locals.var_idspt0_dn4), (locals.var_ids0_dn5 + locals.var_idspt0_dn5), (locals.var_ids0_dn6 + locals.var_idspt0_dn6), (locals.var_ids0_dn7 + locals.var_idspt0_dn7), (locals.var_ids0_dn8 + locals.var_idspt0_dn8), (locals.var_ids0_dn9 + locals.var_idspt0_dn9), (locals.var_ids0_dn10 + locals.var_idspt0_dn10), (locals.var_ids0_dn11 + locals.var_idspt0_dn11), (locals.var_ids0_dn14 + locals.var_idspt0_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62680_e97223;
        locals.var_ids0_dn0 = assign62680_e97223_d_n0;
        locals.var_ids0_dn2 = assign62680_e97223_d_n2;
        locals.var_ids0_dn4 = assign62680_e97223_d_n4;
        locals.var_ids0_dn5 = assign62680_e97223_d_n5;
        locals.var_ids0_dn6 = assign62680_e97223_d_n6;
        locals.var_ids0_dn7 = assign62680_e97223_d_n7;
        locals.var_ids0_dn8 = assign62680_e97223_d_n8;
        locals.var_ids0_dn9 = assign62680_e97223_d_n9;
        locals.var_ids0_dn10 = assign62680_e97223_d_n10;
        locals.var_ids0_dn11 = assign62680_e97223_d_n11;
        locals.var_ids0_dn14 = assign62680_e97223_d_n14;

        let (assign62690_e97233, assign62690_e97233_d_n0, assign62690_e97233_d_n2, assign62690_e97233_d_n4, assign62690_e97233_d_n5, assign62690_e97233_d_n6, assign62690_e97233_d_n7, assign62690_e97233_d_n8, assign62690_e97233_d_n9, assign62690_e97233_d_n10, assign62690_e97233_d_n11, assign62690_e97233_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62690_e97233;
        locals.var_idspt0_dn0 = assign62690_e97233_d_n0;
        locals.var_idspt0_dn2 = assign62690_e97233_d_n2;
        locals.var_idspt0_dn4 = assign62690_e97233_d_n4;
        locals.var_idspt0_dn5 = assign62690_e97233_d_n5;
        locals.var_idspt0_dn6 = assign62690_e97233_d_n6;
        locals.var_idspt0_dn7 = assign62690_e97233_d_n7;
        locals.var_idspt0_dn8 = assign62690_e97233_d_n8;
        locals.var_idspt0_dn9 = assign62690_e97233_d_n9;
        locals.var_idspt0_dn10 = assign62690_e97233_d_n10;
        locals.var_idspt0_dn11 = assign62690_e97233_d_n11;
        locals.var_idspt0_dn14 = assign62690_e97233_d_n14;

        let assign62700_e97240: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign62700_e97240;

        let assign62710_e97243: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign62710_e97243;

        let (assign62720_e97254, assign62720_e97254_d_n0, assign62720_e97254_d_n2, assign62720_e97254_d_n4, assign62720_e97254_d_n5, assign62720_e97254_d_n6, assign62720_e97254_d_n7, assign62720_e97254_d_n8, assign62720_e97254_d_n9, assign62720_e97254_d_n10, assign62720_e97254_d_n11, assign62720_e97254_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62720_e97254;
        locals.var_t4_dn0 = assign62720_e97254_d_n0;
        locals.var_t4_dn2 = assign62720_e97254_d_n2;
        locals.var_t4_dn4 = assign62720_e97254_d_n4;
        locals.var_t4_dn5 = assign62720_e97254_d_n5;
        locals.var_t4_dn6 = assign62720_e97254_d_n6;
        locals.var_t4_dn7 = assign62720_e97254_d_n7;
        locals.var_t4_dn8 = assign62720_e97254_d_n8;
        locals.var_t4_dn9 = assign62720_e97254_d_n9;
        locals.var_t4_dn10 = assign62720_e97254_d_n10;
        locals.var_t4_dn11 = assign62720_e97254_d_n11;
        locals.var_t4_dn14 = assign62720_e97254_d_n14;

        let (assign62730_e97269, assign62730_e97269_d_n0, assign62730_e97269_d_n2, assign62730_e97269_d_n4, assign62730_e97269_d_n5, assign62730_e97269_d_n6, assign62730_e97269_d_n7, assign62730_e97269_d_n8, assign62730_e97269_d_n9, assign62730_e97269_d_n10, assign62730_e97269_d_n11, assign62730_e97269_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62730_e97266: f64 = (locals.var_vgse - p.p300);
        let assign62730_e97267: f64 = (locals.var_uc_rd24 * assign62730_e97266);
        (assign62730_e97267, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn7), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62730_e97269;
        locals.var_t1_dn0 = assign62730_e97269_d_n0;
        locals.var_t1_dn2 = assign62730_e97269_d_n2;
        locals.var_t1_dn4 = assign62730_e97269_d_n4;
        locals.var_t1_dn5 = assign62730_e97269_d_n5;
        locals.var_t1_dn6 = assign62730_e97269_d_n6;
        locals.var_t1_dn7 = assign62730_e97269_d_n7;
        locals.var_t1_dn8 = assign62730_e97269_d_n8;
        locals.var_t1_dn9 = assign62730_e97269_d_n9;
        locals.var_t1_dn10 = assign62730_e97269_d_n10;
        locals.var_t1_dn11 = assign62730_e97269_d_n11;
        locals.var_t1_dn14 = assign62730_e97269_d_n14;

        let (assign62740_e97286, assign62740_e97286_d_n0, assign62740_e97286_d_n2, assign62740_e97286_d_n4, assign62740_e97286_d_n5, assign62740_e97286_d_n6, assign62740_e97286_d_n7, assign62740_e97286_d_n8, assign62740_e97286_d_n9, assign62740_e97286_d_n10, assign62740_e97286_d_n11, assign62740_e97286_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62740_e97280: f64 = (locals.var_t1 - locals.var_t4);
        let assign62740_e97283: f64 = (0.01 * 0.01);
        let assign62740_e97284: f64 = (assign62740_e97280 - assign62740_e97283);
        (assign62740_e97284, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62740_e97286;
        locals.var_tmf1_dn0 = assign62740_e97286_d_n0;
        locals.var_tmf1_dn2 = assign62740_e97286_d_n2;
        locals.var_tmf1_dn4 = assign62740_e97286_d_n4;
        locals.var_tmf1_dn5 = assign62740_e97286_d_n5;
        locals.var_tmf1_dn6 = assign62740_e97286_d_n6;
        locals.var_tmf1_dn7 = assign62740_e97286_d_n7;
        locals.var_tmf1_dn8 = assign62740_e97286_d_n8;
        locals.var_tmf1_dn9 = assign62740_e97286_d_n9;
        locals.var_tmf1_dn10 = assign62740_e97286_d_n10;
        locals.var_tmf1_dn11 = assign62740_e97286_d_n11;
        locals.var_tmf1_dn14 = assign62740_e97286_d_n14;

        let (assign62750_e97303, assign62750_e97303_d_n0, assign62750_e97303_d_n2, assign62750_e97303_d_n4, assign62750_e97303_d_n5, assign62750_e97303_d_n6, assign62750_e97303_d_n7, assign62750_e97303_d_n8, assign62750_e97303_d_n9, assign62750_e97303_d_n10, assign62750_e97303_d_n11, assign62750_e97303_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62750_e97297: f64 = (4.0 * locals.var_t4);
        let assign62750_e97300: f64 = (0.01 * 0.01);
        let assign62750_e97301: f64 = (assign62750_e97297 * assign62750_e97300);
        (assign62750_e97301, ((4.0 * locals.var_t4_dn0) * assign62750_e97300), ((4.0 * locals.var_t4_dn2) * assign62750_e97300), ((4.0 * locals.var_t4_dn4) * assign62750_e97300), ((4.0 * locals.var_t4_dn5) * assign62750_e97300), ((4.0 * locals.var_t4_dn6) * assign62750_e97300), ((4.0 * locals.var_t4_dn7) * assign62750_e97300), ((4.0 * locals.var_t4_dn8) * assign62750_e97300), ((4.0 * locals.var_t4_dn9) * assign62750_e97300), ((4.0 * locals.var_t4_dn10) * assign62750_e97300), ((4.0 * locals.var_t4_dn11) * assign62750_e97300), ((4.0 * locals.var_t4_dn14) * assign62750_e97300),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62750_e97303;
        locals.var_tmf2_dn0 = assign62750_e97303_d_n0;
        locals.var_tmf2_dn2 = assign62750_e97303_d_n2;
        locals.var_tmf2_dn4 = assign62750_e97303_d_n4;
        locals.var_tmf2_dn5 = assign62750_e97303_d_n5;
        locals.var_tmf2_dn6 = assign62750_e97303_d_n6;
        locals.var_tmf2_dn7 = assign62750_e97303_d_n7;
        locals.var_tmf2_dn8 = assign62750_e97303_d_n8;
        locals.var_tmf2_dn9 = assign62750_e97303_d_n9;
        locals.var_tmf2_dn10 = assign62750_e97303_d_n10;
        locals.var_tmf2_dn11 = assign62750_e97303_d_n11;
        locals.var_tmf2_dn14 = assign62750_e97303_d_n14;

        let (assign62760_e97320, assign62760_e97320_d_n0, assign62760_e97320_d_n2, assign62760_e97320_d_n4, assign62760_e97320_d_n5, assign62760_e97320_d_n6, assign62760_e97320_d_n7, assign62760_e97320_d_n8, assign62760_e97320_d_n9, assign62760_e97320_d_n10, assign62760_e97320_d_n11, assign62760_e97320_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let (assign62760_e97318, assign62760_e97318_d_n0, assign62760_e97318_d_n2, assign62760_e97318_d_n4, assign62760_e97318_d_n5, assign62760_e97318_d_n6, assign62760_e97318_d_n7, assign62760_e97318_d_n8, assign62760_e97318_d_n9, assign62760_e97318_d_n10, assign62760_e97318_d_n11, assign62760_e97318_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62760_e97317: f64 = (-locals.var_tmf2);
                (assign62760_e97317, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62760_e97318, assign62760_e97318_d_n0, assign62760_e97318_d_n2, assign62760_e97318_d_n4, assign62760_e97318_d_n5, assign62760_e97318_d_n6, assign62760_e97318_d_n7, assign62760_e97318_d_n8, assign62760_e97318_d_n9, assign62760_e97318_d_n10, assign62760_e97318_d_n11, assign62760_e97318_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62760_e97320;
        locals.var_tmf2_dn0 = assign62760_e97320_d_n0;
        locals.var_tmf2_dn2 = assign62760_e97320_d_n2;
        locals.var_tmf2_dn4 = assign62760_e97320_d_n4;
        locals.var_tmf2_dn5 = assign62760_e97320_d_n5;
        locals.var_tmf2_dn6 = assign62760_e97320_d_n6;
        locals.var_tmf2_dn7 = assign62760_e97320_d_n7;
        locals.var_tmf2_dn8 = assign62760_e97320_d_n8;
        locals.var_tmf2_dn9 = assign62760_e97320_d_n9;
        locals.var_tmf2_dn10 = assign62760_e97320_d_n10;
        locals.var_tmf2_dn11 = assign62760_e97320_d_n11;
        locals.var_tmf2_dn14 = assign62760_e97320_d_n14;

        let (assign62770_e97336, assign62770_e97336_d_n0, assign62770_e97336_d_n2, assign62770_e97336_d_n4, assign62770_e97336_d_n5, assign62770_e97336_d_n6, assign62770_e97336_d_n7, assign62770_e97336_d_n8, assign62770_e97336_d_n9, assign62770_e97336_d_n10, assign62770_e97336_d_n11, assign62770_e97336_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62770_e97331: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62770_e97333: f64 = (assign62770_e97331 + locals.var_tmf2);
        let assign62770_e97334: f64 = (assign62770_e97333).sqrt();
        (assign62770_e97334, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62770_e97334)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62770_e97336;
        locals.var_tmf2_dn0 = assign62770_e97336_d_n0;
        locals.var_tmf2_dn2 = assign62770_e97336_d_n2;
        locals.var_tmf2_dn4 = assign62770_e97336_d_n4;
        locals.var_tmf2_dn5 = assign62770_e97336_d_n5;
        locals.var_tmf2_dn6 = assign62770_e97336_d_n6;
        locals.var_tmf2_dn7 = assign62770_e97336_d_n7;
        locals.var_tmf2_dn8 = assign62770_e97336_d_n8;
        locals.var_tmf2_dn9 = assign62770_e97336_d_n9;
        locals.var_tmf2_dn10 = assign62770_e97336_d_n10;
        locals.var_tmf2_dn11 = assign62770_e97336_d_n11;
        locals.var_tmf2_dn14 = assign62770_e97336_d_n14;

        let (assign62780_e97353, assign62780_e97353_d_n0, assign62780_e97353_d_n2, assign62780_e97353_d_n4, assign62780_e97353_d_n5, assign62780_e97353_d_n6, assign62780_e97353_d_n7, assign62780_e97353_d_n8, assign62780_e97353_d_n9, assign62780_e97353_d_n10, assign62780_e97353_d_n11, assign62780_e97353_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62780_e97349: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62780_e97350: f64 = (1.0 + assign62780_e97349);
        let assign62780_e97351: f64 = (0.5 * assign62780_e97350);
        (assign62780_e97351, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62780_e97353;
        locals.var_t0_dn0 = assign62780_e97353_d_n0;
        locals.var_t0_dn2 = assign62780_e97353_d_n2;
        locals.var_t0_dn4 = assign62780_e97353_d_n4;
        locals.var_t0_dn5 = assign62780_e97353_d_n5;
        locals.var_t0_dn6 = assign62780_e97353_d_n6;
        locals.var_t0_dn7 = assign62780_e97353_d_n7;
        locals.var_t0_dn8 = assign62780_e97353_d_n8;
        locals.var_t0_dn9 = assign62780_e97353_d_n9;
        locals.var_t0_dn10 = assign62780_e97353_d_n10;
        locals.var_t0_dn11 = assign62780_e97353_d_n11;
        locals.var_t0_dn14 = assign62780_e97353_d_n14;

        let (assign62790_e97370, assign62790_e97370_d_n0, assign62790_e97370_d_n2, assign62790_e97370_d_n4, assign62790_e97370_d_n5, assign62790_e97370_d_n6, assign62790_e97370_d_n7, assign62790_e97370_d_n8, assign62790_e97370_d_n9, assign62790_e97370_d_n10, assign62790_e97370_d_n11, assign62790_e97370_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62790_e97366: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62790_e97367: f64 = (0.5 * assign62790_e97366);
        let assign62790_e97368: f64 = (locals.var_t4 + assign62790_e97367);
        (assign62790_e97368, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62790_e97370;
        locals.var_t2_dn0 = assign62790_e97370_d_n0;
        locals.var_t2_dn2 = assign62790_e97370_d_n2;
        locals.var_t2_dn4 = assign62790_e97370_d_n4;
        locals.var_t2_dn5 = assign62790_e97370_d_n5;
        locals.var_t2_dn6 = assign62790_e97370_d_n6;
        locals.var_t2_dn7 = assign62790_e97370_d_n7;
        locals.var_t2_dn8 = assign62790_e97370_d_n8;
        locals.var_t2_dn9 = assign62790_e97370_d_n9;
        locals.var_t2_dn10 = assign62790_e97370_d_n10;
        locals.var_t2_dn11 = assign62790_e97370_d_n11;
        locals.var_t2_dn14 = assign62790_e97370_d_n14;

        let (assign62800_e97385, assign62800_e97385_d_n0, assign62800_e97385_d_n2, assign62800_e97385_d_n4, assign62800_e97385_d_n5, assign62800_e97385_d_n6, assign62800_e97385_d_n7, assign62800_e97385_d_n8, assign62800_e97385_d_n9, assign62800_e97385_d_n10, assign62800_e97385_d_n11, assign62800_e97385_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62800_e97382: f64 = (p.p296 + 1.0);
        let assign62800_e97383: f64 = (locals.var_t4 * assign62800_e97382);
        (assign62800_e97383, (locals.var_t4_dn0 * assign62800_e97382), (locals.var_t4_dn2 * assign62800_e97382), (locals.var_t4_dn4 * assign62800_e97382), (locals.var_t4_dn5 * assign62800_e97382), (locals.var_t4_dn6 * assign62800_e97382), (locals.var_t4_dn7 * assign62800_e97382), (locals.var_t4_dn8 * assign62800_e97382), (locals.var_t4_dn9 * assign62800_e97382), (locals.var_t4_dn10 * assign62800_e97382), (locals.var_t4_dn11 * assign62800_e97382), (locals.var_t4_dn14 * assign62800_e97382),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62800_e97385;
        locals.var_t3_dn0 = assign62800_e97385_d_n0;
        locals.var_t3_dn2 = assign62800_e97385_d_n2;
        locals.var_t3_dn4 = assign62800_e97385_d_n4;
        locals.var_t3_dn5 = assign62800_e97385_d_n5;
        locals.var_t3_dn6 = assign62800_e97385_d_n6;
        locals.var_t3_dn7 = assign62800_e97385_d_n7;
        locals.var_t3_dn8 = assign62800_e97385_d_n8;
        locals.var_t3_dn9 = assign62800_e97385_d_n9;
        locals.var_t3_dn10 = assign62800_e97385_d_n10;
        locals.var_t3_dn11 = assign62800_e97385_d_n11;
        locals.var_t3_dn14 = assign62800_e97385_d_n14;

        let (assign62810_e97402, assign62810_e97402_d_n0, assign62810_e97402_d_n2, assign62810_e97402_d_n4, assign62810_e97402_d_n5, assign62810_e97402_d_n6, assign62810_e97402_d_n7, assign62810_e97402_d_n8, assign62810_e97402_d_n9, assign62810_e97402_d_n10, assign62810_e97402_d_n11, assign62810_e97402_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62810_e97396: f64 = (locals.var_t3 - locals.var_t2);
        let assign62810_e97399: f64 = (0.01 * 0.01);
        let assign62810_e97400: f64 = (assign62810_e97396 - assign62810_e97399);
        (assign62810_e97400, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62810_e97402;
        locals.var_tmf1_dn0 = assign62810_e97402_d_n0;
        locals.var_tmf1_dn2 = assign62810_e97402_d_n2;
        locals.var_tmf1_dn4 = assign62810_e97402_d_n4;
        locals.var_tmf1_dn5 = assign62810_e97402_d_n5;
        locals.var_tmf1_dn6 = assign62810_e97402_d_n6;
        locals.var_tmf1_dn7 = assign62810_e97402_d_n7;
        locals.var_tmf1_dn8 = assign62810_e97402_d_n8;
        locals.var_tmf1_dn9 = assign62810_e97402_d_n9;
        locals.var_tmf1_dn10 = assign62810_e97402_d_n10;
        locals.var_tmf1_dn11 = assign62810_e97402_d_n11;
        locals.var_tmf1_dn14 = assign62810_e97402_d_n14;

        let (assign62820_e97419, assign62820_e97419_d_n0, assign62820_e97419_d_n2, assign62820_e97419_d_n4, assign62820_e97419_d_n5, assign62820_e97419_d_n6, assign62820_e97419_d_n7, assign62820_e97419_d_n8, assign62820_e97419_d_n9, assign62820_e97419_d_n10, assign62820_e97419_d_n11, assign62820_e97419_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62820_e97413: f64 = (4.0 * locals.var_t3);
        let assign62820_e97416: f64 = (0.01 * 0.01);
        let assign62820_e97417: f64 = (assign62820_e97413 * assign62820_e97416);
        (assign62820_e97417, ((4.0 * locals.var_t3_dn0) * assign62820_e97416), ((4.0 * locals.var_t3_dn2) * assign62820_e97416), ((4.0 * locals.var_t3_dn4) * assign62820_e97416), ((4.0 * locals.var_t3_dn5) * assign62820_e97416), ((4.0 * locals.var_t3_dn6) * assign62820_e97416), ((4.0 * locals.var_t3_dn7) * assign62820_e97416), ((4.0 * locals.var_t3_dn8) * assign62820_e97416), ((4.0 * locals.var_t3_dn9) * assign62820_e97416), ((4.0 * locals.var_t3_dn10) * assign62820_e97416), ((4.0 * locals.var_t3_dn11) * assign62820_e97416), ((4.0 * locals.var_t3_dn14) * assign62820_e97416),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62820_e97419;
        locals.var_tmf2_dn0 = assign62820_e97419_d_n0;
        locals.var_tmf2_dn2 = assign62820_e97419_d_n2;
        locals.var_tmf2_dn4 = assign62820_e97419_d_n4;
        locals.var_tmf2_dn5 = assign62820_e97419_d_n5;
        locals.var_tmf2_dn6 = assign62820_e97419_d_n6;
        locals.var_tmf2_dn7 = assign62820_e97419_d_n7;
        locals.var_tmf2_dn8 = assign62820_e97419_d_n8;
        locals.var_tmf2_dn9 = assign62820_e97419_d_n9;
        locals.var_tmf2_dn10 = assign62820_e97419_d_n10;
        locals.var_tmf2_dn11 = assign62820_e97419_d_n11;
        locals.var_tmf2_dn14 = assign62820_e97419_d_n14;

        let (assign62830_e97436, assign62830_e97436_d_n0, assign62830_e97436_d_n2, assign62830_e97436_d_n4, assign62830_e97436_d_n5, assign62830_e97436_d_n6, assign62830_e97436_d_n7, assign62830_e97436_d_n8, assign62830_e97436_d_n9, assign62830_e97436_d_n10, assign62830_e97436_d_n11, assign62830_e97436_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let (assign62830_e97434, assign62830_e97434_d_n0, assign62830_e97434_d_n2, assign62830_e97434_d_n4, assign62830_e97434_d_n5, assign62830_e97434_d_n6, assign62830_e97434_d_n7, assign62830_e97434_d_n8, assign62830_e97434_d_n9, assign62830_e97434_d_n10, assign62830_e97434_d_n11, assign62830_e97434_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62830_e97433: f64 = (-locals.var_tmf2);
                (assign62830_e97433, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62830_e97434, assign62830_e97434_d_n0, assign62830_e97434_d_n2, assign62830_e97434_d_n4, assign62830_e97434_d_n5, assign62830_e97434_d_n6, assign62830_e97434_d_n7, assign62830_e97434_d_n8, assign62830_e97434_d_n9, assign62830_e97434_d_n10, assign62830_e97434_d_n11, assign62830_e97434_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62830_e97436;
        locals.var_tmf2_dn0 = assign62830_e97436_d_n0;
        locals.var_tmf2_dn2 = assign62830_e97436_d_n2;
        locals.var_tmf2_dn4 = assign62830_e97436_d_n4;
        locals.var_tmf2_dn5 = assign62830_e97436_d_n5;
        locals.var_tmf2_dn6 = assign62830_e97436_d_n6;
        locals.var_tmf2_dn7 = assign62830_e97436_d_n7;
        locals.var_tmf2_dn8 = assign62830_e97436_d_n8;
        locals.var_tmf2_dn9 = assign62830_e97436_d_n9;
        locals.var_tmf2_dn10 = assign62830_e97436_d_n10;
        locals.var_tmf2_dn11 = assign62830_e97436_d_n11;
        locals.var_tmf2_dn14 = assign62830_e97436_d_n14;

        let (assign62840_e97452, assign62840_e97452_d_n0, assign62840_e97452_d_n2, assign62840_e97452_d_n4, assign62840_e97452_d_n5, assign62840_e97452_d_n6, assign62840_e97452_d_n7, assign62840_e97452_d_n8, assign62840_e97452_d_n9, assign62840_e97452_d_n10, assign62840_e97452_d_n11, assign62840_e97452_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62840_e97447: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62840_e97449: f64 = (assign62840_e97447 + locals.var_tmf2);
        let assign62840_e97450: f64 = (assign62840_e97449).sqrt();
        (assign62840_e97450, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62840_e97450)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62840_e97452;
        locals.var_tmf2_dn0 = assign62840_e97452_d_n0;
        locals.var_tmf2_dn2 = assign62840_e97452_d_n2;
        locals.var_tmf2_dn4 = assign62840_e97452_d_n4;
        locals.var_tmf2_dn5 = assign62840_e97452_d_n5;
        locals.var_tmf2_dn6 = assign62840_e97452_d_n6;
        locals.var_tmf2_dn7 = assign62840_e97452_d_n7;
        locals.var_tmf2_dn8 = assign62840_e97452_d_n8;
        locals.var_tmf2_dn9 = assign62840_e97452_d_n9;
        locals.var_tmf2_dn10 = assign62840_e97452_d_n10;
        locals.var_tmf2_dn11 = assign62840_e97452_d_n11;
        locals.var_tmf2_dn14 = assign62840_e97452_d_n14;

    }

    pub(super) fn stamp_transient_block_222(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62850_e97469, assign62850_e97469_d_n0, assign62850_e97469_d_n2, assign62850_e97469_d_n4, assign62850_e97469_d_n5, assign62850_e97469_d_n6, assign62850_e97469_d_n7, assign62850_e97469_d_n8, assign62850_e97469_d_n9, assign62850_e97469_d_n10, assign62850_e97469_d_n11, assign62850_e97469_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62850_e97465: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62850_e97466: f64 = (1.0 + assign62850_e97465);
        let assign62850_e97467: f64 = (0.5 * assign62850_e97466);
        (assign62850_e97467, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62850_e97469;
        locals.var_t0_dn0 = assign62850_e97469_d_n0;
        locals.var_t0_dn2 = assign62850_e97469_d_n2;
        locals.var_t0_dn4 = assign62850_e97469_d_n4;
        locals.var_t0_dn5 = assign62850_e97469_d_n5;
        locals.var_t0_dn6 = assign62850_e97469_d_n6;
        locals.var_t0_dn7 = assign62850_e97469_d_n7;
        locals.var_t0_dn8 = assign62850_e97469_d_n8;
        locals.var_t0_dn9 = assign62850_e97469_d_n9;
        locals.var_t0_dn10 = assign62850_e97469_d_n10;
        locals.var_t0_dn11 = assign62850_e97469_d_n11;
        locals.var_t0_dn14 = assign62850_e97469_d_n14;

        let (assign62860_e97486, assign62860_e97486_d_n0, assign62860_e97486_d_n2, assign62860_e97486_d_n4, assign62860_e97486_d_n5, assign62860_e97486_d_n6, assign62860_e97486_d_n7, assign62860_e97486_d_n8, assign62860_e97486_d_n9, assign62860_e97486_d_n10, assign62860_e97486_d_n11, assign62860_e97486_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62860_e97482: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62860_e97483: f64 = (0.5 * assign62860_e97482);
        let assign62860_e97484: f64 = (locals.var_t3 - assign62860_e97483);
        (assign62860_e97484, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62860_e97486;
        locals.var_t7_dn0 = assign62860_e97486_d_n0;
        locals.var_t7_dn2 = assign62860_e97486_d_n2;
        locals.var_t7_dn4 = assign62860_e97486_d_n4;
        locals.var_t7_dn5 = assign62860_e97486_d_n5;
        locals.var_t7_dn6 = assign62860_e97486_d_n6;
        locals.var_t7_dn7 = assign62860_e97486_d_n7;
        locals.var_t7_dn8 = assign62860_e97486_d_n8;
        locals.var_t7_dn9 = assign62860_e97486_d_n9;
        locals.var_t7_dn10 = assign62860_e97486_d_n10;
        locals.var_t7_dn11 = assign62860_e97486_d_n11;
        locals.var_t7_dn14 = assign62860_e97486_d_n14;

        let (assign62870_e97498, assign62870_e97498_d_n0, assign62870_e97498_d_n2, assign62870_e97498_d_n4, assign62870_e97498_d_n5, assign62870_e97498_d_n6, assign62870_e97498_d_n7, assign62870_e97498_d_n8, assign62870_e97498_d_n9, assign62870_e97498_d_n10, assign62870_e97498_d_n11, assign62870_e97498_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62870_e97498;
        locals.var_t7_dn0 = assign62870_e97498_d_n0;
        locals.var_t7_dn2 = assign62870_e97498_d_n2;
        locals.var_t7_dn4 = assign62870_e97498_d_n4;
        locals.var_t7_dn5 = assign62870_e97498_d_n5;
        locals.var_t7_dn6 = assign62870_e97498_d_n6;
        locals.var_t7_dn7 = assign62870_e97498_d_n7;
        locals.var_t7_dn8 = assign62870_e97498_d_n8;
        locals.var_t7_dn9 = assign62870_e97498_d_n9;
        locals.var_t7_dn10 = assign62870_e97498_d_n10;
        locals.var_t7_dn11 = assign62870_e97498_d_n11;
        locals.var_t7_dn14 = assign62870_e97498_d_n14;

        let assign62880_e97501: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign62880_e97501;

        let (assign62890_e97512, assign62890_e97512_d_n0, assign62890_e97512_d_n2,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62890_e97512;
        locals.var_vdse_eff_dn0 = assign62890_e97512_d_n0;
        locals.var_vdse_eff_dn2 = assign62890_e97512_d_n2;

        let (assign62900_e97524, assign62900_e97524_d_n0, assign62900_e97524_d_n2,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62900_e97524;
        locals.var_vdse_eff_dn0 = assign62900_e97524_d_n0;
        locals.var_vdse_eff_dn2 = assign62900_e97524_d_n2;

        let assign62910_e97528: f64 = (20.0 * 1e-12);
        let assign62910_e97529: f64 = if locals.var_vdse_eff < assign62910_e97528 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign62910_e97529;

        let (assign62920_e97560,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62920_e97540: f64 = (20.0 + 1.0);
        let assign62920_e97543: f64 = (p.p297 - 1.0);
        let assign62920_e97544: f64 = (assign62920_e97540).powf(assign62920_e97543);
        let assign62920_e97547: f64 = (20.0 + 1.0);
        let assign62920_e97550: f64 = (0.5 * p.p297);
        let assign62920_e97552: f64 = (assign62920_e97550 * 20.0);
        let assign62920_e97553: f64 = (assign62920_e97547 - assign62920_e97552);
        let assign62920_e97554: f64 = (assign62920_e97544 * assign62920_e97553);
        let assign62920_e97557: f64 = (1e-12_f64).powf(p.p297);
        let assign62920_e97558: f64 = (assign62920_e97554 * assign62920_e97557);
        (assign62920_e97558,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign62920_e97560;

        let (assign62930_e97589,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62930_e97571: f64 = (0.5 * p.p297);
        let assign62930_e97574: f64 = (20.0 + 1.0);
        let assign62930_e97577: f64 = (p.p297 - 1.0);
        let assign62930_e97578: f64 = (assign62930_e97574).powf(assign62930_e97577);
        let assign62930_e97579: f64 = (assign62930_e97571 * assign62930_e97578);
        let assign62930_e97581: f64 = (assign62930_e97579 / 20.0);
        let assign62930_e97585: f64 = (p.p297 - 2.0);
        let assign62930_e97586: f64 = (1e-12_f64).powf(assign62930_e97585);
        let assign62930_e97587: f64 = (assign62930_e97581 * assign62930_e97586);
        (assign62930_e97587,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign62930_e97589;

        let (assign62940_e97606, assign62940_e97606_d_n0, assign62940_e97606_d_n2, assign62940_e97606_d_n4, assign62940_e97606_d_n5, assign62940_e97606_d_n6, assign62940_e97606_d_n7, assign62940_e97606_d_n8, assign62940_e97606_d_n9, assign62940_e97606_d_n10, assign62940_e97606_d_n11, assign62940_e97606_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62940_e97601: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign62940_e97603: f64 = (assign62940_e97601 * locals.var_vdse_eff);
        let assign62940_e97604: f64 = (locals.var_ra_alpha + assign62940_e97603);
        (assign62940_e97604, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign62940_e97601 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign62940_e97601 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62940_e97606;
        locals.var_t1_dn0 = assign62940_e97606_d_n0;
        locals.var_t1_dn2 = assign62940_e97606_d_n2;
        locals.var_t1_dn4 = assign62940_e97606_d_n4;
        locals.var_t1_dn5 = assign62940_e97606_d_n5;
        locals.var_t1_dn6 = assign62940_e97606_d_n6;
        locals.var_t1_dn7 = assign62940_e97606_d_n7;
        locals.var_t1_dn8 = assign62940_e97606_d_n8;
        locals.var_t1_dn9 = assign62940_e97606_d_n9;
        locals.var_t1_dn10 = assign62940_e97606_d_n10;
        locals.var_t1_dn11 = assign62940_e97606_d_n11;
        locals.var_t1_dn14 = assign62940_e97606_d_n14;

        let (assign62950_e97622, assign62950_e97622_d_n0, assign62950_e97622_d_n2, assign62950_e97622_d_n4, assign62950_e97622_d_n5, assign62950_e97622_d_n6, assign62950_e97622_d_n7, assign62950_e97622_d_n8, assign62950_e97622_d_n9, assign62950_e97622_d_n10, assign62950_e97622_d_n11, assign62950_e97622_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign62950_e97618: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62950_e97620: f64 = (assign62950_e97618).powf(p.p297);
        (assign62950_e97620, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62950_e97618).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62950_e97620 * (p.p297 * (locals.var_vdse_eff_dn0 / assign62950_e97618))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62950_e97618).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62950_e97620 * (p.p297 * (locals.var_vdse_eff_dn2 / assign62950_e97618))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62950_e97622;
        locals.var_t1_dn0 = assign62950_e97622_d_n0;
        locals.var_t1_dn2 = assign62950_e97622_d_n2;
        locals.var_t1_dn4 = assign62950_e97622_d_n4;
        locals.var_t1_dn5 = assign62950_e97622_d_n5;
        locals.var_t1_dn6 = assign62950_e97622_d_n6;
        locals.var_t1_dn7 = assign62950_e97622_d_n7;
        locals.var_t1_dn8 = assign62950_e97622_d_n8;
        locals.var_t1_dn9 = assign62950_e97622_d_n9;
        locals.var_t1_dn10 = assign62950_e97622_d_n10;
        locals.var_t1_dn11 = assign62950_e97622_d_n11;
        locals.var_t1_dn14 = assign62950_e97622_d_n14;

        let (assign62960_e97635, assign62960_e97635_d_n0, assign62960_e97635_d_n2, assign62960_e97635_d_n4, assign62960_e97635_d_n5, assign62960_e97635_d_n6, assign62960_e97635_d_n7, assign62960_e97635_d_n8, assign62960_e97635_d_n9, assign62960_e97635_d_n10, assign62960_e97635_d_n11, assign62960_e97635_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62960_e97631: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62960_e97633: f64 = (assign62960_e97631).powf(p.p299);
        (assign62960_e97633, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62960_e97631).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62960_e97633 * (p.p299 * (locals.var_vdse_eff_dn0 / assign62960_e97631))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62960_e97631).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62960_e97633 * (p.p299 * (locals.var_vdse_eff_dn2 / assign62960_e97631))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62960_e97635;
        locals.var_t9_dn0 = assign62960_e97635_d_n0;
        locals.var_t9_dn2 = assign62960_e97635_d_n2;
        locals.var_t9_dn4 = assign62960_e97635_d_n4;
        locals.var_t9_dn5 = assign62960_e97635_d_n5;
        locals.var_t9_dn6 = assign62960_e97635_d_n6;
        locals.var_t9_dn7 = assign62960_e97635_d_n7;
        locals.var_t9_dn8 = assign62960_e97635_d_n8;
        locals.var_t9_dn9 = assign62960_e97635_d_n9;
        locals.var_t9_dn10 = assign62960_e97635_d_n10;
        locals.var_t9_dn11 = assign62960_e97635_d_n11;
        locals.var_t9_dn14 = assign62960_e97635_d_n14;

        let (assign62970_e97654, assign62970_e97654_d_n0, assign62970_e97654_d_n2, assign62970_e97654_d_n4, assign62970_e97654_d_n5, assign62970_e97654_d_n6, assign62970_e97654_d_n7, assign62970_e97654_d_n8, assign62970_e97654_d_n9, assign62970_e97654_d_n10, assign62970_e97654_d_n11, assign62970_e97654_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62970_e97644: f64 = (locals.var_t7 * locals.var_t1);
        let assign62970_e97647: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign62970_e97649: f64 = (assign62970_e97647 * locals.var_t9);
        let assign62970_e97650: f64 = (assign62970_e97644 + assign62970_e97649);
        let assign62970_e97652: f64 = (assign62970_e97650 / locals.var_weff_nf);
        (assign62970_e97652, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign62970_e97647 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign62970_e97647 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign62970_e97647 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign62970_e97647 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (assign62970_e97647 * locals.var_t9_dn8)) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (((locals.var_vbse_dn9 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn9))) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign62970_e97647 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn11 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn11)) + (assign62970_e97647 * locals.var_t9_dn11)) / locals.var_weff_nf), ((((locals.var_t7_dn14 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn14)) + (assign62970_e97647 * locals.var_t9_dn14)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign62970_e97654;
        locals.var_ra_dn0 = assign62970_e97654_d_n0;
        locals.var_ra_dn2 = assign62970_e97654_d_n2;
        locals.var_ra_dn4 = assign62970_e97654_d_n4;
        locals.var_ra_dn5 = assign62970_e97654_d_n5;
        locals.var_ra_dn6 = assign62970_e97654_d_n6;
        locals.var_ra_dn7 = assign62970_e97654_d_n7;
        locals.var_ra_dn8 = assign62970_e97654_d_n8;
        locals.var_ra_dn9 = assign62970_e97654_d_n9;
        locals.var_ra_dn10 = assign62970_e97654_d_n10;
        locals.var_ra_dn11 = assign62970_e97654_d_n11;
        locals.var_ra_dn14 = assign62970_e97654_d_n14;

        let (assign62980_e97665, assign62980_e97665_d_n0, assign62980_e97665_d_n2, assign62980_e97665_d_n4, assign62980_e97665_d_n5, assign62980_e97665_d_n6, assign62980_e97665_d_n7, assign62980_e97665_d_n8, assign62980_e97665_d_n9, assign62980_e97665_d_n10, assign62980_e97665_d_n11, assign62980_e97665_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62980_e97663: f64 = (locals.var_ra * locals.var_ids0);
        (assign62980_e97663, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn11 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn11)), ((locals.var_ra_dn14 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62980_e97665;
        locals.var_t0_dn0 = assign62980_e97665_d_n0;
        locals.var_t0_dn2 = assign62980_e97665_d_n2;
        locals.var_t0_dn4 = assign62980_e97665_d_n4;
        locals.var_t0_dn5 = assign62980_e97665_d_n5;
        locals.var_t0_dn6 = assign62980_e97665_d_n6;
        locals.var_t0_dn7 = assign62980_e97665_d_n7;
        locals.var_t0_dn8 = assign62980_e97665_d_n8;
        locals.var_t0_dn9 = assign62980_e97665_d_n9;
        locals.var_t0_dn10 = assign62980_e97665_d_n10;
        locals.var_t0_dn11 = assign62980_e97665_d_n11;
        locals.var_t0_dn14 = assign62980_e97665_d_n14;

        let (assign62990_e97676, assign62990_e97676_d_n0, assign62990_e97676_d_n2, assign62990_e97676_d_n4, assign62990_e97676_d_n5, assign62990_e97676_d_n6, assign62990_e97676_d_n7, assign62990_e97676_d_n8, assign62990_e97676_d_n9, assign62990_e97676_d_n10, assign62990_e97676_d_n11, assign62990_e97676_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62990_e97674: f64 = (locals.var_vds + 1e-12);
        (assign62990_e97674, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62990_e97676;
        locals.var_t1_dn0 = assign62990_e97676_d_n0;
        locals.var_t1_dn2 = assign62990_e97676_d_n2;
        locals.var_t1_dn4 = assign62990_e97676_d_n4;
        locals.var_t1_dn5 = assign62990_e97676_d_n5;
        locals.var_t1_dn6 = assign62990_e97676_d_n6;
        locals.var_t1_dn7 = assign62990_e97676_d_n7;
        locals.var_t1_dn8 = assign62990_e97676_d_n8;
        locals.var_t1_dn9 = assign62990_e97676_d_n9;
        locals.var_t1_dn10 = assign62990_e97676_d_n10;
        locals.var_t1_dn11 = assign62990_e97676_d_n11;
        locals.var_t1_dn14 = assign62990_e97676_d_n14;

        let (assign63000_e97687, assign63000_e97687_d_n0, assign63000_e97687_d_n2, assign63000_e97687_d_n4, assign63000_e97687_d_n5, assign63000_e97687_d_n6, assign63000_e97687_d_n7, assign63000_e97687_d_n8, assign63000_e97687_d_n9, assign63000_e97687_d_n10, assign63000_e97687_d_n11, assign63000_e97687_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63000_e97685: f64 = (1.0 / locals.var_t1);
        (assign63000_e97685, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63000_e97687;
        locals.var_t2_dn0 = assign63000_e97687_d_n0;
        locals.var_t2_dn2 = assign63000_e97687_d_n2;
        locals.var_t2_dn4 = assign63000_e97687_d_n4;
        locals.var_t2_dn5 = assign63000_e97687_d_n5;
        locals.var_t2_dn6 = assign63000_e97687_d_n6;
        locals.var_t2_dn7 = assign63000_e97687_d_n7;
        locals.var_t2_dn8 = assign63000_e97687_d_n8;
        locals.var_t2_dn9 = assign63000_e97687_d_n9;
        locals.var_t2_dn10 = assign63000_e97687_d_n10;
        locals.var_t2_dn11 = assign63000_e97687_d_n11;
        locals.var_t2_dn14 = assign63000_e97687_d_n14;

        let (assign63010_e97700, assign63010_e97700_d_n0, assign63010_e97700_d_n2, assign63010_e97700_d_n4, assign63010_e97700_d_n5, assign63010_e97700_d_n6, assign63010_e97700_d_n7, assign63010_e97700_d_n8, assign63010_e97700_d_n9, assign63010_e97700_d_n10, assign63010_e97700_d_n11, assign63010_e97700_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63010_e97697: f64 = (locals.var_t0 * locals.var_t2);
        let assign63010_e97698: f64 = (1.0 + assign63010_e97697);
        (assign63010_e97698, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn11 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn11)), ((locals.var_t0_dn14 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63010_e97700;
        locals.var_t3_dn0 = assign63010_e97700_d_n0;
        locals.var_t3_dn2 = assign63010_e97700_d_n2;
        locals.var_t3_dn4 = assign63010_e97700_d_n4;
        locals.var_t3_dn5 = assign63010_e97700_d_n5;
        locals.var_t3_dn6 = assign63010_e97700_d_n6;
        locals.var_t3_dn7 = assign63010_e97700_d_n7;
        locals.var_t3_dn8 = assign63010_e97700_d_n8;
        locals.var_t3_dn9 = assign63010_e97700_d_n9;
        locals.var_t3_dn10 = assign63010_e97700_d_n10;
        locals.var_t3_dn11 = assign63010_e97700_d_n11;
        locals.var_t3_dn14 = assign63010_e97700_d_n14;

        let (assign63020_e97711, assign63020_e97711_d_n0, assign63020_e97711_d_n2, assign63020_e97711_d_n4, assign63020_e97711_d_n5, assign63020_e97711_d_n6, assign63020_e97711_d_n7, assign63020_e97711_d_n8, assign63020_e97711_d_n9, assign63020_e97711_d_n10, assign63020_e97711_d_n11, assign63020_e97711_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63020_e97709: f64 = (1.0 / locals.var_t3);
        (assign63020_e97709, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63020_e97711;
        locals.var_t4_dn0 = assign63020_e97711_d_n0;
        locals.var_t4_dn2 = assign63020_e97711_d_n2;
        locals.var_t4_dn4 = assign63020_e97711_d_n4;
        locals.var_t4_dn5 = assign63020_e97711_d_n5;
        locals.var_t4_dn6 = assign63020_e97711_d_n6;
        locals.var_t4_dn7 = assign63020_e97711_d_n7;
        locals.var_t4_dn8 = assign63020_e97711_d_n8;
        locals.var_t4_dn9 = assign63020_e97711_d_n9;
        locals.var_t4_dn10 = assign63020_e97711_d_n10;
        locals.var_t4_dn11 = assign63020_e97711_d_n11;
        locals.var_t4_dn14 = assign63020_e97711_d_n14;

        let (assign63030_e97722, assign63030_e97722_d_n0, assign63030_e97722_d_n2, assign63030_e97722_d_n4, assign63030_e97722_d_n5, assign63030_e97722_d_n6, assign63030_e97722_d_n7, assign63030_e97722_d_n8, assign63030_e97722_d_n9, assign63030_e97722_d_n10, assign63030_e97722_d_n11, assign63030_e97722_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63030_e97720: f64 = (locals.var_ids0 * locals.var_t4);
        (assign63030_e97720, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn11 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn11)), ((locals.var_ids0_dn14 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn14)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63030_e97722;
        locals.var_ids_dn0 = assign63030_e97722_d_n0;
        locals.var_ids_dn2 = assign63030_e97722_d_n2;
        locals.var_ids_dn4 = assign63030_e97722_d_n4;
        locals.var_ids_dn5 = assign63030_e97722_d_n5;
        locals.var_ids_dn6 = assign63030_e97722_d_n6;
        locals.var_ids_dn7 = assign63030_e97722_d_n7;
        locals.var_ids_dn8 = assign63030_e97722_d_n8;
        locals.var_ids_dn9 = assign63030_e97722_d_n9;
        locals.var_ids_dn10 = assign63030_e97722_d_n10;
        locals.var_ids_dn11 = assign63030_e97722_d_n11;
        locals.var_ids_dn14 = assign63030_e97722_d_n14;

        let (assign63040_e97732, assign63040_e97732_d_n0, assign63040_e97732_d_n2, assign63040_e97732_d_n4, assign63040_e97732_d_n5, assign63040_e97732_d_n6, assign63040_e97732_d_n7, assign63040_e97732_d_n8, assign63040_e97732_d_n9, assign63040_e97732_d_n10, assign63040_e97732_d_n11, assign63040_e97732_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63040_e97732;
        locals.var_ids_dn0 = assign63040_e97732_d_n0;
        locals.var_ids_dn2 = assign63040_e97732_d_n2;
        locals.var_ids_dn4 = assign63040_e97732_d_n4;
        locals.var_ids_dn5 = assign63040_e97732_d_n5;
        locals.var_ids_dn6 = assign63040_e97732_d_n6;
        locals.var_ids_dn7 = assign63040_e97732_d_n7;
        locals.var_ids_dn8 = assign63040_e97732_d_n8;
        locals.var_ids_dn9 = assign63040_e97732_d_n9;
        locals.var_ids_dn10 = assign63040_e97732_d_n10;
        locals.var_ids_dn11 = assign63040_e97732_d_n11;
        locals.var_ids_dn14 = assign63040_e97732_d_n14;

        let (assign63050_e97742, assign63050_e97742_d_n0, assign63050_e97742_d_n2, assign63050_e97742_d_n4, assign63050_e97742_d_n5, assign63050_e97742_d_n6, assign63050_e97742_d_n7, assign63050_e97742_d_n8, assign63050_e97742_d_n9, assign63050_e97742_d_n10, assign63050_e97742_d_n11, assign63050_e97742_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign63050_e97742;
        locals.var_ra_dn0 = assign63050_e97742_d_n0;
        locals.var_ra_dn2 = assign63050_e97742_d_n2;
        locals.var_ra_dn4 = assign63050_e97742_d_n4;
        locals.var_ra_dn5 = assign63050_e97742_d_n5;
        locals.var_ra_dn6 = assign63050_e97742_d_n6;
        locals.var_ra_dn7 = assign63050_e97742_d_n7;
        locals.var_ra_dn8 = assign63050_e97742_d_n8;
        locals.var_ra_dn9 = assign63050_e97742_d_n9;
        locals.var_ra_dn10 = assign63050_e97742_d_n10;
        locals.var_ra_dn11 = assign63050_e97742_d_n11;
        locals.var_ra_dn14 = assign63050_e97742_d_n14;

        let assign63060_e97745: f64 = if p.p27 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign63060_e97745;

        let (assign63070_e97756, assign63070_e97756_d_n0, assign63070_e97756_d_n2, assign63070_e97756_d_n4, assign63070_e97756_d_n5, assign63070_e97756_d_n6, assign63070_e97756_d_n7, assign63070_e97756_d_n8, assign63070_e97756_d_n9, assign63070_e97756_d_n10, assign63070_e97756_d_n11, assign63070_e97756_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63070_e97754: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign63070_e97754, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63070_e97756;
        locals.var_t1_dn0 = assign63070_e97756_d_n0;
        locals.var_t1_dn2 = assign63070_e97756_d_n2;
        locals.var_t1_dn4 = assign63070_e97756_d_n4;
        locals.var_t1_dn5 = assign63070_e97756_d_n5;
        locals.var_t1_dn6 = assign63070_e97756_d_n6;
        locals.var_t1_dn7 = assign63070_e97756_d_n7;
        locals.var_t1_dn8 = assign63070_e97756_d_n8;
        locals.var_t1_dn9 = assign63070_e97756_d_n9;
        locals.var_t1_dn10 = assign63070_e97756_d_n10;
        locals.var_t1_dn11 = assign63070_e97756_d_n11;
        locals.var_t1_dn14 = assign63070_e97756_d_n14;

        let (assign63080_e97765, assign63080_e97765_d_n0, assign63080_e97765_d_n2, assign63080_e97765_d_n4, assign63080_e97765_d_n5, assign63080_e97765_d_n6, assign63080_e97765_d_n7, assign63080_e97765_d_n8, assign63080_e97765_d_n9, assign63080_e97765_d_n10, assign63080_e97765_d_n11, assign63080_e97765_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63080_e97765;
        locals.var_t2_dn0 = assign63080_e97765_d_n0;
        locals.var_t2_dn2 = assign63080_e97765_d_n2;
        locals.var_t2_dn4 = assign63080_e97765_d_n4;
        locals.var_t2_dn5 = assign63080_e97765_d_n5;
        locals.var_t2_dn6 = assign63080_e97765_d_n6;
        locals.var_t2_dn7 = assign63080_e97765_d_n7;
        locals.var_t2_dn8 = assign63080_e97765_d_n8;
        locals.var_t2_dn9 = assign63080_e97765_d_n9;
        locals.var_t2_dn10 = assign63080_e97765_d_n10;
        locals.var_t2_dn11 = assign63080_e97765_d_n11;
        locals.var_t2_dn14 = assign63080_e97765_d_n14;

        let (assign63090_e97776, assign63090_e97776_d_n0, assign63090_e97776_d_n2, assign63090_e97776_d_n4, assign63090_e97776_d_n5, assign63090_e97776_d_n6, assign63090_e97776_d_n7, assign63090_e97776_d_n8, assign63090_e97776_d_n9, assign63090_e97776_d_n10, assign63090_e97776_d_n11, assign63090_e97776_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63090_e97774: f64 = (locals.var_lgatesm - p.p139);
        (assign63090_e97774, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63090_e97776;
        locals.var_t3_dn0 = assign63090_e97776_d_n0;
        locals.var_t3_dn2 = assign63090_e97776_d_n2;
        locals.var_t3_dn4 = assign63090_e97776_d_n4;
        locals.var_t3_dn5 = assign63090_e97776_d_n5;
        locals.var_t3_dn6 = assign63090_e97776_d_n6;
        locals.var_t3_dn7 = assign63090_e97776_d_n7;
        locals.var_t3_dn8 = assign63090_e97776_d_n8;
        locals.var_t3_dn9 = assign63090_e97776_d_n9;
        locals.var_t3_dn10 = assign63090_e97776_d_n10;
        locals.var_t3_dn11 = assign63090_e97776_d_n11;
        locals.var_t3_dn14 = assign63090_e97776_d_n14;

        let (assign63100_e97789, assign63100_e97789_d_n0, assign63100_e97789_d_n2, assign63100_e97789_d_n4, assign63100_e97789_d_n5, assign63100_e97789_d_n6, assign63100_e97789_d_n7, assign63100_e97789_d_n8, assign63100_e97789_d_n9, assign63100_e97789_d_n10, assign63100_e97789_d_n11, assign63100_e97789_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63100_e97786: f64 = (locals.var_t3 * locals.var_t3);
        let assign63100_e97787: f64 = (1.0 / assign63100_e97786);
        (assign63100_e97787, (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign63100_e97786 * assign63100_e97786))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63100_e97789;
        locals.var_t4_dn0 = assign63100_e97789_d_n0;
        locals.var_t4_dn2 = assign63100_e97789_d_n2;
        locals.var_t4_dn4 = assign63100_e97789_d_n4;
        locals.var_t4_dn5 = assign63100_e97789_d_n5;
        locals.var_t4_dn6 = assign63100_e97789_d_n6;
        locals.var_t4_dn7 = assign63100_e97789_d_n7;
        locals.var_t4_dn8 = assign63100_e97789_d_n8;
        locals.var_t4_dn9 = assign63100_e97789_d_n9;
        locals.var_t4_dn10 = assign63100_e97789_d_n10;
        locals.var_t4_dn11 = assign63100_e97789_d_n11;
        locals.var_t4_dn14 = assign63100_e97789_d_n14;

        let (assign63110_e97808, assign63110_e97808_d_n0, assign63110_e97808_d_n2, assign63110_e97808_d_n4, assign63110_e97808_d_n5, assign63110_e97808_d_n6, assign63110_e97808_d_n7, assign63110_e97808_d_n8, assign63110_e97808_d_n9, assign63110_e97808_d_n10, assign63110_e97808_d_n11, assign63110_e97808_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63110_e97799: f64 = (p.p137 - locals.var_pb20b);
        let assign63110_e97800: f64 = (2.0 * assign63110_e97799);
        let assign63110_e97802: f64 = (assign63110_e97800 * locals.var_t1);
        let assign63110_e97804: f64 = (assign63110_e97802 * locals.var_t2);
        let assign63110_e97806: f64 = (assign63110_e97804 * locals.var_t4);
        (assign63110_e97806, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn0)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn0)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn2)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn2)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn4)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn4)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn5)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn5)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn6)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn6)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn7)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn7)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn8)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn8)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn9)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn9)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn10)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn10)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn11)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn11)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn14)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn14)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63110_e97808;
        locals.var_t5_dn0 = assign63110_e97808_d_n0;
        locals.var_t5_dn2 = assign63110_e97808_d_n2;
        locals.var_t5_dn4 = assign63110_e97808_d_n4;
        locals.var_t5_dn5 = assign63110_e97808_d_n5;
        locals.var_t5_dn6 = assign63110_e97808_d_n6;
        locals.var_t5_dn7 = assign63110_e97808_d_n7;
        locals.var_t5_dn8 = assign63110_e97808_d_n8;
        locals.var_t5_dn9 = assign63110_e97808_d_n9;
        locals.var_t5_dn10 = assign63110_e97808_d_n10;
        locals.var_t5_dn11 = assign63110_e97808_d_n11;
        locals.var_t5_dn14 = assign63110_e97808_d_n14;

    }

    pub(super) fn stamp_transient_block_223(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign63120_e97819, assign63120_e97819_d_n0, assign63120_e97819_d_n2, assign63120_e97819_d_n4, assign63120_e97819_d_n5, assign63120_e97819_d_n6, assign63120_e97819_d_n7, assign63120_e97819_d_n8, assign63120_e97819_d_n9, assign63120_e97819_d_n10, assign63120_e97819_d_n11, assign63120_e97819_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63120_e97817: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign63120_e97817, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign63120_e97819;
        locals.var_dvth0_dn0 = assign63120_e97819_d_n0;
        locals.var_dvth0_dn2 = assign63120_e97819_d_n2;
        locals.var_dvth0_dn4 = assign63120_e97819_d_n4;
        locals.var_dvth0_dn5 = assign63120_e97819_d_n5;
        locals.var_dvth0_dn6 = assign63120_e97819_d_n6;
        locals.var_dvth0_dn7 = assign63120_e97819_d_n7;
        locals.var_dvth0_dn8 = assign63120_e97819_d_n8;
        locals.var_dvth0_dn9 = assign63120_e97819_d_n9;
        locals.var_dvth0_dn10 = assign63120_e97819_d_n10;
        locals.var_dvth0_dn11 = assign63120_e97819_d_n11;
        locals.var_dvth0_dn14 = assign63120_e97819_d_n14;

        let (assign63130_e97832, assign63130_e97832_d_n0, assign63130_e97832_d_n2, assign63130_e97832_d_n4, assign63130_e97832_d_n5, assign63130_e97832_d_n6, assign63130_e97832_d_n7, assign63130_e97832_d_n8, assign63130_e97832_d_n9, assign63130_e97832_d_n10, assign63130_e97832_d_n11, assign63130_e97832_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63130_e97828: f64 = (locals.var_t5 * 0.5);
        let assign63130_e97830: f64 = (assign63130_e97828 / locals.var_sqrt_pbsum);
        (assign63130_e97830, ((((locals.var_t5_dn0 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn2 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn4 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn5 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn6 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn7 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn8 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn9 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn10 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn11 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn14 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63130_e97832;
        locals.var_t6_dn0 = assign63130_e97832_d_n0;
        locals.var_t6_dn2 = assign63130_e97832_d_n2;
        locals.var_t6_dn4 = assign63130_e97832_d_n4;
        locals.var_t6_dn5 = assign63130_e97832_d_n5;
        locals.var_t6_dn6 = assign63130_e97832_d_n6;
        locals.var_t6_dn7 = assign63130_e97832_d_n7;
        locals.var_t6_dn8 = assign63130_e97832_d_n8;
        locals.var_t6_dn9 = assign63130_e97832_d_n9;
        locals.var_t6_dn10 = assign63130_e97832_d_n10;
        locals.var_t6_dn11 = assign63130_e97832_d_n11;
        locals.var_t6_dn14 = assign63130_e97832_d_n14;

        let (assign63140_e97853, assign63140_e97853_d_n0, assign63140_e97853_d_n2, assign63140_e97853_d_n4, assign63140_e97853_d_n5, assign63140_e97853_d_n6, assign63140_e97853_d_n7, assign63140_e97853_d_n8, assign63140_e97853_d_n9, assign63140_e97853_d_n10, assign63140_e97853_d_n11, assign63140_e97853_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63140_e97842: f64 = (p.p137 - locals.var_pb20b);
        let assign63140_e97843: f64 = (2.0 * assign63140_e97842);
        let assign63140_e97845: f64 = (assign63140_e97843 * 1.034943e-10);
        let assign63140_e97847: f64 = (assign63140_e97845 * locals.var_t2);
        let assign63140_e97849: f64 = (assign63140_e97847 * locals.var_t4);
        let assign63140_e97851: f64 = (assign63140_e97849 * locals.var_sqrt_pbsum);
        (assign63140_e97851, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn0)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn2)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn4)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn5)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn6)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn7)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn8)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn9)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn10)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn11)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn14)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign63140_e97853;
        locals.var_t7_dn0 = assign63140_e97853_d_n0;
        locals.var_t7_dn2 = assign63140_e97853_d_n2;
        locals.var_t7_dn4 = assign63140_e97853_d_n4;
        locals.var_t7_dn5 = assign63140_e97853_d_n5;
        locals.var_t7_dn6 = assign63140_e97853_d_n6;
        locals.var_t7_dn7 = assign63140_e97853_d_n7;
        locals.var_t7_dn8 = assign63140_e97853_d_n8;
        locals.var_t7_dn9 = assign63140_e97853_d_n9;
        locals.var_t7_dn10 = assign63140_e97853_d_n10;
        locals.var_t7_dn11 = assign63140_e97853_d_n11;
        locals.var_t7_dn14 = assign63140_e97853_d_n14;

        let (assign63150_e97871, assign63150_e97871_d_n0, assign63150_e97871_d_n2, assign63150_e97871_d_n4, assign63150_e97871_d_n5, assign63150_e97871_d_n6, assign63150_e97871_d_n7, assign63150_e97871_d_n8, assign63150_e97871_d_n9, assign63150_e97871_d_n10, assign63150_e97871_d_n11, assign63150_e97871_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63150_e97861: f64 = (-2.0);
        let assign63150_e97863: f64 = (assign63150_e97861 * locals.var_t1);
        let assign63150_e97865: f64 = (assign63150_e97863 * locals.var_t2);
        let assign63150_e97867: f64 = (assign63150_e97865 * locals.var_t4);
        let assign63150_e97869: f64 = (assign63150_e97867 * locals.var_sqrt_pbsum);
        (assign63150_e97869, (((((((assign63150_e97861 * locals.var_t1_dn0) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn0)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn0)), (((((((assign63150_e97861 * locals.var_t1_dn2) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn2)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn2)), (((((((assign63150_e97861 * locals.var_t1_dn4) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn4)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn4)), (((((((assign63150_e97861 * locals.var_t1_dn5) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn5)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn5)), (((((((assign63150_e97861 * locals.var_t1_dn6) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn6)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn6)), (((((((assign63150_e97861 * locals.var_t1_dn7) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn7)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn7)), (((((((assign63150_e97861 * locals.var_t1_dn8) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn8)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn8)), (((((((assign63150_e97861 * locals.var_t1_dn9) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn9)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn9)), (((((((assign63150_e97861 * locals.var_t1_dn10) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn10)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn10)), (((((((assign63150_e97861 * locals.var_t1_dn11) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn11)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn11)), (((((((assign63150_e97861 * locals.var_t1_dn14) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn14)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign63150_e97871;
        locals.var_t8_dn0 = assign63150_e97871_d_n0;
        locals.var_t8_dn2 = assign63150_e97871_d_n2;
        locals.var_t8_dn4 = assign63150_e97871_d_n4;
        locals.var_t8_dn5 = assign63150_e97871_d_n5;
        locals.var_t8_dn6 = assign63150_e97871_d_n6;
        locals.var_t8_dn7 = assign63150_e97871_d_n7;
        locals.var_t8_dn8 = assign63150_e97871_d_n8;
        locals.var_t8_dn9 = assign63150_e97871_d_n9;
        locals.var_t8_dn10 = assign63150_e97871_d_n10;
        locals.var_t8_dn11 = assign63150_e97871_d_n11;
        locals.var_t8_dn14 = assign63150_e97871_d_n14;

        let (assign63160_e97880, assign63160_e97880_d_n0, assign63160_e97880_d_n2, assign63160_e97880_d_n4, assign63160_e97880_d_n5, assign63160_e97880_d_n6, assign63160_e97880_d_n7, assign63160_e97880_d_n8, assign63160_e97880_d_n9, assign63160_e97880_d_n10, assign63160_e97880_d_n11, assign63160_e97880_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_uc_scsti1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63160_e97880;
        locals.var_t4_dn0 = assign63160_e97880_d_n0;
        locals.var_t4_dn2 = assign63160_e97880_d_n2;
        locals.var_t4_dn4 = assign63160_e97880_d_n4;
        locals.var_t4_dn5 = assign63160_e97880_d_n5;
        locals.var_t4_dn6 = assign63160_e97880_d_n6;
        locals.var_t4_dn7 = assign63160_e97880_d_n7;
        locals.var_t4_dn8 = assign63160_e97880_d_n8;
        locals.var_t4_dn9 = assign63160_e97880_d_n9;
        locals.var_t4_dn10 = assign63160_e97880_d_n10;
        locals.var_t4_dn11 = assign63160_e97880_d_n11;
        locals.var_t4_dn14 = assign63160_e97880_d_n14;

        let (assign63170_e97889, assign63170_e97889_d_n0, assign63170_e97889_d_n2, assign63170_e97889_d_n4, assign63170_e97889_d_n5, assign63170_e97889_d_n6, assign63170_e97889_d_n7, assign63170_e97889_d_n8, assign63170_e97889_d_n9, assign63170_e97889_d_n10, assign63170_e97889_d_n11, assign63170_e97889_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_uc_scsti2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63170_e97889;
        locals.var_t6_dn0 = assign63170_e97889_d_n0;
        locals.var_t6_dn2 = assign63170_e97889_d_n2;
        locals.var_t6_dn4 = assign63170_e97889_d_n4;
        locals.var_t6_dn5 = assign63170_e97889_d_n5;
        locals.var_t6_dn6 = assign63170_e97889_d_n6;
        locals.var_t6_dn7 = assign63170_e97889_d_n7;
        locals.var_t6_dn8 = assign63170_e97889_d_n8;
        locals.var_t6_dn9 = assign63170_e97889_d_n9;
        locals.var_t6_dn10 = assign63170_e97889_d_n10;
        locals.var_t6_dn11 = assign63170_e97889_d_n11;
        locals.var_t6_dn14 = assign63170_e97889_d_n14;

        let (assign63180_e97902, assign63180_e97902_d_n0, assign63180_e97902_d_n2, assign63180_e97902_d_n4, assign63180_e97902_d_n5, assign63180_e97902_d_n6, assign63180_e97902_d_n7, assign63180_e97902_d_n8, assign63180_e97902_d_n9, assign63180_e97902_d_n10, assign63180_e97902_d_n11, assign63180_e97902_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63180_e97899: f64 = (locals.var_t6 * locals.var_vdsz__blk441);
        let assign63180_e97900: f64 = (locals.var_t4 + assign63180_e97899);
        (assign63180_e97900, (locals.var_t4_dn0 + ((locals.var_t6_dn0 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn0))), (locals.var_t4_dn2 + ((locals.var_t6_dn2 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn2))), (locals.var_t4_dn4 + ((locals.var_t6_dn4 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn4))), (locals.var_t4_dn5 + ((locals.var_t6_dn5 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn5))), (locals.var_t4_dn6 + ((locals.var_t6_dn6 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn6))), (locals.var_t4_dn7 + ((locals.var_t6_dn7 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn7))), (locals.var_t4_dn8 + ((locals.var_t6_dn8 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn8))), (locals.var_t4_dn9 + ((locals.var_t6_dn9 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn9))), (locals.var_t4_dn10 + ((locals.var_t6_dn10 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn10))), (locals.var_t4_dn11 + ((locals.var_t6_dn11 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn11))), (locals.var_t4_dn14 + ((locals.var_t6_dn14 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63180_e97902;
        locals.var_t1_dn0 = assign63180_e97902_d_n0;
        locals.var_t1_dn2 = assign63180_e97902_d_n2;
        locals.var_t1_dn4 = assign63180_e97902_d_n4;
        locals.var_t1_dn5 = assign63180_e97902_d_n5;
        locals.var_t1_dn6 = assign63180_e97902_d_n6;
        locals.var_t1_dn7 = assign63180_e97902_d_n7;
        locals.var_t1_dn8 = assign63180_e97902_d_n8;
        locals.var_t1_dn9 = assign63180_e97902_d_n9;
        locals.var_t1_dn10 = assign63180_e97902_d_n10;
        locals.var_t1_dn11 = assign63180_e97902_d_n11;
        locals.var_t1_dn14 = assign63180_e97902_d_n14;

        let (assign63190_e97913, assign63190_e97913_d_n0, assign63190_e97913_d_n2, assign63190_e97913_d_n4, assign63190_e97913_d_n5, assign63190_e97913_d_n6, assign63190_e97913_d_n7, assign63190_e97913_d_n8, assign63190_e97913_d_n9, assign63190_e97913_d_n10, assign63190_e97913_d_n11, assign63190_e97913_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63190_e97911: f64 = (locals.var_dvth0 * locals.var_t1);
        (assign63190_e97911, ((locals.var_dvth0_dn0 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn2)), ((locals.var_dvth0_dn4 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn4)), ((locals.var_dvth0_dn5 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn5)), ((locals.var_dvth0_dn6 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn7)), ((locals.var_dvth0_dn8 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn8)), ((locals.var_dvth0_dn9 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn9)), ((locals.var_dvth0_dn10 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn11)), ((locals.var_dvth0_dn14 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn14)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn4, locals.var_dvthscsti_dn5, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn8, locals.var_dvthscsti_dn9, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn14,)
    }
};
        locals.var_dvthscsti = assign63190_e97913;
        locals.var_dvthscsti_dn0 = assign63190_e97913_d_n0;
        locals.var_dvthscsti_dn2 = assign63190_e97913_d_n2;
        locals.var_dvthscsti_dn4 = assign63190_e97913_d_n4;
        locals.var_dvthscsti_dn5 = assign63190_e97913_d_n5;
        locals.var_dvthscsti_dn6 = assign63190_e97913_d_n6;
        locals.var_dvthscsti_dn7 = assign63190_e97913_d_n7;
        locals.var_dvthscsti_dn8 = assign63190_e97913_d_n8;
        locals.var_dvthscsti_dn9 = assign63190_e97913_d_n9;
        locals.var_dvthscsti_dn10 = assign63190_e97913_d_n10;
        locals.var_dvthscsti_dn11 = assign63190_e97913_d_n11;
        locals.var_dvthscsti_dn14 = assign63190_e97913_d_n14;

        let (assign63200_e97926, assign63200_e97926_d_n0, assign63200_e97926_d_n2, assign63200_e97926_d_n4, assign63200_e97926_d_n5, assign63200_e97926_d_n6, assign63200_e97926_d_n7, assign63200_e97926_d_n8, assign63200_e97926_d_n9, assign63200_e97926_d_n10, assign63200_e97926_d_n11, assign63200_e97926_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63200_e97923: f64 = (p.p213 * locals.var_vds);
        let assign63200_e97924: f64 = (locals.var_uc_vthsti - assign63200_e97923);
        (assign63200_e97924, (-(p.p213 * locals.var_vds_dn0)), (-(p.p213 * locals.var_vds_dn2)), (-(p.p213 * locals.var_vds_dn4)), (-(p.p213 * locals.var_vds_dn5)), (-(p.p213 * locals.var_vds_dn6)), (-(p.p213 * locals.var_vds_dn7)), (-(p.p213 * locals.var_vds_dn8)), (-(p.p213 * locals.var_vds_dn9)), (-(p.p213 * locals.var_vds_dn10)), (-(p.p213 * locals.var_vds_dn11)), (-(p.p213 * locals.var_vds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63200_e97926;
        locals.var_t1_dn0 = assign63200_e97926_d_n0;
        locals.var_t1_dn2 = assign63200_e97926_d_n2;
        locals.var_t1_dn4 = assign63200_e97926_d_n4;
        locals.var_t1_dn5 = assign63200_e97926_d_n5;
        locals.var_t1_dn6 = assign63200_e97926_d_n6;
        locals.var_t1_dn7 = assign63200_e97926_d_n7;
        locals.var_t1_dn8 = assign63200_e97926_d_n8;
        locals.var_t1_dn9 = assign63200_e97926_d_n9;
        locals.var_t1_dn10 = assign63200_e97926_d_n10;
        locals.var_t1_dn11 = assign63200_e97926_d_n11;
        locals.var_t1_dn14 = assign63200_e97926_d_n14;

        let (assign63210_e97941, assign63210_e97941_d_n0, assign63210_e97941_d_n2, assign63210_e97941_d_n4, assign63210_e97941_d_n5, assign63210_e97941_d_n6, assign63210_e97941_d_n7, assign63210_e97941_d_n8, assign63210_e97941_d_n9, assign63210_e97941_d_n10, assign63210_e97941_d_n11, assign63210_e97941_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63210_e97935: f64 = (locals.var_vgsz__blk442 - locals.var_vfb);
        let assign63210_e97937: f64 = (assign63210_e97935 + locals.var_t1);
        let assign63210_e97939: f64 = (assign63210_e97937 + locals.var_dvthscsti);
        (assign63210_e97939, ((locals.var_vgsz__blk442_dn0 + locals.var_t1_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz__blk442_dn2 + locals.var_t1_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz__blk442_dn4 + locals.var_t1_dn4) + locals.var_dvthscsti_dn4), ((locals.var_vgsz__blk442_dn5 + locals.var_t1_dn5) + locals.var_dvthscsti_dn5), ((locals.var_vgsz__blk442_dn6 + locals.var_t1_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz__blk442_dn7 + locals.var_t1_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz__blk442_dn8 + locals.var_t1_dn8) + locals.var_dvthscsti_dn8), ((locals.var_vgsz__blk442_dn9 + locals.var_t1_dn9) + locals.var_dvthscsti_dn9), ((locals.var_vgsz__blk442_dn10 + locals.var_t1_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz__blk442_dn11 + locals.var_t1_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz__blk442_dn14 + locals.var_t1_dn14) + locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn4, locals.var_vgssti_dn5, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn8, locals.var_vgssti_dn9, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn14,)
    }
};
        locals.var_vgssti = assign63210_e97941;
        locals.var_vgssti_dn0 = assign63210_e97941_d_n0;
        locals.var_vgssti_dn2 = assign63210_e97941_d_n2;
        locals.var_vgssti_dn4 = assign63210_e97941_d_n4;
        locals.var_vgssti_dn5 = assign63210_e97941_d_n5;
        locals.var_vgssti_dn6 = assign63210_e97941_d_n6;
        locals.var_vgssti_dn7 = assign63210_e97941_d_n7;
        locals.var_vgssti_dn8 = assign63210_e97941_d_n8;
        locals.var_vgssti_dn9 = assign63210_e97941_d_n9;
        locals.var_vgssti_dn10 = assign63210_e97941_d_n10;
        locals.var_vgssti_dn11 = assign63210_e97941_d_n11;
        locals.var_vgssti_dn14 = assign63210_e97941_d_n14;

        let (assign63220_e97954, assign63220_e97954_d_n0, assign63220_e97954_d_n2, assign63220_e97954_d_n4, assign63220_e97954_d_n5, assign63220_e97954_d_n6, assign63220_e97954_d_n7, assign63220_e97954_d_n8, assign63220_e97954_d_n9, assign63220_e97954_d_n10, assign63220_e97954_d_n11, assign63220_e97954_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63220_e97950: f64 = (locals.var_costi0_p2 * locals.var_cox_inv);
        let assign63220_e97952: f64 = (assign63220_e97950 * locals.var_cox_inv);
        (assign63220_e97952, ((((locals.var_costi0_p2_dn0 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn0)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn2)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn2)), ((((locals.var_costi0_p2_dn4 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn4)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn4)), ((((locals.var_costi0_p2_dn5 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn5)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn5)), ((((locals.var_costi0_p2_dn6 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn6)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn7)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn7)), ((((locals.var_costi0_p2_dn8 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn8)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn8)), ((((locals.var_costi0_p2_dn9 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn9)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn9)), ((((locals.var_costi0_p2_dn10 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn10)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn11)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn11)), ((((locals.var_costi0_p2_dn14 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn14)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn4, locals.var_costi3_dn5, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn8, locals.var_costi3_dn9, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn14,)
    }
};
        locals.var_costi3 = assign63220_e97954;
        locals.var_costi3_dn0 = assign63220_e97954_d_n0;
        locals.var_costi3_dn2 = assign63220_e97954_d_n2;
        locals.var_costi3_dn4 = assign63220_e97954_d_n4;
        locals.var_costi3_dn5 = assign63220_e97954_d_n5;
        locals.var_costi3_dn6 = assign63220_e97954_d_n6;
        locals.var_costi3_dn7 = assign63220_e97954_d_n7;
        locals.var_costi3_dn8 = assign63220_e97954_d_n8;
        locals.var_costi3_dn9 = assign63220_e97954_d_n9;
        locals.var_costi3_dn10 = assign63220_e97954_d_n10;
        locals.var_costi3_dn11 = assign63220_e97954_d_n11;
        locals.var_costi3_dn14 = assign63220_e97954_d_n14;

        let (assign63230_e97967, assign63230_e97967_d_n0, assign63230_e97967_d_n2, assign63230_e97967_d_n4, assign63230_e97967_d_n5, assign63230_e97967_d_n6, assign63230_e97967_d_n7, assign63230_e97967_d_n8, assign63230_e97967_d_n9, assign63230_e97967_d_n10, assign63230_e97967_d_n11, assign63230_e97967_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63230_e97963: f64 = (locals.var_costi3 * locals.var_beta);
        let assign63230_e97965: f64 = (assign63230_e97963 * 0.5);
        (assign63230_e97965, (((locals.var_costi3_dn0 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn0)) * 0.5), (((locals.var_costi3_dn2 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn2)) * 0.5), (((locals.var_costi3_dn4 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn4)) * 0.5), (((locals.var_costi3_dn5 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn5)) * 0.5), (((locals.var_costi3_dn6 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn6)) * 0.5), (((locals.var_costi3_dn7 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn7)) * 0.5), (((locals.var_costi3_dn8 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn8)) * 0.5), (((locals.var_costi3_dn9 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn9)) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), (((locals.var_costi3_dn11 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn11)) * 0.5), (((locals.var_costi3_dn14 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn14)) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn4, locals.var_costi4_dn5, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn8, locals.var_costi4_dn9, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn14,)
    }
};
        locals.var_costi4 = assign63230_e97967;
        locals.var_costi4_dn0 = assign63230_e97967_d_n0;
        locals.var_costi4_dn2 = assign63230_e97967_d_n2;
        locals.var_costi4_dn4 = assign63230_e97967_d_n4;
        locals.var_costi4_dn5 = assign63230_e97967_d_n5;
        locals.var_costi4_dn6 = assign63230_e97967_d_n6;
        locals.var_costi4_dn7 = assign63230_e97967_d_n7;
        locals.var_costi4_dn8 = assign63230_e97967_d_n8;
        locals.var_costi4_dn9 = assign63230_e97967_d_n9;
        locals.var_costi4_dn10 = assign63230_e97967_d_n10;
        locals.var_costi4_dn11 = assign63230_e97967_d_n11;
        locals.var_costi4_dn14 = assign63230_e97967_d_n14;

        let (assign63240_e97980, assign63240_e97980_d_n0, assign63240_e97980_d_n2, assign63240_e97980_d_n4, assign63240_e97980_d_n5, assign63240_e97980_d_n6, assign63240_e97980_d_n7, assign63240_e97980_d_n8, assign63240_e97980_d_n9, assign63240_e97980_d_n10, assign63240_e97980_d_n11, assign63240_e97980_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63240_e97976: f64 = (locals.var_costi4 * locals.var_beta);
        let assign63240_e97978: f64 = (assign63240_e97976 * 2.0);
        (assign63240_e97978, (((locals.var_costi4_dn0 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn0)) * 2.0), (((locals.var_costi4_dn2 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn2)) * 2.0), (((locals.var_costi4_dn4 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn4)) * 2.0), (((locals.var_costi4_dn5 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn5)) * 2.0), (((locals.var_costi4_dn6 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn6)) * 2.0), (((locals.var_costi4_dn7 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn7)) * 2.0), (((locals.var_costi4_dn8 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn8)) * 2.0), (((locals.var_costi4_dn9 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn9)) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), (((locals.var_costi4_dn11 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn11)) * 2.0), (((locals.var_costi4_dn14 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn14)) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn4, locals.var_costi5_dn5, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn8, locals.var_costi5_dn9, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn14,)
    }
};
        locals.var_costi5 = assign63240_e97980;
        locals.var_costi5_dn0 = assign63240_e97980_d_n0;
        locals.var_costi5_dn2 = assign63240_e97980_d_n2;
        locals.var_costi5_dn4 = assign63240_e97980_d_n4;
        locals.var_costi5_dn5 = assign63240_e97980_d_n5;
        locals.var_costi5_dn6 = assign63240_e97980_d_n6;
        locals.var_costi5_dn7 = assign63240_e97980_d_n7;
        locals.var_costi5_dn8 = assign63240_e97980_d_n8;
        locals.var_costi5_dn9 = assign63240_e97980_d_n9;
        locals.var_costi5_dn10 = assign63240_e97980_d_n10;
        locals.var_costi5_dn11 = assign63240_e97980_d_n11;
        locals.var_costi5_dn14 = assign63240_e97980_d_n14;

        let (assign63250_e97991, assign63250_e97991_d_n0, assign63250_e97991_d_n2, assign63250_e97991_d_n4, assign63250_e97991_d_n5, assign63250_e97991_d_n6, assign63250_e97991_d_n7, assign63250_e97991_d_n8, assign63250_e97991_d_n9, assign63250_e97991_d_n10, assign63250_e97991_d_n11, assign63250_e97991_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63250_e97989: f64 = (locals.var_beta * 0.25);
        (assign63250_e97989, (locals.var_beta_dn0 * 0.25), (locals.var_beta_dn2 * 0.25), (locals.var_beta_dn4 * 0.25), (locals.var_beta_dn5 * 0.25), (locals.var_beta_dn6 * 0.25), (locals.var_beta_dn7 * 0.25), (locals.var_beta_dn8 * 0.25), (locals.var_beta_dn9 * 0.25), (locals.var_beta_dn10 * 0.25), (locals.var_beta_dn11 * 0.25), (locals.var_beta_dn14 * 0.25),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign63250_e97991;
        locals.var_t11_dn0 = assign63250_e97991_d_n0;
        locals.var_t11_dn2 = assign63250_e97991_d_n2;
        locals.var_t11_dn4 = assign63250_e97991_d_n4;
        locals.var_t11_dn5 = assign63250_e97991_d_n5;
        locals.var_t11_dn6 = assign63250_e97991_d_n6;
        locals.var_t11_dn7 = assign63250_e97991_d_n7;
        locals.var_t11_dn8 = assign63250_e97991_d_n8;
        locals.var_t11_dn9 = assign63250_e97991_d_n9;
        locals.var_t11_dn10 = assign63250_e97991_d_n10;
        locals.var_t11_dn11 = assign63250_e97991_d_n11;
        locals.var_t11_dn14 = assign63250_e97991_d_n14;

        let (assign63260_e98012, assign63260_e98012_d_n0, assign63260_e98012_d_n2, assign63260_e98012_d_n4, assign63260_e98012_d_n5, assign63260_e98012_d_n6, assign63260_e98012_d_n7, assign63260_e98012_d_n8, assign63260_e98012_d_n9, assign63260_e98012_d_n10, assign63260_e98012_d_n11, assign63260_e98012_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63260_e98001: f64 = (locals.var_costi3 * locals.var_t11);
        let assign63260_e98002: f64 = (locals.var_beta_inv - assign63260_e98001);
        let assign63260_e98004: f64 = (assign63260_e98002 + locals.var_vfb);
        let assign63260_e98006: f64 = (assign63260_e98004 - locals.var_uc_vthsti);
        let assign63260_e98008: f64 = (assign63260_e98006 - locals.var_dvthscsti);
        let assign63260_e98010: f64 = (assign63260_e98008 + 1e-25);
        (assign63260_e98010, ((locals.var_beta_inv_dn0 - ((locals.var_costi3_dn0 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn0))) - locals.var_dvthscsti_dn0), ((locals.var_beta_inv_dn2 - ((locals.var_costi3_dn2 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn2))) - locals.var_dvthscsti_dn2), ((locals.var_beta_inv_dn4 - ((locals.var_costi3_dn4 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn4))) - locals.var_dvthscsti_dn4), ((locals.var_beta_inv_dn5 - ((locals.var_costi3_dn5 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn5))) - locals.var_dvthscsti_dn5), ((locals.var_beta_inv_dn6 - ((locals.var_costi3_dn6 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn6))) - locals.var_dvthscsti_dn6), ((locals.var_beta_inv_dn7 - ((locals.var_costi3_dn7 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn7))) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn8 - ((locals.var_costi3_dn8 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn8))) - locals.var_dvthscsti_dn8), ((locals.var_beta_inv_dn9 - ((locals.var_costi3_dn9 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn9))) - locals.var_dvthscsti_dn9), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn10))) - locals.var_dvthscsti_dn10), ((locals.var_beta_inv_dn11 - ((locals.var_costi3_dn11 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn11))) - locals.var_dvthscsti_dn11), ((locals.var_beta_inv_dn14 - ((locals.var_costi3_dn14 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn14))) - locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign63260_e98012;
        locals.var_t10_dn0 = assign63260_e98012_d_n0;
        locals.var_t10_dn2 = assign63260_e98012_d_n2;
        locals.var_t10_dn4 = assign63260_e98012_d_n4;
        locals.var_t10_dn5 = assign63260_e98012_d_n5;
        locals.var_t10_dn6 = assign63260_e98012_d_n6;
        locals.var_t10_dn7 = assign63260_e98012_d_n7;
        locals.var_t10_dn8 = assign63260_e98012_d_n8;
        locals.var_t10_dn9 = assign63260_e98012_d_n9;
        locals.var_t10_dn10 = assign63260_e98012_d_n10;
        locals.var_t10_dn11 = assign63260_e98012_d_n11;
        locals.var_t10_dn14 = assign63260_e98012_d_n14;

        let (assign63270_e98025, assign63270_e98025_d_n0, assign63270_e98025_d_n2, assign63270_e98025_d_n4, assign63270_e98025_d_n5, assign63270_e98025_d_n6, assign63270_e98025_d_n7, assign63270_e98025_d_n8, assign63270_e98025_d_n9, assign63270_e98025_d_n10, assign63270_e98025_d_n11, assign63270_e98025_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63270_e98021: f64 = (locals.var_vgsz__blk442 - locals.var_t10);
        let assign63270_e98023: f64 = (assign63270_e98021 - 0.005);
        (assign63270_e98023, (locals.var_vgsz__blk442_dn0 - locals.var_t10_dn0), (locals.var_vgsz__blk442_dn2 - locals.var_t10_dn2), (locals.var_vgsz__blk442_dn4 - locals.var_t10_dn4), (locals.var_vgsz__blk442_dn5 - locals.var_t10_dn5), (locals.var_vgsz__blk442_dn6 - locals.var_t10_dn6), (locals.var_vgsz__blk442_dn7 - locals.var_t10_dn7), (locals.var_vgsz__blk442_dn8 - locals.var_t10_dn8), (locals.var_vgsz__blk442_dn9 - locals.var_t10_dn9), (locals.var_vgsz__blk442_dn10 - locals.var_t10_dn10), (locals.var_vgsz__blk442_dn11 - locals.var_t10_dn11), (locals.var_vgsz__blk442_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63270_e98025;
        locals.var_t1_dn0 = assign63270_e98025_d_n0;
        locals.var_t1_dn2 = assign63270_e98025_d_n2;
        locals.var_t1_dn4 = assign63270_e98025_d_n4;
        locals.var_t1_dn5 = assign63270_e98025_d_n5;
        locals.var_t1_dn6 = assign63270_e98025_d_n6;
        locals.var_t1_dn7 = assign63270_e98025_d_n7;
        locals.var_t1_dn8 = assign63270_e98025_d_n8;
        locals.var_t1_dn9 = assign63270_e98025_d_n9;
        locals.var_t1_dn10 = assign63270_e98025_d_n10;
        locals.var_t1_dn11 = assign63270_e98025_d_n11;
        locals.var_t1_dn14 = assign63270_e98025_d_n14;

        let (assign63280_e98040, assign63280_e98040_d_n0, assign63280_e98040_d_n2, assign63280_e98040_d_n4, assign63280_e98040_d_n5, assign63280_e98040_d_n6, assign63280_e98040_d_n7, assign63280_e98040_d_n8, assign63280_e98040_d_n9, assign63280_e98040_d_n10, assign63280_e98040_d_n11, assign63280_e98040_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let (assign63280_e98038,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign63280_e98037: f64 = (-1.0);
                (assign63280_e98037,)
            }
        };
        (assign63280_e98038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63280_e98040;
        locals.var_t0_dn0 = assign63280_e98040_d_n0;
        locals.var_t0_dn2 = assign63280_e98040_d_n2;
        locals.var_t0_dn4 = assign63280_e98040_d_n4;
        locals.var_t0_dn5 = assign63280_e98040_d_n5;
        locals.var_t0_dn6 = assign63280_e98040_d_n6;
        locals.var_t0_dn7 = assign63280_e98040_d_n7;
        locals.var_t0_dn8 = assign63280_e98040_d_n8;
        locals.var_t0_dn9 = assign63280_e98040_d_n9;
        locals.var_t0_dn10 = assign63280_e98040_d_n10;
        locals.var_t0_dn11 = assign63280_e98040_d_n11;
        locals.var_t0_dn14 = assign63280_e98040_d_n14;

        let (assign63290_e98060, assign63290_e98060_d_n0, assign63290_e98060_d_n2, assign63290_e98060_d_n4, assign63290_e98060_d_n5, assign63290_e98060_d_n6, assign63290_e98060_d_n7, assign63290_e98060_d_n8, assign63290_e98060_d_n9, assign63290_e98060_d_n10, assign63290_e98060_d_n11, assign63290_e98060_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63290_e98049: f64 = (locals.var_t1 * locals.var_t1);
        let assign63290_e98052: f64 = (locals.var_t0 * 4.0);
        let assign63290_e98054: f64 = (assign63290_e98052 * locals.var_t10);
        let assign63290_e98056: f64 = (assign63290_e98054 * 0.005);
        let assign63290_e98057: f64 = (assign63290_e98049 + assign63290_e98056);
        let assign63290_e98058: f64 = (assign63290_e98057).sqrt();
        (assign63290_e98058, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn7)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((locals.var_t0_dn9 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn9)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn11)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((locals.var_t0_dn14 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn14)) * 0.005)) / (2.0 * assign63290_e98058)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63290_e98060;
        locals.var_t2_dn0 = assign63290_e98060_d_n0;
        locals.var_t2_dn2 = assign63290_e98060_d_n2;
        locals.var_t2_dn4 = assign63290_e98060_d_n4;
        locals.var_t2_dn5 = assign63290_e98060_d_n5;
        locals.var_t2_dn6 = assign63290_e98060_d_n6;
        locals.var_t2_dn7 = assign63290_e98060_d_n7;
        locals.var_t2_dn8 = assign63290_e98060_d_n8;
        locals.var_t2_dn9 = assign63290_e98060_d_n9;
        locals.var_t2_dn10 = assign63290_e98060_d_n10;
        locals.var_t2_dn11 = assign63290_e98060_d_n11;
        locals.var_t2_dn14 = assign63290_e98060_d_n14;

        let (assign63300_e98083, assign63300_e98083_d_n0, assign63300_e98083_d_n2, assign63300_e98083_d_n4, assign63300_e98083_d_n5, assign63300_e98083_d_n6, assign63300_e98083_d_n7, assign63300_e98083_d_n8, assign63300_e98083_d_n9, assign63300_e98083_d_n10, assign63300_e98083_d_n11, assign63300_e98083_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63300_e98071: f64 = (locals.var_t1 + locals.var_t2);
        let assign63300_e98072: f64 = (0.5 * assign63300_e98071);
        let assign63300_e98073: f64 = (locals.var_t10 + assign63300_e98072);
        let assign63300_e98075: f64 = (assign63300_e98073 - locals.var_vfb);
        let assign63300_e98077: f64 = (assign63300_e98075 + locals.var_uc_vthsti);
        let assign63300_e98079: f64 = (assign63300_e98077 + locals.var_dvthscsti);
        let assign63300_e98081: f64 = (assign63300_e98079 - locals.var_vbsz__blk440);
        (assign63300_e98081, (((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbsz__blk440_dn0), (((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbsz__blk440_dn2), (((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) + locals.var_dvthscsti_dn4) - locals.var_vbsz__blk440_dn4), (((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) + locals.var_dvthscsti_dn5) - locals.var_vbsz__blk440_dn5), (((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbsz__blk440_dn6), (((locals.var_t10_dn7 + (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbsz__blk440_dn7), (((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) + locals.var_dvthscsti_dn8) - locals.var_vbsz__blk440_dn8), (((locals.var_t10_dn9 + (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))) + locals.var_dvthscsti_dn9) - locals.var_vbsz__blk440_dn9), (((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbsz__blk440_dn10), (((locals.var_t10_dn11 + (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbsz__blk440_dn11), (((locals.var_t10_dn14 + (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14))) + locals.var_dvthscsti_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63300_e98083;
        locals.var_t3_dn0 = assign63300_e98083_d_n0;
        locals.var_t3_dn2 = assign63300_e98083_d_n2;
        locals.var_t3_dn4 = assign63300_e98083_d_n4;
        locals.var_t3_dn5 = assign63300_e98083_d_n5;
        locals.var_t3_dn6 = assign63300_e98083_d_n6;
        locals.var_t3_dn7 = assign63300_e98083_d_n7;
        locals.var_t3_dn8 = assign63300_e98083_d_n8;
        locals.var_t3_dn9 = assign63300_e98083_d_n9;
        locals.var_t3_dn10 = assign63300_e98083_d_n10;
        locals.var_t3_dn11 = assign63300_e98083_d_n11;
        locals.var_t3_dn14 = assign63300_e98083_d_n14;

        let (assign63310_e98096, assign63310_e98096_d_n0, assign63310_e98096_d_n2, assign63310_e98096_d_n4, assign63310_e98096_d_n5, assign63310_e98096_d_n6, assign63310_e98096_d_n7, assign63310_e98096_d_n8, assign63310_e98096_d_n9, assign63310_e98096_d_n10, assign63310_e98096_d_n11, assign63310_e98096_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63310_e98092: f64 = (locals.var_beta * locals.var_t3);
        let assign63310_e98094: f64 = (assign63310_e98092 - 1.0);
        (assign63310_e98094, ((locals.var_beta_dn0 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn0)), ((locals.var_beta_dn2 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn2)), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), ((locals.var_beta_dn5 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn5)), ((locals.var_beta_dn6 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn6)), ((locals.var_beta_dn7 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn7)), ((locals.var_beta_dn8 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn8)), ((locals.var_beta_dn9 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn9)), ((locals.var_beta_dn10 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn10)), ((locals.var_beta_dn11 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn11)), ((locals.var_beta_dn14 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63310_e98096;
        locals.var_t4_dn0 = assign63310_e98096_d_n0;
        locals.var_t4_dn2 = assign63310_e98096_d_n2;
        locals.var_t4_dn4 = assign63310_e98096_d_n4;
        locals.var_t4_dn5 = assign63310_e98096_d_n5;
        locals.var_t4_dn6 = assign63310_e98096_d_n6;
        locals.var_t4_dn7 = assign63310_e98096_d_n7;
        locals.var_t4_dn8 = assign63310_e98096_d_n8;
        locals.var_t4_dn9 = assign63310_e98096_d_n9;
        locals.var_t4_dn10 = assign63310_e98096_d_n10;
        locals.var_t4_dn11 = assign63310_e98096_d_n11;
        locals.var_t4_dn14 = assign63310_e98096_d_n14;

        let (assign63320_e98107, assign63320_e98107_d_n0, assign63320_e98107_d_n2, assign63320_e98107_d_n4, assign63320_e98107_d_n5, assign63320_e98107_d_n6, assign63320_e98107_d_n7, assign63320_e98107_d_n8, assign63320_e98107_d_n9, assign63320_e98107_d_n10, assign63320_e98107_d_n11, assign63320_e98107_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63320_e98105: f64 = (4.0 / locals.var_costi5);
        (assign63320_e98105, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn4) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn5) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn8) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn9) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn14) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63320_e98107;
        locals.var_t5_dn0 = assign63320_e98107_d_n0;
        locals.var_t5_dn2 = assign63320_e98107_d_n2;
        locals.var_t5_dn4 = assign63320_e98107_d_n4;
        locals.var_t5_dn5 = assign63320_e98107_d_n5;
        locals.var_t5_dn6 = assign63320_e98107_d_n6;
        locals.var_t5_dn7 = assign63320_e98107_d_n7;
        locals.var_t5_dn8 = assign63320_e98107_d_n8;
        locals.var_t5_dn9 = assign63320_e98107_d_n9;
        locals.var_t5_dn10 = assign63320_e98107_d_n10;
        locals.var_t5_dn11 = assign63320_e98107_d_n11;
        locals.var_t5_dn14 = assign63320_e98107_d_n14;

        let (assign63330_e98120, assign63330_e98120_d_n0, assign63330_e98120_d_n2, assign63330_e98120_d_n4, assign63330_e98120_d_n5, assign63330_e98120_d_n6, assign63330_e98120_d_n7, assign63330_e98120_d_n8, assign63330_e98120_d_n9, assign63330_e98120_d_n10, assign63330_e98120_d_n11, assign63330_e98120_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63330_e98117: f64 = (locals.var_t4 * locals.var_t5);
        let assign63330_e98118: f64 = (1.0 + assign63330_e98117);
        (assign63330_e98118, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63330_e98120;
        locals.var_t1_dn0 = assign63330_e98120_d_n0;
        locals.var_t1_dn2 = assign63330_e98120_d_n2;
        locals.var_t1_dn4 = assign63330_e98120_d_n4;
        locals.var_t1_dn5 = assign63330_e98120_d_n5;
        locals.var_t1_dn6 = assign63330_e98120_d_n6;
        locals.var_t1_dn7 = assign63330_e98120_d_n7;
        locals.var_t1_dn8 = assign63330_e98120_d_n8;
        locals.var_t1_dn9 = assign63330_e98120_d_n9;
        locals.var_t1_dn10 = assign63330_e98120_d_n10;
        locals.var_t1_dn11 = assign63330_e98120_d_n11;
        locals.var_t1_dn14 = assign63330_e98120_d_n14;

    }
}
