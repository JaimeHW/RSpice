#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_272(
        locals: &mut StampLocals,
    ) {
        let (assign79020_e119536, assign79020_e119536_d_n0, assign79020_e119536_d_n2, assign79020_e119536_d_n4, assign79020_e119536_d_n5, assign79020_e119536_d_n6, assign79020_e119536_d_n7, assign79020_e119536_d_n8, assign79020_e119536_d_n9, assign79020_e119536_d_n10, assign79020_e119536_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79020_e119528: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79020_e119530: f64 = (-locals.var_t0);
        let assign79020_e119531: f64 = { let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79020_e119532: f64 = (assign79020_e119528 + assign79020_e119531);
        let assign79020_e119534: f64 = (assign79020_e119532 / 2.0);
        (assign79020_e119534, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign79020_e119530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79020_e119536;
        locals.var_t1_dn0 = assign79020_e119536_d_n0;
        locals.var_t1_dn2 = assign79020_e119536_d_n2;
        locals.var_t1_dn4 = assign79020_e119536_d_n4;
        locals.var_t1_dn5 = assign79020_e119536_d_n5;
        locals.var_t1_dn6 = assign79020_e119536_d_n6;
        locals.var_t1_dn7 = assign79020_e119536_d_n7;
        locals.var_t1_dn8 = assign79020_e119536_d_n8;
        locals.var_t1_dn9 = assign79020_e119536_d_n9;
        locals.var_t1_dn10 = assign79020_e119536_d_n10;
        locals.var_t1_dn13 = assign79020_e119536_d_n13;

        let (assign79030_e119548, assign79030_e119548_d_n0, assign79030_e119548_d_n2, assign79030_e119548_d_n4, assign79030_e119548_d_n5, assign79030_e119548_d_n6, assign79030_e119548_d_n7, assign79030_e119548_d_n8, assign79030_e119548_d_n9, assign79030_e119548_d_n10, assign79030_e119548_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79030_e119544: f64 = (locals.var_t1).ln();
        let assign79030_e119546: f64 = (assign79030_e119544 / locals.var_dphi_sb);
        (assign79030_e119546, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign79030_e119544 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign79030_e119548;
        locals.var_c_sb_dn0 = assign79030_e119548_d_n0;
        locals.var_c_sb_dn2 = assign79030_e119548_d_n2;
        locals.var_c_sb_dn4 = assign79030_e119548_d_n4;
        locals.var_c_sb_dn5 = assign79030_e119548_d_n5;
        locals.var_c_sb_dn6 = assign79030_e119548_d_n6;
        locals.var_c_sb_dn7 = assign79030_e119548_d_n7;
        locals.var_c_sb_dn8 = assign79030_e119548_d_n8;
        locals.var_c_sb_dn9 = assign79030_e119548_d_n9;
        locals.var_c_sb_dn10 = assign79030_e119548_d_n10;
        locals.var_c_sb_dn13 = assign79030_e119548_d_n13;

        let (assign79040_e119557,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79040_e119557;

    }

    pub(super) fn stamp_transient_block_273(
        locals: &mut StampLocals,
    ) {
        let mut assign79050_loop_guard: usize = 0;
        while {
            let assign79050_cond_e119567: f64 = (locals.var_lp_s0_max + 1.0);
            let assign79050_cond_e119569: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_lp_s0 <= assign79050_cond_e119567)) { 1.0 } else { 0.0 };
            assign79050_cond_e119569 != 0.0
        } {
            assign79050_loop_guard += 1;
            assert!(assign79050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign79050_body3_e119605, assign79050_body3_e119605_d_n0, assign79050_body3_e119605_d_n2, assign79050_body3_e119605_d_n4, assign79050_body3_e119605_d_n5, assign79050_body3_e119605_d_n6, assign79050_body3_e119605_d_n7, assign79050_body3_e119605_d_n8, assign79050_body3_e119605_d_n9, assign79050_body3_e119605_d_n10, assign79050_body3_e119605_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body3_e119603: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign79050_body3_e119603, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign79050_body3_e119605;
            locals.var_ps0ld_vxb_dn0 = assign79050_body3_e119605_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign79050_body3_e119605_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign79050_body3_e119605_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign79050_body3_e119605_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign79050_body3_e119605_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign79050_body3_e119605_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign79050_body3_e119605_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign79050_body3_e119605_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign79050_body3_e119605_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign79050_body3_e119605_d_n13;
            let (assign79050_body4_e119616, assign79050_body4_e119616_d_n0, assign79050_body4_e119616_d_n2, assign79050_body4_e119616_d_n4, assign79050_body4_e119616_d_n5, assign79050_body4_e119616_d_n6, assign79050_body4_e119616_d_n7, assign79050_body4_e119616_d_n8, assign79050_body4_e119616_d_n9, assign79050_body4_e119616_d_n10, assign79050_body4_e119616_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body4_e119614: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign79050_body4_e119614, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign79050_body4_e119616;
            locals.var_chi_dn0 = assign79050_body4_e119616_d_n0;
            locals.var_chi_dn2 = assign79050_body4_e119616_d_n2;
            locals.var_chi_dn4 = assign79050_body4_e119616_d_n4;
            locals.var_chi_dn5 = assign79050_body4_e119616_d_n5;
            locals.var_chi_dn6 = assign79050_body4_e119616_d_n6;
            locals.var_chi_dn7 = assign79050_body4_e119616_d_n7;
            locals.var_chi_dn8 = assign79050_body4_e119616_d_n8;
            locals.var_chi_dn9 = assign79050_body4_e119616_d_n9;
            locals.var_chi_dn10 = assign79050_body4_e119616_d_n10;
            locals.var_chi_dn13 = assign79050_body4_e119616_d_n13;
            let (assign79050_body5_e119629, assign79050_body5_e119629_d_n0, assign79050_body5_e119629_d_n2, assign79050_body5_e119629_d_n4, assign79050_body5_e119629_d_n5, assign79050_body5_e119629_d_n6, assign79050_body5_e119629_d_n7, assign79050_body5_e119629_d_n8, assign79050_body5_e119629_d_n9, assign79050_body5_e119629_d_n10, assign79050_body5_e119629_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body5_e119626: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign79050_body5_e119627: f64 = (locals.var_c_sb * assign79050_body5_e119626);
        (assign79050_body5_e119627, ((locals.var_c_sb_dn0 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign79050_body5_e119626) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign79050_body5_e119629;
            locals.var_ty_dn0 = assign79050_body5_e119629_d_n0;
            locals.var_ty_dn2 = assign79050_body5_e119629_d_n2;
            locals.var_ty_dn4 = assign79050_body5_e119629_d_n4;
            locals.var_ty_dn5 = assign79050_body5_e119629_d_n5;
            locals.var_ty_dn6 = assign79050_body5_e119629_d_n6;
            locals.var_ty_dn7 = assign79050_body5_e119629_d_n7;
            locals.var_ty_dn8 = assign79050_body5_e119629_d_n8;
            locals.var_ty_dn9 = assign79050_body5_e119629_d_n9;
            locals.var_ty_dn10 = assign79050_body5_e119629_d_n10;
            locals.var_ty_dn13 = assign79050_body5_e119629_d_n13;
            let assign79050_body6_e119632: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1830 = assign79050_body6_e119632;
            let (assign79050_body7_e119644, assign79050_body7_e119644_d_n0, assign79050_body7_e119644_d_n2, assign79050_body7_e119644_d_n4, assign79050_body7_e119644_d_n5, assign79050_body7_e119644_d_n6, assign79050_body7_e119644_d_n7, assign79050_body7_e119644_d_n8, assign79050_body7_e119644_d_n9, assign79050_body7_e119644_d_n10, assign79050_body7_e119644_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body7_e119642: f64 = (locals.var_ty).exp();
        (assign79050_body7_e119642, (assign79050_body7_e119642 * locals.var_ty_dn0), (assign79050_body7_e119642 * locals.var_ty_dn2), (assign79050_body7_e119642 * locals.var_ty_dn4), (assign79050_body7_e119642 * locals.var_ty_dn5), (assign79050_body7_e119642 * locals.var_ty_dn6), (assign79050_body7_e119642 * locals.var_ty_dn7), (assign79050_body7_e119642 * locals.var_ty_dn8), (assign79050_body7_e119642 * locals.var_ty_dn9), (assign79050_body7_e119642 * locals.var_ty_dn10), (assign79050_body7_e119642 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body7_e119644;
            locals.var_t1_dn0 = assign79050_body7_e119644_d_n0;
            locals.var_t1_dn2 = assign79050_body7_e119644_d_n2;
            locals.var_t1_dn4 = assign79050_body7_e119644_d_n4;
            locals.var_t1_dn5 = assign79050_body7_e119644_d_n5;
            locals.var_t1_dn6 = assign79050_body7_e119644_d_n6;
            locals.var_t1_dn7 = assign79050_body7_e119644_d_n7;
            locals.var_t1_dn8 = assign79050_body7_e119644_d_n8;
            locals.var_t1_dn9 = assign79050_body7_e119644_d_n9;
            locals.var_t1_dn10 = assign79050_body7_e119644_d_n10;
            locals.var_t1_dn13 = assign79050_body7_e119644_d_n13;
            let (assign79050_body8_e119659, assign79050_body8_e119659_d_n0, assign79050_body8_e119659_d_n2, assign79050_body8_e119659_d_n4, assign79050_body8_e119659_d_n5, assign79050_body8_e119659_d_n6, assign79050_body8_e119659_d_n7, assign79050_body8_e119659_d_n8, assign79050_body8_e119659_d_n9, assign79050_body8_e119659_d_n10, assign79050_body8_e119659_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body8_e119654: f64 = (-locals.var_c_sb);
        let assign79050_body8_e119656: f64 = (assign79050_body8_e119654 * locals.var_dphi_sb);
        let assign79050_body8_e119657: f64 = (assign79050_body8_e119656).exp();
        (assign79050_body8_e119657, (assign79050_body8_e119657 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn0))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn2))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn4))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn5))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn6))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn7))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn8))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn9))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn10))), (assign79050_body8_e119657 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign79050_body8_e119654 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body8_e119659;
            locals.var_t0_dn0 = assign79050_body8_e119659_d_n0;
            locals.var_t0_dn2 = assign79050_body8_e119659_d_n2;
            locals.var_t0_dn4 = assign79050_body8_e119659_d_n4;
            locals.var_t0_dn5 = assign79050_body8_e119659_d_n5;
            locals.var_t0_dn6 = assign79050_body8_e119659_d_n6;
            locals.var_t0_dn7 = assign79050_body8_e119659_d_n7;
            locals.var_t0_dn8 = assign79050_body8_e119659_d_n8;
            locals.var_t0_dn9 = assign79050_body8_e119659_d_n9;
            locals.var_t0_dn10 = assign79050_body8_e119659_d_n10;
            locals.var_t0_dn13 = assign79050_body8_e119659_d_n13;
            let (assign79050_body9_e119672, assign79050_body9_e119672_d_n0, assign79050_body9_e119672_d_n2, assign79050_body9_e119672_d_n4, assign79050_body9_e119672_d_n5, assign79050_body9_e119672_d_n6, assign79050_body9_e119672_d_n7, assign79050_body9_e119672_d_n8, assign79050_body9_e119672_d_n9, assign79050_body9_e119672_d_n10, assign79050_body9_e119672_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body9_e119670: f64 = (locals.var_t1 - locals.var_t0);
        (assign79050_body9_e119670, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79050_body9_e119672;
            locals.var_t2_dn0 = assign79050_body9_e119672_d_n0;
            locals.var_t2_dn2 = assign79050_body9_e119672_d_n2;
            locals.var_t2_dn4 = assign79050_body9_e119672_d_n4;
            locals.var_t2_dn5 = assign79050_body9_e119672_d_n5;
            locals.var_t2_dn6 = assign79050_body9_e119672_d_n6;
            locals.var_t2_dn7 = assign79050_body9_e119672_d_n7;
            locals.var_t2_dn8 = assign79050_body9_e119672_d_n8;
            locals.var_t2_dn9 = assign79050_body9_e119672_d_n9;
            locals.var_t2_dn10 = assign79050_body9_e119672_d_n10;
            locals.var_t2_dn13 = assign79050_body9_e119672_d_n13;
            let (assign79050_body10_e119688, assign79050_body10_e119688_d_n0, assign79050_body10_e119688_d_n2, assign79050_body10_e119688_d_n4, assign79050_body10_e119688_d_n5, assign79050_body10_e119688_d_n6, assign79050_body10_e119688_d_n7, assign79050_body10_e119688_d_n8, assign79050_body10_e119688_d_n9, assign79050_body10_e119688_d_n10, assign79050_body10_e119688_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body10_e119683: f64 = (1.0 + locals.var_t2);
        let assign79050_body10_e119684: f64 = (assign79050_body10_e119683).ln();
        let assign79050_body10_e119686: f64 = (assign79050_body10_e119684 / locals.var_c_sb);
        (assign79050_body10_e119686, ((((locals.var_t2_dn0 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign79050_body10_e119683) * locals.var_c_sb) - (assign79050_body10_e119684 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79050_body10_e119688;
            locals.var_phi_b_dn0 = assign79050_body10_e119688_d_n0;
            locals.var_phi_b_dn2 = assign79050_body10_e119688_d_n2;
            locals.var_phi_b_dn4 = assign79050_body10_e119688_d_n4;
            locals.var_phi_b_dn5 = assign79050_body10_e119688_d_n5;
            locals.var_phi_b_dn6 = assign79050_body10_e119688_d_n6;
            locals.var_phi_b_dn7 = assign79050_body10_e119688_d_n7;
            locals.var_phi_b_dn8 = assign79050_body10_e119688_d_n8;
            locals.var_phi_b_dn9 = assign79050_body10_e119688_d_n9;
            locals.var_phi_b_dn10 = assign79050_body10_e119688_d_n10;
            locals.var_phi_b_dn13 = assign79050_body10_e119688_d_n13;
            let (assign79050_body11_e119703, assign79050_body11_e119703_d_n0, assign79050_body11_e119703_d_n2, assign79050_body11_e119703_d_n4, assign79050_body11_e119703_d_n5, assign79050_body11_e119703_d_n6, assign79050_body11_e119703_d_n7, assign79050_body11_e119703_d_n8, assign79050_body11_e119703_d_n9, assign79050_body11_e119703_d_n10, assign79050_body11_e119703_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign79050_body11_e119700: f64 = (1.0 + locals.var_t2);
        let assign79050_body11_e119701: f64 = (locals.var_t1 / assign79050_body11_e119700);
        (assign79050_body11_e119701, (((locals.var_t1_dn0 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn0)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn2 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn2)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn4 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn4)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn5 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn5)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn6 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn6)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn7 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn7)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn8 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn8)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn9 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn9)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn10 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn10)) / (assign79050_body11_e119700 * assign79050_body11_e119700)), (((locals.var_t1_dn13 * assign79050_body11_e119700) - (locals.var_t1 * locals.var_t2_dn13)) / (assign79050_body11_e119700 * assign79050_body11_e119700)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79050_body11_e119703;
            locals.var_phi_b_dpss_dn0 = assign79050_body11_e119703_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79050_body11_e119703_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79050_body11_e119703_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79050_body11_e119703_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79050_body11_e119703_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79050_body11_e119703_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79050_body11_e119703_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79050_body11_e119703_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79050_body11_e119703_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79050_body11_e119703_d_n13;
            let (assign79050_body13_e119731, assign79050_body13_e119731_d_n0, assign79050_body13_e119731_d_n2, assign79050_body13_e119731_d_n4, assign79050_body13_e119731_d_n5, assign79050_body13_e119731_d_n6, assign79050_body13_e119731_d_n7, assign79050_body13_e119731_d_n8, assign79050_body13_e119731_d_n9, assign79050_body13_e119731_d_n10, assign79050_body13_e119731_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign79050_body13_e119729: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign79050_body13_e119729, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79050_body13_e119731;
            locals.var_phi_b_dn0 = assign79050_body13_e119731_d_n0;
            locals.var_phi_b_dn2 = assign79050_body13_e119731_d_n2;
            locals.var_phi_b_dn4 = assign79050_body13_e119731_d_n4;
            locals.var_phi_b_dn5 = assign79050_body13_e119731_d_n5;
            locals.var_phi_b_dn6 = assign79050_body13_e119731_d_n6;
            locals.var_phi_b_dn7 = assign79050_body13_e119731_d_n7;
            locals.var_phi_b_dn8 = assign79050_body13_e119731_d_n8;
            locals.var_phi_b_dn9 = assign79050_body13_e119731_d_n9;
            locals.var_phi_b_dn10 = assign79050_body13_e119731_d_n10;
            locals.var_phi_b_dn13 = assign79050_body13_e119731_d_n13;
            let (assign79050_body14_e119743, assign79050_body14_e119743_d_n0, assign79050_body14_e119743_d_n2, assign79050_body14_e119743_d_n4, assign79050_body14_e119743_d_n5, assign79050_body14_e119743_d_n6, assign79050_body14_e119743_d_n7, assign79050_body14_e119743_d_n8, assign79050_body14_e119743_d_n9, assign79050_body14_e119743_d_n10, assign79050_body14_e119743_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79050_body14_e119743;
            locals.var_phi_b_dpss_dn0 = assign79050_body14_e119743_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79050_body14_e119743_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79050_body14_e119743_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79050_body14_e119743_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79050_body14_e119743_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79050_body14_e119743_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79050_body14_e119743_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79050_body14_e119743_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79050_body14_e119743_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79050_body14_e119743_d_n13;
            let (assign79050_body15_e119754, assign79050_body15_e119754_d_n0, assign79050_body15_e119754_d_n2, assign79050_body15_e119754_d_n4, assign79050_body15_e119754_d_n5, assign79050_body15_e119754_d_n6, assign79050_body15_e119754_d_n7, assign79050_body15_e119754_d_n8, assign79050_body15_e119754_d_n9, assign79050_body15_e119754_d_n10, assign79050_body15_e119754_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body15_e119752: f64 = (locals.var_beta * locals.var_phi_b);
        (assign79050_body15_e119752, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign79050_body15_e119754;
            locals.var_chib_dn0 = assign79050_body15_e119754_d_n0;
            locals.var_chib_dn2 = assign79050_body15_e119754_d_n2;
            locals.var_chib_dn4 = assign79050_body15_e119754_d_n4;
            locals.var_chib_dn5 = assign79050_body15_e119754_d_n5;
            locals.var_chib_dn6 = assign79050_body15_e119754_d_n6;
            locals.var_chib_dn7 = assign79050_body15_e119754_d_n7;
            locals.var_chib_dn8 = assign79050_body15_e119754_d_n8;
            locals.var_chib_dn9 = assign79050_body15_e119754_d_n9;
            locals.var_chib_dn10 = assign79050_body15_e119754_d_n10;
            locals.var_chib_dn13 = assign79050_body15_e119754_d_n13;
            let assign79050_body16_e119757: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1831 = assign79050_body16_e119757;
            let (assign79050_body18_e119782, assign79050_body18_e119782_d_n0, assign79050_body18_e119782_d_n2, assign79050_body18_e119782_d_n4, assign79050_body18_e119782_d_n5, assign79050_body18_e119782_d_n6, assign79050_body18_e119782_d_n7, assign79050_body18_e119782_d_n8, assign79050_body18_e119782_d_n9, assign79050_body18_e119782_d_n10, assign79050_body18_e119782_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body18_e119780: f64 = (-0.7071067811865475);
        (assign79050_body18_e119780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body18_e119782;
            locals.var_t0_dn0 = assign79050_body18_e119782_d_n0;
            locals.var_t0_dn2 = assign79050_body18_e119782_d_n2;
            locals.var_t0_dn4 = assign79050_body18_e119782_d_n4;
            locals.var_t0_dn5 = assign79050_body18_e119782_d_n5;
            locals.var_t0_dn6 = assign79050_body18_e119782_d_n6;
            locals.var_t0_dn7 = assign79050_body18_e119782_d_n7;
            locals.var_t0_dn8 = assign79050_body18_e119782_d_n8;
            locals.var_t0_dn9 = assign79050_body18_e119782_d_n9;
            locals.var_t0_dn10 = assign79050_body18_e119782_d_n10;
            locals.var_t0_dn13 = assign79050_body18_e119782_d_n13;
            let (assign79050_body19_e119795, assign79050_body19_e119795_d_n0, assign79050_body19_e119795_d_n2, assign79050_body19_e119795_d_n4, assign79050_body19_e119795_d_n5, assign79050_body19_e119795_d_n6, assign79050_body19_e119795_d_n7, assign79050_body19_e119795_d_n8, assign79050_body19_e119795_d_n9, assign79050_body19_e119795_d_n10, assign79050_body19_e119795_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body19_e119793: f64 = (locals.var_chi * locals.var_t0);
        (assign79050_body19_e119793, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body19_e119795;
            locals.var_fb_dn0 = assign79050_body19_e119795_d_n0;
            locals.var_fb_dn2 = assign79050_body19_e119795_d_n2;
            locals.var_fb_dn4 = assign79050_body19_e119795_d_n4;
            locals.var_fb_dn5 = assign79050_body19_e119795_d_n5;
            locals.var_fb_dn6 = assign79050_body19_e119795_d_n6;
            locals.var_fb_dn7 = assign79050_body19_e119795_d_n7;
            locals.var_fb_dn8 = assign79050_body19_e119795_d_n8;
            locals.var_fb_dn9 = assign79050_body19_e119795_d_n9;
            locals.var_fb_dn10 = assign79050_body19_e119795_d_n10;
            locals.var_fb_dn13 = assign79050_body19_e119795_d_n13;
            let (assign79050_body20_e119808, assign79050_body20_e119808_d_n0, assign79050_body20_e119808_d_n2, assign79050_body20_e119808_d_n4, assign79050_body20_e119808_d_n5, assign79050_body20_e119808_d_n6, assign79050_body20_e119808_d_n7, assign79050_body20_e119808_d_n8, assign79050_body20_e119808_d_n9, assign79050_body20_e119808_d_n10, assign79050_body20_e119808_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 != 0.0)) {
        let assign79050_body20_e119806: f64 = (locals.var_beta * locals.var_t0);
        (assign79050_body20_e119806, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body20_e119808;
            locals.var_fb_dpss_dn0 = assign79050_body20_e119808_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body20_e119808_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body20_e119808_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body20_e119808_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body20_e119808_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body20_e119808_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body20_e119808_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body20_e119808_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body20_e119808_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body20_e119808_d_n13;
            let assign79050_body21_e119811: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1832 = assign79050_body21_e119811;
            let (assign79050_body23_e119863, assign79050_body23_e119863_d_n0, assign79050_body23_e119863_d_n2, assign79050_body23_e119863_d_n4, assign79050_body23_e119863_d_n5, assign79050_body23_e119863_d_n6, assign79050_body23_e119863_d_n7, assign79050_body23_e119863_d_n8, assign79050_body23_e119863_d_n9, assign79050_body23_e119863_d_n10, assign79050_body23_e119863_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body23_e119841: f64 = (locals.var_chi * locals.var_chi);
        let assign79050_body23_e119843: f64 = (assign79050_body23_e119841 / 2.0);
        let assign79050_body23_e119847: f64 = (locals.var_chi / 3.0);
        let assign79050_body23_e119851: f64 = (locals.var_chi / 4.0);
        let assign79050_body23_e119855: f64 = (locals.var_chi / 5.0);
        let assign79050_body23_e119856: f64 = (1.0 - assign79050_body23_e119855);
        let assign79050_body23_e119857: f64 = (assign79050_body23_e119851 * assign79050_body23_e119856);
        let assign79050_body23_e119858: f64 = (1.0 - assign79050_body23_e119857);
        let assign79050_body23_e119859: f64 = (assign79050_body23_e119847 * assign79050_body23_e119858);
        let assign79050_body23_e119860: f64 = (1.0 - assign79050_body23_e119859);
        let assign79050_body23_e119861: f64 = (assign79050_body23_e119843 * assign79050_body23_e119860);
        (assign79050_body23_e119861, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn0 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn0 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn2 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn2 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn4 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn4 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn5 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn5 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn6 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn6 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn7 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn7 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn8 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn8 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn9 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn9 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn10 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn10 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79050_body23_e119860) + (assign79050_body23_e119843 * (-(((locals.var_chi_dn13 / 3.0) * assign79050_body23_e119858) + (assign79050_body23_e119847 * (-(((locals.var_chi_dn13 / 4.0) * assign79050_body23_e119856) + (assign79050_body23_e119851 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body23_e119863;
            locals.var_t0_dn0 = assign79050_body23_e119863_d_n0;
            locals.var_t0_dn2 = assign79050_body23_e119863_d_n2;
            locals.var_t0_dn4 = assign79050_body23_e119863_d_n4;
            locals.var_t0_dn5 = assign79050_body23_e119863_d_n5;
            locals.var_t0_dn6 = assign79050_body23_e119863_d_n6;
            locals.var_t0_dn7 = assign79050_body23_e119863_d_n7;
            locals.var_t0_dn8 = assign79050_body23_e119863_d_n8;
            locals.var_t0_dn9 = assign79050_body23_e119863_d_n9;
            locals.var_t0_dn10 = assign79050_body23_e119863_d_n10;
            locals.var_t0_dn13 = assign79050_body23_e119863_d_n13;
            let (assign79050_body24_e119895, assign79050_body24_e119895_d_n0, assign79050_body24_e119895_d_n2, assign79050_body24_e119895_d_n4, assign79050_body24_e119895_d_n5, assign79050_body24_e119895_d_n6, assign79050_body24_e119895_d_n7, assign79050_body24_e119895_d_n8, assign79050_body24_e119895_d_n9, assign79050_body24_e119895_d_n10, assign79050_body24_e119895_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body24_e119879: f64 = (locals.var_chi / 2.0);
        let assign79050_body24_e119883: f64 = (locals.var_chi / 3.0);
        let assign79050_body24_e119887: f64 = (locals.var_chi / 4.0);
        let assign79050_body24_e119888: f64 = (1.0 - assign79050_body24_e119887);
        let assign79050_body24_e119889: f64 = (assign79050_body24_e119883 * assign79050_body24_e119888);
        let assign79050_body24_e119890: f64 = (1.0 - assign79050_body24_e119889);
        let assign79050_body24_e119891: f64 = (assign79050_body24_e119879 * assign79050_body24_e119890);
        let assign79050_body24_e119892: f64 = (1.0 - assign79050_body24_e119891);
        let assign79050_body24_e119893: f64 = (locals.var_chi * assign79050_body24_e119892);
        (assign79050_body24_e119893, ((locals.var_chi_dn0 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn0 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn2 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn4 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn5 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn6 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn7 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn8 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn9 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn10 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign79050_body24_e119892) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign79050_body24_e119890) + (assign79050_body24_e119879 * (-(((locals.var_chi_dn13 / 3.0) * assign79050_body24_e119888) + (assign79050_body24_e119883 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body24_e119895;
            locals.var_t1_dn0 = assign79050_body24_e119895_d_n0;
            locals.var_t1_dn2 = assign79050_body24_e119895_d_n2;
            locals.var_t1_dn4 = assign79050_body24_e119895_d_n4;
            locals.var_t1_dn5 = assign79050_body24_e119895_d_n5;
            locals.var_t1_dn6 = assign79050_body24_e119895_d_n6;
            locals.var_t1_dn7 = assign79050_body24_e119895_d_n7;
            locals.var_t1_dn8 = assign79050_body24_e119895_d_n8;
            locals.var_t1_dn9 = assign79050_body24_e119895_d_n9;
            locals.var_t1_dn10 = assign79050_body24_e119895_d_n10;
            locals.var_t1_dn13 = assign79050_body24_e119895_d_n13;
            let (assign79050_body25_e119931, assign79050_body25_e119931_d_n0, assign79050_body25_e119931_d_n2, assign79050_body25_e119931_d_n4, assign79050_body25_e119931_d_n5, assign79050_body25_e119931_d_n6, assign79050_body25_e119931_d_n7, assign79050_body25_e119931_d_n8, assign79050_body25_e119931_d_n9, assign79050_body25_e119931_d_n10, assign79050_body25_e119931_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body25_e119909: f64 = (locals.var_chib * locals.var_chib);
        let assign79050_body25_e119911: f64 = (assign79050_body25_e119909 / 2.0);
        let assign79050_body25_e119915: f64 = (locals.var_chib / 3.0);
        let assign79050_body25_e119919: f64 = (locals.var_chib / 4.0);
        let assign79050_body25_e119923: f64 = (locals.var_chib / 5.0);
        let assign79050_body25_e119924: f64 = (1.0 - assign79050_body25_e119923);
        let assign79050_body25_e119925: f64 = (assign79050_body25_e119919 * assign79050_body25_e119924);
        let assign79050_body25_e119926: f64 = (1.0 - assign79050_body25_e119925);
        let assign79050_body25_e119927: f64 = (assign79050_body25_e119915 * assign79050_body25_e119926);
        let assign79050_body25_e119928: f64 = (1.0 - assign79050_body25_e119927);
        let assign79050_body25_e119929: f64 = (assign79050_body25_e119911 * assign79050_body25_e119928);
        (assign79050_body25_e119929, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn0 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn0 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn2 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn2 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn4 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn4 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn5 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn5 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn6 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn6 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn7 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn7 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn8 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn8 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn9 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn9 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn10 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn10 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign79050_body25_e119928) + (assign79050_body25_e119911 * (-(((locals.var_chib_dn13 / 3.0) * assign79050_body25_e119926) + (assign79050_body25_e119915 * (-(((locals.var_chib_dn13 / 4.0) * assign79050_body25_e119924) + (assign79050_body25_e119919 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79050_body25_e119931;
            locals.var_t2_dn0 = assign79050_body25_e119931_d_n0;
            locals.var_t2_dn2 = assign79050_body25_e119931_d_n2;
            locals.var_t2_dn4 = assign79050_body25_e119931_d_n4;
            locals.var_t2_dn5 = assign79050_body25_e119931_d_n5;
            locals.var_t2_dn6 = assign79050_body25_e119931_d_n6;
            locals.var_t2_dn7 = assign79050_body25_e119931_d_n7;
            locals.var_t2_dn8 = assign79050_body25_e119931_d_n8;
            locals.var_t2_dn9 = assign79050_body25_e119931_d_n9;
            locals.var_t2_dn10 = assign79050_body25_e119931_d_n10;
            locals.var_t2_dn13 = assign79050_body25_e119931_d_n13;
            let (assign79050_body26_e119963, assign79050_body26_e119963_d_n0, assign79050_body26_e119963_d_n2, assign79050_body26_e119963_d_n4, assign79050_body26_e119963_d_n5, assign79050_body26_e119963_d_n6, assign79050_body26_e119963_d_n7, assign79050_body26_e119963_d_n8, assign79050_body26_e119963_d_n9, assign79050_body26_e119963_d_n10, assign79050_body26_e119963_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body26_e119947: f64 = (locals.var_chib / 2.0);
        let assign79050_body26_e119951: f64 = (locals.var_chib / 3.0);
        let assign79050_body26_e119955: f64 = (locals.var_chib / 4.0);
        let assign79050_body26_e119956: f64 = (1.0 - assign79050_body26_e119955);
        let assign79050_body26_e119957: f64 = (assign79050_body26_e119951 * assign79050_body26_e119956);
        let assign79050_body26_e119958: f64 = (1.0 - assign79050_body26_e119957);
        let assign79050_body26_e119959: f64 = (assign79050_body26_e119947 * assign79050_body26_e119958);
        let assign79050_body26_e119960: f64 = (1.0 - assign79050_body26_e119959);
        let assign79050_body26_e119961: f64 = (locals.var_chib * assign79050_body26_e119960);
        (assign79050_body26_e119961, ((locals.var_chib_dn0 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn0 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn2 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn4 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn5 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn6 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn7 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn8 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn9 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn10 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign79050_body26_e119960) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign79050_body26_e119958) + (assign79050_body26_e119947 * (-(((locals.var_chib_dn13 / 3.0) * assign79050_body26_e119956) + (assign79050_body26_e119951 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign79050_body26_e119963;
            locals.var_t3_dn0 = assign79050_body26_e119963_d_n0;
            locals.var_t3_dn2 = assign79050_body26_e119963_d_n2;
            locals.var_t3_dn4 = assign79050_body26_e119963_d_n4;
            locals.var_t3_dn5 = assign79050_body26_e119963_d_n5;
            locals.var_t3_dn6 = assign79050_body26_e119963_d_n6;
            locals.var_t3_dn7 = assign79050_body26_e119963_d_n7;
            locals.var_t3_dn8 = assign79050_body26_e119963_d_n8;
            locals.var_t3_dn9 = assign79050_body26_e119963_d_n9;
            locals.var_t3_dn10 = assign79050_body26_e119963_d_n10;
            locals.var_t3_dn13 = assign79050_body26_e119963_d_n13;
            let (assign79050_body27_e119979, assign79050_body27_e119979_d_n0, assign79050_body27_e119979_d_n2, assign79050_body27_e119979_d_n4, assign79050_body27_e119979_d_n5, assign79050_body27_e119979_d_n6, assign79050_body27_e119979_d_n7, assign79050_body27_e119979_d_n8, assign79050_body27_e119979_d_n9, assign79050_body27_e119979_d_n10, assign79050_body27_e119979_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) {
        let assign79050_body27_e119977: f64 = (locals.var_t0 - locals.var_t2);
        (assign79050_body27_e119977, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign79050_body27_e119979;
            locals.var_t4_dn0 = assign79050_body27_e119979_d_n0;
            locals.var_t4_dn2 = assign79050_body27_e119979_d_n2;
            locals.var_t4_dn4 = assign79050_body27_e119979_d_n4;
            locals.var_t4_dn5 = assign79050_body27_e119979_d_n5;
            locals.var_t4_dn6 = assign79050_body27_e119979_d_n6;
            locals.var_t4_dn7 = assign79050_body27_e119979_d_n7;
            locals.var_t4_dn8 = assign79050_body27_e119979_d_n8;
            locals.var_t4_dn9 = assign79050_body27_e119979_d_n9;
            locals.var_t4_dn10 = assign79050_body27_e119979_d_n10;
            locals.var_t4_dn13 = assign79050_body27_e119979_d_n13;
            let assign79050_body28_e119982: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1833 = assign79050_body28_e119982;
            let (assign79050_body29_e119999, assign79050_body29_e119999_d_n0, assign79050_body29_e119999_d_n2, assign79050_body29_e119999_d_n4, assign79050_body29_e119999_d_n5, assign79050_body29_e119999_d_n6, assign79050_body29_e119999_d_n7, assign79050_body29_e119999_d_n8, assign79050_body29_e119999_d_n9, assign79050_body29_e119999_d_n10, assign79050_body29_e119999_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 != 0.0)) {
        let assign79050_body29_e119997: f64 = (locals.var_t4).sqrt();
        (assign79050_body29_e119997, (locals.var_t4_dn0 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn2 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn4 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn5 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn6 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn7 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn8 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn9 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn10 / (2.0 * assign79050_body29_e119997)), (locals.var_t4_dn13 / (2.0 * assign79050_body29_e119997)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body29_e119999;
            locals.var_fb_dn0 = assign79050_body29_e119999_d_n0;
            locals.var_fb_dn2 = assign79050_body29_e119999_d_n2;
            locals.var_fb_dn4 = assign79050_body29_e119999_d_n4;
            locals.var_fb_dn5 = assign79050_body29_e119999_d_n5;
            locals.var_fb_dn6 = assign79050_body29_e119999_d_n6;
            locals.var_fb_dn7 = assign79050_body29_e119999_d_n7;
            locals.var_fb_dn8 = assign79050_body29_e119999_d_n8;
            locals.var_fb_dn9 = assign79050_body29_e119999_d_n9;
            locals.var_fb_dn10 = assign79050_body29_e119999_d_n10;
            locals.var_fb_dn13 = assign79050_body29_e119999_d_n13;
            let (assign79050_body30_e120025, assign79050_body30_e120025_d_n0, assign79050_body30_e120025_d_n2, assign79050_body30_e120025_d_n4, assign79050_body30_e120025_d_n5, assign79050_body30_e120025_d_n6, assign79050_body30_e120025_d_n7, assign79050_body30_e120025_d_n8, assign79050_body30_e120025_d_n9, assign79050_body30_e120025_d_n10, assign79050_body30_e120025_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 != 0.0)) {
        let assign79050_body30_e120015: f64 = (locals.var_beta * 0.5);
        let assign79050_body30_e120019: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign79050_body30_e120020: f64 = (locals.var_t1 - assign79050_body30_e120019);
        let assign79050_body30_e120021: f64 = (assign79050_body30_e120015 * assign79050_body30_e120020);
        let assign79050_body30_e120023: f64 = (assign79050_body30_e120021 / locals.var_fb);
        (assign79050_body30_e120023, ((((((locals.var_beta_dn0 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign79050_body30_e120020) + (assign79050_body30_e120015 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign79050_body30_e120021 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body30_e120025;
            locals.var_fb_dpss_dn0 = assign79050_body30_e120025_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body30_e120025_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body30_e120025_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body30_e120025_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body30_e120025_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body30_e120025_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body30_e120025_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body30_e120025_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body30_e120025_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body30_e120025_d_n13;
            let (assign79050_body32_e120061, assign79050_body32_e120061_d_n0, assign79050_body32_e120061_d_n2, assign79050_body32_e120061_d_n4, assign79050_body32_e120061_d_n5, assign79050_body32_e120061_d_n6, assign79050_body32_e120061_d_n7, assign79050_body32_e120061_d_n8, assign79050_body32_e120061_d_n9, assign79050_body32_e120061_d_n10, assign79050_body32_e120061_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body32_e120061;
            locals.var_fb_dn0 = assign79050_body32_e120061_d_n0;
            locals.var_fb_dn2 = assign79050_body32_e120061_d_n2;
            locals.var_fb_dn4 = assign79050_body32_e120061_d_n4;
            locals.var_fb_dn5 = assign79050_body32_e120061_d_n5;
            locals.var_fb_dn6 = assign79050_body32_e120061_d_n6;
            locals.var_fb_dn7 = assign79050_body32_e120061_d_n7;
            locals.var_fb_dn8 = assign79050_body32_e120061_d_n8;
            locals.var_fb_dn9 = assign79050_body32_e120061_d_n9;
            locals.var_fb_dn10 = assign79050_body32_e120061_d_n10;
            locals.var_fb_dn13 = assign79050_body32_e120061_d_n13;
            let (assign79050_body33_e120078, assign79050_body33_e120078_d_n0, assign79050_body33_e120078_d_n2, assign79050_body33_e120078_d_n4, assign79050_body33_e120078_d_n5, assign79050_body33_e120078_d_n6, assign79050_body33_e120078_d_n7, assign79050_body33_e120078_d_n8, assign79050_body33_e120078_d_n9, assign79050_body33_e120078_d_n10, assign79050_body33_e120078_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 != 0.0)) && (locals.var_guard1833 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body33_e120078;
            locals.var_fb_dpss_dn0 = assign79050_body33_e120078_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body33_e120078_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body33_e120078_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body33_e120078_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body33_e120078_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body33_e120078_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body33_e120078_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body33_e120078_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body33_e120078_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body33_e120078_d_n13;
            let (assign79050_body34_e120095, assign79050_body34_e120095_d_n0, assign79050_body34_e120095_d_n2, assign79050_body34_e120095_d_n4, assign79050_body34_e120095_d_n5, assign79050_body34_e120095_d_n6, assign79050_body34_e120095_d_n7, assign79050_body34_e120095_d_n8, assign79050_body34_e120095_d_n9, assign79050_body34_e120095_d_n10, assign79050_body34_e120095_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body34_e120092: f64 = (-locals.var_chi);
        let assign79050_body34_e120093: f64 = (assign79050_body34_e120092).exp();
        (assign79050_body34_e120093, (assign79050_body34_e120093 * (-locals.var_chi_dn0)), (assign79050_body34_e120093 * (-locals.var_chi_dn2)), (assign79050_body34_e120093 * (-locals.var_chi_dn4)), (assign79050_body34_e120093 * (-locals.var_chi_dn5)), (assign79050_body34_e120093 * (-locals.var_chi_dn6)), (assign79050_body34_e120093 * (-locals.var_chi_dn7)), (assign79050_body34_e120093 * (-locals.var_chi_dn8)), (assign79050_body34_e120093 * (-locals.var_chi_dn9)), (assign79050_body34_e120093 * (-locals.var_chi_dn10)), (assign79050_body34_e120093 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body34_e120095;
            locals.var_t0_dn0 = assign79050_body34_e120095_d_n0;
            locals.var_t0_dn2 = assign79050_body34_e120095_d_n2;
            locals.var_t0_dn4 = assign79050_body34_e120095_d_n4;
            locals.var_t0_dn5 = assign79050_body34_e120095_d_n5;
            locals.var_t0_dn6 = assign79050_body34_e120095_d_n6;
            locals.var_t0_dn7 = assign79050_body34_e120095_d_n7;
            locals.var_t0_dn8 = assign79050_body34_e120095_d_n8;
            locals.var_t0_dn9 = assign79050_body34_e120095_d_n9;
            locals.var_t0_dn10 = assign79050_body34_e120095_d_n10;
            locals.var_t0_dn13 = assign79050_body34_e120095_d_n13;
            let (assign79050_body35_e120112, assign79050_body35_e120112_d_n0, assign79050_body35_e120112_d_n2, assign79050_body35_e120112_d_n4, assign79050_body35_e120112_d_n5, assign79050_body35_e120112_d_n6, assign79050_body35_e120112_d_n7, assign79050_body35_e120112_d_n8, assign79050_body35_e120112_d_n9, assign79050_body35_e120112_d_n10, assign79050_body35_e120112_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body35_e120109: f64 = (-locals.var_chib);
        let assign79050_body35_e120110: f64 = (assign79050_body35_e120109).exp();
        (assign79050_body35_e120110, (assign79050_body35_e120110 * (-locals.var_chib_dn0)), (assign79050_body35_e120110 * (-locals.var_chib_dn2)), (assign79050_body35_e120110 * (-locals.var_chib_dn4)), (assign79050_body35_e120110 * (-locals.var_chib_dn5)), (assign79050_body35_e120110 * (-locals.var_chib_dn6)), (assign79050_body35_e120110 * (-locals.var_chib_dn7)), (assign79050_body35_e120110 * (-locals.var_chib_dn8)), (assign79050_body35_e120110 * (-locals.var_chib_dn9)), (assign79050_body35_e120110 * (-locals.var_chib_dn10)), (assign79050_body35_e120110 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body35_e120112;
            locals.var_t1_dn0 = assign79050_body35_e120112_d_n0;
            locals.var_t1_dn2 = assign79050_body35_e120112_d_n2;
            locals.var_t1_dn4 = assign79050_body35_e120112_d_n4;
            locals.var_t1_dn5 = assign79050_body35_e120112_d_n5;
            locals.var_t1_dn6 = assign79050_body35_e120112_d_n6;
            locals.var_t1_dn7 = assign79050_body35_e120112_d_n7;
            locals.var_t1_dn8 = assign79050_body35_e120112_d_n8;
            locals.var_t1_dn9 = assign79050_body35_e120112_d_n9;
            locals.var_t1_dn10 = assign79050_body35_e120112_d_n10;
            locals.var_t1_dn13 = assign79050_body35_e120112_d_n13;
            let (assign79050_body36_e120133, assign79050_body36_e120133_d_n0, assign79050_body36_e120133_d_n2, assign79050_body36_e120133_d_n4, assign79050_body36_e120133_d_n5, assign79050_body36_e120133_d_n6, assign79050_body36_e120133_d_n7, assign79050_body36_e120133_d_n8, assign79050_body36_e120133_d_n9, assign79050_body36_e120133_d_n10, assign79050_body36_e120133_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) {
        let assign79050_body36_e120127: f64 = (locals.var_chi - locals.var_chib);
        let assign79050_body36_e120130: f64 = (locals.var_t0 - locals.var_t1);
        let assign79050_body36_e120131: f64 = (assign79050_body36_e120127 + assign79050_body36_e120130);
        (assign79050_body36_e120131, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign79050_body36_e120133;
            locals.var_t4_dn0 = assign79050_body36_e120133_d_n0;
            locals.var_t4_dn2 = assign79050_body36_e120133_d_n2;
            locals.var_t4_dn4 = assign79050_body36_e120133_d_n4;
            locals.var_t4_dn5 = assign79050_body36_e120133_d_n5;
            locals.var_t4_dn6 = assign79050_body36_e120133_d_n6;
            locals.var_t4_dn7 = assign79050_body36_e120133_d_n7;
            locals.var_t4_dn8 = assign79050_body36_e120133_d_n8;
            locals.var_t4_dn9 = assign79050_body36_e120133_d_n9;
            locals.var_t4_dn10 = assign79050_body36_e120133_d_n10;
            locals.var_t4_dn13 = assign79050_body36_e120133_d_n13;
            let assign79050_body37_e120136: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1834 = assign79050_body37_e120136;
            let (assign79050_body38_e120154, assign79050_body38_e120154_d_n0, assign79050_body38_e120154_d_n2, assign79050_body38_e120154_d_n4, assign79050_body38_e120154_d_n5, assign79050_body38_e120154_d_n6, assign79050_body38_e120154_d_n7, assign79050_body38_e120154_d_n8, assign79050_body38_e120154_d_n9, assign79050_body38_e120154_d_n10, assign79050_body38_e120154_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79050_body38_e120152: f64 = (locals.var_t4).sqrt();
        (assign79050_body38_e120152, (locals.var_t4_dn0 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn2 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn4 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn5 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn6 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn7 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn8 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn9 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn10 / (2.0 * assign79050_body38_e120152)), (locals.var_t4_dn13 / (2.0 * assign79050_body38_e120152)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body38_e120154;
            locals.var_fb_dn0 = assign79050_body38_e120154_d_n0;
            locals.var_fb_dn2 = assign79050_body38_e120154_d_n2;
            locals.var_fb_dn4 = assign79050_body38_e120154_d_n4;
            locals.var_fb_dn5 = assign79050_body38_e120154_d_n5;
            locals.var_fb_dn6 = assign79050_body38_e120154_d_n6;
            locals.var_fb_dn7 = assign79050_body38_e120154_d_n7;
            locals.var_fb_dn8 = assign79050_body38_e120154_d_n8;
            locals.var_fb_dn9 = assign79050_body38_e120154_d_n9;
            locals.var_fb_dn10 = assign79050_body38_e120154_d_n10;
            locals.var_fb_dn13 = assign79050_body38_e120154_d_n13;
            let (assign79050_body39_e120185, assign79050_body39_e120185_d_n0, assign79050_body39_e120185_d_n2, assign79050_body39_e120185_d_n4, assign79050_body39_e120185_d_n5, assign79050_body39_e120185_d_n6, assign79050_body39_e120185_d_n7, assign79050_body39_e120185_d_n8, assign79050_body39_e120185_d_n9, assign79050_body39_e120185_d_n10, assign79050_body39_e120185_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79050_body39_e120171: f64 = (locals.var_beta * 0.5);
        let assign79050_body39_e120174: f64 = (1.0 - locals.var_t0);
        let assign79050_body39_e120178: f64 = (1.0 - locals.var_t1);
        let assign79050_body39_e120179: f64 = (locals.var_phi_b_dpss * assign79050_body39_e120178);
        let assign79050_body39_e120180: f64 = (assign79050_body39_e120174 - assign79050_body39_e120179);
        let assign79050_body39_e120181: f64 = (assign79050_body39_e120171 * assign79050_body39_e120180);
        let assign79050_body39_e120183: f64 = (assign79050_body39_e120181 / locals.var_fb);
        (assign79050_body39_e120183, ((((((locals.var_beta_dn0 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign79050_body39_e120180) + (assign79050_body39_e120171 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign79050_body39_e120178) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign79050_body39_e120181 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body39_e120185;
            locals.var_fb_dpss_dn0 = assign79050_body39_e120185_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body39_e120185_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body39_e120185_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body39_e120185_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body39_e120185_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body39_e120185_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body39_e120185_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body39_e120185_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body39_e120185_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body39_e120185_d_n13;
            let (assign79050_body41_e120223, assign79050_body41_e120223_d_n0, assign79050_body41_e120223_d_n2, assign79050_body41_e120223_d_n4, assign79050_body41_e120223_d_n5, assign79050_body41_e120223_d_n6, assign79050_body41_e120223_d_n7, assign79050_body41_e120223_d_n8, assign79050_body41_e120223_d_n9, assign79050_body41_e120223_d_n10, assign79050_body41_e120223_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign79050_body41_e120223;
            locals.var_fb_dn0 = assign79050_body41_e120223_d_n0;
            locals.var_fb_dn2 = assign79050_body41_e120223_d_n2;
            locals.var_fb_dn4 = assign79050_body41_e120223_d_n4;
            locals.var_fb_dn5 = assign79050_body41_e120223_d_n5;
            locals.var_fb_dn6 = assign79050_body41_e120223_d_n6;
            locals.var_fb_dn7 = assign79050_body41_e120223_d_n7;
            locals.var_fb_dn8 = assign79050_body41_e120223_d_n8;
            locals.var_fb_dn9 = assign79050_body41_e120223_d_n9;
            locals.var_fb_dn10 = assign79050_body41_e120223_d_n10;
            locals.var_fb_dn13 = assign79050_body41_e120223_d_n13;
            let (assign79050_body42_e120241, assign79050_body42_e120241_d_n0, assign79050_body42_e120241_d_n2, assign79050_body42_e120241_d_n4, assign79050_body42_e120241_d_n5, assign79050_body42_e120241_d_n6, assign79050_body42_e120241_d_n7, assign79050_body42_e120241_d_n8, assign79050_body42_e120241_d_n9, assign79050_body42_e120241_d_n10, assign79050_body42_e120241_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1831 == 0.0)) && (locals.var_guard1832 == 0.0)) && (locals.var_guard1834 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign79050_body42_e120241;
            locals.var_fb_dpss_dn0 = assign79050_body42_e120241_d_n0;
            locals.var_fb_dpss_dn2 = assign79050_body42_e120241_d_n2;
            locals.var_fb_dpss_dn4 = assign79050_body42_e120241_d_n4;
            locals.var_fb_dpss_dn5 = assign79050_body42_e120241_d_n5;
            locals.var_fb_dpss_dn6 = assign79050_body42_e120241_d_n6;
            locals.var_fb_dpss_dn7 = assign79050_body42_e120241_d_n7;
            locals.var_fb_dpss_dn8 = assign79050_body42_e120241_d_n8;
            locals.var_fb_dpss_dn9 = assign79050_body42_e120241_d_n9;
            locals.var_fb_dpss_dn10 = assign79050_body42_e120241_d_n10;
            locals.var_fb_dpss_dn13 = assign79050_body42_e120241_d_n13;
            let assign79050_body43_e120244: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1835 = assign79050_body43_e120244;
            let (assign79050_body45_e120268, assign79050_body45_e120268_d_n0, assign79050_body45_e120268_d_n2, assign79050_body45_e120268_d_n4, assign79050_body45_e120268_d_n5, assign79050_body45_e120268_d_n6, assign79050_body45_e120268_d_n7, assign79050_body45_e120268_d_n8, assign79050_body45_e120268_d_n9, assign79050_body45_e120268_d_n10, assign79050_body45_e120268_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body45_e120268;
            locals.var_fs01_dn0 = assign79050_body45_e120268_d_n0;
            locals.var_fs01_dn2 = assign79050_body45_e120268_d_n2;
            locals.var_fs01_dn4 = assign79050_body45_e120268_d_n4;
            locals.var_fs01_dn5 = assign79050_body45_e120268_d_n5;
            locals.var_fs01_dn6 = assign79050_body45_e120268_d_n6;
            locals.var_fs01_dn7 = assign79050_body45_e120268_d_n7;
            locals.var_fs01_dn8 = assign79050_body45_e120268_d_n8;
            locals.var_fs01_dn9 = assign79050_body45_e120268_d_n9;
            locals.var_fs01_dn10 = assign79050_body45_e120268_d_n10;
            locals.var_fs01_dn13 = assign79050_body45_e120268_d_n13;
            let (assign79050_body46_e120279, assign79050_body46_e120279_d_n0, assign79050_body46_e120279_d_n2, assign79050_body46_e120279_d_n4, assign79050_body46_e120279_d_n5, assign79050_body46_e120279_d_n6, assign79050_body46_e120279_d_n7, assign79050_body46_e120279_d_n8, assign79050_body46_e120279_d_n9, assign79050_body46_e120279_d_n10, assign79050_body46_e120279_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body46_e120279;
            locals.var_fs01_dps0_dn0 = assign79050_body46_e120279_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body46_e120279_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body46_e120279_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body46_e120279_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body46_e120279_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body46_e120279_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body46_e120279_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body46_e120279_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body46_e120279_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body46_e120279_d_n13;
            let (assign79050_body47_e120291, assign79050_body47_e120291_d_n0, assign79050_body47_e120291_d_n2, assign79050_body47_e120291_d_n4, assign79050_body47_e120291_d_n5, assign79050_body47_e120291_d_n6, assign79050_body47_e120291_d_n7, assign79050_body47_e120291_d_n8, assign79050_body47_e120291_d_n9, assign79050_body47_e120291_d_n10, assign79050_body47_e120291_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79050_body47_e120289: f64 = (-locals.var_fb);
        (assign79050_body47_e120289, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body47_e120291;
            locals.var_fs02_dn0 = assign79050_body47_e120291_d_n0;
            locals.var_fs02_dn2 = assign79050_body47_e120291_d_n2;
            locals.var_fs02_dn4 = assign79050_body47_e120291_d_n4;
            locals.var_fs02_dn5 = assign79050_body47_e120291_d_n5;
            locals.var_fs02_dn6 = assign79050_body47_e120291_d_n6;
            locals.var_fs02_dn7 = assign79050_body47_e120291_d_n7;
            locals.var_fs02_dn8 = assign79050_body47_e120291_d_n8;
            locals.var_fs02_dn9 = assign79050_body47_e120291_d_n9;
            locals.var_fs02_dn10 = assign79050_body47_e120291_d_n10;
            locals.var_fs02_dn13 = assign79050_body47_e120291_d_n13;
            let (assign79050_body48_e120303, assign79050_body48_e120303_d_n0, assign79050_body48_e120303_d_n2, assign79050_body48_e120303_d_n4, assign79050_body48_e120303_d_n5, assign79050_body48_e120303_d_n6, assign79050_body48_e120303_d_n7, assign79050_body48_e120303_d_n8, assign79050_body48_e120303_d_n9, assign79050_body48_e120303_d_n10, assign79050_body48_e120303_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79050_body48_e120301: f64 = (-locals.var_fb_dpss);
        (assign79050_body48_e120301, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body48_e120303;
            locals.var_fs02_dps0_dn0 = assign79050_body48_e120303_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body48_e120303_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body48_e120303_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body48_e120303_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body48_e120303_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body48_e120303_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body48_e120303_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body48_e120303_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body48_e120303_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body48_e120303_d_n13;
            let assign79050_body49_e120306: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1836 = assign79050_body49_e120306;
            let assign79050_body50_e120309: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1837 = assign79050_body50_e120309;
            let (assign79050_body51_e120347, assign79050_body51_e120347_d_n0, assign79050_body51_e120347_d_n2, assign79050_body51_e120347_d_n4, assign79050_body51_e120347_d_n5, assign79050_body51_e120347_d_n6, assign79050_body51_e120347_d_n7, assign79050_body51_e120347_d_n8, assign79050_body51_e120347_d_n9, assign79050_body51_e120347_d_n10, assign79050_body51_e120347_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body51_e120325: f64 = (locals.var_chi * locals.var_chi);
        let assign79050_body51_e120327: f64 = (assign79050_body51_e120325 / 2.0);
        let assign79050_body51_e120331: f64 = (locals.var_chi / 3.0);
        let assign79050_body51_e120335: f64 = (locals.var_chi / 4.0);
        let assign79050_body51_e120339: f64 = (locals.var_chi / 5.0);
        let assign79050_body51_e120340: f64 = (1.0 + assign79050_body51_e120339);
        let assign79050_body51_e120341: f64 = (assign79050_body51_e120335 * assign79050_body51_e120340);
        let assign79050_body51_e120342: f64 = (1.0 + assign79050_body51_e120341);
        let assign79050_body51_e120343: f64 = (assign79050_body51_e120331 * assign79050_body51_e120342);
        let assign79050_body51_e120344: f64 = (1.0 + assign79050_body51_e120343);
        let assign79050_body51_e120345: f64 = (assign79050_body51_e120327 * assign79050_body51_e120344);
        (assign79050_body51_e120345, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn0 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn0 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn2 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn2 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn4 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn4 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn5 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn5 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn6 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn6 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn7 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn7 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn8 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn8 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn9 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn9 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn10 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn10 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79050_body51_e120344) + (assign79050_body51_e120327 * (((locals.var_chi_dn13 / 3.0) * assign79050_body51_e120342) + (assign79050_body51_e120331 * (((locals.var_chi_dn13 / 4.0) * assign79050_body51_e120340) + (assign79050_body51_e120335 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79050_body51_e120347;
            locals.var_t0_dn0 = assign79050_body51_e120347_d_n0;
            locals.var_t0_dn2 = assign79050_body51_e120347_d_n2;
            locals.var_t0_dn4 = assign79050_body51_e120347_d_n4;
            locals.var_t0_dn5 = assign79050_body51_e120347_d_n5;
            locals.var_t0_dn6 = assign79050_body51_e120347_d_n6;
            locals.var_t0_dn7 = assign79050_body51_e120347_d_n7;
            locals.var_t0_dn8 = assign79050_body51_e120347_d_n8;
            locals.var_t0_dn9 = assign79050_body51_e120347_d_n9;
            locals.var_t0_dn10 = assign79050_body51_e120347_d_n10;
            locals.var_t0_dn13 = assign79050_body51_e120347_d_n13;
            let (assign79050_body52_e120381, assign79050_body52_e120381_d_n0, assign79050_body52_e120381_d_n2, assign79050_body52_e120381_d_n4, assign79050_body52_e120381_d_n5, assign79050_body52_e120381_d_n6, assign79050_body52_e120381_d_n7, assign79050_body52_e120381_d_n8, assign79050_body52_e120381_d_n9, assign79050_body52_e120381_d_n10, assign79050_body52_e120381_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body52_e120365: f64 = (locals.var_chi / 2.0);
        let assign79050_body52_e120369: f64 = (locals.var_chi / 3.0);
        let assign79050_body52_e120373: f64 = (locals.var_chi / 4.0);
        let assign79050_body52_e120374: f64 = (1.0 + assign79050_body52_e120373);
        let assign79050_body52_e120375: f64 = (assign79050_body52_e120369 * assign79050_body52_e120374);
        let assign79050_body52_e120376: f64 = (1.0 + assign79050_body52_e120375);
        let assign79050_body52_e120377: f64 = (assign79050_body52_e120365 * assign79050_body52_e120376);
        let assign79050_body52_e120378: f64 = (1.0 + assign79050_body52_e120377);
        let assign79050_body52_e120379: f64 = (locals.var_chi * assign79050_body52_e120378);
        (assign79050_body52_e120379, ((locals.var_chi_dn0 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn0 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn2 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn4 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn5 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn6 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn7 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn8 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn9 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn10 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign79050_body52_e120378) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign79050_body52_e120376) + (assign79050_body52_e120365 * (((locals.var_chi_dn13 / 3.0) * assign79050_body52_e120374) + (assign79050_body52_e120369 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body52_e120381;
            locals.var_t1_dn0 = assign79050_body52_e120381_d_n0;
            locals.var_t1_dn2 = assign79050_body52_e120381_d_n2;
            locals.var_t1_dn4 = assign79050_body52_e120381_d_n4;
            locals.var_t1_dn5 = assign79050_body52_e120381_d_n5;
            locals.var_t1_dn6 = assign79050_body52_e120381_d_n6;
            locals.var_t1_dn7 = assign79050_body52_e120381_d_n7;
            locals.var_t1_dn8 = assign79050_body52_e120381_d_n8;
            locals.var_t1_dn9 = assign79050_body52_e120381_d_n9;
            locals.var_t1_dn10 = assign79050_body52_e120381_d_n10;
            locals.var_t1_dn13 = assign79050_body52_e120381_d_n13;
            let (assign79050_body53_e120399, assign79050_body53_e120399_d_n0, assign79050_body53_e120399_d_n2, assign79050_body53_e120399_d_n4, assign79050_body53_e120399_d_n5, assign79050_body53_e120399_d_n6, assign79050_body53_e120399_d_n7, assign79050_body53_e120399_d_n8, assign79050_body53_e120399_d_n9, assign79050_body53_e120399_d_n10, assign79050_body53_e120399_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body53_e120397: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign79050_body53_e120397, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body53_e120399;
            locals.var_fs01_dn0 = assign79050_body53_e120399_d_n0;
            locals.var_fs01_dn2 = assign79050_body53_e120399_d_n2;
            locals.var_fs01_dn4 = assign79050_body53_e120399_d_n4;
            locals.var_fs01_dn5 = assign79050_body53_e120399_d_n5;
            locals.var_fs01_dn6 = assign79050_body53_e120399_d_n6;
            locals.var_fs01_dn7 = assign79050_body53_e120399_d_n7;
            locals.var_fs01_dn8 = assign79050_body53_e120399_d_n8;
            locals.var_fs01_dn9 = assign79050_body53_e120399_d_n9;
            locals.var_fs01_dn10 = assign79050_body53_e120399_d_n10;
            locals.var_fs01_dn13 = assign79050_body53_e120399_d_n13;
            let (assign79050_body54_e120419, assign79050_body54_e120419_d_n0, assign79050_body54_e120419_d_n2, assign79050_body54_e120419_d_n4, assign79050_body54_e120419_d_n5, assign79050_body54_e120419_d_n6, assign79050_body54_e120419_d_n7, assign79050_body54_e120419_d_n8, assign79050_body54_e120419_d_n9, assign79050_body54_e120419_d_n10, assign79050_body54_e120419_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79050_body54_e120415: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign79050_body54_e120417: f64 = (assign79050_body54_e120415 * locals.var_beta);
        (assign79050_body54_e120417, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign79050_body54_e120415 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body54_e120419;
            locals.var_fs01_dps0_dn0 = assign79050_body54_e120419_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body54_e120419_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body54_e120419_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body54_e120419_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body54_e120419_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body54_e120419_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body54_e120419_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body54_e120419_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body54_e120419_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body54_e120419_d_n13;
            let (assign79050_body55_e120437, assign79050_body55_e120437_d_n0, assign79050_body55_e120437_d_n2, assign79050_body55_e120437_d_n4, assign79050_body55_e120437_d_n5, assign79050_body55_e120437_d_n6, assign79050_body55_e120437_d_n7, assign79050_body55_e120437_d_n8, assign79050_body55_e120437_d_n9, assign79050_body55_e120437_d_n10, assign79050_body55_e120437_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body55_e120435: f64 = (locals.var_chi).exp();
        (assign79050_body55_e120435, (assign79050_body55_e120435 * locals.var_chi_dn0), (assign79050_body55_e120435 * locals.var_chi_dn2), (assign79050_body55_e120435 * locals.var_chi_dn4), (assign79050_body55_e120435 * locals.var_chi_dn5), (assign79050_body55_e120435 * locals.var_chi_dn6), (assign79050_body55_e120435 * locals.var_chi_dn7), (assign79050_body55_e120435 * locals.var_chi_dn8), (assign79050_body55_e120435 * locals.var_chi_dn9), (assign79050_body55_e120435 * locals.var_chi_dn10), (assign79050_body55_e120435 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign79050_body55_e120437;
            locals.var_exp_chi_dn0 = assign79050_body55_e120437_d_n0;
            locals.var_exp_chi_dn2 = assign79050_body55_e120437_d_n2;
            locals.var_exp_chi_dn4 = assign79050_body55_e120437_d_n4;
            locals.var_exp_chi_dn5 = assign79050_body55_e120437_d_n5;
            locals.var_exp_chi_dn6 = assign79050_body55_e120437_d_n6;
            locals.var_exp_chi_dn7 = assign79050_body55_e120437_d_n7;
            locals.var_exp_chi_dn8 = assign79050_body55_e120437_d_n8;
            locals.var_exp_chi_dn9 = assign79050_body55_e120437_d_n9;
            locals.var_exp_chi_dn10 = assign79050_body55_e120437_d_n10;
            locals.var_exp_chi_dn13 = assign79050_body55_e120437_d_n13;
            let (assign79050_body56_e120456, assign79050_body56_e120456_d_n0, assign79050_body56_e120456_d_n2, assign79050_body56_e120456_d_n4, assign79050_body56_e120456_d_n5, assign79050_body56_e120456_d_n6, assign79050_body56_e120456_d_n7, assign79050_body56_e120456_d_n8, assign79050_body56_e120456_d_n9, assign79050_body56_e120456_d_n10, assign79050_body56_e120456_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body56_e120454: f64 = (locals.var_exp_chi - 1.0);
        (assign79050_body56_e120454, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79050_body56_e120456;
            locals.var_t1_dn0 = assign79050_body56_e120456_d_n0;
            locals.var_t1_dn2 = assign79050_body56_e120456_d_n2;
            locals.var_t1_dn4 = assign79050_body56_e120456_d_n4;
            locals.var_t1_dn5 = assign79050_body56_e120456_d_n5;
            locals.var_t1_dn6 = assign79050_body56_e120456_d_n6;
            locals.var_t1_dn7 = assign79050_body56_e120456_d_n7;
            locals.var_t1_dn8 = assign79050_body56_e120456_d_n8;
            locals.var_t1_dn9 = assign79050_body56_e120456_d_n9;
            locals.var_t1_dn10 = assign79050_body56_e120456_d_n10;
            locals.var_t1_dn13 = assign79050_body56_e120456_d_n13;
            let (assign79050_body57_e120477, assign79050_body57_e120477_d_n0, assign79050_body57_e120477_d_n2, assign79050_body57_e120477_d_n4, assign79050_body57_e120477_d_n5, assign79050_body57_e120477_d_n6, assign79050_body57_e120477_d_n7, assign79050_body57_e120477_d_n8, assign79050_body57_e120477_d_n9, assign79050_body57_e120477_d_n10, assign79050_body57_e120477_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body57_e120474: f64 = (locals.var_t1 - locals.var_chi);
        let assign79050_body57_e120475: f64 = (locals.var_cfs1 * assign79050_body57_e120474);
        (assign79050_body57_e120475, ((locals.var_cfs1_dn0 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign79050_body57_e120474) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body57_e120477;
            locals.var_fs01_dn0 = assign79050_body57_e120477_d_n0;
            locals.var_fs01_dn2 = assign79050_body57_e120477_d_n2;
            locals.var_fs01_dn4 = assign79050_body57_e120477_d_n4;
            locals.var_fs01_dn5 = assign79050_body57_e120477_d_n5;
            locals.var_fs01_dn6 = assign79050_body57_e120477_d_n6;
            locals.var_fs01_dn7 = assign79050_body57_e120477_d_n7;
            locals.var_fs01_dn8 = assign79050_body57_e120477_d_n8;
            locals.var_fs01_dn9 = assign79050_body57_e120477_d_n9;
            locals.var_fs01_dn10 = assign79050_body57_e120477_d_n10;
            locals.var_fs01_dn13 = assign79050_body57_e120477_d_n13;
            let (assign79050_body58_e120498, assign79050_body58_e120498_d_n0, assign79050_body58_e120498_d_n2, assign79050_body58_e120498_d_n4, assign79050_body58_e120498_d_n5, assign79050_body58_e120498_d_n6, assign79050_body58_e120498_d_n7, assign79050_body58_e120498_d_n8, assign79050_body58_e120498_d_n9, assign79050_body58_e120498_d_n10, assign79050_body58_e120498_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        let assign79050_body58_e120494: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign79050_body58_e120496: f64 = (assign79050_body58_e120494 * locals.var_t1);
        (assign79050_body58_e120496, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign79050_body58_e120494 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body58_e120498;
            locals.var_fs01_dps0_dn0 = assign79050_body58_e120498_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body58_e120498_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body58_e120498_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body58_e120498_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body58_e120498_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body58_e120498_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body58_e120498_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body58_e120498_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body58_e120498_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body58_e120498_d_n13;
            let (assign79050_body60_e120533, assign79050_body60_e120533_d_n0, assign79050_body60_e120533_d_n2, assign79050_body60_e120533_d_n4, assign79050_body60_e120533_d_n5, assign79050_body60_e120533_d_n6, assign79050_body60_e120533_d_n7, assign79050_body60_e120533_d_n8, assign79050_body60_e120533_d_n9, assign79050_body60_e120533_d_n10, assign79050_body60_e120533_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body60_e120530: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign79050_body60_e120531: f64 = (assign79050_body60_e120530).exp();
        (assign79050_body60_e120531, (assign79050_body60_e120531 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign79050_body60_e120531 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign79050_body60_e120531 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign79050_body60_e120531 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign79050_body60_e120531 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign79050_body60_e120531 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign79050_body60_e120531 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign79050_body60_e120531 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign79050_body60_e120531 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign79050_body60_e120531 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign79050_body60_e120533;
            locals.var_exp_bps0_dn0 = assign79050_body60_e120533_d_n0;
            locals.var_exp_bps0_dn2 = assign79050_body60_e120533_d_n2;
            locals.var_exp_bps0_dn4 = assign79050_body60_e120533_d_n4;
            locals.var_exp_bps0_dn5 = assign79050_body60_e120533_d_n5;
            locals.var_exp_bps0_dn6 = assign79050_body60_e120533_d_n6;
            locals.var_exp_bps0_dn7 = assign79050_body60_e120533_d_n7;
            locals.var_exp_bps0_dn8 = assign79050_body60_e120533_d_n8;
            locals.var_exp_bps0_dn9 = assign79050_body60_e120533_d_n9;
            locals.var_exp_bps0_dn10 = assign79050_body60_e120533_d_n10;
            locals.var_exp_bps0_dn13 = assign79050_body60_e120533_d_n13;
            let (assign79050_body61_e120556, assign79050_body61_e120556_d_n0, assign79050_body61_e120556_d_n2, assign79050_body61_e120556_d_n4, assign79050_body61_e120556_d_n5, assign79050_body61_e120556_d_n6, assign79050_body61_e120556_d_n7, assign79050_body61_e120556_d_n8, assign79050_body61_e120556_d_n9, assign79050_body61_e120556_d_n10, assign79050_body61_e120556_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body61_e120551: f64 = (locals.var_chi + 1.0);
        let assign79050_body61_e120552: f64 = (locals.var_exp_bvbs * assign79050_body61_e120551);
        let assign79050_body61_e120553: f64 = (locals.var_exp_bps0 - assign79050_body61_e120552);
        let assign79050_body61_e120554: f64 = (locals.var_cnst1over * assign79050_body61_e120553);
        (assign79050_body61_e120554, ((locals.var_cnst1over_dn0 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign79050_body61_e120553) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign79050_body61_e120551) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79050_body61_e120556;
            locals.var_fs01_dn0 = assign79050_body61_e120556_d_n0;
            locals.var_fs01_dn2 = assign79050_body61_e120556_d_n2;
            locals.var_fs01_dn4 = assign79050_body61_e120556_d_n4;
            locals.var_fs01_dn5 = assign79050_body61_e120556_d_n5;
            locals.var_fs01_dn6 = assign79050_body61_e120556_d_n6;
            locals.var_fs01_dn7 = assign79050_body61_e120556_d_n7;
            locals.var_fs01_dn8 = assign79050_body61_e120556_d_n8;
            locals.var_fs01_dn9 = assign79050_body61_e120556_d_n9;
            locals.var_fs01_dn10 = assign79050_body61_e120556_d_n10;
            locals.var_fs01_dn13 = assign79050_body61_e120556_d_n13;
            let (assign79050_body62_e120577, assign79050_body62_e120577_d_n0, assign79050_body62_e120577_d_n2, assign79050_body62_e120577_d_n4, assign79050_body62_e120577_d_n5, assign79050_body62_e120577_d_n6, assign79050_body62_e120577_d_n7, assign79050_body62_e120577_d_n8, assign79050_body62_e120577_d_n9, assign79050_body62_e120577_d_n10, assign79050_body62_e120577_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79050_body62_e120571: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign79050_body62_e120574: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign79050_body62_e120575: f64 = (assign79050_body62_e120571 * assign79050_body62_e120574);
        (assign79050_body62_e120575, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign79050_body62_e120574) + (assign79050_body62_e120571 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79050_body62_e120577;
            locals.var_fs01_dps0_dn0 = assign79050_body62_e120577_d_n0;
            locals.var_fs01_dps0_dn2 = assign79050_body62_e120577_d_n2;
            locals.var_fs01_dps0_dn4 = assign79050_body62_e120577_d_n4;
            locals.var_fs01_dps0_dn5 = assign79050_body62_e120577_d_n5;
            locals.var_fs01_dps0_dn6 = assign79050_body62_e120577_d_n6;
            locals.var_fs01_dps0_dn7 = assign79050_body62_e120577_d_n7;
            locals.var_fs01_dps0_dn8 = assign79050_body62_e120577_d_n8;
            locals.var_fs01_dps0_dn9 = assign79050_body62_e120577_d_n9;
            locals.var_fs01_dps0_dn10 = assign79050_body62_e120577_d_n10;
            locals.var_fs01_dps0_dn13 = assign79050_body62_e120577_d_n13;
            let assign79050_body63_e120580: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1838 = assign79050_body63_e120580;
            let (assign79050_body64_e120599, assign79050_body64_e120599_d_n0, assign79050_body64_e120599_d_n2, assign79050_body64_e120599_d_n4, assign79050_body64_e120599_d_n5, assign79050_body64_e120599_d_n6, assign79050_body64_e120599_d_n7, assign79050_body64_e120599_d_n8, assign79050_body64_e120599_d_n9, assign79050_body64_e120599_d_n10, assign79050_body64_e120599_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79050_body64_e120594: f64 = (locals.var_fb * locals.var_fb);
        let assign79050_body64_e120596: f64 = (assign79050_body64_e120594 + locals.var_fs01);
        let assign79050_body64_e120597: f64 = (assign79050_body64_e120596).sqrt();
        (assign79050_body64_e120597, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign79050_body64_e120597)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign79050_body64_e120597)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body64_e120599;
            locals.var_fs02_dn0 = assign79050_body64_e120599_d_n0;
            locals.var_fs02_dn2 = assign79050_body64_e120599_d_n2;
            locals.var_fs02_dn4 = assign79050_body64_e120599_d_n4;
            locals.var_fs02_dn5 = assign79050_body64_e120599_d_n5;
            locals.var_fs02_dn6 = assign79050_body64_e120599_d_n6;
            locals.var_fs02_dn7 = assign79050_body64_e120599_d_n7;
            locals.var_fs02_dn8 = assign79050_body64_e120599_d_n8;
            locals.var_fs02_dn9 = assign79050_body64_e120599_d_n9;
            locals.var_fs02_dn10 = assign79050_body64_e120599_d_n10;
            locals.var_fs02_dn13 = assign79050_body64_e120599_d_n13;
            let (assign79050_body65_e120623, assign79050_body65_e120623_d_n0, assign79050_body65_e120623_d_n2, assign79050_body65_e120623_d_n4, assign79050_body65_e120623_d_n5, assign79050_body65_e120623_d_n6, assign79050_body65_e120623_d_n7, assign79050_body65_e120623_d_n8, assign79050_body65_e120623_d_n9, assign79050_body65_e120623_d_n10, assign79050_body65_e120623_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79050_body65_e120614: f64 = (2.0 * locals.var_fb_dpss);
        let assign79050_body65_e120616: f64 = (assign79050_body65_e120614 * locals.var_fb);
        let assign79050_body65_e120618: f64 = (assign79050_body65_e120616 + locals.var_fs01_dps0);
        let assign79050_body65_e120619: f64 = (0.5 * assign79050_body65_e120618);
        let assign79050_body65_e120621: f64 = (assign79050_body65_e120619 / locals.var_fs02);
        (assign79050_body65_e120621, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign79050_body65_e120614 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign79050_body65_e120619 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body65_e120623;
            locals.var_fs02_dps0_dn0 = assign79050_body65_e120623_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body65_e120623_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body65_e120623_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body65_e120623_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body65_e120623_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body65_e120623_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body65_e120623_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body65_e120623_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body65_e120623_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body65_e120623_d_n13;
            let (assign79050_body67_e120655, assign79050_body67_e120655_d_n0, assign79050_body67_e120655_d_n2, assign79050_body67_e120655_d_n4, assign79050_body67_e120655_d_n5, assign79050_body67_e120655_d_n6, assign79050_body67_e120655_d_n7, assign79050_body67_e120655_d_n8, assign79050_body67_e120655_d_n9, assign79050_body67_e120655_d_n10, assign79050_body67_e120655_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79050_body67_e120655;
            locals.var_fs02_dn0 = assign79050_body67_e120655_d_n0;
            locals.var_fs02_dn2 = assign79050_body67_e120655_d_n2;
            locals.var_fs02_dn4 = assign79050_body67_e120655_d_n4;
            locals.var_fs02_dn5 = assign79050_body67_e120655_d_n5;
            locals.var_fs02_dn6 = assign79050_body67_e120655_d_n6;
            locals.var_fs02_dn7 = assign79050_body67_e120655_d_n7;
            locals.var_fs02_dn8 = assign79050_body67_e120655_d_n8;
            locals.var_fs02_dn9 = assign79050_body67_e120655_d_n9;
            locals.var_fs02_dn10 = assign79050_body67_e120655_d_n10;
            locals.var_fs02_dn13 = assign79050_body67_e120655_d_n13;
            let (assign79050_body68_e120670, assign79050_body68_e120670_d_n0, assign79050_body68_e120670_d_n2, assign79050_body68_e120670_d_n4, assign79050_body68_e120670_d_n5, assign79050_body68_e120670_d_n6, assign79050_body68_e120670_d_n7, assign79050_body68_e120670_d_n8, assign79050_body68_e120670_d_n9, assign79050_body68_e120670_d_n10, assign79050_body68_e120670_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79050_body68_e120670;
            locals.var_fs02_dps0_dn0 = assign79050_body68_e120670_d_n0;
            locals.var_fs02_dps0_dn2 = assign79050_body68_e120670_d_n2;
            locals.var_fs02_dps0_dn4 = assign79050_body68_e120670_d_n4;
            locals.var_fs02_dps0_dn5 = assign79050_body68_e120670_d_n5;
            locals.var_fs02_dps0_dn6 = assign79050_body68_e120670_d_n6;
            locals.var_fs02_dps0_dn7 = assign79050_body68_e120670_d_n7;
            locals.var_fs02_dps0_dn8 = assign79050_body68_e120670_d_n8;
            locals.var_fs02_dps0_dn9 = assign79050_body68_e120670_d_n9;
            locals.var_fs02_dps0_dn10 = assign79050_body68_e120670_d_n10;
            locals.var_fs02_dps0_dn13 = assign79050_body68_e120670_d_n13;
            let (assign79050_body69_e120686, assign79050_body69_e120686_d_n0, assign79050_body69_e120686_d_n2, assign79050_body69_e120686_d_n4, assign79050_body69_e120686_d_n5, assign79050_body69_e120686_d_n6, assign79050_body69_e120686_d_n7, assign79050_body69_e120686_d_n8, assign79050_body69_e120686_d_n9, assign79050_body69_e120686_d_n10, assign79050_body69_e120686_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body69_e120678: f64 = (-locals.var_vgpld);
        let assign79050_body69_e120680: f64 = (assign79050_body69_e120678 + locals.var_ps0ld);
        let assign79050_body69_e120683: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign79050_body69_e120684: f64 = (assign79050_body69_e120680 + assign79050_body69_e120683);
        (assign79050_body69_e120684, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign79050_body69_e120686;
            locals.var_fs0_dn0 = assign79050_body69_e120686_d_n0;
            locals.var_fs0_dn2 = assign79050_body69_e120686_d_n2;
            locals.var_fs0_dn4 = assign79050_body69_e120686_d_n4;
            locals.var_fs0_dn5 = assign79050_body69_e120686_d_n5;
            locals.var_fs0_dn6 = assign79050_body69_e120686_d_n6;
            locals.var_fs0_dn7 = assign79050_body69_e120686_d_n7;
            locals.var_fs0_dn8 = assign79050_body69_e120686_d_n8;
            locals.var_fs0_dn9 = assign79050_body69_e120686_d_n9;
            locals.var_fs0_dn10 = assign79050_body69_e120686_d_n10;
            locals.var_fs0_dn13 = assign79050_body69_e120686_d_n13;
            let (assign79050_body70_e120699, assign79050_body70_e120699_d_n0, assign79050_body70_e120699_d_n2, assign79050_body70_e120699_d_n4, assign79050_body70_e120699_d_n5, assign79050_body70_e120699_d_n6, assign79050_body70_e120699_d_n7, assign79050_body70_e120699_d_n8, assign79050_body70_e120699_d_n9, assign79050_body70_e120699_d_n10, assign79050_body70_e120699_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body70_e120696: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign79050_body70_e120697: f64 = (1.0 + assign79050_body70_e120696);
        (assign79050_body70_e120697, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign79050_body70_e120699;
            locals.var_fs0_dps0_dn0 = assign79050_body70_e120699_d_n0;
            locals.var_fs0_dps0_dn2 = assign79050_body70_e120699_d_n2;
            locals.var_fs0_dps0_dn4 = assign79050_body70_e120699_d_n4;
            locals.var_fs0_dps0_dn5 = assign79050_body70_e120699_d_n5;
            locals.var_fs0_dps0_dn6 = assign79050_body70_e120699_d_n6;
            locals.var_fs0_dps0_dn7 = assign79050_body70_e120699_d_n7;
            locals.var_fs0_dps0_dn8 = assign79050_body70_e120699_d_n8;
            locals.var_fs0_dps0_dn9 = assign79050_body70_e120699_d_n9;
            locals.var_fs0_dps0_dn10 = assign79050_body70_e120699_d_n10;
            locals.var_fs0_dps0_dn13 = assign79050_body70_e120699_d_n13;
            let assign79050_body71_e120702: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1839 = assign79050_body71_e120702;
            let (assign79050_body72_e120715,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        let assign79050_body72_e120713: f64 = (locals.var_lp_s0_max + 1.0);
        (assign79050_body72_e120713,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79050_body72_e120715;
            let (assign79050_body73_e120730, assign79050_body73_e120730_d_n0, assign79050_body73_e120730_d_n2, assign79050_body73_e120730_d_n4, assign79050_body73_e120730_d_n5, assign79050_body73_e120730_d_n6, assign79050_body73_e120730_d_n7, assign79050_body73_e120730_d_n8, assign79050_body73_e120730_d_n9, assign79050_body73_e120730_d_n10, assign79050_body73_e120730_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body73_e120726: f64 = (-locals.var_fs0);
        let assign79050_body73_e120728: f64 = (assign79050_body73_e120726 / locals.var_fs0_dps0);
        (assign79050_body73_e120728, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign79050_body73_e120726 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79050_body73_e120730;
            locals.var_dps0_dn0 = assign79050_body73_e120730_d_n0;
            locals.var_dps0_dn2 = assign79050_body73_e120730_d_n2;
            locals.var_dps0_dn4 = assign79050_body73_e120730_d_n4;
            locals.var_dps0_dn5 = assign79050_body73_e120730_d_n5;
            locals.var_dps0_dn6 = assign79050_body73_e120730_d_n6;
            locals.var_dps0_dn7 = assign79050_body73_e120730_d_n7;
            locals.var_dps0_dn8 = assign79050_body73_e120730_d_n8;
            locals.var_dps0_dn9 = assign79050_body73_e120730_d_n9;
            locals.var_dps0_dn10 = assign79050_body73_e120730_d_n10;
            locals.var_dps0_dn13 = assign79050_body73_e120730_d_n13;
            let (assign79050_body74_e120755, assign79050_body74_e120755_d_n0, assign79050_body74_e120755_d_n2, assign79050_body74_e120755_d_n4, assign79050_body74_e120755_d_n5, assign79050_body74_e120755_d_n6, assign79050_body74_e120755_d_n7, assign79050_body74_e120755_d_n8, assign79050_body74_e120755_d_n9, assign79050_body74_e120755_d_n10, assign79050_body74_e120755_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body74_e120742: f64 = (0.5 * 0.1);
        let assign79050_body74_e120746: f64 = (locals.var_ps0ld).abs();
        let (assign79050_body74_e120751, assign79050_body74_e120751_d_n0, assign79050_body74_e120751_d_n2, assign79050_body74_e120751_d_n4, assign79050_body74_e120751_d_n5, assign79050_body74_e120751_d_n6, assign79050_body74_e120751_d_n7, assign79050_body74_e120751_d_n8, assign79050_body74_e120751_d_n9, assign79050_body74_e120751_d_n10, assign79050_body74_e120751_d_n13,) = {
            if (1.0 >= assign79050_body74_e120746) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign79050_body74_e120750: f64 = (locals.var_ps0ld).abs();
                (assign79050_body74_e120750, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign79050_body74_e120752: f64 = (1.0 + assign79050_body74_e120751);
        let assign79050_body74_e120753: f64 = (assign79050_body74_e120742 * assign79050_body74_e120752);
        (assign79050_body74_e120753, (assign79050_body74_e120742 * assign79050_body74_e120751_d_n0), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n2), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n4), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n5), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n6), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n7), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n8), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n9), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n10), (assign79050_body74_e120742 * assign79050_body74_e120751_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign79050_body74_e120755;
            locals.var_dplim_dn0 = assign79050_body74_e120755_d_n0;
            locals.var_dplim_dn2 = assign79050_body74_e120755_d_n2;
            locals.var_dplim_dn4 = assign79050_body74_e120755_d_n4;
            locals.var_dplim_dn5 = assign79050_body74_e120755_d_n5;
            locals.var_dplim_dn6 = assign79050_body74_e120755_d_n6;
            locals.var_dplim_dn7 = assign79050_body74_e120755_d_n7;
            locals.var_dplim_dn8 = assign79050_body74_e120755_d_n8;
            locals.var_dplim_dn9 = assign79050_body74_e120755_d_n9;
            locals.var_dplim_dn10 = assign79050_body74_e120755_d_n10;
            locals.var_dplim_dn13 = assign79050_body74_e120755_d_n13;
            let assign79050_body75_e120757: f64 = (locals.var_dps0).abs();
            let assign79050_body75_e120759: f64 = if assign79050_body75_e120757 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1840 = assign79050_body75_e120759;
            let (assign79050_body76_e120781, assign79050_body76_e120781_d_n0, assign79050_body76_e120781_d_n2, assign79050_body76_e120781_d_n4, assign79050_body76_e120781_d_n5, assign79050_body76_e120781_d_n6, assign79050_body76_e120781_d_n7, assign79050_body76_e120781_d_n8, assign79050_body76_e120781_d_n9, assign79050_body76_e120781_d_n10, assign79050_body76_e120781_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) {
        let (assign79050_body76_e120778,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign79050_body76_e120777: f64 = (-1.0);
                (assign79050_body76_e120777,)
            }
        };
        let assign79050_body76_e120779: f64 = (locals.var_dplim * assign79050_body76_e120778);
        (assign79050_body76_e120779, (locals.var_dplim_dn0 * assign79050_body76_e120778), (locals.var_dplim_dn2 * assign79050_body76_e120778), (locals.var_dplim_dn4 * assign79050_body76_e120778), (locals.var_dplim_dn5 * assign79050_body76_e120778), (locals.var_dplim_dn6 * assign79050_body76_e120778), (locals.var_dplim_dn7 * assign79050_body76_e120778), (locals.var_dplim_dn8 * assign79050_body76_e120778), (locals.var_dplim_dn9 * assign79050_body76_e120778), (locals.var_dplim_dn10 * assign79050_body76_e120778), (locals.var_dplim_dn13 * assign79050_body76_e120778),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79050_body76_e120781;
            locals.var_dps0_dn0 = assign79050_body76_e120781_d_n0;
            locals.var_dps0_dn2 = assign79050_body76_e120781_d_n2;
            locals.var_dps0_dn4 = assign79050_body76_e120781_d_n4;
            locals.var_dps0_dn5 = assign79050_body76_e120781_d_n5;
            locals.var_dps0_dn6 = assign79050_body76_e120781_d_n6;
            locals.var_dps0_dn7 = assign79050_body76_e120781_d_n7;
            locals.var_dps0_dn8 = assign79050_body76_e120781_d_n8;
            locals.var_dps0_dn9 = assign79050_body76_e120781_d_n9;
            locals.var_dps0_dn10 = assign79050_body76_e120781_d_n10;
            locals.var_dps0_dn13 = assign79050_body76_e120781_d_n13;
            let (assign79050_body77_e120795, assign79050_body77_e120795_d_n0, assign79050_body77_e120795_d_n2, assign79050_body77_e120795_d_n4, assign79050_body77_e120795_d_n5, assign79050_body77_e120795_d_n6, assign79050_body77_e120795_d_n7, assign79050_body77_e120795_d_n8, assign79050_body77_e120795_d_n9, assign79050_body77_e120795_d_n10, assign79050_body77_e120795_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) {
        let assign79050_body77_e120793: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign79050_body77_e120793, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign79050_body77_e120795;
            locals.var_ps0ld_dn0 = assign79050_body77_e120795_d_n0;
            locals.var_ps0ld_dn2 = assign79050_body77_e120795_d_n2;
            locals.var_ps0ld_dn4 = assign79050_body77_e120795_d_n4;
            locals.var_ps0ld_dn5 = assign79050_body77_e120795_d_n5;
            locals.var_ps0ld_dn6 = assign79050_body77_e120795_d_n6;
            locals.var_ps0ld_dn7 = assign79050_body77_e120795_d_n7;
            locals.var_ps0ld_dn8 = assign79050_body77_e120795_d_n8;
            locals.var_ps0ld_dn9 = assign79050_body77_e120795_d_n9;
            locals.var_ps0ld_dn10 = assign79050_body77_e120795_d_n10;
            locals.var_ps0ld_dn13 = assign79050_body77_e120795_d_n13;
            let assign79050_body78_e120797: f64 = (locals.var_dps0).abs();
            let assign79050_body78_e120801: f64 = (locals.var_fs0).abs();
            let assign79050_body78_e120804: f64 = if ((assign79050_body78_e120797 <= 1e-12) && (assign79050_body78_e120801 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1841 = assign79050_body78_e120804;
            let (assign79050_body79_e120818,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1841 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign79050_body79_e120818;
            let (assign79050_body80_e120829,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79050_body80_e120827: f64 = (locals.var_lp_s0 + 1.0);
        (assign79050_body80_e120827,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79050_body80_e120829;
        }

    }

    pub(super) fn stamp_transient_block_274(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79070_e120843, assign79070_e120843_d_n0, assign79070_e120843_d_n2, assign79070_e120843_d_n4, assign79070_e120843_d_n5, assign79070_e120843_d_n6, assign79070_e120843_d_n7, assign79070_e120843_d_n8, assign79070_e120843_d_n9, assign79070_e120843_d_n10, assign79070_e120843_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79070_e120841: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79070_e120841, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1760, locals.var_wdld__blk1760_dn0, locals.var_wdld__blk1760_dn2, locals.var_wdld__blk1760_dn4, locals.var_wdld__blk1760_dn5, locals.var_wdld__blk1760_dn6, locals.var_wdld__blk1760_dn7, locals.var_wdld__blk1760_dn8, locals.var_wdld__blk1760_dn9, locals.var_wdld__blk1760_dn10, locals.var_wdld__blk1760_dn13,)
    }
};
        locals.var_wdld__blk1760 = assign79070_e120843;
        locals.var_wdld__blk1760_dn0 = assign79070_e120843_d_n0;
        locals.var_wdld__blk1760_dn2 = assign79070_e120843_d_n2;
        locals.var_wdld__blk1760_dn4 = assign79070_e120843_d_n4;
        locals.var_wdld__blk1760_dn5 = assign79070_e120843_d_n5;
        locals.var_wdld__blk1760_dn6 = assign79070_e120843_d_n6;
        locals.var_wdld__blk1760_dn7 = assign79070_e120843_d_n7;
        locals.var_wdld__blk1760_dn8 = assign79070_e120843_d_n8;
        locals.var_wdld__blk1760_dn9 = assign79070_e120843_d_n9;
        locals.var_wdld__blk1760_dn10 = assign79070_e120843_d_n10;
        locals.var_wdld__blk1760_dn13 = assign79070_e120843_d_n13;

        let (assign79080_e120854, assign79080_e120854_d_n0, assign79080_e120854_d_n2, assign79080_e120854_d_n4, assign79080_e120854_d_n5, assign79080_e120854_d_n6, assign79080_e120854_d_n7, assign79080_e120854_d_n8, assign79080_e120854_d_n9, assign79080_e120854_d_n10, assign79080_e120854_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79080_e120852: f64 = (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760);
        (assign79080_e120852, (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn0), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn2), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn4), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn5), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn6), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn7), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn8), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn9), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn10), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1761, locals.var_q_dep_ld__blk1761_dn0, locals.var_q_dep_ld__blk1761_dn2, locals.var_q_dep_ld__blk1761_dn4, locals.var_q_dep_ld__blk1761_dn5, locals.var_q_dep_ld__blk1761_dn6, locals.var_q_dep_ld__blk1761_dn7, locals.var_q_dep_ld__blk1761_dn8, locals.var_q_dep_ld__blk1761_dn9, locals.var_q_dep_ld__blk1761_dn10, locals.var_q_dep_ld__blk1761_dn13,)
    }
};
        locals.var_q_dep_ld__blk1761 = assign79080_e120854;
        locals.var_q_dep_ld__blk1761_dn0 = assign79080_e120854_d_n0;
        locals.var_q_dep_ld__blk1761_dn2 = assign79080_e120854_d_n2;
        locals.var_q_dep_ld__blk1761_dn4 = assign79080_e120854_d_n4;
        locals.var_q_dep_ld__blk1761_dn5 = assign79080_e120854_d_n5;
        locals.var_q_dep_ld__blk1761_dn6 = assign79080_e120854_d_n6;
        locals.var_q_dep_ld__blk1761_dn7 = assign79080_e120854_d_n7;
        locals.var_q_dep_ld__blk1761_dn8 = assign79080_e120854_d_n8;
        locals.var_q_dep_ld__blk1761_dn9 = assign79080_e120854_d_n9;
        locals.var_q_dep_ld__blk1761_dn10 = assign79080_e120854_d_n10;
        locals.var_q_dep_ld__blk1761_dn13 = assign79080_e120854_d_n13;

        let (assign79090_e120869, assign79090_e120869_d_n0, assign79090_e120869_d_n2, assign79090_e120869_d_n4, assign79090_e120869_d_n5, assign79090_e120869_d_n6, assign79090_e120869_d_n7, assign79090_e120869_d_n8, assign79090_e120869_d_n9, assign79090_e120869_d_n10, assign79090_e120869_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79090_e120863: f64 = (locals.var_q_dep_ld__blk1761 / locals.var_cnst0over_func);
        let assign79090_e120866: f64 = (10.0 * 2.220446049250313e-16);
        let assign79090_e120867: f64 = (assign79090_e120863 + assign79090_e120866);
        (assign79090_e120867, (((locals.var_q_dep_ld__blk1761_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign79090_e120869;
        locals.var_xi0p12_dn0 = assign79090_e120869_d_n0;
        locals.var_xi0p12_dn2 = assign79090_e120869_d_n2;
        locals.var_xi0p12_dn4 = assign79090_e120869_d_n4;
        locals.var_xi0p12_dn5 = assign79090_e120869_d_n5;
        locals.var_xi0p12_dn6 = assign79090_e120869_d_n6;
        locals.var_xi0p12_dn7 = assign79090_e120869_d_n7;
        locals.var_xi0p12_dn8 = assign79090_e120869_d_n8;
        locals.var_xi0p12_dn9 = assign79090_e120869_d_n9;
        locals.var_xi0p12_dn10 = assign79090_e120869_d_n10;
        locals.var_xi0p12_dn13 = assign79090_e120869_d_n13;

        let (assign79100_e120880, assign79100_e120880_d_n0, assign79100_e120880_d_n2, assign79100_e120880_d_n4, assign79100_e120880_d_n5, assign79100_e120880_d_n6, assign79100_e120880_d_n7, assign79100_e120880_d_n8, assign79100_e120880_d_n9, assign79100_e120880_d_n10, assign79100_e120880_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79100_e120878: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79100_e120878, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign79100_e120880;
        locals.var_qbuld_dn0 = assign79100_e120880_d_n0;
        locals.var_qbuld_dn2 = assign79100_e120880_d_n2;
        locals.var_qbuld_dn4 = assign79100_e120880_d_n4;
        locals.var_qbuld_dn5 = assign79100_e120880_d_n5;
        locals.var_qbuld_dn6 = assign79100_e120880_d_n6;
        locals.var_qbuld_dn7 = assign79100_e120880_d_n7;
        locals.var_qbuld_dn8 = assign79100_e120880_d_n8;
        locals.var_qbuld_dn9 = assign79100_e120880_d_n9;
        locals.var_qbuld_dn10 = assign79100_e120880_d_n10;
        locals.var_qbuld_dn13 = assign79100_e120880_d_n13;

        let (assign79110_e120893, assign79110_e120893_d_n0, assign79110_e120893_d_n2, assign79110_e120893_d_n4, assign79110_e120893_d_n5, assign79110_e120893_d_n6, assign79110_e120893_d_n7, assign79110_e120893_d_n8, assign79110_e120893_d_n9, assign79110_e120893_d_n10, assign79110_e120893_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79110_e120890: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79110_e120891: f64 = (1.0 / assign79110_e120890);
        (assign79110_e120891, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79110_e120890 * assign79110_e120890))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign79110_e120890 * assign79110_e120890))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79110_e120893;
        locals.var_t1_dn0 = assign79110_e120893_d_n0;
        locals.var_t1_dn2 = assign79110_e120893_d_n2;
        locals.var_t1_dn4 = assign79110_e120893_d_n4;
        locals.var_t1_dn5 = assign79110_e120893_d_n5;
        locals.var_t1_dn6 = assign79110_e120893_d_n6;
        locals.var_t1_dn7 = assign79110_e120893_d_n7;
        locals.var_t1_dn8 = assign79110_e120893_d_n8;
        locals.var_t1_dn9 = assign79110_e120893_d_n9;
        locals.var_t1_dn10 = assign79110_e120893_d_n10;
        locals.var_t1_dn13 = assign79110_e120893_d_n13;

        let (assign79120_e120906, assign79120_e120906_d_n0, assign79120_e120906_d_n2, assign79120_e120906_d_n4, assign79120_e120906_d_n5, assign79120_e120906_d_n6, assign79120_e120906_d_n7, assign79120_e120906_d_n8, assign79120_e120906_d_n9, assign79120_e120906_d_n10, assign79120_e120906_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79120_e120902: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79120_e120904: f64 = (assign79120_e120902 * locals.var_t1);
        (assign79120_e120904, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign79120_e120902 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79120_e120906;
        locals.var_qiuld_dn0 = assign79120_e120906_d_n0;
        locals.var_qiuld_dn2 = assign79120_e120906_d_n2;
        locals.var_qiuld_dn4 = assign79120_e120906_d_n4;
        locals.var_qiuld_dn5 = assign79120_e120906_d_n5;
        locals.var_qiuld_dn6 = assign79120_e120906_d_n6;
        locals.var_qiuld_dn7 = assign79120_e120906_d_n7;
        locals.var_qiuld_dn8 = assign79120_e120906_d_n8;
        locals.var_qiuld_dn9 = assign79120_e120906_d_n9;
        locals.var_qiuld_dn10 = assign79120_e120906_d_n10;
        locals.var_qiuld_dn13 = assign79120_e120906_d_n13;

        let (assign79130_e120917, assign79130_e120917_d_n0, assign79130_e120917_d_n2, assign79130_e120917_d_n4, assign79130_e120917_d_n5, assign79130_e120917_d_n6, assign79130_e120917_d_n7, assign79130_e120917_d_n8, assign79130_e120917_d_n9, assign79130_e120917_d_n10, assign79130_e120917_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        let assign79130_e120915: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79130_e120915, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign79130_e120917;
        locals.var_qsuld_dn0 = assign79130_e120917_d_n0;
        locals.var_qsuld_dn2 = assign79130_e120917_d_n2;
        locals.var_qsuld_dn4 = assign79130_e120917_d_n4;
        locals.var_qsuld_dn5 = assign79130_e120917_d_n5;
        locals.var_qsuld_dn6 = assign79130_e120917_d_n6;
        locals.var_qsuld_dn7 = assign79130_e120917_d_n7;
        locals.var_qsuld_dn8 = assign79130_e120917_d_n8;
        locals.var_qsuld_dn9 = assign79130_e120917_d_n9;
        locals.var_qsuld_dn10 = assign79130_e120917_d_n10;
        locals.var_qsuld_dn13 = assign79130_e120917_d_n13;

        let assign79140_e120920: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1843 = assign79140_e120920;

        let (assign79150_e120930, assign79150_e120930_d_n0, assign79150_e120930_d_n2, assign79150_e120930_d_n4, assign79150_e120930_d_n5, assign79150_e120930_d_n6, assign79150_e120930_d_n7, assign79150_e120930_d_n8, assign79150_e120930_d_n9, assign79150_e120930_d_n10, assign79150_e120930_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79150_e120926: f64 = (-locals.var_vxbgmtcl);
        let assign79150_e120927: f64 = (locals.var_beta * assign79150_e120926);
        let assign79150_e120928: f64 = (assign79150_e120927).exp();
        (assign79150_e120928, (assign79150_e120928 * ((locals.var_beta_dn0 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign79150_e120928 * ((locals.var_beta_dn2 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign79150_e120928 * ((locals.var_beta_dn4 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign79150_e120928 * ((locals.var_beta_dn5 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign79150_e120928 * ((locals.var_beta_dn6 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign79150_e120928 * ((locals.var_beta_dn7 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign79150_e120928 * ((locals.var_beta_dn8 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign79150_e120928 * ((locals.var_beta_dn9 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign79150_e120928 * ((locals.var_beta_dn10 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign79150_e120928 * ((locals.var_beta_dn13 * assign79150_e120926) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign79150_e120930;
        locals.var_exp_bvbs_dn0 = assign79150_e120930_d_n0;
        locals.var_exp_bvbs_dn2 = assign79150_e120930_d_n2;
        locals.var_exp_bvbs_dn4 = assign79150_e120930_d_n4;
        locals.var_exp_bvbs_dn5 = assign79150_e120930_d_n5;
        locals.var_exp_bvbs_dn6 = assign79150_e120930_d_n6;
        locals.var_exp_bvbs_dn7 = assign79150_e120930_d_n7;
        locals.var_exp_bvbs_dn8 = assign79150_e120930_d_n8;
        locals.var_exp_bvbs_dn9 = assign79150_e120930_d_n9;
        locals.var_exp_bvbs_dn10 = assign79150_e120930_d_n10;
        locals.var_exp_bvbs_dn13 = assign79150_e120930_d_n13;

        let (assign79160_e120938, assign79160_e120938_d_n0, assign79160_e120938_d_n2, assign79160_e120938_d_n4, assign79160_e120938_d_n5, assign79160_e120938_d_n6, assign79160_e120938_d_n7, assign79160_e120938_d_n8, assign79160_e120938_d_n9, assign79160_e120938_d_n10, assign79160_e120938_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79160_e120936: f64 = (locals.var_nin / locals.var_nover_func);
        (assign79160_e120936, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79160_e120938;
        locals.var_t0_dn0 = assign79160_e120938_d_n0;
        locals.var_t0_dn2 = assign79160_e120938_d_n2;
        locals.var_t0_dn4 = assign79160_e120938_d_n4;
        locals.var_t0_dn5 = assign79160_e120938_d_n5;
        locals.var_t0_dn6 = assign79160_e120938_d_n6;
        locals.var_t0_dn7 = assign79160_e120938_d_n7;
        locals.var_t0_dn8 = assign79160_e120938_d_n8;
        locals.var_t0_dn9 = assign79160_e120938_d_n9;
        locals.var_t0_dn10 = assign79160_e120938_d_n10;
        locals.var_t0_dn13 = assign79160_e120938_d_n13;

        let (assign79170_e120946, assign79170_e120946_d_n0, assign79170_e120946_d_n2, assign79170_e120946_d_n4, assign79170_e120946_d_n5, assign79170_e120946_d_n6, assign79170_e120946_d_n7, assign79170_e120946_d_n8, assign79170_e120946_d_n9, assign79170_e120946_d_n10, assign79170_e120946_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79170_e120944: f64 = (locals.var_t0 * locals.var_t0);
        (assign79170_e120944, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign79170_e120946;
        locals.var_cnst1over_dn0 = assign79170_e120946_d_n0;
        locals.var_cnst1over_dn2 = assign79170_e120946_d_n2;
        locals.var_cnst1over_dn4 = assign79170_e120946_d_n4;
        locals.var_cnst1over_dn5 = assign79170_e120946_d_n5;
        locals.var_cnst1over_dn6 = assign79170_e120946_d_n6;
        locals.var_cnst1over_dn7 = assign79170_e120946_d_n7;
        locals.var_cnst1over_dn8 = assign79170_e120946_d_n8;
        locals.var_cnst1over_dn9 = assign79170_e120946_d_n9;
        locals.var_cnst1over_dn10 = assign79170_e120946_d_n10;
        locals.var_cnst1over_dn13 = assign79170_e120946_d_n13;

        let (assign79180_e120954, assign79180_e120954_d_n0, assign79180_e120954_d_n2, assign79180_e120954_d_n4, assign79180_e120954_d_n5, assign79180_e120954_d_n6, assign79180_e120954_d_n7, assign79180_e120954_d_n8, assign79180_e120954_d_n9, assign79180_e120954_d_n10, assign79180_e120954_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79180_e120952: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign79180_e120952, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign79180_e120954;
        locals.var_cfs1_dn0 = assign79180_e120954_d_n0;
        locals.var_cfs1_dn2 = assign79180_e120954_d_n2;
        locals.var_cfs1_dn4 = assign79180_e120954_d_n4;
        locals.var_cfs1_dn5 = assign79180_e120954_d_n5;
        locals.var_cfs1_dn6 = assign79180_e120954_d_n6;
        locals.var_cfs1_dn7 = assign79180_e120954_d_n7;
        locals.var_cfs1_dn8 = assign79180_e120954_d_n8;
        locals.var_cfs1_dn9 = assign79180_e120954_d_n9;
        locals.var_cfs1_dn10 = assign79180_e120954_d_n10;
        locals.var_cfs1_dn13 = assign79180_e120954_d_n13;

        let (assign79190_e120960, assign79190_e120960_d_n0, assign79190_e120960_d_n2, assign79190_e120960_d_n4, assign79190_e120960_d_n5, assign79190_e120960_d_n6, assign79190_e120960_d_n7, assign79190_e120960_d_n8, assign79190_e120960_d_n9, assign79190_e120960_d_n10, assign79190_e120960_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (locals.var_ps0ld_ini__blk1769, locals.var_ps0ld_ini__blk1769_dn0, locals.var_ps0ld_ini__blk1769_dn2, locals.var_ps0ld_ini__blk1769_dn4, locals.var_ps0ld_ini__blk1769_dn5, locals.var_ps0ld_ini__blk1769_dn6, locals.var_ps0ld_ini__blk1769_dn7, locals.var_ps0ld_ini__blk1769_dn8, locals.var_ps0ld_ini__blk1769_dn9, locals.var_ps0ld_ini__blk1769_dn10, locals.var_ps0ld_ini__blk1769_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign79190_e120960;
        locals.var_ps0ld_dn0 = assign79190_e120960_d_n0;
        locals.var_ps0ld_dn2 = assign79190_e120960_d_n2;
        locals.var_ps0ld_dn4 = assign79190_e120960_d_n4;
        locals.var_ps0ld_dn5 = assign79190_e120960_d_n5;
        locals.var_ps0ld_dn6 = assign79190_e120960_d_n6;
        locals.var_ps0ld_dn7 = assign79190_e120960_d_n7;
        locals.var_ps0ld_dn8 = assign79190_e120960_d_n8;
        locals.var_ps0ld_dn9 = assign79190_e120960_d_n9;
        locals.var_ps0ld_dn10 = assign79190_e120960_d_n10;
        locals.var_ps0ld_dn13 = assign79190_e120960_d_n13;

        let (assign79200_e120966,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign79200_e120966;

        let (assign79210_e120979, assign79210_e120979_d_n0, assign79210_e120979_d_n2, assign79210_e120979_d_n4, assign79210_e120979_d_n5, assign79210_e120979_d_n6, assign79210_e120979_d_n7, assign79210_e120979_d_n8, assign79210_e120979_d_n9, assign79210_e120979_d_n10, assign79210_e120979_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79210_e120973: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1762);
        let assign79210_e120975: f64 = (assign79210_e120973 * locals.var_beta_inv);
        let assign79210_e120976: f64 = (2.0 * assign79210_e120975);
        let assign79210_e120977: f64 = (assign79210_e120976).sqrt();
        (assign79210_e120977, ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn0)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn2)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn4)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn5)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn6)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn7)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn8)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn9)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn10)) / (2.0 * assign79210_e120977)), ((2.0 * (assign79210_e120973 * locals.var_beta_inv_dn13)) / (2.0 * assign79210_e120977)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign79210_e120979;
        locals.var_c_w_ld_dn0 = assign79210_e120979_d_n0;
        locals.var_c_w_ld_dn2 = assign79210_e120979_d_n2;
        locals.var_c_w_ld_dn4 = assign79210_e120979_d_n4;
        locals.var_c_w_ld_dn5 = assign79210_e120979_d_n5;
        locals.var_c_w_ld_dn6 = assign79210_e120979_d_n6;
        locals.var_c_w_ld_dn7 = assign79210_e120979_d_n7;
        locals.var_c_w_ld_dn8 = assign79210_e120979_d_n8;
        locals.var_c_w_ld_dn9 = assign79210_e120979_d_n9;
        locals.var_c_w_ld_dn10 = assign79210_e120979_d_n10;
        locals.var_c_w_ld_dn13 = assign79210_e120979_d_n13;

        let assign79220_e120982: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1844 = assign79220_e120982;

        let (assign79230_e120992, assign79230_e120992_d_n0, assign79230_e120992_d_n2, assign79230_e120992_d_n4, assign79230_e120992_d_n5, assign79230_e120992_d_n6, assign79230_e120992_d_n7, assign79230_e120992_d_n8, assign79230_e120992_d_n9, assign79230_e120992_d_n10, assign79230_e120992_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 != 0.0)) {
        let assign79230_e120990: f64 = (p.p334 - locals.var_wdep_func);
        (assign79230_e120990, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79230_e120992;
        locals.var_t2_dn0 = assign79230_e120992_d_n0;
        locals.var_t2_dn2 = assign79230_e120992_d_n2;
        locals.var_t2_dn4 = assign79230_e120992_d_n4;
        locals.var_t2_dn5 = assign79230_e120992_d_n5;
        locals.var_t2_dn6 = assign79230_e120992_d_n6;
        locals.var_t2_dn7 = assign79230_e120992_d_n7;
        locals.var_t2_dn8 = assign79230_e120992_d_n8;
        locals.var_t2_dn9 = assign79230_e120992_d_n9;
        locals.var_t2_dn10 = assign79230_e120992_d_n10;
        locals.var_t2_dn13 = assign79230_e120992_d_n13;

        let (assign79240_e121014, assign79240_e121014_d_n0, assign79240_e121014_d_n2, assign79240_e121014_d_n4, assign79240_e121014_d_n5, assign79240_e121014_d_n6, assign79240_e121014_d_n7, assign79240_e121014_d_n8, assign79240_e121014_d_n9, assign79240_e121014_d_n10, assign79240_e121014_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79240_e121001: f64 = (locals.var_vdsi + p.p137);
        let assign79240_e121004: f64 = (locals.var_vdsi + p.p137);
        let assign79240_e121005: f64 = (assign79240_e121001 * assign79240_e121004);
        let assign79240_e121008: f64 = (4.0 * 0.1);
        let assign79240_e121010: f64 = (assign79240_e121008 * 0.1);
        let assign79240_e121011: f64 = (assign79240_e121005 + assign79240_e121010);
        let assign79240_e121012: f64 = (assign79240_e121011).sqrt();
        (assign79240_e121012, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign79240_e121004) + (assign79240_e121001 * locals.var_vdsi_dn5)) / (2.0 * assign79240_e121012)), 0.0, (((locals.var_vdsi_dn7 * assign79240_e121004) + (assign79240_e121001 * locals.var_vdsi_dn7)) / (2.0 * assign79240_e121012)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79240_e121014;
        locals.var_tmf2_dn0 = assign79240_e121014_d_n0;
        locals.var_tmf2_dn2 = assign79240_e121014_d_n2;
        locals.var_tmf2_dn4 = assign79240_e121014_d_n4;
        locals.var_tmf2_dn5 = assign79240_e121014_d_n5;
        locals.var_tmf2_dn6 = assign79240_e121014_d_n6;
        locals.var_tmf2_dn7 = assign79240_e121014_d_n7;
        locals.var_tmf2_dn8 = assign79240_e121014_d_n8;
        locals.var_tmf2_dn9 = assign79240_e121014_d_n9;
        locals.var_tmf2_dn10 = assign79240_e121014_d_n10;
        locals.var_tmf2_dn13 = assign79240_e121014_d_n13;

        let (assign79250_e121031, assign79250_e121031_d_n0, assign79250_e121031_d_n2, assign79250_e121031_d_n4, assign79250_e121031_d_n5, assign79250_e121031_d_n6, assign79250_e121031_d_n7, assign79250_e121031_d_n8, assign79250_e121031_d_n9, assign79250_e121031_d_n10, assign79250_e121031_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79250_e121025: f64 = (locals.var_vdsi + p.p137);
        let assign79250_e121027: f64 = (assign79250_e121025 / locals.var_tmf2);
        let assign79250_e121028: f64 = (1.0 + assign79250_e121027);
        let assign79250_e121029: f64 = (0.5 * assign79250_e121028);
        (assign79250_e121029, (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign79250_e121025 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign79250_e121025 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79250_e121025 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79250_e121031;
        locals.var_t9_dn0 = assign79250_e121031_d_n0;
        locals.var_t9_dn2 = assign79250_e121031_d_n2;
        locals.var_t9_dn4 = assign79250_e121031_d_n4;
        locals.var_t9_dn5 = assign79250_e121031_d_n5;
        locals.var_t9_dn6 = assign79250_e121031_d_n6;
        locals.var_t9_dn7 = assign79250_e121031_d_n7;
        locals.var_t9_dn8 = assign79250_e121031_d_n8;
        locals.var_t9_dn9 = assign79250_e121031_d_n9;
        locals.var_t9_dn10 = assign79250_e121031_d_n10;
        locals.var_t9_dn13 = assign79250_e121031_d_n13;

        let (assign79260_e121046, assign79260_e121046_d_n0, assign79260_e121046_d_n2, assign79260_e121046_d_n4, assign79260_e121046_d_n5, assign79260_e121046_d_n6, assign79260_e121046_d_n7, assign79260_e121046_d_n8, assign79260_e121046_d_n9, assign79260_e121046_d_n10, assign79260_e121046_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79260_e121041: f64 = (locals.var_vdsi + p.p137);
        let assign79260_e121043: f64 = (assign79260_e121041 + locals.var_tmf2);
        let assign79260_e121044: f64 = (0.5 * assign79260_e121043);
        (assign79260_e121044, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79260_e121046;
        locals.var_t2_dn0 = assign79260_e121046_d_n0;
        locals.var_t2_dn2 = assign79260_e121046_d_n2;
        locals.var_t2_dn4 = assign79260_e121046_d_n4;
        locals.var_t2_dn5 = assign79260_e121046_d_n5;
        locals.var_t2_dn6 = assign79260_e121046_d_n6;
        locals.var_t2_dn7 = assign79260_e121046_d_n7;
        locals.var_t2_dn8 = assign79260_e121046_d_n8;
        locals.var_t2_dn9 = assign79260_e121046_d_n9;
        locals.var_t2_dn10 = assign79260_e121046_d_n10;
        locals.var_t2_dn13 = assign79260_e121046_d_n13;

        let assign79270_e121049: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1845 = assign79270_e121049;

        let (assign79280_e121060, assign79280_e121060_d_n0, assign79280_e121060_d_n2, assign79280_e121060_d_n4, assign79280_e121060_d_n5, assign79280_e121060_d_n6, assign79280_e121060_d_n7, assign79280_e121060_d_n8, assign79280_e121060_d_n9, assign79280_e121060_d_n10, assign79280_e121060_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) && (locals.var_guard1845 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79280_e121060;
        locals.var_t2_dn0 = assign79280_e121060_d_n0;
        locals.var_t2_dn2 = assign79280_e121060_d_n2;
        locals.var_t2_dn4 = assign79280_e121060_d_n4;
        locals.var_t2_dn5 = assign79280_e121060_d_n5;
        locals.var_t2_dn6 = assign79280_e121060_d_n6;
        locals.var_t2_dn7 = assign79280_e121060_d_n7;
        locals.var_t2_dn8 = assign79280_e121060_d_n8;
        locals.var_t2_dn9 = assign79280_e121060_d_n9;
        locals.var_t2_dn10 = assign79280_e121060_d_n10;
        locals.var_t2_dn13 = assign79280_e121060_d_n13;

        let (assign79290_e121071, assign79290_e121071_d_n0, assign79290_e121071_d_n2, assign79290_e121071_d_n4, assign79290_e121071_d_n5, assign79290_e121071_d_n6, assign79290_e121071_d_n7, assign79290_e121071_d_n8, assign79290_e121071_d_n9, assign79290_e121071_d_n10, assign79290_e121071_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) && (locals.var_guard1845 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79290_e121071;
        locals.var_t9_dn0 = assign79290_e121071_d_n0;
        locals.var_t9_dn2 = assign79290_e121071_d_n2;
        locals.var_t9_dn4 = assign79290_e121071_d_n4;
        locals.var_t9_dn5 = assign79290_e121071_d_n5;
        locals.var_t9_dn6 = assign79290_e121071_d_n6;
        locals.var_t9_dn7 = assign79290_e121071_d_n7;
        locals.var_t9_dn8 = assign79290_e121071_d_n8;
        locals.var_t9_dn9 = assign79290_e121071_d_n9;
        locals.var_t9_dn10 = assign79290_e121071_d_n10;
        locals.var_t9_dn13 = assign79290_e121071_d_n13;

        let (assign79300_e121085, assign79300_e121085_d_n0, assign79300_e121085_d_n2, assign79300_e121085_d_n4, assign79300_e121085_d_n5, assign79300_e121085_d_n6, assign79300_e121085_d_n7, assign79300_e121085_d_n8, assign79300_e121085_d_n9, assign79300_e121085_d_n10, assign79300_e121085_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79300_e121080: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79300_e121081: f64 = (assign79300_e121080).sqrt();
        let assign79300_e121083: f64 = (assign79300_e121081 * p.p432);
        (assign79300_e121083, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79300_e121081)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign79300_e121081)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign79300_e121085;
        locals.var_wjunc0_dn0 = assign79300_e121085_d_n0;
        locals.var_wjunc0_dn2 = assign79300_e121085_d_n2;
        locals.var_wjunc0_dn4 = assign79300_e121085_d_n4;
        locals.var_wjunc0_dn5 = assign79300_e121085_d_n5;
        locals.var_wjunc0_dn6 = assign79300_e121085_d_n6;
        locals.var_wjunc0_dn7 = assign79300_e121085_d_n7;
        locals.var_wjunc0_dn8 = assign79300_e121085_d_n8;
        locals.var_wjunc0_dn9 = assign79300_e121085_d_n9;
        locals.var_wjunc0_dn10 = assign79300_e121085_d_n10;
        locals.var_wjunc0_dn13 = assign79300_e121085_d_n13;

        let (assign79310_e121096, assign79310_e121096_d_n0, assign79310_e121096_d_n2, assign79310_e121096_d_n4, assign79310_e121096_d_n5, assign79310_e121096_d_n6, assign79310_e121096_d_n7, assign79310_e121096_d_n8, assign79310_e121096_d_n9, assign79310_e121096_d_n10, assign79310_e121096_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1844 == 0.0)) {
        let assign79310_e121094: f64 = (p.p334 - locals.var_wjunc0);
        (assign79310_e121094, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79310_e121096;
        locals.var_t2_dn0 = assign79310_e121096_d_n0;
        locals.var_t2_dn2 = assign79310_e121096_d_n2;
        locals.var_t2_dn4 = assign79310_e121096_d_n4;
        locals.var_t2_dn5 = assign79310_e121096_d_n5;
        locals.var_t2_dn6 = assign79310_e121096_d_n6;
        locals.var_t2_dn7 = assign79310_e121096_d_n7;
        locals.var_t2_dn8 = assign79310_e121096_d_n8;
        locals.var_t2_dn9 = assign79310_e121096_d_n9;
        locals.var_t2_dn10 = assign79310_e121096_d_n10;
        locals.var_t2_dn13 = assign79310_e121096_d_n13;

        let (assign79320_e121115, assign79320_e121115_d_n0, assign79320_e121115_d_n2, assign79320_e121115_d_n4, assign79320_e121115_d_n5, assign79320_e121115_d_n6, assign79320_e121115_d_n7, assign79320_e121115_d_n8, assign79320_e121115_d_n9, assign79320_e121115_d_n10, assign79320_e121115_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79320_e121102: f64 = (locals.var_t2 * locals.var_t2);
        let assign79320_e121106: f64 = (p.p334 * 0.01);
        let assign79320_e121107: f64 = (4.0 * assign79320_e121106);
        let assign79320_e121110: f64 = (p.p334 * 0.01);
        let assign79320_e121111: f64 = (assign79320_e121107 * assign79320_e121110);
        let assign79320_e121112: f64 = (assign79320_e121102 + assign79320_e121111);
        let assign79320_e121113: f64 = (assign79320_e121112).sqrt();
        (assign79320_e121113, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign79320_e121113)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign79320_e121113)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79320_e121115;
        locals.var_tmf2_dn0 = assign79320_e121115_d_n0;
        locals.var_tmf2_dn2 = assign79320_e121115_d_n2;
        locals.var_tmf2_dn4 = assign79320_e121115_d_n4;
        locals.var_tmf2_dn5 = assign79320_e121115_d_n5;
        locals.var_tmf2_dn6 = assign79320_e121115_d_n6;
        locals.var_tmf2_dn7 = assign79320_e121115_d_n7;
        locals.var_tmf2_dn8 = assign79320_e121115_d_n8;
        locals.var_tmf2_dn9 = assign79320_e121115_d_n9;
        locals.var_tmf2_dn10 = assign79320_e121115_d_n10;
        locals.var_tmf2_dn13 = assign79320_e121115_d_n13;

        let (assign79330_e121127, assign79330_e121127_d_n0, assign79330_e121127_d_n2, assign79330_e121127_d_n4, assign79330_e121127_d_n5, assign79330_e121127_d_n6, assign79330_e121127_d_n7, assign79330_e121127_d_n8, assign79330_e121127_d_n9, assign79330_e121127_d_n10, assign79330_e121127_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79330_e121123: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign79330_e121124: f64 = (1.0 + assign79330_e121123);
        let assign79330_e121125: f64 = (0.5 * assign79330_e121124);
        (assign79330_e121125, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79330_e121127;
        locals.var_t9_dn0 = assign79330_e121127_d_n0;
        locals.var_t9_dn2 = assign79330_e121127_d_n2;
        locals.var_t9_dn4 = assign79330_e121127_d_n4;
        locals.var_t9_dn5 = assign79330_e121127_d_n5;
        locals.var_t9_dn6 = assign79330_e121127_d_n6;
        locals.var_t9_dn7 = assign79330_e121127_d_n7;
        locals.var_t9_dn8 = assign79330_e121127_d_n8;
        locals.var_t9_dn9 = assign79330_e121127_d_n9;
        locals.var_t9_dn10 = assign79330_e121127_d_n10;
        locals.var_t9_dn13 = assign79330_e121127_d_n13;

    }

    pub(super) fn stamp_transient_block_275(
        locals: &mut StampLocals,
    ) {
        let (assign79340_e121137, assign79340_e121137_d_n0, assign79340_e121137_d_n2, assign79340_e121137_d_n4, assign79340_e121137_d_n5, assign79340_e121137_d_n6, assign79340_e121137_d_n7, assign79340_e121137_d_n8, assign79340_e121137_d_n9, assign79340_e121137_d_n10, assign79340_e121137_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79340_e121134: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign79340_e121135: f64 = (0.5 * assign79340_e121134);
        (assign79340_e121135, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79340_e121137;
        locals.var_t2_dn0 = assign79340_e121137_d_n0;
        locals.var_t2_dn2 = assign79340_e121137_d_n2;
        locals.var_t2_dn4 = assign79340_e121137_d_n4;
        locals.var_t2_dn5 = assign79340_e121137_d_n5;
        locals.var_t2_dn6 = assign79340_e121137_d_n6;
        locals.var_t2_dn7 = assign79340_e121137_d_n7;
        locals.var_t2_dn8 = assign79340_e121137_d_n8;
        locals.var_t2_dn9 = assign79340_e121137_d_n9;
        locals.var_t2_dn10 = assign79340_e121137_d_n10;
        locals.var_t2_dn13 = assign79340_e121137_d_n13;

        let assign79350_e121140: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1846 = assign79350_e121140;

        let (assign79360_e121148, assign79360_e121148_d_n0, assign79360_e121148_d_n2, assign79360_e121148_d_n4, assign79360_e121148_d_n5, assign79360_e121148_d_n6, assign79360_e121148_d_n7, assign79360_e121148_d_n8, assign79360_e121148_d_n9, assign79360_e121148_d_n10, assign79360_e121148_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1846 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79360_e121148;
        locals.var_t2_dn0 = assign79360_e121148_d_n0;
        locals.var_t2_dn2 = assign79360_e121148_d_n2;
        locals.var_t2_dn4 = assign79360_e121148_d_n4;
        locals.var_t2_dn5 = assign79360_e121148_d_n5;
        locals.var_t2_dn6 = assign79360_e121148_d_n6;
        locals.var_t2_dn7 = assign79360_e121148_d_n7;
        locals.var_t2_dn8 = assign79360_e121148_d_n8;
        locals.var_t2_dn9 = assign79360_e121148_d_n9;
        locals.var_t2_dn10 = assign79360_e121148_d_n10;
        locals.var_t2_dn13 = assign79360_e121148_d_n13;

        let (assign79370_e121156, assign79370_e121156_d_n0, assign79370_e121156_d_n2, assign79370_e121156_d_n4, assign79370_e121156_d_n5, assign79370_e121156_d_n6, assign79370_e121156_d_n7, assign79370_e121156_d_n8, assign79370_e121156_d_n9, assign79370_e121156_d_n10, assign79370_e121156_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1846 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79370_e121156;
        locals.var_t9_dn0 = assign79370_e121156_d_n0;
        locals.var_t9_dn2 = assign79370_e121156_d_n2;
        locals.var_t9_dn4 = assign79370_e121156_d_n4;
        locals.var_t9_dn5 = assign79370_e121156_d_n5;
        locals.var_t9_dn6 = assign79370_e121156_d_n6;
        locals.var_t9_dn7 = assign79370_e121156_d_n7;
        locals.var_t9_dn8 = assign79370_e121156_d_n8;
        locals.var_t9_dn9 = assign79370_e121156_d_n9;
        locals.var_t9_dn10 = assign79370_e121156_d_n10;
        locals.var_t9_dn13 = assign79370_e121156_d_n13;

        let (assign79380_e121162, assign79380_e121162_d_n0, assign79380_e121162_d_n2, assign79380_e121162_d_n4, assign79380_e121162_d_n5, assign79380_e121162_d_n6, assign79380_e121162_d_n7, assign79380_e121162_d_n8, assign79380_e121162_d_n9, assign79380_e121162_d_n10, assign79380_e121162_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign79380_e121162;
        locals.var_ddriftldc_dn0 = assign79380_e121162_d_n0;
        locals.var_ddriftldc_dn2 = assign79380_e121162_d_n2;
        locals.var_ddriftldc_dn4 = assign79380_e121162_d_n4;
        locals.var_ddriftldc_dn5 = assign79380_e121162_d_n5;
        locals.var_ddriftldc_dn6 = assign79380_e121162_d_n6;
        locals.var_ddriftldc_dn7 = assign79380_e121162_d_n7;
        locals.var_ddriftldc_dn8 = assign79380_e121162_d_n8;
        locals.var_ddriftldc_dn9 = assign79380_e121162_d_n9;
        locals.var_ddriftldc_dn10 = assign79380_e121162_d_n10;
        locals.var_ddriftldc_dn13 = assign79380_e121162_d_n13;

        let (assign79390_e121176, assign79390_e121176_d_n0, assign79390_e121176_d_n2, assign79390_e121176_d_n4, assign79390_e121176_d_n5, assign79390_e121176_d_n6, assign79390_e121176_d_n7, assign79390_e121176_d_n8, assign79390_e121176_d_n9, assign79390_e121176_d_n10, assign79390_e121176_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79390_e121168: f64 = (locals.var_q_nsubld__blk1762 * locals.var_ddriftldc);
        let assign79390_e121170: f64 = (assign79390_e121168 * locals.var_ddriftldc);
        let assign79390_e121172: f64 = (assign79390_e121170 / 2.0);
        let assign79390_e121174: f64 = (assign79390_e121172 / 1.034943e-10);
        (assign79390_e121174, (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1762 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign79390_e121168 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign79390_e121176;
        locals.var_dphi_sb_dn0 = assign79390_e121176_d_n0;
        locals.var_dphi_sb_dn2 = assign79390_e121176_d_n2;
        locals.var_dphi_sb_dn4 = assign79390_e121176_d_n4;
        locals.var_dphi_sb_dn5 = assign79390_e121176_d_n5;
        locals.var_dphi_sb_dn6 = assign79390_e121176_d_n6;
        locals.var_dphi_sb_dn7 = assign79390_e121176_d_n7;
        locals.var_dphi_sb_dn8 = assign79390_e121176_d_n8;
        locals.var_dphi_sb_dn9 = assign79390_e121176_d_n9;
        locals.var_dphi_sb_dn10 = assign79390_e121176_d_n10;
        locals.var_dphi_sb_dn13 = assign79390_e121176_d_n13;

        let (assign79400_e121187, assign79400_e121187_d_n0, assign79400_e121187_d_n2, assign79400_e121187_d_n4, assign79400_e121187_d_n5, assign79400_e121187_d_n6, assign79400_e121187_d_n7, assign79400_e121187_d_n8, assign79400_e121187_d_n9, assign79400_e121187_d_n10, assign79400_e121187_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79400_e121182: f64 = (2.0 * locals.var_beta);
        let assign79400_e121184: f64 = (assign79400_e121182 * locals.var_dphi_sb);
        let assign79400_e121185: f64 = (assign79400_e121184).sqrt();
        (assign79400_e121185, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn0)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn2)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn4)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn5)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn6)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn7)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn8)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn9)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn10)) / (2.0 * assign79400_e121185)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign79400_e121182 * locals.var_dphi_sb_dn13)) / (2.0 * assign79400_e121185)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79400_e121187;
        locals.var_t0_dn0 = assign79400_e121187_d_n0;
        locals.var_t0_dn2 = assign79400_e121187_d_n2;
        locals.var_t0_dn4 = assign79400_e121187_d_n4;
        locals.var_t0_dn5 = assign79400_e121187_d_n5;
        locals.var_t0_dn6 = assign79400_e121187_d_n6;
        locals.var_t0_dn7 = assign79400_e121187_d_n7;
        locals.var_t0_dn8 = assign79400_e121187_d_n8;
        locals.var_t0_dn9 = assign79400_e121187_d_n9;
        locals.var_t0_dn10 = assign79400_e121187_d_n10;
        locals.var_t0_dn13 = assign79400_e121187_d_n13;

        let (assign79410_e121200, assign79410_e121200_d_n0, assign79410_e121200_d_n2, assign79410_e121200_d_n4, assign79410_e121200_d_n5, assign79410_e121200_d_n6, assign79410_e121200_d_n7, assign79410_e121200_d_n8, assign79410_e121200_d_n9, assign79410_e121200_d_n10, assign79410_e121200_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79410_e121192: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79410_e121194: f64 = (-locals.var_t0);
        let assign79410_e121195: f64 = { let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79410_e121196: f64 = (assign79410_e121192 + assign79410_e121195);
        let assign79410_e121198: f64 = (assign79410_e121196 / 2.0);
        (assign79410_e121198, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign79410_e121194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79410_e121200;
        locals.var_t1_dn0 = assign79410_e121200_d_n0;
        locals.var_t1_dn2 = assign79410_e121200_d_n2;
        locals.var_t1_dn4 = assign79410_e121200_d_n4;
        locals.var_t1_dn5 = assign79410_e121200_d_n5;
        locals.var_t1_dn6 = assign79410_e121200_d_n6;
        locals.var_t1_dn7 = assign79410_e121200_d_n7;
        locals.var_t1_dn8 = assign79410_e121200_d_n8;
        locals.var_t1_dn9 = assign79410_e121200_d_n9;
        locals.var_t1_dn10 = assign79410_e121200_d_n10;
        locals.var_t1_dn13 = assign79410_e121200_d_n13;

        let (assign79420_e121209, assign79420_e121209_d_n0, assign79420_e121209_d_n2, assign79420_e121209_d_n4, assign79420_e121209_d_n5, assign79420_e121209_d_n6, assign79420_e121209_d_n7, assign79420_e121209_d_n8, assign79420_e121209_d_n9, assign79420_e121209_d_n10, assign79420_e121209_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79420_e121205: f64 = (locals.var_t1).ln();
        let assign79420_e121207: f64 = (assign79420_e121205 / locals.var_dphi_sb);
        (assign79420_e121207, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign79420_e121205 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign79420_e121209;
        locals.var_c_sb_dn0 = assign79420_e121209_d_n0;
        locals.var_c_sb_dn2 = assign79420_e121209_d_n2;
        locals.var_c_sb_dn4 = assign79420_e121209_d_n4;
        locals.var_c_sb_dn5 = assign79420_e121209_d_n5;
        locals.var_c_sb_dn6 = assign79420_e121209_d_n6;
        locals.var_c_sb_dn7 = assign79420_e121209_d_n7;
        locals.var_c_sb_dn8 = assign79420_e121209_d_n8;
        locals.var_c_sb_dn9 = assign79420_e121209_d_n9;
        locals.var_c_sb_dn10 = assign79420_e121209_d_n10;
        locals.var_c_sb_dn13 = assign79420_e121209_d_n13;

        let (assign79430_e121215,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79430_e121215;

    }

    pub(super) fn stamp_transient_block_276(
        locals: &mut StampLocals,
    ) {
        let mut assign79440_loop_guard: usize = 0;
        while {
            let assign79440_cond_e121222: f64 = (locals.var_lp_s0_max + 1.0);
            let assign79440_cond_e121224: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_lp_s0 <= assign79440_cond_e121222)) { 1.0 } else { 0.0 };
            assign79440_cond_e121224 != 0.0
        } {
            assign79440_loop_guard += 1;
            assert!(assign79440_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign79440_body3_e121251, assign79440_body3_e121251_d_n0, assign79440_body3_e121251_d_n2, assign79440_body3_e121251_d_n4, assign79440_body3_e121251_d_n5, assign79440_body3_e121251_d_n6, assign79440_body3_e121251_d_n7, assign79440_body3_e121251_d_n8, assign79440_body3_e121251_d_n9, assign79440_body3_e121251_d_n10, assign79440_body3_e121251_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body3_e121249: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign79440_body3_e121249, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign79440_body3_e121251;
            locals.var_ps0ld_vxb_dn0 = assign79440_body3_e121251_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign79440_body3_e121251_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign79440_body3_e121251_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign79440_body3_e121251_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign79440_body3_e121251_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign79440_body3_e121251_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign79440_body3_e121251_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign79440_body3_e121251_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign79440_body3_e121251_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign79440_body3_e121251_d_n13;
            let (assign79440_body4_e121259, assign79440_body4_e121259_d_n0, assign79440_body4_e121259_d_n2, assign79440_body4_e121259_d_n4, assign79440_body4_e121259_d_n5, assign79440_body4_e121259_d_n6, assign79440_body4_e121259_d_n7, assign79440_body4_e121259_d_n8, assign79440_body4_e121259_d_n9, assign79440_body4_e121259_d_n10, assign79440_body4_e121259_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body4_e121257: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign79440_body4_e121257, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign79440_body4_e121259;
            locals.var_chi_dn0 = assign79440_body4_e121259_d_n0;
            locals.var_chi_dn2 = assign79440_body4_e121259_d_n2;
            locals.var_chi_dn4 = assign79440_body4_e121259_d_n4;
            locals.var_chi_dn5 = assign79440_body4_e121259_d_n5;
            locals.var_chi_dn6 = assign79440_body4_e121259_d_n6;
            locals.var_chi_dn7 = assign79440_body4_e121259_d_n7;
            locals.var_chi_dn8 = assign79440_body4_e121259_d_n8;
            locals.var_chi_dn9 = assign79440_body4_e121259_d_n9;
            locals.var_chi_dn10 = assign79440_body4_e121259_d_n10;
            locals.var_chi_dn13 = assign79440_body4_e121259_d_n13;
            let (assign79440_body5_e121269, assign79440_body5_e121269_d_n0, assign79440_body5_e121269_d_n2, assign79440_body5_e121269_d_n4, assign79440_body5_e121269_d_n5, assign79440_body5_e121269_d_n6, assign79440_body5_e121269_d_n7, assign79440_body5_e121269_d_n8, assign79440_body5_e121269_d_n9, assign79440_body5_e121269_d_n10, assign79440_body5_e121269_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body5_e121266: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign79440_body5_e121267: f64 = (locals.var_c_sb * assign79440_body5_e121266);
        (assign79440_body5_e121267, ((locals.var_c_sb_dn0 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign79440_body5_e121266) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign79440_body5_e121269;
            locals.var_ty_dn0 = assign79440_body5_e121269_d_n0;
            locals.var_ty_dn2 = assign79440_body5_e121269_d_n2;
            locals.var_ty_dn4 = assign79440_body5_e121269_d_n4;
            locals.var_ty_dn5 = assign79440_body5_e121269_d_n5;
            locals.var_ty_dn6 = assign79440_body5_e121269_d_n6;
            locals.var_ty_dn7 = assign79440_body5_e121269_d_n7;
            locals.var_ty_dn8 = assign79440_body5_e121269_d_n8;
            locals.var_ty_dn9 = assign79440_body5_e121269_d_n9;
            locals.var_ty_dn10 = assign79440_body5_e121269_d_n10;
            locals.var_ty_dn13 = assign79440_body5_e121269_d_n13;
            let assign79440_body6_e121272: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1848 = assign79440_body6_e121272;
            let (assign79440_body7_e121281, assign79440_body7_e121281_d_n0, assign79440_body7_e121281_d_n2, assign79440_body7_e121281_d_n4, assign79440_body7_e121281_d_n5, assign79440_body7_e121281_d_n6, assign79440_body7_e121281_d_n7, assign79440_body7_e121281_d_n8, assign79440_body7_e121281_d_n9, assign79440_body7_e121281_d_n10, assign79440_body7_e121281_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body7_e121279: f64 = (locals.var_ty).exp();
        (assign79440_body7_e121279, (assign79440_body7_e121279 * locals.var_ty_dn0), (assign79440_body7_e121279 * locals.var_ty_dn2), (assign79440_body7_e121279 * locals.var_ty_dn4), (assign79440_body7_e121279 * locals.var_ty_dn5), (assign79440_body7_e121279 * locals.var_ty_dn6), (assign79440_body7_e121279 * locals.var_ty_dn7), (assign79440_body7_e121279 * locals.var_ty_dn8), (assign79440_body7_e121279 * locals.var_ty_dn9), (assign79440_body7_e121279 * locals.var_ty_dn10), (assign79440_body7_e121279 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body7_e121281;
            locals.var_t1_dn0 = assign79440_body7_e121281_d_n0;
            locals.var_t1_dn2 = assign79440_body7_e121281_d_n2;
            locals.var_t1_dn4 = assign79440_body7_e121281_d_n4;
            locals.var_t1_dn5 = assign79440_body7_e121281_d_n5;
            locals.var_t1_dn6 = assign79440_body7_e121281_d_n6;
            locals.var_t1_dn7 = assign79440_body7_e121281_d_n7;
            locals.var_t1_dn8 = assign79440_body7_e121281_d_n8;
            locals.var_t1_dn9 = assign79440_body7_e121281_d_n9;
            locals.var_t1_dn10 = assign79440_body7_e121281_d_n10;
            locals.var_t1_dn13 = assign79440_body7_e121281_d_n13;
            let (assign79440_body8_e121293, assign79440_body8_e121293_d_n0, assign79440_body8_e121293_d_n2, assign79440_body8_e121293_d_n4, assign79440_body8_e121293_d_n5, assign79440_body8_e121293_d_n6, assign79440_body8_e121293_d_n7, assign79440_body8_e121293_d_n8, assign79440_body8_e121293_d_n9, assign79440_body8_e121293_d_n10, assign79440_body8_e121293_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body8_e121288: f64 = (-locals.var_c_sb);
        let assign79440_body8_e121290: f64 = (assign79440_body8_e121288 * locals.var_dphi_sb);
        let assign79440_body8_e121291: f64 = (assign79440_body8_e121290).exp();
        (assign79440_body8_e121291, (assign79440_body8_e121291 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn0))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn2))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn4))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn5))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn6))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn7))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn8))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn9))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn10))), (assign79440_body8_e121291 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign79440_body8_e121288 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body8_e121293;
            locals.var_t0_dn0 = assign79440_body8_e121293_d_n0;
            locals.var_t0_dn2 = assign79440_body8_e121293_d_n2;
            locals.var_t0_dn4 = assign79440_body8_e121293_d_n4;
            locals.var_t0_dn5 = assign79440_body8_e121293_d_n5;
            locals.var_t0_dn6 = assign79440_body8_e121293_d_n6;
            locals.var_t0_dn7 = assign79440_body8_e121293_d_n7;
            locals.var_t0_dn8 = assign79440_body8_e121293_d_n8;
            locals.var_t0_dn9 = assign79440_body8_e121293_d_n9;
            locals.var_t0_dn10 = assign79440_body8_e121293_d_n10;
            locals.var_t0_dn13 = assign79440_body8_e121293_d_n13;
            let (assign79440_body9_e121303, assign79440_body9_e121303_d_n0, assign79440_body9_e121303_d_n2, assign79440_body9_e121303_d_n4, assign79440_body9_e121303_d_n5, assign79440_body9_e121303_d_n6, assign79440_body9_e121303_d_n7, assign79440_body9_e121303_d_n8, assign79440_body9_e121303_d_n9, assign79440_body9_e121303_d_n10, assign79440_body9_e121303_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body9_e121301: f64 = (locals.var_t1 - locals.var_t0);
        (assign79440_body9_e121301, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79440_body9_e121303;
            locals.var_t2_dn0 = assign79440_body9_e121303_d_n0;
            locals.var_t2_dn2 = assign79440_body9_e121303_d_n2;
            locals.var_t2_dn4 = assign79440_body9_e121303_d_n4;
            locals.var_t2_dn5 = assign79440_body9_e121303_d_n5;
            locals.var_t2_dn6 = assign79440_body9_e121303_d_n6;
            locals.var_t2_dn7 = assign79440_body9_e121303_d_n7;
            locals.var_t2_dn8 = assign79440_body9_e121303_d_n8;
            locals.var_t2_dn9 = assign79440_body9_e121303_d_n9;
            locals.var_t2_dn10 = assign79440_body9_e121303_d_n10;
            locals.var_t2_dn13 = assign79440_body9_e121303_d_n13;
            let (assign79440_body10_e121316, assign79440_body10_e121316_d_n0, assign79440_body10_e121316_d_n2, assign79440_body10_e121316_d_n4, assign79440_body10_e121316_d_n5, assign79440_body10_e121316_d_n6, assign79440_body10_e121316_d_n7, assign79440_body10_e121316_d_n8, assign79440_body10_e121316_d_n9, assign79440_body10_e121316_d_n10, assign79440_body10_e121316_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body10_e121311: f64 = (1.0 + locals.var_t2);
        let assign79440_body10_e121312: f64 = (assign79440_body10_e121311).ln();
        let assign79440_body10_e121314: f64 = (assign79440_body10_e121312 / locals.var_c_sb);
        (assign79440_body10_e121314, ((((locals.var_t2_dn0 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign79440_body10_e121311) * locals.var_c_sb) - (assign79440_body10_e121312 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79440_body10_e121316;
            locals.var_phi_b_dn0 = assign79440_body10_e121316_d_n0;
            locals.var_phi_b_dn2 = assign79440_body10_e121316_d_n2;
            locals.var_phi_b_dn4 = assign79440_body10_e121316_d_n4;
            locals.var_phi_b_dn5 = assign79440_body10_e121316_d_n5;
            locals.var_phi_b_dn6 = assign79440_body10_e121316_d_n6;
            locals.var_phi_b_dn7 = assign79440_body10_e121316_d_n7;
            locals.var_phi_b_dn8 = assign79440_body10_e121316_d_n8;
            locals.var_phi_b_dn9 = assign79440_body10_e121316_d_n9;
            locals.var_phi_b_dn10 = assign79440_body10_e121316_d_n10;
            locals.var_phi_b_dn13 = assign79440_body10_e121316_d_n13;
            let (assign79440_body11_e121328, assign79440_body11_e121328_d_n0, assign79440_body11_e121328_d_n2, assign79440_body11_e121328_d_n4, assign79440_body11_e121328_d_n5, assign79440_body11_e121328_d_n6, assign79440_body11_e121328_d_n7, assign79440_body11_e121328_d_n8, assign79440_body11_e121328_d_n9, assign79440_body11_e121328_d_n10, assign79440_body11_e121328_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79440_body11_e121325: f64 = (1.0 + locals.var_t2);
        let assign79440_body11_e121326: f64 = (locals.var_t1 / assign79440_body11_e121325);
        (assign79440_body11_e121326, (((locals.var_t1_dn0 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn0)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn2 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn2)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn4 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn4)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn5 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn5)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn6 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn6)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn7 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn7)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn8 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn8)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn9 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn9)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn10 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn10)) / (assign79440_body11_e121325 * assign79440_body11_e121325)), (((locals.var_t1_dn13 * assign79440_body11_e121325) - (locals.var_t1 * locals.var_t2_dn13)) / (assign79440_body11_e121325 * assign79440_body11_e121325)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79440_body11_e121328;
            locals.var_phi_b_dpss_dn0 = assign79440_body11_e121328_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79440_body11_e121328_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79440_body11_e121328_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79440_body11_e121328_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79440_body11_e121328_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79440_body11_e121328_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79440_body11_e121328_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79440_body11_e121328_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79440_body11_e121328_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79440_body11_e121328_d_n13;
            let (assign79440_body12_e121339, assign79440_body12_e121339_d_n0, assign79440_body12_e121339_d_n2, assign79440_body12_e121339_d_n4, assign79440_body12_e121339_d_n5, assign79440_body12_e121339_d_n6, assign79440_body12_e121339_d_n7, assign79440_body12_e121339_d_n8, assign79440_body12_e121339_d_n9, assign79440_body12_e121339_d_n10, assign79440_body12_e121339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79440_body12_e121337: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign79440_body12_e121337, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign79440_body12_e121339;
            locals.var_phi_b_dn0 = assign79440_body12_e121339_d_n0;
            locals.var_phi_b_dn2 = assign79440_body12_e121339_d_n2;
            locals.var_phi_b_dn4 = assign79440_body12_e121339_d_n4;
            locals.var_phi_b_dn5 = assign79440_body12_e121339_d_n5;
            locals.var_phi_b_dn6 = assign79440_body12_e121339_d_n6;
            locals.var_phi_b_dn7 = assign79440_body12_e121339_d_n7;
            locals.var_phi_b_dn8 = assign79440_body12_e121339_d_n8;
            locals.var_phi_b_dn9 = assign79440_body12_e121339_d_n9;
            locals.var_phi_b_dn10 = assign79440_body12_e121339_d_n10;
            locals.var_phi_b_dn13 = assign79440_body12_e121339_d_n13;
            let (assign79440_body13_e121348, assign79440_body13_e121348_d_n0, assign79440_body13_e121348_d_n2, assign79440_body13_e121348_d_n4, assign79440_body13_e121348_d_n5, assign79440_body13_e121348_d_n6, assign79440_body13_e121348_d_n7, assign79440_body13_e121348_d_n8, assign79440_body13_e121348_d_n9, assign79440_body13_e121348_d_n10, assign79440_body13_e121348_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign79440_body13_e121348;
            locals.var_phi_b_dpss_dn0 = assign79440_body13_e121348_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79440_body13_e121348_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79440_body13_e121348_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79440_body13_e121348_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79440_body13_e121348_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79440_body13_e121348_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79440_body13_e121348_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79440_body13_e121348_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79440_body13_e121348_d_n10;
            locals.var_phi_b_dpss_dn13 = assign79440_body13_e121348_d_n13;
            let (assign79440_body14_e121356, assign79440_body14_e121356_d_n0, assign79440_body14_e121356_d_n2, assign79440_body14_e121356_d_n4, assign79440_body14_e121356_d_n5, assign79440_body14_e121356_d_n6, assign79440_body14_e121356_d_n7, assign79440_body14_e121356_d_n8, assign79440_body14_e121356_d_n9, assign79440_body14_e121356_d_n10, assign79440_body14_e121356_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body14_e121354: f64 = (locals.var_beta * locals.var_phi_b);
        (assign79440_body14_e121354, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign79440_body14_e121356;
            locals.var_chib_dn0 = assign79440_body14_e121356_d_n0;
            locals.var_chib_dn2 = assign79440_body14_e121356_d_n2;
            locals.var_chib_dn4 = assign79440_body14_e121356_d_n4;
            locals.var_chib_dn5 = assign79440_body14_e121356_d_n5;
            locals.var_chib_dn6 = assign79440_body14_e121356_d_n6;
            locals.var_chib_dn7 = assign79440_body14_e121356_d_n7;
            locals.var_chib_dn8 = assign79440_body14_e121356_d_n8;
            locals.var_chib_dn9 = assign79440_body14_e121356_d_n9;
            locals.var_chib_dn10 = assign79440_body14_e121356_d_n10;
            locals.var_chib_dn13 = assign79440_body14_e121356_d_n13;
            let assign79440_body15_e121358: f64 = (locals.var_chi).abs();
            let assign79440_body15_e121360: f64 = if assign79440_body15_e121358 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1849 = assign79440_body15_e121360;
            let (assign79440_body17_e121406, assign79440_body17_e121406_d_n0, assign79440_body17_e121406_d_n2, assign79440_body17_e121406_d_n4, assign79440_body17_e121406_d_n5, assign79440_body17_e121406_d_n6, assign79440_body17_e121406_d_n7, assign79440_body17_e121406_d_n8, assign79440_body17_e121406_d_n9, assign79440_body17_e121406_d_n10, assign79440_body17_e121406_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body17_e121384: f64 = (locals.var_chi * locals.var_chi);
        let assign79440_body17_e121386: f64 = (assign79440_body17_e121384 / 2.0);
        let assign79440_body17_e121390: f64 = (locals.var_chi / 3.0);
        let assign79440_body17_e121394: f64 = (locals.var_chi / 4.0);
        let assign79440_body17_e121398: f64 = (locals.var_chi / 5.0);
        let assign79440_body17_e121399: f64 = (1.0 - assign79440_body17_e121398);
        let assign79440_body17_e121400: f64 = (assign79440_body17_e121394 * assign79440_body17_e121399);
        let assign79440_body17_e121401: f64 = (1.0 - assign79440_body17_e121400);
        let assign79440_body17_e121402: f64 = (assign79440_body17_e121390 * assign79440_body17_e121401);
        let assign79440_body17_e121403: f64 = (1.0 - assign79440_body17_e121402);
        let assign79440_body17_e121404: f64 = (assign79440_body17_e121386 * assign79440_body17_e121403);
        (assign79440_body17_e121404, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn0 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn0 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn2 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn2 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn4 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn4 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn5 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn5 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn6 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn6 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn7 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn7 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn8 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn8 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn9 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn9 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn10 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn10 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79440_body17_e121403) + (assign79440_body17_e121386 * (-(((locals.var_chi_dn13 / 3.0) * assign79440_body17_e121401) + (assign79440_body17_e121390 * (-(((locals.var_chi_dn13 / 4.0) * assign79440_body17_e121399) + (assign79440_body17_e121394 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body17_e121406;
            locals.var_t0_dn0 = assign79440_body17_e121406_d_n0;
            locals.var_t0_dn2 = assign79440_body17_e121406_d_n2;
            locals.var_t0_dn4 = assign79440_body17_e121406_d_n4;
            locals.var_t0_dn5 = assign79440_body17_e121406_d_n5;
            locals.var_t0_dn6 = assign79440_body17_e121406_d_n6;
            locals.var_t0_dn7 = assign79440_body17_e121406_d_n7;
            locals.var_t0_dn8 = assign79440_body17_e121406_d_n8;
            locals.var_t0_dn9 = assign79440_body17_e121406_d_n9;
            locals.var_t0_dn10 = assign79440_body17_e121406_d_n10;
            locals.var_t0_dn13 = assign79440_body17_e121406_d_n13;
            let (assign79440_body18_e121432, assign79440_body18_e121432_d_n0, assign79440_body18_e121432_d_n2, assign79440_body18_e121432_d_n4, assign79440_body18_e121432_d_n5, assign79440_body18_e121432_d_n6, assign79440_body18_e121432_d_n7, assign79440_body18_e121432_d_n8, assign79440_body18_e121432_d_n9, assign79440_body18_e121432_d_n10, assign79440_body18_e121432_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body18_e121416: f64 = (locals.var_chi / 2.0);
        let assign79440_body18_e121420: f64 = (locals.var_chi / 3.0);
        let assign79440_body18_e121424: f64 = (locals.var_chi / 4.0);
        let assign79440_body18_e121425: f64 = (1.0 - assign79440_body18_e121424);
        let assign79440_body18_e121426: f64 = (assign79440_body18_e121420 * assign79440_body18_e121425);
        let assign79440_body18_e121427: f64 = (1.0 - assign79440_body18_e121426);
        let assign79440_body18_e121428: f64 = (assign79440_body18_e121416 * assign79440_body18_e121427);
        let assign79440_body18_e121429: f64 = (1.0 - assign79440_body18_e121428);
        let assign79440_body18_e121430: f64 = (locals.var_chi * assign79440_body18_e121429);
        (assign79440_body18_e121430, ((locals.var_chi_dn0 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn0 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn2 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn4 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn5 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn6 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn7 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn8 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn9 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn10 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign79440_body18_e121429) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign79440_body18_e121427) + (assign79440_body18_e121416 * (-(((locals.var_chi_dn13 / 3.0) * assign79440_body18_e121425) + (assign79440_body18_e121420 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body18_e121432;
            locals.var_t1_dn0 = assign79440_body18_e121432_d_n0;
            locals.var_t1_dn2 = assign79440_body18_e121432_d_n2;
            locals.var_t1_dn4 = assign79440_body18_e121432_d_n4;
            locals.var_t1_dn5 = assign79440_body18_e121432_d_n5;
            locals.var_t1_dn6 = assign79440_body18_e121432_d_n6;
            locals.var_t1_dn7 = assign79440_body18_e121432_d_n7;
            locals.var_t1_dn8 = assign79440_body18_e121432_d_n8;
            locals.var_t1_dn9 = assign79440_body18_e121432_d_n9;
            locals.var_t1_dn10 = assign79440_body18_e121432_d_n10;
            locals.var_t1_dn13 = assign79440_body18_e121432_d_n13;
            let (assign79440_body19_e121462, assign79440_body19_e121462_d_n0, assign79440_body19_e121462_d_n2, assign79440_body19_e121462_d_n4, assign79440_body19_e121462_d_n5, assign79440_body19_e121462_d_n6, assign79440_body19_e121462_d_n7, assign79440_body19_e121462_d_n8, assign79440_body19_e121462_d_n9, assign79440_body19_e121462_d_n10, assign79440_body19_e121462_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body19_e121440: f64 = (locals.var_chib * locals.var_chib);
        let assign79440_body19_e121442: f64 = (assign79440_body19_e121440 / 2.0);
        let assign79440_body19_e121446: f64 = (locals.var_chib / 3.0);
        let assign79440_body19_e121450: f64 = (locals.var_chib / 4.0);
        let assign79440_body19_e121454: f64 = (locals.var_chib / 5.0);
        let assign79440_body19_e121455: f64 = (1.0 - assign79440_body19_e121454);
        let assign79440_body19_e121456: f64 = (assign79440_body19_e121450 * assign79440_body19_e121455);
        let assign79440_body19_e121457: f64 = (1.0 - assign79440_body19_e121456);
        let assign79440_body19_e121458: f64 = (assign79440_body19_e121446 * assign79440_body19_e121457);
        let assign79440_body19_e121459: f64 = (1.0 - assign79440_body19_e121458);
        let assign79440_body19_e121460: f64 = (assign79440_body19_e121442 * assign79440_body19_e121459);
        (assign79440_body19_e121460, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn0 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn0 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn2 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn2 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn4 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn4 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn5 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn5 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn6 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn6 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn7 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn7 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn8 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn8 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn9 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn9 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn10 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn10 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign79440_body19_e121459) + (assign79440_body19_e121442 * (-(((locals.var_chib_dn13 / 3.0) * assign79440_body19_e121457) + (assign79440_body19_e121446 * (-(((locals.var_chib_dn13 / 4.0) * assign79440_body19_e121455) + (assign79440_body19_e121450 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign79440_body19_e121462;
            locals.var_t2_dn0 = assign79440_body19_e121462_d_n0;
            locals.var_t2_dn2 = assign79440_body19_e121462_d_n2;
            locals.var_t2_dn4 = assign79440_body19_e121462_d_n4;
            locals.var_t2_dn5 = assign79440_body19_e121462_d_n5;
            locals.var_t2_dn6 = assign79440_body19_e121462_d_n6;
            locals.var_t2_dn7 = assign79440_body19_e121462_d_n7;
            locals.var_t2_dn8 = assign79440_body19_e121462_d_n8;
            locals.var_t2_dn9 = assign79440_body19_e121462_d_n9;
            locals.var_t2_dn10 = assign79440_body19_e121462_d_n10;
            locals.var_t2_dn13 = assign79440_body19_e121462_d_n13;
            let (assign79440_body20_e121488, assign79440_body20_e121488_d_n0, assign79440_body20_e121488_d_n2, assign79440_body20_e121488_d_n4, assign79440_body20_e121488_d_n5, assign79440_body20_e121488_d_n6, assign79440_body20_e121488_d_n7, assign79440_body20_e121488_d_n8, assign79440_body20_e121488_d_n9, assign79440_body20_e121488_d_n10, assign79440_body20_e121488_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body20_e121472: f64 = (locals.var_chib / 2.0);
        let assign79440_body20_e121476: f64 = (locals.var_chib / 3.0);
        let assign79440_body20_e121480: f64 = (locals.var_chib / 4.0);
        let assign79440_body20_e121481: f64 = (1.0 - assign79440_body20_e121480);
        let assign79440_body20_e121482: f64 = (assign79440_body20_e121476 * assign79440_body20_e121481);
        let assign79440_body20_e121483: f64 = (1.0 - assign79440_body20_e121482);
        let assign79440_body20_e121484: f64 = (assign79440_body20_e121472 * assign79440_body20_e121483);
        let assign79440_body20_e121485: f64 = (1.0 - assign79440_body20_e121484);
        let assign79440_body20_e121486: f64 = (locals.var_chib * assign79440_body20_e121485);
        (assign79440_body20_e121486, ((locals.var_chib_dn0 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn0 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn2 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn4 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn5 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn6 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn7 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn8 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn9 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn10 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign79440_body20_e121485) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign79440_body20_e121483) + (assign79440_body20_e121472 * (-(((locals.var_chib_dn13 / 3.0) * assign79440_body20_e121481) + (assign79440_body20_e121476 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign79440_body20_e121488;
            locals.var_t3_dn0 = assign79440_body20_e121488_d_n0;
            locals.var_t3_dn2 = assign79440_body20_e121488_d_n2;
            locals.var_t3_dn4 = assign79440_body20_e121488_d_n4;
            locals.var_t3_dn5 = assign79440_body20_e121488_d_n5;
            locals.var_t3_dn6 = assign79440_body20_e121488_d_n6;
            locals.var_t3_dn7 = assign79440_body20_e121488_d_n7;
            locals.var_t3_dn8 = assign79440_body20_e121488_d_n8;
            locals.var_t3_dn9 = assign79440_body20_e121488_d_n9;
            locals.var_t3_dn10 = assign79440_body20_e121488_d_n10;
            locals.var_t3_dn13 = assign79440_body20_e121488_d_n13;
            let (assign79440_body21_e121498, assign79440_body21_e121498_d_n0, assign79440_body21_e121498_d_n2, assign79440_body21_e121498_d_n4, assign79440_body21_e121498_d_n5, assign79440_body21_e121498_d_n6, assign79440_body21_e121498_d_n7, assign79440_body21_e121498_d_n8, assign79440_body21_e121498_d_n9, assign79440_body21_e121498_d_n10, assign79440_body21_e121498_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body21_e121496: f64 = (locals.var_t0 - locals.var_t2);
        (assign79440_body21_e121496, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk1770, locals.var_fbsq__blk1770_dn0, locals.var_fbsq__blk1770_dn2, locals.var_fbsq__blk1770_dn4, locals.var_fbsq__blk1770_dn5, locals.var_fbsq__blk1770_dn6, locals.var_fbsq__blk1770_dn7, locals.var_fbsq__blk1770_dn8, locals.var_fbsq__blk1770_dn9, locals.var_fbsq__blk1770_dn10, locals.var_fbsq__blk1770_dn13,)
    }
};
            locals.var_fbsq__blk1770 = assign79440_body21_e121498;
            locals.var_fbsq__blk1770_dn0 = assign79440_body21_e121498_d_n0;
            locals.var_fbsq__blk1770_dn2 = assign79440_body21_e121498_d_n2;
            locals.var_fbsq__blk1770_dn4 = assign79440_body21_e121498_d_n4;
            locals.var_fbsq__blk1770_dn5 = assign79440_body21_e121498_d_n5;
            locals.var_fbsq__blk1770_dn6 = assign79440_body21_e121498_d_n6;
            locals.var_fbsq__blk1770_dn7 = assign79440_body21_e121498_d_n7;
            locals.var_fbsq__blk1770_dn8 = assign79440_body21_e121498_d_n8;
            locals.var_fbsq__blk1770_dn9 = assign79440_body21_e121498_d_n9;
            locals.var_fbsq__blk1770_dn10 = assign79440_body21_e121498_d_n10;
            locals.var_fbsq__blk1770_dn13 = assign79440_body21_e121498_d_n13;
            let (assign79440_body22_e121512, assign79440_body22_e121512_d_n0, assign79440_body22_e121512_d_n2, assign79440_body22_e121512_d_n4, assign79440_body22_e121512_d_n5, assign79440_body22_e121512_d_n6, assign79440_body22_e121512_d_n7, assign79440_body22_e121512_d_n8, assign79440_body22_e121512_d_n9, assign79440_body22_e121512_d_n10, assign79440_body22_e121512_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 != 0.0)) {
        let assign79440_body22_e121508: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign79440_body22_e121509: f64 = (locals.var_t1 - assign79440_body22_e121508);
        let assign79440_body22_e121510: f64 = (locals.var_beta * assign79440_body22_e121509);
        (assign79440_body22_e121510, ((locals.var_beta_dn0 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign79440_body22_e121509) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk1771, locals.var_fbsq_dpss__blk1771_dn0, locals.var_fbsq_dpss__blk1771_dn2, locals.var_fbsq_dpss__blk1771_dn4, locals.var_fbsq_dpss__blk1771_dn5, locals.var_fbsq_dpss__blk1771_dn6, locals.var_fbsq_dpss__blk1771_dn7, locals.var_fbsq_dpss__blk1771_dn8, locals.var_fbsq_dpss__blk1771_dn9, locals.var_fbsq_dpss__blk1771_dn10, locals.var_fbsq_dpss__blk1771_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1771 = assign79440_body22_e121512;
            locals.var_fbsq_dpss__blk1771_dn0 = assign79440_body22_e121512_d_n0;
            locals.var_fbsq_dpss__blk1771_dn2 = assign79440_body22_e121512_d_n2;
            locals.var_fbsq_dpss__blk1771_dn4 = assign79440_body22_e121512_d_n4;
            locals.var_fbsq_dpss__blk1771_dn5 = assign79440_body22_e121512_d_n5;
            locals.var_fbsq_dpss__blk1771_dn6 = assign79440_body22_e121512_d_n6;
            locals.var_fbsq_dpss__blk1771_dn7 = assign79440_body22_e121512_d_n7;
            locals.var_fbsq_dpss__blk1771_dn8 = assign79440_body22_e121512_d_n8;
            locals.var_fbsq_dpss__blk1771_dn9 = assign79440_body22_e121512_d_n9;
            locals.var_fbsq_dpss__blk1771_dn10 = assign79440_body22_e121512_d_n10;
            locals.var_fbsq_dpss__blk1771_dn13 = assign79440_body22_e121512_d_n13;
            let (assign79440_body24_e121540, assign79440_body24_e121540_d_n0, assign79440_body24_e121540_d_n2, assign79440_body24_e121540_d_n4, assign79440_body24_e121540_d_n5, assign79440_body24_e121540_d_n6, assign79440_body24_e121540_d_n7, assign79440_body24_e121540_d_n8, assign79440_body24_e121540_d_n9, assign79440_body24_e121540_d_n10, assign79440_body24_e121540_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body24_e121537: f64 = (-locals.var_chi);
        let assign79440_body24_e121538: f64 = (assign79440_body24_e121537).exp();
        (assign79440_body24_e121538, (assign79440_body24_e121538 * (-locals.var_chi_dn0)), (assign79440_body24_e121538 * (-locals.var_chi_dn2)), (assign79440_body24_e121538 * (-locals.var_chi_dn4)), (assign79440_body24_e121538 * (-locals.var_chi_dn5)), (assign79440_body24_e121538 * (-locals.var_chi_dn6)), (assign79440_body24_e121538 * (-locals.var_chi_dn7)), (assign79440_body24_e121538 * (-locals.var_chi_dn8)), (assign79440_body24_e121538 * (-locals.var_chi_dn9)), (assign79440_body24_e121538 * (-locals.var_chi_dn10)), (assign79440_body24_e121538 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body24_e121540;
            locals.var_t0_dn0 = assign79440_body24_e121540_d_n0;
            locals.var_t0_dn2 = assign79440_body24_e121540_d_n2;
            locals.var_t0_dn4 = assign79440_body24_e121540_d_n4;
            locals.var_t0_dn5 = assign79440_body24_e121540_d_n5;
            locals.var_t0_dn6 = assign79440_body24_e121540_d_n6;
            locals.var_t0_dn7 = assign79440_body24_e121540_d_n7;
            locals.var_t0_dn8 = assign79440_body24_e121540_d_n8;
            locals.var_t0_dn9 = assign79440_body24_e121540_d_n9;
            locals.var_t0_dn10 = assign79440_body24_e121540_d_n10;
            locals.var_t0_dn13 = assign79440_body24_e121540_d_n13;
            let (assign79440_body25_e121551, assign79440_body25_e121551_d_n0, assign79440_body25_e121551_d_n2, assign79440_body25_e121551_d_n4, assign79440_body25_e121551_d_n5, assign79440_body25_e121551_d_n6, assign79440_body25_e121551_d_n7, assign79440_body25_e121551_d_n8, assign79440_body25_e121551_d_n9, assign79440_body25_e121551_d_n10, assign79440_body25_e121551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body25_e121548: f64 = (-locals.var_chib);
        let assign79440_body25_e121549: f64 = (assign79440_body25_e121548).exp();
        (assign79440_body25_e121549, (assign79440_body25_e121549 * (-locals.var_chib_dn0)), (assign79440_body25_e121549 * (-locals.var_chib_dn2)), (assign79440_body25_e121549 * (-locals.var_chib_dn4)), (assign79440_body25_e121549 * (-locals.var_chib_dn5)), (assign79440_body25_e121549 * (-locals.var_chib_dn6)), (assign79440_body25_e121549 * (-locals.var_chib_dn7)), (assign79440_body25_e121549 * (-locals.var_chib_dn8)), (assign79440_body25_e121549 * (-locals.var_chib_dn9)), (assign79440_body25_e121549 * (-locals.var_chib_dn10)), (assign79440_body25_e121549 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body25_e121551;
            locals.var_t1_dn0 = assign79440_body25_e121551_d_n0;
            locals.var_t1_dn2 = assign79440_body25_e121551_d_n2;
            locals.var_t1_dn4 = assign79440_body25_e121551_d_n4;
            locals.var_t1_dn5 = assign79440_body25_e121551_d_n5;
            locals.var_t1_dn6 = assign79440_body25_e121551_d_n6;
            locals.var_t1_dn7 = assign79440_body25_e121551_d_n7;
            locals.var_t1_dn8 = assign79440_body25_e121551_d_n8;
            locals.var_t1_dn9 = assign79440_body25_e121551_d_n9;
            locals.var_t1_dn10 = assign79440_body25_e121551_d_n10;
            locals.var_t1_dn13 = assign79440_body25_e121551_d_n13;
            let (assign79440_body26_e121566, assign79440_body26_e121566_d_n0, assign79440_body26_e121566_d_n2, assign79440_body26_e121566_d_n4, assign79440_body26_e121566_d_n5, assign79440_body26_e121566_d_n6, assign79440_body26_e121566_d_n7, assign79440_body26_e121566_d_n8, assign79440_body26_e121566_d_n9, assign79440_body26_e121566_d_n10, assign79440_body26_e121566_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body26_e121560: f64 = (locals.var_chi - locals.var_chib);
        let assign79440_body26_e121563: f64 = (locals.var_t0 - locals.var_t1);
        let assign79440_body26_e121564: f64 = (assign79440_body26_e121560 + assign79440_body26_e121563);
        (assign79440_body26_e121564, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk1770, locals.var_fbsq__blk1770_dn0, locals.var_fbsq__blk1770_dn2, locals.var_fbsq__blk1770_dn4, locals.var_fbsq__blk1770_dn5, locals.var_fbsq__blk1770_dn6, locals.var_fbsq__blk1770_dn7, locals.var_fbsq__blk1770_dn8, locals.var_fbsq__blk1770_dn9, locals.var_fbsq__blk1770_dn10, locals.var_fbsq__blk1770_dn13,)
    }
};
            locals.var_fbsq__blk1770 = assign79440_body26_e121566;
            locals.var_fbsq__blk1770_dn0 = assign79440_body26_e121566_d_n0;
            locals.var_fbsq__blk1770_dn2 = assign79440_body26_e121566_d_n2;
            locals.var_fbsq__blk1770_dn4 = assign79440_body26_e121566_d_n4;
            locals.var_fbsq__blk1770_dn5 = assign79440_body26_e121566_d_n5;
            locals.var_fbsq__blk1770_dn6 = assign79440_body26_e121566_d_n6;
            locals.var_fbsq__blk1770_dn7 = assign79440_body26_e121566_d_n7;
            locals.var_fbsq__blk1770_dn8 = assign79440_body26_e121566_d_n8;
            locals.var_fbsq__blk1770_dn9 = assign79440_body26_e121566_d_n9;
            locals.var_fbsq__blk1770_dn10 = assign79440_body26_e121566_d_n10;
            locals.var_fbsq__blk1770_dn13 = assign79440_body26_e121566_d_n13;
            let (assign79440_body27_e121585, assign79440_body27_e121585_d_n0, assign79440_body27_e121585_d_n2, assign79440_body27_e121585_d_n4, assign79440_body27_e121585_d_n5, assign79440_body27_e121585_d_n6, assign79440_body27_e121585_d_n7, assign79440_body27_e121585_d_n8, assign79440_body27_e121585_d_n9, assign79440_body27_e121585_d_n10, assign79440_body27_e121585_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1849 == 0.0)) {
        let assign79440_body27_e121576: f64 = (1.0 - locals.var_t0);
        let assign79440_body27_e121580: f64 = (1.0 - locals.var_t1);
        let assign79440_body27_e121581: f64 = (locals.var_phi_b_dpss * assign79440_body27_e121580);
        let assign79440_body27_e121582: f64 = (assign79440_body27_e121576 - assign79440_body27_e121581);
        let assign79440_body27_e121583: f64 = (locals.var_beta * assign79440_body27_e121582);
        (assign79440_body27_e121583, ((locals.var_beta_dn0 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign79440_body27_e121582) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign79440_body27_e121580) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk1771, locals.var_fbsq_dpss__blk1771_dn0, locals.var_fbsq_dpss__blk1771_dn2, locals.var_fbsq_dpss__blk1771_dn4, locals.var_fbsq_dpss__blk1771_dn5, locals.var_fbsq_dpss__blk1771_dn6, locals.var_fbsq_dpss__blk1771_dn7, locals.var_fbsq_dpss__blk1771_dn8, locals.var_fbsq_dpss__blk1771_dn9, locals.var_fbsq_dpss__blk1771_dn10, locals.var_fbsq_dpss__blk1771_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1771 = assign79440_body27_e121585;
            locals.var_fbsq_dpss__blk1771_dn0 = assign79440_body27_e121585_d_n0;
            locals.var_fbsq_dpss__blk1771_dn2 = assign79440_body27_e121585_d_n2;
            locals.var_fbsq_dpss__blk1771_dn4 = assign79440_body27_e121585_d_n4;
            locals.var_fbsq_dpss__blk1771_dn5 = assign79440_body27_e121585_d_n5;
            locals.var_fbsq_dpss__blk1771_dn6 = assign79440_body27_e121585_d_n6;
            locals.var_fbsq_dpss__blk1771_dn7 = assign79440_body27_e121585_d_n7;
            locals.var_fbsq_dpss__blk1771_dn8 = assign79440_body27_e121585_d_n8;
            locals.var_fbsq_dpss__blk1771_dn9 = assign79440_body27_e121585_d_n9;
            locals.var_fbsq_dpss__blk1771_dn10 = assign79440_body27_e121585_d_n10;
            locals.var_fbsq_dpss__blk1771_dn13 = assign79440_body27_e121585_d_n13;
            let assign79440_body28_e121587: f64 = (locals.var_chi).abs();
            let assign79440_body28_e121589: f64 = if assign79440_body28_e121587 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1850 = assign79440_body28_e121589;
            let (assign79440_body29_e121619, assign79440_body29_e121619_d_n0, assign79440_body29_e121619_d_n2, assign79440_body29_e121619_d_n4, assign79440_body29_e121619_d_n5, assign79440_body29_e121619_d_n6, assign79440_body29_e121619_d_n7, assign79440_body29_e121619_d_n8, assign79440_body29_e121619_d_n9, assign79440_body29_e121619_d_n10, assign79440_body29_e121619_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body29_e121597: f64 = (locals.var_chi * locals.var_chi);
        let assign79440_body29_e121599: f64 = (assign79440_body29_e121597 / 2.0);
        let assign79440_body29_e121603: f64 = (locals.var_chi / 3.0);
        let assign79440_body29_e121607: f64 = (locals.var_chi / 4.0);
        let assign79440_body29_e121611: f64 = (locals.var_chi / 5.0);
        let assign79440_body29_e121612: f64 = (1.0 + assign79440_body29_e121611);
        let assign79440_body29_e121613: f64 = (assign79440_body29_e121607 * assign79440_body29_e121612);
        let assign79440_body29_e121614: f64 = (1.0 + assign79440_body29_e121613);
        let assign79440_body29_e121615: f64 = (assign79440_body29_e121603 * assign79440_body29_e121614);
        let assign79440_body29_e121616: f64 = (1.0 + assign79440_body29_e121615);
        let assign79440_body29_e121617: f64 = (assign79440_body29_e121599 * assign79440_body29_e121616);
        (assign79440_body29_e121617, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn0 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn0 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn2 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn2 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn4 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn4 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn5 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn5 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn6 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn6 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn7 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn7 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn8 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn8 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn9 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn9 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn10 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn10 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign79440_body29_e121616) + (assign79440_body29_e121599 * (((locals.var_chi_dn13 / 3.0) * assign79440_body29_e121614) + (assign79440_body29_e121603 * (((locals.var_chi_dn13 / 4.0) * assign79440_body29_e121612) + (assign79440_body29_e121607 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign79440_body29_e121619;
            locals.var_t0_dn0 = assign79440_body29_e121619_d_n0;
            locals.var_t0_dn2 = assign79440_body29_e121619_d_n2;
            locals.var_t0_dn4 = assign79440_body29_e121619_d_n4;
            locals.var_t0_dn5 = assign79440_body29_e121619_d_n5;
            locals.var_t0_dn6 = assign79440_body29_e121619_d_n6;
            locals.var_t0_dn7 = assign79440_body29_e121619_d_n7;
            locals.var_t0_dn8 = assign79440_body29_e121619_d_n8;
            locals.var_t0_dn9 = assign79440_body29_e121619_d_n9;
            locals.var_t0_dn10 = assign79440_body29_e121619_d_n10;
            locals.var_t0_dn13 = assign79440_body29_e121619_d_n13;
            let (assign79440_body30_e121645, assign79440_body30_e121645_d_n0, assign79440_body30_e121645_d_n2, assign79440_body30_e121645_d_n4, assign79440_body30_e121645_d_n5, assign79440_body30_e121645_d_n6, assign79440_body30_e121645_d_n7, assign79440_body30_e121645_d_n8, assign79440_body30_e121645_d_n9, assign79440_body30_e121645_d_n10, assign79440_body30_e121645_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body30_e121629: f64 = (locals.var_chi / 2.0);
        let assign79440_body30_e121633: f64 = (locals.var_chi / 3.0);
        let assign79440_body30_e121637: f64 = (locals.var_chi / 4.0);
        let assign79440_body30_e121638: f64 = (1.0 + assign79440_body30_e121637);
        let assign79440_body30_e121639: f64 = (assign79440_body30_e121633 * assign79440_body30_e121638);
        let assign79440_body30_e121640: f64 = (1.0 + assign79440_body30_e121639);
        let assign79440_body30_e121641: f64 = (assign79440_body30_e121629 * assign79440_body30_e121640);
        let assign79440_body30_e121642: f64 = (1.0 + assign79440_body30_e121641);
        let assign79440_body30_e121643: f64 = (locals.var_chi * assign79440_body30_e121642);
        (assign79440_body30_e121643, ((locals.var_chi_dn0 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn0 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn2 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn4 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn5 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn6 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn7 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn8 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn9 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn10 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign79440_body30_e121642) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign79440_body30_e121640) + (assign79440_body30_e121629 * (((locals.var_chi_dn13 / 3.0) * assign79440_body30_e121638) + (assign79440_body30_e121633 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body30_e121645;
            locals.var_t1_dn0 = assign79440_body30_e121645_d_n0;
            locals.var_t1_dn2 = assign79440_body30_e121645_d_n2;
            locals.var_t1_dn4 = assign79440_body30_e121645_d_n4;
            locals.var_t1_dn5 = assign79440_body30_e121645_d_n5;
            locals.var_t1_dn6 = assign79440_body30_e121645_d_n6;
            locals.var_t1_dn7 = assign79440_body30_e121645_d_n7;
            locals.var_t1_dn8 = assign79440_body30_e121645_d_n8;
            locals.var_t1_dn9 = assign79440_body30_e121645_d_n9;
            locals.var_t1_dn10 = assign79440_body30_e121645_d_n10;
            locals.var_t1_dn13 = assign79440_body30_e121645_d_n13;
            let (assign79440_body31_e121655, assign79440_body31_e121655_d_n0, assign79440_body31_e121655_d_n2, assign79440_body31_e121655_d_n4, assign79440_body31_e121655_d_n5, assign79440_body31_e121655_d_n6, assign79440_body31_e121655_d_n7, assign79440_body31_e121655_d_n8, assign79440_body31_e121655_d_n9, assign79440_body31_e121655_d_n10, assign79440_body31_e121655_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body31_e121653: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign79440_body31_e121653, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body31_e121655;
            locals.var_fs01_dn0 = assign79440_body31_e121655_d_n0;
            locals.var_fs01_dn2 = assign79440_body31_e121655_d_n2;
            locals.var_fs01_dn4 = assign79440_body31_e121655_d_n4;
            locals.var_fs01_dn5 = assign79440_body31_e121655_d_n5;
            locals.var_fs01_dn6 = assign79440_body31_e121655_d_n6;
            locals.var_fs01_dn7 = assign79440_body31_e121655_d_n7;
            locals.var_fs01_dn8 = assign79440_body31_e121655_d_n8;
            locals.var_fs01_dn9 = assign79440_body31_e121655_d_n9;
            locals.var_fs01_dn10 = assign79440_body31_e121655_d_n10;
            locals.var_fs01_dn13 = assign79440_body31_e121655_d_n13;
            let (assign79440_body32_e121667, assign79440_body32_e121667_d_n0, assign79440_body32_e121667_d_n2, assign79440_body32_e121667_d_n4, assign79440_body32_e121667_d_n5, assign79440_body32_e121667_d_n6, assign79440_body32_e121667_d_n7, assign79440_body32_e121667_d_n8, assign79440_body32_e121667_d_n9, assign79440_body32_e121667_d_n10, assign79440_body32_e121667_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        let assign79440_body32_e121663: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign79440_body32_e121665: f64 = (assign79440_body32_e121663 * locals.var_beta);
        (assign79440_body32_e121665, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign79440_body32_e121663 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body32_e121667;
            locals.var_fs01_dps0_dn0 = assign79440_body32_e121667_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body32_e121667_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body32_e121667_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body32_e121667_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body32_e121667_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body32_e121667_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body32_e121667_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body32_e121667_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body32_e121667_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body32_e121667_d_n13;
            let assign79440_body33_e121669: f64 = (locals.var_chi).abs();
            let assign79440_body33_e121671: f64 = if assign79440_body33_e121669 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1851 = assign79440_body33_e121671;
            let (assign79440_body35_e121702, assign79440_body35_e121702_d_n0, assign79440_body35_e121702_d_n2, assign79440_body35_e121702_d_n4, assign79440_body35_e121702_d_n5, assign79440_body35_e121702_d_n6, assign79440_body35_e121702_d_n7, assign79440_body35_e121702_d_n8, assign79440_body35_e121702_d_n9, assign79440_body35_e121702_d_n10, assign79440_body35_e121702_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body35_e121700: f64 = (locals.var_chi).exp();
        (assign79440_body35_e121700, (assign79440_body35_e121700 * locals.var_chi_dn0), (assign79440_body35_e121700 * locals.var_chi_dn2), (assign79440_body35_e121700 * locals.var_chi_dn4), (assign79440_body35_e121700 * locals.var_chi_dn5), (assign79440_body35_e121700 * locals.var_chi_dn6), (assign79440_body35_e121700 * locals.var_chi_dn7), (assign79440_body35_e121700 * locals.var_chi_dn8), (assign79440_body35_e121700 * locals.var_chi_dn9), (assign79440_body35_e121700 * locals.var_chi_dn10), (assign79440_body35_e121700 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign79440_body35_e121702;
            locals.var_exp_chi_dn0 = assign79440_body35_e121702_d_n0;
            locals.var_exp_chi_dn2 = assign79440_body35_e121702_d_n2;
            locals.var_exp_chi_dn4 = assign79440_body35_e121702_d_n4;
            locals.var_exp_chi_dn5 = assign79440_body35_e121702_d_n5;
            locals.var_exp_chi_dn6 = assign79440_body35_e121702_d_n6;
            locals.var_exp_chi_dn7 = assign79440_body35_e121702_d_n7;
            locals.var_exp_chi_dn8 = assign79440_body35_e121702_d_n8;
            locals.var_exp_chi_dn9 = assign79440_body35_e121702_d_n9;
            locals.var_exp_chi_dn10 = assign79440_body35_e121702_d_n10;
            locals.var_exp_chi_dn13 = assign79440_body35_e121702_d_n13;
            let (assign79440_body36_e121715, assign79440_body36_e121715_d_n0, assign79440_body36_e121715_d_n2, assign79440_body36_e121715_d_n4, assign79440_body36_e121715_d_n5, assign79440_body36_e121715_d_n6, assign79440_body36_e121715_d_n7, assign79440_body36_e121715_d_n8, assign79440_body36_e121715_d_n9, assign79440_body36_e121715_d_n10, assign79440_body36_e121715_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body36_e121713: f64 = (locals.var_exp_chi - 1.0);
        (assign79440_body36_e121713, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign79440_body36_e121715;
            locals.var_t1_dn0 = assign79440_body36_e121715_d_n0;
            locals.var_t1_dn2 = assign79440_body36_e121715_d_n2;
            locals.var_t1_dn4 = assign79440_body36_e121715_d_n4;
            locals.var_t1_dn5 = assign79440_body36_e121715_d_n5;
            locals.var_t1_dn6 = assign79440_body36_e121715_d_n6;
            locals.var_t1_dn7 = assign79440_body36_e121715_d_n7;
            locals.var_t1_dn8 = assign79440_body36_e121715_d_n8;
            locals.var_t1_dn9 = assign79440_body36_e121715_d_n9;
            locals.var_t1_dn10 = assign79440_body36_e121715_d_n10;
            locals.var_t1_dn13 = assign79440_body36_e121715_d_n13;
            let (assign79440_body37_e121730, assign79440_body37_e121730_d_n0, assign79440_body37_e121730_d_n2, assign79440_body37_e121730_d_n4, assign79440_body37_e121730_d_n5, assign79440_body37_e121730_d_n6, assign79440_body37_e121730_d_n7, assign79440_body37_e121730_d_n8, assign79440_body37_e121730_d_n9, assign79440_body37_e121730_d_n10, assign79440_body37_e121730_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body37_e121727: f64 = (locals.var_t1 - locals.var_chi);
        let assign79440_body37_e121728: f64 = (locals.var_cfs1 * assign79440_body37_e121727);
        (assign79440_body37_e121728, ((locals.var_cfs1_dn0 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign79440_body37_e121727) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body37_e121730;
            locals.var_fs01_dn0 = assign79440_body37_e121730_d_n0;
            locals.var_fs01_dn2 = assign79440_body37_e121730_d_n2;
            locals.var_fs01_dn4 = assign79440_body37_e121730_d_n4;
            locals.var_fs01_dn5 = assign79440_body37_e121730_d_n5;
            locals.var_fs01_dn6 = assign79440_body37_e121730_d_n6;
            locals.var_fs01_dn7 = assign79440_body37_e121730_d_n7;
            locals.var_fs01_dn8 = assign79440_body37_e121730_d_n8;
            locals.var_fs01_dn9 = assign79440_body37_e121730_d_n9;
            locals.var_fs01_dn10 = assign79440_body37_e121730_d_n10;
            locals.var_fs01_dn13 = assign79440_body37_e121730_d_n13;
            let (assign79440_body38_e121745, assign79440_body38_e121745_d_n0, assign79440_body38_e121745_d_n2, assign79440_body38_e121745_d_n4, assign79440_body38_e121745_d_n5, assign79440_body38_e121745_d_n6, assign79440_body38_e121745_d_n7, assign79440_body38_e121745_d_n8, assign79440_body38_e121745_d_n9, assign79440_body38_e121745_d_n10, assign79440_body38_e121745_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 != 0.0)) {
        let assign79440_body38_e121741: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign79440_body38_e121743: f64 = (assign79440_body38_e121741 * locals.var_t1);
        (assign79440_body38_e121743, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign79440_body38_e121741 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body38_e121745;
            locals.var_fs01_dps0_dn0 = assign79440_body38_e121745_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body38_e121745_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body38_e121745_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body38_e121745_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body38_e121745_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body38_e121745_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body38_e121745_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body38_e121745_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body38_e121745_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body38_e121745_d_n13;
            let (assign79440_body40_e121780, assign79440_body40_e121780_d_n0, assign79440_body40_e121780_d_n2, assign79440_body40_e121780_d_n4, assign79440_body40_e121780_d_n5, assign79440_body40_e121780_d_n6, assign79440_body40_e121780_d_n7, assign79440_body40_e121780_d_n8, assign79440_body40_e121780_d_n9, assign79440_body40_e121780_d_n10, assign79440_body40_e121780_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body40_e121777: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign79440_body40_e121778: f64 = (assign79440_body40_e121777).exp();
        (assign79440_body40_e121778, (assign79440_body40_e121778 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign79440_body40_e121778 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign79440_body40_e121778 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign79440_body40_e121778 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign79440_body40_e121778 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign79440_body40_e121778 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign79440_body40_e121778 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign79440_body40_e121778 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign79440_body40_e121778 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign79440_body40_e121778 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign79440_body40_e121780;
            locals.var_exp_bps0_dn0 = assign79440_body40_e121780_d_n0;
            locals.var_exp_bps0_dn2 = assign79440_body40_e121780_d_n2;
            locals.var_exp_bps0_dn4 = assign79440_body40_e121780_d_n4;
            locals.var_exp_bps0_dn5 = assign79440_body40_e121780_d_n5;
            locals.var_exp_bps0_dn6 = assign79440_body40_e121780_d_n6;
            locals.var_exp_bps0_dn7 = assign79440_body40_e121780_d_n7;
            locals.var_exp_bps0_dn8 = assign79440_body40_e121780_d_n8;
            locals.var_exp_bps0_dn9 = assign79440_body40_e121780_d_n9;
            locals.var_exp_bps0_dn10 = assign79440_body40_e121780_d_n10;
            locals.var_exp_bps0_dn13 = assign79440_body40_e121780_d_n13;
            let (assign79440_body41_e121800, assign79440_body41_e121800_d_n0, assign79440_body41_e121800_d_n2, assign79440_body41_e121800_d_n4, assign79440_body41_e121800_d_n5, assign79440_body41_e121800_d_n6, assign79440_body41_e121800_d_n7, assign79440_body41_e121800_d_n8, assign79440_body41_e121800_d_n9, assign79440_body41_e121800_d_n10, assign79440_body41_e121800_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body41_e121795: f64 = (locals.var_chi + 1.0);
        let assign79440_body41_e121796: f64 = (locals.var_exp_bvbs * assign79440_body41_e121795);
        let assign79440_body41_e121797: f64 = (locals.var_exp_bps0 - assign79440_body41_e121796);
        let assign79440_body41_e121798: f64 = (locals.var_cnst1over * assign79440_body41_e121797);
        (assign79440_body41_e121798, ((locals.var_cnst1over_dn0 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign79440_body41_e121797) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign79440_body41_e121795) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign79440_body41_e121800;
            locals.var_fs01_dn0 = assign79440_body41_e121800_d_n0;
            locals.var_fs01_dn2 = assign79440_body41_e121800_d_n2;
            locals.var_fs01_dn4 = assign79440_body41_e121800_d_n4;
            locals.var_fs01_dn5 = assign79440_body41_e121800_d_n5;
            locals.var_fs01_dn6 = assign79440_body41_e121800_d_n6;
            locals.var_fs01_dn7 = assign79440_body41_e121800_d_n7;
            locals.var_fs01_dn8 = assign79440_body41_e121800_d_n8;
            locals.var_fs01_dn9 = assign79440_body41_e121800_d_n9;
            locals.var_fs01_dn10 = assign79440_body41_e121800_d_n10;
            locals.var_fs01_dn13 = assign79440_body41_e121800_d_n13;
            let (assign79440_body42_e121818, assign79440_body42_e121818_d_n0, assign79440_body42_e121818_d_n2, assign79440_body42_e121818_d_n4, assign79440_body42_e121818_d_n5, assign79440_body42_e121818_d_n6, assign79440_body42_e121818_d_n7, assign79440_body42_e121818_d_n8, assign79440_body42_e121818_d_n9, assign79440_body42_e121818_d_n10, assign79440_body42_e121818_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1850 == 0.0)) && (locals.var_guard1851 == 0.0)) {
        let assign79440_body42_e121812: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign79440_body42_e121815: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign79440_body42_e121816: f64 = (assign79440_body42_e121812 * assign79440_body42_e121815);
        (assign79440_body42_e121816, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign79440_body42_e121815) + (assign79440_body42_e121812 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign79440_body42_e121818;
            locals.var_fs01_dps0_dn0 = assign79440_body42_e121818_d_n0;
            locals.var_fs01_dps0_dn2 = assign79440_body42_e121818_d_n2;
            locals.var_fs01_dps0_dn4 = assign79440_body42_e121818_d_n4;
            locals.var_fs01_dps0_dn5 = assign79440_body42_e121818_d_n5;
            locals.var_fs01_dps0_dn6 = assign79440_body42_e121818_d_n6;
            locals.var_fs01_dps0_dn7 = assign79440_body42_e121818_d_n7;
            locals.var_fs01_dps0_dn8 = assign79440_body42_e121818_d_n8;
            locals.var_fs01_dps0_dn9 = assign79440_body42_e121818_d_n9;
            locals.var_fs01_dps0_dn10 = assign79440_body42_e121818_d_n10;
            locals.var_fs01_dps0_dn13 = assign79440_body42_e121818_d_n13;
            let assign79440_body43_e121821: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1852 = assign79440_body43_e121821;
            let (assign79440_body44_e121832, assign79440_body44_e121832_d_n0, assign79440_body44_e121832_d_n2, assign79440_body44_e121832_d_n4, assign79440_body44_e121832_d_n5, assign79440_body44_e121832_d_n6, assign79440_body44_e121832_d_n7, assign79440_body44_e121832_d_n8, assign79440_body44_e121832_d_n9, assign79440_body44_e121832_d_n10, assign79440_body44_e121832_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 != 0.0)) {
        let assign79440_body44_e121829: f64 = (locals.var_fbsq__blk1770 + locals.var_fs01);
        let assign79440_body44_e121830: f64 = (assign79440_body44_e121829).sqrt();
        (assign79440_body44_e121830, ((locals.var_fbsq__blk1770_dn0 + locals.var_fs01_dn0) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn2 + locals.var_fs01_dn2) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn4 + locals.var_fs01_dn4) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn5 + locals.var_fs01_dn5) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn6 + locals.var_fs01_dn6) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn7 + locals.var_fs01_dn7) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn8 + locals.var_fs01_dn8) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn9 + locals.var_fs01_dn9) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn10 + locals.var_fs01_dn10) / (2.0 * assign79440_body44_e121830)), ((locals.var_fbsq__blk1770_dn13 + locals.var_fs01_dn13) / (2.0 * assign79440_body44_e121830)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body44_e121832;
            locals.var_fs02_dn0 = assign79440_body44_e121832_d_n0;
            locals.var_fs02_dn2 = assign79440_body44_e121832_d_n2;
            locals.var_fs02_dn4 = assign79440_body44_e121832_d_n4;
            locals.var_fs02_dn5 = assign79440_body44_e121832_d_n5;
            locals.var_fs02_dn6 = assign79440_body44_e121832_d_n6;
            locals.var_fs02_dn7 = assign79440_body44_e121832_d_n7;
            locals.var_fs02_dn8 = assign79440_body44_e121832_d_n8;
            locals.var_fs02_dn9 = assign79440_body44_e121832_d_n9;
            locals.var_fs02_dn10 = assign79440_body44_e121832_d_n10;
            locals.var_fs02_dn13 = assign79440_body44_e121832_d_n13;
            let (assign79440_body45_e121846, assign79440_body45_e121846_d_n0, assign79440_body45_e121846_d_n2, assign79440_body45_e121846_d_n4, assign79440_body45_e121846_d_n5, assign79440_body45_e121846_d_n6, assign79440_body45_e121846_d_n7, assign79440_body45_e121846_d_n8, assign79440_body45_e121846_d_n9, assign79440_body45_e121846_d_n10, assign79440_body45_e121846_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 != 0.0)) {
        let assign79440_body45_e121841: f64 = (locals.var_fbsq_dpss__blk1771 + locals.var_fs01_dps0);
        let assign79440_body45_e121842: f64 = (0.5 * assign79440_body45_e121841);
        let assign79440_body45_e121844: f64 = (assign79440_body45_e121842 / locals.var_fs02);
        (assign79440_body45_e121844, ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1771_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign79440_body45_e121842 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body45_e121846;
            locals.var_fs02_dps0_dn0 = assign79440_body45_e121846_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body45_e121846_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body45_e121846_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body45_e121846_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body45_e121846_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body45_e121846_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body45_e121846_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body45_e121846_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body45_e121846_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body45_e121846_d_n13;
            let assign79440_body46_e121849: f64 = if locals.var_fbsq__blk1770 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1853 = assign79440_body46_e121849;
            let (assign79440_body47_e121861, assign79440_body47_e121861_d_n0, assign79440_body47_e121861_d_n2, assign79440_body47_e121861_d_n4, assign79440_body47_e121861_d_n5, assign79440_body47_e121861_d_n6, assign79440_body47_e121861_d_n7, assign79440_body47_e121861_d_n8, assign79440_body47_e121861_d_n9, assign79440_body47_e121861_d_n10, assign79440_body47_e121861_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 != 0.0)) {
        let assign79440_body47_e121859: f64 = (locals.var_fbsq__blk1770).sqrt();
        (assign79440_body47_e121859, (locals.var_fbsq__blk1770_dn0 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn2 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn4 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn5 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn6 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn7 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn8 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn9 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn10 / (2.0 * assign79440_body47_e121859)), (locals.var_fbsq__blk1770_dn13 / (2.0 * assign79440_body47_e121859)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body47_e121861;
            locals.var_fs02_dn0 = assign79440_body47_e121861_d_n0;
            locals.var_fs02_dn2 = assign79440_body47_e121861_d_n2;
            locals.var_fs02_dn4 = assign79440_body47_e121861_d_n4;
            locals.var_fs02_dn5 = assign79440_body47_e121861_d_n5;
            locals.var_fs02_dn6 = assign79440_body47_e121861_d_n6;
            locals.var_fs02_dn7 = assign79440_body47_e121861_d_n7;
            locals.var_fs02_dn8 = assign79440_body47_e121861_d_n8;
            locals.var_fs02_dn9 = assign79440_body47_e121861_d_n9;
            locals.var_fs02_dn10 = assign79440_body47_e121861_d_n10;
            locals.var_fs02_dn13 = assign79440_body47_e121861_d_n13;
            let (assign79440_body48_e121876, assign79440_body48_e121876_d_n0, assign79440_body48_e121876_d_n2, assign79440_body48_e121876_d_n4, assign79440_body48_e121876_d_n5, assign79440_body48_e121876_d_n6, assign79440_body48_e121876_d_n7, assign79440_body48_e121876_d_n8, assign79440_body48_e121876_d_n9, assign79440_body48_e121876_d_n10, assign79440_body48_e121876_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 != 0.0)) {
        let assign79440_body48_e121872: f64 = (0.5 * locals.var_fbsq_dpss__blk1771);
        let assign79440_body48_e121874: f64 = (assign79440_body48_e121872 / locals.var_fs02);
        (assign79440_body48_e121874, ((((0.5 * locals.var_fbsq_dpss__blk1771_dn0) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn2) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn4) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn5) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn6) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn7) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn8) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn9) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn10) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1771_dn13) * locals.var_fs02) - (assign79440_body48_e121872 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body48_e121876;
            locals.var_fs02_dps0_dn0 = assign79440_body48_e121876_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body48_e121876_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body48_e121876_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body48_e121876_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body48_e121876_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body48_e121876_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body48_e121876_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body48_e121876_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body48_e121876_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body48_e121876_d_n13;
            let (assign79440_body49_e121888, assign79440_body49_e121888_d_n0, assign79440_body49_e121888_d_n2, assign79440_body49_e121888_d_n4, assign79440_body49_e121888_d_n5, assign79440_body49_e121888_d_n6, assign79440_body49_e121888_d_n7, assign79440_body49_e121888_d_n8, assign79440_body49_e121888_d_n9, assign79440_body49_e121888_d_n10, assign79440_body49_e121888_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body49_e121888;
            locals.var_fs02_dn0 = assign79440_body49_e121888_d_n0;
            locals.var_fs02_dn2 = assign79440_body49_e121888_d_n2;
            locals.var_fs02_dn4 = assign79440_body49_e121888_d_n4;
            locals.var_fs02_dn5 = assign79440_body49_e121888_d_n5;
            locals.var_fs02_dn6 = assign79440_body49_e121888_d_n6;
            locals.var_fs02_dn7 = assign79440_body49_e121888_d_n7;
            locals.var_fs02_dn8 = assign79440_body49_e121888_d_n8;
            locals.var_fs02_dn9 = assign79440_body49_e121888_d_n9;
            locals.var_fs02_dn10 = assign79440_body49_e121888_d_n10;
            locals.var_fs02_dn13 = assign79440_body49_e121888_d_n13;
            let (assign79440_body50_e121900, assign79440_body50_e121900_d_n0, assign79440_body50_e121900_d_n2, assign79440_body50_e121900_d_n4, assign79440_body50_e121900_d_n5, assign79440_body50_e121900_d_n6, assign79440_body50_e121900_d_n7, assign79440_body50_e121900_d_n8, assign79440_body50_e121900_d_n9, assign79440_body50_e121900_d_n10, assign79440_body50_e121900_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1852 == 0.0)) && (locals.var_guard1853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body50_e121900;
            locals.var_fs02_dps0_dn0 = assign79440_body50_e121900_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body50_e121900_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body50_e121900_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body50_e121900_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body50_e121900_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body50_e121900_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body50_e121900_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body50_e121900_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body50_e121900_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body50_e121900_d_n13;
            let (assign79440_body51_e121914, assign79440_body51_e121914_d_n0, assign79440_body51_e121914_d_n2, assign79440_body51_e121914_d_n4, assign79440_body51_e121914_d_n5, assign79440_body51_e121914_d_n6, assign79440_body51_e121914_d_n7, assign79440_body51_e121914_d_n8, assign79440_body51_e121914_d_n9, assign79440_body51_e121914_d_n10, assign79440_body51_e121914_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79440_body51_e121910,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body51_e121909: f64 = (-1.0);
                (assign79440_body51_e121909,)
            }
        };
        let assign79440_body51_e121912: f64 = (assign79440_body51_e121910 * locals.var_fs02);
        (assign79440_body51_e121912, (assign79440_body51_e121910 * locals.var_fs02_dn0), (assign79440_body51_e121910 * locals.var_fs02_dn2), (assign79440_body51_e121910 * locals.var_fs02_dn4), (assign79440_body51_e121910 * locals.var_fs02_dn5), (assign79440_body51_e121910 * locals.var_fs02_dn6), (assign79440_body51_e121910 * locals.var_fs02_dn7), (assign79440_body51_e121910 * locals.var_fs02_dn8), (assign79440_body51_e121910 * locals.var_fs02_dn9), (assign79440_body51_e121910 * locals.var_fs02_dn10), (assign79440_body51_e121910 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign79440_body51_e121914;
            locals.var_fs02_dn0 = assign79440_body51_e121914_d_n0;
            locals.var_fs02_dn2 = assign79440_body51_e121914_d_n2;
            locals.var_fs02_dn4 = assign79440_body51_e121914_d_n4;
            locals.var_fs02_dn5 = assign79440_body51_e121914_d_n5;
            locals.var_fs02_dn6 = assign79440_body51_e121914_d_n6;
            locals.var_fs02_dn7 = assign79440_body51_e121914_d_n7;
            locals.var_fs02_dn8 = assign79440_body51_e121914_d_n8;
            locals.var_fs02_dn9 = assign79440_body51_e121914_d_n9;
            locals.var_fs02_dn10 = assign79440_body51_e121914_d_n10;
            locals.var_fs02_dn13 = assign79440_body51_e121914_d_n13;
            let (assign79440_body52_e121928, assign79440_body52_e121928_d_n0, assign79440_body52_e121928_d_n2, assign79440_body52_e121928_d_n4, assign79440_body52_e121928_d_n5, assign79440_body52_e121928_d_n6, assign79440_body52_e121928_d_n7, assign79440_body52_e121928_d_n8, assign79440_body52_e121928_d_n9, assign79440_body52_e121928_d_n10, assign79440_body52_e121928_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79440_body52_e121924,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body52_e121923: f64 = (-1.0);
                (assign79440_body52_e121923,)
            }
        };
        let assign79440_body52_e121926: f64 = (assign79440_body52_e121924 * locals.var_fs02_dps0);
        (assign79440_body52_e121926, (assign79440_body52_e121924 * locals.var_fs02_dps0_dn0), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn2), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn4), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn5), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn6), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn7), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn8), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn9), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn10), (assign79440_body52_e121924 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign79440_body52_e121928;
            locals.var_fs02_dps0_dn0 = assign79440_body52_e121928_d_n0;
            locals.var_fs02_dps0_dn2 = assign79440_body52_e121928_d_n2;
            locals.var_fs02_dps0_dn4 = assign79440_body52_e121928_d_n4;
            locals.var_fs02_dps0_dn5 = assign79440_body52_e121928_d_n5;
            locals.var_fs02_dps0_dn6 = assign79440_body52_e121928_d_n6;
            locals.var_fs02_dps0_dn7 = assign79440_body52_e121928_d_n7;
            locals.var_fs02_dps0_dn8 = assign79440_body52_e121928_d_n8;
            locals.var_fs02_dps0_dn9 = assign79440_body52_e121928_d_n9;
            locals.var_fs02_dps0_dn10 = assign79440_body52_e121928_d_n10;
            locals.var_fs02_dps0_dn13 = assign79440_body52_e121928_d_n13;
            let (assign79440_body53_e121941, assign79440_body53_e121941_d_n0, assign79440_body53_e121941_d_n2, assign79440_body53_e121941_d_n4, assign79440_body53_e121941_d_n5, assign79440_body53_e121941_d_n6, assign79440_body53_e121941_d_n7, assign79440_body53_e121941_d_n8, assign79440_body53_e121941_d_n9, assign79440_body53_e121941_d_n10, assign79440_body53_e121941_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body53_e121933: f64 = (-locals.var_vgpld);
        let assign79440_body53_e121935: f64 = (assign79440_body53_e121933 + locals.var_ps0ld);
        let assign79440_body53_e121938: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign79440_body53_e121939: f64 = (assign79440_body53_e121935 + assign79440_body53_e121938);
        (assign79440_body53_e121939, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign79440_body53_e121941;
            locals.var_fs0_dn0 = assign79440_body53_e121941_d_n0;
            locals.var_fs0_dn2 = assign79440_body53_e121941_d_n2;
            locals.var_fs0_dn4 = assign79440_body53_e121941_d_n4;
            locals.var_fs0_dn5 = assign79440_body53_e121941_d_n5;
            locals.var_fs0_dn6 = assign79440_body53_e121941_d_n6;
            locals.var_fs0_dn7 = assign79440_body53_e121941_d_n7;
            locals.var_fs0_dn8 = assign79440_body53_e121941_d_n8;
            locals.var_fs0_dn9 = assign79440_body53_e121941_d_n9;
            locals.var_fs0_dn10 = assign79440_body53_e121941_d_n10;
            locals.var_fs0_dn13 = assign79440_body53_e121941_d_n13;
            let (assign79440_body54_e121951, assign79440_body54_e121951_d_n0, assign79440_body54_e121951_d_n2, assign79440_body54_e121951_d_n4, assign79440_body54_e121951_d_n5, assign79440_body54_e121951_d_n6, assign79440_body54_e121951_d_n7, assign79440_body54_e121951_d_n8, assign79440_body54_e121951_d_n9, assign79440_body54_e121951_d_n10, assign79440_body54_e121951_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body54_e121948: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign79440_body54_e121949: f64 = (1.0 + assign79440_body54_e121948);
        (assign79440_body54_e121949, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign79440_body54_e121951;
            locals.var_fs0_dps0_dn0 = assign79440_body54_e121951_d_n0;
            locals.var_fs0_dps0_dn2 = assign79440_body54_e121951_d_n2;
            locals.var_fs0_dps0_dn4 = assign79440_body54_e121951_d_n4;
            locals.var_fs0_dps0_dn5 = assign79440_body54_e121951_d_n5;
            locals.var_fs0_dps0_dn6 = assign79440_body54_e121951_d_n6;
            locals.var_fs0_dps0_dn7 = assign79440_body54_e121951_d_n7;
            locals.var_fs0_dps0_dn8 = assign79440_body54_e121951_d_n8;
            locals.var_fs0_dps0_dn9 = assign79440_body54_e121951_d_n9;
            locals.var_fs0_dps0_dn10 = assign79440_body54_e121951_d_n10;
            locals.var_fs0_dps0_dn13 = assign79440_body54_e121951_d_n13;
            let assign79440_body55_e121954: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1854 = assign79440_body55_e121954;
            let (assign79440_body56_e121964,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 != 0.0)) {
        let assign79440_body56_e121962: f64 = (locals.var_lp_s0_max + 1.0);
        (assign79440_body56_e121962,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79440_body56_e121964;
            let (assign79440_body57_e121976, assign79440_body57_e121976_d_n0, assign79440_body57_e121976_d_n2, assign79440_body57_e121976_d_n4, assign79440_body57_e121976_d_n5, assign79440_body57_e121976_d_n6, assign79440_body57_e121976_d_n7, assign79440_body57_e121976_d_n8, assign79440_body57_e121976_d_n9, assign79440_body57_e121976_d_n10, assign79440_body57_e121976_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body57_e121972: f64 = (-locals.var_fs0);
        let assign79440_body57_e121974: f64 = (assign79440_body57_e121972 / locals.var_fs0_dps0);
        (assign79440_body57_e121974, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign79440_body57_e121972 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79440_body57_e121976;
            locals.var_dps0_dn0 = assign79440_body57_e121976_d_n0;
            locals.var_dps0_dn2 = assign79440_body57_e121976_d_n2;
            locals.var_dps0_dn4 = assign79440_body57_e121976_d_n4;
            locals.var_dps0_dn5 = assign79440_body57_e121976_d_n5;
            locals.var_dps0_dn6 = assign79440_body57_e121976_d_n6;
            locals.var_dps0_dn7 = assign79440_body57_e121976_d_n7;
            locals.var_dps0_dn8 = assign79440_body57_e121976_d_n8;
            locals.var_dps0_dn9 = assign79440_body57_e121976_d_n9;
            locals.var_dps0_dn10 = assign79440_body57_e121976_d_n10;
            locals.var_dps0_dn13 = assign79440_body57_e121976_d_n13;
            let (assign79440_body58_e121998, assign79440_body58_e121998_d_n0, assign79440_body58_e121998_d_n2, assign79440_body58_e121998_d_n4, assign79440_body58_e121998_d_n5, assign79440_body58_e121998_d_n6, assign79440_body58_e121998_d_n7, assign79440_body58_e121998_d_n8, assign79440_body58_e121998_d_n9, assign79440_body58_e121998_d_n10, assign79440_body58_e121998_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body58_e121985: f64 = (0.5 * 0.1);
        let assign79440_body58_e121989: f64 = (locals.var_ps0ld).abs();
        let (assign79440_body58_e121994, assign79440_body58_e121994_d_n0, assign79440_body58_e121994_d_n2, assign79440_body58_e121994_d_n4, assign79440_body58_e121994_d_n5, assign79440_body58_e121994_d_n6, assign79440_body58_e121994_d_n7, assign79440_body58_e121994_d_n8, assign79440_body58_e121994_d_n9, assign79440_body58_e121994_d_n10, assign79440_body58_e121994_d_n13,) = {
            if (1.0 >= assign79440_body58_e121989) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign79440_body58_e121993: f64 = (locals.var_ps0ld).abs();
                (assign79440_body58_e121993, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign79440_body58_e121995: f64 = (1.0 + assign79440_body58_e121994);
        let assign79440_body58_e121996: f64 = (assign79440_body58_e121985 * assign79440_body58_e121995);
        (assign79440_body58_e121996, (assign79440_body58_e121985 * assign79440_body58_e121994_d_n0), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n2), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n4), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n5), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n6), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n7), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n8), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n9), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n10), (assign79440_body58_e121985 * assign79440_body58_e121994_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign79440_body58_e121998;
            locals.var_dplim_dn0 = assign79440_body58_e121998_d_n0;
            locals.var_dplim_dn2 = assign79440_body58_e121998_d_n2;
            locals.var_dplim_dn4 = assign79440_body58_e121998_d_n4;
            locals.var_dplim_dn5 = assign79440_body58_e121998_d_n5;
            locals.var_dplim_dn6 = assign79440_body58_e121998_d_n6;
            locals.var_dplim_dn7 = assign79440_body58_e121998_d_n7;
            locals.var_dplim_dn8 = assign79440_body58_e121998_d_n8;
            locals.var_dplim_dn9 = assign79440_body58_e121998_d_n9;
            locals.var_dplim_dn10 = assign79440_body58_e121998_d_n10;
            locals.var_dplim_dn13 = assign79440_body58_e121998_d_n13;
            let assign79440_body59_e122000: f64 = (locals.var_dps0).abs();
            let assign79440_body59_e122002: f64 = if assign79440_body59_e122000 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1855 = assign79440_body59_e122002;
            let (assign79440_body60_e122021, assign79440_body60_e122021_d_n0, assign79440_body60_e122021_d_n2, assign79440_body60_e122021_d_n4, assign79440_body60_e122021_d_n5, assign79440_body60_e122021_d_n6, assign79440_body60_e122021_d_n7, assign79440_body60_e122021_d_n8, assign79440_body60_e122021_d_n9, assign79440_body60_e122021_d_n10, assign79440_body60_e122021_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) && (locals.var_guard1855 != 0.0)) {
        let (assign79440_body60_e122018,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign79440_body60_e122017: f64 = (-1.0);
                (assign79440_body60_e122017,)
            }
        };
        let assign79440_body60_e122019: f64 = (locals.var_dplim * assign79440_body60_e122018);
        (assign79440_body60_e122019, (locals.var_dplim_dn0 * assign79440_body60_e122018), (locals.var_dplim_dn2 * assign79440_body60_e122018), (locals.var_dplim_dn4 * assign79440_body60_e122018), (locals.var_dplim_dn5 * assign79440_body60_e122018), (locals.var_dplim_dn6 * assign79440_body60_e122018), (locals.var_dplim_dn7 * assign79440_body60_e122018), (locals.var_dplim_dn8 * assign79440_body60_e122018), (locals.var_dplim_dn9 * assign79440_body60_e122018), (locals.var_dplim_dn10 * assign79440_body60_e122018), (locals.var_dplim_dn13 * assign79440_body60_e122018),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign79440_body60_e122021;
            locals.var_dps0_dn0 = assign79440_body60_e122021_d_n0;
            locals.var_dps0_dn2 = assign79440_body60_e122021_d_n2;
            locals.var_dps0_dn4 = assign79440_body60_e122021_d_n4;
            locals.var_dps0_dn5 = assign79440_body60_e122021_d_n5;
            locals.var_dps0_dn6 = assign79440_body60_e122021_d_n6;
            locals.var_dps0_dn7 = assign79440_body60_e122021_d_n7;
            locals.var_dps0_dn8 = assign79440_body60_e122021_d_n8;
            locals.var_dps0_dn9 = assign79440_body60_e122021_d_n9;
            locals.var_dps0_dn10 = assign79440_body60_e122021_d_n10;
            locals.var_dps0_dn13 = assign79440_body60_e122021_d_n13;
            let (assign79440_body61_e122032, assign79440_body61_e122032_d_n0, assign79440_body61_e122032_d_n2, assign79440_body61_e122032_d_n4, assign79440_body61_e122032_d_n5, assign79440_body61_e122032_d_n6, assign79440_body61_e122032_d_n7, assign79440_body61_e122032_d_n8, assign79440_body61_e122032_d_n9, assign79440_body61_e122032_d_n10, assign79440_body61_e122032_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) {
        let assign79440_body61_e122030: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign79440_body61_e122030, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign79440_body61_e122032;
            locals.var_ps0ld_dn0 = assign79440_body61_e122032_d_n0;
            locals.var_ps0ld_dn2 = assign79440_body61_e122032_d_n2;
            locals.var_ps0ld_dn4 = assign79440_body61_e122032_d_n4;
            locals.var_ps0ld_dn5 = assign79440_body61_e122032_d_n5;
            locals.var_ps0ld_dn6 = assign79440_body61_e122032_d_n6;
            locals.var_ps0ld_dn7 = assign79440_body61_e122032_d_n7;
            locals.var_ps0ld_dn8 = assign79440_body61_e122032_d_n8;
            locals.var_ps0ld_dn9 = assign79440_body61_e122032_d_n9;
            locals.var_ps0ld_dn10 = assign79440_body61_e122032_d_n10;
            locals.var_ps0ld_dn13 = assign79440_body61_e122032_d_n13;
            let assign79440_body62_e122034: f64 = (locals.var_dps0).abs();
            let assign79440_body62_e122038: f64 = (locals.var_fs0).abs();
            let assign79440_body62_e122041: f64 = if ((assign79440_body62_e122034 <= 1e-12) && (assign79440_body62_e122038 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1856 = assign79440_body62_e122041;
            let (assign79440_body63_e122054,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) && (locals.var_guard1854 == 0.0)) && (locals.var_guard1856 != 0.0)) {
        let assign79440_body63_e122052: f64 = (locals.var_flg_conv + 2.0);
        (assign79440_body63_e122052,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign79440_body63_e122054;
            let (assign79440_body64_e122062,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79440_body64_e122060: f64 = (locals.var_lp_s0 + 1.0);
        (assign79440_body64_e122060,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79440_body64_e122062;
        }

    }

    pub(super) fn stamp_transient_block_277(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79460_e122085, assign79460_e122085_d_n0, assign79460_e122085_d_n2, assign79460_e122085_d_n4, assign79460_e122085_d_n5, assign79460_e122085_d_n6, assign79460_e122085_d_n7, assign79460_e122085_d_n8, assign79460_e122085_d_n9, assign79460_e122085_d_n10, assign79460_e122085_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let (assign79460_e122083, assign79460_e122083_d_n0, assign79460_e122083_d_n2, assign79460_e122083_d_n4, assign79460_e122083_d_n5, assign79460_e122083_d_n6, assign79460_e122083_d_n7, assign79460_e122083_d_n8, assign79460_e122083_d_n9, assign79460_e122083_d_n10, assign79460_e122083_d_n13,) = {
            if (locals.var_fbsq__blk1770 >= 0.0) {
                let (assign79460_e122078,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign79460_e122077: f64 = (-1.0);
                        (assign79460_e122077,)
                    }
                };
                let assign79460_e122080: f64 = (locals.var_fbsq__blk1770).sqrt();
                let assign79460_e122081: f64 = (assign79460_e122078 * assign79460_e122080);
                (assign79460_e122081, (assign79460_e122078 * (locals.var_fbsq__blk1770_dn0 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn2 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn4 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn5 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn6 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn7 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn8 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn9 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn10 / (2.0 * assign79460_e122080))), (assign79460_e122078 * (locals.var_fbsq__blk1770_dn13 / (2.0 * assign79460_e122080))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign79460_e122083, assign79460_e122083_d_n0, assign79460_e122083_d_n2, assign79460_e122083_d_n4, assign79460_e122083_d_n5, assign79460_e122083_d_n6, assign79460_e122083_d_n7, assign79460_e122083_d_n8, assign79460_e122083_d_n9, assign79460_e122083_d_n10, assign79460_e122083_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign79460_e122085;
        locals.var_fb_dn0 = assign79460_e122085_d_n0;
        locals.var_fb_dn2 = assign79460_e122085_d_n2;
        locals.var_fb_dn4 = assign79460_e122085_d_n4;
        locals.var_fb_dn5 = assign79460_e122085_d_n5;
        locals.var_fb_dn6 = assign79460_e122085_d_n6;
        locals.var_fb_dn7 = assign79460_e122085_d_n7;
        locals.var_fb_dn8 = assign79460_e122085_d_n8;
        locals.var_fb_dn9 = assign79460_e122085_d_n9;
        locals.var_fb_dn10 = assign79460_e122085_d_n10;
        locals.var_fb_dn13 = assign79460_e122085_d_n13;

        let (assign79470_e122093, assign79470_e122093_d_n0, assign79470_e122093_d_n2, assign79470_e122093_d_n4, assign79470_e122093_d_n5, assign79470_e122093_d_n6, assign79470_e122093_d_n7, assign79470_e122093_d_n8, assign79470_e122093_d_n9, assign79470_e122093_d_n10, assign79470_e122093_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79470_e122091: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79470_e122091, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1760, locals.var_wdld__blk1760_dn0, locals.var_wdld__blk1760_dn2, locals.var_wdld__blk1760_dn4, locals.var_wdld__blk1760_dn5, locals.var_wdld__blk1760_dn6, locals.var_wdld__blk1760_dn7, locals.var_wdld__blk1760_dn8, locals.var_wdld__blk1760_dn9, locals.var_wdld__blk1760_dn10, locals.var_wdld__blk1760_dn13,)
    }
};
        locals.var_wdld__blk1760 = assign79470_e122093;
        locals.var_wdld__blk1760_dn0 = assign79470_e122093_d_n0;
        locals.var_wdld__blk1760_dn2 = assign79470_e122093_d_n2;
        locals.var_wdld__blk1760_dn4 = assign79470_e122093_d_n4;
        locals.var_wdld__blk1760_dn5 = assign79470_e122093_d_n5;
        locals.var_wdld__blk1760_dn6 = assign79470_e122093_d_n6;
        locals.var_wdld__blk1760_dn7 = assign79470_e122093_d_n7;
        locals.var_wdld__blk1760_dn8 = assign79470_e122093_d_n8;
        locals.var_wdld__blk1760_dn9 = assign79470_e122093_d_n9;
        locals.var_wdld__blk1760_dn10 = assign79470_e122093_d_n10;
        locals.var_wdld__blk1760_dn13 = assign79470_e122093_d_n13;

        let (assign79480_e122101, assign79480_e122101_d_n0, assign79480_e122101_d_n2, assign79480_e122101_d_n4, assign79480_e122101_d_n5, assign79480_e122101_d_n6, assign79480_e122101_d_n7, assign79480_e122101_d_n8, assign79480_e122101_d_n9, assign79480_e122101_d_n10, assign79480_e122101_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79480_e122099: f64 = (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760);
        (assign79480_e122099, (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn0), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn2), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn4), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn5), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn6), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn7), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn8), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn9), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn10), (locals.var_q_nsubld__blk1762 * locals.var_wdld__blk1760_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1761, locals.var_q_dep_ld__blk1761_dn0, locals.var_q_dep_ld__blk1761_dn2, locals.var_q_dep_ld__blk1761_dn4, locals.var_q_dep_ld__blk1761_dn5, locals.var_q_dep_ld__blk1761_dn6, locals.var_q_dep_ld__blk1761_dn7, locals.var_q_dep_ld__blk1761_dn8, locals.var_q_dep_ld__blk1761_dn9, locals.var_q_dep_ld__blk1761_dn10, locals.var_q_dep_ld__blk1761_dn13,)
    }
};
        locals.var_q_dep_ld__blk1761 = assign79480_e122101;
        locals.var_q_dep_ld__blk1761_dn0 = assign79480_e122101_d_n0;
        locals.var_q_dep_ld__blk1761_dn2 = assign79480_e122101_d_n2;
        locals.var_q_dep_ld__blk1761_dn4 = assign79480_e122101_d_n4;
        locals.var_q_dep_ld__blk1761_dn5 = assign79480_e122101_d_n5;
        locals.var_q_dep_ld__blk1761_dn6 = assign79480_e122101_d_n6;
        locals.var_q_dep_ld__blk1761_dn7 = assign79480_e122101_d_n7;
        locals.var_q_dep_ld__blk1761_dn8 = assign79480_e122101_d_n8;
        locals.var_q_dep_ld__blk1761_dn9 = assign79480_e122101_d_n9;
        locals.var_q_dep_ld__blk1761_dn10 = assign79480_e122101_d_n10;
        locals.var_q_dep_ld__blk1761_dn13 = assign79480_e122101_d_n13;

        let (assign79490_e122113, assign79490_e122113_d_n0, assign79490_e122113_d_n2, assign79490_e122113_d_n4, assign79490_e122113_d_n5, assign79490_e122113_d_n6, assign79490_e122113_d_n7, assign79490_e122113_d_n8, assign79490_e122113_d_n9, assign79490_e122113_d_n10, assign79490_e122113_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79490_e122107: f64 = (locals.var_q_dep_ld__blk1761 / locals.var_cnst0over_func);
        let assign79490_e122110: f64 = (10.0 * 2.220446049250313e-16);
        let assign79490_e122111: f64 = (assign79490_e122107 + assign79490_e122110);
        (assign79490_e122111, (((locals.var_q_dep_ld__blk1761_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1761_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1761 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign79490_e122113;
        locals.var_xi0p12_dn0 = assign79490_e122113_d_n0;
        locals.var_xi0p12_dn2 = assign79490_e122113_d_n2;
        locals.var_xi0p12_dn4 = assign79490_e122113_d_n4;
        locals.var_xi0p12_dn5 = assign79490_e122113_d_n5;
        locals.var_xi0p12_dn6 = assign79490_e122113_d_n6;
        locals.var_xi0p12_dn7 = assign79490_e122113_d_n7;
        locals.var_xi0p12_dn8 = assign79490_e122113_d_n8;
        locals.var_xi0p12_dn9 = assign79490_e122113_d_n9;
        locals.var_xi0p12_dn10 = assign79490_e122113_d_n10;
        locals.var_xi0p12_dn13 = assign79490_e122113_d_n13;

        let (assign79500_e122121, assign79500_e122121_d_n0, assign79500_e122121_d_n2, assign79500_e122121_d_n4, assign79500_e122121_d_n5, assign79500_e122121_d_n6, assign79500_e122121_d_n7, assign79500_e122121_d_n8, assign79500_e122121_d_n9, assign79500_e122121_d_n10, assign79500_e122121_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79500_e122119: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79500_e122119, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign79500_e122121;
        locals.var_qbuld_dn0 = assign79500_e122121_d_n0;
        locals.var_qbuld_dn2 = assign79500_e122121_d_n2;
        locals.var_qbuld_dn4 = assign79500_e122121_d_n4;
        locals.var_qbuld_dn5 = assign79500_e122121_d_n5;
        locals.var_qbuld_dn6 = assign79500_e122121_d_n6;
        locals.var_qbuld_dn7 = assign79500_e122121_d_n7;
        locals.var_qbuld_dn8 = assign79500_e122121_d_n8;
        locals.var_qbuld_dn9 = assign79500_e122121_d_n9;
        locals.var_qbuld_dn10 = assign79500_e122121_d_n10;
        locals.var_qbuld_dn13 = assign79500_e122121_d_n13;

        let (assign79510_e122131, assign79510_e122131_d_n0, assign79510_e122131_d_n2, assign79510_e122131_d_n4, assign79510_e122131_d_n5, assign79510_e122131_d_n6, assign79510_e122131_d_n7, assign79510_e122131_d_n8, assign79510_e122131_d_n9, assign79510_e122131_d_n10, assign79510_e122131_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79510_e122128: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79510_e122129: f64 = (1.0 / assign79510_e122128);
        (assign79510_e122129, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79510_e122128 * assign79510_e122128))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign79510_e122128 * assign79510_e122128))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign79510_e122131;
        locals.var_t1_dn0 = assign79510_e122131_d_n0;
        locals.var_t1_dn2 = assign79510_e122131_d_n2;
        locals.var_t1_dn4 = assign79510_e122131_d_n4;
        locals.var_t1_dn5 = assign79510_e122131_d_n5;
        locals.var_t1_dn6 = assign79510_e122131_d_n6;
        locals.var_t1_dn7 = assign79510_e122131_d_n7;
        locals.var_t1_dn8 = assign79510_e122131_d_n8;
        locals.var_t1_dn9 = assign79510_e122131_d_n9;
        locals.var_t1_dn10 = assign79510_e122131_d_n10;
        locals.var_t1_dn13 = assign79510_e122131_d_n13;

        let (assign79520_e122141, assign79520_e122141_d_n0, assign79520_e122141_d_n2, assign79520_e122141_d_n4, assign79520_e122141_d_n5, assign79520_e122141_d_n6, assign79520_e122141_d_n7, assign79520_e122141_d_n8, assign79520_e122141_d_n9, assign79520_e122141_d_n10, assign79520_e122141_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79520_e122137: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79520_e122139: f64 = (assign79520_e122137 * locals.var_t1);
        (assign79520_e122139, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign79520_e122137 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79520_e122141;
        locals.var_qiuld_dn0 = assign79520_e122141_d_n0;
        locals.var_qiuld_dn2 = assign79520_e122141_d_n2;
        locals.var_qiuld_dn4 = assign79520_e122141_d_n4;
        locals.var_qiuld_dn5 = assign79520_e122141_d_n5;
        locals.var_qiuld_dn6 = assign79520_e122141_d_n6;
        locals.var_qiuld_dn7 = assign79520_e122141_d_n7;
        locals.var_qiuld_dn8 = assign79520_e122141_d_n8;
        locals.var_qiuld_dn9 = assign79520_e122141_d_n9;
        locals.var_qiuld_dn10 = assign79520_e122141_d_n10;
        locals.var_qiuld_dn13 = assign79520_e122141_d_n13;

        let (assign79530_e122149, assign79530_e122149_d_n0, assign79530_e122149_d_n2, assign79530_e122149_d_n4, assign79530_e122149_d_n5, assign79530_e122149_d_n6, assign79530_e122149_d_n7, assign79530_e122149_d_n8, assign79530_e122149_d_n9, assign79530_e122149_d_n10, assign79530_e122149_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1843 != 0.0)) {
        let assign79530_e122147: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79530_e122147, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign79530_e122149;
        locals.var_qsuld_dn0 = assign79530_e122149_d_n0;
        locals.var_qsuld_dn2 = assign79530_e122149_d_n2;
        locals.var_qsuld_dn4 = assign79530_e122149_d_n4;
        locals.var_qsuld_dn5 = assign79530_e122149_d_n5;
        locals.var_qsuld_dn6 = assign79530_e122149_d_n6;
        locals.var_qsuld_dn7 = assign79530_e122149_d_n7;
        locals.var_qsuld_dn8 = assign79530_e122149_d_n8;
        locals.var_qsuld_dn9 = assign79530_e122149_d_n9;
        locals.var_qsuld_dn10 = assign79530_e122149_d_n10;
        locals.var_qsuld_dn13 = assign79530_e122149_d_n13;

        let (assign79540_e122155, assign79540_e122155_d_n0, assign79540_e122155_d_n2, assign79540_e122155_d_n4, assign79540_e122155_d_n5, assign79540_e122155_d_n6, assign79540_e122155_d_n7, assign79540_e122155_d_n8, assign79540_e122155_d_n9, assign79540_e122155_d_n10, assign79540_e122155_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign79540_e122153: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign79540_e122153, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign79540_e122155;
        locals.var_qiuld_dn0 = assign79540_e122155_d_n0;
        locals.var_qiuld_dn2 = assign79540_e122155_d_n2;
        locals.var_qiuld_dn4 = assign79540_e122155_d_n4;
        locals.var_qiuld_dn5 = assign79540_e122155_d_n5;
        locals.var_qiuld_dn6 = assign79540_e122155_d_n6;
        locals.var_qiuld_dn7 = assign79540_e122155_d_n7;
        locals.var_qiuld_dn8 = assign79540_e122155_d_n8;
        locals.var_qiuld_dn9 = assign79540_e122155_d_n9;
        locals.var_qiuld_dn10 = assign79540_e122155_d_n10;
        locals.var_qiuld_dn13 = assign79540_e122155_d_n13;

        let assign79550_e122158: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1858 = assign79550_e122158;

        let (assign79560_e122165, assign79560_e122165_d_n0, assign79560_e122165_d_n2, assign79560_e122165_d_n4, assign79560_e122165_d_n5, assign79560_e122165_d_n6, assign79560_e122165_d_n7, assign79560_e122165_d_n8, assign79560_e122165_d_n9, assign79560_e122165_d_n10, assign79560_e122165_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) {
        let assign79560_e122163: f64 = (-locals.var_lover_func);
        (assign79560_e122163, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign79560_e122165;
        locals.var_lover_func_dn0 = assign79560_e122165_d_n0;
        locals.var_lover_func_dn2 = assign79560_e122165_d_n2;
        locals.var_lover_func_dn4 = assign79560_e122165_d_n4;
        locals.var_lover_func_dn5 = assign79560_e122165_d_n5;
        locals.var_lover_func_dn6 = assign79560_e122165_d_n6;
        locals.var_lover_func_dn7 = assign79560_e122165_d_n7;
        locals.var_lover_func_dn8 = assign79560_e122165_d_n8;
        locals.var_lover_func_dn9 = assign79560_e122165_d_n9;
        locals.var_lover_func_dn10 = assign79560_e122165_d_n10;
        locals.var_lover_func_dn13 = assign79560_e122165_d_n13;

        let assign79570_e122168: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1859 = assign79570_e122168;

        let assign79580_e122171: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1860 = assign79580_e122171;

        let (assign79590_e122182, assign79590_e122182_d_n0, assign79590_e122182_d_n2, assign79590_e122182_d_n4, assign79590_e122182_d_n5, assign79590_e122182_d_n6, assign79590_e122182_d_n7, assign79590_e122182_d_n8, assign79590_e122182_d_n9, assign79590_e122182_d_n10, assign79590_e122182_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1860 != 0.0)) {
        let assign79590_e122180: f64 = (-locals.var_ps0ld);
        (assign79590_e122180, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk1763, locals.var_vx__blk1763_dn0, locals.var_vx__blk1763_dn2, locals.var_vx__blk1763_dn4, locals.var_vx__blk1763_dn5, locals.var_vx__blk1763_dn6, locals.var_vx__blk1763_dn7, locals.var_vx__blk1763_dn8, locals.var_vx__blk1763_dn9, locals.var_vx__blk1763_dn10, locals.var_vx__blk1763_dn13,)
    }
};
        locals.var_vx__blk1763 = assign79590_e122182;
        locals.var_vx__blk1763_dn0 = assign79590_e122182_d_n0;
        locals.var_vx__blk1763_dn2 = assign79590_e122182_d_n2;
        locals.var_vx__blk1763_dn4 = assign79590_e122182_d_n4;
        locals.var_vx__blk1763_dn5 = assign79590_e122182_d_n5;
        locals.var_vx__blk1763_dn6 = assign79590_e122182_d_n6;
        locals.var_vx__blk1763_dn7 = assign79590_e122182_d_n7;
        locals.var_vx__blk1763_dn8 = assign79590_e122182_d_n8;
        locals.var_vx__blk1763_dn9 = assign79590_e122182_d_n9;
        locals.var_vx__blk1763_dn10 = assign79590_e122182_d_n10;
        locals.var_vx__blk1763_dn13 = assign79590_e122182_d_n13;

        let (assign79600_e122193, assign79600_e122193_d_n0, assign79600_e122193_d_n2, assign79600_e122193_d_n4, assign79600_e122193_d_n5, assign79600_e122193_d_n6, assign79600_e122193_d_n7, assign79600_e122193_d_n8, assign79600_e122193_d_n9, assign79600_e122193_d_n10, assign79600_e122193_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1860 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk1763, locals.var_vx__blk1763_dn0, locals.var_vx__blk1763_dn2, locals.var_vx__blk1763_dn4, locals.var_vx__blk1763_dn5, locals.var_vx__blk1763_dn6, locals.var_vx__blk1763_dn7, locals.var_vx__blk1763_dn8, locals.var_vx__blk1763_dn9, locals.var_vx__blk1763_dn10, locals.var_vx__blk1763_dn13,)
    }
};
        locals.var_vx__blk1763 = assign79600_e122193;
        locals.var_vx__blk1763_dn0 = assign79600_e122193_d_n0;
        locals.var_vx__blk1763_dn2 = assign79600_e122193_d_n2;
        locals.var_vx__blk1763_dn4 = assign79600_e122193_d_n4;
        locals.var_vx__blk1763_dn5 = assign79600_e122193_d_n5;
        locals.var_vx__blk1763_dn6 = assign79600_e122193_d_n6;
        locals.var_vx__blk1763_dn7 = assign79600_e122193_d_n7;
        locals.var_vx__blk1763_dn8 = assign79600_e122193_d_n8;
        locals.var_vx__blk1763_dn9 = assign79600_e122193_d_n9;
        locals.var_vx__blk1763_dn10 = assign79600_e122193_d_n10;
        locals.var_vx__blk1763_dn13 = assign79600_e122193_d_n13;

        let (assign79610_e122214, assign79610_e122214_d_n0, assign79610_e122214_d_n2, assign79610_e122214_d_n4, assign79610_e122214_d_n5, assign79610_e122214_d_n6, assign79610_e122214_d_n7, assign79610_e122214_d_n8, assign79610_e122214_d_n9, assign79610_e122214_d_n10, assign79610_e122214_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79610_e122201: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79610_e122204: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79610_e122205: f64 = (assign79610_e122201 * assign79610_e122204);
        let assign79610_e122208: f64 = (4.0 * 0.1);
        let assign79610_e122210: f64 = (assign79610_e122208 * 0.1);
        let assign79610_e122211: f64 = (assign79610_e122205 + assign79610_e122210);
        let assign79610_e122212: f64 = (assign79610_e122211).sqrt();
        (assign79610_e122212, (((locals.var_vx__blk1763_dn0 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn0)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn2 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn2)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn4 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn4)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn5 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn5)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn6 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn6)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn7 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn7)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn8 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn8)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn9 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn9)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn10 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn10)) / (2.0 * assign79610_e122212)), (((locals.var_vx__blk1763_dn13 * assign79610_e122204) + (assign79610_e122201 * locals.var_vx__blk1763_dn13)) / (2.0 * assign79610_e122212)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79610_e122214;
        locals.var_tmf2_dn0 = assign79610_e122214_d_n0;
        locals.var_tmf2_dn2 = assign79610_e122214_d_n2;
        locals.var_tmf2_dn4 = assign79610_e122214_d_n4;
        locals.var_tmf2_dn5 = assign79610_e122214_d_n5;
        locals.var_tmf2_dn6 = assign79610_e122214_d_n6;
        locals.var_tmf2_dn7 = assign79610_e122214_d_n7;
        locals.var_tmf2_dn8 = assign79610_e122214_d_n8;
        locals.var_tmf2_dn9 = assign79610_e122214_d_n9;
        locals.var_tmf2_dn10 = assign79610_e122214_d_n10;
        locals.var_tmf2_dn13 = assign79610_e122214_d_n13;

        let (assign79620_e122230, assign79620_e122230_d_n0, assign79620_e122230_d_n2, assign79620_e122230_d_n4, assign79620_e122230_d_n5, assign79620_e122230_d_n6, assign79620_e122230_d_n7, assign79620_e122230_d_n8, assign79620_e122230_d_n9, assign79620_e122230_d_n10, assign79620_e122230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79620_e122224: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79620_e122226: f64 = (assign79620_e122224 / locals.var_tmf2);
        let assign79620_e122227: f64 = (1.0 + assign79620_e122226);
        let assign79620_e122228: f64 = (0.5 * assign79620_e122227);
        (assign79620_e122228, (0.5 * (((locals.var_vx__blk1763_dn0 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn2 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn4 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn5 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn6 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn7 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn8 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn9 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn10 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1763_dn13 * locals.var_tmf2) - (assign79620_e122224 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79620_e122230;
        locals.var_t9_dn0 = assign79620_e122230_d_n0;
        locals.var_t9_dn2 = assign79620_e122230_d_n2;
        locals.var_t9_dn4 = assign79620_e122230_d_n4;
        locals.var_t9_dn5 = assign79620_e122230_d_n5;
        locals.var_t9_dn6 = assign79620_e122230_d_n6;
        locals.var_t9_dn7 = assign79620_e122230_d_n7;
        locals.var_t9_dn8 = assign79620_e122230_d_n8;
        locals.var_t9_dn9 = assign79620_e122230_d_n9;
        locals.var_t9_dn10 = assign79620_e122230_d_n10;
        locals.var_t9_dn13 = assign79620_e122230_d_n13;

        let (assign79630_e122244, assign79630_e122244_d_n0, assign79630_e122244_d_n2, assign79630_e122244_d_n4, assign79630_e122244_d_n5, assign79630_e122244_d_n6, assign79630_e122244_d_n7, assign79630_e122244_d_n8, assign79630_e122244_d_n9, assign79630_e122244_d_n10, assign79630_e122244_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79630_e122239: f64 = (locals.var_vx__blk1763 + p.p137);
        let assign79630_e122241: f64 = (assign79630_e122239 + locals.var_tmf2);
        let assign79630_e122242: f64 = (0.5 * assign79630_e122241);
        (assign79630_e122242, (0.5 * (locals.var_vx__blk1763_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk1763_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk1763_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk1763_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk1763_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk1763_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk1763_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk1763_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk1763_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk1763_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79630_e122244;
        locals.var_t2_dn0 = assign79630_e122244_d_n0;
        locals.var_t2_dn2 = assign79630_e122244_d_n2;
        locals.var_t2_dn4 = assign79630_e122244_d_n4;
        locals.var_t2_dn5 = assign79630_e122244_d_n5;
        locals.var_t2_dn6 = assign79630_e122244_d_n6;
        locals.var_t2_dn7 = assign79630_e122244_d_n7;
        locals.var_t2_dn8 = assign79630_e122244_d_n8;
        locals.var_t2_dn9 = assign79630_e122244_d_n9;
        locals.var_t2_dn10 = assign79630_e122244_d_n10;
        locals.var_t2_dn13 = assign79630_e122244_d_n13;

        let assign79640_e122247: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1861 = assign79640_e122247;

        let (assign79650_e122257, assign79650_e122257_d_n0, assign79650_e122257_d_n2, assign79650_e122257_d_n4, assign79650_e122257_d_n5, assign79650_e122257_d_n6, assign79650_e122257_d_n7, assign79650_e122257_d_n8, assign79650_e122257_d_n9, assign79650_e122257_d_n10, assign79650_e122257_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign79650_e122257;
        locals.var_t2_dn0 = assign79650_e122257_d_n0;
        locals.var_t2_dn2 = assign79650_e122257_d_n2;
        locals.var_t2_dn4 = assign79650_e122257_d_n4;
        locals.var_t2_dn5 = assign79650_e122257_d_n5;
        locals.var_t2_dn6 = assign79650_e122257_d_n6;
        locals.var_t2_dn7 = assign79650_e122257_d_n7;
        locals.var_t2_dn8 = assign79650_e122257_d_n8;
        locals.var_t2_dn9 = assign79650_e122257_d_n9;
        locals.var_t2_dn10 = assign79650_e122257_d_n10;
        locals.var_t2_dn13 = assign79650_e122257_d_n13;

        let (assign79660_e122267, assign79660_e122267_d_n0, assign79660_e122267_d_n2, assign79660_e122267_d_n4, assign79660_e122267_d_n5, assign79660_e122267_d_n6, assign79660_e122267_d_n7, assign79660_e122267_d_n8, assign79660_e122267_d_n9, assign79660_e122267_d_n10, assign79660_e122267_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign79660_e122267;
        locals.var_t9_dn0 = assign79660_e122267_d_n0;
        locals.var_t9_dn2 = assign79660_e122267_d_n2;
        locals.var_t9_dn4 = assign79660_e122267_d_n4;
        locals.var_t9_dn5 = assign79660_e122267_d_n5;
        locals.var_t9_dn6 = assign79660_e122267_d_n6;
        locals.var_t9_dn7 = assign79660_e122267_d_n7;
        locals.var_t9_dn8 = assign79660_e122267_d_n8;
        locals.var_t9_dn9 = assign79660_e122267_d_n9;
        locals.var_t9_dn10 = assign79660_e122267_d_n10;
        locals.var_t9_dn13 = assign79660_e122267_d_n13;

        let (assign79670_e122280, assign79670_e122280_d_n0, assign79670_e122280_d_n2, assign79670_e122280_d_n4, assign79670_e122280_d_n5, assign79670_e122280_d_n6, assign79670_e122280_d_n7, assign79670_e122280_d_n8, assign79670_e122280_d_n9, assign79670_e122280_d_n10, assign79670_e122280_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79670_e122275: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79670_e122276: f64 = (assign79670_e122275).sqrt();
        let assign79670_e122278: f64 = (assign79670_e122276 * p.p432);
        (assign79670_e122278, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79670_e122276)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign79670_e122276)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign79670_e122280;
        locals.var_wjunc0_dn0 = assign79670_e122280_d_n0;
        locals.var_wjunc0_dn2 = assign79670_e122280_d_n2;
        locals.var_wjunc0_dn4 = assign79670_e122280_d_n4;
        locals.var_wjunc0_dn5 = assign79670_e122280_d_n5;
        locals.var_wjunc0_dn6 = assign79670_e122280_d_n6;
        locals.var_wjunc0_dn7 = assign79670_e122280_d_n7;
        locals.var_wjunc0_dn8 = assign79670_e122280_d_n8;
        locals.var_wjunc0_dn9 = assign79670_e122280_d_n9;
        locals.var_wjunc0_dn10 = assign79670_e122280_d_n10;
        locals.var_wjunc0_dn13 = assign79670_e122280_d_n13;

        let (assign79680_e122294, assign79680_e122294_d_n0, assign79680_e122294_d_n2, assign79680_e122294_d_n4, assign79680_e122294_d_n5, assign79680_e122294_d_n6, assign79680_e122294_d_n7, assign79680_e122294_d_n8, assign79680_e122294_d_n9, assign79680_e122294_d_n10, assign79680_e122294_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79680_e122288: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign79680_e122291: f64 = (0.1 * locals.var_lover_func);
        let assign79680_e122292: f64 = (assign79680_e122288 - assign79680_e122291);
        (assign79680_e122292, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign79680_e122294;
        locals.var_tmf1_dn0 = assign79680_e122294_d_n0;
        locals.var_tmf1_dn2 = assign79680_e122294_d_n2;
        locals.var_tmf1_dn4 = assign79680_e122294_d_n4;
        locals.var_tmf1_dn5 = assign79680_e122294_d_n5;
        locals.var_tmf1_dn6 = assign79680_e122294_d_n6;
        locals.var_tmf1_dn7 = assign79680_e122294_d_n7;
        locals.var_tmf1_dn8 = assign79680_e122294_d_n8;
        locals.var_tmf1_dn9 = assign79680_e122294_d_n9;
        locals.var_tmf1_dn10 = assign79680_e122294_d_n10;
        locals.var_tmf1_dn13 = assign79680_e122294_d_n13;

        let (assign79690_e122308, assign79690_e122308_d_n0, assign79690_e122308_d_n2, assign79690_e122308_d_n4, assign79690_e122308_d_n5, assign79690_e122308_d_n6, assign79690_e122308_d_n7, assign79690_e122308_d_n8, assign79690_e122308_d_n9, assign79690_e122308_d_n10, assign79690_e122308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79690_e122302: f64 = (4.0 * locals.var_lover_func);
        let assign79690_e122305: f64 = (0.1 * locals.var_lover_func);
        let assign79690_e122306: f64 = (assign79690_e122302 * assign79690_e122305);
        (assign79690_e122306, (((4.0 * locals.var_lover_func_dn0) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign79690_e122305) + (assign79690_e122302 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79690_e122308;
        locals.var_tmf2_dn0 = assign79690_e122308_d_n0;
        locals.var_tmf2_dn2 = assign79690_e122308_d_n2;
        locals.var_tmf2_dn4 = assign79690_e122308_d_n4;
        locals.var_tmf2_dn5 = assign79690_e122308_d_n5;
        locals.var_tmf2_dn6 = assign79690_e122308_d_n6;
        locals.var_tmf2_dn7 = assign79690_e122308_d_n7;
        locals.var_tmf2_dn8 = assign79690_e122308_d_n8;
        locals.var_tmf2_dn9 = assign79690_e122308_d_n9;
        locals.var_tmf2_dn10 = assign79690_e122308_d_n10;
        locals.var_tmf2_dn13 = assign79690_e122308_d_n13;

        let (assign79700_e122322, assign79700_e122322_d_n0, assign79700_e122322_d_n2, assign79700_e122322_d_n4, assign79700_e122322_d_n5, assign79700_e122322_d_n6, assign79700_e122322_d_n7, assign79700_e122322_d_n8, assign79700_e122322_d_n9, assign79700_e122322_d_n10, assign79700_e122322_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let (assign79700_e122320, assign79700_e122320_d_n0, assign79700_e122320_d_n2, assign79700_e122320_d_n4, assign79700_e122320_d_n5, assign79700_e122320_d_n6, assign79700_e122320_d_n7, assign79700_e122320_d_n8, assign79700_e122320_d_n9, assign79700_e122320_d_n10, assign79700_e122320_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign79700_e122319: f64 = (-locals.var_tmf2);
                (assign79700_e122319, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign79700_e122320, assign79700_e122320_d_n0, assign79700_e122320_d_n2, assign79700_e122320_d_n4, assign79700_e122320_d_n5, assign79700_e122320_d_n6, assign79700_e122320_d_n7, assign79700_e122320_d_n8, assign79700_e122320_d_n9, assign79700_e122320_d_n10, assign79700_e122320_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79700_e122322;
        locals.var_tmf2_dn0 = assign79700_e122322_d_n0;
        locals.var_tmf2_dn2 = assign79700_e122322_d_n2;
        locals.var_tmf2_dn4 = assign79700_e122322_d_n4;
        locals.var_tmf2_dn5 = assign79700_e122322_d_n5;
        locals.var_tmf2_dn6 = assign79700_e122322_d_n6;
        locals.var_tmf2_dn7 = assign79700_e122322_d_n7;
        locals.var_tmf2_dn8 = assign79700_e122322_d_n8;
        locals.var_tmf2_dn9 = assign79700_e122322_d_n9;
        locals.var_tmf2_dn10 = assign79700_e122322_d_n10;
        locals.var_tmf2_dn13 = assign79700_e122322_d_n13;

        let (assign79710_e122335, assign79710_e122335_d_n0, assign79710_e122335_d_n2, assign79710_e122335_d_n4, assign79710_e122335_d_n5, assign79710_e122335_d_n6, assign79710_e122335_d_n7, assign79710_e122335_d_n8, assign79710_e122335_d_n9, assign79710_e122335_d_n10, assign79710_e122335_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79710_e122330: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign79710_e122332: f64 = (assign79710_e122330 + locals.var_tmf2);
        let assign79710_e122333: f64 = (assign79710_e122332).sqrt();
        (assign79710_e122333, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign79710_e122333)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign79710_e122333)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign79710_e122335;
        locals.var_tmf2_dn0 = assign79710_e122335_d_n0;
        locals.var_tmf2_dn2 = assign79710_e122335_d_n2;
        locals.var_tmf2_dn4 = assign79710_e122335_d_n4;
        locals.var_tmf2_dn5 = assign79710_e122335_d_n5;
        locals.var_tmf2_dn6 = assign79710_e122335_d_n6;
        locals.var_tmf2_dn7 = assign79710_e122335_d_n7;
        locals.var_tmf2_dn8 = assign79710_e122335_d_n8;
        locals.var_tmf2_dn9 = assign79710_e122335_d_n9;
        locals.var_tmf2_dn10 = assign79710_e122335_d_n10;
        locals.var_tmf2_dn13 = assign79710_e122335_d_n13;

    }

    pub(super) fn stamp_transient_block_278(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79720_e122349, assign79720_e122349_d_n0, assign79720_e122349_d_n2, assign79720_e122349_d_n4, assign79720_e122349_d_n5, assign79720_e122349_d_n6, assign79720_e122349_d_n7, assign79720_e122349_d_n8, assign79720_e122349_d_n9, assign79720_e122349_d_n10, assign79720_e122349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79720_e122345: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign79720_e122346: f64 = (1.0 + assign79720_e122345);
        let assign79720_e122347: f64 = (0.5 * assign79720_e122346);
        (assign79720_e122347, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign79720_e122349;
        locals.var_t0_dn0 = assign79720_e122349_d_n0;
        locals.var_t0_dn2 = assign79720_e122349_d_n2;
        locals.var_t0_dn4 = assign79720_e122349_d_n4;
        locals.var_t0_dn5 = assign79720_e122349_d_n5;
        locals.var_t0_dn6 = assign79720_e122349_d_n6;
        locals.var_t0_dn7 = assign79720_e122349_d_n7;
        locals.var_t0_dn8 = assign79720_e122349_d_n8;
        locals.var_t0_dn9 = assign79720_e122349_d_n9;
        locals.var_t0_dn10 = assign79720_e122349_d_n10;
        locals.var_t0_dn13 = assign79720_e122349_d_n13;

        let (assign79730_e122363, assign79730_e122363_d_n0, assign79730_e122363_d_n2, assign79730_e122363_d_n4, assign79730_e122363_d_n5, assign79730_e122363_d_n6, assign79730_e122363_d_n7, assign79730_e122363_d_n8, assign79730_e122363_d_n9, assign79730_e122363_d_n10, assign79730_e122363_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79730_e122359: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign79730_e122360: f64 = (0.5 * assign79730_e122359);
        let assign79730_e122361: f64 = (locals.var_lover_func - assign79730_e122360);
        (assign79730_e122361, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign79730_e122363;
        locals.var_wjuncld_dn0 = assign79730_e122363_d_n0;
        locals.var_wjuncld_dn2 = assign79730_e122363_d_n2;
        locals.var_wjuncld_dn4 = assign79730_e122363_d_n4;
        locals.var_wjuncld_dn5 = assign79730_e122363_d_n5;
        locals.var_wjuncld_dn6 = assign79730_e122363_d_n6;
        locals.var_wjuncld_dn7 = assign79730_e122363_d_n7;
        locals.var_wjuncld_dn8 = assign79730_e122363_d_n8;
        locals.var_wjuncld_dn9 = assign79730_e122363_d_n9;
        locals.var_wjuncld_dn10 = assign79730_e122363_d_n10;
        locals.var_wjuncld_dn13 = assign79730_e122363_d_n13;

        let (assign79740_e122373, assign79740_e122373_d_n0, assign79740_e122373_d_n2, assign79740_e122373_d_n4, assign79740_e122373_d_n5, assign79740_e122373_d_n6, assign79740_e122373_d_n7, assign79740_e122373_d_n8, assign79740_e122373_d_n9, assign79740_e122373_d_n10, assign79740_e122373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1858 != 0.0)) && (locals.var_guard1859 != 0.0)) {
        let assign79740_e122371: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign79740_e122371, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign79740_e122373;
        locals.var_lover_func_dn0 = assign79740_e122373_d_n0;
        locals.var_lover_func_dn2 = assign79740_e122373_d_n2;
        locals.var_lover_func_dn4 = assign79740_e122373_d_n4;
        locals.var_lover_func_dn5 = assign79740_e122373_d_n5;
        locals.var_lover_func_dn6 = assign79740_e122373_d_n6;
        locals.var_lover_func_dn7 = assign79740_e122373_d_n7;
        locals.var_lover_func_dn8 = assign79740_e122373_d_n8;
        locals.var_lover_func_dn9 = assign79740_e122373_d_n9;
        locals.var_lover_func_dn10 = assign79740_e122373_d_n10;
        locals.var_lover_func_dn13 = assign79740_e122373_d_n13;

        let assign79750_e122376: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1862 = assign79750_e122376;

        let assign79760_e122379: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1863 = assign79760_e122379;

        let assign79770_e122382: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1864 = assign79770_e122382;

        let assign79780_e122385: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1865 = assign79780_e122385;

        let assign79790_e122388: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1866 = assign79790_e122388;

        let (assign79800_e122398, assign79800_e122398_d_n0, assign79800_e122398_d_n2, assign79800_e122398_d_n4, assign79800_e122398_d_n5, assign79800_e122398_d_n6, assign79800_e122398_d_n7, assign79800_e122398_d_n8, assign79800_e122398_d_n9, assign79800_e122398_d_n10, assign79800_e122398_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) && (locals.var_guard1866 != 0.0)) {
        let assign79800_e122396: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79800_e122396, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79800_e122398;
        locals.var_t4_dn0 = assign79800_e122398_d_n0;
        locals.var_t4_dn2 = assign79800_e122398_d_n2;
        locals.var_t4_dn4 = assign79800_e122398_d_n4;
        locals.var_t4_dn5 = assign79800_e122398_d_n5;
        locals.var_t4_dn6 = assign79800_e122398_d_n6;
        locals.var_t4_dn7 = assign79800_e122398_d_n7;
        locals.var_t4_dn8 = assign79800_e122398_d_n8;
        locals.var_t4_dn9 = assign79800_e122398_d_n9;
        locals.var_t4_dn10 = assign79800_e122398_d_n10;
        locals.var_t4_dn13 = assign79800_e122398_d_n13;

        let (assign79810_e122413, assign79810_e122413_d_n0, assign79810_e122413_d_n2, assign79810_e122413_d_n4, assign79810_e122413_d_n5, assign79810_e122413_d_n6, assign79810_e122413_d_n7, assign79810_e122413_d_n8, assign79810_e122413_d_n9, assign79810_e122413_d_n10, assign79810_e122413_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) && (locals.var_guard1866 == 0.0)) {
        let assign79810_e122407: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79810_e122410: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79810_e122411: f64 = (assign79810_e122407 * assign79810_e122410);
        (assign79810_e122411, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79810_e122410), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign79810_e122410),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79810_e122413;
        locals.var_t4_dn0 = assign79810_e122413_d_n0;
        locals.var_t4_dn2 = assign79810_e122413_d_n2;
        locals.var_t4_dn4 = assign79810_e122413_d_n4;
        locals.var_t4_dn5 = assign79810_e122413_d_n5;
        locals.var_t4_dn6 = assign79810_e122413_d_n6;
        locals.var_t4_dn7 = assign79810_e122413_d_n7;
        locals.var_t4_dn8 = assign79810_e122413_d_n8;
        locals.var_t4_dn9 = assign79810_e122413_d_n9;
        locals.var_t4_dn10 = assign79810_e122413_d_n10;
        locals.var_t4_dn13 = assign79810_e122413_d_n13;

        let (assign79820_e122421, assign79820_e122421_d_n0, assign79820_e122421_d_n2, assign79820_e122421_d_n4, assign79820_e122421_d_n5, assign79820_e122421_d_n6, assign79820_e122421_d_n7, assign79820_e122421_d_n8, assign79820_e122421_d_n9, assign79820_e122421_d_n10, assign79820_e122421_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) {
        let assign79820_e122419: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79820_e122419, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign79820_e122421;
        locals.var_qovs_dn0 = assign79820_e122421_d_n0;
        locals.var_qovs_dn2 = assign79820_e122421_d_n2;
        locals.var_qovs_dn4 = assign79820_e122421_d_n4;
        locals.var_qovs_dn5 = assign79820_e122421_d_n5;
        locals.var_qovs_dn6 = assign79820_e122421_d_n6;
        locals.var_qovs_dn7 = assign79820_e122421_d_n7;
        locals.var_qovs_dn8 = assign79820_e122421_d_n8;
        locals.var_qovs_dn9 = assign79820_e122421_d_n9;
        locals.var_qovs_dn10 = assign79820_e122421_d_n10;
        locals.var_qovs_dn13 = assign79820_e122421_d_n13;

        let (assign79830_e122429, assign79830_e122429_d_n0, assign79830_e122429_d_n2, assign79830_e122429_d_n4, assign79830_e122429_d_n5, assign79830_e122429_d_n6, assign79830_e122429_d_n7, assign79830_e122429_d_n8, assign79830_e122429_d_n9, assign79830_e122429_d_n10, assign79830_e122429_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1862 != 0.0)) {
        let assign79830_e122427: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79830_e122427, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign79830_e122429;
        locals.var_qbsld_dn0 = assign79830_e122429_d_n0;
        locals.var_qbsld_dn2 = assign79830_e122429_d_n2;
        locals.var_qbsld_dn4 = assign79830_e122429_d_n4;
        locals.var_qbsld_dn5 = assign79830_e122429_d_n5;
        locals.var_qbsld_dn6 = assign79830_e122429_d_n6;
        locals.var_qbsld_dn7 = assign79830_e122429_d_n7;
        locals.var_qbsld_dn8 = assign79830_e122429_d_n8;
        locals.var_qbsld_dn9 = assign79830_e122429_d_n9;
        locals.var_qbsld_dn10 = assign79830_e122429_d_n10;
        locals.var_qbsld_dn13 = assign79830_e122429_d_n13;

        let (assign79860_e122454, assign79860_e122454_d_n0, assign79860_e122454_d_n2, assign79860_e122454_d_n4, assign79860_e122454_d_n5, assign79860_e122454_d_n6, assign79860_e122454_d_n7, assign79860_e122454_d_n8, assign79860_e122454_d_n9, assign79860_e122454_d_n10, assign79860_e122454_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79860_e122450: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79860_e122452: f64 = (assign79860_e122450 * locals.var_uc_cvdsover);
        (assign79860_e122452, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79860_e122454;
        locals.var_t4_dn0 = assign79860_e122454_d_n0;
        locals.var_t4_dn2 = assign79860_e122454_d_n2;
        locals.var_t4_dn4 = assign79860_e122454_d_n4;
        locals.var_t4_dn5 = assign79860_e122454_d_n5;
        locals.var_t4_dn6 = assign79860_e122454_d_n6;
        locals.var_t4_dn7 = assign79860_e122454_d_n7;
        locals.var_t4_dn8 = assign79860_e122454_d_n8;
        locals.var_t4_dn9 = assign79860_e122454_d_n9;
        locals.var_t4_dn10 = assign79860_e122454_d_n10;
        locals.var_t4_dn13 = assign79860_e122454_d_n13;

        let (assign79870_e122465, assign79870_e122465_d_n0, assign79870_e122465_d_n2, assign79870_e122465_d_n4, assign79870_e122465_d_n5, assign79870_e122465_d_n6, assign79870_e122465_d_n7, assign79870_e122465_d_n8, assign79870_e122465_d_n9, assign79870_e122465_d_n10, assign79870_e122465_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79870_e122463: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79870_e122463, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign79870_e122465;
        locals.var_qovsext_dn0 = assign79870_e122465_d_n0;
        locals.var_qovsext_dn2 = assign79870_e122465_d_n2;
        locals.var_qovsext_dn4 = assign79870_e122465_d_n4;
        locals.var_qovsext_dn5 = assign79870_e122465_d_n5;
        locals.var_qovsext_dn6 = assign79870_e122465_d_n6;
        locals.var_qovsext_dn7 = assign79870_e122465_d_n7;
        locals.var_qovsext_dn8 = assign79870_e122465_d_n8;
        locals.var_qovsext_dn9 = assign79870_e122465_d_n9;
        locals.var_qovsext_dn10 = assign79870_e122465_d_n10;
        locals.var_qovsext_dn13 = assign79870_e122465_d_n13;

        let (assign79880_e122476, assign79880_e122476_d_n0, assign79880_e122476_d_n2, assign79880_e122476_d_n4, assign79880_e122476_d_n5, assign79880_e122476_d_n6, assign79880_e122476_d_n7, assign79880_e122476_d_n8, assign79880_e122476_d_n9, assign79880_e122476_d_n10, assign79880_e122476_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1863 != 0.0) && (locals.var_guard1862 == 0.0))) {
        let assign79880_e122474: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79880_e122474, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign79880_e122476;
        locals.var_qbsldext_dn0 = assign79880_e122476_d_n0;
        locals.var_qbsldext_dn2 = assign79880_e122476_d_n2;
        locals.var_qbsldext_dn4 = assign79880_e122476_d_n4;
        locals.var_qbsldext_dn5 = assign79880_e122476_d_n5;
        locals.var_qbsldext_dn6 = assign79880_e122476_d_n6;
        locals.var_qbsldext_dn7 = assign79880_e122476_d_n7;
        locals.var_qbsldext_dn8 = assign79880_e122476_d_n8;
        locals.var_qbsldext_dn9 = assign79880_e122476_d_n9;
        locals.var_qbsldext_dn10 = assign79880_e122476_d_n10;
        locals.var_qbsldext_dn13 = assign79880_e122476_d_n13;

        let assign79890_e122479: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1867 = assign79890_e122479;

        let (assign79900_e122494, assign79900_e122494_d_n0, assign79900_e122494_d_n2, assign79900_e122494_d_n4, assign79900_e122494_d_n5, assign79900_e122494_d_n6, assign79900_e122494_d_n7, assign79900_e122494_d_n8, assign79900_e122494_d_n9, assign79900_e122494_d_n10, assign79900_e122494_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1867 != 0.0)) {
        let assign79900_e122492: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79900_e122492, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79900_e122494;
        locals.var_t4_dn0 = assign79900_e122494_d_n0;
        locals.var_t4_dn2 = assign79900_e122494_d_n2;
        locals.var_t4_dn4 = assign79900_e122494_d_n4;
        locals.var_t4_dn5 = assign79900_e122494_d_n5;
        locals.var_t4_dn6 = assign79900_e122494_d_n6;
        locals.var_t4_dn7 = assign79900_e122494_d_n7;
        locals.var_t4_dn8 = assign79900_e122494_d_n8;
        locals.var_t4_dn9 = assign79900_e122494_d_n9;
        locals.var_t4_dn10 = assign79900_e122494_d_n10;
        locals.var_t4_dn13 = assign79900_e122494_d_n13;

        let (assign79910_e122514, assign79910_e122514_d_n0, assign79910_e122514_d_n2, assign79910_e122514_d_n4, assign79910_e122514_d_n5, assign79910_e122514_d_n6, assign79910_e122514_d_n7, assign79910_e122514_d_n8, assign79910_e122514_d_n9, assign79910_e122514_d_n10, assign79910_e122514_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1867 == 0.0)) {
        let assign79910_e122508: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79910_e122511: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79910_e122512: f64 = (assign79910_e122508 * assign79910_e122511);
        (assign79910_e122512, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79910_e122511), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign79910_e122511),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79910_e122514;
        locals.var_t4_dn0 = assign79910_e122514_d_n0;
        locals.var_t4_dn2 = assign79910_e122514_d_n2;
        locals.var_t4_dn4 = assign79910_e122514_d_n4;
        locals.var_t4_dn5 = assign79910_e122514_d_n5;
        locals.var_t4_dn6 = assign79910_e122514_d_n6;
        locals.var_t4_dn7 = assign79910_e122514_d_n7;
        locals.var_t4_dn8 = assign79910_e122514_d_n8;
        locals.var_t4_dn9 = assign79910_e122514_d_n9;
        locals.var_t4_dn10 = assign79910_e122514_d_n10;
        locals.var_t4_dn13 = assign79910_e122514_d_n13;

        let (assign79920_e122525, assign79920_e122525_d_n0, assign79920_e122525_d_n2, assign79920_e122525_d_n4, assign79920_e122525_d_n5, assign79920_e122525_d_n6, assign79920_e122525_d_n7, assign79920_e122525_d_n8, assign79920_e122525_d_n9, assign79920_e122525_d_n10, assign79920_e122525_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign79920_e122525;
        locals.var_rd_ps0ld_dn0 = assign79920_e122525_d_n0;
        locals.var_rd_ps0ld_dn2 = assign79920_e122525_d_n2;
        locals.var_rd_ps0ld_dn4 = assign79920_e122525_d_n4;
        locals.var_rd_ps0ld_dn5 = assign79920_e122525_d_n5;
        locals.var_rd_ps0ld_dn6 = assign79920_e122525_d_n6;
        locals.var_rd_ps0ld_dn7 = assign79920_e122525_d_n7;
        locals.var_rd_ps0ld_dn8 = assign79920_e122525_d_n8;
        locals.var_rd_ps0ld_dn9 = assign79920_e122525_d_n9;
        locals.var_rd_ps0ld_dn10 = assign79920_e122525_d_n10;
        locals.var_rd_ps0ld_dn13 = assign79920_e122525_d_n13;

        let assign79930_e122528: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1868 = assign79930_e122528;

        let (assign79940_e122541, assign79940_e122541_d_n0, assign79940_e122541_d_n2, assign79940_e122541_d_n4, assign79940_e122541_d_n5, assign79940_e122541_d_n6, assign79940_e122541_d_n7, assign79940_e122541_d_n8, assign79940_e122541_d_n9, assign79940_e122541_d_n10, assign79940_e122541_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) && (locals.var_guard1868 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign79940_e122541;
        locals.var_rd_qbuld_dn0 = assign79940_e122541_d_n0;
        locals.var_rd_qbuld_dn2 = assign79940_e122541_d_n2;
        locals.var_rd_qbuld_dn4 = assign79940_e122541_d_n4;
        locals.var_rd_qbuld_dn5 = assign79940_e122541_d_n5;
        locals.var_rd_qbuld_dn6 = assign79940_e122541_d_n6;
        locals.var_rd_qbuld_dn7 = assign79940_e122541_d_n7;
        locals.var_rd_qbuld_dn8 = assign79940_e122541_d_n8;
        locals.var_rd_qbuld_dn9 = assign79940_e122541_d_n9;
        locals.var_rd_qbuld_dn10 = assign79940_e122541_d_n10;
        locals.var_rd_qbuld_dn13 = assign79940_e122541_d_n13;

        let (assign79950_e122554, assign79950_e122554_d_n0, assign79950_e122554_d_n2, assign79950_e122554_d_n4, assign79950_e122554_d_n5, assign79950_e122554_d_n6, assign79950_e122554_d_n7, assign79950_e122554_d_n8, assign79950_e122554_d_n9, assign79950_e122554_d_n10, assign79950_e122554_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        let assign79950_e122552: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79950_e122552, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign79950_e122554;
        locals.var_qovd_dn0 = assign79950_e122554_d_n0;
        locals.var_qovd_dn2 = assign79950_e122554_d_n2;
        locals.var_qovd_dn4 = assign79950_e122554_d_n4;
        locals.var_qovd_dn5 = assign79950_e122554_d_n5;
        locals.var_qovd_dn6 = assign79950_e122554_d_n6;
        locals.var_qovd_dn7 = assign79950_e122554_d_n7;
        locals.var_qovd_dn8 = assign79950_e122554_d_n8;
        locals.var_qovd_dn9 = assign79950_e122554_d_n9;
        locals.var_qovd_dn10 = assign79950_e122554_d_n10;
        locals.var_qovd_dn13 = assign79950_e122554_d_n13;

        let (assign79960_e122567, assign79960_e122567_d_n0, assign79960_e122567_d_n2, assign79960_e122567_d_n4, assign79960_e122567_d_n5, assign79960_e122567_d_n6, assign79960_e122567_d_n7, assign79960_e122567_d_n8, assign79960_e122567_d_n9, assign79960_e122567_d_n10, assign79960_e122567_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        let assign79960_e122565: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79960_e122565, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign79960_e122567;
        locals.var_qbdld_dn0 = assign79960_e122567_d_n0;
        locals.var_qbdld_dn2 = assign79960_e122567_d_n2;
        locals.var_qbdld_dn4 = assign79960_e122567_d_n4;
        locals.var_qbdld_dn5 = assign79960_e122567_d_n5;
        locals.var_qbdld_dn6 = assign79960_e122567_d_n6;
        locals.var_qbdld_dn7 = assign79960_e122567_d_n7;
        locals.var_qbdld_dn8 = assign79960_e122567_d_n8;
        locals.var_qbdld_dn9 = assign79960_e122567_d_n9;
        locals.var_qbdld_dn10 = assign79960_e122567_d_n10;
        locals.var_qbdld_dn13 = assign79960_e122567_d_n13;

        let (assign79970_e122578, assign79970_e122578_d_n0, assign79970_e122578_d_n2, assign79970_e122578_d_n4, assign79970_e122578_d_n5, assign79970_e122578_d_n6, assign79970_e122578_d_n7, assign79970_e122578_d_n8, assign79970_e122578_d_n9, assign79970_e122578_d_n10, assign79970_e122578_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1864 != 0.0) && (!((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign79970_e122578;
        locals.var_qbd_qs_dn0 = assign79970_e122578_d_n0;
        locals.var_qbd_qs_dn2 = assign79970_e122578_d_n2;
        locals.var_qbd_qs_dn4 = assign79970_e122578_d_n4;
        locals.var_qbd_qs_dn5 = assign79970_e122578_d_n5;
        locals.var_qbd_qs_dn6 = assign79970_e122578_d_n6;
        locals.var_qbd_qs_dn7 = assign79970_e122578_d_n7;
        locals.var_qbd_qs_dn8 = assign79970_e122578_d_n8;
        locals.var_qbd_qs_dn9 = assign79970_e122578_d_n9;
        locals.var_qbd_qs_dn10 = assign79970_e122578_d_n10;
        locals.var_qbd_qs_dn13 = assign79970_e122578_d_n13;

        let (assign79980_e122595, assign79980_e122595_d_n0, assign79980_e122595_d_n2, assign79980_e122595_d_n4, assign79980_e122595_d_n5, assign79980_e122595_d_n6, assign79980_e122595_d_n7, assign79980_e122595_d_n8, assign79980_e122595_d_n9, assign79980_e122595_d_n10, assign79980_e122595_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign79980_e122591: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79980_e122593: f64 = (assign79980_e122591 * locals.var_uc_cvdsover);
        (assign79980_e122593, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign79980_e122595;
        locals.var_t4_dn0 = assign79980_e122595_d_n0;
        locals.var_t4_dn2 = assign79980_e122595_d_n2;
        locals.var_t4_dn4 = assign79980_e122595_d_n4;
        locals.var_t4_dn5 = assign79980_e122595_d_n5;
        locals.var_t4_dn6 = assign79980_e122595_d_n6;
        locals.var_t4_dn7 = assign79980_e122595_d_n7;
        locals.var_t4_dn8 = assign79980_e122595_d_n8;
        locals.var_t4_dn9 = assign79980_e122595_d_n9;
        locals.var_t4_dn10 = assign79980_e122595_d_n10;
        locals.var_t4_dn13 = assign79980_e122595_d_n13;

        let (assign79990_e122610, assign79990_e122610_d_n0, assign79990_e122610_d_n2, assign79990_e122610_d_n4, assign79990_e122610_d_n5, assign79990_e122610_d_n6, assign79990_e122610_d_n7, assign79990_e122610_d_n8, assign79990_e122610_d_n9, assign79990_e122610_d_n10, assign79990_e122610_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign79990_e122608: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79990_e122608, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign79990_e122610;
        locals.var_qovdext_dn0 = assign79990_e122610_d_n0;
        locals.var_qovdext_dn2 = assign79990_e122610_d_n2;
        locals.var_qovdext_dn4 = assign79990_e122610_d_n4;
        locals.var_qovdext_dn5 = assign79990_e122610_d_n5;
        locals.var_qovdext_dn6 = assign79990_e122610_d_n6;
        locals.var_qovdext_dn7 = assign79990_e122610_d_n7;
        locals.var_qovdext_dn8 = assign79990_e122610_d_n8;
        locals.var_qovdext_dn9 = assign79990_e122610_d_n9;
        locals.var_qovdext_dn10 = assign79990_e122610_d_n10;
        locals.var_qovdext_dn13 = assign79990_e122610_d_n13;

        let (assign80000_e122625, assign80000_e122625_d_n0, assign80000_e122625_d_n2, assign80000_e122625_d_n4, assign80000_e122625_d_n5, assign80000_e122625_d_n6, assign80000_e122625_d_n7, assign80000_e122625_d_n8, assign80000_e122625_d_n9, assign80000_e122625_d_n10, assign80000_e122625_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (!(((locals.var_guard1862 != 0.0) || (locals.var_guard1863 != 0.0)) || (locals.var_guard1864 != 0.0))))) {
        let assign80000_e122623: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign80000_e122623, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign80000_e122625;
        locals.var_qbdldext_dn0 = assign80000_e122625_d_n0;
        locals.var_qbdldext_dn2 = assign80000_e122625_d_n2;
        locals.var_qbdldext_dn4 = assign80000_e122625_d_n4;
        locals.var_qbdldext_dn5 = assign80000_e122625_d_n5;
        locals.var_qbdldext_dn6 = assign80000_e122625_d_n6;
        locals.var_qbdldext_dn7 = assign80000_e122625_d_n7;
        locals.var_qbdldext_dn8 = assign80000_e122625_d_n8;
        locals.var_qbdldext_dn9 = assign80000_e122625_d_n9;
        locals.var_qbdldext_dn10 = assign80000_e122625_d_n10;
        locals.var_qbdldext_dn13 = assign80000_e122625_d_n13;

        locals.var_flg_calcqover = 0.0;

        let assign80020_e122629: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1869 = assign80020_e122629;

        let assign80030_e122632: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1870 = assign80030_e122632;

        let assign80040_e122635: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1871 = assign80040_e122635;

        let assign80050_e122638: f64 = if 3.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1872 = assign80050_e122638;

        let assign80060_e122649: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1873 = assign80060_e122649;

        let (assign80070_e122655,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80070_e122655;

        let (assign80080_e122661,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign80080_e122661;

        let (assign80090_e122669, assign80090_e122669_d_n2, assign80090_e122669_d_n6, assign80090_e122669_d_n7, assign80090_e122669_d_n8,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        let assign80090_e122667: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80090_e122667, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80090_e122669;
        locals.var_vgbgmt_dn2 = assign80090_e122669_d_n2;
        locals.var_vgbgmt_dn6 = assign80090_e122669_d_n6;
        locals.var_vgbgmt_dn7 = assign80090_e122669_d_n7;
        locals.var_vgbgmt_dn8 = assign80090_e122669_d_n8;

        let (assign80100_e122676, assign80100_e122676_d_n0, assign80100_e122676_d_n2, assign80100_e122676_d_n4, assign80100_e122676_d_n5, assign80100_e122676_d_n6, assign80100_e122676_d_n7, assign80100_e122676_d_n8, assign80100_e122676_d_n9, assign80100_e122676_d_n10, assign80100_e122676_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        let assign80100_e122674: f64 = (-locals.var_vbsi);
        (assign80100_e122674, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80100_e122676;
        locals.var_vxbgmt_dn0 = assign80100_e122676_d_n0;
        locals.var_vxbgmt_dn2 = assign80100_e122676_d_n2;
        locals.var_vxbgmt_dn4 = assign80100_e122676_d_n4;
        locals.var_vxbgmt_dn5 = assign80100_e122676_d_n5;
        locals.var_vxbgmt_dn6 = assign80100_e122676_d_n6;
        locals.var_vxbgmt_dn7 = assign80100_e122676_d_n7;
        locals.var_vxbgmt_dn8 = assign80100_e122676_d_n8;
        locals.var_vxbgmt_dn9 = assign80100_e122676_d_n9;
        locals.var_vxbgmt_dn10 = assign80100_e122676_d_n10;
        locals.var_vxbgmt_dn13 = assign80100_e122676_d_n13;

        let (assign80110_e122682,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80110_e122682;

    }

    pub(super) fn stamp_transient_block_279(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign80120_e122688, assign80120_e122688_d_n0, assign80120_e122688_d_n2, assign80120_e122688_d_n4, assign80120_e122688_d_n5, assign80120_e122688_d_n6, assign80120_e122688_d_n7, assign80120_e122688_d_n8, assign80120_e122688_d_n9, assign80120_e122688_d_n10, assign80120_e122688_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80120_e122688;
        locals.var_lover_func_dn0 = assign80120_e122688_d_n0;
        locals.var_lover_func_dn2 = assign80120_e122688_d_n2;
        locals.var_lover_func_dn4 = assign80120_e122688_d_n4;
        locals.var_lover_func_dn5 = assign80120_e122688_d_n5;
        locals.var_lover_func_dn6 = assign80120_e122688_d_n6;
        locals.var_lover_func_dn7 = assign80120_e122688_d_n7;
        locals.var_lover_func_dn8 = assign80120_e122688_d_n8;
        locals.var_lover_func_dn9 = assign80120_e122688_d_n9;
        locals.var_lover_func_dn10 = assign80120_e122688_d_n10;
        locals.var_lover_func_dn13 = assign80120_e122688_d_n13;

        let (assign80130_e122694, assign80130_e122694_d_n0, assign80130_e122694_d_n2, assign80130_e122694_d_n4, assign80130_e122694_d_n5, assign80130_e122694_d_n6, assign80130_e122694_d_n7, assign80130_e122694_d_n8, assign80130_e122694_d_n9, assign80130_e122694_d_n10, assign80130_e122694_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign80130_e122694;
        locals.var_wdep_func_dn0 = assign80130_e122694_d_n0;
        locals.var_wdep_func_dn2 = assign80130_e122694_d_n2;
        locals.var_wdep_func_dn4 = assign80130_e122694_d_n4;
        locals.var_wdep_func_dn5 = assign80130_e122694_d_n5;
        locals.var_wdep_func_dn6 = assign80130_e122694_d_n6;
        locals.var_wdep_func_dn7 = assign80130_e122694_d_n7;
        locals.var_wdep_func_dn8 = assign80130_e122694_d_n8;
        locals.var_wdep_func_dn9 = assign80130_e122694_d_n9;
        locals.var_wdep_func_dn10 = assign80130_e122694_d_n10;
        locals.var_wdep_func_dn13 = assign80130_e122694_d_n13;

        let (assign80140_e122700, assign80140_e122700_d_n0, assign80140_e122700_d_n2, assign80140_e122700_d_n4, assign80140_e122700_d_n5, assign80140_e122700_d_n6, assign80140_e122700_d_n7, assign80140_e122700_d_n8, assign80140_e122700_d_n9, assign80140_e122700_d_n10, assign80140_e122700_d_n13,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign80140_e122700;
        locals.var_cnst0over_func_dn0 = assign80140_e122700_d_n0;
        locals.var_cnst0over_func_dn2 = assign80140_e122700_d_n2;
        locals.var_cnst0over_func_dn4 = assign80140_e122700_d_n4;
        locals.var_cnst0over_func_dn5 = assign80140_e122700_d_n5;
        locals.var_cnst0over_func_dn6 = assign80140_e122700_d_n6;
        locals.var_cnst0over_func_dn7 = assign80140_e122700_d_n7;
        locals.var_cnst0over_func_dn8 = assign80140_e122700_d_n8;
        locals.var_cnst0over_func_dn9 = assign80140_e122700_d_n9;
        locals.var_cnst0over_func_dn10 = assign80140_e122700_d_n10;
        locals.var_cnst0over_func_dn13 = assign80140_e122700_d_n13;

        let (assign80150_e122706,) = {
    if ((locals.var_guard1869 != 0.0) && (locals.var_guard1873 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80150_e122706;

        let assign80160_e122725: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1874 = assign80160_e122725;

        let (assign80170_e122734,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80170_e122734;

        let (assign80180_e122745, assign80180_e122745_d_n2, assign80180_e122745_d_n6, assign80180_e122745_d_n7, assign80180_e122745_d_n8,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        let assign80180_e122743: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign80180_e122743, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80180_e122745;
        locals.var_vgbgmt_dn2 = assign80180_e122745_d_n2;
        locals.var_vgbgmt_dn6 = assign80180_e122745_d_n6;
        locals.var_vgbgmt_dn7 = assign80180_e122745_d_n7;
        locals.var_vgbgmt_dn8 = assign80180_e122745_d_n8;

        let (assign80190_e122755, assign80190_e122755_d_n0, assign80190_e122755_d_n2, assign80190_e122755_d_n4, assign80190_e122755_d_n5, assign80190_e122755_d_n6, assign80190_e122755_d_n7, assign80190_e122755_d_n8, assign80190_e122755_d_n9, assign80190_e122755_d_n10, assign80190_e122755_d_n13,) = {
    if (((locals.var_guard1870 != 0.0) && (locals.var_guard1869 == 0.0)) && (locals.var_guard1874 != 0.0)) {
        let assign80190_e122753: f64 = (-locals.var_vbsei);
        (assign80190_e122753, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80190_e122755;
        locals.var_vxbgmt_dn0 = assign80190_e122755_d_n0;
        locals.var_vxbgmt_dn2 = assign80190_e122755_d_n2;
        locals.var_vxbgmt_dn4 = assign80190_e122755_d_n4;
        locals.var_vxbgmt_dn5 = assign80190_e122755_d_n5;
        locals.var_vxbgmt_dn6 = assign80190_e122755_d_n6;
        locals.var_vxbgmt_dn7 = assign80190_e122755_d_n7;
        locals.var_vxbgmt_dn8 = assign80190_e122755_d_n8;
        locals.var_vxbgmt_dn9 = assign80190_e122755_d_n9;
        locals.var_vxbgmt_dn10 = assign80190_e122755_d_n10;
        locals.var_vxbgmt_dn13 = assign80190_e122755_d_n13;

        let assign80200_e122766: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1875 = assign80200_e122766;

        let (assign80210_e122777,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80210_e122777;

        let (assign80220_e122788,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign80220_e122788;

        let (assign80230_e122801, assign80230_e122801_d_n2, assign80230_e122801_d_n6, assign80230_e122801_d_n7, assign80230_e122801_d_n8,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80230_e122799: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80230_e122799, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80230_e122801;
        locals.var_vgbgmt_dn2 = assign80230_e122801_d_n2;
        locals.var_vgbgmt_dn6 = assign80230_e122801_d_n6;
        locals.var_vgbgmt_dn7 = assign80230_e122801_d_n7;
        locals.var_vgbgmt_dn8 = assign80230_e122801_d_n8;

        let (assign80240_e122814, assign80240_e122814_d_n0, assign80240_e122814_d_n2, assign80240_e122814_d_n4, assign80240_e122814_d_n5, assign80240_e122814_d_n6, assign80240_e122814_d_n7, assign80240_e122814_d_n8, assign80240_e122814_d_n9, assign80240_e122814_d_n10, assign80240_e122814_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80240_e122812: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign80240_e122812, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80240_e122814;
        locals.var_vxbgmt_dn0 = assign80240_e122814_d_n0;
        locals.var_vxbgmt_dn2 = assign80240_e122814_d_n2;
        locals.var_vxbgmt_dn4 = assign80240_e122814_d_n4;
        locals.var_vxbgmt_dn5 = assign80240_e122814_d_n5;
        locals.var_vxbgmt_dn6 = assign80240_e122814_d_n6;
        locals.var_vxbgmt_dn7 = assign80240_e122814_d_n7;
        locals.var_vxbgmt_dn8 = assign80240_e122814_d_n8;
        locals.var_vxbgmt_dn9 = assign80240_e122814_d_n9;
        locals.var_vxbgmt_dn10 = assign80240_e122814_d_n10;
        locals.var_vxbgmt_dn13 = assign80240_e122814_d_n13;

        let (assign80250_e122825,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80250_e122825;

        let (assign80260_e122840, assign80260_e122840_d_n0, assign80260_e122840_d_n2, assign80260_e122840_d_n4, assign80260_e122840_d_n5, assign80260_e122840_d_n6, assign80260_e122840_d_n7, assign80260_e122840_d_n8, assign80260_e122840_d_n9, assign80260_e122840_d_n10, assign80260_e122840_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80260_e122837: f64 = (p.p64 * p.p55);
        let assign80260_e122838: f64 = (p.p63 + assign80260_e122837);
        (assign80260_e122838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80260_e122840;
        locals.var_lover_func_dn0 = assign80260_e122840_d_n0;
        locals.var_lover_func_dn2 = assign80260_e122840_d_n2;
        locals.var_lover_func_dn4 = assign80260_e122840_d_n4;
        locals.var_lover_func_dn5 = assign80260_e122840_d_n5;
        locals.var_lover_func_dn6 = assign80260_e122840_d_n6;
        locals.var_lover_func_dn7 = assign80260_e122840_d_n7;
        locals.var_lover_func_dn8 = assign80260_e122840_d_n8;
        locals.var_lover_func_dn9 = assign80260_e122840_d_n9;
        locals.var_lover_func_dn10 = assign80260_e122840_d_n10;
        locals.var_lover_func_dn13 = assign80260_e122840_d_n13;

        let (assign80270_e122851, assign80270_e122851_d_n0, assign80270_e122851_d_n2, assign80270_e122851_d_n4, assign80270_e122851_d_n5, assign80270_e122851_d_n6, assign80270_e122851_d_n7, assign80270_e122851_d_n8, assign80270_e122851_d_n9, assign80270_e122851_d_n10, assign80270_e122851_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign80270_e122851;
        locals.var_wdep_func_dn0 = assign80270_e122851_d_n0;
        locals.var_wdep_func_dn2 = assign80270_e122851_d_n2;
        locals.var_wdep_func_dn4 = assign80270_e122851_d_n4;
        locals.var_wdep_func_dn5 = assign80270_e122851_d_n5;
        locals.var_wdep_func_dn6 = assign80270_e122851_d_n6;
        locals.var_wdep_func_dn7 = assign80270_e122851_d_n7;
        locals.var_wdep_func_dn8 = assign80270_e122851_d_n8;
        locals.var_wdep_func_dn9 = assign80270_e122851_d_n9;
        locals.var_wdep_func_dn10 = assign80270_e122851_d_n10;
        locals.var_wdep_func_dn13 = assign80270_e122851_d_n13;

        let (assign80280_e122862, assign80280_e122862_d_n0, assign80280_e122862_d_n2, assign80280_e122862_d_n4, assign80280_e122862_d_n5, assign80280_e122862_d_n6, assign80280_e122862_d_n7, assign80280_e122862_d_n8, assign80280_e122862_d_n9, assign80280_e122862_d_n10, assign80280_e122862_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign80280_e122862;
        locals.var_cnst0over_func_dn0 = assign80280_e122862_d_n0;
        locals.var_cnst0over_func_dn2 = assign80280_e122862_d_n2;
        locals.var_cnst0over_func_dn4 = assign80280_e122862_d_n4;
        locals.var_cnst0over_func_dn5 = assign80280_e122862_d_n5;
        locals.var_cnst0over_func_dn6 = assign80280_e122862_d_n6;
        locals.var_cnst0over_func_dn7 = assign80280_e122862_d_n7;
        locals.var_cnst0over_func_dn8 = assign80280_e122862_d_n8;
        locals.var_cnst0over_func_dn9 = assign80280_e122862_d_n9;
        locals.var_cnst0over_func_dn10 = assign80280_e122862_d_n10;
        locals.var_cnst0over_func_dn13 = assign80280_e122862_d_n13;

        let (assign80290_e122873,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80290_e122873;

        let (assign80300_e122885, assign80300_e122885_d_n0, assign80300_e122885_d_n2, assign80300_e122885_d_n4, assign80300_e122885_d_n5, assign80300_e122885_d_n6, assign80300_e122885_d_n7, assign80300_e122885_d_n8, assign80300_e122885_d_n9, assign80300_e122885_d_n10, assign80300_e122885_d_n13,) = {
    if (((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) {
        let assign80300_e122883: f64 = (-locals.var_lover_func);
        (assign80300_e122883, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80300_e122885;
        locals.var_lover_func_dn0 = assign80300_e122885_d_n0;
        locals.var_lover_func_dn2 = assign80300_e122885_d_n2;
        locals.var_lover_func_dn4 = assign80300_e122885_d_n4;
        locals.var_lover_func_dn5 = assign80300_e122885_d_n5;
        locals.var_lover_func_dn6 = assign80300_e122885_d_n6;
        locals.var_lover_func_dn7 = assign80300_e122885_d_n7;
        locals.var_lover_func_dn8 = assign80300_e122885_d_n8;
        locals.var_lover_func_dn9 = assign80300_e122885_d_n9;
        locals.var_lover_func_dn10 = assign80300_e122885_d_n10;
        locals.var_lover_func_dn13 = assign80300_e122885_d_n13;

        let assign80310_e122896: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1876 = assign80310_e122896;

        let (assign80320_e122910, assign80320_e122910_d_n0, assign80320_e122910_d_n2, assign80320_e122910_d_n4, assign80320_e122910_d_n5, assign80320_e122910_d_n6, assign80320_e122910_d_n7, assign80320_e122910_d_n8, assign80320_e122910_d_n9, assign80320_e122910_d_n10, assign80320_e122910_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80320_e122908: f64 = (-locals.var_lover_func);
        (assign80320_e122908, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80320_e122910;
        locals.var_lover_func_dn0 = assign80320_e122910_d_n0;
        locals.var_lover_func_dn2 = assign80320_e122910_d_n2;
        locals.var_lover_func_dn4 = assign80320_e122910_d_n4;
        locals.var_lover_func_dn5 = assign80320_e122910_d_n5;
        locals.var_lover_func_dn6 = assign80320_e122910_d_n6;
        locals.var_lover_func_dn7 = assign80320_e122910_d_n7;
        locals.var_lover_func_dn8 = assign80320_e122910_d_n8;
        locals.var_lover_func_dn9 = assign80320_e122910_d_n9;
        locals.var_lover_func_dn10 = assign80320_e122910_d_n10;
        locals.var_lover_func_dn13 = assign80320_e122910_d_n13;

        let (assign80330_e122923, assign80330_e122923_d_n0, assign80330_e122923_d_n2, assign80330_e122923_d_n4, assign80330_e122923_d_n5, assign80330_e122923_d_n6, assign80330_e122923_d_n7, assign80330_e122923_d_n8, assign80330_e122923_d_n9, assign80330_e122923_d_n10, assign80330_e122923_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign80330_e122923;
        locals.var_t1_dn0 = assign80330_e122923_d_n0;
        locals.var_t1_dn2 = assign80330_e122923_d_n2;
        locals.var_t1_dn4 = assign80330_e122923_d_n4;
        locals.var_t1_dn5 = assign80330_e122923_d_n5;
        locals.var_t1_dn6 = assign80330_e122923_d_n6;
        locals.var_t1_dn7 = assign80330_e122923_d_n7;
        locals.var_t1_dn8 = assign80330_e122923_d_n8;
        locals.var_t1_dn9 = assign80330_e122923_d_n9;
        locals.var_t1_dn10 = assign80330_e122923_d_n10;
        locals.var_t1_dn13 = assign80330_e122923_d_n13;

        let (assign80340_e122942, assign80340_e122942_d_n0, assign80340_e122942_d_n2, assign80340_e122942_d_n4, assign80340_e122942_d_n5, assign80340_e122942_d_n6, assign80340_e122942_d_n7, assign80340_e122942_d_n8, assign80340_e122942_d_n9, assign80340_e122942_d_n10, assign80340_e122942_d_n13,) = {
    if ((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80340_e122936: f64 = (locals.var_t1 * locals.var_t1);
        let assign80340_e122938: f64 = (assign80340_e122936 / locals.var_kjunc);
        let assign80340_e122940: f64 = (assign80340_e122938 - p.p137);
        (assign80340_e122940, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign80340_e122936 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign80340_e122942;
        locals.var_vxb_lim_dn0 = assign80340_e122942_d_n0;
        locals.var_vxb_lim_dn2 = assign80340_e122942_d_n2;
        locals.var_vxb_lim_dn4 = assign80340_e122942_d_n4;
        locals.var_vxb_lim_dn5 = assign80340_e122942_d_n5;
        locals.var_vxb_lim_dn6 = assign80340_e122942_d_n6;
        locals.var_vxb_lim_dn7 = assign80340_e122942_d_n7;
        locals.var_vxb_lim_dn8 = assign80340_e122942_d_n8;
        locals.var_vxb_lim_dn9 = assign80340_e122942_d_n9;
        locals.var_vxb_lim_dn10 = assign80340_e122942_d_n10;
        locals.var_vxb_lim_dn13 = assign80340_e122942_d_n13;

        let assign80350_e122945: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1877 = assign80350_e122945;

        let assign80360_e122952: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1878 = assign80360_e122952;

        let (assign80370_e122969, assign80370_e122969_d_n0, assign80370_e122969_d_n2, assign80370_e122969_d_n4, assign80370_e122969_d_n5, assign80370_e122969_d_n6, assign80370_e122969_d_n7, assign80370_e122969_d_n8, assign80370_e122969_d_n9, assign80370_e122969_d_n10, assign80370_e122969_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80370_e122969;
        locals.var_vxbgmt_dn0 = assign80370_e122969_d_n0;
        locals.var_vxbgmt_dn2 = assign80370_e122969_d_n2;
        locals.var_vxbgmt_dn4 = assign80370_e122969_d_n4;
        locals.var_vxbgmt_dn5 = assign80370_e122969_d_n5;
        locals.var_vxbgmt_dn6 = assign80370_e122969_d_n6;
        locals.var_vxbgmt_dn7 = assign80370_e122969_d_n7;
        locals.var_vxbgmt_dn8 = assign80370_e122969_d_n8;
        locals.var_vxbgmt_dn9 = assign80370_e122969_d_n9;
        locals.var_vxbgmt_dn10 = assign80370_e122969_d_n10;
        locals.var_vxbgmt_dn13 = assign80370_e122969_d_n13;

        let (assign80380_e122993, assign80380_e122993_d_n0, assign80380_e122993_d_n2, assign80380_e122993_d_n4, assign80380_e122993_d_n5, assign80380_e122993_d_n6, assign80380_e122993_d_n7, assign80380_e122993_d_n8, assign80380_e122993_d_n9, assign80380_e122993_d_n10, assign80380_e122993_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let (assign80380_e122991,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign80380_e122989: f64 = (-1.0);
                (assign80380_e122989,)
            } else {
                (1.0,)
            }
        };
        (assign80380_e122991, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign80380_e122993;
        locals.var_tmf3_dn0 = assign80380_e122993_d_n0;
        locals.var_tmf3_dn2 = assign80380_e122993_d_n2;
        locals.var_tmf3_dn4 = assign80380_e122993_d_n4;
        locals.var_tmf3_dn5 = assign80380_e122993_d_n5;
        locals.var_tmf3_dn6 = assign80380_e122993_d_n6;
        locals.var_tmf3_dn7 = assign80380_e122993_d_n7;
        locals.var_tmf3_dn8 = assign80380_e122993_d_n8;
        locals.var_tmf3_dn9 = assign80380_e122993_d_n9;
        locals.var_tmf3_dn10 = assign80380_e122993_d_n10;
        locals.var_tmf3_dn13 = assign80380_e122993_d_n13;

        let (assign80390_e123013, assign80390_e123013_d_n0, assign80390_e123013_d_n2, assign80390_e123013_d_n4, assign80390_e123013_d_n5, assign80390_e123013_d_n6, assign80390_e123013_d_n7, assign80390_e123013_d_n8, assign80390_e123013_d_n9, assign80390_e123013_d_n10, assign80390_e123013_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80390_e123011: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign80390_e123011, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign80390_e123013;
        locals.var_tmf4_dn0 = assign80390_e123013_d_n0;
        locals.var_tmf4_dn2 = assign80390_e123013_d_n2;
        locals.var_tmf4_dn4 = assign80390_e123013_d_n4;
        locals.var_tmf4_dn5 = assign80390_e123013_d_n5;
        locals.var_tmf4_dn6 = assign80390_e123013_d_n6;
        locals.var_tmf4_dn7 = assign80390_e123013_d_n7;
        locals.var_tmf4_dn8 = assign80390_e123013_d_n8;
        locals.var_tmf4_dn9 = assign80390_e123013_d_n9;
        locals.var_tmf4_dn10 = assign80390_e123013_d_n10;
        locals.var_tmf4_dn13 = assign80390_e123013_d_n13;

        let (assign80400_e123037, assign80400_e123037_d_n0, assign80400_e123037_d_n2, assign80400_e123037_d_n4, assign80400_e123037_d_n5, assign80400_e123037_d_n6, assign80400_e123037_d_n7, assign80400_e123037_d_n8, assign80400_e123037_d_n9, assign80400_e123037_d_n10, assign80400_e123037_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80400_e123032: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign80400_e123034: f64 = (assign80400_e123032).powf(p.p113);
        let assign80400_e123035: f64 = (1.0 + assign80400_e123034);
        (assign80400_e123035, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80400_e123032).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80400_e123034 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80400_e123032))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign80400_e123037;
        locals.var_tmf1_dn0 = assign80400_e123037_d_n0;
        locals.var_tmf1_dn2 = assign80400_e123037_d_n2;
        locals.var_tmf1_dn4 = assign80400_e123037_d_n4;
        locals.var_tmf1_dn5 = assign80400_e123037_d_n5;
        locals.var_tmf1_dn6 = assign80400_e123037_d_n6;
        locals.var_tmf1_dn7 = assign80400_e123037_d_n7;
        locals.var_tmf1_dn8 = assign80400_e123037_d_n8;
        locals.var_tmf1_dn9 = assign80400_e123037_d_n9;
        locals.var_tmf1_dn10 = assign80400_e123037_d_n10;
        locals.var_tmf1_dn13 = assign80400_e123037_d_n13;

        let (assign80410_e123059, assign80410_e123059_d_n0, assign80410_e123059_d_n2, assign80410_e123059_d_n4, assign80410_e123059_d_n5, assign80410_e123059_d_n6, assign80410_e123059_d_n7, assign80410_e123059_d_n8, assign80410_e123059_d_n9, assign80410_e123059_d_n10, assign80410_e123059_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80410_e123056: f64 = (1.0 / p.p113);
        let assign80410_e123057: f64 = (locals.var_tmf1).powf(assign80410_e123056);
        (assign80410_e123057, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80410_e123056) as f64).is_finite() && ((assign80410_e123056) as f64).fract() == 0.0 { if assign80410_e123056 == 0.0 { 0.0 } else { (assign80410_e123056 * ((locals.var_tmf1).powf(assign80410_e123056 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign80410_e123057 * (assign80410_e123056 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80410_e123059;
        locals.var_tmf2_dn0 = assign80410_e123059_d_n0;
        locals.var_tmf2_dn2 = assign80410_e123059_d_n2;
        locals.var_tmf2_dn4 = assign80410_e123059_d_n4;
        locals.var_tmf2_dn5 = assign80410_e123059_d_n5;
        locals.var_tmf2_dn6 = assign80410_e123059_d_n6;
        locals.var_tmf2_dn7 = assign80410_e123059_d_n7;
        locals.var_tmf2_dn8 = assign80410_e123059_d_n8;
        locals.var_tmf2_dn9 = assign80410_e123059_d_n9;
        locals.var_tmf2_dn10 = assign80410_e123059_d_n10;
        locals.var_tmf2_dn13 = assign80410_e123059_d_n13;

        let (assign80420_e123081, assign80420_e123081_d_n0, assign80420_e123081_d_n2, assign80420_e123081_d_n4, assign80420_e123081_d_n5, assign80420_e123081_d_n6, assign80420_e123081_d_n7, assign80420_e123081_d_n8, assign80420_e123081_d_n9, assign80420_e123081_d_n10, assign80420_e123081_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 == 0.0)) {
        let assign80420_e123077: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign80420_e123079: f64 = (assign80420_e123077 / locals.var_tmf2);
        (assign80420_e123079, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign80420_e123077 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80420_e123081;
        locals.var_vxbgmt_dn0 = assign80420_e123081_d_n0;
        locals.var_vxbgmt_dn2 = assign80420_e123081_d_n2;
        locals.var_vxbgmt_dn4 = assign80420_e123081_d_n4;
        locals.var_vxbgmt_dn5 = assign80420_e123081_d_n5;
        locals.var_vxbgmt_dn6 = assign80420_e123081_d_n6;
        locals.var_vxbgmt_dn7 = assign80420_e123081_d_n7;
        locals.var_vxbgmt_dn8 = assign80420_e123081_d_n8;
        locals.var_vxbgmt_dn9 = assign80420_e123081_d_n9;
        locals.var_vxbgmt_dn10 = assign80420_e123081_d_n10;
        locals.var_vxbgmt_dn13 = assign80420_e123081_d_n13;

        let (assign80430_e123109, assign80430_e123109_d_n0, assign80430_e123109_d_n2, assign80430_e123109_d_n4, assign80430_e123109_d_n5, assign80430_e123109_d_n6, assign80430_e123109_d_n7, assign80430_e123109_d_n8, assign80430_e123109_d_n9, assign80430_e123109_d_n10, assign80430_e123109_d_n13,) = {
    if (((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) {
        let assign80430_e123096: f64 = (locals.var_vxbgmt + p.p137);
        let assign80430_e123099: f64 = (locals.var_vxbgmt + p.p137);
        let assign80430_e123100: f64 = (assign80430_e123096 * assign80430_e123099);
        let assign80430_e123103: f64 = (4.0 * 0.1);
        let assign80430_e123105: f64 = (assign80430_e123103 * 0.1);
        let assign80430_e123106: f64 = (assign80430_e123100 + assign80430_e123105);
        let assign80430_e123107: f64 = (assign80430_e123106).sqrt();
        (assign80430_e123107, (((locals.var_vxbgmt_dn0 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn0)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn2 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn2)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn4 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn4)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn5 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn5)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn6 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn6)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn7 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn7)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn8 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn8)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn9 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn9)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn10 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn10)) / (2.0 * assign80430_e123107)), (((locals.var_vxbgmt_dn13 * assign80430_e123099) + (assign80430_e123096 * locals.var_vxbgmt_dn13)) / (2.0 * assign80430_e123107)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80430_e123109;
        locals.var_tmf2_dn0 = assign80430_e123109_d_n0;
        locals.var_tmf2_dn2 = assign80430_e123109_d_n2;
        locals.var_tmf2_dn4 = assign80430_e123109_d_n4;
        locals.var_tmf2_dn5 = assign80430_e123109_d_n5;
        locals.var_tmf2_dn6 = assign80430_e123109_d_n6;
        locals.var_tmf2_dn7 = assign80430_e123109_d_n7;
        locals.var_tmf2_dn8 = assign80430_e123109_d_n8;
        locals.var_tmf2_dn9 = assign80430_e123109_d_n9;
        locals.var_tmf2_dn10 = assign80430_e123109_d_n10;
        locals.var_tmf2_dn13 = assign80430_e123109_d_n13;

    }

    pub(super) fn stamp_transient_block_280(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign80440_e123132, assign80440_e123132_d_n0, assign80440_e123132_d_n2, assign80440_e123132_d_n4, assign80440_e123132_d_n5, assign80440_e123132_d_n6, assign80440_e123132_d_n7, assign80440_e123132_d_n8, assign80440_e123132_d_n9, assign80440_e123132_d_n10, assign80440_e123132_d_n13,) = {
    if (((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) {
        let assign80440_e123126: f64 = (locals.var_vxbgmt + p.p137);
        let assign80440_e123128: f64 = (assign80440_e123126 / locals.var_tmf2);
        let assign80440_e123129: f64 = (1.0 + assign80440_e123128);
        let assign80440_e123130: f64 = (0.5 * assign80440_e123129);
        (assign80440_e123130, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign80440_e123126 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign80440_e123132;
        locals.var_t9_dn0 = assign80440_e123132_d_n0;
        locals.var_t9_dn2 = assign80440_e123132_d_n2;
        locals.var_t9_dn4 = assign80440_e123132_d_n4;
        locals.var_t9_dn5 = assign80440_e123132_d_n5;
        locals.var_t9_dn6 = assign80440_e123132_d_n6;
        locals.var_t9_dn7 = assign80440_e123132_d_n7;
        locals.var_t9_dn8 = assign80440_e123132_d_n8;
        locals.var_t9_dn9 = assign80440_e123132_d_n9;
        locals.var_t9_dn10 = assign80440_e123132_d_n10;
        locals.var_t9_dn13 = assign80440_e123132_d_n13;

        let (assign80450_e123153, assign80450_e123153_d_n0, assign80450_e123153_d_n2, assign80450_e123153_d_n4, assign80450_e123153_d_n5, assign80450_e123153_d_n6, assign80450_e123153_d_n7, assign80450_e123153_d_n8, assign80450_e123153_d_n9, assign80450_e123153_d_n10, assign80450_e123153_d_n13,) = {
    if (((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) {
        let assign80450_e123148: f64 = (locals.var_vxbgmt + p.p137);
        let assign80450_e123150: f64 = (assign80450_e123148 + locals.var_tmf2);
        let assign80450_e123151: f64 = (0.5 * assign80450_e123150);
        (assign80450_e123151, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign80450_e123153;
        locals.var_t2_dn0 = assign80450_e123153_d_n0;
        locals.var_t2_dn2 = assign80450_e123153_d_n2;
        locals.var_t2_dn4 = assign80450_e123153_d_n4;
        locals.var_t2_dn5 = assign80450_e123153_d_n5;
        locals.var_t2_dn6 = assign80450_e123153_d_n6;
        locals.var_t2_dn7 = assign80450_e123153_d_n7;
        locals.var_t2_dn8 = assign80450_e123153_d_n8;
        locals.var_t2_dn9 = assign80450_e123153_d_n9;
        locals.var_t2_dn10 = assign80450_e123153_d_n10;
        locals.var_t2_dn13 = assign80450_e123153_d_n13;

        let assign80460_e123156: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1879 = assign80460_e123156;

        let (assign80470_e123173, assign80470_e123173_d_n0, assign80470_e123173_d_n2, assign80470_e123173_d_n4, assign80470_e123173_d_n5, assign80470_e123173_d_n6, assign80470_e123173_d_n7, assign80470_e123173_d_n8, assign80470_e123173_d_n9, assign80470_e123173_d_n10, assign80470_e123173_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign80470_e123173;
        locals.var_t2_dn0 = assign80470_e123173_d_n0;
        locals.var_t2_dn2 = assign80470_e123173_d_n2;
        locals.var_t2_dn4 = assign80470_e123173_d_n4;
        locals.var_t2_dn5 = assign80470_e123173_d_n5;
        locals.var_t2_dn6 = assign80470_e123173_d_n6;
        locals.var_t2_dn7 = assign80470_e123173_d_n7;
        locals.var_t2_dn8 = assign80470_e123173_d_n8;
        locals.var_t2_dn9 = assign80470_e123173_d_n9;
        locals.var_t2_dn10 = assign80470_e123173_d_n10;
        locals.var_t2_dn13 = assign80470_e123173_d_n13;

        let (assign80480_e123190, assign80480_e123190_d_n0, assign80480_e123190_d_n2, assign80480_e123190_d_n4, assign80480_e123190_d_n5, assign80480_e123190_d_n6, assign80480_e123190_d_n7, assign80480_e123190_d_n8, assign80480_e123190_d_n9, assign80480_e123190_d_n10, assign80480_e123190_d_n13,) = {
    if ((((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign80480_e123190;
        locals.var_t9_dn0 = assign80480_e123190_d_n0;
        locals.var_t9_dn2 = assign80480_e123190_d_n2;
        locals.var_t9_dn4 = assign80480_e123190_d_n4;
        locals.var_t9_dn5 = assign80480_e123190_d_n5;
        locals.var_t9_dn6 = assign80480_e123190_d_n6;
        locals.var_t9_dn7 = assign80480_e123190_d_n7;
        locals.var_t9_dn8 = assign80480_e123190_d_n8;
        locals.var_t9_dn9 = assign80480_e123190_d_n9;
        locals.var_t9_dn10 = assign80480_e123190_d_n10;
        locals.var_t9_dn13 = assign80480_e123190_d_n13;

        let (assign80490_e123210, assign80490_e123210_d_n0, assign80490_e123210_d_n2, assign80490_e123210_d_n4, assign80490_e123210_d_n5, assign80490_e123210_d_n6, assign80490_e123210_d_n7, assign80490_e123210_d_n8, assign80490_e123210_d_n9, assign80490_e123210_d_n10, assign80490_e123210_d_n13,) = {
    if (((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) {
        let assign80490_e123205: f64 = (locals.var_kjunc * locals.var_t2);
        let assign80490_e123206: f64 = (assign80490_e123205).sqrt();
        let assign80490_e123208: f64 = (assign80490_e123206 * p.p432);
        (assign80490_e123208, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign80490_e123206)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign80490_e123206)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign80490_e123210;
        locals.var_wjunc0_dn0 = assign80490_e123210_d_n0;
        locals.var_wjunc0_dn2 = assign80490_e123210_d_n2;
        locals.var_wjunc0_dn4 = assign80490_e123210_d_n4;
        locals.var_wjunc0_dn5 = assign80490_e123210_d_n5;
        locals.var_wjunc0_dn6 = assign80490_e123210_d_n6;
        locals.var_wjunc0_dn7 = assign80490_e123210_d_n7;
        locals.var_wjunc0_dn8 = assign80490_e123210_d_n8;
        locals.var_wjunc0_dn9 = assign80490_e123210_d_n9;
        locals.var_wjunc0_dn10 = assign80490_e123210_d_n10;
        locals.var_wjunc0_dn13 = assign80490_e123210_d_n13;

        let (assign80500_e123227, assign80500_e123227_d_n0, assign80500_e123227_d_n2, assign80500_e123227_d_n4, assign80500_e123227_d_n5, assign80500_e123227_d_n6, assign80500_e123227_d_n7, assign80500_e123227_d_n8, assign80500_e123227_d_n9, assign80500_e123227_d_n10, assign80500_e123227_d_n13,) = {
    if (((((locals.var_guard1871 != 0.0) && (!((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)))) && (locals.var_guard1875 != 0.0)) && (locals.var_guard1876 != 0.0)) && (locals.var_guard1877 != 0.0)) {
        let assign80500_e123225: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign80500_e123225, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign80500_e123227;
        locals.var_lover_func_dn0 = assign80500_e123227_d_n0;
        locals.var_lover_func_dn2 = assign80500_e123227_d_n2;
        locals.var_lover_func_dn4 = assign80500_e123227_d_n4;
        locals.var_lover_func_dn5 = assign80500_e123227_d_n5;
        locals.var_lover_func_dn6 = assign80500_e123227_d_n6;
        locals.var_lover_func_dn7 = assign80500_e123227_d_n7;
        locals.var_lover_func_dn8 = assign80500_e123227_d_n8;
        locals.var_lover_func_dn9 = assign80500_e123227_d_n9;
        locals.var_lover_func_dn10 = assign80500_e123227_d_n10;
        locals.var_lover_func_dn13 = assign80500_e123227_d_n13;

        let assign80510_e123246: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1880 = assign80510_e123246;

        let (assign80520_e123259,) = {
    if (((locals.var_guard1872 != 0.0) && (!(((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)) || (locals.var_guard1871 != 0.0)))) && (locals.var_guard1880 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80520_e123259;

        let (assign80530_e123274, assign80530_e123274_d_n2, assign80530_e123274_d_n6, assign80530_e123274_d_n7, assign80530_e123274_d_n8,) = {
    if (((locals.var_guard1872 != 0.0) && (!(((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)) || (locals.var_guard1871 != 0.0)))) && (locals.var_guard1880 != 0.0)) {
        let assign80530_e123272: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign80530_e123272, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign80530_e123274;
        locals.var_vgbgmt_dn2 = assign80530_e123274_d_n2;
        locals.var_vgbgmt_dn6 = assign80530_e123274_d_n6;
        locals.var_vgbgmt_dn7 = assign80530_e123274_d_n7;
        locals.var_vgbgmt_dn8 = assign80530_e123274_d_n8;

        let (assign80540_e123289, assign80540_e123289_d_n0, assign80540_e123289_d_n2, assign80540_e123289_d_n4, assign80540_e123289_d_n5, assign80540_e123289_d_n6, assign80540_e123289_d_n7, assign80540_e123289_d_n8, assign80540_e123289_d_n9, assign80540_e123289_d_n10, assign80540_e123289_d_n13,) = {
    if (((locals.var_guard1872 != 0.0) && (!(((locals.var_guard1869 != 0.0) || (locals.var_guard1870 != 0.0)) || (locals.var_guard1871 != 0.0)))) && (locals.var_guard1880 != 0.0)) {
        let assign80540_e123287: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign80540_e123287, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign80540_e123289;
        locals.var_vxbgmt_dn0 = assign80540_e123289_d_n0;
        locals.var_vxbgmt_dn2 = assign80540_e123289_d_n2;
        locals.var_vxbgmt_dn4 = assign80540_e123289_d_n4;
        locals.var_vxbgmt_dn5 = assign80540_e123289_d_n5;
        locals.var_vxbgmt_dn6 = assign80540_e123289_d_n6;
        locals.var_vxbgmt_dn7 = assign80540_e123289_d_n7;
        locals.var_vxbgmt_dn8 = assign80540_e123289_d_n8;
        locals.var_vxbgmt_dn9 = assign80540_e123289_d_n9;
        locals.var_vxbgmt_dn10 = assign80540_e123289_d_n10;
        locals.var_vxbgmt_dn13 = assign80540_e123289_d_n13;

        let (assign80550_e123293, assign80550_e123293_d_n0, assign80550_e123293_d_n2, assign80550_e123293_d_n4, assign80550_e123293_d_n5, assign80550_e123293_d_n6, assign80550_e123293_d_n7, assign80550_e123293_d_n8, assign80550_e123293_d_n9, assign80550_e123293_d_n10, assign80550_e123293_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1888, locals.var_vbs_bnd_over__blk1888_dn0, locals.var_vbs_bnd_over__blk1888_dn2, locals.var_vbs_bnd_over__blk1888_dn4, locals.var_vbs_bnd_over__blk1888_dn5, locals.var_vbs_bnd_over__blk1888_dn6, locals.var_vbs_bnd_over__blk1888_dn7, locals.var_vbs_bnd_over__blk1888_dn8, locals.var_vbs_bnd_over__blk1888_dn9, locals.var_vbs_bnd_over__blk1888_dn10, locals.var_vbs_bnd_over__blk1888_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1888 = assign80550_e123293;
        locals.var_vbs_bnd_over__blk1888_dn0 = assign80550_e123293_d_n0;
        locals.var_vbs_bnd_over__blk1888_dn2 = assign80550_e123293_d_n2;
        locals.var_vbs_bnd_over__blk1888_dn4 = assign80550_e123293_d_n4;
        locals.var_vbs_bnd_over__blk1888_dn5 = assign80550_e123293_d_n5;
        locals.var_vbs_bnd_over__blk1888_dn6 = assign80550_e123293_d_n6;
        locals.var_vbs_bnd_over__blk1888_dn7 = assign80550_e123293_d_n7;
        locals.var_vbs_bnd_over__blk1888_dn8 = assign80550_e123293_d_n8;
        locals.var_vbs_bnd_over__blk1888_dn9 = assign80550_e123293_d_n9;
        locals.var_vbs_bnd_over__blk1888_dn10 = assign80550_e123293_d_n10;
        locals.var_vbs_bnd_over__blk1888_dn13 = assign80550_e123293_d_n13;

        let (assign80570_e123301,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign80570_e123301;

        let (assign80580_e123305, assign80580_e123305_d_n0, assign80580_e123305_d_n2, assign80580_e123305_d_n4, assign80580_e123305_d_n5, assign80580_e123305_d_n6, assign80580_e123305_d_n7, assign80580_e123305_d_n8, assign80580_e123305_d_n9, assign80580_e123305_d_n10, assign80580_e123305_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign80580_e123305;
        locals.var_fb_dn0 = assign80580_e123305_d_n0;
        locals.var_fb_dn2 = assign80580_e123305_d_n2;
        locals.var_fb_dn4 = assign80580_e123305_d_n4;
        locals.var_fb_dn5 = assign80580_e123305_d_n5;
        locals.var_fb_dn6 = assign80580_e123305_d_n6;
        locals.var_fb_dn7 = assign80580_e123305_d_n7;
        locals.var_fb_dn8 = assign80580_e123305_d_n8;
        locals.var_fb_dn9 = assign80580_e123305_d_n9;
        locals.var_fb_dn10 = assign80580_e123305_d_n10;
        locals.var_fb_dn13 = assign80580_e123305_d_n13;

        let (assign80590_e123309, assign80590_e123309_d_n0, assign80590_e123309_d_n2, assign80590_e123309_d_n4, assign80590_e123309_d_n5, assign80590_e123309_d_n6, assign80590_e123309_d_n7, assign80590_e123309_d_n8, assign80590_e123309_d_n9, assign80590_e123309_d_n10, assign80590_e123309_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign80590_e123309;
        locals.var_fs01_dn0 = assign80590_e123309_d_n0;
        locals.var_fs01_dn2 = assign80590_e123309_d_n2;
        locals.var_fs01_dn4 = assign80590_e123309_d_n4;
        locals.var_fs01_dn5 = assign80590_e123309_d_n5;
        locals.var_fs01_dn6 = assign80590_e123309_d_n6;
        locals.var_fs01_dn7 = assign80590_e123309_d_n7;
        locals.var_fs01_dn8 = assign80590_e123309_d_n8;
        locals.var_fs01_dn9 = assign80590_e123309_d_n9;
        locals.var_fs01_dn10 = assign80590_e123309_d_n10;
        locals.var_fs01_dn13 = assign80590_e123309_d_n13;

        let (assign80600_e123313, assign80600_e123313_d_n0, assign80600_e123313_d_n2, assign80600_e123313_d_n4, assign80600_e123313_d_n5, assign80600_e123313_d_n6, assign80600_e123313_d_n7, assign80600_e123313_d_n8, assign80600_e123313_d_n9, assign80600_e123313_d_n10, assign80600_e123313_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign80600_e123313;
        locals.var_fs02_dn0 = assign80600_e123313_d_n0;
        locals.var_fs02_dn2 = assign80600_e123313_d_n2;
        locals.var_fs02_dn4 = assign80600_e123313_d_n4;
        locals.var_fs02_dn5 = assign80600_e123313_d_n5;
        locals.var_fs02_dn6 = assign80600_e123313_d_n6;
        locals.var_fs02_dn7 = assign80600_e123313_d_n7;
        locals.var_fs02_dn8 = assign80600_e123313_d_n8;
        locals.var_fs02_dn9 = assign80600_e123313_d_n9;
        locals.var_fs02_dn10 = assign80600_e123313_d_n10;
        locals.var_fs02_dn13 = assign80600_e123313_d_n13;

        let (assign80610_e123317, assign80610_e123317_d_n0, assign80610_e123317_d_n2, assign80610_e123317_d_n4, assign80610_e123317_d_n5, assign80610_e123317_d_n6, assign80610_e123317_d_n7, assign80610_e123317_d_n8, assign80610_e123317_d_n9, assign80610_e123317_d_n10, assign80610_e123317_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign80610_e123317;
        locals.var_fs0_dn0 = assign80610_e123317_d_n0;
        locals.var_fs0_dn2 = assign80610_e123317_d_n2;
        locals.var_fs0_dn4 = assign80610_e123317_d_n4;
        locals.var_fs0_dn5 = assign80610_e123317_d_n5;
        locals.var_fs0_dn6 = assign80610_e123317_d_n6;
        locals.var_fs0_dn7 = assign80610_e123317_d_n7;
        locals.var_fs0_dn8 = assign80610_e123317_d_n8;
        locals.var_fs0_dn9 = assign80610_e123317_d_n9;
        locals.var_fs0_dn10 = assign80610_e123317_d_n10;
        locals.var_fs0_dn13 = assign80610_e123317_d_n13;

        let (assign80620_e123321, assign80620_e123321_d_n0, assign80620_e123321_d_n2, assign80620_e123321_d_n4, assign80620_e123321_d_n5, assign80620_e123321_d_n6, assign80620_e123321_d_n7, assign80620_e123321_d_n8, assign80620_e123321_d_n9, assign80620_e123321_d_n10, assign80620_e123321_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign80620_e123321;
        locals.var_dps0_dn0 = assign80620_e123321_d_n0;
        locals.var_dps0_dn2 = assign80620_e123321_d_n2;
        locals.var_dps0_dn4 = assign80620_e123321_d_n4;
        locals.var_dps0_dn5 = assign80620_e123321_d_n5;
        locals.var_dps0_dn6 = assign80620_e123321_d_n6;
        locals.var_dps0_dn7 = assign80620_e123321_d_n7;
        locals.var_dps0_dn8 = assign80620_e123321_d_n8;
        locals.var_dps0_dn9 = assign80620_e123321_d_n9;
        locals.var_dps0_dn10 = assign80620_e123321_d_n10;
        locals.var_dps0_dn13 = assign80620_e123321_d_n13;

        let (assign80630_e123325, assign80630_e123325_d_n0, assign80630_e123325_d_n2, assign80630_e123325_d_n4, assign80630_e123325_d_n5, assign80630_e123325_d_n6, assign80630_e123325_d_n7, assign80630_e123325_d_n8, assign80630_e123325_d_n9, assign80630_e123325_d_n10, assign80630_e123325_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign80630_e123325;
        locals.var_fs0_dps0_dn0 = assign80630_e123325_d_n0;
        locals.var_fs0_dps0_dn2 = assign80630_e123325_d_n2;
        locals.var_fs0_dps0_dn4 = assign80630_e123325_d_n4;
        locals.var_fs0_dps0_dn5 = assign80630_e123325_d_n5;
        locals.var_fs0_dps0_dn6 = assign80630_e123325_d_n6;
        locals.var_fs0_dps0_dn7 = assign80630_e123325_d_n7;
        locals.var_fs0_dps0_dn8 = assign80630_e123325_d_n8;
        locals.var_fs0_dps0_dn9 = assign80630_e123325_d_n9;
        locals.var_fs0_dps0_dn10 = assign80630_e123325_d_n10;
        locals.var_fs0_dps0_dn13 = assign80630_e123325_d_n13;

        let (assign80640_e123329, assign80640_e123329_d_n0, assign80640_e123329_d_n2, assign80640_e123329_d_n4, assign80640_e123329_d_n5, assign80640_e123329_d_n6, assign80640_e123329_d_n7, assign80640_e123329_d_n8, assign80640_e123329_d_n9, assign80640_e123329_d_n10, assign80640_e123329_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign80640_e123329;
        locals.var_fs02_dps0_dn0 = assign80640_e123329_d_n0;
        locals.var_fs02_dps0_dn2 = assign80640_e123329_d_n2;
        locals.var_fs02_dps0_dn4 = assign80640_e123329_d_n4;
        locals.var_fs02_dps0_dn5 = assign80640_e123329_d_n5;
        locals.var_fs02_dps0_dn6 = assign80640_e123329_d_n6;
        locals.var_fs02_dps0_dn7 = assign80640_e123329_d_n7;
        locals.var_fs02_dps0_dn8 = assign80640_e123329_d_n8;
        locals.var_fs02_dps0_dn9 = assign80640_e123329_d_n9;
        locals.var_fs02_dps0_dn10 = assign80640_e123329_d_n10;
        locals.var_fs02_dps0_dn13 = assign80640_e123329_d_n13;

        let (assign80650_e123333, assign80650_e123333_d_n0, assign80650_e123333_d_n2, assign80650_e123333_d_n4, assign80650_e123333_d_n5, assign80650_e123333_d_n6, assign80650_e123333_d_n7, assign80650_e123333_d_n8, assign80650_e123333_d_n9, assign80650_e123333_d_n10, assign80650_e123333_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign80650_e123333;
        locals.var_fb_dpss_dn0 = assign80650_e123333_d_n0;
        locals.var_fb_dpss_dn2 = assign80650_e123333_d_n2;
        locals.var_fb_dpss_dn4 = assign80650_e123333_d_n4;
        locals.var_fb_dpss_dn5 = assign80650_e123333_d_n5;
        locals.var_fb_dpss_dn6 = assign80650_e123333_d_n6;
        locals.var_fb_dpss_dn7 = assign80650_e123333_d_n7;
        locals.var_fb_dpss_dn8 = assign80650_e123333_d_n8;
        locals.var_fb_dpss_dn9 = assign80650_e123333_d_n9;
        locals.var_fb_dpss_dn10 = assign80650_e123333_d_n10;
        locals.var_fb_dpss_dn13 = assign80650_e123333_d_n13;

        let (assign80660_e123337, assign80660_e123337_d_n0, assign80660_e123337_d_n2, assign80660_e123337_d_n4, assign80660_e123337_d_n5, assign80660_e123337_d_n6, assign80660_e123337_d_n7, assign80660_e123337_d_n8, assign80660_e123337_d_n9, assign80660_e123337_d_n10, assign80660_e123337_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign80660_e123337;
        locals.var_fs01_dps0_dn0 = assign80660_e123337_d_n0;
        locals.var_fs01_dps0_dn2 = assign80660_e123337_d_n2;
        locals.var_fs01_dps0_dn4 = assign80660_e123337_d_n4;
        locals.var_fs01_dps0_dn5 = assign80660_e123337_d_n5;
        locals.var_fs01_dps0_dn6 = assign80660_e123337_d_n6;
        locals.var_fs01_dps0_dn7 = assign80660_e123337_d_n7;
        locals.var_fs01_dps0_dn8 = assign80660_e123337_d_n8;
        locals.var_fs01_dps0_dn9 = assign80660_e123337_d_n9;
        locals.var_fs01_dps0_dn10 = assign80660_e123337_d_n10;
        locals.var_fs01_dps0_dn13 = assign80660_e123337_d_n13;

        let (assign80670_e123341, assign80670_e123341_d_n0, assign80670_e123341_d_n2, assign80670_e123341_d_n4, assign80670_e123341_d_n5, assign80670_e123341_d_n6, assign80670_e123341_d_n7, assign80670_e123341_d_n8, assign80670_e123341_d_n9, assign80670_e123341_d_n10, assign80670_e123341_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign80670_e123341;
        locals.var_chi_1_dn0 = assign80670_e123341_d_n0;
        locals.var_chi_1_dn2 = assign80670_e123341_d_n2;
        locals.var_chi_1_dn4 = assign80670_e123341_d_n4;
        locals.var_chi_1_dn5 = assign80670_e123341_d_n5;
        locals.var_chi_1_dn6 = assign80670_e123341_d_n6;
        locals.var_chi_1_dn7 = assign80670_e123341_d_n7;
        locals.var_chi_1_dn8 = assign80670_e123341_d_n8;
        locals.var_chi_1_dn9 = assign80670_e123341_d_n9;
        locals.var_chi_1_dn10 = assign80670_e123341_d_n10;
        locals.var_chi_1_dn13 = assign80670_e123341_d_n13;

        let (assign80680_e123345, assign80680_e123345_d_n0, assign80680_e123345_d_n2, assign80680_e123345_d_n4, assign80680_e123345_d_n5, assign80680_e123345_d_n6, assign80680_e123345_d_n7, assign80680_e123345_d_n8, assign80680_e123345_d_n9, assign80680_e123345_d_n10, assign80680_e123345_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign80680_e123345;
        locals.var_chi_a_dn0 = assign80680_e123345_d_n0;
        locals.var_chi_a_dn2 = assign80680_e123345_d_n2;
        locals.var_chi_a_dn4 = assign80680_e123345_d_n4;
        locals.var_chi_a_dn5 = assign80680_e123345_d_n5;
        locals.var_chi_a_dn6 = assign80680_e123345_d_n6;
        locals.var_chi_a_dn7 = assign80680_e123345_d_n7;
        locals.var_chi_a_dn8 = assign80680_e123345_d_n8;
        locals.var_chi_a_dn9 = assign80680_e123345_d_n9;
        locals.var_chi_a_dn10 = assign80680_e123345_d_n10;
        locals.var_chi_a_dn13 = assign80680_e123345_d_n13;

        let (assign80690_e123349, assign80690_e123349_d_n0, assign80690_e123349_d_n2, assign80690_e123349_d_n4, assign80690_e123349_d_n5, assign80690_e123349_d_n6, assign80690_e123349_d_n7, assign80690_e123349_d_n8, assign80690_e123349_d_n9, assign80690_e123349_d_n10, assign80690_e123349_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign80690_e123349;
        locals.var_chi_b_dn0 = assign80690_e123349_d_n0;
        locals.var_chi_b_dn2 = assign80690_e123349_d_n2;
        locals.var_chi_b_dn4 = assign80690_e123349_d_n4;
        locals.var_chi_b_dn5 = assign80690_e123349_d_n5;
        locals.var_chi_b_dn6 = assign80690_e123349_d_n6;
        locals.var_chi_b_dn7 = assign80690_e123349_d_n7;
        locals.var_chi_b_dn8 = assign80690_e123349_d_n8;
        locals.var_chi_b_dn9 = assign80690_e123349_d_n9;
        locals.var_chi_b_dn10 = assign80690_e123349_d_n10;
        locals.var_chi_b_dn13 = assign80690_e123349_d_n13;

        let (assign80700_e123354,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80700_e123352: f64 = (-1.0);
        (assign80700_e123352,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign80700_e123354;

        let (assign80710_e123358, assign80710_e123358_d_n0, assign80710_e123358_d_n2, assign80710_e123358_d_n4, assign80710_e123358_d_n5, assign80710_e123358_d_n6, assign80710_e123358_d_n7, assign80710_e123358_d_n8, assign80710_e123358_d_n9, assign80710_e123358_d_n10, assign80710_e123358_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1890 = assign80710_e123358;
        locals.var_ps0ld_ini__blk1890_dn0 = assign80710_e123358_d_n0;
        locals.var_ps0ld_ini__blk1890_dn2 = assign80710_e123358_d_n2;
        locals.var_ps0ld_ini__blk1890_dn4 = assign80710_e123358_d_n4;
        locals.var_ps0ld_ini__blk1890_dn5 = assign80710_e123358_d_n5;
        locals.var_ps0ld_ini__blk1890_dn6 = assign80710_e123358_d_n6;
        locals.var_ps0ld_ini__blk1890_dn7 = assign80710_e123358_d_n7;
        locals.var_ps0ld_ini__blk1890_dn8 = assign80710_e123358_d_n8;
        locals.var_ps0ld_ini__blk1890_dn9 = assign80710_e123358_d_n9;
        locals.var_ps0ld_ini__blk1890_dn10 = assign80710_e123358_d_n10;
        locals.var_ps0ld_ini__blk1890_dn13 = assign80710_e123358_d_n13;

        let (assign80720_e123362, assign80720_e123362_d_n0, assign80720_e123362_d_n2, assign80720_e123362_d_n4, assign80720_e123362_d_n5, assign80720_e123362_d_n6, assign80720_e123362_d_n7, assign80720_e123362_d_n8, assign80720_e123362_d_n9, assign80720_e123362_d_n10, assign80720_e123362_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1891, locals.var_fbsq__blk1891_dn0, locals.var_fbsq__blk1891_dn2, locals.var_fbsq__blk1891_dn4, locals.var_fbsq__blk1891_dn5, locals.var_fbsq__blk1891_dn6, locals.var_fbsq__blk1891_dn7, locals.var_fbsq__blk1891_dn8, locals.var_fbsq__blk1891_dn9, locals.var_fbsq__blk1891_dn10, locals.var_fbsq__blk1891_dn13,)
    }
};
        locals.var_fbsq__blk1891 = assign80720_e123362;
        locals.var_fbsq__blk1891_dn0 = assign80720_e123362_d_n0;
        locals.var_fbsq__blk1891_dn2 = assign80720_e123362_d_n2;
        locals.var_fbsq__blk1891_dn4 = assign80720_e123362_d_n4;
        locals.var_fbsq__blk1891_dn5 = assign80720_e123362_d_n5;
        locals.var_fbsq__blk1891_dn6 = assign80720_e123362_d_n6;
        locals.var_fbsq__blk1891_dn7 = assign80720_e123362_d_n7;
        locals.var_fbsq__blk1891_dn8 = assign80720_e123362_d_n8;
        locals.var_fbsq__blk1891_dn9 = assign80720_e123362_d_n9;
        locals.var_fbsq__blk1891_dn10 = assign80720_e123362_d_n10;
        locals.var_fbsq__blk1891_dn13 = assign80720_e123362_d_n13;

        let (assign80730_e123373, assign80730_e123373_d_n0, assign80730_e123373_d_n2, assign80730_e123373_d_n4, assign80730_e123373_d_n5, assign80730_e123373_d_n6, assign80730_e123373_d_n7, assign80730_e123373_d_n8, assign80730_e123373_d_n9, assign80730_e123373_d_n10, assign80730_e123373_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80730_e123366: f64 = (2.0 * locals.var_beta_inv);
        let assign80730_e123369: f64 = (locals.var_nover_func / locals.var_nin);
        let assign80730_e123370: f64 = (assign80730_e123369).ln();
        let assign80730_e123371: f64 = (assign80730_e123366 * assign80730_e123370);
        (assign80730_e123371, (((2.0 * locals.var_beta_inv_dn0) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn2) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn4) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn5) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn6) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn7) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn8) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn9) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn10) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))), (((2.0 * locals.var_beta_inv_dn13) * assign80730_e123370) + (assign80730_e123366 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign80730_e123369))),)
    } else {
        (locals.var_pb2over__blk1886, locals.var_pb2over__blk1886_dn0, locals.var_pb2over__blk1886_dn2, locals.var_pb2over__blk1886_dn4, locals.var_pb2over__blk1886_dn5, locals.var_pb2over__blk1886_dn6, locals.var_pb2over__blk1886_dn7, locals.var_pb2over__blk1886_dn8, locals.var_pb2over__blk1886_dn9, locals.var_pb2over__blk1886_dn10, locals.var_pb2over__blk1886_dn13,)
    }
};
        locals.var_pb2over__blk1886 = assign80730_e123373;
        locals.var_pb2over__blk1886_dn0 = assign80730_e123373_d_n0;
        locals.var_pb2over__blk1886_dn2 = assign80730_e123373_d_n2;
        locals.var_pb2over__blk1886_dn4 = assign80730_e123373_d_n4;
        locals.var_pb2over__blk1886_dn5 = assign80730_e123373_d_n5;
        locals.var_pb2over__blk1886_dn6 = assign80730_e123373_d_n6;
        locals.var_pb2over__blk1886_dn7 = assign80730_e123373_d_n7;
        locals.var_pb2over__blk1886_dn8 = assign80730_e123373_d_n8;
        locals.var_pb2over__blk1886_dn9 = assign80730_e123373_d_n9;
        locals.var_pb2over__blk1886_dn10 = assign80730_e123373_d_n10;
        locals.var_pb2over__blk1886_dn13 = assign80730_e123373_d_n13;

    }

    pub(super) fn stamp_transient_block_281(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign80740_e123381, assign80740_e123381_d_n0, assign80740_e123381_d_n2, assign80740_e123381_d_n4, assign80740_e123381_d_n5, assign80740_e123381_d_n6, assign80740_e123381_d_n7, assign80740_e123381_d_n8, assign80740_e123381_d_n9, assign80740_e123381_d_n10, assign80740_e123381_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80740_e123377: f64 = (0.8 - locals.var_pb2over__blk1886);
        let assign80740_e123379: f64 = (assign80740_e123377 - 0.1);
        (assign80740_e123379, (-locals.var_pb2over__blk1886_dn0), (-locals.var_pb2over__blk1886_dn2), (-locals.var_pb2over__blk1886_dn4), (-locals.var_pb2over__blk1886_dn5), (-locals.var_pb2over__blk1886_dn6), (-locals.var_pb2over__blk1886_dn7), (-locals.var_pb2over__blk1886_dn8), (-locals.var_pb2over__blk1886_dn9), (-locals.var_pb2over__blk1886_dn10), (-locals.var_pb2over__blk1886_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign80740_e123381;
        locals.var_tmf1_dn0 = assign80740_e123381_d_n0;
        locals.var_tmf1_dn2 = assign80740_e123381_d_n2;
        locals.var_tmf1_dn4 = assign80740_e123381_d_n4;
        locals.var_tmf1_dn5 = assign80740_e123381_d_n5;
        locals.var_tmf1_dn6 = assign80740_e123381_d_n6;
        locals.var_tmf1_dn7 = assign80740_e123381_d_n7;
        locals.var_tmf1_dn8 = assign80740_e123381_d_n8;
        locals.var_tmf1_dn9 = assign80740_e123381_d_n9;
        locals.var_tmf1_dn10 = assign80740_e123381_d_n10;
        locals.var_tmf1_dn13 = assign80740_e123381_d_n13;

        let (assign80750_e123389, assign80750_e123389_d_n0, assign80750_e123389_d_n2, assign80750_e123389_d_n4, assign80750_e123389_d_n5, assign80750_e123389_d_n6, assign80750_e123389_d_n7, assign80750_e123389_d_n8, assign80750_e123389_d_n9, assign80750_e123389_d_n10, assign80750_e123389_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80750_e123385: f64 = (4.0 * 0.8);
        let assign80750_e123387: f64 = (assign80750_e123385 * 0.1);
        (assign80750_e123387, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80750_e123389;
        locals.var_tmf2_dn0 = assign80750_e123389_d_n0;
        locals.var_tmf2_dn2 = assign80750_e123389_d_n2;
        locals.var_tmf2_dn4 = assign80750_e123389_d_n4;
        locals.var_tmf2_dn5 = assign80750_e123389_d_n5;
        locals.var_tmf2_dn6 = assign80750_e123389_d_n6;
        locals.var_tmf2_dn7 = assign80750_e123389_d_n7;
        locals.var_tmf2_dn8 = assign80750_e123389_d_n8;
        locals.var_tmf2_dn9 = assign80750_e123389_d_n9;
        locals.var_tmf2_dn10 = assign80750_e123389_d_n10;
        locals.var_tmf2_dn13 = assign80750_e123389_d_n13;

        let (assign80760_e123399, assign80760_e123399_d_n0, assign80760_e123399_d_n2, assign80760_e123399_d_n4, assign80760_e123399_d_n5, assign80760_e123399_d_n6, assign80760_e123399_d_n7, assign80760_e123399_d_n8, assign80760_e123399_d_n9, assign80760_e123399_d_n10, assign80760_e123399_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign80760_e123397, assign80760_e123397_d_n0, assign80760_e123397_d_n2, assign80760_e123397_d_n4, assign80760_e123397_d_n5, assign80760_e123397_d_n6, assign80760_e123397_d_n7, assign80760_e123397_d_n8, assign80760_e123397_d_n9, assign80760_e123397_d_n10, assign80760_e123397_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign80760_e123396: f64 = (-locals.var_tmf2);
                (assign80760_e123396, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign80760_e123397, assign80760_e123397_d_n0, assign80760_e123397_d_n2, assign80760_e123397_d_n4, assign80760_e123397_d_n5, assign80760_e123397_d_n6, assign80760_e123397_d_n7, assign80760_e123397_d_n8, assign80760_e123397_d_n9, assign80760_e123397_d_n10, assign80760_e123397_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80760_e123399;
        locals.var_tmf2_dn0 = assign80760_e123399_d_n0;
        locals.var_tmf2_dn2 = assign80760_e123399_d_n2;
        locals.var_tmf2_dn4 = assign80760_e123399_d_n4;
        locals.var_tmf2_dn5 = assign80760_e123399_d_n5;
        locals.var_tmf2_dn6 = assign80760_e123399_d_n6;
        locals.var_tmf2_dn7 = assign80760_e123399_d_n7;
        locals.var_tmf2_dn8 = assign80760_e123399_d_n8;
        locals.var_tmf2_dn9 = assign80760_e123399_d_n9;
        locals.var_tmf2_dn10 = assign80760_e123399_d_n10;
        locals.var_tmf2_dn13 = assign80760_e123399_d_n13;

        let (assign80770_e123408, assign80770_e123408_d_n0, assign80770_e123408_d_n2, assign80770_e123408_d_n4, assign80770_e123408_d_n5, assign80770_e123408_d_n6, assign80770_e123408_d_n7, assign80770_e123408_d_n8, assign80770_e123408_d_n9, assign80770_e123408_d_n10, assign80770_e123408_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80770_e123403: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign80770_e123405: f64 = (assign80770_e123403 + locals.var_tmf2);
        let assign80770_e123406: f64 = (assign80770_e123405).sqrt();
        (assign80770_e123406, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign80770_e123406)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign80770_e123406)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80770_e123408;
        locals.var_tmf2_dn0 = assign80770_e123408_d_n0;
        locals.var_tmf2_dn2 = assign80770_e123408_d_n2;
        locals.var_tmf2_dn4 = assign80770_e123408_d_n4;
        locals.var_tmf2_dn5 = assign80770_e123408_d_n5;
        locals.var_tmf2_dn6 = assign80770_e123408_d_n6;
        locals.var_tmf2_dn7 = assign80770_e123408_d_n7;
        locals.var_tmf2_dn8 = assign80770_e123408_d_n8;
        locals.var_tmf2_dn9 = assign80770_e123408_d_n9;
        locals.var_tmf2_dn10 = assign80770_e123408_d_n10;
        locals.var_tmf2_dn13 = assign80770_e123408_d_n13;

        let (assign80780_e123418, assign80780_e123418_d_n0, assign80780_e123418_d_n2, assign80780_e123418_d_n4, assign80780_e123418_d_n5, assign80780_e123418_d_n6, assign80780_e123418_d_n7, assign80780_e123418_d_n8, assign80780_e123418_d_n9, assign80780_e123418_d_n10, assign80780_e123418_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80780_e123414: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign80780_e123415: f64 = (1.0 + assign80780_e123414);
        let assign80780_e123416: f64 = (0.5 * assign80780_e123415);
        (assign80780_e123416, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign80780_e123418;
        locals.var_t0_dn0 = assign80780_e123418_d_n0;
        locals.var_t0_dn2 = assign80780_e123418_d_n2;
        locals.var_t0_dn4 = assign80780_e123418_d_n4;
        locals.var_t0_dn5 = assign80780_e123418_d_n5;
        locals.var_t0_dn6 = assign80780_e123418_d_n6;
        locals.var_t0_dn7 = assign80780_e123418_d_n7;
        locals.var_t0_dn8 = assign80780_e123418_d_n8;
        locals.var_t0_dn9 = assign80780_e123418_d_n9;
        locals.var_t0_dn10 = assign80780_e123418_d_n10;
        locals.var_t0_dn13 = assign80780_e123418_d_n13;

        let (assign80790_e123428, assign80790_e123428_d_n0, assign80790_e123428_d_n2, assign80790_e123428_d_n4, assign80790_e123428_d_n5, assign80790_e123428_d_n6, assign80790_e123428_d_n7, assign80790_e123428_d_n8, assign80790_e123428_d_n9, assign80790_e123428_d_n10, assign80790_e123428_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80790_e123424: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign80790_e123425: f64 = (0.5 * assign80790_e123424);
        let assign80790_e123426: f64 = (0.8 - assign80790_e123425);
        (assign80790_e123426, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over__blk1887, locals.var_vbs_max_over__blk1887_dn0, locals.var_vbs_max_over__blk1887_dn2, locals.var_vbs_max_over__blk1887_dn4, locals.var_vbs_max_over__blk1887_dn5, locals.var_vbs_max_over__blk1887_dn6, locals.var_vbs_max_over__blk1887_dn7, locals.var_vbs_max_over__blk1887_dn8, locals.var_vbs_max_over__blk1887_dn9, locals.var_vbs_max_over__blk1887_dn10, locals.var_vbs_max_over__blk1887_dn13,)
    }
};
        locals.var_vbs_max_over__blk1887 = assign80790_e123428;
        locals.var_vbs_max_over__blk1887_dn0 = assign80790_e123428_d_n0;
        locals.var_vbs_max_over__blk1887_dn2 = assign80790_e123428_d_n2;
        locals.var_vbs_max_over__blk1887_dn4 = assign80790_e123428_d_n4;
        locals.var_vbs_max_over__blk1887_dn5 = assign80790_e123428_d_n5;
        locals.var_vbs_max_over__blk1887_dn6 = assign80790_e123428_d_n6;
        locals.var_vbs_max_over__blk1887_dn7 = assign80790_e123428_d_n7;
        locals.var_vbs_max_over__blk1887_dn8 = assign80790_e123428_d_n8;
        locals.var_vbs_max_over__blk1887_dn9 = assign80790_e123428_d_n9;
        locals.var_vbs_max_over__blk1887_dn10 = assign80790_e123428_d_n10;
        locals.var_vbs_max_over__blk1887_dn13 = assign80790_e123428_d_n13;

        let assign80800_e123432: f64 = (locals.var_vbs_max_over__blk1887 * 0.5);
        let assign80800_e123433: f64 = if locals.var_vbs_bnd_over__blk1888 > assign80800_e123432 { 1.0 } else { 0.0 };
        locals.var_guard1893 = assign80800_e123433;

        let (assign80810_e123441, assign80810_e123441_d_n0, assign80810_e123441_d_n2, assign80810_e123441_d_n4, assign80810_e123441_d_n5, assign80810_e123441_d_n6, assign80810_e123441_d_n7, assign80810_e123441_d_n8, assign80810_e123441_d_n9, assign80810_e123441_d_n10, assign80810_e123441_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1893 != 0.0)) {
        let assign80810_e123439: f64 = (0.5 * locals.var_vbs_max_over__blk1887);
        (assign80810_e123439, (0.5 * locals.var_vbs_max_over__blk1887_dn0), (0.5 * locals.var_vbs_max_over__blk1887_dn2), (0.5 * locals.var_vbs_max_over__blk1887_dn4), (0.5 * locals.var_vbs_max_over__blk1887_dn5), (0.5 * locals.var_vbs_max_over__blk1887_dn6), (0.5 * locals.var_vbs_max_over__blk1887_dn7), (0.5 * locals.var_vbs_max_over__blk1887_dn8), (0.5 * locals.var_vbs_max_over__blk1887_dn9), (0.5 * locals.var_vbs_max_over__blk1887_dn10), (0.5 * locals.var_vbs_max_over__blk1887_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1888, locals.var_vbs_bnd_over__blk1888_dn0, locals.var_vbs_bnd_over__blk1888_dn2, locals.var_vbs_bnd_over__blk1888_dn4, locals.var_vbs_bnd_over__blk1888_dn5, locals.var_vbs_bnd_over__blk1888_dn6, locals.var_vbs_bnd_over__blk1888_dn7, locals.var_vbs_bnd_over__blk1888_dn8, locals.var_vbs_bnd_over__blk1888_dn9, locals.var_vbs_bnd_over__blk1888_dn10, locals.var_vbs_bnd_over__blk1888_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1888 = assign80810_e123441;
        locals.var_vbs_bnd_over__blk1888_dn0 = assign80810_e123441_d_n0;
        locals.var_vbs_bnd_over__blk1888_dn2 = assign80810_e123441_d_n2;
        locals.var_vbs_bnd_over__blk1888_dn4 = assign80810_e123441_d_n4;
        locals.var_vbs_bnd_over__blk1888_dn5 = assign80810_e123441_d_n5;
        locals.var_vbs_bnd_over__blk1888_dn6 = assign80810_e123441_d_n6;
        locals.var_vbs_bnd_over__blk1888_dn7 = assign80810_e123441_d_n7;
        locals.var_vbs_bnd_over__blk1888_dn8 = assign80810_e123441_d_n8;
        locals.var_vbs_bnd_over__blk1888_dn9 = assign80810_e123441_d_n9;
        locals.var_vbs_bnd_over__blk1888_dn10 = assign80810_e123441_d_n10;
        locals.var_vbs_bnd_over__blk1888_dn13 = assign80810_e123441_d_n13;

        let assign80820_e123443: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1894 = assign80820_e123443;

        let (assign80830_e123449, assign80830_e123449_d_n0, assign80830_e123449_d_n2, assign80830_e123449_d_n4, assign80830_e123449_d_n5, assign80830_e123449_d_n6, assign80830_e123449_d_n7, assign80830_e123449_d_n8, assign80830_e123449_d_n9, assign80830_e123449_d_n10, assign80830_e123449_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1894 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1887, locals.var_vbs_max_over__blk1887_dn0, locals.var_vbs_max_over__blk1887_dn2, locals.var_vbs_max_over__blk1887_dn4, locals.var_vbs_max_over__blk1887_dn5, locals.var_vbs_max_over__blk1887_dn6, locals.var_vbs_max_over__blk1887_dn7, locals.var_vbs_max_over__blk1887_dn8, locals.var_vbs_max_over__blk1887_dn9, locals.var_vbs_max_over__blk1887_dn10, locals.var_vbs_max_over__blk1887_dn13,)
    }
};
        locals.var_vbs_max_over__blk1887 = assign80830_e123449;
        locals.var_vbs_max_over__blk1887_dn0 = assign80830_e123449_d_n0;
        locals.var_vbs_max_over__blk1887_dn2 = assign80830_e123449_d_n2;
        locals.var_vbs_max_over__blk1887_dn4 = assign80830_e123449_d_n4;
        locals.var_vbs_max_over__blk1887_dn5 = assign80830_e123449_d_n5;
        locals.var_vbs_max_over__blk1887_dn6 = assign80830_e123449_d_n6;
        locals.var_vbs_max_over__blk1887_dn7 = assign80830_e123449_d_n7;
        locals.var_vbs_max_over__blk1887_dn8 = assign80830_e123449_d_n8;
        locals.var_vbs_max_over__blk1887_dn9 = assign80830_e123449_d_n9;
        locals.var_vbs_max_over__blk1887_dn10 = assign80830_e123449_d_n10;
        locals.var_vbs_max_over__blk1887_dn13 = assign80830_e123449_d_n13;

        let assign80840_e123451: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1895 = assign80840_e123451;

        let (assign80850_e123457, assign80850_e123457_d_n0, assign80850_e123457_d_n2, assign80850_e123457_d_n4, assign80850_e123457_d_n5, assign80850_e123457_d_n6, assign80850_e123457_d_n7, assign80850_e123457_d_n8, assign80850_e123457_d_n9, assign80850_e123457_d_n10, assign80850_e123457_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1895 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1888, locals.var_vbs_bnd_over__blk1888_dn0, locals.var_vbs_bnd_over__blk1888_dn2, locals.var_vbs_bnd_over__blk1888_dn4, locals.var_vbs_bnd_over__blk1888_dn5, locals.var_vbs_bnd_over__blk1888_dn6, locals.var_vbs_bnd_over__blk1888_dn7, locals.var_vbs_bnd_over__blk1888_dn8, locals.var_vbs_bnd_over__blk1888_dn9, locals.var_vbs_bnd_over__blk1888_dn10, locals.var_vbs_bnd_over__blk1888_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1888 = assign80850_e123457;
        locals.var_vbs_bnd_over__blk1888_dn0 = assign80850_e123457_d_n0;
        locals.var_vbs_bnd_over__blk1888_dn2 = assign80850_e123457_d_n2;
        locals.var_vbs_bnd_over__blk1888_dn4 = assign80850_e123457_d_n4;
        locals.var_vbs_bnd_over__blk1888_dn5 = assign80850_e123457_d_n5;
        locals.var_vbs_bnd_over__blk1888_dn6 = assign80850_e123457_d_n6;
        locals.var_vbs_bnd_over__blk1888_dn7 = assign80850_e123457_d_n7;
        locals.var_vbs_bnd_over__blk1888_dn8 = assign80850_e123457_d_n8;
        locals.var_vbs_bnd_over__blk1888_dn9 = assign80850_e123457_d_n9;
        locals.var_vbs_bnd_over__blk1888_dn10 = assign80850_e123457_d_n10;
        locals.var_vbs_bnd_over__blk1888_dn13 = assign80850_e123457_d_n13;

        let assign80860_e123459: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1896 = assign80860_e123459;

        let (assign80870_e123470, assign80870_e123470_d_n0, assign80870_e123470_d_n2, assign80870_e123470_d_n4, assign80870_e123470_d_n5, assign80870_e123470_d_n6, assign80870_e123470_d_n7, assign80870_e123470_d_n8, assign80870_e123470_d_n9, assign80870_e123470_d_n10, assign80870_e123470_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1895 == 0.0)) && (locals.var_guard1896 != 0.0)) {
        let assign80870_e123468: f64 = (0.5 * locals.var_vbs_max_over__blk1887);
        (assign80870_e123468, (0.5 * locals.var_vbs_max_over__blk1887_dn0), (0.5 * locals.var_vbs_max_over__blk1887_dn2), (0.5 * locals.var_vbs_max_over__blk1887_dn4), (0.5 * locals.var_vbs_max_over__blk1887_dn5), (0.5 * locals.var_vbs_max_over__blk1887_dn6), (0.5 * locals.var_vbs_max_over__blk1887_dn7), (0.5 * locals.var_vbs_max_over__blk1887_dn8), (0.5 * locals.var_vbs_max_over__blk1887_dn9), (0.5 * locals.var_vbs_max_over__blk1887_dn10), (0.5 * locals.var_vbs_max_over__blk1887_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1888, locals.var_vbs_bnd_over__blk1888_dn0, locals.var_vbs_bnd_over__blk1888_dn2, locals.var_vbs_bnd_over__blk1888_dn4, locals.var_vbs_bnd_over__blk1888_dn5, locals.var_vbs_bnd_over__blk1888_dn6, locals.var_vbs_bnd_over__blk1888_dn7, locals.var_vbs_bnd_over__blk1888_dn8, locals.var_vbs_bnd_over__blk1888_dn9, locals.var_vbs_bnd_over__blk1888_dn10, locals.var_vbs_bnd_over__blk1888_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1888 = assign80870_e123470;
        locals.var_vbs_bnd_over__blk1888_dn0 = assign80870_e123470_d_n0;
        locals.var_vbs_bnd_over__blk1888_dn2 = assign80870_e123470_d_n2;
        locals.var_vbs_bnd_over__blk1888_dn4 = assign80870_e123470_d_n4;
        locals.var_vbs_bnd_over__blk1888_dn5 = assign80870_e123470_d_n5;
        locals.var_vbs_bnd_over__blk1888_dn6 = assign80870_e123470_d_n6;
        locals.var_vbs_bnd_over__blk1888_dn7 = assign80870_e123470_d_n7;
        locals.var_vbs_bnd_over__blk1888_dn8 = assign80870_e123470_d_n8;
        locals.var_vbs_bnd_over__blk1888_dn9 = assign80870_e123470_d_n9;
        locals.var_vbs_bnd_over__blk1888_dn10 = assign80870_e123470_d_n10;
        locals.var_vbs_bnd_over__blk1888_dn13 = assign80870_e123470_d_n13;

        let assign80880_e123474: f64 = (locals.var_vbs_max_over__blk1887 * 0.5);
        let assign80880_e123475: f64 = if locals.var_vbs_bnd_over__blk1888 > assign80880_e123474 { 1.0 } else { 0.0 };
        locals.var_guard1897 = assign80880_e123475;

        let (assign80890_e123483, assign80890_e123483_d_n0, assign80890_e123483_d_n2, assign80890_e123483_d_n4, assign80890_e123483_d_n5, assign80890_e123483_d_n6, assign80890_e123483_d_n7, assign80890_e123483_d_n8, assign80890_e123483_d_n9, assign80890_e123483_d_n10, assign80890_e123483_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1897 != 0.0)) {
        let assign80890_e123481: f64 = (0.5 * locals.var_vbs_max_over__blk1887);
        (assign80890_e123481, (0.5 * locals.var_vbs_max_over__blk1887_dn0), (0.5 * locals.var_vbs_max_over__blk1887_dn2), (0.5 * locals.var_vbs_max_over__blk1887_dn4), (0.5 * locals.var_vbs_max_over__blk1887_dn5), (0.5 * locals.var_vbs_max_over__blk1887_dn6), (0.5 * locals.var_vbs_max_over__blk1887_dn7), (0.5 * locals.var_vbs_max_over__blk1887_dn8), (0.5 * locals.var_vbs_max_over__blk1887_dn9), (0.5 * locals.var_vbs_max_over__blk1887_dn10), (0.5 * locals.var_vbs_max_over__blk1887_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk1888, locals.var_vbs_bnd_over__blk1888_dn0, locals.var_vbs_bnd_over__blk1888_dn2, locals.var_vbs_bnd_over__blk1888_dn4, locals.var_vbs_bnd_over__blk1888_dn5, locals.var_vbs_bnd_over__blk1888_dn6, locals.var_vbs_bnd_over__blk1888_dn7, locals.var_vbs_bnd_over__blk1888_dn8, locals.var_vbs_bnd_over__blk1888_dn9, locals.var_vbs_bnd_over__blk1888_dn10, locals.var_vbs_bnd_over__blk1888_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk1888 = assign80890_e123483;
        locals.var_vbs_bnd_over__blk1888_dn0 = assign80890_e123483_d_n0;
        locals.var_vbs_bnd_over__blk1888_dn2 = assign80890_e123483_d_n2;
        locals.var_vbs_bnd_over__blk1888_dn4 = assign80890_e123483_d_n4;
        locals.var_vbs_bnd_over__blk1888_dn5 = assign80890_e123483_d_n5;
        locals.var_vbs_bnd_over__blk1888_dn6 = assign80890_e123483_d_n6;
        locals.var_vbs_bnd_over__blk1888_dn7 = assign80890_e123483_d_n7;
        locals.var_vbs_bnd_over__blk1888_dn8 = assign80890_e123483_d_n8;
        locals.var_vbs_bnd_over__blk1888_dn9 = assign80890_e123483_d_n9;
        locals.var_vbs_bnd_over__blk1888_dn10 = assign80890_e123483_d_n10;
        locals.var_vbs_bnd_over__blk1888_dn13 = assign80890_e123483_d_n13;

        let assign80900_e123486: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1898 = assign80900_e123486;

        let (assign80910_e123493, assign80910_e123493_d_n0, assign80910_e123493_d_n2, assign80910_e123493_d_n4, assign80910_e123493_d_n5, assign80910_e123493_d_n6, assign80910_e123493_d_n7, assign80910_e123493_d_n8, assign80910_e123493_d_n9, assign80910_e123493_d_n10, assign80910_e123493_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) {
        let assign80910_e123491: f64 = (-locals.var_vxbgmt);
        (assign80910_e123491, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign80910_e123493;
        locals.var_t0_dn0 = assign80910_e123493_d_n0;
        locals.var_t0_dn2 = assign80910_e123493_d_n2;
        locals.var_t0_dn4 = assign80910_e123493_d_n4;
        locals.var_t0_dn5 = assign80910_e123493_d_n5;
        locals.var_t0_dn6 = assign80910_e123493_d_n6;
        locals.var_t0_dn7 = assign80910_e123493_d_n7;
        locals.var_t0_dn8 = assign80910_e123493_d_n8;
        locals.var_t0_dn9 = assign80910_e123493_d_n9;
        locals.var_t0_dn10 = assign80910_e123493_d_n10;
        locals.var_t0_dn13 = assign80910_e123493_d_n13;

        let assign80920_e123496: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1888 { 1.0 } else { 0.0 };
        locals.var_guard1899 = assign80920_e123496;

        let (assign80930_e123506, assign80930_e123506_d_n0, assign80930_e123506_d_n2, assign80930_e123506_d_n4, assign80930_e123506_d_n5, assign80930_e123506_d_n6, assign80930_e123506_d_n7, assign80930_e123506_d_n8, assign80930_e123506_d_n9, assign80930_e123506_d_n10, assign80930_e123506_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80930_e123504: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1888);
        (assign80930_e123504, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1888_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1888_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1888_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1888_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1888_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1888_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1888_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1888_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1888_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over__blk1888_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign80930_e123506;
        locals.var_t1_dn0 = assign80930_e123506_d_n0;
        locals.var_t1_dn2 = assign80930_e123506_d_n2;
        locals.var_t1_dn4 = assign80930_e123506_d_n4;
        locals.var_t1_dn5 = assign80930_e123506_d_n5;
        locals.var_t1_dn6 = assign80930_e123506_d_n6;
        locals.var_t1_dn7 = assign80930_e123506_d_n7;
        locals.var_t1_dn8 = assign80930_e123506_d_n8;
        locals.var_t1_dn9 = assign80930_e123506_d_n9;
        locals.var_t1_dn10 = assign80930_e123506_d_n10;
        locals.var_t1_dn13 = assign80930_e123506_d_n13;

        let (assign80940_e123516, assign80940_e123516_d_n0, assign80940_e123516_d_n2, assign80940_e123516_d_n4, assign80940_e123516_d_n5, assign80940_e123516_d_n6, assign80940_e123516_d_n7, assign80940_e123516_d_n8, assign80940_e123516_d_n9, assign80940_e123516_d_n10, assign80940_e123516_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80940_e123514: f64 = (locals.var_vbs_max_over__blk1887 - locals.var_vbs_bnd_over__blk1888);
        (assign80940_e123514, (locals.var_vbs_max_over__blk1887_dn0 - locals.var_vbs_bnd_over__blk1888_dn0), (locals.var_vbs_max_over__blk1887_dn2 - locals.var_vbs_bnd_over__blk1888_dn2), (locals.var_vbs_max_over__blk1887_dn4 - locals.var_vbs_bnd_over__blk1888_dn4), (locals.var_vbs_max_over__blk1887_dn5 - locals.var_vbs_bnd_over__blk1888_dn5), (locals.var_vbs_max_over__blk1887_dn6 - locals.var_vbs_bnd_over__blk1888_dn6), (locals.var_vbs_max_over__blk1887_dn7 - locals.var_vbs_bnd_over__blk1888_dn7), (locals.var_vbs_max_over__blk1887_dn8 - locals.var_vbs_bnd_over__blk1888_dn8), (locals.var_vbs_max_over__blk1887_dn9 - locals.var_vbs_bnd_over__blk1888_dn9), (locals.var_vbs_max_over__blk1887_dn10 - locals.var_vbs_bnd_over__blk1888_dn10), (locals.var_vbs_max_over__blk1887_dn13 - locals.var_vbs_bnd_over__blk1888_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign80940_e123516;
        locals.var_t2_dn0 = assign80940_e123516_d_n0;
        locals.var_t2_dn2 = assign80940_e123516_d_n2;
        locals.var_t2_dn4 = assign80940_e123516_d_n4;
        locals.var_t2_dn5 = assign80940_e123516_d_n5;
        locals.var_t2_dn6 = assign80940_e123516_d_n6;
        locals.var_t2_dn7 = assign80940_e123516_d_n7;
        locals.var_t2_dn8 = assign80940_e123516_d_n8;
        locals.var_t2_dn9 = assign80940_e123516_d_n9;
        locals.var_t2_dn10 = assign80940_e123516_d_n10;
        locals.var_t2_dn13 = assign80940_e123516_d_n13;

        let (assign80950_e123526, assign80950_e123526_d_n0, assign80950_e123526_d_n2, assign80950_e123526_d_n4, assign80950_e123526_d_n5, assign80950_e123526_d_n6, assign80950_e123526_d_n7, assign80950_e123526_d_n8, assign80950_e123526_d_n9, assign80950_e123526_d_n10, assign80950_e123526_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80950_e123524: f64 = (locals.var_t1 / locals.var_t2);
        (assign80950_e123524, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign80950_e123526;
        locals.var_tmf1_dn0 = assign80950_e123526_d_n0;
        locals.var_tmf1_dn2 = assign80950_e123526_d_n2;
        locals.var_tmf1_dn4 = assign80950_e123526_d_n4;
        locals.var_tmf1_dn5 = assign80950_e123526_d_n5;
        locals.var_tmf1_dn6 = assign80950_e123526_d_n6;
        locals.var_tmf1_dn7 = assign80950_e123526_d_n7;
        locals.var_tmf1_dn8 = assign80950_e123526_d_n8;
        locals.var_tmf1_dn9 = assign80950_e123526_d_n9;
        locals.var_tmf1_dn10 = assign80950_e123526_d_n10;
        locals.var_tmf1_dn13 = assign80950_e123526_d_n13;

        let (assign80960_e123536, assign80960_e123536_d_n0, assign80960_e123536_d_n2, assign80960_e123536_d_n4, assign80960_e123536_d_n5, assign80960_e123536_d_n6, assign80960_e123536_d_n7, assign80960_e123536_d_n8, assign80960_e123536_d_n9, assign80960_e123536_d_n10, assign80960_e123536_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80960_e123534: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign80960_e123534, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign80960_e123536;
        locals.var_tmf2_dn0 = assign80960_e123536_d_n0;
        locals.var_tmf2_dn2 = assign80960_e123536_d_n2;
        locals.var_tmf2_dn4 = assign80960_e123536_d_n4;
        locals.var_tmf2_dn5 = assign80960_e123536_d_n5;
        locals.var_tmf2_dn6 = assign80960_e123536_d_n6;
        locals.var_tmf2_dn7 = assign80960_e123536_d_n7;
        locals.var_tmf2_dn8 = assign80960_e123536_d_n8;
        locals.var_tmf2_dn9 = assign80960_e123536_d_n9;
        locals.var_tmf2_dn10 = assign80960_e123536_d_n10;
        locals.var_tmf2_dn13 = assign80960_e123536_d_n13;

        let (assign80970_e123546, assign80970_e123546_d_n0, assign80970_e123546_d_n2, assign80970_e123546_d_n4, assign80970_e123546_d_n5, assign80970_e123546_d_n6, assign80970_e123546_d_n7, assign80970_e123546_d_n8, assign80970_e123546_d_n9, assign80970_e123546_d_n10, assign80970_e123546_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80970_e123544: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign80970_e123544, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign80970_e123546;
        locals.var_tmf3_dn0 = assign80970_e123546_d_n0;
        locals.var_tmf3_dn2 = assign80970_e123546_d_n2;
        locals.var_tmf3_dn4 = assign80970_e123546_d_n4;
        locals.var_tmf3_dn5 = assign80970_e123546_d_n5;
        locals.var_tmf3_dn6 = assign80970_e123546_d_n6;
        locals.var_tmf3_dn7 = assign80970_e123546_d_n7;
        locals.var_tmf3_dn8 = assign80970_e123546_d_n8;
        locals.var_tmf3_dn9 = assign80970_e123546_d_n9;
        locals.var_tmf3_dn10 = assign80970_e123546_d_n10;
        locals.var_tmf3_dn13 = assign80970_e123546_d_n13;

        let (assign80980_e123556, assign80980_e123556_d_n0, assign80980_e123556_d_n2, assign80980_e123556_d_n4, assign80980_e123556_d_n5, assign80980_e123556_d_n6, assign80980_e123556_d_n7, assign80980_e123556_d_n8, assign80980_e123556_d_n9, assign80980_e123556_d_n10, assign80980_e123556_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80980_e123554: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign80980_e123554, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign80980_e123556;
        locals.var_tmf4_dn0 = assign80980_e123556_d_n0;
        locals.var_tmf4_dn2 = assign80980_e123556_d_n2;
        locals.var_tmf4_dn4 = assign80980_e123556_d_n4;
        locals.var_tmf4_dn5 = assign80980_e123556_d_n5;
        locals.var_tmf4_dn6 = assign80980_e123556_d_n6;
        locals.var_tmf4_dn7 = assign80980_e123556_d_n7;
        locals.var_tmf4_dn8 = assign80980_e123556_d_n8;
        locals.var_tmf4_dn9 = assign80980_e123556_d_n9;
        locals.var_tmf4_dn10 = assign80980_e123556_d_n10;
        locals.var_tmf4_dn13 = assign80980_e123556_d_n13;

        let (assign80990_e123574, assign80990_e123574_d_n0, assign80990_e123574_d_n2, assign80990_e123574_d_n4, assign80990_e123574_d_n5, assign80990_e123574_d_n6, assign80990_e123574_d_n7, assign80990_e123574_d_n8, assign80990_e123574_d_n9, assign80990_e123574_d_n10, assign80990_e123574_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign80990_e123565: f64 = (1.0 + locals.var_tmf1);
        let assign80990_e123567: f64 = (assign80990_e123565 + locals.var_tmf2);
        let assign80990_e123569: f64 = (assign80990_e123567 + locals.var_tmf3);
        let assign80990_e123571: f64 = (assign80990_e123569 + locals.var_tmf4);
        let assign80990_e123572: f64 = (1.0 / assign80990_e123571);
        (assign80990_e123572, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign80990_e123571 * assign80990_e123571))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign80990_e123571 * assign80990_e123571))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign80990_e123574;
        locals.var_tmf0_dn0 = assign80990_e123574_d_n0;
        locals.var_tmf0_dn2 = assign80990_e123574_d_n2;
        locals.var_tmf0_dn4 = assign80990_e123574_d_n4;
        locals.var_tmf0_dn5 = assign80990_e123574_d_n5;
        locals.var_tmf0_dn6 = assign80990_e123574_d_n6;
        locals.var_tmf0_dn7 = assign80990_e123574_d_n7;
        locals.var_tmf0_dn8 = assign80990_e123574_d_n8;
        locals.var_tmf0_dn9 = assign80990_e123574_d_n9;
        locals.var_tmf0_dn10 = assign80990_e123574_d_n10;
        locals.var_tmf0_dn13 = assign80990_e123574_d_n13;

        let (assign81000_e123599, assign81000_e123599_d_n0, assign81000_e123599_d_n2, assign81000_e123599_d_n4, assign81000_e123599_d_n5, assign81000_e123599_d_n6, assign81000_e123599_d_n7, assign81000_e123599_d_n8, assign81000_e123599_d_n9, assign81000_e123599_d_n10, assign81000_e123599_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81000_e123583: f64 = (2.0 * locals.var_tmf1);
        let assign81000_e123584: f64 = (1.0 + assign81000_e123583);
        let assign81000_e123587: f64 = (3.0 * locals.var_tmf2);
        let assign81000_e123588: f64 = (assign81000_e123584 + assign81000_e123587);
        let assign81000_e123591: f64 = (4.0 * locals.var_tmf3);
        let assign81000_e123592: f64 = (assign81000_e123588 + assign81000_e123591);
        let assign81000_e123593: f64 = (-assign81000_e123592);
        let assign81000_e123595: f64 = (assign81000_e123593 * locals.var_tmf0);
        let assign81000_e123597: f64 = (assign81000_e123595 * locals.var_tmf0);
        (assign81000_e123597, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign81000_e123593 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign81000_e123595 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign81000_e123599;
        locals.var_t11_dn0 = assign81000_e123599_d_n0;
        locals.var_t11_dn2 = assign81000_e123599_d_n2;
        locals.var_t11_dn4 = assign81000_e123599_d_n4;
        locals.var_t11_dn5 = assign81000_e123599_d_n5;
        locals.var_t11_dn6 = assign81000_e123599_d_n6;
        locals.var_t11_dn7 = assign81000_e123599_d_n7;
        locals.var_t11_dn8 = assign81000_e123599_d_n8;
        locals.var_t11_dn9 = assign81000_e123599_d_n9;
        locals.var_t11_dn10 = assign81000_e123599_d_n10;
        locals.var_t11_dn13 = assign81000_e123599_d_n13;

        let (assign81010_e123611, assign81010_e123611_d_n0, assign81010_e123611_d_n2, assign81010_e123611_d_n4, assign81010_e123611_d_n5, assign81010_e123611_d_n6, assign81010_e123611_d_n7, assign81010_e123611_d_n8, assign81010_e123611_d_n9, assign81010_e123611_d_n10, assign81010_e123611_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81010_e123608: f64 = (1.0 - locals.var_tmf0);
        let assign81010_e123609: f64 = (locals.var_t2 * assign81010_e123608);
        (assign81010_e123609, ((locals.var_t2_dn0 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign81010_e123608) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81010_e123611;
        locals.var_ty_dn0 = assign81010_e123611_d_n0;
        locals.var_ty_dn2 = assign81010_e123611_d_n2;
        locals.var_ty_dn4 = assign81010_e123611_d_n4;
        locals.var_ty_dn5 = assign81010_e123611_d_n5;
        locals.var_ty_dn6 = assign81010_e123611_d_n6;
        locals.var_ty_dn7 = assign81010_e123611_d_n7;
        locals.var_ty_dn8 = assign81010_e123611_d_n8;
        locals.var_ty_dn9 = assign81010_e123611_d_n9;
        locals.var_ty_dn10 = assign81010_e123611_d_n10;
        locals.var_ty_dn13 = assign81010_e123611_d_n13;

        let (assign81020_e123625, assign81020_e123625_d_n0, assign81020_e123625_d_n2, assign81020_e123625_d_n4, assign81020_e123625_d_n5, assign81020_e123625_d_n6, assign81020_e123625_d_n7, assign81020_e123625_d_n8, assign81020_e123625_d_n9, assign81020_e123625_d_n10, assign81020_e123625_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81020_e123619: f64 = (1.0 - locals.var_tmf0);
        let assign81020_e123622: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign81020_e123623: f64 = (assign81020_e123619 + assign81020_e123622);
        (assign81020_e123623, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81020_e123625;
        locals.var_t0_dn0 = assign81020_e123625_d_n0;
        locals.var_t0_dn2 = assign81020_e123625_d_n2;
        locals.var_t0_dn4 = assign81020_e123625_d_n4;
        locals.var_t0_dn5 = assign81020_e123625_d_n5;
        locals.var_t0_dn6 = assign81020_e123625_d_n6;
        locals.var_t0_dn7 = assign81020_e123625_d_n7;
        locals.var_t0_dn8 = assign81020_e123625_d_n8;
        locals.var_t0_dn9 = assign81020_e123625_d_n9;
        locals.var_t0_dn10 = assign81020_e123625_d_n10;
        locals.var_t0_dn13 = assign81020_e123625_d_n13;

        let (assign81030_e123634, assign81030_e123634_d_n0, assign81030_e123634_d_n2, assign81030_e123634_d_n4, assign81030_e123634_d_n5, assign81030_e123634_d_n6, assign81030_e123634_d_n7, assign81030_e123634_d_n8, assign81030_e123634_d_n9, assign81030_e123634_d_n10, assign81030_e123634_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81030_e123632: f64 = (-locals.var_t11);
        (assign81030_e123632, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign81030_e123634;
        locals.var_t11_dn0 = assign81030_e123634_d_n0;
        locals.var_t11_dn2 = assign81030_e123634_d_n2;
        locals.var_t11_dn4 = assign81030_e123634_d_n4;
        locals.var_t11_dn5 = assign81030_e123634_d_n5;
        locals.var_t11_dn6 = assign81030_e123634_d_n6;
        locals.var_t11_dn7 = assign81030_e123634_d_n7;
        locals.var_t11_dn8 = assign81030_e123634_d_n8;
        locals.var_t11_dn9 = assign81030_e123634_d_n9;
        locals.var_t11_dn10 = assign81030_e123634_d_n10;
        locals.var_t11_dn13 = assign81030_e123634_d_n13;

    }

    pub(super) fn stamp_transient_block_282(
        locals: &mut StampLocals,
    ) {
        let (assign81040_e123644, assign81040_e123644_d_n0, assign81040_e123644_d_n2, assign81040_e123644_d_n4, assign81040_e123644_d_n5, assign81040_e123644_d_n6, assign81040_e123644_d_n7, assign81040_e123644_d_n8, assign81040_e123644_d_n9, assign81040_e123644_d_n10, assign81040_e123644_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 != 0.0)) {
        let assign81040_e123642: f64 = (locals.var_vbs_bnd_over__blk1888 + locals.var_ty);
        (assign81040_e123642, (locals.var_vbs_bnd_over__blk1888_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1888_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1888_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1888_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1888_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1888_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1888_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1888_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1888_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1888_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign81040_e123644;
        locals.var_t10_dn0 = assign81040_e123644_d_n0;
        locals.var_t10_dn2 = assign81040_e123644_d_n2;
        locals.var_t10_dn4 = assign81040_e123644_d_n4;
        locals.var_t10_dn5 = assign81040_e123644_d_n5;
        locals.var_t10_dn6 = assign81040_e123644_d_n6;
        locals.var_t10_dn7 = assign81040_e123644_d_n7;
        locals.var_t10_dn8 = assign81040_e123644_d_n8;
        locals.var_t10_dn9 = assign81040_e123644_d_n9;
        locals.var_t10_dn10 = assign81040_e123644_d_n10;
        locals.var_t10_dn13 = assign81040_e123644_d_n13;

        let (assign81050_e123653, assign81050_e123653_d_n0, assign81050_e123653_d_n2, assign81050_e123653_d_n4, assign81050_e123653_d_n5, assign81050_e123653_d_n6, assign81050_e123653_d_n7, assign81050_e123653_d_n8, assign81050_e123653_d_n9, assign81050_e123653_d_n10, assign81050_e123653_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) && (locals.var_guard1899 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign81050_e123653;
        locals.var_t10_dn0 = assign81050_e123653_d_n0;
        locals.var_t10_dn2 = assign81050_e123653_d_n2;
        locals.var_t10_dn4 = assign81050_e123653_d_n4;
        locals.var_t10_dn5 = assign81050_e123653_d_n5;
        locals.var_t10_dn6 = assign81050_e123653_d_n6;
        locals.var_t10_dn7 = assign81050_e123653_d_n7;
        locals.var_t10_dn8 = assign81050_e123653_d_n8;
        locals.var_t10_dn9 = assign81050_e123653_d_n9;
        locals.var_t10_dn10 = assign81050_e123653_d_n10;
        locals.var_t10_dn13 = assign81050_e123653_d_n13;

        let (assign81060_e123660, assign81060_e123660_d_n0, assign81060_e123660_d_n2, assign81060_e123660_d_n4, assign81060_e123660_d_n5, assign81060_e123660_d_n6, assign81060_e123660_d_n7, assign81060_e123660_d_n8, assign81060_e123660_d_n9, assign81060_e123660_d_n10, assign81060_e123660_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 != 0.0)) {
        let assign81060_e123658: f64 = (-locals.var_t10);
        (assign81060_e123658, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81060_e123660;
        locals.var_vxbgmtcl_dn0 = assign81060_e123660_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81060_e123660_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81060_e123660_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81060_e123660_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81060_e123660_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81060_e123660_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81060_e123660_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81060_e123660_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81060_e123660_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81060_e123660_d_n13;

        let (assign81070_e123667, assign81070_e123667_d_n0, assign81070_e123667_d_n2, assign81070_e123667_d_n4, assign81070_e123667_d_n5, assign81070_e123667_d_n6, assign81070_e123667_d_n7, assign81070_e123667_d_n8, assign81070_e123667_d_n9, assign81070_e123667_d_n10, assign81070_e123667_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1898 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81070_e123667;
        locals.var_vxbgmtcl_dn0 = assign81070_e123667_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81070_e123667_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81070_e123667_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81070_e123667_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81070_e123667_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81070_e123667_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81070_e123667_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81070_e123667_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81070_e123667_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81070_e123667_d_n13;

        let (assign81080_e123673, assign81080_e123673_d_n0, assign81080_e123673_d_n2, assign81080_e123673_d_n4, assign81080_e123673_d_n5, assign81080_e123673_d_n6, assign81080_e123673_d_n7, assign81080_e123673_d_n8, assign81080_e123673_d_n9, assign81080_e123673_d_n10, assign81080_e123673_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81080_e123671: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign81080_e123671, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign81080_e123673;
        locals.var_fac1_dn0 = assign81080_e123673_d_n0;
        locals.var_fac1_dn2 = assign81080_e123673_d_n2;
        locals.var_fac1_dn4 = assign81080_e123673_d_n4;
        locals.var_fac1_dn5 = assign81080_e123673_d_n5;
        locals.var_fac1_dn6 = assign81080_e123673_d_n6;
        locals.var_fac1_dn7 = assign81080_e123673_d_n7;
        locals.var_fac1_dn8 = assign81080_e123673_d_n8;
        locals.var_fac1_dn9 = assign81080_e123673_d_n9;
        locals.var_fac1_dn10 = assign81080_e123673_d_n10;
        locals.var_fac1_dn13 = assign81080_e123673_d_n13;

        let (assign81090_e123679, assign81090_e123679_d_n0, assign81090_e123679_d_n2, assign81090_e123679_d_n4, assign81090_e123679_d_n5, assign81090_e123679_d_n6, assign81090_e123679_d_n7, assign81090_e123679_d_n8, assign81090_e123679_d_n9, assign81090_e123679_d_n10, assign81090_e123679_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81090_e123677: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign81090_e123677, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign81090_e123679;
        locals.var_fac1p2_dn0 = assign81090_e123679_d_n0;
        locals.var_fac1p2_dn2 = assign81090_e123679_d_n2;
        locals.var_fac1p2_dn4 = assign81090_e123679_d_n4;
        locals.var_fac1p2_dn5 = assign81090_e123679_d_n5;
        locals.var_fac1p2_dn6 = assign81090_e123679_d_n6;
        locals.var_fac1p2_dn7 = assign81090_e123679_d_n7;
        locals.var_fac1p2_dn8 = assign81090_e123679_d_n8;
        locals.var_fac1p2_dn9 = assign81090_e123679_d_n9;
        locals.var_fac1p2_dn10 = assign81090_e123679_d_n10;
        locals.var_fac1p2_dn13 = assign81090_e123679_d_n13;

        let (assign81100_e123686, assign81100_e123686_d_n2, assign81100_e123686_d_n6, assign81100_e123686_d_n7, assign81100_e123686_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81100_e123682: f64 = (-locals.var_vgbgmt);
        let assign81100_e123684: f64 = (assign81100_e123682 + locals.var_uc_vfbover);
        (assign81100_e123684, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign81100_e123686;
        locals.var_vgpld_dn2 = assign81100_e123686_d_n2;
        locals.var_vgpld_dn6 = assign81100_e123686_d_n6;
        locals.var_vgpld_dn7 = assign81100_e123686_d_n7;
        locals.var_vgpld_dn8 = assign81100_e123686_d_n8;

        let (assign81110_e123695,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81110_e123689: f64 = (-locals.var_vxbgmtcl);
        let assign81110_e123692: f64 = (10.0 * 2.220446049250313e-16);
        let assign81110_e123693: f64 = (assign81110_e123689 + assign81110_e123692);
        (assign81110_e123693,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign81110_e123695;

        let (assign81120_e123699, assign81120_e123699_d_n0, assign81120_e123699_d_n2, assign81120_e123699_d_n4, assign81120_e123699_d_n5, assign81120_e123699_d_n6, assign81120_e123699_d_n7, assign81120_e123699_d_n8, assign81120_e123699_d_n9, assign81120_e123699_d_n10, assign81120_e123699_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign81120_e123699;
        locals.var_q_dep_ld__blk1882_dn0 = assign81120_e123699_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign81120_e123699_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign81120_e123699_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign81120_e123699_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign81120_e123699_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign81120_e123699_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign81120_e123699_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign81120_e123699_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign81120_e123699_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign81120_e123699_d_n13;

        let (assign81130_e123705,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81130_e123703: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign81130_e123703,)
    } else {
        (locals.var_q_nsubld__blk1883,)
    }
};
        locals.var_q_nsubld__blk1883 = assign81130_e123705;

        let (assign81140_e123711, assign81140_e123711_d_n0, assign81140_e123711_d_n2, assign81140_e123711_d_n4, assign81140_e123711_d_n5, assign81140_e123711_d_n6, assign81140_e123711_d_n7, assign81140_e123711_d_n8, assign81140_e123711_d_n9, assign81140_e123711_d_n10, assign81140_e123711_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81140_e123709: f64 = (locals.var_nin / locals.var_nover_func);
        (assign81140_e123709, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81140_e123711;
        locals.var_t0_dn0 = assign81140_e123711_d_n0;
        locals.var_t0_dn2 = assign81140_e123711_d_n2;
        locals.var_t0_dn4 = assign81140_e123711_d_n4;
        locals.var_t0_dn5 = assign81140_e123711_d_n5;
        locals.var_t0_dn6 = assign81140_e123711_d_n6;
        locals.var_t0_dn7 = assign81140_e123711_d_n7;
        locals.var_t0_dn8 = assign81140_e123711_d_n8;
        locals.var_t0_dn9 = assign81140_e123711_d_n9;
        locals.var_t0_dn10 = assign81140_e123711_d_n10;
        locals.var_t0_dn13 = assign81140_e123711_d_n13;

        let (assign81150_e123717, assign81150_e123717_d_n0, assign81150_e123717_d_n2, assign81150_e123717_d_n4, assign81150_e123717_d_n5, assign81150_e123717_d_n6, assign81150_e123717_d_n7, assign81150_e123717_d_n8, assign81150_e123717_d_n9, assign81150_e123717_d_n10, assign81150_e123717_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81150_e123715: f64 = (locals.var_t0 * locals.var_t0);
        (assign81150_e123715, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign81150_e123717;
        locals.var_cnst1over_dn0 = assign81150_e123717_d_n0;
        locals.var_cnst1over_dn2 = assign81150_e123717_d_n2;
        locals.var_cnst1over_dn4 = assign81150_e123717_d_n4;
        locals.var_cnst1over_dn5 = assign81150_e123717_d_n5;
        locals.var_cnst1over_dn6 = assign81150_e123717_d_n6;
        locals.var_cnst1over_dn7 = assign81150_e123717_d_n7;
        locals.var_cnst1over_dn8 = assign81150_e123717_d_n8;
        locals.var_cnst1over_dn9 = assign81150_e123717_d_n9;
        locals.var_cnst1over_dn10 = assign81150_e123717_d_n10;
        locals.var_cnst1over_dn13 = assign81150_e123717_d_n13;

        let assign81160_e123720: f64 = (-locals.var_vxbgmtcl);
        let assign81160_e123721: f64 = (locals.var_beta * assign81160_e123720);
        let assign81160_e123723: f64 = if assign81160_e123721 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1900 = assign81160_e123723;

        let (assign81170_e123738, assign81170_e123738_d_n0, assign81170_e123738_d_n2, assign81170_e123738_d_n4, assign81170_e123738_d_n5, assign81170_e123738_d_n6, assign81170_e123738_d_n7, assign81170_e123738_d_n8, assign81170_e123738_d_n9, assign81170_e123738_d_n10, assign81170_e123738_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        let assign81170_e123731: f64 = (-locals.var_vxbgmtcl);
        let assign81170_e123732: f64 = (locals.var_beta * assign81170_e123731);
        let assign81170_e123733: f64 = (1.0 + assign81170_e123732);
        let assign81170_e123735: f64 = (assign81170_e123733 - 500.0);
        let assign81170_e123736: f64 = (1.403592217853e217 * assign81170_e123735);
        (assign81170_e123736, (1.403592217853e217 * ((locals.var_beta_dn0 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign81170_e123731) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81170_e123738;
        locals.var_exp_bvbs_dn0 = assign81170_e123738_d_n0;
        locals.var_exp_bvbs_dn2 = assign81170_e123738_d_n2;
        locals.var_exp_bvbs_dn4 = assign81170_e123738_d_n4;
        locals.var_exp_bvbs_dn5 = assign81170_e123738_d_n5;
        locals.var_exp_bvbs_dn6 = assign81170_e123738_d_n6;
        locals.var_exp_bvbs_dn7 = assign81170_e123738_d_n7;
        locals.var_exp_bvbs_dn8 = assign81170_e123738_d_n8;
        locals.var_exp_bvbs_dn9 = assign81170_e123738_d_n9;
        locals.var_exp_bvbs_dn10 = assign81170_e123738_d_n10;
        locals.var_exp_bvbs_dn13 = assign81170_e123738_d_n13;

        let (assign81180_e123744, assign81180_e123744_d_n0, assign81180_e123744_d_n2, assign81180_e123744_d_n4, assign81180_e123744_d_n5, assign81180_e123744_d_n6, assign81180_e123744_d_n7, assign81180_e123744_d_n8, assign81180_e123744_d_n9, assign81180_e123744_d_n10, assign81180_e123744_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81180_e123744;
        locals.var_t0_dn0 = assign81180_e123744_d_n0;
        locals.var_t0_dn2 = assign81180_e123744_d_n2;
        locals.var_t0_dn4 = assign81180_e123744_d_n4;
        locals.var_t0_dn5 = assign81180_e123744_d_n5;
        locals.var_t0_dn6 = assign81180_e123744_d_n6;
        locals.var_t0_dn7 = assign81180_e123744_d_n7;
        locals.var_t0_dn8 = assign81180_e123744_d_n8;
        locals.var_t0_dn9 = assign81180_e123744_d_n9;
        locals.var_t0_dn10 = assign81180_e123744_d_n10;
        locals.var_t0_dn13 = assign81180_e123744_d_n13;

        let (assign81190_e123754, assign81190_e123754_d_n0, assign81190_e123754_d_n2, assign81190_e123754_d_n4, assign81190_e123754_d_n5, assign81190_e123754_d_n6, assign81190_e123754_d_n7, assign81190_e123754_d_n8, assign81190_e123754_d_n9, assign81190_e123754_d_n10, assign81190_e123754_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81190_e123751: f64 = (-locals.var_vxbgmtcl);
        let assign81190_e123752: f64 = (locals.var_beta * assign81190_e123751);
        (assign81190_e123752, ((locals.var_beta_dn0 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign81190_e123751) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81190_e123754;
        locals.var_tmf1_dn0 = assign81190_e123754_d_n0;
        locals.var_tmf1_dn2 = assign81190_e123754_d_n2;
        locals.var_tmf1_dn4 = assign81190_e123754_d_n4;
        locals.var_tmf1_dn5 = assign81190_e123754_d_n5;
        locals.var_tmf1_dn6 = assign81190_e123754_d_n6;
        locals.var_tmf1_dn7 = assign81190_e123754_d_n7;
        locals.var_tmf1_dn8 = assign81190_e123754_d_n8;
        locals.var_tmf1_dn9 = assign81190_e123754_d_n9;
        locals.var_tmf1_dn10 = assign81190_e123754_d_n10;
        locals.var_tmf1_dn13 = assign81190_e123754_d_n13;

        let (assign81200_e123761, assign81200_e123761_d_n0, assign81200_e123761_d_n2, assign81200_e123761_d_n4, assign81200_e123761_d_n5, assign81200_e123761_d_n6, assign81200_e123761_d_n7, assign81200_e123761_d_n8, assign81200_e123761_d_n9, assign81200_e123761_d_n10, assign81200_e123761_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81200_e123761;
        locals.var_exp_bvbs_dn0 = assign81200_e123761_d_n0;
        locals.var_exp_bvbs_dn2 = assign81200_e123761_d_n2;
        locals.var_exp_bvbs_dn4 = assign81200_e123761_d_n4;
        locals.var_exp_bvbs_dn5 = assign81200_e123761_d_n5;
        locals.var_exp_bvbs_dn6 = assign81200_e123761_d_n6;
        locals.var_exp_bvbs_dn7 = assign81200_e123761_d_n7;
        locals.var_exp_bvbs_dn8 = assign81200_e123761_d_n8;
        locals.var_exp_bvbs_dn9 = assign81200_e123761_d_n9;
        locals.var_exp_bvbs_dn10 = assign81200_e123761_d_n10;
        locals.var_exp_bvbs_dn13 = assign81200_e123761_d_n13;

        let mut assign81210_loop_guard: usize = 0;
        while {
            let assign81210_cond_e123769: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign81210_cond_e123769 != 0.0
        } {
            assign81210_loop_guard += 1;
            assert!(assign81210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81210_body0_e123778, assign81210_body0_e123778_d_n0, assign81210_body0_e123778_d_n2, assign81210_body0_e123778_d_n4, assign81210_body0_e123778_d_n5, assign81210_body0_e123778_d_n6, assign81210_body0_e123778_d_n7, assign81210_body0_e123778_d_n8, assign81210_body0_e123778_d_n9, assign81210_body0_e123778_d_n10, assign81210_body0_e123778_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81210_body0_e123776: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign81210_body0_e123776, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign81210_body0_e123778;
            locals.var_exp_bvbs_dn0 = assign81210_body0_e123778_d_n0;
            locals.var_exp_bvbs_dn2 = assign81210_body0_e123778_d_n2;
            locals.var_exp_bvbs_dn4 = assign81210_body0_e123778_d_n4;
            locals.var_exp_bvbs_dn5 = assign81210_body0_e123778_d_n5;
            locals.var_exp_bvbs_dn6 = assign81210_body0_e123778_d_n6;
            locals.var_exp_bvbs_dn7 = assign81210_body0_e123778_d_n7;
            locals.var_exp_bvbs_dn8 = assign81210_body0_e123778_d_n8;
            locals.var_exp_bvbs_dn9 = assign81210_body0_e123778_d_n9;
            locals.var_exp_bvbs_dn10 = assign81210_body0_e123778_d_n10;
            locals.var_exp_bvbs_dn13 = assign81210_body0_e123778_d_n13;
            let (assign81210_body1_e123787, assign81210_body1_e123787_d_n0, assign81210_body1_e123787_d_n2, assign81210_body1_e123787_d_n4, assign81210_body1_e123787_d_n5, assign81210_body1_e123787_d_n6, assign81210_body1_e123787_d_n7, assign81210_body1_e123787_d_n8, assign81210_body1_e123787_d_n9, assign81210_body1_e123787_d_n10, assign81210_body1_e123787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81210_body1_e123785: f64 = (locals.var_tmf1 - 60.0);
        (assign81210_body1_e123785, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign81210_body1_e123787;
            locals.var_tmf1_dn0 = assign81210_body1_e123787_d_n0;
            locals.var_tmf1_dn2 = assign81210_body1_e123787_d_n2;
            locals.var_tmf1_dn4 = assign81210_body1_e123787_d_n4;
            locals.var_tmf1_dn5 = assign81210_body1_e123787_d_n5;
            locals.var_tmf1_dn6 = assign81210_body1_e123787_d_n6;
            locals.var_tmf1_dn7 = assign81210_body1_e123787_d_n7;
            locals.var_tmf1_dn8 = assign81210_body1_e123787_d_n8;
            locals.var_tmf1_dn9 = assign81210_body1_e123787_d_n9;
            locals.var_tmf1_dn10 = assign81210_body1_e123787_d_n10;
            locals.var_tmf1_dn13 = assign81210_body1_e123787_d_n13;
        }

        let (assign81220_e123797, assign81220_e123797_d_n0, assign81220_e123797_d_n2, assign81220_e123797_d_n4, assign81220_e123797_d_n5, assign81220_e123797_d_n6, assign81220_e123797_d_n7, assign81220_e123797_d_n8, assign81220_e123797_d_n9, assign81220_e123797_d_n10, assign81220_e123797_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        let assign81220_e123794: f64 = (locals.var_tmf1).exp();
        let assign81220_e123795: f64 = (locals.var_exp_bvbs * assign81220_e123794);
        (assign81220_e123795, ((locals.var_exp_bvbs_dn0 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign81220_e123794) + (locals.var_exp_bvbs * (assign81220_e123794 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign81220_e123797;
        locals.var_exp_bvbs_dn0 = assign81220_e123797_d_n0;
        locals.var_exp_bvbs_dn2 = assign81220_e123797_d_n2;
        locals.var_exp_bvbs_dn4 = assign81220_e123797_d_n4;
        locals.var_exp_bvbs_dn5 = assign81220_e123797_d_n5;
        locals.var_exp_bvbs_dn6 = assign81220_e123797_d_n6;
        locals.var_exp_bvbs_dn7 = assign81220_e123797_d_n7;
        locals.var_exp_bvbs_dn8 = assign81220_e123797_d_n8;
        locals.var_exp_bvbs_dn9 = assign81220_e123797_d_n9;
        locals.var_exp_bvbs_dn10 = assign81220_e123797_d_n10;
        locals.var_exp_bvbs_dn13 = assign81220_e123797_d_n13;

        let (assign81230_e123804, assign81230_e123804_d_n0, assign81230_e123804_d_n2, assign81230_e123804_d_n4, assign81230_e123804_d_n5, assign81230_e123804_d_n6, assign81230_e123804_d_n7, assign81230_e123804_d_n8, assign81230_e123804_d_n9, assign81230_e123804_d_n10, assign81230_e123804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81230_e123804;
        locals.var_t0_dn0 = assign81230_e123804_d_n0;
        locals.var_t0_dn2 = assign81230_e123804_d_n2;
        locals.var_t0_dn4 = assign81230_e123804_d_n4;
        locals.var_t0_dn5 = assign81230_e123804_d_n5;
        locals.var_t0_dn6 = assign81230_e123804_d_n6;
        locals.var_t0_dn7 = assign81230_e123804_d_n7;
        locals.var_t0_dn8 = assign81230_e123804_d_n8;
        locals.var_t0_dn9 = assign81230_e123804_d_n9;
        locals.var_t0_dn10 = assign81230_e123804_d_n10;
        locals.var_t0_dn13 = assign81230_e123804_d_n13;

        let (assign81240_e123817, assign81240_e123817_d_n0, assign81240_e123817_d_n2, assign81240_e123817_d_n4, assign81240_e123817_d_n5, assign81240_e123817_d_n6, assign81240_e123817_d_n7, assign81240_e123817_d_n8, assign81240_e123817_d_n9, assign81240_e123817_d_n10, assign81240_e123817_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81240_e123809: f64 = (-locals.var_vgpld);
        let assign81240_e123811: f64 = (assign81240_e123809 * 0.5);
        let assign81240_e123813: f64 = (assign81240_e123811 - 0.5);
        let assign81240_e123815: f64 = (assign81240_e123813 - 1.0);
        (assign81240_e123815, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81240_e123817;
        locals.var_tmf1_dn0 = assign81240_e123817_d_n0;
        locals.var_tmf1_dn2 = assign81240_e123817_d_n2;
        locals.var_tmf1_dn4 = assign81240_e123817_d_n4;
        locals.var_tmf1_dn5 = assign81240_e123817_d_n5;
        locals.var_tmf1_dn6 = assign81240_e123817_d_n6;
        locals.var_tmf1_dn7 = assign81240_e123817_d_n7;
        locals.var_tmf1_dn8 = assign81240_e123817_d_n8;
        locals.var_tmf1_dn9 = assign81240_e123817_d_n9;
        locals.var_tmf1_dn10 = assign81240_e123817_d_n10;
        locals.var_tmf1_dn13 = assign81240_e123817_d_n13;

        let (assign81250_e123827, assign81250_e123827_d_n0, assign81250_e123827_d_n2, assign81250_e123827_d_n4, assign81250_e123827_d_n5, assign81250_e123827_d_n6, assign81250_e123827_d_n7, assign81250_e123827_d_n8, assign81250_e123827_d_n9, assign81250_e123827_d_n10, assign81250_e123827_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81250_e123823: f64 = (4.0 * 0.5);
        let assign81250_e123825: f64 = assign81250_e123823;
        (assign81250_e123825, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81250_e123827;
        locals.var_tmf2_dn0 = assign81250_e123827_d_n0;
        locals.var_tmf2_dn2 = assign81250_e123827_d_n2;
        locals.var_tmf2_dn4 = assign81250_e123827_d_n4;
        locals.var_tmf2_dn5 = assign81250_e123827_d_n5;
        locals.var_tmf2_dn6 = assign81250_e123827_d_n6;
        locals.var_tmf2_dn7 = assign81250_e123827_d_n7;
        locals.var_tmf2_dn8 = assign81250_e123827_d_n8;
        locals.var_tmf2_dn9 = assign81250_e123827_d_n9;
        locals.var_tmf2_dn10 = assign81250_e123827_d_n10;
        locals.var_tmf2_dn13 = assign81250_e123827_d_n13;

        let (assign81260_e123839, assign81260_e123839_d_n0, assign81260_e123839_d_n2, assign81260_e123839_d_n4, assign81260_e123839_d_n5, assign81260_e123839_d_n6, assign81260_e123839_d_n7, assign81260_e123839_d_n8, assign81260_e123839_d_n9, assign81260_e123839_d_n10, assign81260_e123839_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign81260_e123837, assign81260_e123837_d_n0, assign81260_e123837_d_n2, assign81260_e123837_d_n4, assign81260_e123837_d_n5, assign81260_e123837_d_n6, assign81260_e123837_d_n7, assign81260_e123837_d_n8, assign81260_e123837_d_n9, assign81260_e123837_d_n10, assign81260_e123837_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign81260_e123836: f64 = (-locals.var_tmf2);
                (assign81260_e123836, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign81260_e123837, assign81260_e123837_d_n0, assign81260_e123837_d_n2, assign81260_e123837_d_n4, assign81260_e123837_d_n5, assign81260_e123837_d_n6, assign81260_e123837_d_n7, assign81260_e123837_d_n8, assign81260_e123837_d_n9, assign81260_e123837_d_n10, assign81260_e123837_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81260_e123839;
        locals.var_tmf2_dn0 = assign81260_e123839_d_n0;
        locals.var_tmf2_dn2 = assign81260_e123839_d_n2;
        locals.var_tmf2_dn4 = assign81260_e123839_d_n4;
        locals.var_tmf2_dn5 = assign81260_e123839_d_n5;
        locals.var_tmf2_dn6 = assign81260_e123839_d_n6;
        locals.var_tmf2_dn7 = assign81260_e123839_d_n7;
        locals.var_tmf2_dn8 = assign81260_e123839_d_n8;
        locals.var_tmf2_dn9 = assign81260_e123839_d_n9;
        locals.var_tmf2_dn10 = assign81260_e123839_d_n10;
        locals.var_tmf2_dn13 = assign81260_e123839_d_n13;

        let (assign81270_e123850, assign81270_e123850_d_n0, assign81270_e123850_d_n2, assign81270_e123850_d_n4, assign81270_e123850_d_n5, assign81270_e123850_d_n6, assign81270_e123850_d_n7, assign81270_e123850_d_n8, assign81270_e123850_d_n9, assign81270_e123850_d_n10, assign81270_e123850_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81270_e123845: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign81270_e123847: f64 = (assign81270_e123845 + locals.var_tmf2);
        let assign81270_e123848: f64 = (assign81270_e123847).sqrt();
        (assign81270_e123848, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign81270_e123848)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign81270_e123848)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign81270_e123850;
        locals.var_tmf2_dn0 = assign81270_e123850_d_n0;
        locals.var_tmf2_dn2 = assign81270_e123850_d_n2;
        locals.var_tmf2_dn4 = assign81270_e123850_d_n4;
        locals.var_tmf2_dn5 = assign81270_e123850_d_n5;
        locals.var_tmf2_dn6 = assign81270_e123850_d_n6;
        locals.var_tmf2_dn7 = assign81270_e123850_d_n7;
        locals.var_tmf2_dn8 = assign81270_e123850_d_n8;
        locals.var_tmf2_dn9 = assign81270_e123850_d_n9;
        locals.var_tmf2_dn10 = assign81270_e123850_d_n10;
        locals.var_tmf2_dn13 = assign81270_e123850_d_n13;

        let (assign81280_e123862, assign81280_e123862_d_n0, assign81280_e123862_d_n2, assign81280_e123862_d_n4, assign81280_e123862_d_n5, assign81280_e123862_d_n6, assign81280_e123862_d_n7, assign81280_e123862_d_n8, assign81280_e123862_d_n9, assign81280_e123862_d_n10, assign81280_e123862_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81280_e123858: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign81280_e123859: f64 = (1.0 + assign81280_e123858);
        let assign81280_e123860: f64 = (0.5 * assign81280_e123859);
        (assign81280_e123860, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81280_e123862;
        locals.var_t0_dn0 = assign81280_e123862_d_n0;
        locals.var_t0_dn2 = assign81280_e123862_d_n2;
        locals.var_t0_dn4 = assign81280_e123862_d_n4;
        locals.var_t0_dn5 = assign81280_e123862_d_n5;
        locals.var_t0_dn6 = assign81280_e123862_d_n6;
        locals.var_t0_dn7 = assign81280_e123862_d_n7;
        locals.var_t0_dn8 = assign81280_e123862_d_n8;
        locals.var_t0_dn9 = assign81280_e123862_d_n9;
        locals.var_t0_dn10 = assign81280_e123862_d_n10;
        locals.var_t0_dn13 = assign81280_e123862_d_n13;

    }

    pub(super) fn stamp_transient_block_283(
        locals: &mut StampLocals,
    ) {
        let (assign81290_e123874, assign81290_e123874_d_n0, assign81290_e123874_d_n2, assign81290_e123874_d_n4, assign81290_e123874_d_n5, assign81290_e123874_d_n6, assign81290_e123874_d_n7, assign81290_e123874_d_n8, assign81290_e123874_d_n9, assign81290_e123874_d_n10, assign81290_e123874_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81290_e123870: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign81290_e123871: f64 = (0.5 * assign81290_e123870);
        let assign81290_e123872: f64 = (0.5 + assign81290_e123871);
        (assign81290_e123872, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81290_e123874;
        locals.var_t1_dn0 = assign81290_e123874_d_n0;
        locals.var_t1_dn2 = assign81290_e123874_d_n2;
        locals.var_t1_dn4 = assign81290_e123874_d_n4;
        locals.var_t1_dn5 = assign81290_e123874_d_n5;
        locals.var_t1_dn6 = assign81290_e123874_d_n6;
        locals.var_t1_dn7 = assign81290_e123874_d_n7;
        locals.var_t1_dn8 = assign81290_e123874_d_n8;
        locals.var_t1_dn9 = assign81290_e123874_d_n9;
        locals.var_t1_dn10 = assign81290_e123874_d_n10;
        locals.var_t1_dn13 = assign81290_e123874_d_n13;

        let assign81300_e123877: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81300_e123880: f64 = (-locals.var_t1);
        let assign81300_e123885: f64 = if ((assign81300_e123877 > assign81300_e123880) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1901 = assign81300_e123885;

        let (assign81310_e123899, assign81310_e123899_d_n0, assign81310_e123899_d_n2, assign81310_e123899_d_n4, assign81310_e123899_d_n5, assign81310_e123899_d_n6, assign81310_e123899_d_n7, assign81310_e123899_d_n8, assign81310_e123899_d_n9, assign81310_e123899_d_n10, assign81310_e123899_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81310_e123893: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81310_e123895: f64 = assign81310_e123893;
        let assign81310_e123897: f64 = (assign81310_e123895 + locals.var_t1);
        (assign81310_e123897, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign81310_e123899;
        locals.var_tmf1_dn0 = assign81310_e123899_d_n0;
        locals.var_tmf1_dn2 = assign81310_e123899_d_n2;
        locals.var_tmf1_dn4 = assign81310_e123899_d_n4;
        locals.var_tmf1_dn5 = assign81310_e123899_d_n5;
        locals.var_tmf1_dn6 = assign81310_e123899_d_n6;
        locals.var_tmf1_dn7 = assign81310_e123899_d_n7;
        locals.var_tmf1_dn8 = assign81310_e123899_d_n8;
        locals.var_tmf1_dn9 = assign81310_e123899_d_n9;
        locals.var_tmf1_dn10 = assign81310_e123899_d_n10;
        locals.var_tmf1_dn13 = assign81310_e123899_d_n13;

        let (assign81320_e123909, assign81320_e123909_d_n0, assign81320_e123909_d_n2, assign81320_e123909_d_n4, assign81320_e123909_d_n5, assign81320_e123909_d_n6, assign81320_e123909_d_n7, assign81320_e123909_d_n8, assign81320_e123909_d_n9, assign81320_e123909_d_n10, assign81320_e123909_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81320_e123907: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign81320_e123907, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign81320_e123909;
        locals.var_x2_dn0 = assign81320_e123909_d_n0;
        locals.var_x2_dn2 = assign81320_e123909_d_n2;
        locals.var_x2_dn4 = assign81320_e123909_d_n4;
        locals.var_x2_dn5 = assign81320_e123909_d_n5;
        locals.var_x2_dn6 = assign81320_e123909_d_n6;
        locals.var_x2_dn7 = assign81320_e123909_d_n7;
        locals.var_x2_dn8 = assign81320_e123909_d_n8;
        locals.var_x2_dn9 = assign81320_e123909_d_n9;
        locals.var_x2_dn10 = assign81320_e123909_d_n10;
        locals.var_x2_dn13 = assign81320_e123909_d_n13;

        let (assign81330_e123919, assign81330_e123919_d_n0, assign81330_e123919_d_n2, assign81330_e123919_d_n4, assign81330_e123919_d_n5, assign81330_e123919_d_n6, assign81330_e123919_d_n7, assign81330_e123919_d_n8, assign81330_e123919_d_n9, assign81330_e123919_d_n10, assign81330_e123919_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81330_e123917: f64 = (locals.var_t1 * locals.var_t1);
        (assign81330_e123917, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign81330_e123919;
        locals.var_xmax2_dn0 = assign81330_e123919_d_n0;
        locals.var_xmax2_dn2 = assign81330_e123919_d_n2;
        locals.var_xmax2_dn4 = assign81330_e123919_d_n4;
        locals.var_xmax2_dn5 = assign81330_e123919_d_n5;
        locals.var_xmax2_dn6 = assign81330_e123919_d_n6;
        locals.var_xmax2_dn7 = assign81330_e123919_d_n7;
        locals.var_xmax2_dn8 = assign81330_e123919_d_n8;
        locals.var_xmax2_dn9 = assign81330_e123919_d_n9;
        locals.var_xmax2_dn10 = assign81330_e123919_d_n10;
        locals.var_xmax2_dn13 = assign81330_e123919_d_n13;

        let (assign81340_e123927, assign81340_e123927_d_n0, assign81340_e123927_d_n2, assign81340_e123927_d_n4, assign81340_e123927_d_n5, assign81340_e123927_d_n6, assign81340_e123927_d_n7, assign81340_e123927_d_n8, assign81340_e123927_d_n9, assign81340_e123927_d_n10, assign81340_e123927_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign81340_e123927;
        locals.var_xp_dn0 = assign81340_e123927_d_n0;
        locals.var_xp_dn2 = assign81340_e123927_d_n2;
        locals.var_xp_dn4 = assign81340_e123927_d_n4;
        locals.var_xp_dn5 = assign81340_e123927_d_n5;
        locals.var_xp_dn6 = assign81340_e123927_d_n6;
        locals.var_xp_dn7 = assign81340_e123927_d_n7;
        locals.var_xp_dn8 = assign81340_e123927_d_n8;
        locals.var_xp_dn9 = assign81340_e123927_d_n9;
        locals.var_xp_dn10 = assign81340_e123927_d_n10;
        locals.var_xp_dn13 = assign81340_e123927_d_n13;

        let (assign81350_e123935, assign81350_e123935_d_n0, assign81350_e123935_d_n2, assign81350_e123935_d_n4, assign81350_e123935_d_n5, assign81350_e123935_d_n6, assign81350_e123935_d_n7, assign81350_e123935_d_n8, assign81350_e123935_d_n9, assign81350_e123935_d_n10, assign81350_e123935_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign81350_e123935;
        locals.var_xmp_dn0 = assign81350_e123935_d_n0;
        locals.var_xmp_dn2 = assign81350_e123935_d_n2;
        locals.var_xmp_dn4 = assign81350_e123935_d_n4;
        locals.var_xmp_dn5 = assign81350_e123935_d_n5;
        locals.var_xmp_dn6 = assign81350_e123935_d_n6;
        locals.var_xmp_dn7 = assign81350_e123935_d_n7;
        locals.var_xmp_dn8 = assign81350_e123935_d_n8;
        locals.var_xmp_dn9 = assign81350_e123935_d_n9;
        locals.var_xmp_dn10 = assign81350_e123935_d_n10;
        locals.var_xmp_dn13 = assign81350_e123935_d_n13;

        let (assign81360_e123943,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81360_e123943;

        let (assign81370_e123951,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81370_e123951;

        let (assign81380_e123959, assign81380_e123959_d_n0, assign81380_e123959_d_n2, assign81380_e123959_d_n4, assign81380_e123959_d_n5, assign81380_e123959_d_n6, assign81380_e123959_d_n7, assign81380_e123959_d_n8, assign81380_e123959_d_n9, assign81380_e123959_d_n10, assign81380_e123959_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign81380_e123959;
        locals.var_arg_dn0 = assign81380_e123959_d_n0;
        locals.var_arg_dn2 = assign81380_e123959_d_n2;
        locals.var_arg_dn4 = assign81380_e123959_d_n4;
        locals.var_arg_dn5 = assign81380_e123959_d_n5;
        locals.var_arg_dn6 = assign81380_e123959_d_n6;
        locals.var_arg_dn7 = assign81380_e123959_d_n7;
        locals.var_arg_dn8 = assign81380_e123959_d_n8;
        locals.var_arg_dn9 = assign81380_e123959_d_n9;
        locals.var_arg_dn10 = assign81380_e123959_d_n10;
        locals.var_arg_dn13 = assign81380_e123959_d_n13;

        let (assign81390_e123967, assign81390_e123967_d_n0, assign81390_e123967_d_n2, assign81390_e123967_d_n4, assign81390_e123967_d_n5, assign81390_e123967_d_n6, assign81390_e123967_d_n7, assign81390_e123967_d_n8, assign81390_e123967_d_n9, assign81390_e123967_d_n10, assign81390_e123967_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81390_e123967;
        locals.var_dnm_dn0 = assign81390_e123967_d_n0;
        locals.var_dnm_dn2 = assign81390_e123967_d_n2;
        locals.var_dnm_dn4 = assign81390_e123967_d_n4;
        locals.var_dnm_dn5 = assign81390_e123967_d_n5;
        locals.var_dnm_dn6 = assign81390_e123967_d_n6;
        locals.var_dnm_dn7 = assign81390_e123967_d_n7;
        locals.var_dnm_dn8 = assign81390_e123967_d_n8;
        locals.var_dnm_dn9 = assign81390_e123967_d_n9;
        locals.var_dnm_dn10 = assign81390_e123967_d_n10;
        locals.var_dnm_dn13 = assign81390_e123967_d_n13;

        let (assign81400_e123977, assign81400_e123977_d_n0, assign81400_e123977_d_n2, assign81400_e123977_d_n4, assign81400_e123977_d_n5, assign81400_e123977_d_n6, assign81400_e123977_d_n7, assign81400_e123977_d_n8, assign81400_e123977_d_n9, assign81400_e123977_d_n10, assign81400_e123977_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81400_e123975: f64 = (locals.var_xp * locals.var_x2);
        (assign81400_e123975, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign81400_e123977;
        locals.var_xp_dn0 = assign81400_e123977_d_n0;
        locals.var_xp_dn2 = assign81400_e123977_d_n2;
        locals.var_xp_dn4 = assign81400_e123977_d_n4;
        locals.var_xp_dn5 = assign81400_e123977_d_n5;
        locals.var_xp_dn6 = assign81400_e123977_d_n6;
        locals.var_xp_dn7 = assign81400_e123977_d_n7;
        locals.var_xp_dn8 = assign81400_e123977_d_n8;
        locals.var_xp_dn9 = assign81400_e123977_d_n9;
        locals.var_xp_dn10 = assign81400_e123977_d_n10;
        locals.var_xp_dn13 = assign81400_e123977_d_n13;

        let (assign81410_e123987, assign81410_e123987_d_n0, assign81410_e123987_d_n2, assign81410_e123987_d_n4, assign81410_e123987_d_n5, assign81410_e123987_d_n6, assign81410_e123987_d_n7, assign81410_e123987_d_n8, assign81410_e123987_d_n9, assign81410_e123987_d_n10, assign81410_e123987_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81410_e123985: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign81410_e123985, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign81410_e123987;
        locals.var_xmp_dn0 = assign81410_e123987_d_n0;
        locals.var_xmp_dn2 = assign81410_e123987_d_n2;
        locals.var_xmp_dn4 = assign81410_e123987_d_n4;
        locals.var_xmp_dn5 = assign81410_e123987_d_n5;
        locals.var_xmp_dn6 = assign81410_e123987_d_n6;
        locals.var_xmp_dn7 = assign81410_e123987_d_n7;
        locals.var_xmp_dn8 = assign81410_e123987_d_n8;
        locals.var_xmp_dn9 = assign81410_e123987_d_n9;
        locals.var_xmp_dn10 = assign81410_e123987_d_n10;
        locals.var_xmp_dn13 = assign81410_e123987_d_n13;

        let (assign81420_e123997, assign81420_e123997_d_n0, assign81420_e123997_d_n2, assign81420_e123997_d_n4, assign81420_e123997_d_n5, assign81420_e123997_d_n6, assign81420_e123997_d_n7, assign81420_e123997_d_n8, assign81420_e123997_d_n9, assign81420_e123997_d_n10, assign81420_e123997_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81420_e123995: f64 = (locals.var_xp + locals.var_xmp);
        (assign81420_e123995, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign81420_e123997;
        locals.var_arg_dn0 = assign81420_e123997_d_n0;
        locals.var_arg_dn2 = assign81420_e123997_d_n2;
        locals.var_arg_dn4 = assign81420_e123997_d_n4;
        locals.var_arg_dn5 = assign81420_e123997_d_n5;
        locals.var_arg_dn6 = assign81420_e123997_d_n6;
        locals.var_arg_dn7 = assign81420_e123997_d_n7;
        locals.var_arg_dn8 = assign81420_e123997_d_n8;
        locals.var_arg_dn9 = assign81420_e123997_d_n9;
        locals.var_arg_dn10 = assign81420_e123997_d_n10;
        locals.var_arg_dn13 = assign81420_e123997_d_n13;

        let (assign81430_e124005, assign81430_e124005_d_n0, assign81430_e124005_d_n2, assign81430_e124005_d_n4, assign81430_e124005_d_n5, assign81430_e124005_d_n6, assign81430_e124005_d_n7, assign81430_e124005_d_n8, assign81430_e124005_d_n9, assign81430_e124005_d_n10, assign81430_e124005_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81430_e124005;
        locals.var_dnm_dn0 = assign81430_e124005_d_n0;
        locals.var_dnm_dn2 = assign81430_e124005_d_n2;
        locals.var_dnm_dn4 = assign81430_e124005_d_n4;
        locals.var_dnm_dn5 = assign81430_e124005_d_n5;
        locals.var_dnm_dn6 = assign81430_e124005_d_n6;
        locals.var_dnm_dn7 = assign81430_e124005_d_n7;
        locals.var_dnm_dn8 = assign81430_e124005_d_n8;
        locals.var_dnm_dn9 = assign81430_e124005_d_n9;
        locals.var_dnm_dn10 = assign81430_e124005_d_n10;
        locals.var_dnm_dn13 = assign81430_e124005_d_n13;

        let assign81440_e124020: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1902 = assign81440_e124020;

        let assign81450_e124023: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1903 = assign81450_e124023;

        let (assign81460_e124035,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81460_e124035;

        let assign81470_e124038: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1904 = assign81470_e124038;

        let (assign81480_e124053,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81480_e124053;

        let assign81490_e124056: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1905 = assign81490_e124056;

        let (assign81500_e124074,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 == 0.0)) && (locals.var_guard1905 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81500_e124074;

        let assign81510_e124077: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1906 = assign81510_e124077;

        let (assign81520_e124098,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_guard1903 == 0.0)) && (locals.var_guard1904 == 0.0)) && (locals.var_guard1905 == 0.0)) && (locals.var_guard1906 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81520_e124098;

        let (assign81530_e124108,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81530_e124108;

        let mut assign81540_loop_guard: usize = 0;
        while {
            let assign81540_cond_e124119: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign81540_cond_e124119 != 0.0
        } {
            assign81540_loop_guard += 1;
            assert!(assign81540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81540_body0_e124130, assign81540_body0_e124130_d_n0, assign81540_body0_e124130_d_n2, assign81540_body0_e124130_d_n4, assign81540_body0_e124130_d_n5, assign81540_body0_e124130_d_n6, assign81540_body0_e124130_d_n7, assign81540_body0_e124130_d_n8, assign81540_body0_e124130_d_n9, assign81540_body0_e124130_d_n10, assign81540_body0_e124130_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        let assign81540_body0_e124128: f64 = (locals.var_dnm).sqrt();
        (assign81540_body0_e124128, (locals.var_dnm_dn0 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn2 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn4 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn5 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn6 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn7 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn8 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn9 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn10 / (2.0 * assign81540_body0_e124128)), (locals.var_dnm_dn13 / (2.0 * assign81540_body0_e124128)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign81540_body0_e124130;
            locals.var_dnm_dn0 = assign81540_body0_e124130_d_n0;
            locals.var_dnm_dn2 = assign81540_body0_e124130_d_n2;
            locals.var_dnm_dn4 = assign81540_body0_e124130_d_n4;
            locals.var_dnm_dn5 = assign81540_body0_e124130_d_n5;
            locals.var_dnm_dn6 = assign81540_body0_e124130_d_n6;
            locals.var_dnm_dn7 = assign81540_body0_e124130_d_n7;
            locals.var_dnm_dn8 = assign81540_body0_e124130_d_n8;
            locals.var_dnm_dn9 = assign81540_body0_e124130_d_n9;
            locals.var_dnm_dn10 = assign81540_body0_e124130_d_n10;
            locals.var_dnm_dn13 = assign81540_body0_e124130_d_n13;
            let (assign81540_body1_e124142,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 != 0.0)) {
        let assign81540_body1_e124140: f64 = (locals.var_m0 + 1.0);
        (assign81540_body1_e124140,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign81540_body1_e124142;
        }

        let (assign81550_e124164, assign81550_e124164_d_n0, assign81550_e124164_d_n2, assign81550_e124164_d_n4, assign81550_e124164_d_n5, assign81550_e124164_d_n6, assign81550_e124164_d_n7, assign81550_e124164_d_n8, assign81550_e124164_d_n9, assign81550_e124164_d_n10, assign81550_e124164_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) && (locals.var_guard1902 == 0.0)) {
        let (assign81550_e124162, assign81550_e124162_d_n0, assign81550_e124162_d_n2, assign81550_e124162_d_n4, assign81550_e124162_d_n5, assign81550_e124162_d_n6, assign81550_e124162_d_n7, assign81550_e124162_d_n8, assign81550_e124162_d_n9, assign81550_e124162_d_n10, assign81550_e124162_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign81550_e124159: f64 = 2.0;
                let assign81550_e124160: f64 = (1.0 / assign81550_e124159);
                let assign81550_e124161: f64 = (locals.var_dnm).powf(assign81550_e124160);
                (assign81550_e124161, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn0)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn2)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn4)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn5)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn6)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn7)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn8)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn9)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn10)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81550_e124160) as f64).is_finite() && ((assign81550_e124160) as f64).fract() == 0.0 { if assign81550_e124160 == 0.0 { 0.0 } else { (assign81550_e124160 * ((locals.var_dnm).powf(assign81550_e124160 - 1.0) * locals.var_dnm_dn13)) } } else { (assign81550_e124161 * (assign81550_e124160 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign81550_e124162, assign81550_e124162_d_n0, assign81550_e124162_d_n2, assign81550_e124162_d_n4, assign81550_e124162_d_n5, assign81550_e124162_d_n6, assign81550_e124162_d_n7, assign81550_e124162_d_n8, assign81550_e124162_d_n9, assign81550_e124162_d_n10, assign81550_e124162_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81550_e124164;
        locals.var_dnm_dn0 = assign81550_e124164_d_n0;
        locals.var_dnm_dn2 = assign81550_e124164_d_n2;
        locals.var_dnm_dn4 = assign81550_e124164_d_n4;
        locals.var_dnm_dn5 = assign81550_e124164_d_n5;
        locals.var_dnm_dn6 = assign81550_e124164_d_n6;
        locals.var_dnm_dn7 = assign81550_e124164_d_n7;
        locals.var_dnm_dn8 = assign81550_e124164_d_n8;
        locals.var_dnm_dn9 = assign81550_e124164_d_n9;
        locals.var_dnm_dn10 = assign81550_e124164_d_n10;
        locals.var_dnm_dn13 = assign81550_e124164_d_n13;

        let (assign81560_e124174, assign81560_e124174_d_n0, assign81560_e124174_d_n2, assign81560_e124174_d_n4, assign81560_e124174_d_n5, assign81560_e124174_d_n6, assign81560_e124174_d_n7, assign81560_e124174_d_n8, assign81560_e124174_d_n9, assign81560_e124174_d_n10, assign81560_e124174_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81560_e124172: f64 = (1.0 / locals.var_dnm);
        (assign81560_e124172, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign81560_e124174;
        locals.var_dnm_dn0 = assign81560_e124174_d_n0;
        locals.var_dnm_dn2 = assign81560_e124174_d_n2;
        locals.var_dnm_dn4 = assign81560_e124174_d_n4;
        locals.var_dnm_dn5 = assign81560_e124174_d_n5;
        locals.var_dnm_dn6 = assign81560_e124174_d_n6;
        locals.var_dnm_dn7 = assign81560_e124174_d_n7;
        locals.var_dnm_dn8 = assign81560_e124174_d_n8;
        locals.var_dnm_dn9 = assign81560_e124174_d_n9;
        locals.var_dnm_dn10 = assign81560_e124174_d_n10;
        locals.var_dnm_dn13 = assign81560_e124174_d_n13;

        let (assign81570_e124186, assign81570_e124186_d_n0, assign81570_e124186_d_n2, assign81570_e124186_d_n4, assign81570_e124186_d_n5, assign81570_e124186_d_n6, assign81570_e124186_d_n7, assign81570_e124186_d_n8, assign81570_e124186_d_n9, assign81570_e124186_d_n10, assign81570_e124186_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81570_e124182: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign81570_e124184: f64 = (assign81570_e124182 * locals.var_dnm);
        (assign81570_e124184, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign81570_e124182 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign81570_e124186;
        locals.var_tmf0_dn0 = assign81570_e124186_d_n0;
        locals.var_tmf0_dn2 = assign81570_e124186_d_n2;
        locals.var_tmf0_dn4 = assign81570_e124186_d_n4;
        locals.var_tmf0_dn5 = assign81570_e124186_d_n5;
        locals.var_tmf0_dn6 = assign81570_e124186_d_n6;
        locals.var_tmf0_dn7 = assign81570_e124186_d_n7;
        locals.var_tmf0_dn8 = assign81570_e124186_d_n8;
        locals.var_tmf0_dn9 = assign81570_e124186_d_n9;
        locals.var_tmf0_dn10 = assign81570_e124186_d_n10;
        locals.var_tmf0_dn13 = assign81570_e124186_d_n13;

        let (assign81580_e124200, assign81580_e124200_d_n0, assign81580_e124200_d_n2, assign81580_e124200_d_n4, assign81580_e124200_d_n5, assign81580_e124200_d_n6, assign81580_e124200_d_n7, assign81580_e124200_d_n8, assign81580_e124200_d_n9, assign81580_e124200_d_n10, assign81580_e124200_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81580_e124194: f64 = (locals.var_t1 * locals.var_xmp);
        let assign81580_e124196: f64 = (assign81580_e124194 * locals.var_dnm);
        let assign81580_e124198: f64 = (assign81580_e124196 / locals.var_arg);
        (assign81580_e124198, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn0)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn2)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn4)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn5)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn6)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn7)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn8)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn9)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn10)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign81580_e124194 * locals.var_dnm_dn13)) * locals.var_arg) - (assign81580_e124196 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81580_e124200;
        locals.var_t0_dn0 = assign81580_e124200_d_n0;
        locals.var_t0_dn2 = assign81580_e124200_d_n2;
        locals.var_t0_dn4 = assign81580_e124200_d_n4;
        locals.var_t0_dn5 = assign81580_e124200_d_n5;
        locals.var_t0_dn6 = assign81580_e124200_d_n6;
        locals.var_t0_dn7 = assign81580_e124200_d_n7;
        locals.var_t0_dn8 = assign81580_e124200_d_n8;
        locals.var_t0_dn9 = assign81580_e124200_d_n9;
        locals.var_t0_dn10 = assign81580_e124200_d_n10;
        locals.var_t0_dn13 = assign81580_e124200_d_n13;

        let (assign81590_e124212, assign81590_e124212_d_n0, assign81590_e124212_d_n2, assign81590_e124212_d_n4, assign81590_e124212_d_n5, assign81590_e124212_d_n6, assign81590_e124212_d_n7, assign81590_e124212_d_n8, assign81590_e124212_d_n9, assign81590_e124212_d_n10, assign81590_e124212_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81590_e124208: f64 = (-locals.var_t1);
        let assign81590_e124210: f64 = (assign81590_e124208 + locals.var_tmf0);
        (assign81590_e124210, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81590_e124212;
        locals.var_t1_dn0 = assign81590_e124212_d_n0;
        locals.var_t1_dn2 = assign81590_e124212_d_n2;
        locals.var_t1_dn4 = assign81590_e124212_d_n4;
        locals.var_t1_dn5 = assign81590_e124212_d_n5;
        locals.var_t1_dn6 = assign81590_e124212_d_n6;
        locals.var_t1_dn7 = assign81590_e124212_d_n7;
        locals.var_t1_dn8 = assign81590_e124212_d_n8;
        locals.var_t1_dn9 = assign81590_e124212_d_n9;
        locals.var_t1_dn10 = assign81590_e124212_d_n10;
        locals.var_t1_dn13 = assign81590_e124212_d_n13;

        let (assign81600_e124220, assign81600_e124220_d_n0, assign81600_e124220_d_n2, assign81600_e124220_d_n4, assign81600_e124220_d_n5, assign81600_e124220_d_n6, assign81600_e124220_d_n7, assign81600_e124220_d_n8, assign81600_e124220_d_n9, assign81600_e124220_d_n10, assign81600_e124220_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81600_e124220;
        locals.var_t0_dn0 = assign81600_e124220_d_n0;
        locals.var_t0_dn2 = assign81600_e124220_d_n2;
        locals.var_t0_dn4 = assign81600_e124220_d_n4;
        locals.var_t0_dn5 = assign81600_e124220_d_n5;
        locals.var_t0_dn6 = assign81600_e124220_d_n6;
        locals.var_t0_dn7 = assign81600_e124220_d_n7;
        locals.var_t0_dn8 = assign81600_e124220_d_n8;
        locals.var_t0_dn9 = assign81600_e124220_d_n9;
        locals.var_t0_dn10 = assign81600_e124220_d_n10;
        locals.var_t0_dn13 = assign81600_e124220_d_n13;

        let (assign81610_e124231, assign81610_e124231_d_n0, assign81610_e124231_d_n2, assign81610_e124231_d_n4, assign81610_e124231_d_n5, assign81610_e124231_d_n6, assign81610_e124231_d_n7, assign81610_e124231_d_n8, assign81610_e124231_d_n9, assign81610_e124231_d_n10, assign81610_e124231_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 == 0.0)) {
        let assign81610_e124229: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign81610_e124229, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81610_e124231;
        locals.var_t1_dn0 = assign81610_e124231_d_n0;
        locals.var_t1_dn2 = assign81610_e124231_d_n2;
        locals.var_t1_dn4 = assign81610_e124231_d_n4;
        locals.var_t1_dn5 = assign81610_e124231_d_n5;
        locals.var_t1_dn6 = assign81610_e124231_d_n6;
        locals.var_t1_dn7 = assign81610_e124231_d_n7;
        locals.var_t1_dn8 = assign81610_e124231_d_n8;
        locals.var_t1_dn9 = assign81610_e124231_d_n9;
        locals.var_t1_dn10 = assign81610_e124231_d_n10;
        locals.var_t1_dn13 = assign81610_e124231_d_n13;

    }

    pub(super) fn stamp_transient_block_284(
        locals: &mut StampLocals,
    ) {
        let (assign81620_e124240, assign81620_e124240_d_n0, assign81620_e124240_d_n2, assign81620_e124240_d_n4, assign81620_e124240_d_n5, assign81620_e124240_d_n6, assign81620_e124240_d_n7, assign81620_e124240_d_n8, assign81620_e124240_d_n9, assign81620_e124240_d_n10, assign81620_e124240_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1901 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign81620_e124240;
        locals.var_t0_dn0 = assign81620_e124240_d_n0;
        locals.var_t0_dn2 = assign81620_e124240_d_n2;
        locals.var_t0_dn4 = assign81620_e124240_d_n4;
        locals.var_t0_dn5 = assign81620_e124240_d_n5;
        locals.var_t0_dn6 = assign81620_e124240_d_n6;
        locals.var_t0_dn7 = assign81620_e124240_d_n7;
        locals.var_t0_dn8 = assign81620_e124240_d_n8;
        locals.var_t0_dn9 = assign81620_e124240_d_n9;
        locals.var_t0_dn10 = assign81620_e124240_d_n10;
        locals.var_t0_dn13 = assign81620_e124240_d_n13;

        let (assign81630_e124248, assign81630_e124248_d_n0, assign81630_e124248_d_n2, assign81630_e124248_d_n4, assign81630_e124248_d_n5, assign81630_e124248_d_n6, assign81630_e124248_d_n7, assign81630_e124248_d_n8, assign81630_e124248_d_n9, assign81630_e124248_d_n10, assign81630_e124248_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81630_e124246: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign81630_e124246, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign81630_e124248;
        locals.var_vxbgmtcl_dn0 = assign81630_e124248_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81630_e124248_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81630_e124248_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81630_e124248_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81630_e124248_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81630_e124248_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81630_e124248_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81630_e124248_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81630_e124248_d_n10;
        locals.var_vxbgmtcl_dn13 = assign81630_e124248_d_n13;

        let (assign81640_e124259,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81640_e124253: f64 = (-locals.var_vxbgmtcl);
        let assign81640_e124256: f64 = (10.0 * 2.220446049250313e-16);
        let assign81640_e124257: f64 = (assign81640_e124253 + assign81640_e124256);
        (assign81640_e124257,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign81640_e124259;

        let assign81650_e124262: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1907 = assign81650_e124262;

        let (assign81670_e124283, assign81670_e124283_d_n0, assign81670_e124283_d_n2, assign81670_e124283_d_n4, assign81670_e124283_d_n5, assign81670_e124283_d_n6, assign81670_e124283_d_n7, assign81670_e124283_d_n8, assign81670_e124283_d_n9, assign81670_e124283_d_n10, assign81670_e124283_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81670_e124275: f64 = (2.0 * locals.var_beta_inv);
        let assign81670_e124277: f64 = (-locals.var_vgs_min);
        let assign81670_e124279: f64 = (assign81670_e124277 / locals.var_fac1);
        let assign81670_e124280: f64 = (assign81670_e124279).ln();
        let assign81670_e124281: f64 = (assign81670_e124275 * assign81670_e124280);
        (assign81670_e124281, (((2.0 * locals.var_beta_inv_dn0) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn2) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn4) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn5) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn6) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn7) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn8) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn9) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn10) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))), (((2.0 * locals.var_beta_inv_dn13) * assign81670_e124280) + (assign81670_e124275 * ((-((assign81670_e124277 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign81670_e124279))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign81670_e124283;
        locals.var_ps0_min_dn0 = assign81670_e124283_d_n0;
        locals.var_ps0_min_dn2 = assign81670_e124283_d_n2;
        locals.var_ps0_min_dn4 = assign81670_e124283_d_n4;
        locals.var_ps0_min_dn5 = assign81670_e124283_d_n5;
        locals.var_ps0_min_dn6 = assign81670_e124283_d_n6;
        locals.var_ps0_min_dn7 = assign81670_e124283_d_n7;
        locals.var_ps0_min_dn8 = assign81670_e124283_d_n8;
        locals.var_ps0_min_dn9 = assign81670_e124283_d_n9;
        locals.var_ps0_min_dn10 = assign81670_e124283_d_n10;
        locals.var_ps0_min_dn13 = assign81670_e124283_d_n13;

        let (assign81680_e124293, assign81680_e124293_d_n0, assign81680_e124293_d_n2, assign81680_e124293_d_n4, assign81680_e124293_d_n5, assign81680_e124293_d_n6, assign81680_e124293_d_n7, assign81680_e124293_d_n8, assign81680_e124293_d_n9, assign81680_e124293_d_n10, assign81680_e124293_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81680_e124290: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81680_e124291: f64 = (locals.var_beta * assign81680_e124290);
        (assign81680_e124291, ((locals.var_beta_dn0 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81680_e124290) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign81680_e124290) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81680_e124293;
        locals.var_tx_dn0 = assign81680_e124293_d_n0;
        locals.var_tx_dn2 = assign81680_e124293_d_n2;
        locals.var_tx_dn4 = assign81680_e124293_d_n4;
        locals.var_tx_dn5 = assign81680_e124293_d_n5;
        locals.var_tx_dn6 = assign81680_e124293_d_n6;
        locals.var_tx_dn7 = assign81680_e124293_d_n7;
        locals.var_tx_dn8 = assign81680_e124293_d_n8;
        locals.var_tx_dn9 = assign81680_e124293_d_n9;
        locals.var_tx_dn10 = assign81680_e124293_d_n10;
        locals.var_tx_dn13 = assign81680_e124293_d_n13;

        let (assign81690_e124303, assign81690_e124303_d_n0, assign81690_e124303_d_n2, assign81690_e124303_d_n4, assign81690_e124303_d_n5, assign81690_e124303_d_n6, assign81690_e124303_d_n7, assign81690_e124303_d_n8, assign81690_e124303_d_n9, assign81690_e124303_d_n10, assign81690_e124303_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81690_e124300: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign81690_e124301: f64 = (1.0 / assign81690_e124300);
        (assign81690_e124301, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign81690_e124300 * assign81690_e124300))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign81690_e124300 * assign81690_e124300))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81690_e124303;
        locals.var_t1_dn0 = assign81690_e124303_d_n0;
        locals.var_t1_dn2 = assign81690_e124303_d_n2;
        locals.var_t1_dn4 = assign81690_e124303_d_n4;
        locals.var_t1_dn5 = assign81690_e124303_d_n5;
        locals.var_t1_dn6 = assign81690_e124303_d_n6;
        locals.var_t1_dn7 = assign81690_e124303_d_n7;
        locals.var_t1_dn8 = assign81690_e124303_d_n8;
        locals.var_t1_dn9 = assign81690_e124303_d_n9;
        locals.var_t1_dn10 = assign81690_e124303_d_n10;
        locals.var_t1_dn13 = assign81690_e124303_d_n13;

        let (assign81700_e124311, assign81700_e124311_d_n0, assign81700_e124311_d_n2, assign81700_e124311_d_n4, assign81700_e124311_d_n5, assign81700_e124311_d_n6, assign81700_e124311_d_n7, assign81700_e124311_d_n8, assign81700_e124311_d_n9, assign81700_e124311_d_n10, assign81700_e124311_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81700_e124309: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign81700_e124309, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81700_e124311;
        locals.var_ty_dn0 = assign81700_e124311_d_n0;
        locals.var_ty_dn2 = assign81700_e124311_d_n2;
        locals.var_ty_dn4 = assign81700_e124311_d_n4;
        locals.var_ty_dn5 = assign81700_e124311_d_n5;
        locals.var_ty_dn6 = assign81700_e124311_d_n6;
        locals.var_ty_dn7 = assign81700_e124311_d_n7;
        locals.var_ty_dn8 = assign81700_e124311_d_n8;
        locals.var_ty_dn9 = assign81700_e124311_d_n9;
        locals.var_ty_dn10 = assign81700_e124311_d_n10;
        locals.var_ty_dn13 = assign81700_e124311_d_n13;

        let (assign81710_e124323, assign81710_e124323_d_n0, assign81710_e124323_d_n2, assign81710_e124323_d_n4, assign81710_e124323_d_n5, assign81710_e124323_d_n6, assign81710_e124323_d_n7, assign81710_e124323_d_n8, assign81710_e124323_d_n9, assign81710_e124323_d_n10, assign81710_e124323_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81710_e124318: f64 = (3.0 * 1.414213562373095);
        let assign81710_e124320: f64 = (assign81710_e124318 * locals.var_ty);
        let assign81710_e124321: f64 = (2.0 + assign81710_e124320);
        (assign81710_e124321, (assign81710_e124318 * locals.var_ty_dn0), (assign81710_e124318 * locals.var_ty_dn2), (assign81710_e124318 * locals.var_ty_dn4), (assign81710_e124318 * locals.var_ty_dn5), (assign81710_e124318 * locals.var_ty_dn6), (assign81710_e124318 * locals.var_ty_dn7), (assign81710_e124318 * locals.var_ty_dn8), (assign81710_e124318 * locals.var_ty_dn9), (assign81710_e124318 * locals.var_ty_dn10), (assign81710_e124318 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign81710_e124323;
        locals.var_ac41_dn0 = assign81710_e124323_d_n0;
        locals.var_ac41_dn2 = assign81710_e124323_d_n2;
        locals.var_ac41_dn4 = assign81710_e124323_d_n4;
        locals.var_ac41_dn5 = assign81710_e124323_d_n5;
        locals.var_ac41_dn6 = assign81710_e124323_d_n6;
        locals.var_ac41_dn7 = assign81710_e124323_d_n7;
        locals.var_ac41_dn8 = assign81710_e124323_d_n8;
        locals.var_ac41_dn9 = assign81710_e124323_d_n9;
        locals.var_ac41_dn10 = assign81710_e124323_d_n10;
        locals.var_ac41_dn13 = assign81710_e124323_d_n13;

        let (assign81720_e124335, assign81720_e124335_d_n0, assign81720_e124335_d_n2, assign81720_e124335_d_n4, assign81720_e124335_d_n5, assign81720_e124335_d_n6, assign81720_e124335_d_n7, assign81720_e124335_d_n8, assign81720_e124335_d_n9, assign81720_e124335_d_n10, assign81720_e124335_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81720_e124329: f64 = (8.0 * locals.var_ac41);
        let assign81720_e124331: f64 = (assign81720_e124329 * locals.var_ac41);
        let assign81720_e124333: f64 = (assign81720_e124331 * locals.var_ac41);
        (assign81720_e124333, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign81720_e124329 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign81720_e124331 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign81720_e124335;
        locals.var_ac4_dn0 = assign81720_e124335_d_n0;
        locals.var_ac4_dn2 = assign81720_e124335_d_n2;
        locals.var_ac4_dn4 = assign81720_e124335_d_n4;
        locals.var_ac4_dn5 = assign81720_e124335_d_n5;
        locals.var_ac4_dn6 = assign81720_e124335_d_n6;
        locals.var_ac4_dn7 = assign81720_e124335_d_n7;
        locals.var_ac4_dn8 = assign81720_e124335_d_n8;
        locals.var_ac4_dn9 = assign81720_e124335_d_n9;
        locals.var_ac4_dn10 = assign81720_e124335_d_n10;
        locals.var_ac4_dn13 = assign81720_e124335_d_n13;

        let (assign81730_e124351, assign81730_e124351_d_n0, assign81730_e124351_d_n2, assign81730_e124351_d_n4, assign81730_e124351_d_n5, assign81730_e124351_d_n6, assign81730_e124351_d_n7, assign81730_e124351_d_n8, assign81730_e124351_d_n9, assign81730_e124351_d_n10, assign81730_e124351_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81730_e124341: f64 = (7.0 * 1.414213562373095);
        let assign81730_e124344: f64 = (9.0 * locals.var_ty);
        let assign81730_e124347: f64 = (locals.var_tx - 2.0);
        let assign81730_e124348: f64 = (assign81730_e124344 * assign81730_e124347);
        let assign81730_e124349: f64 = (assign81730_e124341 - assign81730_e124348);
        (assign81730_e124349, (-(((9.0 * locals.var_ty_dn0) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign81730_e124347) + (assign81730_e124344 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign81730_e124351;
        locals.var_ac31_dn0 = assign81730_e124351_d_n0;
        locals.var_ac31_dn2 = assign81730_e124351_d_n2;
        locals.var_ac31_dn4 = assign81730_e124351_d_n4;
        locals.var_ac31_dn5 = assign81730_e124351_d_n5;
        locals.var_ac31_dn6 = assign81730_e124351_d_n6;
        locals.var_ac31_dn7 = assign81730_e124351_d_n7;
        locals.var_ac31_dn8 = assign81730_e124351_d_n8;
        locals.var_ac31_dn9 = assign81730_e124351_d_n9;
        locals.var_ac31_dn10 = assign81730_e124351_d_n10;
        locals.var_ac31_dn13 = assign81730_e124351_d_n13;

        let (assign81740_e124359, assign81740_e124359_d_n0, assign81740_e124359_d_n2, assign81740_e124359_d_n4, assign81740_e124359_d_n5, assign81740_e124359_d_n6, assign81740_e124359_d_n7, assign81740_e124359_d_n8, assign81740_e124359_d_n9, assign81740_e124359_d_n10, assign81740_e124359_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81740_e124357: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign81740_e124357, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign81740_e124359;
        locals.var_ac3_dn0 = assign81740_e124359_d_n0;
        locals.var_ac3_dn2 = assign81740_e124359_d_n2;
        locals.var_ac3_dn4 = assign81740_e124359_d_n4;
        locals.var_ac3_dn5 = assign81740_e124359_d_n5;
        locals.var_ac3_dn6 = assign81740_e124359_d_n6;
        locals.var_ac3_dn7 = assign81740_e124359_d_n7;
        locals.var_ac3_dn8 = assign81740_e124359_d_n8;
        locals.var_ac3_dn9 = assign81740_e124359_d_n9;
        locals.var_ac3_dn10 = assign81740_e124359_d_n10;
        locals.var_ac3_dn13 = assign81740_e124359_d_n13;

        let assign81750_e124363: f64 = (locals.var_ac3 * 1e-8);
        let assign81750_e124364: f64 = if locals.var_ac4 < assign81750_e124363 { 1.0 } else { 0.0 };
        locals.var_guard1908 = assign81750_e124364;

        let (assign81770_e124385, assign81770_e124385_d_n0, assign81770_e124385_d_n2, assign81770_e124385_d_n4, assign81770_e124385_d_n5, assign81770_e124385_d_n6, assign81770_e124385_d_n7, assign81770_e124385_d_n8, assign81770_e124385_d_n9, assign81770_e124385_d_n10, assign81770_e124385_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 != 0.0)) {
        let assign81770_e124381: f64 = (0.5 * locals.var_ac4);
        let assign81770_e124383: f64 = (assign81770_e124381 / locals.var_ac31);
        (assign81770_e124383, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign81770_e124381 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign81770_e124385;
        locals.var_ac1_dn0 = assign81770_e124385_d_n0;
        locals.var_ac1_dn2 = assign81770_e124385_d_n2;
        locals.var_ac1_dn4 = assign81770_e124385_d_n4;
        locals.var_ac1_dn5 = assign81770_e124385_d_n5;
        locals.var_ac1_dn6 = assign81770_e124385_d_n6;
        locals.var_ac1_dn7 = assign81770_e124385_d_n7;
        locals.var_ac1_dn8 = assign81770_e124385_d_n8;
        locals.var_ac1_dn9 = assign81770_e124385_d_n9;
        locals.var_ac1_dn10 = assign81770_e124385_d_n10;
        locals.var_ac1_dn13 = assign81770_e124385_d_n13;

        let (assign81780_e124397, assign81780_e124397_d_n0, assign81780_e124397_d_n2, assign81780_e124397_d_n4, assign81780_e124397_d_n5, assign81780_e124397_d_n6, assign81780_e124397_d_n7, assign81780_e124397_d_n8, assign81780_e124397_d_n9, assign81780_e124397_d_n10, assign81780_e124397_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 == 0.0)) {
        let assign81780_e124394: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign81780_e124395: f64 = (assign81780_e124394).sqrt();
        (assign81780_e124395, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign81780_e124395)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign81780_e124395)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign81780_e124397;
        locals.var_ac2_dn0 = assign81780_e124397_d_n0;
        locals.var_ac2_dn2 = assign81780_e124397_d_n2;
        locals.var_ac2_dn4 = assign81780_e124397_d_n4;
        locals.var_ac2_dn5 = assign81780_e124397_d_n5;
        locals.var_ac2_dn6 = assign81780_e124397_d_n6;
        locals.var_ac2_dn7 = assign81780_e124397_d_n7;
        locals.var_ac2_dn8 = assign81780_e124397_d_n8;
        locals.var_ac2_dn9 = assign81780_e124397_d_n9;
        locals.var_ac2_dn10 = assign81780_e124397_d_n10;
        locals.var_ac2_dn13 = assign81780_e124397_d_n13;

        let (assign81790_e124409, assign81790_e124409_d_n0, assign81790_e124409_d_n2, assign81790_e124409_d_n4, assign81790_e124409_d_n5, assign81790_e124409_d_n6, assign81790_e124409_d_n7, assign81790_e124409_d_n8, assign81790_e124409_d_n9, assign81790_e124409_d_n10, assign81790_e124409_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) && (locals.var_guard1908 == 0.0)) {
        let assign81790_e124405: f64 = (-locals.var_ac31);
        let assign81790_e124407: f64 = (assign81790_e124405 + locals.var_ac2);
        (assign81790_e124407, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign81790_e124409;
        locals.var_ac1_dn0 = assign81790_e124409_d_n0;
        locals.var_ac1_dn2 = assign81790_e124409_d_n2;
        locals.var_ac1_dn4 = assign81790_e124409_d_n4;
        locals.var_ac1_dn5 = assign81790_e124409_d_n5;
        locals.var_ac1_dn6 = assign81790_e124409_d_n6;
        locals.var_ac1_dn7 = assign81790_e124409_d_n7;
        locals.var_ac1_dn8 = assign81790_e124409_d_n8;
        locals.var_ac1_dn9 = assign81790_e124409_d_n9;
        locals.var_ac1_dn10 = assign81790_e124409_d_n10;
        locals.var_ac1_dn13 = assign81790_e124409_d_n13;

        let (assign81800_e124417, assign81800_e124417_d_n0, assign81800_e124417_d_n2, assign81800_e124417_d_n4, assign81800_e124417_d_n5, assign81800_e124417_d_n6, assign81800_e124417_d_n7, assign81800_e124417_d_n8, assign81800_e124417_d_n9, assign81800_e124417_d_n10, assign81800_e124417_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81800_e124415: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign81800_e124415, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign81800_e124415 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign81800_e124417;
        locals.var_acd_dn0 = assign81800_e124417_d_n0;
        locals.var_acd_dn2 = assign81800_e124417_d_n2;
        locals.var_acd_dn4 = assign81800_e124417_d_n4;
        locals.var_acd_dn5 = assign81800_e124417_d_n5;
        locals.var_acd_dn6 = assign81800_e124417_d_n6;
        locals.var_acd_dn7 = assign81800_e124417_d_n7;
        locals.var_acd_dn8 = assign81800_e124417_d_n8;
        locals.var_acd_dn9 = assign81800_e124417_d_n9;
        locals.var_acd_dn10 = assign81800_e124417_d_n10;
        locals.var_acd_dn13 = assign81800_e124417_d_n13;

        let (assign81810_e124440, assign81810_e124440_d_n0, assign81810_e124440_d_n2, assign81810_e124440_d_n4, assign81810_e124440_d_n5, assign81810_e124440_d_n6, assign81810_e124440_d_n7, assign81810_e124440_d_n8, assign81810_e124440_d_n9, assign81810_e124440_d_n10, assign81810_e124440_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81810_e124422: f64 = (-4.0);
        let assign81810_e124424: f64 = (assign81810_e124422 * 1.414213562373095);
        let assign81810_e124427: f64 = (12.0 * locals.var_ty);
        let assign81810_e124428: f64 = (assign81810_e124424 - assign81810_e124427);
        let assign81810_e124431: f64 = (2.0 * locals.var_acd);
        let assign81810_e124432: f64 = (assign81810_e124428 + assign81810_e124431);
        let assign81810_e124435: f64 = (1.414213562373095 * locals.var_acd);
        let assign81810_e124437: f64 = (assign81810_e124435 * locals.var_acd);
        let assign81810_e124438: f64 = (assign81810_e124432 + assign81810_e124437);
        (assign81810_e124438, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign81810_e124435 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign81810_e124440;
        locals.var_acn_dn0 = assign81810_e124440_d_n0;
        locals.var_acn_dn2 = assign81810_e124440_d_n2;
        locals.var_acn_dn4 = assign81810_e124440_d_n4;
        locals.var_acn_dn5 = assign81810_e124440_d_n5;
        locals.var_acn_dn6 = assign81810_e124440_d_n6;
        locals.var_acn_dn7 = assign81810_e124440_d_n7;
        locals.var_acn_dn8 = assign81810_e124440_d_n8;
        locals.var_acn_dn9 = assign81810_e124440_d_n9;
        locals.var_acn_dn10 = assign81810_e124440_d_n10;
        locals.var_acn_dn13 = assign81810_e124440_d_n13;

        let (assign81820_e124448, assign81820_e124448_d_n0, assign81820_e124448_d_n2, assign81820_e124448_d_n4, assign81820_e124448_d_n5, assign81820_e124448_d_n6, assign81820_e124448_d_n7, assign81820_e124448_d_n8, assign81820_e124448_d_n9, assign81820_e124448_d_n10, assign81820_e124448_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81820_e124446: f64 = (locals.var_acn / locals.var_acd);
        (assign81820_e124446, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn13 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn13)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign81820_e124448;
        locals.var_chi_dn0 = assign81820_e124448_d_n0;
        locals.var_chi_dn2 = assign81820_e124448_d_n2;
        locals.var_chi_dn4 = assign81820_e124448_d_n4;
        locals.var_chi_dn5 = assign81820_e124448_d_n5;
        locals.var_chi_dn6 = assign81820_e124448_d_n6;
        locals.var_chi_dn7 = assign81820_e124448_d_n7;
        locals.var_chi_dn8 = assign81820_e124448_d_n8;
        locals.var_chi_dn9 = assign81820_e124448_d_n9;
        locals.var_chi_dn10 = assign81820_e124448_d_n10;
        locals.var_chi_dn13 = assign81820_e124448_d_n13;

        let (assign81830_e124456, assign81830_e124456_d_n0, assign81830_e124456_d_n2, assign81830_e124456_d_n4, assign81830_e124456_d_n5, assign81830_e124456_d_n6, assign81830_e124456_d_n7, assign81830_e124456_d_n8, assign81830_e124456_d_n9, assign81830_e124456_d_n10, assign81830_e124456_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81830_e124454: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign81830_e124454, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign81830_e124456;
        locals.var_t1_dn0 = assign81830_e124456_d_n0;
        locals.var_t1_dn2 = assign81830_e124456_d_n2;
        locals.var_t1_dn4 = assign81830_e124456_d_n4;
        locals.var_t1_dn5 = assign81830_e124456_d_n5;
        locals.var_t1_dn6 = assign81830_e124456_d_n6;
        locals.var_t1_dn7 = assign81830_e124456_d_n7;
        locals.var_t1_dn8 = assign81830_e124456_d_n8;
        locals.var_t1_dn9 = assign81830_e124456_d_n9;
        locals.var_t1_dn10 = assign81830_e124456_d_n10;
        locals.var_t1_dn13 = assign81830_e124456_d_n13;

        let (assign81840_e124464, assign81840_e124464_d_n0, assign81840_e124464_d_n2, assign81840_e124464_d_n4, assign81840_e124464_d_n5, assign81840_e124464_d_n6, assign81840_e124464_d_n7, assign81840_e124464_d_n8, assign81840_e124464_d_n9, assign81840_e124464_d_n10, assign81840_e124464_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81840_e124462: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign81840_e124462, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign81840_e124464;
        locals.var_t2_dn0 = assign81840_e124464_d_n0;
        locals.var_t2_dn2 = assign81840_e124464_d_n2;
        locals.var_t2_dn4 = assign81840_e124464_d_n4;
        locals.var_t2_dn5 = assign81840_e124464_d_n5;
        locals.var_t2_dn6 = assign81840_e124464_d_n6;
        locals.var_t2_dn7 = assign81840_e124464_d_n7;
        locals.var_t2_dn8 = assign81840_e124464_d_n8;
        locals.var_t2_dn9 = assign81840_e124464_d_n9;
        locals.var_t2_dn10 = assign81840_e124464_d_n10;
        locals.var_t2_dn13 = assign81840_e124464_d_n13;

        let (assign81850_e124475, assign81850_e124475_d_n0, assign81850_e124475_d_n2, assign81850_e124475_d_n4, assign81850_e124475_d_n5, assign81850_e124475_d_n6, assign81850_e124475_d_n7, assign81850_e124475_d_n8, assign81850_e124475_d_n9, assign81850_e124475_d_n10, assign81850_e124475_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81850_e124471: f64 = (locals.var_t2 * locals.var_t2);
        let assign81850_e124472: f64 = (1.0 + assign81850_e124471);
        let assign81850_e124473: f64 = (assign81850_e124472).sqrt();
        (assign81850_e124473, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign81850_e124473)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign81850_e124473)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign81850_e124475;
        locals.var_t3_dn0 = assign81850_e124475_d_n0;
        locals.var_t3_dn2 = assign81850_e124475_d_n2;
        locals.var_t3_dn4 = assign81850_e124475_d_n4;
        locals.var_t3_dn5 = assign81850_e124475_d_n5;
        locals.var_t3_dn6 = assign81850_e124475_d_n6;
        locals.var_t3_dn7 = assign81850_e124475_d_n7;
        locals.var_t3_dn8 = assign81850_e124475_d_n8;
        locals.var_t3_dn9 = assign81850_e124475_d_n9;
        locals.var_t3_dn10 = assign81850_e124475_d_n10;
        locals.var_t3_dn13 = assign81850_e124475_d_n13;

        let (assign81860_e124485, assign81860_e124485_d_n0, assign81860_e124485_d_n2, assign81860_e124485_d_n4, assign81860_e124485_d_n5, assign81860_e124485_d_n6, assign81860_e124485_d_n7, assign81860_e124485_d_n8, assign81860_e124485_d_n9, assign81860_e124485_d_n10, assign81860_e124485_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81860_e124481: f64 = (locals.var_t1 / locals.var_t3);
        let assign81860_e124483: f64 = (assign81860_e124481 - locals.var_vxbgmtcl);
        (assign81860_e124483, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign81860_e124485;
        locals.var_ps0ld_dn0 = assign81860_e124485_d_n0;
        locals.var_ps0ld_dn2 = assign81860_e124485_d_n2;
        locals.var_ps0ld_dn4 = assign81860_e124485_d_n4;
        locals.var_ps0ld_dn5 = assign81860_e124485_d_n5;
        locals.var_ps0ld_dn6 = assign81860_e124485_d_n6;
        locals.var_ps0ld_dn7 = assign81860_e124485_d_n7;
        locals.var_ps0ld_dn8 = assign81860_e124485_d_n8;
        locals.var_ps0ld_dn9 = assign81860_e124485_d_n9;
        locals.var_ps0ld_dn10 = assign81860_e124485_d_n10;
        locals.var_ps0ld_dn13 = assign81860_e124485_d_n13;

        let (assign81870_e124493, assign81870_e124493_d_n0, assign81870_e124493_d_n2, assign81870_e124493_d_n4, assign81870_e124493_d_n5, assign81870_e124493_d_n6, assign81870_e124493_d_n7, assign81870_e124493_d_n8, assign81870_e124493_d_n9, assign81870_e124493_d_n10, assign81870_e124493_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81870_e124491: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign81870_e124491, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign81870_e124493;
        locals.var_t2_dn0 = assign81870_e124493_d_n0;
        locals.var_t2_dn2 = assign81870_e124493_d_n2;
        locals.var_t2_dn4 = assign81870_e124493_d_n4;
        locals.var_t2_dn5 = assign81870_e124493_d_n5;
        locals.var_t2_dn6 = assign81870_e124493_d_n6;
        locals.var_t2_dn7 = assign81870_e124493_d_n7;
        locals.var_t2_dn8 = assign81870_e124493_d_n8;
        locals.var_t2_dn9 = assign81870_e124493_d_n9;
        locals.var_t2_dn10 = assign81870_e124493_d_n10;
        locals.var_t2_dn13 = assign81870_e124493_d_n13;

        let (assign81880_e124501, assign81880_e124501_d_n0, assign81880_e124501_d_n2, assign81880_e124501_d_n4, assign81880_e124501_d_n5, assign81880_e124501_d_n6, assign81880_e124501_d_n7, assign81880_e124501_d_n8, assign81880_e124501_d_n9, assign81880_e124501_d_n10, assign81880_e124501_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        let assign81880_e124499: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign81880_e124499, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign81880_e124501;
        locals.var_qsuld_dn0 = assign81880_e124501_d_n0;
        locals.var_qsuld_dn2 = assign81880_e124501_d_n2;
        locals.var_qsuld_dn4 = assign81880_e124501_d_n4;
        locals.var_qsuld_dn5 = assign81880_e124501_d_n5;
        locals.var_qsuld_dn6 = assign81880_e124501_d_n6;
        locals.var_qsuld_dn7 = assign81880_e124501_d_n7;
        locals.var_qsuld_dn8 = assign81880_e124501_d_n8;
        locals.var_qsuld_dn9 = assign81880_e124501_d_n9;
        locals.var_qsuld_dn10 = assign81880_e124501_d_n10;
        locals.var_qsuld_dn13 = assign81880_e124501_d_n13;

        let (assign81890_e124507, assign81890_e124507_d_n0, assign81890_e124507_d_n2, assign81890_e124507_d_n4, assign81890_e124507_d_n5, assign81890_e124507_d_n6, assign81890_e124507_d_n7, assign81890_e124507_d_n8, assign81890_e124507_d_n9, assign81890_e124507_d_n10, assign81890_e124507_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign81890_e124507;
        locals.var_qbuld_dn0 = assign81890_e124507_d_n0;
        locals.var_qbuld_dn2 = assign81890_e124507_d_n2;
        locals.var_qbuld_dn4 = assign81890_e124507_d_n4;
        locals.var_qbuld_dn5 = assign81890_e124507_d_n5;
        locals.var_qbuld_dn6 = assign81890_e124507_d_n6;
        locals.var_qbuld_dn7 = assign81890_e124507_d_n7;
        locals.var_qbuld_dn8 = assign81890_e124507_d_n8;
        locals.var_qbuld_dn9 = assign81890_e124507_d_n9;
        locals.var_qbuld_dn10 = assign81890_e124507_d_n10;
        locals.var_qbuld_dn13 = assign81890_e124507_d_n13;

    }

    pub(super) fn stamp_transient_block_285(
        locals: &mut StampLocals,
    ) {
        let (assign81900_e124513, assign81900_e124513_d_n0, assign81900_e124513_d_n2, assign81900_e124513_d_n4, assign81900_e124513_d_n5, assign81900_e124513_d_n6, assign81900_e124513_d_n7, assign81900_e124513_d_n8, assign81900_e124513_d_n9, assign81900_e124513_d_n10, assign81900_e124513_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1890 = assign81900_e124513;
        locals.var_ps0ld_ini__blk1890_dn0 = assign81900_e124513_d_n0;
        locals.var_ps0ld_ini__blk1890_dn2 = assign81900_e124513_d_n2;
        locals.var_ps0ld_ini__blk1890_dn4 = assign81900_e124513_d_n4;
        locals.var_ps0ld_ini__blk1890_dn5 = assign81900_e124513_d_n5;
        locals.var_ps0ld_ini__blk1890_dn6 = assign81900_e124513_d_n6;
        locals.var_ps0ld_ini__blk1890_dn7 = assign81900_e124513_d_n7;
        locals.var_ps0ld_ini__blk1890_dn8 = assign81900_e124513_d_n8;
        locals.var_ps0ld_ini__blk1890_dn9 = assign81900_e124513_d_n9;
        locals.var_ps0ld_ini__blk1890_dn10 = assign81900_e124513_d_n10;
        locals.var_ps0ld_ini__blk1890_dn13 = assign81900_e124513_d_n13;

        let assign81910_e124517: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81910_e124518: f64 = (locals.var_beta * assign81910_e124517);
        let assign81910_e124522: f64 = (10.0 * 2.220446049250313e-16);
        let assign81910_e124524: f64 = (assign81910_e124522 - 1.0);
        let assign81910_e124526: f64 = (assign81910_e124524 * locals.var_fac1p2);
        let assign81910_e124528: f64 = (assign81910_e124526 * locals.var_beta2);
        let assign81910_e124530: f64 = (assign81910_e124528 / 4.0);
        let assign81910_e124531: f64 = (1.0 + assign81910_e124530);
        let assign81910_e124532: f64 = if assign81910_e124518 < assign81910_e124531 { 1.0 } else { 0.0 };
        locals.var_guard1909 = assign81910_e124532;

        let (assign81920_e124547, assign81920_e124547_d_n0, assign81920_e124547_d_n2, assign81920_e124547_d_n4, assign81920_e124547_d_n5, assign81920_e124547_d_n6, assign81920_e124547_d_n7, assign81920_e124547_d_n8, assign81920_e124547_d_n9, assign81920_e124547_d_n10, assign81920_e124547_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 != 0.0)) {
        let assign81920_e124542: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81920_e124544: f64 = (assign81920_e124542 / 2.0);
        let assign81920_e124545: f64 = (locals.var_vgpld + assign81920_e124544);
        (assign81920_e124545, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (locals.var_vgpld_dn6 + (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0)), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign81920_e124547;
        locals.var_ps0_inia_dn0 = assign81920_e124547_d_n0;
        locals.var_ps0_inia_dn2 = assign81920_e124547_d_n2;
        locals.var_ps0_inia_dn4 = assign81920_e124547_d_n4;
        locals.var_ps0_inia_dn5 = assign81920_e124547_d_n5;
        locals.var_ps0_inia_dn6 = assign81920_e124547_d_n6;
        locals.var_ps0_inia_dn7 = assign81920_e124547_d_n7;
        locals.var_ps0_inia_dn8 = assign81920_e124547_d_n8;
        locals.var_ps0_inia_dn9 = assign81920_e124547_d_n9;
        locals.var_ps0_inia_dn10 = assign81920_e124547_d_n10;
        locals.var_ps0_inia_dn13 = assign81920_e124547_d_n13;

        let (assign81930_e124571, assign81930_e124571_d_n0, assign81930_e124571_d_n2, assign81930_e124571_d_n4, assign81930_e124571_d_n5, assign81930_e124571_d_n6, assign81930_e124571_d_n7, assign81930_e124571_d_n8, assign81930_e124571_d_n9, assign81930_e124571_d_n10, assign81930_e124571_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 == 0.0)) {
        let assign81930_e124560: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81930_e124561: f64 = (locals.var_beta * assign81930_e124560);
        let assign81930_e124563: f64 = (assign81930_e124561 - 1.0);
        let assign81930_e124564: f64 = (4.0 * assign81930_e124563);
        let assign81930_e124567: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign81930_e124568: f64 = (assign81930_e124564 / assign81930_e124567);
        let assign81930_e124569: f64 = (1.0 + assign81930_e124568);
        (assign81930_e124569, ((((4.0 * ((locals.var_beta_dn0 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn2 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn4 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn5 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn6 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn7 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn8 * assign81930_e124560) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn9 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn9))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn10 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign81930_e124567 * assign81930_e124567)), ((((4.0 * ((locals.var_beta_dn13 * assign81930_e124560) + (locals.var_beta * locals.var_vxbgmtcl_dn13))) * assign81930_e124567) - (assign81930_e124564 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign81930_e124567 * assign81930_e124567)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81930_e124571;
        locals.var_tx_dn0 = assign81930_e124571_d_n0;
        locals.var_tx_dn2 = assign81930_e124571_d_n2;
        locals.var_tx_dn4 = assign81930_e124571_d_n4;
        locals.var_tx_dn5 = assign81930_e124571_d_n5;
        locals.var_tx_dn6 = assign81930_e124571_d_n6;
        locals.var_tx_dn7 = assign81930_e124571_d_n7;
        locals.var_tx_dn8 = assign81930_e124571_d_n8;
        locals.var_tx_dn9 = assign81930_e124571_d_n9;
        locals.var_tx_dn10 = assign81930_e124571_d_n10;
        locals.var_tx_dn13 = assign81930_e124571_d_n13;

        let (assign81940_e124592, assign81940_e124592_d_n0, assign81940_e124592_d_n2, assign81940_e124592_d_n4, assign81940_e124592_d_n5, assign81940_e124592_d_n6, assign81940_e124592_d_n7, assign81940_e124592_d_n8, assign81940_e124592_d_n9, assign81940_e124592_d_n10, assign81940_e124592_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1909 == 0.0)) {
        let assign81940_e124582: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81940_e124584: f64 = (assign81940_e124582 / 2.0);
        let assign81940_e124587: f64 = (locals.var_tx).sqrt();
        let assign81940_e124588: f64 = (1.0 - assign81940_e124587);
        let assign81940_e124589: f64 = (assign81940_e124584 * assign81940_e124588);
        let assign81940_e124590: f64 = (locals.var_vgpld + assign81940_e124589);
        (assign81940_e124590, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn0 / (2.0 * assign81940_e124587))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn2 / (2.0 * assign81940_e124587)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn4 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn5 / (2.0 * assign81940_e124587))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn6 / (2.0 * assign81940_e124587)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn7 / (2.0 * assign81940_e124587)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn8 / (2.0 * assign81940_e124587)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn9 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn10 / (2.0 * assign81940_e124587))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign81940_e124588) + (assign81940_e124584 * (-(locals.var_tx_dn13 / (2.0 * assign81940_e124587))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign81940_e124592;
        locals.var_ps0_inia_dn0 = assign81940_e124592_d_n0;
        locals.var_ps0_inia_dn2 = assign81940_e124592_d_n2;
        locals.var_ps0_inia_dn4 = assign81940_e124592_d_n4;
        locals.var_ps0_inia_dn5 = assign81940_e124592_d_n5;
        locals.var_ps0_inia_dn6 = assign81940_e124592_d_n6;
        locals.var_ps0_inia_dn7 = assign81940_e124592_d_n7;
        locals.var_ps0_inia_dn8 = assign81940_e124592_d_n8;
        locals.var_ps0_inia_dn9 = assign81940_e124592_d_n9;
        locals.var_ps0_inia_dn10 = assign81940_e124592_d_n10;
        locals.var_ps0_inia_dn13 = assign81940_e124592_d_n13;

        let (assign81950_e124603, assign81950_e124603_d_n0, assign81950_e124603_d_n2, assign81950_e124603_d_n4, assign81950_e124603_d_n5, assign81950_e124603_d_n6, assign81950_e124603_d_n7, assign81950_e124603_d_n8, assign81950_e124603_d_n9, assign81950_e124603_d_n10, assign81950_e124603_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign81950_e124600: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign81950_e124601: f64 = (locals.var_beta * assign81950_e124600);
        (assign81950_e124601, ((locals.var_beta_dn0 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign81950_e124600) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign81950_e124603;
        locals.var_chi_dn0 = assign81950_e124603_d_n0;
        locals.var_chi_dn2 = assign81950_e124603_d_n2;
        locals.var_chi_dn4 = assign81950_e124603_d_n4;
        locals.var_chi_dn5 = assign81950_e124603_d_n5;
        locals.var_chi_dn6 = assign81950_e124603_d_n6;
        locals.var_chi_dn7 = assign81950_e124603_d_n7;
        locals.var_chi_dn8 = assign81950_e124603_d_n8;
        locals.var_chi_dn9 = assign81950_e124603_d_n9;
        locals.var_chi_dn10 = assign81950_e124603_d_n10;
        locals.var_chi_dn13 = assign81950_e124603_d_n13;

        let assign81960_e124606: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1910 = assign81960_e124606;

        let (assign81980_e124626, assign81980_e124626_d_n0, assign81980_e124626_d_n2, assign81980_e124626_d_n4, assign81980_e124626_d_n5, assign81980_e124626_d_n6, assign81980_e124626_d_n7, assign81980_e124626_d_n8, assign81980_e124626_d_n9, assign81980_e124626_d_n10, assign81980_e124626_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign81980_e124623: f64 = (-locals.var_chi);
        let assign81980_e124624: f64 = (assign81980_e124623).exp();
        (assign81980_e124624, (assign81980_e124624 * (-locals.var_chi_dn0)), (assign81980_e124624 * (-locals.var_chi_dn2)), (assign81980_e124624 * (-locals.var_chi_dn4)), (assign81980_e124624 * (-locals.var_chi_dn5)), (assign81980_e124624 * (-locals.var_chi_dn6)), (assign81980_e124624 * (-locals.var_chi_dn7)), (assign81980_e124624 * (-locals.var_chi_dn8)), (assign81980_e124624 * (-locals.var_chi_dn9)), (assign81980_e124624 * (-locals.var_chi_dn10)), (assign81980_e124624 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign81980_e124626;
        locals.var_ty_dn0 = assign81980_e124626_d_n0;
        locals.var_ty_dn2 = assign81980_e124626_d_n2;
        locals.var_ty_dn4 = assign81980_e124626_d_n4;
        locals.var_ty_dn5 = assign81980_e124626_d_n5;
        locals.var_ty_dn6 = assign81980_e124626_d_n6;
        locals.var_ty_dn7 = assign81980_e124626_d_n7;
        locals.var_ty_dn8 = assign81980_e124626_d_n8;
        locals.var_ty_dn9 = assign81980_e124626_d_n9;
        locals.var_ty_dn10 = assign81980_e124626_d_n10;
        locals.var_ty_dn13 = assign81980_e124626_d_n13;

        let (assign81990_e124651, assign81990_e124651_d_n0, assign81990_e124651_d_n2, assign81990_e124651_d_n4, assign81990_e124651_d_n5, assign81990_e124651_d_n6, assign81990_e124651_d_n7, assign81990_e124651_d_n8, assign81990_e124651_d_n9, assign81990_e124651_d_n10, assign81990_e124651_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign81990_e124638: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81990_e124639: f64 = (locals.var_beta * assign81990_e124638);
        let assign81990_e124641: f64 = (assign81990_e124639 - 1.0);
        let assign81990_e124643: f64 = (assign81990_e124641 + locals.var_ty);
        let assign81990_e124644: f64 = (4.0 * assign81990_e124643);
        let assign81990_e124647: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign81990_e124648: f64 = (assign81990_e124644 / assign81990_e124647);
        let assign81990_e124649: f64 = (1.0 + assign81990_e124648);
        (assign81990_e124649, ((((4.0 * (((locals.var_beta_dn0 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn2 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn4 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn5 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn6 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn7 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn8 * assign81990_e124638) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn9 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn10 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign81990_e124647 * assign81990_e124647)), ((((4.0 * (((locals.var_beta_dn13 * assign81990_e124638) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign81990_e124647) - (assign81990_e124644 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign81990_e124647 * assign81990_e124647)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign81990_e124651;
        locals.var_tx_dn0 = assign81990_e124651_d_n0;
        locals.var_tx_dn2 = assign81990_e124651_d_n2;
        locals.var_tx_dn4 = assign81990_e124651_d_n4;
        locals.var_tx_dn5 = assign81990_e124651_d_n5;
        locals.var_tx_dn6 = assign81990_e124651_d_n6;
        locals.var_tx_dn7 = assign81990_e124651_d_n7;
        locals.var_tx_dn8 = assign81990_e124651_d_n8;
        locals.var_tx_dn9 = assign81990_e124651_d_n9;
        locals.var_tx_dn10 = assign81990_e124651_d_n10;
        locals.var_tx_dn13 = assign81990_e124651_d_n13;

        let (assign82000_e124671, assign82000_e124671_d_n0, assign82000_e124671_d_n2, assign82000_e124671_d_n4, assign82000_e124671_d_n5, assign82000_e124671_d_n6, assign82000_e124671_d_n7, assign82000_e124671_d_n8, assign82000_e124671_d_n9, assign82000_e124671_d_n10, assign82000_e124671_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82000_e124661: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82000_e124663: f64 = (assign82000_e124661 / 2.0);
        let assign82000_e124666: f64 = (locals.var_tx).sqrt();
        let assign82000_e124667: f64 = (1.0 - assign82000_e124666);
        let assign82000_e124668: f64 = (assign82000_e124663 * assign82000_e124667);
        let assign82000_e124669: f64 = (locals.var_vgpld + assign82000_e124668);
        (assign82000_e124669, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn0 / (2.0 * assign82000_e124666))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn2 / (2.0 * assign82000_e124666)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn4 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn5 / (2.0 * assign82000_e124666))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn6 / (2.0 * assign82000_e124666)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn7 / (2.0 * assign82000_e124666)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn8 / (2.0 * assign82000_e124666)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn9 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn10 / (2.0 * assign82000_e124666))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign82000_e124667) + (assign82000_e124663 * (-(locals.var_tx_dn13 / (2.0 * assign82000_e124666))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82000_e124671;
        locals.var_ps0_inia_dn0 = assign82000_e124671_d_n0;
        locals.var_ps0_inia_dn2 = assign82000_e124671_d_n2;
        locals.var_ps0_inia_dn4 = assign82000_e124671_d_n4;
        locals.var_ps0_inia_dn5 = assign82000_e124671_d_n5;
        locals.var_ps0_inia_dn6 = assign82000_e124671_d_n6;
        locals.var_ps0_inia_dn7 = assign82000_e124671_d_n7;
        locals.var_ps0_inia_dn8 = assign82000_e124671_d_n8;
        locals.var_ps0_inia_dn9 = assign82000_e124671_d_n9;
        locals.var_ps0_inia_dn10 = assign82000_e124671_d_n10;
        locals.var_ps0_inia_dn13 = assign82000_e124671_d_n13;

        let (assign82010_e124684, assign82010_e124684_d_n0, assign82010_e124684_d_n2, assign82010_e124684_d_n4, assign82010_e124684_d_n5, assign82010_e124684_d_n6, assign82010_e124684_d_n7, assign82010_e124684_d_n8, assign82010_e124684_d_n9, assign82010_e124684_d_n10, assign82010_e124684_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82010_e124681: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82010_e124682: f64 = (locals.var_beta * assign82010_e124681);
        (assign82010_e124682, ((locals.var_beta_dn0 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign82010_e124681) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82010_e124684;
        locals.var_chi_dn0 = assign82010_e124684_d_n0;
        locals.var_chi_dn2 = assign82010_e124684_d_n2;
        locals.var_chi_dn4 = assign82010_e124684_d_n4;
        locals.var_chi_dn5 = assign82010_e124684_d_n5;
        locals.var_chi_dn6 = assign82010_e124684_d_n6;
        locals.var_chi_dn7 = assign82010_e124684_d_n7;
        locals.var_chi_dn8 = assign82010_e124684_d_n8;
        locals.var_chi_dn9 = assign82010_e124684_d_n9;
        locals.var_chi_dn10 = assign82010_e124684_d_n10;
        locals.var_chi_dn13 = assign82010_e124684_d_n13;

        let (assign82020_e124695, assign82020_e124695_d_n0, assign82020_e124695_d_n2, assign82020_e124695_d_n4, assign82020_e124695_d_n5, assign82020_e124695_d_n6, assign82020_e124695_d_n7, assign82020_e124695_d_n8, assign82020_e124695_d_n9, assign82020_e124695_d_n10, assign82020_e124695_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82020_e124692: f64 = (-locals.var_chi);
        let assign82020_e124693: f64 = (assign82020_e124692).exp();
        (assign82020_e124693, (assign82020_e124693 * (-locals.var_chi_dn0)), (assign82020_e124693 * (-locals.var_chi_dn2)), (assign82020_e124693 * (-locals.var_chi_dn4)), (assign82020_e124693 * (-locals.var_chi_dn5)), (assign82020_e124693 * (-locals.var_chi_dn6)), (assign82020_e124693 * (-locals.var_chi_dn7)), (assign82020_e124693 * (-locals.var_chi_dn8)), (assign82020_e124693 * (-locals.var_chi_dn9)), (assign82020_e124693 * (-locals.var_chi_dn10)), (assign82020_e124693 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign82020_e124695;
        locals.var_ty_dn0 = assign82020_e124695_d_n0;
        locals.var_ty_dn2 = assign82020_e124695_d_n2;
        locals.var_ty_dn4 = assign82020_e124695_d_n4;
        locals.var_ty_dn5 = assign82020_e124695_d_n5;
        locals.var_ty_dn6 = assign82020_e124695_d_n6;
        locals.var_ty_dn7 = assign82020_e124695_d_n7;
        locals.var_ty_dn8 = assign82020_e124695_d_n8;
        locals.var_ty_dn9 = assign82020_e124695_d_n9;
        locals.var_ty_dn10 = assign82020_e124695_d_n10;
        locals.var_ty_dn13 = assign82020_e124695_d_n13;

        let (assign82030_e124720, assign82030_e124720_d_n0, assign82030_e124720_d_n2, assign82030_e124720_d_n4, assign82030_e124720_d_n5, assign82030_e124720_d_n6, assign82030_e124720_d_n7, assign82030_e124720_d_n8, assign82030_e124720_d_n9, assign82030_e124720_d_n10, assign82030_e124720_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82030_e124707: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82030_e124708: f64 = (locals.var_beta * assign82030_e124707);
        let assign82030_e124710: f64 = (assign82030_e124708 - 1.0);
        let assign82030_e124712: f64 = (assign82030_e124710 + locals.var_ty);
        let assign82030_e124713: f64 = (4.0 * assign82030_e124712);
        let assign82030_e124716: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign82030_e124717: f64 = (assign82030_e124713 / assign82030_e124716);
        let assign82030_e124718: f64 = (1.0 + assign82030_e124717);
        (assign82030_e124718, ((((4.0 * (((locals.var_beta_dn0 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn2 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn4 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn5 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn6 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn7 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn8 * assign82030_e124707) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn9 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn10 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign82030_e124716 * assign82030_e124716)), ((((4.0 * (((locals.var_beta_dn13 * assign82030_e124707) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign82030_e124716) - (assign82030_e124713 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign82030_e124716 * assign82030_e124716)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign82030_e124720;
        locals.var_tx_dn0 = assign82030_e124720_d_n0;
        locals.var_tx_dn2 = assign82030_e124720_d_n2;
        locals.var_tx_dn4 = assign82030_e124720_d_n4;
        locals.var_tx_dn5 = assign82030_e124720_d_n5;
        locals.var_tx_dn6 = assign82030_e124720_d_n6;
        locals.var_tx_dn7 = assign82030_e124720_d_n7;
        locals.var_tx_dn8 = assign82030_e124720_d_n8;
        locals.var_tx_dn9 = assign82030_e124720_d_n9;
        locals.var_tx_dn10 = assign82030_e124720_d_n10;
        locals.var_tx_dn13 = assign82030_e124720_d_n13;

        let (assign82040_e124740, assign82040_e124740_d_n0, assign82040_e124740_d_n2, assign82040_e124740_d_n4, assign82040_e124740_d_n5, assign82040_e124740_d_n6, assign82040_e124740_d_n7, assign82040_e124740_d_n8, assign82040_e124740_d_n9, assign82040_e124740_d_n10, assign82040_e124740_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82040_e124730: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82040_e124732: f64 = (assign82040_e124730 / 2.0);
        let assign82040_e124735: f64 = (locals.var_tx).sqrt();
        let assign82040_e124736: f64 = (1.0 - assign82040_e124735);
        let assign82040_e124737: f64 = (assign82040_e124732 * assign82040_e124736);
        let assign82040_e124738: f64 = (locals.var_vgpld + assign82040_e124737);
        (assign82040_e124738, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn0 / (2.0 * assign82040_e124735))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn2 / (2.0 * assign82040_e124735)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn4 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn5 / (2.0 * assign82040_e124735))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn6 / (2.0 * assign82040_e124735)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn7 / (2.0 * assign82040_e124735)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn8 / (2.0 * assign82040_e124735)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn9 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn10 / (2.0 * assign82040_e124735))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign82040_e124736) + (assign82040_e124732 * (-(locals.var_tx_dn13 / (2.0 * assign82040_e124735))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82040_e124740;
        locals.var_ps0_inia_dn0 = assign82040_e124740_d_n0;
        locals.var_ps0_inia_dn2 = assign82040_e124740_d_n2;
        locals.var_ps0_inia_dn4 = assign82040_e124740_d_n4;
        locals.var_ps0_inia_dn5 = assign82040_e124740_d_n5;
        locals.var_ps0_inia_dn6 = assign82040_e124740_d_n6;
        locals.var_ps0_inia_dn7 = assign82040_e124740_d_n7;
        locals.var_ps0_inia_dn8 = assign82040_e124740_d_n8;
        locals.var_ps0_inia_dn9 = assign82040_e124740_d_n9;
        locals.var_ps0_inia_dn10 = assign82040_e124740_d_n10;
        locals.var_ps0_inia_dn13 = assign82040_e124740_d_n13;

        let (assign82050_e124753, assign82050_e124753_d_n0, assign82050_e124753_d_n2, assign82050_e124753_d_n4, assign82050_e124753_d_n5, assign82050_e124753_d_n6, assign82050_e124753_d_n7, assign82050_e124753_d_n8, assign82050_e124753_d_n9, assign82050_e124753_d_n10, assign82050_e124753_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign82050_e124750: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82050_e124751: f64 = (locals.var_beta * assign82050_e124750);
        (assign82050_e124751, ((locals.var_beta_dn0 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign82050_e124750) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82050_e124753;
        locals.var_chi_dn0 = assign82050_e124753_d_n0;
        locals.var_chi_dn2 = assign82050_e124753_d_n2;
        locals.var_chi_dn4 = assign82050_e124753_d_n4;
        locals.var_chi_dn5 = assign82050_e124753_d_n5;
        locals.var_chi_dn6 = assign82050_e124753_d_n6;
        locals.var_chi_dn7 = assign82050_e124753_d_n7;
        locals.var_chi_dn8 = assign82050_e124753_d_n8;
        locals.var_chi_dn9 = assign82050_e124753_d_n9;
        locals.var_chi_dn10 = assign82050_e124753_d_n10;
        locals.var_chi_dn13 = assign82050_e124753_d_n13;

        let (assign82070_e124795,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82070_e124774: f64 = (2.0_f64).sqrt();
        let assign82070_e124775: f64 = (9.0 * assign82070_e124774);
        let assign82070_e124776: f64 = (1.0 / assign82070_e124775);
        let assign82070_e124780: f64 = (-3.0);
        let assign82070_e124781: f64 = (assign82070_e124780).exp();
        let assign82070_e124782: f64 = (7.0 * assign82070_e124781);
        let assign82070_e124783: f64 = (5.0 + assign82070_e124782);
        let assign82070_e124787: f64 = (-3.0);
        let assign82070_e124788: f64 = (assign82070_e124787).exp();
        let assign82070_e124789: f64 = (2.0 + assign82070_e124788);
        let assign82070_e124790: f64 = (assign82070_e124789).sqrt();
        let assign82070_e124791: f64 = (54.0 * assign82070_e124790);
        let assign82070_e124792: f64 = (assign82070_e124783 / assign82070_e124791);
        let assign82070_e124793: f64 = (assign82070_e124776 - assign82070_e124792);
        (assign82070_e124793,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign82070_e124795;

        let (assign82080_e124823,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82080_e124805: f64 = (-3.0);
        let assign82080_e124806: f64 = (assign82080_e124805).exp();
        let assign82080_e124807: f64 = (1.0 + assign82080_e124806);
        let assign82080_e124811: f64 = (-3.0);
        let assign82080_e124812: f64 = (assign82080_e124811).exp();
        let assign82080_e124813: f64 = (2.0 + assign82080_e124812);
        let assign82080_e124814: f64 = (assign82080_e124813).sqrt();
        let assign82080_e124815: f64 = (2.0 * assign82080_e124814);
        let assign82080_e124816: f64 = (assign82080_e124807 / assign82080_e124815);
        let assign82080_e124818: f64 = (2.0_f64).sqrt();
        let assign82080_e124820: f64 = (assign82080_e124818 / 3.0);
        let assign82080_e124821: f64 = (assign82080_e124816 - assign82080_e124820);
        (assign82080_e124821,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign82080_e124823;

        let (assign82090_e124842, assign82090_e124842_d_n0, assign82090_e124842_d_n2, assign82090_e124842_d_n4, assign82090_e124842_d_n5, assign82090_e124842_d_n6, assign82090_e124842_d_n7, assign82090_e124842_d_n8, assign82090_e124842_d_n9, assign82090_e124842_d_n10, assign82090_e124842_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82090_e124833: f64 = (2.0_f64).sqrt();
        let assign82090_e124834: f64 = (1.0 / assign82090_e124833);
        let assign82090_e124838: f64 = (locals.var_beta * locals.var_fac1);
        let assign82090_e124839: f64 = (1.0 / assign82090_e124838);
        let assign82090_e124840: f64 = (assign82090_e124834 + assign82090_e124839);
        (assign82090_e124840, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign82090_e124838 * assign82090_e124838))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign82090_e124838 * assign82090_e124838))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign82090_e124842;
        locals.var_tc_dn0 = assign82090_e124842_d_n0;
        locals.var_tc_dn2 = assign82090_e124842_d_n2;
        locals.var_tc_dn4 = assign82090_e124842_d_n4;
        locals.var_tc_dn5 = assign82090_e124842_d_n5;
        locals.var_tc_dn6 = assign82090_e124842_d_n6;
        locals.var_tc_dn7 = assign82090_e124842_d_n7;
        locals.var_tc_dn8 = assign82090_e124842_d_n8;
        locals.var_tc_dn9 = assign82090_e124842_d_n9;
        locals.var_tc_dn10 = assign82090_e124842_d_n10;
        locals.var_tc_dn13 = assign82090_e124842_d_n13;

        let (assign82100_e124857, assign82100_e124857_d_n0, assign82100_e124857_d_n2, assign82100_e124857_d_n4, assign82100_e124857_d_n5, assign82100_e124857_d_n6, assign82100_e124857_d_n7, assign82100_e124857_d_n8, assign82100_e124857_d_n9, assign82100_e124857_d_n10, assign82100_e124857_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82100_e124852: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82100_e124853: f64 = (-assign82100_e124852);
        let assign82100_e124855: f64 = (assign82100_e124853 / locals.var_fac1);
        (assign82100_e124855, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign82100_e124853 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign82100_e124857;
        locals.var_td_dn0 = assign82100_e124857_d_n0;
        locals.var_td_dn2 = assign82100_e124857_d_n2;
        locals.var_td_dn4 = assign82100_e124857_d_n4;
        locals.var_td_dn5 = assign82100_e124857_d_n5;
        locals.var_td_dn6 = assign82100_e124857_d_n6;
        locals.var_td_dn7 = assign82100_e124857_d_n7;
        locals.var_td_dn8 = assign82100_e124857_d_n8;
        locals.var_td_dn9 = assign82100_e124857_d_n9;
        locals.var_td_dn10 = assign82100_e124857_d_n10;
        locals.var_td_dn13 = assign82100_e124857_d_n13;

        let (assign82110_e124895, assign82110_e124895_d_n0, assign82110_e124895_d_n2, assign82110_e124895_d_n4, assign82110_e124895_d_n5, assign82110_e124895_d_n6, assign82110_e124895_d_n7, assign82110_e124895_d_n8, assign82110_e124895_d_n9, assign82110_e124895_d_n10, assign82110_e124895_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82110_e124867: f64 = (locals.var_tb * locals.var_tb);
        let assign82110_e124869: f64 = (assign82110_e124867 * locals.var_tb);
        let assign82110_e124872: f64 = (27.0 * locals.var_ta);
        let assign82110_e124874: f64 = (assign82110_e124872 * locals.var_ta);
        let assign82110_e124876: f64 = (assign82110_e124874 * locals.var_ta);
        let assign82110_e124877: f64 = (assign82110_e124869 / assign82110_e124876);
        let assign82110_e124880: f64 = (locals.var_tb * locals.var_tc);
        let assign82110_e124883: f64 = (6.0 * locals.var_ta);
        let assign82110_e124885: f64 = (assign82110_e124883 * locals.var_ta);
        let assign82110_e124886: f64 = (assign82110_e124880 / assign82110_e124885);
        let assign82110_e124887: f64 = (assign82110_e124877 - assign82110_e124886);
        let assign82110_e124891: f64 = (2.0 * locals.var_ta);
        let assign82110_e124892: f64 = (locals.var_td / assign82110_e124891);
        let assign82110_e124893: f64 = (assign82110_e124887 + assign82110_e124892);
        (assign82110_e124893, ((-((locals.var_tb * locals.var_tc_dn0) / assign82110_e124885)) + (locals.var_td_dn0 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn2) / assign82110_e124885)) + (locals.var_td_dn2 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn4) / assign82110_e124885)) + (locals.var_td_dn4 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn5) / assign82110_e124885)) + (locals.var_td_dn5 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn6) / assign82110_e124885)) + (locals.var_td_dn6 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn7) / assign82110_e124885)) + (locals.var_td_dn7 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn8) / assign82110_e124885)) + (locals.var_td_dn8 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn9) / assign82110_e124885)) + (locals.var_td_dn9 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn10) / assign82110_e124885)) + (locals.var_td_dn10 / assign82110_e124891)), ((-((locals.var_tb * locals.var_tc_dn13) / assign82110_e124885)) + (locals.var_td_dn13 / assign82110_e124891)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign82110_e124895;
        locals.var_tq_dn0 = assign82110_e124895_d_n0;
        locals.var_tq_dn2 = assign82110_e124895_d_n2;
        locals.var_tq_dn4 = assign82110_e124895_d_n4;
        locals.var_tq_dn5 = assign82110_e124895_d_n5;
        locals.var_tq_dn6 = assign82110_e124895_d_n6;
        locals.var_tq_dn7 = assign82110_e124895_d_n7;
        locals.var_tq_dn8 = assign82110_e124895_d_n8;
        locals.var_tq_dn9 = assign82110_e124895_d_n9;
        locals.var_tq_dn10 = assign82110_e124895_d_n10;
        locals.var_tq_dn13 = assign82110_e124895_d_n13;

        let (assign82120_e124919, assign82120_e124919_d_n0, assign82120_e124919_d_n2, assign82120_e124919_d_n4, assign82120_e124919_d_n5, assign82120_e124919_d_n6, assign82120_e124919_d_n7, assign82120_e124919_d_n8, assign82120_e124919_d_n9, assign82120_e124919_d_n10, assign82120_e124919_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82120_e124905: f64 = (3.0 * locals.var_ta);
        let assign82120_e124907: f64 = (assign82120_e124905 * locals.var_tc);
        let assign82120_e124910: f64 = (locals.var_tb * locals.var_tb);
        let assign82120_e124911: f64 = (assign82120_e124907 - assign82120_e124910);
        let assign82120_e124914: f64 = (9.0 * locals.var_ta);
        let assign82120_e124916: f64 = (assign82120_e124914 * locals.var_ta);
        let assign82120_e124917: f64 = (assign82120_e124911 / assign82120_e124916);
        (assign82120_e124917, ((assign82120_e124905 * locals.var_tc_dn0) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn2) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn4) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn5) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn6) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn7) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn8) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn9) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn10) / assign82120_e124916), ((assign82120_e124905 * locals.var_tc_dn13) / assign82120_e124916),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign82120_e124919;
        locals.var_tp_dn0 = assign82120_e124919_d_n0;
        locals.var_tp_dn2 = assign82120_e124919_d_n2;
        locals.var_tp_dn4 = assign82120_e124919_d_n4;
        locals.var_tp_dn5 = assign82120_e124919_d_n5;
        locals.var_tp_dn6 = assign82120_e124919_d_n6;
        locals.var_tp_dn7 = assign82120_e124919_d_n7;
        locals.var_tp_dn8 = assign82120_e124919_d_n8;
        locals.var_tp_dn9 = assign82120_e124919_d_n9;
        locals.var_tp_dn10 = assign82120_e124919_d_n10;
        locals.var_tp_dn13 = assign82120_e124919_d_n13;

        let (assign82130_e124938, assign82130_e124938_d_n0, assign82130_e124938_d_n2, assign82130_e124938_d_n4, assign82130_e124938_d_n5, assign82130_e124938_d_n6, assign82130_e124938_d_n7, assign82130_e124938_d_n8, assign82130_e124938_d_n9, assign82130_e124938_d_n10, assign82130_e124938_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82130_e124929: f64 = (locals.var_tq * locals.var_tq);
        let assign82130_e124932: f64 = (locals.var_tp * locals.var_tp);
        let assign82130_e124934: f64 = (assign82130_e124932 * locals.var_tp);
        let assign82130_e124935: f64 = (assign82130_e124929 + assign82130_e124934);
        let assign82130_e124936: f64 = (assign82130_e124935).sqrt();
        (assign82130_e124936, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn0))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn2))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn4))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn5))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn6))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn7))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn8))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn9))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn10))) / (2.0 * assign82130_e124936)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign82130_e124932 * locals.var_tp_dn13))) / (2.0 * assign82130_e124936)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign82130_e124938;
        locals.var_t5_dn0 = assign82130_e124938_d_n0;
        locals.var_t5_dn2 = assign82130_e124938_d_n2;
        locals.var_t5_dn4 = assign82130_e124938_d_n4;
        locals.var_t5_dn5 = assign82130_e124938_d_n5;
        locals.var_t5_dn6 = assign82130_e124938_d_n6;
        locals.var_t5_dn7 = assign82130_e124938_d_n7;
        locals.var_t5_dn8 = assign82130_e124938_d_n8;
        locals.var_t5_dn9 = assign82130_e124938_d_n9;
        locals.var_t5_dn10 = assign82130_e124938_d_n10;
        locals.var_t5_dn13 = assign82130_e124938_d_n13;

        let (assign82140_e124953, assign82140_e124953_d_n0, assign82140_e124953_d_n2, assign82140_e124953_d_n4, assign82140_e124953_d_n5, assign82140_e124953_d_n6, assign82140_e124953_d_n7, assign82140_e124953_d_n8, assign82140_e124953_d_n9, assign82140_e124953_d_n10, assign82140_e124953_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82140_e124947: f64 = (-locals.var_tq);
        let assign82140_e124949: f64 = (assign82140_e124947 + locals.var_t5);
        let assign82140_e124951: f64 = (assign82140_e124949).powf(0.3333333333333333);
        (assign82140_e124951, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign82140_e124949))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82140_e124949).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign82140_e124951 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign82140_e124949))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign82140_e124953;
        locals.var_tu_dn0 = assign82140_e124953_d_n0;
        locals.var_tu_dn2 = assign82140_e124953_d_n2;
        locals.var_tu_dn4 = assign82140_e124953_d_n4;
        locals.var_tu_dn5 = assign82140_e124953_d_n5;
        locals.var_tu_dn6 = assign82140_e124953_d_n6;
        locals.var_tu_dn7 = assign82140_e124953_d_n7;
        locals.var_tu_dn8 = assign82140_e124953_d_n8;
        locals.var_tu_dn9 = assign82140_e124953_d_n9;
        locals.var_tu_dn10 = assign82140_e124953_d_n10;
        locals.var_tu_dn13 = assign82140_e124953_d_n13;

    }

    pub(super) fn stamp_transient_block_286(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82150_e124968, assign82150_e124968_d_n0, assign82150_e124968_d_n2, assign82150_e124968_d_n4, assign82150_e124968_d_n5, assign82150_e124968_d_n6, assign82150_e124968_d_n7, assign82150_e124968_d_n8, assign82150_e124968_d_n9, assign82150_e124968_d_n10, assign82150_e124968_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82150_e124963: f64 = (locals.var_tq + locals.var_t5);
        let assign82150_e124965: f64 = (assign82150_e124963).powf(0.3333333333333333);
        let assign82150_e124966: f64 = (-assign82150_e124965);
        (assign82150_e124966, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign82150_e124963))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82150_e124963).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign82150_e124965 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign82150_e124963))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign82150_e124968;
        locals.var_tv_dn0 = assign82150_e124968_d_n0;
        locals.var_tv_dn2 = assign82150_e124968_d_n2;
        locals.var_tv_dn4 = assign82150_e124968_d_n4;
        locals.var_tv_dn5 = assign82150_e124968_d_n5;
        locals.var_tv_dn6 = assign82150_e124968_d_n6;
        locals.var_tv_dn7 = assign82150_e124968_d_n7;
        locals.var_tv_dn8 = assign82150_e124968_d_n8;
        locals.var_tv_dn9 = assign82150_e124968_d_n9;
        locals.var_tv_dn10 = assign82150_e124968_d_n10;
        locals.var_tv_dn13 = assign82150_e124968_d_n13;

        let (assign82160_e124986, assign82160_e124986_d_n0, assign82160_e124986_d_n2, assign82160_e124986_d_n4, assign82160_e124986_d_n5, assign82160_e124986_d_n6, assign82160_e124986_d_n7, assign82160_e124986_d_n8, assign82160_e124986_d_n9, assign82160_e124986_d_n10, assign82160_e124986_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82160_e124978: f64 = (locals.var_tu + locals.var_tv);
        let assign82160_e124982: f64 = (3.0 * locals.var_ta);
        let assign82160_e124983: f64 = (locals.var_tb / assign82160_e124982);
        let assign82160_e124984: f64 = (assign82160_e124978 - assign82160_e124983);
        (assign82160_e124984, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82160_e124986;
        locals.var_chi_dn0 = assign82160_e124986_d_n0;
        locals.var_chi_dn2 = assign82160_e124986_d_n2;
        locals.var_chi_dn4 = assign82160_e124986_d_n4;
        locals.var_chi_dn5 = assign82160_e124986_d_n5;
        locals.var_chi_dn6 = assign82160_e124986_d_n6;
        locals.var_chi_dn7 = assign82160_e124986_d_n7;
        locals.var_chi_dn8 = assign82160_e124986_d_n8;
        locals.var_chi_dn9 = assign82160_e124986_d_n9;
        locals.var_chi_dn10 = assign82160_e124986_d_n10;
        locals.var_chi_dn13 = assign82160_e124986_d_n13;

        let (assign82170_e125000, assign82170_e125000_d_n0, assign82170_e125000_d_n2, assign82170_e125000_d_n4, assign82170_e125000_d_n5, assign82170_e125000_d_n6, assign82170_e125000_d_n7, assign82170_e125000_d_n8, assign82170_e125000_d_n9, assign82170_e125000_d_n10, assign82170_e125000_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign82170_e124996: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82170_e124998: f64 = (assign82170_e124996 - locals.var_vxbgmtcl);
        (assign82170_e124998, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign82170_e125000;
        locals.var_ps0_inia_dn0 = assign82170_e125000_d_n0;
        locals.var_ps0_inia_dn2 = assign82170_e125000_d_n2;
        locals.var_ps0_inia_dn4 = assign82170_e125000_d_n4;
        locals.var_ps0_inia_dn5 = assign82170_e125000_d_n5;
        locals.var_ps0_inia_dn6 = assign82170_e125000_d_n6;
        locals.var_ps0_inia_dn7 = assign82170_e125000_d_n7;
        locals.var_ps0_inia_dn8 = assign82170_e125000_d_n8;
        locals.var_ps0_inia_dn9 = assign82170_e125000_d_n9;
        locals.var_ps0_inia_dn10 = assign82170_e125000_d_n10;
        locals.var_ps0_inia_dn13 = assign82170_e125000_d_n13;

        let assign82180_e125003: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1911 = assign82180_e125003;

        let (assign82190_e125016, assign82190_e125016_d_n0, assign82190_e125016_d_n2, assign82190_e125016_d_n4, assign82190_e125016_d_n5, assign82190_e125016_d_n6, assign82190_e125016_d_n7, assign82190_e125016_d_n8, assign82190_e125016_d_n9, assign82190_e125016_d_n10, assign82190_e125016_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82190_e125012: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82190_e125014: f64 = (assign82190_e125012 + 0.1);
        (assign82190_e125014, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign82190_e125016;
        locals.var_vgpld_shift_dn0 = assign82190_e125016_d_n0;
        locals.var_vgpld_shift_dn2 = assign82190_e125016_d_n2;
        locals.var_vgpld_shift_dn4 = assign82190_e125016_d_n4;
        locals.var_vgpld_shift_dn5 = assign82190_e125016_d_n5;
        locals.var_vgpld_shift_dn6 = assign82190_e125016_d_n6;
        locals.var_vgpld_shift_dn7 = assign82190_e125016_d_n7;
        locals.var_vgpld_shift_dn8 = assign82190_e125016_d_n8;
        locals.var_vgpld_shift_dn9 = assign82190_e125016_d_n9;
        locals.var_vgpld_shift_dn10 = assign82190_e125016_d_n10;
        locals.var_vgpld_shift_dn13 = assign82190_e125016_d_n13;

        let (assign82200_e125027, assign82200_e125027_d_n0, assign82200_e125027_d_n2, assign82200_e125027_d_n4, assign82200_e125027_d_n5, assign82200_e125027_d_n6, assign82200_e125027_d_n7, assign82200_e125027_d_n8, assign82200_e125027_d_n9, assign82200_e125027_d_n10, assign82200_e125027_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82200_e125025: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82200_e125025, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign82200_e125027;
        locals.var_cfs1_dn0 = assign82200_e125027_d_n0;
        locals.var_cfs1_dn2 = assign82200_e125027_d_n2;
        locals.var_cfs1_dn4 = assign82200_e125027_d_n4;
        locals.var_cfs1_dn5 = assign82200_e125027_d_n5;
        locals.var_cfs1_dn6 = assign82200_e125027_d_n6;
        locals.var_cfs1_dn7 = assign82200_e125027_d_n7;
        locals.var_cfs1_dn8 = assign82200_e125027_d_n8;
        locals.var_cfs1_dn9 = assign82200_e125027_d_n9;
        locals.var_cfs1_dn10 = assign82200_e125027_d_n10;
        locals.var_cfs1_dn13 = assign82200_e125027_d_n13;

        let (assign82210_e125038, assign82210_e125038_d_n0, assign82210_e125038_d_n2, assign82210_e125038_d_n4, assign82210_e125038_d_n5, assign82210_e125038_d_n6, assign82210_e125038_d_n7, assign82210_e125038_d_n8, assign82210_e125038_d_n9, assign82210_e125038_d_n10, assign82210_e125038_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82210_e125036: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82210_e125036, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign82210_e125038;
        locals.var_gammachi_dn0 = assign82210_e125038_d_n0;
        locals.var_gammachi_dn2 = assign82210_e125038_d_n2;
        locals.var_gammachi_dn4 = assign82210_e125038_d_n4;
        locals.var_gammachi_dn5 = assign82210_e125038_d_n5;
        locals.var_gammachi_dn6 = assign82210_e125038_d_n6;
        locals.var_gammachi_dn7 = assign82210_e125038_d_n7;
        locals.var_gammachi_dn8 = assign82210_e125038_d_n8;
        locals.var_gammachi_dn9 = assign82210_e125038_d_n9;
        locals.var_gammachi_dn10 = assign82210_e125038_d_n10;
        locals.var_gammachi_dn13 = assign82210_e125038_d_n13;

        let (assign82220_e125049, assign82220_e125049_d_n0, assign82220_e125049_d_n2, assign82220_e125049_d_n4, assign82220_e125049_d_n5, assign82220_e125049_d_n6, assign82220_e125049_d_n7, assign82220_e125049_d_n8, assign82220_e125049_d_n9, assign82220_e125049_d_n10, assign82220_e125049_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82220_e125047: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign82220_e125047, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign82220_e125049;
        locals.var_t0_dn0 = assign82220_e125049_d_n0;
        locals.var_t0_dn2 = assign82220_e125049_d_n2;
        locals.var_t0_dn4 = assign82220_e125049_d_n4;
        locals.var_t0_dn5 = assign82220_e125049_d_n5;
        locals.var_t0_dn6 = assign82220_e125049_d_n6;
        locals.var_t0_dn7 = assign82220_e125049_d_n7;
        locals.var_t0_dn8 = assign82220_e125049_d_n8;
        locals.var_t0_dn9 = assign82220_e125049_d_n9;
        locals.var_t0_dn10 = assign82220_e125049_d_n10;
        locals.var_t0_dn13 = assign82220_e125049_d_n13;

        let (assign82230_e125060, assign82230_e125060_d_n0, assign82230_e125060_d_n2, assign82230_e125060_d_n4, assign82230_e125060_d_n5, assign82230_e125060_d_n6, assign82230_e125060_d_n7, assign82230_e125060_d_n8, assign82230_e125060_d_n9, assign82230_e125060_d_n10, assign82230_e125060_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82230_e125058: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign82230_e125058, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82230_e125060;
        locals.var_psi_dn0 = assign82230_e125060_d_n0;
        locals.var_psi_dn2 = assign82230_e125060_d_n2;
        locals.var_psi_dn4 = assign82230_e125060_d_n4;
        locals.var_psi_dn5 = assign82230_e125060_d_n5;
        locals.var_psi_dn6 = assign82230_e125060_d_n6;
        locals.var_psi_dn7 = assign82230_e125060_d_n7;
        locals.var_psi_dn8 = assign82230_e125060_d_n8;
        locals.var_psi_dn9 = assign82230_e125060_d_n9;
        locals.var_psi_dn10 = assign82230_e125060_d_n10;
        locals.var_psi_dn13 = assign82230_e125060_d_n13;

        let (assign82240_e125085, assign82240_e125085_d_n0, assign82240_e125085_d_n2, assign82240_e125085_d_n4, assign82240_e125085_d_n5, assign82240_e125085_d_n6, assign82240_e125085_d_n7, assign82240_e125085_d_n8, assign82240_e125085_d_n9, assign82240_e125085_d_n10, assign82240_e125085_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82240_e125069: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82240_e125072: f64 = (locals.var_psi * locals.var_psi);
        let assign82240_e125073: f64 = (assign82240_e125069 + assign82240_e125072);
        let assign82240_e125074: f64 = (assign82240_e125073).ln();
        let assign82240_e125077: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82240_e125078: f64 = (assign82240_e125077).ln();
        let assign82240_e125079: f64 = (assign82240_e125074 - assign82240_e125078);
        let assign82240_e125082: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82240_e125083: f64 = (assign82240_e125079 + assign82240_e125082);
        (assign82240_e125083, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82240_e125073) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82240_e125077)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82240_e125073) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82240_e125077)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82240_e125073) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82240_e125077)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82240_e125073) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82240_e125077)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82240_e125073) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82240_e125077)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82240_e125073) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82240_e125077)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82240_e125073) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82240_e125077)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82240_e125073) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82240_e125077)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82240_e125073) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82240_e125077)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign82240_e125073) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign82240_e125077)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82240_e125085;
        locals.var_chi_1_dn0 = assign82240_e125085_d_n0;
        locals.var_chi_1_dn2 = assign82240_e125085_d_n2;
        locals.var_chi_1_dn4 = assign82240_e125085_d_n4;
        locals.var_chi_1_dn5 = assign82240_e125085_d_n5;
        locals.var_chi_1_dn6 = assign82240_e125085_d_n6;
        locals.var_chi_1_dn7 = assign82240_e125085_d_n7;
        locals.var_chi_1_dn8 = assign82240_e125085_d_n8;
        locals.var_chi_1_dn9 = assign82240_e125085_d_n9;
        locals.var_chi_1_dn10 = assign82240_e125085_d_n10;
        locals.var_chi_1_dn13 = assign82240_e125085_d_n13;

        let assign82250_e125088: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1912 = assign82250_e125088;

        let (assign82260_e125103, assign82260_e125103_d_n0, assign82260_e125103_d_n2, assign82260_e125103_d_n4, assign82260_e125103_d_n5, assign82260_e125103_d_n6, assign82260_e125103_d_n7, assign82260_e125103_d_n8, assign82260_e125103_d_n9, assign82260_e125103_d_n10, assign82260_e125103_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82260_e125099: f64 = (locals.var_psi - locals.var_chi_1);
        let assign82260_e125101: f64 = (assign82260_e125099 - 1.0);
        (assign82260_e125101, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign82260_e125103;
        locals.var_tmf1_dn0 = assign82260_e125103_d_n0;
        locals.var_tmf1_dn2 = assign82260_e125103_d_n2;
        locals.var_tmf1_dn4 = assign82260_e125103_d_n4;
        locals.var_tmf1_dn5 = assign82260_e125103_d_n5;
        locals.var_tmf1_dn6 = assign82260_e125103_d_n6;
        locals.var_tmf1_dn7 = assign82260_e125103_d_n7;
        locals.var_tmf1_dn8 = assign82260_e125103_d_n8;
        locals.var_tmf1_dn9 = assign82260_e125103_d_n9;
        locals.var_tmf1_dn10 = assign82260_e125103_d_n10;
        locals.var_tmf1_dn13 = assign82260_e125103_d_n13;

        let (assign82270_e125118, assign82270_e125118_d_n0, assign82270_e125118_d_n2, assign82270_e125118_d_n4, assign82270_e125118_d_n5, assign82270_e125118_d_n6, assign82270_e125118_d_n7, assign82270_e125118_d_n8, assign82270_e125118_d_n9, assign82270_e125118_d_n10, assign82270_e125118_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82270_e125114: f64 = (4.0 * locals.var_psi);
        let assign82270_e125116: f64 = assign82270_e125114;
        (assign82270_e125116, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82270_e125118;
        locals.var_tmf2_dn0 = assign82270_e125118_d_n0;
        locals.var_tmf2_dn2 = assign82270_e125118_d_n2;
        locals.var_tmf2_dn4 = assign82270_e125118_d_n4;
        locals.var_tmf2_dn5 = assign82270_e125118_d_n5;
        locals.var_tmf2_dn6 = assign82270_e125118_d_n6;
        locals.var_tmf2_dn7 = assign82270_e125118_d_n7;
        locals.var_tmf2_dn8 = assign82270_e125118_d_n8;
        locals.var_tmf2_dn9 = assign82270_e125118_d_n9;
        locals.var_tmf2_dn10 = assign82270_e125118_d_n10;
        locals.var_tmf2_dn13 = assign82270_e125118_d_n13;

        let (assign82280_e125135, assign82280_e125135_d_n0, assign82280_e125135_d_n2, assign82280_e125135_d_n4, assign82280_e125135_d_n5, assign82280_e125135_d_n6, assign82280_e125135_d_n7, assign82280_e125135_d_n8, assign82280_e125135_d_n9, assign82280_e125135_d_n10, assign82280_e125135_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let (assign82280_e125133, assign82280_e125133_d_n0, assign82280_e125133_d_n2, assign82280_e125133_d_n4, assign82280_e125133_d_n5, assign82280_e125133_d_n6, assign82280_e125133_d_n7, assign82280_e125133_d_n8, assign82280_e125133_d_n9, assign82280_e125133_d_n10, assign82280_e125133_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign82280_e125132: f64 = (-locals.var_tmf2);
                (assign82280_e125132, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign82280_e125133, assign82280_e125133_d_n0, assign82280_e125133_d_n2, assign82280_e125133_d_n4, assign82280_e125133_d_n5, assign82280_e125133_d_n6, assign82280_e125133_d_n7, assign82280_e125133_d_n8, assign82280_e125133_d_n9, assign82280_e125133_d_n10, assign82280_e125133_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82280_e125135;
        locals.var_tmf2_dn0 = assign82280_e125135_d_n0;
        locals.var_tmf2_dn2 = assign82280_e125135_d_n2;
        locals.var_tmf2_dn4 = assign82280_e125135_d_n4;
        locals.var_tmf2_dn5 = assign82280_e125135_d_n5;
        locals.var_tmf2_dn6 = assign82280_e125135_d_n6;
        locals.var_tmf2_dn7 = assign82280_e125135_d_n7;
        locals.var_tmf2_dn8 = assign82280_e125135_d_n8;
        locals.var_tmf2_dn9 = assign82280_e125135_d_n9;
        locals.var_tmf2_dn10 = assign82280_e125135_d_n10;
        locals.var_tmf2_dn13 = assign82280_e125135_d_n13;

        let (assign82290_e125151, assign82290_e125151_d_n0, assign82290_e125151_d_n2, assign82290_e125151_d_n4, assign82290_e125151_d_n5, assign82290_e125151_d_n6, assign82290_e125151_d_n7, assign82290_e125151_d_n8, assign82290_e125151_d_n9, assign82290_e125151_d_n10, assign82290_e125151_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82290_e125146: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign82290_e125148: f64 = (assign82290_e125146 + locals.var_tmf2);
        let assign82290_e125149: f64 = (assign82290_e125148).sqrt();
        (assign82290_e125149, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign82290_e125149)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign82290_e125149)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82290_e125151;
        locals.var_tmf2_dn0 = assign82290_e125151_d_n0;
        locals.var_tmf2_dn2 = assign82290_e125151_d_n2;
        locals.var_tmf2_dn4 = assign82290_e125151_d_n4;
        locals.var_tmf2_dn5 = assign82290_e125151_d_n5;
        locals.var_tmf2_dn6 = assign82290_e125151_d_n6;
        locals.var_tmf2_dn7 = assign82290_e125151_d_n7;
        locals.var_tmf2_dn8 = assign82290_e125151_d_n8;
        locals.var_tmf2_dn9 = assign82290_e125151_d_n9;
        locals.var_tmf2_dn10 = assign82290_e125151_d_n10;
        locals.var_tmf2_dn13 = assign82290_e125151_d_n13;

        let (assign82300_e125168, assign82300_e125168_d_n0, assign82300_e125168_d_n2, assign82300_e125168_d_n4, assign82300_e125168_d_n5, assign82300_e125168_d_n6, assign82300_e125168_d_n7, assign82300_e125168_d_n8, assign82300_e125168_d_n9, assign82300_e125168_d_n10, assign82300_e125168_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82300_e125164: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign82300_e125165: f64 = (1.0 + assign82300_e125164);
        let assign82300_e125166: f64 = (0.5 * assign82300_e125165);
        (assign82300_e125166, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82300_e125168;
        locals.var_t1_dn0 = assign82300_e125168_d_n0;
        locals.var_t1_dn2 = assign82300_e125168_d_n2;
        locals.var_t1_dn4 = assign82300_e125168_d_n4;
        locals.var_t1_dn5 = assign82300_e125168_d_n5;
        locals.var_t1_dn6 = assign82300_e125168_d_n6;
        locals.var_t1_dn7 = assign82300_e125168_d_n7;
        locals.var_t1_dn8 = assign82300_e125168_d_n8;
        locals.var_t1_dn9 = assign82300_e125168_d_n9;
        locals.var_t1_dn10 = assign82300_e125168_d_n10;
        locals.var_t1_dn13 = assign82300_e125168_d_n13;

        let (assign82310_e125185, assign82310_e125185_d_n0, assign82310_e125185_d_n2, assign82310_e125185_d_n4, assign82310_e125185_d_n5, assign82310_e125185_d_n6, assign82310_e125185_d_n7, assign82310_e125185_d_n8, assign82310_e125185_d_n9, assign82310_e125185_d_n10, assign82310_e125185_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82310_e125181: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign82310_e125182: f64 = (0.5 * assign82310_e125181);
        let assign82310_e125183: f64 = (locals.var_psi - assign82310_e125182);
        (assign82310_e125183, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82310_e125185;
        locals.var_chi_1_dn0 = assign82310_e125185_d_n0;
        locals.var_chi_1_dn2 = assign82310_e125185_d_n2;
        locals.var_chi_1_dn4 = assign82310_e125185_d_n4;
        locals.var_chi_1_dn5 = assign82310_e125185_d_n5;
        locals.var_chi_1_dn6 = assign82310_e125185_d_n6;
        locals.var_chi_1_dn7 = assign82310_e125185_d_n7;
        locals.var_chi_1_dn8 = assign82310_e125185_d_n8;
        locals.var_chi_1_dn9 = assign82310_e125185_d_n9;
        locals.var_chi_1_dn10 = assign82310_e125185_d_n10;
        locals.var_chi_1_dn13 = assign82310_e125185_d_n13;

        let (assign82320_e125202, assign82320_e125202_d_n0, assign82320_e125202_d_n2, assign82320_e125202_d_n4, assign82320_e125202_d_n5, assign82320_e125202_d_n6, assign82320_e125202_d_n7, assign82320_e125202_d_n8, assign82320_e125202_d_n9, assign82320_e125202_d_n10, assign82320_e125202_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1912 == 0.0)) {
        let (assign82320_e125200, assign82320_e125200_d_n0, assign82320_e125200_d_n2, assign82320_e125200_d_n4, assign82320_e125200_d_n5, assign82320_e125200_d_n6, assign82320_e125200_d_n7, assign82320_e125200_d_n8, assign82320_e125200_d_n9, assign82320_e125200_d_n10, assign82320_e125200_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign82320_e125200, assign82320_e125200_d_n0, assign82320_e125200_d_n2, assign82320_e125200_d_n4, assign82320_e125200_d_n5, assign82320_e125200_d_n6, assign82320_e125200_d_n7, assign82320_e125200_d_n8, assign82320_e125200_d_n9, assign82320_e125200_d_n10, assign82320_e125200_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82320_e125202;
        locals.var_chi_1_dn0 = assign82320_e125202_d_n0;
        locals.var_chi_1_dn2 = assign82320_e125202_d_n2;
        locals.var_chi_1_dn4 = assign82320_e125202_d_n4;
        locals.var_chi_1_dn5 = assign82320_e125202_d_n5;
        locals.var_chi_1_dn6 = assign82320_e125202_d_n6;
        locals.var_chi_1_dn7 = assign82320_e125202_d_n7;
        locals.var_chi_1_dn8 = assign82320_e125202_d_n8;
        locals.var_chi_1_dn9 = assign82320_e125202_d_n9;
        locals.var_chi_1_dn10 = assign82320_e125202_d_n10;
        locals.var_chi_1_dn13 = assign82320_e125202_d_n13;

        let (assign82330_e125216, assign82330_e125216_d_n0, assign82330_e125216_d_n2, assign82330_e125216_d_n4, assign82330_e125216_d_n5, assign82330_e125216_d_n6, assign82330_e125216_d_n7, assign82330_e125216_d_n8, assign82330_e125216_d_n9, assign82330_e125216_d_n10, assign82330_e125216_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let (assign82330_e125214, assign82330_e125214_d_n0, assign82330_e125214_d_n2, assign82330_e125214_d_n4, assign82330_e125214_d_n5, assign82330_e125214_d_n6, assign82330_e125214_d_n7, assign82330_e125214_d_n8, assign82330_e125214_d_n9, assign82330_e125214_d_n10, assign82330_e125214_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82330_e125214, assign82330_e125214_d_n0, assign82330_e125214_d_n2, assign82330_e125214_d_n4, assign82330_e125214_d_n5, assign82330_e125214_d_n6, assign82330_e125214_d_n7, assign82330_e125214_d_n8, assign82330_e125214_d_n9, assign82330_e125214_d_n10, assign82330_e125214_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign82330_e125216;
        locals.var_chi_1_dn0 = assign82330_e125216_d_n0;
        locals.var_chi_1_dn2 = assign82330_e125216_d_n2;
        locals.var_chi_1_dn4 = assign82330_e125216_d_n4;
        locals.var_chi_1_dn5 = assign82330_e125216_d_n5;
        locals.var_chi_1_dn6 = assign82330_e125216_d_n6;
        locals.var_chi_1_dn7 = assign82330_e125216_d_n7;
        locals.var_chi_1_dn8 = assign82330_e125216_d_n8;
        locals.var_chi_1_dn9 = assign82330_e125216_d_n9;
        locals.var_chi_1_dn10 = assign82330_e125216_d_n10;
        locals.var_chi_1_dn13 = assign82330_e125216_d_n13;

        let (assign82340_e125227, assign82340_e125227_d_n0, assign82340_e125227_d_n2, assign82340_e125227_d_n4, assign82340_e125227_d_n5, assign82340_e125227_d_n6, assign82340_e125227_d_n7, assign82340_e125227_d_n8, assign82340_e125227_d_n9, assign82340_e125227_d_n10, assign82340_e125227_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82340_e125225: f64 = (locals.var_psi - locals.var_chi_1);
        (assign82340_e125225, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82340_e125227;
        locals.var_psi_dn0 = assign82340_e125227_d_n0;
        locals.var_psi_dn2 = assign82340_e125227_d_n2;
        locals.var_psi_dn4 = assign82340_e125227_d_n4;
        locals.var_psi_dn5 = assign82340_e125227_d_n5;
        locals.var_psi_dn6 = assign82340_e125227_d_n6;
        locals.var_psi_dn7 = assign82340_e125227_d_n7;
        locals.var_psi_dn8 = assign82340_e125227_d_n8;
        locals.var_psi_dn9 = assign82340_e125227_d_n9;
        locals.var_psi_dn10 = assign82340_e125227_d_n10;
        locals.var_psi_dn13 = assign82340_e125227_d_n13;

        let (assign82350_e125240, assign82350_e125240_d_n0, assign82350_e125240_d_n2, assign82350_e125240_d_n4, assign82350_e125240_d_n5, assign82350_e125240_d_n6, assign82350_e125240_d_n7, assign82350_e125240_d_n8, assign82350_e125240_d_n9, assign82350_e125240_d_n10, assign82350_e125240_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82350_e125237: f64 = (locals.var_beta * 0.1);
        let assign82350_e125238: f64 = (locals.var_psi + assign82350_e125237);
        (assign82350_e125238, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign82350_e125240;
        locals.var_psi_dn0 = assign82350_e125240_d_n0;
        locals.var_psi_dn2 = assign82350_e125240_d_n2;
        locals.var_psi_dn4 = assign82350_e125240_d_n4;
        locals.var_psi_dn5 = assign82350_e125240_d_n5;
        locals.var_psi_dn6 = assign82350_e125240_d_n6;
        locals.var_psi_dn7 = assign82350_e125240_d_n7;
        locals.var_psi_dn8 = assign82350_e125240_d_n8;
        locals.var_psi_dn9 = assign82350_e125240_d_n9;
        locals.var_psi_dn10 = assign82350_e125240_d_n10;
        locals.var_psi_dn13 = assign82350_e125240_d_n13;

        let (assign82360_e125261, assign82360_e125261_d_n0, assign82360_e125261_d_n2, assign82360_e125261_d_n4, assign82360_e125261_d_n5, assign82360_e125261_d_n6, assign82360_e125261_d_n7, assign82360_e125261_d_n8, assign82360_e125261_d_n9, assign82360_e125261_d_n10, assign82360_e125261_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82360_e125249: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82360_e125252: f64 = (locals.var_psi * locals.var_psi);
        let assign82360_e125253: f64 = (assign82360_e125249 + assign82360_e125252);
        let assign82360_e125254: f64 = (assign82360_e125253).ln();
        let assign82360_e125257: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82360_e125258: f64 = (assign82360_e125257).ln();
        let assign82360_e125259: f64 = (assign82360_e125254 - assign82360_e125258);
        (assign82360_e125259, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82360_e125253) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82360_e125257)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82360_e125253) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82360_e125257)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82360_e125253) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82360_e125257)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82360_e125253) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82360_e125257)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82360_e125253) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82360_e125257)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82360_e125253) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82360_e125257)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82360_e125253) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82360_e125257)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82360_e125253) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82360_e125257)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82360_e125253) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82360_e125257)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign82360_e125253) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign82360_e125257)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82360_e125261;
        locals.var_t1_dn0 = assign82360_e125261_d_n0;
        locals.var_t1_dn2 = assign82360_e125261_d_n2;
        locals.var_t1_dn4 = assign82360_e125261_d_n4;
        locals.var_t1_dn5 = assign82360_e125261_d_n5;
        locals.var_t1_dn6 = assign82360_e125261_d_n6;
        locals.var_t1_dn7 = assign82360_e125261_d_n7;
        locals.var_t1_dn8 = assign82360_e125261_d_n8;
        locals.var_t1_dn9 = assign82360_e125261_d_n9;
        locals.var_t1_dn10 = assign82360_e125261_d_n10;
        locals.var_t1_dn13 = assign82360_e125261_d_n13;

        let (assign82370_e125274, assign82370_e125274_d_n0, assign82370_e125274_d_n2, assign82370_e125274_d_n4, assign82370_e125274_d_n5, assign82370_e125274_d_n6, assign82370_e125274_d_n7, assign82370_e125274_d_n8, assign82370_e125274_d_n9, assign82370_e125274_d_n10, assign82370_e125274_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign82370_e125271: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82370_e125272: f64 = (locals.var_t1 + assign82370_e125271);
        (assign82370_e125272, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign82370_e125274;
        locals.var_chi_b_dn0 = assign82370_e125274_d_n0;
        locals.var_chi_b_dn2 = assign82370_e125274_d_n2;
        locals.var_chi_b_dn4 = assign82370_e125274_d_n4;
        locals.var_chi_b_dn5 = assign82370_e125274_d_n5;
        locals.var_chi_b_dn6 = assign82370_e125274_d_n6;
        locals.var_chi_b_dn7 = assign82370_e125274_d_n7;
        locals.var_chi_b_dn8 = assign82370_e125274_d_n8;
        locals.var_chi_b_dn9 = assign82370_e125274_d_n9;
        locals.var_chi_b_dn10 = assign82370_e125274_d_n10;
        locals.var_chi_b_dn13 = assign82370_e125274_d_n13;

        let (assign82380_e125288, assign82380_e125288_d_n0, assign82380_e125288_d_n2, assign82380_e125288_d_n4, assign82380_e125288_d_n5, assign82380_e125288_d_n6, assign82380_e125288_d_n7, assign82380_e125288_d_n8, assign82380_e125288_d_n9, assign82380_e125288_d_n10, assign82380_e125288_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let (assign82380_e125286, assign82380_e125286_d_n0, assign82380_e125286_d_n2, assign82380_e125286_d_n4, assign82380_e125286_d_n5, assign82380_e125286_d_n6, assign82380_e125286_d_n7, assign82380_e125286_d_n8, assign82380_e125286_d_n9, assign82380_e125286_d_n10, assign82380_e125286_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82380_e125286, assign82380_e125286_d_n0, assign82380_e125286_d_n2, assign82380_e125286_d_n4, assign82380_e125286_d_n5, assign82380_e125286_d_n6, assign82380_e125286_d_n7, assign82380_e125286_d_n8, assign82380_e125286_d_n9, assign82380_e125286_d_n10, assign82380_e125286_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign82380_e125288;
        locals.var_chi_b_dn0 = assign82380_e125288_d_n0;
        locals.var_chi_b_dn2 = assign82380_e125288_d_n2;
        locals.var_chi_b_dn4 = assign82380_e125288_d_n4;
        locals.var_chi_b_dn5 = assign82380_e125288_d_n5;
        locals.var_chi_b_dn6 = assign82380_e125288_d_n6;
        locals.var_chi_b_dn7 = assign82380_e125288_d_n7;
        locals.var_chi_b_dn8 = assign82380_e125288_d_n8;
        locals.var_chi_b_dn9 = assign82380_e125288_d_n9;
        locals.var_chi_b_dn10 = assign82380_e125288_d_n10;
        locals.var_chi_b_dn13 = assign82380_e125288_d_n13;

    }

    pub(super) fn stamp_transient_block_287(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82390_e125297, assign82390_e125297_d_n0, assign82390_e125297_d_n2, assign82390_e125297_d_n4, assign82390_e125297_d_n5, assign82390_e125297_d_n6, assign82390_e125297_d_n7, assign82390_e125297_d_n8, assign82390_e125297_d_n9, assign82390_e125297_d_n10, assign82390_e125297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign82390_e125297;
        locals.var_chi_a_dn0 = assign82390_e125297_d_n0;
        locals.var_chi_a_dn2 = assign82390_e125297_d_n2;
        locals.var_chi_a_dn4 = assign82390_e125297_d_n4;
        locals.var_chi_a_dn5 = assign82390_e125297_d_n5;
        locals.var_chi_a_dn6 = assign82390_e125297_d_n6;
        locals.var_chi_a_dn7 = assign82390_e125297_d_n7;
        locals.var_chi_a_dn8 = assign82390_e125297_d_n8;
        locals.var_chi_a_dn9 = assign82390_e125297_d_n9;
        locals.var_chi_a_dn10 = assign82390_e125297_d_n10;
        locals.var_chi_a_dn13 = assign82390_e125297_d_n13;

        let assign82400_e125300: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1913 = assign82400_e125300;

        let assign82410_e125305: f64 = (0.2 * locals.var_chi_b);
        let assign82410_e125306: f64 = (locals.var_chi_b - assign82410_e125305);
        let assign82410_e125310: f64 = (0.2 * locals.var_chi_b);
        let assign82410_e125313: f64 = if ((locals.var_chi_a > assign82410_e125306) && (assign82410_e125310 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1914 = assign82410_e125313;

        let (assign82420_e125332, assign82420_e125332_d_n0, assign82420_e125332_d_n2, assign82420_e125332_d_n4, assign82420_e125332_d_n5, assign82420_e125332_d_n6, assign82420_e125332_d_n7, assign82420_e125332_d_n8, assign82420_e125332_d_n9, assign82420_e125332_d_n10, assign82420_e125332_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82420_e125326: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign82420_e125329: f64 = (0.2 * locals.var_chi_b);
        let assign82420_e125330: f64 = (assign82420_e125326 + assign82420_e125329);
        (assign82420_e125330, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign82420_e125332;
        locals.var_tmf1_dn0 = assign82420_e125332_d_n0;
        locals.var_tmf1_dn2 = assign82420_e125332_d_n2;
        locals.var_tmf1_dn4 = assign82420_e125332_d_n4;
        locals.var_tmf1_dn5 = assign82420_e125332_d_n5;
        locals.var_tmf1_dn6 = assign82420_e125332_d_n6;
        locals.var_tmf1_dn7 = assign82420_e125332_d_n7;
        locals.var_tmf1_dn8 = assign82420_e125332_d_n8;
        locals.var_tmf1_dn9 = assign82420_e125332_d_n9;
        locals.var_tmf1_dn10 = assign82420_e125332_d_n10;
        locals.var_tmf1_dn13 = assign82420_e125332_d_n13;

        let (assign82430_e125347, assign82430_e125347_d_n0, assign82430_e125347_d_n2, assign82430_e125347_d_n4, assign82430_e125347_d_n5, assign82430_e125347_d_n6, assign82430_e125347_d_n7, assign82430_e125347_d_n8, assign82430_e125347_d_n9, assign82430_e125347_d_n10, assign82430_e125347_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82430_e125345: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign82430_e125345, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign82430_e125347;
        locals.var_x2_dn0 = assign82430_e125347_d_n0;
        locals.var_x2_dn2 = assign82430_e125347_d_n2;
        locals.var_x2_dn4 = assign82430_e125347_d_n4;
        locals.var_x2_dn5 = assign82430_e125347_d_n5;
        locals.var_x2_dn6 = assign82430_e125347_d_n6;
        locals.var_x2_dn7 = assign82430_e125347_d_n7;
        locals.var_x2_dn8 = assign82430_e125347_d_n8;
        locals.var_x2_dn9 = assign82430_e125347_d_n9;
        locals.var_x2_dn10 = assign82430_e125347_d_n10;
        locals.var_x2_dn13 = assign82430_e125347_d_n13;

        let (assign82440_e125366, assign82440_e125366_d_n0, assign82440_e125366_d_n2, assign82440_e125366_d_n4, assign82440_e125366_d_n5, assign82440_e125366_d_n6, assign82440_e125366_d_n7, assign82440_e125366_d_n8, assign82440_e125366_d_n9, assign82440_e125366_d_n10, assign82440_e125366_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82440_e125360: f64 = (0.2 * locals.var_chi_b);
        let assign82440_e125363: f64 = (0.2 * locals.var_chi_b);
        let assign82440_e125364: f64 = (assign82440_e125360 * assign82440_e125363);
        (assign82440_e125364, (((0.2 * locals.var_chi_b_dn0) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign82440_e125363) + (assign82440_e125360 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign82440_e125366;
        locals.var_xmax2_dn0 = assign82440_e125366_d_n0;
        locals.var_xmax2_dn2 = assign82440_e125366_d_n2;
        locals.var_xmax2_dn4 = assign82440_e125366_d_n4;
        locals.var_xmax2_dn5 = assign82440_e125366_d_n5;
        locals.var_xmax2_dn6 = assign82440_e125366_d_n6;
        locals.var_xmax2_dn7 = assign82440_e125366_d_n7;
        locals.var_xmax2_dn8 = assign82440_e125366_d_n8;
        locals.var_xmax2_dn9 = assign82440_e125366_d_n9;
        locals.var_xmax2_dn10 = assign82440_e125366_d_n10;
        locals.var_xmax2_dn13 = assign82440_e125366_d_n13;

        let (assign82450_e125379, assign82450_e125379_d_n0, assign82450_e125379_d_n2, assign82450_e125379_d_n4, assign82450_e125379_d_n5, assign82450_e125379_d_n6, assign82450_e125379_d_n7, assign82450_e125379_d_n8, assign82450_e125379_d_n9, assign82450_e125379_d_n10, assign82450_e125379_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82450_e125379;
        locals.var_xp_dn0 = assign82450_e125379_d_n0;
        locals.var_xp_dn2 = assign82450_e125379_d_n2;
        locals.var_xp_dn4 = assign82450_e125379_d_n4;
        locals.var_xp_dn5 = assign82450_e125379_d_n5;
        locals.var_xp_dn6 = assign82450_e125379_d_n6;
        locals.var_xp_dn7 = assign82450_e125379_d_n7;
        locals.var_xp_dn8 = assign82450_e125379_d_n8;
        locals.var_xp_dn9 = assign82450_e125379_d_n9;
        locals.var_xp_dn10 = assign82450_e125379_d_n10;
        locals.var_xp_dn13 = assign82450_e125379_d_n13;

        let (assign82460_e125392, assign82460_e125392_d_n0, assign82460_e125392_d_n2, assign82460_e125392_d_n4, assign82460_e125392_d_n5, assign82460_e125392_d_n6, assign82460_e125392_d_n7, assign82460_e125392_d_n8, assign82460_e125392_d_n9, assign82460_e125392_d_n10, assign82460_e125392_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82460_e125392;
        locals.var_xmp_dn0 = assign82460_e125392_d_n0;
        locals.var_xmp_dn2 = assign82460_e125392_d_n2;
        locals.var_xmp_dn4 = assign82460_e125392_d_n4;
        locals.var_xmp_dn5 = assign82460_e125392_d_n5;
        locals.var_xmp_dn6 = assign82460_e125392_d_n6;
        locals.var_xmp_dn7 = assign82460_e125392_d_n7;
        locals.var_xmp_dn8 = assign82460_e125392_d_n8;
        locals.var_xmp_dn9 = assign82460_e125392_d_n9;
        locals.var_xmp_dn10 = assign82460_e125392_d_n10;
        locals.var_xmp_dn13 = assign82460_e125392_d_n13;

        let (assign82470_e125405,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82470_e125405;

        let (assign82480_e125418,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82480_e125418;

        let (assign82490_e125431, assign82490_e125431_d_n0, assign82490_e125431_d_n2, assign82490_e125431_d_n4, assign82490_e125431_d_n5, assign82490_e125431_d_n6, assign82490_e125431_d_n7, assign82490_e125431_d_n8, assign82490_e125431_d_n9, assign82490_e125431_d_n10, assign82490_e125431_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign82490_e125431;
        locals.var_arg_dn0 = assign82490_e125431_d_n0;
        locals.var_arg_dn2 = assign82490_e125431_d_n2;
        locals.var_arg_dn4 = assign82490_e125431_d_n4;
        locals.var_arg_dn5 = assign82490_e125431_d_n5;
        locals.var_arg_dn6 = assign82490_e125431_d_n6;
        locals.var_arg_dn7 = assign82490_e125431_d_n7;
        locals.var_arg_dn8 = assign82490_e125431_d_n8;
        locals.var_arg_dn9 = assign82490_e125431_d_n9;
        locals.var_arg_dn10 = assign82490_e125431_d_n10;
        locals.var_arg_dn13 = assign82490_e125431_d_n13;

        let (assign82500_e125444, assign82500_e125444_d_n0, assign82500_e125444_d_n2, assign82500_e125444_d_n4, assign82500_e125444_d_n5, assign82500_e125444_d_n6, assign82500_e125444_d_n7, assign82500_e125444_d_n8, assign82500_e125444_d_n9, assign82500_e125444_d_n10, assign82500_e125444_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82500_e125444;
        locals.var_dnm_dn0 = assign82500_e125444_d_n0;
        locals.var_dnm_dn2 = assign82500_e125444_d_n2;
        locals.var_dnm_dn4 = assign82500_e125444_d_n4;
        locals.var_dnm_dn5 = assign82500_e125444_d_n5;
        locals.var_dnm_dn6 = assign82500_e125444_d_n6;
        locals.var_dnm_dn7 = assign82500_e125444_d_n7;
        locals.var_dnm_dn8 = assign82500_e125444_d_n8;
        locals.var_dnm_dn9 = assign82500_e125444_d_n9;
        locals.var_dnm_dn10 = assign82500_e125444_d_n10;
        locals.var_dnm_dn13 = assign82500_e125444_d_n13;

        let (assign82510_e125459, assign82510_e125459_d_n0, assign82510_e125459_d_n2, assign82510_e125459_d_n4, assign82510_e125459_d_n5, assign82510_e125459_d_n6, assign82510_e125459_d_n7, assign82510_e125459_d_n8, assign82510_e125459_d_n9, assign82510_e125459_d_n10, assign82510_e125459_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82510_e125457: f64 = (locals.var_xp * locals.var_x2);
        (assign82510_e125457, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82510_e125459;
        locals.var_xp_dn0 = assign82510_e125459_d_n0;
        locals.var_xp_dn2 = assign82510_e125459_d_n2;
        locals.var_xp_dn4 = assign82510_e125459_d_n4;
        locals.var_xp_dn5 = assign82510_e125459_d_n5;
        locals.var_xp_dn6 = assign82510_e125459_d_n6;
        locals.var_xp_dn7 = assign82510_e125459_d_n7;
        locals.var_xp_dn8 = assign82510_e125459_d_n8;
        locals.var_xp_dn9 = assign82510_e125459_d_n9;
        locals.var_xp_dn10 = assign82510_e125459_d_n10;
        locals.var_xp_dn13 = assign82510_e125459_d_n13;

        let (assign82520_e125474, assign82520_e125474_d_n0, assign82520_e125474_d_n2, assign82520_e125474_d_n4, assign82520_e125474_d_n5, assign82520_e125474_d_n6, assign82520_e125474_d_n7, assign82520_e125474_d_n8, assign82520_e125474_d_n9, assign82520_e125474_d_n10, assign82520_e125474_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82520_e125472: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82520_e125472, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82520_e125474;
        locals.var_xmp_dn0 = assign82520_e125474_d_n0;
        locals.var_xmp_dn2 = assign82520_e125474_d_n2;
        locals.var_xmp_dn4 = assign82520_e125474_d_n4;
        locals.var_xmp_dn5 = assign82520_e125474_d_n5;
        locals.var_xmp_dn6 = assign82520_e125474_d_n6;
        locals.var_xmp_dn7 = assign82520_e125474_d_n7;
        locals.var_xmp_dn8 = assign82520_e125474_d_n8;
        locals.var_xmp_dn9 = assign82520_e125474_d_n9;
        locals.var_xmp_dn10 = assign82520_e125474_d_n10;
        locals.var_xmp_dn13 = assign82520_e125474_d_n13;

        let (assign82530_e125489, assign82530_e125489_d_n0, assign82530_e125489_d_n2, assign82530_e125489_d_n4, assign82530_e125489_d_n5, assign82530_e125489_d_n6, assign82530_e125489_d_n7, assign82530_e125489_d_n8, assign82530_e125489_d_n9, assign82530_e125489_d_n10, assign82530_e125489_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82530_e125487: f64 = (locals.var_xp * locals.var_x2);
        (assign82530_e125487, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign82530_e125489;
        locals.var_xp_dn0 = assign82530_e125489_d_n0;
        locals.var_xp_dn2 = assign82530_e125489_d_n2;
        locals.var_xp_dn4 = assign82530_e125489_d_n4;
        locals.var_xp_dn5 = assign82530_e125489_d_n5;
        locals.var_xp_dn6 = assign82530_e125489_d_n6;
        locals.var_xp_dn7 = assign82530_e125489_d_n7;
        locals.var_xp_dn8 = assign82530_e125489_d_n8;
        locals.var_xp_dn9 = assign82530_e125489_d_n9;
        locals.var_xp_dn10 = assign82530_e125489_d_n10;
        locals.var_xp_dn13 = assign82530_e125489_d_n13;

        let (assign82540_e125504, assign82540_e125504_d_n0, assign82540_e125504_d_n2, assign82540_e125504_d_n4, assign82540_e125504_d_n5, assign82540_e125504_d_n6, assign82540_e125504_d_n7, assign82540_e125504_d_n8, assign82540_e125504_d_n9, assign82540_e125504_d_n10, assign82540_e125504_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82540_e125502: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82540_e125502, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign82540_e125504;
        locals.var_xmp_dn0 = assign82540_e125504_d_n0;
        locals.var_xmp_dn2 = assign82540_e125504_d_n2;
        locals.var_xmp_dn4 = assign82540_e125504_d_n4;
        locals.var_xmp_dn5 = assign82540_e125504_d_n5;
        locals.var_xmp_dn6 = assign82540_e125504_d_n6;
        locals.var_xmp_dn7 = assign82540_e125504_d_n7;
        locals.var_xmp_dn8 = assign82540_e125504_d_n8;
        locals.var_xmp_dn9 = assign82540_e125504_d_n9;
        locals.var_xmp_dn10 = assign82540_e125504_d_n10;
        locals.var_xmp_dn13 = assign82540_e125504_d_n13;

        let (assign82550_e125519, assign82550_e125519_d_n0, assign82550_e125519_d_n2, assign82550_e125519_d_n4, assign82550_e125519_d_n5, assign82550_e125519_d_n6, assign82550_e125519_d_n7, assign82550_e125519_d_n8, assign82550_e125519_d_n9, assign82550_e125519_d_n10, assign82550_e125519_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82550_e125517: f64 = (locals.var_xp + locals.var_xmp);
        (assign82550_e125517, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign82550_e125519;
        locals.var_arg_dn0 = assign82550_e125519_d_n0;
        locals.var_arg_dn2 = assign82550_e125519_d_n2;
        locals.var_arg_dn4 = assign82550_e125519_d_n4;
        locals.var_arg_dn5 = assign82550_e125519_d_n5;
        locals.var_arg_dn6 = assign82550_e125519_d_n6;
        locals.var_arg_dn7 = assign82550_e125519_d_n7;
        locals.var_arg_dn8 = assign82550_e125519_d_n8;
        locals.var_arg_dn9 = assign82550_e125519_d_n9;
        locals.var_arg_dn10 = assign82550_e125519_d_n10;
        locals.var_arg_dn13 = assign82550_e125519_d_n13;

        let (assign82560_e125532, assign82560_e125532_d_n0, assign82560_e125532_d_n2, assign82560_e125532_d_n4, assign82560_e125532_d_n5, assign82560_e125532_d_n6, assign82560_e125532_d_n7, assign82560_e125532_d_n8, assign82560_e125532_d_n9, assign82560_e125532_d_n10, assign82560_e125532_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82560_e125532;
        locals.var_dnm_dn0 = assign82560_e125532_d_n0;
        locals.var_dnm_dn2 = assign82560_e125532_d_n2;
        locals.var_dnm_dn4 = assign82560_e125532_d_n4;
        locals.var_dnm_dn5 = assign82560_e125532_d_n5;
        locals.var_dnm_dn6 = assign82560_e125532_d_n6;
        locals.var_dnm_dn7 = assign82560_e125532_d_n7;
        locals.var_dnm_dn8 = assign82560_e125532_d_n8;
        locals.var_dnm_dn9 = assign82560_e125532_d_n9;
        locals.var_dnm_dn10 = assign82560_e125532_d_n10;
        locals.var_dnm_dn13 = assign82560_e125532_d_n13;

        let assign82570_e125547: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1915 = assign82570_e125547;

        let assign82580_e125550: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1916 = assign82580_e125550;

        let (assign82590_e125567,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82590_e125567;

        let assign82600_e125570: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1917 = assign82600_e125570;

        let (assign82610_e125590,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82610_e125590;

        let assign82620_e125593: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1918 = assign82620_e125593;

        let (assign82630_e125616,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 == 0.0)) && (locals.var_guard1918 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82630_e125616;

        let assign82640_e125619: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1919 = assign82640_e125619;

        let (assign82650_e125645,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) && (locals.var_guard1917 == 0.0)) && (locals.var_guard1918 == 0.0)) && (locals.var_guard1919 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82650_e125645;

        let (assign82660_e125660,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82660_e125660;

        let mut assign82670_loop_guard: usize = 0;
        while {
            let assign82670_cond_e125676: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign82670_cond_e125676 != 0.0
        } {
            assign82670_loop_guard += 1;
            assert!(assign82670_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign82670_body0_e125692, assign82670_body0_e125692_d_n0, assign82670_body0_e125692_d_n2, assign82670_body0_e125692_d_n4, assign82670_body0_e125692_d_n5, assign82670_body0_e125692_d_n6, assign82670_body0_e125692_d_n7, assign82670_body0_e125692_d_n8, assign82670_body0_e125692_d_n9, assign82670_body0_e125692_d_n10, assign82670_body0_e125692_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        let assign82670_body0_e125690: f64 = (locals.var_dnm).sqrt();
        (assign82670_body0_e125690, (locals.var_dnm_dn0 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn2 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn4 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn5 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn6 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn7 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn8 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn9 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn10 / (2.0 * assign82670_body0_e125690)), (locals.var_dnm_dn13 / (2.0 * assign82670_body0_e125690)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign82670_body0_e125692;
            locals.var_dnm_dn0 = assign82670_body0_e125692_d_n0;
            locals.var_dnm_dn2 = assign82670_body0_e125692_d_n2;
            locals.var_dnm_dn4 = assign82670_body0_e125692_d_n4;
            locals.var_dnm_dn5 = assign82670_body0_e125692_d_n5;
            locals.var_dnm_dn6 = assign82670_body0_e125692_d_n6;
            locals.var_dnm_dn7 = assign82670_body0_e125692_d_n7;
            locals.var_dnm_dn8 = assign82670_body0_e125692_d_n8;
            locals.var_dnm_dn9 = assign82670_body0_e125692_d_n9;
            locals.var_dnm_dn10 = assign82670_body0_e125692_d_n10;
            locals.var_dnm_dn13 = assign82670_body0_e125692_d_n13;
            let (assign82670_body1_e125709,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 != 0.0)) {
        let assign82670_body1_e125707: f64 = (locals.var_m0 + 1.0);
        (assign82670_body1_e125707,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign82670_body1_e125709;
        }

        let (assign82680_e125736, assign82680_e125736_d_n0, assign82680_e125736_d_n2, assign82680_e125736_d_n4, assign82680_e125736_d_n5, assign82680_e125736_d_n6, assign82680_e125736_d_n7, assign82680_e125736_d_n8, assign82680_e125736_d_n9, assign82680_e125736_d_n10, assign82680_e125736_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) && (locals.var_guard1915 == 0.0)) {
        let (assign82680_e125734, assign82680_e125734_d_n0, assign82680_e125734_d_n2, assign82680_e125734_d_n4, assign82680_e125734_d_n5, assign82680_e125734_d_n6, assign82680_e125734_d_n7, assign82680_e125734_d_n8, assign82680_e125734_d_n9, assign82680_e125734_d_n10, assign82680_e125734_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign82680_e125731: f64 = (2.0 * 2.0);
                let assign82680_e125732: f64 = (1.0 / assign82680_e125731);
                let assign82680_e125733: f64 = (locals.var_dnm).powf(assign82680_e125732);
                (assign82680_e125733, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn0)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn2)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn4)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn5)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn6)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn7)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn8)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn9)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn10)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82680_e125732) as f64).is_finite() && ((assign82680_e125732) as f64).fract() == 0.0 { if assign82680_e125732 == 0.0 { 0.0 } else { (assign82680_e125732 * ((locals.var_dnm).powf(assign82680_e125732 - 1.0) * locals.var_dnm_dn13)) } } else { (assign82680_e125733 * (assign82680_e125732 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign82680_e125734, assign82680_e125734_d_n0, assign82680_e125734_d_n2, assign82680_e125734_d_n4, assign82680_e125734_d_n5, assign82680_e125734_d_n6, assign82680_e125734_d_n7, assign82680_e125734_d_n8, assign82680_e125734_d_n9, assign82680_e125734_d_n10, assign82680_e125734_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82680_e125736;
        locals.var_dnm_dn0 = assign82680_e125736_d_n0;
        locals.var_dnm_dn2 = assign82680_e125736_d_n2;
        locals.var_dnm_dn4 = assign82680_e125736_d_n4;
        locals.var_dnm_dn5 = assign82680_e125736_d_n5;
        locals.var_dnm_dn6 = assign82680_e125736_d_n6;
        locals.var_dnm_dn7 = assign82680_e125736_d_n7;
        locals.var_dnm_dn8 = assign82680_e125736_d_n8;
        locals.var_dnm_dn9 = assign82680_e125736_d_n9;
        locals.var_dnm_dn10 = assign82680_e125736_d_n10;
        locals.var_dnm_dn13 = assign82680_e125736_d_n13;

        let (assign82690_e125751, assign82690_e125751_d_n0, assign82690_e125751_d_n2, assign82690_e125751_d_n4, assign82690_e125751_d_n5, assign82690_e125751_d_n6, assign82690_e125751_d_n7, assign82690_e125751_d_n8, assign82690_e125751_d_n9, assign82690_e125751_d_n10, assign82690_e125751_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82690_e125749: f64 = (1.0 / locals.var_dnm);
        (assign82690_e125749, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign82690_e125751;
        locals.var_dnm_dn0 = assign82690_e125751_d_n0;
        locals.var_dnm_dn2 = assign82690_e125751_d_n2;
        locals.var_dnm_dn4 = assign82690_e125751_d_n4;
        locals.var_dnm_dn5 = assign82690_e125751_d_n5;
        locals.var_dnm_dn6 = assign82690_e125751_d_n6;
        locals.var_dnm_dn7 = assign82690_e125751_d_n7;
        locals.var_dnm_dn8 = assign82690_e125751_d_n8;
        locals.var_dnm_dn9 = assign82690_e125751_d_n9;
        locals.var_dnm_dn10 = assign82690_e125751_d_n10;
        locals.var_dnm_dn13 = assign82690_e125751_d_n13;

        let (assign82700_e125770, assign82700_e125770_d_n0, assign82700_e125770_d_n2, assign82700_e125770_d_n4, assign82700_e125770_d_n5, assign82700_e125770_d_n6, assign82700_e125770_d_n7, assign82700_e125770_d_n8, assign82700_e125770_d_n9, assign82700_e125770_d_n10, assign82700_e125770_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82700_e125765: f64 = (0.2 * locals.var_chi_b);
        let assign82700_e125766: f64 = (locals.var_tmf1 * assign82700_e125765);
        let assign82700_e125768: f64 = (assign82700_e125766 * locals.var_dnm);
        (assign82700_e125768, ((((locals.var_tmf1_dn0 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign82700_e125765) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign82700_e125766 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign82700_e125770;
        locals.var_tmf0_dn0 = assign82700_e125770_d_n0;
        locals.var_tmf0_dn2 = assign82700_e125770_d_n2;
        locals.var_tmf0_dn4 = assign82700_e125770_d_n4;
        locals.var_tmf0_dn5 = assign82700_e125770_d_n5;
        locals.var_tmf0_dn6 = assign82700_e125770_d_n6;
        locals.var_tmf0_dn7 = assign82700_e125770_d_n7;
        locals.var_tmf0_dn8 = assign82700_e125770_d_n8;
        locals.var_tmf0_dn9 = assign82700_e125770_d_n9;
        locals.var_tmf0_dn10 = assign82700_e125770_d_n10;
        locals.var_tmf0_dn13 = assign82700_e125770_d_n13;

        let (assign82710_e125791, assign82710_e125791_d_n0, assign82710_e125791_d_n2, assign82710_e125791_d_n4, assign82710_e125791_d_n5, assign82710_e125791_d_n6, assign82710_e125791_d_n7, assign82710_e125791_d_n8, assign82710_e125791_d_n9, assign82710_e125791_d_n10, assign82710_e125791_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82710_e125783: f64 = (0.2 * locals.var_chi_b);
        let assign82710_e125785: f64 = (assign82710_e125783 * locals.var_xmp);
        let assign82710_e125787: f64 = (assign82710_e125785 * locals.var_dnm);
        let assign82710_e125789: f64 = (assign82710_e125787 / locals.var_arg);
        (assign82710_e125789, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn0)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn2)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn4)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn5)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn6)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn7)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn8)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn9)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn10)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign82710_e125783 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign82710_e125785 * locals.var_dnm_dn13)) * locals.var_arg) - (assign82710_e125787 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82710_e125791;
        locals.var_t1_dn0 = assign82710_e125791_d_n0;
        locals.var_t1_dn2 = assign82710_e125791_d_n2;
        locals.var_t1_dn4 = assign82710_e125791_d_n4;
        locals.var_t1_dn5 = assign82710_e125791_d_n5;
        locals.var_t1_dn6 = assign82710_e125791_d_n6;
        locals.var_t1_dn7 = assign82710_e125791_d_n7;
        locals.var_t1_dn8 = assign82710_e125791_d_n8;
        locals.var_t1_dn9 = assign82710_e125791_d_n9;
        locals.var_t1_dn10 = assign82710_e125791_d_n10;
        locals.var_t1_dn13 = assign82710_e125791_d_n13;

    }
}
