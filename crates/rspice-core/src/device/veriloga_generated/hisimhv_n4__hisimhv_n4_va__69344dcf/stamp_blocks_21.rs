#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_336(
        locals: &mut StampLocals,
    ) {
        let (assign94620_e146316, assign94620_e146316_d_n0, assign94620_e146316_d_n2, assign94620_e146316_d_n4, assign94620_e146316_d_n5, assign94620_e146316_d_n6, assign94620_e146316_d_n7, assign94620_e146316_d_n8, assign94620_e146316_d_n9, assign94620_e146316_d_n10, assign94620_e146316_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94620_e146313: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94620_e146314: f64 = (0.5 * assign94620_e146313);
        (assign94620_e146314, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94620_e146316;
        locals.var_t2_dn0 = assign94620_e146316_d_n0;
        locals.var_t2_dn2 = assign94620_e146316_d_n2;
        locals.var_t2_dn4 = assign94620_e146316_d_n4;
        locals.var_t2_dn5 = assign94620_e146316_d_n5;
        locals.var_t2_dn6 = assign94620_e146316_d_n6;
        locals.var_t2_dn7 = assign94620_e146316_d_n7;
        locals.var_t2_dn8 = assign94620_e146316_d_n8;
        locals.var_t2_dn9 = assign94620_e146316_d_n9;
        locals.var_t2_dn10 = assign94620_e146316_d_n10;
        locals.var_t2_dn13 = assign94620_e146316_d_n13;

        let assign94630_e146319: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2199 = assign94630_e146319;

        let (assign94640_e146329, assign94640_e146329_d_n0, assign94640_e146329_d_n2, assign94640_e146329_d_n4, assign94640_e146329_d_n5, assign94640_e146329_d_n6, assign94640_e146329_d_n7, assign94640_e146329_d_n8, assign94640_e146329_d_n9, assign94640_e146329_d_n10, assign94640_e146329_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94640_e146329;
        locals.var_t2_dn0 = assign94640_e146329_d_n0;
        locals.var_t2_dn2 = assign94640_e146329_d_n2;
        locals.var_t2_dn4 = assign94640_e146329_d_n4;
        locals.var_t2_dn5 = assign94640_e146329_d_n5;
        locals.var_t2_dn6 = assign94640_e146329_d_n6;
        locals.var_t2_dn7 = assign94640_e146329_d_n7;
        locals.var_t2_dn8 = assign94640_e146329_d_n8;
        locals.var_t2_dn9 = assign94640_e146329_d_n9;
        locals.var_t2_dn10 = assign94640_e146329_d_n10;
        locals.var_t2_dn13 = assign94640_e146329_d_n13;

        let (assign94650_e146339, assign94650_e146339_d_n0, assign94650_e146339_d_n2, assign94650_e146339_d_n4, assign94650_e146339_d_n5, assign94650_e146339_d_n6, assign94650_e146339_d_n7, assign94650_e146339_d_n8, assign94650_e146339_d_n9, assign94650_e146339_d_n10, assign94650_e146339_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94650_e146339;
        locals.var_t9_dn0 = assign94650_e146339_d_n0;
        locals.var_t9_dn2 = assign94650_e146339_d_n2;
        locals.var_t9_dn4 = assign94650_e146339_d_n4;
        locals.var_t9_dn5 = assign94650_e146339_d_n5;
        locals.var_t9_dn6 = assign94650_e146339_d_n6;
        locals.var_t9_dn7 = assign94650_e146339_d_n7;
        locals.var_t9_dn8 = assign94650_e146339_d_n8;
        locals.var_t9_dn9 = assign94650_e146339_d_n9;
        locals.var_t9_dn10 = assign94650_e146339_d_n10;
        locals.var_t9_dn13 = assign94650_e146339_d_n13;

        let (assign94660_e146347, assign94660_e146347_d_n0, assign94660_e146347_d_n2, assign94660_e146347_d_n4, assign94660_e146347_d_n5, assign94660_e146347_d_n6, assign94660_e146347_d_n7, assign94660_e146347_d_n8, assign94660_e146347_d_n9, assign94660_e146347_d_n10, assign94660_e146347_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign94660_e146347;
        locals.var_ddriftldc_dn0 = assign94660_e146347_d_n0;
        locals.var_ddriftldc_dn2 = assign94660_e146347_d_n2;
        locals.var_ddriftldc_dn4 = assign94660_e146347_d_n4;
        locals.var_ddriftldc_dn5 = assign94660_e146347_d_n5;
        locals.var_ddriftldc_dn6 = assign94660_e146347_d_n6;
        locals.var_ddriftldc_dn7 = assign94660_e146347_d_n7;
        locals.var_ddriftldc_dn8 = assign94660_e146347_d_n8;
        locals.var_ddriftldc_dn9 = assign94660_e146347_d_n9;
        locals.var_ddriftldc_dn10 = assign94660_e146347_d_n10;
        locals.var_ddriftldc_dn13 = assign94660_e146347_d_n13;

        let (assign94670_e146363, assign94670_e146363_d_n0, assign94670_e146363_d_n2, assign94670_e146363_d_n4, assign94670_e146363_d_n5, assign94670_e146363_d_n6, assign94670_e146363_d_n7, assign94670_e146363_d_n8, assign94670_e146363_d_n9, assign94670_e146363_d_n10, assign94670_e146363_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94670_e146355: f64 = (locals.var_q_nsubld__blk2115 * locals.var_ddriftldc);
        let assign94670_e146357: f64 = (assign94670_e146355 * locals.var_ddriftldc);
        let assign94670_e146359: f64 = (assign94670_e146357 / 2.0);
        let assign94670_e146361: f64 = (assign94670_e146359 / 1.034943e-10);
        (assign94670_e146361, (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign94670_e146363;
        locals.var_dphi_sb_dn0 = assign94670_e146363_d_n0;
        locals.var_dphi_sb_dn2 = assign94670_e146363_d_n2;
        locals.var_dphi_sb_dn4 = assign94670_e146363_d_n4;
        locals.var_dphi_sb_dn5 = assign94670_e146363_d_n5;
        locals.var_dphi_sb_dn6 = assign94670_e146363_d_n6;
        locals.var_dphi_sb_dn7 = assign94670_e146363_d_n7;
        locals.var_dphi_sb_dn8 = assign94670_e146363_d_n8;
        locals.var_dphi_sb_dn9 = assign94670_e146363_d_n9;
        locals.var_dphi_sb_dn10 = assign94670_e146363_d_n10;
        locals.var_dphi_sb_dn13 = assign94670_e146363_d_n13;

        let (assign94680_e146376, assign94680_e146376_d_n0, assign94680_e146376_d_n2, assign94680_e146376_d_n4, assign94680_e146376_d_n5, assign94680_e146376_d_n6, assign94680_e146376_d_n7, assign94680_e146376_d_n8, assign94680_e146376_d_n9, assign94680_e146376_d_n10, assign94680_e146376_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94680_e146371: f64 = (2.0 * locals.var_beta);
        let assign94680_e146373: f64 = (assign94680_e146371 * locals.var_dphi_sb);
        let assign94680_e146374: f64 = (assign94680_e146373).sqrt();
        (assign94680_e146374, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn0)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn2)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn4)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn5)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn6)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn7)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn8)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn9)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn10)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn13)) / (2.0 * assign94680_e146374)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94680_e146376;
        locals.var_t0_dn0 = assign94680_e146376_d_n0;
        locals.var_t0_dn2 = assign94680_e146376_d_n2;
        locals.var_t0_dn4 = assign94680_e146376_d_n4;
        locals.var_t0_dn5 = assign94680_e146376_d_n5;
        locals.var_t0_dn6 = assign94680_e146376_d_n6;
        locals.var_t0_dn7 = assign94680_e146376_d_n7;
        locals.var_t0_dn8 = assign94680_e146376_d_n8;
        locals.var_t0_dn9 = assign94680_e146376_d_n9;
        locals.var_t0_dn10 = assign94680_e146376_d_n10;
        locals.var_t0_dn13 = assign94680_e146376_d_n13;

        let (assign94690_e146391, assign94690_e146391_d_n0, assign94690_e146391_d_n2, assign94690_e146391_d_n4, assign94690_e146391_d_n5, assign94690_e146391_d_n6, assign94690_e146391_d_n7, assign94690_e146391_d_n8, assign94690_e146391_d_n9, assign94690_e146391_d_n10, assign94690_e146391_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94690_e146383: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94690_e146385: f64 = (-locals.var_t0);
        let assign94690_e146386: f64 = { let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94690_e146387: f64 = (assign94690_e146383 + assign94690_e146386);
        let assign94690_e146389: f64 = (assign94690_e146387 / 2.0);
        (assign94690_e146389, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94690_e146391;
        locals.var_t1_dn0 = assign94690_e146391_d_n0;
        locals.var_t1_dn2 = assign94690_e146391_d_n2;
        locals.var_t1_dn4 = assign94690_e146391_d_n4;
        locals.var_t1_dn5 = assign94690_e146391_d_n5;
        locals.var_t1_dn6 = assign94690_e146391_d_n6;
        locals.var_t1_dn7 = assign94690_e146391_d_n7;
        locals.var_t1_dn8 = assign94690_e146391_d_n8;
        locals.var_t1_dn9 = assign94690_e146391_d_n9;
        locals.var_t1_dn10 = assign94690_e146391_d_n10;
        locals.var_t1_dn13 = assign94690_e146391_d_n13;

        let (assign94700_e146402, assign94700_e146402_d_n0, assign94700_e146402_d_n2, assign94700_e146402_d_n4, assign94700_e146402_d_n5, assign94700_e146402_d_n6, assign94700_e146402_d_n7, assign94700_e146402_d_n8, assign94700_e146402_d_n9, assign94700_e146402_d_n10, assign94700_e146402_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94700_e146398: f64 = (locals.var_t1).ln();
        let assign94700_e146400: f64 = (assign94700_e146398 / locals.var_dphi_sb);
        (assign94700_e146400, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign94700_e146402;
        locals.var_c_sb_dn0 = assign94700_e146402_d_n0;
        locals.var_c_sb_dn2 = assign94700_e146402_d_n2;
        locals.var_c_sb_dn4 = assign94700_e146402_d_n4;
        locals.var_c_sb_dn5 = assign94700_e146402_d_n5;
        locals.var_c_sb_dn6 = assign94700_e146402_d_n6;
        locals.var_c_sb_dn7 = assign94700_e146402_d_n7;
        locals.var_c_sb_dn8 = assign94700_e146402_d_n8;
        locals.var_c_sb_dn9 = assign94700_e146402_d_n9;
        locals.var_c_sb_dn10 = assign94700_e146402_d_n10;
        locals.var_c_sb_dn13 = assign94700_e146402_d_n13;

        let (assign94710_e146410,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94710_e146410;

    }

    pub(super) fn stamp_transient_block_337(
        locals: &mut StampLocals,
    ) {
        let mut assign94720_loop_guard: usize = 0;
        while {
            let assign94720_cond_e146419: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94720_cond_e146421: f64 = if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_lp_s0 <= assign94720_cond_e146419)) { 1.0 } else { 0.0 };
            assign94720_cond_e146421 != 0.0
        } {
            assign94720_loop_guard += 1;
            assert!(assign94720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94720_body3_e146454, assign94720_body3_e146454_d_n0, assign94720_body3_e146454_d_n2, assign94720_body3_e146454_d_n4, assign94720_body3_e146454_d_n5, assign94720_body3_e146454_d_n6, assign94720_body3_e146454_d_n7, assign94720_body3_e146454_d_n8, assign94720_body3_e146454_d_n9, assign94720_body3_e146454_d_n10, assign94720_body3_e146454_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body3_e146452: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94720_body3_e146452, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign94720_body3_e146454;
            locals.var_ps0ld_vxb_dn0 = assign94720_body3_e146454_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94720_body3_e146454_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94720_body3_e146454_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94720_body3_e146454_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94720_body3_e146454_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94720_body3_e146454_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94720_body3_e146454_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94720_body3_e146454_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94720_body3_e146454_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign94720_body3_e146454_d_n13;
            let (assign94720_body4_e146464, assign94720_body4_e146464_d_n0, assign94720_body4_e146464_d_n2, assign94720_body4_e146464_d_n4, assign94720_body4_e146464_d_n5, assign94720_body4_e146464_d_n6, assign94720_body4_e146464_d_n7, assign94720_body4_e146464_d_n8, assign94720_body4_e146464_d_n9, assign94720_body4_e146464_d_n10, assign94720_body4_e146464_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body4_e146462: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94720_body4_e146462, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign94720_body4_e146464;
            locals.var_chi_dn0 = assign94720_body4_e146464_d_n0;
            locals.var_chi_dn2 = assign94720_body4_e146464_d_n2;
            locals.var_chi_dn4 = assign94720_body4_e146464_d_n4;
            locals.var_chi_dn5 = assign94720_body4_e146464_d_n5;
            locals.var_chi_dn6 = assign94720_body4_e146464_d_n6;
            locals.var_chi_dn7 = assign94720_body4_e146464_d_n7;
            locals.var_chi_dn8 = assign94720_body4_e146464_d_n8;
            locals.var_chi_dn9 = assign94720_body4_e146464_d_n9;
            locals.var_chi_dn10 = assign94720_body4_e146464_d_n10;
            locals.var_chi_dn13 = assign94720_body4_e146464_d_n13;
            let (assign94720_body5_e146476, assign94720_body5_e146476_d_n0, assign94720_body5_e146476_d_n2, assign94720_body5_e146476_d_n4, assign94720_body5_e146476_d_n5, assign94720_body5_e146476_d_n6, assign94720_body5_e146476_d_n7, assign94720_body5_e146476_d_n8, assign94720_body5_e146476_d_n9, assign94720_body5_e146476_d_n10, assign94720_body5_e146476_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body5_e146473: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94720_body5_e146474: f64 = (locals.var_c_sb * assign94720_body5_e146473);
        (assign94720_body5_e146474, ((locals.var_c_sb_dn0 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign94720_body5_e146476;
            locals.var_ty_dn0 = assign94720_body5_e146476_d_n0;
            locals.var_ty_dn2 = assign94720_body5_e146476_d_n2;
            locals.var_ty_dn4 = assign94720_body5_e146476_d_n4;
            locals.var_ty_dn5 = assign94720_body5_e146476_d_n5;
            locals.var_ty_dn6 = assign94720_body5_e146476_d_n6;
            locals.var_ty_dn7 = assign94720_body5_e146476_d_n7;
            locals.var_ty_dn8 = assign94720_body5_e146476_d_n8;
            locals.var_ty_dn9 = assign94720_body5_e146476_d_n9;
            locals.var_ty_dn10 = assign94720_body5_e146476_d_n10;
            locals.var_ty_dn13 = assign94720_body5_e146476_d_n13;
            let assign94720_body6_e146479: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2201 = assign94720_body6_e146479;
            let (assign94720_body7_e146490, assign94720_body7_e146490_d_n0, assign94720_body7_e146490_d_n2, assign94720_body7_e146490_d_n4, assign94720_body7_e146490_d_n5, assign94720_body7_e146490_d_n6, assign94720_body7_e146490_d_n7, assign94720_body7_e146490_d_n8, assign94720_body7_e146490_d_n9, assign94720_body7_e146490_d_n10, assign94720_body7_e146490_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body7_e146488: f64 = (locals.var_ty).exp();
        (assign94720_body7_e146488, (assign94720_body7_e146488 * locals.var_ty_dn0), (assign94720_body7_e146488 * locals.var_ty_dn2), (assign94720_body7_e146488 * locals.var_ty_dn4), (assign94720_body7_e146488 * locals.var_ty_dn5), (assign94720_body7_e146488 * locals.var_ty_dn6), (assign94720_body7_e146488 * locals.var_ty_dn7), (assign94720_body7_e146488 * locals.var_ty_dn8), (assign94720_body7_e146488 * locals.var_ty_dn9), (assign94720_body7_e146488 * locals.var_ty_dn10), (assign94720_body7_e146488 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body7_e146490;
            locals.var_t1_dn0 = assign94720_body7_e146490_d_n0;
            locals.var_t1_dn2 = assign94720_body7_e146490_d_n2;
            locals.var_t1_dn4 = assign94720_body7_e146490_d_n4;
            locals.var_t1_dn5 = assign94720_body7_e146490_d_n5;
            locals.var_t1_dn6 = assign94720_body7_e146490_d_n6;
            locals.var_t1_dn7 = assign94720_body7_e146490_d_n7;
            locals.var_t1_dn8 = assign94720_body7_e146490_d_n8;
            locals.var_t1_dn9 = assign94720_body7_e146490_d_n9;
            locals.var_t1_dn10 = assign94720_body7_e146490_d_n10;
            locals.var_t1_dn13 = assign94720_body7_e146490_d_n13;
            let (assign94720_body8_e146504, assign94720_body8_e146504_d_n0, assign94720_body8_e146504_d_n2, assign94720_body8_e146504_d_n4, assign94720_body8_e146504_d_n5, assign94720_body8_e146504_d_n6, assign94720_body8_e146504_d_n7, assign94720_body8_e146504_d_n8, assign94720_body8_e146504_d_n9, assign94720_body8_e146504_d_n10, assign94720_body8_e146504_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body8_e146499: f64 = (-locals.var_c_sb);
        let assign94720_body8_e146501: f64 = (assign94720_body8_e146499 * locals.var_dphi_sb);
        let assign94720_body8_e146502: f64 = (assign94720_body8_e146501).exp();
        (assign94720_body8_e146502, (assign94720_body8_e146502 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn0))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn2))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn4))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn5))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn6))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn7))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn8))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn9))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn10))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body8_e146504;
            locals.var_t0_dn0 = assign94720_body8_e146504_d_n0;
            locals.var_t0_dn2 = assign94720_body8_e146504_d_n2;
            locals.var_t0_dn4 = assign94720_body8_e146504_d_n4;
            locals.var_t0_dn5 = assign94720_body8_e146504_d_n5;
            locals.var_t0_dn6 = assign94720_body8_e146504_d_n6;
            locals.var_t0_dn7 = assign94720_body8_e146504_d_n7;
            locals.var_t0_dn8 = assign94720_body8_e146504_d_n8;
            locals.var_t0_dn9 = assign94720_body8_e146504_d_n9;
            locals.var_t0_dn10 = assign94720_body8_e146504_d_n10;
            locals.var_t0_dn13 = assign94720_body8_e146504_d_n13;
            let (assign94720_body9_e146516, assign94720_body9_e146516_d_n0, assign94720_body9_e146516_d_n2, assign94720_body9_e146516_d_n4, assign94720_body9_e146516_d_n5, assign94720_body9_e146516_d_n6, assign94720_body9_e146516_d_n7, assign94720_body9_e146516_d_n8, assign94720_body9_e146516_d_n9, assign94720_body9_e146516_d_n10, assign94720_body9_e146516_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body9_e146514: f64 = (locals.var_t1 - locals.var_t0);
        (assign94720_body9_e146514, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94720_body9_e146516;
            locals.var_t2_dn0 = assign94720_body9_e146516_d_n0;
            locals.var_t2_dn2 = assign94720_body9_e146516_d_n2;
            locals.var_t2_dn4 = assign94720_body9_e146516_d_n4;
            locals.var_t2_dn5 = assign94720_body9_e146516_d_n5;
            locals.var_t2_dn6 = assign94720_body9_e146516_d_n6;
            locals.var_t2_dn7 = assign94720_body9_e146516_d_n7;
            locals.var_t2_dn8 = assign94720_body9_e146516_d_n8;
            locals.var_t2_dn9 = assign94720_body9_e146516_d_n9;
            locals.var_t2_dn10 = assign94720_body9_e146516_d_n10;
            locals.var_t2_dn13 = assign94720_body9_e146516_d_n13;
            let (assign94720_body10_e146531, assign94720_body10_e146531_d_n0, assign94720_body10_e146531_d_n2, assign94720_body10_e146531_d_n4, assign94720_body10_e146531_d_n5, assign94720_body10_e146531_d_n6, assign94720_body10_e146531_d_n7, assign94720_body10_e146531_d_n8, assign94720_body10_e146531_d_n9, assign94720_body10_e146531_d_n10, assign94720_body10_e146531_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body10_e146526: f64 = (1.0 + locals.var_t2);
        let assign94720_body10_e146527: f64 = (assign94720_body10_e146526).ln();
        let assign94720_body10_e146529: f64 = (assign94720_body10_e146527 / locals.var_c_sb);
        (assign94720_body10_e146529, ((((locals.var_t2_dn0 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94720_body10_e146531;
            locals.var_phi_b_dn0 = assign94720_body10_e146531_d_n0;
            locals.var_phi_b_dn2 = assign94720_body10_e146531_d_n2;
            locals.var_phi_b_dn4 = assign94720_body10_e146531_d_n4;
            locals.var_phi_b_dn5 = assign94720_body10_e146531_d_n5;
            locals.var_phi_b_dn6 = assign94720_body10_e146531_d_n6;
            locals.var_phi_b_dn7 = assign94720_body10_e146531_d_n7;
            locals.var_phi_b_dn8 = assign94720_body10_e146531_d_n8;
            locals.var_phi_b_dn9 = assign94720_body10_e146531_d_n9;
            locals.var_phi_b_dn10 = assign94720_body10_e146531_d_n10;
            locals.var_phi_b_dn13 = assign94720_body10_e146531_d_n13;
            let (assign94720_body11_e146545, assign94720_body11_e146545_d_n0, assign94720_body11_e146545_d_n2, assign94720_body11_e146545_d_n4, assign94720_body11_e146545_d_n5, assign94720_body11_e146545_d_n6, assign94720_body11_e146545_d_n7, assign94720_body11_e146545_d_n8, assign94720_body11_e146545_d_n9, assign94720_body11_e146545_d_n10, assign94720_body11_e146545_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body11_e146542: f64 = (1.0 + locals.var_t2);
        let assign94720_body11_e146543: f64 = (locals.var_t1 / assign94720_body11_e146542);
        (assign94720_body11_e146543, (((locals.var_t1_dn0 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn2 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn4 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn5 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn6 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn7 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn8 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn9 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn10 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn13 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn13)) / (assign94720_body11_e146542 * assign94720_body11_e146542)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94720_body11_e146545;
            locals.var_phi_b_dpss_dn0 = assign94720_body11_e146545_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94720_body11_e146545_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94720_body11_e146545_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94720_body11_e146545_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94720_body11_e146545_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94720_body11_e146545_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94720_body11_e146545_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94720_body11_e146545_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94720_body11_e146545_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94720_body11_e146545_d_n13;
            let (assign94720_body12_e146558, assign94720_body12_e146558_d_n0, assign94720_body12_e146558_d_n2, assign94720_body12_e146558_d_n4, assign94720_body12_e146558_d_n5, assign94720_body12_e146558_d_n6, assign94720_body12_e146558_d_n7, assign94720_body12_e146558_d_n8, assign94720_body12_e146558_d_n9, assign94720_body12_e146558_d_n10, assign94720_body12_e146558_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94720_body12_e146556: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94720_body12_e146556, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94720_body12_e146558;
            locals.var_phi_b_dn0 = assign94720_body12_e146558_d_n0;
            locals.var_phi_b_dn2 = assign94720_body12_e146558_d_n2;
            locals.var_phi_b_dn4 = assign94720_body12_e146558_d_n4;
            locals.var_phi_b_dn5 = assign94720_body12_e146558_d_n5;
            locals.var_phi_b_dn6 = assign94720_body12_e146558_d_n6;
            locals.var_phi_b_dn7 = assign94720_body12_e146558_d_n7;
            locals.var_phi_b_dn8 = assign94720_body12_e146558_d_n8;
            locals.var_phi_b_dn9 = assign94720_body12_e146558_d_n9;
            locals.var_phi_b_dn10 = assign94720_body12_e146558_d_n10;
            locals.var_phi_b_dn13 = assign94720_body12_e146558_d_n13;
            let (assign94720_body13_e146569, assign94720_body13_e146569_d_n0, assign94720_body13_e146569_d_n2, assign94720_body13_e146569_d_n4, assign94720_body13_e146569_d_n5, assign94720_body13_e146569_d_n6, assign94720_body13_e146569_d_n7, assign94720_body13_e146569_d_n8, assign94720_body13_e146569_d_n9, assign94720_body13_e146569_d_n10, assign94720_body13_e146569_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94720_body13_e146569;
            locals.var_phi_b_dpss_dn0 = assign94720_body13_e146569_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94720_body13_e146569_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94720_body13_e146569_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94720_body13_e146569_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94720_body13_e146569_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94720_body13_e146569_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94720_body13_e146569_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94720_body13_e146569_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94720_body13_e146569_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94720_body13_e146569_d_n13;
            let (assign94720_body14_e146579, assign94720_body14_e146579_d_n0, assign94720_body14_e146579_d_n2, assign94720_body14_e146579_d_n4, assign94720_body14_e146579_d_n5, assign94720_body14_e146579_d_n6, assign94720_body14_e146579_d_n7, assign94720_body14_e146579_d_n8, assign94720_body14_e146579_d_n9, assign94720_body14_e146579_d_n10, assign94720_body14_e146579_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body14_e146577: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94720_body14_e146577, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign94720_body14_e146579;
            locals.var_chib_dn0 = assign94720_body14_e146579_d_n0;
            locals.var_chib_dn2 = assign94720_body14_e146579_d_n2;
            locals.var_chib_dn4 = assign94720_body14_e146579_d_n4;
            locals.var_chib_dn5 = assign94720_body14_e146579_d_n5;
            locals.var_chib_dn6 = assign94720_body14_e146579_d_n6;
            locals.var_chib_dn7 = assign94720_body14_e146579_d_n7;
            locals.var_chib_dn8 = assign94720_body14_e146579_d_n8;
            locals.var_chib_dn9 = assign94720_body14_e146579_d_n9;
            locals.var_chib_dn10 = assign94720_body14_e146579_d_n10;
            locals.var_chib_dn13 = assign94720_body14_e146579_d_n13;
            let assign94720_body15_e146581: f64 = (locals.var_chi).abs();
            let assign94720_body15_e146583: f64 = if assign94720_body15_e146581 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2202 = assign94720_body15_e146583;
            let (assign94720_body17_e146633, assign94720_body17_e146633_d_n0, assign94720_body17_e146633_d_n2, assign94720_body17_e146633_d_n4, assign94720_body17_e146633_d_n5, assign94720_body17_e146633_d_n6, assign94720_body17_e146633_d_n7, assign94720_body17_e146633_d_n8, assign94720_body17_e146633_d_n9, assign94720_body17_e146633_d_n10, assign94720_body17_e146633_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body17_e146611: f64 = (locals.var_chi * locals.var_chi);
        let assign94720_body17_e146613: f64 = (assign94720_body17_e146611 / 2.0);
        let assign94720_body17_e146617: f64 = (locals.var_chi / 3.0);
        let assign94720_body17_e146621: f64 = (locals.var_chi / 4.0);
        let assign94720_body17_e146625: f64 = (locals.var_chi / 5.0);
        let assign94720_body17_e146626: f64 = (1.0 - assign94720_body17_e146625);
        let assign94720_body17_e146627: f64 = (assign94720_body17_e146621 * assign94720_body17_e146626);
        let assign94720_body17_e146628: f64 = (1.0 - assign94720_body17_e146627);
        let assign94720_body17_e146629: f64 = (assign94720_body17_e146617 * assign94720_body17_e146628);
        let assign94720_body17_e146630: f64 = (1.0 - assign94720_body17_e146629);
        let assign94720_body17_e146631: f64 = (assign94720_body17_e146613 * assign94720_body17_e146630);
        (assign94720_body17_e146631, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn0 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn0 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn2 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn2 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn4 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn4 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn5 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn5 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn6 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn6 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn7 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn7 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn8 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn8 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn9 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn9 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn10 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn10 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn13 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn13 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body17_e146633;
            locals.var_t0_dn0 = assign94720_body17_e146633_d_n0;
            locals.var_t0_dn2 = assign94720_body17_e146633_d_n2;
            locals.var_t0_dn4 = assign94720_body17_e146633_d_n4;
            locals.var_t0_dn5 = assign94720_body17_e146633_d_n5;
            locals.var_t0_dn6 = assign94720_body17_e146633_d_n6;
            locals.var_t0_dn7 = assign94720_body17_e146633_d_n7;
            locals.var_t0_dn8 = assign94720_body17_e146633_d_n8;
            locals.var_t0_dn9 = assign94720_body17_e146633_d_n9;
            locals.var_t0_dn10 = assign94720_body17_e146633_d_n10;
            locals.var_t0_dn13 = assign94720_body17_e146633_d_n13;
            let (assign94720_body18_e146661, assign94720_body18_e146661_d_n0, assign94720_body18_e146661_d_n2, assign94720_body18_e146661_d_n4, assign94720_body18_e146661_d_n5, assign94720_body18_e146661_d_n6, assign94720_body18_e146661_d_n7, assign94720_body18_e146661_d_n8, assign94720_body18_e146661_d_n9, assign94720_body18_e146661_d_n10, assign94720_body18_e146661_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body18_e146645: f64 = (locals.var_chi / 2.0);
        let assign94720_body18_e146649: f64 = (locals.var_chi / 3.0);
        let assign94720_body18_e146653: f64 = (locals.var_chi / 4.0);
        let assign94720_body18_e146654: f64 = (1.0 - assign94720_body18_e146653);
        let assign94720_body18_e146655: f64 = (assign94720_body18_e146649 * assign94720_body18_e146654);
        let assign94720_body18_e146656: f64 = (1.0 - assign94720_body18_e146655);
        let assign94720_body18_e146657: f64 = (assign94720_body18_e146645 * assign94720_body18_e146656);
        let assign94720_body18_e146658: f64 = (1.0 - assign94720_body18_e146657);
        let assign94720_body18_e146659: f64 = (locals.var_chi * assign94720_body18_e146658);
        (assign94720_body18_e146659, ((locals.var_chi_dn0 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn0 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn2 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn4 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn5 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn6 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn7 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn8 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn9 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn10 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn13 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body18_e146661;
            locals.var_t1_dn0 = assign94720_body18_e146661_d_n0;
            locals.var_t1_dn2 = assign94720_body18_e146661_d_n2;
            locals.var_t1_dn4 = assign94720_body18_e146661_d_n4;
            locals.var_t1_dn5 = assign94720_body18_e146661_d_n5;
            locals.var_t1_dn6 = assign94720_body18_e146661_d_n6;
            locals.var_t1_dn7 = assign94720_body18_e146661_d_n7;
            locals.var_t1_dn8 = assign94720_body18_e146661_d_n8;
            locals.var_t1_dn9 = assign94720_body18_e146661_d_n9;
            locals.var_t1_dn10 = assign94720_body18_e146661_d_n10;
            locals.var_t1_dn13 = assign94720_body18_e146661_d_n13;
            let (assign94720_body19_e146693, assign94720_body19_e146693_d_n0, assign94720_body19_e146693_d_n2, assign94720_body19_e146693_d_n4, assign94720_body19_e146693_d_n5, assign94720_body19_e146693_d_n6, assign94720_body19_e146693_d_n7, assign94720_body19_e146693_d_n8, assign94720_body19_e146693_d_n9, assign94720_body19_e146693_d_n10, assign94720_body19_e146693_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body19_e146671: f64 = (locals.var_chib * locals.var_chib);
        let assign94720_body19_e146673: f64 = (assign94720_body19_e146671 / 2.0);
        let assign94720_body19_e146677: f64 = (locals.var_chib / 3.0);
        let assign94720_body19_e146681: f64 = (locals.var_chib / 4.0);
        let assign94720_body19_e146685: f64 = (locals.var_chib / 5.0);
        let assign94720_body19_e146686: f64 = (1.0 - assign94720_body19_e146685);
        let assign94720_body19_e146687: f64 = (assign94720_body19_e146681 * assign94720_body19_e146686);
        let assign94720_body19_e146688: f64 = (1.0 - assign94720_body19_e146687);
        let assign94720_body19_e146689: f64 = (assign94720_body19_e146677 * assign94720_body19_e146688);
        let assign94720_body19_e146690: f64 = (1.0 - assign94720_body19_e146689);
        let assign94720_body19_e146691: f64 = (assign94720_body19_e146673 * assign94720_body19_e146690);
        (assign94720_body19_e146691, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn0 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn0 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn2 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn2 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn4 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn4 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn5 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn5 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn6 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn6 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn7 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn7 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn8 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn8 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn9 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn9 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn10 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn10 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn13 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn13 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94720_body19_e146693;
            locals.var_t2_dn0 = assign94720_body19_e146693_d_n0;
            locals.var_t2_dn2 = assign94720_body19_e146693_d_n2;
            locals.var_t2_dn4 = assign94720_body19_e146693_d_n4;
            locals.var_t2_dn5 = assign94720_body19_e146693_d_n5;
            locals.var_t2_dn6 = assign94720_body19_e146693_d_n6;
            locals.var_t2_dn7 = assign94720_body19_e146693_d_n7;
            locals.var_t2_dn8 = assign94720_body19_e146693_d_n8;
            locals.var_t2_dn9 = assign94720_body19_e146693_d_n9;
            locals.var_t2_dn10 = assign94720_body19_e146693_d_n10;
            locals.var_t2_dn13 = assign94720_body19_e146693_d_n13;
            let (assign94720_body20_e146721, assign94720_body20_e146721_d_n0, assign94720_body20_e146721_d_n2, assign94720_body20_e146721_d_n4, assign94720_body20_e146721_d_n5, assign94720_body20_e146721_d_n6, assign94720_body20_e146721_d_n7, assign94720_body20_e146721_d_n8, assign94720_body20_e146721_d_n9, assign94720_body20_e146721_d_n10, assign94720_body20_e146721_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body20_e146705: f64 = (locals.var_chib / 2.0);
        let assign94720_body20_e146709: f64 = (locals.var_chib / 3.0);
        let assign94720_body20_e146713: f64 = (locals.var_chib / 4.0);
        let assign94720_body20_e146714: f64 = (1.0 - assign94720_body20_e146713);
        let assign94720_body20_e146715: f64 = (assign94720_body20_e146709 * assign94720_body20_e146714);
        let assign94720_body20_e146716: f64 = (1.0 - assign94720_body20_e146715);
        let assign94720_body20_e146717: f64 = (assign94720_body20_e146705 * assign94720_body20_e146716);
        let assign94720_body20_e146718: f64 = (1.0 - assign94720_body20_e146717);
        let assign94720_body20_e146719: f64 = (locals.var_chib * assign94720_body20_e146718);
        (assign94720_body20_e146719, ((locals.var_chib_dn0 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn0 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn2 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn4 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn5 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn6 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn7 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn8 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn9 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn10 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn13 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign94720_body20_e146721;
            locals.var_t3_dn0 = assign94720_body20_e146721_d_n0;
            locals.var_t3_dn2 = assign94720_body20_e146721_d_n2;
            locals.var_t3_dn4 = assign94720_body20_e146721_d_n4;
            locals.var_t3_dn5 = assign94720_body20_e146721_d_n5;
            locals.var_t3_dn6 = assign94720_body20_e146721_d_n6;
            locals.var_t3_dn7 = assign94720_body20_e146721_d_n7;
            locals.var_t3_dn8 = assign94720_body20_e146721_d_n8;
            locals.var_t3_dn9 = assign94720_body20_e146721_d_n9;
            locals.var_t3_dn10 = assign94720_body20_e146721_d_n10;
            locals.var_t3_dn13 = assign94720_body20_e146721_d_n13;
            let (assign94720_body21_e146733, assign94720_body21_e146733_d_n0, assign94720_body21_e146733_d_n2, assign94720_body21_e146733_d_n4, assign94720_body21_e146733_d_n5, assign94720_body21_e146733_d_n6, assign94720_body21_e146733_d_n7, assign94720_body21_e146733_d_n8, assign94720_body21_e146733_d_n9, assign94720_body21_e146733_d_n10, assign94720_body21_e146733_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body21_e146731: f64 = (locals.var_t0 - locals.var_t2);
        (assign94720_body21_e146731, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk2123, locals.var_fbsq__blk2123_dn0, locals.var_fbsq__blk2123_dn2, locals.var_fbsq__blk2123_dn4, locals.var_fbsq__blk2123_dn5, locals.var_fbsq__blk2123_dn6, locals.var_fbsq__blk2123_dn7, locals.var_fbsq__blk2123_dn8, locals.var_fbsq__blk2123_dn9, locals.var_fbsq__blk2123_dn10, locals.var_fbsq__blk2123_dn13,)
    }
};
            locals.var_fbsq__blk2123 = assign94720_body21_e146733;
            locals.var_fbsq__blk2123_dn0 = assign94720_body21_e146733_d_n0;
            locals.var_fbsq__blk2123_dn2 = assign94720_body21_e146733_d_n2;
            locals.var_fbsq__blk2123_dn4 = assign94720_body21_e146733_d_n4;
            locals.var_fbsq__blk2123_dn5 = assign94720_body21_e146733_d_n5;
            locals.var_fbsq__blk2123_dn6 = assign94720_body21_e146733_d_n6;
            locals.var_fbsq__blk2123_dn7 = assign94720_body21_e146733_d_n7;
            locals.var_fbsq__blk2123_dn8 = assign94720_body21_e146733_d_n8;
            locals.var_fbsq__blk2123_dn9 = assign94720_body21_e146733_d_n9;
            locals.var_fbsq__blk2123_dn10 = assign94720_body21_e146733_d_n10;
            locals.var_fbsq__blk2123_dn13 = assign94720_body21_e146733_d_n13;
            let (assign94720_body22_e146749, assign94720_body22_e146749_d_n0, assign94720_body22_e146749_d_n2, assign94720_body22_e146749_d_n4, assign94720_body22_e146749_d_n5, assign94720_body22_e146749_d_n6, assign94720_body22_e146749_d_n7, assign94720_body22_e146749_d_n8, assign94720_body22_e146749_d_n9, assign94720_body22_e146749_d_n10, assign94720_body22_e146749_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body22_e146745: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94720_body22_e146746: f64 = (locals.var_t1 - assign94720_body22_e146745);
        let assign94720_body22_e146747: f64 = (locals.var_beta * assign94720_body22_e146746);
        (assign94720_body22_e146747, ((locals.var_beta_dn0 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk2124, locals.var_fbsq_dpss__blk2124_dn0, locals.var_fbsq_dpss__blk2124_dn2, locals.var_fbsq_dpss__blk2124_dn4, locals.var_fbsq_dpss__blk2124_dn5, locals.var_fbsq_dpss__blk2124_dn6, locals.var_fbsq_dpss__blk2124_dn7, locals.var_fbsq_dpss__blk2124_dn8, locals.var_fbsq_dpss__blk2124_dn9, locals.var_fbsq_dpss__blk2124_dn10, locals.var_fbsq_dpss__blk2124_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2124 = assign94720_body22_e146749;
            locals.var_fbsq_dpss__blk2124_dn0 = assign94720_body22_e146749_d_n0;
            locals.var_fbsq_dpss__blk2124_dn2 = assign94720_body22_e146749_d_n2;
            locals.var_fbsq_dpss__blk2124_dn4 = assign94720_body22_e146749_d_n4;
            locals.var_fbsq_dpss__blk2124_dn5 = assign94720_body22_e146749_d_n5;
            locals.var_fbsq_dpss__blk2124_dn6 = assign94720_body22_e146749_d_n6;
            locals.var_fbsq_dpss__blk2124_dn7 = assign94720_body22_e146749_d_n7;
            locals.var_fbsq_dpss__blk2124_dn8 = assign94720_body22_e146749_d_n8;
            locals.var_fbsq_dpss__blk2124_dn9 = assign94720_body22_e146749_d_n9;
            locals.var_fbsq_dpss__blk2124_dn10 = assign94720_body22_e146749_d_n10;
            locals.var_fbsq_dpss__blk2124_dn13 = assign94720_body22_e146749_d_n13;
            let (assign94720_body24_e146781, assign94720_body24_e146781_d_n0, assign94720_body24_e146781_d_n2, assign94720_body24_e146781_d_n4, assign94720_body24_e146781_d_n5, assign94720_body24_e146781_d_n6, assign94720_body24_e146781_d_n7, assign94720_body24_e146781_d_n8, assign94720_body24_e146781_d_n9, assign94720_body24_e146781_d_n10, assign94720_body24_e146781_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body24_e146778: f64 = (-locals.var_chi);
        let assign94720_body24_e146779: f64 = (assign94720_body24_e146778).exp();
        (assign94720_body24_e146779, (assign94720_body24_e146779 * (-locals.var_chi_dn0)), (assign94720_body24_e146779 * (-locals.var_chi_dn2)), (assign94720_body24_e146779 * (-locals.var_chi_dn4)), (assign94720_body24_e146779 * (-locals.var_chi_dn5)), (assign94720_body24_e146779 * (-locals.var_chi_dn6)), (assign94720_body24_e146779 * (-locals.var_chi_dn7)), (assign94720_body24_e146779 * (-locals.var_chi_dn8)), (assign94720_body24_e146779 * (-locals.var_chi_dn9)), (assign94720_body24_e146779 * (-locals.var_chi_dn10)), (assign94720_body24_e146779 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body24_e146781;
            locals.var_t0_dn0 = assign94720_body24_e146781_d_n0;
            locals.var_t0_dn2 = assign94720_body24_e146781_d_n2;
            locals.var_t0_dn4 = assign94720_body24_e146781_d_n4;
            locals.var_t0_dn5 = assign94720_body24_e146781_d_n5;
            locals.var_t0_dn6 = assign94720_body24_e146781_d_n6;
            locals.var_t0_dn7 = assign94720_body24_e146781_d_n7;
            locals.var_t0_dn8 = assign94720_body24_e146781_d_n8;
            locals.var_t0_dn9 = assign94720_body24_e146781_d_n9;
            locals.var_t0_dn10 = assign94720_body24_e146781_d_n10;
            locals.var_t0_dn13 = assign94720_body24_e146781_d_n13;
            let (assign94720_body25_e146794, assign94720_body25_e146794_d_n0, assign94720_body25_e146794_d_n2, assign94720_body25_e146794_d_n4, assign94720_body25_e146794_d_n5, assign94720_body25_e146794_d_n6, assign94720_body25_e146794_d_n7, assign94720_body25_e146794_d_n8, assign94720_body25_e146794_d_n9, assign94720_body25_e146794_d_n10, assign94720_body25_e146794_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body25_e146791: f64 = (-locals.var_chib);
        let assign94720_body25_e146792: f64 = (assign94720_body25_e146791).exp();
        (assign94720_body25_e146792, (assign94720_body25_e146792 * (-locals.var_chib_dn0)), (assign94720_body25_e146792 * (-locals.var_chib_dn2)), (assign94720_body25_e146792 * (-locals.var_chib_dn4)), (assign94720_body25_e146792 * (-locals.var_chib_dn5)), (assign94720_body25_e146792 * (-locals.var_chib_dn6)), (assign94720_body25_e146792 * (-locals.var_chib_dn7)), (assign94720_body25_e146792 * (-locals.var_chib_dn8)), (assign94720_body25_e146792 * (-locals.var_chib_dn9)), (assign94720_body25_e146792 * (-locals.var_chib_dn10)), (assign94720_body25_e146792 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body25_e146794;
            locals.var_t1_dn0 = assign94720_body25_e146794_d_n0;
            locals.var_t1_dn2 = assign94720_body25_e146794_d_n2;
            locals.var_t1_dn4 = assign94720_body25_e146794_d_n4;
            locals.var_t1_dn5 = assign94720_body25_e146794_d_n5;
            locals.var_t1_dn6 = assign94720_body25_e146794_d_n6;
            locals.var_t1_dn7 = assign94720_body25_e146794_d_n7;
            locals.var_t1_dn8 = assign94720_body25_e146794_d_n8;
            locals.var_t1_dn9 = assign94720_body25_e146794_d_n9;
            locals.var_t1_dn10 = assign94720_body25_e146794_d_n10;
            locals.var_t1_dn13 = assign94720_body25_e146794_d_n13;
            let (assign94720_body26_e146811, assign94720_body26_e146811_d_n0, assign94720_body26_e146811_d_n2, assign94720_body26_e146811_d_n4, assign94720_body26_e146811_d_n5, assign94720_body26_e146811_d_n6, assign94720_body26_e146811_d_n7, assign94720_body26_e146811_d_n8, assign94720_body26_e146811_d_n9, assign94720_body26_e146811_d_n10, assign94720_body26_e146811_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body26_e146805: f64 = (locals.var_chi - locals.var_chib);
        let assign94720_body26_e146808: f64 = (locals.var_t0 - locals.var_t1);
        let assign94720_body26_e146809: f64 = (assign94720_body26_e146805 + assign94720_body26_e146808);
        (assign94720_body26_e146809, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk2123, locals.var_fbsq__blk2123_dn0, locals.var_fbsq__blk2123_dn2, locals.var_fbsq__blk2123_dn4, locals.var_fbsq__blk2123_dn5, locals.var_fbsq__blk2123_dn6, locals.var_fbsq__blk2123_dn7, locals.var_fbsq__blk2123_dn8, locals.var_fbsq__blk2123_dn9, locals.var_fbsq__blk2123_dn10, locals.var_fbsq__blk2123_dn13,)
    }
};
            locals.var_fbsq__blk2123 = assign94720_body26_e146811;
            locals.var_fbsq__blk2123_dn0 = assign94720_body26_e146811_d_n0;
            locals.var_fbsq__blk2123_dn2 = assign94720_body26_e146811_d_n2;
            locals.var_fbsq__blk2123_dn4 = assign94720_body26_e146811_d_n4;
            locals.var_fbsq__blk2123_dn5 = assign94720_body26_e146811_d_n5;
            locals.var_fbsq__blk2123_dn6 = assign94720_body26_e146811_d_n6;
            locals.var_fbsq__blk2123_dn7 = assign94720_body26_e146811_d_n7;
            locals.var_fbsq__blk2123_dn8 = assign94720_body26_e146811_d_n8;
            locals.var_fbsq__blk2123_dn9 = assign94720_body26_e146811_d_n9;
            locals.var_fbsq__blk2123_dn10 = assign94720_body26_e146811_d_n10;
            locals.var_fbsq__blk2123_dn13 = assign94720_body26_e146811_d_n13;
            let (assign94720_body27_e146832, assign94720_body27_e146832_d_n0, assign94720_body27_e146832_d_n2, assign94720_body27_e146832_d_n4, assign94720_body27_e146832_d_n5, assign94720_body27_e146832_d_n6, assign94720_body27_e146832_d_n7, assign94720_body27_e146832_d_n8, assign94720_body27_e146832_d_n9, assign94720_body27_e146832_d_n10, assign94720_body27_e146832_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body27_e146823: f64 = (1.0 - locals.var_t0);
        let assign94720_body27_e146827: f64 = (1.0 - locals.var_t1);
        let assign94720_body27_e146828: f64 = (locals.var_phi_b_dpss * assign94720_body27_e146827);
        let assign94720_body27_e146829: f64 = (assign94720_body27_e146823 - assign94720_body27_e146828);
        let assign94720_body27_e146830: f64 = (locals.var_beta * assign94720_body27_e146829);
        (assign94720_body27_e146830, ((locals.var_beta_dn0 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2124, locals.var_fbsq_dpss__blk2124_dn0, locals.var_fbsq_dpss__blk2124_dn2, locals.var_fbsq_dpss__blk2124_dn4, locals.var_fbsq_dpss__blk2124_dn5, locals.var_fbsq_dpss__blk2124_dn6, locals.var_fbsq_dpss__blk2124_dn7, locals.var_fbsq_dpss__blk2124_dn8, locals.var_fbsq_dpss__blk2124_dn9, locals.var_fbsq_dpss__blk2124_dn10, locals.var_fbsq_dpss__blk2124_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2124 = assign94720_body27_e146832;
            locals.var_fbsq_dpss__blk2124_dn0 = assign94720_body27_e146832_d_n0;
            locals.var_fbsq_dpss__blk2124_dn2 = assign94720_body27_e146832_d_n2;
            locals.var_fbsq_dpss__blk2124_dn4 = assign94720_body27_e146832_d_n4;
            locals.var_fbsq_dpss__blk2124_dn5 = assign94720_body27_e146832_d_n5;
            locals.var_fbsq_dpss__blk2124_dn6 = assign94720_body27_e146832_d_n6;
            locals.var_fbsq_dpss__blk2124_dn7 = assign94720_body27_e146832_d_n7;
            locals.var_fbsq_dpss__blk2124_dn8 = assign94720_body27_e146832_d_n8;
            locals.var_fbsq_dpss__blk2124_dn9 = assign94720_body27_e146832_d_n9;
            locals.var_fbsq_dpss__blk2124_dn10 = assign94720_body27_e146832_d_n10;
            locals.var_fbsq_dpss__blk2124_dn13 = assign94720_body27_e146832_d_n13;
            let assign94720_body28_e146834: f64 = (locals.var_chi).abs();
            let assign94720_body28_e146836: f64 = if assign94720_body28_e146834 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2203 = assign94720_body28_e146836;
            let (assign94720_body29_e146868, assign94720_body29_e146868_d_n0, assign94720_body29_e146868_d_n2, assign94720_body29_e146868_d_n4, assign94720_body29_e146868_d_n5, assign94720_body29_e146868_d_n6, assign94720_body29_e146868_d_n7, assign94720_body29_e146868_d_n8, assign94720_body29_e146868_d_n9, assign94720_body29_e146868_d_n10, assign94720_body29_e146868_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body29_e146846: f64 = (locals.var_chi * locals.var_chi);
        let assign94720_body29_e146848: f64 = (assign94720_body29_e146846 / 2.0);
        let assign94720_body29_e146852: f64 = (locals.var_chi / 3.0);
        let assign94720_body29_e146856: f64 = (locals.var_chi / 4.0);
        let assign94720_body29_e146860: f64 = (locals.var_chi / 5.0);
        let assign94720_body29_e146861: f64 = (1.0 + assign94720_body29_e146860);
        let assign94720_body29_e146862: f64 = (assign94720_body29_e146856 * assign94720_body29_e146861);
        let assign94720_body29_e146863: f64 = (1.0 + assign94720_body29_e146862);
        let assign94720_body29_e146864: f64 = (assign94720_body29_e146852 * assign94720_body29_e146863);
        let assign94720_body29_e146865: f64 = (1.0 + assign94720_body29_e146864);
        let assign94720_body29_e146866: f64 = (assign94720_body29_e146848 * assign94720_body29_e146865);
        (assign94720_body29_e146866, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn0 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn0 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn2 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn2 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn4 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn4 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn5 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn5 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn6 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn6 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn7 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn7 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn8 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn8 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn9 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn9 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn10 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn10 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn13 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn13 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body29_e146868;
            locals.var_t0_dn0 = assign94720_body29_e146868_d_n0;
            locals.var_t0_dn2 = assign94720_body29_e146868_d_n2;
            locals.var_t0_dn4 = assign94720_body29_e146868_d_n4;
            locals.var_t0_dn5 = assign94720_body29_e146868_d_n5;
            locals.var_t0_dn6 = assign94720_body29_e146868_d_n6;
            locals.var_t0_dn7 = assign94720_body29_e146868_d_n7;
            locals.var_t0_dn8 = assign94720_body29_e146868_d_n8;
            locals.var_t0_dn9 = assign94720_body29_e146868_d_n9;
            locals.var_t0_dn10 = assign94720_body29_e146868_d_n10;
            locals.var_t0_dn13 = assign94720_body29_e146868_d_n13;
            let (assign94720_body30_e146896, assign94720_body30_e146896_d_n0, assign94720_body30_e146896_d_n2, assign94720_body30_e146896_d_n4, assign94720_body30_e146896_d_n5, assign94720_body30_e146896_d_n6, assign94720_body30_e146896_d_n7, assign94720_body30_e146896_d_n8, assign94720_body30_e146896_d_n9, assign94720_body30_e146896_d_n10, assign94720_body30_e146896_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body30_e146880: f64 = (locals.var_chi / 2.0);
        let assign94720_body30_e146884: f64 = (locals.var_chi / 3.0);
        let assign94720_body30_e146888: f64 = (locals.var_chi / 4.0);
        let assign94720_body30_e146889: f64 = (1.0 + assign94720_body30_e146888);
        let assign94720_body30_e146890: f64 = (assign94720_body30_e146884 * assign94720_body30_e146889);
        let assign94720_body30_e146891: f64 = (1.0 + assign94720_body30_e146890);
        let assign94720_body30_e146892: f64 = (assign94720_body30_e146880 * assign94720_body30_e146891);
        let assign94720_body30_e146893: f64 = (1.0 + assign94720_body30_e146892);
        let assign94720_body30_e146894: f64 = (locals.var_chi * assign94720_body30_e146893);
        (assign94720_body30_e146894, ((locals.var_chi_dn0 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn0 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn2 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn4 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn5 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn6 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn7 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn8 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn9 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn10 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn13 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body30_e146896;
            locals.var_t1_dn0 = assign94720_body30_e146896_d_n0;
            locals.var_t1_dn2 = assign94720_body30_e146896_d_n2;
            locals.var_t1_dn4 = assign94720_body30_e146896_d_n4;
            locals.var_t1_dn5 = assign94720_body30_e146896_d_n5;
            locals.var_t1_dn6 = assign94720_body30_e146896_d_n6;
            locals.var_t1_dn7 = assign94720_body30_e146896_d_n7;
            locals.var_t1_dn8 = assign94720_body30_e146896_d_n8;
            locals.var_t1_dn9 = assign94720_body30_e146896_d_n9;
            locals.var_t1_dn10 = assign94720_body30_e146896_d_n10;
            locals.var_t1_dn13 = assign94720_body30_e146896_d_n13;
            let (assign94720_body31_e146908, assign94720_body31_e146908_d_n0, assign94720_body31_e146908_d_n2, assign94720_body31_e146908_d_n4, assign94720_body31_e146908_d_n5, assign94720_body31_e146908_d_n6, assign94720_body31_e146908_d_n7, assign94720_body31_e146908_d_n8, assign94720_body31_e146908_d_n9, assign94720_body31_e146908_d_n10, assign94720_body31_e146908_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body31_e146906: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94720_body31_e146906, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body31_e146908;
            locals.var_fs01_dn0 = assign94720_body31_e146908_d_n0;
            locals.var_fs01_dn2 = assign94720_body31_e146908_d_n2;
            locals.var_fs01_dn4 = assign94720_body31_e146908_d_n4;
            locals.var_fs01_dn5 = assign94720_body31_e146908_d_n5;
            locals.var_fs01_dn6 = assign94720_body31_e146908_d_n6;
            locals.var_fs01_dn7 = assign94720_body31_e146908_d_n7;
            locals.var_fs01_dn8 = assign94720_body31_e146908_d_n8;
            locals.var_fs01_dn9 = assign94720_body31_e146908_d_n9;
            locals.var_fs01_dn10 = assign94720_body31_e146908_d_n10;
            locals.var_fs01_dn13 = assign94720_body31_e146908_d_n13;
            let (assign94720_body32_e146922, assign94720_body32_e146922_d_n0, assign94720_body32_e146922_d_n2, assign94720_body32_e146922_d_n4, assign94720_body32_e146922_d_n5, assign94720_body32_e146922_d_n6, assign94720_body32_e146922_d_n7, assign94720_body32_e146922_d_n8, assign94720_body32_e146922_d_n9, assign94720_body32_e146922_d_n10, assign94720_body32_e146922_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body32_e146918: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94720_body32_e146920: f64 = (assign94720_body32_e146918 * locals.var_beta);
        (assign94720_body32_e146920, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body32_e146922;
            locals.var_fs01_dps0_dn0 = assign94720_body32_e146922_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body32_e146922_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body32_e146922_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body32_e146922_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body32_e146922_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body32_e146922_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body32_e146922_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body32_e146922_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body32_e146922_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body32_e146922_d_n13;
            let assign94720_body33_e146924: f64 = (locals.var_chi).abs();
            let assign94720_body33_e146926: f64 = if assign94720_body33_e146924 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2204 = assign94720_body33_e146926;
            let (assign94720_body35_e146961, assign94720_body35_e146961_d_n0, assign94720_body35_e146961_d_n2, assign94720_body35_e146961_d_n4, assign94720_body35_e146961_d_n5, assign94720_body35_e146961_d_n6, assign94720_body35_e146961_d_n7, assign94720_body35_e146961_d_n8, assign94720_body35_e146961_d_n9, assign94720_body35_e146961_d_n10, assign94720_body35_e146961_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body35_e146959: f64 = (locals.var_chi).exp();
        (assign94720_body35_e146959, (assign94720_body35_e146959 * locals.var_chi_dn0), (assign94720_body35_e146959 * locals.var_chi_dn2), (assign94720_body35_e146959 * locals.var_chi_dn4), (assign94720_body35_e146959 * locals.var_chi_dn5), (assign94720_body35_e146959 * locals.var_chi_dn6), (assign94720_body35_e146959 * locals.var_chi_dn7), (assign94720_body35_e146959 * locals.var_chi_dn8), (assign94720_body35_e146959 * locals.var_chi_dn9), (assign94720_body35_e146959 * locals.var_chi_dn10), (assign94720_body35_e146959 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign94720_body35_e146961;
            locals.var_exp_chi_dn0 = assign94720_body35_e146961_d_n0;
            locals.var_exp_chi_dn2 = assign94720_body35_e146961_d_n2;
            locals.var_exp_chi_dn4 = assign94720_body35_e146961_d_n4;
            locals.var_exp_chi_dn5 = assign94720_body35_e146961_d_n5;
            locals.var_exp_chi_dn6 = assign94720_body35_e146961_d_n6;
            locals.var_exp_chi_dn7 = assign94720_body35_e146961_d_n7;
            locals.var_exp_chi_dn8 = assign94720_body35_e146961_d_n8;
            locals.var_exp_chi_dn9 = assign94720_body35_e146961_d_n9;
            locals.var_exp_chi_dn10 = assign94720_body35_e146961_d_n10;
            locals.var_exp_chi_dn13 = assign94720_body35_e146961_d_n13;
            let (assign94720_body36_e146976, assign94720_body36_e146976_d_n0, assign94720_body36_e146976_d_n2, assign94720_body36_e146976_d_n4, assign94720_body36_e146976_d_n5, assign94720_body36_e146976_d_n6, assign94720_body36_e146976_d_n7, assign94720_body36_e146976_d_n8, assign94720_body36_e146976_d_n9, assign94720_body36_e146976_d_n10, assign94720_body36_e146976_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body36_e146974: f64 = (locals.var_exp_chi - 1.0);
        (assign94720_body36_e146974, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body36_e146976;
            locals.var_t1_dn0 = assign94720_body36_e146976_d_n0;
            locals.var_t1_dn2 = assign94720_body36_e146976_d_n2;
            locals.var_t1_dn4 = assign94720_body36_e146976_d_n4;
            locals.var_t1_dn5 = assign94720_body36_e146976_d_n5;
            locals.var_t1_dn6 = assign94720_body36_e146976_d_n6;
            locals.var_t1_dn7 = assign94720_body36_e146976_d_n7;
            locals.var_t1_dn8 = assign94720_body36_e146976_d_n8;
            locals.var_t1_dn9 = assign94720_body36_e146976_d_n9;
            locals.var_t1_dn10 = assign94720_body36_e146976_d_n10;
            locals.var_t1_dn13 = assign94720_body36_e146976_d_n13;
            let (assign94720_body37_e146993, assign94720_body37_e146993_d_n0, assign94720_body37_e146993_d_n2, assign94720_body37_e146993_d_n4, assign94720_body37_e146993_d_n5, assign94720_body37_e146993_d_n6, assign94720_body37_e146993_d_n7, assign94720_body37_e146993_d_n8, assign94720_body37_e146993_d_n9, assign94720_body37_e146993_d_n10, assign94720_body37_e146993_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body37_e146990: f64 = (locals.var_t1 - locals.var_chi);
        let assign94720_body37_e146991: f64 = (locals.var_cfs1 * assign94720_body37_e146990);
        (assign94720_body37_e146991, ((locals.var_cfs1_dn0 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body37_e146993;
            locals.var_fs01_dn0 = assign94720_body37_e146993_d_n0;
            locals.var_fs01_dn2 = assign94720_body37_e146993_d_n2;
            locals.var_fs01_dn4 = assign94720_body37_e146993_d_n4;
            locals.var_fs01_dn5 = assign94720_body37_e146993_d_n5;
            locals.var_fs01_dn6 = assign94720_body37_e146993_d_n6;
            locals.var_fs01_dn7 = assign94720_body37_e146993_d_n7;
            locals.var_fs01_dn8 = assign94720_body37_e146993_d_n8;
            locals.var_fs01_dn9 = assign94720_body37_e146993_d_n9;
            locals.var_fs01_dn10 = assign94720_body37_e146993_d_n10;
            locals.var_fs01_dn13 = assign94720_body37_e146993_d_n13;
            let (assign94720_body38_e147010, assign94720_body38_e147010_d_n0, assign94720_body38_e147010_d_n2, assign94720_body38_e147010_d_n4, assign94720_body38_e147010_d_n5, assign94720_body38_e147010_d_n6, assign94720_body38_e147010_d_n7, assign94720_body38_e147010_d_n8, assign94720_body38_e147010_d_n9, assign94720_body38_e147010_d_n10, assign94720_body38_e147010_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body38_e147006: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94720_body38_e147008: f64 = (assign94720_body38_e147006 * locals.var_t1);
        (assign94720_body38_e147008, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body38_e147010;
            locals.var_fs01_dps0_dn0 = assign94720_body38_e147010_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body38_e147010_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body38_e147010_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body38_e147010_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body38_e147010_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body38_e147010_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body38_e147010_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body38_e147010_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body38_e147010_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body38_e147010_d_n13;
            let (assign94720_body40_e147049, assign94720_body40_e147049_d_n0, assign94720_body40_e147049_d_n2, assign94720_body40_e147049_d_n4, assign94720_body40_e147049_d_n5, assign94720_body40_e147049_d_n6, assign94720_body40_e147049_d_n7, assign94720_body40_e147049_d_n8, assign94720_body40_e147049_d_n9, assign94720_body40_e147049_d_n10, assign94720_body40_e147049_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body40_e147046: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94720_body40_e147047: f64 = (assign94720_body40_e147046).exp();
        (assign94720_body40_e147047, (assign94720_body40_e147047 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94720_body40_e147047 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94720_body40_e147047 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94720_body40_e147047 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94720_body40_e147047 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94720_body40_e147047 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94720_body40_e147047 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94720_body40_e147047 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94720_body40_e147047 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94720_body40_e147047 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign94720_body40_e147049;
            locals.var_exp_bps0_dn0 = assign94720_body40_e147049_d_n0;
            locals.var_exp_bps0_dn2 = assign94720_body40_e147049_d_n2;
            locals.var_exp_bps0_dn4 = assign94720_body40_e147049_d_n4;
            locals.var_exp_bps0_dn5 = assign94720_body40_e147049_d_n5;
            locals.var_exp_bps0_dn6 = assign94720_body40_e147049_d_n6;
            locals.var_exp_bps0_dn7 = assign94720_body40_e147049_d_n7;
            locals.var_exp_bps0_dn8 = assign94720_body40_e147049_d_n8;
            locals.var_exp_bps0_dn9 = assign94720_body40_e147049_d_n9;
            locals.var_exp_bps0_dn10 = assign94720_body40_e147049_d_n10;
            locals.var_exp_bps0_dn13 = assign94720_body40_e147049_d_n13;
            let (assign94720_body41_e147071, assign94720_body41_e147071_d_n0, assign94720_body41_e147071_d_n2, assign94720_body41_e147071_d_n4, assign94720_body41_e147071_d_n5, assign94720_body41_e147071_d_n6, assign94720_body41_e147071_d_n7, assign94720_body41_e147071_d_n8, assign94720_body41_e147071_d_n9, assign94720_body41_e147071_d_n10, assign94720_body41_e147071_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body41_e147066: f64 = (locals.var_chi + 1.0);
        let assign94720_body41_e147067: f64 = (locals.var_exp_bvbs * assign94720_body41_e147066);
        let assign94720_body41_e147068: f64 = (locals.var_exp_bps0 - assign94720_body41_e147067);
        let assign94720_body41_e147069: f64 = (locals.var_cnst1over * assign94720_body41_e147068);
        (assign94720_body41_e147069, ((locals.var_cnst1over_dn0 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body41_e147071;
            locals.var_fs01_dn0 = assign94720_body41_e147071_d_n0;
            locals.var_fs01_dn2 = assign94720_body41_e147071_d_n2;
            locals.var_fs01_dn4 = assign94720_body41_e147071_d_n4;
            locals.var_fs01_dn5 = assign94720_body41_e147071_d_n5;
            locals.var_fs01_dn6 = assign94720_body41_e147071_d_n6;
            locals.var_fs01_dn7 = assign94720_body41_e147071_d_n7;
            locals.var_fs01_dn8 = assign94720_body41_e147071_d_n8;
            locals.var_fs01_dn9 = assign94720_body41_e147071_d_n9;
            locals.var_fs01_dn10 = assign94720_body41_e147071_d_n10;
            locals.var_fs01_dn13 = assign94720_body41_e147071_d_n13;
            let (assign94720_body42_e147091, assign94720_body42_e147091_d_n0, assign94720_body42_e147091_d_n2, assign94720_body42_e147091_d_n4, assign94720_body42_e147091_d_n5, assign94720_body42_e147091_d_n6, assign94720_body42_e147091_d_n7, assign94720_body42_e147091_d_n8, assign94720_body42_e147091_d_n9, assign94720_body42_e147091_d_n10, assign94720_body42_e147091_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body42_e147085: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94720_body42_e147088: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94720_body42_e147089: f64 = (assign94720_body42_e147085 * assign94720_body42_e147088);
        (assign94720_body42_e147089, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body42_e147091;
            locals.var_fs01_dps0_dn0 = assign94720_body42_e147091_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body42_e147091_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body42_e147091_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body42_e147091_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body42_e147091_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body42_e147091_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body42_e147091_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body42_e147091_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body42_e147091_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body42_e147091_d_n13;
            let assign94720_body43_e147094: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2205 = assign94720_body43_e147094;
            let (assign94720_body44_e147107, assign94720_body44_e147107_d_n0, assign94720_body44_e147107_d_n2, assign94720_body44_e147107_d_n4, assign94720_body44_e147107_d_n5, assign94720_body44_e147107_d_n6, assign94720_body44_e147107_d_n7, assign94720_body44_e147107_d_n8, assign94720_body44_e147107_d_n9, assign94720_body44_e147107_d_n10, assign94720_body44_e147107_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94720_body44_e147104: f64 = (locals.var_fbsq__blk2123 + locals.var_fs01);
        let assign94720_body44_e147105: f64 = (assign94720_body44_e147104).sqrt();
        (assign94720_body44_e147105, ((locals.var_fbsq__blk2123_dn0 + locals.var_fs01_dn0) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn2 + locals.var_fs01_dn2) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn4 + locals.var_fs01_dn4) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn5 + locals.var_fs01_dn5) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn6 + locals.var_fs01_dn6) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn7 + locals.var_fs01_dn7) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn8 + locals.var_fs01_dn8) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn9 + locals.var_fs01_dn9) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn10 + locals.var_fs01_dn10) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn13 + locals.var_fs01_dn13) / (2.0 * assign94720_body44_e147105)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body44_e147107;
            locals.var_fs02_dn0 = assign94720_body44_e147107_d_n0;
            locals.var_fs02_dn2 = assign94720_body44_e147107_d_n2;
            locals.var_fs02_dn4 = assign94720_body44_e147107_d_n4;
            locals.var_fs02_dn5 = assign94720_body44_e147107_d_n5;
            locals.var_fs02_dn6 = assign94720_body44_e147107_d_n6;
            locals.var_fs02_dn7 = assign94720_body44_e147107_d_n7;
            locals.var_fs02_dn8 = assign94720_body44_e147107_d_n8;
            locals.var_fs02_dn9 = assign94720_body44_e147107_d_n9;
            locals.var_fs02_dn10 = assign94720_body44_e147107_d_n10;
            locals.var_fs02_dn13 = assign94720_body44_e147107_d_n13;
            let (assign94720_body45_e147123, assign94720_body45_e147123_d_n0, assign94720_body45_e147123_d_n2, assign94720_body45_e147123_d_n4, assign94720_body45_e147123_d_n5, assign94720_body45_e147123_d_n6, assign94720_body45_e147123_d_n7, assign94720_body45_e147123_d_n8, assign94720_body45_e147123_d_n9, assign94720_body45_e147123_d_n10, assign94720_body45_e147123_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94720_body45_e147118: f64 = (locals.var_fbsq_dpss__blk2124 + locals.var_fs01_dps0);
        let assign94720_body45_e147119: f64 = (0.5 * assign94720_body45_e147118);
        let assign94720_body45_e147121: f64 = (assign94720_body45_e147119 / locals.var_fs02);
        (assign94720_body45_e147121, ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body45_e147123;
            locals.var_fs02_dps0_dn0 = assign94720_body45_e147123_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body45_e147123_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body45_e147123_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body45_e147123_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body45_e147123_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body45_e147123_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body45_e147123_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body45_e147123_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body45_e147123_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body45_e147123_d_n13;
            let assign94720_body46_e147126: f64 = if locals.var_fbsq__blk2123 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2206 = assign94720_body46_e147126;
            let (assign94720_body47_e147140, assign94720_body47_e147140_d_n0, assign94720_body47_e147140_d_n2, assign94720_body47_e147140_d_n4, assign94720_body47_e147140_d_n5, assign94720_body47_e147140_d_n6, assign94720_body47_e147140_d_n7, assign94720_body47_e147140_d_n8, assign94720_body47_e147140_d_n9, assign94720_body47_e147140_d_n10, assign94720_body47_e147140_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94720_body47_e147138: f64 = (locals.var_fbsq__blk2123).sqrt();
        (assign94720_body47_e147138, (locals.var_fbsq__blk2123_dn0 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn2 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn4 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn5 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn6 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn7 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn8 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn9 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn10 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn13 / (2.0 * assign94720_body47_e147138)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body47_e147140;
            locals.var_fs02_dn0 = assign94720_body47_e147140_d_n0;
            locals.var_fs02_dn2 = assign94720_body47_e147140_d_n2;
            locals.var_fs02_dn4 = assign94720_body47_e147140_d_n4;
            locals.var_fs02_dn5 = assign94720_body47_e147140_d_n5;
            locals.var_fs02_dn6 = assign94720_body47_e147140_d_n6;
            locals.var_fs02_dn7 = assign94720_body47_e147140_d_n7;
            locals.var_fs02_dn8 = assign94720_body47_e147140_d_n8;
            locals.var_fs02_dn9 = assign94720_body47_e147140_d_n9;
            locals.var_fs02_dn10 = assign94720_body47_e147140_d_n10;
            locals.var_fs02_dn13 = assign94720_body47_e147140_d_n13;
            let (assign94720_body48_e147157, assign94720_body48_e147157_d_n0, assign94720_body48_e147157_d_n2, assign94720_body48_e147157_d_n4, assign94720_body48_e147157_d_n5, assign94720_body48_e147157_d_n6, assign94720_body48_e147157_d_n7, assign94720_body48_e147157_d_n8, assign94720_body48_e147157_d_n9, assign94720_body48_e147157_d_n10, assign94720_body48_e147157_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94720_body48_e147153: f64 = (0.5 * locals.var_fbsq_dpss__blk2124);
        let assign94720_body48_e147155: f64 = (assign94720_body48_e147153 / locals.var_fs02);
        (assign94720_body48_e147155, ((((0.5 * locals.var_fbsq_dpss__blk2124_dn0) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn2) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn4) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn5) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn6) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn7) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn8) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn9) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn10) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn13) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body48_e147157;
            locals.var_fs02_dps0_dn0 = assign94720_body48_e147157_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body48_e147157_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body48_e147157_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body48_e147157_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body48_e147157_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body48_e147157_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body48_e147157_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body48_e147157_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body48_e147157_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body48_e147157_d_n13;
            let (assign94720_body49_e147171, assign94720_body49_e147171_d_n0, assign94720_body49_e147171_d_n2, assign94720_body49_e147171_d_n4, assign94720_body49_e147171_d_n5, assign94720_body49_e147171_d_n6, assign94720_body49_e147171_d_n7, assign94720_body49_e147171_d_n8, assign94720_body49_e147171_d_n9, assign94720_body49_e147171_d_n10, assign94720_body49_e147171_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body49_e147171;
            locals.var_fs02_dn0 = assign94720_body49_e147171_d_n0;
            locals.var_fs02_dn2 = assign94720_body49_e147171_d_n2;
            locals.var_fs02_dn4 = assign94720_body49_e147171_d_n4;
            locals.var_fs02_dn5 = assign94720_body49_e147171_d_n5;
            locals.var_fs02_dn6 = assign94720_body49_e147171_d_n6;
            locals.var_fs02_dn7 = assign94720_body49_e147171_d_n7;
            locals.var_fs02_dn8 = assign94720_body49_e147171_d_n8;
            locals.var_fs02_dn9 = assign94720_body49_e147171_d_n9;
            locals.var_fs02_dn10 = assign94720_body49_e147171_d_n10;
            locals.var_fs02_dn13 = assign94720_body49_e147171_d_n13;
            let (assign94720_body50_e147185, assign94720_body50_e147185_d_n0, assign94720_body50_e147185_d_n2, assign94720_body50_e147185_d_n4, assign94720_body50_e147185_d_n5, assign94720_body50_e147185_d_n6, assign94720_body50_e147185_d_n7, assign94720_body50_e147185_d_n8, assign94720_body50_e147185_d_n9, assign94720_body50_e147185_d_n10, assign94720_body50_e147185_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body50_e147185;
            locals.var_fs02_dps0_dn0 = assign94720_body50_e147185_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body50_e147185_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body50_e147185_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body50_e147185_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body50_e147185_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body50_e147185_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body50_e147185_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body50_e147185_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body50_e147185_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body50_e147185_d_n13;
            let (assign94720_body51_e147201, assign94720_body51_e147201_d_n0, assign94720_body51_e147201_d_n2, assign94720_body51_e147201_d_n4, assign94720_body51_e147201_d_n5, assign94720_body51_e147201_d_n6, assign94720_body51_e147201_d_n7, assign94720_body51_e147201_d_n8, assign94720_body51_e147201_d_n9, assign94720_body51_e147201_d_n10, assign94720_body51_e147201_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94720_body51_e147197,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body51_e147196: f64 = (-1.0);
                (assign94720_body51_e147196,)
            }
        };
        let assign94720_body51_e147199: f64 = (assign94720_body51_e147197 * locals.var_fs02);
        (assign94720_body51_e147199, (assign94720_body51_e147197 * locals.var_fs02_dn0), (assign94720_body51_e147197 * locals.var_fs02_dn2), (assign94720_body51_e147197 * locals.var_fs02_dn4), (assign94720_body51_e147197 * locals.var_fs02_dn5), (assign94720_body51_e147197 * locals.var_fs02_dn6), (assign94720_body51_e147197 * locals.var_fs02_dn7), (assign94720_body51_e147197 * locals.var_fs02_dn8), (assign94720_body51_e147197 * locals.var_fs02_dn9), (assign94720_body51_e147197 * locals.var_fs02_dn10), (assign94720_body51_e147197 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body51_e147201;
            locals.var_fs02_dn0 = assign94720_body51_e147201_d_n0;
            locals.var_fs02_dn2 = assign94720_body51_e147201_d_n2;
            locals.var_fs02_dn4 = assign94720_body51_e147201_d_n4;
            locals.var_fs02_dn5 = assign94720_body51_e147201_d_n5;
            locals.var_fs02_dn6 = assign94720_body51_e147201_d_n6;
            locals.var_fs02_dn7 = assign94720_body51_e147201_d_n7;
            locals.var_fs02_dn8 = assign94720_body51_e147201_d_n8;
            locals.var_fs02_dn9 = assign94720_body51_e147201_d_n9;
            locals.var_fs02_dn10 = assign94720_body51_e147201_d_n10;
            locals.var_fs02_dn13 = assign94720_body51_e147201_d_n13;
            let (assign94720_body52_e147217, assign94720_body52_e147217_d_n0, assign94720_body52_e147217_d_n2, assign94720_body52_e147217_d_n4, assign94720_body52_e147217_d_n5, assign94720_body52_e147217_d_n6, assign94720_body52_e147217_d_n7, assign94720_body52_e147217_d_n8, assign94720_body52_e147217_d_n9, assign94720_body52_e147217_d_n10, assign94720_body52_e147217_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94720_body52_e147213,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body52_e147212: f64 = (-1.0);
                (assign94720_body52_e147212,)
            }
        };
        let assign94720_body52_e147215: f64 = (assign94720_body52_e147213 * locals.var_fs02_dps0);
        (assign94720_body52_e147215, (assign94720_body52_e147213 * locals.var_fs02_dps0_dn0), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn2), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn4), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn5), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn6), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn7), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn8), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn9), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn10), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body52_e147217;
            locals.var_fs02_dps0_dn0 = assign94720_body52_e147217_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body52_e147217_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body52_e147217_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body52_e147217_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body52_e147217_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body52_e147217_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body52_e147217_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body52_e147217_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body52_e147217_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body52_e147217_d_n13;
            let (assign94720_body53_e147232, assign94720_body53_e147232_d_n0, assign94720_body53_e147232_d_n2, assign94720_body53_e147232_d_n4, assign94720_body53_e147232_d_n5, assign94720_body53_e147232_d_n6, assign94720_body53_e147232_d_n7, assign94720_body53_e147232_d_n8, assign94720_body53_e147232_d_n9, assign94720_body53_e147232_d_n10, assign94720_body53_e147232_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body53_e147224: f64 = (-locals.var_vgpld);
        let assign94720_body53_e147226: f64 = (assign94720_body53_e147224 + locals.var_ps0ld);
        let assign94720_body53_e147229: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94720_body53_e147230: f64 = (assign94720_body53_e147226 + assign94720_body53_e147229);
        (assign94720_body53_e147230, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign94720_body53_e147232;
            locals.var_fs0_dn0 = assign94720_body53_e147232_d_n0;
            locals.var_fs0_dn2 = assign94720_body53_e147232_d_n2;
            locals.var_fs0_dn4 = assign94720_body53_e147232_d_n4;
            locals.var_fs0_dn5 = assign94720_body53_e147232_d_n5;
            locals.var_fs0_dn6 = assign94720_body53_e147232_d_n6;
            locals.var_fs0_dn7 = assign94720_body53_e147232_d_n7;
            locals.var_fs0_dn8 = assign94720_body53_e147232_d_n8;
            locals.var_fs0_dn9 = assign94720_body53_e147232_d_n9;
            locals.var_fs0_dn10 = assign94720_body53_e147232_d_n10;
            locals.var_fs0_dn13 = assign94720_body53_e147232_d_n13;
            let (assign94720_body54_e147244, assign94720_body54_e147244_d_n0, assign94720_body54_e147244_d_n2, assign94720_body54_e147244_d_n4, assign94720_body54_e147244_d_n5, assign94720_body54_e147244_d_n6, assign94720_body54_e147244_d_n7, assign94720_body54_e147244_d_n8, assign94720_body54_e147244_d_n9, assign94720_body54_e147244_d_n10, assign94720_body54_e147244_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body54_e147241: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94720_body54_e147242: f64 = (1.0 + assign94720_body54_e147241);
        (assign94720_body54_e147242, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign94720_body54_e147244;
            locals.var_fs0_dps0_dn0 = assign94720_body54_e147244_d_n0;
            locals.var_fs0_dps0_dn2 = assign94720_body54_e147244_d_n2;
            locals.var_fs0_dps0_dn4 = assign94720_body54_e147244_d_n4;
            locals.var_fs0_dps0_dn5 = assign94720_body54_e147244_d_n5;
            locals.var_fs0_dps0_dn6 = assign94720_body54_e147244_d_n6;
            locals.var_fs0_dps0_dn7 = assign94720_body54_e147244_d_n7;
            locals.var_fs0_dps0_dn8 = assign94720_body54_e147244_d_n8;
            locals.var_fs0_dps0_dn9 = assign94720_body54_e147244_d_n9;
            locals.var_fs0_dps0_dn10 = assign94720_body54_e147244_d_n10;
            locals.var_fs0_dps0_dn13 = assign94720_body54_e147244_d_n13;
            let assign94720_body55_e147247: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2207 = assign94720_body55_e147247;
            let (assign94720_body56_e147259,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94720_body56_e147257: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94720_body56_e147257,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94720_body56_e147259;
            let (assign94720_body57_e147273, assign94720_body57_e147273_d_n0, assign94720_body57_e147273_d_n2, assign94720_body57_e147273_d_n4, assign94720_body57_e147273_d_n5, assign94720_body57_e147273_d_n6, assign94720_body57_e147273_d_n7, assign94720_body57_e147273_d_n8, assign94720_body57_e147273_d_n9, assign94720_body57_e147273_d_n10, assign94720_body57_e147273_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body57_e147269: f64 = (-locals.var_fs0);
        let assign94720_body57_e147271: f64 = (assign94720_body57_e147269 / locals.var_fs0_dps0);
        (assign94720_body57_e147271, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94720_body57_e147273;
            locals.var_dps0_dn0 = assign94720_body57_e147273_d_n0;
            locals.var_dps0_dn2 = assign94720_body57_e147273_d_n2;
            locals.var_dps0_dn4 = assign94720_body57_e147273_d_n4;
            locals.var_dps0_dn5 = assign94720_body57_e147273_d_n5;
            locals.var_dps0_dn6 = assign94720_body57_e147273_d_n6;
            locals.var_dps0_dn7 = assign94720_body57_e147273_d_n7;
            locals.var_dps0_dn8 = assign94720_body57_e147273_d_n8;
            locals.var_dps0_dn9 = assign94720_body57_e147273_d_n9;
            locals.var_dps0_dn10 = assign94720_body57_e147273_d_n10;
            locals.var_dps0_dn13 = assign94720_body57_e147273_d_n13;
            let (assign94720_body58_e147297, assign94720_body58_e147297_d_n0, assign94720_body58_e147297_d_n2, assign94720_body58_e147297_d_n4, assign94720_body58_e147297_d_n5, assign94720_body58_e147297_d_n6, assign94720_body58_e147297_d_n7, assign94720_body58_e147297_d_n8, assign94720_body58_e147297_d_n9, assign94720_body58_e147297_d_n10, assign94720_body58_e147297_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body58_e147284: f64 = (0.5 * 0.1);
        let assign94720_body58_e147288: f64 = (locals.var_ps0ld).abs();
        let (assign94720_body58_e147293, assign94720_body58_e147293_d_n0, assign94720_body58_e147293_d_n2, assign94720_body58_e147293_d_n4, assign94720_body58_e147293_d_n5, assign94720_body58_e147293_d_n6, assign94720_body58_e147293_d_n7, assign94720_body58_e147293_d_n8, assign94720_body58_e147293_d_n9, assign94720_body58_e147293_d_n10, assign94720_body58_e147293_d_n13,) = {
            if (1.0 >= assign94720_body58_e147288) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94720_body58_e147292: f64 = (locals.var_ps0ld).abs();
                (assign94720_body58_e147292, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign94720_body58_e147294: f64 = (1.0 + assign94720_body58_e147293);
        let assign94720_body58_e147295: f64 = (assign94720_body58_e147284 * assign94720_body58_e147294);
        (assign94720_body58_e147295, (assign94720_body58_e147284 * assign94720_body58_e147293_d_n0), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n2), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n4), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n5), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n6), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n7), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n8), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n9), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n10), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign94720_body58_e147297;
            locals.var_dplim_dn0 = assign94720_body58_e147297_d_n0;
            locals.var_dplim_dn2 = assign94720_body58_e147297_d_n2;
            locals.var_dplim_dn4 = assign94720_body58_e147297_d_n4;
            locals.var_dplim_dn5 = assign94720_body58_e147297_d_n5;
            locals.var_dplim_dn6 = assign94720_body58_e147297_d_n6;
            locals.var_dplim_dn7 = assign94720_body58_e147297_d_n7;
            locals.var_dplim_dn8 = assign94720_body58_e147297_d_n8;
            locals.var_dplim_dn9 = assign94720_body58_e147297_d_n9;
            locals.var_dplim_dn10 = assign94720_body58_e147297_d_n10;
            locals.var_dplim_dn13 = assign94720_body58_e147297_d_n13;
            let assign94720_body59_e147299: f64 = (locals.var_dps0).abs();
            let assign94720_body59_e147301: f64 = if assign94720_body59_e147299 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2208 = assign94720_body59_e147301;
            let (assign94720_body60_e147322, assign94720_body60_e147322_d_n0, assign94720_body60_e147322_d_n2, assign94720_body60_e147322_d_n4, assign94720_body60_e147322_d_n5, assign94720_body60_e147322_d_n6, assign94720_body60_e147322_d_n7, assign94720_body60_e147322_d_n8, assign94720_body60_e147322_d_n9, assign94720_body60_e147322_d_n10, assign94720_body60_e147322_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let (assign94720_body60_e147319,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body60_e147318: f64 = (-1.0);
                (assign94720_body60_e147318,)
            }
        };
        let assign94720_body60_e147320: f64 = (locals.var_dplim * assign94720_body60_e147319);
        (assign94720_body60_e147320, (locals.var_dplim_dn0 * assign94720_body60_e147319), (locals.var_dplim_dn2 * assign94720_body60_e147319), (locals.var_dplim_dn4 * assign94720_body60_e147319), (locals.var_dplim_dn5 * assign94720_body60_e147319), (locals.var_dplim_dn6 * assign94720_body60_e147319), (locals.var_dplim_dn7 * assign94720_body60_e147319), (locals.var_dplim_dn8 * assign94720_body60_e147319), (locals.var_dplim_dn9 * assign94720_body60_e147319), (locals.var_dplim_dn10 * assign94720_body60_e147319), (locals.var_dplim_dn13 * assign94720_body60_e147319),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94720_body60_e147322;
            locals.var_dps0_dn0 = assign94720_body60_e147322_d_n0;
            locals.var_dps0_dn2 = assign94720_body60_e147322_d_n2;
            locals.var_dps0_dn4 = assign94720_body60_e147322_d_n4;
            locals.var_dps0_dn5 = assign94720_body60_e147322_d_n5;
            locals.var_dps0_dn6 = assign94720_body60_e147322_d_n6;
            locals.var_dps0_dn7 = assign94720_body60_e147322_d_n7;
            locals.var_dps0_dn8 = assign94720_body60_e147322_d_n8;
            locals.var_dps0_dn9 = assign94720_body60_e147322_d_n9;
            locals.var_dps0_dn10 = assign94720_body60_e147322_d_n10;
            locals.var_dps0_dn13 = assign94720_body60_e147322_d_n13;
            let (assign94720_body61_e147335, assign94720_body61_e147335_d_n0, assign94720_body61_e147335_d_n2, assign94720_body61_e147335_d_n4, assign94720_body61_e147335_d_n5, assign94720_body61_e147335_d_n6, assign94720_body61_e147335_d_n7, assign94720_body61_e147335_d_n8, assign94720_body61_e147335_d_n9, assign94720_body61_e147335_d_n10, assign94720_body61_e147335_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body61_e147333: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94720_body61_e147333, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign94720_body61_e147335;
            locals.var_ps0ld_dn0 = assign94720_body61_e147335_d_n0;
            locals.var_ps0ld_dn2 = assign94720_body61_e147335_d_n2;
            locals.var_ps0ld_dn4 = assign94720_body61_e147335_d_n4;
            locals.var_ps0ld_dn5 = assign94720_body61_e147335_d_n5;
            locals.var_ps0ld_dn6 = assign94720_body61_e147335_d_n6;
            locals.var_ps0ld_dn7 = assign94720_body61_e147335_d_n7;
            locals.var_ps0ld_dn8 = assign94720_body61_e147335_d_n8;
            locals.var_ps0ld_dn9 = assign94720_body61_e147335_d_n9;
            locals.var_ps0ld_dn10 = assign94720_body61_e147335_d_n10;
            locals.var_ps0ld_dn13 = assign94720_body61_e147335_d_n13;
            let assign94720_body62_e147337: f64 = (locals.var_dps0).abs();
            let assign94720_body62_e147341: f64 = (locals.var_fs0).abs();
            let assign94720_body62_e147344: f64 = if ((assign94720_body62_e147337 <= 1e-12) && (assign94720_body62_e147341 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2209 = assign94720_body62_e147344;
            let (assign94720_body63_e147359,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2209 != 0.0)) {
        let assign94720_body63_e147357: f64 = (locals.var_flg_conv + 2.0);
        (assign94720_body63_e147357,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94720_body63_e147359;
            let (assign94720_body64_e147369,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body64_e147367: f64 = (locals.var_lp_s0 + 1.0);
        (assign94720_body64_e147367,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94720_body64_e147369;
        }

    }

    pub(super) fn stamp_transient_block_338(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94740_e147394, assign94740_e147394_d_n0, assign94740_e147394_d_n2, assign94740_e147394_d_n4, assign94740_e147394_d_n5, assign94740_e147394_d_n6, assign94740_e147394_d_n7, assign94740_e147394_d_n8, assign94740_e147394_d_n9, assign94740_e147394_d_n10, assign94740_e147394_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94740_e147392, assign94740_e147392_d_n0, assign94740_e147392_d_n2, assign94740_e147392_d_n4, assign94740_e147392_d_n5, assign94740_e147392_d_n6, assign94740_e147392_d_n7, assign94740_e147392_d_n8, assign94740_e147392_d_n9, assign94740_e147392_d_n10, assign94740_e147392_d_n13,) = {
            if (locals.var_fbsq__blk2123 >= 0.0) {
                let (assign94740_e147387,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign94740_e147386: f64 = (-1.0);
                        (assign94740_e147386,)
                    }
                };
                let assign94740_e147389: f64 = (locals.var_fbsq__blk2123).sqrt();
                let assign94740_e147390: f64 = (assign94740_e147387 * assign94740_e147389);
                (assign94740_e147390, (assign94740_e147387 * (locals.var_fbsq__blk2123_dn0 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn2 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn4 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn5 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn6 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn7 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn8 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn9 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn10 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn13 / (2.0 * assign94740_e147389))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign94740_e147392, assign94740_e147392_d_n0, assign94740_e147392_d_n2, assign94740_e147392_d_n4, assign94740_e147392_d_n5, assign94740_e147392_d_n6, assign94740_e147392_d_n7, assign94740_e147392_d_n8, assign94740_e147392_d_n9, assign94740_e147392_d_n10, assign94740_e147392_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign94740_e147394;
        locals.var_fb_dn0 = assign94740_e147394_d_n0;
        locals.var_fb_dn2 = assign94740_e147394_d_n2;
        locals.var_fb_dn4 = assign94740_e147394_d_n4;
        locals.var_fb_dn5 = assign94740_e147394_d_n5;
        locals.var_fb_dn6 = assign94740_e147394_d_n6;
        locals.var_fb_dn7 = assign94740_e147394_d_n7;
        locals.var_fb_dn8 = assign94740_e147394_d_n8;
        locals.var_fb_dn9 = assign94740_e147394_d_n9;
        locals.var_fb_dn10 = assign94740_e147394_d_n10;
        locals.var_fb_dn13 = assign94740_e147394_d_n13;

        let (assign94750_e147404, assign94750_e147404_d_n0, assign94750_e147404_d_n2, assign94750_e147404_d_n4, assign94750_e147404_d_n5, assign94750_e147404_d_n6, assign94750_e147404_d_n7, assign94750_e147404_d_n8, assign94750_e147404_d_n9, assign94750_e147404_d_n10, assign94750_e147404_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94750_e147402: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94750_e147402, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2113, locals.var_wdld__blk2113_dn0, locals.var_wdld__blk2113_dn2, locals.var_wdld__blk2113_dn4, locals.var_wdld__blk2113_dn5, locals.var_wdld__blk2113_dn6, locals.var_wdld__blk2113_dn7, locals.var_wdld__blk2113_dn8, locals.var_wdld__blk2113_dn9, locals.var_wdld__blk2113_dn10, locals.var_wdld__blk2113_dn13,)
    }
};
        locals.var_wdld__blk2113 = assign94750_e147404;
        locals.var_wdld__blk2113_dn0 = assign94750_e147404_d_n0;
        locals.var_wdld__blk2113_dn2 = assign94750_e147404_d_n2;
        locals.var_wdld__blk2113_dn4 = assign94750_e147404_d_n4;
        locals.var_wdld__blk2113_dn5 = assign94750_e147404_d_n5;
        locals.var_wdld__blk2113_dn6 = assign94750_e147404_d_n6;
        locals.var_wdld__blk2113_dn7 = assign94750_e147404_d_n7;
        locals.var_wdld__blk2113_dn8 = assign94750_e147404_d_n8;
        locals.var_wdld__blk2113_dn9 = assign94750_e147404_d_n9;
        locals.var_wdld__blk2113_dn10 = assign94750_e147404_d_n10;
        locals.var_wdld__blk2113_dn13 = assign94750_e147404_d_n13;

        let (assign94760_e147414, assign94760_e147414_d_n0, assign94760_e147414_d_n2, assign94760_e147414_d_n4, assign94760_e147414_d_n5, assign94760_e147414_d_n6, assign94760_e147414_d_n7, assign94760_e147414_d_n8, assign94760_e147414_d_n9, assign94760_e147414_d_n10, assign94760_e147414_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94760_e147412: f64 = (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113);
        (assign94760_e147412, (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn0), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn2), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn4), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn5), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn6), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn7), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn8), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn9), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn10), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2114, locals.var_q_dep_ld__blk2114_dn0, locals.var_q_dep_ld__blk2114_dn2, locals.var_q_dep_ld__blk2114_dn4, locals.var_q_dep_ld__blk2114_dn5, locals.var_q_dep_ld__blk2114_dn6, locals.var_q_dep_ld__blk2114_dn7, locals.var_q_dep_ld__blk2114_dn8, locals.var_q_dep_ld__blk2114_dn9, locals.var_q_dep_ld__blk2114_dn10, locals.var_q_dep_ld__blk2114_dn13,)
    }
};
        locals.var_q_dep_ld__blk2114 = assign94760_e147414;
        locals.var_q_dep_ld__blk2114_dn0 = assign94760_e147414_d_n0;
        locals.var_q_dep_ld__blk2114_dn2 = assign94760_e147414_d_n2;
        locals.var_q_dep_ld__blk2114_dn4 = assign94760_e147414_d_n4;
        locals.var_q_dep_ld__blk2114_dn5 = assign94760_e147414_d_n5;
        locals.var_q_dep_ld__blk2114_dn6 = assign94760_e147414_d_n6;
        locals.var_q_dep_ld__blk2114_dn7 = assign94760_e147414_d_n7;
        locals.var_q_dep_ld__blk2114_dn8 = assign94760_e147414_d_n8;
        locals.var_q_dep_ld__blk2114_dn9 = assign94760_e147414_d_n9;
        locals.var_q_dep_ld__blk2114_dn10 = assign94760_e147414_d_n10;
        locals.var_q_dep_ld__blk2114_dn13 = assign94760_e147414_d_n13;

        let (assign94770_e147428, assign94770_e147428_d_n0, assign94770_e147428_d_n2, assign94770_e147428_d_n4, assign94770_e147428_d_n5, assign94770_e147428_d_n6, assign94770_e147428_d_n7, assign94770_e147428_d_n8, assign94770_e147428_d_n9, assign94770_e147428_d_n10, assign94770_e147428_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94770_e147422: f64 = (locals.var_q_dep_ld__blk2114 / locals.var_cnst0over_func);
        let assign94770_e147425: f64 = (10.0 * 2.220446049250313e-16);
        let assign94770_e147426: f64 = (assign94770_e147422 + assign94770_e147425);
        (assign94770_e147426, (((locals.var_q_dep_ld__blk2114_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign94770_e147428;
        locals.var_xi0p12_dn0 = assign94770_e147428_d_n0;
        locals.var_xi0p12_dn2 = assign94770_e147428_d_n2;
        locals.var_xi0p12_dn4 = assign94770_e147428_d_n4;
        locals.var_xi0p12_dn5 = assign94770_e147428_d_n5;
        locals.var_xi0p12_dn6 = assign94770_e147428_d_n6;
        locals.var_xi0p12_dn7 = assign94770_e147428_d_n7;
        locals.var_xi0p12_dn8 = assign94770_e147428_d_n8;
        locals.var_xi0p12_dn9 = assign94770_e147428_d_n9;
        locals.var_xi0p12_dn10 = assign94770_e147428_d_n10;
        locals.var_xi0p12_dn13 = assign94770_e147428_d_n13;

        let (assign94780_e147438, assign94780_e147438_d_n0, assign94780_e147438_d_n2, assign94780_e147438_d_n4, assign94780_e147438_d_n5, assign94780_e147438_d_n6, assign94780_e147438_d_n7, assign94780_e147438_d_n8, assign94780_e147438_d_n9, assign94780_e147438_d_n10, assign94780_e147438_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94780_e147436: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94780_e147436, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign94780_e147438;
        locals.var_qbuld_dn0 = assign94780_e147438_d_n0;
        locals.var_qbuld_dn2 = assign94780_e147438_d_n2;
        locals.var_qbuld_dn4 = assign94780_e147438_d_n4;
        locals.var_qbuld_dn5 = assign94780_e147438_d_n5;
        locals.var_qbuld_dn6 = assign94780_e147438_d_n6;
        locals.var_qbuld_dn7 = assign94780_e147438_d_n7;
        locals.var_qbuld_dn8 = assign94780_e147438_d_n8;
        locals.var_qbuld_dn9 = assign94780_e147438_d_n9;
        locals.var_qbuld_dn10 = assign94780_e147438_d_n10;
        locals.var_qbuld_dn13 = assign94780_e147438_d_n13;

        let (assign94790_e147450, assign94790_e147450_d_n0, assign94790_e147450_d_n2, assign94790_e147450_d_n4, assign94790_e147450_d_n5, assign94790_e147450_d_n6, assign94790_e147450_d_n7, assign94790_e147450_d_n8, assign94790_e147450_d_n9, assign94790_e147450_d_n10, assign94790_e147450_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94790_e147447: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94790_e147448: f64 = (1.0 / assign94790_e147447);
        (assign94790_e147448, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign94790_e147447 * assign94790_e147447))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94790_e147450;
        locals.var_t1_dn0 = assign94790_e147450_d_n0;
        locals.var_t1_dn2 = assign94790_e147450_d_n2;
        locals.var_t1_dn4 = assign94790_e147450_d_n4;
        locals.var_t1_dn5 = assign94790_e147450_d_n5;
        locals.var_t1_dn6 = assign94790_e147450_d_n6;
        locals.var_t1_dn7 = assign94790_e147450_d_n7;
        locals.var_t1_dn8 = assign94790_e147450_d_n8;
        locals.var_t1_dn9 = assign94790_e147450_d_n9;
        locals.var_t1_dn10 = assign94790_e147450_d_n10;
        locals.var_t1_dn13 = assign94790_e147450_d_n13;

        let (assign94800_e147462, assign94800_e147462_d_n0, assign94800_e147462_d_n2, assign94800_e147462_d_n4, assign94800_e147462_d_n5, assign94800_e147462_d_n6, assign94800_e147462_d_n7, assign94800_e147462_d_n8, assign94800_e147462_d_n9, assign94800_e147462_d_n10, assign94800_e147462_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94800_e147458: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94800_e147460: f64 = (assign94800_e147458 * locals.var_t1);
        (assign94800_e147460, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign94800_e147462;
        locals.var_qiuld_dn0 = assign94800_e147462_d_n0;
        locals.var_qiuld_dn2 = assign94800_e147462_d_n2;
        locals.var_qiuld_dn4 = assign94800_e147462_d_n4;
        locals.var_qiuld_dn5 = assign94800_e147462_d_n5;
        locals.var_qiuld_dn6 = assign94800_e147462_d_n6;
        locals.var_qiuld_dn7 = assign94800_e147462_d_n7;
        locals.var_qiuld_dn8 = assign94800_e147462_d_n8;
        locals.var_qiuld_dn9 = assign94800_e147462_d_n9;
        locals.var_qiuld_dn10 = assign94800_e147462_d_n10;
        locals.var_qiuld_dn13 = assign94800_e147462_d_n13;

        let (assign94810_e147472, assign94810_e147472_d_n0, assign94810_e147472_d_n2, assign94810_e147472_d_n4, assign94810_e147472_d_n5, assign94810_e147472_d_n6, assign94810_e147472_d_n7, assign94810_e147472_d_n8, assign94810_e147472_d_n9, assign94810_e147472_d_n10, assign94810_e147472_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94810_e147470: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94810_e147470, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign94810_e147472;
        locals.var_qsuld_dn0 = assign94810_e147472_d_n0;
        locals.var_qsuld_dn2 = assign94810_e147472_d_n2;
        locals.var_qsuld_dn4 = assign94810_e147472_d_n4;
        locals.var_qsuld_dn5 = assign94810_e147472_d_n5;
        locals.var_qsuld_dn6 = assign94810_e147472_d_n6;
        locals.var_qsuld_dn7 = assign94810_e147472_d_n7;
        locals.var_qsuld_dn8 = assign94810_e147472_d_n8;
        locals.var_qsuld_dn9 = assign94810_e147472_d_n9;
        locals.var_qsuld_dn10 = assign94810_e147472_d_n10;
        locals.var_qsuld_dn13 = assign94810_e147472_d_n13;

        let (assign94820_e147480, assign94820_e147480_d_n0, assign94820_e147480_d_n2, assign94820_e147480_d_n4, assign94820_e147480_d_n5, assign94820_e147480_d_n6, assign94820_e147480_d_n7, assign94820_e147480_d_n8, assign94820_e147480_d_n9, assign94820_e147480_d_n10, assign94820_e147480_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign94820_e147478: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign94820_e147478, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign94820_e147480;
        locals.var_qiuld_dn0 = assign94820_e147480_d_n0;
        locals.var_qiuld_dn2 = assign94820_e147480_d_n2;
        locals.var_qiuld_dn4 = assign94820_e147480_d_n4;
        locals.var_qiuld_dn5 = assign94820_e147480_d_n5;
        locals.var_qiuld_dn6 = assign94820_e147480_d_n6;
        locals.var_qiuld_dn7 = assign94820_e147480_d_n7;
        locals.var_qiuld_dn8 = assign94820_e147480_d_n8;
        locals.var_qiuld_dn9 = assign94820_e147480_d_n9;
        locals.var_qiuld_dn10 = assign94820_e147480_d_n10;
        locals.var_qiuld_dn13 = assign94820_e147480_d_n13;

        let assign94830_e147483: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2211 = assign94830_e147483;

        let (assign94840_e147492, assign94840_e147492_d_n0, assign94840_e147492_d_n2, assign94840_e147492_d_n4, assign94840_e147492_d_n5, assign94840_e147492_d_n6, assign94840_e147492_d_n7, assign94840_e147492_d_n8, assign94840_e147492_d_n9, assign94840_e147492_d_n10, assign94840_e147492_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) {
        let assign94840_e147490: f64 = (-locals.var_lover_func);
        (assign94840_e147490, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign94840_e147492;
        locals.var_lover_func_dn0 = assign94840_e147492_d_n0;
        locals.var_lover_func_dn2 = assign94840_e147492_d_n2;
        locals.var_lover_func_dn4 = assign94840_e147492_d_n4;
        locals.var_lover_func_dn5 = assign94840_e147492_d_n5;
        locals.var_lover_func_dn6 = assign94840_e147492_d_n6;
        locals.var_lover_func_dn7 = assign94840_e147492_d_n7;
        locals.var_lover_func_dn8 = assign94840_e147492_d_n8;
        locals.var_lover_func_dn9 = assign94840_e147492_d_n9;
        locals.var_lover_func_dn10 = assign94840_e147492_d_n10;
        locals.var_lover_func_dn13 = assign94840_e147492_d_n13;

        let assign94850_e147495: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2212 = assign94850_e147495;

        let assign94860_e147498: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2213 = assign94860_e147498;

        let (assign94870_e147511, assign94870_e147511_d_n0, assign94870_e147511_d_n2, assign94870_e147511_d_n4, assign94870_e147511_d_n5, assign94870_e147511_d_n6, assign94870_e147511_d_n7, assign94870_e147511_d_n8, assign94870_e147511_d_n9, assign94870_e147511_d_n10, assign94870_e147511_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2213 != 0.0)) {
        let assign94870_e147509: f64 = (-locals.var_ps0ld);
        (assign94870_e147509, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk2116, locals.var_vx__blk2116_dn0, locals.var_vx__blk2116_dn2, locals.var_vx__blk2116_dn4, locals.var_vx__blk2116_dn5, locals.var_vx__blk2116_dn6, locals.var_vx__blk2116_dn7, locals.var_vx__blk2116_dn8, locals.var_vx__blk2116_dn9, locals.var_vx__blk2116_dn10, locals.var_vx__blk2116_dn13,)
    }
};
        locals.var_vx__blk2116 = assign94870_e147511;
        locals.var_vx__blk2116_dn0 = assign94870_e147511_d_n0;
        locals.var_vx__blk2116_dn2 = assign94870_e147511_d_n2;
        locals.var_vx__blk2116_dn4 = assign94870_e147511_d_n4;
        locals.var_vx__blk2116_dn5 = assign94870_e147511_d_n5;
        locals.var_vx__blk2116_dn6 = assign94870_e147511_d_n6;
        locals.var_vx__blk2116_dn7 = assign94870_e147511_d_n7;
        locals.var_vx__blk2116_dn8 = assign94870_e147511_d_n8;
        locals.var_vx__blk2116_dn9 = assign94870_e147511_d_n9;
        locals.var_vx__blk2116_dn10 = assign94870_e147511_d_n10;
        locals.var_vx__blk2116_dn13 = assign94870_e147511_d_n13;

        let (assign94880_e147524, assign94880_e147524_d_n0, assign94880_e147524_d_n2, assign94880_e147524_d_n4, assign94880_e147524_d_n5, assign94880_e147524_d_n6, assign94880_e147524_d_n7, assign94880_e147524_d_n8, assign94880_e147524_d_n9, assign94880_e147524_d_n10, assign94880_e147524_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2213 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk2116, locals.var_vx__blk2116_dn0, locals.var_vx__blk2116_dn2, locals.var_vx__blk2116_dn4, locals.var_vx__blk2116_dn5, locals.var_vx__blk2116_dn6, locals.var_vx__blk2116_dn7, locals.var_vx__blk2116_dn8, locals.var_vx__blk2116_dn9, locals.var_vx__blk2116_dn10, locals.var_vx__blk2116_dn13,)
    }
};
        locals.var_vx__blk2116 = assign94880_e147524;
        locals.var_vx__blk2116_dn0 = assign94880_e147524_d_n0;
        locals.var_vx__blk2116_dn2 = assign94880_e147524_d_n2;
        locals.var_vx__blk2116_dn4 = assign94880_e147524_d_n4;
        locals.var_vx__blk2116_dn5 = assign94880_e147524_d_n5;
        locals.var_vx__blk2116_dn6 = assign94880_e147524_d_n6;
        locals.var_vx__blk2116_dn7 = assign94880_e147524_d_n7;
        locals.var_vx__blk2116_dn8 = assign94880_e147524_d_n8;
        locals.var_vx__blk2116_dn9 = assign94880_e147524_d_n9;
        locals.var_vx__blk2116_dn10 = assign94880_e147524_d_n10;
        locals.var_vx__blk2116_dn13 = assign94880_e147524_d_n13;

        let (assign94890_e147547, assign94890_e147547_d_n0, assign94890_e147547_d_n2, assign94890_e147547_d_n4, assign94890_e147547_d_n5, assign94890_e147547_d_n6, assign94890_e147547_d_n7, assign94890_e147547_d_n8, assign94890_e147547_d_n9, assign94890_e147547_d_n10, assign94890_e147547_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94890_e147534: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94890_e147537: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94890_e147538: f64 = (assign94890_e147534 * assign94890_e147537);
        let assign94890_e147541: f64 = (4.0 * 0.1);
        let assign94890_e147543: f64 = (assign94890_e147541 * 0.1);
        let assign94890_e147544: f64 = (assign94890_e147538 + assign94890_e147543);
        let assign94890_e147545: f64 = (assign94890_e147544).sqrt();
        (assign94890_e147545, (((locals.var_vx__blk2116_dn0 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn0)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn2 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn2)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn4 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn4)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn5 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn5)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn6 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn6)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn7 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn7)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn8 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn8)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn9 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn9)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn10 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn10)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn13 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn13)) / (2.0 * assign94890_e147545)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94890_e147547;
        locals.var_tmf2_dn0 = assign94890_e147547_d_n0;
        locals.var_tmf2_dn2 = assign94890_e147547_d_n2;
        locals.var_tmf2_dn4 = assign94890_e147547_d_n4;
        locals.var_tmf2_dn5 = assign94890_e147547_d_n5;
        locals.var_tmf2_dn6 = assign94890_e147547_d_n6;
        locals.var_tmf2_dn7 = assign94890_e147547_d_n7;
        locals.var_tmf2_dn8 = assign94890_e147547_d_n8;
        locals.var_tmf2_dn9 = assign94890_e147547_d_n9;
        locals.var_tmf2_dn10 = assign94890_e147547_d_n10;
        locals.var_tmf2_dn13 = assign94890_e147547_d_n13;

        let (assign94900_e147565, assign94900_e147565_d_n0, assign94900_e147565_d_n2, assign94900_e147565_d_n4, assign94900_e147565_d_n5, assign94900_e147565_d_n6, assign94900_e147565_d_n7, assign94900_e147565_d_n8, assign94900_e147565_d_n9, assign94900_e147565_d_n10, assign94900_e147565_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94900_e147559: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94900_e147561: f64 = (assign94900_e147559 / locals.var_tmf2);
        let assign94900_e147562: f64 = (1.0 + assign94900_e147561);
        let assign94900_e147563: f64 = (0.5 * assign94900_e147562);
        (assign94900_e147563, (0.5 * (((locals.var_vx__blk2116_dn0 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn2 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn4 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn5 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn6 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn7 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn8 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn9 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn10 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn13 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94900_e147565;
        locals.var_t9_dn0 = assign94900_e147565_d_n0;
        locals.var_t9_dn2 = assign94900_e147565_d_n2;
        locals.var_t9_dn4 = assign94900_e147565_d_n4;
        locals.var_t9_dn5 = assign94900_e147565_d_n5;
        locals.var_t9_dn6 = assign94900_e147565_d_n6;
        locals.var_t9_dn7 = assign94900_e147565_d_n7;
        locals.var_t9_dn8 = assign94900_e147565_d_n8;
        locals.var_t9_dn9 = assign94900_e147565_d_n9;
        locals.var_t9_dn10 = assign94900_e147565_d_n10;
        locals.var_t9_dn13 = assign94900_e147565_d_n13;

        let (assign94910_e147581, assign94910_e147581_d_n0, assign94910_e147581_d_n2, assign94910_e147581_d_n4, assign94910_e147581_d_n5, assign94910_e147581_d_n6, assign94910_e147581_d_n7, assign94910_e147581_d_n8, assign94910_e147581_d_n9, assign94910_e147581_d_n10, assign94910_e147581_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94910_e147576: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94910_e147578: f64 = (assign94910_e147576 + locals.var_tmf2);
        let assign94910_e147579: f64 = (0.5 * assign94910_e147578);
        (assign94910_e147579, (0.5 * (locals.var_vx__blk2116_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2116_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2116_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2116_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2116_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2116_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2116_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2116_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2116_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2116_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94910_e147581;
        locals.var_t2_dn0 = assign94910_e147581_d_n0;
        locals.var_t2_dn2 = assign94910_e147581_d_n2;
        locals.var_t2_dn4 = assign94910_e147581_d_n4;
        locals.var_t2_dn5 = assign94910_e147581_d_n5;
        locals.var_t2_dn6 = assign94910_e147581_d_n6;
        locals.var_t2_dn7 = assign94910_e147581_d_n7;
        locals.var_t2_dn8 = assign94910_e147581_d_n8;
        locals.var_t2_dn9 = assign94910_e147581_d_n9;
        locals.var_t2_dn10 = assign94910_e147581_d_n10;
        locals.var_t2_dn13 = assign94910_e147581_d_n13;

        let assign94920_e147584: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2214 = assign94920_e147584;

        let (assign94930_e147596, assign94930_e147596_d_n0, assign94930_e147596_d_n2, assign94930_e147596_d_n4, assign94930_e147596_d_n5, assign94930_e147596_d_n6, assign94930_e147596_d_n7, assign94930_e147596_d_n8, assign94930_e147596_d_n9, assign94930_e147596_d_n10, assign94930_e147596_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94930_e147596;
        locals.var_t2_dn0 = assign94930_e147596_d_n0;
        locals.var_t2_dn2 = assign94930_e147596_d_n2;
        locals.var_t2_dn4 = assign94930_e147596_d_n4;
        locals.var_t2_dn5 = assign94930_e147596_d_n5;
        locals.var_t2_dn6 = assign94930_e147596_d_n6;
        locals.var_t2_dn7 = assign94930_e147596_d_n7;
        locals.var_t2_dn8 = assign94930_e147596_d_n8;
        locals.var_t2_dn9 = assign94930_e147596_d_n9;
        locals.var_t2_dn10 = assign94930_e147596_d_n10;
        locals.var_t2_dn13 = assign94930_e147596_d_n13;

        let (assign94940_e147608, assign94940_e147608_d_n0, assign94940_e147608_d_n2, assign94940_e147608_d_n4, assign94940_e147608_d_n5, assign94940_e147608_d_n6, assign94940_e147608_d_n7, assign94940_e147608_d_n8, assign94940_e147608_d_n9, assign94940_e147608_d_n10, assign94940_e147608_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94940_e147608;
        locals.var_t9_dn0 = assign94940_e147608_d_n0;
        locals.var_t9_dn2 = assign94940_e147608_d_n2;
        locals.var_t9_dn4 = assign94940_e147608_d_n4;
        locals.var_t9_dn5 = assign94940_e147608_d_n5;
        locals.var_t9_dn6 = assign94940_e147608_d_n6;
        locals.var_t9_dn7 = assign94940_e147608_d_n7;
        locals.var_t9_dn8 = assign94940_e147608_d_n8;
        locals.var_t9_dn9 = assign94940_e147608_d_n9;
        locals.var_t9_dn10 = assign94940_e147608_d_n10;
        locals.var_t9_dn13 = assign94940_e147608_d_n13;

        let (assign94950_e147623, assign94950_e147623_d_n0, assign94950_e147623_d_n2, assign94950_e147623_d_n4, assign94950_e147623_d_n5, assign94950_e147623_d_n6, assign94950_e147623_d_n7, assign94950_e147623_d_n8, assign94950_e147623_d_n9, assign94950_e147623_d_n10, assign94950_e147623_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94950_e147618: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94950_e147619: f64 = (assign94950_e147618).sqrt();
        let assign94950_e147621: f64 = (assign94950_e147619 * p.p432);
        (assign94950_e147621, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign94950_e147619)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign94950_e147623;
        locals.var_wjunc0_dn0 = assign94950_e147623_d_n0;
        locals.var_wjunc0_dn2 = assign94950_e147623_d_n2;
        locals.var_wjunc0_dn4 = assign94950_e147623_d_n4;
        locals.var_wjunc0_dn5 = assign94950_e147623_d_n5;
        locals.var_wjunc0_dn6 = assign94950_e147623_d_n6;
        locals.var_wjunc0_dn7 = assign94950_e147623_d_n7;
        locals.var_wjunc0_dn8 = assign94950_e147623_d_n8;
        locals.var_wjunc0_dn9 = assign94950_e147623_d_n9;
        locals.var_wjunc0_dn10 = assign94950_e147623_d_n10;
        locals.var_wjunc0_dn13 = assign94950_e147623_d_n13;

        let (assign94960_e147639, assign94960_e147639_d_n0, assign94960_e147639_d_n2, assign94960_e147639_d_n4, assign94960_e147639_d_n5, assign94960_e147639_d_n6, assign94960_e147639_d_n7, assign94960_e147639_d_n8, assign94960_e147639_d_n9, assign94960_e147639_d_n10, assign94960_e147639_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94960_e147633: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign94960_e147636: f64 = (0.1 * locals.var_lover_func);
        let assign94960_e147637: f64 = (assign94960_e147633 - assign94960_e147636);
        (assign94960_e147637, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign94960_e147639;
        locals.var_tmf1_dn0 = assign94960_e147639_d_n0;
        locals.var_tmf1_dn2 = assign94960_e147639_d_n2;
        locals.var_tmf1_dn4 = assign94960_e147639_d_n4;
        locals.var_tmf1_dn5 = assign94960_e147639_d_n5;
        locals.var_tmf1_dn6 = assign94960_e147639_d_n6;
        locals.var_tmf1_dn7 = assign94960_e147639_d_n7;
        locals.var_tmf1_dn8 = assign94960_e147639_d_n8;
        locals.var_tmf1_dn9 = assign94960_e147639_d_n9;
        locals.var_tmf1_dn10 = assign94960_e147639_d_n10;
        locals.var_tmf1_dn13 = assign94960_e147639_d_n13;

        let (assign94970_e147655, assign94970_e147655_d_n0, assign94970_e147655_d_n2, assign94970_e147655_d_n4, assign94970_e147655_d_n5, assign94970_e147655_d_n6, assign94970_e147655_d_n7, assign94970_e147655_d_n8, assign94970_e147655_d_n9, assign94970_e147655_d_n10, assign94970_e147655_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94970_e147649: f64 = (4.0 * locals.var_lover_func);
        let assign94970_e147652: f64 = (0.1 * locals.var_lover_func);
        let assign94970_e147653: f64 = (assign94970_e147649 * assign94970_e147652);
        (assign94970_e147653, (((4.0 * locals.var_lover_func_dn0) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94970_e147655;
        locals.var_tmf2_dn0 = assign94970_e147655_d_n0;
        locals.var_tmf2_dn2 = assign94970_e147655_d_n2;
        locals.var_tmf2_dn4 = assign94970_e147655_d_n4;
        locals.var_tmf2_dn5 = assign94970_e147655_d_n5;
        locals.var_tmf2_dn6 = assign94970_e147655_d_n6;
        locals.var_tmf2_dn7 = assign94970_e147655_d_n7;
        locals.var_tmf2_dn8 = assign94970_e147655_d_n8;
        locals.var_tmf2_dn9 = assign94970_e147655_d_n9;
        locals.var_tmf2_dn10 = assign94970_e147655_d_n10;
        locals.var_tmf2_dn13 = assign94970_e147655_d_n13;

        let (assign94980_e147671, assign94980_e147671_d_n0, assign94980_e147671_d_n2, assign94980_e147671_d_n4, assign94980_e147671_d_n5, assign94980_e147671_d_n6, assign94980_e147671_d_n7, assign94980_e147671_d_n8, assign94980_e147671_d_n9, assign94980_e147671_d_n10, assign94980_e147671_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let (assign94980_e147669, assign94980_e147669_d_n0, assign94980_e147669_d_n2, assign94980_e147669_d_n4, assign94980_e147669_d_n5, assign94980_e147669_d_n6, assign94980_e147669_d_n7, assign94980_e147669_d_n8, assign94980_e147669_d_n9, assign94980_e147669_d_n10, assign94980_e147669_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign94980_e147668: f64 = (-locals.var_tmf2);
                (assign94980_e147668, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign94980_e147669, assign94980_e147669_d_n0, assign94980_e147669_d_n2, assign94980_e147669_d_n4, assign94980_e147669_d_n5, assign94980_e147669_d_n6, assign94980_e147669_d_n7, assign94980_e147669_d_n8, assign94980_e147669_d_n9, assign94980_e147669_d_n10, assign94980_e147669_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94980_e147671;
        locals.var_tmf2_dn0 = assign94980_e147671_d_n0;
        locals.var_tmf2_dn2 = assign94980_e147671_d_n2;
        locals.var_tmf2_dn4 = assign94980_e147671_d_n4;
        locals.var_tmf2_dn5 = assign94980_e147671_d_n5;
        locals.var_tmf2_dn6 = assign94980_e147671_d_n6;
        locals.var_tmf2_dn7 = assign94980_e147671_d_n7;
        locals.var_tmf2_dn8 = assign94980_e147671_d_n8;
        locals.var_tmf2_dn9 = assign94980_e147671_d_n9;
        locals.var_tmf2_dn10 = assign94980_e147671_d_n10;
        locals.var_tmf2_dn13 = assign94980_e147671_d_n13;

        let (assign94990_e147686, assign94990_e147686_d_n0, assign94990_e147686_d_n2, assign94990_e147686_d_n4, assign94990_e147686_d_n5, assign94990_e147686_d_n6, assign94990_e147686_d_n7, assign94990_e147686_d_n8, assign94990_e147686_d_n9, assign94990_e147686_d_n10, assign94990_e147686_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94990_e147681: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign94990_e147683: f64 = (assign94990_e147681 + locals.var_tmf2);
        let assign94990_e147684: f64 = (assign94990_e147683).sqrt();
        (assign94990_e147684, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign94990_e147684)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94990_e147686;
        locals.var_tmf2_dn0 = assign94990_e147686_d_n0;
        locals.var_tmf2_dn2 = assign94990_e147686_d_n2;
        locals.var_tmf2_dn4 = assign94990_e147686_d_n4;
        locals.var_tmf2_dn5 = assign94990_e147686_d_n5;
        locals.var_tmf2_dn6 = assign94990_e147686_d_n6;
        locals.var_tmf2_dn7 = assign94990_e147686_d_n7;
        locals.var_tmf2_dn8 = assign94990_e147686_d_n8;
        locals.var_tmf2_dn9 = assign94990_e147686_d_n9;
        locals.var_tmf2_dn10 = assign94990_e147686_d_n10;
        locals.var_tmf2_dn13 = assign94990_e147686_d_n13;

    }

    pub(super) fn stamp_transient_block_339(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95000_e147702, assign95000_e147702_d_n0, assign95000_e147702_d_n2, assign95000_e147702_d_n4, assign95000_e147702_d_n5, assign95000_e147702_d_n6, assign95000_e147702_d_n7, assign95000_e147702_d_n8, assign95000_e147702_d_n9, assign95000_e147702_d_n10, assign95000_e147702_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95000_e147698: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign95000_e147699: f64 = (1.0 + assign95000_e147698);
        let assign95000_e147700: f64 = (0.5 * assign95000_e147699);
        (assign95000_e147700, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95000_e147702;
        locals.var_t0_dn0 = assign95000_e147702_d_n0;
        locals.var_t0_dn2 = assign95000_e147702_d_n2;
        locals.var_t0_dn4 = assign95000_e147702_d_n4;
        locals.var_t0_dn5 = assign95000_e147702_d_n5;
        locals.var_t0_dn6 = assign95000_e147702_d_n6;
        locals.var_t0_dn7 = assign95000_e147702_d_n7;
        locals.var_t0_dn8 = assign95000_e147702_d_n8;
        locals.var_t0_dn9 = assign95000_e147702_d_n9;
        locals.var_t0_dn10 = assign95000_e147702_d_n10;
        locals.var_t0_dn13 = assign95000_e147702_d_n13;

        let (assign95010_e147718, assign95010_e147718_d_n0, assign95010_e147718_d_n2, assign95010_e147718_d_n4, assign95010_e147718_d_n5, assign95010_e147718_d_n6, assign95010_e147718_d_n7, assign95010_e147718_d_n8, assign95010_e147718_d_n9, assign95010_e147718_d_n10, assign95010_e147718_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95010_e147714: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign95010_e147715: f64 = (0.5 * assign95010_e147714);
        let assign95010_e147716: f64 = (locals.var_lover_func - assign95010_e147715);
        (assign95010_e147716, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign95010_e147718;
        locals.var_wjuncld_dn0 = assign95010_e147718_d_n0;
        locals.var_wjuncld_dn2 = assign95010_e147718_d_n2;
        locals.var_wjuncld_dn4 = assign95010_e147718_d_n4;
        locals.var_wjuncld_dn5 = assign95010_e147718_d_n5;
        locals.var_wjuncld_dn6 = assign95010_e147718_d_n6;
        locals.var_wjuncld_dn7 = assign95010_e147718_d_n7;
        locals.var_wjuncld_dn8 = assign95010_e147718_d_n8;
        locals.var_wjuncld_dn9 = assign95010_e147718_d_n9;
        locals.var_wjuncld_dn10 = assign95010_e147718_d_n10;
        locals.var_wjuncld_dn13 = assign95010_e147718_d_n13;

        let (assign95020_e147730, assign95020_e147730_d_n0, assign95020_e147730_d_n2, assign95020_e147730_d_n4, assign95020_e147730_d_n5, assign95020_e147730_d_n6, assign95020_e147730_d_n7, assign95020_e147730_d_n8, assign95020_e147730_d_n9, assign95020_e147730_d_n10, assign95020_e147730_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95020_e147728: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign95020_e147728, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign95020_e147730;
        locals.var_lover_func_dn0 = assign95020_e147730_d_n0;
        locals.var_lover_func_dn2 = assign95020_e147730_d_n2;
        locals.var_lover_func_dn4 = assign95020_e147730_d_n4;
        locals.var_lover_func_dn5 = assign95020_e147730_d_n5;
        locals.var_lover_func_dn6 = assign95020_e147730_d_n6;
        locals.var_lover_func_dn7 = assign95020_e147730_d_n7;
        locals.var_lover_func_dn8 = assign95020_e147730_d_n8;
        locals.var_lover_func_dn9 = assign95020_e147730_d_n9;
        locals.var_lover_func_dn10 = assign95020_e147730_d_n10;
        locals.var_lover_func_dn13 = assign95020_e147730_d_n13;

        let (assign95030_e147736, assign95030_e147736_d_n0, assign95030_e147736_d_n2, assign95030_e147736_d_n4, assign95030_e147736_d_n5, assign95030_e147736_d_n6, assign95030_e147736_d_n7, assign95030_e147736_d_n8, assign95030_e147736_d_n9, assign95030_e147736_d_n10, assign95030_e147736_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign95030_e147736;
        locals.var_rd_qbuld_dn0 = assign95030_e147736_d_n0;
        locals.var_rd_qbuld_dn2 = assign95030_e147736_d_n2;
        locals.var_rd_qbuld_dn4 = assign95030_e147736_d_n4;
        locals.var_rd_qbuld_dn5 = assign95030_e147736_d_n5;
        locals.var_rd_qbuld_dn6 = assign95030_e147736_d_n6;
        locals.var_rd_qbuld_dn7 = assign95030_e147736_d_n7;
        locals.var_rd_qbuld_dn8 = assign95030_e147736_d_n8;
        locals.var_rd_qbuld_dn9 = assign95030_e147736_d_n9;
        locals.var_rd_qbuld_dn10 = assign95030_e147736_d_n10;
        locals.var_rd_qbuld_dn13 = assign95030_e147736_d_n13;

        let assign95040_e147747: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2215 = assign95040_e147747;

        let (assign95050_e147751,) = {
    if (locals.var_guard2215 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign95050_e147751;

        let (assign95060_e147755,) = {
    if (locals.var_guard2215 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95060_e147755;

        let (assign95070_e147759,) = {
    if (locals.var_guard2215 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95070_e147759;

        let (assign95080_e147765, assign95080_e147765_d_n0, assign95080_e147765_d_n2, assign95080_e147765_d_n4, assign95080_e147765_d_n5, assign95080_e147765_d_n6, assign95080_e147765_d_n7, assign95080_e147765_d_n8, assign95080_e147765_d_n9, assign95080_e147765_d_n10, assign95080_e147765_d_n13,) = {
    if (locals.var_guard2215 != 0.0) {
        let assign95080_e147763: f64 = (locals.var_cox0 * locals.var_weffcv_nf);
        (assign95080_e147763, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95080_e147765;
        locals.var_t1_dn0 = assign95080_e147765_d_n0;
        locals.var_t1_dn2 = assign95080_e147765_d_n2;
        locals.var_t1_dn4 = assign95080_e147765_d_n4;
        locals.var_t1_dn5 = assign95080_e147765_d_n5;
        locals.var_t1_dn6 = assign95080_e147765_d_n6;
        locals.var_t1_dn7 = assign95080_e147765_d_n7;
        locals.var_t1_dn8 = assign95080_e147765_d_n8;
        locals.var_t1_dn9 = assign95080_e147765_d_n9;
        locals.var_t1_dn10 = assign95080_e147765_d_n10;
        locals.var_t1_dn13 = assign95080_e147765_d_n13;

        let assign95090_e147768: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2216 = assign95090_e147768;

        let (assign95100_e147780, assign95100_e147780_d_n0, assign95100_e147780_d_n2, assign95100_e147780_d_n4, assign95100_e147780_d_n5, assign95100_e147780_d_n6, assign95100_e147780_d_n7, assign95100_e147780_d_n8, assign95100_e147780_d_n9, assign95100_e147780_d_n10, assign95100_e147780_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95100_e147774: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95100_e147777: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95100_e147778: f64 = (assign95100_e147774 * assign95100_e147777);
        (assign95100_e147778, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95100_e147777), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn5)), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn7)), ((locals.var_cov_slp * locals.var_t1_dn8) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn13) * assign95100_e147777),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95100_e147780;
        locals.var_t4_dn0 = assign95100_e147780_d_n0;
        locals.var_t4_dn2 = assign95100_e147780_d_n2;
        locals.var_t4_dn4 = assign95100_e147780_d_n4;
        locals.var_t4_dn5 = assign95100_e147780_d_n5;
        locals.var_t4_dn6 = assign95100_e147780_d_n6;
        locals.var_t4_dn7 = assign95100_e147780_d_n7;
        locals.var_t4_dn8 = assign95100_e147780_d_n8;
        locals.var_t4_dn9 = assign95100_e147780_d_n9;
        locals.var_t4_dn10 = assign95100_e147780_d_n10;
        locals.var_t4_dn13 = assign95100_e147780_d_n13;

        let (assign95110_e147788, assign95110_e147788_d_n0, assign95110_e147788_d_n2, assign95110_e147788_d_n4, assign95110_e147788_d_n5, assign95110_e147788_d_n6, assign95110_e147788_d_n7, assign95110_e147788_d_n8, assign95110_e147788_d_n9, assign95110_e147788_d_n10, assign95110_e147788_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95110_e147786: f64 = (p.p66 * locals.var_t1);
        (assign95110_e147786, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95110_e147788;
        locals.var_t5_dn0 = assign95110_e147788_d_n0;
        locals.var_t5_dn2 = assign95110_e147788_d_n2;
        locals.var_t5_dn4 = assign95110_e147788_d_n4;
        locals.var_t5_dn5 = assign95110_e147788_d_n5;
        locals.var_t5_dn6 = assign95110_e147788_d_n6;
        locals.var_t5_dn7 = assign95110_e147788_d_n7;
        locals.var_t5_dn8 = assign95110_e147788_d_n8;
        locals.var_t5_dn9 = assign95110_e147788_d_n9;
        locals.var_t5_dn10 = assign95110_e147788_d_n10;
        locals.var_t5_dn13 = assign95110_e147788_d_n13;

        let (assign95120_e147796, assign95120_e147796_d_n0, assign95120_e147796_d_n2, assign95120_e147796_d_n4, assign95120_e147796_d_n5, assign95120_e147796_d_n6, assign95120_e147796_d_n7, assign95120_e147796_d_n8, assign95120_e147796_d_n9, assign95120_e147796_d_n10, assign95120_e147796_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95120_e147794: f64 = (1.2 - locals.var_ps0);
        (assign95120_e147794, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95120_e147796;
        locals.var_t9_dn0 = assign95120_e147796_d_n0;
        locals.var_t9_dn2 = assign95120_e147796_d_n2;
        locals.var_t9_dn4 = assign95120_e147796_d_n4;
        locals.var_t9_dn5 = assign95120_e147796_d_n5;
        locals.var_t9_dn6 = assign95120_e147796_d_n6;
        locals.var_t9_dn7 = assign95120_e147796_d_n7;
        locals.var_t9_dn8 = assign95120_e147796_d_n8;
        locals.var_t9_dn9 = assign95120_e147796_d_n9;
        locals.var_t9_dn10 = assign95120_e147796_d_n10;
        locals.var_t9_dn13 = assign95120_e147796_d_n13;

        let (assign95130_e147808, assign95130_e147808_d_n0, assign95130_e147808_d_n2, assign95130_e147808_d_n4, assign95130_e147808_d_n5, assign95130_e147808_d_n6, assign95130_e147808_d_n7, assign95130_e147808_d_n8, assign95130_e147808_d_n9, assign95130_e147808_d_n10, assign95130_e147808_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95130_e147802: f64 = (locals.var_vgs * locals.var_t5);
        let assign95130_e147805: f64 = (locals.var_t4 * locals.var_t9);
        let assign95130_e147806: f64 = (assign95130_e147802 - assign95130_e147805);
        (assign95130_e147806, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), (((locals.var_vgs_dn5 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((locals.var_vgs * locals.var_t5_dn8) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn13) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn13,)
    }
};
        locals.var_qgos = assign95130_e147808;
        locals.var_qgos_dn0 = assign95130_e147808_d_n0;
        locals.var_qgos_dn2 = assign95130_e147808_d_n2;
        locals.var_qgos_dn4 = assign95130_e147808_d_n4;
        locals.var_qgos_dn5 = assign95130_e147808_d_n5;
        locals.var_qgos_dn6 = assign95130_e147808_d_n6;
        locals.var_qgos_dn7 = assign95130_e147808_d_n7;
        locals.var_qgos_dn8 = assign95130_e147808_d_n8;
        locals.var_qgos_dn9 = assign95130_e147808_d_n9;
        locals.var_qgos_dn10 = assign95130_e147808_d_n10;
        locals.var_qgos_dn13 = assign95130_e147808_d_n13;

        let (assign95140_e147823, assign95140_e147823_d_n0, assign95140_e147823_d_n2, assign95140_e147823_d_n4, assign95140_e147823_d_n5, assign95140_e147823_d_n6, assign95140_e147823_d_n7, assign95140_e147823_d_n8, assign95140_e147823_d_n9, assign95140_e147823_d_n10, assign95140_e147823_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95140_e147815: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95140_e147818: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95140_e147820: f64 = (assign95140_e147818 - locals.var_vds);
        let assign95140_e147821: f64 = (assign95140_e147815 * assign95140_e147820);
        (assign95140_e147821, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn13) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95140_e147823;
        locals.var_t4_dn0 = assign95140_e147823_d_n0;
        locals.var_t4_dn2 = assign95140_e147823_d_n2;
        locals.var_t4_dn4 = assign95140_e147823_d_n4;
        locals.var_t4_dn5 = assign95140_e147823_d_n5;
        locals.var_t4_dn6 = assign95140_e147823_d_n6;
        locals.var_t4_dn7 = assign95140_e147823_d_n7;
        locals.var_t4_dn8 = assign95140_e147823_d_n8;
        locals.var_t4_dn9 = assign95140_e147823_d_n9;
        locals.var_t4_dn10 = assign95140_e147823_d_n10;
        locals.var_t4_dn13 = assign95140_e147823_d_n13;

        let (assign95150_e147832, assign95150_e147832_d_n0, assign95150_e147832_d_n2, assign95150_e147832_d_n4, assign95150_e147832_d_n5, assign95150_e147832_d_n6, assign95150_e147832_d_n7, assign95150_e147832_d_n8, assign95150_e147832_d_n9, assign95150_e147832_d_n10, assign95150_e147832_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95150_e147830: f64 = (p.p66 * locals.var_t1);
        (assign95150_e147830, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95150_e147832;
        locals.var_t5_dn0 = assign95150_e147832_d_n0;
        locals.var_t5_dn2 = assign95150_e147832_d_n2;
        locals.var_t5_dn4 = assign95150_e147832_d_n4;
        locals.var_t5_dn5 = assign95150_e147832_d_n5;
        locals.var_t5_dn6 = assign95150_e147832_d_n6;
        locals.var_t5_dn7 = assign95150_e147832_d_n7;
        locals.var_t5_dn8 = assign95150_e147832_d_n8;
        locals.var_t5_dn9 = assign95150_e147832_d_n9;
        locals.var_t5_dn10 = assign95150_e147832_d_n10;
        locals.var_t5_dn13 = assign95150_e147832_d_n13;

        let (assign95160_e147843, assign95160_e147843_d_n0, assign95160_e147843_d_n2, assign95160_e147843_d_n4, assign95160_e147843_d_n5, assign95160_e147843_d_n6, assign95160_e147843_d_n7, assign95160_e147843_d_n8, assign95160_e147843_d_n9, assign95160_e147843_d_n10, assign95160_e147843_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95160_e147839: f64 = (1.2 + locals.var_vds);
        let assign95160_e147841: f64 = (assign95160_e147839 - locals.var_psl);
        (assign95160_e147841, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95160_e147843;
        locals.var_t9_dn0 = assign95160_e147843_d_n0;
        locals.var_t9_dn2 = assign95160_e147843_d_n2;
        locals.var_t9_dn4 = assign95160_e147843_d_n4;
        locals.var_t9_dn5 = assign95160_e147843_d_n5;
        locals.var_t9_dn6 = assign95160_e147843_d_n6;
        locals.var_t9_dn7 = assign95160_e147843_d_n7;
        locals.var_t9_dn8 = assign95160_e147843_d_n8;
        locals.var_t9_dn9 = assign95160_e147843_d_n9;
        locals.var_t9_dn10 = assign95160_e147843_d_n10;
        locals.var_t9_dn13 = assign95160_e147843_d_n13;

        let (assign95170_e147858, assign95170_e147858_d_n0, assign95170_e147858_d_n2, assign95170_e147858_d_n4, assign95170_e147858_d_n5, assign95170_e147858_d_n6, assign95170_e147858_d_n7, assign95170_e147858_d_n8, assign95170_e147858_d_n9, assign95170_e147858_d_n10, assign95170_e147858_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95170_e147850: f64 = (locals.var_vgs - locals.var_vds);
        let assign95170_e147852: f64 = (assign95170_e147850 * locals.var_t5);
        let assign95170_e147855: f64 = (locals.var_t4 * locals.var_t9);
        let assign95170_e147856: f64 = (assign95170_e147852 - assign95170_e147855);
        (assign95170_e147856, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((locals.var_vgs_dn5 - locals.var_vds_dn5) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((-locals.var_vds_dn8) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn13) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn13)) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn13,)
    }
};
        locals.var_qgos = assign95170_e147858;
        locals.var_qgos_dn0 = assign95170_e147858_d_n0;
        locals.var_qgos_dn2 = assign95170_e147858_d_n2;
        locals.var_qgos_dn4 = assign95170_e147858_d_n4;
        locals.var_qgos_dn5 = assign95170_e147858_d_n5;
        locals.var_qgos_dn6 = assign95170_e147858_d_n6;
        locals.var_qgos_dn7 = assign95170_e147858_d_n7;
        locals.var_qgos_dn8 = assign95170_e147858_d_n8;
        locals.var_qgos_dn9 = assign95170_e147858_d_n9;
        locals.var_qgos_dn10 = assign95170_e147858_d_n10;
        locals.var_qgos_dn13 = assign95170_e147858_d_n13;

        let assign95180_e147869: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2217 = assign95180_e147869;

        let (assign95190_e147873,) = {
    if (locals.var_guard2217 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign95190_e147873;

        let (assign95200_e147877,) = {
    if (locals.var_guard2217 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95200_e147877;

        let (assign95210_e147881,) = {
    if (locals.var_guard2217 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95210_e147881;

        let (assign95220_e147887, assign95220_e147887_d_n0, assign95220_e147887_d_n2, assign95220_e147887_d_n4, assign95220_e147887_d_n5, assign95220_e147887_d_n6, assign95220_e147887_d_n7, assign95220_e147887_d_n8, assign95220_e147887_d_n9, assign95220_e147887_d_n10, assign95220_e147887_d_n13,) = {
    if (locals.var_guard2217 != 0.0) {
        let assign95220_e147885: f64 = (locals.var_coxb0 * locals.var_weffcv_nf);
        (assign95220_e147885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95220_e147887;
        locals.var_t1_dn0 = assign95220_e147887_d_n0;
        locals.var_t1_dn2 = assign95220_e147887_d_n2;
        locals.var_t1_dn4 = assign95220_e147887_d_n4;
        locals.var_t1_dn5 = assign95220_e147887_d_n5;
        locals.var_t1_dn6 = assign95220_e147887_d_n6;
        locals.var_t1_dn7 = assign95220_e147887_d_n7;
        locals.var_t1_dn8 = assign95220_e147887_d_n8;
        locals.var_t1_dn9 = assign95220_e147887_d_n9;
        locals.var_t1_dn10 = assign95220_e147887_d_n10;
        locals.var_t1_dn13 = assign95220_e147887_d_n13;

        let assign95230_e147890: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2218 = assign95230_e147890;

        let (assign95240_e147904, assign95240_e147904_d_n0, assign95240_e147904_d_n2, assign95240_e147904_d_n4, assign95240_e147904_d_n5, assign95240_e147904_d_n6, assign95240_e147904_d_n7, assign95240_e147904_d_n8, assign95240_e147904_d_n9, assign95240_e147904_d_n10, assign95240_e147904_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95240_e147896: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95240_e147899: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95240_e147901: f64 = (assign95240_e147899 - locals.var_vds);
        let assign95240_e147902: f64 = (assign95240_e147896 * assign95240_e147901);
        (assign95240_e147902, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn13) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95240_e147904;
        locals.var_t4_dn0 = assign95240_e147904_d_n0;
        locals.var_t4_dn2 = assign95240_e147904_d_n2;
        locals.var_t4_dn4 = assign95240_e147904_d_n4;
        locals.var_t4_dn5 = assign95240_e147904_d_n5;
        locals.var_t4_dn6 = assign95240_e147904_d_n6;
        locals.var_t4_dn7 = assign95240_e147904_d_n7;
        locals.var_t4_dn8 = assign95240_e147904_d_n8;
        locals.var_t4_dn9 = assign95240_e147904_d_n9;
        locals.var_t4_dn10 = assign95240_e147904_d_n10;
        locals.var_t4_dn13 = assign95240_e147904_d_n13;

        let (assign95250_e147912, assign95250_e147912_d_n0, assign95250_e147912_d_n2, assign95250_e147912_d_n4, assign95250_e147912_d_n5, assign95250_e147912_d_n6, assign95250_e147912_d_n7, assign95250_e147912_d_n8, assign95250_e147912_d_n9, assign95250_e147912_d_n10, assign95250_e147912_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95250_e147910: f64 = (p.p63 * locals.var_t1);
        (assign95250_e147910, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95250_e147912;
        locals.var_t5_dn0 = assign95250_e147912_d_n0;
        locals.var_t5_dn2 = assign95250_e147912_d_n2;
        locals.var_t5_dn4 = assign95250_e147912_d_n4;
        locals.var_t5_dn5 = assign95250_e147912_d_n5;
        locals.var_t5_dn6 = assign95250_e147912_d_n6;
        locals.var_t5_dn7 = assign95250_e147912_d_n7;
        locals.var_t5_dn8 = assign95250_e147912_d_n8;
        locals.var_t5_dn9 = assign95250_e147912_d_n9;
        locals.var_t5_dn10 = assign95250_e147912_d_n10;
        locals.var_t5_dn13 = assign95250_e147912_d_n13;

        let (assign95260_e147922, assign95260_e147922_d_n0, assign95260_e147922_d_n2, assign95260_e147922_d_n4, assign95260_e147922_d_n5, assign95260_e147922_d_n6, assign95260_e147922_d_n7, assign95260_e147922_d_n8, assign95260_e147922_d_n9, assign95260_e147922_d_n10, assign95260_e147922_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95260_e147918: f64 = (1.2 + locals.var_vds);
        let assign95260_e147920: f64 = (assign95260_e147918 - locals.var_psl);
        (assign95260_e147920, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95260_e147922;
        locals.var_t9_dn0 = assign95260_e147922_d_n0;
        locals.var_t9_dn2 = assign95260_e147922_d_n2;
        locals.var_t9_dn4 = assign95260_e147922_d_n4;
        locals.var_t9_dn5 = assign95260_e147922_d_n5;
        locals.var_t9_dn6 = assign95260_e147922_d_n6;
        locals.var_t9_dn7 = assign95260_e147922_d_n7;
        locals.var_t9_dn8 = assign95260_e147922_d_n8;
        locals.var_t9_dn9 = assign95260_e147922_d_n9;
        locals.var_t9_dn10 = assign95260_e147922_d_n10;
        locals.var_t9_dn13 = assign95260_e147922_d_n13;

        let (assign95270_e147936, assign95270_e147936_d_n0, assign95270_e147936_d_n2, assign95270_e147936_d_n4, assign95270_e147936_d_n5, assign95270_e147936_d_n6, assign95270_e147936_d_n7, assign95270_e147936_d_n8, assign95270_e147936_d_n9, assign95270_e147936_d_n10, assign95270_e147936_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95270_e147928: f64 = (locals.var_vgs - locals.var_vds);
        let assign95270_e147930: f64 = (assign95270_e147928 * locals.var_t5);
        let assign95270_e147933: f64 = (locals.var_t4 * locals.var_t9);
        let assign95270_e147934: f64 = (assign95270_e147930 - assign95270_e147933);
        (assign95270_e147934, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((locals.var_vgs_dn5 - locals.var_vds_dn5) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((-locals.var_vds_dn8) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn13) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn13)) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn13,)
    }
};
        locals.var_qgod = assign95270_e147936;
        locals.var_qgod_dn0 = assign95270_e147936_d_n0;
        locals.var_qgod_dn2 = assign95270_e147936_d_n2;
        locals.var_qgod_dn4 = assign95270_e147936_d_n4;
        locals.var_qgod_dn5 = assign95270_e147936_d_n5;
        locals.var_qgod_dn6 = assign95270_e147936_d_n6;
        locals.var_qgod_dn7 = assign95270_e147936_d_n7;
        locals.var_qgod_dn8 = assign95270_e147936_d_n8;
        locals.var_qgod_dn9 = assign95270_e147936_d_n9;
        locals.var_qgod_dn10 = assign95270_e147936_d_n10;
        locals.var_qgod_dn13 = assign95270_e147936_d_n13;

        let (assign95280_e147949, assign95280_e147949_d_n0, assign95280_e147949_d_n2, assign95280_e147949_d_n4, assign95280_e147949_d_n5, assign95280_e147949_d_n6, assign95280_e147949_d_n7, assign95280_e147949_d_n8, assign95280_e147949_d_n9, assign95280_e147949_d_n10, assign95280_e147949_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95280_e147943: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95280_e147946: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95280_e147947: f64 = (assign95280_e147943 * assign95280_e147946);
        (assign95280_e147947, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95280_e147946), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn5)), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn7)), ((locals.var_cov_slp * locals.var_t1_dn8) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn13) * assign95280_e147946),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95280_e147949;
        locals.var_t4_dn0 = assign95280_e147949_d_n0;
        locals.var_t4_dn2 = assign95280_e147949_d_n2;
        locals.var_t4_dn4 = assign95280_e147949_d_n4;
        locals.var_t4_dn5 = assign95280_e147949_d_n5;
        locals.var_t4_dn6 = assign95280_e147949_d_n6;
        locals.var_t4_dn7 = assign95280_e147949_d_n7;
        locals.var_t4_dn8 = assign95280_e147949_d_n8;
        locals.var_t4_dn9 = assign95280_e147949_d_n9;
        locals.var_t4_dn10 = assign95280_e147949_d_n10;
        locals.var_t4_dn13 = assign95280_e147949_d_n13;

        let (assign95290_e147958, assign95290_e147958_d_n0, assign95290_e147958_d_n2, assign95290_e147958_d_n4, assign95290_e147958_d_n5, assign95290_e147958_d_n6, assign95290_e147958_d_n7, assign95290_e147958_d_n8, assign95290_e147958_d_n9, assign95290_e147958_d_n10, assign95290_e147958_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95290_e147956: f64 = (p.p63 * locals.var_t1);
        (assign95290_e147956, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95290_e147958;
        locals.var_t5_dn0 = assign95290_e147958_d_n0;
        locals.var_t5_dn2 = assign95290_e147958_d_n2;
        locals.var_t5_dn4 = assign95290_e147958_d_n4;
        locals.var_t5_dn5 = assign95290_e147958_d_n5;
        locals.var_t5_dn6 = assign95290_e147958_d_n6;
        locals.var_t5_dn7 = assign95290_e147958_d_n7;
        locals.var_t5_dn8 = assign95290_e147958_d_n8;
        locals.var_t5_dn9 = assign95290_e147958_d_n9;
        locals.var_t5_dn10 = assign95290_e147958_d_n10;
        locals.var_t5_dn13 = assign95290_e147958_d_n13;

        let (assign95300_e147967, assign95300_e147967_d_n0, assign95300_e147967_d_n2, assign95300_e147967_d_n4, assign95300_e147967_d_n5, assign95300_e147967_d_n6, assign95300_e147967_d_n7, assign95300_e147967_d_n8, assign95300_e147967_d_n9, assign95300_e147967_d_n10, assign95300_e147967_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95300_e147965: f64 = (1.2 - locals.var_ps0);
        (assign95300_e147965, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95300_e147967;
        locals.var_t9_dn0 = assign95300_e147967_d_n0;
        locals.var_t9_dn2 = assign95300_e147967_d_n2;
        locals.var_t9_dn4 = assign95300_e147967_d_n4;
        locals.var_t9_dn5 = assign95300_e147967_d_n5;
        locals.var_t9_dn6 = assign95300_e147967_d_n6;
        locals.var_t9_dn7 = assign95300_e147967_d_n7;
        locals.var_t9_dn8 = assign95300_e147967_d_n8;
        locals.var_t9_dn9 = assign95300_e147967_d_n9;
        locals.var_t9_dn10 = assign95300_e147967_d_n10;
        locals.var_t9_dn13 = assign95300_e147967_d_n13;

    }

    pub(super) fn stamp_transient_block_340(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95310_e147980, assign95310_e147980_d_n0, assign95310_e147980_d_n2, assign95310_e147980_d_n4, assign95310_e147980_d_n5, assign95310_e147980_d_n6, assign95310_e147980_d_n7, assign95310_e147980_d_n8, assign95310_e147980_d_n9, assign95310_e147980_d_n10, assign95310_e147980_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95310_e147974: f64 = (locals.var_vgs * locals.var_t5);
        let assign95310_e147977: f64 = (locals.var_t4 * locals.var_t9);
        let assign95310_e147978: f64 = (assign95310_e147974 - assign95310_e147977);
        (assign95310_e147978, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), (((locals.var_vgs_dn5 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((locals.var_vgs * locals.var_t5_dn8) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn13) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn13,)
    }
};
        locals.var_qgod = assign95310_e147980;
        locals.var_qgod_dn0 = assign95310_e147980_d_n0;
        locals.var_qgod_dn2 = assign95310_e147980_d_n2;
        locals.var_qgod_dn4 = assign95310_e147980_d_n4;
        locals.var_qgod_dn5 = assign95310_e147980_d_n5;
        locals.var_qgod_dn6 = assign95310_e147980_d_n6;
        locals.var_qgod_dn7 = assign95310_e147980_d_n7;
        locals.var_qgod_dn8 = assign95310_e147980_d_n8;
        locals.var_qgod_dn9 = assign95310_e147980_d_n9;
        locals.var_qgod_dn10 = assign95310_e147980_d_n10;
        locals.var_qgod_dn13 = assign95310_e147980_d_n13;

        let (assign95320_e147987,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95320_e147984: f64 = (-locals.var_weffcv_nf);
        let assign95320_e147985: f64 = (locals.var_uc_cgso * assign95320_e147984);
        (assign95320_e147985,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95320_e147987;

        let assign95330_e147990: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2219 = assign95330_e147990;

        let (assign95340_e148002,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2219 != 0.0)) {
        let assign95340_e147996: f64 = (-locals.var_cox0);
        let assign95340_e147998: f64 = (assign95340_e147996 * p.p66);
        let assign95340_e148000: f64 = (assign95340_e147998 * locals.var_weffcv_nf);
        (assign95340_e148000,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95340_e148002;

        let assign95350_e148004: f64 = (-locals.var_cgsoe);
        let assign95350_e148006: f64 = (assign95350_e148004 * locals.var_vgsei);
        locals.var_qgso = assign95350_e148006;
        locals.var_qgso_dn2 = (assign95350_e148004 * locals.var_vgsei_dn2);
        locals.var_qgso_dn6 = (assign95350_e148004 * locals.var_vgsei_dn6);

        let (assign95360_e148013,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95360_e148010: f64 = (-locals.var_weffcv_nf);
        let assign95360_e148011: f64 = (locals.var_uc_cgdo * assign95360_e148010);
        (assign95360_e148011,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95360_e148013;

        let assign95370_e148016: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2220 = assign95370_e148016;

        let (assign95380_e148028,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95380_e148022: f64 = (-locals.var_coxb0);
        let assign95380_e148024: f64 = (assign95380_e148022 * p.p63);
        let assign95380_e148026: f64 = (assign95380_e148024 * locals.var_weffcv_nf);
        (assign95380_e148026,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95380_e148028;

        let assign95390_e148030: f64 = (-locals.var_cgdoe);
        let assign95390_e148033: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95390_e148034: f64 = (assign95390_e148030 * assign95390_e148033);
        locals.var_qgdo = assign95390_e148034;
        locals.var_qgdo_dn0 = (assign95390_e148030 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95390_e148030 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn6 = (assign95390_e148030 * locals.var_vgsei_dn6);

        let assign95400_e148037: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2221 = assign95400_e148037;

        let (assign95410_e148045, assign95410_e148045_d_n0, assign95410_e148045_d_n2, assign95410_e148045_d_n4, assign95410_e148045_d_n5, assign95410_e148045_d_n6, assign95410_e148045_d_n7, assign95410_e148045_d_n8, assign95410_e148045_d_n9, assign95410_e148045_d_n10, assign95410_e148045_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95410_e148042: f64 = (locals.var_vds - locals.var_pds);
        let assign95410_e148043: f64 = (p.p431 * assign95410_e148042);
        (assign95410_e148043, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn13,)
    }
};
        locals.var_qodad = assign95410_e148045;
        locals.var_qodad_dn0 = assign95410_e148045_d_n0;
        locals.var_qodad_dn2 = assign95410_e148045_d_n2;
        locals.var_qodad_dn4 = assign95410_e148045_d_n4;
        locals.var_qodad_dn5 = assign95410_e148045_d_n5;
        locals.var_qodad_dn6 = assign95410_e148045_d_n6;
        locals.var_qodad_dn7 = assign95410_e148045_d_n7;
        locals.var_qodad_dn8 = assign95410_e148045_d_n8;
        locals.var_qodad_dn9 = assign95410_e148045_d_n9;
        locals.var_qodad_dn10 = assign95410_e148045_d_n10;
        locals.var_qodad_dn13 = assign95410_e148045_d_n13;

        let (assign95420_e148051, assign95420_e148051_d_n0, assign95420_e148051_d_n2, assign95420_e148051_d_n4, assign95420_e148051_d_n5, assign95420_e148051_d_n6, assign95420_e148051_d_n7, assign95420_e148051_d_n8, assign95420_e148051_d_n9, assign95420_e148051_d_n10, assign95420_e148051_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95420_e148049: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95420_e148049, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn13,)
    }
};
        locals.var_qovd_add = assign95420_e148051;
        locals.var_qovd_add_dn0 = assign95420_e148051_d_n0;
        locals.var_qovd_add_dn2 = assign95420_e148051_d_n2;
        locals.var_qovd_add_dn4 = assign95420_e148051_d_n4;
        locals.var_qovd_add_dn5 = assign95420_e148051_d_n5;
        locals.var_qovd_add_dn6 = assign95420_e148051_d_n6;
        locals.var_qovd_add_dn7 = assign95420_e148051_d_n7;
        locals.var_qovd_add_dn8 = assign95420_e148051_d_n8;
        locals.var_qovd_add_dn9 = assign95420_e148051_d_n9;
        locals.var_qovd_add_dn10 = assign95420_e148051_d_n10;
        locals.var_qovd_add_dn13 = assign95420_e148051_d_n13;

        let (assign95430_e148057, assign95430_e148057_d_n0, assign95430_e148057_d_n2, assign95430_e148057_d_n4, assign95430_e148057_d_n5, assign95430_e148057_d_n6, assign95430_e148057_d_n7, assign95430_e148057_d_n8, assign95430_e148057_d_n9, assign95430_e148057_d_n10, assign95430_e148057_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95430_e148055: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95430_e148055, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn13,)
    }
};
        locals.var_qbdld_add = assign95430_e148057;
        locals.var_qbdld_add_dn0 = assign95430_e148057_d_n0;
        locals.var_qbdld_add_dn2 = assign95430_e148057_d_n2;
        locals.var_qbdld_add_dn4 = assign95430_e148057_d_n4;
        locals.var_qbdld_add_dn5 = assign95430_e148057_d_n5;
        locals.var_qbdld_add_dn6 = assign95430_e148057_d_n6;
        locals.var_qbdld_add_dn7 = assign95430_e148057_d_n7;
        locals.var_qbdld_add_dn8 = assign95430_e148057_d_n8;
        locals.var_qbdld_add_dn9 = assign95430_e148057_d_n9;
        locals.var_qbdld_add_dn10 = assign95430_e148057_d_n10;
        locals.var_qbdld_add_dn13 = assign95430_e148057_d_n13;

        let (assign95440_e148067, assign95440_e148067_d_n0, assign95440_e148067_d_n2, assign95440_e148067_d_n4, assign95440_e148067_d_n5, assign95440_e148067_d_n6, assign95440_e148067_d_n7, assign95440_e148067_d_n8, assign95440_e148067_d_n9, assign95440_e148067_d_n10, assign95440_e148067_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95440_e148061: f64 = (-p.p431);
        let assign95440_e148064: f64 = (locals.var_vds - locals.var_pds);
        let assign95440_e148065: f64 = (assign95440_e148061 * assign95440_e148064);
        (assign95440_e148065, (assign95440_e148061 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95440_e148061 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95440_e148061 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95440_e148061 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95440_e148061 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95440_e148061 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95440_e148061 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95440_e148061 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95440_e148061 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95440_e148061 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn13,)
    }
};
        locals.var_qodad = assign95440_e148067;
        locals.var_qodad_dn0 = assign95440_e148067_d_n0;
        locals.var_qodad_dn2 = assign95440_e148067_d_n2;
        locals.var_qodad_dn4 = assign95440_e148067_d_n4;
        locals.var_qodad_dn5 = assign95440_e148067_d_n5;
        locals.var_qodad_dn6 = assign95440_e148067_d_n6;
        locals.var_qodad_dn7 = assign95440_e148067_d_n7;
        locals.var_qodad_dn8 = assign95440_e148067_d_n8;
        locals.var_qodad_dn9 = assign95440_e148067_d_n9;
        locals.var_qodad_dn10 = assign95440_e148067_d_n10;
        locals.var_qodad_dn13 = assign95440_e148067_d_n13;

        let (assign95450_e148074, assign95450_e148074_d_n0, assign95450_e148074_d_n2, assign95450_e148074_d_n4, assign95450_e148074_d_n5, assign95450_e148074_d_n6, assign95450_e148074_d_n7, assign95450_e148074_d_n8, assign95450_e148074_d_n9, assign95450_e148074_d_n10, assign95450_e148074_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95450_e148072: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95450_e148072, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn13,)
    }
};
        locals.var_qovs_add = assign95450_e148074;
        locals.var_qovs_add_dn0 = assign95450_e148074_d_n0;
        locals.var_qovs_add_dn2 = assign95450_e148074_d_n2;
        locals.var_qovs_add_dn4 = assign95450_e148074_d_n4;
        locals.var_qovs_add_dn5 = assign95450_e148074_d_n5;
        locals.var_qovs_add_dn6 = assign95450_e148074_d_n6;
        locals.var_qovs_add_dn7 = assign95450_e148074_d_n7;
        locals.var_qovs_add_dn8 = assign95450_e148074_d_n8;
        locals.var_qovs_add_dn9 = assign95450_e148074_d_n9;
        locals.var_qovs_add_dn10 = assign95450_e148074_d_n10;
        locals.var_qovs_add_dn13 = assign95450_e148074_d_n13;

        let (assign95460_e148081, assign95460_e148081_d_n0, assign95460_e148081_d_n2, assign95460_e148081_d_n4, assign95460_e148081_d_n5, assign95460_e148081_d_n6, assign95460_e148081_d_n7, assign95460_e148081_d_n8, assign95460_e148081_d_n9, assign95460_e148081_d_n10, assign95460_e148081_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95460_e148079: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95460_e148079, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn13,)
    }
};
        locals.var_qbsld_add = assign95460_e148081;
        locals.var_qbsld_add_dn0 = assign95460_e148081_d_n0;
        locals.var_qbsld_add_dn2 = assign95460_e148081_d_n2;
        locals.var_qbsld_add_dn4 = assign95460_e148081_d_n4;
        locals.var_qbsld_add_dn5 = assign95460_e148081_d_n5;
        locals.var_qbsld_add_dn6 = assign95460_e148081_d_n6;
        locals.var_qbsld_add_dn7 = assign95460_e148081_d_n7;
        locals.var_qbsld_add_dn8 = assign95460_e148081_d_n8;
        locals.var_qbsld_add_dn9 = assign95460_e148081_d_n9;
        locals.var_qbsld_add_dn10 = assign95460_e148081_d_n10;
        locals.var_qbsld_add_dn13 = assign95460_e148081_d_n13;

        let assign95470_e148083: f64 = (-locals.var_uc_cgbo);
        let assign95470_e148085: f64 = (assign95470_e148083 * locals.var_lgate);
        locals.var_cgbo_loc = assign95470_e148085;

        let assign95480_e148087: f64 = (-locals.var_cgbo_loc);
        let assign95480_e148090: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95480_e148091: f64 = (assign95480_e148087 * assign95480_e148090);
        locals.var_qgbo = assign95480_e148091;
        locals.var_qgbo_dn6 = (assign95480_e148087 * locals.var_vgsi_dn6);
        locals.var_qgbo_dn7 = (assign95480_e148087 * (locals.var_vgsi_dn7 - locals.var_vbsi_dn7));
        locals.var_qgbo_dn8 = (assign95480_e148087 * (-locals.var_vbsi_dn8));

        locals.var_aclm = locals.var_uc_clm1;

        let assign95500_e148095: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2222 = assign95500_e148095;

        let (assign95510_e148109, assign95510_e148109_d_n0, assign95510_e148109_d_n2, assign95510_e148109_d_n4, assign95510_e148109_d_n5, assign95510_e148109_d_n6, assign95510_e148109_d_n7, assign95510_e148109_d_n8, assign95510_e148109_d_n9, assign95510_e148109_d_n10, assign95510_e148109_d_n13,) = {
    if (locals.var_guard2222 != 0.0) {
        let assign95510_e148100: f64 = (locals.var_vds + locals.var_ps0);
        let assign95510_e148101: f64 = (locals.var_aclm * assign95510_e148100);
        let assign95510_e148104: f64 = (1.0 - locals.var_aclm);
        let assign95510_e148106: f64 = (assign95510_e148104 * locals.var_psl);
        let assign95510_e148107: f64 = (assign95510_e148101 + assign95510_e148106);
        (assign95510_e148107, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95510_e148104 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95510_e148104 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95510_e148104 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95510_e148104 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95510_e148104 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95510_e148104 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95510_e148104 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95510_e148104 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95510_e148104 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn13 + locals.var_ps0_dn13)) + (assign95510_e148104 * locals.var_psl_dn13)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95510_e148109;
        locals.var_psdl_dn0 = assign95510_e148109_d_n0;
        locals.var_psdl_dn2 = assign95510_e148109_d_n2;
        locals.var_psdl_dn4 = assign95510_e148109_d_n4;
        locals.var_psdl_dn5 = assign95510_e148109_d_n5;
        locals.var_psdl_dn6 = assign95510_e148109_d_n6;
        locals.var_psdl_dn7 = assign95510_e148109_d_n7;
        locals.var_psdl_dn8 = assign95510_e148109_d_n8;
        locals.var_psdl_dn9 = assign95510_e148109_d_n9;
        locals.var_psdl_dn10 = assign95510_e148109_d_n10;
        locals.var_psdl_dn13 = assign95510_e148109_d_n13;

        let assign95520_e148113: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95520_e148116: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148117: f64 = (assign95520_e148113 - assign95520_e148116);
        let assign95520_e148120: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148121: f64 = (assign95520_e148117 - assign95520_e148120);
        let assign95520_e148125: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148128: f64 = if ((locals.var_psdl > assign95520_e148121) && (assign95520_e148125 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95520_e148128;

        let (assign95530_e148146, assign95530_e148146_d_n0, assign95530_e148146_d_n2, assign95530_e148146_d_n4, assign95530_e148146_d_n5, assign95530_e148146_d_n6, assign95530_e148146_d_n7, assign95530_e148146_d_n8, assign95530_e148146_d_n9, assign95530_e148146_d_n10, assign95530_e148146_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95530_e148135: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95530_e148138: f64 = (10.0 * 2.220446049250313e-16);
        let assign95530_e148139: f64 = (assign95530_e148135 - assign95530_e148138);
        let assign95530_e148140: f64 = (locals.var_psdl - assign95530_e148139);
        let assign95530_e148143: f64 = (10.0 * 2.220446049250313e-16);
        let assign95530_e148144: f64 = (assign95530_e148140 + assign95530_e148143);
        (assign95530_e148144, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign95530_e148146;
        locals.var_tmf1_dn0 = assign95530_e148146_d_n0;
        locals.var_tmf1_dn2 = assign95530_e148146_d_n2;
        locals.var_tmf1_dn4 = assign95530_e148146_d_n4;
        locals.var_tmf1_dn5 = assign95530_e148146_d_n5;
        locals.var_tmf1_dn6 = assign95530_e148146_d_n6;
        locals.var_tmf1_dn7 = assign95530_e148146_d_n7;
        locals.var_tmf1_dn8 = assign95530_e148146_d_n8;
        locals.var_tmf1_dn9 = assign95530_e148146_d_n9;
        locals.var_tmf1_dn10 = assign95530_e148146_d_n10;
        locals.var_tmf1_dn13 = assign95530_e148146_d_n13;

        let (assign95540_e148154, assign95540_e148154_d_n0, assign95540_e148154_d_n2, assign95540_e148154_d_n4, assign95540_e148154_d_n5, assign95540_e148154_d_n6, assign95540_e148154_d_n7, assign95540_e148154_d_n8, assign95540_e148154_d_n9, assign95540_e148154_d_n10, assign95540_e148154_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95540_e148152: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95540_e148152, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign95540_e148154;
        locals.var_x2_dn0 = assign95540_e148154_d_n0;
        locals.var_x2_dn2 = assign95540_e148154_d_n2;
        locals.var_x2_dn4 = assign95540_e148154_d_n4;
        locals.var_x2_dn5 = assign95540_e148154_d_n5;
        locals.var_x2_dn6 = assign95540_e148154_d_n6;
        locals.var_x2_dn7 = assign95540_e148154_d_n7;
        locals.var_x2_dn8 = assign95540_e148154_d_n8;
        locals.var_x2_dn9 = assign95540_e148154_d_n9;
        locals.var_x2_dn10 = assign95540_e148154_d_n10;
        locals.var_x2_dn13 = assign95540_e148154_d_n13;

        let (assign95550_e148166, assign95550_e148166_d_n0, assign95550_e148166_d_n2, assign95550_e148166_d_n4, assign95550_e148166_d_n5, assign95550_e148166_d_n6, assign95550_e148166_d_n7, assign95550_e148166_d_n8, assign95550_e148166_d_n9, assign95550_e148166_d_n10, assign95550_e148166_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95550_e148160: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148163: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148164: f64 = (assign95550_e148160 * assign95550_e148163);
        (assign95550_e148164, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign95550_e148166;
        locals.var_xmax2_dn0 = assign95550_e148166_d_n0;
        locals.var_xmax2_dn2 = assign95550_e148166_d_n2;
        locals.var_xmax2_dn4 = assign95550_e148166_d_n4;
        locals.var_xmax2_dn5 = assign95550_e148166_d_n5;
        locals.var_xmax2_dn6 = assign95550_e148166_d_n6;
        locals.var_xmax2_dn7 = assign95550_e148166_d_n7;
        locals.var_xmax2_dn8 = assign95550_e148166_d_n8;
        locals.var_xmax2_dn9 = assign95550_e148166_d_n9;
        locals.var_xmax2_dn10 = assign95550_e148166_d_n10;
        locals.var_xmax2_dn13 = assign95550_e148166_d_n13;

        let (assign95560_e148172, assign95560_e148172_d_n0, assign95560_e148172_d_n2, assign95560_e148172_d_n4, assign95560_e148172_d_n5, assign95560_e148172_d_n6, assign95560_e148172_d_n7, assign95560_e148172_d_n8, assign95560_e148172_d_n9, assign95560_e148172_d_n10, assign95560_e148172_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95560_e148172;
        locals.var_xp_dn0 = assign95560_e148172_d_n0;
        locals.var_xp_dn2 = assign95560_e148172_d_n2;
        locals.var_xp_dn4 = assign95560_e148172_d_n4;
        locals.var_xp_dn5 = assign95560_e148172_d_n5;
        locals.var_xp_dn6 = assign95560_e148172_d_n6;
        locals.var_xp_dn7 = assign95560_e148172_d_n7;
        locals.var_xp_dn8 = assign95560_e148172_d_n8;
        locals.var_xp_dn9 = assign95560_e148172_d_n9;
        locals.var_xp_dn10 = assign95560_e148172_d_n10;
        locals.var_xp_dn13 = assign95560_e148172_d_n13;

        let (assign95570_e148178, assign95570_e148178_d_n0, assign95570_e148178_d_n2, assign95570_e148178_d_n4, assign95570_e148178_d_n5, assign95570_e148178_d_n6, assign95570_e148178_d_n7, assign95570_e148178_d_n8, assign95570_e148178_d_n9, assign95570_e148178_d_n10, assign95570_e148178_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95570_e148178;
        locals.var_xmp_dn0 = assign95570_e148178_d_n0;
        locals.var_xmp_dn2 = assign95570_e148178_d_n2;
        locals.var_xmp_dn4 = assign95570_e148178_d_n4;
        locals.var_xmp_dn5 = assign95570_e148178_d_n5;
        locals.var_xmp_dn6 = assign95570_e148178_d_n6;
        locals.var_xmp_dn7 = assign95570_e148178_d_n7;
        locals.var_xmp_dn8 = assign95570_e148178_d_n8;
        locals.var_xmp_dn9 = assign95570_e148178_d_n9;
        locals.var_xmp_dn10 = assign95570_e148178_d_n10;
        locals.var_xmp_dn13 = assign95570_e148178_d_n13;

        let (assign95580_e148184,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95580_e148184;

        let (assign95590_e148190,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95590_e148190;

        let (assign95600_e148196, assign95600_e148196_d_n0, assign95600_e148196_d_n2, assign95600_e148196_d_n4, assign95600_e148196_d_n5, assign95600_e148196_d_n6, assign95600_e148196_d_n7, assign95600_e148196_d_n8, assign95600_e148196_d_n9, assign95600_e148196_d_n10, assign95600_e148196_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign95600_e148196;
        locals.var_arg_dn0 = assign95600_e148196_d_n0;
        locals.var_arg_dn2 = assign95600_e148196_d_n2;
        locals.var_arg_dn4 = assign95600_e148196_d_n4;
        locals.var_arg_dn5 = assign95600_e148196_d_n5;
        locals.var_arg_dn6 = assign95600_e148196_d_n6;
        locals.var_arg_dn7 = assign95600_e148196_d_n7;
        locals.var_arg_dn8 = assign95600_e148196_d_n8;
        locals.var_arg_dn9 = assign95600_e148196_d_n9;
        locals.var_arg_dn10 = assign95600_e148196_d_n10;
        locals.var_arg_dn13 = assign95600_e148196_d_n13;

        let (assign95610_e148202, assign95610_e148202_d_n0, assign95610_e148202_d_n2, assign95610_e148202_d_n4, assign95610_e148202_d_n5, assign95610_e148202_d_n6, assign95610_e148202_d_n7, assign95610_e148202_d_n8, assign95610_e148202_d_n9, assign95610_e148202_d_n10, assign95610_e148202_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95610_e148202;
        locals.var_dnm_dn0 = assign95610_e148202_d_n0;
        locals.var_dnm_dn2 = assign95610_e148202_d_n2;
        locals.var_dnm_dn4 = assign95610_e148202_d_n4;
        locals.var_dnm_dn5 = assign95610_e148202_d_n5;
        locals.var_dnm_dn6 = assign95610_e148202_d_n6;
        locals.var_dnm_dn7 = assign95610_e148202_d_n7;
        locals.var_dnm_dn8 = assign95610_e148202_d_n8;
        locals.var_dnm_dn9 = assign95610_e148202_d_n9;
        locals.var_dnm_dn10 = assign95610_e148202_d_n10;
        locals.var_dnm_dn13 = assign95610_e148202_d_n13;

        let (assign95620_e148210, assign95620_e148210_d_n0, assign95620_e148210_d_n2, assign95620_e148210_d_n4, assign95620_e148210_d_n5, assign95620_e148210_d_n6, assign95620_e148210_d_n7, assign95620_e148210_d_n8, assign95620_e148210_d_n9, assign95620_e148210_d_n10, assign95620_e148210_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95620_e148208: f64 = (locals.var_xp * locals.var_x2);
        (assign95620_e148208, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95620_e148210;
        locals.var_xp_dn0 = assign95620_e148210_d_n0;
        locals.var_xp_dn2 = assign95620_e148210_d_n2;
        locals.var_xp_dn4 = assign95620_e148210_d_n4;
        locals.var_xp_dn5 = assign95620_e148210_d_n5;
        locals.var_xp_dn6 = assign95620_e148210_d_n6;
        locals.var_xp_dn7 = assign95620_e148210_d_n7;
        locals.var_xp_dn8 = assign95620_e148210_d_n8;
        locals.var_xp_dn9 = assign95620_e148210_d_n9;
        locals.var_xp_dn10 = assign95620_e148210_d_n10;
        locals.var_xp_dn13 = assign95620_e148210_d_n13;

        let (assign95630_e148218, assign95630_e148218_d_n0, assign95630_e148218_d_n2, assign95630_e148218_d_n4, assign95630_e148218_d_n5, assign95630_e148218_d_n6, assign95630_e148218_d_n7, assign95630_e148218_d_n8, assign95630_e148218_d_n9, assign95630_e148218_d_n10, assign95630_e148218_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95630_e148216: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95630_e148216, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95630_e148218;
        locals.var_xmp_dn0 = assign95630_e148218_d_n0;
        locals.var_xmp_dn2 = assign95630_e148218_d_n2;
        locals.var_xmp_dn4 = assign95630_e148218_d_n4;
        locals.var_xmp_dn5 = assign95630_e148218_d_n5;
        locals.var_xmp_dn6 = assign95630_e148218_d_n6;
        locals.var_xmp_dn7 = assign95630_e148218_d_n7;
        locals.var_xmp_dn8 = assign95630_e148218_d_n8;
        locals.var_xmp_dn9 = assign95630_e148218_d_n9;
        locals.var_xmp_dn10 = assign95630_e148218_d_n10;
        locals.var_xmp_dn13 = assign95630_e148218_d_n13;

        let (assign95640_e148226, assign95640_e148226_d_n0, assign95640_e148226_d_n2, assign95640_e148226_d_n4, assign95640_e148226_d_n5, assign95640_e148226_d_n6, assign95640_e148226_d_n7, assign95640_e148226_d_n8, assign95640_e148226_d_n9, assign95640_e148226_d_n10, assign95640_e148226_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95640_e148224: f64 = (locals.var_xp * locals.var_x2);
        (assign95640_e148224, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95640_e148226;
        locals.var_xp_dn0 = assign95640_e148226_d_n0;
        locals.var_xp_dn2 = assign95640_e148226_d_n2;
        locals.var_xp_dn4 = assign95640_e148226_d_n4;
        locals.var_xp_dn5 = assign95640_e148226_d_n5;
        locals.var_xp_dn6 = assign95640_e148226_d_n6;
        locals.var_xp_dn7 = assign95640_e148226_d_n7;
        locals.var_xp_dn8 = assign95640_e148226_d_n8;
        locals.var_xp_dn9 = assign95640_e148226_d_n9;
        locals.var_xp_dn10 = assign95640_e148226_d_n10;
        locals.var_xp_dn13 = assign95640_e148226_d_n13;

        let (assign95650_e148234, assign95650_e148234_d_n0, assign95650_e148234_d_n2, assign95650_e148234_d_n4, assign95650_e148234_d_n5, assign95650_e148234_d_n6, assign95650_e148234_d_n7, assign95650_e148234_d_n8, assign95650_e148234_d_n9, assign95650_e148234_d_n10, assign95650_e148234_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95650_e148232: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95650_e148232, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95650_e148234;
        locals.var_xmp_dn0 = assign95650_e148234_d_n0;
        locals.var_xmp_dn2 = assign95650_e148234_d_n2;
        locals.var_xmp_dn4 = assign95650_e148234_d_n4;
        locals.var_xmp_dn5 = assign95650_e148234_d_n5;
        locals.var_xmp_dn6 = assign95650_e148234_d_n6;
        locals.var_xmp_dn7 = assign95650_e148234_d_n7;
        locals.var_xmp_dn8 = assign95650_e148234_d_n8;
        locals.var_xmp_dn9 = assign95650_e148234_d_n9;
        locals.var_xmp_dn10 = assign95650_e148234_d_n10;
        locals.var_xmp_dn13 = assign95650_e148234_d_n13;

    }

    pub(super) fn stamp_transient_block_341(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95660_e148242, assign95660_e148242_d_n0, assign95660_e148242_d_n2, assign95660_e148242_d_n4, assign95660_e148242_d_n5, assign95660_e148242_d_n6, assign95660_e148242_d_n7, assign95660_e148242_d_n8, assign95660_e148242_d_n9, assign95660_e148242_d_n10, assign95660_e148242_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95660_e148240: f64 = (locals.var_xp + locals.var_xmp);
        (assign95660_e148240, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign95660_e148242;
        locals.var_arg_dn0 = assign95660_e148242_d_n0;
        locals.var_arg_dn2 = assign95660_e148242_d_n2;
        locals.var_arg_dn4 = assign95660_e148242_d_n4;
        locals.var_arg_dn5 = assign95660_e148242_d_n5;
        locals.var_arg_dn6 = assign95660_e148242_d_n6;
        locals.var_arg_dn7 = assign95660_e148242_d_n7;
        locals.var_arg_dn8 = assign95660_e148242_d_n8;
        locals.var_arg_dn9 = assign95660_e148242_d_n9;
        locals.var_arg_dn10 = assign95660_e148242_d_n10;
        locals.var_arg_dn13 = assign95660_e148242_d_n13;

        let (assign95670_e148248, assign95670_e148248_d_n0, assign95670_e148248_d_n2, assign95670_e148248_d_n4, assign95670_e148248_d_n5, assign95670_e148248_d_n6, assign95670_e148248_d_n7, assign95670_e148248_d_n8, assign95670_e148248_d_n9, assign95670_e148248_d_n10, assign95670_e148248_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95670_e148248;
        locals.var_dnm_dn0 = assign95670_e148248_d_n0;
        locals.var_dnm_dn2 = assign95670_e148248_d_n2;
        locals.var_dnm_dn4 = assign95670_e148248_d_n4;
        locals.var_dnm_dn5 = assign95670_e148248_d_n5;
        locals.var_dnm_dn6 = assign95670_e148248_d_n6;
        locals.var_dnm_dn7 = assign95670_e148248_d_n7;
        locals.var_dnm_dn8 = assign95670_e148248_d_n8;
        locals.var_dnm_dn9 = assign95670_e148248_d_n9;
        locals.var_dnm_dn10 = assign95670_e148248_d_n10;
        locals.var_dnm_dn13 = assign95670_e148248_d_n13;

        let assign95680_e148263: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95680_e148263;

        let assign95690_e148266: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95690_e148266;

        let (assign95700_e148276,) = {
    if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95700_e148276;

        let assign95710_e148279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95710_e148279;

        let (assign95720_e148292,) = {
    if (((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95720_e148292;

        let assign95730_e148295: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95730_e148295;

        let (assign95740_e148311,) = {
    if ((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148311;

        let assign95750_e148314: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95750_e148314;

        let (assign95760_e148333,) = {
    if (((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148333;

        let (assign95770_e148341,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95770_e148341;

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95780_body0_e148359, assign95780_body0_e148359_d_n0, assign95780_body0_e148359_d_n2, assign95780_body0_e148359_d_n4, assign95780_body0_e148359_d_n5, assign95780_body0_e148359_d_n6, assign95780_body0_e148359_d_n7, assign95780_body0_e148359_d_n8, assign95780_body0_e148359_d_n9, assign95780_body0_e148359_d_n10, assign95780_body0_e148359_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body0_e148357: f64 = (locals.var_dnm).sqrt();
        (assign95780_body0_e148357, (locals.var_dnm_dn0 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn2 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn4 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn5 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn6 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn7 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn8 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn9 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn10 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn13 / (2.0 * assign95780_body0_e148357)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign95780_body0_e148359;
            locals.var_dnm_dn0 = assign95780_body0_e148359_d_n0;
            locals.var_dnm_dn2 = assign95780_body0_e148359_d_n2;
            locals.var_dnm_dn4 = assign95780_body0_e148359_d_n4;
            locals.var_dnm_dn5 = assign95780_body0_e148359_d_n5;
            locals.var_dnm_dn6 = assign95780_body0_e148359_d_n6;
            locals.var_dnm_dn7 = assign95780_body0_e148359_d_n7;
            locals.var_dnm_dn8 = assign95780_body0_e148359_d_n8;
            locals.var_dnm_dn9 = assign95780_body0_e148359_d_n9;
            locals.var_dnm_dn10 = assign95780_body0_e148359_d_n10;
            locals.var_dnm_dn13 = assign95780_body0_e148359_d_n13;
            let (assign95780_body1_e148369,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body1_e148367: f64 = (locals.var_m0 + 1.0);
        (assign95780_body1_e148367,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95780_body1_e148369;
        }

        let (assign95790_e148389, assign95790_e148389_d_n0, assign95790_e148389_d_n2, assign95790_e148389_d_n4, assign95790_e148389_d_n5, assign95790_e148389_d_n6, assign95790_e148389_d_n7, assign95790_e148389_d_n8, assign95790_e148389_d_n9, assign95790_e148389_d_n10, assign95790_e148389_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 == 0.0)) {
        let (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95790_e148384: f64 = (2.0 * 2.0);
                let assign95790_e148385: f64 = (1.0 / assign95790_e148384);
                let assign95790_e148386: f64 = (locals.var_dnm).powf(assign95790_e148385);
                (assign95790_e148386, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn13)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95790_e148389;
        locals.var_dnm_dn0 = assign95790_e148389_d_n0;
        locals.var_dnm_dn2 = assign95790_e148389_d_n2;
        locals.var_dnm_dn4 = assign95790_e148389_d_n4;
        locals.var_dnm_dn5 = assign95790_e148389_d_n5;
        locals.var_dnm_dn6 = assign95790_e148389_d_n6;
        locals.var_dnm_dn7 = assign95790_e148389_d_n7;
        locals.var_dnm_dn8 = assign95790_e148389_d_n8;
        locals.var_dnm_dn9 = assign95790_e148389_d_n9;
        locals.var_dnm_dn10 = assign95790_e148389_d_n10;
        locals.var_dnm_dn13 = assign95790_e148389_d_n13;

        let (assign95800_e148397, assign95800_e148397_d_n0, assign95800_e148397_d_n2, assign95800_e148397_d_n4, assign95800_e148397_d_n5, assign95800_e148397_d_n6, assign95800_e148397_d_n7, assign95800_e148397_d_n8, assign95800_e148397_d_n9, assign95800_e148397_d_n10, assign95800_e148397_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95800_e148395: f64 = (1.0 / locals.var_dnm);
        (assign95800_e148395, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95800_e148397;
        locals.var_dnm_dn0 = assign95800_e148397_d_n0;
        locals.var_dnm_dn2 = assign95800_e148397_d_n2;
        locals.var_dnm_dn4 = assign95800_e148397_d_n4;
        locals.var_dnm_dn5 = assign95800_e148397_d_n5;
        locals.var_dnm_dn6 = assign95800_e148397_d_n6;
        locals.var_dnm_dn7 = assign95800_e148397_d_n7;
        locals.var_dnm_dn8 = assign95800_e148397_d_n8;
        locals.var_dnm_dn9 = assign95800_e148397_d_n9;
        locals.var_dnm_dn10 = assign95800_e148397_d_n10;
        locals.var_dnm_dn13 = assign95800_e148397_d_n13;

        let (assign95810_e148409, assign95810_e148409_d_n0, assign95810_e148409_d_n2, assign95810_e148409_d_n4, assign95810_e148409_d_n5, assign95810_e148409_d_n6, assign95810_e148409_d_n7, assign95810_e148409_d_n8, assign95810_e148409_d_n9, assign95810_e148409_d_n10, assign95810_e148409_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95810_e148404: f64 = (10.0 * 2.220446049250313e-16);
        let assign95810_e148405: f64 = (locals.var_tmf1 * assign95810_e148404);
        let assign95810_e148407: f64 = (assign95810_e148405 * locals.var_dnm);
        (assign95810_e148407, (((locals.var_tmf1_dn0 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign95810_e148409;
        locals.var_tmf0_dn0 = assign95810_e148409_d_n0;
        locals.var_tmf0_dn2 = assign95810_e148409_d_n2;
        locals.var_tmf0_dn4 = assign95810_e148409_d_n4;
        locals.var_tmf0_dn5 = assign95810_e148409_d_n5;
        locals.var_tmf0_dn6 = assign95810_e148409_d_n6;
        locals.var_tmf0_dn7 = assign95810_e148409_d_n7;
        locals.var_tmf0_dn8 = assign95810_e148409_d_n8;
        locals.var_tmf0_dn9 = assign95810_e148409_d_n9;
        locals.var_tmf0_dn10 = assign95810_e148409_d_n10;
        locals.var_tmf0_dn13 = assign95810_e148409_d_n13;

        let (assign95820_e148423, assign95820_e148423_d_n0, assign95820_e148423_d_n2, assign95820_e148423_d_n4, assign95820_e148423_d_n5, assign95820_e148423_d_n6, assign95820_e148423_d_n7, assign95820_e148423_d_n8, assign95820_e148423_d_n9, assign95820_e148423_d_n10, assign95820_e148423_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95820_e148415: f64 = (10.0 * 2.220446049250313e-16);
        let assign95820_e148417: f64 = (assign95820_e148415 * locals.var_xmp);
        let assign95820_e148419: f64 = (assign95820_e148417 * locals.var_dnm);
        let assign95820_e148421: f64 = (assign95820_e148419 / locals.var_arg);
        (assign95820_e148421, ((((((assign95820_e148415 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn13) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn13)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95820_e148423;
        locals.var_t0_dn0 = assign95820_e148423_d_n0;
        locals.var_t0_dn2 = assign95820_e148423_d_n2;
        locals.var_t0_dn4 = assign95820_e148423_d_n4;
        locals.var_t0_dn5 = assign95820_e148423_d_n5;
        locals.var_t0_dn6 = assign95820_e148423_d_n6;
        locals.var_t0_dn7 = assign95820_e148423_d_n7;
        locals.var_t0_dn8 = assign95820_e148423_d_n8;
        locals.var_t0_dn9 = assign95820_e148423_d_n9;
        locals.var_t0_dn10 = assign95820_e148423_d_n10;
        locals.var_t0_dn13 = assign95820_e148423_d_n13;

        let (assign95830_e148441, assign95830_e148441_d_n0, assign95830_e148441_d_n2, assign95830_e148441_d_n4, assign95830_e148441_d_n5, assign95830_e148441_d_n6, assign95830_e148441_d_n7, assign95830_e148441_d_n8, assign95830_e148441_d_n9, assign95830_e148441_d_n10, assign95830_e148441_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95830_e148429: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95830_e148432: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148433: f64 = (assign95830_e148429 - assign95830_e148432);
        let assign95830_e148436: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148437: f64 = (assign95830_e148433 - assign95830_e148436);
        let assign95830_e148439: f64 = (assign95830_e148437 + locals.var_tmf0);
        (assign95830_e148439, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95830_e148441;
        locals.var_psdl_dn0 = assign95830_e148441_d_n0;
        locals.var_psdl_dn2 = assign95830_e148441_d_n2;
        locals.var_psdl_dn4 = assign95830_e148441_d_n4;
        locals.var_psdl_dn5 = assign95830_e148441_d_n5;
        locals.var_psdl_dn6 = assign95830_e148441_d_n6;
        locals.var_psdl_dn7 = assign95830_e148441_d_n7;
        locals.var_psdl_dn8 = assign95830_e148441_d_n8;
        locals.var_psdl_dn9 = assign95830_e148441_d_n9;
        locals.var_psdl_dn10 = assign95830_e148441_d_n10;
        locals.var_psdl_dn13 = assign95830_e148441_d_n13;

        let (assign95840_e148447, assign95840_e148447_d_n0, assign95840_e148447_d_n2, assign95840_e148447_d_n4, assign95840_e148447_d_n5, assign95840_e148447_d_n6, assign95840_e148447_d_n7, assign95840_e148447_d_n8, assign95840_e148447_d_n9, assign95840_e148447_d_n10, assign95840_e148447_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95840_e148447;
        locals.var_t0_dn0 = assign95840_e148447_d_n0;
        locals.var_t0_dn2 = assign95840_e148447_d_n2;
        locals.var_t0_dn4 = assign95840_e148447_d_n4;
        locals.var_t0_dn5 = assign95840_e148447_d_n5;
        locals.var_t0_dn6 = assign95840_e148447_d_n6;
        locals.var_t0_dn7 = assign95840_e148447_d_n7;
        locals.var_t0_dn8 = assign95840_e148447_d_n8;
        locals.var_t0_dn9 = assign95840_e148447_d_n9;
        locals.var_t0_dn10 = assign95840_e148447_d_n10;
        locals.var_t0_dn13 = assign95840_e148447_d_n13;

        let (assign95850_e148454, assign95850_e148454_d_n0, assign95850_e148454_d_n2, assign95850_e148454_d_n4, assign95850_e148454_d_n5, assign95850_e148454_d_n6, assign95850_e148454_d_n7, assign95850_e148454_d_n8, assign95850_e148454_d_n9, assign95850_e148454_d_n10, assign95850_e148454_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95850_e148454;
        locals.var_psdl_dn0 = assign95850_e148454_d_n0;
        locals.var_psdl_dn2 = assign95850_e148454_d_n2;
        locals.var_psdl_dn4 = assign95850_e148454_d_n4;
        locals.var_psdl_dn5 = assign95850_e148454_d_n5;
        locals.var_psdl_dn6 = assign95850_e148454_d_n6;
        locals.var_psdl_dn7 = assign95850_e148454_d_n7;
        locals.var_psdl_dn8 = assign95850_e148454_d_n8;
        locals.var_psdl_dn9 = assign95850_e148454_d_n9;
        locals.var_psdl_dn10 = assign95850_e148454_d_n10;
        locals.var_psdl_dn13 = assign95850_e148454_d_n13;

        let (assign95860_e148461, assign95860_e148461_d_n0, assign95860_e148461_d_n2, assign95860_e148461_d_n4, assign95860_e148461_d_n5, assign95860_e148461_d_n6, assign95860_e148461_d_n7, assign95860_e148461_d_n8, assign95860_e148461_d_n9, assign95860_e148461_d_n10, assign95860_e148461_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95860_e148461;
        locals.var_t0_dn0 = assign95860_e148461_d_n0;
        locals.var_t0_dn2 = assign95860_e148461_d_n2;
        locals.var_t0_dn4 = assign95860_e148461_d_n4;
        locals.var_t0_dn5 = assign95860_e148461_d_n5;
        locals.var_t0_dn6 = assign95860_e148461_d_n6;
        locals.var_t0_dn7 = assign95860_e148461_d_n7;
        locals.var_t0_dn8 = assign95860_e148461_d_n8;
        locals.var_t0_dn9 = assign95860_e148461_d_n9;
        locals.var_t0_dn10 = assign95860_e148461_d_n10;
        locals.var_t0_dn13 = assign95860_e148461_d_n13;

        let (assign95870_e148467, assign95870_e148467_d_n0, assign95870_e148467_d_n2, assign95870_e148467_d_n4, assign95870_e148467_d_n5, assign95870_e148467_d_n6, assign95870_e148467_d_n7, assign95870_e148467_d_n8, assign95870_e148467_d_n9, assign95870_e148467_d_n10, assign95870_e148467_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95870_e148467;
        locals.var_ec_dn0 = assign95870_e148467_d_n0;
        locals.var_ec_dn2 = assign95870_e148467_d_n2;
        locals.var_ec_dn4 = assign95870_e148467_d_n4;
        locals.var_ec_dn5 = assign95870_e148467_d_n5;
        locals.var_ec_dn6 = assign95870_e148467_d_n6;
        locals.var_ec_dn7 = assign95870_e148467_d_n7;
        locals.var_ec_dn8 = assign95870_e148467_d_n8;
        locals.var_ec_dn9 = assign95870_e148467_d_n9;
        locals.var_ec_dn10 = assign95870_e148467_d_n10;
        locals.var_ec_dn13 = assign95870_e148467_d_n13;

        let assign95880_e148474: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95880_e148474;

        let (assign95890_e148483, assign95890_e148483_d_n0, assign95890_e148483_d_n2, assign95890_e148483_d_n4, assign95890_e148483_d_n5, assign95890_e148483_d_n6, assign95890_e148483_d_n7, assign95890_e148483_d_n8, assign95890_e148483_d_n9, assign95890_e148483_d_n10, assign95890_e148483_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95890_e148483;
        locals.var_ec_dn0 = assign95890_e148483_d_n0;
        locals.var_ec_dn2 = assign95890_e148483_d_n2;
        locals.var_ec_dn4 = assign95890_e148483_d_n4;
        locals.var_ec_dn5 = assign95890_e148483_d_n5;
        locals.var_ec_dn6 = assign95890_e148483_d_n6;
        locals.var_ec_dn7 = assign95890_e148483_d_n7;
        locals.var_ec_dn8 = assign95890_e148483_d_n8;
        locals.var_ec_dn9 = assign95890_e148483_d_n9;
        locals.var_ec_dn10 = assign95890_e148483_d_n10;
        locals.var_ec_dn13 = assign95890_e148483_d_n13;

        let (assign95900_e148499, assign95900_e148499_d_n0, assign95900_e148499_d_n2, assign95900_e148499_d_n4, assign95900_e148499_d_n5, assign95900_e148499_d_n6, assign95900_e148499_d_n7, assign95900_e148499_d_n8, assign95900_e148499_d_n9, assign95900_e148499_d_n10, assign95900_e148499_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign95900_e148493: f64 = (locals.var_idd / locals.var_qn0);
        let assign95900_e148495: f64 = (assign95900_e148493 * locals.var_beta_inv);
        let assign95900_e148497: f64 = (assign95900_e148495 / locals.var_leff);
        (assign95900_e148497, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn13 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn13)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn13)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95900_e148499;
        locals.var_ec_dn0 = assign95900_e148499_d_n0;
        locals.var_ec_dn2 = assign95900_e148499_d_n2;
        locals.var_ec_dn4 = assign95900_e148499_d_n4;
        locals.var_ec_dn5 = assign95900_e148499_d_n5;
        locals.var_ec_dn6 = assign95900_e148499_d_n6;
        locals.var_ec_dn7 = assign95900_e148499_d_n7;
        locals.var_ec_dn8 = assign95900_e148499_d_n8;
        locals.var_ec_dn9 = assign95900_e148499_d_n9;
        locals.var_ec_dn10 = assign95900_e148499_d_n10;
        locals.var_ec_dn13 = assign95900_e148499_d_n13;

        let assign95910_e148502: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95910_e148502;

        let (assign95920_e148506, assign95920_e148506_d_n0, assign95920_e148506_d_n2, assign95920_e148506_d_n4, assign95920_e148506_d_n5, assign95920_e148506_d_n6, assign95920_e148506_d_n7, assign95920_e148506_d_n8, assign95920_e148506_d_n9, assign95920_e148506_d_n10, assign95920_e148506_d_n13,) = {
    if (locals.var_guard2230 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95920_e148506;
        locals.var_qy_dn0 = assign95920_e148506_d_n0;
        locals.var_qy_dn2 = assign95920_e148506_d_n2;
        locals.var_qy_dn4 = assign95920_e148506_d_n4;
        locals.var_qy_dn5 = assign95920_e148506_d_n5;
        locals.var_qy_dn6 = assign95920_e148506_d_n6;
        locals.var_qy_dn7 = assign95920_e148506_d_n7;
        locals.var_qy_dn8 = assign95920_e148506_d_n8;
        locals.var_qy_dn9 = assign95920_e148506_d_n9;
        locals.var_qy_dn10 = assign95920_e148506_d_n10;
        locals.var_qy_dn13 = assign95920_e148506_d_n13;

        let (assign95930_e148517, assign95930_e148517_d_n0, assign95930_e148517_d_n2, assign95930_e148517_d_n4, assign95930_e148517_d_n5, assign95930_e148517_d_n6, assign95930_e148517_d_n7, assign95930_e148517_d_n8, assign95930_e148517_d_n9, assign95930_e148517_d_n10, assign95930_e148517_d_n13,) = {
    if (locals.var_guard2230 == 0.0) {
        let assign95930_e148511: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95930_e148513: f64 = (assign95930_e148511 * locals.var_wdpl);
        let assign95930_e148515: f64 = (assign95930_e148513 * 1.3);
        (assign95930_e148515, ((assign95930_e148511 * locals.var_wdpl_dn0) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn2) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn4) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn5) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn6) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn7) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn8) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn9) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn10) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn13) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign95930_e148517;
        locals.var_t2_dn0 = assign95930_e148517_d_n0;
        locals.var_t2_dn2 = assign95930_e148517_d_n2;
        locals.var_t2_dn4 = assign95930_e148517_d_n4;
        locals.var_t2_dn5 = assign95930_e148517_d_n5;
        locals.var_t2_dn6 = assign95930_e148517_d_n6;
        locals.var_t2_dn7 = assign95930_e148517_d_n7;
        locals.var_t2_dn8 = assign95930_e148517_d_n8;
        locals.var_t2_dn9 = assign95930_e148517_d_n9;
        locals.var_t2_dn10 = assign95930_e148517_d_n10;
        locals.var_t2_dn13 = assign95930_e148517_d_n13;

        let assign95940_e148520: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95940_e148520;

        let (assign95950_e148531, assign95950_e148531_d_n0, assign95950_e148531_d_n2, assign95950_e148531_d_n4, assign95950_e148531_d_n5, assign95950_e148531_d_n6, assign95950_e148531_d_n7, assign95950_e148531_d_n8, assign95950_e148531_d_n9, assign95950_e148531_d_n10, assign95950_e148531_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95950_e148527: f64 = (locals.var_ec * locals.var_leff);
        let assign95950_e148529: f64 = (assign95950_e148527 + locals.var_ps0);
        (assign95950_e148529, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn13 * locals.var_leff) + locals.var_ps0_dn13),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn13,)
    }
};
        locals.var_pslk = assign95950_e148531;
        locals.var_pslk_dn0 = assign95950_e148531_d_n0;
        locals.var_pslk_dn2 = assign95950_e148531_d_n2;
        locals.var_pslk_dn4 = assign95950_e148531_d_n4;
        locals.var_pslk_dn5 = assign95950_e148531_d_n5;
        locals.var_pslk_dn6 = assign95950_e148531_d_n6;
        locals.var_pslk_dn7 = assign95950_e148531_d_n7;
        locals.var_pslk_dn8 = assign95950_e148531_d_n8;
        locals.var_pslk_dn9 = assign95950_e148531_d_n9;
        locals.var_pslk_dn10 = assign95950_e148531_d_n10;
        locals.var_pslk_dn13 = assign95950_e148531_d_n13;

        let (assign95960_e148548, assign95960_e148548_d_n0, assign95960_e148548_d_n2, assign95960_e148548_d_n4, assign95960_e148548_d_n5, assign95960_e148548_d_n6, assign95960_e148548_d_n7, assign95960_e148548_d_n8, assign95960_e148548_d_n9, assign95960_e148548_d_n10, assign95960_e148548_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95960_e148539: f64 = (locals.var_vdsz__blk439 + locals.var_ps0);
        let assign95960_e148540: f64 = (locals.var_aclm * assign95960_e148539);
        let assign95960_e148543: f64 = (1.0 - locals.var_aclm);
        let assign95960_e148545: f64 = (assign95960_e148543 * locals.var_pslk);
        let assign95960_e148546: f64 = (assign95960_e148540 + assign95960_e148545);
        (assign95960_e148546, ((locals.var_aclm * (locals.var_vdsz__blk439_dn0 + locals.var_ps0_dn0)) + (assign95960_e148543 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn2 + locals.var_ps0_dn2)) + (assign95960_e148543 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn4 + locals.var_ps0_dn4)) + (assign95960_e148543 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn5 + locals.var_ps0_dn5)) + (assign95960_e148543 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn6 + locals.var_ps0_dn6)) + (assign95960_e148543 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn7 + locals.var_ps0_dn7)) + (assign95960_e148543 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn8 + locals.var_ps0_dn8)) + (assign95960_e148543 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn9 + locals.var_ps0_dn9)) + (assign95960_e148543 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn10 + locals.var_ps0_dn10)) + (assign95960_e148543 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn13 + locals.var_ps0_dn13)) + (assign95960_e148543 * locals.var_pslk_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95960_e148548;
        locals.var_t1_dn0 = assign95960_e148548_d_n0;
        locals.var_t1_dn2 = assign95960_e148548_d_n2;
        locals.var_t1_dn4 = assign95960_e148548_d_n4;
        locals.var_t1_dn5 = assign95960_e148548_d_n5;
        locals.var_t1_dn6 = assign95960_e148548_d_n6;
        locals.var_t1_dn7 = assign95960_e148548_d_n7;
        locals.var_t1_dn8 = assign95960_e148548_d_n8;
        locals.var_t1_dn9 = assign95960_e148548_d_n9;
        locals.var_t1_dn10 = assign95960_e148548_d_n10;
        locals.var_t1_dn13 = assign95960_e148548_d_n13;

        let (assign95970_e148564, assign95970_e148564_d_n0, assign95970_e148564_d_n2, assign95970_e148564_d_n4, assign95970_e148564_d_n5, assign95970_e148564_d_n6, assign95970_e148564_d_n7, assign95970_e148564_d_n8, assign95970_e148564_d_n9, assign95970_e148564_d_n10, assign95970_e148564_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95970_e148555: f64 = (locals.var_ps0 + locals.var_vdsz__blk439);
        let assign95970_e148557: f64 = (assign95970_e148555 - locals.var_t1);
        let assign95970_e148559: f64 = (assign95970_e148557 / p.p133);
        let assign95970_e148560: f64 = (-assign95970_e148559);
        let assign95970_e148562: f64 = (assign95970_e148560 * locals.var_t2);
        (assign95970_e148562, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk439_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk439_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk439_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk439_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk439_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk439_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk439_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk439_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk439_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn13 + locals.var_vdsz__blk439_dn13) - locals.var_t1_dn13) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn13)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95970_e148564;
        locals.var_qy_dn0 = assign95970_e148564_d_n0;
        locals.var_qy_dn2 = assign95970_e148564_d_n2;
        locals.var_qy_dn4 = assign95970_e148564_d_n4;
        locals.var_qy_dn5 = assign95970_e148564_d_n5;
        locals.var_qy_dn6 = assign95970_e148564_d_n6;
        locals.var_qy_dn7 = assign95970_e148564_d_n7;
        locals.var_qy_dn8 = assign95970_e148564_d_n8;
        locals.var_qy_dn9 = assign95970_e148564_d_n9;
        locals.var_qy_dn10 = assign95970_e148564_d_n10;
        locals.var_qy_dn13 = assign95970_e148564_d_n13;

        let assign95980_e148567: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95980_e148567;

    }

    pub(super) fn stamp_transient_block_342(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95990_e148578, assign95990_e148578_d_n0, assign95990_e148578_d_n2, assign95990_e148578_d_n4, assign95990_e148578_d_n5, assign95990_e148578_d_n6, assign95990_e148578_d_n7, assign95990_e148578_d_n8, assign95990_e148578_d_n9, assign95990_e148578_d_n10, assign95990_e148578_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2232 != 0.0)) {
        let assign95990_e148575: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign95990_e148576: f64 = (locals.var_qy + assign95990_e148575);
        (assign95990_e148576, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, (locals.var_qy_dn5 + (locals.var_cqyb0 * locals.var_vbs_dn5)), locals.var_qy_dn6, (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbs_dn7)), (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95990_e148578;
        locals.var_qy_dn0 = assign95990_e148578_d_n0;
        locals.var_qy_dn2 = assign95990_e148578_d_n2;
        locals.var_qy_dn4 = assign95990_e148578_d_n4;
        locals.var_qy_dn5 = assign95990_e148578_d_n5;
        locals.var_qy_dn6 = assign95990_e148578_d_n6;
        locals.var_qy_dn7 = assign95990_e148578_d_n7;
        locals.var_qy_dn8 = assign95990_e148578_d_n8;
        locals.var_qy_dn9 = assign95990_e148578_d_n9;
        locals.var_qy_dn10 = assign95990_e148578_d_n10;
        locals.var_qy_dn13 = assign95990_e148578_d_n13;

        locals.var_cfd = locals.var_cfrng;

        locals.var_cfs = locals.var_cfrng;

        let assign96020_e148584: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96020_e148585: f64 = (locals.var_cfd * assign96020_e148584);
        locals.var_qfd = assign96020_e148585;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn6 = (locals.var_cfd * locals.var_vgsei_dn6);

        let assign96030_e148588: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96030_e148588;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn6 = (locals.var_cfs * locals.var_vgsei_dn6);

        let assign96040_e148595: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign96040_e148595;

        let (assign96050_e148601, assign96050_e148601_d_n0, assign96050_e148601_d_n2, assign96050_e148601_d_n4, assign96050_e148601_d_n5, assign96050_e148601_d_n6, assign96050_e148601_d_n7, assign96050_e148601_d_n8, assign96050_e148601_d_n9, assign96050_e148601_d_n10, assign96050_e148601_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96050_e148599: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96050_e148599, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn13 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign96050_e148601;
        locals.var_t0_dn0 = assign96050_e148601_d_n0;
        locals.var_t0_dn2 = assign96050_e148601_d_n2;
        locals.var_t0_dn4 = assign96050_e148601_d_n4;
        locals.var_t0_dn5 = assign96050_e148601_d_n5;
        locals.var_t0_dn6 = assign96050_e148601_d_n6;
        locals.var_t0_dn7 = assign96050_e148601_d_n7;
        locals.var_t0_dn8 = assign96050_e148601_d_n8;
        locals.var_t0_dn9 = assign96050_e148601_d_n9;
        locals.var_t0_dn10 = assign96050_e148601_d_n10;
        locals.var_t0_dn13 = assign96050_e148601_d_n13;

        let (assign96060_e148620, assign96060_e148620_d_n0, assign96060_e148620_d_n2, assign96060_e148620_d_n4, assign96060_e148620_d_n5, assign96060_e148620_d_n6, assign96060_e148620_d_n7, assign96060_e148620_d_n8, assign96060_e148620_d_n9, assign96060_e148620_d_n10, assign96060_e148620_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96060_e148606: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96060_e148609: f64 = (locals.var_eg * locals.var_beta);
        let assign96060_e148610: f64 = (assign96060_e148606 - assign96060_e148609);
        let assign96060_e148613: f64 = (p.p499 * locals.var_log_tratio);
        let assign96060_e148614: f64 = (assign96060_e148610 + assign96060_e148613);
        let assign96060_e148616: f64 = (assign96060_e148614 / locals.var_uc_njd);
        let assign96060_e148617: f64 = (assign96060_e148616).exp();
        let assign96060_e148618: f64 = (locals.var_uc_js0d * assign96060_e148617);
        (assign96060_e148618, (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign96060_e148620;
        locals.var_js_dn0 = assign96060_e148620_d_n0;
        locals.var_js_dn2 = assign96060_e148620_d_n2;
        locals.var_js_dn4 = assign96060_e148620_d_n4;
        locals.var_js_dn5 = assign96060_e148620_d_n5;
        locals.var_js_dn6 = assign96060_e148620_d_n6;
        locals.var_js_dn7 = assign96060_e148620_d_n7;
        locals.var_js_dn8 = assign96060_e148620_d_n8;
        locals.var_js_dn9 = assign96060_e148620_d_n9;
        locals.var_js_dn10 = assign96060_e148620_d_n10;
        locals.var_js_dn13 = assign96060_e148620_d_n13;

        let (assign96070_e148639, assign96070_e148639_d_n0, assign96070_e148639_d_n2, assign96070_e148639_d_n4, assign96070_e148639_d_n5, assign96070_e148639_d_n6, assign96070_e148639_d_n7, assign96070_e148639_d_n8, assign96070_e148639_d_n9, assign96070_e148639_d_n10, assign96070_e148639_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96070_e148625: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96070_e148628: f64 = (locals.var_eg * locals.var_beta);
        let assign96070_e148629: f64 = (assign96070_e148625 - assign96070_e148628);
        let assign96070_e148632: f64 = (p.p499 * locals.var_log_tratio);
        let assign96070_e148633: f64 = (assign96070_e148629 + assign96070_e148632);
        let assign96070_e148635: f64 = (assign96070_e148633 / p.p497);
        let assign96070_e148636: f64 = (assign96070_e148635).exp();
        let assign96070_e148637: f64 = (locals.var_uc_js0swd * assign96070_e148636);
        (assign96070_e148637, (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign96070_e148639;
        locals.var_jssw_dn0 = assign96070_e148639_d_n0;
        locals.var_jssw_dn2 = assign96070_e148639_d_n2;
        locals.var_jssw_dn4 = assign96070_e148639_d_n4;
        locals.var_jssw_dn5 = assign96070_e148639_d_n5;
        locals.var_jssw_dn6 = assign96070_e148639_d_n6;
        locals.var_jssw_dn7 = assign96070_e148639_d_n7;
        locals.var_jssw_dn8 = assign96070_e148639_d_n8;
        locals.var_jssw_dn9 = assign96070_e148639_d_n9;
        locals.var_jssw_dn10 = assign96070_e148639_d_n10;
        locals.var_jssw_dn13 = assign96070_e148639_d_n13;

        let (assign96080_e148658, assign96080_e148658_d_n0, assign96080_e148658_d_n2, assign96080_e148658_d_n4, assign96080_e148658_d_n5, assign96080_e148658_d_n6, assign96080_e148658_d_n7, assign96080_e148658_d_n8, assign96080_e148658_d_n9, assign96080_e148658_d_n10, assign96080_e148658_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96080_e148644: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96080_e148647: f64 = (locals.var_eg * locals.var_beta);
        let assign96080_e148648: f64 = (assign96080_e148644 - assign96080_e148647);
        let assign96080_e148651: f64 = (p.p499 * locals.var_log_tratio);
        let assign96080_e148652: f64 = (assign96080_e148648 + assign96080_e148651);
        let assign96080_e148654: f64 = (assign96080_e148652 / p.p498);
        let assign96080_e148655: f64 = (assign96080_e148654).exp();
        let assign96080_e148656: f64 = (p.p495 * assign96080_e148655);
        (assign96080_e148656, (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign96080_e148658;
        locals.var_jsswg_dn0 = assign96080_e148658_d_n0;
        locals.var_jsswg_dn2 = assign96080_e148658_d_n2;
        locals.var_jsswg_dn4 = assign96080_e148658_d_n4;
        locals.var_jsswg_dn5 = assign96080_e148658_d_n5;
        locals.var_jsswg_dn6 = assign96080_e148658_d_n6;
        locals.var_jsswg_dn7 = assign96080_e148658_d_n7;
        locals.var_jsswg_dn8 = assign96080_e148658_d_n8;
        locals.var_jsswg_dn9 = assign96080_e148658_d_n9;
        locals.var_jsswg_dn10 = assign96080_e148658_d_n10;
        locals.var_jsswg_dn13 = assign96080_e148658_d_n13;

        let (assign96090_e148677, assign96090_e148677_d_n0, assign96090_e148677_d_n2, assign96090_e148677_d_n4, assign96090_e148677_d_n5, assign96090_e148677_d_n6, assign96090_e148677_d_n7, assign96090_e148677_d_n8, assign96090_e148677_d_n9, assign96090_e148677_d_n10, assign96090_e148677_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96090_e148663: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96090_e148666: f64 = (locals.var_eg * locals.var_beta);
        let assign96090_e148667: f64 = (assign96090_e148663 - assign96090_e148666);
        let assign96090_e148670: f64 = (p.p509 * locals.var_log_tratio);
        let assign96090_e148671: f64 = (assign96090_e148667 + assign96090_e148670);
        let assign96090_e148673: f64 = (assign96090_e148671 / locals.var_uc_njd);
        let assign96090_e148674: f64 = (assign96090_e148673).exp();
        let assign96090_e148675: f64 = (locals.var_uc_js0d * assign96090_e148674);
        (assign96090_e148675, (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign96090_e148677;
        locals.var_js2_dn0 = assign96090_e148677_d_n0;
        locals.var_js2_dn2 = assign96090_e148677_d_n2;
        locals.var_js2_dn4 = assign96090_e148677_d_n4;
        locals.var_js2_dn5 = assign96090_e148677_d_n5;
        locals.var_js2_dn6 = assign96090_e148677_d_n6;
        locals.var_js2_dn7 = assign96090_e148677_d_n7;
        locals.var_js2_dn8 = assign96090_e148677_d_n8;
        locals.var_js2_dn9 = assign96090_e148677_d_n9;
        locals.var_js2_dn10 = assign96090_e148677_d_n10;
        locals.var_js2_dn13 = assign96090_e148677_d_n13;

        let (assign96100_e148696, assign96100_e148696_d_n0, assign96100_e148696_d_n2, assign96100_e148696_d_n4, assign96100_e148696_d_n5, assign96100_e148696_d_n6, assign96100_e148696_d_n7, assign96100_e148696_d_n8, assign96100_e148696_d_n9, assign96100_e148696_d_n10, assign96100_e148696_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96100_e148682: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148685: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148686: f64 = (assign96100_e148682 - assign96100_e148685);
        let assign96100_e148689: f64 = (p.p509 * locals.var_log_tratio);
        let assign96100_e148690: f64 = (assign96100_e148686 + assign96100_e148689);
        let assign96100_e148692: f64 = (assign96100_e148690 / p.p497);
        let assign96100_e148693: f64 = (assign96100_e148692).exp();
        let assign96100_e148694: f64 = (locals.var_uc_js0swd * assign96100_e148693);
        (assign96100_e148694, (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign96100_e148696;
        locals.var_jssw2_dn0 = assign96100_e148696_d_n0;
        locals.var_jssw2_dn2 = assign96100_e148696_d_n2;
        locals.var_jssw2_dn4 = assign96100_e148696_d_n4;
        locals.var_jssw2_dn5 = assign96100_e148696_d_n5;
        locals.var_jssw2_dn6 = assign96100_e148696_d_n6;
        locals.var_jssw2_dn7 = assign96100_e148696_d_n7;
        locals.var_jssw2_dn8 = assign96100_e148696_d_n8;
        locals.var_jssw2_dn9 = assign96100_e148696_d_n9;
        locals.var_jssw2_dn10 = assign96100_e148696_d_n10;
        locals.var_jssw2_dn13 = assign96100_e148696_d_n13;

        let (assign96110_e148715, assign96110_e148715_d_n0, assign96110_e148715_d_n2, assign96110_e148715_d_n4, assign96110_e148715_d_n5, assign96110_e148715_d_n6, assign96110_e148715_d_n7, assign96110_e148715_d_n8, assign96110_e148715_d_n9, assign96110_e148715_d_n10, assign96110_e148715_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96110_e148701: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148704: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148705: f64 = (assign96110_e148701 - assign96110_e148704);
        let assign96110_e148708: f64 = (p.p509 * locals.var_log_tratio);
        let assign96110_e148709: f64 = (assign96110_e148705 + assign96110_e148708);
        let assign96110_e148711: f64 = (assign96110_e148709 / p.p498);
        let assign96110_e148712: f64 = (assign96110_e148711).exp();
        let assign96110_e148713: f64 = (p.p495 * assign96110_e148712);
        (assign96110_e148713, (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign96110_e148715;
        locals.var_jsswg2_dn0 = assign96110_e148715_d_n0;
        locals.var_jsswg2_dn2 = assign96110_e148715_d_n2;
        locals.var_jsswg2_dn4 = assign96110_e148715_d_n4;
        locals.var_jsswg2_dn5 = assign96110_e148715_d_n5;
        locals.var_jsswg2_dn6 = assign96110_e148715_d_n6;
        locals.var_jsswg2_dn7 = assign96110_e148715_d_n7;
        locals.var_jsswg2_dn8 = assign96110_e148715_d_n8;
        locals.var_jsswg2_dn9 = assign96110_e148715_d_n9;
        locals.var_jsswg2_dn10 = assign96110_e148715_d_n10;
        locals.var_jsswg2_dn13 = assign96110_e148715_d_n13;

        let assign96120_e148718: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign96120_e148718;

        let assign96130_e148721: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign96130_e148721;

        let (assign96140_e148731, assign96140_e148731_d_n0, assign96140_e148731_d_n2, assign96140_e148731_d_n4, assign96140_e148731_d_n5, assign96140_e148731_d_n6, assign96140_e148731_d_n7, assign96140_e148731_d_n8, assign96140_e148731_d_n9, assign96140_e148731_d_n10, assign96140_e148731_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96140_e148729: f64 = (p.p13 * locals.var_js);
        (assign96140_e148729, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96140_e148731;
        locals.var_isbd_btm_dn0 = assign96140_e148731_d_n0;
        locals.var_isbd_btm_dn2 = assign96140_e148731_d_n2;
        locals.var_isbd_btm_dn4 = assign96140_e148731_d_n4;
        locals.var_isbd_btm_dn5 = assign96140_e148731_d_n5;
        locals.var_isbd_btm_dn6 = assign96140_e148731_d_n6;
        locals.var_isbd_btm_dn7 = assign96140_e148731_d_n7;
        locals.var_isbd_btm_dn8 = assign96140_e148731_d_n8;
        locals.var_isbd_btm_dn9 = assign96140_e148731_d_n9;
        locals.var_isbd_btm_dn10 = assign96140_e148731_d_n10;
        locals.var_isbd_btm_dn13 = assign96140_e148731_d_n13;

        let (assign96150_e148741, assign96150_e148741_d_n0, assign96150_e148741_d_n2, assign96150_e148741_d_n4, assign96150_e148741_d_n5, assign96150_e148741_d_n6, assign96150_e148741_d_n7, assign96150_e148741_d_n8, assign96150_e148741_d_n9, assign96150_e148741_d_n10, assign96150_e148741_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96150_e148739: f64 = (p.p13 * locals.var_js2);
        (assign96150_e148739, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96150_e148741;
        locals.var_isbd2_btm_dn0 = assign96150_e148741_d_n0;
        locals.var_isbd2_btm_dn2 = assign96150_e148741_d_n2;
        locals.var_isbd2_btm_dn4 = assign96150_e148741_d_n4;
        locals.var_isbd2_btm_dn5 = assign96150_e148741_d_n5;
        locals.var_isbd2_btm_dn6 = assign96150_e148741_d_n6;
        locals.var_isbd2_btm_dn7 = assign96150_e148741_d_n7;
        locals.var_isbd2_btm_dn8 = assign96150_e148741_d_n8;
        locals.var_isbd2_btm_dn9 = assign96150_e148741_d_n9;
        locals.var_isbd2_btm_dn10 = assign96150_e148741_d_n10;
        locals.var_isbd2_btm_dn13 = assign96150_e148741_d_n13;

        let (assign96160_e148753, assign96160_e148753_d_n0, assign96160_e148753_d_n2, assign96160_e148753_d_n4, assign96160_e148753_d_n5, assign96160_e148753_d_n6, assign96160_e148753_d_n7, assign96160_e148753_d_n8, assign96160_e148753_d_n9, assign96160_e148753_d_n10, assign96160_e148753_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96160_e148749: f64 = (p.p15 - locals.var_weff_nf);
        let assign96160_e148751: f64 = (assign96160_e148749 * locals.var_jssw);
        (assign96160_e148751, (assign96160_e148749 * locals.var_jssw_dn0), (assign96160_e148749 * locals.var_jssw_dn2), (assign96160_e148749 * locals.var_jssw_dn4), (assign96160_e148749 * locals.var_jssw_dn5), (assign96160_e148749 * locals.var_jssw_dn6), (assign96160_e148749 * locals.var_jssw_dn7), (assign96160_e148749 * locals.var_jssw_dn8), (assign96160_e148749 * locals.var_jssw_dn9), (assign96160_e148749 * locals.var_jssw_dn10), (assign96160_e148749 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96160_e148753;
        locals.var_isbd_sws_dn0 = assign96160_e148753_d_n0;
        locals.var_isbd_sws_dn2 = assign96160_e148753_d_n2;
        locals.var_isbd_sws_dn4 = assign96160_e148753_d_n4;
        locals.var_isbd_sws_dn5 = assign96160_e148753_d_n5;
        locals.var_isbd_sws_dn6 = assign96160_e148753_d_n6;
        locals.var_isbd_sws_dn7 = assign96160_e148753_d_n7;
        locals.var_isbd_sws_dn8 = assign96160_e148753_d_n8;
        locals.var_isbd_sws_dn9 = assign96160_e148753_d_n9;
        locals.var_isbd_sws_dn10 = assign96160_e148753_d_n10;
        locals.var_isbd_sws_dn13 = assign96160_e148753_d_n13;

        let (assign96170_e148765, assign96170_e148765_d_n0, assign96170_e148765_d_n2, assign96170_e148765_d_n4, assign96170_e148765_d_n5, assign96170_e148765_d_n6, assign96170_e148765_d_n7, assign96170_e148765_d_n8, assign96170_e148765_d_n9, assign96170_e148765_d_n10, assign96170_e148765_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96170_e148761: f64 = (p.p15 - locals.var_weff_nf);
        let assign96170_e148763: f64 = (assign96170_e148761 * locals.var_jssw2);
        (assign96170_e148763, (assign96170_e148761 * locals.var_jssw2_dn0), (assign96170_e148761 * locals.var_jssw2_dn2), (assign96170_e148761 * locals.var_jssw2_dn4), (assign96170_e148761 * locals.var_jssw2_dn5), (assign96170_e148761 * locals.var_jssw2_dn6), (assign96170_e148761 * locals.var_jssw2_dn7), (assign96170_e148761 * locals.var_jssw2_dn8), (assign96170_e148761 * locals.var_jssw2_dn9), (assign96170_e148761 * locals.var_jssw2_dn10), (assign96170_e148761 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96170_e148765;
        locals.var_isbd2_sws_dn0 = assign96170_e148765_d_n0;
        locals.var_isbd2_sws_dn2 = assign96170_e148765_d_n2;
        locals.var_isbd2_sws_dn4 = assign96170_e148765_d_n4;
        locals.var_isbd2_sws_dn5 = assign96170_e148765_d_n5;
        locals.var_isbd2_sws_dn6 = assign96170_e148765_d_n6;
        locals.var_isbd2_sws_dn7 = assign96170_e148765_d_n7;
        locals.var_isbd2_sws_dn8 = assign96170_e148765_d_n8;
        locals.var_isbd2_sws_dn9 = assign96170_e148765_d_n9;
        locals.var_isbd2_sws_dn10 = assign96170_e148765_d_n10;
        locals.var_isbd2_sws_dn13 = assign96170_e148765_d_n13;

        let (assign96180_e148775, assign96180_e148775_d_n0, assign96180_e148775_d_n2, assign96180_e148775_d_n4, assign96180_e148775_d_n5, assign96180_e148775_d_n6, assign96180_e148775_d_n7, assign96180_e148775_d_n8, assign96180_e148775_d_n9, assign96180_e148775_d_n10, assign96180_e148775_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96180_e148773: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96180_e148773, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96180_e148775;
        locals.var_isbd_swg_dn0 = assign96180_e148775_d_n0;
        locals.var_isbd_swg_dn2 = assign96180_e148775_d_n2;
        locals.var_isbd_swg_dn4 = assign96180_e148775_d_n4;
        locals.var_isbd_swg_dn5 = assign96180_e148775_d_n5;
        locals.var_isbd_swg_dn6 = assign96180_e148775_d_n6;
        locals.var_isbd_swg_dn7 = assign96180_e148775_d_n7;
        locals.var_isbd_swg_dn8 = assign96180_e148775_d_n8;
        locals.var_isbd_swg_dn9 = assign96180_e148775_d_n9;
        locals.var_isbd_swg_dn10 = assign96180_e148775_d_n10;
        locals.var_isbd_swg_dn13 = assign96180_e148775_d_n13;

        let (assign96190_e148785, assign96190_e148785_d_n0, assign96190_e148785_d_n2, assign96190_e148785_d_n4, assign96190_e148785_d_n5, assign96190_e148785_d_n6, assign96190_e148785_d_n7, assign96190_e148785_d_n8, assign96190_e148785_d_n9, assign96190_e148785_d_n10, assign96190_e148785_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96190_e148783: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96190_e148783, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96190_e148785;
        locals.var_isbd2_swg_dn0 = assign96190_e148785_d_n0;
        locals.var_isbd2_swg_dn2 = assign96190_e148785_d_n2;
        locals.var_isbd2_swg_dn4 = assign96190_e148785_d_n4;
        locals.var_isbd2_swg_dn5 = assign96190_e148785_d_n5;
        locals.var_isbd2_swg_dn6 = assign96190_e148785_d_n6;
        locals.var_isbd2_swg_dn7 = assign96190_e148785_d_n7;
        locals.var_isbd2_swg_dn8 = assign96190_e148785_d_n8;
        locals.var_isbd2_swg_dn9 = assign96190_e148785_d_n9;
        locals.var_isbd2_swg_dn10 = assign96190_e148785_d_n10;
        locals.var_isbd2_swg_dn13 = assign96190_e148785_d_n13;

        let (assign96200_e148796, assign96200_e148796_d_n0, assign96200_e148796_d_n2, assign96200_e148796_d_n4, assign96200_e148796_d_n5, assign96200_e148796_d_n6, assign96200_e148796_d_n7, assign96200_e148796_d_n8, assign96200_e148796_d_n9, assign96200_e148796_d_n10, assign96200_e148796_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96200_e148794: f64 = (p.p13 * locals.var_js);
        (assign96200_e148794, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96200_e148796;
        locals.var_isbd_btm_dn0 = assign96200_e148796_d_n0;
        locals.var_isbd_btm_dn2 = assign96200_e148796_d_n2;
        locals.var_isbd_btm_dn4 = assign96200_e148796_d_n4;
        locals.var_isbd_btm_dn5 = assign96200_e148796_d_n5;
        locals.var_isbd_btm_dn6 = assign96200_e148796_d_n6;
        locals.var_isbd_btm_dn7 = assign96200_e148796_d_n7;
        locals.var_isbd_btm_dn8 = assign96200_e148796_d_n8;
        locals.var_isbd_btm_dn9 = assign96200_e148796_d_n9;
        locals.var_isbd_btm_dn10 = assign96200_e148796_d_n10;
        locals.var_isbd_btm_dn13 = assign96200_e148796_d_n13;

        let (assign96210_e148807, assign96210_e148807_d_n0, assign96210_e148807_d_n2, assign96210_e148807_d_n4, assign96210_e148807_d_n5, assign96210_e148807_d_n6, assign96210_e148807_d_n7, assign96210_e148807_d_n8, assign96210_e148807_d_n9, assign96210_e148807_d_n10, assign96210_e148807_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96210_e148805: f64 = (p.p13 * locals.var_js2);
        (assign96210_e148805, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96210_e148807;
        locals.var_isbd2_btm_dn0 = assign96210_e148807_d_n0;
        locals.var_isbd2_btm_dn2 = assign96210_e148807_d_n2;
        locals.var_isbd2_btm_dn4 = assign96210_e148807_d_n4;
        locals.var_isbd2_btm_dn5 = assign96210_e148807_d_n5;
        locals.var_isbd2_btm_dn6 = assign96210_e148807_d_n6;
        locals.var_isbd2_btm_dn7 = assign96210_e148807_d_n7;
        locals.var_isbd2_btm_dn8 = assign96210_e148807_d_n8;
        locals.var_isbd2_btm_dn9 = assign96210_e148807_d_n9;
        locals.var_isbd2_btm_dn10 = assign96210_e148807_d_n10;
        locals.var_isbd2_btm_dn13 = assign96210_e148807_d_n13;

        let (assign96220_e148816, assign96220_e148816_d_n0, assign96220_e148816_d_n2, assign96220_e148816_d_n4, assign96220_e148816_d_n5, assign96220_e148816_d_n6, assign96220_e148816_d_n7, assign96220_e148816_d_n8, assign96220_e148816_d_n9, assign96220_e148816_d_n10, assign96220_e148816_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96220_e148816;
        locals.var_isbd_sws_dn0 = assign96220_e148816_d_n0;
        locals.var_isbd_sws_dn2 = assign96220_e148816_d_n2;
        locals.var_isbd_sws_dn4 = assign96220_e148816_d_n4;
        locals.var_isbd_sws_dn5 = assign96220_e148816_d_n5;
        locals.var_isbd_sws_dn6 = assign96220_e148816_d_n6;
        locals.var_isbd_sws_dn7 = assign96220_e148816_d_n7;
        locals.var_isbd_sws_dn8 = assign96220_e148816_d_n8;
        locals.var_isbd_sws_dn9 = assign96220_e148816_d_n9;
        locals.var_isbd_sws_dn10 = assign96220_e148816_d_n10;
        locals.var_isbd_sws_dn13 = assign96220_e148816_d_n13;

        let (assign96230_e148825, assign96230_e148825_d_n0, assign96230_e148825_d_n2, assign96230_e148825_d_n4, assign96230_e148825_d_n5, assign96230_e148825_d_n6, assign96230_e148825_d_n7, assign96230_e148825_d_n8, assign96230_e148825_d_n9, assign96230_e148825_d_n10, assign96230_e148825_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96230_e148825;
        locals.var_isbd2_sws_dn0 = assign96230_e148825_d_n0;
        locals.var_isbd2_sws_dn2 = assign96230_e148825_d_n2;
        locals.var_isbd2_sws_dn4 = assign96230_e148825_d_n4;
        locals.var_isbd2_sws_dn5 = assign96230_e148825_d_n5;
        locals.var_isbd2_sws_dn6 = assign96230_e148825_d_n6;
        locals.var_isbd2_sws_dn7 = assign96230_e148825_d_n7;
        locals.var_isbd2_sws_dn8 = assign96230_e148825_d_n8;
        locals.var_isbd2_sws_dn9 = assign96230_e148825_d_n9;
        locals.var_isbd2_sws_dn10 = assign96230_e148825_d_n10;
        locals.var_isbd2_sws_dn13 = assign96230_e148825_d_n13;

        let (assign96240_e148836, assign96240_e148836_d_n0, assign96240_e148836_d_n2, assign96240_e148836_d_n4, assign96240_e148836_d_n5, assign96240_e148836_d_n6, assign96240_e148836_d_n7, assign96240_e148836_d_n8, assign96240_e148836_d_n9, assign96240_e148836_d_n10, assign96240_e148836_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96240_e148834: f64 = (p.p15 * locals.var_jsswg);
        (assign96240_e148834, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96240_e148836;
        locals.var_isbd_swg_dn0 = assign96240_e148836_d_n0;
        locals.var_isbd_swg_dn2 = assign96240_e148836_d_n2;
        locals.var_isbd_swg_dn4 = assign96240_e148836_d_n4;
        locals.var_isbd_swg_dn5 = assign96240_e148836_d_n5;
        locals.var_isbd_swg_dn6 = assign96240_e148836_d_n6;
        locals.var_isbd_swg_dn7 = assign96240_e148836_d_n7;
        locals.var_isbd_swg_dn8 = assign96240_e148836_d_n8;
        locals.var_isbd_swg_dn9 = assign96240_e148836_d_n9;
        locals.var_isbd_swg_dn10 = assign96240_e148836_d_n10;
        locals.var_isbd_swg_dn13 = assign96240_e148836_d_n13;

        let (assign96250_e148847, assign96250_e148847_d_n0, assign96250_e148847_d_n2, assign96250_e148847_d_n4, assign96250_e148847_d_n5, assign96250_e148847_d_n6, assign96250_e148847_d_n7, assign96250_e148847_d_n8, assign96250_e148847_d_n9, assign96250_e148847_d_n10, assign96250_e148847_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96250_e148845: f64 = (p.p15 * locals.var_jsswg2);
        (assign96250_e148845, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96250_e148847;
        locals.var_isbd2_swg_dn0 = assign96250_e148847_d_n0;
        locals.var_isbd2_swg_dn2 = assign96250_e148847_d_n2;
        locals.var_isbd2_swg_dn4 = assign96250_e148847_d_n4;
        locals.var_isbd2_swg_dn5 = assign96250_e148847_d_n5;
        locals.var_isbd2_swg_dn6 = assign96250_e148847_d_n6;
        locals.var_isbd2_swg_dn7 = assign96250_e148847_d_n7;
        locals.var_isbd2_swg_dn8 = assign96250_e148847_d_n8;
        locals.var_isbd2_swg_dn9 = assign96250_e148847_d_n9;
        locals.var_isbd2_swg_dn10 = assign96250_e148847_d_n10;
        locals.var_isbd2_swg_dn13 = assign96250_e148847_d_n13;

        let (assign96260_e148856, assign96260_e148856_d_n0, assign96260_e148856_d_n2, assign96260_e148856_d_n4, assign96260_e148856_d_n5, assign96260_e148856_d_n6, assign96260_e148856_d_n7, assign96260_e148856_d_n8, assign96260_e148856_d_n9, assign96260_e148856_d_n10, assign96260_e148856_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96260_e148854: f64 = (p.p13 * locals.var_js);
        (assign96260_e148854, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96260_e148856;
        locals.var_isbd_btm_dn0 = assign96260_e148856_d_n0;
        locals.var_isbd_btm_dn2 = assign96260_e148856_d_n2;
        locals.var_isbd_btm_dn4 = assign96260_e148856_d_n4;
        locals.var_isbd_btm_dn5 = assign96260_e148856_d_n5;
        locals.var_isbd_btm_dn6 = assign96260_e148856_d_n6;
        locals.var_isbd_btm_dn7 = assign96260_e148856_d_n7;
        locals.var_isbd_btm_dn8 = assign96260_e148856_d_n8;
        locals.var_isbd_btm_dn9 = assign96260_e148856_d_n9;
        locals.var_isbd_btm_dn10 = assign96260_e148856_d_n10;
        locals.var_isbd_btm_dn13 = assign96260_e148856_d_n13;

        let (assign96270_e148865, assign96270_e148865_d_n0, assign96270_e148865_d_n2, assign96270_e148865_d_n4, assign96270_e148865_d_n5, assign96270_e148865_d_n6, assign96270_e148865_d_n7, assign96270_e148865_d_n8, assign96270_e148865_d_n9, assign96270_e148865_d_n10, assign96270_e148865_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96270_e148863: f64 = (p.p13 * locals.var_js2);
        (assign96270_e148863, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96270_e148865;
        locals.var_isbd2_btm_dn0 = assign96270_e148865_d_n0;
        locals.var_isbd2_btm_dn2 = assign96270_e148865_d_n2;
        locals.var_isbd2_btm_dn4 = assign96270_e148865_d_n4;
        locals.var_isbd2_btm_dn5 = assign96270_e148865_d_n5;
        locals.var_isbd2_btm_dn6 = assign96270_e148865_d_n6;
        locals.var_isbd2_btm_dn7 = assign96270_e148865_d_n7;
        locals.var_isbd2_btm_dn8 = assign96270_e148865_d_n8;
        locals.var_isbd2_btm_dn9 = assign96270_e148865_d_n9;
        locals.var_isbd2_btm_dn10 = assign96270_e148865_d_n10;
        locals.var_isbd2_btm_dn13 = assign96270_e148865_d_n13;

    }

    pub(super) fn stamp_transient_block_343(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96280_e148874, assign96280_e148874_d_n0, assign96280_e148874_d_n2, assign96280_e148874_d_n4, assign96280_e148874_d_n5, assign96280_e148874_d_n6, assign96280_e148874_d_n7, assign96280_e148874_d_n8, assign96280_e148874_d_n9, assign96280_e148874_d_n10, assign96280_e148874_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96280_e148872: f64 = (p.p15 * locals.var_jssw);
        (assign96280_e148872, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96280_e148874;
        locals.var_isbd_sws_dn0 = assign96280_e148874_d_n0;
        locals.var_isbd_sws_dn2 = assign96280_e148874_d_n2;
        locals.var_isbd_sws_dn4 = assign96280_e148874_d_n4;
        locals.var_isbd_sws_dn5 = assign96280_e148874_d_n5;
        locals.var_isbd_sws_dn6 = assign96280_e148874_d_n6;
        locals.var_isbd_sws_dn7 = assign96280_e148874_d_n7;
        locals.var_isbd_sws_dn8 = assign96280_e148874_d_n8;
        locals.var_isbd_sws_dn9 = assign96280_e148874_d_n9;
        locals.var_isbd_sws_dn10 = assign96280_e148874_d_n10;
        locals.var_isbd_sws_dn13 = assign96280_e148874_d_n13;

        let (assign96290_e148883, assign96290_e148883_d_n0, assign96290_e148883_d_n2, assign96290_e148883_d_n4, assign96290_e148883_d_n5, assign96290_e148883_d_n6, assign96290_e148883_d_n7, assign96290_e148883_d_n8, assign96290_e148883_d_n9, assign96290_e148883_d_n10, assign96290_e148883_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96290_e148881: f64 = (p.p15 * locals.var_jssw2);
        (assign96290_e148881, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96290_e148883;
        locals.var_isbd2_sws_dn0 = assign96290_e148883_d_n0;
        locals.var_isbd2_sws_dn2 = assign96290_e148883_d_n2;
        locals.var_isbd2_sws_dn4 = assign96290_e148883_d_n4;
        locals.var_isbd2_sws_dn5 = assign96290_e148883_d_n5;
        locals.var_isbd2_sws_dn6 = assign96290_e148883_d_n6;
        locals.var_isbd2_sws_dn7 = assign96290_e148883_d_n7;
        locals.var_isbd2_sws_dn8 = assign96290_e148883_d_n8;
        locals.var_isbd2_sws_dn9 = assign96290_e148883_d_n9;
        locals.var_isbd2_sws_dn10 = assign96290_e148883_d_n10;
        locals.var_isbd2_sws_dn13 = assign96290_e148883_d_n13;

        let (assign96300_e148890, assign96300_e148890_d_n0, assign96300_e148890_d_n2, assign96300_e148890_d_n4, assign96300_e148890_d_n5, assign96300_e148890_d_n6, assign96300_e148890_d_n7, assign96300_e148890_d_n8, assign96300_e148890_d_n9, assign96300_e148890_d_n10, assign96300_e148890_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96300_e148890;
        locals.var_isbd_swg_dn0 = assign96300_e148890_d_n0;
        locals.var_isbd_swg_dn2 = assign96300_e148890_d_n2;
        locals.var_isbd_swg_dn4 = assign96300_e148890_d_n4;
        locals.var_isbd_swg_dn5 = assign96300_e148890_d_n5;
        locals.var_isbd_swg_dn6 = assign96300_e148890_d_n6;
        locals.var_isbd_swg_dn7 = assign96300_e148890_d_n7;
        locals.var_isbd_swg_dn8 = assign96300_e148890_d_n8;
        locals.var_isbd_swg_dn9 = assign96300_e148890_d_n9;
        locals.var_isbd_swg_dn10 = assign96300_e148890_d_n10;
        locals.var_isbd_swg_dn13 = assign96300_e148890_d_n13;

        let (assign96310_e148897, assign96310_e148897_d_n0, assign96310_e148897_d_n2, assign96310_e148897_d_n4, assign96310_e148897_d_n5, assign96310_e148897_d_n6, assign96310_e148897_d_n7, assign96310_e148897_d_n8, assign96310_e148897_d_n9, assign96310_e148897_d_n10, assign96310_e148897_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96310_e148897;
        locals.var_isbd2_swg_dn0 = assign96310_e148897_d_n0;
        locals.var_isbd2_swg_dn2 = assign96310_e148897_d_n2;
        locals.var_isbd2_swg_dn4 = assign96310_e148897_d_n4;
        locals.var_isbd2_swg_dn5 = assign96310_e148897_d_n5;
        locals.var_isbd2_swg_dn6 = assign96310_e148897_d_n6;
        locals.var_isbd2_swg_dn7 = assign96310_e148897_d_n7;
        locals.var_isbd2_swg_dn8 = assign96310_e148897_d_n8;
        locals.var_isbd2_swg_dn9 = assign96310_e148897_d_n9;
        locals.var_isbd2_swg_dn10 = assign96310_e148897_d_n10;
        locals.var_isbd2_swg_dn13 = assign96310_e148897_d_n13;

        let (assign96320_e148905, assign96320_e148905_d_n0, assign96320_e148905_d_n2, assign96320_e148905_d_n4, assign96320_e148905_d_n5, assign96320_e148905_d_n6, assign96320_e148905_d_n7, assign96320_e148905_d_n8, assign96320_e148905_d_n9, assign96320_e148905_d_n10, assign96320_e148905_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96320_e148901: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96320_e148903: f64 = (assign96320_e148901 + locals.var_isbd_swg);
        (assign96320_e148903, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn13 + locals.var_isbd_sws_dn13) + locals.var_isbd_swg_dn13),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    }
};
        locals.var_isbd = assign96320_e148905;
        locals.var_isbd_dn0 = assign96320_e148905_d_n0;
        locals.var_isbd_dn2 = assign96320_e148905_d_n2;
        locals.var_isbd_dn4 = assign96320_e148905_d_n4;
        locals.var_isbd_dn5 = assign96320_e148905_d_n5;
        locals.var_isbd_dn6 = assign96320_e148905_d_n6;
        locals.var_isbd_dn7 = assign96320_e148905_d_n7;
        locals.var_isbd_dn8 = assign96320_e148905_d_n8;
        locals.var_isbd_dn9 = assign96320_e148905_d_n9;
        locals.var_isbd_dn10 = assign96320_e148905_d_n10;
        locals.var_isbd_dn13 = assign96320_e148905_d_n13;

        let assign96330_e148908: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96330_e148908;

        let (assign96340_e148916, assign96340_e148916_d_n0, assign96340_e148916_d_n2, assign96340_e148916_d_n4, assign96340_e148916_d_n5, assign96340_e148916_d_n6, assign96340_e148916_d_n7, assign96340_e148916_d_n8, assign96340_e148916_d_n9, assign96340_e148916_d_n10, assign96340_e148916_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96340_e148914: f64 = (locals.var_isbd + 1e-25);
        (assign96340_e148914, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign96340_e148916;
        locals.var_t2_dn0 = assign96340_e148916_d_n0;
        locals.var_t2_dn2 = assign96340_e148916_d_n2;
        locals.var_t2_dn4 = assign96340_e148916_d_n4;
        locals.var_t2_dn5 = assign96340_e148916_d_n5;
        locals.var_t2_dn6 = assign96340_e148916_d_n6;
        locals.var_t2_dn7 = assign96340_e148916_d_n7;
        locals.var_t2_dn8 = assign96340_e148916_d_n8;
        locals.var_t2_dn9 = assign96340_e148916_d_n9;
        locals.var_t2_dn10 = assign96340_e148916_d_n10;
        locals.var_t2_dn13 = assign96340_e148916_d_n13;

        let (assign96350_e148933, assign96350_e148933_d_n0, assign96350_e148933_d_n2, assign96350_e148933_d_n4, assign96350_e148933_d_n5, assign96350_e148933_d_n6, assign96350_e148933_d_n7, assign96350_e148933_d_n8, assign96350_e148933_d_n9, assign96350_e148933_d_n10, assign96350_e148933_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96350_e148922: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96350_e148925: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96350_e148927: f64 = (assign96350_e148925 / locals.var_t2);
        let assign96350_e148929: f64 = (assign96350_e148927 + 1.0);
        let assign96350_e148930: f64 = (assign96350_e148929).ln();
        let assign96350_e148931: f64 = (assign96350_e148922 * assign96350_e148930);
        (assign96350_e148931, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn13) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn13,)
    }
};
        locals.var_vbdt = assign96350_e148933;
        locals.var_vbdt_dn0 = assign96350_e148933_d_n0;
        locals.var_vbdt_dn2 = assign96350_e148933_d_n2;
        locals.var_vbdt_dn4 = assign96350_e148933_d_n4;
        locals.var_vbdt_dn5 = assign96350_e148933_d_n5;
        locals.var_vbdt_dn6 = assign96350_e148933_d_n6;
        locals.var_vbdt_dn7 = assign96350_e148933_d_n7;
        locals.var_vbdt_dn8 = assign96350_e148933_d_n8;
        locals.var_vbdt_dn9 = assign96350_e148933_d_n9;
        locals.var_vbdt_dn10 = assign96350_e148933_d_n10;
        locals.var_vbdt_dn13 = assign96350_e148933_d_n13;

        let (assign96360_e148944, assign96360_e148944_d_n0, assign96360_e148944_d_n2, assign96360_e148944_d_n4, assign96360_e148944_d_n5, assign96360_e148944_d_n6, assign96360_e148944_d_n7, assign96360_e148944_d_n8, assign96360_e148944_d_n9, assign96360_e148944_d_n10, assign96360_e148944_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96360_e148939: f64 = (locals.var_tratio - 1.0);
        let assign96360_e148941: f64 = (assign96360_e148939 * p.p512);
        let assign96360_e148942: f64 = (assign96360_e148941).exp();
        (assign96360_e148942, (assign96360_e148942 * (locals.var_tratio_dn0 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn2 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn4 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn5 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn6 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn7 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn8 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn9 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn10 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn13 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn13,)
    }
};
        locals.var_exptempd = assign96360_e148944;
        locals.var_exptempd_dn0 = assign96360_e148944_d_n0;
        locals.var_exptempd_dn2 = assign96360_e148944_d_n2;
        locals.var_exptempd_dn4 = assign96360_e148944_d_n4;
        locals.var_exptempd_dn5 = assign96360_e148944_d_n5;
        locals.var_exptempd_dn6 = assign96360_e148944_d_n6;
        locals.var_exptempd_dn7 = assign96360_e148944_d_n7;
        locals.var_exptempd_dn8 = assign96360_e148944_d_n8;
        locals.var_exptempd_dn9 = assign96360_e148944_d_n9;
        locals.var_exptempd_dn10 = assign96360_e148944_d_n10;
        locals.var_exptempd_dn13 = assign96360_e148944_d_n13;

        let (assign96370_e148954, assign96370_e148954_d_n0, assign96370_e148954_d_n2, assign96370_e148954_d_n4, assign96370_e148954_d_n5, assign96370_e148954_d_n6, assign96370_e148954_d_n7, assign96370_e148954_d_n8, assign96370_e148954_d_n9, assign96370_e148954_d_n10, assign96370_e148954_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96370_e148951: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96370_e148952: f64 = (1.0 / assign96370_e148951);
        (assign96370_e148952, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn13,)
    }
};
        locals.var_jd_nvtm_invd = assign96370_e148954;
        locals.var_jd_nvtm_invd_dn0 = assign96370_e148954_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96370_e148954_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96370_e148954_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96370_e148954_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96370_e148954_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96370_e148954_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96370_e148954_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96370_e148954_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96370_e148954_d_n10;
        locals.var_jd_nvtm_invd_dn13 = assign96370_e148954_d_n13;

        let (assign96380_e148963, assign96380_e148963_d_n0, assign96380_e148963_d_n2, assign96380_e148963_d_n4, assign96380_e148963_d_n5, assign96380_e148963_d_n6, assign96380_e148963_d_n7, assign96380_e148963_d_n8, assign96380_e148963_d_n9, assign96380_e148963_d_n10, assign96380_e148963_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96380_e148960: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96380_e148961: f64 = (assign96380_e148960).exp();
        (assign96380_e148961, (assign96380_e148961 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96380_e148961 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96380_e148961 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96380_e148961 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96380_e148961 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96380_e148961 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96380_e148961 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96380_e148961 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96380_e148961 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96380_e148961 * ((locals.var_vbdt_dn13 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn13))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    }
};
        locals.var_jd_expcd = assign96380_e148963;
        locals.var_jd_expcd_dn0 = assign96380_e148963_d_n0;
        locals.var_jd_expcd_dn2 = assign96380_e148963_d_n2;
        locals.var_jd_expcd_dn4 = assign96380_e148963_d_n4;
        locals.var_jd_expcd_dn5 = assign96380_e148963_d_n5;
        locals.var_jd_expcd_dn6 = assign96380_e148963_d_n6;
        locals.var_jd_expcd_dn7 = assign96380_e148963_d_n7;
        locals.var_jd_expcd_dn8 = assign96380_e148963_d_n8;
        locals.var_jd_expcd_dn9 = assign96380_e148963_d_n9;
        locals.var_jd_expcd_dn10 = assign96380_e148963_d_n10;
        locals.var_jd_expcd_dn13 = assign96380_e148963_d_n13;

        let (assign96390_e148982, assign96390_e148982_d_n0, assign96390_e148982_d_n2, assign96390_e148982_d_n4, assign96390_e148982_d_n5, assign96390_e148982_d_n6, assign96390_e148982_d_n7, assign96390_e148982_d_n8, assign96390_e148982_d_n9, assign96390_e148982_d_n10, assign96390_e148982_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96390_e148968: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96390_e148971: f64 = (locals.var_eg * locals.var_beta);
        let assign96390_e148972: f64 = (assign96390_e148968 - assign96390_e148971);
        let assign96390_e148975: f64 = (p.p522 * locals.var_log_tratio);
        let assign96390_e148976: f64 = (assign96390_e148972 + assign96390_e148975);
        let assign96390_e148978: f64 = (assign96390_e148976 / locals.var_uc_njs);
        let assign96390_e148979: f64 = (assign96390_e148978).exp();
        let assign96390_e148980: f64 = (locals.var_uc_js0s * assign96390_e148979);
        (assign96390_e148980, (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign96390_e148982;
        locals.var_js_dn0 = assign96390_e148982_d_n0;
        locals.var_js_dn2 = assign96390_e148982_d_n2;
        locals.var_js_dn4 = assign96390_e148982_d_n4;
        locals.var_js_dn5 = assign96390_e148982_d_n5;
        locals.var_js_dn6 = assign96390_e148982_d_n6;
        locals.var_js_dn7 = assign96390_e148982_d_n7;
        locals.var_js_dn8 = assign96390_e148982_d_n8;
        locals.var_js_dn9 = assign96390_e148982_d_n9;
        locals.var_js_dn10 = assign96390_e148982_d_n10;
        locals.var_js_dn13 = assign96390_e148982_d_n13;

        let (assign96400_e149001, assign96400_e149001_d_n0, assign96400_e149001_d_n2, assign96400_e149001_d_n4, assign96400_e149001_d_n5, assign96400_e149001_d_n6, assign96400_e149001_d_n7, assign96400_e149001_d_n8, assign96400_e149001_d_n9, assign96400_e149001_d_n10, assign96400_e149001_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96400_e148987: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96400_e148990: f64 = (locals.var_eg * locals.var_beta);
        let assign96400_e148991: f64 = (assign96400_e148987 - assign96400_e148990);
        let assign96400_e148994: f64 = (p.p522 * locals.var_log_tratio);
        let assign96400_e148995: f64 = (assign96400_e148991 + assign96400_e148994);
        let assign96400_e148997: f64 = (assign96400_e148995 / p.p520);
        let assign96400_e148998: f64 = (assign96400_e148997).exp();
        let assign96400_e148999: f64 = (locals.var_uc_js0sws * assign96400_e148998);
        (assign96400_e148999, (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign96400_e149001;
        locals.var_jssw_dn0 = assign96400_e149001_d_n0;
        locals.var_jssw_dn2 = assign96400_e149001_d_n2;
        locals.var_jssw_dn4 = assign96400_e149001_d_n4;
        locals.var_jssw_dn5 = assign96400_e149001_d_n5;
        locals.var_jssw_dn6 = assign96400_e149001_d_n6;
        locals.var_jssw_dn7 = assign96400_e149001_d_n7;
        locals.var_jssw_dn8 = assign96400_e149001_d_n8;
        locals.var_jssw_dn9 = assign96400_e149001_d_n9;
        locals.var_jssw_dn10 = assign96400_e149001_d_n10;
        locals.var_jssw_dn13 = assign96400_e149001_d_n13;

        let (assign96410_e149020, assign96410_e149020_d_n0, assign96410_e149020_d_n2, assign96410_e149020_d_n4, assign96410_e149020_d_n5, assign96410_e149020_d_n6, assign96410_e149020_d_n7, assign96410_e149020_d_n8, assign96410_e149020_d_n9, assign96410_e149020_d_n10, assign96410_e149020_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96410_e149006: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96410_e149009: f64 = (locals.var_eg * locals.var_beta);
        let assign96410_e149010: f64 = (assign96410_e149006 - assign96410_e149009);
        let assign96410_e149013: f64 = (p.p522 * locals.var_log_tratio);
        let assign96410_e149014: f64 = (assign96410_e149010 + assign96410_e149013);
        let assign96410_e149016: f64 = (assign96410_e149014 / p.p521);
        let assign96410_e149017: f64 = (assign96410_e149016).exp();
        let assign96410_e149018: f64 = (p.p518 * assign96410_e149017);
        (assign96410_e149018, (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign96410_e149020;
        locals.var_jsswg_dn0 = assign96410_e149020_d_n0;
        locals.var_jsswg_dn2 = assign96410_e149020_d_n2;
        locals.var_jsswg_dn4 = assign96410_e149020_d_n4;
        locals.var_jsswg_dn5 = assign96410_e149020_d_n5;
        locals.var_jsswg_dn6 = assign96410_e149020_d_n6;
        locals.var_jsswg_dn7 = assign96410_e149020_d_n7;
        locals.var_jsswg_dn8 = assign96410_e149020_d_n8;
        locals.var_jsswg_dn9 = assign96410_e149020_d_n9;
        locals.var_jsswg_dn10 = assign96410_e149020_d_n10;
        locals.var_jsswg_dn13 = assign96410_e149020_d_n13;

        let (assign96420_e149039, assign96420_e149039_d_n0, assign96420_e149039_d_n2, assign96420_e149039_d_n4, assign96420_e149039_d_n5, assign96420_e149039_d_n6, assign96420_e149039_d_n7, assign96420_e149039_d_n8, assign96420_e149039_d_n9, assign96420_e149039_d_n10, assign96420_e149039_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96420_e149025: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96420_e149028: f64 = (locals.var_eg * locals.var_beta);
        let assign96420_e149029: f64 = (assign96420_e149025 - assign96420_e149028);
        let assign96420_e149032: f64 = (p.p532 * locals.var_log_tratio);
        let assign96420_e149033: f64 = (assign96420_e149029 + assign96420_e149032);
        let assign96420_e149035: f64 = (assign96420_e149033 / locals.var_uc_njs);
        let assign96420_e149036: f64 = (assign96420_e149035).exp();
        let assign96420_e149037: f64 = (locals.var_uc_js0s * assign96420_e149036);
        (assign96420_e149037, (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign96420_e149039;
        locals.var_js2_dn0 = assign96420_e149039_d_n0;
        locals.var_js2_dn2 = assign96420_e149039_d_n2;
        locals.var_js2_dn4 = assign96420_e149039_d_n4;
        locals.var_js2_dn5 = assign96420_e149039_d_n5;
        locals.var_js2_dn6 = assign96420_e149039_d_n6;
        locals.var_js2_dn7 = assign96420_e149039_d_n7;
        locals.var_js2_dn8 = assign96420_e149039_d_n8;
        locals.var_js2_dn9 = assign96420_e149039_d_n9;
        locals.var_js2_dn10 = assign96420_e149039_d_n10;
        locals.var_js2_dn13 = assign96420_e149039_d_n13;

        let (assign96430_e149058, assign96430_e149058_d_n0, assign96430_e149058_d_n2, assign96430_e149058_d_n4, assign96430_e149058_d_n5, assign96430_e149058_d_n6, assign96430_e149058_d_n7, assign96430_e149058_d_n8, assign96430_e149058_d_n9, assign96430_e149058_d_n10, assign96430_e149058_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96430_e149044: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e149047: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e149048: f64 = (assign96430_e149044 - assign96430_e149047);
        let assign96430_e149051: f64 = (p.p532 * locals.var_log_tratio);
        let assign96430_e149052: f64 = (assign96430_e149048 + assign96430_e149051);
        let assign96430_e149054: f64 = (assign96430_e149052 / p.p520);
        let assign96430_e149055: f64 = (assign96430_e149054).exp();
        let assign96430_e149056: f64 = (locals.var_uc_js0sws * assign96430_e149055);
        (assign96430_e149056, (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign96430_e149058;
        locals.var_jssw2_dn0 = assign96430_e149058_d_n0;
        locals.var_jssw2_dn2 = assign96430_e149058_d_n2;
        locals.var_jssw2_dn4 = assign96430_e149058_d_n4;
        locals.var_jssw2_dn5 = assign96430_e149058_d_n5;
        locals.var_jssw2_dn6 = assign96430_e149058_d_n6;
        locals.var_jssw2_dn7 = assign96430_e149058_d_n7;
        locals.var_jssw2_dn8 = assign96430_e149058_d_n8;
        locals.var_jssw2_dn9 = assign96430_e149058_d_n9;
        locals.var_jssw2_dn10 = assign96430_e149058_d_n10;
        locals.var_jssw2_dn13 = assign96430_e149058_d_n13;

        let (assign96440_e149077, assign96440_e149077_d_n0, assign96440_e149077_d_n2, assign96440_e149077_d_n4, assign96440_e149077_d_n5, assign96440_e149077_d_n6, assign96440_e149077_d_n7, assign96440_e149077_d_n8, assign96440_e149077_d_n9, assign96440_e149077_d_n10, assign96440_e149077_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96440_e149063: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149066: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149067: f64 = (assign96440_e149063 - assign96440_e149066);
        let assign96440_e149070: f64 = (p.p532 * locals.var_log_tratio);
        let assign96440_e149071: f64 = (assign96440_e149067 + assign96440_e149070);
        let assign96440_e149073: f64 = (assign96440_e149071 / p.p521);
        let assign96440_e149074: f64 = (assign96440_e149073).exp();
        let assign96440_e149075: f64 = (p.p518 * assign96440_e149074);
        (assign96440_e149075, (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign96440_e149077;
        locals.var_jsswg2_dn0 = assign96440_e149077_d_n0;
        locals.var_jsswg2_dn2 = assign96440_e149077_d_n2;
        locals.var_jsswg2_dn4 = assign96440_e149077_d_n4;
        locals.var_jsswg2_dn5 = assign96440_e149077_d_n5;
        locals.var_jsswg2_dn6 = assign96440_e149077_d_n6;
        locals.var_jsswg2_dn7 = assign96440_e149077_d_n7;
        locals.var_jsswg2_dn8 = assign96440_e149077_d_n8;
        locals.var_jsswg2_dn9 = assign96440_e149077_d_n9;
        locals.var_jsswg2_dn10 = assign96440_e149077_d_n10;
        locals.var_jsswg2_dn13 = assign96440_e149077_d_n13;

        let assign96450_e149080: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96450_e149080;

        let assign96460_e149083: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96460_e149083;

        let (assign96470_e149093, assign96470_e149093_d_n0, assign96470_e149093_d_n2, assign96470_e149093_d_n4, assign96470_e149093_d_n5, assign96470_e149093_d_n6, assign96470_e149093_d_n7, assign96470_e149093_d_n8, assign96470_e149093_d_n9, assign96470_e149093_d_n10, assign96470_e149093_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96470_e149091: f64 = (p.p14 * locals.var_js);
        (assign96470_e149091, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96470_e149093;
        locals.var_isbs_btm_dn0 = assign96470_e149093_d_n0;
        locals.var_isbs_btm_dn2 = assign96470_e149093_d_n2;
        locals.var_isbs_btm_dn4 = assign96470_e149093_d_n4;
        locals.var_isbs_btm_dn5 = assign96470_e149093_d_n5;
        locals.var_isbs_btm_dn6 = assign96470_e149093_d_n6;
        locals.var_isbs_btm_dn7 = assign96470_e149093_d_n7;
        locals.var_isbs_btm_dn8 = assign96470_e149093_d_n8;
        locals.var_isbs_btm_dn9 = assign96470_e149093_d_n9;
        locals.var_isbs_btm_dn10 = assign96470_e149093_d_n10;
        locals.var_isbs_btm_dn13 = assign96470_e149093_d_n13;

        let (assign96480_e149103, assign96480_e149103_d_n0, assign96480_e149103_d_n2, assign96480_e149103_d_n4, assign96480_e149103_d_n5, assign96480_e149103_d_n6, assign96480_e149103_d_n7, assign96480_e149103_d_n8, assign96480_e149103_d_n9, assign96480_e149103_d_n10, assign96480_e149103_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96480_e149101: f64 = (p.p14 * locals.var_js2);
        (assign96480_e149101, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96480_e149103;
        locals.var_isbs2_btm_dn0 = assign96480_e149103_d_n0;
        locals.var_isbs2_btm_dn2 = assign96480_e149103_d_n2;
        locals.var_isbs2_btm_dn4 = assign96480_e149103_d_n4;
        locals.var_isbs2_btm_dn5 = assign96480_e149103_d_n5;
        locals.var_isbs2_btm_dn6 = assign96480_e149103_d_n6;
        locals.var_isbs2_btm_dn7 = assign96480_e149103_d_n7;
        locals.var_isbs2_btm_dn8 = assign96480_e149103_d_n8;
        locals.var_isbs2_btm_dn9 = assign96480_e149103_d_n9;
        locals.var_isbs2_btm_dn10 = assign96480_e149103_d_n10;
        locals.var_isbs2_btm_dn13 = assign96480_e149103_d_n13;

        let (assign96490_e149115, assign96490_e149115_d_n0, assign96490_e149115_d_n2, assign96490_e149115_d_n4, assign96490_e149115_d_n5, assign96490_e149115_d_n6, assign96490_e149115_d_n7, assign96490_e149115_d_n8, assign96490_e149115_d_n9, assign96490_e149115_d_n10, assign96490_e149115_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96490_e149111: f64 = (p.p16 - locals.var_weff_nf);
        let assign96490_e149113: f64 = (assign96490_e149111 * locals.var_jssw);
        (assign96490_e149113, (assign96490_e149111 * locals.var_jssw_dn0), (assign96490_e149111 * locals.var_jssw_dn2), (assign96490_e149111 * locals.var_jssw_dn4), (assign96490_e149111 * locals.var_jssw_dn5), (assign96490_e149111 * locals.var_jssw_dn6), (assign96490_e149111 * locals.var_jssw_dn7), (assign96490_e149111 * locals.var_jssw_dn8), (assign96490_e149111 * locals.var_jssw_dn9), (assign96490_e149111 * locals.var_jssw_dn10), (assign96490_e149111 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96490_e149115;
        locals.var_isbs_sws_dn0 = assign96490_e149115_d_n0;
        locals.var_isbs_sws_dn2 = assign96490_e149115_d_n2;
        locals.var_isbs_sws_dn4 = assign96490_e149115_d_n4;
        locals.var_isbs_sws_dn5 = assign96490_e149115_d_n5;
        locals.var_isbs_sws_dn6 = assign96490_e149115_d_n6;
        locals.var_isbs_sws_dn7 = assign96490_e149115_d_n7;
        locals.var_isbs_sws_dn8 = assign96490_e149115_d_n8;
        locals.var_isbs_sws_dn9 = assign96490_e149115_d_n9;
        locals.var_isbs_sws_dn10 = assign96490_e149115_d_n10;
        locals.var_isbs_sws_dn13 = assign96490_e149115_d_n13;

        let (assign96500_e149127, assign96500_e149127_d_n0, assign96500_e149127_d_n2, assign96500_e149127_d_n4, assign96500_e149127_d_n5, assign96500_e149127_d_n6, assign96500_e149127_d_n7, assign96500_e149127_d_n8, assign96500_e149127_d_n9, assign96500_e149127_d_n10, assign96500_e149127_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96500_e149123: f64 = (p.p16 - locals.var_weff_nf);
        let assign96500_e149125: f64 = (assign96500_e149123 * locals.var_jssw2);
        (assign96500_e149125, (assign96500_e149123 * locals.var_jssw2_dn0), (assign96500_e149123 * locals.var_jssw2_dn2), (assign96500_e149123 * locals.var_jssw2_dn4), (assign96500_e149123 * locals.var_jssw2_dn5), (assign96500_e149123 * locals.var_jssw2_dn6), (assign96500_e149123 * locals.var_jssw2_dn7), (assign96500_e149123 * locals.var_jssw2_dn8), (assign96500_e149123 * locals.var_jssw2_dn9), (assign96500_e149123 * locals.var_jssw2_dn10), (assign96500_e149123 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96500_e149127;
        locals.var_isbs2_sws_dn0 = assign96500_e149127_d_n0;
        locals.var_isbs2_sws_dn2 = assign96500_e149127_d_n2;
        locals.var_isbs2_sws_dn4 = assign96500_e149127_d_n4;
        locals.var_isbs2_sws_dn5 = assign96500_e149127_d_n5;
        locals.var_isbs2_sws_dn6 = assign96500_e149127_d_n6;
        locals.var_isbs2_sws_dn7 = assign96500_e149127_d_n7;
        locals.var_isbs2_sws_dn8 = assign96500_e149127_d_n8;
        locals.var_isbs2_sws_dn9 = assign96500_e149127_d_n9;
        locals.var_isbs2_sws_dn10 = assign96500_e149127_d_n10;
        locals.var_isbs2_sws_dn13 = assign96500_e149127_d_n13;

        let (assign96510_e149137, assign96510_e149137_d_n0, assign96510_e149137_d_n2, assign96510_e149137_d_n4, assign96510_e149137_d_n5, assign96510_e149137_d_n6, assign96510_e149137_d_n7, assign96510_e149137_d_n8, assign96510_e149137_d_n9, assign96510_e149137_d_n10, assign96510_e149137_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96510_e149135: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96510_e149135, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96510_e149137;
        locals.var_isbs_swg_dn0 = assign96510_e149137_d_n0;
        locals.var_isbs_swg_dn2 = assign96510_e149137_d_n2;
        locals.var_isbs_swg_dn4 = assign96510_e149137_d_n4;
        locals.var_isbs_swg_dn5 = assign96510_e149137_d_n5;
        locals.var_isbs_swg_dn6 = assign96510_e149137_d_n6;
        locals.var_isbs_swg_dn7 = assign96510_e149137_d_n7;
        locals.var_isbs_swg_dn8 = assign96510_e149137_d_n8;
        locals.var_isbs_swg_dn9 = assign96510_e149137_d_n9;
        locals.var_isbs_swg_dn10 = assign96510_e149137_d_n10;
        locals.var_isbs_swg_dn13 = assign96510_e149137_d_n13;

        let (assign96520_e149147, assign96520_e149147_d_n0, assign96520_e149147_d_n2, assign96520_e149147_d_n4, assign96520_e149147_d_n5, assign96520_e149147_d_n6, assign96520_e149147_d_n7, assign96520_e149147_d_n8, assign96520_e149147_d_n9, assign96520_e149147_d_n10, assign96520_e149147_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96520_e149145: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96520_e149145, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96520_e149147;
        locals.var_isbs2_swg_dn0 = assign96520_e149147_d_n0;
        locals.var_isbs2_swg_dn2 = assign96520_e149147_d_n2;
        locals.var_isbs2_swg_dn4 = assign96520_e149147_d_n4;
        locals.var_isbs2_swg_dn5 = assign96520_e149147_d_n5;
        locals.var_isbs2_swg_dn6 = assign96520_e149147_d_n6;
        locals.var_isbs2_swg_dn7 = assign96520_e149147_d_n7;
        locals.var_isbs2_swg_dn8 = assign96520_e149147_d_n8;
        locals.var_isbs2_swg_dn9 = assign96520_e149147_d_n9;
        locals.var_isbs2_swg_dn10 = assign96520_e149147_d_n10;
        locals.var_isbs2_swg_dn13 = assign96520_e149147_d_n13;

    }

    pub(super) fn stamp_transient_block_344(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96530_e149158, assign96530_e149158_d_n0, assign96530_e149158_d_n2, assign96530_e149158_d_n4, assign96530_e149158_d_n5, assign96530_e149158_d_n6, assign96530_e149158_d_n7, assign96530_e149158_d_n8, assign96530_e149158_d_n9, assign96530_e149158_d_n10, assign96530_e149158_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96530_e149156: f64 = (p.p14 * locals.var_js);
        (assign96530_e149156, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96530_e149158;
        locals.var_isbs_btm_dn0 = assign96530_e149158_d_n0;
        locals.var_isbs_btm_dn2 = assign96530_e149158_d_n2;
        locals.var_isbs_btm_dn4 = assign96530_e149158_d_n4;
        locals.var_isbs_btm_dn5 = assign96530_e149158_d_n5;
        locals.var_isbs_btm_dn6 = assign96530_e149158_d_n6;
        locals.var_isbs_btm_dn7 = assign96530_e149158_d_n7;
        locals.var_isbs_btm_dn8 = assign96530_e149158_d_n8;
        locals.var_isbs_btm_dn9 = assign96530_e149158_d_n9;
        locals.var_isbs_btm_dn10 = assign96530_e149158_d_n10;
        locals.var_isbs_btm_dn13 = assign96530_e149158_d_n13;

        let (assign96540_e149169, assign96540_e149169_d_n0, assign96540_e149169_d_n2, assign96540_e149169_d_n4, assign96540_e149169_d_n5, assign96540_e149169_d_n6, assign96540_e149169_d_n7, assign96540_e149169_d_n8, assign96540_e149169_d_n9, assign96540_e149169_d_n10, assign96540_e149169_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96540_e149167: f64 = (p.p14 * locals.var_js2);
        (assign96540_e149167, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96540_e149169;
        locals.var_isbs2_btm_dn0 = assign96540_e149169_d_n0;
        locals.var_isbs2_btm_dn2 = assign96540_e149169_d_n2;
        locals.var_isbs2_btm_dn4 = assign96540_e149169_d_n4;
        locals.var_isbs2_btm_dn5 = assign96540_e149169_d_n5;
        locals.var_isbs2_btm_dn6 = assign96540_e149169_d_n6;
        locals.var_isbs2_btm_dn7 = assign96540_e149169_d_n7;
        locals.var_isbs2_btm_dn8 = assign96540_e149169_d_n8;
        locals.var_isbs2_btm_dn9 = assign96540_e149169_d_n9;
        locals.var_isbs2_btm_dn10 = assign96540_e149169_d_n10;
        locals.var_isbs2_btm_dn13 = assign96540_e149169_d_n13;

        let (assign96550_e149178, assign96550_e149178_d_n0, assign96550_e149178_d_n2, assign96550_e149178_d_n4, assign96550_e149178_d_n5, assign96550_e149178_d_n6, assign96550_e149178_d_n7, assign96550_e149178_d_n8, assign96550_e149178_d_n9, assign96550_e149178_d_n10, assign96550_e149178_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96550_e149178;
        locals.var_isbs_sws_dn0 = assign96550_e149178_d_n0;
        locals.var_isbs_sws_dn2 = assign96550_e149178_d_n2;
        locals.var_isbs_sws_dn4 = assign96550_e149178_d_n4;
        locals.var_isbs_sws_dn5 = assign96550_e149178_d_n5;
        locals.var_isbs_sws_dn6 = assign96550_e149178_d_n6;
        locals.var_isbs_sws_dn7 = assign96550_e149178_d_n7;
        locals.var_isbs_sws_dn8 = assign96550_e149178_d_n8;
        locals.var_isbs_sws_dn9 = assign96550_e149178_d_n9;
        locals.var_isbs_sws_dn10 = assign96550_e149178_d_n10;
        locals.var_isbs_sws_dn13 = assign96550_e149178_d_n13;

        let (assign96560_e149187, assign96560_e149187_d_n0, assign96560_e149187_d_n2, assign96560_e149187_d_n4, assign96560_e149187_d_n5, assign96560_e149187_d_n6, assign96560_e149187_d_n7, assign96560_e149187_d_n8, assign96560_e149187_d_n9, assign96560_e149187_d_n10, assign96560_e149187_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96560_e149187;
        locals.var_isbs2_sws_dn0 = assign96560_e149187_d_n0;
        locals.var_isbs2_sws_dn2 = assign96560_e149187_d_n2;
        locals.var_isbs2_sws_dn4 = assign96560_e149187_d_n4;
        locals.var_isbs2_sws_dn5 = assign96560_e149187_d_n5;
        locals.var_isbs2_sws_dn6 = assign96560_e149187_d_n6;
        locals.var_isbs2_sws_dn7 = assign96560_e149187_d_n7;
        locals.var_isbs2_sws_dn8 = assign96560_e149187_d_n8;
        locals.var_isbs2_sws_dn9 = assign96560_e149187_d_n9;
        locals.var_isbs2_sws_dn10 = assign96560_e149187_d_n10;
        locals.var_isbs2_sws_dn13 = assign96560_e149187_d_n13;

        let (assign96570_e149198, assign96570_e149198_d_n0, assign96570_e149198_d_n2, assign96570_e149198_d_n4, assign96570_e149198_d_n5, assign96570_e149198_d_n6, assign96570_e149198_d_n7, assign96570_e149198_d_n8, assign96570_e149198_d_n9, assign96570_e149198_d_n10, assign96570_e149198_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96570_e149196: f64 = (p.p16 * locals.var_jsswg);
        (assign96570_e149196, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96570_e149198;
        locals.var_isbs_swg_dn0 = assign96570_e149198_d_n0;
        locals.var_isbs_swg_dn2 = assign96570_e149198_d_n2;
        locals.var_isbs_swg_dn4 = assign96570_e149198_d_n4;
        locals.var_isbs_swg_dn5 = assign96570_e149198_d_n5;
        locals.var_isbs_swg_dn6 = assign96570_e149198_d_n6;
        locals.var_isbs_swg_dn7 = assign96570_e149198_d_n7;
        locals.var_isbs_swg_dn8 = assign96570_e149198_d_n8;
        locals.var_isbs_swg_dn9 = assign96570_e149198_d_n9;
        locals.var_isbs_swg_dn10 = assign96570_e149198_d_n10;
        locals.var_isbs_swg_dn13 = assign96570_e149198_d_n13;

        let (assign96580_e149209, assign96580_e149209_d_n0, assign96580_e149209_d_n2, assign96580_e149209_d_n4, assign96580_e149209_d_n5, assign96580_e149209_d_n6, assign96580_e149209_d_n7, assign96580_e149209_d_n8, assign96580_e149209_d_n9, assign96580_e149209_d_n10, assign96580_e149209_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96580_e149207: f64 = (p.p16 * locals.var_jsswg2);
        (assign96580_e149207, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96580_e149209;
        locals.var_isbs2_swg_dn0 = assign96580_e149209_d_n0;
        locals.var_isbs2_swg_dn2 = assign96580_e149209_d_n2;
        locals.var_isbs2_swg_dn4 = assign96580_e149209_d_n4;
        locals.var_isbs2_swg_dn5 = assign96580_e149209_d_n5;
        locals.var_isbs2_swg_dn6 = assign96580_e149209_d_n6;
        locals.var_isbs2_swg_dn7 = assign96580_e149209_d_n7;
        locals.var_isbs2_swg_dn8 = assign96580_e149209_d_n8;
        locals.var_isbs2_swg_dn9 = assign96580_e149209_d_n9;
        locals.var_isbs2_swg_dn10 = assign96580_e149209_d_n10;
        locals.var_isbs2_swg_dn13 = assign96580_e149209_d_n13;

        let (assign96590_e149218, assign96590_e149218_d_n0, assign96590_e149218_d_n2, assign96590_e149218_d_n4, assign96590_e149218_d_n5, assign96590_e149218_d_n6, assign96590_e149218_d_n7, assign96590_e149218_d_n8, assign96590_e149218_d_n9, assign96590_e149218_d_n10, assign96590_e149218_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96590_e149216: f64 = (p.p14 * locals.var_js);
        (assign96590_e149216, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96590_e149218;
        locals.var_isbs_btm_dn0 = assign96590_e149218_d_n0;
        locals.var_isbs_btm_dn2 = assign96590_e149218_d_n2;
        locals.var_isbs_btm_dn4 = assign96590_e149218_d_n4;
        locals.var_isbs_btm_dn5 = assign96590_e149218_d_n5;
        locals.var_isbs_btm_dn6 = assign96590_e149218_d_n6;
        locals.var_isbs_btm_dn7 = assign96590_e149218_d_n7;
        locals.var_isbs_btm_dn8 = assign96590_e149218_d_n8;
        locals.var_isbs_btm_dn9 = assign96590_e149218_d_n9;
        locals.var_isbs_btm_dn10 = assign96590_e149218_d_n10;
        locals.var_isbs_btm_dn13 = assign96590_e149218_d_n13;

        let (assign96600_e149227, assign96600_e149227_d_n0, assign96600_e149227_d_n2, assign96600_e149227_d_n4, assign96600_e149227_d_n5, assign96600_e149227_d_n6, assign96600_e149227_d_n7, assign96600_e149227_d_n8, assign96600_e149227_d_n9, assign96600_e149227_d_n10, assign96600_e149227_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96600_e149225: f64 = (p.p14 * locals.var_js2);
        (assign96600_e149225, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96600_e149227;
        locals.var_isbs2_btm_dn0 = assign96600_e149227_d_n0;
        locals.var_isbs2_btm_dn2 = assign96600_e149227_d_n2;
        locals.var_isbs2_btm_dn4 = assign96600_e149227_d_n4;
        locals.var_isbs2_btm_dn5 = assign96600_e149227_d_n5;
        locals.var_isbs2_btm_dn6 = assign96600_e149227_d_n6;
        locals.var_isbs2_btm_dn7 = assign96600_e149227_d_n7;
        locals.var_isbs2_btm_dn8 = assign96600_e149227_d_n8;
        locals.var_isbs2_btm_dn9 = assign96600_e149227_d_n9;
        locals.var_isbs2_btm_dn10 = assign96600_e149227_d_n10;
        locals.var_isbs2_btm_dn13 = assign96600_e149227_d_n13;

        let (assign96610_e149236, assign96610_e149236_d_n0, assign96610_e149236_d_n2, assign96610_e149236_d_n4, assign96610_e149236_d_n5, assign96610_e149236_d_n6, assign96610_e149236_d_n7, assign96610_e149236_d_n8, assign96610_e149236_d_n9, assign96610_e149236_d_n10, assign96610_e149236_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96610_e149234: f64 = (p.p16 * locals.var_jssw);
        (assign96610_e149234, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96610_e149236;
        locals.var_isbs_sws_dn0 = assign96610_e149236_d_n0;
        locals.var_isbs_sws_dn2 = assign96610_e149236_d_n2;
        locals.var_isbs_sws_dn4 = assign96610_e149236_d_n4;
        locals.var_isbs_sws_dn5 = assign96610_e149236_d_n5;
        locals.var_isbs_sws_dn6 = assign96610_e149236_d_n6;
        locals.var_isbs_sws_dn7 = assign96610_e149236_d_n7;
        locals.var_isbs_sws_dn8 = assign96610_e149236_d_n8;
        locals.var_isbs_sws_dn9 = assign96610_e149236_d_n9;
        locals.var_isbs_sws_dn10 = assign96610_e149236_d_n10;
        locals.var_isbs_sws_dn13 = assign96610_e149236_d_n13;

        let (assign96620_e149245, assign96620_e149245_d_n0, assign96620_e149245_d_n2, assign96620_e149245_d_n4, assign96620_e149245_d_n5, assign96620_e149245_d_n6, assign96620_e149245_d_n7, assign96620_e149245_d_n8, assign96620_e149245_d_n9, assign96620_e149245_d_n10, assign96620_e149245_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96620_e149243: f64 = (p.p16 * locals.var_jssw2);
        (assign96620_e149243, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96620_e149245;
        locals.var_isbs2_sws_dn0 = assign96620_e149245_d_n0;
        locals.var_isbs2_sws_dn2 = assign96620_e149245_d_n2;
        locals.var_isbs2_sws_dn4 = assign96620_e149245_d_n4;
        locals.var_isbs2_sws_dn5 = assign96620_e149245_d_n5;
        locals.var_isbs2_sws_dn6 = assign96620_e149245_d_n6;
        locals.var_isbs2_sws_dn7 = assign96620_e149245_d_n7;
        locals.var_isbs2_sws_dn8 = assign96620_e149245_d_n8;
        locals.var_isbs2_sws_dn9 = assign96620_e149245_d_n9;
        locals.var_isbs2_sws_dn10 = assign96620_e149245_d_n10;
        locals.var_isbs2_sws_dn13 = assign96620_e149245_d_n13;

        let (assign96630_e149252, assign96630_e149252_d_n0, assign96630_e149252_d_n2, assign96630_e149252_d_n4, assign96630_e149252_d_n5, assign96630_e149252_d_n6, assign96630_e149252_d_n7, assign96630_e149252_d_n8, assign96630_e149252_d_n9, assign96630_e149252_d_n10, assign96630_e149252_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96630_e149252;
        locals.var_isbs_swg_dn0 = assign96630_e149252_d_n0;
        locals.var_isbs_swg_dn2 = assign96630_e149252_d_n2;
        locals.var_isbs_swg_dn4 = assign96630_e149252_d_n4;
        locals.var_isbs_swg_dn5 = assign96630_e149252_d_n5;
        locals.var_isbs_swg_dn6 = assign96630_e149252_d_n6;
        locals.var_isbs_swg_dn7 = assign96630_e149252_d_n7;
        locals.var_isbs_swg_dn8 = assign96630_e149252_d_n8;
        locals.var_isbs_swg_dn9 = assign96630_e149252_d_n9;
        locals.var_isbs_swg_dn10 = assign96630_e149252_d_n10;
        locals.var_isbs_swg_dn13 = assign96630_e149252_d_n13;

        let (assign96640_e149259, assign96640_e149259_d_n0, assign96640_e149259_d_n2, assign96640_e149259_d_n4, assign96640_e149259_d_n5, assign96640_e149259_d_n6, assign96640_e149259_d_n7, assign96640_e149259_d_n8, assign96640_e149259_d_n9, assign96640_e149259_d_n10, assign96640_e149259_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96640_e149259;
        locals.var_isbs2_swg_dn0 = assign96640_e149259_d_n0;
        locals.var_isbs2_swg_dn2 = assign96640_e149259_d_n2;
        locals.var_isbs2_swg_dn4 = assign96640_e149259_d_n4;
        locals.var_isbs2_swg_dn5 = assign96640_e149259_d_n5;
        locals.var_isbs2_swg_dn6 = assign96640_e149259_d_n6;
        locals.var_isbs2_swg_dn7 = assign96640_e149259_d_n7;
        locals.var_isbs2_swg_dn8 = assign96640_e149259_d_n8;
        locals.var_isbs2_swg_dn9 = assign96640_e149259_d_n9;
        locals.var_isbs2_swg_dn10 = assign96640_e149259_d_n10;
        locals.var_isbs2_swg_dn13 = assign96640_e149259_d_n13;

        let (assign96650_e149267, assign96650_e149267_d_n0, assign96650_e149267_d_n2, assign96650_e149267_d_n4, assign96650_e149267_d_n5, assign96650_e149267_d_n6, assign96650_e149267_d_n7, assign96650_e149267_d_n8, assign96650_e149267_d_n9, assign96650_e149267_d_n10, assign96650_e149267_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96650_e149263: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96650_e149265: f64 = (assign96650_e149263 + locals.var_isbs_swg);
        (assign96650_e149265, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn13 + locals.var_isbs_sws_dn13) + locals.var_isbs_swg_dn13),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    }
};
        locals.var_isbs = assign96650_e149267;
        locals.var_isbs_dn0 = assign96650_e149267_d_n0;
        locals.var_isbs_dn2 = assign96650_e149267_d_n2;
        locals.var_isbs_dn4 = assign96650_e149267_d_n4;
        locals.var_isbs_dn5 = assign96650_e149267_d_n5;
        locals.var_isbs_dn6 = assign96650_e149267_d_n6;
        locals.var_isbs_dn7 = assign96650_e149267_d_n7;
        locals.var_isbs_dn8 = assign96650_e149267_d_n8;
        locals.var_isbs_dn9 = assign96650_e149267_d_n9;
        locals.var_isbs_dn10 = assign96650_e149267_d_n10;
        locals.var_isbs_dn13 = assign96650_e149267_d_n13;

        let assign96660_e149270: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96660_e149270;

        let (assign96670_e149278, assign96670_e149278_d_n0, assign96670_e149278_d_n2, assign96670_e149278_d_n4, assign96670_e149278_d_n5, assign96670_e149278_d_n6, assign96670_e149278_d_n7, assign96670_e149278_d_n8, assign96670_e149278_d_n9, assign96670_e149278_d_n10, assign96670_e149278_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96670_e149276: f64 = (locals.var_isbs + 1e-25);
        (assign96670_e149276, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign96670_e149278;
        locals.var_t3_dn0 = assign96670_e149278_d_n0;
        locals.var_t3_dn2 = assign96670_e149278_d_n2;
        locals.var_t3_dn4 = assign96670_e149278_d_n4;
        locals.var_t3_dn5 = assign96670_e149278_d_n5;
        locals.var_t3_dn6 = assign96670_e149278_d_n6;
        locals.var_t3_dn7 = assign96670_e149278_d_n7;
        locals.var_t3_dn8 = assign96670_e149278_d_n8;
        locals.var_t3_dn9 = assign96670_e149278_d_n9;
        locals.var_t3_dn10 = assign96670_e149278_d_n10;
        locals.var_t3_dn13 = assign96670_e149278_d_n13;

        let (assign96680_e149295, assign96680_e149295_d_n0, assign96680_e149295_d_n2, assign96680_e149295_d_n4, assign96680_e149295_d_n5, assign96680_e149295_d_n6, assign96680_e149295_d_n7, assign96680_e149295_d_n8, assign96680_e149295_d_n9, assign96680_e149295_d_n10, assign96680_e149295_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96680_e149284: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96680_e149287: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96680_e149289: f64 = (assign96680_e149287 / locals.var_t3);
        let assign96680_e149291: f64 = (assign96680_e149289 + 1.0);
        let assign96680_e149292: f64 = (assign96680_e149291).ln();
        let assign96680_e149293: f64 = (assign96680_e149284 * assign96680_e149292);
        (assign96680_e149293, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn13) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn13,)
    }
};
        locals.var_vbst = assign96680_e149295;
        locals.var_vbst_dn0 = assign96680_e149295_d_n0;
        locals.var_vbst_dn2 = assign96680_e149295_d_n2;
        locals.var_vbst_dn4 = assign96680_e149295_d_n4;
        locals.var_vbst_dn5 = assign96680_e149295_d_n5;
        locals.var_vbst_dn6 = assign96680_e149295_d_n6;
        locals.var_vbst_dn7 = assign96680_e149295_d_n7;
        locals.var_vbst_dn8 = assign96680_e149295_d_n8;
        locals.var_vbst_dn9 = assign96680_e149295_d_n9;
        locals.var_vbst_dn10 = assign96680_e149295_d_n10;
        locals.var_vbst_dn13 = assign96680_e149295_d_n13;

        let (assign96690_e149306, assign96690_e149306_d_n0, assign96690_e149306_d_n2, assign96690_e149306_d_n4, assign96690_e149306_d_n5, assign96690_e149306_d_n6, assign96690_e149306_d_n7, assign96690_e149306_d_n8, assign96690_e149306_d_n9, assign96690_e149306_d_n10, assign96690_e149306_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96690_e149301: f64 = (locals.var_tratio - 1.0);
        let assign96690_e149303: f64 = (assign96690_e149301 * p.p535);
        let assign96690_e149304: f64 = (assign96690_e149303).exp();
        (assign96690_e149304, (assign96690_e149304 * (locals.var_tratio_dn0 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn2 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn4 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn5 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn6 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn7 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn8 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn9 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn10 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn13 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn13,)
    }
};
        locals.var_exptemps = assign96690_e149306;
        locals.var_exptemps_dn0 = assign96690_e149306_d_n0;
        locals.var_exptemps_dn2 = assign96690_e149306_d_n2;
        locals.var_exptemps_dn4 = assign96690_e149306_d_n4;
        locals.var_exptemps_dn5 = assign96690_e149306_d_n5;
        locals.var_exptemps_dn6 = assign96690_e149306_d_n6;
        locals.var_exptemps_dn7 = assign96690_e149306_d_n7;
        locals.var_exptemps_dn8 = assign96690_e149306_d_n8;
        locals.var_exptemps_dn9 = assign96690_e149306_d_n9;
        locals.var_exptemps_dn10 = assign96690_e149306_d_n10;
        locals.var_exptemps_dn13 = assign96690_e149306_d_n13;

        let (assign96700_e149316, assign96700_e149316_d_n0, assign96700_e149316_d_n2, assign96700_e149316_d_n4, assign96700_e149316_d_n5, assign96700_e149316_d_n6, assign96700_e149316_d_n7, assign96700_e149316_d_n8, assign96700_e149316_d_n9, assign96700_e149316_d_n10, assign96700_e149316_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96700_e149313: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96700_e149314: f64 = (1.0 / assign96700_e149313);
        (assign96700_e149314, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn13,)
    }
};
        locals.var_jd_nvtm_invs = assign96700_e149316;
        locals.var_jd_nvtm_invs_dn0 = assign96700_e149316_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96700_e149316_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96700_e149316_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96700_e149316_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96700_e149316_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96700_e149316_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96700_e149316_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96700_e149316_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96700_e149316_d_n10;
        locals.var_jd_nvtm_invs_dn13 = assign96700_e149316_d_n13;

        let (assign96710_e149325, assign96710_e149325_d_n0, assign96710_e149325_d_n2, assign96710_e149325_d_n4, assign96710_e149325_d_n5, assign96710_e149325_d_n6, assign96710_e149325_d_n7, assign96710_e149325_d_n8, assign96710_e149325_d_n9, assign96710_e149325_d_n10, assign96710_e149325_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96710_e149322: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96710_e149323: f64 = (assign96710_e149322).exp();
        (assign96710_e149323, (assign96710_e149323 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96710_e149323 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96710_e149323 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96710_e149323 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96710_e149323 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96710_e149323 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96710_e149323 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96710_e149323 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96710_e149323 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96710_e149323 * ((locals.var_vbst_dn13 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn13))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    }
};
        locals.var_jd_expcs = assign96710_e149325;
        locals.var_jd_expcs_dn0 = assign96710_e149325_d_n0;
        locals.var_jd_expcs_dn2 = assign96710_e149325_d_n2;
        locals.var_jd_expcs_dn4 = assign96710_e149325_d_n4;
        locals.var_jd_expcs_dn5 = assign96710_e149325_d_n5;
        locals.var_jd_expcs_dn6 = assign96710_e149325_d_n6;
        locals.var_jd_expcs_dn7 = assign96710_e149325_d_n7;
        locals.var_jd_expcs_dn8 = assign96710_e149325_d_n8;
        locals.var_jd_expcs_dn9 = assign96710_e149325_d_n9;
        locals.var_jd_expcs_dn10 = assign96710_e149325_d_n10;
        locals.var_jd_expcs_dn13 = assign96710_e149325_d_n13;

        let (assign96720_e149337, assign96720_e149337_d_n0, assign96720_e149337_d_n2, assign96720_e149337_d_n4, assign96720_e149337_d_n5, assign96720_e149337_d_n6, assign96720_e149337_d_n7, assign96720_e149337_d_n8, assign96720_e149337_d_n9, assign96720_e149337_d_n10, assign96720_e149337_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96720_e149329: f64 = (p.p500 * p.p13);
        let assign96720_e149333: f64 = (p.p481 * locals.var_tdiff);
        let assign96720_e149334: f64 = (1.0 + assign96720_e149333);
        let assign96720_e149335: f64 = (assign96720_e149329 * assign96720_e149334);
        (assign96720_e149335, (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn0)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn2)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn4)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn5)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn6)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn7)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn8)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn9)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn10)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign96720_e149337;
        locals.var_czbd_dn0 = assign96720_e149337_d_n0;
        locals.var_czbd_dn2 = assign96720_e149337_d_n2;
        locals.var_czbd_dn4 = assign96720_e149337_d_n4;
        locals.var_czbd_dn5 = assign96720_e149337_d_n5;
        locals.var_czbd_dn6 = assign96720_e149337_d_n6;
        locals.var_czbd_dn7 = assign96720_e149337_d_n7;
        locals.var_czbd_dn8 = assign96720_e149337_d_n8;
        locals.var_czbd_dn9 = assign96720_e149337_d_n9;
        locals.var_czbd_dn10 = assign96720_e149337_d_n10;
        locals.var_czbd_dn13 = assign96720_e149337_d_n13;

        let assign96730_e149340: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96730_e149340;

        let (assign96740_e149356, assign96740_e149356_d_n0, assign96740_e149356_d_n2, assign96740_e149356_d_n4, assign96740_e149356_d_n5, assign96740_e149356_d_n6, assign96740_e149356_d_n7, assign96740_e149356_d_n8, assign96740_e149356_d_n9, assign96740_e149356_d_n10, assign96740_e149356_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96740_e149347: f64 = (p.p15 - locals.var_weff_nf);
        let assign96740_e149348: f64 = (p.p501 * assign96740_e149347);
        let assign96740_e149352: f64 = (p.p483 * locals.var_tdiff);
        let assign96740_e149353: f64 = (1.0 + assign96740_e149352);
        let assign96740_e149354: f64 = (assign96740_e149348 * assign96740_e149353);
        (assign96740_e149354, (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn0)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn2)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn4)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn5)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn6)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn7)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn8)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn9)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn10)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96740_e149356;
        locals.var_czbdsw_dn0 = assign96740_e149356_d_n0;
        locals.var_czbdsw_dn2 = assign96740_e149356_d_n2;
        locals.var_czbdsw_dn4 = assign96740_e149356_d_n4;
        locals.var_czbdsw_dn5 = assign96740_e149356_d_n5;
        locals.var_czbdsw_dn6 = assign96740_e149356_d_n6;
        locals.var_czbdsw_dn7 = assign96740_e149356_d_n7;
        locals.var_czbdsw_dn8 = assign96740_e149356_d_n8;
        locals.var_czbdsw_dn9 = assign96740_e149356_d_n9;
        locals.var_czbdsw_dn10 = assign96740_e149356_d_n10;
        locals.var_czbdsw_dn13 = assign96740_e149356_d_n13;

        let (assign96750_e149370, assign96750_e149370_d_n0, assign96750_e149370_d_n2, assign96750_e149370_d_n4, assign96750_e149370_d_n5, assign96750_e149370_d_n6, assign96750_e149370_d_n7, assign96750_e149370_d_n8, assign96750_e149370_d_n9, assign96750_e149370_d_n10, assign96750_e149370_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96750_e149362: f64 = (p.p502 * locals.var_weff_nf);
        let assign96750_e149366: f64 = (p.p485 * locals.var_tdiff);
        let assign96750_e149367: f64 = (1.0 + assign96750_e149366);
        let assign96750_e149368: f64 = (assign96750_e149362 * assign96750_e149367);
        (assign96750_e149368, (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn0)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn2)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn4)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn5)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn6)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn7)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn8)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn9)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn10)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96750_e149370;
        locals.var_czbdswg_dn0 = assign96750_e149370_d_n0;
        locals.var_czbdswg_dn2 = assign96750_e149370_d_n2;
        locals.var_czbdswg_dn4 = assign96750_e149370_d_n4;
        locals.var_czbdswg_dn5 = assign96750_e149370_d_n5;
        locals.var_czbdswg_dn6 = assign96750_e149370_d_n6;
        locals.var_czbdswg_dn7 = assign96750_e149370_d_n7;
        locals.var_czbdswg_dn8 = assign96750_e149370_d_n8;
        locals.var_czbdswg_dn9 = assign96750_e149370_d_n9;
        locals.var_czbdswg_dn10 = assign96750_e149370_d_n10;
        locals.var_czbdswg_dn13 = assign96750_e149370_d_n13;

        let (assign96760_e149377, assign96760_e149377_d_n0, assign96760_e149377_d_n2, assign96760_e149377_d_n4, assign96760_e149377_d_n5, assign96760_e149377_d_n6, assign96760_e149377_d_n7, assign96760_e149377_d_n8, assign96760_e149377_d_n9, assign96760_e149377_d_n10, assign96760_e149377_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96760_e149377;
        locals.var_czbdsw_dn0 = assign96760_e149377_d_n0;
        locals.var_czbdsw_dn2 = assign96760_e149377_d_n2;
        locals.var_czbdsw_dn4 = assign96760_e149377_d_n4;
        locals.var_czbdsw_dn5 = assign96760_e149377_d_n5;
        locals.var_czbdsw_dn6 = assign96760_e149377_d_n6;
        locals.var_czbdsw_dn7 = assign96760_e149377_d_n7;
        locals.var_czbdsw_dn8 = assign96760_e149377_d_n8;
        locals.var_czbdsw_dn9 = assign96760_e149377_d_n9;
        locals.var_czbdsw_dn10 = assign96760_e149377_d_n10;
        locals.var_czbdsw_dn13 = assign96760_e149377_d_n13;

        let (assign96770_e149392, assign96770_e149392_d_n0, assign96770_e149392_d_n2, assign96770_e149392_d_n4, assign96770_e149392_d_n5, assign96770_e149392_d_n6, assign96770_e149392_d_n7, assign96770_e149392_d_n8, assign96770_e149392_d_n9, assign96770_e149392_d_n10, assign96770_e149392_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 == 0.0)) {
        let assign96770_e149384: f64 = (p.p502 * p.p15);
        let assign96770_e149388: f64 = (p.p485 * locals.var_tdiff);
        let assign96770_e149389: f64 = (1.0 + assign96770_e149388);
        let assign96770_e149390: f64 = (assign96770_e149384 * assign96770_e149389);
        (assign96770_e149390, (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn0)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn2)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn4)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn5)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn6)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn7)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn8)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn9)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn10)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96770_e149392;
        locals.var_czbdswg_dn0 = assign96770_e149392_d_n0;
        locals.var_czbdswg_dn2 = assign96770_e149392_d_n2;
        locals.var_czbdswg_dn4 = assign96770_e149392_d_n4;
        locals.var_czbdswg_dn5 = assign96770_e149392_d_n5;
        locals.var_czbdswg_dn6 = assign96770_e149392_d_n6;
        locals.var_czbdswg_dn7 = assign96770_e149392_d_n7;
        locals.var_czbdswg_dn8 = assign96770_e149392_d_n8;
        locals.var_czbdswg_dn9 = assign96770_e149392_d_n9;
        locals.var_czbdswg_dn10 = assign96770_e149392_d_n10;
        locals.var_czbdswg_dn13 = assign96770_e149392_d_n13;

        let assign96780_e149395: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96780_e149395;

        let (assign96790_e149401, assign96790_e149401_d_n0, assign96790_e149401_d_n2, assign96790_e149401_d_n4, assign96790_e149401_d_n5, assign96790_e149401_d_n6, assign96790_e149401_d_n7, assign96790_e149401_d_n8, assign96790_e149401_d_n9, assign96790_e149401_d_n10, assign96790_e149401_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2241 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign96790_e149401;
        locals.var_czbd_dn0 = assign96790_e149401_d_n0;
        locals.var_czbd_dn2 = assign96790_e149401_d_n2;
        locals.var_czbd_dn4 = assign96790_e149401_d_n4;
        locals.var_czbd_dn5 = assign96790_e149401_d_n5;
        locals.var_czbd_dn6 = assign96790_e149401_d_n6;
        locals.var_czbd_dn7 = assign96790_e149401_d_n7;
        locals.var_czbd_dn8 = assign96790_e149401_d_n8;
        locals.var_czbd_dn9 = assign96790_e149401_d_n9;
        locals.var_czbd_dn10 = assign96790_e149401_d_n10;
        locals.var_czbd_dn13 = assign96790_e149401_d_n13;

        let assign96800_e149404: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96800_e149404;

    }

    pub(super) fn stamp_transient_block_345(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign96810_e149410, assign96810_e149410_d_n0, assign96810_e149410_d_n2, assign96810_e149410_d_n4, assign96810_e149410_d_n5, assign96810_e149410_d_n6, assign96810_e149410_d_n7, assign96810_e149410_d_n8, assign96810_e149410_d_n9, assign96810_e149410_d_n10, assign96810_e149410_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2242 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96810_e149410;
        locals.var_czbdsw_dn0 = assign96810_e149410_d_n0;
        locals.var_czbdsw_dn2 = assign96810_e149410_d_n2;
        locals.var_czbdsw_dn4 = assign96810_e149410_d_n4;
        locals.var_czbdsw_dn5 = assign96810_e149410_d_n5;
        locals.var_czbdsw_dn6 = assign96810_e149410_d_n6;
        locals.var_czbdsw_dn7 = assign96810_e149410_d_n7;
        locals.var_czbdsw_dn8 = assign96810_e149410_d_n8;
        locals.var_czbdsw_dn9 = assign96810_e149410_d_n9;
        locals.var_czbdsw_dn10 = assign96810_e149410_d_n10;
        locals.var_czbdsw_dn13 = assign96810_e149410_d_n13;

        let assign96820_e149413: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96820_e149413;

        let (assign96830_e149419, assign96830_e149419_d_n0, assign96830_e149419_d_n2, assign96830_e149419_d_n4, assign96830_e149419_d_n5, assign96830_e149419_d_n6, assign96830_e149419_d_n7, assign96830_e149419_d_n8, assign96830_e149419_d_n9, assign96830_e149419_d_n10, assign96830_e149419_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96830_e149419;
        locals.var_czbdswg_dn0 = assign96830_e149419_d_n0;
        locals.var_czbdswg_dn2 = assign96830_e149419_d_n2;
        locals.var_czbdswg_dn4 = assign96830_e149419_d_n4;
        locals.var_czbdswg_dn5 = assign96830_e149419_d_n5;
        locals.var_czbdswg_dn6 = assign96830_e149419_d_n6;
        locals.var_czbdswg_dn7 = assign96830_e149419_d_n7;
        locals.var_czbdswg_dn8 = assign96830_e149419_d_n8;
        locals.var_czbdswg_dn9 = assign96830_e149419_d_n9;
        locals.var_czbdswg_dn10 = assign96830_e149419_d_n10;
        locals.var_czbdswg_dn13 = assign96830_e149419_d_n13;

        let (assign96840_e149427, assign96840_e149427_d_n0, assign96840_e149427_d_n2, assign96840_e149427_d_n4, assign96840_e149427_d_n5, assign96840_e149427_d_n6, assign96840_e149427_d_n7, assign96840_e149427_d_n8, assign96840_e149427_d_n9, assign96840_e149427_d_n10, assign96840_e149427_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96840_e149424: f64 = (p.p487 * locals.var_tdiff);
        let assign96840_e149425: f64 = (p.p506 - assign96840_e149424);
        (assign96840_e149425, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign96840_e149427;
        locals.var_pzbd_dn0 = assign96840_e149427_d_n0;
        locals.var_pzbd_dn2 = assign96840_e149427_d_n2;
        locals.var_pzbd_dn4 = assign96840_e149427_d_n4;
        locals.var_pzbd_dn5 = assign96840_e149427_d_n5;
        locals.var_pzbd_dn6 = assign96840_e149427_d_n6;
        locals.var_pzbd_dn7 = assign96840_e149427_d_n7;
        locals.var_pzbd_dn8 = assign96840_e149427_d_n8;
        locals.var_pzbd_dn9 = assign96840_e149427_d_n9;
        locals.var_pzbd_dn10 = assign96840_e149427_d_n10;
        locals.var_pzbd_dn13 = assign96840_e149427_d_n13;

        let (assign96850_e149435, assign96850_e149435_d_n0, assign96850_e149435_d_n2, assign96850_e149435_d_n4, assign96850_e149435_d_n5, assign96850_e149435_d_n6, assign96850_e149435_d_n7, assign96850_e149435_d_n8, assign96850_e149435_d_n9, assign96850_e149435_d_n10, assign96850_e149435_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96850_e149432: f64 = (p.p489 * locals.var_tdiff);
        let assign96850_e149433: f64 = (p.p507 - assign96850_e149432);
        (assign96850_e149433, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign96850_e149435;
        locals.var_pzbdsw_dn0 = assign96850_e149435_d_n0;
        locals.var_pzbdsw_dn2 = assign96850_e149435_d_n2;
        locals.var_pzbdsw_dn4 = assign96850_e149435_d_n4;
        locals.var_pzbdsw_dn5 = assign96850_e149435_d_n5;
        locals.var_pzbdsw_dn6 = assign96850_e149435_d_n6;
        locals.var_pzbdsw_dn7 = assign96850_e149435_d_n7;
        locals.var_pzbdsw_dn8 = assign96850_e149435_d_n8;
        locals.var_pzbdsw_dn9 = assign96850_e149435_d_n9;
        locals.var_pzbdsw_dn10 = assign96850_e149435_d_n10;
        locals.var_pzbdsw_dn13 = assign96850_e149435_d_n13;

        let (assign96860_e149443, assign96860_e149443_d_n0, assign96860_e149443_d_n2, assign96860_e149443_d_n4, assign96860_e149443_d_n5, assign96860_e149443_d_n6, assign96860_e149443_d_n7, assign96860_e149443_d_n8, assign96860_e149443_d_n9, assign96860_e149443_d_n10, assign96860_e149443_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96860_e149440: f64 = (p.p491 * locals.var_tdiff);
        let assign96860_e149441: f64 = (p.p508 - assign96860_e149440);
        (assign96860_e149441, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign96860_e149443;
        locals.var_pzbdswg_dn0 = assign96860_e149443_d_n0;
        locals.var_pzbdswg_dn2 = assign96860_e149443_d_n2;
        locals.var_pzbdswg_dn4 = assign96860_e149443_d_n4;
        locals.var_pzbdswg_dn5 = assign96860_e149443_d_n5;
        locals.var_pzbdswg_dn6 = assign96860_e149443_d_n6;
        locals.var_pzbdswg_dn7 = assign96860_e149443_d_n7;
        locals.var_pzbdswg_dn8 = assign96860_e149443_d_n8;
        locals.var_pzbdswg_dn9 = assign96860_e149443_d_n9;
        locals.var_pzbdswg_dn10 = assign96860_e149443_d_n10;
        locals.var_pzbdswg_dn13 = assign96860_e149443_d_n13;

        let assign96870_e149450: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96870_e149450;

        let (assign96880_e149456, assign96880_e149456_d_n0, assign96880_e149456_d_n2, assign96880_e149456_d_n4, assign96880_e149456_d_n5, assign96880_e149456_d_n6, assign96880_e149456_d_n7, assign96880_e149456_d_n8, assign96880_e149456_d_n9, assign96880_e149456_d_n10, assign96880_e149456_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2244 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign96880_e149456;
        locals.var_pzbd_dn0 = assign96880_e149456_d_n0;
        locals.var_pzbd_dn2 = assign96880_e149456_d_n2;
        locals.var_pzbd_dn4 = assign96880_e149456_d_n4;
        locals.var_pzbd_dn5 = assign96880_e149456_d_n5;
        locals.var_pzbd_dn6 = assign96880_e149456_d_n6;
        locals.var_pzbd_dn7 = assign96880_e149456_d_n7;
        locals.var_pzbd_dn8 = assign96880_e149456_d_n8;
        locals.var_pzbd_dn9 = assign96880_e149456_d_n9;
        locals.var_pzbd_dn10 = assign96880_e149456_d_n10;
        locals.var_pzbd_dn13 = assign96880_e149456_d_n13;

        let assign96890_e149463: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96890_e149463;

        let (assign96900_e149469, assign96900_e149469_d_n0, assign96900_e149469_d_n2, assign96900_e149469_d_n4, assign96900_e149469_d_n5, assign96900_e149469_d_n6, assign96900_e149469_d_n7, assign96900_e149469_d_n8, assign96900_e149469_d_n9, assign96900_e149469_d_n10, assign96900_e149469_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign96900_e149469;
        locals.var_pzbdsw_dn0 = assign96900_e149469_d_n0;
        locals.var_pzbdsw_dn2 = assign96900_e149469_d_n2;
        locals.var_pzbdsw_dn4 = assign96900_e149469_d_n4;
        locals.var_pzbdsw_dn5 = assign96900_e149469_d_n5;
        locals.var_pzbdsw_dn6 = assign96900_e149469_d_n6;
        locals.var_pzbdsw_dn7 = assign96900_e149469_d_n7;
        locals.var_pzbdsw_dn8 = assign96900_e149469_d_n8;
        locals.var_pzbdsw_dn9 = assign96900_e149469_d_n9;
        locals.var_pzbdsw_dn10 = assign96900_e149469_d_n10;
        locals.var_pzbdsw_dn13 = assign96900_e149469_d_n13;

        let assign96910_e149476: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96910_e149476;

        let (assign96920_e149482, assign96920_e149482_d_n0, assign96920_e149482_d_n2, assign96920_e149482_d_n4, assign96920_e149482_d_n5, assign96920_e149482_d_n6, assign96920_e149482_d_n7, assign96920_e149482_d_n8, assign96920_e149482_d_n9, assign96920_e149482_d_n10, assign96920_e149482_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign96920_e149482;
        locals.var_pzbdswg_dn0 = assign96920_e149482_d_n0;
        locals.var_pzbdswg_dn2 = assign96920_e149482_d_n2;
        locals.var_pzbdswg_dn4 = assign96920_e149482_d_n4;
        locals.var_pzbdswg_dn5 = assign96920_e149482_d_n5;
        locals.var_pzbdswg_dn6 = assign96920_e149482_d_n6;
        locals.var_pzbdswg_dn7 = assign96920_e149482_d_n7;
        locals.var_pzbdswg_dn8 = assign96920_e149482_d_n8;
        locals.var_pzbdswg_dn9 = assign96920_e149482_d_n9;
        locals.var_pzbdswg_dn10 = assign96920_e149482_d_n10;
        locals.var_pzbdswg_dn13 = assign96920_e149482_d_n13;

        let (assign96930_e149494, assign96930_e149494_d_n0, assign96930_e149494_d_n2, assign96930_e149494_d_n4, assign96930_e149494_d_n5, assign96930_e149494_d_n6, assign96930_e149494_d_n7, assign96930_e149494_d_n8, assign96930_e149494_d_n9, assign96930_e149494_d_n10, assign96930_e149494_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96930_e149486: f64 = (p.p523 * p.p14);
        let assign96930_e149490: f64 = (p.p482 * locals.var_tdiff);
        let assign96930_e149491: f64 = (1.0 + assign96930_e149490);
        let assign96930_e149492: f64 = (assign96930_e149486 * assign96930_e149491);
        (assign96930_e149492, (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn0)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn2)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn4)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn5)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn6)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn7)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn8)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn9)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn10)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign96930_e149494;
        locals.var_czbs_dn0 = assign96930_e149494_d_n0;
        locals.var_czbs_dn2 = assign96930_e149494_d_n2;
        locals.var_czbs_dn4 = assign96930_e149494_d_n4;
        locals.var_czbs_dn5 = assign96930_e149494_d_n5;
        locals.var_czbs_dn6 = assign96930_e149494_d_n6;
        locals.var_czbs_dn7 = assign96930_e149494_d_n7;
        locals.var_czbs_dn8 = assign96930_e149494_d_n8;
        locals.var_czbs_dn9 = assign96930_e149494_d_n9;
        locals.var_czbs_dn10 = assign96930_e149494_d_n10;
        locals.var_czbs_dn13 = assign96930_e149494_d_n13;

        let assign96940_e149497: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96940_e149497;

        let (assign96950_e149513, assign96950_e149513_d_n0, assign96950_e149513_d_n2, assign96950_e149513_d_n4, assign96950_e149513_d_n5, assign96950_e149513_d_n6, assign96950_e149513_d_n7, assign96950_e149513_d_n8, assign96950_e149513_d_n9, assign96950_e149513_d_n10, assign96950_e149513_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 != 0.0)) {
        let assign96950_e149504: f64 = (p.p16 - locals.var_weff_nf);
        let assign96950_e149505: f64 = (p.p524 * assign96950_e149504);
        let assign96950_e149509: f64 = (p.p484 * locals.var_tdiff);
        let assign96950_e149510: f64 = (1.0 + assign96950_e149509);
        let assign96950_e149511: f64 = (assign96950_e149505 * assign96950_e149510);
        (assign96950_e149511, (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn0)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn2)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn4)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn5)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn6)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn7)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn8)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn9)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn10)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign96950_e149513;
        locals.var_czbssw_dn0 = assign96950_e149513_d_n0;
        locals.var_czbssw_dn2 = assign96950_e149513_d_n2;
        locals.var_czbssw_dn4 = assign96950_e149513_d_n4;
        locals.var_czbssw_dn5 = assign96950_e149513_d_n5;
        locals.var_czbssw_dn6 = assign96950_e149513_d_n6;
        locals.var_czbssw_dn7 = assign96950_e149513_d_n7;
        locals.var_czbssw_dn8 = assign96950_e149513_d_n8;
        locals.var_czbssw_dn9 = assign96950_e149513_d_n9;
        locals.var_czbssw_dn10 = assign96950_e149513_d_n10;
        locals.var_czbssw_dn13 = assign96950_e149513_d_n13;

        let (assign96960_e149527, assign96960_e149527_d_n0, assign96960_e149527_d_n2, assign96960_e149527_d_n4, assign96960_e149527_d_n5, assign96960_e149527_d_n6, assign96960_e149527_d_n7, assign96960_e149527_d_n8, assign96960_e149527_d_n9, assign96960_e149527_d_n10, assign96960_e149527_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 != 0.0)) {
        let assign96960_e149519: f64 = (p.p525 * locals.var_weff_nf);
        let assign96960_e149523: f64 = (p.p486 * locals.var_tdiff);
        let assign96960_e149524: f64 = (1.0 + assign96960_e149523);
        let assign96960_e149525: f64 = (assign96960_e149519 * assign96960_e149524);
        (assign96960_e149525, (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn0)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn2)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn4)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn5)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn6)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn7)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn8)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn9)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn10)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign96960_e149527;
        locals.var_czbsswg_dn0 = assign96960_e149527_d_n0;
        locals.var_czbsswg_dn2 = assign96960_e149527_d_n2;
        locals.var_czbsswg_dn4 = assign96960_e149527_d_n4;
        locals.var_czbsswg_dn5 = assign96960_e149527_d_n5;
        locals.var_czbsswg_dn6 = assign96960_e149527_d_n6;
        locals.var_czbsswg_dn7 = assign96960_e149527_d_n7;
        locals.var_czbsswg_dn8 = assign96960_e149527_d_n8;
        locals.var_czbsswg_dn9 = assign96960_e149527_d_n9;
        locals.var_czbsswg_dn10 = assign96960_e149527_d_n10;
        locals.var_czbsswg_dn13 = assign96960_e149527_d_n13;

        let (assign96970_e149534, assign96970_e149534_d_n0, assign96970_e149534_d_n2, assign96970_e149534_d_n4, assign96970_e149534_d_n5, assign96970_e149534_d_n6, assign96970_e149534_d_n7, assign96970_e149534_d_n8, assign96970_e149534_d_n9, assign96970_e149534_d_n10, assign96970_e149534_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign96970_e149534;
        locals.var_czbssw_dn0 = assign96970_e149534_d_n0;
        locals.var_czbssw_dn2 = assign96970_e149534_d_n2;
        locals.var_czbssw_dn4 = assign96970_e149534_d_n4;
        locals.var_czbssw_dn5 = assign96970_e149534_d_n5;
        locals.var_czbssw_dn6 = assign96970_e149534_d_n6;
        locals.var_czbssw_dn7 = assign96970_e149534_d_n7;
        locals.var_czbssw_dn8 = assign96970_e149534_d_n8;
        locals.var_czbssw_dn9 = assign96970_e149534_d_n9;
        locals.var_czbssw_dn10 = assign96970_e149534_d_n10;
        locals.var_czbssw_dn13 = assign96970_e149534_d_n13;

        let (assign96980_e149549, assign96980_e149549_d_n0, assign96980_e149549_d_n2, assign96980_e149549_d_n4, assign96980_e149549_d_n5, assign96980_e149549_d_n6, assign96980_e149549_d_n7, assign96980_e149549_d_n8, assign96980_e149549_d_n9, assign96980_e149549_d_n10, assign96980_e149549_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 == 0.0)) {
        let assign96980_e149541: f64 = (p.p525 * p.p16);
        let assign96980_e149545: f64 = (p.p486 * locals.var_tdiff);
        let assign96980_e149546: f64 = (1.0 + assign96980_e149545);
        let assign96980_e149547: f64 = (assign96980_e149541 * assign96980_e149546);
        (assign96980_e149547, (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn0)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn2)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn4)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn5)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn6)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn7)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn8)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn9)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn10)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign96980_e149549;
        locals.var_czbsswg_dn0 = assign96980_e149549_d_n0;
        locals.var_czbsswg_dn2 = assign96980_e149549_d_n2;
        locals.var_czbsswg_dn4 = assign96980_e149549_d_n4;
        locals.var_czbsswg_dn5 = assign96980_e149549_d_n5;
        locals.var_czbsswg_dn6 = assign96980_e149549_d_n6;
        locals.var_czbsswg_dn7 = assign96980_e149549_d_n7;
        locals.var_czbsswg_dn8 = assign96980_e149549_d_n8;
        locals.var_czbsswg_dn9 = assign96980_e149549_d_n9;
        locals.var_czbsswg_dn10 = assign96980_e149549_d_n10;
        locals.var_czbsswg_dn13 = assign96980_e149549_d_n13;

        let assign96990_e149552: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96990_e149552;

        let (assign97000_e149558, assign97000_e149558_d_n0, assign97000_e149558_d_n2, assign97000_e149558_d_n4, assign97000_e149558_d_n5, assign97000_e149558_d_n6, assign97000_e149558_d_n7, assign97000_e149558_d_n8, assign97000_e149558_d_n9, assign97000_e149558_d_n10, assign97000_e149558_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign97000_e149558;
        locals.var_czbs_dn0 = assign97000_e149558_d_n0;
        locals.var_czbs_dn2 = assign97000_e149558_d_n2;
        locals.var_czbs_dn4 = assign97000_e149558_d_n4;
        locals.var_czbs_dn5 = assign97000_e149558_d_n5;
        locals.var_czbs_dn6 = assign97000_e149558_d_n6;
        locals.var_czbs_dn7 = assign97000_e149558_d_n7;
        locals.var_czbs_dn8 = assign97000_e149558_d_n8;
        locals.var_czbs_dn9 = assign97000_e149558_d_n9;
        locals.var_czbs_dn10 = assign97000_e149558_d_n10;
        locals.var_czbs_dn13 = assign97000_e149558_d_n13;

        let assign97010_e149561: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign97010_e149561;

        let (assign97020_e149567, assign97020_e149567_d_n0, assign97020_e149567_d_n2, assign97020_e149567_d_n4, assign97020_e149567_d_n5, assign97020_e149567_d_n6, assign97020_e149567_d_n7, assign97020_e149567_d_n8, assign97020_e149567_d_n9, assign97020_e149567_d_n10, assign97020_e149567_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2249 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign97020_e149567;
        locals.var_czbssw_dn0 = assign97020_e149567_d_n0;
        locals.var_czbssw_dn2 = assign97020_e149567_d_n2;
        locals.var_czbssw_dn4 = assign97020_e149567_d_n4;
        locals.var_czbssw_dn5 = assign97020_e149567_d_n5;
        locals.var_czbssw_dn6 = assign97020_e149567_d_n6;
        locals.var_czbssw_dn7 = assign97020_e149567_d_n7;
        locals.var_czbssw_dn8 = assign97020_e149567_d_n8;
        locals.var_czbssw_dn9 = assign97020_e149567_d_n9;
        locals.var_czbssw_dn10 = assign97020_e149567_d_n10;
        locals.var_czbssw_dn13 = assign97020_e149567_d_n13;

        let assign97030_e149570: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign97030_e149570;

        let (assign97040_e149576, assign97040_e149576_d_n0, assign97040_e149576_d_n2, assign97040_e149576_d_n4, assign97040_e149576_d_n5, assign97040_e149576_d_n6, assign97040_e149576_d_n7, assign97040_e149576_d_n8, assign97040_e149576_d_n9, assign97040_e149576_d_n10, assign97040_e149576_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign97040_e149576;
        locals.var_czbsswg_dn0 = assign97040_e149576_d_n0;
        locals.var_czbsswg_dn2 = assign97040_e149576_d_n2;
        locals.var_czbsswg_dn4 = assign97040_e149576_d_n4;
        locals.var_czbsswg_dn5 = assign97040_e149576_d_n5;
        locals.var_czbsswg_dn6 = assign97040_e149576_d_n6;
        locals.var_czbsswg_dn7 = assign97040_e149576_d_n7;
        locals.var_czbsswg_dn8 = assign97040_e149576_d_n8;
        locals.var_czbsswg_dn9 = assign97040_e149576_d_n9;
        locals.var_czbsswg_dn10 = assign97040_e149576_d_n10;
        locals.var_czbsswg_dn13 = assign97040_e149576_d_n13;

        let (assign97050_e149584, assign97050_e149584_d_n0, assign97050_e149584_d_n2, assign97050_e149584_d_n4, assign97050_e149584_d_n5, assign97050_e149584_d_n6, assign97050_e149584_d_n7, assign97050_e149584_d_n8, assign97050_e149584_d_n9, assign97050_e149584_d_n10, assign97050_e149584_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97050_e149581: f64 = (p.p488 * locals.var_tdiff);
        let assign97050_e149582: f64 = (p.p529 - assign97050_e149581);
        (assign97050_e149582, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign97050_e149584;
        locals.var_pzbs_dn0 = assign97050_e149584_d_n0;
        locals.var_pzbs_dn2 = assign97050_e149584_d_n2;
        locals.var_pzbs_dn4 = assign97050_e149584_d_n4;
        locals.var_pzbs_dn5 = assign97050_e149584_d_n5;
        locals.var_pzbs_dn6 = assign97050_e149584_d_n6;
        locals.var_pzbs_dn7 = assign97050_e149584_d_n7;
        locals.var_pzbs_dn8 = assign97050_e149584_d_n8;
        locals.var_pzbs_dn9 = assign97050_e149584_d_n9;
        locals.var_pzbs_dn10 = assign97050_e149584_d_n10;
        locals.var_pzbs_dn13 = assign97050_e149584_d_n13;

        let (assign97060_e149592, assign97060_e149592_d_n0, assign97060_e149592_d_n2, assign97060_e149592_d_n4, assign97060_e149592_d_n5, assign97060_e149592_d_n6, assign97060_e149592_d_n7, assign97060_e149592_d_n8, assign97060_e149592_d_n9, assign97060_e149592_d_n10, assign97060_e149592_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97060_e149589: f64 = (p.p490 * locals.var_tdiff);
        let assign97060_e149590: f64 = (p.p530 - assign97060_e149589);
        (assign97060_e149590, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign97060_e149592;
        locals.var_pzbssw_dn0 = assign97060_e149592_d_n0;
        locals.var_pzbssw_dn2 = assign97060_e149592_d_n2;
        locals.var_pzbssw_dn4 = assign97060_e149592_d_n4;
        locals.var_pzbssw_dn5 = assign97060_e149592_d_n5;
        locals.var_pzbssw_dn6 = assign97060_e149592_d_n6;
        locals.var_pzbssw_dn7 = assign97060_e149592_d_n7;
        locals.var_pzbssw_dn8 = assign97060_e149592_d_n8;
        locals.var_pzbssw_dn9 = assign97060_e149592_d_n9;
        locals.var_pzbssw_dn10 = assign97060_e149592_d_n10;
        locals.var_pzbssw_dn13 = assign97060_e149592_d_n13;

        let (assign97070_e149600, assign97070_e149600_d_n0, assign97070_e149600_d_n2, assign97070_e149600_d_n4, assign97070_e149600_d_n5, assign97070_e149600_d_n6, assign97070_e149600_d_n7, assign97070_e149600_d_n8, assign97070_e149600_d_n9, assign97070_e149600_d_n10, assign97070_e149600_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97070_e149597: f64 = (p.p492 * locals.var_tdiff);
        let assign97070_e149598: f64 = (p.p531 - assign97070_e149597);
        (assign97070_e149598, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign97070_e149600;
        locals.var_pzbsswg_dn0 = assign97070_e149600_d_n0;
        locals.var_pzbsswg_dn2 = assign97070_e149600_d_n2;
        locals.var_pzbsswg_dn4 = assign97070_e149600_d_n4;
        locals.var_pzbsswg_dn5 = assign97070_e149600_d_n5;
        locals.var_pzbsswg_dn6 = assign97070_e149600_d_n6;
        locals.var_pzbsswg_dn7 = assign97070_e149600_d_n7;
        locals.var_pzbsswg_dn8 = assign97070_e149600_d_n8;
        locals.var_pzbsswg_dn9 = assign97070_e149600_d_n9;
        locals.var_pzbsswg_dn10 = assign97070_e149600_d_n10;
        locals.var_pzbsswg_dn13 = assign97070_e149600_d_n13;

        let assign97080_e149607: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign97080_e149607;

        let (assign97090_e149613, assign97090_e149613_d_n0, assign97090_e149613_d_n2, assign97090_e149613_d_n4, assign97090_e149613_d_n5, assign97090_e149613_d_n6, assign97090_e149613_d_n7, assign97090_e149613_d_n8, assign97090_e149613_d_n9, assign97090_e149613_d_n10, assign97090_e149613_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2251 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign97090_e149613;
        locals.var_pzbs_dn0 = assign97090_e149613_d_n0;
        locals.var_pzbs_dn2 = assign97090_e149613_d_n2;
        locals.var_pzbs_dn4 = assign97090_e149613_d_n4;
        locals.var_pzbs_dn5 = assign97090_e149613_d_n5;
        locals.var_pzbs_dn6 = assign97090_e149613_d_n6;
        locals.var_pzbs_dn7 = assign97090_e149613_d_n7;
        locals.var_pzbs_dn8 = assign97090_e149613_d_n8;
        locals.var_pzbs_dn9 = assign97090_e149613_d_n9;
        locals.var_pzbs_dn10 = assign97090_e149613_d_n10;
        locals.var_pzbs_dn13 = assign97090_e149613_d_n13;

        let assign97100_e149620: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97100_e149620;

        let (assign97110_e149626, assign97110_e149626_d_n0, assign97110_e149626_d_n2, assign97110_e149626_d_n4, assign97110_e149626_d_n5, assign97110_e149626_d_n6, assign97110_e149626_d_n7, assign97110_e149626_d_n8, assign97110_e149626_d_n9, assign97110_e149626_d_n10, assign97110_e149626_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign97110_e149626;
        locals.var_pzbssw_dn0 = assign97110_e149626_d_n0;
        locals.var_pzbssw_dn2 = assign97110_e149626_d_n2;
        locals.var_pzbssw_dn4 = assign97110_e149626_d_n4;
        locals.var_pzbssw_dn5 = assign97110_e149626_d_n5;
        locals.var_pzbssw_dn6 = assign97110_e149626_d_n6;
        locals.var_pzbssw_dn7 = assign97110_e149626_d_n7;
        locals.var_pzbssw_dn8 = assign97110_e149626_d_n8;
        locals.var_pzbssw_dn9 = assign97110_e149626_d_n9;
        locals.var_pzbssw_dn10 = assign97110_e149626_d_n10;
        locals.var_pzbssw_dn13 = assign97110_e149626_d_n13;

        let assign97120_e149633: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97120_e149633;

        let (assign97130_e149639, assign97130_e149639_d_n0, assign97130_e149639_d_n2, assign97130_e149639_d_n4, assign97130_e149639_d_n5, assign97130_e149639_d_n6, assign97130_e149639_d_n7, assign97130_e149639_d_n8, assign97130_e149639_d_n9, assign97130_e149639_d_n10, assign97130_e149639_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign97130_e149639;
        locals.var_pzbsswg_dn0 = assign97130_e149639_d_n0;
        locals.var_pzbsswg_dn2 = assign97130_e149639_d_n2;
        locals.var_pzbsswg_dn4 = assign97130_e149639_d_n4;
        locals.var_pzbsswg_dn5 = assign97130_e149639_d_n5;
        locals.var_pzbsswg_dn6 = assign97130_e149639_d_n6;
        locals.var_pzbsswg_dn7 = assign97130_e149639_d_n7;
        locals.var_pzbsswg_dn8 = assign97130_e149639_d_n8;
        locals.var_pzbsswg_dn9 = assign97130_e149639_d_n9;
        locals.var_pzbsswg_dn10 = assign97130_e149639_d_n10;
        locals.var_pzbsswg_dn13 = assign97130_e149639_d_n13;

        let (assign97140_e149646, assign97140_e149646_d_n0, assign97140_e149646_d_n2, assign97140_e149646_d_n4, assign97140_e149646_d_n5, assign97140_e149646_d_n6, assign97140_e149646_d_n7, assign97140_e149646_d_n8, assign97140_e149646_d_n9, assign97140_e149646_d_n10, assign97140_e149646_d_n13,) = {
    if (locals.var_guard2233 == 0.0) {
        let assign97140_e149642: f64 = ctx_temp;
        let assign97140_e149644: f64 = (assign97140_e149642 + p.p11);
        (assign97140_e149644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign97140_e149646;
        locals.var_ttemp_dn0 = assign97140_e149646_d_n0;
        locals.var_ttemp_dn2 = assign97140_e149646_d_n2;
        locals.var_ttemp_dn4 = assign97140_e149646_d_n4;
        locals.var_ttemp_dn5 = assign97140_e149646_d_n5;
        locals.var_ttemp_dn6 = assign97140_e149646_d_n6;
        locals.var_ttemp_dn7 = assign97140_e149646_d_n7;
        locals.var_ttemp_dn8 = assign97140_e149646_d_n8;
        locals.var_ttemp_dn9 = assign97140_e149646_d_n9;
        locals.var_ttemp_dn10 = assign97140_e149646_d_n10;
        locals.var_ttemp_dn13 = assign97140_e149646_d_n13;

    }

    pub(super) fn stamp_transient_block_346(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign97150_e149649: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97150_e149649;
        locals.var_t10_dn0 = (p.p511 * locals.var_jd_nvtm_invd_dn0);
        locals.var_t10_dn2 = (p.p511 * locals.var_jd_nvtm_invd_dn2);
        locals.var_t10_dn4 = (p.p511 * locals.var_jd_nvtm_invd_dn4);
        locals.var_t10_dn5 = (p.p511 * locals.var_jd_nvtm_invd_dn5);
        locals.var_t10_dn6 = (p.p511 * locals.var_jd_nvtm_invd_dn6);
        locals.var_t10_dn7 = (p.p511 * locals.var_jd_nvtm_invd_dn7);
        locals.var_t10_dn8 = (p.p511 * locals.var_jd_nvtm_invd_dn8);
        locals.var_t10_dn9 = (p.p511 * locals.var_jd_nvtm_invd_dn9);
        locals.var_t10_dn10 = (p.p511 * locals.var_jd_nvtm_invd_dn10);
        locals.var_t10_dn13 = (p.p511 * locals.var_jd_nvtm_invd_dn13);

        let assign97160_e149652: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97160_e149652;
        locals.var_t9_dn0 = (p.p510 * locals.var_exptempd_dn0);
        locals.var_t9_dn2 = (p.p510 * locals.var_exptempd_dn2);
        locals.var_t9_dn4 = (p.p510 * locals.var_exptempd_dn4);
        locals.var_t9_dn5 = (p.p510 * locals.var_exptempd_dn5);
        locals.var_t9_dn6 = (p.p510 * locals.var_exptempd_dn6);
        locals.var_t9_dn7 = (p.p510 * locals.var_exptempd_dn7);
        locals.var_t9_dn8 = (p.p510 * locals.var_exptempd_dn8);
        locals.var_t9_dn9 = (p.p510 * locals.var_exptempd_dn9);
        locals.var_t9_dn10 = (p.p510 * locals.var_exptempd_dn10);
        locals.var_t9_dn13 = (p.p510 * locals.var_exptempd_dn13);

        let assign97170_e149655: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97170_e149655;

        let (assign97180_e149661, assign97180_e149661_d_n0, assign97180_e149661_d_n2, assign97180_e149661_d_n4, assign97180_e149661_d_n5, assign97180_e149661_d_n6, assign97180_e149661_d_n7, assign97180_e149661_d_n8, assign97180_e149661_d_n9, assign97180_e149661_d_n10, assign97180_e149661_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97180_e149659: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97180_e149659, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn13 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97180_e149661;
        locals.var_t0_dn0 = assign97180_e149661_d_n0;
        locals.var_t0_dn2 = assign97180_e149661_d_n2;
        locals.var_t0_dn4 = assign97180_e149661_d_n4;
        locals.var_t0_dn5 = assign97180_e149661_d_n5;
        locals.var_t0_dn6 = assign97180_e149661_d_n6;
        locals.var_t0_dn7 = assign97180_e149661_d_n7;
        locals.var_t0_dn8 = assign97180_e149661_d_n8;
        locals.var_t0_dn9 = assign97180_e149661_d_n9;
        locals.var_t0_dn10 = assign97180_e149661_d_n10;
        locals.var_t0_dn13 = assign97180_e149661_d_n13;

        let (assign97190_e149668, assign97190_e149668_d_n0, assign97190_e149668_d_n2, assign97190_e149668_d_n4, assign97190_e149668_d_n5, assign97190_e149668_d_n6, assign97190_e149668_d_n7, assign97190_e149668_d_n8, assign97190_e149668_d_n9, assign97190_e149668_d_n10, assign97190_e149668_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97190_e149664: f64 = (-locals.var_vbd_jct);
        let assign97190_e149666: f64 = (assign97190_e149664 * locals.var_t10);
        (assign97190_e149666, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97190_e149664 * locals.var_t10_dn0)), (assign97190_e149664 * locals.var_t10_dn2), (assign97190_e149664 * locals.var_t10_dn4), (assign97190_e149664 * locals.var_t10_dn5), (assign97190_e149664 * locals.var_t10_dn6), (assign97190_e149664 * locals.var_t10_dn7), (assign97190_e149664 * locals.var_t10_dn8), (((-locals.var_vbd_jct_dn9) * locals.var_t10) + (assign97190_e149664 * locals.var_t10_dn9)), (assign97190_e149664 * locals.var_t10_dn10), (assign97190_e149664 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97190_e149668;
        locals.var_tx_dn0 = assign97190_e149668_d_n0;
        locals.var_tx_dn2 = assign97190_e149668_d_n2;
        locals.var_tx_dn4 = assign97190_e149668_d_n4;
        locals.var_tx_dn5 = assign97190_e149668_d_n5;
        locals.var_tx_dn6 = assign97190_e149668_d_n6;
        locals.var_tx_dn7 = assign97190_e149668_d_n7;
        locals.var_tx_dn8 = assign97190_e149668_d_n8;
        locals.var_tx_dn9 = assign97190_e149668_d_n9;
        locals.var_tx_dn10 = assign97190_e149668_d_n10;
        locals.var_tx_dn13 = assign97190_e149668_d_n13;

        let (assign97200_e149673, assign97200_e149673_d_n0, assign97200_e149673_d_n2, assign97200_e149673_d_n4, assign97200_e149673_d_n5, assign97200_e149673_d_n6, assign97200_e149673_d_n7, assign97200_e149673_d_n8, assign97200_e149673_d_n9, assign97200_e149673_d_n10, assign97200_e149673_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97200_e149671: f64 = (locals.var_tx).exp();
        (assign97200_e149671, (assign97200_e149671 * locals.var_tx_dn0), (assign97200_e149671 * locals.var_tx_dn2), (assign97200_e149671 * locals.var_tx_dn4), (assign97200_e149671 * locals.var_tx_dn5), (assign97200_e149671 * locals.var_tx_dn6), (assign97200_e149671 * locals.var_tx_dn7), (assign97200_e149671 * locals.var_tx_dn8), (assign97200_e149671 * locals.var_tx_dn9), (assign97200_e149671 * locals.var_tx_dn10), (assign97200_e149671 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97200_e149673;
        locals.var_t2_dn0 = assign97200_e149673_d_n0;
        locals.var_t2_dn2 = assign97200_e149673_d_n2;
        locals.var_t2_dn4 = assign97200_e149673_d_n4;
        locals.var_t2_dn5 = assign97200_e149673_d_n5;
        locals.var_t2_dn6 = assign97200_e149673_d_n6;
        locals.var_t2_dn7 = assign97200_e149673_d_n7;
        locals.var_t2_dn8 = assign97200_e149673_d_n8;
        locals.var_t2_dn9 = assign97200_e149673_d_n9;
        locals.var_t2_dn10 = assign97200_e149673_d_n10;
        locals.var_t2_dn13 = assign97200_e149673_d_n13;

        let (assign97210_e149677, assign97210_e149677_d_n0, assign97210_e149677_d_n2, assign97210_e149677_d_n4, assign97210_e149677_d_n5, assign97210_e149677_d_n6, assign97210_e149677_d_n7, assign97210_e149677_d_n8, assign97210_e149677_d_n9, assign97210_e149677_d_n10, assign97210_e149677_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97210_e149677;
        locals.var_t3_dn0 = assign97210_e149677_d_n0;
        locals.var_t3_dn2 = assign97210_e149677_d_n2;
        locals.var_t3_dn4 = assign97210_e149677_d_n4;
        locals.var_t3_dn5 = assign97210_e149677_d_n5;
        locals.var_t3_dn6 = assign97210_e149677_d_n6;
        locals.var_t3_dn7 = assign97210_e149677_d_n7;
        locals.var_t3_dn8 = assign97210_e149677_d_n8;
        locals.var_t3_dn9 = assign97210_e149677_d_n9;
        locals.var_t3_dn10 = assign97210_e149677_d_n10;
        locals.var_t3_dn13 = assign97210_e149677_d_n13;

        let assign97220_e149680: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97220_e149680;

        let (assign97230_e149688, assign97230_e149688_d_n0, assign97230_e149688_d_n2, assign97230_e149688_d_n4, assign97230_e149688_d_n5, assign97230_e149688_d_n6, assign97230_e149688_d_n7, assign97230_e149688_d_n8, assign97230_e149688_d_n9, assign97230_e149688_d_n10, assign97230_e149688_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) {
        let assign97230_e149686: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97230_e149686, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97230_e149688;
        locals.var_tx_dn0 = assign97230_e149688_d_n0;
        locals.var_tx_dn2 = assign97230_e149688_d_n2;
        locals.var_tx_dn4 = assign97230_e149688_d_n4;
        locals.var_tx_dn5 = assign97230_e149688_d_n5;
        locals.var_tx_dn6 = assign97230_e149688_d_n6;
        locals.var_tx_dn7 = assign97230_e149688_d_n7;
        locals.var_tx_dn8 = assign97230_e149688_d_n8;
        locals.var_tx_dn9 = assign97230_e149688_d_n9;
        locals.var_tx_dn10 = assign97230_e149688_d_n10;
        locals.var_tx_dn13 = assign97230_e149688_d_n13;

        let assign97240_e149691: f64 = (-3.0);
        let assign97240_e149693: f64 = (assign97240_e149691 * 34.0);
        let assign97240_e149694: f64 = if locals.var_tx < assign97240_e149693 { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97240_e149694;

        let (assign97250_e149702, assign97250_e149702_d_n0, assign97250_e149702_d_n2, assign97250_e149702_d_n4, assign97250_e149702_d_n5, assign97250_e149702_d_n6, assign97250_e149702_d_n7, assign97250_e149702_d_n8, assign97250_e149702_d_n9, assign97250_e149702_d_n10, assign97250_e149702_d_n13,) = {
    if (((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) && (locals.var_guard2256 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97250_e149702;
        locals.var_t1_dn0 = assign97250_e149702_d_n0;
        locals.var_t1_dn2 = assign97250_e149702_d_n2;
        locals.var_t1_dn4 = assign97250_e149702_d_n4;
        locals.var_t1_dn5 = assign97250_e149702_d_n5;
        locals.var_t1_dn6 = assign97250_e149702_d_n6;
        locals.var_t1_dn7 = assign97250_e149702_d_n7;
        locals.var_t1_dn8 = assign97250_e149702_d_n8;
        locals.var_t1_dn9 = assign97250_e149702_d_n9;
        locals.var_t1_dn10 = assign97250_e149702_d_n10;
        locals.var_t1_dn13 = assign97250_e149702_d_n13;

        let (assign97260_e149712, assign97260_e149712_d_n0, assign97260_e149712_d_n2, assign97260_e149712_d_n4, assign97260_e149712_d_n5, assign97260_e149712_d_n6, assign97260_e149712_d_n7, assign97260_e149712_d_n8, assign97260_e149712_d_n9, assign97260_e149712_d_n10, assign97260_e149712_d_n13,) = {
    if (((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) && (locals.var_guard2256 == 0.0)) {
        let assign97260_e149710: f64 = (locals.var_tx).exp();
        (assign97260_e149710, (assign97260_e149710 * locals.var_tx_dn0), (assign97260_e149710 * locals.var_tx_dn2), (assign97260_e149710 * locals.var_tx_dn4), (assign97260_e149710 * locals.var_tx_dn5), (assign97260_e149710 * locals.var_tx_dn6), (assign97260_e149710 * locals.var_tx_dn7), (assign97260_e149710 * locals.var_tx_dn8), (assign97260_e149710 * locals.var_tx_dn9), (assign97260_e149710 * locals.var_tx_dn10), (assign97260_e149710 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97260_e149712;
        locals.var_t1_dn0 = assign97260_e149712_d_n0;
        locals.var_t1_dn2 = assign97260_e149712_d_n2;
        locals.var_t1_dn4 = assign97260_e149712_d_n4;
        locals.var_t1_dn5 = assign97260_e149712_d_n5;
        locals.var_t1_dn6 = assign97260_e149712_d_n6;
        locals.var_t1_dn7 = assign97260_e149712_d_n7;
        locals.var_t1_dn8 = assign97260_e149712_d_n8;
        locals.var_t1_dn9 = assign97260_e149712_d_n9;
        locals.var_t1_dn10 = assign97260_e149712_d_n10;
        locals.var_t1_dn13 = assign97260_e149712_d_n13;

        let (assign97270_e149734, assign97270_e149734_d_n0, assign97270_e149734_d_n2, assign97270_e149734_d_n4, assign97270_e149734_d_n5, assign97270_e149734_d_n6, assign97270_e149734_d_n7, assign97270_e149734_d_n8, assign97270_e149734_d_n9, assign97270_e149734_d_n10, assign97270_e149734_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) {
        let assign97270_e149719: f64 = (locals.var_t1 - 1.0);
        let assign97270_e149720: f64 = (locals.var_isbd_btm * assign97270_e149719);
        let assign97270_e149724: f64 = (locals.var_t2 - 1.0);
        let assign97270_e149725: f64 = (locals.var_t0 * assign97270_e149724);
        let assign97270_e149726: f64 = (assign97270_e149720 + assign97270_e149725);
        let assign97270_e149730: f64 = (locals.var_t3 - 1.0);
        let assign97270_e149731: f64 = (locals.var_uc_cisbkd * assign97270_e149730);
        let assign97270_e149732: f64 = (assign97270_e149726 + assign97270_e149731);
        (assign97270_e149732, ((((locals.var_isbd_btm_dn0 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_btm_dn2 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_btm_dn4 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_btm_dn5 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_btm_dn6 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_btm_dn7 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_btm_dn8 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_btm_dn9 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_btm_dn10 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_btm_dn13 * assign97270_e149719) + (locals.var_isbd_btm * locals.var_t1_dn13)) + ((locals.var_t0_dn13 * assign97270_e149724) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbkd * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn13,)
    }
};
        locals.var_ibd_btm = assign97270_e149734;
        locals.var_ibd_btm_dn0 = assign97270_e149734_d_n0;
        locals.var_ibd_btm_dn2 = assign97270_e149734_d_n2;
        locals.var_ibd_btm_dn4 = assign97270_e149734_d_n4;
        locals.var_ibd_btm_dn5 = assign97270_e149734_d_n5;
        locals.var_ibd_btm_dn6 = assign97270_e149734_d_n6;
        locals.var_ibd_btm_dn7 = assign97270_e149734_d_n7;
        locals.var_ibd_btm_dn8 = assign97270_e149734_d_n8;
        locals.var_ibd_btm_dn9 = assign97270_e149734_d_n9;
        locals.var_ibd_btm_dn10 = assign97270_e149734_d_n10;
        locals.var_ibd_btm_dn13 = assign97270_e149734_d_n13;

        let (assign97280_e149741, assign97280_e149741_d_n0, assign97280_e149741_d_n2, assign97280_e149741_d_n4, assign97280_e149741_d_n5, assign97280_e149741_d_n6, assign97280_e149741_d_n7, assign97280_e149741_d_n8, assign97280_e149741_d_n9, assign97280_e149741_d_n10, assign97280_e149741_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97280_e149741;
        locals.var_t1_dn0 = assign97280_e149741_d_n0;
        locals.var_t1_dn2 = assign97280_e149741_d_n2;
        locals.var_t1_dn4 = assign97280_e149741_d_n4;
        locals.var_t1_dn5 = assign97280_e149741_d_n5;
        locals.var_t1_dn6 = assign97280_e149741_d_n6;
        locals.var_t1_dn7 = assign97280_e149741_d_n7;
        locals.var_t1_dn8 = assign97280_e149741_d_n8;
        locals.var_t1_dn9 = assign97280_e149741_d_n9;
        locals.var_t1_dn10 = assign97280_e149741_d_n10;
        locals.var_t1_dn13 = assign97280_e149741_d_n13;

        let (assign97290_e149752, assign97290_e149752_d_n0, assign97290_e149752_d_n2, assign97290_e149752_d_n4, assign97290_e149752_d_n5, assign97290_e149752_d_n6, assign97290_e149752_d_n7, assign97290_e149752_d_n8, assign97290_e149752_d_n9, assign97290_e149752_d_n10, assign97290_e149752_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 == 0.0)) {
        let assign97290_e149748: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97290_e149750: f64 = (assign97290_e149748 * locals.var_t1);
        (assign97290_e149750, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97290_e149752;
        locals.var_t4_dn0 = assign97290_e149752_d_n0;
        locals.var_t4_dn2 = assign97290_e149752_d_n2;
        locals.var_t4_dn4 = assign97290_e149752_d_n4;
        locals.var_t4_dn5 = assign97290_e149752_d_n5;
        locals.var_t4_dn6 = assign97290_e149752_d_n6;
        locals.var_t4_dn7 = assign97290_e149752_d_n7;
        locals.var_t4_dn8 = assign97290_e149752_d_n8;
        locals.var_t4_dn9 = assign97290_e149752_d_n9;
        locals.var_t4_dn10 = assign97290_e149752_d_n10;
        locals.var_t4_dn13 = assign97290_e149752_d_n13;

        let (assign97300_e149781, assign97300_e149781_d_n0, assign97300_e149781_d_n2, assign97300_e149781_d_n4, assign97300_e149781_d_n5, assign97300_e149781_d_n6, assign97300_e149781_d_n7, assign97300_e149781_d_n8, assign97300_e149781_d_n9, assign97300_e149781_d_n10, assign97300_e149781_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 == 0.0)) {
        let assign97300_e149760: f64 = (locals.var_t1 - 1.0);
        let assign97300_e149761: f64 = (locals.var_isbd_btm * assign97300_e149760);
        let assign97300_e149765: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97300_e149766: f64 = (locals.var_t4 * assign97300_e149765);
        let assign97300_e149767: f64 = (assign97300_e149761 + assign97300_e149766);
        let assign97300_e149771: f64 = (locals.var_t2 - 1.0);
        let assign97300_e149772: f64 = (locals.var_t0 * assign97300_e149771);
        let assign97300_e149773: f64 = (assign97300_e149767 + assign97300_e149772);
        let assign97300_e149777: f64 = (locals.var_t3 - 1.0);
        let assign97300_e149778: f64 = (locals.var_uc_cisbkd * assign97300_e149777);
        let assign97300_e149779: f64 = (assign97300_e149773 + assign97300_e149778);
        (assign97300_e149779, (((((locals.var_isbd_btm_dn0 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97300_e149765) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_btm_dn2 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_btm_dn4 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_btm_dn5 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_btm_dn6 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_btm_dn7 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_btm_dn8 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_btm_dn9 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97300_e149765) + (locals.var_t4 * (locals.var_vbd_jct_dn9 - locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_btm_dn10 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_btm_dn13 * assign97300_e149760) + (locals.var_isbd_btm * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * assign97300_e149765) + (locals.var_t4 * (-locals.var_vbdt_dn13)))) + ((locals.var_t0_dn13 * assign97300_e149771) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbkd * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn13,)
    }
};
        locals.var_ibd_btm = assign97300_e149781;
        locals.var_ibd_btm_dn0 = assign97300_e149781_d_n0;
        locals.var_ibd_btm_dn2 = assign97300_e149781_d_n2;
        locals.var_ibd_btm_dn4 = assign97300_e149781_d_n4;
        locals.var_ibd_btm_dn5 = assign97300_e149781_d_n5;
        locals.var_ibd_btm_dn6 = assign97300_e149781_d_n6;
        locals.var_ibd_btm_dn7 = assign97300_e149781_d_n7;
        locals.var_ibd_btm_dn8 = assign97300_e149781_d_n8;
        locals.var_ibd_btm_dn9 = assign97300_e149781_d_n9;
        locals.var_ibd_btm_dn10 = assign97300_e149781_d_n10;
        locals.var_ibd_btm_dn13 = assign97300_e149781_d_n13;

        let (assign97310_e149786, assign97310_e149786_d_n0, assign97310_e149786_d_n2, assign97310_e149786_d_n4, assign97310_e149786_d_n5, assign97310_e149786_d_n6, assign97310_e149786_d_n7, assign97310_e149786_d_n8, assign97310_e149786_d_n9, assign97310_e149786_d_n10, assign97310_e149786_d_n13,) = {
    if (locals.var_guard2254 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_btm, locals.var_ibd_btm_dn0, locals.var_ibd_btm_dn2, locals.var_ibd_btm_dn4, locals.var_ibd_btm_dn5, locals.var_ibd_btm_dn6, locals.var_ibd_btm_dn7, locals.var_ibd_btm_dn8, locals.var_ibd_btm_dn9, locals.var_ibd_btm_dn10, locals.var_ibd_btm_dn13,)
    }
};
        locals.var_ibd_btm = assign97310_e149786;
        locals.var_ibd_btm_dn0 = assign97310_e149786_d_n0;
        locals.var_ibd_btm_dn2 = assign97310_e149786_d_n2;
        locals.var_ibd_btm_dn4 = assign97310_e149786_d_n4;
        locals.var_ibd_btm_dn5 = assign97310_e149786_d_n5;
        locals.var_ibd_btm_dn6 = assign97310_e149786_d_n6;
        locals.var_ibd_btm_dn7 = assign97310_e149786_d_n7;
        locals.var_ibd_btm_dn8 = assign97310_e149786_d_n8;
        locals.var_ibd_btm_dn9 = assign97310_e149786_d_n9;
        locals.var_ibd_btm_dn10 = assign97310_e149786_d_n10;
        locals.var_ibd_btm_dn13 = assign97310_e149786_d_n13;

        let assign97320_e149789: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97320_e149789;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_btm_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_btm_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_btm_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_btm_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_btm_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_btm_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_btm_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_btm_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_btm_dn10);
        locals.var_t12_dn13 = (p.p514 * locals.var_isbd2_btm_dn13);

        let assign97330_e149793: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97330_e149794: f64 = (locals.var_ibd_btm + assign97330_e149793);
        locals.var_ibd_btm = assign97330_e149794;
        locals.var_ibd_btm_dn0 = (locals.var_ibd_btm_dn0 + ((locals.var_t12_dn0 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn0)));
        locals.var_ibd_btm_dn2 = (locals.var_ibd_btm_dn2 + (locals.var_t12_dn2 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn4 = (locals.var_ibd_btm_dn4 + (locals.var_t12_dn4 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn5 = (locals.var_ibd_btm_dn5 + (locals.var_t12_dn5 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn6 = (locals.var_ibd_btm_dn6 + (locals.var_t12_dn6 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn7 = (locals.var_ibd_btm_dn7 + (locals.var_t12_dn7 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn8 = (locals.var_ibd_btm_dn8 + (locals.var_t12_dn8 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn9 = (locals.var_ibd_btm_dn9 + ((locals.var_t12_dn9 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn9)));
        locals.var_ibd_btm_dn10 = (locals.var_ibd_btm_dn10 + (locals.var_t12_dn10 * locals.var_vbd_jct));
        locals.var_ibd_btm_dn13 = (locals.var_ibd_btm_dn13 + (locals.var_t12_dn13 * locals.var_vbd_jct));

        let assign97340_e149797: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97340_e149797;

        let (assign97350_e149803, assign97350_e149803_d_n0, assign97350_e149803_d_n2, assign97350_e149803_d_n4, assign97350_e149803_d_n5, assign97350_e149803_d_n6, assign97350_e149803_d_n7, assign97350_e149803_d_n8, assign97350_e149803_d_n9, assign97350_e149803_d_n10, assign97350_e149803_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97350_e149801: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97350_e149801, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn13 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97350_e149803;
        locals.var_t0_dn0 = assign97350_e149803_d_n0;
        locals.var_t0_dn2 = assign97350_e149803_d_n2;
        locals.var_t0_dn4 = assign97350_e149803_d_n4;
        locals.var_t0_dn5 = assign97350_e149803_d_n5;
        locals.var_t0_dn6 = assign97350_e149803_d_n6;
        locals.var_t0_dn7 = assign97350_e149803_d_n7;
        locals.var_t0_dn8 = assign97350_e149803_d_n8;
        locals.var_t0_dn9 = assign97350_e149803_d_n9;
        locals.var_t0_dn10 = assign97350_e149803_d_n10;
        locals.var_t0_dn13 = assign97350_e149803_d_n13;

        let (assign97360_e149810, assign97360_e149810_d_n0, assign97360_e149810_d_n2, assign97360_e149810_d_n4, assign97360_e149810_d_n5, assign97360_e149810_d_n6, assign97360_e149810_d_n7, assign97360_e149810_d_n8, assign97360_e149810_d_n9, assign97360_e149810_d_n10, assign97360_e149810_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97360_e149806: f64 = (-locals.var_vbd_jct);
        let assign97360_e149808: f64 = (assign97360_e149806 * locals.var_t10);
        (assign97360_e149808, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97360_e149806 * locals.var_t10_dn0)), (assign97360_e149806 * locals.var_t10_dn2), (assign97360_e149806 * locals.var_t10_dn4), (assign97360_e149806 * locals.var_t10_dn5), (assign97360_e149806 * locals.var_t10_dn6), (assign97360_e149806 * locals.var_t10_dn7), (assign97360_e149806 * locals.var_t10_dn8), (((-locals.var_vbd_jct_dn9) * locals.var_t10) + (assign97360_e149806 * locals.var_t10_dn9)), (assign97360_e149806 * locals.var_t10_dn10), (assign97360_e149806 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97360_e149810;
        locals.var_tx_dn0 = assign97360_e149810_d_n0;
        locals.var_tx_dn2 = assign97360_e149810_d_n2;
        locals.var_tx_dn4 = assign97360_e149810_d_n4;
        locals.var_tx_dn5 = assign97360_e149810_d_n5;
        locals.var_tx_dn6 = assign97360_e149810_d_n6;
        locals.var_tx_dn7 = assign97360_e149810_d_n7;
        locals.var_tx_dn8 = assign97360_e149810_d_n8;
        locals.var_tx_dn9 = assign97360_e149810_d_n9;
        locals.var_tx_dn10 = assign97360_e149810_d_n10;
        locals.var_tx_dn13 = assign97360_e149810_d_n13;

        let (assign97370_e149815, assign97370_e149815_d_n0, assign97370_e149815_d_n2, assign97370_e149815_d_n4, assign97370_e149815_d_n5, assign97370_e149815_d_n6, assign97370_e149815_d_n7, assign97370_e149815_d_n8, assign97370_e149815_d_n9, assign97370_e149815_d_n10, assign97370_e149815_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97370_e149813: f64 = (locals.var_tx).exp();
        (assign97370_e149813, (assign97370_e149813 * locals.var_tx_dn0), (assign97370_e149813 * locals.var_tx_dn2), (assign97370_e149813 * locals.var_tx_dn4), (assign97370_e149813 * locals.var_tx_dn5), (assign97370_e149813 * locals.var_tx_dn6), (assign97370_e149813 * locals.var_tx_dn7), (assign97370_e149813 * locals.var_tx_dn8), (assign97370_e149813 * locals.var_tx_dn9), (assign97370_e149813 * locals.var_tx_dn10), (assign97370_e149813 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97370_e149815;
        locals.var_t2_dn0 = assign97370_e149815_d_n0;
        locals.var_t2_dn2 = assign97370_e149815_d_n2;
        locals.var_t2_dn4 = assign97370_e149815_d_n4;
        locals.var_t2_dn5 = assign97370_e149815_d_n5;
        locals.var_t2_dn6 = assign97370_e149815_d_n6;
        locals.var_t2_dn7 = assign97370_e149815_d_n7;
        locals.var_t2_dn8 = assign97370_e149815_d_n8;
        locals.var_t2_dn9 = assign97370_e149815_d_n9;
        locals.var_t2_dn10 = assign97370_e149815_d_n10;
        locals.var_t2_dn13 = assign97370_e149815_d_n13;

        let (assign97380_e149819, assign97380_e149819_d_n0, assign97380_e149819_d_n2, assign97380_e149819_d_n4, assign97380_e149819_d_n5, assign97380_e149819_d_n6, assign97380_e149819_d_n7, assign97380_e149819_d_n8, assign97380_e149819_d_n9, assign97380_e149819_d_n10, assign97380_e149819_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97380_e149819;
        locals.var_t3_dn0 = assign97380_e149819_d_n0;
        locals.var_t3_dn2 = assign97380_e149819_d_n2;
        locals.var_t3_dn4 = assign97380_e149819_d_n4;
        locals.var_t3_dn5 = assign97380_e149819_d_n5;
        locals.var_t3_dn6 = assign97380_e149819_d_n6;
        locals.var_t3_dn7 = assign97380_e149819_d_n7;
        locals.var_t3_dn8 = assign97380_e149819_d_n8;
        locals.var_t3_dn9 = assign97380_e149819_d_n9;
        locals.var_t3_dn10 = assign97380_e149819_d_n10;
        locals.var_t3_dn13 = assign97380_e149819_d_n13;

        let assign97390_e149822: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97390_e149822;

        let (assign97400_e149830, assign97400_e149830_d_n0, assign97400_e149830_d_n2, assign97400_e149830_d_n4, assign97400_e149830_d_n5, assign97400_e149830_d_n6, assign97400_e149830_d_n7, assign97400_e149830_d_n8, assign97400_e149830_d_n9, assign97400_e149830_d_n10, assign97400_e149830_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) {
        let assign97400_e149828: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97400_e149828, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97400_e149830;
        locals.var_tx_dn0 = assign97400_e149830_d_n0;
        locals.var_tx_dn2 = assign97400_e149830_d_n2;
        locals.var_tx_dn4 = assign97400_e149830_d_n4;
        locals.var_tx_dn5 = assign97400_e149830_d_n5;
        locals.var_tx_dn6 = assign97400_e149830_d_n6;
        locals.var_tx_dn7 = assign97400_e149830_d_n7;
        locals.var_tx_dn8 = assign97400_e149830_d_n8;
        locals.var_tx_dn9 = assign97400_e149830_d_n9;
        locals.var_tx_dn10 = assign97400_e149830_d_n10;
        locals.var_tx_dn13 = assign97400_e149830_d_n13;

        let assign97410_e149833: f64 = (-3.0);
        let assign97410_e149835: f64 = (assign97410_e149833 * 34.0);
        let assign97410_e149836: f64 = if locals.var_tx < assign97410_e149835 { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97410_e149836;

        let (assign97420_e149844, assign97420_e149844_d_n0, assign97420_e149844_d_n2, assign97420_e149844_d_n4, assign97420_e149844_d_n5, assign97420_e149844_d_n6, assign97420_e149844_d_n7, assign97420_e149844_d_n8, assign97420_e149844_d_n9, assign97420_e149844_d_n10, assign97420_e149844_d_n13,) = {
    if (((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) && (locals.var_guard2259 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97420_e149844;
        locals.var_t1_dn0 = assign97420_e149844_d_n0;
        locals.var_t1_dn2 = assign97420_e149844_d_n2;
        locals.var_t1_dn4 = assign97420_e149844_d_n4;
        locals.var_t1_dn5 = assign97420_e149844_d_n5;
        locals.var_t1_dn6 = assign97420_e149844_d_n6;
        locals.var_t1_dn7 = assign97420_e149844_d_n7;
        locals.var_t1_dn8 = assign97420_e149844_d_n8;
        locals.var_t1_dn9 = assign97420_e149844_d_n9;
        locals.var_t1_dn10 = assign97420_e149844_d_n10;
        locals.var_t1_dn13 = assign97420_e149844_d_n13;

        let (assign97430_e149854, assign97430_e149854_d_n0, assign97430_e149854_d_n2, assign97430_e149854_d_n4, assign97430_e149854_d_n5, assign97430_e149854_d_n6, assign97430_e149854_d_n7, assign97430_e149854_d_n8, assign97430_e149854_d_n9, assign97430_e149854_d_n10, assign97430_e149854_d_n13,) = {
    if (((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) && (locals.var_guard2259 == 0.0)) {
        let assign97430_e149852: f64 = (locals.var_tx).exp();
        (assign97430_e149852, (assign97430_e149852 * locals.var_tx_dn0), (assign97430_e149852 * locals.var_tx_dn2), (assign97430_e149852 * locals.var_tx_dn4), (assign97430_e149852 * locals.var_tx_dn5), (assign97430_e149852 * locals.var_tx_dn6), (assign97430_e149852 * locals.var_tx_dn7), (assign97430_e149852 * locals.var_tx_dn8), (assign97430_e149852 * locals.var_tx_dn9), (assign97430_e149852 * locals.var_tx_dn10), (assign97430_e149852 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97430_e149854;
        locals.var_t1_dn0 = assign97430_e149854_d_n0;
        locals.var_t1_dn2 = assign97430_e149854_d_n2;
        locals.var_t1_dn4 = assign97430_e149854_d_n4;
        locals.var_t1_dn5 = assign97430_e149854_d_n5;
        locals.var_t1_dn6 = assign97430_e149854_d_n6;
        locals.var_t1_dn7 = assign97430_e149854_d_n7;
        locals.var_t1_dn8 = assign97430_e149854_d_n8;
        locals.var_t1_dn9 = assign97430_e149854_d_n9;
        locals.var_t1_dn10 = assign97430_e149854_d_n10;
        locals.var_t1_dn13 = assign97430_e149854_d_n13;

        let (assign97440_e149876, assign97440_e149876_d_n0, assign97440_e149876_d_n2, assign97440_e149876_d_n4, assign97440_e149876_d_n5, assign97440_e149876_d_n6, assign97440_e149876_d_n7, assign97440_e149876_d_n8, assign97440_e149876_d_n9, assign97440_e149876_d_n10, assign97440_e149876_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) {
        let assign97440_e149861: f64 = (locals.var_t1 - 1.0);
        let assign97440_e149862: f64 = (locals.var_isbd_sws * assign97440_e149861);
        let assign97440_e149866: f64 = (locals.var_t2 - 1.0);
        let assign97440_e149867: f64 = (locals.var_t0 * assign97440_e149866);
        let assign97440_e149868: f64 = (assign97440_e149862 + assign97440_e149867);
        let assign97440_e149872: f64 = (locals.var_t3 - 1.0);
        let assign97440_e149873: f64 = (locals.var_uc_cisbkd * assign97440_e149872);
        let assign97440_e149874: f64 = (assign97440_e149868 + assign97440_e149873);
        (assign97440_e149874, ((((locals.var_isbd_sws_dn0 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), ((((locals.var_isbd_sws_dn2 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), ((((locals.var_isbd_sws_dn4 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), ((((locals.var_isbd_sws_dn5 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), ((((locals.var_isbd_sws_dn6 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), ((((locals.var_isbd_sws_dn7 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), ((((locals.var_isbd_sws_dn8 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), ((((locals.var_isbd_sws_dn9 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), ((((locals.var_isbd_sws_dn10 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), ((((locals.var_isbd_sws_dn13 * assign97440_e149861) + (locals.var_isbd_sws * locals.var_t1_dn13)) + ((locals.var_t0_dn13 * assign97440_e149866) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbkd * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn13,)
    }
};
        locals.var_ibd_sws = assign97440_e149876;
        locals.var_ibd_sws_dn0 = assign97440_e149876_d_n0;
        locals.var_ibd_sws_dn2 = assign97440_e149876_d_n2;
        locals.var_ibd_sws_dn4 = assign97440_e149876_d_n4;
        locals.var_ibd_sws_dn5 = assign97440_e149876_d_n5;
        locals.var_ibd_sws_dn6 = assign97440_e149876_d_n6;
        locals.var_ibd_sws_dn7 = assign97440_e149876_d_n7;
        locals.var_ibd_sws_dn8 = assign97440_e149876_d_n8;
        locals.var_ibd_sws_dn9 = assign97440_e149876_d_n9;
        locals.var_ibd_sws_dn10 = assign97440_e149876_d_n10;
        locals.var_ibd_sws_dn13 = assign97440_e149876_d_n13;

    }

    pub(super) fn stamp_transient_block_347(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97450_e149883, assign97450_e149883_d_n0, assign97450_e149883_d_n2, assign97450_e149883_d_n4, assign97450_e149883_d_n5, assign97450_e149883_d_n6, assign97450_e149883_d_n7, assign97450_e149883_d_n8, assign97450_e149883_d_n9, assign97450_e149883_d_n10, assign97450_e149883_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97450_e149883;
        locals.var_t1_dn0 = assign97450_e149883_d_n0;
        locals.var_t1_dn2 = assign97450_e149883_d_n2;
        locals.var_t1_dn4 = assign97450_e149883_d_n4;
        locals.var_t1_dn5 = assign97450_e149883_d_n5;
        locals.var_t1_dn6 = assign97450_e149883_d_n6;
        locals.var_t1_dn7 = assign97450_e149883_d_n7;
        locals.var_t1_dn8 = assign97450_e149883_d_n8;
        locals.var_t1_dn9 = assign97450_e149883_d_n9;
        locals.var_t1_dn10 = assign97450_e149883_d_n10;
        locals.var_t1_dn13 = assign97450_e149883_d_n13;

        let (assign97460_e149894, assign97460_e149894_d_n0, assign97460_e149894_d_n2, assign97460_e149894_d_n4, assign97460_e149894_d_n5, assign97460_e149894_d_n6, assign97460_e149894_d_n7, assign97460_e149894_d_n8, assign97460_e149894_d_n9, assign97460_e149894_d_n10, assign97460_e149894_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 == 0.0)) {
        let assign97460_e149890: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97460_e149892: f64 = (assign97460_e149890 * locals.var_t1);
        (assign97460_e149892, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97460_e149894;
        locals.var_t4_dn0 = assign97460_e149894_d_n0;
        locals.var_t4_dn2 = assign97460_e149894_d_n2;
        locals.var_t4_dn4 = assign97460_e149894_d_n4;
        locals.var_t4_dn5 = assign97460_e149894_d_n5;
        locals.var_t4_dn6 = assign97460_e149894_d_n6;
        locals.var_t4_dn7 = assign97460_e149894_d_n7;
        locals.var_t4_dn8 = assign97460_e149894_d_n8;
        locals.var_t4_dn9 = assign97460_e149894_d_n9;
        locals.var_t4_dn10 = assign97460_e149894_d_n10;
        locals.var_t4_dn13 = assign97460_e149894_d_n13;

        let (assign97470_e149923, assign97470_e149923_d_n0, assign97470_e149923_d_n2, assign97470_e149923_d_n4, assign97470_e149923_d_n5, assign97470_e149923_d_n6, assign97470_e149923_d_n7, assign97470_e149923_d_n8, assign97470_e149923_d_n9, assign97470_e149923_d_n10, assign97470_e149923_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 == 0.0)) {
        let assign97470_e149902: f64 = (locals.var_t1 - 1.0);
        let assign97470_e149903: f64 = (locals.var_isbd_sws * assign97470_e149902);
        let assign97470_e149907: f64 = (locals.var_vbd_jct - locals.var_vbdt);
        let assign97470_e149908: f64 = (locals.var_t4 * assign97470_e149907);
        let assign97470_e149909: f64 = (assign97470_e149903 + assign97470_e149908);
        let assign97470_e149913: f64 = (locals.var_t2 - 1.0);
        let assign97470_e149914: f64 = (locals.var_t0 * assign97470_e149913);
        let assign97470_e149915: f64 = (assign97470_e149909 + assign97470_e149914);
        let assign97470_e149919: f64 = (locals.var_t3 - 1.0);
        let assign97470_e149920: f64 = (locals.var_uc_cisbkd * assign97470_e149919);
        let assign97470_e149921: f64 = (assign97470_e149915 + assign97470_e149920);
        (assign97470_e149921, (((((locals.var_isbd_sws_dn0 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97470_e149907) + (locals.var_t4 * (locals.var_vbd_jct_dn0 - locals.var_vbdt_dn0)))) + ((locals.var_t0_dn0 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbkd * locals.var_t3_dn0)), (((((locals.var_isbd_sws_dn2 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn2)))) + ((locals.var_t0_dn2 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbkd * locals.var_t3_dn2)), (((((locals.var_isbd_sws_dn4 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn4)))) + ((locals.var_t0_dn4 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbkd * locals.var_t3_dn4)), (((((locals.var_isbd_sws_dn5 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn5)))) + ((locals.var_t0_dn5 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbkd * locals.var_t3_dn5)), (((((locals.var_isbd_sws_dn6 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn6)))) + ((locals.var_t0_dn6 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbkd * locals.var_t3_dn6)), (((((locals.var_isbd_sws_dn7 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn7)))) + ((locals.var_t0_dn7 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbkd * locals.var_t3_dn7)), (((((locals.var_isbd_sws_dn8 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn8)))) + ((locals.var_t0_dn8 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbkd * locals.var_t3_dn8)), (((((locals.var_isbd_sws_dn9 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97470_e149907) + (locals.var_t4 * (locals.var_vbd_jct_dn9 - locals.var_vbdt_dn9)))) + ((locals.var_t0_dn9 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbkd * locals.var_t3_dn9)), (((((locals.var_isbd_sws_dn10 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn10)))) + ((locals.var_t0_dn10 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbkd * locals.var_t3_dn10)), (((((locals.var_isbd_sws_dn13 * assign97470_e149902) + (locals.var_isbd_sws * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * assign97470_e149907) + (locals.var_t4 * (-locals.var_vbdt_dn13)))) + ((locals.var_t0_dn13 * assign97470_e149913) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbkd * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn13,)
    }
};
        locals.var_ibd_sws = assign97470_e149923;
        locals.var_ibd_sws_dn0 = assign97470_e149923_d_n0;
        locals.var_ibd_sws_dn2 = assign97470_e149923_d_n2;
        locals.var_ibd_sws_dn4 = assign97470_e149923_d_n4;
        locals.var_ibd_sws_dn5 = assign97470_e149923_d_n5;
        locals.var_ibd_sws_dn6 = assign97470_e149923_d_n6;
        locals.var_ibd_sws_dn7 = assign97470_e149923_d_n7;
        locals.var_ibd_sws_dn8 = assign97470_e149923_d_n8;
        locals.var_ibd_sws_dn9 = assign97470_e149923_d_n9;
        locals.var_ibd_sws_dn10 = assign97470_e149923_d_n10;
        locals.var_ibd_sws_dn13 = assign97470_e149923_d_n13;

        let (assign97480_e149928, assign97480_e149928_d_n0, assign97480_e149928_d_n2, assign97480_e149928_d_n4, assign97480_e149928_d_n5, assign97480_e149928_d_n6, assign97480_e149928_d_n7, assign97480_e149928_d_n8, assign97480_e149928_d_n9, assign97480_e149928_d_n10, assign97480_e149928_d_n13,) = {
    if (locals.var_guard2257 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_sws, locals.var_ibd_sws_dn0, locals.var_ibd_sws_dn2, locals.var_ibd_sws_dn4, locals.var_ibd_sws_dn5, locals.var_ibd_sws_dn6, locals.var_ibd_sws_dn7, locals.var_ibd_sws_dn8, locals.var_ibd_sws_dn9, locals.var_ibd_sws_dn10, locals.var_ibd_sws_dn13,)
    }
};
        locals.var_ibd_sws = assign97480_e149928;
        locals.var_ibd_sws_dn0 = assign97480_e149928_d_n0;
        locals.var_ibd_sws_dn2 = assign97480_e149928_d_n2;
        locals.var_ibd_sws_dn4 = assign97480_e149928_d_n4;
        locals.var_ibd_sws_dn5 = assign97480_e149928_d_n5;
        locals.var_ibd_sws_dn6 = assign97480_e149928_d_n6;
        locals.var_ibd_sws_dn7 = assign97480_e149928_d_n7;
        locals.var_ibd_sws_dn8 = assign97480_e149928_d_n8;
        locals.var_ibd_sws_dn9 = assign97480_e149928_d_n9;
        locals.var_ibd_sws_dn10 = assign97480_e149928_d_n10;
        locals.var_ibd_sws_dn13 = assign97480_e149928_d_n13;

        let assign97490_e149931: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97490_e149931;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_sws_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_sws_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_sws_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_sws_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_sws_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_sws_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_sws_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_sws_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_sws_dn10);
        locals.var_t12_dn13 = (p.p514 * locals.var_isbd2_sws_dn13);

        let assign97500_e149935: f64 = (locals.var_t12 * locals.var_vbd_jct);
        let assign97500_e149936: f64 = (locals.var_ibd_sws + assign97500_e149935);
        locals.var_ibd_sws = assign97500_e149936;
        locals.var_ibd_sws_dn0 = (locals.var_ibd_sws_dn0 + ((locals.var_t12_dn0 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn0)));
        locals.var_ibd_sws_dn2 = (locals.var_ibd_sws_dn2 + (locals.var_t12_dn2 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn4 = (locals.var_ibd_sws_dn4 + (locals.var_t12_dn4 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn5 = (locals.var_ibd_sws_dn5 + (locals.var_t12_dn5 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn6 = (locals.var_ibd_sws_dn6 + (locals.var_t12_dn6 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn7 = (locals.var_ibd_sws_dn7 + (locals.var_t12_dn7 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn8 = (locals.var_ibd_sws_dn8 + (locals.var_t12_dn8 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn9 = (locals.var_ibd_sws_dn9 + ((locals.var_t12_dn9 * locals.var_vbd_jct) + (locals.var_t12 * locals.var_vbd_jct_dn9)));
        locals.var_ibd_sws_dn10 = (locals.var_ibd_sws_dn10 + (locals.var_t12_dn10 * locals.var_vbd_jct));
        locals.var_ibd_sws_dn13 = (locals.var_ibd_sws_dn13 + (locals.var_t12_dn13 * locals.var_vbd_jct));

        let assign97510_e149939: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97510_e149939;

        let assign97520_e149942: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97520_e149942;

        let (assign97530_e149950, assign97530_e149950_d_n0, assign97530_e149950_d_n2, assign97530_e149950_d_n4, assign97530_e149950_d_n5, assign97530_e149950_d_n6, assign97530_e149950_d_n7, assign97530_e149950_d_n8, assign97530_e149950_d_n9, assign97530_e149950_d_n10, assign97530_e149950_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97530_e149948: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97530_e149948, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn13 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97530_e149950;
        locals.var_t0_dn0 = assign97530_e149950_d_n0;
        locals.var_t0_dn2 = assign97530_e149950_d_n2;
        locals.var_t0_dn4 = assign97530_e149950_d_n4;
        locals.var_t0_dn5 = assign97530_e149950_d_n5;
        locals.var_t0_dn6 = assign97530_e149950_d_n6;
        locals.var_t0_dn7 = assign97530_e149950_d_n7;
        locals.var_t0_dn8 = assign97530_e149950_d_n8;
        locals.var_t0_dn9 = assign97530_e149950_d_n9;
        locals.var_t0_dn10 = assign97530_e149950_d_n10;
        locals.var_t0_dn13 = assign97530_e149950_d_n13;

        let (assign97540_e149959, assign97540_e149959_d_n0, assign97540_e149959_d_n2, assign97540_e149959_d_n4, assign97540_e149959_d_n5, assign97540_e149959_d_n6, assign97540_e149959_d_n7, assign97540_e149959_d_n8, assign97540_e149959_d_n9, assign97540_e149959_d_n10, assign97540_e149959_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97540_e149955: f64 = (-locals.var_vbdi_jct);
        let assign97540_e149957: f64 = (assign97540_e149955 * locals.var_t10);
        (assign97540_e149957, (assign97540_e149955 * locals.var_t10_dn0), (assign97540_e149955 * locals.var_t10_dn2), (assign97540_e149955 * locals.var_t10_dn4), (((-locals.var_vbdi_jct_dn5) * locals.var_t10) + (assign97540_e149955 * locals.var_t10_dn5)), (assign97540_e149955 * locals.var_t10_dn6), (assign97540_e149955 * locals.var_t10_dn7), (((-locals.var_vbdi_jct_dn8) * locals.var_t10) + (assign97540_e149955 * locals.var_t10_dn8)), (assign97540_e149955 * locals.var_t10_dn9), (assign97540_e149955 * locals.var_t10_dn10), (assign97540_e149955 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97540_e149959;
        locals.var_tx_dn0 = assign97540_e149959_d_n0;
        locals.var_tx_dn2 = assign97540_e149959_d_n2;
        locals.var_tx_dn4 = assign97540_e149959_d_n4;
        locals.var_tx_dn5 = assign97540_e149959_d_n5;
        locals.var_tx_dn6 = assign97540_e149959_d_n6;
        locals.var_tx_dn7 = assign97540_e149959_d_n7;
        locals.var_tx_dn8 = assign97540_e149959_d_n8;
        locals.var_tx_dn9 = assign97540_e149959_d_n9;
        locals.var_tx_dn10 = assign97540_e149959_d_n10;
        locals.var_tx_dn13 = assign97540_e149959_d_n13;

        let (assign97550_e149966, assign97550_e149966_d_n0, assign97550_e149966_d_n2, assign97550_e149966_d_n4, assign97550_e149966_d_n5, assign97550_e149966_d_n6, assign97550_e149966_d_n7, assign97550_e149966_d_n8, assign97550_e149966_d_n9, assign97550_e149966_d_n10, assign97550_e149966_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97550_e149964: f64 = (locals.var_tx).exp();
        (assign97550_e149964, (assign97550_e149964 * locals.var_tx_dn0), (assign97550_e149964 * locals.var_tx_dn2), (assign97550_e149964 * locals.var_tx_dn4), (assign97550_e149964 * locals.var_tx_dn5), (assign97550_e149964 * locals.var_tx_dn6), (assign97550_e149964 * locals.var_tx_dn7), (assign97550_e149964 * locals.var_tx_dn8), (assign97550_e149964 * locals.var_tx_dn9), (assign97550_e149964 * locals.var_tx_dn10), (assign97550_e149964 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97550_e149966;
        locals.var_t2_dn0 = assign97550_e149966_d_n0;
        locals.var_t2_dn2 = assign97550_e149966_d_n2;
        locals.var_t2_dn4 = assign97550_e149966_d_n4;
        locals.var_t2_dn5 = assign97550_e149966_d_n5;
        locals.var_t2_dn6 = assign97550_e149966_d_n6;
        locals.var_t2_dn7 = assign97550_e149966_d_n7;
        locals.var_t2_dn8 = assign97550_e149966_d_n8;
        locals.var_t2_dn9 = assign97550_e149966_d_n9;
        locals.var_t2_dn10 = assign97550_e149966_d_n10;
        locals.var_t2_dn13 = assign97550_e149966_d_n13;

        let (assign97560_e149972, assign97560_e149972_d_n0, assign97560_e149972_d_n2, assign97560_e149972_d_n4, assign97560_e149972_d_n5, assign97560_e149972_d_n6, assign97560_e149972_d_n7, assign97560_e149972_d_n8, assign97560_e149972_d_n9, assign97560_e149972_d_n10, assign97560_e149972_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97560_e149972;
        locals.var_t3_dn0 = assign97560_e149972_d_n0;
        locals.var_t3_dn2 = assign97560_e149972_d_n2;
        locals.var_t3_dn4 = assign97560_e149972_d_n4;
        locals.var_t3_dn5 = assign97560_e149972_d_n5;
        locals.var_t3_dn6 = assign97560_e149972_d_n6;
        locals.var_t3_dn7 = assign97560_e149972_d_n7;
        locals.var_t3_dn8 = assign97560_e149972_d_n8;
        locals.var_t3_dn9 = assign97560_e149972_d_n9;
        locals.var_t3_dn10 = assign97560_e149972_d_n10;
        locals.var_t3_dn13 = assign97560_e149972_d_n13;

        let assign97570_e149975: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97570_e149975;

        let (assign97580_e149985, assign97580_e149985_d_n0, assign97580_e149985_d_n2, assign97580_e149985_d_n4, assign97580_e149985_d_n5, assign97580_e149985_d_n6, assign97580_e149985_d_n7, assign97580_e149985_d_n8, assign97580_e149985_d_n9, assign97580_e149985_d_n10, assign97580_e149985_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) {
        let assign97580_e149983: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97580_e149983, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), ((locals.var_vbdi_jct_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), ((locals.var_vbdi_jct_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97580_e149985;
        locals.var_tx_dn0 = assign97580_e149985_d_n0;
        locals.var_tx_dn2 = assign97580_e149985_d_n2;
        locals.var_tx_dn4 = assign97580_e149985_d_n4;
        locals.var_tx_dn5 = assign97580_e149985_d_n5;
        locals.var_tx_dn6 = assign97580_e149985_d_n6;
        locals.var_tx_dn7 = assign97580_e149985_d_n7;
        locals.var_tx_dn8 = assign97580_e149985_d_n8;
        locals.var_tx_dn9 = assign97580_e149985_d_n9;
        locals.var_tx_dn10 = assign97580_e149985_d_n10;
        locals.var_tx_dn13 = assign97580_e149985_d_n13;

        let assign97590_e149988: f64 = (-3.0);
        let assign97590_e149990: f64 = (assign97590_e149988 * 34.0);
        let assign97590_e149991: f64 = if locals.var_tx < assign97590_e149990 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97590_e149991;

        let (assign97600_e150001, assign97600_e150001_d_n0, assign97600_e150001_d_n2, assign97600_e150001_d_n4, assign97600_e150001_d_n5, assign97600_e150001_d_n6, assign97600_e150001_d_n7, assign97600_e150001_d_n8, assign97600_e150001_d_n9, assign97600_e150001_d_n10, assign97600_e150001_d_n13,) = {
    if ((((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97600_e150001;
        locals.var_t1_dn0 = assign97600_e150001_d_n0;
        locals.var_t1_dn2 = assign97600_e150001_d_n2;
        locals.var_t1_dn4 = assign97600_e150001_d_n4;
        locals.var_t1_dn5 = assign97600_e150001_d_n5;
        locals.var_t1_dn6 = assign97600_e150001_d_n6;
        locals.var_t1_dn7 = assign97600_e150001_d_n7;
        locals.var_t1_dn8 = assign97600_e150001_d_n8;
        locals.var_t1_dn9 = assign97600_e150001_d_n9;
        locals.var_t1_dn10 = assign97600_e150001_d_n10;
        locals.var_t1_dn13 = assign97600_e150001_d_n13;

        let (assign97610_e150013, assign97610_e150013_d_n0, assign97610_e150013_d_n2, assign97610_e150013_d_n4, assign97610_e150013_d_n5, assign97610_e150013_d_n6, assign97610_e150013_d_n7, assign97610_e150013_d_n8, assign97610_e150013_d_n9, assign97610_e150013_d_n10, assign97610_e150013_d_n13,) = {
    if ((((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 == 0.0)) {
        let assign97610_e150011: f64 = (locals.var_tx).exp();
        (assign97610_e150011, (assign97610_e150011 * locals.var_tx_dn0), (assign97610_e150011 * locals.var_tx_dn2), (assign97610_e150011 * locals.var_tx_dn4), (assign97610_e150011 * locals.var_tx_dn5), (assign97610_e150011 * locals.var_tx_dn6), (assign97610_e150011 * locals.var_tx_dn7), (assign97610_e150011 * locals.var_tx_dn8), (assign97610_e150011 * locals.var_tx_dn9), (assign97610_e150011 * locals.var_tx_dn10), (assign97610_e150011 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97610_e150013;
        locals.var_t1_dn0 = assign97610_e150013_d_n0;
        locals.var_t1_dn2 = assign97610_e150013_d_n2;
        locals.var_t1_dn4 = assign97610_e150013_d_n4;
        locals.var_t1_dn5 = assign97610_e150013_d_n5;
        locals.var_t1_dn6 = assign97610_e150013_d_n6;
        locals.var_t1_dn7 = assign97610_e150013_d_n7;
        locals.var_t1_dn8 = assign97610_e150013_d_n8;
        locals.var_t1_dn9 = assign97610_e150013_d_n9;
        locals.var_t1_dn10 = assign97610_e150013_d_n10;
        locals.var_t1_dn13 = assign97610_e150013_d_n13;

        let (assign97630_e150046, assign97630_e150046_d_n0, assign97630_e150046_d_n2, assign97630_e150046_d_n4, assign97630_e150046_d_n5, assign97630_e150046_d_n6, assign97630_e150046_d_n7, assign97630_e150046_d_n8, assign97630_e150046_d_n9, assign97630_e150046_d_n10, assign97630_e150046_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97630_e150046;
        locals.var_t1_dn0 = assign97630_e150046_d_n0;
        locals.var_t1_dn2 = assign97630_e150046_d_n2;
        locals.var_t1_dn4 = assign97630_e150046_d_n4;
        locals.var_t1_dn5 = assign97630_e150046_d_n5;
        locals.var_t1_dn6 = assign97630_e150046_d_n6;
        locals.var_t1_dn7 = assign97630_e150046_d_n7;
        locals.var_t1_dn8 = assign97630_e150046_d_n8;
        locals.var_t1_dn9 = assign97630_e150046_d_n9;
        locals.var_t1_dn10 = assign97630_e150046_d_n10;
        locals.var_t1_dn13 = assign97630_e150046_d_n13;

        let (assign97640_e150059, assign97640_e150059_d_n0, assign97640_e150059_d_n2, assign97640_e150059_d_n4, assign97640_e150059_d_n5, assign97640_e150059_d_n6, assign97640_e150059_d_n7, assign97640_e150059_d_n8, assign97640_e150059_d_n9, assign97640_e150059_d_n10, assign97640_e150059_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 == 0.0)) {
        let assign97640_e150055: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97640_e150057: f64 = (assign97640_e150055 * locals.var_t1);
        (assign97640_e150057, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97640_e150059;
        locals.var_t4_dn0 = assign97640_e150059_d_n0;
        locals.var_t4_dn2 = assign97640_e150059_d_n2;
        locals.var_t4_dn4 = assign97640_e150059_d_n4;
        locals.var_t4_dn5 = assign97640_e150059_d_n5;
        locals.var_t4_dn6 = assign97640_e150059_d_n6;
        locals.var_t4_dn7 = assign97640_e150059_d_n7;
        locals.var_t4_dn8 = assign97640_e150059_d_n8;
        locals.var_t4_dn9 = assign97640_e150059_d_n9;
        locals.var_t4_dn10 = assign97640_e150059_d_n10;
        locals.var_t4_dn13 = assign97640_e150059_d_n13;

        let (assign97670_e150103, assign97670_e150103_d_n0, assign97670_e150103_d_n2, assign97670_e150103_d_n4, assign97670_e150103_d_n5, assign97670_e150103_d_n6, assign97670_e150103_d_n7, assign97670_e150103_d_n8, assign97670_e150103_d_n9, assign97670_e150103_d_n10, assign97670_e150103_d_n13,) = {
    if (locals.var_guard2260 != 0.0) {
        let assign97670_e150101: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97670_e150101, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn13),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign97670_e150103;
        locals.var_t12_dn0 = assign97670_e150103_d_n0;
        locals.var_t12_dn2 = assign97670_e150103_d_n2;
        locals.var_t12_dn4 = assign97670_e150103_d_n4;
        locals.var_t12_dn5 = assign97670_e150103_d_n5;
        locals.var_t12_dn6 = assign97670_e150103_d_n6;
        locals.var_t12_dn7 = assign97670_e150103_d_n7;
        locals.var_t12_dn8 = assign97670_e150103_d_n8;
        locals.var_t12_dn9 = assign97670_e150103_d_n9;
        locals.var_t12_dn10 = assign97670_e150103_d_n10;
        locals.var_t12_dn13 = assign97670_e150103_d_n13;

        let assign97700_e150119: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97700_e150119;
        locals.var_t10_dn0 = (p.p534 * locals.var_jd_nvtm_invs_dn0);
        locals.var_t10_dn2 = (p.p534 * locals.var_jd_nvtm_invs_dn2);
        locals.var_t10_dn4 = (p.p534 * locals.var_jd_nvtm_invs_dn4);
        locals.var_t10_dn5 = (p.p534 * locals.var_jd_nvtm_invs_dn5);
        locals.var_t10_dn6 = (p.p534 * locals.var_jd_nvtm_invs_dn6);
        locals.var_t10_dn7 = (p.p534 * locals.var_jd_nvtm_invs_dn7);
        locals.var_t10_dn8 = (p.p534 * locals.var_jd_nvtm_invs_dn8);
        locals.var_t10_dn9 = (p.p534 * locals.var_jd_nvtm_invs_dn9);
        locals.var_t10_dn10 = (p.p534 * locals.var_jd_nvtm_invs_dn10);
        locals.var_t10_dn13 = (p.p534 * locals.var_jd_nvtm_invs_dn13);

        let assign97710_e150122: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97710_e150122;
        locals.var_t9_dn0 = (p.p533 * locals.var_exptemps_dn0);
        locals.var_t9_dn2 = (p.p533 * locals.var_exptemps_dn2);
        locals.var_t9_dn4 = (p.p533 * locals.var_exptemps_dn4);
        locals.var_t9_dn5 = (p.p533 * locals.var_exptemps_dn5);
        locals.var_t9_dn6 = (p.p533 * locals.var_exptemps_dn6);
        locals.var_t9_dn7 = (p.p533 * locals.var_exptemps_dn7);
        locals.var_t9_dn8 = (p.p533 * locals.var_exptemps_dn8);
        locals.var_t9_dn9 = (p.p533 * locals.var_exptemps_dn9);
        locals.var_t9_dn10 = (p.p533 * locals.var_exptemps_dn10);
        locals.var_t9_dn13 = (p.p533 * locals.var_exptemps_dn13);

        let assign97720_e150125: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97720_e150125;

        let (assign97730_e150131, assign97730_e150131_d_n0, assign97730_e150131_d_n2, assign97730_e150131_d_n4, assign97730_e150131_d_n5, assign97730_e150131_d_n6, assign97730_e150131_d_n7, assign97730_e150131_d_n8, assign97730_e150131_d_n9, assign97730_e150131_d_n10, assign97730_e150131_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97730_e150129: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97730_e150129, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn13 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97730_e150131;
        locals.var_t0_dn0 = assign97730_e150131_d_n0;
        locals.var_t0_dn2 = assign97730_e150131_d_n2;
        locals.var_t0_dn4 = assign97730_e150131_d_n4;
        locals.var_t0_dn5 = assign97730_e150131_d_n5;
        locals.var_t0_dn6 = assign97730_e150131_d_n6;
        locals.var_t0_dn7 = assign97730_e150131_d_n7;
        locals.var_t0_dn8 = assign97730_e150131_d_n8;
        locals.var_t0_dn9 = assign97730_e150131_d_n9;
        locals.var_t0_dn10 = assign97730_e150131_d_n10;
        locals.var_t0_dn13 = assign97730_e150131_d_n13;

        let (assign97740_e150138, assign97740_e150138_d_n0, assign97740_e150138_d_n2, assign97740_e150138_d_n4, assign97740_e150138_d_n5, assign97740_e150138_d_n6, assign97740_e150138_d_n7, assign97740_e150138_d_n8, assign97740_e150138_d_n9, assign97740_e150138_d_n10, assign97740_e150138_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97740_e150134: f64 = (-locals.var_vbs_jct);
        let assign97740_e150136: f64 = (assign97740_e150134 * locals.var_t10);
        (assign97740_e150136, (assign97740_e150134 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97740_e150134 * locals.var_t10_dn2)), (assign97740_e150134 * locals.var_t10_dn4), (assign97740_e150134 * locals.var_t10_dn5), (assign97740_e150134 * locals.var_t10_dn6), (assign97740_e150134 * locals.var_t10_dn7), (assign97740_e150134 * locals.var_t10_dn8), (assign97740_e150134 * locals.var_t10_dn9), (((-locals.var_vbs_jct_dn10) * locals.var_t10) + (assign97740_e150134 * locals.var_t10_dn10)), (assign97740_e150134 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97740_e150138;
        locals.var_tx_dn0 = assign97740_e150138_d_n0;
        locals.var_tx_dn2 = assign97740_e150138_d_n2;
        locals.var_tx_dn4 = assign97740_e150138_d_n4;
        locals.var_tx_dn5 = assign97740_e150138_d_n5;
        locals.var_tx_dn6 = assign97740_e150138_d_n6;
        locals.var_tx_dn7 = assign97740_e150138_d_n7;
        locals.var_tx_dn8 = assign97740_e150138_d_n8;
        locals.var_tx_dn9 = assign97740_e150138_d_n9;
        locals.var_tx_dn10 = assign97740_e150138_d_n10;
        locals.var_tx_dn13 = assign97740_e150138_d_n13;

        let (assign97750_e150143, assign97750_e150143_d_n0, assign97750_e150143_d_n2, assign97750_e150143_d_n4, assign97750_e150143_d_n5, assign97750_e150143_d_n6, assign97750_e150143_d_n7, assign97750_e150143_d_n8, assign97750_e150143_d_n9, assign97750_e150143_d_n10, assign97750_e150143_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97750_e150141: f64 = (locals.var_tx).exp();
        (assign97750_e150141, (assign97750_e150141 * locals.var_tx_dn0), (assign97750_e150141 * locals.var_tx_dn2), (assign97750_e150141 * locals.var_tx_dn4), (assign97750_e150141 * locals.var_tx_dn5), (assign97750_e150141 * locals.var_tx_dn6), (assign97750_e150141 * locals.var_tx_dn7), (assign97750_e150141 * locals.var_tx_dn8), (assign97750_e150141 * locals.var_tx_dn9), (assign97750_e150141 * locals.var_tx_dn10), (assign97750_e150141 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97750_e150143;
        locals.var_t2_dn0 = assign97750_e150143_d_n0;
        locals.var_t2_dn2 = assign97750_e150143_d_n2;
        locals.var_t2_dn4 = assign97750_e150143_d_n4;
        locals.var_t2_dn5 = assign97750_e150143_d_n5;
        locals.var_t2_dn6 = assign97750_e150143_d_n6;
        locals.var_t2_dn7 = assign97750_e150143_d_n7;
        locals.var_t2_dn8 = assign97750_e150143_d_n8;
        locals.var_t2_dn9 = assign97750_e150143_d_n9;
        locals.var_t2_dn10 = assign97750_e150143_d_n10;
        locals.var_t2_dn13 = assign97750_e150143_d_n13;

        let (assign97760_e150147, assign97760_e150147_d_n0, assign97760_e150147_d_n2, assign97760_e150147_d_n4, assign97760_e150147_d_n5, assign97760_e150147_d_n6, assign97760_e150147_d_n7, assign97760_e150147_d_n8, assign97760_e150147_d_n9, assign97760_e150147_d_n10, assign97760_e150147_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97760_e150147;
        locals.var_t3_dn0 = assign97760_e150147_d_n0;
        locals.var_t3_dn2 = assign97760_e150147_d_n2;
        locals.var_t3_dn4 = assign97760_e150147_d_n4;
        locals.var_t3_dn5 = assign97760_e150147_d_n5;
        locals.var_t3_dn6 = assign97760_e150147_d_n6;
        locals.var_t3_dn7 = assign97760_e150147_d_n7;
        locals.var_t3_dn8 = assign97760_e150147_d_n8;
        locals.var_t3_dn9 = assign97760_e150147_d_n9;
        locals.var_t3_dn10 = assign97760_e150147_d_n10;
        locals.var_t3_dn13 = assign97760_e150147_d_n13;

        let assign97770_e150150: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97770_e150150;

        let (assign97780_e150158, assign97780_e150158_d_n0, assign97780_e150158_d_n2, assign97780_e150158_d_n4, assign97780_e150158_d_n5, assign97780_e150158_d_n6, assign97780_e150158_d_n7, assign97780_e150158_d_n8, assign97780_e150158_d_n9, assign97780_e150158_d_n10, assign97780_e150158_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97780_e150156: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97780_e150156, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), ((locals.var_vbs_jct_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97780_e150158;
        locals.var_tx_dn0 = assign97780_e150158_d_n0;
        locals.var_tx_dn2 = assign97780_e150158_d_n2;
        locals.var_tx_dn4 = assign97780_e150158_d_n4;
        locals.var_tx_dn5 = assign97780_e150158_d_n5;
        locals.var_tx_dn6 = assign97780_e150158_d_n6;
        locals.var_tx_dn7 = assign97780_e150158_d_n7;
        locals.var_tx_dn8 = assign97780_e150158_d_n8;
        locals.var_tx_dn9 = assign97780_e150158_d_n9;
        locals.var_tx_dn10 = assign97780_e150158_d_n10;
        locals.var_tx_dn13 = assign97780_e150158_d_n13;

        let assign97790_e150161: f64 = (-3.0);
        let assign97790_e150163: f64 = (assign97790_e150161 * 34.0);
        let assign97790_e150164: f64 = if locals.var_tx < assign97790_e150163 { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97790_e150164;

        let (assign97800_e150172, assign97800_e150172_d_n0, assign97800_e150172_d_n2, assign97800_e150172_d_n4, assign97800_e150172_d_n5, assign97800_e150172_d_n6, assign97800_e150172_d_n7, assign97800_e150172_d_n8, assign97800_e150172_d_n9, assign97800_e150172_d_n10, assign97800_e150172_d_n13,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97800_e150172;
        locals.var_t1_dn0 = assign97800_e150172_d_n0;
        locals.var_t1_dn2 = assign97800_e150172_d_n2;
        locals.var_t1_dn4 = assign97800_e150172_d_n4;
        locals.var_t1_dn5 = assign97800_e150172_d_n5;
        locals.var_t1_dn6 = assign97800_e150172_d_n6;
        locals.var_t1_dn7 = assign97800_e150172_d_n7;
        locals.var_t1_dn8 = assign97800_e150172_d_n8;
        locals.var_t1_dn9 = assign97800_e150172_d_n9;
        locals.var_t1_dn10 = assign97800_e150172_d_n10;
        locals.var_t1_dn13 = assign97800_e150172_d_n13;

        let (assign97810_e150182, assign97810_e150182_d_n0, assign97810_e150182_d_n2, assign97810_e150182_d_n4, assign97810_e150182_d_n5, assign97810_e150182_d_n6, assign97810_e150182_d_n7, assign97810_e150182_d_n8, assign97810_e150182_d_n9, assign97810_e150182_d_n10, assign97810_e150182_d_n13,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        let assign97810_e150180: f64 = (locals.var_tx).exp();
        (assign97810_e150180, (assign97810_e150180 * locals.var_tx_dn0), (assign97810_e150180 * locals.var_tx_dn2), (assign97810_e150180 * locals.var_tx_dn4), (assign97810_e150180 * locals.var_tx_dn5), (assign97810_e150180 * locals.var_tx_dn6), (assign97810_e150180 * locals.var_tx_dn7), (assign97810_e150180 * locals.var_tx_dn8), (assign97810_e150180 * locals.var_tx_dn9), (assign97810_e150180 * locals.var_tx_dn10), (assign97810_e150180 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97810_e150182;
        locals.var_t1_dn0 = assign97810_e150182_d_n0;
        locals.var_t1_dn2 = assign97810_e150182_d_n2;
        locals.var_t1_dn4 = assign97810_e150182_d_n4;
        locals.var_t1_dn5 = assign97810_e150182_d_n5;
        locals.var_t1_dn6 = assign97810_e150182_d_n6;
        locals.var_t1_dn7 = assign97810_e150182_d_n7;
        locals.var_t1_dn8 = assign97810_e150182_d_n8;
        locals.var_t1_dn9 = assign97810_e150182_d_n9;
        locals.var_t1_dn10 = assign97810_e150182_d_n10;
        locals.var_t1_dn13 = assign97810_e150182_d_n13;

    }

    pub(super) fn stamp_transient_block_348(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97820_e150204, assign97820_e150204_d_n0, assign97820_e150204_d_n2, assign97820_e150204_d_n4, assign97820_e150204_d_n5, assign97820_e150204_d_n6, assign97820_e150204_d_n7, assign97820_e150204_d_n8, assign97820_e150204_d_n9, assign97820_e150204_d_n10, assign97820_e150204_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97820_e150189: f64 = (locals.var_t1 - 1.0);
        let assign97820_e150190: f64 = (locals.var_isbs_btm * assign97820_e150189);
        let assign97820_e150194: f64 = (locals.var_t2 - 1.0);
        let assign97820_e150195: f64 = (locals.var_t0 * assign97820_e150194);
        let assign97820_e150196: f64 = (assign97820_e150190 + assign97820_e150195);
        let assign97820_e150200: f64 = (locals.var_t3 - 1.0);
        let assign97820_e150201: f64 = (locals.var_uc_cisbks * assign97820_e150200);
        let assign97820_e150202: f64 = (assign97820_e150196 + assign97820_e150201);
        (assign97820_e150202, ((((locals.var_isbs_btm_dn0 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_btm_dn2 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_btm_dn4 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_btm_dn5 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_btm_dn6 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_btm_dn7 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_btm_dn8 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_btm_dn9 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_btm_dn10 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_btm_dn13 * assign97820_e150189) + (locals.var_isbs_btm * locals.var_t1_dn13)) + ((locals.var_t0_dn13 * assign97820_e150194) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn13,)
    }
};
        locals.var_ibs_btm = assign97820_e150204;
        locals.var_ibs_btm_dn0 = assign97820_e150204_d_n0;
        locals.var_ibs_btm_dn2 = assign97820_e150204_d_n2;
        locals.var_ibs_btm_dn4 = assign97820_e150204_d_n4;
        locals.var_ibs_btm_dn5 = assign97820_e150204_d_n5;
        locals.var_ibs_btm_dn6 = assign97820_e150204_d_n6;
        locals.var_ibs_btm_dn7 = assign97820_e150204_d_n7;
        locals.var_ibs_btm_dn8 = assign97820_e150204_d_n8;
        locals.var_ibs_btm_dn9 = assign97820_e150204_d_n9;
        locals.var_ibs_btm_dn10 = assign97820_e150204_d_n10;
        locals.var_ibs_btm_dn13 = assign97820_e150204_d_n13;

        let (assign97830_e150211, assign97830_e150211_d_n0, assign97830_e150211_d_n2, assign97830_e150211_d_n4, assign97830_e150211_d_n5, assign97830_e150211_d_n6, assign97830_e150211_d_n7, assign97830_e150211_d_n8, assign97830_e150211_d_n9, assign97830_e150211_d_n10, assign97830_e150211_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97830_e150211;
        locals.var_t1_dn0 = assign97830_e150211_d_n0;
        locals.var_t1_dn2 = assign97830_e150211_d_n2;
        locals.var_t1_dn4 = assign97830_e150211_d_n4;
        locals.var_t1_dn5 = assign97830_e150211_d_n5;
        locals.var_t1_dn6 = assign97830_e150211_d_n6;
        locals.var_t1_dn7 = assign97830_e150211_d_n7;
        locals.var_t1_dn8 = assign97830_e150211_d_n8;
        locals.var_t1_dn9 = assign97830_e150211_d_n9;
        locals.var_t1_dn10 = assign97830_e150211_d_n10;
        locals.var_t1_dn13 = assign97830_e150211_d_n13;

        let (assign97840_e150222, assign97840_e150222_d_n0, assign97840_e150222_d_n2, assign97840_e150222_d_n4, assign97840_e150222_d_n5, assign97840_e150222_d_n6, assign97840_e150222_d_n7, assign97840_e150222_d_n8, assign97840_e150222_d_n9, assign97840_e150222_d_n10, assign97840_e150222_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 == 0.0)) {
        let assign97840_e150218: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97840_e150220: f64 = (assign97840_e150218 * locals.var_t1);
        (assign97840_e150220, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97840_e150222;
        locals.var_t4_dn0 = assign97840_e150222_d_n0;
        locals.var_t4_dn2 = assign97840_e150222_d_n2;
        locals.var_t4_dn4 = assign97840_e150222_d_n4;
        locals.var_t4_dn5 = assign97840_e150222_d_n5;
        locals.var_t4_dn6 = assign97840_e150222_d_n6;
        locals.var_t4_dn7 = assign97840_e150222_d_n7;
        locals.var_t4_dn8 = assign97840_e150222_d_n8;
        locals.var_t4_dn9 = assign97840_e150222_d_n9;
        locals.var_t4_dn10 = assign97840_e150222_d_n10;
        locals.var_t4_dn13 = assign97840_e150222_d_n13;

        let (assign97850_e150251, assign97850_e150251_d_n0, assign97850_e150251_d_n2, assign97850_e150251_d_n4, assign97850_e150251_d_n5, assign97850_e150251_d_n6, assign97850_e150251_d_n7, assign97850_e150251_d_n8, assign97850_e150251_d_n9, assign97850_e150251_d_n10, assign97850_e150251_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 == 0.0)) {
        let assign97850_e150230: f64 = (locals.var_t1 - 1.0);
        let assign97850_e150231: f64 = (locals.var_isbs_btm * assign97850_e150230);
        let assign97850_e150235: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign97850_e150236: f64 = (locals.var_t4 * assign97850_e150235);
        let assign97850_e150237: f64 = (assign97850_e150231 + assign97850_e150236);
        let assign97850_e150241: f64 = (locals.var_t2 - 1.0);
        let assign97850_e150242: f64 = (locals.var_t0 * assign97850_e150241);
        let assign97850_e150243: f64 = (assign97850_e150237 + assign97850_e150242);
        let assign97850_e150247: f64 = (locals.var_t3 - 1.0);
        let assign97850_e150248: f64 = (locals.var_uc_cisbks * assign97850_e150247);
        let assign97850_e150249: f64 = (assign97850_e150243 + assign97850_e150248);
        (assign97850_e150249, (((((locals.var_isbs_btm_dn0 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_btm_dn2 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign97850_e150235) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_btm_dn4 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_btm_dn5 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_btm_dn6 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_btm_dn7 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_btm_dn8 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_btm_dn9 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_btm_dn10 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign97850_e150235) + (locals.var_t4 * (locals.var_vbs_jct_dn10 - locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_btm_dn13 * assign97850_e150230) + (locals.var_isbs_btm * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * assign97850_e150235) + (locals.var_t4 * (-locals.var_vbst_dn13)))) + ((locals.var_t0_dn13 * assign97850_e150241) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn13,)
    }
};
        locals.var_ibs_btm = assign97850_e150251;
        locals.var_ibs_btm_dn0 = assign97850_e150251_d_n0;
        locals.var_ibs_btm_dn2 = assign97850_e150251_d_n2;
        locals.var_ibs_btm_dn4 = assign97850_e150251_d_n4;
        locals.var_ibs_btm_dn5 = assign97850_e150251_d_n5;
        locals.var_ibs_btm_dn6 = assign97850_e150251_d_n6;
        locals.var_ibs_btm_dn7 = assign97850_e150251_d_n7;
        locals.var_ibs_btm_dn8 = assign97850_e150251_d_n8;
        locals.var_ibs_btm_dn9 = assign97850_e150251_d_n9;
        locals.var_ibs_btm_dn10 = assign97850_e150251_d_n10;
        locals.var_ibs_btm_dn13 = assign97850_e150251_d_n13;

        let (assign97860_e150256, assign97860_e150256_d_n0, assign97860_e150256_d_n2, assign97860_e150256_d_n4, assign97860_e150256_d_n5, assign97860_e150256_d_n6, assign97860_e150256_d_n7, assign97860_e150256_d_n8, assign97860_e150256_d_n9, assign97860_e150256_d_n10, assign97860_e150256_d_n13,) = {
    if (locals.var_guard2264 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_btm, locals.var_ibs_btm_dn0, locals.var_ibs_btm_dn2, locals.var_ibs_btm_dn4, locals.var_ibs_btm_dn5, locals.var_ibs_btm_dn6, locals.var_ibs_btm_dn7, locals.var_ibs_btm_dn8, locals.var_ibs_btm_dn9, locals.var_ibs_btm_dn10, locals.var_ibs_btm_dn13,)
    }
};
        locals.var_ibs_btm = assign97860_e150256;
        locals.var_ibs_btm_dn0 = assign97860_e150256_d_n0;
        locals.var_ibs_btm_dn2 = assign97860_e150256_d_n2;
        locals.var_ibs_btm_dn4 = assign97860_e150256_d_n4;
        locals.var_ibs_btm_dn5 = assign97860_e150256_d_n5;
        locals.var_ibs_btm_dn6 = assign97860_e150256_d_n6;
        locals.var_ibs_btm_dn7 = assign97860_e150256_d_n7;
        locals.var_ibs_btm_dn8 = assign97860_e150256_d_n8;
        locals.var_ibs_btm_dn9 = assign97860_e150256_d_n9;
        locals.var_ibs_btm_dn10 = assign97860_e150256_d_n10;
        locals.var_ibs_btm_dn13 = assign97860_e150256_d_n13;

        let assign97870_e150259: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97870_e150259;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_btm_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_btm_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_btm_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_btm_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_btm_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_btm_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_btm_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_btm_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_btm_dn10);
        locals.var_t12_dn13 = (p.p537 * locals.var_isbs2_btm_dn13);

        let assign97880_e150263: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign97880_e150264: f64 = (locals.var_ibs_btm + assign97880_e150263);
        locals.var_ibs_btm = assign97880_e150264;
        locals.var_ibs_btm_dn0 = (locals.var_ibs_btm_dn0 + (locals.var_t12_dn0 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn2 = (locals.var_ibs_btm_dn2 + ((locals.var_t12_dn2 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn2)));
        locals.var_ibs_btm_dn4 = (locals.var_ibs_btm_dn4 + (locals.var_t12_dn4 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn5 = (locals.var_ibs_btm_dn5 + (locals.var_t12_dn5 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn6 = (locals.var_ibs_btm_dn6 + (locals.var_t12_dn6 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn7 = (locals.var_ibs_btm_dn7 + (locals.var_t12_dn7 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn8 = (locals.var_ibs_btm_dn8 + (locals.var_t12_dn8 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn9 = (locals.var_ibs_btm_dn9 + (locals.var_t12_dn9 * locals.var_vbs_jct));
        locals.var_ibs_btm_dn10 = (locals.var_ibs_btm_dn10 + ((locals.var_t12_dn10 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn10)));
        locals.var_ibs_btm_dn13 = (locals.var_ibs_btm_dn13 + (locals.var_t12_dn13 * locals.var_vbs_jct));

        let assign97890_e150267: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97890_e150267;

        let (assign97900_e150273, assign97900_e150273_d_n0, assign97900_e150273_d_n2, assign97900_e150273_d_n4, assign97900_e150273_d_n5, assign97900_e150273_d_n6, assign97900_e150273_d_n7, assign97900_e150273_d_n8, assign97900_e150273_d_n9, assign97900_e150273_d_n10, assign97900_e150273_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97900_e150271: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97900_e150271, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn13 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97900_e150273;
        locals.var_t0_dn0 = assign97900_e150273_d_n0;
        locals.var_t0_dn2 = assign97900_e150273_d_n2;
        locals.var_t0_dn4 = assign97900_e150273_d_n4;
        locals.var_t0_dn5 = assign97900_e150273_d_n5;
        locals.var_t0_dn6 = assign97900_e150273_d_n6;
        locals.var_t0_dn7 = assign97900_e150273_d_n7;
        locals.var_t0_dn8 = assign97900_e150273_d_n8;
        locals.var_t0_dn9 = assign97900_e150273_d_n9;
        locals.var_t0_dn10 = assign97900_e150273_d_n10;
        locals.var_t0_dn13 = assign97900_e150273_d_n13;

        let (assign97910_e150280, assign97910_e150280_d_n0, assign97910_e150280_d_n2, assign97910_e150280_d_n4, assign97910_e150280_d_n5, assign97910_e150280_d_n6, assign97910_e150280_d_n7, assign97910_e150280_d_n8, assign97910_e150280_d_n9, assign97910_e150280_d_n10, assign97910_e150280_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97910_e150276: f64 = (-locals.var_vbs_jct);
        let assign97910_e150278: f64 = (assign97910_e150276 * locals.var_t10);
        (assign97910_e150278, (assign97910_e150276 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97910_e150276 * locals.var_t10_dn2)), (assign97910_e150276 * locals.var_t10_dn4), (assign97910_e150276 * locals.var_t10_dn5), (assign97910_e150276 * locals.var_t10_dn6), (assign97910_e150276 * locals.var_t10_dn7), (assign97910_e150276 * locals.var_t10_dn8), (assign97910_e150276 * locals.var_t10_dn9), (((-locals.var_vbs_jct_dn10) * locals.var_t10) + (assign97910_e150276 * locals.var_t10_dn10)), (assign97910_e150276 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97910_e150280;
        locals.var_tx_dn0 = assign97910_e150280_d_n0;
        locals.var_tx_dn2 = assign97910_e150280_d_n2;
        locals.var_tx_dn4 = assign97910_e150280_d_n4;
        locals.var_tx_dn5 = assign97910_e150280_d_n5;
        locals.var_tx_dn6 = assign97910_e150280_d_n6;
        locals.var_tx_dn7 = assign97910_e150280_d_n7;
        locals.var_tx_dn8 = assign97910_e150280_d_n8;
        locals.var_tx_dn9 = assign97910_e150280_d_n9;
        locals.var_tx_dn10 = assign97910_e150280_d_n10;
        locals.var_tx_dn13 = assign97910_e150280_d_n13;

        let (assign97920_e150285, assign97920_e150285_d_n0, assign97920_e150285_d_n2, assign97920_e150285_d_n4, assign97920_e150285_d_n5, assign97920_e150285_d_n6, assign97920_e150285_d_n7, assign97920_e150285_d_n8, assign97920_e150285_d_n9, assign97920_e150285_d_n10, assign97920_e150285_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97920_e150283: f64 = (locals.var_tx).exp();
        (assign97920_e150283, (assign97920_e150283 * locals.var_tx_dn0), (assign97920_e150283 * locals.var_tx_dn2), (assign97920_e150283 * locals.var_tx_dn4), (assign97920_e150283 * locals.var_tx_dn5), (assign97920_e150283 * locals.var_tx_dn6), (assign97920_e150283 * locals.var_tx_dn7), (assign97920_e150283 * locals.var_tx_dn8), (assign97920_e150283 * locals.var_tx_dn9), (assign97920_e150283 * locals.var_tx_dn10), (assign97920_e150283 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97920_e150285;
        locals.var_t2_dn0 = assign97920_e150285_d_n0;
        locals.var_t2_dn2 = assign97920_e150285_d_n2;
        locals.var_t2_dn4 = assign97920_e150285_d_n4;
        locals.var_t2_dn5 = assign97920_e150285_d_n5;
        locals.var_t2_dn6 = assign97920_e150285_d_n6;
        locals.var_t2_dn7 = assign97920_e150285_d_n7;
        locals.var_t2_dn8 = assign97920_e150285_d_n8;
        locals.var_t2_dn9 = assign97920_e150285_d_n9;
        locals.var_t2_dn10 = assign97920_e150285_d_n10;
        locals.var_t2_dn13 = assign97920_e150285_d_n13;

        let (assign97930_e150289, assign97930_e150289_d_n0, assign97930_e150289_d_n2, assign97930_e150289_d_n4, assign97930_e150289_d_n5, assign97930_e150289_d_n6, assign97930_e150289_d_n7, assign97930_e150289_d_n8, assign97930_e150289_d_n9, assign97930_e150289_d_n10, assign97930_e150289_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97930_e150289;
        locals.var_t3_dn0 = assign97930_e150289_d_n0;
        locals.var_t3_dn2 = assign97930_e150289_d_n2;
        locals.var_t3_dn4 = assign97930_e150289_d_n4;
        locals.var_t3_dn5 = assign97930_e150289_d_n5;
        locals.var_t3_dn6 = assign97930_e150289_d_n6;
        locals.var_t3_dn7 = assign97930_e150289_d_n7;
        locals.var_t3_dn8 = assign97930_e150289_d_n8;
        locals.var_t3_dn9 = assign97930_e150289_d_n9;
        locals.var_t3_dn10 = assign97930_e150289_d_n10;
        locals.var_t3_dn13 = assign97930_e150289_d_n13;

        let assign97940_e150292: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97940_e150292;

        let (assign97950_e150300, assign97950_e150300_d_n0, assign97950_e150300_d_n2, assign97950_e150300_d_n4, assign97950_e150300_d_n5, assign97950_e150300_d_n6, assign97950_e150300_d_n7, assign97950_e150300_d_n8, assign97950_e150300_d_n9, assign97950_e150300_d_n10, assign97950_e150300_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) {
        let assign97950_e150298: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97950_e150298, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), ((locals.var_vbs_jct_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97950_e150300;
        locals.var_tx_dn0 = assign97950_e150300_d_n0;
        locals.var_tx_dn2 = assign97950_e150300_d_n2;
        locals.var_tx_dn4 = assign97950_e150300_d_n4;
        locals.var_tx_dn5 = assign97950_e150300_d_n5;
        locals.var_tx_dn6 = assign97950_e150300_d_n6;
        locals.var_tx_dn7 = assign97950_e150300_d_n7;
        locals.var_tx_dn8 = assign97950_e150300_d_n8;
        locals.var_tx_dn9 = assign97950_e150300_d_n9;
        locals.var_tx_dn10 = assign97950_e150300_d_n10;
        locals.var_tx_dn13 = assign97950_e150300_d_n13;

        let assign97960_e150303: f64 = (-3.0);
        let assign97960_e150305: f64 = (assign97960_e150303 * 34.0);
        let assign97960_e150306: f64 = if locals.var_tx < assign97960_e150305 { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97960_e150306;

        let (assign97970_e150314, assign97970_e150314_d_n0, assign97970_e150314_d_n2, assign97970_e150314_d_n4, assign97970_e150314_d_n5, assign97970_e150314_d_n6, assign97970_e150314_d_n7, assign97970_e150314_d_n8, assign97970_e150314_d_n9, assign97970_e150314_d_n10, assign97970_e150314_d_n13,) = {
    if (((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) && (locals.var_guard2269 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97970_e150314;
        locals.var_t1_dn0 = assign97970_e150314_d_n0;
        locals.var_t1_dn2 = assign97970_e150314_d_n2;
        locals.var_t1_dn4 = assign97970_e150314_d_n4;
        locals.var_t1_dn5 = assign97970_e150314_d_n5;
        locals.var_t1_dn6 = assign97970_e150314_d_n6;
        locals.var_t1_dn7 = assign97970_e150314_d_n7;
        locals.var_t1_dn8 = assign97970_e150314_d_n8;
        locals.var_t1_dn9 = assign97970_e150314_d_n9;
        locals.var_t1_dn10 = assign97970_e150314_d_n10;
        locals.var_t1_dn13 = assign97970_e150314_d_n13;

        let (assign97980_e150324, assign97980_e150324_d_n0, assign97980_e150324_d_n2, assign97980_e150324_d_n4, assign97980_e150324_d_n5, assign97980_e150324_d_n6, assign97980_e150324_d_n7, assign97980_e150324_d_n8, assign97980_e150324_d_n9, assign97980_e150324_d_n10, assign97980_e150324_d_n13,) = {
    if (((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) && (locals.var_guard2269 == 0.0)) {
        let assign97980_e150322: f64 = (locals.var_tx).exp();
        (assign97980_e150322, (assign97980_e150322 * locals.var_tx_dn0), (assign97980_e150322 * locals.var_tx_dn2), (assign97980_e150322 * locals.var_tx_dn4), (assign97980_e150322 * locals.var_tx_dn5), (assign97980_e150322 * locals.var_tx_dn6), (assign97980_e150322 * locals.var_tx_dn7), (assign97980_e150322 * locals.var_tx_dn8), (assign97980_e150322 * locals.var_tx_dn9), (assign97980_e150322 * locals.var_tx_dn10), (assign97980_e150322 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97980_e150324;
        locals.var_t1_dn0 = assign97980_e150324_d_n0;
        locals.var_t1_dn2 = assign97980_e150324_d_n2;
        locals.var_t1_dn4 = assign97980_e150324_d_n4;
        locals.var_t1_dn5 = assign97980_e150324_d_n5;
        locals.var_t1_dn6 = assign97980_e150324_d_n6;
        locals.var_t1_dn7 = assign97980_e150324_d_n7;
        locals.var_t1_dn8 = assign97980_e150324_d_n8;
        locals.var_t1_dn9 = assign97980_e150324_d_n9;
        locals.var_t1_dn10 = assign97980_e150324_d_n10;
        locals.var_t1_dn13 = assign97980_e150324_d_n13;

        let (assign97990_e150346, assign97990_e150346_d_n0, assign97990_e150346_d_n2, assign97990_e150346_d_n4, assign97990_e150346_d_n5, assign97990_e150346_d_n6, assign97990_e150346_d_n7, assign97990_e150346_d_n8, assign97990_e150346_d_n9, assign97990_e150346_d_n10, assign97990_e150346_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) {
        let assign97990_e150331: f64 = (locals.var_t1 - 1.0);
        let assign97990_e150332: f64 = (locals.var_isbs_sws * assign97990_e150331);
        let assign97990_e150336: f64 = (locals.var_t2 - 1.0);
        let assign97990_e150337: f64 = (locals.var_t0 * assign97990_e150336);
        let assign97990_e150338: f64 = (assign97990_e150332 + assign97990_e150337);
        let assign97990_e150342: f64 = (locals.var_t3 - 1.0);
        let assign97990_e150343: f64 = (locals.var_uc_cisbks * assign97990_e150342);
        let assign97990_e150344: f64 = (assign97990_e150338 + assign97990_e150343);
        (assign97990_e150344, ((((locals.var_isbs_sws_dn0 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_sws_dn2 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_sws_dn4 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_sws_dn5 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_sws_dn6 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_sws_dn7 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_sws_dn8 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_sws_dn9 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_sws_dn10 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_sws_dn13 * assign97990_e150331) + (locals.var_isbs_sws * locals.var_t1_dn13)) + ((locals.var_t0_dn13 * assign97990_e150336) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn13,)
    }
};
        locals.var_ibs_sws = assign97990_e150346;
        locals.var_ibs_sws_dn0 = assign97990_e150346_d_n0;
        locals.var_ibs_sws_dn2 = assign97990_e150346_d_n2;
        locals.var_ibs_sws_dn4 = assign97990_e150346_d_n4;
        locals.var_ibs_sws_dn5 = assign97990_e150346_d_n5;
        locals.var_ibs_sws_dn6 = assign97990_e150346_d_n6;
        locals.var_ibs_sws_dn7 = assign97990_e150346_d_n7;
        locals.var_ibs_sws_dn8 = assign97990_e150346_d_n8;
        locals.var_ibs_sws_dn9 = assign97990_e150346_d_n9;
        locals.var_ibs_sws_dn10 = assign97990_e150346_d_n10;
        locals.var_ibs_sws_dn13 = assign97990_e150346_d_n13;

        let (assign98000_e150353, assign98000_e150353_d_n0, assign98000_e150353_d_n2, assign98000_e150353_d_n4, assign98000_e150353_d_n5, assign98000_e150353_d_n6, assign98000_e150353_d_n7, assign98000_e150353_d_n8, assign98000_e150353_d_n9, assign98000_e150353_d_n10, assign98000_e150353_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98000_e150353;
        locals.var_t1_dn0 = assign98000_e150353_d_n0;
        locals.var_t1_dn2 = assign98000_e150353_d_n2;
        locals.var_t1_dn4 = assign98000_e150353_d_n4;
        locals.var_t1_dn5 = assign98000_e150353_d_n5;
        locals.var_t1_dn6 = assign98000_e150353_d_n6;
        locals.var_t1_dn7 = assign98000_e150353_d_n7;
        locals.var_t1_dn8 = assign98000_e150353_d_n8;
        locals.var_t1_dn9 = assign98000_e150353_d_n9;
        locals.var_t1_dn10 = assign98000_e150353_d_n10;
        locals.var_t1_dn13 = assign98000_e150353_d_n13;

        let (assign98010_e150364, assign98010_e150364_d_n0, assign98010_e150364_d_n2, assign98010_e150364_d_n4, assign98010_e150364_d_n5, assign98010_e150364_d_n6, assign98010_e150364_d_n7, assign98010_e150364_d_n8, assign98010_e150364_d_n9, assign98010_e150364_d_n10, assign98010_e150364_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 == 0.0)) {
        let assign98010_e150360: f64 = (locals.var_isbs_sws * locals.var_jd_nvtm_invs);
        let assign98010_e150362: f64 = (assign98010_e150360 * locals.var_t1);
        (assign98010_e150362, ((((locals.var_isbs_sws_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn0)), ((((locals.var_isbs_sws_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn2)), ((((locals.var_isbs_sws_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn4)), ((((locals.var_isbs_sws_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn5)), ((((locals.var_isbs_sws_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn6)), ((((locals.var_isbs_sws_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn7)), ((((locals.var_isbs_sws_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn8)), ((((locals.var_isbs_sws_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn9)), ((((locals.var_isbs_sws_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn10)), ((((locals.var_isbs_sws_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign98010_e150364;
        locals.var_t4_dn0 = assign98010_e150364_d_n0;
        locals.var_t4_dn2 = assign98010_e150364_d_n2;
        locals.var_t4_dn4 = assign98010_e150364_d_n4;
        locals.var_t4_dn5 = assign98010_e150364_d_n5;
        locals.var_t4_dn6 = assign98010_e150364_d_n6;
        locals.var_t4_dn7 = assign98010_e150364_d_n7;
        locals.var_t4_dn8 = assign98010_e150364_d_n8;
        locals.var_t4_dn9 = assign98010_e150364_d_n9;
        locals.var_t4_dn10 = assign98010_e150364_d_n10;
        locals.var_t4_dn13 = assign98010_e150364_d_n13;

        let (assign98020_e150393, assign98020_e150393_d_n0, assign98020_e150393_d_n2, assign98020_e150393_d_n4, assign98020_e150393_d_n5, assign98020_e150393_d_n6, assign98020_e150393_d_n7, assign98020_e150393_d_n8, assign98020_e150393_d_n9, assign98020_e150393_d_n10, assign98020_e150393_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 == 0.0)) {
        let assign98020_e150372: f64 = (locals.var_t1 - 1.0);
        let assign98020_e150373: f64 = (locals.var_isbs_sws * assign98020_e150372);
        let assign98020_e150377: f64 = (locals.var_vbs_jct - locals.var_vbst);
        let assign98020_e150378: f64 = (locals.var_t4 * assign98020_e150377);
        let assign98020_e150379: f64 = (assign98020_e150373 + assign98020_e150378);
        let assign98020_e150383: f64 = (locals.var_t2 - 1.0);
        let assign98020_e150384: f64 = (locals.var_t0 * assign98020_e150383);
        let assign98020_e150385: f64 = (assign98020_e150379 + assign98020_e150384);
        let assign98020_e150389: f64 = (locals.var_t3 - 1.0);
        let assign98020_e150390: f64 = (locals.var_uc_cisbks * assign98020_e150389);
        let assign98020_e150391: f64 = (assign98020_e150385 + assign98020_e150390);
        (assign98020_e150391, (((((locals.var_isbs_sws_dn0 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_sws_dn2 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98020_e150377) + (locals.var_t4 * (locals.var_vbs_jct_dn2 - locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_sws_dn4 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_sws_dn5 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_sws_dn6 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_sws_dn7 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_sws_dn8 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_sws_dn9 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_sws_dn10 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98020_e150377) + (locals.var_t4 * (locals.var_vbs_jct_dn10 - locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_sws_dn13 * assign98020_e150372) + (locals.var_isbs_sws * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * assign98020_e150377) + (locals.var_t4 * (-locals.var_vbst_dn13)))) + ((locals.var_t0_dn13 * assign98020_e150383) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn13,)
    }
};
        locals.var_ibs_sws = assign98020_e150393;
        locals.var_ibs_sws_dn0 = assign98020_e150393_d_n0;
        locals.var_ibs_sws_dn2 = assign98020_e150393_d_n2;
        locals.var_ibs_sws_dn4 = assign98020_e150393_d_n4;
        locals.var_ibs_sws_dn5 = assign98020_e150393_d_n5;
        locals.var_ibs_sws_dn6 = assign98020_e150393_d_n6;
        locals.var_ibs_sws_dn7 = assign98020_e150393_d_n7;
        locals.var_ibs_sws_dn8 = assign98020_e150393_d_n8;
        locals.var_ibs_sws_dn9 = assign98020_e150393_d_n9;
        locals.var_ibs_sws_dn10 = assign98020_e150393_d_n10;
        locals.var_ibs_sws_dn13 = assign98020_e150393_d_n13;

        let (assign98030_e150398, assign98030_e150398_d_n0, assign98030_e150398_d_n2, assign98030_e150398_d_n4, assign98030_e150398_d_n5, assign98030_e150398_d_n6, assign98030_e150398_d_n7, assign98030_e150398_d_n8, assign98030_e150398_d_n9, assign98030_e150398_d_n10, assign98030_e150398_d_n13,) = {
    if (locals.var_guard2267 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_sws, locals.var_ibs_sws_dn0, locals.var_ibs_sws_dn2, locals.var_ibs_sws_dn4, locals.var_ibs_sws_dn5, locals.var_ibs_sws_dn6, locals.var_ibs_sws_dn7, locals.var_ibs_sws_dn8, locals.var_ibs_sws_dn9, locals.var_ibs_sws_dn10, locals.var_ibs_sws_dn13,)
    }
};
        locals.var_ibs_sws = assign98030_e150398;
        locals.var_ibs_sws_dn0 = assign98030_e150398_d_n0;
        locals.var_ibs_sws_dn2 = assign98030_e150398_d_n2;
        locals.var_ibs_sws_dn4 = assign98030_e150398_d_n4;
        locals.var_ibs_sws_dn5 = assign98030_e150398_d_n5;
        locals.var_ibs_sws_dn6 = assign98030_e150398_d_n6;
        locals.var_ibs_sws_dn7 = assign98030_e150398_d_n7;
        locals.var_ibs_sws_dn8 = assign98030_e150398_d_n8;
        locals.var_ibs_sws_dn9 = assign98030_e150398_d_n9;
        locals.var_ibs_sws_dn10 = assign98030_e150398_d_n10;
        locals.var_ibs_sws_dn13 = assign98030_e150398_d_n13;

        let assign98040_e150401: f64 = (p.p537 * locals.var_isbs2_sws);
        locals.var_t12 = assign98040_e150401;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_sws_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_sws_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_sws_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_sws_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_sws_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_sws_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_sws_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_sws_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_sws_dn10);
        locals.var_t12_dn13 = (p.p537 * locals.var_isbs2_sws_dn13);

        let assign98050_e150405: f64 = (locals.var_t12 * locals.var_vbs_jct);
        let assign98050_e150406: f64 = (locals.var_ibs_sws + assign98050_e150405);
        locals.var_ibs_sws = assign98050_e150406;
        locals.var_ibs_sws_dn0 = (locals.var_ibs_sws_dn0 + (locals.var_t12_dn0 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn2 = (locals.var_ibs_sws_dn2 + ((locals.var_t12_dn2 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn2)));
        locals.var_ibs_sws_dn4 = (locals.var_ibs_sws_dn4 + (locals.var_t12_dn4 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn5 = (locals.var_ibs_sws_dn5 + (locals.var_t12_dn5 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn6 = (locals.var_ibs_sws_dn6 + (locals.var_t12_dn6 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn7 = (locals.var_ibs_sws_dn7 + (locals.var_t12_dn7 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn8 = (locals.var_ibs_sws_dn8 + (locals.var_t12_dn8 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn9 = (locals.var_ibs_sws_dn9 + (locals.var_t12_dn9 * locals.var_vbs_jct));
        locals.var_ibs_sws_dn10 = (locals.var_ibs_sws_dn10 + ((locals.var_t12_dn10 * locals.var_vbs_jct) + (locals.var_t12 * locals.var_vbs_jct_dn10)));
        locals.var_ibs_sws_dn13 = (locals.var_ibs_sws_dn13 + (locals.var_t12_dn13 * locals.var_vbs_jct));

        let assign98060_e150409: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign98060_e150409;

        let assign98070_e150412: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign98070_e150412;

        let (assign98080_e150420, assign98080_e150420_d_n0, assign98080_e150420_d_n2, assign98080_e150420_d_n4, assign98080_e150420_d_n5, assign98080_e150420_d_n6, assign98080_e150420_d_n7, assign98080_e150420_d_n8, assign98080_e150420_d_n9, assign98080_e150420_d_n10, assign98080_e150420_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98080_e150418: f64 = (locals.var_isbs2_swg * locals.var_t9);
        (assign98080_e150418, ((locals.var_isbs2_swg_dn0 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn0)), ((locals.var_isbs2_swg_dn2 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn2)), ((locals.var_isbs2_swg_dn4 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn4)), ((locals.var_isbs2_swg_dn5 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn5)), ((locals.var_isbs2_swg_dn6 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn6)), ((locals.var_isbs2_swg_dn7 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn7)), ((locals.var_isbs2_swg_dn8 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn8)), ((locals.var_isbs2_swg_dn9 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn9)), ((locals.var_isbs2_swg_dn10 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn10)), ((locals.var_isbs2_swg_dn13 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign98080_e150420;
        locals.var_t0_dn0 = assign98080_e150420_d_n0;
        locals.var_t0_dn2 = assign98080_e150420_d_n2;
        locals.var_t0_dn4 = assign98080_e150420_d_n4;
        locals.var_t0_dn5 = assign98080_e150420_d_n5;
        locals.var_t0_dn6 = assign98080_e150420_d_n6;
        locals.var_t0_dn7 = assign98080_e150420_d_n7;
        locals.var_t0_dn8 = assign98080_e150420_d_n8;
        locals.var_t0_dn9 = assign98080_e150420_d_n9;
        locals.var_t0_dn10 = assign98080_e150420_d_n10;
        locals.var_t0_dn13 = assign98080_e150420_d_n13;

        let (assign98090_e150429, assign98090_e150429_d_n0, assign98090_e150429_d_n2, assign98090_e150429_d_n4, assign98090_e150429_d_n5, assign98090_e150429_d_n6, assign98090_e150429_d_n7, assign98090_e150429_d_n8, assign98090_e150429_d_n9, assign98090_e150429_d_n10, assign98090_e150429_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98090_e150425: f64 = (-locals.var_vbsi_jct);
        let assign98090_e150427: f64 = (assign98090_e150425 * locals.var_t10);
        (assign98090_e150427, (assign98090_e150425 * locals.var_t10_dn0), (assign98090_e150425 * locals.var_t10_dn2), (assign98090_e150425 * locals.var_t10_dn4), (assign98090_e150425 * locals.var_t10_dn5), (assign98090_e150425 * locals.var_t10_dn6), (((-locals.var_vbsi_jct_dn7) * locals.var_t10) + (assign98090_e150425 * locals.var_t10_dn7)), (((-locals.var_vbsi_jct_dn8) * locals.var_t10) + (assign98090_e150425 * locals.var_t10_dn8)), (assign98090_e150425 * locals.var_t10_dn9), (assign98090_e150425 * locals.var_t10_dn10), (assign98090_e150425 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign98090_e150429;
        locals.var_tx_dn0 = assign98090_e150429_d_n0;
        locals.var_tx_dn2 = assign98090_e150429_d_n2;
        locals.var_tx_dn4 = assign98090_e150429_d_n4;
        locals.var_tx_dn5 = assign98090_e150429_d_n5;
        locals.var_tx_dn6 = assign98090_e150429_d_n6;
        locals.var_tx_dn7 = assign98090_e150429_d_n7;
        locals.var_tx_dn8 = assign98090_e150429_d_n8;
        locals.var_tx_dn9 = assign98090_e150429_d_n9;
        locals.var_tx_dn10 = assign98090_e150429_d_n10;
        locals.var_tx_dn13 = assign98090_e150429_d_n13;

        let (assign98100_e150436, assign98100_e150436_d_n0, assign98100_e150436_d_n2, assign98100_e150436_d_n4, assign98100_e150436_d_n5, assign98100_e150436_d_n6, assign98100_e150436_d_n7, assign98100_e150436_d_n8, assign98100_e150436_d_n9, assign98100_e150436_d_n10, assign98100_e150436_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98100_e150434: f64 = (locals.var_tx).exp();
        (assign98100_e150434, (assign98100_e150434 * locals.var_tx_dn0), (assign98100_e150434 * locals.var_tx_dn2), (assign98100_e150434 * locals.var_tx_dn4), (assign98100_e150434 * locals.var_tx_dn5), (assign98100_e150434 * locals.var_tx_dn6), (assign98100_e150434 * locals.var_tx_dn7), (assign98100_e150434 * locals.var_tx_dn8), (assign98100_e150434 * locals.var_tx_dn9), (assign98100_e150434 * locals.var_tx_dn10), (assign98100_e150434 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98100_e150436;
        locals.var_t2_dn0 = assign98100_e150436_d_n0;
        locals.var_t2_dn2 = assign98100_e150436_d_n2;
        locals.var_t2_dn4 = assign98100_e150436_d_n4;
        locals.var_t2_dn5 = assign98100_e150436_d_n5;
        locals.var_t2_dn6 = assign98100_e150436_d_n6;
        locals.var_t2_dn7 = assign98100_e150436_d_n7;
        locals.var_t2_dn8 = assign98100_e150436_d_n8;
        locals.var_t2_dn9 = assign98100_e150436_d_n9;
        locals.var_t2_dn10 = assign98100_e150436_d_n10;
        locals.var_t2_dn13 = assign98100_e150436_d_n13;

    }

    pub(super) fn stamp_transient_block_349(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98110_e150442, assign98110_e150442_d_n0, assign98110_e150442_d_n2, assign98110_e150442_d_n4, assign98110_e150442_d_n5, assign98110_e150442_d_n6, assign98110_e150442_d_n7, assign98110_e150442_d_n8, assign98110_e150442_d_n9, assign98110_e150442_d_n10, assign98110_e150442_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign98110_e150442;
        locals.var_t3_dn0 = assign98110_e150442_d_n0;
        locals.var_t3_dn2 = assign98110_e150442_d_n2;
        locals.var_t3_dn4 = assign98110_e150442_d_n4;
        locals.var_t3_dn5 = assign98110_e150442_d_n5;
        locals.var_t3_dn6 = assign98110_e150442_d_n6;
        locals.var_t3_dn7 = assign98110_e150442_d_n7;
        locals.var_t3_dn8 = assign98110_e150442_d_n8;
        locals.var_t3_dn9 = assign98110_e150442_d_n9;
        locals.var_t3_dn10 = assign98110_e150442_d_n10;
        locals.var_t3_dn13 = assign98110_e150442_d_n13;

        let assign98120_e150445: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign98120_e150445;

        let (assign98130_e150455, assign98130_e150455_d_n0, assign98130_e150455_d_n2, assign98130_e150455_d_n4, assign98130_e150455_d_n5, assign98130_e150455_d_n6, assign98130_e150455_d_n7, assign98130_e150455_d_n8, assign98130_e150455_d_n9, assign98130_e150455_d_n10, assign98130_e150455_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) {
        let assign98130_e150453: f64 = (locals.var_vbsi_jct * locals.var_jd_nvtm_invs);
        (assign98130_e150453, (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn0), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn2), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn6), ((locals.var_vbsi_jct_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn7)), ((locals.var_vbsi_jct_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn8)), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn10), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign98130_e150455;
        locals.var_tx_dn0 = assign98130_e150455_d_n0;
        locals.var_tx_dn2 = assign98130_e150455_d_n2;
        locals.var_tx_dn4 = assign98130_e150455_d_n4;
        locals.var_tx_dn5 = assign98130_e150455_d_n5;
        locals.var_tx_dn6 = assign98130_e150455_d_n6;
        locals.var_tx_dn7 = assign98130_e150455_d_n7;
        locals.var_tx_dn8 = assign98130_e150455_d_n8;
        locals.var_tx_dn9 = assign98130_e150455_d_n9;
        locals.var_tx_dn10 = assign98130_e150455_d_n10;
        locals.var_tx_dn13 = assign98130_e150455_d_n13;

        let assign98140_e150458: f64 = (-3.0);
        let assign98140_e150460: f64 = (assign98140_e150458 * 34.0);
        let assign98140_e150461: f64 = if locals.var_tx < assign98140_e150460 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98140_e150461;

        let (assign98150_e150471, assign98150_e150471_d_n0, assign98150_e150471_d_n2, assign98150_e150471_d_n4, assign98150_e150471_d_n5, assign98150_e150471_d_n6, assign98150_e150471_d_n7, assign98150_e150471_d_n8, assign98150_e150471_d_n9, assign98150_e150471_d_n10, assign98150_e150471_d_n13,) = {
    if ((((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98150_e150471;
        locals.var_t1_dn0 = assign98150_e150471_d_n0;
        locals.var_t1_dn2 = assign98150_e150471_d_n2;
        locals.var_t1_dn4 = assign98150_e150471_d_n4;
        locals.var_t1_dn5 = assign98150_e150471_d_n5;
        locals.var_t1_dn6 = assign98150_e150471_d_n6;
        locals.var_t1_dn7 = assign98150_e150471_d_n7;
        locals.var_t1_dn8 = assign98150_e150471_d_n8;
        locals.var_t1_dn9 = assign98150_e150471_d_n9;
        locals.var_t1_dn10 = assign98150_e150471_d_n10;
        locals.var_t1_dn13 = assign98150_e150471_d_n13;

        let (assign98160_e150483, assign98160_e150483_d_n0, assign98160_e150483_d_n2, assign98160_e150483_d_n4, assign98160_e150483_d_n5, assign98160_e150483_d_n6, assign98160_e150483_d_n7, assign98160_e150483_d_n8, assign98160_e150483_d_n9, assign98160_e150483_d_n10, assign98160_e150483_d_n13,) = {
    if ((((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 == 0.0)) {
        let assign98160_e150481: f64 = (locals.var_tx).exp();
        (assign98160_e150481, (assign98160_e150481 * locals.var_tx_dn0), (assign98160_e150481 * locals.var_tx_dn2), (assign98160_e150481 * locals.var_tx_dn4), (assign98160_e150481 * locals.var_tx_dn5), (assign98160_e150481 * locals.var_tx_dn6), (assign98160_e150481 * locals.var_tx_dn7), (assign98160_e150481 * locals.var_tx_dn8), (assign98160_e150481 * locals.var_tx_dn9), (assign98160_e150481 * locals.var_tx_dn10), (assign98160_e150481 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98160_e150483;
        locals.var_t1_dn0 = assign98160_e150483_d_n0;
        locals.var_t1_dn2 = assign98160_e150483_d_n2;
        locals.var_t1_dn4 = assign98160_e150483_d_n4;
        locals.var_t1_dn5 = assign98160_e150483_d_n5;
        locals.var_t1_dn6 = assign98160_e150483_d_n6;
        locals.var_t1_dn7 = assign98160_e150483_d_n7;
        locals.var_t1_dn8 = assign98160_e150483_d_n8;
        locals.var_t1_dn9 = assign98160_e150483_d_n9;
        locals.var_t1_dn10 = assign98160_e150483_d_n10;
        locals.var_t1_dn13 = assign98160_e150483_d_n13;

        let (assign98170_e150507, assign98170_e150507_d_n0, assign98170_e150507_d_n2, assign98170_e150507_d_n4, assign98170_e150507_d_n5, assign98170_e150507_d_n6, assign98170_e150507_d_n7, assign98170_e150507_d_n8, assign98170_e150507_d_n9, assign98170_e150507_d_n10, assign98170_e150507_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) {
        let assign98170_e150492: f64 = (locals.var_t1 - 1.0);
        let assign98170_e150493: f64 = (locals.var_isbs_swg * assign98170_e150492);
        let assign98170_e150497: f64 = (locals.var_t2 - 1.0);
        let assign98170_e150498: f64 = (locals.var_t0 * assign98170_e150497);
        let assign98170_e150499: f64 = (assign98170_e150493 + assign98170_e150498);
        let assign98170_e150503: f64 = (locals.var_t3 - 1.0);
        let assign98170_e150504: f64 = (locals.var_uc_cisbks * assign98170_e150503);
        let assign98170_e150505: f64 = (assign98170_e150499 + assign98170_e150504);
        (assign98170_e150505, ((((locals.var_isbs_swg_dn0 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t0_dn0 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), ((((locals.var_isbs_swg_dn2 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t0_dn2 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), ((((locals.var_isbs_swg_dn4 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t0_dn4 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), ((((locals.var_isbs_swg_dn5 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t0_dn5 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), ((((locals.var_isbs_swg_dn6 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t0_dn6 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), ((((locals.var_isbs_swg_dn7 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t0_dn7 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), ((((locals.var_isbs_swg_dn8 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t0_dn8 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), ((((locals.var_isbs_swg_dn9 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t0_dn9 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), ((((locals.var_isbs_swg_dn10 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t0_dn10 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), ((((locals.var_isbs_swg_dn13 * assign98170_e150492) + (locals.var_isbs_swg * locals.var_t1_dn13)) + ((locals.var_t0_dn13 * assign98170_e150497) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn13,)
    }
};
        locals.var_ibs_swg = assign98170_e150507;
        locals.var_ibs_swg_dn0 = assign98170_e150507_d_n0;
        locals.var_ibs_swg_dn2 = assign98170_e150507_d_n2;
        locals.var_ibs_swg_dn4 = assign98170_e150507_d_n4;
        locals.var_ibs_swg_dn5 = assign98170_e150507_d_n5;
        locals.var_ibs_swg_dn6 = assign98170_e150507_d_n6;
        locals.var_ibs_swg_dn7 = assign98170_e150507_d_n7;
        locals.var_ibs_swg_dn8 = assign98170_e150507_d_n8;
        locals.var_ibs_swg_dn9 = assign98170_e150507_d_n9;
        locals.var_ibs_swg_dn10 = assign98170_e150507_d_n10;
        locals.var_ibs_swg_dn13 = assign98170_e150507_d_n13;

        let (assign98180_e150516, assign98180_e150516_d_n0, assign98180_e150516_d_n2, assign98180_e150516_d_n4, assign98180_e150516_d_n5, assign98180_e150516_d_n6, assign98180_e150516_d_n7, assign98180_e150516_d_n8, assign98180_e150516_d_n9, assign98180_e150516_d_n10, assign98180_e150516_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98180_e150516;
        locals.var_t1_dn0 = assign98180_e150516_d_n0;
        locals.var_t1_dn2 = assign98180_e150516_d_n2;
        locals.var_t1_dn4 = assign98180_e150516_d_n4;
        locals.var_t1_dn5 = assign98180_e150516_d_n5;
        locals.var_t1_dn6 = assign98180_e150516_d_n6;
        locals.var_t1_dn7 = assign98180_e150516_d_n7;
        locals.var_t1_dn8 = assign98180_e150516_d_n8;
        locals.var_t1_dn9 = assign98180_e150516_d_n9;
        locals.var_t1_dn10 = assign98180_e150516_d_n10;
        locals.var_t1_dn13 = assign98180_e150516_d_n13;

        let (assign98190_e150529, assign98190_e150529_d_n0, assign98190_e150529_d_n2, assign98190_e150529_d_n4, assign98190_e150529_d_n5, assign98190_e150529_d_n6, assign98190_e150529_d_n7, assign98190_e150529_d_n8, assign98190_e150529_d_n9, assign98190_e150529_d_n10, assign98190_e150529_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 == 0.0)) {
        let assign98190_e150525: f64 = (locals.var_isbs_swg * locals.var_jd_nvtm_invs);
        let assign98190_e150527: f64 = (assign98190_e150525 * locals.var_t1);
        (assign98190_e150527, ((((locals.var_isbs_swg_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn0)), ((((locals.var_isbs_swg_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn2)), ((((locals.var_isbs_swg_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn4)), ((((locals.var_isbs_swg_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn5)), ((((locals.var_isbs_swg_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn6)), ((((locals.var_isbs_swg_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn7)), ((((locals.var_isbs_swg_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn8)), ((((locals.var_isbs_swg_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn9)), ((((locals.var_isbs_swg_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn10)), ((((locals.var_isbs_swg_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign98190_e150529;
        locals.var_t4_dn0 = assign98190_e150529_d_n0;
        locals.var_t4_dn2 = assign98190_e150529_d_n2;
        locals.var_t4_dn4 = assign98190_e150529_d_n4;
        locals.var_t4_dn5 = assign98190_e150529_d_n5;
        locals.var_t4_dn6 = assign98190_e150529_d_n6;
        locals.var_t4_dn7 = assign98190_e150529_d_n7;
        locals.var_t4_dn8 = assign98190_e150529_d_n8;
        locals.var_t4_dn9 = assign98190_e150529_d_n9;
        locals.var_t4_dn10 = assign98190_e150529_d_n10;
        locals.var_t4_dn13 = assign98190_e150529_d_n13;

        let (assign98200_e150560, assign98200_e150560_d_n0, assign98200_e150560_d_n2, assign98200_e150560_d_n4, assign98200_e150560_d_n5, assign98200_e150560_d_n6, assign98200_e150560_d_n7, assign98200_e150560_d_n8, assign98200_e150560_d_n9, assign98200_e150560_d_n10, assign98200_e150560_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 == 0.0)) {
        let assign98200_e150539: f64 = (locals.var_t1 - 1.0);
        let assign98200_e150540: f64 = (locals.var_isbs_swg * assign98200_e150539);
        let assign98200_e150544: f64 = (locals.var_vbsi_jct - locals.var_vbst);
        let assign98200_e150545: f64 = (locals.var_t4 * assign98200_e150544);
        let assign98200_e150546: f64 = (assign98200_e150540 + assign98200_e150545);
        let assign98200_e150550: f64 = (locals.var_t2 - 1.0);
        let assign98200_e150551: f64 = (locals.var_t0 * assign98200_e150550);
        let assign98200_e150552: f64 = (assign98200_e150546 + assign98200_e150551);
        let assign98200_e150556: f64 = (locals.var_t3 - 1.0);
        let assign98200_e150557: f64 = (locals.var_uc_cisbks * assign98200_e150556);
        let assign98200_e150558: f64 = (assign98200_e150552 + assign98200_e150557);
        (assign98200_e150558, (((((locals.var_isbs_swg_dn0 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn0)))) + ((locals.var_t0_dn0 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn0))) + (locals.var_uc_cisbks * locals.var_t3_dn0)), (((((locals.var_isbs_swg_dn2 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn2)))) + ((locals.var_t0_dn2 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn2))) + (locals.var_uc_cisbks * locals.var_t3_dn2)), (((((locals.var_isbs_swg_dn4 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn4)))) + ((locals.var_t0_dn4 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn4))) + (locals.var_uc_cisbks * locals.var_t3_dn4)), (((((locals.var_isbs_swg_dn5 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn5)))) + ((locals.var_t0_dn5 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn5))) + (locals.var_uc_cisbks * locals.var_t3_dn5)), (((((locals.var_isbs_swg_dn6 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn6)))) + ((locals.var_t0_dn6 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn6))) + (locals.var_uc_cisbks * locals.var_t3_dn6)), (((((locals.var_isbs_swg_dn7 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * assign98200_e150544) + (locals.var_t4 * (locals.var_vbsi_jct_dn7 - locals.var_vbst_dn7)))) + ((locals.var_t0_dn7 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn7))) + (locals.var_uc_cisbks * locals.var_t3_dn7)), (((((locals.var_isbs_swg_dn8 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * assign98200_e150544) + (locals.var_t4 * (locals.var_vbsi_jct_dn8 - locals.var_vbst_dn8)))) + ((locals.var_t0_dn8 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn8))) + (locals.var_uc_cisbks * locals.var_t3_dn8)), (((((locals.var_isbs_swg_dn9 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn9)))) + ((locals.var_t0_dn9 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn9))) + (locals.var_uc_cisbks * locals.var_t3_dn9)), (((((locals.var_isbs_swg_dn10 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn10)))) + ((locals.var_t0_dn10 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn10))) + (locals.var_uc_cisbks * locals.var_t3_dn10)), (((((locals.var_isbs_swg_dn13 * assign98200_e150539) + (locals.var_isbs_swg * locals.var_t1_dn13)) + ((locals.var_t4_dn13 * assign98200_e150544) + (locals.var_t4 * (-locals.var_vbst_dn13)))) + ((locals.var_t0_dn13 * assign98200_e150550) + (locals.var_t0 * locals.var_t2_dn13))) + (locals.var_uc_cisbks * locals.var_t3_dn13)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn13,)
    }
};
        locals.var_ibs_swg = assign98200_e150560;
        locals.var_ibs_swg_dn0 = assign98200_e150560_d_n0;
        locals.var_ibs_swg_dn2 = assign98200_e150560_d_n2;
        locals.var_ibs_swg_dn4 = assign98200_e150560_d_n4;
        locals.var_ibs_swg_dn5 = assign98200_e150560_d_n5;
        locals.var_ibs_swg_dn6 = assign98200_e150560_d_n6;
        locals.var_ibs_swg_dn7 = assign98200_e150560_d_n7;
        locals.var_ibs_swg_dn8 = assign98200_e150560_d_n8;
        locals.var_ibs_swg_dn9 = assign98200_e150560_d_n9;
        locals.var_ibs_swg_dn10 = assign98200_e150560_d_n10;
        locals.var_ibs_swg_dn13 = assign98200_e150560_d_n13;

        let (assign98210_e150567, assign98210_e150567_d_n0, assign98210_e150567_d_n2, assign98210_e150567_d_n4, assign98210_e150567_d_n5, assign98210_e150567_d_n6, assign98210_e150567_d_n7, assign98210_e150567_d_n8, assign98210_e150567_d_n9, assign98210_e150567_d_n10, assign98210_e150567_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn13,)
    }
};
        locals.var_ibs_swg = assign98210_e150567;
        locals.var_ibs_swg_dn0 = assign98210_e150567_d_n0;
        locals.var_ibs_swg_dn2 = assign98210_e150567_d_n2;
        locals.var_ibs_swg_dn4 = assign98210_e150567_d_n4;
        locals.var_ibs_swg_dn5 = assign98210_e150567_d_n5;
        locals.var_ibs_swg_dn6 = assign98210_e150567_d_n6;
        locals.var_ibs_swg_dn7 = assign98210_e150567_d_n7;
        locals.var_ibs_swg_dn8 = assign98210_e150567_d_n8;
        locals.var_ibs_swg_dn9 = assign98210_e150567_d_n9;
        locals.var_ibs_swg_dn10 = assign98210_e150567_d_n10;
        locals.var_ibs_swg_dn13 = assign98210_e150567_d_n13;

        let (assign98220_e150573, assign98220_e150573_d_n0, assign98220_e150573_d_n2, assign98220_e150573_d_n4, assign98220_e150573_d_n5, assign98220_e150573_d_n6, assign98220_e150573_d_n7, assign98220_e150573_d_n8, assign98220_e150573_d_n9, assign98220_e150573_d_n10, assign98220_e150573_d_n13,) = {
    if (locals.var_guard2270 != 0.0) {
        let assign98220_e150571: f64 = (p.p537 * locals.var_isbs2_swg);
        (assign98220_e150571, (p.p537 * locals.var_isbs2_swg_dn0), (p.p537 * locals.var_isbs2_swg_dn2), (p.p537 * locals.var_isbs2_swg_dn4), (p.p537 * locals.var_isbs2_swg_dn5), (p.p537 * locals.var_isbs2_swg_dn6), (p.p537 * locals.var_isbs2_swg_dn7), (p.p537 * locals.var_isbs2_swg_dn8), (p.p537 * locals.var_isbs2_swg_dn9), (p.p537 * locals.var_isbs2_swg_dn10), (p.p537 * locals.var_isbs2_swg_dn13),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign98220_e150573;
        locals.var_t12_dn0 = assign98220_e150573_d_n0;
        locals.var_t12_dn2 = assign98220_e150573_d_n2;
        locals.var_t12_dn4 = assign98220_e150573_d_n4;
        locals.var_t12_dn5 = assign98220_e150573_d_n5;
        locals.var_t12_dn6 = assign98220_e150573_d_n6;
        locals.var_t12_dn7 = assign98220_e150573_d_n7;
        locals.var_t12_dn8 = assign98220_e150573_d_n8;
        locals.var_t12_dn9 = assign98220_e150573_d_n9;
        locals.var_t12_dn10 = assign98220_e150573_d_n10;
        locals.var_t12_dn13 = assign98220_e150573_d_n13;

        let (assign98230_e150581, assign98230_e150581_d_n0, assign98230_e150581_d_n2, assign98230_e150581_d_n4, assign98230_e150581_d_n5, assign98230_e150581_d_n6, assign98230_e150581_d_n7, assign98230_e150581_d_n8, assign98230_e150581_d_n9, assign98230_e150581_d_n10, assign98230_e150581_d_n13,) = {
    if (locals.var_guard2270 != 0.0) {
        let assign98230_e150578: f64 = (locals.var_t12 * locals.var_vbsi_jct);
        let assign98230_e150579: f64 = (locals.var_ibs_swg + assign98230_e150578);
        (assign98230_e150579, (locals.var_ibs_swg_dn0 + (locals.var_t12_dn0 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn2 + (locals.var_t12_dn2 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn4 + (locals.var_t12_dn4 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn5 + (locals.var_t12_dn5 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn6 + (locals.var_t12_dn6 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn7 + ((locals.var_t12_dn7 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn7))), (locals.var_ibs_swg_dn8 + ((locals.var_t12_dn8 * locals.var_vbsi_jct) + (locals.var_t12 * locals.var_vbsi_jct_dn8))), (locals.var_ibs_swg_dn9 + (locals.var_t12_dn9 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn10 + (locals.var_t12_dn10 * locals.var_vbsi_jct)), (locals.var_ibs_swg_dn13 + (locals.var_t12_dn13 * locals.var_vbsi_jct)),)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn13,)
    }
};
        locals.var_ibs_swg = assign98230_e150581;
        locals.var_ibs_swg_dn0 = assign98230_e150581_d_n0;
        locals.var_ibs_swg_dn2 = assign98230_e150581_d_n2;
        locals.var_ibs_swg_dn4 = assign98230_e150581_d_n4;
        locals.var_ibs_swg_dn5 = assign98230_e150581_d_n5;
        locals.var_ibs_swg_dn6 = assign98230_e150581_d_n6;
        locals.var_ibs_swg_dn7 = assign98230_e150581_d_n7;
        locals.var_ibs_swg_dn8 = assign98230_e150581_d_n8;
        locals.var_ibs_swg_dn9 = assign98230_e150581_d_n9;
        locals.var_ibs_swg_dn10 = assign98230_e150581_d_n10;
        locals.var_ibs_swg_dn13 = assign98230_e150581_d_n13;

        let (assign98240_e150586, assign98240_e150586_d_n0, assign98240_e150586_d_n2, assign98240_e150586_d_n4, assign98240_e150586_d_n5, assign98240_e150586_d_n6, assign98240_e150586_d_n7, assign98240_e150586_d_n8, assign98240_e150586_d_n9, assign98240_e150586_d_n10, assign98240_e150586_d_n13,) = {
    if (locals.var_guard2270 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_swg, locals.var_ibs_swg_dn0, locals.var_ibs_swg_dn2, locals.var_ibs_swg_dn4, locals.var_ibs_swg_dn5, locals.var_ibs_swg_dn6, locals.var_ibs_swg_dn7, locals.var_ibs_swg_dn8, locals.var_ibs_swg_dn9, locals.var_ibs_swg_dn10, locals.var_ibs_swg_dn13,)
    }
};
        locals.var_ibs_swg = assign98240_e150586;
        locals.var_ibs_swg_dn0 = assign98240_e150586_d_n0;
        locals.var_ibs_swg_dn2 = assign98240_e150586_d_n2;
        locals.var_ibs_swg_dn4 = assign98240_e150586_d_n4;
        locals.var_ibs_swg_dn5 = assign98240_e150586_d_n5;
        locals.var_ibs_swg_dn6 = assign98240_e150586_d_n6;
        locals.var_ibs_swg_dn7 = assign98240_e150586_d_n7;
        locals.var_ibs_swg_dn8 = assign98240_e150586_d_n8;
        locals.var_ibs_swg_dn9 = assign98240_e150586_d_n9;
        locals.var_ibs_swg_dn10 = assign98240_e150586_d_n10;
        locals.var_ibs_swg_dn13 = assign98240_e150586_d_n13;

        let assign98250_e150589: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98250_e150589;

        let assign98260_e150592: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98260_e150592;

        let (assign98270_e150602, assign98270_e150602_d_n0, assign98270_e150602_d_n2, assign98270_e150602_d_n4, assign98270_e150602_d_n5, assign98270_e150602_d_n6, assign98270_e150602_d_n7, assign98270_e150602_d_n8, assign98270_e150602_d_n9, assign98270_e150602_d_n10, assign98270_e150602_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98270_e150599: f64 = (locals.var_vbd_jct / locals.var_pzbd);
        let assign98270_e150600: f64 = (1.0 - assign98270_e150599);
        (assign98270_e150600, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn2) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn4) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn5) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn6) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn7) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn8) / (locals.var_pzbd * locals.var_pzbd)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn10) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn13) / (locals.var_pzbd * locals.var_pzbd)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98270_e150602;
        locals.var_arg_dn0 = assign98270_e150602_d_n0;
        locals.var_arg_dn2 = assign98270_e150602_d_n2;
        locals.var_arg_dn4 = assign98270_e150602_d_n4;
        locals.var_arg_dn5 = assign98270_e150602_d_n5;
        locals.var_arg_dn6 = assign98270_e150602_d_n6;
        locals.var_arg_dn7 = assign98270_e150602_d_n7;
        locals.var_arg_dn8 = assign98270_e150602_d_n8;
        locals.var_arg_dn9 = assign98270_e150602_d_n9;
        locals.var_arg_dn10 = assign98270_e150602_d_n10;
        locals.var_arg_dn13 = assign98270_e150602_d_n13;

        let assign98280_e150605: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98280_e150605;

        let (assign98290_e150616, assign98290_e150616_d_n0, assign98290_e150616_d_n2, assign98290_e150616_d_n4, assign98290_e150616_d_n5, assign98290_e150616_d_n6, assign98290_e150616_d_n7, assign98290_e150616_d_n8, assign98290_e150616_d_n9, assign98290_e150616_d_n10, assign98290_e150616_d_n13,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) {
        let assign98290_e150613: f64 = (locals.var_arg).sqrt();
        let assign98290_e150614: f64 = (1.0 / assign98290_e150613);
        (assign98290_e150614, (-((locals.var_arg_dn0 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn2 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn4 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn5 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn6 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn7 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn8 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn9 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn10 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn13 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98290_e150616;
        locals.var_sarg_dn0 = assign98290_e150616_d_n0;
        locals.var_sarg_dn2 = assign98290_e150616_d_n2;
        locals.var_sarg_dn4 = assign98290_e150616_d_n4;
        locals.var_sarg_dn5 = assign98290_e150616_d_n5;
        locals.var_sarg_dn6 = assign98290_e150616_d_n6;
        locals.var_sarg_dn7 = assign98290_e150616_d_n7;
        locals.var_sarg_dn8 = assign98290_e150616_d_n8;
        locals.var_sarg_dn9 = assign98290_e150616_d_n9;
        locals.var_sarg_dn10 = assign98290_e150616_d_n10;
        locals.var_sarg_dn13 = assign98290_e150616_d_n13;

        let (assign98300_e150633, assign98300_e150633_d_n0, assign98300_e150633_d_n2, assign98300_e150633_d_n4, assign98300_e150633_d_n5, assign98300_e150633_d_n6, assign98300_e150633_d_n7, assign98300_e150633_d_n8, assign98300_e150633_d_n9, assign98300_e150633_d_n10, assign98300_e150633_d_n13,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 == 0.0)) {
        let (assign98300_e150631, assign98300_e150631_d_n0, assign98300_e150631_d_n2, assign98300_e150631_d_n4, assign98300_e150631_d_n5, assign98300_e150631_d_n6, assign98300_e150631_d_n7, assign98300_e150631_d_n8, assign98300_e150631_d_n9, assign98300_e150631_d_n10, assign98300_e150631_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98300_e150629: f64 = (-p.p503);
                let assign98300_e150630: f64 = (locals.var_arg).powf(assign98300_e150629);
                (assign98300_e150630, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn0)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn2)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn4)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn5)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn6)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn7)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn8)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn9)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn10)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn13)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98300_e150631, assign98300_e150631_d_n0, assign98300_e150631_d_n2, assign98300_e150631_d_n4, assign98300_e150631_d_n5, assign98300_e150631_d_n6, assign98300_e150631_d_n7, assign98300_e150631_d_n8, assign98300_e150631_d_n9, assign98300_e150631_d_n10, assign98300_e150631_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98300_e150633;
        locals.var_sarg_dn0 = assign98300_e150633_d_n0;
        locals.var_sarg_dn2 = assign98300_e150633_d_n2;
        locals.var_sarg_dn4 = assign98300_e150633_d_n4;
        locals.var_sarg_dn5 = assign98300_e150633_d_n5;
        locals.var_sarg_dn6 = assign98300_e150633_d_n6;
        locals.var_sarg_dn7 = assign98300_e150633_d_n7;
        locals.var_sarg_dn8 = assign98300_e150633_d_n8;
        locals.var_sarg_dn9 = assign98300_e150633_d_n9;
        locals.var_sarg_dn10 = assign98300_e150633_d_n10;
        locals.var_sarg_dn13 = assign98300_e150633_d_n13;

        let (assign98310_e150651, assign98310_e150651_d_n0, assign98310_e150651_d_n2, assign98310_e150651_d_n4, assign98310_e150651_d_n5, assign98310_e150651_d_n6, assign98310_e150651_d_n7, assign98310_e150651_d_n8, assign98310_e150651_d_n9, assign98310_e150651_d_n10, assign98310_e150651_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98310_e150639: f64 = (locals.var_pzbd * locals.var_czbd);
        let assign98310_e150643: f64 = (locals.var_arg * locals.var_sarg);
        let assign98310_e150644: f64 = (1.0 - assign98310_e150643);
        let assign98310_e150645: f64 = (assign98310_e150639 * assign98310_e150644);
        let assign98310_e150648: f64 = (1.0 - p.p503);
        let assign98310_e150649: f64 = (assign98310_e150645 / assign98310_e150648);
        (assign98310_e150649, (((((locals.var_pzbd_dn0 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn0)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98310_e150648), (((((locals.var_pzbd_dn2 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn2)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98310_e150648), (((((locals.var_pzbd_dn4 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn4)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98310_e150648), (((((locals.var_pzbd_dn5 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn5)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98310_e150648), (((((locals.var_pzbd_dn6 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn6)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98310_e150648), (((((locals.var_pzbd_dn7 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn7)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98310_e150648), (((((locals.var_pzbd_dn8 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn8)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98310_e150648), (((((locals.var_pzbd_dn9 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn9)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98310_e150648), (((((locals.var_pzbd_dn10 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn10)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98310_e150648), (((((locals.var_pzbd_dn13 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn13)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98310_e150648),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98310_e150651;
        locals.var_qbd_btm_dn0 = assign98310_e150651_d_n0;
        locals.var_qbd_btm_dn2 = assign98310_e150651_d_n2;
        locals.var_qbd_btm_dn4 = assign98310_e150651_d_n4;
        locals.var_qbd_btm_dn5 = assign98310_e150651_d_n5;
        locals.var_qbd_btm_dn6 = assign98310_e150651_d_n6;
        locals.var_qbd_btm_dn7 = assign98310_e150651_d_n7;
        locals.var_qbd_btm_dn8 = assign98310_e150651_d_n8;
        locals.var_qbd_btm_dn9 = assign98310_e150651_d_n9;
        locals.var_qbd_btm_dn10 = assign98310_e150651_d_n10;
        locals.var_qbd_btm_dn13 = assign98310_e150651_d_n13;

        let (assign98330_e150666, assign98330_e150666_d_n0, assign98330_e150666_d_n2, assign98330_e150666_d_n4, assign98330_e150666_d_n5, assign98330_e150666_d_n6, assign98330_e150666_d_n7, assign98330_e150666_d_n8, assign98330_e150666_d_n9, assign98330_e150666_d_n10, assign98330_e150666_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98330_e150666;
        locals.var_t1_dn0 = assign98330_e150666_d_n0;
        locals.var_t1_dn2 = assign98330_e150666_d_n2;
        locals.var_t1_dn4 = assign98330_e150666_d_n4;
        locals.var_t1_dn5 = assign98330_e150666_d_n5;
        locals.var_t1_dn6 = assign98330_e150666_d_n6;
        locals.var_t1_dn7 = assign98330_e150666_d_n7;
        locals.var_t1_dn8 = assign98330_e150666_d_n8;
        locals.var_t1_dn9 = assign98330_e150666_d_n9;
        locals.var_t1_dn10 = assign98330_e150666_d_n10;
        locals.var_t1_dn13 = assign98330_e150666_d_n13;

        let (assign98340_e150677, assign98340_e150677_d_n0, assign98340_e150677_d_n2, assign98340_e150677_d_n4, assign98340_e150677_d_n5, assign98340_e150677_d_n6, assign98340_e150677_d_n7, assign98340_e150677_d_n8, assign98340_e150677_d_n9, assign98340_e150677_d_n10, assign98340_e150677_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        let assign98340_e150673: f64 = (locals.var_czbd * p.p503);
        let assign98340_e150675: f64 = (assign98340_e150673 / locals.var_pzbd);
        (assign98340_e150675, ((((locals.var_czbd_dn0 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn2 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn2)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn4 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn4)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn5 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn5)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn6 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn6)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn7 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn7)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn8 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn8)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn9 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn10 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn13 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn13)) / (locals.var_pzbd * locals.var_pzbd)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98340_e150677;
        locals.var_t2_dn0 = assign98340_e150677_d_n0;
        locals.var_t2_dn2 = assign98340_e150677_d_n2;
        locals.var_t2_dn4 = assign98340_e150677_d_n4;
        locals.var_t2_dn5 = assign98340_e150677_d_n5;
        locals.var_t2_dn6 = assign98340_e150677_d_n6;
        locals.var_t2_dn7 = assign98340_e150677_d_n7;
        locals.var_t2_dn8 = assign98340_e150677_d_n8;
        locals.var_t2_dn9 = assign98340_e150677_d_n9;
        locals.var_t2_dn10 = assign98340_e150677_d_n10;
        locals.var_t2_dn13 = assign98340_e150677_d_n13;

        let (assign98350_e150692, assign98350_e150692_d_n0, assign98350_e150692_d_n2, assign98350_e150692_d_n4, assign98350_e150692_d_n5, assign98350_e150692_d_n6, assign98350_e150692_d_n7, assign98350_e150692_d_n8, assign98350_e150692_d_n9, assign98350_e150692_d_n10, assign98350_e150692_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        let assign98350_e150686: f64 = (locals.var_vbd_jct * 0.5);
        let assign98350_e150688: f64 = (assign98350_e150686 * locals.var_t2);
        let assign98350_e150689: f64 = (locals.var_t1 + assign98350_e150688);
        let assign98350_e150690: f64 = (locals.var_vbd_jct * assign98350_e150689);
        (assign98350_e150690, ((locals.var_vbd_jct_dn0 * assign98350_e150689) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98350_e150686 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98350_e150686 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98350_e150686 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98350_e150686 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98350_e150686 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98350_e150686 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98350_e150686 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98350_e150689) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98350_e150686 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98350_e150686 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98350_e150686 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98350_e150692;
        locals.var_qbd_btm_dn0 = assign98350_e150692_d_n0;
        locals.var_qbd_btm_dn2 = assign98350_e150692_d_n2;
        locals.var_qbd_btm_dn4 = assign98350_e150692_d_n4;
        locals.var_qbd_btm_dn5 = assign98350_e150692_d_n5;
        locals.var_qbd_btm_dn6 = assign98350_e150692_d_n6;
        locals.var_qbd_btm_dn7 = assign98350_e150692_d_n7;
        locals.var_qbd_btm_dn8 = assign98350_e150692_d_n8;
        locals.var_qbd_btm_dn9 = assign98350_e150692_d_n9;
        locals.var_qbd_btm_dn10 = assign98350_e150692_d_n10;
        locals.var_qbd_btm_dn13 = assign98350_e150692_d_n13;

        let (assign98370_e150708, assign98370_e150708_d_n0, assign98370_e150708_d_n2, assign98370_e150708_d_n4, assign98370_e150708_d_n5, assign98370_e150708_d_n6, assign98370_e150708_d_n7, assign98370_e150708_d_n8, assign98370_e150708_d_n9, assign98370_e150708_d_n10, assign98370_e150708_d_n13,) = {
    if (locals.var_guard2274 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98370_e150708;
        locals.var_qbd_btm_dn0 = assign98370_e150708_d_n0;
        locals.var_qbd_btm_dn2 = assign98370_e150708_d_n2;
        locals.var_qbd_btm_dn4 = assign98370_e150708_d_n4;
        locals.var_qbd_btm_dn5 = assign98370_e150708_d_n5;
        locals.var_qbd_btm_dn6 = assign98370_e150708_d_n6;
        locals.var_qbd_btm_dn7 = assign98370_e150708_d_n7;
        locals.var_qbd_btm_dn8 = assign98370_e150708_d_n8;
        locals.var_qbd_btm_dn9 = assign98370_e150708_d_n9;
        locals.var_qbd_btm_dn10 = assign98370_e150708_d_n10;
        locals.var_qbd_btm_dn13 = assign98370_e150708_d_n13;

        let assign98390_e150716: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98390_e150716;

        let assign98400_e150719: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98400_e150719;

        let (assign98410_e150729, assign98410_e150729_d_n0, assign98410_e150729_d_n2, assign98410_e150729_d_n4, assign98410_e150729_d_n5, assign98410_e150729_d_n6, assign98410_e150729_d_n7, assign98410_e150729_d_n8, assign98410_e150729_d_n9, assign98410_e150729_d_n10, assign98410_e150729_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) {
        let assign98410_e150726: f64 = (locals.var_vbd_jct / locals.var_pzbdsw);
        let assign98410_e150727: f64 = (1.0 - assign98410_e150726);
        (assign98410_e150727, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn2) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn4) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn5) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn6) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn7) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn8) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn10) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn13) / (locals.var_pzbdsw * locals.var_pzbdsw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98410_e150729;
        locals.var_arg_dn0 = assign98410_e150729_d_n0;
        locals.var_arg_dn2 = assign98410_e150729_d_n2;
        locals.var_arg_dn4 = assign98410_e150729_d_n4;
        locals.var_arg_dn5 = assign98410_e150729_d_n5;
        locals.var_arg_dn6 = assign98410_e150729_d_n6;
        locals.var_arg_dn7 = assign98410_e150729_d_n7;
        locals.var_arg_dn8 = assign98410_e150729_d_n8;
        locals.var_arg_dn9 = assign98410_e150729_d_n9;
        locals.var_arg_dn10 = assign98410_e150729_d_n10;
        locals.var_arg_dn13 = assign98410_e150729_d_n13;

        let assign98420_e150732: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98420_e150732;

        let (assign98430_e150743, assign98430_e150743_d_n0, assign98430_e150743_d_n2, assign98430_e150743_d_n4, assign98430_e150743_d_n5, assign98430_e150743_d_n6, assign98430_e150743_d_n7, assign98430_e150743_d_n8, assign98430_e150743_d_n9, assign98430_e150743_d_n10, assign98430_e150743_d_n13,) = {
    if (((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) && (locals.var_guard2279 != 0.0)) {
        let assign98430_e150740: f64 = (locals.var_arg).sqrt();
        let assign98430_e150741: f64 = (1.0 / assign98430_e150740);
        (assign98430_e150741, (-((locals.var_arg_dn0 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn2 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn4 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn5 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn6 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn7 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn8 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn9 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn10 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn13 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98430_e150743;
        locals.var_sarg_dn0 = assign98430_e150743_d_n0;
        locals.var_sarg_dn2 = assign98430_e150743_d_n2;
        locals.var_sarg_dn4 = assign98430_e150743_d_n4;
        locals.var_sarg_dn5 = assign98430_e150743_d_n5;
        locals.var_sarg_dn6 = assign98430_e150743_d_n6;
        locals.var_sarg_dn7 = assign98430_e150743_d_n7;
        locals.var_sarg_dn8 = assign98430_e150743_d_n8;
        locals.var_sarg_dn9 = assign98430_e150743_d_n9;
        locals.var_sarg_dn10 = assign98430_e150743_d_n10;
        locals.var_sarg_dn13 = assign98430_e150743_d_n13;

    }

    pub(super) fn stamp_transient_block_350(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98440_e150760, assign98440_e150760_d_n0, assign98440_e150760_d_n2, assign98440_e150760_d_n4, assign98440_e150760_d_n5, assign98440_e150760_d_n6, assign98440_e150760_d_n7, assign98440_e150760_d_n8, assign98440_e150760_d_n9, assign98440_e150760_d_n10, assign98440_e150760_d_n13,) = {
    if (((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) && (locals.var_guard2279 == 0.0)) {
        let (assign98440_e150758, assign98440_e150758_d_n0, assign98440_e150758_d_n2, assign98440_e150758_d_n4, assign98440_e150758_d_n5, assign98440_e150758_d_n6, assign98440_e150758_d_n7, assign98440_e150758_d_n8, assign98440_e150758_d_n9, assign98440_e150758_d_n10, assign98440_e150758_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98440_e150756: f64 = (-p.p504);
                let assign98440_e150757: f64 = (locals.var_arg).powf(assign98440_e150756);
                (assign98440_e150757, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn0)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn2)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn4)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn5)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn6)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn7)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn8)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn9)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn10)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn13)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98440_e150758, assign98440_e150758_d_n0, assign98440_e150758_d_n2, assign98440_e150758_d_n4, assign98440_e150758_d_n5, assign98440_e150758_d_n6, assign98440_e150758_d_n7, assign98440_e150758_d_n8, assign98440_e150758_d_n9, assign98440_e150758_d_n10, assign98440_e150758_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98440_e150760;
        locals.var_sarg_dn0 = assign98440_e150760_d_n0;
        locals.var_sarg_dn2 = assign98440_e150760_d_n2;
        locals.var_sarg_dn4 = assign98440_e150760_d_n4;
        locals.var_sarg_dn5 = assign98440_e150760_d_n5;
        locals.var_sarg_dn6 = assign98440_e150760_d_n6;
        locals.var_sarg_dn7 = assign98440_e150760_d_n7;
        locals.var_sarg_dn8 = assign98440_e150760_d_n8;
        locals.var_sarg_dn9 = assign98440_e150760_d_n9;
        locals.var_sarg_dn10 = assign98440_e150760_d_n10;
        locals.var_sarg_dn13 = assign98440_e150760_d_n13;

        let (assign98450_e150778, assign98450_e150778_d_n0, assign98450_e150778_d_n2, assign98450_e150778_d_n4, assign98450_e150778_d_n5, assign98450_e150778_d_n6, assign98450_e150778_d_n7, assign98450_e150778_d_n8, assign98450_e150778_d_n9, assign98450_e150778_d_n10, assign98450_e150778_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) {
        let assign98450_e150766: f64 = (locals.var_pzbdsw * locals.var_czbdsw);
        let assign98450_e150770: f64 = (locals.var_arg * locals.var_sarg);
        let assign98450_e150771: f64 = (1.0 - assign98450_e150770);
        let assign98450_e150772: f64 = (assign98450_e150766 * assign98450_e150771);
        let assign98450_e150775: f64 = (1.0 - p.p504);
        let assign98450_e150776: f64 = (assign98450_e150772 / assign98450_e150775);
        (assign98450_e150776, (((((locals.var_pzbdsw_dn0 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn0)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn2 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn2)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn4 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn4)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn5 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn5)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn6 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn6)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn7 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn7)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn8 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn8)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn9 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn9)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn10 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn10)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn13 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn13)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98450_e150775),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98450_e150778;
        locals.var_qbd_sws_dn0 = assign98450_e150778_d_n0;
        locals.var_qbd_sws_dn2 = assign98450_e150778_d_n2;
        locals.var_qbd_sws_dn4 = assign98450_e150778_d_n4;
        locals.var_qbd_sws_dn5 = assign98450_e150778_d_n5;
        locals.var_qbd_sws_dn6 = assign98450_e150778_d_n6;
        locals.var_qbd_sws_dn7 = assign98450_e150778_d_n7;
        locals.var_qbd_sws_dn8 = assign98450_e150778_d_n8;
        locals.var_qbd_sws_dn9 = assign98450_e150778_d_n9;
        locals.var_qbd_sws_dn10 = assign98450_e150778_d_n10;
        locals.var_qbd_sws_dn13 = assign98450_e150778_d_n13;

        let (assign98470_e150793, assign98470_e150793_d_n0, assign98470_e150793_d_n2, assign98470_e150793_d_n4, assign98470_e150793_d_n5, assign98470_e150793_d_n6, assign98470_e150793_d_n7, assign98470_e150793_d_n8, assign98470_e150793_d_n9, assign98470_e150793_d_n10, assign98470_e150793_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98470_e150793;
        locals.var_t1_dn0 = assign98470_e150793_d_n0;
        locals.var_t1_dn2 = assign98470_e150793_d_n2;
        locals.var_t1_dn4 = assign98470_e150793_d_n4;
        locals.var_t1_dn5 = assign98470_e150793_d_n5;
        locals.var_t1_dn6 = assign98470_e150793_d_n6;
        locals.var_t1_dn7 = assign98470_e150793_d_n7;
        locals.var_t1_dn8 = assign98470_e150793_d_n8;
        locals.var_t1_dn9 = assign98470_e150793_d_n9;
        locals.var_t1_dn10 = assign98470_e150793_d_n10;
        locals.var_t1_dn13 = assign98470_e150793_d_n13;

        let (assign98480_e150804, assign98480_e150804_d_n0, assign98480_e150804_d_n2, assign98480_e150804_d_n4, assign98480_e150804_d_n5, assign98480_e150804_d_n6, assign98480_e150804_d_n7, assign98480_e150804_d_n8, assign98480_e150804_d_n9, assign98480_e150804_d_n10, assign98480_e150804_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        let assign98480_e150800: f64 = (locals.var_czbdsw * p.p504);
        let assign98480_e150802: f64 = (assign98480_e150800 / locals.var_pzbdsw);
        (assign98480_e150802, ((((locals.var_czbdsw_dn0 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn2 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn2)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn4 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn4)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn5 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn5)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn6 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn6)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn7 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn7)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn8 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn8)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn9 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn10 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn13 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn13)) / (locals.var_pzbdsw * locals.var_pzbdsw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98480_e150804;
        locals.var_t2_dn0 = assign98480_e150804_d_n0;
        locals.var_t2_dn2 = assign98480_e150804_d_n2;
        locals.var_t2_dn4 = assign98480_e150804_d_n4;
        locals.var_t2_dn5 = assign98480_e150804_d_n5;
        locals.var_t2_dn6 = assign98480_e150804_d_n6;
        locals.var_t2_dn7 = assign98480_e150804_d_n7;
        locals.var_t2_dn8 = assign98480_e150804_d_n8;
        locals.var_t2_dn9 = assign98480_e150804_d_n9;
        locals.var_t2_dn10 = assign98480_e150804_d_n10;
        locals.var_t2_dn13 = assign98480_e150804_d_n13;

        let (assign98490_e150819, assign98490_e150819_d_n0, assign98490_e150819_d_n2, assign98490_e150819_d_n4, assign98490_e150819_d_n5, assign98490_e150819_d_n6, assign98490_e150819_d_n7, assign98490_e150819_d_n8, assign98490_e150819_d_n9, assign98490_e150819_d_n10, assign98490_e150819_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        let assign98490_e150813: f64 = (locals.var_vbd_jct * 0.5);
        let assign98490_e150815: f64 = (assign98490_e150813 * locals.var_t2);
        let assign98490_e150816: f64 = (locals.var_t1 + assign98490_e150815);
        let assign98490_e150817: f64 = (locals.var_vbd_jct * assign98490_e150816);
        (assign98490_e150817, ((locals.var_vbd_jct_dn0 * assign98490_e150816) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98490_e150813 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98490_e150813 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98490_e150813 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98490_e150813 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98490_e150813 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98490_e150813 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98490_e150813 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98490_e150816) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98490_e150813 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98490_e150813 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98490_e150813 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98490_e150819;
        locals.var_qbd_sws_dn0 = assign98490_e150819_d_n0;
        locals.var_qbd_sws_dn2 = assign98490_e150819_d_n2;
        locals.var_qbd_sws_dn4 = assign98490_e150819_d_n4;
        locals.var_qbd_sws_dn5 = assign98490_e150819_d_n5;
        locals.var_qbd_sws_dn6 = assign98490_e150819_d_n6;
        locals.var_qbd_sws_dn7 = assign98490_e150819_d_n7;
        locals.var_qbd_sws_dn8 = assign98490_e150819_d_n8;
        locals.var_qbd_sws_dn9 = assign98490_e150819_d_n9;
        locals.var_qbd_sws_dn10 = assign98490_e150819_d_n10;
        locals.var_qbd_sws_dn13 = assign98490_e150819_d_n13;

        let (assign98510_e150835, assign98510_e150835_d_n0, assign98510_e150835_d_n2, assign98510_e150835_d_n4, assign98510_e150835_d_n5, assign98510_e150835_d_n6, assign98510_e150835_d_n7, assign98510_e150835_d_n8, assign98510_e150835_d_n9, assign98510_e150835_d_n10, assign98510_e150835_d_n13,) = {
    if (locals.var_guard2277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98510_e150835;
        locals.var_qbd_sws_dn0 = assign98510_e150835_d_n0;
        locals.var_qbd_sws_dn2 = assign98510_e150835_d_n2;
        locals.var_qbd_sws_dn4 = assign98510_e150835_d_n4;
        locals.var_qbd_sws_dn5 = assign98510_e150835_d_n5;
        locals.var_qbd_sws_dn6 = assign98510_e150835_d_n6;
        locals.var_qbd_sws_dn7 = assign98510_e150835_d_n7;
        locals.var_qbd_sws_dn8 = assign98510_e150835_d_n8;
        locals.var_qbd_sws_dn9 = assign98510_e150835_d_n9;
        locals.var_qbd_sws_dn10 = assign98510_e150835_d_n10;
        locals.var_qbd_sws_dn13 = assign98510_e150835_d_n13;

        let assign98530_e150843: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98530_e150843;

        let assign98540_e150846: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98540_e150846;

        let assign98550_e150849: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98550_e150849;

        let (assign98560_e150861, assign98560_e150861_d_n0, assign98560_e150861_d_n2, assign98560_e150861_d_n4, assign98560_e150861_d_n5, assign98560_e150861_d_n6, assign98560_e150861_d_n7, assign98560_e150861_d_n8, assign98560_e150861_d_n9, assign98560_e150861_d_n10, assign98560_e150861_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign98560_e150858: f64 = (locals.var_vbdi_jct / locals.var_pzbdswg);
        let assign98560_e150859: f64 = (1.0 - assign98560_e150858);
        (assign98560_e150859, (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn0) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn5 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn8 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn9) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn13) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98560_e150861;
        locals.var_arg_dn0 = assign98560_e150861_d_n0;
        locals.var_arg_dn2 = assign98560_e150861_d_n2;
        locals.var_arg_dn4 = assign98560_e150861_d_n4;
        locals.var_arg_dn5 = assign98560_e150861_d_n5;
        locals.var_arg_dn6 = assign98560_e150861_d_n6;
        locals.var_arg_dn7 = assign98560_e150861_d_n7;
        locals.var_arg_dn8 = assign98560_e150861_d_n8;
        locals.var_arg_dn9 = assign98560_e150861_d_n9;
        locals.var_arg_dn10 = assign98560_e150861_d_n10;
        locals.var_arg_dn13 = assign98560_e150861_d_n13;

        let assign98570_e150864: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98570_e150864;

        let (assign98580_e150877, assign98580_e150877_d_n0, assign98580_e150877_d_n2, assign98580_e150877_d_n4, assign98580_e150877_d_n5, assign98580_e150877_d_n6, assign98580_e150877_d_n7, assign98580_e150877_d_n8, assign98580_e150877_d_n9, assign98580_e150877_d_n10, assign98580_e150877_d_n13,) = {
    if ((((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 != 0.0)) {
        let assign98580_e150874: f64 = (locals.var_arg).sqrt();
        let assign98580_e150875: f64 = (1.0 / assign98580_e150874);
        (assign98580_e150875, (-((locals.var_arg_dn0 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn2 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn4 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn5 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn6 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn7 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn8 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn9 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn10 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn13 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98580_e150877;
        locals.var_sarg_dn0 = assign98580_e150877_d_n0;
        locals.var_sarg_dn2 = assign98580_e150877_d_n2;
        locals.var_sarg_dn4 = assign98580_e150877_d_n4;
        locals.var_sarg_dn5 = assign98580_e150877_d_n5;
        locals.var_sarg_dn6 = assign98580_e150877_d_n6;
        locals.var_sarg_dn7 = assign98580_e150877_d_n7;
        locals.var_sarg_dn8 = assign98580_e150877_d_n8;
        locals.var_sarg_dn9 = assign98580_e150877_d_n9;
        locals.var_sarg_dn10 = assign98580_e150877_d_n10;
        locals.var_sarg_dn13 = assign98580_e150877_d_n13;

        let (assign98590_e150896, assign98590_e150896_d_n0, assign98590_e150896_d_n2, assign98590_e150896_d_n4, assign98590_e150896_d_n5, assign98590_e150896_d_n6, assign98590_e150896_d_n7, assign98590_e150896_d_n8, assign98590_e150896_d_n9, assign98590_e150896_d_n10, assign98590_e150896_d_n13,) = {
    if ((((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 == 0.0)) {
        let (assign98590_e150894, assign98590_e150894_d_n0, assign98590_e150894_d_n2, assign98590_e150894_d_n4, assign98590_e150894_d_n5, assign98590_e150894_d_n6, assign98590_e150894_d_n7, assign98590_e150894_d_n8, assign98590_e150894_d_n9, assign98590_e150894_d_n10, assign98590_e150894_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98590_e150892: f64 = (-p.p505);
                let assign98590_e150893: f64 = (locals.var_arg).powf(assign98590_e150892);
                (assign98590_e150893, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn0)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn2)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn4)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn5)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn6)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn7)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn8)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn9)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn10)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn13)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98590_e150894, assign98590_e150894_d_n0, assign98590_e150894_d_n2, assign98590_e150894_d_n4, assign98590_e150894_d_n5, assign98590_e150894_d_n6, assign98590_e150894_d_n7, assign98590_e150894_d_n8, assign98590_e150894_d_n9, assign98590_e150894_d_n10, assign98590_e150894_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98590_e150896;
        locals.var_sarg_dn0 = assign98590_e150896_d_n0;
        locals.var_sarg_dn2 = assign98590_e150896_d_n2;
        locals.var_sarg_dn4 = assign98590_e150896_d_n4;
        locals.var_sarg_dn5 = assign98590_e150896_d_n5;
        locals.var_sarg_dn6 = assign98590_e150896_d_n6;
        locals.var_sarg_dn7 = assign98590_e150896_d_n7;
        locals.var_sarg_dn8 = assign98590_e150896_d_n8;
        locals.var_sarg_dn9 = assign98590_e150896_d_n9;
        locals.var_sarg_dn10 = assign98590_e150896_d_n10;
        locals.var_sarg_dn13 = assign98590_e150896_d_n13;

        let (assign98600_e150916, assign98600_e150916_d_n0, assign98600_e150916_d_n2, assign98600_e150916_d_n4, assign98600_e150916_d_n5, assign98600_e150916_d_n6, assign98600_e150916_d_n7, assign98600_e150916_d_n8, assign98600_e150916_d_n9, assign98600_e150916_d_n10, assign98600_e150916_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign98600_e150904: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98600_e150908: f64 = (locals.var_arg * locals.var_sarg);
        let assign98600_e150909: f64 = (1.0 - assign98600_e150908);
        let assign98600_e150910: f64 = (assign98600_e150904 * assign98600_e150909);
        let assign98600_e150913: f64 = (1.0 - p.p505);
        let assign98600_e150914: f64 = (assign98600_e150910 / assign98600_e150913);
        (assign98600_e150914, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn13 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn13)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98600_e150913),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98600_e150916;
        locals.var_qbd_swg_dn0 = assign98600_e150916_d_n0;
        locals.var_qbd_swg_dn2 = assign98600_e150916_d_n2;
        locals.var_qbd_swg_dn4 = assign98600_e150916_d_n4;
        locals.var_qbd_swg_dn5 = assign98600_e150916_d_n5;
        locals.var_qbd_swg_dn6 = assign98600_e150916_d_n6;
        locals.var_qbd_swg_dn7 = assign98600_e150916_d_n7;
        locals.var_qbd_swg_dn8 = assign98600_e150916_d_n8;
        locals.var_qbd_swg_dn9 = assign98600_e150916_d_n9;
        locals.var_qbd_swg_dn10 = assign98600_e150916_d_n10;
        locals.var_qbd_swg_dn13 = assign98600_e150916_d_n13;

        let (assign98620_e150935, assign98620_e150935_d_n0, assign98620_e150935_d_n2, assign98620_e150935_d_n4, assign98620_e150935_d_n5, assign98620_e150935_d_n6, assign98620_e150935_d_n7, assign98620_e150935_d_n8, assign98620_e150935_d_n9, assign98620_e150935_d_n10, assign98620_e150935_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98620_e150935;
        locals.var_t1_dn0 = assign98620_e150935_d_n0;
        locals.var_t1_dn2 = assign98620_e150935_d_n2;
        locals.var_t1_dn4 = assign98620_e150935_d_n4;
        locals.var_t1_dn5 = assign98620_e150935_d_n5;
        locals.var_t1_dn6 = assign98620_e150935_d_n6;
        locals.var_t1_dn7 = assign98620_e150935_d_n7;
        locals.var_t1_dn8 = assign98620_e150935_d_n8;
        locals.var_t1_dn9 = assign98620_e150935_d_n9;
        locals.var_t1_dn10 = assign98620_e150935_d_n10;
        locals.var_t1_dn13 = assign98620_e150935_d_n13;

        let (assign98630_e150948, assign98630_e150948_d_n0, assign98630_e150948_d_n2, assign98630_e150948_d_n4, assign98630_e150948_d_n5, assign98630_e150948_d_n6, assign98630_e150948_d_n7, assign98630_e150948_d_n8, assign98630_e150948_d_n9, assign98630_e150948_d_n10, assign98630_e150948_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        let assign98630_e150944: f64 = (locals.var_czbdswg * p.p505);
        let assign98630_e150946: f64 = (assign98630_e150944 / locals.var_pzbdswg);
        (assign98630_e150946, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn13 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn13)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98630_e150948;
        locals.var_t2_dn0 = assign98630_e150948_d_n0;
        locals.var_t2_dn2 = assign98630_e150948_d_n2;
        locals.var_t2_dn4 = assign98630_e150948_d_n4;
        locals.var_t2_dn5 = assign98630_e150948_d_n5;
        locals.var_t2_dn6 = assign98630_e150948_d_n6;
        locals.var_t2_dn7 = assign98630_e150948_d_n7;
        locals.var_t2_dn8 = assign98630_e150948_d_n8;
        locals.var_t2_dn9 = assign98630_e150948_d_n9;
        locals.var_t2_dn10 = assign98630_e150948_d_n10;
        locals.var_t2_dn13 = assign98630_e150948_d_n13;

        let (assign98640_e150965, assign98640_e150965_d_n0, assign98640_e150965_d_n2, assign98640_e150965_d_n4, assign98640_e150965_d_n5, assign98640_e150965_d_n6, assign98640_e150965_d_n7, assign98640_e150965_d_n8, assign98640_e150965_d_n9, assign98640_e150965_d_n10, assign98640_e150965_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        let assign98640_e150959: f64 = (locals.var_vbdi_jct * 0.5);
        let assign98640_e150961: f64 = (assign98640_e150959 * locals.var_t2);
        let assign98640_e150962: f64 = (locals.var_t1 + assign98640_e150961);
        let assign98640_e150963: f64 = (locals.var_vbdi_jct * assign98640_e150962);
        (assign98640_e150963, (locals.var_vbdi_jct * (locals.var_t1_dn0 + (assign98640_e150959 * locals.var_t2_dn0))), (locals.var_vbdi_jct * (locals.var_t1_dn2 + (assign98640_e150959 * locals.var_t2_dn2))), (locals.var_vbdi_jct * (locals.var_t1_dn4 + (assign98640_e150959 * locals.var_t2_dn4))), ((locals.var_vbdi_jct_dn5 * assign98640_e150962) + (locals.var_vbdi_jct * (locals.var_t1_dn5 + (((locals.var_vbdi_jct_dn5 * 0.5) * locals.var_t2) + (assign98640_e150959 * locals.var_t2_dn5))))), (locals.var_vbdi_jct * (locals.var_t1_dn6 + (assign98640_e150959 * locals.var_t2_dn6))), (locals.var_vbdi_jct * (locals.var_t1_dn7 + (assign98640_e150959 * locals.var_t2_dn7))), ((locals.var_vbdi_jct_dn8 * assign98640_e150962) + (locals.var_vbdi_jct * (locals.var_t1_dn8 + (((locals.var_vbdi_jct_dn8 * 0.5) * locals.var_t2) + (assign98640_e150959 * locals.var_t2_dn8))))), (locals.var_vbdi_jct * (locals.var_t1_dn9 + (assign98640_e150959 * locals.var_t2_dn9))), (locals.var_vbdi_jct * (locals.var_t1_dn10 + (assign98640_e150959 * locals.var_t2_dn10))), (locals.var_vbdi_jct * (locals.var_t1_dn13 + (assign98640_e150959 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98640_e150965;
        locals.var_qbd_swg_dn0 = assign98640_e150965_d_n0;
        locals.var_qbd_swg_dn2 = assign98640_e150965_d_n2;
        locals.var_qbd_swg_dn4 = assign98640_e150965_d_n4;
        locals.var_qbd_swg_dn5 = assign98640_e150965_d_n5;
        locals.var_qbd_swg_dn6 = assign98640_e150965_d_n6;
        locals.var_qbd_swg_dn7 = assign98640_e150965_d_n7;
        locals.var_qbd_swg_dn8 = assign98640_e150965_d_n8;
        locals.var_qbd_swg_dn9 = assign98640_e150965_d_n9;
        locals.var_qbd_swg_dn10 = assign98640_e150965_d_n10;
        locals.var_qbd_swg_dn13 = assign98640_e150965_d_n13;

        let (assign98660_e150985, assign98660_e150985_d_n0, assign98660_e150985_d_n2, assign98660_e150985_d_n4, assign98660_e150985_d_n5, assign98660_e150985_d_n6, assign98660_e150985_d_n7, assign98660_e150985_d_n8, assign98660_e150985_d_n9, assign98660_e150985_d_n10, assign98660_e150985_d_n13,) = {
    if ((locals.var_guard2280 != 0.0) && (locals.var_guard2281 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98660_e150985;
        locals.var_qbd_swg_dn0 = assign98660_e150985_d_n0;
        locals.var_qbd_swg_dn2 = assign98660_e150985_d_n2;
        locals.var_qbd_swg_dn4 = assign98660_e150985_d_n4;
        locals.var_qbd_swg_dn5 = assign98660_e150985_d_n5;
        locals.var_qbd_swg_dn6 = assign98660_e150985_d_n6;
        locals.var_qbd_swg_dn7 = assign98660_e150985_d_n7;
        locals.var_qbd_swg_dn8 = assign98660_e150985_d_n8;
        locals.var_qbd_swg_dn9 = assign98660_e150985_d_n9;
        locals.var_qbd_swg_dn10 = assign98660_e150985_d_n10;
        locals.var_qbd_swg_dn13 = assign98660_e150985_d_n13;

        let assign98680_e150995: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98680_e150995;

        let assign98690_e150998: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98690_e150998;

        let (assign98700_e151011, assign98700_e151011_d_n0, assign98700_e151011_d_n2, assign98700_e151011_d_n4, assign98700_e151011_d_n5, assign98700_e151011_d_n6, assign98700_e151011_d_n7, assign98700_e151011_d_n8, assign98700_e151011_d_n9, assign98700_e151011_d_n10, assign98700_e151011_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign98700_e151008: f64 = (locals.var_vbd_jct / locals.var_pzbdswg);
        let assign98700_e151009: f64 = (1.0 - assign98700_e151008);
        (assign98700_e151009, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn13) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98700_e151011;
        locals.var_arg_dn0 = assign98700_e151011_d_n0;
        locals.var_arg_dn2 = assign98700_e151011_d_n2;
        locals.var_arg_dn4 = assign98700_e151011_d_n4;
        locals.var_arg_dn5 = assign98700_e151011_d_n5;
        locals.var_arg_dn6 = assign98700_e151011_d_n6;
        locals.var_arg_dn7 = assign98700_e151011_d_n7;
        locals.var_arg_dn8 = assign98700_e151011_d_n8;
        locals.var_arg_dn9 = assign98700_e151011_d_n9;
        locals.var_arg_dn10 = assign98700_e151011_d_n10;
        locals.var_arg_dn13 = assign98700_e151011_d_n13;

        let assign98710_e151014: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98710_e151014;

        let (assign98720_e151028, assign98720_e151028_d_n0, assign98720_e151028_d_n2, assign98720_e151028_d_n4, assign98720_e151028_d_n5, assign98720_e151028_d_n6, assign98720_e151028_d_n7, assign98720_e151028_d_n8, assign98720_e151028_d_n9, assign98720_e151028_d_n10, assign98720_e151028_d_n13,) = {
    if ((((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) {
        let assign98720_e151025: f64 = (locals.var_arg).sqrt();
        let assign98720_e151026: f64 = (1.0 / assign98720_e151025);
        (assign98720_e151026, (-((locals.var_arg_dn0 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn2 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn4 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn5 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn6 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn7 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn8 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn9 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn10 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn13 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98720_e151028;
        locals.var_sarg_dn0 = assign98720_e151028_d_n0;
        locals.var_sarg_dn2 = assign98720_e151028_d_n2;
        locals.var_sarg_dn4 = assign98720_e151028_d_n4;
        locals.var_sarg_dn5 = assign98720_e151028_d_n5;
        locals.var_sarg_dn6 = assign98720_e151028_d_n6;
        locals.var_sarg_dn7 = assign98720_e151028_d_n7;
        locals.var_sarg_dn8 = assign98720_e151028_d_n8;
        locals.var_sarg_dn9 = assign98720_e151028_d_n9;
        locals.var_sarg_dn10 = assign98720_e151028_d_n10;
        locals.var_sarg_dn13 = assign98720_e151028_d_n13;

        let (assign98730_e151048, assign98730_e151048_d_n0, assign98730_e151048_d_n2, assign98730_e151048_d_n4, assign98730_e151048_d_n5, assign98730_e151048_d_n6, assign98730_e151048_d_n7, assign98730_e151048_d_n8, assign98730_e151048_d_n9, assign98730_e151048_d_n10, assign98730_e151048_d_n13,) = {
    if ((((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        let (assign98730_e151046, assign98730_e151046_d_n0, assign98730_e151046_d_n2, assign98730_e151046_d_n4, assign98730_e151046_d_n5, assign98730_e151046_d_n6, assign98730_e151046_d_n7, assign98730_e151046_d_n8, assign98730_e151046_d_n9, assign98730_e151046_d_n10, assign98730_e151046_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98730_e151044: f64 = (-p.p505);
                let assign98730_e151045: f64 = (locals.var_arg).powf(assign98730_e151044);
                (assign98730_e151045, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn0)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn2)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn4)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn5)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn6)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn7)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn8)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn9)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn10)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn13)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98730_e151046, assign98730_e151046_d_n0, assign98730_e151046_d_n2, assign98730_e151046_d_n4, assign98730_e151046_d_n5, assign98730_e151046_d_n6, assign98730_e151046_d_n7, assign98730_e151046_d_n8, assign98730_e151046_d_n9, assign98730_e151046_d_n10, assign98730_e151046_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98730_e151048;
        locals.var_sarg_dn0 = assign98730_e151048_d_n0;
        locals.var_sarg_dn2 = assign98730_e151048_d_n2;
        locals.var_sarg_dn4 = assign98730_e151048_d_n4;
        locals.var_sarg_dn5 = assign98730_e151048_d_n5;
        locals.var_sarg_dn6 = assign98730_e151048_d_n6;
        locals.var_sarg_dn7 = assign98730_e151048_d_n7;
        locals.var_sarg_dn8 = assign98730_e151048_d_n8;
        locals.var_sarg_dn9 = assign98730_e151048_d_n9;
        locals.var_sarg_dn10 = assign98730_e151048_d_n10;
        locals.var_sarg_dn13 = assign98730_e151048_d_n13;

        let (assign98740_e151069, assign98740_e151069_d_n0, assign98740_e151069_d_n2, assign98740_e151069_d_n4, assign98740_e151069_d_n5, assign98740_e151069_d_n6, assign98740_e151069_d_n7, assign98740_e151069_d_n8, assign98740_e151069_d_n9, assign98740_e151069_d_n10, assign98740_e151069_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign98740_e151057: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98740_e151061: f64 = (locals.var_arg * locals.var_sarg);
        let assign98740_e151062: f64 = (1.0 - assign98740_e151061);
        let assign98740_e151063: f64 = (assign98740_e151057 * assign98740_e151062);
        let assign98740_e151066: f64 = (1.0 - p.p505);
        let assign98740_e151067: f64 = (assign98740_e151063 / assign98740_e151066);
        (assign98740_e151067, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn13 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn13)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98740_e151066),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98740_e151069;
        locals.var_qbd_swg_dn0 = assign98740_e151069_d_n0;
        locals.var_qbd_swg_dn2 = assign98740_e151069_d_n2;
        locals.var_qbd_swg_dn4 = assign98740_e151069_d_n4;
        locals.var_qbd_swg_dn5 = assign98740_e151069_d_n5;
        locals.var_qbd_swg_dn6 = assign98740_e151069_d_n6;
        locals.var_qbd_swg_dn7 = assign98740_e151069_d_n7;
        locals.var_qbd_swg_dn8 = assign98740_e151069_d_n8;
        locals.var_qbd_swg_dn9 = assign98740_e151069_d_n9;
        locals.var_qbd_swg_dn10 = assign98740_e151069_d_n10;
        locals.var_qbd_swg_dn13 = assign98740_e151069_d_n13;

        let (assign98760_e151090, assign98760_e151090_d_n0, assign98760_e151090_d_n2, assign98760_e151090_d_n4, assign98760_e151090_d_n5, assign98760_e151090_d_n6, assign98760_e151090_d_n7, assign98760_e151090_d_n8, assign98760_e151090_d_n9, assign98760_e151090_d_n10, assign98760_e151090_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98760_e151090;
        locals.var_t1_dn0 = assign98760_e151090_d_n0;
        locals.var_t1_dn2 = assign98760_e151090_d_n2;
        locals.var_t1_dn4 = assign98760_e151090_d_n4;
        locals.var_t1_dn5 = assign98760_e151090_d_n5;
        locals.var_t1_dn6 = assign98760_e151090_d_n6;
        locals.var_t1_dn7 = assign98760_e151090_d_n7;
        locals.var_t1_dn8 = assign98760_e151090_d_n8;
        locals.var_t1_dn9 = assign98760_e151090_d_n9;
        locals.var_t1_dn10 = assign98760_e151090_d_n10;
        locals.var_t1_dn13 = assign98760_e151090_d_n13;

        let (assign98770_e151104, assign98770_e151104_d_n0, assign98770_e151104_d_n2, assign98770_e151104_d_n4, assign98770_e151104_d_n5, assign98770_e151104_d_n6, assign98770_e151104_d_n7, assign98770_e151104_d_n8, assign98770_e151104_d_n9, assign98770_e151104_d_n10, assign98770_e151104_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        let assign98770_e151100: f64 = (locals.var_czbdswg * p.p505);
        let assign98770_e151102: f64 = (assign98770_e151100 / locals.var_pzbdswg);
        (assign98770_e151102, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn13 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn13)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98770_e151104;
        locals.var_t2_dn0 = assign98770_e151104_d_n0;
        locals.var_t2_dn2 = assign98770_e151104_d_n2;
        locals.var_t2_dn4 = assign98770_e151104_d_n4;
        locals.var_t2_dn5 = assign98770_e151104_d_n5;
        locals.var_t2_dn6 = assign98770_e151104_d_n6;
        locals.var_t2_dn7 = assign98770_e151104_d_n7;
        locals.var_t2_dn8 = assign98770_e151104_d_n8;
        locals.var_t2_dn9 = assign98770_e151104_d_n9;
        locals.var_t2_dn10 = assign98770_e151104_d_n10;
        locals.var_t2_dn13 = assign98770_e151104_d_n13;

        let (assign98780_e151122, assign98780_e151122_d_n0, assign98780_e151122_d_n2, assign98780_e151122_d_n4, assign98780_e151122_d_n5, assign98780_e151122_d_n6, assign98780_e151122_d_n7, assign98780_e151122_d_n8, assign98780_e151122_d_n9, assign98780_e151122_d_n10, assign98780_e151122_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        let assign98780_e151116: f64 = (locals.var_vbd_jct * 0.5);
        let assign98780_e151118: f64 = (assign98780_e151116 * locals.var_t2);
        let assign98780_e151119: f64 = (locals.var_t1 + assign98780_e151118);
        let assign98780_e151120: f64 = (locals.var_vbd_jct * assign98780_e151119);
        (assign98780_e151120, ((locals.var_vbd_jct_dn0 * assign98780_e151119) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98780_e151116 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98780_e151116 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98780_e151116 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98780_e151116 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98780_e151116 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98780_e151116 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98780_e151116 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98780_e151119) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98780_e151116 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98780_e151116 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98780_e151116 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98780_e151122;
        locals.var_qbd_swg_dn0 = assign98780_e151122_d_n0;
        locals.var_qbd_swg_dn2 = assign98780_e151122_d_n2;
        locals.var_qbd_swg_dn4 = assign98780_e151122_d_n4;
        locals.var_qbd_swg_dn5 = assign98780_e151122_d_n5;
        locals.var_qbd_swg_dn6 = assign98780_e151122_d_n6;
        locals.var_qbd_swg_dn7 = assign98780_e151122_d_n7;
        locals.var_qbd_swg_dn8 = assign98780_e151122_d_n8;
        locals.var_qbd_swg_dn9 = assign98780_e151122_d_n9;
        locals.var_qbd_swg_dn10 = assign98780_e151122_d_n10;
        locals.var_qbd_swg_dn13 = assign98780_e151122_d_n13;

        let (assign98800_e151144, assign98800_e151144_d_n0, assign98800_e151144_d_n2, assign98800_e151144_d_n4, assign98800_e151144_d_n5, assign98800_e151144_d_n6, assign98800_e151144_d_n7, assign98800_e151144_d_n8, assign98800_e151144_d_n9, assign98800_e151144_d_n10, assign98800_e151144_d_n13,) = {
    if ((locals.var_guard2280 == 0.0) && (locals.var_guard2284 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98800_e151144;
        locals.var_qbd_swg_dn0 = assign98800_e151144_d_n0;
        locals.var_qbd_swg_dn2 = assign98800_e151144_d_n2;
        locals.var_qbd_swg_dn4 = assign98800_e151144_d_n4;
        locals.var_qbd_swg_dn5 = assign98800_e151144_d_n5;
        locals.var_qbd_swg_dn6 = assign98800_e151144_d_n6;
        locals.var_qbd_swg_dn7 = assign98800_e151144_d_n7;
        locals.var_qbd_swg_dn8 = assign98800_e151144_d_n8;
        locals.var_qbd_swg_dn9 = assign98800_e151144_d_n9;
        locals.var_qbd_swg_dn10 = assign98800_e151144_d_n10;
        locals.var_qbd_swg_dn13 = assign98800_e151144_d_n13;

    }

    pub(super) fn stamp_transient_block_351(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign98820_e151155: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98820_e151155;

        let assign98830_e151158: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98830_e151158;

        let (assign98840_e151168, assign98840_e151168_d_n0, assign98840_e151168_d_n2, assign98840_e151168_d_n4, assign98840_e151168_d_n5, assign98840_e151168_d_n6, assign98840_e151168_d_n7, assign98840_e151168_d_n8, assign98840_e151168_d_n9, assign98840_e151168_d_n10, assign98840_e151168_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) {
        let assign98840_e151165: f64 = (locals.var_vbs_jct / locals.var_pzbs);
        let assign98840_e151166: f64 = (1.0 - assign98840_e151165);
        (assign98840_e151166, (-(-((locals.var_vbs_jct * locals.var_pzbs_dn0) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn4) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn5) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn6) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn7) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn8) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn9) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn13) / (locals.var_pzbs * locals.var_pzbs)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98840_e151168;
        locals.var_arg_dn0 = assign98840_e151168_d_n0;
        locals.var_arg_dn2 = assign98840_e151168_d_n2;
        locals.var_arg_dn4 = assign98840_e151168_d_n4;
        locals.var_arg_dn5 = assign98840_e151168_d_n5;
        locals.var_arg_dn6 = assign98840_e151168_d_n6;
        locals.var_arg_dn7 = assign98840_e151168_d_n7;
        locals.var_arg_dn8 = assign98840_e151168_d_n8;
        locals.var_arg_dn9 = assign98840_e151168_d_n9;
        locals.var_arg_dn10 = assign98840_e151168_d_n10;
        locals.var_arg_dn13 = assign98840_e151168_d_n13;

        let assign98850_e151171: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98850_e151171;

        let (assign98860_e151182, assign98860_e151182_d_n0, assign98860_e151182_d_n2, assign98860_e151182_d_n4, assign98860_e151182_d_n5, assign98860_e151182_d_n6, assign98860_e151182_d_n7, assign98860_e151182_d_n8, assign98860_e151182_d_n9, assign98860_e151182_d_n10, assign98860_e151182_d_n13,) = {
    if (((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) {
        let assign98860_e151179: f64 = (locals.var_arg).sqrt();
        let assign98860_e151180: f64 = (1.0 / assign98860_e151179);
        (assign98860_e151180, (-((locals.var_arg_dn0 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn2 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn4 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn5 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn6 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn7 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn8 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn9 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn10 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn13 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98860_e151182;
        locals.var_sarg_dn0 = assign98860_e151182_d_n0;
        locals.var_sarg_dn2 = assign98860_e151182_d_n2;
        locals.var_sarg_dn4 = assign98860_e151182_d_n4;
        locals.var_sarg_dn5 = assign98860_e151182_d_n5;
        locals.var_sarg_dn6 = assign98860_e151182_d_n6;
        locals.var_sarg_dn7 = assign98860_e151182_d_n7;
        locals.var_sarg_dn8 = assign98860_e151182_d_n8;
        locals.var_sarg_dn9 = assign98860_e151182_d_n9;
        locals.var_sarg_dn10 = assign98860_e151182_d_n10;
        locals.var_sarg_dn13 = assign98860_e151182_d_n13;

        let (assign98870_e151199, assign98870_e151199_d_n0, assign98870_e151199_d_n2, assign98870_e151199_d_n4, assign98870_e151199_d_n5, assign98870_e151199_d_n6, assign98870_e151199_d_n7, assign98870_e151199_d_n8, assign98870_e151199_d_n9, assign98870_e151199_d_n10, assign98870_e151199_d_n13,) = {
    if (((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 == 0.0)) {
        let (assign98870_e151197, assign98870_e151197_d_n0, assign98870_e151197_d_n2, assign98870_e151197_d_n4, assign98870_e151197_d_n5, assign98870_e151197_d_n6, assign98870_e151197_d_n7, assign98870_e151197_d_n8, assign98870_e151197_d_n9, assign98870_e151197_d_n10, assign98870_e151197_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98870_e151195: f64 = (-p.p526);
                let assign98870_e151196: f64 = (locals.var_arg).powf(assign98870_e151195);
                (assign98870_e151196, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn0)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn2)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn4)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn5)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn6)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn7)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn8)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn9)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn10)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn13)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98870_e151197, assign98870_e151197_d_n0, assign98870_e151197_d_n2, assign98870_e151197_d_n4, assign98870_e151197_d_n5, assign98870_e151197_d_n6, assign98870_e151197_d_n7, assign98870_e151197_d_n8, assign98870_e151197_d_n9, assign98870_e151197_d_n10, assign98870_e151197_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98870_e151199;
        locals.var_sarg_dn0 = assign98870_e151199_d_n0;
        locals.var_sarg_dn2 = assign98870_e151199_d_n2;
        locals.var_sarg_dn4 = assign98870_e151199_d_n4;
        locals.var_sarg_dn5 = assign98870_e151199_d_n5;
        locals.var_sarg_dn6 = assign98870_e151199_d_n6;
        locals.var_sarg_dn7 = assign98870_e151199_d_n7;
        locals.var_sarg_dn8 = assign98870_e151199_d_n8;
        locals.var_sarg_dn9 = assign98870_e151199_d_n9;
        locals.var_sarg_dn10 = assign98870_e151199_d_n10;
        locals.var_sarg_dn13 = assign98870_e151199_d_n13;

        let (assign98880_e151217, assign98880_e151217_d_n0, assign98880_e151217_d_n2, assign98880_e151217_d_n4, assign98880_e151217_d_n5, assign98880_e151217_d_n6, assign98880_e151217_d_n7, assign98880_e151217_d_n8, assign98880_e151217_d_n9, assign98880_e151217_d_n10, assign98880_e151217_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) {
        let assign98880_e151205: f64 = (locals.var_pzbs * locals.var_czbs);
        let assign98880_e151209: f64 = (locals.var_arg * locals.var_sarg);
        let assign98880_e151210: f64 = (1.0 - assign98880_e151209);
        let assign98880_e151211: f64 = (assign98880_e151205 * assign98880_e151210);
        let assign98880_e151214: f64 = (1.0 - p.p526);
        let assign98880_e151215: f64 = (assign98880_e151211 / assign98880_e151214);
        (assign98880_e151215, (((((locals.var_pzbs_dn0 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn0)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98880_e151214), (((((locals.var_pzbs_dn2 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn2)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98880_e151214), (((((locals.var_pzbs_dn4 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn4)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98880_e151214), (((((locals.var_pzbs_dn5 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn5)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98880_e151214), (((((locals.var_pzbs_dn6 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn6)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98880_e151214), (((((locals.var_pzbs_dn7 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn7)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98880_e151214), (((((locals.var_pzbs_dn8 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn8)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98880_e151214), (((((locals.var_pzbs_dn9 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn9)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98880_e151214), (((((locals.var_pzbs_dn10 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn10)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98880_e151214), (((((locals.var_pzbs_dn13 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn13)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98880_e151214),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98880_e151217;
        locals.var_qbs_btm_dn0 = assign98880_e151217_d_n0;
        locals.var_qbs_btm_dn2 = assign98880_e151217_d_n2;
        locals.var_qbs_btm_dn4 = assign98880_e151217_d_n4;
        locals.var_qbs_btm_dn5 = assign98880_e151217_d_n5;
        locals.var_qbs_btm_dn6 = assign98880_e151217_d_n6;
        locals.var_qbs_btm_dn7 = assign98880_e151217_d_n7;
        locals.var_qbs_btm_dn8 = assign98880_e151217_d_n8;
        locals.var_qbs_btm_dn9 = assign98880_e151217_d_n9;
        locals.var_qbs_btm_dn10 = assign98880_e151217_d_n10;
        locals.var_qbs_btm_dn13 = assign98880_e151217_d_n13;

        let (assign98900_e151232, assign98900_e151232_d_n0, assign98900_e151232_d_n2, assign98900_e151232_d_n4, assign98900_e151232_d_n5, assign98900_e151232_d_n6, assign98900_e151232_d_n7, assign98900_e151232_d_n8, assign98900_e151232_d_n9, assign98900_e151232_d_n10, assign98900_e151232_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98900_e151232;
        locals.var_t1_dn0 = assign98900_e151232_d_n0;
        locals.var_t1_dn2 = assign98900_e151232_d_n2;
        locals.var_t1_dn4 = assign98900_e151232_d_n4;
        locals.var_t1_dn5 = assign98900_e151232_d_n5;
        locals.var_t1_dn6 = assign98900_e151232_d_n6;
        locals.var_t1_dn7 = assign98900_e151232_d_n7;
        locals.var_t1_dn8 = assign98900_e151232_d_n8;
        locals.var_t1_dn9 = assign98900_e151232_d_n9;
        locals.var_t1_dn10 = assign98900_e151232_d_n10;
        locals.var_t1_dn13 = assign98900_e151232_d_n13;

        let (assign98910_e151243, assign98910_e151243_d_n0, assign98910_e151243_d_n2, assign98910_e151243_d_n4, assign98910_e151243_d_n5, assign98910_e151243_d_n6, assign98910_e151243_d_n7, assign98910_e151243_d_n8, assign98910_e151243_d_n9, assign98910_e151243_d_n10, assign98910_e151243_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        let assign98910_e151239: f64 = (locals.var_czbs * p.p526);
        let assign98910_e151241: f64 = (assign98910_e151239 / locals.var_pzbs);
        (assign98910_e151241, ((((locals.var_czbs_dn0 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn0)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn2 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn4 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn4)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn5 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn5)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn6 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn6)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn7 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn7)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn8 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn8)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn9 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn9)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn10 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn13 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn13)) / (locals.var_pzbs * locals.var_pzbs)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98910_e151243;
        locals.var_t2_dn0 = assign98910_e151243_d_n0;
        locals.var_t2_dn2 = assign98910_e151243_d_n2;
        locals.var_t2_dn4 = assign98910_e151243_d_n4;
        locals.var_t2_dn5 = assign98910_e151243_d_n5;
        locals.var_t2_dn6 = assign98910_e151243_d_n6;
        locals.var_t2_dn7 = assign98910_e151243_d_n7;
        locals.var_t2_dn8 = assign98910_e151243_d_n8;
        locals.var_t2_dn9 = assign98910_e151243_d_n9;
        locals.var_t2_dn10 = assign98910_e151243_d_n10;
        locals.var_t2_dn13 = assign98910_e151243_d_n13;

        let (assign98920_e151258, assign98920_e151258_d_n0, assign98920_e151258_d_n2, assign98920_e151258_d_n4, assign98920_e151258_d_n5, assign98920_e151258_d_n6, assign98920_e151258_d_n7, assign98920_e151258_d_n8, assign98920_e151258_d_n9, assign98920_e151258_d_n10, assign98920_e151258_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        let assign98920_e151252: f64 = (locals.var_vbs_jct * 0.5);
        let assign98920_e151254: f64 = (assign98920_e151252 * locals.var_t2);
        let assign98920_e151255: f64 = (locals.var_t1 + assign98920_e151254);
        let assign98920_e151256: f64 = (locals.var_vbs_jct * assign98920_e151255);
        (assign98920_e151256, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign98920_e151252 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign98920_e151255) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign98920_e151252 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign98920_e151252 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign98920_e151252 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign98920_e151252 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign98920_e151252 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign98920_e151252 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign98920_e151252 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign98920_e151255) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign98920_e151252 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign98920_e151252 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98920_e151258;
        locals.var_qbs_btm_dn0 = assign98920_e151258_d_n0;
        locals.var_qbs_btm_dn2 = assign98920_e151258_d_n2;
        locals.var_qbs_btm_dn4 = assign98920_e151258_d_n4;
        locals.var_qbs_btm_dn5 = assign98920_e151258_d_n5;
        locals.var_qbs_btm_dn6 = assign98920_e151258_d_n6;
        locals.var_qbs_btm_dn7 = assign98920_e151258_d_n7;
        locals.var_qbs_btm_dn8 = assign98920_e151258_d_n8;
        locals.var_qbs_btm_dn9 = assign98920_e151258_d_n9;
        locals.var_qbs_btm_dn10 = assign98920_e151258_d_n10;
        locals.var_qbs_btm_dn13 = assign98920_e151258_d_n13;

        let (assign98940_e151274, assign98940_e151274_d_n0, assign98940_e151274_d_n2, assign98940_e151274_d_n4, assign98940_e151274_d_n5, assign98940_e151274_d_n6, assign98940_e151274_d_n7, assign98940_e151274_d_n8, assign98940_e151274_d_n9, assign98940_e151274_d_n10, assign98940_e151274_d_n13,) = {
    if (locals.var_guard2287 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98940_e151274;
        locals.var_qbs_btm_dn0 = assign98940_e151274_d_n0;
        locals.var_qbs_btm_dn2 = assign98940_e151274_d_n2;
        locals.var_qbs_btm_dn4 = assign98940_e151274_d_n4;
        locals.var_qbs_btm_dn5 = assign98940_e151274_d_n5;
        locals.var_qbs_btm_dn6 = assign98940_e151274_d_n6;
        locals.var_qbs_btm_dn7 = assign98940_e151274_d_n7;
        locals.var_qbs_btm_dn8 = assign98940_e151274_d_n8;
        locals.var_qbs_btm_dn9 = assign98940_e151274_d_n9;
        locals.var_qbs_btm_dn10 = assign98940_e151274_d_n10;
        locals.var_qbs_btm_dn13 = assign98940_e151274_d_n13;

        let assign98960_e151282: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98960_e151282;

        let assign98970_e151285: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98970_e151285;

        let (assign98980_e151295, assign98980_e151295_d_n0, assign98980_e151295_d_n2, assign98980_e151295_d_n4, assign98980_e151295_d_n5, assign98980_e151295_d_n6, assign98980_e151295_d_n7, assign98980_e151295_d_n8, assign98980_e151295_d_n9, assign98980_e151295_d_n10, assign98980_e151295_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) {
        let assign98980_e151292: f64 = (locals.var_vbs_jct / locals.var_pzbssw);
        let assign98980_e151293: f64 = (1.0 - assign98980_e151292);
        (assign98980_e151293, (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn0) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn4) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn5) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn6) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn7) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn8) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn9) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn13) / (locals.var_pzbssw * locals.var_pzbssw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98980_e151295;
        locals.var_arg_dn0 = assign98980_e151295_d_n0;
        locals.var_arg_dn2 = assign98980_e151295_d_n2;
        locals.var_arg_dn4 = assign98980_e151295_d_n4;
        locals.var_arg_dn5 = assign98980_e151295_d_n5;
        locals.var_arg_dn6 = assign98980_e151295_d_n6;
        locals.var_arg_dn7 = assign98980_e151295_d_n7;
        locals.var_arg_dn8 = assign98980_e151295_d_n8;
        locals.var_arg_dn9 = assign98980_e151295_d_n9;
        locals.var_arg_dn10 = assign98980_e151295_d_n10;
        locals.var_arg_dn13 = assign98980_e151295_d_n13;

        let assign98990_e151298: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98990_e151298;

        let (assign99000_e151309, assign99000_e151309_d_n0, assign99000_e151309_d_n2, assign99000_e151309_d_n4, assign99000_e151309_d_n5, assign99000_e151309_d_n6, assign99000_e151309_d_n7, assign99000_e151309_d_n8, assign99000_e151309_d_n9, assign99000_e151309_d_n10, assign99000_e151309_d_n13,) = {
    if (((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) && (locals.var_guard2292 != 0.0)) {
        let assign99000_e151306: f64 = (locals.var_arg).sqrt();
        let assign99000_e151307: f64 = (1.0 / assign99000_e151306);
        (assign99000_e151307, (-((locals.var_arg_dn0 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn2 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn4 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn5 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn6 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn7 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn8 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn9 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn10 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn13 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99000_e151309;
        locals.var_sarg_dn0 = assign99000_e151309_d_n0;
        locals.var_sarg_dn2 = assign99000_e151309_d_n2;
        locals.var_sarg_dn4 = assign99000_e151309_d_n4;
        locals.var_sarg_dn5 = assign99000_e151309_d_n5;
        locals.var_sarg_dn6 = assign99000_e151309_d_n6;
        locals.var_sarg_dn7 = assign99000_e151309_d_n7;
        locals.var_sarg_dn8 = assign99000_e151309_d_n8;
        locals.var_sarg_dn9 = assign99000_e151309_d_n9;
        locals.var_sarg_dn10 = assign99000_e151309_d_n10;
        locals.var_sarg_dn13 = assign99000_e151309_d_n13;

        let (assign99010_e151326, assign99010_e151326_d_n0, assign99010_e151326_d_n2, assign99010_e151326_d_n4, assign99010_e151326_d_n5, assign99010_e151326_d_n6, assign99010_e151326_d_n7, assign99010_e151326_d_n8, assign99010_e151326_d_n9, assign99010_e151326_d_n10, assign99010_e151326_d_n13,) = {
    if (((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) && (locals.var_guard2292 == 0.0)) {
        let (assign99010_e151324, assign99010_e151324_d_n0, assign99010_e151324_d_n2, assign99010_e151324_d_n4, assign99010_e151324_d_n5, assign99010_e151324_d_n6, assign99010_e151324_d_n7, assign99010_e151324_d_n8, assign99010_e151324_d_n9, assign99010_e151324_d_n10, assign99010_e151324_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99010_e151322: f64 = (-p.p527);
                let assign99010_e151323: f64 = (locals.var_arg).powf(assign99010_e151322);
                (assign99010_e151323, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn0)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn2)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn4)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn5)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn6)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn7)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn8)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn9)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn10)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn13)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99010_e151324, assign99010_e151324_d_n0, assign99010_e151324_d_n2, assign99010_e151324_d_n4, assign99010_e151324_d_n5, assign99010_e151324_d_n6, assign99010_e151324_d_n7, assign99010_e151324_d_n8, assign99010_e151324_d_n9, assign99010_e151324_d_n10, assign99010_e151324_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99010_e151326;
        locals.var_sarg_dn0 = assign99010_e151326_d_n0;
        locals.var_sarg_dn2 = assign99010_e151326_d_n2;
        locals.var_sarg_dn4 = assign99010_e151326_d_n4;
        locals.var_sarg_dn5 = assign99010_e151326_d_n5;
        locals.var_sarg_dn6 = assign99010_e151326_d_n6;
        locals.var_sarg_dn7 = assign99010_e151326_d_n7;
        locals.var_sarg_dn8 = assign99010_e151326_d_n8;
        locals.var_sarg_dn9 = assign99010_e151326_d_n9;
        locals.var_sarg_dn10 = assign99010_e151326_d_n10;
        locals.var_sarg_dn13 = assign99010_e151326_d_n13;

        let (assign99020_e151344, assign99020_e151344_d_n0, assign99020_e151344_d_n2, assign99020_e151344_d_n4, assign99020_e151344_d_n5, assign99020_e151344_d_n6, assign99020_e151344_d_n7, assign99020_e151344_d_n8, assign99020_e151344_d_n9, assign99020_e151344_d_n10, assign99020_e151344_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) {
        let assign99020_e151332: f64 = (locals.var_pzbssw * locals.var_czbssw);
        let assign99020_e151336: f64 = (locals.var_arg * locals.var_sarg);
        let assign99020_e151337: f64 = (1.0 - assign99020_e151336);
        let assign99020_e151338: f64 = (assign99020_e151332 * assign99020_e151337);
        let assign99020_e151341: f64 = (1.0 - p.p527);
        let assign99020_e151342: f64 = (assign99020_e151338 / assign99020_e151341);
        (assign99020_e151342, (((((locals.var_pzbssw_dn0 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn0)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99020_e151341), (((((locals.var_pzbssw_dn2 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn2)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99020_e151341), (((((locals.var_pzbssw_dn4 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn4)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99020_e151341), (((((locals.var_pzbssw_dn5 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn5)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99020_e151341), (((((locals.var_pzbssw_dn6 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn6)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99020_e151341), (((((locals.var_pzbssw_dn7 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn7)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99020_e151341), (((((locals.var_pzbssw_dn8 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn8)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99020_e151341), (((((locals.var_pzbssw_dn9 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn9)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99020_e151341), (((((locals.var_pzbssw_dn10 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn10)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99020_e151341), (((((locals.var_pzbssw_dn13 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn13)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99020_e151341),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99020_e151344;
        locals.var_qbs_sws_dn0 = assign99020_e151344_d_n0;
        locals.var_qbs_sws_dn2 = assign99020_e151344_d_n2;
        locals.var_qbs_sws_dn4 = assign99020_e151344_d_n4;
        locals.var_qbs_sws_dn5 = assign99020_e151344_d_n5;
        locals.var_qbs_sws_dn6 = assign99020_e151344_d_n6;
        locals.var_qbs_sws_dn7 = assign99020_e151344_d_n7;
        locals.var_qbs_sws_dn8 = assign99020_e151344_d_n8;
        locals.var_qbs_sws_dn9 = assign99020_e151344_d_n9;
        locals.var_qbs_sws_dn10 = assign99020_e151344_d_n10;
        locals.var_qbs_sws_dn13 = assign99020_e151344_d_n13;

        let (assign99040_e151359, assign99040_e151359_d_n0, assign99040_e151359_d_n2, assign99040_e151359_d_n4, assign99040_e151359_d_n5, assign99040_e151359_d_n6, assign99040_e151359_d_n7, assign99040_e151359_d_n8, assign99040_e151359_d_n9, assign99040_e151359_d_n10, assign99040_e151359_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99040_e151359;
        locals.var_t1_dn0 = assign99040_e151359_d_n0;
        locals.var_t1_dn2 = assign99040_e151359_d_n2;
        locals.var_t1_dn4 = assign99040_e151359_d_n4;
        locals.var_t1_dn5 = assign99040_e151359_d_n5;
        locals.var_t1_dn6 = assign99040_e151359_d_n6;
        locals.var_t1_dn7 = assign99040_e151359_d_n7;
        locals.var_t1_dn8 = assign99040_e151359_d_n8;
        locals.var_t1_dn9 = assign99040_e151359_d_n9;
        locals.var_t1_dn10 = assign99040_e151359_d_n10;
        locals.var_t1_dn13 = assign99040_e151359_d_n13;

        let (assign99050_e151370, assign99050_e151370_d_n0, assign99050_e151370_d_n2, assign99050_e151370_d_n4, assign99050_e151370_d_n5, assign99050_e151370_d_n6, assign99050_e151370_d_n7, assign99050_e151370_d_n8, assign99050_e151370_d_n9, assign99050_e151370_d_n10, assign99050_e151370_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        let assign99050_e151366: f64 = (locals.var_czbssw * p.p527);
        let assign99050_e151368: f64 = (assign99050_e151366 / locals.var_pzbssw);
        (assign99050_e151368, ((((locals.var_czbssw_dn0 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn0)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn2 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn4 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn4)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn5 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn5)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn6 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn6)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn7 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn7)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn8 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn8)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn9 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn9)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn10 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn13 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn13)) / (locals.var_pzbssw * locals.var_pzbssw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99050_e151370;
        locals.var_t2_dn0 = assign99050_e151370_d_n0;
        locals.var_t2_dn2 = assign99050_e151370_d_n2;
        locals.var_t2_dn4 = assign99050_e151370_d_n4;
        locals.var_t2_dn5 = assign99050_e151370_d_n5;
        locals.var_t2_dn6 = assign99050_e151370_d_n6;
        locals.var_t2_dn7 = assign99050_e151370_d_n7;
        locals.var_t2_dn8 = assign99050_e151370_d_n8;
        locals.var_t2_dn9 = assign99050_e151370_d_n9;
        locals.var_t2_dn10 = assign99050_e151370_d_n10;
        locals.var_t2_dn13 = assign99050_e151370_d_n13;

        let (assign99060_e151385, assign99060_e151385_d_n0, assign99060_e151385_d_n2, assign99060_e151385_d_n4, assign99060_e151385_d_n5, assign99060_e151385_d_n6, assign99060_e151385_d_n7, assign99060_e151385_d_n8, assign99060_e151385_d_n9, assign99060_e151385_d_n10, assign99060_e151385_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        let assign99060_e151379: f64 = (locals.var_vbs_jct * 0.5);
        let assign99060_e151381: f64 = (assign99060_e151379 * locals.var_t2);
        let assign99060_e151382: f64 = (locals.var_t1 + assign99060_e151381);
        let assign99060_e151383: f64 = (locals.var_vbs_jct * assign99060_e151382);
        (assign99060_e151383, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99060_e151379 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99060_e151382) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99060_e151379 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99060_e151379 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99060_e151379 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99060_e151379 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99060_e151379 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99060_e151379 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99060_e151379 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign99060_e151382) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign99060_e151379 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign99060_e151379 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99060_e151385;
        locals.var_qbs_sws_dn0 = assign99060_e151385_d_n0;
        locals.var_qbs_sws_dn2 = assign99060_e151385_d_n2;
        locals.var_qbs_sws_dn4 = assign99060_e151385_d_n4;
        locals.var_qbs_sws_dn5 = assign99060_e151385_d_n5;
        locals.var_qbs_sws_dn6 = assign99060_e151385_d_n6;
        locals.var_qbs_sws_dn7 = assign99060_e151385_d_n7;
        locals.var_qbs_sws_dn8 = assign99060_e151385_d_n8;
        locals.var_qbs_sws_dn9 = assign99060_e151385_d_n9;
        locals.var_qbs_sws_dn10 = assign99060_e151385_d_n10;
        locals.var_qbs_sws_dn13 = assign99060_e151385_d_n13;

        let (assign99080_e151401, assign99080_e151401_d_n0, assign99080_e151401_d_n2, assign99080_e151401_d_n4, assign99080_e151401_d_n5, assign99080_e151401_d_n6, assign99080_e151401_d_n7, assign99080_e151401_d_n8, assign99080_e151401_d_n9, assign99080_e151401_d_n10, assign99080_e151401_d_n13,) = {
    if (locals.var_guard2290 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99080_e151401;
        locals.var_qbs_sws_dn0 = assign99080_e151401_d_n0;
        locals.var_qbs_sws_dn2 = assign99080_e151401_d_n2;
        locals.var_qbs_sws_dn4 = assign99080_e151401_d_n4;
        locals.var_qbs_sws_dn5 = assign99080_e151401_d_n5;
        locals.var_qbs_sws_dn6 = assign99080_e151401_d_n6;
        locals.var_qbs_sws_dn7 = assign99080_e151401_d_n7;
        locals.var_qbs_sws_dn8 = assign99080_e151401_d_n8;
        locals.var_qbs_sws_dn9 = assign99080_e151401_d_n9;
        locals.var_qbs_sws_dn10 = assign99080_e151401_d_n10;
        locals.var_qbs_sws_dn13 = assign99080_e151401_d_n13;

        let assign99100_e151409: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign99100_e151409;

        let assign99110_e151412: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99110_e151412;

        let assign99120_e151415: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99120_e151415;

        let (assign99130_e151427, assign99130_e151427_d_n0, assign99130_e151427_d_n2, assign99130_e151427_d_n4, assign99130_e151427_d_n5, assign99130_e151427_d_n6, assign99130_e151427_d_n7, assign99130_e151427_d_n8, assign99130_e151427_d_n9, assign99130_e151427_d_n10, assign99130_e151427_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99130_e151424: f64 = (locals.var_vbsi_jct / locals.var_pzbsswg);
        let assign99130_e151425: f64 = (1.0 - assign99130_e151424);
        (assign99130_e151425, (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn2) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbsi_jct_dn7 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(((locals.var_vbsi_jct_dn8 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn13) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign99130_e151427;
        locals.var_arg_dn0 = assign99130_e151427_d_n0;
        locals.var_arg_dn2 = assign99130_e151427_d_n2;
        locals.var_arg_dn4 = assign99130_e151427_d_n4;
        locals.var_arg_dn5 = assign99130_e151427_d_n5;
        locals.var_arg_dn6 = assign99130_e151427_d_n6;
        locals.var_arg_dn7 = assign99130_e151427_d_n7;
        locals.var_arg_dn8 = assign99130_e151427_d_n8;
        locals.var_arg_dn9 = assign99130_e151427_d_n9;
        locals.var_arg_dn10 = assign99130_e151427_d_n10;
        locals.var_arg_dn13 = assign99130_e151427_d_n13;

        let assign99140_e151430: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99140_e151430;

        let (assign99150_e151443, assign99150_e151443_d_n0, assign99150_e151443_d_n2, assign99150_e151443_d_n4, assign99150_e151443_d_n5, assign99150_e151443_d_n6, assign99150_e151443_d_n7, assign99150_e151443_d_n8, assign99150_e151443_d_n9, assign99150_e151443_d_n10, assign99150_e151443_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 != 0.0)) {
        let assign99150_e151440: f64 = (locals.var_arg).sqrt();
        let assign99150_e151441: f64 = (1.0 / assign99150_e151440);
        (assign99150_e151441, (-((locals.var_arg_dn0 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn2 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn4 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn5 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn6 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn7 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn8 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn9 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn10 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn13 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99150_e151443;
        locals.var_sarg_dn0 = assign99150_e151443_d_n0;
        locals.var_sarg_dn2 = assign99150_e151443_d_n2;
        locals.var_sarg_dn4 = assign99150_e151443_d_n4;
        locals.var_sarg_dn5 = assign99150_e151443_d_n5;
        locals.var_sarg_dn6 = assign99150_e151443_d_n6;
        locals.var_sarg_dn7 = assign99150_e151443_d_n7;
        locals.var_sarg_dn8 = assign99150_e151443_d_n8;
        locals.var_sarg_dn9 = assign99150_e151443_d_n9;
        locals.var_sarg_dn10 = assign99150_e151443_d_n10;
        locals.var_sarg_dn13 = assign99150_e151443_d_n13;

        let (assign99160_e151462, assign99160_e151462_d_n0, assign99160_e151462_d_n2, assign99160_e151462_d_n4, assign99160_e151462_d_n5, assign99160_e151462_d_n6, assign99160_e151462_d_n7, assign99160_e151462_d_n8, assign99160_e151462_d_n9, assign99160_e151462_d_n10, assign99160_e151462_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 == 0.0)) {
        let (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99160_e151458: f64 = (-p.p528);
                let assign99160_e151459: f64 = (locals.var_arg).powf(assign99160_e151458);
                (assign99160_e151459, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn0)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn2)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn4)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn5)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn6)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn7)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn8)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn9)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn10)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn13)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99160_e151462;
        locals.var_sarg_dn0 = assign99160_e151462_d_n0;
        locals.var_sarg_dn2 = assign99160_e151462_d_n2;
        locals.var_sarg_dn4 = assign99160_e151462_d_n4;
        locals.var_sarg_dn5 = assign99160_e151462_d_n5;
        locals.var_sarg_dn6 = assign99160_e151462_d_n6;
        locals.var_sarg_dn7 = assign99160_e151462_d_n7;
        locals.var_sarg_dn8 = assign99160_e151462_d_n8;
        locals.var_sarg_dn9 = assign99160_e151462_d_n9;
        locals.var_sarg_dn10 = assign99160_e151462_d_n10;
        locals.var_sarg_dn13 = assign99160_e151462_d_n13;

        let (assign99170_e151482, assign99170_e151482_d_n0, assign99170_e151482_d_n2, assign99170_e151482_d_n4, assign99170_e151482_d_n5, assign99170_e151482_d_n6, assign99170_e151482_d_n7, assign99170_e151482_d_n8, assign99170_e151482_d_n9, assign99170_e151482_d_n10, assign99170_e151482_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99170_e151470: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99170_e151474: f64 = (locals.var_arg * locals.var_sarg);
        let assign99170_e151475: f64 = (1.0 - assign99170_e151474);
        let assign99170_e151476: f64 = (assign99170_e151470 * assign99170_e151475);
        let assign99170_e151479: f64 = (1.0 - p.p528);
        let assign99170_e151480: f64 = (assign99170_e151476 / assign99170_e151479);
        (assign99170_e151480, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn13 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn13)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99170_e151479),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99170_e151482;
        locals.var_qbs_swg_dn0 = assign99170_e151482_d_n0;
        locals.var_qbs_swg_dn2 = assign99170_e151482_d_n2;
        locals.var_qbs_swg_dn4 = assign99170_e151482_d_n4;
        locals.var_qbs_swg_dn5 = assign99170_e151482_d_n5;
        locals.var_qbs_swg_dn6 = assign99170_e151482_d_n6;
        locals.var_qbs_swg_dn7 = assign99170_e151482_d_n7;
        locals.var_qbs_swg_dn8 = assign99170_e151482_d_n8;
        locals.var_qbs_swg_dn9 = assign99170_e151482_d_n9;
        locals.var_qbs_swg_dn10 = assign99170_e151482_d_n10;
        locals.var_qbs_swg_dn13 = assign99170_e151482_d_n13;

        let (assign99190_e151501, assign99190_e151501_d_n0, assign99190_e151501_d_n2, assign99190_e151501_d_n4, assign99190_e151501_d_n5, assign99190_e151501_d_n6, assign99190_e151501_d_n7, assign99190_e151501_d_n8, assign99190_e151501_d_n9, assign99190_e151501_d_n10, assign99190_e151501_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99190_e151501;
        locals.var_t1_dn0 = assign99190_e151501_d_n0;
        locals.var_t1_dn2 = assign99190_e151501_d_n2;
        locals.var_t1_dn4 = assign99190_e151501_d_n4;
        locals.var_t1_dn5 = assign99190_e151501_d_n5;
        locals.var_t1_dn6 = assign99190_e151501_d_n6;
        locals.var_t1_dn7 = assign99190_e151501_d_n7;
        locals.var_t1_dn8 = assign99190_e151501_d_n8;
        locals.var_t1_dn9 = assign99190_e151501_d_n9;
        locals.var_t1_dn10 = assign99190_e151501_d_n10;
        locals.var_t1_dn13 = assign99190_e151501_d_n13;

    }
}
